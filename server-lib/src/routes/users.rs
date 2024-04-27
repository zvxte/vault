use crate::database::{Db, DbUser};
use crate::model::MessageResponse;
use crate::routers::AppState;
use crate::utils;
use axum::{
    extract::{rejection::JsonRejection, Json, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use crypto::Hasher;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::str::FromStr;

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
            salt: dbuser.salt,
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
        Err(_) => {
            return MessageResponse::bad_request("Failed to register a new account".to_string())
        }
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
            )
                .into_response();
        }
    }
    MessageResponse::bad_request("Failed to login".to_string())
}

pub async fn post_users_logout(headers: HeaderMap, State(state): State<AppState<'_>>) -> Response {
    let session_id = match utils::get_headers_value(&headers, "session_id") {
        Ok(user_id) => match Uuid::from_str(&user_id) {
            Ok(user_id) => user_id,
            Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
        },
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };
    match state.database.delete_session(&session_id).await {
        Ok(_) => return MessageResponse::ok("Session deleted".to_string()),
        Err(_) => return MessageResponse::bad_request("Failed to delete session".to_string()),
    }
}
