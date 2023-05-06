use lazy_static::lazy_static;
use marco_polo_rs_core::database::models::video_storage::VideoFormat;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref YOUTUBE_URL: Regex = Regex::new(r#"^((?:https?:)?\/\/)?((?:www|m)\.)?((?:youtube\.com|youtu.be))(\/(?:[\w\-]+\?v=|embed\/|v\/)?)([\w\-]+)(\S+)?$"#).unwrap();
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct CreateVideo {
    pub title: String,
    pub description: String,
    #[validate(regex(path = "YOUTUBE_URL", message = "Invalid Youtube URL"))]
    pub video_url: String,
    pub channel_id: i32,
    pub language: Option<String>,
    pub format: Option<VideoFormat>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}
