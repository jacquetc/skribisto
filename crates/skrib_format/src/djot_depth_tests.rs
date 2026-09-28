// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Djot ceiling: what it refuses, what it lets through, and proof that what it
//! lets through parses.
//!
//! The parses run on a thread with a quarter of the 2 MiB stack a long operation
//! gets ([`QUARTER_STACK`]), in the debug build the suite runs in, through
//! `set_djot_sync`, which runs `jotdown` on the calling thread. A stack overflow is
//! not a panic, so a regression here does not fail a test: it aborts the test
//! binary, which is the signal.

use super::*;
use proptest::prelude::*;
use text_document::{ListStyle, TextDocument};

/// The stack every parse here runs on: 512 KiB, a quarter of the 2 MiB a long
/// operation's worker gets from `std::thread::spawn`.
///
/// Unlike the importers, `jotdown` recurses once per container by design, on
/// whatever thread parses, and the Djot ceiling is what bounds it: at
/// [`MAX_DEPTH`] it needs about 350 KiB in a debug build on Linux. A pass on a
/// quarter of the real stack proves the real one holds four times what the parse
/// needs here, and its frames were measured about 6 % larger on macOS, well inside
/// that. See [`crate::xml_depth`]'s module note.
const QUARTER_STACK: usize = 512 * 1024;

/// Parse `djot` on a thread standing in for a long operation's, with
/// [`QUARTER_STACK`], and return its plain text, or the parser's own error.
pub(crate) fn parse_on_a_long_operation_stack(djot: String) -> Result<String, String> {
    std::thread::Builder::new()
        .stack_size(QUARTER_STACK)
        .spawn(move || {
            let doc = TextDocument::new();
            doc.set_djot_sync(&djot).map_err(|e| e.to_string())?;
            doc.to_plain_text().map_err(|e| e.to_string())
        })
        .expect("spawn the parse thread")
        .join()
        .expect("the parse must not unwind")
}

/// The nesting refusal `check` gives `text`: it must be refused, and for how deeply it
/// nests.
fn refused_for_nesting(text: &str, why: &str) -> TooDeep {
    match check(text) {
        Err(DjotRefusal::TooDeep(refused)) => refused,
        other => panic!("{why}: {other:?}"),
    }
}

/// Every marker `jotdown` 0.10 opens a container with at the start of a line, as
/// the text that opens one more level when it is repeated on one line.
const ONE_LINE_MARKERS: [&str; 19] = [
    "- ", "* ", "+ ", "1. ", "1) ", "(1) ", "a. ", "B) ", "(c) ", "iv. ", "XII) ", "(ix) ",
    "- [ ] ", "* [x] ", "+ [X] ", "[^a]: ", "[^note]:", ": ", "> ",
];

/// `levels` of `marker` on one line, then a word, which the last one holds.
pub(crate) fn one_line(marker: &str, levels: usize) -> String {
    format!("{}deep\n", marker.repeat(levels))
}

/// A list whose items each step in past the last by the width of their marker, with a
/// paragraph line at no indentation after each and a blank line before the next. The
/// paragraph line continues every item open above it, and each item strips at most its
/// marker's width from the lines it continues, so each item nests inside the last:
/// `levels` deep. `prefix` goes in front of every line, a quotation's `> ` for instance.
fn lazy_staircase(marker: &str, levels: usize, prefix: &str) -> String {
    let step = marker.trim_end().len();
    (0..levels)
        .map(|level| {
            format!(
                "{prefix}{}{marker}item {level}\n{prefix}lazy\n",
                " ".repeat(step * level)
            )
        })
        .collect::<Vec<_>>()
        .join(&format!("{}\n", prefix.trim_end()))
}

/// How deep the hostile prose below nests: past the 617 containers at which the
/// parser aborts a 2 MiB thread in a debug build.
const PAST_THE_PARSERS_LIMIT: usize = 700;

/// Prose the parser cannot survive on a long operation's stack, in every shape a
/// container can take. The first was already refused before the guard counted
/// markers other than `>`; every other one passed it, and aborted the process the
/// first time the row was parsed.
pub(crate) fn past_the_parsers_limit() -> Vec<(&'static str, String)> {
    let levels = PAST_THE_PARSERS_LIMIT;
    let mixture: String = ["- ", "> ", "1. ", "[^a]: ", ": ", "(iv) ", "- [ ] "]
        .iter()
        .cycle()
        .take(levels)
        .copied()
        .collect();
    let descending = (0..levels)
        .map(|i| ":".repeat(levels + 2 - i))
        .collect::<Vec<_>>()
        .join("\n")
        + "\ndeep\n";
    vec![
        (
            "a run of blockquote markers, a tab after each",
            format!("{}deep\n", ">\t".repeat(4_000)),
        ),
        ("bullets on one line", one_line("- ", levels)),
        ("ordered items on one line", one_line("1. ", levels)),
        ("roman numerals in parentheses", one_line("(iv) ", levels)),
        ("task items on one line", one_line("- [ ] ", levels)),
        (
            "footnote definitions on one line",
            one_line("[^a]: ", levels),
        ),
        ("definition list items on one line", one_line(": ", levels)),
        ("a mixture on one line", format!("{mixture}deep\n")),
        (
            "a div opened inside a quote on every line",
            "> ::: note\n".repeat(levels),
        ),
        ("fences each one colon shorter", descending),
        (
            "divs a code fence keeps open",
            format!("::: a\n{}", "- item\n\n  ```x\n:::\n".repeat(levels)),
        ),
        (
            "list items stepping in between paragraph lines",
            lazy_staircase("- ", levels, ""),
        ),
        (
            "footnotes stepping in between paragraph lines",
            lazy_staircase("[^a]: ", levels, ""),
        ),
        (
            "quoted list items stepping in between paragraph lines",
            lazy_staircase("1. ", levels, "> "),
        ),
    ]
}

/// How many lines the paragraphs below hold something open over: past the 3,952 that
/// overflow a 2 MiB stack in a release build, and far past the 721 of a debug one.
const PAST_THE_LINES_THE_PARSER_HOLDS: usize = 4_000;

/// Djot the parser cannot be given though it nests nothing past the ceiling, and a word
/// its refusal is said with: a heading deeper than the parser can count, which panics
/// it, paragraphs of openers nothing closes, which it takes minutes to read, and blocks
/// that keep one thing open over thousands of lines no join can shorten, which overflow
/// its stack.
pub(crate) fn beyond_the_parser() -> Vec<(&'static str, String, &'static str)> {
    let lines = PAST_THE_LINES_THE_PARSER_HOLDS;
    vec![
        (
            "a link's destination left open over thousands of lines",
            format!("[a link](https://example.com/\n{}", "more/\n".repeat(lines)),
            "cannot be joined",
        ),
        (
            "a heading holding a quotation mark open over thousands of lines",
            format!("# \"She said\n{}", "# and went on\n".repeat(lines)),
            "cannot be joined",
        ),
        (
            "a heading deeper than the parser counts",
            format!("{} Zeus\n", "#".repeat(MAX_HEADING_LEVEL + 1)),
            "heading",
        ),
        (
            "a paragraph of unclosed brackets",
            "[".repeat(40_000),
            "brackets",
        ),
        (
            "a paragraph of unclosed formatting marks",
            "{+{-".repeat(10_000),
            "brackets",
        ),
    ]
}

/// Paragraphs that keep one thing open over thousands of lines, which the parser would
/// overflow its stack on, and which [`admit`] joins into one line rather than refusing:
/// their line breaks are spaces to the parser, or, in a code span, turn into spaces. Each
/// with whether the joined paragraph reads exactly as it did.
pub(crate) fn joined_by_the_load() -> Vec<(&'static str, String, bool)> {
    let lines = PAST_THE_LINES_THE_PARSER_HOLDS;
    vec![
        (
            "a quotation mark left open over thousands of lines",
            format!("\"She said\n{}", "and went on\n".repeat(lines)),
            true,
        ),
        (
            "an attribute set read on over thousands of lines",
            format!("{{a=b\n{}", "c=d\n".repeat(lines)),
            true,
        ),
        (
            "emphasis over thousands of lines of a quotation",
            format!(
                "> _She said\n{}> and stopped_\n",
                "> and went on\n".repeat(lines)
            ),
            true,
        ),
        (
            "a code span left open over thousands of lines",
            format!("`code\n{}", "more code\n".repeat(lines)),
            false,
        ),
    ]
}

