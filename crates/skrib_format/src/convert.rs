// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Content conversion for the legacy upgrade chain, backed by `text-document`.
//!
//! The C++ upgrader used `MarkdownTextDocument`, a Qt `QTextDocument` subclass
//! whose "Skribisto Markdown" dialect handled only bold/italic/underline/strike
//! plus bullet lists — and whose escaping and line-wrapping paths were dead code
//! (string literals mistaken for regexes). We replace it with `text-document`'s
//! HTML/Djot engine, so migrated content lands as proper Djot instead of the
//! lossy legacy format.
//!
//! Legacy content format by DB version: Qt Markdown (≤1.6), Qt HTML (1.7–1.9),
//! Skribisto Markdown (≥2.0). The version steps convert it as the C++ did, only
//! through `text-document` rather than Qt — except the final hop, which targets
//! Djot (this crate's actual prose format) instead of reproducing that dialect.

use anyhow::Result;
use text_document::TextDocument;

/// Convert Qt rich-text HTML to Djot. Blank input → empty string.
pub fn html_to_djot(html: &str) -> Result<String> {
    Ok(html_to_djot_and_text(html)?.0)
}

/// HTML → (Djot, plain text), from **one** parse.
///
/// The document importer needs both: the Djot is what gets stored, and the plain
/// text is the coordinate space an imported comment's quote is captured in. Asking
/// for them separately would parse the same HTML twice and — worse — leave open the
/// possibility of the two answers coming from different parses of different input.
pub fn html_to_djot_and_text(html: &str) -> Result<(String, String)> {
    if html.trim().is_empty() {
        return Ok((String::new(), String::new()));
    }
    // Qt rich text puts CSS in a `<head><style>`; text-document's HTML parser
    // emits the contents of unknown elements as text, so strip non-content
    // blocks first to keep the stylesheet out of the converted text.
    let cleaned = strip_block(&strip_block(html, "style"), "script");
    let doc = TextDocument::new();
    doc.set_html(&cleaned)?.wait()?;
    Ok((doc.to_djot()?, doc.to_plain_text()?))
}

/// Remove every `<tag …>…</tag>` block (case-insensitive). Byte offsets line up
/// because `to_ascii_lowercase` is length-preserving and tag delimiters are ASCII.
fn strip_block(html: &str, tag: &str) -> String {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let lower = html.to_ascii_lowercase();
    let mut out = String::with_capacity(html.len());
    let mut pos = 0;
    while let Some(rel) = lower[pos..].find(&open) {
        let start = pos + rel;
        out.push_str(&html[pos..start]);
        match lower[start..].find(&close) {
            Some(rel_end) => pos = start + rel_end + close.len(),
            None => {
                pos = html.len(); // unterminated — drop the remainder
                break;
            }
        }
    }
    out.push_str(&html[pos..]);
    out
}

/// Convert Markdown to HTML. Blank input → empty string. Used by the 1.6→1.7
/// step (which historically turned the then-Markdown content into HTML).
pub fn markdown_to_html(markdown: &str) -> Result<String> {
    if markdown.trim().is_empty() {
        return Ok(String::new());
    }
    let doc = TextDocument::new();
    doc.set_markdown(markdown)?.wait()?;
    Ok(doc.to_html()?)
}

/// Convert Markdown to Djot — the format `Content.data` actually holds. Blank
/// input → empty string.
///
/// The conversion goes through `text-document`'s own document model rather than
/// rewriting the source text, which is what makes it safe: CommonMark and Djot
/// **swap their emphasis delimiters** (`*x*` is emphasis in Markdown and *strong*
/// in Djot), so copying the bytes across would silently bold every italicised
/// word in an imported manuscript — the single most common construct in fiction.
/// Parsing to a model that records "this run is italic" and re-emitting sidesteps
/// the whole class. Measured at roughly 250 µs for a 1,500-word scene.
///
/// **Two things the caller must handle before calling.** `text-document`'s
/// Markdown reader has no arm for `Event::Rule`, so a thematic break (`***`,
/// `---`, `* * *`) is *silently dropped* — a scene-break marker must be
/// recognised and re-emitted in its escaped Djot form by the caller, never handed
/// through as source. And YAML front matter is not recognised either: a leading
/// `---\ntitle: …\n---` parses as a setext heading and corrupts into
/// `## title: …`, so it must be stripped first.
pub fn markdown_to_djot(markdown: &str) -> Result<String> {
    Ok(markdown_to_djot_and_text(markdown)?.0)
}

