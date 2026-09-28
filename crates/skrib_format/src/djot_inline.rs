// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Two ceilings on `jotdown` 0.10's inline pass over a piece of Djot: on the work it
//! does, and on the lines it holds open.
//!
//! # The failures this prevents
//!
//! The inline parser keeps the openers it has met and not yet closed (a `[`, a `{+`, a
//! `*` or a quote followed by a word) on a stack, and for every token after them it
//! looks down that stack from the top for one the token closes (`Parser::
//! parse_container`'s `rposition`). An opener nothing closes stays there to the end of
//! its paragraph. So a paragraph of openers nobody closes costs the square of its
//! length: 320 KB of `[` took 54 seconds in a release build, on the thread that opens
//! the row, which is the one every window of the app draws with. Every kind of opener
//! costs the same way.
//!
//! The pass is also handed a block's words one line at a time, and while something it
//! has read is still open (an opener, a verbatim span, an attribute set it has not seen
//! the end of) it holds back what it has read and asks for the next line. The parser
//! hands that line over from inside the call that asked (`Parser::block` ends in
//! `return self.next_span()`), so each such line is one more call on the stack until the
//! pass lets go. One quotation mark nothing closes, followed by a few thousand more lines
//! of the same paragraph, is charged nothing, since the stack it looks down holds that
//! one entry, and it overflows the thread: about 4,000 lines on the 2 MiB stack of a
//! long operation's thread in a release build, 720 in a debug one. That is no panic but
//! an abort, and it takes every window of the process with it.
//!
//! [`djot_depth::check`](crate::djot_depth::check) finds such text, with the other
//! boundaries the parser cannot cross, and
//! [`djot_depth::admit`](crate::djot_depth::admit) refuses it as it is read from the
//! bundle, before anything parses it, unless joining a paragraph's lines brings it
//! within them (see below).
//!
//! # How it counts
//!
//! By following the pass token by token, as `jotdown` does: its lexer, ported here; its
//! verbatim spans, which hide everything in them; the attribute sets, autolinks,
//! symbols and footnote references it skips whole; and the openers, pushed and closed
//! by the parser's own rules, a bidirectional mark's spacing and the empty container it
//! refuses to close included. Each token that looks down the stack is charged the
//! entries it would look at, past the first
//! [`FREE_STEPS_PER_TOKEN`](crate::djot_inline::FREE_STEPS_PER_TOKEN), and a text whose
//! charges add up past [`MAX_EXCESS_STEPS`](crate::djot_inline::MAX_EXCESS_STEPS) is refused
//! at the line where they do.
//!
//! The paragraphs, headings, table cells and captions are the ones the exact nesting
//! scan finds (`djot_nesting`), read line by line as `jotdown` hands them to this pass:
//! after the containers' markers, each line trimmed at its start, the last at its end,
//! and each cell of a row on its own. A heading is charged twice, since `jotdown` reads
//! it once to name it and once to show it.
//!
//! The lines are counted at the same time, as deep as the parser's calls go. The pass
//! lets go of what it holds, and the calls return, as soon as it holds something and
//! nothing keeps it: no opener or verbatim span open, and a last event that is not a
//! string the next token could add to. That is at the end of most lines, and often in
//! the middle of one. A line handed over with nothing let go of since the line before
//! is one call deeper; one handed over after the pass let go starts again. An attribute
//! set read on across lines asks for each of them with nothing let go, and the lines it
//! read ahead are not handed over a second time if it turns out not to be one. A block
//! that goes more than [`MAX_HELD_LINES`](crate::djot_inline::MAX_HELD_LINES) lines deep
//! is refused at the line that passes it. A heading counts once here: the reading that
//! names it is a loop, and only the one that shows it recurses. A table's cells are one
//! line each.
//!
//! # Joining what the parser would read as one line
//!
//! A load does not refuse a paragraph for the lines it holds when those lines can be
//! joined into one: [`djot_depth::admit`](crate::djot_depth::admit) asks the pass for the
//! line breaks of each paragraph past the ceiling (`Join`) and writes each as a space,
//! the next line's markers and indentation with it. Between two words, `jotdown` reads
//! such a break as a soft break, and `text-document` reads a soft break as a space, so
//! the joined paragraph is the one the editor would have opened. A break is left where
//! it is read as something else: inside a link's destination or reference, which the
//! parser rebuilds from its lines without the break; after a footnote label the break
//! ended, which a space would not end; and before a line opening with an attribute set,
//! which the parser attaches to the word before a space. A break inside a code span is
//! kept by the parser, so joining it turns it into a space; that is done only when
//! joining every other break still leaves the paragraph past the ceiling.
//!
//! # Exact, because nothing less is safe
//!
//! The count is the parser's own, entry for entry: it was held equal to a copy of
//! `jotdown` 0.10 that adds up the entries `rposition` looks at, on two hundred thousand
//! generated documents of every inline and block shape, and the tests pin a set of
//! those counts. An approximation cannot promise to err one way. A mark read as not
//! closing what the parser closes stays open, and the next mark that would have opened
//! something closes it instead, leaving the stack shorter than the parser's from then
//! on: the first version of this scan, which counted fewer events than the parser to
//! take more containers for empty, came out lower than the parser's work that way.
//! The lines held were held equal the same way, to the deepest the parser's calls go in
//! a copy of it that counts them, on four hundred thousand generated documents of many
//! lines each, and the tests pin a set of those too.
//!
//! # What the editor writes is never refused
//!
//! The editor and every importer write their Djot through `text-document`'s escaper,
//! which since `text-document` 1.12.3 escapes each character that could open a container
//! in the words it writes. The only openers in that Djot are the markup they put there
//! themselves, and that is always closed: a link, a footnote reference, a run of bold or
//! of any other format. The stack then holds no more than the formats nested at one point
//! of the text, a handful, far below the
//! [`FREE_STEPS_PER_TOKEN`](crate::djot_inline::FREE_STEPS_PER_TOKEN) every token is
//! allowed, so nothing is charged at all. Prose saved before that version left its
//! straight quotation marks and apostrophes unescaped, and those can open too; but in
//! prose a quotation mark that opens is soon followed by the one that closes it, and an
//! apostrophe inside a word opens nothing, so the stack stays as short and that prose is
//! charged nothing either. What reaches the ceiling is a paragraph holding thousands of
//! openers nothing closes, followed by thousands of words, which only a crafted or
//! damaged file holds.
//!
//! They did not always write a paragraph on one line. Up to 1.12.2, `text-document` kept
//! the line breaks of a pasted preformatted passage (a `<pre>`, for one) inside the
//! paragraph's text, and wrote each as a line break of the Djot; from 1.12.3 it splits
//! the passage into one paragraph per line. Formatted across those lines, or opened by a
//! quotation mark nothing closes, such a paragraph in a project saved before holds every
//! line of it, and one of a few hundred lines reaches
//! [`MAX_HELD_LINES`](crate::djot_inline::MAX_HELD_LINES). A load joins those lines rather
//! than refusing them (see above), so that ceiling is only ever held against Djot written
//! some other way, by hand or in a crafted file.

