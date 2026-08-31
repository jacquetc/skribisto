// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **The three pieces of a result's snippet**, cut around one occurrence.
//!
//! Its own module because two use cases cut snippets and they must cut them the
//! same way. `run_search` cuts one per matching field, for the row a writer scans;
//! `occurrences_for_result` cuts one per occurrence inside a field, for the rows
//! under it when they open that row. A search whose second level quoted its matches
//! differently from its first would look like two different searches.
//!
//! Extracted rather than shared by one use case calling the other: this codebase
//! does not let a use case call a use case, and `replace_in_project` records the
//! reason against its own copy of a different chain -- "two copies of that chain
//! would drift, and a writer would meet the difference".

/// How much text either side of a match a snippet carries, in **chars**.
const SNIPPET_CONTEXT: usize = 60;

/// How many words of run-up a match keeps when its sentence is cut short.
///
/// Past this the excerpt starts near the match with a leading ellipsis instead of
/// at the sentence's first word. Two, because the row is one line in a dock a
/// writer has narrowed to give the manuscript the width: what has to survive is
/// the match and enough after it to tell one hit from another, and a long run-up
/// buys neither at the cost of both.
const LEAD_WORDS: usize = 2;

/// Where the sentence containing `at` begins, in chars.
///
/// A sentence ends at `.`, `?`, `!`, `…` or a line break, and the next one begins
/// after the spaces that follow. Deliberately naive about abbreviations: mistaking
/// "Mr. Ardel" for two sentences costs an excerpt two words of run-up, where
/// carrying a per-language abbreviation list would cost a resource this crate has
/// no business owning -- the same reasoning that keeps a stopword list out of the
/// analysis this application does elsewhere.
fn sentence_start(text: &str, at: usize) -> usize {
    let mut start = 0usize;
    let mut pending: Option<usize> = None;
    for (i, ch) in text.chars().enumerate() {
        if i >= at {
            break;
        }
        match ch {
            '.' | '?' | '!' | '…' | '\n' | '\r' => pending = Some(i + 1),
            // The sentence begins at the first thing that is not the space after
            // the stop -- and a closing quote or bracket belongs to the sentence
            // that ended, not the one starting.
            c if c.is_whitespace() || matches!(c, '"' | '\'' | '»' | '”' | '’' | ')') => {
                if let Some(p) = pending {
                    pending = Some(p.max(i + 1));
                }
            }
            _ => {
                if pending.take().is_some() {
                    start = i;
                }
            }
        }
    }
    start
}

/// The char index `words` words back from `at`, but never before `floor`.
///
/// Counts word *beginnings* walking backwards, so "two words before" is where the
/// second word back starts rather than wherever two spaces happen to fall -- and
/// exactly `words` of them are kept, not one more.
fn back_words(text: &str, at: usize, words: usize, floor: usize) -> usize {
    let chars: Vec<(usize, char)> = text
        .chars()
        .enumerate()
        .skip(floor)
        .take_while(|(i, _)| *i < at)
        .collect();
    let mut seen = 0usize;
    let mut i = chars.len();
    while i > 0 {
        i -= 1;
        let (idx, ch) = chars[i];
        let prev_is_space = i == 0 || chars[i - 1].1.is_whitespace();
        if !ch.is_whitespace() && prev_is_space {
            seen += 1;
            if seen >= words {
                return idx;
            }
        }
    }
    floor
}

