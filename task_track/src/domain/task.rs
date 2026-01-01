use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    ToDo,
    InProgress,
    Done
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub status: TaskStatus,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}