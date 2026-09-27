// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Styled paragraphs to Djot, and the proof that the pinned parser reads it back.
//!
//! [`render`] writes one member of a run (a paragraph or a table) and says what the parser
//! must read back for it: each block's text and link destinations, and where that text sits
//! in the member's own plain text. [`prove_with`] writes a whole run, parses it with the parser
//! the editor uses, and keeps a member's Djot only once it reads back as written. A member
//! that does not is written again with every punctuation mark escaped, then as its words
//! alone, and proved each time. What is stored is always a writing that was proved, or, when
//! none was, the plainest one, reported.

use std::ops::Range;

use anyhow::Result;
use skrib_format::{
    DjotEscaping, DjotInlineStyle, DjotReading, djot_link_destination, guard_djot_line_start,
    is_djot_whitespace, push_djot_run_with, push_djot_verbatim_run,
};
use skribisto_model::scene_break;

use super::{
    Alignment, BlockProps, Direction, IMAGE_PLACEHOLDER, MAX_LIST_LEVELS, ParagraphKind, Run,
    RunImage, RunStyle,
};

/// How many members one parse proves at once.
///
/// The pinned parser's cost grows faster than linearly with the length of what it reads,
/// while every separate parse pays a fixed setup cost. A run is therefore proved a slice at
/// a time. The slices are independent: members are separated by blank lines and none of
/// them continues into the next, so a slice reads back as it does inside the whole run.
const MEMBERS_PER_PARSE: usize = 64;

/// How much of a member's formatting its Djot keeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Fidelity {
    /// Character styles, links and block attributes, with only the escapes the parser needs.
    Formatted,
    /// The same, with every ASCII punctuation mark behind a backslash.
    Escaped,
    /// The words alone: no style, no link, no block attribute, every punctuation mark
    /// escaped. Footnote references and pictures stay, since they are not formatting.
    PlainText,
}

impl Fidelity {
    fn next(self) -> Option<Self> {
        match self {
            Fidelity::Formatted => Some(Fidelity::Escaped),
            Fidelity::Escaped => Some(Fidelity::PlainText),
            Fidelity::PlainText => None,
        }
    }
}

/// Where the members of a run are written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Frame {
    /// Each member a block of its own: manuscript prose, or a comment's or a note's text.
    Blocks,
    /// Every member inside one blockquote: the epigraph a row heads. One quotation, however
    /// many paragraphs, because the compiler marks each blockquote it meets as an epigraph.
    Epigraph,
}

/// One member of a run, as the emitter reads it.
#[derive(Debug, Clone, Copy)]
pub(super) enum Source<'a> {
    Paragraph {
        kind: ParagraphKind,
        runs: &'a [Run],
        props: BlockProps,
    },
    Table {
        rows: &'a [Vec<Vec<Run>>],
    },
}

/// One stretch of a member's own plain text, and where it landed in the run's text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Segment {
    /// Where the stretch starts in the member's own plain text, in characters.
    pub source_start: usize,
    /// Its length there, edge whitespace included.
    pub source_len: usize,
    /// How many of its leading characters are whitespace the parser does not keep.
    pub lead: usize,
    /// Where its text starts in the run's addressable text.
    pub stored_start: usize,
    /// Its length there.
    pub stored_len: usize,
}

impl Segment {
    /// Where the member's character `offset` lands in the run's text, if this stretch
    /// holds it. Whitespace that was trimmed maps to the nearest kept character.
    pub fn map(&self, offset: usize) -> Option<usize> {
        let within = offset.checked_sub(self.source_start)?;
        if within > self.source_len {
            return None;
        }
        Some(self.stored_start + within.saturating_sub(self.lead).min(self.stored_len))
    }

    /// The stretch's kept text, as a range of the run's text.
    pub fn stored(&self) -> Range<usize> {
        self.stored_start..self.stored_start + self.stored_len
    }
}

/// What the proof says about one member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MemberProof {
    /// Where each stretch of the member landed. Empty only when the run's parse could not
    /// be matched to its members at all.
    pub segments: Vec<Segment>,
    /// Whether the member reads back exactly as its source, so an offset into it can be
    /// trusted.
    pub exact: bool,
    /// Whether the member lost its formatting, or its text, on the way into storage.
    pub reported: bool,
    /// How many stretches of blank space it underlined or struck through, each written
    /// without its line (see [`styled_blanks`]). Zero for a member stored as its words
    /// alone, which `reported` already covers.
    pub styled_blanks: usize,
    /// Whether the member is a list item nested deeper than [`MAX_LIST_LEVELS`], written
    /// at that level instead (see [`list_indent`]).
    pub list_flattened: bool,
}

