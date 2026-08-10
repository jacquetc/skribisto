// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! M-T2a round trip: `text-document` now has an ODT *writer*
//! (`TextDocument::to_odt_with_options`) to go with this crate's ODT *reader*
//! (`sources::odt::OdtScanner`), on two different sides of the same repo boundary — the writer
//! lives in `text-document` (a generic document-model crate that knows nothing about
//! `skribisto_model` or this crate's block shape), the reader lives here. Nothing forces the two
//! to agree on what a paragraph, a heading, or a nested list looks like in the wire format except
//! them actually agreeing — which is exactly what `tests/containers.rs` proves against
//! *real producers* (LibreOffice, Pandoc) and this file proves against *this workspace's own*
//! producer.
//!
//! Unlike `containers.rs` (which reads static fixtures written by real tools once and checked
//! in), this test **generates** the `.odt` fresh on every run, via `text-document`'s public API,
//! so a regression in either side breaks this test immediately rather than waiting for someone to
//! regenerate a fixture.
//!
//! Scope, matching M-T2a's own scope: footnotes/images are not exercised, because
//! `document_ingest`'s own `Walker::inline` deliberately does not carry a footnote's citation or
//! body into any `SourceBlock` at all (`(Some(NS_TEXT), "note") => { self.footnotes += 1; }` — a
//! diagnostic count, nothing else) — that is a known, pre-existing, and correctly documented
//! limitation of the *reader*, not something this writer could satisfy no matter what it emitted.
//! What the first half of this file proves is the part meant to round-trip: headings, prose (with
//! inline bold/italic/hyperlink), nested lists, and a blockquote's prose.
//!
//! ## M-T2b: comments now round-trip too
//!
//! `text-document`'s ODT writer gained comment-range support in M-T2b
//! (`document_io::use_cases::export_odt_uc`'s `office:annotation`/`office:annotation-end`
//! machinery), and this crate's own `sources::odt::Walker::open_annotation`/`close_annotation`
//! is what that encoding was measured against in the first place — so a comment this writer
//! emits and this reader reads back is the *same* pair of eyes checking its own agreement,
//! exactly as the prose half of this file already does. What the second half proves: a comment's
//! author, resolved flag, anchored quote, threaded reply, and rich (bold/italic) body all survive
//! the round trip.
//!
//! **M-S7 update:** `skrb:uid`, the writer's own private-namespace uid carrier (see
//! `export_odt_uc`'s module doc), now *does* have a reader-side consumer —
//! `SourceAnnotation::uid`/`SourceAnnotationReply::uid`, read in `odt.rs::open_annotation` — so
//! the uid tests below assert it round-trips through a *real* uuid string, the shape
//! `apply_document_import_uc` actually receives. `author_initials` is the one field that does
//! **not** round-trip here: ODF's `<office:annotation>` has no carrier for it at all (a
//! documented format ceiling — see `export_odt_uc`'s own module doc), so it always comes back
//! empty regardless of what was asked for on write. `docx_writer_roundtrip.rs` is where
//! `author_initials` gets its real round-trip proof, since DOCX's `w:initials` actually carries it.

use document_ingest::{AnnotationKind, ScannerRegistry, SourceBlock, SourceDocument};
use std::path::Path;
use text_document::{
    CommentReply, DocumentComment, DocumentComments, FindOptions, OdtExportOptions,
};

/// Djot chosen to exercise headings, inline formatting, a hyperlink, a nested list, and a
/// blockquote — everything `document_ingest::sources::odt`'s reader actually turns into
/// `SourceBlock`s. Scene breaks, footnotes and images are deliberately left out — see this
/// module's doc comment for why they would not prove anything about this pair either way.
const SOURCE_DJOT: &str = "\
# Chapter One

The city held its breath before the *storm*, and Aurélien knew it.

