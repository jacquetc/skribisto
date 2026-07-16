// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The save indicator's state — what the status-bar save button shows.
//!
//! Saving is otherwise almost invisible: the only feedback the app has ever given
//! is the Save affordance greying out, and under **autosave that affordance isn't
//! there at all** (the menu item and Ctrl+S are hidden). So a writer had no way to
//! tell whether the last paragraph was on disk. This is that answer, sitting in
//! the status bar: a quiet glyph, not a toast — a save happens every few seconds
//! and a toast that often is noise. Failures *are* toasts (a save that didn't
//! happen is not a quiet fact), so the indicator has no failure state.
//!
//! Pure, like its siblings `can_save` and `unsaved_decision`: the whole decision
//! table is a function of four booleans, unit-tested below.

use std::time::{Duration, Instant};

/// What the status-bar save button shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SaveStatus {
    /// No project open, or a read-only backup file is open (saving is inert there;
    /// the banner already explains it and offers Save As / Restore) — show nothing.
    Hidden,
    /// Edits not yet on disk.
    Unsaved,
    /// A write is in flight — see [`SpinnerGate`] for why this isn't shown for
    /// every save.
    Saving,
    /// Everything is on disk.
    Saved,
}

/// The indicator's state.
///
/// `unsaved` is `App`'s derived flag (`dirty_seq > saved_seq`, so it stays true
/// while a save that predates the latest edits is in flight — see `save_queue`).
/// `saving` is [`SpinnerGate::visible`], not the raw "an op is running".
pub fn save_status(has_work: bool, backup_mode: bool, saving: bool, unsaved: bool) -> SaveStatus {
    if !has_work || backup_mode {
        return SaveStatus::Hidden;
    }
    if saving {
        return SaveStatus::Saving;
    }
    if unsaved {
        return SaveStatus::Unsaved;
    }
    SaveStatus::Saved
}

/// Can the button be clicked?
///
/// **Exactly when File ▸ Save is enabled**, so the two save affordances never
/// disagree: there must be something to write (`Unsaved`), and the user must be the
/// one deciding when — under autosave the timer owns it (which is why the menu item
/// and Ctrl+S are hidden there), so the button is *disabled*, not a live-looking
/// control that silently ignores clicks.
///
/// A clean project is therefore not clickable: clicking would re-serialize the whole
/// manuscript for no change — the "pointless disk write" the Save command already
/// refuses to do (see `can_save` in `app.rs`).
pub fn save_clickable(status: SaveStatus, autosave: bool) -> bool {
    !autosave && status == SaveStatus::Unsaved
}

/// A save is shown as "in progress" only once it has been running for this long.
const SPINNER_DELAY: Duration = Duration::from_millis(200);
/// …and once shown, it stays up at least this long.
const SPINNER_MIN_SHOWN: Duration = Duration::from_millis(250);

/// Hysteresis for the transient "saving…" state.
///
/// A local save finishes in well under 100 ms, so showing the spinner the instant
/// one starts would strobe the status bar on every autosave tick — a flicker in
/// the corner of the eye of someone trying to write. The spinner therefore appears
/// only if the save is *still* running after [`SPINNER_DELAY`], and once it has
/// appeared it stays for at least [`SPINNER_MIN_SHOWN`] so it can't blink out
/// instantly either. The states a writer actually sees are "unsaved" and "saved";
/// the spinner is there for the save that is genuinely slow — a network drive, a
/// huge manuscript — where the alternative is an indicator that looks stuck.
///
/// `now` is injected rather than read from the clock, so the whole thing is
/// testable.
#[derive(Debug, Default)]
pub struct SpinnerGate {
    /// When the running save started (`None` = nothing running).
    started: Option<Instant>,
    /// When the spinner became visible (`None` = not shown).
    shown: Option<Instant>,
}

impl SpinnerGate {
    /// A save started (`saving = true`) or landed (`false`).
    pub fn set_saving(&mut self, saving: bool, now: Instant) {
        match (saving, self.started) {
            (true, None) => self.started = Some(now),
            (true, Some(_)) => {} // already running; keep the original start
            (false, _) => self.started = None,
        }
    }

    /// Recompute against the clock. Returns when the state next changes on its own
    /// — the caller arms a wake-up for it (no polling at 60 fps). `None` when the
    /// state is stable.
    pub fn poll(&mut self, now: Instant) -> Option<Instant> {
        match (self.started, self.shown) {
            // Running, not yet shown: reveal once it has been slow enough.
            (Some(started), None) => {
                let at = started + SPINNER_DELAY;
                if now >= at {
                    self.shown = Some(now);
                    // Now it must stay up for the minimum.
                    Some(now + SPINNER_MIN_SHOWN)
                } else {
                    Some(at)
                }
            }
            // Finished, but the spinner is up: hold it for the minimum.
            (None, Some(shown)) => {
                let at = shown + SPINNER_MIN_SHOWN;
                if now >= at {
                    self.shown = None;
                    None
                } else {
                    Some(at)
                }
            }
            // Running and shown: stable until it finishes. Idle and hidden: stable.
            (Some(_), Some(_)) | (None, None) => None,
        }
    }

