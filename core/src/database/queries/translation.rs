use uuid::Uuid;

pub struct CreateTranslationDto<'a> {
    pub video_id: &'a Uuid,
    pub translator_id: i32,
    pub translation_id: Option<String>,
    pub storage_id: i32,
    pub path: &'a str,
}

pub async fn create<'a>(
    pool: &sqlx::PgPool,
    dto: CreateTranslationDto<'a>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO videos_translations (video_id, translator_id, translation_id, storage_id, path)
        VALUES ($1, $2, $3, $4, $5);
        "#,
        dto.video_id,
        dto.translator_id,
        dto.translation_id,
        dto.storage_id,
        dto.path
    )
    .execute(pool)
    .await?;

    Ok(())
}
