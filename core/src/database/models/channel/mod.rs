use chrono::NaiveDateTime;
use marco_polo_rs_macros::{Filtrate, Paginate};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, types::Json, FromRow, Row};

use self::{auth::AuthType, platform::Platform};

use super::traits::FromRowAlias;

pub mod auth;
pub mod platform;

#[derive(Debug, Serialize, Deserialize, FromRow, Filtrate, Paginate)]
pub struct Channel {
    pub id: i32,
    pub creator_id: i32,
    pub error: bool,
    pub name: Option<String>,
    pub platform: Platform,
    #[filtrate(skip = true)]
    pub auth: Json<AuthType>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl FromRowAlias for Channel {
    fn from_row_alias(row: &PgRow, alias: &str) -> Result<Self, sqlx::Error> {
        let alias = alias.to_owned() + ".";
        let channel = Channel {
            id: row.try_get(format!("{}id", alias).as_str())?,
            creator_id: row.try_get(format!("{}creator_id", alias).as_str())?,
            error: row.try_get(format!("{}error", alias).as_str())?,
            name: row.try_get(format!("{}name", alias).as_str())?,
            platform: row.try_get(format!("{}platform", alias).as_str())?,
            auth: row.try_get(format!("{}auth", alias).as_str())?,
            created_at: row.try_get(format!("{}created_at", alias).as_str())?,
            updated_at: row.try_get(format!("{}updated_at", alias).as_str())?,
            deleted_at: row.try_get(format!("{}deleted_at", alias).as_str())?,
        };

        return Ok(channel);
    }
}
