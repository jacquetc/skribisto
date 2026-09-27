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
//! Neither DOCX nor ODF has a "comment on the whole document" concept: `Document`
//! is a kind neither format can express, and the comment UI never wires it up
//! either (there is no surface to open one from). A comment landing on a *heading*
//! block becomes a `Paragraph` comment on the row's first block (see
//! `planned_comment`), a comment on a row whose Djot failed to parse becomes a
//! `Paragraph` comment pinned to block 0 with an empty quote (see
//! [`assemble_row`]), and a `Document` annotation a scanner still hands over becomes
//! a `Paragraph` comment on the block it names. `sources::rich` no longer mints one at
//! all. The enum variant itself is not removed: comments minted before this change
//! exist on disk and must keep loading, and the UI (`docks::comments`) still renders
//! them, as a comment that resolved successfully yet has nowhere to point, never as
//! one that silently vanished.
//!
//! ## Every comment lands on a row that stores prose
//!
//! A comment hangs off the `Content` holding its row's prose, so a row that stores none
//! (a book title directly above a chapter, a heading with no text under it, a row not
//! created at all) cannot hold one. [`build_plan`] moves such a comment to the nearest
//! paragraph that is stored and reports it once, as it reports a comment the scanner
//! already had to move. Only a file with no stored prose at all leaves a comment nowhere
//! to go, and that is reported as a comment not imported.
//!
//! ## One row, one assembly
//!
//! [`build_plan`] decides which blocks make which row: where headings open rows, and
//! which row an epigraph belongs to. Everything after that, for one row, is
//! [`assemble_row`]: its Djot, its epigraph, its comments proved against the prose it
//! stores, its footnotes, its scene breaks and its word count. A converter that already
//! knows its rows (one per document, say) calls it directly and gets the same anchoring
//! the document importer uses, since there is no second anchoring stage to drift from it.

use std::collections::HashSet;

use common::entities::{CommentAnchorKind, CommentOrphanReason, ContentRole};
use skribisto_model::comment_anchor::{self, Anchor, Resolution};
use skribisto_model::counting::{CountMethod, cached_count};
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

/// One footnote arriving with a row's prose.
///
/// Paired to its row by the `[^label]` in that row's Djot, never by position — see
/// [`crate::block::SourceFootnote`] for why the label is a placeholder rather than
/// anything the writer chose. A note cited twice in one row appears once here; a
/// note cited from two rows appears on both, because
/// `apply_document_import` mints a `Footnote` per row and a `Footnote` belongs to
/// exactly one `Content`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlannedFootnote {
    /// The placeholder the row's Djot cites. Rewritten at apply time to a label the
    /// project has free.
    pub label: String,
    /// The note's own text, Djot.
    pub body: String,
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
    /// Footnotes this row's prose cites, in first-citation order. Empty for a
    /// format that carries none and for a row that cites none.
    pub footnotes: Vec<PlannedFootnote>,
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

/// What one row stores, assembled from its blocks and proved.
///
/// Narrow on purpose: everything here follows from the row's own blocks, and nothing
/// depends on where the row sits in a tree, what type it was given or which heading opened
/// it. [`build_plan`] adds those to make a [`PlannedRow`]; a converter that already knows
/// its rows takes this as it is.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AssembledProse {
    /// The row's prose, Djot, scene-break markers spliced in. Empty for a row whose
    /// blocks hold no prose.
    pub djot: String,
    /// The epigraph the row heads, Djot, one blockquote per quotation. See
    /// [`PlannedRow::epigraph`].
    pub epigraph: String,
    /// Every comment on the row's blocks, anchored against `djot`. Never
    /// `CommentAnchorKind::Document`.
    pub comments: Vec<PlannedComment>,
    /// The notes `djot` and `epigraph` cite, in first-citation order.
    pub footnotes: Vec<PlannedFootnote>,
    /// How many scene breaks `djot` carries.
    pub scene_breaks: usize,
    /// Words in `djot`, markers excluded, counted the way the app counts a row.
    pub word_count: usize,
    /// What went wrong on the way, for the row.
    pub diagnostics: Vec<ImportDiagnostic>,
}

