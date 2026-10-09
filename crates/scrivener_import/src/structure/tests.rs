// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One test per rule of the structure mapping (D3.n) and per trap the
//! completeness critique named, over projects built in memory, plus the
//! invariants as properties over arbitrary binders and choices.

use super::*;
use crate::content::{Carries, Heading, ItemFacts, TextFacts};
use crate::model::{
    BinderRow, ConflictScheme, Container, Counts, Generation, IdScheme, ItemKey, ItemKind,
    ItemMeta, ManifestChoice, ManifestOrigin, MemberRef, Payload, Platform, ProjectNote,
    ProjectProperties, ProjectTargets, RecoveredOrigin, RecoveredText, SectionTypeDef,
    SectionTypes, SettingsFiles, SourceFacts, TextRole, Vocabulary, Writer,
};

// ---------------------------------------------------------------- building

struct P {
    project: Project,
    facts: ContentFacts,
}

fn member(path: &str) -> MemberRef {
    MemberRef {
        path: path.to_string(),
        size: 64,
        modified_ns: None,
    }
}

impl P {
    fn new() -> P {
        P {
            project: Project {
                source: SourceFacts {
                    container: Container::Folder,
                    generation: Generation::Format20,
                    writer: Writer {
                        platform: Platform::Mac,
                        version: None,
                        creator: None,
                    },
                    manifest: ManifestOrigin {
                        name: "Novel.scrivx".to_string(),
                        chosen: ManifestChoice::Only,
                    },
                    folder_name: "Novel.scriv".to_string(),
                    folder_title: "Novel".to_string(),
                    identifier: None,
                    modified: None,
                    version_stamp: None,
                    id_scheme: IdScheme::Uuid,
                    template_title: None,
                    created_from_template: None,
                },
                binder: Vec::new(),
                labels: Vocabulary::default(),
                statuses: Vocabulary::default(),
                keywords: Vec::new(),
                custom_fields: Vec::new(),
                section_types: SectionTypes::default(),
                collections: Vec::new(),
                project_links: Vec::new(),
                project_notes: Vec::new(),
                targets: ProjectTargets::default(),
                template_folder: None,
                bookmarks_folder: None,
                favorites: Vec::new(),
                auto_complete: Vec::new(),
                properties: ProjectProperties::default(),
                settings: SettingsFiles::default(),
                compile: Default::default(),
                styles: None,
                writing_history: None,
                recent_writing_day: None,
                custom_icons: Vec::new(),
                recovered: Vec::new(),
                counts: Counts::default(),
            },
            facts: ContentFacts::default(),
        }
    }

    fn add(&mut self, depth: u16, kind: ItemKind, title: &str) -> u32 {
        let i = self.project.binder.len() as u32;
        self.project.binder.push(BinderRow {
            depth,
            key: Some(ItemKey::Number(i + 100)),
            number: Some(i + 100),
            owns_key: true,
            kind,
            title: Some(title.to_string()),
            created: None,
            modified: None,
            meta: ItemMeta {
                include_in_compile: true,
                ..ItemMeta::default()
            },
            keyword_ids: Vec::new(),
            links: Vec::new(),
            target: None,
            script_mode: false,
            default_child_template: None,
            binder_separator: false,
            payload: Payload::default(),
            snapshots: 0,
        });
        self.facts.items.push(ItemFacts::default());
        i
    }

    fn draft(&mut self) -> u32 {
        self.add(0, ItemKind::DraftFolder, "Draft")
    }

    fn folder(&mut self, depth: u16, title: &str) -> u32 {
        self.add(depth, ItemKind::Folder, title)
    }

    /// A text with writing in it.
    fn text(&mut self, depth: u16, title: &str) -> u32 {
        let i = self.add(depth, ItemKind::Text, title);
        self.with_text(i)
    }

    fn with_text(&mut self, i: u32) -> u32 {
        self.facts.items[i as usize].text = TextFacts {
            carries: Carries::Yes,
            words: 1000,
            headings: Vec::new(),
            lead: Carries::Yes,
            letters: Default::default(),
        };
        i
    }

    fn with_notes(&mut self, i: u32) -> u32 {
        self.facts.items[i as usize].notes = Carries::Yes;
        i
    }

    fn with_synopsis(&mut self, i: u32) -> u32 {
        self.facts.items[i as usize].synopsis = Carries::Yes;
        i
    }

    fn with_media(&mut self, i: u32) -> u32 {
        self.project.binder[i as usize].payload.file = Some(member("Files/Data/x/content.png"));
        i
    }

    fn with_card(&mut self, i: u32) -> u32 {
        self.project.binder[i as usize].payload.card_image =
            Some(member("Files/Data/x/card-image.png"));
        i
    }

    fn meta(&mut self, i: u32) -> &mut ItemMeta {
        &mut self.project.binder[i as usize].meta
    }

    fn types(&mut self, names: &[&str]) {
        self.project.section_types.definitions = names
            .iter()
            .enumerate()
            .map(|(k, name)| SectionTypeDef {
                id: format!("T{k}"),
                name: name.to_string(),
            })
            .collect();
    }

    fn plan(&self) -> StructurePlan {
        self.plan_with(&Choices::default())
    }

    fn plan_with(&self, choices: &Choices) -> StructurePlan {
        match plan(&self.project, &self.facts, choices) {
            Ok(plan) => plan,
            Err(e) => panic!("no plan: {e}"),
        }
    }
}

fn title_of(row: &PlannedRow) -> String {
    match &row.title {
        PlannedTitle::Own(t) => t.clone(),
        PlannedTitle::NotesOf(t) => format!("{t} (notes)"),
        PlannedTitle::ProjectTitle(t) => format!("[{t}]"),
        PlannedTitle::ProjectNotes => "<project notes>".to_string(),
        PlannedTitle::Recovered => "<recovered>".to_string(),
    }
}

/// A binder as lines: indent, role/sub-role, title, `~` for a deleted row.
fn show(rows: &[PlannedRow]) -> Vec<String> {
    rows.iter()
        .map(|r| {
            format!(
                "{}{:?}/{:?} {}{}",
                "  ".repeat(r.depth as usize),
                r.role,
                r.sub_role,
                title_of(r),
                if r.activated { "" } else { " ~" }
            )
            .trim_end()
            .to_string()
        })
        .collect()
}

fn keys(plan: &StructurePlan) -> Vec<&'static str> {
    plan.notices.iter().map(StructureNotice::key).collect()
}

fn has(plan: &StructurePlan, notice: &StructureNotice) -> bool {
    plan.notices.contains(notice)
}

/// The row representing `item`.
fn row_of(plan: &StructurePlan, item: u32) -> &PlannedRow {
    let Some(at) = plan.placements[item as usize].item else {
        panic!("item {item} has no row");
    };
    match at.binder {
        BinderKind::Manuscript => &plan.manuscript[at.row as usize],
        BinderKind::Notes => &plan.notes[at.row as usize],
    }
}

fn spell(n: u32) -> String {
    const ONES: [&str; 20] = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    const TENS: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    if n < 20 {
        ONES[n as usize].to_string()
    } else if n.is_multiple_of(10) {
        TENS[(n / 10) as usize].to_string()
    } else {
        format!("{}-{}", TENS[(n / 10) as usize], ONES[(n % 10) as usize])
    }
}

// ---------------------------------------------------------------- D3.1, D3.3 shape

#[test]
fn a_novel_maps_to_a_book_of_parts_chapters_and_scenes_and_a_notes_binder() {
    let mut p = P::new();
    p.draft();
    p.folder(1, "The Coast");
    p.folder(2, "Landfall");
    p.text(3, "Arrival");
    p.text(3, "The Harbour");
    p.folder(2, "Storm");
    p.text(3, "Night Watch");
    p.add(0, ItemKind::ResearchFolder, "Research");
    let map = p.add(1, ItemKind::Image, "Harbour map");
    p.with_media(map);
    p.add(0, ItemKind::TrashFolder, "Trash");
    p.text(1, "Deleted scene");
    let plan = p.plan();
    assert_eq!(
        show(&plan.manuscript),
        [
            "Folder/Book [Novel]",
            "  Folder/Part The Coast",
            "    Folder/ChapterScene Landfall",
            "      Item/Scene Arrival",
            "      Item/Scene The Harbour",
            "    Folder/ChapterScene Storm",
            "      Item/Scene Night Watch",
            "  Item/BookEnd",
        ]
    );
    assert_eq!(
        show(&plan.notes),
        [
            "Folder/Note Research",
            "  Item/Note Harbour map",
            "Item/Note Deleted scene ~",
        ]
    );
    assert_eq!(plan.trash_roots, [2]);
    assert_eq!(plan.chapter_encoding, ChapterEncoding::Folder);
    assert_eq!(
        row_of(&plan, 1).why,
        Why::Shape { height: 2 },
        "no title or section type: the shape decides"
    );
    assert_eq!(plan.shape.books, 1);
    assert_eq!(plan.shape.parts, 1);
    assert_eq!(plan.shape.chapter_folders, 2);
    assert_eq!(plan.shape.scenes, 3);
}

#[test]
fn one_deep_chapter_does_not_turn_its_siblings_into_parts() {
    // A corpus novel: eight folders of scenes, one of them holding a folder.
    let mut p = P::new();
    p.draft();
    for k in 0..8 {
        p.folder(1, &format!("Arc {k}"));
        p.text(2, &format!("Scene {k}a"));
        if k == 3 {
            p.folder(2, "Aside");
            p.text(3, "Aside scene");
        }
    }
    let plan = p.plan();
    assert_eq!(plan.shape.parts, 0);
    assert_eq!(plan.shape.chapter_folders, 8);
    assert_eq!(plan.shape.plain_folders, 1);
    let depth1 = &plan.review.containers[0];
    assert_eq!(depth1.depth, 1);
    assert_eq!(depth1.detected, ContainerRung::Chapter);
    assert!(depth1.flags.contains(&ShapeFlag::MedianDiffersFromMaximum));
}

#[test]
fn a_lone_container_among_texts_stays_plain_so_it_adopts_nothing() {
    // A corpus novel: one text with children among many texts.
    let mut p = P::new();
    p.draft();
    p.text(1, "Opening");
    p.text(1, "Setup");
    let lone = p.text(1, "Kites of Tarn");
    p.text(2, "Inner one");
    p.text(2, "Inner two");
    p.text(1, "Later");
    p.text(1, "Last");
    let plan = p.plan();
    let row = row_of(&plan, lone);
    assert_eq!(
        (row.role.clone(), row.sub_role.clone()),
        (Role::Folder, SubRole::None)
    );
    assert_eq!(row.why, Why::Minority);
    assert!(
        !keys(&plan).contains(&"scrivener-structure-adopted-by-chapter"),
        "{:?}",
        keys(&plan)
    );
    assert!(
        plan.review.containers[0]
            .flags
            .contains(&ShapeFlag::MinorityContainers)
    );
    // Its own prose moved to a first child carrying its title (D3.6).
    assert!(show(&plan.manuscript).contains(&"    Item/Scene Kites of Tarn".to_string()));
    assert!(has(
        &plan,
        &StructureNotice::TextWithChildren { item: lone }
    ));
}