/// A run, written and proved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Proven {
    /// The Djot to store, exactly as written.
    pub djot: String,
    /// What the parser reads it as, the coordinate space of every [`Segment`].
    pub text: String,
    pub members: Vec<MemberProof>,
}

/// One block the parser must report for a member.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Expected {
    text: String,
    links: Vec<String>,
    /// `(start, len, lead)` in the member's own plain text, or `None` for a table cell
    /// added only to square the grid.
    source: Option<(usize, usize, usize)>,
}

/// One member written at one fidelity.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rendered {
    djot: String,
    expected: Vec<Expected>,
    /// The member's [`styled_blanks`], at the fidelity it was written in.
    styled_blanks: usize,
    /// Whether it is a list item written shallower than its source nests it.
    list_flattened: bool,
}

/// Write `sources` as one run in `frame`, prove every member with `read`, and fall back
/// member by member where the proof fails.
///
/// `read` is `skrib_format::read_djot`, the parser the editor reads prose with. It is a
/// parameter so the fallback can be exercised: nothing this emitter writes fails to read
/// back, which is the point of it.
pub(super) fn prove_with(
    sources: &[Source<'_>],
    frame: Frame,
    read: &dyn Fn(&str) -> Result<DjotReading>,
) -> Result<Proven> {
    let mut fidelity = vec![Fidelity::Formatted; sources.len()];
    let mut rendered: Vec<Rendered> = sources
        .iter()
        .map(|s| render(s, Fidelity::Formatted, frame))
        .collect();
    let mut failed = vec![false; sources.len()];

    let slices: Vec<Range<usize>> = match frame {
        // One quotation: splitting it would prove a shape that is not the one stored.
        Frame::Epigraph => std::iter::once(0..sources.len()).collect(),
        Frame::Blocks => (0..sources.len())
            .step_by(MEMBERS_PER_PARSE)
            .map(|start| start..(start + MEMBERS_PER_PARSE).min(sources.len()))
            .collect(),
    };

    let mut members: Vec<MemberProof> = Vec::with_capacity(sources.len());
    let mut texts: Vec<String> = Vec::with_capacity(slices.len());
    let mut offset = 0usize;
    for slice in slices {
        let reading = read(&join(&rendered[slice.clone()], frame))?;
        let first = align(&reading, &rendered[slice.clone()]);
        let (reading, aligned) = match first {
            Some(aligned) if aligned.iter().all(|m| m.matches) => (reading, Some(aligned)),
            _ => {
                // Something in the slice does not read back. Each member is proved alone,
                // falling back until it reads back or there is nothing plainer to write.
                for i in slice.clone() {
                    loop {
                        let alone = read(&join(std::slice::from_ref(&rendered[i]), frame))?;
                        let proved = align(&alone, std::slice::from_ref(&rendered[i]))
                            .is_some_and(|a| a.iter().all(|m| m.matches));
                        if proved {
                            break;
                        }
                        match fidelity[i].next() {
                            Some(lower) => {
                                fidelity[i] = lower;
                                rendered[i] = render(&sources[i], lower, frame);
                            }
                            None => {
                                failed[i] = true;
                                break;
                            }
                        }
                    }
                }
                let reading = read(&join(&rendered[slice.clone()], frame))?;
                let aligned = align(&reading, &rendered[slice.clone()]);
                (reading, aligned)
            }
        };

        match aligned {
            Some(aligned) => {
                for (k, i) in slice.clone().enumerate() {
                    let proof = &aligned[k];
                    members.push(MemberProof {
                        segments: segments_of(&rendered[i], &proof.starts, offset),
                        exact: proof.matches && !failed[i],
                        reported: failed[i] || !proof.matches || fidelity[i] == Fidelity::PlainText,
                        styled_blanks: rendered[i].styled_blanks,
                        list_flattened: rendered[i].list_flattened,
                    });
                }
            }
            // Every member proved alone yet the slice as a whole does not read as their
            // sum. Nothing in the emitter writes such a shape; should one appear, no offset
            // into the slice is trusted and every member is reported.
            None => {
                for i in slice.clone() {
                    members.push(MemberProof {
                        segments: Vec::new(),
                        exact: false,
                        reported: true,
                        styled_blanks: rendered[i].styled_blanks,
                        list_flattened: rendered[i].list_flattened,
                    });
                }
            }
        }
        offset += reading.text.chars().count() + 1;
        texts.push(reading.text);
    }

    let text = texts.join("\n");
    Ok(Proven {
        djot: join(&rendered, frame),
        text,
        members,
    })
}

/// One member's Djot and what it must read back as.
fn render(source: &Source<'_>, fidelity: Fidelity, frame: Frame) -> Rendered {
    match source {
        Source::Paragraph { kind, runs, props } => {
            render_paragraph(*kind, runs, *props, fidelity, frame)
        }
        Source::Table { rows } => render_table(rows, fidelity),
    }
}

/// Members joined the way the frame joins them.
fn join(rendered: &[Rendered], frame: Frame) -> String {
    let separator = match frame {
        Frame::Blocks => "\n\n",
        // An empty quoted line keeps the next paragraph inside the same blockquote.
        Frame::Epigraph => "\n>\n",
    };
    rendered
        .iter()
        .map(|r| r.djot.as_str())
        .collect::<Vec<_>>()
        .join(separator)
}

/// What aligning a parse with its members found for one member.
struct AlignedMember {
    /// The start of each of its blocks in the parse's text.
    starts: Vec<usize>,
    /// Whether every one of its blocks read back as expected.
    matches: bool,
}

/// Match a parse block for block against the members that were written, or `None` when
/// the parse holds a different number of blocks than they said they would produce.
fn align(reading: &DjotReading, rendered: &[Rendered]) -> Option<Vec<AlignedMember>> {
    let expected: usize = rendered.iter().map(|r| r.expected.len()).sum();
    if reading.blocks.len() != expected {
        return None;
    }
    let mut blocks = reading.blocks.iter();
    let mut out = Vec::with_capacity(rendered.len());
    for member in rendered {
        let mut starts = Vec::with_capacity(member.expected.len());
        let mut matches = true;
        for want in &member.expected {
            let got = blocks.next()?;
            starts.push(got.start);
            matches &= got.text == want.text && got.links == want.links;
        }
        out.push(AlignedMember { starts, matches });
    }
    Some(out)
}

/// The segments of one member, from where its blocks were found.
fn segments_of(rendered: &Rendered, starts: &[usize], offset: usize) -> Vec<Segment> {
    rendered
        .expected
        .iter()
        .zip(starts)
        .filter_map(|(want, start)| {
            let (source_start, source_len, lead) = want.source?;
            Some(Segment {
                source_start,
                source_len,
                lead,
                stored_start: offset + start,
                stored_len: want.text.chars().count(),
            })
        })
        .collect()
}

// ── one paragraph ───────────────────────────────────────────────────────────────

/// One piece of a paragraph, between the characters it occupies.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Text {
        text: String,
        style: RunStyle,
        link: Option<String>,
    },
    Image {
        alt: String,
        image: RunImage,
    },
    Footnote {
        label: String,
    },
}

