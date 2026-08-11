// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Pure progress arithmetic and the two colour vocabularies.
//!
//! Every surface that draws a goal reads its ratio and its colour from here, so a scene's
//! bar in the status bar, its cell in the Overview and its readout in the Inspector cannot
//! disagree about what "nearly there" looks like.
//!
//! **Two role functions, not one.** A writing *session* is a sprint: there is no such thing
//! as writing too much in one, so [`sprint_role`] tops out at "good" and never flags an
//! overshoot. A *document target* is a budget: a chapter that is half again as long as it
//! was planned to be is worth noticing, so [`target_role`] adds a fourth band above it.
//! Manuskript is the one surveyed tool that thought about this and it draws the same
//! conclusion (its bands stop at 120% and then change colour); bibisco's three bands cannot
//! tell "just made it" from "way over", and Scrivener's outliner silently drops the overrun
//! state its editor footer has, which is a filed complaint against it.

use teksilo::tokens::TextRole;

/// Ratio at and above which a document target reads as overshot rather than met.
///
/// 1.2 follows Manuskript, the only prior art that picked a threshold deliberately. Below
/// it, being over target is just "done".
pub const OVERSHOT: f32 = 1.2;

/// Progress toward a target as a raw, **unclamped** ratio, or `None` when there is no
/// target (`0` is the no-goal sentinel throughout this app, and a negative target is
/// nonsense rather than a goal).
///
/// Unclamped on purpose: [`target_role`] cannot see an overshoot through a clamped value.
/// Callers that feed a `ProgressBar` want [`bar_fill`] instead.
pub fn ratio(written: i64, target: i64) -> Option<f32> {
    (target > 0).then(|| written.max(0) as f32 / target as f32)
}

/// The `0.0..=1.0` fill a `ProgressBar` takes, from a raw [`ratio`].
pub fn bar_fill(ratio: f32) -> f32 {
    ratio.clamp(0.0, 1.0)
}

/// The three-band red → amber → green gauge of a writing **session**.
///
/// Semantic theme roles only (no colour interpolation primitive exists), so it works in
/// light and dark.
pub fn sprint_role(fill: f32) -> TextRole {
    if fill < 0.34 {
        TextRole::Error
    } else if fill < 0.75 {
        TextRole::Warning
    } else {
        TextRole::Success
    }
}

/// The four-band gauge of a **document target**: the session's three, plus an accent band
/// once the writing has run well past what was planned.
///
/// `Accent` rather than `Warning` for the fourth: overshooting a target is noteworthy, not
/// an error and not something done wrong, and re-using amber after green would read as a
/// regression.
pub fn target_role(ratio: f32) -> TextRole {
    if ratio >= OVERSHOT {
        TextRole::Accent
    } else {
        sprint_role(bar_fill(ratio))
    }
}

/// Progress toward a target in `0.0..=1.0`, or `None` when there is no target.
///
/// The session gauge's shape, kept because a sprint only ever wants the clamped value.
pub fn words_progress(written: i64, target: Option<i64>) -> Option<f32> {
    ratio(written, target.unwrap_or(0)).map(bar_fill)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ratio_is_unclamped_and_treats_zero_as_no_goal() {
        assert_eq!(ratio(250, 0), None, "0 target = no goal");
        assert_eq!(ratio(250, -5), None, "a negative target is not a goal");
        assert_eq!(ratio(250, 500), Some(0.5));
        assert_eq!(ratio(750, 500), Some(1.5), "overshoot survives");
        assert_eq!(ratio(-5, 500), Some(0.0), "a negative count floors at zero");
    }

    #[test]
    fn bar_fill_clamps_both_ends() {
        assert_eq!(bar_fill(-1.0), 0.0);
        assert_eq!(bar_fill(0.5), 0.5);
        assert_eq!(bar_fill(1.5), 1.0);
    }

    #[test]
    fn sprint_role_buckets_red_amber_green() {
        assert_eq!(sprint_role(0.0), TextRole::Error);
        assert_eq!(sprint_role(0.33), TextRole::Error);
        assert_eq!(sprint_role(0.34), TextRole::Warning);
        assert_eq!(sprint_role(0.74), TextRole::Warning);
        assert_eq!(sprint_role(0.75), TextRole::Success);
        assert_eq!(sprint_role(1.0), TextRole::Success);
    }

    /// The fourth band is the whole reason a document target does not reuse the sprint
    /// gauge: a sprint has no "too much", a chapter budget does.
    #[test]
    fn target_role_adds_an_overshoot_band_above_the_sprint_three() {
        assert_eq!(target_role(0.0), TextRole::Error);
        assert_eq!(target_role(0.5), TextRole::Warning);
        assert_eq!(target_role(1.0), TextRole::Success);
        assert_eq!(
            target_role(1.19),
            TextRole::Success,
            "just over is still done"
        );
        assert_eq!(target_role(OVERSHOT), TextRole::Accent);
        assert_eq!(target_role(3.0), TextRole::Accent);
    }

    #[test]
    fn words_progress_matches_the_session_gauge_it_replaced() {
        assert_eq!(words_progress(250, None), None);
        assert_eq!(words_progress(250, Some(0)), None, "0 target = no goal");
        assert_eq!(words_progress(250, Some(500)), Some(0.5));
        assert_eq!(words_progress(600, Some(500)), Some(1.0), "clamped at full");
        assert_eq!(words_progress(-5, Some(500)), Some(0.0));
    }
}
