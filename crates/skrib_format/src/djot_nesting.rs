// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The nesting `jotdown` 0.10 builds, counted exactly and without recursing.
//!
//! [`djot_depth::check`](crate::djot_depth::check) refuses prose on this count, and on
//! nothing else about its nesting. A count of the markers each line opens with, read
//! line by line, cannot do it: it cannot see a container a line continues without
//! restating it, since a list item goes on through a paragraph line at any
//! indentation, after a line that was not blank,
//!
//! ```text
//! - item
//! a paragraph line, which the item continues
//!
//!  - an item nested in the first
//! a paragraph line, which both continue
//! ```
//!
//! (seven hundred such steps, a line of one space more each, once passed such a count
//! at depth 1 and ended the process the first time the row was parsed), and it counts
//! markers on lines that open nothing, a code block's line of `>` among them, which
//! locked writers out of projects the parser reads without nesting anything.
//!
//! The scan also reports what else of each line the other limits need: the heading it
//! opens, the sections open after it, and the words it holds for the inline pass
//! (`djot_inline`). And it closes, for the compiler, what a text leaves open at its end
//! ([`closing_fences`]).
//!
//! # How it counts
//!
//! The way `jotdown`'s block pass reads a document, one line at a time. The containers
//! open at the current line are kept as a stack, outermost first, each with the state
//! `jotdown` keeps for it (`Kind` in its `block.rs`), and for each new line:
//!
//! 1. each open container, outermost first, is asked whether it continues on this line
//!    by `jotdown`'s own rule (`Kind::continues`), and hands the next one in the line as
//!    it strips it (`parse_container`: a blockquote strips its `> `, a list item or a
//!    footnote at most its marker's width of indentation, a div at most its fence's
//!    indentation). The first that does not continue closes, and all it held with it;
//! 2. if every container continued, the block last opened inside the innermost one (a
//!    paragraph, a heading, a code block, a table) is asked whether the line is more of
//!    it;
//! 3. otherwise the line is read as a new block (`IdentifiedBlock::new`), and while that
//!    block is a container, what follows its marker on the same line is read as the
//!    first block inside it.
//!
//! A table counts as one container, as `jotdown`'s events nest it. Whitespace is ASCII
//! whitespace, the only kind the parser reads as indentation.
//!
//! The same scan guards `text-document`'s own parse (its `djot_depth` module), where its
//! count is held equal to `jotdown`'s on generated documents of every shape the block
//! grammar has. It is repeated here rather than called because this crate reaches
//! `text-document` only through its public interface, which does not expose it.

/// Whitespace as the block parser reads it.
fn is_space(byte: u8) -> bool {
    byte.is_ascii_whitespace()
}

/// Whether `view` holds nothing but whitespace.
fn is_blank(view: &[u8]) -> bool {
    view.iter().all(|&byte| is_space(byte))
}

/// `view` without its whitespace at either end, and how much whitespace led it:
/// `str::trim_matches`, as `jotdown` measures a container's prefix with it. A line of
/// nothing but whitespace trims to nothing, led by none.
fn trimmed(view: &[u8]) -> (usize, &[u8]) {
    let Some(start) = view.iter().position(|&byte| !is_space(byte)) else {
        return (0, &[]);
    };
    let end = view.len()
        - view
            .iter()
            .rev()
            .take_while(|&&byte| is_space(byte))
            .count();
    (start, &view[start..end])
}

/// What a line starts, as `jotdown` 0.10's `IdentifiedBlock::new` identifies it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    /// Nothing but whitespace.
    Blank,
    /// A paragraph: anything no other kind claims.
    Paragraph,
    /// A heading, at its level.
    Heading(usize),
    /// A block attribute line or a thematic break, complete on its one line.
    Atom,
    /// A blockquote: `>`, then whitespace or the end of the line.
    Blockquote,
    /// A list item of any kind, a definition's `:` included, or a footnote definition:
    /// the containers continued by indentation, which `jotdown` continues by the same
    /// rule. `indent` is the whitespace in front of the marker.
    Item { indent: usize },
    /// A link definition, `[label]: destination`, which holds no blocks.
    LinkDefinition,
    /// A table row, `| … |`.
    TableRow,
    /// A fence of three or more of `mark`: a div's colons, or a code block's backticks
    /// or tildes. `bare` when nothing follows the marks, which is the only fence that
    /// closes a block. `indent` is the whitespace in front of it.
    Fence {
        mark: u8,
        len: usize,
        bare: bool,
        indent: usize,
    },
}

