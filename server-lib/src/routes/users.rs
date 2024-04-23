use axum::{
    extract::{
        rejection::JsonRejection,
        Json, State,
    },
    http::StatusCode,
    response::{Response, IntoResponse},
};
use crypto::Hasher;
use crate::routes::model::{DataResponse, MessageResponse, UserIn, UserOut};
use crate::database::Db;
use crate::routers::AppState;

pub async fn post_users_register<'a>(
    State(state): State<AppState<'a>>,
    user: Result<Json<UserIn>, JsonRejection>,
) -> Response {
    let user = match user {
        Ok(user) => {
            user.0
        },
        Err(err) => {
            return MessageResponse::new(
                StatusCode::BAD_REQUEST,
                err.to_string(),
            ).into_response()
        }
    };
    let hasher = &state.hasher;
    
    let hashed_password = match hasher.hash_password(&user.password) {
        Ok(pwd) => pwd,
        Err(_) => {
            return MessageResponse::new(
                StatusCode::BAD_REQUEST,
                "Failed to register a new account".to_string(),
            ).into_response()
        }
    };
    
    let database = &state.database;
    match database.create_user(&user.name, &hashed_password).await {
        Ok(_) => {
            MessageResponse::new(
                StatusCode::CREATED,
                "Account created".to_string(),
            ).into_response()
        },
        Err(_) => {
            MessageResponse::new(
                StatusCode::BAD_REQUEST,
                "Failed to register a new account".to_string(),
            ).into_response()
        }
    }
}

pub async fn post_users_login<'a>(
    State(state): State<AppState<'a>>,
    user: Result<Json<UserIn>, JsonRejection>,
) -> Response {
    let user = match user {
        Ok(user) => {
            user.0
        },
        Err(err) => {
            return MessageResponse::new(
                StatusCode::BAD_REQUEST,
                err.to_string(),
            ).into_response()
        }
    };

    let database = &state.database;
    let dbuser = match database.check_user(&user.name).await {
        Ok(dbuser) => dbuser,
        Err(_) => return {
            MessageResponse::new(
                StatusCode::BAD_REQUEST,
                "Failed to login".to_string(),
            ).into_response()
        }
    };

    let hasher = &state.hasher;
    if let Ok(result) = hasher.cmp_password(&user.password,&dbuser.password) {
        if result {
            return DataResponse::new(
                StatusCode::OK,
                UserOut::from_dbuser(dbuser),
            ).into_response()
        }
    }

    MessageResponse::new(
        StatusCode::BAD_REQUEST,
        "Failed to login".to_string(),
    ).into_response()
}
