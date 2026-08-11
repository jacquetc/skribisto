// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive rows behind the **Overview** table — one container's whole subtree as a
//! dense, sortable, searchable outliner (the Overview segment of a Chapter / Part / Book
//! / Note-folder tab).
//!
//! Backed by a Teksilo [`TreeDataSlice`] keyed by the row's **durable
//! [`uid`](common::uid)**, not by its store id. That choice is the whole reason `uid`
//! exists: store ids are re-minted on every load and a `.skrib`'s `file_id`s churn on
//! every save, so an expand set keyed by either would be meaningless the next time the
//! project opened. A `TreeDataSlice`'s domain-keyed expand state survives a full
//! re-source — which a `TreeModel` mirror cannot promise, since its `NodeId`s are
//! reassigned on rebuild.
//!
//! Like [`BinderBinderItemsTreeModel`](super::BinderBinderItemsTreeModel), this is a thin
//! **domain facade**: the tree algorithm (depth → hierarchy, per-view expand state,
//! collapse-aware flattening, divergence, the DnD cycle guard) lives once in
//! `teksilo_data`. What is genuinely domain-specific lives here — the subtree slice, the
//! bottom-up word fold, the search/sort shaping, the drag policy, and the row source
//! (the sole real-vs-mock seam).
//!
//! **Seam shape.** The type and the algorithm are written once and only `mod rows` is
//! `#[cfg]`-gated, the same exception `BinderBinderItemsTreeModel` takes: gating the whole
//! model would duplicate the fold and the slice, which is exactly the code most worth
//! having one copy of.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use teksilo::core::ObserverHandle;
use teksilo::data::{
    DragEligibility, DropCommit, DropPosition, DropQuery, DropResponse, FlatEntry, SortDirection,
    TreeDataSlice, TreeDataSource, TreeFilterMode, TreeRow, TreeRowFilter,
};
use teksilo::prelude::{BuildContext, Signal};
use uuid::Uuid;

use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, GoalUnit};
use frontend::common::event::{
    BinderItemManagementEvent, DirectAccessEntity, EntityEvent, Event, Origin,
    TrashManagementEvent, WorkManagementEvent,
};
use skribisto_model::counting::CountingMethodSetting;

use super::row_search;

/// Stable column ids — the persistence key for sort/width/order, and the string the
/// header, the comparator lookup and the view-model all agree on. Declared here (beside
/// the comparators they select) rather than in the view, so a renamed column cannot
/// silently stop sorting.
pub const COL_TITLE: &str = "title";
pub const COL_TYPE: &str = "type";
pub const COL_LABEL: &str = "label";
pub const COL_OWN_WORDS: &str = "own_words";
pub const COL_TOTAL_WORDS: &str = "total_words";
pub const COL_OPEN_COMMENTS: &str = "open_comments";
pub const COL_TOTAL_COMMENTS: &str = "total_comments";
pub const COL_TAGS: &str = "tags";
pub const COL_GOAL: &str = "goal";

/// One row of the Overview table: a single `BinderItem` in the container's subtree.
///
/// `PartialEq` powers the slice's divergence check, so a reload that changed only one
/// row's word count repaints only that row.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OverviewRow {
    /// The live store id — what every command takes.
    pub item_id: u64,
    /// The durable identity — what the tree is keyed by and what expand-state
    /// persistence will store. Survives save → load; `item_id` does not.
    pub uid: Uuid,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    pub title: String,
    /// The writer's own note under the title (`BinderItem.label`) — its own column, and
    /// part of the searchable text.
    pub label: String,
    /// Words of *this row's own* scene prose, or `None` when the row carries none
    /// (`skribisto_model::counts_prose`). `None` and `Some(0)` are different facts: a
    /// Part has no prose to count, an empty Scene has prose that is empty — the column
    /// renders the first blank and the second as `0`.
    pub own_words: Option<usize>,
    /// This row's own words plus every descendant's — the bottom-up fold. Correct even
    /// while the row is collapsed, which is the entire point of showing it.
    pub total_words: usize,
    /// The item's tags, by `Tag` id — rendered as the dot row every other view uses.
    ///
    /// Carried on the row rather than fetched per cell: the tag ids arrive free with the
    /// `BinderItemDto` the loader already reads, and a per-cell lookup would issue one
    /// backend read per visible row per rebuild.
    pub tags: Vec<u64>,
    /// The chapter/part ordinal this row carries in the book, or `None` for a row that
    /// holds none. Its own field, never folded into `title` — the Title column is
    /// edit-in-place, and its editor seeds from `title`.
    pub number: Option<usize>,
    /// What to call this row when it has no title of its own — "Chapter 3", localized.
    /// See `crate::models::label_and_badge` for the rule; `title` stays the writer's own
    /// string, because that is what renames seed from and what search matches.
    pub fallback_label: Option<String>,
    /// Open (unresolved, non-orphaned) comment threads anchored to *this row's own*
    /// Content rows.
    ///
    /// Open rather than total, because the number a writer acts on is "what still
    /// needs me" — a chapter of resolved threads should read as done, not as busy.
    /// The total is still one hover away in the dock.
    /// Whether this row reaches the exported book. **Per item, never inherited** — that is
    /// how `count_words_uc` and the export scope resolver both read it, which is why the
    /// Inspector ships an "Apply to children" button beside the switch.
    pub is_exportable: bool,
    /// What this row contributes to a manuscript total: its own words when it is exported,
    /// zero when it is not.
    ///
    /// Kept apart from `own_words` on purpose. The **Own words** column answers "how long
    /// is this piece", which stays true of a scene the writer has cut from the export; the
    /// **Total** column answers "how much book is in here", which does not. Folding
    /// `own_words` would have made the second column disagree with every other count in the
    /// app; hiding the first would have made a 1 200-word scene read as empty.
    pub manuscript_words: usize,
    /// This row's target in the project's unit (`0` = none).
    pub goal: i64,
    pub own_comments: usize,
    /// This row's own open threads plus every descendant's — the same bottom-up
    /// fold `total_words` uses, and correct while collapsed for the same reason.
    pub total_comments: usize,
}

/// The reactive shaping inputs, owned by
/// [`OverviewViewModel`](crate::view_models::OverviewViewModel) and shared by clone into
/// the model, which reads them in its row source and re-sources when either changes.
#[derive(Clone)]
pub struct OverviewFilters {
    /// Live text filter; empty = no filtering.
    pub query: Signal<String>,
    /// Active sort, or `None` for manuscript order.
    pub sort: Signal<Option<(String, SortDirection)>>,
}

impl OverviewFilters {
    pub fn new() -> Self {
        Self {
            query: Signal::new(String::new()),
            sort: Signal::new(None),
        }
    }
}

impl Default for OverviewFilters {
    fn default() -> Self {
        Self::new()
    }
}

/// Reorder hook injected by the view-model: `(dragged, target, position) -> applied`.
/// Applies the move through the backend (with undo) and reports whether it took. Keyed by
/// `uid`, so the closure resolves uid → `item_id` against the current rows.
pub type CommitMove = Rc<dyn Fn(Uuid, Uuid, DropPosition) -> bool>;

#[derive(Clone)]
pub struct OverviewRowsModel {
    slice: TreeDataSlice<Uuid, OverviewRow>,
    /// The `Content` ids currently in scope, refreshed by every row load. A
    /// `Content(Updated)` event outside this set belongs to some other part of the
    /// manuscript and must not cost this table a full re-source — with a split editor
    /// open on an unrelated scene, an unscoped subscription would reload the whole
    /// subtree on every autosave tick.
    scope_contents: Rc<RefCell<HashSet<u64>>>,
    /// Whether the container this table tabulates still exists in the binder. Goes false
    /// when it is trashed out from under an open tab, so the empty state can say so
    /// instead of offering a "＋ New" that would silently fail.
    container_present: Signal<bool>,
    /// uid → live store id for the rows currently loaded, refreshed on every source run.
    ///
    /// Exists so the drag-reorder closure can resolve a key without holding **this
    /// model**. `set_reorder` stores that closure inside the slice's `Rc<Inner>`, so a
    /// closure capturing the model would capture the slice that owns it — a reference
    /// cycle, and the whole table (rows, contents, expand set) would leak once per
    /// container tab ever opened. This map references nothing, so capturing it is free.
    ids_by_uid: Rc<RefCell<HashMap<Uuid, u64>>>,
    /// **The authoritative expand set**, held outside the slice.
    ///
    /// `TreeDataSlice::build` rebuilds its own expanded set from the incoming rows and
    /// drops every key that is not among them (`expanded.retain(|k| row_pos.contains_key(k))`).
    /// That is right for the slice — it cannot expand a row it does not have — but it
    /// means **any re-source that narrows the rows silently forgets state**, and search
    /// narrows the rows on every keystroke. Expand a chapter, search for something that
    /// filters it out, clear the box: the slice has pruned it, and the chapter comes back
    /// collapsed.
    ///
    /// So the slice's set is treated as a *projection* of this one. Every user toggle
    /// writes here too, and every reload re-applies this set (unioned with whatever the
    /// slice decided, so `set_expand_new_nodes` still gets to auto-expand genuinely new
    /// rows).
    remembered: Rc<RefCell<HashSet<Uuid>>>,
    /// Keeps the filter-signal observers alive for the model's lifetime.
    _filters: Rc<Vec<ObserverHandle>>,
    /// One-shot guard so `wire` subscribes to backend events only once.
    subscribed: Rc<Cell<bool>>,
    /// The open Work's own id — guards the `LoadWork`/`NewWork` re-source
    /// subscription in [`Self::wire`] against a sibling Work's project boundary
    /// (see that method's docs). Not otherwise read: `reload`/the slice's own
    /// source closure already captured their own clone of it in [`Self::new`].
    work_id: Signal<Option<u64>>,
}

