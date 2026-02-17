mod db;
mod handlers;
mod models;
mod routes;
mod middleware;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Sic Mundus Backend starting on http://localhost:8006");
    dotenvy::dotenv().ok(); // Load .env if exists (dev)

    let pool = db::init_pool().await;
    let db_data = web::Data::new(pool);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(db_data.clone())
            .configure(routes::configure)
    })
    .bind("0.0.0.0:8006")?
    .run()
    .await
}
