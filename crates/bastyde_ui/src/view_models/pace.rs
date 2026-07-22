// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Book's writing **Pace** - deadline + schedule + progress statistics.
//!
//! Per the house rules the arithmetic decisions live in the pure, date-injected
//! functions at the top (unit-tested, no `Signal`s, no backend); the view-model
//! below is the thin reactive shell that mirrors a [`PaceModel`] into `Signal`s,
//! refreshes on the matching backend events, and exposes the statistics by
//! calling the pure functions with the current values. It exists only for a
//! **Book** container - `PaceViewModel::new` returns `None` otherwise, gated on
//! the same [`StreamLevel::for_container`] the stream and the tab use.
//!
//! "Current words" is the last recorded `ProgressSnapshot`'s per-book count, not
//! a live as-you-type figure (no such aggregate exists yet); it advances when the
//! project's word count is recorded.

use std::rc::Rc;

use bastyde::prelude::*;

use chrono::{Datelike, NaiveDate};

use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};

use crate::app_ids::AppIds;
use crate::models::{DailyCount, HolidayRow, MilestoneRow, PaceModel, StreamLevel};

// ─────────────────────────── pure stats core ───────────────────────────────

/// Words actually written on a day = the rise in the cumulative count since the
/// previously recorded day (never negative - a shrink counts as zero written).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DailyDelta {
    pub date: NaiveDate,
    pub words_written: i64,
}

/// A holiday span, inclusive of both ends - writing is not scheduled inside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HolidayRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

/// Per-day words written, derived from consecutive cumulative counts.
///
/// The first recorded day has no prior baseline, so it yields no delta:
/// `output.len() == history.len().saturating_sub(1)`. Precondition: `history` is
/// sorted ascending with at most one entry per date (the model guarantees both).
pub fn daily_deltas(history: &[DailyCount]) -> Vec<DailyDelta> {
    history
        .windows(2)
        .map(|w| DailyDelta {
            date: w[1].date,
            words_written: (w[1].words - w[0].words).max(0),
        })
        .collect()
}

/// The current writing streak: consecutive calendar days ending at `today` on
/// which the writer actually wrote (a delta with `words_written > 0`).
///
/// A calendar gap (no snapshot that day — a weekend, or the app was not
/// opened) breaks the streak exactly like a zero-word day: the streak is "did
/// you write", not "were you scheduled to".
pub fn streak(deltas: &[DailyDelta], today: NaiveDate) -> u32 {
    let mut count = 0;
    let mut day = today;
    while deltas.iter().any(|d| d.date == day && d.words_written > 0) {
        count += 1;
        let Some(prev) = day.pred_opt() else { break };
        day = prev;
    }
    count
}

/// Is `date` a scheduled writing day - its weekday is in the mask and it is not
/// inside any holiday? Bit convention: Mon = 1, Tue = 2, Wed = 4, Thu = 8,
/// Fri = 16, Sat = 32, Sun = 64.
pub fn is_scheduled_day(date: NaiveDate, weekday_mask: i64, holidays: &[HolidayRange]) -> bool {
    let bit = 1_i64 << date.weekday().num_days_from_monday();
    (weekday_mask & bit) != 0 && !holidays.iter().any(|h| date >= h.start && date <= h.end)
}

/// How many scheduled writing days fall in `[start, end]` inclusive (0 if
/// `start > end`).
pub fn writing_days_in_range(
    start: NaiveDate,
    end: NaiveDate,
    weekday_mask: i64,
    holidays: &[HolidayRange],
) -> u32 {
    if start > end {
        return 0;
    }
    let mut n = 0;
    let mut d = start;
    loop {
        if is_scheduled_day(d, weekday_mask, holidays) {
            n += 1;
        }
        if d == end {
            break;
        }
        let Some(next) = d.succ_opt() else { break };
        d = next;
    }
    n
}

/// Scheduled writing days remaining from `today` to `end` inclusive - `today`
/// itself counts if scheduled; 0 once `today > end`.
pub fn writing_days_left(
    today: NaiveDate,
    end: NaiveDate,
    weekday_mask: i64,
    holidays: &[HolidayRange],
) -> u32 {
    writing_days_in_range(today, end, weekday_mask, holidays)
}

