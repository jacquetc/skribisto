// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A ceiling on how deeply nested the Djot in a bundle may be.
//!
//! # The failure this prevents
//!
//! The Djot parser (`jotdown` 0.10, reached through `text-document`) descends once
//! per nested block container (`parse_block` calls `parse_container`, which calls
//! `parse_block` for the container's content) and has no depth limit of its own.
//! That is not a panic: **a stack overflow aborts the process**, it cannot be
//! caught by `catch_unwind` or a panic hook, and every unsaved document in every
//! window of that process dies with it.
//!
//! Measured on the 2 MiB stack a long operation's thread gets, in a debug build:
//! `jotdown` parses 616 list items nested on one line and aborts at 617, and every
//! other kind of container costs it about the same. A line of `"- ".repeat(617)`
//! is a little over a kilobyte.
//!
//! The parse happens when a document is opened or exported, not during
//! `read_folder`, but a bundle is the boundary the bytes cross, and it is the last
//! place a refusal can still name a file and leave the writer's own project
//! untouched.
//!
//! # What is held to it
//!
//! Every piece of Djot a bundle stores, as the bundle is read: each row's prose,
//! each note template's body, and the body of every comment, every reply and every
//! footnote, in the sidecars beside the prose and in the two orphanages (`check_comments`
//! and `check_footnotes` below). Those last three are not prose, but they are parsed
//! all the same, by the comment and footnote cards on the UI thread and by the
//! exporter, so a body past the ceiling aborts the process exactly as a scene would.
//! A comment or reply body a bundle stamped before v12 stores is plain text rather than
//! Djot, and the load rewrites it as Djot a load always accepts, so it is left to that.
//! The readers of a project's past hold what they hand over to it too: the history
//! log's blobs, and a backup's prose and comment threads.
//!
//! # Why a scan rather than a parser limit
//!
//! The parser is the right place for a depth limit and this is not it (see the
//! note at the bottom). What this module can do without reaching into that crate
//! is bound the *input*: a container is only ever opened by a marker at the start
//! of a line, and only ever continued by a marker, by indentation or by a fence
//! that is still open, so counting those is an upper bound on how deep the parser
//! can go.
//!
//! It is deliberately an over-estimate. Every construct counted here *may* open a
//! container and some will not (a `>` inside a code block is prose). Over-counting
//! is the safe direction: it can only refuse a document that was closer to the
//! ceiling than it looked, and the ceiling is set far above anything a person
//! writes.
//!
//! # What is counted, line by line
//!
//! The markers `jotdown` 0.10 opens a container with at the start of a line, all
//! of them, and any number of them in a row, since each opens the next one's
//! line (`- > 1. [^a]: x` is four containers deep on one line):
//!
//! * a blockquote, `>`;
//! * a list item of every kind: a bullet (`-`, `*`, `+`), a task (`- [ ]`), an
//!   ordered item numbered with digits, a letter or roman numerals and closed by
//!   `.` or `)` or enclosed in `( )`, and a definition-list item, `:`;
//! * a footnote definition, `[^note]:`, and a link definition, `[link]:`, which
//!   opens nothing but is counted all the same;
//! * a table row, `|`;
//! * a `:::` div, which stays open on the lines after it (below).
//!
//! A container opened on an earlier line is continued by a `>` (a blockquote), by
//! at least one byte of indentation (a list item or a footnote, which strip at
//! least one byte of each line they continue), or by nothing at all (a div, until
//! its closing fence). So the whitespace in front of the last marker on a line
//! counts one level per byte, except for the one byte after a `>` that the
//! blockquote itself consumes.
//!
//! # A line that opens nothing counts nothing
//!
//! Indentation alone opens no container. `jotdown` identifies a line's block after
//! trimming every byte of its leading whitespace (`IdentifiedBlock::new`), and Djot
//! has no indented code block: a line of whitespace and words is a paragraph, a
//! leaf, however far it is indented. Indentation only decides whether a line goes on
//! *continuing* a list item, a footnote or a definition opened on an earlier line
//! (`Kind::continues`, which asks for more whitespace than the item's own marker
//! had), and a line continuing a container sits exactly as deep as the line that
//! opened it, which was measured when it was read. So a line with no marker adds
//! nothing past the divs still open around it.
//!
//! The previous guard counted one level per two bytes of such indentation, and the
//! editor writes a paragraph's leading spaces and tabs back as they were typed: a
//! paragraph opening with 194 of them saved, and the next load refused the whole
//! project, locking the writer out of their own work.
//!
//! **Whitespace here is ASCII whitespace**, the only kind the parser reads as
//! indentation or as the space that ends a marker (`jotdown`'s block scanner tests
//! `is_ascii_whitespace` at every one of those points). A paragraph opening with a
//! hundred no-break spaces is a paragraph whose text starts with them.
//!
//! # Divs
//!
//! A div is the one container nothing on its later lines has to mention, so the
//! scan keeps the open ones in a list and counts them on every line. A fence opens
//! one unless it closes one, and a fence closes a div only when it is bare (colons
//! and nothing else), at least as long as the div's own, in the same blockquotes,
//! and not inside a code block the div saw open. That last rule is `jotdown`'s
//! (`nested_raw`), and it is mirrored here because it is exactly what lets a fence
//! that looks like a close open a div instead.
//!
//! A div is only forgotten when the parser is certain to have closed it: a bare
//! fence that matches it, or a line that ends the blockquotes around it. A bare
//! fence is counted as opening a div as well, since it does when it closes nothing,
//! and a div opened after a list or definition marker is never forgotten. Both
//! over-count, by one open div for each run of sibling divs, which is the price of
//! a count that no arrangement of fences can talk down. A run of fences each one
//! colon shorter than the last opens a div inside a div on every line, and a
//! `> ::: note` repeated on every line of a quotation opens one more on each; both
//! used to pass the previous guard at any depth.
//!
//! # The limit
//!
//! [`MAX_DEPTH`](crate::MAX_DJOT_DEPTH) is 96. For scale: a blockquote inside a
//! list inside a footnote inside a div is 4, and the Djot `text-document` writes
//! for a list nested ten deep, two spaces a level, counts 19 here. No manuscript
//! reaches 96, and 96 is far below the 617 that overflows a debug build.