impl OverviewRowsModel {
    pub fn new(
        ctx: Rc<AppContext>,
        work_id: Signal<Option<u64>>,
        container_id: u64,
        counting_method: Signal<CountingMethodSetting>,
        filters: OverviewFilters,
        goal_unit: Signal<GoalUnit>,
    ) -> Self {
        let slice = TreeDataSlice::new();
        // A newly created scene appears expanded rather than hidden inside a collapsed
        // parent; the writer's later collapses survive reloads (the slice tracks a
        // `seen` set).
        slice.set_expand_new_nodes(true);
        let scope_contents: Rc<RefCell<HashSet<u64>>> = Rc::new(RefCell::new(HashSet::new()));
        // Optimistic until the first load says otherwise — a fresh tab is not "gone".
        let container_present = Signal::new(true);
        let remembered: Rc<RefCell<HashSet<Uuid>>> = Rc::new(RefCell::new(HashSet::new()));
        let ids_by_uid: Rc<RefCell<HashMap<Uuid, u64>>> = Rc::new(RefCell::new(HashMap::new()));

        {
            let ctx = ctx.clone();
            let work_id = work_id.clone();
            let f = filters.clone();
            let method = counting_method.clone();
            let unit = goal_unit.clone();
            let scope = scope_contents.clone();
            let present = container_present.clone();
            let ids = ids_by_uid.clone();
            slice.set_source(move || {
                let loaded = rows::load(&ctx, &work_id, container_id, method.get(), &unit.get());
                *scope.borrow_mut() = loaded.in_scope;
                if present.get() != loaded.container_present {
                    present.set(loaded.container_present);
                }
                *ids.borrow_mut() = loaded
                    .rows
                    .iter()
                    .map(|r| (r.key, r.item.item_id))
                    .collect();
                shape(loaded.rows, &f)
            });
        }

        // Every Overview row is a real item, and any of them may be dragged — unlike the
        // outline, whose binder roots are fixed. The container itself is not a row, so
        // there is nothing here that must stay put.
        slice.set_drag_policy(|_| DragEligibility::CanDrag);
        // A drop `Into` a leaf is meaningless (a Scene holds no children), so it lands
        // *after* it instead — the same redirect the outline applies. The slice's cycle
        // guard (self / own descendant) has already run by this point.
        slice.set_drop_resolver(|_dragged, _target, target_row, position| match position {
            DropPosition::Into if target_row.role != BinderItemRole::Folder => {
                Some(DropPosition::After)
            }
            p => Some(p),
        });
        // Deliberately NOT loaded here. `ContentTab::new` builds this model for every
        // Overview-capable container, but a tab opens on its own page — so loading in the
        // constructor would read every content row under a Book and fold its word counts
        // on the UI thread for a pane the writer may never select. `wire()` (called when
        // the pane is actually built) does the first load, exactly as
        // `CorkboardCardsModel` does.

        // Re-source on any shaping change.
        //
        // Search flips the **reveal override** (`set_all_expanded`), which
        // `TreeDataSlice` documents as preserving the per-view expand set underneath —
        // "turning it off restores the user's real collapse state". So there is nothing
        // to snapshot: the override is a display mode, not a mutation.
        //
        // An earlier version *did* snapshot on entry and restore on exit, which was worse
        // than redundant — it overwrote the live set, so any chapter the writer collapsed
        // while a search was open sprang back the moment they cleared the box.
        //
        // A sort change is not a search: `set_all_expanded` no-ops when unchanged, so a
        // sort never touches expand state at all.
        //
        // Cycle-safe: the closure captures the slice + signals, never `self` (capturing
        // the model would form model → closure → model).
        let resource: Rc<dyn Fn()> = {
            let slice = slice.clone();
            let f = filters.clone();
            let remembered = remembered.clone();
            Rc::new(move || {
                slice.set_all_expanded(row_search::needle(&f.query.get()).is_some());
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
                filters.sort.observe(move |_| r())
            },
            {
                let r = resource.clone();
                counting_method.observe(move |_| r())
            },
            {
                // Switching the project's unit changes which of the two stored targets
                // every Target cell reads. Nothing about the prose changed, but the column
                // would keep printing the old unit's numbers until something else forced a
                // reload.
                let r = resource.clone();
                goal_unit.observe(move |_| r())
            },
        ];

        Self {
            slice,
            scope_contents,
            container_present,
            ids_by_uid,
            remembered,
            _filters: Rc::new(observers),
            subscribed: Rc::new(Cell::new(false)),
            work_id,
        }
    }

    /// Subscribe once so the table re-sources on any backend change that can alter it —
    /// whoever caused it (this table, the outline, the corkboard, a stream row, an
    /// import, an undo).
    pub fn wire(&self, ctx: &mut BuildContext) {
        if self.subscribed.replace(true) {
            return;
        }
        // The first load: this is the moment the pane exists, and it is why the
        // constructor deliberately does not load (see `new`).
        self.slice.reload();
        use DirectAccessEntity::BinderItem;
        use EntityEvent::{Created, Removed, Updated};
        // Structural + metadata changes: any of these can add, drop, move or rename a row.
        // `Created` is handled separately below — it is the one origin that must also
        // open the way to the new row, not merely re-source.
        let origins = [
            Origin::DirectAccess(BinderItem(Updated)),
            Origin::DirectAccess(BinderItem(Removed)),
            // A `Work` update carries the numbering settings — flipping them in
            // Settings changes every ordinal badge here, and nothing else here
            // would notice.
            Origin::DirectAccess(DirectAccessEntity::Work(Updated)),
            Origin::BinderItemManagement(BinderItemManagementEvent::Duplicate),
            Origin::BinderItemManagement(BinderItemManagementEvent::MoveItems),
            Origin::BinderItemManagement(BinderItemManagementEvent::MergeTwoScenes),
            Origin::BinderItemManagement(BinderItemManagementEvent::SplitScene),
            Origin::BinderItemManagement(BinderItemManagementEvent::Promote),
            Origin::TrashManagement(TrashManagementEvent::TrashBinderItems),
            Origin::TrashManagement(TrashManagementEvent::TrashBinder),
            Origin::TrashManagement(TrashManagementEvent::RestoreItems),
            Origin::TrashManagement(TrashManagementEvent::EmptyTrash),
        ];
        // Coalesced — see `models::coalesced_reload`.
        {
            let me = self.clone();
            crate::models::coalesced_reload::reload_on_events(ctx, origins, move || me.reload());
        }
        // Project (re)load — guarded (loose form): the slice's own source closure
        // always re-derives from this model's own `work_id`, so a sibling Work's
        // Load/New would only cost a harmless, still-correct re-derive; guarded
        // anyway so opening a second Work doesn't force a wasted rebuild of every
        // other open window's Overview table.
        for wev in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
            let me = self.clone();
            let work_id = self.work_id.clone();
            ctx.subscribe_event(Origin::WorkManagement(wev), move |event: &Event| {
                let mine = work_id.get();
                if mine.is_none() || event.ids.first() == mine.as_ref() {
                    me.reload();
                }
            });
        }
        // A create must also *reveal*: give a childless container its first child and
        // the row exists but sits under a parent that, having had nothing to show, was
        // never expanded. Reload first so the row is in the tree, then walk up from it.
        //
        // The event carries store ids, which are re-minted by every `load_work`, so the
        // id is resolved back through `ids_by_uid` to the durable uid this tree keys on.
        {
            let me = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(BinderItem(Created)),
                move |e: &Event| {
                    me.reload();
                    let Some(&id) = e.ids.first() else {
                        return;
                    };
                    let uid = me
                        .ids_by_uid
                        .borrow()
                        .iter()
                        .find(|(_, store_id)| **store_id == id)
                        .map(|(uid, _)| *uid);
                    if let Some(uid) = uid {
                        me.expand_ancestors(&uid);
                    }
                },
            );
        }
        // Prose edits move the numbers, so the word columns would otherwise go stale the
        // moment the writer types in a scene with the Overview open beside it. Scoped to
        // the content rows this table actually shows: an edit anywhere else in the
        // manuscript is none of its business.
        {
            let me = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::Content(Updated)),
                move |e: &Event| {
                    if me.touches_scope(&e.ids) {
                        me.reload();
                    }
                },
            );
        }
    }

    /// Does this event touch a `Content` row the table is currently showing?
    ///
    /// An event carrying **no** ids is treated as in-scope: a bulk or unattributed
    /// content change is exactly the case where refusing to reload would leave stale
    /// numbers on screen, and a spurious reload is merely a re-read.
    fn touches_scope(&self, ids: &[u64]) -> bool {
        if ids.is_empty() {
            return true;
        }
        let scope = self.scope_contents.borrow();
        ids.iter().any(|id| scope.contains(id))
    }

    /// The uid → store-id map for the loaded rows, for a caller that must resolve keys
    /// without holding this model (see [`Self::ids_by_uid`]).
    pub fn ids_by_uid(&self) -> Rc<RefCell<HashMap<Uuid, u64>>> {
        self.ids_by_uid.clone()
    }

    /// Inject the reorder command. On a successful move the slice re-sources itself.
    pub fn set_reorder(&self, commit: CommitMove) {
        self.slice
            .set_reorder(move |dragged, target, place| commit(dragged, target, place));
    }

    /// Re-source the rows, preserving the writer's expand state across the re-source.
    pub fn reload(&self) {
        reload_preserving(&self.slice, &self.remembered);
    }

    /// Resolve a uid to its live store id — the bridge every command call crosses.
    pub fn item_id_of(&self, uid: &Uuid) -> Option<u64> {
        self.slice.with_key(uid, |r| r.item_id)
    }

    /// A clone of the row behind `uid`, for a view-model that needs its metadata.
    pub fn row_of(&self, uid: &Uuid) -> Option<OverviewRow> {
        self.slice.with_key(uid, |r| r.clone())
    }

    /// Whether the container still exists in the binder (see [`Loaded`]).
    pub fn container_present(&self) -> Signal<bool> {
        self.container_present.clone()
    }

    /// The expanded set, by durable uid — what expand-state persistence stores.
    ///
    /// Reads the **real** set, never the search reveal override, so capturing while a
    /// filter is open records what the writer actually collapsed rather than
    /// "everything, because a search was running".
    pub fn expanded_uids(&self) -> Vec<Uuid> {
        self.slice.expanded_keys()
    }

    /// Apply a remembered expanded set (the persistence restore).
    pub fn set_expanded_uids(&self, uids: &[Uuid]) {
        *self.remembered.borrow_mut() = uids.iter().copied().collect();
        self.slice.set_expanded_keys(uids);
    }

    /// A `Weak` to one of this model's own allocations — a test hook for proving the
    /// model actually drops (i.e. that nothing it installed holds it alive).
    #[cfg(all(test, feature = "mocks"))]
    pub fn weak_probe(&self) -> std::rc::Weak<RefCell<HashSet<u64>>> {
        Rc::downgrade(&self.scope_contents)
    }

    /// Expand every ancestor of `uid` — the Overview's half of the same guarantee
    /// the outline makes in
    /// [`BinderBinderItemsTreeModel::expand_ancestors`](crate::models::BinderBinderItemsTreeModel::expand_ancestors).
    ///
    /// The two trees keep *separate* expand state (both persisted in
    /// `tree_expansion.toml`), so expanding a parent in the outline leaves the same
    /// parent shut here. A first child created from the Overview's own Create button
    /// would otherwise land invisibly in this table too.
    ///
    /// Goes through `set_expanded` so each step mirrors into `remembered` and the
    /// expansion survives the next reload.
    pub fn expand_ancestors(&self, uid: &Uuid) {
        use teksilo::data::TreeDataSource;
        let mut cur = self.parent(uid);
        while let Some(p) = cur {
            self.set_expanded(&p, true);
            cur = self.parent(&p);
        }
    }

    pub fn expand_all(&self) {
        self.slice.expand_all();
        *self.remembered.borrow_mut() = self.slice.expanded_keys().into_iter().collect();
    }

    pub fn collapse_all(&self) {
        self.slice.collapse_all();
        // Cleared outright, not intersected with the visible rows: "collapse all" is a
        // statement about the container, and a row hidden by a filter must not come back
        // expanded just because it was not on screen when the writer said so.
        self.remembered.borrow_mut().clear();
    }
}

