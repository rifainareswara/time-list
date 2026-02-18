use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeEntry {
    pub id: String,
    pub task_id: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_minutes: i64,
    pub notes: String,
    pub created_at: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTimeEntryRequest {
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_minutes: i64,
    #[serde(default)]
    pub notes: Option<String>,
}
