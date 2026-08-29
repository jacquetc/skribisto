// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Per-`(role, sub_role)` editor tabs.
//!
//! Every valid `(role, sub_role)` combination has its **own module** — a single
//! visual tab (`item_scene`, `item_chapter_scene`, `folder_book`, …); [`tab_pane`]
//! dispatches each combination to its module. Not every outline row is a prose
//! editor: a writing item (Scene / ChapterScene / Note) opens the dual-pane
//! editor; a title-bearing item (Chapter / Part / BookBegin) opens a heading form;
//! a structural folder (Book / Part / Chapter) opens a **container** tab with a
//! `SegmentedControl` (its own page, the manuscript streams, Corkboard and Overview);
//! a contentless row
//! (BookEnd / Text) opens a placeholder. What several combinations share — the
//! composite pane bodies and the low-level editor primitives — lives in
//! [`shared`]. One [`ContentTab`] payload type carries them all.
//!
//! A tab owns **no documents of its own**: its live editing state (main text +
//! synopsis + titles, the dirty flag) lives in a
//! shared [`OpenDoc`] held by the [`OpenDocsStore`],
//! keyed by item id. A `ContentTab` is a thin **view** that references that
//! `Rc<OpenDoc>` plus its own per-tab presentation state (segment, column width,
//! typography). Opening the same item in two panes yields two `ContentTab`s over
//! one `OpenDoc`, so the two editors share one live `TextDocument`.
//!
//! Prose is Djot end-to-end: documents load via `set_djot` and write back via
//! `to_djot` into `Content` rows. Write-back is **role-aware** — an `OpenDoc`
//! only ever owns the content roles `skribisto_model` allows for its
//! `(role, sub_role)`, so non-prose rows can never be corrupted.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole, GoalUnit};
use frontend::direct_access::ContentDto;
use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::text_document::TextDocument;
use teksilo::widgets::{
    Banner, Button, ButtonVariant, Expand, Orientation, PaneDescriptor, SplitterModel, VStack,
};

use crate::app_ids::AppIds;
use crate::models::{OpenDoc, OpenDocsStore};
use crate::pace::PaceViewModel;
use crate::settings::{EditorTypography, EditorTypographySet};
use crate::shared::SynopsisPlacement;
use crate::singles::{SingleBinderItem, SingleContent};
use crate::stream::{SplitFlavour, StreamViewModel};

// One module per valid `(role, sub_role)` combination — each a single visual tab
// (see `skribisto_model::COMBINATIONS`). `tab_pane` dispatches to them.
/// `pub`, not `pub(crate)`: an extension registers its own Analysis category through
/// [`analysis::register_category`], which it could not name from outside the crate.
pub mod analysis;
pub(crate) mod corkboard;
mod folder_book;
mod folder_chapter_scene;
mod folder_none;
mod folder_note;
mod folder_paratext;
mod folder_part;
mod item_book_begin;
mod item_book_end;
mod item_chapter_scene;
mod item_note;
mod item_paratext;
mod item_part;
mod item_scene;
mod item_text;
/// The `Item/Note` tab's "Details" segment body, a sibling of `item_note`, not a
/// dispatch target of its own: `tab_pane` never routes here directly, `item_note`'s
/// own `render` calls into it (alongside `SEG_NOTE_OWN` and `SEG_NOTE_IN_PROSE`) the
/// same way `folder_synopsis_with_overview` calls into `story_bible_place`.
pub(crate) mod note_details;
/// The `Item/Note` tab's third segment: a writable stream of the manuscript prose a
/// story-bible entry has been declared present in. `pub(crate)`, not private, for the
/// same reason `story_bible_place` is: `item_note` (a sibling module, not a descendant of
/// this one) has to reach [`note_in_prose::note_in_prose_pane`] to put it on the segment
/// bar.
pub(crate) mod note_in_prose;
pub(crate) mod overview;
pub(crate) mod pace;
/// `pub`, not `pub(crate)`, because [`shared::segments`] is an extension slot: a
/// `pub` item inside a `pub(crate)` module is unreachable from outside the crate
/// however public it looks, which is exactly what `container.segments` was until
/// something outside the crate first tried to use it.
pub mod shared;
pub(crate) mod story_bible_place;

/// Which of the item's two names this field edits.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TitlePart {
    Title,
    SubTitle,
}

/// A short, single-line name (the item's title, or a Book's subtitle), edited via a
/// `TextInput` bound to `value`.
///
/// It persists through [`SingleBinderItem`], **not** through `SingleContent` — because
/// the name has two homes that must never drift: `BinderItem.title`, which the outline
/// tree and the tab show, and the title `Content` row, which is what gets compiled into
/// the manuscript. The single writes both. Editing a chapter's title in its editor used
/// to write only the content row, leaving the tree and the tab showing the old name.
pub struct TitleField {
    pub value: Signal<String>,
    original: Rc<RefCell<String>>,
    item: SingleBinderItem,
    part: TitlePart,
}

/// A rich prose content (SceneText/NoteText/SynopsisText), edited in a
/// `RichTextEditor` over `doc` and persisted as a `.djot` blob via its
/// [`SingleContent`].
pub struct ProseField {
    pub doc: TextDocument,
    content: SingleContent,
    /// `doc.content_revision()` as of the last successful [`flush`](Self::flush)
    /// (or the load that built this field) — see [`Self::is_stale`].
    flushed_revision: Cell<u64>,
}

