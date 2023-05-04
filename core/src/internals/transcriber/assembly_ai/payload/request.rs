use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct TranscribeRequestBody {
    #[serde(rename = "audio_url")]
    pub audio_url: String,
    #[serde(rename = "webhook_url")]
    pub webhook_url: String,
    #[serde(rename = "webhook_auth_header_name")]
    pub webhook_auth_header_name: String,
    #[serde(rename = "webhook_auth_header_value")]
    pub webhook_auth_header_value: String,
}