/// Words that *should* be done by the start of `today` to stay on an even pace -
/// `goal` spread across every scheduled day in `[start, end]`, times the days
/// already elapsed (`[start, today - 1]`, i.e. "by end of yesterday"; today's own
/// quota is not due yet). Clamped to `[0, goal]`.
pub fn expected_words_by(
    today: NaiveDate,
    start: NaiveDate,
    end: NaiveDate,
    weekday_mask: i64,
    holidays: &[HolidayRange],
    goal: i64,
) -> i64 {
    let total = writing_days_in_range(start, end, weekday_mask, holidays);
    if total == 0 || goal <= 0 {
        return 0;
    }
    let elapsed = match today.pred_opt() {
        Some(prev) if prev >= start => writing_days_in_range(start, prev, weekday_mask, holidays),
        _ => 0,
    };
    let per_day = goal as f64 / total as f64;
    ((per_day * elapsed as f64).round() as i64).clamp(0, goal)
}

/// How far ahead (+) or behind (−) an even pace the writer is: actual − expected.
pub fn ahead_behind(actual_words: i64, expected_words: i64) -> i64 {
    actual_words - expected_words
}

/// The ideal cumulative word count *by the end of* `date` on an even pace: the
/// goal spread over every scheduled day in `[start, end]`, times the scheduled
/// days elapsed through `date` inclusive. Clamped to `[0, goal]`. This is the
/// target line the progression chart plots against the actual cumulative - note
/// it counts `date` itself (a target *line*), unlike [`expected_words_by`] which
/// stops at yesterday (words *due* so far).
pub fn target_cumulative(
    date: NaiveDate,
    start: NaiveDate,
    end: NaiveDate,
    weekday_mask: i64,
    holidays: &[HolidayRange],
    goal: i64,
) -> i64 {
    let total = writing_days_in_range(start, end, weekday_mask, holidays);
    if total == 0 || goal <= 0 {
        return 0;
    }
    let elapsed = writing_days_in_range(start, date, weekday_mask, holidays);
    let per_day = goal as f64 / total as f64;
    ((per_day * elapsed as f64).round() as i64).clamp(0, goal)
}

/// Fraction of the goal reached, clamped to `[0, 1]`, or `None` when no goal is
/// set (`goal <= 0`).
pub fn percent_done(current_words: i64, goal_words: i64) -> Option<f32> {
    if goal_words <= 0 {
        return None;
    }
    Some((current_words as f32 / goal_words as f32).clamp(0.0, 1.0))
}

/// The daily rate needed to hit the goal by the deadline: remaining words spread
/// over the scheduled days left (rounded up). `None` when no scheduled days
/// remain (deadline reached / nothing scheduled).
pub fn words_per_writing_day(remaining_words: i64, writing_days_left: u32) -> Option<i64> {
    if writing_days_left == 0 {
        return None;
    }
    Some((remaining_words.max(0) as f64 / writing_days_left as f64).ceil() as i64)
}

/// The date the "days left to the deadline" window starts at: the later of today
/// and the plan's start, so a schedule that has not begun yet (`start > today`)
/// does not count the days before it started as remaining.
pub fn effective_start(today: NaiveDate, plan_start: Option<NaiveDate>) -> NaiveDate {
    plan_start.map_or(today, |s| s.max(today))
}

// ─────────────────────────── the view-model ────────────────────────────────

struct Inner {
    model: PaceModel,
    /// This Book's item id - used to ignore `BinderItem(Updated)` events for
    /// *other* items (only this Book's own goal edit is relevant).
    book_item_id: u64,
    pace_id: Signal<Option<u64>>,
    start: Signal<Option<NaiveDate>>,
    end: Signal<Option<NaiveDate>>,
    weekday_mask: Signal<i64>,
    active: Signal<bool>,
    goal_words: Signal<i64>,
    holidays: Signal<Vec<HolidayRow>>,
    milestones: Signal<Vec<MilestoneRow>>,
    history: Signal<Vec<DailyCount>>,
    /// The last recorded cumulative Book word count - cached from `history` on
    /// each reload so the hot stats accessors read one `i64` instead of cloning
    /// the whole history `Vec` on every call.
    current_words: Signal<i64>,
    /// Bumped on every reload - a single "something changed" trigger the pane's
    /// derived stat text binds to, so the statistics (which combine several
    /// signals plus `today`) recompute on any change without zipping them all.
    version: Signal<u64>,
}

