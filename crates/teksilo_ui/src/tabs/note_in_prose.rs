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

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::text_document::{HighlightMask, TextDocument};
use teksilo::widgets::{
    Divider, Expand, HStack, IconButton, Segment, SegmentId, SegmentedControl, Spacer, TextWidget,
    VStack,
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
/// [`crate::tabs::shared`] trusts the constraint matrix rather than re-checking its own
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
    // Shared with the walk below, which reads the layers this pane builds and never
    // owns any of its own.
    let highlights: Rc<RefCell<HashMap<u64, crate::story_bible::highlight::SubjectHighlight>>> =
        Rc::new(RefCell::new(HashMap::new()));
    let page_scroll = area.scroll_y_signal().clone();
    // Created here, not inside the body: its counter is mounted **outside** the scrolling
    // page so it stays put, and the body needs the same handle to keep the layers in step.
    let walk = Rc::new(crate::story_bible::highlight::SubjectWalk::new(
        highlights.clone(),
    ));
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
        mention_index: tab.mention_index(),
        extents: extents.clone(),
        page_scroll: page_scroll.clone(),
        selected_book: Signal::new(None),
        generation: Signal::new(0),
        docs: docs.clone(),
        highlights: highlights.clone(),
        walk: walk.clone(),
        highlight_format: RefCell::new(None),
        find_docs: tab.in_prose_docs_sink(),
        store: tab.docs(),
        active: None,
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
    // **Pinned above the page, not inside it.** The mentions it counts are spread down a
    // scroll that can be a whole Book long, so a counter that scrolled away with the first
    // row would be gone at exactly the moment a reader wanted the next one. Same place,
    // and the same reason, as the find banner directly above it.
    let page = VStack::new()
        .spacing(0.0)
        .child(shared::centered(
            mention_bar(walk.clone(), tab.format.clone()),
            &tab.column_width,
        ))
        .child(
            Expand::new().child(
                HStack::new()
                    .child(Expand::new().child(area.child(col)))
                    .child(lane),
            ),
        );
    // **Ctrl+F reads this page too.** It is the same shape as a Full Book stream — many
    // manuscript documents on one axis — and a writer checking where a character turns
    // up wants to walk the mentions, not the row boundaries. The banner sits above the
    // page and beside nothing: the lane is inside, for the reason `manuscript_page`
    // records.
    match tab.page_find().cloned() {
        Some(find) => Box::new(crate::tabs::shared::editor::find_banner_over(find, page)),
        None => Box::new(page),
    }
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
    /// **Who is named where, across this Work**, threaded from the same
    /// [`crate::sessions::WorkSession`] [`Self::tags`] is, for the reason
    /// `ContentTab::mention_index`'s own field doc gives, and never read from
    /// `ctx.app_state::<MentionIndex>()`.
    ///
    /// Read for one thing here: its `discoverable_table()`, which is what the scan this
    /// reading marks with resolves overlaps against. Without it a reading about "Grace"
    /// washed every "Grace Kelly" in the book, on the same sentence the roster beside it
    /// credited to Kelly alone.
    mention_index: crate::mentions::MentionIndex,
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
    /// Every declared row's document, opened through the shared [`OpenDocsStore`] so
    /// editing here edits the real document. Reconciled on every build against the
    /// currently declared set (see [`NoteInProseBody::sync_docs`]); released for good when
    /// this widget drops.
    /// The rows' open documents, shared with the lane beside this page so the two
    /// cannot disagree about which document a row is showing.
    docs: Rc<RefCell<HashMap<u64, Rc<OpenDoc>>>>,
    /// One highlight layer per shown row: every name of this entry, marked in the prose
    /// it is read against. Keyed like [`Self::docs`] and pruned with it.
    ///
    /// Held here rather than inside each row's widget because the layer must outlive a
    /// row's *build* — it carries the derived ranges and the document subscription, and a
    /// fresh one per build would re-scan every row's prose on every keystroke elsewhere
    /// on the page.
    highlights: Rc<RefCell<HashMap<u64, crate::story_bible::highlight::SubjectHighlight>>>,
    /// Where the reader is among those marks, and how many there are — what the header's
    /// counter reads and its two chevrons move. Shares the map above rather than owning
    /// layers of its own: the reading builds and prunes those with the rows it shows.
    walk: Rc<crate::story_bible::highlight::SubjectWalk>,
    /// The format those layers were built with, so a theme switch rebuilds them rather
    /// than leaving the previous theme's colour washed over the prose. The same staleness
    /// `FindSession`'s own lazily-created session has, fixed here because this layer is
    /// created without anyone opening anything.
    highlight_format: RefCell<Option<teksilo::text_document::HighlightFormat>>,
    /// **What this reading has put in front of the writer**, in the order it shows it —
    /// republished on every build, and read by the tab's page find banner.
    ///
    /// The banner is built with the tab, long before this page exists and with no route
    /// to it, so the page announces itself instead. Written here rather than derived
    /// there because the order and the membership are this build's own: the selected
    /// Book filters them, and a row whose document has not opened is not shown.
    find_docs: Rc<RefCell<Vec<(u64, TextDocument)>>>,
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
    /// **Whether this page is the one on screen**, from the framework's own answer:
    /// `BuildContext::activation_signal`, `true` while active and `false` while parked
    /// dormant. Installed on the first build; `None` until then, which is a state only a
    /// page being built for the first time is in, and such a page is active.
    ///
    /// This page is what a `Switcher` calls a mounted page: built the first time the writer
    /// picks **In prose**, and then kept for the switcher's lifetime. Leaving the reading
    /// runs no `Drop` and stops no effect, so everything the page had registered against
    /// state outside its own subtree stayed registered.
    ///
    /// Both switches it can disappear behind are `Switcher`s (the segment bar's, and
    /// `TabWidget`'s own), so one signal answers for both. Reading `ContentTab::segment`
    /// instead would answer only the first: a `ContentTab` is not told which pane holds it
    /// or whether it is that pane's front tab, and `receive_tab` moves a handle between
    /// panes.
    ///
    /// `activation_signal`'s own doc says ordinary widgets never need it, since a dormant
    /// subtree is not painted and so vanishes for free. This page is the other case it
    /// names: what it leaves behind does not live in its subtree. A range session lives on
    /// a document a scene tab is still showing, and the lane subject is a thread-local every
    /// stream in the project reads.
    active: Option<Signal<bool>>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for NoteInProseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoteInProseBody").finish_non_exhaustive()
    }
}

