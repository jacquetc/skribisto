// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A project of every generation and platform reads into the model it was built
//! from: the binder in order with its depths and keys, each item's files where
//! that generation keeps them, the project blocks, the counts, and nothing
//! reported that is not there.

use scrivener_fixtures::{
    Generation as FixtureGeneration, Kind, Platform as FixturePlatform, Spec,
};
use scrivener_import::model::{
    Container, Generation, IdScheme, ItemKey, ItemKind, ManifestChoice, Platform,
};
use scrivener_import::{Analysis, Refusal};

use crate::support::{key, read, refusal, run, temp, write};

fn kind_of(kind: &Kind) -> ItemKind {
    match kind {
        Kind::Draft => ItemKind::DraftFolder,
        Kind::Research => ItemKind::ResearchFolder,
        Kind::Trash => ItemKind::TrashFolder,
        Kind::Folder => ItemKind::Folder,
        Kind::Text => ItemKind::Text,
        Kind::Image => ItemKind::Image,
        Kind::Pdf => ItemKind::Pdf,
        Kind::Other => ItemKind::Other,
    }
}

/// Build the novel for `generation` on `platform`, read it, and check every
/// fact the builder put in.
fn novel_reads_back(generation: FixtureGeneration, platform: FixturePlatform) {
    let dir = temp();
    let spec = Spec::novel(generation, platform);
    let path = write(&spec, dir.path());
    let analysis = run(&path);
    let json = serde_json::to_string(&analysis).unwrap_or_default();
    let back: Result<Analysis, _> = serde_json::from_str(&json);
    assert!(
        back.is_ok_and(|b| b == analysis),
        "the analysis survives a JSON round trip"
    );
    let read = read(analysis);
    let stub = generation == FixtureGeneration::IosStub;
    let flat = matches!(
        generation,
        FixtureGeneration::Format10 | FixtureGeneration::Format15
    );

    // Nothing to report about a healthy project, but where it came from, what stays in
    // the Scrivener project (its snapshots, its writing history) and that it states no
    // language.
    let keys: Vec<&str> = read.notices.iter().map(|n| n.key()).collect();
    if stub {
        assert_eq!(
            keys,
            vec![
                "scrivener-notice-imported-from-ios-stub",
                "scrivener-notice-language-not-stated"
            ]
        );
    } else if generation == FixtureGeneration::Format20 {
        assert_eq!(
            keys,
            vec![
                "scrivener-notice-snapshots-not-imported",
                "scrivener-notice-writing-history-not-imported",
                "scrivener-notice-language-not-stated"
            ]
        );
    } else {
        assert_eq!(
            keys,
            vec![
                "scrivener-notice-snapshots-not-imported",
                "scrivener-notice-language-not-stated"
            ]
        );
    }
    assert!(read.pending.is_empty(), "{:?}", read.pending);
    assert!(read.project.check_shape().is_ok());

    // Where it came from.
    let source = &read.project.source;
    assert_eq!(source.container, Container::Folder);
    assert_eq!(
        source.generation,
        match generation {
            FixtureGeneration::Format10 => Generation::Format10,
            FixtureGeneration::Format15 => Generation::Format15,
            FixtureGeneration::Format20 => Generation::Format20,
            FixtureGeneration::IosStub => Generation::IosStub,
            FixtureGeneration::ScrivenerOne => unreachable_generation(),
        }
    );
    assert_eq!(
        source.id_scheme,
        match generation {
            FixtureGeneration::Format10 => IdScheme::Id,
            FixtureGeneration::Format15 => IdScheme::IdAndUuid,
            _ => IdScheme::Uuid,
        }
    );
    assert_eq!(
        source.writer.platform,
        match (generation, platform) {
            (FixtureGeneration::IosStub, _) => Platform::Ios,
            (_, FixturePlatform::Mac) => Platform::Mac,
            (_, FixturePlatform::Linux) if generation != FixtureGeneration::Format20 =>
                Platform::Linux,
            _ => Platform::Windows,
        }
    );
    assert_eq!(
        source.version_stamp,
        match generation {
            FixtureGeneration::Format10 => Some(16),
            FixtureGeneration::Format15 => Some(18),
            FixtureGeneration::Format20 => Some(23),
            _ => None,
        }
    );
    assert_eq!(source.manifest.name, spec.manifest_name());
    assert_eq!(
        source.manifest.chosen,
        if stub {
            ManifestChoice::MobileBinder
        } else {
            ManifestChoice::Only
        }
    );
    assert_eq!(source.folder_name, spec.folder());
    if !stub {
        assert_eq!(source.saved_as_template.as_deref(), Some("Novel"));
    }
    assert_eq!(
        read.card.device.is_some(),
        !matches!(generation, FixtureGeneration::Format10)
    );

    // The binder, row for row.
    let rows = &read.project.binder;
    assert_eq!(rows.len(), spec.items.len());
    for (i, (row, item)) in rows.iter().zip(&spec.items).enumerate() {
        assert_eq!(row.depth, item.depth, "row {i}");
        assert_eq!(row.kind, kind_of(&item.kind), "row {i}");
        assert_eq!(row.title, item.title, "row {i}");
        assert!(row.owns_key, "row {i}");
        let expected_key = if generation == FixtureGeneration::Format10 {
            ItemKey::Number(spec.number(i))
        } else {
            ItemKey::Uuid(uuid::Uuid::parse_str(&spec.uuid(i)).unwrap_or_default())
        };
        assert_eq!(row.key, Some(expected_key), "row {i}");
        assert_eq!(row.number, flat.then(|| spec.number(i)), "row {i}");
        match &row.payload.text {
            Some(text) => {
                assert!(!item.text.is_empty(), "row {i} has no text in the spec");
                assert_eq!(text.path, spec.text_path(i), "row {i}");
                assert!(text.size > 0);
            }
            None => assert!(item.text.is_empty(), "row {i}'s text was not found"),
        }
        assert_eq!(
            row.payload.notes.is_some(),
            !item.notes.is_empty(),
            "row {i}"
        );
        assert_eq!(
            row.payload.synopsis.is_some(),
            item.synopsis.is_some(),
            "row {i}"
        );
        assert_eq!(row.payload.file.is_some(), item.media.is_some(), "row {i}");
        assert_eq!(
            row.payload.comments.is_some(),
            item.comments > 0 && generation != FixtureGeneration::Format10,
            "row {i}"
        );
        assert_eq!(
            row.payload.links.is_some(),
            item.comments > 0 && generation == FixtureGeneration::Format10,
            "row {i}"
        );
        if !stub {
            assert_eq!(
                row.meta.include_in_compile, item.include_in_compile,
                "row {i}"
            );
            assert_eq!(row.keyword_ids, item.keywords, "row {i}");
            let custom: Vec<(String, String)> = row
                .meta
                .custom
                .iter()
                .map(|c| (c.field.clone(), c.value.clone()))
                .collect();
            assert_eq!(custom, item.custom, "row {i}");
            assert_eq!(row.snapshots, item.snapshots, "row {i}");
        }
        assert_eq!(row.meta.label, item.label, "row {i}");
        assert_eq!(row.meta.status, item.status, "row {i}");
        if !stub {
            assert_eq!(
                row.target.as_ref().map(|t| t.value.clone()),
                item.target.map(|t| t.to_string()),
                "row {i}"
            );
        }
        assert_eq!(
            row.meta.section_type.is_some(),
            item.section_type.is_some() && generation == FixtureGeneration::Format20,
            "row {i}"
        );
    }

    // The project blocks.
    let project = &read.project;
    assert_eq!(project.labels.entries.len(), spec.labels.len());
    assert_eq!(project.labels.title.as_deref(), Some("Label"));
    assert_eq!(project.statuses.entries.len(), spec.statuses.len());
    // An iOS binder carries only the binder, labels, statuses and targets.
    assert_eq!(
        project.template_folder,
        (!stub).then(|| key(&read, "Templates"))
    );
    if stub {
        assert!(
            project.keywords.is_empty()
                && project.custom_fields.is_empty()
                && project.collections.is_empty()
        );
    } else {
        let keywords: Vec<(u16, i64)> = project.keywords.iter().map(|k| (k.depth, k.id)).collect();
        assert_eq!(keywords, vec![(0, 1), (1, 2), (1, 3)]);
        assert_eq!(project.custom_fields.len(), 1);
        assert_eq!(project.custom_fields[0].title.as_deref(), Some("Setting"));
        assert_eq!(project.collections.len(), 2);
        // Collections name their members by the items' own keys, whatever the
        // generation wrote.
        assert_eq!(
            project.collections[1].members,
            vec![key(&read, "Arrival"), key(&read, "The Harbour")]
        );
        assert!(project.targets.draft.is_some() && project.targets.session.is_some());
        assert!(project.settings.template_info.is_some());
    }
    assert_eq!(
        project.section_types.definitions.len(),
        if generation == FixtureGeneration::Format20 {
            2
        } else {
            0
        }
    );
    assert_eq!(project.project_notes.len(), usize::from(flat));
    if flat {
        assert!(project.project_notes[0].file.is_some());
    }
    assert!(project.recovered.is_empty());

    // The counts the report needs.
    let counts = &project.counts;
    assert_eq!(counts.items as usize, spec.items.len());
    assert_eq!(
        counts.text_items as usize,
        spec.items.iter().filter(|i| i.kind == Kind::Text).count()
    );
    assert_eq!(
        counts.texts_present as usize,
        spec.items.iter().filter(|i| !i.text.is_empty()).count()
    );
    assert_eq!(counts.folders_with_text, 1);
    assert_eq!(
        counts.media_items as usize,
        spec.items.iter().filter(|i| i.media.is_some()).count()
    );
    assert_eq!(counts.media_missing, 0);
    assert_eq!(
        counts.research_files as usize,
        spec.items.iter().filter(|i| i.kind == Kind::Pdf).count()
    );
    assert_eq!(counts.trashed_items, 1);
    assert_eq!(counts.snapshots, if stub { 0 } else { 2 });
    assert_eq!(counts.items_with_snapshots, if stub { 0 } else { 1 });
}

