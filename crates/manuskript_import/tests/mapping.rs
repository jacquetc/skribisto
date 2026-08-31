// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the mapper makes of a project: the shape it infers, the vocabularies it
//! mints, and everything it attaches to a row.

use std::sync::atomic::AtomicBool;

use common::entities::{BinderItemRole as Role, BinderItemSubRole as SubRole, ContentRole};
use manuskript_import::map::{self, Mapped, Names};
use manuskript_import::model::*;

fn names() -> Names {
    Names {
        manuscript_binder: "Manuscript".into(),
        story_bible_binder: "Story bible".into(),
        characters_group: "Characters".into(),
        world_group: "World".into(),
        plots_group: "Plots".into(),
        project_info_note: "Project information".into(),
        summary_note: "Summary".into(),
        importance: ["Minor".into(), "Secondary".into(), "Main".into()],
    }
}

fn build(project: &Project) -> Mapped {
    map::build_bundle(project, &names(), &|_, _| {}, &AtomicBool::new(false))
}

fn scene(id: &str, title: &str, text: &str) -> OutlineItem {
    OutlineItem {
        id: Some(id.into()),
        title: title.into(),
        kind: OutlineKind::Text,
        text: text.into(),
        ..OutlineItem::default()
    }
}

fn folder(id: &str, title: &str, children: Vec<OutlineItem>) -> OutlineItem {
    OutlineItem {
        id: Some(id.into()),
        title: title.into(),
        kind: OutlineKind::Folder,
        children,
        ..OutlineItem::default()
    }
}

fn project_with(outline: Vec<OutlineItem>) -> Project {
    Project {
        info: Info {
            title: "A Novel".into(),
            ..Info::default()
        },
        outline,
        ..Project::default()
    }
}

/// The manuscript binder's rows, in order.
fn manuscript(mapped: &Mapped) -> Vec<(Role, SubRole, String)> {
    mapped.bundle.binders[0]
        .items
        .iter()
        .map(|i| {
            (
                i.item.role.clone(),
                i.item.sub_role.clone(),
                i.item.title.clone(),
            )
        })
        .collect()
}

fn find<'a>(mapped: &'a Mapped, binder: usize, title: &str) -> &'a skrib_format::BundledItem {
    mapped.bundle.binders[binder]
        .items
        .iter()
        .find(|i| i.item.title == title)
        .unwrap_or_else(|| panic!("no row titled '{title}'"))
}

fn prose(item: &skrib_format::BundledItem, role: ContentRole) -> String {
    let reference = item
        .item
        .prose_refs
        .iter()
        .find(|p| p.role == role)
        .unwrap_or_else(|| panic!("'{}' has no {role:?}", item.item.title));
    item.prose
        .get(&reference.file_id)
        .cloned()
        .unwrap_or_default()
}

fn inline(item: &skrib_format::BundledItem, role: ContentRole) -> String {
    item.item
        .inline_contents
        .iter()
        .find(|c| c.role == role)
        .map(|c| c.text.clone())
        .unwrap_or_else(|| panic!("'{}' has no inline {role:?}", item.item.title))
}

// ── Structure ───────────────────────────────────────────────────────────────

/// Manuskript has no Book. Pace planning, analysis and every export scope are
/// Book-shaped, so one is synthesised and closed.
#[test]
fn a_book_is_synthesised_around_the_whole_outline_and_closed_at_the_end() {
    let mapped = build(&project_with(vec![scene("1", "Opening", "Words.")]));
    let rows = manuscript(&mapped);
    assert_eq!(
        rows.first().map(|r| (r.0.clone(), r.1.clone())),
        Some((Role::Folder, SubRole::Book))
    );
    assert_eq!(rows[0].2, "A Novel");
    assert_eq!(rows.last().map(|r| r.1.clone()), Some(SubRole::BookEnd));
    assert_eq!(
        inline(find(&mapped, 0, "A Novel"), ContentRole::BookTitle),
        "A Novel"
    );
}

