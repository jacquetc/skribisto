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
/// `[Manuscript, Notes, Research, Notebook, Chapter, Scene, Note, Front matter, Back
/// matter]`. Missing or empty entries fall back to the English word, so a short list
/// can't panic.
///
/// **Append only.** The list is positional, so inserting in the middle silently retitles
/// everything after it. The two paratext folder names were added at the end for that
/// reason.
///
/// These are binder names — organising scaffolding, in the *interface* language. The
/// paratext item titles are not here and never will be: they come from the preset file
/// verbatim, in the language of the tradition they belong to, because they are content
/// bound for the book rather than chrome.
pub struct TemplateLabels {
    pub manuscript: String,
    pub notes: String,
    pub research: String,
    pub notebook: String,
    pub chapter: String,
    pub scene: String,
    pub note: String,
    pub front_matter: String,
    pub back_matter: String,
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
            front_matter: at(7, "Front matter"),
            back_matter: at(8, "Back matter"),
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

pub fn build_template(
    template: NewWorkTemplate,
    title: &str,
    l: &TemplateLabels,
    chapter_scene: bool,
) -> Vec<TemplateBinder> {
    build_template_with_paratexts(template, title, l, chapter_scene, &ParatextPlan::default())
}

/// [`build_template`] plus a paratext structure.
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
    let Some(manuscript) = binders
        .iter_mut()
        .find(|b| b.items.iter().any(|i| i.sub_role == BinderItemSubRole::Book))
    else {
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

#[cfg(test)]
mod paratext_tests {
    use super::*;

    fn labels() -> TemplateLabels {
        TemplateLabels::from_list(&[
            "Manuscript".into(),
            "Notes".into(),
            "Research".into(),
            "Notebook".into(),
            "Chapter".into(),
            "Scene".into(),
            "Note".into(),
            "Front matter".into(),
            "Back matter".into(),
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
            .filter(|i| i.role == BinderItemRole::Folder && i.sub_role == BinderItemSubRole::Paratext)
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
        assert_eq!(manuscript(&with).items.len(), manuscript(&without).items.len());
    }
}
