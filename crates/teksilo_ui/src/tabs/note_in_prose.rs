// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **"In prose"** segment on an `Item/Note` tab: a writable stream of the manuscript
//! prose this note has been *declared* present in, scoped to one Book at a time.
//!
//! **Its own composition, not the container stream.** `stream_pane` drives itself off
//! `ContentTab::stream()`, which is `Some` only for a Folder container (`StreamLevel::for_container`
//! never matches `Item/Note`); its row list is a container's *contiguous extent*, while
//! membership here is a set of non-adjacent rows scattered across the whole Book; and its
//! `StreamRow` carries no field for *why* a row is present, which is exactly what this
//! segment has to show. So this file reuses the writing primitives
//! ([`shared::writing_column`], [`shared::centered`], [`shared::writing_page_scroll`]) and
//! copies the *shape* of a stream row's caption, without calling into `shared::stream` or
//! [`crate::models::StreamRowsModel`] at all. See `tabs/shared/stream.rs`'s own module doc
//! for the fuller list of reasons that door stays shut.
//!
//! **Declaration only, on purpose.** The membership rule
//! ([`crate::models::declared_rows_in_book`]) never consults
//! [`crate::mentions::MentionIndex`]: a prose name-match does not put a row in this
//! stream, only a writer's own point-of-view or cast pin does. That is what lets this
//! reading make the promise its caption keeps: every row on screen is one the writer put
//! this note in themselves.
//!
//! **No margin lane, no comment cards, in this first cut.** Every row still carries a
//! comment binding and a footnote binding (both plain accessors on the already-open
//! `OpenDoc`, wired for free), so an existing thread still underlines in the text and a
//! footnote reference is still an accessible target; what does not exist here is the
//! *margin* surface ([`crate::margin_lane`]) that would draw a card beside it. Full Book /
//! Part / Chapter get one because [`shared::laned_stream`] maps a single container's
//! contiguous extent; this reading's rows are scattered across the whole Book and are not
//! the shape that helper measures. Wiring a lane over a scattered row set is future work,
//! not attempted here.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{
    Divider, Expand, HStack, Segment, SegmentId, SegmentedControl, Spacer, TextWidget, VStack,
};

use frontend::common::event::{
    BinderItemManagementEvent, DirectAccessEntity, EntityEvent, Origin, TrashManagementEvent,
};

use skribisto_model::SubRoleExt;

use crate::app_ids::AppIds;
use crate::format::FormatViewModel;
use crate::models::{
    BookChoice, Declaration, NoteBookChoiceService, NoteProseRow, OpenDoc, OpenDocsStore,
};

use super::{ContentTab, shared};