impl Item {
    fn plain_len(&self) -> usize {
        match self {
            Item::Text { text, .. } => text.chars().count(),
            Item::Image { .. } | Item::Footnote { .. } => 1,
        }
    }
}

/// A container's runs as items: adjacent text with the same style and link merged, and
/// every line break turned into a space.
///
/// Word splits one styled word across several identically formatted runs, and ODF does the
/// same across span boundaries; merging them writes `{*bold*}` once rather than three
/// times. A line break a container meant is split into a paragraph of its own by the
/// scanner, so one still inside a run is stray, and Djot would read it as a space anyway.
/// Replacing it character for character keeps every offset measured in the run's text.
fn items_of(runs: &[Run]) -> Vec<Item> {
    let mut out: Vec<Item> = Vec::with_capacity(runs.len());
    for run in runs {
        if let Some(image) = &run.image {
            out.push(Item::Image {
                alt: without_line_breaks(&run.text),
                image: image.clone(),
            });
            continue;
        }
        if let Some(label) = &run.footnote {
            out.push(Item::Footnote {
                label: label.clone(),
            });
            continue;
        }
        if run.text.is_empty() {
            continue;
        }
        let link = run.link.clone().filter(|l| !l.trim().is_empty());
        let text = without_line_breaks(&run.text);
        match out.last_mut() {
            Some(Item::Text {
                text: last,
                style,
                link: last_link,
            }) if *style == run.style && *last_link == link => last.push_str(&text),
            _ => out.push(Item::Text {
                text,
                style: run.style,
                link,
            }),
        }
    }
    out
}

