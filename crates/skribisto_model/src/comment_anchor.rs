// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where a comment points, and how it keeps pointing there.
//!
//! Everything here is a **pure function over plain data** — no widgets, no
//! document, no store — so the rules that decide whether a writer's note survives
//! a rewrite are table-testable in isolation. That is deliberate: this is the one
//! part of the feature where being subtly wrong is invisible until someone's
//! comment has silently moved to the wrong sentence.
//!
//! ## Why an anchor is a *quote*, not an offset
//!
//! Nothing in this stack has durable text identity. `text-document` re-mints block
//! ids from 1 on every reopen, `load_work` re-mints every `EntityId`,
//! `CharacterFormat` is a closed struct whose runs may not overlap, and the Djot
//! importer discards inline span attributes. The only thing that survives a
//! save→reopen is the prose text itself.
//!
//! So an anchor is a W3C-Web-Annotation-style **selector**: a quote
//! (prefix / exact / suffix) plus a position hint. The hint alone is right far more
//! often than it sounds — the Djot round-trip is a property-tested fixpoint that
//! preserves plain text and block count, so offsets are stable across save→reopen
//! whenever the prose is unedited. The quote is what rescues the case where it was
//! edited, including by git or by hand outside the app.
//!
//! ## Two coordinate spaces, one of which is a trap
//!
//! Offsets here are **document-absolute CHARACTER offsets** — the space
//! `TextCursor`, `FindMatch` and `DocumentEvent::ContentsChanged` all speak. They
//! are *not* the byte offsets `FormatRun` uses internally, and mixing the two
//! corrupts an anchor on any paragraph containing a non-ASCII character.

use common::entities::CommentOrphanReason;

/// How many characters of context to capture on each side of a quote.
///
/// Long enough to disambiguate a repeated phrase in ordinary prose, short enough
/// that a nearby edit does not routinely invalidate it. Both ends are captured
/// because either alone is defeated by a common case: a repeated line start
/// ("He said,") needs the suffix, a repeated line end needs the prefix.
pub const CONTEXT_CHARS: usize = 32;

/// Above this length a quote is stored as head + tail rather than verbatim, so one
/// comment on a very long selection cannot bloat the sidecar. The matcher then
/// compares the two ends instead of the whole string.
pub const MAX_EXACT_CHARS: usize = 300;
const TRUNCATED_SIDE: usize = 150;

/// The persisted selector for one comment.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Anchor {
    /// Position hint, in document-absolute char offsets.
    pub start: usize,
    pub length: usize,
    pub prefix: String,
    pub exact: String,
    /// When set, `exact` is `head…tail`, not a literal substring.
    pub exact_truncated: bool,
    pub suffix: String,
    /// 0-based index of the block containing `start` when this last resolved.
    /// A tie-breaker, and the fallback for a paragraph comment — never a block
    /// `EntityId`, which is re-minted on every reopen and would be meaningless.
    pub block_ordinal: usize,
    /// How many consecutive blocks a paragraph comment covers (1 for a single
    /// paragraph, and for every range comment).
    ///
    /// A *count*, not an end index: block indices shift when a paragraph is added
    /// above, but "this comment covers three paragraphs" survives that untouched.
    pub block_span: usize,
}

/// What a re-anchor pass concluded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// The anchor still points at real text, at these (possibly updated) offsets.
    Anchored { start: usize, length: usize },
    /// It does not, and this is why. Always actionable in the UI — never a silent
    /// disappearance.
    Orphan(CommentOrphanReason),
}

impl Resolution {
    pub fn is_anchored(&self) -> bool {
        matches!(self, Resolution::Anchored { .. })
    }
}

// ---------------------------------------------------------------------------
// Live tracking: the exclusive-at-both-edges shift rule
// ---------------------------------------------------------------------------

