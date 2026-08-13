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
//!
//! ## This planner never mints `CommentAnchorKind::Document`
//!
//! Neither DOCX nor ODF has a "comment on the whole document" concept — `Document`
//! is a kind neither format can express, and the comment UI never wires it up
//! either (there is no surface to open one from). A comment landing on a *heading*
//! block used to become one; it becomes a `Paragraph` comment on the row's first
//! block instead (see `planned_comment`), and a comment on a row whose Djot
//! failed to parse becomes a `Paragraph` comment pinned to block 0 with an empty
//! quote (see `anchor_comments`) — both real, storable anchors rather than a
//! placeholder kind. `sources::rich` still mints `Document` for the two cases that
//! genuinely have no text to point at (a blank paragraph, a table) — a deliberate,
//! separate decision, not an oversight here. The enum variant itself is not
//! removed: comments minted before this change exist on disk and must keep
//! loading, and the UI (`docks::comments`) still renders them — as a comment that
//! resolved successfully yet has nowhere to point, never as one that silently
//! vanished.

use common::entities::{CommentAnchorKind, CommentOrphanReason, ContentRole};
use skribisto_model::comment_anchor::{self, Anchor, Resolution};
use skribisto_model::scene_break;
use skribisto_model::{ChapterMode, CreateType, allowed_content, content_allowed};

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
    /// In the row's own **addressable** character space — what
    /// `skrib_format::djot_plain_text` reports for `PlannedRow::djot`, which is
    /// what the editor's document will report when the row is opened.
    pub anchor: Anchor,
    /// True when the quote could not be found in the converted prose. The comment is
    /// still created: an orphan the writer can see and re-place beats a comment that
    /// was thrown away, and beats one confidently pointed at the wrong sentence.
    pub orphaned: bool,
    pub orphan_reason: CommentOrphanReason,
    /// Carried straight across from `SourceAnnotation::uid` (M-S7) — the identity
    /// this comment carried in the source file, when that file is one Skribisto
    /// itself exported. `apply_document_import_uc` is what turns this into "update
    /// the matching `Comment` row" rather than "create a new one" — see its own
    /// module doc for the recognition rule and what it does and does not update.
    pub uid: Option<uuid::Uuid>,
    /// See [`crate::block::SourceAnnotation::uid_tag`] — the identity that survives an
    /// editor's save, and therefore the one that does the recognising in practice. A *lookup
    /// key* rather than a uid: `apply_document_import_uc` hashes each comment the project
    /// already holds and matches on the result.
    pub uid_tag: Option<String>,
    pub author: String,
    /// See `SourceAnnotation::author_initials` — empty means "none".
    pub author_initials: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub body: String,
    pub resolved: bool,
    /// Each reply already carries its own `SourceAnnotationReply::uid` /
    /// `author_initials` — nothing to rebase here, since a reply has no anchor of
    /// its own to prove against this row's Djot.
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
    /// The epigraph this row heads, already Djot (one blockquote per quotation), or
    /// empty when it has none — which is every row of every format that carries no
    /// style information, and most rows of the ones that do.
    ///
    /// A field of its own rather than part of [`Self::djot`] because it becomes a
    /// *different* `Content`: `ContentRole::EpigraphText`, not the row's prose. Folding
    /// it into the prose is precisely the bug this exists to fix — it duplicates the
    /// quotation on every round trip and adds its words to the manuscript's count, which
    /// `skribisto_model`'s `an_epigraph_is_never_counted_as_prose` exists to forbid.
    ///
    /// Only ever non-empty on a row whose type can hold one (Part or Chapter). An
    /// epigraph beside any other heading falls back to prose with an
    /// [`ImportDiagnostic::EpigraphNotCarried`], so `apply` never has to decide.
    pub epigraph: String,
    /// How many scene breaks the prose carries. Shown per row so the writer can
    /// see the granularity they are getting before committing to it — a chapter
    /// reading "4,100 words, 3 breaks" is one item, not four.
    pub scene_breaks: usize,
    /// Words in the prose, markers excluded.
    pub word_count: usize,
    /// Which source this came from — the full path, and it goes no further than
    /// this crate and the review step. See [`SourceDocument::origin`].
    pub origin: String,
    /// blake3 of that source file's own bytes. See
    /// [`SourceDocument::source_file_digest`], and note that it answers a
    /// different question from [`Self::source_digest`] a few fields down: this
    /// one is of the document, that one is of this row's prose as it was
    /// exported.
    pub source_file_digest: String,
    /// Whether to create it. The review step unchecks rather than deletes.
    pub included: bool,
    /// Editors' comments arriving with this row's prose, already anchored against
    /// it. Empty for a format that carries none.
    pub comments: Vec<PlannedComment>,
    /// The `BinderItem` this row *was*, when the file being imported is one this project
    /// exported — recovered from a round-trip mark. `None` for a first arrival, for a file
    /// from anywhere else, and for a row the writer added inside the file.
    ///
    /// A lookup key rather than a uid; see [`PlannedComment::uid_tag`].
    pub source_uid_tag: Option<String>,
    /// The digest of this row's prose **as it was exported**, from the same mark. Compared
    /// against the digest of the prose in this file and of the prose the project holds now, it
    /// answers "who changed this row" — the baseline of a three-way merge, carried in the file
    /// rather than stored anywhere. `None` whenever `source_uid_tag` is.
    pub source_digest: Option<String>,
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
        append_document(
            &mut plan,
            doc,
            rules,
            &chapter_mode,
            base_indent,
            &mut open_levels,
        );
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
            // The Djot this planner just built failed to parse, so there is no
            // parsed text to prove any quote against — not even "the row's first
            // block", which is what a heading comment falls back to below, and
            // which depends on this very parse having succeeded.
            //
            // This is *not* `CommentAnchorKind::Document`: that kind is reserved
            // for a format that genuinely has no text to point into (a blank
            // paragraph, a table — see `sources::rich`'s module doc), and it is
            // neither DOCX nor ODF's vocabulary, nor one the comment UI ever
            // wires up. A row whose Djot failed to parse is not that — it is an
            // ordinary paragraph comment whose text simply is not available
            // *yet*. So every comment on the row becomes a `Paragraph` comment
            // pinned to block 0 with an empty quote: exactly the tier-3 fallback
            // `comment_anchor::resolve` already falls back to when a paragraph's
            // wording cannot be found (`_ => anchor.block_ordinal`). It cannot be
            // resolved here — there is no parsed text to resolve it against —
            // but it is a real, storable anchor rather than a zero-length
            // placeholder tied to a kind the UI cannot render, so the live
            // editor's own re-anchor pass places it on block 0 the first time
            // the row is actually opened (by which point its Djot, whatever this
            // planner built, is what the editor parses too).
            pin_to_first_block(&mut row.comments);
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

