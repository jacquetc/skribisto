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
//! `SingleWorkInfo` (flipping the "Save as…" menu). Single-instance live state
//! created in `main.rs`, registered as app-state.
//!
//! In-flight ops are keyed by their long-operation id, and each records the
//! `WorkInfo` id captured **when the op started** — so completion always targets
//! the project that was actually saved, even if the user switched projects while
//! the background save was running, and concurrent Save-As ops don't drop each
//! other's completion.
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

use super::long_op::{event_id, parse_payload};

/// What a running Save-As needs to record on completion — pinned at start time.
struct Pending {
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
    pub fn begin(&self, ctx: &mut EventContext, target: String, as_folder: bool) {
        match self.start_flushed(target.clone(), as_folder) {
            Ok(()) => {
                let toast = if as_folder {
                    tr!(saving_as_folder(target = target))
                } else {
                    tr!(saving_as_file(target = target))
                };
                ctx.show_toast(Toast::info(toast));
            }
            Err(e) => {
                ctx.show_toast(Toast::error(tr!(save_error(error = e.to_string()))));
            }
        }
    }

    /// [`Self::begin`] without the toasts: flush the editors, start the background
    /// op, register it as in-flight. The background op is read-only, so it reads
    /// the store the flush just wrote; `on_long_op_completed` records the new
    /// path/shape into `WorkInfo` when it lands.
    ///
    /// Split out ctx-free so the flush-before-serialize invariant is testable
    /// headlessly (this crate has no `EventContext` harness — see the tests below
    /// and `backup_scheduler.rs`'s).
    fn start_flushed(&self, target: String, as_folder: bool) -> anyhow::Result<()> {
        self.flush();
        let op_id = work_management_commands::save_as(
            &self.app_ctx,
            &SaveAsDto {
                work_id: self.ids.work_id.get().unwrap_or_default(),
                file_name: target,
                as_folder,
            },
        )?;
        self.start(op_id, as_folder);
        Ok(())
    }

    /// Register the in-flight Save As, capturing the CURRENT `WorkInfo` id (the
    /// project being saved) so completion targets it even if the user switches
    /// projects before the background op finishes. Called by [`Self::begin`] once
    /// the long operation has started.
    fn start(&self, op_id: String, as_folder: bool) {
        let work_info_id = self.ids.work_info_id.get();
        self.pending.borrow_mut().insert(
            op_id,
            Pending {
                as_folder,
                work_info_id,
            },
        );
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
                // This window now points at the freshly-written output path with
                // no `LoadWork`/`CloseWork` in between, so drop whatever claim it
                // held before (T1-5's `replace_claim`, not a bare additive `claim`).
                crate::shell::open_registry::replace_claim(&output_path, &self.single_work.title().get());
                ctx.show_toast(Toast::success(tr!(saved_as(target = output_path))));
            }
            Err(e) => {
                ctx.show_toast(Toast::error(tr!(save_error(error = e.to_string()))));
            }
        };
    }

    /// A Save As failed: if it was one of ours, surface the error. The background
    /// operation is read-only, so nothing in the store needs undoing.
    pub fn on_long_op_failed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = event_id(event) else {
            return;
        };
        if self.pending.borrow_mut().remove(&op_id).is_none() {
            return; // not one of ours
        }
        let error = parse_payload(event)
            .and_then(|p| p.get("error").and_then(|e| e.as_str()).map(str::to_string))
            .unwrap_or_default();
        ctx.show_toast(Toast::error(tr!(save_error(error = error))));
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
}
