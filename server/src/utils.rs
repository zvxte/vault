use crate::error::Error;
use axum::http::HeaderMap;
use rand::Rng;
use uuid::Uuid;

pub fn get_headers_value(headers: &HeaderMap, key: &str) -> Result<String, Error> {
    match headers.get(key) {
        Some(value) => match value.to_str() {
            Ok(value) => Ok(value.into()),
            Err(_) => Err(Error::HeadersError),
        },
        None => Err(Error::HeadersError),
    }
}

pub fn get_current_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}

pub fn create_uuid_v4() -> Uuid {
    Uuid::new_v4()
}

pub fn create_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    rand::thread_rng().fill(&mut salt);
    salt
}

pub fn create_session_id() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