#[test]
fn a_chapter_among_texts_adopts_the_texts_after_it_and_says_so() {
    let mut p = P::new();
    p.draft();
    p.text(1, "Before");
    let chapter = p.folder(1, "Chapter 1");
    p.text(2, "Inside");
    let after = p.text(1, "After");
    let plan = p.plan();
    assert_eq!(row_of(&plan, chapter).sub_role, SubRole::ChapterScene);
    assert!(has(
        &plan,
        &StructureNotice::AdoptedByChapter {
            item: after,
            chapter
        }
    ));
    assert!(
        plan.review.containers[0]
            .flags
            .contains(&ShapeFlag::AdoptsTexts)
    );
}

// ---------------------------------------------------------------- D3.3 signals

#[test]
fn section_types_outrank_titles_and_the_shape() {
    let mut p = P::new();
    p.types(&["Chapter Heading", "Part Title", "Scene", "Section"]);
    p.draft();
    // A book two levels deep, so the shape would call depth 1 a part.
    let a = p.folder(1, "Part 1");
    p.meta(a).section_type = Some("T0".to_string());
    let inner = p.folder(2, "Inner");
    p.text(3, "A");
    let b = p.folder(1, "Second");
    p.folder(2, "Deeper");
    p.text(3, "B");
    let plan = p.plan();
    assert_eq!(row_of(&plan, a).sub_role, SubRole::ChapterScene);
    assert_eq!(row_of(&plan, a).why, Why::SectionType);
    // Its unclaimed sub-folder cannot be a chapter inside a chapter.
    assert_eq!(row_of(&plan, inner).sub_role, SubRole::None);
    assert_eq!(row_of(&plan, b).sub_role, SubRole::Part);
}

#[test]
fn a_section_type_is_handed_down_to_descendants() {
    // A corpus book: a book whose Default Subdocument Type is "Part Heading".
    let mut p = P::new();
    p.types(&["Part Heading"]);
    p.draft();
    let book = p.folder(1, "Volume");
    p.meta(book).section_child_default = Some("t0".to_string());
    let act = p.folder(2, "Chapter 3 - The Lantern Fair");
    p.text(3, "Scene");
    let plan = p.plan();
    assert_eq!(row_of(&plan, act).sub_role, SubRole::Part);
    assert_eq!(
        row_of(&plan, act).why,
        Why::InheritedSectionType { from: book }
    );
}

#[test]
fn titles_outrank_the_templates_level_types() {
    // A corpus novel: level type "Chapter Heading" at every folder depth, a folder
    // titled "Part One - Introduction" holding "Chapter One" and "Chapter Two".
    let mut p = P::new();
    p.types(&["Chapter Heading", "Scene"]);
    p.project.section_types.folders = vec!["T0".to_string()];
    p.project.section_types.files = vec!["T1".to_string()];
    p.draft();
    let part = p.folder(1, "Part One - Introduction");
    let one = p.folder(2, "Chapter One");
    let scene = p.text(3, "Waking");
    let two = p.folder(2, "Chapter Two");
    p.text(3, "Walking");
    let other = p.folder(1, "Interlude folder");
    p.text(2, "Loose");
    let plan = p.plan();
    assert_eq!(row_of(&plan, part).sub_role, SubRole::Part);
    assert_eq!(row_of(&plan, part).why, Why::Title);
    assert_eq!(row_of(&plan, one).sub_role, SubRole::ChapterScene);
    assert_eq!(row_of(&plan, two).sub_role, SubRole::ChapterScene);
    assert_eq!(row_of(&plan, scene).why, Why::LevelType { level: 3 });
    // No title: the level type speaks.
    assert_eq!(row_of(&plan, other).sub_role, SubRole::ChapterScene);
    assert_eq!(row_of(&plan, other).why, Why::LevelType { level: 1 });
}

#[test]
fn a_claim_under_its_own_level_is_overruled_and_reported() {
    let mut p = P::new();
    p.types(&["Chapter"]);
    p.draft();
    p.folder(1, "Chapter 1");
    let nested = p.folder(2, "Inner");
    p.meta(nested).section_type = Some("T0".to_string());
    p.text(3, "Text");
    let plan = p.plan();
    assert_eq!(row_of(&plan, nested).sub_role, SubRole::None);
    assert!(has(
        &plan,
        &StructureNotice::ClaimOverruled {
            item: nested,
            claimed: Level::Chapter
        }
    ));
}

#[test]
fn a_folder_whose_texts_are_chapters_is_transparent() {
    // A corpus book: a "Chapters" folder holding flat chapters.
    let mut p = P::new();
    p.types(&["Chapter"]);
    p.draft();
    let chapters = p.folder(1, "Chapters");
    for k in 1..=4 {
        let t = p.text(2, &format!("The glory {k}"));
        p.meta(t).section_type = Some("T0".to_string());
    }
    let plan = p.plan();
    assert_eq!(row_of(&plan, chapters).sub_role, SubRole::None);
    assert_eq!(row_of(&plan, chapters).why, Why::ClaimsBelow);
    assert_eq!(plan.shape.flat_chapters, 4);
    assert_eq!(plan.chapter_encoding, ChapterEncoding::Flat);
}

#[test]
fn siblings_sharing_a_number_are_not_chapters() {
    let mut p = P::new();
    p.draft();
    p.folder(1, "Episode");
    let a = p.text(2, "Chapter 1 - Scene 1");
    let b = p.text(2, "Chapter 1 - Scene 2");
    let plan = p.plan();
    assert_eq!(row_of(&plan, a).sub_role, SubRole::Scene);
    assert_eq!(row_of(&plan, b).sub_role, SubRole::Scene);
}

#[test]
fn a_sibling_titled_with_the_same_word_claims_the_same_level() {
    // A corpus project: "Chapter: Ironmongery" among "Chapter 3: Tinsmiths".
    let mut p = P::new();
    p.draft();
    p.folder(1, "Chapter 3: Tinsmiths");
    p.text(2, "a");
    p.folder(1, "Chapter 4: Heat");
    p.text(2, "b");
    let odd = p.folder(1, "Chapter: Ironmongery");
    p.text(2, "c");
    let plan = p.plan();
    assert_eq!(row_of(&plan, odd).sub_role, SubRole::ChapterScene);
    assert_eq!(row_of(&plan, odd).why, Why::SiblingTitles);
}

#[test]
fn a_long_run_of_spelled_out_chapters_stays_chapters() {
    // Critique case A: the lexicon used to stop at twenty-nine.
    let mut p = P::new();
    p.draft();
    for n in 1..=35 {
        p.text(1, &format!("Chapter {}", spell(n)));
    }
    let plan = p.plan();
    assert_eq!(plan.shape.flat_chapters, 35);
    assert_eq!(plan.shape.scenes, 0);
}

#[test]
fn a_prologue_is_a_chapter_left_out_of_the_numbering() {
    let mut p = P::new();
    p.draft();
    let prologue = p.folder(1, "Prologue");
    p.text(2, "Dawn");
    p.folder(1, "Chapter 1");
    p.text(2, "Day");
    let plan = p.plan();
    let row = row_of(&plan, prologue);
    assert_eq!(row.sub_role, SubRole::ChapterScene);
    assert!(row.exclude_from_numbering);
    assert!(has(
        &plan,
        &StructureNotice::PrologueNotNumbered { item: prologue }
    ));
}

// ---------------------------------------------------------------- D3.4 empty folders

#[test]
fn an_empty_folder_takes_its_siblings_level_or_stays_plain() {
    let mut p = P::new();
    p.draft();
    p.folder(1, "One");
    p.text(2, "a");
    p.folder(1, "Two");
    p.text(2, "b");
    let planned = p.folder(1, "Three, not written yet");
    p.folder(1, "Four");
    p.text(2, "c");
    let plan = p.plan();
    assert_eq!(row_of(&plan, planned).sub_role, SubRole::ChapterScene);
    assert_eq!(row_of(&plan, planned).why, Why::EmptyFolder);

    let mut p = P::new();
    p.draft();
    p.text(1, "a");
    p.text(1, "b");
    let umbruch = p.folder(1, "UMBRUCH");
    p.text(1, "c");
    let plan = p.plan();
    assert_eq!(row_of(&plan, umbruch).sub_role, SubRole::None);
    assert!(has(
        &plan,
        &StructureNotice::EmptyFolderPlain { item: umbruch }
    ));
}

// ---------------------------------------------------------------- D3.5, D3.6, D3.10

#[test]
fn a_chapter_folder_keeps_its_prose_and_its_notes_follow_it() {
    let mut p = P::new();
    p.draft();
    let chapter = p.folder(1, "Introduction: Lorem Ipsum");
    p.with_text(chapter);
    p.with_notes(chapter);
    p.with_synopsis(chapter);
    let scene = p.text(2, "Section");
    p.with_notes(scene);
    let plan = p.plan();
    assert_eq!(
        show(&plan.manuscript),
        [
            "Folder/Book [Novel]",
            "  Folder/ChapterScene Introduction: Lorem Ipsum",
            "    Item/Note Introduction: Lorem Ipsum (notes)",
            "    Item/Scene Section",
            "    Item/Note Section (notes)",
            "  Item/BookEnd",
        ]
    );
    let row = row_of(&plan, chapter);
    let roles: Vec<ContentRole> = row.contents.iter().map(|c| c.role.clone()).collect();
    assert_eq!(
        roles,
        [
            ContentRole::ChapterTitle,
            ContentRole::SceneText,
            ContentRole::SynopsisText
        ]
    );
    assert!(
        !plan.manuscript[2].exportable,
        "document notes never export"
    );
    assert!(has(&plan, &StructureNotice::DocumentNotes { item: scene }));
}

#[test]
fn a_part_hands_its_prose_to_a_first_child_titled_as_it_is() {
    let mut p = P::new();
    p.draft();
    let part = p.folder(1, "The Coast");
    p.with_text(part);
    p.with_synopsis(part);
    p.with_notes(part);
    p.folder(2, "Landfall");
    p.text(3, "Arrival");
    let plan = p.plan();
    assert_eq!(
        show(&plan.manuscript)[1..5],
        [
            "  Folder/Part The Coast",
            "    Item/Scene The Coast",
            "    Item/Note The Coast (notes)",
            "    Folder/ChapterScene Landfall",
        ]
    );
    // The synopsis stays on the folder (D3.19); the prose is on the child.
    let folder = row_of(&plan, part);
    assert!(
        folder
            .contents
            .iter()
            .any(|c| c.role == ContentRole::SynopsisText)
    );
    let child = &plan.manuscript[2];
    assert_eq!(child.origin, Origin::OwnText { item: part });
    assert_eq!(
        plan.placements[part as usize].text,
        Some((
            RowRef {
                binder: BinderKind::Manuscript,
                row: 2
            },
            ContentRole::SceneText
        ))
    );
    assert!(has(&plan, &StructureNotice::OwnTextMoved { item: part }));
}