fn unreachable_generation() -> Generation {
    panic!("Scrivener 1 is never read")
}

#[test]
fn format_2_0_from_a_mac_reads_back() {
    novel_reads_back(FixtureGeneration::Format20, FixturePlatform::Mac);
}

#[test]
fn format_2_0_from_windows_reads_back() {
    novel_reads_back(FixtureGeneration::Format20, FixturePlatform::Windows);
}

#[test]
fn format_1_5_from_a_mac_reads_back() {
    novel_reads_back(FixtureGeneration::Format15, FixturePlatform::Mac);
}

#[test]
fn format_1_5_from_windows_reads_back() {
    novel_reads_back(FixtureGeneration::Format15, FixturePlatform::Windows);
}

#[test]
fn format_1_5_from_the_linux_beta_reads_back() {
    novel_reads_back(FixtureGeneration::Format15, FixturePlatform::Linux);
}

#[test]
fn format_1_0_from_a_mac_reads_back() {
    novel_reads_back(FixtureGeneration::Format10, FixturePlatform::Mac);
}

#[test]
fn format_1_0_from_windows_reads_back() {
    novel_reads_back(FixtureGeneration::Format10, FixturePlatform::Windows);
}

#[test]
fn format_1_0_from_the_linux_beta_reads_back() {
    novel_reads_back(FixtureGeneration::Format10, FixturePlatform::Linux);
}

