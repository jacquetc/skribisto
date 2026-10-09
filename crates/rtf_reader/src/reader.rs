// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Tokens to the document model.
//!
//! One loop over the tokens and an explicit stack of group states: a `{` pushes a copy of the
//! state, a `}` restores the one below (RTF 1.9.1, page 10). Nothing recurses on the input's
//! nesting, so a file of a hundred thousand braces costs a vector of frames, bounded by
//! [`Limits::max_depth`], never the call stack. A group being skipped pushes nothing at all:
//! only a counter tracks where it ends.
//!
//! A group's first control word may name a *destination*: where the group's text goes (the
//! font table, a field instruction, a footnote, a picture's data). A group opening with `\*`
//! and a word this reader does not know is skipped whole, as the specification asks; a group
//! opening with an unknown word and no `\*` keeps its text, as it also asks. The destinations
//! whose text is known not to be document text (headers, index entries, bookmarks, a list's
//! level text) are listed and skipped, star or not.

use std::collections::HashMap;
use std::mem::size_of;

use crate::codepage;
use crate::diagnostics::Diagnostic;
use crate::lexer::{LeadBytes, Lexer, Token};
use crate::limits::{Limits, Refusal};
use crate::model::{
    Alignment, Annotation, Block, BreakKind, CharProps, Charset, Direction, Document, Field, Font,
    FontFamily, Inline, LegacyNumbering, List, ListLevel, ListOverride, ListRef, Note, NoteKind,
    ParagraphEnd, ParagraphProps, Picture, PictureFormat, PictureOrigin, Rgb, RtfTime, Run, Style,
    StyleKind, VerticalPosition,
};
use crate::story::{MAX_TABLE_NESTING, Story, slot_bytes};
use crate::{Options, RawText};