fn without_line_breaks(text: &str) -> String {
    text.replace(['\n', '\r'], " ")
}

/// How many stretches of blank space `items` underline or strike through: a line left to
/// fill in by hand, most often.
///
/// Such a stretch is written as plain spaces (`skrib_format::push_djot_run`): the parser
/// reads a style on spaces alone, but the editor drops it the first time it writes the
/// paragraph back, so storing it would only postpone the loss to an edit that nobody would
/// connect with the import. At a paragraph's start or end it is not written at all, since
/// the parser keeps no blank space there, styled or not, and neither is a paragraph that
/// holds nothing else ([`styled_blanks_in`]). Every one of them is counted, whichever way
/// it went. Bold, italic or a raised position on spaces alone shows nothing, and is not
/// counted.
fn styled_blanks(items: &[Item]) -> usize {
    items
        .iter()
        .filter(|item| {
            matches!(item, Item::Text { text, style, .. }
                if (style.underline || style.strikethrough)
                    && !text.is_empty()
                    && text.chars().all(is_djot_whitespace))
        })
        .count()
}

/// How many stretches of blank space a member underlines or strikes through, for a member
/// that holds nothing else and is therefore not written at all: a paragraph that is only a
/// line left to fill in, or a table of empty cells.
pub(super) fn styled_blanks_in(source: &Source<'_>) -> usize {
    match source {
        Source::Paragraph { runs, .. } => styled_blanks(&items_of(runs)),
        Source::Table { rows } => rows
            .iter()
            .flatten()
            .map(|cell| styled_blanks(&items_of(cell)))
            .sum(),
    }
}

/// The plain text the items make, the space every offset in the member is measured in.
fn plain_of(items: &[Item]) -> String {
    let mut out = String::new();
    for item in items {
        match item {
            Item::Text { text, .. } => out.push_str(text),
            Item::Image { .. } | Item::Footnote { .. } => out.push(IMAGE_PLACEHOLDER),
        }
    }
    out
}

fn render_paragraph(
    kind: ParagraphKind,
    runs: &[Run],
    props: BlockProps,
    fidelity: Fidelity,
    frame: Frame,
) -> Rendered {
    let items = items_of(runs);
    let plain = plain_of(&items);
    let chars: Vec<char> = plain.chars().collect();
    let start = chars
        .iter()
        .position(|c| !is_djot_whitespace(*c))
        .unwrap_or(chars.len());
    let end = chars
        .iter()
        .rposition(|c| !is_djot_whitespace(*c))
        .map_or(start, |i| i + 1);
    let text: String = chars[start..end].iter().collect();

    let (inline, links) = render_inline(&items, &plain, start..end, fidelity);
    let line = guard_djot_line_start(&inline);
    let attributes = match fidelity {
        Fidelity::PlainText => None,
        Fidelity::Formatted | Fidelity::Escaped => attribute_line(kind, props, &text, frame),
    };

    let mut list_flattened = false;
    let djot = match (frame, kind) {
        (Frame::Epigraph, _) | (Frame::Blocks, ParagraphKind::Quote) => {
            quoted(attributes.as_deref(), &line)
        }
        (Frame::Blocks, ParagraphKind::ListItem { ordered, depth }) => {
            let marker = if ordered { "1." } else { "-" };
            let (indent, flattened) = list_indent(depth);
            list_flattened = flattened;
            format!("{indent}{marker} {line}")
        }
        (Frame::Blocks, _) => match attributes {
            Some(attributes) => format!("{attributes}\n{line}"),
            None => line,
        },
    };
    Rendered {
        djot,
        expected: vec![Expected {
            text,
            links,
            source: Some((0, chars.len(), start)),
        }],
        styled_blanks: match fidelity {
            Fidelity::PlainText => 0,
            Fidelity::Formatted | Fidelity::Escaped => styled_blanks(&items),
        },
        list_flattened,
    }
}

