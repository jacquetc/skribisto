// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive, ordered list of the rows belonging to one **container** — the data
//! behind the Full Chapter / Full Part / Full Book streams and their Full Synopsis
//! twins.
//!
//! Per the writing model, a container's extent is a run of the flat, ordered,
//! *activated* item stream: everything after the container head, up to the first
//! item that closes it. What closes it depends on the container's **level** — a
//! chapter ends at the next chapter/part/book boundary, a part ends only at the
//! next part/book (chapters live *inside* it), a book ends only at the next book or
//! at a `BookEnd`. Inside that extent, a **row** is any item that carries scene
//! prose or opens a chapter or a part; notes, plain folders, `Item/Text` and
//! `Item/BookEnd` are not rows. All the predicates come from
//! [`skribisto_model::SubRoleExt`] (single source of truth); folder nesting /
//! `indent` is deliberately ignored — containment is UI-only.
//!
//! The container **head is never a row**: it is the pane header, and its own
//! content (a chapter folder's prose, any container's synopsis) is rendered as the
//! pane's own section. Only folder containers host a stream ([`StreamLevel::for_container`]),
//! so the head is always a `Folder` — which matters, because a chapter folder now
//! `carries_scene()` just like the flat `Item/ChapterScene` it promotes to.
//!
//! Owns a [`teksilo::data::ListModel<StreamRow>`] for the `Repeater`, updated by a
//! **keyed diff** (remove / insert / move / in-place update) so the reconciling
//! `Repeater` reuses each row's mounted editor across structural edits. Two
//! `#[cfg]`-gated `mod imp` variants share one public surface (see
//! [`crate::models`]).

use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
// The stream-folding primitives now live in `skribisto_model::compile` (one definition,
// shared with the backend exporter). Re-exported so this module's public surface and its
// callers (`stream`, `tabs::shared::stream`) are unchanged. Under `mocks` the
// real `imp` (the only `row_indices` caller) is cfg'd out, so allow the unused re-export
// there — exactly as the lifted functions carried `allow(dead_code)` before.
#[cfg_attr(feature = "mocks", allow(unused_imports))]
pub use skribisto_model::compile::{StreamLevel, row_indices};

/// One row in a stream. Carries the full `(role, sub_role)` — the view needs it to
/// pick the row's chrome (part heading / chapter heading / scene header) and to ask
/// the constraint matrix which editors the row gets; the view-model needs it to gate
/// merge and split.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StreamRow {
    pub item_id: u64,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    /// The chapter/part ordinal this row carries in the book, or `None` for a scene.
    ///
    /// The stream is the manuscript as it reads, so this is the closest thing in the app
    /// to the exported heading — and it is computed by the same function the exporter
    /// calls, over the same whole-manuscript stream, so the two agree by construction.
    pub number: Option<usize>,
    /// What to call this row when it has no title of its own — "Chapter 3", localized.
    /// See `crate::models::label_and_badge` for the rule; `title` stays the writer's own
    /// string, because that is what renames seed from and what search matches.
    pub fallback_label: Option<String>,
    /// How the row reads when the stream is read **as exported**, or `None` in the
    /// ordinary working stream. See [`ReadingRow`].
    pub reading: Option<ReadingRow>,
}

/// What a row of a stream read as exported shows: the part of the compiler's
/// [`RowPlan`](skribisto_compiler::plan::RowPlan) a screen can show faithfully.
///
/// The compiler decides it, through
/// [`reading_plan`](skribisto_compiler::reading_plan), over the same scope an export of the
/// container would sweep, so the reading and the file agree about which rows are in the
/// book, what heads each one and where its epigraph sits.
///
/// Deliberately **not** the plan's `prose` flag. That one follows whether the stored text is
/// empty, and a row's data changing rebuilds the row: a writer clearing a scene in this view
/// would lose their caret the moment the clearing was saved. A row with prose of its own
/// shows its editor whatever it holds; an empty one prints nothing in the file and shows
/// nothing here either. The epigraph and the synopsis follow the same rule once shown:
/// clearing one keeps its editor until the reading is next switched on (see
/// `keep_shown_fields`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadingRow {
    /// The file opens a new page in front of this row. Where the break *lands*, which is not
    /// always the row that asks for it (see
    /// [`PlannedRow::opens_page`](skribisto_compiler::PlannedRow::opens_page)).
    pub starts_page: bool,
    /// The first row of the reading, which has nothing above it to break away from.
    pub first: bool,
    /// Where the row's epigraph prints, or `None` when it does not.
    pub epigraph: Option<skribisto_compiler::EpigraphPlacement>,
    /// The heading the row opens on, worded as the export words it.
    pub heading: Option<skribisto_compiler::plan::RowHeading>,
    /// The style prints the row's synopsis.
    pub synopsis: bool,
}

impl ReadingRow {
    fn from_plan(planned: &skribisto_compiler::PlannedRow, first: bool) -> Self {
        Self {
            starts_page: planned.opens_page,
            first,
            epigraph: planned.plan.epigraph,
            heading: planned.plan.heading.clone(),
            synopsis: planned.plan.synopsis,
        }
    }

    /// Keep an epigraph or a synopsis editor that `was` showing while the style still prints
    /// that kind of field, even though its text has just been cleared.
    ///
    /// Without this, deleting the last character of an epigraph in a reading would rebuild
    /// the row and take the editor, caret and all, out from under the writer the moment the
    /// edit was saved.
    #[cfg_attr(feature = "mocks", allow(dead_code))]
    fn keep_shown_fields(&mut self, was: &ReadingRow, style: &skribisto_compiler::Preset) {
        if self.epigraph.is_none() && was.epigraph.is_some() && style.include_epigraphs {
            self.epigraph = Some(style.epigraph_placement);
        }
        if !self.synopsis && was.synopsis && style.include_synopses {
            self.synopsis = true;
        }
    }
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use frontend::common::event::{
        BinderItemManagementEvent, DirectAccessEntity, EntityEvent, Event, Origin,
        TrashManagementEvent, WorkManagementEvent,
    };

    use skribisto_compiler::Preset;
    use skribisto_model::compile::{ScopeKind, resolve_scope};

    use super::{ReadingRow, StreamLevel, StreamRow, row_indices};

