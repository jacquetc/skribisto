// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Djot ceiling: what it refuses, what it lets through, and proof that what it
//! lets through parses.
//!
//! The parses run on a thread with the 2 MiB stack a long operation gets, in the
//! debug build the suite runs in, through `set_djot_sync`, which runs `jotdown` on
//! the calling thread. A stack overflow is not a panic, so a regression here does
//! not fail a test: it aborts the test binary, which is the signal.

use super::*;
use proptest::prelude::*;
use text_document::TextDocument;

/// The stack `std::thread::spawn` gives a long operation's worker by default.
const LONG_OPERATION_STACK: usize = 2 << 20;

/// Parse `djot` on a thread with a long operation's stack and return its plain
/// text, or the parser's own error.
pub(crate) fn parse_on_a_long_operation_stack(djot: String) -> Result<String, String> {
    std::thread::Builder::new()
        .stack_size(LONG_OPERATION_STACK)
        .spawn(move || {
            let doc = TextDocument::new();
            doc.set_djot_sync(&djot).map_err(|e| e.to_string())?;
            doc.to_plain_text().map_err(|e| e.to_string())
        })
        .expect("spawn the parse thread")
        .join()
        .expect("the parse must not unwind")
}

/// Every marker `jotdown` 0.10 opens a container with at the start of a line, as
/// the text that opens one more level when it is repeated on one line.
const ONE_LINE_MARKERS: [&str; 20] = [
    "- ", "* ", "+ ", "1. ", "1) ", "(1) ", "a. ", "B) ", "(c) ", "iv. ", "XII) ", "(ix) ",
    "- [ ] ", "* [x] ", "+ [X] ", "[^a]: ", "[^note]:", "[link]: ", ": ", "> ",
];

/// `levels` of `marker` on one line, then a word, which the last one holds.
pub(crate) fn one_line(marker: &str, levels: usize) -> String {
    format!("{}deep\n", marker.repeat(levels))
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
            "a run of blockquote markers",
            format!("{}deep\n", ">".repeat(4_000)),
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

/// A blockquote marker run is the cheapest way to reach the parser's
/// recursion, and the shape that was measured aborting the process.
#[test]
fn a_deep_blockquote_run_is_refused() {
    let text = format!("{}deep\n", ">".repeat(2_000));
    let err = check(&text).expect_err("2000 levels must be refused");
    assert_eq!(err.line, 1);
    assert!(err.depth > MAX_DEPTH);
}

#[test]
fn deeply_stacked_divs_are_refused() {
    let text = "::: a\n".repeat(500);
    let err = check(&text).expect_err("500 open divs must be refused");
    assert!(err.depth > MAX_DEPTH);
}

#[test]
fn runaway_indentation_is_refused() {
    let text = format!("{}item\n", " ".repeat(1_000));
    assert!(check(&text).is_err());
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
        let err = check(&text).expect_err("300 levels must be refused");
        assert!(err.depth > MAX_DEPTH, "{space:?}: {err:?}");
    }
}

#[test]
fn the_error_names_the_line() {
    let text = format!("fine\n{}deep\n", ">".repeat(300));
    let err = check(&text).unwrap_err();
    assert_eq!(err.line, 2);
    assert!(err.to_string().contains("line 2"), "{err}");
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
        let err = check(&text).expect_err(&format!("{marker:?} past the ceiling"));
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
    let err = check(&nested(MAX_DEPTH + 1)).expect_err("one level past");
    assert_eq!(err.line, 2 * MAX_DEPTH + 1, "the deepest item's line");
}

/// A div opened inside a blockquote on every line of it nests one more on each,
/// since the blockquote carries every line into the div the line before opened.
#[test]
fn a_div_opened_after_a_quote_on_every_line_is_counted() {
    let text = "> ::: note\n".repeat(MAX_DEPTH);
    let err = check(&text).expect_err("a div per line, inside one quote");
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
                prop_assert_eq!(refused.line, 2 * before + 1);
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

    /// A refusal always names a line the text has.
    #[test]
    fn a_refusal_names_a_line_of_the_text(text in marker_soup()) {
        if let Err(refused) = check(&text) {
            prop_assert!(refused.line >= 1);
            prop_assert!(refused.line <= text.split('\n').count());
            prop_assert!(refused.depth > MAX_DEPTH);
        }
    }
}
