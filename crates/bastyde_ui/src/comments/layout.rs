// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where each comment card sits in the margin, and how its leader reaches back to
//! the text.
//!
//! Pure geometry over plain numbers — no widgets, no document, no store — so the
//! rules that decide whether two cards overlap are table-testable on their own.
//! This is the part of the margin that is easy to get subtly wrong and impossible
//! to eyeball: two anchors three lines apart produce cards that *look* fine until
//! a third lands between them.
//!
//! ## The stacking rule
//!
//! Every card wants to sit at its anchor's own y, so the eye can travel straight
//! out from the text to the note. When two would overlap, the later one is pushed
//! down just enough to clear the one above by `GAP`. That is the greedy,
//! single-pass, top-to-bottom layout the published description of this pattern
//! uses (US 10,657,319, the margin-annotation stacking patent behind the
//! Word/Docs-style sidebar) — and it is what LibreOffice does too.
//!
//! Pushing down is strictly better than the obvious alternative of shrinking
//! cards to fit: a comment is text the writer has to read, and a card that
//! silently clipped its own body to avoid a neighbour would hide the thing it
//! exists to show.

/// Vertical breathing room between two stacked cards.
pub const GAP: f32 = 6.0;

/// How far a card may be pushed from its anchor before the connection is better
/// served by drawing the leader as a long diagonal than by pretending the card is
/// still "next to" its text.
///
/// Not a limit on pushing — a card is never dropped — only the point past which
/// [`Placement::detached`] is set, so the margin can draw the leader more visibly.
pub const DETACH_DISTANCE: f32 = 120.0;

/// One card's requested position: where its anchor is, and how tall the card is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardRequest {
    /// Identity, carried through so the caller can match results back to threads.
    pub comment_id: u64,
    /// The y the card would sit at if nothing else competed — the top of the
    /// anchored text.
    pub anchor_y: f32,
    pub height: f32,
}

/// Where a card ended up.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placement {
    pub comment_id: u64,
    /// The y the card is actually drawn at.
    pub y: f32,
    pub height: f32,
    /// The y the *leader* should meet, which stays the anchor's own y even when
    /// the card was pushed. The line bends rather than the anchor moving: the
    /// anchor is a fact about the text and must keep pointing at it.
    pub anchor_y: f32,
    /// Whether this card was pushed far enough from its anchor that the leader is
    /// a long run rather than a short hop.
    pub detached: bool,
}

/// Lay out `requests` in the margin, top to bottom, never overlapping.
///
/// Input order does not matter — requests are sorted by anchor position first, so
/// the visual order always matches the reading order of the text, whatever order
/// the model happened to hand them over in.
pub fn stack(requests: &[CardRequest]) -> Vec<Placement> {
    let mut sorted: Vec<CardRequest> = requests.to_vec();
    // Anchor first, then id: two comments anchored at the very same y (two threads
    // on one line) must still come out in a stable order, or the margin would
    // reshuffle itself on every rebuild.
    sorted.sort_by(|a, b| {
        a.anchor_y
            .partial_cmp(&b.anchor_y)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.comment_id.cmp(&b.comment_id))
    });

    let mut out: Vec<Placement> = Vec::with_capacity(sorted.len());
    let mut next_free = f32::NEG_INFINITY;
    for req in sorted {
        let y = req.anchor_y.max(next_free);
        next_free = y + req.height + GAP;
        out.push(Placement {
            comment_id: req.comment_id,
            y,
            height: req.height,
            anchor_y: req.anchor_y,
            detached: (y - req.anchor_y) > DETACH_DISTANCE,
        });
    }
    out
}