/// Identify the block `view` starts, and where its marker ends: the bytes a container's
/// marker takes from the line, after which its content begins on the same line.
fn identify(view: &[u8]) -> (Block, usize) {
    let indent = view
        .iter()
        .take_while(|&&byte| is_space(byte) && byte != b'\n')
        .count();
    let line = &view[indent..];
    let content = line.len()
        - line
            .iter()
            .rev()
            .take_while(|&&byte| is_space(byte))
            .count();
    let line_t = &line[..content];
    let Some(&first) = line.first() else {
        return (Block::Blank, indent);
    };
    let ends_marker = |at: usize| line.get(at).is_none_or(|&byte| is_space(byte));

    let found = match first {
        b'\n' => Some((Block::Blank, indent + 1)),
        b'#' => {
            let level = line.iter().take_while(|&&byte| byte == b'#').count();
            ends_marker(level).then_some((Block::Heading(level), indent + level))
        }
        b'>' => ends_marker(1).then_some((Block::Blockquote, indent + 1)),
        b'{' => (attributes_len(line) == content).then_some((Block::Atom, indent + line.len())),
        b'|' => (content >= 2 && line_t.ends_with(b"|") && !line_t.ends_with(b"\\|"))
            .then_some((Block::TableRow, indent)),
        b'[' => definition(line).map(|(label, footnote)| {
            let end = indent + 3 + label;
            if footnote {
                (Block::Item { indent }, end)
            } else {
                (Block::LinkDefinition, end)
            }
        }),
        b'-' | b'*' if is_thematic_break(&line[1..]) => Some((Block::Atom, indent + content)),
        b'-' | b'*' | b'+' => line.get(1).is_none_or(|&byte| byte == b' ').then(|| {
            // A task item's box belongs to its marker: `- [ ]` is five bytes wide.
            let task = line.get(2) == Some(&b'[')
                && matches!(line.get(3), Some(b'x' | b'X' | b' '))
                && line.get(4) == Some(&b']')
                && ends_marker(5);
            (Block::Item { indent }, indent + if task { 5 } else { 1 })
        }),
        b':' if ends_marker(1) => Some((Block::Item { indent }, indent + 1)),
        b'`' | b':' | b'~' => fence(line_t, first).map(|(len, bare)| {
            (
                Block::Fence {
                    mark: first,
                    len,
                    bare,
                    indent,
                },
                indent + line.len(),
            )
        }),
        _ => ordered_marker(line).map(|len| (Block::Item { indent }, indent + len)),
    };
    found.unwrap_or((Block::Paragraph, indent))
}

/// The state of `jotdown` 0.10's attribute validator (`attr::State`), one byte at a
/// time: a block attribute line here, and an inline attribute set in `djot_inline`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AttrState {
    Start,
    Whitespace,
    CommentFirst,
    Comment,
    CommentNewline,
    ClassFirst,
    Class,
    IdentifierFirst,
    Identifier,
    Key,
    ValueFirst,
    Value,
    ValueQuoted,
    ValueEscape,
    ValueNewline,
    ValueContinued,
    Done,
    Invalid,
}