/// Give every comment in `comments` a `Paragraph` anchor pinned to block 0 with
/// an empty quote — the parse-failure fallback `anchor_comments` uses, pulled out
/// so it is unit-testable on its own. In practice `djot_plain_text` failing on
/// Djot this planner just built is vanishingly rare (the parser it wraps is
/// lenient rather than rejecting), so this path is exercised directly here
/// rather than by trying to manufacture a real parse failure.
fn pin_to_first_block(comments: &mut [PlannedComment]) {
    for comment in comments {
        comment.kind = CommentAnchorKind::Paragraph;
        comment.anchor = Anchor {
            block_span: 1,
            ..Anchor::default()
        };
    }
}

/// A short, single-line rendering of a comment body, for a diagnostic.
///
/// The *body*, not the quote: a diagnostic naming "the passage beginning 'She
/// turned…'" reads as though the prose were at fault, while the writer recognises
/// their editor's note instantly.
///
/// `body` is Djot (M-S4: an imported comment can carry its own emphasis), and this
/// is a plain-text preview — `skrib_format::djot_plain_text` strips the markup
/// *before* the whitespace collapse and the 60-character truncation run over it,
/// so a bold word does not turn the review panel into `"...the *right* word..."`
/// with the asterisks counted as visible characters the writer never typed.
///
/// `skrib_format` (already a dependency here, via `anchor_comments`'s own use of
/// its `djot_plain_text`) rather than pulling in `text-document` directly: a
/// second, direct dependency on it would need docx-rs unified against
/// `document_io`'s own exact `=0.4.21` pin, for a function this crate does not
/// otherwise need. On the vanishingly rare parse failure this falls back to the
/// raw Djot rather than losing the preview outright — `djot_plain_text`'s own
/// doc calls that failure "vanishingly rare" for exactly this reason: the parser
/// it wraps is lenient rather than rejecting.
fn body_preview(body: &str) -> String {
    let plain = skrib_format::djot_plain_text(body)
        .map(|(text, _)| text)
        .unwrap_or_else(|_| body.to_string());
    let one_line = plain.split_whitespace().collect::<Vec<_>>().join(" ");
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
    // Only ever read to answer "can this row hold an epigraph" — `Chapter` is the one
    // `CreateType` whose `(role, sub_role)` depends on it, and it is one of the two types
    // that can.
    chapter_mode: &ChapterMode,
    base_indent: i64,
    // Heading level → the indent its row sits at. Rebuilt as levels are met so a
    // document that skips a level (`#` then `####`) nests one step, not three:
    // the phantom-folder failure other importers are documented to produce. Owned
    // [`build_plan`] and carried across documents — see the note there.
    open_levels: &mut Vec<u8>,
) {
    // The row currently collecting prose. A document may open with prose before
    // any heading, which becomes a row of its own rather than being silently
    // attached to the first heading that follows.
    let mut current: Option<PlannedRow> = None;
    // An epigraph that belongs to the row the *next* heading will open — the
    // `EpigraphPlacement::BeforeHeading` shape, where the quotation opens the chapter
    // above its own title. Carried rather than attached on sight, because the row it
    // belongs to does not exist yet.
    let mut deferred_epigraph = String::new();
    // Whether `current` was opened by a heading and has taken nothing since. An epigraph
    // is its row's only when it sits *immediately* under the heading; one that follows a
    // paragraph of the scene is a quotation inside the scene, which is a different thing
    // and stays where the writer put it.
    let mut at_row_head = false;
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
                    // An epigraph held over from before this heading is this row's, and
                    // `epigraph_block` has already proven this type can hold one — it is
                    // only ever deferred when the following heading's own type passed.
                    epigraph: std::mem::take(&mut deferred_epigraph),
                    scene_breaks: 0,
                    word_count: 0,
                    origin: doc.origin.clone(),
                    source_file_digest: doc.source_file_digest.clone(),
                    included: true,
                    comments: Vec::new(),
                    // Filled by the mark loop below, once this row is the current one.
                    source_uid_tag: None,
                    source_digest: None,
                    diagnostics,
                });
                at_row_head = true;
            }
            SourceBlock::Prose { djot, text } => {
                let row = current.get_or_insert_with(|| leading_row(doc, rules, base_indent));
                append_djot(&mut row.djot, djot);
                plain_len = block_offset + text.chars().count();
                at_row_head = false;
            }
            SourceBlock::SceneBreak { tier } => {
                let row = current.get_or_insert_with(|| leading_row(doc, rules, base_indent));
                append_djot(&mut row.djot, scene_break::canonical_djot(*tier));
                row.scene_breaks += 1;
                plain_len = block_offset + scene_break::canonical_plain(*tier).chars().count();
                at_row_head = false;
            }
            SourceBlock::Epigraph { djot, text } => {
                // Decided here, from the file's own shape, and never from the export
                // preset that wrote it: `EpigraphPlacement` is a choice made on the way
                // *out*, recorded nowhere in the file, and absent entirely from a
                // document this app did not produce. What the file does say is which
                // heading the quotation is touching, and that is what is read.
                let placed = epigraph_block(
                    EpigraphContext {
                        doc,
                        rules,
                        chapter_mode,
                        block_index,
                        at_row_head,
                    },
                    djot,
                    &mut current,
                    &mut deferred_epigraph,
                    || leading_row(doc, rules, base_indent),
                );
                if placed == EpigraphPlacementOutcome::KeptAsProse {
                    // Not an epigraph after all — a quotation somewhere in the scene, or
                    // one beside a heading whose type cannot hold it. Either way it is
                    // this row's prose, and it advances the offsets like any other.
                    plain_len = block_offset + text.chars().count();
                    at_row_head = false;
                }
            }
        }

        // Comments on this block belong to whichever row it just joined. A comment
        // on a *heading* has no prose to point into — a heading becomes a row's
        // title — so `planned_comment` gives it a `Paragraph` anchor on the row's
        // first block instead of a quote it could never carry.
        for annotation in doc
            .annotations
            .iter()
            .filter(|a| a.block_index == block_index)
        {
            let row = current.get_or_insert_with(|| leading_row(doc, rules, base_indent));
            row.comments
                .push(planned_comment(annotation, block, block_offset));
        }

        // A round-trip mark on this block names the row the block just joined. **First mark
        // wins**: a row's identity is written once, at its first character, so a second mark
        // inside the same row usually means the writer's chapters have been merged in the
        // editor — in which case the row genuinely is the first of them, and quietly adopting
        // the second's identity would move the other chapter's history onto this one.
        //
        // ⚠ There is one case where "usually" is wrong, and it is left wrong on purpose. A row
        // only ever opens on a `Heading`, so an exported row that carries **no heading of its
        // own** — a paratext, by design — has its mark folded into whichever row precedes it.
        // Under every shipped preset that is harmless, and in fact right: a Book earns no mark
        // (its type stores no prose), so the paratext's is the first and the merged row pairs
        // back to the paratext. It only bites with `include_synopses` on *and* a non-empty
        // synopsis on the Book itself, where the Book's mark arrives first and the row pairs
        // against a Book — which stores no prose, so `apply_document_import` refuses the whole
        // import rather than losing it.
        //
        // Fixing it means letting a disagreeing mark open a new row, which would also split a
        // genuinely merged chapter back in two and lose the editor's merge. That is a trade
        // between two wrong answers and wants a decision, not a quiet preference, so it is
        // recorded here rather than made.
        for mark in doc
            .row_marks
            .iter()
            .filter(|m| m.block_index == block_index)
        {
            let row = current.get_or_insert_with(|| leading_row(doc, rules, base_indent));
            if row.source_uid_tag.is_none() {
                row.source_uid_tag = Some(mark.uid_tag.clone());
                row.source_digest = Some(mark.digest.clone());
            }
        }
    }
    push(plan, current.take());
}

