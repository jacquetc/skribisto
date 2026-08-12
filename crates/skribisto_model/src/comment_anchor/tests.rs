// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

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