/// No folders at all: a flat set of scenes under the book.
#[test]
fn a_flat_project_becomes_scenes_directly_under_the_book() {
    let mapped = build(&project_with(vec![
        scene("1", "One", "a"),
        scene("2", "Two", "b"),
    ]));
    let rows = manuscript(&mapped);
    assert_eq!(
        rows.iter().map(|r| r.1.clone()).collect::<Vec<_>>(),
        [
            SubRole::Book,
            SubRole::Scene,
            SubRole::Scene,
            SubRole::BookEnd
        ]
    );
}

/// One level of folders: they are chapters, not parts. A project of chapters
/// should not arrive with a phantom part layer.
#[test]
fn one_level_of_folders_becomes_chapters() {
    let mapped = build(&project_with(vec![
        folder("1", "Chapter One", vec![scene("2", "Opening", "a")]),
        folder("3", "Chapter Two", vec![scene("4", "Later", "b")]),
    ]));
    let rows = manuscript(&mapped);
    assert_eq!(rows[1].1, SubRole::ChapterScene);
    assert_eq!(rows[3].1, SubRole::ChapterScene);
    assert_eq!(
        inline(find(&mapped, 0, "Chapter One"), ContentRole::ChapterTitle),
        "Chapter One"
    );
}

#[test]
fn two_levels_of_folders_become_parts_and_chapters() {
    let mapped = build(&project_with(vec![folder(
        "1",
        "Jerusalem",
        vec![folder("2", "Chapter One", vec![scene("3", "Opening", "a")])],
    )]));
    let rows = manuscript(&mapped);
    assert_eq!(rows[1].1, SubRole::Part);
    assert_eq!(rows[2].1, SubRole::ChapterScene);
    assert_eq!(rows[3].1, SubRole::Scene);
    assert_eq!(
        inline(find(&mapped, 0, "Jerusalem"), ContentRole::PartTitle),
        "Jerusalem"
    );
}

/// Below the chapter rung the ladder runs out. A deeper folder groups without
/// claiming a level the model does not have, and its scenes still arrive.
#[test]
fn a_folder_below_the_chapter_rung_is_a_plain_grouping_folder() {
    let mapped = build(&project_with(vec![folder(
        "1",
        "Part",
        vec![folder(
            "2",
            "Chapter",
            vec![folder("3", "Group", vec![scene("4", "Deep", "a")])],
        )],
    )]));
    let rows = manuscript(&mapped);
    assert_eq!(rows[1].1, SubRole::Part);
    assert_eq!(rows[2].1, SubRole::ChapterScene);
    assert_eq!(rows[3].1, SubRole::None);
    assert_eq!(rows[3].0, Role::Folder);
    assert_eq!(
        rows[4].1,
        SubRole::Scene,
        "the scene inside it still arrives"
    );
}

#[test]
fn indent_nests_everything_under_the_book() {
    let mapped = build(&project_with(vec![folder(
        "1",
        "Part",
        vec![folder("2", "Chapter", vec![scene("3", "Scene", "a")])],
    )]));
    let indents: Vec<i64> = mapped.bundle.binders[0]
        .items
        .iter()
        .map(|i| i.item.indent)
        .collect();
    assert_eq!(
        indents,
        [0, 1, 2, 3, 1],
        "book, part, chapter, scene, book end"
    );
}

// ── Contents ────────────────────────────────────────────────────────────────

#[test]
fn a_scenes_prose_and_its_two_summaries_arrive() {
    let mut row = scene("1", "Opening", "The first words.");
    row.summary_sentence = "A beginning.".into();
    row.summary_full = "A longer telling of the beginning.".into();
    let mapped = build(&project_with(vec![row]));
    let item = find(&mapped, 0, "Opening");
    assert!(prose(item, ContentRole::SceneText).contains("The first words."));
    let synopsis = prose(item, ContentRole::SynopsisText);
    assert!(synopsis.contains("A beginning."), "{synopsis}");
    assert!(synopsis.contains("A longer telling"), "{synopsis}");
}

