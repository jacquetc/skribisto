//! Per-`(role, sub_role)` editor tabs.
//!
//! Every valid `(role, sub_role)` combination has its **own module** — a single
//! visual tab (`item_scene`, `item_chapter_scene`, `folder_book`, …); [`tab_pane`]
//! dispatches each combination to its module. Not every outline row is a prose
//! editor: a writing item (Scene / ChapterScene / Note) opens the dual-pane
//! editor; a title-bearing item (Chapter / Part / BookBegin) opens a heading form;
//! a structural folder (Book / Part / Chapter) opens a **container** tab with a
//! `SegmentedControl` (Synopsis now; Corkboard/Overview later); a contentless row
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

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::text_document::TextDocument;
use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::ContentDto;

use crate::app_ids::AppIds;
use crate::models::{OpenDoc, OpenDocsStore};
use crate::singles::{SingleBinderItem, SingleContent};
use crate::view_models::{EditorTypography, EditorTypographySet, StreamViewModel};

// One module per valid `(role, sub_role)` combination — each a single visual tab
// (see `skribisto_model::COMBINATIONS`). `tab_pane` dispatches to them.
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
    /// The app's entity ids — needed for the undo stack when a name field commits.
    ids: AppIds,
    /// The per-editor find banner (Ctrl+F) — `Some` only when this tab has a main
    /// prose field to search. Persisted on the tab so it survives tab rebuilds
    /// (its `FindSession` + query outlive the widget tree it draws into).
    find: Option<crate::view_models::FindViewModel>,
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
    let _ = doc.set_djot(&content.data().get()).and_then(|op| op.wait());
    doc.set_modified(false);
    ProseField { doc, content }
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
    )
}

/// Build the widget for a tab (the `TabWidget` factory): dispatch each
/// `(role, sub_role)` to its own visual-tab module. Mirrors
/// `skribisto_model::COMBINATIONS`.
pub fn tab_pane(tab: &ContentTab) -> Box<dyn Widget> {
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
    ) -> Self {
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
        Self {
            open_doc,
            stream,
            ids,
            find,
            segment: Signal::new(0),
            column_width,
            show_synopsis,
            typography,
        }
    }

    /// The per-editor find banner's view-model — `Some` only when the tab has a
    /// main prose field (Scene / ChapterScene / Note).
    pub fn find(&self) -> Option<&crate::view_models::FindViewModel> {
        self.find.as_ref()
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
        Ok(())
    }

    /// Re-read the persisted prose into the live document, discarding any live edit
    /// (see [`OpenDoc::reload`](crate::models::OpenDoc::reload)).
    pub(crate) fn reload(&self) {
        self.content.reload();
        let data = self.content.data().get();
        let _ = self.doc.set_djot(&data).and_then(|op| op.wait());
        self.doc.set_modified(false);
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
                &AppIds::new(),
            );
            let mut tree = WidgetTree::new();
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
                is_printable: true,
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
}
