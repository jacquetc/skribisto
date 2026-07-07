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
//! Prose is Djot end-to-end: documents load via `set_djot` and write back via
//! `to_djot` into `Content` rows. Write-back is **role-aware** — a tab only ever
//! owns the content roles `skribisto_model` allows for its `(role, sub_role)`,
//! so non-prose rows can never be corrupted.

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::text_document::TextDocument;
use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::ContentDto;

use crate::singles::SingleContent;

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

/// The dynamic-tab payload: an item's editable fields + its presentation.
pub struct ContentTab {
    pub item_id: u64,
    pub layout: TabLayout,
    pub title: Option<TitleField>,
    pub subtitle: Option<TitleField>,
    pub main: Option<ProseField>,
    pub synopsis: Option<ProseField>,
    /// Selected segment for the folder container's `SegmentedControl`.
    pub segment: Signal<usize>,
    /// `true` once the user has edited a field since the last save. Set
    /// reactively from the editors' `on_change`; cleared by [`flush`](Self::flush).
    /// Drives autosave + the unsaved-state read.
    pub dirty: Signal<bool>,
    /// Shared "an edit happened" counter (set by `EditorsViewModel` so every open
    /// tab bumps the same signal) — drives the debounced autosave timer.
    pub edited: Option<Signal<u64>>,
    pub column_width: Signal<f32>,
    /// Persisted "show synopsis pane above the manuscript" setting (Settings ▸
    /// Manuscript & Fonts). Consumed live by the dual-pane writing editor.
    pub show_synopsis: Signal<bool>,
}

fn layout_for(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> TabLayout {
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

fn prose_field(
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

fn title_field(
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

/// Build the tab for `item_id`, loading each allowed content role from
/// `contents` (the rows fetched at open time) into the right field, each backed
/// by a [`SingleContent`] over `ctx`.
pub fn tab_for(
    ctx: &Rc<AppContext>,
    item_id: u64,
    role: &BinderItemRole,
    sub_role: &BinderItemSubRole,
    contents: &[ContentDto],
    column_width: Signal<f32>,
    show_synopsis: Signal<bool>,
) -> ContentTab {
    let layout = layout_for(role, sub_role);
    let mut tab = ContentTab {
        item_id,
        layout,
        title: None,
        subtitle: None,
        main: None,
        synopsis: None,
        segment: Signal::new(0),
        dirty: Signal::new(false),
        edited: None,
        column_width,
        show_synopsis,
    };
    for cr in skribisto_model::allowed_content(role, sub_role) {
        let existing = contents.iter().find(|c| &c.role == cr);
        match cr {
            ContentRole::SynopsisText => {
                tab.synopsis = Some(prose_field(ctx, item_id, cr.clone(), existing))
            }
            ContentRole::SceneText | ContentRole::NoteText => {
                tab.main = Some(prose_field(ctx, item_id, cr.clone(), existing))
            }
            ContentRole::BookSubtitle => {
                tab.subtitle = Some(title_field(ctx, item_id, cr.clone(), existing))
            }
            ContentRole::BookTitle | ContentRole::ChapterTitle | ContentRole::PartTitle => {
                tab.title = Some(title_field(ctx, item_id, cr.clone(), existing))
            }
        }
    }
    tab
}

/// Build the widget for a tab (the `TabWidget` factory).
pub fn tab_pane(tab: &ContentTab) -> Box<dyn Widget> {
    match tab.layout {
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
    /// Persist every changed field back to its `Content` row (creating the row
    /// if the item didn't have one yet) via each field's [`SingleContent`].
    /// Routes through the undo `stack`.
    pub fn flush(&self, stack: Option<u64>) -> anyhow::Result<()> {
        if let Some(f) = &self.title {
            f.flush(stack)?;
        }
        if let Some(f) = &self.subtitle {
            f.flush(stack)?;
        }
        if let Some(f) = &self.main {
            f.flush(stack)?;
        }
        if let Some(f) = &self.synopsis {
            f.flush(stack)?;
        }
        self.dirty.set(false);
        Ok(())
    }

    /// Wire the editors' `on_change` to set `dirty`. Called by each render fn
    /// when it builds the prose editors (the title fields are diffed at flush
    /// time, so they don't need a change hook).
    pub fn mark_dirty_fn(&self) -> impl Fn() + 'static {
        let dirty = self.dirty.clone();
        let edited = self.edited.clone();
        move || {
            dirty.set(true);
            if let Some(e) = &edited {
                e.set(e.get().wrapping_add(1));
            }
        }
    }
}

impl TitleField {
    fn flush(&self, stack: Option<u64>) -> anyhow::Result<()> {
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
    fn flush(&self, stack: Option<u64>) -> anyhow::Result<()> {
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
            );
            assert_eq!(tab.layout, expected, "{role:?}/{sub_role:?}");
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
        let scene = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
        );
        assert!(scene.main.is_some() && scene.synopsis.is_some() && scene.title.is_none());

        let cs = tab_for(
            &ctx,
            1,
            &Item,
            &ChapterScene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
        );
        assert!(cs.main.is_some() && cs.synopsis.is_some() && cs.title.is_some());

        let bb = tab_for(
            &ctx,
            1,
            &Item,
            &BookBegin,
            &[],
            Signal::new(700.0),
            Signal::new(true),
        );
        assert!(bb.title.is_some() && bb.subtitle.is_some() && bb.main.is_none());

        let end = tab_for(
            &ctx,
            1,
            &Item,
            &BookEnd,
            &[],
            Signal::new(700.0),
            Signal::new(true),
        );
        assert!(end.main.is_none() && end.synopsis.is_none() && end.title.is_none());
    }
}