/// The polyline a leader follows, from the text out to its card.
///
/// Three points, not two: the run leaves the anchor horizontally through the gap
/// between lines (which is what makes it read as a rule rather than as a stray
/// diagonal across the prose), then turns once at `elbow_x` to meet the card. When
/// the card sits at its anchor's own y the turn is a no-op and the whole thing is
/// one straight horizontal line — the common case, and the one in the screenshot.
///
/// `elbow_x` is the caller's to choose, and the choice matters: put it hard against
/// the card and a pushed card's second segment becomes a long vertical run *along
/// the card's own border*, which reads as part of the card's frame rather than as
/// a line pointing at it. The margin puts it in the middle of its gutter, so that
/// segment falls in clear space and the connection stays legible however far the
/// card was pushed.
pub fn leader(
    anchor_x: f32,
    anchor_y: f32,
    elbow_x: f32,
    card_x: f32,
    card_y: f32,
) -> [(f32, f32); 3] {
    [
        (anchor_x, anchor_y),
        // Clamped forward of the anchor only: a narrow margin could otherwise put
        // the elbow left of where the run started and double the line back over the
        // prose it just left.
        (elbow_x.max(anchor_x), anchor_y),
        (card_x, card_y),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req(id: u64, y: f32, h: f32) -> CardRequest {
        CardRequest {
            comment_id: id,
            anchor_y: y,
            height: h,
        }
    }

    #[test]
    fn cards_that_do_not_compete_sit_exactly_at_their_anchors() {
        // The whole point of the margin: the eye travels straight out from the
        // text. Nothing should move unless it has to.
        let got = stack(&[req(1, 10.0, 40.0), req(2, 200.0, 40.0)]);
        assert_eq!(got[0].y, 10.0);
        assert_eq!(got[1].y, 200.0);
        assert!(got.iter().all(|p| !p.detached));
    }

    #[test]
    fn a_colliding_card_is_pushed_down_just_enough_to_clear() {
        let got = stack(&[req(1, 10.0, 40.0), req(2, 20.0, 40.0)]);
        assert_eq!(got[0].y, 10.0);
        assert_eq!(
            got[1].y,
            10.0 + 40.0 + GAP,
            "pushed to clear the one above by exactly GAP, not further"
        );
    }

    #[test]
    fn a_pushed_card_keeps_its_leader_pointing_at_the_real_anchor() {
        // The card moves; the anchor does not. A leader that followed the card
        // would point at the wrong line, which is the whole failure this avoids.
        let got = stack(&[req(1, 10.0, 40.0), req(2, 20.0, 40.0)]);
        assert_eq!(got[1].anchor_y, 20.0);
        assert!(got[1].y > got[1].anchor_y);
    }

    #[test]
    fn a_run_of_tightly_packed_anchors_never_produces_an_overlap() {
        // Ten threads on nearly the same line — the case that motivates the whole
        // pass, and the one a naive "draw at anchor_y" would render as a pile.
        let reqs: Vec<CardRequest> = (0..10).map(|i| req(i, 100.0 + i as f32, 30.0)).collect();
        let got = stack(&reqs);
        for w in got.windows(2) {
            assert!(
                w[1].y >= w[0].y + w[0].height,
                "cards overlap: {:?} then {:?}",
                w[0],
                w[1]
            );
        }
    }

    #[test]
    fn placement_order_follows_the_text_not_the_input_order() {
        let got = stack(&[req(1, 500.0, 20.0), req(2, 10.0, 20.0)]);
        assert_eq!(
            got.iter().map(|p| p.comment_id).collect::<Vec<_>>(),
            vec![2, 1],
            "the margin reads top-to-bottom like the prose it annotates"
        );
    }

    #[test]
    fn two_threads_on_one_line_keep_a_stable_order() {
        // Equal anchors must not reshuffle between rebuilds, or the margin would
        // flicker as the writer types elsewhere.
        let a = stack(&[req(7, 50.0, 20.0), req(3, 50.0, 20.0)]);
        let b = stack(&[req(3, 50.0, 20.0), req(7, 50.0, 20.0)]);
        assert_eq!(
            a.iter().map(|p| p.comment_id).collect::<Vec<_>>(),
            b.iter().map(|p| p.comment_id).collect::<Vec<_>>()
        );
    }

    #[test]
    fn a_card_shoved_far_from_its_anchor_reports_itself_detached() {
        let reqs: Vec<CardRequest> = (0..8).map(|i| req(i, 10.0, 40.0)).collect();
        let got = stack(&reqs);
        assert!(!got[0].detached, "the first still sits at its anchor");
        assert!(
            got.last().unwrap().detached,
            "the last is far below its anchor and says so, so the leader can be \
             drawn to match"
        );
    }

    #[test]
    fn an_empty_margin_places_nothing() {
        assert!(stack(&[]).is_empty());
    }

    #[test]
    fn a_leader_to_an_unpushed_card_is_flat() {
        // The common case, and the one in the reference screenshot: a single
        // horizontal rule through the line gap.
        let [a, b, c] = leader(100.0, 50.0, 286.0, 300.0, 50.0);
        assert_eq!(a.1, 50.0);
        assert_eq!(b.1, 50.0);
        assert_eq!(c.1, 50.0);
        assert!(b.0 > a.0 && c.0 > b.0, "and it runs left to right");
    }

    #[test]
    fn a_leader_to_a_pushed_card_turns_once_at_the_elbow() {
        let [a, b, c] = leader(100.0, 50.0, 286.0, 300.0, 120.0);
        assert_eq!(a.1, b.1, "it leaves the text horizontally");
        assert_eq!(b.0, 286.0, "it turns where the caller asked");
        assert_eq!(c, (300.0, 120.0), "and finishes at the card");
        assert!(
            b.0 > a.0,
            "the turn happens near the card, not near the text — a long diagonal \
             across the prose is exactly what the elbow avoids"
        );
    }

    #[test]
    fn a_pushed_cards_finishing_segment_clears_the_card_border() {
        // The failure this guards: an elbow flush with the card turns the second
        // segment into a long vertical dotted run *on* the card's left border,
        // where it reads as frame rather than as a pointer.
        let [_, b, c] = leader(100.0, 50.0, 286.0, 300.0, 300.0);
        assert!(
            c.0 - b.0 > 8.0,
            "the finishing segment is nearly vertical and hugs the card ({b:?} → {c:?})"
        );
    }

    #[test]
    fn a_leader_never_runs_backwards_when_the_card_is_close() {
        // A narrow margin could put the elbow left of the anchor; clamping keeps
        // the run monotonic rather than doubling back over the text.
        let [a, b, _] = leader(320.0, 10.0, 286.0, 300.0, 10.0);
        assert!(b.0 >= a.0);
    }
}

// ---------------------------------------------------------------------------
// Where the writing column and its margin sit in the pane
// ---------------------------------------------------------------------------

/// Horizontal placement of the writing column and the comment margin.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PanePlacement {
    pub column_x: f32,
    pub column_width: f32,
    /// `None` when there is no margin to place.
    pub margin_x: Option<f32>,
}

