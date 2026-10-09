// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The document model a reading produces.
//!
//! Plain data, every field public, so that a layer above can read it and also rewrite it
//! before mapping it onward (see the crate documentation for what such a layer is given).
//! Positions are never character offsets: a footnote, a picture, a field boundary or a
//! comment's range is an item in the paragraph's inline stream, where it stays right however
//! the text around it is later cut or joined.
//!
//! Indices (`usize` in [`Inline`], [`Run::field`]) point into the vectors of the same
//! [`Document`]: [`Document::fields`], [`Document::pictures`], [`Document::notes`] and
//! [`Document::annotations`].

use crate::diagnostics::Diagnostics;

/// Everything read from one RTF stream.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Document {
    pub header: Header,
    pub info: Info,
    /// The font table, in the order the file defines it. Look a font up by its number with
    /// [`Document::font`]: a number is the file's own, not an index.
    pub fonts: Vec<Font>,
    /// The colour table. `None` is an entry that names no colour: the "automatic" colour a
    /// table conventionally opens with, which is what index `0` means in `\cfN` and `\cbN`.
    pub colors: Vec<Option<Rgb>>,
    pub styles: Vec<Style>,
    /// `\listtable`: the list definitions a `\lsN` paragraph reaches through
    /// [`Document::list_overrides`].
    pub lists: Vec<List>,
    /// `\listoverridetable`: which list each `\lsN` number names.
    pub list_overrides: Vec<ListOverride>,
    /// The main text, in reading order.
    pub body: Vec<Block>,
    pub notes: Vec<Note>,
    pub annotations: Vec<Annotation>,
    pub fields: Vec<Field>,
    pub pictures: Vec<Picture>,
    /// What the reading met and could not keep, or kept in a changed form, counted by kind.
    pub diagnostics: Diagnostics,
    pub stats: Stats,
}

impl Document {
    /// The font the file numbers `number`: the last definition, when a file defines a
    /// number twice.
    pub fn font(&self, number: i32) -> Option<&Font> {
        self.fonts.iter().rev().find(|f| f.number == number)
    }

    /// The colour at `index` in the colour table, `None` for an automatic entry or an index
    /// past the table.
    pub fn color(&self, index: u16) -> Option<Rgb> {
        self.colors.get(usize::from(index)).copied().flatten()
    }

    /// The paragraph style numbered `number` (`\sN`).
    pub fn paragraph_style(&self, number: i32) -> Option<&Style> {
        self.styles
            .iter()
            .rev()
            .find(|s| s.kind == StyleKind::Paragraph && s.number == number)
    }

    /// The list level a paragraph's `\lsN\ilvlM` names, through the override table.
    pub fn list_level(&self, list: ListRef) -> Option<&ListLevel> {
        let id = self
            .list_overrides
            .iter()
            .rev()
            .find(|o| o.number == list.override_number)?
            .list_id;
        self.lists
            .iter()
            .rev()
            .find(|l| l.id == id)?
            .levels
            .get(usize::from(list.level))
    }

    /// Whether the background colour at `index` shows. Index `0` is "none", and a white
    /// background is none as well: the macOS writer states `\cb1` with a white first colour
    /// on text nobody highlighted.
    pub fn background_shows(&self, index: u16) -> bool {
        index != 0
            && self.color(index).is_some_and(|c| {
                c != Rgb {
                    red: 255,
                    green: 255,
                    blue: 255,
                }
            })
    }
}

/// What the file says about itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// The `N` of `\rtfN`.
    pub version: Option<i32>,
    pub charset: Charset,
    /// The code page 8-bit text is read in where no font says otherwise: `\ansicpgN`, or
    /// the one the character set implies.
    pub code_page: u16,
    /// `\deffN`: the font text is in when nothing selects one.
    pub default_font: Option<i32>,
    /// `\cocoartfN`: written by macOS's text system. A hint about the writer, never a
    /// switch: one project can hold files from several writers.
    pub cocoa_version: Option<i32>,
    /// `\*\generator`: the writing application, when it names itself.
    pub generator: Option<String>,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            version: None,
            charset: Charset::Ansi,
            code_page: 1252,
            default_font: None,
            cocoa_version: None,
            generator: None,
        }
    }
}

