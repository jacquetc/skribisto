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
mod tests;
