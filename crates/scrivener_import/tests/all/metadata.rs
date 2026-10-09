// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What a project states about itself, end to end: the title, the authors and the language
//! it proposes, its labels as tags and its status list as the ladder, its dates, and what
//! stays in the Scrivener project, for every generation that writes them; then their round
//! trip through a real store.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use common::entities::StatusCategory;
use scrivener_fixtures::{Generation, Item, Kind, Platform, Spec};
use scrivener_import::metadata::{
    AuthorSource, Field, LanguageProposal, LanguageSource, MetadataNotice, TitleSource,
};
use scrivener_import::structure::Choices;
use scrivener_import::{
    Answers, CompileTreatment, Notice, ProseChoices, ProseNotice, Request, Summary, convert,
};
use skrib_format::{BundledItem, WorkBundle};

use crate::convert::{instant, item, items, loads, read, words};
use crate::support::{options, read as read_project, run, temp, write, write_files};

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

/// Convert the project at `source` with the Save page left to what the project states.
fn convert_stated(source: &Path, out: &Path) -> Summary {
    let (choices, words, answers, options) =
        (Choices::default(), words(), Answers::default(), options());
    let output = out.to_string_lossy().into_owned();
    let request = Request {
        source,
        answers: &answers,
        options: &options,
        choices: &choices,
        prose: ProseChoices::default(),
        fingerprint: None,
        output: &output,
        overwrite: false,
        words: &words,
        title: None,
        author: None,
        language: None,
        copy_research_files: false,
        now: instant(),
    };
    match convert(&request, &|_, _| {}, &AtomicBool::new(false)) {
        Ok(summary) => summary,
        Err(e) => panic!("the conversion failed: {e}"),
    }
}

/// The names of the tags a row wears.
fn tags_of<'b>(bundle: &'b WorkBundle, row: &BundledItem) -> Vec<&'b str> {
    row.item
        .tag_ids
        .iter()
        .filter_map(|id| bundle.tags.iter().find(|t| t.file_id == *id))
        .map(|t| t.name.as_str())
        .collect()
}

/// The name of a row's rung.
fn rung_of<'b>(bundle: &'b WorkBundle, row: &BundledItem) -> Option<&'b str> {
    row.item
        .status_id
        .and_then(|id| bundle.statuses.iter().find(|s| s.file_id == id))
        .map(|s| s.name.as_str())
}

// ── every generation ─────────────────────────────────────────────────────────────