/// `SNIPPET_CONTEXT` chars either side of the match, on char boundaries.
///
/// Walks `char_indices` rather than collecting the field into a `Vec<char>`. The
/// difference is not cosmetic: this runs for every matching row, up to `RESULT_CAP`, and a
/// manuscript of long scenes would otherwise materialise the *whole scene* at four bytes
/// per char — tens of megabytes of transient garbage, on the UI thread — to produce a
/// snippet of a couple of hundred characters.
pub fn cut(text: &str, char_start: usize, char_len: usize) -> (String, String, String) {
    let end = char_start + char_len;
    // **Start at a sentence, not at a character count.** An excerpt cut sixty
    // characters back begins mid-word in the middle of the previous sentence, which
    // a reader has to parse before they can judge the hit. Starting where the
    // sentence starts is the difference between reading a result and decoding one.
    let sentence = sentence_start(text, char_start);
    let lead = back_words(text, char_start, LEAD_WORDS, sentence);
    // A match deep in a long sentence keeps only its last few words of run-up, and
    // says so with a leading ellipsis; one near the opening keeps the opening.
    let elided = lead > sentence;
    let from = lead.max(char_start.saturating_sub(SNIPPET_CONTEXT));
    let to = end + SNIPPET_CONTEXT;

    // One pass: the byte offset of each of the four char positions that bound the three
    // pieces. Any that lies past the end of the text simply never gets set, and falls back
    // to the end — which is what `.min(len)` did before, without the allocation.
    let (mut b_from, mut b_start, mut b_end, mut b_to) = (None, None, None, None);
    for (i, (byte, _)) in text.char_indices().enumerate() {
        if i == from {
            b_from = Some(byte);
        }
        if i == char_start {
            b_start = Some(byte);
        }
        if i == end {
            b_end = Some(byte);
        }
        if i == to {
            b_to = Some(byte);
            break;
        }
    }
    let len = text.len();
    let (b_from, b_start) = (b_from.unwrap_or(len), b_start.unwrap_or(len));
    let (b_end, b_to) = (b_end.unwrap_or(len), b_to.unwrap_or(len));

    let before = if elided {
        format!("…{}", &text[b_from..b_start])
    } else {
        text[b_from..b_start].to_string()
    };
    (
        before,
        text[b_start..b_end].to_string(),
        text[b_end..b_to].to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Where a hit sits, given a marker in the text.
    fn at(text: &str, needle: &str) -> (usize, usize) {
        let byte = text.find(needle).expect("needle");
        (text[..byte].chars().count(), needle.chars().count())
    }

    /// **The excerpt begins where the sentence does.** Cut by character count it
    /// would begin mid-word in the sentence before, which a reader has to unpick
    /// before they can judge the hit.
    #[test]
    fn an_excerpt_starts_at_its_sentence() {
        let text = "The ferry was late. She saw ice everywhere.";
        let (start, len) = at(text, "ice");
        let (before, matched, _) = cut(text, start, len);
        assert_eq!(matched, "ice");
        assert_eq!(
            before, "She saw ",
            "the sentence's own opening, and not a word of the one before it"
        );
    }

    /// A match within four words of the opening keeps the opening whole, and says
    /// nothing about it: there is nothing elided to announce.
    #[test]
    fn a_match_near_the_opening_keeps_it_and_takes_no_ellipsis() {
        let text = "Devon raised the blaster slowly.";
        let (start, len) = at(text, "the");
        let (before, matched, _) = cut(text, start, len);
        assert_eq!(matched, "the");
        assert_eq!(
            before, "Devon raised ",
            "two words in, so the opening is whole"
        );
        assert!(!before.starts_with('…'));
    }

    /// **Deep in a long sentence, the run-up is cut and marked.** Keeping every
    /// word from the sentence's start would push the match itself off the end of a
    /// narrow dock, which is the one thing the row exists to show.
    #[test]
    fn a_match_deep_in_a_sentence_is_leading_ellipsed() {
        let text = "She had walked for days across the frozen sea before she saw the ice.";
        let (start, len) = at(text, "ice.");
        let (before, matched, _) = cut(text, start, len);
        assert_eq!(matched, "ice.");
        assert!(before.starts_with('…'), "got {before:?}");
        assert!(
            before.ends_with("saw the "),
            "the last two words are kept: {before:?}"
        );
        assert!(
            !before.contains("walked") && !before.contains("she saw"),
            "and no more than two: {before:?}"
        );
    }

    /// The sentence is found through `?` and `!` as well, and through a line break
    /// — a scene's paragraphs are separated by one and the previous paragraph is
    /// not run-up.
    #[test]
    fn a_sentence_ends_at_any_of_its_terminators() {
        for text in [
            "Where is the Captain? The ice was gone.",
            "Let him talk! The ice was gone.",
            "He said nothing\nThe ice was gone.",
        ] {
            let (start, len) = at(text, "ice");
            let (before, _, _) = cut(text, start, len);
            assert_eq!(before, "The ", "got {before:?} for {text:?}");
        }
    }

    /// A closing quote belongs to the sentence that ended, not to the one starting
    /// — dialogue would otherwise begin every excerpt with someone else's
    /// punctuation.
    #[test]
    fn a_sentence_starting_after_dialogue_skips_the_closing_quote() {
        let text = "\u{201c}He is gone.\u{201d} The ice held her weight.";
        let (start, len) = at(text, "ice");
        let (before, _, _) = cut(text, start, len);
        assert_eq!(before, "The ", "got {before:?}");
    }

    /// The snippet is cut on **char** boundaries, from text that is full of multi-byte
    /// chars — slicing it by byte would panic in the middle of an `é`.
    #[test]
    fn a_snippet_is_cut_on_char_boundaries() {
        let text = "Aurélien traversa la forêt où l'ombre s'étirait.";
        let hit = text.chars().collect::<Vec<_>>();
        let start = 21; // "forêt"
        assert_eq!(hit[start..start + 5].iter().collect::<String>(), "forêt");

        let (before, matched, after) = cut(text, start, 5);
        assert_eq!(matched, "forêt");
        // Three words of run-up is more than the two a cut sentence keeps, so the
        // first is dropped and said to be dropped.
        assert_eq!(before, "…traversa la ");
        assert_eq!(after, " où l'ombre s'étirait.");
        assert_eq!(
            format!("{}{matched}{after}", before.trim_start_matches('…')),
            &text[text.find("traversa").unwrap()..],
            "the pieces reassemble the text from where the excerpt starts"
        );
    }

    /// A match at the very start and at the very end — the two places an off-by-one in the
    /// byte-offset walk would show up as a panic or a truncated snippet.
    #[test]
    fn a_snippet_at_either_edge_of_the_text() {
        let text = "Élena rentra chez ellé";

        let (before, matched, after) = cut(text, 0, 5);
        assert_eq!(before, "");
        assert_eq!(matched, "Élena");
        assert_eq!(after, " rentra chez ellé");

        let n = text.chars().count();
        let (before, matched, after) = cut(text, n - 4, 4);
        assert_eq!(matched, "ellé");
        assert_eq!(after, "", "nothing follows the last char");
        assert_eq!(before, "…rentra chez ", "two words of run-up, and a mark");
    }

    /// Context is clamped to `SNIPPET_CONTEXT` chars either side, not bytes — so an accented
    /// scene does not get a shorter snippet than an ASCII one.
    #[test]
    fn a_snippet_is_clamped_to_the_context_in_chars() {
        let text = format!("{}CIBLE{}", "é".repeat(200), "à".repeat(200));
        let (before, matched, after) = cut(&text, 200, 5);
        assert_eq!(matched, "CIBLE");
        assert_eq!(before.chars().count(), SNIPPET_CONTEXT);
        assert_eq!(after.chars().count(), SNIPPET_CONTEXT);
    }

    /// The whole text is shorter than the context window: take what there is, and do not run
    /// off the end.
    #[test]
    fn a_snippet_of_a_text_shorter_than_its_context() {
        let (before, matched, after) = cut("où", 0, 2);
        assert_eq!(
            (before.as_str(), matched.as_str(), after.as_str()),
            ("", "où", "")
        );
    }
}