/// Re-source, then restore the expand state the re-source pruned.
///
/// The union with the slice's own post-reload set is what keeps
/// `set_expand_new_nodes(true)` working: a genuinely new row is not in `remembered`, but
/// the slice has just auto-expanded it, and `set_expanded_keys` **replaces** rather than
/// merges — so restoring `remembered` alone would collapse every newly created scene.
/// Folding the union back into `remembered` is what makes that auto-expansion stick.
fn reload_preserving(
    slice: &TreeDataSlice<Uuid, OverviewRow>,
    remembered: &RefCell<HashSet<Uuid>>,
) {
    slice.reload();
    let mut union = remembered.borrow().clone();
    union.extend(slice.expanded_keys());
    let keys: Vec<Uuid> = union.iter().copied().collect();
    slice.set_expanded_keys(&keys);
    *remembered.borrow_mut() = union;
}

/// Apply the search filter and the sort to a freshly loaded row set.
///
/// Search uses `KeepAncestors`: a match stays *reachable*, so its enclosing chapter and
/// part remain as context rather than the match appearing at the root of nowhere. Sort is
/// **per sibling group** — `TreeRowFilter::sort` reorders children within each parent and
/// never flattens the hierarchy, because a book whose scenes were globally sorted by word
/// count would no longer be a book.
fn shape(
    rows: Vec<TreeRow<Uuid, OverviewRow>>,
    filters: &OverviewFilters,
) -> Vec<TreeRow<Uuid, OverviewRow>> {
    let needle = row_search::needle(&filters.query.get());
    let sort = filters.sort.get();
    if needle.is_none() && sort.is_none() {
        return rows;
    }
    let mut sieve = TreeRowFilter::new().filter_mode(TreeFilterMode::KeepAncestors);
    if let Some(needle) = needle {
        sieve = sieve.filter(move |r: &OverviewRow| {
            // Title and label only — the two things the table actually shows. Matching
            // on data with no column would keep rows the writer sees no reason for.
            row_search::row_matches(&needle, &[&r.title, &r.label])
        });
    }
    if let Some((col, dir)) = sort {
        let cmp = comparator(&col);
        sieve = match dir {
            SortDirection::Ascending => sieve.sort(cmp),
            SortDirection::Descending => sieve.sort_desc(cmp),
        };
    }
    sieve.apply(rows)
}

/// The comparator for a column id. An unknown id sorts everything equal, which leaves the
/// rows in manuscript order — the honest answer for "sort by a column that no longer
/// exists" (a persisted sort naming a removed column), and never a panic.
fn comparator(col_id: &str) -> impl Fn(&OverviewRow, &OverviewRow) -> std::cmp::Ordering + 'static {
    let col = col_id.to_string();
    move |a: &OverviewRow, b: &OverviewRow| match col.as_str() {
        COL_TITLE => a.title.to_lowercase().cmp(&b.title.to_lowercase()),
        COL_LABEL => a.label.to_lowercase().cmp(&b.label.to_lowercase()),
        // A row with no prose at all sorts below an empty one: "nothing to count" is
        // less than "counted, and it was zero".
        COL_OWN_WORDS => a.own_words.cmp(&b.own_words),
        COL_TOTAL_WORDS => a.total_words.cmp(&b.total_words),
        COL_OPEN_COMMENTS => a.own_comments.cmp(&b.own_comments),
        COL_TOTAL_COMMENTS => a.total_comments.cmp(&b.total_comments),
        // Structural rank, not the label's alphabet — sorting by type should group a
        // book's parts above its chapters above its scenes, in the order they nest,
        // which "Book, Chapter, Part, Scene" would not.
        COL_TYPE => type_rank(&a.role, &a.sub_role).cmp(&type_rank(&b.role, &b.sub_role)),
        _ => std::cmp::Ordering::Equal,
    }
}

