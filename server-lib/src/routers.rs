use axum::{
    routing::post,
    Router,
};
use crate::routes::users;
use crypto::Argon2Hasher;

pub fn users_router() -> Router {
    let argon2_state = Argon2Hasher::new();
    Router::new()
        .route("/register", post(users::post_users_register))
        .with_state(argon2_state)
}
