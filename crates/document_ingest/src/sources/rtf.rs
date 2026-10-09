// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! RTF, as `rtf_reader` reads it, mapped onto [`rich`]'s model.
//!
//! `rtf_reader` knows RTF and nothing of Skribisto; [`rich`] knows what the editor keeps and
//! nothing of RTF. [`to_rich`] is where the two meet, so the Djot a paragraph becomes is
//! written by the same emitter, and proved by the same parser, as a Word or an OpenDocument
//! paragraph's.
//!
//! It is generic RTF. An application's own conventions written *as text* inside RTF
//! (Scrivener's inline footnotes, its style markers) are not read here: a layer that knows
//! them rewrites the reader's model first (the model is plain data, and the reader's crate
//! documentation says what such a layer is given), then maps the result with this.
//!
//! ## What is carried
//!
//! * Paragraphs, with centred or flush-right alignment, a right-to-left direction and a page
//!   break before them (`\pagebb`, or a `\page` in the text before them).
//! * Headings: a paragraph's outline level, else its style's (following `\sbasedon`), else a
//!   style named `heading N`. Level 0 is heading 1. They open rows, or stay in the prose
//!   when the caller asks ([`RtfOptions::headings_in_prose`]).
//! * List items, ordered or not from the list table (`\levelnfc`, macOS's `\levelmarker`),
//!   Word 95's `\pn`, or the label the writer stored; nested by `\ilvl`. A list level that
//!   numbers nothing (`\levelnfc255`) is an indent, not a list.
//! * Quotations, epigraphs and code blocks from their style's name, as [`rich::styled_as`]
//!   reads a DOCX or ODT style.
//! * Bold, italic, underline, strike-through, superscript and subscript. A font's family is
//!   never read as code: Scrivener for Windows declares every font `\fmodern`. Code is what
//!   the caller says it is ([`RtfOptions::code_fonts`]).
//! * Links to web and mail addresses. The text of any other hyperlink is kept, the link not.
//! * Footnotes and endnotes, both as footnotes; comments with their range
//!   (`\atrfstart`/`\atrfend`) or on their paragraph, their replies threaded.
//! * Tables, a cell's paragraphs joined by spaces. Pictures, brought in as a Word document's
//!   are ([`crate::pictures`]) and shown at the size the file shows them; see
//!   [`RtfOptions::picture_source`].
//! * Tabs and every other character, kept as the file has them. A line break
//!   (`\line`, `\column`, U+2028) ends a paragraph and the rest starts a new one of the same
//!   kind, as the DOCX scanner does, since Djot keeps no line break inside a paragraph.
//!
//! ## What is reported
//!
//! In the document-import vocabulary, where it has a word: pictures left out, with why
//! ([`ImportDiagnostic::ImageNotIngested`]: a format the editor cannot show, one past the
//! reader's limits, one inside a note or a comment), fields other than links, shown as their last
//! result ([`ImportDiagnostic::FieldFlattened`]), text boxes and objects with nothing to show
//! ([`ImportDiagnostic::TextBoxDropped`], [`ImportDiagnostic::EmbeddedObjectDropped`]),
//! notes with no text or cited from inside another note
//! ([`ImportDiagnostic::FootnoteNotCarried`]), replies whose comment is not in the file
//! ([`ImportDiagnostic::CommentRepliesFlattened`]).
//!
//! Everything else the rich model has no place for is counted in [`NotCarried`]: hidden
//! text, highlighting, text colour, capitals, links kept as text, stray object replacement
//! characters, joined cells, nested tables, endnotes, headers and footers, what the reader
//! could not decode, groups it did not know, and what followed the document's end. No word
//! for these exists in the document-import vocabulary yet, because no scanner met them
//! before: the Scrivener importer reports them in its own, and a scanner registering `.rtf`
//! would add them to `ImportDiagnostic` with their sentences.
//!
//! Layout is neither carried nor reported, as for every other format: fonts and sizes,
//! indents, spacing, tab stops, justification and sections are the export style's to set.

use std::collections::{HashMap, HashSet};

use rtf_reader::{
    Alignment as RtfAlignment, Block, BreakKind, CharProps, Diagnostic, Direction as RtfDirection,
    Document, FieldKind, Inline, List, ListLevel, ListRef, NoteKind, Paragraph, Picture,
    PictureFormat, Style, StyleKind, VerticalPosition, paragraphs,
};

use super::rich::{
    self, Alignment, AnnotationEnd, BlockProps, Direction, FOOTNOTE_LABEL_PREFIX, ParagraphKind,
    RichAnnotation, RichBlock, RichDocument, RichFootnote, RichReply, Run, RunStyle,
};
use crate::diagnostics::ImportDiagnostic;
use crate::pictures::{DocumentPictures, IngestedPicture, PictureRefusal, PlacedPicture};

/// A picture a caller brought in for the prose, and whether only its first frame came.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcedPicture {
    pub placed: PlacedPicture,
    /// It was an animation, and only its first frame was kept: reported
    /// ([`ImportDiagnostic::PictureFirstFrameOnly`]).
    pub first_frame_only: bool,
}

/// How a caller that keeps pictures itself brings one in: given its index in
/// [`Document::pictures`] and the picture, where the prose names it, or why it stays out.
///
/// A caller importing several documents into one project passes one, so that a picture
/// shown in two documents is one picture there, and one budget bounds them all.
pub type PictureSource<'a> = &'a dyn Fn(usize, &Picture) -> Result<SourcedPicture, PictureRefusal>;

