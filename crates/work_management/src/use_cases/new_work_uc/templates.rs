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
    pub is_printable: bool,
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
    is_printable: bool,
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
        is_printable,
        contents,
    }
}

/// The Manuscript binder shared by every novel-family template: a Book wrapping
/// `chapters` chapters (each a Chapter folder with one Scene), then a BookEnd.
fn manuscript_binder(title: &str, l: &TemplateLabels, chapters: usize) -> TemplateBinder {
    use BinderItemRole::{Folder, Item};
    use BinderItemSubRole::{Book, BookEnd, Chapter, Scene};
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
        items.push(item(
            Folder,
            Chapter,
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
/// (used as the book title); `l` supplies the translated labels.
pub fn build_template(
    template: NewWorkTemplate,
    title: &str,
    l: &TemplateLabels,
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
                manuscript_binder(title, l, chapters),
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
        let b = build_template(NewWorkTemplate::None, "My Book", &labels());
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
            let b = build_template(t.clone(), "My Book", &labels());
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
            build_template(t, "T", &labels())[0]
                .items
                .iter()
                .filter(|i| i.sub_role == BinderItemSubRole::Chapter)
                .count()
        };
        assert_eq!(count_chapters(NewWorkTemplate::EmptyNovel), 1);
        assert_eq!(count_chapters(NewWorkTemplate::LightNovel), 15);
        assert_eq!(count_chapters(NewWorkTemplate::Novel), 20);
    }

    #[test]
    fn manuscript_has_book_scenes_and_bookend() {
        let b = build_template(NewWorkTemplate::Novel, "My Book", &labels());
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
        let b = build_template(NewWorkTemplate::NoteBook, "T", &labels());
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
        assert_all_valid(&build_template(NewWorkTemplate::Novel, "T", &l));
    }
}
