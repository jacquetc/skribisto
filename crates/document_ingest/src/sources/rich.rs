// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The half `.docx` and `.odt` share: styled runs in, [`SourceBlock`]s out.
//!
//! Both formats are a zip of XML describing the same thing — a sequence of
//! paragraphs, each a list of runs each carrying a bit of character formatting,
//! plus tables, plus comments anchored to run ranges. Only the spelling differs.
//! So the spelling is all their scanners do; everything downstream of "here are the
//! paragraphs" happens once, here, and the same input produces the same prose
//! whichever container it arrived in.
//!
//! ## Build HTML, then convert — never hand-emit Djot
//!
//! `skrib_format::html_to_djot_and_text` goes through `text-document`'s document
//! model, and its own doc says why that matters: **CommonMark and Djot swap their
//! emphasis delimiters**, so any hand-written emitter has to re-derive that, plus
//! Djot escaping, plus list and table syntax — twice, once per format. Emitting
//! `<p><strong>…</strong><em>…</em></p>` from styled runs is trivial and
//! unambiguous, and the conversion that follows is the one the Markdown scanner
//! already trusts.
//!
//! It also neutralises Word's habit of splitting one bold word across three runs
//! with identical properties: parsing to a model merges adjacent identical-format
//! runs, so `*b**o**ld*` cannot come out the far end. Runs are still coalesced by
//! property set before emitting, because emitting six tags where one will do is
//! wasteful and makes the intermediate unreadable when something needs debugging.
//!
//! ## What the conversion carries, measured rather than assumed
//!
//! Verified against the real converter: `<strong>`→`*x*`, `<em>`→`_x_`,
//! `<u>`→`{+x+}`, `<s>`/`<del>`→`{-x-}`, `<code>`→`` `x` ``, `<blockquote>`→`> x`,
//! `<ul>`/`<ol>`→Djot lists at any nesting depth, `<table>`→a Djot pipe table, and
//! HTML entities decode to their characters.
//!
//! Three things it does **not** carry, each handled here rather than left to
//! surprise someone:
//!
//! * **`<sup>` and `<sub>` are dropped to plain text.** The characters survive, the
//!   raising does not. No diagnostic: this is character *styling*, like colour,
//!   font and size, none of which Skribisto's model carries either — saying so once
//!   here is more honest than a warning on every document that contains a footnote
//!   marker.
//! * **`<br>` vanishes entirely**, gluing the words either side of it together.
//!   A soft line break is therefore split into a paragraph of its own before the
//!   HTML is built, which is what every other manuscript importer does with one and
//!   is the only option that does not silently corrupt the sentence.
//! * **A truly empty `<p>` produces no block at all**, while a whitespace-only one
//!   does. Blank paragraphs are dropped before emitting so the two cannot disagree,
//!   and so the offset arithmetic below stays exact.
//!
//! ## Where a comment ends up, and why a table is flushed alone
//!
//! An annotation is captured against **one block's own plain text**, and the
//! planner rebases it into the row that block lands in. That rebasing is arithmetic
//! — every block contributes exactly one line to the run's plain text, joined by one
//! `\n` — and it is *checked* rather than trusted: [`assemble`] verifies the slice it
//! computed really is the block's text before using the offset, and degrades the
//! annotation to a whole-row [`AnnotationKind::Document`] comment when it is not.
//!
//! A table is the one construct that breaks the arithmetic: `text-document` reports
//! block positions for a table that do not agree with its own plain text (measured:
//! a one-cell table's following paragraph reports position 4 in a 7-character
//! string where it actually starts at 2). So a table is flushed as a prose block of
//! its own, and a comment inside one becomes a `Document` comment rather than a
//! confidently misplaced range.

use anyhow::Result;
use skribisto_model::comment_anchor::{self, Anchor};
use skribisto_model::scene_break;

use crate::block::{
    AnnotationKind, SourceAnnotation, SourceAnnotationReply, SourceBlock, SourceDocument,
    SourceRowMark,
};

/// Character formatting a container format can express and Djot can carry.
///
/// Deliberately closed and small: these five are exactly what survives the
/// conversion. A sixth field would be a promise this layer cannot keep.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct RunStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub code: bool,
}

/// The object-replacement character an inline image occupies in plain text.
///
/// Not this crate's invention: it is what the conversion emits for an `<img>`, and
/// what `comment_anchor::for_display` already knows to substitute a picture glyph
/// for. Counting it as one character here is what keeps an annotation that follows
/// an image on the right words.
pub const IMAGE_PLACEHOLDER: char = '\u{FFFC}';

/// One run of text and how it is formatted.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Run {
    pub text: String,
    pub style: RunStyle,
    /// The hyperlink this run sits inside, if any. Carried rather than flattened:
    /// a URL a writer put in their manuscript is content, and Djot has a link.
    pub link: Option<String>,
    /// When set, this run **is** an image reference — `text` is its alt text and
    /// this is the path inside the container. It occupies exactly one character of
    /// plain text, [`IMAGE_PLACEHOLDER`].
    pub image: Option<String>,
}

