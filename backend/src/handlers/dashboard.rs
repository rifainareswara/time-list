use actix_web::{web, HttpRequest, HttpMessage, HttpResponse, Responder};
use sqlx::query_as;

use crate::db::DbPool;
use crate::models::dashboard::*;
use crate::models::user::AuthClaims;

pub async fn get_dashboard(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };

    let total_tasks: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tasks WHERE user_id = $1")
        .bind(&user_id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let completed_tasks: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tasks WHERE status = 'completed' AND user_id = $1")
        .bind(&user_id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let pending_tasks: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tasks WHERE status = 'pending' AND user_id = $1")
        .bind(&user_id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let in_progress_tasks: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tasks WHERE status = 'in_progress' AND user_id = $1")
        .bind(&user_id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let total_minutes_today: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(duration_minutes), 0)::BIGINT FROM time_entries WHERE CAST(created_at AS DATE) = CURRENT_DATE AND user_id = $1")
        .bind(&user_id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let total_minutes_month: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(duration_minutes), 0)::BIGINT FROM time_entries WHERE TO_CHAR(CAST(created_at AS DATE), 'YYYY-MM') = TO_CHAR(CURRENT_DATE, 'YYYY-MM') AND user_id = $1")
        .bind(&user_id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let total_entries_today: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM time_entries WHERE CAST(created_at AS DATE) = CURRENT_DATE AND user_id = $1")
        .bind(&user_id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    let recent_entries: Vec<RecentEntry> = query_as(
        "SELECT e.id as entry_id, e.task_id, t.title as task_title, e.duration_minutes, e.notes, e.created_at
         FROM time_entries e
         JOIN tasks t ON t.id = e.task_id
         WHERE e.user_id = $1
         ORDER BY e.created_at DESC
         LIMIT 10"
    )
    .bind(&user_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let project_stats: Vec<ProjectStat> = query_as(
        "SELECT p.name, p.color, COUNT(DISTINCT t.id)::BIGINT as task_count, COALESCE(SUM(e.duration_minutes), 0)::BIGINT as total_minutes
         FROM projects p
         LEFT JOIN tasks t ON t.project_id = p.id
         LEFT JOIN time_entries e ON e.task_id = t.id
         WHERE p.user_id = $1
         GROUP BY p.id, p.name, p.color
         ORDER BY total_minutes DESC"
    )
    .bind(&user_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let daily_minutes: Vec<DailyMinutes> = query_as(
        "SELECT SUBSTRING(created_at FROM 1 FOR 10) as date, SUM(duration_minutes)::BIGINT as minutes
         FROM time_entries
         WHERE CAST(created_at AS DATE) >= CURRENT_DATE - INTERVAL '6 days' AND user_id = $1
         GROUP BY date
         ORDER BY date"
    )
    .bind(&user_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let summary = DashboardSummary {
        total_tasks,
        completed_tasks,
        pending_tasks,
        in_progress_tasks,
        total_minutes_today,
        total_minutes_month,
        total_entries_today,
        recent_entries,
        project_stats,
        daily_minutes: daily_minutes.into_iter().map(|d| DailyMinutes {
            date: d.date.or(Some("".to_string())),
            minutes: d.minutes.or(Some(0)),
        }).collect(),
    };

    HttpResponse::Ok().json(summary)
}