/// Release every document this pane still holds open, and take this reading's marks off
/// the margin lane. Mirrors `StreamViewModel`'s own `Inner::drop`, and for the identical
/// reason: without this, a Book with forty declared rows would leak forty `OpenDoc`s every
/// time this segment was abandoned for another one.
///
/// The lane subject is the other half of [`crate::margin_lane::set_active_subject`]'s own
/// contract ("set while a note's **In prose** segment is on screen, cleared when it goes
/// away"), and nothing was honouring it: the story-bible provider is `default_on` for
/// every stream, so one visit to this page left a dot at every occurrence of that entry's
/// names on every Full book / Full part / Full chapter stream in the project, for the rest
/// of the session, with no reading open and nothing on screen explaining them.
/// [`crate::margin_lane::clear_subject_for`] and not `set_active_subject(None)`: a second
/// window may have a reading of another note open, and this one may only take back its own.
impl Drop for NoteInProseBody {
    fn drop(&mut self) {
        let stack = self.ids.stack_id.get();
        for id in self.docs.borrow().keys() {
            self.store.release(*id, stack);
        }
        crate::margin_lane::clear_subject_for(self.scope);
    }
}

/// `Work.unique_id`, or `None` when there is nothing to key anything by. An unsaved project
/// has none, and keying by `""` would make every such project share one.
fn work_uid_of(app_ctx: &Rc<frontend::AppContext>, ids: &AppIds) -> Option<String> {
    let work_id = ids.work_id.get()?;
    let uid = frontend::commands::work_commands::get_work(app_ctx, &work_id)
        .ok()
        .flatten()?
        .unique_id;
    crate::models::uid_is_usable(&uid).then_some(uid)
}

/// Put a reading's entry on the margin lane, or take it back off.
///
/// Published when there is something to publish **and the page is on screen**, and
/// **withdrawn by name** otherwise: an entry with no title and no alias, or a project with no
/// uid yet, has nothing for the lane to mark, and clearing unconditionally would blank the
/// marks a second window's reading of a different note had put there. The same "only if it is
/// mine" rule `NoteInProseBody`'s [`Drop`] takes this off the lane with.
///
/// The on-screen half is what [`crate::margin_lane::set_active_subject`]'s own contract asks
/// for ("set while a note's **In prose** segment is on screen, cleared when it goes away") and
/// what `Drop` alone could not deliver, because the page is never dropped for merely being
/// left; [`NoteInProseBody::active`] is where the answer comes from.
/// The story-bible provider is `default_on` for every stream, so a subject left
/// published puts a dot at every occurrence of that entry's names on every Full book / Full
/// part / Full chapter stream in the project, with nothing on screen explaining them.
///
/// The wash in the prose needs none of this. It is an opt-in layer only this reading's own
/// row editors name (see [`crate::story_bible::highlight`]), so it is bounded by
/// construction. The lane subject has no such scoping, being one thread-local every stream
/// in the project reads, so it is bounded by the page's own visibility instead.
fn publish_subject(
    app_ctx: &Rc<frontend::AppContext>,
    ids: &AppIds,
    scope: crate::margin_lane::LaneScope,
    on_screen: bool,
    entity: Option<&skribisto_model::mentions::DiscoverableEntity>,
    table: &[skribisto_model::mentions::DiscoverableEntity],
) {
    let subject = on_screen.then_some(()).and_then(|()| {
        entity
            .cloned()
            .zip(work_uid_of(app_ctx, ids))
            .map(|(entity, work_uid)| crate::margin_lane::LaneSubject {
                entity,
                table: table.to_vec(),
                work_uid,
                publisher: scope,
            })
    });
    match subject {
        Some(subject) => crate::margin_lane::set_active_subject(Some(subject)),
        None => crate::margin_lane::clear_subject_for(scope),
    }
}

impl NoteInProseBody {
    /// Whether this reading is the page the writer is actually looking at. See
    /// [`Self::active`]. `true` before the first build has asked the framework, which is the
    /// state a page is in while it is being built for the first time.
    fn on_screen(&self) -> bool {
        self.active.as_ref().is_none_or(|a| a.get())
    }

    /// This reading's entry on the margin lane. See [`publish_subject`].
    fn publish_subject(
        &self,
        entity: Option<&skribisto_model::mentions::DiscoverableEntity>,
        table: &[skribisto_model::mentions::DiscoverableEntity],
    ) {
        publish_subject(
            &self.app_ctx,
            &self.ids,
            self.scope,
            self.on_screen(),
            entity,
            table,
        );
    }

