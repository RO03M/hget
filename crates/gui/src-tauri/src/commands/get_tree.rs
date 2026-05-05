use std::sync::Mutex;

use hget_core::helpers::{FSNode, list_http_tree};
use tauri::State;

use crate::AppState;

#[tauri::command]
pub async fn get_tree(state: State<'_, Mutex<AppState>>) -> Result<FSNode, String> {
    let state = state.lock().map_err(|e| e.to_string())?;

    let nodes = list_http_tree(&state.repository.root);

    let root = FSNode {  
        name: state.repository.get_name(),
        is_dir: true,
        children: nodes,
    };

    return Ok(root);
}