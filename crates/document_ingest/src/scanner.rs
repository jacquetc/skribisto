// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The seam every format plugs into.
//!
//! One trait, taking bytes and returning a [`SourceDocument`]. Bytes rather than
//! text on purpose: `.docx` and `.odt` are zip containers, so "decode these bytes
//! to a string" is a Markdown-and-plain-text concern, not a universal one.
//!
//! Adding a format is: implement [`SourceScanner`], register it, add the
//! extension to the file-picker filter, and add its diagnostics' strings to both
//! locales. If it ever costs more than that, the seam has drifted and belongs
//! back here rather than in the new scanner.

use std::path::Path;

use anyhow::Result;

use crate::block::SourceDocument;
use crate::diagnostics::ImportDiagnostic;

/// Turns one file's bytes into a [`SourceDocument`].
///
/// Implementations report rather than refuse: a construct that cannot be carried
/// over becomes an [`ImportDiagnostic`] on the returned document, and `Err` is
/// reserved for "this produced nothing at all".
pub trait SourceScanner {
    /// Lower-case extensions, without the dot, that this scanner claims.
    fn extensions(&self) -> &[&str];

    /// A stable identifier for the format, for diagnostics and telemetry.
    fn format_name(&self) -> &'static str;

    fn scan(&self, bytes: &[u8], display_name: &str, origin: &str) -> Result<SourceDocument>;
}

/// Extension → scanner. Built once per import and consulted per file.
#[derive(Default)]
pub struct ScannerRegistry {
    scanners: Vec<Box<dyn SourceScanner>>,
}

impl ScannerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Every scanner this build carries — one line per format, each behind its own
    /// cargo feature. Adding a format really is one more line here; the rest of the
    /// pipeline does not change.
    pub fn with_builtin_scanners() -> Self {
        let mut registry = Self::new();
        #[cfg(feature = "markdown")]
        {
            registry.register(Box::new(crate::sources::markdown::MarkdownScanner));
            registry.register(Box::new(crate::sources::plain::PlainTextScanner));
        }
        #[cfg(feature = "odt")]
        registry.register(Box::new(crate::sources::odt::OdtScanner));
        #[cfg(feature = "docx")]
        registry.register(Box::new(crate::sources::docx::DocxScanner));
        registry
    }

    pub fn register(&mut self, scanner: Box<dyn SourceScanner>) {
        self.scanners.push(scanner);
    }

    /// Every extension any registered scanner claims — what the file picker and
    /// the drop zone should accept, so the two can never disagree with what the
    /// importer can actually read.
    pub fn accepted_extensions(&self) -> Vec<&str> {
        let mut all: Vec<&str> = self
            .scanners
            .iter()
            .flat_map(|s| s.extensions().iter().copied())
            .collect();
        all.sort_unstable();
        all.dedup();
        all
    }

    pub fn scanner_for_extension(&self, extension: &str) -> Option<&dyn SourceScanner> {
        let wanted = extension.to_ascii_lowercase();
        self.scanners
            .iter()
            .find(|s| s.extensions().contains(&wanted.as_str()))
            .map(|s| s.as_ref())
    }

    /// Scan one file's bytes, choosing the scanner from `path`'s extension.
    ///
    /// An unclaimed extension is a document carrying one `Error` diagnostic
    /// rather than an `Err`, so a folder holding a stray `.pdf` still imports its
    /// sixty `.md` files and says plainly what it skipped.
    pub fn scan_bytes(&self, path: &Path, bytes: &[u8]) -> SourceDocument {
        let origin = path.to_string_lossy().to_string();
        let display_name = display_name_for(path);
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        // The one place a document's bytes are in hand, so the one place its
        // digest can be taken without reading the file a second time — by which
        // point it may have been edited, moved or deleted. Stamped on **every**
        // return below, including the two failure paths: "this is the file that
        // could not be read" is exactly as much provenance as "this is the file
        // these words came from", and a scanner is free to build its document
        // however it likes without having to remember.
        let source_file_digest = blake3::hash(bytes).to_hex().to_string();

        let Some(scanner) = self.scanner_for_extension(&extension) else {
            let mut doc = SourceDocument::new(display_name, &origin);
            doc.source_file_digest = source_file_digest;
            doc.diagnostics.push(ImportDiagnostic::UnsupportedFormat {
                path: origin,
                extension,
            });
            return doc;
        };

        let mut doc = match scanner.scan(bytes, &display_name, &origin) {
            Ok(doc) => doc,
            Err(err) => {
                let mut doc = SourceDocument::new(display_name, &origin);
                doc.diagnostics.push(ImportDiagnostic::FileUnreadable {
                    path: origin,
                    reason: err.to_string(),
                });
                doc
            }
        };
        doc.source_file_digest = source_file_digest;
        doc
    }
}

/// A file stem cleaned up enough to show a writer, and to fall back on as a
/// title.
///
/// Strips a leading ordinal-and-separator run (`01_`, `1 - `, `003.`), which is
/// how a folder of scenes is almost always ordered on disk — the number is the
/// ordering, not part of the name, and carrying it into a binder title would put
/// a second, wrong numbering next to Skribisto's own. A stem that is *only*
/// digits keeps them, since stripping would leave nothing.
pub fn display_name_for(path: &Path) -> String {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .trim();

    let without_ordinal = strip_leading_ordinal(stem);
    let cleaned = if without_ordinal.is_empty() {
        stem
    } else {
        without_ordinal
    };
    cleaned
        .replace(['_', '-'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// `"01_the-storm"` → `"the-storm"`; `"12"` → `""` (caller keeps the original).
fn strip_leading_ordinal(stem: &str) -> &str {
    let rest = stem.trim_start_matches(|c: char| c.is_ascii_digit());
    if rest.len() == stem.len() {
        return stem; // no leading digits at all
    }
    let rest = rest.trim_start_matches([' ', '-', '_', '.', ')']);
    rest.trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_numeric_prefix_is_ordering_not_a_title() {
        assert_eq!(display_name_for(Path::new("01_the-storm.md")), "the storm");
        assert_eq!(display_name_for(Path::new("1 - The Storm.md")), "The Storm");
        assert_eq!(display_name_for(Path::new("003.the-storm.md")), "the storm");
    }

    #[test]
    fn a_name_that_is_only_a_number_keeps_it() {
        assert_eq!(display_name_for(Path::new("12.md")), "12");
    }

    #[test]
    fn a_name_starting_with_a_word_is_untouched_but_tidied() {
        assert_eq!(display_name_for(Path::new("chapter_one.md")), "chapter one");
        assert_eq!(display_name_for(Path::new("Chapter 10.md")), "Chapter 10");
    }

    #[test]
    fn the_registry_offers_every_extension_it_can_read() {
        let registry = ScannerRegistry::with_builtin_scanners();
        let exts = registry.accepted_extensions();
        for expected in ["markdown", "md", "txt"] {
            assert!(exts.contains(&expected), "missing '{expected}' in {exts:?}");
        }
    }

    #[test]
    fn an_unclaimed_extension_is_reported_not_fatal() {
        let registry = ScannerRegistry::with_builtin_scanners();
        let doc = registry.scan_bytes(Path::new("cover.pdf"), b"%PDF-1.7");
        assert!(doc.blocks.is_empty());
        assert!(matches!(
            doc.diagnostics.as_slice(),
            [ImportDiagnostic::UnsupportedFormat { .. }]
        ));
    }
}
