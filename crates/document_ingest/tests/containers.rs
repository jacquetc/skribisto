// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `.docx` and `.odt`, read from files real producers actually wrote.
//!
//! The unit tests beside each scanner pin the *rules* against containers built in
//! the test body. These pin *reality*, and the difference has already earned its
//! keep: LibreOffice writes a point-anchored comment as a bare `w:commentReference`
//! with no range at all, pandoc renders a thematic break as an empty paragraph with
//! nothing but a bottom border, and Word splits one italic phrase across three runs
//! with identical properties. None of those would have appeared in XML we wrote
//! ourselves, and all three are things this importer had to be taught.
//!
//! `tests/fixtures/generate.py` regenerates every file here and records where each
//! one came from.

use document_ingest::{
    AnnotationKind, ImportDiagnostic, ScannerRegistry, SourceBlock, SourceDocument,
};
use std::path::{Path, PathBuf};

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn scan(name: &str) -> SourceDocument {
    let path = fixture(name);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|e| panic!("{name} is missing — run tests/fixtures/generate.py ({e})"));
    ScannerRegistry::with_builtin_scanners().scan_bytes(&path, &bytes)
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

fn prose(doc: &SourceDocument) -> String {
    doc.blocks
        .iter()
        .filter_map(|b| match b {
            SourceBlock::Prose { djot, .. } => Some(djot.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// The quoted text an annotation actually points at, taken from the block it names
/// rather than from what the annotation says about itself.
fn quoted(doc: &SourceDocument, index: usize) -> String {
    let annotation = &doc.annotations[index];
    let text = doc.blocks[annotation.block_index].plain_text();
    let chars: Vec<char> = text.chars().collect();
    let start = annotation.anchor.start.min(chars.len());
    let end = (start + annotation.anchor.length).min(chars.len());
    chars[start..end].iter().collect()
}

// ── structure ───────────────────────────────────────────────────────────────

/// Heading depth from `w:outlineLvl` and `text:outline-level`, never from a style
/// *name* — which is localized, and is the failure mode that would only show up on
/// somebody's French manuscript.
#[test]
fn both_formats_read_their_heading_levels_from_the_document_not_the_style_name() {
    for name in [
        "word-shaped.docx",
        "libreoffice.odt",
        "pandoc.odt",
        "pandoc.docx",
    ] {
        let doc = scan(name);
        assert_eq!(
            headings(&doc),
            vec![(1, "The Salt Road"), (2, "Chapter One"), (2, "Chapter Two")],
            "wrong heading ladder in {name}"
        );
    }
}

/// The whole point of the shared half: one source, two containers, the same prose.
#[test]
fn the_same_manuscript_reads_the_same_out_of_either_container() {
    let odt = prose(&scan("pandoc.odt"));
    let docx = prose(&scan("pandoc.docx"));
    for expected in ["_gone_", "*nothing*", "{-rain-}", "- one", "- two"] {
        assert!(
            odt.contains(expected),
            "missing {expected:?} in the ODT: {odt}"
        );
        assert!(
            docx.contains(expected),
            "missing {expected:?} in the DOCX: {docx}"
        );
    }
}

/// Word splits one italic phrase across three identically-formatted runs. The
/// conversion must not show the seams — `*g**o**ne*` is what getting this wrong
/// looks like, and it is invisible until someone reads their own imported book.
#[test]
fn a_phrase_split_across_identical_runs_comes_out_as_one_span() {
    let djot = prose(&scan("word-shaped.docx"));
    assert!(
        djot.contains("_the street was gone_"),
        "the three runs did not merge: {djot}"
    );
}

/// Pandoc writes `* * *` into ODT as an empty paragraph whose style declares
/// nothing but a bottom border. Reading it is what keeps ODT able to carry a break
/// its writer can see — and it is not the alignment heuristic `block.rs` refuses,
/// because a rule is the format's own spelling of a thematic break.
#[test]
fn a_horizontal_rule_is_read_as_the_scene_break_it_is() {
    let doc = scan("pandoc.odt");
    assert!(
        doc.blocks
            .iter()
            .any(|b| matches!(b, SourceBlock::SceneBreak { .. })),
        "the rule was lost: {:?}",
        doc.blocks
    );
}

// ── comments ────────────────────────────────────────────────────────────────

/// The one that matters most: the quote has to land on the words it was about. A
/// count alone would pass while every comment sat on the wrong sentence.
#[test]
fn a_comment_lands_on_the_words_it_was_about() {
    for name in ["word-shaped.docx", "libreoffice.odt"] {
        let doc = scan(name);
        let ranged = doc
            .annotations
            .iter()
            .position(|a| a.kind == AnnotationKind::Range)
            .unwrap_or_else(|| panic!("no ranged comment in {name}: {:?}", doc.annotations));

        assert_eq!(
            quoted(&doc, ranged),
            "the street was gone",
            "the comment in {name} points at the wrong text"
        );
        assert_eq!(doc.annotations[ranged].body, "Is this the right word?");
        assert_eq!(doc.annotations[ranged].author, "Editor");
        assert!(
            doc.annotations[ranged].anchor.exact == "the street was gone",
            "the stored quote must be the text too, or it cannot be re-found"
        );
    }
}

/// Word threads through `w15:commentsEx`; LibreOffice through `loext:parent-name`.
/// Both are one level deep, and so is Skribisto's card.
#[test]
fn a_reply_to_a_comment_stays_a_reply() {
    for name in ["word-shaped.docx", "libreoffice.odt"] {
        let doc = scan(name);
        let threaded = doc
            .annotations
            .iter()
            .find(|a| !a.replies.is_empty())
            .unwrap_or_else(|| panic!("no thread in {name}: {:?}", doc.annotations));

        assert_eq!(threaded.body, "Is this the right word?");
        assert_eq!(threaded.replies.len(), 1);
        assert_eq!(threaded.replies[0].author, "Writer");
        assert_eq!(threaded.replies[0].body, "Yes, I meant it.");
        assert_eq!(
            threaded.turns(),
            2,
            "one comment plus one reply is two turns"
        );
        assert!(
            !doc.annotations.iter().any(|a| a.body == "Yes, I meant it."),
            "a reply must not also be a comment of its own in {name}"
        );
    }
}

#[test]
fn a_resolved_comment_arrives_resolved() {
    for name in ["word-shaped.docx", "libreoffice.odt"] {
        let doc = scan(name);
        let resolved: Vec<&str> = doc
            .annotations
            .iter()
            .filter(|a| a.resolved)
            .map(|a| a.body.as_str())
            .collect();
        assert_eq!(
            resolved,
            vec!["A whole-paragraph note."],
            "wrong resolved set in {name}"
        );
    }
}

/// A comment with no range belongs to the paragraph its marker sat in, and covers
/// all of it. LibreOffice writes those as a bare `w:commentReference` that the
/// typed OOXML reader does not surface at all — which is why the DOCX scanner does
/// a second, targeted pass over `word/document.xml`.
#[test]
fn a_comment_on_a_whole_paragraph_covers_that_paragraph() {
    for name in ["word-shaped.docx", "libreoffice.odt"] {
        let doc = scan(name);
        let index = doc
            .annotations
            .iter()
            .position(|a| a.body == "A whole-paragraph note.")
            .unwrap_or_else(|| panic!("the paragraph comment vanished in {name}"));

        assert_eq!(
            doc.annotations[index].kind,
            AnnotationKind::Paragraph,
            "in {name}"
        );
        assert_eq!(
            quoted(&doc, index),
            "The second chapter opens quietly.",
            "the paragraph comment in {name} does not cover its paragraph"
        );
    }
}

/// The author's own date, not the moment of import — the one fact about a comment
/// that only the source file knows.
#[test]
fn a_comment_keeps_the_date_its_author_wrote_it() {
    for name in ["word-shaped.docx", "libreoffice.odt"] {
        let doc = scan(name);
        let dated = doc
            .annotations
            .iter()
            .find(|a| a.body == "Is this the right word?")
            .unwrap_or_else(|| panic!("no comment in {name}"));
        let created = dated
            .created
            .unwrap_or_else(|| panic!("no date on the comment in {name}"));
        assert_eq!(
            created.to_rfc3339(),
            "2026-01-02T03:04:05+00:00",
            "in {name}"
        );
    }
}

/// M-S4: a comment's own text is Djot now, not plain text — an editor who bolds
/// or italicises a word in their remark, or writes it as two paragraphs, must
/// see that survive the import rather than being flattened to plain prose.
/// Both scanners carry a comment authored exactly that way.
#[test]
fn a_comment_with_bold_and_italic_imports_with_formatting_intact() {
    for name in ["word-shaped.docx", "libreoffice.odt"] {
        let doc = scan(name);
        let rich = doc
            .annotations
            .iter()
            .find(|a| a.body.contains("real") && a.body.contains("italics"))
            .unwrap_or_else(|| {
                panic!(
                    "no richly formatted comment found in {name}: {:?}",
                    doc.annotations
                )
            });
        assert!(
            rich.body.contains("*real*"),
            "bold did not survive as Djot in {name}: {:?}",
            rich.body
        );
        assert!(
            rich.body.contains("_italics_"),
            "italic did not survive as Djot in {name}: {:?}",
            rich.body
        );
        assert!(
            rich.body.contains("A second paragraph in the same note."),
            "the second paragraph did not survive in {name}: {:?}",
            rich.body
        );
        assert_ne!(
            rich.body,
            "This needs real emphasis, and italics too.\n\nA second paragraph in the same note.",
            "the body must carry Djot markers, not the plain text a flattening \
             scanner would have produced, in {name}"
        );
    }
}

// ── what a container carries that Markdown does not ─────────────────────────

/// Accepted insertions, dropped deletions — and one sentence saying so, because a
/// writer who finds text they remember rejecting should have been warned.
#[test]
fn tracked_changes_are_accepted_and_reported() {
    let doc = scan("word-shaped.docx");
    let djot = prose(&doc);
    assert!(
        djot.contains("Later, she would say she had known."),
        "the insertion was not accepted: {djot}"
    );
    assert!(
        !djot.contains("always claimed"),
        "the deletion was not dropped: {djot}"
    );
    assert!(
        doc.diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::TrackedChangesFlattened { .. })),
        "the revision went unmentioned: {:?}",
        doc.diagnostics
    );
}

/// CommonMark has no comment syntax, so its scanner has nothing to report. That the
/// side channel stays *empty* rather than inventing something is worth pinning: a
/// format genuinely carrying data another does not is fine; a format inventing it
/// is the cross-format inconsistency the block model forbids.
#[test]
fn a_markdown_import_produces_no_annotations() {
    let registry = ScannerRegistry::with_builtin_scanners();
    let doc = registry.scan_bytes(
        Path::new("chapter.md"),
        b"# Chapter One\n\nShe turned the corner.\n",
    );
    assert!(doc.annotations.is_empty());
}

/// Both extensions have to reach the file filter and the drop zone, and both are
/// derived from the registry — so this is also the test that the feature gating
/// actually reaches the UI.
#[test]
fn the_registry_offers_both_container_formats() {
    let registry = ScannerRegistry::with_builtin_scanners();
    let exts = registry.accepted_extensions();
    for expected in ["docx", "odt", "fodt", "md", "txt"] {
        assert!(exts.contains(&expected), "missing '{expected}' in {exts:?}");
    }
}
