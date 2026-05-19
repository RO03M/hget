use gpui::*;

pub struct ResponsePane;

impl ResponsePane {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for ResponsePane {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        return div().child("response pane");
    }
}
