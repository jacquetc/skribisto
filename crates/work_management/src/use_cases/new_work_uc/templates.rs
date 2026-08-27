// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Pure, store-independent builders for the `NewWork` project templates.
//!
//! Each template is expressed as a flat, ordered list of binders → items →
//! content, exactly the shape `NewWorkUseCase::execute` then persists. Book
//! structure is the flat `sub_role` state machine (Book/Chapter/Scene/BookEnd);
//! folders are UI-only containment expressed via `indent`. Every content row is
//! filtered through [`skribisto_model::content_allowed`], so a template can never
//! describe an item that violates the writing-model constraint matrix.
//!
//! All human-visible strings come from the UI-supplied `labels` (the backend
//! can't do i18n) — see [`TemplateLabels`].

use crate::NewWorkTemplate;
use common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use skribisto_model::content_allowed;

/// Translated words the UI passes in `NewWorkDto.labels`, in this fixed order:
/// `[Manuscript, Notes, Research, Notebook, <retired>, Scene, Note, Front matter, Back
/// matter, Characters, Places]`. Missing or empty entries fall back to the English word,
/// so a short list can't panic.
///
/// **Append only, and slots are never reused.** The list is positional, so inserting in
/// the middle silently retitles everything after it. The two paratext folder names, then
/// the two notes-folder names, were added at the end for that reason.
///
/// **Slot 4 is retired.** It held the word "Chapter", which this file used to write into
/// every generated chapter's title. It no longer does — see [`manuscript_binder`] — and
/// the slot stays occupied rather than being reclaimed, so a `labels` list assembled by
/// an older caller still lines up. The UI sends a placeholder there.
///
/// These are binder and folder names — organising scaffolding, in the *interface*
/// language. Structural titles are not here and never will be: a chapter is named by the
/// manuscript's own numbering, in the language it is *written* in, and the paratext item
/// titles come from the preset file verbatim, in the language of the tradition they
/// belong to.
pub struct TemplateLabels {
    pub manuscript: String,
    pub notes: String,
    pub research: String,
    pub notebook: String,
    pub scene: String,
    pub note: String,
    pub front_matter: String,
    pub back_matter: String,
    pub characters: String,
    pub places: String,
}

impl TemplateLabels {
    /// The documented order of `NewWorkDto.labels`.
    pub fn from_list(labels: &[String]) -> Self {
        let at = |i: usize, fallback: &str| {
            labels
                .get(i)
                .filter(|s| !s.is_empty())
                .cloned()
                .unwrap_or_else(|| fallback.to_string())
        };
        TemplateLabels {
            manuscript: at(0, "Manuscript"),
            notes: at(1, "Notes"),
            research: at(2, "Research"),
            notebook: at(3, "Notebook"),
            // 4: the retired "Chapter" slot — see the struct's own doc.
            scene: at(5, "Scene"),
            note: at(6, "Note"),
            front_matter: at(7, "Front matter"),
            back_matter: at(8, "Back matter"),
            characters: at(9, "Characters"),
            places: at(10, "Places"),
        }
    }
}

/// One item in a template tree (flat list + `indent`, like the stored binder).
pub struct TemplateItem {
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    pub title: String,
    pub indent: i64,
    pub is_exportable: bool,
    /// Already filtered to model-valid `(role, sub_role, content_role)` triples.
    pub contents: Vec<(ContentRole, String)>,
}

/// One binder in a template tree.
pub struct TemplateBinder {
    pub name: String,
    pub activated: bool,
    pub items: Vec<TemplateItem>,
}

/// Build an item, dropping any content role the constraint matrix forbids for
/// its `(role, sub_role)` — the template can only ever produce valid items.
fn item(
    role: BinderItemRole,
    sub_role: BinderItemSubRole,
    title: impl Into<String>,
    indent: i64,
    is_exportable: bool,
    contents: Vec<(ContentRole, String)>,
) -> TemplateItem {
    let contents = contents
        .into_iter()
        .filter(|(cr, _)| content_allowed(&role, &sub_role, cr))
        .collect();
    TemplateItem {
        role,
        sub_role,
        title: title.into(),
        indent,
        is_exportable,
        contents,
    }
}

