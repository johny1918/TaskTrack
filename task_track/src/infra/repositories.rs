use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

pub struct UserRepo {
    pool: PgPool,
}

impl UserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert_user(&self, email: &str, password_hash: &str)
        -> anyhow::Result<Uuid> {
            let rec= sqlx::query!(
                r#"
                INSERT INTO users(email,password_hash)
                VALUES($1, $2)
                RETURNING id
                "#,
                email,
                password_hash
            )
            .fetch_one(&self.pool)
            .await?;
            Ok(rec.id)
        }
}