/// Build the segment's body from `tab`. `tab.item_id()` is the note itself; this
/// function is called unconditionally by its caller (the `Item/Note` tab's own segment
/// bar), which gates whether the writer can ever *reach* this segment on the note
/// carrying a discoverable tag, the way every other composite pane in
/// [`shared::panes`] trusts the constraint matrix rather than re-checking its own
/// applicability. That gate lives on the segment's chip, not on whether this function
/// runs at all: `Switcher` mounts a page lazily, on first selection, and a hidden chip
/// is never selectable, so building this `Box<dyn Widget>` for a note with no
/// discoverable tag costs a struct literal and never reaches the backend reads inside
/// [`NoteInProseBody::build`].
pub(crate) fn note_in_prose_pane(tab: &ContentTab) -> Box<dyn Widget> {
    let will_show = shared::segment_will_show(tab, shared::segments::SEG_NOTE_IN_PROSE);
    let (area, port, _binding) = shared::writing_page_scroll(tab, will_show);
    let mark_dirty: Rc<dyn Fn()> = Rc::new(tab.mark_dirty_fn());
    // Where each row landed, so the lane can map it. Same machinery as a Full Book
    // stream: this reading is one scroll area over many documents, which is exactly the
    // shape `RowExtents` exists for.
    let extents = crate::margin_lane::RowExtents::new();
    let docs: Rc<RefCell<HashMap<u64, Rc<OpenDoc>>>> = Rc::new(RefCell::new(HashMap::new()));
    let page_scroll = area.scroll_y_signal().clone();
    // One token for this page. Every editor on it is the same surface, and a scene
    // that is also open in a tab of its own must not answer for this reading. See
    // [`crate::margin_lane::LaneScope`].
    let scope = crate::margin_lane::LaneScope::fresh();

    let body = NoteInProseBody {
        app_ctx: tab.app_ctx(),
        ids: tab.ids().clone(),
        note_id: tab.item_id(),
        scope,
        header_width: tab.column_width.clone(),
        editor_width: tab.main_column_width().clone(),
        // The **Scene** bundle, deliberately not `tab.main_typography()`: that accessor
        // resolves to the Notes bundle for an `Item/Note` tab (this tab's own kind), but
        // every row shown here is manuscript prose (a Scene or a chapter folder's own
        // subordinate text), which is Scene-typeset everywhere else in the app.
        typography: tab.typography.scene.clone(),
        mark_dirty,
        format: tab.format.clone(),
        typewriter: tab.typewriter.clone(),
        caret: tab.caret_band(),
        games: tab.writing_games(),
        arrival_project: tab.work_unique_id(),
        tags: tab.tags(),
        extents: extents.clone(),
        page_scroll: page_scroll.clone(),
        selected_book: Signal::new(None),
        generation: Signal::new(0),
        wired: Cell::new(false),
        docs: docs.clone(),
        store: tab.docs(),
        root: None,
    };

    let col = VStack::new()
        .spacing(8.0)
        .child(shared::vspace(12.0))
        .child(body)
        .child(shared::vspace(28.0))
        .child(port);

    // The lane, beside the page and not inside it, exactly as a Full Book stream mounts
    // its own. `LaneSurface::Stream` because that is what this is: many documents on one
    // axis. It therefore carries every provider a stream carries, comments and spelling
    // and boundaries included, which is right -- a writer editing real prose here expects
    // the marks they left on it -- and the story-bible provider on top.
    let opened = docs.clone();
    let row: Rc<dyn Fn(u64) -> Option<crate::margin_lane::LaneRow>> = Rc::new(move |item| {
        let doc = opened.borrow().get(&item).cloned()?;
        let field = doc.main.as_ref()?;
        Some(crate::margin_lane::LaneRow {
            item,
            doc: field.doc.clone(),
            comments: doc.comment_binding_main(),
            spell: doc.spell_main(),
            markers: Default::default(),
        })
    });
    let lane = crate::margin_lane::lane_for(
        &area,
        crate::margin_lane::LaneInputs {
            app_ctx: tab.app_ctx(),
            ids: tab.ids().clone(),
            surface: crate::margin_lane::LaneSurface::Stream,
            kind: crate::format::EditorKind::Prose,
            scope,
            format: tab.format.clone(),
            rows: crate::margin_lane::LaneRows::Placed { extents, row },
        },
    );
    Box::new(
        HStack::new()
            .child(Expand::new().child(area.child(col)))
            .child(lane),
    )
}

/// Every backend origin that can change which rows this note is declared present in, or
/// how they are captioned: a point-of-view/cast pin (`BinderItem` `Updated`), a row
/// created, removed, moved, split, merged, promoted or trashed/restored, and a `Work`
/// update (the numbering settings every ordinal badge here reads). Mirrors
/// `OverviewRowsModel::wire`'s own origin list, minus the `Content` subscription: this
/// reading's membership never depends on prose *text*, only on the relationship and
/// structural fields above, so a keystroke elsewhere on the page must never reload it.
fn reload_origins() -> Vec<Origin> {
    use DirectAccessEntity::BinderItem;
    use EntityEvent::{Created, Removed, Updated};
    vec![
        Origin::DirectAccess(BinderItem(Created)),
        Origin::DirectAccess(BinderItem(Updated)),
        Origin::DirectAccess(BinderItem(Removed)),
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
    ]
}

/// The `SegmentId` a Book's segment is addressed by, derived from its durable uid rather
/// than minted by `Segment::new` (which would allocate `SegmentId::fresh()`, a
/// process-global counter that goes stale the moment this widget rebuilds, exactly the
/// trap `shared::segments::segment_id`'s own doc records). Nothing here persists the
/// number across a restart the way `shared::segments` does; it only has to survive this
/// pane's own rebuilds within one running session, which a value **derived** from the
/// Book's uid does for free: the same input always derives the same id.
fn book_segment_id(book: &BookChoice) -> SegmentId {
    shared::segments::segment_id(&book.uid.to_string())
}

