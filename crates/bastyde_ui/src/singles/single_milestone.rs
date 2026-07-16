// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SingleMilestone` — the Inspector's read+write probe for the one `Milestone`
//! that pins a target date on a given Part/Chapter inside a Book's writing `Pace`.
//!
//! A milestone is keyed here by its **`target_item`** (the Part/Chapter it marks): the
//! Inspector points the probe at `(enclosing Book, focused Part/Chapter)` and the probe
//! exposes that item's milestone date reactively ([`date_signal`](imp::SingleMilestone::date_signal)).
//! [`set_date`](imp::SingleMilestone::set_date) upserts the milestone under the Book's
//! Pace (`Some` — creating the Pace lazily if the writer hasn't planned one yet) or
//! deletes it (`None`).
//!
//! Backend shape (see `models::pace_model`): a `Milestone` is reachable only through
//! `Pace.milestones` (strong, ordered) and carries a weak `target_item` link, so there is
//! no by-item query — the probe fetches the Book's Pace, then its milestones, and matches
//! on `target_item`. Creating attaches under the Pace via `create_milestone(pace_id, -1)`;
//! deleting detaches from `Pace.milestones` **then** removes, in that order (a bare remove
//! would leave a dangling id in the relationship).
//!
//! Two `mod imp` variants share one public surface. See [`crate::singles`].

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;
    use chrono::{DateTime, Duration, NaiveDate, Utc};

    use frontend::AppContext;
    use frontend::commands::{
        binder_item_commands, milestone_commands, pace_commands, undo_redo_commands, work_commands,
    };
    use frontend::common::direct_access::pace::PaceRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
    use frontend::common::types::EntityId;
    use frontend::direct_access::{
        CreateMilestoneDto, CreatePaceDto, MilestoneDto, PaceDto, PaceRelationshipDto,
    };

    use crate::app_ids::AppIds;

    /// Mon–Fri, the default counted-weekday mask for a freshly created Pace (matches
    /// `models::pace_model::MON_TO_FRI` — the value a Pace born from setting a milestone
    /// gets, identical to one born from setting the deadline).
    const MON_TO_FRI: i64 = 0b0001_1111;

    /// UTC midnight for a calendar day — the on-disk convention shared with the Pace
    /// editor and `record_progress_snapshot`, so dates round-trip through one instant.
    fn to_utc(d: NaiveDate) -> DateTime<Utc> {
        d.and_hms_opt(0, 0, 0).expect("midnight is valid").and_utc()
    }

    struct Inner {
        ctx: Rc<AppContext>,
        ids: AppIds,
        book_item_id: Cell<Option<u64>>,
        target_item_id: Cell<Option<u64>>,
        date: Signal<Option<NaiveDate>>,
    }

    #[derive(Clone)]
    pub struct SingleMilestone {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // public reactive surface; wired to consumers incrementally
    impl SingleMilestone {
        pub fn new(ctx: Rc<AppContext>, ids: AppIds) -> Self {
            Self {
                inner: Rc::new(Inner {
                    ctx,
                    ids,
                    book_item_id: Cell::new(None),
                    target_item_id: Cell::new(None),
                    date: Signal::new(None),
                }),
            }
        }

        /// Point the probe at `(enclosing Book, target Part/Chapter)` and read the
        /// current milestone date synchronously (so the cached signal is current the
        /// moment this returns — usable as a one-shot probe *and* a bound handle).
        pub fn set_book_and_item(&self, book_item: Option<u64>, target_item: Option<u64>) {
            self.inner.book_item_id.set(book_item);
            self.inner.target_item_id.set(target_item);
            self.refresh();
        }

        pub fn book_item(&self) -> Option<u64> {
            self.inner.book_item_id.get()
        }
        pub fn target_item(&self) -> Option<u64> {
            self.inner.target_item_id.get()
        }

        /// The reactive milestone date for the target item (`None` = no milestone set).
        pub fn date_signal(&self) -> Signal<Option<NaiveDate>> {
            self.inner.date.clone()
        }
        pub fn date(&self) -> Option<NaiveDate> {
            self.inner.date.get()
        }

        /// Auto-refresh when a `Milestone` is created/updated/removed (undo-redo, or the
        /// Pace view's own edits) or the owning `Pace`'s relationship changes. Call every
        /// build — the subscription is scoped to the current build, exactly like
        /// [`SingleBinderItem::wire`](crate::singles::SingleBinderItem).
        pub fn wire(&self, ctx: &mut BuildContext) {
            for origin in [
                Origin::DirectAccess(DirectAccessEntity::Milestone(EntityEvent::Created)),
                Origin::DirectAccess(DirectAccessEntity::Milestone(EntityEvent::Updated)),
                Origin::DirectAccess(DirectAccessEntity::Milestone(EntityEvent::Removed)),
                Origin::DirectAccess(DirectAccessEntity::Pace(EntityEvent::Updated)),
            ] {
                let s = self.clone();
                ctx.subscribe_event(origin, move |_event: &Event| s.refresh());
            }
        }

        fn refresh(&self) {
            let date = self
                .current_milestone()
                .map(|m| m.target_date.date_naive());
            if self.inner.date.get() != date {
                self.inner.date.set(date);
            }
        }

        /// Set (`Some`) or clear (`None`) the milestone date for the target item.
        pub fn set_date(&self, date: Option<NaiveDate>) {
            match date {
                Some(d) => self.upsert(d),
                None => self.delete(),
            }
            self.refresh();
        }

        // ── internals ──

        fn stack(&self) -> Option<u64> {
            self.inner.ids.stack_id.get()
        }

        /// The Book's Pace, if one exists (mirrors `PaceModel::find_pace` — the backend
        /// enforces no uniqueness, so pick the first defensively).
        fn find_pace(&self) -> Option<PaceDto> {
            let ctx = &self.inner.ctx;
            let book = self.inner.book_item_id.get()?;
            let work_id = self.inner.ids.work_id.get()?;
            let pace_ids =
                work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Paces)
                    .ok()?;
            pace_commands::get_pace_multi(ctx, &pace_ids)
                .ok()?
                .into_iter()
                .flatten()
                .find(|p| p.book_item == Some(book))
        }

        /// The Book's Pace, creating one with sensible defaults if none exists yet
        /// (mirrors `PaceModel::ensure_pace`). Setting a milestone is, like setting the
        /// deadline, a deliberate "plan this Book" act — so it may birth the Pace.
        fn ensure_pace(&self) -> Option<u64> {
            if let Some(p) = self.find_pace() {
                return Some(p.id);
            }
            let ctx = &self.inner.ctx;
            let book = self.inner.book_item_id.get()?;
            let work_id = self.inner.ids.work_id.get()?;
            let now = Utc::now();
            let today = now.date_naive();
            let dto = CreatePaceDto {
                created_at: now,
                updated_at: now,
                book_item: Some(book),
                start_date: to_utc(today),
                end_date: to_utc(today + Duration::days(90)),
                weekday_mask: MON_TO_FRI,
                active: true,
                holidays: Vec::new(),
                milestones: Vec::new(),
            };
            pace_commands::create_pace(ctx, self.stack(), &dto, work_id, -1)
                .ok()
                .map(|p| p.id)
        }

        /// This item's milestone under the Book's Pace, if any (matched on `target_item`).
        fn current_milestone(&self) -> Option<MilestoneDto> {
            let target = self.inner.target_item_id.get()?;
            let pace = self.find_pace()?;
            milestone_commands::get_milestone_multi(&self.inner.ctx, &pace.milestones)
                .ok()?
                .into_iter()
                .flatten()
                .find(|m| m.target_item == Some(target))
        }

        /// The target item's current title + word goal — the label and word target a
        /// fresh (or re-saved) milestone carries, kept in step with the item.
        fn item_label_and_goal(&self) -> (String, Option<i64>) {
            let Some(target) = self.inner.target_item_id.get() else {
                return (String::new(), None);
            };
            match binder_item_commands::get_binder_item(&self.inner.ctx, &target) {
                Ok(Some(it)) => {
                    let goal = (it.word_count_goal > 0).then_some(it.word_count_goal);
                    (it.title, goal)
                }
                _ => (String::new(), None),
            }
        }

        fn upsert(&self, date: NaiveDate) {
            let Some(target) = self.inner.target_item_id.get() else {
                return;
            };
            let (label, target_word_count) = self.item_label_and_goal();
            let now = Utc::now();
            if let Some(mut existing) = self.current_milestone() {
                // Update through the *relationship*-preserving path: `UpdateMilestoneDto`
                // drops `target_item`, so a scalar update would strand the milestone from
                // its item. `update_..._with_relationships` carries the full DTO.
                existing.updated_at = now;
                existing.label = label;
                existing.target_date = to_utc(date);
                existing.target_word_count = target_word_count;
                let _ = milestone_commands::update_milestone_with_relationships(
                    &self.inner.ctx,
                    self.stack(),
                    &existing,
                );
                return;
            }
            // Creating the first milestone may also lazily birth the Book's Pace — group
            // both writes into one composite so a single Ctrl+Z reverts the whole "plan
            // this section" action (no stray default Pace stranded). `end_composite` drops
            // an empty group, so a bailed `ensure_pace` (no open work) is harmless. This
            // is the composite pattern `OutlineViewModel` uses for its multi-step edits.
            let ctx = &self.inner.ctx;
            let _ = undo_redo_commands::begin_composite(ctx, self.stack());
            if let Some(pace_id) = self.ensure_pace() {
                let dto = CreateMilestoneDto {
                    created_at: now,
                    updated_at: now,
                    label,
                    target_item: Some(target),
                    target_date: to_utc(date),
                    target_word_count,
                };
                // `create` with an owner attaches under `Pace.milestones` (the entity's one
                // strong parent) — same mechanism as `create_pace(work_id)`.
                let _ = milestone_commands::create_milestone(ctx, self.stack(), &dto, pace_id, -1);
            }
            undo_redo_commands::end_composite(ctx);
        }

        fn delete(&self) {
            let Some(existing) = self.current_milestone() else {
                return;
            };
            let Some(pace) = self.find_pace() else { return };
            let ctx = &self.inner.ctx;
            // Detach + remove are two commands; group them so one Ctrl+Z restores the
            // milestone AND its slot in `Pace.milestones` together (a split undo would
            // resurrect an orphan the UI can't see). Both early returns are above
            // `begin_composite`, so the group is never left open.
            let _ = undo_redo_commands::begin_composite(ctx, self.stack());
            // Detach from `Pace.milestones` (REPLACE semantics) *then* remove the orphan —
            // reversing the order leaves a dangling id in the relationship.
            let ids: Vec<EntityId> = pace
                .milestones
                .into_iter()
                .filter(|&id| id != existing.id)
                .collect();
            let dto = PaceRelationshipDto {
                id: pace.id,
                field: PaceRelationshipField::Milestones,
                right_ids: ids,
            };
            let _ = pace_commands::set_pace_relationship(ctx, self.stack(), &dto);
            let _ = milestone_commands::remove_milestone(ctx, self.stack(), &existing.id);
            undo_redo_commands::end_composite(ctx);
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::rc::Rc;

    use bastyde::prelude::*;
    use chrono::NaiveDate;

    use frontend::AppContext;

    use crate::app_ids::AppIds;

    /// Fabricated milestone dates keyed by target item, shared across every clone so a
    /// set in the Inspector sticks for the session. Seeded empty — the mock Pace fixture
    /// (`models::pace_model`) already carries its own display milestones; this is the
    /// editable per-item layer the Inspector writes.
    struct Inner {
        book_item_id: Cell<Option<u64>>,
        target_item_id: Cell<Option<u64>>,
        date: Signal<Option<NaiveDate>>,
        store: RefCell<HashMap<u64, NaiveDate>>,
    }

    #[derive(Clone)]
    pub struct SingleMilestone {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // identical surface to the real variant
    impl SingleMilestone {
        pub fn new(_ctx: Rc<AppContext>, _ids: AppIds) -> Self {
            Self {
                inner: Rc::new(Inner {
                    book_item_id: Cell::new(None),
                    target_item_id: Cell::new(None),
                    date: Signal::new(None),
                    store: RefCell::new(HashMap::new()),
                }),
            }
        }

        pub fn set_book_and_item(&self, book_item: Option<u64>, target_item: Option<u64>) {
            self.inner.book_item_id.set(book_item);
            self.inner.target_item_id.set(target_item);
            self.refresh();
        }

        pub fn book_item(&self) -> Option<u64> {
            self.inner.book_item_id.get()
        }
        pub fn target_item(&self) -> Option<u64> {
            self.inner.target_item_id.get()
        }

        pub fn date_signal(&self) -> Signal<Option<NaiveDate>> {
            self.inner.date.clone()
        }
        pub fn date(&self) -> Option<NaiveDate> {
            self.inner.date.get()
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}

        fn refresh(&self) {
            let date = self
                .inner
                .target_item_id
                .get()
                .and_then(|t| self.inner.store.borrow().get(&t).copied());
            if self.inner.date.get() != date {
                self.inner.date.set(date);
            }
        }

        pub fn set_date(&self, date: Option<NaiveDate>) {
            if let Some(target) = self.inner.target_item_id.get() {
                match date {
                    Some(d) => {
                        self.inner.store.borrow_mut().insert(target, d);
                    }
                    None => {
                        self.inner.store.borrow_mut().remove(&target);
                    }
                }
            }
            self.refresh();
        }
    }
}

pub use imp::SingleMilestone;
