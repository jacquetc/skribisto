// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! M-T1/M-S7 round trip: the DOCX analogue of `odt_writer_roundtrip.rs` — see that file's own
//! module doc for why this shape (generate fresh via `text-document`'s public API, then read it
//! back through this crate's own scanner, on every run) catches a regression on either side
//! immediately, rather than waiting for someone to notice a stale checked-in fixture disagrees.
//!
//! `text-document`'s DOCX writer (`document_io::use_cases::export_docx_uc`) is what
//! `sources::docx::Walker`/`CommentTable`/`RawScan` were measured against, so a comment thread
//! this writer emits and this reader reads back is the same pair of eyes checking its own
//! agreement — exactly as `odt_writer_roundtrip.rs` already does for ODT.
//!
//! ## What this file proves that `odt_writer_roundtrip.rs` cannot
//!
//! DOCX can carry two things ODF genuinely cannot (`export_odt_uc`'s own module doc calls this
//! a documented format ceiling, not a bug): `w:initials` and — for both formats, but only DOCX
//! is exercised for it *here* because ODT's own equivalent already has its own coverage in
//! `odt_writer_roundtrip.rs` — a `skrb:uid` this crate's `RawScan` reads back via a raw pass over
//! `word/comments.xml` (`docx-rs` 0.4.22's typed `Comment` surfaces neither field at all; see
//! `sources::docx`'s own module doc). Both are asserted here, round-tripped through a *real*
//! uuid string — the shape `apply_document_import_uc` actually receives, not the opaque test
//! labels (`"cmt-root-1"`) `text-document`'s own writer-only tests use, which would not parse as
//! a uid at all.

use document_ingest::{AnnotationKind, ScannerRegistry, SourceBlock, SourceDocument};
use std::path::Path;
use text_document::{
    CommentReply, DocumentComment, DocumentComments, DocxExportOptions, FindOptions,
};

/// Djot chosen to exercise headings, inline formatting, a hyperlink, a nested list, and a
/// blockquote — the DOCX-side twin of `odt_writer_roundtrip.rs`'s `SOURCE_DJOT`, deliberately the
/// same text so the two files can be compared side by side.
const SOURCE_DJOT: &str = "\
# Chapter One

The city held its breath before the *storm*, and Aurélien knew it.

