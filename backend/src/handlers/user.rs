use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use serde::Serialize;
use crate::{
    db::DbPool,
    models::user::{AuthClaims, ResetPasswordRequest, UpdateProfileRequest, UpdateRoleRequest, User},
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use sqlx::query_as;

// Admin only: Get all users
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let users = query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Admin only: Delete user
pub async fn delete_user(pool: web::Data<DbPool>, path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Admin only: Update user role
pub async fn update_role(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<UpdateRoleRequest>,
) -> impl Responder {
    let user_id = path.into_inner();
    
    // Validate role
    if body.role != "admin" && body.role != "user" {
        return HttpResponse::BadRequest().body("Invalid role");
    }

    let result = sqlx::query("UPDATE users SET role = $1 WHERE id = $2")
        .bind(&body.role)
        .bind(user_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Admin only: Reset password
pub async fn reset_password(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<ResetPasswordRequest>,
) -> impl Responder {
    let user_id = path.into_inner();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(body.new_password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    let result = sqlx::query("UPDATE users SET password_hash = $1, force_change_password = TRUE WHERE id = $2")
        .bind(password_hash)
        .bind(user_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Current user: Update profile (full_name)
pub async fn update_profile(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<UpdateProfileRequest>,
) -> impl Responder {
    let user_id = {
        let ext = req.extensions();
        match ext.get::<AuthClaims>() {
            Some(c) => c.sub.clone(),
            None => return HttpResponse::Unauthorized().finish(),
        }
    };

    let result = sqlx::query("UPDATE users SET full_name = $1 WHERE id = $2")
        .bind(&body.full_name)
        .bind(&user_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Admin only: Time report — total minutes per user per project
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TimeReportRow {
    pub user_id: String,
    pub username: String,
    pub full_name: String,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub project_color: Option<String>,
    pub total_minutes: i64,
}

pub async fn get_time_report_admin(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
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

    let sql = "SELECT u.id AS user_id, u.username, u.full_name,
                      p.id AS project_id, p.name AS project_name, p.color AS project_color,
                      COALESCE(SUM(e.duration_minutes), 0)::BIGINT AS total_minutes
               FROM users u
               LEFT JOIN time_entries e ON e.user_id = u.id
               LEFT JOIN tasks t ON t.id = e.task_id
               LEFT JOIN projects p ON p.id = t.project_id
               WHERE u.role != 'admin'
               GROUP BY u.id, u.username, u.full_name, p.id, p.name, p.color
               ORDER BY u.username, total_minutes DESC";

    let result = query_as::<_, TimeReportRow>(sql)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => {
            eprintln!("get_time_report_admin error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