#[test]
fn a_leaf_chapter_is_a_flat_chapter_and_the_majority_sets_the_encoding() {
    let mut p = P::new();
    p.types(&["Chapter"]);
    p.draft();
    for k in 0..3 {
        let t = p.text(1, &format!("Leaf {k}"));
        p.meta(t).section_type = Some("T0".to_string());
    }
    p.folder(1, "Chapter 9");
    p.text(2, "In a folder");
    let plan = p.plan();
    assert_eq!(plan.shape.flat_chapters, 3);
    assert_eq!(plan.shape.chapter_folders, 1);
    assert_eq!(plan.chapter_encoding, ChapterEncoding::Flat);
}

// ---------------------------------------------------------------- D3.7 paratext

#[test]
fn front_and_back_matter_in_the_draft_become_paratext() {
    let mut p = P::new();
    p.types(&["Notes"]);
    p.draft();
    let title_page = p.text(1, "Title Page");
    p.folder(1, "Chapter 1");
    p.text(2, "Opening");
    let appendices = p.folder(1, "More");
    p.with_text(appendices);
    let a1 = p.text(2, "Appendix-1");
    p.meta(a1).include_in_compile = false;
    p.text(2, "Appendix B: tables");
    let note = p.text(1, "To research");
    p.meta(note).section_type = Some("T0".to_string());
    let plan = p.plan();
    assert_eq!(
        show(&plan.manuscript),
        [
            "Folder/Book [Novel]",
            "  Item/Paratext Title Page",
            "  Folder/ChapterScene Chapter 1",
            "    Item/Scene Opening",
            "  Folder/Paratext More",
            "    Item/Paratext More",
            "    Item/Paratext Appendix-1",
            "    Item/Paratext Appendix B: tables",
            "  Item/Note To research",
            "  Item/BookEnd",
        ]
    );
    assert!(row_of(&plan, title_page).exportable);
    assert!(!row_of(&plan, a1).exportable, "follows IncludeInCompile");
    assert!(!row_of(&plan, note).exportable, "a note never exports");
    assert!(has(
        &plan,
        &StructureNotice::ParatextNotCounted {
            documents: 4,
            words: 4000
        }
    ));
}

// ---------------------------------------------------------------- D3.8, D3.9 books

#[test]
fn several_books_keep_every_item_inside_a_book() {
    let mut p = P::new();
    let front = p.folder(0, "Front Matter");
    p.meta(front).icon = Some("Front Matter".to_string());
    p.text(1, "Title Page");
    let draft = p.draft();
    p.with_synopsis(draft);
    let opening = p.text(1, "Opening");
    let one = p.folder(1, "Book One");
    p.folder(2, "Chapter 1");
    p.text(3, "a");
    let two = p.folder(1, "Book Two");
    p.folder(2, "Chapter 1");
    p.text(3, "b");
    let coda = p.text(1, "Coda");
    let back = p.folder(0, "Back Matter");
    p.meta(back).icon = Some("Back Matter.tiff".to_string());
    p.text(1, "About the author");
    let plan = p.plan();
    assert_eq!(
        show(&plan.manuscript),
        [
            "Item/Note Draft",
            "Folder/Book Book One",
            "  Folder/Paratext Front Matter",
            "    Item/Paratext Title Page",
            "  Item/Scene Opening",
            "  Folder/ChapterScene Chapter 1",
            "    Item/Scene a",
            "  Item/BookEnd",
            "Folder/Book Book Two",
            "  Folder/ChapterScene Chapter 1",
            "    Item/Scene b",
            "  Item/Scene Coda",
            "  Folder/Paratext Back Matter",
            "    Item/Paratext About the author",
            "  Item/BookEnd",
        ]
    );
    assert!(has(
        &plan,
        &StructureNotice::DraftContentAsNote { item: draft }
    ));
    assert_eq!(row_of(&plan, opening).sub_role, SubRole::Scene);
    assert_eq!(row_of(&plan, coda).sub_role, SubRole::Scene);
    let books = plan
        .review
        .books
        .as_ref()
        .map(|b| (b.claimed.clone(), b.detected));
    assert_eq!(books, Some((vec![one, two], BooksChoice::Several)));
    // Read as one book, the two become parts.
    let one_book = p.plan_with(&Choices {
        books: Some(BooksChoice::One),
        ..Choices::default()
    });
    assert_eq!(one_book.shape.books, 1);
    assert_eq!(one_book.shape.parts, 2);
    assert!(has(
        &one_book,
        &StructureNotice::BookInsideBook { item: one }
    ));
}

#[test]
fn a_single_book_folder_is_a_part_unless_the_writer_says_book() {
    let mut p = P::new();
    p.draft();
    let book = p.folder(1, "Book One");
    p.folder(2, "Chapter 1");
    p.text(3, "a");
    let plan = p.plan();
    assert_eq!(row_of(&plan, book).sub_role, SubRole::Part);
    assert_eq!(
        plan.review.books.as_ref().map(|b| b.detected),
        Some(BooksChoice::One)
    );
    let several = adjust(
        &p.project,
        &p.facts,
        &plan,
        &Adjustment::Books(Some(BooksChoice::Several)),
    );
    let several = match several {
        Ok(plan) => plan,
        Err(e) => panic!("{e}"),
    };
    assert_eq!(row_of(&several, book).sub_role, SubRole::Book);
    assert_eq!(several.shape.books, 1);
}

#[test]
fn the_book_takes_the_project_title_unless_the_writer_renamed_the_draft() {
    let mut p = P::new();
    p.draft();
    assert_eq!(
        p.plan().manuscript[0].title,
        PlannedTitle::ProjectTitle("Novel".to_string())
    );
    p.project.binder[0].title = Some("Bowl of Salt".to_string());
    assert_eq!(
        p.plan().manuscript[0].title,
        PlannedTitle::Own("Bowl of Salt".to_string())
    );
    p.project.binder[0].title = Some("<Proposal Title>".to_string());
    assert_eq!(
        p.plan().manuscript[0].title,
        PlannedTitle::ProjectTitle("Novel".to_string())
    );
    p.project.binder[0].title = Some("Short Story".to_string());
    assert_eq!(
        p.plan().manuscript[0].title,
        PlannedTitle::ProjectTitle("Novel".to_string())
    );
    p.project.properties.title = Some("The Keeper".to_string());
    assert_eq!(
        p.plan().manuscript[0].title,
        PlannedTitle::ProjectTitle("The Keeper".to_string())
    );
}

#[test]
fn a_project_without_a_draft_still_has_a_book() {
    let mut p = P::new();
    p.folder(0, "Notes");
    p.text(1, "Idea");
    let plan = p.plan();
    assert_eq!(
        show(&plan.manuscript),
        ["Folder/Book [Novel]", "  Item/BookEnd"]
    );
    assert_eq!(plan.manuscript[0].origin, Origin::Book);
}

// ---------------------------------------------------------------- D3.11 root matter

#[test]
fn root_front_and_back_matter_go_inside_the_book_out_of_export() {
    let mut p = P::new();
    p.types(&["Front Matter"]);
    let front = p.folder(0, "Elementos pré-textuais");
    p.meta(front).icon = Some("Front Matter.png".to_string());
    let dedication = p.text(1, "Dedicatória");
    p.with_notes(dedication);
    p.draft();
    p.text(1, "Scene");
    let by_type = p.text(0, "Metadata page");
    p.meta(by_type).section_type = Some("T0".to_string());
    let back = p.text(0, "Back Matter");
    p.text(1, "Bibliografia");
    let plan = p.plan();
    assert_eq!(
        show(&plan.manuscript),
        [
            "Folder/Book [Novel]",
            "  Folder/Paratext Elementos pré-textuais",
            "    Item/Paratext Dedicatória",
            "    Item/Note Dedicatória (notes)",
            "  Item/Paratext Metadata page",
            "  Item/Scene Scene",
            "  Folder/Paratext Back Matter",
            "    Item/Paratext Back Matter",
            "    Item/Paratext Bibliografia",
            "  Item/BookEnd",
        ]
    );
    for row in &plan.manuscript[1..9] {
        if row.origin != (Origin::Item { item: 3 }) {
            assert!(!row.exportable || row.sub_role == SubRole::Scene, "{row:?}");
        }
    }
    assert!(has(
        &plan,
        &StructureNotice::MatterInsideBook {
            item: front,
            back: false
        }
    ));
    assert!(has(
        &plan,
        &StructureNotice::MatterInsideBook {
            item: back,
            back: true
        }
    ));
    assert!(plan.notes.is_empty());
}

// ---------------------------------------------------------------- D3.12, D3.13

#[test]
fn media_items_become_notes_where_they_are() {
    let mut p = P::new();
    p.draft();
    let stray = p.add(1, ItemKind::Pdf, "Stray PDF");
    p.with_media(stray);
    p.add(0, ItemKind::ResearchFolder, "Recherches");
    let pic = p.add(1, ItemKind::Image, "Map");
    p.with_media(pic);
    p.with_text(pic);
    let with_children = p.add(1, ItemKind::Image, "Album");
    p.with_media(with_children);
    p.text(2, "Caption");
    let plan = p.plan();
    assert_eq!(row_of(&plan, stray).sub_role, SubRole::Note);
    assert!(has(&plan, &StructureNotice::MediaInDraft { item: stray }));
    assert_eq!(
        show(&plan.notes),
        [
            "Folder/Note Recherches",
            "  Item/Note Map",
            "  Folder/Note Album",
            "    Item/Note Album",
            "    Item/Note Caption",
        ]
    );
    let map = row_of(&plan, pic);
    assert_eq!(
        map.contents[0].body,
        Body::From(vec![
            Source::Media { item: pic },
            Source::Text { item: pic }
        ])
    );
}

// ---------------------------------------------------------------- D3.14 story bible

#[test]
fn story_bible_folders_tag_their_entries_and_never_themselves() {
    let mut p = P::new();
    p.draft();
    p.text(1, "Scene");
    let cast = p.folder(0, "Cast list");
    p.meta(cast).icon = Some("Characters (Photo)".to_string());
    p.with_text(cast);
    let mara = p.text(1, "Mara");
    let minor = p.folder(1, "Minor people");
    p.with_text(minor);
    let ben = p.text(2, "Ben");
    let hero = p.text(1, "Hero");
    let entry = p.text(1, "The Inspector");
    p.text(2, "His past");
    p.add(0, ItemKind::ResearchFolder, "Research");
    let second = p.folder(1, "cast list");
    p.meta(second).icon = Some("Characters (Mask)".to_string());
    p.text(2, "Olga");
    p.add(0, ItemKind::TrashFolder, "Trash");
    let places = p.folder(1, "Places");
    p.text(2, "Old town");
    let plan = p.plan();
    assert_eq!(plan.tags.len(), 1, "same title merges: {:?}", plan.tags);
    let tag = &plan.tags[0];
    assert_eq!(tag.name, "Cast list");
    assert_eq!(tag.folder, cast);
    let tagged: Vec<String> = plan
        .notes
        .iter()
        .filter(|r| r.tag.is_some())
        .map(title_of)
        .collect();
    // Every entry below the folder, a sub-document of an entry included.
    assert_eq!(
        tagged,
        ["Mara", "Ben", "Hero", "The Inspector", "His past", "Olga"]
    );
    // The folder's own text is not an entry, nor is a sub-folder's.
    for row in &plan.notes {
        if row.origin == (Origin::OwnText { item: cast })
            || row.origin == (Origin::OwnText { item: minor })
        {
            assert_eq!(row.tag, None);
        }
    }
    assert_eq!(row_of(&plan, mara).tag, Some(0));
    assert_eq!(row_of(&plan, ben).tag, Some(0));
    assert!(has(
        &plan,
        &StructureNotice::GenericSheetTitle { item: hero }
    ));
    // A trashed "Places" folder mints no live tag.
    assert_eq!(row_of(&plan, places).tag, None);
    // A text with documents under it is an entry: its own text is tagged.
    let own = plan
        .notes
        .iter()
        .find(|r| r.origin == (Origin::OwnText { item: entry }));
    assert_eq!(own.map(|r| r.tag), Some(Some(0)));
}

