use gpui::*;
use gpui_component::{h_flex, list::ListItem, tree::{TreeState, tree}};
use hget_core::repository::Repository;

pub struct RepositoryTreeState {
    tree_state: Entity<TreeState>
}

pub struct RepositoryTree {
    // state: Entity<RepositoryTreeState>
}

// impl RepositoryTree {
//     pub fn new(repository: Repository) -> Self {
//         println("creating repository tree");
//         let tree = 

//         Self {
//             state: 
//         }
//     }
// }

impl Render for RepositoryTree {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            // .child(
            //     tree(&self.state.read(cx).tree_state, |ix, entry, selected, window, cx| {
            //         ListItem::new(ix).child(h_flex().gap_2().child(entry.item().label.clone()))
            //     })
            // )
    }
}