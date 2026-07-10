//! Per-`(role, sub_role)` editor tabs.
//!
//! Not every outline row is a prose editor: a writing item (Scene / Note /
//! ChapterScene) opens the dual-pane editor (`item_scene_tab`); a title-bearing
//! item (Chapter / Part / BookBegin) opens a heading form; a structural folder
//! (Book / Part / Chapter) opens a **container** tab with a `SegmentedControl`
//! (Synopsis now; Corkboard/Overview later); a contentless row (BookEnd / Text)
//! opens a placeholder. One [`ContentTab`] payload type carries them all; the
//! `TabWidget` factory dispatches on its [`TabLayout`].
//!
//! A tab owns **no documents of its own**: its live editing state (main text +
//! synopsis + titles, the dirty flag, the Full Chapter view-model) lives in a
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

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::text_document::TextDocument;
use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::ContentDto;

use crate::app_ids::AppIds;
use crate::models::OpenDoc;
use crate::singles::SingleContent;
use crate::view_models::{ChapterViewModel, EditorTypography, EditorTypographySet};

pub mod folder_book;
pub mod folder_chapter;
pub mod folder_note_tab;
pub mod folder_part;
pub mod heading_tab;
pub mod item_scene_tab;
pub mod no_content_tab;
mod shared;

/// Which view a tab presents (selected from `(role, sub_role)`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TabLayout {
    /// Dual-pane prose: optional title + main text + synopsis (Scene, ChapterScene, Note).
    Prose,
    /// Title (+ optional subtitle / synopsis) form (Item Chapter / Part / BookBegin).
    Heading,
    FolderChapter,
    FolderPart,
    FolderBook,
    /// Synopsis only (Folder None / Note).
    FolderNote,
    /// No editable content (BookEnd, Text).
    NoContent,
}

/// A short, single-line title content (BookTitle/Subtitle, Chapter/PartTitle),
/// edited via a `TextInput` bound to `value`; persisted through its
/// [`SingleContent`] (which owns the row id / `created_at` / create-or-update).
pub struct TitleField {
    pub value: Signal<String>,
    original: RefCell<String>,
    content: SingleContent,
}

/// A rich prose content (SceneText/NoteText/SynopsisText), edited in a
/// `RichTextEditor` over `doc` and persisted as a `.djot` blob via its
/// [`SingleContent`].
pub struct ProseField {
    pub doc: TextDocument,
    content: SingleContent,
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
}

/// Which view a `(role, sub_role)` opens.
pub(crate) fn layout_for(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> TabLayout {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    match (role, sub_role) {
        (Item, Scene) | (Item, ChapterScene) | (Item, Note) => TabLayout::Prose,
        (Item, Chapter) | (Item, Part) | (Item, BookBegin) => TabLayout::Heading,
        (Folder, Chapter) => TabLayout::FolderChapter,
        (Folder, Part) => TabLayout::FolderPart,
        (Folder, Book) => TabLayout::FolderBook,
        (Folder, None) | (Folder, Note) => TabLayout::FolderNote,
        _ => TabLayout::NoContent,
    }
}

/// Which prose kind a `TabLayout::Prose` main-text editor is, so it can pick the
/// Scene vs Note typography bundle. `None` for every non-prose layout (folder
/// tabs, headings) — they never populate `OpenDoc::main`.
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
        (Item, Scene) | (Item, ChapterScene) => Some(ProseKind::Scene),
        (Item, Note) => Some(ProseKind::Note),
        // `None` is shadowed by `BinderItemSubRole::None` under the glob import.
        _ => Option::None,
    }
}

pub(crate) fn prose_field(
    ctx: &Rc<AppContext>,
    item_id: u64,
    role: ContentRole,
    existing: Option<&ContentDto>,
) -> ProseField {
    let data = existing.map(|c| c.data.as_str()).unwrap_or("");
    let doc = TextDocument::new();
    let _ = doc.set_djot(data).and_then(|op| op.wait());
    doc.set_modified(false);
    let content = SingleContent::for_field(ctx.clone(), item_id, role, existing);
    ProseField { doc, content }
}

pub(crate) fn title_field(
    ctx: &Rc<AppContext>,
    item_id: u64,
    role: ContentRole,
    existing: Option<&ContentDto>,
) -> TitleField {
    let data = existing.map(|c| c.data.clone()).unwrap_or_default();
    let content = SingleContent::for_field(ctx.clone(), item_id, role, existing);
    TitleField {
        value: Signal::new(data.clone()),
        original: RefCell::new(data),
        content,
    }
}

/// Build a standalone tab for `item_id` (its own fresh, unshared [`OpenDoc`]).
///
/// The real app opens tabs through `EditorsViewModel` / [`OpenDocsStore`](crate::models::OpenDocsStore),
/// which shares one `OpenDoc` across panes; this convenience is for tests and any
/// call site that wants a self-contained tab.
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
    ids: &AppIds,
) -> ContentTab {
    let open_doc = Rc::new(OpenDoc::build(
        ctx,
        ids,
        item_id,
        role,
        sub_role,
        contents,
        Signal::new(0),
    ));
    ContentTab::new(open_doc, column_width, show_synopsis, typography)
}

