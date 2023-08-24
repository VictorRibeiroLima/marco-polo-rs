use serde_json::Value;

use self::data::Oath2Data;
use serde::{Deserialize, Serialize};

pub mod data;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum AuthType {
    Oauth2(Oath2Data),
    Invalid,
}

impl<'de> Deserialize<'de> for AuthType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut value: Value = Deserialize::deserialize(deserializer)?;

        let data = value
            .get_mut("data")
            .ok_or(serde::de::Error::missing_field("data"))?
            .take();

        let type_ = value
            .get("type")
            .ok_or(serde::de::Error::missing_field("type"))?;

        match type_.as_str() {
            Some("OAUTH2") => {
                let data: Oath2Data = match serde_json::from_value(data) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Error deserializing OAUTH2 data: {}", e);
                        return Ok(AuthType::Invalid);
                    }
                };
                Ok(AuthType::Oauth2(data))
            }
            _ => {
                eprintln!("Invalid auth type {:?}", type_);
                Ok(AuthType::Invalid)
            }
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_invalid_input() {
        let json = r#"{
          "type": "INVALID",
          "data": {}
        }"#;

        let auth: super::AuthType = serde_json::from_str(json).unwrap();

        assert_eq!(auth, super::AuthType::Invalid);
    }
}
