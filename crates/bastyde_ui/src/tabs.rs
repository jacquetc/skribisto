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
//! shared [`OpenDoc`] held by the [`OpenDocsStore`](crate::models::OpenDocsStore),
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

use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::text_document::TextDocument;
use bastyde::widgets::{
    Banner, Button, ButtonVariant, Expand, Orientation, PaneDescriptor, SplitterModel, VStack,
};
use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::ContentDto;

use crate::app_ids::AppIds;
use crate::models::{OpenDoc, OpenDocsStore};
use crate::singles::{SingleBinderItem, SingleContent};
use crate::view_models::{
    EditorTypography, EditorTypographySet, PaceViewModel, StreamViewModel, SynopsisPlacement,
};

// One module per valid `(role, sub_role)` combination — each a single visual tab
// (see `skribisto_model::COMBINATIONS`). `tab_pane` dispatches to them.
pub(crate) mod analysis;
pub(crate) mod corkboard;
mod folder_book;
mod folder_chapter_scene;
mod folder_none;
mod folder_note;
mod folder_part;
mod item_book_begin;
mod item_book_end;
mod item_chapter_scene;
mod item_note;
mod item_part;
mod item_scene;
mod item_text;
pub(crate) mod overview;
pub(crate) mod pace;
pub(crate) mod shared;

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
    analysis: Option<crate::view_models::AnalysisViewModel>,
    /// The Corkboard view-model — `Some` only for a folder container (Chapter /
    /// Part / Book), gated on the same [`StreamLevel::for_container`] as `stream`.
    corkboard: Option<crate::view_models::CorkboardViewModel>,
    /// The Overview view-model — `Some` for every container that offers the segment,
    /// gated on [`skribisto_model::overview_capable`]. That is a **wider** gate than the
    /// stream's and the corkboard's: a `Folder/Note` has no manuscript extent, so it has
    /// no stream, but it does have a subtree worth tabulating.
    overview: Option<crate::view_models::OverviewViewModel>,
    /// The app's entity ids — needed for the undo stack when a name field commits.
    ids: AppIds,
    /// The per-editor find banner (Ctrl+F) — `Some` only when this tab has a main
    /// prose field to search. Persisted on the tab so it survives tab rebuilds
    /// (its `FindSession` + query outlive the widget tree it draws into).
    find: Option<crate::view_models::FindViewModel>,
    /// This tab's **synopsis** editor handle, re-attached on every build the way
    /// the prose one is (a tab rebuild mints a fresh editor and a fresh handle).
    ///
    /// It does not live on `find` beside the prose handle, even though that type
    /// admits owning "the prose editor of this tab": a synopsis has no find
    /// banner, so a tab with only a synopsis would have no `FindViewModel` to
    /// hang it on. Unifying the two under one owner is worth doing, but not by
    /// giving `find` a back-reference to this tab — the `stream` field above
    /// records what closing that particular `Rc` cycle costs.
    synopsis_handle: Rc<RefCell<Option<bastyde::widgets::rich_text::EditorHandle>>>,
    /// This tab's remembered caret + page scroll, and the live ports the mounted
    /// pane publishes so both can be read back.
    ///
    /// Per-*pane*, not per-document, which is why it is here and not on the
    /// shared `OpenDoc` beside the spell and replacement sessions: two split
    /// panes on one item have one `TextDocument` but two carets. The signal is
    /// the seed a freshly-built pane starts from; the ports are the live wiring.
    /// See [`crate::view_models::ViewState`].
    view_state: Signal<crate::view_models::ViewState>,
    view_state_ports: Rc<crate::view_models::ViewStatePorts>,
    /// Selected segment for the folder container's `SegmentedControl` — per-tab
    /// (each pane keeps its own segment).
    pub segment: Signal<usize>,
    pub column_width: Signal<f32>,
    /// Persisted "show synopsis pane" setting (Settings ▸ Manuscript & Fonts),
    /// consumed live by the dual-pane writing editor.
    ///
    /// **Per-caller, not simply the global signal.** A pane tab is handed the
    /// setting itself; the distraction-free surface's tab is handed that mode's own
    /// local flag instead, so showing the synopsis while writing full-screen does
    /// not rewrite a preference that governs every other window.
    pub show_synopsis: Signal<bool>,
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
    pub typewriter: crate::view_models::TypewriterSettings,
    /// The ambient caret band, shared live from Settings — how much text around the caret
    /// is shaded, and in what colour. Every writing surface this tab builds reads it.
    pub caret_highlight: crate::view_models::CaretHighlightSettings,
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
    pub view_memory: crate::view_models::EditorViewMemory,
    /// This window's Format surfaces — every writing editor this tab builds
    /// registers with it (never process-wide `app_state`).
    pub format: crate::view_models::FormatViewModel,
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
    view_memory: crate::view_models::EditorViewMemory,
    ids: &AppIds,
) -> ContentTab {
    let open_doc = Rc::new(OpenDoc::build(
        ctx,
        item_id,
        role,
        sub_role,
        contents,
        Signal::new(0),
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
        crate::view_models::TypewriterSettings::off(),
        // Likewise for the caret band: no Settings behind a standalone tab, so it draws none.
        crate::view_models::CaretHighlightSettings::off(),
        view_memory,
        crate::view_models::CorkboardDefaults::detached(),
        crate::view_models::TreeExpansionViewModel::new(
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
        crate::view_models::FormatViewModel::detached(),
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
/// item's editor. Its Restore button fires [`AppIntent::RestoreTrashedItem`], which
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
        typewriter: crate::view_models::TypewriterSettings,
        caret_highlight: crate::view_models::CaretHighlightSettings,
        view_memory: crate::view_models::EditorViewMemory,
        corkboard_defaults: crate::view_models::CorkboardDefaults,
        tree_expansion: crate::view_models::TreeExpansionViewModel,
        distraction_free: Signal<bool>,
        distraction_free_width: Signal<f32>,
        format: crate::view_models::FormatViewModel,
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
            crate::view_models::AnalysisViewModel::new(
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
            crate::view_models::CorkboardViewModel::new(
                app_ctx.clone(),
                ids.clone(),
                docs.clone(),
                open_doc.item_id,
                cd.nested.clone(),
                cd.card_size.clone(),
                cd.show_word_count.clone(),
                cd.counting_method.clone(),
                typography.corkboard.clone(),
                crate::view_models::CaretBand::new(caret_highlight.clone(), caret_locale.clone()),
                format.clone(),
            )
        });
        // The Overview gates on `overview_capable`, which is deliberately *not* the
        // stream's gate — a notes folder gets a table but no stream. Built before
        // `stream` consumes `app_ctx`. It reuses the corkboard's counting-method setting
        // rather than introducing a second one: "how a word is counted" is one answer per
        // project, not one per view.
        let overview = crate::view_models::OverviewViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            open_doc.item_id,
            &open_doc.role,
            &open_doc.sub_role,
            corkboard_defaults.counting_method.clone(),
            tree_expansion.clone(),
        );
        let stream = StreamViewModel::new(
            app_ctx,
            ids.clone(),
            docs,
            open_doc.item_id,
            &open_doc.role,
            &open_doc.sub_role,
        );
        // A find banner only for a tab with a main prose surface to search.
        let find = open_doc
            .main
            .as_ref()
            .map(|m| crate::view_models::FindViewModel::new(m.doc.clone()));
        // Seed the container's view from the per-type memory (own page = 0 when
        // disabled or for a non-segmented type).
        let segment = Signal::new(view_memory.initial(&open_doc.sub_role));
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
        Self {
            open_doc,
            stream,
            pace,
            analysis,
            corkboard,
            overview,
            ids,
            find,
            synopsis_handle: Rc::new(RefCell::new(None)),
            view_state: Signal::new(crate::view_models::ViewState::default()),
            view_state_ports: Rc::new(crate::view_models::ViewStatePorts::default()),
            segment,
            column_width,
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
        }
    }

    /// The Corkboard view-model — `Some` only for a folder container.
    pub fn corkboard(&self) -> Option<&crate::view_models::CorkboardViewModel> {
        self.corkboard.as_ref()
    }

    /// The Overview view-model — `Some` for every Overview-capable container.
    pub fn overview(&self) -> Option<&crate::view_models::OverviewViewModel> {
        self.overview.as_ref()
    }

    /// The per-editor find banner's view-model — `Some` only when the tab has a
    /// main prose field (Scene / ChapterScene / Note).
    pub fn find(&self) -> Option<&crate::view_models::FindViewModel> {
        self.find.as_ref()
    }

    /// The sink the synopsis editor re-attaches its handle to on every build.
    pub fn synopsis_handle_sink(
        &self,
    ) -> Rc<RefCell<Option<bastyde::widgets::rich_text::EditorHandle>>> {
        self.synopsis_handle.clone()
    }

    /// This tab's synopsis editor handle, if one is currently built.
    pub fn synopsis_handle(&self) -> Option<bastyde::widgets::rich_text::EditorHandle> {
        self.synopsis_handle.borrow().clone()
    }

    /// The live ports the mounted pane publishes its editor handle and page
    /// scroll into. Handed to `writing_column` and `writing_page_scroll`.
    pub fn view_state_ports(&self) -> Rc<crate::view_models::ViewStatePorts> {
        self.view_state_ports.clone()
    }

    /// The position a freshly-built pane starts from — the seed, not the live
    /// caret. Read by the deferred scroll restore, which has to know what it is
    /// aiming for before the page has a scroll range to aim within.
    pub fn view_state(&self) -> Signal<crate::view_models::ViewState> {
        self.view_state.clone()
    }

    /// What a freshly-built pane is handed: the position to start at, and the
    /// ports to publish itself into.
    pub fn view_state_binding(&self) -> crate::view_models::ViewStateBinding {
        crate::view_models::ViewStateBinding {
            initial: self.view_state.get(),
            ports: self.view_state_ports.clone(),
        }
    }

    /// Snapshot the **live** caret and page scroll off the mounted pane, falling
    /// back to whatever was last seeded for a tab that has never been built (a
    /// restored tab the writer has not clicked into yet).
    ///
    /// Reads the mounted widgets rather than the `view_state` mirror on purpose:
    /// the mirror is only ever a seed, so trusting it would persist the position
    /// the tab *opened* at rather than the one the writer left it at.
    pub fn capture_view_state(&self) -> crate::view_models::ViewState {
        self.view_state_ports.capture(self.view_state.get())
    }

    /// Push a position onto the **already-mounted** pane, with no rebuild — the
    /// distraction-free surface's exit path, where the tab underneath still has
    /// its editor and scroll area alive and only needs re-pointing.
    ///
    /// Also updates the seed, so a later rebuild of this tab starts from the
    /// same place rather than from where it was first opened.
    pub fn apply_view_state(&self, state: crate::view_models::ViewState) {
        self.view_state.set(state);
        let max_caret = self
            .main()
            .map(|m| m.doc.character_count())
            .unwrap_or(usize::MAX);
        self.view_state_ports.apply(state, max_caret);
    }

    /// Set the position a **not-yet-built** pane will start from — the workspace
    /// restore and distraction-free entry paths, both of which run before the
    /// pane exists. Read once by `writing_column`/`writing_page_scroll`.
    pub fn seed_view_state(&self, state: crate::view_models::ViewState) {
        self.view_state.set(state);
    }

    /// The `BinderItem` this tab edits.
    pub fn item_id(&self) -> u64 {
        self.open_doc.item_id
    }
    /// The `(role, sub_role)` pair this tab edits — what [`tab_pane`] dispatches on.
    pub fn role(&self) -> &BinderItemRole {
        &self.open_doc.role
    }
    /// This tab's Analysis view-model — `Some` only on a `Folder/Book`.
    pub(crate) fn analysis(&self) -> Option<&crate::view_models::AnalysisViewModel> {
        self.analysis.as_ref()
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
    pub fn caret_band(&self) -> crate::view_models::CaretBand {
        crate::view_models::CaretBand::new(self.caret_highlight.clone(), self.caret_locale.clone())
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
                    *width + bastyde::widgets::splitter::SPLITTER_GUTTER_THICKNESS
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
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    /// A per-type typography set with distinguishable fonts (Scene/Synopsis =
    /// Literata, Notes = Inter) so tests can assert the right bundle reaches the
    /// right editor.
    fn test_typography() -> EditorTypographySet {
        let bundle = |family: &str| EditorTypography {
            font_family: Signal::new(family.to_string()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
        };
        EditorTypographySet {
            scene: bundle("Literata"),
            synopsis: bundle("Literata"),
            notes: bundle("Inter"),
            corkboard: bundle("Literata"),
            distraction_free: bundle("Literata"),
        }
    }

    /// Every valid `(role, sub_role)` must build a tab and lay it out headlessly
    /// without panicking — the per-combination dispatch + each tab's widget tree.
    /// The `bool` flags which combinations open the dual-pane prose editor (Scene /
    /// ChapterScene / Note): those carry a prose kind + a main editor; the rest do
    /// not.
    #[test]
    fn every_combination_builds_and_lays_out() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let combos = [
            (Item, Scene, true),
            (Item, ChapterScene, true),
            (Item, Note, true),
            (Item, Part, false),
            (Item, BookBegin, false),
            (Item, BookEnd, false),
            (Item, Text, false),
            (Folder, None, false),
            // A chapter folder carries its own prose, like the flat ChapterScene.
            (Folder, ChapterScene, true),
            (Folder, Part, false),
            (Folder, Book, false),
            (Folder, Note, false),
        ];
        let ctx = Rc::new(AppContext::new());
        for (role, sub_role, is_prose) in combos {
            let tab = tab_for(
                &ctx,
                1,
                &role,
                &sub_role,
                &[],
                Signal::new(700.0),
                Signal::new(true),
                test_typography(),
                crate::view_models::EditorViewMemory::detached(false),
                &AppIds::new(),
            );
            // Prose tabs carry a kind + a main editor; every other combination has
            // neither.
            if is_prose {
                assert!(
                    tab.kind().is_some(),
                    "{role:?}/{sub_role:?} prose needs a kind"
                );
                assert!(
                    tab.main().is_some(),
                    "{role:?}/{sub_role:?} prose needs main"
                );
            } else {
                assert!(
                    tab.kind().is_none(),
                    "{role:?}/{sub_role:?} non-prose has no kind"
                );
            }
            let mut tree = WidgetTree::new();
            let id = tree.add_boxed(tab_pane(&tab));
            tree.layout(bastyde::prelude::SizeProposal::exact(1000.0, 700.0));
            assert!(
                tree.bounds(id).width > 0.0,
                "{role:?}/{sub_role:?} laid out to zero width"
            );
        }
    }

    /// A trashed open item shows the permanent "in the Trash" warning banner above
    /// its editor (gated on `open_doc.trashed`), and the tab still lays out.
    #[test]
    fn a_trashed_tab_shows_the_restore_banner() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.open_doc.trashed.set(true);
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(bastyde::prelude::SizeProposal::exact(1000.0, 700.0));
        assert!(
            first_of_type(&tree, id, "Banner").is_some(),
            "a trashed tab must render the Trash banner"
        );
        assert!(
            tree.bounds(id).width > 0.0,
            "trashed tab laid out to zero width"
        );
    }

    /// The three folder containers (Book / Part / Chapter) render a `SegmentedControl`
    /// bar. The per-type "last view" memory wraps that body in a `RememberSegment`
    /// passthrough; this pins that the wrapper doesn't swallow the bar's layout (the
    /// bar must still be present **and** lay out to a non-zero size).
    #[test]
    fn segmented_containers_lay_out_their_bar() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        for sub_role in [Book, Part, ChapterScene] {
            let tab = tab_for(
                &ctx,
                1,
                &Folder,
                &sub_role,
                &[],
                Signal::new(700.0),
                Signal::new(true),
                test_typography(),
                crate::view_models::EditorViewMemory::detached(false),
                &AppIds::new(),
            );
            let mut tree = WidgetTree::new();
            let id = tree.add_boxed(tab_pane(&tab));
            tree.layout(bastyde::prelude::SizeProposal::exact(1000.0, 700.0));
            let bar = first_of_type(&tree, id, "SegmentedControl").unwrap_or_else(|| {
                panic!("Folder/{sub_role:?} has no SegmentedControl in its tree")
            });
            let b = tree.bounds(bar);
            assert!(
                b.width > 0.0 && b.height > 0.0,
                "Folder/{sub_role:?} segmented bar laid out to zero size ({b:?})"
            );
        }
    }

    /// A **notes folder** is segmented too, now that it offers an Overview of its
    /// contents. It used to be a bare synopsis page with no bar at all, so this pins the
    /// bar's existence, not only its size.
    #[test]
    fn a_notes_folder_lays_out_its_two_segment_bar() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            1,
            &Folder,
            &Note,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(bastyde::prelude::SizeProposal::exact(1000.0, 700.0));
        let bar = first_of_type(&tree, id, "SegmentedControl")
            .expect("Folder/Note has no SegmentedControl in its tree");
        let b = tree.bounds(bar);
        assert!(
            b.width > 0.0 && b.height > 0.0,
            "the notes folder's segmented bar laid out to zero size ({b:?})"
        );
    }

    /// The Overview view-model exists for exactly the containers that offer the segment
    /// — a **wider** set than the stream's, because a notes folder has a subtree but no
    /// manuscript extent.
    ///
    /// `ContentTab` and `skribisto_model::overview_capable` must agree: a tab that built
    /// no view-model would render the segment's pane as an empty `VStack`, which looks
    /// like a bug in the table rather than a gate that said no.
    #[test]
    fn the_overview_view_model_matches_the_model_gate() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let combos = [
            (Item, Scene),
            (Item, ChapterScene),
            (Item, Note),
            (Item, Part),
            (Item, BookBegin),
            (Item, BookEnd),
            (Item, Text),
            (Folder, None),
            (Folder, ChapterScene),
            (Folder, Part),
            (Folder, Book),
            (Folder, Note),
        ];
        for (role, sub_role) in combos {
            let tab = tab_for(
                &ctx,
                1,
                &role,
                &sub_role,
                &[],
                Signal::new(700.0),
                Signal::new(true),
                test_typography(),
                crate::view_models::EditorViewMemory::detached(false),
                &AppIds::new(),
            );
            assert_eq!(
                tab.overview().is_some(),
                skribisto_model::overview_capable(&role, &sub_role),
                "{role:?}/{sub_role:?}: the tab and the model disagree about the Overview"
            );
        }
    }

    /// The Overview segment **mounts and lays out** — the positional
    /// `SegmentedControl` ↔ `Switcher` contract, end to end.
    ///
    /// The two are matched by index, not by name, so a segment added without its child
    /// (or in the wrong order) does not fail to compile: it silently shows the *previous*
    /// view under the new label. Selecting the last index and finding a real table is
    /// what proves the pairing.
    #[cfg(feature = "mocks")]
    #[test]
    fn the_overview_segment_mounts_a_table() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        // (sub_role, the Overview's index in that container's bar)
        for (sub_role, overview_index) in [(ChapterScene, 4), (Part, 4), (Book, 6), (Note, 1)] {
            let tab = tab_for(
                &ctx,
                101, // the mock Book container — its fixture subtree has rows
                &Folder,
                &sub_role,
                &[],
                Signal::new(700.0),
                Signal::new(true),
                test_typography(),
                crate::view_models::EditorViewMemory::detached(false),
                &AppIds::new(),
            );
            tab.segment.set(overview_index);
            // The Overview pane subscribes to backend events in its wiring child, so it
            // needs a tree that has an event source (see `crate::test_support`).
            let mut tree = crate::test_support::tree_with_events(&ctx);
            let id = tree.add_boxed(tab_pane(&tab));
            tree.layout(bastyde::prelude::SizeProposal::exact(1000.0, 700.0));
            let table = first_containing(&tree, id, "TreeTableView").unwrap_or_else(|| {
                panic!(
                    "Folder/{sub_role:?} segment {overview_index} mounted no TreeTableView — \
                     the segment and its Switcher child have drifted out of step"
                )
            });
            let b = tree.bounds(table);
            assert!(
                b.width > 0.0 && b.height > 0.0,
                "Folder/{sub_role:?}: the Overview table laid out to zero size ({b:?})"
            );
        }
    }

    /// The Analysis segment mounts its own pane, and only on a Book.
    ///
    /// Same reasoning as the Overview test above, and the same trap: the bar and the
    /// `Switcher` are matched by position, so an inserted segment whose child was forgotten
    /// compiles cleanly and silently shows the neighbouring view under the new label. This
    /// selects Analysis by index and insists on finding the pane that belongs there.
    #[cfg(feature = "mocks")]
    #[test]
    fn only_the_book_mounts_an_analysis_segment() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());

        // Book: own page / Full Book / Full Synopsis / Pace / Analysis / Corkboard / Overview
        let tab = tab_for(
            &ctx,
            101,
            &Folder,
            &Book,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.segment.set(4);
        assert!(tab.analysis().is_some(), "a Book carries an Analysis view-model");

        // The pane starts an analysis on open and subscribes to long-operation events, so
        // it needs a tree with an event source.
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(bastyde::prelude::SizeProposal::exact(1000.0, 700.0));
        let pane = first_containing(&tree, id, "AnalysisPane").expect(
            "segment 4 of a Book mounted no AnalysisPane — the segment and its Switcher \
             child have drifted out of step",
        );
        let b = tree.bounds(pane);
        assert!(b.width > 0.0 && b.height > 0.0, "the Analysis pane laid out to zero size ({b:?})");

        // A Part has no Analysis at all: the gate is the same Book-only one Pace uses, and
        // widening it by accident would put a book-scale report on a chapter.
        let part = tab_for(
            &ctx,
            101,
            &Folder,
            &Part,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        assert!(part.analysis().is_none(), "only a Book is analysed for now");
    }

    /// The tags column reaches the table, and renders dots only for rows that have tags.
    ///
    /// The Overview was the last view showing binder rows that did not surface tags (the
    /// stream, corkboard and editor all did), because its column was reserved as an inert
    /// seam while tags were built in a parallel worktree. This pins that the seam is
    /// actually wired, not merely still reserved.
    #[cfg(feature = "mocks")]
    #[test]
    fn the_overview_shows_tag_dots_for_tagged_rows_only() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            101,
            &Folder,
            &Book,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.segment.set(6); // the Book's Overview (Pace and Analysis sit before it)
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(bastyde::prelude::SizeProposal::exact(1200.0, 700.0));

        // The fixture tags scenes 201 and 202 and leaves the rest untagged, so a correctly
        // wired column mounts *some* dot rows but not one per row.
        let mut dot_rows = 0;
        count_containing(&tree, id, "TagDotsRow", &mut dot_rows);
        assert!(
            dot_rows > 0,
            "no TagDotsRow in the Overview - the tags column is not wired"
        );
        let rows = {
            use bastyde::data::TreeDataSource;
            tab.overview()
                .expect("a Book has an Overview")
                .rows()
                .visible_count()
        };
        assert!(
            dot_rows < rows,
            "every one of the {rows} rows mounted a dot row ({dot_rows}); an untagged row \
             must render an empty cell, or the column stops distinguishing tagged from not"
        );
    }

    /// Count nodes at/under `root` whose type name contains `needle`.
    fn count_containing(tree: &WidgetTree, root: WidgetId, needle: &str, n: &mut usize) {
        if tree
            .widget_type_name(root)
            .is_some_and(|t| t.contains(needle))
        {
            *n += 1;
        }
        for c in tree.children(root) {
            count_containing(tree, c, needle, n);
        }
    }

    /// F2 opens the editor on the **focused cell**, and does so exactly once.
    ///
    /// The table used to carry its own F2 handler on top of the one
    /// `TreeTableView` already implements (`EditTrigger::F2OrTypeOrDoubleClick`
    /// is the default, and both editable columns opt in). Bastyde fires the
    /// external handler and the widget's own with no short-circuit on
    /// `Handled`, so both ran: the widget's opened the *focused* cell, mine
    /// opened the first *selected* row. When those disagree the second call
    /// commits and closes the first one's edit before opening its own — the
    /// user gets the wrong row, or a stray undo entry, depending on ordering.
    ///
    /// So the duplicate is gone and this pins what remains: the surviving
    /// handler is the widget's, and it works.
    #[cfg(feature = "mocks")]
    #[test]
    fn f2_opens_the_editor_on_the_focused_cell() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        use bastyde::data::TreeDataSource;
        use bastyde::widgets::TreeTableView;
        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            101,
            &Folder,
            &Book,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.segment.set(6); // the Book's Overview, after Pace and Analysis
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let root = tree.add_boxed(tab_pane(&tab));
        tree.layout(bastyde::prelude::SizeProposal::exact(1200.0, 700.0));

        fn find(tree: &WidgetTree, id: WidgetId, needle: &str) -> Option<WidgetId> {
            if tree
                .widget_type_name(id)
                .is_some_and(|t| t.contains(needle))
            {
                return Some(id);
            }
            tree.children(id)
                .into_iter()
                .find_map(|c| find(tree, c, needle))
        }
        let table =
            find(&tree, root, "TreeTableView").expect("the Overview mounts a TreeTableView");

        tree.focus(table);
        tree.widget_as_any(table)
            .unwrap()
            .downcast_ref::<TreeTableView<crate::models::OverviewRow>>()
            .expect("the table is keyed by OverviewRow")
            .set_focused_cell(0, 0);

        let vm = tab.overview().unwrap().clone();
        // `Option::None` spelled out: the `BinderItemSubRole::*` glob above puts a
        // `None` *variant* in scope, which is what a bare `None` would resolve to.
        assert_eq!(
            vm.editing_cell().get(),
            Option::None,
            "nothing is being edited yet"
        );

        tree.press_key(Key::F2, Modifiers::NONE);

        let (uid, col) = vm.editing_cell().get().expect(
            "F2 must open an editor; removing the app-side handler must not have \
                     taken the only working one with it",
        );
        assert_eq!(col, crate::models::COL_TITLE, "F2 edits the focused column");
        assert_eq!(
            Some(uid),
            vm.rows().key_at(0),
            "F2 edits the focused row, not merely some row"
        );
    }

    /// "Rename" must actually open the cell editor.
    ///
    /// The view-model addresses an edit by `(uid, col_id)` — a durable key, because this
    /// table re-sources constantly — while `CellContext::is_editing`, the only thing the
    /// cell delegates consult, is the *widget's* `(row, display_pos)`. Nothing bridged the
    /// two, so the menu item set the intent and no cell ever noticed: it did nothing at
    /// all, silently, with no error anywhere.
    ///
    /// Differential on purpose. The header carries a `SearchField`, which is itself a
    /// `TextInput`, so an absolute count would pass on the broken code; only the change
    /// across `begin_edit` is the cell editor.
    #[cfg(feature = "mocks")]
    #[test]
    fn renaming_an_overview_row_mounts_a_cell_editor() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            101,
            &Folder,
            &Book,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.segment.set(6); // the Book's Overview, after Pace and Analysis

        // The pane is rebuilt at each step rather than mutated: the editing signal is
        // bound at `BindingLevel::Rebuild`, so a rebuild is exactly what the framework
        // does, and a fresh `TreeTableView` (with a fresh, empty `editing_cell`) is the
        // condition the seed has to survive.
        let inputs = |tab: &ContentTab| {
            let mut tree = crate::test_support::tree_with_events(&ctx);
            let id = tree.add_boxed(tab_pane(tab));
            tree.layout(bastyde::prelude::SizeProposal::exact(1200.0, 700.0));
            let mut n = 0;
            count_containing(&tree, id, "TextInput", &mut n);
            n
        };

        let idle = inputs(&tab);

        let vm = tab.overview().unwrap().clone();
        let uid = common::uid::fixture_uid(201);
        vm.begin_edit(uid, crate::models::COL_TITLE);
        let editing = inputs(&tab);
        assert!(
            editing > idle,
            "begin_edit set the view-model's editing signal but no cell editor mounted \
             ({idle} -> {editing} TextInputs)"
        );

        // Seeded from the row it targets, not left blank — an editor that opens empty
        // would silently clear the title on commit.
        assert!(
            !vm.edit_buffer().text.get().is_empty(),
            "the open editor was not seeded with the row's title"
        );

        // ...and it closes again. Without this the mount could be a one-way latch that
        // never returns the table to its normal cells.
        vm.cancel_edit();
        assert_eq!(
            inputs(&tab),
            idle,
            "cancelling the edit left the cell editor mounted"
        );
    }

    /// A Book's bar carries the extra "Pace" segment, so its Overview sits one further
    /// along than a Chapter's or a Part's. Pinned because the index above is a magic
    /// number that only the bar's construction order justifies.
    #[cfg(feature = "mocks")]
    #[test]
    fn only_a_book_has_the_extra_book_only_segments() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let segments = |sub_role: BinderItemSubRole| {
            let tab = tab_for(
                &ctx,
                101,
                &Folder,
                &sub_role,
                &[],
                Signal::new(700.0),
                Signal::new(true),
                test_typography(),
                crate::view_models::EditorViewMemory::detached(false),
                &AppIds::new(),
            );
            let mut tree = WidgetTree::new();
            let id = tree.add_boxed(tab_pane(&tab));
            tree.layout(bastyde::prelude::SizeProposal::exact(1000.0, 700.0));
            let bar = first_of_type(&tree, id, "SegmentedControl").expect("a bar");
            tree.children(bar).len()
        };
        let chapter = segments(ChapterScene);
        assert_eq!(
            segments(Part),
            chapter,
            "a Part and a Chapter offer the same views"
        );
        assert_eq!(
            segments(Book),
            chapter + 2,
            "a Book adds Pace and Analysis, which is why its Overview index is two higher"
        );
    }

    /// Switching a container's view persists it per type, and a newly-opened tab of
    /// the same type inherits it — the whole "remember last view" chain: the built
    /// tab's `RememberSegment` effect writes `EditorViewMemory` on a segment change,
    /// and `ContentTab::new` seeds a new tab's segment from it.
    #[test]
    fn switching_a_container_view_persists_and_a_new_tab_inherits() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let mem = crate::view_models::EditorViewMemory::detached(true);
        let open = |id: u64| {
            tab_for(
                &ctx,
                id,
                &Folder,
                &ChapterScene,
                &[],
                Signal::new(700.0),
                Signal::new(true),
                test_typography(),
                mem.clone(),
                &AppIds::new(),
            )
        };
        // Open a chapter; it starts on its own page.
        let chapter1 = open(1);
        assert_eq!(chapter1.segment.get(), 0);
        let mut tree = WidgetTree::new();
        tree.add_boxed(tab_pane(&chapter1)); // sets up the persist effect
        tree.layout(bastyde::prelude::SizeProposal::exact(1000.0, 700.0));
        // Switch it to "Full Chapter" (index 1).
        chapter1.segment.set(1);
        assert_eq!(
            mem.initial(&ChapterScene),
            1,
            "the chosen view was remembered"
        );
        // A newly-opened chapter inherits it.
        assert_eq!(
            open(2).segment.get(),
            1,
            "a new chapter opens on Full Chapter"
        );
        // ...but a Scene (no segmented control) is unaffected.
        let scene = tab_for(
            &ctx,
            3,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            mem.clone(),
            &AppIds::new(),
        );
        assert_eq!(scene.segment.get(), 0);
    }

    /// Two open tabs of the same container type share one per-type memory: each of
    /// their `SegmentedControl` switches writes it, and the *last switch wins* — a
    /// tab's own switch never fights a peer's. This works only because `ctx.effect`
    /// fires on *changes*, not on setup (proven by the counter-test above, which
    /// starts each on its own page and confirms nothing is written until a switch):
    /// so merely having a second same-type tab open — or a tab rebuilding — installs
    /// observers that stay quiet, and only a real switch persists.
    ///
    /// (Built at segment 0 / the own page throughout: a non-zero segment mounts the
    /// manuscript-stream pane, whose event wiring needs an app-level event source
    /// the headless `WidgetTree` doesn't provide — so the switches are made via the
    /// signal, which fires the persist effect without swapping the mounted pane.)
    #[test]
    fn same_type_tabs_share_one_last_view_and_the_last_switch_wins() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let mem = crate::view_models::EditorViewMemory::detached(true);
        let open = |id: u64| {
            tab_for(
                &ctx,
                id,
                &Folder,
                &ChapterScene,
                &[],
                Signal::new(700.0),
                Signal::new(true),
                test_typography(),
                mem.clone(),
                &AppIds::new(),
            )
        };
        // Both open on their own page (memory starts at 0), so no stream mounts.
        let a = open(1);
        let b = open(2);
        assert_eq!(a.segment.get(), 0);
        assert_eq!(b.segment.get(), 0);
        let build = |tab: &ContentTab| {
            let mut tree = WidgetTree::new();
            tree.add_boxed(tab_pane(tab));
            tree.layout(bastyde::prelude::SizeProposal::exact(1000.0, 700.0));
            tree // keep alive so the effect it installed stays live
        };
        let _ta = build(&a);
        let _tb = build(&b);
        // Merely opening + building a second same-type tab wrote nothing.
        assert_eq!(mem.initial(&ChapterScene), 0);
        a.segment.set(1); // A switches → Full Chapter
        assert_eq!(mem.initial(&ChapterScene), 1);
        b.segment.set(2); // B switches → Full Synopsis: last switch wins
        assert_eq!(mem.initial(&ChapterScene), 2);
        a.segment.set(0); // A switches back → its switch wins in turn
        assert_eq!(mem.initial(&ChapterScene), 0);
    }

    /// First node at/under `root` whose type name *contains* `needle` (DFS pre-order).
    ///
    /// Separate from [`first_of_type`] because a generic widget's `type_name` carries its
    /// parameters (`TreeTableView<..::OverviewRow>`), so a suffix match never fires on one.
    fn first_containing(tree: &WidgetTree, root: WidgetId, needle: &str) -> Option<WidgetId> {
        if tree
            .widget_type_name(root)
            .is_some_and(|n| n.contains(needle))
        {
            return Some(root);
        }
        tree.children(root)
            .into_iter()
            .find_map(|c| first_containing(tree, c, needle))
    }

    /// A Scene tab built with the given typewriter setting, laid out, plus the
    /// page `ScrollArea`'s maximum scroll offset.
    ///
    /// The pin lives on the editors, but the *range* that lets the last line
    /// reach it lives on the page — this is the link between them.
    fn scene_page_max_scroll(typewriter: crate::view_models::TypewriterSettings) -> (f32, f32) {
        use bastyde::widgets::ScrollArea;
        let ctx = Rc::new(AppContext::new());
        let open_doc = Rc::new(OpenDoc::build(
            &ctx,
            1,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            Signal::new(0),
        ));
        let tab = ContentTab::new(
            ctx.clone(),
            AppIds::new(),
            OpenDocsStore::new(ctx.clone()),
            open_doc,
            Signal::new(700.0),
            Signal::new(true),
            Signal::new(SynopsisPlacement::default()),
            Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
            test_typography(),
            typewriter,
            crate::view_models::CaretHighlightSettings::off(),
            crate::view_models::EditorViewMemory::detached(false),
            crate::view_models::CorkboardDefaults::detached(),
            crate::view_models::TreeExpansionViewModel::new(
                ctx.clone(),
                AppIds::new(),
                crate::models::TreeExpansionService::in_memory_default(),
            ),
            Signal::new(false),
            Signal::new(crate::DISTRACTION_FREE_WIDTH_DEFAULT),
            crate::view_models::FormatViewModel::detached(),
        );
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let root = tree.add_boxed(tab_pane(&tab));
        tree.layout(bastyde::prelude::SizeProposal::exact(1000.0, 400.0));
        let sa_id = first_of_type(&tree, root, "ScrollArea").expect("the page scrolls");
        let max_scroll = tree
            .widget_as_any(sa_id)
            .and_then(|a| a.downcast_ref::<ScrollArea>())
            .expect("ScrollArea opts into as_any")
            .max_scroll_y_signal()
            .get();
        (max_scroll, tree.bounds(sa_id).height)
    }

    /// With typewriter scrolling on, the writing page must be able to scroll
    /// *past* its last line — otherwise the pin silently stops working over the
    /// final page, which is exactly where a writer spends their time. With it
    /// off, the page must stop at its content like any other.
    ///
    /// This is the one link the unit tests either side of it cannot cover: that
    /// every writing surface really does go through `writing_page_scroll`.
    #[test]
    fn the_writing_page_buys_scroll_range_only_while_pinning() {
        use crate::view_models::{TypewriterAnchor, TypewriterSettings};

        let on = |a: TypewriterAnchor| {
            scene_page_max_scroll(TypewriterSettings::new(
                Signal::new(true),
                Signal::new(Some(a)),
            ))
        };

        let (off, _) = scene_page_max_scroll(TypewriterSettings::off());
        let (middle, viewport) = on(TypewriterAnchor::Middle);
        let (top_third, _) = on(TypewriterAnchor::TopThird);
        let (bottom_quarter, _) = on(TypewriterAnchor::BottomQuarter);

        assert!(
            middle > off,
            "pinning must buy scroll range past the last line (off={off}, on={middle})"
        );

        // A higher pin needs more room beneath it, so the range grows as the
        // anchor rises. Ordering alone would pass on a constant offset — the
        // exact differences below are what tie the range to the anchor.
        assert!(top_third > middle && middle > bottom_quarter);

        // Each enabled page differs from the next by exactly the difference in
        // their scroll-past-end fractions, scaled by the viewport. Asserted as a
        // *difference* so it holds whatever the tab chrome leaves for content —
        // an absolute expectation would encode this tab's incidental layout.
        let expected = |a: TypewriterAnchor, b: TypewriterAnchor| {
            (a.scroll_past_end() - b.scroll_past_end()) * viewport
        };
        assert!(
            (top_third - middle - expected(TypewriterAnchor::TopThird, TypewriterAnchor::Middle))
                .abs()
                < 0.5,
            "top-third vs middle: got {}, expected {}",
            top_third - middle,
            expected(TypewriterAnchor::TopThird, TypewriterAnchor::Middle)
        );
        assert!(
            (middle
                - bottom_quarter
                - expected(TypewriterAnchor::Middle, TypewriterAnchor::BottomQuarter))
            .abs()
                < 0.5,
            "middle vs bottom-quarter: got {}, expected {}",
            middle - bottom_quarter,
            expected(TypewriterAnchor::Middle, TypewriterAnchor::BottomQuarter)
        );
    }

    /// First node at/under `root` whose fully-qualified type name ends with `suffix`
    /// (DFS pre-order); type names come from `std::any::type_name`, so match the leaf.
    fn first_of_type(tree: &WidgetTree, root: WidgetId, suffix: &str) -> Option<WidgetId> {
        if tree
            .widget_type_name(root)
            .is_some_and(|n| n.ends_with(suffix))
        {
            return Some(root);
        }
        tree.children(root)
            .into_iter()
            .find_map(|c| first_of_type(tree, c, suffix))
    }

    /// The Book's Pace dashboard **must reflow with width**. `pace_pane` drops the
    /// prose-column `centered()` wrapper and flows its section panels through a `ColumnFlow`
    /// under the scroll viewport, with the empty/planner swap done by a *layout-forwarding*
    /// composing widget (`PaceBody`), not a `Switcher`. This pins that the resulting chain —
    /// `ScrollArea > Padding > VStack > (forwarding swap) > ColumnFlow` — carries the
    /// viewport's **bounded** width all the way to the flow, so it packs into several short
    /// columns when wide and one tall column when narrow.
    ///
    /// Two traps this guards, both of which reported one column at *every* width: the old
    /// `centered()` measured the content at its **hugging** width, and a `Switcher` measures
    /// *all* its children with an **unbounded** width to size to the largest. Five 200px
    /// panels: wide ≈ two rows, narrow ≈ five.
    #[test]
    fn column_flow_reflows_in_the_pace_scaffold() {
        use bastyde::widgets::{ColumnFlow, FixedSize, Padding, RectWidget, ScrollArea, VStack};

        let flow_height = |width: f32| -> f32 {
            let mut flow = ColumnFlow::new()
                .min_column_width(300.0)
                .max_columns(3)
                .column_spacing(12.0)
                .item_spacing(12.0);
            for _ in 0..5 {
                flow = flow.child(FixedSize::new().height(200.0).child(RectWidget::new()));
            }
            // `Forwarder` stands in for `pace_pane`'s `PaceBody` swap: it forwards layout to
            // its one child, so (unlike a `Switcher`) the parent's bounded width reaches it.
            let body = VStack::new().child(Forwarder {
                child: Some(Box::new(flow)),
                root: None,
            });
            let scaffold = ScrollArea::new().child(Padding::symmetric(0.0, 24.0).child(body));

            let mut tree = WidgetTree::new();
            let root = tree.add_boxed(Box::new(scaffold));
            tree.layout(bastyde::prelude::SizeProposal::exact(width, 4000.0));
            first_of_type(&tree, root, "ColumnFlow")
                .map(|id| tree.bounds(id).height)
                .expect("the scaffold contains a ColumnFlow")
        };

        let wide = flow_height(1200.0);
        let narrow = flow_height(360.0);
        assert!(
            wide > 0.0 && narrow > 0.0,
            "scaffold laid out (wide {wide}, narrow {narrow})"
        );
        // Wide packs five panels into three columns (≈ two rows); narrow stacks all five.
        // A generous margin, not a pixel assertion — it only has to have reflowed at all.
        assert!(
            wide * 1.5 < narrow,
            "wide dashboard ({wide}px) must reflow far shorter than narrow ({narrow}px) — \
             the viewport's bounded width did not reach the ColumnFlow"
        );
    }

    /// A one-child composing widget that forwards its layout to that child — the layout
    /// shape of `pace_pane`'s `PaceBody`. Proves the empty/planner swap does not, unlike a
    /// `Switcher`, drop the parent's bounded width proposal.
    struct Forwarder {
        child: Option<Box<dyn Widget>>,
        root: Option<WidgetId>,
    }
    impl std::fmt::Debug for Forwarder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Forwarder").finish()
        }
    }
    impl Widget for Forwarder {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let id = ctx.add_boxed(self.child.take().expect("Forwarder built once"));
            self.root = Some(id);
            vec![id]
        }
        fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
            self.root
                .and_then(|id| ctx.child_size(id, proposal))
                .map(LayoutResponse::from)
                .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
        }
    }

    /// A writing item exposes the fields the matrix allows: a Scene gets main +
    /// synopsis prose; a ChapterScene adds a title; a BookBegin gets two titles
    /// plus the book's synopsis (symmetric with the Folder/Book container).
    #[test]
    fn tab_for_loads_allowed_fields() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let mk = |sr: BinderItemSubRole| {
            tab_for(
                &ctx,
                1,
                &Item,
                &sr,
                &[],
                Signal::new(700.0),
                Signal::new(true),
                test_typography(),
                crate::view_models::EditorViewMemory::detached(false),
                &AppIds::new(),
            )
        };
        let scene = mk(Scene);
        assert!(scene.main().is_some() && scene.synopsis().is_some() && scene.title().is_none());

        let cs = mk(ChapterScene);
        assert!(cs.main().is_some() && cs.synopsis().is_some() && cs.title().is_some());

        let bb = mk(BookBegin);
        assert!(
            bb.title().is_some()
                && bb.subtitle().is_some()
                && bb.synopsis().is_some()
                && bb.main().is_none()
        );

        let end = mk(BookEnd);
        assert!(end.main().is_none() && end.synopsis().is_none() && end.title().is_none());
    }

    /// The widest right edge anywhere under `id`, with the widget that owns it — the
    /// name matters, because "something overflows" is useless without "what".
    fn widest_right(tree: &WidgetTree, id: WidgetId) -> (f32, String) {
        let mut worst = (
            tree.bounds(id).right(),
            tree.widget_type_name(id).unwrap_or("?").to_string(),
        );
        for child in tree.children(id) {
            let got = widest_right(tree, child);
            if got.0 > worst.0 {
                worst = got;
            }
        }
        worst
    }

    /// **The writing column must shrink with the window, not overflow it.**
    ///
    /// The column grows up to the width set in Settings, but below that it has to
    /// follow the window down. It did not: `centered()` laid its cap out inside an
    /// `HStack` + `Spacer`, and an alignment parent measures its child with an
    /// **unbounded** proposal — so `MaxSize` always reported its full cap and never
    /// shrank. Narrow the window under the column width and the tab overflowed to the
    /// right by the difference, for the entire height of the document.
    ///
    /// That overhang is what froze the app: the inspector painted hazard stripes over
    /// an overflow strip as tall as the whole scene, and a single 45° band across it
    /// became a 7573x7563 path — a 229 MB rasterization too big for the path atlas to
    /// store, so it was rebuilt and thrown away on every frame at 100% CPU. The
    /// renderer and the overlay are both hardened now, but the layout is where the
    /// absurd geometry was born, so it is pinned here too.
    #[test]
    fn a_window_narrower_than_the_column_does_not_overflow() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;

        let ctx = Rc::new(AppContext::new());
        // The settings column cap is far wider than the window we lay out in.
        const CAP: f32 = 700.0;
        const BOX_W: f32 = 300.0;

        for (role, sub_role) in [
            (Item, Scene),
            (Item, ChapterScene),
            (Item, Note),
            (Folder, Book),
            (Folder, ChapterScene),
        ] {
            let tab = tab_for(
                &ctx,
                1,
                &role,
                &sub_role,
                &[],
                Signal::new(CAP),
                Signal::new(true),
                test_typography(),
                crate::view_models::EditorViewMemory::detached(false),
                &AppIds::new(),
            );
            // A real text backend is required for a faithful narrow-window
            // check: `single_line` labels (the segment bar) only report a shrink
            // weight — so an over-constrained stack truncates them with an
            // ellipsis instead of overflowing — through the real text-layout
            // path. The no-backend 8px/char fallback returns a rigid size, so
            // the bar's labels would spill exactly as they never do in the live
            // app (which always has a backend).
            let mut tree = WidgetTree::new().with_text_backend(std::rc::Rc::new(
                std::cell::RefCell::new(bastyde::canvas::MockTextBackend::new()),
            ));
            let id = tree.add_boxed(tab_pane(&tab));
            tree.layout(bastyde::prelude::SizeProposal::exact(BOX_W, 700.0));

            let (right, who) = widest_right(&tree, id);
            assert!(
                right <= BOX_W + 0.5,
                "{role:?}/{sub_role:?}: `{who}` reaches x={right} in a {BOX_W}px window \
                 (column cap {CAP}) — the writing column must shrink with the window, \
                 not overhang it by {:.0}px for the full height of the scene",
                right - BOX_W
            );
        }
    }

    /// …but it stops shrinking at a floor, so the column never collapses to nothing.
    #[test]
    fn the_writing_column_shrinks_no_further_than_its_floor() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;

        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(tab_pane(&tab));
        // Absurdly narrow — far below the floor.
        tree.layout(bastyde::prelude::SizeProposal::exact(20.0, 400.0));

        let (right, _who) = widest_right(&tree, id);
        assert!(
            right >= shared::editor::MIN_COLUMN_WIDTH - 0.5,
            "the column collapsed to {right}px; it must bottom out at \
             {}px rather than shrink to nothing",
            shared::editor::MIN_COLUMN_WIDTH
        );
    }

    /// Every **on-screen** prose editor's rect, in tree order.
    ///
    /// Two filters, both load-bearing. Dormant subtrees are skipped: a `Switcher`
    /// keeps the page it switched away from mounted, and a parked node still
    /// reports the bounds it had when it was last laid out — so a naive walk sees
    /// the Top *and* Side layouts at once and counts four editors where the writer
    /// sees two. And recursion stops at an editor, because each one nests its own
    /// padded viewport under the same type name and would otherwise be counted
    /// twice.
    fn editor_rects(tree: &WidgetTree, id: WidgetId, out: &mut Vec<bastyde::prelude::Rect>) {
        if !tree.is_active(id) {
            return;
        }
        let bounds = tree.bounds(id);
        let is_editor = tree
            .widget_type_name(id)
            .is_some_and(|t| t.contains("RichTextEditor"));
        if is_editor && bounds.height > 0.0 && bounds.width > 0.0 {
            out.push(bounds);
            return;
        }
        for c in tree.children(id) {
            editor_rects(tree, c, out);
        }
    }

    /// Mount a Side-placed scene at `width` and report the laid-out editor rects.
    ///
    /// Settling takes more than one pass on purpose. The breakpoint is decided
    /// during layout and published to a signal the `Switcher` consumes as a
    /// *deferred* rebuild, and showing a splitter pane is an animated tween — so a
    /// single `layout()` reads a half-open divider, which is exactly the trap a
    /// test written by analogy to the instant `VisibleWhen` hide would fall into.
    fn side_scene_editors(width: f32, collapsed: bool) -> Vec<bastyde::prelude::Rect> {
        use BinderItemRole::*;
        use BinderItemSubRole::*;

        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.synopsis_placement
            .set(crate::view_models::SynopsisPlacement::Side);
        if collapsed {
            tab.side_splitter
                .set_collapsed(crate::tabs::shared::editor::SYNOPSIS_PANE, true);
        }

        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(tab_pane(&tab));
        for _ in 0..4 {
            tree.layout(bastyde::prelude::SizeProposal::exact(width, 700.0));
        }
        tree.tick_animations(std::time::Duration::from_millis(400));
        tree.layout(bastyde::prelude::SizeProposal::exact(width, 700.0));

        let mut rects = Vec::new();
        editor_rects(&tree, id, &mut rects);
        rects
    }

    /// Side placement puts the synopsis in its own column **beside** the prose —
    /// the whole point of the setting.
    #[test]
    fn side_placement_seats_the_synopsis_left_of_the_manuscript() {
        let rects = side_scene_editors(1200.0, false);
        assert_eq!(
            rects.len(),
            2,
            "a Side scene lays out both its synopsis and its prose, got {rects:?}"
        );
        let (synopsis, prose) = (rects[0], rects[1]);
        assert!(
            synopsis.x < prose.x,
            "the synopsis column must sit to the left of the manuscript \
             (synopsis at x={}, prose at x={})",
            synopsis.x,
            prose.x
        );
        assert!(
            synopsis.x + synopsis.width <= prose.x + 1.0,
            "the two columns must not overlap — the divider separates them"
        );
    }

    /// …but only where there is room for it. The editor can be split down to a
    /// 320px pane, and a 280px synopsis taken out of that leaves a prose column
    /// nobody can write in. Below the threshold the tab renders Top instead —
    /// stacked, not side by side — rather than honouring the setting into
    /// uselessness.
    #[test]
    fn a_pane_too_narrow_for_two_columns_falls_back_to_the_top_layout() {
        let rects = side_scene_editors(460.0, false);
        assert_eq!(
            rects.len(),
            2,
            "the synopsis is still shown — only its placement changed, got {rects:?}"
        );
        let (synopsis, prose) = (rects[0], rects[1]);
        assert!(
            (synopsis.x - prose.x).abs() < 60.0,
            "the fallback must stack the two in one column, not seat them side by \
             side (synopsis at x={}, prose at x={})",
            synopsis.x,
            prose.x
        );
        assert!(
            synopsis.y < prose.y,
            "stacked means the synopsis is above the prose"
        );
    }

    /// Count the laid-out splitter dividers under `id`.
    fn gutters(tree: &WidgetTree, id: WidgetId, out: &mut usize) {
        if !tree.is_active(id) {
            return;
        }
        if tree
            .widget_type_name(id)
            .is_some_and(|t| t.contains("SplitterHandleBody"))
            && tree.bounds(id).width > 0.0
        {
            *out += 1;
        }
        for c in tree.children(id) {
            gutters(tree, c, out);
        }
    }

    /// The Side pane's own fold-away control gives the width back to the
    /// manuscript — **and leaves a way to get it back.**
    ///
    /// The first cut of this folded with `set_pane_visible(false)`, which removes
    /// the pane *and its divider*. The only control that could restore the column
    /// lived inside the column, so folding it was a one-way door: the writer was
    /// left with no affordance at all and no way back short of the Settings window.
    /// Folding is a **collapse** instead, which is the framework's other state
    /// precisely because it keeps the divider — draggable, double-clickable and
    /// keyboard-reachable — as the way back.
    #[test]
    fn folding_the_side_column_leaves_the_divider_as_the_way_back() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;

        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.synopsis_placement
            .set(crate::view_models::SynopsisPlacement::Side);

        let mut tree = WidgetTree::new();
        let root = tree.add_boxed(tab_pane(&tab));
        // Collapsing a pane is an animated tween, and the breakpoint decision is
        // published from layout for the `Switcher` to pick up on the *next* pass —
        // so settling means interleaving layouts and clock ticks until both have
        // finished, not one of each.
        let settle = |tree: &mut WidgetTree| {
            for _ in 0..8 {
                tree.layout(bastyde::prelude::SizeProposal::exact(1200.0, 700.0));
                tree.tick_animations(std::time::Duration::from_millis(120));
            }
            tree.layout(bastyde::prelude::SizeProposal::exact(1200.0, 700.0));
        };
        let survey = |tree: &WidgetTree| {
            let (mut rects, mut n) = (Vec::new(), 0);
            editor_rects(tree, root, &mut rects);
            gutters(tree, root, &mut n);
            (rects, n)
        };

        settle(&mut tree);
        let (open, open_gutters) = survey(&tree);
        assert_eq!(open.len(), 2, "open: synopsis + manuscript");
        assert_eq!(open_gutters, 1, "one divider between the two columns");

        tab.side_splitter
            .set_collapsed(crate::tabs::shared::editor::SYNOPSIS_PANE, true);
        settle(&mut tree);
        let (folded, folded_gutters) = survey(&tree);
        assert_eq!(
            folded.len(),
            1,
            "folded: the synopsis column is gone, got {folded:?}"
        );
        assert_eq!(
            folded_gutters, 1,
            "the divider must SURVIVE the fold — it is the only thing left to \
             pull the column back with"
        );
        // The writing column is width-capped by design, so it does not get *wider*
        // — it re-centres in the space the synopsis gave back.
        assert!(
            folded[0].x < open[1].x,
            "the manuscript reclaims the space and re-centres (x {} -> {})",
            open[1].x,
            folded[0].x
        );

        tab.side_splitter
            .set_collapsed(crate::tabs::shared::editor::SYNOPSIS_PANE, false);
        settle(&mut tree);
        let (restored, _) = survey(&tree);
        assert_eq!(
            restored.len(),
            2,
            "and dragging it back open restores the column, got {restored:?}"
        );
    }

    /// Dragging the divider persists the new width; the app's own show/hide
    /// bookkeeping does not.
    ///
    /// The divider's `version` signal is one coarse notification bumped by every
    /// mutation there is — including the two the show/hide dance makes on every
    /// toggle. Writing back on every bump would let hiding the synopsis overwrite
    /// the width the writer chose with whatever the model happened to hold
    /// mid-dance, so the next tab would open at a width nobody picked.
    #[test]
    fn only_a_real_drag_persists_the_synopsis_column_width() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;

        let ctx = Rc::new(AppContext::new());
        let show = Signal::new(true);
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            show.clone(),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.synopsis_placement
            .set(crate::view_models::SynopsisPlacement::Side);
        let stored = tab.synopsis_side_width.clone();

        let mut tree = WidgetTree::new();
        tree.add_boxed(tab_pane(&tab));
        let settle = |tree: &mut WidgetTree| {
            for _ in 0..4 {
                tree.layout(bastyde::prelude::SizeProposal::exact(1200.0, 700.0));
            }
            tree.tick_animations(std::time::Duration::from_millis(400));
        };
        settle(&mut tree);
        assert_eq!(
            stored.get(),
            crate::SYNOPSIS_SIDE_WIDTH_DEFAULT,
            "merely showing the column is not the writer choosing a width"
        );

        // What a drag does to the model.
        tab.side_splitter.set_stored_size(0, 350.0);
        assert_eq!(stored.get(), 350.0, "a drag is the one thing that persists");

        // …and the dance that runs when the synopsis is folded away must leave it.
        show.set(false);
        settle(&mut tree);
        assert_eq!(
            stored.get(),
            350.0,
            "hiding the column must not overwrite the width the writer chose"
        );

        show.set(true);
        settle(&mut tree);
        assert_eq!(stored.get(), 350.0, "nor must showing it again");
    }

    /// A synopsis spell session is awake exactly while some mounted view is
    /// showing it — no more, and no less.
    ///
    /// This used to be one global flag pushed into every open document, which was
    /// the right shape only while "is the synopsis visible?" had a single
    /// app-wide answer. It no longer does: a tab can fold its Side synopsis away
    /// on its own, and distraction-free mode has its own toggle. So the question
    /// became a count, and the thing that must not happen is a **leak** — a tab
    /// closed while its synopsis was up, pinning a session awake for the rest of
    /// the session with nothing on screen to justify it.
    #[test]
    fn a_synopsis_session_sleeps_unless_a_mounted_view_is_showing_it() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;

        let ctx = Rc::new(AppContext::new());
        let show = Signal::new(true);
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            show.clone(),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        let doc = tab.open_doc.clone();
        let layout = |tree: &mut WidgetTree| {
            tree.layout(bastyde::prelude::SizeProposal::exact(900.0, 700.0))
        };

        assert_eq!(
            doc.synopsis_viewers(),
            0,
            "a document nobody has mounted yet is not being shown"
        );

        let mut tree = WidgetTree::new();
        tree.add_boxed(tab_pane(&tab));
        layout(&mut tree);
        assert_eq!(doc.synopsis_viewers(), 1, "the mounted pane shows it");

        show.set(false);
        layout(&mut tree);
        assert_eq!(doc.synopsis_viewers(), 0, "hidden — the session may sleep");

        show.set(true);
        layout(&mut tree);
        assert_eq!(doc.synopsis_viewers(), 1, "shown again — awake again");

        drop(tree);
        assert_eq!(
            doc.synopsis_viewers(),
            0,
            "a pane torn down while the synopsis was showing must release its \
             claim — otherwise closing a tab pins the session awake forever"
        );
    }

    /// The same document open twice (both panes of a split, or a pane plus the
    /// distraction-free surface) is shown once as far as its spell session is
    /// concerned — and closing *one* of the two must not put it to sleep while the
    /// other is still displaying it.
    #[test]
    fn two_views_of_one_document_count_as_one_awake_synopsis() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;

        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        let doc = tab.open_doc.clone();

        let mut both = WidgetTree::new();
        both.add_boxed(tab_pane(&tab));
        both.add_boxed(tab_pane(&tab));
        both.layout(bastyde::prelude::SizeProposal::exact(900.0, 700.0));
        assert_eq!(doc.synopsis_viewers(), 2, "two mounted views, two claims");

        drop(both);
        assert_eq!(doc.synopsis_viewers(), 0);
    }

    /// Every tab is born with a Side-synopsis divider, and it must be **weightless**
    /// until something shows it.
    ///
    /// A `Splitter` folds every pane's `min_size` into its own intrinsic minimum
    /// whether or not that pane is visible. So a synopsis pane parked at a real
    /// minimum would put a floor under the width of *every* tab — including the Top
    /// ones that never draw it, and including a secondary editor pane that is only
    /// 320px wide to begin with. Hidden means `min_size` 0; the width is raised only
    /// while the pane is actually on screen.
    #[test]
    fn a_new_tabs_side_divider_starts_hidden_and_weightless() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;

        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        let m = &tab.side_splitter;

        assert_eq!(m.pane_count(), 2, "synopsis | manuscript");
        assert!(
            !m.is_pane_visible(0),
            "the synopsis pane must start hidden — placement is Top by default"
        );
        assert_eq!(
            m.min_size(0),
            0.0,
            "a hidden synopsis pane must contribute no minimum, or it widens every tab"
        );
        assert!(
            m.min_size(1) > 0.0,
            "the manuscript pane keeps a real floor so Side can never squeeze it away"
        );
        assert_eq!(
            m.stored_size(0),
            crate::SYNOPSIS_SIDE_WIDTH_DEFAULT,
            "hidden, but seeded at the persisted width so showing it opens where the \
             writer left it"
        );
        assert!(
            m.is_collapsible(0),
            "the synopsis pane must be collapsible, or folding it away would leave \
             no divider to pull it back with"
        );
        assert!(!m.is_collapsed(0), "a fresh tab has not been folded away");
    }

    /// Hiding the synopsis pane must give its space back to the prose.
    ///
    /// It used to be a `Switcher` (which parks a zero-size page); it is now
    /// `VisibleWhen`, which sends the node *dormant* — out of layout entirely. This
    /// pins the property that actually matters to the writer: with the pane off, the
    /// tab is shorter by the height of the synopsis, rather than leaving a gap where
    /// it used to be.
    #[test]
    fn hiding_the_synopsis_pane_reclaims_its_height() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;

        let ctx = Rc::new(AppContext::new());
        let height_with = |show: bool| {
            let tab = tab_for(
                &ctx,
                1,
                &Item,
                &Scene,
                &[],
                Signal::new(700.0),
                Signal::new(show),
                test_typography(),
                crate::view_models::EditorViewMemory::detached(false),
                &AppIds::new(),
            );
            let mut tree = WidgetTree::new();
            let id = tree.add_boxed(tab_pane(&tab));
            tree.layout(bastyde::prelude::SizeProposal::exact(900.0, 700.0));

            // Count the editors that actually take up space. The tab always fills its
            // 700px box (the outer ScrollArea fills), so the tab's own height says
            // nothing; what changes is whether the synopsis editor is laid out at all.
            fn editors_with_height(tree: &WidgetTree, id: WidgetId, n: &mut usize) {
                let is_editor = tree
                    .widget_type_name(id)
                    .is_some_and(|t| t.contains("RichTextEditor"));
                if is_editor && tree.bounds(id).height > 0.0 {
                    *n += 1;
                }
                for c in tree.children(id) {
                    editors_with_height(tree, c, n);
                }
            }
            let mut n = 0;
            editors_with_height(&tree, id, &mut n);
            n
        };

        let shown = height_with(true);
        let hidden = height_with(false);
        assert!(
            hidden > 0,
            "the prose editor must still be laid out with the synopsis hidden"
        );
        assert!(
            hidden < shown,
            "{shown} editor nodes take space with the synopsis shown and {hidden} with it \
             hidden — hiding it must send the pane DORMANT and drop it out of layout, not \
             park an empty row where it used to be"
        );
    }

    /// The editor half of "one title, two homes": typing a name into a container's own
    /// page and committing it (blur / Enter) must reach **both** `BinderItem.title` —
    /// what the outline tree and the tab caption show — and the title `Content` row that
    /// compiles into the manuscript.
    ///
    /// It used to write only the content row, which is why renaming a chapter in its
    /// editor left the tree and the tab showing the old name.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn committing_a_title_reaches_both_of_its_homes() {
        use frontend::commands::{binder_commands, binder_item_commands, content_commands};
        use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
        use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};

        let ctx = Rc::new(AppContext::new());
        let work = frontend::commands::work_commands::create_orphan_work(
            &ctx,
            None,
            &CreateWorkDto::default(),
        )
        .unwrap();
        let binder = binder_commands::create_binder(
            &ctx,
            None,
            &CreateBinderDto {
                name: "B".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            0,
        )
        .unwrap();
        let item = binder_item_commands::create_binder_item(
            &ctx,
            None,
            &CreateBinderItemDto {
                title: "Old name".into(),
                role: BinderItemRole::Folder,
                sub_role: BinderItemSubRole::ChapterScene,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            binder.id,
            -1,
        )
        .unwrap();

        let tab = tab_for(
            &ctx,
            item.id,
            &BinderItemRole::Folder,
            &BinderItemSubRole::ChapterScene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        let title = tab.title().expect("a chapter folder has a title field");
        assert_eq!(
            title.value.get(),
            "Old name",
            "the field is seeded from the entity — the name the writer sees in the tree"
        );

        title.value.set("The Long Road".to_string());
        tab.commit_names_fn()(); // what blur / Enter fires

        // Home 1: the entity field the tree and the tab caption read.
        let dto = binder_item_commands::get_binder_item(&ctx, &item.id)
            .unwrap()
            .unwrap();
        assert_eq!(dto.title, "The Long Road");

        // Home 2: the content row the manuscript compiles.
        let content_ids = binder_item_commands::get_binder_item_relationship(
            &ctx,
            &item.id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap();
        let chapter_title = content_commands::get_content_multi(&ctx, &content_ids)
            .unwrap()
            .into_iter()
            .flatten()
            .find(|c| c.role == ContentRole::ChapterTitle)
            .map(|c| c.data);
        assert_eq!(chapter_title.as_deref(), Some("The Long Road"));
    }

    /// Scene, ChapterScene and Note are no longer collapsed into one prose kind:
    /// `tab_for` tags each, and `main_typography` resolves the right bundle
    /// (Scene → Scene font, Note → Notes font).
    #[test]
    fn prose_kind_distinguishes_scene_from_note() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let mk = |sr: BinderItemSubRole| {
            tab_for(
                &ctx,
                1,
                &Item,
                &sr,
                &[],
                Signal::new(700.0),
                Signal::new(true),
                test_typography(),
                crate::view_models::EditorViewMemory::detached(false),
                &AppIds::new(),
            )
        };
        assert_eq!(mk(Scene).kind(), Some(ProseKind::Scene));
        assert_eq!(mk(ChapterScene).kind(), Some(ProseKind::Scene));
        assert_eq!(mk(Note).kind(), Some(ProseKind::Note));
        assert_eq!(mk(Part).kind(), Option::None); // Item/Part → heading, no prose kind

        // `main_typography` picks the bundle by kind.
        assert_eq!(mk(Scene).main_typography().font_family.get(), "Literata");
        assert_eq!(mk(Note).main_typography().font_family.get(), "Inter");
    }

    /// Distraction-free mode is a *window-mode* axis, not a content-type one:
    /// `main_typography` must return the distraction-free bundle for BOTH a
    /// Scene and a Note tab the instant this tab's window enters the mode, and
    /// must fall back to the normal Scene/Notes split the instant it leaves —
    /// the branch `ContentTab::new`'s `distraction_free` flag exists for.
    /// `main_column_width` rides the very same flag, so it is pinned here too:
    /// this is the regression test for the column-width slider that used to be
    /// a complete no-op (nothing downstream of `ContentTab::column_width` ever
    /// read `distraction_free_width`).
    #[test]
    fn distraction_free_overrides_prose_kind_typography_while_active() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let mut typo = test_typography();
        typo.distraction_free.font_family = Signal::new("Distraction Serif".to_string());
        let distraction_free = Signal::new(false);
        let mk = |sr: BinderItemSubRole,
                  typo: &EditorTypographySet,
                  df: &Signal<bool>,
                  df_width: &Signal<f32>| {
            let open_doc = Rc::new(OpenDoc::build(&ctx, 1, &Item, &sr, &[], Signal::new(0)));
            ContentTab::new(
                ctx.clone(),
                AppIds::new(),
                OpenDocsStore::new(ctx.clone()),
                open_doc,
                Signal::new(700.0),
                Signal::new(true),
                Signal::new(SynopsisPlacement::default()),
                Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
                typo.clone(),
                crate::view_models::TypewriterSettings::off(),
                crate::view_models::CaretHighlightSettings::off(),
                crate::view_models::EditorViewMemory::detached(false),
                crate::view_models::CorkboardDefaults::detached(),
                crate::view_models::TreeExpansionViewModel::new(
                    ctx.clone(),
                    AppIds::new(),
                    crate::models::TreeExpansionService::in_memory_default(),
                ),
                df.clone(),
                df_width.clone(),
                crate::view_models::FormatViewModel::detached(),
            )
        };
        let distraction_free_width = Signal::new(620.0);

        // Inactive: the normal Scene/Note split still applies, and the column
        // stays at the normal width.
        let scene = mk(Scene, &typo, &distraction_free, &distraction_free_width);
        let note = mk(Note, &typo, &distraction_free, &distraction_free_width);
        assert_eq!(scene.main_typography().font_family.get(), "Literata");
        assert_eq!(note.main_typography().font_family.get(), "Inter");
        assert_eq!(scene.main_column_width().get(), 700.0);
        assert_eq!(note.main_column_width().get(), 700.0);

        // Active: both read the distraction-free bundle — and the
        // distraction-free width — instead.
        distraction_free.set(true);
        assert_eq!(
            scene.main_typography().font_family.get(),
            "Distraction Serif"
        );
        assert_eq!(
            note.main_typography().font_family.get(),
            "Distraction Serif"
        );
        assert_eq!(scene.main_column_width().get(), 620.0);
        assert_eq!(note.main_column_width().get(), 620.0);

        // The distraction-free width is itself live, exactly like every other
        // Settings-backed signal — dragging the slider must reach an already
        // built tab, not just a freshly opened one.
        distraction_free_width.set(500.0);
        assert_eq!(scene.main_column_width().get(), 500.0);

        // Deactivated again: back to the normal split/width — the flag is
        // live, not baked in at construction time.
        distraction_free.set(false);
        assert_eq!(scene.main_typography().font_family.get(), "Literata");
        assert_eq!(scene.main_column_width().get(), 700.0);
    }

    /// The manuscript stream honours the tab's **main** typography and column,
    /// not the Scene bundle and the normal column it used to hardcode.
    ///
    /// This is what stops the distraction-free surface's whole point from ending
    /// at a container's first segment: switch a Full Chapter into the surface and
    /// the prose must be typeset like the mode's single-scene view. The row
    /// *headers* deliberately stay on the tab's normal column — page furniture,
    /// not manuscript.
    #[cfg(feature = "mocks")]
    #[test]
    fn the_manuscript_stream_follows_the_tabs_main_typography_and_column() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let mut typo = test_typography();
        typo.distraction_free.font_family = Signal::new("Distraction Serif".to_string());

        // A container tab built the way the distraction-free surface builds one:
        // the flag pinned true for the life of the tab.
        let open_doc = Rc::new(OpenDoc::build(
            &ctx,
            101,
            &Folder,
            &ChapterScene,
            &[],
            Signal::new(0),
        ));
        let tab = ContentTab::new(
            ctx.clone(),
            AppIds::new(),
            OpenDocsStore::new(ctx.clone()),
            open_doc,
            Signal::new(700.0),
            Signal::new(true),
            Signal::new(SynopsisPlacement::default()),
            Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
            typo,
            crate::view_models::TypewriterSettings::off(),
            crate::view_models::CaretHighlightSettings::off(),
            crate::view_models::EditorViewMemory::detached(false),
            crate::view_models::CorkboardDefaults::detached(),
            crate::view_models::TreeExpansionViewModel::new(
                ctx.clone(),
                AppIds::new(),
                crate::models::TreeExpansionService::in_memory_default(),
            ),
            Signal::new(true),
            Signal::new(420.0),
            crate::view_models::FormatViewModel::detached(),
        );

        // Segment 1 is the manuscript stream (own page / manuscript / Full
        // Synopsis / Corkboard / Overview).
        tab.segment.set(1);
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(bastyde::prelude::SizeProposal::exact(1400.0, 900.0));

        // The stream's prose columns are capped at the distraction-free width.
        // The header columns keep the normal 700 — both must be present, which is
        // what proves the two widths did not collapse back into one.
        let caps = all_max_size_widths(&tree, id);
        assert!(
            caps.iter().any(|w| (*w - 420.0).abs() < 0.5),
            "no stream column at the distraction-free width — the stream is still \
             hardcoded to the tab's normal column. Widths seen: {caps:?}"
        );
        assert!(
            caps.iter().any(|w| (*w - 700.0).abs() < 0.5),
            "no stream column at the normal width — the row headers should stay on \
             the tab's own column. Widths seen: {caps:?}"
        );
    }

    /// Every laid-out `MaxSize` cap in the subtree — the writing columns.
    fn all_max_size_widths(tree: &WidgetTree, root: WidgetId) -> Vec<f32> {
        let mut out = Vec::new();
        fn walk(tree: &WidgetTree, id: WidgetId, out: &mut Vec<f32>) {
            if tree
                .widget_type_name(id)
                .is_some_and(|n| n.ends_with("MaxSize"))
            {
                let b = tree.bounds(id);
                if b.width > 0.0 {
                    out.push(b.width);
                }
            }
            for c in tree.children(id) {
                walk(tree, c, out);
            }
        }
        walk(tree, root, &mut out);
        out
    }

    // ── per-document view state (caret + page scroll) ─────────────────────

    /// Build a Scene tab whose prose is `paragraphs` lines long, mount its pane,
    /// and return both. A long document is what gives the page `ScrollArea` a
    /// non-zero maximum, without which a restored scroll offset is clamped
    /// straight back to 0 and the test would prove nothing.
    fn mounted_scene(paragraphs: usize) -> (ContentTab, WidgetTree) {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            Signal::new(false),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        let text = "The rain kept on.\n".repeat(paragraphs);
        tab.main()
            .expect("a Scene has a main prose field")
            .doc
            .cursor_at(0)
            .insert_text(&text)
            .unwrap();
        // A real text backend, for the same reason
        // `a_window_narrower_than_the_column_does_not_overflow` needs one: the
        // no-backend fallback does not give the prose a faithful height, so the
        // page would have nothing to scroll and every offset would clamp to 0.
        let mut tree = WidgetTree::new().with_text_backend(std::rc::Rc::new(
            std::cell::RefCell::new(bastyde::canvas::MockTextBackend::new()),
        ));
        tree.add_boxed(tab_pane(&tab));
        tree.layout(bastyde::prelude::SizeProposal::exact(900.0, 300.0));
        (tab, tree)
    }

    /// A mounted prose pane must publish **both** ports. They come from two
    /// different widgets — the caret from the editor handle, the scroll from the
    /// page `ScrollArea` — because the editors run with their own scroll bars
    /// suppressed, so `RichTextEditor::scroll_y()` on a prose column is
    /// permanently 0. If either port went unattached, `capture_view_state` would
    /// quietly return the seed forever and nothing would ever be persisted.
    #[test]
    fn a_mounted_prose_pane_publishes_both_view_state_ports() {
        let (tab, _tree) = mounted_scene(4);
        let ports = tab.view_state_ports();
        assert!(ports.editor().is_some(), "the editor handle port is empty");
        assert!(
            ports.max_scroll().is_some(),
            "the page scroll port is empty — writing_page_scroll did not publish it"
        );
    }

    /// The round trip the whole feature rests on: a seeded caret reaches the
    /// editor as it builds, and the caret the writer actually leaves behind is
    /// what comes back out — not the seed.
    #[test]
    fn a_seeded_caret_reaches_the_editor_and_the_live_one_comes_back() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            Signal::new(false),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.main()
            .unwrap()
            .doc
            .cursor_at(0)
            .insert_text("The rain kept on for three days.")
            .unwrap();
        // Seeded BEFORE the pane exists — the workspace-restore and
        // distraction-free-entry path.
        tab.seed_view_state(crate::view_models::ViewState {
            caret: 9,
            scroll: 0.0,
        });

        let mut tree = crate::test_support::tree_with_events(&ctx);
        tree.add_boxed(tab_pane(&tab));
        tree.layout(bastyde::prelude::SizeProposal::exact(900.0, 600.0));

        let handle = tab.view_state_ports().editor().unwrap();
        assert_eq!(
            handle.cursor_position(),
            9,
            "the editor did not open at the seeded caret"
        );

        // The writer moves. Capture must report that, not the seed.
        handle.select_range(20, 20);
        assert_eq!(tab.capture_view_state().caret, 20);
    }

    /// A caret past the end of the document is clamped rather than landing
    /// somewhere arbitrary. Reachable whenever a document was edited in another
    /// window (or another split pane) between capture and restore — `New Window`
    /// on the same project makes that an ordinary thing to do, not a corner case.
    #[test]
    fn a_stale_caret_past_the_end_is_clamped_to_the_document() {
        let (tab, _tree) = mounted_scene(2);
        let len = tab.main().unwrap().doc.character_count();
        tab.apply_view_state(crate::view_models::ViewState {
            caret: len + 5_000,
            scroll: 0.0,
        });
        assert_eq!(
            tab.view_state_ports().editor().unwrap().cursor_position(),
            len
        );
    }

    /// A rebuild that has nothing to do with the writer — a Promote, a
    /// settings-driven relayout — mints a fresh editor over the same document.
    /// It must not throw the caret back to wherever the tab was first opened,
    /// which is what a naive "seed on every build" would do.
    #[test]
    fn rebuilding_a_pane_carries_the_caret_over() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let ctx = Rc::new(AppContext::new());
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            Signal::new(false),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.main()
            .unwrap()
            .doc
            .cursor_at(0)
            .insert_text("The rain kept on for three days.")
            .unwrap();

        let mut tree = crate::test_support::tree_with_events(&ctx);
        tree.add_boxed(tab_pane(&tab));
        tree.layout(bastyde::prelude::SizeProposal::exact(900.0, 600.0));
        tab.view_state_ports()
            .editor()
            .unwrap()
            .select_range(17, 17);

        // Rebuild, exactly as the tab factory would.
        let mut tree2 = crate::test_support::tree_with_events(&ctx);
        tree2.add_boxed(tab_pane(&tab));
        tree2.layout(bastyde::prelude::SizeProposal::exact(900.0, 600.0));

        assert_eq!(
            tab.view_state_ports().editor().unwrap().cursor_position(),
            17,
            "the rebuild reset the caret instead of carrying it over"
        );
    }

    /// `is_stale` distinguishes "flushed and quiet" from "flushed, then edited
    /// again" — the exact gap `OpenDoc::dirty`/`doc.is_modified()` leaves, since
    /// both are booleans a flush clears unconditionally with no memory of which
    /// edit they were cleared against.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn is_stale_distinguishes_a_later_edit_from_flushed_and_quiet() {
        use frontend::commands::binder_commands;
        use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};

        let ctx = Rc::new(AppContext::new());
        let work = frontend::commands::work_commands::create_orphan_work(
            &ctx,
            None,
            &CreateWorkDto::default(),
        )
        .unwrap();
        let binder = binder_commands::create_binder(
            &ctx,
            None,
            &CreateBinderDto {
                name: "B".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            0,
        )
        .unwrap();
        let item = frontend::commands::binder_item_commands::create_binder_item(
            &ctx,
            None,
            &CreateBinderItemDto {
                title: "Scene".into(),
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Scene,
                activated: true,
                ..Default::default()
            },
            binder.id,
            -1,
        )
        .unwrap();

        let field = prose_field(&ctx, item.id, ContentRole::SceneText, None);
        assert!(!field.is_stale(), "a freshly loaded field is never stale");

        // A first edit — `TextCursor::insert_text`, the same primitive live typing
        // uses, so this queues a real `ContentsChanged` and bumps `content_revision`
        // (unlike `set_djot_sync`/`set_plain_text`, which only reset the document
        // and never touch either `content_revision` or `is_modified`).
        field.doc.cursor_at(0).insert_text("First draft.").unwrap();
        assert!(field.is_stale(), "an edit must show stale");
        field.flush(None).expect("flush a real item's field");
        assert!(!field.is_stale(), "flush must clear staleness");
        assert!(
            !field.doc.is_modified(),
            "flush must also clear the coarse flag"
        );

        // A SECOND edit after that flush: `content_revision` moves again, so
        // `is_stale` still catches it — exactly the case a boolean
        // `dirty`/`is_modified` flag cannot distinguish from "flushed and quiet"
        // the instant after any flush clears it.
        field
            .doc
            .cursor_at(0)
            .insert_text("Second draft, written after the save. ")
            .unwrap();
        assert!(
            field.is_stale(),
            "an edit made after the last flush must be detected, even though \
             right after any flush it looks identical to 'flushed and quiet' from \
             `is_modified()`'s point of view"
        );
    }
}
