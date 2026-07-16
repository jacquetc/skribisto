// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Resolve the word(s) an editor's "Add to dictionary" menu item should act on,
//! from the current selection or the word under the caret.
//!
//! Kept out of the already-large `editor.rs` and, crucially, uses the **same**
//! tokenizer the spell-check squiggles use (`crate::spellcheck::word_positions`),
//! so what is addable and what is flagged can never disagree on where a word
//! begins and ends (apostrophes, elisions, CJK).
//!
//! Document char offsets are absolute (`block.position()` + block-local offset),
//! the same space [`SpellSession`](crate::spellcheck) works in. A bare right-click
//! now moves the caret to the click point (bastyde's `RichTextEditor` fix), so
//! [`resolve_words`] reads the live caret/selection without needing the pixel.

use bastyde::text_document::TextDocument;
use bastyde::widgets::rich_text::EditorHandle;

use crate::spellcheck::{SpellSession, word_positions};

/// A selection spanning more than this many characters, or resolving to more
/// than [`MAX_SELECTION_WORDS`] words, is treated as an accidental broad drag —
/// the item resolves to nothing (disabled) rather than whitelisting a paragraph.
const MAX_SELECTION_CHARS: usize = 120;
const MAX_SELECTION_WORDS: usize = 15;

/// The word token(s) to offer "Add to dictionary" for, given the editor's live
/// caret/selection. Empty ⇒ the menu item is disabled.
///
/// With a selection, the word tokens it overlaps (same block, bounded); with no
/// selection, the single word under the caret. **Only words the spell-checker
/// flags** are kept — the offer matches exactly what is squiggled, and a word
/// already in the dictionary (hence not flagged) is never re-offered. Without an
/// active checker nothing is "wrong", so nothing is offered. Deduped case-exactly,
/// first casing wins.
pub(crate) fn resolve_words(
    doc: &TextDocument,
    handle: &EditorHandle,
    spell: Option<&SpellSession>,
) -> Vec<String> {
    let Some(spell) = spell else {
        return Vec::new();
    };
    let pos = handle.cursor_position();
    let anchor = handle.cursor_anchor_signal().get();
    let (start, end) = (anchor.min(pos), anchor.max(pos));
    let words = if start != end {
        words_in_range(doc, start, end)
    } else {
        word_at(doc, pos).into_iter().collect()
    };
    keep_misspelled(words, |w| spell.is_misspelled(w))
}

/// Keep only the flagged words, then dedup case-exactly (first casing wins).
fn keep_misspelled(words: Vec<String>, is_misspelled: impl Fn(&str) -> bool) -> Vec<String> {
    dedup_keep_first(words.into_iter().filter(|w| is_misspelled(w)).collect())
}

/// Word tokens overlapping the document char range `[start, end)`. Same-block
/// only; empty if the range is empty, crosses a block boundary, exceeds
/// [`MAX_SELECTION_CHARS`], or resolves to more than [`MAX_SELECTION_WORDS`].
pub(crate) fn words_in_range(doc: &TextDocument, start: usize, end: usize) -> Vec<String> {
    if end <= start || end - start > MAX_SELECTION_CHARS {
        return Vec::new();
    }
    for block in doc.blocks() {
        let base = block.position();
        let text = block.text();
        let block_end = base + text.chars().count();
        if start < base || end > block_end {
            continue; // not (wholly) in this block — same-block scope
        }
        let mut out = Vec::new();
        for (char_off, len, word) in word_positions(&text) {
            let wstart = base + char_off;
            let wend = wstart + len;
            // Overlap with [start, end) and at least one alphabetic char.
            if wstart < end && wend > start && is_wordlike(word) {
                out.push(word.to_string());
                if out.len() > MAX_SELECTION_WORDS {
                    return Vec::new(); // too broad → disable rather than truncate
                }
            }
        }
        return out;
    }
    Vec::new()
}

