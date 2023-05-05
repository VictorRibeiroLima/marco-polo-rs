use marco_polo_rs_core::database::models::video_storage::VideoFormat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateVideo {
    pub title: String,
    pub description: String,
    pub video_url: String,
    pub channel_id: i32,
    pub language: Option<String>,
    pub format: Option<VideoFormat>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}
