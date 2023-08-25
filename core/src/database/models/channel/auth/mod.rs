use serde_json::Value;

use self::data::Oath2Data;
use serde::{Deserialize, Serialize};

pub mod data;

#[derive(Debug, Clone, PartialEq)]
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

        let data = match value.get_mut("data") {
            Some(data) => data.take(),
            None => {
                eprintln!("Missing data field");
                return Ok(AuthType::Invalid);
            }
        };

        let type_ = match value.get("type") {
            Some(type_) => type_,
            None => {
                eprintln!("Missing type field");
                return Ok(AuthType::Invalid);
            }
        };

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

impl Serialize for AuthType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            AuthType::Oauth2(data) => {
                let mut value = serde_json::value::Map::new();
                let data = serde_json::to_value(data).unwrap();
                let type_ = serde_json::to_value("OAUTH2").unwrap();

                value.insert("type".to_string(), type_);
                value.insert("data".to_string(), data);

                value.serialize(serializer)
            }
            AuthType::Invalid => {
                let value = serde_json::to_value("INVALID").unwrap();
                value.serialize(serializer)
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