/// The dynamic-tab payload: a thin per-tab **view** over a shared [`OpenDoc`]
/// (the item's live documents, held by the store) plus this tab's own
/// presentation state. Field access to the shared documents goes through the
/// accessor methods, which forward to `open_doc`.
pub struct ContentTab {
    /// The shared, refcounted open-document state for this item — the live
    /// `TextDocument`s + write-back. Two tabs (e.g. one per split pane) showing
    /// the same item hold the **same** `Rc<OpenDoc>`.
    pub open_doc: Rc<OpenDoc>,
    /// The manuscript-stream view-model — `Some` only for a folder container (Chapter /
    /// Part / Book). It lives here, not on the shared `OpenDoc`, because it holds the
    /// `OpenDocsStore` that owns that `OpenDoc`: hanging it there would close an `Rc`
    /// cycle and make `OpenDocsStore::clear()` re-enter its own `RefCell`. See
    /// [`StreamViewModel`].
    stream: Option<StreamViewModel>,
    /// The Book's writing-plan view-model — `Some` only for a `Folder/Book`
    /// container, the one combination with a "Pace" segment. Like `stream`, it
    /// lives on the tab (not the shared `OpenDoc`) and reads through the backend,
    /// not through documents. Consumed by the Pace pane.
    #[allow(dead_code)]
    pace: Option<PaceViewModel>,
    /// The Analysis view-model — `Some` only for a `Folder/Book` container, gated exactly
    /// as `pace` is. Per-tab rather than shared: two containers open side by side are two
    /// analyses of two different scopes, and one shared instance would have the second
    /// overwrite the first.
    analysis: Option<crate::analysis::AnalysisViewModel>,
    /// The Corkboard view-model — `Some` only for a folder container (Chapter /
    /// Part / Book), gated on the same
    /// [`StreamLevel::for_container`](crate::models::StreamLevel::for_container) as `stream`.
    corkboard: Option<crate::corkboard::CorkboardViewModel>,
    /// The Story bible grid's own Books filter chip, per-tab like [`Self::segment`]:
    /// each open notes-folder tab keeps its own choice. `None` is the unfiltered
    /// default (every entry, filed or not), matching every other Books surface in
    /// this edition. See [`crate::tabs::story_bible_place`].
    pub story_bible_book_filter: Signal<Option<u64>>,
    /// The Overview view-model — `Some` for every container that offers the segment,
    /// gated on [`skribisto_model::overview_capable`]. That is a **wider** gate than the
    /// stream's and the corkboard's: a `Folder/Note` has no manuscript extent, so it has
    /// no stream, but it does have a subtree worth tabulating.
    overview: Option<crate::overview::OverviewViewModel>,
    /// The project's target unit and the window's counting method — what the container
    /// pages need to draw a target readout that agrees with the Overview beside it.
    goal_unit: Signal<GoalUnit>,
    counting_method: Signal<skribisto_model::counting::CountingMethodSetting>,
    /// The app's entity ids — needed for the undo stack when a name field commits,
    /// and published by [`Self::ids`] for the `container.segments` slot.
    ids: AppIds,
    /// This Work's save state, narrowed to what a registered segment may touch —
    /// published by [`Self::work`] for the same slot, and for the same reason
    /// `ids`/`app_ctx` are: a segment that edited its own state without it left
    /// the project reading clean, and Close/Quit discarded the edit in silence.
    work: crate::save::WorkHandle,
    /// The backend handle this tab was built against.
    ///
    /// Kept even though every sub-view-model was already handed its own clone at
    /// construction: it is what [`Self::app_ctx`] publishes, and a registered
    /// segment has no other route to the store — see that accessor's docs.
    app_ctx: Rc<AppContext>,
    /// The per-editor find banner (Ctrl+F) — `Some` only when this tab has a main
    /// prose field to search. Persisted on the tab so it survives tab rebuilds
    /// (its `FindSession` + query outlive the widget tree it draws into).
    find: Option<crate::search::FindViewModel>,
    /// The find banner of this tab's **multi-document** pages: a manuscript stream, a
    /// full-synopsis stream, a note's In-prose reading. `Some` for every tab that has
    /// one of those pages at all.
    ///
    /// A second view-model rather than a mode on `find`, because the two are searching
    /// different things and the writer moves between them by switching segment: keeping
    /// a query and a match cursor per page is what makes going back to a stream land
    /// where it was left. Only one of the two is ever mounted, because only one segment
    /// is — see [`Self::active_find`], which is what Ctrl+F resolves through.
    page_find: Option<crate::search::FindViewModel>,
    /// **What the In-prose reading is showing**, in the order it shows it.
    ///
    /// Published by that page on every build and read by `page_find`'s document source,
    /// which is built here — long before the page exists and with no route to it. Only
    /// ever consulted while that segment is the one on screen, so the list a segment
    /// switch leaves behind is never searched.
    in_prose_docs: Rc<RefCell<Vec<(u64, teksilo::text_document::TextDocument)>>>,
    /// This tab's **synopsis** editor handle, re-attached on every build the way
    /// the prose one is (a tab rebuild mints a fresh editor and a fresh handle).
    ///
    /// It does not live on `find` beside the prose handle, even though that type
    /// admits owning "the prose editor of this tab": a synopsis has no find
    /// banner, so a tab with only a synopsis would have no `FindViewModel` to
    /// hang it on. Unifying the two under one owner is worth doing, but not by
    /// giving `find` a back-reference to this tab — the `stream` field above
    /// records what closing that particular `Rc` cycle costs.
    synopsis_handle: Rc<RefCell<Option<teksilo::widgets::rich_text::EditorHandle>>>,
    /// This tab's remembered caret + page scroll, and the live ports the mounted
    /// pane publishes so both can be read back.
    ///
    /// Per-*pane*, not per-document, which is why it is here and not on the
    /// shared `OpenDoc` beside the spell and replacement sessions: two split
    /// panes on one item have one `TextDocument` but two carets. The signal is
    /// the seed a freshly-built pane starts from; the ports are the live wiring.
    /// See [`crate::shared::ViewState`].
    view_state: Signal<crate::shared::ViewState>,
    view_state_ports: Rc<crate::shared::ViewStatePorts>,
    /// The segment this tab should open on, remembered per **tab** rather than per
    /// item type, and consumed by `RememberSegment::wrap` the once.
    ///
    /// Held as the segment's stable **string** id, because `segments::segment_id`
    /// derives the numeric one from it one way only. `wrap` is also the only place
    /// that can tell whether the string still names a segment this container has,
    /// which is why the seed is validated there rather than applied here: an
    /// extension unregistered since the position was written down must fall back to
    /// the app-global remembered view, not select a page at random.
    segment_seed: RefCell<Option<String>>,
    /// The segment currently on screen, as its string id, mirrored out of
    /// `RememberSegment` so a capture can write it down. Empty until this tab has
    /// built a segmented body at all, which is every non-container combination.
    segment_shown: Rc<RefCell<String>>,
    /// Selected segment for the folder container's `SegmentedControl` — per-tab
    /// (each pane keeps its own segment).
    /// Which segment the container's bar has selected, **keyed** rather than positional.
    ///
    /// A segment can be contributed through `shared::segments`, so the list is no longer
    /// closed: an index would silently re-point at a neighbour the moment one registered
    /// ahead of the selected one. `None` is "nothing chosen", which the bar resolves to
    /// its first segment. Ids come from `shared::segments::segment_id`.
    pub segment: Signal<Option<teksilo::widgets::SegmentId>>,
    pub column_width: Signal<f32>,
    /// Persisted "show synopsis pane" setting (Settings ▸ Manuscript & Fonts),
    /// consumed live by the dual-pane writing editor.
    ///
    /// **Per-caller, not simply the global signal.** A pane tab is handed the
    /// setting itself; the distraction-free surface's tab is handed that mode's own
    /// local flag instead, so showing the synopsis while writing full-screen does
    /// not rewrite a preference that governs every other window.
    pub show_synopsis: Signal<bool>,
    /// Whether the epigraph disclosure is open. Per-tab view state, seeded on build from
    /// whether there is an epigraph to show: an authored one is open so it is not hidden
    /// from its own author, an empty one is folded away so a book that has no epigraph
    /// does not carry an empty box on every chapter page.
    ///
    /// Lives here rather than on `ProseField` for the same reason `show_synopsis` does —
    /// a `ProseField` owns a document and its write-back, nothing about presentation, and
    /// one document is shared by every simultaneous view of the item.
    pub epigraph_expanded: Signal<bool>,
    /// Persisted synopsis placement (above vs beside the manuscript), shared live
    /// from Settings like [`Self::column_width`].
    pub synopsis_placement: Signal<SynopsisPlacement>,
    /// Persisted width of the Side synopsis column, shared live from Settings. Seeds
    /// [`Self::side_splitter`] and sets the width below which Side is not attempted.
    pub synopsis_side_width: Signal<f32>,
    /// The divider between the Side synopsis and the manuscript. One model per tab:
    /// every open tab can be showing Side at once, in either pane of the split.
    ///
    /// Pane 0 is the synopsis, pane 1 the manuscript. The synopsis pane starts
    /// hidden with `min_size` 0 for the same reason the editor's own side pane does
    /// — a `Splitter` sums *every* pane's minimum into its own, visible or not, so a
    /// hidden pane holding a real minimum would inflate the tab's minimum width.
    pub side_splitter: SplitterModel,
    /// The three per-editor-type typography bundles (Scene / Synopsis / Notes),
    /// shared live from Settings. Every editor this tab builds reads its bundle
    /// from here, so a preference change fans out to all open tabs at once.
    pub typography: EditorTypographySet,
    /// Typewriter scrolling, shared live from Settings — the enabled flag plus
    /// the pinned-line preset. Every full-page writing surface this tab builds
    /// reads it, and the tab's `ScrollArea` buys its scroll-past-end range from
    /// it, so the two can never disagree about whether pinning is on.
    pub typewriter: crate::shared::TypewriterSettings,
    /// The ambient caret band, shared live from Settings — how much text around the caret
    /// is shaded, and in what colour. Every writing surface this tab builds reads it.
    pub caret_highlight: crate::shared::CaretHighlightSettings,
    /// The language of this tab's document, resolved once at build from the same
    /// `effective_language` the spell-checker reads. Only the band's sentence scope needs it.
    caret_locale: Option<String>,
    /// Whether *this tab's window* is currently in distraction-free mode — the
    /// same `Signal` `FocusViewModel::active_signal()` exposes, threaded down
    /// through `EditorsViewModel` (never a private copy: a copy would go stale
    /// the instant the mode toggled). Read by [`Self::main_typography`] to pick
    /// the distraction-free typography bundle and by [`Self::main_column_width`]
    /// to pick the distraction-free column width, both instead of the normal
    /// Scene/Notes split — nothing else on the tab branches on it.
    pub distraction_free: Signal<bool>,
    /// Max width (px) of the writing column while [`Self::distraction_free`] is
    /// active (Settings ▸ Editor ▸ Editor Behavior's own "Column width" slider),
    /// shared live from Settings — same shape as [`Self::column_width`], kept as
    /// its own field so widening the normal column can never silently widen (or
    /// narrow) the distraction-free one. Read by [`Self::main_column_width`].
    pub distraction_free_width: Signal<f32>,
    /// Per-container-type "last view" memory: seeds this tab's initial [`Self::segment`]
    /// and (for a folder container) is written back when the user switches view, so a
    /// new tab of the same type inherits it. Shared live from Settings.
    pub view_memory: crate::settings::EditorViewMemory,
    /// This window's Format surfaces — every writing editor this tab builds
    /// registers with it (never process-wide `app_state`).
    pub format: crate::format::FormatViewModel,
    /// The writing games this project is playing, shared live: the per-`Work`
    /// activation paired with the app-global "which surfaces" options. Every
    /// writing editor this tab builds reads it, so switching a game on reaches
    /// every open surface of this project at once — including the ones in a
    /// second window, since the activation half is Tier 2.
    pub writing_games: crate::writing_session::WritingGamesViewModel,
    /// This tab's tag palette handle, threaded from the same [`crate::sessions::WorkSession`]
    /// real windows build [`crate::editors::EditorsViewModel`] from. **Not** read from
    /// `ctx.app_state::<TagsViewModel>()`: `note_details`'s Details segment builds its "+"
    /// popover from this field. `app_state` resolves to whatever window's session
    /// registered last, which at first launch (no project open yet) is `startup.rs`'s
    /// throwaway `WorkSession` on a fresh, never-seeded `AppIds`, so `TagsViewModel::create`,
    /// which needs a real `work_id`, silently created nothing. See [`Self::tags`].
    tags: crate::tags::TagsViewModel,
    statuses: crate::statuses::StatusesViewModel,
    /// Who is named where, across this Work, threaded from the same
    /// [`crate::sessions::WorkSession`] [`Self::tags`] is, and **not** read from
    /// `ctx.app_state::<MentionIndex>()`. That resolves to whichever session was
    /// registered in `lib.rs`, which on the ordinary Launcher path is a throwaway
    /// `WorkSession` built on a fresh `AppIds` that is never seeded: its index's own
    /// `fire` returns early for want of a `work_id`, so it never scans, and
    /// `backlinks_for` answers empty for every note, forever. What that looks like from
    /// the writer's chair is "Appears in the manuscript" simply never working.
    mention_index: crate::mentions::MentionIndex,
    /// This tab's shared document store, threaded from the same
    /// [`crate::sessions::WorkSession`] [`Self::tags`] is. **Not** read from
    /// `ctx.app_state::<OpenDocsStore>()`: [`crate::tabs::note_in_prose::note_in_prose_pane`]
    /// opens every declared row's document through this handle. `app_state`
    /// resolves to whatever window's session registered last, which at first
    /// launch (no project open yet) is `startup.rs`'s throwaway `WorkSession`,
    /// with its own, empty `OpenDocsStore`, disjoint from the one this tab's own
    /// editor actually shares. A row opened through that stale store would be a
    /// second, independent `OpenDoc` for the same item: the same silent
    /// split-document risk [`Self::tags`]'s doc records, one document instead of
    /// one tag. See [`Self::docs`].
    docs: OpenDocsStore,
}

