// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The open Work's status ladder, as a reactive Layer A handle.
//!
//! One file, two `#[cfg]`-gated `mod imp` of the same-named type with identical signatures,
//! and an un-gated `pub use` — the architecture every model here follows, so no consumer
//! ever carries a `#[cfg(feature = "mocks")]` of its own.
//!
//! The mocks half matters more than usual for this one. `StatusesViewModel` reads the ladder
//! through here, so without a fabricated half the whole feature would render as "no status"
//! on every row of the mocks build — the build whose entire purpose is doing UI work on
//! surfaces like this one without a backend. It is also **mutable**, unlike most fixtures
//! here, because the ladder editor is a page whose entire content is the list it edits.
//!
//! ## Order is the data
//!
//! `Work.statuses` is an `ordered_one_to_many`, and that order *is* the ladder: it is what
//! makes "which of these two rungs is less finished" answerable, which merge's demote rule,
//! the Overview's sort and the completion readout's notion of "finished" all need. So the
//! rows are held and returned in **relationship order** and never sorted by name — the one
//! place this model deliberately parts company with `WorkTagsListModel`, which sorts.
//!
//! ## Why it caches, unlike the view-model above it
//!
//! The ladder is held in a `ListModel` refreshed on events rather than re-read per call.
//! Two reasons, and the second is the load-bearing one:
//!
//! * A `ListView` needs a model to bind, and the ladder editor is a list.
//! * **Every status picker resolves the whole ladder while it builds** — the Overview draws
//!   one per visible row, and so do the corkboard, the stream and the inspector. Reading
//!   through meant two store round-trips per cell per rebuild, each taking the store's lock,
//!   for a list of four rows that changes about as often as a writer renames a tag.

use std::rc::Rc;

use common::entities::StatusCategory;

