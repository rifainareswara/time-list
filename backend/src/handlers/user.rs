use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use crate::{
    db::DbPool,
    models::user::{ResetPasswordRequest, UpdateRoleRequest, User},
};
use actix_web::{web, HttpResponse, Responder};
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

    let result = sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
        .bind(password_hash)
        .bind(user_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
