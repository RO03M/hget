use std::{path::PathBuf, sync::Mutex};

use hget_core::{helpers::slugify, http_request::HttpRequest};
use tauri::State;

use crate::AppState;

#[tauri::command]
pub fn create_empty_file(state: State<'_, Mutex<AppState>>, parent_path: PathBuf, request_name: String) -> Result<(), String> {
    let state = state.lock().unwrap();
    let slugfied = format_args!("{}.http", slugify(&request_name)).to_string();
    let path = parent_path.join(slugfied);
    
    let _ = state.repository.save_http_file(
        &HttpRequest{
            body: None,
            headers: vec![],
            method: "GET".to_string(),
            name: request_name.to_string(),
            url: "https://fake.com".to_string()
        },
        &path
    );

    Ok(())
}