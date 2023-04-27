use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadPayload {
    #[serde(rename = "s3VideoURI")]
    pub s3video_uri: String,
}

// TODO: Implement enum iterator
#[derive(Debug)]
pub enum PayloadType {
    BatukaVideoUpload(UploadPayload),
}

impl PayloadType {
    pub fn from_str(s: &str) -> Result<PayloadType, Box<dyn std::error::Error>> {
        let v: Value = serde_json::from_str(s)?;
        let type_field = match v["type"].as_str() {
            Some(type_field) => type_field,
            None => return Err("No type field".into()),
        };
        if v["payload"].is_null() {
            return Err("No payload field".into());
        }
        let payload = v["payload"].to_string();
        match type_field {
            "BatukaVideoUpload" => {
                let payload: UploadPayload = serde_json::from_str(&payload)?;
                return Ok(PayloadType::BatukaVideoUpload(payload));
            }
            _ => Err("Invalid type field".into()),
        }
    }
}