/// The indentation a list item at the 0-based `depth` is written with, and whether it had
/// to be written shallower than that.
///
/// Two spaces a level, as `text-document` writes a nested list back, down to the deepest
/// level the emitter writes ([`MAX_LIST_LEVELS`]). An item nested deeper is written at that
/// level, beside the deepest items kept, rather than as prose the next load of the project
/// would refuse (`skrib_format::djot_depth`), and that `text-document` from 1.12.3 reads as
/// literal text rather than as a list. Its words, its marker and its formatting are all
/// kept.
fn list_indent(depth: u8) -> (String, bool) {
    let deepest = MAX_LIST_LEVELS.saturating_sub(1);
    let written = usize::from(depth).min(deepest);
    ("  ".repeat(written), usize::from(depth) > deepest)
}

/// A paragraph inside a blockquote, its attribute line inside the quote too.
fn quoted(attributes: Option<&str>, line: &str) -> String {
    match attributes {
        Some(attributes) => format!("> {attributes}\n> {line}"),
        None => format!("> {line}"),
    }
}

/// The block attribute line a paragraph carries, if any.
///
/// Three keys, the ones `text-document` reads on a paragraph and writes back:
///
/// * **`alignment`**, only where it differs from where the paragraph starts anyway: centred,
///   or flush with the far edge (right in left-to-right text, left in right-to-left text).
///   Justified text is not written: it is a typesetting choice the export style makes for
///   the whole book, and the editor offers no control for it, so every paragraph of a
///   justified manuscript would carry an attribute the writer could not see or remove.
/// * **`direction=rtl`**. Left to right is how a paragraph with no stated direction reads
///   Latin text, and writing it would stamp every paragraph of a document whose default
///   style names it.
/// * **`page_break_before`**, on a paragraph of the manuscript. An epigraph is placed by the
///   export style, so a page break inside one means nothing.
///
/// A list item carries none: `text-document` drops block attributes there. Nor does a line
/// that reads as a scene break, since the break vocabulary matches the bare line and an
/// attribute in front of it would hide the break.
fn attribute_line(
    kind: ParagraphKind,
    props: BlockProps,
    text: &str,
    frame: Frame,
) -> Option<String> {
    if matches!(kind, ParagraphKind::ListItem { .. })
        || scene_break::tier_of_plain_line(text.trim()).is_some()
    {
        return None;
    }
    let rtl = props.direction == Some(Direction::RightToLeft);
    let mut keys: Vec<&str> = Vec::new();
    match props.alignment {
        Some(Alignment::Center) => keys.push("alignment=center"),
        Some(Alignment::Right) if !rtl => keys.push("alignment=right"),
        Some(Alignment::Left) if rtl => keys.push("alignment=left"),
        _ => {}
    }
    if rtl {
        keys.push("direction=rtl");
    }
    if props.page_break_before && frame == Frame::Blocks {
        keys.push("page_break_before=true");
    }
    (!keys.is_empty()).then(|| format!("{{{}}}", keys.join(" ")))
}

/// The inline Djot of `items` over the characters `range` of `plain`, and the destination
/// of each link it writes, in order.
fn render_inline(
    items: &[Item],
    plain: &str,
    range: Range<usize>,
    fidelity: Fidelity,
) -> (String, Vec<String>) {
    // Byte offset of every character boundary, so the text either side of a piece can be
    // handed to the escaper as context.
    let bytes: Vec<usize> = plain
        .char_indices()
        .map(|(b, _)| b)
        .chain(std::iter::once(plain.len()))
        .collect();
    let escaping = match fidelity {
        Fidelity::Formatted => DjotEscaping::Needed,
        Fidelity::Escaped | Fidelity::PlainText => DjotEscaping::EveryMark,
    };

    let mut out = String::new();
    let mut links: Vec<String> = Vec::new();
    // The destination of the link being written, while one is open.
    let mut open_link: Option<String> = None;
    let mut position = 0usize;
    for item in items {
        let span = position..position + item.plain_len();
        position = span.end;
        let from = span.start.max(range.start);
        let to = span.end.min(range.end);
        if from >= to {
            continue;
        }

        let wanted = match item {
            Item::Text { link, .. } if fidelity != Fidelity::PlainText => link.clone(),
            _ => None,
        };
        if open_link.is_some() && open_link != wanted {
            close_link(&mut out, &mut open_link, &mut links);
        }
        if open_link.is_none()
            && let Some(url) = wanted
        {
            out.push('[');
            open_link = Some(url);
        }

        match item {
            Item::Text { style, .. } => {
                let piece = &plain[bytes[from]..bytes[to]];
                let before = &plain[bytes[range.start]..bytes[from]];
                let after = &plain[bytes[to]..bytes[range.end]];
                let djot_style = match fidelity {
                    Fidelity::PlainText => DjotInlineStyle::default(),
                    Fidelity::Formatted | Fidelity::Escaped => inline_style(*style),
                };
                if style.code && fidelity != Fidelity::PlainText {
                    push_djot_verbatim_run(&mut out, piece, djot_style);
                } else {
                    push_djot_run_with(&mut out, piece, djot_style, before, after, escaping);
                }
            }
            Item::Image { alt, image } => {
                out.push_str("![");
                push_djot_run_with(&mut out, alt, DjotInlineStyle::default(), "", "", escaping);
                out.push_str("](");
                out.push_str(&djot_link_destination(&image.src));
                out.push(')');
                // Both or neither, as `text-document` writes a picture's size back.
                if image.width > 0 && image.height > 0 {
                    out.push_str(&format!(
                        "{{width={} height={}}}",
                        image.width, image.height
                    ));
                }
            }
            // A reference is a node of its own, never inside a styled run's delimiters.
            Item::Footnote { label } => {
                out.push_str("[^");
                out.push_str(label);
                out.push(']');
            }
        }
    }
    close_link(&mut out, &mut open_link, &mut links);
    (out, links)
}