/// The label each footnote is given, one call per footnote, in the order the text cites
/// them.
pub type FootnoteLabels<'a> = &'a dyn Fn() -> String;

/// How to map one document.
#[derive(Clone, Copy)]
pub struct RtfOptions<'a> {
    /// The document's origin, named in every diagnostic.
    pub origin: &'a str,
    /// Where each picture is kept, when the caller keeps them itself.
    ///
    /// With none, the mapping brings each picture in on its own (see [`crate::pictures`]),
    /// hands them back in [`RtfConversion::pictures`], and names them in the prose by the
    /// name they will have in a project.
    pub picture_source: Option<PictureSource<'a>>,
    /// Headings stay in the prose they are written in ([`ParagraphKind::InlineHeading`])
    /// rather than opening rows: for a caller whose documents are already rows of their
    /// own, like a Scrivener project's.
    pub headings_in_prose: bool,
    /// Fonts whose text is code, by their number in the font table. A font's family never
    /// says so (see the module documentation); a caller that knows better from elsewhere,
    /// such as a character style it resolved, says so here.
    pub code_fonts: &'a [i32],
    /// The label of each footnote. With none, each document numbers its own
    /// ([`FOOTNOTE_LABEL_PREFIX`] and a count), which a caller rewrites; a caller that
    /// owns the whole project mints the final labels here instead.
    pub footnote_labels: Option<FootnoteLabels<'a>>,
}

impl<'a> RtfOptions<'a> {
    pub fn new(origin: &'a str) -> Self {
        RtfOptions {
            origin,
            picture_source: None,
            headings_in_prose: false,
            code_fonts: &[],
            footnote_labels: None,
        }
    }
}

/// A document mapped, and what the mapping says about it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RtfConversion {
    pub document: RichDocument,
    /// What the document-import vocabulary can name.
    pub diagnostics: Vec<ImportDiagnostic>,
    /// What it cannot, counted.
    pub not_carried: NotCarried,
    /// The pictures the mapping brought in itself, each once, when the caller gave no
    /// [`RtfOptions::picture_source`]: what a project holding this prose must store.
    pub pictures: Vec<IngestedPicture>,
    /// For each block of [`Self::document`], the index of the block of the RTF body it came
    /// from: a paragraph's lines each make a block of their own, so a caller that splits
    /// the text where the RTF's paragraphs are finds its way back here.
    pub block_origins: Vec<usize>,
}

/// Formatting and content the rich model has no place for, counted by kind. See the module
/// documentation for why these are not [`ImportDiagnostic`]s.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NotCarried {
    /// Characters of hidden text (`\v`), left out.
    pub hidden_characters: usize,
    /// Runs on a coloured background (`\highlight`, `\cb`), carried without it. A white
    /// background is none.
    pub highlighted_runs: usize,
    /// Runs in a colour other than black or the automatic one, carried without it.
    pub coloured_runs: usize,
    /// Runs in small capitals, carried as their letters.
    pub small_caps_runs: usize,
    /// Runs shown in capitals whatever their letters (`\caps`), carried as the letters
    /// stored, which is not what the writer saw.
    pub all_caps_runs: usize,
    /// Hyperlinks to anything but a web or mail address (a link into the writing
    /// application, a bookmark, a file): their text is kept, the link is not.
    pub links_kept_as_text: usize,
    /// Object replacement characters (U+FFFC) standing alone in the text, removed: a
    /// picture is an item of its own here, and the character marks nothing.
    pub object_replacement_characters: usize,
    /// Table cells holding more than one paragraph, their paragraphs joined by spaces.
    pub joined_cells: usize,
    /// Tables nested in a cell, written into the cell as its text.
    pub nested_tables: usize,
    /// Endnotes, brought over as footnotes.
    pub endnotes: usize,
    /// Comments whose date the file states: RTF does not say how to read it, so none is
    /// guessed.
    pub comment_dates: usize,
    /// Headers and footers, which are not part of the document's flow.
    pub headers_and_footers: usize,
    /// Characters standing for a page number, a date or a time where the document is shown.
    pub computed_characters: usize,
    /// Characters the reader could not decode, carried as U+FFFD.
    pub undecodable_characters: usize,
    /// Control characters the reader removed from the text.
    pub control_characters: usize,
    /// Groups the file marks as skippable (`{\*\...}`) that the reader does not know, left
    /// out as RTF asks: whatever they held is not in the import.
    pub unread_destinations: usize,
    /// Pieces of the file after the document's own group closed, left out: a stray closing
    /// brace ends a document early, and the text after it with it.
    pub after_document: usize,
    /// Pictures a list label shows (a picture bullet): the label is not kept, and its
    /// picture goes with it.
    pub label_pictures: usize,
}

impl NotCarried {
    pub fn is_empty(&self) -> bool {
        *self == NotCarried::default()
    }
}

/// Map a document read by `rtf_reader` onto the rich model.
pub fn to_rich(doc: &Document, options: &RtfOptions<'_>) -> RtfConversion {
    let mut mapper = Mapper::new(doc, options);
    let mut origins: Vec<usize> = Vec::with_capacity(doc.body.len());
    for (index, block) in doc.body.iter().enumerate() {
        let before = mapper.blocks.len();
        match block {
            Block::Paragraph(paragraph) => mapper.paragraph(paragraph),
            Block::Table(table) => mapper.table(table),
        }
        origins.extend(std::iter::repeat_n(index, mapper.blocks.len() - before));
    }
    let annotations = mapper.annotations();
    let mut conversion = mapper.finish(annotations);
    conversion.block_origins = origins;
    conversion
}

