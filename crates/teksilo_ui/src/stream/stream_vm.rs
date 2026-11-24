// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `StreamViewModel` — business logic for a container tab's **manuscript streams**:
//! the Full Chapter / Full Part / Full Book view and its Full Synopsis twin.
//!
//! It owns the container's ordered row list ([`StreamRowsModel`]) and the row
//! mutations (rename, set label, insert / add / move / merge / split / trash). The
//! two stream *flavours* — prose and synopsis — are the same rows with a different
//! editor per row, so one view-model drives both; [`SplitFlavour`] names which one an
//! action came from.
//!
//! **One instance per open container tab**, created by `ContentTab::new` — *not* by
//! `OpenDoc::build`. That placement is load-bearing:
//!
//! * This view-model holds the [`OpenDocsStore`] (it opens its rows' documents through
//!   it), and the store owns every `OpenDoc`. Hanging the view-model off an `OpenDoc`
//!   would close an `Rc` cycle — and worse, `OpenDocsStore::clear()` drops its
//!   `OpenDoc`s *while holding the map's `RefCell` borrow*, so a `Drop` that released
//!   row refs would re-enter `borrow_mut()` and panic. On `ContentTab` the chain
//!   `TabHandle → ContentTab → StreamViewModel → OpenDocsStore` runs one way, and
//!   nothing the store owns points back.
//! * Sharing it across panes is no longer needed for what that used to buy: two panes
//!   on the same container get two view-models, but both call `store.open(row_id)` for
//!   the same ids, so their rows are the **same live documents** regardless.
//!
//! Rows take their documents from the store, so a scene open as its own tab *and*
//! shown inside a stream is one document — which is what the old per-view `SingleScene`
//! got wrong (two `TextDocument`s over one `Content` row, silently diverging).
//!
//! All name entry is a modal `InputDialog` (mirrors `OutlineViewModel::begin_rename`);
//! each `begin_*` presents the dialog and the matching apply-method does the undoable
//! backend call. Plain Rust → headless-testable.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use teksilo::data::ListModel;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*; // EventContext, Signal, BuildContext, tr!
use teksilo::text_document::TextDocument;
use teksilo::widgets::InputDialog;

use crate::comments::binding::CommentBinding;

use frontend::AppContext;
use frontend::binder_item_management::{MergeTwoScenesDto, MovePlace, SplitSceneDto};
use frontend::commands::trash_management_commands;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
use frontend::trash_management::TrashSelectionDto;

use skribisto_compiler::Preset;

use crate::app_ids::AppIds;
use crate::export::ExportStylesViewModel;
use crate::models::{OpenDoc, OpenDocsStore, StreamLevel, StreamRow, StreamRowsModel};
use crate::singles::SingleBinderItem;

use super::DepthFilter;

use crate::shared::binder_ops::{self, is_prose_bearing, opens_a_section, split_djot};

/// Which of a row's two writing surfaces an action came from. The stream pane picks
/// its body with this, and a split cuts *that* role at the caret.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SplitFlavour {
    Prose,
    Synopsis,
}

/// One cached row: its shared document plus a probe for the reactive title / label.
#[derive(Clone)]
struct RowHandle {
    doc: Rc<OpenDoc>,
    probe: SingleBinderItem,
    /// The `sub_role` the `doc` was built from. An `OpenDoc`'s fields are decided once,
    /// at construction, from the constraint matrix — so if a Promote changes the row's
    /// type, the cached doc is stale and must be evicted and reopened.
    sub_role: BinderItemSubRole,
}

struct Inner {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    docs: OpenDocsStore,
    container_id: u64,
    container_sub_role: BinderItemSubRole,
    level: StreamLevel,
    rows: StreamRowsModel,
    container_probe: SingleBinderItem,
    /// One handle per row, created lazily and reused across list refreshes so an edited
    /// row's document survives structural changes.
    row_handles: RefCell<HashMap<u64, RowHandle>>,

    /// Whether this tab reads the stream **as exported**. Per tab, like the segment it
    /// lives on: two panes on one book can show the draft and the book side by side.
    ///
    /// The switch, not what is on screen: it only takes effect while the prose page is
    /// the one shown (see [`shown`](Self::shown)).
    reading_on: Signal<bool>,
    /// Whether the page on screen shows the working stream, as a signal of its own: what
    /// the working stream's furniture (the container header, the add button) binds its
    /// visibility to. The negation of "the switch is on **and** the prose page is shown".
    working_on: Signal<bool>,
    /// Which of the two stream pages was last put on screen, reported by the page itself
    /// as it becomes active.
    ///
    /// Read as exported is a Prose matter: a synopsis is not part of the book, and the
    /// switch is on the prose page only. Both pages show the same row list, though, so
    /// without this the reading followed the writer to the Full Synopsis, which then
    /// showed the book's prose with no switch on the page to turn it off.
    shown: Cell<SplitFlavour>,
    /// The levels this tab shows (see [`DepthFilter`]). Per tab, shared by its two stream
    /// pages, which show one row list. `Option` because that is what the page's
    /// `ComboBox` binds; `None` reads as no filter.
    depth_choice: Signal<Option<DepthFilter>>,
    /// Whether the page on screen hides levels: a filter is chosen, and the page shows
    /// the working stream (a reading shows every row). What the banner shows on.
    filter_on: Signal<bool>,
    /// Its negation, as a signal of its own: whether rows may be added, moved, merged or
    /// split. Each of those acts on rows a hidden level can be made of (a flat chapter
    /// moved up past a hidden scene takes that scene into it), so while levels are hidden
    /// they are greyed out, and refused here as well.
    structure_editable: Signal<bool>,
    /// The style a reading follows: the book's remembered export style, resolved against
    /// every style this machine has. Kept current while the reading is off too, so the
    /// picker shows the right entry the moment it appears.
    reading_style: Signal<Option<Preset>>,
    /// The writer's own export styles, from app state, captured on the first build.
    styles: RefCell<Option<ExportStylesViewModel>>,
}

/// Release every document this stream still holds, when the tab closes (the
/// `ContentTab` drops, taking the last `Rc<Inner>` with it).
///
/// This is *why* `wire`'s closures capture a `Weak`: an `Rc` capture would keep `Inner`
/// alive forever, this `Drop` would never run, and every row the stream ever opened
/// would leak. Calling `release` here is safe — `OpenDocsStore::release` drops the
/// evicted `Rc<OpenDoc>` *after* letting go of the map's borrow, and an `OpenDoc` is a
/// leaf that points at nothing.
impl Drop for Inner {
    fn drop(&mut self) {
        let stack = self.ids.stack_id.get();
        for id in self.row_handles.borrow().keys() {
            self.docs.release(*id, stack);
        }
    }
}

#[derive(Clone)]
pub struct StreamViewModel {
    inner: Rc<Inner>,
}

