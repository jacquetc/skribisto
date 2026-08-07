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
//!
//! ## Imported comments are anchored here, against the prose that will be stored
//!
//! A scanner captures an editor's comment against **one block's** plain text,
//! because a block is what survives into a row. This is where those become
//! [`PlannedComment`]s: the row's Djot is assembled, its plain text is re-derived
//! through the *same* parse the editor will do (`skrib_format::djot_plain_text`),
//! and every quote is proved against it by `comment_anchor::resolve` — the same
//! function that re-anchors a comment on every reopen.
//!
//! Proving rather than computing is the point. The offsets could be arrived at by
//! arithmetic, and they mostly would be right; a comment that is mostly right is
//! one silently attached to the wrong sentence, which the anchor module's own doc
//! calls the failure that "is invisible until someone's comment has silently moved".
//! A quote that does not survive the conversion is reported and the comment is kept
//! as an orphan the writer can see and act on — never quietly repositioned.

use common::entities::{CommentAnchorKind, CommentOrphanReason, ContentRole};
use skribisto_model::comment_anchor::{self, Anchor, Resolution};
use skribisto_model::scene_break;
use skribisto_model::{ChapterMode, CreateType, allowed_content};

use crate::block::{
    AnnotationKind, SourceAnnotation, SourceAnnotationReply, SourceBlock, SourceDocument,
};
use crate::diagnostics::ImportDiagnostic;
use crate::structure::LevelRules;
use crate::title;

/// One imported comment, anchored against the row it belongs to.
///
/// Shaped like the `Comment` entity it becomes, so `apply_document_import` reads it
/// field by field with no second interpretation of what an anchor means.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedComment {
    pub kind: CommentAnchorKind,
    /// In the row's own plain-text character space — what
    /// `skrib_format::djot_plain_text` reports for `PlannedRow::djot`, which is
    /// what the editor's document will report when the row is opened.
    pub anchor: Anchor,
    /// True when the quote could not be found in the converted prose. The comment is
    /// still created: an orphan the writer can see and re-place beats a comment that
    /// was thrown away, and beats one confidently pointed at the wrong sentence.
    pub orphaned: bool,
    pub orphan_reason: CommentOrphanReason,
    pub author: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub body: String,
    pub resolved: bool,
    pub replies: Vec<SourceAnnotationReply>,
}

impl PlannedComment {
    /// Turns in this thread — the opening comment plus its replies.
    pub fn turns(&self) -> usize {
        1 + self.replies.len()
    }
}

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
    /// Editors' comments arriving with this row's prose, already anchored against
    /// it. Empty for a format that carries none.
    pub comments: Vec<PlannedComment>,
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
            // Nothing became a row, so a comment on this document has no prose to
            // point into and nowhere to live. Named rather than dropped.
            for annotation in &doc.annotations {
                plan.diagnostics.push(ImportDiagnostic::CommentUnanchored {
                    path: doc.origin.clone(),
                    quote: body_preview(&annotation.body),
                });
            }
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

    anchor_comments(&mut plan);
    flag_duplicate_titles(&mut plan);
    flag_illegal_combinations(&mut plan, &chapter_mode);
    plan
}

/// Prove every imported comment against the prose that will actually be stored.
///
/// One `djot_plain_text` per row that carries comments — and none at all for the
/// common case of an import with none, which is every Markdown import.
///
/// The offsets the scanner supplied are a *hint*, not an answer: they are exact
/// arithmetic over the block texts, but the conversion between them and this row's
/// Djot is a real parse, and the only way to know the quote still points at the same
/// words is to look. `comment_anchor::resolve` is the same three-tier matcher that
/// re-anchors the comment on every reopen, so a comment that lands here lands there.
fn anchor_comments(plan: &mut ImportPlan) {
    for row in &mut plan.rows {
        if row.comments.is_empty() {
            continue;
        }
        let Ok((text, block_starts)) = skrib_format::djot_plain_text(&row.djot) else {
            // The Djot this planner just built failed to parse. Every comment on
            // the row becomes a comment on the row as a whole rather than a lie
            // about where it points.
            for comment in &mut row.comments {
                comment.kind = CommentAnchorKind::Document;
                comment.anchor = Anchor::default();
            }
            continue;
        };

        for comment in &mut row.comments {
            if comment.kind == CommentAnchorKind::Document {
                comment.anchor = Anchor::default();
                continue;
            }
            let is_paragraph = comment.kind == CommentAnchorKind::Paragraph;
            comment.anchor.block_ordinal =
                comment_anchor::block_of(&block_starts, comment.anchor.start);

            match comment_anchor::resolve(&text, &comment.anchor, is_paragraph, &block_starts) {
                Resolution::Anchored { start, length } => {
                    // Re-capture at the proven position, so what is stored was
                    // measured against this row's text rather than a block's.
                    let ordinal = comment_anchor::block_of(&block_starts, start);
                    let mut anchor = comment_anchor::capture(&text, start, start + length, ordinal);
                    anchor.block_span = comment.anchor.block_span.max(1);
                    comment.anchor = anchor;
                }
                Resolution::Orphan(reason) => {
                    comment.orphaned = true;
                    comment.orphan_reason = reason;
                    row.diagnostics.push(ImportDiagnostic::CommentUnanchored {
                        path: row.origin.clone(),
                        quote: body_preview(&comment.body),
                    });
                }
            }
        }
    }
}

