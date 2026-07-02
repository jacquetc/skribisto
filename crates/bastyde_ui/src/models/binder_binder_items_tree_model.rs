//! Reactive model for the binder-item tree, backed by a Bastyde
//! [`TreeDataSlice`] so a `TreeView` can render, navigate, expand and
//! **drag-reorder** it. The flat item stream nests via each item's `indent`;
//! binders are the tree roots.
//!
//! This type is now a thin **domain facade**: the whole tree algorithm
//! (indent → tree derivation, per-view expand state, collapse-aware flattening,
//! [`first_changed_index`](bastyde::data::TreeDataSource::first_changed_index)
//! divergence, DnD cycle guard + plumbing) lives once in `bastyde_data`'s
//! `TreeDataSlice`. Skribisto supplies only what is genuinely domain-specific:
//! the tagged key ([`BinderTreeKey`]), the row payload ([`TreeNode`]), the row
//! source ([`rows::load`], the sole real-vs-mock seam), the drag/drop policy,
//! and the reorder command ([`CommitMove`], injected via `set_reorder`). The
//! `TreeDataSource` impl is a straight delegation onto the slice.
//!
//! Rows are sourced for the **open Work only**, identified by the `work_id`
//! signal from [`AppIds`](crate::app_ids) (ids-only global state) — no
//! `get_all_work`. Mutations are not applied here: drops route through the
//! injected [`CommitMove`] closure and the slice re-reads itself.

use std::rc::Rc;

use bastyde::data::{
    DragEligibility, DropCommit, DropPosition, DropQuery, DropResponse, FlatEntry, TreeDataSlice,
    TreeDataSource,
};
use bastyde::prelude::Signal;

use frontend::AppContext;
use frontend::common::entities::BinderItemSubRole;

/// Stable per-row identity. Binders and items share the row space but live in
/// disjoint id namespaces in the backend, so the key is tagged.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinderTreeKey {
    Binder(u64),
    Item(u64),
}

/// One node in the navigation tree. Shared by both variants. `PartialEq` powers
/// the slice's divergence check (a content edit whose structure is unchanged is
/// still detected, so a consumer caching row heights re-measures only the
/// changed rows).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TreeNode {
    pub title: String,
    /// The user-written note shown under the title (`BinderItem.label`).
    pub label: String,
    /// `"binder"` | `"folder"` | `"item"`.
    pub kind: String,
    /// The item's structural sub-role — selects the leading icon. Binder rows
    /// keep the default; their icon comes from the `kind == "binder"` branch.
    pub sub_role: BinderItemSubRole,
    /// The `BinderItem` id (`None` for binder rows) — used to open its editor.
    pub item_id: Option<u64>,
    /// The owning `Binder` id (`Some` for binder rows and item rows alike).
    pub binder_id: Option<u64>,
}

impl TreeNode {
    pub fn binder(name: String, binder_id: u64) -> Self {
        Self {
            title: name,
            label: String::new(),
            kind: "binder".to_string(),
            sub_role: BinderItemSubRole::default(),
            item_id: None,
            binder_id: Some(binder_id),
        }
    }
}

/// Reorder hook injected by the view-model: `(dragged, target, position) ->
/// applied`. Applies the move through the backend (with undo) and reports
/// whether it took.
pub type CommitMove = Rc<dyn Fn(BinderTreeKey, BinderTreeKey, DropPosition) -> bool>;

#[derive(Clone)]
pub struct BinderBinderItemsTreeModel {
    slice: TreeDataSlice<BinderTreeKey, TreeNode>,
}

