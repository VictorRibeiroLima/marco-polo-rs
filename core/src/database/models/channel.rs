use chrono::NaiveDateTime;
use marco_polo_rs_macros::{Filtrate, Paginate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Filtrate, Paginate)]
pub struct Channel {
    pub id: i32,
    pub creator_id: i32,
    pub error: bool,
    pub name: Option<String>,
    pub csrf_token: Option<String>,
    pub refresh_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