impl AttrState {
    /// The state after `c`, as `attr::State::step` takes it. `Done` and `Invalid` are
    /// never stepped from: every caller stops on either.
    pub(crate) fn step(self, c: u8) -> AttrState {
        use AttrState::*;
        fn is_name(byte: u8) -> bool {
            byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'_' | b'-')
        }
        match self {
            Start if c == b'{' => Whitespace,
            Start => Invalid,
            Whitespace => match c {
                b'}' => Done,
                b'.' => ClassFirst,
                b'#' => IdentifierFirst,
                b'%' => CommentFirst,
                c if is_name(c) => Key,
                c if c.is_ascii_whitespace() => Whitespace,
                _ => Invalid,
            },
            CommentFirst | Comment | CommentNewline if c == b'%' => Whitespace,
            CommentFirst | Comment | CommentNewline if c == b'}' => Done,
            CommentFirst | Comment | CommentNewline if c == b'\n' => CommentNewline,
            CommentFirst | Comment | CommentNewline => Comment,
            ClassFirst if is_name(c) => Class,
            ClassFirst => Invalid,
            IdentifierFirst if is_name(c) => Identifier,
            IdentifierFirst => Invalid,
            s @ (Class | Identifier | Value) if is_name(c) => s,
            Class | Identifier | Value if c.is_ascii_whitespace() => Whitespace,
            Class | Identifier | Value if c == b'}' => Done,
            Class | Identifier | Value => Invalid,
            Key if is_name(c) => Key,
            Key if c == b'=' => ValueFirst,
            Key => Invalid,
            ValueFirst if is_name(c) => Value,
            ValueFirst if c == b'"' => ValueQuoted,
            ValueFirst => Invalid,
            ValueQuoted | ValueNewline | ValueContinued if c == b'"' => Whitespace,
            ValueQuoted | ValueNewline | ValueContinued | ValueEscape if c == b'\n' => ValueNewline,
            ValueQuoted if c == b'\\' => ValueEscape,
            ValueQuoted | ValueEscape => ValueQuoted,
            ValueNewline | ValueContinued => ValueContinued,
            Done | Invalid => self,
        }
    }
}

/// The length of the attribute block `line` opens with (the line from its `{` to its
/// end, line break included), or 0 if it does not open with a complete one: `jotdown`
/// 0.10's `attr::valid`. A line is an attribute block only when this reaches exactly the
/// end of its content.
fn attributes_len(line: &[u8]) -> usize {
    let mut state = AttrState::Start;
    for (at, &byte) in line.iter().enumerate() {
        state = state.step(byte);
        match state {
            AttrState::Done => return at + 1,
            AttrState::Invalid => return 0,
            _ => {}
        }
    }
    0
}

/// A footnote or link definition opening `line` at its `[`: the byte length of its
/// label (a footnote's `^` included), and whether it is a footnote. The label runs to
/// the first `]`, which a `:` must follow.
fn definition(line: &[u8]) -> Option<(usize, bool)> {
    let rest = line.get(1..)?;
    let label = rest.iter().position(|&byte| byte == b']')?;
    (rest.get(label + 1) == Some(&b':')).then_some((label, rest.first() == Some(&b'^')))
}

/// Whether what follows a `-` or `*` makes a thematic break: at least two more of
/// either mark, and nothing but whitespace besides.
fn is_thematic_break(after: &[u8]) -> bool {
    let mut marks = 1usize;
    for &byte in after {
        if matches!(byte, b'-' | b'*') {
            marks += 1;
        } else if !is_space(byte) {
            return false;
        }
    }
    marks >= 3
}

/// The fence `line` (trimmed of its trailing whitespace) opens with `mark`: its length,
/// and whether nothing follows it. A div's class holds only name characters; a code
/// block's language no whitespace and no backtick.
fn fence(line: &[u8], mark: u8) -> Option<(usize, bool)> {
    let len = line.iter().take_while(|&&byte| byte == mark).count();
    let spec = &line[len..];
    let spec = &spec[spec.iter().take_while(|&&byte| is_space(byte)).count()..];
    let valid = if mark == b':' {
        spec.iter()
            .all(|&byte| byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'_' | b'-'))
    } else {
        !spec.iter().any(|&byte| is_space(byte) || byte == b'`')
    };
    (valid && len >= 3).then_some((len, spec.is_empty()))
}