/// Move a comment's `[start, end)` across one edit, keeping both edges
/// **exclusive** — text typed at either boundary lands *outside* the comment.
///
/// This matches Word and Google Docs, and it is why `TextCursor` cannot be used to
/// do this for us: `adjust_cursors` applies one **left-sticky** rule to both
/// endpoints (`offset <= edit_pos` → unchanged), and `CursorData` carries no
/// sticky-side field. Under that rule an insertion at a comment's *start* leaves
/// `start` put while pushing `end` along, so the comment silently swallows the new
/// text. The two endpoints genuinely need different rules:
///
/// * **start is right-sticky** — an insertion exactly at `start` pushes it forward,
///   so the new text sits before the comment. If the start itself was deleted, it
///   resumes *after* whatever replaced it.
/// * **end is left-sticky** — an insertion exactly at `end` leaves it put, so the
///   new text sits after the comment. If the end was deleted, it stops *before* the
///   replacement.
///
/// Both "was deleted" branches therefore differ from text-document's own
/// `adjust_offset`, which clamps both endpoints to the same point (`p + a`).
///
/// `position` / `chars_removed` / `chars_added` come straight from
/// `DocumentEvent::ContentsChanged`, which text-document also emits on undo and
/// redo (computed as a real text diff) — so undo/redo tracking comes along free.
///
/// A returned empty range means the comment's text is gone: the caller flags it
/// orphaned on the spot rather than waiting for the next load.
pub fn shift_range(
    start: usize,
    end: usize,
    position: usize,
    chars_removed: usize,
    chars_added: usize,
) -> (usize, usize) {
    let removed_end = position + chars_removed;

    let new_start = if start < position {
        start
    } else if start <= removed_end {
        position + chars_added
    } else {
        start - chars_removed + chars_added
    };

    let new_end = if end <= position {
        end
    } else if end <= removed_end {
        position
    } else {
        end - chars_removed + chars_added
    };

    (new_start, new_end.max(new_start))
}

/// A quote as it should be *shown* to the writer.
///
/// An inline image is one `U+FFFC` in the document text, so a comment on a
/// passage containing one captures it — and must, or the quote would no longer
/// match the prose it came from and the comment would orphan itself the first
/// time it was re-resolved. But the character has no glyph, so a list or a card
/// that printed it verbatim would show an empty box in the middle of the
/// sentence. Substituted only at the point of display, never in what is stored.
pub fn for_display(quote: &str) -> String {
    if quote.contains('\u{FFFC}') {
        quote.replace('\u{FFFC}', "🖼")
    } else {
        quote.to_string()
    }
}

// ---------------------------------------------------------------------------
// Capture
// ---------------------------------------------------------------------------

/// Build a selector for `[start, end)` of `text`.
///
/// `text` is the document's plain text; `block_ordinal` is the index of the block
/// containing `start`. Offsets are clamped so a caller cannot capture out of range.
pub fn capture(text: &str, start: usize, end: usize, block_ordinal: usize) -> Anchor {
    let chars: Vec<char> = text.chars().collect();
    let start = start.min(chars.len());
    let end = end.clamp(start, chars.len());

    let exact_chars = &chars[start..end];
    let (exact, exact_truncated) = if exact_chars.len() > MAX_EXACT_CHARS {
        let head: String = exact_chars[..TRUNCATED_SIDE].iter().collect();
        let tail: String = exact_chars[exact_chars.len() - TRUNCATED_SIDE..]
            .iter()
            .collect();
        (format!("{head}…{tail}"), true)
    } else {
        (exact_chars.iter().collect::<String>(), false)
    };

    Anchor {
        start,
        length: end - start,
        block_span: 1,
        prefix: chars[start.saturating_sub(CONTEXT_CHARS)..start]
            .iter()
            .collect(),
        exact,
        exact_truncated,
        suffix: chars[end..(end + CONTEXT_CHARS).min(chars.len())]
            .iter()
            .collect(),
        block_ordinal,
    }
}