#[test]
fn a_story_bible_tag_starts_entries_from_the_folders_template() {
    let mut p = P::new();
    p.draft();
    p.text(1, "Scene");
    let holder = p.folder(0, "Holder");
    p.text(1, "x");
    let characters = p.folder(0, "Characters");
    p.text(1, "Mara");
    let templates = p.folder(0, "Templates");
    let sheet = p.text(1, "Character Sketch");
    let scene_sheet = p.text(1, "Scene Sheet");
    p.project.template_folder = p.project.binder[templates as usize].key;
    p.project.binder[characters as usize].default_child_template =
        p.project.binder[sheet as usize].key;
    // A folder naming a template no story-bible tag can carry.
    p.project.binder[holder as usize].default_child_template =
        p.project.binder[scene_sheet as usize].key;
    let plan = p.plan();
    assert_eq!(plan.templates.len(), 2);
    assert_eq!(plan.tags.len(), 1);
    assert_eq!(plan.tags[0].template, Some(0));
    assert!(has(
        &plan,
        &StructureNotice::StoryBibleFolder {
            item: characters,
            template: true
        }
    ));
    assert!(has(
        &plan,
        &StructureNotice::DefaultTemplateNotLinked { item: holder }
    ));
}

// ---------------------------------------------------------------- D3.15, D3.16, D3.17

#[test]
fn project_notes_skip_the_empty_ones() {
    let mut p = P::new();
    p.draft();
    p.project.project_notes = vec![
        ProjectNote {
            id: Some(1),
            title: Some("General".to_string()),
            modified: None,
            file: None,
        },
        ProjectNote {
            id: Some(2),
            title: Some("Plot\nideas".to_string()),
            modified: None,
            file: Some(member("Files/ProjectNotes/Note-2.rtf")),
        },
    ];
    p.facts.project_notes = vec![Carries::No, Carries::Yes];
    let plan = p.plan();
    assert_eq!(
        show(&plan.notes),
        ["Folder/Note <project notes>", "  Item/Note Plot ideas"]
    );
    assert!(has(&plan, &StructureNotice::ProjectNotes { count: 1 }));
}

#[test]
fn the_templates_folder_becomes_note_templates_and_keeps_what_they_cannot_hold() {
    let mut p = P::new();
    p.draft();
    p.text(1, "Scene");
    let templates = p.folder(0, "Templates");
    let sketch = p.text(1, "Character Sketch");
    p.with_synopsis(sketch);
    let group = p.folder(1, "Places");
    let place = p.text(2, "Setting Sketch");
    let picture = p.add(1, ItemKind::Image, "Mood board");
    p.with_media(picture);
    let empty = p.add(1, ItemKind::Text, "Blank sheet");
    p.project.template_folder = p.project.binder[templates as usize].key;
    let plan = p.plan();
    assert_eq!(
        plan.templates
            .iter()
            .map(|t| (t.name.as_str(), t.body))
            .collect::<Vec<_>>(),
        [
            ("Character Sketch", Some(Source::Text { item: sketch })),
            ("Setting Sketch", Some(Source::Text { item: place })),
            ("Blank sheet", None),
        ]
    );
    assert_eq!(
        show(&plan.notes),
        ["Item/Note Character Sketch", "Item/Note Mood board"]
    );
    assert!(has(
        &plan,
        &StructureNotice::TemplateNestingFlattened { item: group }
    ));
    assert!(has(&plan, &StructureNotice::TemplatesCreated { count: 3 }));
    assert_eq!(
        plan.placements[templates as usize].absence,
        Some(Absence::SpecialFolder)
    );
    assert_eq!(plan.placements[empty as usize].template, Some(2));
}

#[test]
fn a_templates_folder_in_the_trash_is_trash() {
    let mut p = P::new();
    p.draft();
    p.add(0, ItemKind::TrashFolder, "Trash");
    let templates = p.folder(1, "Templates");
    p.text(2, "Old sheet");
    p.project.template_folder = p.project.binder[templates as usize].key;
    let plan = p.plan();
    assert!(plan.templates.is_empty());
    assert_eq!(
        show(&plan.notes),
        ["Folder/Note Templates ~", "  Item/Note Old sheet ~"]
    );
}

#[test]
fn the_trash_comes_across_deleted_or_is_left_out_and_said_so() {
    let mut p = P::new();
    p.draft();
    p.add(0, ItemKind::TrashFolder, "Trash");
    let branch = p.folder(1, "Old branch");
    p.text(2, "Gone");
    let leaf = p.text(1, "Cut scene");
    p.with_notes(leaf);
    let plan = p.plan();
    assert_eq!(
        show(&plan.notes),
        [
            "Folder/Note Old branch ~",
            "  Item/Note Gone ~",
            "Item/Note Cut scene ~",
            "Item/Note Cut scene (notes) ~",
        ]
    );
    assert_eq!(plan.trash_roots, [0, 2, 3]);
    assert!(has(
        &plan,
        &StructureNotice::TrashBroughtAcross {
            items: 3,
            with_writing: 2
        }
    ));
    assert_eq!(
        plan.review
            .trash
            .as_ref()
            .map(|t| (t.items, t.with_writing)),
        Some((3, 2))
    );
    assert_eq!(plan.review.set_aside.trashed, 2);
    let out = p.plan_with(&Choices {
        trash: TrashChoice::LeaveOut,
        ..Choices::default()
    });
    assert!(out.notes.is_empty());
    assert_eq!(
        out.placements[branch as usize].absence,
        Some(Absence::TrashLeftOut)
    );
    assert!(has(
        &out,
        &StructureNotice::TrashLeftOut {
            items: 3,
            with_writing: 2
        }
    ));
}

#[test]
fn recovered_texts_have_a_folder_of_their_own() {
    let mut p = P::new();
    p.draft();
    let scene = p.text(1, "Scene");
    p.project.recovered = vec![
        RecoveredText {
            origin: RecoveredOrigin::Unreferenced,
            role: TextRole::Text,
            member: member("Files/Docs/88.rtf"),
            names: None,
        },
        RecoveredText {
            origin: RecoveredOrigin::Unreferenced,
            role: TextRole::Text,
            member: member("Files/Docs/89.rtf"),
            names: p.project.binder[scene as usize].key,
        },
        RecoveredText {
            origin: RecoveredOrigin::Unreferenced,
            role: TextRole::Notes,
            member: member("Files/Docs/90_notes.rtf"),
            names: None,
        },
        RecoveredText {
            origin: RecoveredOrigin::SyncConflict(ConflictScheme::Dropbox),
            role: TextRole::Notes,
            member: member("Files/Docs/89_notes (Tav's conflicted copy 2019-01-02).rtf"),
            names: p.project.binder[scene as usize].key,
        },
        RecoveredText {
            origin: RecoveredOrigin::Unreferenced,
            role: TextRole::Text,
            member: member("Files/Data/4A1D6B18-0952-42EB-A487-D17915DD3BA1/content.rtf"),
            names: None,
        },
    ];
    p.facts.recovered = vec![
        Carries::Yes,
        Carries::Unknown,
        Carries::No,
        Carries::Yes,
        Carries::Yes,
    ];
    let plan = p.plan();
    // Titled after the item copied (its notes as its notes), else by the Scrivener
    // document number the file's name gives, else blank: never a word Scrivener chose
    // for its files, never a person's name a sync service wrote (X-18).
    assert_eq!(
        show(&plan.notes),
        [
            "Folder/Note <recovered>",
            "  Item/Note 88",
            "  Item/Note Scene",
            "  Item/Note Scene (notes)",
            "  Item/Note"
        ]
    );
    assert!(has(&plan, &StructureNotice::RecoveredTexts { count: 4 }));
    assert_eq!(plan.review.set_aside.recovered, 4);
}

/// A report names a recovered text as its row is titled: by the path of the item it is a
/// copy of, else by the document number its file's name gives, else by nothing. Never by
/// the file's own name, which a sync service writes a person's or a computer's name into
/// (X-18): the report crosses to the interface, and from there to anything that logs it.
#[test]
fn a_recovered_text_is_named_as_its_row_is_titled() {
    let mut p = P::new();
    p.draft();
    let scene = p.text(1, "Scene");
    let gone = ItemKey::Uuid(uuid::Uuid::from_u128(
        0x4A1D_6B18_0952_42EB_A487_D179_15DD_3BA1,
    ));
    p.project.recovered = vec![
        RecoveredText {
            origin: RecoveredOrigin::OrphanCopy(ConflictScheme::Dropbox),
            role: TextRole::Text,
            member: member("Files/Docs/88 (Jane Doe's conflicted copy 2019-03-04).rtf"),
            names: Some(ItemKey::Number(88)),
        },
        RecoveredText {
            origin: RecoveredOrigin::OrphanCopy(ConflictScheme::Syncthing),
            role: TextRole::Text,
            member: member(
                "Files/Data/4A1D6B18-0952-42EB-A487-D17915DD3BA1/content.sync-conflict-20190304-101112-JANEDSK.rtf",
            ),
            names: Some(gone),
        },
        RecoveredText {
            origin: RecoveredOrigin::SyncConflict(ConflictScheme::Stem),
            role: TextRole::Text,
            member: member("Files/Docs/89-DESKTOP-JANE.rtf"),
            names: p.project.binder[scene as usize].key,
        },
    ];
    p.facts.recovered = vec![Carries::Yes; 3];
    let names = p.project.source_names();
    assert_eq!(
        names.recovered,
        vec![
            vec!["88".to_string()],
            Vec::new(),
            vec!["Draft".to_string(), "Scene".to_string()],
        ]
    );
    let spelled = format!("{names:?}");
    for personal in ["Jane", "JANE", "conflict", ".rtf"] {
        assert!(!spelled.contains(personal), "{personal} in {spelled}");
    }
    // The rows the plan titles agree with what the report calls them.
    let plan = p.plan();
    assert_eq!(
        show(&plan.notes),
        [
            "Folder/Note <recovered>",
            "  Item/Note 88",
            "  Item/Note",
            "  Item/Note Scene",
        ]
    );
}

// ---------------------------------------------------------------- D3.18 titles, A3