impl BinderBinderItemsTreeModel {
    pub fn new(ctx: Rc<AppContext>, work_id: Signal<Option<u64>>) -> Self {
        let slice = TreeDataSlice::new();
        // New nodes (e.g. a freshly-created scene) appear expanded; the user's
        // later collapses survive reloads (the slice tracks a `seen` set).
        slice.set_expand_new_nodes(true);
        // The row source — the only real/mock seam (see the `rows` modules).
        slice.set_source(move || rows::load(&ctx, &work_id));
        // Domain policy: binders can't be dragged, items can.
        slice.set_drag_policy(|key| match key {
            BinderTreeKey::Binder(_) => DragEligibility::NoDrag,
            BinderTreeKey::Item(_) => DragEligibility::CanDrag,
        });
        // Domain policy: a drop onto a binder → into it; `Into` a leaf item →
        // `After` it. The slice's cycle guard (self / descendant) runs first.
        slice.set_drop_resolver(|_dragged, target, target_item, position| match target {
            BinderTreeKey::Binder(_) => Some(DropPosition::Into),
            BinderTreeKey::Item(_) => match position {
                DropPosition::Into if target_item.kind != "folder" => Some(DropPosition::After),
                p => Some(p),
            },
        });
        slice.reload();
        Self { slice }
    }

    /// Inject the reorder command (`dragged, target, position -> applied`). On a
    /// successful move the slice re-sources itself.
    pub fn set_reorder(&self, commit: CommitMove) {
        self.slice
            .set_reorder(move |dragged, target, place| commit(dragged, target, place));
    }

    /// True when `key` still exists in the tree — used by the view-model to
    /// prune a stale selection after a reload.
    pub fn contains(&self, key: &BinderTreeKey) -> bool {
        self.slice.contains_key(key)
    }

    /// Resolve a key to `(item_id, title)` (binder rows have `item_id == None`).
    pub fn node_of(&self, key: &BinderTreeKey) -> Option<(Option<u64>, String)> {
        self.slice.with_key(key, |n| (n.item_id, n.title.clone()))
    }

    /// The owning binder id for any key.
    pub fn binder_of(&self, key: &BinderTreeKey) -> Option<u64> {
        match key {
            BinderTreeKey::Binder(id) => Some(*id),
            BinderTreeKey::Item(_) => self.slice.with_key(key, |n| n.binder_id).flatten(),
        }
    }

    /// Re-source the rows for the open Work (real: from the backend / mock:
    /// static) and reproject. The data seam is the only real/mock difference.
    pub fn reload(&self) {
        self.slice.reload();
    }
}

/// Straight delegation onto the backing [`TreeDataSlice`] — all tree behaviour
/// (flatten / expand / divergence / DnD) lives there.
impl TreeDataSource for BinderBinderItemsTreeModel {
    type Item = TreeNode;
    type Key = BinderTreeKey;

    fn visible_count(&self) -> usize {
        self.slice.visible_count()
    }

    fn with_entry<R>(
        &self,
        flat_index: usize,
        f: impl FnOnce(&Self::Item, &FlatEntry<Self::Key>) -> R,
    ) -> Option<R> {
        self.slice.with_entry(flat_index, f)
    }

    fn key_at(&self, flat_index: usize) -> Option<BinderTreeKey> {
        self.slice.key_at(flat_index)
    }

    fn flat_index_of(&self, key: &BinderTreeKey) -> Option<usize> {
        self.slice.flat_index_of(key)
    }

    fn parent(&self, key: &BinderTreeKey) -> Option<BinderTreeKey> {
        self.slice.parent_of(key)
    }

    fn child_keys(&self, key: &BinderTreeKey) -> Vec<BinderTreeKey> {
        self.slice.child_keys_of(key)
    }

    fn version_signal(&self) -> Signal<u64> {
        self.slice.version_signal()
    }

    fn first_changed_index(&self) -> Option<usize> {
        self.slice.first_changed_index()
    }

    fn contains_key(&self, key: &BinderTreeKey) -> bool {
        self.slice.contains_key(key)
    }

    fn is_expanded(&self, key: &BinderTreeKey) -> bool {
        self.slice.is_expanded(key)
    }

    fn set_expanded(&self, key: &BinderTreeKey, expanded: bool) {
        self.slice.set_expanded(key, expanded);
    }

    fn drag(&self, key: &BinderTreeKey) -> DragEligibility {
        self.slice.drag(key)
    }

