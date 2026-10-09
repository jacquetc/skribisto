// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Scrivener's dialect: what Scrivener writes as *text* inside its RTF, read and rewritten
//! into what the RTF reader's model says directly.
//!
//! Scrivener keeps several of its own constructs as literal text in the documents it
//! writes, which any RTF reader shows as prose:
//!
//! * **inline footnotes** `{\Scrv_fn=` … `\end_Scrv_fn}` and **inline annotations**
//!   `{\Scrv_annot \color={…} \text=` … `\end_Scrv_annot}`, their markers often written in
//!   different RTF groups from their text (on Windows, most of the time), and split into
//!   several pairs wherever the note's formatting changes;
//! * **Preserve Formatting** passages `{\Scrv_ps=` … `\end_Scrv_ps}`;
//! * **style markers** `<$Scr_Ps::N>` … `<!$Scr_Ps::N>` (paragraph), `<$Scr_Cs::N>` (character),
//!   `<$Scr_H::N>` (heading) and `<$ScrKeepWithNext>`, `N` an index into the document's own
//!   style list;
//! * **linked images** `{$SCRImageLink[w:W;h:H]=…}`.
//!
//! Its comments and linked footnotes are fields instead (`HYPERLINK "scrivcmt://<ID>"` over
//! the words they mark, their bodies in a sidecar), and links to other documents too
//! (`scrivlnk://`).
//!
//! # How it reads them
//!
//! One story at a time (the document's text, or the paragraphs of one table cell), each run
//! of paragraphs between two tables is laid out as one line of *atoms*: a character with the
//! run it came from, an inline object, or a paragraph's end. The markers are found in the
//! characters, never in the RTF bytes, whatever groups they were written in; nothing is
//! measured per paragraph, since a note may run over several (D5.1).
//!
//! Then nothing moves: markers are flagged, a note's span is flagged as leaving the host
//! text, and what the rewrite adds (a note's reference, a comment's range) is recorded as
//! an insertion before an atom. The paragraphs are rebuilt once, at the end, from the atoms
//! still standing. Every position stays an atom index throughout, so no decision can shift
//! another.
//!
//! # The rules, by decision of the design (D5)
//!
//! * Markers pair on a stack; a note inside a note, a closer with nothing open, an opener
//!   never closed or an unknown `\Scrv_` word stay the literal text they are, counted (D5.2).
//! * Notes split by formatting are joined back when nothing separates them, and so are the
//!   one-pair-per-paragraph notes Scrivener for Windows writes, when only paragraph breaks
//!   and empty paragraphs separate them; a gap of spaces keeps two notes apart (D5.3).
//! * A paragraph break just inside a note's edge belongs to the text around it; every other
//!   break inside is the note's own (D5.4).
//! * A footnote is cited where it opened (D5.5); an annotation becomes a comment on the
//!   nearest word, or on the next paragraph when it filled whole paragraphs, and a document
//!   holding nothing but annotations keeps them as its text (D5.6, X-03).
//! * A comment field becomes a range over its words, edge spaces left out (D5.9); a linked
//!   footnote is cited after the last character of its words (D5.10); a format 1.0 range is
//!   read in UTF-16 units with the fallbacks D5.11 names.
//! * Style markers always go; a heading keeps its level inside the text, a block quote and
//!   code keep their kind, every other style is only a name the report lists (D5.8).
//! * A link to another document keeps its words and loses its link, its target handed up for
//!   the reference the converter may make of it (D5.16).
//!
//! Never recursive on the input: a table's cells are rewritten from a list of stories still
//! to do.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use rtf_reader::{
    Alignment, Annotation, Block, BreakKind, CharProps, Document, FieldKind, Font, FontFamily,
    Inline, Note, NoteKind, Paragraph, ParagraphEnd, ParagraphProps, Picture, PictureFormat,
    PictureOrigin, Run, Style, StyleKind,
};
use skribisto_model::scene_break::{self, SceneBreakTier};

use super::notice::{CompileTreatment, RangeReading};
use super::sidecar::{Entry, Sidecar};
use super::styles::{ParaMeaning, StreamStyles, StyleType};

/// What a story is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Role {
    /// A document's text or notes, a project note, a recovered text: notes, comments and
    /// links are read.
    Stream,
    /// A comment's or a footnote's own text, which holds no note and no comment: any written
    /// inside one keep their words in place.
    Body,
}

/// Which blank-line and ornament conversions apply.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct SceneBreaks {
    /// The text is manuscript prose, where a blank line can be a scene break.
    pub(crate) manuscript: bool,
    /// Turn blank lines between paragraphs of prose into scene breaks (UQ-33: on).
    pub(crate) blank_lines: bool,
    /// Turn centred lines of ornaments into scene breaks (UQ-33: off).
    pub(crate) ornaments: bool,
}

/// What a story is read with.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Input<'a> {
    pub(crate) role: Role,
    pub(crate) styles: StreamStyles<'a>,
    /// The comments and footnotes beside the document, and its range links.
    pub(crate) sidecar: Option<&'a Sidecar>,
    /// The item's own title, for a first heading that repeats it.
    pub(crate) title: Option<&'a str>,
    /// The project's footnote marker glyph, when it anchors new footnotes to one.
    pub(crate) footnote_marker: Option<&'a str>,
    pub(crate) scene_breaks: SceneBreaks,
    /// Keep inline annotations as prose: the document holds nothing else.
    pub(crate) annotations_as_prose: bool,
}

/// A picture the text links to rather than holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ImageLink {
    /// The picture added to the document in its place, when it names one of the project's
    /// own items (`$PROJECT://<UUID>.<ext>`).
    pub(crate) picture: usize,
    /// That item's UUID, upper-cased.
    pub(crate) uuid: String,
}

/// What the pass found, for the report.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Found {
    pub(crate) unmatched: u32,
    pub(crate) merged: u32,
    pub(crate) joined_across: u32,
    pub(crate) blank_notes: u32,
    pub(crate) annotations: u32,
    pub(crate) annotation_colours: u32,
    pub(crate) annotations_kept_as_prose: bool,
    pub(crate) preserve_formatting: u32,
    pub(crate) nested_unwrapped: u32,
    pub(crate) styles_unresolved: u32,
    /// Style names with no meaning here, and the paragraphs or characters they covered.
    pub(crate) styles_not_carried: BTreeMap<String, u32>,
    /// Styles the project's compile leaves out or passes raw, by name and treatment, and
    /// the paragraphs or characters they covered: printed here.
    pub(crate) styles_compiled_away: BTreeMap<(String, CompileTreatment), u32>,
    pub(crate) anchors_without_entry: u32,
    pub(crate) comments_without_anchor: u32,
    pub(crate) footnotes_without_anchor: u32,
    /// Format 1.0 ranges placed by a fallback reading, per reading.
    pub(crate) range_readings: BTreeMap<RangeReadingKey, u32>,
    pub(crate) ranges_approximate: u32,
    pub(crate) comment_colours: u32,
    pub(crate) soft_breaks: u32,
    pub(crate) typed_indents: u32,
    pub(crate) left_indents: u32,
    pub(crate) blank_lines_removed: u32,
    pub(crate) blank_lines_breaks: u32,
    pub(crate) ornament_breaks: u32,
    pub(crate) ornaments_kept: u32,
    pub(crate) justified: u32,
    pub(crate) markdown: u32,
    pub(crate) citations: u32,
    pub(crate) tables: u32,
    /// Placeholders as the project writes them, and how many.
    pub(crate) placeholders: BTreeMap<String, u32>,
    /// Pictures linked to a file on disk, by file name.
    pub(crate) linked_on_disk: Vec<String>,
    pub(crate) heading_repeats_title: bool,
    /// Blank lines between a poem's stanzas, removed.
    pub(crate) stanza_gaps: u32,
}

/// [`RangeReading`] as a map key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RangeReadingKey {
    MarkersCounted,
    PicturesNotCounted,
}

impl RangeReadingKey {
    pub(crate) fn reading(self) -> RangeReading {
        match self {
            RangeReadingKey::MarkersCounted => RangeReading::MarkersCounted,
            RangeReadingKey::PicturesNotCounted => RangeReading::PicturesNotCounted,
        }
    }
}

/// A story rewritten.
#[derive(Debug, Clone)]
pub(crate) struct Rewritten {
    pub(crate) doc: Document,
    /// For each top-level block of `doc.body`, the top-level block of the original body it
    /// comes from (the paragraph whose end ends it, or the table): where a heading found in
    /// the original now is.
    pub(crate) origins: Vec<usize>,
    /// The font every run of code is in, for the mapping to read as code.
    pub(crate) code_font: Option<i32>,
    /// The targets of links to other documents, as written (a UUID or a number), each time
    /// one occurs.
    pub(crate) links: Vec<String>,
    pub(crate) image_links: Vec<ImageLink>,
    pub(crate) found: Found,
}

/// The paragraph style numbers the rewrite gives its paragraphs, below any a file uses.
const SYNTHETIC_STYLE_BASE: i32 = -1_000_000;
/// The font number the rewrite gives code, likewise.
const CODE_FONT: i32 = -1_000_000;
/// Paragraphs a blank line needs on either side, and how long each, to be read as a scene
/// break rather than as spacing around a figure or a list (the D5 critique's measured rule).
const SCENE_PROSE_PARAGRAPHS: usize = 5;
const SCENE_PROSE_CHARS: usize = 60;
/// The longest line read as an ornament.
const ORNAMENT_CHARS: usize = 16;

/// Rewrite `doc`'s body.
pub(crate) fn rewrite(mut doc: Document, input: &Input<'_>) -> Rewritten {
    let body = std::mem::take(&mut doc.body);
    let mut pass = Pass {
        input,
        doc,
        found: Found::default(),
        links: Vec::new(),
        image_links: Vec::new(),
        tag_serial: 0,
        entry_used: vec![false; input.sidecar.map_or(0, |s| s.entries.len())],
        entry_ids: input.sidecar.map(Sidecar::ids).unwrap_or_default(),
        range_order: input.sidecar.map(range_order).unwrap_or_default(),
        link_order: input.sidecar.map(link_order).unwrap_or_default(),
        ranges_unplaced: input.sidecar.map_or(0, |s| {
            s.entries.iter().filter(|e| e.range.is_some()).count()
        }),
        synthetic: HashMap::new(),
        code_font: None,
        range_base: [0; 3],
    };

    // The top-level story, then every table cell's, from a list rather than by recursion.
    let (top, origins) = pass.rewrite_list(body, true);
    pass.doc.body = top;
    let mut stack: Vec<&mut Vec<Block>> = Vec::new();
    let mut cells: Vec<Vec<Block>> = Vec::new();
    // Cells are taken out, rewritten, and put back in the order they were taken.
    collect_cells(&mut pass.doc.body, &mut stack);
    for list in &mut stack {
        cells.push(std::mem::take(*list));
    }
    let mut rewritten_cells = Vec::with_capacity(cells.len());
    let mut pending: Vec<Vec<Block>> = cells;
    // Each rewritten cell may hold tables of its own; their cells are rewritten in turn.
    while !pending.is_empty() {
        let mut next: Vec<Vec<Block>> = Vec::new();
        for blocks in pending {
            let (mut rewritten, _) = pass.rewrite_list(blocks, false);
            let mut inner: Vec<&mut Vec<Block>> = Vec::new();
            collect_cells(&mut rewritten, &mut inner);
            for list in &mut inner {
                next.push(std::mem::take(*list));
            }
            rewritten_cells.push(rewritten);
        }
        pending = next;
    }
    put_back(&mut pass.doc.body, rewritten_cells);

    // Entries the text never anchored.
    pass.anchor_leftovers();

    let Pass {
        doc,
        found,
        links,
        image_links,
        code_font,
        ..
    } = pass;
    Rewritten {
        doc,
        origins,
        code_font,
        links,
        image_links,
        found,
    }
}

/// The sidecar's ranged entries, by where their range starts.
fn range_order(sidecar: &Sidecar) -> Vec<usize> {
    let mut order: Vec<(u64, usize)> = sidecar
        .entries
        .iter()
        .enumerate()
        .filter_map(|(i, e)| e.range.map(|(location, _)| (location, i)))
        .collect();
    order.sort_unstable();
    order.into_iter().map(|(_, i)| i).collect()
}