#[test]
fn either_summary_alone_is_used_alone() {
    let mut only_sentence = scene("1", "A", "x");
    only_sentence.summary_sentence = "Just the sentence.".into();
    let mapped = build(&project_with(vec![only_sentence]));
    assert_eq!(
        prose(find(&mapped, 0, "A"), ContentRole::SynopsisText).trim(),
        "Just the sentence."
    );
}

/// A folder cannot hold prose in Manuskript and a chapter folder here can, so
/// there is simply nothing to put in it — but its summary still belongs to it.
#[test]
fn a_chapter_folder_carries_its_summary() {
    let mut chapter = folder("1", "Chapter", vec![scene("2", "S", "x")]);
    chapter.summary_full = "What happens here.".into();
    let mapped = build(&project_with(vec![chapter]));
    assert_eq!(
        prose(find(&mapped, 0, "Chapter"), ContentRole::SynopsisText).trim(),
        "What happens here."
    );
}

/// Neither a scene nor a chapter folder may hold note text, so a row's notes
/// become a Note of their own.
#[test]
fn a_rows_notes_become_a_note_that_is_not_part_of_the_book() {
    let mut row = scene("1", "Opening", "Words.");
    row.notes = "Check the date.".into();
    let mapped = build(&project_with(vec![row]));
    let note = find(&mapped, 0, "Opening (notes)");
    assert_eq!(note.item.sub_role, SubRole::Note);
    assert!(prose(note, ContentRole::NoteText).contains("Check the date."));
    assert!(!note.item.is_exportable, "a note is not part of the book");
}

#[test]
fn a_folders_notes_become_its_first_child_and_a_leaf_gets_its_next_sibling() {
    let mut chapter = folder("1", "Chapter", vec![scene("2", "Scene", "x")]);
    chapter.summary_full = "".into();
    chapter.notes = "About the chapter.".into();
    let mapped = build(&project_with(vec![chapter]));
    let titles: Vec<String> = manuscript(&mapped).into_iter().map(|r| r.2).collect();
    assert_eq!(
        titles,
        ["A Novel", "Chapter", "Chapter (notes)", "Scene", ""],
        "the note sits between the chapter and its scenes"
    );
    let indents: Vec<i64> = mapped.bundle.binders[0]
        .items
        .iter()
        .map(|i| i.item.indent)
        .collect();
    assert_eq!(indents[2], 2, "a folder's note is a child of it");
}

// ── The compile flag ────────────────────────────────────────────────────────

/// Only `0` excludes, and it excludes everything beneath it. Flattening this to a
/// per-row boolean exports scenes the writer switched off.
#[test]
fn an_excluded_folder_excludes_every_scene_under_it() {
    let mut part = folder(
        "1",
        "Cut",
        vec![scene("2", "Also cut", "x"), scene("3", "Cut too", "y")],
    );
    part.compile = Some(0);
    for child in &mut part.children {
        child.compile = Some(2);
    }
    let mapped = build(&project_with(vec![
        part,
        folder("4", "Kept", vec![scene("5", "In", "z")]),
    ]));
    assert!(!find(&mapped, 0, "Cut").item.is_exportable);
    assert!(!find(&mapped, 0, "Also cut").item.is_exportable);
    assert!(!find(&mapped, 0, "Cut too").item.is_exportable);
    assert!(find(&mapped, 0, "In").item.is_exportable);
}

// ── Vocabularies ────────────────────────────────────────────────────────────

fn project_with_vocabularies() -> Project {
    let mut row = scene("1", "Opening", "x");
    row.label = Some(2);
    row.status = Some(3);
    row.set_goal = Some(1500);
    Project {
        labels: vec![
            Label {
                name: "Idea".into(),
                color: Some("#ffff00".into()),
            },
            Label {
                name: "Chapter".into(),
                color: Some("#0000ff".into()),
            },
        ],
        statuses: vec!["TODO".into(), "First draft".into(), "Final".into()],
        ..project_with(vec![row])
    }
}

