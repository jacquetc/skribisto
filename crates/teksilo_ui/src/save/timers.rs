// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The two frame-driven countdowns the app runs while a project is open: the
//! debounced autosave, and the "back up every N hours" interval.
//!
//! The *policy* is a pure function of `(now, settings, internal deadline)`
//! returning what the caller should do — injecting `now` makes the whole thing
//! testable in microseconds, without a `WidgetTree` or real sleeps. `App` keeps
//! the two side effects it cannot own — calling `wake_at` and actually
//! saving/backing up — and this module owns every decision about *when*.
//!
//! ## Why `wake_at` at all
//!
//! Teksilo only pumps frames when something asks it to. A naive countdown polled on every
//! frame would either drain the battery at 60 fps or never run at all once the UI went
//! idle. `wake_at` schedules exactly one wake at the deadline: the loop sleeps until then,
//! the `frame_tick` effect fires on the frame that pumps, and [`Tick::Sleep`] re-arms the
//! wake whenever the deadline has not yet lapsed (a frame pumped for some unrelated reason
//! must not consume the pending wake).

use std::cell::Cell;
use std::time::{Duration, Instant};

/// How long after the last mutation an autosave fires.
pub(crate) const AUTOSAVE_DEBOUNCE: Duration = Duration::from_millis(1500);

/// The longest an unsaved change waits for an autosave, however steadily the
/// writer keeps typing.
///
/// The debounce alone waits for a pause of [`AUTOSAVE_DEBOUNCE`], and a writer in
/// a sprint (or the "always forward" game) may never pause that long: every
/// keystroke pushed the save out again, and a crash after twenty minutes of
/// typing lost twenty minutes. So the deadline is never later than this long
/// after the first change the disk does not have yet.
pub(crate) const AUTOSAVE_MAX_WAIT: Duration = Duration::from_secs(30);

/// When the autosave for a change made at `now` should fire, given that the oldest
/// change not yet saved was made at `first_unsaved`: a pause of
/// [`AUTOSAVE_DEBOUNCE`] after this change, but never later than
/// [`AUTOSAVE_MAX_WAIT`] after that oldest one.
pub(crate) fn autosave_deadline(now: Instant, first_unsaved: Instant) -> Instant {
    (now + AUTOSAVE_DEBOUNCE).min(first_unsaved + AUTOSAVE_MAX_WAIT)
}

/// What a frame tick should do about a one-shot deadline.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Tick {
    /// Nothing armed — no wake to schedule.
    Idle,
    /// Still counting down; re-arm the wake for this instant.
    Sleep(Instant),
    /// The deadline lapsed. The timer has disarmed itself; do the thing.
    Fire,
}

/// A one-shot deadline. Arm it, poll it once per frame.
#[derive(Debug, Default)]
pub(crate) struct DeadlineTimer {
    at: Cell<Option<Instant>>,
}

impl DeadlineTimer {
    pub(crate) fn new() -> Self {
        Self {
            at: Cell::new(None),
        }
    }

    /// (Re)arm for `at`, replacing any deadline already pending — this is what makes the
    /// autosave a *debounce*: each new mutation pushes the deadline further out.
    pub(crate) fn arm(&self, at: Instant) {
        self.at.set(Some(at));
    }

    pub(crate) fn disarm(&self) {
        self.at.set(None);
    }

    /// Poll at `now`. Returns [`Tick::Fire`] **once** per armed deadline — it disarms
    /// itself first, so a caller that ignores the result cannot get a second fire.
    pub(crate) fn poll(&self, now: Instant) -> Tick {
        match self.at.get() {
            None => Tick::Idle,
            Some(at) if now >= at => {
                self.at.set(None);
                Tick::Fire
            }
            Some(at) => Tick::Sleep(at),
        }
    }
}

/// The debounced autosave countdown.
///
/// Every mutation (a keystroke via the editors' `edited` signal, or a tree/metadata event)
/// pushes the deadline out; the save fires once the user has been still for
/// [`AUTOSAVE_DEBOUNCE`], or [`AUTOSAVE_MAX_WAIT`] after the first change it has to
/// write, whichever comes first (see [`autosave_deadline`]).
#[derive(Debug, Default)]
pub(crate) struct AutosaveCountdown {
    timer: DeadlineTimer,
    /// When the oldest change not yet handed to a save was made. Set by the first
    /// mutation after a save, cleared when the countdown fires and when a save from
    /// anywhere else leaves nothing unsaved ([`Self::on_saved`]).
    first_unsaved: Cell<Option<Instant>>,
}