/// The length of the ordered list marker opening `line`, or `None`: `jotdown` 0.10's
/// `maybe_ordered_list_item`. An optional `(`, then up to 19 digits, up to 13 roman
/// numerals all of one case, or one letter, then `)` (always, after a `(`) or `.`, then
/// whitespace or the end of the line.
fn ordered_marker(line: &[u8]) -> Option<usize> {
    fn roman_lower(byte: u8) -> bool {
        matches!(byte, b'i' | b'v' | b'x' | b'l' | b'c' | b'd' | b'm')
    }
    fn roman_upper(byte: u8) -> bool {
        matches!(byte, b'I' | b'V' | b'X' | b'L' | b'C' | b'D' | b'M')
    }
    let paren = line.first() == Some(&b'(');
    let start = usize::from(paren);
    let first = *line.get(start)?;
    let (numeral, most): (fn(u8) -> bool, usize) = if first.is_ascii_digit() {
        (|byte| byte.is_ascii_digit(), 19)
    } else if roman_lower(first) {
        (roman_lower, 13)
    } else if roman_upper(first) {
        (roman_upper, 13)
    } else if first.is_ascii_lowercase() {
        (|byte| byte.is_ascii_lowercase(), 1)
    } else if first.is_ascii_uppercase() {
        (|byte| byte.is_ascii_uppercase(), 1)
    } else {
        return None;
    };
    let number = 1 + line
        .get(start + 1..)
        .unwrap_or_default()
        .iter()
        .take(most - 1)
        .take_while(|&&byte| numeral(byte))
        .count();
    let closes = match line.get(start + number) {
        Some(b')') => true,
        Some(b'.') => !paren,
        _ => false,
    };
    let len = start + number + 1;
    (closes && line.get(len).is_none_or(|&byte| is_space(byte))).then_some(len)
}

/// A container open at the line being read, with the state `jotdown` keeps for it.
#[derive(Debug)]
enum Frame {
    /// A blockquote.
    Quote,
    /// A list item or a footnote definition. `marker_end` is how many bytes its marker
    /// took from its first line, the most indentation it strips from each later one.
    Item {
        indent: usize,
        marker_end: usize,
        last_blank: bool,
    },
    /// A div. `raw` is a code fence it saw open and not yet closed (`jotdown`'s
    /// `nested_raw`): while one is open, no fence closes the div. `first` until its
    /// first line of content, which `jotdown` does not strip.
    Div {
        indent: usize,
        len: usize,
        raw: Option<(u8, usize)>,
        closed: bool,
        first: bool,
    },
}

impl Frame {
    /// Whether this container goes on to `view`, the line as the containers around it
    /// leave it: `jotdown`'s `Kind::continues`.
    fn continues(&mut self, view: &[u8]) -> bool {
        match self {
            // A blockquote goes on through a line that states it again, and lazily
            // through a paragraph's line.
            Frame::Quote => matches!(identify(view).0, Block::Blockquote | Block::Paragraph),
            // An item goes on through a blank line, a line indented past its marker,
            // and lazily through a paragraph's line after a line that was not blank.
            Frame::Item {
                indent, last_blank, ..
            } => {
                let whitespace = view.iter().take_while(|&&byte| is_space(byte)).count();
                let next = identify(view).0;
                let lazy = !*last_blank && next == Block::Paragraph;
                *last_blank = next == Block::Blank;
                *last_blank || whitespace > *indent || lazy
            }
            // A div goes on until a bare fence of its own kind at least as long as its
            // own, which is the last line it holds.
            Frame::Div {
                len, raw, closed, ..
            } => {
                if *closed {
                    return false;
                }
                if let Block::Fence {
                    mark,
                    len: fence_len,
                    bare,
                    ..
                } = identify(view).0
                {
                    match *raw {
                        Some((open, open_len)) => {
                            if mark == open && fence_len >= open_len && bare {
                                *raw = None;
                            }
                        }
                        None if mark == b':' => *closed = fence_len >= *len && bare,
                        None => *raw = Some((mark, fence_len)),
                    }
                }
                true
            }
        }
    }

    /// `view` as this container hands it to what it holds, on any line after its first:
    /// `jotdown`'s `parse_container`, which never strips the line break.
    fn strip<'a>(&mut self, view: &'a [u8]) -> &'a [u8] {
        let body = view.iter().take_while(|&&byte| byte != b'\n').count();
        let (whitespace, content) = trimmed(view);
        let skip = match self {
            Frame::Quote => {
                if content == b">" {
                    whitespace + 1
                } else if content.first() == Some(&b'>')
                    && content.get(1).is_some_and(|&byte| is_space(byte))
                {
                    whitespace + 2
                } else {
                    0
                }
            }
            Frame::Item { marker_end, .. } => whitespace.min(*marker_end),
            Frame::Div { indent, first, .. } => {
                if std::mem::take(first) {
                    0
                } else {
                    whitespace.min(*indent)
                }
            }
        };
        &view[skip.min(body)..]
    }
}