    /// Called with the ids of rows that left the stream, so the owner can release
    /// their shared documents. Stored (not one-shot) — it fires from every refresh.
    type OnRemoved = Box<dyn Fn(&[u64])>;

    struct Inner {
        model: ListModel<StreamRow>,
        ctx: Rc<AppContext>,
        work_id: Signal<Option<u64>>,
        head_id: u64,
        level: StreamLevel,
        on_removed: RefCell<Option<OnRemoved>>,
        /// The style the stream is read in, or `None` for the ordinary working stream.
        reading: RefCell<Option<Preset>>,

        /// How the container itself reads, when the stream is read as exported. See
        /// [`StreamRowsModel::head_reading`].
        head_reading: Signal<Option<ReadingRow>>,
    }

    #[derive(Clone)]
    pub struct StreamRowsModel {
        inner: Rc<Inner>,
    }

    impl StreamRowsModel {
        /// `level` is resolved once by the caller (via [`StreamLevel::for_container`]),
        /// so this model stays a total function over a container its caller already
        /// validated.
        pub fn new(
            ctx: Rc<AppContext>,
            work_id: Signal<Option<u64>>,
            head_id: u64,
            level: StreamLevel,
        ) -> Self {
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::new(),
                    ctx,
                    work_id,
                    head_id,
                    level,
                    on_removed: RefCell::new(None),
                    reading: RefCell::new(None),
                    head_reading: Signal::new(None),
                }),
            }
        }

        /// The `Repeater`'s data source.
        pub fn list(&self) -> ListModel<StreamRow> {
            self.inner.model.clone()
        }

        /// Current ordered rows — for the view-model's synchronous prev/next gating
        /// (merge/split), which needs `(role, sub_role)` and not just ids.
        pub fn rows(&self) -> Vec<StreamRow> {
            let m = &self.inner.model;
            (0..m.len())
                .filter_map(|i| m.with_item(i, |r| r.clone()))
                .collect()
        }

        /// Current ordered row ids.
        ///
        /// Reads the ids out of the model rather than going through
        /// [`rows`](Self::rows), which clones every row whole — a label `String` per
        /// row included. That is wasted work for a list of `u64`s, and this one is asked
        /// once a frame while a stream's find banner is open, over a Book that can be
        /// hundreds of rows long.
        pub fn ids(&self) -> Vec<u64> {
            let m = &self.inner.model;
            (0..m.len())
                .filter_map(|i| m.with_item(i, |r| r.item_id))
                .collect()
        }

        /// How the container itself reads when the stream is read as exported: its heading,
        /// its epigraph and its synopsis, decided by the same plan as every row. `None` in
        /// the working stream, and when the export leaves the container out (its own
        /// *Include in exports* is off), which is also what hides its own prose.
        ///
        /// The head is never a row, here as in the working stream. Its own prose already
        /// has an editor at the top of the page; a second one on the same document would
        /// put two extents for one item on the margin lane.
        pub fn head_reading(&self) -> Signal<Option<ReadingRow>> {
            self.inner.head_reading.clone()
        }

        /// Read the stream as exported in `style`, or (`None`) as the ordinary working
        /// stream, and refill it.
        pub fn set_reading(&self, style: Option<Preset>) {
            if *self.inner.reading.borrow() == style {
                return;
            }
            *self.inner.reading.borrow_mut() = style;
            self.refresh();
        }

        /// Refill now, as an event would. For tests, which build no widget to wire.
        #[cfg(test)]
        pub(crate) fn refill(&self) {
            self.refresh();
        }

        /// Subscribe once to structural events and do an initial fill. `on_removed`
        /// is invoked (from this and every later refresh) with the ids that left the
        /// stream, so the owner can release their shared documents.
        ///
        /// **`on_removed` must not capture its owner strongly.** It is stored for the
        /// model's lifetime, and the model is itself owned by that owner — an `Rc`
        /// capture would close a cycle, the owner's `Drop` would never run, and every
        /// document the stream ever opened would leak. `StreamViewModel::wire` passes
        /// a `Weak`-capturing closure.
        ///
        /// `Promote` rewrites a row's `(role, sub_role)` in place, which `reconcile` turns
        /// into an in-place `set`. A rename (`BinderItem::Updated`) is watched too, for
        /// the one thing about a title a row bakes in: see [`working_origins`].
        pub fn wire(&self, ctx: &mut BuildContext, on_removed: impl Fn(&[u64]) + 'static) {
            *self.inner.on_removed.borrow_mut() = Some(Box::new(on_removed));
            // Subscribe on **every** build. `BuildContext::subscribe_event` scopes a
            // subscription to the widget's current build and drops it on the next one, so
            // a "subscribe once" guard would make this model go deaf the first time its
            // host widget rebuilt. Re-subscribing cannot duplicate: the old callbacks are
            // already gone.
            {
                use EntityEvent::{Created, Removed, Updated};
                for origin in working_origins() {
                    let me = self.clone();
                    ctx.subscribe_event(origin, move |_event: &Event| me.refresh());
                }
                // Read as exported, a row's own fields decide whether it is in the book at
                // all: "Include in exports", its title (the heading), whether its epigraph
                // or synopsis holds text. The working stream already refills on a
                // `BinderItem` update (see `working_origins`); the content ones are not
                // structural, so it rightly ignores them, and nothing else here would
                // notice. The refill is a no-op diff when nothing a reading shows has moved.
                let reading_only = [
                    Origin::DirectAccess(DirectAccessEntity::Content(Created)),
                    Origin::DirectAccess(DirectAccessEntity::Content(Updated)),
                    Origin::DirectAccess(DirectAccessEntity::Content(Removed)),
                ];
                for origin in reading_only {
                    let me = self.clone();
                    ctx.subscribe_event(origin, move |_event: &Event| {
                        if me.inner.reading.borrow().is_some() {
                            me.refresh();
                        }
                    });
                }
                // Project (re)load — guarded (loose form): `refresh` always
                // re-derives from this model's own `work_id`, so a sibling Work's
                // Load/New would only cost a harmless, still-correct re-derive;
                // guarded anyway so opening a second Work doesn't force a wasted
                // rebuild of every other open window's stream.
                for wev in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
                    let me = self.clone();
                    let work_id = self.inner.work_id.clone();
                    ctx.subscribe_event(Origin::WorkManagement(wev), move |event: &Event| {
                        let mine = work_id.get();
                        if mine.is_none() || event.ids.first() == mine.as_ref() {
                            me.refresh();
                        }
                    });
                }
            }
            self.refresh();
        }

        fn refresh(&self) {
            let reading = self.inner.reading.borrow().clone();
            let (next, head) = match &reading {
                Some(style) => query_as_exported(
                    &self.inner.ctx,
                    self.inner.work_id.get(),
                    self.inner.head_id,
                    self.inner.level,
                    style,
                ),
                None => (
                    query(
                        &self.inner.ctx,
                        self.inner.work_id.get(),
                        self.inner.head_id,
                        self.inner.level,
                    ),
                    None,
                ),
            };
            // A field shown a moment ago stays shown while its text is cleared: see
            // `ReadingRow::keep_shown_fields`.
            let (next, head) = match &reading {
                Some(style) => keep_shown(
                    &self.inner.model,
                    next,
                    head,
                    &self.inner.head_reading,
                    style,
                ),
                None => (next, head),
            };
            self.inner.head_reading.set_if_changed(head);
            let removed = reconcile(&self.inner.model, next);
            if !removed.is_empty() {
                // Take the callback out of the RefCell before invoking it: it calls
                // back into the owner, which may re-enter this model.
                let cb = self.inner.on_removed.borrow_mut().take();
                if let Some(cb) = cb {
                    cb(&removed);
                    *self.inner.on_removed.borrow_mut() = Some(cb);
                }
            }
        }
    }

    /// Every backend event the stream refills on, read as exported or not.
    ///
    /// A `BinderItem` update is among them for an untitled chapter or part. Its header
    /// shows the name the stream generates for it ("Chapter 3"), baked into its
    /// [`StreamRow::fallback_label`] because a generated name has no live source to
    /// follow; a titled row shows the live title instead. Renaming the untitled row
    /// left the baked name on screen, in the stream only, until something else refilled
    /// the list; clearing a title left a blank heading the same way. The refill is a
    /// no-op diff for every other update (a titled row's rename, a label, a status), so
    /// the rows and their editors stay put: only a row whose generated name came or went
    /// is rebuilt. The binder, the Overview and the corkboard refill on it already.
    pub(super) fn working_origins() -> [Origin; 12] {
        use DirectAccessEntity::BinderItem;
        use EntityEvent::{Created, Removed, Updated};
        [
            Origin::DirectAccess(BinderItem(Created)),
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
            Origin::TrashManagement(TrashManagementEvent::RestoreItems),
            Origin::TrashManagement(TrashManagementEvent::EmptyTrash),
        ]
    }

    /// The container's ordered rows, per the extent + row rules.
    fn query(
        ctx: &AppContext,
        work_id: Option<u64>,
        head_id: u64,
        level: StreamLevel,
    ) -> Vec<StreamRow> {
        let Some(work_id) = work_id else {
            return Vec::new();
        };
        let binder_ids_all =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();
        for binder_id in binder_ids_all.iter() {
            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            let flat: Vec<_> = binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .filter(|it| it.activated)
                .collect();
            if let Some(pos) = flat.iter().position(|it| it.id == head_id) {
                let sub_roles: Vec<BinderItemSubRole> =
                    flat.iter().map(|it| it.sub_role.clone()).collect();
                // Numbered over the **whole Work**, not this binder — `flat` above is one
                // binder's rows, and a manuscript whose chapters span two binders would
                // otherwise restart at one here while the outline and the exported file
                // (both numbered over the concatenated stream) kept counting. Same rule,
                // one source: `crate::models::numbers_for_items`.
                let numbers = {
                    let mut all = Vec::new();
                    for bid in &binder_ids_all {
                        let ids = binder_commands::get_binder_relationship(
                            ctx,
                            bid,
                            &BinderRelationshipField::BinderItems,
                        )
                        .unwrap_or_default();
                        let by_id: std::collections::HashMap<u64, _> =
                            binder_item_commands::get_binder_item_multi(ctx, &ids)
                                .unwrap_or_default()
                                .into_iter()
                                .flatten()
                                .map(|it| (it.id, it))
                                .collect();
                        all.extend(ids.into_iter().filter_map(|id| by_id.get(&id).cloned()));
                    }
                    crate::models::numbers_for_items(ctx, work_id, &all)
                };
                let work_langs = crate::models::work_language_tags(ctx, work_id);
                return row_indices(&sub_roles, pos, level)
                    .into_iter()
                    .map(|i| StreamRow {
                        item_id: flat[i].id,
                        role: flat[i].role.clone(),
                        sub_role: flat[i].sub_role.clone(),
                        number: numbers
                            .get(&flat[i].id)
                            .map(skribisto_model::numbering::Numbered::number),
                        fallback_label: crate::models::fallback_label_for(
                            &flat[i],
                            numbers.get(&flat[i].id),
                            &work_langs,
                        ),
                        reading: None,
                    })
                    .collect();
            }
        }
        Vec::new()
    }

    /// The container read as exported in `style`: the rows an export of it would print,
    /// in the order it prints them.
    ///
    /// Also returns how the container head itself reads (see
    /// [`StreamRowsModel::head_reading`]): a chapter folder opens on its own heading and
    /// epigraph, and those come from the same plan as every row below it.
    ///
    /// Scope and plan both come from the export's own functions: the extent is
    /// [`resolve_scope`] over the whole Work's stream (so "Include in exports" and the
    /// binder's edge cut it exactly as they cut a file), and each row is what
    /// [`skribisto_compiler::reading_plan`] says it prints. A row the style leaves out
    /// is absent, and one that prints nothing (a front-matter folder, a book opener under a
    /// title page) is skipped unless it has prose of its own to write into.
    fn query_as_exported(
        ctx: &AppContext,
        work_id: Option<u64>,
        head_id: u64,
        level: StreamLevel,
        style: &Preset,
    ) -> (Vec<StreamRow>, Option<ReadingRow>) {
        let Some(work_id) = work_id else {
            return (Vec::new(), None);
        };
        let gathered = match crate::export::gather::gather_work(ctx, work_id, false) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("stream: could not read the project to show it as exported: {e:#}");
                return (Vec::new(), None);
            }
        };
        let metas = skribisto_compiler::item_metas(&gathered);
        let Some(head) = metas.iter().position(|m| m.id == head_id) else {
            return (Vec::new(), None);
        };
        let scope = match level {
            StreamLevel::Chapter => ScopeKind::Chapter,
            StreamLevel::Part => ScopeKind::Part,
            StreamLevel::Book => ScopeKind::Book,
        };
        let include = resolve_scope(&metas, head, scope).unwrap_or_default();
        // The export resolves the book's language the same way, at its own boundary.
        let work_lang = skribisto_model::language::primary(&gathered.work.dict_language);
        let kinds: HashMap<u64, (BinderItemRole, BinderItemSubRole)> = metas
            .iter()
            .map(|m| (m.id, (m.role.clone(), m.sub_role.clone())))
            .collect();
        let mut rows = Vec::new();
        let mut head_reading = None;
        // Whether the container itself puts anything on the page above the rows: a heading
        // or an epigraph, or its own prose editor. A row's page rule needs something above.
        let mut head_shows = false;
        for planned in
            skribisto_compiler::reading_plan(&gathered, &include, style, work_lang, false)
        {
            if planned.item_id == head_id {
                head_shows = planned.plan.prints_anything()
                    || kinds
                        .get(&head_id)
                        .is_some_and(|(role, sub_role)| has_own_prose(role, sub_role));
                head_reading = Some(ReadingRow::from_plan(&planned, true));
                continue;
            }
            let Some((role, sub_role)) = kinds.get(&planned.item_id) else {
                continue;
            };
            if !planned.plan.prints_anything() && !has_own_prose(role, sub_role) {
                continue;
            }
            rows.push(StreamRow {
                item_id: planned.item_id,
                role: role.clone(),
                sub_role: sub_role.clone(),
                // The heading carries the words the export prints; the working stream's
                // badge and generated name have nothing to add to it here.
                number: None,
                fallback_label: None,
                reading: Some(ReadingRow::from_plan(
                    &planned,
                    rows.is_empty() && !head_shows,
                )),
            });
        }
        (rows, head_reading)
    }

    /// Apply [`ReadingRow::keep_shown_fields`] to every row, and the head, against what the
    /// reading shows now.
    fn keep_shown(
        model: &ListModel<StreamRow>,
        mut next: Vec<StreamRow>,
        mut head: Option<ReadingRow>,
        current_head: &Signal<Option<ReadingRow>>,
        style: &Preset,
    ) -> (Vec<StreamRow>, Option<ReadingRow>) {
        let shown: HashMap<u64, ReadingRow> = (0..model.len())
            .filter_map(|i| model.with_item(i, |r| r.reading.clone().map(|x| (r.item_id, x))))
            .flatten()
            .collect();
        for row in &mut next {
            if let (Some(now), Some(was)) = (row.reading.as_mut(), shown.get(&row.item_id)) {
                now.keep_shown_fields(was, style);
            }
        }
        if let (Some(now), Some(was)) = (head.as_mut(), current_head.get()) {
            now.keep_shown_fields(&was, style);
        }
        (next, head)
    }

    /// Whether the row has prose of its own to write into: a scene, a chapter, a note, a
    /// paratext page, as the constraint matrix allows it.
    fn has_own_prose(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> bool {
        skribisto_compiler::plan::main_prose_role(sub_role)
            .is_some_and(|content| skribisto_model::content_allowed(role, sub_role, &content))
    }

    /// Apply `next` onto `model` with the fewest granular ops (remove / insert /
    /// move / in-place set), so the reconciling `Repeater` keeps surviving rows'
    /// editors. Returns the ids that left the stream.
    ///
    /// **Requires `next` to carry unique `item_id`s.** `query()` derives rows
    /// straight from the store's unique `BinderItem` ids, so this holds today —
    /// but nothing enforces it structurally. The alignment loop below relies on
    /// it: each `next` position is expected to claim a distinct slot in `cur`,
    /// and a duplicate id would claim the same slot twice, leaving `cur` one
    /// element short of what a later `pos` needs. Neither `Vec::insert` nor
    /// `ListModel::move_item` clamp an out-of-bounds target — they panic — so
    /// this is asserted here and clamped defensively below.
    fn reconcile(model: &ListModel<StreamRow>, next: Vec<StreamRow>) -> Vec<u64> {
        debug_assert!(
            next.iter().map(|r| r.item_id).collect::<HashSet<_>>().len() == next.len(),
            "reconcile(): `next` contains a duplicate item_id — StreamRowsModel \
             assumes one row per BinderItem; a duplicate drives the alignment \
             loop's target index past `cur`'s length (a release-mode panic in \
             Vec::insert/ListModel::move_item, clamped away below only as a \
             last resort so it degrades to a wrong-order row instead)"
        );

        let mut cur: Vec<StreamRow> = (0..model.len())
            .filter_map(|i| model.with_item(i, |r| r.clone()))
            .collect();

        // 1. Drop rows no longer present (back-to-front to keep indices stable).
        let keep: HashSet<u64> = next.iter().map(|r| r.item_id).collect();
        let mut removed = Vec::new();
        let mut i = cur.len();
        while i > 0 {
            i -= 1;
            if !keep.contains(&cur[i].item_id) {
                removed.push(cur[i].item_id);
                model.remove(i);
                cur.remove(i);
            }
        }

        // 2. Align the survivors to `next`'s order, insert new ids, and update in
        //    place any row whose *type* changed under it — a Promote rewrites
        //    `(role, sub_role)` without moving the item, and the row's chrome and
        //    its allowed editors both depend on it.
        for (pos, want) in next.iter().enumerate() {
            match cur.iter().position(|r| r.item_id == want.item_id) {
                Some(j) => {
                    // In the unique-id case `j != pos` always implies `pos <
                    // cur.len()`: `cur[0..pos]` already matches `next[0..pos]`
                    // (everything before `pos` was placed by an earlier
                    // iteration), so anything still findable by `position()`
                    // sits at or past `pos`, well inside `cur`. A duplicate
                    // `item_id` breaks that — the earlier occurrence already
                    // claimed this row at some index `< pos`, so it turns up
                    // again here with nowhere new reserved for it. Clamp to
                    // the last valid slot instead of handing `move_item` /
                    // `insert` an out-of-bounds target: the duplicate's
                    // content still lands (last write wins), just possibly at
                    // the wrong position rather than crashing the app.
                    let target = if j != pos {
                        let to = pos.min(cur.len().saturating_sub(1));
                        model.move_item(j, to);
                        let it = cur.remove(j);
                        cur.insert(to, it);
                        to
                    } else {
                        pos
                    };
                    if cur[target] != *want {
                        model.set(target, want.clone());
                        cur[target] = want.clone();
                    }
                }
                None => {
                    // Same clamp, mirrored: a prior duplicate can already have
                    // consumed budget this brand-new id expected, running
                    // `pos` past `cur`'s current length.
                    let target = pos.min(cur.len());
                    model.insert(target, want.clone());
                    cur.insert(target, want.clone());
                }
            }
        }
        removed
    }

    #[cfg(test)]
    mod reconcile_tests {
        use super::*;
        use frontend::common::entities::BinderItemRole;

        fn row(item_id: u64) -> StreamRow {
            StreamRow {
                item_id,
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Scene,
                number: None,
                fallback_label: None,
                reading: None,
            }
        }

        fn ids(model: &ListModel<StreamRow>) -> Vec<u64> {
            (0..model.len())
                .filter_map(|i| model.with_item(i, |r| r.item_id))
                .collect()
        }

        /// `debug_assert!` fires before the clamp fallback ever runs, so a
        /// plain `cargo test` (which builds with `debug_assertions = true`)
        /// can only observe the *debug* half of this contract: a duplicate
        /// `item_id` must panic loudly here, not silently corrupt the
        /// stream. The companion test below — gated the other way — proves
        /// the *release* half (the clamp itself never panics) using the
        /// exact shape the panic audit found: a duplicate appearing twice
        /// in `next`, starting from a fresh (empty) model.
        #[cfg(debug_assertions)]
        #[test]
        #[should_panic(expected = "duplicate item_id")]
        fn duplicate_item_id_trips_the_debug_assert() {
            let model: ListModel<StreamRow> = ListModel::new();
            let _ = reconcile(&model, vec![row(1), row(1)]);
        }

        /// Release builds compile the debug_assert above out entirely, so
        /// this is the path the audited panic actually reached in
        /// production: `model.move_item`/`cur.insert` handed an
        /// out-of-bounds target by the second `id=1` row. Only compiled for
        /// a build *without* debug assertions (`cargo test --release`) —
        /// under the default dev profile the debug_assert above already
        /// covers this input, and would fire first.
        #[cfg(not(debug_assertions))]
        #[test]
        fn duplicate_item_id_does_not_panic_in_release() {
            let model: ListModel<StreamRow> = ListModel::new();
            let removed = reconcile(&model, vec![row(1), row(1)]);
            assert!(removed.is_empty());
            // Both duplicate entries collapse onto the one physical row the
            // model has room for — a wrong count, not a crash.
            assert_eq!(model.len(), 1);
        }

        /// Same release-only fallback, but for the `None` branch: a
        /// duplicate earlier in `next` consumes a `cur` slot a later,
        /// genuinely-new id was counting on, so *its* insert target
        /// overshoots `cur`'s length too.
        #[cfg(not(debug_assertions))]
        #[test]
        fn duplicate_before_a_new_id_does_not_panic_in_release() {
            let model = ListModel::from_vec(vec![row(1)]);
            let removed = reconcile(&model, vec![row(1), row(1), row(99)]);
            assert!(removed.is_empty());
            // The new id (99) still makes it into the model somewhere.
            assert!(ids(&model).contains(&99));
        }

        /// Sanity check on the ordinary, non-duplicate path: unaffected by
        /// the clamp, since `pos` never needs clamping when ids are unique.
        /// Runs under every profile.
        #[test]
        fn unique_ids_reorder_without_clamping() {
            let model = ListModel::from_vec(vec![row(1), row(2), row(3)]);
            let removed = reconcile(&model, vec![row(3), row(1), row(2)]);
            assert!(removed.is_empty());
            assert_eq!(ids(&model), vec![3, 1, 2]);
        }
    }
}

