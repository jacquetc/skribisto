// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One test per rule of the metadata mapping (D4-2, D4-3, D4-4, D4-15), over projects
//! built in memory.

use common::entities::StatusCategory;

use super::*;
use crate::content::{Carries, ItemFacts, TextFacts};
use crate::convert::{DefaultRung, TITLE_SLOT, Words};
use crate::model::{
    BinderRow, CompileAuthor, CompileFile, CompileLanguage, Container, Counts, Generation,
    IdScheme, ItemKey, ItemMeta, ManifestChoice, ManifestOrigin, MemberRef, Payload, Platform,
    ProjectProperties, ProjectTargets, Rgb, SectionTypes, SettingsFiles, SourceFacts, VocabEntry,
    Vocabulary, Writer,
};
use crate::structure::{self, Choices, Origin, StructurePlan};

// ---------------------------------------------------------------- building

struct P {
    project: Project,
    facts: ContentFacts,
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
                    folder_name: "Novel Backup.scriv".to_string(),
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
                compile: CompileFacts::default(),
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

    /// A text with writing in it.
    fn text(&mut self, depth: u16, title: &str) -> u32 {
        let i = self.add(depth, ItemKind::Text, title);
        self.writing(i)
    }

    fn writing(&mut self, i: u32) -> u32 {
        self.project.binder[i as usize].payload.text = Some(MemberRef {
            path: format!("Files/Docs/{i}.rtf"),
            size: 64,
            modified_ns: None,
        });
        self.facts.items[i as usize].text = TextFacts {
            carries: Carries::Yes,
            words: 100,
            headings: Vec::new(),
            lead: Carries::Yes,
            letters: Default::default(),
        };
        i
    }

    fn label(&mut self, i: u32, id: i64) {
        self.project.binder[i as usize].meta.label = Some(id);
    }

    fn status(&mut self, i: u32, id: i64) {
        self.project.binder[i as usize].meta.status = Some(id);
    }

    fn plan(&self) -> StructurePlan {
        match structure::plan(&self.project, &self.facts, &Choices::default()) {
            Ok(plan) => plan,
            Err(e) => panic!("no plan: {e}"),
        }
    }

    fn vocabularies(&self) -> Vocabularies {
        vocabularies(&self.project, &self.plan(), &[], &words())
    }
}

fn list(title: &str, entries: &[(i64, &str)]) -> Vocabulary {
    Vocabulary {
        title: Some(title.to_string()),
        default: Some(-1),
        entries: entries
            .iter()
            .map(|(id, name)| VocabEntry {
                id: *id,
                name: name.to_string(),
                color: None,
            })
            .collect(),
    }
}

fn words() -> Words {
    Words {
        manuscript_binder: "W-Manuscript".to_string(),
        notes_binder: "W-Notes".to_string(),
        notes_of: format!("{TITLE_SLOT} W-notes"),
        project_notes: "W-ProjectNotes".to_string(),
        recovered: "W-Recovered".to_string(),
        untitled_template: "W-Untitled".to_string(),
        research_folder: format!("{TITLE_SLOT} W-research"),
        list_separator: " W+ ".to_string(),
        label_details: format!("W-from {TITLE_SLOT}"),
        status_details: format!("W-status {TITLE_SLOT}"),
        unnamed_label: format!("W-label {TITLE_SLOT}"),
        unnamed_status: format!("W-status {TITLE_SLOT}"),
        label_tag: format!("{TITLE_SLOT} W-(label)"),
        untitled_tag: "W-untitled".to_string(),
        default_statuses: vec![
            DefaultRung {
                name: "W-idea".to_string(),
                category: StatusCategory::Planned,
            },
            DefaultRung {
                name: "W-done".to_string(),
                category: StatusCategory::Final,
            },
        ],
    }
}

/// A small novel: a Draft with a chapter folder holding two scenes and a third scene, the
/// Research folder with a note, the Trash.
fn novel() -> (P, [u32; 6]) {
    let mut p = P::new();
    let draft = p.add(0, ItemKind::DraftFolder, "Draft");
    let chapter = p.add(1, ItemKind::Folder, "Chapter 1");
    let a = p.text(2, "Arrival");
    let b = p.text(2, "Harbour");
    let c = p.text(1, "Night");
    p.add(0, ItemKind::ResearchFolder, "Research");
    let note = p.text(1, "Mara");
    p.add(0, ItemKind::TrashFolder, "Trash");
    let _ = draft;
    (p, [chapter, a, b, c, note, draft])
}