    /// `Work.unique_id`, or `None` when there is nothing to key a remembered choice by.
    /// Mirrors `crate::settings::TreeExpansionViewModel::work_uid` exactly, for the same
    /// reason: a brand-new unsaved project has no uid yet, and keying by `""` would make
    /// every such project share one remembered Book.
    fn work_uid(&self) -> Option<String> {
        work_uid_of(&self.app_ctx, &self.ids)
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

    /// **The same names the lane marks, highlighted in the prose beside it.**
    ///
    /// One layer per shown row, kept in step with [`Self::docs`], plus the per-frame
    /// re-derive every range session in this app needs: the prose here is editable, and
    /// an offset carried across an edit would mark the wrong characters.
    ///
    /// The refresh is registered on every build because `ctx.effect` is scoped to one —
    /// and it captures *this build's* names, so a rename re-derives without the layers
    /// having to watch the store themselves.
    fn sync_highlights(
        &self,
        ctx: &mut BuildContext,
        entity: Option<&skribisto_model::mentions::DiscoverableEntity>,
        table: Vec<skribisto_model::mentions::DiscoverableEntity>,
        order: Vec<u64>,
    ) {
        let color = crate::story_bible::highlight::subject_color(&ctx.theme().colors);
        let format = crate::story_bible::highlight::subject_format(color);
        let current_format = crate::story_bible::highlight::subject_current_format(color);
        // The strip's own switch, which governs this too — see the layer's module note.
        // With no settings store at all (a headless build) the marks are on, the same
        // answer `subject_enabled` gives for an unregistered provider.
        let wanted: Option<skribisto_model::mentions::DiscoverableEntity> =
            match ctx.app_state::<teksilo::settings::SettingsStore>() {
                Some(store) if !crate::story_bible::highlight::subject_enabled(store) => None,
                _ => entity.cloned(),
            };
        {
            let mut layers = self.highlights.borrow_mut();
            // A theme switch changes the colour a layer was built with, and a layer
            // carries its formats for life. Rebuilding the set is the cheap correct
            // answer: it happens once per theme change, not once per frame.
            if self.highlight_format.borrow().as_ref() != Some(&format) {
                layers.clear();
                *self.highlight_format.borrow_mut() = Some(format.clone());
            }
            let docs = self.docs.borrow();
            layers.retain(|id, _| docs.contains_key(id));
            for (id, doc) in docs.iter() {
                let Some(field) = doc.main.as_ref() else {
                    continue;
                };
                layers.entry(*id).or_insert_with(|| {
                    crate::story_bible::highlight::SubjectHighlight::new(
                        &field.doc,
                        format.clone(),
                        current_format.clone(),
                    )
                });
            }
        }
        // The order the page mounts them in, so a step crosses rows the way the reading
        // is read rather than the way a `HashMap` happens to hold them.
        self.walk.set_order(order);
        // Once here as well as on the frame tick below, so the first paint of a freshly
        // mounted reading already carries its marks and its count rather than acquiring
        // them a frame later, and only when this page is the one on screen, since a
        // rebuild reaches a parked page too (`reload_origins` fires on any project write).
        self.walk.refresh(
            self.on_screen().then_some(&wanted).and_then(|w| w.as_ref()),
            &table,
        );

        // **Only while the reading is being read.** `ctx.frame_tick` is the tree's own
        // signal, not the visibility-gated `subscribe_frame_tick`, so its observers fire on
        // every frame the window renders, including for a page a `Switcher` has parked.
        //
        // What that costs is not a mention scan per row: `SubjectHighlight::refresh` bails
        // out unless an edit staled it or the names changed. It is the *check*: one
        // element-by-element comparison of the whole discoverable table per row, per frame,
        // plus a full re-derive of every row on the frame after any keystroke anywhere in
        // the project. None of it can reach a pixel while the page is parked.
        //
        // Both closures share one `Rc` of the table and one of the entity: a Book's table is
        // a `String` title and a `Vec<String>` of aliases per entry, and this runs on every
        // build, which `reload_origins` triggers on any project write.
        let names = Rc::new(table);
        let subject_entity = Rc::new(entity.cloned());
        let active = self
            .active
            .clone()
            .unwrap_or_else(|| ctx.activation_signal(ctx.self_id()));
        let refresh = {
            let (walk, wanted, names) = (self.walk.clone(), wanted, names.clone());
            let active = active.clone();
            move || {
                // `None` is not merely "stop refreshing": the ranges pushed while the
                // reading was up are still on the documents, and its own walk still counts
                // them in the header. `None` retires both.
                let subject = active.get().then_some(&wanted).and_then(|w| w.as_ref());
                walk.refresh(subject, &names);
            }
        };
        {
            let refresh = refresh.clone();
            let tick = ctx.frame_tick();
            ctx.effect(&tick, move |_| refresh());
        }
        // And on the switch itself, so leaving the reading clears within the frame rather
        // than whenever the next tick happens to arrive, and coming back re-derives from
        // this build's names.
        //
        // `entity`, not `wanted`: the second carries the *marks* switch
        // (`editor.margin_lane.provider.story_bible`), and the lane provider reads that
        // switch itself. Publishing the gated one here would make the strip's own setting
        // withdraw the subject as well, and would do it only down this path (`build`
        // publishes the ungated entity), so which of the two the lane saw would depend on
        // whether a rebuild or a park happened last.
        let (app_ctx, ids, scope) = (self.app_ctx.clone(), self.ids.clone(), self.scope);
        ctx.effect(&active, move |on_screen| {
            refresh();
            publish_subject(
                &app_ctx,
                &ids,
                scope,
                // The value the signal just took, rather than a re-read of it.
                *on_screen,
                subject_entity.as_ref().as_ref(),
                &names,
            );
        });
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
            .child(
                TextWidget::new(lit!(title_text))
                    .style(style)
                    .color(color)
                    // See `binder::dock`: the indicator after the spacer must stay put.
                    .single_line(),
            )
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
            // **The one view that draws this row's wash.** The layer is an opt-in session
            // (see `crate::story_bible::highlight`), so no editor renders it until its mask
            // names it, and the scene on this row is very often open in a tab of its own,
            // or a row of the Full Chapter in the other half of the split, at the same time.
            //
            // Minted by `sync_highlights` **earlier in this same `build`**, which is what
            // makes reading it here safe: a theme change clears and re-mints every layer, and
            // both halves happen in one build, so the id an editor is given can never name a
            // retired session.
            let mask = self
                .highlights
                .borrow()
                .get(&row.item_id)
                .map(|layer| HighlightMask::all().with(layer.session()));
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
                mask,
            ));
        }
        col
    }
}

