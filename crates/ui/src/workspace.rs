use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    ActiveTheme, Root, resizable::{h_resizable, resizable_panel}
};

use crate::{file_tree::FileTree, request_pane::{RequestPane, ResponseReceivedEvent}, response_pane::ResponsePane};

pub fn build_workspace_view(window: &mut Window, cx: &mut App) -> Entity<Root> {
    let view = cx.new(|cx| Workspace::new(window, cx));

    return cx.new(|cx| Root::new(view, window, cx));
}

pub struct Workspace {
    file_tree: Entity<FileTree>,
    request_pane: Entity<RequestPane>,
    response_pane: Entity<ResponsePane>,
    _subscriptions: Subscription,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let request_pane = cx.new(|cx| RequestPane::new(window, cx));
        let response_pane = cx.new(|cx| ResponsePane::new(window, cx));

        let response_sub = cx.subscribe(&request_pane, {
            let response_pane = response_pane.clone();
            move |_, event: &ResponseReceivedEvent, cx| {
                response_pane.update(cx, |pane, cx| {
                    pane.set_response(cx, event.0.clone());
                });
            }
        });

        Self {
            file_tree: cx.new(|cx| FileTree::new(cx)),
            request_pane,
            response_pane,
            _subscriptions: response_sub,
        }
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notification_layer = Root::render_notification_layer(window, cx);

        return div()
            .size_full()
            .bg(cx.theme().background.to_rgb())
            .child(
                h_resizable("main-resizable")
                    .child(resizable_panel().child(self.file_tree.clone()))
                    .child(resizable_panel().child(self.request_pane.clone()))
                    .child(resizable_panel().child(self.response_pane.clone())),
            )
            .when(notification_layer.is_some(), |this| {
                this.child(notification_layer.unwrap())
            });
    }
}