/// Structural depth-rank of a `(role, sub_role)`, outermost first. Drives the Type
/// column's sort so the order reads as the manuscript nests, not as the alphabet.
fn type_rank(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> u8 {
    use BinderItemRole::{Folder, Item};
    use BinderItemSubRole::*;
    match (role, sub_role) {
        (_, Book) | (_, BookBegin) => 0,
        (_, Part) => 1,
        (_, ChapterScene) => 2,
        (_, Scene) => 3,
        (Folder, Note) => 4,
        (Item, Note) => 5,
        (_, Text) => 6,
        (_, BookEnd) => 7,
        (Folder, None) => 8,
        _ => 9,
    }
}

/// One row-source load: the rows, whether the container still exists, and the content
/// ids to scope `Content(Updated)` reloads by.
///
/// `container_present` is **not** derivable from `rows.is_empty()` — a container that was
/// just trashed and one that is merely empty both yield no rows, and only the first means
/// the tab is looking at something that is gone.
pub(crate) struct Loaded {
    pub rows: Vec<TreeRow<Uuid, OverviewRow>>,
    pub container_present: bool,
    pub in_scope: HashSet<u64>,
}

/// Where a container sits in the work's flat item stream, and what belongs to it.
///
/// Distinguishes the two cases the caller must not conflate: the container is **absent**
/// from the stream (it was trashed, or the work is not loaded) versus present and simply
/// **childless**. Both produce an empty table, but only one of them means the tab is
/// looking at something that no longer exists.
#[cfg_attr(feature = "mocks", allow(dead_code))]
pub(crate) enum Subtree<'a, T> {
    /// No row in the stream has this id.
    ContainerGone,
    /// The container's descendants (possibly empty), and its own indent to rebase from.
    Found { rows: &'a [T], base: i64 },
}

/// The contiguous subtree of `container_id`: every row after it whose indent stays deeper
/// than its own, stopping at the first row that returns to the container's level or
/// shallower — **or at the first row belonging to a different binder**.
///
/// The container's own row is not included: the Overview tabulates what is *inside* the
/// container, and the container itself is the tab you are already looking at.
///
/// **The binder guard is load-bearing, not defensive.** The flat stream is
/// binder-major with no separator and indents are scoped *per binder*, so a container
/// that happens to be the last item of its binder would otherwise keep walking into the
/// next binder and adopt its leading items — folding their word counts into its total —
/// whenever those start at a deeper indent than the container. Stopping at the binder
/// change makes the walk correct regardless of what indent the next binder opens on.
///
/// Lives up here, above the `#[cfg]`-gated `rows` seam, so it is exercised by tests under
/// **both** feature sets; only the real row source calls it (the mock fabricates its
/// subtree directly), which is why the mocks build sees it as unused outside tests.
#[cfg_attr(feature = "mocks", allow(dead_code))]
pub(crate) fn subtree_of<T>(
    flat: &[T],
    container_id: u64,
    id_of: impl Fn(&T) -> u64,
    indent_of: impl Fn(&T) -> i64,
    binder_of: impl Fn(&T) -> u64,
) -> Subtree<'_, T> {
    let Some(pos) = flat.iter().position(|it| id_of(it) == container_id) else {
        return Subtree::ContainerGone;
    };
    let base = indent_of(&flat[pos]);
    let binder = binder_of(&flat[pos]);
    let after = &flat[pos + 1..];
    let end = after
        .iter()
        .position(|it| indent_of(it) <= base || binder_of(it) != binder)
        .unwrap_or(after.len());
    Subtree::Found {
        rows: &after[..end],
        base,
    }
}

/// Fold `manuscript_words` up the tree into `total_words`, in one reverse pass.
///
/// **Not `own_words`.** The two differ for a row the export leaves out, and the totals
/// have to agree with `count_words_uc` — the same admission gate, applied per row with no
/// cascade, so a non-exportable chapter folder still counts the scenes inside it.
///
/// Walking backwards, every entry still on the stack that is *deeper* than the current
/// row is one of its direct children — each already carrying its own subtree's total,
/// because processing it popped its own descendants. So popping them all and summing is
/// exactly this row's subtree, with no double counting and no second traversal.
///
/// This runs over the whole subtree at load, not lazily per visible row, because a
/// collapsed container must still show a correct total — which is the only reason the
/// column is worth having.
pub(crate) fn fold_totals(rows: &mut [TreeRow<Uuid, OverviewRow>]) {
    // (depth, words, comments) for each not-yet-consumed forest root, deepest last.
    //
    // Both totals fold in ONE reverse pass rather than two: the walk is the
    // expensive part, the arithmetic is not, and two passes would be two chances
    // for the stack discipline to drift apart.
    let mut stack: Vec<(usize, usize, usize)> = Vec::new();
    for row in rows.iter_mut().rev() {
        let depth = row.depth;
        let mut total = row.item.manuscript_words;
        let mut total_c = row.item.own_comments;
        while let Some(&(child_depth, child_total, child_comments)) = stack.last() {
            if child_depth > depth {
                total += child_total;
                total_c += child_comments;
                stack.pop();
            } else {
                break;
            }
        }
        row.item.total_words = total;
        row.item.total_comments = total_c;
        stack.push((depth, total, total_c));
    }
}

/// Straight delegation onto the backing [`TreeDataSlice`].
impl TreeDataSource for OverviewRowsModel {
    type Item = OverviewRow;
    type Key = Uuid;

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

    fn key_at(&self, flat_index: usize) -> Option<Uuid> {
        self.slice.key_at(flat_index)
    }

    fn flat_index_of(&self, key: &Uuid) -> Option<usize> {
        self.slice.flat_index_of(key)
    }

    fn parent(&self, key: &Uuid) -> Option<Uuid> {
        self.slice.parent_of(key)
    }

    fn child_keys(&self, key: &Uuid) -> Vec<Uuid> {
        self.slice.child_keys_of(key)
    }

    fn version_signal(&self) -> Signal<u64> {
        self.slice.version_signal()
    }

    fn first_changed_index(&self) -> Option<usize> {
        self.slice.first_changed_index()
    }

    fn contains_key(&self, key: &Uuid) -> bool {
        self.slice.contains_key(key)
    }

    fn is_expanded(&self, key: &Uuid) -> bool {
        self.slice.is_expanded(key)
    }

    fn set_expanded(&self, key: &Uuid, expanded: bool) {
        // Mirror every toggle into the authoritative set — this is the write that makes
        // the slice's own set a projection rather than the truth.
        if expanded {
            self.remembered.borrow_mut().insert(*key);
        } else {
            self.remembered.borrow_mut().remove(key);
        }
        self.slice.set_expanded(key, expanded);
    }

    fn drag(&self, key: &Uuid) -> DragEligibility {
        self.slice.drag(key)
    }

    fn can_accept(&self, query: &DropQuery<'_, Uuid>) -> DropResponse {
        self.slice.can_accept(query)
    }

    fn accept_drop(&self, commit: DropCommit<'_, Uuid>) -> bool {
        self.slice.accept_drop(commit)
    }
}

// ── The row-source seam: the only real/mock difference ──────────────────────

/// Load the container's subtree from the backend: one ordered pass over the work's items,
/// sliced to the container, then **one batched** content read for the whole subtree.
#[cfg(not(feature = "mocks"))]
mod rows {
    use std::collections::{HashMap, HashSet};

    use teksilo::data::TreeRow;
    use teksilo::prelude::Signal;
    use uuid::Uuid;

    use frontend::AppContext;
    use frontend::commands::{
        binder_commands, binder_item_commands, comment_commands, content_commands, work_commands,
    };
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::comment::CommentRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::{ContentRole, GoalUnit};
    use frontend::direct_access::BinderItemDto;
    use skribisto_model::counting::{self, CountMethod, CountingMethodSetting};
    use skribisto_model::language;

    use super::{Loaded, OverviewRow, Subtree, fold_totals, subtree_of};

    /// The annotated `Content` id of every **open** thread in `work_id`, one entry
    /// per thread (so a content row with three open threads appears three times).
    ///
    /// Resolved and orphaned threads are excluded: the Overview column answers
    /// "what still needs me", and an orphan has no scene to be counted against
    /// anyway — it lives in the docks' "no home" bucket.
    fn open_comment_counts(ctx: &AppContext, work_id: u64) -> Vec<u64> {
        let ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Comments)
                .unwrap_or_default();
        if ids.is_empty() {
            return Vec::new();
        }
        comment_commands::get_comment_multi(ctx, &ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .filter(|c| !c.resolved && !c.orphaned)
            .filter_map(|c| {
                comment_commands::get_comment_relationship(
                    ctx,
                    &c.id,
                    &CommentRelationshipField::Content,
                )
                .unwrap_or_default()
                .into_iter()
                .next()
            })
            .collect()
    }