/// The sidecar's range links, by where they start.
fn link_order(sidecar: &Sidecar) -> Vec<usize> {
    let mut order: Vec<(u64, usize)> = sidecar
        .links
        .iter()
        .enumerate()
        .map(|(i, l)| (l.range.0, i))
        .collect();
    order.sort_unstable();
    order.into_iter().map(|(_, i)| i).collect()
}

/// Every cell's block list directly inside `blocks`' tables, in order.
fn collect_cells<'b>(blocks: &'b mut [Block], out: &mut Vec<&'b mut Vec<Block>>) {
    for block in blocks {
        if let Block::Table(table) = block {
            for row in &mut table.rows {
                for cell in &mut row.cells {
                    out.push(&mut cell.blocks);
                }
            }
        }
    }
}

/// Put rewritten cells back, in the breadth-first order [`rewrite`] took them out: each
/// level's cells, then the next level's.
fn put_back(body: &mut [Block], rewritten: Vec<Vec<Block>>) {
    let mut rewritten = rewritten.into_iter();
    let mut level: Vec<&mut Vec<Block>> = Vec::new();
    collect_cells(body, &mut level);
    while !level.is_empty() {
        let mut next: Vec<&mut Vec<Block>> = Vec::new();
        for slot in level {
            if let Some(blocks) = rewritten.next() {
                *slot = blocks;
            }
            collect_cells(slot, &mut next);
        }
        level = next;
    }
}

/// One atom of a story's line. An object is boxed so that a character, which is nearly
/// every atom, costs as little as it can: a long text is laid out one atom per character.
#[derive(Debug, Clone)]
enum Atom {
    Char {
        c: char,
        run: u32,
    },
    Inline(Box<Inline>),
    /// A paragraph's end: an index into the segment's metas.
    End(u32),
}

#[derive(Debug, Clone)]
struct RunMeta {
    props: CharProps,
    group: u32,
    field: Option<usize>,
}

#[derive(Debug, Clone)]
struct Meta {
    props: ParagraphProps,
    label: Option<String>,
    end: ParagraphEnd,
    origin: usize,
}

/// Who an insertion belongs to when it sits inside a note's span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Owner {
    /// The text around the note: a note's own reference, a comment placed by a rule.
    Host,
    /// Whichever text holds its position.
    Any,
}

/// A run of paragraphs between two tables, laid out as atoms.
struct Segment {
    atoms: Vec<Atom>,
    runs: Vec<RunMeta>,
    metas: Vec<Meta>,
    /// The meta of the paragraph each atom is in.
    para: Vec<u32>,
    /// The characters, a paragraph end as `\n`, and the atom each is.
    text: Vec<char>,
    at: Vec<usize>,
    /// Marker characters, emitted nowhere.
    marker: Vec<bool>,
    /// Atoms that left the host text for a note.
    in_span: Vec<bool>,
    /// Characters in a code span.
    code: Vec<bool>,
    inserts: BTreeMap<usize, Vec<(Inline, Owner)>>,
    /// Fields whose runs lose their field: links dropped, comments turned into ranges.
    unlinked: HashSet<usize>,
    /// What each paragraph's style means, by meta.
    meaning: Vec<ParaMeaning>,
}

impl Segment {
    fn build(paragraphs: Vec<(Paragraph, usize)>) -> Segment {
        let mut seg = Segment {
            atoms: Vec::new(),
            runs: Vec::new(),
            metas: Vec::new(),
            para: Vec::new(),
            text: Vec::new(),
            at: Vec::new(),
            marker: Vec::new(),
            in_span: Vec::new(),
            code: Vec::new(),
            inserts: BTreeMap::new(),
            unlinked: HashSet::new(),
            meaning: Vec::new(),
        };
        for (paragraph, origin) in paragraphs {
            let meta = seg.metas.len() as u32;
            seg.metas.push(Meta {
                props: paragraph.props,
                label: paragraph.label,
                end: paragraph.end,
                origin,
            });
            for inline in paragraph.inlines {
                match inline {
                    Inline::Text(run) => {
                        let index = seg.runs.len() as u32;
                        seg.runs.push(RunMeta {
                            props: run.props,
                            group: run.group,
                            field: run.field,
                        });
                        for c in run.text.chars() {
                            seg.text.push(c);
                            seg.at.push(seg.atoms.len());
                            seg.atoms.push(Atom::Char { c, run: index });
                            seg.para.push(meta);
                        }
                    }
                    other => {
                        seg.atoms.push(Atom::Inline(Box::new(other)));
                        seg.para.push(meta);
                    }
                }
            }
            seg.text.push('\n');
            seg.at.push(seg.atoms.len());
            seg.atoms.push(Atom::End(meta));
            seg.para.push(meta);
        }
        let n = seg.atoms.len();
        seg.marker = vec![false; n];
        seg.in_span = vec![false; n];
        seg.code = vec![false; n];
        seg.meaning = vec![ParaMeaning::None; seg.metas.len()];
        seg
    }

    fn insert(&mut self, before: usize, inline: Inline, owner: Owner) {
        self.inserts
            .entry(before)
            .or_default()
            .push((inline, owner));
    }

    /// The atom standing in the host text at `i`.
    fn host(&self, i: usize) -> bool {
        !self.marker[i] && !self.in_span[i]
    }

    fn is_blank_char(&self, i: usize) -> bool {
        matches!(self.atoms[i], Atom::Char { c, .. } if c.is_whitespace() || c == '\u{FFFC}')
    }

    /// Whether atom `i` is writing a reader would see: a character that is not blank, a
    /// picture, a note's reference.
    fn is_content(&self, i: usize) -> bool {
        match &self.atoms[i] {
            Atom::Char { c, .. } => !c.is_whitespace() && *c != '\u{FFFC}',
            Atom::Inline(inline) => {
                matches!(**inline, Inline::Picture(_) | Inline::NoteReference(_))
            }
            _ => false,
        }
    }

    /// Whether an insertion before `i` that the host text shows is writing.
    fn host_insert_is_content(&self, i: usize) -> bool {
        self.inserts.get(&i).is_some_and(|list| {
            list.iter().any(|(inline, owner)| {
                (*owner == Owner::Host || !self.in_span[i])
                    && matches!(inline, Inline::NoteReference(_) | Inline::Picture(_))
            })
        })
    }
}