/// The rows carrying `item`, by binder and index.
fn rows_of(plan: &StructurePlan, item: u32) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for (b, rows) in [&plan.manuscript, &plan.notes].into_iter().enumerate() {
        for (r, row) in rows.iter().enumerate() {
            let carries = match row.origin {
                Origin::Item { item: i }
                | Origin::OwnText { item: i }
                | Origin::SplitPart { item: i, .. }
                | Origin::DraftContent { item: i } => i == item,
                _ => false,
            };
            if carries {
                out.push((b, r));
            }
        }
    }
    out
}

fn tag_names(v: &Vocabularies) -> Vec<&str> {
    v.tags.iter().map(|t| t.name.as_str()).collect()
}

fn rung_names(v: &Vocabularies) -> Vec<&str> {
    v.ladder.iter().map(|r| r.name.as_str()).collect()
}

// ---------------------------------------------------------------- labels (D4-3)

/// One tag per label a row wears, in the list's order, on the rows that carry the item;
/// `0` is a real label; a label no document wears stays behind and is listed.
#[test]
fn labels_become_tags_only_where_a_row_wears_them() {
    let (mut p, [_, a, b, c, note, _]) = novel();
    p.project.labels = list(
        "Label",
        &[
            (-1, "No Label"),
            (7, "Chapter"),
            (0, "Concept"),
            (3, "Red"),
            (4, "Character Notes"),
        ],
    );
    p.project.labels.entries[1].color = Some(Rgb {
        r: 0.993_5,
        g: 0.701_213,
        b: 0.732_586,
    });
    p.label(a, 0);
    p.label(b, 7);
    p.label(note, 0);
    let v = p.vocabularies();
    assert_eq!(tag_names(&v), ["Chapter", "Concept"], "the list's order");
    assert_eq!(v.tags[0].color, "#fdb3bb", "the writer's colour, exactly");
    assert_eq!(
        v.tags[1].color, PALETTE[1],
        "no colour: the palette, in order"
    );
    assert_eq!(v.tags[0].details, "W-from Label");
    let plan = p.plan();
    for (item, tag) in [(a, 1), (b, 0), (note, 1)] {
        for (binder, row) in rows_of(&plan, item) {
            assert_eq!(v.row_tags[binder][row], Some(tag), "item {item}");
        }
    }
    for (binder, row) in rows_of(&plan, c) {
        assert_eq!(v.row_tags[binder][row], None);
    }
    assert!(v.notices.contains(&MetadataNotice::UnusedLabelsSkipped {
        names: vec!["Red".to_string(), "Character Notes".to_string()]
    }));
    assert!(
        v.notices
            .contains(&MetadataNotice::LabelsBecameTags { tags: 2, rows: 3 })
    );
    assert!(
        !v.notices
            .iter()
            .any(|n| matches!(n, MetadataNotice::FieldRenamed { .. })),
        "a shipped field title is no rename"
    );
}

/// The field's own name goes into every tag's details whatever it is, and a name no
/// shipped list knows is said once, never used to rename the tags (X-39).
#[test]
fn a_renamed_field_names_itself_in_details_and_renames_nothing() {
    let (mut p, [_, a, _, _, _, _]) = novel();
    p.project.labels = list("Time", &[(-1, "No Label"), (1, "Dawn")]);
    p.project.statuses = list("Rating", &[(-1, "No Status"), (1, "★"), (2, "★★")]);
    p.label(a, 1);
    let v = p.vocabularies();
    assert_eq!(tag_names(&v), ["Dawn"]);
    assert_eq!(v.tags[0].details, "W-from Time");
    assert_eq!(v.ladder[0].details, "W-status Rating");
    assert!(v.notices.contains(&MetadataNotice::FieldRenamed {
        field: Field::Label,
        title: "Time".to_string()
    }));
    assert!(v.notices.contains(&MetadataNotice::FieldRenamed {
        field: Field::Status,
        title: "Rating".to_string()
    }));
    // A French project's field is no rename.
    p.project.labels = list("Étiquette", &[(-1, "Aucune"), (1, "Aube")]);
    let v = p.vocabularies();
    assert!(!v.notices.contains(&MetadataNotice::FieldRenamed {
        field: Field::Label,
        title: "Étiquette".to_string()
    }));
    assert_eq!(v.tags[0].details, "W-from Étiquette");
}

