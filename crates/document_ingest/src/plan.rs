// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The tree the importer would create, before it creates any of it.
//!
//! A flat, depth-encoded row list rather than a nested type — which is what the
//! binder is anyway (an ordered stream plus an indent), and what a Qleany DTO can
//! carry, and what a `TreeTableView` binds to.
//!
//! ## Scene breaks are preserved, never split
//!
//! A detected break becomes an escaped marker *inside* the prose of the row it
//! falls in. It never ends one row and starts another. That is the model's own
//! stated position — `skribisto_model::scene_break`: *"splitting prose into two
//! Scene items is a chunking decision, not a narrative signal"* — and it is what
//! the two importers that already exist do (`plume::map::emit_separator`, and the
//! legacy `.skrib` upgrader).
//!
//! The marker must be written in its **escaped** Djot form. A bare `* * *` in
//! Djot source is a thematic break, which the document model cannot represent and
//! the parser discards, so an unescaped marker would simply vanish on the next
//! load. `scene_break::canonical_djot` is the one authority on those bytes.

use common::entities::ContentRole;
use skribisto_model::scene_break;
use skribisto_model::{ChapterMode, CreateType, allowed_content};

use crate::block::{SourceBlock, SourceDocument};
use crate::diagnostics::ImportDiagnostic;
use crate::structure::LevelRules;
use crate::title;

/// One row the import would create.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedRow {
    /// Indent in the binder's flat stream — 0 is top level.
    pub indent: i64,
    pub create_type: CreateType,
    pub title: String,
    /// The ordinal lifted off the title, if any. Kept so the review step can show
    /// what was removed and put it back.
    pub stripped_ordinal: Option<String>,
    /// Prose for this row, already Djot, scene-break markers already spliced in.
    /// Empty for a row that is purely structural.
    pub djot: String,
    /// How many scene breaks the prose carries. Shown per row so the writer can
    /// see the granularity they are getting before committing to it — a chapter
    /// reading "4,100 words, 3 breaks" is one item, not four.
    pub scene_breaks: usize,
    /// Words in the prose, markers excluded.
    pub word_count: usize,
    /// Which source this came from.
    pub origin: String,
    /// Whether to create it. The review step unchecks rather than deletes.
    pub included: bool,
    pub diagnostics: Vec<ImportDiagnostic>,
}

/// A reviewable import.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportPlan {
    pub rows: Vec<PlannedRow>,
    /// Diagnostics that belong to the import as a whole rather than one row.
    pub diagnostics: Vec<ImportDiagnostic>,
}

impl ImportPlan {
    pub fn included_rows(&self) -> impl Iterator<Item = &PlannedRow> {
        self.rows.iter().filter(|r| r.included)
    }
}

/// Turn scanned documents into a reviewable plan.
///
/// `chapter_mode` is read once and applied to every chapter, because the model
/// offers no per-item override — a project is Folder-chaptered or Flat-chaptered,
/// not both.
pub fn build_plan(
    docs: &[SourceDocument],
    rules: &LevelRules,
    chapter_mode: ChapterMode,
    base_indent: i64,
) -> ImportPlan {
    let mut plan = ImportPlan::default();

    // The heading ladder spans the whole import, not each document.
    //
    // The level → type rules already do (`infer_rules` is fed every level any
    // document used), and the two must agree: a per-document ladder gave
    // `## Chapter Two` in a file of its own the type Chapter — from the global
    // rule — at indent 0, while `## Chapter Two` under a `#` in another file
    // landed at indent 1. Same heading, same type, two different depths, one of
    // them outside the book it belongs to. A book split across files shares one
    // heading convention; that is the whole reason the files are being imported
    // together.
    let mut open_levels: Vec<u8> = Vec::new();

    for doc in docs {
        plan.diagnostics.extend(doc.diagnostics.iter().cloned());
        if doc.is_empty() {
            continue;
        }
        append_document(&mut plan, doc, rules, base_indent, &mut open_levels);
    }

    // Word counts, once, on the assembled prose — and with markers stripped,
    // because a break is furniture the writer placed, not words they wrote.
    // Out here rather than per document: inside the loop it re-counted every
    // row already in the plan for each further document.
    for row in &mut plan.rows {
        row.word_count = scene_break::strip_markers_djot(&row.djot)
            .split_whitespace()
            .count();
    }

    flag_duplicate_titles(&mut plan);
    flag_illegal_combinations(&mut plan, &chapter_mode);
    plan
}