/// What one block of a [`SourceDocument`] gives the row it joins, by index into
/// [`SourceDocument::blocks`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowPart {
    /// A heading whose text is the row's title. It adds nothing to the row's Djot; a
    /// comment on it becomes a paragraph comment on the row's first block.
    Title(usize),
    /// A prose block, a scene break, or an epigraph kept in the prose: appended to the
    /// row's Djot, its comments rebased onto it.
    Prose(usize),
    /// An epigraph the row heads: appended to the row's epigraph. Its comments become
    /// paragraph comments on the row's first block, since an epigraph carries no comment
    /// layer of its own.
    Epigraph(usize),
}

impl RowPart {
    fn block(self) -> usize {
        match self {
            RowPart::Title(index) | RowPart::Prose(index) | RowPart::Epigraph(index) => index,
        }
    }
}

/// Assemble one row from its blocks: Djot, epigraph, comments proved against the Djot,
/// footnotes, scene breaks and words.
///
/// The one place an imported comment is anchored. A block's comments arrive measured
/// against that block's own text; they are rebased onto the row, then every quote is
/// proved against the row's Djot through `skrib_format::djot_plain_text`, the parse the
/// editor will do, and `comment_anchor::resolve`, the matcher that re-anchors the comment
/// on every reopen. A quote that does not survive is kept as an orphan and reported, never
/// quietly moved. A comment the scanner already had to move to the nearest paragraph
/// ([`SourceAnnotation::unanchored`]) is reported too, once.
pub fn assemble_row(doc: &SourceDocument, parts: &[RowPart]) -> AssembledProse {
    assemble_parts(doc, parts, &doc.annotations)
}

