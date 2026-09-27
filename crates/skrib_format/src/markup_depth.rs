// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A ceiling on how deeply nested the Markdown and HTML the converters read may be,
//! and what is kept of a document nested past it.
//!
//! # The failure this prevents
//!
//! [`crate::convert`] hands Markdown and HTML to `text-document`, whose readers and
//! writers recurse once per level of nesting and have no limit of their own.
//! Measured in a debug build, with 2 MiB of stack, the size a long operation's
//! thread gets:
//!
//! * the Markdown reader, which runs on a thread `text-document` starts itself,
//!   reads 3,028 nested blockquotes and aborts at 3,029;
//! * the Djot writer, which runs on the thread that asks for the Djot, writes 476
//!   and aborts at 477, and the HTML writer aborts before 300;
//! * the HTML reader stops descending at its own depth of 256, which it reaches
//!   by about 128 nested elements, and **silently drops everything deeper**: a
//!   paragraph at the bottom of 200 nested `<div>`s converts to nothing, and says
//!   nothing.
//!
//! About a kilobyte of Markdown was enough to take the app down from any of the
//! importers that convert it, and a stack overflow is not a panic: it aborts the
//! process, and every window of this single-instance app goes with it.
//!
//! # Two defences, and why both
//!
//! 1. [`check_markdown`](crate::markup_depth::check_markdown) and
//!    [`check_html`](crate::markup_depth::check_html) read the text before any
//!    parser does and refuse a document nested past its ceiling. Both scans are
//!    loops, never recursions, so neither can overflow on the input it exists to
//!    refuse. What a converter does with a refused document is its choice; the ones
//!    in [`crate::convert`] keep its words, as plain text, one paragraph per line.
//! 2. The conversions run on the parser stack
//!    ([`crate::xml_depth::on_parser_stack`]), so the writers that recurse on the
//!    calling thread have more than thirty times what a document at the ceiling
//!    needs, whatever thread the conversion was called from.
//!
//! # What is counted
//!
//! **Markdown**, line by line, as the Djot guard counts Djot ([`crate::djot_depth`],
//! which holds the scan both share): every marker that opens a container at the
//! start of a line, blockquote (`>`, with or without the space), list item and
//! footnote definition, however many share the line, and one level per column of
//! whitespace in front of the last one, since a list item continues only on lines
//! indented past its marker. A tab counts four columns, the most it can reach.
//!
//! **HTML**, element by element, as `html5ever` (behind `text-document`'s reader)
//! builds its tree: every start tag opens a level except the void elements, and an
//! end tag closes one only when it names the element the scan last opened. The
//! tree builder closes more than that (an end tag closes everything inside the
//! element it names, a `<p>` closes the paragraph open before it), so the scan
//! over-counts wherever the markup is sloppy, which is the safe direction; the
//! closings it does mirror are the ones the tree builder makes on its current
//! element whatever surrounds it. A table counts three levels, for the body and the
//! row the tree builder supplies inside it when the markup leaves them out.
//!
//! Tags are found where the standard's tokenizer finds them: comments, doctypes
//! and processing instructions are skipped as it skips them (an abrupt `<!-->`
//! included), attribute values quote to quote, so a `>` inside one ends nothing,
//! and the text of a `<title>`, `<textarea>` or the other raw text elements runs
//! to their own end tag, so a `<!--` inside one hides nothing after it. Where the
//! tokenizer's reading depends on the tree builder's state rather than on the
//! markup (inside `<svg>`, `<math>` or `<select>`, after a `<script>`), the scan
//! stops following the markup: from there on, every `<` followed by a letter opens
//! a level, wherever it sits, and nothing closes one.
//!
//! # The limits
//!
//! Markdown's is the Djot ceiling itself,
//! [`MAX_MARKDOWN_DEPTH`](crate::markup_depth::MAX_MARKDOWN_DEPTH): nothing deeper
//! could be stored anyway, and nothing at it comes near the reader's or the
//! writers' limits. HTML's, [`MAX_HTML_DEPTH`](crate::markup_depth::MAX_HTML_DEPTH),
//! is 64 elements, half the depth at which the reader starts dropping text, which
//! leaves the other half for the elements the tree builder adds on its own. The Qt
//! rich text Plume, Manuskript and older Skribisto projects store rarely nests ten.

use crate::djot_depth::{Grammar, line_start};

/// The most nested block containers Markdown may declare before it is converted.
pub const MAX_MARKDOWN_DEPTH: usize = crate::djot_depth::MAX_DEPTH;

