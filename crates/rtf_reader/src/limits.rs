// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! How much a reading may take, and the refusal that names the limit a file went past.
//!
//! A crafted file must never cost more than these: a hundred thousand nested groups would
//! overflow the stack of a recursive reader (this one keeps its own), a paragraph mark every
//! five bytes amplifies a few megabytes into hundreds of megabytes of model, and a `\binN`
//! can promise gigabytes. Each limit is far above what a real manuscript reaches.
//!
//! A file past the nesting or the model's size is refused as a whole, with a reason, rather
//! than read in part. A picture past its limit is not: its bytes are dropped and the picture
//! flagged ([`crate::Picture::oversized`]) and counted
//! ([`crate::Diagnostic::PictureTooLarge`]), and the text around it is read as usual. One
//! photograph a writer pasted into a chapter is no reason to lose the chapter's words.

use std::fmt;

/// The limits a reading holds to. [`Limits::default`] is the one to use; a caller lowers a
/// limit only to test it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// How deeply groups may nest. Real files stay under ten (Scrivener's reach five); a
    /// word processor's nested tables and fields add a few more.
    pub max_depth: usize,
    /// How large the model may grow, as the reading accounts it: text, structure and names,
    /// each list at the room it allocates, not picture data (see
    /// [`crate::Stats::document_bytes`]). The heap the model holds stays within twice what is
    /// accounted and a few kilobytes, whatever the file's shape. A 300,000-word novel in one
    /// file accounts for about ten megabytes.
    pub max_document_bytes: usize,
    /// The longest `\binN` whose data is kept. A longer one is skipped, and the picture it
    /// belongs to dropped (see the module documentation).
    pub max_binary_bytes: usize,
    /// The largest picture whose data is kept, in decoded bytes. A larger one is dropped.
    pub max_picture_bytes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            max_depth: 1_024,
            max_document_bytes: 128 << 20,
            max_binary_bytes: 64 << 20,
            max_picture_bytes: 64 << 20,
        }
    }
}

/// Why a file was not read.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Refusal {
    /// The bytes do not open with `{\rtf`, after an optional byte-order mark and white space.
    NotRtf,
    /// Groups nest deeper than [`Limits::max_depth`].
    NestedTooDeep { limit: usize },
    /// The model would pass [`Limits::max_document_bytes`].
    DocumentTooLarge { limit: usize },
}

impl Refusal {
    /// The limit the file went past, when it went past one.
    pub fn limit(&self) -> Option<usize> {
        match self {
            Refusal::NotRtf => None,
            Refusal::NestedTooDeep { limit } | Refusal::DocumentTooLarge { limit } => Some(*limit),
        }
    }
}

/// An untranslated rendering, for logs and tests. A user interface names a refusal from its
/// variant, in its own language.
impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Refusal::NotRtf => write!(f, "not an RTF document"),
            Refusal::NestedTooDeep { limit } => {
                write!(f, "groups nest deeper than {limit} levels")
            }
            Refusal::DocumentTooLarge { limit } => {
                write!(f, "the document would take more than {limit} bytes")
            }
        }
    }
}

impl std::error::Error for Refusal {}
