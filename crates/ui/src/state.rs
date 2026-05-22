use std::path::PathBuf;

use gpui::SharedString;
use hget_core::repository::Repository;

pub struct State {
    pub repository: Option<Repository>,
    pub active_path: Option<SharedString>
}

impl gpui::Global for State {}

impl State {
    pub fn new() -> Self {
        Self {
            repository: None,
            active_path: Some("/home/romera/projects/hget/sample/single.http".into())
        }
    }

    pub fn load_repository_at(&mut self, path: PathBuf) {
        let repository = Repository::new(path);

        self.repository = Some(repository);
    }
}

pub fn init(cx: &mut gpui::App) {
    let global_state = State::new();
    cx.set_global(global_state);
}