/// The document's character set declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Charset {
    /// `\ansi`, the default.
    #[default]
    Ansi,
    /// `\mac`: Mac Roman.
    Mac,
    /// `\pc`: code page 437.
    Pc,
    /// `\pca`: code page 850.
    Pca,
}

/// `\info`: the document's own metadata, the fields a manuscript might use.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Info {
    pub title: Option<String>,
    pub subject: Option<String>,
    pub author: Option<String>,
    pub keywords: Option<String>,
    /// `\doccomm`.
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Font {
    /// The `N` of `\fN`.
    pub number: i32,
    pub family: FontFamily,
    /// `\fcharsetN`, as written.
    pub charset: Option<i32>,
    /// The code page this font's 8-bit text is read in: `\cpgN`, else the one its character
    /// set names. `None` means the document's own code page.
    pub code_page: Option<u16>,
    pub name: String,
}

/// A font's family, as the table declares it.
///
/// Declared, not true: Scrivener for Windows declares every font `\fmodern`, so a family is
/// never a reason to read text as code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontFamily {
    #[default]
    Nil,
    Roman,
    Swiss,
    Modern,
    Script,
    Decor,
    Tech,
    Bidi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Style {
    pub number: i32,
    pub kind: StyleKind,
    pub name: String,
    /// `\sbasedonN`.
    pub based_on: Option<i32>,
    /// `\snextN`.
    pub next: Option<i32>,
    /// `\outlinelevelN` in the style's definition: 0 is the top level.
    pub outline_level: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleKind {
    /// `\sN`.
    Paragraph,
    /// `\*\csN`.
    Character,
    /// `\dsN`.
    Section,
    /// `\*\tsN`.
    Table,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct List {
    /// `\listidN`.
    pub id: i32,
    pub levels: Vec<ListLevel>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListLevel {
    /// `\levelnfcN`: 0 decimal, 1 and 2 roman, 3 and 4 letters, 23 a bullet, 255 no number
    /// at all.
    pub number_format: Option<i32>,
    /// `\levelstartatN`.
    pub start_at: Option<i32>,
    /// macOS's `\*\levelmarker`, such as `{decimal}.` or `{disc}`.
    pub marker: Option<String>,
}

impl ListLevel {
    /// Whether the level numbers its items, as far as the file says: `Some(false)` for a
    /// bullet, `None` when the level states neither a format nor a marker.
    pub fn is_numbered(&self) -> Option<bool> {
        if let Some(format) = self.number_format {
            return Some(!matches!(format, 23 | 255));
        }
        let marker = self.marker.as_deref()?;
        let name = marker
            .split(['{', '}'])
            .nth(1)
            .unwrap_or(marker)
            .to_ascii_lowercase();
        Some(!matches!(
            name.as_str(),
            "disc" | "circle" | "square" | "hyphen" | "check" | "diamond" | "box"
        ))
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ListOverride {
    /// The list it names: a [`List::id`].
    pub list_id: i32,
    /// The `N` a paragraph's `\lsN` uses.
    pub number: i32,
}

/// One block of a story: a paragraph or a table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Paragraph(Paragraph),
    Table(Table),
}

/// Rows of cells, each cell holding blocks of its own, a nested table included.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Table {
    pub rows: Vec<Row>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Row {
    pub cells: Vec<Cell>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cell {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Paragraph {
    /// The paragraph formatting in force at the mark that ended it, which is the formatting
    /// RTF gives the paragraph.
    pub props: ParagraphProps,
    /// The list number or bullet the writer stored in `\listtext` or `\pntext`, for readers
    /// that do not build lists. Tabs included, as written.
    pub label: Option<String>,
    pub inlines: Vec<Inline>,
    pub end: ParagraphEnd,
}

impl Paragraph {
    /// The paragraph's text runs, in order.
    pub fn runs(&self) -> impl Iterator<Item = &Run> {
        self.inlines.iter().filter_map(|inline| match inline {
            Inline::Text(run) => Some(run),
            _ => None,
        })
    }
}

/// What ended a paragraph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParagraphEnd {
    /// `\par`, or a backslash before a line end.
    #[default]
    Paragraph,
    /// `\sect`.
    Section,
    /// `\cell` or `\nestcell`: the last paragraph of a table cell.
    Cell,
    /// Nothing: the story ended with text after its last mark.
    Implicit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ParagraphProps {
    /// `None` until the paragraph states one: RTF's default is flush left, but a
    /// right-to-left paragraph starts at the other edge, so "not stated" is kept apart from
    /// `\ql`.
    pub alignment: Option<Alignment>,
    /// `\rtlpar` / `\ltrpar`, when stated.
    pub direction: Option<Direction>,
    /// `\liN`, in twips.
    pub left_indent: i32,
    /// `\riN`, in twips.
    pub right_indent: i32,
    /// `\fiN`, in twips; negative for a hanging indent.
    pub first_line_indent: i32,
    /// `\sbN`, in twips.
    pub space_before: i32,
    /// `\saN`, in twips.
    pub space_after: i32,
    /// `\slN`, in twips, read with [`Self::line_spacing_multiple`].
    pub line_spacing: i32,
    /// `\slmult1`: the line spacing is a multiple of single spacing.
    pub line_spacing_multiple: bool,
    /// `\pagebb`.
    pub page_break_before: bool,
    /// `\keepn`.
    pub keep_with_next: bool,
    /// `\outlinelevelN`: 0 is the top level.
    pub outline_level: Option<u8>,
    /// `\sN`: the paragraph style, a [`Style::number`].
    pub style: Option<i32>,
    /// `\lsN` and `\ilvlN`.
    pub list: Option<ListRef>,
    /// Word 95's numbering, `\*\pn`.
    pub legacy_numbering: Option<LegacyNumbering>,
    /// `\intbl`: the paragraph is in a table.
    pub in_table: bool,
    /// `\itapN`: the table nesting level, 0 for the main text.
    pub table_nesting: Option<u16>,
}

impl ParagraphProps {
    /// How deep in tables the paragraph sits: 0 for the main text, 1 for a table, 2 for a
    /// table nested in a cell.
    pub fn table_level(&self) -> usize {
        match (self.table_nesting, self.in_table) {
            (Some(0), true) | (None, true) => 1,
            (Some(level), _) => usize::from(level),
            (None, false) => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    /// `\qj`, and the distributed forms `\qd`, `\qk` and `\qt`.
    Justify,
    /// macOS's `\qnatural`: the start edge of the paragraph's direction.
    Natural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListRef {
    /// The `N` of `\lsN`, a [`ListOverride::number`].
    pub override_number: i32,
    /// The `N` of `\ilvlN`: 0 is the outermost level.
    pub level: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyNumbering {
    /// 0-based.
    pub level: u8,
    /// `\pnlvlblt`.
    pub bullet: bool,
    /// `\pnlvlcont`: a paragraph that continues the item above, with no number of its own.
    pub continuation: bool,
}

/// One item of a paragraph, in reading order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inline {
    Text(Run),
    Break(BreakKind),
    /// A picture: an index into [`Document::pictures`].
    Picture(usize),
    /// Where a footnote or an endnote is cited: an index into [`Document::notes`].
    NoteReference(usize),
    /// Where a comment (`\annotation`) is anchored: an index into [`Document::annotations`].
    AnnotationReference(usize),
    /// `\*\atrfstart`: the start of the text a comment covers, named by the tag its
    /// [`Annotation::reference`] repeats.
    AnnotationRangeStart(String),
    /// `\*\atrfend`: where that text ends.
    AnnotationRangeEnd(String),
    /// A field opens here: an index into [`Document::fields`]. Its result, if it has one,
    /// follows as runs carrying the same [`Run::field`].
    FieldStart(usize),
    /// The field closes here.
    FieldEnd(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakKind {
    /// `\line`: a new line inside the paragraph.
    Line,
    /// `\page`.
    Page,
    /// `\column`.
    Column,
}

/// A stretch of text and the character formatting it was written with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    /// Decoded text. Escaped braces and backslashes are literal characters here, tabs are
    /// `'\t'`, and control characters other than the tab are removed (and counted).
    pub text: String,
    pub props: CharProps,
    /// The serial number of the RTF group the text was read in. Adjacent text is one run only
    /// when it shares a group, formatting and field, so text a writer split across groups
    /// stays split, and a layer above can tell where each part was written.
    pub group: u32,
    /// The innermost field whose result this text is: an index into [`Document::fields`].
    pub field: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CharProps {
    pub bold: bool,
    pub italic: bool,
    /// Any underline style (`\ul`, `\uld`, `\ulw` and the rest).
    pub underline: bool,
    /// `\strike` or `\striked`.
    pub strike: bool,
    pub vertical: VerticalPosition,
    /// `\scaps`.
    pub small_caps: bool,
    /// `\caps`: shown in capitals whatever the stored letters are.
    pub all_caps: bool,
    /// `\v`: hidden text. Kept, flagged; whether to show it is the consumer's decision.
    pub hidden: bool,
    /// `\fN`, `None` for the default font.
    pub font: Option<i32>,
    /// `\fsN`, in half-points.
    pub font_size: Option<u16>,
    /// `\cfN`: an index into [`Document::colors`], `None` for automatic.
    pub color: Option<u16>,
    /// `\highlightN`, `\cbN` or `\chcbpatN`: an index into [`Document::colors`]. See
    /// [`Document::background_shows`].
    pub background: Option<u16>,
    /// `\rtlch`.
    pub right_to_left: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalPosition {
    #[default]
    Baseline,
    Superscript,
    Subscript,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub kind: NoteKind,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteKind {
    Footnote,
    /// `\footnote\ftnalt`.
    Endnote,
}

/// A comment, RTF's `\annotation`, with the author parts that precede it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Annotation {
    /// `\*\atnid`: the author's initials.
    pub initials: Option<String>,
    /// `\*\atnauthor`.
    pub author: Option<String>,
    /// `\*\atndate`, as written: the specification does not say how to read it.
    pub date: Option<String>,
    /// `\*\atntime`.
    pub time: Option<RtfTime>,
    /// `\*\atnref`: the tag of the `\atrfstart`/`\atrfend` pair around the text it covers.
    pub reference: Option<String>,
    /// `\*\atnparent`: the comment this one answers.
    pub parent: Option<String>,
    pub blocks: Vec<Block>,
}

/// A date and time written as `\yrN\moN\dyN\hrN\minN`, in the writer's local time.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RtfTime {
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
    pub hour: Option<i32>,
    pub minute: Option<i32>,
}

/// A field: its instruction and whether a result was written for it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Field {
    /// `\fldinst`'s text, decoded, every group of it joined.
    pub instruction: String,
    /// Whether the field had an `\fldrslt`. Its text is in the paragraphs, as runs carrying
    /// this field's index.
    pub has_result: bool,
}

impl Field {
    /// The instruction read as a hyperlink or as another field type.
    pub fn kind(&self) -> FieldKind {
        FieldKind::parse(&self.instruction)
    }
}

/// What a field instruction asks for, as far as this reader interprets it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldKind {
    /// `HYPERLINK`. The target is the instruction's argument as written, whatever its
    /// scheme: telling a web address from a link into the writer's own application is the
    /// consumer's business.
    Hyperlink {
        target: Option<String>,
        /// The `\l` switch: a place inside the target, or in this document when there is
        /// no target.
        fragment: Option<String>,
    },
    /// Any other field (`PAGE`, `DATE`, `SYMBOL`...): its name, upper-cased.
    Other { name: String },
}

impl FieldKind {
    /// Read a field instruction.
    pub fn parse(instruction: &str) -> FieldKind {
        let words = instruction_words(instruction);
        let name = words
            .first()
            .map(|w| w.to_ascii_uppercase())
            .unwrap_or_default();
        if name != "HYPERLINK" {
            return FieldKind::Other { name };
        }
        let mut target = None;
        let mut fragment = None;
        let mut iter = words.iter().skip(1);
        while let Some(word) = iter.next() {
            match word.as_str() {
                // Switches that take an argument.
                "\\l" => fragment = iter.next().cloned(),
                "\\m" | "\\n" | "\\h" => {}
                "\\o" | "\\t" => {
                    iter.next();
                }
                w if w.starts_with('\\') => {}
                w if target.is_none() => target = Some(w.to_string()),
                _ => {}
            }
        }
        FieldKind::Hyperlink { target, fragment }
    }
}

/// A field instruction split into words, a double-quoted stretch being one word without its
/// quotes.
fn instruction_words(instruction: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut chars = instruction.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        let mut word = String::new();
        if c == '"' {
            chars.next();
            for c in chars.by_ref() {
                if c == '"' {
                    break;
                }
                word.push(c);
            }
        } else {
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() || c == '"' {
                    break;
                }
                word.push(c);
                chars.next();
            }
        }
        words.push(word);
    }
    words
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Picture {
    pub format: PictureFormat,
    pub origin: PictureOrigin,
    /// The name macOS writers store with a picture (`\*\nisusfilename`), the file name of an
    /// embedded PDF (`\*\pdffilename`), or the attachment's file name for
    /// [`PictureFormat::NextAttachment`].
    pub name: Option<String>,
    /// `\picwN`: pixels for a bitmap.
    pub width: Option<i32>,
    /// `\pichN`.
    pub height: Option<i32>,
    /// `\picwgoalN`: the width it is shown at, in twips.
    pub goal_width: Option<i32>,
    /// `\pichgoalN`.
    pub goal_height: Option<i32>,
    /// `\picscalexN`, in percent.
    pub scale_x: Option<i32>,
    /// `\picscaleyN`.
    pub scale_y: Option<i32>,
    /// The picture's bytes, decoded from hexadecimal or `\bin`. Empty for an
    /// [`Self::oversized`] picture.
    pub data: Vec<u8>,
    /// Its data passed the reader's limit and was dropped: the picture keeps its place and
    /// what the file says about it, and has no bytes. Counted as
    /// [`crate::Diagnostic::PictureTooLarge`].
    pub oversized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PictureFormat {
    Png,
    Jpeg,
    /// `\emfblip`.
    Emf,
    /// `\wmetafileN`.
    Wmf,
    /// `\macpict`.
    MacPict,
    /// `\dibitmapN`.
    Dib,
    /// `\wbitmapN`.
    Bitmap,
    /// `\pmmetafileN`.
    Os2Metafile,
    /// Apple's `\NeXTGraphic`: a file beside the RTF in its `.rtfd` package, named by
    /// [`Picture::name`]. The RTF holds no bytes for it.
    NextAttachment,
    /// A PDF document, written whole into the text (`\*\scrivenerpdf`).
    Pdf,
    /// No format word.
    #[default]
    Unknown,
}

/// Where in the file a picture was written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PictureOrigin {
    /// `{\*\shppict{\pict ...}}`, the form macOS, Scrivener and Word write.
    ShapePicture,
    /// A bare `{\pict ...}`.
    #[default]
    Bare,
    /// A drawing shape's picture property (`\shp` with `pib`).
    Shape,
    /// The picture an embedded object displays (`\object` then `\result`).
    ObjectResult,
    /// `\NeXTGraphic`.
    NextAttachment,
    /// `{\*\scrivenerpdf ...}`: a PDF written into the text, as Scrivener 2 for macOS does,
    /// named by `\*\pdffilename`.
    EmbeddedPdf,
}

/// Counters the reading keeps, for callers and for the tests that bound its work.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Stats {
    /// Tokens read. Never more than the input's length in bytes: every token consumes at
    /// least one byte.
    pub tokens: usize,
    /// The deepest group nesting met.
    pub max_depth: usize,
    /// The model's size as the reading accounted it, in bytes: text, structure and names,
    /// not picture data. Never more than [`crate::Limits::max_document_bytes`].
    pub document_bytes: usize,
    /// Picture data kept, in bytes.
    pub picture_bytes: usize,
}

/// Every paragraph of `blocks` in reading order, the paragraphs of a table cell by cell, row
/// by row, a nested table's inside its cell.
///
/// Walks with a stack of its own, so a deeply nested table costs memory, not call depth.
pub fn paragraphs(blocks: &[Block]) -> Paragraphs<'_> {
    Paragraphs {
        stack: vec![blocks.iter()],
    }
}

/// The iterator [`paragraphs`] returns.
pub struct Paragraphs<'a> {
    stack: Vec<std::slice::Iter<'a, Block>>,
}

impl<'a> Iterator for Paragraphs<'a> {
    type Item = &'a Paragraph;

    fn next(&mut self) -> Option<&'a Paragraph> {
        loop {
            let top = self.stack.last_mut()?;
            match top.next() {
                None => {
                    self.stack.pop();
                }
                Some(Block::Paragraph(paragraph)) => return Some(paragraph),
                Some(Block::Table(table)) => {
                    // Pushed in reverse so the first cell's blocks come out first.
                    for cell in table.rows.iter().flat_map(|r| r.cells.iter()).rev() {
                        self.stack.push(cell.blocks.iter());
                    }
                }
            }
        }
    }
}