#[test]
fn a_label_becomes_a_tag_of_the_same_name_and_colour() {
    let mapped = build(&project_with_vocabularies());
    let tag = mapped
        .bundle
        .tags
        .iter()
        .find(|t| t.name == "Chapter")
        .expect("a Chapter tag");
    assert_eq!(tag.color, "#0000ff");
    assert!(
        !tag.discoverable,
        "a label is a filing mark, not a story-bible entry"
    );
    assert_eq!(find(&mapped, 0, "Opening").item.tag_ids, [tag.file_id]);
}

/// The names come from the project, not from a preset: a French project keeps its
/// French rungs.
#[test]
fn the_status_ladder_is_the_projects_own_in_its_own_order() {
    let mapped = build(&project_with_vocabularies());
    let names: Vec<&str> = mapped
        .bundle
        .statuses
        .iter()
        .map(|s| s.name.as_str())
        .collect();
    assert_eq!(names, ["TODO", "First draft", "Final"]);
    use common::entities::StatusCategory as C;
    let categories: Vec<C> = mapped
        .bundle
        .statuses
        .iter()
        .map(|s| s.category.clone())
        .collect();
    assert_eq!(categories, [C::Planned, C::Revised, C::Final]);
    let third = mapped.bundle.statuses[2].file_id;
    assert_eq!(find(&mapped, 0, "Opening").item.status_id, Some(third));
}

#[test]
fn a_goal_arrives_as_a_word_target() {
    let mapped = build(&project_with_vocabularies());
    assert_eq!(find(&mapped, 0, "Opening").item.word_count_goal, 1500);
}

/// A vocabulary edited out from under its rows. Nothing is invented, and the
/// writer is told.
#[test]
fn an_index_past_the_end_of_its_vocabulary_is_reported_and_left_unset() {
    let mut row = scene("1", "Opening", "x");
    row.label = Some(9);
    row.status = Some(9);
    let project = Project {
        labels: vec![Label {
            name: "Idea".into(),
            color: None,
        }],
        statuses: vec!["TODO".into()],
        ..project_with(vec![row])
    };
    let mapped = build(&project);
    let item = find(&mapped, 0, "Opening");
    assert!(item.item.tag_ids.is_empty());
    assert!(item.item.status_id.is_none());
    assert_eq!(
        mapped
            .warnings
            .iter()
            .filter(|w| w.contains("Opening"))
            .count(),
        2
    );
}

// ── The story bible ─────────────────────────────────────────────────────────

fn peopled() -> Project {
    let mut row = scene("1", "Opening", "Then {C:0:Peter} spoke of {W:5:Jerusalem}.");
    row.pov = Some("0".into());
    Project {
        characters: vec![
            Character {
                id: Some("0".into()),
                name: "Peter".into(),
                importance: Some(2),
                color: "#ff0000".into(),
                summary_sentence: "A fisherman.".into(),
                motivation: "To be understood".into(),
                infos: vec![("Hometown".into(), "Bethsaida".into())],
                ..Character::default()
            },
            Character {
                id: Some("1".into()),
                name: "Paul".into(),
                importance: Some(1),
                ..Character::default()
            },
        ],
        world: vec![WorldItem {
            id: Some("4".into()),
            name: "Places".into(),
            children: vec![WorldItem {
                id: Some("5".into()),
                name: "Jerusalem".into(),
                description: "The city.".into(),
                ..WorldItem::default()
            }],
            ..WorldItem::default()
        }],
        plots: vec![Plot {
            id: Some("0".into()),
            name: "The rift".into(),
            importance: Some(2),
            characters: vec!["0".into(), "1".into()],
            steps: vec![PlotStep {
                id: Some("9".into()),
                name: "The argument".into(),
                summary: "It boils over.".into(),
                ..PlotStep::default()
            }],
            ..Plot::default()
        }],
        ..project_with(vec![row])
    }
}