impl Run {
    pub fn plain(text: impl Into<String>) -> Self {
        Run {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn styled(text: impl Into<String>, style: RunStyle) -> Self {
        Run {
            text: text.into(),
            style,
            ..Default::default()
        }
    }

    pub fn linked(text: impl Into<String>, style: RunStyle, url: impl Into<String>) -> Self {
        Run {
            text: text.into(),
            style,
            link: Some(url.into()),
            image: None,
        }
    }

    pub fn image(alt: impl Into<String>, src: impl Into<String>) -> Self {
        Run {
            text: alt.into(),
            style: RunStyle::default(),
            link: None,
            image: Some(src.into()),
        }
    }

    fn plain_push(&self, out: &mut String) {
        if self.image.is_some() {
            out.push(IMAGE_PLACEHOLDER);
        } else {
            out.push_str(&self.text);
        }
    }
}

/// What kind of paragraph this is, in the vocabulary both containers share.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParagraphKind {
    /// A heading at the depth the source expressed — `w:outlineLvl` in OOXML,
    /// `text:outline-level` in ODF. Normalising a gap is the pipeline's job.
    Heading {
        level: u8,
    },
    Body,
    /// A blockquote's paragraph, recognised from the `Quote` **named paragraph style**
    /// both this workspace's writers apply (`export_docx_uc::QUOTE_STYLE_ID`,
    /// `odt_render::named_styles_xml`) and Word ships as a built-in.
    ///
    /// Never inferred from indentation. An indent is a measurement — verse, a Tab
    /// somebody pressed, and a quotation are indistinguishable by it — so reading one as
    /// a quotation would invent structure from layout, which this crate's block model
    /// refuses by rule. A style *name* is the document stating what the paragraph is, in
    /// the same way `text:outline-level` states a heading depth.
    Quote,
    /// A blockquote the source named as an **epigraph** — the quotation set at the head
    /// of a part or a chapter, from the `Epigraph`/`EpigraphAttribution` named styles.
    ///
    /// Separate from [`ParagraphKind::Quote`] because it is not the row's prose at all:
    /// an epigraph is quoted matter belonging to `ContentRole::EpigraphText`, its words
    /// are not the manuscript's, and it must not be counted or concatenated into a
    /// scene. The pipeline lifts it out of the prose stream entirely — see
    /// [`crate::block::SourceBlock::Epigraph`].
    Epigraph,
    /// `depth` is 0-based, so a top-level bullet is 0.
    ListItem {
        ordered: bool,
        depth: u8,
    },
}

/// What a paragraph style says the paragraph *is*, when it says anything at all.
///
/// The one table both containers consult, so `.docx` and `.odt` cannot come to
/// different conclusions about the same manuscript — the same reason every other
/// post-"here are the paragraphs" decision lives in this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyledAs {
    Epigraph,
    Quote,
}

/// Read a paragraph style's stable identifier as a claim about the paragraph.
///
/// **What "stable identifier" means differs by format, and the difference is the point.**
/// OOXML localizes a style's *name* ("Quote" is "Citation" in French Word) but not its
/// `w:styleId`, so the DOCX scanner passes ids here — the same reasoning its heading code
/// already states for `Heading1`. ODF does not localize at all: `style:name` is the stored
/// English name whatever the UI shows, so the ODT scanner passes names. Neither passes
/// something a locale can move.
///
/// Two families are recognised:
///
/// * **Ours.** `Epigraph` / `EpigraphAttribution` / `Quote` — what `text-document`'s DOCX and
///   ODT writers apply. These are what closes the round trip, and they survive an editor's
///   save (measured against a real LibreOffice save, unlike the `skrb:uid` attribute that
///   `round_trip`'s module doc records being deleted).
/// * **The host applications' own.** LibreOffice's `Quotations` and Word's `IntenseQuote` /
///   `BlockText` are the styles a writer gets by pressing the block-quote button in the
///   application they actually wrote the manuscript in. Reading them is the same move as
///   reading `style:default-outline-level` off a novel template's chapter style: not name
///   *guessing*, but the document stating what a paragraph is in the vocabulary its own
///   producer uses.
///
/// Matching folds case and drops separators, so `Epigraph Attribution`, `epigraph-attribution`
/// and ODF's own `Epigraph_20_Attribution` escape all answer alike. `_20_` (ODF's escape for a
/// space) is removed *before* the fold, or it would survive it as a literal `20`.
pub fn styled_as(identifier: &str) -> Option<StyledAs> {
    let folded: String = identifier
        .replace("_20_", "")
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    match folded.as_str() {
        "epigraph" | "epigraphattribution" => Some(StyledAs::Epigraph),
        "quote" | "quotations" | "intensequote" | "blocktext" => Some(StyledAs::Quote),
        _ => None,
    }
}

/// The paragraph kind a resolved style verdict produces.
///
/// A tiny helper rather than two `match`es, because the two scanners reaching different
/// answers here is exactly the class of drift this module exists to prevent.
pub fn kind_for_style(styled: StyledAs) -> ParagraphKind {
    match styled {
        StyledAs::Epigraph => ParagraphKind::Epigraph,
        StyledAs::Quote => ParagraphKind::Quote,
    }
}

/// One block of a rich document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RichBlock {
    /// Contributes exactly one line to the plain text of the run it joins.
    Paragraph { kind: ParagraphKind, runs: Vec<Run> },
    /// Rows of cells of runs. Flushed on its own — see the module note.
    Table { rows: Vec<Vec<Vec<Run>>> },
}