/// Which prose kind a dual-pane main-text editor is, so it can pick the Scene vs
/// Note typography bundle. `None` for every non-prose combination (folder tabs,
/// headings) — they never populate `OpenDoc::main`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProseKind {
    Scene,
    Note,
}

pub(crate) fn prose_kind_for(
    role: &BinderItemRole,
    sub_role: &BinderItemSubRole,
) -> Option<ProseKind> {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    match (role, sub_role) {
        // Both encodings of a chapter carry scene prose, as does a plain Scene.
        (Item, Scene) | (Item, ChapterScene) | (Folder, ChapterScene) => Some(ProseKind::Scene),
        (Item, Note) => Some(ProseKind::Note),
        // A paratext is set like the body it sits beside — a preface is typeset as
        // manuscript prose, not as a note — so it takes the Scene bundle rather than
        // earning a fourth one nobody asked for.
        (Item, Paratext) => Some(ProseKind::Scene),
        // `None` is shadowed by `BinderItemSubRole::None` under the glob import.
        _ => Option::None,
    }
}

/// Whether `(role, sub_role)` renders through [`shared::prose`] — the dual-pane
/// writing editor, and the only body with a synopsis the writer can show, hide or
/// place beside the manuscript.
///
/// Exactly `{Item/Scene, Item/ChapterScene, Item/Note}`. Two nearby predicates
/// look like they answer this and do not: `is_synopsis_bearing` is true for ten of
/// the twelve combinations (a Book folder has a synopsis, but an unconditional one
/// on its own page), and [`prose_kind_for`] alone includes `Folder/ChapterScene`,
/// which carries scene prose for typography's sake but renders through
/// `folder_segmented`. Gating a synopsis control on either lights it up on tabs
/// that have nothing for it to act on.
pub(crate) fn renders_prose(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> bool {
    matches!(role, BinderItemRole::Item) && prose_kind_for(role, sub_role).is_some()
}

/// A prose field seeded from its [`SingleContent`], **not** from `existing` directly —
/// the single *is* the real/mock seam (its mock variant fabricates content for an item
/// the empty mock backend has no rows for), so reading around it would leave every
/// editor blank under `--features mocks`.
pub(crate) fn prose_field(
    ctx: &Rc<AppContext>,
    item_id: u64,
    role: ContentRole,
    existing: Option<&ContentDto>,
) -> ProseField {
    let content = SingleContent::for_field(ctx.clone(), item_id, role, existing);
    let doc = TextDocument::new();
    // `set_djot_sync`, not `set_djot(..).wait()`: this is a *load*, and the async
    // form spawns a worker thread only for us to block on it — overhead that does
    // not shrink with the text, so an empty scene paid it in full. A container
    // stream opens one document per row up front (see `tabs::shared::stream`), so
    // that per-load cost is multiplied by the whole book: it is what made
    // switching a Book to Full Book / Full Synopsis freeze for seconds.
    let _ = doc.set_djot_sync(&content.data().get());
    doc.set_modified(false);
    let flushed_revision = Cell::new(doc.content_revision());
    ProseField {
        doc,
        content,
        flushed_revision,
    }
}

/// A name field over `item_id`, seeded from the **entity** (`BinderItem.title` /
/// `.sub_title`) — the value the outline tree and the tab show, and therefore the one
/// the writer means by "the title". The matching `Content` row is kept in step by
/// [`SingleBinderItem::set_title`] on save.
pub(crate) fn title_field(ctx: &Rc<AppContext>, item_id: u64, part: TitlePart) -> TitleField {
    let item = SingleBinderItem::new(ctx.clone());
    item.set_id(Some(item_id));
    let data = match part {
        TitlePart::Title => item.title().get(),
        TitlePart::SubTitle => item.sub_title().get(),
    };
    TitleField {
        value: Signal::new(data.clone()),
        original: Rc::new(RefCell::new(data)),
        item,
        part,
    }
}

/// Build a standalone tab for `item_id` (its own fresh, unshared [`OpenDoc`] over a
/// private [`OpenDocsStore`]).
///
/// The real app opens tabs through `EditorsViewModel` / the app-wide [`OpenDocsStore`],
/// which shares one `OpenDoc` across panes **and** across a stream's rows; this
/// convenience is for tests and any call site that wants a self-contained tab.
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)] // standalone-tab convenience; exercised by the tab tests
pub fn tab_for(
    ctx: &Rc<AppContext>,
    item_id: u64,
    role: &BinderItemRole,
    sub_role: &BinderItemSubRole,
    contents: &[ContentDto],
    column_width: Signal<f32>,
    show_synopsis: Signal<bool>,
    typography: EditorTypographySet,
    view_memory: crate::settings::EditorViewMemory,
    ids: &AppIds,
) -> ContentTab {
    let open_doc = Rc::new(OpenDoc::build(
        ctx,
        item_id,
        role,
        sub_role,
        contents,
        Signal::new(0),
        std::path::Path::new(""),
    ));
    ContentTab::new(
        ctx.clone(),
        ids.clone(),
        OpenDocsStore::new(ctx.clone()),
        open_doc,
        column_width,
        show_synopsis,
        // Its own unshared placement, defaulting to Top like a fresh install. A
        // caller that wants Side sets `tab.synopsis_placement` afterwards — these
        // are live signals, so nothing needs threading through this helper's
        // already-long argument list to do it.
        Signal::new(SynopsisPlacement::default()),
        Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
        typography,
        // A standalone tab is never a real project window; the tab tests that
        // exercise pinning build a `ContentTab` directly and pass a live one.
        crate::shared::TypewriterSettings::off(),
        // Likewise for the caret band: no Settings behind a standalone tab, so it draws none.
        crate::shared::CaretHighlightSettings::off(),
        view_memory,
        crate::settings::CorkboardDefaults::detached(),
        crate::settings::TreeExpansionViewModel::new(
            ctx.clone(),
            ids.clone(),
            crate::models::TreeExpansionService::in_memory_default(),
        ),
        // A standalone tab is never a real project window, so it is never in
        // distraction-free mode; tests that need to exercise that branch build
        // a `ContentTab` via `ContentTab::new` directly and set this signal.
        Signal::new(false),
        // Unreachable while the flag above stays `false` — same compile-time
        // default `SettingsViewModel::distraction_free_width` seeds from.
        Signal::new(crate::DISTRACTION_FREE_WIDTH_DEFAULT),
        crate::format::FormatViewModel::detached(),
        // A standalone tab plays no writing game: nothing switches one on, and
        // the two option signals below it are the shipped defaults.
        crate::writing_session::WritingGamesViewModel::detached(),
        // A standalone tab has no `WorkSession`, so it gets its own inert save
        // state rather than a null object: marking a change on it is a real state
        // change on a real object, there is simply no window polling it.
        crate::save::WorkHandle::detached(ctx.clone(), ids.clone()),
        // Words, like a fresh project: a standalone tab has no `Work` behind it to ask.
        Signal::new(GoalUnit::default()),
        // Likewise a standalone palette, not a null object: it reads/writes through the
        // same `ids` this tab got, so a caller that already set `ids.work_id` (a test
        // exercising a real Work) gets a handle that genuinely creates tags against it.
        crate::tags::TagsViewModel::detached(ctx.clone(), ids.clone()),
        // Likewise its own index, on the same `ids`: a standalone tab has no session to
        // borrow one from, and a shared handle here would be the very confusion the
        // field's own doc warns about.
        crate::statuses::StatusesViewModel::new(ctx.clone(), ids.clone()),
        crate::mentions::MentionIndex::new(ctx.clone(), ids.clone()),
    )
}

