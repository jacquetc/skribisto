// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Resolve what an editor's right-click menu can do about spelling — the word(s) its "Add to
//! dictionary" item should act on and the flagged word its corrections should replace — from the
//! current selection or the word under the caret.
//!
//! [`resolve_spelling`] answers both in **one** pass, returning a [`SpellingMenu`]. They are the
//! same question asked twice: resolving them separately meant walking the blocks, tokenising and
//! spell-checking the same word twice per right-click, with two code paths obliged to keep
//! agreeing forever about where a word begins.
//!
//! Kept out of the already-large `editor.rs` and, crucially, uses the **same**
//! tokenizer the spell-check squiggles use (`crate::spellcheck::word_positions`),
//! so what is addable and what is flagged can never disagree on where a word
//! begins and ends (apostrophes, elisions, CJK).
//!
//! Document char offsets are absolute (`block.position()` + block-local offset),
//! the same space [`SpellSession`](crate::spellcheck) works in. A bare right-click
//! now moves the caret to the click point (teksilo's `RichTextEditor` fix), so
//! [`resolve_spelling`] reads the live caret/selection without needing the pixel.

use teksilo::text_document::TextDocument;
use teksilo::widgets::rich_text::EditorHandle;

use crate::spellcheck::{SpellSession, word_positions};

/// A selection spanning more than this many characters, or resolving to more
/// than [`MAX_SELECTION_WORDS`] words, is treated as an accidental broad drag —
/// the item resolves to nothing (disabled) rather than whitelisting a paragraph.
const MAX_SELECTION_CHARS: usize = 120;
const MAX_SELECTION_WORDS: usize = 15;

/// Everything the right-click menu's spelling group needs, resolved from the editor's live
/// caret/selection in **one** pass — see the module docs for why one pass, not two.
#[derive(Default)]
pub(crate) struct SpellingMenu {
    /// The flagged words "Add to dictionary" should act on. Empty ⇒ the item is disabled.
    pub words: Vec<String>,
    /// The single flagged word the corrections replace, if the gesture names exactly one.
    pub correction: Option<Correction>,
}