impl Widget for NoteInProseBody {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Install-or-reuse, so a rebuild keeps the same signal the parked-page effects of
        // the previous build were watching. See [`Self::active`].
        if self.active.is_none() {
            self.active = Some(ctx.activation_signal(ctx.self_id()));
        }
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
        // The entry as the app's own mention matcher takes it: the title and the aliases
        // kept apart, not flattened. Ordering and overlap are the matcher's business —
        // see `margin_lane::subject::hits` for the bug that rule is written in.
        let entity =
            frontend::commands::binder_item_commands::get_binder_item(&self.app_ctx, &self.note_id)
                .ok()
                .flatten()
                .map(|it| skribisto_model::mentions::DiscoverableEntity {
                    id: self.note_id,
                    title: it.title,
                    aliases: it
                        .aliases
                        .into_iter()
                        .filter(|s| !s.trim().is_empty())
                        .collect(),
                })
                .filter(|e| !e.title.trim().is_empty() || !e.aliases.is_empty());
        // The Work's whole discoverable table, read fresh on every build so an alias
        // added elsewhere in the story bible reaches both surfaces at once. Empty until
        // the index's first scan lands, which `subject::hits` handles: it matches the
        // reading's own entry either way.
        let table = self.mention_index.discoverable_table();
        // A rebuild can reach a page nobody is looking at (`reload_origins` fires on any
        // project write), so this goes through the on-screen gate like every other publish.
        self.publish_subject(entity.as_ref(), &table);

