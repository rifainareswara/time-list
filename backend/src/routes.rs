use actix_web::web;
use crate::{handlers, middleware};

pub fn configure(cfg: &mut web::ServiceConfig) {
    // Public Auth Routes
    cfg.service(
        web::scope("/api/auth")
            .route("/login", web::post().to(handlers::auth::login))
            .route("/register", web::post().to(handlers::auth::register))
    );

    // Protected Routes
    cfg.service(
        web::scope("/api")
            .wrap(middleware::auth::Auth)
            .route("/auth/me", web::get().to(handlers::auth::me))
            // Admin Routes
            .route("/users", web::get().to(handlers::user::get_users))
            .route("/users/{id}", web::delete().to(handlers::user::delete_user))
            .route("/users/{id}/role", web::put().to(handlers::user::update_role))
            .route("/users/{id}/password", web::put().to(handlers::user::reset_password))
            // Task routes
            .route("/tasks/bulk-delete", web::post().to(handlers::task::delete_tasks_bulk))
            .route("/tasks", web::get().to(handlers::task::get_tasks))
            .route("/tasks", web::post().to(handlers::task::create_task))
            .route("/tasks/{id}", web::put().to(handlers::task::update_task))
            .route("/tasks/{id}", web::delete().to(handlers::task::delete_task))
            // Time entry routes
            .route("/tasks/{id}/entries", web::get().to(handlers::entry::get_entries))
            .route("/tasks/{id}/entries", web::post().to(handlers::entry::create_entry))
            .route("/entries/{id}", web::delete().to(handlers::entry::delete_entry))
            .route("/entries", web::get().to(handlers::entry::get_all_entries))
            // Subtask routes
            .route("/tasks/{id}/subtasks", web::get().to(handlers::subtask::get_subtasks))
            .route("/tasks/{id}/subtasks", web::post().to(handlers::subtask::create_subtask))
            .route("/subtasks/{id}", web::put().to(handlers::subtask::update_subtask))
            .route("/subtasks/{id}", web::delete().to(handlers::subtask::delete_subtask))
            // Project routes
            .route("/projects", web::get().to(handlers::project::get_projects))
            .route("/projects", web::post().to(handlers::project::create_project))
            .route("/projects/{id}", web::put().to(handlers::project::update_project))
            .route("/projects/{id}", web::delete().to(handlers::project::delete_project))
            // Dashboard
            .route("/dashboard/summary", web::get().to(handlers::dashboard::get_dashboard))
            // Timer
            .route("/timer/start/{task_id}", web::post().to(handlers::timer::start_timer))
            .route("/timer/stop", web::post().to(handlers::timer::stop_timer))
            .route("/timer/active", web::get().to(handlers::timer::get_active_timer))
    );
}