impl AutosaveCountdown {
    pub(crate) fn new() -> Self {
        Self {
            timer: DeadlineTimer::new(),
            first_unsaved: Cell::new(None),
        }
    }

    /// A mutation happened at `now`. Returns the instant to wake at, or `None` when
    /// autosave is off (the timer stays disarmed — the user saves by hand).
    ///
    /// Note this does *not* disarm a pending deadline when `autosave` is false: turning
    /// autosave off mid-countdown leaves the deadline to lapse, and [`Self::tick`] then
    /// declines to save. That is deliberate — it matches the check-at-fire-time ordering
    /// the inline version had, so the setting is read at the moment it matters rather
    /// than at the moment it was armed.
    pub(crate) fn on_mutation(&self, now: Instant, autosave: bool) -> Option<Instant> {
        if !autosave {
            return None;
        }
        let first = self.first_unsaved.get().unwrap_or(now);
        self.first_unsaved.set(Some(first));
        let at = autosave_deadline(now, first);
        self.timer.arm(at);
        Some(at)
    }

    /// Everything is on disk: a save from somewhere other than this countdown
    /// (Ctrl+S, a close, a project switch) caught up with every change. The next
    /// change starts a fresh maximum wait, and the pending deadline is dropped,
    /// since it would only write the same project again.
    pub(crate) fn on_saved(&self) {
        self.first_unsaved.set(None);
        self.timer.disarm();
    }

    /// Poll on a frame tick. `Some(instant)` means "re-arm the wake"; `None` means there
    /// is nothing pending. `save` is set when the caller should flush to disk now.
    pub(crate) fn tick(&self, now: Instant, autosave: bool) -> (bool, Option<Instant>) {
        match self.timer.poll(now) {
            Tick::Idle => (false, None),
            Tick::Sleep(at) => (false, Some(at)),
            // Read `autosave` at fire time, not arm time: a user who turns autosave off
            // during the debounce window must not get one last surprise write.
            //
            // The save about to run covers every change so far, so the next one starts
            // a new maximum wait. A save that fails leaves the project unsaved, and the
            // next keystroke arms again, as it always has.
            Tick::Fire => {
                self.first_unsaved.set(None);
                (autosave, None)
            }
        }
    }
}

/// What the interval-backup countdown wants on this frame.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum IntervalTick {
    /// Interval backups are off, or no project is open. Nothing pending.
    Disarmed,
    /// Counting down; re-arm the wake for this instant.
    Sleep(Instant),
    /// Take a backup now, then wake again at this instant.
    Fire(Instant),
}

/// The "back up every N hours" countdown.
///
/// Keyed on the scheduler's `completed_epoch`: **any** backup — manual, on-open, on-close,
/// or one of our own ticks — restarts the countdown, so "every N hours" means N hours since
/// the last backup actually happened, not N hours since the timer last armed. Without that,
/// hitting "Back up now" and then waiting would produce a second backup minutes later.
#[derive(Debug)]
pub(crate) struct IntervalCountdown {
    timer: DeadlineTimer,
    seen_epoch: Cell<u64>,
}

impl IntervalCountdown {
    pub(crate) fn new(epoch: u64) -> Self {
        Self {
            timer: DeadlineTimer::new(),
            seen_epoch: Cell::new(epoch),
        }
    }