/// The most nested elements HTML may reach before it is converted.
pub const MAX_HTML_DEPTH: usize = 64;

/// Why a Markdown or HTML document was not handed to the parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkupTooDeep {
    /// How deep the scan had counted when it stopped: always past the ceiling.
    pub depth: usize,
    /// The 1-based line it stopped on.
    pub line: usize,
}

impl std::fmt::Display for MarkupTooDeep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the markup nests {} levels deep at line {}, deeper than it can be converted",
            self.depth, self.line
        )
    }
}

impl std::error::Error for MarkupTooDeep {}

/// Refuse `markdown` if its block nesting could exceed [`MAX_MARKDOWN_DEPTH`].
pub fn check_markdown(markdown: &str) -> Result<(), MarkupTooDeep> {
    for (index, line) in markdown.split('\n').enumerate() {
        let start = line_start(line, Grammar::Markdown, MAX_MARKDOWN_DEPTH);
        if start.containers > MAX_MARKDOWN_DEPTH {
            return Err(MarkupTooDeep {
                depth: start.containers,
                line: index + 1,
            });
        }
    }
    Ok(())
}

/// `markdown` with every line's container markers and indentation taken off, and
/// each line made a paragraph of its own.
///
/// What a converter reads in place of a document [`check_markdown`] refused: the
/// same words and inline formatting, with nothing left to nest, since every
/// container opens with a marker at the start of a line.
pub(crate) fn markdown_without_nesting(markdown: &str) -> String {
    let mut out = String::with_capacity(markdown.len());
    for line in markdown.split('\n') {
        let rest = line_start(line, Grammar::Markdown, usize::MAX)
            .rest
            .trim_end_matches(|c: char| c.is_ascii_whitespace());
        if rest.is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push_str("\n\n");
        }
        out.push_str(rest);
    }
    out
}

/// Refuse `html` if its elements could nest past [`MAX_HTML_DEPTH`].
///
/// `html` is read as the converter hands it to the parser: without its `<style>`
/// and `<script>` blocks, which the converter removes first.
pub fn check_html(html: &str) -> Result<(), MarkupTooDeep> {
    check_clean_html(&crate::convert::without_style_and_script(html))
}

/// [`check_html`] for HTML whose `<style>` and `<script>` blocks are already gone.
pub(crate) fn check_clean_html(html: &str) -> Result<(), MarkupTooDeep> {
    let mut open: Vec<(&str, usize)> = Vec::new();
    let mut depth = 0usize;
    for token in Tokens::new(html) {
        match token {
            Token::Start { name, at } => {
                if is_one_of(name, UNFOLLOWED) {
                    return check_every_tag_from(html, at, depth);
                }
                close_before(&mut open, &mut depth, name);
                let weight = weight(name);
                if weight == 0 {
                    continue;
                }
                open.push((name, weight));
                depth += weight;
                if depth > MAX_HTML_DEPTH {
                    return Err(MarkupTooDeep {
                        depth,
                        line: line_of(html, at),
                    });
                }
            }
            Token::End { name } => {
                if let Some(&(top, weight)) = open.last()
                    && top.eq_ignore_ascii_case(name)
                {
                    open.pop();
                    depth -= weight;
                }
            }
            Token::Text(_) => {}
        }
    }
    Ok(())
}

/// How many levels a start tag named `name` opens.
fn weight(name: &str) -> usize {
    if is_one_of(name, VOID) || is_one_of(name, IGNORED) {
        0
    } else {
        opened_by(name)
    }
}

/// How many levels a start tag named `name` may open when it opens any: a table
/// three, for the body and the row the tree builder supplies inside it when the
/// markup leaves them out, and every other element one.
fn opened_by(name: &str) -> usize {
    if name.eq_ignore_ascii_case("table") {
        3
    } else {
        1
    }
}

/// The count from a tag whose tokenizing depends on where the tree builder is, at
/// byte `from`, to the end: every `<` followed by a letter, anywhere, opens a level,
/// and nothing closes one. `depth` is what was open before it.
///
/// No tag the tokenizer could read is missed however it reads what follows, since
/// the count does not tokenize at all, and nothing the tree builder closes can
/// matter to a count that never goes down.
fn check_every_tag_from(html: &str, from: usize, mut depth: usize) -> Result<(), MarkupTooDeep> {
    let bytes = html.as_bytes();
    let mut at = from;
    while let Some(offset) = bytes
        .get(at..)
        .and_then(|rest| rest.iter().position(|&b| b == b'<'))
    {
        let tag = at + offset;
        at = tag + 1;
        let name_len = bytes
            .get(at..)
            .unwrap_or_default()
            .iter()
            .take_while(|byte| byte.is_ascii_alphanumeric())
            .count();
        let starts_with_letter = bytes.get(at).is_some_and(u8::is_ascii_alphabetic);
        if !starts_with_letter {
            continue;
        }
        depth += opened_by(html.get(at..at + name_len).unwrap_or_default());
        if depth > MAX_HTML_DEPTH {
            return Err(MarkupTooDeep {
                depth,
                line: line_of(html, tag),
            });
        }
    }
    Ok(())
}