use std::ops::Range;

use crate::djot_nesting::{AttrState, Inline, InlineKind};

/// How many entries of the opener stack each token may look at without being charged:
/// the formats nested at one point of a text the editor wrote are far fewer.
pub(crate) const FREE_STEPS_PER_TOKEN: u64 = 32;

/// The most stack entries a text's tokens may look at past their free ones, before
/// the text is refused. `jotdown` looks at one in well under a nanosecond in a
/// release build and a few in a debug one, so this is a few tenths of a second of
/// parsing at most.
pub(crate) const MAX_EXCESS_STEPS: u64 = 1 << 26;

/// The most lines in a row a block may hand the inline pass with nothing let go of in
/// between, each of them one call deeper on the parser's stack.
///
/// Measured through `text-document`'s `set_djot_sync`, one such line takes about half a
/// kilobyte of stack in a release build: 1,967 fit a 1 MiB stack (a Windows program's
/// main thread, the smallest a parse runs on) and 3,949 a 2 MiB one (a long operation's
/// thread). This is a quarter of the smallest, leaving the calls beneath the parse the
/// rest. A debug build takes about 3 KB a line: 717 fit a 2 MiB stack, which this also
/// leaves room under, but only 352 fit a 1 MiB one, so a debug build on Windows can still
/// run out of stack on its main thread before this ceiling is reached.
pub const MAX_HELD_LINES: usize = 512;

/// Which of the two ceilings a text passed, and at which 1-based line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Exceeded {
    /// The steps charged passed [`MAX_EXCESS_STEPS`].
    Steps { line: usize },
    /// A block went more than [`MAX_HELD_LINES`] lines deep into the parser's calls;
    /// `line` is the first past the ceiling.
    HeldLines { line: usize },
}

/// A line break inside a paragraph that the parser reads as a space of its words, and
/// a load may write as one: `at` is the break, and `resume` where the next line's words
/// start, past its container markers and indentation. Joining replaces the bytes from
/// `at` to `resume` with one space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Join {
    pub(crate) at: usize,
    pub(crate) resume: usize,
    /// The break is inside a code span, where the parser keeps it as a line break: a
    /// space in its place is a change the reader can see.
    pub(crate) in_code: bool,
}

