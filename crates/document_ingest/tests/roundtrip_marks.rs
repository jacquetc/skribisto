// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What an editor's *save* does to the two ways a file can carry Skribisto's identity.
//!
//! Exporting comments was only half the round trip. The other half — reading a returning file
//! back onto the book it came from — needs every row and every comment to still be recognisable
//! after the file has been through Word or LibreOffice. The first comment-export milestone chose
//! a private-namespace attribute (`skrb:uid`) for that, and never tested what an editor's own
//! application does to it.
//!
//! It deletes it. Verified against a real returning file — a manuscript exported from this app,
//! commented and replied to in LibreOffice 25.8.5.2, and saved: not one `skrb:uid` survived, and
//! the `skrb` namespace declaration was gone with them. `office:name`, the ODF-native handle the
//! writer pairs an annotation with its end by, had been rewritten to LibreOffice's own
//! `__Annotation__337_4185181322`. Recognition-on-re-import, in other words, worked only for a
//! file nobody had opened.
//!
//! A **bookmark** does survive, because it is not an extension: `text:bookmark` and
//! `w:bookmarkStart` are first-class in ODF and OOXML, position-tracked through edits, and
//! preserved by every writer that claims to support either format. That is why the marks this
//! importer looks for are bookmarks and not attributes.
//!
//! These tests pin both halves of that against fixtures produced by real LibreOffice
//! (`tests/fixtures/generate.py`, `ROUNDTRIP_FODT`), whose source carries *both* carriers on the
//! same annotation so the contrast is one file, not two. They are deliberately structural — they
//! read the container's XML rather than going through the scanner — because the claim under test
//! is about the file, not about our reading of it. The scanner's own tests live beside it.

use std::io::Read;
use std::path::{Path, PathBuf};

/// The row mark for Chapter One: `skrb_r` + a 64-bit `BinderItem.uid` prefix + a 48-bit digest
/// of the row's normalised text at export time.
const ROW_ONE: &str = "skrb_r0000000000000001_aaaaaaaaaaaa";
const ROW_TWO: &str = "skrb_r0000000000000002_bbbbbbbbbbbb";
/// The comment mark: `skrb_c` + a 64-bit `Comment.uid` prefix, as a *pair* bracketing the
/// commented range.
const COMMENT_ONE: &str = "skrb_c000000000000c001";

/// Word caps a bookmark name at 40 characters and accepts only letters, digits and underscore.
/// Both name shapes are built to fit, and a change that overruns it would otherwise be found by
/// Word silently dropping the mark — which is to say, not found.
const WORD_BOOKMARK_NAME_LIMIT: usize = 40;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// One part of a zip container, as text.
fn part(container: &str, part: &str) -> String {
    let path = fixture(container);
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        panic!("{container} is missing — run tests/fixtures/generate.py ({e})")
    });
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes))
        .unwrap_or_else(|e| panic!("{container} is not a zip: {e}"));
    let mut file = zip
        .by_name(part)
        .unwrap_or_else(|e| panic!("{container} has no {part}: {e}"));
    let mut out = String::new();
    file.read_to_string(&mut out)
        .unwrap_or_else(|e| panic!("{container}/{part} is not UTF-8: {e}"));
    out
}

/// Every XML part of a container, concatenated. `skrb:uid` must be absent from *all* of them,
/// not merely from the one the marks live in: on the OOXML path a comment's attributes would be
/// in `word/comments.xml`, nowhere near `word/document.xml`.
fn all_xml(container: &str) -> String {
    let path = fixture(container);
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        panic!("{container} is missing — run tests/fixtures/generate.py ({e})")
    });
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes))
        .unwrap_or_else(|e| panic!("{container} is not a zip: {e}"));
    let names: Vec<String> = zip.file_names().map(str::to_string).collect();
    let mut out = String::new();
    for name in names {
        if !name.ends_with(".xml") {
            continue;
        }
        let mut file = zip.by_name(&name).expect("named part exists");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect("part is readable");
        out.push_str(&String::from_utf8_lossy(&buf));
    }
    out
}