/// A short, single-line rendering of a comment body, for a diagnostic.
///
/// The *body*, not the quote: a diagnostic naming "the passage beginning 'She
/// turned…'" reads as though the prose were at fault, while the writer recognises
/// their editor's note instantly.
fn body_preview(body: &str) -> String {
    let one_line = body.split_whitespace().collect::<Vec<_>>().join(" ");
    let chars: Vec<char> = one_line.chars().collect();
    if chars.len() <= 60 {
        one_line
    } else {
        format!("{}…", chars[..59].iter().collect::<String>())
    }
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
    // How long the current row's prose is in *plain text*, mirroring `append_djot`'s
    // `\n\n` join with the single `\n` it renders to. This is what rebases a
    // block-relative comment offset into a row-relative one; `anchor_comments` then
    // proves the result rather than trusting it.
    let mut plain_len = 0usize;

    let push = |plan: &mut ImportPlan, row: Option<PlannedRow>| {
        if let Some(row) = row
            && !(row.title.trim().is_empty() && row.djot.trim().is_empty())
        {
            plan.rows.push(row);
        }
    };

    for (block_index, block) in doc.blocks.iter().enumerate() {
        // Where this block's text begins inside the row it is about to join.
        let block_offset = if plain_len == 0 { 0 } else { plain_len + 1 };

        match block {
            SourceBlock::Heading { level, text } => {
                push(plan, current.take());
                plain_len = 0;

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
                    comments: Vec::new(),
                    diagnostics,
                });
            }
            SourceBlock::Prose { djot, text } => {
                let row = current.get_or_insert_with(|| leading_row(doc, rules, base_indent));
                append_djot(&mut row.djot, djot);
                plain_len = block_offset + text.chars().count();
            }
            SourceBlock::SceneBreak { tier } => {
                let row = current.get_or_insert_with(|| leading_row(doc, rules, base_indent));
                append_djot(&mut row.djot, scene_break::canonical_djot(*tier));
                row.scene_breaks += 1;
                plain_len = block_offset + scene_break::canonical_plain(*tier).chars().count();
            }
        }

        // Comments on this block belong to whichever row it just joined. A comment
        // on a *heading* has no prose to point into — a heading becomes a row's
        // title — so it becomes a comment on the row as a whole, which is exactly
        // what `CommentAnchorKind::Document` is for.
        for annotation in doc
            .annotations
            .iter()
            .filter(|a| a.block_index == block_index)
        {
            let row = current.get_or_insert_with(|| leading_row(doc, rules, base_indent));
            row.comments
                .push(planned_comment(annotation, block, block_offset));
        }
    }
    push(plan, current.take());
}

/// Rebase one scanned annotation onto the row its block joined.
///
/// The anchor arrives measured against the block's own text; shifting `start` by
/// where that block begins in the row is the whole of the rebasing. Everything else
/// — the quote, its context, whether it was truncated — is carried across untouched,
/// because it describes prose rather than position.
fn planned_comment(
    annotation: &SourceAnnotation,
    block: &SourceBlock,
    block_offset: usize,
) -> PlannedComment {
    let kind = match annotation.kind {
        // A heading is a title, not prose, so nothing inside it can be pointed at.
        _ if matches!(block, SourceBlock::Heading { .. }) => CommentAnchorKind::Document,
        AnnotationKind::Range => CommentAnchorKind::Range,
        AnnotationKind::Paragraph => CommentAnchorKind::Paragraph,
        AnnotationKind::Document => CommentAnchorKind::Document,
    };
    let mut anchor = annotation.anchor.clone();
    anchor.start += block_offset;
    anchor.block_span = anchor.block_span.max(1);

    PlannedComment {
        kind,
        anchor,
        orphaned: false,
        orphan_reason: CommentOrphanReason::NotOrphaned,
        author: annotation.author.clone(),
        created: annotation.created,
        body: annotation.body.clone(),
        resolved: annotation.resolved,
        replies: annotation.replies.clone(),
    }
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
        comments: Vec::new(),
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
        SourceBlock::prose(s, s)
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
