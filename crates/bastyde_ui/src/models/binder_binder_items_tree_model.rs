//! Reactive model for `Binder.binder_items` — the binder-item tree (flattened
//! via each item's `indent`).
//!
//! Self-contained reference shape for a future Qleany tera template: this file
//! owns BOTH a real and a mock `BinderBinderItemsTreeModel` with an identical
//! public API, selected by the `mocks` feature. The only `#[cfg]` in the file
//! gates the two inner `imp` modules; nothing outside the file is ever gated.

/// One node in the navigation tree. Shared by both variants.
#[derive(Clone, Debug, Default)]
pub struct TreeNode {
    pub title: String,
    /// The user-written note shown under the title (`BinderItem.label`).
    pub label: String,
    /// `"binder"` | `"folder"` | `"item"`.
    pub kind: String,
}

impl TreeNode {
    pub fn binder(name: String) -> Self {
        Self { title: name, label: String::new(), kind: "binder".to_string() }
    }
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use super::TreeNode;
    use bastyde::data::{NodeId, TreeModel};
    use std::rc::Rc;

    use frontend::AppContext;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::BinderItemRole;
    use frontend::direct_access::BinderItemDto;

    /// Backed by the Qleany store: `reload()` pulls Work → binders → items.
    #[derive(Clone)]
    pub struct BinderBinderItemsTreeModel {
        tree: TreeModel<TreeNode>,
        ctx: Rc<AppContext>,
    }

    impl BinderBinderItemsTreeModel {
        pub fn new(ctx: Rc<AppContext>) -> Self {
            Self { tree: TreeModel::new(), ctx }
        }

        /// The `bastyde` tree model to bind to a `TreeView`.
        pub fn tree(&self) -> &TreeModel<TreeNode> {
            &self.tree
        }

        /// Rebuild the tree from the in-memory store: Work → binders → items,
        /// reconstructing nesting from each item's `indent`.
        pub fn reload(&self) {
            clear(&self.tree);
            let ctx = &*self.ctx;

            let works = match work_commands::get_all_work(ctx) {
                Ok(w) => w,
                Err(_) => return,
            };

            for work in &works {
                let binder_ids = work_commands::get_work_relationship(
                    ctx,
                    &work.id,
                    &WorkRelationshipField::Binders,
                )
                .unwrap_or_default();

                for binder_id in binder_ids {
                    let Ok(Some(binder)) = binder_commands::get_binder(ctx, &binder_id) else {
                        continue;
                    };
                    let binder_root =
                        self.tree.insert_root(self.tree.root_count(), TreeNode::binder(binder.name));

                    let item_ids = binder_commands::get_binder_relationship(
                        ctx,
                        &binder_id,
                        &BinderRelationshipField::BinderItems,
                    )
                    .unwrap_or_default();
                    let items = binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                        .unwrap_or_default();

                    // Stack of (indent, node): a row's parent is the nearest
                    // ancestor with a strictly smaller indent.
                    let mut stack: Vec<(i64, NodeId)> = vec![(-1, binder_root)];
                    for maybe in items.into_iter().flatten() {
                        while stack.len() > 1
                            && stack.last().map(|(i, _)| *i).unwrap_or(-1) >= maybe.indent
                        {
                            stack.pop();
                        }
                        let parent = stack.last().map(|(_, n)| *n).unwrap_or(binder_root);
                        let node = self.tree.insert_child(
                            parent,
                            self.tree.child_count(parent),
                            node_from_item(&maybe),
                        );
                        stack.push((maybe.indent, node));
                    }
                }
            }
        }
    }

    fn clear(tree: &TreeModel<TreeNode>) {
        while tree.root_count() > 0 {
            tree.remove(tree.root(0));
        }
    }

    fn node_from_item(dto: &BinderItemDto) -> TreeNode {
        let kind = match dto.role {
            BinderItemRole::Folder => "folder",
            BinderItemRole::Item => "item",
        }
        .to_string();
        TreeNode { title: dto.title.clone(), label: dto.label.clone(), kind }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use super::TreeNode;
    use bastyde::data::TreeModel;
    use std::rc::Rc;

    use frontend::AppContext;

    /// Fabricates a small binder tree so the UI runs with no real backend.
    #[derive(Clone)]
    pub struct BinderBinderItemsTreeModel {
        tree: TreeModel<TreeNode>,
    }

    impl BinderBinderItemsTreeModel {
        pub fn new(_ctx: Rc<AppContext>) -> Self {
            let tree = TreeModel::new();
            populate(&tree);
            Self { tree }
        }

        pub fn tree(&self) -> &TreeModel<TreeNode> {
            &self.tree
        }

        /// No-op: the mock tree is static, filled at construction.
        pub fn reload(&self) {}
    }

    fn node(title: &str, label: &str, kind: &str) -> TreeNode {
        TreeNode { title: title.to_string(), label: label.to_string(), kind: kind.to_string() }
    }

    fn populate(tree: &TreeModel<TreeNode>) {
        let manuscript = tree.insert_root(0, TreeNode::binder("Manuscript".to_string()));
        let chapter = tree.insert_child(manuscript, 0, node("Chapter 1", "the setup", "folder"));
        tree.insert_child(chapter, 0, node("Opening scene", "1st plot point", "item"));
        tree.insert_child(chapter, 1, node("Inciting incident", "", "item"));
        let chapter2 =
            tree.insert_child(manuscript, 1, node("Chapter 2", "rising action", "folder"));
        tree.insert_child(chapter2, 0, node("The journey begins", "", "item"));

        let notes = tree.insert_root(1, TreeNode::binder("Notes".to_string()));
        tree.insert_child(notes, 0, node("Protagonist", "wants freedom", "item"));
        tree.insert_child(notes, 1, node("Antagonist", "", "item"));
    }
}

pub use imp::BinderBinderItemsTreeModel;
