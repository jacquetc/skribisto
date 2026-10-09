// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The second half of the metadata mapping, end to end, for every generation that writes
//! each kind: keywords, custom metadata, collections, bookmarks and references, targets,
//! the deadline, the auto-complete list, compile replacements, the project's own lines,
//! icons and script mode; the "From Scrivener" folder that lists what has no field; and
//! their round trip through a real store (`load_work`, `save_work`, `load_work`).

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use common::entities::{ContentRole, GoalUnit};
use scrivener_fixtures::{Generation, Platform, Spec, Targets};
use scrivener_import::metadata::{DeadlineReason, Field, MetadataNotice, TargetUnit};
use scrivener_import::structure::{Choices, NoteSection};
use scrivener_import::{Answers, Request, Summary, convert};
use skrib_format::{BundledItem, WorkBundle};

use crate::convert::{instant, item, items, loads, prose, read, survives_the_editor, words};
use crate::support::{options, read as read_project, run, temp, write};

/// Every generation and platform the fixtures write.
const GENERATIONS: [(Generation, Platform); 9] = [
    (Generation::Format10, Platform::Mac),
    (Generation::Format10, Platform::Windows),
    (Generation::Format10, Platform::Linux),
    (Generation::Format15, Platform::Mac),
    (Generation::Format15, Platform::Windows),
    (Generation::Format15, Platform::Linux),
    (Generation::Format20, Platform::Mac),
    (Generation::Format20, Platform::Windows),
    (Generation::IosStub, Platform::Ios),
];

/// Convert `spec` with the Save page left alone and the manuscript in `language`.
fn converted(spec: &Spec, language: Option<&str>) -> (tempfile::TempDir, WorkBundle, Summary) {
    let dir = temp();
    let folder = write(spec, dir.path());
    let out = dir.path().join("novel.skrib");
    let (choices, words, answers, options) =
        (Choices::default(), words(), Answers::default(), options());
    let output = out.to_string_lossy().into_owned();
    let request = Request {
        source: &folder,
        answers: &answers,
        options: &options,
        choices: &choices,
        prose: Default::default(),
        fingerprint: None,
        output: &output,
        overwrite: false,
        words: &words,
        title: None,
        author: None,
        language,
        copy_research_files: false,
        now: instant(),
        today: instant().date_naive(),
    };
    let summary = match convert(&request, &|_, _| {}, &AtomicBool::new(false)) {
        Ok(summary) => summary,
        Err(e) => panic!("the conversion failed: {e}"),
    };
    let bundle = read(&out);
    (dir, bundle, summary)
}

/// The names of the tags a row wears.
fn tags_of(bundle: &WorkBundle, row: &BundledItem) -> Vec<String> {
    row.item
        .tag_ids
        .iter()
        .filter_map(|id| bundle.tags.iter().find(|t| t.file_id == *id))
        .map(|t| t.name.clone())
        .collect()
}

/// The titles of the rows a row refers to.
fn references_of(bundle: &WorkBundle, row: &BundledItem) -> Vec<String> {
    row.item
        .reference_ids
        .iter()
        .filter_map(|id| items(bundle).find(|i| i.item.file_id == *id))
        .map(|i| i.item.title.clone())
        .collect()
}

/// What a reader of a row's note text sees.
fn note_text(row: &BundledItem) -> String {
    match skrib_format::djot_plain_text(&prose(row, ContentRole::NoteText)) {
        Ok((text, _)) => text,
        Err(e) => panic!("'{}': the note does not read: {e}", row.item.title),
    }
}

fn has(summary: &Summary, wanted: &MetadataNotice) -> bool {
    summary.metadata.contains(wanted)
}

fn find<'s>(summary: &'s Summary, key: &str) -> Option<&'s MetadataNotice> {
    summary.metadata.iter().find(|n| n.key() == key)
}

// ── every generation ─────────────────────────────────────────────────────────────

