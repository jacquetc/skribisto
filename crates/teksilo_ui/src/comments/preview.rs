// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A markup-free, one-line-friendly preview of a comment or reply's own Djot
//! body — shared by every surface that shows a *summary* of a turn's text
//! rather than its full, editable form: the AccessKit annotation summary
//! (`comments::binding::CommentBinding::annotation_spans`), the two docks'
//! preview line (`crate::comments::dock::comment_card`'s `body_line`), and nothing
//! else — the card itself (`comments::card::Turn`) shows the real body in a
//! real `RichTextEditor`, which needs no plain-text stand-in at all.
//!
//! Written once, here, rather than at each of those two call sites: a body is
//! Djot now (M-S4 — an editor's own emphasis is real content, not flattened
//! away on import), and before this existed every one of these surfaces handed
//! the raw Djot straight to a screen reader or a `TextWidget` as if it were
//! prose. `*this*` read out — or printed — as `*this*`, asterisks and all,
//! rather than as the emphasised word it actually is. Three copies of the fix
//! would have been three chances for the next Djot feature (a footnote
//! reference, say) to be handled correctly in one and forgotten in the other
//! two.

use teksilo::text_document::{DjotImportOptions, djot_to_plain_text};

/// The plain-text rendering of `body`, collapsed to one line.
///
/// Multiple paragraphs are joined with a single space rather than kept on
/// separate lines: every caller here is a *summary* — a screen-reader
/// sentence, a dock's subtitle row — never the full text, and a preview that
/// suddenly grew an embedded `\n` would either be swallowed by the widget's
/// own single-line layout or announce as a pause a listener has no way to
/// interpret. The result is trimmed, so a body that opens or closes on a
/// blank paragraph does not leave a stray leading or trailing space.
///
/// `djot_to_plain_text` (via `teksilo::text_document`, the same pure,
/// synchronous converter — no `TextDocument`, no async `.wait()` — the
/// importer's own diagnostics use) rather than a live `TextDocument`: every
/// call site here already has the stored Djot string in hand from a `CommentRow`/
/// `ReplyRow`, and minting a document per preview would parse the same text on
/// every rebuild of every card, for text nobody is about to edit through it.
pub fn plain_preview(body: &str) -> String {
    if body.is_empty() {
        return String::new();
    }
    djot_to_plain_text(body, &DjotImportOptions::default())
        .replace('\n', " ")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A paragraph break becomes exactly **one** space, never two.
    ///
    /// Worth pinning rather than assuming: the source string contains `\n\n`, so a reader
    /// expects `replace('\n', " ")` to leave a double space behind. It does not, because the
    /// replacement runs on the *parsed* text, and `djot_to_plain_text` joins blocks with a
    /// single `\n` — the blank line is Djot's paragraph separator, not part of the prose. The
    /// existing tests checked only that no newline survived, which would pass either way.
    #[test]
    fn a_paragraph_break_becomes_exactly_one_space() {
        assert_eq!(
            plain_preview("First thought.\n\nSecond thought."),
            "First thought. Second thought."
        );
    }

    #[test]
    fn plain_prose_passes_through_unchanged() {
        assert_eq!(
            plain_preview("Is this too on-the-nose?"),
            "Is this too on-the-nose?"
        );
    }

    #[test]
    fn emphasis_markers_are_stripped_not_shown_literally() {
        assert_eq!(
            plain_preview("Is this *really* the _right_ word?"),
            "Is this really the right word?"
        );
    }

    #[test]
    fn strikethrough_markers_are_stripped_too() {
        assert_eq!(
            plain_preview("{-Cut this-} entirely."),
            "Cut this entirely."
        );
    }

    /// A comment written as two paragraphs (M-S4: `Enter` inside the card's
    /// body editor is a real paragraph break) previews as one line, not two —
    /// this is a summary, not the body itself.
    #[test]
    fn multiple_paragraphs_collapse_to_one_line() {
        let preview = plain_preview("First thought.\n\nSecond thought.");
        assert!(!preview.contains('\n'), "got {preview:?}");
        assert!(preview.contains("First thought."));
        assert!(preview.contains("Second thought."));
    }

    #[test]
    fn an_empty_body_previews_as_empty() {
        assert_eq!(plain_preview(""), "");
    }
}