- A crate of oranges
- A [locked door](https://example.com/door)

  - Its key, missing since spring

> Every winter asks the same question twice.
";

/// Build a `text-document` document from `SOURCE_DJOT`, export it to a real `.odt` file, and
/// return that file's bytes.
fn write_odt() -> Vec<u8> {
    let doc = text_document::TextDocument::new();
    doc.set_djot_sync(SOURCE_DJOT).expect("set_djot_sync");

    let path = std::env::temp_dir().join(format!(
        "odt_writer_roundtrip_{}_{}.odt",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default()
    ));

    doc.to_odt_with_options(
        &path.to_string_lossy(),
        text_document::OdtExportOptions::default(),
    )
    .expect("to_odt_with_options")
    .wait()
    .expect("odt export completes");

    let bytes = std::fs::read(&path).expect("read exported odt");
    let _ = std::fs::remove_file(&path);
    bytes
}

fn scan(bytes: &[u8]) -> SourceDocument {
    // A real path with a `.odt` extension, matching what `containers.rs`'s own `scan` helper
    // feeds `ScannerRegistry::scan_bytes` — extension is how the registry picks a scanner, and
    // going through the full registry (not `OdtScanner` directly) is what proves the exported
    // file is recognised as ODT at all, not merely readable once you already know it is.
    let path = Path::new("roundtrip.odt");
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

/// Every prose block's plain text, concatenated with a separator — enough to prove the words
/// survived without depending on exactly how the reader chose to split paragraphs.
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
fn the_odt_writer_produces_a_file_this_crates_odt_reader_recognises_and_reads_back() {
    let bytes = write_odt();
    assert!(!bytes.is_empty());
    assert!(bytes.starts_with(b"PK"), "an .odt is a zip container");

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

/// The reader's own Djot conversion (`SourceBlock::Prose::djot`) must carry the hyperlink
/// through as a real Djot link, not merely as its visible text — proving the writer's
/// `<text:a>` and the reader's `(Some(NS_TEXT), "a")` handling agree on more than plain text.
#[test]
fn the_hyperlink_survives_as_a_real_link_not_just_its_visible_text() {
    let bytes = write_odt();
    let doc = scan(&bytes);
    let djot: String = doc
        .blocks
        .iter()
        .filter_map(|b| match b {
            SourceBlock::Prose { djot, .. } => Some(djot.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        djot.contains("(https://example.com/door)"),
        "the link target did not survive into the Djot conversion: {djot:?}"
    );
}

// ---------------------------------------------------------------------------
// M-T2b: comment round trip
// ---------------------------------------------------------------------------

/// A resolved root comment (rich, bold-and-italic body), a reply, and a second, unrelated
/// paragraph the comment must NOT anchor to — exercising the same shape
/// `docx_comment_export_tests.rs`'s golden-fixture test does for DOCX.
const COMMENT_DJOT: &str = "\
This manuscript opens with a sentence that needs review.

A second, unrelated paragraph follows.
";

/// Export `doc` to a real `.odt` file with `options` and return that file's bytes — the same
/// recipe [`write_odt`] follows, factored out so a comment test can call
/// [`text_document::TextDocument::find`] on `doc` (to compute an exact anchor range) *before*
/// deciding what to export, which `write_odt`'s all-in-one shape has no room for.
fn export_odt_bytes(doc: &text_document::TextDocument, options: OdtExportOptions) -> Vec<u8> {
    let path = std::env::temp_dir().join(format!(
        "odt_writer_roundtrip_comments_{}_{}.odt",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default()
    ));

    doc.to_odt_with_options(&path.to_string_lossy(), options)
        .expect("to_odt_with_options")
        .wait()
        .expect("odt export completes");

    let bytes = std::fs::read(&path).expect("read exported odt");
    let _ = std::fs::remove_file(&path);
    bytes
}

/// The `[start, end)` addressable-character range of `needle`'s first occurrence in `doc` —
/// `TextDocument::find`'s own offset space, which is exactly the space
/// `common::parser_tools::DocumentComment::start`/`end` (and, on the reader side,
/// `skribisto_model::comment_anchor::Anchor::start`) are defined in. See that module's own doc
/// comment for why this is not the same space `FormatRun` byte offsets live in.
fn find_range(doc: &text_document::TextDocument, needle: &str) -> (u32, u32) {
    let m = doc
        .find(needle, 0, &FindOptions::default())
        .expect("find")
        .unwrap_or_else(|| panic!("{needle:?} not found in the document"));
    (m.position as u32, (m.position + m.length) as u32)
}

/// A resolved comment thread with a reply and a rich (bold/italic) body, anchored to a real
/// range, round-trips through this crate's own `OdtScanner`: author, resolved flag, the
/// anchored quote, the reply's own author and body, and the emphasis inside both bodies all
/// survive. This is the M-T2b analogue of
/// `the_odt_writer_produces_a_file_this_crates_odt_reader_recognises_and_reads_back` above —
/// same writer, same reader, now proving the comment machinery both sides agree on rather than
/// the prose machinery.
#[test]
fn a_resolved_comment_thread_with_a_rich_body_and_a_reply_round_trips() {
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

    let bytes = export_odt_bytes(
        &doc,
        OdtExportOptions {
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

    // A real range, not a degraded whole-document comment — proves the writer's
    // `office:annotation`/`office:annotation-end` pair landed at the right characters for the
    // reader's own `close_annotation` to measure a non-zero length from.
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
        "the skrb:uid attribute must survive (M-S7)"
    );
    assert_eq!(
        annotation.author_initials, "",
        "ODF has no carrier for initials at all — a documented format ceiling, not a bug"
    );
    assert!(annotation.resolved, "the resolved flag must survive");
    assert!(
        annotation.body.contains("tighten"),
        "root body text missing: {:?}",
        annotation.body
    );
    assert!(
        annotation.body.contains('*'),
        "the root body's italic emphasis must survive as real Djot markup, not flatten to \
         plain text: {:?}",
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

/// An UNresolved thread must come back unresolved — the false case exercised as deliberately as
/// the true one in [`a_resolved_comment_thread_with_a_rich_body_and_a_reply_round_trips`], so a
/// writer bug that always wrote (or always omitted) `loext:resolved="true"` could not hide
/// behind a suite that only ever asks for a resolved thread.
#[test]
fn an_unresolved_comment_thread_stays_unresolved() {
    let doc = text_document::TextDocument::new();
    doc.set_djot_sync(COMMENT_DJOT).expect("set_djot_sync");

    let range = find_range(&doc, "second, unrelated paragraph");
    let mut comments = DocumentComments::new();
    comments.insert(DocumentComment {
        start: range.0,
        end: range.1,
        uid: "cmt-unresolved-1".to_string(),
        author: "Editor".to_string(),
        author_initials: String::new(),
        date: "2026-01-01T00:00:00Z".to_string(),
        resolved: false,
        body: "Still needs a look.".to_string(),
        replies: Vec::new(),
    });

    let bytes = export_odt_bytes(
        &doc,
        OdtExportOptions {
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
    assert!(
        !annotation.resolved,
        "an unresolved thread must not come back resolved"
    );
    assert_eq!(annotation.anchor.exact, "second, unrelated paragraph");
    assert!(annotation.body.contains("Still needs a look."));
}

/// Two comments anchored to two DIFFERENT paragraphs both survive, each keeping its own author
/// and quote — proves `office:name`'s per-thread uniqueness (a name collision would merge or
/// misattribute one thread to the other) and that a second `office:annotation` pair does not
/// disturb the first's own pairing.
#[test]
fn two_comments_on_two_different_paragraphs_both_survive_independently() {
    let doc = text_document::TextDocument::new();
    doc.set_djot_sync(COMMENT_DJOT).expect("set_djot_sync");

    let range1 = find_range(&doc, "needs review");
    let range2 = find_range(&doc, "unrelated paragraph");
    let mut comments = DocumentComments::new();
    comments.insert(DocumentComment {
        start: range1.0,
        end: range1.1,
        uid: "cmt-first".to_string(),
        author: "First Author".to_string(),
        author_initials: String::new(),
        date: "2026-01-01T00:00:00Z".to_string(),
        resolved: false,
        body: "First remark.".to_string(),
        replies: Vec::new(),
    });
    comments.insert(DocumentComment {
        start: range2.0,
        end: range2.1,
        uid: "cmt-second".to_string(),
        author: "Second Author".to_string(),
        author_initials: String::new(),
        date: "2026-01-01T00:00:00Z".to_string(),
        resolved: false,
        body: "Second remark.".to_string(),
        replies: Vec::new(),
    });

    let bytes = export_odt_bytes(
        &doc,
        OdtExportOptions {
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
    let mut by_author: Vec<(&str, &str)> = source
        .annotations
        .iter()
        .map(|a| (a.author.as_str(), a.anchor.exact.as_str()))
        .collect();
    by_author.sort();
    assert_eq!(
        by_author,
        vec![
            ("First Author", "needs review"),
            ("Second Author", "unrelated paragraph"),
        ]
    );
}

/// A comment with no uid at all must come back as `uid: None`, never as `Some` of a fabricated
/// value and never as a parse error that aborts the scan — the ODT twin of
/// `docx_writer_roundtrip.rs`'s identically named test.
#[test]
fn a_comment_with_no_uid_reads_back_as_none() {
    let doc = text_document::TextDocument::new();
    doc.set_djot_sync(COMMENT_DJOT).expect("set_djot_sync");

    let range = find_range(&doc, "needs review");
    let mut comments = DocumentComments::new();
    comments.insert(DocumentComment {
        start: range.0,
        end: range.1,
        uid: String::new(),
        author: "Editor".to_string(),
        author_initials: String::new(),
        date: "2026-01-01T00:00:00Z".to_string(),
        resolved: false,
        body: "No uid at all.".to_string(),
        replies: Vec::new(),
    });

    let bytes = export_odt_bytes(
        &doc,
        OdtExportOptions {
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
    assert_eq!(
        source.annotations[0].uid, None,
        "an empty skrb:uid must not parse as a real uid"
    );
}