    pub fn load(
        ctx: &AppContext,
        work_id: &Signal<Option<u64>>,
        container_id: u64,
        method: CountingMethodSetting,
        unit: &GoalUnit,
    ) -> Loaded {
        let gone = |present: bool| Loaded {
            rows: Vec::new(),
            container_present: present,
            in_scope: HashSet::new(),
        };
        let Some(work_id) = work_id.get() else {
            // No project open: the container is not *missing*, there is simply nothing
            // loaded yet, so don't accuse the tab of pointing at a deleted item.
            return gone(true);
        };
        let flat = flat_items(ctx, work_id);
        // Numbered from the whole Work before the subtree below narrows it: an Overview
        // opened on Part Two must still call its chapters eleven and twelve.
        let numbers = {
            let metas: Vec<skribisto_model::compile::ItemMeta> = flat
                .iter()
                .map(|(_, it)| crate::models::item_meta_of(it))
                .collect();
            crate::models::numbers_for_work(ctx, work_id, &metas)
        };
        let work_langs = crate::models::work_language_tags(ctx, work_id);
        let Subtree::Found {
            rows: subtree,
            base,
        } = subtree_of(
            &flat,
            container_id,
            |(_, it)| it.id,
            |(_, it)| it.indent,
            |(binder, _)| *binder,
        )
        else {
            return gone(false);
        };
        if subtree.is_empty() {
            return gone(true); // present, just childless
        }
        let mut in_scope = HashSet::new();

        // One read for every content row in the subtree, rather than one per item: a
        // Book's Overview would otherwise issue a query per scene on every reload.
        let content_ids: Vec<u64> = subtree
            .iter()
            .flat_map(|(_, it)| it.contents.iter().copied())
            .collect();
        in_scope.extend(content_ids.iter().copied());
        let contents = content_commands::get_content_multi(ctx, &content_ids).unwrap_or_default();
        // Index by owning item so each row picks up only its own scene prose.
        let mut by_item: HashMap<u64, Vec<(ContentRole, String)>> = HashMap::new();
        let mut open_comments: HashMap<u64, usize> = HashMap::new();
        {
            let mut owner: HashMap<u64, u64> = HashMap::new();
            for (_, it) in subtree {
                for cid in &it.contents {
                    owner.insert(*cid, it.id);
                }
            }
            for c in contents.into_iter().flatten() {
                if let Some(item) = owner.get(&c.id) {
                    by_item.entry(*item).or_default().push((c.role, c.data));
                }
            }
            // Open threads per item, bucketed through the very same content→item
            // map: `Content` has no back-pointer to its `BinderItem`, so this
            // inversion is the only way to answer "which scene is this comment in",
            // and building it twice would be two chances to disagree.
            for cm in open_comment_counts(ctx, work_id) {
                if let Some(item) = owner.get(&cm) {
                    *open_comments.entry(*item).or_default() += 1;
                }
            }
        }

        let mut rows: Vec<TreeRow<Uuid, OverviewRow>> = subtree
            .iter()
            .map(|(_, it)| {
                let own = by_item.get(&it.id);
                let scene = own.and_then(|cs| {
                    cs.iter()
                        .find(|(role, _)| *role == ContentRole::SceneText)
                        .map(|(_, data)| data)
                });
                // `counts_prose` — not "has a SceneText row" — so a prose-bearing item
                // whose row has not been created yet still reads as `Some(0)` rather
                // than blank. The distinction is the column's whole meaning.
                let own_words = skribisto_model::counts_prose(&it.role, &it.sub_role).then(|| {
                    let m = counting::resolve_method(
                        method,
                        CountMethod::UnicodeWords,
                        language::primary(&it.dict_language),
                    );
                    // Content-addressed cache: an unchanged scene is a hit, so a reload
                    // triggered by an edit elsewhere re-counts only what changed.
                    scene.map_or(0, |d| counting::cached_count(d, m).words)
                });
                TreeRow::new(
                    it.uid,
                    OverviewRow {
                        item_id: it.id,
                        uid: it.uid,
                        role: it.role.clone(),
                        sub_role: it.sub_role.clone(),
                        title: it.title.clone(),
                        label: it.label.clone(),
                        own_words,
                        is_exportable: it.is_exportable,
                        // Per item, no cascade — the same rule `count_words_uc` and the
                        // export scope resolver apply, so a non-exportable chapter folder
                        // still counts the scenes inside it.
                        manuscript_words: if it.is_exportable {
                            own_words.unwrap_or(0)
                        } else {
                            0
                        },
                        goal: match unit {
                            GoalUnit::Words => it.word_count_goal,
                            GoalUnit::Characters => it.char_count_goal,
                        },
                        total_words: 0, // filled by the fold below
                        tags: it.tags.clone(),
                        number: numbers
                            .get(&it.id)
                            .map(skribisto_model::numbering::Numbered::number),
                        fallback_label: crate::models::fallback_label_for(
                            it,
                            numbers.get(&it.id),
                            &work_langs,
                        ),
                        own_comments: open_comments.get(&it.id).copied().unwrap_or(0),
                        total_comments: 0, // filled by the same fold
                    },
                    // Rebase onto the container: its direct children are depth 0, since
                    // the container's own row is not in the table.
                    (it.indent - base - 1).max(0) as usize,
                )
            })
            .collect();
        fold_totals(&mut rows);
        Loaded {
            rows,
            container_present: true,
            in_scope,
        }
    }

    /// Every activated binder item of `work_id` **paired with its owning binder**,
    /// binder-major, in each binder's stored relationship order — the same order a save
    /// writes. `indent` nests them *within* a binder, which is why the binder id has to
    /// travel alongside: it is the only thing marking where one binder's indents stop
    /// meaning anything to the next (see `subtree_of`).
    fn flat_items(ctx: &AppContext, work_id: u64) -> Vec<(u64, BinderItemDto)> {
        let mut out = Vec::new();
        let binder_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();
        for binder_id in binder_ids {
            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                &binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            // `get_binder_item_multi` returns db-key order, so index by id and walk
            // `item_ids` (the authoritative relationship order).
            let by_id: HashMap<u64, BinderItemDto> =
                binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                    .map(|it| (it.id, it))
                    .collect();
            for id in item_ids {
                if let Some(it) = by_id.get(&id)
                    && it.activated
                {
                    out.push((binder_id, it.clone()));
                }
            }
        }
        out
    }
}

/// The static mock subtree (no backend). Mirrors the fixture ids used by the outline,
/// corkboard and stream mocks, so the same item is the same item across every view.
#[cfg(feature = "mocks")]
mod rows {
    use std::collections::HashSet;

    use teksilo::data::TreeRow;
    use teksilo::prelude::Signal;
    use uuid::Uuid;

