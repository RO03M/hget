use gpui::*;
use gpui_component::{
    IndexPath, button::Button, input::{Input, InputState}, select::{SearchableVec, Select, SelectState}
};
use hget_core::http_request::HttpRequest;

pub struct SendRequestEvent;

pub struct UrlInput {
    input_state: Entity<InputState>,
    select_state: Entity<SelectState<Vec<SharedString>>>,
    url: SharedString,
    method: SharedString,
}

impl EventEmitter<SendRequestEvent> for UrlInput {}

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

    pub fn on_request_change(&self, window: &mut Window, cx: &mut Context<Self>, http_request: Option<HttpRequest>) {
        let (url, method) = if let Some(http_request) = http_request.clone() {
            (http_request.url, http_request.method)
        } else {
            ("".into(), "".into())
        };

        self.input_state.update(cx, |state, cx| {
            state.set_value(url, window, cx);
        });

        self.select_state.update(cx, |state, cx| {
            state.set_selected_value(&method.into(), window, cx);
        });
    }

    pub fn get_url(&self, cx: &App) -> SharedString {
        return self.input_state.read(cx).value();
    }

    pub fn get_method(&self, cx: &App) -> SharedString {
        return self.select_state.read(cx).selected_value().unwrap_or(&"".into()).clone();
    }
}

impl Render for UrlInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().w_full().child(
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
                .child(Input::new(&self.input_state).flex_grow().w_0())
                .child(
                    Button::new("dispatch-request")
                        .label(">")
                        .on_click(cx.listener(|_this, _event, _window, cx| {
                            cx.emit(SendRequestEvent);
                        }))
                ),
        )
    }
}