/// The `[start, end)` of the blocks `first..=last`, given every block's start offset.
///
/// A paragraph comment stores only its block span and re-derives the character
/// extent on every resolve, so a paragraph that grows or shrinks while the writer
/// works stays exactly covered rather than keeping a frozen length.
///
/// **The end excludes the block separator.** A block's successor starts one
/// character *after* it, and that character is the paragraph break — so ending at
/// the successor's start puts the extent's last position on the following line,
/// and a bracket drawn from it hangs one line below the paragraph it marks.
pub fn block_extent(
    block_starts: &[usize],
    total_chars: usize,
    first: usize,
    last: usize,
) -> (usize, usize) {
    if block_starts.is_empty() {
        return (0, total_chars);
    }
    let hi = block_starts.len() - 1;
    let (first, last) = (first.min(hi), last.min(hi).max(first.min(hi)));
    let start = block_starts[first];
    let end = match block_starts.get(last + 1) {
        // Step back over the separator that begins the next block.
        Some(&next) => next.saturating_sub(1),
        None => total_chars,
    };
    (start, end.max(start))
}

/// Which block contains `offset`.
pub fn block_of(block_starts: &[usize], offset: usize) -> usize {
    match block_starts.binary_search(&offset) {
        Ok(i) => i,
        Err(0) => 0,
        Err(i) => i - 1,
    }
}

// ---------------------------------------------------------------------------
// Resolve: the three tiers
// ---------------------------------------------------------------------------

/// Re-anchor `anchor` against `text`.
///
/// Tier 1 — the stored offset still holds. Guaranteed to hit when the prose is
/// unedited, because the Djot round-trip preserves plain text exactly.
/// Tier 2 — search for the quote and disambiguate by context, then by block
/// proximity. A **tie is reported as `Ambiguous`, never guessed**: a comment
/// silently attached to the wrong sentence is worse than one that admits it is lost.
/// Tier 3 — a paragraph comment falls back to its block ordinal, which still means
/// something even when the wording changed completely.
pub fn resolve(
    text: &str,
    anchor: &Anchor,
    is_paragraph: bool,
    block_starts: &[usize],
) -> Resolution {
    let chars: Vec<char> = text.chars().collect();

    if is_paragraph {
        // A paragraph comment's extent is always re-derived, never trusted from
        // storage. Locate the first block by quote (the wording is the better
        // signal), fall back to the stored ordinal, and keep the stored *span* so
        // a comment over several paragraphs still covers all of them.
        let first = match find_unique(&chars, anchor) {
            Found::One(at) => block_of(block_starts, at),
            _ => anchor.block_ordinal,
        };
        if block_starts.is_empty() {
            // No blocks to derive an extent from.
            //
            // With text present this used to fall through to `block_extent`,
            // whose empty-input fast path answers `(0, total_chars)` — so one
            // paragraph comment silently claimed the entire manuscript. Nor can
            // it be called an orphan: this verdict is written back to the store,
            // and reporting a comment lost because the block index happened to
            // be unavailable would be a worse lie than the one it replaces.
            //
            // Keep what was stored instead. It is the only answer that neither
            // invents an extent nor destroys one.
            if chars.is_empty() {
                return Resolution::Orphan(CommentOrphanReason::TextNotFound);
            }
            let start = anchor.start.min(chars.len());
            return Resolution::Anchored {
                start,
                length: anchor.length.min(chars.len() - start),
            };
        }
        let last = first + anchor.block_span.saturating_sub(1);
        let (s, e) = block_extent(block_starts, chars.len(), first, last);
        return Resolution::Anchored {
            start: s,
            length: e - s,
        };
    }

    // Tier 1: the hint is still exactly right.
    if matches_at(&chars, anchor, anchor.start) {
        return Resolution::Anchored {
            start: anchor.start,
            length: anchor.length,
        };
    }

    // Tier 2: search + disambiguate.
    match find_unique(&chars, anchor) {
        Found::One(at) => Resolution::Anchored {
            start: at,
            length: anchor.length.min(chars.len().saturating_sub(at)),
        },
        Found::Ambiguous => Resolution::Orphan(CommentOrphanReason::Ambiguous),
        Found::None => Resolution::Orphan(CommentOrphanReason::TextNotFound),
    }
}

