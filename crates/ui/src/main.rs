use gpui::*;
use gpui_component::{Root, h_flex, list::ListItem, tree::{TreeItem, TreeState}};
use hget_core::helpers::list_http_tree;
use rfd::FileDialog;
use ui::{app::run_app, dir_picker, repository_tree, state};

struct HgetUI {
    tree_state: Entity<TreeState>,
}

impl HgetUI {
    pub fn new(window: &mut Window, cx: &mut gpui::App) -> Self {
        let tree_state = cx.new(|cx| {
            TreeState::new(cx).items(vec![
                TreeItem::new("src", "src")
                    .expanded(true)
                    .child(TreeItem::new("src/lib.rs", "lib.rs"))
                    .child(TreeItem::new("src/main.rs", "main.rs")),
                TreeItem::new("Cargo.toml", "Cargo.toml"),
                TreeItem::new("README.md", "README.md"),
            ])
        });

        return Self {
            tree_state: tree_state,
        };
    }
}

impl Render for HgetUI {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        return div()
            .size_full()
            .child(
                dir_picker::DirPicker {}
            );
    }
}

fn main() {
    run_app(gpui_platform::application());
}