/// A label or rung with no name is named with its number, in the writer's language.
#[test]
fn a_value_with_no_name_is_named_with_its_number() {
    let (mut p, [_, a, _, _, _, _]) = novel();
    p.project.labels = list("Label", &[(-1, "No Label"), (5, "  ")]);
    p.project.statuses = list("Status", &[(-1, "No Status"), (2, "")]);
    p.label(a, 5);
    let v = p.vocabularies();
    assert_eq!(tag_names(&v), ["W-label 5"]);
    assert_eq!(rung_names(&v), ["W-status 2"]);
    assert!(v.notices.contains(&MetadataNotice::UnnamedValue {
        field: Field::Label,
        id: 5
    }));
    assert!(v.notices.contains(&MetadataNotice::UnnamedValue {
        field: Field::Status,
        id: 2
    }));
}

/// A label named like a story-bible tag keeps its words, marked as a label; a second
/// clash takes a number (D4-9).
#[test]
fn a_label_named_like_a_story_bible_tag_is_told_apart() {
    let (mut p, [_, a, b, _, _, _]) = novel();
    p.project.labels = list(
        "Label",
        &[
            (-1, "No Label"),
            (1, "Locations"),
            (2, "locations W-(label)"),
        ],
    );
    p.label(a, 1);
    p.label(b, 2);
    let plan = p.plan();
    let v = vocabularies(&p.project, &plan, &["LOCATIONS".to_string()], &words());
    assert_eq!(
        tag_names(&v),
        ["Locations W-(label)", "locations W-(label) W-(label)"]
    );
    let v = vocabularies(
        &p.project,
        &plan,
        &["Locations".to_string(), "Locations W-(label)".to_string()],
        &words(),
    );
    assert_eq!(v.tags[0].name, "Locations W-(label) (2)");
    assert!(
        v.notices
            .iter()
            .any(|n| matches!(n, MetadataNotice::TagNameDisambiguated { .. }))
    );
}

// ---------------------------------------------------------------- none (D4-2, X-38)

/// `-1`, absent and dangling ids are no value; so is an entry named like the list's own
/// none, which a Scrivener 1 upgrade leaves beside it. With that signature on the labels,
/// status `0` is the upgrade's none too.
#[test]
fn the_upgrade_signature_collapses_none_on_both_fields() {
    let (mut p, [_, a, b, c, _, _]) = novel();
    p.project.labels = list("Label", &[(-1, "No Label"), (0, " no label "), (1, "Idea")]);
    p.project.statuses = list(
        "Status",
        &[(-1, "No Status"), (0, "N/A"), (1, "To Do"), (2, "Done")],
    );
    for item in [a, b] {
        p.label(item, 0);
        p.status(item, 0);
    }
    p.label(c, 9);
    p.status(c, 1);
    let v = p.vocabularies();
    assert!(v.tags.is_empty(), "{:?}", v.tags);
    assert_eq!(rung_names(&v), ["To Do", "Done"]);
    assert!(v.notices.contains(&MetadataNotice::NoneValueCollapsed {
        field: Field::Label,
        name: " no label ".to_string(),
        items: 2
    }));
    assert!(v.notices.contains(&MetadataNotice::NoneValueCollapsed {
        field: Field::Status,
        name: "N/A".to_string(),
        items: 2
    }));
    let plan = p.plan();
    for (binder, row) in rows_of(&plan, a) {
        assert_eq!(v.row_rungs[binder][row], None);
    }
    for (binder, row) in rows_of(&plan, c) {
        assert_eq!(v.row_tags[binder][row], None, "a dangling label is none");
        assert_eq!(v.row_rungs[binder][row], Some(0));
    }
}