/// Markdown → (Djot, plain text), from **one** parse. The Markdown twin of
/// [`html_to_djot_and_text`], and for the same reason.
pub fn markdown_to_djot_and_text(markdown: &str) -> Result<(String, String)> {
    if markdown.trim().is_empty() {
        return Ok((String::new(), String::new()));
    }
    let doc = TextDocument::new();
    doc.set_markdown(markdown)?.wait()?;
    Ok((doc.to_djot()?, doc.to_plain_text()?))
}

/// The plain text of a Djot string, plus every block's start offset.
///
/// The two coordinates `skribisto_model::comment_anchor` speaks — document-absolute
/// **character** offsets, and the block index a paragraph comment falls back to.
///
/// Exists so that a caller holding Djot it is about to store as `Content.data` can
/// ask what the editor will see, using the editor's own parse rather than a
/// reconstruction of it. The document importer anchors imported comments through
/// this: a `.docx` comment's quote is captured against the *source* document's
/// prose, and has to be proved against the *converted* prose before anything is
/// created, because a quote that fails to match is a comment silently attached to
/// the wrong sentence.
///
/// `to_plain_text` and `blocks().position()` are the same pair
/// `comments::binding::snapshot` reads off a live editor document, so an anchor
/// verified here resolves identically the first time the row is opened.
pub fn djot_plain_text(djot: &str) -> Result<(String, Vec<usize>)> {
    if djot.trim().is_empty() {
        return Ok((String::new(), Vec::new()));
    }
    let doc = TextDocument::new();
    doc.set_djot(djot)?.wait()?;
    let text = doc.to_plain_text()?;
    let starts = doc.blocks().into_iter().map(|b| b.position()).collect();
    Ok((text, starts))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The contract the importer's comment anchoring is built on: blocks are joined
    /// by exactly **one** character in the plain text, and every block's reported
    /// start is that plain text's own character index.
    #[test]
    fn plain_text_joins_blocks_with_one_character_and_reports_where_each_starts() {
        let djot = "First paragraph.\n\nSecond *emphasised* paragraph.\n\nThird.";
        let (text, starts) = djot_plain_text(djot).expect("convert");

        assert_eq!(
            text, "First paragraph.\nSecond emphasised paragraph.\nThird.",
            "markup is not part of the space an anchor is measured in"
        );
        assert_eq!(starts.len(), 3);
        assert_eq!(starts[0], 0);
        for (i, start) in starts.iter().enumerate() {
            let chars: Vec<char> = text.chars().collect();
            let head: String = chars[*start..(*start + 5).min(chars.len())]
                .iter()
                .collect();
            assert!(
                ["First", "Secon", "Third"].contains(&head.as_str()),
                "block {i} starts mid-word at {start}: {head:?}"
            );
        }
    }

    /// The escaped marker `plan.rs` splices into a row is a paragraph like any
    /// other, and reads back as the glyph a writer sees — so a comment that sits
    /// after one is not knocked out of alignment by it.
    #[test]
    fn an_escaped_scene_break_reads_back_as_its_glyph() {
        let (text, starts) = djot_plain_text("Before.\n\n\\* \\* \\*\n\nAfter.").expect("convert");
        assert_eq!(text, "Before.\n* * *\nAfter.");
        assert_eq!(starts.len(), 3);
    }

    #[test]
    fn empty_djot_has_no_text_and_no_blocks() {
        assert_eq!(
            djot_plain_text("   ").expect("convert"),
            (String::new(), Vec::new())
        );
    }

    /// One parse, two answers — and they must be answers about the same document.
    #[test]
    fn the_djot_and_the_plain_text_of_one_conversion_agree() {
        let (djot, text) =
            html_to_djot_and_text("<p>A <strong>bold</strong> word.</p><p>Second one.</p>")
                .expect("convert");
        assert_eq!(djot, "A *bold* word.\n\nSecond one.");
        assert_eq!(text, "A bold word.\nSecond one.");
        assert_eq!(djot_plain_text(&djot).expect("convert").0, text);
    }

    #[test]
    fn markdown_emphasis_survives_into_both_answers() {
        let (djot, text) = markdown_to_djot_and_text("He was *utterly* lost.").expect("convert");
        assert_eq!(
            djot, "He was _utterly_ lost.",
            "Djot italics, not Markdown's"
        );
        assert_eq!(text, "He was utterly lost.");
    }
}
