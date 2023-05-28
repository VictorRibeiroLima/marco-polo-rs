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
#[cfg(test)]
mod test {
    use std::str::FromStr;

    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::database::queries::translation::{create, CreateTranslationDto};

    #[sqlx::test(migrations = "../migrations", fixtures("videos", "service_providers"))]
    async fn test_create_translation(pool: PgPool) {
        let id = Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();

        let dto = CreateTranslationDto {
            video_id: &id,
            translator_id: 1234,
            translation_id: Some(String::from("id_translation")),
            storage_id: 5678,
            path: "../translation",
        };

        let test = create(&pool, dto).await;
        assert!(test.is_ok());

        let count = sqlx::query!(
            "SELECT COUNT(*) AS count FROM videos_translations WHERE video_id = $1",
            id
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(count.count.unwrap(), 1);
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn test_create_translation_if_foreign_key(pool: PgPool) {
        let id = Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();

        let dto = CreateTranslationDto {
            video_id: &id,
            translator_id: 1234,
            translation_id: Some(String::from("id_translation")),
            storage_id: 5678,
            path: "../translation",
        };

        let test = create(&pool, dto).await;
        assert!(test.is_err());
    }
}
