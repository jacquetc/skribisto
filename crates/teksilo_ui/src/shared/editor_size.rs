// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Changing a writing editor's text size from the page, rather than from Settings.
//!
//! Not a view-model — four free functions and an accumulator, shared by the two
//! surfaces that drive the size: the Ctrl+Wheel handler in
//! [`TypographyBoundEditor`](crate::tabs::shared::editor) and the
//! `editor.size.*` commands behind Ctrl+= / Ctrl+− / Ctrl+0. Both must clamp,
//! snap and announce identically, so both call these rather than each rolling
//! their own arithmetic.
//!
//! There is no separate "zoom" state anywhere: the size a writer sets here *is*
//! the persisted `editor.<kind>.size` preference, the same `Signal` the Settings
//! slider drives. That is what makes the gesture survive a restart, reach every
//! open editor of that kind at once, and leave the slider showing the truth.

use std::time::Duration;

use teksilo::core::event::ScrollDelta;
use teksilo::prelude::*;
use teksilo::widgets::Toast;

use crate::settings::{EditorTypography, TypographyKind};

/// One wheel notch, in `ScrollDelta::Lines` units.
///
/// Not `1.0`: teksilo's platform layer multiplies a winit notch by
/// `LINES_PER_NOTCH = 3.0` (matching the Windows/GTK default) before the event
/// ever reaches a widget, so a single physical detent arrives as `y: ±3.0`.
const LINES_PER_NOTCH: f32 = 3.0;

/// One wheel notch, in `ScrollDelta::Pixels` units.
///
/// Wayland — this app's primary target — delivers wheel detents as *pixel*
/// deltas rather than lines, and a trackpad delivers a continuous stream of
/// them. `RichTextEditor` reads a line as 16 px, so a three-line notch is 48.
/// Deriving it from the framework's own two constants rather than picking a
/// feel-good divisor is what keeps a notch here the same size as a notch
/// everywhere else in the app.
const PIXELS_PER_NOTCH: f32 = LINES_PER_NOTCH * 16.0;

/// Dedup key for the size readout, shared by the wheel and the keyboard so a
/// burst from either updates one live toast instead of stacking a queue of
/// them. Re-showing a toast under a live id mutates that entry in place and
/// restarts its dismiss timer.
const SIZE_TOAST_ID: &str = "editor.size";

/// How long the readout stays. Far shorter than the framework default: this
/// fires on every notch and every keypress, not once per operation, so a
/// long-lived toast would still be sitting there describing a size the writer
/// left behind several seconds ago.
const SIZE_TOAST_DISMISS: Duration = Duration::from_millis(1400);

/// Turn a raw wheel/trackpad stream into whole notches.
///
/// Stateful because a high-resolution wheel and a trackpad both deliver
/// fractions of a notch per event: without carrying the remainder, a Wayland
/// pixel dribble either steps on every single event (wildly over-sensitive) or
/// never steps at all. One lives in each mounted editor's wheel closure.
#[derive(Default)]
pub struct WheelAccumulator {
    pending: f32,
}

impl WheelAccumulator {
    /// Feed one `Scroll` delta; returns the whole notches it completed, if any.
    ///
    /// **Positive means bigger.** The sign is flipped here, once, because
    /// teksilo's `ScrollDelta` is a *scroll offset* delta rather than a raw
    /// wheel reading — the platform layer negates winit so that positive `y`
    /// grows a scroll offset, i.e. positive `y` is the wheel turning **down**.
    /// Every consumer that maps a notch to a value rather than an offset has to
    /// undo that (teksilo's own `SpinBox` had this backwards until it was fixed);
    /// doing it in one place means the two call sites cannot disagree.
    pub fn feed(&mut self, delta: ScrollDelta) -> Option<i32> {
        let notches = match delta {
            ScrollDelta::Lines { y, .. } => y / LINES_PER_NOTCH,
            ScrollDelta::Pixels { y, .. } => y / PIXELS_PER_NOTCH,
        };
        self.pending -= notches;
        let whole = self.pending.trunc();
        if whole == 0.0 {
            return None;
        }
        self.pending -= whole;
        Some(whole as i32)
    }
}