/// Every shape in [`past_the_parsers_limit`] is refused by the guard, and exists in
/// a form at the ceiling that the guard accepts and the parser reads from a long
/// operation's stack.
#[test]
fn every_hostile_shape_is_refused_and_its_ceiling_form_parses() {
    for (shape, text) in past_the_parsers_limit() {
        assert!(check(&text).is_err(), "{shape} must be refused");
    }
    for marker in ["- ", "1. ", "(iv) ", "- [ ] ", "[^a]: ", ": ", "> "] {
        let text = one_line(marker, MAX_DEPTH);
        assert!(check(&text).is_ok(), "{marker:?}");
        assert!(parse_on_a_long_operation_stack(text).is_ok(), "{marker:?}");
    }
}

#[test]
fn ordinary_prose_passes() {
    for text in [
        "The ferry was late.\n\nShe waited.\n",
        "> He said it plainly.\n>\n> Then he left.\n",
        "- one\n  - two\n    - three\n",
        "::: note\nA note.\n:::\n",
        "> - a quoted list\n>   - nested once\n",
        "1. First.\n\n   a. Inside.\n\n      i. Deeper.\n",
        "A note.[^1]\n\n[^1]: The note, which runs on.\n\n    A second paragraph.\n",
        "- [ ] a task\n- [x] a done one\n",
        "Term\n\n: its definition\n",
        "| a | b |\n|---|---|\n| c | d |\n",
        "* * *\n\n- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -\n",
        "",
    ] {
        assert!(check(text).is_ok(), "should accept: {text:?}");
    }
}

/// A blockquote marker run is the cheapest way to reach the parser's recursion, and
/// the shape that was measured aborting the process: each `>` followed by a space.
/// Without the spaces the run is a paragraph's first word to the parser, and nests
/// nothing.
#[test]
fn a_deep_blockquote_run_is_refused() {
    let err = refused_for_nesting(&format!("{}deep\n", "> ".repeat(2_000)), "2000 levels");
    assert_eq!(err.line, 1);
    assert!(err.depth > MAX_DEPTH);

    let unspaced = format!("{}deep\n", ">".repeat(2_000));
    assert!(check(&unspaced).is_ok());
    assert!(parse_on_a_long_operation_stack(unspaced).is_ok());
}

#[test]
fn deeply_stacked_divs_are_refused() {
    let text = "::: a\n".repeat(500);
    let err = refused_for_nesting(&text, "500 open divs");
    assert!(err.depth > MAX_DEPTH);
}

/// Indentation in front of no marker opens nothing, however long it runs: a
/// paragraph whose first words sit behind a thousand spaces or tabs is one paragraph.
/// The previous guard counted half a level per byte of it, and refused the project a
/// writer had typed it into (see the module note).
#[test]
fn indentation_alone_opens_nothing_however_long() {
    for blank in [" ", "\t", " \t", "\r"] {
        for text in [
            format!("{}item\n", blank.repeat(1_000)),
            format!(
                "First.\n\n{}Second.\n\n{}\n",
                blank.repeat(400),
                blank.repeat(700)
            ),
            format!("- a list item\n\n{}continued far in\n", blank.repeat(900)),
            format!("> a quotation\n>{}still in it\n", blank.repeat(900)),
            format!(
                "```\n{}code keeps its indentation\n```\n",
                blank.repeat(900)
            ),
        ] {
            assert!(check(&text).is_ok(), "{blank:?}: {text:.60?}");
            assert!(parse_on_a_long_operation_stack(text).is_ok(), "{blank:?}");
        }
    }
}

/// A lone marker behind any amount of indentation opens exactly one level, because
/// jotdown opens one list (or blockquote, or table) for it and no item encloses it.
///
/// This is the fix: the old guard counted indentation in front of a marker one level
/// per byte, so the editor's own two-spaces-a-level list — 96 spaces at 48 levels
/// deep — was read as 96 levels and refused, though jotdown nests it 48. A single
/// indented marker nests nothing, so it is depth one however far it is pushed in.
#[test]
fn a_lone_indented_marker_opens_one_level_however_far_it_is_indented() {
    for marker in ["- ", "1. ", "[^a]: ", ": ", "> ", "| a |"] {
        for indent in [0, 1, MAX_DEPTH, 4 * MAX_DEPTH, 4_000] {
            let text = format!("{}{marker}item\n", " ".repeat(indent));
            assert!(
                check(&text).is_ok(),
                "{marker:?} behind {indent} spaces must open one level, not one per byte"
            );
        }
    }
}

/// A list whose items step in one column a level, one item per line, is one item: its
/// later lines are more of its paragraph, however far in they step, since a paragraph
/// goes on through every line that is not blank. The marker count this guard used to
/// refuse on read one level per step, and turned away a hundred of them; the parser
/// nests nothing past the first. With a blank line between the items it is the parser's
/// own nesting, and counted as such (`a_list_nested_by_one_space_a_level_is_counted_per_space`).
#[test]
fn a_list_stepping_in_one_column_a_level_without_blank_lines_is_one_item() {
    let stepped = |levels: usize| {
        (0..levels)
            .map(|level| format!("{}- level {level}", " ".repeat(level)))
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    };
    for levels in [MAX_DEPTH, MAX_DEPTH + 1, 4 * MAX_DEPTH] {
        let text = stepped(levels);
        assert!(check(&text).is_ok(), "{levels}");
        assert!(parse_on_a_long_operation_stack(text).is_ok(), "{levels}");
    }
}

/// A list item goes on through a paragraph line at no indentation after a line that was
/// not blank, so a list whose items step in one marker's width at a time between such
/// lines nests one level a step, however little each one is indented. At the ceiling it
/// loads and parses; one step past it is refused at the deepest item's line.
///
/// The count of markers and indents alone read each paragraph line as closing every
/// item, and let seven hundred steps through at depth 1; the first parse of the row
/// aborted the process.
#[test]
fn a_list_continued_by_paragraph_lines_is_refused_at_its_real_depth() {
    for (marker, prefix, quotes) in [("- ", "", 0), ("[^a]: ", "", 0), ("1. ", "> ", 1)] {
        let at_the_ceiling = lazy_staircase(marker, MAX_DEPTH - quotes, prefix);
        assert!(check(&at_the_ceiling).is_ok(), "{marker:?} {prefix:?}");
        assert!(parse_on_a_long_operation_stack(at_the_ceiling).is_ok());

        let err = refused_for_nesting(
            &lazy_staircase(marker, MAX_DEPTH - quotes + 1, prefix),
            "one step past the ceiling",
        );
        assert_eq!(err.depth, MAX_DEPTH + 1, "{marker:?} {prefix:?}");
        assert_eq!(
            err.line,
            3 * MAX_DEPTH + 1 - 3 * quotes,
            "the deepest item's line"
        );
    }
}

/// Closing fences must bring the depth back down, or a long document with
/// many sibling divs would be refused for nesting it never had.
#[test]
fn sibling_divs_do_not_accumulate() {
    let text = "::: note\nbody\n:::\n".repeat(500);
    assert!(check(&text).is_ok());
}

/// Djot lets an outer div use a longer fence so another can nest inside it.
/// A `::::` line is a *close* when it carries no class; reading it as a
/// second opener made the count climb for ever.
#[test]
fn a_longer_closing_fence_closes_rather_than_opens() {
    let text = ":::: outer\n::: inner\nbody\n:::\n::::\n".repeat(200);
    assert!(
        check(&text).is_ok(),
        "nested fences must not accumulate depth"
    );
}

/// Only ASCII whitespace indents a Djot line. A paragraph opening with no-break
/// spaces, ideographic spaces or narrow no-break spaces is a paragraph whose text
/// starts with them, however many there are, and so is one where a run of `>`
/// follows them.
#[test]
fn unicode_spaces_at_a_line_start_are_text_not_indentation() {
    for space in ['\u{A0}', '\u{3000}', '\u{202F}', '\u{2003}'] {
        let text = format!(
            "{}A title set in the middle of the page.\n",
            space.to_string().repeat(300)
        );
        assert!(check(&text).is_ok(), "{space:?} is not indentation");
        let text = format!("{space}{}x\n", ">".repeat(300));
        assert!(check(&text).is_ok(), "{space:?} then `>` is text");
        let text = format!("{space}{}x\n", "- ".repeat(300));
        assert!(check(&text).is_ok(), "{space:?} then `-` is text");
    }
}

