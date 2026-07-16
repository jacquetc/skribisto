// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **writing session** — an ephemeral sprint timer + word tracker.
//!
//! A play/pause control in the status bar starts a focused writing sprint: an optional
//! word goal and/or time limit, a red→green gauge that fills as the words come, and the
//! time left. It is **ephemeral** — a stopwatch, not backend state: the running clock and
//! the words-this-session live only in this view-model (only the *targets* persist, so a
//! writer's usual goal is remembered). It never touches the store or the undo history.
//!
//! Words are tracked off [`StatsModel`]'s focused count and **accumulated across scene
//! switches**: when the writer moves to another scene mid-sprint, the words written in the
//! one they left are banked and the new scene re-baselines, so the total is "words written
//! this session" regardless of where they wrote them.
//!
//! Per the house rules, the timing/threshold decisions are the pure, `Instant`-injected
//! functions below (unit-tested); the view-model is the thin stateful shell over them.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use bastyde::prelude::*;
use bastyde::settings::SettingsStore;

use crate::models::StatsModel;

/// Persisted so a writer's usual sprint goal is remembered across launches. `0` = none.
const WORD_TARGET_KEY: &str = "session.word_target";
/// Persisted time limit in **minutes**. `0` = no limit (a pure word sprint).
const TIME_TARGET_MIN_KEY: &str = "session.time_target_min";

// ─────────────────────────────────────────────────────────────────────────────
// Pure core (Instant-injected — unit-tested)
// ─────────────────────────────────────────────────────────────────────────────

/// A pause-aware stopwatch: `accumulated` from previous runs plus the current run since
/// `started`. `None` start = paused/stopped.
#[derive(Debug, Default, Clone, Copy)]
pub struct SessionClock {
    started: Option<Instant>,
    accumulated: Duration,
}

impl SessionClock {
    pub fn running(&self) -> bool {
        self.started.is_some()
    }
    /// Begin (or resume) running. Idempotent while already running.
    pub fn start(&mut self, now: Instant) {
        if self.started.is_none() {
            self.started = Some(now);
        }
    }
    /// Freeze: bank the current run into `accumulated`.
    pub fn pause(&mut self, now: Instant) {
        if let Some(s) = self.started.take() {
            self.accumulated += now.saturating_duration_since(s);
        }
    }
    /// Back to zero, stopped.
    pub fn reset(&mut self) {
        self.started = None;
        self.accumulated = Duration::ZERO;
    }
    pub fn elapsed(&self, now: Instant) -> Duration {
        self.accumulated
            + self
                .started
                .map(|s| now.saturating_duration_since(s))
                .unwrap_or_default()
    }
}

/// Time left toward a target (`None` = no time limit). Saturates at zero (never negative).
pub fn remaining(elapsed: Duration, target: Option<Duration>) -> Option<Duration> {
    target.map(|t| t.saturating_sub(elapsed))
}

/// Progress toward the word goal in `0.0..=1.0`, or `None` when there is no word goal.
pub fn words_progress(written: i64, target: Option<i64>) -> Option<f32> {
    target
        .filter(|t| *t > 0)
        .map(|t| (written.max(0) as f32 / t as f32).clamp(0.0, 1.0))
}

/// The bucketed red→green gauge colour for a progress ratio — kept to semantic theme
/// roles (no colour interpolation primitive exists), so it works in light and dark.
pub fn gauge_role(progress: f32) -> TextRole {
    if progress < 0.34 {
        TextRole::Error
    } else if progress < 0.75 {
        TextRole::Warning
    } else {
        TextRole::Success
    }
}

/// `M:SS` of a duration (the status bar's compact time readout).
pub fn format_mmss(d: Duration) -> String {
    let s = d.as_secs();
    format!("{}:{:02}", s / 60, s % 60)
}

// ─────────────────────────────────────────────────────────────────────────────
// The view-model (ephemeral state; the widget wires the reactivity)
// ─────────────────────────────────────────────────────────────────────────────

