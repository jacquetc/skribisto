// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A person's initials, for the short label a word processor shows beside a comment.
//!
//! OOXML gives every comment a `w:initials` alongside `w:author`, and Word renders *that*
//! in the margin — so a comment exported without one is a remark with no visible owner
//! once it reaches the editor's screen.
//!
//! # Seeded once, then owned by the row
//!
//! This is called **only when a comment or reply is created**, never at export time. The
//! distinction is the whole point: an editor's own initials arrive inside the `.docx` or
//! `.odt` they send back, and they are theirs — "Mary-Jane O'Brien" is as plausibly `MO`
//! as `MJO`, and a rule that recomputed them on the way out would quietly overwrite what a
//! real person chose about their own name. So a stored value is always preferred, and this
//! only fills the blank at the moment a row is born with nothing.
//!
//! # The rule, and its limits
//!
//! The first character of each name-like token, upper-cased, at most
//! [`MAX_INITIALS`]. Tokens split on whitespace and on the separators that join compound
//! names (`-`, `'`, `.`), so "Mary-Jane O'Brien" yields `MJO` and "Jean-Luc Picard" `JLP`.
//!
//! It is deliberately a *seed*, not a claim to correctness. No rule gets every naming
//! culture right — a Spanish double surname, a Japanese name written family-first, a mononym
//! — and this makes no attempt to. It produces something reasonable and short, and the value
//! is a plain stored string the writer or their editor can replace.

/// The most initials worth showing in a margin label.
///
/// Word's own field is unbounded, but the label is drawn in a narrow gutter and a long
/// string is simply clipped. Three covers the overwhelming majority of names, including
/// the compound forms below.
pub const MAX_INITIALS: usize = 3;

/// Initials for `name`, or an empty string when it holds no letters.
///
/// Empty in, empty out — and empty is a legitimate answer, not a failure: it means the
/// writers emit no initials rather than inventing one.
pub fn initials_from_name(name: &str) -> String {
    name.split(|c: char| c.is_whitespace() || c == '-' || c == '\'' || c == '.' || c == '’')
        .filter_map(|token| token.chars().find(|c| c.is_alphabetic()))
        .take(MAX_INITIALS)
        .flat_map(|c| c.to_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plain_two_part_name_gives_two_initials() {
        assert_eq!(initials_from_name("Mara Vane"), "MV");
    }

    #[test]
    fn compound_names_split_on_their_joiners() {
        assert_eq!(initials_from_name("Mary-Jane O'Brien"), "MJO");
        assert_eq!(initials_from_name("Jean-Luc Picard"), "JLP");
        assert_eq!(initials_from_name("J.R.R. Tolkien"), "JRR");
        // The typographic apostrophe the smart-punctuation engine produces must split
        // exactly as the ASCII one does, or a name typed in the app initials differently
        // from the same name pasted in.
        assert_eq!(initials_from_name("Mary-Jane O’Brien"), "MJO");
    }

    #[test]
    fn a_long_name_is_capped() {
        assert_eq!(
            initials_from_name("One Two Three Four Five").len(),
            MAX_INITIALS
        );
    }

    #[test]
    fn a_mononym_gives_one_initial() {
        assert_eq!(initials_from_name("Colette"), "C");
    }

    #[test]
    fn non_latin_names_take_their_own_first_letters() {
        assert_eq!(initials_from_name("Лев Толстой"), "ЛТ");
        assert_eq!(initials_from_name("مها الفيصل"), "ما");
    }

    /// A name with no letters at all — punctuation, digits, or nothing — must yield an
    /// empty string rather than a stray character, because empty is what tells the
    /// writers to emit no initials.
    #[test]
    fn a_name_with_no_letters_yields_nothing() {
        assert_eq!(initials_from_name(""), "");
        assert_eq!(initials_from_name("   "), "");
        assert_eq!(initials_from_name("42"), "");
        assert_eq!(initials_from_name("-.'"), "");
    }

    /// A lower-case name still gives upper-case initials — the label is an abbreviation,
    /// not a quotation.
    #[test]
    fn initials_are_upper_cased() {
        assert_eq!(initials_from_name("mara vane"), "MV");
    }
}
