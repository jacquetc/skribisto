// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! End-to-end: real document shapes in, a whole plan out.
//!
//! The unit tests pin each stage; these pin the thing a writer actually does.
//! Four shapes, chosen because each one broke a real importer somewhere:
//!
//! - **Skribisto's own Markdown export**, which spells its major scene break
//!   `# # #` — an ATX heading to any CommonMark parser.
//! - **A folder of one-scene-per-file**, the note-app shape, where the
//!   ordering lives in the file names and there are no headings at all.
//! - **One long manuscript** with `#`/`##`/`###` structure, the export-from-Word
//!   shape.
//! - **A hostile file** carrying every trap at once.

use std::path::Path;

use document_ingest::block::SourceBlock;
use document_ingest::{ImportDiagnostic, ScannerRegistry, build_plan, infer_rules, levels_used};
use skribisto_model::scene_break::{self, SceneBreakTier};
use skribisto_model::{ChapterMode, CreateType};

fn scan_all(files: &[(&str, &str)]) -> Vec<document_ingest::SourceDocument> {
    let registry = ScannerRegistry::with_builtin_scanners();
    files
        .iter()
        .map(|(name, body)| registry.scan_bytes(Path::new(name), body.as_bytes()))
        .collect()
}

/// Goes through the one entry point the use case will, so these tests exercise
/// the same ordering the app gets rather than trusting the literal order the
/// fixture happens to list files in.
fn plan_for(files: &[(&str, &str)], start: CreateType) -> document_ingest::ImportPlan {
    let registry = ScannerRegistry::with_builtin_scanners();
    let owned: Vec<(std::path::PathBuf, Vec<u8>)> = files
        .iter()
        .map(|(name, body)| (std::path::PathBuf::from(name), body.as_bytes().to_vec()))
        .collect();
    document_ingest::scan_and_plan(&registry, &owned, start, ChapterMode::Folder, 0)
}

/// What Skribisto itself writes with a Glyph preset. The major break is `# # #`,
/// which is a level-1 ATX heading — get this wrong and re-importing your own
/// export invents a near-empty Book, silently, with no review step to catch it
/// since breaks are preserved rather than proposed.
#[test]
fn the_apps_own_export_reimports_without_inventing_rows() {
    let source = "\
# The Visitor

## Chapter One

She turned the corner.

* * *

The fog had not lifted.

# # #

## Chapter Two

Morning came late.
";
    let plan = plan_for(&[("The Visitor.md", source)], CreateType::Book);

    let shape: Vec<(i64, &str, CreateType)> = plan
        .rows
        .iter()
        .map(|r| (r.indent, r.title.as_str(), r.create_type))
        .collect();
    assert_eq!(
        shape,
        vec![
            (0, "The Visitor", CreateType::Book),
            // Chapters, not Parts: two heading levels walk the ladder to
            // Book/Part, but a Part holds no prose, so the deepest level
            // re-anchors onto Chapter. A Book of chapters is also what this
            // document plainly is.
            (1, "Chapter One", CreateType::Chapter),
            (1, "Chapter Two", CreateType::Chapter),
        ],
        "no row may be invented from a break glyph"
    );

    let chapter_one = &plan.rows[1];
    assert_eq!(chapter_one.scene_breaks, 2, "both breaks preserved inline");
    assert!(
        chapter_one
            .djot
            .contains(scene_break::canonical_djot(SceneBreakTier::Minor))
    );
    assert!(
        chapter_one
            .djot
            .contains(scene_break::canonical_djot(SceneBreakTier::Major))
    );
}

