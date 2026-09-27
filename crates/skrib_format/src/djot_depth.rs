// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the Djot parser cannot be given, refused as a bundle is read.
//!
//! # The failures this prevents
//!
//! The Djot parser (`jotdown` 0.10, reached through `text-document`) has five limits
//! of its own that a piece of Djot can cross, and none of them ends in an error:
//!
//! * **Nesting.** It descends once per nested block container (`parse_block` calls
//!   `parse_container`, which calls `parse_block` for the container's content) and has
//!   no depth limit. That is not a panic: **a stack overflow aborts the process**, it
//!   cannot be caught by `catch_unwind` or a panic hook, and every unsaved document in
//!   every window of that process dies with it. Measured on the 2 MiB stack a long
//!   operation's thread gets, in a debug build: 616 list items nested on one line
//!   parse, and 617 abort. A line of `"- ".repeat(617)` is a little over a kilobyte.
//! * **A heading's level.** It counts every `#` before the space and stores the count
//!   in 16 bits with an `unwrap` (`block.rs`, `level.try_into().unwrap()`): a heading
//!   of 65,536 `#` panics, on the thread that opens the row.
//! * **Sections.** Each heading outside every container opens a section inside the
//!   open sections of lower levels, and a list stores how many containers are open
//!   around it, sections included, in 16 bits with an `unwrap` too
//!   (`self.open.len().try_into().unwrap()`).
//! * **Inline work.** Its inline pass looks down the stack of openers nothing has
//!   closed yet for every token, so a paragraph of unmatched `[` costs the square of
//!   its length: 320 KB of them froze the window for most of a minute. See
//!   `djot_inline`.
//! * **Lines held open.** While its inline pass holds something open (an opener, a
//!   verbatim span, an attribute set), each further line of the paragraph is handed to
//!   it one call deeper, and none of them returns until the pass lets go. One quotation
//!   mark nothing closes, then a few thousand more lines of the same paragraph, aborts
//!   the process like the nesting does. See `djot_inline` too.
//!
//! The parse happens when a document is opened or exported, not during `read_folder`,
//! but a bundle is the boundary the bytes cross, and it is the last place a refusal
//! can still name a file and leave the writer's own project untouched.
//!
//! # Joined, not refused
//!
//! The editor writes a paragraph over several lines when it holds a pasted preformatted
//! passage, and one formatted from end to end, or opened by a quotation mark nothing
//! closes, holds every one of its lines. A load does not refuse it:
//! [`admit`](crate::djot_depth::admit) writes the line breaks of such a paragraph as
//! spaces, which is how the editor reads them, and hands on the joined text. The limits
//! are refused only when joining cannot bring a text within them.
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
//! The parser is the right place for these limits and this is not it (see the note at
//! the bottom). What this module can do without reaching into that crate is bound the
//! *input*, by reading it the way the parser will.
//!
//! # The nesting is counted exactly, and only exactly
//!
//! [`check`](crate::djot_depth::check) counts the nesting the parser builds with the
//! scan in `djot_nesting`, which follows `jotdown`'s block pass line by line, each open
//! container continued by the parser's own rule: a list item going on through a
//! paragraph line at any indentation, a div until the fence that closes it, a code
//! block's lines holding nothing at all. `text-document` refuses on that same count
//! past its own ceiling of 128, so prose this module accepts is never shown there as
//! its raw source.
//!
//! It used to refuse on the larger of that count and a second one, of the markers each
//! line opens with, read line by line. The second could only ever count more than the
//! parser nests, and it did, on lines that nest nothing: a code block's line of a
//! hundred `>`, a pasted preformatted line, a run of `>` with no space after them. Each
//! was prose a writer had typed or imported, saved, and been locked out of at the next
//! load, the project refusing to open over a line the parser reads at depth 0. The
//! exact count is the one the parser keeps, so it is the only one refused on.
//!
//! **Whitespace here is ASCII whitespace**, the only kind the parser reads as
//! indentation or as the space that ends a marker (`jotdown`'s block scanner tests
//! `is_ascii_whitespace` at every one of those points). A paragraph opening with a
//! hundred no-break spaces is a paragraph whose text starts with them.
//!
//! # The limits
//!
//! [`MAX_DEPTH`](crate::MAX_DJOT_DEPTH) is 96. For scale: a blockquote inside a
//! list inside a footnote inside a div is 4, and the Djot `text-document` writes for a
//! list nested ten deep counts 10. No manuscript reaches 96, and 96 is far below the
//! 617 that overflows a debug build.
//!
//! [`MAX_HEADING_LEVEL`](crate::djot_depth::MAX_HEADING_LEVEL) and
//! [`MAX_SECTIONS`](crate::djot_depth::MAX_SECTIONS) are exactly the parser's own 16-bit
//! limits, since nothing short of them can crash it and the editor writes back only
//! the levels it has read: a heading at a level the parser can store is one the next
//! load accepts.
//!
//! [`MAX_HELD_LINES`](crate::djot_depth::MAX_HELD_LINES) is 512, a quarter of the 1,967
//! held lines that fill the smallest stack a parse runs on (a 1 MiB Windows main thread)
//! in a release build. The editor holds lines only in the paragraphs
//! [`admit`](crate::djot_depth::admit) joins, so the ceiling itself is only ever met by
//! Djot written by hand or crafted.

