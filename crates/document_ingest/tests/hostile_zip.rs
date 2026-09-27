// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A `.docx` or `.odt` built to unpack far larger than it is is refused unread,
//! through the review's own diagnostic, in bounded time and memory.
//!
//! Two shapes per format, the two an untrusted zip can carry:
//!
//! * a **compressible member** whose header honestly states a size past what a single
//!   document may hold — a decompression bomb;
//! * a **zip64 over-declared member** holding a few real bytes but whose header claims
//!   an enormous size, the shape that aborts a reader which reserves the declared size
//!   before reading (`docx-rs`).
//!
//! Both are refused over the central directory, before a byte is inflated, so the test
//! is instant and never allocates the claimed size. Before the guard, each aborted the
//! process — `docx-rs` on the reservation, the `.odt` scanner on reading a real bomb to
//! the end; see the SubagentHandback report for the `ulimit -v` proofs.

#![cfg(all(feature = "docx", feature = "odt"))]

use std::path::Path;

use document_ingest::ImportDiagnostic;
use document_ingest::scanner::ScannerRegistry;
use skrib_format::zip_guard::fixtures::{compressible_zip, over_declared_zip};

/// The one member the scanner reads must exceed a single document's ceiling. A member
/// of 96 MiB of zeros is past the guard's 64 MiB ratio floor for one member, so it is
/// refused whatever its compressed size.
const BOMB_MIB: u64 = 96;

/// A declared size no 64-bit-free zip can express, so it forces a zip64 field.
const OVER_DECLARED: u64 = 16 << 30;

fn scanned(name: &str, bytes: &[u8]) -> Vec<ImportDiagnostic> {
    let registry = ScannerRegistry::with_builtin_scanners();
    registry.scan_bytes(Path::new(name), bytes).diagnostics
}

fn refused_as_too_large(diagnostics: &[ImportDiagnostic]) -> bool {
    diagnostics
        .iter()
        .any(|d| matches!(d, ImportDiagnostic::ArchiveTooLarge { .. }))
}

#[test]
fn a_docx_decompression_bomb_is_refused() {
    let bytes = compressible_zip("word/document.xml", BOMB_MIB << 20);
    let diagnostics = scanned("bomb.docx", &bytes);
    assert!(
        refused_as_too_large(&diagnostics),
        "a bomb .docx must be refused: {diagnostics:?}"
    );
}

#[test]
fn a_docx_zip64_over_declared_member_is_refused() {
    let bytes = over_declared_zip("word/document.xml", OVER_DECLARED);
    let diagnostics = scanned("liar.docx", &bytes);
    assert!(
        refused_as_too_large(&diagnostics),
        "an over-declared .docx member must be refused: {diagnostics:?}"
    );
}

#[test]
fn an_odt_decompression_bomb_is_refused() {
    let bytes = compressible_zip("content.xml", BOMB_MIB << 20);
    let diagnostics = scanned("bomb.odt", &bytes);
    assert!(
        refused_as_too_large(&diagnostics),
        "a bomb .odt must be refused: {diagnostics:?}"
    );
}

#[test]
fn an_odt_zip64_over_declared_member_is_refused() {
    let bytes = over_declared_zip("content.xml", OVER_DECLARED);
    let diagnostics = scanned("liar.odt", &bytes);
    assert!(
        refused_as_too_large(&diagnostics),
        "an over-declared .odt member must be refused: {diagnostics:?}"
    );
}

/// An honest, ordinary `.odt` still imports: the guard refuses only what is built to
/// exhaust memory, never a real document.
#[test]
fn an_honest_odt_still_imports() {
    // A minimal but real ODT: a content part with one paragraph.
    let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<office:document-content
 xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
 xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0">
 <office:body><office:text>
  <text:h text:outline-level="1">A Chapter</text:h>
  <text:p>The ferry was late.</text:p>
 </office:text></office:body>
</office:document-content>"#;
    let mut buffer = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(&mut buffer);
    let options = zip::write::SimpleFileOptions::default();
    use std::io::Write;
    writer.start_file("content.xml", options).unwrap();
    writer.write_all(content.as_bytes()).unwrap();
    writer.finish().unwrap();
    let bytes = buffer.into_inner();

    let diagnostics = scanned("honest.odt", &bytes);
    assert!(
        !refused_as_too_large(&diagnostics),
        "an honest .odt must not be refused: {diagnostics:?}"
    );
}
