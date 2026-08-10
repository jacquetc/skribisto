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
    Quote,
    /// `depth` is 0-based, so a top-level bullet is 0.
    ListItem {
        ordered: bool,
        depth: u8,
    },
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

    for (index, block) in doc.blocks.iter().enumerate() {
        if block.is_blank() {
            continue;
        }
        match classify(block) {
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
            Boundary::Prose => {
                pending.push((index, html_of(block), block.plain_text()));
            }
        }
    }
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
    if pending.is_empty() {
        return Ok(());
    }
    let members = std::mem::take(pending);
    let html: String = members.iter().map(|(_, h, _)| h.as_str()).collect();
    let (djot, text) = skrib_format::html_to_djot_and_text(&html)?;
    if djot.trim().is_empty() {
        return Ok(());
    }
    let source_block = out.blocks.len();
    out.blocks.push(SourceBlock::Prose { djot, text });
    let SourceBlock::Prose { text, .. } = &out.blocks[source_block] else {
        unreachable!("just pushed a prose block");
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
                ParagraphKind::Body => format!("<p>{inner}</p>"),
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
mod tests {
    use super::*;

    fn bold() -> RunStyle {
        RunStyle {
            bold: true,
            ..Default::default()
        }
    }

    fn assemble_doc(blocks: Vec<RichBlock>, annotations: Vec<RichAnnotation>) -> SourceDocument {
        let mut out = SourceDocument::new("fixture", "fixture.docx");
        assemble(
            &RichDocument {
                blocks,
                annotations,
                row_marks: Vec::new(),
            },
            &mut out,
        )
        .expect("assemble");
        out
    }

    fn annotation(block_index: usize, start: usize, length: usize) -> RichAnnotation {
        RichAnnotation {
            block_index,
            start,
            length,
            uid: None,
            uid_tag: None,
            author: "Editor".into(),
            author_initials: String::new(),
            created: None,
            paragraphs: vec![vec![Run::plain("Is this the right word?")]],
            resolved: false,
            replies: Vec::new(),
        }
    }

    #[test]
    fn styled_runs_become_djot_with_the_right_delimiters() {
        let doc = assemble_doc(
            vec![RichBlock::body(vec![
                Run::plain("He was "),
                Run::styled(
                    "utterly",
                    RunStyle {
                        italic: true,
                        ..Default::default()
                    },
                ),
                Run::plain(" lost, and "),
                Run::styled("furious", bold()),
                Run::plain("."),
            ])],
            Vec::new(),
        );
        let SourceBlock::Prose { djot, text } = &doc.blocks[0] else {
            panic!("expected prose, got {:?}", doc.blocks);
        };
        assert_eq!(djot, "He was _utterly_ lost, and *furious*.");
        assert_eq!(
            text, "He was utterly lost, and furious.",
            "the plain text is the space an anchor is measured in"
        );
    }

    /// Word splits one styled word across several identically-formatted runs. The
    /// conversion must not show the seams.
    #[test]
    fn a_word_split_across_three_identical_runs_comes_out_once() {
        let doc = assemble_doc(
            vec![RichBlock::body(vec![
                Run::styled("b", bold()),
                Run::styled("ol", bold()),
                Run::styled("d", bold()),
            ])],
            Vec::new(),
        );
        let SourceBlock::Prose { djot, .. } = &doc.blocks[0] else {
            panic!("expected prose");
        };
        assert_eq!(djot, "*bold*", "not *b**o**ld*");
    }

    #[test]
    fn a_heading_is_a_boundary_not_prose() {
        let doc = assemble_doc(
            vec![
                RichBlock::Paragraph {
                    kind: ParagraphKind::Heading { level: 2 },
                    runs: vec![Run::plain("Chapter One")],
                },
                RichBlock::body(vec![Run::plain("Prose.")]),
            ],
            Vec::new(),
        );
        assert!(matches!(
            doc.blocks.as_slice(),
            [
                SourceBlock::Heading { level: 2, .. },
                SourceBlock::Prose { .. }
            ]
        ));
    }

    /// Content beats style, the same rule the Markdown scanner applies to a raw
    /// span — so re-importing Skribisto's own export does not invent a chapter.
    #[test]
    fn a_paragraph_that_is_only_a_break_glyph_is_a_break_whatever_it_was_styled() {
        for (kind, glyph) in [
            (ParagraphKind::Body, "* * *"),
            (ParagraphKind::Heading { level: 1 }, "# # #"),
            (ParagraphKind::Body, "⁂"),
        ] {
            let doc = assemble_doc(
                vec![
                    RichBlock::body(vec![Run::plain("Before.")]),
                    RichBlock::Paragraph {
                        kind,
                        runs: vec![Run::plain(glyph)],
                    },
                    RichBlock::body(vec![Run::plain("After.")]),
                ],
                Vec::new(),
            );
            assert!(
                doc.blocks
                    .iter()
                    .any(|b| matches!(b, SourceBlock::SceneBreak { .. })),
                "{glyph:?} styled {kind:?} was not read as a break: {:?}",
                doc.blocks
            );
            assert!(
                !doc.blocks
                    .iter()
                    .any(|b| matches!(b, SourceBlock::Heading { .. })),
                "{glyph:?} must not also be a heading"
            );
        }
    }

    #[test]
    fn consecutive_paragraphs_become_one_prose_block() {
        let doc = assemble_doc(
            vec![
                RichBlock::body(vec![Run::plain("First.")]),
                RichBlock::body(vec![Run::plain("Second.")]),
                RichBlock::body(vec![Run::plain("Third.")]),
            ],
            Vec::new(),
        );
        assert_eq!(doc.blocks.len(), 1);
        let SourceBlock::Prose { text, .. } = &doc.blocks[0] else {
            panic!("expected prose");
        };
        assert_eq!(text, "First.\nSecond.\nThird.");
    }

    /// The arithmetic the whole comment feature rests on: block *n* of a run starts
    /// where the plain text says it does.
    #[test]
    fn an_annotation_on_a_later_paragraph_is_offset_by_the_ones_before_it() {
        let doc = assemble_doc(
            vec![
                RichBlock::body(vec![Run::plain("First.")]),
                RichBlock::body(vec![Run::plain("She turned the corner.")]),
            ],
            vec![annotation(1, 4, 6)], // "turned"
        );
        assert_eq!(doc.annotations.len(), 1);
        let a = &doc.annotations[0];
        assert_eq!(a.kind, AnnotationKind::Range);
        assert_eq!(a.block_index, 0, "one prose block holds both paragraphs");
    }

    #[test]
    fn a_comment_with_no_range_is_a_paragraph_comment() {
        let doc = assemble_doc(
            vec![RichBlock::body(vec![Run::plain("A paragraph.")])],
            vec![annotation(0, 0, 0)],
        );
        assert_eq!(doc.annotations[0].kind, AnnotationKind::Paragraph);
    }

    /// A table's own block positions cannot be trusted, so a comment inside one is
    /// carried as a comment on the row rather than pointed confidently at nothing.
    #[test]
    fn a_comment_inside_a_table_becomes_a_whole_row_comment() {
        let doc = assemble_doc(
            vec![RichBlock::Table {
                rows: vec![vec![vec![Run::plain("a")], vec![Run::plain("b")]]],
            }],
            vec![annotation(0, 0, 1)],
        );
        assert_eq!(doc.annotations[0].kind, AnnotationKind::Document);
    }

    #[test]
    fn a_table_is_a_prose_block_of_its_own() {
        let doc = assemble_doc(
            vec![
                RichBlock::body(vec![Run::plain("Intro.")]),
                RichBlock::Table {
                    rows: vec![vec![vec![Run::plain("a")], vec![Run::plain("b")]]],
                },
                RichBlock::body(vec![Run::plain("Outro.")]),
            ],
            Vec::new(),
        );
        assert_eq!(
            doc.blocks.len(),
            3,
            "the table must not share a block with the prose around it: {:?}",
            doc.blocks
        );
    }

    #[test]
    fn a_blank_paragraph_produces_nothing_and_does_not_shift_what_follows() {
        let doc = assemble_doc(
            vec![
                RichBlock::body(vec![Run::plain("First.")]),
                RichBlock::body(vec![Run::plain("   ")]),
                RichBlock::body(vec![Run::plain("Second.")]),
            ],
            vec![annotation(2, 0, 6)], // "Second"
        );
        let SourceBlock::Prose { text, .. } = &doc.blocks[0] else {
            panic!("expected prose");
        };
        assert_eq!(text, "First.\nSecond.");
        assert_eq!(doc.annotations[0].kind, AnnotationKind::Range);
    }

    #[test]
    fn a_comment_whose_paragraph_produced_nothing_is_reported_not_dropped() {
        let doc = assemble_doc(
            vec![
                RichBlock::body(vec![Run::plain("Kept.")]),
                RichBlock::body(vec![Run::plain("  ")]),
            ],
            vec![annotation(1, 0, 0)],
        );
        assert_eq!(doc.annotations.len(), 1, "the comment survives");
        assert_eq!(doc.annotations[0].kind, AnnotationKind::Document);
        assert!(
            doc.diagnostics
                .iter()
                .any(|d| matches!(d, crate::ImportDiagnostic::CommentUnanchored { .. })),
            "and says so: {:?}",
            doc.diagnostics
        );
    }

    #[test]
    fn lists_and_quotes_convert_and_keep_one_line_each() {
        let doc = assemble_doc(
            vec![
                RichBlock::Paragraph {
                    kind: ParagraphKind::ListItem {
                        ordered: false,
                        depth: 0,
                    },
                    runs: vec![Run::plain("one")],
                },
                RichBlock::Paragraph {
                    kind: ParagraphKind::ListItem {
                        ordered: false,
                        depth: 1,
                    },
                    runs: vec![Run::plain("deep")],
                },
                RichBlock::Paragraph {
                    kind: ParagraphKind::Quote,
                    runs: vec![Run::plain("quoted")],
                },
            ],
            Vec::new(),
        );
        let SourceBlock::Prose { djot, text } = &doc.blocks[0] else {
            panic!("expected prose");
        };
        assert!(djot.contains("- one"), "got {djot:?}");
        assert!(djot.contains("> quoted"), "got {djot:?}");
        assert_eq!(text, "one\ndeep\nquoted");
    }

    /// The characters that would otherwise become markup.
    #[test]
    fn markup_characters_in_the_prose_survive_as_themselves() {
        let doc = assemble_doc(
            vec![RichBlock::body(vec![Run::plain(
                "Tom & Jerry <3 — \"quoted\" and 5 > 3",
            )])],
            Vec::new(),
        );
        let SourceBlock::Prose { text, .. } = &doc.blocks[0] else {
            panic!("expected prose");
        };
        assert_eq!(text, "Tom & Jerry <3 — \"quoted\" and 5 > 3");
    }

    /// A stray newline inside a run would survive into the plain text without
    /// producing a block, and every later offset in the run would be wrong.
    #[test]
    fn a_stray_newline_inside_a_run_does_not_desynchronise_the_offsets() {
        let doc = assemble_doc(
            vec![
                RichBlock::body(vec![Run::plain("First\nline")]),
                RichBlock::body(vec![Run::plain("Second.")]),
            ],
            vec![annotation(1, 0, 6)],
        );
        let SourceBlock::Prose { text, .. } = &doc.blocks[0] else {
            panic!("expected prose");
        };
        assert_eq!(text, "First line\nSecond.");
        assert_eq!(
            doc.annotations[0].kind,
            AnnotationKind::Range,
            "the offset check must still have passed"
        );
    }

    #[test]
    fn a_body_preview_is_one_short_line() {
        assert_eq!(preview("  two\n  lines  "), "two lines");
        assert_eq!(preview(&"x".repeat(80)).chars().count(), 60);
    }

    // ── Rich comment bodies (M-S4) ──────────────────────────────────────────
    //
    // A comment's own text goes through the same HTML→Djot pipeline as
    // manuscript prose, so its emphasis survives instead of being flattened —
    // see `body_to_djot`'s doc.

    #[test]
    fn a_comments_own_emphasis_survives_as_djot() {
        let doc = assemble_doc(
            vec![RichBlock::body(vec![Run::plain("A paragraph.")])],
            vec![RichAnnotation {
                block_index: 0,
                start: 0,
                length: 0,
                uid: None,
                uid_tag: None,
                author: "Editor".into(),
                author_initials: String::new(),
                created: None,
                paragraphs: vec![vec![
                    Run::plain("Is this "),
                    Run::styled("really", bold()),
                    Run::plain(" the "),
                    Run::styled(
                        "right",
                        RunStyle {
                            italic: true,
                            ..Default::default()
                        },
                    ),
                    Run::plain(" word?"),
                ]],
                resolved: false,
                replies: Vec::new(),
            }],
        );
        assert_eq!(
            doc.annotations[0].body,
            "Is this *really* the _right_ word?"
        );
    }

    /// A reply carries the same richness as the opening comment — the card
    /// treats every turn alike, and so must the importer.
    #[test]
    fn a_replys_own_emphasis_survives_as_djot_too() {
        let doc = assemble_doc(
            vec![RichBlock::body(vec![Run::plain("A paragraph.")])],
            vec![RichAnnotation {
                block_index: 0,
                start: 0,
                length: 0,
                uid: None,
                uid_tag: None,
                author: "Editor".into(),
                author_initials: String::new(),
                created: None,
                paragraphs: vec![vec![Run::plain("Opening remark.")]],
                resolved: false,
                replies: vec![RichReply {
                    uid: None,
                    author: "Writer".into(),
                    author_initials: String::new(),
                    created: None,
                    paragraphs: vec![vec![Run::styled("Fixed.", bold())]],
                }],
            }],
        );
        assert_eq!(doc.annotations[0].replies.len(), 1);
        assert_eq!(doc.annotations[0].replies[0].body, "*Fixed.*");
    }

    /// A comment written as more than one paragraph — the LibreOffice
    /// convention for "Enter" inside a comment box — keeps both paragraphs
    /// rather than being glued into one run-on sentence.
    #[test]
    fn a_multi_paragraph_comment_keeps_both_paragraphs() {
        let doc = assemble_doc(
            vec![RichBlock::body(vec![Run::plain("A paragraph.")])],
            vec![RichAnnotation {
                block_index: 0,
                start: 0,
                length: 0,
                uid: None,
                uid_tag: None,
                author: "Editor".into(),
                author_initials: String::new(),
                created: None,
                paragraphs: vec![
                    vec![Run::plain("First thought.")],
                    vec![Run::plain("Second thought.")],
                ],
                resolved: false,
                replies: Vec::new(),
            }],
        );
        let body = &doc.annotations[0].body;
        assert!(body.contains("First thought."), "got {body:?}");
        assert!(body.contains("Second thought."), "got {body:?}");
        assert_ne!(
            body, "First thought.Second thought.",
            "the paragraph break must survive, not glue the two sentences together"
        );
    }

    /// A paragraph that is entirely whitespace — an editor who pressed Enter
    /// twice without typing anything — must not turn into a bare, meaningless
    /// Djot paragraph marker, the same rule `RichBlock::is_blank` applies to
    /// manuscript prose.
    #[test]
    fn a_blank_paragraph_in_a_comment_contributes_nothing() {
        let doc = assemble_doc(
            vec![RichBlock::body(vec![Run::plain("A paragraph.")])],
            vec![RichAnnotation {
                block_index: 0,
                start: 0,
                length: 0,
                uid: None,
                uid_tag: None,
                author: "Editor".into(),
                author_initials: String::new(),
                created: None,
                paragraphs: vec![vec![Run::plain("Only thought.")], vec![Run::plain("   ")]],
                resolved: false,
                replies: Vec::new(),
            }],
        );
        assert_eq!(doc.annotations[0].body, "Only thought.");
    }

    /// An annotation with no paragraphs at all (defensive — neither scanner
    /// produces this) converts to the empty string rather than erroring.
    #[test]
    fn an_annotation_with_no_paragraphs_converts_to_an_empty_body() {
        assert_eq!(body_to_djot(&[]).expect("convert"), "");
        assert_eq!(
            body_to_djot(&[vec![Run::plain("   ")]]).expect("convert"),
            ""
        );
    }

    // ── the reply-citation stripper ─────────────────────────────────────────────────────

    /// What LibreOffice actually writes, in the locales it writes it in.
    #[test]
    fn a_word_processors_own_citation_line_is_recognised() {
        for line in [
            "Répondre à  (10/08/2026, 09:27): \"en sorte qu\"",
            "Reply to Editor (08/10/2026, 09:27): \u{201C}the passage\u{201D}",
            "Antwort an Lektor (10.08.2026, 09:27): \"die Stelle\"",
            "Ответить (2026-08-10, 09:27): \u{00AB}текст\u{00BB}",
        ] {
            assert!(is_reply_citation(line), "not recognised: {line}");
        }
    }

    /// The editor's own words, which merely share the punctuation.
    ///
    /// Every one of these was eaten by the shape test before it required a clock time as well
    /// as a date: the reply arrived in the writer's thread with its first paragraph missing,
    /// on both formats, whichever application wrote the file.
    #[test]
    fn a_real_reply_that_merely_looks_like_one_is_kept() {
        for line in [
            "My note from our call (10/14): \"cut this scene entirely\"",
            "See the style guide (p. 231): \"never open on weather\"",
            "As we agreed (rev. 3.2): \"this chapter stays\"",
            "She said it herself (twice): \"I am not going\"",
            "Compare chapters 4-5 with 11-12: \"the same beat\"",
            // These three survived the first tightening — "contains a date-ish pair and a
            // time-ish pair" is satisfied by a time range, a scripture reference and a
            // chapter-and-verse span. What rules them out is that a real stamp holds nothing
            // but digits and separators.
            "Confirm the schedule (9:15-10:30): \"keep as is\"",
            "As discussed (John 3:16, Matt 5:3-12): \"consider these verses\"",
            "Check the timeline (ch. 3:16-5:22): \"way too fast\"",
        ] {
            assert!(!is_reply_citation(line), "wrongly eaten: {line}");
        }
    }

    /// The timestamp test on its own, since it is what carries the whole judgement.
    #[test]
    fn a_bare_stamp_is_told_from_anything_a_person_would_write() {
        for stamp in [
            "10/08/2026, 09:27",
            "2026-08-10, 09:27",
            "10.08.2026, 09.27",
            "08/10/2026, 9:27 AM",
            " 10/08/2026 , 09:27:31 ",
        ] {
            assert!(is_timestamp(stamp), "not a stamp: {stamp:?}");
        }
        for not in [
            "9:15-10:30",
            "John 3:16, Matt 5:3-12",
            "ch. 3:16-5:22",
            "2026, 9",
            "p. 231",
            "",
            "10/08/2026",
            "see fig. 3, table 4",
        ] {
            assert!(!is_timestamp(not), "wrongly a stamp: {not:?}");
        }
    }

    /// A citation with nothing after it is a reply whose content the editor deleted. Dropping
    /// the only paragraph would leave an empty reply rather than an odd one.
    #[test]
    fn a_reply_that_is_only_a_citation_keeps_it() {
        let only = vec![vec![Run::plain(
            "Répondre à  (10/08/2026, 09:27): \"en sorte qu\"",
        )]];
        assert_eq!(without_reply_citation(&only).len(), 1);
    }

    #[test]
    fn a_citation_ahead_of_real_words_is_dropped_and_the_words_are_not() {
        let reply = vec![
            vec![Run::plain(
                "Répondre à  (10/08/2026, 09:27): \"en sorte qu\"",
            )],
            vec![Run::plain("My reply")],
        ];
        let kept = without_reply_citation(&reply);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0][0].text, "My reply");
    }

    // ── attaching marks to annotations ──────────────────────────────────────────────────

    fn comment_mark(block: usize, start: usize, length: usize, tag: &str) -> CommentMark {
        CommentMark {
            uid_tag: tag.into(),
            block,
            start,
            length,
        }
    }

    /// The function's whole documented purpose — position, not name and not order — and until
    /// now nothing tested it with more than one comment in play. Gutting the `(block, start)`
    /// check to "the first unclaimed annotation" left every round-trip test green.
    #[test]
    fn each_mark_reaches_the_annotation_at_its_own_position() {
        // Deliberately out of order relative to the annotations, which is the case an
        // order-based match would get wrong.
        let mut annotations = vec![
            annotation(0, 5, 4),
            annotation(0, 40, 6),
            annotation(2, 5, 4),
        ];
        let marks = [
            comment_mark(2, 5, 4, "tag-third"),
            comment_mark(0, 40, 6, "tag-second"),
            comment_mark(0, 5, 4, "tag-first"),
        ];
        attach_comment_marks(&mut annotations, &marks);

        assert_eq!(annotations[0].uid_tag.as_deref(), Some("tag-first"));
        assert_eq!(annotations[1].uid_tag.as_deref(), Some("tag-second"));
        assert_eq!(annotations[2].uid_tag.as_deref(), Some("tag-third"));
    }

    /// A mark whose comment the editor deleted is simply unused, and an annotation the editor
    /// wrote themselves keeps no tag — which is what makes it import as a new comment.
    #[test]
    fn a_mark_without_an_annotation_and_an_annotation_without_a_mark_both_survive() {
        let mut annotations = vec![annotation(0, 5, 4), annotation(0, 90, 3)];
        let marks = [
            comment_mark(0, 90, 3, "tag-ours"),
            comment_mark(1, 12, 5, "tag-for-a-deleted-comment"),
        ];
        attach_comment_marks(&mut annotations, &marks);

        assert_eq!(annotations[0].uid_tag, None, "the editor's own remark");
        assert_eq!(annotations[1].uid_tag.as_deref(), Some("tag-ours"));
    }

    /// A paragraph comment stores no range of its own and takes the mark's, so the extent the
    /// editor's application maintained is what comes home.
    #[test]
    fn a_paragraph_comment_adopts_its_marks_extent_but_a_ranged_one_keeps_its_own() {
        let mut annotations = vec![annotation(0, 5, 0), annotation(1, 5, 4)];
        let marks = [
            comment_mark(0, 5, 30, "tag-para"),
            comment_mark(1, 5, 99, "tag-range"),
        ];
        attach_comment_marks(&mut annotations, &marks);

        assert_eq!(annotations[0].length, 30, "a paragraph comment adopts it");
        assert_eq!(annotations[1].length, 4, "a ranged one keeps what it had");
    }
}