struct Inner {
    stats: StatsModel,
    clock: RefCell<SessionClock>,
    running: Signal<bool>,
    /// Wall-clock elapsed, refreshed each second by [`poll`](WritingSessionViewModel::poll).
    elapsed: Signal<Duration>,
    /// Words written this session so far (accumulated across scene switches).
    session_words: Signal<i64>,
    /// Persisted targets (`0` = none).
    word_target: Signal<i64>,
    time_target_min: Signal<i64>,
    // Scene-switch accumulation.
    accumulated: Cell<i64>,
    scene_baseline: Cell<i64>,
    last_focused: Cell<i64>,
    current_scene: Cell<Option<u64>>,
}

/// Cloneable handle to the one live writing session.
#[derive(Clone)]
pub struct WritingSessionViewModel {
    inner: Rc<Inner>,
}

impl WritingSessionViewModel {
    pub fn new(stats: StatsModel, store: &SettingsStore) -> Self {
        Self {
            inner: Rc::new(Inner {
                stats,
                clock: RefCell::new(SessionClock::default()),
                running: Signal::new(false),
                elapsed: Signal::new(Duration::ZERO),
                session_words: Signal::new(0),
                word_target: store.signal(WORD_TARGET_KEY, 0i64),
                time_target_min: store.signal(TIME_TARGET_MIN_KEY, 0i64),
                accumulated: Cell::new(0),
                scene_baseline: Cell::new(0),
                last_focused: Cell::new(0),
                current_scene: Cell::new(None),
            }),
        }
    }

    pub fn running(&self) -> Signal<bool> {
        self.inner.running.clone()
    }
    pub fn elapsed_signal(&self) -> Signal<Duration> {
        self.inner.elapsed.clone()
    }
    pub fn session_words_signal(&self) -> Signal<i64> {
        self.inner.session_words.clone()
    }
    pub fn word_target(&self) -> Signal<i64> {
        self.inner.word_target.clone()
    }
    pub fn time_target_min(&self) -> Signal<i64> {
        self.inner.time_target_min.clone()
    }
    /// The edit + focus signals the widget binds so words refresh as the writer types.
    pub fn stats(&self) -> &StatsModel {
        &self.inner.stats
    }

    fn focused(&self) -> i64 {
        self.inner.stats.focused_word_count().unwrap_or(0) as i64
    }

    /// Start a fresh sprint / resume a paused one, or pause a running one.
    pub fn toggle(&self) {
        let now = Instant::now();
        if self.inner.running.get() {
            self.inner.clock.borrow_mut().pause(now);
            self.inner.running.set(false);
        } else {
            // A start from zero (not a resume) re-baselines the word tracker.
            let fresh = {
                let c = self.inner.clock.borrow();
                !c.running() && c.elapsed(now).is_zero()
            };
            if fresh {
                self.begin_baselines();
            }
            self.inner.clock.borrow_mut().start(now);
            self.inner.elapsed.set(self.inner.clock.borrow().elapsed(now));
            self.inner.running.set(true);
        }
    }

    /// Zero the clock and the word tracker (keeps the targets) — the context-menu action.
    pub fn reset(&self) {
        self.inner.clock.borrow_mut().reset();
        self.inner.running.set(false);
        self.inner.elapsed.set(Duration::ZERO);
        self.begin_baselines();
    }

    fn begin_baselines(&self) {
        let f = self.focused();
        self.inner.accumulated.set(0);
        self.inner.scene_baseline.set(f);
        self.inner.last_focused.set(f);
        self.inner.current_scene.set(self.inner.stats.active_item().get());
        self.inner.session_words.set(0);
    }

    /// Recompute words-this-session from the focused count — the widget calls this on
    /// every edit and focus change. A no-op while paused/stopped.
    pub fn recompute_words(&self) {
        if !self.inner.running.get() {
            return;
        }
        let id = self.inner.stats.active_item().get();
        let count = self.focused();
        if id != self.inner.current_scene.get() {
            // Moved to another scene: bank what was written in the one just left, then
            // re-baseline against the new scene.
            let banked = (self.inner.last_focused.get() - self.inner.scene_baseline.get()).max(0);
            self.inner.accumulated.set(self.inner.accumulated.get() + banked);
            self.inner.scene_baseline.set(count);
            self.inner.current_scene.set(id);
        }
        self.inner.last_focused.set(count);
        let written =
            self.inner.accumulated.get() + (count - self.inner.scene_baseline.get()).max(0);
        // Only publish a change — a keystroke that adds no word must not rebuild the item.
        if written != self.inner.session_words.get() {
            self.inner.session_words.set(written);
        }
    }