impl RichBlock {
    pub fn body(runs: Vec<Run>) -> Self {
        RichBlock::Paragraph {
            kind: ParagraphKind::Body,
            runs,
        }
    }

    /// The text of this block with no formatting — the coordinate space an
    /// annotation on it is measured in.
    pub fn plain_text(&self) -> String {
        let mut out = String::new();
        match self {
            RichBlock::Paragraph { runs, .. } => {
                for run in runs {
                    run.plain_push(&mut out);
                }
            }
            RichBlock::Table { rows } => {
                let mut first = true;
                for cell in rows.iter().flat_map(|row| row.iter()) {
                    if !first {
                        out.push('\n');
                    }
                    first = false;
                    for run in cell {
                        run.plain_push(&mut out);
                    }
                }
            }
        }
        out
    }

    fn is_blank(&self) -> bool {
        self.plain_text().trim().is_empty()
    }
}

/// A comment as its container expressed it, before it knows anything about rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RichAnnotation {
    /// Index into [`RichDocument::blocks`].
    pub block_index: usize,
    /// Character offset within that block's [`RichBlock::plain_text`].
    pub start: usize,
    /// `0` means the comment had no range — it belongs to the paragraph.
    pub length: usize,
    /// The uid this comment carried in the source file, when the file is one
    /// Skribisto itself exported (the DOCX/ODT writers' own `skrb:uid` attribute —
    /// see [`crate::block::SourceAnnotation::uid`]). `None` for a comment an editor
    /// typed straight into Word or LibreOffice.
    pub uid: Option<uuid::Uuid>,
    /// See [`crate::block::SourceAnnotation::uid_tag`] — the identity carried by a
    /// `skrb_c…` bookmark pair, which is what actually survives an editor's save.
    pub uid_tag: Option<String>,
    pub author: String,
    /// See [`crate::block::SourceAnnotation::author_initials`] — empty means
    /// "none", never carried at all on ODT.
    pub author_initials: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// The comment's own text, one `Vec` of [`Run`]s per paragraph — **not yet
    /// Djot**. An editor's remark can carry the same bold/italic/underline/
    /// strikethrough a manuscript paragraph can, and it is converted the same
    /// way: [`assemble`] runs every annotation (and every reply) through
    /// [`body_to_djot`] in the one pass that already owns the "never hand-emit
    /// Djot" pipeline, so a scanner never has to carry a second copy of it just
    /// to stringify a comment.
    pub paragraphs: Vec<Vec<Run>>,
    pub resolved: bool,
    pub replies: Vec<RichReply>,
}

/// One reply, before its body is converted — see [`RichAnnotation::paragraphs`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RichReply {
    /// See [`RichAnnotation::uid`] — the same recognition mechanism, for one reply
    /// rather than the thread's opening comment.
    pub uid: Option<uuid::Uuid>,
    pub author: String,
    /// See [`RichAnnotation::author_initials`].
    pub author_initials: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub paragraphs: Vec<Vec<Run>>,
}

/// One round-trip row mark, before its block index is rebased onto the neutral block model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RichRowMark {
    /// Index into [`RichDocument::blocks`].
    pub block_index: usize,
    pub uid_tag: String,
    pub digest: String,
}

/// A round-trip comment mark whose bookmark range has been opened and not yet closed.
///
/// Shared by both container scanners rather than written twice: ODF and OOXML disagree about
/// how a bookmark is spelled — ODF names both halves, OOXML names only the start and closes by
/// numeric id — but what a mark *means* once opened is identical, and two copies of that would
/// eventually disagree about which annotation a mark belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenMark {
    pub uid_tag: String,
    pub block: usize,
    pub start: usize,
}

/// A closed comment mark: which characters it bracketed, and whose identity it carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentMark {
    pub uid_tag: String,
    pub block: usize,
    pub start: usize,
    pub length: usize,
}

/// Give each annotation the identity of the round-trip mark bracketing the same text.
///
/// Matched on `(block, start)` — not by name, and not by document order.
///
/// **Not by name** because there is no shared name to match on: LibreOffice rewrites
/// `office:name` to its own `__Annotation__…` on save, and OOXML's comment ids are reassigned
/// freely. That an annotation's own identity does not survive is the entire reason a bookmark
/// carries it instead.
///
/// **Not by order** because an editor adds and deletes comments wherever they like, so the
/// third annotation in the returning file need not be the third mark.
///
/// A mark with no annotation at its position is simply unused — the editor deleted the comment
/// and the bookmark outlived it, which both applications allow. An annotation with no mark
/// keeps `uid_tag: None` and imports as a new comment, which is exactly right for one the
/// editor wrote themselves.
pub fn attach_comment_marks(annotations: &mut [RichAnnotation], marks: &[CommentMark]) {
    for mark in marks {
        let hit = annotations
            .iter_mut()
            .find(|a| a.block_index == mark.block && a.start == mark.start && a.uid_tag.is_none());
        if let Some(a) = hit {
            a.uid_tag = Some(mark.uid_tag.clone());
            // The mark's extent is what the comment covered when it was written, and a
            // bookmark is maintained by the editor's own application as text moves around it.
            // Adopted only when the annotation has no range of its own, so a genuine
            // paragraph comment — which means "the whole paragraph" and stores zero — stays
            // one instead of silently acquiring a range.
            if a.length == 0 && mark.length > 0 {
                a.length = mark.length;
            }
        }
    }
}