    /// Poll on a frame tick. `interval` is the open project's effective setting — `None`
    /// disarms (interval off, or no project). `epoch` is the scheduler's completed-backup
    /// counter.
    pub(crate) fn tick(
        &self,
        now: Instant,
        interval: Option<Duration>,
        epoch: u64,
    ) -> IntervalTick {
        let Some(interval) = interval else {
            self.timer.disarm();
            return IntervalTick::Disarmed;
        };

        // A backup landed since we last looked — restart from now rather than firing.
        if epoch != self.seen_epoch.get() {
            self.seen_epoch.set(epoch);
            let at = now + interval;
            self.timer.arm(at);
            return IntervalTick::Sleep(at);
        }

        match self.timer.poll(now) {
            Tick::Idle => {
                let at = now + interval;
                self.timer.arm(at);
                IntervalTick::Sleep(at)
            }
            Tick::Sleep(at) => IntervalTick::Sleep(at),
            Tick::Fire => {
                let next = now + interval;
                self.timer.arm(next);
                IntervalTick::Fire(next)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t0() -> Instant {
        Instant::now()
    }

    // ── DeadlineTimer ────────────────────────────────────────────────────────

    #[test]
    fn an_unarmed_timer_is_idle() {
        let t = DeadlineTimer::new();
        assert_eq!(t.poll(t0()), Tick::Idle);
    }

    #[test]
    fn an_armed_timer_sleeps_until_its_deadline_then_fires_exactly_once() {
        let now = t0();
        let t = DeadlineTimer::new();
        t.arm(now + Duration::from_secs(10));

        assert_eq!(t.poll(now), Tick::Sleep(now + Duration::from_secs(10)));
        assert_eq!(
            t.poll(now + Duration::from_secs(9)),
            Tick::Sleep(now + Duration::from_secs(10))
        );
        assert_eq!(t.poll(now + Duration::from_secs(10)), Tick::Fire);
        // Disarmed by the fire — a caller polling again must not save twice.
        assert_eq!(t.poll(now + Duration::from_secs(11)), Tick::Idle);
    }

    #[test]
    fn arming_again_replaces_the_pending_deadline() {
        let now = t0();
        let t = DeadlineTimer::new();
        t.arm(now + Duration::from_secs(1));
        t.arm(now + Duration::from_secs(5));
        // The first deadline is gone — this is what makes the autosave a debounce.
        assert_eq!(
            t.poll(now + Duration::from_secs(2)),
            Tick::Sleep(now + Duration::from_secs(5))
        );
    }

    // ── AutosaveCountdown ────────────────────────────────────────────────────

    #[test]
    fn a_mutation_arms_the_debounce_and_the_save_fires_after_it() {
        let now = t0();
        let a = AutosaveCountdown::new();

        assert_eq!(a.on_mutation(now, true), Some(now + AUTOSAVE_DEBOUNCE));
        assert_eq!(a.tick(now, true), (false, Some(now + AUTOSAVE_DEBOUNCE)));
        assert_eq!(a.tick(now + AUTOSAVE_DEBOUNCE, true), (true, None));
        // One save per quiet period.
        assert_eq!(a.tick(now + AUTOSAVE_DEBOUNCE * 2, true), (false, None));
    }

    #[test]
    fn typing_keeps_pushing_the_deadline_out() {
        let now = t0();
        let a = AutosaveCountdown::new();

        a.on_mutation(now, true);
        // A keystroke 1s in re-arms: no save at the original deadline.
        let later = now + Duration::from_secs(1);
        a.on_mutation(later, true);

        assert_eq!(
            a.tick(now + AUTOSAVE_DEBOUNCE, true),
            (false, Some(later + AUTOSAVE_DEBOUNCE))
        );
        assert_eq!(a.tick(later + AUTOSAVE_DEBOUNCE, true), (true, None));
    }

    #[test]
    fn the_deadline_is_the_debounce_while_the_first_change_is_recent() {
        let first = t0();
        let now = first + Duration::from_secs(10);
        assert_eq!(autosave_deadline(now, first), now + AUTOSAVE_DEBOUNCE);
        assert_eq!(autosave_deadline(first, first), first + AUTOSAVE_DEBOUNCE);
    }

    #[test]
    fn the_deadline_never_passes_the_maximum_wait_after_the_first_change() {
        let first = t0();
        // 29 s in, a pause of 1.5 s would land past the 30 s cap.
        let now = first + Duration::from_secs(29);
        assert_eq!(autosave_deadline(now, first), first + AUTOSAVE_MAX_WAIT);
        // Long past the cap (a change typed while the save was held up): due now.
        let late = first + Duration::from_secs(45);
        assert_eq!(autosave_deadline(late, first), first + AUTOSAVE_MAX_WAIT);
    }

    /// The bug this cap exists for: a writer who never pauses for the debounce
    /// still gets a save, at most [`AUTOSAVE_MAX_WAIT`] after the first word, and
    /// again at most that long after the save.
    #[test]
    fn typing_without_a_pause_still_saves_within_the_maximum_wait() {
        let start = t0();
        let a = AutosaveCountdown::new();
        let keystroke = Duration::from_millis(400);

        let mut saves = Vec::new();
        let mut now = start;
        while now < start + Duration::from_secs(70) {
            a.on_mutation(now, true);
            now += keystroke;
            if a.tick(now, true).0 {
                saves.push(now);
            }
        }

        assert_eq!(saves.len(), 2, "one save per maximum wait: {saves:?}");
        assert!(saves[0] <= start + AUTOSAVE_MAX_WAIT + keystroke);
        assert!(saves[0] >= start + AUTOSAVE_MAX_WAIT);
        // The second wait starts at the first keystroke after the save, not at the
        // very first word of the session.
        assert!(saves[1] <= saves[0] + AUTOSAVE_MAX_WAIT + keystroke);
        assert!(saves[1] >= saves[0] + AUTOSAVE_MAX_WAIT);
    }

    /// A Ctrl+S in the middle of a sprint saves everything: the next change waits
    /// a full maximum again, counted from itself, and the deadline armed before the
    /// Ctrl+S is gone.
    #[test]
    fn a_save_from_elsewhere_restarts_the_maximum_wait() {
        let start = t0();
        let a = AutosaveCountdown::new();

        a.on_mutation(start, true);
        let saved_at = start + Duration::from_secs(20);
        a.on_mutation(saved_at, true);
        a.on_saved();
        assert_eq!(
            a.tick(saved_at, true),
            (false, None),
            "nothing is left to save"
        );

        let next = saved_at + Duration::from_secs(1);
        a.on_mutation(next, true);
        // Without the reset the cap would still be `start + 30 s`, 9 s from here.
        let at = start + Duration::from_secs(29);
        assert_eq!(
            autosave_deadline(at, next),
            at + AUTOSAVE_DEBOUNCE,
            "the new wait counts from the first change after the save"
        );
        assert_eq!(a.on_mutation(at, true), Some(at + AUTOSAVE_DEBOUNCE));
    }

    #[test]
    fn with_autosave_off_a_mutation_arms_nothing() {
        let now = t0();
        let a = AutosaveCountdown::new();

        assert_eq!(a.on_mutation(now, false), None);
        assert_eq!(a.tick(now + AUTOSAVE_DEBOUNCE, false), (false, None));
    }

    /// Turning autosave off during the debounce window must not produce one last write:
    /// the flag is read at fire time, not at arm time.
    #[test]
    fn turning_autosave_off_mid_countdown_cancels_the_save() {
        let now = t0();
        let a = AutosaveCountdown::new();

        a.on_mutation(now, true);
        let (save, wake) = a.tick(now + AUTOSAVE_DEBOUNCE, false);
        assert!(!save, "the setting is read when the deadline lapses");
        assert_eq!(wake, None);
    }

    // ── IntervalCountdown ────────────────────────────────────────────────────

    #[test]
    fn no_interval_disarms() {
        let c = IntervalCountdown::new(0);
        assert_eq!(c.tick(t0(), None, 0), IntervalTick::Disarmed);
    }

    #[test]
    fn the_first_tick_arms_and_the_deadline_fires_then_re_arms() {
        let now = t0();
        let every = Duration::from_secs(3600);
        let c = IntervalCountdown::new(0);

        assert_eq!(
            c.tick(now, Some(every), 0),
            IntervalTick::Sleep(now + every)
        );
        assert_eq!(
            c.tick(now + Duration::from_secs(60), Some(every), 0),
            IntervalTick::Sleep(now + every)
        );

        let fire_at = now + every;
        assert_eq!(
            c.tick(fire_at, Some(every), 0),
            IntervalTick::Fire(fire_at + every)
        );
        // Re-armed, so it keeps cycling rather than firing every frame afterwards.
        assert_eq!(
            c.tick(fire_at + Duration::from_secs(1), Some(every), 0),
            IntervalTick::Sleep(fire_at + every)
        );
    }

    /// Any backup restarts the countdown — the whole point of tracking the epoch. A manual
    /// "Back up now" one minute before the deadline must not be followed by an interval
    /// backup a minute later.
    #[test]
    fn a_backup_from_anywhere_restarts_the_countdown() {
        let now = t0();
        let every = Duration::from_secs(3600);
        let c = IntervalCountdown::new(0);

        c.tick(now, Some(every), 0);

        // 59 minutes in, someone hits "Back up now" — the epoch moves.
        let manual = now + Duration::from_secs(3540);
        assert_eq!(
            c.tick(manual, Some(every), 1),
            IntervalTick::Sleep(manual + every)
        );

        // The original deadline passes with no fire.
        assert_eq!(
            c.tick(now + every, Some(every), 1),
            IntervalTick::Sleep(manual + every)
        );
        // A full interval after the manual backup, it fires.
        assert_eq!(
            c.tick(manual + every, Some(every), 1),
            IntervalTick::Fire(manual + every + every)
        );
    }

    /// Switching the interval off and back on re-arms from the moment it came back,
    /// rather than resuming a stale deadline.
    #[test]
    fn disarming_and_re_arming_starts_a_fresh_countdown() {
        let now = t0();
        let every = Duration::from_secs(600);
        let c = IntervalCountdown::new(0);

        c.tick(now, Some(every), 0);
        assert_eq!(
            c.tick(now + Duration::from_secs(60), None, 0),
            IntervalTick::Disarmed
        );

        let back = now + Duration::from_secs(120);
        assert_eq!(
            c.tick(back, Some(every), 0),
            IntervalTick::Sleep(back + every)
        );
    }
}