/// How much book a novel-family template lays down.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ManuscriptShape {
    /// `Folder/Part` layers between the book and its chapters. `0` is a book whose
    /// chapters hang directly off it — every template but `NovelInParts`.
    parts: usize,
    /// Chapters **per part**, or the whole book's chapters when `parts` is `0`.
    chapters: usize,
}

/// The Manuscript binder shared by every novel-family template: a Book wrapping
/// `shape.parts` parts (or none) of `shape.chapters` chapters each, then a BookEnd.
///
/// `chapter_scene` picks the per-chapter encoding: `false` (default) gives the
/// classic layout — a `Folder/ChapterScene` holding one empty `Item/Scene`; `true`
/// gives a single flat `Item/ChapterScene` per chapter (opens the chapter *and*
/// carries its own prose), which the user writes straight into. The two differ only
/// on the `role` axis (extent by containment vs. by marker) and compile to the same
/// book — see the writing model in `skribisto_model`.
///
/// # Parts and chapters are created **untitled**, and that is the point
///
/// This template used to write `"Chapter 1".."Chapter N"` into each chapter's `title`
/// *and* into a `ChapterTitle` content row, because for a long time that was the only
/// place a writer could see the number. It is not any more, and the generated title had
/// become dead weight that three separate mechanisms existed to work around:
///
/// * the binder's `StructureNumber` badge sits beside the title, so the row read
///   "1. Chapter 1";
/// * the exporter needs `headings::is_redundant_number_title` to keep
///   `NumberAndTitle` from printing "Chapter 3 — Chapter 3";
/// * **Document ▸ Tidy chapter titles…** exists to clear, after the fact, exactly what
///   this function wrote.
///
/// Worse, it wrote the word in the **interface** locale while both of those mechanisms
/// judge redundancy in the language the row is *written* in. A French-interface writer
/// starting an English project got `"Chapitre 1"` titles that neither surface would
/// recognise, and the export printed "Chapter 1 — Chapitre 1".
///
/// An untitled structural row is a fully-supported, first-class state: the binder names
/// it through `models::numbering::fallback_label_for` and the exporter through
/// `HeadingScheme::NumberAndTitle`'s number-only arm — both in the manuscript's own
/// language, both from the same numbering pass. So the template writes no title at all
/// and lets the book say what chapter this is.
fn manuscript_binder(
    title: &str,
    l: &TemplateLabels,
    shape: ManuscriptShape,
    chapter_scene: bool,
) -> TemplateBinder {
    use BinderItemRole::{Folder, Item};
    use BinderItemSubRole::{Book, BookEnd, ChapterScene, Part, Scene};
    use ContentRole::{BookTitle, SceneText, SynopsisText};

    let mut items = Vec::new();
    // Book container (opens the book; carries the title the writer typed).
    items.push(item(
        Folder,
        Book,
        title,
        0,
        true,
        vec![(BookTitle, title.to_string())],
    ));
    // With parts, a chapter sits one level deeper. Book 0 → Part 1 → chapter 2 → scene 3.
    let chapter_indent = if shape.parts > 0 { 2 } else { 1 };
    for _ in 0..shape.parts.max(1) {
        if shape.parts > 0 {
            // Untitled, like the chapters below it and for the same reasons — a part is
            // numbered, and `Part` is one of the levels `numbering::level_of` answers for.
            items.push(item(Folder, Part, String::new(), 1, true, vec![]));
        }
        for _ in 0..shape.chapters {
            if chapter_scene {
                // One flat ChapterScene per chapter — no folder, no child scene.
                items.push(item(
                    Item,
                    ChapterScene,
                    String::new(),
                    chapter_indent,
                    true,
                    vec![(SceneText, String::new()), (SynopsisText, String::new())],
                ));
            } else {
                // Classic: a chapter folder holding one empty Scene. The folder's own
                // prose and synopsis rows are left to be created on first use, exactly
                // as they always have been here.
                items.push(item(
                    Folder,
                    ChapterScene,
                    String::new(),
                    chapter_indent,
                    true,
                    vec![],
                ));
                items.push(item(
                    Item,
                    Scene,
                    format!("{} 1", l.scene),
                    chapter_indent + 1,
                    true,
                    vec![(SceneText, String::new()), (SynopsisText, String::new())],
                ));
            }
        }
    }
    // Closes the book (empty marker). Load-bearing rather than decorative: back matter is
    // appended after it, and it is what puts those rows *outside* the book.
    items.push(item(Item, BookEnd, String::new(), 1, true, vec![]));

    TemplateBinder {
        name: l.manuscript.clone(),
        activated: true,
        items,
    }
}