/// A project of each generation with every second-half kind it writes: each comes across
/// where the design puts it, or is listed in the "From Scrivener" folder, or is said.
#[test]
fn every_generation_brings_what_it_writes_of_the_second_half() {
    for (generation, platform) in GENERATIONS {
        let label = format!("{generation:?} {platform:?}");
        let spec = Spec::novel(generation, platform).with_metadata().with_s2();
        let (_dir, bundle, summary) = converted(&spec, Some("en-US"));
        let v3 = generation == Generation::Format20;
        let desktop = generation != Generation::IosStub;
        let x = matches!(generation, Generation::Format10 | Generation::Format15);
        let notes = |title: &str| note_text(item(&bundle, title));

        if !desktop {
            // An iPad binder writes none of the second half's kinds (R1 §8): no folder,
            // nothing said but the draft target, which it writes as zero.
            assert!(
                items(&bundle).all(|i| !i.item.title.starts_with("W-from")),
                "{label}"
            );
            assert!(bundle.dict_words.is_empty() && bundle.paces.is_empty());
            loads(bundle, &_dir.path().join("novel.skrib"));
            continue;
        }

        // Keywords (D4-7): flat tags on the rows that wear them, their place in the list in
        // the details, the one no document wears listed, the template's said.
        let keyword = |name: &str| {
            bundle
                .tags
                .iter()
                .find(|t| t.details.starts_with("W-keyword-details") && t.name.starts_with(name))
                .unwrap_or_else(|| panic!("{label}: no keyword tag {name}"))
        };
        assert_eq!(
            keyword("Mara").details,
            "W-keyword-details People › Mara",
            "{label}"
        );
        assert_eq!(
            keyword("Mara").name,
            "Mara W-(keyword-tag)",
            "{label}: the label Mara holds the name"
        );
        assert_eq!(keyword("Harbour town").color, "#e6cc99", "{label}");
        assert!(
            tags_of(&bundle, item(&bundle, "Night Watch")).contains(&"The inspector".to_string()),
            "{label}"
        );
        assert!(
            tags_of(&bundle, item(&bundle, "The Harbour")).contains(&"Harbour town".to_string()),
            "{label}"
        );
        assert!(
            has(
                &summary,
                &MetadataNotice::UnusedKeywordsSkipped {
                    names: vec!["Unused".to_string()]
                }
            ),
            "{label}: {:?}",
            summary.metadata
        );
        assert!(has(
            &summary,
            &MetadataNotice::KeywordHierarchyFlattened { keywords: 3 }
        ));
        for field in [
            Field::Keyword,
            Field::CustomField,
            Field::Bookmark,
            Field::Target,
        ] {
            assert!(
                has(
                    &summary,
                    &MetadataNotice::ValuesWithoutRow { field, items: 1 }
                ),
                "{label}: the document template's {field:?}"
            );
        }

        // Custom metadata (D4-8): text values in the folder's note, a note-like row's own in
        // its own text, read back exactly.
        let custom = notes("W-note-custom-metadata");
        assert!(custom.contains("Setting"), "{label}: {custom}");
        assert!(
            custom.contains("Manuscript › Part One › Chapter 1 › The Harbour"),
            "{label}: {custom}"
        );
        assert!(custom.contains("Quay: 10:30:45"), "{label}: {custom}");
        let mara = notes("Mara");
        assert!(mara.contains("W-note-custom-metadata"), "{label}: {mara}");
        assert!(mara.contains("Eyes"), "{label}: {mara}");
        assert!(
            mara.contains("Grey\nAt 10:30:45 she said \"no\""),
            "{label}: {mara}"
        );
        assert!(
            custom.contains("W-in-own-note") || !custom.contains("Research › Mara"),
            "{label}: a pointer to the note that holds its own values"
        );
        assert!(has(
            &summary,
            &MetadataNotice::UndefinedCustomField {
                ids: vec!["ghost".to_string()],
                values: 1
            }
        ));
        if v3 {
            assert!(
                tags_of(&bundle, item(&bundle, "Arrival")).contains(&"Platform: Mac".to_string()),
                "{label}"
            );
            let harbour = tags_of(&bundle, item(&bundle, "The Harbour"));
            assert!(
                harbour.contains(&"Platform: Windows".to_string()),
                "{label}"
            );
            assert!(harbour.contains(&"Done".to_string()), "{label}");
            assert!(
                !tags_of(&bundle, item(&bundle, "Arrival")).contains(&"Done".to_string()),
                "an unticked box is no tag"
            );
            assert!(has(
                &summary,
                &MetadataNotice::ListValueInvalid {
                    field: "Platform".to_string(),
                    count: 1
                }
            ));
            assert!(has(
                &summary,
                &MetadataNotice::FieldsBecameTags {
                    fields: 2,
                    tags: 3,
                    rows: 3
                }
            ));
        }

        // Collections (D4-13).
        for title in ["Arrival", "The Harbour"] {
            assert!(
                tags_of(&bundle, item(&bundle, title)).contains(&"To revise".to_string()),
                "{label}: {title}"
            );
        }
        assert!(tags_of(&bundle, item(&bundle, "Mara Holt")).contains(&"Cast".to_string()));
        assert!(has(
            &summary,
            &MetadataNotice::EmptyCollectionsSkipped {
                names: vec!["Empty".to_string()]
            }
        ));
        assert!(has(
            &summary,
            &MetadataNotice::CollectionMembersMissing {
                name: "Templates only".to_string(),
                count: 1
            }
        ));
        assert!(has(
            &summary,
            &MetadataNotice::CollectionOrderLost {
                names: vec!["To revise".to_string()]
            }
        ));
        assert!(
            bundle.tags.iter().all(|t| !t.name.contains("Binder")
                && !t.name.contains("Search Results")
                && !t.name.contains("Templates only")),
            "{label}: no tag for the binder, a search or a collection with no row"
        );
        if v3 {
            let searches = notes("W-note-saved-searches");
            assert!(searches.contains("Settings"), "{label}: {searches}");
            assert!(searches.contains("Quay"), "{label}: {searches}");
            // Scrivener's own tokens are the interface's words, never written as they are.
            assert!(
                searches.contains("W-search-in-custom-metadata (Setting)"),
                "{searches}"
            );
            assert!(!searches.contains("CustomMetaData"), "{searches}");
        }

        // Bookmarks and references (D4-14, X-35): a link where Skribisto shows one, every
        // other listed under its document, the dangling one counted.
        assert_eq!(
            references_of(&bundle, item(&bundle, "Mara")),
            ["Ideas"],
            "{label}: a note to a note is a Link"
        );
        assert_eq!(
            references_of(&bundle, item(&bundle, "Arrival")),
            ["The Inspector"],
            "{label}: a scene to a story-bible entry is its Cast"
        );
        let bookmarks = notes("W-note-bookmarks");
        assert!(
            bookmarks.contains("Research › Mara"),
            "{label}: {bookmarks}"
        );
        assert!(bookmarks.contains("Tide chart"), "{label}: {bookmarks}");
        assert!(
            bookmarks.contains("Templates › Scene sheet"),
            "{label}: {bookmarks}"
        );
        assert!(
            !bookmarks.contains("Manuscript › Part One › Chapter 1 › Arrival\nManuscript › Part One › Chapter 1 › Arrival"),
            "a bookmark to itself is dropped"
        );
        if x {
            assert!(
                bookmarks.contains("Where she grew up"),
                "{label}: {bookmarks}"
            );
        }
        let arrival_bookmarks = prose(item(&bundle, "W-note-bookmarks"), ContentRole::NoteText);
        assert!(
            arrival_bookmarks.contains("](https://example.org/tides)"),
            "{label}: a web bookmark is a link: {arrival_bookmarks}"
        );
        assert!(
            mara.contains("Map"),
            "{label}: the note's own bookmarks: {mara}"
        );
        assert!(has(
            &summary,
            &MetadataNotice::BookmarksUnresolved { count: 1 }
        ));
        assert!(has(
            &summary,
            &MetadataNotice::BookmarkEndsWithoutRow { count: 1 }
        ));
        assert!(has(
            &summary,
            &MetadataNotice::BookmarksBecameLinks { count: 2 }
        ));
        let project_links = notes("W-note-project-bookmarks");
        assert!(
            project_links.contains("Notes › Ideas"),
            "{label}: {project_links}"
        );
        if v3 {
            assert!(project_links.contains("Site"), "{label}: {project_links}");
        }

        // Targets (D4-11, X-30): goals on the prose they measure, the draft's on the book,
        // notes' listed.
        assert_eq!(
            item(&bundle, "Arrival").item.word_count_goal,
            1500,
            "{label}"
        );
        assert_eq!(item(&bundle, "The Harbour").item.word_count_goal, 900);
        let book = items(&bundle)
            .find(|i| i.item.sub_role == common::entities::BinderItemSubRole::Book)
            .unwrap_or_else(|| panic!("{label}: no book"));
        assert_eq!(book.item.word_count_goal, 80_000, "{label}");
        assert_eq!(item(&bundle, "Mara").item.word_count_goal, 0, "{label}");
        let targets = notes("W-note-targets");
        assert!(
            targets.contains("Research › Mara\n600 (W-unit-words)"),
            "{targets}"
        );
        assert!(
            targets.contains("Notes › Ideas\n2000 (W-unit-characters)"),
            "{targets}"
        );
        assert_eq!(bundle.manifest.work.goal_unit, GoalUnit::Words);
        assert!(has(&summary, &MetadataNotice::TargetsOnNotes { count: 2 }));
        assert!(has(
            &summary,
            &MetadataNotice::DraftTargetKept {
                value: 80_000,
                unit: TargetUnit::Words
            }
        ));

        // The deadline (D4-12): a plan from the import's day to the deadline's, the deadline
        // itself a day off as Scrivener's calculator kept it.
        assert_eq!(bundle.paces.len(), 1, "{label}");
        let pace = &bundle.paces[0];
        assert_eq!(pace.book_item, Some(book.item.file_id));
        assert_eq!(pace.start_date, "2026-09-21T00:00:00+00:00", "{label}");
        assert_eq!(pace.end_date, "2099-12-01T00:00:00+00:00");
        assert_eq!(pace.weekday_mask, 127);
        assert!(pace.active);
        assert_eq!(pace.holidays.len(), 1);
        assert_eq!(pace.holidays[0].label, "W-deadline-day");
        assert_eq!(pace.holidays[0].start_date, pace.end_date);
        assert!(matches!(
            find(&summary, "scrivener-metadata-pace-planned"),
            Some(MetadataNotice::PacePlanned { day_off: true, .. })
        ));

        // The project's own lines (D4-15, D4-20).
        let project = notes("W-note-project");
        assert!(
            project.contains("W-session-target\n1000 (W-unit-words)"),
            "{project}"
        );
        assert!(project.contains("W-deadline\n2099-12-01"), "{project}");
        assert!(
            project.contains("W-abbreviated-title\nKeepers"),
            "{label}: {project}"
        );
        if v3 {
            assert!(project.contains("W-subjects\nSea W+ Lights"), "{project}");
            assert!(project.contains("W-company\nHarbour Press"), "{project}");
        }

        // The auto-complete list (D4-17) and compile replacements (D4-18).
        if v3 {
            let mut words: Vec<&str> = bundle.dict_words.iter().map(|w| w.word.as_str()).collect();
            words.sort_unstable();
            assert_eq!(words, ["Harbour", "LUKE", "Mara"], "{label}");
            assert_eq!(
                bundle.manifest.work.dict_word_ids.len(),
                3,
                "the project names them"
            );
            assert!(notes("W-note-auto-complete").contains("Harbour master"));
            let replacements = notes("W-note-replacements");
            assert!(
                replacements.contains(r"\s+\[@ → [@ (W-replacement-regex W+ W-replacement-off)"),
                "{replacements}"
            );
            assert!(replacements.contains("{Name} → Mara"), "{replacements}");
        } else {
            assert!(bundle.dict_words.is_empty());
        }
        if x && platform == Platform::Mac {
            assert!(notes("W-note-replacements").contains("!fig($@) → <$n:figure:$@>"));
        }

        // Icons and script mode (D4-16, D4-19).
        assert!(matches!(
            find(&summary, "scrivener-metadata-icons-dropped"),
            Some(MetadataNotice::IconsDropped { rows: 3, names })
                if names.first().map(String::as_str) == Some("Book (Blue Book)")
        ));
        if matches!(generation, Generation::Format15 | Generation::Format20) {
            assert!(has(&summary, &MetadataNotice::ScriptModeRows { rows: 1 }));
        }

        // The point of view (D4-6): a hint, nothing linked.
        assert!(
            has(
                &summary,
                &MetadataNotice::PovHint {
                    values: vec!["Mara".to_string()],
                    matches: vec!["Mara Holt".to_string()]
                }
            ),
            "{label}: {:?}",
            summary.metadata
        );
        assert!(items(&bundle).all(|i| i.item.point_of_view_ids.is_empty()));

        // The folder: after the writer's own notes, before the Trash, never exported.
        let titles: Vec<&str> = bundle.binders[1]
            .items
            .iter()
            .map(|i| i.item.title.as_str())
            .collect();
        let folder = titles
            .iter()
            .position(|t| *t == "W-from-scrivener")
            .unwrap_or_else(|| panic!("{label}: no folder in {titles:?}"));
        assert!(
            titles[folder + 1..]
                .iter()
                .all(|t| t.starts_with("W-note-") || *t == "Deleted scene"),
            "{label}: {titles:?}"
        );
        assert!(
            bundle.binders[1].items[folder..]
                .iter()
                .all(|i| !i.item.is_exportable)
        );
        let listed: Vec<NoteSection> = summary
            .metadata
            .iter()
            .filter_map(|n| match n {
                MetadataNotice::ListedInImportNote { section, .. } => Some(*section),
                _ => None,
            })
            .collect();
        assert!(listed.contains(&NoteSection::CustomMetadata), "{label}");
        assert!(listed.contains(&NoteSection::Bookmarks), "{label}");

        survives_the_editor(&bundle);
        loads(bundle, &_dir.path().join("novel.skrib"));
    }
}