/// What [`epigraph_block`] did with the quotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EpigraphPlacementOutcome {
    /// It became a row's `EpigraphText` — either the row above it or the one the next
    /// heading is about to open.
    Attached,
    /// It stayed in the prose stream, as the quotation it visibly is.
    KeptAsProse,
}

/// Everything `epigraph_block` needs to read about *where* the quotation sits.
struct EpigraphContext<'a> {
    doc: &'a SourceDocument,
    rules: &'a LevelRules,
    chapter_mode: &'a ChapterMode,
    block_index: usize,
    at_row_head: bool,
}

/// Decide which row an epigraph belongs to, from adjacency alone.
///
/// An epigraph heads a part or a chapter, and both editorial placements put it against
/// that heading — after it (Chicago, French, German and Russian practice, and this
/// compiler's default) or above it (the `\epigraphhead` shape LaTeX ships). So the
/// question is only ever *which* of the two headings it is touching, and the answer is
/// read off the block stream:
///
/// 1. **The heading above**, when the quotation sits immediately under it and that row's
///    type can hold an epigraph. The convention, so it wins.
/// 2. **The heading below**, when there is no heading above it or the one above cannot
///    hold an epigraph — a Book's, most often, since a Book heads its own front matter
///    and the matrix gives it no `EpigraphText`.
/// 3. **Neither**, and it stays prose. A quotation in the middle of a scene is a
///    quotation in the middle of a scene, and moving it would be an invention; one
///    beside a heading that cannot hold it is reported, because there the writer did
///    mean an epigraph and needs to know it did not become one.
///
/// When 1 and 2 are *both* available the writer is told (an
/// [`ImportDiagnostic::EpigraphPlacementAmbiguous`]) rather than the tie being resolved
/// silently — the two placements are genuinely both real practice.
fn epigraph_block(
    ctx: EpigraphContext<'_>,
    djot: &str,
    current: &mut Option<PlannedRow>,
    deferred: &mut String,
    make_leading_row: impl Fn() -> PlannedRow,
) -> EpigraphPlacementOutcome {
    // The heading immediately below, if the very next block is one. Blank blocks never
    // reach `SourceDocument::blocks`, so "the next block" really is the next thing in
    // the document.
    let below = match ctx.doc.blocks.get(ctx.block_index + 1) {
        Some(SourceBlock::Heading { level, text }) => Some((ctx.rules.kind_for(*level), text)),
        _ => None,
    };

    let above_ok = ctx.at_row_head
        && current
            .as_ref()
            .is_some_and(|row| carries_epigraph(row.create_type, ctx.chapter_mode));
    let below_ok = below.is_some_and(|(kind, _)| carries_epigraph(kind, ctx.chapter_mode));

    if above_ok {
        let row = current.as_mut().expect("above_ok proved it is Some");
        if let Some((_, below_title)) = below.filter(|_| below_ok) {
            row.diagnostics
                .push(ImportDiagnostic::EpigraphPlacementAmbiguous {
                    above: row.title.clone(),
                    below: below_title.clone(),
                });
        }
        // Appended, never assigned: a row may legitimately head two quotations, and
        // `mark_epigraph` marks each blockquote separately on the way back out.
        append_djot(&mut row.epigraph, djot);
        return EpigraphPlacementOutcome::Attached;
    }

    if below_ok {
        append_djot(deferred, djot);
        return EpigraphPlacementOutcome::Attached;
    }

    // Report only when it was *beside* a heading. A quotation mid-scene was never
    // claiming to be an epigraph, and warning about every one of them would train the
    // writer to ignore the warning that matters.
    let beside = if ctx.at_row_head {
        current.as_ref().map(|r| (r.title.clone(), r.create_type))
    } else {
        below.map(|(kind, title)| (title.clone(), kind))
    };
    let row = current.get_or_insert_with(make_leading_row);
    if let Some((title, kind)) = beside {
        row.diagnostics
            .push(ImportDiagnostic::EpigraphNotCarried { title, kind });
    }
    append_djot(&mut row.djot, djot);
    EpigraphPlacementOutcome::KeptAsProse
}

