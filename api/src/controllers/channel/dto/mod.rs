use chrono::NaiveDateTime;
use marco_polo_rs_core::database::models::channel::Channel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OauthQueryParams {
    pub code: String,
    pub state: String,
    pub scope: String,
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct ChannelDTO {
    pub id: i32,
    pub creator_id: i32,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Channel> for ChannelDTO {
    fn from(value: Channel) -> Self {
        return Self {
            id: value.id,
            name: value.name,
            creator_id: value.creator_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        };
    }
}