/// A project of each generation with its metadata set: the title, the authors and the
/// language it states are proposed and written when the Save page is left alone; the
/// labels a row wears become tags on it with their own colours; the status list becomes
/// the ladder, in its order, on the rows that wore it; dates keep their instants and an
/// unreadable one is said; the snapshots and the writing history are counted, not carried.
#[test]
fn every_generation_brings_its_title_labels_ladder_and_dates() {
    for (generation, platform) in GENERATIONS {
        let label = format!("{generation:?} {platform:?}");
        let dir = temp();
        let spec = Spec::novel(generation, platform).with_metadata();
        let folder = write(&spec, dir.path());

        // What the analysis proposes, and says about it.
        let analysis = read_project(run(&folder));
        let proposals = &analysis.proposals;
        let desktop = generation != Generation::IosStub;
        if desktop {
            assert_eq!(proposals.title.title, "The Lighthouse Keepers", "{label}");
            let (title_source, author_source) = if generation == Generation::Format20 {
                (TitleSource::CompileSettings, AuthorSource::CompileSettings)
            } else {
                (
                    TitleSource::ProjectProperties,
                    AuthorSource::ProjectProperties,
                )
            };
            assert_eq!(proposals.title.source, title_source, "{label}");
            let author = proposals
                .author
                .as_ref()
                .map(|a| (a.names.clone(), a.source));
            assert_eq!(
                author,
                Some((vec!["Mara Holt".to_string()], author_source)),
                "{label}: the sorting form written as it prints"
            );
            assert_eq!(
                proposals.language,
                Some(LanguageProposal {
                    tag: "fr".to_string(),
                    source: LanguageSource::CompileSettings,
                }),
                "{label}"
            );
            assert!(
                analysis.notices.contains(&Notice::TitleFromProject {
                    source: title_source
                }),
                "{label}"
            );
            assert!(
                analysis.notices.contains(&Notice::SnapshotsNotImported {
                    snapshots: 2,
                    documents: 1,
                    of_deleted: 0
                }),
                "{label}: {:?}",
                analysis.notices
            );
        } else {
            // An iPad project states nothing: its name is its folder's.
            assert_eq!(proposals.title.title, "Lighthouse");
            assert_eq!(proposals.title.source, TitleSource::FolderName);
            assert_eq!(proposals.author, None);
            assert_eq!(proposals.language, None);
            assert!(analysis.notices.contains(&Notice::LanguageNotStated));
        }
        if generation == Generation::Format20 {
            assert!(
                analysis
                    .notices
                    .contains(&Notice::WritingHistoryNotImported {
                        days: 3,
                        first_day: Some("2024-03-02".to_string()),
                        last_day: Some("2024-04-01".to_string()),
                        unreadable: 1,
                    }),
                "{label}: {:?}",
                analysis.notices
            );
        }

        // What the conversion writes.
        let out = dir.path().join("novel.skrib");
        let summary = convert_stated(&folder, &out);
        let bundle = read(&out);
        let work = &bundle.manifest.work;
        if desktop {
            assert_eq!(work.title, "The Lighthouse Keepers", "{label}");
            assert_eq!(work.author_name, "Mara Holt", "{label}");
            assert_eq!(work.dict_language, ["fr"], "{label}");
        } else {
            assert_eq!(work.title, "Lighthouse");
            assert_eq!(work.author_name, "");
            assert!(work.dict_language.is_empty());
        }

        // Labels: the ones rows wear, in the list's order, with the writer's colours.
        let labels: Vec<(&str, &str, &str, bool)> = bundle
            .tags
            .iter()
            .filter(|t| !t.discoverable)
            .map(|t| {
                (
                    t.name.as_str(),
                    t.color.as_str(),
                    t.details.as_str(),
                    t.discoverable,
                )
            })
            .collect();
        assert_eq!(
            labels,
            [
                ("Scene", "#fdb333", "W-field Label", false),
                ("Mara", "#e4b34d", "W-field Label", false),
                ("Keeper", "#cab366", "W-field Label", false),
            ],
            "{label}"
        );
        assert_eq!(
            tags_of(&bundle, item(&bundle, "Arrival")),
            ["Mara"],
            "{label}"
        );
        assert_eq!(tags_of(&bundle, item(&bundle, "The Harbour")), ["Scene"]);
        assert_eq!(tags_of(&bundle, item(&bundle, "Chapter 1")), ["Keeper"]);
        assert_eq!(tags_of(&bundle, item(&bundle, "Mara")), ["Mara"], "{label}");
        assert!(tags_of(&bundle, item(&bundle, "Night Watch")).is_empty());
        assert!(bundle.manifest.work.tag_ids.len() >= 3);

        // The ladder, in the list's order, each rung's kind from its name.
        let ladder: Vec<(&str, StatusCategory)> = bundle
            .statuses
            .iter()
            .map(|s| (s.name.as_str(), s.category.clone()))
            .collect();
        assert_eq!(
            ladder,
            [
                ("To Do", StatusCategory::Planned),
                ("First Draft", StatusCategory::Drafting),
                ("Revised Draft", StatusCategory::Revised),
                ("Done", StatusCategory::Final),
            ],
            "{label}"
        );
        assert!(bundle.statuses.iter().all(|s| !s.uid.is_nil()));
        assert_eq!(
            rung_of(&bundle, item(&bundle, "Arrival")),
            Some("First Draft")
        );
        assert_eq!(rung_of(&bundle, item(&bundle, "The Harbour")), Some("Done"));
        assert_eq!(
            rung_of(&bundle, item(&bundle, "Chapter 1")),
            Some("Revised Draft")
        );
        assert_eq!(rung_of(&bundle, item(&bundle, "Night Watch")), None);
        assert!(
            summary
                .metadata
                .contains(&MetadataNotice::UnusedLabelsSkipped {
                    names: vec!["Unused".to_string()]
                })
        );
        assert!(
            summary
                .metadata
                .iter()
                .any(|n| matches!(n, MetadataNotice::StatusesBecameLadder { rungs: 4, .. })),
            "{label}"
        );

        // Dates: the instant Scrivener wrote, whatever its offset; an unreadable one is
        // the import's, and said.
        let arrival = item(&bundle, "Arrival");
        assert_eq!(
            arrival.item.created_at, "2024-03-13T22:30:00+00:00",
            "{label}"
        );
        assert_eq!(
            arrival.item.updated_at, "2024-06-03T01:40:12+00:00",
            "{label}"
        );
        for content in &arrival.item.prose_refs {
            assert_eq!(content.created_at, arrival.item.created_at);
        }
        let harbour = item(&bundle, "The Harbour");
        assert_eq!(harbour.item.created_at, "2024-03-14T09:15:00+00:00");
        assert_eq!(harbour.item.updated_at, instant().to_rfc3339(), "{label}");
        assert!(
            summary
                .metadata
                .contains(&MetadataNotice::DatesUnreadable { items: 1 }),
            "{label}: {:?}",
            summary.metadata
        );
        // Everything Scrivener does not date is the import's.
        for tag in &bundle.tags {
            assert_eq!(tag.created_at, instant().to_rfc3339());
        }
        for rung in &bundle.statuses {
            assert_eq!(rung.created_at, instant().to_rfc3339());
        }
        assert_eq!(work.created_at, instant().to_rfc3339());
        assert!(bundle.history.entries.is_empty(), "no snapshot is carried");
        assert!(
            bundle.progress_snapshots.is_empty(),
            "no writing history is carried"
        );
        loads(bundle, &out);
    }
}