/// The Notes binder every novel-family template ships: three `Folder/Note` folders, and
/// nothing in them.
///
/// It replaces the two **empty binders** ("Notes" and "Research") this file used to
/// create. A binder is not a `BinderItem`: its row carries no `item_id`, so it opens no
/// tab at all — those two were inert roots holding nothing, and nothing could be done
/// with them but create inside them.
///
/// `Folder/Note` and not `Folder/None`, because the difference is the whole point:
/// `skribisto_model::overview_capable` excludes a plain grouping folder, and the Story
/// bible card grid is a notes folder's segment. Until this existed no shipped template
/// produced a single `Folder/Note` anywhere, so a brand-new project had nowhere for the
/// story bible to appear and nothing for a tag's `creates_in` to point at — while the
/// New Work wizard was already offering to lay down a `character` / `place` palette.
///
/// Out of the export, like every note (see `is_exportable` on the notes folder the
/// Notebook template creates): a story bible is the writer's own workings.
fn notes_binder(l: &TemplateLabels) -> TemplateBinder {
    use BinderItemRole::Folder;
    use BinderItemSubRole::Note;

    let items = [&l.characters, &l.places, &l.research]
        .into_iter()
        .map(|name| item(Folder, Note, name.clone(), 0, false, vec![]))
        .collect();
    TemplateBinder {
        name: l.notes.clone(),
        activated: true,
        items,
    }
}

/// An empty binder — the whole of the `None` template.
fn empty_binder(name: &str) -> TemplateBinder {
    TemplateBinder {
        name: name.to_string(),
        activated: true,
        items: Vec::new(),
    }
}

/// Build the full binder list for `template`. `title` is the project title
/// (used as the book title); `l` supplies the translated labels. `chapter_scene`
/// selects the per-chapter encoding for the novel family (see
/// [`manuscript_binder`]); it is ignored by the non-manuscript templates.
/// The paratext structure a new project opens with: the titles to create before the
/// manuscript and the ones to create after it, already resolved from the chosen preset.
///
/// Verbatim titles — the caller reads them out of the preset file and does not translate
/// them. Empty on both sides means "None", which is a first-class choice and not a
/// degenerate case.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParatextPlan {
    pub front: Vec<String>,
    pub back: Vec<String>,
}

impl ParatextPlan {
    pub fn is_empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }
}

/// A `Folder/Paratext` holding `titles`, at `indent`.
///
/// The folder's name is translated (it is organising scaffolding, and it emits nothing
/// into the export); the items' titles are not (they are the book's own words).
fn paratext_folder(name: &str, titles: &[String], indent: i64) -> Vec<TemplateItem> {
    use BinderItemRole::{Folder, Item};
    use BinderItemSubRole::Paratext;
    use ContentRole::{ParatextText, SynopsisText};

    let mut items = vec![item(Folder, Paratext, name, indent, true, vec![])];
    for title in titles {
        items.push(item(
            Item,
            Paratext,
            title,
            indent + 1,
            true,
            vec![(ParatextText, String::new()), (SynopsisText, String::new())],
        ));
    }
    items
}

/// Test-only convenience: [`build_template_with_paratexts`] with no paratexts. Production
/// code (`new_work_uc.rs`) always has a `ParatextPlan` in hand and calls that directly.
#[cfg(test)]
pub fn build_template(
    template: NewWorkTemplate,
    title: &str,
    l: &TemplateLabels,
    chapter_scene: bool,
) -> Vec<TemplateBinder> {
    build_template_with_paratexts(template, title, l, chapter_scene, &ParatextPlan::default())
}

