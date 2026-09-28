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
//! ## Djot written directly, and proved before it is kept
//!
//! This module used to build HTML from the runs and hand it to `text-document`'s HTML
//! reader. That reader follows HTML's rules, and HTML's rules are not a manuscript's: it
//! collapses a double space and trims a paragraph's leading tab, it has no reading for a
//! centred paragraph or a page break, and its plain text and the Djot it wrote were two
//! different parses. Every paragraph after one with a double space or a tab then failed
//! the offset check, and its comments lost their words. On a real corpus, more than a
//! quarter of paragraphs changed on their first load.
//!
//! The runs are therefore written as Djot here, by the `emit` module, with three rules
//! that remove the reasons the old note gave for going through HTML:
//!
//! * **Braced delimiters always** (`{*…*}`, `{_…_}`, `{+…+}`, `{-…-}`, `{^…^}`, `{~…~}`).
//!   They read the same inside a word and when nested, where a bare `*` or `_` depends on
//!   what surrounds it, and no Markdown is involved to swap them.
//! * **Escaping from `skrib_format`**, built on `text-document`'s own escaper and widened
//!   to the strings its parser still rewrites (`10:30:45`, straight quotes, `--`, `...`,
//!   `I. `), with the neighbouring runs as context.
//! * **A proof.** Every member of a run is parsed back with the parser the editor uses, and
//!   its text and link targets compared with the source. A member that does not read back
//!   is written again with every punctuation mark escaped, then as its words alone; one
//!   stored below its formatting is reported ([`ImportDiagnostic::ProseNotVerbatim`]).
//!   Nothing is stored that was not proved, or reported.
//!
//! What is stored is the Djot as written, never a re-export of it: the editor writes a
//! row back only once the writer changes it, so the escapes above stay until then.
//!
//! ## What a paragraph carries
//!
//! Bold, italic, underline, strikethrough, superscript, subscript and code; links;
//! footnote references and pictures, each one character of plain text; and on a paragraph,
//! centred or flush-right alignment, a right-to-left direction and a page break before it.
//! Justified text is not carried (see `emit::attribute_line`). Nor is an underline or a
//! strike-through on blank space alone, which the editor would drop on its first save; it
//! is reported instead ([`ImportDiagnostic::StyledSpacesNotCarried`]). Lists become Djot lists,
//! tables pipe tables, a quotation a blockquote, and an epigraph run one blockquote. A list
//! item nested deeper than [`MAX_LIST_LEVELS`] is written at that level and reported
//! ([`ImportDiagnostic::ListNestingFlattened`]).
//!
//! ## Where a comment ends up
//!
//! An annotation is captured against **one block's own plain text**, and the planner
//! rebases it into the row that block lands in. The proof is what makes that rebasing
//! exact: it reports where each paragraph's text starts in the stored prose and how much
//! edge whitespace was left out of it, and a comment is placed through that map rather
//! than through arithmetic. A table is proved cell by cell like any paragraph, so a comment
//! inside one keeps its words too.
//!
//! **No annotation becomes a comment on the whole document.** Neither format can express
//! one and the comment panel cannot open one. A comment that cannot be placed on its words
//! (its paragraph produced nothing, or failed the proof) becomes a comment on the nearest
//! paragraph, the one before it first, flagged [`SourceAnnotation::unanchored`]. The
//! planner reports it, once ([`ImportDiagnostic::CommentUnanchored`]), since only the
//! planner knows whether the paragraph it landed on is stored at all. A scanner says which
//! comments were made on an empty paragraph with [`mark_between_blocks`] and
//! [`settle_empty_lines`]: the index such a comment carries is the next block's, and read as
//! its own it would land there silently.

use anyhow::Result;
use skrib_format::DjotReading;
use skribisto_model::comment_anchor::{self, Anchor};
use skribisto_model::scene_break;

use crate::block::{
    AnnotationKind, SourceAnnotation, SourceAnnotationReply, SourceBlock, SourceDocument,
    SourceRowMark,
};
use crate::diagnostics::ImportDiagnostic;

mod emit;

use emit::{Frame, MemberProof, Segment, Source};

pub use super::MAX_LIST_LEVELS;

/// Character formatting a container format can express and Djot can carry.
///
/// Deliberately closed and small: these are exactly what the editor's document model
/// keeps. A field beyond them would be a promise this layer cannot keep.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct RunStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub code: bool,
    /// Raised above the baseline: an exponent, an ordinal's suffix (`1er`, `2nd`).
    pub superscript: bool,
    /// Lowered below the baseline: a chemical formula's count. When a source sets both, the
    /// raising wins, since one character cannot be both.
    pub subscript: bool,
}