/// Every ASCII whitespace character the parser accepts after a blockquote marker
/// keeps the count going, not only the space and the tab.
#[test]
fn a_marker_run_separated_by_any_ascii_whitespace_is_counted_whole() {
    for space in ['\u{C}', '\r', ' ', '\t'] {
        let text = format!("{}deep\n", format!(">{space}").repeat(300));
        let err = refused_for_nesting(&text, "300 levels");
        assert!(err.depth > MAX_DEPTH, "{space:?}: {err:?}");
    }
}

#[test]
fn the_error_names_the_line() {
    let text = format!("fine\n\n{}deep\n", "> ".repeat(300));
    let err = check(&text).expect_err("300 levels");
    assert_eq!(err.line(), 3);
    assert!(err.to_string().contains("line 3"), "{err}");
}

/// A paragraph goes on through a line that looks like a blockquote, however many `> `
/// it opens with: without a blank line before it, the line is more of the paragraph's
/// words, and nests nothing.
#[test]
fn a_quote_marker_run_after_a_paragraph_line_is_its_words() {
    let text = format!("fine\n{}deep\n", "> ".repeat(300));
    assert!(check(&text).is_ok());
    assert!(parse_on_a_long_operation_stack(text).is_ok());
}

/// The hole this guard used to have: a container opened on one line, of any
/// kind, as many times over as fits. Exactly at the ceiling each one passes, and
/// one more is refused at its line.
#[test]
fn every_container_a_line_can_open_is_counted_to_the_ceiling() {
    for marker in ONE_LINE_MARKERS {
        assert!(
            check(&one_line(marker, MAX_DEPTH)).is_ok(),
            "{marker:?} at the ceiling must pass"
        );
        let text = format!("Before.\n\n{}", one_line(marker, MAX_DEPTH + 1));
        let err = refused_for_nesting(&text, &format!("{marker:?} past the ceiling"));
        assert_eq!(err.line, 3, "{marker:?}: the refusal names the line");
        assert!(err.depth > MAX_DEPTH, "{marker:?}: {err:?}");
    }
}

/// Markers of different kinds on one line nest inside one another all the same.
#[test]
fn a_mixture_of_markers_on_one_line_adds_up() {
    let cycle = ["- ", "> ", "1. ", "[^a]: ", ": ", "(iv) ", "- [ ] "];
    let line = |levels: usize| {
        let markers: String = cycle.iter().cycle().take(levels).copied().collect();
        format!("{markers}deep\n")
    };
    assert!(check(&line(MAX_DEPTH)).is_ok());
    assert!(check(&line(MAX_DEPTH + 1)).is_err());
}

/// Nothing is counted that `jotdown` reads as text: a word ending in a full stop,
/// a bullet not followed by a space, a colon inside a word, a thematic break.
#[test]
fn text_that_only_looks_like_a_marker_is_not_counted() {
    for text in [
        format!("{}\n", "Mr. ".repeat(300)),
        format!("{}\n", "-x ".repeat(300)),
        format!("{}\n", "a:b ".repeat(300)),
        format!("{}\n", "- ".repeat(300)),
        format!("{}\n", "* ".repeat(300)),
        format!("{}\n", "[a] ".repeat(300)),
        format!("{}\n", "abc. ".repeat(300)),
        format!("{}deep\n", "[link]: ".repeat(300)),
    ] {
        assert!(check(&text).is_ok(), "should pass: {text:.40}");
    }
}

/// A list nests one level per byte of indentation when a blank line separates its
/// items, since each level strips one byte of each line it continues. The old
/// count of one level per two columns let twice the ceiling through.
#[test]
fn a_list_nested_by_one_space_a_level_is_counted_per_space() {
    let nested = |levels: usize| {
        (0..levels)
            .map(|level| format!("{}- level {level}\n", " ".repeat(level)))
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert!(check(&nested(MAX_DEPTH)).is_ok());
    let err = refused_for_nesting(&nested(MAX_DEPTH + 1), "one level past");
    assert_eq!(err.line, 2 * MAX_DEPTH + 1, "the deepest item's line");
}

/// A div opened inside a blockquote on every line of it nests one more on each,
/// since the blockquote carries every line into the div the line before opened.
#[test]
fn a_div_opened_after_a_quote_on_every_line_is_counted() {
    let text = "> ::: note\n".repeat(MAX_DEPTH);
    let err = refused_for_nesting(&text, "a div per line, inside one quote");
    assert!(err.depth > MAX_DEPTH);
}

/// Fences each one colon shorter than the one before: none of them is long
/// enough to close the div around it, so each opens one more inside it.
#[test]
fn fences_that_shorten_line_by_line_are_counted_as_opening() {
    let descending = |levels: usize| {
        (0..levels)
            .map(|i| ":".repeat(levels + 2 - i))
            .collect::<Vec<_>>()
            .join("\n")
            + "\nwords\n"
    };
    assert!(check(&descending(MAX_DEPTH)).is_ok());
    assert!(check(&descending(MAX_DEPTH + 1)).is_err());
}

/// A div that saw a code fence open ignores every fence until one closes it
/// (`jotdown`'s `nested_raw`), even when the code block itself has ended with the
/// list item it sat in. So the bare fence that follows closes nothing: it opens a
/// div inside the one before, round after round. Verified against `jotdown`: seven
/// hundred rounds abort a 2 MiB thread.
#[test]
fn a_fence_the_div_reads_as_code_does_not_close_it() {
    let text = format!("::: a\n{}", "- item\n\n  ```x\n:::\n".repeat(MAX_DEPTH));
    assert!(
        check(&text).is_err(),
        "each round opens a div inside the last"
    );
    // A code block that closes leaves the div free to close.
    let closed = "::: a\n```\ncode\n```\n:::\n".repeat(500);
    assert!(check(&closed).is_ok());
}

/// The divs a quotation held are forgotten when the quotation ends, by its own
/// closing fence or by the blank line after it.
#[test]
fn quoted_divs_are_forgotten_when_the_quotation_ends() {
    let quoted = "> ::: note\n> A line.\n> :::\n\nBetween.\n\n".repeat(500);
    assert!(check(&quoted).is_ok());
    let unclosed = "> ::: note\n> A line.\n\n".repeat(500);
    assert!(check(&unclosed).is_ok(), "a blank line ends the quotation");
}

/// The measurement behind the ceiling, kept honest: every kind of container at
/// the ceiling parses from a long operation's stack.
#[test]
fn every_kind_at_the_ceiling_parses_from_a_two_mebibyte_thread() {
    for marker in ONE_LINE_MARKERS {
        let text = one_line(marker, MAX_DEPTH);
        assert!(check(&text).is_ok(), "{marker:?}");
        let parsed = parse_on_a_long_operation_stack(text).expect("parse");
        // A definition's body is not prose (`to_plain_text` leaves footnotes and
        // link definitions out), so only the other kinds are checked for their word.
        if !marker.starts_with(['[', ':']) {
            assert!(parsed.contains("deep"), "{marker:?}: {parsed:?}");
        }
    }
    let descending = (0..MAX_DEPTH)
        .map(|i| ":".repeat(MAX_DEPTH + 2 - i))
        .collect::<Vec<_>>()
        .join("\n")
        + "\ndeep\n";
    assert!(check(&descending).is_ok());
    assert!(
        parse_on_a_long_operation_stack(descending)
            .expect("parse")
            .contains("deep")
    );
}

/// What a generated line may open with: container markers, the whitespace between
/// them, and text that only looks like one.
fn marker_token() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "- ", "* ", "+ ", "1. ", "12) ", "(3) ", "a. ", "Z) ", "(b) ", "iv. ", "(XL) ", "- [ ] ",
        "- [x] ", "[^n]: ", "[^n]:", "[l]: ", ": ", "> ", ">", ">\t", " ", "  ", "\t", "|",
        "::: c", ":::", "::::", "```", "~~~", "# ", "{.c}", "x", "Mr. ", "-x", "10:30", "***",
    ])
}

/// A line of container markers and nothing else before its word, `0..max` of them.
fn container_line(max: usize) -> impl Strategy<Value = String> {
    let marker = prop::sample::select(
        ONE_LINE_MARKERS
            .iter()
            .copied()
            .chain([">", ">\t", "  ", "\t"])
            .collect::<Vec<_>>(),
    );
    prop::collection::vec(marker, 0..max).prop_map(|markers| markers.concat() + "words")
}

