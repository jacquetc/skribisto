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
use text_document::{ReplaceFormatPolicy, ReplaceOptions, ReplaceRange, TextDocument};

/// The attribute an HTML producer marks a footnote reference with, re-exported
/// from `text-document` so the importers can reach it.
///
/// They build HTML and hand it to [`html_to_djot`] rather than depending on
/// `text-document` themselves, so the one contract that HTML has to honour would
/// otherwise be un-nameable from where it is written. A hard-coded copy of the
/// string in each scanner is exactly the kind of pair that drifts silently: the
/// producer would emit an attribute nothing reads, and the footnote would vanish
/// with no error.
pub use text_document::HTML_FOOTNOTE_ATTR;

/// Prose converted from another markup language, as Djot a bundle can hold.
///
/// What [`html_to_djot_and_text`] and [`markdown_to_djot_and_text`] return: the Djot to
/// store, its plain text, and whether the markup had to be given up to store it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConvertedDjot {
    /// The Djot to store. Always within [`crate::MAX_DJOT_DEPTH`], so a load accepts it.
    pub djot: String,
    /// Its plain text, from the same parse as `djot`.
    pub text: String,
    /// Whether the markup nested past [`crate::MAX_DJOT_DEPTH`], so the words were stored
    /// as plain text, one paragraph per line, rather than as prose the next load of the
    /// project would refuse. A caller that can tell the writer should.
    pub flattened: bool,
}

/// `djot`, converted from markup whose plain text is `text`, as prose a load accepts.
///
/// A converter writes whatever the markup nests, and markup can nest deeper than a bundle
/// may: two hundred quoted levels, or a list indented four hundred columns. Stored as
/// written, such prose is refused by the next load of the project
/// ([`crate::djot_depth`]), which then cannot be opened at all. So when the Djot would be
/// refused, the words are kept instead, as plain text, which is always accepted.
///
/// The Djot refused is never parsed on the way: its plain text comes from the parse that
/// wrote it, and the plain Djot written in its place is what is read back.
fn within_depth(djot: String, text: String) -> Result<ConvertedDjot> {
    if crate::djot_depth::check(&djot).is_ok() {
        return Ok(ConvertedDjot {
            djot,
            text,
            flattened: false,
        });
    }
    words_alone(&text)
}

/// `text` stored as its words alone: plain text, one paragraph per line, which a load
/// always accepts, and reported as such.
fn words_alone(text: &str) -> Result<ConvertedDjot> {
    let djot = plain_text_to_djot_verbatim(text);
    let (text, _) = djot_plain_text(&djot)?;
    Ok(ConvertedDjot {
        djot,
        text,
        flattened: true,
    })
}

/// Run a conversion on the parser stack ([`crate::xml_depth::on_parser_stack`]).
///
/// `text-document` reads on a thread of its own, but writes the Djot, HTML and plain text
/// it is asked for on the calling thread, recursing once per level of what it read. From
/// a long operation's 2 MiB thread in a debug build, its Djot writer overflows at 477
/// nested blockquotes and its HTML writer before 300. What a converter hands it is held
/// to a ceiling first ([`crate::markup_depth`]), and this is the headroom above it,
/// whatever thread the conversion was called from.
fn on_parser_stack<T: Send>(work: impl FnOnce() -> Result<T> + Send) -> Result<T> {
    match crate::xml_depth::on_parser_stack(work) {
        Ok(converted) => converted,
        Err(crate::xml_depth::XmlError::NoParserThread(e)) => Err(anyhow::anyhow!(
            "could not start a thread to convert the text on: {e}"
        )),
        Err(other) => Err(anyhow::Error::new(other)),
    }
}

/// Convert Qt rich-text HTML to Djot. Blank input → empty string.
///
/// Markup nested past what a bundle may hold arrives as plain text, as
/// [`html_to_djot_and_text`] describes.
pub fn html_to_djot(html: &str) -> Result<String> {
    Ok(html_to_djot_and_text(html)?.djot)
}

/// HTML → Djot and plain text, from **one** parse.
///
/// Asking for the two separately would parse the same HTML twice, and leave open the
/// possibility of the two answers coming from different parses of different input.
///
/// The Word and OpenDocument importers no longer come through here: HTML's rules
/// collapse a double space and have no reading for a centred paragraph, so they write
/// Djot directly and prove it with [`read_djot`]. This stays for HTML that really is
/// HTML: legacy content and the Plume and Manuskript bodies stored as it.
///
/// HTML nesting past what a bundle can hold is stored as its words alone, and says so
/// ([`ConvertedDjot::flattened`]). Past [`crate::markup_depth::MAX_HTML_DEPTH`] elements
/// the HTML is not handed to the parser at all, since the parser silently drops text
/// nested much deeper: its words are read out of the markup first, a paragraph for each
/// block, and only those are parsed.
pub fn html_to_djot_and_text(html: &str) -> Result<ConvertedDjot> {
    if html.trim().is_empty() {
        return Ok(ConvertedDjot::default());
    }
    let cleaned = without_style_and_script(html);
    on_parser_stack(|| {
        let doc = TextDocument::new();
        if crate::markup_depth::check_clean_html(&cleaned).is_err() {
            doc.set_html(&crate::markup_depth::html_without_nesting(&cleaned))?
                .wait()?;
            return words_alone(&doc.to_plain_text()?);
        }
        doc.set_html(&cleaned)?.wait()?;
        within_depth(doc.to_djot()?, doc.to_plain_text()?)
    })
}