    fn can_accept(&self, query: &DropQuery<'_, BinderTreeKey>) -> DropResponse {
        self.slice.can_accept(query)
    }

    fn accept_drop(&self, commit: DropCommit<'_, BinderTreeKey>) -> bool {
        self.slice.accept_drop(commit)
    }
}

// ── The row-source seam: the only real/mock difference ──────────────────────

/// Pull the binder/item rows for the open `Work` from the backend as an
/// indent-ordered stream (the slice derives the tree from `depth`). Trashed
/// binders/items are omitted. Returns empty when no project is open.
#[cfg(not(feature = "mocks"))]
mod rows {
    use bastyde::data::TreeRow;
    use bastyde::prelude::Signal;

    use frontend::AppContext;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::BinderItemRole;

    use super::{BinderTreeKey, TreeNode};

    pub fn load(
        ctx: &AppContext,
        work_id: &Signal<Option<u64>>,
    ) -> Vec<TreeRow<BinderTreeKey, TreeNode>> {
        let mut rows: Vec<TreeRow<BinderTreeKey, TreeNode>> = Vec::new();
        let Some(work_id) = work_id.get() else {
            return rows; // no project open
        };
        let binder_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();
        for binder_id in binder_ids {
            let Ok(Some(binder)) = binder_commands::get_binder(ctx, &binder_id) else {
                continue;
            };
            if !binder.activated {
                continue; // trashed binders are hidden
            }
            rows.push(TreeRow::new(
                BinderTreeKey::Binder(binder_id),
                TreeNode::binder(binder.name, binder_id),
                0,
            ));

            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                &binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            let items =
                binder_item_commands::get_binder_item_multi(ctx, &item_ids).unwrap_or_default();

            // The slice derives each item's parent from its indent depth (nearest
            // preceding row of strictly smaller depth); binders are depth 0.
            for it in items.into_iter().flatten() {
                if !it.activated {
                    continue; // trashed items (and trashed subtrees) are hidden
                }
                let kind = match it.role {
                    BinderItemRole::Folder => "folder",
                    BinderItemRole::Item => "item",
                }
                .to_string();
                rows.push(TreeRow::new(
                    BinderTreeKey::Item(it.id),
                    TreeNode {
                        title: it.title,
                        label: it.label,
                        kind,
                        sub_role: it.sub_role,
                        item_id: Some(it.id),
                        binder_id: Some(binder_id),
                    },
                    (it.indent.max(0) as usize) + 1,
                ));
            }
        }
        rows
    }
}

/// The static mock tree (no backend). `work_id` is ignored.
#[cfg(feature = "mocks")]
mod rows {
    use bastyde::data::TreeRow;
    use bastyde::prelude::Signal;

    use frontend::AppContext;
    use frontend::common::entities::BinderItemSubRole;

    use super::{BinderTreeKey, TreeNode};

    fn item(
        id: u64,
        binder: u64,
        title: &str,
        label: &str,
        kind: &str,
        sub_role: BinderItemSubRole,
        depth: usize,
    ) -> TreeRow<BinderTreeKey, TreeNode> {
        TreeRow::new(
            BinderTreeKey::Item(id),
            TreeNode {
                title: title.to_string(),
                label: label.to_string(),
                kind: kind.to_string(),
                sub_role,
                item_id: Some(id),
                binder_id: Some(binder),
            },
            depth,
        )
    }

    // A tiny coherent book, arranged to exercise the full range of sub_role
    // icons: binder / book / book-begin / scene / chapter / chapter-scene /
    // note / text. (Structure kept stable — the model tests below assert the
    // row/child counts.)
    pub fn load(
        _ctx: &AppContext,
        _work_id: &Signal<Option<u64>>,
    ) -> Vec<TreeRow<BinderTreeKey, TreeNode>> {
        use BinderItemSubRole::{Book, BookBegin, Chapter, ChapterScene, Note, Scene, Text};
        vec![
            TreeRow::new(
                BinderTreeKey::Binder(1),
                TreeNode::binder("Manuscript".into(), 1),
                0,
            ),
            item(101, 1, "Book One", "the setup", "folder", Book, 1),
            item(102, 1, "Opening", "1st plot point", "item", BookBegin, 2),
            item(103, 1, "Scene at dawn", "", "item", Scene, 2),
            item(104, 1, "Chapter Two", "rising action", "folder", Chapter, 1),
            item(105, 1, "Confrontation", "", "item", ChapterScene, 2),
            TreeRow::new(
                BinderTreeKey::Binder(2),
                TreeNode::binder("Notes".into(), 2),
                0,
            ),
            item(106, 2, "Character sketch", "wants freedom", "item", Note, 1),
            item(107, 2, "Random idea", "", "item", Text, 1),
        ]
    }
}