/// What a container scanner produces, before any Skribisto vocabulary is applied.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RichDocument {
    pub blocks: Vec<RichBlock>,
    pub annotations: Vec<RichAnnotation>,
    /// See [`crate::block::SourceRowMark`]. Empty for every file this app did not write.
    pub row_marks: Vec<RichRowMark>,
}

/// Turn a rich document into the neutral block model, carrying its comments.
///
/// `out` is filled rather than returned so a scanner can put its metadata and its
/// own diagnostics on the document first — an annotation the assembler cannot place
/// reports itself through `out.diagnostics`, and doing that to a half-built document
/// would lose whatever the scanner had already said.
pub fn assemble(doc: &RichDocument, out: &mut SourceDocument) -> Result<()> {
    // Which `SourceBlock` each rich block ended up in, and where in that block's
    // plain text it starts. `None` for a rich block that produced nothing.
    let mut placement: Vec<Option<Placement>> = vec![None; doc.blocks.len()];

    // The prose run being accumulated: (rich index, html, plain text).
    let mut pending: Vec<(usize, String, String)> = Vec::new();
    // The epigraph run being accumulated, in the same shape. Separate because an
    // epigraph becomes a block of its own; the two never hold members at once, since
    // each is flushed the moment the other starts.
    let mut pending_epigraph: Vec<(usize, String, String)> = Vec::new();

    for (index, block) in doc.blocks.iter().enumerate() {
        if block.is_blank() {
            continue;
        }
        let boundary = classify(block);
        // An epigraph run ends at the first block that is not part of it, and the prose
        // run ends where an epigraph starts. Flushing both at the boundary is what keeps
        // `out.blocks` in document order — which is the whole basis on which `plan`
        // decides, from adjacency alone, which heading an epigraph belongs to.
        if !matches!(boundary, Boundary::Epigraph) {
            flush_epigraph(&mut pending_epigraph, out, &mut placement)?;
        }
        match boundary {
            Boundary::SceneBreak(tier) => {
                flush(&mut pending, out, &mut placement)?;
                placement[index] = Some(Placement {
                    source_block: out.blocks.len(),
                    offset: 0,
                    exact: false,
                });
                out.blocks.push(SourceBlock::SceneBreak { tier });
            }
            Boundary::Heading { level, text } => {
                flush(&mut pending, out, &mut placement)?;
                placement[index] = Some(Placement {
                    source_block: out.blocks.len(),
                    offset: 0,
                    exact: false,
                });
                out.blocks.push(SourceBlock::Heading { level, text });
            }
            Boundary::Table => {
                // Alone, because the block positions of a document containing a
                // table cannot be trusted — see the module note.
                flush(&mut pending, out, &mut placement)?;
                pending.push((index, html_of(block), block.plain_text()));
                flush(&mut pending, out, &mut placement)?;
                if let Some(p) = placement[index].as_mut() {
                    p.exact = false;
                }
            }
            Boundary::Epigraph => {
                // The prose above it is closed first, so the epigraph block lands
                // between the two runs exactly where the document put it.
                flush(&mut pending, out, &mut placement)?;
                pending_epigraph.push((index, html_of(block), block.plain_text()));
            }
            Boundary::Prose => {
                pending.push((index, html_of(block), block.plain_text()));
            }
        }
    }
    flush_epigraph(&mut pending_epigraph, out, &mut placement)?;
    flush(&mut pending, out, &mut placement)?;

    // Row marks, rebased onto the neutral block model the same way an annotation is.
    //
    // A mark whose rich block produced nothing is **dropped**, not carried to a neighbouring
    // block. It names a row by pointing at a passage, and pointing it at a different passage
    // than the one it was written into would be worse than not having it: the fallback when a
    // mark is missing is matching by type and title, which is a guess the writer can see and
    // correct, while a mark is trusted outright.
    for mark in &doc.row_marks {
        if let Some(place) = placement.get(mark.block_index).copied().flatten() {
            out.row_marks.push(SourceRowMark {
                block_index: place.source_block,
                uid_tag: mark.uid_tag.clone(),
                digest: mark.digest.clone(),
            });
        }
    }

    for annotation in &doc.annotations {
        // Converted once, here — the one place in either scanner that turns a
        // comment's own paragraphs into Djot, on the same terms manuscript prose
        // gets converted a few lines up. Doing this per-annotation rather than
        // inside each scanner's own recursive walk is what keeps `docx.rs` and
        // `odt.rs` from needing their own copy of `html_to_djot_and_text`'s
        // error handling — there is exactly one call site to get right.
        let body = body_to_djot(&annotation.paragraphs)?;
        let mut replies = Vec::with_capacity(annotation.replies.len());
        for reply in &annotation.replies {
            replies.push(SourceAnnotationReply {
                uid: reply.uid,
                author: reply.author.clone(),
                author_initials: reply.author_initials.clone(),
                created: reply.created,
                body: body_to_djot(without_reply_citation(&reply.paragraphs))?,
            });
        }

        match placement.get(annotation.block_index).copied().flatten() {
            Some(place) => {
                let block_text = out.blocks[place.source_block].plain_text().to_string();
                let member_len = doc.blocks[annotation.block_index]
                    .plain_text()
                    .chars()
                    .count();
                out.annotations.push(place_annotation(
                    annotation,
                    place,
                    &block_text,
                    member_len,
                    body,
                    replies,
                ));
            }
            // The block it pointed at produced nothing at all — a comment on a
            // paragraph that was blank, or one the scanner indexed past the end.
            // The comment is still the writer's, so it is carried as a comment on
            // the document rather than dropped, and it says so.
            None => {
                // The diagnostic's quote is plain text, not Djot — a message
                // naming "the passage beginning '*Is this*'" would show the
                // writer their editor's own markup rather than their editor's
                // words. `preview`'s own contract is a plain string it collapses
                // whitespace in, so the Djot is converted back down for it here
                // rather than changing what `preview` accepts. Falls back to the
                // raw Djot on the vanishingly rare parse failure — a diagnostic
                // with a slightly rougher quote beats one silently skipped.
                let plain = skrib_format::djot_plain_text(&body)
                    .map(|(text, _)| text)
                    .unwrap_or_else(|_| body.clone());
                out.diagnostics
                    .push(crate::diagnostics::ImportDiagnostic::CommentUnanchored {
                        path: out.origin.clone(),
                        quote: preview(&plain),
                    });
                out.annotations.push(SourceAnnotation {
                    block_index: out.blocks.len().saturating_sub(1),
                    kind: AnnotationKind::Document,
                    anchor: Anchor::default(),
                    uid: annotation.uid,
                    uid_tag: annotation.uid_tag.clone(),
                    author: annotation.author.clone(),
                    author_initials: annotation.author_initials.clone(),
                    created: annotation.created,
                    body,
                    resolved: annotation.resolved,
                    replies,
                });
            }
        }
    }
    Ok(())
}