/// A field's result, as a link or not.
#[derive(Debug, Clone, PartialEq, Eq)]
enum FieldLink {
    /// A hyperlink to an address a manuscript keeps.
    Web(String),
    /// A link to anything else (a link into an application, a bookmark, a file): its text
    /// is kept, the link is not.
    Other,
    /// Not a link at all.
    Text,
}

/// Where text is being mapped: the body may cite notes, a note's or a comment's own text
/// may not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Context {
    Body,
    Inner,
}

/// A stretch of one paragraph between two line breaks.
#[derive(Debug, Default)]
struct Piece {
    runs: Vec<Run>,
    /// Its length in the rich model's plain text: characters, and one for a picture or a
    /// note's reference.
    len: usize,
    page_break_before: bool,
}

impl Piece {
    fn is_blank(&self) -> bool {
        self.runs
            .iter()
            .all(|run| run.image.is_none() && run.footnote.is_none() && run.text.trim().is_empty())
    }
}

/// A place a comment points at, found in a paragraph.
#[derive(Debug, Clone, PartialEq, Eq)]
enum MarkerKind {
    Annotation(usize),
    RangeStart(String),
    RangeEnd(String),
}

#[derive(Debug, Clone)]
struct Marker {
    kind: MarkerKind,
    piece: usize,
    offset: usize,
}

/// A position in the rich document: a block and a character offset in its plain text.
type Position = (usize, usize);

struct Mapper<'d, 'o> {
    doc: &'d Document,
    options: &'o RtfOptions<'o>,
    links: Vec<FieldLink>,
    /// Paragraph styles by number, the last definition of a number winning, as
    /// [`Document::paragraph_style`] finds them. Looked up once per paragraph and per step of
    /// a style's `\sbasedon` chain, where a walk over the whole style sheet each time would
    /// make a long document with a long style sheet quadratic.
    styles: HashMap<i32, &'d Style>,
    /// Which list each `\lsN` names, and each list by its id, the last definition winning,
    /// as [`Document::list_level`] finds them, for the same reason.
    list_overrides: HashMap<i32, i32>,
    lists: HashMap<i32, &'d List>,
    blocks: Vec<RichBlock>,
    footnotes: Vec<RichFootnote>,
    annotation_at: HashMap<usize, Position>,
    range_starts: HashMap<String, Position>,
    range_ends: HashMap<String, Position>,
    pictures_shown: HashSet<usize>,
    /// Links kept as their words, counted once each.
    links_counted: HashSet<usize>,
    /// The pictures brought in when the caller keeps none itself.
    pictures: DocumentPictures,
    notes_not_carried: usize,
    replies_flattened: usize,
    pending_page_break: bool,
    diagnostics: Vec<ImportDiagnostic>,
    not_carried: NotCarried,
}

impl<'d, 'o> Mapper<'d, 'o> {
    fn new(doc: &'d Document, options: &'o RtfOptions<'o>) -> Self {
        let not_carried = NotCarried::default();
        let links = doc
            .fields
            .iter()
            .map(|field| match field.kind() {
                FieldKind::Hyperlink {
                    target: Some(target),
                    fragment,
                } if is_web_address(&target) => FieldLink::Web(match fragment {
                    Some(fragment) => format!("{target}#{fragment}"),
                    None => target,
                }),
                FieldKind::Hyperlink { .. } => FieldLink::Other,
                FieldKind::Other { .. } => FieldLink::Text,
            })
            .collect();
        let mut styles = HashMap::new();
        for style in doc.styles.iter().filter(|s| s.kind == StyleKind::Paragraph) {
            styles.insert(style.number, style);
        }
        let list_overrides = doc
            .list_overrides
            .iter()
            .map(|o| (o.number, o.list_id))
            .collect();
        let lists = doc.lists.iter().map(|l| (l.id, l)).collect();
        Mapper {
            doc,
            options,
            links,
            styles,
            list_overrides,
            lists,
            blocks: Vec::new(),
            footnotes: Vec::new(),
            annotation_at: HashMap::new(),
            range_starts: HashMap::new(),
            range_ends: HashMap::new(),
            pictures_shown: HashSet::new(),
            links_counted: HashSet::new(),
            pictures: DocumentPictures::new(),
            notes_not_carried: 0,
            replies_flattened: 0,
            pending_page_break: false,
            diagnostics: Vec::new(),
            not_carried,
        }
    }

    // ── paragraphs ───────────────────────────────────────────────────────────

    fn paragraph(&mut self, paragraph: &Paragraph) {
        let kind = self.kind_of(paragraph);
        let props = props_of(paragraph);
        let (pieces, markers) = self.pieces_of(paragraph, Context::Body);
        let first = self.blocks.len();
        for (index, piece) in pieces.into_iter().enumerate() {
            let mut block_props = props;
            block_props.page_break_before = if index == 0 {
                props.page_break_before || std::mem::take(&mut self.pending_page_break)
            } else {
                piece.page_break_before
            };
            // A blank paragraph produces no block downstream, so a page break it carries
            // goes to the next paragraph that does.
            if piece.is_blank() && block_props.page_break_before {
                self.pending_page_break = true;
                block_props.page_break_before = false;
            }
            self.blocks.push(RichBlock::Paragraph {
                kind,
                runs: piece.runs,
                props: block_props,
            });
        }
        for marker in markers {
            self.record(marker.kind, (first + marker.piece, marker.offset));
        }
    }