/// Step `typo`'s size by `notches` grid steps (positive grows), clamped and
/// snapped to its own range. Returns the value actually stored.
///
/// Snapping is not cosmetic: `0.7 + 0.05 × 9` is `1.1500001` in `f32`, and a
/// slider that can only produce exact grid values would then never show the
/// size the wheel just set. Every write goes through
/// [`TypographySizeRange::snap`](crate::settings::TypographySizeRange::snap) so
/// the two surfaces always describe the same number.
pub fn step(typo: &EditorTypography, notches: i32) -> f32 {
    let range = typo.size_range;
    let stepped = typo.size.get() + notches as f32 * range.step;
    let snapped = range.snap(stepped);
    typo.size.set(snapped);
    snapped
}

/// Put `typo`'s size back to its bundle's compile-time default.
pub fn reset(typo: &EditorTypography) -> f32 {
    let value = typo.size_range.snap(typo.size_range.default);
    typo.size.set(value);
    value
}

/// The readout for one bundle, naming the surface so a writer who was pointing
/// at a synopsis is never left guessing which of the six sizes just moved.
///
/// Six whole messages rather than one `{ $surface }: { $percent }` template fed
/// a nested label. A Fluent argument is a string or a number, not another
/// message — and even if it could nest, "Manuscript" and "Corkboard (expanded)"
/// do not necessarily take the same connector, article or word order in every
/// locale. A translator gets six complete sentences to work with.
fn readout(kind: TypographyKind, percent: String) -> LocalizedString {
    match kind {
        TypographyKind::Scene => tr!(editor_size_changed_manuscript(percent = percent)),
        TypographyKind::Synopsis => tr!(editor_size_changed_synopsis(percent = percent)),
        TypographyKind::Notes => tr!(editor_size_changed_notes(percent = percent)),
        TypographyKind::Corkboard => tr!(editor_size_changed_corkboard(percent = percent)),
        TypographyKind::CorkboardExpanded => {
            tr!(editor_size_changed_corkboard_expanded(percent = percent))
        }
        TypographyKind::DistractionFree => {
            tr!(editor_size_changed_distraction_free(percent = percent))
        }
    }
}

/// Show the `"<surface>: <percent>"` readout for `typo`'s current size.
///
/// The percentage is formatted exactly as the Settings slider's own readout
/// does, so the two never disagree by a rounding step.
///
/// The dedup id folds in the presenting window, not the open `Work`: the size is
/// an app-global preference, so [`crate::toast_scope::ToastWorkExt::scoped_id`]'s
/// per-Work scoping would be the wrong axis — two windows on the *same* Work
/// would still fight over one entry, while the thing that must not collide is
/// two windows, full stop.
pub fn announce(ctx: &mut EventContext, typo: &EditorTypography) {
    let percent = format!("{:.0}%", typo.size.get() * 100.0);
    let window = ctx.window().map(|w| w.id());
    let mut toast = Toast::info(readout(typo.size_range.kind, percent))
        .auto_dismiss_after(SIZE_TOAST_DISMISS)
        // Without this every notch of a spin lands in the notification archive
        // behind the status bar's bell, burying whatever the writer actually
        // wanted to keep under a hundred size readouts.
        .archive(false);
    if let Some(window) = window {
        toast = toast.id(format!("{SIZE_TOAST_ID}.{window:?}"));
    }
    ctx.show_toast(toast);
}

/// Step and announce in one call — what both the wheel and the keyboard do.
pub fn step_and_announce(ctx: &mut EventContext, typo: &EditorTypography, notches: i32) {
    step(typo, notches);
    announce(ctx, typo);
}

/// Reset and announce in one call.
pub fn reset_and_announce(ctx: &mut EventContext, typo: &EditorTypography) {
    reset(typo);
    announce(ctx, typo);
}

#[cfg(test)]
mod tests;