/// With the Save page's answers, the project's own statements give way.
#[test]
fn the_writers_answers_on_the_save_page_are_what_is_written() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac).with_metadata();
    let folder = write(&spec, dir.path());
    let out = dir.path().join("novel.skrib");
    let (choices, words, answers, options) =
        (Choices::default(), words(), Answers::default(), options());
    let output = out.to_string_lossy().into_owned();
    let request = Request {
        source: &folder,
        answers: &answers,
        options: &options,
        choices: &choices,
        prose: ProseChoices::default(),
        fingerprint: None,
        output: &output,
        overwrite: false,
        words: &words,
        title: Some("Keepers"),
        author: Some(" M. V. "),
        language: Some("ja"),
        copy_research_files: false,
        now: instant(),
    };
    if let Err(e) = convert(&request, &|_, _| {}, &AtomicBool::new(false)) {
        panic!("the conversion failed: {e}");
    }
    let work = read(&out).manifest.work;
    assert_eq!(work.title, "Keepers");
    assert_eq!(work.author_name, "M. V.");
    assert_eq!(work.dict_language, ["ja"]);
    assert_eq!(
        work.goal_unit,
        common::entities::GoalUnit::Characters,
        "a Japanese manuscript counts characters, as New Work would start it"
    );
}

/// A renamed label field keeps its tags' names and says where they came from; a project
/// with no status list starts with Skribisto's default ladder, in the writer's language.
#[test]
fn a_renamed_field_and_a_missing_ladder() {
    let dir = temp();
    let mut spec = Spec::novel(Generation::Format15, Platform::Windows).with_metadata();
    spec.label_title = "Time".to_string();
    spec.statuses = vec![(-1, "No Status".to_string())];
    for item in &mut spec.items {
        item.status = None;
    }
    let folder = write(&spec, dir.path());
    let out = dir.path().join("novel.skrib");
    let summary = convert_stated(&folder, &out);
    let bundle = read(&out);
    let mara = bundle.tags.iter().find(|t| t.name == "Mara");
    assert_eq!(mara.map(|t| t.details.as_str()), Some("W-field Time"));
    assert!(summary.metadata.contains(&MetadataNotice::FieldRenamed {
        field: Field::Label,
        title: "Time".to_string()
    }));
    let ladder: Vec<&str> = bundle.statuses.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(ladder, ["W-planned", "W-final"]);
    assert!(
        summary
            .metadata
            .contains(&MetadataNotice::LadderSeeded { rungs: 2 })
    );
    loads(bundle, &out);
}