/// Build the widget for a tab (the `TabWidget` factory): dispatch each
/// `(role, sub_role)` to its own visual-tab module. Mirrors
/// `skribisto_model::COMBINATIONS`.
pub fn tab_pane(tab: &ContentTab) -> Box<dyn Widget> {
    // Carry the outgoing pane's caret and scroll into the seed before anything
    // rebuilds. This is the one door every tab body is built through, so doing it
    // here means the editors and the page scroll below can each simply read the
    // seed without caring which of them is rebuilt first — and a rebuild that is
    // nothing to do with the writer (a Promote, a settings-driven relayout) does
    // not throw them back to wherever the tab was first opened.
    //
    // A no-op on a tab that has never been built: `capture_view_state` falls back
    // to the seed when the ports are empty.
    tab.seed_view_state(tab.capture_view_state());
    let content: Box<dyn Widget> = {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        match (tab.role(), tab.sub_role()) {
            (Item, Scene) => item_scene::render(tab),
            (Item, ChapterScene) => item_chapter_scene::render(tab),
            (Item, Note) => item_note::render(tab),
            (Item, Part) => item_part::render(tab),
            (Item, BookBegin) => item_book_begin::render(tab),
            (Item, BookEnd) => item_book_end::render(tab),
            (Item, Text) => item_text::render(tab),
            (Folder, None) => folder_none::render(tab),
            (Folder, Note) => folder_note::render(tab),
            (Folder, ChapterScene) => folder_chapter_scene::render(tab),
            (Folder, Part) => folder_part::render(tab),
            (Folder, Book) => folder_book::render(tab),
            (Item, Paratext) => item_paratext::render(tab),
            (Folder, Paratext) => folder_paratext::render(tab),
            // Any pair outside the constraint matrix is invalid by construction; fall
            // back to the contentless placeholder rather than panic.
            _ => item_text::render(tab),
        }
    };
    // A trashed item can be open (from the trash dock) — show a permanent warning
    // banner between the tab bar and the content, gated to zero height otherwise
    // (the exact `VisibleWhen` shape the Ctrl+F find banner uses). Per-tab: if the
    // same item is open in both split panes, each shows its own.
    let trashed = tab.open_doc.trashed.clone();
    let item_id = tab.item_id();
    Box::new(
        VStack::new()
            .spacing(0.0)
            .child(crate::tabs::shared::editor::VisibleWhen::new(
                trashed,
                trash_banner(item_id),
            ))
            .child(Expand::new().child(Boxed::new(content))),
    )
}

/// The permanent "this item is in the Trash" warning banner shown above a trashed
/// item's editor. Its Restore button fires
/// [`AppIntent::RestoreTrashedItem`](crate::intents::AppIntent::RestoreTrashedItem), which
/// the trash view-model turns into the destination picker. No `on_dismiss` ⇒ a
/// permanent reminder (the [`crate::backup::banner`] technique).
fn trash_banner(item_id: u64) -> impl Widget {
    Banner::warning(tr!(trash_banner_title()))
        .description(tr!(trash_banner_description()))
        .action(
            Button::new(tr!(trash_banner_restore()))
                .variant(ButtonVariant::Filled)
                .on_activate_fn(move |c| {
                    c.send_intent(crate::intents::AppIntent::RestoreTrashedItem { item_id })
                }),
        )
}

/// Wraps an already-boxed widget as an `impl Widget` that fills its bounds — so a
/// `Box<dyn Widget>` (like `tab_pane`'s per-type body) can be a `VStack`/`Expand`
/// child.
pub struct Boxed {
    pending: Option<Box<dyn Widget>>,
    child_id: Option<WidgetId>,
}

impl Boxed {
    pub fn new(child: Box<dyn Widget>) -> Self {
        Self {
            pending: Some(child),
            child_id: None,
        }
    }
}

impl std::fmt::Debug for Boxed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Boxed").finish()
    }
}

impl Widget for Boxed {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        if let Some(w) = self.pending.take() {
            self.child_id = Some(ctx.add_boxed(w));
        }
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child_id
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

/// Which of a tab's **multi-document** pages the segment bar is showing.
///
/// The three pages that put more than one writing surface in front of the reader at
/// once. Everything else — a container's own page, the Corkboard, the Overview, a
/// note's Details — is either one document or none, and Ctrl+F resolves to the tab's
/// own banner (or to nothing) there.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FindPage {
    /// A manuscript stream or its full-synopsis twin: the container's own surface, then
    /// one editor per row.
    Stream(SplitFlavour),
    /// An `Item/Note`'s In prose reading: the manuscript rows it is declared present in.
    InProse,
}

/// Read the segment bar for [`FindPage`]. `None` while the bar has chosen nothing yet,
/// which resolves to the first segment — a container's own page, never a stream.
fn find_page(segment: &Signal<Option<teksilo::widgets::SegmentId>>) -> Option<FindPage> {
    use crate::tabs::shared::segments as seg;
    let shown = segment.get()?;
    if shown == seg::segment_id(seg::SEG_MANUSCRIPT) {
        Some(FindPage::Stream(SplitFlavour::Prose))
    } else if shown == seg::segment_id(seg::SEG_SYNOPSIS) {
        Some(FindPage::Stream(SplitFlavour::Synopsis))
    } else if shown == seg::segment_id(seg::SEG_NOTE_IN_PROSE) {
        Some(FindPage::InProse)
    } else {
        None
    }
}