/// Without the upgrade signature, `N/A` is a rung the writer saw, and `0` a real value.
#[test]
fn without_the_signature_status_zero_is_a_rung() {
    let (mut p, [_, a, _, _, _, _]) = novel();
    p.project.labels = list("Label", &[(-1, "No Label"), (0, "Concept")]);
    p.project.statuses = list(
        "Status",
        &[(-1, "No Status"), (0, "N/A"), (1, "First Draft")],
    );
    p.status(a, 0);
    let v = p.vocabularies();
    assert_eq!(rung_names(&v), ["N/A", "First Draft"]);
    assert_eq!(v.ladder[0].category, StatusCategory::Planned);
    let plan = p.plan();
    let (binder, row) = rows_of(&plan, a)[0];
    assert_eq!(v.row_rungs[binder][row], Some(0));
}

// ---------------------------------------------------------------- the ladder (D4-4)

/// Every rung, in the list's order (never the ids'), each kind from the shipped names,
/// else from its place; the unknown ones are said.
#[test]
fn the_ladder_keeps_the_menu_order_and_every_rung() {
    let (mut p, [_, a, _, _, _, _]) = novel();
    p.project.statuses = list(
        "Status",
        &[
            (-1, "No Status"),
            (9, "Relecture ✎"),
            (5, "Final Draft"),
            (2, "To Do"),
            (3, "Relecture ✎✎"),
        ],
    );
    p.status(a, 2);
    let v = p.vocabularies();
    assert_eq!(
        rung_names(&v),
        ["Relecture ✎", "Final Draft", "To Do", "Relecture ✎✎"]
    );
    let kinds: Vec<StatusCategory> = v.ladder.iter().map(|r| r.category.clone()).collect();
    assert_eq!(
        kinds,
        [
            StatusCategory::Planned,
            StatusCategory::Final,
            StatusCategory::Planned,
            StatusCategory::Final,
        ]
    );
    assert!(
        v.notices
            .contains(&MetadataNotice::StatusCategoriesGuessed {
                rungs: vec!["Relecture ✎".to_string(), "Relecture ✎✎".to_string()]
            })
    );
    assert!(
        v.notices
            .contains(&MetadataNotice::StatusesBecameLadder { rungs: 4, rows: 1 })
    );
    assert_eq!(v.ladder[1].status, Some(5));
}

/// A project with no status list starts with Skribisto's default ladder, as a new
/// project does.
#[test]
fn a_project_with_no_status_list_starts_with_the_default_ladder() {
    let (mut p, _) = novel();
    p.project.statuses = list("Status", &[(-1, "No Status")]);
    let v = p.vocabularies();
    assert_eq!(rung_names(&v), ["W-idea", "W-done"]);
    assert!(v.ladder.iter().all(|r| r.status.is_none()));
    assert!(
        v.notices
            .contains(&MetadataNotice::LadderSeeded { rungs: 2 })
    );
    assert!(
        !v.notices
            .iter()
            .any(|n| matches!(n, MetadataNotice::StatusesBecameLadder { .. }))
    );
}

/// A container's status and label go to it and to the first child holding its own text
/// (D4-1, split containers), never to the note holding its document notes.
#[test]
fn a_containers_status_goes_to_every_row_carrying_it() {
    let mut p = P::new();
    p.add(0, ItemKind::DraftFolder, "Draft");
    let part = p.add(1, ItemKind::Folder, "Part One");
    p.writing(part);
    p.facts.items[part as usize].notes = Carries::Yes;
    let sub = p.add(2, ItemKind::Folder, "Chapter 1");
    p.text(3, "Scene");
    p.text(3, "Scene two");
    let _ = sub;
    p.project.statuses = list("Status", &[(-1, "No Status"), (1, "To Do")]);
    p.project.labels = list("Label", &[(-1, "No Label"), (1, "Idea")]);
    p.status(part, 1);
    p.label(part, 1);
    let plan = p.plan();
    let v = vocabularies(&p.project, &plan, &[], &words());
    let carrying = rows_of(&plan, part);
    assert!(carrying.len() >= 2, "{carrying:?}");
    for (binder, row) in &carrying {
        assert_eq!(v.row_rungs[*binder][*row], Some(0));
        assert_eq!(v.row_tags[*binder][*row], Some(0));
    }
    let notes: Vec<usize> = plan
        .manuscript
        .iter()
        .enumerate()
        .filter(|(_, r)| matches!(r.origin, Origin::DocumentNotes { .. }))
        .map(|(i, _)| i)
        .collect();
    assert!(!notes.is_empty());
    for r in notes {
        assert_eq!(v.row_rungs[0][r], None, "the document notes' note");
        assert_eq!(v.row_tags[0][r], None);
    }
}

