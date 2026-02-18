use actix_web::{web, HttpRequest, HttpMessage, HttpResponse, Responder};
use sqlx::{query_as, FromRow};
use serde::{Deserialize, Serialize};

use crate::db::DbPool;
use crate::models::user::AuthClaims;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TimeEntry {
    pub id: String,
    pub task_id: String,
    #[sqlx(default)]
    pub task_title: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_minutes: i64,
    pub notes: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateEntryRequest {
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_minutes: i64,
    pub notes: String,
}

pub async fn get_entries(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let task_id = path.into_inner();
    let entries: Vec<TimeEntry> = query_as(
        "SELECT e.*, t.title as task_title 
         FROM time_entries e 
         JOIN tasks t ON e.task_id = t.id 
         WHERE e.task_id = $1 AND e.user_id = $2
         ORDER BY e.created_at DESC"
    )
    .bind(task_id)
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();
    
    HttpResponse::Ok().json(entries)
}

pub async fn create_entry(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<String>, body: web::Json<CreateEntryRequest>) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let task_id = path.into_inner();
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query("INSERT INTO time_entries (id, task_id, start_time, end_time, duration_minutes, notes, created_at, user_id) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)")
        .bind(&id).bind(&task_id).bind(&body.start_time).bind(&body.end_time).bind(body.duration_minutes).bind(&body.notes).bind(&now).bind(&user_id)
        .execute(pool.get_ref()).await.ok();
        
    HttpResponse::Created().json(serde_json::json!({"id": id}))
}

pub async fn delete_entry(pool: web::Data<DbPool>, req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let id = path.into_inner();
    sqlx::query("DELETE FROM time_entries WHERE id = $1 AND user_id = $2").bind(id).bind(user_id).execute(pool.get_ref()).await.ok();
    HttpResponse::Ok().json(serde_json::json!({"deleted": true}))
}

pub async fn get_all_entries(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let entries: Vec<TimeEntry> = query_as(
        "SELECT e.*, t.title as task_title 
         FROM time_entries e 
         JOIN tasks t ON e.task_id = t.id 
         WHERE e.user_id = $1
         ORDER BY e.created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();
    HttpResponse::Ok().json(entries)
}