/// Close what the tree builder surely closes on its current element when a start
/// tag named `name` arrives: a paragraph before a block, a list item before a list
/// item, a table cell (and its row) before a cell or a row.
fn close_before(open: &mut Vec<(&str, usize)>, depth: &mut usize, name: &str) {
    let mut close_top = |open: &mut Vec<(&str, usize)>, names: &[&str]| {
        if let Some(&(top, weight)) = open.last()
            && is_one_of(top, names)
        {
            open.pop();
            *depth -= weight;
        }
    };
    if is_one_of(name, CLOSES_A_PARAGRAPH) {
        close_top(open, &["p"]);
    }
    if name.eq_ignore_ascii_case("li") {
        close_top(open, &["li"]);
    } else if is_one_of(name, &["dd", "dt"]) {
        close_top(open, &["dd", "dt"]);
    } else if is_one_of(name, &["td", "th"]) {
        close_top(open, &["td", "th"]);
    } else if name.eq_ignore_ascii_case("tr") {
        close_top(open, &["td", "th"]);
        close_top(open, &["tr"]);
    }
}

/// `html` as paragraphs of its words and nothing else: every block element, line
/// break and table cell ends one, and the text between them is kept as it was
/// written, character references and all.
///
/// What a converter reads in place of a document [`check_html`] refused. The text
/// of `<title>`, `<noscript>` and `<template>` is left out, as the converter's own
/// reader leaves it out.
pub(crate) fn html_without_nesting(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut paragraph = String::new();
    let mut hidden = 0usize;
    let flush = |paragraph: &mut String, out: &mut String| {
        if !paragraph.trim().is_empty() {
            out.push_str("<p>");
            out.push_str(paragraph);
            out.push_str("</p>\n");
        }
        paragraph.clear();
    };
    for token in Tokens::new(html) {
        match token {
            Token::Start { name, .. } => {
                if is_one_of(name, HIDDEN) {
                    hidden += 1;
                } else if is_one_of(name, BREAKS) {
                    flush(&mut paragraph, &mut out);
                }
            }
            Token::End { name } => {
                if is_one_of(name, HIDDEN) {
                    hidden = hidden.saturating_sub(1);
                } else if is_one_of(name, BREAKS) {
                    flush(&mut paragraph, &mut out);
                }
            }
            Token::Text(text) if hidden == 0 => {
                // Every `<` in a text run is one the tokenizer read as text; escaped,
                // the reader reads it the same way whatever follows it.
                paragraph.push_str(&text.replace('<', "&lt;"));
            }
            Token::Text(_) => {}
        }
    }
    flush(&mut paragraph, &mut out);
    out
}

/// Elements that never hold anything, so open no level. The tree builder inserts
/// and closes them at once, or ignores them.
const VOID: &[&str] = &[
    "area", "base", "basefont", "bgsound", "br", "col", "embed", "hr", "img", "input", "keygen",
    "link", "meta", "param", "source", "track", "wbr",
];

/// Tags the tree builder never opens a level for inside a body: the document's own
/// frame, which a fragment already has.
const IGNORED: &[&str] = &["html", "head", "body"];

/// Start tags after which the tokenizer's reading of the markup depends on the tree
/// builder's state rather than on the markup alone: foreign content (`<svg>`,
/// `<math>`), where `/>` closes and CDATA is text; `<select>`, inside which most
/// tags are dropped and the ones that switch the tokenizer do not; and `<script>`,
/// whose text ends in one of several places. The converter removes `<script>`
/// before this scan, so meeting one means the text came from elsewhere.
const UNFOLLOWED: &[&str] = &["svg", "math", "select", "script"];

/// Elements whose text the tokenizer reads as text up to their own end tag, whatever
/// it holds, everywhere the scan follows the markup.
const RAW_TEXT: &[&str] = &[
    "title", "textarea", "style", "xmp", "iframe", "noembed", "noframes", "noscript",
];