/// Build the widget for a tab (the `TabWidget` factory).
pub fn tab_pane(tab: &ContentTab) -> Box<dyn Widget> {
    match tab.layout() {
        TabLayout::Prose => item_scene_tab::render(tab),
        TabLayout::Heading => heading_tab::render(tab),
        TabLayout::FolderChapter => folder_chapter::render(tab),
        TabLayout::FolderPart => folder_part::render(tab),
        TabLayout::FolderBook => folder_book::render(tab),
        TabLayout::FolderNote => folder_note_tab::render(tab),
        TabLayout::NoContent => no_content_tab::render(tab),
    }
}

impl ContentTab {
    /// Wrap a shared `OpenDoc` with this tab's presentation state.
    pub fn new(
        open_doc: Rc<OpenDoc>,
        column_width: Signal<f32>,
        show_synopsis: Signal<bool>,
        typography: EditorTypographySet,
    ) -> Self {
        Self {
            open_doc,
            segment: Signal::new(0),
            column_width,
            show_synopsis,
            typography,
        }
    }

    /// The `BinderItem` this tab edits.
    pub fn item_id(&self) -> u64 {
        self.open_doc.item_id
    }
    /// Which view this tab presents.
    pub fn layout(&self) -> TabLayout {
        self.open_doc.layout
    }
    /// The main prose kind (Scene / Note), or `None` for non-prose layouts.
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
    /// The Full Chapter view-model (only a `FolderChapter` tab has one).
    pub fn chapter(&self) -> Option<&ChapterViewModel> {
        self.open_doc.chapter.as_ref()
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
}

impl TitleField {
    pub(crate) fn flush(&self, stack: Option<u64>) -> anyhow::Result<()> {
        let val = self.value.get();
        if *self.original.borrow() == val {
            return Ok(());
        }
        self.content.set_data(val.clone());
        self.content.save(stack)?;
        *self.original.borrow_mut() = val;
        Ok(())
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
        Ok(())
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
        }
    }

    /// Every valid `(role, sub_role)` must build a tab and lay it out headlessly
    /// without panicking — the per-sub_role dispatch + each layout's widget tree.
    #[test]
    fn every_combination_builds_and_lays_out() {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let combos = [
            (Item, Scene, TabLayout::Prose),
            (Item, ChapterScene, TabLayout::Prose),
            (Item, Note, TabLayout::Prose),
            (Item, Chapter, TabLayout::Heading),
            (Item, Part, TabLayout::Heading),
            (Item, BookBegin, TabLayout::Heading),
            (Item, BookEnd, TabLayout::NoContent),
            (Item, Text, TabLayout::NoContent),
            (Folder, None, TabLayout::FolderNote),
            (Folder, Chapter, TabLayout::FolderChapter),
            (Folder, Part, TabLayout::FolderPart),
            (Folder, Book, TabLayout::FolderBook),
            (Folder, Note, TabLayout::FolderNote),
        ];
        let ctx = Rc::new(AppContext::new());
        for (role, sub_role, expected) in combos {
            let tab = tab_for(
                &ctx,
                1,
                &role,
                &sub_role,
                &[],
                Signal::new(700.0),
                Signal::new(true),
                test_typography(),
                &AppIds::new(),
            );
            assert_eq!(tab.layout(), expected, "{role:?}/{sub_role:?}");
            // Prose tabs carry a kind + a main editor; every other layout has
            // neither.
            if expected == TabLayout::Prose {
                assert!(tab.kind().is_some(), "{role:?}/{sub_role:?} prose needs a kind");
                assert!(tab.main().is_some(), "{role:?}/{sub_role:?} prose needs main");
            } else {
                assert!(tab.kind().is_none(), "{role:?}/{sub_role:?} non-prose has no kind");
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

    /// A writing item exposes the fields the matrix allows: a Scene gets main +
    /// synopsis prose; a ChapterScene adds a title; a BookBegin gets two titles.
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
                &AppIds::new(),
            )
        };
        let scene = mk(Scene);
        assert!(scene.main().is_some() && scene.synopsis().is_some() && scene.title().is_none());

        let cs = mk(ChapterScene);
        assert!(cs.main().is_some() && cs.synopsis().is_some() && cs.title().is_some());

        let bb = mk(BookBegin);
        assert!(bb.title().is_some() && bb.subtitle().is_some() && bb.main().is_none());

        let end = mk(BookEnd);
        assert!(end.main().is_none() && end.synopsis().is_none() && end.title().is_none());
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
                &AppIds::new(),
            )
        };
        assert_eq!(mk(Scene).kind(), Some(ProseKind::Scene));
        assert_eq!(mk(ChapterScene).kind(), Some(ProseKind::Scene));
        assert_eq!(mk(Note).kind(), Some(ProseKind::Note));
        assert_eq!(mk(Chapter).kind(), Option::None); // Item/Chapter → Heading, no prose kind

        // `main_typography` picks the bundle by kind.
        assert_eq!(mk(Scene).main_typography().font_family.get(), "Literata");
        assert_eq!(mk(Note).main_typography().font_family.get(), "Inter");
    }
}
