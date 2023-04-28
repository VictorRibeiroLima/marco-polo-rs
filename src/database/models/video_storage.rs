#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "video_format", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VideoFormat {
    Mp4,
    Webm,
    Ogg,
}
