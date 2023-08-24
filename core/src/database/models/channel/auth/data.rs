use super::auth_type::AuthType;

pub trait AuthData {
    fn get_auth_type() -> AuthType;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Oath2Data {
    pub csrf_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl AuthData for Oath2Data {
    fn get_auth_type() -> AuthType {
        AuthType::Oauth2
    }
}