/// The review shows the folder the import writes: the analysis's plan and the written
/// notes binder agree row for row.
#[test]
fn the_review_and_the_import_agree_on_the_folder() {
    let spec = Spec::novel(Generation::Format20, Platform::Mac)
        .with_metadata()
        .with_s2();
    let dir = temp();
    let folder = write(&spec, dir.path());
    let read = read_project(run(&folder));
    let planned: Vec<String> = read
        .structure
        .notes
        .iter()
        .map(|row| format!("{} {:?}", row.depth, row.origin))
        .collect();
    let (_d, _bundle, summary) = converted(&spec, Some("en-US"));
    let written: Vec<String> = summary
        .rows
        .iter()
        .filter(|r| r.at.binder == scrivener_import::structure::BinderKind::Notes)
        .map(|r| {
            let row = &read.structure.notes[r.at.row as usize];
            format!("{} {:?}", row.depth, r.origin)
        })
        .collect();
    assert_eq!(planned, written);
    assert!(
        planned.iter().any(|r| r.contains("FromScrivenerNote")),
        "{planned:?}"
    );
}

// ── the deadline and the unit ───────────────────────────────────────────────────

/// A deadline behind the import makes no plan: its day is listed and said.
#[test]
fn a_past_deadline_is_listed_not_planned() {
    let mut spec = Spec::novel(Generation::Format15, Platform::Windows);
    spec.targets = Targets {
        deadline: Some("2012-07-01 23:30:00 -0700".to_string()),
        ignore_deadline: false,
        ..Targets::default()
    };
    let (_dir, bundle, summary) = converted(&spec, None);
    assert!(bundle.paces.is_empty());
    assert!(has(
        &summary,
        &MetadataNotice::DeadlineNotPlanned {
            reason: DeadlineReason::Passed,
            day: Some(1_341_100_800)
        }
    ));
    let project = note_text(item(&bundle, "W-note-project"));
    assert!(
        project.contains("W-deadline\n2012-07-01"),
        "the writer's own calendar day: {project}"
    );
}

