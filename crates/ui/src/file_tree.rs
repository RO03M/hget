use gpui::*;
use gpui_component::{h_flex, list::ListItem, tree::{tree, TreeItem, TreeState}};
use hget_core::helpers::{FSNode, list_http_tree};

use crate::state::{self, State};

pub fn convert_fs_nodes(nodes: &Vec<FSNode>) -> Vec<TreeItem> {
    // Start with an empty base parent path
    map_nodes_recursive(&nodes, "")
}

/// Recursive helper that carries the accumulating parent path down the tree
fn map_nodes_recursive(nodes: &Vec<FSNode>, parent_path: &str) -> Vec<TreeItem> {
    nodes
        .iter()
        .map(|node| {
            let current_id = if parent_path.is_empty() {
                node.name.clone()
            } else {
                format!("{}/{}", parent_path, node.name)
            };

            let mut item = TreeItem::new(&current_id, &node.name);

            if node.is_dir {
                item = item.expanded(true);

                let converted_children = map_nodes_recursive(&node.children, &current_id);

                for child in converted_children {
                    item = item.child(child);
                }
            }

            item
        })
        .collect()
}

pub struct FileTree {
    tree_state: Entity<TreeState>,
}

impl FileTree {
    pub fn new(cx: &mut App) -> Self {
        let state = cx.global::<state::State>();
        let repository = state.repository.clone().unwrap();

        // if let Some(repository) = &repository {
        // }
        let foo = list_http_tree(&repository.root);
        let tree_items = convert_fs_nodes(&foo);

        Self {
            tree_state: cx.new(|cx| TreeState::new(cx).items(tree_items)),
        }
    }

    fn on_select_path(&self, cx: &mut App, path: SharedString) {
        cx.update_global::<State, _>(|state, _cx| {
            state.active_path = Some(path);
        });
    }
}

impl Render for FileTree {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        return div()
            .size_full()
            .child(tree(&self.tree_state, move |ix, entry, selected, window, cx| {
                let path = entry.item().id.clone();

                ListItem::new(ix)
                    .child(h_flex()
                    .gap_2()
                    .child(entry.item().label.clone()))
                    .on_click({
                        let value = entity.clone();
                        move |event, window, cx| {
                            let path = path.clone();
                            value.update(cx, move |this, cx| {
                                this.on_select_path(cx, path);
                            });
                        }
                    })
                
            }));
    }
}