#[cfg(all(test, feature = "mocks"))]
mod tests {
    use super::*;
    use bastyde::data::DragSource;
    use frontend::AppContext;

    fn model() -> BinderBinderItemsTreeModel {
        BinderBinderItemsTreeModel::new(Rc::new(AppContext::new()), Signal::new(None))
    }

    #[test]
    fn fully_expanded_shows_every_row() {
        let m = model();
        // 2 binders + 7 items, all auto-expanded.
        assert_eq!(m.visible_count(), 9);
    }

    #[test]
    fn collapsing_a_folder_hides_its_subtree() {
        let m = model();
        m.set_expanded(&BinderTreeKey::Item(101), false); // "Book One" (2 children)
        assert_eq!(m.visible_count(), 7);
        m.set_expanded(&BinderTreeKey::Item(101), true);
        assert_eq!(m.visible_count(), 9);
    }

    #[test]
    fn binders_cannot_drag_items_can() {
        let m = model();
        assert_eq!(m.drag(&BinderTreeKey::Binder(1)), DragEligibility::NoDrag);
        assert_eq!(m.drag(&BinderTreeKey::Item(102)), DragEligibility::CanDrag);
    }

    #[test]
    fn same_view_sibling_drop_is_accepted() {
        let m = model();
        let q = DropQuery {
            source: DragSource::SameView {
                key: BinderTreeKey::Item(102),
            },
            target: BinderTreeKey::Item(106),
            position: DropPosition::Before,
        };
        assert_eq!(m.can_accept(&q), DropResponse::Accept);
    }

    #[test]
    fn into_a_leaf_redirects_to_after() {
        let m = model();
        let q = DropQuery {
            source: DragSource::SameView {
                key: BinderTreeKey::Item(102),
            },
            target: BinderTreeKey::Item(103), // a leaf item
            position: DropPosition::Into,
        };
        assert_eq!(
            m.can_accept(&q),
            DropResponse::Redirect(DropPosition::After)
        );
    }

    #[test]
    fn into_own_subtree_is_rejected() {
        let m = model();
        // Drag the "Book One" folder onto its own child → cycle.
        let q = DropQuery {
            source: DragSource::SameView {
                key: BinderTreeKey::Item(101),
            },
            target: BinderTreeKey::Item(102),
            position: DropPosition::Into,
        };
        assert_eq!(m.can_accept(&q), DropResponse::Reject);
    }

    #[test]
    fn node_of_and_binder_of_resolve_keys() {
        let m = model();
        // node_of: item → (Some(id), title); binder → (None, name).
        assert_eq!(
            m.node_of(&BinderTreeKey::Item(102)),
            Some((Some(102), "Opening".to_string()))
        );
        assert_eq!(
            m.node_of(&BinderTreeKey::Binder(1)),
            Some((None, "Manuscript".to_string()))
        );
        // binder_of resolves an item to its owning binder, and a binder to itself.
        assert_eq!(m.binder_of(&BinderTreeKey::Item(105)), Some(1));
        assert_eq!(m.binder_of(&BinderTreeKey::Binder(2)), Some(2));
    }

    #[test]
    fn contains_tracks_membership() {
        let m = model();
        assert!(m.contains(&BinderTreeKey::Item(101)));
        assert!(!m.contains(&BinderTreeKey::Item(999)));
    }
}