/// `html` without its `<style>` and `<script>` blocks, as every reader of HTML here
/// takes it.
///
/// Qt rich text puts CSS in a `<head><style>`; text-document's HTML parser emits the
/// contents of unknown elements as text, so non-content blocks are stripped first to
/// keep the stylesheet out of the converted text.
pub(crate) fn without_style_and_script(html: &str) -> String {
    strip_block(&strip_block(html, "style"), "script")
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
///
/// Markdown nested past [`crate::markup_depth::MAX_MARKDOWN_DEPTH`] is not handed to
/// the parser: its words are, a paragraph for each line, and arrive as HTML paragraphs
/// of plain text, as [`markdown_to_djot_and_text`] stores them.
pub fn markdown_to_html(markdown: &str) -> Result<String> {
    if markdown.trim().is_empty() {
        return Ok(String::new());
    }
    on_parser_stack(|| {
        let doc = TextDocument::new();
        if crate::markup_depth::check_markdown(markdown).is_err() {
            doc.set_markdown(&crate::markup_depth::markdown_without_nesting(markdown))?
                .wait()?;
            return Ok(plain_text_to_html(&doc.to_plain_text()?));
        }
        doc.set_markdown(markdown)?.wait()?;
        Ok(doc.to_html()?)
    })
}

/// Plain text as HTML paragraphs, one per non-blank line, with the characters HTML
/// would read as markup escaped.
fn plain_text_to_html(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + text.len() / 4);
    for line in plain_text_lines(text) {
        out.push_str("<p>");
        for c in line.chars() {
            match c {
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                c => out.push(c),
            }
        }
        out.push_str("</p>");
    }
    out
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
///
/// Markdown nested past what a bundle may hold arrives as plain text, as
/// [`markdown_to_djot_and_text`] describes.
pub fn markdown_to_djot(markdown: &str) -> Result<String> {
    Ok(markdown_to_djot_and_text(markdown)?.djot)
}

/// Markdown → Djot and plain text, from **one** parse. The Markdown twin of
/// [`html_to_djot_and_text`], and for the same reason.
///
/// Markdown nesting past what a bundle can hold is stored as its words alone, and says
/// so ([`ConvertedDjot::flattened`]). Past [`crate::markup_depth::MAX_MARKDOWN_DEPTH`]
/// the Markdown is not handed to the parser at all: its container markers are taken off
/// every line first, which leaves the same words with nothing to nest, and only those
/// are parsed.
pub fn markdown_to_djot_and_text(markdown: &str) -> Result<ConvertedDjot> {
    Ok(convert_markdown(markdown, None)?.0)
}

/// [`markdown_to_djot_and_text`], with every list item nested deeper than `levels`
/// written at the deepest of them instead, beside the deepest items kept; and how many
/// items were.
///
/// An item keeps its words, its marker and its formatting; only its depth changes. It is
/// what the Word and OpenDocument importers do with a list nested past the levels
/// imported prose keeps, so the same list arrives the same way whichever of the three
/// formats holds it. Markdown nested past [`crate::markup_depth::MAX_MARKDOWN_DEPTH`] is
/// stored as its words alone, as [`markdown_to_djot_and_text`] describes, and no item is
/// counted then: nothing of its lists is left to move.
pub fn markdown_to_djot_within_list_levels(
    markdown: &str,
    levels: usize,
) -> Result<(ConvertedDjot, usize)> {
    convert_markdown(markdown, Some(levels))
}

/// The two conversions above: `levels`, when given, is the depth lists are held to.
fn convert_markdown(markdown: &str, levels: Option<usize>) -> Result<(ConvertedDjot, usize)> {
    if markdown.trim().is_empty() {
        return Ok((ConvertedDjot::default(), 0));
    }
    on_parser_stack(|| {
        let doc = TextDocument::new();
        if crate::markup_depth::check_markdown(markdown).is_err() {
            doc.set_markdown(&crate::markup_depth::markdown_without_nesting(markdown))?
                .wait()?;
            return Ok((words_alone(&doc.to_plain_text()?)?, 0));
        }
        doc.set_markdown(markdown)?.wait()?;
        let moved = match levels {
            Some(levels) => hold_list_levels(&doc, levels)?,
            None => 0,
        };
        let converted = within_depth(doc.to_djot()?, doc.to_plain_text()?)?;
        let moved = if converted.flattened { 0 } else { moved };
        Ok((converted, moved))
    })
}

/// Put every list of `doc` nested deeper than `levels` at the deepest of them, and say
/// how many list items that moved.
///
/// A list's depth is a property of the list, not of its items (`TextList::indent`, from
/// 0 at the top), and the Djot writer writes an item two columns deeper for each level,
/// so setting it is all it takes: the items of a list moved up are written beside those
/// of the deepest list kept.
fn hold_list_levels(doc: &TextDocument, levels: usize) -> Result<usize> {
    let deepest = u8::try_from(levels.saturating_sub(1)).unwrap_or(u8::MAX);
    let mut lists = std::collections::BTreeSet::new();
    let mut items = 0usize;
    for block in doc.blocks() {
        if let Some(list) = block.list()
            && list.indent() > deepest
        {
            lists.insert(list.id());
            items += 1;
        }
    }
    if lists.is_empty() {
        return Ok(0);
    }
    let cursor = doc.cursor();
    let format = text_document::ListFormat {
        indent: Some(deepest),
        ..Default::default()
    };
    for list in lists {
        cursor.set_list_format(list, &format)?;
    }
    Ok(items)
}

/// The **addressable** text of a Djot string, plus every block's start offset.
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
/// `to_addressable_text` and `blocks().position()` are the same pair
/// `comments::binding::snapshot` reads off a live editor document, so an anchor
/// verified here resolves identically the first time the row is opened.
///
/// It must be `to_addressable_text()` and NOT `to_plain_text()`: block positions,
/// selections and search offsets all live in the document's own char space, where an
/// embedded table occupies its `U+FFFC` anchor plus a `\n` separator. `to_plain_text()`
/// is the human-readable export and omits the anchor, so pairing it with these starts
/// skewed every offset after a table by two characters — a comment made on
/// `"salt-bleached"` in a row with a table stored its quote as `"lt-bleached d"`.
/// `spike_block_starts.rs` / `spike_table_capture.rs` pin the repaired pairing.
///
/// Only [Djot whitespace](is_djot_whitespace) makes the input blank. A paragraph holding
/// nothing but a no-break space is a paragraph to the parser and to the editor, so it is
/// one here too.
pub fn djot_plain_text(djot: &str) -> Result<(String, Vec<usize>)> {
    if trim_djot_whitespace(djot).is_empty() {
        return Ok((String::new(), Vec::new()));
    }
    let doc = TextDocument::new();
    doc.set_djot(djot)?.wait()?;
    let text = doc.to_addressable_text()?;
    let starts = doc.blocks().into_iter().map(|b| b.position()).collect();
    Ok((text, starts))
}

/// One replacement for [`rewrite_djot_text`]: a character range of the document's
/// addressable text, and the plain text to put there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextEdit {
    /// The first character replaced, as an offset into the addressable text.
    pub start: usize,
    /// How many characters are replaced.
    pub len: usize,
    /// Plain text, never Djot: it is escaped on the way out.
    pub replacement: String,
}

