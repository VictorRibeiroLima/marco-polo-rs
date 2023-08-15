use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::database::models::video_storage::VideoFormat;

#[derive(Debug, Serialize)]
pub struct VideoPayload {
    pub video_uri: String,
    pub video_id: Uuid,
}

impl VideoPayload {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct SrtPayload {
    pub video_id: Uuid,
    pub srt_uri: String,
}

impl SrtPayload {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoDownloadPayload {
    pub video_url: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub video_format: VideoFormat,
    pub video_id: Uuid,
}

impl VideoDownloadPayload {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug)]
pub enum PayloadType {
    BatukaVideoRawUpload(VideoPayload),
    BatukaVideoProcessedUpload(VideoPayload),
    BatukaSrtTranscriptionUpload(SrtPayload),
    BatukaSrtTranslationUpload(SrtPayload),
    BatukaDownloadVideo(VideoDownloadPayload),
}

impl PayloadType {
    pub fn to_json(&self) -> String {
        match self {
            PayloadType::BatukaVideoRawUpload(payload) => {
                let json = json!({"type": "BatukaVideoRawUpload", "payload": payload});
                return json.to_string();
            }
            PayloadType::BatukaVideoProcessedUpload(payload) => {
                let json = json!({"type": "BatukaVideoProcessedUpload", "payload": payload});
                return json.to_string();
            }
            PayloadType::BatukaSrtTranscriptionUpload(payload) => {
                let json = json!({"type": "BatukaSrtTranscriptionUpload", "payload": payload});
                return json.to_string();
            }
            PayloadType::BatukaSrtTranslationUpload(payload) => {
                let json = json!({"type": "BatukaSrtTranslationUpload", "payload": payload});
                return json.to_string();
            }
            PayloadType::BatukaDownloadVideo(payload) => {
                let json = json!({"type": "BatukaDownloadVideo", "payload": payload});
                return json.to_string();
            }
        }
    }

    pub fn video_ids(&self) -> Vec<Uuid> {
        match self {
            PayloadType::BatukaVideoRawUpload(payload) => vec![payload.video_id],
            PayloadType::BatukaVideoProcessedUpload(payload) => vec![payload.video_id],
            PayloadType::BatukaSrtTranscriptionUpload(payload) => vec![payload.video_id],
            PayloadType::BatukaSrtTranslationUpload(payload) => vec![payload.video_id],
            PayloadType::BatukaDownloadVideo(payload) => vec![payload.video_id],
        }
    }
}
