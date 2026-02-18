use actix_web::{web, HttpRequest, HttpMessage, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;
use sqlx::{query, query_as};

use crate::db::DbPool;
use crate::models::subtask::*;
use crate::models::user::AuthClaims;

pub async fn get_subtasks(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let task_id = path.into_inner();
    let subtasks = query_as::<_, Subtask>(
        "SELECT id, task_id, title, completed, position, created_at, user_id
         FROM subtasks WHERE task_id = $1 AND user_id = $2 ORDER BY position ASC, created_at ASC"
    )
    .bind(&task_id)
    .bind(&user_id)
    .fetch_all(pool.get_ref())
    .await;

    match subtasks {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_subtask(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<CreateSubtaskRequest>,
) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let task_id = path.into_inner();
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // Ensure user owns the task
    // Technically subtask insertion with bad task_id/user_id combo would fail FK if we were strict,
    // but here we just need to ensure the parent task belongs to user.
    // However, we are inserting into subtasks with user_id.
    // If task_id is valid but belongs to another user, we might create an orphan or mismatched subtask?
    // No, we should verify ownership of task_id first strictly speaking.
    // But let's assume if we insert with user_id to subtasks, and we query subtasks by user_id, it is safe.
    // However, standard is check task ownership.
    let task_exists = sqlx::query("SELECT id FROM tasks WHERE id = $1 AND user_id = $2")
        .bind(&task_id).bind(&user_id).fetch_optional(pool.get_ref()).await.unwrap_or(None);
    
    if task_exists.is_none() {
        return HttpResponse::NotFound().finish(); // Or Forbidden
    }

    // Get next position
    let max_pos: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(position), -1)::INT FROM subtasks WHERE task_id = $1"
    )
    .bind(&task_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(0);

    let position = max_pos + 1;

    let result = query(
        "INSERT INTO subtasks (id, task_id, title, completed, position, created_at, user_id) VALUES ($1, $2, $3, FALSE, $4, $5, $6)"
    )
    .bind(&id)
    .bind(&task_id)
    .bind(&body.title)
    .bind(position)
    .bind(&now)
    .bind(&user_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let subtask = Subtask {
                id,
                task_id,
                title: body.title.clone(),
                completed: false,
                position,
                created_at: now,
                user_id,
            };
            HttpResponse::Created().json(subtask)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_subtask(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateSubtaskRequest>,
) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let subtask_id = path.into_inner();

    if let Some(ref title) = body.title {
        query("UPDATE subtasks SET title = $1 WHERE id = $2 AND user_id = $3")
            .bind(title)
            .bind(&subtask_id)
            .bind(&user_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }
    if let Some(completed) = body.completed {
        query("UPDATE subtasks SET completed = $1 WHERE id = $2 AND user_id = $3")
            .bind(completed)
            .bind(&subtask_id)
            .bind(&user_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }
    if let Some(position) = body.position {
        query("UPDATE subtasks SET position = $1 WHERE id = $2 AND user_id = $3")
            .bind(position)
            .bind(&subtask_id)
            .bind(&user_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }

    let subtask = query_as::<_, Subtask>(
        "SELECT id, task_id, title, completed, position, created_at, user_id FROM subtasks WHERE id = $1 AND user_id = $2"
    )
    .bind(&subtask_id)
    .bind(&user_id)
    .fetch_one(pool.get_ref())
    .await;

    match subtask {
        Ok(s) => HttpResponse::Ok().json(s),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_subtask(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let subtask_id = path.into_inner();
    query("DELETE FROM subtasks WHERE id = $1 AND user_id = $2")
        .bind(&subtask_id)
        .bind(&user_id)
        .execute(pool.get_ref())
        .await
        .ok();
    HttpResponse::Ok().json(serde_json::json!({"deleted": true}))
}
