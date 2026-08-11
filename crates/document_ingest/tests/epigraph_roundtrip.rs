// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! An epigraph exported to `.docx`/`.odt` comes home as an epigraph.
//!
//! # What was broken
//!
//! `skribisto_compiler` has always marked a chapter's epigraph
//! (`render::push_epigraph` → `{semantic_role=epigraph}`), and `text-document`'s two
//! writers have always turned that into a real named paragraph style — `Epigraph`, with
//! `EpigraphAttribution` for its right-aligned source line. The readers in
//! `document_ingest` never looked. So a returning file's epigraph arrived as ordinary body
//! prose and was concatenated into whichever row's manuscript it happened to touch:
//!
//! * with `EpigraphPlacement::AfterHeading` (the default) it prepended itself to its own
//!   chapter's `SceneText` — leaving `EpigraphText` untouched, so the quotation was
//!   **duplicated**, counted as manuscript words against
//!   `skribisto_model`'s `an_epigraph_is_never_counted_as_prose`, and gained one further
//!   copy on every export/import cycle;
//! * with `BeforeHeading` it appended itself to the **previous** chapter's prose, landing
//!   in a different chapter altogether.
//!
//! Neither was visible in a diff of the file: the style was there and correct the whole
//! time. Only the reading was missing.
//!
//! # Why the tests are shaped this way
//!
//! Like `odt_writer_roundtrip.rs`, every file here is **generated fresh** through
//! `text-document`'s public writer rather than checked in, so a regression on either side
//! of the repo boundary breaks the test immediately instead of waiting for someone to
//! regenerate a fixture.
//!
//! [`survives_a_real_libreoffice_save`] then does the one thing generation cannot: it
//! pushes the file through a real `soffice --convert-to` and reads it again. That is the
//! claim the whole design rests on — a *named style* survives an editor's save, where the
//! `skrb:uid` attribute `round_trip`'s module doc records LibreOffice deleting does not —
//! and it is an empirical claim, so it is settled empirically. It skips rather than fails
//! where LibreOffice is not installed; the generated half still runs everywhere.
//!
//! # Where this sits in the chain
//!
//! The trip out is four links, and each is pinned where it lives rather than end to end
//! from here — an end-to-end test would need a real `.skrib` and a real export scope, and
//! `frontend/tests/round_trip_harness.rs` is deliberately the hand-run half for that.
//!
//! 1. `skribisto_compiler::render::mark_epigraph` writes `{semantic_role=epigraph}` onto
//!    the blockquote — pinned by that module's own tests.
//! 2. `text-document`'s parser lifts it onto the frame as `fmt_semantic_role`.
//! 3. Its DOCX/ODT writers turn that into the `Epigraph`/`EpigraphAttribution` named
//!    styles — pinned by `document_io`'s `*_export_tests`.
//! 4. **This file**: those styles are read back as an epigraph, and the epigraph reaches
//!    the row it heads.
//!
//! The `{semantic_role=epigraph}` in the constants below is therefore link 1's *output*,
//! written out by hand because the compiler is not a dependency here — but a change to
//! its spelling breaks link 1's own test first, so the transcription cannot rot silently.

use document_ingest::{ScannerRegistry, SourceBlock, SourceDocument, scan_and_plan};
use skribisto_model::{ChapterMode, CreateType};
use std::path::{Path, PathBuf};

/// `EpigraphPlacement::AfterHeading` — the default, and what Chicago, French, German and
/// Russian practice all describe: the quotation follows the chapter's title.
const AFTER_HEADING: &str = "\
# Chapter One

> {semantic_role=epigraph}
> Every winter asks the same question twice.

The city held its breath before the storm.
";

/// `EpigraphPlacement::BeforeHeading` — the `\\epigraphhead` shape, where the quotation
/// opens the page above the title. Chapter One is given prose of its own so that the
/// epigraph is unambiguously *not* touching its heading.
const BEFORE_HEADING: &str = "\
# Chapter One

Rain all week, and the river still low.

> {semantic_role=epigraph}
> Every winter asks the same question twice.

# Chapter Two

The city held its breath before the storm.
";

/// An ordinary quotation inside a scene — no `semantic_role`. Proves the other half of
/// the change: a blockquote now carries a `Quote` named style out and is read back as a
/// blockquote, where before it returned as body text with the indent's italics baked in.
const PLAIN_QUOTE: &str = "\
# Chapter One

She read the letter twice.