/// A token, as `jotdown`'s `lex::Kind` names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Text,
    Newline,
    Nbsp,
    Hardbreak,
    Escape,
    Open(Delim),
    Close(Delim),
    Sym(Sym),
    Seq(Seq),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Delim {
    Brace,
    BraceAsterisk,
    BraceCaret,
    BraceEqual,
    BraceHyphen,
    BracePlus,
    BraceTilde,
    BraceUnderscore,
    Bracket,
    BraceQuote1,
    BraceQuote2,
    Paren,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sym {
    Asterisk,
    Caret,
    ExclaimBracket,
    Lt,
    Pipe,
    Quote1,
    Quote2,
    Tilde,
    Underscore,
    Colon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seq {
    Backtick,
    Hyphen,
    Period,
}

impl Seq {
    fn byte(self) -> u8 {
        match self {
            Seq::Backtick => b'`',
            Seq::Hyphen => b'-',
            Seq::Period => b'.',
        }
    }
}

/// One token: its kind, and where it lies in the text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Token {
    kind: Kind,
    start: usize,
    len: usize,
}

impl Token {
    fn end(self) -> usize {
        self.start + self.len
    }
}

/// `jotdown`'s `lex::is_special`: the bytes that end a run of plain text.
fn is_special(byte: u8) -> bool {
    matches!(
        byte,
        b'\\'
            | b'['
            | b']'
            | b'('
            | b')'
            | b'{'
            | b'}'
            | b'*'
            | b'^'
            | b'='
            | b'+'
            | b'~'
            | b'_'
            | b'\''
            | b'"'
            | b'-'
            | b'!'
            | b'<'
            | b'|'
            | b':'
            | b'`'
            | b'.'
            | b'\n'
    )
}

/// `jotdown`'s `lex::Lexer`, over one line of an inline block: the bytes of the text
/// from `pos` to `end`, with positions kept in the text's own terms.
#[derive(Debug, Clone)]
struct Lexer<'a> {
    src: &'a [u8],
    end: usize,
    pos: usize,
    /// The next byte is escaped.
    escape: bool,
    /// The token peeked and not yet taken.
    next: Option<Token>,
    /// Escapes are not read, inside a verbatim span opened on this line.
    verbatim: bool,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a [u8], start: usize, end: usize) -> Self {
        Self {
            src,
            end,
            pos: start.min(end),
            escape: false,
            next: None,
            verbatim: false,
        }
    }

    fn peek(&mut self) -> Option<Token> {
        if self.next.is_none() {
            self.next = self.token();
        }
        self.next
    }

    /// The rest of the line after the token last taken.
    fn ahead(&self) -> &'a [u8] {
        let at = self.pos - self.next.map_or(0, |token| token.len);
        self.src.get(at..self.end).unwrap_or_default()
    }

    /// Start again `n` bytes past everything lexed so far, as `skip_ahead` does.
    fn skip_ahead(&mut self, n: usize) {
        *self = Lexer::new(self.src, self.pos + n, self.end);
    }

    fn eat(&mut self) -> Option<Token> {
        self.next.take().or_else(|| self.next_token())
    }

    fn next_token(&mut self) -> Option<Token> {
        let mut current = self.token();
        if let Some(token) = &mut current
            && token.kind == Kind::Text
        {
            // Consecutive text is one token.
            self.next = self.token();
            while let Some(next) = self.next
                && next.kind == Kind::Text
            {
                token.len += next.len;
                self.next = self.token();
            }
        }
        current
    }

    fn peek_byte(&self, n: usize) -> Option<u8> {
        let at = self.pos + n;
        (at < self.end).then(|| self.src[at])
    }

    fn eat_byte(&mut self) -> Option<u8> {
        let byte = self.peek_byte(0)?;
        self.pos += 1;
        Some(byte)
    }

    fn eat_while(&mut self, mut keep: impl FnMut(u8) -> bool) {
        while let Some(byte) = self.peek_byte(0)
            && keep(byte)
        {
            self.pos += 1;
        }
    }

    fn close_or(&mut self, open: Kind, close: Delim) -> Kind {
        if self.peek_byte(0) == Some(b'}') {
            self.pos += 1;
            Kind::Close(close)
        } else {
            open
        }
    }

    fn token(&mut self) -> Option<Token> {
        let start = self.pos;
        let kind = if self.escape {
            self.escape = false;
            match self.eat_byte()? {
                b'\n' => Kind::Hardbreak,
                b'\t' | b' '
                    if self.src[self.pos..self.end]
                        .iter()
                        .find(|&&byte| !matches!(byte, b' ' | b'\t'))
                        == Some(&b'\n') =>
                {
                    while !matches!(self.eat_byte(), Some(b'\n') | None) {}
                    Kind::Hardbreak
                }
                b' ' => Kind::Nbsp,
                _ => Kind::Text,
            }
        } else {
            self.eat_while(|byte| !is_special(byte));
            if start < self.pos {
                Kind::Text
            } else {
                match self.eat_byte()? {
                    b'\n' => Kind::Newline,
                    b'\\' => {
                        if self.peek_byte(0).is_some_and(|byte| {
                            byte.is_ascii_whitespace() || byte.is_ascii_punctuation()
                        }) {
                            self.escape = !self.verbatim;
                            Kind::Escape
                        } else {
                            Kind::Text
                        }
                    }
                    b'[' => Kind::Open(Delim::Bracket),
                    b']' => Kind::Close(Delim::Bracket),
                    b'(' => Kind::Open(Delim::Paren),
                    b')' => Kind::Close(Delim::Paren),
                    b'{' => {
                        let explicit = match self.peek_byte(0) {
                            Some(b'*') => Some(Delim::BraceAsterisk),
                            Some(b'^') => Some(Delim::BraceCaret),
                            Some(b'=') => Some(Delim::BraceEqual),
                            Some(b'-') => Some(Delim::BraceHyphen),
                            Some(b'+') => Some(Delim::BracePlus),
                            Some(b'~') => Some(Delim::BraceTilde),
                            Some(b'_') => Some(Delim::BraceUnderscore),
                            Some(b'\'') => Some(Delim::BraceQuote1),
                            Some(b'"') => Some(Delim::BraceQuote2),
                            _ => None,
                        };
                        match explicit {
                            Some(delim) => {
                                self.pos += 1;
                                Kind::Open(delim)
                            }
                            None => Kind::Open(Delim::Brace),
                        }
                    }
                    b'}' => Kind::Close(Delim::Brace),
                    b'*' => self.close_or(Kind::Sym(Sym::Asterisk), Delim::BraceAsterisk),
                    b'^' => self.close_or(Kind::Sym(Sym::Caret), Delim::BraceCaret),
                    b'=' => self.close_or(Kind::Text, Delim::BraceEqual),
                    b'+' => self.close_or(Kind::Text, Delim::BracePlus),
                    b'~' => self.close_or(Kind::Sym(Sym::Tilde), Delim::BraceTilde),
                    b'_' => self.close_or(Kind::Sym(Sym::Underscore), Delim::BraceUnderscore),
                    b'\'' => self.close_or(Kind::Sym(Sym::Quote1), Delim::BraceQuote1),
                    b'"' => self.close_or(Kind::Sym(Sym::Quote2), Delim::BraceQuote2),
                    b'-' => {
                        if self.peek_byte(0) == Some(b'}') {
                            self.pos += 1;
                            Kind::Close(Delim::BraceHyphen)
                        } else {
                            while self.peek_byte(0) == Some(b'-') && self.peek_byte(1) != Some(b'}')
                            {
                                self.pos += 1;
                            }
                            Kind::Seq(Seq::Hyphen)
                        }
                    }
                    b'!' => {
                        if self.peek_byte(0) == Some(b'[') {
                            self.pos += 1;
                            Kind::Sym(Sym::ExclaimBracket)
                        } else {
                            Kind::Text
                        }
                    }
                    b'<' => Kind::Sym(Sym::Lt),
                    b'|' => Kind::Sym(Sym::Pipe),
                    b':' => Kind::Sym(Sym::Colon),
                    b'`' => self.seq(Seq::Backtick),
                    b'.' => self.seq(Seq::Period),
                    _ => Kind::Text,
                }
            }
        };
        Some(Token {
            kind,
            start,
            len: self.pos - start,
        })
    }

    fn seq(&mut self, seq: Seq) -> Kind {
        self.eat_while(|byte| byte == seq.byte());
        Kind::Seq(seq)
    }
}