    use frontend::AppContext;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole, GoalUnit};
    use skribisto_model::counting::CountingMethodSetting;

    use super::{Loaded, OverviewRow, fold_totals};

    #[allow(clippy::too_many_arguments)]
    fn row(
        item_id: u64,
        role: BinderItemRole,
        sub_role: BinderItemSubRole,
        title: &str,
        label: &str,
        own_words: Option<usize>,
        depth: usize,
        tags: &[u64],
    ) -> TreeRow<Uuid, OverviewRow> {
        // Distinct per row: `fixture_uid` is deterministic, so the same mock row keeps
        // the same identity across refreshes (a fresh `new_uid()` per call would make
        // every reload look like a brand-new tree to anything keyed by uid).
        let uid = common::uid::fixture_uid(item_id);
        TreeRow::new(
            uid,
            OverviewRow {
                item_id,
                uid,
                role,
                sub_role,
                title: title.to_string(),
                label: label.to_string(),
                own_words,
                // The fixture keeps one row out of the export so the mocks build shows the
                // muted cell and the total that excludes it — the pair this feature's
                // whole "own length versus book length" distinction rests on.
                is_exportable: item_id != 303,
                manuscript_words: if item_id == 303 {
                    0
                } else {
                    own_words.unwrap_or(0)
                },
                goal: match item_id {
                    301 => 12_000,
                    104 => 4_000,
                    105 => 10_000,
                    201 | 202 => 2_000,
                    _ => 0,
                },
                total_words: 0, // filled by the fold below
                tags: tags.to_vec(),
                // Every fixture row is titled, so none needs the fallback.
                fallback_label: None,
                // The fixture book's chapter ordinals, in step with the mock binder tree.
                number: match item_id {
                    301u64 => Some(1), // Part One
                    104 => Some(1),    // the first chapter
                    302 => Some(2),
                    105 => Some(3),
                    _ => None,
                },
                // A couple of fabricated threads on the prose-bearing rows, so the
                // mock build exercises the column and its fold rather than a
                // uniformly-zero one that would hide an arithmetic bug.
                own_comments: if own_words.is_some() {
                    item_id as usize % 3
                } else {
                    0
                },
                total_comments: 0, // filled by the same fold
            },
            depth,
        )
    }

    pub fn load(
        _ctx: &AppContext,
        _work_id: &Signal<Option<u64>>,
        container_id: u64,
        _method: CountingMethodSetting,
        _unit: &GoalUnit,
    ) -> Loaded {
        use BinderItemRole::{Folder, Item};
        use BinderItemSubRole::{ChapterScene, Note, Part, Scene};
        let mut rows = match container_id {
            // Book One (101) — a part holding a chapter folder and a flat chapter, then
            // a loose chapter. Two levels deep, so the fold has something to fold.
            101 => vec![
                row(
                    102,
                    Item,
                    BinderItemSubRole::BookBegin,
                    "Opening",
                    "1st plot point",
                    None,
                    0,
                    &[],
                ),
                row(103, Item, Scene, "Scene at dawn", "", Some(412), 0, &[]),
                row(301, Folder, Part, "Part One — Arrival", "", None, 0, &[]),
                row(
                    104,
                    Folder,
                    ChapterScene,
                    "Chapter Two",
                    "rising action",
                    Some(90),
                    1,
                    &[],
                ),
                row(
                    201,
                    Item,
                    Scene,
                    "Scene 1",
                    "opening beat",
                    Some(1180),
                    2,
                    &[1, 2],
                ),
                row(202, Item, Scene, "Scene 2", "", Some(640), 2, &[3]),
                row(203, Item, Note, "First night", "", None, 2, &[]),
                row(
                    302,
                    Item,
                    ChapterScene,
                    "Into the Dark",
                    "",
                    Some(755),
                    1,
                    &[],
                ),
                row(303, Item, Scene, "The light returns", "", Some(300), 1, &[]),
                row(
                    105,
                    Item,
                    ChapterScene,
                    "Confrontation",
                    "",
                    Some(1502),
                    0,
                    &[1],
                ),
            ],
            // Part One (301) — its two chapters, one of them a folder with scenes.
            301 => vec![
                row(
                    104,
                    Folder,
                    ChapterScene,
                    "Chapter Two",
                    "rising action",
                    Some(90),
                    0,
                    &[],
                ),
                row(
                    201,
                    Item,
                    Scene,
                    "Scene 1",
                    "opening beat",
                    Some(1180),
                    1,
                    &[1, 2],
                ),
                row(202, Item, Scene, "Scene 2", "", Some(640), 1, &[3]),
                row(203, Item, Note, "First night", "", None, 1, &[]),
                row(
                    302,
                    Item,
                    ChapterScene,
                    "Into the Dark",
                    "",
                    Some(755),
                    0,
                    &[],
                ),
            ],
            // A chapter folder (104) — its own scenes, one level.
            104 => vec![
                row(
                    201,
                    Item,
                    Scene,
                    "Scene 1",
                    "opening beat",
                    Some(1180),
                    0,
                    &[1, 2],
                ),
                row(202, Item, Scene, "Scene 2", "", Some(640), 0, &[3]),
                row(203, Item, Note, "First night", "", None, 0, &[]),
            ],
            // Anything else (including a genuinely empty container) shows nothing.
            _ => Vec::new(),
        };
        fold_totals(&mut rows);
        Loaded {
            rows,
            // The fixture only knows three containers; treat every id as present so the
            // mock build never shows the "this container is gone" state spuriously.
            container_present: true,
            in_scope: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An exportable fixture row: its own words are also what it contributes to a total.
    fn r(uid: u64, own: Option<usize>, depth: usize) -> TreeRow<Uuid, OverviewRow> {
        row_with(uid, own, depth, true)
    }

    /// A fixture row that names its own exportability, so the two columns can be checked
    /// against each other where they are meant to disagree.
    fn row_with(
        uid: u64,
        own: Option<usize>,
        depth: usize,
        is_exportable: bool,
    ) -> TreeRow<Uuid, OverviewRow> {
        TreeRow::new(
            common::uid::fixture_uid(uid),
            OverviewRow {
                item_id: uid,
                uid: common::uid::fixture_uid(uid),
                own_words: own,
                is_exportable,
                manuscript_words: if is_exportable { own.unwrap_or(0) } else { 0 },
                ..Default::default()
            },
            depth,
        )
    }

    /// The one place the two word columns are meant to disagree.
    ///
    /// A scene the writer has taken out of the export keeps its own length — the writing
    /// is still there, and "how long is this piece" is still a question with an answer —
    /// but it is no longer part of the book, so nothing above it counts it. This mirrors
    /// `count_words_uc`'s own test at the backend layer; the two admission gates have to
    /// stay the same gate or a chapter's progress bar would disagree with its export.
    #[test]
    fn a_row_left_out_of_the_export_keeps_its_length_but_leaves_every_total() {
        let mut rows = vec![
            r(1, None, 0),                    // chapter folder
            r(2, Some(1_200), 1),             // an ordinary scene
            row_with(3, Some(900), 1, false), // cut, but kept
        ];
        fold_totals(&mut rows);
        assert_eq!(
            rows[2].item.own_words,
            Some(900),
            "its own length is still the truth about it"
        );
        assert_eq!(
            rows[0].item.total_words, 1_200,
            "the chapter counts only what reaches the book"
        );
    }

    /// Exclusion is per row and never inherited — the same rule the exporter and
    /// `count_words_uc` apply, and the reason the Inspector needs an explicit
    /// "Apply to children" button beside the switch.
    #[test]
    fn excluding_a_container_does_not_exclude_what_is_inside_it() {
        let mut rows = vec![
            row_with(1, None, 0, false), // an excluded chapter folder…
            r(2, Some(500), 1),          // …whose scenes are not excluded
            r(3, Some(300), 1),
        ];
        fold_totals(&mut rows);
        assert_eq!(
            rows[0].item.total_words, 800,
            "the scenes inside still reach the book"
        );
    }

    /// The fold sums each row's own words plus its whole subtree's — the number a
    /// *collapsed* container must still show.
    #[test]
    fn totals_fold_bottom_up_through_several_levels() {
        // part(0) ├ chapter(1) ├ scene(2) = 100
        //         │            └ scene(2) = 200
        //         └ scene(1)             = 50
        let mut rows = vec![
            r(1, None, 0),      // part
            r(2, Some(10), 1),  // chapter folder, own prose
            r(3, Some(100), 2), // scene
            r(4, Some(200), 2), // scene
            r(5, Some(50), 1),  // scene directly under the part
        ];
        fold_totals(&mut rows);
        let totals: Vec<usize> = rows.iter().map(|r| r.item.total_words).collect();
        assert_eq!(
            totals,
            vec![360, 310, 100, 200, 50],
            "part = 10+100+200+50, chapter = 10+100+200, leaves = their own"
        );
    }

    /// A row with no prose contributes nothing but still totals its descendants — the
    /// Part / Book case, which is the whole reason the column is not just `own_words`.
    #[test]
    fn a_prose_less_container_still_totals_its_children() {
        let mut rows = vec![r(1, None, 0), r(2, Some(7), 1)];
        fold_totals(&mut rows);
        assert_eq!(rows[0].item.total_words, 7);
        assert_eq!(
            rows[0].item.own_words, None,
            "and reports no prose of its own"
        );
    }

    /// Comments fold through the same single pass as words. Asserted on a shape
    /// where the two differ (a prose-less Part carrying its own thread) so a bug
    /// that folded one into the other could not hide behind equal numbers.
    #[test]
    fn open_comments_fold_bottom_up_alongside_words() {
        let mut rows = vec![r(1, None, 0), r(2, Some(10), 1), r(3, Some(20), 2)];
        rows[0].item.own_comments = 1; // on the Part itself
        rows[1].item.own_comments = 0;
        rows[2].item.own_comments = 4;
        fold_totals(&mut rows);
        assert_eq!(rows[0].item.total_comments, 5, "1 on the part + 4 below");
        assert_eq!(rows[1].item.total_comments, 4, "0 of its own + 4 below");
        assert_eq!(rows[2].item.total_comments, 4);
        // ...and the word fold is untouched by the comment fold sharing its pass.
        assert_eq!(rows[0].item.total_words, 30);
    }

    #[test]
    fn comment_siblings_do_not_leak_into_each_other() {
        let mut rows = vec![r(1, Some(1), 0), r(2, Some(1), 1), r(3, Some(1), 0)];
        rows[0].item.own_comments = 0;
        rows[1].item.own_comments = 7;
        rows[2].item.own_comments = 2;
        fold_totals(&mut rows);
        assert_eq!(rows[0].item.total_comments, 7);
        assert_eq!(
            rows[2].item.total_comments, 2,
            "the sibling keeps only its own"
        );
    }

    #[test]
    fn folding_an_empty_set_is_a_noop() {
        let mut rows: Vec<TreeRow<Uuid, OverviewRow>> = Vec::new();
        fold_totals(&mut rows);
        assert!(rows.is_empty());
    }

    /// Siblings must not absorb each other: two rows at the same depth are independent
    /// subtrees, however deep the one before them went.
    #[test]
    fn siblings_do_not_leak_into_each_other() {
        let mut rows = vec![
            r(1, Some(1), 0),
            r(2, Some(2), 1),
            r(3, Some(4), 2),
            r(4, Some(8), 0), // a fresh sibling of row 1
        ];
        fold_totals(&mut rows);
        assert_eq!(rows[0].item.total_words, 7, "1 + 2 + 4");
        assert_eq!(
            rows[3].item.total_words, 8,
            "the sibling keeps only its own"
        );
    }

    // ── the subtree slice ────────────────────────────────────────────────────

    /// `(binder, id, indent)` stand-ins for the flat item stream.
    fn flat() -> Vec<(u64, u64, i64)> {
        vec![
            (1, 100, 0), // Book
            (1, 1, 1),   // Part
            (1, 2, 2),   // Chapter folder
            (1, 3, 3),   // Scene
            (1, 4, 3),   // Scene
            (1, 5, 2),   // flat Chapter
            (1, 6, 1),   // a second Part, OUTSIDE part 1
        ]
    }

    fn slice_of(flat: &[(u64, u64, i64)], container: u64) -> Subtree<'_, (u64, u64, i64)> {
        subtree_of(flat, container, |t| t.1, |t| t.2, |t| t.0)
    }

    fn slice_ids(container: u64) -> Vec<u64> {
        let f = flat();
        match slice_of(&f, container) {
            Subtree::Found { rows, .. } => rows.iter().map(|t| t.1).collect(),
            Subtree::ContainerGone => panic!("container {container} should be present"),
        }
    }

    #[test]
    fn the_slice_is_the_container_subtree_without_the_container() {
        assert_eq!(slice_ids(1), vec![2, 3, 4, 5], "part 1's descendants only");
        assert_eq!(slice_ids(2), vec![3, 4], "the chapter folder's scenes");
        assert_eq!(
            slice_ids(100),
            vec![1, 2, 3, 4, 5, 6],
            "the book holds everything"
        );
    }

    #[test]
    fn the_slice_stops_at_the_first_sibling() {
        // Part 6 follows part 1 at the same indent — it must not be swept in.
        assert!(!slice_ids(1).contains(&6));
    }

    #[test]
    fn a_childless_container_slices_to_nothing_but_is_still_present() {
        // The last part has no children — present, just empty.
        let f = flat();
        match slice_of(&f, 6) {
            Subtree::Found { rows, .. } => assert!(rows.is_empty()),
            Subtree::ContainerGone => panic!("part 6 is in the stream"),
        }
    }

    /// "Gone" and "empty" are different answers, and the caller shows different things
    /// for them — an empty container offers a create button, a vanished one must not.
    #[test]
    fn an_absent_container_is_reported_as_gone_not_empty() {
        let f = flat();
        assert!(matches!(slice_of(&f, 999), Subtree::ContainerGone));
    }

    /// **The binder guard.** Indents are scoped per binder and the flat stream carries no
    /// separator, so a container that is the last item of its binder would otherwise walk
    /// straight into the next binder's items and adopt any that start deeper than it —
    /// folding their word counts into its total.
    #[test]
    fn the_slice_stops_at_a_binder_boundary() {
        let f = vec![
            (1, 10, 0), // binder 1: a Part …
            (1, 11, 1), // … and its scene
            (2, 20, 1), // binder 2 opens at indent 1 — DEEPER than the Part
            (2, 21, 2),
        ];
        match slice_of(&f, 10) {
            Subtree::Found { rows, .. } => {
                let ids: Vec<u64> = rows.iter().map(|t| t.1).collect();
                assert_eq!(
                    ids,
                    vec![11],
                    "binder 2's items must not be adopted as the Part's children"
                );
            }
            Subtree::ContainerGone => panic!("the Part is in the stream"),
        }
    }

    /// Depths are rebased so the container's direct children sit at 0 — the container's
    /// own row is the tab, not a row of the table.
    #[test]
    fn the_slice_reports_the_base_indent_to_rebase_from() {
        let f = flat();
        let Subtree::Found { rows, base } = slice_of(&f, 1) else {
            panic!("part 1 is present");
        };
        assert_eq!(base, 1, "part 1 sits at indent 1");
        let depths: Vec<i64> = rows.iter().map(|t| t.2 - base - 1).collect();
        assert_eq!(
            depths,
            vec![0, 1, 1, 0],
            "chapter, its 2 scenes, flat chapter"
        );
    }

    // ── sort comparators ─────────────────────────────────────────────────────

    fn row_for(title: &str, label: &str, own: Option<usize>, total: usize) -> OverviewRow {
        OverviewRow {
            title: title.to_string(),
            label: label.to_string(),
            own_words: own,
            total_words: total,
            ..Default::default()
        }
    }

    #[test]
    fn title_sorts_case_insensitively() {
        let cmp = comparator(COL_TITLE);
        assert_eq!(
            cmp(
                &row_for("apple", "", None, 0),
                &row_for("Banana", "", None, 0)
            ),
            std::cmp::Ordering::Less,
            "lowercase 'apple' must precede 'Banana' — a raw byte compare would not"
        );
    }

    #[test]
    fn no_prose_sorts_below_zero_prose() {
        let cmp = comparator(COL_OWN_WORDS);
        assert_eq!(
            cmp(&row_for("a", "", None, 0), &row_for("b", "", Some(0), 0)),
            std::cmp::Ordering::Less,
            "'nothing to count' is less than 'counted, and it was zero'"
        );
    }

    #[test]
    fn totals_sort_numerically_not_lexically() {
        let cmp = comparator(COL_TOTAL_WORDS);
        assert_eq!(
            cmp(&row_for("a", "", None, 9), &row_for("b", "", None, 100)),
            std::cmp::Ordering::Less,
            "9 < 100 — a string sort would put 100 first"
        );
    }

    /// Type sorts by structural nesting, not by the label's alphabet.
    #[test]
    fn type_sorts_outermost_first() {
        use BinderItemRole::{Folder, Item};
        use BinderItemSubRole::{Book, ChapterScene, Part, Scene};
        let mk = |role: BinderItemRole, sub_role: BinderItemSubRole| OverviewRow {
            role,
            sub_role,
            ..Default::default()
        };
        let cmp = comparator(COL_TYPE);
        assert_eq!(
            cmp(&mk(Folder, Book), &mk(Folder, Part)),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            cmp(&mk(Folder, Part), &mk(Folder, ChapterScene)),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            cmp(&mk(Folder, ChapterScene), &mk(Item, Scene)),
            std::cmp::Ordering::Less,
            "a chapter groups above the scenes inside it"
        );
        // ...which the alphabet would get wrong: "Chapter" < "Part" < "Scene" would put
        // a chapter before its part.
        assert_eq!(
            cmp(&mk(Item, Scene), &mk(Folder, Part)),
            std::cmp::Ordering::Greater
        );
    }

    /// A sort naming a column that no longer exists leaves manuscript order alone rather
    /// than panicking or inventing one.
    #[test]
    fn an_unknown_sort_column_is_inert() {
        let cmp = comparator("no-such-column");
        assert_eq!(
            cmp(&row_for("z", "", None, 0), &row_for("a", "", None, 0)),
            std::cmp::Ordering::Equal
        );
    }
}

