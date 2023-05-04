use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookRequestBody {
    pub status: String,
    #[serde(rename = "transcript_id")]
    pub transcript_id: String,
}
