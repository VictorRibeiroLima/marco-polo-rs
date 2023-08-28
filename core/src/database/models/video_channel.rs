use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use super::traits::FromRowAlias;

pub const ALIAS_COLUMNS: &'static str = r#"
    vc.video_id as "vc.video_id",
    vc.channel_id as "vc.channel_id",
    vc.uploaded as "vc.uploaded",
    vc.error as "vc.error",
    vc.created_at as "vc.created_at",
    vc.updated_at as "vc.updated_at",
    vc.uploaded_at as "vc.uploaded_at",
    vc.url as "vc.url
"#;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VideoChannel {
    pub video_id: Uuid,
    pub channel_id: i32,
    pub uploaded: bool,
    pub error: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub uploaded_at: Option<NaiveDateTime>,
    pub url: Option<String>,
}

impl FromRowAlias for VideoChannel {
    fn from_row_alias(row: &sqlx::postgres::PgRow, alias: &str) -> Result<Self, sqlx::Error> {
        let video_id = row.try_get(format!("{}.video_id", alias).as_str())?;
        let channel_id = row.try_get(format!("{}.channel_id", alias).as_str())?;
        let uploaded = row.try_get(format!("{}.uploaded", alias).as_str())?;
        let error = row.try_get(format!("{}.error", alias).as_str())?;
        let created_at = row.try_get(format!("{}.created_at", alias).as_str())?;
        let updated_at = row.try_get(format!("{}.updated_at", alias).as_str())?;
        let uploaded_at = row.try_get(format!("{}.uploaded_at", alias).as_str())?;
        let url = row.try_get(format!("{}.url", alias).as_str())?;

        return Ok(Self {
            video_id,
            channel_id,
            uploaded,
            error,
            created_at,
            updated_at,
            uploaded_at,
            url,
        });
    }
}