enum Found {
    One(usize),
    Ambiguous,
    None,
}

/// Does the anchored text sit at `at`?
fn matches_at(chars: &[char], anchor: &Anchor, at: usize) -> bool {
    let end = at + anchor.length;
    if end > chars.len() {
        return false;
    }
    if anchor.exact_truncated {
        // Compare the two captured ends rather than the whole span.
        //
        // Split by the *known* head length, not by searching for the `…`
        // separator `capture` joined them with. The head and tail are real
        // prose, and prose contains ellipses — "the sentence trailed off…" —
        // so `split_once('…')` finds whichever one comes first and hands back a
        // head and tail that were never captured. The comparison then fails
        // against text nobody edited, and the comment is reported lost.
        //
        // `capture` always writes exactly `TRUNCATED_SIDE` characters either
        // side of one separator character, so the split point is arithmetic and
        // needs no searching.
        let exact_chars: Vec<char> = anchor.exact.chars().collect();
        if exact_chars.len() != TRUNCATED_SIDE * 2 + 1 {
            return false;
        }
        let head: String = exact_chars[..TRUNCATED_SIDE].iter().collect();
        let tail: String = exact_chars[TRUNCATED_SIDE + 1..].iter().collect();
        let head_n = TRUNCATED_SIDE;
        let tail_n = TRUNCATED_SIDE;
        if head_n + tail_n > anchor.length {
            return false;
        }
        let got_head: String = chars[at..at + head_n].iter().collect();
        let got_tail: String = chars[end - tail_n..end].iter().collect();
        got_head == head && got_tail == tail
    } else {
        let got: String = chars[at..end].iter().collect();
        got == anchor.exact
    }
}

/// Every position where the quote matches, scored by how much surrounding context
/// still agrees; a single clear winner wins.
fn find_unique(chars: &[char], anchor: &Anchor) -> Found {
    if anchor.length == 0 || anchor.length > chars.len() {
        return Found::None;
    }
    let mut best: Vec<(usize, usize)> = Vec::new(); // (score, position)
    let mut best_score = 0usize;

    for at in 0..=(chars.len() - anchor.length) {
        if !matches_at(chars, anchor, at) {
            continue;
        }
        let score = context_score(chars, anchor, at);
        if best.is_empty() || score > best_score {
            best_score = score;
            best.clear();
            best.push((score, at));
        } else if score == best_score {
            best.push((score, at));
        }
    }

    match best.len() {
        0 => Found::None,
        1 => Found::One(best[0].1),
        _ => {
            // Several candidates agree on context. Break the tie by block
            // proximity — the writer's comment is far more likely to be near where
            // it was than somewhere else in the document.
            let target = anchor.start;
            let mut by_distance = best.clone();
            by_distance.sort_by_key(|(_, at)| at.abs_diff(target));
            let closest = by_distance[0].1.abs_diff(target);
            let contenders = by_distance
                .iter()
                .filter(|(_, at)| at.abs_diff(target) == closest)
                .count();
            if contenders == 1 {
                Found::One(by_distance[0].1)
            } else {
                // Genuinely indistinguishable. Refuse to guess.
                Found::Ambiguous
            }
        }
    }
}