/// [`assemble_row`], taking the row's comments from `annotations` rather than from the
/// document's own list: [`build_plan`] hands over that list with every comment its row
/// cannot hold moved to the nearest stored paragraph (see [`rehome_comments`]).
fn assemble_parts(
    doc: &SourceDocument,
    parts: &[RowPart],
    annotations: &[SourceAnnotation],
) -> AssembledProse {
    let mut prose = AssembledProse::default();
    // Parallel to `prose.comments`: whether each one already landed away from its words.
    let mut moved: Vec<bool> = Vec::new();
    // How long the row's prose is in *plain text*, mirroring `append_djot`'s `\n\n` join
    // with the single `\n` it reads back as. This is what rebases a block-relative
    // comment offset into a row-relative one; the proof below then checks the result.
    let mut plain_len = 0usize;

    for part in parts {
        let Some(block) = doc.blocks.get(part.block()) else {
            continue;
        };
        let block_offset = if plain_len == 0 { 0 } else { plain_len + 1 };
        let role = match *part {
            RowPart::Title(_) => PartRole::Title,
            RowPart::Epigraph(_) => {
                if let SourceBlock::Epigraph { djot, .. } | SourceBlock::Prose { djot, .. } = block
                {
                    // Appended, never assigned: a row may head two quotations, and
                    // `mark_epigraph` marks each blockquote separately on the way out.
                    append_djot(&mut prose.epigraph, djot);
                }
                PartRole::Title
            }
            RowPart::Prose(_) => {
                match block {
                    SourceBlock::Prose { djot, text } | SourceBlock::Epigraph { djot, text } => {
                        append_djot(&mut prose.djot, djot);
                        plain_len = block_offset + text.chars().count();
                    }
                    SourceBlock::SceneBreak { tier } => {
                        append_djot(&mut prose.djot, scene_break::canonical_djot(*tier));
                        prose.scene_breaks += 1;
                        plain_len =
                            block_offset + scene_break::canonical_plain(*tier).chars().count();
                    }
                    // A heading is never prose; `build_plan` never hands one over as such.
                    SourceBlock::Heading { .. } => {}
                }
                PartRole::Prose
            }
        };
        for annotation in annotations.iter().filter(|a| a.block_index == part.block()) {
            prose
                .comments
                .push(planned_comment(annotation, role, block_offset));
            moved.push(annotation.unanchored);
        }
    }

    anchor_comments(
        &prose.djot,
        &doc.origin,
        &mut prose.comments,
        &moved,
        &mut prose.diagnostics,
    );
    prose.footnotes = cited_footnotes(doc, &prose.djot, &prose.epigraph);
    // Counted the way the app counts a row once it is stored, so the review shows the
    // number the binder will: markers stripped, markup and attribute lines not words.
    prose.word_count = cached_count(&prose.djot, CountMethod::WhitespaceSplit).words;
    prose
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
                plan.diagnostics.push(ImportDiagnostic::CommentNotCarried {
                    path: doc.origin.clone(),
                    quote: body_preview(&annotation.body),
                });
            }
            continue;
        }

        let drafts = draft_rows(doc, rules, &chapter_mode, base_indent, &mut open_levels);
        let mut assembled: Vec<AssembledProse> = drafts
            .iter()
            .map(|draft| assemble_parts(doc, &draft.parts, &doc.annotations))
            .collect();
        // A row with neither a title nor prose is not created. What it held is not lost
        // with it: its comments are moved below, like any comment its row cannot hold.
        let created: Vec<bool> = drafts
            .iter()
            .zip(&assembled)
            .map(|(draft, prose)| !(draft.title.trim().is_empty() && prose.djot.trim().is_empty()))
            .collect();
        let homes = rehome_comments(doc, &drafts, &assembled, &created);
        plan.diagnostics.extend(homes.not_carried);
        if let Some(annotations) = homes.annotations {
            // Whether a row is created follows from its title and its Djot, and neither
            // depends on its comments, so the second assembly creates the same rows.
            assembled = drafts
                .iter()
                .map(|draft| assemble_parts(doc, &draft.parts, &annotations))
                .collect();
        }

        let mut cited: HashSet<String> = HashSet::new();
        for ((draft, prose), created) in drafts.into_iter().zip(assembled).zip(created) {
            if !created {
                continue;
            }
            cited.extend(prose.footnotes.iter().map(|f| f.label.clone()));
            plan.rows.push(draft.into_row(doc, prose));
        }

        // A note nothing cites is reported, not attached. It happens for real: a
        // footnote on a chapter *title* has nowhere to live, because a title is a plain
        // string on the `BinderItem` and a `Footnote` annotates a `Content`.
        let uncited = doc
            .footnotes
            .iter()
            .filter(|f| !cited.contains(&f.label))
            .count();
        if uncited > 0 {
            plan.diagnostics.push(ImportDiagnostic::FootnoteNotCarried {
                path: doc.origin.clone(),
                count: uncited,
            });
        }
    }

    flag_duplicate_titles(&mut plan);
    flag_illegal_combinations(&mut plan, &chapter_mode);
    plan
}

/// The notes a row's prose and epigraph cite, in first-citation order.
///
/// Pairing is by the `[^label]` the scanner wrote into the prose, matched with
/// `skribisto_model::footnote_numbering::references_in`, the same reader the editor
/// and the exporter use, so a placeholder shown inside a code span or after a
/// backslash escape is *not* a citation here either, exactly as it would not be once
/// the row is stored.
///
/// Scoped per document, and that is what makes the placeholder safe: it only has to
/// be unique within one file (see [`crate::block::SourceFootnote::label`]), so two
/// files importing together may both call their first note `srcfn-1` without either
/// claiming the other's body.
fn cited_footnotes(doc: &SourceDocument, djot: &str, epigraph: &str) -> Vec<PlannedFootnote> {
    if doc.footnotes.is_empty() {
        return Vec::new();
    }
    let labels: Vec<String> = doc.footnotes.iter().map(|f| f.label.clone()).collect();
    // Both texts, because both become a `Content` this row owns: a note cited from an
    // epigraph is as real as one cited from the prose.
    let mut here: Vec<String> = Vec::new();
    for text in [djot, epigraph] {
        for (_, label) in skribisto_model::footnote_numbering::references_in(text, &labels) {
            if !here.contains(&label) {
                here.push(label);
            }
        }
    }
    here.into_iter()
        .filter_map(|label| {
            doc.footnotes
                .iter()
                .find(|f| f.label == label)
                .map(|note| PlannedFootnote {
                    label,
                    body: note.body.clone(),
                })
        })
        .collect()
}

