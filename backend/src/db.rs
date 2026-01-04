use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use anyhow::Result;

pub async fn create_pool() -> Result<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:dissipate.db".to_string());
    let pool = SqlitePool::connect(&database_url).await?;
    Ok(pool)
}
