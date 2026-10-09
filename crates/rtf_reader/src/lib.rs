// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Panic hygiene: this crate is at zero `unwrap()`/`expect()`/`panic!` outside tests, so the
// lint is switched on here to keep it that way. See the note in the workspace `Cargo.toml`
// for why this is per crate and not workspace-wide.
#![warn(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! A reader for RTF, the Rich Text Format: bytes in, a small document model out.
//!
//! [`parse`] turns one RTF stream into a [`Document`]: paragraphs of styled runs, tables,
//! fields, footnotes, comments and pictures, plus the tables they refer to (fonts, colours,
//! styles, lists) and a count of everything the reading met and did not keep as written
//! ([`Diagnostics`]). It knows RTF and nothing else: no type from the rest of Skribisto, and
//! no convention an application layers on top of RTF.
//!
//! ## Provenance
//!
//! Written for Skribisto from the *Rich Text Format (RTF) Specification, version 1.9.1*
//! (Microsoft, 2008), Apple's published notes on its RTF extensions, and the RTF files that
//! real writing applications produce, observed directly: Scrivener's macOS and Windows
//! writers, Word, RichEdit and TextEdit. No other RTF reader's code was used. LibreOffice
//! served only as a black box to compare plain-text output against, never as a source.
//!
//! ## What it handles
//!
//! * Text in any code page the file declares (`\ansicpgN`, a font's `\fcharsetN` or
//!   `\cpgN`, `\mac`, `\pc`, `\pca`), double-byte pages included, `\'hh` escapes buffered and
//!   decoded together; `\uN` with the `\ucN` fallback count scoped by group, negative and
//!   surrogate forms joined. See the `codepage` notes in [`Font::code_page`].
//! * Destinations: `{\*...}` groups this reader does not know are skipped as the
//!   specification asks, and counted by name; the ones it does know are read (fonts, colours,
//!   the style sheet, `\info`, lists, fields, pictures in `\*\shppict`, `\*\ud` in `\upr`).
//! * Paragraph formatting (alignment, indents, spacing, direction, outline level, style,
//!   list membership, page break before) and character formatting (bold, italic, underline,
//!   strike-through, super- and subscript on either axis, small and all capitals, hidden,
//!   font, size, colour, background, direction).
//! * Tables, nested ones included, rebuilt from `\intbl`, `\itapN`, `\cell`, `\row` and their
//!   nested forms; line, page and column breaks; tabs.
//! * Fields, kept as fields: the instruction as written and the result as runs that name
//!   the field. A `HYPERLINK`'s target is read ([`FieldKind`]) but never judged by its
//!   scheme.
//! * Footnotes and endnotes (`\footnote`, `\ftnalt`) and comments (`\annotation`, with
//!   `\atnid`, `\atnauthor` and the `\atrfstart`/`\atrfend` range), each as a story of its
//!   own cited from the text.
//! * Pictures: the format, the sizes and the bytes, from hexadecimal or `\bin`, whether
//!   written in `\*\shppict`, bare, in a drawing shape or as an object's result; Apple's
//!   `\NeXTGraphic` attachments by name; a PDF written whole into the text
//!   (`\*\scrivenerpdf`), as a picture in that format.
//!
//! ## Limits
//!
//! The reader never recurses on the input's nesting and never passes its [`Limits`]: group
//! depth, the model's size, one `\bin`, one picture. A file nesting too deep or growing too
//! large is refused with a [`Refusal`] naming the limit. A picture past its limit is dropped
//! instead ([`Picture::oversized`], counted as [`Diagnostic::PictureTooLarge`]) and the text
//! around it read as usual: a large photograph is no reason to lose a chapter. Every token
//! consumes at least one byte, so the work done is bounded by the input's length
//! ([`Stats::tokens`]).
//!
//! ## For a layer above this reader
//!
//! An application that writes its own conventions into RTF as *text* (Scrivener writes
//! inline footnotes as `{\Scrv_fn=...\end_Scrv_fn}` and style markers as `<$Scr_Ps::0>`, all
//! with escaped braces and backslashes) reads them from this model, not from the bytes:
//!
//! * The text arrives decoded, escapes resolved: `\{`, `\}` and `\\` are the characters
//!   `{`, `}` and `\` in [`Run::text`]. Nothing in the text is interpreted.
//! * Each [`Run`] names the RTF group it was written in ([`Run::group`]) and carries that
//!   group's formatting. Text split across groups stays split, so a marker whose halves sit
//!   in different groups (the common case on Windows) is found by joining the runs, and each
//!   half's formatting and group can still be told apart.
//! * Fields stay fields: an [`Inline::FieldStart`] and [`Inline::FieldEnd`] around the
//!   result, and [`Run::field`] on every run of it, with the instruction on the [`Field`].
//!   Links into an application (`scrivcmt://`, `scrivlnk://`) are ordinary hyperlinks here.
//! * Paragraph ends are kept apart from line breaks ([`Inline::Break`]), table cells from
//!   paragraphs ([`ParagraphEnd::Cell`] and the table structure), and a character such as
//!   U+2028 or U+FFFC stays a character: deciding what it means is the layer's job.
//!
//! [`paragraphs`] walks a story's paragraphs in reading order without recursion; the model's
//! fields are public, so such a layer can rewrite it (remove marker text, add a note) before
//! handing it on.