/// A Book's segment label: its title (or the ordinal fallback) with the badge folded in,
/// since a `SegmentedControl` segment takes one plain string rather than the
/// icon-plus-`StructureNumber` pairing a row header can afford.
fn book_label(book: &BookChoice) -> String {
    let (text, badge) =
        crate::models::label_and_badge(&book.title, book.fallback_label.as_deref(), book.number);
    match badge {
        Some(n) => format!("{n}. {text}"),
        None => text,
    }
}

/// The reactive body: the Book picker, the empty states, and the declared rows
/// themselves. Split out from [`note_in_prose_pane`] because only this part needs to
/// re-derive its content on a live signal. The outer scroll area and its remembered
/// position are built once, exactly as every other segment's page is.
struct NoteInProseBody {
    app_ctx: Rc<frontend::AppContext>,
    ids: AppIds,
    note_id: u64,
    /// This page's surface token, minted once by [`note_in_prose_pane`] and carried on
    /// every row's [`crate::margin_lane::LaneAnchor`] so anything resolving an editor by
    /// item reaches *these* rows and not a tab open on the same scene.
    scope: crate::margin_lane::LaneScope,
    header_width: Signal<f32>,
    editor_width: Signal<f32>,
    typography: crate::settings::EditorTypography,
    mark_dirty: Rc<dyn Fn()>,
    format: FormatViewModel,
    typewriter: crate::shared::TypewriterSettings,
    caret: crate::shared::CaretBand,
    games: crate::writing_session::WritingGamesViewModel,
    arrival_project: Option<String>,
    /// This project's palette, for the capture submenu on every row's editor. Held
    /// rather than reached for: it is Tier-2, so `app_state` would answer with whichever
    /// Work registered first.
    tags: crate::tags::TagsViewModel,
    /// Which Book's rows are on screen. Created once, here, and never recreated on a
    /// rebuild of this struct's own `build`: a `Signal` minted inside `build` itself would
    /// lose the writer's choice the moment any of [`reload_origins`] fired. This one
    /// survives because it is a field, set once when [`note_in_prose_pane`] constructs
    /// this widget, exactly as `ContentTab::story_bible_book_filter` survives
    /// `StoryBiblePane`'s own rebuilds by living one level further out.
    /// Where each row landed, published for the lane. See [`crate::margin_lane::rows`].
    extents: crate::margin_lane::RowExtents,
    page_scroll: Signal<f32>,
    selected_book: Signal<Option<SegmentId>>,
    /// Bumped by [`reload_origins`]'s coalesced reload; bound to `BindingLevel::Rebuild`
    /// below purely to give this widget a rebuild trigger of its own; the rows are always
    /// re-read fresh, never diffed against a previous generation.
    generation: Signal<u64>,
    /// Guards the one-time seed of [`Self::selected_book`] and the one-time registration
    /// of the reload subscription: both belong at the *first* build only, the same
    /// `Cell<bool>` guard `OverviewRowsModel::wire` and `StreamRowsModel`'s own `wire`
    /// already use for the identical reason (`ctx.subscribe_event` re-registered on every
    /// build would fire the same event handler once per build, not once per event).
    wired: Cell<bool>,
    /// Every declared row's document, opened through the shared [`OpenDocsStore`] so
    /// editing here edits the real document. Reconciled on every build against the
    /// currently declared set (see [`NoteInProseBody::sync_docs`]); released for good when
    /// this widget drops.
    /// The rows' open documents, shared with the lane beside this page so the two
    /// cannot disagree about which document a row is showing.
    docs: Rc<RefCell<HashMap<u64, Rc<OpenDoc>>>>,
    /// This tab's shared document store, threaded from [`ContentTab::docs`] once, at
    /// construction, exactly as [`Self::ids`]/[`Self::app_ctx`] are. **Not** read from
    /// `ctx.app_state::<OpenDocsStore>()`: that slot resolves to whatever window's
    /// session registered last, which at first launch (no project open yet) is
    /// `startup.rs`'s throwaway `WorkSession`, a second, disjoint `OpenDocsStore` that a
    /// row opened through would silently split from the very document this note's own
    /// "Note" tab is editing. A plain field, not a `RefCell`, because it never changes
    /// after construction, the same shape `Self::ids` already has; [`Drop`] reads it
    /// directly with no `BuildContext` needed.
    store: OpenDocsStore,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for NoteInProseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoteInProseBody").finish_non_exhaustive()
    }
}