/// An opener on the stack, by what closes it: `jotdown`'s `Opener`, the two spans (a
/// link's text and an image's) closing alike.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Class {
    Span,
    LinkReference,
    LinkInline,
    StrongBi,
    StrongUni,
    EmphasisBi,
    EmphasisUni,
    SuperscriptBi,
    SuperscriptUni,
    SubscriptBi,
    SubscriptUni,
    Mark,
    Delete,
    Insert,
    SingleQuoted,
    DoubleQuoted,
}

const CLASSES: usize = 16;

impl Class {
    fn index(self) -> usize {
        self as usize
    }

    /// `Opener::from_token`.
    fn opened_by(kind: Kind) -> Option<Class> {
        Some(match kind {
            Kind::Sym(Sym::Asterisk) => Class::StrongBi,
            Kind::Sym(Sym::Underscore) => Class::EmphasisBi,
            Kind::Sym(Sym::Caret) => Class::SuperscriptBi,
            Kind::Sym(Sym::Tilde) => Class::SubscriptBi,
            Kind::Sym(Sym::Quote1) | Kind::Open(Delim::BraceQuote1) => Class::SingleQuoted,
            Kind::Sym(Sym::Quote2) | Kind::Open(Delim::BraceQuote2) => Class::DoubleQuoted,
            Kind::Sym(Sym::ExclaimBracket) | Kind::Open(Delim::Bracket) => Class::Span,
            Kind::Open(Delim::BraceAsterisk) => Class::StrongUni,
            Kind::Open(Delim::BraceUnderscore) => Class::EmphasisUni,
            Kind::Open(Delim::BraceCaret) => Class::SuperscriptUni,
            Kind::Open(Delim::BraceTilde) => Class::SubscriptUni,
            Kind::Open(Delim::BraceEqual) => Class::Mark,
            Kind::Open(Delim::BraceHyphen) => Class::Delete,
            Kind::Open(Delim::BracePlus) => Class::Insert,
            _ => return None,
        })
    }

    /// The classes `kind` closes: `Opener::closed_by`, asked of every class.
    fn closed_by(kind: Kind) -> &'static [Class] {
        match kind {
            Kind::Close(Delim::Bracket) => &[Class::Span, Class::LinkReference],
            Kind::Close(Delim::Paren) => &[Class::LinkInline],
            Kind::Sym(Sym::Asterisk) => &[Class::StrongBi],
            Kind::Close(Delim::BraceAsterisk) => &[Class::StrongUni],
            Kind::Sym(Sym::Underscore) => &[Class::EmphasisBi],
            Kind::Close(Delim::BraceUnderscore) => &[Class::EmphasisUni],
            Kind::Sym(Sym::Caret) => &[Class::SuperscriptBi],
            Kind::Close(Delim::BraceCaret) => &[Class::SuperscriptUni],
            Kind::Sym(Sym::Tilde) => &[Class::SubscriptBi],
            Kind::Close(Delim::BraceTilde) => &[Class::SubscriptUni],
            Kind::Close(Delim::BraceEqual) => &[Class::Mark],
            Kind::Close(Delim::BraceHyphen) => &[Class::Delete],
            Kind::Close(Delim::BracePlus) => &[Class::Insert],
            Kind::Sym(Sym::Quote1) | Kind::Close(Delim::BraceQuote1) => &[Class::SingleQuoted],
            Kind::Sym(Sym::Quote2) | Kind::Close(Delim::BraceQuote2) => &[Class::DoubleQuoted],
            _ => &[],
        }
    }

    /// `Opener::bidirectional`: a mark that opens only before a word and closes only
    /// after one.
    fn bidirectional(self) -> bool {
        matches!(
            self,
            Class::StrongBi
                | Class::EmphasisBi
                | Class::SuperscriptBi
                | Class::SubscriptBi
                | Class::SingleQuoted
                | Class::DoubleQuoted
        )
    }

    /// A span or a link, which `jotdown` closes even with nothing inside.
    fn closes_empty(self) -> bool {
        matches!(self, Class::Span | Class::LinkReference | Class::LinkInline)
    }

    /// Whether the event an opener leaves when it is pushed is a string: every one but
    /// a quote's, which is a quotation mark.
    fn pushes_str(self) -> bool {
        !matches!(self, Class::SingleQuoted | Class::DoubleQuoted)
    }
}

/// An opener on the stack: its class, and how many events had been pushed once it
/// was, to tell the container it opens is still empty.
#[derive(Debug, Clone, Copy)]
struct Open {
    class: Class,
    mark: u64,
}

/// The pass over one inline block.
struct Pass<'a> {
    src: &'a [u8],
    lines: &'a [Range<usize>],
    numbers: &'a [usize],
    /// The index of the line after the one being read.
    next_line: usize,
    lexer: Lexer<'a>,
    stack: Vec<Open>,
    /// For each class, the stack positions its openers hold, lowest first.
    by_class: [Vec<usize>; CLASSES],
    /// Events pushed so far, never more than the parser pushes.
    events: u64,
    /// Whether the last event pushed is certainly a string.
    last_str: bool,
    /// The length of the backtick run of an open verbatim span.
    verbatim: Option<usize>,
    /// What each step past the free ones costs: 2 for a heading, read twice.
    weight: u64,
    /// The excess steps charged to the text so far, this block's included.
    spent: u64,
    /// The steps each token looks at free, the most the text may spend past them, and
    /// the most lines a block may hold.
    limits: Limits,
    /// The index of the last line the parser has handed the pass. A line an attribute
    /// set was read ahead into is handed over then, and not again when the pass comes
    /// back to it.
    handed: usize,
    /// Whether the pass holds events it has not let go of yet.
    queued: bool,
    /// Whether the pass has let go of what it held since the parser last handed it a
    /// line, which the parser's calls return from.
    let_go: bool,
    /// Whether the token just read asked the parser for more lines, for an attribute set
    /// that turned out not to be one. The parser returned then without looking whether
    /// to let go, and reads the brace again as a word before it next looks.
    asked_for_lines: bool,
    /// The lines handed over in a row with nothing let go of in between, each one call
    /// deeper than the last.
    held: usize,
    /// The most there were at once, in this block.
    most_held: usize,
    /// The line breaks of this block a load may join, when they are being gathered: only
    /// ever for a paragraph.
    joins: Option<Vec<Join>>,
    /// A footnote label on the line being read ran to its line break without closing: the
    /// parser stops a label at a line break but not at a space, so joining that break
    /// would read the next line into the label.
    label_to_line_end: bool,
}