/// The duplicate "none" a Scrivener 1 upgrade leaves beside the real one is none, and its
/// status `N/A` with it (X-38).
#[test]
fn the_upgrade_residue_is_none() {
    let dir = temp();
    let mut spec = Spec::novel(Generation::Format10, Platform::Mac);
    spec.labels = vec![
        (-1, "No Label".to_string()),
        (0, "No Label".to_string()),
        (1, "Idea".to_string()),
    ];
    spec.statuses = vec![
        (-1, "No Status".to_string()),
        (0, "N/A".to_string()),
        (1, "To Do".to_string()),
        (2, "Done".to_string()),
    ];
    for item in &mut spec.items {
        item.label = Some(0);
        item.status = Some(0);
    }
    let folder = write(&spec, dir.path());
    let out = dir.path().join("novel.skrib");
    let summary = convert_stated(&folder, &out);
    let bundle = read(&out);
    assert!(bundle.tags.iter().all(|t| t.discoverable), "no label tag");
    let ladder: Vec<&str> = bundle.statuses.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(ladder, ["To Do", "Done"]);
    assert!(items(&bundle).all(|i| i.item.status_id.is_none()));
    let collapsed: Vec<Field> = summary
        .metadata
        .iter()
        .filter_map(|n| match n {
            MetadataNotice::NoneValueCollapsed { field, .. } => Some(*field),
            _ => None,
        })
        .collect();
    assert_eq!(collapsed, [Field::Label, Field::Status]);
}

// ── include in compile ───────────────────────────────────────────────────────────

/// Each row exports as its own item says, in every generation: Scrivener never hands an
/// exclusion down, so a scene it includes under a chapter it leaves out still exports, and
/// one it leaves out under a chapter it includes does not (D4-10).
#[test]
fn every_item_exports_as_it_says_whatever_its_parent_says() {
    for (generation, platform) in GENERATIONS {
        let label = format!("{generation:?} {platform:?}");
        let dir = temp();
        let mut spec = Spec::novel(generation, platform);
        for (title, compiled) in [("Chapter 1", false), ("Night Watch", false)] {
            if let Some(index) = spec.index_of(title) {
                spec.items[index].include_in_compile = compiled;
            }
        }
        let folder = write(&spec, dir.path());
        let out = dir.path().join("novel.skrib");
        convert_stated(&folder, &out);
        let bundle = read(&out);
        let exported = |title: &str| item(&bundle, title).item.is_exportable;
        assert!(!exported("Chapter 1"), "{label}");
        assert!(
            exported("Arrival"),
            "{label}: included under an excluded chapter"
        );
        assert!(exported("Chapter 2"), "{label}");
        assert!(
            !exported("Night Watch"),
            "{label}: excluded under an included chapter"
        );
        assert!(!exported("Mara"), "{label}: a note never exports");
    }
}

// ── the prose the compile left out ───────────────────────────────────────────────

/// Struck-through text is kept; when the project's compile removed it, and when the
/// compile format last used deleted a style, the export will print what Scrivener left
/// out, and the writer is told.
#[test]
fn what_the_compile_left_out_is_kept_and_said() {
    for (generation, platform) in [
        (Generation::Format20, Platform::Mac),
        (Generation::Format15, Platform::Mac),
    ] {
        let label = format!("{generation:?} {platform:?}");
        let dir = temp();
        let spec = Spec::novel(generation, platform).with_metadata();
        let mut files = spec.build();
        let arrival = spec.index_of("Arrival").unwrap_or_default();
        let style = "5E1F0000-0000-4000-8000-000000000001";
        files.put(
            spec.text_path(arrival),
            br"{\rtf1\ansi Kept {\strike cut words} here.\par <$Scr_Ps::0>Check the dates.<!$Scr_Ps::0>\par}"
                .to_vec(),
        );
        if generation == Generation::Format20 {
            files.put(
                "Files/styles.xml",
                format!(
                    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Styles>\n    <Style Name=\"Private Note\" ID=\"{style}\" Type=\"Para\">\n        <Format><![CDATA[{{\\rtf1 x}}]]></Format>\n    </Style>\n</Styles>\n"
                )
                .into_bytes(),
            );
            files.put(
                format!("{}content.styles", spec.payload_prefix(arrival)),
                style.as_bytes().to_vec(),
            );
        }
        let folder = write_files(&files, dir.path(), &spec.folder());
        let out = dir.path().join("novel.skrib");
        let summary = convert_stated(&folder, &out);
        let struck: Vec<u32> = summary
            .prose
            .iter()
            .filter_map(|n| match n {
                ProseNotice::StruckTextCompiledAway { count, .. } => Some(*count),
                _ => None,
            })
            .collect();
        assert_eq!(struck, [8], "{label}");
        let styles: Vec<(&str, CompileTreatment)> = summary
            .prose
            .iter()
            .filter_map(|n| match n {
                ProseNotice::CompiledAwayStylePrinted {
                    name, treatment, ..
                } => Some((name.as_str(), *treatment)),
                _ => None,
            })
            .collect();
        if generation == Generation::Format20 {
            assert_eq!(
                styles,
                [("Private Note", CompileTreatment::Deleted)],
                "{label}"
            );
        } else {
            assert!(
                styles.is_empty(),
                "{label}: format 1.x has no compile formats"
            );
        }
        let bundle = read(&out);
        let text = crate::convert::prose(
            item(&bundle, "Arrival"),
            common::entities::ContentRole::SceneText,
        );
        assert!(text.contains("{-cut words-}"), "{label}: {text}");
        assert!(text.contains("Check the dates."), "{label}: {text}");
    }
}