#[test]
fn titles_are_cleaned_never_invented_and_placeholders_are_stored_blank() {
    let mut p = P::new();
    p.draft();
    let chapter = p.folder(1, "Chapter");
    let scene = p.text(2, "Scene");
    let messy = p.text(2, "Varesk\nVaresk  Tolvane\tQuen\n");
    let nfd = p.text(2, "Aloe\u{0308} last draft");
    let blank = p.add(2, ItemKind::Text, "");
    p.with_text(blank);
    p.project.binder[blank as usize].title = None;
    let numbered = p.folder(1, "CHAPTER II");
    p.text(2, "More");
    let plan = p.plan();
    assert_eq!(
        row_of(&plan, chapter).title,
        PlannedTitle::Own(String::new())
    );
    assert!(has(
        &plan,
        &StructureNotice::PlaceholderTitle { item: chapter }
    ));
    assert_eq!(
        row_of(&plan, scene).title,
        PlannedTitle::Own("Scene".to_string()),
        "scene titles do not print: kept"
    );
    assert_eq!(
        row_of(&plan, messy).title,
        PlannedTitle::Own("Varesk Varesk Tolvane Quen".to_string())
    );
    assert!(has(&plan, &StructureNotice::TitleCleaned { item: messy }));
    assert_eq!(
        row_of(&plan, nfd).title,
        PlannedTitle::Own("Aloë last draft".to_string())
    );
    assert!(has(&plan, &StructureNotice::TitleCleaned { item: nfd }));
    assert_eq!(row_of(&plan, blank).title, PlannedTitle::Own(String::new()));
    assert!(has(
        &plan,
        &StructureNotice::TitleRepeatsNumber { item: numbered }
    ));
}

// ---------------------------------------------------------------- D3.23

#[test]
fn page_breaks_and_compile_as_is_are_reported_where_they_have_no_slot() {
    let mut p = P::new();
    p.draft();
    let chapter = p.folder(1, "Chapter 1");
    p.meta(chapter).page_break_before = true;
    let scene = p.text(2, "Scene");
    p.meta(scene).page_break_before = true;
    p.meta(scene).compile_as_is = true;
    let plan = p.plan();
    assert!(!has(
        &plan,
        &StructureNotice::PageBreakNotKept { item: chapter }
    ));
    assert!(has(
        &plan,
        &StructureNotice::PageBreakNotKept { item: scene }
    ));
    assert!(has(
        &plan,
        &StructureNotice::CompileAsIsNotKept { item: scene }
    ));
}

#[test]
fn an_item_left_out_of_compile_is_left_out_of_export() {
    let mut p = P::new();
    p.draft();
    let scene = p.text(1, "Draft only");
    p.meta(scene).include_in_compile = false;
    let plan = p.plan();
    assert!(!row_of(&plan, scene).exportable);
    assert!(has(&plan, &StructureNotice::NotCompiled { item: scene }));
    assert!(plan.manuscript[0].exportable, "the book always exports");
}

// ---------------------------------------------------------------- A2, A12, S3

fn one_document(p: &mut P, depth: u16, lead: bool) -> u32 {
    let doc = p.text(depth, "Manuscript");
    p.facts.items[doc as usize].text.headings = (1..=4)
        .map(|k| Heading {
            block: k * 10,
            title: format!("CHAPTER {}", ["I", "II", "III", "IV"][k as usize - 1]),
        })
        .collect();
    p.facts.items[doc as usize].text.lead = if lead { Carries::Yes } else { Carries::No };
    doc
}

#[test]
fn a_one_document_manuscript_is_split_into_chapters_by_default() {
    let mut p = P::new();
    p.draft();
    let doc = one_document(&mut p, 1, true);
    p.with_synopsis(doc);
    let plan = p.plan();
    assert_eq!(
        show(&plan.manuscript),
        [
            "Folder/Book [Novel]",
            "  Item/Scene Manuscript",
            "  Item/ChapterScene CHAPTER I",
            "  Item/ChapterScene CHAPTER II",
            "  Item/ChapterScene CHAPTER III",
            "  Item/ChapterScene CHAPTER IV",
            "  Item/BookEnd",
        ]
    );
    assert_eq!(
        plan.manuscript[3].origin,
        Origin::SplitPart { item: doc, part: 2 }
    );
    assert!(has(
        &plan,
        &StructureNotice::OneDocumentManuscript {
            item: doc,
            headings: 4,
            split: true
        }
    ));
    assert_eq!(plan.review.splits.len(), 1);
    let kept = adjust(
        &p.project,
        &p.facts,
        &plan,
        &Adjustment::Split {
            item: doc,
            split: Some(false),
        },
    );
    let kept = match kept {
        Ok(plan) => plan,
        Err(e) => panic!("{e}"),
    };
    assert_eq!(
        show(&kept.manuscript),
        [
            "Folder/Book [Novel]",
            "  Item/Scene Manuscript",
            "  Item/BookEnd"
        ]
    );
    assert!(has(
        &kept,
        &StructureNotice::OneDocumentManuscript {
            item: doc,
            headings: 4,
            split: false
        }
    ));
}

#[test]
fn nothing_before_the_first_heading_means_the_first_chapter_is_the_document() {
    let mut p = P::new();
    p.draft();
    let doc = one_document(&mut p, 1, false);
    let plan = p.plan();
    assert_eq!(plan.manuscript[1].origin, Origin::Item { item: doc });
    assert_eq!(plan.shape.flat_chapters, 4);
    assert_eq!(plan.shape.scenes, 0);
}

#[test]
fn a_document_inside_a_chapter_is_not_offered_a_split() {
    let mut p = P::new();
    p.draft();
    p.folder(1, "Chapter 1");
    one_document(&mut p, 2, true);
    let plan = p.plan();
    assert!(plan.review.splits.is_empty());
    assert_eq!(plan.shape.flat_chapters, 0);
    // Reported, so the writer can look at it, but not split.
    assert!(plan.notices.iter().any(|n| matches!(
        n,
        StructureNotice::OneDocumentManuscript { split: false, .. }
    )));
}

#[test]
fn a_chapter_folder_holding_the_whole_book_in_its_text_is_split_after_it() {
    // A corpus book: a childless folder titled like a chapter whose own text
    // holds every chapter as a heading.
    let mut p = P::new();
    p.draft();
    let folder = p.folder(1, "CHAPTER II");
    p.with_text(folder);
    p.facts.items[folder as usize].text.headings = ["CHAPTER II", "CHAPTER III", "CHAPTER IV"]
        .iter()
        .enumerate()
        .map(|(k, t)| Heading {
            block: k as u32 * 5,
            title: t.to_string(),
        })
        .collect();
    p.facts.items[folder as usize].text.lead = Carries::No;
    let plan = p.plan();
    assert_eq!(
        show(&plan.manuscript),
        [
            "Folder/Book [Novel]",
            "  Folder/ChapterScene CHAPTER II",
            "  Item/ChapterScene CHAPTER III",
            "  Item/ChapterScene CHAPTER IV",
            "  Item/BookEnd",
        ]
    );
    assert_eq!(
        plan.manuscript[1].contents[1].body,
        Body::From(vec![Source::TextPart {
            item: folder,
            part: 1
        }])
    );
    assert_eq!(plan.review.splits.len(), 1);
}

#[test]
fn a_scene_titled_with_an_article_and_a_level_word_stays_a_scene() {
    // "Une partie d'échecs" is a game of chess. Read as "part one", it became
    // a part whose heading the export printed, holding every later scene.
    let mut p = P::new();
    p.draft();
    p.text(1, "Le matin");
    let game = p.text(1, "Une partie d'échecs");
    p.text(1, "Le soir");
    let plan = p.plan();
    assert_eq!(
        show(&plan.manuscript),
        [
            "Folder/Book [Novel]",
            "  Item/Scene Le matin",
            "  Item/Scene Une partie d'échecs",
            "  Item/Scene Le soir",
            "  Item/BookEnd",
        ]
    );
    assert_eq!(row_of(&plan, game).why, Why::Text);
}

#[test]
fn a_document_of_numbered_scenes_is_not_split_into_chapters() {
    let mut p = P::new();
    p.draft();
    let doc = p.text(1, "Chapter 1");
    let facts = crate::content::rtf(
        br"{\rtf1\ansi Chapter 1 - Scene 1\par It began.\par Chapter 1 - Scene 2\par It went on.\par Chapter 1 - Scene 3\par It ended.\par}",
    );
    p.facts.items[doc as usize].text = facts;
    let plan = p.plan();
    assert!(plan.review.splits.is_empty(), "{:?}", plan.review.splits);
    assert_eq!(
        show(&plan.manuscript),
        [
            "Folder/Book [Novel]",
            "  Item/ChapterScene Chapter 1",
            "  Item/BookEnd",
        ]
    );
}

#[test]
fn a_draft_mostly_in_script_mode_is_a_script_to_check() {
    let mut p = P::new();
    p.draft();
    for k in 0..3 {
        let t = p.text(1, &format!("Scene {k}"));
        p.project.binder[t as usize].script_mode = k < 2;
    }
    let plan = p.plan();
    assert!(has(
        &plan,
        &StructureNotice::ScriptModeDraft {
            script_documents: 2,
            draft_documents: 3
        }
    ));
}

// ---------------------------------------------------------------- D3.20 review

#[test]
fn the_review_offers_levels_texts_roots_and_the_trash() {
    let mut p = P::new();
    p.draft();
    for k in 0..3 {
        p.text(1, &format!("Départ {k}"));
    }
    let book2 = p.folder(0, "Book 2");
    p.folder(1, "Episode 24");
    p.text(2, "More story");
    let ideas = p.folder(0, "Ideas");
    p.text(1, "Maybe");
    let plan = p.plan();
    let texts = &plan.review.texts;
    assert_eq!(texts.len(), 1);
    assert_eq!((texts[0].depth, texts[0].texts), (1, 3));
    assert_eq!(texts[0].median_words, 1000);
    let roots: Vec<(u32, bool)> = plan
        .review
        .roots
        .iter()
        .map(|r| (r.item, r.looks_like_manuscript))
        .collect();
    assert_eq!(roots, [(book2, true), (ideas, false)]);
    assert!(has(
        &plan,
        &StructureNotice::LooksLikeManuscript { item: book2 }
    ));

    // Texts at depth 1 are chapters.
    let chapters = adjust(
        &p.project,
        &p.facts,
        &plan,
        &Adjustment::Texts {
            depth: 1,
            kind: Some(TextKind::Chapters),
        },
    );
    let chapters = match chapters {
        Ok(plan) => plan,
        Err(e) => panic!("{e}"),
    };
    assert_eq!(chapters.shape.flat_chapters, 3);

    // "Book 2" is more of the book; "Ideas" is left out.
    let mut choices = Choices::default();
    choices.apply(&Adjustment::Root {
        item: book2,
        destination: Some(Destination::Manuscript),
    });
    choices.apply(&Adjustment::Root {
        item: ideas,
        destination: Some(Destination::LeaveOut),
    });
    let moved = p.plan_with(&choices);
    assert_eq!(moved.shape.books, 2);
    assert_eq!(
        show(&moved.manuscript)[4..],
        [
            "  Item/BookEnd",
            "Folder/Book Book 2",
            "  Folder/ChapterScene Episode 24",
            "    Item/Scene More story",
            "  Item/BookEnd",
        ]
    );
    assert!(moved.notes.is_empty());
    assert_eq!(
        moved.placements[ideas as usize].absence,
        Some(Absence::LeftOut)
    );
    assert!(has(
        &moved,
        &StructureNotice::LeftOut {
            item: ideas,
            items: 2
        }
    ));

    // A story bible by choice.
    let bible = p.plan_with(&Choices {
        roots: [(ideas, Destination::StoryBible)].into(),
        ..Choices::default()
    });
    assert_eq!(bible.tags.len(), 1);
    assert_eq!(bible.tags[0].name, "Ideas");
}