/// Replace ranges of a Djot string's **text**, keep its formatting, and return the new
/// Djot.
///
/// `edits` is handed the addressable text (the editor's own view of the prose, as
/// [`djot_plain_text`] returns it) and names the ranges to replace. The replacement is
/// made on the document and written back by `text-document`'s exporter, which escapes the
/// new text against the text around it.
///
/// Splicing plain text into the Djot string instead is not safe. A marker a writer typed
/// as `{C:0:Peter}` is stored as `\{C\:0\:Peter\}`, so a splice that finds the `{` keeps
/// the backslash in front of it, and `\Peter` reads back with a literal backslash. And a
/// name written in raw can form Djot syntax with its neighbours that neither held alone: a
/// colon on each side makes it a symbol, which the parser drops.
///
/// Returns `djot` byte for byte when there is nothing to replace. A replacement keeps the
/// formatting of the text it replaces when one run covered all of it. A range that crosses
/// a paragraph boundary, or overlaps another, is skipped, as
/// [`TextDocument::replace_ranges`] documents.
pub fn rewrite_djot_text(djot: &str, edits: impl FnOnce(&str) -> Vec<TextEdit>) -> Result<String> {
    if djot.trim().is_empty() {
        return Ok(djot.to_string());
    }
    let doc = TextDocument::new();
    doc.set_djot(djot)?.wait()?;
    let edits = edits(&doc.to_addressable_text()?);
    if edits.is_empty() {
        return Ok(djot.to_string());
    }
    let ranges: Vec<ReplaceRange> = edits
        .into_iter()
        .map(|e| ReplaceRange {
            position: e.start,
            length: e.len,
            replacement: e.replacement,
        })
        .collect();
    let options =
        ReplaceOptions::default().with_format_policy(ReplaceFormatPolicy::PreserveIfFullyCovered);
    doc.replace_ranges(&ranges, &options)?;
    Ok(doc.to_djot()?)
}

// ── Text into Djot that reads back verbatim ─────────────────────────────────────
//
// text-document 1.12.2's own escaper (`escape_djot_inline`, `guard_djot_block_start`,
// `plain_text_to_djot`) leaves several ordinary strings for the parser to rewrite on the
// first load: `10:30:45` loses `:30:` as a symbol, a paragraph opening `I. ` becomes a
// list item and loses its numeral, straight quotes are curled, `--` and `...` become a
// dash and an ellipsis. The functions below close those gaps on top of the upstream
// ones, so an importer never stores text that the editor would change on opening it.
// Every extra rule first checks whether the upstream function already escaped the
// character, which makes it a no-op once text-document escapes it itself.

/// Whether the pinned Djot parser strips `c` from the edges of a paragraph.
///
/// These are the ASCII whitespace characters: space, tab, line feed, form feed and
/// carriage return. A no-break space is not one of them and survives at an edge. A
/// paragraph can only be read back without these at its two ends, so text is trimmed
/// with [`trim_djot_whitespace`] before it is written or compared.
pub fn is_djot_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

/// `s` without the leading and trailing characters a Djot paragraph cannot keep.
pub fn trim_djot_whitespace(s: &str) -> &str {
    s.trim_matches(is_djot_whitespace)
}

/// What surrounds a piece of inline text, for the escaping rules that depend on it.
///
/// Two of Djot's inline constructs are made of characters that are harmless one at a
/// time: `--` and `...` become typographic dashes and an ellipsis, and `:word:` becomes a
/// symbol that the editor drops. A paragraph imported from a styled document arrives as
/// several runs, and one of those sequences can straddle two of them (`x-` in one run and
/// `-y` in the next, split at a comment boundary). Escaping each run on its own would
/// miss it, so the escaper is told what the reader sees on either side.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EscapeContext<'a> {
    /// The paragraph's text before this piece, as plain text rather than Djot. Only its
    /// last word and the character before that word are read.
    pub before: &'a str,
    /// The paragraph's text after this piece, as plain text. Only its first word and the
    /// character after that word are read.
    pub after: &'a str,
    /// Whether the piece is written between braced delimiters such as `{_…_}`.
    ///
    /// There an unescaped `=` can turn a paragraph that is one styled run into a block
    /// attribute line: `{_E=mc2_}` parses as the attribute `_E="mc2_"` and the paragraph
    /// disappears. Outside braces `=` means nothing and is left alone.
    pub braced: bool,
}

/// Inline plain text as Djot that the pinned parser reads back as exactly that text.
///
/// Built on `text_document::escape_djot_inline`, the escaper the editor's own Djot
/// export uses, and extended with the characters that version leaves open:
///
/// * `'` and `"`, which the parser curls into typographic quotes;
/// * a `:` that opens or closes a symbol (`:30:` in `10:30:45`) or sits against another
///   colon (`std::vector`), since a symbol is removed from the text;
/// * every `-` and `.` of a run of two or more, counting the neighbours in `context`,
///   which would become an en dash, an em dash or an ellipsis;
/// * `=` when `context.braced` says the text sits between braced delimiters.
///
/// A character the upstream escaper already put behind a backslash is left as it is.
///
/// Line starts are not handled here: a leading `#`, `-` or `A.` is a block marker only at
/// the start of a line, which is [`guard_djot_line_start`]'s job. `text` holds no line
/// break: a newline inside a Djot paragraph reads back as a space.
pub fn escape_djot_text(text: &str, context: EscapeContext<'_>) -> String {
    extend_inline_escapes(&text_document::escape_djot_inline(text), text, context)
}

