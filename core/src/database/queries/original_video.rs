use sqlx::PgExecutor;

pub async fn create(trx: impl PgExecutor<'_>, url: impl Into<String>) -> Result<i32, sqlx::Error> {
    let url = url.into();
    let row = sqlx::query!(
        r#"
      INSERT INTO original_videos (url)
      VALUES ($1)
      RETURNING id
    "#,
        url
    )
    .fetch_one(trx)
    .await?;

    Ok(row.id)
}
