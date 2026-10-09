// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The project owner's ground truth: real projects Scrivener 3.5.2 for Mac wrote, in
//! `tests/fixtures/ground-truth-mac-3.5.2/` (see its `NOTICE`), made with the owner's own
//! words to settle what Scrivener really writes. Every other test reads a fixture this
//! project built from its own reading of the format; these read Scrivener's.
//!
//! One test per change the ground truth asked for (C1 to C20 of the session's change
//! list), each against the committed files, and one that keeps them free of what the
//! scrub removed.

use std::io::Read;
use std::path::{Path, PathBuf};

use base64::Engine;
use common::entities::ContentRole;
use scrivener_import::metadata::{DeadlineReason, MetadataNotice, TargetUnit, TitleSource};
use scrivener_import::model::Container;
use scrivener_import::outcome::BackupSign;
use scrivener_import::prose::ProseNotice;
use scrivener_import::structure::StructureNotice;
use scrivener_import::{Notice, Question, Summary};
use skrib_format::WorkBundle;

use crate::convert::{addressable, converted, item, items, prose, read};
use crate::support::{question, read as read_project, run, temp};

/// The scrubbed device name every manifest and lock carries.
const DEVICE: &str = "Writers-Mac";

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ground-truth-mac-3.5.2")
}

fn fixture(name: &str) -> PathBuf {
    fixtures().join(name)
}

/// Convert one fixture; its summary and the project written.
fn import(name: &str) -> (tempfile::TempDir, Summary, WorkBundle) {
    let dir = temp();
    let out = dir.path().join("imported.skrib");
    let summary = converted(&fixture(name), &out);
    let bundle = read(&out);
    (dir, summary, bundle)
}

fn metadata_has(summary: &Summary, wanted: impl Fn(&MetadataNotice) -> bool) -> bool {
    summary.metadata.iter().any(wanted)
}

fn prose_has(summary: &Summary, wanted: impl Fn(&ProseNotice) -> bool) -> bool {
    summary.prose.iter().any(wanted)
}

/// The tags a row wears, by name.
fn tags_of(bundle: &WorkBundle, title: &str) -> Vec<String> {
    item(bundle, title)
        .item
        .tag_ids
        .iter()
        .filter_map(|id| bundle.tags.iter().find(|t| t.file_id == *id))
        .map(|t| t.name.clone())
        .collect()
}

/// What a reader sees of a row's note text.
fn note_text(bundle: &WorkBundle, title: &str) -> String {
    let djot = prose(item(bundle, title), ContentRole::NoteText);
    skrib_format::djot_plain_text(&djot)
        .map(|(text, _)| text)
        .unwrap_or_default()
}

// ── what the scrub removed stays removed ─────────────────────────────────────────────

/// Every file of the fixtures, and every file inside each zip, binder copy and template
/// payload, by a name that says where it was found.
fn every_member() -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    let mut pending: Vec<PathBuf> = vec![fixtures()];
    while let Some(dir) = pending.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(e) => panic!("reading {}: {e}", dir.display()),
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            let name = path
                .strip_prefix(fixtures())
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();
            match std::fs::read(&path) {
                Ok(bytes) => expand(name, bytes, &mut out),
                Err(e) => panic!("reading {name}: {e}"),
            }
        }
    }
    out
}

fn expand(name: String, bytes: Vec<u8>, out: &mut Vec<(String, Vec<u8>)>) {
    if bytes.starts_with(b"PK\x03\x04") {
        let mut archive = match zip::ZipArchive::new(std::io::Cursor::new(bytes.clone())) {
            Ok(archive) => archive,
            Err(e) => panic!("{name} is no zip: {e}"),
        };
        assert!(archive.comment().is_empty(), "{name} carries a zip comment");
        for i in 0..archive.len() {
            let mut member = match archive.by_index(i) {
                Ok(member) => member,
                Err(e) => panic!("{name} member {i}: {e}"),
            };
            let inner = format!("{name}!{}", member.name());
            out.push((format!("{inner} (name)"), member.name().as_bytes().to_vec()));
            let mut data = Vec::new();
            if let Err(e) = member.read_to_end(&mut data) {
                panic!("{inner}: {e}");
            }
            drop(member);
            expand(inner, data, out);
        }
    } else if bytes.windows(18).any(|w| w == b"<ScrivenerTemplate") {
        let text = String::from_utf8_lossy(&bytes).into_owned();
        let payload = text
            .split("<ProjectZipData Type=\"Base64\">")
            .nth(1)
            .and_then(|rest| rest.split("</ProjectZipData>").next())
            .unwrap_or_default();
        match base64::engine::general_purpose::STANDARD.decode(payload.trim()) {
            Ok(zip) => expand(format!("{name}!payload"), zip, out),
            Err(e) => panic!("{name}: the payload is no base64: {e}"),
        }
    }
    out.push((name, bytes));
}