/// The allowances and the ceilings a text is held to: [`FREE_STEPS_PER_TOKEN`],
/// [`MAX_EXCESS_STEPS`] and [`MAX_HELD_LINES`], except in tests measuring the work
/// itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Limits {
    pub(crate) free: u64,
    pub(crate) max: u64,
    pub(crate) max_held: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            free: FREE_STEPS_PER_TOKEN,
            max: MAX_EXCESS_STEPS,
            max_held: MAX_HELD_LINES,
        }
    }
}

/// What one block cost: the text's excess steps by its end, and the most lines the
/// block held in a row.
#[derive(Debug, Clone)]
struct BlockCost {
    spent: u64,
    most_held: usize,
    joins: Vec<Join>,
}

impl<'a> Pass<'a> {
    fn new(
        src: &'a [u8],
        lines: &'a [Range<usize>],
        numbers: &'a [usize],
        weight: u64,
        spent: u64,
        limits: Limits,
        gather_joins: bool,
    ) -> Self {
        Self {
            limits,
            joins: gather_joins.then(Vec::new),
            label_to_line_end: false,
            src,
            lines,
            numbers,
            next_line: 0,
            lexer: Lexer::new(src, 0, 0),
            stack: Vec::new(),
            by_class: Default::default(),
            events: 0,
            last_str: false,
            verbatim: None,
            weight,
            spent,
            handed: 0,
            queued: false,
            let_go: false,
            asked_for_lines: false,
            held: 0,
            most_held: 0,
        }
    }

    /// Read the whole block, and return what it cost, or which ceiling the text passed
    /// and where.
    fn run(mut self) -> Result<BlockCost, Exceeded> {
        while let Some(range) = self.lines.get(self.next_line) {
            self.lexer = Lexer::new(self.src, range.start, range.end);
            self.next_line += 1;
            self.label_to_line_end = false;
            while let Some(token) = self.lexer.eat() {
                self.token(token)?;
                if !std::mem::take(&mut self.asked_for_lines) {
                    self.settle();
                }
            }
            self.line_ends()?;
        }
        Ok(BlockCost {
            spent: self.spent,
            most_held: self.most_held,
            joins: self.joins.unwrap_or_default(),
        })
    }

    /// Note the line break `token` as one a load may join to the next line, when this
    /// block's breaks are being gathered and the parser reads this one as a space of the
    /// words (or, `in_code`, keeps it inside a code span).
    ///
    /// Not inside a link's destination or reference, which the parser rebuilds from its
    /// lines without their breaks (a destination) or with each as one space (a
    /// reference), not after a footnote label the break ended, which a space would not
    /// end, and not before a line opening with an attribute set, which the parser
    /// attaches to the word before a space but not to anything before a break.
    fn note_join(&mut self, token: Token, in_code: bool) {
        if self.joins.is_none()
            || self.label_to_line_end
            || !self.by_class[Class::LinkInline.index()].is_empty()
            || !self.by_class[Class::LinkReference.index()].is_empty()
        {
            return;
        }
        let Some(next) = self.lines.get(self.next_line) else {
            return;
        };
        if self.src.get(next.start) == Some(&b'{') {
            return;
        }
        let join = Join {
            at: token.start,
            resume: next.start,
            in_code,
        };
        if let Some(joins) = self.joins.as_mut() {
            joins.push(join);
        }
    }

    /// `Parser::next`'s loop, after each token: the pass lets go of what it holds, and
    /// the parser's calls return, once it holds something and nothing keeps it: no
    /// opener, no verbatim span, and a last event that is not a string, which the next
    /// token might still add to.
    fn settle(&mut self) {
        if self.queued && self.stack.is_empty() && self.verbatim.is_none() && !self.last_str {
            self.queued = false;
            self.let_go = true;
        }
    }

    /// The pass has read to the end of a line (the one before `next_line`, which an
    /// attribute set may have moved on), and asks for the next. A line the parser has
    /// already handed over, because an attribute set was read ahead into it, is read on
    /// without a call.
    fn line_ends(&mut self) -> Result<(), Exceeded> {
        let next = self.next_line;
        if next >= self.lines.len() || next <= self.handed {
            // The block's last line, after which nothing more is handed over, or a line
            // the pass already has.
            return Ok(());
        }
        self.hand_over(next)
    }

    /// The parser hands the pass the line at `index`, from inside the call that asked
    /// for it: one call deeper than the line before, unless the pass let go of what it
    /// held in between and the calls returned.
    fn hand_over(&mut self, index: usize) -> Result<(), Exceeded> {
        self.handed = index;
        if std::mem::take(&mut self.let_go) {
            self.held = 0;
            return Ok(());
        }
        self.held += 1;
        self.most_held = self.most_held.max(self.held);
        if self.held > self.limits.max_held {
            Err(Exceeded::HeldLines {
                line: self.numbers.get(index).copied().unwrap_or(1),
            })
        } else {
            Ok(())
        }
    }

    fn line_number(&self) -> usize {
        self.numbers
            .get(self.next_line.saturating_sub(1))
            .copied()
            .unwrap_or(1)
    }

    /// Record an event: a string, or anything else.
    fn push(&mut self, str: bool) {
        self.events += 1;
        self.last_str = str;
        self.queued = true;
    }

    fn charge(&mut self, scanned: usize) -> Result<(), Exceeded> {
        let excess = (scanned as u64).saturating_sub(self.limits.free);
        self.spent = self
            .spent
            .saturating_add(excess.saturating_mul(self.weight));
        if self.spent > self.limits.max {
            Err(Exceeded::Steps {
                line: self.line_number(),
            })
        } else {
            Ok(())
        }
    }