/// Place the writing column and its margin across a pane `available` wide.
///
/// Three rules, in priority order:
///
/// 1. **The margin is flush to the column.** It sits immediately at the column's
///    right edge, not pinned to the far side of the pane — a card marooned against
///    the window edge with a long empty run of leader between it and its text is
///    exactly what "the margin is too far" means.
/// 2. **The column stays centred in the *whole* pane** for as long as the margin
///    still fits beside it. The page does not lurch left the moment a comment
///    appears; it only moves when it has to.
/// 3. **When it must move, the column shifts left rather than shrinking**, and
///    only shrinks once it is already hard against the left edge.
///
/// `margin` is `None` when the document has no comments, in which case this is
/// plain centring.
pub fn place_pane(available: f32, desired_column: f32, margin: Option<f32>) -> PanePlacement {
    let Some(margin_w) = margin else {
        let w = desired_column.min(available).max(0.0);
        return PanePlacement {
            column_x: ((available - w) / 2.0).max(0.0),
            column_width: w,
            margin_x: None,
        };
    };

    // Rule 3: the column gives up width only when the pair cannot fit at all.
    let w = desired_column.min((available - margin_w).max(0.0));
    // Rule 2: centred in the whole pane, as if the margin were not there.
    let centred = ((available - w) / 2.0).max(0.0);
    // Rule 1 + 3: pull left just enough to keep the flush margin on-screen.
    let max_x = (available - margin_w - w).max(0.0);
    let x = centred.min(max_x);

    PanePlacement {
        column_x: x,
        column_width: w,
        margin_x: Some(x + w),
    }
}

#[cfg(test)]
mod pane_tests {
    use super::*;

    #[test]
    fn with_no_margin_the_column_is_simply_centred() {
        let p = place_pane(1000.0, 600.0, None);
        assert_eq!(p.column_x, 200.0);
        assert_eq!(p.column_width, 600.0);
        assert!(p.margin_x.is_none());
    }

    #[test]
    fn the_margin_sits_flush_against_the_column() {
        // The whole point: no gap between the text and its cards.
        let p = place_pane(1400.0, 600.0, Some(300.0));
        assert_eq!(
            p.margin_x,
            Some(p.column_x + p.column_width),
            "a gap here is the 'margin too far' bug"
        );
    }

    #[test]
    fn the_column_stays_centred_while_the_margin_still_fits() {
        // 1400 wide, 600 column → centred at 400, margin at 1000..1300. Fits, so
        // the page must not move at all.
        let p = place_pane(1400.0, 600.0, Some(300.0));
        assert_eq!(p.column_x, 400.0, "the page should not lurch when a comment appears");
        assert_eq!(p.column_width, 600.0);
    }

    #[test]
    fn the_column_shifts_left_only_as_far_as_it_must() {
        // 1000 wide: centred would be 200, putting the margin at 800..1100 —
        // 100px off-screen. It should shift exactly 100px, not to the far left.
        let p = place_pane(1000.0, 600.0, Some(300.0));
        assert_eq!(p.column_x, 100.0);
        assert_eq!(p.column_width, 600.0, "and it does not shrink while shifting can do the job");
        assert_eq!(p.margin_x, Some(700.0));
    }

    #[test]
    fn the_column_shrinks_only_once_it_is_against_the_left_edge() {
        // 700 wide with a 300 margin leaves 400 for a 600 column.
        let p = place_pane(700.0, 600.0, Some(300.0));
        assert_eq!(p.column_x, 0.0);
        assert_eq!(p.column_width, 400.0);
        assert_eq!(p.margin_x, Some(400.0));
    }

    #[test]
    fn a_pane_narrower_than_the_margin_degrades_without_panicking() {
        let p = place_pane(200.0, 600.0, Some(300.0));
        assert!(p.column_width >= 0.0);
        assert!(p.column_x >= 0.0);
        assert!(p.margin_x.unwrap() >= 0.0);
    }
}