/// The values of every `key="…"` attribute in `text`.
fn attribute_values<'t>(text: &'t str, key: &str) -> Vec<&'t str> {
    let open = format!("{key}=\"");
    text.match_indices(&open)
        .filter_map(|(at, _)| {
            let rest = &text[at + open.len()..];
            rest.find('"').map(|end| &rest[..end])
        })
        .collect()
}

/// The guard against anyone putting the raw handover back: the device and the lock's host
/// are the scrubbed name, the only home folder is the scrubbed one, and none of the files
/// the Finder adds (or what it records about downloads) is anywhere.
#[test]
fn ground_truth_holds_no_personal_data() {
    let members = every_member();
    assert!(members.len() > 100, "{} files", members.len());
    let mut devices = 0;
    let mut hosts = 0;
    for (name, bytes) in &members {
        let text = String::from_utf8_lossy(bytes);
        for forbidden in [
            "__MACOSX",
            ".DS_Store",
            "QuickLook",
            "kMDItem",
            "com.apple.",
        ] {
            assert!(!name.contains(forbidden), "{name}: {forbidden}");
            assert!(!text.contains(forbidden), "{name} holds {forbidden}");
        }
        for device in attribute_values(&text, "Device") {
            assert_eq!(device, DEVICE, "{name}");
            devices += 1;
        }
        for line in text.lines() {
            if let Some(host) = line.strip_prefix("host=") {
                assert_eq!(host, DEVICE, "{name}");
                hosts += 1;
            }
        }
        for (at, _) in text.match_indices("/Users/") {
            assert!(
                text[at..].starts_with("/Users/writer/"),
                "{name}: a home folder other than the scrubbed one"
            );
        }
        if bytes.starts_with(b"%PDF") {
            assert!(
                !text.contains("/Info") && !text.contains("/Metadata"),
                "{name}: a PDF with metadata"
            );
        }
    }
    assert!(devices >= 20, "every manifest names its device: {devices}");
    assert_eq!(hosts, 2, "the two locks");
}

// ── the bytes Scrivener wrote ────────────────────────────────────────────────────────

/// SHA-1, the digest `docs.checksum` records, for the few files checked here.
fn sha1(bytes: &[u8]) -> String {
    let mut h: [u32; 5] = [
        0x6745_2301,
        0xEFCD_AB89,
        0x98BA_DCFE,
        0x1032_5476,
        0xC3D2_E1F0,
    ];
    let mut message = bytes.to_vec();
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&((bytes.len() as u64).wrapping_mul(8)).to_be_bytes());
    for block in message.as_chunks::<64>().0 {
        let mut w = [0u32; 80];
        for (i, word) in block.as_chunks::<4>().0.iter().enumerate() {
            w[i] = u32::from_be_bytes(*word);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }
        let [mut a, mut b, mut c, mut d, mut e] = h;
        for (i, word) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => ((b & c) | (!b & d), 0x5A82_7999),
                20..=39 => (b ^ c ^ d, 0x6ED9_EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC),
                _ => (b ^ c ^ d, 0xCA62_C1D6),
            };
            let t = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(*word);
            (e, d, c, b, a) = (d, c, b.rotate_left(30), a, t);
        }
        for (slot, value) in h.iter_mut().zip([a, b, c, d, e]) {
            *slot = slot.wrapping_add(value);
        }
    }
    h.iter().map(|v| format!("{v:08x}")).collect()
}