// Not every affordance has a caller until every stream surface is wired.
#[allow(dead_code)]
impl StreamViewModel {
    /// A stream for `container_id`, or `None` if this `(role, sub_role)` has none. The
    /// gate is [`StreamLevel::for_container`] — the *same* one `ContentTab::new` uses,
    /// so there is no second partial function to keep in sync by hand.
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        docs: OpenDocsStore,
        container_id: u64,
        container_role: &BinderItemRole,
        container_sub_role: &BinderItemSubRole,
    ) -> Option<Self> {
        let level = StreamLevel::for_container(container_role, container_sub_role)?;
        let rows = StreamRowsModel::new(app_ctx.clone(), ids.work_id.clone(), container_id, level);
        let container_probe = SingleBinderItem::new(app_ctx.clone());
        container_probe.set_id(Some(container_id));
        Some(Self {
            inner: Rc::new(Inner {
                app_ctx,
                ids,
                docs,
                container_id,
                container_sub_role: container_sub_role.clone(),
                level,
                rows,
                container_probe,
                row_handles: RefCell::new(HashMap::new()),
                reading_on: Signal::new(false),
                working_on: Signal::new(true),
                shown: Cell::new(SplitFlavour::Prose),
                depth_choice: Signal::new(Some(DepthFilter::Everything)),
                filter_on: Signal::new(false),
                structure_editable: Signal::new(true),
                reading_style: Signal::new(None),
                styles: RefCell::new(None),
            }),
        })
    }

    /// Subscribe once (row list + per-row metadata) and fill the list.
    ///
    /// Both closures capture a **`Weak`**, never `self`. They are *stored* — the removal
    /// callback for the row model's lifetime, the event subscription for the widget
    /// tree's — and the row model is itself owned by this view-model, so an `Rc` capture
    /// would close the cycle `Inner → rows → stored closure → Inner`. `Drop` would never
    /// run and every row this stream opened would leak. A stale `Weak` simply fails to
    /// `upgrade()` once the tab is gone.
    pub fn wire(&self, ctx: &mut BuildContext) {
        // Rows that left the stream (trashed, merged away, moved out): drop our
        // reference to their shared documents.
        let weak = Rc::downgrade(&self.inner);
        self.inner.rows.wire(ctx, move |removed: &[u64]| {
            let Some(inner) = weak.upgrade() else {
                return;
            };
            let stack = inner.ids.stack_id.get();
            let gone: Vec<u64> = {
                let mut cache = inner.row_handles.borrow_mut();
                removed
                    .iter()
                    .filter(|id| cache.remove(id).is_some())
                    .copied()
                    .collect()
            };
            for id in gone {
                inner.docs.release(id, stack);
            }
        });

        // Every build, not once — see `StreamRowsModel::wire`: a subscription is scoped
        // to the widget's current build and dropped on the next one.
        {
            let weak = Rc::downgrade(&self.inner);
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
                move |event: &Event| {
                    let Some(inner) = weak.upgrade() else {
                        return;
                    };
                    if event.ids.contains(&inner.container_id) {
                        inner.container_probe.set_id(Some(inner.container_id));
                    }
                    let want: HashMap<u64, BinderItemSubRole> = inner
                        .rows
                        .rows()
                        .into_iter()
                        .map(|r| (r.item_id, r.sub_role))
                        .collect();
                    // A Promote rewrites a row's type in place. That row's `OpenDoc`
                    // decided its fields at construction, from the *old* type, so it is
                    // now stale. Dropping our cached handle is not enough: while any other
                    // holder (a standalone tab, the other pane) still references the entry,
                    // the store keeps the stale doc and hands it straight back. Rebuild it
                    // in place instead, so every holder sees the fresh one.
                    let stale: Vec<u64> = {
                        let mut cache = inner.row_handles.borrow_mut();
                        let mut stale = Vec::new();
                        for id in &event.ids {
                            let Some(h) = cache.get(id) else { continue };
                            h.probe.set_id(Some(*id));
                            if want.get(id).is_some_and(|sr| *sr != h.sub_role) {
                                stale.push(*id);
                            }
                        }
                        for id in &stale {
                            cache.remove(id);
                        }
                        stale
                    };
                    let stack = inner.ids.stack_id.get();
                    for id in stale {
                        inner.docs.rebuild(id, stack);
                    }
                },
            );
        }

        // The reading follows the book's remembered style, so it moves when the Work does
        // (a style picked here, in another pane, or by an export) and when the writer edits
        // their styles in Settings. Every build, like the subscriptions above.
        if self.inner.styles.borrow().is_none() {
            *self.inner.styles.borrow_mut() = ctx.app_state::<ExportStylesViewModel>().cloned();
        }
        {
            let weak = Rc::downgrade(&self.inner);
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::Work(EntityEvent::Updated)),
                move |_event: &Event| {
                    if let Some(inner) = weak.upgrade() {
                        StreamViewModel { inner }.sync_reading();
                    }
                },
            );
        }
        let changed = self
            .inner
            .styles
            .borrow()
            .as_ref()
            .map(|s| s.changed_signal());
        if let Some(changed) = changed {
            let weak = Rc::downgrade(&self.inner);
            ctx.effect(&changed, move |_| {
                if let Some(inner) = weak.upgrade() {
                    StreamViewModel { inner }.sync_reading();
                }
            });
        }
        self.sync_reading();
    }

    // ── read as exported ──

    /// Whether this tab reads the stream as exported.
    pub fn reading_on(&self) -> Signal<bool> {
        self.inner.reading_on.clone()
    }

    /// Whether the stream page on screen shows the working stream: false only while
    /// [`reading_on`](Self::reading_on) is on and the prose page is the one shown.
    pub fn working_on(&self) -> Signal<bool> {
        self.inner.working_on.clone()
    }

    /// How the container itself reads as exported. See
    /// [`StreamRowsModel::head_reading`](crate::models::StreamRowsModel::head_reading).
    pub fn head_reading(&self) -> Signal<Option<crate::models::ReadingRow>> {
        self.inner.rows.head_reading()
    }

    /// Whether the row is marked "Include in exports", live.
    pub fn row_exportable(&self, id: u64) -> Signal<bool> {
        match self.handle(id) {
            Some(h) => h
                .probe
                .dto_signal()
                .map(|d| d.as_ref().is_none_or(|x| x.is_exportable)),
            None => Signal::new(true),
        }
    }

    /// The style a reading follows (see the field's own note).
    pub fn reading_style(&self) -> Signal<Option<Preset>> {
        self.inner.reading_style.clone()
    }

    /// Every style the reading can follow: the built-ins, then the writer's own.
    pub fn style_catalogue(&self) -> Vec<Preset> {
        crate::export::style::catalogue(self.inner.styles.borrow().as_ref())
    }

    /// Read the stream as exported, or go back to the working stream.
    pub fn set_reading_on(&self, on: bool) {
        self.inner.reading_on.set_if_changed(on);
        self.sync_reading();
    }

    /// The `flavour` page has just been put on screen. Called by the page as it becomes
    /// active, so the rows follow whichever of the two is shown: the reading on the
    /// prose page, the working rows on the synopsis page. See [`Inner::shown`].
    pub fn show_flavour(&self, flavour: SplitFlavour) {
        if self.inner.shown.replace(flavour) != flavour {
            self.sync_reading();
        }
    }

    /// Follow the `flavour` page as it comes on screen and goes: call once per build of
    /// that page, from inside it.
    ///
    /// The page's own activation, not the tab's segment: a `Switcher` parks a page it
    /// leaves rather than dropping it, and only activation says which of the two built
    /// pages is the one on screen. Captures a **`Weak`**, like every closure in
    /// [`wire`](Self::wire), so a parked page cannot keep a closed tab's rows alive.
    pub fn follow_page(&self, ctx: &mut BuildContext, flavour: SplitFlavour) {
        let active = ctx.activation_signal(ctx.self_id());
        // Seeded: an effect fires on later changes only, and a page is normally already
        // active by the time it builds.
        if active.get() {
            self.show_flavour(flavour);
        }
        let weak = Rc::downgrade(&self.inner);
        ctx.effect(&active, move |&on| {
            if on && let Some(inner) = weak.upgrade() {
                StreamViewModel { inner }.show_flavour(flavour);
            }
        });
    }

    /// Whether the rows are read as exported right now: the switch is on and the page
    /// on screen is the prose one.
    fn reading_shown(&self) -> bool {
        self.inner.reading_on.get() && self.inner.shown.get() == SplitFlavour::Prose
    }

    /// Read the book in `style` from now on, here and in the Export panel.
    ///
    /// Written to the Work, on its undo stack: the style a book is read and exported in is
    /// manuscript data a co-author shares, not a preference of this window. The Work's
    /// own update then re-syncs every stream on the book.
    pub fn choose_reading_style(&self, style: &Preset) {
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return;
        };
        if let Err(e) = crate::export::style::remember(
            &self.inner.app_ctx,
            work_id,
            self.inner.ids.stack_id.get(),
            &style.id,
        ) {
            eprintln!("stream: could not remember the export style: {e:#}");
        }
        self.sync_reading();
    }

    /// Resolve the book's style and hand the rows the one they are read in, or none.
    fn sync_reading(&self) {
        let remembered = self
            .inner
            .ids
            .work_id
            .get()
            .map(|work_id| crate::export::style::remembered(&self.inner.app_ctx, work_id))
            .unwrap_or_default();
        let style = crate::export::style::resolve(&remembered, &self.style_catalogue());
        self.inner.reading_style.set_if_changed(style.clone());
        let reading = self.reading_shown();
        self.inner.working_on.set_if_changed(!reading);
        self.inner.rows.set_reading(style.filter(|_| reading));
        self.sync_filter_flags();
    }

    // ── levels shown ──

    /// The filters this container offers, the first being none; empty for a Chapter.
    pub fn depth_filters_offered(&self) -> &'static [DepthFilter] {
        DepthFilter::offered(self.inner.level)
    }

    /// The levels chosen, as the page's `ComboBox` binds them.
    pub fn depth_choice(&self) -> Signal<Option<DepthFilter>> {
        self.inner.depth_choice.clone()
    }

    /// The levels chosen.
    pub fn depth_filter(&self) -> DepthFilter {
        self.inner.depth_choice.get().unwrap_or_default()
    }

    /// Show the levels `filter` keeps. A filter this container does not offer shows
    /// everything (see [`DepthFilter::within`]).
    pub fn set_depth_filter(&self, filter: DepthFilter) {
        let filter = filter.within(self.inner.level);
        self.inner.depth_choice.set_if_changed(Some(filter));
        self.inner.rows.set_shown_down_to(filter.limit());
        self.sync_filter_flags();
    }

    /// Restore the filter a tab was remembered with, by its stable string (see
    /// [`DepthFilter::id`]).
    pub fn seed_depth_filter(&self, id: Option<&str>) {
        self.set_depth_filter(DepthFilter::from_id(id));
    }

    /// Whether the page on screen hides levels. See [`Inner::filter_on`].
    pub fn filter_on(&self) -> Signal<bool> {
        self.inner.filter_on.clone()
    }

    /// Whether rows may be added, moved, merged or split. See
    /// [`Inner::structure_editable`].
    pub fn structure_editable(&self) -> Signal<bool> {
        self.inner.structure_editable.clone()
    }

    /// The rows `id` sits under, nearest first: the chapter a scene is in, then the part
    /// that chapter is in. Empty for a row at the first level, and for anything that is
    /// not a row (the container's own text).
    ///
    /// Where the page goes when a filter hides the passage being read: to the nearest of
    /// these that is still shown.
    pub fn row_parents(&self, id: u64) -> Vec<u64> {
        let rows = self.inner.rows.ids();
        let Some(pos) = rows.iter().position(|&r| r == id) else {
            return Vec::new();
        };
        let mut depth = self.row_depth(id).get();
        let mut parents = Vec::new();
        for &above in rows[..pos].iter().rev() {
            if depth <= 1 {
                break;
            }
            let d = self.row_depth(above).get();
            if d < depth {
                parents.push(above);
                depth = d;
            }
        }
        parents
    }

    /// Keep the banner and the structural actions in step with the filter and with
    /// what the page on screen shows.
    fn sync_filter_flags(&self) {
        let on = self.depth_filter() != DepthFilter::Everything && !self.reading_shown();
        self.inner.filter_on.set_if_changed(on);
        self.inner.structure_editable.set_if_changed(!on);
    }

    /// Whether a structural action may run now. See [`Inner::structure_editable`].
    fn structure_unlocked(&self) -> bool {
        self.inner.structure_editable.get()
    }

    // ── reactive reads ──

    /// The `Repeater`'s data source.
    pub fn list(&self) -> ListModel<StreamRow> {
        self.inner.rows.list()
    }

    /// A snapshot of the rows — for the synchronous gating below, which needs each row's
    /// `(role, sub_role)`, not just its id.
    pub fn rows(&self) -> Vec<StreamRow> {
        self.inner.rows.rows()
    }

    pub fn level(&self) -> StreamLevel {
        self.inner.level
    }

    /// How deep the row `id` sits under the container, live: 1 for its first level.
    /// What the row is set in by (see `crate::stream::level_inset`).
    pub fn row_depth(&self, id: u64) -> Signal<u8> {
        self.inner.rows.depth(id)
    }

    /// The deepest level among the rows shown, live; 0 in a reading and on an empty
    /// page.
    pub fn deepest(&self) -> Signal<u8> {
        self.inner.rows.deepest()
    }

    pub fn container_title(&self) -> Signal<String> {
        self.inner.container_probe.title()
    }

    /// The row's shared live documents, opened through the store — so a row and a
    /// standalone tab on the same item are one document. Cached and reused.
    pub fn row_doc(&self, id: u64) -> Option<Rc<OpenDoc>> {
        self.handle(id).map(|h| h.doc)
    }

    /// Every document this stream is **already showing**, in the order it shows them —
    /// what a find spanning the whole page searches.
    ///
    /// Deliberately non-opening: it reads the row cache directly rather than going
    /// through [`row_doc`](Self::row_doc), which on a miss runs a full synchronous Djot
    /// import — the thing that once made switching a Book to Full Book freeze for
    /// seconds. A row the page has not opened yet is a row the reader is not looking at,
    /// and it contributes nothing rather than being imported on a keystroke.
    ///
    /// In practice that excludes nothing on a mounted page: `Repeater` is not
    /// virtualized, so every row of the stream has a live editor and its document is
    /// opened by the row factory. What it does exclude is the moment *before* that — the
    /// banner is the page's parent and builds first — which the per-frame re-sync in
    /// [`FindViewModel::tick`](crate::search::FindViewModel::tick) picks up.
    pub fn page_documents(&self, flavour: SplitFlavour) -> Vec<(u64, TextDocument)> {
        let cache = self.inner.row_handles.borrow();
        self.inner
            .rows
            .ids()
            .into_iter()
            .filter_map(|id| {
                let handle = cache.get(&id)?;
                let field = match flavour {
                    SplitFlavour::Prose => handle.doc.main.as_ref(),
                    SplitFlavour::Synopsis => handle.doc.synopsis.as_ref(),
                }?;
                Some((id, field.doc.clone()))
            })
            .collect()
    }

    /// This row's door to the comment feature, for the surface `flavour` names.
    ///
    /// A stream row is a perfectly ordinary commentable editor: `row_doc` opens
    /// through the **shared** store, so the row and a standalone tab on the same
    /// item are one document, and the binding it mints is the same one that tab
    /// would get. What a row genuinely cannot have is a *view-state* binding —
    /// "the caret of this tab" has no answer with twelve editors on the page — and
    /// the two were conflated when the streams were first written, which is why
    /// comments were washed into stream prose that offered no way to read them.
    pub fn row_comments(&self, id: u64, flavour: SplitFlavour) -> Option<CommentBinding> {
        let doc = self.row_doc(id)?;
        match flavour {
            SplitFlavour::Prose => doc.comment_binding_main(),
            SplitFlavour::Synopsis => doc.comment_binding_synopsis(),
        }
    }

    /// Any row's comments view-model, for a page that needs to watch the store but
    /// whose container has no commentable surface of its own (a Part or a Book has
    /// no prose). Every binding on the page shares one view-model, so the first
    /// that resolves answers for all of them.
    pub fn row_comments_any_view_model(
        &self,
        flavour: SplitFlavour,
    ) -> Option<crate::comments::CommentsViewModel> {
        self.inner
            .rows
            .ids()
            .into_iter()
            .find_map(|id| self.row_comments(id, flavour))
            .map(|b| b.view_model())
            // A container with no rows *yet* — a freshly created Chapter, a Part
            // whose scenes have not been written — has no binding to ask, but its
            // page still has to watch the comment store: rows and their comments
            // arrive later, and whatever subscribed at build time is all this page
            // will ever have. Fall back to the store's project-wide handle, which
            // exists from `App::build` onward and does not depend on a row.
            .or_else(|| self.inner.docs.comments())
    }

    /// Does any row on this page carry a live comment?
    ///
    /// The page-level question behind the gutter reservation — see
    /// [`ColumnWithMargin::reserve`](crate::comments::pane::ColumnWithMargin::reserve).
    /// Walks the rows rather than the comment store because it is the *rows on this
    /// page* that decide, and a container's stream is a small slice of a project's
    /// comments.
    pub fn any_row_has_comments(&self, flavour: SplitFlavour) -> bool {
        self.inner.rows.ids().into_iter().any(|id| {
            self.row_comments(id, flavour)
                .is_some_and(|b| b.has_live_cards())
        })
    }

    pub fn row_title(&self, id: u64) -> Signal<String> {
        match self.handle(id) {
            Some(h) => h.probe.title(),
            None => Signal::new(String::new()),
        }
    }

    pub fn row_label(&self, id: u64) -> Signal<String> {
        match self.handle(id) {
            Some(h) => h
                .probe
                .dto_signal()
                .map(|d| d.as_ref().map(|x| x.label.clone()).unwrap_or_default()),
            None => Signal::new(String::new()),
        }
    }

    /// The row's tag ids, live.
    ///
    /// Read from the row's own probe rather than baked into `StreamRow`, because
    /// `StreamRowsModel` deliberately does not watch `BinderItem(Updated)` — a baked field
    /// would go stale the moment a tag was assigned. This is the same shape as
    /// [`row_label`](Self::row_label), and for the same reason.
    pub fn row_tags(&self, id: u64) -> Signal<Vec<u64>> {
        match self.handle(id) {
            Some(h) => h
                .probe
                .dto_signal()
                .map(|d| d.as_ref().map(|x| x.tags.clone()).unwrap_or_default()),
            None => Signal::new(Vec::new()),
        }
    }

    /// This project's workflow ladder.
    ///
    /// Built from `app_ctx` + `ids` rather than threaded from the `WorkSession`, and that is
    /// safe *here* only because `StatusesViewModel` caches nothing: `ladder()` reads through
    /// the relationship on every call, so this handle and the session's can never disagree
    /// about what the rungs are. The one thing it does not share is the session handle's
    /// `revision` signal, which nothing on a stream row binds to — a row rebuilds on its own
    /// `BinderItem::Updated`, and the picker's list is rebuilt with it.
    pub fn statuses(&self) -> crate::statuses::StatusesViewModel {
        crate::statuses::StatusesViewModel::new(self.inner.app_ctx.clone(), self.inner.ids.clone())
    }

    /// The row's current rung, live. `None` is "no status" — and so is a rung that no
    /// longer resolves, since the reference is weak by design.
    pub fn row_status(&self, id: u64) -> Signal<Option<u64>> {
        match self.handle(id) {
            Some(h) => h
                .probe
                .dto_signal()
                .map(|d| d.as_ref().and_then(|x| x.status)),
            None => Signal::new(None),
        }
    }

    /// Persist a row's tag ids.
    ///
    /// Tags are a *relationship*, not a scalar on the DTO, so this cannot go through
    /// `update_binder_item` the way [`set_row_label`](Self::set_row_label) does — it writes
    /// through the probe's `set_tags`, which issues the relationship command.
    pub fn set_row_tags(&self, id: u64, tags: &[u64]) {
        if let Some(h) = self.handle(id) {
            let _ = h.probe.set_tags(tags, self.stack());
        }
    }

    /// Get-or-open the row's handle. Opening refs the document in the store; the ref is
    /// dropped when the row leaves the stream (see [`wire`](Self::wire)) or the tab
    /// closes (see `Drop for Inner`).
    fn handle(&self, id: u64) -> Option<RowHandle> {
        if let Some(h) = self.inner.row_handles.borrow().get(&id) {
            return Some(h.clone());
        }
        let doc = self.inner.docs.open(id)?;
        let probe = SingleBinderItem::new(self.inner.app_ctx.clone());
        probe.set_id(Some(id));
        let h = RowHandle {
            sub_role: doc.sub_role.clone(),
            doc,
            probe,
        };
        self.inner.row_handles.borrow_mut().insert(id, h.clone());
        Some(h)
    }

    // ── gating (what the row menus offer) ──

    /// This row carries scene prose that can be cut in two.
    pub fn can_split(&self, id: u64) -> bool {
        can_split_row(&self.rows(), id)
    }

    /// This row can be merged into the previous one.
    pub fn can_merge_into_previous(&self, id: u64) -> bool {
        can_merge_row(&self.rows(), id)
    }

    pub fn can_move_up(&self, id: u64) -> bool {
        matches!(self.row_pos(id), Some(p) if p > 0)
    }
    pub fn can_move_down(&self, id: u64) -> bool {
        let ids = self.inner.rows.ids();
        matches!(ids.iter().position(|&x| x == id), Some(p) if p + 1 < ids.len())
    }

    // ── dialog entry points (present an InputDialog, apply on OK) ──

    pub fn begin_rename_container(&self, ctx: &mut EventContext) {
        let current = self.inner.container_probe.title().get();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_rename()))
            .default_text(current)
            .on_result(move |r, ctx| {
                if let Some(name) = r
                    && !name.trim().is_empty()
                {
                    vm.rename_container(ctx, name.trim());
                }
            })
            .present(ctx);
    }

    pub fn begin_rename_row(&self, ctx: &mut EventContext, id: u64) {
        let current = self.row_title(id).get();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_rename()))
            .default_text(current)
            .on_result(move |r, ctx| {
                if let Some(name) = r
                    && !name.trim().is_empty()
                {
                    vm.rename_row(ctx, id, name.trim());
                }
            })
            .present(ctx);
    }

    pub fn begin_set_label(&self, ctx: &mut EventContext, id: u64) {
        let current = self.row_label(id).get();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_set_label()))
            .default_text(current)
            .on_result(move |r, ctx| {
                if let Some(label) = r {
                    vm.set_row_label(ctx, id, label.trim());
                }
            })
            .present(ctx);
    }

    /// What "Insert …" on row `id` is called. It names what the row's action creates
    /// (a part heading recommends a chapter, a scene another scene), and the dialog it
    /// opens is titled the same. `None` when the row has nothing to insert.
    pub fn insert_label(&self, id: u64) -> Option<LocalizedString> {
        let row = self.rows().into_iter().find(|r| r.item_id == id);
        crate::binder::create_labels::insert_action_label(
            row.and_then(|r| binder_ops::recommended_type(&r.role, &r.sub_role)),
        )
    }

    /// What the "Add …" button at the end of the stream is called: what the container
    /// recommends, a chapter in a part or a book and a scene in a chapter. The dialog it
    /// opens is titled the same.
    pub fn add_label(&self) -> Option<LocalizedString> {
        crate::binder::create_labels::add_action_label(binder_ops::recommended_type(
            &BinderItemRole::Folder,
            &self.inner.container_sub_role,
        ))
    }

    /// Ask for the new row's name and insert it after row `id`. An empty name is a
    /// choice, not a cancel: see
    /// [`name_dialog`](crate::binder::create_labels::name_dialog).
    pub fn begin_insert_after(&self, ctx: &mut EventContext, id: u64) {
        if !self.structure_unlocked() {
            return;
        }
        let Some(label) = self.insert_label(id) else {
            return;
        };
        let vm = self.clone();
        crate::binder::create_labels::name_dialog(label)
            .on_result(move |r, ctx| {
                if let Some(name) = r {
                    vm.insert_after(ctx, id, name.trim());
                }
            })
            .present(ctx);
    }

    /// Ask for the new row's name and add it at the end of the stream. An empty name
    /// is a choice, not a cancel: see
    /// [`name_dialog`](crate::binder::create_labels::name_dialog).
    pub fn begin_add_row(&self, ctx: &mut EventContext) {
        if !self.structure_unlocked() {
            return;
        }
        let Some(label) = self.add_label() else {
            return;
        };
        let vm = self.clone();
        crate::binder::create_labels::name_dialog(label)
            .on_result(move |r, ctx| {
                if let Some(name) = r {
                    vm.add_row(ctx, name.trim());
                }
            })
            .present(ctx);
    }

    // ── apply methods (undoable backend calls; unit-tested) ──

    /// Rename the container. Through the single, so its title `Content` row follows the
    /// entity field — one title, two homes (see [`SingleBinderItem`]).
    pub fn rename_container(&self, _ctx: &mut EventContext, title: &str) {
        let _ = self.inner.container_probe.set_title(title, self.stack());
    }

    pub fn rename_row(&self, _ctx: &mut EventContext, id: u64, title: &str) {
        if let Some(h) = self.handle(id) {
            let _ = h.probe.set_title(title, self.stack());
        }
    }

    pub fn set_row_label(&self, _ctx: &mut EventContext, id: u64, label: &str) {
        if binder_ops::set_labels(&self.inner.app_ctx, self.stack(), &[id], label) > 0
            && let Some(h) = self.handle(id)
        {
            h.probe.set_id(Some(id));
        }
    }

    /// Create the recommended type after `id`, placed by the **relation** the model
    /// recommends. Not "right after the anchor, at the anchor's own indent": in a Full
    /// Part / Full Book stream the anchor may be a chapter head, whose default
    /// recommendation is `Child`, and that shortcut would drop the new scene *outside*
    /// the chapter it was added to.
    pub fn insert_after(&self, _ctx: &mut EventContext, id: u64, title: &str) {
        if !self.structure_unlocked() {
            return;
        }
        let Some(row) = self.rows().into_iter().find(|r| r.item_id == id) else {
            return;
        };
        binder_ops::create_by_recommendation(
            &self.inner.app_ctx,
            &self.inner.ids,
            id,
            &row.role,
            &row.sub_role,
            title,
        );
    }

    /// Append what the container recommends (see [`add_label`](Self::add_label)) at the
    /// end of the stream: a chapter after the last chapter of a part, a scene after the
    /// last scene of a chapter. Placed by `binder_ops::append_to_stream`.
    pub fn add_row(&self, _ctx: &mut EventContext, title: &str) {
        if !self.structure_unlocked() {
            return;
        }
        binder_ops::append_to_stream(
            &self.inner.app_ctx,
            &self.inner.ids,
            self.inner.container_id,
            &self.inner.rows.ids(),
            self.inner.level,
            title,
        );
    }

    pub fn move_row_up(&self, _ctx: &mut EventContext, id: u64) {
        if !self.structure_unlocked() {
            return;
        }
        let ids = self.inner.rows.ids();
        let Some(pos) = ids.iter().position(|&x| x == id) else {
            return;
        };
        if pos == 0 {
            return;
        }
        binder_ops::move_relative(
            &self.inner.app_ctx,
            &self.inner.ids,
            id,
            ids[pos - 1],
            MovePlace::Before,
        );
    }

    pub fn move_row_down(&self, _ctx: &mut EventContext, id: u64) {
        if !self.structure_unlocked() {
            return;
        }
        let ids = self.inner.rows.ids();
        let Some(pos) = ids.iter().position(|&x| x == id) else {
            return;
        };
        if pos + 1 >= ids.len() {
            return;
        }
        binder_ops::move_relative(
            &self.inner.app_ctx,
            &self.inner.ids,
            id,
            ids[pos + 1],
            MovePlace::After,
        );
    }

    /// Merge this row into the previous one. The backend concatenates **both** writing
    /// roles, so the survivor must reload both — the old chapter view reloaded only the
    /// prose and left a stale synopsis on screen.
    pub fn merge_into_previous(&self, ctx: &mut EventContext, id: u64) {
        if !self.structure_unlocked() || !self.can_merge_into_previous(id) {
            return;
        }
        let rows = self.rows();
        let Some(pos) = rows.iter().position(|r| r.item_id == id) else {
            return;
        };
        let prev_id = rows[pos - 1].item_id;
        let (Some(prev), Some(cur)) = (self.row_doc(prev_id), self.row_doc(id)) else {
            return;
        };
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return; // no project open
        };

        // Merge reads content from the store — flush both rows' docs first.
        let stack = self.stack();
        let _ = prev.flush(stack);
        let _ = cur.flush(stack);
        // The comments on the absorbed row's text go with it, and the model the reload
        // below re-anchors from knows where they went.
        let _ = binder_ops::merge_scenes_carrying_comments(
            &self.inner.app_ctx,
            stack,
            &MergeTwoScenesDto {
                work_id,
                target_id: prev_id,
                source_id: id,
            },
        );
        self.reread_comments();
        // `prev` absorbed `id`'s prose *and* synopsis — reflect both in the (reused)
        // editors. `set_djot` only queues a document event, so pump a frame for the
        // editors the user did not touch.
        prev.reload();
        ctx.request_frame();
    }

    /// Split the row at `caret` into two. `which` says which editor the caret is in:
    /// that role is cut there, and the *other* role is passed whole to the source and
    /// empty to the new scene — which is exactly "it stays entirely on the original
    /// scene". See `split_scene_uc`, which applies both roles uniformly.
    pub fn split_row(&self, ctx: &mut EventContext, id: u64, which: SplitFlavour, caret: usize) {
        if !self.structure_unlocked() || !self.can_split(id) {
            return;
        }
        let Some(doc) = self.row_doc(id) else { return };
        let stack = self.stack();
        let _ = doc.flush(stack);

        let (prose, synopsis) = (doc.main.as_ref(), doc.synopsis.as_ref());
        // The comments on the moved words go with them, read off the live anchors of the
        // text being cut before anything changes it.
        let moved = |binding: Option<CommentBinding>, from: usize| {
            binding
                .map(|b| binder_ops::comments_moved_by_split(&b.live(), from))
                .unwrap_or_default()
        };
        let (before_text, after_text, before_synopsis, after_synopsis, role, comments, split) =
            match which {
                SplitFlavour::Prose => {
                    let Some(m) = prose else { return };
                    let Ok(split) = split_djot(&m.doc, caret) else {
                        return;
                    };
                    let whole = synopsis.map(|s| s.djot()).unwrap_or_default();
                    let comments = moved(doc.comment_binding_main(), split.moved_from);
                    (
                        split.before.clone(),
                        split.after.clone(),
                        whole,
                        String::new(),
                        ContentRole::SceneText,
                        comments,
                        split,
                    )
                }
                SplitFlavour::Synopsis => {
                    let Some(s) = synopsis else { return };
                    let Ok(split) = split_djot(&s.doc, caret) else {
                        return;
                    };
                    let whole = prose.map(|m| m.djot()).unwrap_or_default();
                    let comments = moved(doc.comment_binding_synopsis(), split.moved_from);
                    (
                        whole,
                        String::new(),
                        split.before.clone(),
                        split.after.clone(),
                        ContentRole::SynopsisText,
                        comments,
                        split,
                    )
                }
            };

        let _ = binder_ops::split_scene_carrying_comments(
            &self.inner.app_ctx,
            &self.inner.ids,
            stack,
            &SplitSceneDto {
                source_id: id,
                before_text,
                after_text,
                before_synopsis,
                after_synopsis,
                new_title: tr!(new_scene_title()).resolve_now(),
            },
            role,
            &comments,
            &split,
        );
        if !comments.is_empty() {
            self.reread_comments();
        }
        // `id` now holds only the before-halves — reflect them in the (reused) editors,
        // pumping a frame so the queued document events are drained.
        doc.reload();
        ctx.request_frame();
    }

    pub fn trash_row(&self, _ctx: &mut EventContext, id: u64) {
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return; // no project open
        };
        // The origin binder is resolved by the use case; the stream does not
        // need to locate the row first.
        let _ = trash_management_commands::trash_selection(
            &self.inner.app_ctx,
            self.stack(),
            &TrashSelectionDto {
                work_id,
                binder_ids: Vec::new(),
                binder_item_ids: vec![id],
            },
        );
    }

    // ── helpers ──

    /// Have the comments model read where every comment is now, before a reload
    /// re-anchors the reloaded text's comments from it (see `CommentsListModel::reread`).
    fn reread_comments(&self) {
        if let Some(comments) = self.inner.docs.comments() {
            comments.model().reread();
        }
    }

    fn stack(&self) -> Option<u64> {
        self.inner.ids.stack_id.get()
    }

    fn row_pos(&self, id: u64) -> Option<usize> {
        self.inner.rows.ids().iter().position(|&x| x == id)
    }
}