fn append_document(
    plan: &mut ImportPlan,
    doc: &SourceDocument,
    rules: &LevelRules,
    base_indent: i64,
    // Heading level → the indent its row sits at. Rebuilt as levels are met so a
    // document that skips a level (`#` then `####`) nests one step, not three:
    // the phantom-folder failure Scrivener is documented to produce. Owned by
    // [`build_plan`] and carried across documents — see the note there.
    open_levels: &mut Vec<u8>,
) {
    // The row currently collecting prose. A document may open with prose before
    // any heading, which becomes a row of its own rather than being silently
    // attached to the first heading that follows.
    let mut current: Option<PlannedRow> = None;

    let push = |plan: &mut ImportPlan, row: Option<PlannedRow>| {
        if let Some(row) = row
            && !(row.title.trim().is_empty() && row.djot.trim().is_empty())
        {
            plan.rows.push(row);
        }
    };

    for block in &doc.blocks {
        match block {
            SourceBlock::Heading { level, text } => {
                push(plan, current.take());

                while open_levels.last().is_some_and(|open| *open >= *level) {
                    open_levels.pop();
                }
                let previous_depth = open_levels.len();
                open_levels.push(*level);
                let indent = base_indent + previous_depth as i64;

                let mut diagnostics = Vec::new();
                if let Some(previous) = open_levels.iter().rev().nth(1)
                    && level.saturating_sub(*previous) > 1
                {
                    diagnostics.push(ImportDiagnostic::HeadingLevelJump {
                        title: text.clone(),
                        from: *previous,
                        to: *level,
                    });
                }

                let (title_text, stripped) = match title::extract_leading_ordinal(text) {
                    // A heading that is *only* an ordinal keeps its own text —
                    // "7" as a title is better than an untitled row, and the
                    // writer can clear it in review.
                    Some(e) if e.remaining_title.is_empty() => (text.clone(), None),
                    Some(e) => {
                        let label = ordinal_label(&e);
                        (e.remaining_title, Some(label))
                    }
                    None => (text.clone(), None),
                };

                current = Some(PlannedRow {
                    indent,
                    create_type: rules.kind_for(*level),
                    title: title_text,
                    stripped_ordinal: stripped,
                    djot: String::new(),
                    scene_breaks: 0,
                    word_count: 0,
                    origin: doc.origin.clone(),
                    included: true,
                    diagnostics,
                });
            }
            SourceBlock::Prose { djot } => {
                let row = current.get_or_insert_with(|| leading_row(doc, rules, base_indent));
                append_djot(&mut row.djot, djot);
            }
            SourceBlock::SceneBreak { tier } => {
                let row = current.get_or_insert_with(|| leading_row(doc, rules, base_indent));
                append_djot(&mut row.djot, scene_break::canonical_djot(*tier));
                row.scene_breaks += 1;
            }
        }
    }
    push(plan, current.take());
}

/// The row that collects prose appearing before a document's first heading.
///
/// Manuskript leaves this case unresolved in its own source comment, letting the
/// text inherit a title it has no claim to. Giving it a row of its own named for
/// the document is duller and correct: nothing is lost and nothing is misfiled.
fn leading_row(doc: &SourceDocument, rules: &LevelRules, base_indent: i64) -> PlannedRow {
    PlannedRow {
        indent: base_indent,
        create_type: rules.kind_for(u8::MAX),
        title: doc.effective_title().to_string(),
        stripped_ordinal: None,
        djot: String::new(),
        scene_breaks: 0,
        word_count: 0,
        origin: doc.origin.clone(),
        included: true,
        diagnostics: Vec::new(),
    }
}

fn append_djot(buffer: &mut String, addition: &str) {
    if addition.trim().is_empty() {
        return;
    }
    if !buffer.is_empty() {
        buffer.push_str("\n\n");
    }
    buffer.push_str(addition.trim_end());
}

fn ordinal_label(e: &title::ExtractedOrdinal) -> String {
    match (&e.keyword, e.numeral) {
        (Some(kw), Some(n)) => format!("{kw} {n}"),
        (None, Some(n)) => n.to_string(),
        (Some(kw), None) => kw.clone(),
        (None, None) => String::new(),
    }
}

/// A title appearing more than once is the shape a second import of the same
/// files takes. Not an error — a manuscript may hold two scenes called "Later" —
/// but it is worth saying before it doubles somebody's novel.
fn flag_duplicate_titles(plan: &mut ImportPlan) {
    let mut counts: std::collections::BTreeMap<String, usize> = Default::default();
    for row in &plan.rows {
        let key = row.title.trim().to_lowercase();
        if !key.is_empty() {
            *counts.entry(key).or_default() += 1;
        }
    }
    for (title, occurrences) in counts {
        if occurrences > 1 {
            plan.diagnostics
                .push(ImportDiagnostic::DuplicateTitle { title, occurrences });
        }
    }
}