/// A format 2.0 compile that says nothing about struck-through text (Scrivener 3 writes the
/// switch only when it is on) keeps it, whatever a Scrivener 2 `compile.plist` left beside
/// it from before the upgrade says: that file no longer compiles anything.
#[test]
fn a_scrivener_2_leftover_does_not_speak_for_a_scrivener_3_compile() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac).with_metadata();
    let mut files = spec.build();
    let arrival = spec.index_of("Arrival").unwrap_or_default();
    files.put(
        spec.text_path(arrival),
        br"{\rtf1\ansi Kept {\strike cut words} here.\par}".to_vec(),
    );
    files.put(
        "Settings/compile.xml",
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<CompileSettings>\n    <ProjectSettings>\n        <Options>\n            <RemoveAnnotations>Yes</RemoveAnnotations>\n        </Options>\n        <MetaData>\n            <EbookLanguage>en</EbookLanguage>\n        </MetaData>\n    </ProjectSettings>\n</CompileSettings>\n",
    );
    files.put(
        "Settings/compile.plist",
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<plist version=\"1.0\">\n<dict>\n\t<key>SCRCompileDeleteStruckThroughTextKey</key>\n\t<true/>\n</dict>\n</plist>\n",
    );
    let folder = write_files(&files, dir.path(), &spec.folder());
    let analysis = read_project(run(&folder));
    assert!(!analysis.project.compile.delete_struck_through);
    let out = dir.path().join("novel.skrib");
    let summary = convert_stated(&folder, &out);
    assert!(
        !summary
            .prose
            .iter()
            .any(|n| matches!(n, ProseNotice::StruckTextCompiledAway { .. })),
        "{:?}",
        summary.prose
    );
}

// ── names the import words ───────────────────────────────────────────────────────

/// A title taken from the folder's name never keeps what a sync service or a backup added
/// to it, nor a computer's or a person's name.
#[test]
fn a_title_from_the_folder_keeps_no_computers_or_persons_name() {
    for (name, generation, title) in [
        (
            "Lighthouse (Tav's MacBook's conflicted copy 2024-01-02)",
            Generation::Format20,
            "Lighthouse",
        ),
        (
            "Lighthouse (Tav's MacBook's conflicted copy 2024-01-02 (1))",
            Generation::Format20,
            "Lighthouse",
        ),
        ("Lighthouse-Test-Mac", Generation::Format20, "Lighthouse"),
        (
            "Lighthouse-DESKTOP-4F2A9QK",
            Generation::Format20,
            "Lighthouse",
        ),
        // The copy Scrivener keeps before upgrading is of the format it upgrades from; a
        // current project may well be called that.
        ("Lighthouse Backup", Generation::Format15, "Lighthouse"),
        ("The Backup", Generation::Format20, "The Backup"),
    ] {
        let dir = temp();
        let mut spec = Spec::novel(generation, Platform::Mac);
        spec.name = name.to_string();
        let folder = write(&spec, dir.path());
        let analysis = read_project(run(&folder));
        assert_eq!(analysis.proposals.title.title, title, "{name}");
        assert_eq!(analysis.project.source.folder_title, title, "{name}");
        let out = dir.path().join("novel.skrib");
        convert_stated(&folder, &out);
        let bundle = read(&out);
        assert_eq!(bundle.manifest.work.title, title, "{name}");
        let book =
            items(&bundle).find(|i| i.item.sub_role == common::entities::BinderItemSubRole::Book);
        assert_eq!(book.map(|b| b.item.title.as_str()), Some(title), "{name}");
    }
}