#[cfg(all(test, feature = "mocks"))]
mod mock_tests {
    use super::*;
    use frontend::AppContext;

    /// The visible rows in display order. A test helper, not model API — nothing in the
    /// app asks for the whole row set (the view reads rows one at a time through
    /// `with_entry`, and the view-model resolves individual rows by uid).
    fn visible_rows(m: &OverviewRowsModel) -> Vec<OverviewRow> {
        (0..m.visible_count())
            .filter_map(|i| m.with_entry(i, |r, _| r.clone()))
            .collect()
    }

    /// A loaded model. The explicit `reload()` stands in for `wire()`, which is what
    /// performs the first load in the app — the constructor deliberately does not, so a
    /// container tab does not pay for a pane it may never show.
    fn model(container: u64) -> OverviewRowsModel {
        let m = OverviewRowsModel::new(
            Rc::new(AppContext::new()),
            Signal::new(Some(1)),
            container,
            Signal::new(CountingMethodSetting::default()),
            OverviewFilters::new(),
            Signal::new(GoalUnit::default()),
        );
        m.reload();
        m
    }

    /// The same, but keeping the filters so a test can drive search / sort.
    pub(super) fn model_with(container: u64, filters: OverviewFilters) -> OverviewRowsModel {
        let m = OverviewRowsModel::new(
            Rc::new(AppContext::new()),
            Signal::new(Some(1)),
            container,
            Signal::new(CountingMethodSetting::default()),
            filters,
            Signal::new(GoalUnit::default()),
        );
        m.reload();
        m
    }

    /// Every row of the fixture subtree shows, fully expanded, with the container's own
    /// row absent.
    #[test]
    fn a_container_shows_its_whole_subtree() {
        let m = model(101);
        assert_eq!(m.visible_count(), 10);
        assert!(
            !visible_rows(&m).iter().any(|r| r.item_id == 101),
            "the container itself is the tab, not a row of its own table"
        );
    }

    #[test]
    fn an_unknown_container_shows_an_empty_table() {
        assert_eq!(model(999).visible_count(), 0);
    }

    /// Collapsing a container hides its subtree but keeps its own row — and the total it
    /// shows still counts what is now hidden.
    #[test]
    fn collapsing_hides_the_subtree_but_not_the_total() {
        let m = model(101);
        let chapter = common::uid::fixture_uid(104);
        let before = m.row_of(&chapter).expect("the chapter folder is a row");
        assert_eq!(
            before.total_words,
            90 + 1180 + 640,
            "own prose plus both scenes (the Note carries no scene prose)"
        );
        m.set_expanded(&chapter, false);
        assert_eq!(m.visible_count(), 7, "its 3 children are hidden");
        assert_eq!(
            m.row_of(&chapter).unwrap().total_words,
            before.total_words,
            "a collapsed container still reports its subtree's words"
        );
    }

