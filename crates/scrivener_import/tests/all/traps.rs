// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What real projects hold that a naive reader gets wrong, one test per trap.
//!
//! Each builds the novel fixture, lays one trap on it the way real projects were
//! found to carry it, and checks the typed outcome the design settles on.

use scrivener_fixtures::traps::{CopyScheme, copy_name, icloud_stub, nextcloud_stub};
use scrivener_fixtures::{Backup, Files, Generation, Item, Kind, Platform, Spec};
use scrivener_import::model::{
    ConflictScheme, Generation as ReadGeneration, ItemKey, ManifestChoice, RecoveredOrigin,
    TextRole,
};
use scrivener_import::outcome::{ManifestDamage, ManifestEvidence, MobileEdits};
use scrivener_import::{Analysis, Answers, Notice, Question, Refusal, SiblingChoice};

use crate::support::{
    HOST, key, on_long_operation_stack, question, read, refusal, row, run, run_with, temp, write,
    write_files,
};

fn novel() -> Spec {
    Spec::novel(Generation::Format20, Platform::Mac)
}

/// `manifest` saved again: another `ModID`, as every save writes.
fn saved_again(manifest: &str) -> String {
    match manifest.find("ModID=\"") {
        Some(at) => {
            let start = at + "ModID=\"".len();
            let end = manifest[start..].find('"').map_or(start, |e| start + e);
            format!(
                "{}{}{}",
                &manifest[..start],
                "0DDBA11A-0000-4000-8000-000000000009",
                &manifest[end..]
            )
        }
        None => manifest.to_string(),
    }
}

/// Write `files` as the novel's folder and analyse it.
fn analyse(files: &Files) -> (tempfile::TempDir, Analysis) {
    let dir = temp();
    let folder = write_files(files, dir.path(), "Lighthouse.scriv");
    let analysis = run(&folder);
    (dir, analysis)
}

// A1: a copy whose documents never arrived.

/// A sync service copied only the outer folder: `Files/` is there and empty.
/// Importing would give a book of empty documents under a success message.
#[test]
fn a_copy_whose_files_never_synced_is_refused() {
    let spec = novel();
    let mut files = spec.build();
    files.remove_under("Files");
    files.put_dir("Files");
    let (_dir, analysis) = analyse(&files);
    assert_eq!(refusal(analysis), Refusal::IncompleteCopy);
}