/// Start tags before which the tree builder closes an open paragraph.
const CLOSES_A_PARAGRAPH: &[&str] = &[
    "address",
    "article",
    "aside",
    "blockquote",
    "center",
    "details",
    "dialog",
    "dir",
    "div",
    "dl",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "header",
    "hgroup",
    "main",
    "menu",
    "nav",
    "ol",
    "p",
    "search",
    "section",
    "summary",
    "ul",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "pre",
    "listing",
    "form",
    "li",
    "dd",
    "dt",
    "plaintext",
    "hr",
    "xmp",
];

/// Elements whose text the converter's reader does not keep as prose.
const HIDDEN: &[&str] = &["title", "noscript", "template"];

/// Elements that start or end a block of text, and the line break.
const BREAKS: &[&str] = &[
    "address",
    "article",
    "aside",
    "blockquote",
    "br",
    "caption",
    "center",
    "dd",
    "details",
    "dialog",
    "dir",
    "div",
    "dl",
    "dt",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hgroup",
    "hr",
    "li",
    "listing",
    "main",
    "menu",
    "nav",
    "ol",
    "p",
    "pre",
    "section",
    "summary",
    "table",
    "tbody",
    "td",
    "tfoot",
    "th",
    "thead",
    "tr",
    "ul",
    "xmp",
];

fn is_one_of(name: &str, names: &[&str]) -> bool {
    names.iter().any(|known| known.eq_ignore_ascii_case(name))
}

fn line_of(text: &str, at: usize) -> usize {
    1 + text
        .as_bytes()
        .get(..at)
        .unwrap_or_default()
        .iter()
        .filter(|&&byte| byte == b'\n')
        .count()
}

