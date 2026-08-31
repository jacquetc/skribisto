// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Turning a text file's bytes into a `String`, without ever refusing.
//!
//! Shared by the Markdown and plain-text scanners; a future `.docx`/`.odt`
//! scanner will not use it, since those are zip containers whose text comes out
//! of XML already decoded.
//!
//! The policy is deliberately narrow: honour a byte-order mark, otherwise assume
//! UTF-8 and decode lossily, and *say so* when characters were replaced. There is
//! no statistical charset guessing, because its failure mode is silent
//! corruption — prose that looks fine and is subtly wrong is worse than prose
//! with a visible replacement character and a warning attached. And nothing here
//! can panic: a surveyed importer hard-crashes on one stray byte at position
//! 488905 of somebody's novel, which is the outcome this exists to make
//! impossible.

use encoding_rs::Encoding;

use crate::diagnostics::ImportDiagnostic;

/// A decoded file plus whatever the decode is owed an explanation for.
pub struct DecodedText {
    pub text: String,
    pub diagnostics: Vec<ImportDiagnostic>,
}

/// Decode `bytes` to text, reporting rather than failing.
///
/// Line endings are normalised to `\n` as well: a stray `\r` surviving into a
/// Djot line would be invisible in the editor and visible in the export.
pub fn decode(bytes: &[u8], path: &str) -> DecodedText {
    let mut diagnostics = Vec::new();

    let (cow, encoding, had_errors) = match Encoding::for_bom(bytes) {
        Some((encoding, bom_len)) => {
            if encoding != encoding_rs::UTF_8 {
                diagnostics.push(ImportDiagnostic::DecodedFromBom {
                    path: path.to_string(),
                    encoding: encoding.name(),
                });
            }
            let (cow, had_errors) = encoding.decode_without_bom_handling(&bytes[bom_len..]);
            (cow, encoding, had_errors)
        }
        None => {
            let (cow, had_errors) = encoding_rs::UTF_8.decode_without_bom_handling(bytes);
            (cow, encoding_rs::UTF_8, had_errors)
        }
    };
    let _ = encoding;

    let text = normalise_newlines(&cow);

    if had_errors {
        // U+FFFD is what `encoding_rs` substitutes; counting them tells the
        // writer how much of the file is suspect rather than just that it is.
        let replacements = text.chars().filter(|c| *c == '\u{FFFD}').count();
        diagnostics.push(ImportDiagnostic::LossyDecode {
            path: path.to_string(),
            replacements,
        });
    }

    DecodedText { text, diagnostics }
}

/// CRLF and lone CR both become LF. Borrow-free only when there is work to do.
fn normalise_newlines(s: &str) -> String {
    if !s.contains('\r') {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\r' {
            if chars.peek() == Some(&'\n') {
                chars.next();
            }
            out.push('\n');
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_utf8_decodes_untouched_and_says_nothing() {
        let d = decode("Café — naïve.".as_bytes(), "a.md");
        assert_eq!(d.text, "Café — naïve.");
        assert!(d.diagnostics.is_empty());
    }

    #[test]
    fn a_utf8_bom_is_consumed_not_imported_as_a_character() {
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice(b"Chapter One");
        let d = decode(&bytes, "a.md");
        assert_eq!(d.text, "Chapter One");
        assert!(
            d.diagnostics.is_empty(),
            "a UTF-8 BOM is not worth a warning"
        );
    }

    #[test]
    fn a_utf16_bom_is_honoured_and_reported() {
        let mut bytes = vec![0xFF, 0xFE];
        for unit in "Hi".encode_utf16() {
            bytes.extend_from_slice(&unit.to_le_bytes());
        }
        let d = decode(&bytes, "a.md");
        assert_eq!(d.text, "Hi");
        assert!(matches!(
            d.diagnostics.as_slice(),
            [ImportDiagnostic::DecodedFromBom { .. }]
        ));
    }

    /// The surveyed crash case: one byte that is not valid UTF-8, no BOM to
    /// explain it. It must not panic and must not abort the batch.
    #[test]
    fn an_undeclared_non_utf8_byte_is_lossy_and_counted_never_fatal() {
        let bytes = b"He paused\x81then nothing.";
        let d = decode(bytes, "a.md");
        assert!(d.text.contains('\u{FFFD}'));
        match d.diagnostics.as_slice() {
            [ImportDiagnostic::LossyDecode { replacements, .. }] => assert_eq!(*replacements, 1),
            other => panic!("expected one LossyDecode, got {other:?}"),
        }
    }

    #[test]
    fn crlf_and_lone_cr_both_become_lf() {
        assert_eq!(decode(b"a\r\nb\rc", "a.md").text, "a\nb\nc");
    }
}