    /// `Parser::parse_event`, for one token.
    fn token(&mut self, token: Token) -> Result<(), Exceeded> {
        if let Some(open) = self.verbatim {
            if token.kind == Kind::Seq(Seq::Backtick) && token.len == open {
                self.verbatim = None;
                // The parser looks for a raw format while its lexer still reads the
                // line as verbatim, so the token it peeks there ignores an escape.
                let raw = self.raw_format();
                self.lexer.verbatim = false;
                self.push(false);
                if !raw && self.next_is_brace() {
                    // Attributes on the span replace its placeholder: no event.
                    self.attributes(token.end())?;
                }
            } else {
                if token.kind == Kind::Newline {
                    self.note_join(token, true);
                }
                self.push(true);
            }
            return Ok(());
        }
        if token.kind == Kind::Seq(Seq::Backtick) && token.len <= usize::from(u8::MAX) {
            self.verbatim = Some(token.len);
            self.lexer.verbatim = true;
            self.push(false);
            return Ok(());
        }
        if token.kind == Kind::Open(Delim::Brace)
            && let Some(filled) = self.attributes(token.start)?
        {
            // Attributes after a word leave two events, the set and a placeholder for
            // the word, unless the set holds nothing, when they leave none.
            if filled {
                self.events += 2;
                self.last_str = false;
                self.queued = true;
            }
            return Ok(());
        }
        if token.kind == Kind::Sym(Sym::Lt) && self.autolink() {
            return Ok(());
        }
        if token.kind == Kind::Sym(Sym::Colon) && self.symbol() {
            return Ok(());
        }
        if token.kind == Kind::Open(Delim::Bracket)
            && self
                .lexer
                .peek()
                .is_some_and(|next| next.kind == Kind::Sym(Sym::Caret))
        {
            // The caret is taken whether or not a label follows.
            self.lexer.eat();
            if self.footnote_label() {
                self.push(false);
                return Ok(());
            }
        }
        if self.container(token)? {
            return Ok(());
        }
        if token.kind == Kind::Newline {
            // `parse_atom`'s soft break.
            self.note_join(token, false);
        }
        // `parse_atom`, or the string every other token is.
        let atom = matches!(
            token.kind,
            Kind::Newline
                | Kind::Hardbreak
                | Kind::Escape
                | Kind::Nbsp
                | Kind::Sym(Sym::Quote1 | Sym::Quote2)
                | Kind::Open(Delim::BraceQuote1 | Delim::BraceQuote2)
                | Kind::Close(Delim::BraceQuote1 | Delim::BraceQuote2)
        ) || (token.kind == Kind::Seq(Seq::Hyphen) && token.len >= 2);
        self.push(!atom);
        Ok(())
    }

    fn next_is_brace(&mut self) -> bool {
        self.lexer
            .peek()
            .is_some_and(|next| next.kind == Kind::Open(Delim::Brace))
    }

    /// `Parser::parse_container`: close the topmost opener the token closes, or push
    /// the token as an opener. Returns whether it did either.
    fn container(&mut self, token: Token) -> Result<bool, Exceeded> {
        let found = Class::closed_by(token.kind)
            .iter()
            .filter_map(|class| self.by_class[class.index()].last().copied())
            .max();
        self.charge(found.map_or(self.stack.len(), |at| self.stack.len() - at))?;
        if let Some(at) = found {
            let open = self.stack[at];
            let empty = !open.class.closes_empty() && self.events == open.mark;
            let space_before = self
                .src
                .get(token.start.saturating_sub(1))
                .is_some_and(u8::is_ascii_whitespace);
            if !empty && !(open.class.bidirectional() && space_before) {
                self.drain(at);
                match open.class {
                    Class::Span => {
                        if let Some(next) = self.lexer.peek()
                            && matches!(next.kind, Kind::Open(Delim::Bracket | Delim::Paren))
                        {
                            // The span is a link's text: its destination or its
                            // reference follows, and closes as a link.
                            self.push(true);
                            let class = if next.kind == Kind::Open(Delim::Paren) {
                                Class::LinkInline
                            } else {
                                Class::LinkReference
                            };
                            self.push_open(class);
                            self.lexer.eat();
                            self.push(true);
                            return Ok(true);
                        }
                        self.push(true);
                    }
                    // A link rewrites the events it spanned, down to its destination,
                    // and ends on the event that closes it.
                    Class::LinkReference | Class::LinkInline => {
                        self.events = open.mark;
                        self.last_str = false;
                    }
                    _ => self.push(false),
                }
                if self.next_is_brace() && self.attributes(token.end())? == Some(true) {
                    // A set with something in it turns the span it closes into an
                    // element, ending on its exit rather than on the `]`.
                    self.last_str = false;
                }
                return Ok(true);
            }
        }
        let Some(class) = Class::opened_by(token.kind) else {
            return Ok(false);
        };
        let space_after = self
            .lexer
            .ahead()
            .first()
            .is_none_or(u8::is_ascii_whitespace);
        if class.bidirectional() && space_after {
            return Ok(false);
        }
        if class == Class::SingleQuoted {
            let space_before = token.start > 0 && self.src[token.start - 1].is_ascii_whitespace();
            if self.last_str && !space_before {
                return Ok(false);
            }
        }
        // A placeholder, then the opener's own event.
        self.events += 1;
        self.push(class.pushes_str());
        self.push_open(class);
        Ok(true)
    }

    fn push_open(&mut self, class: Class) {
        let at = self.stack.len();
        self.stack.push(Open {
            class,
            mark: self.events,
        });
        self.by_class[class.index()].push(at);
    }

    /// Close the opener at `at` and every one above it.
    fn drain(&mut self, at: usize) {
        self.stack.truncate(at);
        for positions in &mut self.by_class {
            while positions.last().is_some_and(|&position| position >= at) {
                positions.pop();
            }
        }
    }