    /// Is the spinner on screen?
    pub fn visible(&self) -> bool {
        self.shown.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t0() -> Instant {
        Instant::now()
    }

    // ── the status table ────────────────────────────────────────────────────

    #[test]
    fn no_project_and_backup_mode_show_nothing() {
        // A backup file is read-only: `save_to_disk` is inert there, so an
        // indicator offering to save it would be a lie. The banner owns that story.
        assert_eq!(save_status(false, false, false, true), SaveStatus::Hidden);
        assert_eq!(save_status(true, true, false, true), SaveStatus::Hidden);
        assert_eq!(save_status(true, true, true, false), SaveStatus::Hidden);
    }

    #[test]
    fn a_running_save_outranks_the_dirty_flag() {
        // `unsaved` stays true while a save is in flight (it only clears when the
        // write that covers those edits lands), so "saving" must win or the
        // indicator would never show progress at all.
        assert_eq!(save_status(true, false, true, true), SaveStatus::Saving);
    }

    #[test]
    fn dirty_and_clean() {
        assert_eq!(save_status(true, false, false, true), SaveStatus::Unsaved);
        assert_eq!(save_status(true, false, false, false), SaveStatus::Saved);
    }

    #[test]
    fn clickable_exactly_when_file_save_is() {
        // The button and File ▸ Save must never disagree about whether the project
        // can be saved (`can_save` = unsaved && !backup_mode; autosave hides Save).
        assert!(save_clickable(SaveStatus::Unsaved, false));
        assert!(
            !save_clickable(SaveStatus::Saved, false),
            "a clean project has nothing to write — the Save command is greyed too"
        );
        assert!(
            !save_clickable(SaveStatus::Unsaved, true),
            "autosave owns it"
        );
        assert!(!save_clickable(SaveStatus::Saved, true), "autosave owns it");
        // Nothing to click while the write is in flight, or when there's no button.
        assert!(!save_clickable(SaveStatus::Saving, false));
        assert!(!save_clickable(SaveStatus::Hidden, false));
    }

    // ── the spinner's hysteresis ────────────────────────────────────────────

    #[test]
    fn a_fast_save_never_shows_the_spinner() {
        // The common case: a local save lands in ~50 ms. Flashing a spinner on
        // every autosave tick would strobe the status bar.
        let t = t0();
        let mut gate = SpinnerGate::default();
        gate.set_saving(true, t);
        let wake = gate.poll(t);
        assert_eq!(wake, Some(t + SPINNER_DELAY), "armed for the reveal");
        assert!(!gate.visible());

        gate.set_saving(false, t + Duration::from_millis(50));
        assert_eq!(gate.poll(t + Duration::from_millis(50)), None);
        assert!(!gate.visible(), "it was over before the spinner was due");
    }

    #[test]
    fn a_slow_save_shows_the_spinner() {
        let t = t0();
        let mut gate = SpinnerGate::default();
        gate.set_saving(true, t);
        gate.poll(t);

        let late = t + SPINNER_DELAY + Duration::from_millis(1);
        let wake = gate.poll(late);
        assert!(
            gate.visible(),
            "still running after the delay — show progress"
        );
        assert_eq!(
            wake,
            Some(late + SPINNER_MIN_SHOWN),
            "and hold it for the minimum"
        );
    }

    #[test]
    fn a_shown_spinner_is_held_for_the_minimum_then_drops() {
        let t = t0();
        let mut gate = SpinnerGate::default();
        gate.set_saving(true, t);
        let shown_at = t + SPINNER_DELAY;
        gate.poll(shown_at);
        assert!(gate.visible());

        // The save lands right after the spinner appeared: it must not blink out.
        gate.set_saving(false, shown_at + Duration::from_millis(10));
        let wake = gate.poll(shown_at + Duration::from_millis(10));
        assert!(gate.visible(), "held — a 10 ms flash would be a flicker");
        assert_eq!(wake, Some(shown_at + SPINNER_MIN_SHOWN));

        assert_eq!(gate.poll(shown_at + SPINNER_MIN_SHOWN), None);
        assert!(!gate.visible(), "the minimum has elapsed");
    }

    #[test]
    fn a_follow_up_save_keeps_the_spinner_up_rather_than_restarting_it() {
        // `SaveQueue` coalesces: a save can be followed immediately by another one
        // covering the edits typed during it. The spinner must ride through that as
        // one continuous "saving", not blink between the two ops.
        let t = t0();
        let mut gate = SpinnerGate::default();
        gate.set_saving(true, t);
        gate.poll(t + SPINNER_DELAY);
        assert!(gate.visible());

        // Still saving (the follow-up): `set_saving(true)` must not reset the clock
        // or hide anything.
        gate.set_saving(true, t + Duration::from_millis(300));
        assert!(gate.visible());
        assert_eq!(gate.poll(t + Duration::from_millis(300)), None, "stable");
    }
}
