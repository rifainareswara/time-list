use actix_web::{web, HttpRequest, HttpMessage, HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;
use sqlx::{query, query_as};

use crate::db::DbPool;
use crate::models::project::*;
use crate::models::user::AuthClaims;

// ─── GET /api/projects ───

pub async fn get_projects(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };

    let result = query_as::<_, ProjectWithStats>(
        "SELECT p.id, p.name, p.color, p.description, p.created_at,
                COALESCE(COUNT(t.id), 0)::BIGINT AS task_count,
                COALESCE(SUM(CASE WHEN t.status = 'pending' THEN 1 ELSE 0 END), 0)::BIGINT AS pending_count,
                COALESCE(SUM(CASE WHEN t.status = 'in_progress' THEN 1 ELSE 0 END), 0)::BIGINT AS in_progress_count,
                COALESCE(SUM(CASE WHEN t.status = 'completed' THEN 1 ELSE 0 END), 0)::BIGINT AS completed_count,
                COALESCE((SELECT SUM(e.duration_minutes) FROM time_entries e
                          INNER JOIN tasks t2 ON t2.id = e.task_id
                          WHERE t2.project_id = p.id), 0)::BIGINT AS total_minutes
         FROM projects p
         LEFT JOIN tasks t ON t.project_id = p.id
         WHERE p.user_id = $1
         GROUP BY p.id, p.name, p.color, p.description, p.created_at
         ORDER BY p.created_at ASC"
    )
    .bind(&user_id)
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// ─── POST /api/projects ───

pub async fn create_project(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<CreateProjectRequest>,
) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let color = body.color.clone().unwrap_or_else(|| "#3b82f6".into());
    let desc = body.description.clone().unwrap_or_default();

    let result = query(
        "INSERT INTO projects (id, name, color, description, created_at, user_id) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(&id)
    .bind(&body.name)
    .bind(&color)
    .bind(&desc)
    .bind(&now)
    .bind(&user_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let project = Project {
                id,
                name: body.name.clone(),
                color,
                description: desc,
                created_at: now,
                user_id,
            };
            HttpResponse::Created().json(project)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// ─── PUT /api/projects/{id} ───

pub async fn update_project(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateProjectRequest>,
) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let project_id = path.into_inner();

    if let Some(ref name) = body.name {
        query("UPDATE projects SET name = $1 WHERE id = $2 AND user_id = $3")
            .bind(name).bind(&project_id).bind(&user_id)
            .execute(pool.get_ref()).await.ok();
    }
    if let Some(ref color) = body.color {
        query("UPDATE projects SET color = $1 WHERE id = $2 AND user_id = $3")
            .bind(color).bind(&project_id).bind(&user_id)
            .execute(pool.get_ref()).await.ok();
    }
    if let Some(ref desc) = body.description {
        query("UPDATE projects SET description = $1 WHERE id = $2 AND user_id = $3")
            .bind(desc).bind(&project_id).bind(&user_id)
            .execute(pool.get_ref()).await.ok();
    }

    let project = query_as::<_, Project>(
        "SELECT id, name, color, description, created_at, user_id FROM projects WHERE id = $1 AND user_id = $2"
    )
    .bind(&project_id)
    .bind(&user_id)
    .fetch_one(pool.get_ref())
    .await;

    match project {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

// ─── DELETE /api/projects/{id} ───

pub async fn delete_project(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = if let Some(claims) = req.extensions().get::<AuthClaims>() {
        claims.sub.clone()
    } else {
        return HttpResponse::Unauthorized().finish();
    };
    let project_id = path.into_inner();

    // Move tasks from this project to default
    // TODO: Need user's default project or set to NULL?
    // Setting to NULL for now as "default" ID might not belong to this user if we are strict.
    // If we want a "Default" project for every user, we need to create it.
    // Let's just set to NULL for now or delete them? No, keep tasks.
    query("UPDATE tasks SET project_id = NULL WHERE project_id = $1 AND user_id = $2")
        .bind(&project_id)
        .bind(&user_id)
        .execute(pool.get_ref())
        .await
        .ok();

    let result = query("DELETE FROM projects WHERE id = $1 AND user_id = $2")
        .bind(&project_id)
        .bind(&user_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                HttpResponse::Ok().json(serde_json::json!({"deleted": true}))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
