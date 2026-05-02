use std::{path::PathBuf, sync::Mutex};

use hget_core::http_request::HttpRequest;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct LoadFileResponse {
    http_request: HttpRequest,
    raw_http: String,
}

#[tauri::command]
pub async fn load_file(state: State<'_, Mutex<AppState>>, path: PathBuf) -> Result<LoadFileResponse, String> {
    let state = state.lock().unwrap();

    let path = path.strip_prefix("/").unwrap_or(&path);

    let (http_request, raw_file) = state.repository.get_http_file(path)?;

    let response = LoadFileResponse {
        http_request: http_request,
        raw_http: raw_file,
    };

    return Ok(response);
}