// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A ceiling on how deeply nested the Djot in a bundle may be.
//!
//! # The failure this prevents
//!
//! The Djot parser (`jotdown`, reached through `text-document`) descends once
//! per nested block container and has no depth limit of its own. A few kilobytes
//! of prose — on the order of two thousand nested blockquote markers — exhausts
//! the stack. That is not a panic: **a stack overflow aborts the process**, it
//! cannot be caught by `catch_unwind` or a panic hook, and every unsaved
//! document in every window of that process dies with it.
//!
//! The parse happens when a document is opened or exported, not during
//! `read_folder` — but a bundle is the boundary the bytes
//! cross, and it is the last place a refusal can still name a file and leave the
//! writer's own project untouched.
//!
//! # Why a scan rather than a parser limit
//!
//! The parser is the right place for a depth limit and this is not it — see the
//! note at the bottom. What this module can do without reaching into that crate
//! is bound the *input*: nesting cannot exceed the number of nesting markers the
//! text actually contains, so counting them is a conservative upper bound on how
//! deep the parser can go.
//!
//! It is deliberately an over-estimate. Every construct counted here *may* open
//! a container; some will not (a `>` inside a code block is prose). Over-counting
//! is the safe direction: it can only refuse a document that was closer to the
//! ceiling than it looked, and the ceiling is set two orders of magnitude above
//! anything a person writes.
//!
//! # The limit
//!
//! [`MAX_DEPTH`](crate::MAX_DJOT_DEPTH) is 96. For scale: a blockquote inside a list inside a footnote
//! inside a div is 4. Markdown's own reference implementation warns above 30ish;
//! CommonMark's spec caps list nesting far lower than this. No manuscript
//! reaches 96, and 96 is far below the ~2000 that overflows a debug build.

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
            "prose nests {} levels deep at line {} — the limit is {MAX_DEPTH}. \
             A file this deeply nested crashes the Djot parser rather than \
             rendering, so it is refused rather than opened",
            self.depth, self.line
        )
    }
}

impl std::error::Error for TooDeep {}

/// Refuse `text` if its block nesting could exceed [`MAX_DEPTH`].
///
/// Counts, per line: the run of blockquote markers that opens it, the number of
/// currently-open `:::` div fences, and one level per two columns of leading
/// indentation (the coarsest list-nesting unit Djot admits). Their sum is the
/// bound for that line.
pub fn check(text: &str) -> Result<(), TooDeep> {
    let mut open_divs = 0usize;

    for (i, line) in text.lines().enumerate() {
        // A div fence is `:::` optionally followed by a class; a bare `:::`
        // closes the innermost one. Both are counted before the depth test, so a
        // line that only closes a fence is judged at the depth it leaves behind.
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix(":::") {
            // A **closing** fence is colons and nothing else. Djot lets a fence
            // be longer than three so that one div can nest inside another, so
            // `strip_prefix(":::")` leaves `":"` on a `::::` line — and treating
            // any non-empty remainder as a class made a closing `::::` count as
            // a second opener. `open_divs` then never came back down, and about
            // fifty legitimately nested pairs were enough to refuse the project.
            let rest = rest.trim();
            if rest.is_empty() || rest.chars().all(|c| c == ':') {
                open_divs = open_divs.saturating_sub(1);
            } else {
                open_divs += 1;
                // A run of thousands of opening fences is the same attack as a
                // run of thousands of `>`; check as we go rather than only at
                // the end of the line's own marker run.
                if open_divs > MAX_DEPTH {
                    return Err(TooDeep {
                        depth: open_divs,
                        line: i + 1,
                    });
                }
            }
            continue;
        }

        let indent = line.len() - trimmed.len();
        let mut quotes = 0usize;
        for ch in trimmed.chars() {
            match ch {
                '>' => quotes += 1,
                ' ' | '\t' => {}
                _ => break,
            }
        }

        let depth = quotes + open_divs + indent / 2;
        if depth > MAX_DEPTH {
            return Err(TooDeep { depth, line: i + 1 });
        }
    }

    Ok(())
}

// A depth limit inside `jotdown` would be strictly better than this: it would
// bound the recursion itself rather than an over-estimate of it, and it would
// cover every caller rather than the ones that remember to ask. That fix belongs
// in `jotdown` (or in `text-document`, which wraps it) and is not this crate's
// to make. This module is the boundary guard that does not require it.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_prose_passes() {
        for text in [
            "The ferry was late.\n\nShe waited.\n",
            "> He said it plainly.\n>\n> Then he left.\n",
            "- one\n  - two\n    - three\n",
            "::: note\nA note.\n:::\n",
            "> - a quoted list\n>   - nested once\n",
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
    /// A `::::` line is a *close* when it carries no class — reading it as a
    /// second opener made the count climb for ever.
    #[test]
    fn a_longer_closing_fence_closes_rather_than_opens() {
        let text = ":::: outer\n::: inner\nbody\n:::\n::::\n".repeat(200);
        assert!(
            check(&text).is_ok(),
            "nested fences must not accumulate depth"
        );
    }

    #[test]
    fn the_error_names_the_line() {
        let text = format!("fine\n{}deep\n", ">".repeat(300));
        let err = check(&text).unwrap_err();
        assert_eq!(err.line, 2);
        assert!(err.to_string().contains("line 2"), "{err}");
    }
}
