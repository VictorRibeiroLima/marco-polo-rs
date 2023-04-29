use serde::{Deserialize, Serialize};

use crate::internals::transcriber::traits::TranscriptionSentence;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscribeSentencesResponse {
    pub sentences: Vec<Sentence>,
    pub id: String,
    pub confidence: f64,
    #[serde(rename = "audio_duration")]
    pub audio_duration: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sentence {
    pub text: String,
    pub start: i32,
    pub end: i32,
    pub confidence: f64,
    pub words: Vec<Word>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Word {
    pub text: String,
    pub start: i32,
    pub end: i32,
    pub confidence: f64,
}

impl Into<TranscriptionSentence> for Sentence {
    fn into(self) -> TranscriptionSentence {
        TranscriptionSentence {
            text: self.text,
            start_time: self.start,
            end_time: self.end,
        }
    }
}
