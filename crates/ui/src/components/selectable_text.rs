use gpui::*;
use gpui_component::input::{Input, InputState};

#[derive(Clone)]
pub struct SelectableText {
    input_state: Entity<InputState>,
}

impl SelectableText {
    pub fn new(text: SharedString, window: &mut Window, cx: &mut App) -> Self {
        println!("{text}");

        Self {
            input_state: cx.new(|cx| InputState::new(window, cx).multi_line(true).default_value(text)),
        }
    }

    pub fn set_text(&self, window: &mut Window, cx: &mut App, text: SharedString) {
        self.input_state.update(cx, |this, cx| {
            this.set_value(text, window, cx);
        })
    }
}

impl Render for SelectableText {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Input::new(&self.input_state).appearance(false).h_full()
    }
}
