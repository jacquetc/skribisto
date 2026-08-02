// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `TrashViewModel` — the trash dock's business logic: list the trashed roots,
//! restore (in place, with an orphan → destination-picker fallback), restore a
//! single item to a chosen destination, permanently delete an entry, and empty
//! the whole trash.
//!
//! Single-instance live state, created once in `App::build` and shared by
//! `.clone()`. Like `SearchReplaceViewModel` it **shares** the outline's
//! `DockingModel` (the leading rail hosts several tabs — one model owns them all)
//! and holds a distinct `dock_id`; it is not registered as `app_state`.
//!
//! **Destructive ops keep their undo, then commit on a grace timer.** Empty Trash
//! and Delete Forever run the (undoable) backend op, then raise a warning toast
//! with an *Undo* action; if the toast times out (or the user dismisses it) the
//! project's undo history is cleared — the point of no return described in
//! `qleany docs undo`. The backend op itself never clears any stack.

use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
use std::time::Duration;

use bastyde::data::{KeyedSelectionModel, SelectionMode};
use bastyde::i18n::LocalizedString;
use bastyde::prelude::*;
use bastyde::widgets::{
    DockWidgetId, DockingModel, MessageBox, MessageBoxButtons, StandardButton, Toast, ToastAction,
    ToastDismissCause, ToastPriority,
};

use frontend::AppContext;
use frontend::commands::{
    binder_item_commands, trash_info_commands, trash_management_commands, undo_redo_commands,
    work_commands,
};
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::trash_management::{
    DeleteTrashEntriesDto, DropPosition, EmptyTrashDto, RestoreItemsDto, RestoreItemsToDto,
    RestoreItemsToResultDto,
};

use crate::app_ids::AppIds;
use crate::models::{TrashRootKind, TrashTreeKey, TrashTreeModel};
use crate::toast_scope::ToastWorkExt;

/// How long the Undo affordance stays live before a destructive op becomes
/// permanent (matches the toast's own visible countdown — the default
/// auto-dismiss window).
const TRASH_UNDO_GRACE: Duration = Duration::from_secs(10);

#[derive(Clone)]
pub struct TrashViewModel {
    app_ctx: Rc<AppContext>,
    model: TrashTreeModel,
    selection: KeyedSelectionModel<TrashTreeKey>,
    docking: DockingModel,
    dock_id: DockWidgetId,
    ids: AppIds,
}

