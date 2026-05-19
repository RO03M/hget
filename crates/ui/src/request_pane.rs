use gpui::*;

use crate::url_input::UrlInput;

pub struct RequestPane {
    url_input: Entity<UrlInput>
}

impl RequestPane {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let url_input = cx.new(|cx| UrlInput::new(window, cx));

        Self {
            url_input
        }
    }
}

impl Render for RequestPane {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        return div()
            .flex_col()
            .flex_1()
            .bg(rgb(0x0000ff))
            .child(self.url_input.clone())
            .child("rest");
    }
}