/// The Book's writing-plan view-model - a cloneable handle over one `PaceModel`.
#[derive(Clone)]
pub struct PaceViewModel {
    inner: Rc<Inner>,
}

// Not every affordance has a caller until the Pace pane (M4c) is built out.
#[allow(dead_code)]
impl PaceViewModel {
    /// A Pace view-model for this container, or `None` unless it is a Book - the
    /// gate is [`StreamLevel::for_container`], the same one the tab and stream use.
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        book_item_id: u64,
        role: &BinderItemRole,
        sub_role: &BinderItemSubRole,
    ) -> Option<Self> {
        if !matches!(
            StreamLevel::for_container(role, sub_role),
            Some(StreamLevel::Book)
        ) {
            return None;
        }
        let vm = Self {
            inner: Rc::new(Inner {
                model: PaceModel::new(app_ctx, ids, book_item_id),
                book_item_id,
                pace_id: Signal::new(None),
                start: Signal::new(None),
                end: Signal::new(None),
                weekday_mask: Signal::new(0),
                active: Signal::new(false),
                goal_words: Signal::new(0),
                holidays: Signal::new(Vec::new()),
                milestones: Signal::new(Vec::new()),
                history: Signal::new(Vec::new()),
                current_words: Signal::new(0),
                version: Signal::new(0),
            }),
        };
        vm.reload();
        Some(vm)
    }

    /// Refresh when the Pace, its Holidays/Milestones, the Book's goal, or a
    /// recorded ProgressSnapshot change. Subscribed **every build** (a `Weak`
    /// capture, so a closed tab's `Inner` is freed), mirroring `StreamViewModel::wire`.
    pub fn wire(&self, ctx: &mut BuildContext) {
        use DirectAccessEntity::{BinderItem, Holiday, Milestone, Pace, ProgressSnapshot};
        use EntityEvent::{Created, Removed, Updated};
        // Pace / Holiday / Milestone / ProgressSnapshot events don't carry this
        // Book's id, so reload unconditionally on any of them.
        let origins = [
            Origin::DirectAccess(Pace(Created)),
            Origin::DirectAccess(Pace(Updated)),
            Origin::DirectAccess(Pace(Removed)),
            Origin::DirectAccess(Holiday(Created)),
            Origin::DirectAccess(Holiday(Updated)),
            Origin::DirectAccess(Holiday(Removed)),
            Origin::DirectAccess(Milestone(Created)),
            Origin::DirectAccess(Milestone(Updated)),
            Origin::DirectAccess(Milestone(Removed)),
            Origin::DirectAccess(ProgressSnapshot(Created)),
            Origin::DirectAccess(ProgressSnapshot(Updated)),
        ];
        for origin in origins {
            let weak = Rc::downgrade(&self.inner);
            ctx.subscribe_event(origin, move |_event: &Event| {
                if let Some(inner) = weak.upgrade() {
                    PaceViewModel { inner }.reload();
                }
            });
        }
        // The goal is a `BinderItem` field: reload only when *this Book's* item
        // changed, not on every binder edit elsewhere in the Work.
        let weak = Rc::downgrade(&self.inner);
        ctx.subscribe_event(
            Origin::DirectAccess(BinderItem(Updated)),
            move |event: &Event| {
                if let Some(inner) = weak.upgrade()
                    && event.ids.contains(&inner.book_item_id)
                {
                    PaceViewModel { inner }.reload();
                }
            },
        );
    }

    /// Pull a fresh `PaceState` and push
    /// each field into its `Signal` - only when it changed, to avoid needless
    /// repaints on an unrelated event.
    fn reload(&self) {
        let s = self.inner.model.load();
        let current = s.history.last().map(|d| d.words).unwrap_or(0);
        set_changed(&self.inner.pace_id, s.pace_id);
        set_changed(&self.inner.start, s.start);
        set_changed(&self.inner.end, s.end);
        set_changed(&self.inner.weekday_mask, s.weekday_mask);
        set_changed(&self.inner.active, s.active);
        set_changed(&self.inner.goal_words, s.goal_words);
        set_changed(&self.inner.holidays, s.holidays);
        set_changed(&self.inner.milestones, s.milestones);
        set_changed(&self.inner.history, s.history);
        set_changed(&self.inner.current_words, current);
        let v = &self.inner.version;
        v.set(v.get().wrapping_add(1));
    }

    /// Bumped on every reload; the pane binds derived stat text to it so the
    /// statistics recompute on any change.
    pub fn version(&self) -> Signal<u64> {
        self.inner.version.clone()
    }

    // ── reactive reads (bind these) ──

    pub fn pace_id(&self) -> Signal<Option<u64>> {
        self.inner.pace_id.clone()
    }
    pub fn start(&self) -> Signal<Option<NaiveDate>> {
        self.inner.start.clone()
    }
    pub fn end(&self) -> Signal<Option<NaiveDate>> {
        self.inner.end.clone()
    }
    pub fn weekday_mask(&self) -> Signal<i64> {
        self.inner.weekday_mask.clone()
    }
    pub fn active(&self) -> Signal<bool> {
        self.inner.active.clone()
    }
    pub fn goal_words(&self) -> Signal<i64> {
        self.inner.goal_words.clone()
    }
    pub fn holidays(&self) -> Signal<Vec<HolidayRow>> {
        self.inner.holidays.clone()
    }
    pub fn milestones(&self) -> Signal<Vec<MilestoneRow>> {
        self.inner.milestones.clone()
    }
    pub fn history(&self) -> Signal<Vec<DailyCount>> {
        self.inner.history.clone()
    }

    // ── derived statistics (inject `today` - the pane passes the real date) ──

    /// The last recorded cumulative Book word count (0 with no history) - read
    /// from the cached `Signal`, not by re-scanning `history`.
    pub fn current_words(&self) -> i64 {
        self.inner.current_words.get()
    }

    pub fn streak(&self, today: NaiveDate) -> u32 {
        streak(&daily_deltas(&self.inner.history.get()), today)
    }

    pub fn percent_done(&self) -> Option<f32> {
        percent_done(self.current_words(), self.inner.goal_words.get())
    }

    /// Scheduled writing days remaining to the deadline. Counts from the later of
    /// `today` and the plan's `start` - a schedule that has not begun yet
    /// (`today < start`) must not count the days before it started as "left".
    pub fn writing_days_left(&self, today: NaiveDate) -> u32 {
        let Some(end) = self.inner.end.get() else {
            return 0;
        };
        let from = effective_start(today, self.inner.start.get());
        writing_days_left(
            from,
            end,
            self.inner.weekday_mask.get(),
            &self.holiday_ranges(),
        )
    }

    /// The daily rate needed to hit the goal by the deadline, or `None` when no
    /// goal is set - mirroring [`percent_done`](Self::percent_done) and
    /// [`ahead_behind`](Self::ahead_behind) rather than reporting a spurious `0`.
    pub fn words_per_writing_day(&self, today: NaiveDate) -> Option<i64> {
        let goal = self.inner.goal_words.get();
        if goal <= 0 {
            return None;
        }
        let remaining = goal - self.current_words();
        words_per_writing_day(remaining, self.writing_days_left(today))
    }

    /// Words ahead (+) / behind (−) an even pace, or `None` until a start, end,
    /// and goal are all set.
    pub fn ahead_behind(&self, today: NaiveDate) -> Option<i64> {
        let start = self.inner.start.get()?;
        let end = self.inner.end.get()?;
        let goal = self.inner.goal_words.get();
        if goal <= 0 {
            return None;
        }
        let expected = expected_words_by(
            today,
            start,
            end,
            self.inner.weekday_mask.get(),
            &self.holiday_ranges(),
            goal,
        );
        Some(ahead_behind(self.current_words(), expected))
    }

    /// The recorded cumulative Book word count per day - the progression chart's
    /// "actual" line, ascending by date.
    pub fn actual_series(&self) -> Vec<(NaiveDate, i64)> {
        self.inner
            .history
            .get()
            .iter()
            .map(|d| (d.date, d.words))
            .collect()
    }

    /// Words actually written each day (the rise since the previous recorded day)
    /// - the words-per-day bar chart. Drops the first (baseline) day.
    pub fn words_per_day(&self) -> Vec<(NaiveDate, i64)> {
        daily_deltas(&self.inner.history.get())
            .into_iter()
            .map(|d| (d.date, d.words_written))
            .collect()
    }

    /// The ideal cumulative words by `date` on an even pace - the progression
    /// chart's target line. `None` until a start, end, and goal are all set.
    pub fn target_for(&self, date: NaiveDate) -> Option<i64> {
        let start = self.inner.start.get()?;
        let end = self.inner.end.get()?;
        let goal = self.inner.goal_words.get();
        if goal <= 0 {
            return None;
        }
        Some(target_cumulative(
            date,
            start,
            end,
            self.inner.weekday_mask.get(),
            &self.holiday_ranges(),
            goal,
        ))
    }

    /// The even-pace daily target: the goal spread over every scheduled day.
    /// `None` until a start, end and goal are all set. The words-per-day chart
    /// flags any day below this as behind pace.
    pub fn target_daily_rate(&self) -> Option<i64> {
        let start = self.inner.start.get()?;
        let end = self.inner.end.get()?;
        let goal = self.inner.goal_words.get();
        if goal <= 0 {
            return None;
        }
        let total = writing_days_in_range(
            start,
            end,
            self.inner.weekday_mask.get(),
            &self.holiday_ranges(),
        );
        if total == 0 {
            return None;
        }
        Some((goal as f64 / total as f64).round() as i64)
    }

    fn holiday_ranges(&self) -> Vec<HolidayRange> {
        self.inner
            .holidays
            .get()
            .iter()
            .map(|h| HolidayRange {
                start: h.start,
                end: h.end,
            })
            .collect()
    }

    // ── mutations (write through the model, then refresh our Signals) ──

    pub fn set_dates(&self, start: NaiveDate, end: NaiveDate) {
        self.inner.model.set_dates(start, end);
        self.reload();
    }
    pub fn set_weekday_mask(&self, mask: i64) {
        self.inner.model.set_weekday_mask(mask);
        self.reload();
    }
    pub fn set_active(&self, active: bool) {
        self.inner.model.set_active(active);
        self.reload();
    }
    pub fn set_goal_words(&self, goal: i64) {
        self.inner.model.set_goal_words(goal);
        self.reload();
    }
    pub fn add_holiday(&self, label: String, start: NaiveDate, end: Option<NaiveDate>) {
        self.inner.model.add_holiday(label, start, end);
        self.reload();
    }
    pub fn remove_holiday(&self, holiday_id: u64) {
        self.inner.model.remove_holiday(holiday_id);
        self.reload();
    }
    pub fn add_milestone(&self, label: String, target_date: NaiveDate, target_words: Option<i64>) {
        self.inner
            .model
            .add_milestone(label, target_date, target_words);
        self.reload();
    }
    pub fn remove_milestone(&self, milestone_id: u64) {
        self.inner.model.remove_milestone(milestone_id);
        self.reload();
    }
}