/// Where one rich block ended up.
#[derive(Debug, Clone, Copy)]
struct Placement {
    source_block: usize,
    /// Character offset of this rich block's text within that `SourceBlock`'s text.
    offset: usize,
    /// Whether the offset was *verified* against the converted text. When false the
    /// block is still identified, but a range annotation on it degrades to a
    /// whole-row comment rather than pointing somewhere unproven.
    exact: bool,
}

enum Boundary {
    Heading { level: u8, text: String },
    SceneBreak(scene_break::SceneBreakTier),
    Table,
    Epigraph,
    Prose,
}

/// Decide what one block is — **vocabulary first**, exactly as the Markdown
/// scanner does.
///
/// A paragraph reading `* * *` is a scene break whether Word styled it Normal,
/// Heading 1 or centred: Skribisto's break vocabulary is content-based, so asking
/// the text before asking the style is what keeps every format answering the same
/// way. It is also what stops a re-imported Skribisto export from turning its own
/// major break into a spurious chapter.
fn classify(block: &RichBlock) -> Boundary {
    let text = block.plain_text();
    if let Some(tier) = scene_break::tier_of_plain_line(text.trim()) {
        return Boundary::SceneBreak(tier);
    }
    match block {
        RichBlock::Table { .. } => Boundary::Table,
        RichBlock::Paragraph {
            kind: ParagraphKind::Heading { level },
            ..
        } => Boundary::Heading {
            level: *level,
            text: text.trim().to_string(),
        },
        RichBlock::Paragraph {
            kind: ParagraphKind::Epigraph,
            ..
        } => Boundary::Epigraph,
        RichBlock::Paragraph { .. } => Boundary::Prose,
    }
}

/// Convert the accumulated run into one prose block and record where each of its
/// members landed inside it.
fn flush(
    pending: &mut Vec<(usize, String, String)>,
    out: &mut SourceDocument,
    placement: &mut [Option<Placement>],
) -> Result<()> {
    flush_run(pending, out, placement, RunKind::Prose)
}

/// The same, for a run of epigraph-styled paragraphs — one `<blockquote>` around the
/// whole run, landing in a [`SourceBlock::Epigraph`].
fn flush_epigraph(
    pending: &mut Vec<(usize, String, String)>,
    out: &mut SourceDocument,
    placement: &mut [Option<Placement>],
) -> Result<()> {
    flush_run(pending, out, placement, RunKind::Epigraph)
}

/// Which of the two block kinds a flushed run becomes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RunKind {
    Prose,
    Epigraph,
}

/// Convert the accumulated run into one block and record where each of its members
/// landed inside it.
///
/// One function for both kinds because the offset arithmetic below is the part that
/// must not diverge: it is what a comment's position is rebased through, and two
/// copies of it would eventually disagree about where a paragraph starts.
fn flush_run(
    pending: &mut Vec<(usize, String, String)>,
    out: &mut SourceDocument,
    placement: &mut [Option<Placement>],
    kind: RunKind,
) -> Result<()> {
    if pending.is_empty() {
        return Ok(());
    }
    let members = std::mem::take(pending);
    let inner: String = members.iter().map(|(_, h, _)| h.as_str()).collect();
    // One quotation around the whole epigraph run, not one per paragraph — see
    // `html_of`'s epigraph arm for why several would come back as several epigraphs.
    let html = match kind {
        RunKind::Prose => inner,
        RunKind::Epigraph => format!("<blockquote>{inner}</blockquote>"),
    };
    let (djot, text) = skrib_format::html_to_djot_and_text(&html)?;
    if djot.trim().is_empty() {
        return Ok(());
    }
    let source_block = out.blocks.len();
    out.blocks.push(match kind {
        RunKind::Prose => SourceBlock::Prose { djot, text },
        RunKind::Epigraph => SourceBlock::Epigraph { djot, text },
    });
    let (SourceBlock::Prose { text, .. } | SourceBlock::Epigraph { text, .. }) =
        &out.blocks[source_block]
    else {
        unreachable!("just pushed a prose or epigraph block");
    };

    // Every block contributes one line, joined by one `\n` — but check it, because
    // a wrong offset here is a comment silently attached to the wrong sentence.
    let converted: Vec<char> = text.chars().collect();
    let mut offset = 0usize;
    for (index, _, plain) in &members {
        let len = plain.chars().count();
        let exact = converted
            .get(offset..offset + len)
            .is_some_and(|slice| slice.iter().collect::<String>() == *plain);
        placement[*index] = Some(Placement {
            source_block,
            offset,
            exact,
        });
        offset += len + 1;
    }
    Ok(())
}