/// Prove every comment on a row against the prose that will actually be stored.
///
/// The offsets the scanner supplied are a *hint*, not an answer: they are exact
/// arithmetic over the block texts, but the Djot between them and the row is a real
/// parse, and the only way to know the quote still points at the same words is to look.
/// `comment_anchor::resolve` is the same three-tier matcher that re-anchors the comment
/// on every reopen, so a comment that lands here lands there.
///
/// One `djot_plain_text` per row that carries comments, and none at all for the common
/// case of a row with none, which is every Markdown import.
///
/// `moved` runs parallel to `comments`: whether each one already landed away from its
/// words before it reached this row. Each comment is reported at most once, whether it was
/// moved, its quote did not survive, or both.
fn anchor_comments(
    djot: &str,
    origin: &str,
    comments: &mut [PlannedComment],
    moved: &[bool],
    diagnostics: &mut Vec<ImportDiagnostic>,
) {
    if comments.is_empty() {
        return;
    }
    let report = |comment: &PlannedComment, diagnostics: &mut Vec<ImportDiagnostic>| {
        diagnostics.push(ImportDiagnostic::CommentUnanchored {
            path: origin.to_string(),
            quote: body_preview(&comment.body),
        });
    };
    let Ok((text, block_starts)) = skrib_format::djot_plain_text(djot) else {
        // The Djot this planner just built failed to parse, so there is no parsed text
        // to prove any quote against, not even "the row's first block", which is what a
        // heading comment falls back to below and which depends on this very parse.
        //
        // Every comment on the row becomes a `Paragraph` comment pinned to block 0 with
        // an empty quote: the tier-3 fallback `comment_anchor::resolve` already takes
        // when a paragraph's wording cannot be found. It is a real, storable anchor, and
        // the editor's own re-anchor pass places it the first time the row is opened.
        pin_to_first_block(comments);
        for (comment, _) in comments.iter().zip(moved).filter(|(_, moved)| **moved) {
            report(comment, diagnostics);
        }
        return;
    };

    for (comment, moved) in comments.iter_mut().zip(moved) {
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
                if *moved {
                    report(comment, diagnostics);
                }
            }
            Resolution::Orphan(reason) => {
                comment.orphaned = true;
                comment.orphan_reason = reason;
                report(comment, diagnostics);
            }
        }
    }
}

/// What [`rehome_comments`] decided for one document.
#[derive(Debug, Default)]
struct Homes {
    /// The document's annotations with every comment its row could not hold moved to the
    /// nearest stored paragraph, or `None` when every comment already has a home.
    annotations: Option<Vec<SourceAnnotation>>,
    /// One [`ImportDiagnostic::CommentNotCarried`] per comment no stored prose can hold.
    not_carried: Vec<ImportDiagnostic>,
}