/// Release every document this pane still holds open. Mirrors
/// `StreamViewModel`'s own `Inner::drop`, and for the identical reason: without this, a
/// Book with forty declared rows would leak forty `OpenDoc`s every time this segment was
/// abandoned for another one.
impl Drop for NoteInProseBody {
    fn drop(&mut self) {
        let stack = self.ids.stack_id.get();
        for id in self.docs.borrow().keys() {
            self.store.release(*id, stack);
        }
    }
}

impl NoteInProseBody {
    /// `Work.unique_id`, or `None` when there is nothing to key a remembered choice by.
    /// Mirrors `crate::settings::TreeExpansionViewModel::work_uid` exactly, for the same
    /// reason: a brand-new unsaved project has no uid yet, and keying by `""` would make
    /// every such project share one remembered Book.
    fn work_uid(&self) -> Option<String> {
        let work_id = self.ids.work_id.get()?;
        let uid = frontend::commands::work_commands::get_work(&self.app_ctx, &work_id)
            .ok()
            .flatten()?
            .unique_id;
        crate::models::uid_is_usable(&uid).then_some(uid)
    }

    /// Open every newly-declared row and release every row that fell out of the current
    /// set, in one pass. Never on a per-keystroke basis, since this only runs from
    /// `build`, which only reruns on [`reload_origins`] firing or the writer switching
    /// Books.
    fn sync_docs(&self, rows: &[NoteProseRow], store: &OpenDocsStore) {
        let stack = self.ids.stack_id.get();
        let wanted: std::collections::HashSet<u64> = rows.iter().map(|r| r.item_id).collect();
        let mut docs = self.docs.borrow_mut();
        docs.retain(|id, _| {
            if wanted.contains(id) {
                true
            } else {
                store.release(*id, stack);
                false
            }
        });
        for id in wanted {
            if let std::collections::hash_map::Entry::Vacant(slot) = docs.entry(id)
                && let Some(doc) = store.open(id)
            {
                slot.insert(doc);
            }
        }
    }

    fn book_bar(&self, books: &[BookChoice]) -> SegmentedControl {
        let mut bar = SegmentedControl::new(self.selected_book.clone());
        for book in books {
            bar = bar.segment(Segment::new(lit!(book_label(book))).id(book_segment_id(book)));
        }
        bar
    }

    /// One declared row: its caption (icon, ordinal badge, title, the declaration that
    /// put it here) and its live editor. No split, no merge, no add, no per-row menu: a
    /// scene does not belong to a character, so this reading offers none of the verbs a
    /// manuscript stream does.
    fn row_widget(&self, row: &NoteProseRow, doc: &Rc<OpenDoc>) -> impl Widget + 'static {
        let is_heading = row.sub_role.opens_chapter() || row.sub_role.opens_part();
        let (title_text, badge) =
            crate::models::label_and_badge(&row.title, row.fallback_label.as_deref(), row.number);
        let (style, color) = if is_heading {
            (TextStyleRole::BodyBold, TextRole::Primary)
        } else {
            (TextStyleRole::SmallBold, TextRole::Secondary)
        };

        let header = HStack::new()
            .spacing(8.0)
            .child(crate::binder::icons::sub_role_icon(&row.sub_role).icon_size(14.0))
            .child(crate::widgets::StructureNumber::new(badge))
            .child(TextWidget::new(lit!(title_text)).style(style).color(color))
            .child(Expand::horizontal().child(Spacer::new()))
            .child(declaration_indicator(row.declaration));

        let mut col = VStack::new()
            .spacing(4.0)
            .child(shared::vspace(if is_heading { 22.0 } else { 10.0 }))
            .child(shared::centered(header, &self.header_width))
            .child(shared::vspace(4.0))
            .child(shared::centered(
                Expand::horizontal().child(Divider::new()),
                &self.header_width,
            ));