/// A run of fences, each as long as the one before or one colon shorter, after the
/// same prefix, some of them carrying a class and some a code block's.
fn fence_run() -> impl Strategy<Value = String> {
    let prefix = prop::sample::select(vec!["", "> ", "> > ", "- ", "  ", "\t", ">  "]);
    let fence = prop::sample::select(vec!["", "", "", " c", "`", "```x", "~~~"]);
    (
        prefix,
        3usize..130,
        prop::collection::vec((fence, any::<bool>()), 1..140),
    )
        .prop_map(|(prefix, longest, fences)| {
            let mut len = longest;
            let mut out = Vec::new();
            for (kind, shorten) in fences {
                let line = match kind {
                    "`" => format!("{prefix}```"),
                    "```x" | "~~~" => format!("{prefix}{kind}"),
                    class => format!("{prefix}{}{class}", ":".repeat(len)),
                };
                out.push(line);
                if shorten && len > 3 {
                    len -= 1;
                }
            }
            out.join("\n")
        })
}

/// A list nested by indentation, one item a level, a blank line between them.
fn indented_list() -> impl Strategy<Value = String> {
    let marker = prop::sample::select(vec!["- ", "1. ", "[^a]: ", ": ", "> - "]);
    (marker, 1usize..3, 0usize..150).prop_map(|(marker, step, levels)| {
        (0..levels)
            .map(|level| format!("{}{marker}level {level}", " ".repeat(step * level)))
            .collect::<Vec<_>>()
            .join("\n\n")
    })
}

/// A list whose items step in between paragraph lines, from a few levels to past the
/// parser's limit, in a quotation or out of one.
///
/// It starts after a blank line. Without one, a paragraph line before it reads every
/// line of it as more of that paragraph (none of them is blank outside the quotation),
/// and a paragraph of hundreds of lines meets a different limit of the parser: its
/// inline pass recurses once per line that an emphasis or a quote left open spans,
/// which [`MAX_HELD_LINES`] bounds rather than the nesting ceiling.
fn lazy_list() -> impl Strategy<Value = String> {
    let marker = prop::sample::select(vec!["- ", "1. ", "[^a]: ", ": ", "- [ ] "]);
    let prefix = prop::sample::select(vec!["", "> ", "> > "]);
    (marker, prefix, 1usize..PAST_THE_PARSERS_LIMIT).prop_map(|(marker, prefix, levels)| {
        format!("\n{}", lazy_staircase(marker, levels, prefix))
    })
}

/// A document of lines built from markers and fences, as deep in places as the
/// profile it is drawn from allows: some well within the ceiling, some just either
/// side of it, and some far enough past it to reach the parser's limit if the guard
/// let them through.
fn marker_soup() -> impl Strategy<Value = String> {
    prop_oneof![
        Just(48usize),
        Just(MAX_DEPTH),
        Just(MAX_DEPTH + 8),
        Just(4 * MAX_DEPTH)
    ]
    .prop_flat_map(|max| {
        let mixed =
            prop::collection::vec(marker_token(), 0..160).prop_map(|tokens| tokens.concat());
        let repeated = (container_line(max), 1usize..8).prop_map(|(line, times)| {
            std::iter::repeat_n(line, times)
                .collect::<Vec<_>>()
                .join("\n")
        });
        let group = prop_oneof![
            3 => container_line(max),
            2 => mixed,
            2 => fence_run(),
            1 => indented_list(),
            1 => lazy_list(),
            1 => repeated,
            1 => Just(String::new()),
        ];
        prop::collection::vec(group, 1..12).prop_map(|groups| groups.join("\n") + " words\n")
    })
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 256, ..ProptestConfig::default() })]

    /// One line of `levels` container markers, each of which opens the next one's
    /// line, nests exactly `levels` deep: accepted exactly when that is within the
    /// ceiling, and refused at its line otherwise.
    #[test]
    fn a_line_of_container_markers_is_refused_exactly_past_the_ceiling(
        markers in prop::collection::vec(
            prop::sample::select(ONE_LINE_MARKERS.to_vec()), 1..(2 * MAX_DEPTH)),
        before in 0usize..3,
    ) {
        let levels = markers.len();
        let text = format!("{}{}deep\n", "Before.\n\n".repeat(before), markers.concat());
        match check(&text) {
            Ok(()) => prop_assert!(levels <= MAX_DEPTH, "{levels} levels passed"),
            Err(refused) => {
                prop_assert!(levels > MAX_DEPTH, "{} levels refused: {:?}", levels, refused);
                prop_assert_eq!(refused.line(), 2 * before + 1);
                prop_assert!(refused.too_deep().is_some(), "{:?}", refused);
            }
        }
    }

    /// Whatever the guard lets through parses from a long operation's stack. The
    /// assertion is the parse coming back at all: a guard that let a document past
    /// the parser's limit through would abort the test binary here.
    #[test]
    fn whatever_the_guard_accepts_parses_from_a_two_mebibyte_thread(text in marker_soup()) {
        if check(&text).is_ok() {
            let parsed = parse_on_a_long_operation_stack(text);
            prop_assert!(parsed.is_ok(), "{:?}", parsed);
        }
    }

    /// A refusal always names a line the text has, and a limit the text passes there:
    /// the nesting, or, when the soup reads as one long paragraph (a line with no space
    /// after its `>` starts one, and every line after it goes on with it), the lines it
    /// holds open, as deep as the parser's calls would go by that line.
    #[test]
    fn a_refusal_names_a_line_of_the_text(text in marker_soup()) {
        if let Err(refused) = check(&text) {
            prop_assert!(refused.line() >= 1);
            prop_assert!(refused.line() <= text.split('\n').count());
            match &refused {
                DjotRefusal::TooDeep(deep) => prop_assert!(deep.depth > MAX_DEPTH),
                DjotRefusal::HeldOpenTooLong { line } => {
                    let to_the_line = text.split('\n').take(*line).collect::<Vec<_>>().join("\n");
                    let unbounded = crate::djot_inline::Limits {
                        max_held: usize::MAX,
                        ..crate::djot_inline::Limits::default()
                    };
                    let held = scan(&to_the_line, unbounded).map(|cost| cost.held_lines);
                    prop_assert!(
                        matches!(held, Ok(lines) if lines > MAX_HELD_LINES),
                        "{:?}: {:?}", refused, held
                    );
                }
                other => prop_assert!(false, "{:?}", other),
            }
        }
    }
}

// ── What the editor stores, a load accepts ──────────────────────────────────────────

/// One piece of what a writer can type: a word, a run of spaces or tabs of any length,
/// any character a Djot marker is made of, alone or as a marker, or repeated in a long
/// run, and the break between two paragraphs.
fn typed_piece() -> impl Strategy<Value = String> {
    let blank = prop::sample::select(vec![" ", "\t", " \t", "\t ", "\r", "\u{A0}", "\u{3000}"]);
    let marker = prop::sample::select(vec![
        "-", "*", "+", ">", ":", "|", "[", "]", "^", "(", ")", ".", "#", "{", "}", "`", "~", "=",
        "_", "!", "\\", "\"", "'", "<", "&", "$", "%", "1.", "a)", "(iv)", "XII.", "- ", "* ",
        "+ ", "> ", ": ", "1. ", "[^a]: ", "[^a]:", "[l]: ", "- [ ] ", "* [x] ", "::: c", ":::",
        "```", "~~~", "| a |", "* * *", "{.c}", "# ", "10:30:45", "Mr. ",
    ]);
    prop_oneof![
        3 => "[A-Za-z]{1,9}",
        3 => (blank.clone(), 1usize..600).prop_map(|(blank, n)| blank.repeat(n)),
        3 => marker.clone().prop_map(str::to_string),
        1 => (marker, 2usize..300).prop_map(|(marker, n)| marker.repeat(n)),
        1 => (blank, 1usize..300, "[a-z]{1,6}")
            .prop_map(|(blank, n, word)| format!("{}{word}", blank.repeat(n))),
        1 => Just("\n".to_string()),
    ]
}

/// A stretch of typing: paragraphs, and whatever their lines open with.
fn typed_text() -> impl Strategy<Value = String> {
    prop::collection::vec(typed_piece(), 0..16).prop_map(|pieces| pieces.concat())
}

/// One paragraph of a document the writer edited, and how they shaped it.
#[derive(Debug, Clone)]
enum Edit {
    /// Typed as it stands.
    Typed(String),
    /// Typed with a character format on: bold, italic, underlined or struck through.
    Formatted(String, u8),
    /// Made a list item, then Tabbed `depth` levels in.
    Listed(String, ListStyle, u8),
    /// Made a quotation, `depth` levels deep, as Tab inside one nests it.
    Quoted(String, u8),
    /// A footnote's reference at the caret, then the typing after it.
    Noted(String),
}

