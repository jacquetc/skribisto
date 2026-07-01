//! Reactive model for the binder-item tree, implemented as a Bastyde
//! [`TreeDataSource`] so a `TreeView` can render, navigate, expand and
//! **drag-reorder** it. The flat item stream nests via each item's `indent`;
//! binders are the tree roots.
//!
//! Self-contained reference shape for a future Qleany tera template. The public
//! type and the whole `TreeDataSource` algorithm (visibility flattening,
//! drag-drop resolution, projection) are written **once**; the only part that
//! differs real-vs-mock is the *row source*, isolated behind two `#[cfg]`-gated
//! [`rows`] modules exposing an identical `load(ctx, work_id) -> Vec<Row>`. So no
//! `#[cfg]` leaks into consuming code and there is no duplicated tree logic to
//! drift. (The small `singles/` handles use the fuller two-`mod imp` shape; this
//! large shared-algorithm model gates only the seam — see the convention note in
//! `models.rs`.) Parity is enforced by building both feature modes.
//!
//! Rows are sourced for the **open Work only**, identified by the `work_id`
//! signal from [`AppIds`](crate::app_ids) (ids-only global state) — no
//! `get_all_work`. Mutations are not applied here: drops route through an
//! injected [`CommitMove`] closure (`set_reorder`) and the model re-reads itself.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use bastyde::data::{
    DragEligibility, DragSource, DropCommit, DropPosition, DropQuery, DropResponse, FlatEntry,
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

/// One node in the navigation tree. Shared by both variants.
#[derive(Clone, Debug, Default)]
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

struct Row {
    key: BinderTreeKey,
    node: TreeNode,
    depth: usize,
    parent: Option<BinderTreeKey>,
    has_children: bool,
}

struct Inner {
    rows: RefCell<Vec<Row>>,
    /// Indices into `rows` that are currently visible (collapse-aware flatten).
    visible: RefCell<Vec<usize>>,
    /// key → position within `visible` (the flat index the view sees).
    vis_pos: RefCell<HashMap<BinderTreeKey, usize>>,
    /// key → index within `rows`.
    row_pos: RefCell<HashMap<BinderTreeKey, usize>>,
    expanded: RefCell<HashSet<BinderTreeKey>>,
    /// Keys seen at least once — used to auto-expand newly-appearing nodes while
    /// preserving the user's later collapses across reloads.
    seen: RefCell<HashSet<BinderTreeKey>>,
    version: Signal<u64>,
    commit_move: RefCell<Option<CommitMove>>,
    /// The open Work whose tree is shown (ids-only global state). Real `reload`
    /// sources from it; the mock seam ignores it.
    work_id: Signal<Option<u64>>,
    ctx: Rc<AppContext>,
}

#[derive(Clone)]
pub struct BinderBinderItemsTreeModel {
    inner: Rc<Inner>,
}

impl BinderBinderItemsTreeModel {
    pub fn new(ctx: Rc<AppContext>, work_id: Signal<Option<u64>>) -> Self {
        let model = Self {
            inner: Rc::new(Inner {
                rows: RefCell::new(Vec::new()),
                visible: RefCell::new(Vec::new()),
                vis_pos: RefCell::new(HashMap::new()),
                row_pos: RefCell::new(HashMap::new()),
                expanded: RefCell::new(HashSet::new()),
                seen: RefCell::new(HashSet::new()),
                version: Signal::new(0),
                commit_move: RefCell::new(None),
                work_id,
                ctx,
            }),
        };
        model.reload();
        model
    }

    /// Inject the reorder command (`dragged, target, position -> applied`).
    pub fn set_reorder(&self, commit: CommitMove) {
        *self.inner.commit_move.borrow_mut() = Some(commit);
    }

    /// True when `key` still exists in the tree — used by the view-model to
    /// prune a stale selection after a reload.
    pub fn contains(&self, key: &BinderTreeKey) -> bool {
        self.inner.row_pos.borrow().contains_key(key)
    }

    /// Resolve a key to `(item_id, title)` (binder rows have `item_id == None`).
    pub fn node_of(&self, key: &BinderTreeKey) -> Option<(Option<u64>, String)> {
        let rows = self.inner.rows.borrow();
        let idx = *self.inner.row_pos.borrow().get(key)?;
        rows.get(idx)
            .map(|r| (r.node.item_id, r.node.title.clone()))
    }

