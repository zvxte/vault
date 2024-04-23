use axum::{
    extract::{Request, State},
    http::HeaderValue,
    middleware::Next,
    response::Response,
};

use crate::{database::Db, routers::AppState, model::MessageResponse};

pub async fn validate_session<'a>(
    State(state): State<AppState<'a>>,
    mut request: Request,
    next: Next,
) -> Response {
    let session_id = match request.headers().get("session_id") {
        Some(session_id) => session_id.to_str(),
        None => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };
    let user_id = match session_id {
        Ok(session_id) => {
            let user_id = state.database.check_session(
                session_id.to_string()
            ).await;
            match user_id {
                Ok(user_id) => user_id,
                Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
            }
        },
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };
    let user_id: HeaderValue = match user_id.to_string().as_str().parse() {
        Ok(header_value) => header_value,
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };
    request.headers_mut().insert("user_id", user_id);
    next.run(request).await
}
