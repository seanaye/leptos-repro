use axum::extract::FromRef;
use leptos::config::LeptosOptions;
use sqlx::SqlitePool;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: SqlitePool,
}