// The accessors/commands are the dock's public API; wired incrementally, so not
// every one has a caller yet — same convention as `view_models::outline`.
#[allow(dead_code)]
impl TrashViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        model: TrashTreeModel,
        docking: DockingModel,
        dock_id: DockWidgetId,
    ) -> Self {
        Self {
            app_ctx,
            // A multi-select model: batch restore/delete act on the whole
            // selection; `reload`'s `prune_missing` handles a multi-set.
            selection: KeyedSelectionModel::new(SelectionMode::Multi),
            model,
            docking,
            dock_id,
            ids,
        }
    }

    pub fn app_ctx(&self) -> Rc<AppContext> {
        self.app_ctx.clone()
    }
    /// The open-Work id signal (for a picker's fresh binder-tree model).
    pub fn work_id(&self) -> Signal<Option<u64>> {
        self.ids.work_id.clone()
    }
    pub fn model(&self) -> TrashTreeModel {
        self.model.clone()
    }
    pub fn selection(&self) -> KeyedSelectionModel<TrashTreeKey> {
        self.selection.clone()
    }
    pub fn docking(&self) -> DockingModel {
        self.docking.clone()
    }
    pub fn dock_id(&self) -> DockWidgetId {
        self.dock_id
    }

    /// Reveal the trash dock (a switchable leading tab, so `reveal_dock`, not a
    /// side-visibility toggle).
    pub fn show(&self) {
        self.docking.reveal_dock(self.dock_id);
    }

    /// Subscribe the tree model to backend changes. Call from the dock's `build`.
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.model.wire(ctx);
    }

    pub fn reload(&self) {
        self.model.reload();
        let model = self.model.clone();
        self.selection.prune_missing(move |k| model.contains(k));
    }

    fn stack(&self) -> Option<u64> {
        self.ids.stack_id.get()
    }

    /// The trash index for the open Work (source of truth — see `TrashTreeModel`).
    fn indexed(&self) -> Vec<u64> {
        let Some(work) = self.ids.work_id.get() else {
            return Vec::new();
        };
        work_commands::get_work_relationship(
            &self.app_ctx,
            &work,
            &WorkRelationshipField::TrashInfos,
        )
        .unwrap_or_default()
    }

    /// The trashed `BinderItem` id a `TrashInfo` points at (item entries only).
    fn item_of_trash_info(&self, trash_info_id: u64) -> Option<u64> {
        trash_info_commands::get_trash_info(&self.app_ctx, &trash_info_id)
            .ok()
            .flatten()
            .and_then(|i| i.trashed_binder_item)
    }

    fn item_title(&self, item_id: u64) -> String {
        binder_item_commands::get_binder_item(&self.app_ctx, &item_id)
            .ok()
            .flatten()
            .map(|i| i.title)
            .unwrap_or_default()
    }

    /// Roots inside the selection (cascade rows are inert for actions).
    pub fn selected_roots(&self) -> Vec<u64> {
        self.selection
            .selected_keys()
            .into_iter()
            .filter_map(|k| match k {
                TrashTreeKey::Root(id) => Some(id),
                TrashTreeKey::Descendant(_) => None,
            })
            .collect()
    }

    /// True when a `TrashInfo` root is an item entry (whole-binder roots can't be
    /// "restored to" — binders aren't nested).
    pub fn is_item_root(&self, trash_info_id: u64) -> bool {
        matches!(self.model.kind_of(trash_info_id), Some(TrashRootKind::Item))
    }

    // ── restore (in place, orphan → picker) ──────────────────────────────────

    /// Restore trashed roots in place. If the backend reports `orphaned`, the
    /// still-indexed (failed) item roots each open the destination picker in turn.
    pub fn restore(&self, ctx: &mut EventContext, roots: &[u64]) {
        if roots.is_empty() {
            return;
        }
        // No open project → nothing to restore into. Same guard as
        // `empty_trash` — a data-loss-shaped op bails loudly rather than
        // guessing which open Work the caller meant.
        let Some(work_id) = self.ids.work_id.get() else {
            ctx.show_toast(Toast::error(tr!(trash_restore_no_project())));
            return;
        };
        let dto = RestoreItemsDto {
            work_id,
            trash_info_ids: roots.iter().map(|&x| x as i64).collect(),
        };
        match trash_management_commands::restore_items(&self.app_ctx, self.stack(), &dto) {
            Ok(res) => {
                self.reload();
                if res.orphaned {
                    // Which requested roots are still in the index (i.e. failed)?
                    let after: HashSet<u64> = self.indexed().into_iter().collect();
                    let failed_items: Vec<u64> = roots
                        .iter()
                        .filter(|id| after.contains(id))
                        .filter_map(|&tid| self.item_of_trash_info(tid))
                        .collect();
                    if failed_items.is_empty() {
                        ctx.show_toast(
                            Toast::warning(tr!(trash_restore_orphaned()))
                                .target_work(Some(work_id)),
                        );
                    } else {
                        ctx.show_toast(
                            Toast::info(tr!(trash_restore_orphaned())).target_work(Some(work_id)),
                        );
                        self.open_orphan_picker_chain(ctx, failed_items);
                    }
                } else {
                    ctx.show_toast(
                        Toast::success(tr!(trash_restored_ok(count = res.restored_count)))
                            .target_work(Some(work_id)),
                    );
                }
            }
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(trash_restore_error(error = e.to_string())))
                        .target_work(Some(work_id)),
                );
            }
        }
    }

    /// Present the destination picker for each still-orphaned item, one at a time.
    fn open_orphan_picker_chain(&self, ctx: &mut EventContext, items: Vec<u64>) {
        let queue = Rc::new(RefCell::new(VecDeque::from(items)));
        self.present_next_orphan(ctx, queue);
    }

    fn present_next_orphan(&self, ctx: &mut EventContext, queue: Rc<RefCell<VecDeque<u64>>>) {
        let Some(item_id) = queue.borrow_mut().pop_front() else {
            return;
        };
        let title = self.item_title(item_id);
        let vm = self.clone();
        let q = queue.clone();
        let on_done: Rc<dyn Fn(&mut EventContext)> =
            Rc::new(move |c| vm.present_next_orphan(c, q.clone()));
        crate::trash::restore_target_panel::present_trash_restore_target(
            ctx,
            self.clone(),
            item_id,
            title,
            on_done,
        );
    }

    // ── restore to a chosen destination ──────────────────────────────────────

    /// Open the destination picker for a single trashed item (the banner /
    /// descendant "Restore to…" entry point).
    pub fn restore_item(&self, ctx: &mut EventContext, item_id: u64) {
        let title = self.item_title(item_id);
        crate::trash::restore_target_panel::present_trash_restore_target(
            ctx,
            self.clone(),
            item_id,
            title,
            Rc::new(|_| {}),
        );
    }

    /// Perform the relocation restore (called by the picker once a destination is
    /// chosen). Peels the item's trashed subtree into `destination_binder_id`.
    pub fn restore_to(
        &self,
        item_id: u64,
        destination_binder_id: u64,
        anchor_item_id: Option<u64>,
        drop_position: DropPosition,
    ) -> anyhow::Result<RestoreItemsToResultDto> {
        // No open project → nothing to restore into. The caller already
        // renders any `Err` here as an error toast (see
        // `restore_target_panel.rs`), so a plain error is enough — no
        // dedicated toast copy needed for this rarer, already-error-handled
        // path.
        let work_id = self
            .ids
            .work_id
            .get()
            .ok_or_else(|| anyhow::anyhow!(tr!(trash_restore_no_project()).resolve_now()))?;
        let dto = RestoreItemsToDto {
            work_id,
            binder_item_ids: vec![item_id],
            destination_binder_id,
            anchor_item_id,
            drop_position,
        };
        let res = trash_management_commands::restore_items_to(&self.app_ctx, self.stack(), &dto)?;
        self.reload();
        Ok(res)
    }

    // ── delete forever / empty (destructive, undo-on-a-timer) ─────────────────

    pub fn confirm_delete_forever(&self, ctx: &mut EventContext, roots: &[u64]) {
        if roots.is_empty() {
            return;
        }
        let count = roots.len() as i64;
        let vm = self.clone();
        let roots = roots.to_vec();
        MessageBox::question(tr!(trash_delete_forever_confirm_title()))
            .text(tr!(trash_delete_forever_confirm_text(count = count)))
            .buttons(MessageBoxButtons::OkCancel)
            .on_result(move |r, c| {
                if r.button == StandardButton::Ok {
                    vm.delete_forever(c, &roots);
                }
            })
            .present(ctx);
    }

    pub fn delete_forever(&self, ctx: &mut EventContext, roots: &[u64]) {
        // No open project → nothing to delete. Same guard as `empty_trash` —
        // without it, a wrong/closed work_id would either error loudly (good)
        // or, before the backend validated it, silently no-op ("Delete
        // forever" reporting success while deleting nothing).
        let Some(work_id) = self.ids.work_id.get() else {
            ctx.show_toast(Toast::error(tr!(trash_delete_no_project())));
            return;
        };
        let ids: Vec<u64> = roots.to_vec();
        let count = roots.len() as i64;
        self.run_with_undo_toast(
            ctx,
            tr!(trash_deleted_title()),
            tr!(trash_deleted_body(count = count)),
            move |app_ctx, stack| {
                trash_management_commands::delete_trash_entries(
                    app_ctx,
                    stack,
                    &DeleteTrashEntriesDto {
                        work_id,
                        trash_info_ids: ids,
                    },
                )
            },
        );
    }

    pub fn confirm_empty_trash(&self, ctx: &mut EventContext) {
        let count = self.model.visible_root_count() as i64;
        if count == 0 {
            return;
        }
        let vm = self.clone();
        MessageBox::question(tr!(trash_empty_confirm_title()))
            .text(tr!(trash_empty_confirm_text(count = count)))
            .buttons(MessageBoxButtons::OkCancel)
            .on_result(move |r, c| {
                if r.button == StandardButton::Ok {
                    vm.empty_trash(c);
                }
            })
            .present(ctx);
    }

    pub fn empty_trash(&self, ctx: &mut EventContext) {
        // No open project → nothing to empty. Data-loss-shaped op, so this bails
        // loudly rather than guessing a Work — see `EmptyTrashDto`'s manifest doc.
        let Some(work_id) = self.ids.work_id.get() else {
            ctx.show_toast(Toast::error(tr!(trash_empty_no_project())));
            return;
        };
        self.run_with_undo_toast(
            ctx,
            tr!(trash_emptied_title()),
            tr!(trash_emptied_body()),
            move |app_ctx, stack| {
                trash_management_commands::empty_trash(app_ctx, stack, &EmptyTrashDto { work_id })
            },
        );
    }

    /// Run a destructive (but undoable) op, then raise a warning toast with an
    /// **Undo** action and a grace window. On the window lapsing (timeout / the
    /// user dismissing it) the project undo history is cleared — the commit point.
    /// The Undo action reverses the op; eviction / shutdown never force-commit.
    fn run_with_undo_toast(
        &self,
        ctx: &mut EventContext,
        title: LocalizedString,
        body: LocalizedString,
        op: impl FnOnce(&AppContext, Option<u64>) -> anyhow::Result<()>,
    ) {
        let work_id = self.ids.work_id.get();
        let stack = self.stack();
        if let Err(e) = op(&self.app_ctx, stack) {
            ctx.show_toast(
                Toast::error(tr!(trash_restore_error(error = e.to_string()))).target_work(work_id),
            );
            return;
        }
        self.reload();
        let undo_ctx = self.app_ctx.clone();
        let clear_ctx = self.app_ctx.clone();
        ctx.show_toast(
            Toast::warning(title)
                .body(body)
                .priority(ToastPriority::High) // never evicted before the window ends
                // Work-scoped (F2): a bare "trash.commit" shared by every window
                // would let a second Work's destructive op find THIS Work's
                // still-live Undo toast (`ToastRegistry::enqueue` dedups on id
                // alone) and silently steal/retarget it — the Undo action would
                // then reverse the wrong Work's op, and this Work's own
                // still-pending deletion would commit unseen once its grace
                // timer (now overwritten) never fires. A second op on THIS SAME
                // Work still replaces the toast (one grace timer per Work), matching
                // the comment this replaced.
                .scoped_id("trash.commit", work_id)
                .auto_dismiss_after(TRASH_UNDO_GRACE)
                .target_work(work_id)
                .action(ToastAction::primary(tr!(trash_undo()), move |_c| {
                    let _ = undo_redo_commands::undo(&undo_ctx, stack);
                }))
                .on_dismiss(move |cause, _c| {
                    // Only the grace timer expiring is the point of no return: it
                    // clears the project's undo history so the deletion can't be
                    // reverted. Every other dismissal — Undo clicked, the user
                    // closing the toast (✕/Esc), eviction, shutdown — leaves the op
                    // undoable. (Dismissing a notification must not silently wipe
                    // the undo history; and clear only THIS work's stack, not every
                    // stack.)
                    //
                    // Deliberately no `clear_all_stacks` fallback for a `None` stack
                    // (the multi-Work migration removed it): with several Works open
                    // at once, each with its own stack, "clear every stack because
                    // this one couldn't be resolved" would wipe every *other* open
                    // Work's undo history too — a correctness regression far worse
                    // than leaving this one op's history alone. A live `TrashViewModel`
                    // always has a seeded `stack_id` once a Work is open (`AppIds::open_stack`
                    // runs on every `LoadWork`/`NewWork`), so `None` here means no Work
                    // is open at all — nothing to clear, and the op above could not
                    // have succeeded either.
                    if cause == ToastDismissCause::Timeout
                        && let Some(sid) = stack
                    {
                        undo_redo_commands::clear_stack(&clear_ctx, sid);
                    }
                }),
        );
    }
}