/// One rung, flattened for the UI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusRow {
    pub id: u64,
    pub uid: uuid::Uuid,
    pub name: String,
    pub category: StatusCategory,
    pub details: String,
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use super::*;

    use frontend::AppContext;
    use frontend::commands::{binder_status_commands, work_commands};
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::event::{
        DirectAccessEntity, EntityEvent, Event, Origin, WorkManagementEvent,
    };
    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use crate::app_ids::AppIds;

    struct Inner {
        model: ListModel<StatusRow>,
        version: Signal<u64>,
        ctx: Rc<AppContext>,
        /// The open Work's own ids — scopes every read to *this* Work's `statuses`
        /// relationship. A second simultaneously-open Work's ladder must never merge into
        /// this one's, which is exactly what `get_all_binder_status` would do.
        ids: AppIds,
    }

    #[derive(Clone)]
    pub struct WorkStatusesListModel {
        inner: Rc<Inner>,
    }

    impl std::fmt::Debug for WorkStatusesListModel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("WorkStatusesListModel").finish()
        }
    }

    impl WorkStatusesListModel {
        pub fn new(ctx: Rc<AppContext>, ids: AppIds) -> Self {
            let rows = load_rows(&ctx, ids.work_id.get());
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::from_vec(rows),
                    version: Signal::new(0),
                    ctx,
                    ids,
                }),
            }
        }

        /// Subscribe to everything that can change the ladder.
        ///
        /// `Work(Updated)` is in the list beside the three `BinderStatus` ones because
        /// *reordering* the ladder rewrites the relationship on the Work and touches no
        /// rung at all — without it a move would leave every reader showing the old order
        /// until something else happened to refresh it.
        ///
        /// **Re-subscribes on every call**, like `WorkTagsListModel::wire` and for the same
        /// reason: `BuildContext::subscribe_event` is scoped to the current build and
        /// dropped on the next, so a one-shot guard leaves the model permanently deaf while
        /// still reporting itself wired.
        ///
        /// The `LoadWork`/`NewWork` arms fall back to the **event's** work id. This model is
        /// constructed with `WorkSession` and wired from `App::build`, which historically
        /// runs before ProjectLifecycle's seed writes `work_id` — so a bare refresh would
        /// read `None` and leave the ladder empty for the rest of the session, with every
        /// picker on screen quietly claiming the project has no statuses.
        pub fn wire(&self, ctx: &mut BuildContext) {
            for ev in [
                EntityEvent::Created,
                EntityEvent::Updated,
                EntityEvent::Removed,
            ] {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::BinderStatus(ev)),
                    move |_: &Event| me.refresh(),
                );
            }
            {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::Work(EntityEvent::Updated)),
                    move |_: &Event| me.refresh(),
                );
            }
            for wev in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
                let me = self.clone();
                ctx.subscribe_event(Origin::WorkManagement(wev), move |event: &Event| {
                    if !me.inner.ids.is_bootstrap_or_own(&event.ids) {
                        return;
                    }
                    let work_id = me
                        .inner
                        .ids
                        .work_id
                        .get()
                        .or_else(|| event.ids.first().copied());
                    me.refresh_for(work_id);
                });
            }
            {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::WorkManagement(WorkManagementEvent::CloseWork),
                    move |event: &Event| {
                        if me.inner.ids.is_event_for_my_work(&event.ids) {
                            me.refresh_for(None)
                        }
                    },
                );
            }
            // Catch up when this window is already seeded (rebuild / late wire).
            self.refresh_for(self.inner.ids.work_id.get());
        }

        /// Re-read the ladder for the currently seeded Work (`None` → empty).
        pub fn refresh(&self) {
            self.refresh_for(self.inner.ids.work_id.get());
        }

        fn refresh_for(&self, work_id: Option<u64>) {
            let rows = load_rows(&self.inner.ctx, work_id);
            // **The version bump is conditional, and that is load-bearing.** It means "the
            // ladder changed", never "somebody re-read it" — and the difference is the
            // difference between a repaint and a hang.
            //
            // `wire` ends with a catch-up `refresh_for`, and every view that follows the
            // ladder calls `wire(ctx)` from its own `build()` and then binds this version at
            // `BindingLevel::Rebuild`. A rebuild is unregister-then-re-register, and the
            // binding group deliberately keeps its `last_seen` ledger across that (see
            // `WidgetTree::process_pending_rebuilds`) so a write made by `build()` before it
            // re-binds is not swallowed. An unconditional bump is therefore exactly such a
            // write: the widget's own build dirties the widget, which rebuilds, which builds,
            // which bumps — a self-sustaining rebuild loop, at frame rate, forever. It cost
            // 205 rebuilds of the Overview's filter row in six seconds of an idle window,
            // pinning a core and putting ~120 ms of latency under every interaction, and it
            // was invisible until the first *external* refresh started it.
            //
            // Comparing first also spares every other ladder view a rebuild it has no reason
            // to do: a `Work(Updated)` for an unrelated field re-reads the same five rungs.
            let changed = self.items() != rows;
            // Keyed, so a rename touches one row rather than replacing the list — which is
            // what lets the editor's inline fields keep focus while a sibling row commits.
            self.inner.model.reconcile_by_key(rows, |r| r.id);
            if changed {
                let v = &self.inner.version;
                v.set(v.get().wrapping_add(1));
            }
        }

        /// The reactive model a `ListView` binds to.
        pub fn list_model(&self) -> ListModel<StatusRow> {
            self.inner.model.clone()
        }

        /// Bumped on every refresh, so a view can rebind at `Rebuild` level.
        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        /// The ladder, **in ladder order**.
        pub fn items(&self) -> Vec<StatusRow> {
            (0..self.inner.model.len())
                .filter_map(|i| self.inner.model.with_item(i, |r| r.clone()))
                .collect()
        }

        /// Append a rung to the end of the ladder. `None` when no project is open or the
        /// write failed; the backend's `Created` event drives every reader's refresh.
        ///
        /// The `uid` is minted here rather than left nil. `BinderStatus` has no
        /// `with_identity` helper of its own — only `Binder` and `BinderItem` carry one —
        /// so nothing downstream would ever fill it in, and a rung created nil-identified
        /// survives the session but not the save/reload that re-mints every `EntityId`.
        pub fn create(
            &self,
            name: &str,
            category: StatusCategory,
            details: &str,
            stack_id: Option<u64>,
        ) -> Option<u64> {
            let work_id = self.inner.ids.work_id.get()?;
            let now = chrono::Utc::now();
            let dto = frontend::direct_access::CreateBinderStatusDto {
                uid: common::uid::new_uid(),
                created_at: now,
                updated_at: now,
                name: name.trim().to_string(),
                category,
                details: details.to_string(),
            };
            // `-1` appends. The ladder's order is the writer's, and a new rung joining at
            // the top would silently claim to be the least finished thing in the project.
            match binder_status_commands::create_binder_status(
                &self.inner.ctx,
                stack_id,
                &dto,
                work_id,
                -1,
            ) {
                Ok(created) => {
                    // Refresh here rather than waiting for the `Created` event to come back
                    // round through the hub. The event still arrives and refreshes again
                    // (harmlessly, `reconcile_by_key` is idempotent), but the caller can
                    // read the ladder it just changed *now* — which `nudge` and the add
                    // field's duplicate check both do, in the same tick.
                    self.refresh();
                    Some(created.id)
                }
                Err(e) => {
                    eprintln!("statuses: create failed: {e}");
                    None
                }
            }
        }

        /// Patch one rung. Every scalar is sent, so a caller passes the row's current
        /// values for whatever it is not changing.
        ///
        /// Read-modify-write off the live entity, for the reason `WorkTagsListModel::update`
        /// records: [`StatusRow`] carries no `created_at`, and inventing one here would
        /// reset the rung's creation time on every rename.
        pub fn update(
            &self,
            id: u64,
            name: &str,
            category: StatusCategory,
            details: &str,
            stack_id: Option<u64>,
        ) {
            let existing = match binder_status_commands::get_binder_status(&self.inner.ctx, &id) {
                Ok(Some(s)) => s,
                Ok(None) => return,
                Err(e) => {
                    eprintln!("statuses: update failed to read {id}: {e}");
                    return;
                }
            };
            let dto = frontend::direct_access::UpdateBinderStatusDto {
                // Carried through unchanged: a nil here would overwrite the durable
                // identity every saved bundle keys this rung by.
                uid: existing.uid,
                id,
                created_at: existing.created_at,
                updated_at: chrono::Utc::now(),
                name: name.trim().to_string(),
                category,
                details: details.to_string(),
            };
            match binder_status_commands::update_binder_status(&self.inner.ctx, stack_id, &dto) {
                Ok(_) => self.refresh(),
                Err(e) => eprintln!("statuses: update failed: {e}"),
            }
        }

        /// Delete a rung.
        ///
        /// Every item wearing it is left alone: the reference is **weak**, so those rows
        /// simply read as unset afterwards rather than dangling. That is the whole reason
        /// deleting a rung in use can be offered at all — see
        /// `StatusesViewModel::usage_counts` for the number the pane warns with first.
        pub fn remove(&self, id: u64, stack_id: Option<u64>) {
            match binder_status_commands::remove_binder_status(&self.inner.ctx, stack_id, &id) {
                Ok(()) => self.refresh(),
                Err(e) => eprintln!("statuses: removing rung {id} failed: {e}"),
            }
        }

        /// Move a rung to `new_index` within the ladder.
        ///
        /// Rewrites the relationship on the **`Work`**, not the rung — which is why
        /// [`wire`](Self::wire) listens for `Work(Updated)`. Nothing about the rung itself
        /// changes, so no `BinderStatus` event is published at all.
        pub fn move_to(&self, id: u64, new_index: i32, stack_id: Option<u64>) {
            let Some(work_id) = self.inner.ids.work_id.get() else {
                return;
            };
            match work_commands::move_work_relationship(
                &self.inner.ctx,
                stack_id,
                &work_id,
                &WorkRelationshipField::Statuses,
                &[id],
                new_index,
            ) {
                Ok(_) => self.refresh(),
                Err(e) => eprintln!("statuses: moving rung {id} to {new_index} failed: {e}"),
            }
        }
    }

    /// Read one Work's ladder (via `Work.statuses`), in relationship order — **not**
    /// `get_all_binder_status`, which returns every rung in the whole shared store: with a
    /// second Work simultaneously open that would leak one project's ladder into the
    /// other's pickers.
    fn load_rows(ctx: &AppContext, work_id: Option<u64>) -> Vec<StatusRow> {
        let Some(work_id) = work_id else {
            return Vec::new(); // no project open
        };
        let ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Statuses)
                .unwrap_or_default();
        if ids.is_empty() {
            return Vec::new();
        }
        binder_status_commands::get_binder_status_multi(ctx, &ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .map(|d| StatusRow {
                id: d.id,
                uid: d.uid,
                name: d.name,
                category: d.category,
                details: d.details,
            })
            .collect()
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use super::*;

    use std::cell::RefCell;

    use frontend::AppContext;
    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use crate::app_ids::AppIds;

    /// The fabricated ladder. Ids are the ones the other mock fixtures point at — the
    /// corkboard cards and the Overview rows both name 901 and 903 — so the mocks build
    /// renders a set rung, a *different* set rung and the unset case side by side, which is
    /// the whole reason to look at it.
    ///
    /// Four rungs spanning four different categories, so all four glyphs and both accent
    /// roles are on screen at once.
    fn mock_ladder() -> Vec<StatusRow> {
        let row = |id: u64, name: &str, category: StatusCategory| StatusRow {
            id,
            uid: common::uid::fixture_uid(id),
            name: name.to_string(),
            category,
            details: String::new(),
        };
        vec![
            row(900, "To do", StatusCategory::Planned),
            row(901, "Draft", StatusCategory::Drafting),
            row(902, "Needs work", StatusCategory::NeedsWork),
            row(903, "Final", StatusCategory::Final),
        ]
    }

    /// **Mutable**, unlike most fabricated fixtures here, because the ladder editor is
    /// exactly the kind of surface the mocks build exists to let someone work on: a page
    /// whose whole content is the list it edits is untestable by eye against a fixture that
    /// refuses every edit. Writes land in the model and bump `version`, so the pane reacts
    /// as it does against the real store — it simply forgets on restart.
    #[derive(Clone)]
    pub struct WorkStatusesListModel {
        model: ListModel<StatusRow>,
        version: Signal<u64>,
        /// Mints ids for rungs added during the session, above the fixture's own block so
        /// a new rung can never collide with one an Overview row already points at.
        next_id: Rc<RefCell<u64>>,
    }

    impl std::fmt::Debug for WorkStatusesListModel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("WorkStatusesListModel").finish()
        }
    }

    impl WorkStatusesListModel {
        pub fn new(_ctx: Rc<AppContext>, _ids: AppIds) -> Self {
            Self {
                model: ListModel::from_vec(mock_ladder()),
                version: Signal::new(0),
                next_id: Rc::new(RefCell::new(950)),
            }
        }

        /// Nothing to subscribe to: there is no store to publish events, and every write
        /// below bumps `version` itself.
        pub fn wire(&self, _ctx: &mut BuildContext) {}

        /// Nothing to re-read: the fixture is the truth here.
        pub fn refresh(&self) {}

        pub fn version_signal(&self) -> Signal<u64> {
            self.version.clone()
        }

        pub fn list_model(&self) -> ListModel<StatusRow> {
            self.model.clone()
        }

        pub fn items(&self) -> Vec<StatusRow> {
            (0..self.model.len())
                .filter_map(|i| self.model.with_item(i, |r| r.clone()))
                .collect()
        }

        fn commit(&self, rows: Vec<StatusRow>) {
            self.model.reconcile_by_key(rows, |r| r.id);
            let n = self.version.get();
            self.version.set(n.wrapping_add(1));
        }

        pub fn create(
            &self,
            name: &str,
            category: StatusCategory,
            details: &str,
            _stack_id: Option<u64>,
        ) -> Option<u64> {
            let id = {
                let mut n = self.next_id.borrow_mut();
                *n += 1;
                *n
            };
            let mut rows = self.items();
            rows.push(StatusRow {
                id,
                uid: common::uid::fixture_uid(id),
                name: name.trim().to_string(),
                category,
                details: details.to_string(),
            });
            self.commit(rows);
            Some(id)
        }

        pub fn update(
            &self,
            id: u64,
            name: &str,
            category: StatusCategory,
            details: &str,
            _stack_id: Option<u64>,
        ) {
            let mut rows = self.items();
            if let Some(r) = rows.iter_mut().find(|r| r.id == id) {
                r.name = name.trim().to_string();
                r.category = category;
                r.details = details.to_string();
            }
            self.commit(rows);
        }

        pub fn remove(&self, id: u64, _stack_id: Option<u64>) {
            let mut rows = self.items();
            rows.retain(|r| r.id != id);
            self.commit(rows);
        }

        pub fn move_to(&self, id: u64, new_index: i32, _stack_id: Option<u64>) {
            let mut rows = self.items();
            let Some(from) = rows.iter().position(|r| r.id == id) else {
                return;
            };
            let to = new_index.clamp(0, rows.len().saturating_sub(1) as i32) as usize;
            let row = rows.remove(from);
            rows.insert(to, row);
            self.commit(rows);
        }
    }
}