/// Catch a row whose prose its own type cannot hold.
///
/// The backend does not check this: `validate_item` is called only by the UI, and
/// `content_allowed` is enforced only at save time, where it *drops the row
/// silently*. Catching it here is what turns "your chapter lost its text
/// sometime later" into a warning on the row, while it can still be retyped.
fn flag_illegal_combinations(plan: &mut ImportPlan, chapter_mode: &ChapterMode) {
    for row in &mut plan.rows {
        if row.djot.trim().is_empty() {
            continue;
        }
        let (role, sub_role) = row.create_type.combo(chapter_mode.clone());
        let holds_prose = allowed_content(&role, &sub_role).iter().any(is_prose_role);
        if !holds_prose {
            row.diagnostics.push(ImportDiagnostic::IllegalCombination {
                title: row.title.clone(),
                kind: row.create_type,
            });
        }
    }
}

fn is_prose_role(role: &ContentRole) -> bool {
    matches!(
        role,
        ContentRole::SceneText | ContentRole::NoteText | ContentRole::ParatextText
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structure::infer_rules;
    use skribisto_model::scene_break::SceneBreakTier;

    fn doc(origin: &str, blocks: Vec<SourceBlock>) -> SourceDocument {
        let mut d = SourceDocument::new("fixture", origin);
        d.blocks = blocks;
        d
    }

    fn heading(level: u8, text: &str) -> SourceBlock {
        SourceBlock::Heading {
            level,
            text: text.into(),
        }
    }

    fn prose(s: &str) -> SourceBlock {
        SourceBlock::Prose { djot: s.into() }
    }

    #[test]
    fn a_chapter_with_breaks_is_one_row_carrying_them_inline() {
        let d = doc(
            "a.md",
            vec![
                heading(1, "Chapter One"),
                prose("First."),
                SourceBlock::SceneBreak {
                    tier: SceneBreakTier::Minor,
                },
                prose("Second."),
                SourceBlock::SceneBreak {
                    tier: SceneBreakTier::Minor,
                },
                prose("Third."),
            ],
        );
        let rules = infer_rules(&[1], CreateType::Chapter);
        let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

        assert_eq!(plan.rows.len(), 1, "breaks must not create rows");
        let row = &plan.rows[0];
        assert_eq!(row.scene_breaks, 2);
        assert!(
            row.djot
                .contains(scene_break::canonical_djot(SceneBreakTier::Minor))
        );
        assert_eq!(row.word_count, 3, "markers are furniture, not words");
    }

    /// A book split across files shares one heading convention, so the ladder
    /// has to span them. Per document, this `## Chapter Two` — the only heading
    /// in its file — became a top-level row: type Chapter (the rules are global)
    /// at indent 0, i.e. a chapter sitting *outside* the book its sibling is in.
    #[test]
    fn the_heading_ladder_spans_every_document_in_one_import() {
        let first = doc(
            "01.md",
            vec![heading(1, "Book"), heading(2, "Chapter One"), prose("x")],
        );
        let second = doc("02.md", vec![heading(2, "Chapter Two"), prose("y")]);

        let rules = infer_rules(&[1, 2], CreateType::Book);
        let plan = build_plan(&[first, second], &rules, ChapterMode::Folder, 0);

        let shape: Vec<(i64, &str)> = plan
            .rows
            .iter()
            .map(|r| (r.indent, r.title.as_str()))
            .collect();
        assert_eq!(
            shape,
            vec![(0, "Book"), (1, "Chapter One"), (1, "Chapter Two")],
            "both chapters belong to the book"
        );
    }

    /// …and a later file that opens at the *top* level still starts over, which
    /// is what makes the ordinary one-file-per-chapter export work.
    #[test]
    fn a_later_document_reopening_the_top_level_returns_to_the_top() {
        let first = doc(
            "01.md",
            vec![heading(1, "One"), heading(2, "A scene"), prose("x")],
        );
        let second = doc("02.md", vec![heading(1, "Two"), prose("y")]);

        let rules = infer_rules(&[1, 2], CreateType::Chapter);
        let plan = build_plan(&[first, second], &rules, ChapterMode::Folder, 0);

        let shape: Vec<(i64, &str)> = plan
            .rows
            .iter()
            .map(|r| (r.indent, r.title.as_str()))
            .collect();
        assert_eq!(shape, vec![(0, "One"), (1, "A scene"), (0, "Two")]);
    }

    /// Word counting moved out of the per-document loop; it must still be right
    /// for every row of a multi-document import, not only the last one's.
    #[test]
    fn every_document_gets_its_words_counted() {
        let first = doc("01.md", vec![heading(1, "One"), prose("one two three")]);
        let second = doc("02.md", vec![heading(1, "Two"), prose("four five")]);

        let rules = infer_rules(&[1], CreateType::Chapter);
        let plan = build_plan(&[first, second], &rules, ChapterMode::Folder, 0);

        let counts: Vec<usize> = plan.rows.iter().map(|r| r.word_count).collect();
        assert_eq!(counts, vec![3, 2]);
    }

    #[test]
    fn nested_headings_become_indents() {
        let d = doc(
            "a.md",
            vec![
                heading(1, "Book"),
                heading(2, "Chapter One"),
                prose("x"),
                heading(2, "Chapter Two"),
                prose("y"),
            ],
        );
        let rules = infer_rules(&[1, 2], CreateType::Book);
        let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

        let shape: Vec<(i64, &str)> = plan
            .rows
            .iter()
            .map(|r| (r.indent, r.title.as_str()))
            .collect();
        assert_eq!(
            shape,
            vec![(0, "Book"), (1, "Chapter One"), (1, "Chapter Two")]
        );
    }

    /// A skipped level nests one step and says so, rather than growing the
    /// phantom folders Scrivener is documented to produce.
    #[test]
    fn a_skipped_heading_level_nests_one_step_and_is_reported() {
        let d = doc(
            "a.md",
            vec![heading(1, "Book"), heading(4, "Deep"), prose("x")],
        );
        let rules = infer_rules(&[1, 4], CreateType::Book);
        let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

        assert_eq!(plan.rows[1].indent, 1);
        assert!(
            plan.rows[1]
                .diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::HeadingLevelJump { .. }))
        );
    }

    #[test]
    fn prose_before_the_first_heading_gets_its_own_row() {
        let d = doc(
            "a.md",
            vec![prose("Preamble."), heading(1, "Chapter One"), prose("x")],
        );
        let rules = infer_rules(&[1], CreateType::Chapter);
        let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

        assert_eq!(plan.rows.len(), 2);
        assert_eq!(plan.rows[0].djot, "Preamble.");
        assert_eq!(plan.rows[1].title, "Chapter One");
    }

    #[test]
    fn an_ordinal_comes_off_the_title_and_is_kept() {
        let d = doc("a.md", vec![heading(1, "Chapter 3: The Storm"), prose("x")]);
        let rules = infer_rules(&[1], CreateType::Chapter);
        let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

        assert_eq!(plan.rows[0].title, "The Storm");
        assert_eq!(plan.rows[0].stripped_ordinal.as_deref(), Some("chapter 3"));
    }

    #[test]
    fn sixty_headingless_files_become_sixty_rows() {
        let docs: Vec<SourceDocument> = (0..60)
            .map(|i| doc(&format!("{i}.md"), vec![prose("Scene prose.")]))
            .collect();
        let rules = infer_rules(&[], CreateType::Scene);
        let plan = build_plan(&docs, &rules, ChapterMode::Folder, 0);

        assert_eq!(plan.rows.len(), 60);
        assert!(plan.rows.iter().all(|r| r.indent == 0));
    }

    #[test]
    fn base_indent_offsets_the_whole_import() {
        let d = doc(
            "a.md",
            vec![heading(1, "Chapter"), heading(2, "Scene"), prose("x")],
        );
        let rules = infer_rules(&[1, 2], CreateType::Chapter);
        let plan = build_plan(&[d], &rules, ChapterMode::Folder, 3);

        assert_eq!(plan.rows[0].indent, 3);
        assert_eq!(plan.rows[1].indent, 4);
    }

    #[test]
    fn a_repeated_title_is_flagged_because_that_is_what_a_double_import_looks_like() {
        let d = doc(
            "a.md",
            vec![
                heading(1, "Later"),
                prose("x"),
                heading(1, "Later"),
                prose("y"),
            ],
        );
        let rules = infer_rules(&[1], CreateType::Chapter);
        let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

        assert!(
            plan.diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::DuplicateTitle { occurrences: 2, .. }))
        );
    }

    #[test]
    fn prose_landing_on_a_type_that_cannot_hold_it_is_flagged_not_dropped() {
        let d = doc(
            "a.md",
            vec![heading(1, "A Book"), prose("Prose in a Book.")],
        );
        // Built by hand: `infer_rules` now keeps the deepest level on something
        // prose-bearing, so inference no longer produces this pairing. The writer
        // still can, by retyping a row to Book in the review tree, and it must be
        // caught there rather than by `content_allowed` silently dropping the
        // text at the next save.
        let rules =
            LevelRules::from_table(std::collections::BTreeMap::from([(1, CreateType::Book)]));
        let plan = build_plan(&[d], &rules, ChapterMode::Folder, 0);

        assert_eq!(plan.rows[0].create_type, CreateType::Book);
        assert!(
            plan.rows[0]
                .diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::IllegalCombination { .. })),
            "a Book holds no prose, and save-time would drop it silently"
        );
    }
}