/// Add the escapes `text` still needs to `escaped`, the output of an inline escaper run
/// over the same `text`.
///
/// When `escaped` is not `text` with some ASCII punctuation marks put behind a backslash
/// (an escaper whose output this does not recognise), every ASCII punctuation mark is
/// escaped instead. Djot reads a backslash before any of them as that mark, so the result
/// still reads back exactly.
fn extend_inline_escapes(escaped: &str, text: &str, context: EscapeContext<'_>) -> String {
    let chars: Vec<char> = text.chars().collect();
    let already = escaped_flags(escaped, &chars)
        .unwrap_or_else(|| chars.iter().map(char::is_ascii_punctuation).collect());
    let neighbours = Neighbours::new(&chars, context);
    let mut out = String::with_capacity(escaped.len() + 8);
    for ((i, &c), &escaped) in chars.iter().enumerate().zip(&already) {
        if escaped || neighbours.needs_escape(i, c, context.braced) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// Which characters of `text` the inline Djot `escaped` writes behind a backslash, or
/// `None` when `escaped` is not `text` with some punctuation marks escaped.
fn escaped_flags(escaped: &str, text: &[char]) -> Option<Vec<bool>> {
    let mut flags = Vec::with_capacity(text.len());
    let mut source = escaped.chars();
    for &expected in text {
        let (c, flag) = match source.next()? {
            '\\' => {
                // Djot reads `\` before anything but punctuation or a space as a literal
                // backslash, so an escape of anything else is not one this can keep.
                let c = source.next()?;
                if !c.is_ascii_punctuation() {
                    return None;
                }
                (c, true)
            }
            c => (c, false),
        };
        if c != expected {
            return None;
        }
        flags.push(flag);
    }
    source.next().is_none().then_some(flags)
}

/// A character Djot accepts inside a symbol's name, between its two colons.
fn is_symbol_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '-' | '+' | '_')
}

/// A piece of text with as much of its paragraph around it as the escaping rules read.
struct Neighbours {
    /// The kept end of the text before, the text itself, then the kept start of the text
    /// after.
    chars: Vec<char>,
    /// Where the text itself starts in `chars`.
    offset: usize,
}

impl Neighbours {
    fn new(text: &[char], context: EscapeContext<'_>) -> Self {
        // A symbol's name holds only symbol characters, so a scan for its other colon
        // never reads past the first character that is not one. Keeping that run plus
        // the character that stopped it is enough for every rule below.
        let mut before: Vec<char> = Vec::new();
        for c in context.before.chars().rev() {
            before.push(c);
            if !is_symbol_char(c) {
                break;
            }
        }
        before.reverse();
        let offset = before.len();
        let mut chars = before;
        chars.extend_from_slice(text);
        for c in context.after.chars() {
            chars.push(c);
            if !is_symbol_char(c) {
                break;
            }
        }
        Self { chars, offset }
    }

    /// Whether the character `c` at `i` in the text needs a backslash the upstream
    /// escaper does not give it.
    fn needs_escape(&self, i: usize, c: char, braced: bool) -> bool {
        let at = self.offset + i;
        match c {
            '\'' | '"' => true,
            '=' => braced,
            '-' | '.' => {
                let before = at.checked_sub(1).and_then(|p| self.chars.get(p));
                let after = self.chars.get(at + 1);
                before == Some(&c) || after == Some(&c)
            }
            ':' => self.opens_symbol(at) || self.closes_symbol(at),
            _ => false,
        }
    }

    /// Whether the colon at `at` starts a symbol: symbol characters, possibly none, up to
    /// the next colon.
    fn opens_symbol(&self, at: usize) -> bool {
        let rest = self.chars.get(at + 1..).unwrap_or_default();
        rest.iter().find(|c| !is_symbol_char(**c)) == Some(&':')
    }

    /// Whether the colon at `at` ends a symbol that started at an earlier colon.
    fn closes_symbol(&self, at: usize) -> bool {
        let head = self.chars.get(..at).unwrap_or_default();
        head.iter().rev().find(|c| !is_symbol_char(**c)) == Some(&':')
    }
}

/// A line of Djot whose start can never open a block.
///
/// `line` already holds inline Djot, as [`escape_djot_text`] or an emitter built on it
/// writes it. Built on `text_document::guard_djot_block_start` and extended with the
/// markers that version leaves open: a letter or roman ordered-list marker (`a.`, `B)`,
/// `iv.`, `(c)`) followed by a space, a tab or nothing; a leading `(`; a thematic break;
/// a `:::` or backtick fence; a footnote or link definition (`[^1]: …`); and any of these
/// after leading spaces or tabs, which the parser skips before it looks for a marker.
///
/// Where the line opens a block, one backslash goes before the character that makes it
/// one, so the text reads back unchanged. A line that opens none is returned as it came,
/// which is also why a line the upstream guard already neutralised is left alone.
///
/// A leading `{` is not touched. In escaped text it is always `\{`, and a styled run
/// written by [`push_djot_run`] opens with a braced delimiter that no attribute can start
/// once `=` inside it is escaped. The emitter writes a list item as its marker plus the
/// guarded content, so `- 1. x` keeps its `1.` as text.
pub fn guard_djot_line_start(line: &str) -> String {
    neutralise_block_start(&text_document::guard_djot_block_start(line))
}

/// Put one backslash where `line` would otherwise open a block.
fn neutralise_block_start(line: &str) -> String {
    let rest = line.trim_start_matches(|c: char| is_djot_whitespace(c) && c != '\n');
    let indent = line.len() - rest.len();
    // Every offset `block_marker_escape` returns sits before an ASCII character, so both
    // halves always exist.
    match block_marker_escape(rest).and_then(|at| line.split_at_checked(indent + at)) {
        Some((head, tail)) => format!("{head}\\{tail}"),
        None => line.to_string(),
    }
}

