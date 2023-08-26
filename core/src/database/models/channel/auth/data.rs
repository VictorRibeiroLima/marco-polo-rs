pub trait AuthData {}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Oath2Data {
    pub csrf_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[cfg(test)]
mod test {

    use serde_json;

    use crate::database::models::channel::auth::AuthType;

    #[test]
    fn test_deserialize_valid() {
        let json = r#"{
      "type": "OAUTH2",
      "data": {
        "csrf_token": "csrf_token",
        "refresh_token": "refresh_token"
      }
    }"#;

        let auth: AuthType = serde_json::from_str(json).unwrap();

        match auth {
            AuthType::Oauth2(data) => {
                assert_eq!(data.csrf_token, Some("csrf_token".to_string()));
                assert_eq!(data.refresh_token, Some("refresh_token".to_string()));
            }
            _ => {
                panic!("Invalid auth type")
            }
        }
    }

    #[test]
    fn test_deserialize_invalid() {
        let json = r#"{
      "type": "OAUTH2",
      "data": "INVALID"
    }"#;

        let auth: AuthType = serde_json::from_str(json).unwrap();

        assert_eq!(auth, AuthType::Invalid);
    }
}