/// Whether a row of this type may hold a `ContentRole::EpigraphText`.
///
/// Asks `skribisto_model` rather than listing the four combinations here — the
/// constraint matrix is the one authority, and a second copy of it would drift the first
/// time a row is added to it.
fn carries_epigraph(create_type: CreateType, chapter_mode: &ChapterMode) -> bool {
    let (role, sub_role) = create_type.combo(chapter_mode.clone());
    content_allowed(&role, &sub_role, &ContentRole::EpigraphText)
}

/// Rebase one scanned annotation onto the row its block joined.
///
/// The anchor arrives measured against the block's own text; shifting `start` by
/// where that block begins in the row is the whole of the rebasing. Everything else
/// — the quote, its context, whether it was truncated — is carried across untouched,
/// because it describes prose rather than position. The one exception is a
/// heading's annotation, which carries no offset across at all — see below.
fn planned_comment(
    annotation: &SourceAnnotation,
    block: &SourceBlock,
    block_offset: usize,
) -> PlannedComment {
    // Both of these are blocks whose text is *not* part of the row's Djot — a heading
    // becomes the row's title, an epigraph becomes a `Content` of its own — so an offset
    // into either would point at unrelated words once the row's prose is assembled. They
    // take the same treatment for the same reason.
    //
    // An epigraph carries no comment layer in the editor either (see `OpenDoc::build`),
    // so there is no anchor for an imported remark to keep even in principle; becoming a
    // paragraph comment on the row's first block is what keeps the editor's words instead
    // of dropping them.
    let is_heading = matches!(
        block,
        SourceBlock::Heading { .. } | SourceBlock::Epigraph { .. }
    );
    let kind = match annotation.kind {
        // A heading is a title, not prose — it becomes the row's `title` field,
        // never its Djot — so nothing inside it can be pointed at with a quote.
        //
        // This used to become `CommentAnchorKind::Document`: a kind neither DOCX
        // nor ODF (the only formats that carry comments at all) can express, and
        // one the comment UI never wires up — see the crate's design note on why
        // the importer stopped minting it. It becomes a `Paragraph` comment
        // instead, with an empty quote and no offset carried across (below):
        // `anchor_comments` re-derives that, once the row's Djot is fully
        // assembled, into a real anchor on the row's *first* block — the same
        // tier-3 "fall back to the block ordinal" path `comment_anchor::resolve`
        // already takes for a paragraph comment whose wording cannot be found.
        _ if is_heading => CommentAnchorKind::Paragraph,
        AnnotationKind::Range => CommentAnchorKind::Range,
        AnnotationKind::Paragraph => CommentAnchorKind::Paragraph,
        AnnotationKind::Document => CommentAnchorKind::Document,
    };
    // A heading's own offset describes a position inside the *title*, which is
    // never part of the row's Djot — carrying it across (`+= block_offset`)
    // would hand `anchor_comments`' block lookup an offset that happens to land
    // in some unrelated block instead of the row's first one. Starting from a
    // blank anchor is what makes the block-ordinal fallback described above
    // land on block 0, deliberately, rather than by accident.
    let mut anchor = if is_heading {
        Anchor::default()
    } else {
        annotation.anchor.clone()
    };
    if !is_heading {
        anchor.start += block_offset;
    }
    anchor.block_span = anchor.block_span.max(1);

    PlannedComment {
        kind,
        anchor,
        orphaned: false,
        orphan_reason: CommentOrphanReason::NotOrphaned,
        uid: annotation.uid,
        uid_tag: annotation.uid_tag.clone(),
        author: annotation.author.clone(),
        author_initials: annotation.author_initials.clone(),
        created: annotation.created,
        body: annotation.body.clone(),
        resolved: annotation.resolved,
        replies: annotation.replies.clone(),
    }
}

/// The row that collects prose appearing before a document's first heading.
///
/// One surveyed tool leaves this case unresolved in its own source comment, letting
/// the text inherit a title it has no claim to. Giving it a row of its own named for
/// the document is duller and correct: nothing is lost and nothing is misfiled.
fn leading_row(doc: &SourceDocument, rules: &LevelRules, base_indent: i64) -> PlannedRow {
    PlannedRow {
        indent: base_indent,
        create_type: rules.kind_for(u8::MAX),
        title: doc.effective_title().to_string(),
        stripped_ordinal: None,
        djot: String::new(),
        // Never an epigraph: this row exists because prose arrived before any heading,
        // and an epigraph with no heading to head is not one.
        epigraph: String::new(),
        scene_breaks: 0,
        word_count: 0,
        origin: doc.origin.clone(),
        source_file_digest: doc.source_file_digest.clone(),
        included: true,
        comments: Vec::new(),
        source_uid_tag: None,
        source_digest: None,
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
mod tests;
