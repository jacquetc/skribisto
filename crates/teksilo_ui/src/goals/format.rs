// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! How a count is written, in one place.
//!
//! A book-scale figure is unreadable as a bare run of digits, and every surface that prints
//! one — the Overview's columns, the Inspector's readout, the status bar, the Distribute
//! preview — has to group it the same way or the same number looks like two.

/// Group a count with thin spaces, so a five-figure book total stays readable at a glance.
///
/// Locale-independent on purpose: a narrow no-break space reads correctly everywhere, while
/// a comma or a period is ambiguous between the two conventions a reader might be applying.
pub fn format_count(n: usize) -> String {
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push('\u{202F}'); // narrow no-break space
        }
        out.push(c);
    }
    out
}

/// The same grouping for a stored target, which is an `i64` and is never negative.
pub fn format_goal(n: i64) -> String {
    format_count(n.max(0) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_in_threes_from_the_right() {
        assert_eq!(format_count(0), "0");
        assert_eq!(format_count(999), "999");
        assert_eq!(format_count(1_000), "1\u{202F}000");
        assert_eq!(format_count(90_000), "90\u{202F}000");
        assert_eq!(format_count(1_234_567), "1\u{202F}234\u{202F}567");
    }

    #[test]
    fn a_target_is_grouped_the_same_way_and_never_prints_a_minus() {
        assert_eq!(format_goal(90_000), "90\u{202F}000");
        assert_eq!(format_goal(-1), "0");
    }
}
