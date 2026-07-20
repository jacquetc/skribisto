// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

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

use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::Rc;

use bastyde::core::ObserverHandle;
use bastyde::data::{
    DragEligibility, DropCommit, DropPosition, DropQuery, DropResponse, FlatEntry, TreeDataSlice,
    TreeDataSource, TreeFilterMode, TreeRowFilter,
};
use bastyde::prelude::BuildContext;
use bastyde::prelude::Signal;
use frontend::common::event::{
    BinderItemManagementEvent, DirectAccessEntity, EntityEvent, Event, Origin, TrashManagementEvent,
};

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

/// The reactive filter inputs that shape which rows the tree shows. Owned by
/// [`OutlineViewModel`](crate::view_models::OutlineViewModel) and shared (by
/// signal clone) into the tree model, which reads them in its row source and
/// re-sources itself when any changes.
#[derive(Clone)]
pub struct TreeFilters {
    /// Display scope: `None` = all binders (default); `Some(id)` = that binder
    /// only (its root + items). Driven by the binder switcher.
    pub binder: Signal<Option<u64>>,
    /// Live text filter; empty = no text filtering. Driven by the search field.
    pub query: Signal<String>,
    /// Search scope: `false` = current binder, `true` = all binders. Only
    /// meaningful while a query is active.
    pub all_binders: Signal<bool>,
}

#[derive(Clone)]
pub struct BinderBinderItemsTreeModel {
    slice: TreeDataSlice<BinderTreeKey, TreeNode>,
    /// **The authoritative expand set**, held outside the slice.
    ///
    /// `TreeDataSlice::build` drops every expand key absent from the incoming rows. That
    /// is right for the slice, but this tree re-sources with a *narrower* row set in two
    /// routine situations — a binder-scope switch, and every keystroke of a search — so
    /// the slice's own set forgets state constantly:
    ///
    /// * switch to another binder and back, and the first binder returns fully collapsed
    ///   (its keys were pruned while it was off-screen);
    /// * expand a chapter, search for something that filters it out, clear the box, and
    ///   the chapter comes back collapsed.
    ///
    /// The first used to be papered over with `expand_all()` on every scope change, which
    /// traded a forgotten tree for a *destroyed* one — every collapse the writer had made
    /// was discarded. The second used to be papered over by snapshotting on search entry
    /// and restoring on exit, which discarded any collapse made *during* the search.
    ///
    /// Both are the same bug, so both get the same fix: the slice's set is a projection of
    /// this one. Toggles write here; every reload re-applies it.
    remembered: Rc<RefCell<HashSet<BinderTreeKey>>>,
    /// Keeps the filter-signal observers alive for the model's lifetime — an
    /// `ObserverHandle` unsubscribes on drop. Shared across clones so the last
    /// clone standing owns them.
    _filters: Rc<Vec<ObserverHandle>>,
    /// One-shot guard so `wire` subscribes to backend events only once.
    subscribed: Rc<Cell<bool>>,
}

impl BinderBinderItemsTreeModel {
    pub fn new(ctx: Rc<AppContext>, work_id: Signal<Option<u64>>, filters: TreeFilters) -> Self {
        let slice = TreeDataSlice::new();
        // New nodes (e.g. a freshly-created scene) appear expanded; the user's
        // later collapses survive reloads (the slice tracks a `seen` set).
        slice.set_expand_new_nodes(true);
        // The row source: binder-scoped rows from the backend (the real/mock
        // seam, see the `rows` modules), then — when a query is active — a live
        // text filter. `KeepAncestors` keeps a match's parent binder/folders so
        // the match stays reachable; the reveal override (wired below) shows them.
        {
            let ctx = ctx.clone();
            let work_id = work_id.clone();
            let f = filters.clone();
            slice.set_source(move || {
                let q = f.query.get();
                let searching = !q.trim().is_empty();
                // An all-binders search broadens the view past the switcher's
                // display scope so cross-binder matches show.
                let scope = if searching && f.all_binders.get() {
                    None
                } else {
                    f.binder.get()
                };
                let rows = rows::load(&ctx, &work_id, scope);
                if !searching {
                    return rows;
                }
                let needle = q.to_lowercase();
                TreeRowFilter::new()
                    .filter_mode(TreeFilterMode::KeepAncestors)
                    .filter(move |n: &TreeNode| {
                        // Match items only; binder rows survive as ancestors.
                        n.kind != "binder"
                            && (n.title.to_lowercase().contains(&needle)
                                || n.label.to_lowercase().contains(&needle))
                    })
                    .apply(rows)
            });
        }
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
        let remembered: Rc<RefCell<HashSet<BinderTreeKey>>> = Rc::new(RefCell::new(HashSet::new()));
        reload_preserving(&slice, &remembered);

        // Live re-source on any filter change.
        //
        // The reveal override (`set_all_expanded`) tracks the SEARCH query ONLY — a
        // binder-scoped view is a normal, collapsible tree, and forcing the override
        // there would make its chevrons dead (the toggle flips the per-row state but the
        // override keeps every row shown).
        //
        // Everything else is handled by `remembered` + `reload_preserving`: a scope
        // switch, a search that filters rows out, and a search cleared again all
        // re-source with a different row set, and the restore puts back whatever that
        // re-source pruned. No snapshot, and emphatically no `expand_all()`.
        //
        // Cycle-safe (cf. `install_reorder`): the closure captures the slice + signals +
        // the remembered set, never `self`. The observer handles (in `_filters`) own it.
        let resource: Rc<dyn Fn()> = {
            let slice = slice.clone();
            let f = filters.clone();
            let remembered = remembered.clone();
            Rc::new(move || {
                slice.set_all_expanded(!f.query.get().trim().is_empty());
                reload_preserving(&slice, &remembered);
            })
        };
        let observers = vec![
            {
                let r = resource.clone();
                filters.query.observe(move |_| r())
            },
            {
                let r = resource.clone();
                filters.binder.observe(move |_| r())
            },
            {
                let r = resource.clone();
                filters.all_binders.observe(move |_| r())
            },
        ];

        Self {
            slice,
            remembered,
            _filters: Rc::new(observers),
            subscribed: Rc::new(Cell::new(false)),
        }
    }

