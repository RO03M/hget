use std::path::PathBuf;

use hget_core::{executor::HttpResponse, helpers::{FSNode, list_http_tree}, http_request::HttpRequest, repository::Repository};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn send_request(request: HttpRequest) -> Result<HttpResponse, String> {
    let response = request.run().await.map_err(|e| e.to_string())?;

    return Ok(response);
}

#[tauri::command]
async fn save_request(request: HttpRequest) {
    let repository = Repository::new("./testemalandro".into());
    repository.create_collection("teste");
    repository.save_http_file(&request, &".".into());
}

#[tauri::command]
async fn load_file(path: PathBuf) -> Result<HttpRequest, String> {
    let repository = Repository::new("/home/romera/projects/hget/sample".into());

    let path = path.strip_prefix("/").unwrap_or(&path);

    let http_request = repository.get_http_file(path)?;

    return Ok(http_request);
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
        .invoke_handler(tauri::generate_handler![
            greet,
            send_request,
            save_request,
            get_tree,
            load_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