fn edit() -> impl Strategy<Value = Edit> {
    let style = prop::sample::select(vec![
        ListStyle::Disc,
        ListStyle::Decimal,
        ListStyle::LowerAlpha,
        ListStyle::UpperRoman,
    ]);
    prop_oneof![
        2 => typed_text().prop_map(Edit::Typed),
        1 => (typed_text(), 0u8..4).prop_map(|(text, format)| Edit::Formatted(text, format)),
        1 => (typed_text(), style, 0u8..6).prop_map(|(text, style, depth)| {
            Edit::Listed(text, style, depth)
        }),
        1 => (typed_text(), 1u8..4).prop_map(|(text, depth)| Edit::Quoted(text, depth)),
        1 => typed_text().prop_map(Edit::Noted),
    ]
}

/// Type `text` at the caret as the editor takes it in, from the keyboard or a paste:
/// every line break, `\r\n`, `\r` or `\n`, a new paragraph (`teksilo`'s
/// `insert_multiline_plain`, and Enter), and the rest inserted as it stands. The editor
/// never puts a line break inside a paragraph.
fn type_at(cursor: &text_document::TextCursor, text: &str) {
    let text = text.replace("\r\n", "\n").replace('\r', "\n");
    for (index, line) in text.split('\n').enumerate() {
        if index > 0 {
            let _ = cursor.insert_block();
        }
        if !line.is_empty() {
            let _ = cursor.insert_text(line);
        }
    }
}

/// What `edits` store as, made the way the editor makes them: each one a paragraph
/// after the last, through the cursor calls the editor's keys and menus make, and
/// written with the Djot writer every text surface saves through (prose, a comment, a
/// reply, a footnote, a note template). A call the editor would refuse in that place
/// is refused here too, and ignored, as the editor ignores it.
fn edited_djot(edits: &[Edit]) -> String {
    use text_document::{ListFormat, MoveMode, MoveOperation, TextFormat};
    let doc = TextDocument::new();
    let cursor = doc.cursor();
    for (index, edit) in edits.iter().enumerate() {
        cursor.move_position(MoveOperation::End, MoveMode::MoveAnchor, 1);
        if index > 0 {
            let _ = cursor.insert_block();
        }
        match edit {
            Edit::Typed(text) => type_at(&cursor, text),
            Edit::Formatted(text, format) => {
                // Typed, then selected and formatted, as Ctrl+B and its siblings do.
                let start = cursor.position();
                type_at(&cursor, text);
                cursor.set_position(start, MoveMode::KeepAnchor);
                let format = TextFormat {
                    font_bold: Some(*format == 0),
                    font_italic: Some(*format == 1),
                    font_underline: Some(*format == 2),
                    font_strikeout: Some(*format == 3),
                    ..TextFormat::default()
                };
                let _ = cursor.merge_char_format(&format);
                cursor.move_position(MoveOperation::End, MoveMode::MoveAnchor, 1);
            }
            Edit::Listed(text, style, depth) => {
                type_at(&cursor, text);
                let _ = cursor.create_list(style.clone());
                // Tab in a list item, as `teksilo`'s editor answers it: the item leaves
                // its list for a new one a level further in.
                for level in 1..=*depth {
                    let _ = cursor.remove_current_block_from_list();
                    let _ = cursor.create_list(style.clone());
                    let _ = cursor.set_current_list_format(&ListFormat {
                        indent: Some(level),
                        ..ListFormat::default()
                    });
                }
            }
            Edit::Quoted(text, depth) => {
                type_at(&cursor, text);
                for _ in 0..*depth {
                    let _ = cursor.increase_blockquote_depth();
                }
            }
            Edit::Noted(text) => {
                // What Insert footnote puts at the caret (`FootnotesViewModel::insert_at`).
                let _ = cursor.insert_djot("[^fn1]");
                cursor.move_position(MoveOperation::End, MoveMode::MoveAnchor, 1);
                type_at(&cursor, text);
            }
        }
    }
    doc.to_djot()
        .expect("the editor writes its document as Djot")
}

/// What the editor stores for `text` set as a document's whole text: what a scene's
/// prose, a comment's body or a footnote's is, once typed.
fn typed_djot(text: &str) -> String {
    let doc = TextDocument::new();
    doc.set_plain_text(text).expect("set the typed text");
    doc.to_djot()
        .expect("the editor writes its document as Djot")
}

/// Accepted by the load, and parsed from a long operation's stack.
fn a_load_accepts(djot: String) -> Result<(), TestCaseError> {
    if let Err(refused) = check(&djot) {
        return Err(TestCaseError::fail(format!(
            "the editor stored {:.120?}, and the load refuses it: {refused}",
            djot
        )));
    }
    let parsed = parse_on_a_long_operation_stack(djot);
    prop_assert!(parsed.is_ok(), "{:?}", parsed);
    Ok(())
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 96, ..ProptestConfig::default() })]

    /// Whatever a writer types, the Djot the editor stores for it is accepted by the next
    /// load and parses from a long operation's stack. A paragraph typed after two hundred
    /// spaces or tabs used to be stored as written and refused on the next load, which
    /// then would not open the project at all.
    #[test]
    fn whatever_the_editor_stores_for_typing_a_load_accepts(text in typed_text()) {
        a_load_accepts(typed_djot(&text))?;
    }

    /// The same of a document shaped as well as typed: formatting, a nested list, a
    /// nested quotation and a footnote's reference, each with typing of any shape in it,
    /// and the body of a comment typed into its card beside it.
    #[test]
    fn whatever_the_editor_stores_for_a_shaped_document_a_load_accepts(
        edits in prop::collection::vec(edit(), 1..7),
        remark in prop::collection::vec(edit(), 1..3),
    ) {
        a_load_accepts(edited_djot(&edits))?;
        a_load_accepts(edited_djot(&remark))?;
    }
}

// ── Deep nesting the editor writes loads at its real depth ───────────────────────────

/// A list the editor Tabbed `levels` deep, its items stepping one level a line the way
/// the editor writes them (two spaces a level). This is the shape the old guard
/// over-counted: at 48 levels it wrote 96 leading spaces and was refused, though
/// jotdown nests it 48.
fn editor_list_chain(levels: usize) -> String {
    let edits: Vec<Edit> = (0..levels)
        .map(|d| Edit::Listed(format!("level {d}"), ListStyle::Disc, d as u8))
        .collect();
    edited_djot(&edits)
}

/// Whatever the editor writes for a list nested N levels, the guard counts at N (not at
/// twice it), so a chain at the ceiling loads and parses; a chain past the ceiling is
/// refused. The property the fix exists for.
#[test]
fn what_the_editor_writes_for_a_nested_list_is_counted_at_its_real_depth() {
    for levels in [1, 10, 48, MAX_DEPTH] {
        let djot = editor_list_chain(levels);
        assert!(
            check(&djot).is_ok(),
            "a {levels}-level editor list must load: the count must follow jotdown, not the byte indent\n{djot:.200}"
        );
        assert!(
            parse_on_a_long_operation_stack(djot).is_ok(),
            "a {levels}-level editor list must parse from a long operation's stack"
        );
    }
    // A chain deep enough to reach the ceiling is still refused.
    assert!(check(&editor_list_chain(MAX_DEPTH + 8)).is_err());
    assert!(check(&editor_list_chain(MAX_DEPTH + 24)).is_err());
}

/// The same for quotes: the editor writes N nested blockquotes as N `>` markers, which
/// the guard has always counted per marker, so a quote at the ceiling loads and parses
/// and one past it is refused. Kept beside the list case so the two shapes the property
/// test names travel together.
#[test]
fn what_the_editor_writes_for_a_nested_quote_is_counted_at_its_real_depth() {
    let quoted = |depth: u8| edited_djot(&[Edit::Quoted("deep".to_string(), depth)]);
    for depth in [1u8, 10, 48, MAX_DEPTH as u8] {
        let djot = quoted(depth);
        assert!(
            check(&djot).is_ok(),
            "a {depth}-level quote must load: {djot:.120}"
        );
        assert!(parse_on_a_long_operation_stack(djot).is_ok());
    }
    assert!(check(&quoted((MAX_DEPTH + 8) as u8)).is_err());
}