> I shall not be home before the thaw.

Then she folded it away.
";

fn write(djot: &str, ext: &str, dir: &Path) -> PathBuf {
    let doc = text_document::TextDocument::new();
    doc.set_djot_sync(djot).expect("set_djot_sync");
    let path = dir.join(format!("book.{ext}"));
    let p = path.to_string_lossy().to_string();
    if ext == "odt" {
        doc.to_odt_with_options(&p, text_document::OdtExportOptions::default())
            .expect("to_odt_with_options")
            .wait()
            .expect("odt write");
    } else {
        doc.to_docx_with_options(&p, text_document::DocxExportOptions::default())
            .expect("to_docx_with_options")
            .wait()
            .expect("docx write");
    }
    path
}

fn scan(bytes: &[u8], ext: &str) -> SourceDocument {
    ScannerRegistry::with_builtin_scanners().scan_bytes(Path::new(&format!("book.{ext}")), bytes)
}

/// A temporary directory of this test's own, removed on the way out.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("skrib_epi_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

/// Every epigraph block's Djot, in document order.
fn epigraphs(doc: &SourceDocument) -> Vec<String> {
    doc.blocks
        .iter()
        .filter_map(|b| match b {
            SourceBlock::Epigraph { djot, .. } => Some(djot.trim().to_string()),
            _ => None,
        })
        .collect()
}

/// Every prose block's Djot, in document order.
fn prose(doc: &SourceDocument) -> Vec<String> {
    doc.blocks
        .iter()
        .filter_map(|b| match b {
            SourceBlock::Prose { djot, .. } => Some(djot.trim().to_string()),
            _ => None,
        })
        .collect()
}

