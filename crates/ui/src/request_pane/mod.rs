mod content;
mod url_input;

use std::path::PathBuf;

use gpui::*;
use gpui_component::{
    WindowExt,
    notification::NotificationType,
    tab::{Tab, TabBar},
};
use hget_core::http_request::HttpRequest;

use crate::{
    request_pane::{content::Content, url_input::UrlInput},
    state::State,
};

actions!(request_pane, [Save]);

pub struct RequestPane {
    url_input: Entity<UrlInput>,
    content: Entity<Content>,
    pending_update: bool,
    http_request: Option<HttpRequest>,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl RequestPane {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let url_input = cx.new(|cx| UrlInput::new(window, cx));
        let content = cx.new(|cx| Content::new(window, cx));

        let subscription = cx.observe_global::<State>(|this, cx| {
            this.pending_update = true;
            cx.notify();
        });

        cx.bind_keys([KeyBinding::new("ctrl-s", Save, None)]);

        Self {
            url_input,
            content,
            _subscriptions: vec![subscription],
            pending_update: true,
            http_request: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn load_current_request(&mut self, cx: &mut Context<Self>) {
        let state = cx.global::<State>();

        let repository = state.repository.clone().unwrap();
        let (http_request, raw) = repository
            .get_http_file(&std::path::Path::new(
                &state.active_path.clone().unwrap().to_string(),
            ))
            .unwrap();

        self.http_request = Some(http_request);
        cx.notify();
    }

    pub fn on_save(&self, window: &mut Window, cx: &mut Context<Self>) {
        let state = cx.global::<State>();
        let Some(repository) = state.repository.clone() else {
            return;
        };

        let Some(active_path) = state.active_path.clone() else {
            return;
        };

        let url_input = self.url_input.read(cx);

        let url = url_input.get_url(cx);
        let method = url_input.get_method(cx);

        let headers = self.content.read(cx).get_headers(cx);

        let mut http_request = HttpRequest::new(url, method);
        http_request.set_headers(headers);

        let result = repository.save_http_file(&http_request, &PathBuf::from(active_path.as_str()));

        if result.is_ok() {
            window.push_notification(
                (NotificationType::Success, "Request saved successfully"),
                cx,
            );
        } else {
            window.push_notification(
                (NotificationType::Error, "Something went wrong"),
                cx,
            );
        }

        println!("{:?}", http_request);
    }
}

impl Render for RequestPane {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.pending_update {
            self.load_current_request(cx);
            let http_request = self.http_request.clone();

            self.content.update(cx, |this, cx| {
                this.notify_request_change(window, cx, http_request.clone());
            });

            self.url_input.update(cx, |this, cx| {
                this.on_request_change(window, cx, http_request);
            });

            self.pending_update = false;
        }

        return div()
            .track_focus(&self.focus_handle)
            .flex_col()
            .flex_1()
            .bg(rgb(0x0000ff))
            .on_action(cx.listener(|this, _: &Save, window, cx| {
                this.on_save(window, cx);
            }))
            .child(self.url_input.clone())
            .child(self.content.clone());
    }
}
