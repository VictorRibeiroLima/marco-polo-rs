use sqlx::types::Json;

use self::{auth_type::AuthType, data::AuthData};

pub mod auth_type;
pub mod data;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Auth<T: AuthData> {
    #[serde(rename = "type")] // This is needed because `type` is a reserved keyword in Rust
    pub auth_type: AuthType,
    pub data: Json<T>,
}

#[cfg(test)]
mod test {

    use serde_json;

    use crate::database::models::channel::auth::auth_type::AuthType;

    use super::{data::Oath2Data, Auth};

    #[test]
    fn test_deserialize() {
        let json = r#"{
      "type": "OAUTH2",
      "data": {
        "csrf_token": "csrf_token",
        "refresh_token": "refresh_token"
      }
    }"#;

        let auth: Auth<Oath2Data> = serde_json::from_str(json).unwrap();

        assert_eq!(auth.auth_type, AuthType::Oauth2);
        assert_eq!(auth.data.csrf_token, Some("csrf_token".to_string()));
        assert_eq!(auth.data.refresh_token, Some("refresh_token".to_string()));
    }
}
