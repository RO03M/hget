mod commands;

use std::{path::PathBuf, sync::Mutex};

use hget_core::{executor::HttpResponse, helpers::{FSNode, list_http_tree}, http_request::HttpRequest, repository::Repository};
use tauri::{Manager, State};

use crate::commands::load_file;

struct AppState {
    repository: Repository
}

#[tauri::command]
async fn send_request(request: HttpRequest) -> Result<HttpResponse, String> {
    let response = request.run().await.map_err(|e| e.to_string())?;

    return Ok(response);
}

#[tauri::command]
async fn save_request(state: State<'_, Mutex<AppState>>, request: HttpRequest, path: PathBuf) -> Result<(), String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    let _ = state.repository.save_http_file(&request, &path);

    Ok(())
}

#[tauri::command]
async fn get_tree() -> Vec<FSNode> {
    let nodes = list_http_tree(std::path::Path::new("/home/romera/projects/hget/sample"));

    return nodes;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState {
                repository: Repository::new("/home/romera/projects/hget/sample".into())
            }));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            send_request,
            save_request,
            get_tree,
            load_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
