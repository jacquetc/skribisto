//! The Export **Choose…** checkbox tree — the "Customize selection" surface.
//!
//! A self-contained tri-state tree built from the client-gathered Work snapshot (the same
//! `Gathered` the preview uses), so it needs no changes to the shared outline model. Each
//! item is a `Checkbox::tristate` bound to a [`TreeCheckedModel`] node signal, which gives
//! the "Outlook folder" cascade for free (checking a chapter checks its scenes; a mixed
//! chapter shows the indeterminate dash). The checked leaves' ids become the `Custom` export
//! scope's `binder_item_ids`.
//!
//! Non-exportable items (the Inspector's per-item toggle) are hidden by default; a panel
//! "Show non-exportable" toggle rebuilds the tree with them revealed (a revealed check is an
//! explicit per-export override). The default check = `activated && is_exportable && prose`
//! (scenes + notes); folders derive their state by aggregation.

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::core::ObserverHandle;
use bastyde::data::{CheckState, NodeId, TreeModel};
use bastyde::data::TreeCheckedModel;
use bastyde::prelude::*;
use bastyde::widgets::{StandardTreeItem, TextWidget, TreeView};

use frontend::common::entities::BinderItemSubRole;
use skrib_format::Gathered;
use skribisto_model::SubRoleExt;

/// One row of the Choose tree.
#[derive(Clone)]
pub struct ChooseNode {
    /// The `BinderItem` id, or `None` for a binder root row.
    pub item_id: Option<u64>,
    pub title: String,
    pub sub_role: BinderItemSubRole,
    pub kind: String, // "binder" | "folder" | "item"
    pub exportable: bool,
}

impl ChooseNode {
    fn binder(name: String) -> Self {
        Self {
            item_id: None,
            title: name,
            sub_role: BinderItemSubRole::default(),
            kind: "binder".to_string(),
            exportable: true,
        }
    }
}

/// The tree + its check model, plus the observers that bump `changed` on any check so the
/// panel's live preview refreshes. Cloneable — every field is an `Rc`-backed handle.
#[derive(Clone)]
pub struct ChooseModel {
    pub tree: TreeModel<ChooseNode>,
    pub checked: TreeCheckedModel<ChooseNode>,
    _observers: Rc<Vec<ObserverHandle>>,
}

impl ChooseModel {
    /// Build the tree from a gathered snapshot. `show_non_exportable` reveals items the
    /// user marked non-exportable; otherwise only exportable, activated items appear.
    /// `changed` is bumped on any check change (the caller owns it, stable across rebuilds,
    /// so the preview binding survives a "show non-exportable" toggle).
    pub fn build(g: &Gathered, show_non_exportable: bool, changed: Signal<u64>) -> Self {
        let tree: TreeModel<ChooseNode> = TreeModel::new();
        for bwi in &g.binders {
            let broot = tree.insert_root(
                tree.root_count(),
                ChooseNode::binder(if bwi.binder.name.is_empty() {
                    "Binder".to_string()
                } else {
                    bwi.binder.name.clone()
                }),
            );
            // Indent → tree: a binder is depth 0, an item is depth `indent + 1`; each item
            // attaches under the nearest shown ancestor of smaller depth.
            let mut stack: Vec<(usize, NodeId)> = vec![(0, broot)];
            for iwc in &bwi.items {
                let it = &iwc.item;
                if !it.activated {
                    continue; // trashed items are hidden
                }
                if !show_non_exportable && !it.is_exportable {
                    continue;
                }
                let depth = (it.indent.max(0) as usize) + 1;
                while stack.last().map(|(d, _)| *d >= depth).unwrap_or(false) {
                    stack.pop();
                }
                let parent = stack.last().map(|(_, id)| *id).unwrap_or(broot);
                let kind = match it.role {
                    frontend::common::entities::BinderItemRole::Folder => "folder",
                    frontend::common::entities::BinderItemRole::Item => "item",
                }
                .to_string();
                let node = tree.insert_child(
                    parent,
                    tree.child_count(parent),
                    ChooseNode {
                        item_id: Some(it.id),
                        title: it.title.clone(),
                        sub_role: it.sub_role.clone(),
                        kind,
                        exportable: it.is_exportable,
                    },
                );
                stack.push((depth, node));
            }
        }

        let checked = TreeCheckedModel::new(tree.clone());
        // Seed: check every exportable prose row (scene / note); folders derive their state
        // by aggregation, markers/titles stay unchecked. Order-independent — all seeds agree.
        for node in all_nodes(&tree) {
            let seed = tree
                .with_item(node, |n| {
                    n.exportable && is_prose(&n.sub_role) && n.item_id.is_some()
                })
                .unwrap_or(false);
            if seed {
                checked.check(node);
            }
        }

        // One observer per node bumps `changed`, so a check anywhere refreshes the preview.
        let mut observers = Vec::new();
        for node in all_nodes(&tree) {
            let c = changed.clone();
            observers.push(
                checked
                    .signal_for(node)
                    .observe(move |_| c.set(c.get().wrapping_add(1))),
            );
        }

        Self { tree, checked, _observers: Rc::new(observers) }
    }

