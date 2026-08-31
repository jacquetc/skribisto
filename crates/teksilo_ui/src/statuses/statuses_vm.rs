// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The status feature's business logic: the ladder, and what a row is at on it.
//!
//! Work-scoped (Tier 2 — one per open `Work`, shared by every window on it), like the tag
//! and dictionary models beside it: the ladder belongs to the project, not to a window.
//!
//! ## Why the ladder is read, not cached
//!
//! A project has a handful of rungs and they change about as often as a writer renames a
//! tag, so [`StatusesViewModel::ladder`] reads through the relationship each time rather than
//! holding a mirror that four surfaces would each have to invalidate. The one piece of
//! state here is [`StatusesViewModel::revision`] — bumped when the ladder itself changes — so
//! a view can rebind without any of them knowing how the others found out.

use std::rc::Rc;

use common::entities::StatusCategory;
use frontend::AppContext;
use frontend::commands::{binder_item_commands, binder_status_commands};
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::direct_access::CreateBinderStatusDto;
use teksilo::prelude::*;

use crate::app_ids::AppIds;
use crate::models::{LadderRow, WorkStatusesListModel};
use crate::statuses::Preset;

/// One rung, as every surface reads it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusRung {
    pub id: u64,
    pub uid: uuid::Uuid,
    pub name: String,
    pub category: StatusCategory,
    pub details: String,
}

impl From<LadderRow> for StatusRung {
    fn from(d: LadderRow) -> Self {
        Self {
            id: d.id,
            uid: d.uid,
            name: d.name,
            category: d.category,
            details: d.details,
        }
    }
}

/// What the writer's ladder is, and what sits on it.
#[derive(Clone)]
pub struct StatusesViewModel {
    inner: Rc<Inner>,
}

struct Inner {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    /// Layer A. Reading through it rather than through `frontend` directly is what gives
    /// the mocks build a fabricated ladder instead of an empty one.
    ladder: WorkStatusesListModel,
}

impl std::fmt::Debug for StatusesViewModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusesViewModel").finish()
    }
}