/// `build_template` plus a paratext structure.
///
/// Two axes, kept orthogonal on purpose: the manuscript template says how much book, the
/// plan says which tradition. Merged into one enum they would multiply — "Novel, twenty
/// chapters, roman français" — and never stop.
pub fn build_template_with_paratexts(
    template: NewWorkTemplate,
    title: &str,
    l: &TemplateLabels,
    chapter_scene: bool,
    paratexts: &ParatextPlan,
) -> Vec<TemplateBinder> {
    let mut binders = build_template_inner(template, title, l, chapter_scene);
    if paratexts.is_empty() {
        return binders;
    }
    // Around the **book**, and only if there is one. A notebook template's first binder
    // is a Notes binder with no book in it at all, and dropping a "Half title" and a
    // "Table of contents" into someone's notebook would be nonsense — front and back
    // matter are the furniture of a book, so with no book there is nothing to furnish.
    //
    // Keyed on the presence of a book row rather than on the template enum: the question
    // is what was actually built, and a future template that grows a book should get its
    // paratexts without anyone remembering to add it to a list.
    let Some(manuscript) = binders.iter_mut().find(|b| {
        b.items
            .iter()
            .any(|i| i.sub_role == BinderItemSubRole::Book)
    }) else {
        return binders;
    };
    // Front matter before the book row, back matter after everything. Both at the book's
    // own indent, so they are siblings of it rather than inside it — a preface is not part
    // of the book's body, which is the whole point of the thing.
    {
        let mut front = paratext_folder(&l.front_matter, &paratexts.front, 0);
        if !paratexts.front.is_empty() {
            front.append(&mut manuscript.items);
            manuscript.items = front;
        }
        if !paratexts.back.is_empty() {
            manuscript
                .items
                .extend(paratext_folder(&l.back_matter, &paratexts.back, 0));
        }
    }
    binders
}

