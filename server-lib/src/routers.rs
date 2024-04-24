use std::env;
use axum::{
    routing::{post, get},
    Router,
};
use crypto::Argon2Hasher;
use crate::routes::{users, passwords};
use crate::database::PostgreDb;
use crate::middleware;

#[derive(Clone)]
pub struct UsersState<'a> {
    pub hasher: Argon2Hasher<'a>,
    pub database: PostgreDb,
}

#[derive(Clone)]
pub struct PasswordsState {
    pub database: PostgreDb,
}

pub async fn users_router() -> Router {
    let app_state = UsersState {
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
    let app_state = PasswordsState {
        database: PostgreDb::build(
            env::var("DATABASE_URL").expect("DATABASE_URL not set")
        ).await.expect("Invalid database configuration"),
    };
    Router::new()
        .route("/", post(passwords::post_passwords))
        .route("/:id", get(passwords::get_passwords_id))
        .route_layer(axum::middleware::from_fn_with_state(app_state.clone(), middleware::validate_session))
        .with_state(app_state)
}