use anyhow::Context;

use crate::bundle::{CommentFile, FootnoteFile};
use crate::djot_inline::Join;

/// The most nested block containers a bundle's prose may declare.
pub const MAX_DEPTH: usize = 96;

pub use crate::djot_inline::MAX_HELD_LINES;

/// The deepest heading `jotdown` can store: its level is a `u16`.
pub const MAX_HEADING_LEVEL: usize = u16::MAX as usize;

/// The most sections that may be open at once. A list stores the count of the
/// containers open around it in a `u16`: the document, every open section, the
/// containers the list item sits in (fewer than [`MAX_DEPTH`]) and the list itself.
pub const MAX_SECTIONS: usize = u16::MAX as usize - 1 - MAX_DEPTH;

/// Why a prose blob was refused for nesting.
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

/// Why a piece of Djot was refused: the first thing in it the parser cannot be given,
/// and the 1-based line it is on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DjotRefusal {
    /// Block containers nested past [`MAX_DEPTH`].
    TooDeep(TooDeep),
    /// A heading of more levels than [`MAX_HEADING_LEVEL`].
    HeadingTooDeep { level: usize, line: usize },
    /// More sections open at once than [`MAX_SECTIONS`].
    TooManySections { sections: usize, line: usize },
    /// More openers left unclosed, before more words, than the inline pass reads in
    /// reasonable time (see `djot_inline`).
    TooSlowToRead { line: usize },
    /// A paragraph, heading or caption that keeps something open over more than
    /// [`MAX_HELD_LINES`] of its lines in a row, and whose lines [`admit`] cannot join
    /// (see `djot_inline`).
    HeldOpenTooLong { line: usize },
}

impl DjotRefusal {
    /// The 1-based line the refusal names.
    pub fn line(&self) -> usize {
        match self {
            DjotRefusal::TooDeep(refused) => refused.line,
            DjotRefusal::HeadingTooDeep { line, .. }
            | DjotRefusal::TooManySections { line, .. }
            | DjotRefusal::TooSlowToRead { line }
            | DjotRefusal::HeldOpenTooLong { line } => *line,
        }
    }

    /// The nesting refusal, when this is one.
    pub fn too_deep(&self) -> Option<&TooDeep> {
        match self {
            DjotRefusal::TooDeep(refused) => Some(refused),
            _ => None,
        }
    }
}