/// Where a backslash stops `rest`, a line with its indentation removed, from opening a
/// block, or `None` when it opens none.
///
/// Mirrors the block identification of jotdown 0.10, the parser behind text-document
/// 1.12.2, one arm per marker it recognises.
fn block_marker_escape(rest: &str) -> Option<usize> {
    let mut chars = rest.chars();
    let first = chars.next()?;
    let second = chars.next();
    let space_or_end = |c: Option<char>| c.is_none_or(|c| c.is_ascii_whitespace());
    match first {
        '#' => space_or_end(rest.trim_start_matches('#').chars().next()).then_some(0),
        '>' => space_or_end(second).then_some(0),
        '|' => {
            let line = rest.trim_end_matches(is_djot_whitespace);
            (line.len() >= 2 && line.ends_with('|') && !line.ends_with("\\|")).then_some(0)
        }
        '[' => {
            let close = rest.get(1..)?.find(']')? + 1;
            rest.get(close + 1..)?.starts_with(':').then_some(close + 1)
        }
        '-' | '*' if is_thematic_break(rest) => Some(0),
        '-' | '*' | '+' => second.is_none_or(|c| c == ' ').then_some(0),
        ':' => (space_or_end(second) || is_fence(rest, ':')).then_some(0),
        '`' | '~' => is_fence(rest, first).then_some(0),
        '(' => Some(0),
        _ => ordered_list_delimiter(rest),
    }
}

/// Three or more of `-` and `*`, and nothing else but spaces or tabs.
fn is_thematic_break(rest: &str) -> bool {
    let mut marks = 0;
    for c in rest.chars() {
        match c {
            '-' | '*' => marks += 1,
            c if c.is_ascii_whitespace() => {}
            _ => return false,
        }
    }
    marks >= 3
}

/// Whether `rest` opens a fenced block with `fence` (a backtick, a tilde or a colon).
fn is_fence(rest: &str, fence: char) -> bool {
    let length = rest.chars().take_while(|c| *c == fence).count();
    if length < 3 {
        return false;
    }
    let line = rest.trim_end_matches(is_djot_whitespace);
    // The fence characters are ASCII, so their count is their length in bytes.
    let spec = line
        .get(length..)
        .unwrap_or_default()
        .trim_start_matches(is_djot_whitespace);
    if fence == ':' {
        spec.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, ':' | '_' | '-'))
    } else {
        !spec.chars().any(|c| c.is_ascii_whitespace() || c == '`')
    }
}

/// Where the delimiter of an ordered-list marker opening `rest` sits, or `None`.
///
/// A marker is a number of up to 19 digits, one letter, or up to 13 roman digits all of
/// one case, then `.` or `)`, then a space, a tab or the end of the line. The roman
/// reading wins where both apply, so `c.` and `mix.` open a list just as `a.` does.
fn ordered_list_delimiter(rest: &str) -> Option<usize> {
    fn is_roman_lower(c: char) -> bool {
        matches!(c, 'i' | 'v' | 'x' | 'l' | 'c' | 'd' | 'm')
    }
    fn is_roman_upper(c: char) -> bool {
        matches!(c, 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M')
    }
    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }
    fn nothing(_: char) -> bool {
        false
    }

    let first = rest.chars().next()?;
    let (continues, max_len): (fn(char) -> bool, usize) = if first.is_ascii_digit() {
        (is_digit, 19)
    } else if is_roman_lower(first) {
        (is_roman_lower, 13)
    } else if is_roman_upper(first) {
        (is_roman_upper, 13)
    } else if first.is_ascii_alphabetic() {
        (nothing, 1)
    } else {
        return None;
    };
    // Every character counted here is ASCII, so the count is also a byte offset.
    let len = 1 + rest
        .get(1..)?
        .chars()
        .take(max_len - 1)
        .take_while(|c| continues(*c))
        .count();
    let after_number = rest.get(len..)?;
    let delimiter = after_number.chars().next()?;
    if delimiter != '.' && delimiter != ')' {
        return None;
    }
    let after = after_number.get(1..)?.chars().next();
    after.is_none_or(|c| c.is_ascii_whitespace()).then_some(len)
}

/// Plain text as Djot that reads back as the same text, one paragraph per line.
///
/// For text that was never markup: a synopsis, a plain-text note or comment body, a field
/// from another application's form, a `.txt` file. Every line goes through
/// [`escape_djot_text`] and [`guard_djot_line_start`], where
/// `text_document::plain_text_to_djot` leaves `10:30:45`, `I. Introduction`, straight
/// quotes, `--` and `...` for the first load to rewrite.
///
/// # The contract
///
/// [`djot_plain_text`] of the result is `text` with each line trimmed of
/// [Djot whitespace](is_djot_whitespace), blank lines removed, and the lines joined by
/// one `\n`. `\r\n` and a lone `\r` both end a line. That shape is the target's, not a
/// choice: the parser joins paragraphs with exactly one `\n`, strips a paragraph's edge
/// whitespace and reads a newline inside a paragraph as a space, so one paragraph per
/// line is the only shape that keeps every line, and no Djot brings back a blank line or
/// a space at the edge of one.
pub fn plain_text_to_djot_verbatim(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + text.len() / 8);
    for line in plain_text_lines(text) {
        if !out.is_empty() {
            out.push_str("\n\n");
        }
        out.push_str(&guard_djot_line_start(&escape_djot_text(
            line,
            EscapeContext::default(),
        )));
    }
    out
}

/// The lines of `text` a paragraph can hold: split at `\n`, `\r\n` or `\r`, trimmed of
/// Djot whitespace, blank ones dropped.
fn plain_text_lines(text: &str) -> impl Iterator<Item = &str> {
    text.split(['\n', '\r'])
        .map(trim_djot_whitespace)
        .filter(|line| !line.is_empty())
}

/// The character styles a run of imported text can carry into Djot.
///
/// Named after what the writer sees rather than after Djot's constructs: underline is
/// written as Djot's insert (`{+…+}`) and strikethrough as its delete (`{-…-}`), which is
/// how text-document reads them back.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DjotInlineStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub superscript: bool,
    pub subscript: bool,
}

impl DjotInlineStyle {
    /// The braced delimiters this style is written with, outermost first.
    ///
    /// A character cannot be raised and lowered at once, so superscript wins when a
    /// caller asks for both.
    fn delimiters(self) -> impl DoubleEndedIterator<Item = (&'static str, &'static str)> {
        [
            (self.bold, "{*", "*}"),
            (self.italic, "{_", "_}"),
            (self.underline, "{+", "+}"),
            (self.strikethrough, "{-", "-}"),
            (self.superscript, "{^", "^}"),
            (self.subscript && !self.superscript, "{~", "~}"),
        ]
        .into_iter()
        .filter(|(on, _, _)| *on)
        .map(|(_, open, close)| (open, close))
    }