/// A group folder is a notes folder, which is what earns it the story-bible card
/// grid and an Overview; a plain grouping folder gets neither.
#[test]
fn each_group_is_a_notes_folder_carrying_a_discoverable_tag() {
    let mapped = build(&peopled());
    for group in ["Characters", "World"] {
        let item = find(&mapped, 1, group);
        assert_eq!(item.item.role, Role::Folder);
        assert_eq!(item.item.sub_role, SubRole::Note, "{group}");
        let tag = mapped
            .bundle
            .tags
            .iter()
            .find(|t| t.file_id == item.item.tag_ids[0])
            .expect("its tag");
        assert!(
            tag.discoverable,
            "{group} entries are what the mention index hunts for"
        );
        assert_eq!(
            tag.creates_in,
            Some(item.item.file_id),
            "new notes are filed here"
        );
    }
    // A plot's name is a label for a thread, not a word the prose contains.
    let plots = find(&mapped, 1, "Plots");
    let tag = mapped
        .bundle
        .tags
        .iter()
        .find(|t| t.file_id == plots.item.tag_ids[0])
        .expect("its tag");
    assert!(!tag.discoverable);
}

#[test]
fn a_character_becomes_a_note_with_its_sheet_under_headings() {
    let mapped = build(&peopled());
    let peter = find(&mapped, 1, "Peter");
    assert_eq!(peter.item.sub_role, SubRole::Note);
    assert_eq!(
        peter.item.label, "Main",
        "importance becomes the row's label"
    );
    assert_eq!(
        prose(peter, ContentRole::SynopsisText).trim(),
        "A fisherman.",
        "the one-line summary is the synopsis"
    );
    let body = prose(peter, ContentRole::NoteText);
    assert!(body.contains("Motivation"), "{body}");
    assert!(body.contains("To be understood"), "{body}");
    assert!(
        body.contains("Hometown"),
        "a field the writer added: {body}"
    );
    assert!(body.contains("Bethsaida"), "{body}");
}

#[test]
fn a_world_entry_with_children_is_a_folder_and_a_leaf_is_a_note() {
    let mapped = build(&peopled());
    assert_eq!(find(&mapped, 1, "Places").item.role, Role::Folder);
    let jerusalem = find(&mapped, 1, "Jerusalem");
    assert_eq!(jerusalem.item.role, Role::Item);
    assert!(prose(jerusalem, ContentRole::NoteText).contains("The city."));
}

#[test]
fn a_plot_links_to_its_characters_and_keeps_its_beats_as_children() {
    let mapped = build(&peopled());
    let plot = find(&mapped, 1, "The rift");
    let peter = find(&mapped, 1, "Peter").item.file_id;
    let paul = find(&mapped, 1, "Paul").item.file_id;
    assert!(plot.item.reference_ids.contains(&peter));
    assert!(plot.item.reference_ids.contains(&paul));
    assert_eq!(plot.item.role, Role::Folder, "a plot with beats holds them");
    let step = find(&mapped, 1, "The argument");
    assert!(step.item.indent > plot.item.indent);
}

// ── Links and point of view ─────────────────────────────────────────────────

#[test]
fn a_point_of_view_names_the_character_note_it_is_told_through() {
    let mapped = build(&peopled());
    let peter = find(&mapped, 1, "Peter").item.file_id;
    assert_eq!(find(&mapped, 0, "Opening").item.point_of_view_ids, [peter]);
}

/// The marker leaves the prose as the words it stood for, and becomes a link.
#[test]
fn an_inline_reference_becomes_readable_words_and_a_link() {
    let mapped = build(&peopled());
    let opening = find(&mapped, 0, "Opening");
    let text = prose(opening, ContentRole::SceneText);
    assert!(text.contains("Peter"), "{text}");
    assert!(text.contains("Jerusalem"), "{text}");
    assert!(!text.contains("{C:"), "the marker itself is gone: {text}");
    let peter = find(&mapped, 1, "Peter").item.file_id;
    let jerusalem = find(&mapped, 1, "Jerusalem").item.file_id;
    assert!(opening.item.reference_ids.contains(&peter));
    assert!(opening.item.reference_ids.contains(&jerusalem));
}

