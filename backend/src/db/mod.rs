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
    // ─── Auth Tables (Must be first for Foreign Keys) ───

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'user',
            created_at TEXT NOT NULL,
            force_change_password BOOLEAN NOT NULL DEFAULT FALSE
        )",
    )
    .execute(pool)
    .await
    .unwrap();
    
    // Create Default Admin if not exists (handled by register usually, but good for migrations if needed)

    // ─── Core Tables ───

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color TEXT NOT NULL DEFAULT '#3b82f6',
            description TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            user_id TEXT REFERENCES users(id) ON DELETE CASCADE
        )",
    )
    .execute(pool)
    .await
    .unwrap();

    // Default project (system-wide or per-user? -> We will likely duplicate or handle in code)
    // For now, allow default project creation without user_id if we want, OR make it nullable. 
    // BUT goal is Isolation. So we probably shouldn't have a global default project anymore.
    // Keeping this for legacy but making user_id optional here would be weird. 
    // Let's assume we handle default project in code per user.

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
            updated_at TEXT NOT NULL,
            project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
            user_id TEXT REFERENCES users(id) ON DELETE CASCADE
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
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
            user_id TEXT REFERENCES users(id) ON DELETE CASCADE
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
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
            user_id TEXT REFERENCES users(id) ON DELETE CASCADE
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
            FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
            user_id TEXT REFERENCES users(id) ON DELETE CASCADE
        )",
    )
    .execute(pool)
    .await
    .unwrap();

    // ─── Column Migrations (safe to re-run) ───

    // Backfill user_id? No, difficult without knowing who owns what. 
    // We will assume wipe or manual update.
    // Adding columns if they don't exist (Nullable first so it doesn't fail on existing rows)
    
    // Projects
    sqlx::query("ALTER TABLE projects ADD COLUMN IF NOT EXISTS user_id TEXT REFERENCES users(id) ON DELETE CASCADE").execute(pool).await.expect("Failed to add user_id to projects");
    
    // Tasks
    sqlx::query("ALTER TABLE tasks ADD COLUMN IF NOT EXISTS priority TEXT NOT NULL DEFAULT 'normal'").execute(pool).await.expect("Failed to add priority to tasks");
    sqlx::query("ALTER TABLE tasks ADD COLUMN IF NOT EXISTS project_id TEXT REFERENCES projects(id)").execute(pool).await.expect("Failed to add project_id to tasks");
    sqlx::query("ALTER TABLE tasks ADD COLUMN IF NOT EXISTS user_id TEXT REFERENCES users(id) ON DELETE CASCADE").execute(pool).await.expect("Failed to add user_id to tasks");

    // Time Entries
    sqlx::query("ALTER TABLE time_entries ADD COLUMN IF NOT EXISTS user_id TEXT REFERENCES users(id) ON DELETE CASCADE").execute(pool).await.expect("Failed to add user_id to time_entries");
    
    // Active Timers
    sqlx::query("ALTER TABLE active_timers ADD COLUMN IF NOT EXISTS user_id TEXT REFERENCES users(id) ON DELETE CASCADE").execute(pool).await.expect("Failed to add user_id to active_timers");

    // Subtasks
    sqlx::query("ALTER TABLE subtasks ADD COLUMN IF NOT EXISTS user_id TEXT REFERENCES users(id) ON DELETE CASCADE").execute(pool).await.expect("Failed to add user_id to subtasks");

    // Users
    sqlx::query("ALTER TABLE users ADD COLUMN IF NOT EXISTS force_change_password BOOLEAN NOT NULL DEFAULT FALSE").execute(pool).await.expect("Failed to add force_change_password to users");
    sqlx::query("ALTER TABLE users ADD COLUMN IF NOT EXISTS full_name TEXT NOT NULL DEFAULT ''").execute(pool).await.expect("Failed to add full_name to users");
}
