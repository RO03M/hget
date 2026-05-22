use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{
    Root,
    resizable::{h_resizable, resizable_panel},
};

use crate::{file_tree::FileTree, request_pane::RequestPane, response_pane::ResponsePane};

pub fn build_workspace_view(window: &mut Window, cx: &mut App) -> Entity<Root> {
    let view = cx.new(|cx| Workspace::new(window, cx));

    return cx.new(|cx| Root::new(view, window, cx));
}

pub struct Workspace {
    file_tree: Entity<FileTree>,
    request_pane: Entity<RequestPane>,
    response_pane: Entity<ResponsePane>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        Self {
            file_tree: cx.new(|cx| FileTree::new(cx)),
            request_pane: cx.new(|cx| RequestPane::new(window, cx)),
            response_pane: cx.new(|_| ResponsePane::new()),
        }
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notification_layer = Root::render_notification_layer(window, cx);

        return div()
            .size_full()
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