mod codepage;
mod diagnostics;
mod lexer;
mod limits;
mod model;
mod reader;
mod story;

pub use diagnostics::{Diagnostic, Diagnostics};
pub use limits::{Limits, Refusal};
pub use model::{
    Alignment, Annotation, Block, BreakKind, Cell, CharProps, Charset, Direction, Document, Field,
    FieldKind, Font, FontFamily, Header, Info, Inline, LegacyNumbering, List, ListLevel,
    ListOverride, ListRef, Note, NoteKind, Paragraph, ParagraphEnd, ParagraphProps, Paragraphs,
    Picture, PictureFormat, PictureOrigin, Rgb, Row, RtfTime, Run, Stats, Style, StyleKind, Table,
    VerticalPosition, paragraphs,
};
pub use story::MAX_TABLE_NESTING;

/// How raw 8-bit text in the input is to be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RawText {
    /// In the code page in force, as RTF says: the normal case for a file read as bytes.
    #[default]
    CodePage,
    /// As UTF-8: for RTF that arrived inside another document (an XML sidecar) and was
    /// decoded with it, whose characters are Unicode already. `\'hh` escapes are still read
    /// in their code page.
    Utf8,
}

/// How to read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Options {
    pub limits: Limits,
    pub raw_text: RawText,
}

/// Read one RTF stream with the default [`Limits`].
pub fn parse(input: &[u8]) -> Result<Document, Refusal> {
    parse_with(input, &Options::default())
}

/// Read RTF held in a string, its raw characters taken as the Unicode they already are.
pub fn parse_str(input: &str) -> Result<Document, Refusal> {
    parse_with(
        input.as_bytes(),
        &Options {
            raw_text: RawText::Utf8,
            ..Options::default()
        },
    )
}

/// Read one RTF stream.
pub fn parse_with(input: &[u8], options: &Options) -> Result<Document, Refusal> {
    let start = rtf_start(input).ok_or(Refusal::NotRtf)?;
    reader::Reader::new(input, start, options).run()
}

/// Where the RTF document starts in `input`: at `{\rtf`, after an optional UTF-8
/// byte-order mark and white space. `None` when it does not start there.
///
/// A caller holding RTF behind a prefix of its own finds `{\rtf` itself and hands over the
/// rest: this reader does not guess where a document begins.
pub fn rtf_start(input: &[u8]) -> Option<usize> {
    let after_bom = if input.starts_with(&[0xEF, 0xBB, 0xBF]) {
        3
    } else {
        0
    };
    let rest = input.get(after_bom..)?;
    let blank = rest.iter().take_while(|b| b.is_ascii_whitespace()).count();
    let start = after_bom + blank;
    input.get(start..)?.starts_with(br"{\rtf").then_some(start)
}