/// A token found in a segment's characters.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    /// Characters `start..end` of the segment's text.
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Open(NoteKind2),
    Close(NoteKind2),
    /// `{\Scrv_…` or `\end_Scrv_…` of a kind nobody writes: literal.
    Unknown,
    StyleOpen(Family, u32),
    StyleClose(Family, u32),
    /// `<$ScrKeepWithNext>` and any other `<$Scr…>` marker: removed, meaning nothing.
    ScrMarker,
    ImageLink {
        width: Option<u32>,
        height: Option<u32>,
        target: String,
    },
    Placeholder(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NoteKind2 {
    Footnote,
    /// With its colour, as written: a colour change splits an annotation in two.
    Annotation(Option<String>),
    Preserve,
}

impl NoteKind2 {
    fn same_kind(&self, other: &NoteKind2) -> bool {
        matches!(
            (self, other),
            (NoteKind2::Footnote, NoteKind2::Footnote)
                | (NoteKind2::Annotation(_), NoteKind2::Annotation(_))
                | (NoteKind2::Preserve, NoteKind2::Preserve)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Family {
    Paragraph,
    Character,
    Heading,
}

/// A note's markers, paired.
#[derive(Debug, Clone)]
struct Pair {
    kind: NoteKind2,
    open: (usize, usize),
    close: (usize, usize),
}

/// A note's span taken out of the host text.
struct Span {
    /// The body's atoms, `None` when it holds nothing.
    body: Option<(usize, usize)>,
    /// A paragraph break just after the opener stayed in the host text.
    leading_break: bool,
    /// A paragraph break just before the closer stayed in the host text.
    trailing_break: bool,
}

/// A note made of one pair or several joined.
#[derive(Debug, Clone)]
struct Group {
    kind: NoteKind2,
    /// Text positions: the first opener's start and end, the last closer's start and end.
    open: (usize, usize),
    close: (usize, usize),
    /// Every marker of the pairs joined, the first opener and the last closer included.
    markers: Vec<(usize, usize)>,
    /// Whether it took a pair nothing separated, or one only paragraph breaks did: each
    /// counts the note once in the report, however many pieces it joined.
    merged: bool,
    joined_across: bool,
}

/// One field of a segment: its id, the atom that opens it and the atom that closes it.
type FieldSpan = (usize, usize, usize);

struct Pass<'i, 'a> {
    input: &'i Input<'a>,
    doc: Document,
    found: Found,
    links: Vec<String>,
    image_links: Vec<ImageLink>,
    tag_serial: u32,
    entry_used: Vec<bool>,
    /// The sidecar's entries by UUID, found once rather than searched for every field.
    entry_ids: HashMap<&'a str, usize>,
    /// The sidecar's ranged entries, by where their range starts: each segment looks only
    /// at those that can fall inside it.
    range_order: Vec<usize>,
    /// The sidecar's range links, by where they start, likewise.
    link_order: Vec<usize>,
    /// Ranged entries not yet placed.
    ranges_unplaced: usize,
    synthetic: HashMap<ParaKey, i32>,
    code_font: Option<i32>,
    /// Where the next segment starts in a format 1.0 range's units, per reading.
    range_base: [u64; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ParaKey {
    Heading(u8),
    Quote,
    CodeBlock,
}

impl Pass<'_, '_> {
    /// Rewrite one story: its runs of paragraphs, table by table. Tables pass through, their
    /// cells rewritten by the caller.
    fn rewrite_list(&mut self, blocks: Vec<Block>, top: bool) -> (Vec<Block>, Vec<usize>) {
        let mut out: Vec<Block> = Vec::with_capacity(blocks.len());
        let mut origins: Vec<usize> = Vec::with_capacity(blocks.len());
        let mut run: Vec<(Paragraph, usize)> = Vec::new();
        for (index, block) in blocks.into_iter().enumerate() {
            match block {
                Block::Paragraph(paragraph) => run.push((paragraph, index)),
                table @ Block::Table(_) => {
                    self.flush(std::mem::take(&mut run), &mut out, &mut origins);
                    self.found.tables += 1;
                    if self.has_ranges() {
                        // The units a format 1.0 range counts for a table: its cells' text and
                        // one for each cell's end, as the text Scrivener measured holds them.
                        let units: u64 = rtf_reader::paragraphs(std::slice::from_ref(&table))
                            .map(|p| {
                                p.runs()
                                    .map(|r| r.text.encode_utf16().count() as u64)
                                    .sum::<u64>()
                                    + 1
                            })
                            .sum();
                        for base in &mut self.range_base {
                            *base += units;
                        }
                    }
                    out.push(table);
                    origins.push(index);
                }
            }
        }
        self.flush(run, &mut out, &mut origins);
        if top && self.input.role == Role::Stream {
            self.story_rules(&mut out);
        }
        (out, origins)
    }

    /// Whether the sidecar anchors anything by range (format 1.0).
    fn has_ranges(&self) -> bool {
        self.input
            .sidecar
            .is_some_and(|s| s.entries.iter().any(|e| e.range.is_some()) || !s.links.is_empty())
    }

    fn flush(
        &mut self,
        paragraphs: Vec<(Paragraph, usize)>,
        out: &mut Vec<Block>,
        origins: &mut Vec<usize>,
    ) {
        if paragraphs.is_empty() {
            return;
        }
        for (paragraph, origin) in self.rewrite_segment(paragraphs) {
            out.push(Block::Paragraph(paragraph));
            origins.push(origin);
        }
    }

    fn rewrite_segment(&mut self, paragraphs: Vec<(Paragraph, usize)>) -> Vec<(Paragraph, usize)> {
        let mut seg = Segment::build(paragraphs);
        let tokens = scan(&seg.text);
        self.fields(&mut seg);
        // Markers first: a style range is measured over the characters that are not.
        self.markers(&mut seg, &tokens);
        self.styles(&mut seg, &tokens);
        self.notes(&mut seg, &tokens);
        self.ranges(&mut seg, &tokens);
        self.rebuild(&seg)
    }

    // ── fields: comments, linked footnotes, document links ─────────────────────────

    fn fields(&mut self, seg: &mut Segment) {
        let starts: Vec<(usize, usize)> = seg
            .atoms
            .iter()
            .enumerate()
            .filter_map(|(i, a)| match a {
                Atom::Inline(inline) => match **inline {
                    Inline::FieldStart(id) => Some((i, id)),
                    _ => None,
                },
                _ => None,
            })
            .collect();
        // Where each field ends, found once: a search per field would cost the text's length
        // for every link in it.
        let mut ends: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, atom) in seg.atoms.iter().enumerate() {
            if let Atom::Inline(inline) = atom
                && let Inline::FieldEnd(id) = **inline
            {
                ends.entry(id).or_default().push(i);
            }
        }
        // A comment's fields, by the entry they name, in the order the text first names
        // each: Scrivener may write one comment over several paragraphs as one field per
        // paragraph, which is still one comment.
        let mut comments: Vec<(String, Vec<FieldSpan>)> = Vec::new();
        let mut comment_at: HashMap<String, usize> = HashMap::new();
        for (start, id) in starts {
            let target = match self.doc.fields.get(id).map(|f| f.kind()) {
                Some(FieldKind::Hyperlink {
                    target: Some(target),
                    ..
                }) => target,
                _ => continue,
            };
            let lower = target.trim().to_ascii_lowercase();
            let end = ends
                .get(&id)
                .and_then(|list| list.iter().find(|&&e| e > start).copied())
                .unwrap_or(seg.atoms.len());
            if let Some(rest) = lower.strip_prefix("scrivcmt://") {
                let key = rest.trim_end_matches('/').to_ascii_uppercase();
                match comment_at.get(&key).and_then(|&at| comments.get_mut(at)) {
                    Some((_, fields)) => fields.push((id, start, end)),
                    None => {
                        comment_at.insert(key.clone(), comments.len());
                        comments.push((key, vec![(id, start, end)]));
                    }
                }
            } else if let Some(rest) = lower.strip_prefix("scrivlnk://") {
                self.links
                    .push(rest.trim_end_matches('/').to_ascii_uppercase());
                self.drop_field(seg, id, start, end);
            }
            // Any other link stays for the mapping, which keeps a web or mail address and
            // counts the rest as words without their link.
        }
        for (key, fields) in comments {
            self.comment_fields(seg, &key, &fields);
        }
    }

    /// A field whose link goes: its words stay, as plain text.
    fn drop_field(&mut self, seg: &mut Segment, id: usize, start: usize, end: usize) {
        seg.unlinked.insert(id);
        seg.marker[start] = true;
        if end < seg.atoms.len() {
            seg.marker[end] = true;
        }
    }

    /// The fields naming one comment or linked footnote: a range over their words, or the
    /// footnote cited after them.
    fn comment_fields(&mut self, seg: &mut Segment, key: &str, fields: &[FieldSpan]) {
        for (id, start, end) in fields {
            self.drop_field(seg, *id, *start, *end);
        }
        let Some(&(_, start, _)) = fields.first() else {
            return;
        };
        let entry = match (self.input.role, self.input.sidecar) {
            (Role::Stream, Some(_)) => self
                .entry_ids
                .get(key.trim().to_ascii_uppercase().as_str())
                .copied(),
            _ => None,
        };
        let Some(entry_index) = entry else {
            self.found.anchors_without_entry += 1;
            return;
        };
        // A comment a table interrupted: its first part carries it, the rest keep their words.
        if self.entry_used.get(entry_index).copied().unwrap_or(true) {
            return;
        }
        let Some(entry) = self.input.sidecar.and_then(|s| s.entries.get(entry_index)) else {
            return;
        };
        self.mark_used(entry_index);
        // The words, their edge spaces left out.
        let words: Vec<usize> = fields
            .iter()
            .flat_map(|(_, start, end)| start + 1..*end)
            .filter(|&i| matches!(seg.atoms[i], Atom::Char { .. }) && !seg.is_blank_char(i))
            .collect();
        let (first, last) = match (words.iter().min(), words.iter().max()) {
            (Some(&first), Some(&last)) => (Some(first), Some(last)),
            _ => (None, None),
        };
        if entry.footnote {
            // A footnote marker the project anchors new footnotes to is its own mark, not
            // words: it goes, and the reference takes its place.
            if let (Some(marker), Some(first), Some(last)) =
                (self.input.footnote_marker, first, last)
            {
                let text: String = (first..=last)
                    .filter_map(|i| match seg.atoms[i] {
                        Atom::Char { c, .. } => Some(c),
                        _ => None,
                    })
                    .collect();
                if text == marker {
                    for i in first..=last {
                        seg.marker[i] = true;
                    }
                }
            }
            let note = self.graft_note(entry);
            let at = last.map_or(start, |last| last + 1);
            seg.insert(at, Inline::NoteReference(note), Owner::Any);
            return;
        }
        let annotation = self.graft_annotation(entry, true);
        let tag = annotation.reference.clone();
        let index = self.doc.annotations.len();
        self.doc.annotations.push(annotation);
        match (first, last, tag) {
            (Some(first), Some(last), Some(tag)) => {
                seg.insert(first, Inline::AnnotationRangeStart(tag.clone()), Owner::Any);
                seg.insert(first, Inline::AnnotationReference(index), Owner::Any);
                seg.insert(last + 1, Inline::AnnotationRangeEnd(tag), Owner::Any);
            }
            _ => seg.insert(start, Inline::AnnotationReference(index), Owner::Any),
        }
        if entry.coloured {
            self.found.comment_colours += 1;
        }
    }

    // ── style markers ────────────────────────────────────────────────────────────────

    fn styles(&mut self, seg: &mut Segment, tokens: &[Token]) {
        // Ranges: family, index, start and end characters.
        let mut open: HashMap<(u8, u32), Vec<usize>> = HashMap::new();
        let mut ranges: Vec<(Family, u32, usize, usize)> = Vec::new();
        let family_key = |f: Family| match f {
            Family::Paragraph => 0u8,
            Family::Character => 1,
            Family::Heading => 2,
        };
        for token in tokens {
            match &token.kind {
                TokenKind::StyleOpen(family, n) => open
                    .entry((family_key(*family), *n))
                    .or_default()
                    .push(token.end),
                TokenKind::StyleClose(family, n) => {
                    if let Some(start) = open.get_mut(&(family_key(*family), *n)).and_then(Vec::pop)
                    {
                        ranges.push((*family, *n, start, token.start));
                    }
                }
                _ => {}
            }
        }
        // A range never closed ends with the paragraph it opened in.
        for ((family, n), starts) in open {
            let family = match family {
                0 => Family::Paragraph,
                1 => Family::Character,
                _ => Family::Heading,
            };
            for start in starts {
                let end = (start..seg.text.len())
                    .find(|&i| seg.text[i] == '\n')
                    .unwrap_or(seg.text.len());
                ranges.push((family, n, start, end));
            }
        }
        if ranges.is_empty() {
            return;
        }
        ranges.sort_by_key(|r| (r.2, r.3));

        // Which paragraphs and characters each range reaches: one sweep over the
        // characters, the ranges opening and closing as it goes.
        let mut events: Vec<(usize, bool, usize)> = Vec::with_capacity(ranges.len() * 2);
        for (k, range) in ranges.iter().enumerate() {
            events.push((range.2, true, k));
            events.push((range.3, false, k));
        }
        events.sort_by_key(|e| (e.0, e.1));
        let mut active: Vec<usize> = Vec::new();
        let mut next = 0usize;
        // Bumped whenever a range opens or closes, so a paragraph records what is open once
        // per change rather than once per character.
        let mut generation = 0usize;
        let mut paragraph_ranges: HashMap<u32, Vec<usize>> = HashMap::new();
        let mut char_counts: BTreeMap<String, u32> = BTreeMap::new();
        let mut compiled_counts: BTreeMap<(String, CompileTreatment), u32> = BTreeMap::new();
        let mut last_seen: Option<(u32, usize)> = None;
        let mut resolved: HashMap<(u8, u32), Option<&super::styles::StyleDef>> = HashMap::new();
        for position in 0..seg.text.len() {
            while next < events.len() && events[next].0 <= position {
                let (_, opening, k) = events[next];
                if opening {
                    active.push(k);
                } else if let Some(at) = active.iter().position(|a| *a == k) {
                    active.remove(at);
                }
                generation += 1;
                next += 1;
            }
            if active.is_empty() || seg.text[position] == '\n' {
                continue;
            }
            let atom = seg.at[position];
            if seg.marker[atom] {
                continue;
            }
            let paragraph = seg.para[atom];
            if last_seen != Some((paragraph, generation)) {
                paragraph_ranges
                    .entry(paragraph)
                    .or_default()
                    .extend(active.iter().copied());
                last_seen = Some((paragraph, generation));
            }
            for &k in &active {
                let (family, n, _, _) = ranges[k];
                if family != Family::Character {
                    continue;
                }
                let def = *resolved
                    .entry((1, n))
                    .or_insert_with(|| self.input.styles.resolve(n));
                if let Some(treatment) = def.and_then(|d| d.compile)
                    && let Some(def) = def
                {
                    *compiled_counts
                        .entry((def.name.clone(), treatment))
                        .or_default() += 1;
                }
                match def {
                    Some(def) if def.is_code_span() => seg.code[atom] = true,
                    Some(def) => *char_counts.entry(def.name.clone()).or_default() += 1,
                    None => {}
                }
            }
        }
        for (name, count) in char_counts {
            *self.found.styles_not_carried.entry(name).or_default() += count;
        }
        for (key, count) in compiled_counts {
            *self.found.styles_compiled_away.entry(key).or_default() += count;
        }
        // Unresolved markers are counted once per range.
        for (family, n, _, _) in &ranges {
            if *family != Family::Heading && self.input.styles.resolve(*n).is_none() {
                self.found.styles_unresolved += 1;
            }
        }
        for (paragraph, mut hit) in paragraph_ranges {
            hit.sort_unstable();
            hit.dedup();
            let mut meaning = ParaMeaning::None;
            let mut names: Vec<String> = Vec::new();
            let mut compiled: BTreeSet<(String, CompileTreatment)> = BTreeSet::new();
            for k in hit {
                let (family, n, _, _) = ranges[k];
                let candidate = match family {
                    Family::Heading => ParaMeaning::Heading(n.clamp(1, 6) as u8),
                    Family::Paragraph => match self.input.styles.resolve(n) {
                        Some(def) if def.kind != StyleType::Character => {
                            if let Some(treatment) = def.compile {
                                compiled.insert((def.name.clone(), treatment));
                            }
                            let m = def.paragraph_meaning();
                            if m == ParaMeaning::None {
                                names.push(def.name.clone());
                            }
                            m
                        }
                        _ => ParaMeaning::None,
                    },
                    Family::Character => ParaMeaning::None,
                };
                meaning = stronger(meaning, candidate);
            }
            if let Some(slot) = seg.meaning.get_mut(paragraph as usize) {
                *slot = meaning;
            }
            if meaning == ParaMeaning::None {
                for name in names {
                    *self.found.styles_not_carried.entry(name).or_default() += 1;
                }
            }
            // Whatever the paragraph became here, the compile did not print it as prose.
            for key in compiled {
                *self.found.styles_compiled_away.entry(key).or_default() += 1;
            }
        }
    }

    // ── markers that simply go, and what they stand for ──────────────────────────────

    fn markers(&mut self, seg: &mut Segment, tokens: &[Token]) {
        for token in tokens {
            match &token.kind {
                TokenKind::StyleOpen(..) | TokenKind::StyleClose(..) | TokenKind::ScrMarker => {
                    mark(seg, token);
                }
                TokenKind::ImageLink {
                    width,
                    height,
                    target,
                } => {
                    mark(seg, token);
                    let at = seg.at[token.start];
                    match project_image(target) {
                        Some(uuid) => {
                            let id = self.doc.pictures.len();
                            self.doc.pictures.push(Picture {
                                format: PictureFormat::Unknown,
                                origin: PictureOrigin::Bare,
                                name: Some(target.rsplit('/').next().unwrap_or(target).to_string()),
                                // Points, taken as the logical pixels the editor measures in;
                                // fifteen twips to one.
                                goal_width: width.map(|w| (w as i32).saturating_mul(15)),
                                goal_height: height.map(|h| (h as i32).saturating_mul(15)),
                                ..Picture::default()
                            });
                            seg.insert(at, Inline::Picture(id), Owner::Any);
                            self.image_links.push(ImageLink { picture: id, uuid });
                        }
                        None => self.found.linked_on_disk.push(file_name_of(target)),
                    }
                }
                TokenKind::Placeholder(kind) => {
                    *self.found.placeholders.entry(kind.clone()).or_default() += 1;
                }
                _ => {}
            }
        }
    }

    // ── inline notes ─────────────────────────────────────────────────────────────────

    fn notes(&mut self, seg: &mut Segment, tokens: &[Token]) {
        // Pair on a stack of one: Scrivener's notes do not nest.
        let mut pairs: Vec<Pair> = Vec::new();
        let mut open: Option<(NoteKind2, (usize, usize))> = None;
        for token in tokens {
            match &token.kind {
                TokenKind::Open(kind) => match open {
                    Some(_) => self.found.unmatched += 1,
                    None => open = Some((kind.clone(), (token.start, token.end))),
                },
                TokenKind::Close(kind) => match open.take() {
                    Some((opened, at)) if opened.same_kind(kind) => pairs.push(Pair {
                        kind: opened,
                        open: at,
                        close: (token.start, token.end),
                    }),
                    Some(other) => {
                        // A closer of another kind: literal, and the open note stays open.
                        self.found.unmatched += 1;
                        open = Some(other);
                    }
                    None => self.found.unmatched += 1,
                },
                TokenKind::Unknown => self.found.unmatched += 1,
                _ => {}
            }
        }
        if open.is_some() {
            self.found.unmatched += 1;
        }

        let groups = self.merge(seg, pairs);
        let mut annotations: Vec<Group> = Vec::new();
        for group in groups {
            let unwrap = match &group.kind {
                NoteKind2::Preserve => {
                    self.found.preserve_formatting += 1;
                    true
                }
                NoteKind2::Footnote => self.input.role == Role::Body,
                NoteKind2::Annotation(_) => {
                    self.input.role == Role::Body || self.input.annotations_as_prose
                }
            };
            if unwrap {
                if self.input.role == Role::Body && group.kind != NoteKind2::Preserve {
                    self.found.nested_unwrapped += 1;
                }
                for (start, end) in &group.markers {
                    mark_chars(seg, *start, *end);
                }
                continue;
            }
            match group.kind {
                NoteKind2::Footnote => self.footnote(seg, &group),
                NoteKind2::Annotation(_) => annotations.push(group),
                NoteKind2::Preserve => {}
            }
        }
        // Annotations after every footnote, so that a footnote's reference beside one counts
        // as the words it is when the annotation looks for its paragraph. Every annotation
        // leaves the host text before any is placed: one placed first would otherwise take
        // the markers of the next for words, and each placement would scan past all the
        // others still standing, which costs the square of their number in one paragraph.
        if annotations.is_empty() {
            return;
        }
        let spans: Vec<(Group, Span)> = annotations
            .into_iter()
            .map(|group| {
                let span = self.take_span(seg, &group);
                (group, span)
            })
            .collect();
        let nearest = Nearest::of(seg);
        for (group, span) in spans {
            self.annotation(seg, &group, span, &nearest);
        }
    }

    /// Join pairs of one note: those nothing separates, and those only paragraph breaks and
    /// empty paragraphs separate (D5.3).
    fn merge(&mut self, seg: &Segment, pairs: Vec<Pair>) -> Vec<Group> {
        let mut groups: Vec<Group> = Vec::new();
        for pair in pairs {
            if let Some(last) = groups.last_mut()
                && last.kind == pair.kind
                && !matches!(pair.kind, NoteKind2::Preserve)
            {
                // The report counts notes, not joins: a note made of three pieces is one.
                match gap(seg, last.close.1, pair.open.0) {
                    Gap::None => {
                        if !last.merged {
                            last.merged = true;
                            self.found.merged += 1;
                        }
                        last.close = pair.close;
                        last.markers.extend([pair.open, pair.close]);
                        continue;
                    }
                    Gap::Breaks => {
                        if !last.joined_across {
                            last.joined_across = true;
                            self.found.joined_across += 1;
                        }
                        last.close = pair.close;
                        last.markers.extend([pair.open, pair.close]);
                        continue;
                    }
                    Gap::Text => {}
                }
            }
            groups.push(Group {
                kind: pair.kind,
                open: pair.open,
                close: pair.close,
                markers: vec![pair.open, pair.close],
                merged: false,
                joined_across: false,
            });
        }
        groups
    }

    /// Take a note's span out of the host text, keeping a break at its edge there (D5.4).
    fn take_span(&mut self, seg: &mut Segment, group: &Group) -> Span {
        let first = seg.at[group.open.0];
        let last = seg.at[group.close.1 - 1];
        let body_start = if group.open.1 < seg.text.len() {
            seg.at[group.open.1]
        } else {
            seg.atoms.len()
        };
        let body_end = seg.at[group.close.0];
        for i in first..=last {
            seg.in_span[i] = true;
        }
        for (start, end) in &group.markers {
            mark_chars(seg, *start, *end);
        }
        // A break with only blank space between it and the closer, or the opener, is the
        // host's.
        let edge = |range: &mut dyn Iterator<Item = usize>| -> Option<usize> {
            for i in range {
                if seg.marker[i] {
                    continue;
                }
                match seg.atoms[i] {
                    Atom::End(_) => return Some(i),
                    Atom::Char { .. } if seg.is_blank_char(i) => continue,
                    Atom::Inline(ref inline)
                        if matches!(**inline, Inline::FieldStart(_) | Inline::FieldEnd(_)) =>
                    {
                        continue;
                    }
                    _ => return None,
                }
            }
            None
        };
        let trailing = edge(&mut (body_start..body_end).rev());
        let leading = edge(&mut (body_start..body_end)).filter(|l| Some(*l) != trailing);
        for i in [trailing, leading].into_iter().flatten() {
            seg.in_span[i] = false;
        }
        let holds = (body_start..body_end)
            .any(|i| seg.in_span[i] && !seg.marker[i] && seg.is_content(i))
            || (body_start..=body_end).any(|i| {
                seg.inserts.get(&i).is_some_and(|l| {
                    l.iter().any(|(inline, owner)| {
                        *owner == Owner::Any
                            && matches!(inline, Inline::Picture(_) | Inline::NoteReference(_))
                    })
                })
            });
        Span {
            body: holds.then_some((body_start, body_end)),
            leading_break: leading.is_some(),
            trailing_break: trailing.is_some(),
        }
    }

    fn footnote(&mut self, seg: &mut Segment, group: &Group) {
        let first = seg.at[group.open.0];
        let Some((start, end)) = self.take_span(seg, group).body else {
            self.found.blank_notes += 1;
            return;
        };
        let blocks = body_blocks(seg, start, end);
        let id = self.doc.notes.len();
        self.doc.notes.push(Note {
            kind: NoteKind::Footnote,
            blocks: blocks.into_iter().map(Block::Paragraph).collect(),
        });
        seg.insert(first, Inline::NoteReference(id), Owner::Host);
    }

    /// Place an annotation whose span has been taken: `nearest` answers where the host's
    /// writing is, once every annotation has left it.
    fn annotation(&mut self, seg: &mut Segment, group: &Group, span: Span, nearest: &Nearest) {
        let first = seg.at[group.open.0];
        let last = seg.at[group.close.1 - 1];
        let colour = matches!(&group.kind, NoteKind2::Annotation(Some(_)));
        let Some((start, end)) = span.body else {
            self.found.blank_notes += 1;
            return;
        };
        let blocks = body_blocks(seg, start, end);
        self.found.annotations += 1;
        if colour {
            self.found.annotation_colours += 1;
        }
        self.tag_serial += 1;
        let tag = format!("scrv-annot-{}", self.tag_serial);
        let index = self.doc.annotations.len();
        self.doc.annotations.push(Annotation {
            reference: Some(tag.clone()),
            blocks: blocks.into_iter().map(Block::Paragraph).collect(),
            ..Annotation::default()
        });

        // A break kept at the note's edge is where its paragraph starts or ends: the text on
        // the far side of it is another paragraph's.
        let before = (!span.leading_break).then(|| nearest.back(first)).flatten();
        let after = (!span.trailing_break)
            .then(|| nearest.forward(last + 1))
            .flatten();
        if before.is_none() && after.is_none() {
            // Whole paragraphs: a comment on the next paragraph, else the previous one.
            let next = if span.trailing_break {
                nearest.first_content(last + 1)
            } else {
                nearest.next_paragraph(last + 1)
            };
            let target = next.or_else(|| nearest.previous_paragraph(first));
            let at = target.unwrap_or(first);
            seg.insert(at, Inline::AnnotationReference(index), Owner::Host);
            return;
        }
        // Inside a paragraph: the nearest word, the one before it first.
        let word = before
            .and_then(|at| word_around(seg, at))
            .or_else(|| after.and_then(|at| word_around(seg, at)));
        match word {
            Some((word_start, word_end)) => {
                seg.insert(
                    word_start,
                    Inline::AnnotationRangeStart(tag.clone()),
                    Owner::Host,
                );
                seg.insert(word_start, Inline::AnnotationReference(index), Owner::Host);
                seg.insert(word_end + 1, Inline::AnnotationRangeEnd(tag), Owner::Host);
            }
            None => seg.insert(first, Inline::AnnotationReference(index), Owner::Host),
        }
    }

    // ── format 1.0 range comments and links ─────────────────────────────────────────

    fn ranges(&mut self, seg: &mut Segment, tokens: &[Token]) {
        let Some(sidecar) = self.input.sidecar else {
            return;
        };
        if self.ranges_unplaced == 0 && sidecar.links.is_empty() {
            self.advance_base(seg, tokens);
            return;
        }
        // Units before each atom, under each reading.
        let token_char: Vec<bool> = {
            let mut is = vec![false; seg.text.len()];
            for token in tokens {
                if matches!(
                    token.kind,
                    TokenKind::Open(NoteKind2::Footnote | NoteKind2::Annotation(_))
                        | TokenKind::Close(NoteKind2::Footnote | NoteKind2::Annotation(_))
                ) {
                    for flag in &mut is[token.start..token.end] {
                        *flag = true;
                    }
                }
            }
            is
        };
        let mut is_token_atom = vec![false; seg.atoms.len()];
        for (position, flag) in token_char.iter().enumerate() {
            if *flag {
                is_token_atom[seg.at[position]] = true;
            }
        }
        let readings = [
            RangeReading::MarkersRemoved,
            RangeReading::MarkersCounted,
            RangeReading::PicturesNotCounted,
        ];
        let mut units: Vec<Vec<u64>> = Vec::with_capacity(3);
        for (r, reading) in readings.iter().enumerate() {
            let mut before = Vec::with_capacity(seg.atoms.len() + 1);
            let mut total = self.range_base[r];
            for (i, atom) in seg.atoms.iter().enumerate() {
                before.push(total);
                let width = match atom {
                    Atom::Char { c, .. } => {
                        if is_token_atom[i] && *reading != RangeReading::MarkersCounted {
                            0
                        } else {
                            c.len_utf16() as u64
                        }
                    }
                    Atom::End(_) => 1,
                    Atom::Inline(inline) => match **inline {
                        Inline::Break(_) => 1,
                        Inline::Picture(_) => {
                            u64::from(*reading != RangeReading::PicturesNotCounted)
                        }
                        _ => 0,
                    },
                };
                total += width;
            }
            before.push(total);
            units.push(before);
        }
        for (r, base) in self.range_base.iter_mut().enumerate() {
            *base = units[r].last().copied().unwrap_or(*base);
        }
        // Only a range starting inside the segment, under some reading, can be placed in it.
        let low = units.iter().filter_map(|u| u.first().copied()).min();
        let high = units.iter().filter_map(|u| u.last().copied()).max();
        let (Some(low), Some(high)) = (low, high) else {
            return;
        };
        let location_of = |i: &usize| {
            sidecar
                .entries
                .get(*i)
                .and_then(|e| e.range)
                .map_or(0, |r| r.0)
        };
        let from = self.range_order.partition_point(|i| location_of(i) < low);
        let to = self.range_order.partition_point(|i| location_of(i) <= high);
        let mut wanted: Vec<usize> = self
            .range_order
            .get(from..to)
            .unwrap_or_default()
            .iter()
            .copied()
            .filter(|i| !self.entry_used.get(*i).copied().unwrap_or(true))
            .collect();
        // In the sidecar's order, as the notes and comments it makes are numbered.
        wanted.sort_unstable();
        let edges = if wanted.is_empty() {
            Edges::default()
        } else {
            Edges::of(seg)
        };

        for entry_index in wanted {
            let Some(entry) = sidecar.entries.get(entry_index) else {
                continue;
            };
            // A range the file states past any text (a crafted `.links`) fits no reading: the
            // entry is placed with the others the text never anchored.
            let Some((location, end)) = entry
                .range
                .and_then(|(location, length)| Some((location, location.checked_add(length)?)))
            else {
                continue;
            };
            let mut placed: Option<(usize, usize, usize)> = None;
            let mut fallback: Option<(usize, usize)> = None;
            for (r, before) in units.iter().enumerate() {
                let Some((start, end)) = atoms_for(before, location, end) else {
                    continue;
                };
                if fallback.is_none() {
                    fallback = Some((start, end));
                }
                if on_word_boundaries(seg, start, end) {
                    placed = Some((r, start, end));
                    break;
                }
            }
            let (start, end) = match (placed, fallback) {
                (Some((r, start, end)), _) => {
                    let key = match r {
                        1 => Some(RangeReadingKey::MarkersCounted),
                        2 => Some(RangeReadingKey::PicturesNotCounted),
                        _ => None,
                    };
                    if let Some(key) = key {
                        *self.found.range_readings.entry(key).or_default() += 1;
                    }
                    (start, end)
                }
                (None, Some((start, end))) => {
                    self.found.ranges_approximate += 1;
                    (start, end)
                }
                // Not in this segment: a later one, or none.
                (None, None) => continue,
            };
            self.mark_used(entry_index);
            // A range inside a note's span is placed where the note is cited.
            let host = |i: usize| -> usize {
                let mut i = i.min(seg.atoms.len().saturating_sub(1));
                while i > 0 && seg.in_span[i] {
                    i -= 1;
                }
                i
            };
            let (start, end) = (host(start), host(end.saturating_sub(1)) + 1);
            // The words the range covers, its edge spaces and line breaks left out, as a
            // comment field's are: a selection that took the blank line above its
            // paragraph, or its own line break, would otherwise hang its ends in a line
            // that is not kept, or in the next paragraph.
            let (start, end) = match (edges.first_at(start), edges.last_before(end)) {
                (Some(first), Some(last)) if first <= last && last < end => (first, last + 1),
                _ => (start, end),
            };
            if entry.footnote {
                let note = self.graft_note(entry);
                // After the range's last character that is not blank.
                seg.insert(end, Inline::NoteReference(note), Owner::Host);
            } else {
                let annotation = self.graft_annotation(entry, true);
                let tag = annotation.reference.clone();
                let index = self.doc.annotations.len();
                self.doc.annotations.push(annotation);
                if entry.coloured {
                    self.found.comment_colours += 1;
                }
                if let Some(tag) = tag {
                    seg.insert(
                        start,
                        Inline::AnnotationRangeStart(tag.clone()),
                        Owner::Host,
                    );
                    seg.insert(end, Inline::AnnotationRangeEnd(tag), Owner::Host);
                }
                seg.insert(start, Inline::AnnotationReference(index), Owner::Host);
            }
        }
        // Range links: a reference target each, the text untouched.
        let top = units
            .first()
            .map(|u| (u.first().copied(), u.last().copied()));
        if let Some((Some(from), Some(to))) = top {
            let start_of = |i: &usize| sidecar.links.get(*i).map_or(0, |l| l.range.0);
            let first = self.link_order.partition_point(|i| start_of(i) < from);
            let end = self.link_order.partition_point(|i| start_of(i) < to);
            let mut inside: Vec<usize> =
                self.link_order.get(first..end).unwrap_or_default().to_vec();
            inside.sort_unstable();
            for i in inside {
                if let Some(link) = sidecar.links.get(i) {
                    self.links.push(link.target.clone());
                }
            }
        }
    }

    /// Record that the sidecar entry `index` has its place.
    fn mark_used(&mut self, index: usize) {
        let ranged = self
            .input
            .sidecar
            .and_then(|s| s.entries.get(index))
            .is_some_and(|e| e.range.is_some());
        if let Some(used) = self.entry_used.get_mut(index)
            && !*used
        {
            *used = true;
            if ranged {
                self.ranges_unplaced = self.ranges_unplaced.saturating_sub(1);
            }
        }
    }

    /// Move the range bases past a segment with nothing to place.
    fn advance_base(&mut self, seg: &Segment, tokens: &[Token]) {
        if !self.has_ranges() {
            return;
        }
        let token_units: u64 = tokens
            .iter()
            .filter(|t| {
                matches!(
                    t.kind,
                    TokenKind::Open(NoteKind2::Footnote | NoteKind2::Annotation(_))
                        | TokenKind::Close(NoteKind2::Footnote | NoteKind2::Annotation(_))
                )
            })
            .map(|t| {
                seg.text[t.start..t.end]
                    .iter()
                    .map(|c| c.len_utf16() as u64)
                    .sum::<u64>()
            })
            .sum();
        let mut plain = 0u64;
        let mut pictures = 0u64;
        for atom in &seg.atoms {
            match atom {
                Atom::Char { c, .. } => plain += c.len_utf16() as u64,
                Atom::End(_) => plain += 1,
                Atom::Inline(inline) => match **inline {
                    Inline::Break(_) => plain += 1,
                    Inline::Picture(_) => pictures += 1,
                    _ => {}
                },
            }
        }
        self.range_base[0] += plain - token_units + pictures;
        self.range_base[1] += plain + pictures;
        self.range_base[2] += plain - token_units;
    }

    // ── what was never anchored ─────────────────────────────────────────────────────

    /// Comments and footnotes the sidecar holds that the text never anchored: a comment on
    /// the first paragraph, a footnote cited at the end of the text (X-03).
    fn anchor_leftovers(&mut self) {
        let Some(sidecar) = self.input.sidecar else {
            return;
        };
        if self.input.role != Role::Stream {
            return;
        }
        let leftovers: Vec<usize> = (0..sidecar.entries.len())
            .filter(|i| !self.entry_used.get(*i).copied().unwrap_or(true))
            .collect();
        if leftovers.is_empty() {
            return;
        }
        if !self
            .doc
            .body
            .iter()
            .any(|b| matches!(b, Block::Paragraph(_)))
        {
            self.doc.body.push(Block::Paragraph(Paragraph::default()));
        }
        // Every leftover's place is found once and filled once: a sidecar may hold many
        // thousands of entries whose anchors a damaged text lost.
        let mut opening: Vec<Inline> = Vec::new();
        let mut closing: Vec<Inline> = Vec::new();
        for index in leftovers {
            let Some(entry) = sidecar.entries.get(index) else {
                continue;
            };
            if entry.footnote {
                self.found.footnotes_without_anchor += 1;
                let note = self.graft_note(entry);
                closing.push(Inline::NoteReference(note));
            } else {
                self.found.comments_without_anchor += 1;
                let annotation = self.graft_annotation(entry, false);
                let annotation_index = self.doc.annotations.len();
                self.doc.annotations.push(annotation);
                if entry.coloured {
                    self.found.comment_colours += 1;
                }
                opening.push(Inline::AnnotationReference(annotation_index));
            }
        }
        // The comments in the order the sidecar holds them, each put before the ones after
        // it had been, as one insertion at a time at the front would have left them.
        opening.reverse();
        let is_paragraph = |b: &Block| matches!(b, Block::Paragraph(_));
        let first = self.doc.body.iter().position(is_paragraph);
        let last = self.doc.body.iter().rposition(is_paragraph);
        if let Some(Block::Paragraph(paragraph)) = last.and_then(|i| self.doc.body.get_mut(i)) {
            paragraph.inlines.extend(closing);
        }
        if let Some(Block::Paragraph(paragraph)) = first.and_then(|i| self.doc.body.get_mut(i)) {
            paragraph.inlines.splice(0..0, opening);
        }
    }

    // ── bodies from the sidecar ─────────────────────────────────────────────────────

    /// A sidecar footnote's body, as a note of this document.
    fn graft_note(&mut self, entry: &Entry) -> usize {
        let blocks = self.body_of(entry);
        let id = self.doc.notes.len();
        self.doc.notes.push(Note {
            kind: NoteKind::Footnote,
            blocks,
        });
        id
    }

    /// A sidecar comment as an annotation of this document, with a range tag when `ranged`.
    ///
    /// Its author is the `Author` Scrivener wrote, else the name of an exact typed "Name,
    /// date, time" last line (which stays in the body: the date is the writer's record),
    /// else nobody (D5.12). Never the importing writer: they did not write these remarks.
    fn graft_annotation(&mut self, entry: &Entry, ranged: bool) -> Annotation {
        let blocks = self.body_of(entry);
        let reference = ranged.then(|| {
            self.tag_serial += 1;
            format!("scrv-cmt-{}", self.tag_serial)
        });
        let author = entry.author.clone().or_else(|| {
            let last = rtf_reader::paragraphs(&blocks)
                .map(plain_text)
                .filter(|t| !t.trim().is_empty())
                .last()?;
            signed_name(&last)
        });
        Annotation {
            author,
            reference,
            blocks,
            ..Annotation::default()
        }
    }

    /// A sidecar entry's body read, its own dialect read, and moved into this document: its
    /// fields and pictures renumbered into this document's lists, its fonts and colours
    /// dropped, since they name the body's own tables.
    fn body_of(&mut self, entry: &Entry) -> Vec<Block> {
        let text = entry.body.trim();
        let parsed = if text.starts_with("{\\rtf") {
            rtf_reader::parse_str(text).ok()
        } else {
            None
        };
        let Some(body) = parsed else {
            // Not RTF: the body as plain text, one paragraph a line.
            return text
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|line| {
                    Block::Paragraph(Paragraph {
                        inlines: vec![Inline::Text(Run {
                            text: line.to_string(),
                            props: CharProps::default(),
                            group: 0,
                            field: None,
                        })],
                        ..Paragraph::default()
                    })
                })
                .collect();
        };
        let inner_input = Input {
            role: Role::Body,
            styles: StreamStyles::default(),
            sidecar: None,
            title: None,
            footnote_marker: None,
            scene_breaks: SceneBreaks::default(),
            annotations_as_prose: false,
        };
        let rewritten = rewrite(body, &inner_input);
        self.found.nested_unwrapped += rewritten.found.nested_unwrapped;
        self.found.unmatched += rewritten.found.unmatched;
        self.links.extend(rewritten.links);
        let body = rewritten.doc;
        let field_base = self.doc.fields.len();
        let picture_base = self.doc.pictures.len();
        self.doc.fields.extend(body.fields);
        self.doc.pictures.extend(body.pictures);
        let mut blocks = body.body;
        renumber(&mut blocks, field_base, picture_base);
        blocks
    }

    // ── rebuilding ──────────────────────────────────────────────────────────────────

    fn rebuild(&mut self, seg: &Segment) -> Vec<(Paragraph, usize)> {
        let mut out: Vec<(Paragraph, usize)> = Vec::new();
        let mut builder = Builder::default();
        let code_font = if seg.code.iter().any(|c| *c) {
            Some(self.code_font())
        } else {
            None
        };
        // Paragraphs something was taken out of. One left with no writing at all is not
        // kept: it held only what left (an annotation, a note), and kept blank it would
        // read as a blank line, which the scene-break rules may take for a break.
        let mut touched = vec![false; seg.metas.len()];
        for i in 0..seg.atoms.len() {
            if (seg.marker[i] || seg.in_span[i])
                && let Some(flag) = touched.get_mut(seg.para[i] as usize)
            {
                *flag = true;
            }
        }
        for i in 0..seg.atoms.len() {
            if let Some(list) = seg.inserts.get(&i) {
                for (inline, owner) in list {
                    if *owner == Owner::Host || !seg.in_span[i] {
                        builder.inline(inline.clone());
                    }
                }
            }
            if !seg.host(i) {
                continue;
            }
            match &seg.atoms[i] {
                Atom::Char { c, run } => builder.char(*c, *run, seg.code[i]),
                Atom::Inline(inline) => builder.inline((**inline).clone()),
                Atom::End(meta) => {
                    let emptied = touched.get(*meta as usize).copied().unwrap_or(false)
                        && !builder.holds_writing();
                    let inlines = builder.finish(seg, code_font);
                    if emptied {
                        // What it still held (a comment's place) goes with the next one.
                        for inline in inlines {
                            if !matches!(inline, Inline::Text(_)) {
                                builder.inline(inline);
                            }
                        }
                        continue;
                    }
                    let paragraph = self.paragraph(seg, *meta, inlines);
                    let origin = seg.metas.get(*meta as usize).map_or(0, |m| m.origin);
                    out.push((paragraph, origin));
                }
            }
        }
        if let Some(list) = seg.inserts.get(&seg.atoms.len()) {
            for (inline, _) in list {
                builder.inline(inline.clone());
            }
        }
        let rest = builder.finish(seg, code_font);
        if !rest.is_empty() {
            match out.last_mut() {
                Some((paragraph, _)) => paragraph.inlines.extend(rest),
                None => out.push((
                    Paragraph {
                        inlines: rest,
                        ..Paragraph::default()
                    },
                    0,
                )),
            }
        }
        out
    }

    fn paragraph(&mut self, seg: &Segment, meta: u32, inlines: Vec<Inline>) -> Paragraph {
        let Some(m) = seg.metas.get(meta as usize) else {
            return Paragraph {
                inlines,
                ..Paragraph::default()
            };
        };
        let mut props = m.props;
        match seg
            .meaning
            .get(meta as usize)
            .copied()
            .unwrap_or(ParaMeaning::None)
        {
            ParaMeaning::Heading(level) => {
                props.style = Some(self.synthetic_style(ParaKey::Heading(level)));
                props.outline_level = None;
            }
            ParaMeaning::Quote => {
                props.style = Some(self.synthetic_style(ParaKey::Quote));
                props.outline_level = None;
            }
            ParaMeaning::CodeBlock => {
                props.style = Some(self.synthetic_style(ParaKey::CodeBlock));
                props.outline_level = None;
            }
            ParaMeaning::None => {}
        }
        Paragraph {
            props,
            label: m.label.clone(),
            inlines,
            end: m.end,
        }
    }

    fn synthetic_style(&mut self, key: ParaKey) -> i32 {
        if let Some(number) = self.synthetic.get(&key) {
            return *number;
        }
        let number = SYNTHETIC_STYLE_BASE - self.synthetic.len() as i32;
        let (name, outline_level) = match key {
            ParaKey::Heading(level) => (format!("heading {level}"), Some(level.saturating_sub(1))),
            ParaKey::Quote => ("Block Quote".to_string(), None),
            ParaKey::CodeBlock => ("Code Block".to_string(), None),
        };
        self.doc.styles.push(Style {
            number,
            kind: StyleKind::Paragraph,
            name,
            based_on: None,
            next: None,
            outline_level,
        });
        self.synthetic.insert(key, number);
        number
    }

    fn code_font(&mut self) -> i32 {
        if let Some(font) = self.code_font {
            return font;
        }
        self.doc.fonts.push(Font {
            number: CODE_FONT,
            family: FontFamily::Modern,
            charset: None,
            code_page: None,
            name: "code".to_string(),
        });
        self.code_font = Some(CODE_FONT);
        CODE_FONT
    }

    // ── rules over the whole text ───────────────────────────────────────────────────

    /// What the top-level story's paragraphs say as a whole: scene breaks made of blank
    /// lines and ornaments, and the counts the report needs.
    fn story_rules(&mut self, blocks: &mut [Block]) {
        let texts: Vec<Option<String>> = blocks
            .iter()
            .map(|b| match b {
                Block::Paragraph(p) => Some(plain_text(p)),
                Block::Table(_) => None,
            })
            .collect();
        let kinds: Vec<LineKind> = blocks
            .iter()
            .zip(&texts)
            .map(|(b, t)| match (b, t) {
                (Block::Paragraph(p), Some(text)) => line_kind(p, text),
                _ => LineKind::Other,
            })
            .collect();

        for (block, text) in blocks.iter().zip(&texts) {
            let (Block::Paragraph(p), Some(text)) = (block, text) else {
                continue;
            };
            if text.trim().is_empty() {
                continue;
            }
            if text.starts_with([' ', '\t']) {
                self.found.typed_indents += 1;
            }
            if p.props.left_indent > 0 && p.props.list.is_none() && p.label.is_none() {
                self.found.left_indents += 1;
            }
            if p.props.alignment == Some(Alignment::Justify) {
                self.found.justified += 1;
            }
            self.found.soft_breaks += p
                .inlines
                .iter()
                .map(|i| match i {
                    Inline::Break(BreakKind::Line) => 1,
                    Inline::Text(run) => run
                        .text
                        .chars()
                        .filter(|c| matches!(c, '\u{2028}' | '\u{2029}'))
                        .count() as u32,
                    _ => 0,
                })
                .sum::<u32>();
            if reads_as_markdown(text) {
                self.found.markdown += 1;
            }
            if holds_citation_key(text) {
                self.found.citations += 1;
            }
        }

        // A first heading repeating the title (D5.8, UQ-35: kept and reported).
        if let Some(title) = self.input.title
            && let Some((block, text)) = blocks
                .iter()
                .zip(&texts)
                .find(|(_, t)| t.as_deref().is_some_and(|t| !t.trim().is_empty()))
            && let Block::Paragraph(p) = block
            && p.props.style.is_some_and(|s| {
                self.synthetic
                    .iter()
                    .any(|(k, n)| *n == s && matches!(k, ParaKey::Heading(_)))
            })
            && let Some(text) = text
            && fold(text) == fold(title)
            && !fold(title).is_empty()
        {
            self.found.heading_repeats_title = true;
        }

        // Scene breaks from blank lines and ornaments (D5.20, D5.22).
        let breaks = self.input.scene_breaks;
        let prose = kinds.iter().filter(|k| **k == LineKind::Prose).count();
        // Each blank line that becomes a break, and the paragraph written after it.
        let mut to_break: Vec<(usize, usize)> = Vec::new();
        let mut to_remove: u32 = 0;
        if breaks.manuscript && prose >= SCENE_PROSE_PARAGRAPHS {
            // Joints between consecutive written paragraphs, and those with a blank line.
            let written: Vec<usize> = (0..kinds.len())
                .filter(|i| kinds[*i] != LineKind::Blank)
                .collect();
            let blank_joints = written.windows(2).filter(|w| w[1] > w[0] + 1).count();
            let contiguous = written.len() > 1 && blank_joints * 5 <= (written.len() - 1);
            for w in written.windows(2) {
                let gap = w[1] - w[0] - 1;
                let between_prose =
                    kinds[w[0]] == LineKind::Prose && kinds[w[1]] == LineKind::Prose;
                let is_break =
                    between_prose && ((contiguous && gap == 1) || (!contiguous && gap == 2));
                if is_break && breaks.blank_lines {
                    to_break.push((w[0] + 1, w[1]));
                }
            }
        }
        // Every blank paragraph that does not become a break is removed by the mapping.
        let blank_total = kinds.iter().filter(|k| **k == LineKind::Blank).count() as u32;
        for &(at, next) in &to_break {
            let mut held: Vec<Inline> = Vec::new();
            if let Some(Block::Paragraph(p)) = blocks.get_mut(at) {
                // What a blank line holds besides its spaces (a comment's place, a page
                // break) goes to the paragraph after it rather than with the spaces.
                held = std::mem::replace(&mut p.inlines, vec![break_run()])
                    .into_iter()
                    .filter(|inline| !matches!(inline, Inline::Text(_)))
                    .collect();
            }
            if !held.is_empty()
                && let Some(Block::Paragraph(p)) = blocks.get_mut(next)
            {
                p.inlines.splice(0..0, held);
            }
        }
        // A double gap became one break: its second line is removed with the rest.
        to_remove += blank_total.saturating_sub(to_break.len() as u32);
        // The space between two stanzas of verse typed with soft line breaks (I3 A14): an
        // empty line inside the paragraph, or a blank paragraph between two such. The
        // mapping cuts a paragraph at its soft breaks and drops what is empty, so the
        // stanzas run on; said so apart from the blank lines. Short lines each a paragraph
        // of their own are not counted: a character sheet or a list has exactly that shape,
        // and the report would claim a poem it cannot show.
        let written: Vec<usize> = (0..kinds.len())
            .filter(|i| kinds[*i] != LineKind::Blank)
            .collect();
        self.found.stanza_gaps += written
            .windows(2)
            .filter(|w| {
                w[1] > w[0] + 1 && kinds[w[0]] == LineKind::Verse && kinds[w[1]] == LineKind::Verse
            })
            .count() as u32;
        for (block, kind) in blocks.iter().zip(&kinds) {
            if let (Block::Paragraph(p), LineKind::Verse) = (block, kind) {
                self.found.stanza_gaps += gaps_between_lines(&soft_lines(p));
            }
        }
        self.found.blank_lines_breaks += to_break.len() as u32;
        self.found.blank_lines_removed += to_remove;
        for (i, kind) in kinds.iter().enumerate() {
            if *kind != LineKind::Ornament {
                continue;
            }
            if breaks.manuscript && breaks.ornaments {
                if let Some(Block::Paragraph(p)) = blocks.get_mut(i) {
                    p.inlines = vec![break_run()];
                }
                self.found.ornament_breaks += 1;
            } else {
                self.found.ornaments_kept += 1;
            }
        }
    }
}

/// Where the host's writing is, from any atom: answered by tables built once all the
/// annotations of a segment have left the host text, so that placing each costs nothing
/// however many share a paragraph.
///
/// Writing is a host character that is not blank, a host picture or note reference, or a
/// note reference or picture the rewrite inserts where the host shows it. A paragraph's
/// end that stays in the host text bounds the searches that stop at one.
struct Nearest {
    /// The nearest writing before an atom, within its paragraph.
    back: Vec<u32>,
    /// The nearest writing at or after an atom, within its paragraph.
    forward: Vec<u32>,
    /// The first host character or object of writing at or after an atom, in any paragraph.
    first: Vec<u32>,
    /// The first writing of the next paragraph holding any, after the one an atom is in.
    next: Vec<u32>,
    /// Some writing of the previous paragraph holding any, before the one an atom is in.
    previous: Vec<u32>,
}

/// The host's writing nearest each atom, in any paragraph, for trimming a range to its
/// words in one step whatever its length.
#[derive(Default)]
struct Edges {
    /// The first host writing at or after an atom.
    first: Vec<u32>,
    /// The last host writing before an atom.
    last: Vec<u32>,
}

impl Edges {
    fn of(seg: &Segment) -> Edges {
        let n = seg.atoms.len();
        let index = |i: usize| u32::try_from(i).unwrap_or(NOWHERE);
        let writing = |i: usize| seg.host(i) && seg.is_content(i);
        let mut first = vec![NOWHERE; n + 1];
        for i in (0..n).rev() {
            first[i] = if writing(i) { index(i) } else { first[i + 1] };
        }
        let mut last = vec![NOWHERE; n + 1];
        for i in 0..n {
            last[i + 1] = if writing(i) { index(i) } else { last[i] };
        }
        Edges { first, last }
    }

    /// The first host writing at or after `at`.
    fn first_at(&self, at: usize) -> Option<usize> {
        Nearest::look(&self.first, at)
    }

    /// The last host writing before `at`.
    fn last_before(&self, at: usize) -> Option<usize> {
        Nearest::look(&self.last, at)
    }
}

/// No atom: a segment never holds this many.
const NOWHERE: u32 = u32::MAX;

impl Nearest {
    fn of(seg: &Segment) -> Nearest {
        let n = seg.atoms.len();
        let index = |i: usize| u32::try_from(i).unwrap_or(NOWHERE);
        let host_end = |i: usize| seg.host(i) && matches!(seg.atoms[i], Atom::End(_));
        let host_content = |i: usize| seg.host(i) && seg.is_content(i);

        let mut back = vec![NOWHERE; n + 1];
        let mut previous_content = vec![NOWHERE; n + 1];
        let mut previous_end = vec![NOWHERE; n + 1];
        for i in 0..n {
            // Looking back, a host paragraph end is the edge, whatever the rewrite inserted
            // before it: that belongs to the paragraph it ends. Looking forward, the same
            // insertion is still inside the paragraph, so it is writing there.
            back[i + 1] = if host_end(i) {
                NOWHERE
            } else if seg.host_insert_is_content(i) || host_content(i) {
                index(i)
            } else {
                back[i]
            };
            previous_content[i + 1] = if host_content(i) {
                index(i)
            } else {
                previous_content[i]
            };
            previous_end[i + 1] = if host_end(i) {
                index(i)
            } else {
                previous_end[i]
            };
        }
        let mut forward = vec![NOWHERE; n + 1];
        let mut first = vec![NOWHERE; n + 1];
        let mut next_end = vec![NOWHERE; n + 1];
        for i in (0..n).rev() {
            forward[i] = if seg.host_insert_is_content(i) || host_content(i) {
                index(i)
            } else if host_end(i) {
                NOWHERE
            } else {
                forward[i + 1]
            };
            first[i] = if host_content(i) {
                index(i)
            } else {
                first[i + 1]
            };
            next_end[i] = if host_end(i) {
                index(i)
            } else {
                next_end[i + 1]
            };
        }
        let at = |table: &[u32], i: u32| -> u32 {
            usize::try_from(i)
                .ok()
                .and_then(|i| table.get(i).copied())
                .unwrap_or(NOWHERE)
        };
        let next: Vec<u32> = next_end
            .iter()
            .map(|&end| at(&first, end.saturating_add(1)))
            .collect();
        let previous: Vec<u32> = previous_end
            .iter()
            .map(|&end| at(&previous_content, end))
            .collect();
        Nearest {
            back,
            forward,
            first,
            next,
            previous,
        }
    }

    fn look(table: &[u32], from: usize) -> Option<usize> {
        table
            .get(from)
            .copied()
            .filter(|&i| i != NOWHERE)
            .and_then(|i| usize::try_from(i).ok())
    }

    /// The nearest writing before `from`, in its paragraph.
    fn back(&self, from: usize) -> Option<usize> {
        Nearest::look(&self.back, from)
    }

    /// The nearest writing from `from` on, in its paragraph.
    fn forward(&self, from: usize) -> Option<usize> {
        Nearest::look(&self.forward, from)
    }

    /// The first host writing from `from` on, in whichever paragraph.
    fn first_content(&self, from: usize) -> Option<usize> {
        Nearest::look(&self.first, from)
    }

    /// The first writing of the next paragraph holding any, after the one `from` is in.
    fn next_paragraph(&self, from: usize) -> Option<usize> {
        Nearest::look(&self.next, from)
    }

    /// Some writing of the previous paragraph holding any, before the one `from` is in.
    fn previous_paragraph(&self, from: usize) -> Option<usize> {
        Nearest::look(&self.previous, from)
    }
}

/// The word holding the host atom `at`: its first and last character atoms. A marker
/// inside a word (a character style that starts mid-word) does not end it.
fn word_around(seg: &Segment, at: usize) -> Option<(usize, usize)> {
    let letter = |i: usize| {
        seg.host(i) && matches!(seg.atoms[i], Atom::Char { .. }) && !seg.is_blank_char(i)
    };
    let invisible = |i: usize| seg.marker[i] && matches!(seg.atoms[i], Atom::Char { .. });
    if !letter(at) {
        return None;
    }
    let mut start = at;
    let mut probe = at;
    while probe > 0 {
        probe -= 1;
        if letter(probe) {
            start = probe;
        } else if !invisible(probe) {
            break;
        }
    }
    let mut end = at;
    let mut probe = at + 1;
    while probe < seg.atoms.len() {
        if letter(probe) {
            end = probe;
        } else if !invisible(probe) {
            break;
        }
        probe += 1;
    }
    Some((start, end))
}

/// What separates the end of one note from the start of the next.
enum Gap {
    None,
    /// Only paragraph breaks and blank space, one break at least.
    Breaks,
    Text,
}

fn gap(seg: &Segment, from: usize, to: usize) -> Gap {
    let mut breaks = false;
    let mut blank = false;
    for position in from..to {
        let atom = seg.at[position];
        if seg.marker[atom] {
            continue;
        }
        match seg.atoms[atom] {
            Atom::End(_) => breaks = true,
            Atom::Char { c, .. } if c.is_whitespace() => blank = true,
            _ => return Gap::Text,
        }
    }
    // Objects between the characters (a picture) also separate.
    let (first, last) = (
        seg.at.get(from).copied().unwrap_or(seg.atoms.len()),
        seg.at.get(to).copied().unwrap_or(seg.atoms.len()),
    );
    for i in first..last {
        if matches!(&seg.atoms[i], Atom::Inline(inline) if matches!(**inline, Inline::Picture(_) | Inline::NoteReference(_)))
        {
            return Gap::Text;
        }
    }
    match (breaks, blank) {
        (false, false) => Gap::None,
        (true, _) => Gap::Breaks,
        (false, true) => Gap::Text,
    }
}

/// The atoms a note's body spans, as paragraphs: its breaks its own, the insertions that
/// fall inside it its own.
fn body_blocks(seg: &Segment, start: usize, end: usize) -> Vec<Paragraph> {
    let mut out: Vec<Paragraph> = Vec::new();
    let mut builder = Builder::default();
    for i in start..=end.min(seg.atoms.len()) {
        if let Some(list) = seg.inserts.get(&i) {
            for (inline, owner) in list {
                if *owner == Owner::Any {
                    builder.inline(inline.clone());
                }
            }
        }
        if i >= end || !seg.in_span[i] || seg.marker[i] {
            continue;
        }
        match &seg.atoms[i] {
            Atom::Char { c, run } => builder.char(*c, *run, seg.code[i]),
            Atom::Inline(inline) => builder.inline((**inline).clone()),
            Atom::End(meta) => {
                let inlines = builder.finish(seg, None);
                out.push(Paragraph {
                    props: seg
                        .metas
                        .get(*meta as usize)
                        .map(|m| m.props)
                        .unwrap_or_default(),
                    inlines,
                    ..Paragraph::default()
                });
            }
        }
    }
    let rest = builder.finish(seg, None);
    out.push(Paragraph {
        inlines: rest,
        ..Paragraph::default()
    });
    out.retain(|p| !p.inlines.is_empty());
    out
}

/// One piece of a paragraph being rebuilt: characters of one run, or an object.
enum Piece {
    Text { run: u32, code: bool, text: String },
    Inline(Inline),
}

/// Collects characters into runs and objects between them.
#[derive(Default)]
struct Builder {
    pieces: Vec<Piece>,
}

impl Builder {
    fn char(&mut self, c: char, run: u32, code: bool) {
        match self.pieces.last_mut() {
            Some(Piece::Text {
                run: last,
                code: last_code,
                text,
            }) if *last == run && *last_code == code => text.push(c),
            _ => self.pieces.push(Piece::Text {
                run,
                code,
                text: c.to_string(),
            }),
        }
    }

    fn inline(&mut self, inline: Inline) {
        self.pieces.push(Piece::Inline(inline));
    }

    /// Whether what was collected holds any writing: a character that is not blank, a
    /// picture, a note's reference.
    fn holds_writing(&self) -> bool {
        self.pieces.iter().any(|piece| match piece {
            Piece::Text { text, .. } => text.chars().any(|c| !c.is_whitespace() && c != '\u{FFFC}'),
            Piece::Inline(inline) => {
                matches!(inline, Inline::Picture(_) | Inline::NoteReference(_))
            }
        })
    }

    /// The paragraph's inlines, each run given its formatting, its group and its field.
    fn finish(&mut self, seg: &Segment, code_font: Option<i32>) -> Vec<Inline> {
        std::mem::take(&mut self.pieces)
            .into_iter()
            .map(|piece| match piece {
                Piece::Inline(inline) => inline,
                Piece::Text { run, code, text } => {
                    let (mut props, group, field) = match seg.runs.get(run as usize) {
                        Some(meta) => (
                            meta.props,
                            meta.group,
                            meta.field.filter(|f| !seg.unlinked.contains(f)),
                        ),
                        None => (CharProps::default(), 0, None),
                    };
                    if code && let Some(font) = code_font {
                        props.font = Some(font);
                    }
                    Inline::Text(Run {
                        text,
                        props,
                        group,
                        field,
                    })
                }
            })
            .collect()
    }
}

/// Mark a token's characters as markers.
fn mark(seg: &mut Segment, token: &Token) {
    mark_chars(seg, token.start, token.end);
}

fn mark_chars(seg: &mut Segment, start: usize, end: usize) {
    for position in start..end.min(seg.text.len()) {
        let atom = seg.at[position];
        if matches!(seg.atoms[atom], Atom::Char { .. }) {
            seg.marker[atom] = true;
        }
    }
}

/// The stronger of two meanings a paragraph's styles give it.
fn stronger(a: ParaMeaning, b: ParaMeaning) -> ParaMeaning {
    let rank = |m: ParaMeaning| match m {
        ParaMeaning::Heading(_) => 3,
        ParaMeaning::CodeBlock => 2,
        ParaMeaning::Quote => 1,
        ParaMeaning::None => 0,
    };
    if rank(b) > rank(a) { b } else { a }
}

/// The atoms a range `start..end` of a reading's units covers: the first atom at `start`,
/// and the first atom at `end`, from the units before each atom (`before` holds one more
/// entry than there are atoms: the units at the segment's end). `None` when either end falls
/// inside a character's units, or outside the segment.
fn atoms_for(before: &[u64], start: u64, end: u64) -> Option<(usize, usize)> {
    let first = before.partition_point(|u| *u < start);
    if before.get(first) != Some(&start) {
        return None;
    }
    let last = before.partition_point(|u| *u < end);
    if before.get(last) != Some(&end) || last <= first {
        return None;
    }
    Some((first, last))
}

/// Whether the atoms `start..end` start and end on word boundaries.
fn on_word_boundaries(seg: &Segment, start: usize, end: usize) -> bool {
    let word_char =
        |i: usize| matches!(seg.atoms.get(i), Some(Atom::Char { c, .. }) if c.is_alphanumeric());
    let starts = start == 0 || !word_char(start) || !word_char(start - 1);
    let ends = end >= seg.atoms.len() || !word_char(end) || end == 0 || !word_char(end - 1);
    starts && ends
}

/// Renumber a grafted body's fields and pictures, and drop the fonts and colours that name
/// its own tables.
fn renumber(blocks: &mut [Block], field_base: usize, picture_base: usize) {
    let mut stack: Vec<&mut [Block]> = vec![blocks];
    while let Some(list) = stack.pop() {
        for block in list {
            match block {
                Block::Paragraph(p) => {
                    for inline in &mut p.inlines {
                        match inline {
                            Inline::Text(run) => {
                                run.field = run.field.map(|f| f + field_base);
                                run.props.font = None;
                                run.props.color = None;
                                run.props.background = None;
                            }
                            Inline::Picture(id) => *id += picture_base,
                            Inline::FieldStart(id) | Inline::FieldEnd(id) => *id += field_base,
                            _ => {}
                        }
                    }
                }
                Block::Table(table) => {
                    for row in &mut table.rows {
                        for cell in &mut row.cells {
                            stack.push(&mut cell.blocks);
                        }
                    }
                }
            }
        }
    }
}

/// A picture link's target as one of the project's own items: `$PROJECT://<UUID>.<ext>`.
fn project_image(target: &str) -> Option<String> {
    let rest = target.trim().strip_prefix("$PROJECT://")?;
    let uuid = rest.split('.').next()?.trim();
    (uuid.len() == 36).then(|| uuid.to_ascii_uppercase())
}

/// The last segment of a path from either platform: nothing of the writer's folders.
fn file_name_of(target: &str) -> String {
    target
        .trim()
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or_default()
        .to_string()
}

/// A paragraph's text, as the rest of the pass reads it.
fn plain_text(paragraph: &Paragraph) -> String {
    let mut out = String::new();
    for inline in &paragraph.inlines {
        match inline {
            Inline::Text(run) => out.push_str(&run.text),
            Inline::Picture(_) | Inline::NoteReference(_) => out.push('\u{FFFC}'),
            _ => {}
        }
    }
    out
}

/// What a top-level paragraph is, for the scene-break rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineKind {
    Blank,
    /// Prose long enough to stand on either side of a scene break.
    Prose,
    /// A short line of words: a line of verse, a heading, a list's item.
    Short,
    /// Short lines of words a soft line break separates: a stanza. Never beside a scene
    /// break, since the blank line after a stanza is the poem's own.
    Verse,
    /// A centred line of glyphs that are not words, and not a break Skribisto knows.
    Ornament,
    Other,
}

fn line_kind(paragraph: &Paragraph, text: &str) -> LineKind {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        let objects = paragraph
            .inlines
            .iter()
            .any(|i| matches!(i, Inline::Picture(_) | Inline::NoteReference(_)));
        return if objects {
            LineKind::Other
        } else {
            LineKind::Blank
        };
    }
    let chars = trimmed.chars().count();
    if paragraph.props.alignment == Some(Alignment::Center)
        && chars <= ORNAMENT_CHARS
        && !trimmed.chars().any(char::is_alphanumeric)
        && scene_break::tier_of_plain_line(trimmed).is_none()
        // A lone ellipsis is a beat of silence, never a break.
        && trimmed != "…"
        && trimmed != "..."
    {
        return LineKind::Ornament;
    }
    if is_verse(&soft_lines(paragraph)) {
        return LineKind::Verse;
    }
    let letters = trimmed
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .count();
    if chars >= SCENE_PROSE_CHARS && letters * 10 >= chars * 8 {
        LineKind::Prose
    } else if trimmed.chars().any(char::is_alphabetic) {
        LineKind::Short
    } else {
        LineKind::Other
    }
}

/// Whether a paragraph's lines are a stanza's: more than one line of words, each short.
fn is_verse(lines: &[String]) -> bool {
    let written: Vec<&String> = lines.iter().filter(|l| !l.is_empty()).collect();
    written.len() > 1
        && written
            .iter()
            .all(|l| l.chars().count() < SCENE_PROSE_CHARS)
        && written
            .iter()
            .filter(|l| l.chars().any(char::is_alphabetic))
            .count()
            > 1
}

/// The empty lines between two written ones: where a poem typed with soft line breaks
/// leaves the space between its stanzas. Each run of them counts once.
fn gaps_between_lines(lines: &[String]) -> u32 {
    let mut gaps = 0;
    let mut written_before = false;
    let mut pending = false;
    for line in lines {
        if line.is_empty() {
            pending |= written_before;
        } else {
            if pending {
                gaps += 1;
                pending = false;
            }
            written_before = true;
        }
    }
    gaps
}

/// A paragraph's lines, cut at its soft line breaks, each trimmed.
fn soft_lines(paragraph: &Paragraph) -> Vec<String> {
    let mut lines = vec![String::new()];
    for inline in &paragraph.inlines {
        match inline {
            Inline::Break(BreakKind::Line) => lines.push(String::new()),
            Inline::Text(run) => {
                for c in run.text.chars() {
                    if matches!(c, '\u{2028}' | '\u{2029}') {
                        lines.push(String::new());
                    } else if let Some(line) = lines.last_mut() {
                        line.push(c);
                    }
                }
            }
            _ => {}
        }
    }
    lines.into_iter().map(|l| l.trim().to_string()).collect()
}

/// A run holding the canonical minor scene break, which the mapping reads as a break.
fn break_run() -> Inline {
    Inline::Text(Run {
        text: scene_break::canonical_plain(SceneBreakTier::Minor).to_string(),
        props: CharProps::default(),
        group: 0,
        field: None,
    })
}

/// Whether a paragraph reads as Markdown typed as text (A7): a heading's hashes, strong
/// emphasis between double marks, a link written as `[text](url)`, a fenced code line.
fn reads_as_markdown(text: &str) -> bool {
    let trimmed = text.trim_start();
    // A line of hashes alone (`# # #`) is a scene break, not a heading: a heading has words.
    if scene_break::tier_of_plain_line(trimmed.trim()).is_some() {
        return false;
    }
    let heading = {
        let hashes = trimmed.chars().take_while(|c| *c == '#').count();
        (1..=6).contains(&hashes)
            && trimmed[hashes..].starts_with(' ')
            && trimmed[hashes..].chars().any(char::is_alphabetic)
    };
    heading
        || trimmed.starts_with("```")
        || (trimmed.contains("](") && trimmed.contains('['))
        || has_double_marks(trimmed, "**")
        || has_double_marks(trimmed, "__")
}

fn has_double_marks(text: &str, mark: &str) -> bool {
    let Some(open) = text.find(mark) else {
        return false;
    };
    let rest = &text[open + mark.len()..];
    rest.find(mark).is_some_and(|close| close > 0)
}

/// Whether a paragraph holds a citation key for a bibliography tool (`[@key]`, `[-@key]`).
fn holds_citation_key(text: &str) -> bool {
    text.match_indices("[@")
        .chain(text.match_indices("[-@"))
        .any(|(at, found)| {
            text[at + found.len()..]
                .chars()
                .next()
                .is_some_and(|c| c.is_alphanumeric() || c == '_')
        })
}

/// The name of a typed signature line, "Name, date" with an optional time: `Pradel Ianto,
/// 8/24/16, 3:55 PM` or `TW, 11/09/2025 16:52`. Nothing else is read from it: the date's
/// order (day or month first) is the writer's locale, which the file does not say.
fn signed_name(line: &str) -> Option<String> {
    let line = line.trim();
    let (name, rest) = line.split_once(", ")?;
    let name = name.trim();
    if name.is_empty()
        || name.chars().count() > 64
        || name
            .chars()
            .any(|c| c.is_ascii_digit() || matches!(c, '/' | ':'))
    {
        return None;
    }
    let rest = rest.trim();
    let (date, time) = match rest.split_once([',', ' ']) {
        Some((date, time)) => (date.trim(), Some(time.trim())),
        None => (rest, None),
    };
    let date_parts: Vec<&str> = date.split(['/', '.', '-']).collect();
    let is_date = date_parts.len() == 3
        && date_parts
            .iter()
            .all(|p| !p.is_empty() && p.len() <= 4 && p.chars().all(|c| c.is_ascii_digit()));
    if !is_date {
        return None;
    }
    if let Some(time) = time {
        let clock = time
            .strip_suffix("AM")
            .or_else(|| time.strip_suffix("PM"))
            .or_else(|| time.strip_suffix("am"))
            .or_else(|| time.strip_suffix("pm"))
            .unwrap_or(time)
            .trim();
        let parts: Vec<&str> = clock.split(':').collect();
        let is_time = (2..=3).contains(&parts.len())
            && parts
                .iter()
                .all(|p| !p.is_empty() && p.len() <= 2 && p.chars().all(|c| c.is_ascii_digit()));
        if !is_time {
            return None;
        }
    }
    Some(name.to_string())
}

/// Text folded for comparing a heading with a title.
fn fold(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

// ── the tokens ──────────────────────────────────────────────────────────────────────

/// The longest a placeholder or a picture link may run before it is taken as text.
const MAX_TOKEN_CHARS: usize = 4096;

/// Every token in a segment's characters, in order. None crosses a paragraph end.
fn scan(text: &[char]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut i = 0usize;
    while i < text.len() {
        let found = match text[i] {
            '{' => brace_token(text, i),
            '\\' => end_token(text, i),
            '<' => angle_token(text, i),
            _ => None,
        };
        match found {
            Some((kind, len)) => {
                tokens.push(Token {
                    kind,
                    start: i,
                    end: i + len,
                });
                i += len;
            }
            None => i += 1,
        }
    }
    tokens
}

fn starts(text: &[char], at: usize, pattern: &str) -> bool {
    (at..)
        .zip(pattern.chars())
        .all(|(i, p)| text.get(i) == Some(&p))
}

fn brace_token(text: &[char], i: usize) -> Option<(TokenKind, usize)> {
    if starts(text, i, "{\\Scrv_fn=") {
        return Some((TokenKind::Open(NoteKind2::Footnote), 10));
    }
    if starts(text, i, "{\\Scrv_ps=") {
        return Some((TokenKind::Open(NoteKind2::Preserve), 10));
    }
    if starts(text, i, "{\\Scrv_annot") {
        let mut j = i + "{\\Scrv_annot".len();
        let mut colour = None;
        let skip = |mut j: usize| {
            while text.get(j) == Some(&' ') {
                j += 1;
            }
            j
        };
        j = skip(j);
        if starts(text, j, "\\color={") {
            let open = j + "\\color={".len();
            if let Some(close) = (open..(open + 64).min(text.len()))
                .find(|&k| text[k] == '}' || text[k] == '\n')
                .filter(|&k| text[k] == '}')
            {
                colour = Some(text[open..close].iter().collect::<String>());
                j = skip(close + 1);
            }
        }
        if starts(text, j, "\\text=") {
            j += "\\text=".len();
        }
        return Some((TokenKind::Open(NoteKind2::Annotation(colour)), j - i));
    }
    if starts(text, i, "{\\Scrv_") {
        return Some((TokenKind::Unknown, "{\\Scrv_".len()));
    }
    if starts(text, i, "{$SCRImageLink") {
        let limit = (i + MAX_TOKEN_CHARS).min(text.len());
        let close = (i..limit).find(|&k| text[k] == '}' || text[k] == '\n')?;
        if text[close] != '}' {
            return None;
        }
        let inner: String = text[i + "{$SCRImageLink".len()..close].iter().collect();
        let (size, target) = match inner.split_once('=') {
            Some((size, target)) => (size, target),
            None => ("", inner.as_str()),
        };
        let number = |key: &str| {
            size.split(['[', ']', ';'])
                .find_map(|part| part.trim().strip_prefix(key))
                .and_then(|v| v.trim().parse::<u32>().ok())
        };
        return Some((
            TokenKind::ImageLink {
                width: number("w:"),
                height: number("h:"),
                target: target.to_string(),
            },
            close + 1 - i,
        ));
    }
    None
}

fn end_token(text: &[char], i: usize) -> Option<(TokenKind, usize)> {
    for (pattern, kind) in [
        ("\\end_Scrv_fn}", NoteKind2::Footnote),
        ("\\end_Scrv_annot}", NoteKind2::Annotation(None)),
        ("\\end_Scrv_ps}", NoteKind2::Preserve),
    ] {
        if starts(text, i, pattern) {
            return Some((TokenKind::Close(kind), pattern.chars().count()));
        }
    }
    if starts(text, i, "\\end_Scrv_") {
        return Some((TokenKind::Unknown, "\\end_Scrv_".len()));
    }
    None
}

fn angle_token(text: &[char], i: usize) -> Option<(TokenKind, usize)> {
    let closing = text.get(i + 1) == Some(&'!');
    let dollar = i + 1 + usize::from(closing);
    if text.get(dollar) != Some(&'$') {
        return None;
    }
    // Scrivener's own markers.
    if starts(text, dollar, "$Scr") {
        let limit = (dollar + 48).min(text.len());
        let close = (dollar..limit).find(|&k| text[k] == '>' || text[k] == '\n')?;
        if text[close] != '>' {
            return None;
        }
        let name: String = text[dollar + 1..close].iter().collect();
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':')
        {
            return None;
        }
        let len = close + 1 - i;
        let family = [
            ("Scr_Ps::", Family::Paragraph),
            ("Scr_Cs::", Family::Character),
            ("Scr_H::", Family::Heading),
        ]
        .into_iter()
        .find_map(|(prefix, family)| name.strip_prefix(prefix).map(|n| (family, n)));
        return Some(match family {
            Some((family, n)) => match n.parse::<u32>() {
                Ok(n) if closing => (TokenKind::StyleClose(family, n), len),
                Ok(n) => (TokenKind::StyleOpen(family, n), len),
                Err(_) => (TokenKind::ScrMarker, len),
            },
            None => (TokenKind::ScrMarker, len),
        });
    }
    if closing {
        return None;
    }
    // A compile placeholder: `<$n>`, `<$projecttitle>`, `<$custom:Field>`.
    if !text.get(dollar + 1).is_some_and(|c| c.is_alphabetic()) {
        return None;
    }
    let limit = (dollar + 256).min(text.len());
    let close = (dollar..limit).find(|&k| text[k] == '>' || text[k] == '\n' || text[k] == '<')?;
    if text[close] != '>' {
        return None;
    }
    // Kept as the project writes it, case and argument included: `<$R>` is not `<$r>`,
    // and the report quotes what the writer can search for. A placeholder is never
    // removed, so its whole text stays.
    let written: String = text[i..=close].iter().collect();
    Some((TokenKind::Placeholder(written), close + 1 - i))
}

#[cfg(test)]
mod tests;