/// An `Item/Note` — the one non-container combination with a page of many documents
/// (its In prose reading).
fn is_note(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> bool {
    matches!(
        (role, sub_role),
        (BinderItemRole::Item, BinderItemSubRole::Note)
    )
}

impl ContentTab {
    /// Wrap a shared `OpenDoc` with this tab's presentation state, plus — for a folder
    /// container — its manuscript-stream view-model. `StreamViewModel::new` returns
    /// `None` for every combination that has no stream, so one
    /// `StreamLevel::for_container` gate decides it both here and inside the view-model.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        docs: OpenDocsStore,
        open_doc: Rc<OpenDoc>,
        column_width: Signal<f32>,
        show_synopsis: Signal<bool>,
        synopsis_placement: Signal<SynopsisPlacement>,
        synopsis_side_width: Signal<f32>,
        typography: EditorTypographySet,
        typewriter: crate::shared::TypewriterSettings,
        caret_highlight: crate::shared::CaretHighlightSettings,
        view_memory: crate::settings::EditorViewMemory,
        corkboard_defaults: crate::settings::CorkboardDefaults,
        tree_expansion: crate::settings::TreeExpansionViewModel,
        distraction_free: Signal<bool>,
        distraction_free_width: Signal<f32>,
        format: crate::format::FormatViewModel,
        writing_games: crate::writing_session::WritingGamesViewModel,
        work: crate::save::WorkHandle,
        goal_unit: Signal<GoalUnit>,
        tags: crate::tags::TagsViewModel,
        statuses: crate::statuses::StatusesViewModel,
        mention_index: crate::mentions::MentionIndex,
    ) -> Self {
        // The Pace view-model gates on the same `StreamLevel::for_container` as
        // the stream (Book only). Built first, so it can borrow `app_ctx` before
        // `StreamViewModel::new` consumes it.
        let pace = PaceViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            open_doc.item_id,
            &open_doc.role,
            &open_doc.sub_role,
        );
        // The Analysis view-model, gated to the Book exactly as Pace is: Shape and the
        // balance chart are book-scale questions, and proving the segment on the Book first
        // is what keeps the positional bar honest before it is widened to Part/Chapter.
        // Staleness rides on the shared open-document edit counter, so the panel and the
        // save indicator cannot disagree about whether the manuscript has moved.
        let analysis = matches!(
            (&open_doc.role, &open_doc.sub_role),
            (
                frontend::common::entities::BinderItemRole::Folder,
                frontend::common::entities::BinderItemSubRole::Book
            )
        )
        .then(|| {
            crate::analysis::AnalysisViewModel::new(
                app_ctx.clone(),
                ids.clone(),
                open_doc.item_id,
                docs.edited_any(),
            )
        });
        // The language this tab's prose is written in, for the caret band's sentence scope.
        // Read here, while `docs` is still in hand — `stream` takes it below. Resolved once
        // per tab build, exactly as the typography and the spell dictionaries are.
        let caret_locale = docs.effective_language(open_doc.item_id).first().cloned();
        // The Corkboard exists for exactly the folder containers a stream does. Built
        // before `stream` consumes `app_ctx`.
        let corkboard = crate::models::StreamLevel::for_container(
            &open_doc.role,
            &open_doc.sub_role,
        )
        .map(|_| {
            let cd = &corkboard_defaults;
            crate::corkboard::CorkboardViewModel::new(
                app_ctx.clone(),
                ids.clone(),
                docs.clone(),
                open_doc.item_id,
                cd.nested.clone(),
                cd.card_size.clone(),
                cd.show_word_count.clone(),
                cd.show_card_numbers.clone(),
                cd.modal_size.clone(),
                cd.counting_method.clone(),
                typography.corkboard.clone(),
                crate::shared::CaretBand::new(caret_highlight.clone(), caret_locale.clone()),
                writing_games.clone(),
                format.clone(),
            )
        });
        // The Overview gates on `overview_capable`, which is deliberately *not* the
        // stream's gate — a notes folder gets a table but no stream. Built before
        // `stream` consumes `app_ctx`. It reuses the corkboard's counting-method setting
        // rather than introducing a second one: "how a word is counted" is one answer per
        // project, not one per view.
        let overview = crate::overview::OverviewViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            open_doc.item_id,
            &open_doc.role,
            &open_doc.sub_role,
            corkboard_defaults.counting_method.clone(),
            tree_expansion.clone(),
            goal_unit.clone(),
        );
        let counting_method = corkboard_defaults.counting_method.clone();
        // Kept for `Self::docs` below, since `StreamViewModel::new` consumes `docs`
        // by move, so the tab's own handle has to be cloned off before that call.
        let docs_for_tab = docs.clone();
        let stream = StreamViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            docs,
            open_doc.item_id,
            &open_doc.role,
            &open_doc.sub_role,
        );
        // A find banner only for a tab with a main prose surface to search.
        let find = open_doc.main.as_ref().map(|m| {
            // Named with the item it searches, so its hits reach the margin lane
            // through the arbiter — the lane cannot reach this view-model, which
            // lives on this tab and is findable only through focus.
            crate::search::FindViewModel::new(m.doc.clone()).for_item(open_doc.item_id)
        });
        // Seed the container's view from the per-type memory (own page = 0 when
        // disabled or for a non-segmented type).
        let segment = Signal::new(view_memory.initial(&open_doc.role, &open_doc.sub_role));
        // Where the In-prose reading publishes the rows it is showing, so the banner
        // below — built here, before that page exists — can search them.
        let in_prose_docs: Rc<RefCell<Vec<(u64, TextDocument)>>> =
            Rc::new(RefCell::new(Vec::new()));
        // The banner over this tab's pages of *many* documents. Present for a container
        // (both of its streams) and for an `Item/Note` (its In-prose reading); every
        // other combination has only single-document pages, and `find` above is theirs.
        let page_find =
            (stream.is_some() || is_note(&open_doc.role, &open_doc.sub_role)).then(|| {
                let documents: crate::search::PageDocuments = {
                    let segment = segment.clone();
                    let stream = stream.clone();
                    let open_doc = open_doc.clone();
                    let in_prose = in_prose_docs.clone();
                    Rc::new(move || match find_page(&segment) {
                        Some(FindPage::Stream(flavour)) => {
                            let Some(vm) = stream.as_ref() else {
                                return Vec::new();
                            };
                            // The container's **own** writing surface first: a chapter
                            // folder's prose is the top of the page, above its rows, and
                            // a page that skipped it would find nothing in the one
                            // paragraph the writer typed straight into the chapter.
                            let own = match flavour {
                                SplitFlavour::Prose => open_doc.main.as_ref(),
                                SplitFlavour::Synopsis => open_doc.synopsis.as_ref(),
                            };
                            own.map(|field| (open_doc.item_id, field.doc.clone()))
                                .into_iter()
                                .chain(vm.page_documents(flavour))
                                .collect()
                        }
                        Some(FindPage::InProse) => in_prose.borrow().clone(),
                        None => Vec::new(),
                    })
                };
                // A page builds one editor per row, so the only thing that can say which
                // mounted editor is showing row 31 is the registry every writing editor
                // announces itself to. Keyed by the kind the *visible* page is made of:
                // a Full Synopsis is a page of synopsis editors over the same items.
                let resolve: crate::search::ResolveEditor = {
                    let segment = segment.clone();
                    let format = format.clone();
                    Rc::new(move |item: u64| {
                        let kind = match find_page(&segment) {
                            Some(FindPage::Stream(SplitFlavour::Synopsis)) => {
                                crate::format::EditorKind::Synopsis
                            }
                            _ => crate::format::EditorKind::Prose,
                        };
                        let mut handles: Vec<_> = format
                            .handles_by_item(kind)
                            .into_iter()
                            .filter(|(shown, _)| *shown == item)
                            .map(|(_, handle)| handle)
                            .collect();
                        // **Laid out first**, and that is the whole of what this order
                        // guarantees. The registry answers in the order the writer opened
                        // their tabs in and has no notion of which pane the banner is
                        // over: the same scene can be a row on this page, a tab of its
                        // own and the other half of a split, and all three register. So
                        // this closure cannot name "the right one" — which is why the
                        // caller reveals the match in every one of them rather than
                        // stopping at the first that answers.
                        //
                        // What it can be asked is whether an editor has any geometry at
                        // all, and an editor that has laid out is one the writer may be
                        // looking at. Putting those in front decides the places where the
                        // caller does have to pick exactly one: the editor Escape hands
                        // the caret back to, and the row a stream is scrolled to when no
                        // editor could scroll to the match itself. Same rule as
                        // `FormatViewModel::handle_for_item` and `FindViewModel`'s own
                        // `laid_out_first`, restated rather than shared only because
                        // neither is reachable from here.
                        //
                        // `sort_by_key` is stable, so registration order survives inside
                        // each group.
                        handles.sort_by_key(|h| u8::from(h.content_height().is_none()));
                        handles
                    })
                };
                crate::search::FindViewModel::over_page(documents, resolve)
            });
        // Synopsis | manuscript. Seeded from the persisted width, but *hidden* and
        // at `min_size` 0 until something shows it: a Splitter counts hidden panes'
        // minimums into its own, so a pane parked at its real minimum would set a
        // floor under every tab — including the Top ones that never draw it.
        let side_splitter = SplitterModel::from_panes(
            vec![
                PaneDescriptor::new()
                    .size(synopsis_side_width.get())
                    .stretch(0.0)
                    .min_size(0.0)
                    .visible(false)
                    // Collapsible, so folding the column away leaves the divider
                    // behind as the way back. `visible(false)` removes the gutter
                    // too and would strand the writer with no affordance at all —
                    // which is exactly what the first cut of this got wrong. The
                    // two states are different things and both are used: hidden
                    // is "the setting says no synopsis", collapsed is "this tab
                    // folded it away for now".
                    .collapsible(true),
                PaneDescriptor::new()
                    .stretch(1.0)
                    .min_size(crate::tabs::shared::editor::PROSE_MIN_WIDTH),
            ],
            Orientation::Horizontal,
        );
        // Seed the disclosure from whether there is anything to disclose. Read once, at
        // build: this is the *initial* state of a per-tab toggle, not a binding — once the
        // author opens or folds the box, that choice is theirs for the life of the tab and
        // must not be overwritten by their next keystroke emptying it.
        let open_doc_epigraph_seed = open_doc
            .epigraph
            .as_ref()
            .and_then(|f| f.doc.to_plain_text().ok())
            .is_some_and(|t| !t.trim().is_empty());
        Self {
            open_doc,
            stream,
            pace,
            analysis,
            corkboard,
            overview,
            story_bible_book_filter: Signal::new(None),
            goal_unit,
            counting_method,
            ids,
            work,
            app_ctx,
            find,
            page_find,
            in_prose_docs,
            synopsis_handle: Rc::new(RefCell::new(None)),
            view_state: Signal::new(crate::shared::ViewState::default()),
            view_state_ports: Rc::new(crate::shared::ViewStatePorts::default()),
            segment_seed: RefCell::new(None),
            segment_shown: Rc::new(RefCell::new(String::new())),
            segment,
            column_width,
            epigraph_expanded: Signal::new(open_doc_epigraph_seed),
            show_synopsis,
            synopsis_placement,
            synopsis_side_width,
            side_splitter,
            typography,
            typewriter,
            caret_highlight,
            caret_locale,
            distraction_free,
            distraction_free_width,
            view_memory,
            format,
            writing_games,
            tags,
            statuses,
            mention_index,
            docs: docs_for_tab,
        }
    }

    /// The writing games this project is playing — handed to every writing
    /// editor this tab builds.
    pub fn writing_games(&self) -> crate::writing_session::WritingGamesViewModel {
        self.writing_games.clone()
    }

    /// This tab's tag palette handle. See [`Self`]'s own field doc for why this
    /// is threaded rather than read off `ctx.app_state::<TagsViewModel>()`.
    /// `pub`, not `pub(crate)`: [`note_details`] is a sibling module, the same
    /// reach every other accessor here already grants it.
    pub fn tags(&self) -> crate::tags::TagsViewModel {
        self.tags.clone()
    }

    /// This Work's status ladder.
    ///
    /// Threaded rather than resolved through `app_state`, for the reason `tags` above is:
    /// `app_state` answers with whichever window's session registered last, which on the
    /// launcher-first startup path is a throwaway session on a never-seeded `AppIds` — so
    /// the ladder would read empty for the whole session and the filter row would never
    /// appear, however many rungs the project has.
    pub fn statuses(&self) -> crate::statuses::StatusesViewModel {
        self.statuses.clone()
    }

    /// This Work's mention index. See [`Self::mention_index`]'s own field doc for why it
    /// is threaded rather than resolved through `app_state`.
    pub fn mention_index(&self) -> crate::mentions::MentionIndex {
        self.mention_index.clone()
    }

    /// This tab's shared document store. See [`Self`]'s own field doc for why
    /// this is threaded rather than read off `ctx.app_state::<OpenDocsStore>()`.
    /// `pub`, not `pub(crate)`: [`note_in_prose`] is a sibling module, the same
    /// reach [`Self::tags`] already grants it.
    pub fn docs(&self) -> OpenDocsStore {
        self.docs.clone()
    }

    /// The Corkboard view-model — `Some` only for a folder container.
    pub fn corkboard(&self) -> Option<&crate::corkboard::CorkboardViewModel> {
        self.corkboard.as_ref()
    }

    /// The Overview view-model — `Some` for every Overview-capable container.
    pub fn overview(&self) -> Option<&crate::overview::OverviewViewModel> {
        self.overview.as_ref()
    }

    /// The per-editor find banner's view-model — `Some` only when the tab has a
    /// main prose field (Scene / ChapterScene / Note).
    pub fn find(&self) -> Option<&crate::search::FindViewModel> {
        self.find.as_ref()
    }

    /// The find banner of this tab's pages of many documents — a manuscript stream, a
    /// full synopsis, a note's In-prose reading. See [`Self::page_find`].
    pub fn page_find(&self) -> Option<&crate::search::FindViewModel> {
        self.page_find.as_ref()
    }

    /// **The banner Ctrl+F opens**: the one belonging to the page actually on screen.
    ///
    /// A segmented tab has two, and which of them the writer means is decided by the
    /// segment bar, not by the tab's type. Resolved on every command rather than latched
    /// so that switching segment moves the shortcut with it.
    pub fn active_find(&self) -> Option<&crate::search::FindViewModel> {
        match find_page(&self.segment) {
            Some(_) => self.page_find.as_ref(),
            None => self.find.as_ref(),
        }
    }

    /// The sink the In-prose reading publishes the rows it is showing into, so this
    /// tab's page banner can search them. See [`Self::in_prose_docs`].
    pub fn in_prose_docs_sink(&self) -> Rc<RefCell<Vec<(u64, TextDocument)>>> {
        self.in_prose_docs.clone()
    }

    /// The sink the synopsis editor re-attaches its handle to on every build.
    pub fn synopsis_handle_sink(
        &self,
    ) -> Rc<RefCell<Option<teksilo::widgets::rich_text::EditorHandle>>> {
        self.synopsis_handle.clone()
    }

    /// This tab's synopsis editor handle, if one is currently built.
    pub fn synopsis_handle(&self) -> Option<teksilo::widgets::rich_text::EditorHandle> {
        self.synopsis_handle.borrow().clone()
    }

    /// The live ports the mounted pane publishes its editor handle and page
    /// scroll into. Handed to `writing_column` and `writing_page_scroll`.
    pub fn view_state_ports(&self) -> Rc<crate::shared::ViewStatePorts> {
        self.view_state_ports.clone()
    }

    /// The position a freshly-built pane starts from — the seed, not the live
    /// caret. Read by the deferred scroll restore, which has to know what it is
    /// aiming for before the page has a scroll range to aim within.
    pub fn view_state(&self) -> Signal<crate::shared::ViewState> {
        self.view_state.clone()
    }

    /// Snapshot the **live** caret and page scroll off the mounted pane, falling
    /// back to whatever was last seeded for a tab that has never been built (a
    /// restored tab the writer has not clicked into yet).
    ///
    /// Reads the mounted widgets rather than the `view_state` mirror on purpose:
    /// the mirror is only ever a seed, so trusting it would persist the position
    /// the tab *opened* at rather than the one the writer left it at.
    pub fn capture_view_state(&self) -> crate::shared::ViewState {
        self.view_state_ports.capture(self.view_state.get())
    }

    /// Push a position onto the **already-mounted** pane, with no rebuild — the
    /// distraction-free surface's exit path, where the tab underneath still has
    /// its editor and scroll area alive and only needs re-pointing.
    ///
    /// Also updates the seed, so a later rebuild of this tab starts from the
    /// same place rather than from where it was first opened.
    pub fn apply_view_state(&self, state: crate::shared::ViewState) {
        self.view_state.set(state);
        self.view_state_ports.apply(state);
    }

    /// Set the position a **not-yet-built** pane will start from — the workspace
    /// restore and distraction-free entry paths, both of which run before the
    /// pane exists. Read once by `writing_column`/`writing_page_scroll`.
    pub fn seed_view_state(&self, state: crate::shared::ViewState) {
        self.view_state.set(state);
    }

    /// As [`Self::seed_view_state`], for a position that came from a **remembered**
    /// one rather than from this tab's own live pane: a workspace restore, the
    /// per-item memory seeding a freshly opened tab, the distraction-free surface
    /// taking a document over.
    ///
    /// The difference is the reveal: a remembered scroll offset can have gone stale
    /// (the document was edited in another window, a narrower window reflowed the
    /// prose), and only a laid-out editor can say whether the caret still falls
    /// inside it. `tab_pane`'s rebuild carry-over deliberately does **not** come
    /// through here: a writer who scrolled away from their caret and then changed a
    /// setting must not be yanked back to it.
    pub fn seed_remembered_view_state(&self, state: crate::shared::ViewState) {
        self.view_state.set(state);
        self.view_state_ports.request_reveal();
    }

    /// Ask this tab's main editor to take keyboard focus once its pane is mounted.
    ///
    /// For a tab whose pane is **already** built there is nothing to wait for and
    /// the caller focuses the live handle itself; this is the other half, for a tab
    /// that has just been opened or was restored and never selected.
    pub fn request_focus(&self) {
        self.view_state_ports.request_focus();
    }

    /// Set the segment this tab should open on, as its stable string id. Consumed
    /// once, by `RememberSegment::wrap`, which is the only place that knows whether
    /// the string still names a segment this container has.
    pub fn seed_segment(&self, id: &str) {
        if !id.is_empty() {
            *self.segment_seed.borrow_mut() = Some(id.to_string());
        }
    }

    /// Take the seeded segment, if one is still waiting.
    pub(crate) fn take_segment_seed(&self) -> Option<String> {
        self.segment_seed.borrow_mut().take()
    }

    /// The seeded segment without consuming it: what a page asks to find out
    /// whether it is the one about to be shown.
    pub(crate) fn peek_segment_seed(&self) -> Option<String> {
        self.segment_seed.borrow().clone()
    }

    /// The slot `RememberSegment` writes the segment on screen into. It is the one
    /// place that can resolve a `SegmentId` back to the string it was derived from,
    /// and it outlives no tab, so it takes a handle rather than the tab itself.
    pub(crate) fn segment_shown_sink(&self) -> Rc<RefCell<String>> {
        self.segment_shown.clone()
    }

    /// The segment on screen, as its string id. Empty for every combination that
    /// has no segmented bar at all, which is what gets written down for them.
    pub fn segment_shown(&self) -> String {
        self.segment_shown.borrow().clone()
    }

    /// This tab's Corkboard navigation, as store ids + the live filter — `None`
    /// for a tab with no Corkboard segment, and for a board still at its own
    /// container with nothing typed (nothing worth persisting).
    pub fn capture_corkboard_state(&self) -> Option<(Vec<u64>, String)> {
        let vm = self.corkboard.as_ref()?;
        let trail = vm.trail_ids();
        let query = vm.search_query_signal().get();
        (!trail.is_empty() || !query.is_empty()).then_some((trail, query))
    }

    /// Seed the Corkboard segment's navigation before it first builds — the
    /// workspace-restore path. A no-op on a tab with no Corkboard.
    pub fn seed_corkboard_state(&self, ids: &[u64], titles: &[String], query: &str) {
        if let Some(vm) = self.corkboard.as_ref() {
            vm.restore_trail(ids, titles, query);
        }
    }

    /// The `BinderItem` this tab edits.
    pub fn item_id(&self) -> u64 {
        self.open_doc.item_id
    }

    /// The app's entity ids — the open `Work`, its `WorkInfo`, its undo stack.
    ///
    /// `pub` for the [`shared::segments`] slot. A registered segment is handed
    /// nothing but this tab, so without these two accessors it can read the
    /// document but never the project it belongs to — which makes the slot usable
    /// only by a segment that renders from its own state and needs no context at
    /// all. Capturing an `AppContext` at registration time is *not* the way round
    /// it: the app builds its own in `run`, so an extension that made one early
    /// would be reading a second, permanently empty store.
    pub fn ids(&self) -> &AppIds {
        &self.ids
    }

    /// The backend handle — the store and the event hub this tab was built
    /// against. See [`Self::ids`] for why a slot needs it.
    pub fn app_ctx(&self) -> Rc<AppContext> {
        self.app_ctx.clone()
    }

    /// This Work's save state, narrowed for the [`shared::segments`] slot: a
    /// registered segment calls `work().mark_changed(..)` with the outcome of its
    /// own mutation so the edit joins the manuscript's unsaved-changes guard.
    ///
    /// `pub` for the same reason [`Self::ids`] and [`Self::app_ctx`] are — a
    /// segment is handed nothing but this tab.
    pub fn work(&self) -> &crate::save::WorkHandle {
        &self.work
    }

    /// This project's durable `Work.unique_id`, or `None` when it has none.
    ///
    /// `None` is an **unsaved** project — the empty uid, which is not a project
    /// identity but the absence of one. Every per-project store in the workspace
    /// refuses it rather than pooling every unsaved project under one key, and a
    /// caller here gets the same answer in a shape it cannot ignore.
    ///
    /// Read through the store rather than held, because it is set when the
    /// project is first saved and this tab may have been built before that.
    /// `ids.work_id` is the authoritative "current Work" — never a model's own
    /// cached id.
    pub fn work_unique_id(&self) -> Option<String> {
        let work_id = self.ids.work_id.get()?;
        let uid = frontend::commands::work_commands::get_work(&self.app_ctx, &work_id)
            .ok()
            .flatten()?
            .unique_id;
        (!uid.is_empty()).then_some(uid)
    }
    /// What every editor on this tab needs to offer "Add as note": this project's tags
    /// and the uid its capture recents are filed under.
    ///
    /// Always `Some` here. A `ContentTab` exists only because a project is open, so
    /// there is always a palette to show, even when it is empty and the submenu comes
    /// down to Untagged alone. The `Option` is the *editor's*, for the surfaces built
    /// with no tab around them at all.
    pub fn capture_palette(&self) -> Option<crate::tabs::shared::editor::CapturePalette> {
        Some(crate::tabs::shared::editor::CapturePalette {
            tags: self.tags(),
            work_uid: self.work_unique_id(),
        })
    }
    /// The `(role, sub_role)` pair this tab edits — what [`tab_pane`] dispatches on.
    pub fn role(&self) -> &BinderItemRole {
        &self.open_doc.role
    }
    /// This tab's Analysis view-model — `Some` only on a `Folder/Book`.
    pub(crate) fn analysis(&self) -> Option<&crate::analysis::AnalysisViewModel> {
        self.analysis.as_ref()
    }

    /// The project's target unit.
    pub fn goal_unit(&self) -> &Signal<GoalUnit> {
        &self.goal_unit
    }

    /// The counting method this window displays with — the same one the Overview and the
    /// corkboard use, so a container's readout and its table cannot print different
    /// numbers for the same subtree.
    pub fn counting_method(&self) -> &Signal<skribisto_model::counting::CountingMethodSetting> {
        &self.counting_method
    }

    pub fn sub_role(&self) -> &BinderItemSubRole {
        &self.open_doc.sub_role
    }
    /// The main prose kind (Scene / Note), or `None` for non-prose combinations.
    #[allow(dead_code)] // accessor mirroring the others; asserted in tests
    pub fn kind(&self) -> Option<ProseKind> {
        self.open_doc.kind
    }
    pub fn title(&self) -> Option<&TitleField> {
        self.open_doc.title.as_ref()
    }
    pub fn subtitle(&self) -> Option<&TitleField> {
        self.open_doc.subtitle.as_ref()
    }
    pub fn main(&self) -> Option<&ProseField> {
        self.open_doc.main.as_ref()
    }
    pub fn synopsis(&self) -> Option<&ProseField> {
        self.open_doc.synopsis.as_ref()
    }
    /// The epigraph field — `Some` only for the six headed combinations the matrix
    /// allows one on (book / part / chapter, either encoding).
    pub fn epigraph(&self) -> Option<&ProseField> {
        self.open_doc.epigraph.as_ref()
    }
    /// The manuscript-stream view-model — only a folder container (Chapter / Part /
    /// Book) has one.
    pub fn stream(&self) -> Option<&StreamViewModel> {
        self.stream.as_ref()
    }

    /// The Book's writing-plan (Pace) view-model — `Some` only for a `Folder/Book`
    /// container. The Pace pane binds its signals and calls its methods.
    #[allow(dead_code)] // consumed by the Pace pane
    pub fn pace(&self) -> Option<&PaceViewModel> {
        self.pace.as_ref()
    }

    /// Persist every changed field back to its `Content` row via the shared
    /// `OpenDoc`. Idempotent — flushing a shared doc twice (once per pane) is a
    /// no-op the second time. Routes through the undo `stack`.
    pub fn flush(&self, stack: Option<u64>) -> anyhow::Result<()> {
        self.open_doc.flush(stack)
    }

    /// This tab's caret band: the shared preference plus this document's language.
    pub fn caret_band(&self) -> crate::shared::CaretBand {
        crate::shared::CaretBand::new(self.caret_highlight.clone(), self.caret_locale.clone())
    }

    /// The typography bundle for this tab's **main** prose editor: the
    /// distraction-free bundle whenever this tab's window is in distraction-free
    /// mode (checked *before* the `ProseKind` match — a window-mode axis, not a
    /// content-type one, so it must win regardless of Scene vs Note); otherwise
    /// the Notes bundle for a Note, the Scene bundle for everything else (Scene /
    /// ChapterScene, and a safe fallback for any layout without a `kind`).
    pub fn main_typography(&self) -> &EditorTypography {
        if self.distraction_free.get() {
            return &self.typography.distraction_free;
        }
        match self.open_doc.kind {
            Some(ProseKind::Note) => &self.typography.notes,
            _ => &self.typography.scene,
        }
    }

    /// Whether this tab's page is a **floating card** on the distraction-free
    /// surface's margin, rather than a full-bleed background.
    ///
    /// Only a prose tab gets the card: it is the paper you write on, and it is
    /// exactly as wide as the writing column. A corkboard, an overview table or
    /// a segmented container bar has no column to float — a narrow card behind a
    /// full-width board would read as a rendering fault — so those keep a
    /// full-bleed page.
    ///
    /// One predicate, read by both the backdrop below *and* the surface that
    /// draws the card, so the two can never disagree about which is which.
    ///
    /// **`kind.is_some()` is not the test**, tempting as it is: a chapter *folder*
    /// carries scene prose too (`prose_kind_for` maps `(Folder, ChapterScene)` to
    /// `Some(Scene)` — that is what gives it the right typography), but it renders
    /// through `folder_segmented`, a full-width segment bar over a `Switcher`. It
    /// would have got a card the width of a writing column sitting behind a
    /// corkboard. The three tabs that render `panes::prose` are exactly the
    /// `Item`-role ones with prose, so the role is the part that matters.
    pub fn floats_on_a_page(&self) -> bool {
        self.distraction_free.get() && self.renders_prose()
    }

    /// Whether this tab is one of the three that render the dual-pane writing
    /// editor — see [`renders_prose`].
    pub fn renders_prose(&self) -> bool {
        renders_prose(&self.open_doc.role, &self.open_doc.sub_role)
    }

    /// How much horizontal room this tab's Side synopsis is claiming right now —
    /// its column plus the divider, or zero whenever the synopsis is not beside
    /// the manuscript.
    ///
    /// Read by the distraction-free surface, which centres a page card and needs
    /// to know that the card grew on one side only. Everything else can read the
    /// splitter directly; the surface cannot, because it sizes the card *around*
    /// the tab rather than inside it.
    pub fn side_pane_extent(&self) -> Signal<f32> {
        let model = self.side_splitter.clone();
        self.synopsis_placement
            .zip3(&self.show_synopsis, &model.version())
            .zip(&self.synopsis_side_width)
            .map(move |((placement, show, _version), width)| {
                // `version` is in the zip purely so this recomputes when the
                // divider is folded or dragged — the fold lives on the splitter
                // now, and a derived signal has no other way to hear about it.
                let folded = model.is_collapsed(crate::tabs::shared::editor::SYNOPSIS_PANE);
                if placement.is_side() && *show && !folded {
                    *width + teksilo::widgets::splitter::SPLITTER_GUTTER_THICKNESS
                } else {
                    0.0
                }
            })
    }

    /// What `tab_backdrop` should paint behind this tab's body.
    ///
    /// `Transparent` exactly when the surface is drawing the page itself — see
    /// [`Self::floats_on_a_page`]. Everywhere else the tab paints its own
    /// `Content` page as it always has.
    pub fn backdrop_role(&self) -> SurfaceRole {
        if self.floats_on_a_page() {
            SurfaceRole::Transparent
        } else {
            SurfaceRole::Content
        }
    }

    /// The column width for this tab's **main** prose editor: the
    /// distraction-free width whenever this tab's window is in distraction-free
    /// mode, otherwise the normal writing-column width — the same window-mode
    /// axis [`Self::main_typography`] checks, and for the same reason (it must
    /// win regardless of prose kind). Nothing but the main editor's column reads
    /// this: the title, synopsis and every other pane still centre on
    /// [`Self::column_width`], per the Settings ▸ Editor Behavior hint.
    pub fn main_column_width(&self) -> &Signal<f32> {
        if self.distraction_free.get() {
            &self.distraction_free_width
        } else {
            &self.column_width
        }
    }

    /// Wire the editors' `on_change` to mark the shared doc dirty (and bump the
    /// store's aggregate edit signal). Called by each render fn when it builds the
    /// prose editors (the title fields are diffed at flush time, so they don't
    /// need a change hook).
    pub fn mark_dirty_fn(&self) -> impl Fn() + 'static {
        self.open_doc.mark_dirty_fn()
    }

    /// Commit the name fields **now** — what a title input calls when it loses focus or
    /// takes Enter.
    ///
    /// Names are not like prose. Prose can wait for the autosave debounce, but a name is
    /// also an *identifier*: the outline tree, the tab and the Inspector all show it, and
    /// they only learn about a rename when the entity is written. Leaving that to the
    /// next save meant renaming a chapter in its editor and watching the tree keep the
    /// old name until you happened to save or switch tabs. So it commits on blur, which
    /// is the moment the writer has finished typing it.
    pub fn commit_names_fn(&self) -> impl Fn() + 'static {
        let doc = self.open_doc.clone();
        let stack = self.ids.stack_id.clone();
        move || {
            let stack = stack.get();
            if let Some(f) = doc.title.as_ref() {
                let _ = f.flush(stack);
            }
            if let Some(f) = doc.subtitle.as_ref() {
                let _ = f.flush(stack);
            }
        }
    }

    /// Writer for the subtitle's tag dots.
    ///
    /// Handed out as a closure for the same reason `commit_names_fn` is: `panes.rs` composes
    /// from `&ContentTab` with no context and no access to the stack id, so the tab builds
    /// the writer and the pane just mounts it. The local mirror keeps the dots in step with
    /// a tick before the entity write echoes back.
    pub fn set_tags_fn(&self) -> crate::tags::tag_pill_field::SetTags {
        let doc = self.open_doc.clone();
        let stack = self.ids.stack_id.clone();
        std::rc::Rc::new(move |ids: Vec<u64>, _c: &mut EventContext| {
            let _ = doc.tag_probe.set_tags(&ids, stack.get());
            doc.tags.set(ids);
        })
    }
}