pub use imp::WorkStatusesListModel;

/// The version signal's contract: it moves when the *ladder* moves, and at no other time.
///
/// Only the real half can be wrong about this — the mock half never re-reads anything, so
/// its `commit` is already change-only by construction.
#[cfg(all(test, not(feature = "mocks")))]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;
    use common::entities::StatusCategory;
    use frontend::AppContext;
    use frontend::commands::{smart_punctuation_commands, work_commands};
    use frontend::common::entities::QuoteStyle;
    use frontend::direct_access::{CreateSmartPunctuationDto, CreateWorkDto};

    fn now() -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }

    /// A bare Work, seeded onto a fresh `AppIds` exactly as a window's session seeds it.
    fn model_on_a_work() -> (WorkStatusesListModel, AppIds) {
        let ctx = Rc::new(AppContext::new());
        // A Work owns exactly one SmartPunctuation (one_to_one, strong), so it has to
        // exist before the Work that points at it.
        let smart_punctuation = smart_punctuation_commands::create_orphan_smart_punctuation(
            &ctx,
            None,
            &CreateSmartPunctuationDto {
                created_at: now(),
                updated_at: now(),
                override_app_default: false,
                dashes: false,
                ellipsis: false,
                quotes: false,
                quote_style: QuoteStyle::LocaleDefault,
                pre_punctuation_spacing: false,
                dialogue_marker: false,
            },
        )
        .expect("create smart_punctuation")
        .id;
        let work = work_commands::create_orphan_work(
            &ctx,
            None,
            &CreateWorkDto {
                statuses: Vec::new(),
                created_at: now(),
                updated_at: now(),
                title: "W".into(),
                smart_punctuation,
                ..Default::default()
            },
        )
        .expect("create work")
        .id;
        let ids = AppIds::new();
        ids.work_id.set(Some(work));
        (WorkStatusesListModel::new(ctx, ids.clone()), ids)
    }

    /// **The hang, as a unit test.** A re-read that finds the ladder unchanged must not
    /// move the version.
    ///
    /// Every ladder view calls `wire(ctx)` from its `build()` — which ends in a catch-up
    /// `refresh_for` — and then binds this version at `BindingLevel::Rebuild`. A binding
    /// group keeps its `last_seen` ledger across a rebuild on purpose, so a bump here is a
    /// write the widget's own build made: it dirties the widget, which rebuilds, which
    /// builds, which bumps. Measured before the fix: 205 rebuilds of the Overview's status
    /// filter row in six seconds of an *idle* window.
    #[test]
    fn re_reading_an_unchanged_ladder_does_not_move_the_version() {
        let (model, _ids) = model_on_a_work();
        model.create("Draft", StatusCategory::Drafting, "", None);
        let settled = model.version_signal().get();

        for _ in 0..20 {
            model.refresh();
        }

        assert_eq!(
            model.version_signal().get(),
            settled,
            "a re-read that changed nothing bumped the version — every view bound at \
             Rebuild level now rebuilds itself forever"
        );
    }

    /// And the half that must still work: a real edit does move it, or nothing would
    /// repaint at all.
    #[test]
    fn a_real_ladder_change_moves_the_version() {
        let (model, _ids) = model_on_a_work();
        let before = model.version_signal().get();
        let id = model
            .create("Draft", StatusCategory::Drafting, "", None)
            .expect("create rung");
        let after_create = model.version_signal().get();
        assert!(
            after_create > before,
            "creating a rung must move the version"
        );

        model.update(id, "Revised", StatusCategory::NeedsWork, "", None);
        assert!(
            model.version_signal().get() > after_create,
            "renaming a rung must move the version"
        );
    }
}