/// A row may reference one that has not been emitted yet, so the link is resolved
/// in a second pass over the finished binder.
#[test]
fn a_reference_to_a_later_row_still_resolves() {
    let first = scene("1", "First", "See {T:2:the ending}.");
    let last = scene("2", "The ending", "Here.");
    let mapped = build(&project_with(vec![first, last]));
    let ending = find(&mapped, 0, "The ending").item.file_id;
    let opening = find(&mapped, 0, "First");
    assert_eq!(opening.item.reference_ids, [ending]);
    assert!(prose(opening, ContentRole::SceneText).contains("The ending"));
}

// ── Project paperwork ───────────────────────────────────────────────────────

#[test]
fn the_work_carries_the_title_the_author_and_the_language() {
    let project = Project {
        info: Info {
            title: "A Novel".into(),
            author: "Luke".into(),
            subtitle: "A subtitle".into(),
            ..Info::default()
        },
        settings: Settings {
            dict: Some("en_US".into()),
            ..Settings::default()
        },
        ..project_with(vec![scene("1", "A", "x")])
    };
    let mapped = build(&project);
    let work = &mapped.bundle.manifest.work;
    assert_eq!(work.title, "A Novel");
    assert_eq!(work.author_name, "Luke");
    assert_eq!(
        work.dict_language,
        ["en-US"],
        "the locale is respelled, not dropped"
    );
    assert!(
        !work.chapter_flat,
        "Manuskript chapters are folders holding scenes"
    );
    let book = find(&mapped, 0, "A Novel");
    assert_eq!(book.item.sub_title, "A subtitle");
    assert_eq!(inline(book, ContentRole::BookSubtitle), "A subtitle");
}

/// Fields Skribisto has nowhere for are kept rather than dropped.
#[test]
fn the_leftover_project_details_become_a_note_only_when_there_are_any() {
    let bare = build(&project_with(vec![scene("1", "A", "x")]));
    assert!(
        bare.bundle
            .binders
            .get(1)
            .map(|b| b
                .items
                .iter()
                .all(|i| i.item.title != "Project information"))
            .unwrap_or(true)
    );

    let project = Project {
        info: Info {
            title: "A Novel".into(),
            serie: "The Chronicles".into(),
            volume: "2".into(),
            genre: "Fantasy".into(),
            ..Info::default()
        },
        ..project_with(vec![scene("1", "A", "x")])
    };
    let mapped = build(&project);
    let note = find(&mapped, 1, "Project information");
    let body = prose(note, ContentRole::NoteText);
    assert!(body.contains("The Chronicles"), "{body}");
    assert!(body.contains("Fantasy"), "{body}");
}

/// Five nested tellings of one book do not fit one synopsis, so the longest
/// becomes the book's and the ladder is kept whole beside it.
#[test]
fn the_summary_ladder_is_kept_whole_and_its_longest_rung_leads_the_book() {
    let project = Project {
        summary: Summary {
            situation: "A short one.".into(),
            full: "A much longer telling of the whole book, at length.".into(),
            ..Summary::default()
        },
        ..project_with(vec![scene("1", "A", "x")])
    };
    let mapped = build(&project);
    assert!(
        prose(find(&mapped, 0, "A Novel"), ContentRole::SynopsisText)
            .contains("much longer telling")
    );
    let ladder = prose(find(&mapped, 1, "Summary"), ContentRole::NoteText);
    assert!(ladder.contains("Situation"), "{ladder}");
    assert!(ladder.contains("A short one."), "{ladder}");
    assert!(ladder.contains("Full"), "{ladder}");
}

// ── History ─────────────────────────────────────────────────────────────────