- A crate of oranges
- A [locked door](https://example.com/door)

  - Its key, missing since spring

> Every winter asks the same question twice.
";

fn write_docx_with_options(djot: &str, options: DocxExportOptions) -> Vec<u8> {
    let doc = text_document::TextDocument::new();
    doc.set_djot_sync(djot).expect("set_djot_sync");

    let path = std::env::temp_dir().join(format!(
        "docx_writer_roundtrip_{}_{}.docx",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default()
    ));

    doc.to_docx_with_options(&path.to_string_lossy(), options)
        .expect("to_docx_with_options")
        .wait()
        .expect("docx export completes");

    let bytes = std::fs::read(&path).expect("read exported docx");
    let _ = std::fs::remove_file(&path);
    bytes
}

fn write_docx(djot: &str) -> Vec<u8> {
    write_docx_with_options(djot, DocxExportOptions::default())
}

fn scan(bytes: &[u8]) -> SourceDocument {
    // A real path with a `.docx` extension — extension is how the registry picks a scanner, so
    // this proves the exported file is recognised as DOCX at all, not merely readable once you
    // already know what it is.
    let path = Path::new("roundtrip.docx");
    ScannerRegistry::with_builtin_scanners().scan_bytes(path, bytes)
}

fn headings(doc: &SourceDocument) -> Vec<(u8, &str)> {
    doc.blocks
        .iter()
        .filter_map(|b| match b {
            SourceBlock::Heading { level, text } => Some((*level, text.as_str())),
            _ => None,
        })
        .collect()
}

fn prose_text(doc: &SourceDocument) -> String {
    doc.blocks
        .iter()
        .filter_map(|b| match b {
            SourceBlock::Prose { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\x1e")
}

#[test]
fn the_docx_writer_produces_a_file_this_crates_docx_reader_recognises_and_reads_back() {
    let bytes = write_docx(SOURCE_DJOT);
    assert!(!bytes.is_empty());
    assert!(bytes.starts_with(b"PK"), "a .docx is a zip container");

    let doc = scan(&bytes);
    assert!(
        !doc.is_empty(),
        "the round-tripped document must not come back empty; diagnostics: {:?}",
        doc.diagnostics
    );

    assert_eq!(
        headings(&doc),
        vec![(1, "Chapter One")],
        "the heading and its level must survive"
    );

    let prose = prose_text(&doc);
    assert!(
        prose.contains("The city held its breath before the"),
        "prose text missing: {prose:?}"
    );
    assert!(
        prose.contains("storm"),
        "the italicised word must survive as text: {prose:?}"
    );
    assert!(
        prose.contains("Aurélien"),
        "a non-ASCII character must survive byte-for-byte: {prose:?}"
    );
    assert!(
        prose.contains("A crate of oranges"),
        "the first list item's text is missing: {prose:?}"
    );
    assert!(
        prose.contains("locked door"),
        "the hyperlink's visible text is missing: {prose:?}"
    );
    assert!(
        prose.contains("Its key, missing since spring"),
        "the nested list item's text is missing: {prose:?}"
    );
    assert!(
        prose.contains("Every winter asks the same question twice."),
        "the blockquote's prose is missing: {prose:?}"
    );
}

// ---------------------------------------------------------------------------
// M-S7: comment round trip, including `skrb:uid` and `w:initials`
// ---------------------------------------------------------------------------

const COMMENT_DJOT: &str = "\
This manuscript opens with a sentence that needs review.

A second, unrelated paragraph follows.
";

/// The `[start, end)` addressable-character range of `needle`'s first occurrence in `doc` —
/// the same offset space `common::parser_tools::DocumentComment::start`/`end` and, on the
/// reader side, `skribisto_model::comment_anchor::Anchor::start` are defined in.
fn find_range(doc: &text_document::TextDocument, needle: &str) -> (u32, u32) {
    let m = doc
        .find(needle, 0, &FindOptions::default())
        .expect("find")
        .unwrap_or_else(|| panic!("{needle:?} not found in the document"));
    (m.position as u32, (m.position + m.length) as u32)
}

fn export_docx_bytes(doc: &text_document::TextDocument, options: DocxExportOptions) -> Vec<u8> {
    let path = std::env::temp_dir().join(format!(
        "docx_writer_roundtrip_comments_{}_{}.docx",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default()
    ));

    doc.to_docx_with_options(&path.to_string_lossy(), options)
        .expect("to_docx_with_options")
        .wait()
        .expect("docx export completes");

    let bytes = std::fs::read(&path).expect("read exported docx");
    let _ = std::fs::remove_file(&path);
    bytes
}

/// A resolved comment thread — real uuid strings for both the root and the reply, real
/// initials on both (DOCX, unlike ODT, has a carrier for them: `w:initials`) — round-trips
/// through this crate's own `DocxScanner`: author, initials, uid, resolved flag, the anchored
/// quote, the reply's own author/initials/uid/body, and the emphasis inside both bodies.
#[test]
fn a_resolved_comment_thread_with_uid_and_initials_round_trips() {
    let doc = text_document::TextDocument::new();
    doc.set_djot_sync(COMMENT_DJOT).expect("set_djot_sync");

    let root_uid = uuid::Uuid::new_v4();
    let reply_uid = uuid::Uuid::new_v4();

    let range = find_range(&doc, "needs review");
    let mut root = DocumentComment {
        start: range.0,
        end: range.1,
        uid: root_uid.to_string(),
        author: "Alice Editor".to_string(),
        author_initials: "AE".to_string(),
        date: "2026-01-01T00:00:00Z".to_string(),
        resolved: true,
        body: "Please *tighten* this phrase.".to_string(),
        replies: Vec::new(),
    };
    root.replies.push(CommentReply {
        uid: reply_uid.to_string(),
        author: "Bob Writer".to_string(),
        author_initials: "BW".to_string(),
        date: "2026-01-02T00:00:00Z".to_string(),
        body: "Good catch, will **fix** it in the next pass.".to_string(),
    });
    let mut comments = DocumentComments::new();
    comments.insert(root);

    let bytes = export_docx_bytes(
        &doc,
        DocxExportOptions {
            comments,
            ..Default::default()
        },
    );
    let source = scan(&bytes);

    assert_eq!(
        source.annotations.len(),
        1,
        "exactly one comment thread must come back; diagnostics: {:?}",
        source.diagnostics
    );
    let annotation = &source.annotations[0];

    assert_eq!(
        annotation.kind,
        AnnotationKind::Range,
        "the anchor must resolve to a real range, not degrade to a whole-document comment \
         (diagnostics: {:?})",
        source.diagnostics
    );
    assert_eq!(annotation.anchor.exact, "needs review");
    assert!(!annotation.anchor.exact_truncated);

    assert_eq!(annotation.author, "Alice Editor");
    assert_eq!(
        annotation.uid,
        Some(root_uid),
        "the skrb:uid attribute must survive the raw pass over word/comments.xml"
    );
    assert_eq!(
        annotation.author_initials, "AE",
        "w:initials must survive — unlike ODT, DOCX has a real carrier for it"
    );
    assert!(annotation.resolved, "the resolved flag must survive");
    assert!(
        annotation.body.contains("tighten"),
        "root body text missing: {:?}",
        annotation.body
    );
    assert!(
        annotation.body.contains('*'),
        "the root body's emphasis must survive as real Djot markup: {:?}",
        annotation.body
    );

    assert_eq!(
        annotation.replies.len(),
        1,
        "the reply must survive as one thread member"
    );
    let reply = &annotation.replies[0];
    assert_eq!(reply.author, "Bob Writer");
    assert_eq!(
        reply.uid,
        Some(reply_uid),
        "a reply's own uid must survive too"
    );
    assert_eq!(reply.author_initials, "BW");
    assert!(
        reply.body.contains("Good catch") && reply.body.contains("fix"),
        "reply body text missing: {:?}",
        reply.body
    );
    assert!(
        reply.body.contains('*'),
        "the reply body's bold emphasis must survive as real Djot markup: {:?}",
        reply.body
    );
}

/// A comment with no uid at all (`DocumentComment::uid` empty, exactly what an editor's own
/// remark — never round-tripped through Skribisto — would look like if it somehow carried the
/// `skrb:` namespace at all, which in practice it never does) must come back as `uid: None`,
/// never as `Some` of some fabricated value and never as a parse error that aborts the scan.
#[test]
fn a_comment_with_no_uid_reads_back_as_none() {
    let doc = text_document::TextDocument::new();
    doc.set_djot_sync(COMMENT_DJOT).expect("set_djot_sync");

    let range = find_range(&doc, "second, unrelated paragraph");
    let mut comments = DocumentComments::new();
    comments.insert(DocumentComment {
        start: range.0,
        end: range.1,
        uid: String::new(),
        author: "Editor".to_string(),
        author_initials: String::new(),
        date: "2026-01-01T00:00:00Z".to_string(),
        resolved: false,
        body: "Still needs a look.".to_string(),
        replies: Vec::new(),
    });

    let bytes = export_docx_bytes(
        &doc,
        DocxExportOptions {
            comments,
            ..Default::default()
        },
    );
    let source = scan(&bytes);

    assert_eq!(
        source.annotations.len(),
        1,
        "diagnostics: {:?}",
        source.diagnostics
    );
    let annotation = &source.annotations[0];
    assert_eq!(
        annotation.uid, None,
        "an empty skrb:uid must not parse as a real uid"
    );
    assert_eq!(annotation.author_initials, "");
    assert!(!annotation.resolved);
}

/// Two comments anchored to two different paragraphs, each with its own uid, both survive
/// independently — proves the raw `word/comments.xml` pass keys strictly by `w:id` and does not
/// mix up two threads' attributes when more than one is present.
#[test]
fn two_comments_with_different_uids_both_survive_independently() {
    let doc = text_document::TextDocument::new();
    doc.set_djot_sync(COMMENT_DJOT).expect("set_djot_sync");

    let uid1 = uuid::Uuid::new_v4();
    let uid2 = uuid::Uuid::new_v4();

    let range1 = find_range(&doc, "needs review");
    let range2 = find_range(&doc, "unrelated paragraph");
    let mut comments = DocumentComments::new();
    comments.insert(DocumentComment {
        start: range1.0,
        end: range1.1,
        uid: uid1.to_string(),
        author: "First Author".to_string(),
        author_initials: "FA".to_string(),
        date: "2026-01-01T00:00:00Z".to_string(),
        resolved: false,
        body: "First remark.".to_string(),
        replies: Vec::new(),
    });
    comments.insert(DocumentComment {
        start: range2.0,
        end: range2.1,
        uid: uid2.to_string(),
        author: "Second Author".to_string(),
        author_initials: "SA".to_string(),
        date: "2026-01-01T00:00:00Z".to_string(),
        resolved: false,
        body: "Second remark.".to_string(),
        replies: Vec::new(),
    });

    let bytes = export_docx_bytes(
        &doc,
        DocxExportOptions {
            comments,
            ..Default::default()
        },
    );
    let source = scan(&bytes);

    assert_eq!(
        source.annotations.len(),
        2,
        "diagnostics: {:?}",
        source.diagnostics
    );
    let mut by_author: Vec<(&str, Option<uuid::Uuid>, &str)> = source
        .annotations
        .iter()
        .map(|a| (a.author.as_str(), a.uid, a.author_initials.as_str()))
        .collect();
    by_author.sort();
    let mut expected = vec![
        ("First Author", Some(uid1), "FA"),
        ("Second Author", Some(uid2), "SA"),
    ];
    expected.sort();
    assert_eq!(by_author, expected);
}
