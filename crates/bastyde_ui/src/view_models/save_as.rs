// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SaveAsViewModel` — records the new path/shape into `WorkInfo` after a
//! background "Save As" completes.
//!
//! `save_as` runs **read-only** on a background thread (it must not hold a
//! whole-store write savepoint there — see `save_as_uc.rs`). When the long
//! operation completes, this view-model — wired to the `Origin::LongOperation`
//! events in `App::build` — reads the result and applies the new `file_name` +
//! `WorkShape` to `WorkInfo` **synchronously on the UI thread** via
//! `update_work_info`, which fires `WorkInfo Updated` and refreshes
//! `SingleWorkInfo` (flipping the "Save as…" menu). Built fresh per window
//! (`shell::windows::ProjectWindowFactory::window_config`), bound to that
//! window's own `ids` — never a shared instance, since a second
//! simultaneously-open Work must never see, or drive, this window's Save As.
//!
//! In-flight ops are keyed by their long-operation id, and each records the
//! `WorkInfo` id **and** the Work id captured **when the op started** — so
//! completion always targets the project that was actually saved, even if the
//! user switched projects while the background save was running, and
//! concurrent Save-As ops don't drop each other's completion.
//!
//! Every completion/failure toast below routes on [`Pending::tracked`]'s
//! `work_id()` — a [`super::long_op::CapturedWork`] bundled with the op id in
//! a [`super::long_op::TrackedOp`], captured once in [`SaveAsViewModel::start`]
//! — never a live `self.ids.work_id.get()`. This view-model outlives any one
//! Save As, including across an in-place project switch in the SAME window
//! while the background op is still running (`ProjectSwitchViewModel::request`
//! gates a switch only on unsaved edits/autosave, never on "is a Save As
//! running"). A switch reseeds `ids.work_id` on the SAME `Signal` this
//! view-model holds, so a handler reading it live would silently answer with
//! whichever Work this window shows *at completion time*, not the Work that
//! was actually saved — misrouting the toast to the new Work while the Work
//! that was really saved never hears it finished (see `long_op::CapturedWork`'s
//! and `long_op::TrackedOp`'s own docs).
//!
//! [`SaveAsViewModel::begin`] is the **only** door to the backend `save_as`: it
//! flushes the live editor buffers into the store before the background op reads
//! it. Typing does not write through to `Content` — it only marks the doc dirty
//! (`OpenDoc::mark_dirty_fn`) until an explicit flush — so a Save As that skipped
//! that step would serialize the *pre-edit* prose and silently write a file
//! missing everything typed since the last flush boundary. That is worst in
//! backup mode, where Save is off and Save As is the only way to keep the edits
//! at all. Same invariant as `save_work` (via `EditorsViewModel::save_to_disk`)
//! and every backup trigger (via the scheduler's flush hook).

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use bastyde::prelude::*; // EventContext, tr!
use bastyde::widgets::Toast;

use frontend::AppContext;
use frontend::commands::{work_info_commands, work_management_commands};
use frontend::common::entities::WorkShape;
use frontend::common::event::Event;
use frontend::direct_access::UpdateWorkInfoDto;
use frontend::work_management::SaveAsDto;

use crate::app_ids::AppIds;
use crate::backup::BackupContext;
use crate::singles::SingleWork;
use crate::toast_scope::ToastWorkExt;

use super::long_op::{CapturedWork, TrackedOp, event_id, parse_payload};

/// Update-in-place key for the single toast a Save As drives (starting →
/// success / error) — folded through [`crate::toast_scope::work_scoped_toast_id`] with the
/// captured [`Pending::tracked`]'s `work_id()` at every use, never bare, for the same
/// reason [`super::export::ExportViewModel`]'s own toast id is: two Works
/// running their own Save As at once must never collide in the shared
/// `ToastRegistry` (`ToastRegistry::enqueue` dedups on id alone). Reusing one
/// id across the "Saving as…" → "Saved as"/error toasts also means the
/// completion replaces the in-progress toast in place, instead of leaving it
/// stacked alongside a second one.
const SAVE_AS_TOAST_ID: &str = "save_as.work";

