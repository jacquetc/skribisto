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
    pub author: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub body: String,
    pub resolved: bool,
    pub replies: Vec<SourceAnnotationReply>,
}

/// What a container scanner produces, before any Skribisto vocabulary is applied.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RichDocument {
    pub blocks: Vec<RichBlock>,
    pub annotations: Vec<RichAnnotation>,
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

    for annotation in &doc.annotations {
        match placement.get(annotation.block_index).copied().flatten() {
            Some(place) => {
                let block_text = out.blocks[place.source_block].plain_text().to_string();
                let member_len = doc.blocks[annotation.block_index]
                    .plain_text()
                    .chars()
                    .count();
                out.annotations
                    .push(place_annotation(annotation, place, &block_text, member_len));
            }
            // The block it pointed at produced nothing at all — a comment on a
            // paragraph that was blank, or one the scanner indexed past the end.
            // The comment is still the writer's, so it is carried as a comment on
            // the document rather than dropped, and it says so.
            None => {
                out.diagnostics
                    .push(crate::diagnostics::ImportDiagnostic::CommentUnanchored {
                        path: out.origin.clone(),
                        quote: preview(&annotation.body),
                    });
                out.annotations.push(SourceAnnotation {
                    block_index: out.blocks.len().saturating_sub(1),
                    kind: AnnotationKind::Document,
                    anchor: Anchor::default(),
                    author: annotation.author.clone(),
                    created: annotation.created,
                    body: annotation.body.clone(),
                    resolved: annotation.resolved,
                    replies: annotation.replies.clone(),
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
        author: annotation.author.clone(),
        created: annotation.created,
        body: annotation.body.clone(),
        resolved: annotation.resolved,
        replies: annotation.replies.clone(),
    }
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
            author: "Editor".into(),
            created: None,
            body: "Is this the right word?".into(),
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
}
