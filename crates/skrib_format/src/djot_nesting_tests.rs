// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The exact nesting scan, on the shapes that decide it. Each expected depth is the one
//! `jotdown` 0.10 builds, which `text-document`'s copy of this scan is held to on
//! generated documents; these pin the port to the same answers.

use super::*;

/// The deepest line of `text`, read with no limit.
fn depth(text: &str) -> usize {
    let mut nesting = Nesting::default();
    text.split_inclusive('\n')
        .map(|line| nesting.read(line, usize::MAX))
        .max()
        .unwrap_or(0)
}

/// A list whose items each step in past the last by the width of their marker, with a
/// paragraph line at no indentation after each and a blank line before the next.
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

#[test]
fn ordinary_prose_nests_as_the_parser_nests_it() {
    for (text, expected) in [
        ("The ferry was late.\n\nShe waited.\n", 0),
        ("> He said it plainly.\n>\n> Then he left.\n", 1),
        ("- one\n\n  - two\n\n    - three\n", 3),
        ("::: note\nA note.\n:::\n", 1),
        ("> - a quoted list\n>\n>   - nested once\n", 3),
        ("1. First.\n\n   a. Inside.\n\n      i. Deeper.\n", 3),
        (
            "A note.[^1]\n\n[^1]: The note.\n\n    A second paragraph.\n",
            1,
        ),
        ("- [ ] a task\n- [x] a done one\n", 1),
        ("Term\n\n: its definition\n", 1),
        ("| a | b |\n|---|---|\n| c | d |\n", 1),
        ("* * *\n\n- - - - - -\n", 0),
        ("", 0),
    ] {
        assert_eq!(depth(text), expected, "{text:?}");
    }
}

/// A list item goes on through a paragraph line at no indentation after a line that was
/// not blank, so each item one step further in nests inside the last: one level a step,
/// in a quotation or out of one, for every kind of item.
#[test]
fn a_list_continued_by_paragraph_lines_nests_one_level_per_step() {
    for (marker, prefix, quotes) in [
        ("- ", "", 0),
        ("[^a]: ", "", 0),
        ("1. ", "> ", 1),
        ("- ", "> > ", 2),
    ] {
        for levels in [1, 2, 10, 97] {
            assert_eq!(
                depth(&lazy_staircase(marker, levels, prefix)),
                levels + quotes,
                "{marker:?} {prefix:?} x{levels}"
            );
        }
    }
}

/// A line that opens nothing sits exactly as deep as what it continues, however far it
/// is indented: indentation alone opens nothing.
#[test]
fn a_line_that_opens_nothing_sits_as_deep_as_what_it_continues() {
    assert_eq!(depth(&format!("{}words\n", " ".repeat(1_000))), 0);
    assert_eq!(depth(&format!("- item\n\n{}words\n", " ".repeat(900))), 1);
    assert_eq!(depth(&format!("> quote\n>{}words\n", " ".repeat(900))), 1);
}

/// A quotation opened inside a list item on a later line of it is nested in the item.
#[test]
fn a_quote_opened_on_an_items_later_line_nests_in_it() {
    assert_eq!(depth("- a\n\n  > - b\n  >\n  >   > - c\n"), 5);
}

/// The parser strips a div's indentation from each line of its content except the
/// first, so an item there keeps it, and the next line closes that item rather than
/// nesting in it.
#[test]
fn a_div_leaves_its_first_line_as_it_found_it() {
    assert_eq!(depth("  ::: d\n  - a\n\n   - b\n  :::\n"), 2);
}

/// A div's closing fence closes it, and what it held; a code fence the div saw open
/// keeps any fence from closing it.
#[test]
fn a_div_is_closed_by_its_own_fence_unless_code_holds_it_open() {
    assert_eq!(
        depth(&":::: outer\n::: inner\nbody\n:::\n::::\n".repeat(50)),
        2
    );
    assert_eq!(
        depth(&format!("::: a\n{}", "- item\n\n  ```x\n:::\n".repeat(10))),
        11
    );
}

