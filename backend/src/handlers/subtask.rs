use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;
use sqlx::{query, query_as};

use crate::db::DbPool;
use crate::models::subtask::*;

pub async fn get_subtasks(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let task_id = path.into_inner();
    let subtasks = query_as::<_, Subtask>(
        "SELECT id, task_id, title, completed, position, created_at
         FROM subtasks WHERE task_id = $1 ORDER BY position ASC, created_at ASC"
    )
    .bind(&task_id)
    .fetch_all(pool.get_ref())
    .await;

    match subtasks {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_subtask(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<CreateSubtaskRequest>,
) -> impl Responder {
    let task_id = path.into_inner();
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

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
        "INSERT INTO subtasks (id, task_id, title, completed, position, created_at) VALUES ($1, $2, $3, FALSE, $4, $5)"
    )
    .bind(&id)
    .bind(&task_id)
    .bind(&body.title)
    .bind(position)
    .bind(&now)
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
            };
            HttpResponse::Created().json(subtask)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_subtask(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<UpdateSubtaskRequest>,
) -> impl Responder {
    let subtask_id = path.into_inner();

    if let Some(ref title) = body.title {
        query("UPDATE subtasks SET title = $1 WHERE id = $2")
            .bind(title)
            .bind(&subtask_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }
    if let Some(completed) = body.completed {
        query("UPDATE subtasks SET completed = $1 WHERE id = $2")
            .bind(completed)
            .bind(&subtask_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }
    if let Some(position) = body.position {
        query("UPDATE subtasks SET position = $1 WHERE id = $2")
            .bind(position)
            .bind(&subtask_id)
            .execute(pool.get_ref())
            .await
            .ok();
    }

    let subtask = query_as::<_, Subtask>(
        "SELECT id, task_id, title, completed, position, created_at FROM subtasks WHERE id = $1"
    )
    .bind(&subtask_id)
    .fetch_one(pool.get_ref())
    .await;

    match subtask {
        Ok(s) => HttpResponse::Ok().json(s),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn delete_subtask(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> impl Responder {
    let subtask_id = path.into_inner();
    query("DELETE FROM subtasks WHERE id = $1")
        .bind(&subtask_id)
        .execute(pool.get_ref())
        .await
        .ok();
    HttpResponse::Ok().json(serde_json::json!({"deleted": true}))
}