    fn kind_of(&self, paragraph: &Paragraph) -> ParagraphKind {
        if let Some(level) = self.heading_level(paragraph) {
            return if self.options.headings_in_prose {
                ParagraphKind::InlineHeading {
                    level: level.clamp(1, 6),
                }
            } else {
                ParagraphKind::Heading { level }
            };
        }
        if let Some(item) = self.list_item(paragraph) {
            return item;
        }
        self.style_name(paragraph)
            .and_then(rich::styled_as)
            .map_or(ParagraphKind::Body, rich::kind_for_style)
    }

    fn style_name(&self, paragraph: &Paragraph) -> Option<&'d str> {
        let number = paragraph.props.style?;
        self.styles.get(&number).map(|style| style.name.as_str())
    }

    /// The list level a paragraph's `\lsN\ilvlM` names, as [`Document::list_level`] reads
    /// it.
    fn list_level(&self, list: ListRef) -> Option<&'d ListLevel> {
        let id = self.list_overrides.get(&list.override_number)?;
        self.lists.get(id)?.levels.get(usize::from(list.level))
    }

    /// The heading level, from 1: the paragraph's outline level, its style's (through the
    /// styles it is based on), or a style named `heading N`.
    fn heading_level(&self, paragraph: &Paragraph) -> Option<u8> {
        if let Some(outline) = paragraph.props.outline_level {
            return Some(outline.saturating_add(1));
        }
        let mut number = paragraph.props.style?;
        // A style sheet can base a style on itself or loop; a few steps reach any real
        // chain.
        for _ in 0..16 {
            let style = *self.styles.get(&number)?;
            if let Some(outline) = style.outline_level {
                return Some(outline.saturating_add(1));
            }
            if let Some(level) = heading_name_level(&style.name) {
                return Some(level);
            }
            number = style.based_on?;
        }
        None
    }

    fn list_item(&self, paragraph: &Paragraph) -> Option<ParagraphKind> {
        if let Some(list) = paragraph.props.list {
            let level = self.list_level(list);
            if level.and_then(|l| l.number_format) == Some(255) {
                return None;
            }
            let ordered = level
                .and_then(ListLevel::is_numbered)
                .or_else(|| paragraph.label.as_deref().map(label_is_numbered))
                .unwrap_or(false);
            return Some(ParagraphKind::ListItem {
                ordered,
                depth: list.level,
            });
        }
        if let Some(numbering) = paragraph.props.legacy_numbering {
            if numbering.continuation {
                return None;
            }
            return Some(ParagraphKind::ListItem {
                ordered: !numbering.bullet,
                depth: numbering.level,
            });
        }
        let label = paragraph
            .label
            .as_deref()
            .filter(|l| !l.trim().is_empty())?;
        Some(ParagraphKind::ListItem {
            ordered: label_is_numbered(label),
            depth: 0,
        })
    }

    /// A paragraph's inline items as pieces between line breaks, and the comment places in
    /// them.
    fn pieces_of(&mut self, paragraph: &Paragraph, context: Context) -> (Vec<Piece>, Vec<Marker>) {
        let mut pieces = vec![Piece::default()];
        let mut markers = Vec::new();
        for inline in &paragraph.inlines {
            match inline {
                Inline::Text(run) => self.text(run, &mut pieces),
                Inline::Break(BreakKind::Page) => pieces.push(Piece {
                    page_break_before: true,
                    ..Piece::default()
                }),
                Inline::Break(BreakKind::Line | BreakKind::Column) => {
                    pieces.push(Piece::default());
                }
                Inline::Picture(id) => {
                    if let Some(run) = self.picture(*id, context) {
                        push_object(&mut pieces, run);
                    }
                }
                Inline::NoteReference(id) => match context {
                    Context::Body => {
                        if let Some(run) = self.note(*id) {
                            push_object(&mut pieces, run);
                        }
                    }
                    Context::Inner => self.notes_not_carried += 1,
                },
                Inline::AnnotationReference(id) => {
                    markers.push(marker(&pieces, MarkerKind::Annotation(*id)));
                }
                Inline::AnnotationRangeStart(tag) => {
                    markers.push(marker(&pieces, MarkerKind::RangeStart(tag.clone())));
                }
                Inline::AnnotationRangeEnd(tag) => {
                    markers.push(marker(&pieces, MarkerKind::RangeEnd(tag.clone())));
                }
                Inline::FieldStart(_) | Inline::FieldEnd(_) => {}
            }
        }
        (pieces, markers)
    }

    fn text(&mut self, run: &rtf_reader::Run, pieces: &mut Vec<Piece>) {
        if run.props.hidden {
            self.not_carried.hidden_characters += run.text.chars().count();
            return;
        }
        if !run.text.trim().is_empty() {
            self.count_formatting(&run.props);
        }
        let style = RunStyle {
            bold: run.props.bold,
            italic: run.props.italic,
            underline: run.props.underline,
            strikethrough: run.props.strike,
            code: run
                .props
                .font
                .is_some_and(|font| self.options.code_fonts.contains(&font)),
            superscript: run.props.vertical == VerticalPosition::Superscript,
            subscript: run.props.vertical == VerticalPosition::Subscript,
        };
        let link = run.field.and_then(|id| match self.links.get(id) {
            Some(FieldLink::Web(url)) => Some(url.clone()),
            Some(FieldLink::Other) => {
                // Counted once per link whose words the text shows, not per field the file
                // declares: a caller may have turned a field's words into plain text already.
                if self.links_counted.insert(id) {
                    self.not_carried.links_kept_as_text += 1;
                }
                None
            }
            _ => None,
        });
        let mut stretch = String::new();
        for c in run.text.chars() {
            match c {
                '\u{2028}' | '\u{2029}' => {
                    push_text(pieces, &mut stretch, style, &link);
                    pieces.push(Piece::default());
                }
                rich::IMAGE_PLACEHOLDER => self.not_carried.object_replacement_characters += 1,
                _ => stretch.push(c),
            }
        }
        push_text(pieces, &mut stretch, style, &link);
    }

    fn count_formatting(&mut self, props: &CharProps) {
        if props
            .background
            .is_some_and(|index| self.doc.background_shows(index))
        {
            self.not_carried.highlighted_runs += 1;
        }
        let black = rtf_reader::Rgb {
            red: 0,
            green: 0,
            blue: 0,
        };
        if props
            .color
            .and_then(|index| self.doc.color(index))
            .is_some_and(|rgb| rgb != black)
        {
            self.not_carried.coloured_runs += 1;
        }
        if props.small_caps {
            self.not_carried.small_caps_runs += 1;
        }
        if props.all_caps {
            self.not_carried.all_caps_runs += 1;
        }
    }

    /// The picture `id` as an image run where the text shows it, or `None` when it stays out,
    /// said why ([`ImportDiagnostic::ImageNotIngested`]). A note's or a comment's picture
    /// stays out, as it does from a Word or an OpenDocument file: those keep their words
    /// and no picture.
    fn picture(&mut self, id: usize, context: Context) -> Option<Run> {
        let picture = self.doc.pictures.get(id)?;
        self.pictures_shown.insert(id);
        let name = picture_name(id, picture);
        let brought = if context == Context::Inner {
            Err(PictureRefusal::InNote)
        } else if picture.oversized {
            Err(PictureRefusal::TooLarge)
        } else {
            match self.options.picture_source {
                Some(source) => source(id, picture),
                None => self.bring(id, picture, &name),
            }
        };
        match brought {
            Ok(sourced) => {
                if sourced.first_frame_only {
                    self.diagnostics
                        .push(ImportDiagnostic::PictureFirstFrameOnly {
                            path: self.options.origin.to_string(),
                            target: name.clone(),
                        });
                }
                // The size the file shows it at, else its own.
                let (width, height) = match display_size(picture) {
                    (0, _) | (_, 0) => (sourced.placed.width, sourced.placed.height),
                    shown => shown,
                };
                Some(Run::picture("", sourced.placed.src, name, width, height))
            }
            Err(reason) => {
                self.diagnostics.push(ImportDiagnostic::ImageNotIngested {
                    path: self.options.origin.to_string(),
                    target: name,
                    reason,
                });
                None
            }
        }
    }

    /// Bring picture `id` in with this document's own pictures.
    fn bring(
        &mut self,
        id: usize,
        picture: &Picture,
        name: &str,
    ) -> Result<SourcedPicture, PictureRefusal> {
        if picture.data.is_empty() {
            return Err(no_bytes(picture));
        }
        let brought = self
            .pictures
            .bring(&format!("#{id}"), name, || Ok(picture.data.clone()));
        brought.result.map(|placed| SourcedPicture {
            placed,
            first_frame_only: brought.first_frame_only,
        })
    }

    /// The reference to note `id`, its text added to the footnotes. `None` for a note with
    /// no text, which would cite nothing.
    fn note(&mut self, id: usize) -> Option<Run> {
        let note = self.doc.notes.get(id)?;
        let body = self.inner_paragraphs(&note.blocks);
        if body.is_empty() {
            self.notes_not_carried += 1;
            return None;
        }
        if note.kind == NoteKind::Endnote {
            self.not_carried.endnotes += 1;
        }
        let label = match self.options.footnote_labels {
            Some(mint) => mint(),
            None => format!("{FOOTNOTE_LABEL_PREFIX}{}", self.footnotes.len() + 1),
        };
        self.footnotes.push(RichFootnote {
            label: label.clone(),
            paragraphs: body,
        });
        Some(Run {
            footnote: Some(label),
            ..Run::default()
        })
    }

    /// A note's or a comment's own text: one paragraph per piece, a table's paragraphs cell
    /// by cell. Blank pieces are left out.
    fn inner_paragraphs(&mut self, blocks: &[Block]) -> Vec<Vec<Run>> {
        let mut out = Vec::new();
        for paragraph in paragraphs(blocks) {
            let (pieces, _) = self.pieces_of(paragraph, Context::Inner);
            out.extend(
                pieces
                    .into_iter()
                    .filter(|piece| !piece.is_blank())
                    .map(|piece| piece.runs),
            );
        }
        out
    }

    // ── tables ───────────────────────────────────────────────────────────────

    fn table(&mut self, table: &rtf_reader::Table) {
        let index = self.blocks.len();
        let mut rows = Vec::with_capacity(table.rows.len());
        // Offset of the next cell in the table's plain text, where cells are joined by one
        // line feed (`RichBlock::plain_text`).
        let mut offset = 0usize;
        let mut first = true;
        for row in &table.rows {
            let mut cells = Vec::with_capacity(row.cells.len());
            for cell in &row.cells {
                if !first {
                    offset += 1;
                }
                first = false;
                let (runs, len, markers) = self.cell(cell);
                for (kind, at) in markers {
                    self.record(kind, (index, offset + at));
                }
                offset += len;
                cells.push(runs);
            }
            rows.push(cells);
        }
        self.blocks.push(RichBlock::Table { rows });
    }

    /// A cell's paragraphs as one run list, joined by spaces, and the comment places in it.
    fn cell(&mut self, cell: &rtf_reader::Cell) -> (Vec<Run>, usize, Vec<(MarkerKind, usize)>) {
        self.not_carried.nested_tables += nested_tables(&cell.blocks);
        let mut runs = Vec::new();
        let mut len = 0usize;
        let mut markers = Vec::new();
        let mut kept = 0usize;
        for paragraph in paragraphs(&cell.blocks) {
            let (pieces, found) = self.pieces_of(paragraph, Context::Body);
            let mut starts = Vec::with_capacity(pieces.len());
            let blank: Vec<bool> = pieces.iter().map(Piece::is_blank).collect();
            for (piece, blank) in pieces.into_iter().zip(blank.iter().copied()) {
                if blank {
                    starts.push(len);
                    continue;
                }
                if kept > 0 {
                    runs.push(Run::plain(" "));
                    len += 1;
                }
                kept += 1;
                starts.push(len);
                len += piece.len;
                runs.extend(piece.runs);
            }
            for marker in found {
                let place = match (starts.get(marker.piece), blank.get(marker.piece)) {
                    // A blank piece is not written, so a place in it is where the next kept
                    // text starts: counting into its white space would land that many
                    // characters into the text after it.
                    (Some(start), Some(true)) => *start,
                    (Some(start), _) => start + marker.offset,
                    (None, _) => len,
                };
                markers.push((marker.kind, place));
            }
        }
        if kept > 1 {
            self.not_carried.joined_cells += 1;
        }
        (runs, len, markers)
    }

    // ── comments ─────────────────────────────────────────────────────────────

    fn record(&mut self, kind: MarkerKind, at: Position) {
        match kind {
            MarkerKind::Annotation(id) => {
                self.annotation_at.entry(id).or_insert(at);
            }
            MarkerKind::RangeStart(tag) => {
                self.range_starts.entry(tag).or_insert(at);
            }
            MarkerKind::RangeEnd(tag) => {
                self.range_ends.entry(tag).or_insert(at);
            }
        }
    }

    fn annotations(&mut self) -> Vec<RichAnnotation> {
        let doc = self.doc;
        // A reply names the comment it answers by that comment's reference tag.
        let by_reference: HashMap<&str, usize> = doc
            .annotations
            .iter()
            .enumerate()
            .filter_map(|(i, a)| Some((a.reference.as_deref()?, i)))
            .collect();
        let roots = thread_roots(doc, &by_reference);
        // Each block's length in plain text, for a range that runs past the block it starts
        // in: measured once here, not once per comment.
        let lengths: Vec<usize> = self
            .blocks
            .iter()
            .map(|b| b.plain_text().chars().count())
            .collect();

        let mut threads: Vec<Option<RichAnnotation>> = Vec::with_capacity(doc.annotations.len());
        let mut replies: Vec<(usize, usize)> = Vec::new();
        for (index, annotation) in doc.annotations.iter().enumerate() {
            if annotation.date.is_some() || annotation.time.is_some() {
                self.not_carried.comment_dates += 1;
            }
            if annotation.parent.is_some() {
                match roots.get(index).copied().flatten() {
                    Some(root) => {
                        replies.push((root, index));
                        threads.push(None);
                        continue;
                    }
                    None => self.replies_flattened += 1,
                }
            }
            let (block_index, start, length, end) = self.anchor(index, annotation, &lengths);
            let paragraphs = self.inner_paragraphs(&annotation.blocks);
            threads.push(Some(RichAnnotation {
                block_index,
                start,
                length,
                end,
                between_blocks: false,
                uid: None,
                uid_tag: None,
                author: author_of(annotation),
                author_initials: annotation.initials.clone().unwrap_or_default(),
                created: None,
                paragraphs,
                resolved: false,
                replies: Vec::new(),
            }));
        }
        for (root, index) in replies {
            let Some(annotation) = doc.annotations.get(index) else {
                continue;
            };
            let paragraphs = self.inner_paragraphs(&annotation.blocks);
            if let Some(Some(thread)) = threads.get_mut(root) {
                thread.replies.push(RichReply {
                    uid: None,
                    author: author_of(annotation),
                    author_initials: annotation.initials.clone().unwrap_or_default(),
                    created: None,
                    paragraphs,
                });
            }
        }
        threads.into_iter().flatten().collect()
    }

    /// Where comment `index` points: its range when the file marks one, else the place it
    /// is anchored, as a comment on that paragraph.
    fn anchor(
        &self,
        index: usize,
        annotation: &rtf_reader::Annotation,
        lengths: &[usize],
    ) -> (usize, usize, usize, Option<AnnotationEnd>) {
        let range = annotation.reference.as_ref().and_then(|tag| {
            let start = *self.range_starts.get(tag)?;
            let end = *self.range_ends.get(tag)?;
            (end > start).then_some((start, end))
        });
        if let Some(((block, offset), (end_block, end_offset))) = range {
            if block == end_block {
                return (block, offset, end_offset - offset, None);
            }
            let block_len = lengths.get(block).copied().unwrap_or(0);
            return (
                block,
                offset,
                block_len.saturating_sub(offset),
                Some(AnnotationEnd {
                    block_index: end_block,
                    offset: end_offset,
                }),
            );
        }
        match self.annotation_at.get(&index) {
            Some(&(block, offset)) => (block, offset, 0, None),
            // Anchored where no body block is (inside a note): `assemble` places it on the
            // last paragraph and the planner reports it.
            None => (usize::MAX, 0, 0, None),
        }
    }

    // ── the end ──────────────────────────────────────────────────────────────

    fn finish(mut self, annotations: Vec<RichAnnotation>) -> RtfConversion {
        let origin = self.options.origin.to_string();
        let diagnostics = &self.doc.diagnostics;

        // Pictures the text never shows sit inside a list label (a picture bullet), which is
        // not kept: counted, as the label's other parts are not.
        self.not_carried.label_pictures = (0..self.doc.pictures.len())
            .filter(|id| !self.pictures_shown.contains(id))
            .count();
        let fields = self
            .doc
            .fields
            .iter()
            .filter(|f| matches!(f.kind(), FieldKind::Other { .. }))
            .count();
        let text_boxes = diagnostics.count(&Diagnostic::TextBoxSkipped);
        let objects = diagnostics.count(&Diagnostic::ObjectWithoutResult)
            + diagnostics.count(&Diagnostic::ShapeSkipped);
        for (count, diagnostic) in [
            (
                fields,
                ImportDiagnostic::FieldFlattened {
                    path: origin.clone(),
                    count: fields,
                },
            ),
            (
                text_boxes,
                ImportDiagnostic::TextBoxDropped {
                    path: origin.clone(),
                    count: text_boxes,
                },
            ),
            (
                objects,
                ImportDiagnostic::EmbeddedObjectDropped {
                    path: origin.clone(),
                    count: objects,
                },
            ),
            (
                self.notes_not_carried,
                ImportDiagnostic::FootnoteNotCarried {
                    path: origin.clone(),
                    count: self.notes_not_carried,
                },
            ),
            (
                self.replies_flattened,
                ImportDiagnostic::CommentRepliesFlattened {
                    path: origin.clone(),
                    count: self.replies_flattened,
                },
            ),
        ] {
            if count > 0 {
                self.diagnostics.push(diagnostic);
            }
        }

        let not_carried = &mut self.not_carried;
        not_carried.headers_and_footers = diagnostics.count(&Diagnostic::HeaderOrFooterSkipped);
        not_carried.computed_characters = diagnostics.count(&Diagnostic::DynamicCharacterSkipped);
        not_carried.control_characters = diagnostics.count(&Diagnostic::ControlCharacterRemoved);
        not_carried.after_document = diagnostics.count(&Diagnostic::BytesAfterDocument);
        not_carried.unread_destinations = diagnostics
            .iter()
            .filter(|(kind, _)| {
                matches!(
                    kind,
                    Diagnostic::IgnorableDestinationSkipped(_)
                        | Diagnostic::OtherIgnorableDestinationsSkipped
                )
            })
            .map(|(_, count)| count)
            .sum();
        not_carried.undecodable_characters = diagnostics
            .iter()
            .filter(|(kind, _)| {
                matches!(
                    kind,
                    Diagnostic::UndecodableBytes
                        | Diagnostic::UnsupportedCodePage(_)
                        | Diagnostic::LoneSurrogate
                        | Diagnostic::InvalidUnicode
                )
            })
            .map(|(_, count)| count)
            .sum();

        RtfConversion {
            document: RichDocument {
                blocks: self.blocks,
                annotations,
                row_marks: Vec::new(),
                footnotes: self.footnotes,
            },
            diagnostics: self.diagnostics,
            not_carried: self.not_carried,
            pictures: self.pictures.into_pictures(),
            block_origins: Vec::new(),
        }
    }
}