#[test]
fn a_depth_choice_applies_to_unclaimed_containers_only() {
    let mut p = P::new();
    p.types(&["Chapter"]);
    p.draft();
    let claimed = p.folder(1, "First");
    p.meta(claimed).section_type = Some("T0".to_string());
    p.text(2, "a");
    let free = p.folder(1, "Second");
    p.text(2, "b");
    let plan = p.plan();
    let depth1 = &plan.review.containers[0];
    assert_eq!((depth1.containers, depth1.adjustable), (2, 1));
    let parts = p.plan_with(&Choices {
        containers: [(1, ContainerRung::Part)].into(),
        ..Choices::default()
    });
    assert_eq!(row_of(&parts, claimed).sub_role, SubRole::ChapterScene);
    assert_eq!(row_of(&parts, free).sub_role, SubRole::Part);
    assert_eq!(row_of(&parts, free).why, Why::Choice);
    assert_eq!(parts.review.containers[0].chosen, Some(ContainerRung::Part));
    assert!(parts.review.containers[0].mixed);
}

#[test]
fn choices_the_review_does_not_offer_are_refused_or_have_no_effect() {
    let mut p = P::new();
    p.draft();
    let scene = p.text(1, "Scene");
    let bad_root = plan(
        &p.project,
        &p.facts,
        &Choices {
            roots: [(scene, Destination::Manuscript)].into(),
            ..Choices::default()
        },
    );
    assert_eq!(
        bad_root.err(),
        Some(PlanError::Choice(ChoiceError::RootNotOffered {
            item: scene
        }))
    );
    let bad_depth = plan(
        &p.project,
        &p.facts,
        &Choices {
            containers: [(4, ContainerRung::Part)].into(),
            ..Choices::default()
        },
    );
    assert_eq!(
        bad_depth.err(),
        Some(PlanError::Choice(ChoiceError::DepthOutOfRange { depth: 4 }))
    );
    // A split for a document with no headings changes nothing.
    let stale = p.plan_with(&Choices {
        splits: [(scene, true)].into(),
        ..Choices::default()
    });
    assert_eq!(show(&stale.manuscript), show(&p.plan().manuscript));
}

#[test]
fn the_plan_and_the_choices_survive_json() {
    let mut p = P::new();
    p.draft();
    p.folder(1, "Chapter 1");
    p.text(2, "Scene");
    let root = p.folder(0, "Book 2");
    p.text(1, "x");
    let mut choices = Choices::default();
    for adjustment in [
        Adjustment::Containers {
            depth: 1,
            rung: Some(ContainerRung::Plain),
        },
        Adjustment::Texts {
            depth: 2,
            kind: Some(TextKind::Chapters),
        },
        Adjustment::Root {
            item: root,
            destination: Some(Destination::StoryBible),
        },
        Adjustment::Books(Some(BooksChoice::One)),
        Adjustment::Split {
            item: 2,
            split: Some(false),
        },
        Adjustment::Trash(TrashChoice::LeaveOut),
    ] {
        let json = serde_json::to_string(&adjustment).unwrap_or_default();
        let back: Result<Adjustment, _> = serde_json::from_str(&json);
        assert_eq!(back.ok(), Some(adjustment.clone()));
        choices.apply(&adjustment);
    }
    let json = serde_json::to_string(&choices).unwrap_or_default();
    let back: Result<Choices, _> = serde_json::from_str(&json);
    assert_eq!(back.ok(), Some(choices.clone()));
    let plan = p.plan_with(&choices);
    let json = serde_json::to_string(&plan).unwrap_or_default();
    let back: Result<StructurePlan, _> = serde_json::from_str(&json);
    assert_eq!(back.ok(), Some(plan));
    // Restoring a default removes the choice.
    choices.apply(&Adjustment::Containers {
        depth: 1,
        rung: None,
    });
    assert!(choices.containers.is_empty());
}

// ---------------------------------------------------------------- G1, G4

/// Binders of tens of thousands of items in the shapes that once cost time
/// growing with the square of their size: sibling groups of empty folders or
/// of folders among texts, card images left behind, a Trash or a root folder
/// left out behind thousands of notices, recovered texts, story-bible folders.
/// The review re-plans on every click, so each must stay a moment.
#[test]
fn planning_takes_time_in_proportion_to_the_binder() {
    fn shape(kind: &str, n: u32) -> (P, Choices) {
        let mut p = P::new();
        let mut choices = Choices::default();
        p.draft();
        match kind {
            "empty folders" => {
                p.folder(1, "A");
                p.text(2, "t");
                for k in 0..n {
                    p.folder(1, &format!("E{k}"));
                }
            }
            "folders among texts" => {
                for k in 0..n {
                    p.folder(1, &format!("F{k}"));
                    p.text(2, "t");
                    p.text(1, "u");
                    p.text(1, "v");
                }
            }
            "card images" => {
                for k in 0..n {
                    let t = p.text(1, &format!("T{k}"));
                    p.with_card(t);
                }
            }
            "trash left out" | "root left out" => {
                for k in 0..n {
                    let t = p.text(1, &format!("T{k}"));
                    p.with_notes(t);
                }
                let r = if kind == "trash left out" {
                    choices.trash = TrashChoice::LeaveOut;
                    p.add(0, ItemKind::TrashFolder, "Trash")
                } else {
                    let r = p.folder(0, "Stuff");
                    choices.roots.insert(r, Destination::LeaveOut);
                    r
                };
                let _ = r;
                for k in 0..n {
                    p.text(1, &format!("D{k}"));
                }
            }
            "recovered texts" => {
                for k in 0..n {
                    p.text(1, &format!("T{k}"));
                }
                for k in 0..n {
                    p.project.recovered.push(RecoveredText {
                        origin: RecoveredOrigin::Unreferenced,
                        role: TextRole::Text,
                        member: member("Files/Docs/99.rtf"),
                        names: Some(ItemKey::Number(k + 100 + n / 2)),
                    });
                    p.facts.recovered.push(Carries::Yes);
                }
            }
            _ => {
                // Story-bible folders, each an entry holding a sub-entry.
                p.text(1, "Scene");
                for k in 0..n {
                    let f = p.folder(0, &format!("Cast {k}"));
                    p.meta(f).icon = Some("Characters (Photo)".to_string());
                    p.text(1, &format!("Mara {k}"));
                    p.text(2, "sub");
                }
            }
        }
        (p, choices)
    }
    let mut slow = Vec::new();
    for (kind, n) in [
        ("empty folders", 20_000),
        ("folders among texts", 20_000),
        ("card images", 60_000),
        ("trash left out", 30_000),
        ("root left out", 30_000),
        ("recovered texts", 40_000),
        ("story-bible folders", 20_000),
    ] {
        let (p, choices) = shape(kind, n);
        let started = std::time::Instant::now();
        let planned = plan(&p.project, &p.facts, &choices);
        let seconds = started.elapsed().as_secs_f64();
        assert!(planned.is_ok(), "{kind}: {:?}", planned.err());
        if seconds > 10.0 {
            slow.push(format!(
                "{kind} ({} items): {seconds:.1} s",
                p.project.binder.len()
            ));
        }
    }
    assert!(slow.is_empty(), "{slow:?}");
}

#[test]
fn a_binder_nested_thousands_deep_plans_on_a_small_stack() {
    let work = std::thread::Builder::new().stack_size(256 << 10).spawn(|| {
        let mut p = P::new();
        p.draft();
        for d in 1..=3000u16 {
            p.text(d, &format!("Level {d}"));
        }
        p.add(0, ItemKind::TrashFolder, "Trash");
        for d in 1..=3000u16 {
            p.folder(d, "Deeper");
        }
        let plan = p.plan();
        let json = serde_json::to_string(&plan).unwrap_or_default();
        let back: Result<StructurePlan, _> = serde_json::from_str(&json);
        (plan.manuscript.len(), back.is_ok())
    });
    let result = work.map(|h| h.join());
    match result {
        Ok(Ok((rows, round_trip))) => {
            assert!(rows > 3000);
            assert!(round_trip);
        }
        _ => panic!("the plan did not finish on a small stack"),
    }
}

// ---------------------------------------------------------------- the checker

fn small() -> P {
    let mut p = P::new();
    p.draft();
    let chapter = p.folder(1, "Chapter 1");
    p.with_synopsis(chapter);
    p.text(2, "One");
    p.text(2, "Two");
    p.add(0, ItemKind::ResearchFolder, "Research");
    p.text(1, "Idea");
    p
}

fn violation(p: &P, plan: &StructurePlan) -> Option<Violation> {
    check(&p.project, &p.facts, plan).err()
}

#[test]
fn the_checker_catches_every_kind_of_broken_plan() {
    let p = small();
    let good = p.plan();
    assert_eq!(violation(&p, &good), None);

    let mut bad = good.clone();
    bad.manuscript[1].sub_role = SubRole::BookBegin;
    bad.manuscript[1].role = Role::Folder;
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::Combination { .. })
    ));

    let mut bad = good.clone();
    bad.manuscript[2].depth = 3;
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::IndentJump { .. })
    ));

    let mut bad = good.clone();
    bad.manuscript[3].depth = 3;
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::UnderAnItem { .. })
    ));

    let mut bad = good.clone();
    let end = bad.manuscript.pop();
    bad.manuscript
        .insert(1, end.unwrap_or_else(|| panic!("a book end")));
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::OutsideBook { .. })
    ));

    let mut bad = good.clone();
    bad.manuscript[1]
        .contents
        .retain(|c| c.role != ContentRole::SynopsisText);
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::NotPlaced {
            source: Source::Synopsis { .. }
        })
    ));

    // One item's writing copied onto another item's row.
    let mut bad = good.clone();
    let copied = bad.manuscript[2].contents.clone();
    bad.manuscript[3].contents = copied;
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::Misrouted { .. })
    ));

    // A text written twice on its own row.
    let mut bad = good.clone();
    for content in &mut bad.manuscript[2].contents {
        if let Body::From(sources) = &mut content.body {
            let again = sources.clone();
            sources.extend(again);
        }
    }
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::PlacedTwice { .. })
    ));

    let mut bad = good.clone();
    bad.manuscript.swap(2, 3);
    bad.placements = build::placements(
        p.project.binder.len(),
        &bad.manuscript,
        &bad.notes,
        &bad.templates,
        &vec![None; p.project.binder.len()],
    );
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::OrderChanged { .. })
    ));

    let mut bad = good.clone();
    bad.notes[1].origin = Origin::OwnText { item: 5 };
    bad.placements = build::placements(
        p.project.binder.len(),
        &bad.manuscript,
        &bad.notes,
        &bad.templates,
        &vec![None; p.project.binder.len()],
    );
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::ItemUnaccounted { .. })
    ));

    let mut bad = good.clone();
    bad.notes[0].exportable = true;
    bad.notes[0].sub_role = SubRole::Note;
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::NoteExported { .. })
    ));

    let mut bad = good.clone();
    bad.notes[1].activated = false;
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::Activation { .. })
    ));

    let mut bad = good.clone();
    bad.manuscript[2].tag = Some(0);
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::TagMisplaced { .. })
    ));

    let mut bad = good.clone();
    let idea = bad.notes.remove(1);
    bad.manuscript.insert(3, PlannedRow { depth: 2, ..idea });
    bad.placements = build::placements(
        p.project.binder.len(),
        &bad.manuscript,
        &bad.notes,
        &bad.templates,
        &vec![None; p.project.binder.len()],
    );
    assert!(matches!(
        violation(&p, &bad),
        Some(Violation::SubtreeSplit { .. } | Violation::OrderChanged { .. })
    ));
}