/// The note-app shape: ordering in the file names, no headings at all. Other
/// importers are documented to take this in whatever order the filesystem
/// happened to hand it over; the numeric prefix is ordering and must not survive
/// into the title, where it would fight Skribisto's own chapter numbering.
#[test]
fn a_folder_of_one_scene_per_file_becomes_one_row_each() {
    // Deliberately handed over out of order, the way a filesystem does — this is
    // the case that landed a writer's scenes as 4, 1, 2, 5, 3 elsewhere.
    let mut files: Vec<(String, String)> = (1..=12)
        .map(|i| {
            (
                format!("{i:02}_scene-{i}.md"),
                format!("Prose for scene {i}."),
            )
        })
        .collect();
    files.reverse();
    files.swap(0, 7);
    let borrowed: Vec<(&str, &str)> = files
        .iter()
        .map(|(n, b)| (n.as_str(), b.as_str()))
        .collect();

    let plan = plan_for(&borrowed, CreateType::Scene);

    assert_eq!(plan.rows.len(), 12);
    assert!(plan.rows.iter().all(|r| r.indent == 0));
    assert!(
        plan.rows.iter().all(|r| r.create_type == CreateType::Scene),
        "no headings means every file is one row of the starting kind"
    );
    assert_eq!(plan.rows[0].title, "scene 1");
    assert!(
        !plan.rows[0].title.starts_with("01"),
        "a numeric filename prefix is ordering, not part of the title"
    );
    // Twelve before two, not "12" before "2" — the sort is on the parsed number.
    assert_eq!(plan.rows[11].title, "scene 12");
}

/// The export-from-Word shape: one long file, three heading levels.
#[test]
fn one_long_manuscript_splits_along_its_heading_levels() {
    let mut source = String::from("# The Long Novel\n\n");
    for part in 1..=2 {
        source.push_str(&format!("## Part {part}\n\n"));
        for chapter in 1..=3 {
            source.push_str(&format!("### Chapter {chapter}: A Title\n\n"));
            source.push_str("Prose.\n\n* * *\n\nMore prose.\n\n");
        }
    }
    let plan = plan_for(&[("novel.md", &source)], CreateType::Book);

    assert_eq!(plan.rows.len(), 1 + 2 + 6);
    assert_eq!(plan.rows[0].create_type, CreateType::Book);
    assert_eq!(plan.rows[1].create_type, CreateType::Part);
    assert_eq!(plan.rows[2].create_type, CreateType::Chapter);
    assert_eq!(plan.rows[2].indent, 2);
    // The ordinal came off every chapter title, and was kept.
    assert_eq!(plan.rows[2].title, "A Title");
    assert_eq!(plan.rows[2].stripped_ordinal.as_deref(), Some("chapter 1"));
    assert_eq!(plan.rows[2].scene_breaks, 1);
}

/// A manuscript that spells its chapter numbers out, which is the other half of what
/// arrives in the wild — and the end-to-end proof that the word table reaches the plan,
/// not just `extract_leading_ordinal`'s own tests.
///
/// The last chapter deliberately opens with a number word that is *part of its title*.
/// If the reader ever loosened to "a leading number word is an ordinal", that row would
/// silently become a chapter called "Last Thing", and this is where it would show.
#[test]
fn a_manuscript_that_spells_its_numbers_out_still_lands_the_right_titles() {
    let source = "\
# The Long Novel

## Chapter One: The Storm

Prose.

## Chapter Twenty One: The Reckoning

More prose.

## Chapitre premier

Nothing but an ordinal.

## One Last Thing

Closing prose.
";
    let plan = plan_for(&[("novel.md", source)], CreateType::Book);

    assert_eq!(plan.rows.len(), 5, "one book and four chapters");

    assert_eq!(plan.rows[1].title, "The Storm");
    assert_eq!(
        plan.rows[1].stripped_ordinal.as_deref(),
        Some("chapter 1"),
        "the label is the canonical form of what came off, not the words themselves"
    );

    assert_eq!(plan.rows[2].title, "The Reckoning");
    assert_eq!(
        plan.rows[2].stripped_ordinal.as_deref(),
        Some("chapter 21"),
        "the whole multi-word ordinal comes off, not just its first word"
    );

    // A heading that is *only* an ordinal keeps its own text — `build_plan` prefers a
    // row titled "Chapitre premier" to an untitled one, and the writer can clear it in
    // review. Pinned here because it is the one case where reading the number changes
    // nothing on screen.
    assert_eq!(plan.rows[3].title, "Chapitre premier");
    assert_eq!(plan.rows[3].stripped_ordinal, None);

    assert_eq!(
        plan.rows[4].title, "One Last Thing",
        "a number word that opens a real title must survive untouched"
    );
    assert_eq!(plan.rows[4].stripped_ordinal, None);
}

