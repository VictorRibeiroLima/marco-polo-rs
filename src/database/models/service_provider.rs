use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(
    type_name = "service_provider_type",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ServiceProviderType {
    Storage,
    Transcription,
    Translation,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceProvider {
    pub id: i32,
    pub name: String,
    pub service_provider_type: ServiceProviderType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