#[cfg(all(test, feature = "mocks"))]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;

    fn vm() -> TrashViewModel {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::default();
        let model = TrashTreeModel::new(app_ctx.clone(), ids.work_id.clone());
        TrashViewModel::new(
            app_ctx,
            ids,
            model,
            DockingModel::new(),
            DockWidgetId::from_raw(crate::docks::TRASH_DOCK_ID),
        )
    }

    #[test]
    fn selected_roots_filters_to_root_keys() {
        let vm = vm();
        // No selection → no roots.
        assert!(vm.selected_roots().is_empty());
    }

    #[test]
    fn is_item_root_reads_the_model() {
        let vm = vm();
        // Mock fixture: 9001 = whole binder, 9002 = item.
        assert!(!vm.is_item_root(9001));
        assert!(vm.is_item_root(9002));
    }

    /// `f2_toast_id_tests::two_works_destructive_undo_toasts_never_collide` only
    /// proves `work_scoped_toast_id` itself is collision-free — it never touches
    /// `run_with_undo_toast`'s actual `.scoped_id(...)` call site,
    /// so reverting that call site back to a bare `"trash.commit"` would still
    /// leave it green. This one drives the real (private, but same-file-testable)
    /// `run_with_undo_toast` through a real `ToastRegistry`: two `TrashViewModel`s
    /// captured for two different Works each raise their destructive-op Undo
    /// toast through a wired `Button` + a dispatched click (a real
    /// `EventContext`), then asserts both stay live — `ToastRegistry::enqueue`'s
    /// update-in-place merge would collapse them to ONE entry (stealing Work A's
    /// still-pending Undo grace window) if the id were ever bare again.
    #[test]
    fn destructive_undo_toasts_for_two_works_both_stay_live_in_a_real_registry() {
        use bastyde::i18n::lit;
        use bastyde::widgets::{Button, ToastInstallOptions, ToastRegistry};

        let vm_a = vm();
        vm_a.ids.work_id.set(Some(1));
        let vm_b = vm();
        vm_b.ids.work_id.set(Some(2));

        let registry = ToastRegistry::new(ToastInstallOptions {
            archive: None,
            ..ToastInstallOptions::default()
        });
        let mut tree = crate::test_support::tree_with_toast_registry(&vm_a.app_ctx, &registry);

        let a = vm_a.clone();
        let b = vm_b.clone();
        let btn_a = tree.add(Button::new(lit!("a")).on_activate_fn(move |ctx| {
            a.run_with_undo_toast(ctx, lit!("Emptied"), lit!("Undo?"), |_ctx, _stack| Ok(()));
        }));
        let btn_b = tree.add(Button::new(lit!("b")).on_activate_fn(move |ctx| {
            b.run_with_undo_toast(ctx, lit!("Emptied"), lit!("Undo?"), |_ctx, _stack| Ok(()));
        }));
        tree.layout(SizeProposal::exact(200.0, 80.0));

        crate::test_support::click(&mut tree, btn_a);
        crate::test_support::click(&mut tree, btn_b);

        assert_eq!(
            registry.live_count(),
            2,
            "two different Works' destructive-op Undo toasts must both stay live — a \
             bare \"trash.commit\" id would let Work B's enqueue find Work A's still-live \
             entry (ToastRegistry::enqueue dedups on id alone) and merge into it, \
             leaving only 1"
        );
    }
}

// Not gated on the `mocks` feature (unlike the module above) — a pure string
// check, so it runs under both default and `--features mocks` builds.
#[cfg(test)]
mod f2_toast_id_tests {
    /// F2: `run_with_undo_toast`'s Undo toast (Empty Trash / Delete Forever) used
    /// a bare `"trash.commit"` id shared by every window. `ToastRegistry::enqueue`
    /// dedups on id alone (no route check) and overwrites the matched entry's
    /// route in place — so a second Work committing its own destructive op would
    /// find THIS Work's still-live Undo toast, silently steal it, and retarget it
    /// to the second Work's audience. This Work's own deletion would then commit
    /// unseen (its grace timer overwritten by the steal), and clicking "Undo" on
    /// the visible toast would reverse the WRONG Work's op. Both Works' Undo
    /// toasts must survive independently.
    #[test]
    fn two_works_destructive_undo_toasts_never_collide() {
        let a = crate::toast_scope::work_scoped_toast_id("trash.commit", Some(1));
        let b = crate::toast_scope::work_scoped_toast_id("trash.commit", Some(2));
        assert_ne!(
            a, b,
            "two different Works' Empty-Trash/Delete-Forever Undo toasts must \
             never collide in the shared ToastRegistry"
        );
    }
}