        if let Some(field) = doc.main.as_ref() {
            let mark_dirty = self.mark_dirty.clone();
            let on_change = move || mark_dirty();
            let min_lines = if is_heading {
                shared::HEADING_PROSE_MIN_LINES
            } else {
                shared::MAIN_MIN_LINES
            };
            col = col.child(shared::writing_column(
                &field.doc,
                &self.editor_width,
                &self.typography,
                min_lines,
                on_change,
                // No split: a scene does not belong to a character, so "split scene"
                // has no meaning on this reading of it.
                None,
                // No find banner: this is a stream of rows, not the tab's own single
                // focused editor, same reasoning `shared::stream`'s own rows use.
                None,
                doc.spell_main(),
                doc.replacement_main(),
                Some(self.format.clone()),
                Some(self.typewriter.clone()),
                Some(self.caret.clone()),
                Some(self.games.clone()),
                // No view-state binding: with several rows on the page there is no
                // single "the" caret for this tab to remember, the same reason a
                // manuscript stream's own rows take none.
                None,
                // Comment highlighting still lights up in the text (see the module
                // doc); the gutter stays at its default zero, since there is no
                // margin lane here to reserve one for.
                doc.comment_binding_main(),
                doc.footnote_binding_main(),
                doc.images(),
                self.arrival_project.clone(),
                doc.trashed.get(),
                Some(crate::margin_lane::LaneAnchor::new(row.item_id, self.scope)),
                // Below-the-fold rows: estimate height before layout, exactly as
                // every stream row does, so the page's scroll extent is not off by
                // an order of magnitude before the first paint settles.
                true,
                // Every row here is manuscript prose in a real project, so the
                // capture submenu is available from it like any other editor.
                Some(self.tags.clone()),
            ));
        }
        col
    }
}

impl Widget for NoteInProseBody {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.selected_book
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        self.generation
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let book_service = ctx.app_state::<NoteBookChoiceService>().cloned();

        let Some(work_id) = self.ids.work_id.get() else {
            self.root = None;
            return Vec::new();
        };

        let books = crate::models::books_in_work(&self.app_ctx, work_id);

        // **Whose names the lane marks**, published while this reading is on screen and
        // withdrawn when it goes away. The lane's provider is registered long before this
        // tab exists and has no route back to it, which is the same problem, and the same
        // answer, as the find banner's own `active_query`. See `margin_lane::subject`.
        {
            let names = {
                let mut n: Vec<String> = frontend::commands::binder_item_commands::get_binder_item(
                    &self.app_ctx,
                    &self.note_id,
                )
                .ok()
                .flatten()
                .map(|it| {
                    std::iter::once(it.title)
                        .chain(it.aliases)
                        .filter(|s| !s.trim().is_empty())
                        .collect()
                })
                .unwrap_or_default();
                // Longest first, so "Elizabeth Bennet" is marked once rather than twice
                // for the name inside it. `subject::hits` relies on this order.
                n.sort_by_key(|s| std::cmp::Reverse(s.chars().count()));
                n
            };
            crate::margin_lane::set_active_subject(self.work_uid().map(|work_uid| {
                crate::margin_lane::LaneSubject {
                    note_id: self.note_id,
                    names,
                    work_uid,
                }
            }));
        }

        if !self.wired.replace(true) {
            if self.selected_book.get().is_none() {
                let persisted = book_service
                    .as_ref()
                    .and_then(|svc| self.work_uid().map(|uid| svc.book(&uid)))
                    .flatten();
                if let Some(book_id) = crate::models::resolve_book_choice(&books, persisted)
                    && let Some(book) = books.iter().find(|b| b.item_id == book_id)
                {
                    self.selected_book.set(Some(book_segment_id(book)));
                }
            }
            let generation = self.generation.clone();
            crate::models::coalesced_reload::reload_on_events(ctx, reload_origins(), move || {
                generation.set(generation.get().wrapping_add(1));
            });
        }