/// Destinations whose text is not document text, skipped whether or not they open with `\*`,
/// and without being counted: every writer produces some of these, and none of them holds a
/// word of the manuscript.
const NON_BODY: &[&[u8]] = &[
    b"aftncn",
    b"aftnsep",
    b"aftnsepc",
    b"atnicn",
    b"background",
    b"bkmkend",
    b"bkmkstart",
    b"blipuid",
    b"category",
    b"colorschememapping",
    b"comment",
    b"company",
    b"datafield",
    b"datastore",
    b"defchp",
    b"defpap",
    b"do",
    b"docvar",
    b"expandedcolortbl",
    b"falt",
    b"fchars",
    b"ffdeftext",
    b"ffentrymcr",
    b"ffexitmcr",
    b"ffformat",
    b"ffhelptext",
    b"ffl",
    b"ffname",
    b"ffstattext",
    b"filetbl",
    b"fldtype",
    b"fname",
    b"fontemb",
    b"fontfile",
    b"formfield",
    b"ftncn",
    b"ftnsep",
    b"ftnsepc",
    b"hlinkbase",
    b"keycode",
    b"latentstyles",
    b"lchars",
    b"levelnumbers",
    b"leveltext",
    b"listname",
    b"listpicture",
    b"liststylename",
    b"manager",
    b"mmathPr",
    b"nonesttables",
    b"nonshppict",
    b"objalias",
    b"objclass",
    b"objdata",
    b"objname",
    b"objsect",
    b"objtime",
    b"oleclsid",
    b"panose",
    b"passwordhash",
    b"pgdsc",
    b"pgdsctbl",
    b"pgptbl",
    b"picprop",
    b"pntxta",
    b"pntxtb",
    b"private",
    b"protusertbl",
    b"revtbl",
    b"rsidtbl",
    b"rxe",
    b"shprslt",
    b"tc",
    b"tcn",
    b"template",
    b"themedata",
    b"txe",
    b"userprops",
    b"wgrffmtfilter",
    b"xe",
    b"xmlnstbl",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InfoField {
    Title,
    Subject,
    Author,
    Keywords,
    Comment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MetaField {
    Initials,
    Author,
    Date,
    Reference,
    Parent,
}

/// Where a group's text goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dest {
    /// Document text, into the frame's story.
    Body,
    /// Nothing: the group is skipped.
    Skip,
    FontTable,
    ColorTable,
    Stylesheet,
    StyleEntry,
    Info,
    ListTable,
    List,
    ListLevel,
    ListOverrideTable,
    ListOverride,
    /// The `\field` group itself, outside its instruction and result.
    Field(usize),
    FieldInstruction(usize),
    Picture(usize),
    PictureName(usize),
    NextAttachment(usize),
    /// `\listtext` or `\pntext`: the paragraph's label.
    ListLabel,
    /// `\*\pn`: Word 95 numbering, words only.
    LegacyNumbering,
    /// `\*\nesttableprops`: row properties, where `\nestrow` ends a nested row.
    NestTableProps,
    /// `\upr`: its first half is the text for readers without Unicode, skipped; its
    /// `\*\ud` half is read.
    Upr,
    Shape,
    ShapeInstance,
    ShapeProperty,
    /// `\sv` of a shape's `pib` property: where its picture is.
    ShapePictureValue,
    Object,
    AnnotationTime,
    /// Text gathered into the reader's `collected` buffer and put somewhere at the close.
    Collect(Collector),
}

impl Dest {
    /// Whether text here belongs to a story: the places a field, a picture or a note can
    /// start.
    fn is_text(self) -> bool {
        matches!(self, Dest::Body | Dest::ListLabel)
    }

    fn accepts_text(self) -> bool {
        matches!(
            self,
            Dest::Body
                | Dest::ListLabel
                | Dest::FontTable
                | Dest::ColorTable
                | Dest::StyleEntry
                | Dest::FieldInstruction(_)
                | Dest::PictureName(_)
                | Dest::NextAttachment(_)
                | Dest::Collect(_)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Collector {
    Info(InfoField),
    Generator,
    LevelMarker,
    Meta(MetaField),
    RangeStart,
    RangeEnd,
    ShapePropertyName,
}

/// What closing a group finishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Closes {
    Nothing,
    Field(usize),
    Picture,
    Note { story: usize, note: usize },
    Annotation { story: usize, annotation: usize },
    Collected(Collector),
    AnnotationTime,
    FontGroup,
    StyleEntry,
    LegacyNumbering,
    NextAttachment,
    Shape { pictures_before: usize },
    Object { results_before: usize },
}

/// The state a group carries, restored when it closes.
#[derive(Debug, Clone, Copy)]
struct Frame {
    dest: Dest,
    /// The text destination in force when a container (a field, an `\upr`, an object, a
    /// shape) opened: where its result goes.
    outer: Dest,
    /// `\ucN`: how many characters follow a `\uN` for readers without Unicode.
    uc: u32,
    chars: CharProps,
    para: ParagraphProps,
    /// `\ilvlN` seen before the `\lsN` it belongs to.
    list_level: u8,
    story: usize,
    /// The innermost field whose result this group is in.
    field: Option<usize>,
    /// Inside `\*\shppict`.
    shape_picture: bool,
    /// Inside an object's `\result`.
    object_result: bool,
    closes: Closes,
    serial: u32,
}

/// What a story belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Owner {
    Body,
    Note(usize),
    Annotation(usize),
}

/// A colour table entry being read.
#[derive(Debug, Clone, Copy, Default)]
struct ColorEntry {
    stated: bool,
    rgb: [u8; 3],
}

pub(crate) struct Reader<'a> {
    lexer: Lexer<'a>,
    limits: Limits,
    utf8_text: bool,
    doc: Document,
    frame: Frame,
    stack: Vec<Frame>,
    /// Groups opened inside a skipped group, which push no frame.
    skip_depth: usize,
    next_serial: u32,
    /// The token before this one was a `{`.
    first_in_group: bool,
    /// The group opened with `\*`.
    star: bool,
    /// Characters still to skip after a `\uN`.
    skip_chars: u32,
    high_surrogate: Option<u16>,
    /// 8-bit text not yet decoded, and the code page it is in.
    pending: Vec<u8>,
    pending_code_page: u16,
    stories: Vec<Story>,
    owners: Vec<Owner>,
    /// Font number to the code page its text is in, for fonts that name one.
    font_pages: HashMap<i32, u16>,
    font: Option<Font>,
    color: ColorEntry,
    style: Option<Style>,
    style_named: bool,
    collected: String,
    shape_property: String,
    /// Author parts read before the `\annotation` they belong to.
    pending_annotation: Annotation,
    pending_time: RtfTime,
    picture_nibble: Option<u8>,
    /// The byte after an Apple attachment stands for it and is not text.
    drop_attachment_byte: bool,
    object_results: usize,
    root_closed: bool,
    refusal: Option<Refusal>,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(input: &'a [u8], start: usize, options: &Options) -> Self {
        let frame = Frame {
            dest: Dest::Body,
            outer: Dest::Body,
            uc: 1,
            chars: CharProps::default(),
            para: ParagraphProps::default(),
            list_level: 0,
            story: 0,
            field: None,
            shape_picture: false,
            object_result: false,
            closes: Closes::Nothing,
            serial: 0,
        };
        Reader {
            lexer: Lexer::new(input, start),
            limits: options.limits,
            utf8_text: options.raw_text == RawText::Utf8,
            doc: Document::default(),
            frame,
            stack: Vec::new(),
            skip_depth: 0,
            next_serial: 1,
            first_in_group: false,
            star: false,
            skip_chars: 0,
            high_surrogate: None,
            pending: Vec::new(),
            pending_code_page: 1252,
            stories: vec![Story::default()],
            owners: vec![Owner::Body],
            font_pages: HashMap::new(),
            font: None,
            color: ColorEntry::default(),
            style: None,
            style_named: false,
            collected: String::new(),
            shape_property: String::new(),
            pending_annotation: Annotation::default(),
            pending_time: RtfTime::default(),
            picture_nibble: None,
            drop_attachment_byte: false,
            object_results: 0,
            root_closed: false,
            refusal: None,
        }
    }

    pub(crate) fn run(mut self) -> Result<Document, Refusal> {
        loop {
            if let Some(refusal) = self.refusal.take() {
                return Err(refusal);
            }
            let lead = self.lead_bytes();
            let Some(token) = self.lexer.next(lead) else {
                break;
            };
            if self.root_closed {
                self.after_root(token);
                continue;
            }
            self.token(token);
        }
        self.finish()
    }

    // ── tokens ───────────────────────────────────────────────────────────────

    fn token(&mut self, token: Token<'a>) {
        let first = std::mem::take(&mut self.first_in_group);
        let star = std::mem::take(&mut self.star);
        let attachment = std::mem::take(&mut self.drop_attachment_byte);
        match token {
            Token::Open => self.open(),
            Token::Close => self.close(),
            Token::Word { name, param } => {
                self.flush();
                self.word(name, param, first, star);
            }
            Token::Par => {
                self.flush();
                self.word(b"par", None, false, false);
            }
            Token::Symbol(b'*') if first => {
                // `\*` counts only as the first token of a group, and keeps the group's
                // first word looking like its first.
                self.star = true;
                self.first_in_group = true;
            }
            Token::Symbol(symbol) => self.symbol(symbol),
            Token::Hex(byte) => self.hex(byte),
            Token::BadHex => {
                self.doc.diagnostics.add(Diagnostic::BadHexEscape, 1);
                if self.skip_chars > 0 {
                    self.skip_chars -= 1;
                }
            }
            Token::Text(bytes) => self.text(bytes, attachment),
            Token::Bin { declared, data } => self.bin(declared, data),
        }
    }

    fn after_root(&mut self, token: Token<'a>) {
        let blank =
            matches!(token, Token::Text(bytes) if bytes.iter().all(u8::is_ascii_whitespace));
        if !blank {
            self.doc.diagnostics.add(Diagnostic::BytesAfterDocument, 1);
        }
    }

    fn open(&mut self) {
        self.flush();
        self.skip_chars = 0;
        let depth = self.stack.len() + self.skip_depth + 1;
        if depth > self.limits.max_depth {
            self.refusal = Some(Refusal::NestedTooDeep {
                limit: self.limits.max_depth,
            });
            return;
        }
        self.doc.stats.max_depth = self.doc.stats.max_depth.max(depth);
        if self.frame.dest == Dest::Skip {
            self.skip_depth += 1;
            return;
        }
        let mut child = self.frame;
        child.closes = Closes::Nothing;
        child.serial = self.next_serial;
        self.next_serial = self.next_serial.saturating_add(1);
        match self.frame.dest {
            // Every group of the style sheet is one style's definition.
            Dest::Stylesheet => {
                self.finish_style();
                self.style = Some(Style {
                    number: 0,
                    kind: StyleKind::Paragraph,
                    name: String::new(),
                    based_on: None,
                    next: None,
                    outline_level: None,
                });
                self.style_named = false;
                child.dest = Dest::StyleEntry;
                child.closes = Closes::StyleEntry;
            }
            // A font table may hold one group per font.
            Dest::FontTable => child.closes = Closes::FontGroup,
            _ => {}
        }
        self.stack.push(self.frame);
        self.frame = child;
        self.first_in_group = true;
    }

    fn close(&mut self) {
        self.flush();
        self.skip_chars = 0;
        if self.skip_depth > 0 {
            self.skip_depth -= 1;
            return;
        }
        // Never empty here: the document opens with its own group, and a brace after that
        // group closes is read by `after_root`.
        let Some(parent) = self.stack.pop() else {
            return;
        };
        let closed = std::mem::replace(&mut self.frame, parent);
        if self.stack.is_empty() {
            // The document's own group: text after its last paragraph mark is a paragraph
            // with the formatting in force here, not the formatting outside the document.
            self.end_story_paragraph(0, closed.para);
            self.root_closed = true;
        }
        self.closing(closed);
    }

    fn closing(&mut self, closed: Frame) {
        match closed.closes {
            Closes::Nothing => {}
            Closes::Field(id) => {
                if closed.outer == Dest::Body {
                    self.push_inline_to(closed.story, Inline::FieldEnd(id));
                }
            }
            Closes::Picture => {
                if self.picture_nibble.take().is_some() {
                    self.doc.diagnostics.add(Diagnostic::OddPictureHex, 1);
                }
            }
            Closes::Note { story, note } => {
                let blocks = self.finish_story(story, closed.para);
                if let Some(note) = self.doc.notes.get_mut(note) {
                    note.blocks = blocks;
                }
            }
            Closes::Annotation { story, annotation } => {
                let blocks = self.finish_story(story, closed.para);
                if let Some(annotation) = self.doc.annotations.get_mut(annotation) {
                    annotation.blocks = blocks;
                }
            }
            Closes::Collected(collector) => {
                let text = std::mem::take(&mut self.collected);
                self.collected_text(collector, text, closed.story);
            }
            Closes::AnnotationTime => {
                let time = std::mem::take(&mut self.pending_time);
                match self.annotation_of(closed.story) {
                    Some(annotation) => annotation.time = Some(time),
                    None => self.pending_annotation.time = Some(time),
                }
            }
            Closes::FontGroup => self.finish_font(),
            Closes::StyleEntry => self.finish_style(),
            Closes::LegacyNumbering => {
                self.frame.para.legacy_numbering = closed.para.legacy_numbering;
            }
            Closes::NextAttachment => self.drop_attachment_byte = true,
            Closes::Shape { pictures_before } => {
                if self.doc.pictures.len() == pictures_before {
                    self.doc.diagnostics.add(Diagnostic::ShapeSkipped, 1);
                }
            }
            Closes::Object { results_before } => {
                let kind = if self.object_results == results_before {
                    Diagnostic::ObjectWithoutResult
                } else {
                    Diagnostic::ObjectReadAsResult
                };
                self.doc.diagnostics.add(kind, 1);
            }
        }
    }

    fn collected_text(&mut self, collector: Collector, text: String, story: usize) {
        let trimmed = text.trim().to_string();
        match collector {
            Collector::Info(field) => {
                let info = &mut self.doc.info;
                let slot = match field {
                    InfoField::Title => &mut info.title,
                    InfoField::Subject => &mut info.subject,
                    InfoField::Author => &mut info.author,
                    InfoField::Keywords => &mut info.keywords,
                    InfoField::Comment => &mut info.comment,
                };
                *slot = Some(trimmed);
            }
            Collector::Generator => {
                let name = trimmed.trim_end_matches(';').trim().to_string();
                self.doc.header.generator = Some(name);
            }
            Collector::LevelMarker => {
                if let Some(level) = self
                    .doc
                    .lists
                    .last_mut()
                    .and_then(|list| list.levels.last_mut())
                {
                    level.marker = Some(trimmed);
                }
            }
            Collector::Meta(field) => {
                let target = match self.owners.get(story) {
                    Some(Owner::Annotation(i)) => self.doc.annotations.get_mut(*i),
                    _ => None,
                };
                let annotation = match target {
                    Some(annotation) => annotation,
                    None => &mut self.pending_annotation,
                };
                let slot = match field {
                    MetaField::Initials => &mut annotation.initials,
                    MetaField::Author => &mut annotation.author,
                    MetaField::Date => &mut annotation.date,
                    MetaField::Reference => &mut annotation.reference,
                    MetaField::Parent => &mut annotation.parent,
                };
                *slot = Some(trimmed);
            }
            Collector::RangeStart => {
                self.push_inline_to(story, Inline::AnnotationRangeStart(trimmed));
            }
            Collector::RangeEnd => {
                self.push_inline_to(story, Inline::AnnotationRangeEnd(trimmed));
            }
            Collector::ShapePropertyName => self.shape_property = trimmed,
        }
    }

    fn annotation_of(&mut self, story: usize) -> Option<&mut Annotation> {
        match self.owners.get(story) {
            Some(Owner::Annotation(i)) => self.doc.annotations.get_mut(*i),
            _ => None,
        }
    }

    // ── control words ────────────────────────────────────────────────────────

    fn word(&mut self, name: &[u8], param: Option<i32>, first: bool, star: bool) {
        if self.frame.dest == Dest::Skip {
            return;
        }
        if first || star {
            if self.destination(name, param) {
                return;
            }
            if star {
                if !NON_BODY.contains(&name) {
                    self.doc.diagnostics.skipped_destination(name);
                }
                self.skip_group();
                return;
            }
        }
        // A control word is one of the characters a `\uN` asks to skip.
        if self.skip_chars > 0 {
            self.skip_chars -= 1;
            return;
        }
        if self.destination_word(name, param) {
            return;
        }
        self.common_word(name, param);
    }

    fn skip_group(&mut self) {
        if self.frame.closes == Closes::StyleEntry {
            self.style = None;
        }
        self.frame.dest = Dest::Skip;
        self.frame.closes = Closes::Nothing;
    }

    fn set(&mut self, dest: Dest, closes: Closes) -> bool {
        self.frame.dest = dest;
        self.frame.closes = closes;
        true
    }

    fn collect(&mut self, collector: Collector) -> bool {
        self.collected.clear();
        self.set(Dest::Collect(collector), Closes::Collected(collector))
    }

    /// A group's first word, read as a destination. False when it is not one here.
    fn destination(&mut self, name: &[u8], param: Option<i32>) -> bool {
        let dest = self.frame.dest;
        let text = dest.is_text();
        match name {
            b"fonttbl" => self.set(Dest::FontTable, Closes::FontGroup),
            b"colortbl" => {
                self.color = ColorEntry::default();
                self.set(Dest::ColorTable, Closes::Nothing)
            }
            b"stylesheet" => self.set(Dest::Stylesheet, Closes::Nothing),
            b"info" => self.set(Dest::Info, Closes::Nothing),
            b"title" if dest == Dest::Info => self.collect(Collector::Info(InfoField::Title)),
            b"subject" if dest == Dest::Info => self.collect(Collector::Info(InfoField::Subject)),
            b"author" if dest == Dest::Info => self.collect(Collector::Info(InfoField::Author)),
            b"keywords" if dest == Dest::Info => self.collect(Collector::Info(InfoField::Keywords)),
            b"doccomm" if dest == Dest::Info => self.collect(Collector::Info(InfoField::Comment)),
            b"generator" => self.collect(Collector::Generator),
            b"listtable" => self.set(Dest::ListTable, Closes::Nothing),
            b"list" if dest == Dest::ListTable => {
                self.doc.lists.push(List::default());
                self.grow(size_of::<List>());
                self.set(Dest::List, Closes::Nothing)
            }
            b"listlevel" if dest == Dest::List => {
                // Every list has a level list of its own, whose first level allocates room
                // for several.
                let mut bytes = size_of::<ListLevel>();
                if let Some(list) = self.doc.lists.last_mut() {
                    bytes = slot_bytes(&list.levels);
                    list.levels.push(ListLevel::default());
                }
                self.grow(bytes);
                self.set(Dest::ListLevel, Closes::Nothing)
            }
            b"levelmarker" if dest == Dest::ListLevel => self.collect(Collector::LevelMarker),
            b"listoverridetable" => self.set(Dest::ListOverrideTable, Closes::Nothing),
            b"listoverride" if dest == Dest::ListOverrideTable => {
                self.doc.list_overrides.push(ListOverride::default());
                self.grow(size_of::<ListOverride>());
                self.set(Dest::ListOverride, Closes::Nothing)
            }
            b"field" if text => {
                let id = self.doc.fields.len();
                self.doc.fields.push(Field::default());
                self.grow(size_of::<Field>());
                if dest == Dest::Body {
                    self.push_inline(Inline::FieldStart(id));
                }
                self.frame.outer = dest;
                self.set(Dest::Field(id), Closes::Field(id))
            }
            b"fldinst" => match dest {
                Dest::Field(id) => self.set(Dest::FieldInstruction(id), Closes::Nothing),
                // A field nested in an instruction: its parts join the outer instruction
                // (counted once, at its `\field`).
                Dest::FieldInstruction(_) => true,
                _ => self.set(Dest::Skip, Closes::Nothing),
            },
            b"fldrslt" => match dest {
                Dest::Field(id) => {
                    if let Some(field) = self.doc.fields.get_mut(id) {
                        field.has_result = true;
                    }
                    self.frame.dest = self.frame.outer;
                    self.frame.field = Some(id);
                    true
                }
                Dest::FieldInstruction(_) => true,
                _ => self.set(Dest::Skip, Closes::Nothing),
            },
            b"field" if matches!(dest, Dest::FieldInstruction(_)) => {
                self.doc.diagnostics.add(Diagnostic::FieldInInstruction, 1);
                true
            }
            b"shppict" => {
                self.frame.shape_picture = true;
                true
            }
            b"pict" if text || dest == Dest::ShapePictureValue => {
                let origin = if dest == Dest::ShapePictureValue {
                    PictureOrigin::Shape
                } else if self.frame.shape_picture {
                    PictureOrigin::ShapePicture
                } else if self.frame.object_result {
                    PictureOrigin::ObjectResult
                } else {
                    PictureOrigin::Bare
                };
                let id = self.new_picture(origin, PictureFormat::Unknown, dest);
                self.picture_nibble = None;
                self.set(Dest::Picture(id), Closes::Picture)
            }
            b"nisusfilename" | b"pdffilename" => match dest {
                Dest::Picture(id) => self.set(Dest::PictureName(id), Closes::Nothing),
                _ => self.set(Dest::Skip, Closes::Nothing),
            },
            // A PDF written whole into the text, as Scrivener 2 for macOS writes one: its name
            // in `\*\pdffilename`, its bytes in hexadecimal as a picture's are. Read as a
            // picture rather than skipped as an unknown destination, so that the text keeps
            // where it stood and what it was called.
            b"scrivenerpdf" if text => {
                let id = self.new_picture(PictureOrigin::EmbeddedPdf, PictureFormat::Pdf, dest);
                self.picture_nibble = None;
                self.set(Dest::Picture(id), Closes::Picture)
            }
            b"NeXTGraphic" if text => {
                let id = self.new_picture(
                    PictureOrigin::NextAttachment,
                    PictureFormat::NextAttachment,
                    dest,
                );
                self.set(Dest::NextAttachment(id), Closes::NextAttachment)
            }
            b"listtext" | b"pntext" if text => self.set(Dest::ListLabel, Closes::Nothing),
            b"pn" => {
                self.frame.para.legacy_numbering = Some(LegacyNumbering {
                    level: 0,
                    bullet: false,
                    continuation: false,
                });
                self.set(Dest::LegacyNumbering, Closes::LegacyNumbering)
            }
            b"footnote" if dest == Dest::Body => {
                let note = self.doc.notes.len();
                self.doc.notes.push(Note {
                    kind: NoteKind::Footnote,
                    blocks: Vec::new(),
                });
                self.grow(size_of::<Note>());
                self.push_inline(Inline::NoteReference(note));
                let story = self.new_story(Owner::Note(note));
                self.enter_story(story);
                self.set(Dest::Body, Closes::Note { story, note })
            }
            b"annotation" if dest == Dest::Body => {
                let annotation = self.doc.annotations.len();
                let parts = std::mem::take(&mut self.pending_annotation);
                self.doc.annotations.push(Annotation {
                    initials: parts.initials,
                    author: parts.author,
                    time: parts.time,
                    ..Annotation::default()
                });
                self.grow(size_of::<Annotation>());
                self.push_inline(Inline::AnnotationReference(annotation));
                let story = self.new_story(Owner::Annotation(annotation));
                self.enter_story(story);
                self.set(Dest::Body, Closes::Annotation { story, annotation })
            }
            b"atnid" => self.collect(Collector::Meta(MetaField::Initials)),
            b"atnauthor" => self.collect(Collector::Meta(MetaField::Author)),
            b"atndate" => self.collect(Collector::Meta(MetaField::Date)),
            b"atnref" => self.collect(Collector::Meta(MetaField::Reference)),
            b"atnparent" => self.collect(Collector::Meta(MetaField::Parent)),
            b"atntime" => {
                self.pending_time = RtfTime::default();
                self.set(Dest::AnnotationTime, Closes::AnnotationTime)
            }
            b"atrfstart" => self.collect(Collector::RangeStart),
            b"atrfend" => self.collect(Collector::RangeEnd),
            b"nesttableprops" => self.set(Dest::NestTableProps, Closes::Nothing),
            b"upr" => {
                self.frame.outer = dest;
                self.set(Dest::Upr, Closes::Nothing)
            }
            b"ud" if dest == Dest::Upr => {
                self.frame.dest = self.frame.outer;
                true
            }
            b"shp" if text => {
                self.frame.outer = dest;
                let pictures_before = self.doc.pictures.len();
                self.set(Dest::Shape, Closes::Shape { pictures_before })
            }
            b"shpinst" if dest == Dest::Shape => self.set(Dest::ShapeInstance, Closes::Nothing),
            b"sp" if dest == Dest::ShapeInstance => {
                self.shape_property.clear();
                self.set(Dest::ShapeProperty, Closes::Nothing)
            }
            b"sn" if dest == Dest::ShapeProperty => self.collect(Collector::ShapePropertyName),
            b"sv" if dest == Dest::ShapeProperty => {
                if self.shape_property == "pib" {
                    self.set(Dest::ShapePictureValue, Closes::Nothing)
                } else {
                    self.set(Dest::Skip, Closes::Nothing)
                }
            }
            b"shptxt" => {
                self.doc.diagnostics.add(Diagnostic::TextBoxSkipped, 1);
                self.set(Dest::Skip, Closes::Nothing)
            }
            b"object" if text => {
                self.frame.outer = dest;
                let results_before = self.object_results;
                self.set(Dest::Object, Closes::Object { results_before })
            }
            b"result" if dest == Dest::Object => {
                self.object_results += 1;
                self.frame.dest = self.frame.outer;
                self.frame.object_result = true;
                true
            }
            b"header" | b"headerl" | b"headerr" | b"headerf" | b"footer" | b"footerl"
            | b"footerr" | b"footerf" => {
                self.doc
                    .diagnostics
                    .add(Diagnostic::HeaderOrFooterSkipped, 1);
                self.set(Dest::Skip, Closes::Nothing)
            }
            // A style sheet's character, section and table styles open with `\*`; they are
            // entries all the same.
            b"cs" | b"ds" | b"ts" | b"s" if dest == Dest::StyleEntry => {
                self.style_word(name, param);
                true
            }
            _ if NON_BODY.contains(&name) => self.set(Dest::Skip, Closes::Nothing),
            _ => false,
        }
    }

    /// Words whose meaning depends on the destination they are read in. False when this one
    /// has none here.
    fn destination_word(&mut self, name: &[u8], param: Option<i32>) -> bool {
        match self.frame.dest {
            Dest::FontTable => self.font_word(name, param),
            Dest::ColorTable => {
                let value = u8::try_from(param.unwrap_or(0).clamp(0, 255)).unwrap_or(0);
                let slot = match name {
                    b"red" => 0,
                    b"green" => 1,
                    b"blue" => 2,
                    _ => return false,
                };
                self.color.rgb[slot] = value;
                self.color.stated = true;
                true
            }
            Dest::StyleEntry => self.style_word(name, param),
            Dest::List => {
                if name == b"listid" {
                    if let Some(list) = self.doc.lists.last_mut() {
                        list.id = param.unwrap_or(0);
                    }
                    return true;
                }
                false
            }
            Dest::ListLevel => {
                let level = self
                    .doc
                    .lists
                    .last_mut()
                    .and_then(|list| list.levels.last_mut());
                let Some(level) = level else {
                    return false;
                };
                match name {
                    b"levelnfc" | b"levelnfcn" => level.number_format = param,
                    b"levelstartat" => level.start_at = param,
                    _ => return false,
                }
                true
            }
            Dest::ListOverride => {
                let Some(entry) = self.doc.list_overrides.last_mut() else {
                    return false;
                };
                match name {
                    b"listid" => entry.list_id = param.unwrap_or(0),
                    b"ls" => entry.number = param.unwrap_or(0),
                    _ => return false,
                }
                true
            }
            Dest::Picture(id) => {
                // Nothing inside a picture changes the text around it.
                if let Some(picture) = self.doc.pictures.get_mut(id) {
                    picture_word(picture, name, param);
                }
                true
            }
            Dest::NextAttachment(id) => {
                let Some(picture) = self.doc.pictures.get_mut(id) else {
                    return false;
                };
                match name {
                    b"width" => picture.goal_width = param,
                    b"height" => picture.goal_height = param,
                    _ => return false,
                }
                true
            }
            Dest::AnnotationTime => {
                let time = &mut self.pending_time;
                match name {
                    b"yr" => time.year = param,
                    b"mo" => time.month = param,
                    b"dy" => time.day = param,
                    b"hr" => time.hour = param,
                    b"min" => time.minute = param,
                    _ => return false,
                }
                true
            }
            Dest::LegacyNumbering => {
                let numbering = self
                    .frame
                    .para
                    .legacy_numbering
                    .get_or_insert(LegacyNumbering {
                        level: 0,
                        bullet: false,
                        continuation: false,
                    });
                match name {
                    b"pnlvl" => {
                        let level = param.unwrap_or(1).clamp(1, 9) - 1;
                        numbering.level = u8::try_from(level).unwrap_or(0);
                    }
                    b"pnlvlblt" => numbering.bullet = true,
                    b"pnlvlbody" => numbering.bullet = false,
                    b"pnlvlcont" => numbering.continuation = true,
                    _ => return false,
                }
                true
            }
            Dest::NestTableProps => {
                if name == b"nestrow" {
                    self.row(2);
                }
                // A nested row's properties say nothing about its text.
                true
            }
            _ => false,
        }
    }

    fn font_word(&mut self, name: &[u8], param: Option<i32>) -> bool {
        if name == b"f" {
            self.finish_font();
            self.font = Some(Font {
                number: param.unwrap_or(0),
                family: FontFamily::Nil,
                charset: None,
                code_page: None,
                name: String::new(),
            });
            return true;
        }
        let Some(font) = self.font.as_mut() else {
            return false;
        };
        match name {
            b"fnil" => font.family = FontFamily::Nil,
            b"froman" => font.family = FontFamily::Roman,
            b"fswiss" => font.family = FontFamily::Swiss,
            b"fmodern" => font.family = FontFamily::Modern,
            b"fscript" => font.family = FontFamily::Script,
            b"fdecor" => font.family = FontFamily::Decor,
            b"ftech" => font.family = FontFamily::Tech,
            b"fbidi" => font.family = FontFamily::Bidi,
            b"fcharset" => font.charset = param,
            b"cpg" => {
                font.code_page = param
                    .and_then(|p| u16::try_from(p).ok())
                    .filter(|p| *p != 0);
            }
            _ => return false,
        }
        true
    }

    fn style_word(&mut self, name: &[u8], param: Option<i32>) -> bool {
        let Some(style) = self.style.as_mut() else {
            return false;
        };
        match name {
            b"s" => {
                style.kind = StyleKind::Paragraph;
                style.number = param.unwrap_or(0);
            }
            b"cs" => {
                style.kind = StyleKind::Character;
                style.number = param.unwrap_or(0);
            }
            b"ds" => {
                style.kind = StyleKind::Section;
                style.number = param.unwrap_or(0);
            }
            b"ts" => {
                style.kind = StyleKind::Table;
                style.number = param.unwrap_or(0);
            }
            b"sbasedon" => style.based_on = param,
            b"snext" => style.next = param,
            b"outlinelevel" => style.outline_level = outline_level(param),
            _ => return false,
        }
        true
    }

    fn common_word(&mut self, name: &[u8], param: Option<i32>) {
        let on = param != Some(0);
        let body = self.frame.dest == Dest::Body;
        match name {
            // The document.
            b"rtf" => self.doc.header.version = param,
            b"ansi" => self.set_charset(Charset::Ansi, 1252),
            b"mac" => self.set_charset(Charset::Mac, 10000),
            b"pc" => self.set_charset(Charset::Pc, 437),
            b"pca" => self.set_charset(Charset::Pca, 850),
            b"ansicpg" => {
                if let Some(page) = param
                    .and_then(|p| u16::try_from(p).ok())
                    .filter(|p| *p != 0)
                {
                    self.doc.header.code_page = page;
                }
            }
            b"deff" => self.doc.header.default_font = param,
            b"cocoartf" => self.doc.header.cocoa_version = param,
            b"uc" => {
                self.frame.uc = u32::try_from(param.unwrap_or(1).max(0)).unwrap_or(0);
            }
            b"u" => self.unicode(param.unwrap_or(0)),

            // Marks and special characters.
            b"par" if body => self.end_paragraph(ParagraphEnd::Paragraph, None),
            b"sect" if body => self.end_paragraph(ParagraphEnd::Section, None),
            b"line" if body => self.push_inline(Inline::Break(BreakKind::Line)),
            b"page" if body => self.push_inline(Inline::Break(BreakKind::Page)),
            b"column" if body => self.push_inline(Inline::Break(BreakKind::Column)),
            b"cell" if body => self.cell(1),
            b"nestcell" if body => self.cell(2),
            b"row" if body => self.row(1),
            b"nestrow" if body => self.row(2),
            b"tab" => self.route_char('\t'),
            b"emdash" => self.route_char('\u{2014}'),
            b"endash" => self.route_char('\u{2013}'),
            b"emspace" => self.route_char('\u{2003}'),
            b"enspace" => self.route_char('\u{2002}'),
            b"qmspace" => self.route_char('\u{2005}'),
            b"bullet" => self.route_char('\u{2022}'),
            b"lquote" => self.route_char('\u{2018}'),
            b"rquote" => self.route_char('\u{2019}'),
            b"ldblquote" => self.route_char('\u{201C}'),
            b"rdblquote" => self.route_char('\u{201D}'),
            b"zwj" => self.route_char('\u{200D}'),
            b"zwnj" => self.route_char('\u{200C}'),
            b"ltrmark" => self.route_char('\u{200E}'),
            b"rtlmark" => self.route_char('\u{200F}'),
            b"zwbo" => self.route_char('\u{200B}'),
            b"zwnbo" => self.route_char('\u{2060}'),
            b"chdate" | b"chdpl" | b"chdpa" | b"chtime" | b"chpgn" | b"sectnum" if body => {
                self.doc
                    .diagnostics
                    .add(Diagnostic::DynamicCharacterSkipped, 1);
            }
            b"ftnalt" => {
                if let Some(Owner::Note(i)) = self.owners.get(self.frame.story).copied()
                    && let Some(note) = self.doc.notes.get_mut(i)
                {
                    note.kind = NoteKind::Endnote;
                }
            }

            // Character formatting.
            b"plain" => self.frame.chars = CharProps::default(),
            b"b" => self.frame.chars.bold = on,
            b"i" => self.frame.chars.italic = on,
            b"ul" | b"uld" | b"uldash" | b"uldashd" | b"uldashdd" | b"uldb" | b"ulhwave"
            | b"ulldash" | b"ulth" | b"ulthd" | b"ulthdash" | b"ulthdashd" | b"ulthdashdd"
            | b"ulthldash" | b"ululdbwave" | b"ulw" | b"ulwave" => {
                self.frame.chars.underline = on;
            }
            b"ulnone" => self.frame.chars.underline = false,
            b"strike" | b"striked" => self.frame.chars.strike = on,
            b"super" => self.set_vertical(on, VerticalPosition::Superscript),
            b"sub" => self.set_vertical(on, VerticalPosition::Subscript),
            b"nosupersub" => self.frame.chars.vertical = VerticalPosition::Baseline,
            // `\up` and `\dn` share one axis with `\super` and `\sub`: `\up0` puts a
            // subscript back on the baseline too.
            b"up" => self.set_vertical(param.unwrap_or(6) > 0, VerticalPosition::Superscript),
            b"dn" => self.set_vertical(param.unwrap_or(6) > 0, VerticalPosition::Subscript),
            b"scaps" => self.frame.chars.small_caps = on,
            b"caps" => self.frame.chars.all_caps = on,
            b"v" => self.frame.chars.hidden = on,
            b"f" => self.frame.chars.font = param,
            b"fs" => {
                self.frame.chars.font_size = param.and_then(|p| u16::try_from(p).ok());
            }
            b"cf" => self.frame.chars.color = color_index(param),
            b"cb" | b"highlight" | b"chcbpat" => self.frame.chars.background = color_index(param),
            b"rtlch" => self.frame.chars.right_to_left = true,
            b"ltrch" => self.frame.chars.right_to_left = false,

            // Paragraph formatting.
            b"pard" => {
                self.frame.para = ParagraphProps::default();
                self.frame.list_level = 0;
            }
            b"ql" => self.frame.para.alignment = Some(Alignment::Left),
            b"qc" => self.frame.para.alignment = Some(Alignment::Center),
            b"qr" => self.frame.para.alignment = Some(Alignment::Right),
            b"qj" | b"qd" | b"qk" | b"qt" => {
                self.frame.para.alignment = Some(Alignment::Justify);
            }
            b"qnatural" => self.frame.para.alignment = Some(Alignment::Natural),
            b"li" | b"lin" => self.frame.para.left_indent = param.unwrap_or(0),
            b"ri" | b"rin" => self.frame.para.right_indent = param.unwrap_or(0),
            b"fi" => self.frame.para.first_line_indent = param.unwrap_or(0),
            b"sb" => self.frame.para.space_before = param.unwrap_or(0),
            b"sa" => self.frame.para.space_after = param.unwrap_or(0),
            b"sl" => self.frame.para.line_spacing = param.unwrap_or(0),
            b"slmult" => self.frame.para.line_spacing_multiple = param == Some(1),
            b"pagebb" => self.frame.para.page_break_before = on,
            b"keepn" => self.frame.para.keep_with_next = on,
            b"intbl" => self.frame.para.in_table = on,
            b"itap" => {
                self.frame.para.table_nesting = param
                    .map(|p| p.clamp(0, i32::from(u16::MAX)))
                    .and_then(|p| u16::try_from(p).ok());
            }
            b"ls" => {
                self.frame.para.list = param.map(|override_number| ListRef {
                    override_number,
                    level: self.frame.list_level,
                });
            }
            b"ilvl" => {
                let level = u8::try_from(param.unwrap_or(0).clamp(0, 255)).unwrap_or(0);
                self.frame.list_level = level;
                if let Some(list) = self.frame.para.list.as_mut() {
                    list.level = level;
                }
            }
            b"s" => self.frame.para.style = param,
            b"outlinelevel" => self.frame.para.outline_level = outline_level(param),
            b"rtlpar" => self.frame.para.direction = Some(Direction::RightToLeft),
            b"ltrpar" => self.frame.para.direction = Some(Direction::LeftToRight),
            _ => {}
        }
    }

    fn set_charset(&mut self, charset: Charset, code_page: u16) {
        self.doc.header.charset = charset;
        self.doc.header.code_page = code_page;
    }

    /// Raise or lower the text, or with `on` false put it back on the baseline, from either
    /// side: one axis carries both.
    fn set_vertical(&mut self, on: bool, position: VerticalPosition) {
        self.frame.chars.vertical = if on {
            position
        } else {
            VerticalPosition::Baseline
        };
    }

    // ── symbols, bytes and text ──────────────────────────────────────────────

    fn symbol(&mut self, symbol: u8) {
        if self.frame.dest == Dest::Skip {
            return;
        }
        if self.skip_chars > 0 {
            self.skip_chars -= 1;
            return;
        }
        match symbol {
            b'\\' | b'{' | b'}' => {
                if self.frame.dest.accepts_text() {
                    self.buffer(&[symbol]);
                }
            }
            b'~' => self.route_char('\u{A0}'),
            b'-' => self.route_char('\u{AD}'),
            b'_' => self.route_char('\u{2011}'),
            // `\:` (an index subentry), `\|` (a formula) and unknown symbols carry no text.
            _ => {}
        }
    }

    fn hex(&mut self, byte: u8) {
        if self.frame.dest == Dest::Skip {
            return;
        }
        if self.skip_chars > 0 {
            self.skip_chars -= 1;
            return;
        }
        if self.frame.dest.accepts_text() {
            self.buffer(&[byte]);
        }
    }

    fn text(&mut self, bytes: &[u8], attachment: bool) {
        let mut bytes = bytes;
        if attachment && bytes.first() == Some(&0xAC) {
            bytes = bytes.get(1..).unwrap_or_default();
        }
        match self.frame.dest {
            Dest::Picture(id) => {
                self.picture_hex(id, bytes);
                return;
            }
            dest if !dest.accepts_text() => return,
            _ => {}
        }
        while self.skip_chars > 0 && !bytes.is_empty() {
            self.skip_chars -= 1;
            // `\ucN` counts bytes of a code page; text already in UTF-8 counts characters.
            let width = if self.utf8_text {
                bytes.first().copied().map_or(1, utf8_width)
            } else {
                1
            };
            bytes = bytes.get(width.min(bytes.len())..).unwrap_or_default();
        }
        if bytes.is_empty() {
            return;
        }
        if self.utf8_text {
            self.flush();
            let text = String::from_utf8_lossy(bytes);
            if let std::borrow::Cow::Owned(owned) = &text {
                let replaced = owned.chars().filter(|c| *c == '\u{FFFD}').count();
                self.doc
                    .diagnostics
                    .add(Diagnostic::UndecodableBytes, replaced);
            }
            self.route(&text);
        } else {
            self.buffer(bytes);
        }
    }

    fn bin(&mut self, declared: i64, data: &[u8]) {
        let wanted = u64::try_from(declared.max(0)).unwrap_or(0);
        if (data.len() as u64) < wanted {
            self.doc.diagnostics.add(Diagnostic::BinaryTruncated, 1);
        }
        if self.frame.dest == Dest::Skip {
            return;
        }
        // The whole of `\binN` and its data is one character for `\uN` to skip.
        if self.skip_chars > 0 {
            self.skip_chars -= 1;
            return;
        }
        match self.frame.dest {
            // A length past the limit drops the picture rather than refusing the file: the
            // lexer has already stepped over the data (it is a slice of the input, never
            // copied), so skipping it costs nothing, and the text around it is kept.
            Dest::Picture(id)
                if wanted > u64::try_from(self.limits.max_binary_bytes).unwrap_or(u64::MAX) =>
            {
                self.drop_picture(id);
            }
            Dest::Picture(id) => {
                for &byte in data {
                    if !self.picture_byte(id, byte) {
                        return;
                    }
                }
            }
            _ => self
                .doc
                .diagnostics
                .add(Diagnostic::BinaryOutsidePicture, 1),
        }
    }

    fn picture_hex(&mut self, id: usize, bytes: &[u8]) {
        let mut not_hex = 0usize;
        for &byte in bytes {
            let digit = match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                b'A'..=b'F' => byte - b'A' + 10,
                b' ' | b'\t' => continue,
                _ => {
                    not_hex += 1;
                    continue;
                }
            };
            match self.picture_nibble.take() {
                None => self.picture_nibble = Some(digit),
                Some(high) => {
                    if !self.picture_byte(id, high << 4 | digit) {
                        return;
                    }
                }
            }
        }
        self.doc
            .diagnostics
            .add(Diagnostic::PictureDataNotHex, not_hex);
    }

    /// Append one byte of picture data. False once the picture has passed its limit, which
    /// drops it ([`Self::drop_picture`]).
    fn picture_byte(&mut self, id: usize, byte: u8) -> bool {
        let Some(picture) = self.doc.pictures.get_mut(id) else {
            return false;
        };
        if picture.oversized {
            return false;
        }
        if picture.data.len() >= self.limits.max_picture_bytes {
            self.drop_picture(id);
            return false;
        }
        picture.data.push(byte);
        self.doc.stats.picture_bytes += 1;
        true
    }

    /// Drop a picture past its limit: its bytes go, its place and its description stay, and
    /// it is counted once. What follows it in the file is read as usual.
    fn drop_picture(&mut self, id: usize) {
        let Some(picture) = self.doc.pictures.get_mut(id) else {
            return;
        };
        if picture.oversized {
            return;
        }
        picture.oversized = true;
        self.doc.stats.picture_bytes = self
            .doc
            .stats
            .picture_bytes
            .saturating_sub(picture.data.len());
        // Released now rather than when the model is: a picture past the limit is the one
        // that would hold the most memory.
        picture.data = Vec::new();
        self.doc.diagnostics.add(Diagnostic::PictureTooLarge, 1);
    }

    /// Buffer 8-bit text for decoding with the code page it is in.
    fn buffer(&mut self, bytes: &[u8]) {
        let code_page = self.current_code_page();
        if !self.pending.is_empty() && code_page != self.pending_code_page {
            self.flush();
        }
        self.pending_code_page = code_page;
        self.pending.extend_from_slice(bytes);
    }

    /// Decode the buffered bytes and route their text.
    fn flush(&mut self) {
        if self.pending.is_empty() {
            return;
        }
        let bytes = std::mem::take(&mut self.pending);
        let mut text = String::with_capacity(bytes.len());
        let found = codepage::decode(&bytes, self.pending_code_page, &mut text);
        self.doc
            .diagnostics
            .add(Diagnostic::UndecodableBytes, found.undecodable);
        self.doc.diagnostics.add(
            Diagnostic::UnsupportedCodePage(self.pending_code_page),
            found.unsupported,
        );
        // Kept for the next buffer, so a long run of short escapes reuses one allocation.
        self.pending = bytes;
        self.pending.clear();
        self.route(&text);
    }

    fn current_code_page(&self) -> u16 {
        let document = self.doc.header.code_page;
        if self.frame.dest == Dest::FontTable {
            return self
                .font
                .as_ref()
                .and_then(font_code_page)
                .unwrap_or(document);
        }
        self.frame
            .chars
            .font
            .or(self.doc.header.default_font)
            .and_then(|font| self.font_pages.get(&font).copied())
            .unwrap_or(document)
    }

    fn lead_bytes(&self) -> Option<LeadBytes> {
        if self.utf8_text || !self.frame.dest.accepts_text() {
            return None;
        }
        codepage::lead_bytes(self.current_code_page())
    }

    fn unicode(&mut self, value: i32) {
        // Values above 32767 may be written as negative numbers, the signed 16-bit reading of
        // the same code unit (RTF 1.9.1, page 14).
        let code = if value < 0 {
            (value >= -32768)
                .then(|| value.unsigned_abs())
                .map(|magnitude| 65536 - magnitude)
        } else {
            u32::try_from(value).ok()
        };
        match code {
            Some(unit @ 0xD800..=0xDBFF) => {
                let high = u16::try_from(unit).unwrap_or(0xD800);
                if self.high_surrogate.replace(high).is_some() {
                    self.doc.diagnostics.add(Diagnostic::LoneSurrogate, 1);
                    self.route_raw("\u{FFFD}");
                }
            }
            Some(unit @ 0xDC00..=0xDFFF) => match self.high_surrogate.take() {
                Some(high) => {
                    let joined = 0x10000 + ((u32::from(high) - 0xD800) << 10) + (unit - 0xDC00);
                    let c = char::from_u32(joined).unwrap_or('\u{FFFD}');
                    let mut buffer = [0u8; 4];
                    self.route_raw(c.encode_utf8(&mut buffer));
                }
                None => {
                    self.doc.diagnostics.add(Diagnostic::LoneSurrogate, 1);
                    self.route("\u{FFFD}");
                }
            },
            Some(unit) => match char::from_u32(unit) {
                Some(c) => {
                    let c = if self.current_code_page() == codepage::SYMBOL {
                        codepage::symbol_private_use(c)
                    } else {
                        c
                    };
                    self.route_char(c);
                }
                None => {
                    self.doc.diagnostics.add(Diagnostic::InvalidUnicode, 1);
                    self.route_char('\u{FFFD}');
                }
            },
            None => {
                self.doc.diagnostics.add(Diagnostic::InvalidUnicode, 1);
                self.route_char('\u{FFFD}');
            }
        }
        self.skip_chars = self.frame.uc;
    }

    fn route_char(&mut self, c: char) {
        self.flush();
        let mut buffer = [0u8; 4];
        self.route(c.encode_utf8(&mut buffer));
    }

    /// Route decoded text, writing a replacement character first for a high surrogate that
    /// never got its other half.
    fn route(&mut self, text: &str) {
        if self.high_surrogate.take().is_some() {
            self.doc.diagnostics.add(Diagnostic::LoneSurrogate, 1);
            self.route_raw("\u{FFFD}");
        }
        self.route_raw(text);
    }

    fn route_raw(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        match self.frame.dest {
            Dest::Body => self.story_text(text),
            Dest::ListLabel => {
                let clean = self.without_controls(text);
                if clean.is_empty() {
                    return;
                }
                self.grow(clean.len());
                let story = self.story_mut();
                story.label.get_or_insert_with(String::new).push_str(&clean);
                story.has_content = true;
            }
            Dest::FontTable => {
                self.grow(text.len());
                for c in text.chars() {
                    if c == ';' {
                        self.finish_font();
                    } else if let Some(font) = self.font.as_mut() {
                        font.name.push(c);
                    }
                }
            }
            Dest::ColorTable => {
                for c in text.chars() {
                    if c == ';' {
                        let entry = std::mem::take(&mut self.color);
                        let [red, green, blue] = entry.rgb;
                        self.doc
                            .colors
                            .push(entry.stated.then_some(Rgb { red, green, blue }));
                        self.grow(size_of::<Option<Rgb>>());
                    }
                }
            }
            Dest::StyleEntry => {
                self.grow(text.len());
                for c in text.chars() {
                    if c == ';' {
                        self.style_named = true;
                    } else if !self.style_named
                        && let Some(style) = self.style.as_mut()
                    {
                        style.name.push(c);
                    }
                }
            }
            Dest::FieldInstruction(id) => {
                self.grow(text.len());
                if let Some(field) = self.doc.fields.get_mut(id) {
                    field.instruction.push_str(text);
                }
            }
            Dest::PictureName(id) | Dest::NextAttachment(id) => {
                self.grow(text.len());
                if let Some(picture) = self.doc.pictures.get_mut(id) {
                    picture.name.get_or_insert_with(String::new).push_str(text);
                }
            }
            Dest::Collect(_) => {
                self.grow(text.len());
                self.collected.push_str(text);
            }
            _ => {}
        }
    }

    /// `text` without the control characters RTF text should not hold. The tab is text.
    fn without_controls(&mut self, text: &str) -> String {
        let is_control = |c: char| c < ' ' && c != '\t';
        if !text.chars().any(is_control) {
            return text.to_string();
        }
        let removed = text.chars().filter(|c| is_control(*c)).count();
        self.doc
            .diagnostics
            .add(Diagnostic::ControlCharacterRemoved, removed);
        text.chars().filter(|c| !is_control(*c)).collect()
    }

    fn story_text(&mut self, text: &str) {
        let clean = self.without_controls(text);
        if clean.is_empty() {
            return;
        }
        let chars = self.frame.chars;
        let group = self.frame.serial;
        let field = self.frame.field;
        let mut added = clean.len();
        let story = self.stories.get_mut(self.frame.story);
        let Some(story) = story else {
            return;
        };
        match story.inlines.last_mut() {
            Some(Inline::Text(run))
                if run.props == chars && run.group == group && run.field == field =>
            {
                run.text.push_str(&clean);
            }
            _ => {
                added += slot_bytes(&story.inlines);
                story.inlines.push(Inline::Text(Run {
                    text: clean,
                    props: chars,
                    group,
                    field,
                }));
            }
        }
        story.has_content = true;
        self.grow(added);
    }

    // ── stories, paragraphs, tables ──────────────────────────────────────────

    fn story_mut(&mut self) -> &mut Story {
        let index = self.frame.story.min(self.stories.len().saturating_sub(1));
        &mut self.stories[index]
    }

    fn new_story(&mut self, owner: Owner) -> usize {
        self.stories.push(Story::default());
        self.owners.push(owner);
        self.grow(size_of::<Story>() + size_of::<Owner>());
        self.stories.len() - 1
    }

    /// Point the current group at `story`. A note's or a comment's text does not take the
    /// formatting of the text around it (RTF 1.9.1, page 9).
    fn enter_story(&mut self, story: usize) {
        self.frame.story = story;
        self.frame.chars = CharProps::default();
        self.frame.para = ParagraphProps::default();
        self.frame.list_level = 0;
        self.frame.field = None;
    }

    fn finish_story(&mut self, story: usize, props: ParagraphProps) -> Vec<Block> {
        self.end_story_paragraph(story, props);
        match self.stories.get_mut(story) {
            Some(story) => std::mem::take(story).finish(props, 0),
            None => Vec::new(),
        }
    }

    /// End the paragraph a story is reading, if it has read anything, with `props`.
    fn end_story_paragraph(&mut self, story: usize, props: ParagraphProps) {
        let has_content = self.stories.get(story).is_some_and(|s| s.has_content);
        if !has_content {
            return;
        }
        let level = self.clamp_level(props.table_level());
        let bytes = match self.stories.get_mut(story) {
            Some(story) => story.end_paragraph(props, ParagraphEnd::Implicit, level),
            None => 0,
        };
        self.grow(bytes);
    }

    fn push_inline(&mut self, inline: Inline) {
        self.push_inline_to(self.frame.story, inline);
    }

    fn push_inline_to(&mut self, story: usize, inline: Inline) {
        self.flush();
        if self.high_surrogate.take().is_some() {
            self.doc.diagnostics.add(Diagnostic::LoneSurrogate, 1);
            self.route_raw("\u{FFFD}");
        }
        let bytes = match self.stories.get_mut(story) {
            Some(story) => {
                let slot = slot_bytes(&story.inlines);
                story.inlines.push(inline);
                story.has_content = true;
                slot
            }
            None => 0,
        };
        self.grow(bytes);
    }

    fn clamp_level(&mut self, level: usize) -> usize {
        if level > MAX_TABLE_NESTING {
            self.doc
                .diagnostics
                .add(Diagnostic::TableNestingFlattened, 1);
            MAX_TABLE_NESTING
        } else {
            level
        }
    }

    fn end_paragraph(&mut self, end: ParagraphEnd, level: Option<usize>) {
        if self.high_surrogate.take().is_some() {
            self.doc.diagnostics.add(Diagnostic::LoneSurrogate, 1);
            self.route_raw("\u{FFFD}");
        }
        let props = self.frame.para;
        let level = self.clamp_level(level.unwrap_or_else(|| props.table_level()));
        let bytes = self.story_mut().end_paragraph(props, end, level);
        self.grow(bytes);
    }

    /// `\cell` or `\nestcell`: the paragraph read so far ends its cell.
    fn cell(&mut self, least: usize) {
        let level = self.clamp_level(least.max(self.frame.para.table_level()));
        self.end_paragraph(ParagraphEnd::Cell, Some(level));
        let bytes = self.story_mut().close_cell(level);
        self.grow(bytes);
    }

    /// `\row` or `\nestrow`. A paragraph read since the last cell mark ends a cell first.
    fn row(&mut self, least: usize) {
        let level = if least == 1 {
            1
        } else {
            self.clamp_level(least.max(self.frame.para.table_level()))
        };
        if self.story_mut().has_content {
            self.end_paragraph(ParagraphEnd::Cell, Some(level));
            let bytes = self.story_mut().close_cell(level);
            self.grow(bytes);
        }
        let bytes = self.story_mut().close_row(level);
        self.grow(bytes);
    }

    // ── tables of the document ───────────────────────────────────────────────

    fn new_picture(&mut self, origin: PictureOrigin, format: PictureFormat, dest: Dest) -> usize {
        let id = self.doc.pictures.len();
        self.doc.pictures.push(Picture {
            format,
            origin,
            ..Picture::default()
        });
        self.grow(size_of::<Picture>());
        // A picture is shown where it is written in the text; one written inside a list
        // label (a picture bullet) is part of the label, which is not kept.
        let shown = match dest {
            Dest::Body => true,
            Dest::ShapePictureValue => self.frame.outer == Dest::Body,
            _ => false,
        };
        if shown {
            self.push_inline(Inline::Picture(id));
        }
        id
    }

    fn finish_font(&mut self) {
        let Some(mut font) = self.font.take() else {
            return;
        };
        font.name = font.name.trim().to_string();
        if let Some(page) = font_code_page(&font) {
            self.font_pages.insert(font.number, page);
        }
        self.grow(size_of::<Font>());
        self.doc.fonts.push(font);
    }

    fn finish_style(&mut self) {
        let Some(mut style) = self.style.take() else {
            return;
        };
        style.name = style.name.trim().to_string();
        self.grow(size_of::<Style>());
        self.doc.styles.push(style);
    }

    // ── limits and the end ───────────────────────────────────────────────────

    fn grow(&mut self, bytes: usize) {
        let stats = &mut self.doc.stats;
        stats.document_bytes = stats.document_bytes.saturating_add(bytes);
        if stats.document_bytes > self.limits.max_document_bytes && self.refusal.is_none() {
            self.refusal = Some(Refusal::DocumentTooLarge {
                limit: self.limits.max_document_bytes,
            });
        }
    }

    fn finish(mut self) -> Result<Document, Refusal> {
        if let Some(refusal) = self.refusal.take() {
            return Err(refusal);
        }
        self.flush();
        // Groups the input never closed are closed here, innermost first. Text read after
        // the last paragraph mark keeps the formatting in force where the input stopped, and
        // its paragraph ends only when the group holding its story closes: ended first, it
        // would leave the end of a field or of a comment's range still open to a paragraph of
        // its own after it.
        let stopped_in = self.frame.story;
        let stopped_props = self.frame.para;
        let unclosed = self.stack.len();
        self.doc
            .diagnostics
            .add(Diagnostic::UnclosedGroup, unclosed);
        while let Some(parent) = self.stack.pop() {
            let closed = std::mem::replace(&mut self.frame, parent);
            let finished = match closed.closes {
                Closes::Note { story, .. } | Closes::Annotation { story, .. } => Some(story),
                _ if self.stack.is_empty() => Some(0),
                _ => None,
            };
            match finished {
                Some(story) if story == stopped_in => {
                    self.end_story_paragraph(story, stopped_props);
                }
                // The document's own group: a note or a comment ends its own paragraph as it
                // closes.
                Some(0) => self.end_story_paragraph(0, closed.para),
                _ => {}
            }
            self.closing(closed);
        }
        if self.high_surrogate.take().is_some() {
            self.doc.diagnostics.add(Diagnostic::LoneSurrogate, 1);
        }
        self.finish_font();
        self.finish_style();
        let props = self.frame.para;
        let body = std::mem::take(&mut self.stories[0]).finish(props, 0);
        self.doc.body = body;
        for picture in &mut self.doc.pictures {
            if let Some(name) = picture.name.as_mut() {
                *name = name.trim().to_string();
            }
        }
        self.doc.stats.tokens = self.lexer.tokens;
        if let Some(refusal) = self.refusal.take() {
            return Err(refusal);
        }
        Ok(self.doc)
    }
}