use anyhow::Context;

use crate::bundle::{CommentFile, FootnoteFile};

/// The most nested block containers a bundle's prose may declare.
pub const MAX_DEPTH: usize = 96;

/// Why a prose blob was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TooDeep {
    /// The depth the scan measured.
    pub depth: usize,
    /// The 1-based line it was measured on.
    pub line: usize,
}

impl std::fmt::Display for TooDeep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "prose nests {} levels deep at line {}, and the limit is {MAX_DEPTH}. \
             Prose nested this deeply would crash the Djot parser, so it is refused \
             rather than opened",
            self.depth, self.line
        )
    }
}

impl std::error::Error for TooDeep {}

/// Refuse `text` if its block nesting could exceed [`MAX_DEPTH`].
///
/// See the module note for what is counted. The scan is a loop over the lines,
/// never a recursion, and stops at the first line past the ceiling.
pub fn check(text: &str) -> Result<(), TooDeep> {
    let mut divs = OpenDivs::default();
    for (index, line) in text.split('\n').enumerate() {
        let start = line_start(line, Grammar::Djot, MAX_DEPTH);
        divs.read(&start);
        // Indentation in front of no marker is not counted: see the module note.
        let depth = divs.count() + start.containers;
        if depth > MAX_DEPTH {
            return Err(TooDeep {
                depth,
                line: index + 1,
            });
        }
    }
    Ok(())
}

/// Refuse a list of comment threads if the body of any comment or reply in it could
/// nest past [`MAX_DEPTH`], naming that body by its place in the list.
///
/// One file at a time: a `.comments.ron` sidecar or `orphan_comments.ron`, whose name
/// the caller adds. Positions count from one, as a writer counts, and the refusal's
/// own line is the line inside that body.
pub(crate) fn check_comments(threads: &[CommentFile]) -> anyhow::Result<()> {
    for (index, comment) in threads.iter().enumerate() {
        check(&comment.body).with_context(|| format!("comment {}", index + 1))?;
        for (turn, reply) in comment.replies.iter().enumerate() {
            check(&reply.body)
                .with_context(|| format!("reply {} to comment {}", turn + 1, index + 1))?;
        }
    }
    Ok(())
}