impl std::fmt::Display for DjotRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DjotRefusal::TooDeep(refused) => refused.fmt(f),
            DjotRefusal::HeadingTooDeep { level, line } => write!(
                f,
                "prose opens a heading {level} levels deep at line {line}, and the limit is \
                 {MAX_HEADING_LEVEL}. A heading this deep would crash the Djot parser, so it \
                 is refused rather than opened"
            ),
            DjotRefusal::TooManySections { sections, line } => write!(
                f,
                "prose opens {sections} headings each deeper than the one before by line \
                 {line}, and the limit is {MAX_SECTIONS}. Headings stacked this deeply would \
                 crash the Djot parser, so they are refused rather than opened"
            ),
            DjotRefusal::TooSlowToRead { line } => write!(
                f,
                "prose at line {line} leaves more brackets and formatting marks open than the \
                 Djot parser can read in a reasonable time, so it is refused rather than \
                 opened"
            ),
            DjotRefusal::HeldOpenTooLong { line } => write!(
                f,
                "prose at line {line} keeps a bracket, a quotation mark, a code span or a \
                 formatting mark open over more than {MAX_HELD_LINES} lines that cannot be \
                 joined into one, as in a heading, a table caption or a link's address. The \
                 Djot parser goes one call deeper for each of those lines and sets no limit \
                 of its own, so prose past this margin is refused rather than opened"
            ),
        }
    }
}

impl std::error::Error for DjotRefusal {}

/// Refuse `text` if the Djot parser cannot be given it: see the module note.
///
/// One pass over the lines, never a recursion. It stops at the first line past a
/// limit, and on a text of any shape costs a bounded amount of work per byte.
///
/// This is the limit alone. A reader handing the text on to the parser calls [`admit`],
/// which joins the lines of a paragraph rather than refusing them where it can.
pub fn check(text: &str) -> Result<(), DjotRefusal> {
    scan(text, crate::djot_inline::Limits::default()).map(|_| ())
}

/// `text` as a load hands it on: as it is when [`check`] accepts it, or, when what it
/// passes is [`MAX_HELD_LINES`], with the lines of each paragraph past that ceiling
/// joined into one. The refusal when neither can be given to the parser.
///
/// A paragraph's line breaks are written as spaces, the next line's container markers
/// and indentation going with them. The parser reads such a break as a soft break and
/// the editor reads a soft break as a space, so the joined paragraph is the one the
/// editor would have opened, formatting and all. Only when that still leaves a paragraph
/// past the ceiling are the breaks inside its code spans joined too, which the parser
/// keeps as line breaks: those lines then run on, their words all kept. A join that would
/// change what any line of the text is read as (a paragraph turning into a table row,
/// say) is not made. See `djot_inline` for which breaks are joined.
pub fn admit(text: String) -> Result<String, DjotRefusal> {
    let held = match check(&text) {
        Ok(()) => return Ok(text),
        Err(refused @ DjotRefusal::HeldOpenTooLong { .. }) => refused,
        Err(refused) => return Err(refused),
    };
    if let Some(joined) = joined(&text, MAX_HELD_LINES + 1) {
        return Ok(joined);
    }
    // Name anything else the text passes, which the scan stopped short of.
    let unheld = crate::djot_inline::Limits {
        max_held: usize::MAX,
        ..crate::djot_inline::Limits::default()
    };
    match scan(&text, unheld) {
        Err(other) => Err(other),
        Ok(_) => Err(held),
    }
}

/// `text` with the lines of every paragraph holding at least `from` lines in a row
/// joined, when that brings it within [`check`] and leaves every line read as the same
/// kind of block: the breaks the parser reads as spaces, and only if that is not enough,
/// those in code spans too. `None` when no join does.
pub(crate) fn joined(text: &str, from: usize) -> Option<String> {
    let joins = joins_in(text, from)?;
    let spaces: Vec<Join> = joins.iter().filter(|join| !join.in_code).copied().collect();
    let mut tries = vec![spaces];
    if joins.iter().any(|join| join.in_code) {
        tries.push(joins);
    }
    tries
        .into_iter()
        .filter(|chosen| !chosen.is_empty())
        .filter_map(|chosen| join_lines(text, &chosen))
        .find(|candidate| check(candidate).is_ok())
}

