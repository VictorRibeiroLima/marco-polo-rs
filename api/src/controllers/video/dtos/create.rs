use marco_polo_rs_core::database::models::video_storage::VideoFormat;

pub struct CreateVideo {
    pub title: String,
    pub description: String,
    pub video_url: String,
    pub channel_id: i32,
    pub format: Option<VideoFormat>,
}