/// A checkbox ticked by default (`Default="Yes"`, as Scrivener 3.5.2 for Mac writes it, M9)
/// ticks every document that writes no value for it; an explicit `No` stays unticked, and
/// the default is nothing to report.
#[test]
fn a_checkbox_ticked_by_default_ticks_what_writes_no_value() {
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac)
        .with_metadata()
        .with_s2();
    for field in &mut spec.typed_fields {
        if field.kind == "Checkbox" {
            field.default = Some("Yes".to_string());
        }
    }
    let (_dir, bundle, summary) = converted(&spec, Some("en-US"));
    let done = |title: &str| tags_of(&bundle, item(&bundle, title)).contains(&"Done".to_string());
    assert!(done("The Harbour"), "ticked one by one");
    assert!(done("Night Watch"), "no value: the default");
    assert!(done("Mara"), "a research note takes the default too");
    assert!(!done("Arrival"), "an explicit No");
    assert!(
        !summary
            .metadata
            .iter()
            .any(|n| matches!(n, MetadataNotice::CheckboxDefaultUnknown { .. })),
        "{:?}",
        summary.metadata
    );
}

/// A session target Scrivener calculated from the deadline is listed as calculated and
/// said so (M19); one the writer typed keeps its line.
#[test]
fn a_calculated_session_target_is_listed_and_said_as_calculated() {
    for calculated in [true, false] {
        let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
        spec.targets.determined_from_deadline = calculated;
        let (_dir, bundle, summary) = converted(&spec, None);
        assert!(
            has(
                &summary,
                &MetadataNotice::SessionTargetNotImported {
                    value: 1_000,
                    unit: TargetUnit::Words,
                    calculated,
                }
            ),
            "{:?}",
            summary.metadata
        );
        let project = note_text(item(&bundle, "W-note-project"));
        let label = if calculated {
            "W-session-target-calculated\n1000"
        } else {
            "W-session-target\n1000"
        };
        assert!(project.contains(label), "{project}");
    }
}

