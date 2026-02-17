use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // Task routes
        .route("/api/tasks/bulk-delete", web::post().to(handlers::task::delete_tasks_bulk))
        .route("/api/tasks", web::get().to(handlers::task::get_tasks))
        .route("/api/tasks", web::post().to(handlers::task::create_task))
        .route("/api/tasks/{id}", web::put().to(handlers::task::update_task))
        .route("/api/tasks/{id}", web::delete().to(handlers::task::delete_task))
        // Time entry routes
        .route("/api/tasks/{id}/entries", web::get().to(handlers::entry::get_entries))
        .route("/api/tasks/{id}/entries", web::post().to(handlers::entry::create_entry))
        .route("/api/entries/{id}", web::delete().to(handlers::entry::delete_entry))
        .route("/api/entries", web::get().to(handlers::entry::get_all_entries))
        // Subtask routes
        .route("/api/tasks/{id}/subtasks", web::get().to(handlers::subtask::get_subtasks))
        .route("/api/tasks/{id}/subtasks", web::post().to(handlers::subtask::create_subtask))
        .route("/api/subtasks/{id}", web::put().to(handlers::subtask::update_subtask))
        .route("/api/subtasks/{id}", web::delete().to(handlers::subtask::delete_subtask))
        // Project routes
        .route("/api/projects", web::get().to(handlers::project::get_projects))
        .route("/api/projects", web::post().to(handlers::project::create_project))
        .route("/api/projects/{id}", web::put().to(handlers::project::update_project))
        .route("/api/projects/{id}", web::delete().to(handlers::project::delete_project))
        // Dashboard
        .route("/api/dashboard/summary", web::get().to(handlers::dashboard::get_dashboard))
        // Timer
        .route("/api/timer/start/{task_id}", web::post().to(handlers::timer::start_timer))
        .route("/api/timer/stop", web::post().to(handlers::timer::stop_timer))
        .route("/api/timer/active", web::get().to(handlers::timer::get_active_timer));
}