/// The block last opened inside the innermost open container, when it is not itself a
/// container: what a line that every container goes on to may simply be more of.
#[derive(Debug, Default)]
enum Leaf {
    /// None, or one that ends on its own line (a blank line, an attribute line, a
    /// thematic break): the next line starts a block.
    #[default]
    None,
    Paragraph,
    Heading(usize),
    LinkDefinition,
    Code {
        mark: u8,
        len: usize,
        closed: bool,
    },
    /// A table, which counts as a container.
    Table {
        caption: bool,
        blank: bool,
    },
}

impl Leaf {
    /// Whether `view` is more of this block: `jotdown`'s `Kind::continues`.
    fn continues(&mut self, view: &[u8]) -> bool {
        match self {
            Leaf::None => false,
            Leaf::Paragraph | Leaf::Table { caption: true, .. } => !is_blank(view),
            Leaf::Heading(level) => match identify(view).0 {
                Block::Paragraph => true,
                Block::Heading(next) => next == *level,
                _ => false,
            },
            Leaf::LinkDefinition => view.first() == Some(&b' ') && !is_blank(view),
            Leaf::Code { mark, len, closed } => {
                if *closed {
                    return false;
                }
                if let Block::Fence {
                    mark: fence_mark,
                    len: fence_len,
                    bare,
                    ..
                } = identify(view).0
                    && fence_mark == *mark
                {
                    *closed = fence_len >= *len && bare;
                }
                true
            }
            Leaf::Table { caption, blank } => {
                let (_, row) = trimmed(view);
                if row.is_empty() {
                    *blank = true;
                    true
                } else if row.starts_with(b"^ ") {
                    *caption = true;
                    true
                } else {
                    !*blank
                        && row.starts_with(b"|")
                        && row.ends_with(b"|")
                        && !row.ends_with(b"\\|")
                }
            }
        }
    }
}

/// The kind of block whose words a line holds, as `jotdown` hands them to its inline
/// parser: the leaves whose text is inline Djot. A code block and a link definition
/// hold raw text, and every other block holds none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InlineKind {
    /// A paragraph, a definition's term among them.
    Paragraph,
    /// A heading, which `jotdown` reads twice: once to name it, once to show it.
    Heading,
    /// One row of a table. `jotdown` reads each of its cells on its own, and so does
    /// `djot_inline`, which splits the row the way `jotdown` does.
    TableRow,
    /// A table's caption, from its `^ ` to the end of the table.
    Caption,
}

/// What one line holds for the inline parser: nothing, the first line of a new block
/// of words, or more of the one before. `from` is the byte offset in the line where
/// the words start, past every container marker and the block's own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Inline {
    #[default]
    None,
    Starts {
        kind: InlineKind,
        from: usize,
    },
    Continues {
        kind: InlineKind,
        from: usize,
    },
}

/// What [`Nesting::read_line`] learnt of one line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LineRead {
    /// How many containers the line sits in.
    pub(crate) depth: usize,
    /// The level of the heading the line opens, if it opens one: the count of `#`
    /// `jotdown` stores in 16 bits.
    pub(crate) heading: Option<usize>,
    /// How many sections are open after the line: each top-level heading opens one,
    /// inside every open section of a lower level, and `jotdown` counts them among
    /// the containers a list's depth is stored in, in 16 bits as well.
    pub(crate) sections: usize,
    /// The words the line holds for the inline parser.
    pub(crate) inline: Inline,
}

/// The containers open at the line being read, outermost first, and the block last
/// opened inside the innermost of them: the state of one scan of one text.
#[derive(Debug, Default)]
pub(crate) struct Nesting {
    frames: Vec<Frame>,
    leaf: Leaf,
    /// The levels of the open sections, lowest first: `jotdown`'s `open_sections`, a
    /// heading closing every section of its own level or deeper and opening its own.
    sections: Vec<usize>,
    /// The heading the line being read opens, noted as it is read.
    heading: Option<usize>,
    /// The words the line being read holds, noted as it is read.
    inline: Inline,
}

/// Where `view`, a part of `line`, starts in it.
fn offset_in(line: &[u8], view: &[u8]) -> usize {
    (view.as_ptr() as usize).saturating_sub(line.as_ptr() as usize)
}

impl Nesting {
    /// Read the next line of the text, its line break included, and return how many
    /// containers it sits in. Opens no more than `limit + 1` containers on it, so a line
    /// past `limit` is only known to be past it.
    #[cfg(test)]
    pub(crate) fn read(&mut self, line: &str, limit: usize) -> usize {
        self.read_line(line, limit).depth
    }