        // Persist every genuine switch of Book. Registered fresh on every build (the seed
        // above never fires this: `ctx.effect` only wakes on a *change* after this point
        // of registration, not on the value it starts at), exactly as
        // `panes::remember::RememberSegment`'s own persistence effect is.
        {
            let books = books.clone();
            let ids = self.ids.clone();
            let app_ctx = self.app_ctx.clone();
            let service = book_service.clone();
            ctx.effect(&self.selected_book, move |v| {
                let (Some(seg), Some(service)) = (*v, service.as_ref()) else {
                    return;
                };
                let Some(book) = books.iter().find(|b| book_segment_id(b) == seg) else {
                    return;
                };
                let Some(work_id) = ids.work_id.get() else {
                    return;
                };
                let uid = frontend::commands::work_commands::get_work(&app_ctx, &work_id)
                    .ok()
                    .flatten()
                    .map(|w| w.unique_id)
                    .filter(|u| crate::models::uid_is_usable(u));
                let Some(work_uid) = uid else {
                    return;
                };
                let path = crate::current_project_path(&app_ctx, &ids).unwrap_or_default();
                if let Err(e) = service.set_book(&work_uid, &path, book.uid) {
                    eprintln!("note in prose: could not remember the chosen Book: {e}");
                }
            });
        }

        let selected_book_id: Option<u64> = self.selected_book.get().and_then(|seg| {
            books
                .iter()
                .find(|b| book_segment_id(b) == seg)
                .map(|b| b.item_id)
        });

        let rows = selected_book_id
            .map(|book_id| {
                crate::models::declared_rows_in_book(&self.app_ctx, work_id, self.note_id, book_id)
            })
            .unwrap_or_default();

        self.sync_docs(&rows, &self.store);

        let mut col = VStack::new().spacing(10.0);
        if books.is_empty() {
            col = col.child(shared::centered(no_books_text(), &self.header_width));
        } else {
            // **One Book, no bar.** A control offering a single choice answers a question
            // the writer never asked, and the rest of the app already refuses it: the
            // Inspector's Books section and `note_details`'s both render nothing at all
            // below two Books, on `docks::inspector::live_books`'s own stated reasoning.
            // A bar here at one Book put the same tab in two minds about whether "which
            // Book" was a fact worth surfacing, with the Inspector saying no in the
            // trailing rail while this said yes an inch away.
            //
            // The *scope* is unchanged: one Book is still selected and still filters the
            // rows below. Only the chrome that would let a writer change it goes.
            if books.len() >= 2 {
                col = col.child(shared::centered(self.book_bar(&books), &self.header_width));
            }
            if rows.is_empty() {
                col = col.child(shared::centered(empty_book_text(), &self.header_width));
            } else {
                let docs = self.docs.borrow();
                for row in &rows {
                    if let Some(doc) = docs.get(&row.item_id) {
                        col = col.child(crate::margin_lane::RowExtent::new(
                            row.item_id,
                            self.extents.clone(),
                            self.page_scroll.clone(),
                            self.row_widget(row, doc),
                        ));
                    }
                }
            }
        }

        let id = ctx.add(col);
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// The row's declaration, in plain words: whether this note holds the point of view
/// there, is in the cast there, or both. Rendered instead of left implicit because a row
/// can be both at once (see [`Declaration::Both`]'s own doc), and a reader must be able to
/// tell a deep-POV scene that never names this character from one that does.
fn declaration_indicator(declaration: Declaration) -> impl Widget {
    let text = match declaration {
        Declaration::PointOfView => tr!(note_in_prose_pov()),
        Declaration::Cast => tr!(note_in_prose_cast()),
        Declaration::Both => tr!(note_in_prose_pov_and_cast()),
    };
    TextWidget::new(text)
        .style(TextStyleRole::Tiny)
        .color(TextRole::Secondary)
}

fn no_books_text() -> impl Widget {
    TextWidget::new(tr!(note_in_prose_no_books()))
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary)
}

fn empty_book_text() -> impl Widget {
    TextWidget::new(tr!(note_in_prose_empty_book()))
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary)
}

#[cfg(test)]
mod tests {
    use super::*;

    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use frontend::direct_access::{
        BinderItemRelationshipDto, CreateBinderDto, CreateBinderItemDto, CreateWorkDto,
    };