    /// `ahead_raw_format`: a `{=format}` right after a verbatim span is skipped whole.
    fn raw_format(&mut self) -> bool {
        if !self
            .lexer
            .peek()
            .is_some_and(|next| next.kind == Kind::Open(Delim::BraceEqual))
        {
            return false;
        }
        let mut end = false;
        let len = self
            .lexer
            .ahead()
            .iter()
            .skip(2)
            .take_while(|&&byte| {
                if byte == b'{' {
                    return false;
                }
                if byte == b'}' {
                    end = true;
                }
                !end && !byte.is_ascii_whitespace()
            })
            .count();
        if len > 0 && end {
            self.lexer.eat();
            self.lexer.skip_ahead(len + 1);
            true
        } else {
            false
        }
    }

    /// `parse_autolink`: `<` then an address to a `>` on the same line.
    fn autolink(&mut self) -> bool {
        let (mut end, mut url) = (false, false);
        let len = self
            .lexer
            .ahead()
            .iter()
            .take_while(|&&byte| {
                if byte == b'<' {
                    return false;
                }
                if byte == b'>' {
                    end = true;
                }
                if matches!(byte, b':' | b'@') {
                    url = true;
                }
                !end && !byte.is_ascii_whitespace()
            })
            .count();
        if end && url {
            self.lexer.skip_ahead(len + 1);
            self.push(false);
            true
        } else {
            false
        }
    }

    /// `parse_symbol`: `:name:` on the same line.
    fn symbol(&mut self) -> bool {
        let (mut end, mut valid) = (false, true);
        let len = self
            .lexer
            .ahead()
            .iter()
            .take_while(|&&byte| {
                if byte == b':' {
                    end = true;
                } else if !byte.is_ascii_alphanumeric() && !matches!(byte, b'-' | b'+' | b'_') {
                    valid = false;
                }
                !end && !byte.is_ascii_whitespace()
            })
            .count();
        if end && valid {
            self.lexer.skip_ahead(len + 1);
            self.push(false);
            true
        } else {
            false
        }
    }

    /// `parse_footnote_reference`, past its caret: a label to a `]` on the same line.
    fn footnote_label(&mut self) -> bool {
        let mut end = false;
        let ahead = self.lexer.ahead();
        let len = ahead
            .iter()
            .take_while(|&&byte| {
                if byte == b'[' {
                    return false;
                }
                if byte == b']' {
                    end = true;
                }
                !end && byte != b'\n'
            })
            .count();
        if end {
            self.lexer.skip_ahead(len + 1);
            true
        } else {
            if ahead.get(len) == Some(&b'\n') {
                self.label_to_line_end = true;
            }
            false
        }
    }

    /// `ahead_attributes` and `resume_attributes`: validate the attribute sets that
    /// start at `start` (a `{`), across the rest of the block, and skip them whole when
    /// at least one is complete. Returns `None` when none is, and otherwise whether they
    /// hold anything (a class, an identifier, a pair or a comment), which decides the
    /// events they leave: the caller's to count.
    ///
    /// When the validation read on into later lines, the parser goes on lexing from the
    /// end of the attributes to the end of the last line it read, the bytes between the
    /// lines included (their line breaks, and the markers and indentation a line's words
    /// are otherwise read without), and so does this.
    ///
    /// Each line the validation reads on into, the parser asks for with the set still
    /// open, so each is handed over while the pass holds, the first time it is read.
    fn attributes(&mut self, start: usize) -> Result<Option<bool>, Exceeded> {
        let Some(mut line) = self.next_line.checked_sub(1) else {
            return Ok(None);
        };
        let mut from = start;
        let mut state = AttrState::Start;
        // The end of the last complete set, and whether the sets up to it hold anything.
        let mut end = None;
        let mut filled = false;
        let mut filling = false;
        let mut asked = false;
        'lines: while let Some(range) = self.lines.get(line).cloned() {
            let mut at = from;
            while at < range.end {
                state = state.step(self.src[at]);
                at += 1;
                match state {
                    AttrState::Done => {
                        end = Some(at);
                        filled |= filling;
                        if self.src.get(at) == Some(&b'{') {
                            // Another set follows at once, and belongs with this one.
                            state = AttrState::Start;
                            filling = false;
                            continue;
                        }
                        break 'lines;
                    }
                    AttrState::Invalid => break 'lines,
                    AttrState::Start | AttrState::Whitespace => {}
                    _ => filling = true,
                }
            }
            let Some(next) = self.lines.get(line + 1) else {
                break;
            };
            line += 1;
            from = next.start;
            if line > self.handed {
                // The set is still open: the parser asks for this line with nothing let go.
                self.hand_over(line)?;
                asked = true;
            }
        }
        let Some((after, range_end)) = end.zip(self.lines.get(line).map(|range| range.end)) else {
            self.asked_for_lines = asked;
            return Ok(None);
        };
        self.next_line = line + 1;
        self.lexer = Lexer::new(self.src, after, range_end);
        Ok(Some(filled))
    }
}

/// Whether the parser hands a block's lines over without their leading whitespace:
/// every block's but a caption's, which reach it as they are past the caption's `^ `.
fn trims(kind: InlineKind) -> bool {
    kind != InlineKind::Caption
}

/// The inline blocks of one text, gathered line by line as the nesting scan reads it,
/// each charged as it ends.
pub(crate) struct InlineWork<'t> {
    src: &'t [u8],
    kind: Option<InlineKind>,
    lines: Vec<Range<usize>>,
    numbers: Vec<usize>,
    spent: u64,
    /// The most lines any block has held in a row.
    most_held: usize,
    limits: Limits,
    /// When the line breaks a load may join are being gathered: from the paragraphs
    /// holding at least this many lines in a row.
    join_from: Option<usize>,
    /// The line breaks gathered so far, in the order of the text.
    joins: Vec<Join>,
}