/// Capture the annotation's selector against the block it landed in.
///
/// `block_text` is that [`SourceBlock`]'s plain text and `member_len` the length of
/// the one paragraph the comment was written on — needed because a paragraph
/// comment's extent is the whole paragraph, not the zero-length range the container
/// stored for it.
///
/// The capture itself goes through `comment_anchor::capture`, never a local
/// reimplementation: a selector built by one set of rules and resolved by another is
/// exactly the drift that module was moved down to prevent.
fn place_annotation(
    annotation: &RichAnnotation,
    place: Placement,
    block_text: &str,
    member_len: usize,
    body: String,
    replies: Vec<SourceAnnotationReply>,
) -> SourceAnnotation {
    let (kind, anchor) = if !place.exact {
        (AnnotationKind::Document, Anchor::default())
    } else {
        // A comment with no range belongs to the paragraph its marker sat in, and
        // covers **all** of it — not the empty span at the marker. Quoting from the
        // marker outwards would capture nothing at all when the marker sits at the
        // end of the paragraph, which is exactly where Word and LibreOffice put it,
        // and a zero-length quote can never be re-found.
        let (kind, start, end) = if annotation.length == 0 {
            (
                AnnotationKind::Paragraph,
                place.offset,
                place.offset + member_len,
            )
        } else {
            let start = place.offset + annotation.start;
            (AnnotationKind::Range, start, start + annotation.length)
        };
        let ordinal = block_text
            .chars()
            .take(start)
            .filter(|c| *c == '\n')
            .count();
        (
            kind,
            comment_anchor::capture(block_text, start, end, ordinal),
        )
    };

    SourceAnnotation {
        block_index: place.source_block,
        kind,
        anchor,
        uid: annotation.uid,
        uid_tag: annotation.uid_tag.clone(),
        author: annotation.author.clone(),
        author_initials: annotation.author_initials.clone(),
        created: annotation.created,
        body,
        resolved: annotation.resolved,
        replies,
    }
}

/// Convert a comment or reply's own paragraphs — an editor's remark, carrying
/// whatever emphasis they gave it — into the Djot [`common::entities::Comment`]
/// (via `skribisto_model`'s crate boundary, [`SourceAnnotation::body`]) and
/// [`SourceAnnotationReply::body`] now store.
///
/// Goes through the same HTML→Djot pipeline manuscript prose takes ([`flush`]),
/// not a comment-only emitter: see the module doc's "never hand-emit Djot" for
/// why a second emitter would have to re-derive Djot's swapped emphasis
/// delimiters and its escaping rules on its own, and would eventually disagree
/// with the one manuscript prose already trusts.
///
/// A paragraph carrying no text and no image contributes nothing — the same
/// rule [`RichBlock::is_blank`] applies to manuscript prose, so a remark that is
/// entirely whitespace (an editor who pressed Enter twice and typed nothing)
/// does not turn into a bare, meaningless Djot paragraph marker. An annotation
/// with no non-blank paragraph at all converts to the empty string, exactly as
/// `flush` produces no block for an all-blank prose run.
/// A reply's own paragraphs, with LibreOffice's citation block removed if it wrote one.
///
/// LibreOffice's **Reply** button does not merely thread a reply — it prepends a paragraph
/// quoting what is being replied to, in the shape
///
/// ```text
/// Répondre à  (10/08/2026, 09:27): "…"
/// ```
///
/// Left alone, that paragraph is imported as part of the editor's words, and it *accumulates*:
/// every export-reply-import cycle prepends another one, and a thread that has been round-tripped
/// three times reads as three nested quotations before its actual content. Worse, the same reply
/// coming back a second time no longer matches what the project stored, so a re-import that
/// should have been a no-op reports it as edited.
///
/// # Detected by shape, not by wording
///
/// The verb is localised — "Répondre à", "Reply to", "Antwort an" — so matching the string would
/// work in whatever language this was written against and silently stop working in the others.
/// What is stable is the punctuation the timestamp and quote are wrapped in, and that is what is
/// tested: a parenthesised **timestamp**, followed by `): "` and a closing quote at the end.
///
/// Four guards keep it from eating a real reply. It must be the **first** paragraph, there must
/// be **another paragraph after it** (a reply that is *only* a citation has had its content
/// deleted, and dropping it would leave an empty reply rather than an odd one), the shape has to
/// match in full, and the parenthesised group has to hold a **date and a time**, not merely some
/// digits. When in doubt the paragraph is kept: an editor's words showing up with an odd prefix
/// is a blemish, and an editor's words disappearing is data loss.
///
/// That last guard is not hypothetical tightening. "Digits and a separator" also describes an
/// ordinary editorial note — `My note from our call (10/14): "cut this scene entirely"` — which
/// this ate, silently, on both formats and whatever application wrote them. Requiring a clock
/// time as well as a date costs nothing (every locale LibreOffice writes this in stamps both)
/// and takes the false positive from plausible to contrived.
fn without_reply_citation(paragraphs: &[Vec<Run>]) -> &[Vec<Run>] {
    if paragraphs.len() < 2 {
        return paragraphs;
    }
    let first: String = paragraphs[0].iter().map(|r| r.text.as_str()).collect();
    if is_reply_citation(first.trim()) {
        &paragraphs[1..]
    } else {
        paragraphs
    }
}

