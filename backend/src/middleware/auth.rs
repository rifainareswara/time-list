use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    HttpMessage, HttpResponse, error::ErrorUnauthorized,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::models::user::AuthClaims;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Init validation
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
                    
                    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
                    let validation = Validation::new(Algorithm::HS256);

                    match decode::<AuthClaims>(token, &decoding_key, &validation) {
                        Ok(token_data) => {
                            // Attach claims to request extensions
                            req.extensions_mut().insert(token_data.claims);
                            let fut = self.service.call(req);
                            return Box::pin(async move {
                                let res = fut.await?;
                                Ok(res)
                            });
                        }
                        Err(_) => {
                            return Box::pin(async { Err(ErrorUnauthorized("Invalid token")) });
                        }
                    }
                }
            }
        }

        Box::pin(async { Err(ErrorUnauthorized("Missing or invalid Authorization header")) })
    }
}
