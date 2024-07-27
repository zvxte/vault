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

impl From<DbUser> for UserOut {
    fn from(dbuser: DbUser) -> Self {
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
    let user_id = utils::create_uuid_v4();
    let salt = utils::create_salt();
    let timestamp = utils::get_current_timestamp();

    match state
        .database
        .create_user(
            &user_id,
            &user.username,
            &hashed_password,
            &salt,
            timestamp,
            timestamp,
        )
        .await
    {
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

    let dbuser = match state.database.get_user(&user.username).await {
        Ok(dbuser) => dbuser,
        Err(_) => return MessageResponse::bad_request("Failed to login".to_string()),
    };

    if let Ok(result) = state.hasher.cmp_data(&user.password, &dbuser.password) {
        if result {
            let session_id = utils::create_session_id();
            let hashed_session_id = crypto::hash_with_sha3(&session_id);
            if let Err(_) = state
                .database
                .create_session(&hashed_session_id, &dbuser.user_id)
                .await
            {
                return MessageResponse::bad_request("Failed to create session".to_string());
            };

            let connected_at = utils::get_current_timestamp();
            state
                .database
                .update_user_timestamp(&dbuser.user_id, connected_at)
                .await
                .unwrap_or(());

            return (
                StatusCode::OK,
                [
                    ("Content-Type", "application/json"),
                    ("session_id", &session_id),
                ],
                Json(UserOut::from(dbuser)),
            )
                .into_response();
        }
    }
    MessageResponse::bad_request("Failed to login".to_string())
}

pub async fn post_users_logout(headers: HeaderMap, State(state): State<AppState<'_>>) -> Response {
    let session_id = match utils::get_headers_value(&headers, "session_id") {
        Ok(user_id) => user_id,
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };
    let hashed_session_id = crypto::hash_with_sha3(&session_id);
    match state.database.delete_session(&hashed_session_id).await {
        Ok(_) => return MessageResponse::ok("Session deleted".to_string()),
        Err(_) => return MessageResponse::bad_request("Failed to delete session".to_string()),
    }
}
