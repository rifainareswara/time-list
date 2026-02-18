use actix_web::{web, HttpRequest, HttpMessage, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::{query, query_as};

use crate::db::DbPool;
use crate::models::timer::*;
use crate::models::user::AuthClaims;

pub async fn start_timer(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<StartTimerRequest>,
) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let task_id = path.into_inner();
    let now = Utc::now();
    let now_str = now.to_rfc3339();

    // 1. Check if there's already an active timer FOR THIS USER — stop it first
    let existing: Option<(String, String, String, String)> = query_as(
        "SELECT id, task_id, start_time, notes FROM active_timers WHERE user_id = $1 LIMIT 1"
    )
    .bind(&user_id)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None);

    if let Some((timer_id, old_task_id, start_time_str, old_notes)) = existing {
        // Calculate duration
        let start: DateTime<Utc> = start_time_str.parse().unwrap_or(now);
        let duration_secs = (now - start).num_seconds().max(0);
        let duration_minutes = (duration_secs as f64 / 60.0).ceil() as i64;

        // Create a time_entry from the stopped timer
        let entry_id = Uuid::new_v4().to_string();
        query(
            "INSERT INTO time_entries (id, task_id, start_time, end_time, duration_minutes, notes, created_at, user_id) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)"
        )
        .bind(&entry_id).bind(&old_task_id).bind(&start_time_str).bind(&now_str).bind(duration_minutes).bind(&old_notes).bind(&now_str).bind(&user_id)
        .execute(pool.get_ref()).await.ok();

        // Delete the old timer
        query("DELETE FROM active_timers WHERE id = $1").bind(&timer_id).execute(pool.get_ref()).await.ok();

        // Update old task
        // Ensure we only update if it belongs to user? active_timer was user's, so task should be too.
        query("UPDATE tasks SET status = 'pending', updated_at = $1 WHERE id = $2 AND status = 'in_progress'")
            .bind(&now_str).bind(&old_task_id).execute(pool.get_ref()).await.ok();
    }

    // 2. Create new active timer
    let id = Uuid::new_v4().to_string();
    let notes = body.notes.clone().unwrap_or_default();

    query(
        "INSERT INTO active_timers (id, task_id, start_time, notes, created_at, user_id) VALUES ($1,$2,$3,$4,$5,$6)"
    )
    .bind(&id).bind(&task_id).bind(&now_str).bind(&notes).bind(&now_str).bind(&user_id)
    .execute(pool.get_ref()).await.unwrap();

    // Set task to in_progress
    query("UPDATE tasks SET status = 'in_progress', updated_at = $1 WHERE id = $2 AND user_id = $3")
        .bind(&now_str).bind(&task_id).bind(&user_id).execute(pool.get_ref()).await.ok();

    // Fetch task title
    let task_title: String = sqlx::query_scalar("SELECT title FROM tasks WHERE id = $1")
        .bind(&task_id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or_default();

    let timer = ActiveTimer {
        id,
        task_id,
        task_title,
        start_time: now_str.clone(),
        notes,
        elapsed_seconds: 0,
        created_at: now_str,
    };

    HttpResponse::Ok().json(timer)
}

pub async fn stop_timer(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let now = Utc::now();
    let now_str = now.to_rfc3339();

    let existing: Option<(String, String, String, String)> = query_as(
        "SELECT id, task_id, start_time, notes FROM active_timers WHERE user_id = $1 LIMIT 1"
    )
    .bind(&user_id)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None);

    match existing {
        Some((timer_id, task_id, start_time_str, notes)) => {
            let start: DateTime<Utc> = start_time_str.parse().unwrap_or(now);
            let duration_secs = (now - start).num_seconds().max(0);
            let duration_minutes = (duration_secs as f64 / 60.0).ceil() as i64;

            // Create time entry
            let entry_id = Uuid::new_v4().to_string();
            query(
                "INSERT INTO time_entries (id, task_id, start_time, end_time, duration_minutes, notes, created_at, user_id) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)"
            )
            .bind(&entry_id).bind(&task_id).bind(&start_time_str).bind(&now_str).bind(duration_minutes).bind(&notes).bind(&now_str).bind(&user_id)
            .execute(pool.get_ref()).await.ok();

            // Delete timer
            query("DELETE FROM active_timers WHERE id = $1").bind(&timer_id).execute(pool.get_ref()).await.ok();

            // Update task status back to pending
            query("UPDATE tasks SET status = 'pending', updated_at = $1 WHERE id = $2")
                .bind(&now_str).bind(&task_id).execute(pool.get_ref()).await.ok();

            HttpResponse::Ok().json(serde_json::json!({
                "stopped": true,
                "task_id": task_id,
                "duration_minutes": duration_minutes,
                "entry_id": entry_id
            }))
        }
        None => HttpResponse::Ok().json(serde_json::json!({"stopped": false, "message": "No active timer"}))
    }
}

pub async fn get_active_timer(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let now = Utc::now();

    let result: Option<(String, String, String, String, String, String)> = query_as(
        "SELECT at.id, at.task_id, t.title, at.start_time, at.notes, at.created_at
         FROM active_timers at
         JOIN tasks t ON t.id = at.task_id
         WHERE at.user_id = $1
         LIMIT 1"
    )
    .bind(&user_id)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None);

    match result {
        Some((id, task_id, task_title, start_time_str, notes, created_at)) => {
            let start: DateTime<Utc> = start_time_str.parse().unwrap_or(now);
            let elapsed = (now - start).num_seconds().max(0);

            let timer = ActiveTimer {
                id,
                task_id,
                task_title,
                start_time: start_time_str,
                notes,
                elapsed_seconds: elapsed,
                created_at,
            };

            HttpResponse::Ok().json(serde_json::json!({"active": true, "timer": timer}))
        }
        None => HttpResponse::Ok().json(serde_json::json!({"active": false, "timer": null}))
    }
}