fn close_link(out: &mut String, open_link: &mut Option<String>, links: &mut Vec<String>) {
    if let Some(url) = open_link.take() {
        let destination = djot_link_destination(&url);
        out.push_str("](");
        out.push_str(&destination);
        out.push(')');
        links.push(destination);
    }
}

/// A run's character style as the Djot writer names it. Code is written as a verbatim span
/// rather than as a delimiter, so it is not part of this.
fn inline_style(style: RunStyle) -> DjotInlineStyle {
    DjotInlineStyle {
        bold: style.bold,
        italic: style.italic,
        underline: style.underline,
        strikethrough: style.strikethrough,
        superscript: style.superscript,
        subscript: style.subscript,
    }
}

// ── one table ───────────────────────────────────────────────────────────────────

/// A pipe table, its first row set off by a separator line.
///
/// The shape `text-document` writes a table back in, so the editor's first save changes
/// nothing. Every row is padded to the widest one: the editor keeps a short row as it is,
/// but writes back only as many cells of a long row as its first row has, so a ragged
/// table left as it came would lose cells on the first save.
fn render_table(rows: &[Vec<Vec<Run>>], fidelity: Fidelity) -> Rendered {
    let width = rows.iter().map(Vec::len).max().unwrap_or(0);
    let mut lines: Vec<String> = Vec::with_capacity(rows.len() + 1);
    let mut expected: Vec<Expected> = Vec::new();
    // Offset of the next cell in the table's own plain text, where cells are joined by
    // one line break.
    let mut source = 0usize;
    let mut first_cell = true;
    let mut blanks = 0usize;
    for (row_index, row) in rows.iter().enumerate() {
        let mut cells: Vec<String> = Vec::with_capacity(width);
        for column in 0..width {
            let Some(runs) = row.get(column) else {
                cells.push(String::new());
                expected.push(Expected {
                    text: String::new(),
                    links: Vec::new(),
                    source: None,
                });
                continue;
            };
            if !first_cell {
                source += 1;
            }
            first_cell = false;
            let items = items_of(runs);
            if fidelity != Fidelity::PlainText {
                blanks += styled_blanks(&items);
            }
            let plain = plain_of(&items);
            let chars: Vec<char> = plain.chars().collect();
            let start = chars
                .iter()
                .position(|c| !is_djot_whitespace(*c))
                .unwrap_or(chars.len());
            let end = chars
                .iter()
                .rposition(|c| !is_djot_whitespace(*c))
                .map_or(start, |i| i + 1);
            let (inline, links) = render_inline(&items, &plain, start..end, fidelity);
            cells.push(inline);
            expected.push(Expected {
                text: chars[start..end].iter().collect(),
                links,
                source: Some((source, chars.len(), start)),
            });
            source += chars.len();
        }
        lines.push(format!("| {} |", cells.join(" | ")));
        if row_index == 0 {
            lines.push(format!("|{}", "---|".repeat(width)));
        }
    }
    Rendered {
        djot: lines.join("\n"),
        expected,
        styled_blanks: blanks,
        list_flattened: false,
    }
}

#[cfg(test)]
mod tests;
