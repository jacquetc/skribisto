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
/// `[Manuscript, Notes, Research, Notebook, Chapter, Scene, Note]`. Missing or
/// empty entries fall back to the English word, so a short list can't panic.
pub struct TemplateLabels {
    pub manuscript: String,
    pub notes: String,
    pub research: String,
    pub notebook: String,
    pub chapter: String,
    pub scene: String,
    pub note: String,
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
            chapter: at(4, "Chapter"),
            scene: at(5, "Scene"),
            note: at(6, "Note"),
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

/// The Manuscript binder shared by every novel-family template: a Book wrapping
/// `chapters` chapters, then a BookEnd.
///
/// `chapter_scene` picks the per-chapter encoding: `false` (default) gives the
/// classic layout — a `Folder/ChapterScene` holding one empty `Item/Scene`; `true`
/// gives a single flat `Item/ChapterScene` per chapter (opens the chapter *and*
/// carries its own prose), which the user writes straight into. The two differ only
/// on the `role` axis (extent by containment vs. by marker) and compile to the same
/// book — see the writing model in `skribisto_model`.
fn manuscript_binder(
    title: &str,
    l: &TemplateLabels,
    chapters: usize,
    chapter_scene: bool,
) -> TemplateBinder {
    use BinderItemRole::{Folder, Item};
    use BinderItemSubRole::{Book, BookEnd, ChapterScene, Scene};
    use ContentRole::{BookTitle, ChapterTitle, SceneText, SynopsisText};

    let mut items = Vec::new();
    // Book container (opens the book; carries the title).
    items.push(item(
        Folder,
        Book,
        title,
        0,
        true,
        vec![(BookTitle, title.to_string())],
    ));
    for n in 1..=chapters {
        let chapter_title = format!("{} {}", l.chapter, n);
        if chapter_scene {
            // One flat ChapterScene per chapter — no folder, no child scene.
            items.push(item(
                Item,
                ChapterScene,
                chapter_title.clone(),
                1,
                true,
                vec![
                    (ChapterTitle, chapter_title),
                    (SceneText, String::new()),
                    (SynopsisText, String::new()),
                ],
            ));
        } else {
            // Classic: a chapter folder holding one empty Scene.
            items.push(item(
                Folder,
                ChapterScene,
                chapter_title.clone(),
                1,
                true,
                vec![(ChapterTitle, chapter_title)],
            ));
            items.push(item(
                Item,
                Scene,
                format!("{} 1", l.scene),
                2,
                true,
                vec![(SceneText, String::new()), (SynopsisText, String::new())],
            ));
        }
    }
    // Closes the book (empty marker).
    items.push(item(Item, BookEnd, String::new(), 1, true, vec![]));

    TemplateBinder {
        name: l.manuscript.clone(),
        activated: true,
        items,
    }
}

/// An empty binder (used for the Notes and Research binders of novel templates).
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
pub fn build_template(
    template: NewWorkTemplate,
    title: &str,
    l: &TemplateLabels,
    chapter_scene: bool,
) -> Vec<TemplateBinder> {
    match template {
        // Empty project: a single, empty Manuscript binder.
        NewWorkTemplate::None => vec![empty_binder(&l.manuscript)],

        // Novel family: Manuscript (Book + N chapters) + empty Notes + Research.
        NewWorkTemplate::EmptyNovel | NewWorkTemplate::LightNovel | NewWorkTemplate::Novel => {
            let chapters = match template {
                NewWorkTemplate::EmptyNovel => 1,
                NewWorkTemplate::LightNovel => 15,
                NewWorkTemplate::Novel => 20,
                _ => unreachable!(),
            };
            vec![
                manuscript_binder(title, l, chapters, chapter_scene),
                empty_binder(&l.notes),
                empty_binder(&l.research),
            ]
        }

        // Notebook: one binder holding a Notes folder with a starter Note.
        NewWorkTemplate::NoteBook => {
            use BinderItemRole::{Folder, Item};
            use BinderItemSubRole::{None as SubNone, Note};
            use ContentRole::{NoteText, SynopsisText};

            let items = vec![
                item(Folder, SubNone, l.notes.clone(), 0, false, vec![]),
                item(
                    Item,
                    Note,
                    format!("{} 1", l.note),
                    1,
                    true,
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
            "Chapitre".into(),
            "Scène".into(),
            "Note".into(),
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

    #[test]
    fn none_is_one_empty_binder() {
        let b = build_template(NewWorkTemplate::None, "My Book", &labels(), false);
        assert_eq!(b.len(), 1);
        assert_eq!(b[0].name, "Manuscrit");
        assert!(b[0].items.is_empty());
    }

    #[test]
    fn novel_family_has_three_binders_named_from_labels() {
        for t in [
            NewWorkTemplate::EmptyNovel,
            NewWorkTemplate::LightNovel,
            NewWorkTemplate::Novel,
        ] {
            let b = build_template(t.clone(), "My Book", &labels(), false);
            assert_eq!(b.len(), 3, "{t:?} should have 3 binders");
            assert_eq!(b[0].name, "Manuscrit");
            assert_eq!(b[1].name, "Notes");
            assert_eq!(b[2].name, "Recherche");
            assert!(b[1].items.is_empty(), "Notes empty");
            assert!(b[2].items.is_empty(), "Research empty");
            assert_all_valid(&b);
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

    #[test]
    fn notebook_has_notes_folder_and_note() {
        let b = build_template(NewWorkTemplate::NoteBook, "T", &labels(), false);
        assert_eq!(b.len(), 1);
        assert_eq!(b[0].name, "Carnet");
        assert_eq!(b[0].items[0].sub_role, BinderItemSubRole::None);
        assert_eq!(b[0].items[1].sub_role, BinderItemSubRole::Note);
        assert_all_valid(&b);
    }

    #[test]
    fn from_list_falls_back_when_short() {
        let l = TemplateLabels::from_list(&[]);
        assert_eq!(l.manuscript, "Manuscript");
        assert_eq!(l.note, "Note");
        // No panic building with fallbacks.
        assert_all_valid(&build_template(NewWorkTemplate::Novel, "T", &l, false));
    }

    #[test]
    fn chapter_scene_mode_emits_chapterscenes() {
        // With chapter_scene = true, each chapter is a single flat ChapterScene
        // (no Chapter folder, no child Scene), carrying title + prose + synopsis.
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
        // Every ChapterScene is flat (indent 1) and carries all three content rows.
        for cs in m
            .iter()
            .filter(|i| i.sub_role == BinderItemSubRole::ChapterScene)
        {
            assert_eq!(cs.role, BinderItemRole::Item);
            assert_eq!(cs.indent, 1);
            let roles: Vec<_> = cs.contents.iter().map(|(cr, _)| cr.clone()).collect();
            assert!(roles.contains(&ContentRole::ChapterTitle));
            assert!(roles.contains(&ContentRole::SceneText));
            assert!(roles.contains(&ContentRole::SynopsisText));
        }
        // Still bounded by a BookEnd, and still model-valid throughout.
        assert_eq!(m.last().unwrap().sub_role, BinderItemSubRole::BookEnd);
        assert_all_valid(&b);
    }
}
