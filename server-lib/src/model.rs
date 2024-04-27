use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

const CONTENT_TYPE_JSON: [(&str, &str); 1] = [("Content-Type", "application/json")];

#[derive(Serialize)]
pub struct DataResponse<T: Serialize> {
    #[serde(skip)]
    status_code: StatusCode,
    #[serde(flatten)]
    data: T,
}

impl<T: Serialize> DataResponse<T> {
    pub fn _new(status_code: StatusCode, data: T) -> Self {
        Self { status_code, data }
    }

    pub fn ok(data: T) -> Response {
        Self {
            status_code: StatusCode::OK,
            data,
        }
        .into_response()
    }

    pub fn created(data: T) -> Response {
        Self {
            status_code: StatusCode::CREATED,
            data,
        }
        .into_response()
    }
}

impl<T: Serialize> IntoResponse for DataResponse<T> {
    fn into_response(self) -> Response {
        (self.status_code, CONTENT_TYPE_JSON, Json(self.data)).into_response()
    }
}

#[derive(Serialize)]
pub struct MessageResponse {
    #[serde(skip)]
    status_code: StatusCode,
    message: String,
}

impl MessageResponse {
    pub fn _new(status_code: StatusCode, message: String) -> Response {
        Self {
            status_code,
            message,
        }
        .into_response()
    }

    pub fn ok(message: String) -> Response {
        Self {
            status_code: StatusCode::OK,
            message,
        }
        .into_response()
    }

    pub fn created(message: String) -> Response {
        Self {
            status_code: StatusCode::CREATED,
            message,
        }
        .into_response()
    }

    pub fn bad_request(message: String) -> Response {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            message,
        }
        .into_response()
    }

    pub fn unauthorized(message: String) -> Response {
        Self {
            status_code: StatusCode::UNAUTHORIZED,
            message,
        }
        .into_response()
    }
}

impl IntoResponse for MessageResponse {
    fn into_response(self) -> Response {
        (self.status_code, CONTENT_TYPE_JSON, Json(self)).into_response()
    }
}
