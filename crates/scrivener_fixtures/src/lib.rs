// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Synthetic Scrivener projects, for the Scrivener importer's tests.
//!
//! Real Scrivener projects can never enter this repository, except the project
//! owner's own ground-truth projects (UQ-48: made for this purpose, in their own
//! words, released under the repository licence, in
//! `crates/scrivener_import/tests/fixtures/ground-truth-mac-3.5.2/`): most carry no
//! licence, and Literature & Latte's templates, tutorials and manual are theirs.
//! So every other fixture is built here, at test time, from a [`Spec`]: a project of any
//! generation (Scrivener 1 for Mac, formats 1.0, 1.5 and 2.0, an iOS project never
//! opened on a desktop), for any platform, written byte for byte in the layout
//! that generation writes, into any container (a folder, a zip, a
//! `.scrivtemplate`), with the traps real projects hold laid on top ([`traps`],
//! [`hostile`]).
//!
//! The text is invented English ([`text`]), owned by the project. Nothing is
//! copied from a real project.
//!
//! # Fenced by what real projects look like
//!
//! A generator that invents a format detail makes a reader pass tests for a format
//! nobody writes, which is how several published Scrivener readers ended up wrong.
//! So every element and attribute a builder writes into a manifest, every sidecar
//! element, and the shape of every file path it creates are checked against a
//! census of real projects (`observed_shape.json`, element and attribute names and
//! path shapes only, no content) by this crate's `tests/shape.rs`. A shape real
//! projects are not known to hold must be listed there as unverified, with the
//! question it waits on.

pub mod files;
pub mod hostile;
pub mod project;
pub mod rtf;
mod sha1;
pub mod text;
pub mod traps;

pub use files::{Files, TemplateEncoding, template};
pub use project::{
    Author, Backup, Bookmark, CollectionSpec, Compile, Format, Generation, Item, Kind, Platform,
    Properties, Replacement, Spec, Targets, TypedField,
};

/// A 1×1 PNG.
pub const TINY_PNG: &[u8] = &[
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
    0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0xF8, 0xFF, 0xFF, 0x3F,
    0x00, 0x05, 0xFE, 0x02, 0xFE, 0x0D, 0xEF, 0x46, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E,
    0x44, 0xAE, 0x42, 0x60, 0x82,
];

/// The smallest thing a PDF reader opens: an empty one-page document.
pub const TINY_PDF: &[u8] = b"%PDF-1.4\n1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj\n2 0 obj<</Type/Pages/Kids[3 0 R]/Count 1>>endobj\n3 0 obj<</Type/Page/Parent 2 0 R/MediaBox[0 0 72 72]>>endobj\ntrailer<</Root 1 0 R>>\n%%EOF\n";

/// `text` escaped for XML content and attribute values.
pub fn xml_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}
