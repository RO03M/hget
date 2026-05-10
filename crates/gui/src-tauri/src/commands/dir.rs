use std::{path::PathBuf, sync::Mutex};

use tauri::State;

use crate::AppState;

#[tauri::command]
pub fn create_dir_command(state: State<'_, Mutex<AppState>>, path: PathBuf, name: String) -> Result<(), String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    
    let path = path.join(name.strip_prefix("/").unwrap());

    state.repository.create_dir(path)?;

    Ok(())
}