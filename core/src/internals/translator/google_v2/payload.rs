use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoogleTranslateResponse {
    pub data: TranslateTextResponseList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslateTextResponseList {
    pub translations: Vec<TranslateTextResponseTranslation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateTextResponseTranslation {
    #[serde(rename = "detectedSourceLanguage")]
    pub detected_source_language: Option<String>,
    pub model: Option<String>,

    #[serde(rename = "translatedText")]
    pub translated_text: String,
}
