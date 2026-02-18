use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub status: String,
    #[serde(default = "default_priority")]
    #[sqlx(default)]
    pub priority: String,
    pub user_id: String,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    #[sqlx(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    #[sqlx(default)]
    pub project_name: Option<String>,
    #[serde(default)]
    #[sqlx(default)]
    pub project_color: Option<String>,
    #[serde(default)]
    #[sqlx(default)]
    pub total_minutes: i64,
    #[serde(default)]
    #[sqlx(default)]
    pub entry_count: i64,
    #[serde(default)]
    #[sqlx(default)]
    pub subtask_count: i64,
    #[serde(default)]
    #[sqlx(default)]
    pub subtask_done: i64,
}

fn default_priority() -> String {
    "normal".into()
}

/// Used by the admin overview endpoint — includes the owner's username.
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct AdminTask {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub status: String,
    #[serde(default = "default_priority")]
    #[sqlx(default)]
    pub priority: String,
    pub user_id: String,
    pub username: String,
    pub full_name: String,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    #[sqlx(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    #[sqlx(default)]
    pub project_name: Option<String>,
    #[serde(default)]
    #[sqlx(default)]
    pub project_color: Option<String>,
    #[serde(default)]
    #[sqlx(default)]
    pub total_minutes: i64,
    #[serde(default)]
    #[sqlx(default)]
    pub subtask_count: i64,
    #[serde(default)]
    #[sqlx(default)]
    pub subtask_done: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub project_id: Option<String>,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
}