/// Refuse a list of footnotes if the body of any could nest past [`MAX_DEPTH`],
/// naming that note by its place in the list.
///
/// By place and not by label: the label is the file's own string, of any length,
/// and the refusal is what the writer reads.
pub(crate) fn check_footnotes(notes: &[FootnoteFile]) -> anyhow::Result<()> {
    for (index, note) in notes.iter().enumerate() {
        check(&note.body).with_context(|| format!("footnote {}", index + 1))?;
    }
    Ok(())
}

/// Which markup a line is read as.
///
/// Djot and Markdown open their containers with nearly the same markers, so one
/// scan serves both: every marker either grammar opens a container with is counted
/// in both, which only ever over-counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Grammar {
    /// Djot as `jotdown` 0.10 reads it, where a tab is one byte of indentation.
    Djot,
    /// Markdown as `pulldown-cmark` reads it, where a tab reaches the next tab stop,
    /// up to four columns, and every column can continue a list.
    Markdown,
}

impl Grammar {
    /// How many levels one byte of leading whitespace can continue.
    fn width(self, byte: u8) -> usize {
        match (self, byte) {
            (Grammar::Markdown, b'\t') => 4,
            _ => 1,
        }
    }
}

/// What the markers at the start of one line add up to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LineStart<'a> {
    /// An upper bound on the containers the line opens or continues through its own
    /// markers: one per marker, and one per level the whitespace in front of the last
    /// marker could continue. The scan stops once this passes its limit, so past the
    /// limit it is only known to be past it.
    pub containers: usize,
    /// How many `>` the line opens with before any other marker, whitespace aside.
    pub quotes: usize,
    /// Whether those `>` are all the line opens before [`Self::rest`], each followed
    /// by whitespace or the end of the line, so that what follows is read inside
    /// exactly that many blockquotes.
    pub only_quotes: bool,
    /// The line after its markers and the whitespace between them.
    pub rest: &'a str,
}

/// Read the markers at the start of `line` (no line break in it) as `grammar`
/// opens containers with them, stopping once more than `limit` are counted.
pub(crate) fn line_start(line: &str, grammar: Grammar, limit: usize) -> LineStart<'_> {
    let bytes = line.as_bytes();
    // From this offset to the end the line is nothing but `-`, `*` and whitespace,
    // which is where a `-` or `*` may be a thematic break rather than a bullet.
    let decoration_from = bytes
        .iter()
        .rposition(|&byte| !matches!(byte, b'-' | b'*') && !byte.is_ascii_whitespace())
        .map_or(0, |last| last + 1);

    let mut containers = 0usize;
    // Levels the whitespace since the last marker could continue, counted once
    // another marker follows it.
    let mut pending = 0usize;
    // Whether a marker other than `>` has opened a container on this line: every
    // container after it is new, and whitespace in front of a new one continues
    // nothing.
    let mut opened = false;
    // Whether the next whitespace byte ends the `>` before it.
    let mut after_quote = false;
    let mut quotes = 0usize;
    let mut only_quotes = true;
    let mut at = 0usize;

    while containers <= limit {
        let Some(&byte) = bytes.get(at) else {
            break;
        };
        if byte.is_ascii_whitespace() {
            let width = grammar.width(byte);
            if after_quote {
                pending += width - 1;
            } else if !opened {
                pending += width;
            }
            after_quote = false;
            at += 1;
            continue;
        }
        after_quote = false;
        let ends_marker = |offset: usize| {
            bytes
                .get(offset)
                .is_none_or(|byte| byte.is_ascii_whitespace())
        };

        let marker = match byte {
            b'>' => {
                if !opened {
                    quotes += 1;
                    if !ends_marker(at + 1) {
                        only_quotes = false;
                    }
                }
                after_quote = true;
                Some(1)
            }
            b'-' | b'*' if at >= decoration_from && is_thematic_break(&bytes[at..]) => None,
            b'-' | b'*' | b'+' => ends_marker(at + 1).then_some(1),
            b':' => ends_marker(at + 1).then_some(1),
            b'[' => match definition(&bytes[at..]) {
                Some(len) => Some(len),
                None if opened && task_box(&bytes[at..]) => {
                    // The box belongs to the bullet before it: nothing new opens.
                    at += 3;
                    continue;
                }
                None => None,
            },
            b'|' => {
                // A table row: one container, and its cells hold no blocks. The row
                // itself stays in `rest`, as the words it holds.
                containers += pending + 1;
                only_quotes = false;
                break;
            }
            _ => ordered_marker(&bytes[at..]),
        };
        let Some(len) = marker else {
            break;
        };
        containers += pending + 1;
        pending = 0;
        if byte != b'>' {
            opened = true;
            only_quotes = false;
        }
        at += len;
    }

    let rest = line
        .get(at.min(line.len())..)
        .unwrap_or_default()
        .trim_start_matches(|c: char| c.is_ascii_whitespace());
    LineStart {
        containers,
        quotes,
        only_quotes,
        rest,
    }
}

