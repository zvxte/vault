// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crypto::{AesGcmEncrypter, EncryptedData, Encrypter};
use std::sync::Mutex;
use tauri::State;

#[derive(Default)]
struct AppState {
    pub encrypter: Mutex<Option<crypto::AesGcmEncrypter>>,
}

#[tauri::command]
fn create_encrypter(
    state: State<AppState>,
    plain_master_password: String,
    salt: [u8; 32],
) -> Result<(), String> {
    let encrypter = match AesGcmEncrypter::build(plain_master_password, &salt) {
        Ok(encrypter) => encrypter,
        Err(_) => return Err("Failed to create encrypter".to_string()),
    };
    *state.encrypter.lock().unwrap() = Some(encrypter);
    Ok(())
}

#[tauri::command]
fn encrypt(state: State<AppState>, data: String) -> Result<(Vec<u8>, [u8; 12]), String> {
    match state
        .encrypter
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .encrypt(data)
    {
        Ok(encrypted_data) => return Ok((encrypted_data.content, encrypted_data.nonce)),
        Err(_) => Err("Failed to encrypt data".to_string()),
    }
}

#[tauri::command]
fn decrypt(state: State<AppState>, data: Vec<u8>, nonce: [u8; 12]) -> Result<String, String> {
    match state
        .encrypter
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .decrypt(EncryptedData {
            content: data,
            nonce,
        }) {
        Ok(result) => Ok(result),
        Err(_) => Err("Failed to decrypt data".to_string()),
    }
}

fn main() {
    let app_state = AppState::default();
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![create_encrypter, encrypt, decrypt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