/// A Full Book read as exported, against the real store and the bundled Starforgers
/// project: the review that asked for this feature was written about this book.
#[cfg(all(test, not(feature = "mocks")))]
mod reading_tests {
    use std::rc::Rc;

    use frontend::AppContext;
    use frontend::commands::{binder_item_commands, work_commands, work_management_commands};
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use frontend::direct_access::{BinderItemDto, UpdateBinderItemDto};
    use frontend::work_management::LoadWorkDto;
    use skribisto_compiler::{EpigraphPlacement, Preset, builtin_presets};

    use super::{StreamLevel, StreamRowsModel};
    use crate::app_ids::AppIds;

    fn starforgers() -> (Rc<AppContext>, AppIds, Vec<BinderItemDto>) {
        let ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        work_management_commands::load_work(
            &ctx,
            &LoadWorkDto {
                media_root: crate::media_paths::media_root_string(),
                file_name: format!(
                    "{}/../../resources/examples/starforgers/Starforgers.skrib",
                    env!("CARGO_MANIFEST_DIR")
                ),
            },
        )
        .expect("load Starforgers");
        let work_id = work_commands::get_all_work(&ctx).expect("work")[0].id;
        ids.seed(&ctx, work_id);
        let items = binder_item_commands::get_all_binder_item(&ctx).expect("items");
        (ctx, ids, items)
    }