/// Why a picture the file names has no bytes to bring in.
fn no_bytes(picture: &Picture) -> PictureRefusal {
    match picture.format {
        // A file beside the RTF in its `.rtfd` package: the RTF holds only its name.
        PictureFormat::NextAttachment => PictureRefusal::Missing,
        PictureFormat::Pdf => PictureRefusal::Unsupported { format: "PDF" },
        PictureFormat::Emf => PictureRefusal::Unsupported { format: "EMF" },
        PictureFormat::Wmf => PictureRefusal::Unsupported { format: "WMF" },
        PictureFormat::MacPict => PictureRefusal::Unsupported { format: "PICT" },
        _ => PictureRefusal::Unreadable,
    }
}

/// For each comment, the comment its thread opens with when it is a reply: the first one up
/// its chain of parents with no parent of its own. `None` for a comment with no parent, and
/// for a reply whose chain leads nowhere in the file or into a loop.
///
/// Each comment's thread is kept once found, so a chain is walked once however many replies
/// hang from it: walked again from every reply, a long chain would be quadratic.
fn thread_roots(doc: &Document, by_reference: &HashMap<&str, usize>) -> Vec<Option<usize>> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum State {
        Unknown,
        /// On the chain being walked: met again, it is a loop.
        OnPath,
        /// The first comment of the thread this one is in, `None` for a chain that leads
        /// nowhere or into a loop.
        Thread(Option<usize>),
    }
    let annotations = &doc.annotations;
    let mut state = vec![State::Unknown; annotations.len()];
    let mut path = Vec::new();
    for start in 0..annotations.len() {
        let mut current = start;
        let found = loop {
            match state.get(current).copied() {
                Some(State::Thread(first)) => break first,
                Some(State::OnPath) | None => break None,
                Some(State::Unknown) => {}
            }
            let Some(parent) = annotations.get(current).and_then(|a| a.parent.as_deref()) else {
                state[current] = State::Thread(Some(current));
                break Some(current);
            };
            state[current] = State::OnPath;
            path.push(current);
            match by_reference.get(parent) {
                Some(&next) => current = next,
                None => break None,
            }
        };
        for index in path.drain(..) {
            state[index] = State::Thread(found);
        }
    }
    annotations
        .iter()
        .zip(state)
        .map(|(annotation, state)| match state {
            State::Thread(first) if annotation.parent.is_some() => first,
            _ => None,
        })
        .collect()
}