    /// The owning binder id for any key.
    pub fn binder_of(&self, key: &BinderTreeKey) -> Option<u64> {
        match key {
            BinderTreeKey::Binder(id) => Some(*id),
            BinderTreeKey::Item(_) => {
                let rows = self.inner.rows.borrow();
                let idx = *self.inner.row_pos.borrow().get(key)?;
                rows.get(idx).and_then(|r| r.node.binder_id)
            }
        }
    }

    /// Re-source the rows for the open Work (real: from the backend / mock:
    /// static) and reproject. The data seam is the only real/mock difference.
    pub fn reload(&self) {
        let rows = rows::load(&self.inner.ctx, &self.inner.work_id);
        self.install_rows(rows);
    }

    /// Finalise a freshly-built `rows`: derive `has_children`, auto-expand
    /// newly-seen nodes, rebuild the visible projection, bump the version.
    fn install_rows(&self, mut rows: Vec<Row>) {
        let parents: HashSet<BinderTreeKey> = rows.iter().filter_map(|r| r.parent).collect();
        for r in rows.iter_mut() {
            r.has_children = parents.contains(&r.key);
        }

        {
            let mut row_pos = self.inner.row_pos.borrow_mut();
            row_pos.clear();
            for (i, r) in rows.iter().enumerate() {
                row_pos.insert(r.key, i);
            }
        }
        {
            let mut expanded = self.inner.expanded.borrow_mut();
            let mut seen = self.inner.seen.borrow_mut();
            for r in &rows {
                if !seen.contains(&r.key) {
                    seen.insert(r.key);
                    expanded.insert(r.key); // new nodes start expanded
                }
            }
        }

        *self.inner.rows.borrow_mut() = rows;
        self.rebuild_visible();
        self.bump();
    }

    fn rebuild_visible(&self) {
        let rows = self.inner.rows.borrow();
        let expanded = self.inner.expanded.borrow();
        let mut visible = Vec::with_capacity(rows.len());
        let mut vis_pos = HashMap::with_capacity(rows.len());
        let mut collapse_depth: Option<usize> = None;
        for (i, row) in rows.iter().enumerate() {
            if let Some(cd) = collapse_depth {
                if row.depth > cd {
                    continue; // hidden under a collapsed ancestor
                }
                collapse_depth = None;
            }
            vis_pos.insert(row.key, visible.len());
            visible.push(i);
            if row.has_children && !expanded.contains(&row.key) {
                collapse_depth = Some(row.depth);
            }
        }
        *self.inner.visible.borrow_mut() = visible;
        *self.inner.vis_pos.borrow_mut() = vis_pos;
    }

    fn bump(&self) {
        let v = self.inner.version.get().wrapping_add(1);
        self.inner.version.set(v);
    }

    fn is_folder(&self, key: &BinderTreeKey) -> bool {
        let rows = self.inner.rows.borrow();
        self.inner
            .row_pos
            .borrow()
            .get(key)
            .and_then(|&i| rows.get(i))
            .map(|r| r.node.kind == "folder")
            .unwrap_or(false)
    }

    /// Is `maybe_descendant` inside the subtree rooted at `ancestor`?
    fn is_descendant(&self, maybe_descendant: BinderTreeKey, ancestor: BinderTreeKey) -> bool {
        let rows = self.inner.rows.borrow();
        let row_pos = self.inner.row_pos.borrow();
        let mut cur = maybe_descendant;
        // Walk up the parent chain.
        for _ in 0..rows.len() {
            let Some(&idx) = row_pos.get(&cur) else {
                return false;
            };
            let Some(parent) = rows[idx].parent else {
                return false;
            };
            if parent == ancestor {
                return true;
            }
            cur = parent;
        }
        false
    }

    /// Resolve a requested drop into its effective position, or `None` if
    /// forbidden. A drop onto a binder → into the binder; `Into` a leaf item →
    /// `After` it; self / cycle drops are rejected.
    fn resolve(
        &self,
        dragged: BinderTreeKey,
        target: BinderTreeKey,
        position: DropPosition,
    ) -> Option<DropPosition> {
        if dragged == target || self.is_descendant(target, dragged) {
            return None;
        }
        match target {
            BinderTreeKey::Binder(_) => Some(DropPosition::Into),
            BinderTreeKey::Item(_) => match position {
                DropPosition::Into if !self.is_folder(&target) => Some(DropPosition::After),
                p => Some(p),
            },
        }
    }
}