    use crate::settings::{EditorTypography, EditorTypographySet};
    use crate::tabs::tab_for;

    fn test_typography() -> EditorTypographySet {
        let bundle = |family: &str| EditorTypography {
            font_family: Signal::new(family.to_string()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
            size_range: crate::settings::TypographySizeRange::default(),
        };
        EditorTypographySet {
            scene: bundle("Literata"),
            synopsis: bundle("Literata"),
            notes: bundle("Inter"),
            corkboard: bundle("Literata"),
            distraction_free: bundle("Literata"),
        }
    }

    /// A Work with one Book holding one Scene, a Note declared present in that Scene's
    /// cast, and the `Item/Note` tab built against it. Enough for
    /// [`declared_rows_in_book`] to return exactly the one row this module's own
    /// `sync_docs` opens.
    struct Fixture {
        ctx: Rc<frontend::AppContext>,
        tab: ContentTab,
        scene_id: u64,
    }

    fn seed() -> Fixture {
        let ctx = Rc::new(frontend::AppContext::new());
        let work = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default())
            .expect("create work");
        let manuscript = binder_commands::create_binder(
            &ctx,
            None,
            &CreateBinderDto {
                name: "Manuscript".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            0,
        )
        .expect("create manuscript binder")
        .id;
        let notes = binder_commands::create_binder(
            &ctx,
            None,
            &CreateBinderDto {
                name: "Notes".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            1,
        )
        .expect("create notes binder")
        .id;
        let note = binder_item_commands::create_binder_item(
            &ctx,
            None,
            &CreateBinderItemDto {
                title: "A note".into(),
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Note,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            notes,
            0,
        )
        .expect("create note")
        .id;
        binder_item_commands::create_binder_item(
            &ctx,
            None,
            &CreateBinderItemDto {
                title: "Book one".into(),
                role: BinderItemRole::Folder,
                sub_role: BinderItemSubRole::Book,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            manuscript,
            0,
        )
        .expect("create book");
        let scene = binder_item_commands::create_binder_item(
            &ctx,
            None,
            &CreateBinderItemDto {
                title: "A scene".into(),
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Scene,
                activated: true,
                is_exportable: true,
                indent: 1,
                ..Default::default()
            },
            manuscript,
            1,
        )
        .expect("create scene")
        .id;
        binder_item_commands::set_binder_item_relationship(
            &ctx,
            None,
            &BinderItemRelationshipDto {
                id: scene,
                field: BinderItemRelationshipField::References,
                right_ids: vec![note],
            },
        )
        .expect("declare the note in the scene's cast");

        let ids = AppIds::new();
        ids.work_id.set(Some(work.id));
        let tab = tab_for(
            &ctx,
            note,
            &BinderItemRole::Item,
            &BinderItemSubRole::Note,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &ids,
        );
        Fixture {
            ctx,
            tab,
            scene_id: scene,
        }
    }

    /// Before the fix this file threaded [`ContentTab::docs`] for, the store this pane
    /// opened its rows through came from `ctx.app_state::<OpenDocsStore>()`, a slot
    /// that, at first launch with no project open, resolves to `startup.rs`'s throwaway
    /// `WorkSession`'s own, empty `OpenDocsStore`, disjoint from the one the rest of
    /// this tab (and every other open view of the same scene) actually shares. Built
    /// with **no `app_state` registered at all**, the same "nothing to find" state that
    /// lookup was reaching in the wild: the declared row must still open, and it must
    /// open through the tab's own store, not silently do nothing (the old fallback for
    /// a missing store) and not open a second, disjoint one nothing else can see.
    #[test]
    fn a_declared_row_opens_through_the_tabs_own_document_store() {
        let f = seed();

        let mut tree = crate::test_support::tree_with_events(&f.ctx);
        tree.add_boxed(note_in_prose_pane(&f.tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        assert!(
            f.tab.docs().open_item_ids().contains(&f.scene_id),
            "the declared scene must be open through the tab's own OpenDocsStore, the \
             same one `ContentTab::docs` hands out, not a stray `app_state` store (or \
             silently nothing, which is what an absent `app_state` entry used to mean \
             here)"
        );
    }
}