/// A comment's place in the piece being read.
fn marker(pieces: &[Piece], kind: MarkerKind) -> Marker {
    Marker {
        kind,
        piece: pieces.len().saturating_sub(1),
        offset: pieces.last().map_or(0, |p| p.len),
    }
}

fn push_text(pieces: &mut [Piece], stretch: &mut String, style: RunStyle, link: &Option<String>) {
    if stretch.is_empty() {
        return;
    }
    let text = std::mem::take(stretch);
    if let Some(piece) = pieces.last_mut() {
        piece.len += text.chars().count();
        piece.runs.push(Run {
            text,
            style,
            link: link.clone(),
            footnote: None,
            image: None,
        });
    }
}

/// A picture or a note's reference: one character of plain text.
fn push_object(pieces: &mut [Piece], run: Run) {
    if let Some(piece) = pieces.last_mut() {
        piece.len += 1;
        piece.runs.push(run);
    }
}

fn props_of(paragraph: &Paragraph) -> BlockProps {
    BlockProps {
        alignment: match paragraph.props.alignment {
            Some(RtfAlignment::Left) => Some(Alignment::Left),
            Some(RtfAlignment::Center) => Some(Alignment::Center),
            Some(RtfAlignment::Right) => Some(Alignment::Right),
            Some(RtfAlignment::Justify) => Some(Alignment::Justify),
            // The start edge of the paragraph's own direction: what no alignment means.
            Some(RtfAlignment::Natural) | None => None,
        },
        direction: paragraph.props.direction.map(|d| match d {
            RtfDirection::LeftToRight => Direction::LeftToRight,
            RtfDirection::RightToLeft => Direction::RightToLeft,
        }),
        page_break_before: paragraph.props.page_break_before,
    }
}

