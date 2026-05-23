use gpui::BorrowAppContext;

use crate::{state, theme, window, workspace};

pub fn run_app(app: gpui::Application) {
    app.run(move |cx| {
        gpui_component::init(cx);

        theme::init(cx);
        state::init(cx);

        cx.update_global::<state::State, _>(|state, cx| {
            state.load_repository_at("./sample".into());
        });

        cx.spawn(async move |cx| {
            let _ = cx.open_window(window::get_window_options(), workspace::build_workspace_view);
        })
        .detach();
    });
}