    fn style(paratexts: bool) -> Preset {
        let mut p = builtin_presets()
            .into_iter()
            .find(|p| p.id == "neutral")
            .expect("the neutral style");
        p.include_paratexts = paratexts;
        p.include_epigraphs = true;
        p.epigraph_placement = EpigraphPlacement::AfterHeading;
        p
    }

    fn find(items: &[BinderItemDto], title: &str, sub_role: BinderItemSubRole) -> u64 {
        items
            .iter()
            .find(|it| it.title == title && it.sub_role == sub_role && it.activated)
            .unwrap_or_else(|| panic!("Starforgers has an active {sub_role:?} {title:?}"))
            .id
    }

    /// Read the model in `style` (or as the working stream) from scratch.
    fn read(model: &StreamRowsModel, style: Option<Preset>) -> Vec<super::StreamRow> {
        model.set_reading(Some(self::style(false)));
        model.set_reading(None);
        model.set_reading(style);
        model.rows()
    }

    #[test]
    fn the_book_read_as_exported_has_its_dedication_and_its_epigraphs() {
        let (ctx, ids, items) = starforgers();
        let book = items
            .iter()
            .find(|it| it.role == BinderItemRole::Folder && it.sub_role == BinderItemSubRole::Book)
            .expect("a Book")
            .id;
        let dedication = find(&items, "Dedication", BinderItemSubRole::Paratext);
        let model = StreamRowsModel::new(ctx, ids.work_id.clone(), book, StreamLevel::Book);

        let working = read(&model, None);
        assert!(
            working.iter().all(|r| r.reading.is_none()),
            "the working stream carries no reading"
        );
        assert!(
            !working.iter().any(|r| r.item_id == dedication),
            "the draft keeps skipping the dedication, as it always has"
        );

        let reading = read(&model, Some(style(true)));
        assert!(
            reading.iter().any(|r| r.item_id == dedication),
            "read as exported, the dedication is in the book"
        );
        let epigraphs = reading
            .iter()
            .filter(|r| r.reading.as_ref().is_some_and(|x| x.epigraph.is_some()))
            .count();
        assert!(
            epigraphs >= 30,
            "every numbered chapter opens on its epigraph, found {epigraphs}"
        );
        // The Book opens on its own title in this style (no title page), so the head
        // opens the reading and no row has to.
        let head_opens = model.head_reading().get().is_some();
        let rows_open = reading
            .iter()
            .filter(|r| r.reading.as_ref().is_some_and(|x| x.first))
            .count();
        assert_eq!(
            usize::from(head_opens) + rows_open,
            1,
            "exactly one thing opens the reading"
        );

        let without = read(&model, Some(style(false)));
        assert!(
            !without.iter().any(|r| r.item_id == dedication),
            "a style that drops paratexts drops the dedication from the reading too"
        );
    }

