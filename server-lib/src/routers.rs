use std::env;
use axum::{
    routing::post,
    Router,
};
use crate::routes::users;
use crate::database::PostgreDb;
use crypto::Argon2Hasher;

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