/// A character folder recognised by its icon but given no title names its story-bible tag
/// with the writer's word for it, never an empty name.
#[test]
fn an_untitled_story_bible_folder_names_its_tag_in_the_writers_language() {
    let dir = temp();
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    let at = spec.index_of("Notes").unwrap_or(spec.items.len());
    spec.items.insert(
        at,
        Item {
            title: None,
            icon: Some("Characters (Photo)".to_string()),
            ..Item::new(0, Kind::Folder, "")
        },
    );
    spec.items.insert(
        at + 1,
        Item::new(1, Kind::Text, "Inspector Vane").with_text(12, 1),
    );
    spec.template_folder = spec.index_of("Templates");
    let folder = write(&spec, dir.path());
    let out = dir.path().join("novel.skrib");
    convert_stated(&folder, &out);
    let bundle = read(&out);
    let bible: Vec<&str> = bundle
        .tags
        .iter()
        .filter(|t| t.discoverable)
        .map(|t| t.name.as_str())
        .collect();
    assert_eq!(bible, ["W-untitled-tag"]);
    assert_eq!(
        tags_of(&bundle, item(&bundle, "Inspector Vane")),
        ["W-untitled-tag"]
    );
}

/// A document the binder no longer lists is titled with its Scrivener number and dated
/// the import: nothing in Scrivener dates it.
#[test]
fn a_recovered_text_is_titled_by_its_number_and_dated_the_import() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format15, Platform::Windows);
    let mut files = spec.build();
    files.put(
        "Files/Docs/88.rtf",
        scrivener_fixtures::rtf::document(
            scrivener_fixtures::rtf::Dialect::Qt,
            &["A scene the binder lost.".to_string()],
        )
        .into_bytes(),
    );
    let folder = write_files(&files, dir.path(), &spec.folder());
    let out = dir.path().join("novel.skrib");
    convert_stated(&folder, &out);
    let bundle = read(&out);
    let recovered = item(&bundle, "88");
    assert_eq!(recovered.item.created_at, instant().to_rfc3339());
    assert_eq!(recovered.item.updated_at, instant().to_rfc3339());
    let folder_row = item(&bundle, "W-Recovered");
    assert_eq!(folder_row.item.created_at, instant().to_rfc3339());
}

/// A date at the edge of the years Scrivener's four digits write, whose instant crosses
/// them once in UTC, is not read: the row is dated the import and it is said, and the
/// project still loads. Stored as it reads, it would be year -1 or 10000, which the app's
/// own reader refuses, and with it the whole project.
#[test]
fn a_date_past_the_years_the_app_stores_leaves_the_project_loadable() {
    let dir = temp();
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    if let Some(i) = spec.index_of("Arrival") {
        spec.items[i].created = Some("0000-01-01 00:30:00 +0100".to_string());
        spec.items[i].modified = Some("9999-12-31 23:30:00 -0100".to_string());
    }
    let folder = write(&spec, dir.path());
    let out = dir.path().join("novel.skrib");
    let summary = convert_stated(&folder, &out);
    let bundle = read(&out);
    let arrival = item(&bundle, "Arrival");
    assert_eq!(arrival.item.created_at, instant().to_rfc3339());
    assert_eq!(arrival.item.updated_at, instant().to_rfc3339());
    assert!(
        summary
            .metadata
            .contains(&MetadataNotice::DatesUnreadable { items: 1 }),
        "{:?}",
        summary.metadata
    );
    loads(bundle, &out);
}

// ── the backend ──────────────────────────────────────────────────────────────────