/// Every trap at once, in one file. None of them may be fatal, and each must be
/// reported rather than silently swallowed.
#[test]
fn a_hostile_file_imports_and_explains_itself() {
    let source = "\
---
title: Hostile
order: 9
tags:
  - one
---

Preamble prose before any heading.

# Chapter 1: Real Heading

```
# not a heading
```

Text with a footnote.[^1]

![a photo](images/x.png)

    # not a heading either

***

After the break.

[^1]: The note.
";
    let plan = plan_for(&[("hostile.md", source)], CreateType::Chapter);

    // The fenced and indented `#` lines did not become headings.
    let headings: Vec<&str> = plan.rows.iter().map(|r| r.title.as_str()).collect();
    assert_eq!(headings, vec!["Hostile", "Real Heading"]);

    // Front matter never reached the prose.
    assert!(
        !plan.rows.iter().any(|r| r.djot.contains("title:")),
        "front matter leaked into prose"
    );

    // The preamble kept its own row rather than inheriting the heading's name.
    assert!(plan.rows[0].djot.contains("Preamble prose"));

    let keys: Vec<&str> = plan.diagnostics.iter().map(|d| d.key()).collect();
    for expected in [
        "front-matter-not-flat",
        "footnotes-degraded",
        "image-not-ingested",
    ] {
        assert!(keys.contains(&expected), "missing {expected} in {keys:?}");
    }
    assert_eq!(plan.rows[1].scene_breaks, 1);
}

/// A batch must never be abandoned over one bad member — surveyed folder imports
/// crash on a single undecodable byte, and one chapterizer exits outright below
/// three headings.
#[test]
fn one_broken_file_does_not_take_the_batch_down() {
    let registry = ScannerRegistry::with_builtin_scanners();
    let mut docs = Vec::new();
    for i in 0..5 {
        docs.push(registry.scan_bytes(
            Path::new(&format!("{i:02}_ok.md")),
            format!("# Chapter {i}\n\nProse.").as_bytes(),
        ));
    }
    // One file that is not valid UTF-8 and has no BOM to explain itself.
    docs.push(registry.scan_bytes(
        Path::new("06_broken.md"),
        b"# Chapter 6\n\nHe paused\x81then nothing.",
    ));
    // And one nobody can read at all.
    docs.push(registry.scan_bytes(Path::new("cover.pdf"), b"%PDF-1.7"));

    let rules = infer_rules(&levels_used(&docs), CreateType::Chapter);
    let plan = build_plan(&docs, &rules, ChapterMode::Folder, 0);

    assert_eq!(
        plan.rows.len(),
        6,
        "the five good files and the salvaged one"
    );
    let keys: Vec<&str> = plan.diagnostics.iter().map(|d| d.key()).collect();
    assert!(keys.contains(&"lossy-decode"));
    assert!(keys.contains(&"unsupported-format"));
}

/// Breaks are preserved, never split — the decision this whole pipeline is built
/// around. A chapter with four breaks is one row, and says so.
#[test]
fn a_chapter_of_five_scenes_is_still_one_row() {
    let mut source = String::from("# Chapter One\n\n");
    for i in 0..5 {
        source.push_str(&format!("Scene {i} prose.\n\n"));
        if i < 4 {
            source.push_str("* * *\n\n");
        }
    }
    let plan = plan_for(&[("c.md", &source)], CreateType::Chapter);

    assert_eq!(plan.rows.len(), 1);
    assert_eq!(plan.rows[0].scene_breaks, 4);
    assert_eq!(
        plan.rows[0].word_count, 15,
        "markers are furniture and are not counted"
    );
    assert!(matches!(
        scan_all(&[("c.md", &source)])[0].blocks.first(),
        Some(SourceBlock::Heading { level: 1, .. })
    ));
    assert!(
        !plan
            .diagnostics
            .iter()
            .any(|d| matches!(d, ImportDiagnostic::IllegalCombination { .. }))
    );
}