#[test]
fn recorded_revisions_become_the_projects_version_history() {
    let project = Project {
        revisions: vec![
            Revision {
                item_id: "1".into(),
                timestamp: 1_455_033_267,
                text: "Older words.".into(),
            },
            Revision {
                item_id: "1".into(),
                timestamp: 1_600_000_000,
                text: "Newer words.".into(),
            },
        ],
        ..project_with(vec![scene("1", "Opening", "Newest words.")])
    };
    let mapped = build(&project);
    assert_eq!(mapped.imported_revisions, 2);
    let log = &mapped.bundle.history;
    assert_eq!(log.entries.len(), 2);
    let uid = find(&mapped, 0, "Opening").item.uid;
    assert!(log.entries.iter().all(|e| e.item_uid == uid));
    assert!(log.entries[0].at < log.entries[1].at, "oldest first");
    assert_eq!(log.blobs.len(), 2);
    for entry in &log.entries {
        assert!(
            log.blobs.contains_key(&entry.hash),
            "every entry has its blob"
        );
    }
}

#[test]
fn a_revision_naming_a_row_that_is_gone_is_reported_rather_than_dropped_quietly() {
    let project = Project {
        revisions: vec![Revision {
            item_id: "999".into(),
            timestamp: 1_455_033_267,
            text: "Orphaned.".into(),
        }],
        ..project_with(vec![scene("1", "Opening", "x")])
    };
    let mapped = build(&project);
    assert_eq!(mapped.imported_revisions, 0);
    assert!(
        mapped
            .warnings
            .iter()
            .any(|w| w.contains("could not be placed")),
        "{:?}",
        mapped.warnings
    );
}

// ── Invariants ──────────────────────────────────────────────────────────────

/// Nothing the mapper builds may violate the writing model, whatever the source
/// looked like.
#[test]
fn every_row_and_every_content_is_valid_by_the_writing_model() {
    let mut project = peopled();
    project.labels = vec![Label {
        name: "Idea".into(),
        color: None,
    }];
    project.statuses = vec!["TODO".into(), "Done".into()];
    project.summary = Summary {
        full: "A book.".into(),
        ..Summary::default()
    };
    project.info.genre = "Fantasy".into();
    let mapped = build(&project);
    for binder in &mapped.bundle.binders {
        for item in &binder.items {
            for content in &item.item.inline_contents {
                assert!(
                    skribisto_model::content_allowed(
                        &item.item.role,
                        &item.item.sub_role,
                        &content.role
                    ),
                    "{:?}/{:?} may not hold {:?} ('{}')",
                    item.item.role,
                    item.item.sub_role,
                    content.role,
                    item.item.title
                );
            }
            for reference in &item.item.prose_refs {
                assert!(
                    skribisto_model::content_allowed(
                        &item.item.role,
                        &item.item.sub_role,
                        &reference.role
                    ),
                    "{:?}/{:?} may not hold {:?} ('{}')",
                    item.item.role,
                    item.item.sub_role,
                    reference.role,
                    item.item.title
                );
            }
        }
    }
}

/// A Manuskript id is re-minted by Manuskript itself on collision, so nothing
/// durable may be built on it. Every row mints its own.
#[test]
fn every_row_carries_a_distinct_uid() {
    let mapped = build(&peopled());
    let mut uids: Vec<uuid::Uuid> = mapped
        .bundle
        .binders
        .iter()
        .flat_map(|b| b.items.iter().map(|i| i.item.uid))
        .collect();
    let total = uids.len();
    assert!(total > 0);
    assert!(
        uids.iter().all(|u| !u.is_nil()),
        "a nil uid was never a valid value"
    );
    uids.sort();
    uids.dedup();
    assert_eq!(uids.len(), total, "two rows share a uid");
}

