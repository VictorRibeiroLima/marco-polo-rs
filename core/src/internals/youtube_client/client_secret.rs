use oauth2::basic::BasicClient;
use oauth2::AuthUrl;
use oauth2::ClientId;
use oauth2::RedirectUrl;
use oauth2::TokenUrl;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientSecret {
    pub web: Web,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Web {
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "auth_uri")]
    pub auth_uri: String,
    #[serde(rename = "token_uri")]
    pub token_uri: String,
    #[serde(rename = "auth_provider_x509_cert_url")]
    pub auth_provider_x509_cert_url: String,
    #[serde(rename = "client_secret")]
    pub client_secret: String,
    #[serde(rename = "redirect_uris")]
    pub redirect_uris: Vec<String>,
    #[serde(rename = "javascript_origins")]
    pub javascript_origins: Vec<String>,
}

impl Into<BasicClient> for ClientSecret {
    fn into(mut self) -> BasicClient {
        let redirect_url = self.web.redirect_uris.pop().unwrap();

        let client = BasicClient::new(
            ClientId::new(self.web.client_id),
            Some(oauth2::ClientSecret::new(self.web.client_secret)),
            AuthUrl::new(self.web.auth_uri).unwrap(),
            Some(TokenUrl::new(self.web.token_uri).unwrap()),
        )
        // Set the URL the user will be redirected to after the authorization process.
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

        return client;
    }
}