    /// Clearing an epigraph while reading keeps its editor, so the writer's caret is not
    /// pulled out from under them; switching the reading on again shows the book as it now
    /// is, without it.
    #[test]
    fn a_cleared_epigraph_keeps_its_editor_until_the_reading_is_switched_on_again() {
        use frontend::commands::content_commands;
        use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
        use frontend::common::entities::ContentRole;
        use frontend::direct_access::UpdateContentDto;

        let (ctx, ids, items) = starforgers();
        ids.open_stack(&ctx);
        let book = items
            .iter()
            .find(|it| it.role == BinderItemRole::Folder && it.sub_role == BinderItemSubRole::Book)
            .expect("a Book")
            .id;
        let model = StreamRowsModel::new(ctx.clone(), ids.work_id.clone(), book, StreamLevel::Book);
        let rows = read(&model, Some(style(true)));
        let chapter = rows
            .iter()
            .find(|r| r.reading.as_ref().is_some_and(|x| x.epigraph.is_some()))
            .expect("a chapter with an epigraph")
            .item_id;
        let shown = |model: &StreamRowsModel| {
            model
                .rows()
                .iter()
                .find(|r| r.item_id == chapter)
                .and_then(|r| r.reading.as_ref())
                .is_some_and(|x| x.epigraph.is_some())
        };

        let content_ids = binder_item_commands::get_binder_item_relationship(
            &ctx,
            &chapter,
            &BinderItemRelationshipField::Contents,
        )
        .expect("contents");
        let epigraph = content_commands::get_content_multi(&ctx, &content_ids)
            .expect("read")
            .into_iter()
            .flatten()
            .find(|c| c.role == ContentRole::EpigraphText)
            .expect("the epigraph");
        let mut cleared = UpdateContentDto::from(epigraph);
        cleared.data = String::new();
        content_commands::update_content(&ctx, ids.stack_id.get(), &cleared)
            .expect("clear the epigraph");

        model.refill();
        assert!(shown(&model), "the editor stays while the reading is on");
        read(&model, Some(style(true)));
        assert!(!shown(&model), "a fresh reading shows the book without it");
    }

