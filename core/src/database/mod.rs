use sqlx::{migrate, PgPool};

pub mod models;
pub mod queries;

pub async fn create_pool() -> PgPool {
    println!("Creating database pool...");
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&database_url).await.unwrap();

    migrate!("../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    return pool;
}