/// Round `djot` through both container formats and hand each scanned document to `check`.
///
/// Both formats, always: the two scanners resolve the style through entirely different
/// machinery (`w:basedOn` over `w:styleId` in OOXML, `style:parent-style-name` over
/// `style:name` in ODF), and a fix that lands in one of them is a fix in neither.
fn for_both_formats(name: &str, djot: &str, check: impl Fn(&str, SourceDocument)) {
    let dir = scratch(name);
    for ext in ["odt", "docx"] {
        let path = write(djot, ext, &dir);
        let bytes = std::fs::read(&path).expect("read back");
        check(ext, scan(&bytes, ext));
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_epigraph_after_its_heading_is_read_as_an_epigraph_not_as_prose() {
    for_both_formats("after", AFTER_HEADING, |ext, doc| {
        assert_eq!(
            epigraphs(&doc),
            vec!["> _Every winter asks the same question twice._"],
            "[{ext}] the quotation must come back as an epigraph block"
        );
        // The point of the whole change: it is *not* also in the prose. Before the fix
        // this vector held the epigraph and the chapter's own paragraph joined together.
        assert_eq!(
            prose(&doc),
            vec!["The city held its breath before the storm."],
            "[{ext}] the epigraph must not be concatenated into the chapter's manuscript"
        );
    });
}

#[test]
fn an_epigraph_before_its_heading_is_read_as_an_epigraph_not_as_prose() {
    for_both_formats("before", BEFORE_HEADING, |ext, doc| {
        assert_eq!(
            epigraphs(&doc),
            vec!["> _Every winter asks the same question twice._"],
            "[{ext}] the quotation must come back as an epigraph block"
        );
        assert_eq!(
            prose(&doc),
            vec![
                "Rain all week, and the river still low.",
                "The city held its breath before the storm.",
            ],
            "[{ext}] neither chapter's prose may absorb the epigraph between them"
        );
    });
}

/// The epigraph reaches the row it heads, both ways round.
///
/// The scanner half above proves the quotation is *recognised*; this proves it is
/// *attributed*. `BEFORE_HEADING` is the case a naive reading gets wrong — the epigraph
/// physically precedes Chapter Two's heading and follows Chapter One's prose, so anything
/// keyed on "the row currently collecting" would file it under Chapter One.
#[test]
fn the_plan_gives_each_epigraph_to_the_chapter_it_heads() {
    let dir = scratch("plan");
    for ext in ["odt", "docx"] {
        for (label, djot, owner) in [
            ("after", AFTER_HEADING, "Chapter One"),
            ("before", BEFORE_HEADING, "Chapter Two"),
        ] {
            let path = write(djot, ext, &dir);
            let bytes = std::fs::read(&path).expect("read back");
            let plan = scan_and_plan(
                &ScannerRegistry::with_builtin_scanners(),
                &[(path.clone(), bytes)],
                CreateType::Chapter,
                ChapterMode::Folder,
                0,
            );

            let carrying: Vec<&str> = plan
                .rows
                .iter()
                .filter(|r| !r.epigraph.trim().is_empty())
                .map(|r| r.title.as_str())
                .collect();
            assert_eq!(
                carrying,
                vec![owner],
                "[{ext}/{label}] exactly one row heads the quotation, and it is {owner}"
            );

            // And no row's manuscript quietly gained it — the duplication half of the bug.
            for row in &plan.rows {
                assert!(
                    !row.djot.contains("Every winter"),
                    "[{ext}/{label}] '{}' absorbed the epigraph into its prose",
                    row.title
                );
            }
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// An ordinary blockquote survives as a blockquote.
///
/// Before the `Quote` named style existed, neither scanner ever produced
/// `ParagraphKind::Quote` at all — it was constructed only in a unit test — so every
/// quotation in a manuscript came home as a plain paragraph. The `>` here is the proof
/// that it does not any more.
#[test]
fn an_ordinary_blockquote_survives_as_a_blockquote() {
    for_both_formats("quote", PLAIN_QUOTE, |ext, doc| {
        let all = prose(&doc).join("\n");
        assert!(
            all.contains("> I shall not be home before the thaw."),
            "[{ext}] the quotation must come back quoted, not as body text: {all:?}"
        );
        assert!(
            epigraphs(&doc).is_empty(),
            "[{ext}] a quotation with no semantic role is not an epigraph"
        );
    });
}

/// The claim the design rests on: a **named style** survives an editor's own save.
///
/// `roundtrip_marks.rs` records the measurement that forced the round-trip marks to be
/// bookmarks — LibreOffice 25.8 deletes a private-namespace `skrb:uid` attribute and its
/// namespace declaration with it. A paragraph style is not an extension: it is first-class
/// in both ODF and OOXML, it is visible in the application's own style panel, and it comes
/// back untouched. This test is what turns that from an argument into a measurement.
///
/// Skipped, not failed, where `soffice` is absent: this suite must stay runnable on a
/// machine with no office suite installed.
#[test]
fn survives_a_real_libreoffice_save() {
    if which_soffice().is_none() {
        eprintln!("skipping: no `soffice` on PATH");
        return;
    }
    let dir = scratch("lo");
    for (ext, filter) in [("odt", "odt"), ("docx", "docx:MS Word 2007 XML")] {
        let path = write(AFTER_HEADING, ext, &dir);
        let Some(returned) = through_libreoffice(&path, filter, ext, &dir) else {
            panic!("[{ext}] LibreOffice is on PATH but the conversion produced nothing");
        };
        let doc = scan(&returned, ext);
        assert_eq!(
            epigraphs(&doc),
            vec!["> _Every winter asks the same question twice._"],
            "[{ext}] the Epigraph style must survive a real save, or the round trip \
             only ever worked for a file nobody opened"
        );
        assert_eq!(
            prose(&doc),
            vec!["The city held its breath before the storm."],
            "[{ext}] and it must still not be part of the manuscript"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

fn which_soffice() -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|dir| dir.join("soffice"))
            .find(|candidate| candidate.is_file())
    })
}

/// Convert `path` in place through headless LibreOffice, returning what it wrote.
///
/// A **private user profile** per format, under this test's own scratch directory: a
/// headless `soffice` sharing the machine's real profile fights whatever the developer
/// has open, and two of these running concurrently under `cargo test` would fight each
/// other.
fn through_libreoffice(path: &Path, filter: &str, ext: &str, dir: &Path) -> Option<Vec<u8>> {
    let outdir = dir.join(format!("lo_{ext}"));
    let profile = dir.join(format!("loprofile_{ext}"));
    std::fs::create_dir_all(&outdir).ok()?;
    let status = std::process::Command::new("soffice")
        .arg(format!(
            "-env:UserInstallation=file://{}",
            profile.to_string_lossy()
        ))
        .args([
            "--headless",
            "--norestore",
            "--convert-to",
            filter,
            "--outdir",
        ])
        .arg(&outdir)
        .arg(path)
        .status()
        .ok()?;
    if !status.success() {
        return None;
    }
    let stem = path.file_stem()?.to_string_lossy().to_string();
    std::fs::read(outdir.join(format!("{stem}.{ext}"))).ok()
}