/// What the HTML tokenizer hands the tree builder, as far as nesting goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token<'a> {
    /// A start tag, named as written, at byte `at`.
    Start { name: &'a str, at: usize },
    /// An end tag, named as written.
    End { name: &'a str },
    /// Text between tags, character references still in it.
    Text(&'a str),
}

/// HTML read the way the standard's tokenizer reads it, in the states that decide
/// where a tag starts and ends: comments (including the abrupt `<!-->`), bogus
/// comments and doctypes to the first `>`, attributes through their quoted values,
/// and the raw text of [`RAW_TEXT`] elements to their own end tag. `<plaintext>`
/// makes the rest of the document text, as it does in the tokenizer.
struct Tokens<'a> {
    html: &'a str,
    at: usize,
    plaintext: bool,
    /// Where the raw text of the element just opened ends, once one is open.
    raw_text_end: Option<usize>,
}

impl<'a> Tokens<'a> {
    fn new(html: &'a str) -> Self {
        Self {
            html,
            at: 0,
            plaintext: false,
            raw_text_end: None,
        }
    }

    /// The offset of the end tag closing a raw text element named `name`, whose
    /// text starts at `from`: the first `</` followed by the name in any case and
    /// then whitespace, `/` or `>`, or the end of the input when there is none.
    fn raw_text_end(&self, from: usize, name: &str) -> usize {
        let bytes = self.html.as_bytes();
        let mut at = from;
        while let Some(offset) = bytes
            .get(at..)
            .and_then(|rest| rest.windows(2).position(|pair| pair == b"</"))
        {
            let close = at + offset;
            let name_at = close + 2;
            let named = bytes
                .get(name_at..name_at + name.len())
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name.as_bytes()));
            let delimited = bytes
                .get(name_at + name.len())
                .is_some_and(|&byte| byte.is_ascii_whitespace() || matches!(byte, b'/' | b'>'));
            if named && delimited {
                return close;
            }
            at = name_at;
        }
        bytes.len()
    }

    fn byte(&self, at: usize) -> Option<u8> {
        self.html.as_bytes().get(at).copied()
    }

    /// The offset just past the first `needle` at or after `from`, or the end.
    fn past(&self, from: usize, needle: &[u8]) -> usize {
        self.html
            .as_bytes()
            .get(from..)
            .and_then(|rest| rest.windows(needle.len()).position(|w| w == needle))
            .map_or(self.html.len(), |offset| from + offset + needle.len())
    }

    /// The end of a comment whose `<!--` ends just before `from`.
    fn comment_end(&self, from: usize) -> usize {
        let rest = self.html.as_bytes().get(from..).unwrap_or_default();
        if rest.starts_with(b">") {
            return from + 1;
        }
        if rest.starts_with(b"->") {
            return from + 2;
        }
        let closed = self.past(from, b"-->");
        let banged = self.past(from, b"--!>");
        closed.min(banged)
    }

    /// Read a tag whose name starts at `from` through its attributes, returning its
    /// name and the offset just past its `>` (or the end of the input).
    fn tag(&self, from: usize) -> (&'a str, usize) {
        let bytes = self.html.as_bytes();
        let name_end = from
            + bytes
                .get(from..)
                .unwrap_or_default()
                .iter()
                .take_while(|&&byte| !byte.is_ascii_whitespace() && byte != b'/' && byte != b'>')
                .count();
        let name = self.html.get(from..name_end).unwrap_or_default();
        let mut at = name_end;
        // The tokenizer's attribute states, collapsed to what moves the end of the
        // tag: a quote only opens a value right after `=`.
        loop {
            // Before an attribute name, or after one.
            while self
                .byte(at)
                .is_some_and(|byte| byte.is_ascii_whitespace() || byte == b'/')
            {
                at += 1;
            }
            match self.byte(at) {
                None => return (name, at),
                Some(b'>') => return (name, at + 1),
                _ => {}
            }
            // The attribute name: a leading `=` belongs to it.
            at += 1;
            while self.byte(at).is_some_and(|byte| {
                !byte.is_ascii_whitespace() && !matches!(byte, b'/' | b'>' | b'=')
            }) {
                at += 1;
            }
            while self.byte(at).is_some_and(|byte| byte.is_ascii_whitespace()) {
                at += 1;
            }
            if self.byte(at) != Some(b'=') {
                continue;
            }
            at += 1;
            while self.byte(at).is_some_and(|byte| byte.is_ascii_whitespace()) {
                at += 1;
            }
            match self.byte(at) {
                Some(quote @ (b'"' | b'\'')) => {
                    at = self.past(at + 1, &[quote]);
                }
                Some(b'>') | None => {}
                Some(_) => {
                    while self
                        .byte(at)
                        .is_some_and(|byte| !byte.is_ascii_whitespace() && byte != b'>')
                    {
                        at += 1;
                    }
                }
            }
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let bytes = self.html.as_bytes();
        loop {
            let start = self.at;
            if start >= bytes.len() {
                return None;
            }
            if self.plaintext {
                self.at = bytes.len();
                return self.html.get(start..).map(Token::Text);
            }
            if let Some(end) = self.raw_text_end.take() {
                self.at = end;
                if end > start {
                    return self.html.get(start..end).map(Token::Text);
                }
                continue;
            }
            if bytes[start] != b'<' {
                let end = bytes[start..]
                    .iter()
                    .position(|&byte| byte == b'<')
                    .map_or(bytes.len(), |offset| start + offset);
                self.at = end;
                return self.html.get(start..end).map(Token::Text);
            }
            let next = self.byte(start + 1);
            match next {
                Some(b'!') => {
                    self.at = if bytes[start..].starts_with(b"<!--") {
                        self.comment_end(start + 4)
                    } else {
                        // A doctype, a bogus comment, or a CDATA section outside
                        // foreign content: all end at the first `>`.
                        self.past(start + 2, b">")
                    };
                }
                Some(b'?') => self.at = self.past(start + 2, b">"),
                Some(b'/') => match self.byte(start + 2) {
                    Some(byte) if byte.is_ascii_alphabetic() => {
                        let (name, end) = self.tag(start + 2);
                        self.at = end;
                        return Some(Token::End { name });
                    }
                    Some(b'>') => self.at = start + 3,
                    None => {
                        self.at = bytes.len();
                        return self.html.get(start..).map(Token::Text);
                    }
                    Some(_) => self.at = self.past(start + 2, b">"),
                },
                Some(byte) if byte.is_ascii_alphabetic() => {
                    let (name, end) = self.tag(start + 1);
                    self.at = end;
                    if name.eq_ignore_ascii_case("plaintext") {
                        self.plaintext = true;
                    } else if is_one_of(name, RAW_TEXT) {
                        self.raw_text_end = Some(self.raw_text_end(end, name));
                    }
                    return Some(Token::Start { name, at: start });
                }
                _ => {
                    // A `<` that opens nothing is text, up to the next `<`.
                    let end = bytes[start + 1..]
                        .iter()
                        .position(|&byte| byte == b'<')
                        .map_or(bytes.len(), |offset| start + 1 + offset);
                    self.at = end;
                    return self.html.get(start..end).map(Token::Text);
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "markup_depth_tests.rs"]
mod tests;