    /// Advance `elapsed` against the clock; returns the next 1-second wake while running
    /// (the widget arms it via `wake_at`, so the timer ticks without a 60 fps drain).
    pub fn poll(&self, now: Instant) -> Option<Instant> {
        if !self.inner.running.get() {
            return None;
        }
        let e = self.inner.clock.borrow().elapsed(now);
        // The readout is whole-seconds (M:SS) — publish only when the second rolls over,
        // so the frame-tick effect can't rebuild the status bar at 60 fps.
        if e.as_secs() != self.inner.elapsed.get().as_secs() {
            self.inner.elapsed.set(e);
        }
        Some(now + Duration::from_secs(1))
    }

    /// The time limit as a `Duration`, or `None`.
    pub fn time_target(&self) -> Option<Duration> {
        let m = self.inner.time_target_min.get();
        (m > 0).then(|| Duration::from_secs(m as u64 * 60))
    }
    /// The word goal, or `None`.
    pub fn word_target_opt(&self) -> Option<i64> {
        let w = self.inner.word_target.get();
        (w > 0).then_some(w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t0() -> Instant {
        Instant::now()
    }

    #[test]
    fn clock_accumulates_across_pause_and_resume() {
        let t = t0();
        let mut c = SessionClock::default();
        c.start(t);
        assert_eq!(c.elapsed(t + Duration::from_secs(5)), Duration::from_secs(5));
        c.pause(t + Duration::from_secs(5)); // banked 5s
        // Paused: time no longer advances.
        assert_eq!(c.elapsed(t + Duration::from_secs(9)), Duration::from_secs(5));
        c.start(t + Duration::from_secs(9)); // resume
        assert_eq!(c.elapsed(t + Duration::from_secs(11)), Duration::from_secs(7));
        c.reset();
        assert_eq!(c.elapsed(t + Duration::from_secs(20)), Duration::ZERO);
        assert!(!c.running());
    }

    #[test]
    fn start_is_idempotent_and_pause_banks() {
        let t = t0();
        let mut c = SessionClock::default();
        c.start(t);
        c.start(t + Duration::from_secs(3)); // must NOT reset the origin
        assert_eq!(c.elapsed(t + Duration::from_secs(4)), Duration::from_secs(4));
    }

    #[test]
    fn remaining_saturates_at_zero() {
        assert_eq!(remaining(Duration::from_secs(10), None), None);
        assert_eq!(
            remaining(Duration::from_secs(10), Some(Duration::from_secs(25))),
            Some(Duration::from_secs(15))
        );
        assert_eq!(
            remaining(Duration::from_secs(40), Some(Duration::from_secs(25))),
            Some(Duration::ZERO),
            "over the limit reads as 0 left, never negative"
        );
    }

    #[test]
    fn words_progress_ratio_and_no_goal() {
        assert_eq!(words_progress(250, None), None);
        assert_eq!(words_progress(250, Some(0)), None, "0 target = no goal");
        assert_eq!(words_progress(250, Some(500)), Some(0.5));
        assert_eq!(words_progress(600, Some(500)), Some(1.0), "clamped at full");
        assert_eq!(words_progress(-5, Some(500)), Some(0.0), "negative clamps to 0");
    }

    #[test]
    fn gauge_role_buckets_red_amber_green() {
        assert_eq!(gauge_role(0.0), TextRole::Error);
        assert_eq!(gauge_role(0.33), TextRole::Error);
        assert_eq!(gauge_role(0.34), TextRole::Warning);
        assert_eq!(gauge_role(0.74), TextRole::Warning);
        assert_eq!(gauge_role(0.75), TextRole::Success);
        assert_eq!(gauge_role(1.0), TextRole::Success);
    }

    #[test]
    fn format_mmss_pads_seconds() {
        assert_eq!(format_mmss(Duration::from_secs(0)), "0:00");
        assert_eq!(format_mmss(Duration::from_secs(9)), "0:09");
        assert_eq!(format_mmss(Duration::from_secs(65)), "1:05");
        assert_eq!(format_mmss(Duration::from_secs(1500)), "25:00");
    }
}