    /// Read the next line of the text, its line break included, and report what it
    /// holds: see [`LineRead`]. Opens no more than `limit + 1` containers on it, so a
    /// line past `limit` is only known to be past it.
    pub(crate) fn read_line(&mut self, line: &str, limit: usize) -> LineRead {
        self.heading = None;
        self.inline = Inline::None;
        let depth = self.scan(line.as_bytes(), limit);
        LineRead {
            depth,
            heading: self.heading,
            sections: self.sections.len(),
            inline: self.inline,
        }
    }

    fn scan(&mut self, line: &[u8], limit: usize) -> usize {
        let mut view = line;
        let mut level = 0;
        while let Some(frame) = self.frames.get_mut(level) {
            if !frame.continues(view) {
                // It ends before this line, and all it held with it.
                self.frames.truncate(level);
                return self.open(line, view, limit);
            }
            if matches!(frame, Frame::Div { closed: true, .. }) {
                // A div's closing fence belongs to it and to nothing inside it.
                self.frames.truncate(level + 1);
                self.leaf = Leaf::None;
                return self.depth();
            }
            view = frame.strip(view);
            level += 1;
        }
        let caption_before = matches!(self.leaf, Leaf::Table { caption: true, .. });
        if self.leaf.continues(view) {
            self.note_continuation(line, view, caption_before);
            return self.depth();
        }
        self.open(line, view, limit)
    }

    /// Note the words a line holds that the block before goes on through: `view` is the
    /// line as the containers around that block leave it.
    fn note_continuation(&mut self, line: &[u8], view: &[u8], caption_before: bool) {
        let from = offset_in(line, view);
        self.inline = match self.leaf {
            Leaf::Paragraph => Inline::Continues {
                kind: InlineKind::Paragraph,
                from,
            },
            Leaf::Heading(_) => {
                // A heading's later line may restate its `#`, which is markup there
                // and not words (`jotdown`'s `parse_block` strips it).
                let (block, marker_end) = identify(view);
                let from = if matches!(block, Block::Heading(_)) {
                    from + marker_end
                } else {
                    from
                };
                Inline::Continues {
                    kind: InlineKind::Heading,
                    from,
                }
            }
            Leaf::Table { caption: true, .. } if caption_before => Inline::Continues {
                kind: InlineKind::Caption,
                from,
            },
            Leaf::Table { caption: true, .. } => {
                // The line that starts the caption: its words follow the `^ `.
                let (whitespace, _) = trimmed(view);
                Inline::Starts {
                    kind: InlineKind::Caption,
                    from: from + whitespace + 2,
                }
            }
            Leaf::Table { .. } if !is_blank(view) => Inline::Starts {
                kind: InlineKind::TableRow,
                from,
            },
            _ => Inline::None,
        };
    }

    /// Read `view` as the first line of a new block inside the innermost open container,
    /// opening every container its markers open.
    fn open(&mut self, line: &[u8], mut view: &[u8], limit: usize) -> usize {
        self.leaf = Leaf::None;
        while self.frames.len() <= limit {
            let (block, marker_end) = identify(view);
            match block {
                Block::Blank | Block::Atom => break,
                Block::Paragraph => {
                    self.leaf = Leaf::Paragraph;
                    self.inline = Inline::Starts {
                        kind: InlineKind::Paragraph,
                        from: offset_in(line, view),
                    };
                    break;
                }
                Block::Heading(level) => {
                    self.leaf = Leaf::Heading(level);
                    self.heading = Some(level);
                    // Only a heading outside every container opens a section.
                    if self.frames.is_empty() {
                        while self.sections.last().is_some_and(|&open| open >= level) {
                            self.sections.pop();
                        }
                        self.sections.push(level);
                    }
                    self.inline = Inline::Starts {
                        kind: InlineKind::Heading,
                        from: offset_in(line, view) + marker_end.min(view.len()),
                    };
                    break;
                }
                Block::LinkDefinition => {
                    self.leaf = Leaf::LinkDefinition;
                    break;
                }
                Block::TableRow => {
                    self.leaf = Leaf::Table {
                        caption: false,
                        blank: false,
                    };
                    self.inline = Inline::Starts {
                        kind: InlineKind::TableRow,
                        from: offset_in(line, view),
                    };
                    break;
                }
                Block::Fence {
                    mark: b':',
                    len,
                    indent,
                    ..
                } => {
                    // A div's content starts on the line after its fence.
                    self.frames.push(Frame::Div {
                        indent,
                        len,
                        raw: None,
                        closed: false,
                        first: true,
                    });
                    break;
                }
                Block::Fence { mark, len, .. } => {
                    self.leaf = Leaf::Code {
                        mark,
                        len,
                        closed: false,
                    };
                    break;
                }
                Block::Blockquote => {
                    self.frames.push(Frame::Quote);
                    view = &view[marker_end..];
                    // The one space or tab after a `>` is the quote's own.
                    if matches!(view.first(), Some(b' ' | b'\t')) {
                        view = &view[1..];
                    }
                }
                Block::Item { indent } => {
                    self.frames.push(Frame::Item {
                        indent,
                        marker_end,
                        last_blank: false,
                    });
                    view = &view[marker_end..];
                }
            }
        }
        self.depth()
    }

