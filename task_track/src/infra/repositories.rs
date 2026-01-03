use anyhow::Ok;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct UserRepo {
   pub pool: PgPool,
}

pub struct TaskRepo {
    pub pool: PgPool,
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

    pub async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<(uuid::Uuid, String)>> {
        let rec = sqlx::query!(
            r#"SELECT id, password_hash FROM users WHERE email = $1"#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(r) = rec {
            Ok(Some((r.id, r.password_hash)))
        }else {
            Ok(None)
        }
    }

}

impl TaskRepo {
    pub fn new (pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn create_task(&self, title: &str, descriptions: Option<String>, status: &str)
        -> anyhow::Result<uuid::Uuid> {
            let rec = sqlx::query!(
                r#"
                INSERT INTO tasks(title, description, status)
                VALUES($1, $2, $3)
                RETURNING id
                "#,
                title,
                descriptions,
                status
            )
            .fetch_one(&self.pool)
            .await?;
        Ok(rec.id)
    }


}