    /// Search keeps a match's ancestors so it stays reachable, and clearing restores the
    /// collapse state the writer had — the reveal is an override, not a mutation.
    #[test]
    fn search_keeps_ancestors_and_restores_collapse_on_clear() {
        let filters = OverviewFilters::new();
        let m = model_with(101, filters.clone());
        // Collapse the part before searching, so there is a state to restore.
        let part = common::uid::fixture_uid(301);
        m.set_expanded(&part, false);
        let collapsed_count = m.visible_count();
        assert!(collapsed_count < 10);

        // "beat" is Scene 1's label — two levels down, under a *collapsed* part. It
        // must still be reachable.
        filters.query.set("beat".to_string());
        let rows = visible_rows(&m);
        assert!(
            rows.iter().any(|r| r.item_id == 201),
            "the match itself shows"
        );
        assert!(
            rows.iter().any(|r| r.item_id == 301) && rows.iter().any(|r| r.item_id == 104),
            "its part and chapter survive as ancestors, revealed despite the collapse"
        );

        filters.query.set(String::new());
        assert_eq!(
            m.visible_count(),
            collapsed_count,
            "clearing the search restores the writer's collapse exactly"
        );
    }

    /// Capturing the expanded set mid-search must report the *real* state, not the
    /// reveal override — otherwise persistence would save "everything expanded" every
    /// time the writer quit with a search box still full.
    #[test]
    fn expanded_uids_ignore_the_search_reveal() {
        let filters = OverviewFilters::new();
        let m = model_with(101, filters.clone());
        let part = common::uid::fixture_uid(301);
        m.set_expanded(&part, false);
        // Read the slice directly: `expanded_keys()` is what expand-state persistence
        // will call, and the point is that it reports the REAL set even while the reveal
        // override is on. (No model accessor wraps it yet — A2 adds one when it has a
        // caller.)
        let quiet = m.slice.expanded_keys();
        filters.query.set("beat".to_string());
        assert!(
            !m.slice.expanded_keys().contains(&part),
            "the reveal override must not leak into the captured set"
        );
        assert_eq!(m.slice.expanded_keys().len(), quiet.len());
    }

    /// **An expanded row that a search filters out must come back expanded.**
    ///
    /// `TreeDataSlice::build` drops every expand key absent from the incoming rows, and a
    /// search narrows the rows — so without an authoritative set held outside the slice,
    /// expanding a chapter and then searching for something that excludes it silently
    /// collapses it. Found by probing after an earlier "simplification" removed the
    /// snapshot the outline uses; the reveal override is non-destructive, but the source
    /// filtering it runs alongside is not.
    #[test]
    fn an_expanded_row_filtered_out_by_a_search_comes_back_expanded() {
        let filters = OverviewFilters::new();
        let m = model_with(101, filters.clone());
        let chapter = common::uid::fixture_uid(104);
        assert!(m.is_expanded(&chapter), "the chapter starts expanded");

        // "Confrontation" matches row 105 only — the chapter is filtered out entirely.
        filters.query.set("Confrontation".to_string());
        filters.query.set(String::new());

        assert!(
            m.is_expanded(&chapter),
            "the chapter was pruned by the search's row filter and never restored"
        );
    }

    /// The converse: a row the writer collapsed must NOT come back expanded after a
    /// search filters it out and returns it — the remembered set has to forget on
    /// collapse, not just remember on expand.
    #[test]
    fn a_collapsed_row_filtered_out_by_a_search_comes_back_collapsed() {
        let filters = OverviewFilters::new();
        let m = model_with(101, filters.clone());
        let chapter = common::uid::fixture_uid(104);
        m.set_expanded(&chapter, false);

        filters.query.set("Confrontation".to_string());
        filters.query.set(String::new());

        assert!(!m.is_expanded(&chapter));
    }

    /// A collapse made **while searching** must survive clearing the box.
    ///
    /// The reveal override is a display mode, not a mutation, so a chevron the writer
    /// clicks during a search writes the real expand set and must stay written. An
    /// earlier version snapshotted on entry and restored on exit, which silently undid
    /// exactly this.
    #[test]
    fn a_collapse_made_during_a_search_survives_clearing_it() {
        let filters = OverviewFilters::new();
        let m = model_with(101, filters.clone());
        let chapter = common::uid::fixture_uid(104);
        assert!(m.is_expanded(&chapter), "starts expanded");

        filters.query.set("scene".to_string());
        m.set_expanded(&chapter, false); // the writer collapses it mid-search
        filters.query.set(String::new());

        assert!(
            !m.is_expanded(&chapter),
            "clearing the search must not resurrect a collapse the writer made during it"
        );
    }

    #[test]
    fn a_search_matching_nothing_empties_the_table() {
        let filters = OverviewFilters::new();
        let m = model_with(101, filters.clone());
        filters.query.set("zzz-nothing".to_string());
        assert_eq!(m.visible_count(), 0);
    }

    /// Sorting reorders rows **within each parent** and never flattens the tree: after
    /// sorting by title, every row still sits under the same parent.
    #[test]
    fn sorting_is_per_sibling_and_preserves_the_hierarchy() {
        let filters = OverviewFilters::new();
        let m = model_with(101, filters.clone());
        let scene1 = common::uid::fixture_uid(201);
        let chapter = common::uid::fixture_uid(104);
        assert_eq!(m.parent(&scene1), Some(chapter));

        filters
            .sort
            .set(Some((COL_TITLE.to_string(), SortDirection::Descending)));
        assert_eq!(m.visible_count(), 10, "sorting hides nothing");
        assert_eq!(
            m.parent(&scene1),
            Some(chapter),
            "a scene stays inside its chapter however the table is sorted"
        );
    }

    /// Sorting must not disturb expand state — it is not a search.
    #[test]
    fn sorting_leaves_collapse_state_alone() {
        let filters = OverviewFilters::new();
        let m = model_with(101, filters.clone());
        let chapter = common::uid::fixture_uid(104);
        m.set_expanded(&chapter, false);
        let collapsed = m.visible_count();
        filters.sort.set(Some((
            COL_TOTAL_WORDS.to_string(),
            SortDirection::Ascending,
        )));
        assert_eq!(
            m.visible_count(),
            collapsed,
            "the collapsed chapter stays collapsed through a sort"
        );
    }

    /// Rows are keyed by durable uid, so a full re-source (what every backend event
    /// triggers) leaves expand state intact — the property store ids cannot offer.
    #[test]
    fn expand_state_survives_a_full_resource() {
        let m = model(101);
        let chapter = common::uid::fixture_uid(104);
        m.set_expanded(&chapter, false);
        let before = m.visible_count();
        m.reload();
        assert_eq!(
            m.visible_count(),
            before,
            "re-sourcing must not silently re-expand the tree"
        );
        assert!(!m.is_expanded(&chapter));
    }

    /// Every row is draggable, and a drop *into* a leaf redirects to *after* it (a Scene
    /// has nowhere to put a child).
    #[test]
    fn drag_policy_matches_the_outline() {
        use teksilo::data::DragSource;
        let m = model(101);
        let scene = common::uid::fixture_uid(201);
        let other = common::uid::fixture_uid(202);
        assert_eq!(m.drag(&scene), DragEligibility::CanDrag);

        let into_leaf = DropQuery {
            source: DragSource::SameView { key: scene },
            target: other,
            position: DropPosition::Into,
        };
        assert_eq!(
            m.can_accept(&into_leaf),
            DropResponse::Redirect(DropPosition::After)
        );

        let into_folder = DropQuery {
            source: DragSource::SameView { key: scene },
            target: common::uid::fixture_uid(301), // the Part folder
            position: DropPosition::Into,
        };
        assert_eq!(m.can_accept(&into_folder), DropResponse::Accept);
    }

    /// The cycle guard: a folder cannot be dropped inside its own subtree.
    #[test]
    fn a_folder_cannot_be_dropped_into_itself() {
        use teksilo::data::DragSource;
        let m = model(101);
        let q = DropQuery {
            source: DragSource::SameView {
                key: common::uid::fixture_uid(301), // the Part
            },
            target: common::uid::fixture_uid(201), // a scene two levels inside it
            position: DropPosition::Into,
        };
        assert_eq!(m.can_accept(&q), DropResponse::Reject);
    }

    #[test]
    fn uid_resolves_to_the_live_store_id() {
        let m = model(101);
        assert_eq!(m.item_id_of(&common::uid::fixture_uid(201)), Some(201));
        assert_eq!(m.item_id_of(&common::uid::fixture_uid(9999)), None);
    }
}
