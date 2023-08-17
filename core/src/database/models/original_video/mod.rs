use chrono::NaiveDateTime;
use marco_polo_rs_macros::{Filtrate, Paginate};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};

pub mod with;

#[derive(Debug, Filtrate, PartialEq, Paginate, Clone, Serialize, Deserialize, FromRow)]
pub struct OriginalVideo {
    pub id: i32,
    pub url: String,
    pub duration: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl OriginalVideo {
    pub fn from_row_alias(row: &PgRow, alias: &str) -> Result<Self, sqlx::Error> {
        let original = OriginalVideo {
            id: row.try_get(format!("{}.id", alias).as_str())?,
            url: row.try_get(format!("{}.url", alias).as_str())?,
            duration: row.try_get(format!("{}.duration", alias).as_str())?,
            created_at: row.try_get(format!("{}.created_at", alias).as_str())?,
            updated_at: row.try_get(format!("{}.updated_at", alias).as_str())?,
        };
        return Ok(original);
    }
}
