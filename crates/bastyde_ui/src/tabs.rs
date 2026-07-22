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
use bastyde::widgets::{Banner, Button, ButtonVariant, Expand, VStack};
use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::ContentDto;

use crate::app_ids::AppIds;
use crate::models::{OpenDoc, OpenDocsStore};
use crate::singles::{SingleBinderItem, SingleContent};
use crate::view_models::{EditorTypography, EditorTypographySet, PaceViewModel, StreamViewModel};

// One module per valid `(role, sub_role)` combination — each a single visual tab
// (see `skribisto_model::COMBINATIONS`). `tab_pane` dispatches to them.
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
    /// not through documents. Consumed by the Pace pane (built out over M4c/M4d).
    #[allow(dead_code)]
    pace: Option<PaceViewModel>,
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
    /// Selected segment for the folder container's `SegmentedControl` — per-tab
    /// (each pane keeps its own segment).
    pub segment: Signal<usize>,
    pub column_width: Signal<f32>,
    /// Persisted "show synopsis pane above the manuscript" setting (Settings ▸
    /// Manuscript & Fonts). Consumed live by the dual-pane writing editor.
    pub show_synopsis: Signal<bool>,
    /// The three per-editor-type typography bundles (Scene / Synopsis / Notes),
    /// shared live from Settings. Every editor this tab builds reads its bundle
    /// from here, so a preference change fans out to all open tabs at once.
    pub typography: EditorTypographySet,
    /// Per-container-type "last view" memory: seeds this tab's initial [`Self::segment`]
    /// and (for a folder container) is written back when the user switches view, so a
    /// new tab of the same type inherits it. Shared live from Settings.
    pub view_memory: crate::view_models::EditorViewMemory,
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
        typography,
        view_memory,
        crate::view_models::CorkboardDefaults::detached(),
        crate::view_models::TreeExpansionViewModel::new(
            ctx.clone(),
            ids.clone(),
            crate::models::TreeExpansionService::in_memory_default(),
        ),
    )
}

/// Build the widget for a tab (the `TabWidget` factory): dispatch each
/// `(role, sub_role)` to its own visual-tab module. Mirrors
/// `skribisto_model::COMBINATIONS`.
pub fn tab_pane(tab: &ContentTab) -> Box<dyn Widget> {
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
struct Boxed {
    pending: Option<Box<dyn Widget>>,
    child_id: Option<WidgetId>,
}

impl Boxed {
    fn new(child: Box<dyn Widget>) -> Self {
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
        typography: EditorTypographySet,
        view_memory: crate::view_models::EditorViewMemory,
        corkboard_defaults: crate::view_models::CorkboardDefaults,
        tree_expansion: crate::view_models::TreeExpansionViewModel,
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
        // The Corkboard exists for exactly the folder containers a stream does. Built
        // before `stream` consumes `app_ctx`.
        let corkboard =
            crate::models::StreamLevel::for_container(&open_doc.role, &open_doc.sub_role).map(
                |_| {
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
                    )
                },
            );
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
        Self {
            open_doc,
            stream,
            pace,
            corkboard,
            overview,
            ids,
            find,
            synopsis_handle: Rc::new(RefCell::new(None)),
            segment,
            column_width,
            show_synopsis,
            typography,
            view_memory,
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

    /// The `BinderItem` this tab edits.
    pub fn item_id(&self) -> u64 {
        self.open_doc.item_id
    }
    /// The `(role, sub_role)` pair this tab edits — what [`tab_pane`] dispatches on.
    pub fn role(&self) -> &BinderItemRole {
        &self.open_doc.role
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
    #[allow(dead_code)] // consumed by the Pace pane (M4c/M4d)
    pub fn pace(&self) -> Option<&PaceViewModel> {
        self.pace.as_ref()
    }

    /// Persist every changed field back to its `Content` row via the shared
    /// `OpenDoc`. Idempotent — flushing a shared doc twice (once per pane) is a
    /// no-op the second time. Routes through the undo `stack`.
    pub fn flush(&self, stack: Option<u64>) -> anyhow::Result<()> {
        self.open_doc.flush(stack)
    }

    /// The typography bundle for this tab's **main** prose editor: the Notes
    /// bundle for a Note, the Scene bundle otherwise (Scene / ChapterScene, and a
    /// safe fallback for any layout without a `kind`).
    pub fn main_typography(&self) -> &EditorTypography {
        match self.open_doc.kind {
            Some(ProseKind::Note) => &self.typography.notes,
            _ => &self.typography.scene,
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
        for (sub_role, overview_index) in [(ChapterScene, 4), (Part, 4), (Book, 5), (Note, 1)] {
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
        tab.segment.set(5); // the Book's Overview
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
        tab.segment.set(5);
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let root = tree.add_boxed(tab_pane(&tab));
        tree.layout(bastyde::prelude::SizeProposal::exact(1200.0, 700.0));

        fn find(tree: &WidgetTree, id: WidgetId, needle: &str) -> Option<WidgetId> {
            if tree.widget_type_name(id).is_some_and(|t| t.contains(needle)) {
                return Some(id);
            }
            tree.children(id)
                .into_iter()
                .find_map(|c| find(tree, c, needle))
        }
        let table = find(&tree, root, "TreeTableView").expect("the Overview mounts a TreeTableView");

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

        let (uid, col) = vm
            .editing_cell()
            .get()
            .expect("F2 must open an editor; removing the app-side handler must not have \
                     taken the only working one with it");
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
        tab.segment.set(5);

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
    fn only_a_book_has_the_extra_pace_segment() {
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
            chapter + 1,
            "a Book adds Pace, which is why its Overview index is one higher"
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
        assert!(!field.doc.is_modified(), "flush must also clear the coarse flag");

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
