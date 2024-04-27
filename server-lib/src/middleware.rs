use std::str::FromStr;

use crate::{database::Db, model::MessageResponse, routers::AppState, utils};
use axum::{
    extract::{Request, State},
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use sqlx::types::Uuid;

pub async fn validate_session(
    State(state): State<AppState<'_>>,
    mut request: Request,
    next: Next,
) -> Response {
    let session_id = match utils::get_headers_value(request.headers(), "session_id") {
        Ok(session_id) => match Uuid::from_str(&session_id) {
            Ok(session_id) => session_id,
            Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
        },
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };
    let user_id = match state.database.validate_session(&session_id).await {
        Ok(user_id) => user_id,
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };
    let user_id: HeaderValue = match user_id.to_string().parse() {
        Ok(header_value) => header_value,
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };
    request.headers_mut().insert("user_id", user_id);
    next.run(request).await
}
