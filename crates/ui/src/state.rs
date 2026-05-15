pub struct State {
    pub repository: hget_core::repository::Repository
}

impl gpui::Global for State {}

impl State {
    pub fn new() -> Self {
        Self {
            repository: hget_core::repository::Repository::new("./sample".into())
        }
    }
}