/// The word token whose span contains the caret char offset `pos` (inclusive of
/// both ends, so a caret at a word's edge still resolves it — the same rule the
/// squiggle-exemption uses). `None` when the caret is not in a word-like token.
pub(crate) fn word_at(doc: &TextDocument, pos: usize) -> Option<String> {
    for block in doc.blocks() {
        let base = block.position();
        let text = block.text();
        let block_end = base + text.chars().count();
        if pos < base || pos > block_end {
            continue;
        }
        for (char_off, len, word) in word_positions(&text) {
            let wstart = base + char_off;
            let wend = wstart + len;
            if pos >= wstart && pos <= wend {
                return is_wordlike(word).then(|| word.to_string());
            }
        }
        return None;
    }
    None
}

/// At least one alphabetic character — matches `SpellChecker::misspelled`'s guard,
/// so numbers / punctuation never become addable.
fn is_wordlike(word: &str) -> bool {
    word.chars().any(|c| c.is_alphabetic())
}

/// Dedup exact-case, preserving first-seen order/casing.
fn dedup_keep_first(words: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    words.into_iter().filter(|w| seen.insert(w.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(text: &str) -> TextDocument {
        let d = TextDocument::new();
        d.set_plain_text(text).unwrap();
        d
    }

    #[test]
    fn word_at_finds_the_token_under_the_caret() {
        let d = doc("Hello world");
        assert_eq!(word_at(&d, 0).as_deref(), Some("Hello"));
        assert_eq!(word_at(&d, 3).as_deref(), Some("Hello"));
        assert_eq!(word_at(&d, 5).as_deref(), Some("Hello")); // inclusive of the end edge
        assert_eq!(word_at(&d, 8).as_deref(), Some("world"));
        assert_eq!(word_at(&d, 11).as_deref(), Some("world"));
    }

    #[test]
    fn word_at_keeps_apostrophes_and_rejects_non_words() {
        assert_eq!(word_at(&doc("I don't"), 4).as_deref(), Some("don't"));
        // A caret in a run of digits / punctuation resolves to nothing addable.
        assert_eq!(word_at(&doc("12 345"), 1), None);
    }

    #[test]
    fn words_in_range_returns_overlapping_tokens() {
        let d = doc("Hello brave world");
        assert_eq!(words_in_range(&d, 0, 17), vec!["Hello", "brave", "world"]);
        assert_eq!(words_in_range(&d, 6, 11), vec!["brave"]);
        // A partial overlap still counts the touched word.
        assert_eq!(words_in_range(&d, 8, 11), vec!["brave"]);
    }

    #[test]
    fn words_in_range_is_empty_for_empty_or_oversized_ranges() {
        let d = doc("Hello world");
        assert!(words_in_range(&d, 5, 5).is_empty(), "empty range");
        let long = "word ".repeat(40);
        let d2 = doc(&long);
        assert!(
            words_in_range(&d2, 0, long.chars().count()).is_empty(),
            "an over-long/over-wide selection disables the item"
        );
    }

    #[test]
    fn keep_misspelled_offers_only_flagged_words() {
        // Only words the checker flags survive (and dedup still applies). Here
        // "Ursai" and "Drexel" are flagged; the ordinary words are dropped.
        let flagged = ["Ursai", "Drexel"];
        let got = keep_misspelled(
            vec![
                "In".into(),
                "Ursai".into(),
                "an".into(),
                "Drexel".into(),
                "world".into(),
            ],
            |w| flagged.contains(&w),
        );
        assert_eq!(got, vec!["Ursai", "Drexel"]);
        // Nothing flagged ⇒ nothing offered (the item is disabled).
        assert!(keep_misspelled(vec!["all".into(), "fine".into()], |_| false).is_empty());
    }

    #[test]
    fn dedup_keep_first_collapses_exact_repeats() {
        // Exact-case dedup (applied by `resolve_words` over the raw token list):
        // "the" and "The" are distinct; a repeated "the" collapses, first casing kept.
        assert_eq!(
            dedup_keep_first(vec!["the".into(), "The".into(), "the".into()]),
            vec!["the", "The"]
        );
        // And the raw range still returns every overlapping token (no dedup there).
        let d = doc("the The the");
        assert_eq!(words_in_range(&d, 0, 11), vec!["the", "The", "the"]);
    }
}
