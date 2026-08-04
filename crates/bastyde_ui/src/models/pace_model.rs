// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Layer-A backend seam for a Book's writing **Pace** - its schedule (start/end
//! dates, the counted-weekday mask, the active flag), its Holidays and
//! Milestones, the Book's word-count goal, and the recorded ProgressSnapshot
//! history for the open Work.
//!
//! I/O only: [`load`](imp::PaceModel::load) reads a whole [`PaceState`] in one
//! shot; the mutators create the Pace lazily (on the first edit) and update it
//! and its children. The reactive `Signal`s and the pure stats live one layer
//! up in [`PaceViewModel`](crate::view_models::PaceViewModel), which calls
//! [`load`](imp::PaceModel::load) after every mutation and on the matching
//! backend `Updated` events.
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface - the real one
//! over `frontend` commands, the mock one over an in-memory `RefCell` - per the
//! model-layer convention (see [`crate::models`]). The plain data types below are
//! declared once, un-gated, so both variants and the view-model speak the same
//! vocabulary.

use chrono::NaiveDate;

/// One holiday span during which writing is paused. Inclusive of both ends; a
/// single-day holiday has `start == end`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HolidayRow {
    pub id: u64,
    pub label: String,
    pub start: NaiveDate,
    pub end: NaiveDate,
}

/// One milestone: a target date (and an optional word target) the writer set for
/// a Part/Chapter or the Book itself, shown along the Book's Pace timeline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MilestoneRow {
    pub id: u64,
    pub label: String,
    pub target_date: NaiveDate,
    pub target_words: Option<i64>,
}

/// One recorded day's **cumulative** Book word count, taken from a
/// `ProgressSnapshot`'s per-book entry. The delta actually written on a day is
/// derived from consecutive entries by the view-model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DailyCount {
    pub date: NaiveDate,
    pub words: i64,
}

/// A full snapshot of a Book's pace state, loaded in one shot. `pace_id` is
/// `None` until the Pace entity is first created - creation is lazy, on the
/// first edit, so merely viewing a Book never writes one.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PaceState {
    pub pace_id: Option<u64>,
    pub start: Option<NaiveDate>,
    pub end: Option<NaiveDate>,
    /// Bit field, Mon = 1, Tue = 2, … Sun = 64 (see `is_scheduled_day`).
    pub weekday_mask: i64,
    pub active: bool,
    pub goal_words: i64,
    pub holidays: Vec<HolidayRow>,
    pub milestones: Vec<MilestoneRow>,
    /// Ascending by date, one entry per day.
    pub history: Vec<DailyCount>,
}