/// Only `version.txt` arrived, with no document and no search index to rescue
/// any: many documents and not one file of writing is asked about, never
/// imported silently.
#[test]
fn a_copy_with_no_writing_in_it_asks_before_importing() {
    let mut spec = novel();
    for i in 0..10 {
        spec.items
            .push(Item::new(1, Kind::Text, &format!("Scene {i}")).with_text(i, 1));
    }
    let mut files = spec.build();
    let kept = files
        .get("Files/version.txt")
        .map(<[u8]>::to_vec)
        .unwrap_or_default();
    files.remove_under("Files");
    files.put("Files/version.txt", kept);
    let (dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert!(
        read.pending
            .iter()
            .any(|q| matches!(q, Question::NoTextInCopy { text_items } if *text_items >= 10))
    );
    let accepted = read_answered(
        dir.path(),
        Answers {
            accept_no_text: true,
            ..Answers::default()
        },
    );
    assert!(accepted.pending.is_empty());
}

fn read_answered(dir: &std::path::Path, answers: Answers) -> scrivener_import::ReadProject {
    read(run_with(&dir.join("Lighthouse.scriv"), &answers))
}

/// A project that is genuinely empty (one blank document) is not a partial copy.
#[test]
fn a_genuinely_empty_project_imports_with_a_word_about_it() {
    let mut spec = novel();
    spec.items = vec![
        Item::new(0, Kind::Draft, "Manuscript"),
        Item::new(1, Kind::Text, "Untitled"),
        Item::new(0, Kind::Research, "Research"),
        Item::new(0, Kind::Trash, "Trash"),
    ];
    spec.template_folder = None;
    let (_dir, analysis) = analyse(&spec.build());
    let read = read(analysis);
    assert!(read.pending.is_empty());
    assert!(
        read.notices
            .contains(&Notice::NoTextFound { text_items: 1 })
    );
}

// Missing and truncated writing.

/// A document file is gone but Scrivener's search index still holds its words:
/// they are kept, as plain text flagged as possibly older.
#[test]
fn a_missing_text_is_salvaged_from_the_search_index() {
    let spec = novel();
    let mut files = spec.build();
    let arrival = spec
        .items
        .iter()
        .position(|i| i.title.as_deref() == Some("Arrival"))
        .unwrap_or(0);
    assert!(files.remove(&spec.text_path(arrival)).is_some());
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    let key = key(&read, "Arrival");
    assert!(read.notices.contains(&Notice::ContentMissing {
        item: key,
        role: TextRole::Text,
        salvaged: true
    }));
    let salvaged = row(&read, "Arrival")
        .payload
        .salvaged
        .text
        .clone()
        .unwrap_or_default();
    assert!(salvaged.contains(&spec.items[arrival].text[0]));
}

/// Without a readable index, a text file the checksum list records and the disk
/// does not have is reported, with nothing to salvage.
#[test]
fn a_missing_text_is_reported_from_the_checksum_list_when_there_is_no_index() {
    let spec = novel();
    let mut files = spec.build();
    files.remove("Files/search.indexes");
    let harbour = spec
        .items
        .iter()
        .position(|i| i.title.as_deref() == Some("The Harbour"))
        .unwrap_or(0);
    files.remove(&spec.text_path(harbour));
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert!(read.notices.contains(&Notice::ContentMissing {
        item: key(&read, "The Harbour"),
        role: TextRole::Text,
        salvaged: false
    }));
}

/// A file that is empty on disk although the checksum list recorded content was
/// cut short by a sync.
#[test]
fn a_file_emptied_by_a_sync_is_reported_as_truncated() {
    let spec = novel();
    let mut files = spec.build();
    let harbour = spec
        .items
        .iter()
        .position(|i| i.title.as_deref() == Some("The Harbour"))
        .unwrap_or(0);
    files.put(spec.text_path(harbour), Vec::new());
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert!(read.notices.contains(&Notice::ContentTruncated {
        item: key(&read, "The Harbour"),
        role: TextRole::Text
    }));
}

/// A picture the binder lists and the disk does not have.
#[test]
fn a_missing_picture_is_reported() {
    let spec = novel();
    let mut files = spec.build();
    let map = spec
        .items
        .iter()
        .position(|i| i.kind == Kind::Image)
        .unwrap_or(0);
    files.remove(&format!("{}content.png", spec.payload_prefix(map)));
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert!(read.notices.contains(&Notice::MissingMedia {
        item: key(&read, "Harbour map")
    }));
    assert_eq!(read.project.counts.media_missing, 1);
}

// Several manifests.

/// A corpus project: the live manifest beside six `-conflicted-` copies of it.
/// Scrivener's autosave matches it, it alone carries the folder's name, and the
/// copies list no writing it does not: it is chosen, and the copies named.
#[test]
fn sync_copies_of_the_manifest_are_set_aside_automatically() {
    let spec = novel();
    let mut files = spec.build();
    let manifest = files
        .get("Lighthouse.scrivx")
        .map(<[u8]>::to_vec)
        .unwrap_or_default();
    let older = saved_again(
        &String::from_utf8_lossy(&manifest)
            .replace("<Title>Arrival</Title>", "<Title>Arrival (old)</Title>"),
    );
    let copy = copy_name("Lighthouse.scrivx", CopyScheme::IsoConflicted);
    files.put(copy.clone(), older.into_bytes());
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert_eq!(
        read.project.source.manifest.chosen,
        ManifestChoice::Automatic
    );
    assert!(read.notices.contains(&Notice::ManifestCopiesIgnored {
        chosen: "Lighthouse.scrivx".to_string(),
        copies: vec![copy],
    }));
}

/// Two manifests and nothing to tell them apart: the writer chooses, and
/// nothing is preselected.
#[test]
fn several_manifests_without_evidence_are_asked_about() {
    let spec = novel();
    let mut files = spec.build();
    files.remove("Files/binder.autosave");
    let manifest = files.remove("Lighthouse.scrivx").unwrap_or_default();
    let text = String::from_utf8_lossy(&manifest).into_owned();
    // Neither carries the folder's name, and neither says when it was saved.
    let undated = |t: &str| t.replace("Modified=\"2024-06-02 18:40:12 +0200\" ", "");
    files.put("Draft A.scrivx", undated(&text).into_bytes());
    files.put(
        "Draft B.scrivx",
        undated(&text.replace("<Title>Arrival</Title>", "<Title>Arrival B</Title>")).into_bytes(),
    );
    let (dir, analysis) = analyse(&files);
    let Question::SeveralManifests {
        candidates,
        preselected,
        evidence,
    } = question(analysis)
    else {
        panic!("expected the manifests to be asked about");
    };
    assert_eq!(candidates.len(), 2);
    assert_eq!(preselected, None);
    assert!(evidence.is_empty());
    let answered = read_answered(
        dir.path(),
        Answers {
            manifest: Some("Draft B.scrivx".to_string()),
            ..Answers::default()
        },
    );
    assert_eq!(
        answered.project.source.manifest.chosen,
        ManifestChoice::Answered
    );
    assert!(
        answered
            .project
            .binder
            .iter()
            .any(|r| r.title.as_deref() == Some("Arrival B"))
    );
}

/// The autosave and the folder's name agree, but a copy lists a document the
/// chosen manifest does not, whose file holds writing: ignoring it would lose
/// that writing, so the writer is asked, the evidence preselecting.
#[test]
fn a_manifest_copy_listing_more_writing_is_never_set_aside_silently() {
    let mut spec = novel();
    let mut files = spec.build();
    spec.items
        .push(Item::new(1, Kind::Text, "Only in the copy").with_text(3, 1));
    let richer = spec.build();
    let copy = copy_name("Lighthouse.scrivx", CopyScheme::Dropbox);
    let richer_manifest =
        String::from_utf8_lossy(richer.get("Lighthouse.scrivx").unwrap_or_default()).into_owned();
    files.put(copy, saved_again(&richer_manifest).into_bytes());
    let extra = spec.text_path(spec.items.len() - 1);
    files.put(
        extra.clone(),
        richer.get(&extra).unwrap_or_default().to_vec(),
    );
    let (_dir, analysis) = analyse(&files);
    let Question::SeveralManifests {
        preselected,
        evidence,
        ..
    } = question(analysis)
    else {
        panic!("expected the manifests to be asked about");
    };
    assert_eq!(preselected.as_deref(), Some("Lighthouse.scrivx"));
    assert!(evidence.contains(&ManifestEvidence::MatchesAutosave));
    assert!(evidence.contains(&ManifestEvidence::FolderName));
}

#[test]
fn byte_identical_manifests_are_one_project() {
    let spec = novel();
    let mut files = spec.build();
    let manifest = files
        .get("Lighthouse.scrivx")
        .map(<[u8]>::to_vec)
        .unwrap_or_default();
    files.put("Lighthouse copy.scrivx", manifest);
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert_eq!(
        read.project.source.manifest.chosen,
        ManifestChoice::Identical
    );
    assert_eq!(read.project.source.manifest.name, "Lighthouse.scrivx");
    assert!(
        read.notices
            .contains(&Notice::IdenticalManifestCopies { count: 2 })
    );
}

/// A picked manifest named like a sync copy of the one beside it is still the
/// one read, and the writer is told what it looks like.
#[test]
fn a_picked_sync_copy_is_read_and_named_as_one() {
    let spec = novel();
    let mut files = spec.build();
    let copy = copy_name("Lighthouse.scrivx", CopyScheme::IsoConflicted);
    files.put(
        copy.clone(),
        files.get("Lighthouse.scrivx").unwrap_or_default().to_vec(),
    );
    let dir = temp();
    let folder = write_files(&files, dir.path(), "Lighthouse.scriv");
    let read = read(run(&folder.join(&copy)));
    assert_eq!(read.project.source.manifest.name, copy);
    assert!(
        read.notices
            .contains(&Notice::PinnedManifestLooksLikeACopy {
                name: copy.clone(),
                original: "Lighthouse.scrivx".to_string(),
            })
    );
    assert!(
        read.notices
            .contains(&Notice::PinnedManifestHasSiblings { others: 1 })
    );
}

// The copy Scrivener keeps before an upgrade.

fn backup_pair(dir: &std::path::Path, sibling_stamp: Option<&str>) -> std::path::PathBuf {
    let mut old = Spec::novel(Generation::Format10, Platform::Mac);
    old.name = "Lighthouse Backup".to_string();
    old.manifest_stem = Some("Lighthouse".to_string());
    let backup = write(&old, dir);
    let upgraded = Spec::novel(Generation::Format20, Platform::Mac);
    let mut files = upgraded.build();
    if let Some(stamp) = sibling_stamp {
        files.put("Files/version.txt", stamp);
    }
    write_files(&files, dir, "Lighthouse.scriv");
    backup
}

/// `X Backup.scriv` beside an upgraded `X.scriv`: the writer chooses, the
/// upgraded one preselected because it reads cleanly.
#[test]
fn a_pre_upgrade_copy_offers_its_upgraded_sibling() {
    let dir = temp();
    let backup = backup_pair(dir.path(), None);
    assert_eq!(
        question(run(&backup)),
        Question::UpgradedSibling {
            backup: "Lighthouse Backup.scriv".to_string(),
            sibling: "Lighthouse.scriv".to_string(),
            sibling_generation: Some(ReadGeneration::Format20),
            preselect_sibling: true,
        }
    );
    let sibling = read(run_with(
        &backup,
        &Answers {
            upgraded_sibling: Some(SiblingChoice::Sibling),
            ..Answers::default()
        },
    ));
    assert_eq!(sibling.project.source.generation, ReadGeneration::Format20);
    let kept = read(run_with(
        &backup,
        &Answers {
            upgraded_sibling: Some(SiblingChoice::Backup),
            ..Answers::default()
        },
    ));
    assert_eq!(kept.project.source.generation, ReadGeneration::Format10);
    assert!(kept.notices.contains(&Notice::ImportingPreUpgradeCopy));
}

/// Picking the pre-upgrade copy's manifest and choosing the upgraded sibling
/// reads the sibling's own manifest, whatever the picked one was called.
#[test]
fn choosing_the_sibling_from_a_picked_manifest_reads_the_sibling() {
    let dir = temp();
    let backup = backup_pair(dir.path(), None);
    let picked = backup.join("Lighthouse.scrivx");
    assert!(matches!(
        question(run(&picked)),
        Question::UpgradedSibling { .. }
    ));
    let sibling = read(run_with(
        &picked,
        &Answers {
            upgraded_sibling: Some(SiblingChoice::Sibling),
            ..Answers::default()
        },
    ));
    assert_eq!(sibling.project.source.generation, ReadGeneration::Format20);
    assert_eq!(sibling.project.source.manifest.chosen, ManifestChoice::Only);
}

/// A desktop manifest beside a `Mobile/` folder and no `Files/`: the desktop
/// files are not in this copy, in an archive as in a folder.
#[test]
fn a_desktop_manifest_without_its_files_is_refused_in_an_archive_too() {
    let spec = novel();
    let mut files = spec.build();
    files.remove_under("Files");
    let ipad = Spec::novel(Generation::IosStub, Platform::Ios).build();
    files.put(
        "Mobile/binder.mob",
        ipad.get("Mobile/binder.mob").unwrap_or_default().to_vec(),
    );
    let (_dir, analysis) = analyse(&files);
    assert!(matches!(
        refusal(analysis),
        Refusal::ManifestWithoutFiles { .. }
    ));
    let dir = temp();
    let zip = crate::support::write_file(
        &files.zip(Some("Lighthouse.scriv")).unwrap_or_default(),
        dir.path(),
        "Lighthouse.zip",
    );
    assert!(matches!(
        refusal(run(&zip)),
        Refusal::ManifestWithoutFiles { .. }
    ));
}

/// An upgrade interrupted half-way is when the Backup is the right copy: a
/// sibling whose stamp disagrees with its manifest is offered, not preselected.
#[test]
fn a_sibling_that_does_not_read_cleanly_is_not_preselected() {
    let dir = temp();
    let backup = backup_pair(dir.path(), Some("18"));
    assert!(matches!(
        question(run(&backup)),
        Question::UpgradedSibling {
            preselect_sibling: false,
            ..
        }
    ));
}

// iOS.

/// A project made on an iPad whose `Mobile/` folder did not come along.
#[test]
fn an_ipad_project_without_its_mobile_folder_is_refused() {
    let spec = Spec::novel(Generation::IosStub, Platform::Ios);
    let mut files = spec.build();
    files.remove_under("Mobile");
    files.put_dir("Files");
    let (_dir, analysis) = analyse(&files);
    assert_eq!(refusal(analysis), Refusal::IosStubWithoutMobileData);
    // Without `Files/` either, the stub is a lone manifest: the same refusal.
    let mut bare = spec.build();
    bare.remove_under("Mobile");
    let (_dir, analysis) = analyse(&bare);
    assert_eq!(refusal(analysis), Refusal::IosStubWithoutMobileData);
}

/// Edits an iPad made that no desktop Scrivener merged: import the desktop state
/// without them, or wait. They are never merged here.
#[test]
fn unmerged_ipad_edits_are_asked_about_and_never_merged() {
    let spec = Spec::novel(Generation::Format15, Platform::Mac);
    let mut files = spec.build();
    let ipad = Spec::novel(Generation::IosStub, Platform::Ios).build();
    files.put(
        "Mobile/binder.mob",
        ipad.get("Mobile/binder.mob").unwrap_or_default().to_vec(),
    );
    let (dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert_eq!(
        read.pending,
        vec![Question::UnmergedMobileEdits {
            source: MobileEdits::MobileFolder
        }]
    );
    let accepted = read_answered(
        dir.path(),
        Answers {
            accept_unmerged_mobile_edits: true,
            ..Answers::default()
        },
    );
    assert!(accepted.pending.is_empty());
    assert_eq!(accepted.project.binder.len(), spec.items.len());
}

/// A `Files/binder.mob` older than the manifest is the leftover of a sync long
/// merged; a newer one is edits waiting.
#[test]
fn a_mobile_binder_counts_only_when_newer_than_the_manifest() {
    let spec = novel();
    let ipad = Spec::novel(Generation::IosStub, Platform::Ios).build();
    let mob =
        String::from_utf8_lossy(ipad.get("Mobile/binder.mob").unwrap_or_default()).into_owned();
    let mut stale = spec.build();
    stale.put(
        "Files/binder.mob",
        mob.replace("2024-06-02 18:40:12 +0200", "2019-01-01 10:00:00 +0000"),
    );
    let (_dir, analysis) = analyse(&stale);
    let read_stale = read(analysis);
    assert!(
        read_stale
            .notices
            .contains(&Notice::StaleMobileBinderIgnored)
    );
    assert!(read_stale.pending.is_empty());
    let mut newer = spec.build();
    newer.put(
        "Files/binder.mob",
        mob.replace("2024-06-02 18:40:12 +0200", "2030-01-01 10:00:00 +0000"),
    );
    let (_dir, analysis) = analyse(&newer);
    assert_eq!(
        read(analysis).pending,
        vec![Question::UnmergedMobileEdits {
            source: MobileEdits::NewerMobileBinder
        }]
    );
}

// The lock.

/// A lock never blocks (crashes leave them, and git keeps them for years); it
/// says whether it names this computer, and nothing personal goes into what the
/// import keeps.
#[test]
fn a_lock_is_reported_and_its_host_kept_off_the_record() {
    let mut spec = novel();
    spec.lock_host = Some("Test-Mac".to_string());
    let (_dir, analysis) = analyse(&spec.build());
    let read_here = read(analysis);
    // The tests run as "Test-Mac.local"; the lock says "Test-Mac".
    assert!(HOST.starts_with("Test-Mac"));
    assert!(
        read_here
            .notices
            .contains(&Notice::LockPresent { this_machine: true })
    );
    assert_eq!(read_here.card.lock_host.as_deref(), Some("Test-Mac"));

    spec.lock_host = Some("SOMEONE-ELSE".to_string());
    let (_dir, analysis) = analyse(&spec.build());
    let read_elsewhere = read(analysis);
    assert!(read_elsewhere.notices.contains(&Notice::LockPresent {
        this_machine: false
    }));
    // What persists (the model) and what is reported (the notices) never carry
    // the host, the user, or the device the manifest names.
    let kept = serde_json::to_string(&(&read_elsewhere.project, &read_elsewhere.notices))
        .unwrap_or_default();
    for personal in ["SOMEONE-ELSE", "Test Writer", "Test-Mac", "/Users/test"] {
        assert!(!kept.contains(personal), "{personal} leaked");
    }
    assert_eq!(read_elsewhere.card.device.as_deref(), Some("Test-Mac"));
}

/// Zipped while Scrivener had it open: the lock came along.
#[test]
fn a_lock_inside_an_archive_says_it_was_archived_while_open() {
    let mut spec = novel();
    spec.lock_host = Some("Test-Mac".to_string());
    let dir = temp();
    let zip = crate::support::write_file(
        &spec.build().zip(Some(&spec.folder())).unwrap_or_default(),
        dir.path(),
        "Lighthouse.zip",
    );
    assert!(read(run(&zip)).notices.contains(&Notice::ArchivedWhileOpen));
}

// Sync debris.

/// Every naming scheme a sync service uses for its copy of a document is
/// recognised when the original is there, and the copy comes across as a
/// recovered text tied to its item, never dropped.
#[test]
fn every_sync_copy_of_a_document_is_recovered() {
    for scheme in CopyScheme::ALL {
        let spec = novel();
        let mut files = spec.build();
        let arrival = spec
            .items
            .iter()
            .position(|i| i.title.as_deref() == Some("Arrival"))
            .unwrap_or(0);
        let prefix = spec.payload_prefix(arrival);
        let copy = format!("{prefix}{}", copy_name("content.rtf", scheme));
        files.put(copy.clone(), b"{\\rtf1 a divergent edit}".to_vec());
        let (_dir, analysis) = analyse(&files);
        let read = read(analysis);
        let recovered: Vec<_> = read
            .project
            .recovered
            .iter()
            .filter(|r| r.member.path == copy)
            .collect();
        assert_eq!(
            recovered.len(),
            1,
            "{scheme:?}: {:?}",
            read.project.recovered
        );
        assert!(
            matches!(recovered[0].origin, RecoveredOrigin::SyncConflict(_)),
            "{scheme:?}"
        );
        assert_eq!(
            recovered[0].names,
            Some(key(&read, "Arrival")),
            "{scheme:?}"
        );
        assert_eq!(recovered[0].role, TextRole::Text);
        assert!(
            read.notices.contains(&Notice::SyncConflictCopies {
                count: 1,
                original_missing: 0
            }),
            "{scheme:?}"
        );
    }
}

/// A copy whose original is gone may be the only copy of that text: recovered,
/// and said to be an orphan.
#[test]
fn a_sync_copy_whose_original_is_gone_is_recovered_as_an_orphan() {
    let spec = Spec::novel(Generation::Format15, Platform::Windows);
    let mut files = spec.build();
    let harbour = spec
        .items
        .iter()
        .position(|i| i.title.as_deref() == Some("The Harbour"))
        .unwrap_or(0);
    let original = spec.text_path(harbour);
    let text = files.remove(&original).unwrap_or_default();
    let orphan = original.replace(".rtf", "_2.rtf");
    files.put(orphan.clone(), text);
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    let recovered: Vec<_> = read
        .project
        .recovered
        .iter()
        .filter(|r| r.member.path == orphan)
        .collect();
    assert_eq!(recovered.len(), 1);
    assert_eq!(
        recovered[0].origin,
        RecoveredOrigin::OrphanCopy(ConflictScheme::Underscore)
    );
    assert_eq!(recovered[0].names, Some(key(&read, "The Harbour")));
    assert!(read.notices.contains(&Notice::SyncConflictCopies {
        count: 1,
        original_missing: 1
    }));
}

/// Writing no binder item lists (a document a deleted item left, a folder of a
/// spare binder's items) comes across as recovered text; an empty file is only
/// counted.
#[test]
fn writing_no_item_lists_is_recovered_not_dropped() {
    let spec = novel();
    let mut files = spec.build();
    let stray = "Files/Data/0BADF00D-0000-4000-8000-000000000001";
    files.put(
        format!("{stray}/content.rtf"),
        b"{\\rtf1 a scene nobody lists}".to_vec(),
    );
    files.put(format!("{stray}/synopsis.txt"), Vec::new());
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    let recovered: Vec<_> = read
        .project
        .recovered
        .iter()
        .filter(|r| r.member.path.starts_with(stray))
        .collect();
    assert_eq!(recovered.len(), 1);
    assert_eq!(recovered[0].origin, RecoveredOrigin::Unreferenced);
    assert!(
        read.notices
            .contains(&Notice::RecoveredUnreferencedText { count: 1 })
    );
    assert_eq!(read.project.counts.unreferenced_empty_files, 1);
}

/// Copies of Scrivener's own bookkeeping hold no writing: counted, not carried.
/// Litter is not even counted; anything else is named as unrecognised.
#[test]
fn debris_is_sorted_from_writing() {
    let spec = novel();
    let mut files = spec.build();
    files.put(
        format!(
            "Files/{}",
            copy_name("binder.autosave", CopyScheme::Numbered)
        ),
        b"PK\x05\x06".to_vec(),
    );
    files.put(
        format!(
            "Files/{}",
            copy_name("search.indexes", CopyScheme::BracketConflict)
        ),
        b"<x/>".to_vec(),
    );
    files.put("Files/.DS_Store", b"\0\0\0\x01Bud1".to_vec());
    files.put("Files/Data/Thumbs.db", b"\0".to_vec());
    files.put(".gitignore", b"Files/user.lock\n".to_vec());
    files.put("Files/Chapter 3 draft.docx", b"PK\x03\x04".to_vec());
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert!(
        read.notices
            .contains(&Notice::SyncDebrisIgnored { count: 2 })
    );
    assert!(read.notices.contains(&Notice::UnrecognisedFiles {
        count: 1,
        examples: vec!["Files/Chapter 3 draft.docx".to_string()],
    }));
    assert!(read.project.recovered.is_empty());
}

/// Scrivener 1 files left beside a later project (an upgrade that never cleaned
/// up) are set aside, and said to be.
#[test]
fn scrivener_one_leftovers_beside_a_later_project_are_set_aside() {
    let spec = Spec::novel(Generation::Format10, Platform::Mac);
    let mut files = spec.build();
    let old = Spec::novel(Generation::ScrivenerOne, Platform::Mac).build();
    files.merge(&old);
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert_eq!(read.project.source.generation, ReadGeneration::Format10);
    assert!(
        read.notices
            .contains(&Notice::LegacyV1FilesBeside { count: 4 })
    );
}

/// Both document layouts present: files are looked for where the manifest's
/// identifiers point, and the other tree's writing is recovered, not lost.
#[test]
fn a_half_upgraded_bundle_reads_by_its_manifest() {
    let spec = Spec::novel(Generation::Format15, Platform::Mac);
    let mut files = spec.build();
    let upgraded = novel().build();
    for path in upgraded.paths_under("Files/Data") {
        files.put(
            path.clone(),
            upgraded.get(&path).unwrap_or_default().to_vec(),
        );
    }
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert!(read.notices.contains(&Notice::MixedPayloadTrees));
    assert!(
        row(&read, "Arrival")
            .payload
            .text
            .as_ref()
            .is_some_and(|t| t.path.starts_with("Files/Docs/"))
    );
    assert!(!read.project.recovered.is_empty());
}

// Identifiers.

/// S4: two items with one key. The first owns it and its files; the second keeps
/// its place and gets none, and the writer is told.
#[test]
fn a_duplicate_item_key_is_owned_by_the_first() {
    let spec = novel();
    let mut files = spec.build();
    let manifest =
        String::from_utf8_lossy(files.get("Lighthouse.scrivx").unwrap_or_default()).into_owned();
    let arrival = spec.uuid(
        spec.items
            .iter()
            .position(|i| i.title.as_deref() == Some("Arrival"))
            .unwrap_or(0),
    );
    let harbour = spec.uuid(
        spec.items
            .iter()
            .position(|i| i.title.as_deref() == Some("The Harbour"))
            .unwrap_or(0),
    );
    files.put("Lighthouse.scrivx", manifest.replace(&harbour, &arrival));
    files.remove("Files/binder.autosave");
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    let first = row(&read, "Arrival");
    let second = row(&read, "The Harbour");
    assert!(first.owns_key && first.payload.text.is_some());
    assert!(!second.owns_key && second.payload.text.is_none());
    assert_eq!(first.key, second.key);
    assert!(
        read.notices
            .iter()
            .any(|n| matches!(n, Notice::DuplicateItemKey { .. }))
    );
}

/// An identifier becomes part of a path, so anything but Scrivener's own shape is
/// dropped: the item keeps its place and gets no files.
#[test]
fn identifiers_that_are_not_identifiers_are_dropped() {
    let spec = novel();
    let mut files = spec.build();
    let manifest =
        String::from_utf8_lossy(files.get("Lighthouse.scrivx").unwrap_or_default()).into_owned();
    let harbour = spec.uuid(
        spec.items
            .iter()
            .position(|i| i.title.as_deref() == Some("The Harbour"))
            .unwrap_or(0),
    );
    files.put(
        "Lighthouse.scrivx",
        manifest.replace(&harbour, "../../../../etc/passwd"),
    );
    files.remove("Files/binder.autosave");
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert!(
        read.notices
            .contains(&Notice::InvalidItemIdentifier { count: 1 })
    );
    assert!(
        read.notices
            .contains(&Notice::ItemsWithoutIdentifier { count: 1 })
    );
    assert_eq!(row(&read, "The Harbour").key, None);
    // Its writing is still on disk, and no item lists it now: recovered.
    assert!(read.project.recovered.iter().any(|r| r.names
        == Some(ItemKey::Uuid(
            uuid::Uuid::parse_str(&harbour).unwrap_or_default()
        ))));
}

#[test]
fn a_binder_with_no_usable_identifier_is_refused() {
    let manifest = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ScrivenerProject Version=\"2.0\"><Binder><BinderItem UUID=\"x\" Type=\"DraftFolder\"/><BinderItem Type=\"Text\"/></Binder></ScrivenerProject>\n";
    let mut files = Files::new();
    files.put("Lighthouse.scrivx", manifest);
    files.put("Files/version.txt", "23");
    files.put_dir("Files/Data");
    let (_dir, analysis) = analyse(&files);
    assert_eq!(refusal(analysis), Refusal::NoItemIdentifiers);
}

/// A binder whose identifiers are not its version's: files are looked for where
/// the identifiers point, and the writer told.
#[test]
fn identifiers_decide_where_files_are_when_the_version_disagrees() {
    let spec = Spec::novel(Generation::Format10, Platform::Mac);
    let mut files = spec.build();
    let manifest =
        String::from_utf8_lossy(files.get("Lighthouse.scrivx").unwrap_or_default()).into_owned();
    files.put(
        "Lighthouse.scrivx",
        manifest.replace("Version=\"1.0\"", "Version=\"2.0\""),
    );
    files.put("Files/version.txt", "23");
    files.remove("Files/binder.autosave");
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert!(read.notices.contains(&Notice::IdSchemeMismatch {
        version: "2.0".to_string()
    }));
    assert!(
        row(&read, "Arrival")
            .payload
            .text
            .as_ref()
            .is_some_and(|t| t.path.starts_with("Files/Docs/"))
    );
}

// The version gate.

#[test]
fn a_newer_format_is_refused_and_an_unreadable_stamp_too() {
    let spec = novel();
    let mut newer = spec.build();
    newer.put("Files/version.txt", "24");
    let (_dir, analysis) = analyse(&newer);
    assert_eq!(
        refusal(analysis),
        Refusal::NewerFormatStamp {
            stamp: 24,
            newest: 23
        }
    );

    let mut garbled = spec.build();
    garbled.put("Files/version.txt", "twenty-three");
    let (_dir, analysis) = analyse(&garbled);
    assert_eq!(refusal(analysis), Refusal::VersionStampUnreadable);

    let mut future = spec.build();
    let manifest =
        String::from_utf8_lossy(future.get("Lighthouse.scrivx").unwrap_or_default()).into_owned();
    future.put(
        "Lighthouse.scrivx",
        manifest.replace("Version=\"2.0\"", "Version=\"3.0\""),
    );
    future.remove("Files/binder.autosave");
    let (_dir, analysis) = analyse(&future);
    assert_eq!(
        refusal(analysis),
        Refusal::UnknownFormatVersion {
            version: "3.0".to_string()
        }
    );

    let mut odd = spec.build();
    odd.put("Files/version.txt", "18");
    let (_dir, analysis) = analyse(&odd);
    assert!(
        read(analysis)
            .notices
            .contains(&Notice::VersionStampMismatch {
                stamp: 18,
                version: "2.0".to_string()
            })
    );
}

// A damaged manifest (refused until spare binders are read, milestone S5).

#[test]
fn a_damaged_manifest_is_refused_naming_the_damage() {
    let spec = novel();
    let mut broken = spec.build();
    broken.put(
        "Lighthouse.scrivx",
        "<?xml version=\"1.0\"?><ScrivenerProject Version=\"2.0\"><Binder><BinderItem",
    );
    let (_dir, analysis) = analyse(&broken);
    assert!(matches!(
        refusal(analysis),
        Refusal::DamagedManifest {
            damage: ManifestDamage::NotXml { .. },
            spare_copies: 2,
            ..
        }
    ));

    let mut placeholder = Spec::novel(Generation::Format10, Platform::Mac);
    placeholder.backup = Backup::Placeholder;
    let built = placeholder.build();
    let mut files = built.clone();
    // The live manifest replaced by the placeholder a spare binder can be.
    files.put(
        "Lighthouse.scrivx",
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ScrivenerProject Version=\"1.0\" Creator=\"SCRMAC/2.5/25238\" NewProject=\"Yes\"></ScrivenerProject>\n",
    );
    let (_dir, analysis) = analyse(&files);
    assert!(matches!(
        refusal(analysis),
        Refusal::DamagedManifest {
            damage: ManifestDamage::NewProjectPlaceholder,
            ..
        }
    ));
}

/// G1: a manifest nested past any real one is refused before a parser sees it,
/// on the stack a long operation gets, and one just under the ceiling reads.
#[test]
fn a_deep_manifest_is_refused_without_taking_the_process_down() {
    let deep = on_long_operation_stack(|| {
        let mut spec = novel();
        spec.items = vec![Item::new(0, Kind::Draft, "Manuscript")];
        for depth in 1..200 {
            spec.items.push(Item::new(depth, Kind::Folder, "Level"));
        }
        spec.template_folder = None;
        let (_dir, analysis) = analyse(&spec.build());
        analysis
    });
    assert!(matches!(
        refusal(deep),
        Refusal::DamagedManifest {
            damage: ManifestDamage::TooDeep { .. },
            ..
        }
    ));
    let under = on_long_operation_stack(|| {
        let mut spec = novel();
        spec.items = vec![Item::new(0, Kind::Draft, "Manuscript")];
        for depth in 1..120 {
            spec.items
                .push(Item::new(depth, Kind::Folder, "Level").with_text(depth as usize, 1));
        }
        spec.template_folder = None;
        let (_dir, analysis) = analyse(&spec.build());
        analysis
    });
    let read = read(under);
    assert_eq!(read.project.binder.len(), 120);
    assert_eq!(read.project.binder.last().map(|r| r.depth), Some(119));
    // G4: flat, so the deep model survives JSON, which caps nesting at 128.
    let json = serde_json::to_string(&read.project).unwrap_or_default();
    let back: Result<scrivener_import::model::Project, _> = serde_json::from_str(&json);
    assert!(back.is_ok_and(|p| p == read.project));
}

// Cloud placeholders by name.

/// iCloud evicted a document and left its hidden stub: the import needs it, so
/// the writer is asked to download the project first.
#[test]
fn a_document_left_in_the_cloud_refuses_the_import() {
    let spec = novel();
    let mut files = spec.build();
    let arrival = spec
        .items
        .iter()
        .position(|i| i.title.as_deref() == Some("Arrival"))
        .unwrap_or(0);
    let text = spec.text_path(arrival);
    files.remove(&text);
    let prefix = spec.payload_prefix(arrival);
    files.put(
        format!("{prefix}{}", icloud_stub("content.rtf")),
        b"bplist00".to_vec(),
    );
    let (_dir, analysis) = analyse(&files);
    assert_eq!(
        refusal(analysis),
        Refusal::CloudPlaceholders {
            count: 1,
            examples: vec![text],
        }
    );
}

/// A picture left in the cloud is not needed to read the project: skipped and
/// counted.
#[test]
fn a_picture_left_in_the_cloud_is_skipped() {
    let spec = Spec::novel(Generation::Format15, Platform::Linux);
    let mut files = spec.build();
    let map = spec
        .items
        .iter()
        .position(|i| i.kind == Kind::Image)
        .unwrap_or(0);
    let picture = format!("{}.png", spec.payload_prefix(map));
    files.remove(&picture);
    files.put(nextcloud_stub(&picture), Vec::new());
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert!(
        read.notices
            .iter()
            .all(|n| !matches!(n, Notice::MissingMedia { .. }))
    );
    assert!(
        read.notices
            .contains(&Notice::CloudPlaceholdersSkipped { count: 1 })
    );
    assert!(row(&read, "Harbour map").payload.file.is_none());
}

/// A project whose folders were renamed to lower case on a case-sensitive disk
/// (the Linux beta, Wine) still finds its files.
#[test]
fn case_differences_do_not_hide_files() {
    let spec = novel();
    let files = spec.build();
    let mut lower = Files::new();
    for (path, bytes) in files.iter() {
        let renamed = path.replacen("Files/Data/", "files/data/", 1);
        lower.put(renamed, bytes.to_vec());
    }
    let (_dir, analysis) = analyse(&lower);
    let read = read(analysis);
    assert!(row(&read, "Arrival").payload.text.is_some());
}

/// Snapshots are not imported; how many Scrivener keeps is counted for the
/// report, and a snapshot folder naming no item too.
#[test]
fn snapshots_are_counted_not_read() {
    let spec = novel();
    let mut files = spec.build();
    files.put(
        "Snapshots/0BADF00D-0000-4000-8000-000000000002.snapshots/2020-01-01-10-00-00+0000.rtf",
        b"{\\rtf1 old}".to_vec(),
    );
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    assert_eq!(row(&read, "Arrival").snapshots, 2);
    assert_eq!(read.project.counts.snapshots, 2);
    assert_eq!(read.project.counts.orphan_snapshot_folders, 1);
}

/// The fingerprint tells an unchanged project from a changed one, so an import
/// can tell the project it analysed is the one it is about to read.
#[test]
fn the_fingerprint_follows_the_project() {
    let spec = novel();
    let dir = temp();
    let folder = write(&spec, dir.path());
    let first = read(run(&folder)).fingerprint;
    assert_eq!(read(run(&folder)).fingerprint, first);
    let arrival = spec
        .items
        .iter()
        .position(|i| i.title.as_deref() == Some("Arrival"))
        .unwrap_or(0);
    let _ = std::fs::write(
        folder.join(spec.text_path(arrival)),
        b"{\\rtf1 rewritten, and longer than before}",
    );
    assert_ne!(read(run(&folder)).fingerprint, first);
}

/// Rewrite `path` with other bytes of the same length, its time put back: what a copy that
/// keeps times (a restore, a sync) does to a file whose content changed.
fn same_size_and_time(path: &std::path::Path) {
    let before = std::fs::metadata(path).and_then(|m| m.modified()).ok();
    let mut bytes = std::fs::read(path).unwrap_or_default();
    let Some(last) = bytes.len().checked_sub(1) else {
        panic!("{} is empty", path.display());
    };
    bytes[last] = if bytes[last] == b' ' { b'\t' } else { b' ' };
    if let Err(e) = std::fs::write(path, &bytes) {
        panic!("{e}");
    }
    if let Some(before) = before
        && let Ok(file) = std::fs::File::options().write(true).open(path)
    {
        let _ = file.set_modified(before);
    }
}

/// The fingerprint follows what the import reads, not when the files were last touched:
/// the same bytes written again, or the lock Scrivener writes while it has the project
/// open, are no reason to review it again; other bytes behind an unchanged size and time
/// are one.
#[test]
fn the_fingerprint_follows_content_rather_than_times() {
    let spec = novel();
    let dir = temp();
    let folder = write(&spec, dir.path());
    let first = read(run(&folder)).fingerprint;
    let Some(arrival) = spec.index_of("Arrival") else {
        panic!("no Arrival");
    };
    let text = folder.join(spec.text_path(arrival));
    let bytes = std::fs::read(&text).unwrap_or_default();
    if let Ok(file) = std::fs::File::options().write(true).open(&text) {
        let _ =
            file.set_modified(std::time::SystemTime::now() + std::time::Duration::from_secs(60));
    }
    let _ = std::fs::write(&text, &bytes);
    assert_eq!(
        read(run(&folder)).fingerprint,
        first,
        "the same bytes, a later time"
    );

    let lock = folder.join("Files/user.lock");
    let _ = std::fs::write(
        &lock,
        b"<?xml version=\"1.0\"?><Lock><Host>Other.local</Host></Lock>",
    );
    assert_eq!(read(run(&folder)).fingerprint, first, "a lock appeared");
    let _ = std::fs::remove_file(&lock);

    for member in ["content.rtf", "content.comments", "synopsis.txt"] {
        let before = read(run(&folder)).fingerprint;
        same_size_and_time(&folder.join(format!("{}{member}", spec.payload_prefix(arrival))));
        assert_ne!(
            read(run(&folder)).fingerprint,
            before,
            "{member}: other bytes, the same size and time"
        );
    }
}

/// Answers that do not apply are ignored rather than trusted: a project name
/// the pick does not hold still asks.
#[test]
fn an_answer_naming_nothing_is_not_trusted() {
    let dir = temp();
    write(&novel(), dir.path());
    let mut other = novel();
    other.name = "Second".to_string();
    write(&other, dir.path());
    let asked = question(run_with(
        dir.path(),
        &Answers {
            project: Some("../elsewhere".to_string()),
            ..Answers::default()
        },
    ));
    assert!(matches!(asked, Question::SeveralProjects { .. }));
}

/// The card may show the device a manifest names and the host a lock names;
/// nothing else may, and a `Debug` print of an analysis is the first thing that
/// reaches a log line.
#[test]
fn the_card_s_personal_data_stays_out_of_debug_output() {
    let mut spec = novel();
    spec.lock_host = Some("SOMEONE-ELSE".to_string());
    let (_dir, analysis) = analyse(&spec.build());
    let printed = format!("{analysis:?}");
    for personal in ["SOMEONE-ELSE", "Test-Mac"] {
        assert!(
            !printed.contains(personal),
            "{personal} is in the Debug output"
        );
    }
    let read = read(analysis);
    assert_eq!(read.card.lock_host.as_deref(), Some("SOMEONE-ELSE"));
    assert_eq!(read.card.device.as_deref(), Some("Test-Mac"));
    // This computer's own name, which the analysis only compares, is withheld
    // the same way from whoever prints the options it was given.
    assert!(!format!("{:?}", crate::support::options()).contains(HOST));
}

/// A picture item holding a text file of its own (a Windows 1.8 project in the
/// corpus has one, 625 words): the text is the item's, and reaches the model with
/// it rather than vanishing because pictures are not supposed to have any.
#[test]
fn a_picture_item_s_own_text_is_not_dropped() {
    for (generation, platform) in [
        (Generation::Format10, Platform::Windows),
        (Generation::Format20, Platform::Mac),
    ] {
        let spec = Spec::novel(generation, platform);
        let mut files = spec.build();
        let map = spec
            .items
            .iter()
            .position(|i| i.kind == Kind::Image)
            .unwrap_or(0);
        let text = spec.text_path(map);
        files.put(
            text.clone(),
            b"{\\rtf1 words kept with the picture}".to_vec(),
        );
        let (_dir, analysis) = analyse(&files);
        let read = read(analysis);
        let row = row(&read, "Harbour map");
        assert_eq!(
            row.payload.text.as_ref().map(|m| m.path.as_str()),
            Some(text.as_str()),
            "{generation:?}"
        );
        assert!(row.payload.file.is_some(), "{generation:?}");
        assert!(
            read.project.recovered.iter().all(|r| r.member.path != text),
            "{generation:?}"
        );
    }
}

/// A second file for a document, spelled differently only in case (a project
/// merged onto a disk that tells `A` from `a`): the one Scrivener's spelling names
/// is read, and the other is a divergent copy, recovered rather than dropped.
#[test]
fn a_second_spelling_of_a_document_is_recovered_not_dropped() {
    for generation in [Generation::Format20, Generation::Format15] {
        let spec = Spec::novel(generation, Platform::Windows);
        let mut files = spec.build();
        let arrival = spec
            .items
            .iter()
            .position(|i| i.title.as_deref() == Some("Arrival"))
            .unwrap_or(0);
        let text = spec.text_path(arrival);
        let variant = match generation {
            Generation::Format20 => {
                text.replace(&spec.uuid(arrival), &spec.uuid(arrival).to_lowercase())
            }
            _ => text.replace(".rtf", ".RTF"),
        };
        assert_ne!(variant, text);
        files.put(
            variant.clone(),
            b"{\\rtf1 the other machine's version}".to_vec(),
        );
        // A zip: two spellings of one name cannot both be written to a
        // case-insensitive disk, which is where these tests also run.
        let dir = temp();
        let zip = crate::support::write_file(
            &files.zip(Some("Lighthouse.scriv")).unwrap_or_default(),
            dir.path(),
            "Lighthouse.zip",
        );
        let read = read(run(&zip));
        assert_eq!(
            row(&read, "Arrival")
                .payload
                .text
                .as_ref()
                .map(|m| m.path.as_str()),
            Some(text.as_str()),
            "{generation:?}"
        );
        let recovered: Vec<_> = read
            .project
            .recovered
            .iter()
            .filter(|r| r.member.path == variant)
            .collect();
        assert_eq!(
            recovered.len(),
            1,
            "{generation:?}: {:?}",
            read.project.recovered
        );
        assert_eq!(
            recovered[0].origin,
            RecoveredOrigin::SyncConflict(ConflictScheme::Spelling),
            "{generation:?}"
        );
        assert_eq!(
            recovered[0].names,
            Some(key(&read, "Arrival")),
            "{generation:?}"
        );
        assert!(
            read.notices.contains(&Notice::SyncConflictCopies {
                count: 1,
                original_missing: 0
            }),
            "{generation:?}"
        );
    }
}

/// A format 1.x project note whose `index.xml` did not come along (a Windows 1.x
/// project in the corpus has one, 106 words): the note is still the project's,
/// carried without the title only the index held, never dropped.
#[test]
fn a_project_note_the_index_does_not_list_is_carried() {
    let spec = Spec::novel(Generation::Format10, Platform::Windows);
    let mut files = spec.build();
    assert!(files.remove("Files/ProjectNotes/index.xml").is_some());
    let (_dir, analysis) = analyse(&files);
    let read = read(analysis);
    let carried: Vec<_> = read
        .project
        .project_notes
        .iter()
        .filter_map(|note| note.file.as_ref().map(|file| (note.id, file.path.as_str())))
        .collect();
    assert_eq!(
        carried,
        vec![(Some(10), "Files/ProjectNotes/Note-10.rtf")],
        "{:?}",
        read.project.project_notes
    );
}