/// How a paragraph is aligned, in the four values the editor's document model has.
///
/// Physical, as the editor lays a line out: [`Alignment::Left`] is the left edge whatever
/// the paragraph's direction. A scanner resolves its format's own logical values (OOXML's
/// `start`/`end`, ODF's) against the paragraph's direction before storing one here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Which way a paragraph's text runs, when the source states it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
}

/// Paragraph formatting a container states and the editor can carry.
///
/// Every field is what the source says about this one paragraph, its style chain resolved.
/// Which of them reach the stored Djot is the emitter's decision, not the scanner's.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct BlockProps {
    pub alignment: Option<Alignment>,
    pub direction: Option<Direction>,
    /// The paragraph starts a new page: OOXML's `w:pageBreakBefore` or a page break just
    /// before it, ODF's `fo:break-before="page"`.
    pub page_break_before: bool,
}

/// The object-replacement character an inline image occupies in plain text.
///
/// Not this crate's invention: it is what `text-document` counts a picture or a footnote
/// reference as, and what `comment_anchor::for_display` already knows to substitute a
/// picture glyph for. Counting it as one character here is what keeps an annotation that
/// follows an image on the right words.
pub const IMAGE_PLACEHOLDER: char = '\u{FFFC}';

/// One run of text and how it is formatted.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Run {
    pub text: String,
    pub style: RunStyle,
    /// The hyperlink this run sits inside, if any. Carried rather than flattened:
    /// a URL a writer put in their manuscript is content, and Djot has a link.
    pub link: Option<String>,
    /// This run *is* a footnote reference, and this is the label it names.
    ///
    /// A run of its own rather than a property of the surrounding text, because
    /// that is what it is in both source formats: OOXML's `<w:footnoteReference>`
    /// and ODF's `<text:note>` sit between runs, not inside one. The run carries
    /// no `text` — the printed number is derived from position, never stored, so
    /// storing one here would be a second answer to the same question.
    pub footnote: Option<String>,
    /// When set, this run **is** an image reference: `text` is its alt text, and this
    /// says where the picture is and how large it is shown. It occupies exactly one
    /// character of plain text, [`IMAGE_PLACEHOLDER`].
    pub image: Option<RunImage>,
}