/// Resolve the spelling group for the editor's live caret/selection.
///
/// **Only words the spell-checker flags** are offered — the group matches exactly what is
/// squiggled, and a word already in the dictionary (hence not flagged) is never re-offered.
/// Without an active checker nothing is "wrong", so nothing is offered.
///
/// Three gestures, in order:
///
/// - **No selection** — the word under the caret (where a right-click leaves it). Correctable.
/// - **A selection spanning exactly one word token** — correctable too. This is the gesture a
///   double-click makes, and `reposition_caret_for_context_menu` deliberately preserves a
///   selection the right-click lands inside; treating any selection as uncorrectable would mean
///   double-click-then-right-click, one of the two normal ways to reach this menu, silently
///   offered no corrections at all.
/// - **A wider or partial selection** — the "add these words" gesture: every flagged token it
///   overlaps (same block, bounded), deduped case-exactly, first casing wins. No single target,
///   so no corrections.
pub(crate) fn resolve_spelling(
    doc: &TextDocument,
    handle: &EditorHandle,
    spell: Option<&SpellSession>,
) -> SpellingMenu {
    let Some(spell) = spell else {
        return SpellingMenu::default();
    };
    // One read for both ends. Pairing the live `cursor_position()` with the
    // `cursor_anchor_signal()` mirror would mix two moments in time: the mirror only
    // refreshes on sync, and this crate already knows it lags (`wire_spell` reads the
    // caret live for exactly that reason). A stale anchor against a fresh caret invents
    // a selection that isn't there — and the whole spelling group then silently
    // vanishes from the menu.
    let (anchor, pos) = handle.selection();
    let (sel_start, sel_end) = (anchor.min(pos), anchor.max(pos));

    // The single-word target, if this gesture names one. A selection is still held to
    // [`MAX_SELECTION_CHARS`]: one "word" can be huge (a pasted hash or URL with no internal
    // break is a single UAX#29 token), and the broad-drag guard must not be escapable just by
    // landing exactly on a token's edges.
    let single = if sel_start == sel_end {
        word_range_at(doc, pos)
    } else if sel_end - sel_start <= MAX_SELECTION_CHARS {
        word_spanning(doc, sel_start, sel_end).map(|w| (sel_start, sel_end, w))
    } else {
        None
    };
    if let Some((start, end, word)) = single {
        if !spell.is_misspelled(&word) {
            return SpellingMenu::default();
        }
        let suggestions = spell.suggest(&word);
        return SpellingMenu {
            words: vec![word],
            correction: Some(Correction {
                start,
                end,
                suggestions,
            }),
        };
    }
    if sel_start != sel_end {
        let words = keep_misspelled(words_in_range(doc, sel_start, sel_end), |w| {
            spell.is_misspelled(w)
        });
        return SpellingMenu {
            words,
            correction: None,
        };
    }
    SpellingMenu::default()
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

/// Where a correction lands and what it may replace the text with: the flagged word's **absolute
/// document char range** `[start, end)` plus the ranked corrections to offer.
///
/// The range is the whole point: adding a word to the dictionary only needs its text (which
/// [`SpellingMenu::words`] already carries), but *replacing* it needs the exact span, in the same
/// char space
/// [`EditorHandle::replace_range`](teksilo::widgets::rich_text::EditorHandle::replace_range)
/// works in.
pub(crate) struct Correction {
    pub start: usize,
    pub end: usize,
    /// Possibly empty — a flagged word nothing can correct is a real outcome, and the menu says
    /// so rather than silently dropping the section.
    pub suggestions: Vec<String>,
}

/// The word token whose span contains the char offset `pos` (inclusive of both ends, so a caret
/// at a word's edge still resolves it — the same rule the squiggle-exemption uses), with its
/// absolute char range `[start, end)`. `None` when `pos` is not in a word-like token.
///
/// Offsets are `block.position()` + the block-local char offset, the space `EditorHandle`'s
/// cursor / selection / replacement APIs use.
fn word_range_at(doc: &TextDocument, pos: usize) -> Option<(usize, usize, String)> {
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
                return is_wordlike(word).then(|| (wstart, wend, word.to_string()));
            }
        }
        return None;
    }
    None
}