/// Writing days set to some days only: the Pace keeps them (Scrivener numbers the days
/// from Sunday, so `1-2, 4-6` is Sunday, Monday, Wednesday, Thursday and Friday), nothing
/// is said, and the import note does not list them again.
#[test]
fn partial_writing_days_are_the_paces_writing_days() {
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    spec.targets = Targets {
        deadline: Some("2099-01-01 10:00:00 +0000".to_string()),
        ignore_deadline: false,
        writing_days: "1-2, 4-6".to_string(),
        can_write_on_deadline: true,
        ..Targets::default()
    };
    let (_dir, bundle, summary) = converted(&spec, None);
    assert_eq!(bundle.paces.len(), 1);
    assert_eq!(bundle.paces[0].weekday_mask, 64 + 1 + 4 + 8 + 16);
    assert!(
        bundle.paces[0].holidays.is_empty(),
        "writing allowed on the day"
    );
    assert!(
        !summary
            .metadata
            .iter()
            .any(|n| matches!(n, MetadataNotice::WritingDaysNotRead { .. })),
        "{:?}",
        summary.metadata
    );
    let project = note_text(item(&bundle, "W-note-project"));
    assert!(!project.contains("W-writing-days"), "{project}");
}

/// Writing days that cannot be read: every day is a writing day, said, and the days are
/// listed as written.
#[test]
fn unreadable_writing_days_count_every_day_and_say_so() {
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    spec.targets = Targets {
        deadline: Some("2099-01-01 10:00:00 +0000".to_string()),
        ignore_deadline: false,
        writing_days: "2-9".to_string(),
        can_write_on_deadline: true,
        ..Targets::default()
    };
    let (_dir, bundle, summary) = converted(&spec, None);
    assert_eq!(bundle.paces.len(), 1);
    assert_eq!(bundle.paces[0].weekday_mask, 127);
    assert!(has(
        &summary,
        &MetadataNotice::WritingDaysNotRead {
            raw: "2-9".to_string()
        }
    ));
    let project = note_text(item(&bundle, "W-note-project"));
    assert!(project.contains("W-writing-days\n2-9"), "{project}");
}

