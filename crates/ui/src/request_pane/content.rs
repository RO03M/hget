use std::path::Path;

use gpui::*;
use gpui_component::{
    tab::{Tab, TabBar},
    table::{Table, TableCell, TableHead, TableHeader, TableRow},
};
use hget_core::http_request::HttpRequest;

use crate::{components::key_value_table::KeyValueTable, state::State};

pub struct Content {
    active_tab: usize,
    headers_table: Entity<KeyValueTable>,
}

impl Content {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let headers_table = cx.new(|cx| KeyValueTable::new(window, cx));

        // let state = cx.global::<State>();

        // let repository = state.repository.clone().unwrap();
        // let (http_request, raw) = repository.get_http_file(&Path::new(&state.active_path.clone().unwrap().to_string())).unwrap();

        // let subscription = cx.observe_global::<State>(|this, cx| {
        //     this._pending_update = true;
        //     cx.notify();
        // });
        
        // this.headers_table.update(cx, |table, cx| {
        //     table.set_rows(window, cx, http_request.headers.clone());
        // });
        
        Self {
            active_tab: 0,
            headers_table,
        }
    }

    // pub fn load_path(&mut self, cx: &mut Context<Self>)  {
    //     let state = cx.global::<State>();

    //     let repository = state.repository.clone().unwrap();
    //     let (http_request, raw) = repository.get_http_file(&Path::new(&state.active_path.clone().unwrap().to_string())).unwrap();

    //     self.http_request = Some(http_request);
    //     cx.notify();
    // }

    pub fn notify_request_change(&mut self, window: &mut Window, cx: &mut Context<Self>, http_request: Option<HttpRequest>) {
        if http_request.is_none() {
            self.headers_table.update(cx, move |table, cx| {
                table.set_rows(window, cx, http_request.clone().unwrap().headers.clone());
            });

            return;
        }

        self.headers_table.update(cx, move |table, cx| {
            table.set_rows(window, cx, http_request.clone().unwrap().headers.clone());
        });
    }

    pub fn get_headers(&self, cx: &App) -> Vec<(SharedString, SharedString)> {
        return self.headers_table.read(cx).read_rows_as_vec(cx);
    }
}

impl Render for Content {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // if self._pending_update {
        //     self.load_path(cx);
            
        //     let http_request = self.http_request.clone();
        //     self.headers_table.update(cx, move |table, cx| {
        //         table.set_rows(window, cx, http_request.clone().unwrap().headers.clone());
        //     });

        //     self._pending_update = false;
        // }
        div()
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
                0 => self.headers_table.clone().into_any_element(),
                1 => div().child("rendering profile").into_any_element(),
                2 => div().child("rendering documents").into_any_element(),
                _ => div().child("what?").into_any_element(),
            })
    }
}
