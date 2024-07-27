use crate::{database::Db, model::MessageResponse, routers::AppState, utils};
use axum::{
    extract::{Request, State},
    http::HeaderValue,
    middleware::Next,
    response::Response,
};

pub async fn validate_session(
    State(state): State<AppState<'_>>,
    mut request: Request,
    next: Next,
) -> Response {
    let session_id = match utils::get_headers_value(request.headers(), "session_id") {
        Ok(session_id) => session_id,
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };
    let hashed_session_id = crypto::hash_with_sha3(&session_id);
    let user_id = match state.database.validate_session(&hashed_session_id).await {
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