#[test]
fn sha1_is_sha1() {
    assert_eq!(sha1(b""), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    assert_eq!(sha1(b"abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
}

/// The fixtures hold the bytes Scrivener wrote, line ends included: every
/// `docs.checksum` line the scrub found right is right, and the one it found stale, Scene
/// A's text in `gt-writing` (G5), stays stale. A checkout that rewrote line ends, or a
/// scrub that recomputed every line, would read differently here.
#[test]
fn every_checksum_is_as_scrivener_left_it() {
    let mut stale = Vec::new();
    let mut checked = 0;
    for (project, data) in [
        ("gt-writing.scriv", "Files/Data"),
        ("gt-metadata.scriv", "Files/Data"),
        ("gt-footnote-marker.scriv", "Files/Data"),
        ("gt-writing-v2.scriv", "Files/Docs"),
    ] {
        let dir = fixture(project).join(data);
        let list = match std::fs::read_to_string(dir.join("docs.checksum")) {
            Ok(list) => list,
            Err(e) => panic!("{project}: {e}"),
        };
        for line in list.lines().filter(|l| !l.trim().is_empty()) {
            let Some((path, digest)) = line.rsplit_once('=') else {
                panic!("{project}: {line:?}");
            };
            // Scrivener for Mac lists a folder in lower case and names it in upper case.
            let file = match path.split_once('/') {
                Some((folder, name)) => dir.join(folder.to_ascii_uppercase()).join(name),
                None => dir.join(path),
            };
            let bytes = match std::fs::read(&file) {
                Ok(bytes) => bytes,
                Err(e) => panic!("{project}: {path}: {e}"),
            };
            checked += 1;
            if sha1(&bytes) != digest {
                stale.push(format!("{project}/{path}"));
            }
        }
    }
    assert_eq!(checked, 11);
    assert_eq!(
        stale,
        ["gt-writing.scriv/1b4f5cef-f577-45af-ad2f-fc774dfd9c99/content.rtf"]
    );
}

// ── C1: an inline annotation with an inline footnote typed right after it ────────────

/// Scene A's last paragraph holds an annotation and a footnote typed one after the other,
/// the footnote's opener written before the annotation's closer. Every copy of it comes
/// across as the two notes, the comment on "sentence." and the footnote cited right after
/// it, beside the scene's other notes and nothing more, with no marker left anywhere.
#[test]
fn an_annotation_and_the_footnote_typed_after_it_are_two_notes_in_every_copy() {
    for name in [
        "gt-writing.scriv",
        "gt-writing-bak1.zip",
        "gt-writing 2026-10-05 15-40.zip",
        "gt-writing-v2.scriv",
    ] {
        let (_dir, summary, bundle) = import(name);
        assert!(
            !prose_has(&summary, |n| matches!(
                n,
                ProseNotice::InlineNoteUnmatched { .. }
            )),
            "{name}: {:?}",
            summary.prose
        );
        let scene = item(&bundle, "Scene A");
        let djot = prose(scene, ContentRole::SceneText);
        let text = addressable(&djot);
        assert!(!text.contains("Scrv"), "{name}: {text:?}");
        let content = scene
            .item
            .prose_refs
            .iter()
            .find(|p| p.role == ContentRole::SceneText)
            .map(|p| p.file_id)
            .unwrap_or_default();
        // The scene's linked footnote, then the inline one: two, each once.
        let notes: Vec<(String, String)> = scene
            .footnotes
            .get(&content)
            .into_iter()
            .flatten()
            .map(|n| (n.label.clone(), addressable(&n.body)))
            .collect();
        let bodies: Vec<&str> = notes.iter().map(|(_, body)| body.as_str()).collect();
        assert_eq!(bodies, ["linked footnote", "inline footnote"], "{name}");
        // The inline footnote is cited right after the annotated word, at the end of the
        // last paragraph, and nowhere else.
        let label = &notes[1].0;
        let cite = format!("[^{label}]");
        assert_eq!(djot.matches(cite.as_str()).count(), 1, "{name}: {djot}");
        let last = djot.trim_end().lines().last().unwrap_or_default();
        assert!(
            last.ends_with(&format!("sentence.{cite}")),
            "{name}: {last:?}"
        );
        // The scene's two comments, then the annotation, on "sentence.": three, each once.
        let comments: Vec<(String, String)> = scene
            .comments
            .get(&content)
            .into_iter()
            .flatten()
            .map(|c| (c.quote_exact.clone(), addressable(&c.body)))
            .collect();
        let inline = ("sentence.".to_string(), "inline note".to_string());
        assert_eq!(comments.len(), 3, "{name}: {comments:?}");
        assert_eq!(
            comments.iter().filter(|c| **c == inline).count(),
            1,
            "{name}: {comments:?}"
        );
        for (_, body) in &comments {
            assert!(!body.contains("Scrv"), "{name}: {body:?}");
        }
        for (_, note) in &notes {
            assert!(!note.contains("Scrv"), "{name}: {note:?}");
        }
    }
}

// ── C2: a checkbox ticked by default ─────────────────────────────────────────────────

/// `Checked` is ticked by default: One writes no value and wears the tag; Two writes `No`
/// and does not.
#[test]
fn a_checkbox_ticked_by_default_ticks_one_and_not_two() {
    let (_dir, summary, bundle) = import("gt-metadata.scriv");
    assert!(tags_of(&bundle, "One").contains(&"Checked".to_string()));
    assert!(!tags_of(&bundle, "Two").contains(&"Checked".to_string()));
    assert!(!metadata_has(&summary, |n| matches!(
        n,
        MetadataNotice::CheckboxDefaultUnknown { .. }
    )));
}

// ── C3: the writing days ─────────────────────────────────────────────────────────────

/// Monday, Wednesday and Friday, written `2, 4, 6`, are the Pace's writing days.
#[test]
fn the_pace_keeps_monday_wednesday_and_friday() {
    let (_dir, summary, bundle) = import("gt-metadata.scriv");
    assert_eq!(bundle.paces.len(), 1);
    assert_eq!(bundle.paces[0].weekday_mask, 1 + 4 + 16);
    assert!(!metadata_has(&summary, |n| matches!(
        n,
        MetadataNotice::WritingDaysNotRead { .. }
    )));
}

// ── C4 and C17: Scrivener 3 for Mac's backup names ───────────────────────────────────

/// `-bak.zip`, `-bak1.zip` and Back Up To's `<name> YYYY-MM-DD HH-MM.zip` are each said to
/// be a backup, and each is titled after its project. Scrivener's own zips wrap a clean
/// `gt-writing.scriv/` whose compile settings state the title, so the title here says
/// nothing of the name stripping; the test after this one reaches it.
#[test]
fn every_backup_scrivener_named_is_said_to_be_one() {
    for (name, sign) in [
        ("gt-writing-bak.zip", BackupSign::BakArchive),
        ("gt-writing-bak1.zip", BackupSign::BakArchive),
        ("gt-writing 2026-10-05 15-40.zip", BackupSign::DateStamped),
    ] {
        let read = read_project(run(&fixture(name)));
        assert!(
            read.notices.contains(&Notice::SourceIsBackup { sign }),
            "{name}: {:?}",
            read.notices
        );
        assert_eq!(read.proposals.title.title, "gt-writing", "{name}");
    }
}

/// Copy a fixture folder to `to`, leaving out the members named.
fn copy_without(from: &Path, to: &Path, left_out: &[&str]) {
    let mut pending: Vec<PathBuf> = vec![PathBuf::new()];
    while let Some(relative) = pending.pop() {
        if let Err(e) = std::fs::create_dir_all(to.join(&relative)) {
            panic!("creating {}: {e}", to.join(&relative).display());
        }
        let entries = match std::fs::read_dir(from.join(&relative)) {
            Ok(entries) => entries,
            Err(e) => panic!("reading {}: {e}", from.join(&relative).display()),
        };
        for entry in entries.flatten() {
            let member = relative.join(entry.file_name());
            if left_out.iter().any(|l| Path::new(l) == member) {
                continue;
            }
            if entry.path().is_dir() {
                pending.push(member);
            } else if let Err(e) = std::fs::copy(entry.path(), to.join(&member)) {
                panic!("copying {}: {e}", member.display());
            }
        }
    }
}

/// With no title stated, a copy of the project kept under a backup's name, `-bak1` or
/// Back Up To's date and time, is titled after the project, those decorations dropped.
#[test]
fn a_folder_kept_under_a_backups_name_is_titled_after_its_project() {
    let dir = temp();
    for name in ["gt-writing-bak1.scriv", "gt-writing 2026-10-05 15-40.scriv"] {
        let copy = dir.path().join(name);
        copy_without(
            &fixture("gt-writing.scriv"),
            &copy,
            &["Settings/compile.xml"],
        );
        let read = read_project(run(&copy));
        assert_eq!(read.proposals.title.title, "gt-writing", "{name}");
        assert_eq!(
            read.proposals.title.source,
            TitleSource::FolderName,
            "{name}"
        );
    }
}

// ── C5: a session target Scrivener calculated ────────────────────────────────────────

#[test]
fn the_session_target_scrivener_calculated_is_said_to_be_calculated() {
    let (_dir, summary, bundle) = import("gt-metadata.scriv");
    assert!(
        summary
            .metadata
            .contains(&MetadataNotice::SessionTargetNotImported {
                value: 318,
                unit: TargetUnit::Words,
                calculated: true,
            }),
        "{:?}",
        summary.metadata
    );
    let project = note_text(&bundle, "W-note-project");
    assert!(
        project.contains("W-session-target-calculated\n318"),
        "{project}"
    );
}

// ── C6, C10, C20: Scene B's styles ───────────────────────────────────────────────────

/// Scene B's first line is a Heading 1, bold by its style: a heading, not bold.
#[test]
fn a_heading_comes_across_as_a_heading_not_bold() {
    let (_dir, _summary, bundle) = import("gt-writing.scriv");
    let djot = prose(item(&bundle, "Scene B"), ContentRole::SceneText);
    assert!(djot.starts_with("# First line\n"), "{djot}");
}

/// The block quote's indent is its kind's: six indents are lost, not seven.
#[test]
fn the_block_quotes_indent_is_not_counted_as_lost() {
    let (_dir, summary, _bundle) = import("gt-writing.scriv");
    assert!(
        prose_has(&summary, |n| matches!(
            n,
            ProseNotice::LeftIndentsNotKept { count: 6, .. }
        )),
        "{:?}",
        summary.prose
    );
}

/// The export to Scrivener's older format keeps the heading's `<$ScrKeepWithNext>` and
/// drops its style: said there, and never on the project it was exported from.
#[test]
fn the_heading_the_export_lost_is_said_and_only_there() {
    let lost = |summary: &Summary| {
        summary.prose.iter().find_map(|n| match n {
            ProseNotice::StylesLostInExport { count, .. } => Some(*count),
            _ => None,
        })
    };
    let (_dir, exported, _bundle) = import("gt-writing-v2.scriv");
    assert_eq!(lost(&exported), Some(1), "{:?}", exported.prose);
    let (_dir, live, _bundle) = import("gt-writing.scriv");
    assert_eq!(lost(&live), None, "{:?}", live.prose);
}

// ── C7: a date field ─────────────────────────────────────────────────────────────────

/// `When` is a short date: listed as the days One and Two hold, not as raw stamps.
#[test]
fn a_date_field_is_listed_as_its_days() {
    let (_dir, _summary, bundle) = import("gt-metadata.scriv");
    let custom = note_text(&bundle, "W-note-custom-metadata");
    assert!(custom.contains("When"), "{custom}");
    assert!(custom.contains("2026-10-05"), "{custom}");
    assert!(custom.contains("2026-10-23"), "{custom}");
    assert!(!custom.contains("00000"), "{custom}");
    assert!(
        !custom.contains("15:02"),
        "a short date shows no time: {custom}"
    );
}

// ── C8: Scrivener's own comment colours ──────────────────────────────────────────────

#[test]
fn scriveners_default_comment_colours_are_nothing_lost() {
    for name in ["gt-writing.scriv", "gt-footnote-marker.scriv"] {
        let (_dir, summary, _bundle) = import(name);
        assert!(
            !prose_has(&summary, |n| matches!(
                n,
                ProseNotice::CommentColoursLost { .. }
            )),
            "{name}: {:?}",
            summary.prose
        );
    }
}

// ── C9, C12, C13, C18: the template ──────────────────────────────────────────────────

/// A template keeps its goals, plans no Pace from the deadline saved in it and says so,
/// and lists that deadline and the writing days, Monday, Wednesday and Friday, which no
/// Pace keeps.
#[test]
fn a_template_keeps_its_goals_and_plans_no_pace() {
    let (_dir, summary, bundle) = import("gt-template.scrivtemplate");
    assert!(bundle.paces.is_empty());
    assert!(!metadata_has(&summary, |n| matches!(
        n,
        MetadataNotice::PacePlanned { .. }
    )));
    assert!(
        metadata_has(&summary, |n| matches!(
            n,
            MetadataNotice::DeadlineNotPlanned {
                reason: DeadlineReason::Template,
                day: Some(_)
            }
        )),
        "{:?}",
        summary.metadata
    );
    assert_eq!(item(&bundle, "One").item.word_count_goal, 1200);
    let project = note_text(&bundle, "W-note-project");
    assert!(
        project.contains("2027-10-05"),
        "the deadline listed: {project}"
    );
    assert!(
        project.contains("W-weekday-monday W+ W-weekday-wednesday W+ W-weekday-friday"),
        "the writing days listed: {project}"
    );
}

/// A template proposes its own title, the one its card shows.
#[test]
fn a_template_proposes_its_own_title() {
    let read = read_project(run(&fixture("gt-template.scrivtemplate")));
    assert!(matches!(
        read.project.source.container,
        Container::Template(_)
    ));
    assert_eq!(read.proposals.title.title, "gt-template");
    assert_eq!(read.proposals.title.source, TitleSource::Template);
}

/// A project that is all its archive holds was not found below the pick.
#[test]
fn scriveners_own_archives_say_nothing_of_a_project_below_the_pick() {
    for name in [
        "gt-writing-bak.zip",
        "gt-writing-bak1.zip",
        "gt-writing 2026-10-05 15-40.zip",
        "gt-template.scrivtemplate",
    ] {
        let read = read_project(run(&fixture(name)));
        assert!(
            !read
                .notices
                .iter()
                .any(|n| matches!(n, Notice::ProjectFoundBelowPick { .. })),
            "{name}: {:?}",
            read.notices
        );
    }
}

// ── C11: two statuses of one name ────────────────────────────────────────────────────

#[test]
fn two_statuses_of_one_name_become_two_rungs() {
    let (_dir, summary, bundle) = import("gt-metadata.scriv");
    let names: Vec<&str> = bundle.statuses.iter().map(|s| s.name.as_str()).collect();
    assert_eq!(names, ["First Draft", "Revised Draft", "Revised Draft (2)"]);
    assert!(
        summary
            .metadata
            .contains(&MetadataNotice::RungNamesDisambiguated {
                names: vec!["Revised Draft (2)".to_string()]
            })
    );
}

// ── C14: `templateinfo.xml` ──────────────────────────────────────────────────────────

/// The project Save As Template ran from records the template it was saved as.
#[test]
fn templateinfo_names_the_template_a_project_was_saved_as() {
    let read = read_project(run(&fixture("gt-metadata.scriv")));
    assert_eq!(
        read.project.source.saved_as_template.as_deref(),
        Some("gt-template")
    );
    let read = read_project(run(&fixture("gt-writing.scriv")));
    assert_eq!(read.project.source.saved_as_template, None);
}

// ── C15: the title Scrivener 3 states ────────────────────────────────────────────────

/// The export to the older format keeps the compile settings, which state the name the
/// project had when it was created: `gt-writing`, not the export's own name.
#[test]
fn the_stated_title_is_the_name_the_project_was_created_with() {
    let read = read_project(run(&fixture("gt-writing-v2.scriv")));
    assert_eq!(read.proposals.title.title, "gt-writing");
    assert_eq!(read.proposals.title.source, TitleSource::CompileSettings);
    assert!(read.notices.contains(&Notice::TitleFromProject {
        source: TitleSource::CompileSettings
    }));
}

// ── C16: an untitled item ────────────────────────────────────────────────────────────

/// The Blank template's starting document has no title: stored blank and said, once.
#[test]
fn the_blank_templates_untitled_document_is_said_once() {
    for name in ["gt-writing.scriv", "gt-footnote-marker.scriv"] {
        let (_dir, summary, bundle) = import(name);
        let untitled = summary
            .structure
            .iter()
            .filter(|n| matches!(n, StructureNotice::UntitledItem { .. }))
            .count();
        assert_eq!(untitled, 1, "{name}: {:?}", summary.structure);
        assert!(
            items(&bundle).any(|i| i.item.title.is_empty()),
            "{name}: stored blank"
        );
    }
}

// ── C19: the folder around them all ──────────────────────────────────────────────────

/// The folder of the handover holds four projects, three backups and a template: the
/// writer is asked which, and offered all eight.
#[test]
fn the_folder_around_the_ground_truth_offers_every_project_backup_and_template() {
    let asked = question(run(&fixtures()));
    let Question::SeveralProjects { candidates } = asked else {
        panic!("expected several projects, got {asked:?}");
    };
    let paths: Vec<&str> = candidates.iter().map(|c| c.path.as_str()).collect();
    assert_eq!(
        paths,
        [
            "gt-footnote-marker.scriv",
            "gt-metadata.scriv",
            "gt-writing-v2.scriv",
            "gt-writing.scriv",
            "gt-template.scrivtemplate",
            "gt-writing 2026-10-05 15-40.zip",
            "gt-writing-bak.zip",
            "gt-writing-bak1.zip",
        ]
    );
}
