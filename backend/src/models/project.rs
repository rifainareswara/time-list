use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ProjectWithStats {
    pub id: String,
    pub name: String,
    pub color: String,
    pub description: String,
    pub created_at: String,
    #[serde(default)]
    #[sqlx(default)]
    pub task_count: i64,
    #[serde(default)]
    #[sqlx(default)]
    pub pending_count: i64,
    #[serde(default)]
    #[sqlx(default)]
    pub in_progress_count: i64,
    #[serde(default)]
    #[sqlx(default)]
    pub completed_count: i64,
    #[serde(default)]
    #[sqlx(default)]
    pub total_minutes: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
}