/// The word token occupying **exactly** the char range `[start, end)` — a selection that names
/// one whole word and nothing else, as a double-click makes. `None` for a partial word, a span
/// covering several tokens, or one that also swallows surrounding punctuation.
///
/// Deliberately an exact-span match rather than "the token at `start`": where two word tokens
/// touch with no separator (adjacent CJK characters, which UAX#29 splits into one-char tokens) a
/// boundary offset is ambiguous, and guessing there would rewrite the neighbouring character.
fn word_spanning(doc: &TextDocument, start: usize, end: usize) -> Option<String> {
    for block in doc.blocks() {
        let base = block.position();
        let text = block.text();
        let block_end = base + text.chars().count();
        if start < base || end > block_end {
            continue; // not (wholly) in this block — same-block scope
        }
        for (char_off, len, word) in word_positions(&text) {
            let wstart = base + char_off;
            if wstart == start && wstart + len == end {
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
    words
        .into_iter()
        .filter(|w| seen.insert(w.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spellcheck::SpellChecker;
    use std::rc::Rc;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::text_document::Color;
    use teksilo::widgets::rich_text::RichTextEditor;

    fn doc(text: &str) -> TextDocument {
        let d = TextDocument::new();
        d.set_plain_text(text).unwrap();
        d
    }

    /// A live editor over `text` plus a spell session that knows `hello`/`world` and holds
    /// `Skribisto` as a personal word — so `wrld` is flagged by the dictionary and `skribisto`
    /// is flagged by exact-case matching.
    ///
    /// The `WidgetTree` is returned and must be kept alive: the handle reads the editor's state,
    /// and the editor is owned by the tree. Laid out once so the cursor APIs have a real layout.
    fn editor(text: &str) -> (TextDocument, EditorHandle, Rc<SpellSession>, WidgetTree) {
        let d = doc(text);
        let ed = RichTextEditor::editor(d.clone());
        let handle = ed.handle();
        let mut tree = WidgetTree::new();
        tree.add(ed);
        tree.layout(teksilo::prelude::SizeProposal::exact(600.0, 400.0));

        let spell = SpellSession::new(&d);
        spell.set_checker(
            Some(SpellChecker::from_word_lists(
                &["hello", "world"],
                &["Skribisto"],
            )),
            Color::rgb(220, 50, 50),
        );
        (d, handle, spell, tree)
    }

    // ── resolve_spelling (the whole menu group, against a live editor) ──

    #[test]
    fn caret_in_a_flagged_word_offers_it_for_correction_and_for_adding() {
        let (d, handle, spell, _tree) = editor("hello wrld");
        handle.select_range(8, 8); // caret inside "wrld", no selection
        let menu = resolve_spelling(&d, &handle, Some(&spell));

        assert_eq!(menu.words, ["wrld"], "the flagged word is addable");
        let c = menu
            .correction
            .expect("a flagged word under the caret is correctable");
        assert_eq!(
            (c.start, c.end),
            (6, 10),
            "the span the replacement rewrites"
        );
        assert!(
            c.suggestions.contains(&"world".to_string()),
            "the dictionary corrects it, got {:?}",
            c.suggestions
        );
    }

    /// The double-click gesture. Regression guard: a selection used to make the correction
    /// resolve to `None`, so double-click-then-right-click silently offered no corrections at
    /// all — and `reposition_caret_for_context_menu` keeps that selection alive by design.
    #[test]
    fn a_selection_spanning_exactly_one_word_is_still_correctable() {
        let (d, handle, spell, _tree) = editor("hello wrld");
        handle.select_range(6, 10); // exactly "wrld", as a double-click selects it
        let menu = resolve_spelling(&d, &handle, Some(&spell));

        let c = menu
            .correction
            .expect("one selected word is a correction target");
        assert_eq!((c.start, c.end), (6, 10));
        assert!(c.suggestions.contains(&"world".to_string()));
        assert_eq!(menu.words, ["wrld"]);
    }

    /// A backwards selection (dragged right-to-left) puts the anchor after the caret; the span
    /// must still resolve.
    #[test]
    fn a_backwards_selection_of_one_word_resolves_the_same() {
        let (d, handle, spell, _tree) = editor("hello wrld");
        handle.select_range(10, 6); // anchor 10, caret 6
        let menu = resolve_spelling(&d, &handle, Some(&spell));
        let c = menu.correction.expect("direction must not matter");
        assert_eq!((c.start, c.end), (6, 10));
    }

    /// The broad-drag guard must not be escapable by landing exactly on a token's edges. One
    /// "word" can be huge — a pasted hash or URL with no internal break is a single UAX#29 token,
    /// which a double-click selects whole.
    #[test]
    fn an_oversized_single_word_selection_still_hits_the_broad_drag_cap() {
        let long = "a".repeat(MAX_SELECTION_CHARS + 80);
        let (d, handle, spell, _tree) = editor(&long);
        handle.select_range(0, long.chars().count());
        let menu = resolve_spelling(&d, &handle, Some(&spell));
        assert!(
            menu.correction.is_none(),
            "a {}-char token is a broad drag, not a correction target",
            long.len()
        );
        assert!(menu.words.is_empty(), "and it is not whitelistable either");
    }

    #[test]
    fn a_multi_word_selection_adds_but_does_not_correct() {
        let (d, handle, spell, _tree) = editor("wrld helo");
        handle.select_range(0, 9); // both words
        let menu = resolve_spelling(&d, &handle, Some(&spell));

        assert!(
            menu.correction.is_none(),
            "no single target, so nothing to correct"
        );
        assert_eq!(
            menu.words,
            ["wrld", "helo"],
            "both flagged words are addable"
        );
    }

    #[test]
    fn a_correctly_spelled_word_offers_nothing() {
        let (d, handle, spell, _tree) = editor("hello wrld");
        handle.select_range(2, 2); // caret inside "hello"
        let menu = resolve_spelling(&d, &handle, Some(&spell));
        assert!(menu.correction.is_none());
        assert!(menu.words.is_empty(), "a good word is never re-offered");
    }

    #[test]
    fn without_a_checker_nothing_is_offered() {
        let (d, handle, _spell, _tree) = editor("hello wrld");
        handle.select_range(8, 8);
        let menu = resolve_spelling(&d, &handle, None);
        assert!(menu.correction.is_none());
        assert!(menu.words.is_empty(), "no checker ⇒ nothing is 'wrong'");
    }

    /// The personal-dictionary payoff, end to end through the resolver: the stored casing is
    /// offered for a word only the project knows.
    #[test]
    fn a_personal_word_typed_in_the_wrong_case_is_correctable() {
        let (d, handle, spell, _tree) = editor("hello skribisto");
        handle.select_range(9, 9); // caret inside "skribisto"
        let menu = resolve_spelling(&d, &handle, Some(&spell));

        let c = menu.correction.expect("exact-case matching flags it");
        assert_eq!(menu.words, ["skribisto"]);
        assert_eq!(
            c.suggestions.first().map(String::as_str),
            Some("Skribisto"),
            "the project's own casing leads, got {:?}",
            c.suggestions
        );
    }

    #[test]
    fn a_caret_outside_any_word_offers_nothing() {
        let (d, handle, spell, _tree) = editor("hello wrld");
        handle.select_range(5, 5); // on the space — but inclusive edges make this "hello"
        let menu = resolve_spelling(&d, &handle, Some(&spell));
        assert!(
            menu.words.is_empty(),
            "'hello' is correct, so nothing is offered"
        );

        let (d2, handle2, spell2, _tree2) = editor("12 345");
        handle2.select_range(1, 1);
        let menu2 = resolve_spelling(&d2, &handle2, Some(&spell2));
        assert!(menu2.correction.is_none(), "digits are never correctable");
        assert!(menu2.words.is_empty());
    }

    /// Just the word, for the cases that don't care about the span.
    fn word_at(d: &TextDocument, pos: usize) -> Option<String> {
        word_range_at(d, pos).map(|(_, _, w)| w)
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
    fn word_spanning_matches_only_an_exact_whole_word() {
        let d = doc("Hello wrld today");
        // The double-click gesture: the selection covers "wrld" exactly.
        assert_eq!(word_spanning(&d, 6, 10).as_deref(), Some("wrld"));
        assert_eq!(word_spanning(&d, 0, 5).as_deref(), Some("Hello"));
        // A partial word, a span with a trailing space, and a two-word span all decline.
        assert_eq!(word_spanning(&d, 6, 9), None, "partial word");
        assert_eq!(
            word_spanning(&d, 6, 11),
            None,
            "word plus the following space"
        );
        assert_eq!(word_spanning(&d, 0, 10), None, "two words");
        assert_eq!(word_spanning(&doc("12 345"), 0, 2), None, "not word-like");
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
    fn word_range_at_reports_the_span_a_replacement_needs() {
        let d = doc("Hello wrld today");
        // "wrld" spans chars [6, 10) — the exact range `replace_range` must rewrite.
        assert_eq!(
            word_range_at(&d, 8),
            Some((6, 10, "wrld".to_string())),
            "the caret inside a word resolves its span"
        );
        assert_eq!(word_range_at(&d, 0), Some((0, 5, "Hello".to_string())));
        // Non-word runs resolve to nothing to replace.
        assert_eq!(word_range_at(&doc("12 345"), 1), None);
    }

    #[test]
    fn word_range_at_is_document_absolute_and_char_based() {
        // Second block: offsets carry `block.position()`, and the accented first
        // block must not shift them by its extra *bytes*.
        let d = doc("café\nwrld");
        // Block 2 starts at char 5 (4 chars + the 1-char block gap).
        assert_eq!(
            word_range_at(&d, 6),
            Some((5, 9, "wrld".to_string())),
            "char offsets, absolute across blocks"
        );
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