/// A draft target in characters is the book's goal and the project's unit; a pace plan
/// counts words, so the deadline makes none, and the scenes' word goals are said hidden.
#[test]
fn a_character_draft_target_sets_the_unit() {
    let mut spec = Spec::novel(Generation::Format20, Platform::Windows).with_s2();
    spec.targets = Targets {
        draft_kind: "Characters".to_string(),
        draft: 400_000,
        deadline: Some("2099-01-01 10:00:00 +0000".to_string()),
        ignore_deadline: false,
        ..Targets::default()
    };
    let (_dir, bundle, summary) = converted(&spec, Some("en-US"));
    assert_eq!(bundle.manifest.work.goal_unit, GoalUnit::Characters);
    assert!(bundle.paces.is_empty());
    let book = items(&bundle)
        .find(|i| i.item.sub_role == common::entities::BinderItemSubRole::Book)
        .unwrap_or_else(|| panic!("no book"));
    assert_eq!(book.item.char_count_goal, 400_000);
    assert_eq!(book.item.word_count_goal, 0);
    assert!(has(
        &summary,
        &MetadataNotice::DeadlineNotPlanned {
            reason: DeadlineReason::CharacterGoal,
            day: Some(4_070_908_800)
        }
    ));
    assert!(has(
        &summary,
        &MetadataNotice::GoalsInOtherUnit { count: 2 }
    ));
}

