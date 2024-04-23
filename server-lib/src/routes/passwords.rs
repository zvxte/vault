use axum::{
    extract::{State, Json, rejection::JsonRejection},
    response::Response,
    http::HeaderMap,
};

use crate::routers::AppState;
use crate::model::{MessageResponse, PasswordIn};

pub async fn post_passwords<'a>(
    _headers: HeaderMap,
    State(_state): State<AppState<'a>>,
    password: Result<Json<PasswordIn>, JsonRejection>,
) -> Response {
    let _password = match password {
        Ok(password) => password.0,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };

    /*
    TODO!
    ...
    */

    todo!()
}
