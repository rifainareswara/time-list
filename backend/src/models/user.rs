use serde::{Deserialize, Serialize};

use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub role: String,
    #[serde(skip_serializing)] // Never send password hash to client
    pub password_hash: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: String, // User ID
    pub role: String,
    pub exp: usize,
}
#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub new_password: String,
}