    #[test]
    fn a_scene_left_out_of_exports_leaves_the_reading_and_stays_in_the_draft() {
        let (ctx, ids, items) = starforgers();
        let book = items
            .iter()
            .find(|it| it.role == BinderItemRole::Folder && it.sub_role == BinderItemSubRole::Book)
            .expect("a Book")
            .id;
        let prologue = find(&items, "Prologue", BinderItemSubRole::Scene);
        let model = StreamRowsModel::new(ctx.clone(), ids.work_id.clone(), book, StreamLevel::Book);
        assert!(
            read(&model, Some(style(true)))
                .iter()
                .any(|r| r.item_id == prologue)
        );

        let dto = items
            .iter()
            .find(|it| it.id == prologue)
            .cloned()
            .expect("the prologue");
        let mut update = UpdateBinderItemDto::from(dto);
        update.is_exportable = false;
        binder_item_commands::update_binder_item(&ctx, ids.stack_id.get(), &update)
            .expect("turn Include in exports off");

        assert!(
            read(&model, None).iter().any(|r| r.item_id == prologue),
            "the draft still shows the cut prologue"
        );
        assert!(
            !read(&model, Some(style(true)))
                .iter()
                .any(|r| r.item_id == prologue),
            "the reading does not"
        );

        // The container itself left out: the reading drops it too, which is what hides its
        // own prose at the top of the page.
        assert!(
            model.head_reading().get().is_some(),
            "the Book is in its reading"
        );
        let book_dto = items
            .iter()
            .find(|it| it.id == book)
            .cloned()
            .expect("the Book");
        let mut update = UpdateBinderItemDto::from(book_dto);
        update.is_exportable = false;
        binder_item_commands::update_binder_item(&ctx, ids.stack_id.get(), &update)
            .expect("turn the Book's Include in exports off");
        read(&model, Some(style(true)));
        assert_eq!(
            model.head_reading().get(),
            None,
            "and leaves it out of the reading"
        );
    }

