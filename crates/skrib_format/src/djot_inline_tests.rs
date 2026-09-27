// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The inline pass's work and the lines it holds open: counted as `jotdown` counts
//! them, refused past their ceilings, and never charged for what the editor or an
//! importer writes.

use std::time::{Duration, Instant};

use super::*;
use crate::djot_depth::{DjotRefusal, InlineCost, check, scan};

/// What the inline pass costs `text` with no ceiling, none of its steps free.
fn cost(text: &str) -> InlineCost {
    match scan(
        text,
        Limits {
            free: 0,
            max: u64::MAX,
            max_held: usize::MAX,
        },
    ) {
        Ok(cost) => cost,
        Err(refused) => panic!("{text:?} was refused: {refused}"),
    }
}

/// Every stack entry the inline pass looks at on `text`, none of them free.
fn steps(text: &str) -> u64 {
    cost(text).steps
}

/// The most lines one block of `text` goes into the parser's calls.
fn held(text: &str) -> usize {
    cost(text).held_lines
}

/// Parse `djot` on a thread with the stack a long operation gets, as the editor and
/// the exporter do, and return how long it took.
fn parse_time(djot: String) -> Duration {
    let start = Instant::now();
    let parsed = crate::djot_depth::tests::parse_on_a_long_operation_stack(djot);
    assert!(parsed.is_ok(), "{parsed:?}");
    start.elapsed()
}

/// The counts `jotdown` 0.10 itself gives, taken from a copy of it that adds up the
/// entries `Parser::parse_container`'s `rposition` looks at (and checked equal to this
/// scan's on two hundred thousand generated documents). Each pins a rule the scan
/// follows: a verbatim span's peek ignoring an escape, a set of empty attributes leaving
/// no event, a quote's closing brace, attributes turning a span into an element, a
/// footnote label stopping at a cell's edge, a heading read twice, a table's cells and
/// caption, a quotation's lazy line, a list item's continuation, a link across lines,
/// attributes across lines, the formats the editor writes, and a code block's lines.
#[test]
fn the_work_is_counted_as_the_parser_does_it() {
    for (text, expected) in [
        ("a [b [c](d) e] f [g", 11),
        ("*strong _emph_ *not closed", 9),
        ("`x`\\[:", 1),
        ("~{}~~", 2),
        ("\"{#i}\"}\"||", 3),
        ("[]{a=\"\"}'=", 2),
        ("|[^w|]|", 1),
        ("# [ [ [ heading\n# more [", 36),
        (
            "| [ | [ [ |\n|---|---|\n| a | b |\n^ [ caption\nstill [ caption",
            8,
        ),
        ("> [ quoted [\nlazy [ line", 11),
        ("- [ item\n  [ continued", 5),
        ("[x\ny](u\nv) [", 8),
        ("{a=\"x\ny\"} [ [", 2),
        ("don't 'quote' \"dialogue\" it's", 4),
        (
            "{+ins+} {-del-} {=mark=} ^sup^ ~sub~ [link](url) ![img](src) [^fn]",
            18,
        ),
        ("```\n[ [ [\n```\n[ [", 2),
    ] {
        assert_eq!(steps(text), expected, "{text:?}");
    }
}

/// Every kind of opener nothing closes costs every token after it in its paragraph, and
/// 160 KB of any of them is refused, at the line holding them, before anything parses
/// them. Measured before the ceiling, in a release build: 7.3 seconds for `[`, and two
/// or more for every other kind.
#[test]
fn a_paragraph_of_unclosed_openers_is_refused_at_its_line() {
    for unit in [
        "[", "{+{-", "{~", "{_", "{=", "{*", "{^", "{'", "{\"", "[^", "![", "[x](", "[x][", "a [ ",
        "*a ", "_a ", "\"a ", "'a ", "^a ", "~a ",
    ] {
        let text = format!(
            "Before.\n\n{}\n\nAfter.\n",
            unit.repeat(160_000 / unit.len())
        );
        match check(&text) {
            Err(DjotRefusal::TooSlowToRead { line }) => assert_eq!(line, 3, "{unit:?}"),
            other => panic!("{unit:?}: {other:?}"),
        }
    }
}