/// A label or status on an item that has no row (a document template) does not come
/// across, and is said.
#[test]
fn values_on_items_without_a_row_are_said() {
    let (mut p, _) = novel();
    let templates = p.add(0, ItemKind::Folder, "Templates");
    let sheet = p.text(1, "Sheet");
    p.project.template_folder = Some(ItemKey::Number(templates + 100));
    p.project.labels = list("Label", &[(-1, "No Label"), (1, "Idea")]);
    p.project.statuses = list("Status", &[(-1, "No Status"), (1, "To Do")]);
    p.label(sheet, 1);
    p.status(sheet, 1);
    let plan = p.plan();
    assert!(rows_of(&plan, sheet).is_empty());
    let v = vocabularies(&p.project, &plan, &[], &words());
    assert!(v.tags.is_empty(), "no row wears it");
    assert!(v.notices.contains(&MetadataNotice::ValuesWithoutRow {
        field: Field::Label,
        items: 1
    }));
    assert!(v.notices.contains(&MetadataNotice::ValuesWithoutRow {
        field: Field::Status,
        items: 1
    }));
    assert!(
        !v.notices
            .iter()
            .any(|n| matches!(n, MetadataNotice::UnusedLabelsSkipped { .. })),
        "a label a template wears is not unused"
    );
}

/// A row of the Trash wears its label and status: restoring it brings them back.
#[test]
fn a_trashed_row_keeps_its_label_and_status() {
    let (mut p, _) = novel();
    let deleted = p.text(1, "Deleted scene");
    p.project.labels = list("Label", &[(-1, "No Label"), (1, "Idea")]);
    p.project.statuses = list("Status", &[(-1, "No Status"), (1, "To Do")]);
    p.label(deleted, 1);
    p.status(deleted, 1);
    let plan = p.plan();
    let v = vocabularies(&p.project, &plan, &[], &words());
    let rows = rows_of(&plan, deleted);
    assert_eq!(rows.len(), 1);
    let (binder, row) = rows[0];
    assert_eq!(binder, 1);
    assert!(!plan.notes[row].activated);
    assert_eq!(v.row_tags[binder][row], Some(0));
    assert_eq!(v.row_rungs[binder][row], Some(0));
}

// ---------------------------------------------------------------- proposals (D4-15)

/// The title the settings state, a placeholder refused and said, else the folder's.
#[test]
fn the_title_is_what_the_settings_state_else_the_folders_name() {
    let (mut p, _) = novel();
    let (proposals, refused) = propose(&p.project, &p.facts);
    assert_eq!(proposals.title.title, "Novel");
    assert_eq!(proposals.title.source, TitleSource::FolderName);
    assert!(refused.is_empty());

    p.project.compile.title = Some("<$template_projectName>".to_string());
    p.project.properties.title = Some("  The   Keepers ".to_string());
    let (proposals, refused) = propose(&p.project, &p.facts);
    assert_eq!(proposals.title.title, "The Keepers");
    assert_eq!(proposals.title.source, TitleSource::ProjectProperties);
    assert_eq!(refused, [Refused::Title(TitleSource::CompileSettings)]);

    p.project.compile.title = Some("Les Veilleurs".to_string());
    let (proposals, _) = propose(&p.project, &p.facts);
    assert_eq!(proposals.title.title, "Les Veilleurs");
    assert_eq!(proposals.title.source, TitleSource::CompileSettings);

    p.project.compile.title = None;
    p.project.properties.title = Some("<Proposal Title>".to_string());
    p.project.source.template_title = Some("Starter".to_string());
    let (proposals, refused) = propose(&p.project, &p.facts);
    assert_eq!(proposals.title.title, "Starter");
    assert_eq!(proposals.title.source, TitleSource::Template);
    assert_eq!(refused, [Refused::Title(TitleSource::ProjectProperties)]);
}

