//! Content conversion for the legacy upgrade chain, backed by `text-document`.
//!
//! The C++ upgrader used `MarkdownTextDocument`, a Qt `QTextDocument` subclass
//! whose "Skribisto Markdown" dialect handled only bold/italic/underline/strike
//! plus bullet lists — and whose escaping and line-wrapping paths were dead code
//! (string literals mistaken for regexes). We replace it with `text-document`'s
//! HTML/Markdown engine, so migrated content lands as proper GFM Markdown
//! (headings, lists, links, code, tables, …) instead of the lossy legacy format.
//!
//! Legacy content format by DB version: Qt Markdown (≤1.6), Qt HTML (1.7–1.9),
//! Skribisto Markdown (≥2.0). The version steps convert it as the C++ did, only
//! through `text-document` rather than Qt.

use anyhow::Result;
use text_document::TextDocument;

/// Convert Qt rich-text HTML to GFM Markdown. Blank input → empty string.
pub fn html_to_djot(html: &str) -> Result<String> {
    if html.trim().is_empty() {
        return Ok(String::new());
    }
    // Qt rich text puts CSS in a `<head><style>`; text-document's HTML parser
    // emits the contents of unknown elements as text, so strip non-content
    // blocks first to keep the stylesheet out of the Markdown.
    let cleaned = strip_block(&strip_block(html, "style"), "script");
    let doc = TextDocument::new();
    doc.set_html(&cleaned)?.wait()?;
    Ok(doc.to_djot()?)
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
