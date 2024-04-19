use axum::{
    extract::{
        rejection::JsonRejection,
        Json,
    },
    http::StatusCode,
    response::Response,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

const CONTENT_TYPE_JSON: [(&str, &str); 1] = [("Content-Type", "application/json")];

#[derive(Serialize)]
pub struct DataResponse<T: Serialize> {
    #[serde(skip)]
    status_code: StatusCode,
    #[serde(flatten)]
    data: T,
}

impl<T: Serialize> DataResponse<T> {
    fn _new(status_code: StatusCode, data: T) -> Self {
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

#[derive(Deserialize, Serialize)]
pub struct User {
    name: String,
    password: String,
}

pub async fn post_users_register(user: Result<Json<User>, JsonRejection>) -> Response {
    let _user = match user {
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
    
    /*
    TODO!
    ...
    */

    MessageResponse::new(
        StatusCode::CREATED,
        "Account created".to_string(),
    ).into_response()
}