#[test]
fn a_project_made_on_an_ipad_reads_from_its_mobile_folder() {
    novel_reads_back(FixtureGeneration::IosStub, FixturePlatform::Ios);
}

/// Scrivener 1 for Mac is recognised, and refused by decision: a precise refusal
/// rather than "not a project".
#[test]
fn scrivener_one_for_mac_is_refused_by_name() {
    let dir = temp();
    let path = write(
        &Spec::novel(FixtureGeneration::ScrivenerOne, FixturePlatform::Mac),
        dir.path(),
    );
    assert_eq!(refusal(run(&path)), Refusal::ScrivenerOneForMac);
    // Picking its binder file says the same.
    assert_eq!(
        refusal(run(&path.join("binder.scrivproj"))),
        Refusal::ScrivenerOneForMac
    );
}

/// The binder of a format 1.5 project is identified by UUID, and its files are
/// found by number: a reader that takes "has a UUID" for Scrivener 3 finds none.
#[test]
fn format_1_5_files_are_found_by_number_not_by_uuid() {
    let dir = temp();
    let spec = Spec::novel(FixtureGeneration::Format15, FixturePlatform::Mac);
    let path = write(&spec, dir.path());
    let read = read(run(&path));
    let arrival = crate::support::row(&read, "Arrival");
    assert!(matches!(arrival.key, Some(ItemKey::Uuid(_))));
    assert_eq!(
        arrival.payload.text.as_ref().map(|t| t.path.clone()),
        Some(format!(
            "Files/Docs/{}.rtf",
            arrival.number.unwrap_or_default()
        ))
    );
}
