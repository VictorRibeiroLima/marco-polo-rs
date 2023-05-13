use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct VideosSubtitling {
    pub video_id: Uuid,
    pub subtitler_id: i32,
    pub subtitling_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