/// `heading N`, as Word names its built-in heading styles in RTF, N from 1 to 9.
fn heading_name_level(name: &str) -> Option<u8> {
    let (word, number) = name.trim().rsplit_once(' ')?;
    if !word.trim().eq_ignore_ascii_case("heading") {
        return None;
    }
    number.parse::<u8>().ok().filter(|n| (1..=9).contains(n))
}

/// Whether a stored list label is a number (`1.`, `iv)`, `a.`) rather than a bullet.
fn label_is_numbered(label: &str) -> bool {
    label
        .trim()
        .chars()
        .next()
        .is_some_and(char::is_alphanumeric)
}

fn is_web_address(target: &str) -> bool {
    let lower = target.trim().to_ascii_lowercase();
    ["http://", "https://", "mailto:"]
        .iter()
        .any(|scheme| lower.starts_with(scheme))
}

fn author_of(annotation: &rtf_reader::Annotation) -> String {
    annotation
        .author
        .clone()
        .or_else(|| annotation.initials.clone())
        .unwrap_or_default()
}

/// What a picture is called in a diagnostic, and the reference it keeps until it is stored.
fn picture_name(id: usize, picture: &Picture) -> String {
    match picture.name.as_deref().filter(|n| !n.trim().is_empty()) {
        Some(name) => name.to_string(),
        None => format!("picture {}", id + 1),
    }
}