impl TitleField {
    /// Persist the name **to both of its homes** (the entity field the tree shows, and
    /// the title `Content` row the manuscript compiles) — see [`SingleBinderItem`].
    /// No-op when unchanged.
    pub(crate) fn flush(&self, stack: Option<u64>) -> anyhow::Result<()> {
        let val = self.value.get();
        if *self.original.borrow() == val {
            return Ok(());
        }
        match self.part {
            TitlePart::Title => self.item.set_title(&val, stack)?,
            TitlePart::SubTitle => self.item.set_sub_title(&val, stack)?,
        }
        *self.original.borrow_mut() = val;
        Ok(())
    }

    /// A cloneable "does this field hold an unsaved edit?" probe, for the input widget's
    /// effect. A `TextInput` has no `on_change` hook, so an edit is detected by diffing
    /// the bound signal against the value that was loaded — which also means an effect
    /// that fires on registration cannot mark a freshly-opened tab dirty.
    pub(crate) fn edited_probe(&self) -> Rc<dyn Fn() -> bool> {
        let value = self.value.clone();
        let original = self.original.clone();
        Rc::new(move || *original.borrow() != value.get())
    }

    /// Re-read the persisted name, discarding any live edit (see
    /// [`OpenDoc::reload`](crate::models::OpenDoc::reload)).
    pub(crate) fn reload(&self) {
        if let Some(id) = self.item.id() {
            self.item.set_id(Some(id)); // synchronous re-read
        }
        let data = match self.part {
            TitlePart::Title => self.item.title().get(),
            TitlePart::SubTitle => self.item.sub_title().get(),
        };
        self.value.set(data.clone());
        *self.original.borrow_mut() = data;
    }
}

