use axum::{
    extract::{rejection::JsonRejection, Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use crypto::Hasher;
use crate::model::MessageResponse;
use crate::database::{Db, DbUser};
use crate::routers::AppState;

#[derive(Deserialize)]
pub struct UserIn {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserOut {
    user_id: String,
    username: String,
    salt: [u8; 32],
}

impl UserOut {
    pub fn from_dbuser(dbuser: DbUser) -> Self {
        Self {
            user_id: dbuser.user_id.to_string(),
            username: dbuser.username,
            salt: dbuser.salt
        }
    }
}

pub async fn post_users_register(
    State(state): State<AppState<'_>>,
    user: Result<Json<UserIn>, JsonRejection>,
) -> Response {
    let user = match user {
        Ok(user) => user.0,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };

    let hasher = &state.hasher;
    let hashed_password = match hasher.hash_data(&user.password) {
        Ok(pwd) => pwd,
        Err(_) => return MessageResponse::bad_request("Failed to register a new account".to_string()),
    };

    let database = &state.database;
    match database.create_user(&user.username, &hashed_password).await {
        Ok(_) => MessageResponse::created("Account created".to_string()),
        Err(_) => MessageResponse::bad_request("Failed to register a new account".to_string()),
    }
}

pub async fn post_users_login(
    State(state): State<AppState<'_>>,
    user: Result<Json<UserIn>, JsonRejection>,
) -> Response {
    let user = match user {
        Ok(user) => user.0,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };

    let database = &state.database;
    let dbuser = match database.get_user(&user.username).await {
        Ok(dbuser) => dbuser,
        Err(_) => return MessageResponse::bad_request("Failed to login".to_string()),
    };

    if let Ok(result) = state.hasher.cmp_data(&user.password, &dbuser.password) {
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
