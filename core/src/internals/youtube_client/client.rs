use crate::database::models::video::with::VideoWithStorageAndChannel;
use crate::util::fs::create_temp_dir;
use async_trait::async_trait;
use google_youtube3::api::{Video, VideoSnippet, VideoStatus};
use google_youtube3::hyper::{Body, Client};
use google_youtube3::oauth2::AccessTokenAuthenticator;
use hyper_tls::HttpsConnector;
use oauth2::{AuthorizationCode, CsrfToken, RefreshToken, Scope, TokenResponse};

use std::fs::File;
use std::io::Read;

use crate::internals::youtube_client::client_secret::ClientSecret;
use crate::SyncError;

use super::channel_info::ChannelInfo;
use super::upload_delegator::UploadDelegator;

pub struct YoutubeClient {
    oauth2_client: oauth2::basic::BasicClient,
}

impl YoutubeClient {
    pub fn new() -> Self {
        println!("starting YoutubeClient ...");
        let mut file = File::open("yt-client-secret.json").unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        let client_secret: ClientSecret = serde_json::from_str(&file_content).unwrap();

        return YoutubeClient {
            oauth2_client: client_secret.into(),
        };
    }

    async fn get_token(&self, refresh_token: String) -> Result<String, SyncError> {
        let result = self
            .oauth2_client
            .exchange_refresh_token(&RefreshToken::new(refresh_token))
            .request_async(oauth2::reqwest::async_http_client)
            .await;

        let token = match result {
            Ok(token) => token,
            Err(err) => match err {
                oauth2::RequestTokenError::ServerResponse(response) => {
                    let fallback_description =
                        String::from("Token request error without description");
                    let description = response
                        .error_description()
                        .unwrap_or(&fallback_description);

                    println!("error description: {}", description);

                    return Err(description.to_string().into());
                }
                _ => {
                    return Err(err.into());
                }
            },
        };

        return Ok(token.access_token().secret().to_string());
    }
}

#[async_trait]
impl super::traits::YoutubeClient for YoutubeClient {
    fn generate_url(&self) -> (String, String) {
        let (auth_url, csrf_token) = self
            .oauth2_client
            .authorize_url(CsrfToken::new_random)
            .add_extra_param("access_type", "offline")
            .add_extra_param("approval_prompt", "force")
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/youtube".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/youtube.readonly".to_string(),
            ))
            .url();

        return (auth_url.to_string(), csrf_token.secret().to_string());
    }

    async fn get_refresh_token(&self, code: String) -> Result<String, SyncError> {
        let token = self
            .oauth2_client
            .exchange_code(AuthorizationCode::new(code))
            .add_extra_param("access_type", "offline")
            .request_async(oauth2::reqwest::async_http_client)
            .await?;

        let token = match token.refresh_token() {
            Some(token) => token,
            None => {
                return Err("no refresh token".into());
            }
        };

        return Ok(token.secret().to_string());
    }

    async fn get_channel_info(&self, refresh_token: String) -> Result<ChannelInfo, SyncError> {
        let token = self.get_token(refresh_token).await?;
        let url =
            "https://www.googleapis.com/youtube/v3/channels?part=snippet,contentDetails&mine=true";

        let client = reqwest::Client::new();
        let response = client.get(url).bearer_auth(token).send().await?;

        if !response.status().is_success() {
            let error = format!(
                "request to {} error with status: {}",
                url,
                response.status()
            );
            return Err(error.into());
        }

        let response = response.json().await?;

        return Ok(response);
    }

    async fn upload_video(&self, video: &VideoWithStorageAndChannel) -> Result<Video, SyncError> {
        let chunk_size: u64 = 5 * 1024 * 1024; // 5MB

        let storage = &video.storage;
        let channel = &video.channel;
        let video = &video.video;

        let refresh_token = match &channel.refresh_token {
            Some(refresh_token) => refresh_token.to_string(),
            None => {
                return Err("no refresh token".into());
            }
        };

        let video_id = video.id.to_string();
        let format = storage.format.to_string();

        let temp_dir = create_temp_dir()?;
        let path = format!("output_{}.{}", video_id, format);
        let path = temp_dir.join(path);

        let video_file = File::open(path.clone())?;
        let reader = std::io::BufReader::new(video_file);

        let token = self.get_token(refresh_token).await?;

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);

        let authenticator = AccessTokenAuthenticator::builder(token).build().await?;
        let hub = google_youtube3::YouTube::new(client, authenticator);

        let tags = match video.tags.as_ref() {
            Some(tags) => Some(tags.split(";").map(|s| s.to_string()).collect()),
            None => None,
        };

        let video = Video {
            snippet: Some(VideoSnippet {
                title: Some(video.title.to_string()),
                description: Some(video.description.to_string()),
                tags,
                ..Default::default()
            }),
            status: Some(VideoStatus {
                privacy_status: Some("public".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut delegate = UploadDelegator::new(chunk_size);

        let insert_call = hub.videos().insert(video).delegate(&mut delegate);

        let (response, video_response) = insert_call
            .upload_resumable(reader, "application/octet-stream".parse().unwrap())
            .await?;

        if !response.status().is_success() {
            return Err(format!(
                "request to {} error with status: {}",
                "Youtube API",
                response.status()
            )
            .into());
        }

        if video_response.id.is_none() {
            return Err("no video id".into());
        }

        match std::fs::remove_file(path) {
            Ok(_) => {}
            Err(err) => {
                println!("failed to remove file: {}", err);
            }
        }

        return Ok(video_response);
    }
}
