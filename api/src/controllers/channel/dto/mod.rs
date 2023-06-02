use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OauthQueryParams {
    pub code: String,
    pub state: String,
    pub scope: String,
}
