use std::path::PathBuf;

use hget_core::repository::Repository;

pub struct State {
    pub repository: Option<Repository>,
}

impl gpui::Global for State {}

impl State {
    pub fn new() -> Self {
        Self {
            repository: None,
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