/// The size a picture is shown at, in pixels at 96 to the inch: its goal size in twips when
/// the file states one, else a bitmap's own pixels, each scaled. Zero when neither is known,
/// which leaves the size to the picture.
fn display_size(picture: &Picture) -> (u32, u32) {
    let bitmap = matches!(
        picture.format,
        PictureFormat::Png | PictureFormat::Jpeg | PictureFormat::Dib | PictureFormat::Bitmap
    );
    let side = |goal: Option<i32>, pixels: Option<i32>, scale: Option<i32>| -> u32 {
        let scale = f64::from(scale.filter(|s| *s > 0).unwrap_or(100)) / 100.0;
        let base = match (goal.filter(|g| *g > 0), pixels.filter(|p| *p > 0)) {
            // Fifteen twips to a pixel at 96 to the inch.
            (Some(twips), _) => f64::from(twips) / 15.0,
            (None, Some(pixels)) if bitmap => f64::from(pixels),
            _ => return 0,
        };
        let size = (base * scale).round();
        if size.is_finite() && size >= 1.0 && size <= f64::from(u32::MAX) {
            size as u32
        } else {
            0
        }
    };
    let width = side(picture.goal_width, picture.width, picture.scale_x);
    let height = side(picture.goal_height, picture.height, picture.scale_y);
    // Both or neither, as the emitter writes a picture's size.
    if width == 0 || height == 0 {
        (0, 0)
    } else {
        (width, height)
    }
}

/// How many tables sit inside `blocks`, at any depth.
fn nested_tables(blocks: &[Block]) -> usize {
    let mut count = 0usize;
    let mut stack: Vec<&[Block]> = vec![blocks];
    while let Some(blocks) = stack.pop() {
        for block in blocks {
            if let Block::Table(table) = block {
                count += 1;
                for cell in table.rows.iter().flat_map(|row| row.cells.iter()) {
                    stack.push(&cell.blocks);
                }
            }
        }
    }
    count
}
