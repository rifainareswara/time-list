use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardSummary {
    pub total_tasks: i64,
    pub completed_tasks: i64,
    pub pending_tasks: i64,
    pub in_progress_tasks: i64,
    pub total_minutes_today: i64,
    pub total_minutes_month: i64,
    pub total_entries_today: i64,
    pub recent_entries: Vec<RecentEntry>,
    pub project_stats: Vec<ProjectStat>,
    pub daily_minutes: Vec<DailyMinutes>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecentEntry {
    pub entry_id: String,
    pub task_id: String,
    pub task_title: String,
    pub duration_minutes: i64,
    pub notes: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProjectStat {
    pub name: String,
    pub color: String,
    pub task_count: i64,
    pub total_minutes: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DailyMinutes {
    pub date: Option<String>,
    pub minutes: Option<i64>,
}