/// Whether `from`, starting at a `-` or `*`, is a thematic break: three or more of
/// that one mark and nothing else but whitespace. A break opens nothing, and both
/// parsers test for one before they test for a bullet.
///
/// Markdown needs the marks to match, and `jotdown` takes a mixture as well. Only a
/// run of matching marks is taken for a break here, so a mixed one counts as the
/// bullets Markdown reads it as, which is more than `jotdown` reads.
fn is_thematic_break(from: &[u8]) -> bool {
    let Some(&mark) = from.first() else {
        return false;
    };
    let mut marks = 0usize;
    for &byte in from {
        if byte == mark {
            marks += 1;
        } else if !byte.is_ascii_whitespace() {
            return false;
        }
    }
    marks >= 3
}

/// The length of a footnote or link definition's `[label]:` opening `from`, or
/// `None`. The label runs to the first `]`, as `jotdown` reads it.
fn definition(from: &[u8]) -> Option<usize> {
    let close = from.get(1..)?.iter().position(|&byte| byte == b']')? + 1;
    (from.get(close + 1) == Some(&b':')).then_some(close + 2)
}

/// Whether `from` is a task list's box, `[ ]`, `[x]` or `[X]`, then whitespace or
/// the end of the line.
fn task_box(from: &[u8]) -> bool {
    matches!(from.get(..3), Some([b'[', b' ' | b'x' | b'X', b']']))
        && from.get(3).is_none_or(u8::is_ascii_whitespace)
}

/// The length of an ordered list marker opening `from`, or `None`.
///
/// `jotdown` 0.10's `maybe_ordered_list_item`, without its length limits (which
/// only ever turn a marker away): an optional `(`, then digits, a run of roman
/// numerals all of one case, or one letter, then `)` (always, after a `(`) or `.`,
/// then whitespace or the end of the line. Markdown's own markers, digits then `.`
/// or `)`, are among them.
fn ordered_marker(from: &[u8]) -> Option<usize> {
    fn roman_lower(byte: &u8) -> bool {
        matches!(byte, b'i' | b'v' | b'x' | b'l' | b'c' | b'd' | b'm')
    }
    fn roman_upper(byte: &u8) -> bool {
        matches!(byte, b'I' | b'V' | b'X' | b'L' | b'C' | b'D' | b'M')
    }
    let paren = from.first() == Some(&b'(');
    let number = usize::from(paren);
    let first = *from.get(number)?;
    let digits = from.get(number..).unwrap_or_default();
    let run = if first.is_ascii_digit() {
        digits
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count()
    } else if roman_lower(&first) {
        digits.iter().take_while(|byte| roman_lower(byte)).count()
    } else if roman_upper(&first) {
        digits.iter().take_while(|byte| roman_upper(byte)).count()
    } else if first.is_ascii_alphabetic() {
        1
    } else {
        return None;
    };
    let delimiter = number + run;
    let closes = match from.get(delimiter) {
        Some(b')') => true,
        Some(b'.') => !paren,
        _ => false,
    };
    (closes && from.get(delimiter + 1).is_none_or(u8::is_ascii_whitespace)).then_some(delimiter + 1)
}

