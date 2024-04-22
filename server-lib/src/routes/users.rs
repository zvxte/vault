use axum::{
    extract::{
        rejection::JsonRejection,
        Json, State,
    },
    http::StatusCode,
    response::Response,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use crypto::Hasher;
use crate::database::{Db, DbUser};
use crate::routers::AppState;

const CONTENT_TYPE_JSON: [(&str, &str); 1] = [("Content-Type", "application/json")];

#[derive(Serialize)]
pub struct DataResponse<T: Serialize> {
    #[serde(skip)]
    status_code: StatusCode,
    #[serde(flatten)]
    data: T,
}

impl<T: Serialize> DataResponse<T> {
    fn new(status_code: StatusCode, data: T) -> Self {
        Self { status_code, data }
    }
}

impl<T: Serialize> IntoResponse for DataResponse<T> {
    fn into_response(self) -> Response {
        (
            self.status_code,
            CONTENT_TYPE_JSON,
            Json(self.data),
        ).into_response()
    }
}

#[derive(Serialize)]
pub struct MessageResponse {
    #[serde(skip)]
    status_code: StatusCode,
    message: String,
}

impl MessageResponse {
    fn new(status_code: StatusCode, message: String) -> Self {
        Self { status_code, message }
    }
}

impl IntoResponse for MessageResponse {
    fn into_response(self) -> Response {
        (
            self.status_code,
            CONTENT_TYPE_JSON,
            Json(self),
        ).into_response()
    }
}

#[derive(Deserialize)]
pub struct UserIn {
    name: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserOut {
    user_id: String,
    username: String,
    salt: [u8; 32],
}

impl UserOut {
    fn from_dbuser(dbuser: DbUser) -> Self {
        Self {
            user_id: dbuser.user_id.to_string(),
            username: dbuser.username,
            salt: dbuser.salt
        }
    }
}

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