/// The written project loads into a real store through `load_work` and saves back out with
/// its tags, its ladder, every row's tags and rung, and every row's dates.
#[test]
fn tags_the_ladder_and_dates_survive_the_store() {
    use common::database::db_context::DbContext;
    use common::event::EventHub;
    use common::long_operation::{LongOperationManager, OperationStatus};
    use work_management::{LoadWorkDto, SaveWorkDto, work_management_controller as controller};

    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac).with_metadata();
    let folder = write(&spec, dir.path());
    let out = dir.path().join("novel.skrib");
    convert_stated(&folder, &out);
    let imported = read(&out);

    let db = match DbContext::new() {
        Ok(db) => db,
        Err(e) => panic!("a store: {e}"),
    };
    let hub = Arc::new(EventHub::new());
    let media = dir.path().join("media");
    if let Err(e) = controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            file_name: out.to_string_lossy().into_owned(),
            media_root: media.to_string_lossy().into_owned(),
        },
    ) {
        panic!("load_work refuses the project: {e:#}");
    }
    let work_id = db
        .get_store()
        .works
        .read()
        .ok()
        .and_then(|works| works.keys().next().copied())
        .unwrap_or_default();
    let resaved = dir.path().join("resaved.skrib");
    let mut long_ops = LongOperationManager::new();
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

    let vocabulary = |bundle: &WorkBundle| {
        let tags: Vec<(String, String, String, bool)> = bundle
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
        let ladder: Vec<(String, StatusCategory, String)> = bundle
            .statuses
            .iter()
            .map(|s| (s.name.clone(), s.category.clone(), s.details.clone()))
            .collect();
        (tags, ladder)
    };
    assert_eq!(vocabulary(&saved), vocabulary(&imported));
    assert_eq!(
        saved.statuses.iter().map(|s| s.uid).collect::<Vec<_>>(),
        imported.statuses.iter().map(|s| s.uid).collect::<Vec<_>>(),
        "each rung keeps its identity"
    );
    type RowFacts = (Vec<String>, Option<String>, String, String, bool);
    let rows = |bundle: &WorkBundle| -> BTreeMap<String, RowFacts> {
        items(bundle)
            .map(|i| {
                let mut tags: Vec<String> =
                    tags_of(bundle, i).into_iter().map(str::to_string).collect();
                tags.sort();
                (
                    format!("{} {:?}", i.item.title, i.item.sub_role),
                    (
                        tags,
                        rung_of(bundle, i).map(str::to_string),
                        i.item.created_at.clone(),
                        i.item.updated_at.clone(),
                        i.item.is_exportable,
                    ),
                )
            })
            .collect()
    };
    let before = rows(&imported);
    assert_eq!(rows(&saved), before);
    assert!(
        before
            .values()
            .any(|(tags, rung, _, _, _)| !tags.is_empty() && rung.is_some()),
        "the comparison holds rows with a tag and a rung, not only defaults"
    );
    assert!(
        before.values().any(|(_, _, _, _, exported)| *exported)
            && before.values().any(|(_, _, _, _, exported)| !*exported),
        "the comparison holds rows that export and rows that do not"
    );
    // And what was saved opens again.
    let again = match DbContext::new() {
        Ok(db) => db,
        Err(e) => panic!("a store: {e}"),
    };
    if let Err(e) = controller::load_work(
        &again,
        &Arc::new(EventHub::new()),
        &LoadWorkDto {
            file_name: resaved.to_string_lossy().into_owned(),
            media_root: dir
                .path()
                .join("media-again")
                .to_string_lossy()
                .into_owned(),
        },
    ) {
        panic!("load_work refuses the saved project: {e:#}");
    }
    assert_eq!(saved.manifest.work.title, imported.manifest.work.title);
    assert_eq!(
        saved.manifest.work.author_name,
        imported.manifest.work.author_name
    );
    assert_eq!(
        saved.manifest.work.dict_language,
        imported.manifest.work.dict_language
    );
    assert_eq!(
        imported.manifest.format_min_read_version,
        Some(14),
        "a ladder claims the read floor an older build would delete it under"
    );
    assert_eq!(saved.manifest.format_min_read_version, Some(14));
}

/// A project that ends with no ladder (no status list, and nothing to seed it with) claims
/// no read floor for one: only a ladder an older build would delete needs it.
#[test]
fn no_ladder_claims_no_read_floor() {
    let dir = temp();
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    spec.statuses = vec![(-1, "No Status".to_string())];
    for item in &mut spec.items {
        item.status = None;
    }
    let folder = write(&spec, dir.path());
    let out = dir.path().join("novel.skrib");
    let (choices, answers, options) = (Choices::default(), Answers::default(), options());
    let words = scrivener_import::Words {
        default_statuses: Vec::new(),
        ..words()
    };
    let output = out.to_string_lossy().into_owned();
    let request = crate::convert::request(&folder, &output, &choices, &words, &answers, &options);
    if let Err(e) = convert(&request, &|_, _| {}, &AtomicBool::new(false)) {
        panic!("the conversion failed: {e}");
    }
    let bundle = read(&out);
    assert!(bundle.statuses.is_empty());
    assert!(
        bundle
            .manifest
            .format_min_read_version
            .is_none_or(|floor| floor < 14),
        "{:?}",
        bundle.manifest.format_min_read_version
    );
    loads(bundle, &out);
}
