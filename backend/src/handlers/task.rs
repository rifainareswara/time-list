use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;
use sqlx::{query, query_as};

use crate::db::DbPool;
use crate::models::task::*;

// ─── Shared SQL fragments ───

const TASK_SELECT: &str =
    "SELECT t.id, t.title, t.description, t.category, t.status, t.priority,
            t.start_date, t.due_date, t.created_at, t.updated_at,
            t.project_id,
            p.name AS project_name,
            p.color AS project_color,
            COALESCE(SUM(e.duration_minutes), 0)::BIGINT AS total_minutes,
            COUNT(e.id)::BIGINT AS entry_count,
            COALESCE((SELECT COUNT(*) FROM subtasks WHERE task_id = t.id), 0)::BIGINT AS subtask_count,
            COALESCE((SELECT COUNT(*) FROM subtasks WHERE task_id = t.id AND completed = TRUE), 0)::BIGINT AS subtask_done
     FROM tasks t
     LEFT JOIN time_entries e ON e.task_id = t.id
     LEFT JOIN projects p ON p.id = t.project_id";

const TASK_SELECT_SINGLE: &str =
    "SELECT t.id, t.title, t.description, t.category, t.status, t.priority,
            t.start_date, t.due_date, t.created_at, t.updated_at,
            t.project_id,
            p.name AS project_name,
            p.color AS project_color,
            COALESCE((SELECT SUM(duration_minutes) FROM time_entries WHERE task_id = t.id), 0)::BIGINT AS total_minutes,
            COALESCE((SELECT COUNT(*) FROM time_entries WHERE task_id = t.id), 0)::BIGINT AS entry_count,
            COALESCE((SELECT COUNT(*) FROM subtasks WHERE task_id = t.id), 0)::BIGINT AS subtask_count,
            COALESCE((SELECT COUNT(*) FROM subtasks WHERE task_id = t.id AND completed = TRUE), 0)::BIGINT AS subtask_done
     FROM tasks t
     LEFT JOIN projects p ON p.id = t.project_id
     WHERE t.id = $1";

// ─── GET /api/tasks ───

pub async fn get_tasks(pool: web::Data<DbPool>) -> impl Responder {
    let sql = format!("{} GROUP BY t.id, p.name, p.color ORDER BY t.updated_at DESC", TASK_SELECT);
    let result = query_as::<_, Task>(&sql)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => {
            eprintln!("get_tasks error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// ─── POST /api/tasks ───

pub async fn create_task(
    pool: web::Data<DbPool>,
    body: web::Json<CreateTaskRequest>,
) -> impl Responder {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let desc = body.description.clone().unwrap_or_default();
    let cat = body.category.clone().unwrap_or_else(|| "General".into());
    let priority = body.priority.clone().unwrap_or_else(|| "normal".into());
    let project_id = body.project_id.clone().unwrap_or_else(|| "default".into());

    let result = query(
        "INSERT INTO tasks (id, title, description, category, status, priority, project_id, due_date, start_date, created_at, updated_at)
         VALUES ($1, $2, $3, $4, 'pending', $5, $6, $7, $8, $9, $10)"
    )
    .bind(&id)
    .bind(&body.title)
    .bind(&desc)
    .bind(&cat)
    .bind(&priority)
    .bind(&project_id)
    .bind(&body.due_date)
    .bind(&body.start_date)
    .bind(&now)
    .bind(&now)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            // Fetch the full task with project info
            let task = query_as::<_, Task>(TASK_SELECT_SINGLE)
                .bind(&id)
                .fetch_one(pool.get_ref())
                .await;
            match task {
                Ok(t) => HttpResponse::Created().json(t),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(e) => {
            eprintln!("create_task error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// ─── PUT /api/tasks/{id} ───

async fn update_field(pool: &DbPool, col: &str, val: &str, now: &str, task_id: &str) -> bool {
    let sql = format!("UPDATE tasks SET {} = $1, updated_at = $2 WHERE id = $3", col);
    query(&sql).bind(val).bind(now).bind(task_id).execute(pool).await.is_ok()
}

async fn update_nullable_field(pool: &DbPool, col: &str, val: &str, now: &str, task_id: &str) -> bool {
    let bind_val: Option<&str> = if val.is_empty() { None } else { Some(val) };
    let sql = format!("UPDATE tasks SET {} = $1, updated_at = $2 WHERE id = $3", col);
    query(&sql).bind(bind_val).bind(now).bind(task_id).execute(pool).await.is_ok()
}

pub async fn update_task(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<UpdateTaskRequest>,
) -> impl Responder {
    let task_id = path.into_inner();
    let now = Utc::now().to_rfc3339();
    let db = pool.get_ref();

    if let Some(ref v) = body.title       { if !update_field(db, "title", v, &now, &task_id).await       { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.description  { if !update_field(db, "description", v, &now, &task_id).await { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.category     { if !update_field(db, "category", v, &now, &task_id).await    { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.status       { if !update_field(db, "status", v, &now, &task_id).await      { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.priority     { if !update_field(db, "priority", v, &now, &task_id).await    { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.project_id   { if !update_field(db, "project_id", v, &now, &task_id).await  { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.due_date     { if !update_nullable_field(db, "due_date", v, &now, &task_id).await   { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.start_date   { if !update_nullable_field(db, "start_date", v, &now, &task_id).await { return HttpResponse::InternalServerError().finish(); } }

    let task = query_as::<_, Task>(TASK_SELECT_SINGLE)
        .bind(&task_id)
        .fetch_one(pool.get_ref())
        .await;

    match task {
        Ok(t) => HttpResponse::Ok().json(t),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// ─── DELETE /api/tasks/{id} ───

pub async fn delete_task(pool: web::Data<DbPool>, path: web::Path<String>) -> impl Responder {
    let task_id = path.into_inner();
    let db = pool.get_ref();

    query("DELETE FROM active_timers WHERE task_id = $1").bind(&task_id).execute(db).await.ok();
    query("DELETE FROM subtasks WHERE task_id = $1").bind(&task_id).execute(db).await.ok();
    query("DELETE FROM time_entries WHERE task_id = $1").bind(&task_id).execute(db).await.ok();
    query("DELETE FROM tasks WHERE id = $1").bind(&task_id).execute(db).await.ok();

    HttpResponse::Ok().json(serde_json::json!({"deleted": true}))
}

// ─── POST /api/tasks/bulk-delete ───

#[derive(serde::Deserialize)]
pub struct BulkDeleteRequest {
    pub ids: Vec<String>,
}

pub async fn delete_tasks_bulk(pool: web::Data<DbPool>, body: web::Json<BulkDeleteRequest>) -> impl Responder {
    let db = pool.get_ref();
    for id in &body.ids {
        query("DELETE FROM active_timers WHERE task_id = $1").bind(id).execute(db).await.ok();
        query("DELETE FROM subtasks WHERE task_id = $1").bind(id).execute(db).await.ok();
        query("DELETE FROM time_entries WHERE task_id = $1").bind(id).execute(db).await.ok();
        query("DELETE FROM tasks WHERE id = $1").bind(id).execute(db).await.ok();
    }
    HttpResponse::Ok().json(serde_json::json!({"deleted_count": body.ids.len()}))
}