/// Can the row `id` be cut in two? Only a prose-bearing row has anything to split, and
/// the backend rejects anything else.
///
/// Pure over the row list so the rule is unit-testable without a backend.
fn can_split_row(rows: &[StreamRow], id: u64) -> bool {
    rows.iter()
        .any(|r| r.item_id == id && is_prose_bearing(&r.role, &r.sub_role))
}

/// Can the row `id` be merged **into the previous row**? Three conditions, all necessary:
///
/// 1. it is prose-bearing — there is something to merge;
/// 2. it does not open a chapter/part/book — merging it away would delete that boundary
///    (and, for a chapter *folder*, orphan its child scenes);
/// 3. the immediately preceding **row** is prose-bearing — which is what stops a scene
///    being merged backwards across the heading that starts its chapter, now that a Full
///    Part / Full Book stream shows several chapters at once.
///
/// The backend enforces the same invariants; this is what hides the menu item.
/// Pure over the row list so the rule is unit-testable without a backend.
fn can_merge_row(rows: &[StreamRow], id: u64) -> bool {
    let Some(pos) = rows.iter().position(|r| r.item_id == id) else {
        return false;
    };
    if pos == 0 {
        return false;
    }
    let (this, prev) = (&rows[pos], &rows[pos - 1]);
    is_prose_bearing(&this.role, &this.sub_role)
        && !opens_a_section(&this.sub_role)
        && is_prose_bearing(&prev.role, &prev.sub_role)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Only the backend-seeding tests use it, and they are compiled out under `mocks`.
    #[cfg(not(feature = "mocks"))]
    use frontend::commands::binder_item_commands;
    use frontend::common::entities::BinderItemRole::{Folder, Item};
    use frontend::common::entities::BinderItemSubRole::{Book, ChapterScene, Note, Part, Scene};

    fn vm(role: BinderItemRole, sub_role: BinderItemSubRole) -> Option<StreamViewModel> {
        let ctx = Rc::new(AppContext::new());
        let docs = OpenDocsStore::new(ctx.clone());
        StreamViewModel::new(ctx, AppIds::new(), docs, 1, &role, &sub_role)
    }

    /// Only folder containers host a stream — a flat chapter keeps its dual-pane editor.
    /// `ContentTab::new` gates on the same `StreamLevel::for_container`, so the two
    /// surfaces cannot drift apart.
    #[test]
    fn only_folder_containers_get_a_view_model() {
        assert_eq!(
            vm(Folder, ChapterScene).map(|v| v.level()),
            Some(StreamLevel::Chapter)
        );
        assert_eq!(vm(Folder, Part).map(|v| v.level()), Some(StreamLevel::Part));
        assert_eq!(vm(Folder, Book).map(|v| v.level()), Some(StreamLevel::Book));
        assert!(
            vm(Item, ChapterScene).is_none(),
            "the flat chapter is not a container"
        );
        assert!(vm(Item, Scene).is_none());
        assert!(vm(Folder, Note).is_none());
    }

    // The `is_prose_bearing` / `is_synopsis_bearing` / `opens_a_section` / `split_djot`
    // tests moved to `shared::binder_ops` along with the functions themselves. What
    // stays here is what stays here: the `can_split_row` / `can_merge_row` row-list rules,
    // which are the stream's own.

    fn row(item_id: u64, role: BinderItemRole, sub_role: BinderItemSubRole) -> StreamRow {
        StreamRow {
            item_id,
            role,
            sub_role,
            number: None,
            fallback_label: None,
            reading: None,
        }
    }

    /// A Full Book stream over: Part(1) → chapter folder(2) → scenes 3,4 → flat
    /// chapter(5) → scene 6. The merge rule has to hold across every adjacency in it.
    fn book_rows() -> Vec<StreamRow> {
        vec![
            row(1, Folder, Part),
            row(2, Folder, ChapterScene),
            row(3, Item, Scene),
            row(4, Item, Scene),
            row(5, Item, ChapterScene),
            row(6, Item, Scene),
        ]
    }

    #[test]
    fn merge_is_offered_only_between_adjacent_prose_rows() {
        let rows = book_rows();
        // Scene after scene, inside one chapter: the ordinary case.
        assert!(can_merge_row(&rows, 4));
        // The chapter's *first* scene may merge into the chapter folder itself — the
        // folder carries prose, so this is the exact inverse of splitting it.
        assert!(can_merge_row(&rows, 3));
    }

    #[test]
    fn merge_never_crosses_a_structural_boundary() {
        let rows = book_rows();
        // Scene 6 is the first scene of the flat chapter 5 — and 5 *is* prose-bearing,
        // so this merges into its own chapter head, not across a boundary. Allowed.
        assert!(can_merge_row(&rows, 6));
        // But chapter 5 itself must NOT be merged into scene 4: it opens a chapter, and
        // merging it away would delete that boundary and swallow the chapter title.
        assert!(
            !can_merge_row(&rows, 5),
            "a chapter head must never be merged away"
        );
        // Nor may the chapter folder be merged into the part heading above it.
        assert!(
            !can_merge_row(&rows, 2),
            "a chapter folder must never be merged away (it would orphan its scenes)"
        );
        // The first row has nothing before it.
        assert!(!can_merge_row(&rows, 1));
        // A part heading carries no prose at all.
        let flat = vec![row(1, Folder, Part), row(2, Folder, Part)];
        assert!(!can_merge_row(&flat, 2));
    }

    #[test]
    fn split_is_offered_only_on_prose_rows() {
        let rows = book_rows();
        assert!(can_split_row(&rows, 3), "a scene splits");
        assert!(can_split_row(&rows, 5), "a flat chapter splits");
        assert!(
            can_split_row(&rows, 2),
            "a chapter folder's own prose splits — the matrix gives it a SceneText"
        );
        assert!(
            !can_split_row(&rows, 1),
            "a part heading carries no prose to split"
        );
        assert!(!can_split_row(&rows, 999), "unknown row");
    }

    /// "Add new chapter" on a Full Book, answered with OK and no name, adds an untitled
    /// chapter; answered with Cancel, adds nothing. The dialog is titled with the
    /// button's own words. A tester's "Add new chapter" opened "New scene", and an
    /// empty OK used to be read as Cancel (2026-11-24).
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn add_new_chapter_with_no_name_adds_an_untitled_chapter() {
        use teksilo::core::ModalContent;
        use teksilo::core::accessibility::widget_id_to_node_id;
        use teksilo::core::widget_id::WidgetId;
        use teksilo::core::widget_tree::WidgetTree;

        let project = crate::test_support::RealProject::empty_novel();
        let ctx = project.app_ctx.clone();
        let book = binder_item_commands::get_all_binder_item(&ctx)
            .expect("items")
            .into_iter()
            .find(|it| it.sub_role == Book && it.activated)
            .expect("the empty novel has a Book")
            .id;
        let docs = OpenDocsStore::new(ctx.clone());
        let vm = StreamViewModel::new(ctx.clone(), project.ids.clone(), docs, book, &Folder, &Book)
            .expect("a Book hosts a stream");
        let chapters = || -> Vec<(u64, String)> {
            binder_item_commands::get_all_binder_item(&ctx)
                .expect("items")
                .into_iter()
                .filter(|it| it.sub_role == ChapterScene && it.activated)
                .map(|it| (it.id, it.title))
                .collect()
        };

        fn buttons(tree: &WidgetTree, root: WidgetId, out: &mut Vec<WidgetId>) {
            if tree
                .widget_type_name(root)
                .is_some_and(|t| t.ends_with("Button"))
            {
                out.push(root);
            }
            for c in tree.children(root) {
                buttons(tree, c, out);
            }
        }
        // Press "Add new chapter", check the dialog's title, and click its OK (the last
        // footer button) or its Cancel (the first) without typing anything.
        let answer = |ok: bool| {
            let mut tree = WidgetTree::new();
            let v = vm.clone();
            crate::test_support::press(&mut tree, move |c| v.begin_add_row(c));
            let request = tree
                .drain_pending_modal_requests()
                .pop()
                .expect("the dialog appears")
                .request;
            assert_eq!(
                request.title,
                Some(tr!(add_chapter()).resolve_now()),
                "titled with the button's words"
            );
            let ModalContent::Deferred(builder) = request.content else {
                panic!("InputDialog presents deferred content");
            };
            let content = builder(&mut tree);
            tree.layout(SizeProposal::exact(420.0, 180.0));
            let mut found = Vec::new();
            buttons(&tree, content, &mut found);
            let target = if ok { found.last() } else { found.first() };
            let target = *target.expect("the dialog has its buttons");
            tree.dispatch_event(WidgetEvent::AccessAction {
                action: teksilo::core::accesskit::Action::Click,
                target: Some(target),
                target_node: widget_id_to_node_id(target),
                data: None,
            });
        };

        let before = chapters();
        answer(false);
        assert_eq!(chapters(), before, "Cancel adds nothing");

        answer(true);
        let after = chapters();
        assert_eq!(
            after.len(),
            before.len() + 1,
            "OK with no name adds a chapter"
        );
        let added = after
            .iter()
            .find(|c| !before.contains(c))
            .expect("the new chapter");
        assert_eq!(
            added.1, "",
            "untitled, so the binder and the book number it"
        );
    }

    /// A Book offers every level, a Part its chapters and scenes, a Chapter nothing: a
    /// filter there could only empty the page.
    #[test]
    fn a_container_offers_the_levels_it_holds() {
        let book = vm(Folder, Book).expect("a book hosts a stream");
        assert_eq!(book.depth_filters_offered().len(), 3);
        let part = vm(Folder, Part).expect("a part hosts a stream");
        assert_eq!(part.depth_filters_offered().len(), 2);
        let chapter = vm(Folder, ChapterScene).expect("a chapter folder hosts a stream");
        assert!(chapter.depth_filters_offered().is_empty());
        chapter.set_depth_filter(DepthFilter::DownToChapters);
        assert_eq!(
            chapter.depth_filter(),
            DepthFilter::Everything,
            "a chapter cannot hide its own scenes"
        );
        assert!(!chapter.filter_on().get());
    }

    /// While levels are hidden the banner shows and the structural actions are off. A
    /// reading shows every row, so neither while it is on; the synopsis page shows the
    /// working rows, so both again there; and showing everything puts both back.
    #[test]
    fn hidden_levels_turn_the_banner_on_and_the_structural_actions_off() {
        let book = vm(Folder, Book).expect("a book hosts a stream");
        assert!(!book.filter_on().get() && book.structure_editable().get());

        book.set_depth_filter(DepthFilter::DownToChapters);
        assert!(book.filter_on().get() && !book.structure_editable().get());

        book.set_reading_on(true);
        assert!(
            !book.filter_on().get() && book.structure_editable().get(),
            "a reading shows every row it prints"
        );
        book.show_flavour(SplitFlavour::Synopsis);
        assert!(
            book.filter_on().get() && !book.structure_editable().get(),
            "the synopsis page is never a reading, so the filter is on there"
        );
        book.show_flavour(SplitFlavour::Prose);
        book.set_reading_on(false);
        assert!(book.filter_on().get());

        book.set_depth_filter(DepthFilter::Everything);
        assert!(!book.filter_on().get() && book.structure_editable().get());
    }

    /// A remembered filter comes back by its string, and anything else shows every
    /// level.
    #[test]
    fn a_remembered_filter_is_restored_by_its_string() {
        let book = vm(Folder, Book).expect("a book hosts a stream");
        book.seed_depth_filter(Some("parts"));
        assert_eq!(book.depth_filter(), DepthFilter::PartsOnly);
        book.seed_depth_filter(Some("an older build's word"));
        assert_eq!(book.depth_filter(), DepthFilter::Everything);
        let part = vm(Folder, Part).expect("a part hosts a stream");
        part.seed_depth_filter(Some("parts"));
        assert_eq!(
            part.depth_filter(),
            DepthFilter::Everything,
            "a Part has no parts to show alone"
        );
    }

    /// The mocks' book: a part, a chapter folder with three scenes, a flat chapter with
    /// one. Hiding the scenes keeps the part and both chapters, each where it was; parts
    /// alone keep the part; showing everything brings every row back.
    #[cfg(feature = "mocks")]
    #[test]
    fn a_filter_hides_the_deeper_rows_and_moves_none_that_stay() {
        let book = vm(Folder, Book).expect("a book hosts a stream");
        let ids = |v: &StreamViewModel| v.rows().iter().map(|r| r.item_id).collect::<Vec<_>>();
        assert_eq!(ids(&book), vec![301, 104, 201, 202, 203, 302, 303]);
        assert_eq!(book.deepest().get(), 3);

        book.set_depth_filter(DepthFilter::DownToChapters);
        assert_eq!(ids(&book), vec![301, 104, 302]);
        assert_eq!(
            [301, 104, 302].map(|id| book.row_depth(id).get()),
            [1, 2, 2],
            "no row that stays moves"
        );
        assert_eq!(
            book.deepest().get(),
            2,
            "the deepest level shown is the chapters'"
        );

        book.set_depth_filter(DepthFilter::PartsOnly);
        assert_eq!(ids(&book), vec![301]);
        assert_eq!(book.deepest().get(), 1);

        book.set_depth_filter(DepthFilter::Everything);
        assert_eq!(ids(&book), vec![301, 104, 201, 202, 203, 302, 303]);
        assert_eq!(book.deepest().get(), 3);
    }

    /// The rows a row sits under, nearest first: where a hidden passage hands over to.
    #[cfg(feature = "mocks")]
    #[test]
    fn a_rows_parents_are_the_rows_it_sits_under_nearest_first() {
        let book = vm(Folder, Book).expect("a book hosts a stream");
        assert_eq!(book.row_parents(303), vec![302, 301]);
        assert_eq!(book.row_parents(202), vec![104, 301]);
        assert_eq!(book.row_parents(302), vec![301]);
        assert!(
            book.row_parents(301).is_empty(),
            "a part sits under nothing"
        );
        assert!(
            book.row_parents(1).is_empty(),
            "the container's own text is not a row"
        );
    }

    /// **A structural action does nothing while levels are hidden**, however it is
    /// reached: the menu items are greyed, and this is the guard behind them. Once
    /// every level is shown, the same action works.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn structural_actions_wait_until_every_level_is_shown() {
        use crate::test_support::RealProject;
        let project = RealProject::empty_novel();
        let order = |p: &RealProject| -> Vec<u64> {
            crate::models::binder_stream::ordered_binder_items(&p.app_ctx, p.work_id)
                .into_iter()
                .map(|r| r.id)
                .collect()
        };
        let folder_of = |p: &RealProject, wanted: BinderItemSubRole| -> u64 {
            order(p)
                .into_iter()
                .find(|id| {
                    binder_item_commands::get_binder_item(&p.app_ctx, id)
                        .ok()
                        .flatten()
                        .is_some_and(|it| it.role == Folder && it.sub_role == wanted)
                })
                .expect("the novel has this folder")
        };
        let book_id = folder_of(&project, Book);
        let chapter = folder_of(&project, ChapterScene);
        let book = StreamViewModel::new(
            project.app_ctx.clone(),
            project.ids.clone(),
            OpenDocsStore::new(project.app_ctx.clone()),
            book_id,
            &Folder,
            &Book,
        )
        .expect("a book hosts a stream");
        book.inner.rows.refill();
        assert_eq!(book.rows().len(), 2, "fixture: a chapter and its scene");

        book.set_depth_filter(DepthFilter::DownToChapters);
        assert_eq!(book.rows().len(), 1, "the scene is hidden");
        let before = order(&project);
        {
            let book = book.clone();
            crate::editors::test_support::with_event_context(move |ctx| {
                book.insert_after(ctx, chapter, "Out of sight");
                book.add_row(ctx, "Out of sight too");
                book.move_row_down(ctx, chapter);
            });
        }
        assert_eq!(
            order(&project),
            before,
            "nothing was created or moved while the scenes were hidden"
        );

        book.set_depth_filter(DepthFilter::Everything);
        {
            let book = book.clone();
            crate::editors::test_support::with_event_context(move |ctx| {
                book.insert_after(ctx, chapter, "In sight");
            });
        }
        assert_eq!(
            order(&project).len(),
            before.len() + 1,
            "and the same action works once everything is shown"
        );
    }
}