    /// Re-apply a checked set of item ids (leaves only; folders re-derive by aggregation) —
    /// used to preserve the user's choices across a "show non-exportable" rebuild.
    pub fn apply_checked(&self, ids: &[u64]) {
        let want: std::collections::HashSet<u64> = ids.iter().copied().collect();
        for node in all_nodes(&self.tree) {
            if self.tree.has_children(node) {
                continue; // a container derives its state from its descendants
            }
            if let Some(Some(id)) = self.tree.with_item(node, |n| n.item_id) {
                if want.contains(&id) {
                    self.checked.check(node);
                } else {
                    self.checked.uncheck(node);
                }
            }
        }
    }

    /// The checked item ids, in tree (document) order — the `Custom` scope's include set.
    /// Binder rows (no `item_id`) are dropped; a checked folder keeps its id so its heading
    /// renders.
    pub fn checked_item_ids(&self) -> Vec<u64> {
        // Walk the tree in order and keep the checked ones, so the export reads top-to-bottom
        // (checked_nodes() alone is unordered).
        let mut out = Vec::new();
        for node in all_nodes(&self.tree) {
            if self.checked.check_state(node) == CheckState::Checked
                && let Some(Some(id)) = self.tree.with_item(node, |n| n.item_id)
            {
                out.push(id);
            }
        }
        out
    }
}

/// Every `NodeId` in the tree, in pre-order (roots then descendants, document order).
fn all_nodes(tree: &TreeModel<ChooseNode>) -> Vec<NodeId> {
    let mut out = Vec::new();
    for r in 0..tree.root_count() {
        push_subtree(tree, tree.root(r), &mut out);
    }
    out
}

fn push_subtree(tree: &TreeModel<ChooseNode>, node: NodeId, out: &mut Vec<NodeId>) {
    out.push(node);
    for c in tree.children(node) {
        push_subtree(tree, c, out);
    }
}

fn is_prose(sr: &BinderItemSubRole) -> bool {
    sr.carries_scene() || matches!(sr, BinderItemSubRole::Note)
}

/// The checkbox tree widget: a `TreeView` over the [`ChooseModel`], each row a tri-state
/// checkbox + icon + title. Renders an empty state when there is no model (no project).
pub struct ChooseTreeWidget {
    model: Option<ChooseModel>,
    root_child: Option<WidgetId>,
}

impl ChooseTreeWidget {
    pub fn new(model: Option<ChooseModel>) -> Self {
        Self { model, root_child: None }
    }
}

impl std::fmt::Debug for ChooseTreeWidget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChooseTreeWidget").finish()
    }
}