/// Give every comment a row that stores prose to hold it.
///
/// A comment lives on the `Content` of its row's prose, so a row that stores none cannot
/// hold one: a title-only row (a Book heading directly above a Chapter heading, a chapter
/// with no text yet), or a row that is not created at all. Left there, such a comment
/// either vanished with the row or reached `apply_document_import` attached to a row with
/// no prose, which refuses the whole import. Instead it becomes a paragraph comment on the
/// nearest paragraph that is stored, flagged [`SourceAnnotation::unanchored`], so the row
/// reports it. Which way is nearest depends on what the comment was on:
///
/// * a heading or an epigraph introduces what follows it, so a comment on one goes forward
///   to the first stored paragraph after it, even under a later heading (a part's title
///   directly above its first chapter), and back only when nothing follows;
/// * anything else, most often an empty line, goes back to the paragraph before it, since
///   a comment there most often belongs to the passage it follows, and forward only when
///   nothing comes before.
///
/// Only a document with no stored prose at all leaves a comment without a home, and that is
/// reported as not imported.
fn rehome_comments(
    doc: &SourceDocument,
    drafts: &[RowDraft],
    assembled: &[AssembledProse],
    created: &[bool],
) -> Homes {
    // The blocks of every row that stores prose, and among them the prose blocks a
    // comment can point at.
    let mut held: HashSet<usize> = HashSet::new();
    let mut homes: Vec<usize> = Vec::new();
    for ((draft, prose), created) in drafts.iter().zip(assembled).zip(created) {
        if !created || prose.djot.trim().is_empty() {
            continue;
        }
        for part in &draft.parts {
            held.insert(part.block());
            if let RowPart::Prose(index) = *part
                && doc
                    .blocks
                    .get(index)
                    .is_some_and(|block| !block.plain_text().trim().is_empty())
            {
                homes.push(index);
            }
        }
    }
    homes.sort_unstable();

    if doc
        .annotations
        .iter()
        .all(|a| held.contains(&a.block_index))
    {
        return Homes::default();
    }
    let mut out = Homes::default();
    let mut annotations = Vec::with_capacity(doc.annotations.len());
    for annotation in &doc.annotations {
        if held.contains(&annotation.block_index) {
            annotations.push(annotation.clone());
            continue;
        }
        let before = homes
            .iter()
            .rev()
            .find(|home| **home < annotation.block_index);
        let after = homes.iter().find(|home| **home > annotation.block_index);
        let introduces = matches!(
            doc.blocks.get(annotation.block_index),
            Some(SourceBlock::Heading { .. } | SourceBlock::Epigraph { .. })
        );
        let before = before.map(|h| (*h, true));
        let after = after.map(|h| (*h, false));
        let home = if introduces {
            after.or(before)
        } else {
            before.or(after)
        };
        let Some((home, from_before)) = home else {
            out.not_carried.push(ImportDiagnostic::CommentNotCarried {
                path: doc.origin.clone(),
                quote: body_preview(&annotation.body),
            });
            continue;
        };
        let text = doc.blocks.get(home).map_or("", SourceBlock::plain_text);
        let chars: Vec<char> = text.chars().collect();
        // The paragraph of the home block nearest the comment: its last when the block
        // comes before the comment, its first otherwise.
        let (start, end) = if from_before {
            let start = chars.iter().rposition(|c| *c == '\n').map_or(0, |i| i + 1);
            (start, chars.len())
        } else {
            (
                0,
                chars.iter().position(|c| *c == '\n').unwrap_or(chars.len()),
            )
        };
        let ordinal = chars[..start].iter().filter(|c| **c == '\n').count();
        let mut moved = annotation.clone();
        moved.block_index = home;
        moved.kind = AnnotationKind::Paragraph;
        moved.anchor = comment_anchor::capture(text, start, end, ordinal);
        moved.unanchored = true;
        annotations.push(moved);
    }
    out.annotations = Some(annotations);
    out
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

/// A row before its prose is assembled: where it sits, what opened it, and which blocks
/// it takes.
struct RowDraft {
    indent: i64,
    create_type: CreateType,
    title: String,
    stripped_ordinal: Option<String>,
    source_uid_tag: Option<String>,
    source_digest: Option<String>,
    diagnostics: Vec<ImportDiagnostic>,
    parts: Vec<RowPart>,
}

impl RowDraft {
    fn into_row(self, doc: &SourceDocument, prose: AssembledProse) -> PlannedRow {
        let mut diagnostics = self.diagnostics;
        diagnostics.extend(prose.diagnostics);
        PlannedRow {
            indent: self.indent,
            create_type: self.create_type,
            title: self.title,
            stripped_ordinal: self.stripped_ordinal,
            djot: prose.djot,
            epigraph: prose.epigraph,
            scene_breaks: prose.scene_breaks,
            word_count: prose.word_count,
            origin: doc.origin.clone(),
            source_file_digest: doc.source_file_digest.clone(),
            included: true,
            comments: prose.comments,
            footnotes: prose.footnotes,
            source_uid_tag: self.source_uid_tag,
            source_digest: self.source_digest,
            diagnostics,
        }
    }
}

/// Decide which blocks of `doc` make which row.
fn draft_rows(
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
) -> Vec<RowDraft> {
    let mut drafts: Vec<RowDraft> = Vec::new();
    // The row currently collecting prose. A document may open with prose before
    // any heading, which becomes a row of its own rather than being silently
    // attached to the first heading that follows.
    let mut current: Option<RowDraft> = None;
    // An epigraph that belongs to the row the *next* heading will open — the
    // `EpigraphPlacement::BeforeHeading` shape, where the quotation opens the chapter
    // above its own title. Carried rather than attached on sight, because the row it
    // belongs to does not exist yet.
    let mut deferred_epigraph: Vec<RowPart> = Vec::new();
    // Whether `current` was opened by a heading and has taken nothing since. An epigraph
    // is its row's only when it sits *immediately* under the heading; one that follows a
    // paragraph of the scene is a quotation inside the scene, which is a different thing
    // and stays where the writer put it.
    let mut at_row_head = false;

    for (block_index, block) in doc.blocks.iter().enumerate() {
        match block {
            SourceBlock::Heading { level, text } => {
                if let Some(done) = current.take() {
                    drafts.push(done);
                }

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

                // An epigraph held over from before this heading is this row's, and
                // `place_epigraph` has already proven this type can hold one: it is
                // only ever deferred when the following heading's own type passed. It
                // comes first, in document order.
                let mut parts = std::mem::take(&mut deferred_epigraph);
                parts.push(RowPart::Title(block_index));
                current = Some(RowDraft {
                    indent,
                    create_type: rules.kind_for(*level),
                    title: title_text,
                    stripped_ordinal: stripped,
                    // Filled by the mark loop below, once this row is the current one.
                    source_uid_tag: None,
                    source_digest: None,
                    diagnostics,
                    parts,
                });
                at_row_head = true;
            }
            SourceBlock::Prose { .. } | SourceBlock::SceneBreak { .. } => {
                current
                    .get_or_insert_with(|| leading_row(doc, rules, base_indent))
                    .parts
                    .push(RowPart::Prose(block_index));
                at_row_head = false;
            }
            SourceBlock::Epigraph { .. } => {
                // Decided here, from the file's own shape, and never from the export
                // preset that wrote it: `EpigraphPlacement` is a choice made on the way
                // *out*, recorded nowhere in the file, and absent entirely from a
                // document this app did not produce. What the file does say is which
                // heading the quotation is touching, and that is what is read.
                let placed = place_epigraph(
                    EpigraphContext {
                        doc,
                        rules,
                        chapter_mode,
                        block_index,
                        at_row_head,
                    },
                    &mut current,
                    &mut deferred_epigraph,
                    || leading_row(doc, rules, base_indent),
                );
                if placed == EpigraphPlacementOutcome::KeptAsProse {
                    at_row_head = false;
                }
            }
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
    // Nothing is left in `deferred_epigraph` here: an epigraph is only deferred when the
    // very next block is the heading that takes it.
    if let Some(done) = current.take() {
        drafts.push(done);
    }
    drafts
}

/// What [`place_epigraph`] did with the quotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EpigraphPlacementOutcome {
    /// It became a row's `EpigraphText` — either the row above it or the one the next
    /// heading is about to open.
    Attached,
    /// It stayed in the prose stream, as the quotation it visibly is.
    KeptAsProse,
}

/// Everything `place_epigraph` needs to read about *where* the quotation sits.
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
fn place_epigraph(
    ctx: EpigraphContext<'_>,
    current: &mut Option<RowDraft>,
    deferred: &mut Vec<RowPart>,
    make_leading_row: impl Fn() -> RowDraft,
) -> EpigraphPlacementOutcome {
    // The heading immediately below, if the very next block is one. Blank blocks never
    // reach `SourceDocument::blocks`, so "the next block" really is the next thing in
    // the document.
    let below = match ctx.doc.blocks.get(ctx.block_index + 1) {
        Some(SourceBlock::Heading { level, text }) => Some((ctx.rules.kind_for(*level), text)),
        _ => None,
    };
    let below_ok = below.is_some_and(|(kind, _)| carries_epigraph(kind, ctx.chapter_mode));

    if ctx.at_row_head
        && let Some(row) = current
            .as_mut()
            .filter(|row| carries_epigraph(row.create_type, ctx.chapter_mode))
    {
        if let Some((_, below_title)) = below.filter(|_| below_ok) {
            row.diagnostics
                .push(ImportDiagnostic::EpigraphPlacementAmbiguous {
                    above: row.title.clone(),
                    below: below_title.clone(),
                });
        }
        row.parts.push(RowPart::Epigraph(ctx.block_index));
        return EpigraphPlacementOutcome::Attached;
    }

    if below_ok {
        deferred.push(RowPart::Epigraph(ctx.block_index));
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
    row.parts.push(RowPart::Prose(ctx.block_index));
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

/// How a block's comments join its row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PartRole {
    /// Its text is part of the row's Djot, so a comment keeps its words, rebased.
    Prose,
    /// Its text is not: a heading became the title, an epigraph a `Content` of its own.
    Title,
}

/// Rebase one scanned annotation onto the row its block joined.
///
/// The anchor arrives measured against the block's own text; shifting `start` by
/// where that block begins in the row is the whole of the rebasing. Everything else
/// (the quote, its context, whether it was truncated) is carried across untouched,
/// because it describes prose rather than position.
///
/// A comment whose block is not the row's prose (a heading or an epigraph) becomes a
/// `Paragraph` comment with a blank anchor: the proof then places it on the row's
/// *first* block, the same tier-3 "fall back to the block ordinal" path
/// `comment_anchor::resolve` takes for a paragraph comment whose wording cannot be
/// found. An epigraph carries no comment layer in the editor either (see
/// `OpenDoc::build`), so this is what keeps the editor's words instead of dropping them.
///
/// And a `Document` annotation, which no scanner in this crate mints any more but a
/// future one could, becomes a `Paragraph` comment on the block it names, since the
/// comment UI has no surface to open a comment on the whole document from.
fn planned_comment(
    annotation: &SourceAnnotation,
    role: PartRole,
    block_offset: usize,
) -> PlannedComment {
    let kind = match (role, annotation.kind) {
        (PartRole::Title, _) => CommentAnchorKind::Paragraph,
        (PartRole::Prose, AnnotationKind::Range) => CommentAnchorKind::Range,
        (PartRole::Prose, AnnotationKind::Paragraph | AnnotationKind::Document) => {
            CommentAnchorKind::Paragraph
        }
    };
    let mut anchor = match (role, annotation.kind) {
        (PartRole::Title, _) => Anchor::default(),
        // No text to quote: the block's position is all it has, and the proof resolves a
        // paragraph comment with no quote by its block.
        (PartRole::Prose, AnnotationKind::Document) => Anchor {
            start: block_offset,
            ..Anchor::default()
        },
        (PartRole::Prose, _) => {
            let mut anchor = annotation.anchor.clone();
            anchor.start += block_offset;
            anchor
        }
    };
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
fn leading_row(doc: &SourceDocument, rules: &LevelRules, base_indent: i64) -> RowDraft {
    RowDraft {
        indent: base_indent,
        create_type: rules.kind_for(u8::MAX),
        title: doc.effective_title().to_string(),
        stripped_ordinal: None,
        source_uid_tag: None,
        source_digest: None,
        diagnostics: Vec::new(),
        // Never an epigraph: this row exists because prose arrived before any heading,
        // and an epigraph with no heading to head is not one.
        parts: Vec::new(),
    }
}

/// Append a block's Djot to a row's, a blank line between them.
///
/// Only the whitespace Djot itself ignores is trimmed from the end. A paragraph ending in
/// a no-break space keeps it: the parser does, and the importer proved the paragraph with
/// it.
fn append_djot(buffer: &mut String, addition: &str) {
    let addition = addition.trim_end_matches(skrib_format::is_djot_whitespace);
    if addition.trim().is_empty() {
        return;
    }
    if !buffer.is_empty() {
        buffer.push_str("\n\n");
    }
    buffer.push_str(addition);
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
