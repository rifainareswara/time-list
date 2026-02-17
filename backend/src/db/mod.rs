use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn init_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    run_migrations(&pool).await;

    pool
}

async fn run_migrations(pool: &DbPool) {
    // ─── Core Tables ───

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color TEXT NOT NULL DEFAULT '#3b82f6',
            description TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await
    .unwrap();

    // Default project so existing tasks have a home
    sqlx::query(
        "INSERT INTO projects (id, name, color, description, created_at)
         VALUES ('default', 'General', '#3b82f6', 'Default project', '2024-01-01T00:00:00Z')
         ON CONFLICT (id) DO NOTHING"
    )
    .execute(pool)
    .await
    .ok();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            category TEXT NOT NULL DEFAULT 'General',
            status TEXT NOT NULL DEFAULT 'pending',
            priority TEXT NOT NULL DEFAULT 'normal',
            start_date TEXT,
            due_date TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS time_entries (
            id TEXT PRIMARY KEY,
            task_id TEXT NOT NULL,
            start_time TEXT NOT NULL,
            end_time TEXT,
            duration_minutes BIGINT NOT NULL DEFAULT 0,
            notes TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
        )",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS active_timers (
            id TEXT PRIMARY KEY,
            task_id TEXT NOT NULL UNIQUE,
            start_time TEXT NOT NULL,
            notes TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
        )",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS subtasks (
            id TEXT PRIMARY KEY,
            task_id TEXT NOT NULL,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE,
            position INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
        )",
    )
    .execute(pool)
    .await
    .unwrap();

    // ─── Column Migrations (safe to re-run) ───

    sqlx::query("ALTER TABLE tasks ADD COLUMN IF NOT EXISTS priority TEXT NOT NULL DEFAULT 'normal'")
        .execute(pool).await.ok();

    sqlx::query("ALTER TABLE tasks ADD COLUMN IF NOT EXISTS project_id TEXT REFERENCES projects(id)")
        .execute(pool).await.ok();

    // Assign existing tasks without project to default
    sqlx::query("UPDATE tasks SET project_id = 'default' WHERE project_id IS NULL")
        .execute(pool).await.ok();

    // ─── Auth Tables ───

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'user',
            created_at TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await
    .unwrap();
}