    /// Subscribe once so the tree **re-sources on any structural backend change**
    /// — regardless of who caused it (the outline's own commands, the Full Chapter
    /// view, import, another process). Without this the tree only refreshed after
    /// `OutlineViewModel`'s own mutations. Call from a long-lived widget's `build`.
    pub fn wire(&self, ctx: &mut BuildContext) {
        if self.subscribed.replace(true) {
            return;
        }
        use DirectAccessEntity::{Binder, BinderItem};
        use EntityEvent::{Created, Removed, Updated};
        let origins = [
            Origin::DirectAccess(BinderItem(Created)),
            Origin::DirectAccess(BinderItem(Updated)),
            Origin::DirectAccess(BinderItem(Removed)),
            Origin::DirectAccess(Binder(Created)),
            Origin::DirectAccess(Binder(Updated)),
            Origin::DirectAccess(Binder(Removed)),
            Origin::BinderItemManagement(BinderItemManagementEvent::Duplicate),
            Origin::BinderItemManagement(BinderItemManagementEvent::MoveItems),
            Origin::BinderItemManagement(BinderItemManagementEvent::MergeTwoScenes),
            Origin::BinderItemManagement(BinderItemManagementEvent::SplitScene),
            Origin::TrashManagement(TrashManagementEvent::TrashBinderItems),
            Origin::TrashManagement(TrashManagementEvent::TrashBinder),
            Origin::TrashManagement(TrashManagementEvent::RestoreItems),
            Origin::TrashManagement(TrashManagementEvent::EmptyTrash),
        ];
        for origin in origins {
            let me = self.clone();
            ctx.subscribe_event(origin, move |_e: &Event| me.reload());
        }
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

    /// True when `key` is a folder-kind item row (drop-target resolution: a drop
    /// lands *into* a folder, *after* a leaf).
    pub fn node_is_folder(&self, key: &BinderTreeKey) -> bool {
        self.slice
            .with_key(key, |n| n.kind == "folder")
            .unwrap_or(false)
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
        reload_preserving(&self.slice, &self.remembered);
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
        // Mirror every toggle into the authoritative set — this is the write that makes
        // the slice's own set a projection rather than the truth.
        if expanded {
            self.remembered.borrow_mut().insert(*key);
        } else {
            self.remembered.borrow_mut().remove(key);
        }
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

/// Re-source, then restore the expand state the re-source pruned.
///
/// The union with the slice's own post-reload set is what keeps
/// `set_expand_new_nodes(true)` working: a genuinely new row is not in `remembered`, but
/// the slice has just auto-expanded it, and `set_expanded_keys` **replaces** rather than
/// merges — so restoring `remembered` alone would collapse every newly created scene.
/// Folding the union back in is what makes that auto-expansion stick, and is also what
/// expands a binder the writer is visiting for the first time (all of its rows are new).
fn reload_preserving(
    slice: &TreeDataSlice<BinderTreeKey, TreeNode>,
    remembered: &RefCell<HashSet<BinderTreeKey>>,
) {
    slice.reload();
    let mut union = remembered.borrow().clone();
    union.extend(slice.expanded_keys());
    let keys: Vec<BinderTreeKey> = union.iter().copied().collect();
    slice.set_expanded_keys(&keys);
    *remembered.borrow_mut() = union;
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
        scope: Option<u64>,
    ) -> Vec<TreeRow<BinderTreeKey, TreeNode>> {
        let mut rows: Vec<TreeRow<BinderTreeKey, TreeNode>> = Vec::new();
        let Some(work_id) = work_id.get() else {
            return rows; // no project open
        };
        let binder_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();
        for binder_id in binder_ids {
            // Display scope: `Some(id)` shows only that binder (the switcher's
            // current binder); `None` shows every binder.
            if scope.is_some_and(|only| only != binder_id) {
                continue;
            }
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
        scope: Option<u64>,
    ) -> Vec<TreeRow<BinderTreeKey, TreeNode>> {
        use BinderItemSubRole::{Book, BookBegin, ChapterScene, Note, Part, Scene, Text};
        let rows = vec![
            TreeRow::new(
                BinderTreeKey::Binder(1),
                TreeNode::binder("Manuscript".into(), 1),
                0,
            ),
            item(101, 1, "Book One", "the setup", "folder", Book, 1),
            item(102, 1, "Opening", "1st plot point", "item", BookBegin, 2),
            item(103, 1, "Scene at dawn", "", "item", Scene, 2),
            // A part holding both chapter encodings, so every container opens onto a
            // non-trivial stream. Kept in step with `models::StreamRowsModel`'s mock rows
            // and `singles::SingleBinderItem`'s `mock_dto`.
            item(301, 1, "Part One — Arrival", "", "folder", Part, 2),
            item(
                104,
                1,
                "Chapter Two",
                "rising action",
                "folder",
                ChapterScene,
                3,
            ),
            item(201, 1, "Scene 1", "opening beat", "item", Scene, 4),
            item(202, 1, "Scene 2", "", "item", Scene, 4),
            item(203, 1, "Scene 3", "", "item", Scene, 4),
            item(302, 1, "Into the Dark", "", "item", ChapterScene, 3),
            item(303, 1, "The light returns", "", "item", Scene, 3),
            item(105, 1, "Confrontation", "", "item", ChapterScene, 2),
            TreeRow::new(
                BinderTreeKey::Binder(2),
                TreeNode::binder("Notes".into(), 2),
                0,
            ),
            item(106, 2, "Character sketch", "wants freedom", "item", Note, 1),
            item(107, 2, "Random idea", "", "item", Text, 1),
        ];
        // Display scope: keep only the requested binder's rows (its binder row
        // and items both carry `binder_id`); `None` keeps everything.
        match scope {
            None => rows,
            Some(only) => rows
                .into_iter()
                .filter(|r| r.item.binder_id == Some(only))
                .collect(),
        }
    }
}

#[cfg(all(test, feature = "mocks"))]
mod tests {
    use super::*;
    use bastyde::data::DragSource;
    use frontend::AppContext;

    fn default_filters() -> TreeFilters {
        TreeFilters {
            binder: Signal::new(None),
            query: Signal::new(String::new()),
            all_binders: Signal::new(false),
        }
    }

    /// Build a model plus a handle to its filter signals (so tests can drive the
    /// binder scope / text query and observe the re-source).
    fn model_with_filters() -> (BinderBinderItemsTreeModel, TreeFilters) {
        let filters = default_filters();
        let m = BinderBinderItemsTreeModel::new(
            Rc::new(AppContext::new()),
            Signal::new(None),
            filters.clone(),
        );
        (m, filters)
    }

    fn model() -> BinderBinderItemsTreeModel {
        model_with_filters().0
    }

    #[test]
    fn fully_expanded_shows_every_row() {
        let m = model();
        // 2 binders + 13 items, all auto-expanded.
        assert_eq!(m.visible_count(), 15);
    }

    #[test]
    fn filter_to_one_binder_hides_others() {
        let (m, f) = model_with_filters();
        assert_eq!(m.visible_count(), 15); // all binders
        // Manuscript = 1 binder row + 11 items.
        f.binder.set(Some(1));
        assert_eq!(m.visible_count(), 12);
        // Notes = 1 binder row + 2 items.
        f.binder.set(Some(2));
        assert_eq!(m.visible_count(), 3);
        f.binder.set(None);
        assert_eq!(m.visible_count(), 15);
    }

    #[test]
    fn search_keeps_ancestors_of_matches() {
        let (m, f) = model_with_filters();
        // "dawn" matches only item 103 "Scene at dawn" (Manuscript > Book One >
        // Scene at dawn). KeepAncestors retains its two ancestors; the reveal
        // override shows them even though Book One would otherwise be collapsible.
        f.query.set("dawn".to_string());
        assert_eq!(m.visible_count(), 3);
        // Binder rows never match by name (items only), so a non-matching term
        // clears the tree entirely.
        f.query.set("zzz-nothing".to_string());
        assert_eq!(m.visible_count(), 0);
        // Clearing restores the full tree (and the persistent expand state).
        f.query.set(String::new());
        assert_eq!(m.visible_count(), 15);
    }

    #[test]
    fn single_binder_view_allows_collapse() {
        // Regression: a selected binder must be a *normal* collapsible tree — the
        // reveal override (set_all_expanded) is for search only. Forcing it here
        // made the expand/collapse chevrons dead (toggle flips per-row state but
        // the override keeps every row shown).
        let (m, f) = model_with_filters();
        f.binder.set(Some(1)); // Manuscript: binder + 11 items, expanded → 12
        assert_eq!(m.visible_count(), 12);
        // Collapse "Book One" — the whole book is its subtree (10 rows).
        m.set_expanded(&BinderTreeKey::Item(101), false);
        assert_eq!(m.visible_count(), 2);
        m.set_expanded(&BinderTreeKey::Item(101), true);
        assert_eq!(m.visible_count(), 12);
    }

    /// **Switching binder scope and back must preserve the writer's collapses.**
    ///
    /// `TreeDataSlice::build` prunes expand keys absent from the incoming rows, so
    /// leaving a binder used to forget everything about it; the old workaround was
    /// `expand_all()` on every scope change, which did not merely forget the state — it
    /// destroyed it, throwing away every collapse the writer had made.
    #[test]
    fn a_collapse_survives_leaving_a_binder_and_coming_back() {
        let (m, f) = model_with_filters();
        f.binder.set(Some(1)); // Manuscript: binder + 11 items
        assert_eq!(m.visible_count(), 12);

        m.set_expanded(&BinderTreeKey::Item(101), false); // collapse "Book One"
        assert_eq!(m.visible_count(), 2);

        f.binder.set(Some(2)); // away to Notes …
        f.binder.set(Some(1)); // … and back

        assert_eq!(
            m.visible_count(),
            2,
            "Book One must still be collapsed; `expand_all()` used to blow this open"
        );
        assert!(!m.is_expanded(&BinderTreeKey::Item(101)));
    }

    /// …and a binder visited for the **first** time still opens expanded, which is what
    /// `set_expand_new_nodes(true)` is for. The restore must not defeat it.
    #[test]
    fn a_binder_visited_for_the_first_time_opens_expanded() {
        let (m, f) = model_with_filters();
        f.binder.set(Some(1));
        assert_eq!(m.visible_count(), 12, "every row of Manuscript is showing");
    }

    /// **An expanded row that a search filters out must come back expanded.**
    ///
    /// A search narrows the row set, so `build` prunes it. This used to be papered over
    /// by snapshotting on entry and restoring on exit — which then discarded any collapse
    /// made *during* the search (pinned by the test below).
    #[test]
    fn an_expanded_row_filtered_out_by_a_search_comes_back_expanded() {
        let (m, f) = model_with_filters();
        f.binder.set(Some(1));
        assert!(m.is_expanded(&BinderTreeKey::Item(101)), "Book One starts open");

        // "Random idea" lives in the Notes binder, so nothing under Book One matches and
        // the whole subtree is filtered away.
        f.query.set("Random idea".to_string());
        f.query.set(String::new());

        assert!(
            m.is_expanded(&BinderTreeKey::Item(101)),
            "Book One was pruned by the search's row filter and never restored"
        );
    }

    /// A collapse made **while a search is active** must survive clearing it — the
    /// failure mode of the snapshot-and-restore this replaced.
    #[test]
    fn a_collapse_made_during_a_search_survives_clearing_it() {
        let (m, f) = model_with_filters();
        f.binder.set(Some(1));
        f.query.set("Scene".to_string());
        m.set_expanded(&BinderTreeKey::Item(101), false);
        f.query.set(String::new());
        assert!(
            !m.is_expanded(&BinderTreeKey::Item(101)),
            "clearing the search resurrected a collapse the writer made during it"
        );
    }

    #[test]
    fn collapsing_a_folder_hides_its_subtree() {
        let m = model();
        m.set_expanded(&BinderTreeKey::Item(101), false); // "Book One" (10 descendants)
        assert_eq!(m.visible_count(), 5);
        m.set_expanded(&BinderTreeKey::Item(101), true);
        assert_eq!(m.visible_count(), 15);
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