proptest! {
    // Each case builds a text-document through the cursor (work quadratic in the depth)
    // and may parse it on a spawned thread, so the case count is kept low.
    #![proptest_config(ProptestConfig { cases: 12, ..ProptestConfig::default() })]

    /// Whatever the editor writes for a list Tabbed N levels deep, the guard's count
    /// is within a small constant of N — never twice it — and, when it accepts, the
    /// output parses from a long operation's stack. `N` ranges either side of the
    /// ceiling: below it must load, well past it must be refused, and around it the
    /// count tracks N rather than the byte indent.
    #[test]
    fn the_guard_counts_a_nested_list_within_a_constant_of_its_depth(levels in 1usize..(MAX_DEPTH + 24)) {
        let djot = editor_list_chain(levels);
        match check(&djot) {
            Ok(()) => {
                // Accepted only when the real depth is within the ceiling: never the
                // old 2x over-count that refused a list half this deep.
                prop_assert!(levels <= MAX_DEPTH + 1, "{levels} accepted");
                prop_assert!(parse_on_a_long_operation_stack(djot).is_ok());
            }
            Err(refused) => {
                // Refused only when the depth genuinely reaches the ceiling — within a
                // small constant of `levels`, not half of it.
                let depth = refused.too_deep().map_or(0, |deep| deep.depth);
                prop_assert!(depth > MAX_DEPTH, "{:?}", refused);
                prop_assert!(levels + 2 >= depth, "counted {} for {levels} levels", depth);
            }
        }
    }
}

// ── Lines that nest nothing are never refused ────────────────────────────────────────

/// Lines a writer can put in a code block, as many as they like on one line, that
/// open containers anywhere else.
fn marker_lines() -> Vec<String> {
    vec![
        ">".repeat(100),
        "> ".repeat(100),
        ">> ".repeat(100),
        format!("{}end", "+ ".repeat(100)),
        format!("{}x", "- ".repeat(100)),
        "1. ".repeat(120),
        ": ".repeat(100),
        "[^a]: ".repeat(100),
        "::: note ".repeat(100),
    ]
}

/// A code block keeps its lines as they were written, and the parser reads them as its
/// text: however many markers a line of one opens with, it nests nothing. The marker
/// count this guard used to refuse on read them as containers, and past 96 of them
/// locked the writer out of a project whose code block the editor had saved.
#[test]
fn a_code_blocks_lines_nest_nothing() {
    for line in marker_lines() {
        for fence in ["```", "~~~", "`````", "```rust"] {
            let close: String = fence
                .chars()
                .take_while(|c| matches!(c, '`' | '~'))
                .collect();
            let text = format!("Before.\n\n{fence}\ncode\n{line}\n{close}\n\nAfter.\n");
            assert!(check(&text).is_ok(), "{fence} {line:.20}");
            assert!(parse_on_a_long_operation_stack(text).is_ok());
        }
    }
}

/// A paragraph goes on through every line that is not blank, and its later lines are
/// its words whatever they open with. Skribisto 3.0.4 saved a preformatted paste as one
/// paragraph with its line breaks kept (its `text-document`, 1.12.2, wrote it so), so a
/// pasted line of markers in a project it saved is one of these.
#[test]
fn a_paragraphs_later_lines_nest_nothing() {
    for line in marker_lines() {
        let text = format!("start\n{line}\nend\n");
        assert!(check(&text).is_ok(), "{line:.20}");
        assert!(parse_on_a_long_operation_stack(text).is_ok());
    }
}

/// What the editor stores for a code block that holds a line of markers, typed or
/// imported, and for a preformatted paste of one, loads again, markers and all.
#[test]
fn the_editors_code_block_and_preformatted_paste_load_again() {
    use text_document::{MoveMode, MoveOperation};
    for line in marker_lines() {
        let doc = TextDocument::new();
        doc.set_markdown(&format!(
            "Before.\n\n```\ncode\n{}\n```\n\nAfter.\n",
            line.trim_end()
        ))
        .and_then(|operation| operation.wait())
        .expect("the editor takes the Markdown");
        let imported = doc
            .to_djot()
            .expect("the editor writes its document as Djot");
        assert!(check(&imported).is_ok(), "{imported:.200}");
        assert!(parse_on_a_long_operation_stack(imported).is_ok());

        let doc = TextDocument::new();
        doc.set_plain_text("Before.").expect("typed");
        let cursor = doc.cursor();
        cursor.move_position(MoveOperation::End, MoveMode::MoveAnchor, 1);
        let _ = cursor.insert_block();
        cursor
            .insert_html(&format!("<pre>start\n{line}\nend</pre>"))
            .expect("the editor takes the paste");
        let pasted = doc
            .to_djot()
            .expect("the editor writes its document as Djot");
        assert!(check(&pasted).is_ok(), "{pasted:.200}");
        assert!(parse_on_a_long_operation_stack(pasted).is_ok());
    }
}

// ── A heading the parser cannot count ────────────────────────────────────────────────

/// A heading of 65,536 `#` panics the parser, which stores its level in 16 bits; the
/// deepest it can store loads and parses. Refused wherever the heading sits: alone, in a
/// quotation, in a list item, after a paragraph's blank line.
#[test]
fn a_heading_deeper_than_the_parser_counts_is_refused() {
    let at = "#".repeat(MAX_HEADING_LEVEL);
    let past = "#".repeat(MAX_HEADING_LEVEL + 1);
    for (prefix, line) in [("", 1), ("> ", 1), ("- ", 1), ("Words.\n\n", 3)] {
        let text = format!("{prefix}{past} Zeus\n");
        match check(&text) {
            Err(DjotRefusal::HeadingTooDeep { level, line: at }) => {
                assert_eq!(level, MAX_HEADING_LEVEL + 1, "{prefix:?}");
                assert_eq!(at, line, "{prefix:?}");
            }
            other => panic!("{prefix:?}: {other:?}"),
        }
        let text = format!("{prefix}{at} Zeus\n");
        assert!(check(&text).is_ok(), "{prefix:?}");
        assert!(parse_on_a_long_operation_stack(text).is_ok(), "{prefix:?}");
    }
    // Only a heading is one: the same run followed by a word is a paragraph's text, and
    // on a paragraph's later line it is more of the paragraph.
    for text in [
        format!("{past}x\n"),
        format!("words\n{past} Zeus\n"),
        format!("```\n{past} Zeus\n```\n"),
    ] {
        assert!(check(&text).is_ok(), "{text:.40}");
        assert!(parse_on_a_long_operation_stack(text).is_ok());
    }
}

/// The refusal says what it refused, and where.
#[test]
fn the_heading_refusal_names_the_heading_and_its_line() {
    let text = format!("Words.\n\n{} Zeus\n", "#".repeat(70_000));
    let refused = check(&text).expect_err("a heading of 70,000 levels");
    assert_eq!(refused.line(), 3);
    let said = refused.to_string();
    assert!(said.contains("70000") && said.contains("line 3"), "{said}");
}

/// The parser also stores, in 16 bits, how many containers are open around a list,
/// sections included, and each heading outside every container opens a section inside
/// the ones of lower levels. The ceiling leaves room for the document, the list and
/// every container a list item may sit in.
#[test]
fn the_sections_ceiling_leaves_the_parser_room_for_a_list() {
    let open_at_a_list = 1 + MAX_SECTIONS + (MAX_DEPTH - 1) + 1;
    assert_eq!(open_at_a_list, usize::from(u16::MAX));
}

/// Sections are counted as the parser opens and closes them: one per heading deeper
/// than every open one, a heading closing the sections of its own level and deeper, and
/// a heading inside a container opening none.
#[test]
fn sections_are_counted_as_the_parser_opens_them() {
    let sections = |text: &str| {
        let mut nesting = crate::djot_nesting::Nesting::default();
        text.split_inclusive('\n')
            .map(|line| nesting.read_line(line, MAX_DEPTH).sections)
            .last()
            .unwrap_or(0)
    };
    let chain: String = (1..=50)
        .map(|level| format!("{} h\n\n", "#".repeat(level)))
        .collect();
    assert_eq!(sections(&chain), 50);
    assert_eq!(sections(&format!("{chain}### back up\n")), 3);
    assert_eq!(sections(&format!("{chain}> ## quoted\n")), 50);
    assert_eq!(sections(&format!("{chain}- # listed\n")), 50);
}

// ── Joined, not refused ──────────────────────────────────────────────────────────────