impl ProseField {
    /// The `Content` row this field reads and writes — the anchor target for any
    /// comment created in its editor.
    pub fn content_id(&self) -> Option<u64> {
        self.content.id()
    }

    /// The row *handle*, for a caller that must work before the row exists.
    ///
    /// [`content_id`](Self::content_id) is `None` until the field's first save:
    /// a `Content` is created on write, so a chapter folder nobody has typed
    /// into yet has none. A comment can live with that — it needs a selection,
    /// so there is prose, so there is a row — but a footnote cannot: adding one
    /// is a perfectly ordinary *first* thing to do in an empty chapter, and a
    /// command that silently refuses until an invisible autosave has happened
    /// reads as the feature being broken for chapters.
    pub fn content(&self) -> SingleContent {
        self.content.clone()
    }

    pub(crate) fn flush(&self, stack: Option<u64>) -> anyhow::Result<()> {
        if !self.doc.is_modified() {
            return Ok(());
        }
        self.content.set_data(self.doc.to_djot()?);
        self.content.save(stack)?;
        self.doc.set_modified(false);
        self.flushed_revision.set(self.doc.content_revision());
        Ok(())
    }

    /// Re-read the persisted prose into the live document, discarding any live edit
    /// (see [`OpenDoc::reload`](crate::models::OpenDoc::reload)).
    pub(crate) fn reload(&self) {
        self.content.reload();
        let data = self.content.data().get();
        // A load, like `prose_field` — same reason for the synchronous form.
        let _ = self.doc.set_djot_sync(&data);
        self.doc.set_modified(false);
        self.flushed_revision.set(self.doc.content_revision());
    }