impl TreeDataSource for BinderBinderItemsTreeModel {
    type Item = TreeNode;
    type Key = BinderTreeKey;

    fn visible_count(&self) -> usize {
        self.inner.visible.borrow().len()
    }

    fn with_entry<R>(
        &self,
        flat_index: usize,
        f: impl FnOnce(&Self::Item, &FlatEntry<Self::Key>) -> R,
    ) -> Option<R> {
        let row_idx = *self.inner.visible.borrow().get(flat_index)?;
        let rows = self.inner.rows.borrow();
        let row = rows.get(row_idx)?;
        let entry = FlatEntry {
            node_id: row.key,
            depth: row.depth,
            has_children: row.has_children,
            is_expanded: self.inner.expanded.borrow().contains(&row.key),
        };
        Some(f(&row.node, &entry))
    }

    fn key_at(&self, flat_index: usize) -> Option<Self::Key> {
        let row_idx = *self.inner.visible.borrow().get(flat_index)?;
        self.inner.rows.borrow().get(row_idx).map(|r| r.key)
    }

    fn flat_index_of(&self, key: &Self::Key) -> Option<usize> {
        self.inner.vis_pos.borrow().get(key).copied()
    }

    fn parent(&self, key: &Self::Key) -> Option<Self::Key> {
        let rows = self.inner.rows.borrow();
        let idx = *self.inner.row_pos.borrow().get(key)?;
        rows.get(idx).and_then(|r| r.parent)
    }

    fn child_keys(&self, key: &Self::Key) -> Vec<Self::Key> {
        self.inner
            .rows
            .borrow()
            .iter()
            .filter(|r| r.parent == Some(*key))
            .map(|r| r.key)
            .collect()
    }

    fn version_signal(&self) -> Signal<u64> {
        self.inner.version.clone()
    }

    fn is_expanded(&self, key: &Self::Key) -> bool {
        self.inner.expanded.borrow().contains(key)
    }

    fn set_expanded(&self, key: &Self::Key, expanded: bool) {
        {
            let mut set = self.inner.expanded.borrow_mut();
            if expanded {
                set.insert(*key);
            } else {
                set.remove(key);
            }
        }
        self.rebuild_visible();
        self.bump();
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.inner.row_pos.borrow().contains_key(key)
    }

    fn drag(&self, key: &Self::Key) -> DragEligibility {
        match key {
            BinderTreeKey::Binder(_) => DragEligibility::NoDrag,
            BinderTreeKey::Item(_) => DragEligibility::CanDrag,
        }
    }

    fn can_accept(&self, query: &DropQuery<'_, Self::Key>) -> DropResponse {
        let DragSource::SameView { key: dragged } = query.source else {
            return DropResponse::Reject;
        };
        match self.resolve(dragged, query.target, query.position) {
            Some(p) if p == query.position => DropResponse::Accept,
            Some(p) => DropResponse::Redirect(p),
            None => DropResponse::Reject,
        }
    }

    fn accept_drop(&self, commit: DropCommit<'_, Self::Key>) -> bool {
        let DragSource::SameView { key: dragged } = commit.source else {
            return false;
        };
        let Some(place) = self.resolve(dragged, commit.target, commit.position) else {
            return false;
        };
        let Some(commit_move) = self.inner.commit_move.borrow().clone() else {
            return false;
        };
        if commit_move(dragged, commit.target, place) {
            self.reload();
            true
        } else {
            false
        }
    }
}

// ── The row-source seam: the only real/mock difference ──────────────────────

/// Pull the binder/item rows for the open `Work` from the backend, nesting the
/// flat item stream by `indent`. Trashed binders/items (and their subtrees) are
/// omitted. Returns empty when no project is open.
#[cfg(not(feature = "mocks"))]
mod rows {
    use bastyde::prelude::Signal;

    use frontend::AppContext;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::BinderItemRole;

    use super::{BinderTreeKey, Row, TreeNode};

    pub fn load(ctx: &AppContext, work_id: &Signal<Option<u64>>) -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::new();
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
            let bkey = BinderTreeKey::Binder(binder_id);
            rows.push(Row {
                key: bkey,
                node: TreeNode::binder(binder.name, binder_id),
                depth: 0,
                parent: None,
                has_children: false,
            });

            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                &binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            let items =
                binder_item_commands::get_binder_item_multi(ctx, &item_ids).unwrap_or_default();