/// Each author with no role or the author's role, the sorting form written as it prints;
/// else the forename and surname; else the format 1.x properties. Never a placeholder.
#[test]
fn the_authors_are_the_ones_the_project_names() {
    let (mut p, _) = novel();
    assert_eq!(propose(&p.project, &p.facts).0.author, None);

    let author = |name: &str, role: Option<&str>, file_as: Option<&str>| CompileAuthor {
        name: name.to_string(),
        role: role.map(str::to_string),
        file_as: file_as.map(str::to_string),
    };
    p.project.compile.surname = Some("Hollest".to_string());
    p.project.compile.forename = Some("Oskel".to_string());
    p.project.compile.authors = vec![
        author("Hollest, Oskel", None, None),
        author("Sam Roe", Some("edt"), None),
        author("Doe, Jane", Some("aut"), Some("Doe, Jane")),
    ];
    let (proposals, _) = propose(&p.project, &p.facts);
    assert_eq!(
        proposals.author,
        Some(AuthorProposal {
            names: vec!["Oskel Hollest".to_string(), "Doe, Jane".to_string()],
            source: AuthorSource::CompileSettings,
        })
    );

    p.project.compile.authors = vec![author("<$template_author>", None, None)];
    let (proposals, refused) = propose(&p.project, &p.facts);
    assert_eq!(
        proposals.author.map(|a| a.names),
        Some(vec!["Oskel Hollest".to_string()]),
        "the forename and surname, when no author is words"
    );
    assert_eq!(refused, [Refused::Author(AuthorSource::CompileSettings)]);

    p.project.compile = CompileFacts::default();
    p.project.properties.first_name = Some("Mara".to_string());
    p.project.properties.last_name = Some("Holt".to_string());
    assert_eq!(
        propose(&p.project, &p.facts).0.author,
        Some(AuthorProposal {
            names: vec!["Mara Holt".to_string()],
            source: AuthorSource::ProjectProperties,
        })
    );
    p.project.properties.full_name = Some("M. Holt".to_string());
    assert_eq!(
        propose(&p.project, &p.facts).0.author.map(|a| a.names),
        Some(vec!["M. Holt".to_string()])
    );
}

/// A compile language the writer changed, else the Draft's script, else nothing.
#[test]
fn the_language_is_proposed_only_when_the_project_says_it() {
    let (mut p, [_, a, _, _, note, _]) = novel();
    assert_eq!(propose(&p.project, &p.facts).0.language, None);
    p.project.compile.languages = vec![CompileLanguage {
        file: CompileFile::Xml,
        value: "en".to_string(),
    }];
    assert_eq!(
        propose(&p.project, &p.facts).0.language,
        None,
        "the language Scrivener ships"
    );
    // Research in Chinese names nothing; the Draft in Japanese does.
    p.facts.items[note as usize].text.letters.han = 900;
    assert_eq!(propose(&p.project, &p.facts).0.language, None);
    p.facts.items[a as usize].text.letters.han = 400;
    p.facts.items[a as usize].text.letters.kana = 300;
    p.facts.items[a as usize].text.letters.latin = 100;
    assert_eq!(
        propose(&p.project, &p.facts).0.language,
        Some(LanguageProposal {
            tag: "ja".to_string(),
            source: LanguageSource::Script,
        })
    );
    p.project.compile.languages.push(CompileLanguage {
        file: CompileFile::Plist,
        value: "fr".to_string(),
    });
    assert_eq!(
        propose(&p.project, &p.facts).0.language,
        Some(LanguageProposal {
            tag: "fr".to_string(),
            source: LanguageSource::CompileSettings,
        }),
        "what the writer set wins over the script"
    );
}

// ---------------------------------------------------------------- compile formats