/// A project that goes through every path the checker's second look covers:
/// scenes with synopses and notes, a notes-typed folder in the Draft, a
/// one-document manuscript, a root folder kept in the notes, a character sheet
/// with a portrait, and the Trash.
fn every_path() -> P {
    let mut p = P::new();
    p.types(&["Notes"]);
    p.draft();
    let chapter = p.folder(1, "Chapter 1");
    p.with_synopsis(chapter);
    let arrival = p.text(2, "Arrival");
    p.with_synopsis(arrival);
    p.with_notes(arrival);
    p.text(2, "Harbour");
    let notes = p.folder(1, "Research notes");
    p.meta(notes).section_type = Some("T0".to_string());
    p.text(2, "Note one");
    let doc = p.text(1, "Manuscript");
    p.facts.items[doc as usize].text.headings = (1..=3)
        .map(|k| Heading {
            block: k * 2,
            title: format!("Chapter {k}"),
        })
        .collect();
    p.folder(0, "Stuff");
    p.text(1, "Loose page");
    p.add(0, ItemKind::ResearchFolder, "Research");
    let mara = p.text(1, "Mara");
    p.with_card(mara);
    p.add(0, ItemKind::TrashFolder, "Trash");
    p.text(1, "Deleted");
    p
}

/// Each plan is what a mapper with one rule wrong would return, with
/// placements that agree with its rows, so only an independent look can see it.
#[test]
fn the_checker_catches_what_the_mappers_own_bookkeeping_would_not() {
    fn replan(p: &P, bad: &mut StructurePlan, absences: &[Option<Absence>]) {
        bad.placements = build::placements(
            p.project.binder.len(),
            &bad.manuscript,
            &bad.notes,
            &bad.templates,
            absences,
        );
    }
    fn row_at(plan: &mut StructurePlan, at: RowRef) -> &mut PlannedRow {
        match at.binder {
            BinderKind::Manuscript => &mut plan.manuscript[at.row as usize],
            BinderKind::Notes => &mut plan.notes[at.row as usize],
        }
    }
    let p = every_path();
    let good = p.plan();
    assert_eq!(violation(&p, &good), None);
    let absences: Vec<Option<Absence>> = good.placements.iter().map(|x| x.absence).collect();
    let item_row = |plan: &StructurePlan, item: u32| {
        plan.placements[item as usize]
            .item
            .unwrap_or_else(|| panic!("item {item} has a row"))
    };
    let mut missed: Vec<&str> = Vec::new();

    // A scene's synopsis written as its prose, and its prose as its synopsis.
    let mut bad = good.clone();
    let at = item_row(&bad, 2);
    for content in &mut row_at(&mut bad, at).contents {
        content.role = match content.role {
            ContentRole::SceneText => ContentRole::SynopsisText,
            _ => ContentRole::SceneText,
        };
    }
    replan(&p, &mut bad, &absences);
    if !matches!(violation(&p, &bad), Some(Violation::Misrouted { .. })) {
        missed.push("synopsis and prose swapped");
    }

    // A scene's document notes folded into its prose, which exports.
    let mut bad = good.clone();
    let at = item_row(&bad, 2);
    bad.manuscript
        .retain(|r| r.origin != Origin::DocumentNotes { item: 2 });
    for content in &mut row_at(&mut bad, at).contents {
        if content.role == ContentRole::SceneText
            && let Body::From(sources) = &mut content.body
        {
            sources.push(Source::Notes { item: 2 });
        }
    }
    replan(&p, &mut bad, &absences);
    if !matches!(violation(&p, &bad), Some(Violation::Misrouted { .. })) {
        missed.push("document notes exported as prose");
    }

    // A root folder the writer kept, left out as if they had asked.
    let left = p.plan_with(&Choices {
        roots: [(7, Destination::LeaveOut)].into(),
        ..Choices::default()
    });
    let mut bad = left.clone();
    bad.choices = good.choices.clone();
    if !matches!(
        violation(&p, &bad),
        Some(Violation::AbsenceUnfounded { .. })
    ) {
        missed.push("a kept root left out");
    }

    // Where each item went, with its notes named as its synopsis.
    let mut bad = good.clone();
    let placement = &mut bad.placements[2];
    std::mem::swap(&mut placement.notes, &mut placement.synopsis);
    if !matches!(
        violation(&p, &bad),
        Some(Violation::PlacementsDisagree { .. })
    ) {
        missed.push("placements swapped");
    }

    // The chapters of a one-document manuscript in reverse order.
    let mut bad = good.clone();
    let parts: Vec<usize> = bad
        .manuscript
        .iter()
        .enumerate()
        .filter(|(_, r)| matches!(r.origin, Origin::SplitPart { .. }))
        .map(|(k, _)| k)
        .collect();
    assert_eq!(parts.len(), 3);
    bad.manuscript.swap(parts[0], parts[2]);
    replan(&p, &mut bad, &absences);
    if !matches!(violation(&p, &bad), Some(Violation::OrderChanged { .. })) {
        missed.push("split chapters reversed");
    }

    // A character's portrait left behind although the sheet can hold it.
    let mut bad = good.clone();
    let at = item_row(&bad, 10);
    for content in &mut row_at(&mut bad, at).contents {
        if let Body::From(sources) = &mut content.body {
            sources.retain(|s| !matches!(s, Source::CardImage { .. }));
        }
    }
    bad.notices
        .push(StructureNotice::CardImageLeftBehind { item: 10 });
    replan(&p, &mut bad, &absences);
    if !matches!(violation(&p, &bad), Some(Violation::CardLeftOnSheet { .. })) {
        missed.push("portrait left behind on a sheet");
    }

    // The Trash brought back as live notes.
    let mut bad = good.clone();
    for row in &mut bad.notes {
        row.activated = true;
    }
    bad.trash_roots.clear();
    if !matches!(violation(&p, &bad), Some(Violation::TrashMismatch { .. })) {
        missed.push("trash brought back live");
    }

    // A note of a notes-typed Draft folder turned into a scene, which exports.
    let mut bad = good.clone();
    let at = item_row(&bad, 5);
    let row = row_at(&mut bad, at);
    row.sub_role = SubRole::Scene;
    row.exportable = true;
    for content in &mut row.contents {
        if content.role == ContentRole::NoteText {
            content.role = ContentRole::SceneText;
        }
    }
    replan(&p, &mut bad, &absences);
    if !matches!(violation(&p, &bad), Some(Violation::ModeBroken { .. })) {
        missed.push("a note of the Draft exported as a scene");
    }

    // With several books, a Draft text before the first book moved after
    // that book's own chapters: its place in the book changes.
    let mut books = P::new();
    books.draft();
    books.text(1, "Before");
    books.folder(1, "Book One");
    books.text(2, "One a");
    books.folder(1, "Book Two");
    books.text(2, "Two a");
    let good_books = books.plan();
    let absences: Vec<Option<Absence>> = good_books.placements.iter().map(|x| x.absence).collect();
    let mut bad = good_books.clone();
    let before = item_row(&bad, 1).row as usize;
    let after = item_row(&bad, 3).row as usize;
    bad.manuscript.swap(before, after);
    replan(&books, &mut bad, &absences);
    if !matches!(
        violation(&books, &bad),
        Some(Violation::OrderChanged { .. })
    ) {
        missed.push("a Draft text moved after the next book's chapters");
    }

    assert!(missed.is_empty(), "not caught: {missed:?}");
}

// ---------------------------------------------------------------- notices

#[test]
fn every_notice_has_one_listed_key_and_a_group() {
    let every = [
        StructureNotice::ScriptModeDraft {
            script_documents: 1,
            draft_documents: 1,
        },
        StructureNotice::OneDocumentManuscript {
            item: 0,
            headings: 3,
            split: true,
        },
        StructureNotice::TitleRepeatsNumber { item: 0 },
        StructureNotice::ClaimOverruled {
            item: 0,
            claimed: Level::Chapter,
        },
        StructureNotice::AdoptedByChapter {
            item: 0,
            chapter: 1,
        },
        StructureNotice::LooksLikeManuscript { item: 0 },
        StructureNotice::GenericSheetTitle { item: 0 },
        StructureNotice::PlaceholderTitle { item: 0 },
        StructureNotice::TitleCleaned { item: 0 },
        StructureNotice::OwnTextMoved { item: 0 },
        StructureNotice::TextWithChildren { item: 0 },
        StructureNotice::EmptyFolderPlain { item: 0 },
        StructureNotice::BookInsideBook { item: 0 },
        StructureNotice::PrologueNotNumbered { item: 0 },
        StructureNotice::MatterInsideBook {
            item: 0,
            back: false,
        },
        StructureNotice::ParatextNotCounted {
            documents: 1,
            words: 1,
        },
        StructureNotice::DocumentNotes { item: 0 },
        StructureNotice::DraftContentAsNote { item: 0 },
        StructureNotice::StoryBibleFolder {
            item: 0,
            template: false,
        },
        StructureNotice::TemplatesCreated { count: 1 },
        StructureNotice::TemplateNestingFlattened { item: 0 },
        StructureNotice::TemplateKeptAsNote { item: 0 },
        StructureNotice::RecoveredTexts { count: 1 },
        StructureNotice::ProjectNotes { count: 1 },
        StructureNotice::TrashBroughtAcross {
            items: 1,
            with_writing: 1,
        },
        StructureNotice::NotCompiled { item: 0 },
        StructureNotice::MediaInDraft { item: 0 },
        StructureNotice::LeftOut { item: 0, items: 1 },
        StructureNotice::TrashLeftOut {
            items: 1,
            with_writing: 1,
        },
        StructureNotice::CardImageLeftBehind { item: 0 },
        StructureNotice::DefaultTemplateNotLinked { item: 0 },
        StructureNotice::PageBreakNotKept { item: 0 },
        StructureNotice::CompileAsIsNotKept { item: 0 },
    ];
    let mut seen: Vec<&str> = every.iter().map(StructureNotice::key).collect();
    for key in &seen {
        assert!(StructureNotice::KEYS.contains(key), "{key}");
        assert!(key.starts_with("scrivener-structure-"));
    }
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen.len(), every.len(), "one key per variant");
    assert_eq!(StructureNotice::KEYS.len(), every.len());
    let check_these = every
        .iter()
        .filter(|n| n.group() == ReportGroup::CheckThese)
        .count();
    assert_eq!(check_these, 7);
}