impl StatusesViewModel {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        Self {
            inner: Rc::new(Inner {
                ladder: WorkStatusesListModel::new(app_ctx.clone(), ids.clone()),
                app_ctx,
                ids,
            }),
        }
    }

    /// Rebind on this to follow ladder edits.
    ///
    /// **Layer A's own signal, not a second one beside it.** It was a separate `Signal`
    /// here until the editor landed, and the two could not agree by construction: every
    /// ladder change arriving as a backend *event* bumps the model's version, while only
    /// [`seed`](Self::seed) bumped this one. So a rung renamed in the Settings pane — or
    /// by an undo, or from a second window on the same project — reached the picker (which
    /// re-reads on build) but never the Overview's filter chip row, which binds *this* and
    /// would have gone on offering the old name until something else forced a rebuild.
    pub fn revision(&self) -> Signal<u64> {
        self.inner.ladder.version_signal()
    }

    /// The ladder, **in ladder order**.
    ///
    /// That order is the relationship's own (`Work.statuses` is an `ordered_one_to_many`),
    /// which is what makes "lower rung" answerable at all — merge demotes to the lower of
    /// two, and a sort orders by it.
    pub fn ladder(&self) -> Vec<StatusRung> {
        self.inner
            .ladder
            .items()
            .into_iter()
            .map(StatusRung::from)
            .collect()
    }

    /// The reactive ladder a `ListView` binds to — Layer A's own model, in ladder order.
    ///
    /// Handed out rather than copied into a fresh one per build: binding the live model is
    /// what lets the editor's inline fields keep focus, since a rename reconciles one row
    /// instead of replacing the list.
    pub fn list_model(&self) -> teksilo::data::ListModel<LadderRow> {
        self.inner.ladder.list_model()
    }

    /// Subscribe to ladder changes. Delegates to Layer A, which owns the event set.
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.inner.ladder.wire(ctx);
    }

    /// One rung by id, or `None` when it no longer resolves.
    ///
    /// Every reader must tolerate that `None`: the reference from an item is **weak**, so a
    /// rung deleted from the vocabulary leaves the rows that wore it intact and statusless
    /// rather than dangling them.
    pub fn rung(&self, id: u64) -> Option<StatusRung> {
        self.ladder().into_iter().find(|r| r.id == id)
    }

    /// Set (or with `None`, clear) an item's rung.
    ///
    /// Goes through `set_binder_item_relationship`, **not** an item update:
    /// `UpdateBinderItemDto` deliberately carries no relationship fields, precisely so a
    /// scalar patch cannot clobber them. That also means this is undoable for free —
    /// `set_relationship` is backed by the undoable use case — and that it marks the
    /// project dirty for free, since it publishes `BinderItem(Updated)`.
    pub fn set_item_status(&self, item_id: u64, status: Option<u64>) {
        let stack = self.inner.ids.stack_id.get();
        let ids = status.map(|id| vec![id]).unwrap_or_default();
        if let Err(e) = binder_item_commands::set_binder_item_relationship(
            &self.inner.app_ctx,
            stack,
            &frontend::direct_access::BinderItemRelationshipDto {
                id: item_id,
                field: BinderItemRelationshipField::Status,
                right_ids: ids,
            },
        ) {
            eprintln!("statuses: setting item {item_id}'s status failed: {e}");
        }
    }

    /// The open project, for a caller that has to name it in a scoped toast.
    pub fn work_id(&self) -> Option<u64> {
        self.inner.ids.work_id.get()
    }

    fn stack(&self) -> Option<u64> {
        self.inner.ids.stack_id.get()
    }

    /// Append a rung. `None` when no project is open or the write failed.
    ///
    /// Appends rather than inserts, always: the ladder's order is a claim about progress,
    /// and a new rung is not automatically the least finished thing in the book. The
    /// writer moves it where they mean it to go.
    pub fn create(&self, name: &str, category: StatusCategory, details: &str) -> Option<u64> {
        self.inner
            .ladder
            .create(name, category, details, self.stack())
    }

    /// Rename a rung, keeping its category and description.
    pub fn rename(&self, id: u64, name: &str) {
        let Some(r) = self.rung(id) else { return };
        self.inner
            .ladder
            .update(id, name, r.category, &r.details, self.stack());
    }

    /// Re-file a rung under a different app-owned category — i.e. change its glyph and
    /// its colour, which are the only two things the writer does *not* author directly.
    pub fn set_category(&self, id: u64, category: StatusCategory) {
        let Some(r) = self.rung(id) else { return };
        self.inner
            .ladder
            .update(id, &r.name, category, &r.details, self.stack());
    }

    /// Set the one-line description the picker shows as a trailing hint.
    pub fn set_details(&self, id: u64, details: &str) {
        let Some(r) = self.rung(id) else { return };
        self.inner
            .ladder
            .update(id, &r.name, r.category, details, self.stack());
    }

    /// Delete a rung. Rows wearing it read as unset afterwards — see
    /// [`usage_counts`](Self::usage_counts) for the number to warn with first.
    pub fn delete(&self, id: u64) {
        self.inner.ladder.remove(id, self.stack());
    }

    /// Move a rung one place towards the start (`-1`) or the end (`+1`) of the ladder.
    ///
    /// A no-op at either end rather than a wrap: the ladder is an order, and a rung
    /// jumping from "most finished" to "least" because the writer clicked once too often
    /// is a destructive answer to a harmless mistake.
    pub fn nudge(&self, id: u64, delta: i32) {
        let ladder = self.ladder();
        let Some(at) = ladder.iter().position(|r| r.id == id) else {
            return;
        };
        let to = at as i32 + delta;
        if to < 0 || to as usize >= ladder.len() {
            return;
        }
        self.inner.ladder.move_to(id, to, self.stack());
    }

    /// A rung already carrying `candidate`'s name, if any — `exclude` skips the row being
    /// edited so renaming a rung to itself is not a collision.
    ///
    /// A **warning**, never a refusal, for the reason the tag palette records: two rungs
    /// legitimately share a name for the moment one of them is being retyped, and the
    /// backend neither knows nor should be made to care.
    pub fn duplicate_name(&self, candidate: &str, exclude: Option<u64>) -> Option<String> {
        let key = crate::shared::list_naming::name_key(candidate);
        if key.is_empty() {
            return None;
        }
        self.ladder()
            .into_iter()
            .find(|r| Some(r.id) != exclude && crate::shared::list_naming::name_key(&r.name) == key)
            .map(|r| r.name)
    }

    /// How many binder items wear each rung, keyed by rung id.
    ///
    /// Counts **every** item, not only the prose-bearing ones
    /// [`completion`](super::completion) measures: this answers "what would I disturb by
    /// deleting this", and a notes folder filed under "Needs work" is as disturbed as a
    /// scene. Trashed rows are counted too, for the same reason — restoring one must not
    /// discover that its rung quietly vanished while it was away.
    ///
    /// One scan, on demand. This is the Settings pane's number and nothing else reads it,
    /// so it is not worth a cache that four surfaces would have to invalidate.
    pub fn usage_counts(&self) -> std::collections::HashMap<u64, usize> {
        let mut counts = std::collections::HashMap::new();
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return counts;
        };
        for item in crate::models::binder_stream::ordered_binder_items(&self.inner.app_ctx, work_id)
        {
            if let Ok(Some(dto)) =
                binder_item_commands::get_binder_item(&self.inner.app_ctx, &item.id)
                && let Some(status) = dto.status
            {
                *counts.entry(status).or_insert(0) += 1;
            }
        }
        counts
    }

    /// Lay down a preset ladder, in order.
    ///
    /// Returns how many rungs were created. Refuses to run over a project that already has
    /// a ladder: seeding is a *creation* step, and re-running it would duplicate every rung
    /// rather than merge — the tag preset can skip colliding names because a tag is
    /// identified by its name, and a rung deliberately is not.
    pub fn seed(&self, preset: Preset) -> usize {
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return 0;
        };
        if !self.ladder().is_empty() {
            return 0;
        }
        let now = chrono::Utc::now();
        let dtos: Vec<CreateBinderStatusDto> = preset
            .rows()
            .into_iter()
            .map(|r| CreateBinderStatusDto {
                // Minted here: a rung outlives the session it was made in, and every
                // `EntityId` is re-minted on the next `load_work`.
                uid: common::uid::new_uid(),
                created_at: now,
                updated_at: now,
                name: r.name.resolve_now(),
                category: r.category,
                details: String::new(),
            })
            .collect();
        let n = dtos.len();
        // One call, so the ladder lands in one order and one event rather than N of each.
        // `index: 0` appends into an empty collection, which the guard above guarantees.
        match binder_status_commands::create_binder_status_multi(
            &self.inner.app_ctx,
            self.inner.ids.stack_id.get(),
            &dtos,
            work_id,
            0,
        ) {
            Ok(created) => {
                // Written straight through the commands (one call, so the ladder lands in
                // one order and one event), so Layer A has not seen it — unlike every other
                // write here, which goes through the model and refreshes itself.
                self.inner.ladder.refresh();
                created.len()
            }
            Err(e) => {
                eprintln!("statuses: seeding the {preset:?} ladder ({n} rungs) failed: {e}");
                0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A real project with a real ladder, read back off the **Work** rather than off this
    /// view-model — under `--features mocks` the list model mutates a fabricated ladder and
    /// never writes, so asking the view-model would answer about the mock.
    #[cfg(not(feature = "mocks"))]
    fn project() -> (Rc<AppContext>, AppIds, StatusesViewModel, u64) {
        use frontend::commands::work_commands;
        use frontend::direct_access::CreateWorkDto;

        let ctx = Rc::new(AppContext::new());
        let work = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default())
            .expect("create work");
        let ids = AppIds::new();
        ids.work_id.set(Some(work.id));
        let vm = StatusesViewModel::new(ctx.clone(), ids.clone());
        (ctx, ids, vm, work.id)
    }

    /// **The ladder is editable, and the edits stick.** This is the half of the two-level
    /// model the writer owns: the name is theirs, the category is the app's.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn a_rung_can_be_added_renamed_recategorised_and_removed() {
        let (_ctx, _ids, vm, _work) = project();
        assert!(vm.ladder().is_empty(), "a fresh project has no ladder");

        let id = vm
            .create("Rough", StatusCategory::Drafting, "still finding it")
            .expect("create");
        assert_eq!(vm.ladder().len(), 1);
        assert_eq!(vm.rung(id).unwrap().name, "Rough");
        assert_eq!(vm.rung(id).unwrap().details, "still finding it");

        vm.rename(id, "Rough draft");
        assert_eq!(vm.rung(id).unwrap().name, "Rough draft");
        // The rename left the other two fields alone — every write here is a full-scalar
        // patch, so a caller forgetting to carry one would silently blank it.
        assert_eq!(vm.rung(id).unwrap().category, StatusCategory::Drafting);
        assert_eq!(vm.rung(id).unwrap().details, "still finding it");

        vm.set_category(id, StatusCategory::NeedsWork);
        assert_eq!(vm.rung(id).unwrap().category, StatusCategory::NeedsWork);
        assert_eq!(vm.rung(id).unwrap().name, "Rough draft");

        vm.set_details(id, "");
        assert_eq!(vm.rung(id).unwrap().details, "", "blank clears it");

        vm.delete(id);
        assert!(vm.ladder().is_empty());
    }

    /// **The ladder's order is the writer's, and it is what "less finished" means.**
    ///
    /// Read back through the relationship, because that vector *is* the order — a test
    /// that asserted on a sorted or id-ordered read would pass while the feature was
    /// broken.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn a_rung_moves_up_and_down_and_stops_at_the_ends() {
        let (_ctx, _ids, vm, _work) = project();
        let a = vm.create("A", StatusCategory::Planned, "").unwrap();
        let b = vm.create("B", StatusCategory::Drafting, "").unwrap();
        let c = vm.create("C", StatusCategory::Final, "").unwrap();
        let names = || vm.ladder().into_iter().map(|r| r.name).collect::<Vec<_>>();
        assert_eq!(names(), ["A", "B", "C"], "created rungs append, in order");

        vm.nudge(c, -1);
        assert_eq!(names(), ["A", "C", "B"]);
        vm.nudge(c, -1);
        assert_eq!(names(), ["C", "A", "B"]);

        // At the top, and it stays there: a wrap would silently turn the most finished
        // rung into the least, which is a destructive answer to one click too many.
        vm.nudge(c, -1);
        assert_eq!(names(), ["C", "A", "B"], "no wrap at the start");

        vm.nudge(b, 1);
        assert_eq!(names(), ["C", "A", "B"], "no wrap at the end");

        vm.nudge(a, 1);
        assert_eq!(names(), ["C", "B", "A"]);
    }

    /// A duplicate name is reported, and a rung never collides with itself — otherwise
    /// every rename would warn about the rung being renamed.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn a_duplicate_name_is_reported_but_a_rung_does_not_collide_with_itself() {
        let (_ctx, _ids, vm, _work) = project();
        let draft = vm.create("Draft", StatusCategory::Drafting, "").unwrap();
        vm.create("Final", StatusCategory::Final, "").unwrap();

        assert_eq!(vm.duplicate_name("draft", None).as_deref(), Some("Draft"));
        assert_eq!(
            vm.duplicate_name("  DRAFT  ", None).as_deref(),
            Some("Draft")
        );
        assert_eq!(vm.duplicate_name("Draft", Some(draft)), None);
        assert_eq!(vm.duplicate_name("Polish", None), None);
        assert_eq!(vm.duplicate_name("   ", None), None, "blank is not a clash");
    }

    /// **Deleting a rung leaves the prose.** The reference from an item is weak on
    /// purpose, so the rows wearing it read as unset rather than dangling — and
    /// `usage_counts` is what lets the pane say how many that will be *before* the click.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn usage_counts_the_items_a_deletion_would_unfile() {
        use frontend::commands::{binder_commands, binder_item_commands, work_commands};
        use frontend::common::direct_access::work::WorkRelationshipField;
        use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto};

        let (ctx, ids, vm, work_id) = project();
        let rung = vm.create("Draft", StatusCategory::Drafting, "").unwrap();
        let other = vm.create("Final", StatusCategory::Final, "").unwrap();

        let binder = binder_commands::create_binder(
            &ctx,
            None,
            &CreateBinderDto {
                uid: common::uid::new_uid(),
                ..Default::default()
            },
            work_id,
            -1,
        )
        .expect("binder");
        let mut items = Vec::new();
        for _ in 0..3 {
            let item = binder_item_commands::create_binder_item(
                &ctx,
                None,
                &CreateBinderItemDto {
                    uid: common::uid::new_uid(),
                    ..Default::default()
                },
                binder.id,
                -1,
            )
            .expect("item");
            items.push(item.id);
        }
        // Two on the rung about to be deleted, one on the other — so a count that ignored
        // which rung it was asked about would read 3 and look plausible.
        vm.set_item_status(items[0], Some(rung));
        vm.set_item_status(items[1], Some(rung));
        vm.set_item_status(items[2], Some(other));

        let counts = vm.usage_counts();
        assert_eq!(counts.get(&rung).copied(), Some(2));
        assert_eq!(counts.get(&other).copied(), Some(1));

        vm.delete(rung);
        // The items survive; they simply have no rung any more.
        for id in &items[..2] {
            let dto = binder_item_commands::get_binder_item(&ctx, id)
                .unwrap()
                .expect("the item outlives the rung it wore");
            assert_eq!(dto.status, None, "a deleted rung reads as unset");
        }
        assert_eq!(
            work_commands::get_work_relationship(&ctx, &work_id, &WorkRelationshipField::Statuses)
                .unwrap()
                .len(),
            1
        );
        let _ = ids;
    }

    /// **`revision()` is Layer A's signal, not a second one beside it.**
    ///
    /// Every ladder change that arrives as a backend event bumps the model's version; only
    /// `seed` used to bump the view-model's own. A surface binding this one — the Overview's
    /// status filter chip row does — therefore went deaf to every rename, reorder and
    /// deletion the editor makes.
    #[test]
    fn the_revision_signal_is_the_one_layer_a_bumps() {
        let ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let vm = StatusesViewModel::new(ctx, ids);
        let a = vm.revision();
        let b = vm.revision();
        let before = a.get();
        b.set(before.wrapping_add(1));
        assert_eq!(
            a.get(),
            before.wrapping_add(1),
            "two reads of `revision()` must be the same signal"
        );
    }
}