/// The refusal costs no more than reading up to the ceiling: a megabyte of openers is
/// turned away as fast as a few thousand of them, not after the minutes the parser would
/// have spent.
#[test]
fn refusing_a_megabyte_of_openers_is_quick() {
    let text = "[".repeat(1 << 20);
    let start = Instant::now();
    assert!(matches!(
        check(&text),
        Err(DjotRefusal::TooSlowToRead { line: 1 })
    ));
    assert!(
        start.elapsed() < Duration::from_secs(5),
        "took {:?}",
        start.elapsed()
    );
}

/// What the ceiling lets through, the parser reads in a bounded time: the longest run
/// of unclosed openers it accepts parses from a long operation's stack in well under the
/// minutes a refused one would take, even in the debug build this suite runs in.
#[test]
fn what_the_ceiling_lets_through_parses_in_bounded_time() {
    let mut openers = 1_000usize;
    while check(&"[".repeat(openers * 2)).is_ok() {
        openers *= 2;
    }
    let (mut low, mut high) = (openers, openers * 2);
    while high - low > 1 {
        let middle = (low + high) / 2;
        if check(&"[".repeat(middle)).is_ok() {
            low = middle;
        } else {
            high = middle;
        }
    }
    let longest = "[".repeat(low);
    assert!(check(&longest).is_ok());
    assert!(check(&"[".repeat(low + 1)).is_err());
    let took = parse_time(longest);
    assert!(took < Duration::from_secs(60), "took {took:?}");
}

/// A heading is read twice, once to name it and once to show it, and is charged twice.
#[test]
fn a_heading_is_charged_twice() {
    let openers = "[ ".repeat(200);
    assert_eq!(
        steps(&format!("# {openers}\n")),
        2 * steps(&format!("{openers}\n"))
    );
}

/// Each cell of a table row is read afresh, so openers left in one cell cost nothing in
/// the next, and a caption is read whole, across its lines.
#[test]
fn a_tables_cells_are_read_apart_and_its_caption_whole() {
    let cell = "[ ".repeat(100);
    let one = steps(&format!("| {cell} |\n"));
    assert_eq!(steps(&format!("| {cell} | {cell} |\n")), 2 * one);
    let caption_one_line = steps(&format!("| a |\n^ {cell}{cell}\n"));
    let caption_two_lines = steps(&format!("| a |\n^ {cell}\n{cell}\n"));
    // The line break is one more token, looking at the hundred openers before it.
    assert_eq!(caption_two_lines, caption_one_line + 100);
}

/// Everything the editor writes, however much of it one paragraph holds, costs nothing:
/// its only openers are the formats and links it writes itself, all closed. Five
/// thousand links and ten thousand runs of every format in one paragraph, and a
/// footnote reference after each, written by the editor's own Djot writer.
#[test]
fn a_paragraph_the_editor_wrote_costs_nothing_however_long() {
    let mut html = String::from("<p>");
    for n in 0..5_000 {
        html.push_str(&format!(
            "Words [{n}] {{with}} *marks* _in_ \"them\" and it's <a href=\"https://example.com/{n}\">a \
             <b>bold <i>italic <u>underlined</u></i></b> link</a> then <s>struck</s>, \
             <sup>up</sup> and <sub>down</sub>. "
        ));
    }
    html.push_str("</p>");
    let doc = text_document::TextDocument::new();
    doc.set_html(&html)
        .and_then(|operation| operation.wait())
        .expect("the editor takes the paste");
    let djot = doc
        .to_djot()
        .expect("the editor writes its document as Djot");
    assert!(djot.len() > 1_000_000, "{}", djot.len());
    assert_eq!(
        scan(&djot, Limits::default()),
        Ok(InlineCost {
            steps: 0,
            held_lines: 0
        }),
        "the editor's own markup is charged"
    );
}

