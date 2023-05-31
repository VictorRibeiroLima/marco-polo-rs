use oauth2::{CsrfToken, PkceCodeChallenge, Scope};
use std::fs::File;
use std::io::Read;

use self::client_secret::ClientSecret;

mod client_secret;

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

    pub fn generate_url(&self) -> (String, String) {
        let (pkce_challenge, _) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self
            .oauth2_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/youtube".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/youtube.readonly".to_string(),
            ))
            .set_pkce_challenge(pkce_challenge)
            .url();

        return (auth_url.to_string(), csrf_token.secret().to_string());
    }
}