/// The code page a font's text is in, when the font names one.
fn font_code_page(font: &Font) -> Option<u16> {
    font.code_page
        .or_else(|| font.charset.and_then(codepage::for_charset))
}

fn picture_word(picture: &mut Picture, name: &[u8], param: Option<i32>) {
    match name {
        b"pngblip" => picture.format = PictureFormat::Png,
        b"jpegblip" => picture.format = PictureFormat::Jpeg,
        b"emfblip" => picture.format = PictureFormat::Emf,
        b"wmetafile" => picture.format = PictureFormat::Wmf,
        b"macpict" => picture.format = PictureFormat::MacPict,
        b"dibitmap" => picture.format = PictureFormat::Dib,
        b"wbitmap" => picture.format = PictureFormat::Bitmap,
        b"pmmetafile" => picture.format = PictureFormat::Os2Metafile,
        b"picw" => picture.width = param,
        b"pich" => picture.height = param,
        b"picwgoal" => picture.goal_width = param,
        b"pichgoal" => picture.goal_height = param,
        b"picscalex" => picture.scale_x = param,
        b"picscaley" => picture.scale_y = param,
        _ => {}
    }
}

/// A colour table index from `\cfN` and its kind, `0` being "automatic".
fn color_index(param: Option<i32>) -> Option<u16> {
    param
        .and_then(|p| u16::try_from(p).ok())
        .filter(|index| *index != 0)
}

/// `\outlinelevelN`, 0 to 8. Word writes 9 for body text, which is no outline level.
fn outline_level(param: Option<i32>) -> Option<u8> {
    param
        .filter(|level| (0..=8).contains(level))
        .and_then(|level| u8::try_from(level).ok())
}

/// How many bytes the UTF-8 character starting with `first` takes.
fn utf8_width(first: u8) -> usize {
    match first {
        0xF0..=0xF7 => 4,
        0xE0..=0xEF => 3,
        0xC0..=0xDF => 2,
        _ => 1,
    }
}