/// Text that only looks like a container opens nothing: a run of `>` with no space, a
/// run of link definitions (a leaf), a thematic break, a number too long to be a list
/// marker.
#[test]
fn text_that_only_looks_like_a_container_opens_nothing() {
    for text in [
        format!("{}deep\n", ">".repeat(4_000)),
        format!("{}deep\n", "[link]: ".repeat(400)),
        format!("{}\n", "- * ".repeat(300)),
        format!("{}\n", "12345678901234567890. ".repeat(300)),
        format!("{}Title\n", "\u{A0}".repeat(300)),
    ] {
        assert_eq!(depth(&text), 0, "{text:.40?}");
    }
}

/// Past its limit, a line opens one container more and no further, so refusing a line
/// costs no more work than reading one at the limit.
#[test]
fn a_line_opens_no_more_than_one_past_the_limit() {
    let mut nesting = Nesting::default();
    assert_eq!(nesting.read(&"> ".repeat(10_000), 96), 97);
}

/// How deep a line written after `text`, a blank line and its closing fences sits: zero
/// when the fences closed everything the text left open.
fn depth_after_closing(text: &str) -> usize {
    let written = format!("{text}\n\n{}", closing_fences(text));
    let mut nesting = Nesting::default();
    for line in written.split_inclusive('\n') {
        nesting.read(line, usize::MAX);
    }
    nesting.read("# The next chapter\n", usize::MAX)
}

/// Every div and code block a text leaves open is closed, whatever holds it open:
/// divs inside divs, a longer fence, a code fence a div saw open, a code block alone.
/// A bare fence closes the outermost div it is long enough for, and everything inside
/// it, so divs of one fence length close with one. What a quotation or a list item held
/// ends with them, and needs no fence.
#[test]
fn closing_fences_close_everything_a_text_leaves_open() {
    for (text, fences) in [
        ("Words.", ""),
        ("::: aside\nWords.", ":::\n\n"),
        ("::: a\n::: b\nWords.", ":::\n\n"),
        (":::: outer\n::: inner\nWords.", ":::\n::::\n\n"),
        ("```\ncode", "```\n\n"),
        ("~~~~ lang\ncode", "~~~~\n\n"),
        ("::: a\n```\ncode", "```\n:::\n\n"),
        ("::: a\n- item\n\n  ```x\n", "```\n:::\n\n"),
        ("> ::: quoted\n> Words.", ""),
        ("- ::: listed\n  Words.", ""),
        ("::: a\nWords.\n:::", ""),
    ] {
        assert_eq!(closing_fences(text), fences, "{text:?}");
        assert_eq!(depth_after_closing(text), 0, "{text:?}");
    }
    let deep = "::: aside\n".repeat(200);
    assert_eq!(closing_fences(&deep), ":::\n\n");
    assert_eq!(depth_after_closing(&deep), 0);
    let descending: String = (0..200)
        .map(|i| format!("{}\n", ":".repeat(203 - i)))
        .collect();
    assert_eq!(closing_fences(&descending).lines().count(), 201);
    assert_eq!(depth_after_closing(&descending), 0);
}

proptest::proptest! {
    /// Whatever a text of divs, code fences, quotations and list items leaves open, the
    /// line after its closing fences sits at the top level.
    #[test]
    fn nothing_outlives_the_closing_fences(
        lines in proptest::collection::vec(
            proptest::sample::select(vec![
                "::: a", ":::: b", ":::", "::::", "```", "```x", "~~~", "````", "- item", "  - in",
                "> quoted", "> ::: q", "[^n]: note", "  more", "", "words", "| a |", "# h",
            ]),
            0..40,
        )
    ) {
        let text = lines.join("\n");
        proptest::prop_assert_eq!(depth_after_closing(&text), 0, "{:?}", text);
    }
}