    fn is_plain(self) -> bool {
        self.delimiters().next().is_none()
    }
}

/// How much of a run's text [`push_djot_run_with`] puts behind a backslash.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DjotEscaping {
    /// Only what the pinned parser would otherwise rewrite, as [`escape_djot_text`] decides.
    /// Ordinary prose stays readable in the stored file.
    #[default]
    Needed,
    /// Every ASCII punctuation mark. Djot reads a backslash before any of them as that mark,
    /// so this reads back exactly whatever the text holds. It is the second attempt for a
    /// paragraph whose first writing did not read back as written.
    EveryMark,
}

/// `text` with every ASCII punctuation mark behind a backslash.
fn escape_every_mark(text: &str) -> String {
    let mut out = String::with_capacity(text.len() * 2);
    for c in text.chars() {
        if c.is_ascii_punctuation() {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// Append one run of styled text to a paragraph being written as Djot.
///
/// The shape the document importer writes every run of text in:
///
/// * braced delimiters always (`{*…*}`, `{_…_}`, `{+…+}`, `{-…-}`, `{^…^}`, `{~…~}`),
///   which read the same inside a word (`half{*way*}`) and when nested, where bare
///   `*` and `_` depend on what surrounds them;
/// * the run's edge whitespace outside the delimiters, so a styled run never opens or
///   closes on a space;
/// * the text through [`escape_djot_text`], told what the reader sees on either side and
///   whether it sits between braces.
///
/// `before` and `after` are the paragraph's plain text on either side of `text`. The
/// caller trims the paragraph's own edges with [`trim_djot_whitespace`] before splitting
/// it into runs, and guards the finished line with [`guard_djot_line_start`]. A run that
/// is only whitespace is written without its style: `text-document` reads a style on
/// spaces alone, but drops it the first time the editor writes the paragraph back, so it
/// could not be kept.
pub fn push_djot_run(
    out: &mut String,
    text: &str,
    style: DjotInlineStyle,
    before: &str,
    after: &str,
) {
    push_djot_run_with(out, text, style, before, after, DjotEscaping::Needed);
}

/// [`push_djot_run`], escaping as `escaping` says.
pub fn push_djot_run_with(
    out: &mut String,
    text: &str,
    style: DjotInlineStyle,
    before: &str,
    after: &str,
    escaping: DjotEscaping,
) {
    let Some((lead, core, trail)) = split_edge_whitespace(text) else {
        out.push_str(text);
        return;
    };
    let context = EscapeContext {
        before: if lead.is_empty() { before } else { lead },
        after: if trail.is_empty() { after } else { trail },
        braced: !style.is_plain(),
    };
    out.push_str(lead);
    for (open, _) in style.delimiters() {
        out.push_str(open);
    }
    match escaping {
        DjotEscaping::Needed => out.push_str(&escape_djot_text(core, context)),
        DjotEscaping::EveryMark => out.push_str(&escape_every_mark(core)),
    }
    for (_, close) in style.delimiters().rev() {
        out.push_str(close);
    }
    out.push_str(trail);
}

/// `text` as its leading Djot whitespace, its core and its trailing Djot whitespace, or
/// `None` when it is only whitespace.
fn split_edge_whitespace(text: &str) -> Option<(&str, &str, &str)> {
    let without_lead = text.trim_start_matches(is_djot_whitespace);
    let core = without_lead.trim_end_matches(is_djot_whitespace);
    if core.is_empty() {
        return None;
    }
    let lead = text.strip_suffix(without_lead).unwrap_or_default();
    let trail = without_lead.strip_prefix(core).unwrap_or_default();
    Some((lead, core, trail))
}

/// Append one run of text written as code: a verbatim span, inside the braced delimiters
/// of `style`.
///
/// Nothing inside a verbatim span is markup, so the text is written as it is, between
/// backtick fences one longer than its longest run of backticks. When the text opens or
/// closes with a backtick, one space pads that side, which Djot removes again on reading.
/// Edge whitespace goes outside the span, as [`push_djot_run`] puts it outside delimiters.
pub fn push_djot_verbatim_run(out: &mut String, text: &str, style: DjotInlineStyle) {
    let Some((lead, core, trail)) = split_edge_whitespace(text) else {
        out.push_str(text);
        return;
    };
    let mut longest = 0;
    let mut current = 0;
    for c in core.chars() {
        if c == '`' {
            current += 1;
            longest = longest.max(current);
        } else {
            current = 0;
        }
    }
    let fence = "`".repeat(longest + 1);
    let pad_start = if core.starts_with('`') { " " } else { "" };
    let pad_end = if core.ends_with('`') { " " } else { "" };
    out.push_str(lead);
    for (open, _) in style.delimiters() {
        out.push_str(open);
    }
    out.push_str(&fence);
    out.push_str(pad_start);
    out.push_str(core);
    out.push_str(pad_end);
    out.push_str(&fence);
    for (_, close) in style.delimiters().rev() {
        out.push_str(close);
    }
    out.push_str(trail);
}

/// A link or image destination as Djot reads it back unchanged.
///
/// The pinned parser keeps a destination's characters as they are written, backslashes
/// included, so escaping a `)` with one would store the backslash in the link. A bare
/// `(`/`)` pair is taken as nesting and a lone `)` ends the destination early, angle
/// brackets are kept in the target, and a backtick opens a verbatim span that swallows the
/// rest of the paragraph. Those characters, the others a URL may not hold unencoded
/// (`"`, `\`, `^`, `{`, `|`, `}`), whitespace and control characters are therefore
/// percent-encoded (`(` becomes `%28`), which names the same resource and reads back byte
/// for byte. Everything else is written as it came.
pub fn djot_link_destination(url: &str) -> String {
    let mut out = String::with_capacity(url.len());
    for c in url.chars() {
        let encode = matches!(
            c,
            '(' | ')' | '\\' | '<' | '>' | '`' | '"' | '^' | '{' | '|' | '}'
        ) || c.is_whitespace()
            || c.is_control();
        if encode {
            let mut bytes = [0u8; 4];
            for byte in c.encode_utf8(&mut bytes).bytes() {
                out.push_str(&format!("%{byte:02X}"));
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// What the pinned parser reads a Djot string as, block by block.
///
/// The document importer proves what it writes against this before storing it: the text
/// every block reads back as, and the destination of every link, since a link whose target
/// read back differently would be a broken link the plain text cannot show.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DjotReading {
    /// The addressable text, exactly as [`djot_plain_text`] reports it.
    pub text: String,
    /// One entry per block, in order.
    pub blocks: Vec<DjotBlockReading>,
}

/// One block of a [`DjotReading`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DjotBlockReading {
    /// The block's first character in [`DjotReading::text`].
    pub start: usize,
    /// The block's own text: from `start` to the end of its line.
    pub text: String,
    /// The destination of each link in the block, in order. Adjacent text carrying the
    /// same destination is one link.
    pub links: Vec<String>,
}

/// Read `djot` with the pinned parser: its addressable text, and every block's start, text
/// and link destinations. The same parse as [`djot_plain_text`], with more of it kept.
pub fn read_djot(djot: &str) -> Result<DjotReading> {
    if trim_djot_whitespace(djot).is_empty() {
        return Ok(DjotReading::default());
    }
    let doc = TextDocument::new();
    doc.set_djot(djot)?.wait()?;
    let text = doc.to_addressable_text()?;
    let chars: Vec<char> = text.chars().collect();
    let mut blocks = Vec::new();
    for block in doc.blocks() {
        let start = block.position();
        let line: String = chars
            .get(start..)
            .unwrap_or_default()
            .iter()
            .take_while(|c| **c != '\n')
            .collect();
        let mut links: Vec<String> = Vec::new();
        let mut previous: Option<String> = None;
        for fragment in block.fragments() {
            let href = match fragment {
                text_document::FragmentContent::Text { format, .. } => format.anchor_href,
                _ => None,
            };
            if let Some(target) = &href
                && previous.as_ref() != Some(target)
            {
                links.push(target.clone());
            }
            previous = href;
        }
        blocks.push(DjotBlockReading {
            start,
            text: line,
            links,
        });
    }
    Ok(DjotReading { text, blocks })
}

/// `djot` as the editor writes it back: read with the pinned parser, then written by the
/// same library's Djot writer, which is what the editor stores once the text is edited.
///
/// Two strings with the same result show the same words with the same formatting and links,
/// whatever markup spelled them: `It *always* is.` and `It {*always*} is.` agree, while
/// `It {_always_} is.` and a link to another destination do not. Blank input gives the
/// empty string.
pub fn djot_as_the_editor_writes_it(djot: &str) -> Result<String> {
    if trim_djot_whitespace(djot).is_empty() {
        return Ok(String::new());
    }
    let doc = TextDocument::new();
    doc.set_djot(djot)?.wait()?;
    Ok(doc.to_djot()?)
}

#[cfg(test)]
mod escape_tests;

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

    /// Replace the first `needle` in `text` (a range counted in characters).
    fn edit_at(text: &str, needle: &str, replacement: &str) -> TextEdit {
        let byte = text.find(needle).expect("the needle is in the text");
        TextEdit {
            start: text[..byte].chars().count(),
            len: needle.chars().count(),
            replacement: replacement.to_string(),
        }
    }

    /// The replacement is plain text, escaped against what surrounds it: `30` between
    /// two colons would make `:30:` a Djot symbol, which the parser drops, if it were
    /// spliced into the string raw.
    #[test]
    fn rewritten_text_reads_back_exactly_as_replaced() {
        let djot = markdown_to_djot("Meet at 10:{X}:45, it's \"late\".").expect("convert");
        let out =
            rewrite_djot_text(&djot, |text| vec![edit_at(text, "{X}", "30")]).expect("rewrite");
        assert_eq!(
            djot_plain_text(&out).expect("read back").0,
            "Meet at 10:30:45, it's \"late\".",
            "rewritten to {out:?}"
        );
    }

    /// Offsets are characters of the addressable text, so an accented letter or an
    /// earlier paragraph does not shift the range.
    #[test]
    fn edits_address_characters_across_paragraphs() {
        let djot = "Première ligne, déjà.\n\nPuis {X} et _{Y}_.";
        let out = rewrite_djot_text(djot, |text| {
            vec![edit_at(text, "{X}", "Élise"), edit_at(text, "{Y}", "Zoé")]
        })
        .expect("rewrite");
        assert_eq!(out, "Première ligne, déjà.\n\nPuis Élise et _Zoé_.");
    }

    /// Nothing to replace: the Djot comes back byte for byte, not re-canonicalised.
    #[test]
    fn djot_with_nothing_to_replace_is_returned_untouched() {
        for djot in ["An *odd*   spacing kept.", "", "   "] {
            assert_eq!(
                rewrite_djot_text(djot, |_| Vec::new()).expect("rewrite"),
                djot
            );
        }
    }

    #[test]
    fn empty_djot_has_no_text_and_no_blocks() {
        assert_eq!(
            djot_plain_text("   ").expect("convert"),
            (String::new(), Vec::new())
        );
    }

    /// The parser keeps a paragraph that is only a no-break space, so the text an anchor
    /// is measured in must keep it as well.
    #[test]
    fn a_paragraph_of_only_a_no_break_space_is_still_a_paragraph() {
        assert_eq!(
            djot_plain_text("\u{a0}").expect("convert"),
            ("\u{a0}".to_string(), vec![0])
        );
    }

    /// Markup that spells the same formatting agrees; a different style or a different link
    /// destination over the same words does not.
    #[test]
    fn the_editors_writing_tells_formatting_apart_but_not_its_spelling() {
        let same = |a: &str, b: &str| {
            djot_as_the_editor_writes_it(a).expect("convert")
                == djot_as_the_editor_writes_it(b).expect("convert")
        };
        assert!(same("It *always* is.", "It {*always*} is."));
        assert!(same("At 10\\:30.", "At 10\\:30."));
        assert!(!same("It *always* is.", "It {_always_} is."));
        assert!(!same("It *always* is.", "It always is."));
        assert!(!same(
            "See [the map](https://a.example/map).",
            "See [the map](https://b.example/map)."
        ));
        assert_eq!(djot_as_the_editor_writes_it(" \n ").expect("convert"), "");
    }

    /// One parse, two answers — and they must be answers about the same document.
    #[test]
    fn the_djot_and_the_plain_text_of_one_conversion_agree() {
        let converted =
            html_to_djot_and_text("<p>A <strong>bold</strong> word.</p><p>Second one.</p>")
                .expect("convert");
        assert_eq!(converted.djot, "A *bold* word.\n\nSecond one.");
        assert_eq!(converted.text, "A bold word.\nSecond one.");
        assert_eq!(
            djot_plain_text(&converted.djot).expect("convert").0,
            converted.text
        );
        assert!(!converted.flattened);
    }

    /// A list held to its levels keeps every item, its words and its formatting, and
    /// writes the deeper ones beside the deepest kept, the next load reading them as list
    /// items at that level.
    #[test]
    fn a_markdown_list_is_held_to_the_levels_asked_for() {
        let markdown: String = (0..6)
            .map(|level| format!("{}- item *{level}*\n", "  ".repeat(level)))
            .collect();
        let (held, moved) = markdown_to_djot_within_list_levels(&markdown, 3).expect("convert");
        assert_eq!(moved, 3, "the items of the fourth, fifth and sixth levels");
        assert!(!held.flattened);
        assert_eq!(
            held.text, "item 0\nitem 1\nitem 2\nitem 3\nitem 4\nitem 5",
            "every item's words"
        );
        let doc = TextDocument::new();
        doc.set_djot(&held.djot)
            .expect("parse")
            .wait()
            .expect("parse");
        let depths: Vec<u8> = doc
            .blocks()
            .iter()
            .filter_map(|block| block.list().map(|list| list.indent()))
            .collect();
        assert_eq!(depths, vec![0, 1, 2, 2, 2, 2], "{}", held.djot);
        assert!(held.djot.contains("- item _5_"), "{}", held.djot);

        let (as_written, moved) =
            markdown_to_djot_within_list_levels(&markdown, 16).expect("convert");
        assert_eq!(moved, 0);
        assert_eq!(
            as_written,
            markdown_to_djot_and_text(&markdown).expect("convert")
        );
    }

    #[test]
    fn markdown_emphasis_survives_into_both_answers() {
        let converted = markdown_to_djot_and_text("He was *utterly* lost.").expect("convert");
        assert_eq!(
            converted.djot, "He was _utterly_ lost.",
            "Djot italics, not Markdown's"
        );
        assert_eq!(converted.text, "He was utterly lost.");
        assert!(!converted.flattened);
    }

    /// Markup nested past what a bundle may hold is kept as its words, in a form the next
    /// load accepts, and says so. Converted as written, each of these was prose the load
    /// refuses, which would have left the project unopenable.
    #[test]
    fn markup_nested_past_the_ceiling_is_stored_as_words_a_load_accepts() {
        let quoted_markdown = format!("{}Deep words.", "> ".repeat(150));
        let listed_markdown = (0..120)
            .map(|level| format!("{}- Level {level}.", "  ".repeat(level)))
            .collect::<Vec<_>>()
            .join("\n");
        let quoted_html = format!(
            "{}<p>Deep words.</p>{}",
            "<blockquote>".repeat(150),
            "</blockquote>".repeat(150)
        );
        let conversions = [
            (
                "quoted Markdown",
                markdown_to_djot_and_text(&quoted_markdown),
            ),
            (
                "a nested Markdown list",
                markdown_to_djot_and_text(&listed_markdown),
            ),
            ("quoted HTML", html_to_djot_and_text(&quoted_html)),
        ];
        for (shape, converted) in conversions {
            let converted = converted.expect("convert");
            assert!(converted.flattened, "{shape}: the markup could not be kept");
            assert!(
                crate::djot_depth::check(&converted.djot).is_ok(),
                "{shape}: a load must accept {:?}",
                converted.djot
            );
            assert_eq!(
                djot_plain_text(&converted.djot).expect("parse").0,
                converted.text,
                "{shape}: the text is what the stored Djot reads back as"
            );
        }
        let quoted = markdown_to_djot_and_text(&quoted_markdown).expect("convert");
        assert_eq!(quoted.text, "Deep words.");
        let listed = markdown_to_djot_and_text(&listed_markdown).expect("convert");
        for level in [0, 60, 119] {
            assert!(
                listed.text.contains(&format!("Level {level}.")),
                "every item's words arrive: {:?}",
                listed.text
            );
        }

        // Indentation opens no container, so a code block set four hundred columns in
        // nests nothing: it is kept as written, its indentation with it, and the load
        // accepts it (`djot_depth`'s "a line that opens nothing counts nothing").
        let indented_code = format!("```\n{}Deep words.\n```", " ".repeat(300));
        let preformatted_html = format!("<pre>{}Deep words.</pre>", " ".repeat(300));
        for (shape, converted) in [
            (
                "an indented code block",
                markdown_to_djot_and_text(&indented_code),
            ),
            (
                "preformatted HTML",
                html_to_djot_and_text(&preformatted_html),
            ),
        ] {
            let converted = converted.expect("convert");
            assert!(!converted.flattened, "{shape}: {:?}", converted.djot);
            assert!(
                crate::djot_depth::check(&converted.djot).is_ok(),
                "{shape}: a load must accept {:?}",
                converted.djot
            );
            assert!(
                converted
                    .text
                    .contains(&format!("{}Deep words.", " ".repeat(300))),
                "{shape}: the code keeps its indentation: {:?}",
                converted.text
            );
        }

        // Nesting a bundle holds is kept as it is.
        let kept =
            markdown_to_djot_and_text(&format!("{}Words.", "> ".repeat(20))).expect("convert");
        assert!(!kept.flattened);
        assert!(kept.djot.starts_with("> > "), "{:?}", kept.djot);
    }
}