/// The styles the last-used format leaves out, when the project holds it; else what any
/// of the project's own formats leaves out.
#[test]
fn the_format_last_compiled_with_decides_what_was_left_out() {
    let format = |id: &str, deleted: &[&str], raw: &[&str]| CompileFormat {
        id: id.to_string(),
        deleted_styles: deleted.iter().map(|s| s.to_string()).collect(),
        raw_styles: raw.iter().map(|s| s.to_string()).collect(),
    };
    let mut compile = CompileFacts {
        formats: vec![
            format("A", &["Notes to self"], &[]),
            format("B", &["Mac-Only Text"], &["Raw LaTeX"]),
        ],
        ..CompileFacts::default()
    };
    assert_eq!(
        compiled_away(&compile),
        (
            vec!["Mac-Only Text".to_string(), "Notes to self".to_string()],
            vec!["Raw LaTeX".to_string()]
        )
    );
    compile.last_format = Some("A".to_string());
    assert_eq!(
        compiled_away(&compile),
        (vec!["Notes to self".to_string()], Vec::new())
    );
    compile.last_format = Some("manuscript-courier".to_string());
    assert_eq!(compiled_away(&compile).0.len(), 2, "a built-in format");
}

/// Letters of the Draft are counted over its whole subtree only.
#[test]
fn only_the_drafts_letters_decide_the_script() {
    let (mut p, [chapter, a, _, _, note, draft]) = novel();
    p.facts.items[a as usize].text.letters.hebrew = 10;
    p.facts.items[chapter as usize].text.letters.hebrew = 5;
    p.facts.items[draft as usize].text.letters.latin = 1;
    p.facts.items[note as usize].text.letters.latin = 1000;
    let letters = draft_letters(&p.project, &p.facts);
    assert_eq!(letters.hebrew, 15);
    assert_eq!(letters.latin, 1);
}

// ---------------------------------------------------------------- hostile costs

/// Thousands of labels sharing one name, each worn by its own document: each tag is told
/// apart from the others in a moment, never by counting up past every name taken before
/// it, and each row finds its label without searching the list.
#[test]
fn many_labels_of_one_name_are_named_in_linear_time() {
    const LABELS: i64 = 20_000;
    let mut p = P::new();
    p.add(0, ItemKind::DraftFolder, "Draft");
    let mut entries = vec![(-1, "No Label".to_string())];
    for id in 0..LABELS {
        let item = p.text(1, "Scene");
        p.label(item, id);
        entries.push((id, "Same".to_string()));
    }
    let entries: Vec<(i64, &str)> = entries.iter().map(|(i, n)| (*i, n.as_str())).collect();
    p.project.labels = list("Label", &entries);
    let plan = p.plan();
    let started = std::time::Instant::now();
    let v = vocabularies(&p.project, &plan, &[], &words());
    let seconds = started.elapsed().as_secs_f64();
    assert_eq!(v.tags.len(), LABELS as usize);
    let mut names: Vec<String> = v.tags.iter().map(|t| folded(&t.name)).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), LABELS as usize, "every tag has its own name");
    assert!(seconds < 10.0, "{seconds:.1} s");
}

/// A compile listing a hundred thousand authors proposes a byline in a moment: each name
/// is compared once, and no more are read than any book prints.
#[test]
fn a_hundred_thousand_authors_are_read_in_linear_time() {
    let (mut p, _) = novel();
    p.project.compile.authors = (0..100_000)
        .map(|i| CompileAuthor {
            name: format!("Author {i}"),
            role: None,
            file_as: None,
        })
        .collect();
    let started = std::time::Instant::now();
    let (proposals, _) = propose(&p.project, &p.facts);
    let seconds = started.elapsed().as_secs_f64();
    let names = proposals.author.map(|a| a.names).unwrap_or_default();
    assert!(!names.is_empty());
    assert!(names.len() <= 100, "{} names", names.len());
    assert_eq!(names[0], "Author 0");
    assert!(seconds < 10.0, "{seconds:.1} s");
}

// ---------------------------------------------------------------- placeholders

/// A placeholder is a code Scrivener fills in (`<$author>`, `<$n>`) or a whole title in
/// angle brackets; a title that merely holds `<`, `$` or `<$` keeps its words.
#[test]
fn only_a_code_or_a_bracketed_title_is_a_placeholder() {
    for placeholder in [
        "<$template_projectName>",
        "<$fullname>",
        "Draft <$n> of the book",
        "<$img:Cover.png>",
        "<Proposal Title>",
        " <Title> ",
    ] {
        assert!(is_placeholder(placeholder), "{placeholder}");
    }
    for title in [
        "Deals under <$5",
        "Love <3",
        "$100 and a <b",
        "A < B > C",
        "<>",
        "The $ Game",
    ] {
        assert!(!is_placeholder(title), "{title}");
    }
}