/// Mon–Fri, the default counted-weekday mask for a freshly created Pace.
pub const MON_TO_FRI: i64 = 0b0001_1111;

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::rc::Rc;

    use chrono::{DateTime, Duration, NaiveDate, Utc};

    use frontend::AppContext;
    use frontend::commands::{
        binder_item_commands, holiday_commands, milestone_commands, pace_commands,
        progress_snapshot_commands, work_commands,
    };
    use frontend::common::direct_access::pace::PaceRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::types::EntityId;
    use frontend::direct_access::{
        CreateHolidayDto, CreateMilestoneDto, CreatePaceDto, PaceDto, PaceRelationshipDto,
        UpdateBinderItemDto, UpdatePaceDto,
    };

    use crate::app_ids::AppIds;

    use super::{DailyCount, HolidayRow, MON_TO_FRI, MilestoneRow, PaceState};

    /// UTC midnight for a calendar day - the on-disk convention (`record_progress_snapshot`
    /// truncates to UTC midnight, so dates round-trip through the same instant).
    fn to_utc(d: NaiveDate) -> DateTime<Utc> {
        d.and_hms_opt(0, 0, 0).expect("midnight is valid").and_utc()
    }

    pub struct PaceModel {
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        book_item_id: u64,
    }

    impl PaceModel {
        pub fn new(app_ctx: Rc<AppContext>, ids: AppIds, book_item_id: u64) -> Self {
            Self {
                app_ctx,
                ids,
                book_item_id,
            }
        }

        pub fn load(&self) -> PaceState {
            let ctx = &self.app_ctx;

            // The Book's goal lives on the BinderItem, independent of whether a
            // Pace exists yet.
            let goal_words = binder_item_commands::get_binder_item(ctx, &self.book_item_id)
                .ok()
                .flatten()
                .map(|it| it.word_count_goal)
                .unwrap_or(0);

            // The recorded history is every ProgressSnapshot in the store (one
            // Work per process → all belong to this Work's WorkInfo), projected
            // onto this Book's per-book count. Sorted + de-duped by day
            // defensively: the writer upserts one row per day, but we must not
            // depend on relationship order being chronological.
            let mut history: Vec<DailyCount> =
                progress_snapshot_commands::get_all_progress_snapshot(ctx)
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|s| {
                        let pos = s
                            .book_item_ids
                            .iter()
                            .position(|&id| id == self.book_item_id)?;
                        Some(DailyCount {
                            date: s.day.date_naive(),
                            words: *s.book_word_counts.get(pos)?,
                        })
                    })
                    .collect();
            history.sort_by_key(|d| d.date);
            // Collapse same-day duplicates deterministically. `get_all_progress_snapshot`
            // returns rows in the store's arbitrary `HashMap` order, so a first-wins
            // `dedup_by_key` could keep a stale count (and even flip between app runs);
            // a cumulative count only grows, so keep the largest per day. The normal
            // single-writer upsert never produces duplicates - a hand-edited or
            // merge-conflicted `snapshots.ron` can.
            history.dedup_by(|a, b| {
                if a.date == b.date {
                    b.words = b.words.max(a.words);
                    true
                } else {
                    false
                }
            });

            let Some(pace) = self.find_pace() else {
                // No Pace yet: still surface the goal + history so the view can
                // show progress and offer to set a schedule.
                return PaceState {
                    goal_words,
                    history,
                    ..Default::default()
                };
            };

            let holidays = holiday_commands::get_holiday_multi(ctx, &pace.holidays)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .map(|h| HolidayRow {
                    id: h.id,
                    label: h.label,
                    start: h.start_date.date_naive(),
                    end: h
                        .end_date
                        .map(|d| d.date_naive())
                        .unwrap_or_else(|| h.start_date.date_naive()),
                })
                .collect();

            let milestones = milestone_commands::get_milestone_multi(ctx, &pace.milestones)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .map(|m| MilestoneRow {
                    id: m.id,
                    label: m.label,
                    target_date: m.target_date.date_naive(),
                    target_words: m.target_word_count,
                })
                .collect();

            PaceState {
                pace_id: Some(pace.id),
                start: Some(pace.start_date.date_naive()),
                end: Some(pace.end_date.date_naive()),
                weekday_mask: pace.weekday_mask,
                active: pace.active,
                goal_words,
                holidays,
                milestones,
                history,
            }
        }

        /// Setting the deadline is the deliberate "plan my Book" action - the one
        /// mutator that may *create* the Pace (with the writer's own dates). Every
        /// other edit only refines an already-created schedule, so none of them
        /// fabricates a Pace from a stray click.
        pub fn set_dates(&self, start: NaiveDate, end: NaiveDate) {
            if let Some(id) = self.ensure_pace() {
                self.update_scalars(id, |dto| {
                    dto.start_date = to_utc(start);
                    dto.end_date = to_utc(end);
                });
            }
        }

        pub fn set_weekday_mask(&self, mask: i64) {
            if let Some(id) = self.existing_pace_id() {
                self.update_scalars(id, |dto| dto.weekday_mask = mask);
            }
        }

        pub fn set_active(&self, active: bool) {
            if let Some(id) = self.existing_pace_id() {
                self.update_scalars(id, |dto| dto.active = active);
            }
        }

        /// The goal is a field on the **BinderItem**, not the Pace - write it the
        /// same way the outline / stream do (scalar `UpdateBinderItemDto`).
        pub fn set_goal_words(&self, goal: i64) {
            let ctx = &self.app_ctx;
            let Some(mut it) = binder_item_commands::get_binder_item(ctx, &self.book_item_id)
                .ok()
                .flatten()
            else {
                return;
            };
            it.word_count_goal = goal;
            if let Err(e) = binder_item_commands::update_binder_item(
                ctx,
                self.stack(),
                &UpdateBinderItemDto::from(it),
            ) {
                eprintln!("pace: set goal words failed: {e}");
            }
        }

        pub fn add_holiday(&self, label: String, start: NaiveDate, end: Option<NaiveDate>) {
            // Take the existing list from the fetched Pace, not a second
            // relationship read - a swallowed read error there would feed an
            // empty list into the REPLACE-semantics `set_children` and wipe the
            // other holidays.
            let Some(pace) = self.find_pace() else { return };
            let now = Utc::now();
            let dto = CreateHolidayDto {
                created_at: now,
                updated_at: now,
                label,
                start_date: to_utc(start),
                end_date: end.map(to_utc),
            };
            let Ok(h) = holiday_commands::create_orphan_holiday(&self.app_ctx, self.stack(), &dto)
            else {
                return;
            };
            let mut ids = pace.holidays;
            ids.push(h.id);
            self.set_children(pace.id, PaceRelationshipField::Holidays, ids);
        }

        pub fn remove_holiday(&self, holiday_id: u64) {
            let Some(pace) = self.find_pace() else { return };
            let ids: Vec<EntityId> = pace
                .holidays
                .into_iter()
                .filter(|&id| id != holiday_id)
                .collect();
            self.set_children(pace.id, PaceRelationshipField::Holidays, ids);
            // Detaching leaves the row orphaned (Holiday has no owner) - delete it.
            if let Err(e) =
                holiday_commands::remove_holiday(&self.app_ctx, self.stack(), &holiday_id)
            {
                eprintln!("pace: remove holiday failed: {e}");
            }
        }

        pub fn add_milestone(
            &self,
            label: String,
            target_date: NaiveDate,
            target_words: Option<i64>,
        ) {
            let Some(pace) = self.find_pace() else { return };
            let now = Utc::now();
            let dto = CreateMilestoneDto {
                created_at: now,
                updated_at: now,
                label,
                target_item: None,
                target_date: to_utc(target_date),
                target_word_count: target_words,
            };
            let Ok(m) =
                milestone_commands::create_orphan_milestone(&self.app_ctx, self.stack(), &dto)
            else {
                return;
            };
            let mut ids = pace.milestones;
            ids.push(m.id);
            self.set_children(pace.id, PaceRelationshipField::Milestones, ids);
        }

        pub fn remove_milestone(&self, milestone_id: u64) {
            let Some(pace) = self.find_pace() else { return };
            let ids: Vec<EntityId> = pace
                .milestones
                .into_iter()
                .filter(|&id| id != milestone_id)
                .collect();
            self.set_children(pace.id, PaceRelationshipField::Milestones, ids);
            if let Err(e) =
                milestone_commands::remove_milestone(&self.app_ctx, self.stack(), &milestone_id)
            {
                eprintln!("pace: remove milestone failed: {e}");
            }
        }

        // ── internals ──

        fn stack(&self) -> Option<u64> {
            self.ids.stack_id.get()
        }

        /// The one Pace whose `book_item` is this Book, or `None`. The backend
        /// does not enforce uniqueness, so pick the first defensively.
        fn find_pace(&self) -> Option<PaceDto> {
            let ctx = &self.app_ctx;
            let work_id = self.ids.work_id.get()?;
            let pace_ids =
                work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Paces)
                    .ok()?;
            pace_commands::get_pace_multi(ctx, &pace_ids)
                .ok()?
                .into_iter()
                .flatten()
                .find(|p| p.book_item == Some(self.book_item_id))
        }

        /// The id of this Book's Pace *if one already exists* - never creates.
        fn existing_pace_id(&self) -> Option<u64> {
            self.find_pace().map(|p| p.id)
        }

        /// The Pace for this Book, creating one with sensible defaults if none
        /// exists yet. Returns its id. Only [`set_dates`](Self::set_dates) calls
        /// this - see its doc.
        fn ensure_pace(&self) -> Option<u64> {
            if let Some(p) = self.find_pace() {
                return Some(p.id);
            }
            let ctx = &self.app_ctx;
            let work_id = self.ids.work_id.get()?;
            let now = Utc::now();
            let today = now.date_naive();
            let end = today + Duration::days(90);
            let dto = CreatePaceDto {
                created_at: now,
                updated_at: now,
                book_item: Some(self.book_item_id),
                start_date: to_utc(today),
                end_date: to_utc(end),
                weekday_mask: MON_TO_FRI,
                active: true,
                holidays: Vec::new(),
                milestones: Vec::new(),
            };
            pace_commands::create_pace(ctx, self.stack(), &dto, work_id, -1)
                .ok()
                .map(|p| p.id)
        }

        /// Read a Pace, apply a scalar edit, write it back. Relationship fields
        /// (holidays/milestones/book_item) are untouched - `UpdatePaceDto` has
        /// only the scalars.
        fn update_scalars(&self, pace_id: u64, edit: impl FnOnce(&mut UpdatePaceDto)) {
            let ctx = &self.app_ctx;
            let Some(pace) = pace_commands::get_pace(ctx, &pace_id).ok().flatten() else {
                return;
            };
            let mut dto = UpdatePaceDto::from(pace);
            dto.updated_at = Utc::now();
            edit(&mut dto);
            if let Err(e) = pace_commands::update_pace(ctx, self.stack(), &dto) {
                eprintln!("pace: update scalars failed: {e}");
            }
        }

        /// `set_relationship` replaces the whole ordered list (verified in
        /// `pace_controller::set_relationship`).
        fn set_children(
            &self,
            pace_id: u64,
            field: PaceRelationshipField,
            right_ids: Vec<EntityId>,
        ) {
            let dto = PaceRelationshipDto {
                id: pace_id,
                field,
                right_ids,
            };
            if let Err(e) = pace_commands::set_pace_relationship(&self.app_ctx, self.stack(), &dto)
            {
                eprintln!("pace: set relationship failed: {e}");
            }
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::RefCell;
    use std::rc::Rc;

    use chrono::{Datelike, Duration, NaiveDate, Utc};

    use frontend::AppContext;

    use crate::app_ids::AppIds;

    use super::{DailyCount, HolidayRow, MON_TO_FRI, MilestoneRow, PaceState};

    /// Fabricated Book pace state for the `mocks` build: a Mon–Fri schedule
    /// running from a month ago to two months out, an 80k goal, ~a month of
    /// counted-up daily history with a couple of deliberate gaps and one flat
    /// (zero-word) day to exercise the streak logic, one holiday, two milestones.
    pub struct PaceModel {
        state: RefCell<PaceState>,
        next_id: RefCell<u64>,
    }

    impl PaceModel {
        pub fn new(_app_ctx: Rc<AppContext>, _ids: AppIds, _book_item_id: u64) -> Self {
            let today = Utc::now().date_naive();
            let start = today - Duration::days(30);
            let end = today + Duration::days(60);

            let mut history = Vec::new();
            let mut words = 12_000_i64;
            for i in 0..30 {
                let date = start + Duration::days(i);
                // Weekdays only, minus two mid-run gaps.
                if date.weekday().num_days_from_monday() >= 5 {
                    continue;
                }
                if i == 11 || i == 12 {
                    continue;
                }
                let delta = if i == 7 { 0 } else { 700 + (i % 5) * 130 };
                words += delta;
                history.push(DailyCount { date, words });
            }

            let state = PaceState {
                pace_id: Some(1),
                start: Some(start),
                end: Some(end),
                weekday_mask: MON_TO_FRI,
                active: true,
                goal_words: 80_000,
                holidays: vec![HolidayRow {
                    id: 101,
                    label: "Winter break".to_string(),
                    start: today + Duration::days(20),
                    end: today + Duration::days(27),
                }],
                milestones: vec![
                    MilestoneRow {
                        id: 201,
                        label: "Part I complete".to_string(),
                        target_date: today + Duration::days(15),
                        target_words: Some(40_000),
                    },
                    MilestoneRow {
                        id: 202,
                        label: "First draft".to_string(),
                        target_date: today + Duration::days(55),
                        target_words: Some(80_000),
                    },
                ],
                history,
            };
            Self {
                state: RefCell::new(state),
                next_id: RefCell::new(300),
            }
        }

        pub fn load(&self) -> PaceState {
            self.state.borrow().clone()
        }

        // `set_dates` creates the Pace (assigns `pace_id`); the others only
        // refine an existing one - the same contract as the real seam.
        pub fn set_dates(&self, start: NaiveDate, end: NaiveDate) {
            let mut s = self.state.borrow_mut();
            s.pace_id.get_or_insert(1);
            s.start = Some(start);
            s.end = Some(end);
        }

        pub fn set_weekday_mask(&self, mask: i64) {
            let mut s = self.state.borrow_mut();
            if s.pace_id.is_some() {
                s.weekday_mask = mask;
            }
        }

        pub fn set_active(&self, active: bool) {
            let mut s = self.state.borrow_mut();
            if s.pace_id.is_some() {
                s.active = active;
            }
        }

        pub fn set_goal_words(&self, goal: i64) {
            self.state.borrow_mut().goal_words = goal;
        }

        pub fn add_holiday(&self, label: String, start: NaiveDate, end: Option<NaiveDate>) {
            if self.state.borrow().pace_id.is_none() {
                return;
            }
            let id = self.bump();
            self.state.borrow_mut().holidays.push(HolidayRow {
                id,
                label,
                start,
                end: end.unwrap_or(start),
            });
        }

        pub fn remove_holiday(&self, holiday_id: u64) {
            self.state
                .borrow_mut()
                .holidays
                .retain(|h| h.id != holiday_id);
        }

        pub fn add_milestone(
            &self,
            label: String,
            target_date: NaiveDate,
            target_words: Option<i64>,
        ) {
            if self.state.borrow().pace_id.is_none() {
                return;
            }
            let id = self.bump();
            self.state.borrow_mut().milestones.push(MilestoneRow {
                id,
                label,
                target_date,
                target_words,
            });
        }

        pub fn remove_milestone(&self, milestone_id: u64) {
            self.state
                .borrow_mut()
                .milestones
                .retain(|m| m.id != milestone_id);
        }

        fn bump(&self) -> u64 {
            let mut n = self.next_id.borrow_mut();
            *n += 1;
            *n
        }
    }
}

pub use imp::PaceModel;
