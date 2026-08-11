// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Who a comment is signed by, and with which initials.
//!
//! Two sources, in order. The app-level identity ([`crate::USER_NAME_KEY`] /
//! [`crate::USER_INITIALS_KEY`]) comes first because it is the only one that
//! describes *the person typing*. The project's own `Work.author_name` is the
//! fallback, and a poor one: it is the **book's byline**, it travels inside the
//! `.skrib`, and an editor who opens someone else's manuscript would sign every
//! remark they write with the novelist's name. It stays as a fallback only
//! because signing with the byline beats signing with nothing at all on the
//! single-author projects that are the common case.
//!
//! Resolved **at creation time and stored on the row** — never re-derived when a
//! comment is read or exported. A remark is signed by whoever wrote it, so
//! changing the name here must not retroactively re-attribute what is already on
//! disk. It also means an editor's own name and initials, arriving inside a
//! returned `.docx`, are never overwritten by ours.
//!
//! Pure functions over `&str`, deliberately: no signals, no settings store, no
//! `Work` — so the fallback order is unit-testable without a running app, which
//! is what the tests at the bottom of this file do.

use skribisto_model::initials::initials_from_name;

/// The name and initials a comment created *now* would carry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Signature {
    /// Empty when nothing is set anywhere — a legitimate state, rendered as
    /// "Unknown author" on the card and as no `w:author` on the way out.
    pub name: String,
    /// Empty only when there is no name to derive from and none was typed.
    pub initials: String,
}

impl Signature {
    /// Nothing identifies the writer, so a comment made now would be anonymous.
    ///
    /// What the "your comments are unsigned" toast gates on. Keyed on the *name*
    /// alone: initials without a name still leaves the card reading "Unknown
    /// author", which is the thing the writer is being warned about.
    pub fn is_anonymous(&self) -> bool {
        self.name.is_empty()
    }
}

/// Resolve the signature from the app-level identity and the project's byline.
///
/// Every input is trimmed, so a field holding only spaces counts as unset rather
/// than signing comments with whitespace.
pub fn resolve(user_name: &str, user_initials: &str, work_author: &str) -> Signature {
    let name = {
        let user = user_name.trim();
        if user.is_empty() {
            work_author.trim()
        } else {
            user
        }
    };
    // Typed initials win even when the name is blank: someone who filled in only
    // that field has said what they want in the margin, and deriving `""` from an
    // empty name would silently throw it away.
    let typed = user_initials.trim();
    let initials = if typed.is_empty() {
        initials_from_name(name)
    } else {
        typed.to_string()
    };
    Signature {
        name: name.to_string(),
        initials,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_app_level_name_wins_over_the_books_byline() {
        // The case the whole module exists for: an editor working on someone
        // else's manuscript signs with their own name, not the novelist's.
        let s = resolve("Rae Okafor", "", "C. Jacquet");
        assert_eq!(s.name, "Rae Okafor");
        assert_eq!(s.initials, "RO");
    }

    #[test]
    fn the_byline_is_the_fallback_when_no_user_is_set() {
        let s = resolve("", "", "C. Jacquet");
        assert_eq!(s.name, "C. Jacquet");
        assert_eq!(s.initials, "CJ");
    }

    #[test]
    fn typed_initials_override_the_derivation() {
        // "Mary-Jane O'Brien" derives MJO; someone who wants MO must get MO.
        let s = resolve("Mary-Jane O'Brien", "MO", "");
        assert_eq!(s.initials, "MO");
    }

    #[test]
    fn typed_initials_survive_an_empty_name() {
        let s = resolve("", "MO", "");
        assert_eq!(s.name, "");
        assert_eq!(s.initials, "MO");
        assert!(s.is_anonymous(), "no name is still an unsigned comment");
    }

    #[test]
    fn whitespace_only_fields_count_as_unset() {
        let s = resolve("   ", "  ", "  C. Jacquet ");
        assert_eq!(s.name, "C. Jacquet");
        assert_eq!(s.initials, "CJ");
    }

    #[test]
    fn nothing_set_anywhere_is_anonymous_and_not_a_guess() {
        let s = resolve("", "", "");
        assert!(s.is_anonymous());
        assert_eq!(s.initials, "", "no name means no invented initials");
    }

    #[test]
    fn a_signed_comment_is_not_anonymous() {
        assert!(!resolve("", "", "C. Jacquet").is_anonymous());
    }
}
