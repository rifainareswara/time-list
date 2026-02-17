use crate::{
    db::DbPool,
    models::user::{AuthClaims, AuthResponse, LoginRequest, RegisterRequest, User},
};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::{query, query_as};
use uuid::Uuid;

pub async fn register(
    pool: web::Data<DbPool>,
    body: web::Json<RegisterRequest>,
) -> impl Responder {
    // 1. Check if user exists
    let user_exists = query("SELECT id FROM users WHERE username = $1")
        .bind(&body.username)
        .fetch_optional(pool.get_ref())
        .await;

    if let Ok(Some(_)) = user_exists {
        return HttpResponse::Conflict().body("Username already exists");
    }

    // 2. Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    // 3. Determine role (first user = admin)
    let user_count: i64 = query_as::<_, (i64,)>("SELECT COUNT(*) FROM users")
        .fetch_one(pool.get_ref())
        .await
        .map(|r| r.0)
        .unwrap_or(0);

    let role = if user_count == 0 { "admin" } else { "user" };
    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();

    // 4. Insert user
    let insert_result = query(
        "INSERT INTO users (id, username, password_hash, role, created_at) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(&id)
    .bind(&body.username)
    .bind(&password_hash)
    .bind(role)
    .bind(&created_at)
    .execute(pool.get_ref())
    .await;

    match insert_result {
        Ok(_) => {
            // 5. Generate Token
            let token = generate_token(&id, role);
            HttpResponse::Ok().json(AuthResponse {
                token,
                user: User {
                    id,
                    username: body.username.clone(),
                    role: role.to_string(),
                    password_hash: "".to_string(),
                    created_at,
                },
            })
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn login(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
) -> impl Responder {
    // 1. Find user
    let user = query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&body.username)
        .fetch_optional(pool.get_ref())
        .await
        .unwrap_or(None);

    if let Some(user) = user {
        // 2. Verify password
        let parsed_hash = PasswordHash::new(&user.password_hash).expect("Invalid hash in DB");
        if Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            // 3. Generate token
            let token = generate_token(&user.id, &user.role);
            return HttpResponse::Ok().json(AuthResponse {
                token,
                user,
            });
        }
    }

    HttpResponse::Unauthorized().body("Invalid username or password")
}

pub async fn me(req: HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions().get::<AuthClaims>() {
        HttpResponse::Ok().json(claims)
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

fn generate_token(id: &str, role: &str) -> String {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = AuthClaims {
        sub: id.to_string(),
        role: role.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Token generation failed")
}
