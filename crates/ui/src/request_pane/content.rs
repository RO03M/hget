use gpui::*;
use gpui_component::tab::{Tab, TabBar};
use hget_core::http_request::HttpRequest;

use crate::components::{key_value_table::KeyValueTable, selectable_text::SelectableText};

pub struct Content {
    active_tab: usize,
    headers_table: Entity<KeyValueTable>,
    raw_file: Entity<SelectableText>,
}

impl Content {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let headers_table = cx.new(|cx| KeyValueTable::new(window, cx));

        Self {
            active_tab: 0,
            headers_table,
            raw_file: cx.new(|cx| SelectableText::new("".into(), window, cx)),
        }
    }

    pub fn notify_request_change(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
        http_request: Option<HttpRequest>,
    ) {
        if http_request.is_none() {
            self.headers_table.update(cx, |table, cx| {
                table.set_rows(window, cx, Vec::new());
            });

            self.raw_file.update(cx, |this, cx| {
                this.set_text(window, cx, "".into());
            });
            return;
        }

        self.headers_table.update(cx, |table, cx| {
            table.set_rows(window, cx, http_request.clone().unwrap().headers.clone());
        });

        self.raw_file.update(cx, |this, cx| {
            this.set_text(window, cx, http_request.clone().unwrap().to_string().into());
        });
    }

    pub fn get_headers(&self, cx: &App) -> Vec<(SharedString, SharedString)> {
        return self.headers_table.read(cx).read_rows_as_vec(cx);
    }
}

impl Render for Content {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h_full()
            .child(
                TabBar::new("default-tabs")
                    .selected_index(self.active_tab)
                    .on_click(cx.listener(|view, index, _, cx| {
                        view.active_tab = *index;
                        cx.notify();
                    }))
                    .child(Tab::new().label("Params"))
                    .child(Tab::new().label("Body"))
                    .child(Tab::new().label("Headers"))
                    .child(Tab::new().label("Auth"))
                    .child(Tab::new().label("File")),
            )
            .child(match self.active_tab {
                0 => div().child("rendering params").into_any_element(),
                1 => div().child("rendering body").into_any_element(),
                2 => self.headers_table.clone().into_any_element(),
                3 => div().child("auth").into_any_element(),
                4 => self.raw_file.clone().into_any_element(),
                _ => div().child("what?").into_any_element(),
            })
    }
}