impl Widget for ChooseTreeWidget {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let Some(model) = self.model.clone() else {
            let id = ctx.add(
                bastyde::widgets::Center::new().child(
                    TextWidget::new(tr!(export_choose_empty()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
            );
            self.root_child = Some(id);
            return vec![id];
        };
        let checked = model.checked.clone();
        let tree = model.tree.clone();
        // A "choose what to export" tree is most useful fully open — the writer sees every
        // scene to check/uncheck without hunting through collapsed binders. The TreeView owns
        // its expand state (defaults collapsed) with no pre-expand builder; the only handle to
        // the slice is the per-row `TreeRowContext`. Capture it on the first row built, then
        // `expand_all` from a frame-tick effect — NOT inside the delegate, where the slice is
        // already borrowed (that re-entrant borrow panics).
        let expand_fn: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::new(RefCell::new(None));
        let capture = expand_fn.clone();
        let view = TreeView::new_with_context(
            tree.clone(),
            move |node: &ChooseNode, entry, selected, rowctx| {
                if capture.borrow().is_none() {
                    let handle = rowctx.slice_handle().clone();
                    *capture.borrow_mut() = Some(Rc::new(move || handle.expand_all()));
                }
                let sig = checked.signal_for(entry.node_id);
                let icon = if node.kind == "binder" {
                    crate::binder_icons::binder_icon()
                } else {
                    crate::binder_icons::sub_role_icon(&node.sub_role)
                };
                let title_color = if node.exportable {
                    TextRole::Primary
                } else {
                    TextRole::Secondary
                };
                // `StandardTreeItem` gives the depth indentation, the expand/collapse chevron
                // (via has_children/is_expanded/on_toggle), the tri-state checkbox and the
                // leading sub-role icon — the same chrome the outline tree uses.
                Box::new(
                    StandardTreeItem::new(lit!(node.title.clone()))
                        .depth(entry.depth)
                        .has_children(entry.has_children)
                        .is_expanded(entry.is_expanded)
                        .selected(selected)
                        .on_toggle_rc(rowctx.toggle_callback())
                        .tristate_checkbox(sig)
                        .leading_slot(icon)
                        .label_color(title_color),
                ) as Box<dyn Widget>
            },
        )
        .item_height(28.0);

        let id = ctx.add(view);
        // Fire the captured `expand_all` once, on the first frame after a row populated the
        // slice handle — safely outside the delegate's borrow.
        let done = Rc::new(std::cell::Cell::new(false));
        let tick = ctx.frame_tick();
        ctx.effect(&tick, move |_| {
            if !done.get()
                && let Some(f) = expand_fn.borrow().clone()
            {
                f();
                done.set(true);
            }
        });
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::common::entities::{Binder, BinderItem, BinderItemRole, Work};
    use skrib_format::{BinderWithItems, ItemWithContents};

    fn item(id: u64, role: BinderItemRole, sub_role: BinderItemSubRole, indent: i64, exportable: bool) -> ItemWithContents {
        ItemWithContents {
            item: BinderItem {
                id,
                role,
                sub_role,
                indent,
                activated: true,
                is_exportable: exportable,
                title: format!("Item {id}"),
                ..Default::default()
            },
            contents: vec![],
        }
    }

    fn gathered(items: Vec<ItemWithContents>) -> Gathered {
        Gathered {
            work: Work { id: 1, ..Default::default() },
            tags: vec![],
            dict_words: vec![],
            trash_infos: vec![],
            paces: vec![],
            progress_snapshots: vec![],
            binders: vec![BinderWithItems {
                binder: Binder { id: 10, name: "Manuscript".into(), ..Default::default() },
                items,
            }],
            work_info: None,
        }
    }

    #[test]
    fn seeds_prose_checked_and_reports_ids() {
        use BinderItemRole::{Folder, Item};
        use BinderItemSubRole as SR;
        let g = gathered(vec![
            item(1, Folder, SR::ChapterScene, 0, true), // chapter folder
            item(2, Item, SR::Scene, 1, true),          // prose → checked
            item(3, Item, SR::Scene, 1, true),          // prose → checked
            item(4, Item, SR::Note, 1, true),           // prose (note) → checked
        ]);
        let m = ChooseModel::build(&g, false, Signal::new(0));
        let ids = m.checked_item_ids();
        assert!(ids.contains(&2) && ids.contains(&3) && ids.contains(&4), "prose seeded: {ids:?}");
        // The chapter folder aggregates to Checked (all children checked) → its heading id in.
        assert!(ids.contains(&1), "a fully-checked chapter folder is included: {ids:?}");
    }

    #[test]
    fn non_exportable_items_are_hidden_by_default_and_revealed_on_demand() {
        use BinderItemRole::Item;
        use BinderItemSubRole as SR;
        let g = gathered(vec![
            item(1, Item, SR::Scene, 0, true),
            item(2, Item, SR::Scene, 0, false), // non-exportable
        ]);
        // Hidden: only the exportable scene is seeded/checked.
        let hidden = ChooseModel::build(&g, false, Signal::new(0));
        assert_eq!(hidden.checked_item_ids(), vec![1]);
        // Revealed: the non-exportable scene appears but is NOT default-checked.
        let shown = ChooseModel::build(&g, true, Signal::new(0));
        assert_eq!(shown.checked_item_ids(), vec![1], "revealed non-exportable stays unchecked");
    }

    #[test]
    fn unchecking_a_scene_drops_it_from_the_ids() {
        use BinderItemRole::Item;
        use BinderItemSubRole as SR;
        let g = gathered(vec![item(1, Item, SR::Scene, 0, true), item(2, Item, SR::Scene, 0, true)]);
        let m = ChooseModel::build(&g, false, Signal::new(0));
        // Uncheck scene 1 via its node.
        let n1 = m.tree.find_by(|cn| cn.item_id == Some(1)).unwrap();
        m.checked.uncheck(n1);
        assert_eq!(m.checked_item_ids(), vec![2]);
    }
}