/// The read floor is the writer's to compute, from the content it is about to
/// commit. Stamping it here as well would be a second answer to the same question.
#[test]
fn the_importer_leaves_the_read_floor_to_the_writer() {
    let mapped = build(&peopled());
    assert!(mapped.bundle.manifest.format_min_read_version.is_none());
    assert_eq!(
        mapped.bundle.manifest.format_version,
        skrib_format::FORMAT_VERSION
    );
}

#[test]
fn an_empty_project_still_makes_a_readable_book() {
    let mapped = build(&project_with(Vec::new()));
    let rows = manuscript(&mapped);
    assert_eq!(rows.len(), 2, "a book and its closing marker");
    assert_eq!(
        mapped.bundle.binders.len(),
        1,
        "no story bible when there is nothing in it"
    );
}

// ── Regressions ─────────────────────────────────────────────────────────────

/// The readers convert prose to Djot at the format boundary, so the mapper must
/// pass it through. Converting again reads Djot as Markdown, where `*x*` is
/// emphasis rather than strong, and every bold run quietly becomes italic.
#[test]
fn the_mapper_does_not_convert_prose_a_second_time() {
    let mut row = scene("1", "Opening", "A *strong* word.");
    row.summary_full = String::new();
    let mapped = build(&project_with(vec![row]));
    let text = prose(find(&mapped, 0, "Opening"), ContentRole::SceneText);
    assert!(text.contains("*strong*"), "still strong: {text}");
    assert!(!text.contains("_strong_"), "demoted to emphasis: {text}");
}

/// A project whose `infos.txt` has no `Title:` still has a name on disk, and
/// arriving as "Imported Manuskript project" throws it away.
#[test]
fn a_project_with_no_title_is_named_after_itself() {
    let project = Project {
        info: Info::default(),
        source_name: "Le Tour du monde".into(),
        outline: vec![scene("1", "A", "x")],
        ..Project::default()
    };
    let mapped = build(&project);
    assert_eq!(mapped.bundle.manifest.work.title, "Le Tour du monde");
    assert!(
        mapped.bundle.binders[0]
            .items
            .iter()
            .any(|i| i.item.title == "Le Tour du monde"),
        "and the Book takes the same name"
    );
}

/// Only when there is nothing else at all.
#[test]
fn a_project_with_no_name_anywhere_still_gets_one() {
    let mapped = build(&Project {
        outline: vec![scene("1", "A", "x")],
        ..Project::default()
    });
    assert_eq!(
        mapped.bundle.manifest.work.title,
        "Imported Manuskript project"
    );
}

/// An item's `status` is a 1-based index into the list **as stored**. A blank row
/// (which a format-0 `status.xml` carries, since its table keeps Manuskript's own
/// empty entries) must be skipped without consuming its number, or every rung
/// below it moves and the rows that wore them land on the wrong one.
#[test]
fn a_blank_rung_does_not_shift_the_ones_below_it() {
    let mut row = scene("1", "Opening", "x");
    row.status = Some(4);
    let project = Project {
        // Position 2 is blank, so "Final" is still the fourth rung.
        statuses: vec!["TODO".into(), "  ".into(), "Draft".into(), "Final".into()],
        ..project_with(vec![row])
    };
    let mapped = build(&project);
    let names: Vec<&str> = mapped
        .bundle
        .statuses
        .iter()
        .map(|s| s.name.as_str())
        .collect();
    assert_eq!(
        names,
        ["TODO", "Draft", "Final"],
        "the blank row is not a rung"
    );

    let final_rung = mapped
        .bundle
        .statuses
        .iter()
        .find(|s| s.name == "Final")
        .expect("Final");
    assert_eq!(
        find(&mapped, 0, "Opening").item.status_id,
        Some(final_rung.file_id),
        "status 4 is still Final, not Draft"
    );

    // The ends of the ladder are its first and last real rungs.
    use common::entities::StatusCategory as C;
    let categories: Vec<C> = mapped
        .bundle
        .statuses
        .iter()
        .map(|s| s.category.clone())
        .collect();
    assert_eq!(categories, [C::Planned, C::Revised, C::Final]);
}