            // (indent, key) stack — a row's parent is the nearest ancestor with a
            // strictly smaller indent; the binder is the (-1) base.
            let mut stack: Vec<(i64, BinderTreeKey)> = vec![(-1, bkey)];
            for it in items.into_iter().flatten() {
                if !it.activated {
                    continue; // trashed items (and trashed subtrees) are hidden
                }
                while stack.len() > 1 && stack.last().map(|(i, _)| *i).unwrap_or(-1) >= it.indent {
                    stack.pop();
                }
                let parent = stack.last().map(|(_, k)| *k).unwrap_or(bkey);
                let key = BinderTreeKey::Item(it.id);
                let kind = match it.role {
                    BinderItemRole::Folder => "folder",
                    BinderItemRole::Item => "item",
                }
                .to_string();
                rows.push(Row {
                    key,
                    node: TreeNode {
                        title: it.title,
                        label: it.label,
                        kind,
                        sub_role: it.sub_role,
                        item_id: Some(it.id),
                        binder_id: Some(binder_id),
                    },
                    depth: (it.indent.max(0) as usize) + 1,
                    parent: Some(parent),
                    has_children: false,
                });
                stack.push((it.indent, key));
            }
        }
        rows
    }
}

/// The static mock tree (no backend). `work_id` is ignored.
#[cfg(feature = "mocks")]
mod rows {
    use bastyde::prelude::Signal;

    use frontend::AppContext;
    use frontend::common::entities::BinderItemSubRole;

    use super::{BinderTreeKey, Row, TreeNode};

    #[allow(clippy::too_many_arguments)]
    fn item(
        id: u64,
        binder: u64,
        title: &str,
        label: &str,
        kind: &str,
        sub_role: BinderItemSubRole,
        depth: usize,
        parent: BinderTreeKey,
    ) -> Row {
        Row {
            key: BinderTreeKey::Item(id),
            node: TreeNode {
                title: title.to_string(),
                label: label.to_string(),
                kind: kind.to_string(),
                sub_role,
                item_id: Some(id),
                binder_id: Some(binder),
            },
            depth,
            parent: Some(parent),
            has_children: false,
        }
    }

    // A tiny coherent book, arranged to exercise the full range of sub_role
    // icons: binder / book / book-begin / scene / chapter / chapter-scene /
    // note / text. (Structure kept stable — the model tests below assert the
    // row/child counts.)
    pub fn load(_ctx: &AppContext, _work_id: &Signal<Option<u64>>) -> Vec<Row> {
        // Import specific variants (not a glob — that would pull `None` in and
        // shadow `Option::None` used for the binder rows' `parent`).
        use BinderItemSubRole::{Book, BookBegin, Chapter, ChapterScene, Note, Scene, Text};
        let m = BinderTreeKey::Binder(1);
        let n = BinderTreeKey::Binder(2);
        let book = BinderTreeKey::Item(101);
        let ch2 = BinderTreeKey::Item(104);
        vec![
            Row {
                key: m,
                node: TreeNode::binder("Manuscript".into(), 1),
                depth: 0,
                parent: None,
                has_children: false,
            },
            item(101, 1, "Book One", "the setup", "folder", Book, 1, m),
            item(102, 1, "Opening", "1st plot point", "item", BookBegin, 2, book),
            item(103, 1, "Scene at dawn", "", "item", Scene, 2, book),
            item(104, 1, "Chapter Two", "rising action", "folder", Chapter, 1, m),
            item(105, 1, "Confrontation", "", "item", ChapterScene, 2, ch2),
            Row {
                key: n,
                node: TreeNode::binder("Notes".into(), 2),
                depth: 0,
                parent: None,
                has_children: false,
            },
            item(106, 2, "Character sketch", "wants freedom", "item", Note, 1, n),
            item(107, 2, "Random idea", "", "item", Text, 1, n),
        ]
    }
}

#[cfg(all(test, feature = "mocks"))]
mod tests {
    use super::*;
    use bastyde::data::{DragSource, DropQuery};
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
        m.set_expanded(&BinderTreeKey::Item(101), false); // "Chapter 1" (2 children)
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
        // Drag the "Chapter 1" folder onto its own child → cycle.
        let q = DropQuery {
            source: DragSource::SameView {
                key: BinderTreeKey::Item(101),
            },
            target: BinderTreeKey::Item(102),
            position: DropPosition::Into,
        };
        assert_eq!(m.can_accept(&q), DropResponse::Reject);
    }
}