        // **Re-resolved on every build, never seeded once.** A choice that no longer names
        // a live Book is not a choice: trash the Book this reading was on and the previous
        // one-shot seed left `selected_book` pointing at a row `books` no longer holds, so
        // `selected_book_id` answered `None`, `rows` came back empty and the page showed
        // "nothing declared in this Book yet" for good, on a note declared throughout the
        // Book that *is* still there. The same trap caught the first mount: a segment
        // opened before any `Folder/Book` row existed consumed its one seed on an empty
        // list. `resolve_book_choice` is written for exactly this - it refuses to trust a
        // stored uid blindly - and it is only worth anything if it is asked again.
        let resolves = self
            .selected_book
            .get()
            .is_some_and(|seg| books.iter().any(|b| book_segment_id(b) == seg));
        if !resolves {
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
        // Registered on **every** build, not once: `ctx.subscribe_event` and `ctx.effect`
        // are scoped to the build that registered them (`rebuild_single_widget` drains
        // both before calling `build` again), so a one-shot registration on a widget that
        // does rebuild - and this one rebuilds on its own `selected_book`/`generation`
        // bindings - stops hearing the backend after the writer's first switch of Book.
        // Re-registering cannot double-fire for the same reason: the previous build's
        // handles are already gone.
        {
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
        // The rows this build will actually mount, in reading order: a declared row whose
        // document did not open is not on the page, and must not be counted or stepped to.
        let order: Vec<u64> = {
            let docs = self.docs.borrow();
            rows.iter()
                .map(|r| r.item_id)
                .filter(|id| docs.contains_key(id))
                .collect()
        };
        self.sync_highlights(ctx, entity.as_ref(), table, order);

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
        // Tell the find banner what it is over — the same rows, in the same order, that
        // the column above just mounted, so Ctrl+F reads this page as one run of prose.
        // Reset first: a Book with no declared rows, or a note with no Book at all, must
        // leave the previous build's list behind rather than be searched through it.
        *self.find_docs.borrow_mut() = {
            let docs = self.docs.borrow();
            rows.iter()
                .filter_map(|row| {
                    let field = docs.get(&row.item_id)?.main.as_ref()?;
                    Some((row.item_id, field.doc.clone()))
                })
                .collect()
        };

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

/// **How many times this entry is named on this page, and a way to get to each.**
///
/// Marking every mention says *that* they are there; on a Book's reading of forty
/// documents it does not get anyone to them. So the count is a control: two chevrons
/// that step through the marks in reading order, crossing rows, exactly as the find
/// banner steps through what was typed.
///
/// **Always on the page**, not behind a shortcut, because the marks it counts are —
/// and hidden entirely when there is nothing to count, so a reading with no textual
/// mention (an entry declared only as a point of view) shows no counter rather than a
/// zero and two dead buttons.
///
/// The label reads as the plain total until the reader steps into it, and as "3 of
/// 17" after. Reporting an ordinal before anyone has moved would be claiming a
/// position the reader has not taken.
fn mention_bar(
    walk: Rc<crate::story_bible::highlight::SubjectWalk>,
    format: FormatViewModel,
) -> impl Widget + 'static {
    let total = walk.total_signal();
    let ordinal = walk.ordinal_signal();
    let label = total.zip(&ordinal).map(|(total, ordinal)| {
        if *ordinal == 0 {
            tr!(note_in_prose_mentions(n = *total as i64)).resolve_now()
        } else {
            tr!(note_in_prose_mention_at(
                current = *ordinal as i64,
                total = *total as i64
            ))
            .resolve_now()
        }
    });
    let step = |walk: Rc<crate::story_bible::highlight::SubjectWalk>,
                format: FormatViewModel,
                forward: bool| {
        move |c: &mut EventContext| {
            // **Ask for a frame before anything else.** Stepping writes the counter's
            // two signals, and the label binds a *derived* signal over them, which the
            // binding registry picks up by polling generations on the next frame that
            // runs — writing a signal does not pump one. The reveal below usually does,
            // by moving a selection, but not when the step finds no mounted editor and
            // not when the mention was already on screen. The counter changed either
            // way, so the frame is asked for either way.
            //
            // The find banner needs the same thing and gets it the same two ways: its
            // Next selects (which repaints), and `replace_current` / `replace_all`, which
            // change a document without any editor interaction, call this outright.
            c.request_frame();
            let Some((item, start, length)) = walk.step(forward) else {
                return;
            };
            // **Every** editor showing that row, by name. A page builds one per row and
            // none of them is "this tab's", so the registry is the only thing that can
            // answer — the same route the stream find's reveal takes.
            //
            // Every one, not the first: the same scene can be a row here *and* a tab of
            // its own, and a tab that is not on screen is parked dormant with no layout to
            // locate an offset in. Revealing through that one requests nothing at all,
            // which is exactly what "the page does not follow" looked like. `reveal_range`
            // reports whether it could, so the first that can does the scrolling.
            let mut revealed = false;
            for handle in format
                .handles_by_item(crate::format::EditorKind::Prose)
                .into_iter()
                .filter(|(shown, _)| *shown == item)
                .map(|(_, handle)| handle)
            {
                // Selected and scrolled to, exactly as the find banner's own Next does.
                // The wash already says *which* mention, so the selection is not carrying
                // the mark — it is carrying everything else a reader expects of "go to
                // the next one": a caret to type at, Ctrl+C on the name, and the repaint
                // the editor's own state change brings.
                handle.select_range(start, start + length);
                revealed |= !revealed && handle.reveal_range(c, start, start + length);
            }
        }
    };
    let row = HStack::new()
        .spacing(4.0)
        .child(
            TextWidget::new(lit!(""))
                .text(label)
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
        .child(
            IconButton::new(crate::icons::find::nav_prev_icon())
                .embedded()
                .tooltip(tr!(note_in_prose_mention_previous()))
                .on_activate_fn(step(walk.clone(), format.clone(), false)),
        )
        .child(
            IconButton::new(crate::icons::find::nav_next_icon())
                .embedded()
                .tooltip(tr!(note_in_prose_mention_next()))
                .on_activate_fn(step(walk.clone(), format.clone(), true)),
        )
        .child(Expand::horizontal().child(Spacer::new()));
    crate::tabs::shared::editor::VisibleWhen::new(total.map(|n| *n > 0), row)
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
        /// The binder the Books and their scenes live in, so a test can add a second one.
        manuscript: u64,
        /// The note this tab is about, so a test can declare it in another Book's scene.
        note_id: u64,
        /// The one Book `seed` creates, so a test can trash it.
        book_id: u64,
    }

    impl Fixture {
        /// Give the Work a `unique_id`, which is what the lane subject is keyed by:
        /// `CreateWorkDto::default()` leaves it empty, and an empty one reads as "no
        /// project to key by" (`models::uid_is_usable`), so nothing is published.
        fn give_the_work_a_uid(&self) {
            let work_id = self
                .tab
                .ids()
                .work_id
                .get()
                .expect("the fixture opens a Work");
            let w = work_commands::get_work(&self.ctx, &work_id)
                .expect("read the Work")
                .expect("the Work exists");
            work_commands::update_work(
                &self.ctx,
                None,
                &frontend::direct_access::UpdateWorkDto {
                    id: w.id,
                    created_at: w.created_at,
                    updated_at: chrono::Utc::now(),
                    title: w.title,
                    author_name: w.author_name,
                    dict_language: w.dict_language,
                    unique_id: "a-test-project".into(),
                    chapter_mode: w.chapter_mode,
                    custom_replacement_rules_enabled: w.custom_replacement_rules_enabled,
                    goal_unit: w.goal_unit,
                    number_chapters: w.number_chapters,
                    part_resets_chapter: w.part_resets_chapter,
                },
            )
            .expect("give the Work a uid");
        }
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
                status: None,
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
        let book = binder_item_commands::create_binder_item(
            &ctx,
            None,
            &CreateBinderItemDto {
                status: None,
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
        .expect("create book")
        .id;
        let scene = binder_item_commands::create_binder_item(
            &ctx,
            None,
            &CreateBinderItemDto {
                status: None,
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
            manuscript,
            note_id: note,
            book_id: book,
        }
    }

    /// A second (or third) `Folder/Book` at `index` in the manuscript binder.
    fn add_book(f: &Fixture, title: &str, index: i32) -> u64 {
        binder_item_commands::create_binder_item(
            &f.ctx,
            None,
            &CreateBinderItemDto {
                status: None,
                title: title.into(),
                role: BinderItemRole::Folder,
                sub_role: BinderItemSubRole::Book,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            f.manuscript,
            index,
        )
        .expect("create book")
        .id
    }

    /// A Scene at `index` that declares the fixture's note in its own cast, which is
    /// what puts it in this reading.
    fn add_scene_declaring_the_note(f: &Fixture, title: &str, index: i32) -> u64 {
        let scene = binder_item_commands::create_binder_item(
            &f.ctx,
            None,
            &CreateBinderItemDto {
                status: None,
                title: title.into(),
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Scene,
                activated: true,
                is_exportable: true,
                indent: 1,
                ..Default::default()
            },
            f.manuscript,
            index,
        )
        .expect("create scene")
        .id;
        binder_item_commands::set_binder_item_relationship(
            &f.ctx,
            None,
            &BinderItemRelationshipDto {
                id: scene,
                field: BinderItemRelationshipField::References,
                right_ids: vec![f.note_id],
            },
        )
        .expect("declare the note in the scene's cast");
        scene
    }

    /// Trash a row, the way the outline does: `activated` is the trashed flag inverted,
    /// and `books_in_work` reads only activated rows.
    fn trash(f: &Fixture, item_id: u64) {
        let it = binder_item_commands::get_binder_item(&f.ctx, &item_id)
            .expect("read the row")
            .expect("the row exists");
        let mut dto = crate::shared::binder_ops::update_item_dto(&it);
        dto.activated = false;
        binder_item_commands::update_binder_item(&f.ctx, None, &dto).expect("trash the row");
    }

    /// The `NoteInProseBody` node inside the mounted page: the widget whose `build` this
    /// module's own logic lives in, and the one a test drives a rebuild of.
    fn body_id(tree: &teksilo::core::widget_tree::WidgetTree, root: WidgetId) -> WidgetId {
        fn find(tree: &teksilo::core::widget_tree::WidgetTree, id: WidgetId) -> Option<WidgetId> {
            if tree
                .widget_type_name(id)
                .is_some_and(|n| n.ends_with("::NoteInProseBody"))
            {
                return Some(id);
            }
            tree.children(id).into_iter().find_map(|c| find(tree, c))
        }
        find(tree, root).expect("the page mounts a NoteInProseBody")
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
    /// **Ctrl+F reads this page as one run of prose.**
    ///
    /// The banner is built with the tab, before this page exists and with no route to
    /// it, so the page announces the rows it has mounted. That handshake is the whole
    /// wiring, and it is invisible from either side alone: the banner would simply find
    /// nothing, which is also what an empty Book looks like.
    #[test]
    fn the_rows_this_reading_shows_are_what_its_find_banner_searches() {
        let f = seed();
        f.tab.segment.set(Some(shared::segments::segment_id(
            shared::segments::SEG_NOTE_IN_PROSE,
        )));

        let mut tree = crate::test_support::tree_with_events(&f.ctx);
        tree.add_boxed(note_in_prose_pane(&f.tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        let published: Vec<u64> = f
            .tab
            .in_prose_docs_sink()
            .borrow()
            .iter()
            .map(|(id, _)| *id)
            .collect();
        assert_eq!(
            published,
            vec![f.scene_id],
            "the declared row, and the order it is shown in"
        );

        // The scene the note is declared in, with something to find in it.
        let doc = f
            .tab
            .docs()
            .open(f.scene_id)
            .and_then(|d| d.main.as_ref().map(|m| m.doc.clone()))
            .expect("the declared row's prose");
        doc.set_plain_text("the ferry left, and the ferry came back")
            .unwrap();

        let find = f
            .tab
            .active_find()
            .cloned()
            .expect("the In prose reading has a banner");
        find.ensure_session(
            teksilo::text_document::HighlightFormat::default(),
            teksilo::text_document::HighlightFormat::default(),
        );
        find.query_signal().set("ferry".into());
        find.refresh_query();
        assert_eq!(
            find.count_signal().get(),
            2,
            "both hits, found in a document this page mounted rather than in the note"
        );
    }

    /// **The names are marked in the prose, not only beside it.**
    ///
    /// The strip says which rows and roughly where; a reader scanning a scene for the one
    /// place the name is actually written was left to find it by eye. This is the other
    /// half of the same measurement, and the wiring between the two is a handshake the
    /// layer alone cannot prove: the pane has to build one per shown row, keep it in step
    /// with the documents it opened, and re-derive it.
    #[test]
    fn the_names_of_the_entry_are_marked_in_the_prose_it_is_read_against() {
        let f = seed();
        let doc = f
            .tab
            .docs()
            .open(f.scene_id)
            .and_then(|d| d.main.as_ref().map(|m| m.doc.clone()))
            .expect("the declared row's prose");
        // "A note" is the entry's title in this fixture, written once here and once as
        // part of a longer word that must not match.
        doc.set_plain_text("A note was left. Anoteworthy day.")
            .unwrap();

        let mut tree = crate::test_support::tree_with_events(&f.ctx);
        tree.add_boxed(note_in_prose_pane(&f.tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        // The layer this page built over the row's document: the one private session on
        // it, since nothing else in the app registers one.
        let mine = doc.opt_in_session_ids();
        assert_eq!(mine.len(), 1, "the reading builds one layer for this row");

        let marked: Vec<(usize, usize)> = paint_spans(&doc, &HighlightMask::all().with(mine[0]))
            .into_iter()
            .filter(|s| s.background_color.is_some())
            .map(|s| (s.start, s.length))
            .collect();
        assert_eq!(
            marked,
            vec![(0, 6)],
            "the name where it is written, and not inside a longer word"
        );

        // **And nowhere else.** This very document is what a tab on the same scene, a row
        // of the Full Chapter beside it and the search preview band all show; each is on
        // the default mask, and the wash is a fact about this reading, not about the prose.
        assert!(
            paint_spans(&doc, &HighlightMask::all()).is_empty(),
            "an editor that did not ask for the reading's marks must draw none: {:?}",
            paint_spans(&doc, &HighlightMask::all())
        );
    }

    fn paint_spans(
        doc: &TextDocument,
        mask: &HighlightMask,
    ) -> Vec<teksilo::text_document::PaintHighlightSpan> {
        use teksilo::text_document::FlowElementSnapshot;
        match &doc.snapshot_flow_masked(mask).elements[0] {
            FlowElementSnapshot::Block(b) => b.paint_highlights.clone(),
            _ => panic!("block"),
        }
    }

    /// **The marks are reachable, not only visible.**
    ///
    /// Marking every mention says that they are there; on a Book's reading of forty
    /// documents it does not get anyone to them. The counter and its two chevrons are the
    /// other half, and they are on the page rather than behind a shortcut because the
    /// marks they count are.
    ///
    /// Hidden entirely with nothing to count: a reading whose entry is declared as a
    /// point of view but never named would otherwise show a zero and two dead buttons.
    /// Asserted on laid-out height, because the bar is `VisibleWhen`-gated — it is built
    /// either way and takes no space when dormant.
    #[test]
    fn the_mention_counter_is_on_the_page_only_when_there_is_something_to_count() {
        let chevron_height = |prose: &str| {
            let f = seed();
            f.tab
                .docs()
                .open(f.scene_id)
                .and_then(|d| d.main.as_ref().map(|m| m.doc.clone()))
                .expect("the declared row's prose")
                .set_plain_text(prose)
                .unwrap();

            let mut tree = crate::test_support::tree_with_events(&f.ctx);
            let root = tree.add_boxed(note_in_prose_pane(&f.tab));
            tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));
            fn tallest(tree: &teksilo::core::widget_tree::WidgetTree, id: WidgetId) -> f32 {
                let mine = if tree
                    .widget_type_name(id)
                    .is_some_and(|n| n.ends_with("::IconButton"))
                {
                    tree.bounds(id).height
                } else {
                    0.0
                };
                tree.children(id)
                    .into_iter()
                    .map(|c| tallest(tree, c))
                    .fold(mine, f32::max)
            }
            tallest(&tree, root)
        };

        // "A note" is the entry's title in this fixture.
        assert!(
            chevron_height("A note was left, and A note again.") > 0.0,
            "two mentions, so a counter and a way to walk them"
        );
        assert_eq!(
            chevron_height("Nobody came."),
            0.0,
            "nothing named here, so no counter and no dead chevrons"
        );
    }

    /// **The chevrons have somewhere to send the reader.**
    ///
    /// Stepping is two halves: the walk moves its cursor, and the row's *editor* is asked
    /// to select and scroll to the span. The second half goes through the editor registry
    /// by item id — a page builds one editor per row and none of them is "this tab's" —
    /// and if a row's editor never registered under its own id, the walk would move
    /// silently and the page would not budge. That is exactly what "the buttons do
    /// nothing" looks like, and nothing else in the suite would notice.
    #[test]
    fn a_mounted_row_is_reachable_through_the_editor_registry() {
        let f = seed();
        f.tab
            .docs()
            .open(f.scene_id)
            .and_then(|d| d.main.as_ref().map(|m| m.doc.clone()))
            .expect("the declared row's prose")
            .set_plain_text("A note was left, and A note again.")
            .unwrap();

        let mut tree = crate::test_support::tree_with_events(&f.ctx);
        tree.add_boxed(note_in_prose_pane(&f.tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        let reachable: Vec<u64> = f
            .tab
            .format
            .handles_by_item(crate::format::EditorKind::Prose)
            .into_iter()
            .map(|(item, _)| item)
            .collect();
        assert!(
            reachable.contains(&f.scene_id),
            "the declared row's editor must be reachable by its own id, or a step has \
             nowhere to reveal; registry holds {reachable:?}"
        );
    }

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

    /// **A Book that stops being a Book must not strand the reading.**
    ///
    /// The choice used to be seeded exactly once, behind the same one-shot guard the
    /// reload subscription sat behind, so a Book that was trashed after the page had
    /// opened left `selected_book` naming a row `books_in_work` no longer answers with:
    /// no Book resolved, no rows were read, and the page said "nothing declared in this
    /// Book yet" for the rest of the tab's life, on a note declared in the Book that is
    /// still there, with no control on screen to recover. `resolve_book_choice` exists
    /// to refuse a stale choice; it is worth nothing if it is only ever asked once.
    ///
    /// The rebuild is driven directly rather than by trashing and waiting for the event:
    /// a headless tree drops backend events (`test_support`'s `NullPoster`), so what is
    /// under test here is `build`'s own re-resolution, which is where the defect was.
    #[test]
    fn the_chosen_book_is_re_resolved_when_it_is_no_longer_a_live_book() {
        let f = seed();
        let _book_two = add_book(&f, "Book two", 2);
        let scene_two = add_scene_declaring_the_note(&f, "A later scene", 3);

        let mut tree = crate::test_support::tree_with_events(&f.ctx);
        let root = tree.add_boxed(note_in_prose_pane(&f.tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));
        let shown = || -> Vec<u64> {
            f.tab
                .in_prose_docs_sink()
                .borrow()
                .iter()
                .map(|(id, _)| *id)
                .collect()
        };
        assert_eq!(
            shown(),
            vec![f.scene_id],
            "the reading opens on the first Book"
        );

        // The writer trashes the Book this reading is on.
        trash(&f, f.book_id);
        let body = body_id(&tree, root);
        tree.arena_mark_needs_rebuild_for_testing(body);
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        assert_eq!(
            shown(),
            vec![scene_two],
            "the reading must fall back to a Book that still exists, not to an empty page"
        );
    }

    /// **The lane's marks go away with the reading that asked for them.**
    ///
    /// `margin_lane::subject`'s own contract is "set while a note's In prose segment is
    /// on screen, cleared when it goes away", and nothing was honouring the second half:
    /// `clear_subject_for` had no caller at all. The story-bible provider is `default_on`
    /// for every stream, so one visit to this page left a dot at every occurrence of this
    /// entry's names on every Full book stream in the project for the rest of the
    /// session, with no reading open and nothing on screen to explain or remove them.
    #[test]
    fn the_lane_subject_is_withdrawn_when_the_reading_goes_away() {
        let f = seed();
        f.give_the_work_a_uid();

        let mut tree = crate::test_support::tree_with_events(&f.ctx);
        tree.add_boxed(note_in_prose_pane(&f.tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));
        assert_eq!(
            crate::margin_lane::active_subject()
                .get()
                .map(|s| s.note_id()),
            Some(f.note_id),
            "the reading publishes the entry it is about"
        );

        drop(tree);
        assert!(
            crate::margin_lane::active_subject().get().is_none(),
            "and takes it back when the page is gone"
        );
    }

    /// **Leaving the reading is not the same as closing it, and both must clear.**
    ///
    /// `Switcher` mounts a page lazily and then keeps it for its own lifetime, so picking
    /// another segment, or another tab, drops nothing, runs no `Drop` and cancels no effect.
    /// The page went on publishing its subject and re-deriving its marks every frame, for a
    /// reading nobody was looking at.
    ///
    /// Parked through a real `Switcher` rather than by writing `ContentTab::segment`,
    /// because the gate is the framework's own activation and the point is that it answers
    /// for a tab switch as well as a segment one. Both are `Switcher`s; this is one of them.
    ///
    /// Two things are checked, because the two halves fail differently: the lane subject is
    /// withdrawn by name, and the walk retires the ranges it had pushed onto documents that
    /// other views are still showing.
    #[test]
    fn parking_the_page_clears_what_the_reading_had_published() {
        let f = seed();
        f.give_the_work_a_uid();
        let doc = f
            .tab
            .docs()
            .open(f.scene_id)
            .and_then(|d| d.main.as_ref().map(|m| m.doc.clone()))
            .expect("the declared row's prose");
        doc.set_plain_text("A note was left.").unwrap();

        // Page 0 is the reading, page 1 stands in for whatever the writer switches to.
        let page = Signal::new(0usize);
        let mut tree = crate::test_support::tree_with_events(&f.ctx);
        tree.add(
            teksilo::widgets::Switcher::new(page.clone())
                .child_boxed(note_in_prose_pane(&f.tab))
                .child(TextWidget::new(lit!("elsewhere"))),
        );
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        let mine = doc.opt_in_session_ids();
        assert_eq!(mine.len(), 1, "the reading builds one layer for this row");
        let marks = || paint_spans(&doc, &HighlightMask::all().with(mine[0])).len();
        assert!(marks() > 0, "the reading marks its entry while it is up");
        assert!(crate::margin_lane::active_subject().get().is_some());

        // The writer switches away. The page is not destroyed: it stays mounted, still
        // holding its documents, still observing.
        page.set(1);
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        assert!(
            crate::margin_lane::active_subject().get().is_none(),
            "the strip must not go on marking this entry on every stream in the project"
        );
        assert_eq!(
            marks(),
            0,
            "and the ranges must come off the documents the rest of the app is showing"
        );

        // Coming back re-derives rather than leaving a blank reading behind.
        page.set(0);
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));
        assert!(
            marks() > 0,
            "returning to the reading brings its marks back"
        );
        assert_eq!(
            crate::margin_lane::active_subject()
                .get()
                .map(|s| s.note_id()),
            Some(f.note_id),
            "and its subject with them"
        );
    }

    /// A reading with nothing to publish (an entry with no title and no alias, or a
    /// project with no uid yet) must **withdraw its own** subject and not blank the
    /// marks a second window's reading of a different note put on the lane.
    #[test]
    fn a_reading_with_nothing_to_publish_leaves_another_readings_marks_alone() {
        let f = seed();
        // No uid on this Work, so this page has nothing to publish.
        let elsewhere = crate::margin_lane::LaneSubject {
            entity: skribisto_model::mentions::DiscoverableEntity {
                id: f.note_id + 1_000,
                title: "Elizabeth".into(),
                aliases: vec!["Lizzy".into()],
            },
            table: Vec::new(),
            work_uid: "another-project".into(),
            publisher: crate::margin_lane::LaneScope::fresh(),
        };
        crate::margin_lane::set_active_subject(Some(elsewhere.clone()));

        let mut tree = crate::test_support::tree_with_events(&f.ctx);
        tree.add_boxed(note_in_prose_pane(&f.tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));

        assert_eq!(
            crate::margin_lane::active_subject().get(),
            Some(elsewhere),
            "another window's reading is not this page's to clear"
        );
    }
}
