// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One module per input format — the swappable half of the pipeline.
//!
//! Each implements `SourceScanner` and nothing else depends on which one ran.
//!
//! Each is behind its own cargo feature, which is what this module asked for when
//! `.docx` and `.odt` were still hypothetical: they want heavier dependencies (a
//! zip reader, an XML parser, an OOXML reader), and a consumer that only imports
//! Markdown should not build them. `ScannerRegistry::with_builtin_scanners`
//! registers whatever is compiled in, and every list the UI shows is derived from
//! the registry — so turning a format off removes it from the file filter and the
//! drop zone's accept list too, rather than offering a format that then reports
//! itself unsupported.
//!
//! [`rich`] is the shared half of the two container formats. It is *not* behind a
//! feature of its own: it belongs to whichever of `odt` and `docx` is enabled, and
//! to neither when both are off.

/// What a single imported document's zip container may hold, before and while it
/// inflates (see [`skrib_format::zip_guard`]).
///
/// Generous rather than tight, because refusing a real manuscript is a worse
/// failure than a slow one, but far below what exhausts memory: a book with tracked
/// changes, comments and its whole revision history in one `document.xml` lands in
/// the low tens of megabytes, and its embedded images push the total up but no one
/// part past the member ceiling. A `.docx` or `.odt` past any of these is one built
/// to be, not one anyone wrote.
#[cfg(any(feature = "odt", feature = "docx"))]
pub(crate) const DOCUMENT_ZIP_LIMITS: skrib_format::zip_guard::ZipLimits =
    skrib_format::zip_guard::ZipLimits {
        // A document with hundreds of images and their relationship parts, an order
        // of magnitude of headroom over any real one.
        max_entries: 50_000,
        // One part — a `document.xml` or `content.xml`, or one embedded image.
        max_member_bytes: 512 << 20,
        // The whole document, images included.
        max_total_bytes: 2 << 30,
        // A document's parts are whatever its author put in it.
        max_ratio: skrib_format::zip_guard::MAX_RATIO,
    };

/// The most list levels imported prose keeps. A list item nested deeper arrives at the
/// deepest of them, and is reported ([`crate::ImportDiagnostic::ListNestingFlattened`]).
///
/// Word's numbering has nine levels (`w:ilvl` 0 to 8) and LibreOffice's ten, so sixteen
/// keeps every level either application can write, with six to spare for a producer that
/// writes more. It is also far inside what stored prose may nest: the deepest item is
/// written thirty columns in, which `skrib_format::djot_depth` counts, with its marker, as
/// thirty-one of its [`skrib_format::MAX_DJOT_DEPTH`] levels, leaving room for any
/// blockquote or footnote around it. A list written deeper than that ceiling is not merely
/// unusual: the next load refuses the whole project over it, and from `text-document`
/// 1.12.3 the parser reads such a line as literal text rather than as a list.
///
/// Here rather than in `rich` because Markdown keeps the same levels, and `rich` is built
/// only for the two container formats.
pub const MAX_LIST_LEVELS: usize = 16;

#[cfg(feature = "docx")]
pub mod docx;
#[cfg(feature = "markdown")]
pub mod markdown;
#[cfg(feature = "odt")]
pub mod odt;
#[cfg(feature = "plain")]
pub mod plain;
#[cfg(any(feature = "odt", feature = "docx"))]
pub mod rich;