#[test]
fn odt_keeps_every_bookmark_mark_through_a_libreoffice_save() {
    let content = part("roundtrip.odt", "content.xml");

    for name in [ROW_ONE, ROW_TWO] {
        assert!(
            content.contains(&format!(r#"<text:bookmark text:name="{name}"/>"#)),
            "the row mark {name} did not survive: {}",
            "LibreOffice dropped a zero-length text:bookmark"
        );
    }

    // A comment mark is a *pair*, and ODF names both ends. Losing either end would leave a
    // range with no close, which is indistinguishable from no mark at all.
    assert!(
        content.contains(&format!(
            r#"<text:bookmark-start text:name="{COMMENT_ONE}"/>"#
        )),
        "the comment mark's start did not survive"
    );
    assert!(
        content.contains(&format!(
            r#"<text:bookmark-end text:name="{COMMENT_ONE}"/>"#
        )),
        "the comment mark's end did not survive"
    );
}

#[test]
fn odt_drops_the_private_uid_attribute_through_a_libreoffice_save() {
    let xml = all_xml("roundtrip.odt");
    assert!(
        !xml.contains("skrb:uid"),
        "skrb:uid survived a LibreOffice save — if this ever becomes true, the attribute is a \
         usable identity carrier again and this crate's reliance on bookmarks could be revisited"
    );
    // The namespace declaration goes with it, which is why the attribute cannot simply be
    // re-read under a different prefix.
    assert!(
        !xml.contains("urn:ferntech:text-document:comment:1"),
        "the skrb namespace declaration survived a LibreOffice save"
    );
}

#[test]
fn docx_keeps_every_bookmark_mark_through_a_libreoffice_save() {
    let document = part("roundtrip.docx", "word/document.xml");

    for name in [ROW_ONE, ROW_TWO, COMMENT_ONE] {
        assert!(
            document.contains(&format!(r#"w:name="{name}""#)),
            "the mark {name} did not survive the OOXML writer"
        );
    }
}

#[test]
fn a_docx_bookmark_closes_by_id_not_by_name() {
    // OOXML names a bookmark only on its start; the end carries the numeric id alone. A reader
    // that looks for the name on both ends finds a range that never closes — so the comment
    // mark's extent has to be resolved through the id table, and this pins that it is the only
    // way to.
    let document = part("roundtrip.docx", "word/document.xml");
    let start = document
        .split_once(&format!(r#"w:name="{COMMENT_ONE}""#))
        .expect("the comment mark is present")
        .0;
    let id = start
        .rsplit_once("<w:bookmarkStart w:id=\"")
        .expect("the mark is a bookmarkStart")
        .1
        .trim_end_matches("\" ")
        .to_string();

    assert!(
        document.contains(&format!(r#"<w:bookmarkEnd w:id="{id}"/>"#)),
        "the comment mark's end is not addressable by its id {id}"
    );
    assert!(
        !document.contains(&format!(
            r#"<w:bookmarkEnd w:id="{id}" w:name="{COMMENT_ONE}""#
        )),
        "an end carrying the name would make the id table unnecessary — the reader may be \
         simplified if OOXML ever starts emitting one"
    );
}

#[test]
fn docx_drops_the_private_uid_attribute_through_a_libreoffice_save() {
    let xml = all_xml("roundtrip.docx");
    assert!(
        !xml.contains("skrb:uid"),
        "skrb:uid survived the OOXML path — it does not even reach word/comments.xml"
    );
}

#[test]
fn both_mark_names_fit_inside_words_bookmark_name_limit() {
    for name in [ROW_ONE, ROW_TWO, COMMENT_ONE] {
        assert!(
            name.len() <= WORD_BOOKMARK_NAME_LIMIT,
            "{name} is {} characters, over Word's {WORD_BOOKMARK_NAME_LIMIT}",
            name.len()
        );
        assert!(
            name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
            "{name} carries a character Word rejects in a bookmark name"
        );
        assert!(
            name.starts_with(|c: char| c.is_ascii_alphabetic()),
            "{name} must start with a letter"
        );
    }
}

#[test]
fn the_returning_file_carries_libreoffices_own_reply_citation() {
    // Not a curiosity: LibreOffice's Reply button writes a citation paragraph — "Répondre à
    // (date): "…"" — into the body of the reply itself, ahead of what the editor typed. Import
    // it verbatim and it accumulates in the writer's thread on every round trip. This pins that
    // the fixture really contains the shape the stripper has to recognise, so the stripper's own
    // tests are testing against reality rather than against a transcription of it.
    for container in ["roundtrip.odt", "roundtrip.docx"] {
        let xml = all_xml(container);
        assert!(
            xml.contains("Répondre à"),
            "{container} lost the reply citation the stripper exists for"
        );
        assert!(
            xml.contains("Yes, I meant it."),
            "{container} lost the reply's actual text"
        );
    }
}

// ── What the scanner makes of all that ──────────────────────────────────────────────────
//
// Everything above is about the *file*. These are about reading it: the same two fixtures, put
// through the real scanners, asserting that identity arrives where the importer needs it.

use document_ingest::block::{AnnotationKind, SourceDocument};
use document_ingest::scanner::ScannerRegistry;

fn scan(name: &str) -> SourceDocument {
    let path = fixture(name);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|e| panic!("{name} is missing — run tests/fixtures/generate.py ({e})"));
    ScannerRegistry::with_builtin_scanners().scan_bytes(&path, &bytes)
}

/// Both containers, one expectation: the marks come back, naming the same rows in the same order.
#[test]
fn every_row_mark_is_recovered_from_both_containers() {
    for container in ["roundtrip.odt", "roundtrip.docx"] {
        let doc = scan(container);
        let got: Vec<(&str, &str)> = doc
            .row_marks
            .iter()
            .map(|m| (m.uid_tag.as_str(), m.digest.as_str()))
            .collect();
        assert_eq!(
            got,
            vec![
                ("0000000000000001", "aaaaaaaaaaaa"),
                ("0000000000000002", "bbbbbbbbbbbb"),
            ],
            "{container} did not yield both row marks"
        );

        // And each names the block it was written into, not merely *a* block.
        for (mark, expected) in doc
            .row_marks
            .iter()
            .zip(["She turned the corner", "The second chapter opens quietly."])
        {
            let text = doc.blocks[mark.block_index].plain_text();
            assert!(
                text.starts_with(expected),
                "{container}: mark {} landed on {text:?}",
                mark.uid_tag
            );
        }
    }
}

/// The asymmetry that justifies the whole design, asserted through the reader.
///
/// The fixture's source carries *both* carriers on the same annotation. After a real
/// LibreOffice save, the scanner can recover only one of them — and it is the bookmark.
#[test]
fn a_comment_is_identified_by_its_mark_because_the_attribute_is_gone() {
    for container in ["roundtrip.odt", "roundtrip.docx"] {
        let doc = scan(container);
        assert_eq!(doc.annotations.len(), 1, "{container}");
        let a = &doc.annotations[0];
        assert_eq!(
            a.uid_tag.as_deref(),
            Some("000000000000c001"),
            "{container}: the bookmark's identity did not reach the annotation"
        );
        assert_eq!(
            a.uid, None,
            "{container}: skrb:uid cannot have survived — if it did, re-read \
             roundtrip_marks.rs's module doc before trusting it"
        );
        assert_eq!(a.kind, AnnotationKind::Range, "{container}");
    }
}

/// LibreOffice's own citation block never becomes part of the editor's words.
///
/// Left in, it accumulates: each export-reply-import cycle prepends another quotation, and a
/// reply that came back unchanged stops matching what the project stored, so a re-import that
/// should be a no-op reports it as edited.
#[test]
fn a_replys_body_is_what_the_editor_typed_and_nothing_else() {
    for container in ["roundtrip.odt", "roundtrip.docx"] {
        let doc = scan(container);
        let replies = &doc.annotations[0].replies;
        assert_eq!(replies.len(), 1, "{container}");
        assert_eq!(
            replies[0].body.trim(),
            "Yes, I meant it.",
            "{container}: the citation block reached the writer's thread"
        );
        assert!(
            !replies[0].body.contains("Répondre"),
            "{container}: {:?}",
            replies[0].body
        );
    }
}

/// …and the marks survive being turned into a plan, which is what the importer actually reads.
#[test]
fn a_planned_row_carries_the_identity_of_the_row_it_came_from() {
    for container in ["roundtrip.odt", "roundtrip.docx"] {
        let doc = scan(container);
        let levels: Vec<u8> = doc
            .blocks
            .iter()
            .filter_map(|b| match b {
                document_ingest::block::SourceBlock::Heading { level, .. } => Some(*level),
                _ => None,
            })
            .collect();
        let rules =
            document_ingest::structure::infer_rules(&levels, skribisto_model::CreateType::Book);
        let plan = document_ingest::plan::build_plan(
            std::slice::from_ref(&doc),
            &rules,
            skribisto_model::ChapterMode::Folder,
            0,
        );

        let identified: Vec<(&str, &str)> = plan
            .rows
            .iter()
            .filter_map(|r| Some((r.source_uid_tag.as_deref()?, r.source_digest.as_deref()?)))
            .collect();
        assert_eq!(
            identified,
            vec![
                ("0000000000000001", "aaaaaaaaaaaa"),
                ("0000000000000002", "bbbbbbbbbbbb"),
            ],
            "{container}: the plan lost the rows' identity"
        );

        // The comment reaches its row with its identity intact, which is what lets a returning
        // file update a thread instead of duplicating it.
        let commented: Vec<&document_ingest::plan::PlannedComment> =
            plan.rows.iter().flat_map(|r| r.comments.iter()).collect();
        assert_eq!(commented.len(), 1, "{container}");
        assert_eq!(commented[0].uid_tag.as_deref(), Some("000000000000c001"));
    }
}
