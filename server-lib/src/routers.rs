use axum::{
    routing::post,
    Router,
};
use crate::routes::users;

pub fn users_router() -> Router {
    Router::new()
        .route("/register", post(users::post_users_register))
}