/// The line breaks [`joined`] may join in `text`: those of every paragraph holding at
/// least `from` lines in a row, in the order of the text. `None` when the text passes a
/// limit other than the lines it holds.
pub(crate) fn joins_in(text: &str, from: usize) -> Option<Vec<Join>> {
    let unheld = crate::djot_inline::Limits {
        max_held: usize::MAX,
        ..crate::djot_inline::Limits::default()
    };
    scan_gathering(text, unheld, Some(from))
        .ok()
        .map(|(_, joins)| joins)
}

/// `text` with each of `joins` made: the bytes from its line break to where the next
/// line's words start replaced by one space. `None` when a join is not at a line break
/// followed by the next line, or when the joined text reads any line as another kind of
/// block than `text` did.
pub(crate) fn join_lines(text: &str, joins: &[Join]) -> Option<String> {
    let mut out = String::with_capacity(text.len());
    let mut from = 0usize;
    for join in joins {
        if join.at < from || text.as_bytes().get(join.at) != Some(&b'\n') {
            return None;
        }
        if text.get(join.at + 1..join.resume)?.contains('\n') {
            return None;
        }
        out.push_str(text.get(from..join.at)?);
        out.push(' ');
        from = join.resume;
    }
    out.push_str(text.get(from..)?);
    same_blocks(text, &out, joins).then_some(out)
}

/// Whether `joined`, `text` with `joins` made, reads every line it kept as `text` read
/// it: in as many containers, opening the same heading, and holding words of the same
/// kind from the same place. The lines the joins took into the one before are the only
/// ones missing.
fn same_blocks(text: &str, joined: &str, joins: &[Join]) -> bool {
    let starts: Vec<usize> = parsed_lines(text).map(|(_, range)| range.start).collect();
    let taken: std::collections::BTreeSet<usize> = joins
        .iter()
        .map(|join| {
            starts
                .partition_point(|&start| start <= join.resume)
                .saturating_sub(1)
        })
        .collect();
    let mut before = crate::djot_nesting::Nesting::default();
    let kept = parsed_lines(text)
        .map(|(_, range)| before.read_line(text.get(range).unwrap_or_default(), MAX_DEPTH))
        .enumerate()
        .filter(|(index, _)| !taken.contains(index))
        .map(|(_, read)| read);
    let mut after = crate::djot_nesting::Nesting::default();
    let reread = parsed_lines(joined)
        .map(|(_, range)| after.read_line(joined.get(range).unwrap_or_default(), MAX_DEPTH));
    kept.eq(reread)
}

/// Each line of `text` as the parser reads it, its line break included, with its
/// 1-based number. The empty piece after a final line break is no line to the parser.
fn parsed_lines(text: &str) -> impl Iterator<Item = (usize, std::ops::Range<usize>)> + '_ {
    let mut offset = 0usize;
    text.split('\n')
        .enumerate()
        .filter_map(move |(index, line)| {
            let start = offset;
            let end = (offset + line.len() + 1).min(text.len());
            offset += line.len() + 1;
            (start < end).then_some((index + 1, start..end))
        })
}

/// [`check`], holding the inline pass to `limits`, and returning what it cost.
pub(crate) fn scan(
    text: &str,
    limits: crate::djot_inline::Limits,
) -> Result<InlineCost, DjotRefusal> {
    scan_gathering(text, limits, None).map(|(cost, _)| cost)
}

