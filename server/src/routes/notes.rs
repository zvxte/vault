use crate::database::{Db, DbNote};
use crate::model::{DataResponse, MessageResponse};
use crate::routers::AppState;
use crate::utils;
use axum::{
    extract::{
        rejection::{JsonRejection, PathRejection},
        Json, Path, State,
    },
    http::HeaderMap,
    response::Response,
};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct NoteIn {
    pub title: Vec<u8>,
    pub title_nonce: [u8; 12],
    pub content: Vec<u8>,
    pub content_nonce: [u8; 12],
}

#[derive(Serialize)]
pub struct NoteOut {
    note_id: String,
    title: Vec<u8>,
    title_nonce: [u8; 12],
    content: Vec<u8>,
    content_nonce: [u8; 12],
}

impl From<DbNote> for NoteOut {
    fn from(dbnote: DbNote) -> Self {
        Self {
            note_id: dbnote.note_id.to_string(),
            title: dbnote.title,
            title_nonce: dbnote.title_nonce,
            content: dbnote.content,
            content_nonce: dbnote.content_nonce,
        }
    }
}

pub async fn post_notes(
    headers: HeaderMap,
    State(state): State<AppState<'_>>,
    notein: Result<Json<NoteIn>, JsonRejection>,
) -> Response {
    let notein = match notein {
        Ok(notein) => notein.0,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };

    let user_id = match utils::get_headers_value(&headers, "user_id") {
        Ok(user_id) => match Uuid::from_str(&user_id) {
            Ok(user_id) => user_id,
            Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
        },
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };
    let note_id = utils::create_uuid_v4();
    match state
        .database
        .create_note(
            &note_id,
            &user_id,
            &notein.title,
            &notein.title_nonce,
            &notein.content,
            &notein.content_nonce,
        )
        .await
    {
        Ok(dbnote) => DataResponse::created(NoteOut::from(dbnote)),
        Err(_) => MessageResponse::bad_request("Failed to add a new note".to_string()),
    }
}

pub async fn get_notes(headers: HeaderMap, State(state): State<AppState<'_>>) -> Response {
    let user_id = match utils::get_headers_value(&headers, "user_id") {
        Ok(user_id) => match Uuid::from_str(&user_id) {
            Ok(user_id) => user_id,
            Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
        },
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };

    let dbnotes = match state.database.get_notes(&user_id).await {
        Ok(dbnotes) => dbnotes,
        Err(_) => return MessageResponse::bad_request("Failed to get notes".to_string()),
    };

    DataResponse::ok(
        dbnotes
            .into_iter()
            .map(|dbnote| NoteOut::from(dbnote))
            .collect::<Vec<NoteOut>>(),
    )
}

pub async fn get_notes_id(
    headers: HeaderMap,
    State(state): State<AppState<'_>>,
    note_id: Result<Path<Uuid>, PathRejection>,
) -> Response {
    let note_id = match note_id {
        Ok(note_id) => note_id.0,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };

    let user_id = match utils::get_headers_value(&headers, "user_id") {
        Ok(user_id) => match Uuid::from_str(&user_id) {
            Ok(user_id) => user_id,
            Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
        },
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };

    match state.database.get_note(&user_id, &note_id).await {
        Ok(dbnote) => {
            if dbnote.user_id == user_id {
                return DataResponse::ok(NoteOut::from(dbnote));
            } else {
                return MessageResponse::unauthorized("Unauthorized access".to_string());
            }
        }
        Err(_) => return MessageResponse::bad_request("Failed to get a note".to_string()),
    }
}

pub async fn delete_notes_id(
    headers: HeaderMap,
    State(state): State<AppState<'_>>,
    note_id: Result<Path<Uuid>, PathRejection>,
) -> Response {
    let note_id = match note_id {
        Ok(note_id) => note_id,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };

    let user_id = match utils::get_headers_value(&headers, "user_id") {
        Ok(user_id) => match Uuid::from_str(&user_id) {
            Ok(user_id) => user_id,
            Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
        },
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };

    match state.database.delete_note(&user_id, &note_id).await {
        Ok(_) => MessageResponse::ok("Note deleted".to_string()),
        Err(_) => MessageResponse::bad_request("Failed to delete a note".to_string()),
    }
}

pub async fn patch_notes_id(
    headers: HeaderMap,
    State(state): State<AppState<'_>>,
    note_id: Result<Path<Uuid>, PathRejection>,
    notein: Result<Json<NoteIn>, JsonRejection>,
) -> Response {
    let notein = match notein {
        Ok(notein) => notein.0,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };

    let note_id = match note_id {
        Ok(note_id) => note_id.0,
        Err(err) => return MessageResponse::bad_request(err.to_string()),
    };

    let user_id = match utils::get_headers_value(&headers, "user_id") {
        Ok(user_id) => match Uuid::from_str(&user_id) {
            Ok(user_id) => user_id,
            Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
        },
        Err(_) => return MessageResponse::unauthorized("Unauthorized access".to_string()),
    };

    match state
        .database
        .patch_note(
            &note_id,
            &user_id,
            &notein.title,
            &notein.title_nonce,
            &notein.content,
            &notein.content_nonce,
        )
        .await
    {
        Ok(dbnote) => DataResponse::created(NoteOut::from(dbnote)),
        Err(_) => MessageResponse::bad_request("Failed to edit a note".to_string()),
    }
}
