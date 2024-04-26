use std::str::FromStr;
use axum::{
    extract::{State, Json, Path, rejection::{JsonRejection, PathRejection}},
    response::Response,
    http::HeaderMap,
};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use crate::routers::AppState;
use crate::model::{MessageResponse, DataResponse};
use crate::database::{Db, DbPassword};
use crate::utils;

#[derive(Deserialize, Debug)]
pub struct PasswordIn {
    pub domain_name: String,
    pub username: String,
    pub password: Vec<u8>,
    pub nonce: [u8; 12],
}

#[derive(Serialize)]
pub struct PasswordOut {
    password_id: String,
    domain_name: String,
    username: String,
    password: Vec<u8>,
    nonce: [u8; 12],
}

impl PasswordOut {
    pub fn from_dbpassword(dbpassword: DbPassword) -> Self {
        Self {
            password_id: dbpassword.password_id.to_string(),
            domain_name: dbpassword.domain_name,
            username: dbpassword.username,
            password: dbpassword.password,
            nonce: dbpassword.nonce,
        }
    }
}

pub async fn post_passwords(
    headers: HeaderMap,
    State(state): State<AppState<'_>>,
    password: Result<Json<PasswordIn>, JsonRejection>,
) -> Response {

    let password = match password {
        Ok(password) => password.0,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };

    let user_id = match utils::get_headers_value(&headers, "user_id") {
        Ok(user_id) => match Uuid::from_str(&user_id) {
            Ok(user_id) => user_id,
            Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
        },
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };

    match state.database.create_password(
        &user_id, &password.domain_name, &password.username, &password.password, &password.nonce
    ).await {
        Ok(dbpassword) => DataResponse::created(
            PasswordOut::from_dbpassword(dbpassword),
        ),
        Err(_) => MessageResponse::bad_request("Failed to add a new password".to_string()),
    }
}

pub async fn get_passwords_id(
    headers: HeaderMap,
    State(state): State<AppState<'_>>,
    password_id: Result<Path<Uuid>, PathRejection>,
) -> Response {
    let password_id = match password_id {
        Ok(password_id) => password_id.0,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };
    
    let user_id = match utils::get_headers_value(&headers, "user_id") {
        Ok(user_id) => match Uuid::from_str(&user_id) {
            Ok(user_id) => user_id,
            Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
        },
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };

    match state.database.get_password(&user_id, &password_id).await {
        Ok(dbpassword) => {
            if dbpassword.user_id == user_id {
                return DataResponse::ok(PasswordOut::from_dbpassword(dbpassword))
            } else { 
                return MessageResponse::unauthorized("Unauthorized access".to_string())
            }
        },
        Err(_) => return MessageResponse::bad_request("Failed to get a password".to_string()),
    }
}

pub async fn get_passwords(
    headers: HeaderMap,
    State(state): State<AppState<'_>>,
) -> Response {
    let user_id = match utils::get_headers_value(&headers, "user_id") {
        Ok(user_id) => match Uuid::from_str(&user_id) {
            Ok(user_id) => user_id,
            Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
        },
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };

    let dbpasswords = match state.database.get_passwords(&user_id).await {
        Ok(dbpasswords) => dbpasswords,
        Err(_) => return MessageResponse::bad_request("Failed to get passwords".to_string()),
    };

    DataResponse::ok(
        dbpasswords
            .into_iter()
            .map(|dbpassword| PasswordOut::from_dbpassword(dbpassword) )
            .collect::<Vec<PasswordOut>>()
    )
}

pub async fn delete_passwords_id(
    headers: HeaderMap,
    State(state): State<AppState<'_>>,
    password_id: Result<Path<Uuid>, PathRejection>,
) -> Response {
    let password_id = match password_id {
        Ok(password_id) => password_id,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };

    let user_id = match utils::get_headers_value(&headers, "user_id") {
        Ok(user_id) => match Uuid::from_str(&user_id) {
            Ok(user_id) => user_id,
            Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
        },
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };

    match state.database.delete_password(&user_id, &password_id).await {
        Ok(_) => MessageResponse::ok("Password deleted".to_string()),
        Err(_) => MessageResponse::bad_request("Failed to delete a password".to_string()),
    }
}
