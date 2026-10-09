// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What a reading met and did not keep as written, counted by kind.
//!
//! Data, never sentences: a consumer turns a kind into words in its writer's language, or
//! into one of its own diagnostics. Counts rather than positions, because every kind here is
//! either harmless on its own (a lone brace) or already visible in the model (a hidden run
//! keeps its text, flagged), and what a consumer reports is how often.

use std::collections::BTreeMap;

/// One kind of thing a reading counts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Diagnostic {
    /// The input ended with groups still open; each was closed there.
    UnclosedGroup,
    /// Tokens after the document's own group closed, a stray `}` included, ignored.
    /// Trailing whitespace and NUL bytes (a RichEdit writer appends `\r\n\0`) are not
    /// counted.
    BytesAfterDocument,
    /// Bytes in a code page this reader has no table for, each read as U+FFFD.
    UnsupportedCodePage(u16),
    /// Bytes that are not a character in their code page, read as U+FFFD.
    UndecodableBytes,
    /// Half of a surrogate pair written without its other half, read as U+FFFD.
    LoneSurrogate,
    /// A `\uN` naming no character, read as U+FFFD.
    InvalidUnicode,
    /// `\'` not followed by two hexadecimal digits.
    BadHexEscape,
    /// A picture whose hexadecimal data had an odd number of digits; the last one was
    /// dropped.
    OddPictureHex,
    /// Characters in picture data that are not hexadecimal digits, ignored.
    PictureDataNotHex,
    /// A `\binN` promising more bytes than the input held.
    BinaryTruncated,
    /// A picture whose data passed [`crate::Limits::max_picture_bytes`], or was written with
    /// a `\binN` longer than [`crate::Limits::max_binary_bytes`]: its bytes were dropped and
    /// the picture flagged [`crate::Picture::oversized`], the text around it kept.
    PictureTooLarge,
    /// `\bin` data outside a picture, ignored.
    BinaryOutsidePicture,
    /// A control character other than a tab in the text, removed.
    ControlCharacterRemoved,
    /// An ignorable destination (`{\*\name ...}`) this reader does not read, skipped as the
    /// specification asks. The name is kept for the first few distinct ones.
    IgnorableDestinationSkipped(String),
    /// Ignorable destinations beyond the distinct names kept.
    OtherIgnorableDestinationsSkipped,
    /// A header or footer, whose text is not part of the document's flow.
    HeaderOrFooterSkipped,
    /// A table nested deeper than [`crate::MAX_TABLE_NESTING`]; its paragraphs were kept in
    /// the deepest table allowed.
    TableNestingFlattened,
    /// A drawing shape with no picture this reader can take from it.
    ShapeSkipped,
    /// A shape's text box.
    TextBoxSkipped,
    /// An embedded object that wrote no result to show in its place.
    ObjectWithoutResult,
    /// An embedded object read as the result it shows (a picture, or text).
    ObjectReadAsResult,
    /// A field nested in another field's instruction; its text joined the outer
    /// instruction.
    FieldInInstruction,
    /// A character that stands for something computed where the document is shown (the
    /// date, the time, a page or section number), which has no text of its own.
    DynamicCharacterSkipped,
}

/// How many distinct skipped destination names are kept before the rest are only counted:
/// enough for every one a real writer uses, and a bound on what a crafted file can make the
/// reader remember.
const DISTINCT_DESTINATIONS: usize = 64;

/// The counts a reading kept.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Diagnostics {
    counts: BTreeMap<Diagnostic, usize>,
    /// How many distinct names [`Diagnostic::IgnorableDestinationSkipped`] holds. Kept
    /// apart rather than counted over the map at each skipped destination: the map also
    /// holds a key per unsupported code page, up to 65,536 of them, which such a count would
    /// walk for every destination a file skips.
    distinct_destinations: usize,
}

impl Diagnostics {
    /// How many times `kind` was counted.
    pub fn count(&self, kind: &Diagnostic) -> usize {
        self.counts.get(kind).copied().unwrap_or(0)
    }

    /// Every kind counted at least once, with its count, in a stable order.
    pub fn iter(&self) -> impl Iterator<Item = (&Diagnostic, usize)> {
        self.counts.iter().map(|(kind, count)| (kind, *count))
    }

    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    pub(crate) fn add(&mut self, kind: Diagnostic, count: usize) {
        if count == 0 {
            return;
        }
        let entry = self.counts.entry(kind).or_insert(0);
        *entry = entry.saturating_add(count);
    }

    /// Count a skipped ignorable destination under its name, while the names kept are few.
    pub(crate) fn skipped_destination(&mut self, name: &[u8]) {
        let name: String = name
            .iter()
            .take(32)
            .map(|b| char::from(*b))
            .collect::<String>();
        let key = Diagnostic::IgnorableDestinationSkipped(name);
        if self.counts.contains_key(&key) {
            self.add(key, 1);
        } else if self.distinct_destinations < DISTINCT_DESTINATIONS {
            self.distinct_destinations += 1;
            self.add(key, 1);
        } else {
            self.add(Diagnostic::OtherIgnorableDestinationsSkipped, 1);
        }
    }
}