/// No target at all: the unit is the one New Work starts a project in this language with.
#[test]
fn without_targets_the_language_decides_the_unit() {
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    spec.targets.draft = 0;
    for item in &mut spec.items {
        item.target = None;
    }
    let (_dir, bundle, summary) = converted(&spec, Some("ja"));
    assert_eq!(bundle.manifest.work.goal_unit, GoalUnit::Characters);
    assert!(find(&summary, "scrivener-metadata-draft-target-kept").is_none());
}

/// Scrivener counted every document, or the compile group only: the book's goal reads
/// against its exported scenes, said.
#[test]
fn a_draft_counted_otherwise_is_said() {
    for (count_included_only, compile_group_only) in [(false, false), (true, true)] {
        let mut spec = Spec::novel(Generation::Format15, Platform::Mac);
        spec.targets.count_included_only = count_included_only;
        spec.targets.compile_group_only = compile_group_only;
        let (_dir, _bundle, summary) = converted(&spec, None);
        assert!(
            has(&summary, &MetadataNotice::CountingDiffers),
            "{count_included_only} {compile_group_only}"
        );
    }
    let spec = Spec::novel(Generation::Format15, Platform::Mac);
    let (_dir, _bundle, summary) = converted(&spec, None);
    assert!(!has(&summary, &MetadataNotice::CountingDiffers));
}

// ── the store ────────────────────────────────────────────────────────────────────