/// How `text-document` reads `djot`, as its own Djot and its plain text, parsed on a stack
/// deep enough for any paragraph here: the reading a load must leave as it was. `None`
/// when `text-document` refuses it.
pub(crate) fn reading(djot: String) -> Option<(String, String)> {
    std::thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(move || {
            let doc = TextDocument::new();
            doc.set_djot_sync(&djot).ok()?;
            Some((doc.to_djot().ok()?, doc.to_plain_text().ok()?))
        })
        .expect("spawn the parse thread")
        .join()
        .expect("the parse must not unwind")
}

/// What the editor stores after `html` is pasted as a paragraph of its own below the
/// words "Epigraph:", and `format`, if any, applied to all of it afterwards.
fn pasted_passage(html: &str, format: Option<text_document::TextFormat>) -> String {
    use text_document::{MoveMode, MoveOperation};
    let doc = TextDocument::new();
    doc.set_plain_text("Epigraph:").expect("typed");
    let cursor = doc.cursor_at(doc.character_count());
    cursor.insert_block().expect("a new paragraph");
    let start = doc.character_count();
    cursor
        .insert_html(html)
        .expect("the editor takes the paste");
    if let Some(format) = format {
        let cursor = doc.cursor_at(start);
        cursor.move_position(MoveOperation::End, MoveMode::KeepAnchor, 1);
        cursor
            .merge_char_format(&format)
            .expect("the editor formats the passage");
    }
    doc.to_djot()
        .expect("the editor writes its document as Djot")
}

/// Lines of a pasted passage, one per line of its `<pre>`.
fn verses(lines: usize) -> String {
    (0..lines)
        .map(|i| format!("verse {i} of the poem\n"))
        .collect()
}

/// Every way the editor formats a pasted preformatted passage from end to end: a
/// format applied after the paste, one the paste carried in, and a passage pasted into
/// a quotation or a list. What the editor writes for each, one paragraph per line since
/// `text-document` 1.12.3.
pub(crate) fn formatted_passages(lines: usize) -> Vec<(&'static str, String)> {
    use text_document::{CharVerticalAlignment, TextFormat};
    let pre = format!("<pre>{}</pre>", verses(lines));
    let after = |format: TextFormat| pasted_passage(&pre, Some(format));
    vec![
        (
            "italic",
            after(TextFormat {
                font_italic: Some(true),
                ..TextFormat::default()
            }),
        ),
        (
            "bold",
            after(TextFormat {
                font_bold: Some(true),
                ..TextFormat::default()
            }),
        ),
        (
            "underlined",
            after(TextFormat {
                font_underline: Some(true),
                ..TextFormat::default()
            }),
        ),
        (
            "struck out",
            after(TextFormat {
                font_strikeout: Some(true),
                ..TextFormat::default()
            }),
        ),
        (
            "a link",
            after(TextFormat {
                anchor_href: Some("https://example.com".to_string()),
                is_anchor: Some(true),
                ..TextFormat::default()
            }),
        ),
        (
            "superscript",
            after(TextFormat {
                vertical_alignment: Some(CharVerticalAlignment::SuperScript),
                ..TextFormat::default()
            }),
        ),
        (
            "pasted in italics",
            pasted_passage(&format!("<pre><em>{}</em></pre>", verses(lines)), None),
        ),
        (
            "pasted in italics into a quotation",
            pasted_passage(
                &format!(
                    "<blockquote><pre><em>{}</em></pre></blockquote>",
                    verses(lines)
                ),
                None,
            ),
        ),
        (
            "pasted in italics into a list",
            pasted_passage(
                &format!("<ul><li><pre><em>{}</em></pre></li></ul>", verses(lines)),
                None,
            ),
        ),
    ]
}

/// What Skribisto 3.0.4 saved for a preformatted passage of `lines` lines pasted and
/// formatted from end to end: one paragraph over all of them, the format opened on the
/// first line and closed after the last, so every line is held open. Its `text-document`,
/// 1.12.2, wrote every way [`formatted_passages`] formats a passage in exactly this shape,
/// and one pasted into a quotation or a list the same as one pasted in italics. Built here
/// rather than through the editor, which writes the passage one paragraph per line from
/// 1.12.3 on; projects saved before hold this shape, and the load still meets it.
pub(crate) fn formatted_passages_saved_by_3_0_4(lines: usize) -> Vec<(&'static str, String)> {
    let held = |open: &str, close: &str| format!("Epigraph:\n\n{open}{}{close}", verses(lines));
    vec![
        ("italic", held("_", "_")),
        ("bold", held("*", "*")),
        ("underlined", held("{+", "+}")),
        ("struck out", held("{-", "-}")),
        ("a link", held("[", "](https://example.com)")),
        ("superscript", held("^", "^")),
    ]
}

/// A preformatted passage Skribisto 3.0.4 saved, pasted and formatted from end to end, is
/// one paragraph over all its lines, every one of them held open. Two hundred lines of it
/// load as they are; six hundred pass the ceiling, and the load joins them into one line
/// instead of refusing the project, the editor reading the joined paragraph exactly as the
/// one saved. Before, 129 lines of it were refused, and the project with them.
#[test]
fn a_pasted_passage_saved_formatted_across_its_lines_opens_as_it_was_written() {
    for (name, djot) in formatted_passages_saved_by_3_0_4(200) {
        assert_eq!(check(&djot), Ok(()), "{name}, 200 lines");
        assert_eq!(admit(djot.clone()).as_ref(), Ok(&djot), "{name}, 200 lines");
    }
    for (name, djot) in formatted_passages_saved_by_3_0_4(600) {
        assert!(
            matches!(check(&djot), Err(DjotRefusal::HeldOpenTooLong { .. })),
            "{name}: {:?}",
            check(&djot)
        );
        let joined = match admit(djot.clone()) {
            Ok(joined) => joined,
            Err(refused) => panic!("{name}: the load refused what 3.0.4 saved: {refused}"),
        };
        assert_ne!(joined, djot, "{name}");
        assert_eq!(check(&joined), Ok(()), "{name}");
        assert!(joined.lines().count() < 10, "{name}: {joined:.300}");
        let before = reading(djot);
        assert!(before.is_some(), "{name}");
        assert_eq!(
            reading(joined.clone()),
            before,
            "{name}: the editor reads it as it was"
        );
        assert!(parse_on_a_long_operation_stack(joined).is_ok(), "{name}");
    }
}

/// From `text-document` 1.12.3 the editor writes a pasted preformatted passage one
/// paragraph per line, in every way it is formatted from end to end, each line's format
/// closed on its own line. Nothing is held open past a line, so six hundred lines load as
/// the editor wrote them, and read back line for line.
#[test]
fn a_pasted_passage_is_written_as_its_lines_and_opens_as_they_are() {
    let lines: Vec<String> = std::iter::once("Epigraph:".to_string())
        .chain(verses(600).lines().map(str::to_string))
        .collect();
    let code = pasted_passage(&format!("<pre><code>{}</code></pre>", verses(600)), None);
    let passages = formatted_passages(600)
        .into_iter()
        .chain(std::iter::once(("pasted as code", code)));
    for (name, djot) in passages {
        assert_eq!(check(&djot), Ok(()), "{name}");
        assert_eq!(admit(djot.clone()).as_ref(), Ok(&djot), "{name}");
        let Some((_, shown)) = reading(djot) else {
            panic!("{name}: the editor reads what it wrote");
        };
        assert_eq!(shown.lines().collect::<Vec<_>>(), lines, "{name}");
    }
}

