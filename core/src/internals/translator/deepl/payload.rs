
use serde::{Serialize,Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeeplResponse {
    pub translations: Vec<Translation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translation {
    #[serde(rename = "detected_source_language")]
    pub detected_source_language: String,
    pub text: String,
}