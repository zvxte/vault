use serde::{Serialize, Deserialize};
use axum::{
    http::StatusCode,
    extract::Json,
    response::{Response, IntoResponse},
};
use crate::database::DbUser;

const CONTENT_TYPE_JSON: [(&str, &str); 1] = [("Content-Type", "application/json")];

#[derive(Serialize)]
pub struct DataResponse<T: Serialize> {
    #[serde(skip)]
    status_code: StatusCode,
    #[serde(flatten)]
    data: T,
}

impl<T: Serialize> DataResponse<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
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
    pub fn new(status_code: StatusCode, message: String) -> Self {
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
    pub name: String,
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