/// A fence at the start of what follows a line's markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fence {
    /// A div's `:::`, at least three colons long. `bare` when nothing follows the
    /// colons, which is the only fence that can close a div.
    Div { len: usize, bare: bool },
    /// A code block's fence, at least three backticks or tildes long.
    Code { mark: u8, len: usize, bare: bool },
}

/// The fence `rest` opens with, as `jotdown`'s `IdentifiedBlock` reads one.
fn fence(rest: &str) -> Option<Fence> {
    let bytes = rest.as_bytes();
    let mark = *bytes.first()?;
    if !matches!(mark, b':' | b'`' | b'~') {
        return None;
    }
    let len = bytes.iter().take_while(|&&byte| byte == mark).count();
    if len < 3 {
        return None;
    }
    let spec = rest
        .get(len..)
        .unwrap_or_default()
        .trim_matches(|c: char| c.is_ascii_whitespace());
    let bare = spec.is_empty();
    if mark == b':' {
        let valid = spec
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'_' | b'-'));
        valid.then_some(Fence::Div { len, bare })
    } else {
        let valid = !spec
            .bytes()
            .any(|byte| byte.is_ascii_whitespace() || byte == b'`');
        valid.then_some(Fence::Code { mark, len, bare })
    }
}

/// A div the scan counts as open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OpenDiv {
    /// Its fence's length in colons: only a bare fence at least this long closes it.
    len: usize,
    /// The blockquotes its fence sat inside, and in which a closing fence must sit.
    quotes: usize,
    /// Whether its fence followed nothing but those blockquotes. A div opened after a
    /// list or definition marker is never known to be closed, and stays counted.
    closable: bool,
    /// The code fence it saw open and not yet closed, `jotdown`'s `nested_raw`: while
    /// one is open, no fence closes the div.
    raw: Option<(u8, usize)>,
}

/// The divs a Djot document may have open at the line being read.
#[derive(Debug, Default)]
struct OpenDivs {
    open: Vec<OpenDiv>,
}

impl OpenDivs {
    fn count(&self) -> usize {
        self.open.len()
    }

    /// Account for the line `start` describes.
    fn read(&mut self, start: &LineStart<'_>) {
        let fence = fence(start.rest);
        if start.only_quotes {
            let quotes = start.quotes;
            // A line inside fewer blockquotes that is a fence or blank ends every
            // blockquote past them, and every div inside those: a blockquote is only
            // continued by its own marker or by a paragraph running on.
            if fence.is_some() || start.rest.is_empty() {
                self.open.retain(|div| div.quotes <= quotes);
            }
            match fence {
                Some(Fence::Code { mark, len, bare }) => {
                    for div in self.open.iter_mut().filter(|div| div.quotes == quotes) {
                        div.raw = match div.raw {
                            Some((open_mark, open_len))
                                if open_mark == mark && len >= open_len && bare =>
                            {
                                None
                            }
                            Some(open) => Some(open),
                            None => Some((mark, len)),
                        };
                    }
                }
                Some(Fence::Div { len, bare }) => {
                    if bare {
                        self.open.retain(|div| {
                            !(div.closable
                                && div.quotes == quotes
                                && div.len <= len
                                && div.raw.is_none())
                        });
                    }
                    self.open.push(OpenDiv {
                        len,
                        quotes,
                        closable: true,
                        raw: None,
                    });
                }
                None => {}
            }
        } else if let Some(Fence::Div { len, .. }) = fence {
            self.open.push(OpenDiv {
                len,
                quotes: start.quotes,
                closable: false,
                raw: None,
            });
        }
    }
}

// A depth limit inside `jotdown` would be strictly better than this: it would
// bound the recursion itself rather than an over-estimate of it, and it would
// cover every caller rather than the ones that remember to ask. That fix belongs
// in `jotdown` (or in `text-document`, which wraps it) and is not this crate's
// to make. This module is the boundary guard that does not require it.

/// `pub(crate)` for the hostile prose it builds, which every reader of stored Djot is
/// tested against.
#[cfg(test)]
#[path = "djot_depth_tests.rs"]
pub(crate) mod tests;