/// Prose an older version stored without escaping its quotation marks and apostrophes
/// still loads, however long its paragraph: a quotation mark nothing closes costs only
/// until the paragraph ends, and prose leaves few.
#[test]
fn a_long_paragraph_of_unescaped_dialogue_loads() {
    let paragraph = "\"Don't,\" she said. 'It's fine.' The '90s were \"loud and 'odd', he \
                     thought. Rock 'n' roll. "
        .repeat(4_000);
    assert!(paragraph.len() > 300_000);
    assert!(check(&paragraph).is_ok());
}

/// How deep the parser's calls go, counted as it goes: the most lines one block hands
/// the pass with nothing let go of in between, as a copy of `jotdown` 0.10 that counts
/// its own calls gives it (and checked equal to this scan's on four hundred thousand
/// generated documents). Each pins a rule: a quotation, a bracket, a code span and a
/// link held across lines; plain lines, which let go at every line's end; a closed
/// format letting go in the middle of a line, before what it opens next; an attribute
/// set read across lines, whether it turns out to be one or not, after a closed quote
/// too, when the parser does not look at whether to let go before it reads the brace
/// again as a word; a heading, a caption, a quotation and a list item going on across
/// lines; a code block's lines and a table's cells, which hold nothing.
#[test]
fn the_lines_held_are_counted_as_the_parser_recurses() {
    for (text, expected) in [
        ("\"a\nb\nc\nd", 3),
        ("[\na\nb", 2),
        ("`a\nb\nc", 2),
        ("[a](b\nc\nd)", 2),
        ("[x\ny](u\nv) [", 2),
        ("x\ny\nz", 0),
        ("_b_`\n:", 0),
        ("*a* {a=b\nc=d}", 0),
        ("{a=b\nc=d\ne=f}x", 2),
        ("{a=\"x\ny\"} [ [", 1),
        ("x `y\nz`{.c\nd}\ne", 2),
        ("'q'{a=\"a\n\"x\n}\nm", 3),
        ("\"a\" b\n\"c\nd\ne", 2),
        ("# \"a\n# b\n# c", 2),
        ("| a |\n^ \"b\nc\nd", 2),
        ("> \"quoted\n> more\nlazy", 2),
        ("- \"item\n  more\n  more", 2),
        ("```\n\"a\nb\n```", 0),
        ("| \"a | b |\n| c | d |", 0),
    ] {
        assert_eq!(held(text), expected, "{text:?}");
    }
}

/// What a load does with a paragraph past the ceiling: joins its lines into one, or
/// refuses it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PastTheCeiling {
    /// Joined, and read exactly as it was.
    Joined,
    /// Joined, the line breaks of a code span turned into spaces.
    JoinedInCode,
    /// Refused: its lines cannot be joined.
    Refused,
}