/// What a running Save-As needs to record on completion — pinned at start time.
struct Pending {
    /// The long-operation id (also this entry's `HashMap` key, kept here too
    /// so a handler need not thread the key through separately) bundled with
    /// the Work it was captured for (F4) — see `long_op::TrackedOp`'s doc and
    /// the module doc's "F1" section. Every completion/failure toast routes
    /// on `tracked.work_id()`, never a live `self.ids.work_id.get()`.
    tracked: TrackedOp,
    as_folder: bool,
    /// The `WorkInfo` id of the project being saved, captured at `start()`. `None`
    /// if no project was open (shouldn't happen — Save As requires one).
    work_info_id: Option<u64>,
}

#[derive(Clone)]
pub struct SaveAsViewModel {
    /// In-flight Save-As ops keyed by long-operation id. A map (not a single
    /// slot) so a second Save As can't silently drop the first's completion.
    pending: Rc<RefCell<HashMap<String, Pending>>>,
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    single_work: SingleWork,
    /// Cleared on a successful Save As: whatever this window was showing, it now
    /// points at the freshly-written file, and `from_entities` always stamps that
    /// `kind: Regular` — so it is never a backup. This is what lets "Save As" be
    /// the escape hatch out of backup mode (the banner's other exit is Restore).
    backup_mode: Signal<bool>,
    backup_context: Signal<Option<BackupContext>>,
    /// Pushes the live editor buffers into the store (in practice
    /// `editors.flush_all()`), installed by `App::build` once the editors exist.
    /// A no-op until then — and in headless tests, which is why it is a hook
    /// rather than a hard `EditorsViewModel` dependency. Shared (`Rc<RefCell<_>>`)
    /// so installing it is visible on every clone already handed out.
    flush_hook: Rc<RefCell<Rc<dyn Fn()>>>,
}

