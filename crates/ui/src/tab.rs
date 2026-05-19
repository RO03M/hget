use std::path::PathBuf;

use gpui::SharedString;

#[derive(Debug, Clone)]
pub struct Tab {
    pub active: bool,
    pub path: PathBuf,
    pub label: SharedString,
}