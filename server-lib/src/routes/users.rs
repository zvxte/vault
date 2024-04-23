use axum::{
    extract::{rejection::JsonRejection, Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crypto::Hasher;
use crate::database::Db;
use crate::model::{MessageResponse, UserIn, UserOut};
use crate::routers::AppState;

pub async fn post_users_register<'a>(
    State(state): State<AppState<'a>>,
    user: Result<Json<UserIn>, JsonRejection>,
) -> Response {
    let user = match user {
        Ok(user) => user.0,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };

    let hasher = &state.hasher;
    let hashed_password = match hasher.hash_password(&user.password) {
        Ok(pwd) => pwd,
        Err(_) => return MessageResponse::bad_request("Failed to register a new account".to_string()),
    };

    let database = &state.database;
    match database.create_user(&user.name, &hashed_password).await {
        Ok(_) => MessageResponse::created("Account created".to_string()),
        Err(_) => MessageResponse::bad_request("Failed to register a new account".to_string()),
    }
}

pub async fn post_users_login<'a>(
    State(state): State<AppState<'a>>,
    user: Result<Json<UserIn>, JsonRejection>,
) -> Response {
    let user = match user {
        Ok(user) => user.0,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };

    let database = &state.database;
    let dbuser = match database.check_user(&user.name).await {
        Ok(dbuser) => dbuser,
        Err(_) => return MessageResponse::bad_request("Failed to login".to_string()),
    };

    let hasher = &state.hasher;
    if let Ok(result) = hasher.cmp_password(&user.password, &dbuser.password) {
        if result {
            let session_id = match database.create_session(&dbuser.user_id).await {
                Ok(session_id) => session_id,
                Err(_) => return MessageResponse::bad_request("Failed to login".to_string()),
            };

            return (
                StatusCode::OK,
                [
                    ("Content-Type", "application/json"),
                    ("session_id", &session_id),
                ],
                Json(UserOut::from_dbuser(dbuser)),
            ).into_response();
        }
    }

    MessageResponse::bad_request("Failed to login".to_string())
}