    /// Renaming an untitled chapter shows its new title in the working stream, and
    /// clearing a title shows the generated name again. The header draws an untitled row
    /// from its baked `fallback_label` and a titled one from the live title, so the row
    /// itself has to change when a title comes or goes: a tester renamed untitled
    /// chapters in the outline and the Full Synopsis kept their generated names
    /// (2026-11-24).
    #[test]
    fn renaming_an_untitled_chapter_drops_its_generated_name_from_the_stream() {
        use frontend::common::event::{DirectAccessEntity, EntityEvent, Origin};
        use skribisto_model::SubRoleExt;

        let (ctx, ids, items) = starforgers();
        ids.open_stack(&ctx);
        let book = items
            .iter()
            .find(|it| it.role == BinderItemRole::Folder && it.sub_role == BinderItemSubRole::Book)
            .expect("a Book")
            .id;
        let model = StreamRowsModel::new(ctx.clone(), ids.work_id.clone(), book, StreamLevel::Book);

        // A binder rename reaches the working stream: it is the event `refill` stands
        // in for below.
        assert!(
            super::imp::working_origins().contains(&Origin::DirectAccess(
                DirectAccessEntity::BinderItem(EntityEvent::Updated)
            )),
            "the working stream refills on a BinderItem update"
        );

        let row_of = |model: &StreamRowsModel, id: u64| {
            model
                .rows()
                .into_iter()
                .find(|r| r.item_id == id)
                .expect("the chapter is a row of the Full Book")
        };
        model.refill();
        let chapter = model
            .rows()
            .into_iter()
            .find(|r| r.sub_role.opens_chapter())
            .expect("Starforgers has a chapter")
            .item_id;
        let rename = |title: &str| {
            let dto = binder_item_commands::get_binder_item(&ctx, &chapter)
                .expect("read")
                .expect("the chapter");
            let mut update = UpdateBinderItemDto::from(dto);
            update.title = title.to_string();
            binder_item_commands::update_binder_item(&ctx, ids.stack_id.get(), &update)
                .expect("rename the chapter");
        };

        rename("");
        model.refill();
        assert!(
            row_of(&model, chapter).fallback_label.is_some(),
            "an untitled chapter is shown under its generated name"
        );

        rename("The Forge");
        model.refill();
        assert_eq!(
            row_of(&model, chapter).fallback_label,
            None,
            "once titled, the row shows its live title, not the generated name"
        );

        rename("");
        model.refill();
        assert!(
            row_of(&model, chapter).fallback_label.is_some(),
            "and clearing the title brings the generated name back"
        );
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    use skribisto_compiler::Preset;
    use skribisto_compiler::plan::{RowFacts, plan_row};

    use super::{ReadingRow, StreamLevel, StreamRow};

    #[derive(Clone)]
    pub struct StreamRowsModel {
        model: ListModel<StreamRow>,
        level: StreamLevel,
        head_reading: Signal<Option<ReadingRow>>,
    }

    /// Fabricated rows matching the mock binder fixture in
    /// `singles::SingleBinderItem`: the mock Book (101) holds a Part (301) with two
    /// chapters — a chapter folder (104) holding scenes 201-203, and a flat
    /// `Item/ChapterScene` (302) followed by scene 303.
    fn mock_rows(level: StreamLevel) -> Vec<StreamRow> {
        use BinderItemRole::{Folder, Item};
        use BinderItemSubRole::{ChapterScene, Part, Scene};
        let row = |item_id: u64, role, sub_role| StreamRow {
            item_id,
            role,
            sub_role,
            // Every fixture row is titled, so none needs the fallback.
            fallback_label: None,
            // The fixture book's chapter ordinals, in step with the mock binder tree.
            number: match item_id {
                301 => Some(1), // Part One
                104 => Some(1), // the first chapter
                302 => Some(2),
                105 => Some(3),
                _ => None,
            },
            reading: None,
        };
        match level {
            // A chapter streams only its own scenes.
            StreamLevel::Chapter => vec![
                row(201, Item, Scene),
                row(202, Item, Scene),
                row(203, Item, Scene),
            ],
            // A part streams its chapters' heads *and* their scenes.
            StreamLevel::Part => vec![
                row(104, Folder, ChapterScene),
                row(201, Item, Scene),
                row(202, Item, Scene),
                row(203, Item, Scene),
                row(302, Item, ChapterScene),
                row(303, Item, Scene),
            ],
            // A book adds the part heads on top.
            StreamLevel::Book => vec![
                row(301, Folder, Part),
                row(104, Folder, ChapterScene),
                row(201, Item, Scene),
                row(202, Item, Scene),
                row(203, Item, Scene),
                row(302, Item, ChapterScene),
                row(303, Item, Scene),
            ],
        }
    }

    impl StreamRowsModel {
        pub fn new(
            _ctx: Rc<AppContext>,
            _work_id: Signal<Option<u64>>,
            _head_id: u64,
            level: StreamLevel,
        ) -> Self {
            Self {
                model: ListModel::from_vec(mock_rows(level)),
                level,
                // The fixture's containers carry no heading text of their own to read.
                head_reading: Signal::new(None),
            }
        }

        pub fn head_reading(&self) -> Signal<Option<ReadingRow>> {
            self.head_reading.clone()
        }

        /// The fixture read as exported in `style`, decided by the compiler's own
        /// [`plan_row`] over facts invented for the fixture: every row has prose, and the
        /// chapter folder (104) carries an epigraph.
        pub fn set_reading(&self, style: Option<Preset>) {
            let rows = mock_rows(self.level);
            let next = match style {
                None => rows,
                Some(style) => {
                    let mut out: Vec<StreamRow> = Vec::new();
                    for row in rows {
                        let facts = RowFacts {
                            sub_role: &row.sub_role,
                            title: None,
                            number: row.number,
                            heading_lang: "en",
                            has_epigraph: row.item_id == 104,
                            has_main: true,
                            has_synopsis: false,
                        };
                        if let Some(plan) = plan_row(&facts, &style, false) {
                            let planned = skribisto_compiler::PlannedRow {
                                item_id: row.item_id,
                                opens_page: plan.starts_page,
                                plan,
                            };
                            let reading = ReadingRow::from_plan(&planned, out.is_empty());
                            out.push(StreamRow {
                                number: None,
                                reading: Some(reading),
                                ..row
                            });
                        }
                    }
                    out
                }
            };
            self.model.replace_all(next);
        }

        pub fn list(&self) -> ListModel<StreamRow> {
            self.model.clone()
        }

        pub fn rows(&self) -> Vec<StreamRow> {
            let m = &self.model;
            (0..m.len())
                .filter_map(|i| m.with_item(i, |r| r.clone()))
                .collect()
        }

        pub fn ids(&self) -> Vec<u64> {
            let m = &self.model;
            (0..m.len())
                .filter_map(|i| m.with_item(i, |r| r.item_id))
                .collect()
        }

        pub fn wire(&self, _ctx: &mut BuildContext, _on_removed: impl Fn(&[u64]) + 'static) {}
    }
}

pub use imp::StreamRowsModel;

#[cfg(test)]
mod tests {
    use super::{StreamLevel, row_indices};
    use frontend::common::entities::BinderItemRole;
    use frontend::common::entities::BinderItemSubRole::*;

    /// The original Full Chapter rule. The head is the pane header, never a row —
    /// which is now load-bearing: a chapter folder `carries_scene()` like any scene,
    /// so without the head-skip a `Folder/ChapterScene` head would list itself.
    #[test]
    fn boundary_rule() {
        let chapter = |s: &[_], h| row_indices(s, h, StreamLevel::Chapter);
        // A Note between scenes is skipped (not scene-bearing, not a boundary); the
        // run stops at the next chapter.
        assert_eq!(
            chapter(&[ChapterScene, Scene, Note, Scene, ChapterScene, Scene], 0),
            vec![1, 3]
        );
        // Text carries no content — neither scene nor boundary.
        assert_eq!(chapter(&[ChapterScene, Text, Scene], 0), vec![2]);
        // BookBegin / BookEnd are boundaries.
        assert_eq!(chapter(&[ChapterScene, Scene, BookEnd, Scene], 0), vec![1]);
        // Runs to the end of the stream when no boundary follows.
        assert_eq!(chapter(&[ChapterScene, Scene, Scene], 0), vec![1, 2]);
        // The head is never a row, even though it carries scene prose.
        assert_eq!(
            chapter(&[ChapterScene, Part, Scene], 0),
            Vec::<usize>::new()
        );
    }

    /// A part does not end at a chapter — it contains them, heads and scenes alike.
    #[test]
    fn part_stream_contains_its_chapters_heads_and_their_scenes() {
        assert_eq!(
            row_indices(
                &[
                    Part,
                    ChapterScene,
                    Scene,
                    Scene,
                    ChapterScene,
                    Scene,
                    Part,
                    Scene
                ],
                0,
                StreamLevel::Part
            ),
            vec![1, 2, 3, 4, 5],
            "stops at the next Part, keeps both chapter heads"
        );
    }

    /// A book contains parts, chapters and scenes.
    #[test]
    fn book_stream_contains_parts_chapters_and_scenes() {
        assert_eq!(
            row_indices(
                &[
                    Book,
                    Part,
                    ChapterScene,
                    Scene,
                    Scene,
                    Part,
                    ChapterScene,
                    Scene
                ],
                0,
                StreamLevel::Book
            ),
            vec![1, 2, 3, 4, 5, 6, 7],
            "a part no longer ends a book stream"
        );
    }

    #[test]
    fn book_end_terminates_a_book_stream() {
        assert_eq!(
            row_indices(
                &[Book, ChapterScene, Scene, BookEnd, Scene],
                0,
                StreamLevel::Book
            ),
            vec![1, 2]
        );
    }

    /// Both encodings of "a new book starts here" close the previous one.
    #[test]
    fn a_second_book_terminates_the_first() {
        assert_eq!(
            row_indices(
                &[Book, ChapterScene, Scene, Book, Scene],
                0,
                StreamLevel::Book
            ),
            vec![1, 2]
        );
        assert_eq!(
            row_indices(
                &[Book, ChapterScene, Scene, BookBegin, Scene],
                0,
                StreamLevel::Book
            ),
            vec![1, 2]
        );
    }

    /// Notes and plain folders are organisational, not manuscript — skipped, but they
    /// do not close the run.
    #[test]
    fn notes_and_folders_are_skipped_not_boundaries() {
        assert_eq!(
            row_indices(
                &[Part, Scene, Note, None, Scene, Part],
                0,
                StreamLevel::Part
            ),
            vec![1, 4]
        );
    }

    #[test]
    fn degenerate_extents() {
        assert_eq!(
            row_indices(&[Part, Scene, Scene], 0, StreamLevel::Part),
            vec![1, 2]
        );
        assert_eq!(
            row_indices(&[Part, Part, Scene], 0, StreamLevel::Part),
            Vec::<usize>::new(),
            "an empty part yields no rows"
        );
        assert_eq!(
            row_indices(&[ChapterScene], 0, StreamLevel::Chapter),
            Vec::<usize>::new()
        );
    }

    /// Only the folder containers host a stream — the flat chapter/book markers keep
    /// their own tabs.
    #[test]
    fn only_folder_containers_have_a_level() {
        use BinderItemRole::{Folder, Item};
        assert_eq!(
            StreamLevel::for_container(&Folder, &ChapterScene),
            Some(StreamLevel::Chapter)
        );
        assert_eq!(
            StreamLevel::for_container(&Folder, &Part),
            Some(StreamLevel::Part)
        );
        assert_eq!(
            StreamLevel::for_container(&Folder, &Book),
            Some(StreamLevel::Book)
        );
        // The flat chapter is *not* a container: it keeps its dual-pane editor.
        assert_eq!(
            StreamLevel::for_container(&Item, &ChapterScene),
            Option::None
        );
        assert_eq!(StreamLevel::for_container(&Item, &BookBegin), Option::None);
        assert_eq!(StreamLevel::for_container(&Item, &Scene), Option::None);
        assert_eq!(StreamLevel::for_container(&Folder, &Note), Option::None);
        assert_eq!(StreamLevel::for_container(&Folder, &None), Option::None);
    }
}
