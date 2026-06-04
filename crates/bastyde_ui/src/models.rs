//! Layer A — backend-fed reactive models.
//!
//! Adapts the Qleany backend (commands + events) to a `bastyde::data::TreeModel`.
//! This is the part destined to be generalised and upstreamed into Qleany; for
//! the slice it covers just the binder-item tree.

use bastyde::data::{NodeId, TreeModel};

use frontend::AppContext;
use frontend::commands::{binder_commands, binder_item_commands, work_commands};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::direct_access::BinderItemDto;

/// One node in the navigation tree.
#[derive(Clone, Debug)]
pub struct TreeNode {
    pub title: String,
    /// The user-written note shown under the title (BinderItem.label).
    pub label: String,
    /// "binder" | "folder" | "item".
    pub kind: String,
}

impl TreeNode {
    pub fn binder(name: String) -> Self {
        Self { title: name, label: String::new(), kind: "binder".to_string() }
    }
    fn from_item(dto: &BinderItemDto) -> Self {
        Self { title: dto.title.clone(), label: dto.label.clone(), kind: dto.role.clone() }
    }
}

fn clear(model: &TreeModel<TreeNode>) {
    while model.root_count() > 0 {
        model.remove(model.root(0));
    }
}

/// Rebuild the whole tree from the in-memory store: Work → binders → items,
/// reconstructing nesting from each item's `indent`.
pub fn populate_from_backend(model: &TreeModel<TreeNode>, ctx: &AppContext) {
    clear(model);

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
            let binder_root = model.insert_root(model.root_count(), TreeNode::binder(binder.name));

            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                &binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            let items = binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                .unwrap_or_default();

            // Stack of (indent, node): a row's parent is the nearest ancestor
            // with a strictly smaller indent.
            let mut stack: Vec<(i64, NodeId)> = vec![(-1, binder_root)];
            for maybe in items.into_iter().flatten() {
                while stack.len() > 1 && stack.last().map(|(i, _)| *i).unwrap_or(-1) >= maybe.indent
                {
                    stack.pop();
                }
                let parent = stack.last().map(|(_, n)| *n).unwrap_or(binder_root);
                let node = model.insert_child(
                    parent,
                    model.child_count(parent),
                    TreeNode::from_item(&maybe),
                );
                stack.push((maybe.indent, node));
            }
        }
    }
}