    /// How many containers the line just read sits in.
    fn depth(&self) -> usize {
        self.frames.len() + usize::from(matches!(self.leaf, Leaf::Table { .. }))
    }

    /// The fences that close what the text read so far leaves open at its end, for
    /// text that goes on after it: see [`closing_fences`].
    fn closers(&self) -> Option<String> {
        // Only the divs outside every other container outlive the text: a quotation
        // ends at the blank line after it, and a list item or a footnote at the first
        // line after a blank one that is not indented into it.
        let outer = self
            .frames
            .iter()
            .take_while(|frame| matches!(frame, Frame::Div { closed: false, .. }))
            .count();
        let open_code =
            self.frames.len() == outer && matches!(self.leaf, Leaf::Code { closed: false, .. });
        if outer == 0 && !open_code {
            return None;
        }
        let innermost = if outer == 0 {
            None
        } else {
            self.frames.get(outer - 1)
        };
        Some(match (innermost, &self.leaf) {
            // The div's content ends with the div, a code block in it included, once
            // no code fence holds it open.
            (Some(Frame::Div { len, raw: None, .. }), _) => ":".repeat(*len),
            (
                Some(Frame::Div {
                    raw: Some((mark, len)),
                    ..
                }),
                _,
            ) => char::from(*mark).to_string().repeat(*len),
            (_, Leaf::Code { mark, len, .. }) => char::from(*mark).to_string().repeat(*len),
            _ => return None,
        })
    }
}

/// The lines that close every div and code block `text` leaves open at its end, for
/// a caller that writes `text`, then a blank line, then these, then more Djot: the
/// compiler, which puts one row's prose after another.
///
/// A div stays open until a bare fence closes it, and a code block until its own
/// closing fence, so without them the next row's text is read inside whatever the
/// last one left open, and a book of such rows nests one level deeper with each.
/// Nothing else outlives the blank line: a quotation ends at it, and a list item or a
/// footnote at the first line after it that is not indented into them, which each
/// fence is not. A code block left open keeps that blank line as one of its own.
///
/// Worked out with the scan itself, one fence at a time: a fence can clear a code
/// fence a div was holding open rather than close the div, and only reading it back
/// tells which. Each returned line ends in a line break, and a blank line follows the
/// last. Empty when the text leaves nothing open.
pub fn closing_fences(text: &str) -> String {
    let mut nesting = Nesting::default();
    for line in text.split_inclusive('\n') {
        nesting.read_line(line, usize::MAX);
    }
    // The blank line the caller writes after the text.
    nesting.read_line("\n", usize::MAX);
    let mut out = String::new();
    // Each fence closes a div or clears the code fence one holds open, and one more
    // closes a code block outside every div, so this many are always enough.
    for _ in 0..=2 * nesting.frames.len() {
        let Some(fence) = nesting.closers() else {
            break;
        };
        let line = format!("{fence}\n");
        nesting.read_line(&line, usize::MAX);
        out.push_str(&line);
    }
    if !out.is_empty() {
        out.push('\n');
    }
    out
}

#[cfg(test)]
#[path = "djot_nesting_tests.rs"]
mod tests;
