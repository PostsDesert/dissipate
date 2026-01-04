use sqlx::{SqlitePool, Row};
use anyhow::Result;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub async fn create_pool() -> Result<SqlitePool> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:dissipate.db".to_string());
    let pool = SqlitePool::connect(&database_url).await?;
    Ok(pool)
}

pub async fn init_schema(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            username TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            salt TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
        CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);

        CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_messages_user_id ON messages(user_id);
        CREATE INDEX IF NOT EXISTS idx_messages_created_at ON messages(created_at DESC);
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn create_user(
    pool: &SqlitePool,
    id: &Uuid,
    email: &str,
    username: &str,
    password_hash: &str,
    salt: &str,
    created_at: &DateTime<Utc>,
    updated_at: &DateTime<Utc>,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO users (id, email, username, password_hash, salt, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(id.to_string())
    .bind(email)
    .bind(username)
    .bind(password_hash)
    .bind(salt)
    .bind(created_at.to_rfc3339())
    .bind(updated_at.to_rfc3339())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_user_by_email(pool: &SqlitePool, email: &str) -> Result<Option<(Uuid, String, String, String, String, DateTime<Utc>, DateTime<Utc>)>> {
    let row = sqlx::query(
        r#"
        SELECT id, email, username, password_hash, salt, created_at, updated_at
        FROM users
        WHERE email = ?
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        Ok(Some((
            Uuid::parse_str(&row.get::<String, _>("id"))?,
            row.get("email"),
            row.get("username"),
            row.get("password_hash"),
            row.get("salt"),
            DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
            DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
        )))
    } else {
        Ok(None)
    }
}

pub async fn get_user_by_id(pool: &SqlitePool, user_id: &Uuid) -> Result<Option<(String, String, DateTime<Utc>, DateTime<Utc>)>> {
    let row = sqlx::query(
        r#"
        SELECT email, username, created_at, updated_at
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(user_id.to_string())
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        Ok(Some((
            row.get("email"),
            row.get("username"),
            DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
            DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
        )))
    } else {
        Ok(None)
    }
}

pub async fn update_user_email(pool: &SqlitePool, user_id: &Uuid, email: &str) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE users SET email = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(email)
    .bind(Utc::now().to_rfc3339())
    .bind(user_id.to_string())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_user_username(pool: &SqlitePool, user_id: &Uuid, username: &str) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE users SET username = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(username)
    .bind(Utc::now().to_rfc3339())
    .bind(user_id.to_string())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_user_password(pool: &SqlitePool, user_id: &Uuid, password_hash: &str, salt: &str) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE users SET password_hash = ?, salt = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(password_hash)
    .bind(salt)
    .bind(Utc::now().to_rfc3339())
    .bind(user_id.to_string())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn create_message(
    pool: &SqlitePool,
    id: &Uuid,
    user_id: &Uuid,
    content: &str,
    created_at: &DateTime<Utc>,
    updated_at: &DateTime<Utc>,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO messages (id, user_id, content, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(id.to_string())
    .bind(user_id.to_string())
    .bind(content)
    .bind(created_at.to_rfc3339())
    .bind(updated_at.to_rfc3339())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_user_messages(pool: &SqlitePool, user_id: &Uuid) -> Result<Vec<(Uuid, Uuid, String, DateTime<Utc>, DateTime<Utc>)>> {
    let rows = sqlx::query(
        r#"
        SELECT id, user_id, content, created_at, updated_at
        FROM messages
        WHERE user_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id.to_string())
    .fetch_all(pool)
    .await?;

    let messages = rows
        .iter()
        .map(|row| {
            Ok((
                Uuid::parse_str(&row.get::<String, _>("id"))?,
                Uuid::parse_str(&row.get::<String, _>("user_id"))?,
                row.get("content"),
                DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(messages)
}

pub async fn get_message(pool: &SqlitePool, message_id: &Uuid) -> Result<Option<(Uuid, Uuid, String, DateTime<Utc>, DateTime<Utc>)>> {
    let row = sqlx::query(
        r#"
        SELECT id, user_id, content, created_at, updated_at
        FROM messages
        WHERE id = ?
        "#,
    )
    .bind(message_id.to_string())
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        Ok(Some((
            Uuid::parse_str(&row.get::<String, _>("id"))?,
            Uuid::parse_str(&row.get::<String, _>("user_id"))?,
            row.get("content"),
            DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
            DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
        )))
    } else {
        Ok(None)
    }
}

pub async fn update_message(pool: &SqlitePool, message_id: &Uuid, content: &str) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE messages SET content = ?, updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(content)
    .bind(Utc::now().to_rfc3339())
    .bind(message_id.to_string())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_message(pool: &SqlitePool, message_id: &Uuid) -> Result<()> {
    sqlx::query(
        r#"
        DELETE FROM messages WHERE id = ?
        "#,
    )
    .bind(message_id.to_string())
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth;

    async fn create_test_pool() -> Result<SqlitePool> {
        let pool = SqlitePool::connect("sqlite::memory:").await?;
        init_schema(&pool).await?;
        Ok(pool)
    }

    #[tokio::test]
    async fn test_create_pool() {
        let pool = create_test_pool().await;
        assert!(pool.is_ok());
    }

    #[tokio::test]
    async fn test_init_schema() {
        let pool = create_test_pool().await.unwrap();
        let result = init_schema(&pool).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_schema_tables_exist() {
        let pool = create_test_pool().await.unwrap();
        
        let users_exist: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='users')"
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        
        let messages_exist: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='messages')"
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        
        assert!(users_exist);
        assert!(messages_exist);
    }

    #[tokio::test]
    async fn test_create_and_get_user() {
        let pool = create_test_pool().await.unwrap();
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        let password_hash = auth::hash_password("password123").unwrap();
        let salt = "test_salt".to_string();

        create_user(
            &pool,
            &user_id,
            "test@example.com",
            "testuser",
            &password_hash,
            &salt,
            &now,
            &now,
        )
        .await
        .unwrap();

        let user = get_user_by_email(&pool, "test@example.com")
            .await
            .unwrap();
        
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.0, user_id);
        assert_eq!(user.1, "test@example.com");
        assert_eq!(user.2, "testuser");
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
        let pool = create_test_pool().await.unwrap();
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        let password_hash = auth::hash_password("password123").unwrap();
        let salt = "test_salt".to_string();

        create_user(
            &pool,
            &user_id,
            "test@example.com",
            "testuser",
            &password_hash,
            &salt,
            &now,
            &now,
        )
        .await
        .unwrap();

        let user = get_user_by_id(&pool, &user_id)
            .await
            .unwrap();
        
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.0, "test@example.com");
        assert_eq!(user.1, "testuser");
    }

    #[tokio::test]
    async fn test_create_and_get_message() {
        let pool = create_test_pool().await.unwrap();
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        let password_hash = auth::hash_password("password123").unwrap();
        let salt = "test_salt".to_string();

        create_user(
            &pool,
            &user_id,
            "test@example.com",
            "testuser",
            &password_hash,
            &salt,
            &now,
            &now,
        )
        .await
        .unwrap();

        let message_id = Uuid::new_v4();
        create_message(
            &pool,
            &message_id,
            &user_id,
            "Hello, world!",
            &now,
            &now,
        )
        .await
        .unwrap();

        let message = get_message(&pool, &message_id)
            .await
            .unwrap();
        
        assert!(message.is_some());
        let message = message.unwrap();
        assert_eq!(message.0, message_id);
        assert_eq!(message.2, "Hello, world!");
    }

    #[tokio::test]
    async fn test_get_user_messages() {
        let pool = create_test_pool().await.unwrap();
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        let password_hash = auth::hash_password("password123").unwrap();
        let salt = "test_salt".to_string();

        create_user(
            &pool,
            &user_id,
            "test@example.com",
            "testuser",
            &password_hash,
            &salt,
            &now,
            &now,
        )
        .await
        .unwrap();

        let msg1_id = Uuid::new_v4();
        let msg2_id = Uuid::new_v4();
        create_message(&pool, &msg1_id, &user_id, "First message", &now, &now)
            .await
            .unwrap();
        create_message(&pool, &msg2_id, &user_id, "Second message", &now, &now)
            .await
            .unwrap();

        let messages = get_user_messages(&pool, &user_id)
            .await
            .unwrap();
        
        assert_eq!(messages.len(), 2);
    }

    #[tokio::test]
    async fn test_update_message() {
        let pool = create_test_pool().await.unwrap();
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        let password_hash = auth::hash_password("password123").unwrap();
        let salt = "test_salt".to_string();

        create_user(
            &pool,
            &user_id,
            "test@example.com",
            "testuser",
            &password_hash,
            &salt,
            &now,
            &now,
        )
        .await
        .unwrap();

        let message_id = Uuid::new_v4();
        create_message(
            &pool,
            &message_id,
            &user_id,
            "Hello, world!",
            &now,
            &now,
        )
        .await
        .unwrap();

        update_message(&pool, &message_id, "Updated content")
            .await
            .unwrap();

        let message = get_message(&pool, &message_id)
            .await
            .unwrap();
        
        assert_eq!(message.unwrap().2, "Updated content");
    }

    #[tokio::test]
    async fn test_delete_message() {
        let pool = create_test_pool().await.unwrap();
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        let password_hash = auth::hash_password("password123").unwrap();
        let salt = "test_salt".to_string();

        create_user(
            &pool,
            &user_id,
            "test@example.com",
            "testuser",
            &password_hash,
            &salt,
            &now,
            &now,
        )
        .await
        .unwrap();

        let message_id = Uuid::new_v4();
        create_message(
            &pool,
            &message_id,
            &user_id,
            "Hello, world!",
            &now,
            &now,
        )
        .await
        .unwrap();

        delete_message(&pool, &message_id)
            .await
            .unwrap();

        let message = get_message(&pool, &message_id)
            .await
            .unwrap();
        
        assert!(message.is_none());
    }

    #[tokio::test]
    async fn test_update_user_email() {
        let pool = create_test_pool().await.unwrap();
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        let password_hash = auth::hash_password("password123").unwrap();
        let salt = "test_salt".to_string();

        create_user(
            &pool,
            &user_id,
            "test@example.com",
            "testuser",
            &password_hash,
            &salt,
            &now,
            &now,
        )
        .await
        .unwrap();

        update_user_email(&pool, &user_id, "newemail@example.com")
            .await
            .unwrap();

        let user = get_user_by_id(&pool, &user_id)
            .await
            .unwrap();
        
        assert_eq!(user.unwrap().0, "newemail@example.com");
    }

    #[tokio::test]
    async fn test_update_user_username() {
        let pool = create_test_pool().await.unwrap();
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        let password_hash = auth::hash_password("password123").unwrap();
        let salt = "test_salt".to_string();

        create_user(
            &pool,
            &user_id,
            "test@example.com",
            "testuser",
            &password_hash,
            &salt,
            &now,
            &now,
        )
        .await
        .unwrap();

        update_user_username(&pool, &user_id, "newusername")
            .await
            .unwrap();

        let user = get_user_by_id(&pool, &user_id)
            .await
            .unwrap();
        
        assert_eq!(user.unwrap().1, "newusername");
    }
}
