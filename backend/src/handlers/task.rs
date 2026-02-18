use actix_web::{web, HttpRequest, HttpMessage, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;
use sqlx::{query, query_as};

use crate::db::DbPool;
use crate::models::task::*;
use crate::models::user::AuthClaims;

// ─── Shared SQL fragments ───

const TASK_SELECT: &str =
    "SELECT t.id, t.title, t.description, t.category, t.status, t.priority,
            t.start_date, t.due_date, t.created_at, t.updated_at,
            t.project_id,
            p.name AS project_name,
            p.color AS project_color,
            t.user_id,
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
            t.user_id,
            COALESCE((SELECT SUM(duration_minutes) FROM time_entries WHERE task_id = t.id), 0)::BIGINT AS total_minutes,
            COALESCE((SELECT COUNT(*) FROM time_entries WHERE task_id = t.id), 0)::BIGINT AS entry_count,
            COALESCE((SELECT COUNT(*) FROM subtasks WHERE task_id = t.id), 0)::BIGINT AS subtask_count,
            COALESCE((SELECT COUNT(*) FROM subtasks WHERE task_id = t.id AND completed = TRUE), 0)::BIGINT AS subtask_done
     FROM tasks t
     LEFT JOIN projects p ON p.id = t.project_id
     WHERE t.id = $1";

// ─── GET /api/tasks ───

pub async fn get_tasks(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };

    // TASK_SELECT doesn't have WHERE, so we add it
    let sql = format!("{} WHERE t.user_id = $1 GROUP BY t.id, p.name, p.color ORDER BY t.updated_at DESC", TASK_SELECT);
    let result = query_as::<_, Task>(&sql)
        .bind(&user_id)
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
    req: HttpRequest,
    body: web::Json<CreateTaskRequest>,
) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let desc = body.description.clone().unwrap_or_default();
    let cat = body.category.clone().unwrap_or_else(|| "General".into());
    let priority = body.priority.clone().unwrap_or_else(|| "normal".into());
    
    // Handle "default" project_id from frontend by treating it as NULL
    let project_id_input = body.project_id.clone().unwrap_or_default();
    let project_id = if project_id_input.is_empty() || project_id_input == "default" {
        None
    } else {
        Some(project_id_input)
    };

    let result = query(
        "INSERT INTO tasks (id, title, description, category, status, priority, project_id, due_date, start_date, created_at, updated_at, user_id)
         VALUES ($1, $2, $3, $4, 'pending', $5, $6, $7, $8, $9, $10, $11)"
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
    .bind(&user_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            // Fetch the full task with project info
            // Ensure we fetch by ID. Since we just created it, ID is unique.
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

async fn update_field(pool: &DbPool, col: &str, val: &str, now: &str, task_id: &str, user_id: &str) -> bool {
    let sql = format!("UPDATE tasks SET {} = $1, updated_at = $2 WHERE id = $3 AND user_id = $4", col);
    query(&sql).bind(val).bind(now).bind(task_id).bind(user_id).execute(pool).await.is_ok()
}

async fn update_nullable_field(pool: &DbPool, col: &str, val: &str, now: &str, task_id: &str, user_id: &str) -> bool {
    let bind_val: Option<&str> = if val.is_empty() { None } else { Some(val) };
    let sql = format!("UPDATE tasks SET {} = $1, updated_at = $2 WHERE id = $3 AND user_id = $4", col);
    query(&sql).bind(bind_val).bind(now).bind(task_id).bind(user_id).execute(pool).await.is_ok()
}

pub async fn update_task(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateTaskRequest>,
) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let task_id = path.into_inner();
    let now = Utc::now().to_rfc3339();
    let db = pool.get_ref();

    if let Some(ref v) = body.title       { if !update_field(db, "title", v, &now, &task_id, &user_id).await       { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.description  { if !update_field(db, "description", v, &now, &task_id, &user_id).await { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.category     { if !update_field(db, "category", v, &now, &task_id, &user_id).await    { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.status       { if !update_field(db, "status", v, &now, &task_id, &user_id).await      { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.priority     { if !update_field(db, "priority", v, &now, &task_id, &user_id).await    { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.project_id   { if !update_field(db, "project_id", v, &now, &task_id, &user_id).await  { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.due_date     { if !update_nullable_field(db, "due_date", v, &now, &task_id, &user_id).await   { return HttpResponse::InternalServerError().finish(); } }
    if let Some(ref v) = body.start_date   { if !update_nullable_field(db, "start_date", v, &now, &task_id, &user_id).await { return HttpResponse::InternalServerError().finish(); } }

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

pub async fn delete_task(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let task_id = path.into_inner();
    let db = pool.get_ref();

    // Ensure ownership before delete
    // We can just add user_id AND to the DELETE. 
    // BUT we are wiping child tables too. We should verify ownership first OR add user_id to child deletes if they have user_id, 
    // OR just use CASCADE but we don't have CASCADE from user -> task deletion here, we are deleting TASK.
    // If we delete task where user_id=$1, and it works, fine.
    // But we need to delete subtasks/entries first or rely on CASCADE.
    // The DB schema has ON DELETE CASCADE for task_id. So we can just delete from TASKS where id=X AND user_id=Y.
    // However, existing code deletes children manually. Let's keep that but guard it?
    // Actually, if we delete task first, children go away.
    // Let's rely on checking ownership ONCE.

    let owner_check = query("SELECT id FROM tasks WHERE id = $1 AND user_id = $2")
        .bind(&task_id)
        .bind(&user_id)
        .fetch_optional(db)
        .await;

    if let Ok(Some(_)) = owner_check {
        query("DELETE FROM active_timers WHERE task_id = $1").bind(&task_id).execute(db).await.ok();
        query("DELETE FROM subtasks WHERE task_id = $1").bind(&task_id).execute(db).await.ok();
        query("DELETE FROM time_entries WHERE task_id = $1").bind(&task_id).execute(db).await.ok();
        query("DELETE FROM tasks WHERE id = $1").bind(&task_id).execute(db).await.ok();
        HttpResponse::Ok().json(serde_json::json!({"deleted": true}))
    } else {
         HttpResponse::NotFound().finish()
    }
}

// ─── POST /api/tasks/bulk-delete ───

#[derive(serde::Deserialize)]
pub struct BulkDeleteRequest {
    pub ids: Vec<String>,
}

pub async fn delete_tasks_bulk(pool: web::Data<DbPool>, req: HttpRequest, body: web::Json<BulkDeleteRequest>) -> impl Responder {
     let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let db = pool.get_ref();

    let mut deleted_count = 0;
    for id in &body.ids {
        // Verify ownership for each or just try delete with WHERE user_id
        // Since we are doing manual deletes of children, we should check ownership.
        // Optimization: DELETE FROM tasks WHERE id = $1 AND user_id = $2 RETURNING id
        // IF returned, then delete children? 
        // OR rely on CASCADE. Database has ON DELETE CASCADE.
        // "FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE"
        // So we ONLY need to delete from TASKS.
        // The previous code verified checking children deletion manually.
        // I will simplify to just delete from tasks with user_id check.
        
        let result = query("DELETE FROM tasks WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(&user_id)
            .execute(db)
            .await;
            
        if let Ok(res) = result {
            if res.rows_affected() > 0 {
                deleted_count += 1;
            }
        }
    }
    HttpResponse::Ok().json(serde_json::json!({"deleted_count": deleted_count}))
}

// ─── GET /api/admin/tasks (admin only) ───

pub async fn get_all_tasks_admin(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    // Role check — extract role before extensions temporary is dropped
    let role = {
        let ext = req.extensions();
        match ext.get::<AuthClaims>() {
            Some(c) => c.role.clone(),
            None => return HttpResponse::Unauthorized().finish(),
        }
    };
    if role != "admin" {
        return HttpResponse::Forbidden().body("Admin only");
    }

    let sql = "SELECT t.id, t.title, t.description, t.category, t.status, t.priority,
                      t.start_date, t.due_date, t.created_at, t.updated_at,
                      t.project_id,
                      p.name AS project_name,
                      p.color AS project_color,
                      t.user_id,
                      u.username,
                      u.full_name,
                      COALESCE(SUM(e.duration_minutes), 0)::BIGINT AS total_minutes,
                      COALESCE((SELECT COUNT(*) FROM subtasks WHERE task_id = t.id), 0)::BIGINT AS subtask_count,
                      COALESCE((SELECT COUNT(*) FROM subtasks WHERE task_id = t.id AND completed = TRUE), 0)::BIGINT AS subtask_done
               FROM tasks t
               LEFT JOIN time_entries e ON e.task_id = t.id
               LEFT JOIN projects p ON p.id = t.project_id
               JOIN users u ON u.id = t.user_id
               GROUP BY t.id, p.name, p.color, u.username, u.full_name
               ORDER BY t.updated_at DESC";

    let result = query_as::<_, crate::models::task::AdminTask>(sql)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => {
            eprintln!("get_all_tasks_admin error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

