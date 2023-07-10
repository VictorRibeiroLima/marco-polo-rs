use chrono::{DateTime, Utc};
use marco_polo_rs_macros::Paginate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Paginate)]
pub struct Channel {
    pub id: i32,
    pub creator_id: i32,
    pub name: Option<String>,
    pub csrf_token: Option<String>,
    pub refresh_token: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
