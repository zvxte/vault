use crate::error::Error;
use axum::http::HeaderMap;

pub fn get_headers_value(headers: &HeaderMap, key: &str) -> Result<String, Error> {
    match headers.get(key) {
        Some(value) => match value.to_str() {
            Ok(value) => Ok(value.into()),
            Err(_) => Err(Error::HeadersError),
        },
        None => Err(Error::HeadersError),
    }
}