/// [`scan`], also gathering the line breaks a load may join in the paragraphs holding at
/// least `join_from` lines in a row, when asked.
fn scan_gathering(
    text: &str,
    limits: crate::djot_inline::Limits,
    join_from: Option<usize>,
) -> Result<(InlineCost, Vec<Join>), DjotRefusal> {
    let mut nesting = crate::djot_nesting::Nesting::default();
    let mut inline = crate::djot_inline::InlineWork::with_limits(text, limits);
    if let Some(from) = join_from {
        inline = inline.gathering_joins(from);
    }
    for (number, range) in parsed_lines(text) {
        let (start, end) = (range.start, range.end);
        let parsed_line = text.get(range).unwrap_or_default();
        let read = nesting.read_line(parsed_line, MAX_DEPTH);
        if read.depth > MAX_DEPTH {
            return Err(DjotRefusal::TooDeep(TooDeep {
                depth: read.depth,
                line: number,
            }));
        }
        if let Some(level) = read.heading
            && level > MAX_HEADING_LEVEL
        {
            return Err(DjotRefusal::HeadingTooDeep {
                level,
                line: number,
            });
        }
        if read.sections > MAX_SECTIONS {
            return Err(DjotRefusal::TooManySections {
                sections: read.sections,
                line: number,
            });
        }
        inline
            .line(read.inline, start..end, number)
            .map_err(refused_inline)?;
    }
    inline.finish().map_err(refused_inline)?;
    Ok((
        InlineCost {
            steps: inline.spent(),
            held_lines: inline.most_held(),
        },
        inline.joins().to_vec(),
    ))
}

/// What the inline pass costs a text [`scan`] accepts: the steps it charged, and the
/// most lines one block held in a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct InlineCost {
    pub(crate) steps: u64,
    pub(crate) held_lines: usize,
}

/// The refusal for the inline ceiling a text passed.
fn refused_inline(exceeded: crate::djot_inline::Exceeded) -> DjotRefusal {
    match exceeded {
        crate::djot_inline::Exceeded::Steps { line } => DjotRefusal::TooSlowToRead { line },
        crate::djot_inline::Exceeded::HeldLines { line } => DjotRefusal::HeldOpenTooLong { line },
    }
}

/// [`admit`] the body of every comment and reply in a list of comment threads, in
/// place, or refuse the list, naming the body the Djot parser cannot be given by its
/// place in the list.
///
/// One file at a time: a `.comments.ron` sidecar or `orphan_comments.ron`, whose name
/// the caller adds. Positions count from one, as a writer counts, and the refusal's
/// own line is the line inside that body.
pub(crate) fn admit_comments(threads: &mut [CommentFile]) -> anyhow::Result<()> {
    for (index, comment) in threads.iter_mut().enumerate() {
        comment.body = admit(std::mem::take(&mut comment.body))
            .with_context(|| format!("comment {}", index + 1))?;
        for (turn, reply) in comment.replies.iter_mut().enumerate() {
            reply.body = admit(std::mem::take(&mut reply.body))
                .with_context(|| format!("reply {} to comment {}", turn + 1, index + 1))?;
        }
    }
    Ok(())
}

/// [`admit`] the body of every footnote in a list, in place, or refuse the list,
/// naming the note the Djot parser cannot be given by its place in the list.
///
/// By place and not by label: the label is the file's own string, of any length,
/// and the refusal is what the writer reads.
pub(crate) fn admit_footnotes(notes: &mut [FootnoteFile]) -> anyhow::Result<()> {
    for (index, note) in notes.iter_mut().enumerate() {
        note.body = admit(std::mem::take(&mut note.body))
            .with_context(|| format!("footnote {}", index + 1))?;
    }
    Ok(())
}

// Limits inside `jotdown` would be strictly better than this: they would bound the
// recursion and the work themselves rather than a second reading of the input made
// beside the parser, and they would cover every caller rather than the ones that
// remember to ask. That fix belongs
// in `jotdown` (or in `text-document`, which wraps it) and is not this crate's
// to make. This module is the boundary guard that does not require it.

/// `pub(crate)` for the hostile prose it builds, which every reader of stored Djot is
/// tested against.
#[cfg(test)]
#[path = "djot_depth_tests.rs"]
pub(crate) mod tests;
