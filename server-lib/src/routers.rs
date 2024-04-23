use std::env;
use axum::{
    routing::post,
    Router,
};
use crypto::Argon2Hasher;
use crate::routes::{users, passwords};
use crate::database::PostgreDb;
use crate::middleware;

#[derive(Clone)]
pub struct AppState<'a> {
    pub hasher: Argon2Hasher<'a>,
    pub database: PostgreDb,
}

pub async fn users_router() -> Router {
    let app_state = AppState {
        hasher: Argon2Hasher::new(),
        database: PostgreDb::build(
            env::var("DATABASE_URL").expect("DATABASE_URL not set")
        ).await.expect("Invalid database configuration"),
    };
    Router::new()
        .route("/register", post(users::post_users_register))
        .route("/login", post(users::post_users_login))
        .with_state(app_state)
}

pub async fn passwords_router() -> Router {
    let app_state = AppState {
        hasher: Argon2Hasher::new(),
        database: PostgreDb::build(
            env::var("DATABASE_URL").expect("DATABASE_URL not set")
        ).await.expect("Invalid database configuration"),
    };
    Router::new()
        .route("/", post(passwords::post_passwords))
        .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), middleware::validate_session))
        .with_state(app_state)
}