/// Every way of holding something open across a paragraph's lines: at the ceiling it
/// loads, and the parser reads it on a long operation's stack in the debug build this
/// suite runs in; one line more passes [`check`], at the line that passes the ceiling. So
/// does a paragraph a hundred thousand lines long, as soon as that line is read. The load
/// ([`admit`](crate::djot_depth::admit)) then joins its lines where it can, and refuses
/// the rest at that same line.
#[test]
fn a_paragraph_held_open_past_the_ceiling_is_joined_or_refused_at_its_line() {
    use PastTheCeiling::{Joined, JoinedInCode, Refused};
    for (first, then, load) in [
        ("\"She said", "and went on", Joined),
        ("'Twas said", "and went on", Joined),
        ("[a note", "that goes on", Joined),
        ("*bold", "and more", Joined),
        ("{_underlined", "and more", Joined),
        ("`code", "more code", JoinedInCode),
        ("{a=b", "c=d", Joined),
        ("[a link](https://example.com/a", "b", Refused),
        ("> \"Quoted", "> and more", Joined),
        ("# \"Heading", "# and more", Refused),
        ("- \"Item", "  and more", Joined),
    ] {
        let paragraph =
            |lines: usize| format!("Before.\n\n{first}\n{}", format!("{then}\n").repeat(lines));
        let at_the_ceiling = paragraph(MAX_HELD_LINES);
        assert_eq!(
            scan(&at_the_ceiling, Limits::default()).map(|cost| cost.held_lines),
            Ok(MAX_HELD_LINES),
            "{first:?}"
        );
        assert_eq!(
            crate::djot_depth::admit(at_the_ceiling.clone()).as_ref(),
            Ok(&at_the_ceiling),
            "{first:?}"
        );
        let parsed = crate::djot_depth::tests::parse_on_a_long_operation_stack(at_the_ceiling);
        assert!(parsed.is_ok(), "{first:?}: {parsed:?}");
        // The paragraph opens on line 3, and each line after it goes one call deeper.
        let past = MAX_HELD_LINES + 3 + 1;
        for lines in [MAX_HELD_LINES + 1, 100_000] {
            let text = paragraph(lines);
            assert_eq!(
                check(&text),
                Err(DjotRefusal::HeldOpenTooLong { line: past }),
                "{first:?}, {lines} lines"
            );
            let admitted = crate::djot_depth::admit(text.clone());
            match (load, admitted) {
                (Refused, Err(refused)) => assert_eq!(
                    refused,
                    DjotRefusal::HeldOpenTooLong { line: past },
                    "{first:?}, {lines} lines"
                ),
                (Joined | JoinedInCode, Ok(joined)) => {
                    assert_eq!(check(&joined), Ok(()), "{first:?}, {lines} lines");
                    if lines < 1_000 {
                        let (before, after) = (
                            crate::djot_depth::tests::reading(text),
                            crate::djot_depth::tests::reading(joined),
                        );
                        if load == Joined {
                            assert_eq!(after, before, "{first:?}: read as it was");
                        } else {
                            let words = |read: Option<(String, String)>| {
                                read.map(|(_, plain)| {
                                    plain
                                        .split_whitespace()
                                        .map(str::to_string)
                                        .collect::<Vec<_>>()
                                })
                            };
                            assert_eq!(words(after), words(before), "{first:?}: every word kept");
                        }
                    }
                }
                (expected, got) => panic!("{first:?}, {lines} lines: {expected:?}, {got:?}"),
            }
        }
    }
}

/// A paragraph that lets go of what it holds, even once in a while, starts again: a
/// hundred thousand lines, each closing the quotation the one before opened, or each a
/// few plain words, go no deeper than a line or two.
#[test]
fn a_paragraph_that_closes_what_it_opens_goes_no_deeper() {
    let closing = "\"Words\nmore words,\" she said.\n".repeat(50_000);
    assert_eq!(held(&closing), 1);
    assert!(check(&closing).is_ok());
    let plain = "A line of a paragraph written by hand.\n".repeat(100_000);
    assert_eq!(held(&plain), 0);
    assert!(check(&plain).is_ok());
}

/// What an importer writes holds nothing open across lines, however its source was
/// wrapped: a Markdown paragraph of a thousand lines, opening a quotation and a bracket
/// on its first and closing neither, comes out as one line of Djot.
#[test]
fn a_paragraph_imported_from_wrapped_lines_holds_nothing() {
    let markdown = format!(
        "\"She said, [and `then`\n{}",
        "she went on, *and on\n".repeat(1_000)
    );
    let djot = crate::convert::markdown_to_djot(&markdown).expect("the importer converts it");
    assert_eq!(held(&djot), 0, "{}", &djot[..djot.len().min(200)]);
    assert!(check(&djot).is_ok());
}
