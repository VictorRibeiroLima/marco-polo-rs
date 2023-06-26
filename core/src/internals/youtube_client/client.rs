use async_trait::async_trait;
use oauth2::{AuthorizationCode, CsrfToken, RefreshToken, Scope, TokenResponse};

use std::fs::File;
use std::io::Read;

use crate::internals::youtube_client::client_secret::ClientSecret;
use crate::SyncError;

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
        let token = self
            .oauth2_client
            .exchange_refresh_token(&RefreshToken::new(refresh_token))
            .request_async(oauth2::reqwest::async_http_client)
            .await?;

        return Ok(token.access_token().secret().to_string());
    }
}

#[async_trait]
impl super::traits::YoutubeClient for YoutubeClient {
    fn generate_url(&self) -> (String, String) {
        let (auth_url, csrf_token) = self
            .oauth2_client
            .authorize_url(CsrfToken::new_random)
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

    async fn get_channel_info(&self, refresh_token: String) -> Result<String, SyncError> {
        let token = self.get_token(refresh_token).await?;

        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/youtube/v3/channels")
            .bearer_auth(token)
            .send()
            .await?;

        let response = response.text().await?;

        println!("response: {}", response);
        return Ok(response);
    }
}