/// Every second-half kind survives `load_work`, `save_work` and `load_work` again: the
/// tags and who wears them, the goals and the unit, the references, the dictionary, the
/// pace plan and its day off, and the folder's notes word for word.
#[test]
fn the_second_half_survives_the_store() {
    use common::database::db_context::DbContext;
    use common::event::EventHub;
    use common::long_operation::{LongOperationManager, OperationStatus};
    use work_management::{LoadWorkDto, SaveWorkDto, work_management_controller as controller};

    let spec = Spec::novel(Generation::Format20, Platform::Mac)
        .with_metadata()
        .with_s2();
    let (dir, imported, _summary) = converted(&spec, Some("en-US"));
    let out = dir.path().join("novel.skrib");
    let media = dir.path().join("media");
    let load = |db: &DbContext, path: &Path, media: &Path| {
        if let Err(e) = controller::load_work(
            db,
            &Arc::new(EventHub::new()),
            &LoadWorkDto {
                file_name: path.to_string_lossy().into_owned(),
                media_root: media.to_string_lossy().into_owned(),
            },
        ) {
            panic!("load_work refuses {}: {e:#}", path.display());
        }
    };
    let db = match DbContext::new() {
        Ok(db) => db,
        Err(e) => panic!("a store: {e}"),
    };
    load(&db, &out, &media);
    let work_id = db
        .get_store()
        .works
        .read()
        .ok()
        .and_then(|works| works.keys().next().copied())
        .unwrap_or_default();
    let resaved = dir.path().join("resaved.skrib");
    let mut long_ops = LongOperationManager::new();
    let hub = Arc::new(EventHub::new());
    let op = match controller::save_work(
        &db,
        &hub,
        &mut long_ops,
        &SaveWorkDto {
            work_id,
            file_name: resaved.to_string_lossy().into_owned(),
            overwrite: true,
            media_root: media.to_string_lossy().into_owned(),
        },
    ) {
        Ok(op) => op,
        Err(e) => panic!("save_work does not start: {e:#}"),
    };
    assert!(long_ops.wait_for_operation(&op, Some(Duration::from_secs(120))));
    assert_eq!(
        long_ops.get_operation_status(&op),
        Some(OperationStatus::Completed)
    );
    let saved = read(&resaved);

    type Facts = (Vec<String>, i64, i64, Vec<String>, String);
    let facts = |bundle: &WorkBundle| -> BTreeMap<String, Facts> {
        items(bundle)
            .map(|i| {
                let mut tags = tags_of(bundle, i);
                tags.sort();
                let mut references = references_of(bundle, i);
                references.sort();
                (
                    format!("{} {:?}", i.item.title, i.item.sub_role),
                    (
                        tags,
                        i.item.word_count_goal,
                        i.item.char_count_goal,
                        references,
                        prose(i, ContentRole::NoteText),
                    ),
                )
            })
            .collect()
    };
    let before = facts(&imported);
    assert_eq!(facts(&saved), before);
    assert!(
        before.values().any(|(_, words, ..)| *words > 0),
        "goals compared"
    );
    assert!(
        before.values().any(|(.., refs, _)| !refs.is_empty()),
        "links compared"
    );
    assert!(
        before
            .keys()
            .any(|k| k.starts_with("W-note-custom-metadata")),
        "the folder's notes compared"
    );
    let tags = |bundle: &WorkBundle| -> Vec<(String, String, String, bool)> {
        let mut tags: Vec<_> = bundle
            .tags
            .iter()
            .map(|t| {
                (
                    t.name.clone(),
                    t.color.clone(),
                    t.details.clone(),
                    t.discoverable,
                )
            })
            .collect();
        tags.sort();
        tags
    };
    assert_eq!(tags(&saved), tags(&imported));
    let words = |bundle: &WorkBundle| -> Vec<String> {
        let mut words: Vec<String> = bundle.dict_words.iter().map(|w| w.word.clone()).collect();
        words.sort();
        words
    };
    assert_eq!(words(&saved), words(&imported));
    assert!(!words(&imported).is_empty());
    assert_eq!(
        saved.manifest.work.goal_unit,
        imported.manifest.work.goal_unit
    );
    let pace = |bundle: &WorkBundle| {
        bundle
            .paces
            .iter()
            .map(|p| {
                let book = p.book_item.and_then(|id| {
                    items(bundle)
                        .find(|i| i.item.file_id == id)
                        .map(|i| i.item.title.clone())
                });
                (
                    book,
                    p.start_date.clone(),
                    p.end_date.clone(),
                    p.weekday_mask,
                    p.active,
                    p.holidays
                        .iter()
                        .map(|h| (h.label.clone(), h.start_date.clone(), h.end_date.clone()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(pace(&saved), pace(&imported));
    assert_eq!(pace(&imported).len(), 1);
    assert_eq!(saved.manifest.format_min_read_version, Some(14));
    // And what was saved opens again.
    let again = match DbContext::new() {
        Ok(db) => db,
        Err(e) => panic!("a store: {e}"),
    };
    load(&again, &resaved, &dir.path().join("media-again"));
}