impl SaveAsViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        single_work: SingleWork,
        backup_mode: Signal<bool>,
        backup_context: Signal<Option<BackupContext>>,
    ) -> Self {
        Self {
            pending: Rc::new(RefCell::new(HashMap::new())),
            app_ctx,
            ids,
            single_work,
            backup_mode,
            backup_context,
            flush_hook: Rc::new(RefCell::new(Rc::new(|| {}) as Rc<dyn Fn()>)),
        }
    }

    /// Install the real flush hook (`editors.flush_all()`). Called once from
    /// `App::build`; visible on every existing clone.
    pub fn set_flush_hook(&self, hook: Rc<dyn Fn()>) {
        *self.flush_hook.borrow_mut() = hook;
    }

    /// Copy the live editor buffers into the store. Cheap when nothing is dirty.
    fn flush(&self) {
        let hook = self.flush_hook.borrow().clone();
        hook();
    }

    /// Start a Save As to `target` — **the only door to the backend `save_as`**.
    ///
    /// Flushes the editors first (see the module docs: without it the background
    /// op serializes the pre-edit prose), then starts the long operation, records
    /// it as in-flight, and toasts. On a start failure the error is surfaced and
    /// nothing is registered.
    ///
    /// The "starting" toast routes on the Work [`Self::start_flushed`] just
    /// captured (F1), not a fresh live read — at this exact instant the two
    /// agree, but reading the same snapshot [`Self::start`] recorded keeps this
    /// in lockstep with every later handler below, which cannot re-read live.
    pub fn begin(&self, ctx: &mut EventContext, target: String, as_folder: bool) {
        match self.start_flushed(target.clone(), as_folder) {
            Ok((work_id, op_id)) => {
                let toast = if as_folder {
                    tr!(saving_as_folder(target = target))
                } else {
                    tr!(saving_as_file(target = target))
                };
                ctx.show_toast(
                    // Keyed on the operation, not just its Work: two windows on
                    // one Work can each run a Save As, and a Work-only key would
                    // let the second overwrite the first's toast in place.
                    Toast::info(toast)
                        .scoped_op_id(SAVE_AS_TOAST_ID, work_id, &op_id)
                        .target_work(work_id),
                );
            }
            Err(e) => {
                // The op never started, so nothing was captured to route on
                // instead — the window's current Work is the only Work this
                // failure could ever concern.
                let work_id = self.ids.work_id.get();
                ctx.show_toast(
                    Toast::error(tr!(save_error(error = e.to_string())))
                        .scoped_id(SAVE_AS_TOAST_ID, work_id)
                        .target_work(work_id),
                );
            }
        }
    }

    /// [`Self::begin`] without the toasts: flush the editors, start the background
    /// op, register it as in-flight, and return the Work it was captured for
    /// (F1). The background op is read-only, so it reads the store the flush
    /// just wrote; `on_long_op_completed` records the new path/shape into
    /// `WorkInfo` when it lands.
    ///
    /// Split out ctx-free so the flush-before-serialize invariant is testable
    /// headlessly (this crate has no `EventContext` harness — see the tests below
    /// and `backup_scheduler.rs`'s).
    ///
    /// Returns the captured Work **and the operation's own id** — [`Self::begin`]
    /// needs the latter for its "starting" toast's dedup key, which is per
    /// operation rather than merely per Work (see `ToastWorkExt::scoped_op_id`).
    fn start_flushed(
        &self,
        target: String,
        as_folder: bool,
    ) -> anyhow::Result<(CapturedWork, String)> {
        self.flush();
        let op_id = work_management_commands::save_as(
            &self.app_ctx,
            &SaveAsDto {
                media_root: crate::media_paths::media_root_string(),
                work_id: self.ids.work_id.get().unwrap_or_default(),
                file_name: target,
                as_folder,
            },
        )?;
        Ok((self.start(op_id.clone(), as_folder), op_id))
    }

    /// Register the in-flight Save As, capturing the CURRENT `WorkInfo` id (the
    /// project being saved) and the CURRENT Work (F1 — a [`super::long_op::CapturedWork`])
    /// so completion targets both even if the user switches projects before the
    /// background op finishes. Called by [`Self::start_flushed`] once the long
    /// operation has started; returns the captured Work for [`Self::begin`]'s own
    /// "starting" toast.
    fn start(&self, op_id: String, as_folder: bool) -> CapturedWork {
        let work_info_id = self.ids.work_info_id.get();
        // Captured NOW, bundled with the op id — see `long_op::TrackedOp`'s doc.
        let tracked = TrackedOp::start(&self.ids, op_id.clone());
        let work_id = tracked.work_id();
        self.pending.borrow_mut().insert(
            op_id,
            Pending {
                tracked,
                as_folder,
                work_info_id,
            },
        );
        work_id
    }

    /// A Save As finished: if it was one of ours, record the new path/shape into
    /// the *originating* project's `WorkInfo` on the UI thread.
    pub fn on_long_op_completed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = event_id(event) else {
            return;
        };
        let Some(pending) = self.pending.borrow_mut().remove(&op_id) else {
            return; // not one of our Save-As ops (import/backup/etc.)
        };

        let output_path = match work_management_commands::get_save_as_result(&self.app_ctx, &op_id)
        {
            Ok(Some(res)) => res.output_path,
            // Completed without a recoverable result (shouldn't happen).
            Ok(None) | Err(_) => return,
        };
        // The WorkInfo of the project that was saved — NOT the currently-open one.
        let Some(id) = pending.work_info_id else {
            return; // no project was open when it started
        };
        // Re-read the stored WorkInfo to preserve `created_at` (a scalar update
        // overwrites every field). If it's gone (project closed), skip.
        let Ok(Some(cur)) = work_info_commands::get_work_info(&self.app_ctx, &id) else {
            return;
        };
        let dto = UpdateWorkInfoDto {
            id,
            created_at: cur.created_at,
            updated_at: chrono::Utc::now(),
            file_name: Some(output_path.clone()),
            shape: if pending.as_folder {
                WorkShape::Folder
            } else {
                WorkShape::Zip
            },
        };
        match work_info_commands::update_work_info(&self.app_ctx, &dto) {
            Ok(_) => {
                // The window now *is* the file it just wrote. If it was showing a
                // backup, that's no longer true: drop backup mode (banner goes, Save
                // re-enables) instead of leaving a read-only-file window pointed at a
                // regular project. Re-claim so other instances see the right path.
                if self.backup_mode.get() {
                    self.backup_mode.set(false);
                    self.backup_context.set(None);
                }
                // This window now points at the freshly-written output path with no
                // `LoadWork`/`CloseWork` in between, so release whatever claim it held
                // before (`cur.file_name`, read above before this update overwrote it) and
                // claim the new path. NOT `replace_claim`/`release_all()`: that drops every
                // claim the whole *process* holds, which with a second Work open in a
                // second window would silently un-claim that sibling's still-open,
                // untouched project too — see `open_registry::release`'s own doc
                // anticipating exactly this.
                if let Some(prev) = cur.file_name.as_deref() {
                    crate::shell::open_registry::release(prev);
                }
                crate::shell::open_registry::claim(&output_path, &self.single_work.title().get());
                // Routes AND scopes on the Work THIS Save As started for — see
                // `Pending::tracked`'s doc — never a live `self.ids.work_id.get()`.
                ctx.show_toast(
                    Toast::success(tr!(saved_as(target = output_path)))
                        .scoped_op_id(SAVE_AS_TOAST_ID, pending.tracked.work_id(), &op_id)
                        .target_work(pending.tracked.work_id()),
                );
            }
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(save_error(error = e.to_string())))
                        .scoped_op_id(SAVE_AS_TOAST_ID, pending.tracked.work_id(), &op_id)
                        .target_work(pending.tracked.work_id()),
                );
            }
        };
    }

    /// A Save As failed: if it was one of ours, surface the error. The background
    /// operation is read-only, so nothing in the store needs undoing.
    pub fn on_long_op_failed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = event_id(event) else {
            return;
        };
        let Some(pending) = self.pending.borrow_mut().remove(&op_id) else {
            return; // not one of ours
        };
        let error = parse_payload(event)
            .and_then(|p| p.get("error").and_then(|e| e.as_str()).map(str::to_string))
            .unwrap_or_default();
        // Routes AND scopes on the Work THIS Save As started for — see
        // `Pending::tracked`'s doc — never a live `self.ids.work_id.get()`.
        ctx.show_toast(
            Toast::error(tr!(save_error(error = error)))
                .scoped_op_id(SAVE_AS_TOAST_ID, pending.tracked.work_id(), &op_id)
                .target_work(pending.tracked.work_id()),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    /// No project is open in these tests, so `start_flushed`'s backend call is a
    /// no-op-ish long op that finds nothing to write — what is under test is the
    /// flush that happens *before* it, which is the invariant Save As used to
    /// violate. (`begin` itself needs a real `&mut EventContext` for its toasts,
    /// and this crate has no `EventContext` harness — same constraint the backup
    /// scheduler's flush tests work around, and why `start_flushed` is ctx-free.)
    fn test_vm() -> SaveAsViewModel {
        let app_ctx = Rc::new(AppContext::new());
        let single_work = SingleWork::new(app_ctx.clone());
        SaveAsViewModel::new(
            app_ctx,
            AppIds::default(),
            single_work,
            Signal::new(false),
            Signal::new(None),
        )
    }

    fn target() -> String {
        std::env::temp_dir()
            .join("skribisto-save-as-flush-test.skrib")
            .to_string_lossy()
            .into_owned()
    }

    #[test]
    fn save_as_flushes_the_editors_before_it_reads_the_store() {
        let vm = test_vm();
        let flushed = Rc::new(Cell::new(0u32));
        {
            let flushed = flushed.clone();
            vm.set_flush_hook(Rc::new(move || flushed.set(flushed.get() + 1)));
        }
        let _ = vm.start_flushed(target(), false);
        assert_eq!(
            flushed.get(),
            1,
            "Save As must flush the live editor buffers into the store, or the \
             background op serializes the pre-edit prose"
        );
    }

    #[test]
    fn flush_hook_defaults_to_a_harmless_no_op() {
        // Constructing the view-model without installing a hook (the headless
        // shape) must not panic: `start_flushed`'s `self.flush()` is always safe.
        let vm = test_vm();
        let _ = vm.start_flushed(target(), false);
    }

    #[test]
    fn set_flush_hook_is_visible_on_every_existing_clone() {
        // The hook cell is shared, so installing it on ONE clone (as `App::build`
        // does) must be visible on clones handed out earlier — e.g. the ones held
        // by the title-bar menu and the backup banner.
        let vm = test_vm();
        let earlier_clone = vm.clone();
        let flushed = Rc::new(Cell::new(0u32));
        {
            let flushed = flushed.clone();
            vm.set_flush_hook(Rc::new(move || flushed.set(flushed.get() + 1)));
        }
        let _ = earlier_clone.start_flushed(target(), false);
        assert_eq!(flushed.get(), 1, "the earlier clone must see the new hook");
    }

    // ── F1: the Work a Save As started for must survive a later in-place switch ─
    //
    // `SaveAsViewModel` is minted once per window and outlives any one Save As
    // (see the module doc's "F1" section). Before this fix, `on_long_op_completed`/
    // `on_long_op_failed` re-read `self.ids.work_id.get()` live, so a window that
    // started a Save As, then switched to a different Work before it finished,
    // would route (and dedup) the completion/failure toast against the NEW Work
    // — the Work that was actually saved never hearing about it. This pins the
    // fix through the real call site: `on_long_op_failed` doesn't touch the
    // backend beyond parsing its payload, so it's the one handler testable
    // without a real long-operation result.

    #[test]
    fn the_captured_work_id_survives_a_later_in_place_switch() {
        let vm = test_vm();
        vm.ids.work_id.set(Some(1));

        // Simulate what `Self::start` does the instant the long operation
        // begins: snapshot the window's current Work into `Pending`.
        vm.pending.borrow_mut().insert(
            "fake-save-as-op".to_string(),
            Pending {
                tracked: TrackedOp::start(&vm.ids, "fake-save-as-op".to_string()),
                as_folder: false,
                work_info_id: None,
            },
        );
        let captured = vm
            .pending
            .borrow()
            .get("fake-save-as-op")
            .unwrap()
            .tracked
            .work_id();
        assert_eq!(captured, Some(1));

        // An in-place project switch reseeds `ids.work_id` on the SAME `AppIds`
        // this long-lived view-model holds (`ProjectSwitchViewModel::request`
        // gates only on unsaved edits, never on a Save As in flight).
        vm.ids.work_id.set(Some(2));

        assert_eq!(
            vm.pending
                .borrow()
                .get("fake-save-as-op")
                .unwrap()
                .tracked
                .work_id(),
            captured,
            "the in-flight Save As's own Work must stay pinned to what `start` \
             captured, even after this window switches to a different Work"
        );
        assert_ne!(
            vm.pending
                .borrow()
                .get("fake-save-as-op")
                .unwrap()
                .tracked
                .work_id(),
            vm.ids.work_id.get(),
            "the captured Work must now differ from the window's live \
             `ids.work_id` — proving a handler reading `pending.tracked.work_id()` cannot \
             silently be reading the same live value `ids.work_id` would give it"
        );
    }

    #[test]
    fn save_as_toast_ids_for_two_captured_works_never_collide() {
        // The other half of the fix: even with the right Work captured, a bare
        // `SAVE_AS_TOAST_ID` shared by every window would let a second Work's
        // Save As find this one's still-live toast entry (`ToastRegistry::
        // enqueue` dedups on id alone) and silently retarget/steal it.
        let id_a = crate::toast_scope::work_scoped_toast_id(SAVE_AS_TOAST_ID, Some(1));
        let id_b = crate::toast_scope::work_scoped_toast_id(SAVE_AS_TOAST_ID, Some(2));
        assert_ne!(
            id_a, id_b,
            "two different Works' Save-As toasts must never collide"
        );
    }

    /// The test above only proves `work_scoped_toast_id` itself is collision-free
    /// — it never touches `on_long_op_failed`'s actual call site, so reverting
    /// that handler back to a live `self.ids.work_id.get()` would still leave it
    /// green. This one drives the REAL call site through a real `ToastRegistry`,
    /// reproducing the exact F1 scenario the task asks for: a Save As started on
    /// Work A, whose window then switches in place to Work B — landing on the
    /// SAME Work a second, genuinely-running Save As (a different window/VM) is
    /// captured for.
    ///
    /// With the fix, `vm_a`'s failure toast still carries Work A's id (captured
    /// at `start`, before the switch), so it never collides with `vm_b`'s own
    /// Work-B toast — two live entries. Reverted to a live
    /// `self.ids.work_id.get()` read, `vm_a`'s toast would resolve to Work B
    /// too (the window's post-switch Work) and collide with `vm_b`'s
    /// (`ToastRegistry::enqueue` dedups on id alone), collapsing to one entry —
    /// exactly the silent "the wrong Work hears about it, the right one never
    /// does" bug F1 describes.
    #[test]
    fn a_save_as_failure_captured_before_a_switch_never_collides_with_the_new_works_own() {
        use bastyde::i18n::lit;
        use bastyde::widgets::{Button, ToastInstallOptions, ToastRegistry};
        use frontend::common::event::{LongOperationEvent, Origin};

        fn failed_event(op_id: &str) -> Event {
            Event {
                origin: Origin::LongOperation(LongOperationEvent::Failed),
                ids: vec![],
                data: Some(format!(r#"{{"id":"{op_id}","error":"boom"}}"#)),
            }
        }

        // Window A: Save As starts on Work 1…
        let vm_a = test_vm();
        vm_a.ids.work_id.set(Some(1));
        vm_a.pending.borrow_mut().insert(
            "fake-op-a".to_string(),
            Pending {
                tracked: TrackedOp::start(&vm_a.ids, "fake-op-a".to_string()), // captures Work 1
                as_folder: false,
                work_info_id: None,
            },
        );
        // …then window A switches in place to Work 2 — the SAME Work window B
        // is genuinely running its own Save As for, below.
        vm_a.ids.work_id.set(Some(2));

        // Window B: its own Save As, genuinely for Work 2, never switched.
        let vm_b = test_vm();
        vm_b.ids.work_id.set(Some(2));
        vm_b.pending.borrow_mut().insert(
            "fake-op-b".to_string(),
            Pending {
                tracked: TrackedOp::start(&vm_b.ids, "fake-op-b".to_string()), // captures Work 2
                as_folder: false,
                work_info_id: None,
            },
        );

        let registry = ToastRegistry::new(ToastInstallOptions {
            archive: None,
            ..ToastInstallOptions::default()
        });
        let mut tree = crate::test_support::tree_with_toast_registry(&vm_a.app_ctx, &registry);

        let a = vm_a.clone();
        let b = vm_b.clone();
        let btn_a = tree.add(Button::new(lit!("a")).on_activate_fn(move |ctx| {
            a.on_long_op_failed(ctx, &failed_event("fake-op-a"));
        }));
        let btn_b = tree.add(Button::new(lit!("b")).on_activate_fn(move |ctx| {
            b.on_long_op_failed(ctx, &failed_event("fake-op-b"));
        }));
        tree.layout(SizeProposal::exact(200.0, 80.0));

        crate::test_support::click(&mut tree, btn_a);
        crate::test_support::click(&mut tree, btn_b);

        assert_eq!(
            registry.live_count(),
            2,
            "Work A's Save-As failure (captured before the in-place switch) must \
             not collide with Work B's own still-live toast — reverting to a live \
             `self.ids.work_id.get()` read would make both resolve to Work B and \
             merge into a single entry"
        );
    }
}