/// Set a `Signal` only if its value actually changed - avoids repaint churn when
/// an unrelated backend event triggers a reload.
fn set_changed<T: Clone + PartialEq + 'static>(sig: &Signal<T>, v: T) {
    if sig.get() != v {
        sig.set(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2026-07-13 is a Monday.
    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    fn counts(days: &[(NaiveDate, i64)]) -> Vec<DailyCount> {
        days.iter()
            .map(|&(date, words)| DailyCount { date, words })
            .collect()
    }

    #[test]
    fn daily_deltas_drops_the_baseline_and_floors_at_zero() {
        let h = counts(&[
            (d(2026, 7, 13), 1000),
            (d(2026, 7, 14), 1700),
            (d(2026, 7, 15), 1700), // flat day
            (d(2026, 7, 16), 1500), // a shrink → floored to 0
            (d(2026, 7, 17), 2200),
        ]);
        let deltas = daily_deltas(&h);
        assert_eq!(deltas.len(), 4, "the first day is the baseline, no delta");
        assert_eq!(deltas[0].words_written, 700);
        assert_eq!(deltas[1].words_written, 0);
        assert_eq!(
            deltas[2].words_written, 0,
            "a shrink counts as zero written"
        );
        assert_eq!(deltas[3].words_written, 700);
    }

    #[test]
    fn streak_counts_back_to_the_first_gap_or_zero() {
        // Wrote Mon, Tue, Wed; flat Thu; wrote Fri. Streak on Fri = 1 (Thu breaks it).
        let deltas = daily_deltas(&counts(&[
            (d(2026, 7, 13), 100), // Mon baseline
            (d(2026, 7, 14), 300), // Tue +200
            (d(2026, 7, 15), 500), // Wed +200
            (d(2026, 7, 16), 500), // Thu +0
            (d(2026, 7, 17), 900), // Fri +400
        ]));
        assert_eq!(streak(&deltas, d(2026, 7, 17)), 1);
        // A missing calendar day breaks it too: no Thu entry at all.
        let gapped = daily_deltas(&counts(&[
            (d(2026, 7, 13), 100),
            (d(2026, 7, 14), 300),
            (d(2026, 7, 17), 900), // Fri, skipping Wed/Thu
        ]));
        assert_eq!(
            streak(&gapped, d(2026, 7, 17)),
            1,
            "a calendar gap ends the streak"
        );
        // Three straight writing days.
        let run = daily_deltas(&counts(&[
            (d(2026, 7, 14), 100),
            (d(2026, 7, 15), 300),
            (d(2026, 7, 16), 500),
            (d(2026, 7, 17), 900),
        ]));
        assert_eq!(streak(&run, d(2026, 7, 17)), 3);
        assert_eq!(streak(&run, d(2026, 7, 20)), 0, "no entry today → zero");
    }

    #[test]
    fn is_scheduled_day_respects_mask_and_holidays() {
        let mon_fri = MON_TO_FRI_TEST;
        // 2026-07-13 Mon .. 2026-07-19 Sun.
        assert!(is_scheduled_day(d(2026, 7, 13), mon_fri, &[]), "Mon");
        assert!(is_scheduled_day(d(2026, 7, 17), mon_fri, &[]), "Fri");
        assert!(!is_scheduled_day(d(2026, 7, 18), mon_fri, &[]), "Sat off");
        assert!(!is_scheduled_day(d(2026, 7, 19), mon_fri, &[]), "Sun off");
        // A holiday knocks out an otherwise-scheduled Wednesday.
        let hol = [HolidayRange {
            start: d(2026, 7, 15),
            end: d(2026, 7, 15),
        }];
        assert!(!is_scheduled_day(d(2026, 7, 15), mon_fri, &hol), "holiday");
        assert!(
            is_scheduled_day(d(2026, 7, 14), mon_fri, &hol),
            "Tue still on"
        );
    }

    const MON_TO_FRI_TEST: i64 = 0b0001_1111;

    #[test]
    fn writing_days_counts_scheduled_days_only() {
        // Mon 13th .. Sun 19th: 5 weekdays.
        assert_eq!(
            writing_days_in_range(d(2026, 7, 13), d(2026, 7, 19), MON_TO_FRI_TEST, &[]),
            5
        );
        // Minus a one-day holiday → 4.
        let hol = [HolidayRange {
            start: d(2026, 7, 15),
            end: d(2026, 7, 15),
        }];
        assert_eq!(
            writing_days_in_range(d(2026, 7, 13), d(2026, 7, 19), MON_TO_FRI_TEST, &hol),
            4
        );
        // Reversed range → 0.
        assert_eq!(
            writing_days_in_range(d(2026, 7, 19), d(2026, 7, 13), MON_TO_FRI_TEST, &[]),
            0
        );
        // `writing_days_left` on the last day counts it if scheduled (Fri).
        assert_eq!(
            writing_days_left(d(2026, 7, 17), d(2026, 7, 17), MON_TO_FRI_TEST, &[]),
            1
        );
        // Past the deadline → 0.
        assert_eq!(
            writing_days_left(d(2026, 7, 20), d(2026, 7, 17), MON_TO_FRI_TEST, &[]),
            0
        );
    }

    #[test]
    fn expected_and_ahead_behind() {
        // 10 scheduled days (two Mon–Fri weeks), goal 1000 → 100/day.
        let start = d(2026, 7, 13); // Mon
        let end = d(2026, 7, 24); // Fri (two weeks later)
        assert_eq!(writing_days_in_range(start, end, MON_TO_FRI_TEST, &[]), 10);
        // "By" Wed the 15th = end of Tue = 2 elapsed scheduled days → 200 expected.
        let exp = expected_words_by(d(2026, 7, 15), start, end, MON_TO_FRI_TEST, &[], 1000);
        assert_eq!(exp, 200);
        assert_eq!(ahead_behind(350, exp), 150, "350 done vs 200 → +150 ahead");
        assert_eq!(ahead_behind(120, exp), -80, "behind");
        // Before the start nothing is due; clamp at 0.
        assert_eq!(
            expected_words_by(start, start, end, MON_TO_FRI_TEST, &[], 1000),
            0
        );
        // No scheduled days → 0, never a divide-by-zero.
        assert_eq!(
            expected_words_by(d(2026, 7, 15), start, end, 0, &[], 1000),
            0
        );
    }

    #[test]
    fn target_cumulative_is_the_inclusive_pace_line() {
        // 10 scheduled days (Mon 13th → Fri 24th), goal 1000 → 100/scheduled-day.
        let start = d(2026, 7, 13);
        let end = d(2026, 7, 24);
        // By end of the 2nd scheduled day (Tue 14th): 2 days elapsed inclusive → 200.
        assert_eq!(
            target_cumulative(d(2026, 7, 14), start, end, MON_TO_FRI_TEST, &[], 1000),
            200
        );
        // The start day itself counts (inclusive) → 100, unlike expected_words_by=0.
        assert_eq!(
            target_cumulative(start, start, end, MON_TO_FRI_TEST, &[], 1000),
            100
        );
        // At/after the deadline → the whole goal.
        assert_eq!(
            target_cumulative(end, start, end, MON_TO_FRI_TEST, &[], 1000),
            1000
        );
        // No goal / no scheduled days → 0, never a divide-by-zero.
        assert_eq!(
            target_cumulative(end, start, end, MON_TO_FRI_TEST, &[], 0),
            0
        );
        assert_eq!(target_cumulative(end, start, end, 0, &[], 1000), 0);
    }

    #[test]
    fn percent_and_rate() {
        assert_eq!(percent_done(2500, 10_000), Some(0.25));
        assert_eq!(percent_done(50_000, 10_000), Some(1.0), "clamped to 1");
        assert_eq!(percent_done(500, 0), None, "no goal");
        // 900 words over 4 days → 225/day (ceil).
        assert_eq!(words_per_writing_day(900, 4), Some(225));
        // Already done → 0/day, still Some.
        assert_eq!(words_per_writing_day(-50, 4), Some(0));
        // No days left → None.
        assert_eq!(words_per_writing_day(900, 0), None);
    }

    #[test]
    fn effective_start_clamps_a_future_schedule() {
        let today = d(2026, 7, 15);
        // A schedule that has not begun yet starts counting at its start, not today.
        assert_eq!(
            effective_start(today, Some(d(2026, 8, 3))),
            d(2026, 8, 3),
            "future start wins"
        );
        // A schedule already under way counts from today.
        assert_eq!(
            effective_start(today, Some(d(2026, 7, 1))),
            today,
            "past start → today"
        );
        // No start set → today.
        assert_eq!(effective_start(today, None), today);
    }
}
