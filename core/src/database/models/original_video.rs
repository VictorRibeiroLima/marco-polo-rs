use chrono::NaiveDateTime;
use marco_polo_rs_macros::{Filtrate, Paginate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Filtrate, PartialEq, Paginate, Clone, Serialize, Deserialize, FromRow)]
pub struct OriginalVideo {
    pub id: i32,
    pub url: String,
    pub duration: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