fn build_template_inner(
    template: NewWorkTemplate,
    title: &str,
    l: &TemplateLabels,
    chapter_scene: bool,
) -> Vec<TemplateBinder> {
    match template {
        // Empty project: a single, empty Manuscript binder.
        NewWorkTemplate::None => vec![empty_binder(&l.manuscript)],

        // Novel family: Manuscript (Book + chapters, in parts or not) + a Notes binder
        // holding the three story-bible folders.
        NewWorkTemplate::EmptyNovel
        | NewWorkTemplate::LightNovel
        | NewWorkTemplate::Novel
        | NewWorkTemplate::NovelInParts => {
            let shape = match template {
                NewWorkTemplate::EmptyNovel => ManuscriptShape {
                    parts: 0,
                    chapters: 1,
                },
                NewWorkTemplate::LightNovel => ManuscriptShape {
                    parts: 0,
                    chapters: 15,
                },
                NewWorkTemplate::Novel => ManuscriptShape {
                    parts: 0,
                    chapters: 20,
                },
                // Three parts of eight — the three-act shape, at the same order of
                // magnitude as `Novel` so the two read as variants of one book rather
                // than as different sizes of project.
                NewWorkTemplate::NovelInParts => ManuscriptShape {
                    parts: 3,
                    chapters: 8,
                },
                _ => unreachable!(),
            };
            vec![
                manuscript_binder(title, l, shape, chapter_scene),
                notes_binder(l),
            ]
        }

        // Notebook: one binder holding a notes folder with a starter Note.
        NewWorkTemplate::NoteBook => {
            use BinderItemRole::{Folder, Item};
            use BinderItemSubRole::Note;
            use ContentRole::{NoteText, SynopsisText};

            let items = vec![
                // `Folder/Note`, not `Folder/None`: a notebook is a subtree worth
                // tabulating and a place a story bible can live, and only the notes
                // folder is `overview_capable` or carries the Story bible segment.
                item(Folder, Note, l.notes.clone(), 0, false, vec![]),
                // Out of the export, like every note: a notebook is the writer's own
                // workings, and the switch is one click away in the Inspector for the
                // rare note that is genuinely meant to be printed.
                item(
                    Item,
                    Note,
                    format!("{} 1", l.note),
                    1,
                    false,
                    vec![(NoteText, String::new()), (SynopsisText, String::new())],
                ),
            ];
            vec![TemplateBinder {
                name: l.notebook.clone(),
                activated: true,
                items,
            }]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels() -> TemplateLabels {
        TemplateLabels::from_list(&[
            "Manuscrit".into(),
            "Notes".into(),
            "Recherche".into(),
            "Carnet".into(),
            // Slot 4 is retired; the UI sends a placeholder and nothing reads it.
            String::new(),
            "Scène".into(),
            "Note".into(),
            "Pages liminaires".into(),
            "Annexes".into(),
            "Personnages".into(),
            "Lieux".into(),
        ])
    }

    /// Every content row a template describes must be model-valid.
    fn assert_all_valid(binders: &[TemplateBinder]) {
        for b in binders {
            for it in &b.items {
                for (cr, _) in &it.contents {
                    assert!(
                        content_allowed(&it.role, &it.sub_role, cr),
                        "invalid content {cr:?} on {:?}/{:?}",
                        it.role,
                        it.sub_role
                    );
                }
            }
        }
    }

    /// Every template that builds a book, for the sweeps that must hold across all of them.
    const BOOK_TEMPLATES: [NewWorkTemplate; 4] = [
        NewWorkTemplate::EmptyNovel,
        NewWorkTemplate::LightNovel,
        NewWorkTemplate::Novel,
        NewWorkTemplate::NovelInParts,
    ];

    #[test]
    fn none_is_one_empty_binder() {
        let b = build_template(NewWorkTemplate::None, "My Book", &labels(), false);
        assert_eq!(b.len(), 1);
        assert_eq!(b[0].name, "Manuscrit");
        assert!(b[0].items.is_empty());
    }

    #[test]
    fn novel_family_has_a_manuscript_and_a_notes_binder() {
        for t in BOOK_TEMPLATES {
            let b = build_template(t.clone(), "My Book", &labels(), false);
            assert_eq!(b.len(), 2, "{t:?} should have 2 binders");
            assert_eq!(b[0].name, "Manuscrit");
            assert_eq!(b[1].name, "Notes");
            assert_all_valid(&b);
        }
    }

    /// The Notes binder is what makes a new project a place a story bible can live: the
    /// two empty binders it replaced could not even be opened, and `Folder/None` would
    /// carry neither an Overview nor the Story bible grid.
    #[test]
    fn the_notes_binder_holds_three_note_folders() {
        for t in BOOK_TEMPLATES {
            let b = build_template(t.clone(), "My Book", &labels(), false);
            let notes = &b[1];
            let names: Vec<&str> = notes.items.iter().map(|i| i.title.as_str()).collect();
            assert_eq!(names, vec!["Personnages", "Lieux", "Recherche"], "{t:?}");
            for it in &notes.items {
                assert_eq!(it.role, BinderItemRole::Folder, "{t:?}");
                assert_eq!(it.sub_role, BinderItemSubRole::Note, "{t:?}");
                assert!(skribisto_model::overview_capable(&it.role, &it.sub_role));
                assert_eq!(it.indent, 0);
                // A story bible is the writer's own workings, not part of the book.
                assert!(!it.is_exportable, "{t:?}");
            }
        }
    }

    #[test]
    fn chapter_counts_match_template() {
        let count_chapters = |t| {
            build_template(t, "T", &labels(), false)[0]
                .items
                .iter()
                .filter(|i| i.sub_role == BinderItemSubRole::ChapterScene)
                .count()
        };
        assert_eq!(count_chapters(NewWorkTemplate::EmptyNovel), 1);
        assert_eq!(count_chapters(NewWorkTemplate::LightNovel), 15);
        assert_eq!(count_chapters(NewWorkTemplate::Novel), 20);
        // Three parts of eight.
        assert_eq!(count_chapters(NewWorkTemplate::NovelInParts), 24);
    }

    #[test]
    fn manuscript_has_book_scenes_and_bookend() {
        let b = build_template(NewWorkTemplate::Novel, "My Book", &labels(), false);
        let m = &b[0].items;
        // Opens with a Book folder carrying the title.
        assert_eq!(m[0].role, BinderItemRole::Folder);
        assert_eq!(m[0].sub_role, BinderItemSubRole::Book);
        assert_eq!(m[0].contents[0], (ContentRole::BookTitle, "My Book".into()));
        // One Scene per chapter.
        let scenes = m
            .iter()
            .filter(|i| i.sub_role == BinderItemSubRole::Scene)
            .count();
        assert_eq!(scenes, 20);
        // Closes with an empty BookEnd.
        let last = m.last().unwrap();
        assert_eq!(last.sub_role, BinderItemSubRole::BookEnd);
        assert!(last.contents.is_empty());
        assert_all_valid(&b);
    }

    /// **The whole point of the chapter-title change.** A generated "Chapter 7" was
    /// written in the *interface* locale while both the exporter's redundancy guard and
    /// Document ▸ Tidy chapter titles… judge it in the language the row is *written* in,
    /// so a mismatched pair produced a title neither could recognise — and the binder
    /// badge rendered "7. Chapter 7" regardless. No structural row carries a title, and
    /// none carries a title content row either.
    #[test]
    fn no_structural_row_is_born_with_a_title() {
        for t in BOOK_TEMPLATES {
            for chapter_scene in [false, true] {
                let b = build_template(t.clone(), "My Book", &labels(), chapter_scene);
                for it in &b[0].items {
                    let level = skribisto_model::numbering::level_of(&it.sub_role);
                    // The Book is titled — it carries the name the writer typed, which
                    // is not generated and is not an ordinal.
                    if level.is_none() || it.sub_role == BinderItemSubRole::Book {
                        continue;
                    }
                    assert!(
                        it.title.is_empty(),
                        "{t:?}/{chapter_scene}: {:?} was born titled {:?}",
                        it.sub_role,
                        it.title
                    );
                    for (role, _) in &it.contents {
                        assert!(
                            !matches!(role, ContentRole::ChapterTitle | ContentRole::PartTitle),
                            "{t:?}/{chapter_scene}: {:?} carries a generated {role:?}",
                            it.sub_role
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn notebook_has_notes_folder_and_note() {
        let b = build_template(NewWorkTemplate::NoteBook, "T", &labels(), false);
        assert_eq!(b.len(), 1);
        assert_eq!(b[0].name, "Carnet");
        // A notes folder, not a plain grouping folder — only the former gets an Overview
        // and the Story bible grid.
        assert_eq!(b[0].items[0].role, BinderItemRole::Folder);
        assert_eq!(b[0].items[0].sub_role, BinderItemSubRole::Note);
        assert!(skribisto_model::overview_capable(
            &b[0].items[0].role,
            &b[0].items[0].sub_role
        ));
        assert_eq!(b[0].items[1].sub_role, BinderItemSubRole::Note);
        assert_all_valid(&b);
    }

    #[test]
    fn from_list_falls_back_when_short() {
        let l = TemplateLabels::from_list(&[]);
        assert_eq!(l.manuscript, "Manuscript");
        assert_eq!(l.note, "Note");
        assert_eq!(l.characters, "Characters");
        assert_eq!(l.places, "Places");
        // No panic building with fallbacks.
        assert_all_valid(&build_template(NewWorkTemplate::Novel, "T", &l, false));
    }

    /// The parts template is the same book with one more level: parts hold the chapters,
    /// and every chapter sits one indent deeper than it would without them.
    #[test]
    fn parts_wrap_the_chapters_they_hold() {
        let b = build_template(NewWorkTemplate::NovelInParts, "My Book", &labels(), false);
        let m = &b[0].items;

        let parts: Vec<&TemplateItem> = m
            .iter()
            .filter(|i| i.sub_role == BinderItemSubRole::Part)
            .collect();
        assert_eq!(parts.len(), 3);
        for p in &parts {
            assert_eq!(p.role, BinderItemRole::Folder);
            assert_eq!(p.indent, 1, "a part sits directly under the book");
        }
        for c in m
            .iter()
            .filter(|i| i.sub_role == BinderItemSubRole::ChapterScene)
        {
            assert_eq!(c.indent, 2, "a chapter sits inside its part");
        }
        for s in m.iter().filter(|i| i.sub_role == BinderItemSubRole::Scene) {
            assert_eq!(s.indent, 3);
        }
        // Each part opens before the chapters it holds: the first row after a part is a
        // chapter, never another part.
        let firsts: Vec<usize> = m
            .iter()
            .enumerate()
            .filter(|(_, i)| i.sub_role == BinderItemSubRole::Part)
            .map(|(n, _)| n)
            .collect();
        for n in firsts {
            assert_eq!(m[n + 1].sub_role, BinderItemSubRole::ChapterScene);
        }
        assert_all_valid(&b);
    }

    /// The part layer is orthogonal to the chapter encoding — flat chapters still land
    /// inside their part.
    #[test]
    fn parts_hold_flat_chapters_too() {
        let b = build_template(NewWorkTemplate::NovelInParts, "My Book", &labels(), true);
        let m = &b[0].items;
        assert_eq!(
            m.iter()
                .filter(|i| i.sub_role == BinderItemSubRole::Part)
                .count(),
            3
        );
        let chapters: Vec<&TemplateItem> = m
            .iter()
            .filter(|i| i.sub_role == BinderItemSubRole::ChapterScene)
            .collect();
        assert_eq!(chapters.len(), 24);
        for c in chapters {
            assert_eq!(c.role, BinderItemRole::Item);
            assert_eq!(c.indent, 2);
        }
        assert!(m.iter().all(|i| i.sub_role != BinderItemSubRole::Scene));
        assert_all_valid(&b);
    }

    #[test]
    fn chapter_scene_mode_emits_chapterscenes() {
        // With chapter_scene = true, each chapter is a single flat ChapterScene
        // (no Chapter folder, no child Scene), carrying prose + synopsis.
        let b = build_template(NewWorkTemplate::Novel, "My Book", &labels(), true);
        let m = &b[0].items;

        let chapter_scenes = m
            .iter()
            .filter(|i| i.sub_role == BinderItemSubRole::ChapterScene)
            .count();
        assert_eq!(chapter_scenes, 20, "one ChapterScene per chapter");
        // The classic encoding is entirely absent.
        assert!(
            m.iter().all(|i| i.sub_role != BinderItemSubRole::Scene),
            "no plain Scene items in ChapterScene mode"
        );
        assert!(
            m.iter().all(|i| !(i.role == BinderItemRole::Folder
                && i.sub_role == BinderItemSubRole::ChapterScene)),
            "no chapter folders in flat-chapter mode"
        );
        // Every ChapterScene is flat (indent 1) and carries both prose rows.
        for cs in m
            .iter()
            .filter(|i| i.sub_role == BinderItemSubRole::ChapterScene)
        {
            assert_eq!(cs.role, BinderItemRole::Item);
            assert_eq!(cs.indent, 1);
            let roles: Vec<_> = cs.contents.iter().map(|(cr, _)| cr.clone()).collect();
            assert!(roles.contains(&ContentRole::SceneText));
            assert!(roles.contains(&ContentRole::SynopsisText));
        }
        // Still bounded by a BookEnd, and still model-valid throughout.
        assert_eq!(m.last().unwrap().sub_role, BinderItemSubRole::BookEnd);
        assert_all_valid(&b);
    }
}

#[cfg(test)]
mod paratext_tests {
    use super::*;

    fn labels() -> TemplateLabels {
        TemplateLabels::from_list(&[
            "Manuscript".into(),
            "Notes".into(),
            "Research".into(),
            "Notebook".into(),
            // Slot 4 is retired — see `TemplateLabels`.
            String::new(),
            "Scene".into(),
            "Note".into(),
            "Front matter".into(),
            "Back matter".into(),
            "Characters".into(),
            "Places".into(),
        ])
    }

    fn plan() -> ParatextPlan {
        ParatextPlan {
            front: vec!["Faux-titre".into(), "Page de titre".into()],
            back: vec!["Achevé d'imprimer".into()],
        }
    }

    fn manuscript(binders: &[TemplateBinder]) -> &TemplateBinder {
        binders.first().expect("a manuscript binder")
    }

    /// The two axes are independent: every template that builds a **book** accepts every
    /// structure. Merged into one enum they would multiply, which is the whole reason they
    /// are two parameters.
    #[test]
    fn every_book_template_accepts_a_paratext_structure() {
        for template in [
            NewWorkTemplate::EmptyNovel,
            NewWorkTemplate::LightNovel,
            NewWorkTemplate::Novel,
            NewWorkTemplate::NovelInParts,
        ] {
            let binders =
                build_template_with_paratexts(template.clone(), "T", &labels(), false, &plan());
            let items = &manuscript(&binders).items;
            assert!(
                items.iter().any(|i| i.title == "Faux-titre"),
                "{template:?} dropped the front matter"
            );
            assert!(
                items.iter().any(|i| i.title == "Achevé d'imprimer"),
                "{template:?} dropped the back matter"
            );
        }
    }

    /// Titles arrive verbatim — never translated, because they are the book's own words.
    /// The two folders around them are translated, because they are ours.
    #[test]
    fn item_titles_are_verbatim_and_folder_names_are_not() {
        let binders =
            build_template_with_paratexts(NewWorkTemplate::Novel, "T", &labels(), false, &plan());
        let items = &manuscript(&binders).items;

        let folders: Vec<&str> = items
            .iter()
            .filter(|i| {
                i.role == BinderItemRole::Folder && i.sub_role == BinderItemSubRole::Paratext
            })
            .map(|i| i.title.as_str())
            .collect();
        assert_eq!(folders, vec!["Front matter", "Back matter"]);

        let leaves: Vec<&str> = items
            .iter()
            .filter(|i| i.role == BinderItemRole::Item && i.sub_role == BinderItemSubRole::Paratext)
            .map(|i| i.title.as_str())
            .collect();
        assert_eq!(
            leaves,
            vec!["Faux-titre", "Page de titre", "Achevé d'imprimer"]
        );
    }

    /// Front matter is created before the book and back matter after it — placement at
    /// creation, which the writer then owns entirely.
    #[test]
    fn front_comes_before_the_book_and_back_after_it() {
        let binders =
            build_template_with_paratexts(NewWorkTemplate::Novel, "T", &labels(), false, &plan());
        let items = &manuscript(&binders).items;
        let pos = |title: &str| items.iter().position(|i| i.title == title).unwrap();
        let book = items
            .iter()
            .position(|i| i.sub_role == BinderItemSubRole::Book)
            .expect("a book row");

        assert!(pos("Faux-titre") < book, "front matter precedes the book");
        assert!(pos("Achevé d'imprimer") > book, "back matter follows it");
    }

    /// A paratext leaf carries its own content role, never `SceneText` — the one thing
    /// that keeps it out of the word count everywhere.
    #[test]
    fn a_created_paratext_carries_paratext_prose() {
        let binders =
            build_template_with_paratexts(NewWorkTemplate::Novel, "T", &labels(), false, &plan());
        let leaf = manuscript(&binders)
            .items
            .iter()
            .find(|i| i.title == "Faux-titre")
            .unwrap();
        let roles: Vec<&ContentRole> = leaf.contents.iter().map(|(r, _)| r).collect();
        assert!(roles.contains(&&ContentRole::ParatextText));
        assert!(!roles.contains(&&ContentRole::SceneText));
    }

    /// A template with no book gets no front or back matter, whatever the writer picked.
    /// A "Half title" and a "Table of contents" in someone's notebook would be nonsense:
    /// they are the furniture of a book, and there is no book to furnish.
    #[test]
    fn a_bookless_template_gets_no_paratexts() {
        for template in [NewWorkTemplate::None, NewWorkTemplate::NoteBook] {
            let binders =
                build_template_with_paratexts(template.clone(), "T", &labels(), false, &plan());
            for b in &binders {
                assert!(
                    !b.items
                        .iter()
                        .any(|i| i.sub_role == BinderItemSubRole::Paratext),
                    "{template:?} has no book, so it must get no paratexts"
                );
            }
        }
    }

    /// "No structure" is a first-class answer, not a degenerate case: nothing is created
    /// and the manuscript is exactly what it would have been.
    #[test]
    fn no_structure_creates_nothing() {
        let l = labels();
        let with = build_template_with_paratexts(
            NewWorkTemplate::Novel,
            "T",
            &l,
            false,
            &ParatextPlan::default(),
        );
        let without = build_template(NewWorkTemplate::Novel, "T", &l, false);
        assert_eq!(
            manuscript(&with).items.len(),
            manuscript(&without).items.len()
        );
    }
}
