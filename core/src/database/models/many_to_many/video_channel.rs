use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VideoChannel {
    pub video_id: Uuid,
    pub channel_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