impl<'t> InlineWork<'t> {
    /// The inline blocks of `text`, held to `limits`.
    pub(crate) fn with_limits(text: &'t str, limits: Limits) -> Self {
        Self {
            src: text.as_bytes(),
            kind: None,
            lines: Vec::new(),
            numbers: Vec::new(),
            spent: 0,
            most_held: 0,
            limits,
            join_from: None,
            joins: Vec::new(),
        }
    }

    /// Also gather the line breaks a load may join, in every paragraph that holds at
    /// least `from` lines in a row: see [`joins`](Self::joins).
    pub(crate) fn gathering_joins(mut self, from: usize) -> Self {
        self.join_from = Some(from);
        self
    }

    /// The line breaks gathered (see [`gathering_joins`](Self::gathering_joins)), in the
    /// order of the text.
    pub(crate) fn joins(&self) -> &[Join] {
        &self.joins
    }

    /// The steps charged so far, past each token's free ones.
    pub(crate) fn spent(&self) -> u64 {
        self.spent
    }

    /// The most lines a block has held in a row so far.
    pub(crate) fn most_held(&self) -> usize {
        self.most_held
    }

    /// Take what the nesting scan found on the line numbered `number` (from one),
    /// which spans `line` in the text, its break included. The ceiling the text passes,
    /// and the line where it does, is the error.
    pub(crate) fn line(
        &mut self,
        inline: Inline,
        line: Range<usize>,
        number: usize,
    ) -> Result<(), Exceeded> {
        match inline {
            Inline::None => self.end_block(),
            Inline::Starts { kind, from } => {
                self.end_block()?;
                self.kind = Some(kind);
                self.add(line.start + from, line.end, number, trims(kind));
                Ok(())
            }
            Inline::Continues { kind, from } => {
                if self.kind != Some(kind) {
                    self.end_block()?;
                    self.kind = Some(kind);
                }
                self.add(line.start + from, line.end, number, trims(kind));
                Ok(())
            }
        }
    }

    /// The text has been read: charge its last block.
    pub(crate) fn finish(&mut self) -> Result<(), Exceeded> {
        self.end_block()
    }

    fn add(&mut self, from: usize, to: usize, number: usize, trim: bool) {
        let to = to.min(self.src.len());
        let mut from = from.min(to);
        if trim {
            while from < to && self.src[from].is_ascii_whitespace() {
                from += 1;
            }
        }
        self.lines.push(from..to);
        self.numbers.push(number);
    }

    fn end_block(&mut self) -> Result<(), Exceeded> {
        let Some(kind) = self.kind.take() else {
            return Ok(());
        };
        // The block's last line loses its trailing whitespace, then the lines left
        // empty are not handed over at all.
        if let Some(last) = self.lines.last_mut() {
            while last.end > last.start && self.src[last.end - 1].is_ascii_whitespace() {
                last.end -= 1;
            }
        }
        let mut lines = Vec::with_capacity(self.lines.len());
        let mut numbers = Vec::with_capacity(self.numbers.len());
        for (range, &number) in self.lines.drain(..).zip(self.numbers.iter()) {
            if !range.is_empty() {
                lines.push(range);
                numbers.push(number);
            }
        }
        self.numbers.clear();
        if kind == InlineKind::TableRow {
            // Each cell of a row is read on its own, the parser starting afresh.
            for (row, &number) in lines.iter().zip(&numbers) {
                for cell in self.cells(row.clone()) {
                    self.charge_block(&[cell], &[number], 1, false)?;
                }
            }
            return Ok(());
        }
        let weight = if kind == InlineKind::Heading { 2 } else { 1 };
        self.charge_block(&lines, &numbers, weight, kind == InlineKind::Paragraph)
    }

    /// Run the pass over one block's `lines`, and add up what it cost. A `paragraph`'s
    /// line breaks are gathered when a load asked for them and it holds enough lines.
    fn charge_block(
        &mut self,
        lines: &[Range<usize>],
        numbers: &[usize],
        weight: u64,
        paragraph: bool,
    ) -> Result<(), Exceeded> {
        let gather = paragraph && self.join_from.is_some();
        let cost = Pass::new(
            self.src,
            lines,
            numbers,
            weight,
            self.spent,
            self.limits,
            gather,
        )
        .run()?;
        self.spent = cost.spent;
        self.most_held = self.most_held.max(cost.most_held);
        if self.join_from.is_some_and(|from| cost.most_held >= from) {
            self.joins.extend(cost.joins);
        }
        Ok(())
    }

    /// The cells of a table row, as `jotdown`'s `parse_table` splits it: at each `|`
    /// outside a verbatim span, after the row's own opening `|`, each cell trimmed.
    fn cells(&self, row: Range<usize>) -> Vec<Range<usize>> {
        let trim = |mut cell: Range<usize>| {
            while cell.start < cell.end && self.src[cell.start].is_ascii_whitespace() {
                cell.start += 1;
            }
            while cell.end > cell.start && self.src[cell.end - 1].is_ascii_whitespace() {
                cell.end -= 1;
            }
            cell
        };
        let row = trim(row);
        let mut cells = Vec::new();
        if row.is_empty() {
            return cells;
        }
        let mut lexer = Lexer::new(self.src, row.start + 1, row.end);
        let mut cell_start = row.start + 1;
        let mut verbatim = None;
        while let Some(token) = lexer.eat() {
            match verbatim {
                Some(len) => {
                    if token.kind == Kind::Seq(Seq::Backtick) && token.len == len {
                        lexer.verbatim = false;
                        verbatim = None;
                    }
                }
                None => match token.kind {
                    Kind::Sym(Sym::Pipe) => {
                        cells.push(trim(cell_start..token.start));
                        cell_start = token.end();
                    }
                    Kind::Seq(Seq::Backtick) => {
                        lexer.verbatim = true;
                        verbatim = Some(token.len);
                    }
                    _ => {}
                },
            }
        }
        cells.retain(|cell| !cell.is_empty());
        cells
    }
}

#[cfg(test)]
#[path = "djot_inline_tests.rs"]
pub(crate) mod tests;