// ---------------------------------------------------------------- card images

#[test]
fn a_card_image_goes_on_a_sheet_and_is_left_behind_on_a_manuscript_row() {
    let mut p = P::new();
    p.draft();
    let scene = p.text(1, "Scene");
    p.with_card(scene);
    let characters = p.folder(0, "Characters");
    p.with_card(characters);
    let sheet = p.text(1, "Brisane");
    p.with_card(sheet);
    let plan = p.plan();
    assert!(has(
        &plan,
        &StructureNotice::CardImageLeftBehind { item: scene }
    ));
    assert_eq!(
        row_of(&plan, sheet).contents[0].body,
        Body::From(vec![
            Source::CardImage { item: sheet },
            Source::Text { item: sheet }
        ])
    );
    // A folder's card image needs a note to sit in: its own-text child.
    assert_eq!(
        show(&plan.notes),
        [
            "Folder/Note Characters",
            "  Item/Note Characters",
            "  Item/Note Brisane"
        ]
    );
}

// ---------------------------------------------------------------- properties

mod properties {
    use super::*;
    use proptest::prelude::*;

    /// One generated item: its depth step, its kind, a title from a pool that
    /// hits every lexicon class, and what it holds.
    #[derive(Debug, Clone)]
    struct Gen {
        step: u16,
        kind: u8,
        title: u8,
        text: u8,
        notes: bool,
        synopsis: bool,
        card: bool,
        compiled: bool,
        section: u8,
        child_default: u8,
        icon: u8,
        headings: u8,
        script: bool,
    }

    const TITLES: &[&str] = &[
        "Chapter 1",
        "Chapter 1",
        "Chapter Two",
        "Part II",
        "Book One",
        "Book Two",
        "Scene 4",
        "Prologue",
        "Title Page",
        "Appendix A",
        "Characters",
        "Places",
        "Chapter",
        "Hero",
        "Front Matter",
        "Back Matter",
        "Research",
        "Notes",
        "",
        "Arrival",
        "Book 2",
        "Draft - ideas",
        "Chapter: Ironmongery",
        "Episode 30",
    ];

    fn item() -> impl Strategy<Value = Gen> {
        (
            (
                0u16..4,
                0u8..12,
                0u8..(TITLES.len() as u8),
                0u8..4,
                any::<bool>(),
            ),
            (any::<bool>(), any::<bool>(), any::<bool>(), 0u8..8, 0u8..6),
            (0u8..5, 0u8..6, any::<bool>()),
        )
            .prop_map(
                |(
                    (step, kind, title, text, notes),
                    (synopsis, card, compiled, section, child_default),
                    (icon, headings, script),
                )| Gen {
                    step,
                    kind,
                    title,
                    text,
                    notes,
                    synopsis,
                    card,
                    compiled,
                    section,
                    child_default,
                    icon,
                    headings,
                    script,
                },
            )
    }

    fn build(items: &[Gen], extras: (u8, u8, u8)) -> P {
        let mut p = P::new();
        p.types(&[
            "Chapter Heading",
            "Part Title",
            "Book Title",
            "Scene",
            "Front Matter",
            "Notes",
            "Prologue",
        ]);
        p.project.section_types.folders = vec!["T1".to_string(), "T0".to_string()];
        p.project.section_types.files = vec!["T3".to_string()];
        let mut previous: Option<u16> = None;
        for g in items {
            let allowed = previous.map_or(0, |d| d + 1);
            let depth = g.step.min(allowed);
            previous = Some(depth);
            let kind = match g.kind {
                0 => ItemKind::DraftFolder,
                1 => ItemKind::ResearchFolder,
                2 => ItemKind::TrashFolder,
                3..=5 => ItemKind::Folder,
                6 => ItemKind::Image,
                7 => ItemKind::Pdf,
                8 => ItemKind::Unknown("Bookmark".to_string()),
                _ => ItemKind::Text,
            };
            let media = kind.is_media();
            let i = p.add(depth, kind, TITLES[g.title as usize]);
            if g.title as usize % 7 == 3 {
                p.project.binder[i as usize].title = None;
            }
            match g.text {
                1 => {
                    p.with_text(i);
                }
                2 => {
                    p.facts.items[i as usize].text.carries = Carries::Unknown;
                    p.facts.items[i as usize].text.lead = Carries::Unknown;
                }
                _ => {}
            }
            if g.headings >= 3 && g.text == 1 {
                p.facts.items[i as usize].text.headings = (0..g.headings)
                    .map(|k| Heading {
                        block: u32::from(k) * 3 + 1,
                        title: format!("Chapter {}", k + 1),
                    })
                    .collect();
                p.facts.items[i as usize].text.lead = if g.headings == 4 {
                    Carries::No
                } else {
                    Carries::Yes
                };
            }
            if g.notes {
                p.with_notes(i);
            }
            if g.synopsis {
                p.with_synopsis(i);
            }
            if g.card {
                p.with_card(i);
            }
            if media {
                p.with_media(i);
            }
            let meta = p.meta(i);
            meta.include_in_compile = g.compiled;
            meta.section_type = (g.section < 7).then(|| format!("T{}", g.section));
            if g.section == 7 {
                meta.section_type = None;
            }
            if g.section % 2 == 1 {
                meta.section_type = None;
            }
            meta.section_child_default =
                (g.child_default < 3).then(|| format!("T{}", g.child_default));
            meta.icon = match g.icon {
                0 => Some("Characters (Photo)".to_string()),
                1 => Some("Front Matter".to_string()),
                2 => Some("Back Matter.tiff".to_string()),
                _ => None,
            };
            p.project.binder[i as usize].script_mode = g.script;
        }
        let (template, project_notes, recovered) = extras;
        let n = p.project.binder.len();
        if template as usize % (n + 1) < n {
            let k = template as usize % (n + 1);
            p.project.template_folder = p.project.binder[k].key;
            let key = p.project.binder[k].key;
            if let Some(row) = p.project.binder.get_mut(n / 2) {
                row.default_child_template = key;
            }
        }
        for k in 0..project_notes % 3 {
            p.project.project_notes.push(ProjectNote {
                id: Some(u32::from(k)),
                title: Some(format!("Note {k}")),
                modified: None,
                file: Some(member("Files/ProjectNotes/Note.rtf")),
            });
            p.facts.project_notes.push(if k % 2 == 0 {
                Carries::Yes
            } else {
                Carries::No
            });
        }
        for k in 0..recovered % 3 {
            p.project.recovered.push(RecoveredText {
                origin: RecoveredOrigin::Unreferenced,
                role: TextRole::Text,
                member: member("Files/Docs/99.rtf"),
                names: p.project.binder.first().and_then(|r| r.key),
            });
            p.facts.recovered.push(if k == 0 {
                Carries::Yes
            } else {
                Carries::Unknown
            });
        }
        p
    }

    /// Every choice the review offers in `plan`, one per control.
    fn offered(plan: &StructurePlan, pick: u8) -> Vec<Adjustment> {
        let mut out = Vec::new();
        let rungs = [
            ContainerRung::Part,
            ContainerRung::Chapter,
            ContainerRung::Plain,
        ];
        for c in &plan.review.containers {
            out.push(Adjustment::Containers {
                depth: c.depth,
                rung: Some(rungs[(pick as usize + c.depth as usize) % 3]),
            });
        }
        for t in &plan.review.texts {
            out.push(Adjustment::Texts {
                depth: t.depth,
                kind: Some(TextKind::Chapters),
            });
        }
        let destinations = [
            Destination::Notes,
            Destination::StoryBible,
            Destination::Manuscript,
            Destination::LeaveOut,
        ];
        for (k, r) in plan.review.roots.iter().enumerate() {
            out.push(Adjustment::Root {
                item: r.item,
                destination: Some(destinations[(pick as usize + k) % 4]),
            });
        }
        if plan.review.books.is_some() {
            out.push(Adjustment::Books(Some(if pick.is_multiple_of(2) {
                BooksChoice::One
            } else {
                BooksChoice::Several
            })));
        }
        for s in &plan.review.splits {
            out.push(Adjustment::Split {
                item: s.item,
                split: Some(!pick.is_multiple_of(3)),
            });
        }
        out.push(Adjustment::Trash(if pick.is_multiple_of(2) {
            TrashChoice::LeaveOut
        } else {
            TrashChoice::Import
        }));
        out
    }

    proptest! {
        #![proptest_config(ProptestConfig { cases: 400, ..ProptestConfig::default() })]

        /// Any binder, any facts, any choice the review offers: a plan that
        /// passes every invariant, the same after a JSON round trip.
        #[test]
        fn every_binder_and_every_offered_choice_plans_within_the_invariants(
            items in prop::collection::vec(item(), 1..60),
            extras in (any::<u8>(), any::<u8>(), any::<u8>()),
            picks in prop::collection::vec(any::<u8>(), 1..4),
        ) {
            let p = build(&items, extras);
            let first = plan(&p.project, &p.facts, &Choices::default());
            prop_assert!(first.is_ok(), "{:?}", first.err());
            let mut current = first.unwrap_or_else(|e| panic!("{e}"));
            for pick in picks {
                for adjustment in offered(&current, pick) {
                    let next = adjust(&p.project, &p.facts, &current, &adjustment);
                    prop_assert!(next.is_ok(), "{:?} after {:?}", next.err(), adjustment);
                    current = next.unwrap_or_else(|e| panic!("{e}"));
                }
            }
            let json = serde_json::to_string(&current).unwrap_or_default();
            let back: Result<StructurePlan, _> = serde_json::from_str(&json);
            prop_assert_eq!(back.ok(), Some(current.clone()));
            // Re-planning from the round-tripped choices gives the same plan.
            let again = plan(&p.project, &p.facts, &current.choices);
            prop_assert_eq!(again.ok(), Some(current));
        }

        /// Fail safe: facts that know nothing (every file unread) never give a
        /// plan with fewer rows than the measured facts.
        #[test]
        fn assumed_facts_never_lose_a_row(
            items in prop::collection::vec(item(), 1..40),
        ) {
            let mut p = build(&items, (255, 0, 0));
            for (i, row) in p.project.binder.iter_mut().enumerate() {
                if p.facts.items[i].text.carries.counts() {
                    row.payload.text = Some(member("Files/Data/x/content.rtf"));
                }
                if p.facts.items[i].notes.counts() {
                    row.payload.notes = Some(member("Files/Data/x/notes.rtf"));
                }
                if p.facts.items[i].synopsis.counts() {
                    row.payload.synopsis = Some(member("Files/Data/x/synopsis.txt"));
                }
                p.facts.items[i].text.headings.clear();
            }
            let measured = plan(&p.project, &p.facts, &Choices::default());
            let assumed_facts = ContentFacts::assumed(&p.project);
            let assumed = plan(&p.project, &assumed_facts, &Choices::default());
            prop_assert!(measured.is_ok(), "{:?}", measured.err());
            prop_assert!(assumed.is_ok(), "{:?}", assumed.err());
            let rows = |r: &Result<StructurePlan, PlanError>| {
                r.as_ref().map_or(0, |p| p.manuscript.len() + p.notes.len())
            };
            prop_assert!(rows(&assumed) >= rows(&measured));
        }
    }
}