/// A picture a run shows.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RunImage {
    /// Where the picture is inside the container.
    pub src: String,
    /// Its display width in pixels, at 96 to the inch: what the editor's document model
    /// measures a picture in. `0` when the source states none.
    pub width: u32,
    /// Its display height, on the same terms.
    pub height: u32,
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
            footnote: None,
        }
    }

    pub fn image(alt: impl Into<String>, src: impl Into<String>) -> Self {
        Self::sized_image(alt, src, 0, 0)
    }

    /// A picture shown at `width` by `height` pixels.
    pub fn sized_image(
        alt: impl Into<String>,
        src: impl Into<String>,
        width: u32,
        height: u32,
    ) -> Self {
        Run {
            text: alt.into(),
            style: RunStyle::default(),
            link: None,
            image: Some(RunImage {
                src: src.into(),
                width,
                height,
            }),
            footnote: None,
        }
    }

    /// This run's contribution to the block's plain text.
    ///
    /// A footnote reference contributes [`IMAGE_PLACEHOLDER`] for the same reason an
    /// image does, and it is **not** cosmetic: this string is the coordinate space
    /// every annotation offset is measured in, and it has to agree character for
    /// character with what the parser reads the stored Djot as, where both an image and
    /// a footnote reference are one object-replacement character (`text-document`'s
    /// "atomic one-character piece"). Contributing nothing here would put every comment
    /// after a footnote one character early.
    fn plain_push(&self, out: &mut String) {
        if self.image.is_some() || self.footnote.is_some() {
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
/// * **The host applications' own.** LibreOffice's `Quotations`, Word's `IntenseQuote` /
///   `BlockText`, and the `Block Quote` style Scrivener's compiler writes into the Word and
///   OpenDocument files it produces, are the styles a writer gets from the block-quote
///   button of the application they actually wrote the manuscript in. Reading them is the
///   same move as reading `style:default-outline-level` off a novel template's chapter
///   style: not name *guessing*, but the document stating what a paragraph is in the
///   vocabulary its own producer uses.
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
        "quote" | "quotations" | "intensequote" | "blocktext" | "blockquote" => {
            Some(StyledAs::Quote)
        }
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
    /// One paragraph: one line of the plain text of the run it joins.
    Paragraph {
        kind: ParagraphKind,
        runs: Vec<Run>,
        props: BlockProps,
    },
    /// Rows of cells of runs. Flushed as a block of its own, each cell proved like a
    /// paragraph.
    Table { rows: Vec<Vec<Vec<Run>>> },
}

impl RichBlock {
    /// A body paragraph with no paragraph formatting.
    pub fn body(runs: Vec<Run>) -> Self {
        RichBlock::Paragraph {
            kind: ParagraphKind::Body,
            runs,
            props: BlockProps::default(),
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

    /// Nothing here that could be a block of its own.
    ///
    /// Plain text is the measure, with one exception: a **footnote reference
    /// carries no text**. Its run is a node, not characters (see [`Run::footnote`]),
    /// so a paragraph holding nothing but a citation reads as empty — and
    /// [`assemble`] skips blank blocks outright. Word puts a note in a paragraph of
    /// its own often enough (a caption, a source line under a table) that dropping
    /// those would lose the note *and* the diagnostic, since the reference would
    /// simply never reach the prose to be counted.
    fn is_blank(&self) -> bool {
        if self.has_footnote_reference() {
            return false;
        }
        self.plain_text().trim().is_empty()
    }

    /// This block's text as a **title**.
    ///
    /// The same plain text, minus footnote references. A heading becomes a
    /// `BinderItem.title` — a plain string, not a `Content` — and a `Footnote`
    /// annotates a `Content`, so there is nothing for a note on a chapter title to
    /// hang off. Left in, the reference's object-replacement character would simply
    /// appear in the title as a stray glyph; the note itself is then cited by no row,
    /// which is what `plan` reports.
    fn title_text(&self) -> String {
        match self {
            RichBlock::Paragraph { runs, .. } => {
                let mut out = String::new();
                for run in runs.iter().filter(|r| r.footnote.is_none()) {
                    run.plain_push(&mut out);
                }
                out
            }
            RichBlock::Table { .. } => self.plain_text(),
        }
    }

    fn has_footnote_reference(&self) -> bool {
        let mut runs: Box<dyn Iterator<Item = &Run>> = match self {
            RichBlock::Paragraph { runs, .. } => Box::new(runs.iter()),
            RichBlock::Table { rows } => Box::new(
                rows.iter()
                    .flat_map(|row| row.iter())
                    .flat_map(|c| c.iter()),
            ),
        };
        runs.any(|run| run.footnote.is_some())
    }

    /// This block as the emitter reads it.
    fn source(&self) -> Source<'_> {
        match self {
            RichBlock::Paragraph { kind, runs, props } => Source::Paragraph {
                kind: *kind,
                runs,
                props: *props,
            },
            RichBlock::Table { rows } => Source::Table { rows },
        }
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
    ///
    /// For a range that runs on into a later block ([`Self::end`]), this is its length
    /// within this block, to the end of it.
    pub length: usize,
    /// Where the range ends, when an editor laid it across several paragraphs and it ends
    /// in a later block than it starts in. `None` for a range inside one block.
    pub end: Option<AnnotationEnd>,
    /// The comment was made where the file holds no text: on a paragraph holding no words,
    /// its lines all empty. No block exists there, so `block_index` is the block that
    /// followed it. (A comment on an empty line of a paragraph that has words on another
    /// stays in that paragraph instead: see [`settle_empty_lines`].)
    ///
    /// Such a comment goes on the nearest paragraph, the one before it first, since a
    /// comment on an empty line most often belongs to the passage it follows, and it is
    /// reported. Read as a position of its own, it would land silently on the paragraph
    /// after it, which may open the next chapter. A scanner sets this with
    /// [`settle_empty_lines`] or [`mark_between_blocks`], and clears it the moment the
    /// comment's range reaches words.
    pub between_blocks: bool,
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
    /// The comment's own text, one `Vec` of [`Run`]s per paragraph, **not yet
    /// Djot**. An editor's remark can carry the same formatting a manuscript
    /// paragraph can, and it is written the same way: [`assemble`] runs every
    /// annotation (and every reply) through the emitter and its proof, so a scanner
    /// never has to carry a second copy of either just to stringify a comment.
    pub paragraphs: Vec<Vec<Run>>,
    pub resolved: bool,
    pub replies: Vec<RichReply>,
}

/// Where a comment range that spans blocks ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnnotationEnd {
    /// Index into [`RichDocument::blocks`] of the block the range ends in.
    pub block_index: usize,
    /// Character offset of the end within that block's [`RichBlock::plain_text`].
    pub offset: usize,
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

/// Flag the comments a table opened where it produced no block, a table with no cells.
///
/// A scanner calls this when it has finished such a table: `from` is how many annotations
/// there were before it started, and `next_block` the index the next block will take. (A
/// paragraph's comments are placed by [`settle_empty_lines`], which knows its lines.) A
/// comment opened in the table whose range still covers nothing points at
/// `next_block` although it was made before it (see [`RichAnnotation::between_blocks`]).
/// A range still open is flagged too, and cleared by [`reaches_words`] when it closes on
/// words further on.
pub fn mark_between_blocks(annotations: &mut [RichAnnotation], from: usize, next_block: usize) {
    for annotation in annotations.iter_mut().skip(from) {
        if annotation.block_index == next_block
            && annotation.length == 0
            && annotation.end.is_none()
        {
            annotation.between_blocks = true;
        }
    }
}

/// Clear [`RichAnnotation::between_blocks`] once a range has closed on words: it then
/// starts where the next block starts, which is exactly where it was made.
pub fn reaches_words(annotation: &mut RichAnnotation) {
    if annotation.length > 0 || annotation.end.is_some() {
        annotation.between_blocks = false;
    }
}

/// An empty line of a paragraph, and the comments opened on it.
///
/// A line break ends one block and starts the next, so a paragraph is read as lines, the
/// stretches between its breaks, and a line holding nothing produces no block. A scanner
/// records each such line as it finishes it, and hands them all to [`settle_empty_lines`]
/// once the paragraph is read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyLine {
    /// Which line of the paragraph: how many line breaks come before it.
    pub line: usize,
    /// The annotations opened on it, as indices into the scanner's annotations.
    pub annotations: std::ops::Range<usize>,
}

/// Place the comments a paragraph opened on its empty lines, once the paragraph is read.
///
/// `lines` holds, for each line of the paragraph, the block it produced, or `None` for an
/// empty one; `empty` the comments opened on each empty line. Such a comment carries the
/// index of whatever block is made next, since none was made where it sits.
///
/// **A comment that covers nothing stays in its paragraph when the paragraph has words.**
/// It goes to the end of the line before it, or, when it opens the paragraph, to the start
/// of the line after it: LibreOffice writes a comment made at the top of a new page before
/// the paragraph's leading page break. It is where it was made, give or take the line
/// break, so nothing is reported. Sent to the paragraph before, as a comment on an empty
/// paragraph is, it left its own paragraph, and at the start of a scene the scene too.
///
/// Only a paragraph with no words at all makes its comments between blocks
/// ([`RichAnnotation::between_blocks`]), for the planner to place on the paragraph before
/// and report.
///
/// A range still open (`is_open`) runs on past its line. It starts on the paragraph's next
/// line when there is one, exactly where it was made, and is otherwise made between blocks
/// until [`reaches_words`] clears it.
pub fn settle_empty_lines(
    annotations: &mut [RichAnnotation],
    blocks: &[RichBlock],
    lines: &[Option<usize>],
    empty: &[EmptyLine],
    is_open: impl Fn(usize) -> bool,
) {
    for EmptyLine {
        line,
        annotations: opened,
    } in empty
    {
        let split = (*line).min(lines.len());
        let before = lines[..split].iter().rev().flatten().next().copied();
        let after = lines
            .get(split + 1..)
            .into_iter()
            .flatten()
            .flatten()
            .next()
            .copied();
        for index in opened.clone() {
            let open = is_open(index);
            let Some(annotation) = annotations.get_mut(index) else {
                continue;
            };
            if annotation.length > 0 || annotation.end.is_some() {
                continue;
            }
            if open {
                if after.is_none() {
                    annotation.between_blocks = true;
                }
                continue;
            }
            match (before, after) {
                (Some(block), _) => {
                    annotation.block_index = block;
                    annotation.start = blocks
                        .get(block)
                        .map_or(0, |b| b.plain_text().chars().count());
                }
                (None, Some(block)) => {
                    annotation.block_index = block;
                    annotation.start = 0;
                }
                (None, None) => annotation.between_blocks = true,
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
    /// The body of every footnote the document defines, keyed by the label its
    /// references name. Document-scoped rather than per-block: a note is defined
    /// once and may be cited more than once, and in OOXML its text does not even
    /// live in the same part as the reference.
    pub footnotes: Vec<RichFootnote>,
}

/// One footnote's own text, as its container expressed it.
///
/// The label is the scanner's, not the writer's — see
/// [`crate::block::SourceFootnote::label`]. The body arrives as styled paragraphs
/// on exactly the terms a comment's does, and [`assemble`] turns both into Djot at
/// the same single call site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RichFootnote {
    pub label: String,
    pub paragraphs: Vec<Vec<Run>>,
}

/// Turn a rich document into the neutral block model, carrying its comments.
///
/// `out` is filled rather than returned so a scanner can put its metadata and its own
/// diagnostics on the document first: the assembler adds its own to `out.diagnostics`
/// (paragraphs stored as plain text, lines left to fill in), and doing that to a half-built
/// document would lose whatever the scanner had already said. A comment it cannot keep on
/// its words is not reported here: it lands on the nearest paragraph, flagged
/// [`SourceAnnotation::unanchored`], and `plan` reports it, being the layer that knows
/// whether that paragraph is stored at all.
pub fn assemble(doc: &RichDocument, out: &mut SourceDocument) -> Result<()> {
    assemble_with(doc, out, &skrib_format::read_djot)
}

/// [`assemble`], proving what it writes with `read`, so a test can make a proof fail.
fn assemble_with(
    doc: &RichDocument,
    out: &mut SourceDocument,
    read: &dyn Fn(&str) -> Result<DjotReading>,
) -> Result<()> {
    let mut assembly = Assembly {
        placement: vec![None; doc.blocks.len()],
        not_verbatim: 0,
        styled_blanks: 0,
        lists_flattened: 0,
        read,
    };

    // The prose run being accumulated, as rich block indices.
    let mut pending: Vec<usize> = Vec::new();
    // The epigraph run being accumulated. Separate because an epigraph becomes a block of
    // its own; the two never hold members at once, since each is flushed the moment the
    // other starts.
    let mut pending_epigraph: Vec<usize> = Vec::new();

    for (index, block) in doc.blocks.iter().enumerate() {
        if block.is_blank() {
            // Nothing to write. A line left to fill in can be a paragraph of its own,
            // though, and it is reported with the ones written as plain spaces.
            assembly.styled_blanks += emit::styled_blanks_in(&block.source());
            continue;
        }
        let boundary = classify(block);
        // An epigraph run ends at the first block that is not part of it, and the prose
        // run ends where an epigraph starts. Flushing both at the boundary is what keeps
        // `out.blocks` in document order — which is the whole basis on which `plan`
        // decides, from adjacency alone, which heading an epigraph belongs to.
        if !matches!(boundary, Boundary::Epigraph) {
            assembly.flush(doc, &mut pending_epigraph, Frame::Epigraph, out)?;
        }
        match boundary {
            Boundary::SceneBreak(tier) => {
                assembly.flush(doc, &mut pending, Frame::Blocks, out)?;
                // Not exact: the source may spell the break `***` where the row stores
                // `* * *`. A comment on it becomes a comment on the break line.
                let glyph_len = scene_break::canonical_plain(tier).chars().count();
                assembly.placement[index] = Some(Placement {
                    source_block: out.blocks.len(),
                    segments: vec![Segment {
                        source_start: 0,
                        source_len: block.plain_text().chars().count(),
                        lead: 0,
                        stored_start: 0,
                        stored_len: glyph_len,
                    }],
                    exact: false,
                });
                out.blocks.push(SourceBlock::SceneBreak { tier });
            }
            Boundary::Heading { level, text } => {
                assembly.flush(doc, &mut pending, Frame::Blocks, out)?;
                // A heading becomes a row's title, which has no prose to point into.
                // `plan` gives a comment on it a paragraph anchor on the row's first block.
                assembly.placement[index] = Some(Placement {
                    source_block: out.blocks.len(),
                    segments: Vec::new(),
                    exact: false,
                });
                out.blocks.push(SourceBlock::Heading { level, text });
            }
            Boundary::Table => {
                // A block of its own: a table is structure the writer placed between two
                // passages, and keeping it apart keeps each passage's own block intact.
                assembly.flush(doc, &mut pending, Frame::Blocks, out)?;
                pending.push(index);
                assembly.flush(doc, &mut pending, Frame::Blocks, out)?;
            }
            Boundary::Epigraph => {
                // The prose above it is closed first, so the epigraph block lands
                // between the two runs exactly where the document put it.
                assembly.flush(doc, &mut pending, Frame::Blocks, out)?;
                pending_epigraph.push(index);
            }
            Boundary::Prose => pending.push(index),
        }
    }
    assembly.flush(doc, &mut pending_epigraph, Frame::Epigraph, out)?;
    assembly.flush(doc, &mut pending, Frame::Blocks, out)?;

    // Row marks, rebased onto the neutral block model the same way an annotation is.
    //
    // A mark whose rich block produced nothing is **dropped**, not carried to a neighbouring
    // block. It names a row by pointing at a passage, and pointing it at a different passage
    // than the one it was written into would be worse than not having it: the fallback when a
    // mark is missing is matching by type and title, which is a guess the writer can see and
    // correct, while a mark is trusted outright.
    for mark in &doc.row_marks {
        if let Some(place) = assembly
            .placement
            .get(mark.block_index)
            .and_then(Option::as_ref)
        {
            out.row_marks.push(SourceRowMark {
                block_index: place.source_block,
                uid_tag: mark.uid_tag.clone(),
                digest: mark.digest.clone(),
            });
        }
    }

    // Bodies, written and proved at the same one call site as a comment's and for the
    // same reason. A note is carried whether or not its reference survived into a
    // block: `plan` is what pairs the two up, and it is better placed to say so,
    // being the layer that knows which row the reference landed in.
    for footnote in &doc.footnotes {
        let body = assembly.body_to_djot(&footnote.paragraphs)?;
        out.footnotes.push(crate::block::SourceFootnote {
            label: footnote.label.clone(),
            body,
        });
    }

    for annotation in &doc.annotations {
        let body = assembly.body_to_djot(&annotation.paragraphs)?;
        let mut replies = Vec::with_capacity(annotation.replies.len());
        for reply in &annotation.replies {
            replies.push(SourceAnnotationReply {
                uid: reply.uid,
                author: reply.author.clone(),
                author_initials: reply.author_initials.clone(),
                created: reply.created,
                body: assembly.body_to_djot(without_reply_citation(&reply.paragraphs))?,
            });
        }

        let placed = assembly.place_annotation(annotation, out);
        out.annotations.push(SourceAnnotation {
            block_index: placed.source_block,
            kind: placed.kind,
            anchor: placed.anchor,
            unanchored: placed.unanchored,
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

    if assembly.not_verbatim > 0 {
        out.diagnostics.push(ImportDiagnostic::ProseNotVerbatim {
            path: out.origin.clone(),
            count: assembly.not_verbatim,
        });
    }
    if assembly.styled_blanks > 0 {
        out.diagnostics
            .push(ImportDiagnostic::StyledSpacesNotCarried {
                path: out.origin.clone(),
                count: assembly.styled_blanks,
            });
    }
    if assembly.lists_flattened > 0 {
        out.diagnostics
            .push(ImportDiagnostic::ListNestingFlattened {
                path: out.origin.clone(),
                count: assembly.lists_flattened,
                limit: MAX_LIST_LEVELS,
            });
    }
    Ok(())
}

/// Where one rich block ended up.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Placement {
    /// Index of the `SourceBlock` it landed in.
    source_block: usize,
    /// How its own plain text maps onto that block's text. Empty for a heading, whose
    /// text became a title.
    segments: Vec<Segment>,
    /// Whether the map was proved: the stored text reads back as the source's. When false
    /// the block is still identified, but a comment on it covers the whole paragraph
    /// rather than pointing at an unproved range.
    exact: bool,
}

impl Placement {
    /// Where the end of a range at the source character `offset` lands: in the last
    /// stretch it reaches into, clamped to that stretch, so a range running past its
    /// paragraph stops where the paragraph does.
    fn map_end(&self, offset: usize) -> Option<usize> {
        self.segments
            .iter()
            .rev()
            .find(|s| s.source_start < offset)
            .and_then(|s| s.map(offset.min(s.source_start + s.source_len)))
    }

    /// The stretch holding the source character `offset`, or the first one.
    fn segment_at(&self, offset: usize) -> Option<&Segment> {
        self.segments
            .iter()
            .find(|s| s.map(offset).is_some())
            .or(self.segments.first())
    }
}

/// Where one annotation landed.
struct Placed {
    source_block: usize,
    kind: AnnotationKind,
    anchor: Anchor,
    /// It could not be placed on its words and landed on the nearest paragraph instead.
    unanchored: bool,
}

/// The state [`assemble`] carries from one run to the next.
struct Assembly<'r> {
    /// Which `SourceBlock` each rich block ended up in, and where. `None` for a rich
    /// block that produced nothing.
    placement: Vec<Option<Placement>>,
    /// Paragraphs stored without their formatting, or not exactly as the source wrote
    /// them: the count [`ImportDiagnostic::ProseNotVerbatim`] reports.
    not_verbatim: usize,
    /// Stretches of blank space underlined or struck through, written as plain spaces or,
    /// at a paragraph's edge, not at all: the count
    /// [`ImportDiagnostic::StyledSpacesNotCarried`] reports.
    styled_blanks: usize,
    /// List items nested deeper than [`MAX_LIST_LEVELS`], written at that level: the count
    /// [`ImportDiagnostic::ListNestingFlattened`] reports.
    lists_flattened: usize,
    /// The parser every run is proved with.
    read: &'r dyn Fn(&str) -> Result<DjotReading>,
}

impl Assembly<'_> {
    /// Write the accumulated run as one block, prove it, and record where each of its
    /// members landed inside it.
    ///
    /// One function for prose and epigraphs because the placement below is the part that
    /// must not diverge: it is what a comment's position is rebased through, and two
    /// copies of it would eventually disagree about where a paragraph starts.
    fn flush(
        &mut self,
        doc: &RichDocument,
        pending: &mut Vec<usize>,
        frame: Frame,
        out: &mut SourceDocument,
    ) -> Result<()> {
        if pending.is_empty() {
            return Ok(());
        }
        let members = std::mem::take(pending);
        let sources: Vec<Source<'_>> = members.iter().map(|i| doc.blocks[*i].source()).collect();
        let proven = emit::prove_with(&sources, frame, self.read)?;
        if trim_is_empty(&proven.djot) {
            return Ok(());
        }
        let source_block = out.blocks.len();
        for (index, proof) in members.iter().zip(proven.members) {
            let MemberProof {
                segments,
                exact,
                reported,
                styled_blanks,
                list_flattened,
            } = proof;
            if reported {
                self.not_verbatim += 1;
            }
            self.styled_blanks += styled_blanks;
            if list_flattened {
                self.lists_flattened += 1;
            }
            self.placement[*index] = Some(Placement {
                source_block,
                segments,
                exact,
            });
        }
        out.blocks.push(match frame {
            Frame::Blocks => SourceBlock::Prose {
                djot: proven.djot,
                text: proven.text,
            },
            Frame::Epigraph => SourceBlock::Epigraph {
                djot: proven.djot,
                text: proven.text,
            },
        });
        Ok(())
    }

    /// A comment's or a note's own paragraphs as proved Djot.
    ///
    /// Written by the same emitter as manuscript prose and proved the same way, rather
    /// than by a comment-only writer that would eventually disagree with it. A paragraph
    /// carrying no text and no image contributes nothing, the rule
    /// [`RichBlock::is_blank`] applies to prose, so a remark that is entirely whitespace
    /// (an editor who pressed Enter twice and typed nothing) does not become an empty
    /// paragraph. A body with no such paragraph at all is the empty string. A line left to
    /// fill in that made up one of those paragraphs is still counted, as it is in prose.
    fn body_to_djot(&mut self, paragraphs: &[Vec<Run>]) -> Result<String> {
        fn body(runs: &[Run]) -> Source<'_> {
            Source::Paragraph {
                kind: ParagraphKind::Body,
                runs,
                props: BlockProps::default(),
            }
        }
        let (blank, kept): (Vec<&[Run]>, Vec<&[Run]>) =
            paragraphs.iter().map(Vec::as_slice).partition(|runs| {
                runs.iter()
                    .all(|r| r.image.is_none() && r.footnote.is_none() && r.text.trim().is_empty())
            });
        self.styled_blanks += blank
            .into_iter()
            .map(|runs| emit::styled_blanks_in(&body(runs)))
            .sum::<usize>();
        let sources: Vec<Source<'_>> = kept.into_iter().map(body).collect();
        if sources.is_empty() {
            return Ok(String::new());
        }
        let proven = emit::prove_with(&sources, Frame::Blocks, self.read)?;
        self.not_verbatim += proven.members.iter().filter(|m| m.reported).count();
        self.styled_blanks += proven
            .members
            .iter()
            .map(|m| m.styled_blanks)
            .sum::<usize>();
        Ok(proven.djot)
    }

    /// Where an annotation lands, and the selector that points there.
    ///
    /// The capture goes through `comment_anchor::capture`, never a local reimplementation:
    /// a selector built by one set of rules and resolved by another is exactly the drift
    /// that module was moved down to prevent.
    fn place_annotation(&self, annotation: &RichAnnotation, out: &SourceDocument) -> Placed {
        if annotation.between_blocks {
            // Made before `block_index`, not on it: the block itself is a candidate only
            // when nothing comes before it.
            return self.place_on_nearest(annotation.block_index, annotation.block_index, out);
        }
        let own = self
            .placement
            .get(annotation.block_index)
            .and_then(Option::as_ref);
        let after = annotation.block_index.saturating_add(1);
        let Some(place) = own else {
            return self.place_on_nearest(annotation.block_index, after, out);
        };
        let Some(block) = out.blocks.get(place.source_block) else {
            return self.place_on_nearest(annotation.block_index, after, out);
        };
        let block_text = block.plain_text();

        // A range whose both ends were proved keeps its words.
        if place.exact && (annotation.length > 0 || annotation.end.is_some()) {
            let start = place.segments.iter().find_map(|s| s.map(annotation.start));
            // A range laid across paragraphs keeps its whole extent when its end landed in
            // the same stored block, proved; otherwise it stops at the end of the
            // paragraph it started in.
            let spanned = annotation.end.and_then(|end| {
                let end_place = self.placement.get(end.block_index)?.as_ref()?;
                (end_place.exact && end_place.source_block == place.source_block)
                    .then(|| end_place.map_end(end.offset))
                    .flatten()
            });
            let end = spanned.or_else(|| place.map_end(annotation.start + annotation.length));
            if let (Some(start), Some(end)) = (start, end)
                && end > start
            {
                return Placed {
                    source_block: place.source_block,
                    kind: AnnotationKind::Range,
                    anchor: comment_anchor::capture(
                        block_text,
                        start,
                        end,
                        ordinal(block_text, start),
                    ),
                    unanchored: false,
                };
            }
        }

        // Otherwise the paragraph it sits in: a comment with no range means the whole
        // paragraph (quoting from its marker, which Word and LibreOffice put at the end,
        // would capture nothing), and one whose range could not be proved is kept on the
        // paragraph rather than pointed at unproved words.
        let unanchored = !place.exact
            && !matches!(
                block,
                SourceBlock::Heading { .. } | SourceBlock::SceneBreak { .. }
            );
        match place.segment_at(annotation.start) {
            Some(segment) => {
                let range = segment.stored();
                Placed {
                    source_block: place.source_block,
                    kind: AnnotationKind::Paragraph,
                    anchor: comment_anchor::capture(
                        block_text,
                        range.start,
                        range.end,
                        ordinal(block_text, range.start),
                    ),
                    unanchored,
                }
            }
            // A heading: `plan` pins it to the first paragraph of the row it titles.
            None => Placed {
                source_block: place.source_block,
                kind: AnnotationKind::Paragraph,
                anchor: Anchor::default(),
                unanchored,
            },
        }
    }

    /// A paragraph comment on the placed paragraph nearest a position: the last one before
    /// `before_end` when there is one, since a comment on an empty line most often belongs
    /// to the passage it follows, and otherwise the first one from `after_start` on.
    ///
    /// For a comment on a block that produced nothing, the two are that block's index and
    /// the next one. For a comment made between blocks ([`RichAnnotation::between_blocks`])
    /// both are the index of the block after it. `usize::MAX` (a comment the scanner found
    /// no position for) lands on the last paragraph.
    fn place_on_nearest(
        &self,
        before_end: usize,
        after_start: usize,
        out: &SourceDocument,
    ) -> Placed {
        let before = self.placement[..before_end.min(self.placement.len())]
            .iter()
            .rev()
            .flatten()
            .next()
            .map(|p| (p, true));
        let after = self
            .placement
            .get(after_start.min(self.placement.len())..)
            .into_iter()
            .flatten()
            .flatten()
            .next()
            .map(|p| (p, false));
        let Some((place, from_before)) = before.or(after) else {
            // Nothing in the document produced a block, so there is no paragraph to be
            // near. Pointed past the last block: `plan` finds no row holding it and reports
            // a comment it could not bring over, rather than storing it.
            return Placed {
                source_block: usize::MAX,
                kind: AnnotationKind::Paragraph,
                anchor: Anchor::default(),
                unanchored: true,
            };
        };
        let segment = if from_before {
            place.segments.last()
        } else {
            place.segments.first()
        };
        let anchor = match (segment, out.blocks.get(place.source_block)) {
            (Some(segment), Some(block)) => {
                let text = block.plain_text();
                let range = segment.stored();
                comment_anchor::capture(text, range.start, range.end, ordinal(text, range.start))
            }
            _ => Anchor::default(),
        };
        Placed {
            source_block: place.source_block,
            kind: AnnotationKind::Paragraph,
            anchor,
            unanchored: true,
        }
    }
}

/// Which block of `text` the character `offset` sits in.
fn ordinal(text: &str, offset: usize) -> usize {
    text.chars().take(offset).filter(|c| *c == '\n').count()
}

fn trim_is_empty(djot: &str) -> bool {
    djot.trim().is_empty()
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
            text: block.title_text().trim().to_string(),
        },
        RichBlock::Paragraph {
            kind: ParagraphKind::Epigraph,
            ..
        } => Boundary::Epigraph,
        RichBlock::Paragraph { .. } => Boundary::Prose,
    }
}

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
