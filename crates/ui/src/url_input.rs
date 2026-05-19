use gpui::*;
use gpui_component::{
    IndexPath,
    input::{Input, InputState},
    select::{SearchableVec, Select, SelectState},
};

pub struct UrlInput {
    input_state: Entity<InputState>,
    select_state: Entity<SelectState<Vec<SharedString>>>,
    url: SharedString,
    method: SharedString,
}

impl UrlInput {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx));

        let select_state = cx.new(|cx| {
            SelectState::new(
                vec![
                    SharedString::new("GET"),
                    SharedString::new("POST"),
                    SharedString::new("PUT"),
                    SharedString::new("DELETE"),
                    SharedString::new("PATCH"),
                    SharedString::new("OPTIONS"),
                    SharedString::new("HEAD"),
                    SharedString::new("TRACE"),
                    SharedString::new("CONNECT"),
                ],
                None,
                window,
                cx,
            )
        });

        select_state.update(cx, |state, cx| {
            state.set_selected_index(Some(IndexPath::default()), window, cx);
        });

        Self {
            input_state,
            select_state,
            method: "GET".into(),
            url: "".into(),
        }
    }
}

impl Render for UrlInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        println!("{:?}", self.select_state.read(cx).selected_value());

        div().flex().w_full().child(
            // Wrapping everything inside a single unified boundary container
            div()
                .flex()
                .flex_row()
                .items_center()
                .w_full()
                .border_1()
                .child(
                    div()
                        .flex_shrink_0()
                        .w_auto()
                        .child(
                            Select::new(&self.select_state)
                                .menu_width(px(110.0))
                        ),
                )
                .child(Input::new(&self.input_state).flex_grow().w_0()),
        )
    }
}