/// A passage Skribisto 3.0.4 saved after it was pasted as code, or in a monospaced font,
/// is one code span over all its lines, and the parser keeps its line breaks. Up to the
/// ceiling it loads as it is. Past it, those line breaks are the only ones there are to
/// join: the load writes them as spaces, keeping every word and the code formatting,
/// rather than refusing the project. Built directly, as its `text-document` wrote it: the
/// editor now writes such a passage one code span per line
/// ([`a_pasted_passage_is_written_as_its_lines_and_opens_as_they_are`]).
#[test]
fn a_code_span_over_a_pasted_passage_runs_on_rather_than_being_refused() {
    let code = |lines: usize| format!("Epigraph:\n\n`{}`", verses(lines));
    let within = code(400);
    assert_eq!(admit(within.clone()).as_ref(), Ok(&within));

    let past = code(600);
    assert!(matches!(
        check(&past),
        Err(DjotRefusal::HeldOpenTooLong { .. })
    ));
    let joined = match admit(past.clone()) {
        Ok(joined) => joined,
        Err(refused) => panic!("the load refused what 3.0.4 saved: {refused}"),
    };
    assert_eq!(check(&joined), Ok(()));
    let Some((_, before)) = reading(past) else {
        panic!("the editor reads what 3.0.4 saved");
    };
    let Some((_, after)) = reading(joined.clone()) else {
        panic!("the editor reads the joined passage");
    };
    let Some((first, passage)) = before.split_once('\n') else {
        panic!("two paragraphs: {before:.200}");
    };
    // As saved, the passage reads as its six hundred lines; joined, as one line holding
    // every one of them, run on. The line break closing the last line becomes a space at
    // the end of the code, where it changes nothing a reader sees, and `text-document`
    // 1.12.3 reads no line after it in the saved form, so the ends are left out of it.
    assert_eq!(passage.lines().count(), 600, "{passage:.200}");
    assert_eq!(
        after.trim_end(),
        format!("{first}\n{}", passage.replace('\n', " ").trim_end())
    );
    assert!(parse_on_a_long_operation_stack(joined).is_ok());
}

/// A project saved by Skribisto 3.0.4, whose `text-document` wrote a straight quotation
/// mark as it was and kept the line breaks of a plain-text paste: one quotation mark
/// nothing closes holds every line after it. It opened in 3.0.4, and opens again, read
/// as it was then.
#[test]
fn a_project_saved_by_3_0_4_with_an_unclosed_quotation_mark_opens() {
    for lines in [130, 600, 4_000] {
        let djot = format!(
            "Before.\n\nHe said \"let us begin\n{}and that was all.\n",
            "and then the next line of the paste\n".repeat(lines)
        );
        let joined = match admit(djot.clone()) {
            Ok(joined) => joined,
            Err(refused) => panic!("{lines} lines: {refused}"),
        };
        assert_eq!(check(&joined), Ok(()), "{lines} lines");
        if lines <= MAX_HELD_LINES {
            assert_eq!(joined, djot, "{lines} lines load as they are");
        }
        assert_eq!(reading(joined), reading(djot), "{lines} lines");
    }
}

/// Joining never makes a line read as another kind of block: a paragraph opening with a
/// `|` and ending, hundreds of lines later, with one would join into a table row. That
/// paragraph is refused as it was, and the joins that change nothing are all it is
/// offered.
#[test]
fn a_join_that_would_turn_a_paragraph_into_a_table_row_is_not_made() {
    let text = format!("| \"a quotation\n{}the end |\n", "and more\n".repeat(600));
    let Some(joins) = joins_in(&text, MAX_HELD_LINES + 1) else {
        panic!("nothing but the lines held stands in the way");
    };
    assert_eq!(joins.len(), 601);
    assert_eq!(join_lines(&text, &joins), None);
    assert!(matches!(
        admit(text),
        Err(DjotRefusal::HeldOpenTooLong { line }) if line == MAX_HELD_LINES + 2
    ));
}

/// What joining cannot shorten is refused as before: a link's destination, which the
/// parser rebuilds from its lines without their breaks; a heading's lines, each of which
/// states the heading again; and lines opening with an attribute set, which a space would
/// attach to the word before it.
#[test]
fn breaks_the_parser_reads_as_something_else_are_left_and_refused() {
    for (name, text) in [
        (
            "a link's destination",
            format!("[a link](https://example.com/\n{}", "more/\n".repeat(600)),
        ),
        (
            "a heading",
            format!("# \"She said\n{}", "# and went on\n".repeat(600)),
        ),
        (
            "lines opening with attributes",
            format!("\"She said\n{}", "{.aside} and went on\n".repeat(600)),
        ),
    ] {
        assert!(
            matches!(admit(text), Err(DjotRefusal::HeldOpenTooLong { .. })),
            "{name}"
        );
    }
    let refused = DjotRefusal::HeldOpenTooLong { line: 600 }.to_string();
    assert!(refused.contains("cannot be joined"), "{refused}");
    assert!(!refused.contains("crash"), "{refused}");
}

/// When joining brings the lines held within the ceiling but the text passes another
/// limit further on, the refusal names that one, at its own line.
#[test]
fn past_the_joined_lines_the_next_limit_is_the_one_refused() {
    let text = format!(
        "\"She said\n{}\n{}",
        "and went on\n".repeat(600),
        one_line("- ", MAX_DEPTH + 1)
    );
    assert!(matches!(
        check(&text),
        Err(DjotRefusal::HeldOpenTooLong { .. })
    ));
    match admit(text) {
        Err(DjotRefusal::TooDeep(deep)) => assert_eq!(deep.line, 603),
        other => panic!("{other:?}"),
    }
}

/// The shapes [`joined_by_the_load`] lists are admitted, and parse from a long
/// operation's stack; those it says are read as they were are.
#[test]
fn every_joinable_shape_is_admitted_and_parses() {
    for (name, text, exact) in joined_by_the_load() {
        assert!(check(&text).is_err(), "{name}");
        let joined = match admit(text.clone()) {
            Ok(joined) => joined,
            Err(refused) => panic!("{name}: {refused}"),
        };
        if exact {
            assert_eq!(reading(joined.clone()), reading(text), "{name}");
        }
        assert!(parse_on_a_long_operation_stack(joined).is_ok(), "{name}");
    }
}

/// One piece of a line of words: every character that opens, closes or escapes
/// something inline, alone and in the pairs the lexer reads as one, and plain words and
/// whitespace between them.
fn inline_piece() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "word",
        "a",
        "Zeus",
        " ",
        "  ",
        "\t",
        "_",
        "*",
        "\"",
        "'",
        "[",
        "]",
        "(",
        ")",
        "{",
        "}",
        "{_",
        "_}",
        "{*",
        "*}",
        "{+",
        "+}",
        "{-",
        "-}",
        "{=",
        "=}",
        "{^",
        "^}",
        "{~",
        "~}",
        "^",
        "~",
        "`",
        "``",
        "\\",
        "\\\\",
        "-",
        "--",
        "---",
        "...",
        "<",
        ">",
        ":",
        "|",
        "!",
        "![",
        "$",
        "=",
        "+",
        "#",
        "%",
        ".c",
        "#i",
        "a=b",
        "https://example.com",
        "<https://example.com>",
        ":smile:",
        "[^1]",
        "](u)",
        "][r]",
        "][]",
        "{.c}",
        "{#i}",
        "{a=\"b\"}",
        "{%c%}",
        "`x`{=html}",
    ])
}

/// A paragraph of one to six lines of pieces, in a container: none, a quotation, a list
/// item (its later lines indented or lazy), a footnote or a div.
fn paragraph_in_a_container() -> impl Strategy<Value = String> {
    let line = prop::collection::vec(inline_piece(), 1..9).prop_map(|pieces| pieces.concat());
    let lines = prop::collection::vec(line, 1..7);
    let container = prop::sample::select(vec![
        ("", ""),
        ("> ", "> "),
        ("- ", "  "),
        ("- ", ""),
        ("[^n]: ", "  "),
        ("> - ", ">   "),
    ]);
    (lines, container, any::<bool>()).prop_map(|(lines, (first, then), div)| {
        let body = lines
            .iter()
            .enumerate()
            .map(|(index, line)| format!("{}{line}", if index == 0 { first } else { then }))
            .collect::<Vec<_>>()
            .join("\n");
        if div {
            format!("::: d\n{body}\n:::")
        } else {
            body
        }
    })
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 512, ..ProptestConfig::default() })]

    /// Joining a paragraph's lines where the load would join them changes nothing the
    /// editor reads, whatever the words hold: every line break of every paragraph is
    /// joined here, not only those of a paragraph past the ceiling, and the document the
    /// editor builds from the joined text is the one it builds from the text.
    #[test]
    fn joining_a_paragraphs_lines_changes_nothing_the_editor_reads(
        paragraphs in prop::collection::vec(paragraph_in_a_container(), 1..4),
    ) {
        let text = paragraphs.join("\n\n") + "\n";
        let Some(joins) = joins_in(&text, 0) else {
            return Ok(());
        };
        let spaces: Vec<_> = joins.into_iter().filter(|join| !join.in_code).collect();
        if let Some(joined) = join_lines(&text, &spaces) {
            let before = reading(text.clone());
            if before.is_some() {
                prop_assert_eq!(reading(joined.clone()), before, "{:?} became {:?}", text, joined);
            }
        }
    }
}
