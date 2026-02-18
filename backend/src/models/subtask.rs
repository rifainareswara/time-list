use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Subtask {
    pub id: String,
    pub task_id: String,
    pub title: String,
    pub completed: bool,
    pub position: i32,
    pub created_at: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubtaskRequest {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubtaskRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub position: Option<i32>,
}