/// Whether `text` is a word processor's own "replying to X" citation line.
fn is_reply_citation(text: &str) -> bool {
    // Ends with a closing quote — straight or typographic, since which one appears depends on
    // the editor's autocorrect settings rather than on anything structural.
    if !text.ends_with(['"', '\u{201D}', '\u{00BB}']) {
        return false;
    }
    // The timestamp/quote boundary: `): ` followed by an opening quote.
    let boundary = ["): \"", "): \u{201C}", "): \u{00AB}"]
        .iter()
        .find_map(|b| text.rfind(b));
    let Some(boundary) = boundary else {
        return false;
    };
    // A parenthesised group before it holding **nothing but a timestamp**.
    //
    // Asking only that the group *contain* a date-ish and a time-ish pair is not enough, and
    // that mistake has now been made twice: `(9:15-10:30)`, `(John 3:16, Matt 5:3-12)` and
    // `(ch. 3:16-5:22)` all satisfy it, and all three are things an editor writes. What
    // actually distinguishes a machine-written timestamp is that there is nothing else in
    // there — no words, no prose. So the whole group must be digits, separators and space,
    // with at most a trailing meridiem, and must hold a date *and* a time separated by a
    // comma, which is the shape every locale writes this in.
    let Some(open) = text[..boundary].rfind('(') else {
        return false;
    };
    is_timestamp(&text[open + 1..boundary])
}

/// Whether `group` is a bare date-and-time stamp and nothing else.
///
/// ⚠ Deliberately ASCII-numeric. A locale that spells the date in its own script — Japanese
/// `2026年8月10日`, Arabic-Indic digits — fails this and the citation is *kept*, which is the
/// documented default: an editor's words showing up under an odd prefix is a blemish, and an
/// editor's words disappearing is data loss.
fn is_timestamp(group: &str) -> bool {
    let group = group.trim();
    // A meridiem is the one word allowed, and only at the end.
    let core = ["AM", "PM", "am", "pm", "A.M.", "P.M."]
        .iter()
        .find_map(|m| group.strip_suffix(m))
        .unwrap_or(group)
        .trim();

    // Date and time, in that order, separated by the comma every locale puts between them.
    let Some((date, time)) = core.split_once(',') else {
        return false;
    };
    // Nothing but digits and the separators a stamp is built from — this is what rejects
    // `John 3:16, Matt 5:3-12`, where both halves are otherwise convincing.
    let numeric = |s: &str| {
        let s = s.trim();
        !s.is_empty()
            && s.chars()
                .all(|c| c.is_ascii_digit() || matches!(c, '/' | '-' | '.' | ':' | ' '))
    };
    numeric(date)
        && numeric(time)
        // Two components each, at least: a bare `(2026, 9)` is not a timestamp.
        && separates_digits(date, &['/', '-', '.'])
        && separates_digits(time, &[':', '.'])
        && core.chars().filter(char::is_ascii_digit).count() >= 6
}

/// Whether any of `seps` appears in `text` with a digit on both sides.
///
/// A bare `contains` would accept the hyphen in an em-dashed aside or the full stop ending the
/// sentence before the parenthesis; what a timestamp actually looks like is digits *around* its
/// separators.
fn separates_digits(text: &str, seps: &[char]) -> bool {
    let chars: Vec<char> = text.chars().collect();
    chars
        .windows(3)
        .any(|w| w[0].is_ascii_digit() && seps.contains(&w[1]) && w[2].is_ascii_digit())
}

fn body_to_djot(paragraphs: &[Vec<Run>]) -> Result<String> {
    let mut html = String::new();
    for runs in paragraphs {
        if runs
            .iter()
            .all(|r| r.image.is_none() && r.text.trim().is_empty())
        {
            continue;
        }
        html.push_str("<p>");
        html.push_str(&runs_html(runs));
        html.push_str("</p>");
    }
    if html.is_empty() {
        return Ok(String::new());
    }
    Ok(skrib_format::html_to_djot_and_text(&html)?.0)
}