    /// Exact "has this field changed since it was last flushed (or reloaded)"
    /// check — a sharper question than `doc.is_modified()`, the plain boolean
    /// [`flush`](Self::flush) itself gates on and unconditionally clears.
    ///
    /// `is_modified()`/`OpenDoc::dirty` are flags: set on any edit, cleared on
    /// flush, with no memory of *which* edit they were cleared against. That
    /// can't distinguish "flushed, then edited again" from "flushed and quiet"
    /// — both leave the flag `false` right after a flush. Comparing
    /// `content_revision()` (a monotonic counter the document itself bumps on
    /// every real content change) against the revision recorded at the last
    /// flush is exact: a concurrent/subsequent edit bumps the live revision
    /// past the recorded one, so this stays `true` even though `is_modified()`
    /// was just reset.
    ///
    /// **Not** a fix for undo-past-save reporting dirty: `content_revision()`
    /// bumps on undo too (see `text_document::TextDocument::content_revision`'s
    /// docs), so undoing back to the exact saved text still reports stale here.
    ///
    /// Called by `OpenDoc::is_stale` (currently also `#[allow(dead_code)]` —
    /// see its doc), plus this module's own test proving the property below.
    #[allow(dead_code)]
    pub(crate) fn is_stale(&self) -> bool {
        self.doc.content_revision() != self.flushed_revision.get()
    }

    /// This field's current text as Djot (empty on a serialisation error) — the
    /// "pass the untouched role whole to the source" half of a split.
    pub(crate) fn djot(&self) -> String {
        self.doc.to_djot().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests;
