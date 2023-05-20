use serde::Serialize;
use uuid::Uuid;

use crate::database::models::video_storage::VideoFormat;

#[derive(Debug, Serialize)]
pub struct VideoPayload {
    pub video_uri: String,
    pub video_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct SrtPayload {
    pub video_id: Uuid,
    pub srt_uri: String,
}

#[derive(Debug, Serialize)]
pub struct VideoDownloadPayload {
    pub video_url: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub video_format: VideoFormat,
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
            PayloadType::BatukaVideoRawUpload(payload) => serde_json::to_string(&payload).unwrap(),
            PayloadType::BatukaVideoProcessedUpload(payload) => {
                serde_json::to_string(&payload).unwrap()
            }
            PayloadType::BatukaSrtTranscriptionUpload(payload) => {
                serde_json::to_string(&payload).unwrap()
            }
            PayloadType::BatukaSrtTranslationUpload(payload) => {
                serde_json::to_string(&payload).unwrap()
            }
            PayloadType::BatukaDownloadVideo(payload) => serde_json::to_string(&payload).unwrap(),
        }
    }
}