/// How many characters of the captured prefix and suffix still match at `at`.
fn context_score(chars: &[char], anchor: &Anchor, at: usize) -> usize {
    let prefix: Vec<char> = anchor.prefix.chars().collect();
    let suffix: Vec<char> = anchor.suffix.chars().collect();

    let mut score = 0;
    // Walk backwards from the match start through the captured prefix.
    for (i, want) in prefix.iter().rev().enumerate() {
        if at < i + 1 {
            break;
        }
        if chars[at - i - 1] == *want {
            score += 1;
        } else {
            break;
        }
    }
    let end = at + anchor.length;
    for (i, want) in suffix.iter().enumerate() {
        if end + i >= chars.len() {
            break;
        }
        if chars[end + i] == *want {
            score += 1;
        } else {
            break;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── the shift rule ──────────────────────────────────────────────────────
    //
    // Table-driven, because the whole point of the exclusive-edge decision is the
    // boundary cases, and those are exactly what an example-by-example test misses.

    #[test]
    fn an_edit_entirely_before_the_comment_shifts_it_wholesale() {
        // "abc[def)" -> insert 2 at 0
        assert_eq!(shift_range(3, 6, 0, 0, 2), (5, 8));
    }

    #[test]
    fn an_edit_entirely_after_the_comment_leaves_it_alone() {
        assert_eq!(shift_range(3, 6, 10, 0, 5), (3, 6));
    }

    #[test]
    fn an_insertion_strictly_inside_extends_the_comment() {
        // The one case where a comment SHOULD grow: the writer is elaborating
        // within the very text they annotated.
        assert_eq!(shift_range(3, 6, 4, 0, 3), (3, 9));
    }

    #[test]
    fn typing_at_the_start_boundary_stays_outside_the_comment() {
        // The decision: exclusive at both edges (Word / Google Docs behaviour).
        // A left-sticky rule applied to both endpoints — which is what
        // `TextCursor` does — would give (3, 9) here and swallow the new text.
        let (s, e) = shift_range(3, 6, 3, 0, 3);
        assert_eq!((s, e), (6, 9));
        assert_eq!(e - s, 3, "length is unchanged; the comment merely moved");
    }

    #[test]
    fn typing_at_the_end_boundary_stays_outside_the_comment() {
        let (s, e) = shift_range(3, 6, 6, 0, 4);
        assert_eq!((s, e), (3, 6));
        assert_eq!(e - s, 3);
    }

    #[test]
    fn deleting_the_head_of_the_comment_resumes_after_the_replacement() {
        // Replace [2,4) with 1 char. The comment started at 3, inside that range.
        let (s, e) = shift_range(3, 8, 2, 2, 1);
        assert_eq!(s, 3, "start resumes after the inserted replacement");
        assert_eq!(e, 7, "end shifts by the -1 delta");
    }

    #[test]
    fn deleting_the_tail_of_the_comment_stops_before_the_replacement() {
        // Replace [5,9) with 2 chars. The comment ended at 8, inside that range.
        let (s, e) = shift_range(3, 8, 5, 4, 2);
        assert_eq!(s, 3);
        assert_eq!(
            e, 5,
            "end stops before the new text rather than absorbing it"
        );
    }

    #[test]
    fn deleting_the_whole_comment_collapses_it_to_zero_length() {
        let (s, e) = shift_range(3, 8, 0, 20, 0);
        assert_eq!(e, s, "an empty range is the live-orphan signal");
    }

    #[test]
    fn select_all_and_retype_collapses_the_comment() {
        let (s, e) = shift_range(10, 20, 0, 100, 7);
        assert_eq!(e, s);
    }

    #[test]
    fn the_end_never_precedes_the_start() {
        // Fuzz the rule's invariant over a grid of edits; a start/end inversion
        // would be a panic-shaped bug in every consumer downstream.
        for start in 0..12 {
            for len in 0..8 {
                for pos in 0..16 {
                    for removed in 0..8 {
                        for added in 0..4 {
                            let (s, e) = shift_range(start, start + len, pos, removed, added);
                            assert!(e >= s, "inverted for {start},{len},{pos},{removed},{added}");
                        }
                    }
                }
            }
        }
    }

    // ── images ──────────────────────────────────────────────────────────────

    /// The document text of a scene holding one inline image. The image is a
    /// single `U+FFFC`, which is what makes every offset after it depend on the
    /// picture still being counted.
    const WITH_IMAGE: &str = "The lighthouse stood alone. \u{FFFC} The keeper had gone.";

    #[test]
    fn a_comment_after_an_image_lands_on_the_words_it_names() {
        // The regression this guards: the addressable plain text briefly dropped
        // the sentinel while the document kept it, so every anchor past a picture
        // pointed one character early — enough to slice a word in half and, once
        // re-resolved, to orphan a comment that never moved.
        let chars: Vec<char> = WITH_IMAGE.chars().collect();
        let start = WITH_IMAGE.chars().count() - "keeper had gone.".chars().count();
        let end = start + "keeper".chars().count();
        assert_eq!(chars[start..end].iter().collect::<String>(), "keeper");

        let a = capture(WITH_IMAGE, start, end, 0);
        assert_eq!(a.exact, "keeper");
        match resolve(WITH_IMAGE, &a, false, &[0]) {
            Resolution::Anchored { start: at, length } => {
                assert_eq!(chars[at..at + length].iter().collect::<String>(), "keeper");
            }
            other => panic!("a comment after an image must still anchor: {other:?}"),
        }
    }

    #[test]
    fn a_quote_spanning_an_image_keeps_it_and_still_matches() {
        // The sentinel stays in the stored quote. Normalising it out would make
        // the quote something the prose does not contain, so the next resolve
        // would fail to find it and orphan a comment that never moved.
        let start = "The ".chars().count();
        let end = "The lighthouse stood alone. \u{FFFC} The".chars().count();
        let a = capture(WITH_IMAGE, start, end, 0);
        assert!(
            a.exact.contains('\u{FFFC}'),
            "the image left the quote: {:?}",
            a.exact
        );
        assert_eq!(
            resolve(WITH_IMAGE, &a, false, &[0]),
            Resolution::Anchored {
                start,
                length: end - start
            }
        );
    }

    #[test]
    fn a_quote_with_an_image_is_shown_as_a_picture_not_an_empty_box() {
        // Stored verbatim, displayed legibly.
        assert_eq!(for_display("a \u{FFFC} b"), "a 🖼 b");
        assert_eq!(for_display("no image here"), "no image here");
    }

    // ── capture ─────────────────────────────────────────────────────────────

    #[test]
    fn capture_takes_bounded_context_on_both_sides() {
        let text = "The lamp guttered, and then it did not.";
        let a = capture(text, 4, 8, 0);
        assert_eq!(a.exact, "lamp");
        assert_eq!(a.prefix, "The ");
        assert!(a.suffix.starts_with(" guttered"));
        assert_eq!(a.length, 4);
        assert!(!a.exact_truncated);
    }

    #[test]
    fn capture_is_char_indexed_not_byte_indexed() {
        // Every char here is multi-byte. A byte-indexed capture would slice
        // mid-character and either panic or produce mojibake.
        let text = "éàü—ñ";
        let a = capture(text, 1, 3, 0);
        assert_eq!(a.exact, "àü");
        assert_eq!(a.prefix, "é");
        assert_eq!(a.suffix, "—ñ");
    }

    #[test]
    fn a_long_selection_is_captured_as_head_and_tail() {
        let text: String = std::iter::repeat_n('x', 100)
            .chain("MIDDLE".chars())
            .chain(std::iter::repeat_n('y', 400))
            .collect();
        let a = capture(&text, 0, text.chars().count(), 0);
        assert!(a.exact_truncated);
        assert!(a.exact.contains('…'));
        assert!(a.exact.chars().count() < MAX_EXACT_CHARS + 2);
    }

    /// **The regression.** Prose contains ellipses. A truncated quote is stored
    /// as `head…tail`, and the comparison used to recover the two halves by
    /// searching for the first `…` — which, when the captured head itself ended
    /// a sentence with one, is not the separator. The halves came back wrong,
    /// the comparison failed against text nobody had touched, and the comment
    /// was reported lost on the next open.
    #[test]
    fn a_quote_whose_own_prose_contains_an_ellipsis_still_matches_itself() {
        // An ellipsis inside the captured head, well before the separator.
        let text: String = "he trailed off… "
            .chars()
            .chain(std::iter::repeat_n('x', 500))
            .collect();
        let total = text.chars().count();
        let a = capture(&text, 0, total, 0);

        assert!(a.exact_truncated, "the fixture must exercise truncation");
        assert!(
            a.exact.chars().take(TRUNCATED_SIDE).any(|c| c == '…'),
            "the fixture must put a real ellipsis inside the captured head"
        );

        let chars: Vec<char> = text.chars().collect();
        assert!(
            matches_at(&chars, &a, 0),
            "an unedited quote must match itself, ellipsis in the prose or not"
        );
        assert!(
            resolve(&text, &a, false, &[0]).is_anchored(),
            "and must not be reported orphaned"
        );
    }

    /// A paragraph comment with no block index must not swallow the manuscript.
    ///
    /// `block_extent`'s empty-input fast path answers "the whole document",
    /// which is right for measuring and catastrophic as a comment's extent.
    /// Nor may it orphan: the verdict is written back, so a missing block index
    /// would mark real comments lost. It keeps the stored range instead.
    #[test]
    fn a_paragraph_comment_without_blocks_keeps_its_stored_range() {
        let text = "one two three four five";
        let a = Anchor {
            start: 4,
            length: 3,
            exact: "two".into(),
            block_span: 1,
            ..Default::default()
        };
        match resolve(text, &a, true, &[]) {
            Resolution::Anchored { start, length } => {
                assert_eq!(
                    (start, length),
                    (4, 3),
                    "the stored range must survive, not expand to the document"
                );
            }
            other => panic!("expected the stored range, got {other:?}"),
        }
    }

    #[test]
    fn capture_clamps_rather_than_panicking_on_out_of_range_offsets() {
        let a = capture("short", 3, 999, 0);
        assert_eq!(a.exact, "rt");
    }

    // ── resolve ─────────────────────────────────────────────────────────────

    fn scene() -> &'static str {
        "The lamp guttered, and then it did not."
    }

    #[test]
    fn tier1_hits_when_the_prose_is_unedited() {
        let a = capture(scene(), 4, 8, 0);
        assert_eq!(
            resolve(scene(), &a, false, &[0]),
            Resolution::Anchored {
                start: 4,
                length: 4
            }
        );
    }

    #[test]
    fn tier2_relocates_the_quote_after_an_edit_upstream() {
        let a = capture(scene(), 4, 8, 0);
        let edited = format!("Yesterday. {}", scene());
        assert_eq!(
            resolve(&edited, &a, false, &[0]),
            Resolution::Anchored {
                start: 15,
                length: 4
            },
            "the stored offset is stale but the quote still locates it exactly"
        );
    }

    #[test]
    fn deleted_text_orphans_with_text_not_found() {
        let a = capture(scene(), 4, 8, 0);
        assert_eq!(
            resolve("Something else entirely.", &a, false, &[0]),
            Resolution::Orphan(CommentOrphanReason::TextNotFound)
        );
    }

    #[test]
    fn context_disambiguates_a_repeated_phrase() {
        let text = "he said softly. she said softly. they said softly.";
        // Anchor the middle "said", with "she " as its distinguishing prefix.
        let start = text.find("she said").unwrap() + 4;
        let a = capture(text, start, start + 4, 0);
        // Shift everything right; the offset hint is now wrong for every candidate.
        let edited = format!("Well, {text}");
        match resolve(&edited, &a, false, &[0]) {
            Resolution::Anchored { start: at, .. } => {
                let got: String = edited.chars().skip(at.saturating_sub(4)).take(8).collect();
                assert_eq!(got, "she said", "context must pick the right occurrence");
            }
            other => panic!("expected a unique match, got {other:?}"),
        }
    }

    #[test]
    fn a_genuinely_ambiguous_quote_is_reported_not_guessed() {
        // Two identical occurrences with identical context, equidistant from the
        // stored hint. Guessing here is what produces a comment silently attached
        // to the wrong sentence.
        let text = "aXa aXa";
        let a = Anchor {
            start: 3,
            length: 1,
            prefix: "a".into(),
            exact: "X".into(),
            exact_truncated: false,
            suffix: "a".into(),
            block_ordinal: 0,
            block_span: 1,
        };
        assert_eq!(
            resolve(text, &a, false, &[0]),
            Resolution::Orphan(CommentOrphanReason::Ambiguous)
        );
    }

    // ── paragraph comments ──────────────────────────────────────────────────

    #[test]
    fn a_paragraph_comment_re_derives_its_extent_so_a_growing_paragraph_stays_covered() {
        let starts = [0usize, 10, 30];
        let a = Anchor {
            start: 10,
            length: 5,
            prefix: String::new(),
            exact: "second".into(),
            exact_truncated: false,
            suffix: String::new(),
            block_ordinal: 1,
            block_span: 1,
        };
        // The text is longer than when the anchor was captured; the extent must be
        // the block's CURRENT bounds, not the stored length.
        let text: String = std::iter::repeat_n('z', 40).collect();
        assert_eq!(
            resolve(&text, &a, true, &starts),
            Resolution::Anchored {
                start: 10,
                // 10..29 — the block's own characters. 29 is the paragraph break
                // that begins block 2, and covering it would put the mark on the
                // next line.
                length: 19
            }
        );
    }

    #[test]
    fn a_paragraph_comment_survives_a_total_rewrite_of_its_paragraph() {
        // Its quote is gone, so a range comment would orphan — but "this paragraph"
        // still means something, so it falls back to the ordinal.
        let starts = [0usize, 10];
        let a = Anchor {
            start: 10,
            length: 4,
            prefix: String::new(),
            exact: "gone".into(),
            exact_truncated: false,
            suffix: String::new(),
            block_ordinal: 1,
            block_span: 1,
        };
        let text: String = std::iter::repeat_n('q', 25).collect();
        assert!(resolve(&text, &a, true, &starts).is_anchored());
    }

    #[test]
    fn block_lookup_maps_offsets_to_their_containing_block() {
        let starts = [0usize, 10, 30];
        assert_eq!(block_of(&starts, 0), 0);
        assert_eq!(block_of(&starts, 9), 0);
        assert_eq!(block_of(&starts, 10), 1);
        assert_eq!(block_of(&starts, 29), 1);
        assert_eq!(block_of(&starts, 30), 2);
        assert_eq!(block_of(&starts, 999), 2);
    }

    #[test]
    fn block_extent_of_the_last_block_runs_to_the_end_of_the_document() {
        let starts = [0usize, 10, 30];
        assert_eq!(block_extent(&starts, 42, 2, 2), (30, 42));
    }

    #[test]
    fn block_extent_stops_short_of_the_paragraph_break() {
        // Block 1 begins at 10, so 9 is block 0's terminating break. Ending at 10
        // would draw block 0's mark down onto block 1's first line — the "bracket
        // takes one line more" bug.
        let starts = [0usize, 10, 30];
        assert_eq!(block_extent(&starts, 42, 0, 0), (0, 9));
    }

    #[test]
    fn a_span_of_contiguous_blocks_covers_all_of_them_as_one_extent() {
        let starts = [0usize, 10, 30];
        assert_eq!(block_extent(&starts, 42, 0, 1), (0, 29));
        assert_eq!(block_extent(&starts, 42, 0, 2), (0, 42));
    }

    #[test]
    fn a_span_running_past_the_last_block_clamps_instead_of_panicking() {
        // Blocks vanish under the writer: a three-paragraph comment survives its
        // paragraphs being merged into one.
        let starts = [0usize, 10];
        assert_eq!(block_extent(&starts, 20, 0, 9), (0, 20));
        assert_eq!(block_extent(&starts, 20, 5, 9), (10, 20));
    }
}