/// One block's HTML.
fn html_of(block: &RichBlock) -> String {
    match block {
        RichBlock::Paragraph { kind, runs } => {
            let inner = runs_html(runs);
            match kind {
                ParagraphKind::Heading { level } => {
                    let level = (*level).clamp(1, 6);
                    format!("<h{level}>{inner}</h{level}>")
                }
                // An epigraph's own paragraphs are bare here: `flush_epigraph` wraps the
                // whole run in **one** `<blockquote>`, so a two-paragraph quotation and its
                // attribution line come back as one epigraph rather than three. Wrapping each
                // separately (as `Quote` must, since consecutive quoted paragraphs in a scene
                // are not necessarily one quotation) would export as three epigraphs on the
                // next trip out — `render::mark_epigraph` marks every blockquote it finds.
                ParagraphKind::Body | ParagraphKind::Epigraph => format!("<p>{inner}</p>"),
                ParagraphKind::Quote => format!("<blockquote><p>{inner}</p></blockquote>"),
                ParagraphKind::ListItem { ordered, depth } => {
                    // One `<li>` per block, nested by repeating the container.
                    // `text-document` reads any depth and re-emits it as an
                    // indented Djot list, one plain-text line per item.
                    let tag = if *ordered { "ol" } else { "ul" };
                    let open = format!("<{tag}>").repeat(*depth as usize + 1);
                    let close = format!("</{tag}>").repeat(*depth as usize + 1);
                    format!("{open}<li>{inner}</li>{close}")
                }
            }
        }
        RichBlock::Table { rows } => {
            let mut html = String::from("<table>");
            for row in rows {
                html.push_str("<tr>");
                for cell in row {
                    html.push_str("<td>");
                    html.push_str(&runs_html(cell));
                    html.push_str("</td>");
                }
                html.push_str("</tr>");
            }
            html.push_str("</table>");
            html
        }
    }
}

/// Coalesced, escaped, tagged inline HTML for a paragraph's runs.
fn runs_html(runs: &[Run]) -> String {
    let mut html = String::new();
    for run in coalesce(runs) {
        if let Some(src) = &run.image {
            html.push_str("<img src=\"");
            push_attr(&mut html, src);
            html.push_str("\" alt=\"");
            push_attr(&mut html, &run.text);
            html.push_str("\"/>");
            continue;
        }
        if run.text.is_empty() {
            continue;
        }
        if let Some(url) = &run.link {
            html.push_str("<a href=\"");
            push_attr(&mut html, url);
            html.push_str("\">");
        }
        let (open, close) = tags(run.style);
        html.push_str(&open);
        push_escaped(&mut html, &run.text);
        html.push_str(&close);
        if run.link.is_some() {
            html.push_str("</a>");
        }
    }
    html
}

/// Merge adjacent runs sharing a property set.
///
/// Word routinely splits one styled word into three identically-formatted runs
/// (spell-check state, revision ids, arbitrary editing history), and ODF does the
/// same across a span boundary. The conversion would survive it either way, but
/// six tags where one will do makes the intermediate unreadable.
///
/// An image run is never merged into anything: it is one character of plain text
/// whose `text` is alt text rather than prose, and gluing it to its neighbour would
/// put both of those wrong.
fn coalesce(runs: &[Run]) -> Vec<Run> {
    let mut out: Vec<Run> = Vec::with_capacity(runs.len());
    for run in runs {
        match out.last_mut() {
            Some(last)
                if last.style == run.style
                    && last.link == run.link
                    && last.image.is_none()
                    && run.image.is_none() =>
            {
                last.text.push_str(&run.text)
            }
            _ => out.push(run.clone()),
        }
    }
    out
}

/// Escape an attribute value.
fn push_attr(html: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '&' => html.push_str("&amp;"),
            '<' => html.push_str("&lt;"),
            '>' => html.push_str("&gt;"),
            '"' => html.push_str("&quot;"),
            '\n' | '\r' => html.push(' '),
            _ => html.push(ch),
        }
    }
}

/// Outermost to innermost, so `code` never wraps markup it would have to escape.
fn tags(style: RunStyle) -> (String, String) {
    let mut open = String::new();
    let mut close = String::new();
    for (on, tag) in [
        (style.bold, "strong"),
        (style.italic, "em"),
        (style.underline, "u"),
        (style.strikethrough, "s"),
        (style.code, "code"),
    ] {
        if on {
            open.push_str(&format!("<{tag}>"));
            close.insert_str(0, &format!("</{tag}>"));
        }
    }
    (open, close)
}

/// Escape a text node, and normalise the line separators the conversion cannot see.
///
/// A literal newline inside a `<p>` survives into the plain text without producing
/// a second block, which would put every later block's offset out by one — the
/// arithmetic in [`flush`] would then be wrong for the whole rest of the run. A
/// container's *deliberate* line break is split into its own paragraph by the
/// scanner before it ever gets here; anything left is stray, and becomes a space.
fn push_escaped(html: &mut String, text: &str) {
    for ch in text.chars() {
        match ch {
            '&' => html.push_str("&amp;"),
            '<' => html.push_str("&lt;"),
            '>' => html.push_str("&gt;"),
            '\n' | '\r' => html.push(' '),
            _ => html.push(ch),
        }
    }
}

/// A short, single-line rendering of a comment body, for a diagnostic.
pub fn preview(body: &str) -> String {
    let one_line: String = body.split_whitespace().collect::<Vec<_>>().join(" ");
    let chars: Vec<char> = one_line.chars().collect();
    if chars.len() <= 60 {
        one_line
    } else {
        format!("{}…", chars[..59].iter().collect::<String>())
    }
}

#[cfg(test)]
mod tests;
