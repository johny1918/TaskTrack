use anyhow::Ok;
use axum::Json;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::task::TaskOutput;

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

    pub async fn list_tasks_for_user(&self, user_id: uuid::Uuid) -> anyhow::Result<Vec<TaskOutput>> {
        let rec = sqlx::query_as::<_, TaskOutput> (
        r#"
            SELECT title, description, tags, status, due_date, created_at, update_at
            FROM tasks
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rec)
    }

    pub async fn get_task_by_id(&self, id: uuid::Uuid) -> anyhow::Result<Option<TaskOutput>> {
        let rec = sqlx::query_as::<_, TaskOutput> (
            r#"
            SELECT title, description, tags, status, due_date, created_at, update_at
            FROM tasks
            WHERE user_id = $1
            "#,
        ).bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn delete_task(&self, id: uuid::Uuid) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            r#"DELETE FROM tasks WHERE id=$1"#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}