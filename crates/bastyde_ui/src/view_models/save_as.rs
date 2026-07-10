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
//! (owns the in-flight op id); created in `main.rs`, registered as app-state.

use std::rc::Rc;

use bastyde::prelude::*; // EventContext, Signal, tr!
use bastyde::widgets::Toast;

use frontend::AppContext;
use frontend::commands::{work_info_commands, work_management_commands};
use frontend::common::entities::WorkShape;
use frontend::common::event::Event;
use frontend::direct_access::UpdateWorkInfoDto;

use crate::app_ids::AppIds;

use super::long_op::{event_id, parse_payload};

#[derive(Clone)]
pub struct SaveAsViewModel {
    /// The `(op_id, as_folder)` of the Save As running right now, if any — set on
    /// start, cleared when it completes / fails. Drives event filtering (only the
    /// matching op touches WorkInfo) and picks the `WorkShape` to record.
    active: Signal<Option<(String, bool)>>,
    app_ctx: Rc<AppContext>,
    ids: AppIds,
}

impl SaveAsViewModel {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        Self {
            active: Signal::new(None),
            app_ctx,
            ids,
        }
    }

    /// Register the in-flight Save As so its completion updates `WorkInfo`.
    /// Called from the Save-As menu action once the long operation has started.
    pub fn start(&self, op_id: String, as_folder: bool) {
        self.active.set(Some((op_id, as_folder)));
    }

    /// The Save As finished: fetch its output path and record the new
    /// path/shape into `WorkInfo` on the UI thread (fires `WorkInfo Updated`).
    pub fn on_long_op_completed(&self, ctx: &mut EventContext, event: &Event) {
        let Some((op_id, as_folder)) = self.active.get() else {
            return;
        };
        if event_id(event).as_deref() != Some(op_id.as_str()) {
            return;
        }
        self.active.set(None);

        let output_path = match work_management_commands::get_save_as_result(&self.app_ctx, &op_id) {
            Ok(Some(res)) => res.output_path,
            // Completed without a recoverable result (shouldn't happen) — nothing
            // to record.
            Ok(None) | Err(_) => return,
        };

        let Some(id) = self.ids.work_info_id.get() else {
            return;
        };
        // Re-read the stored WorkInfo to preserve `created_at` — a scalar update
        // overwrites every field.
        let Ok(Some(cur)) = work_info_commands::get_work_info(&self.app_ctx, &id) else {
            return;
        };
        let dto = UpdateWorkInfoDto {
            id,
            created_at: cur.created_at,
            updated_at: chrono::Utc::now(),
            file_name: Some(output_path.clone()),
            shape: if as_folder {
                WorkShape::Folder
            } else {
                WorkShape::Zip
            },
        };
        match work_info_commands::update_work_info(&self.app_ctx, &dto) {
            Ok(_) => ctx.show_toast(Toast::success(tr!(saved_as(target = output_path)))),
            Err(e) => ctx.show_toast(Toast::error(tr!(save_error(error = e.to_string())))),
        };
    }

    /// The Save As failed: surface the error. The background operation is
    /// read-only, so nothing in the store needs undoing.
    pub fn on_long_op_failed(&self, ctx: &mut EventContext, event: &Event) {
        let Some((op_id, _)) = self.active.get() else {
            return;
        };
        if event_id(event).as_deref() != Some(op_id.as_str()) {
            return;
        }
        self.active.set(None);
        let error = parse_payload(event)
            .and_then(|p| p.get("error").and_then(|e| e.as_str()).map(str::to_string))
            .unwrap_or_default();
        ctx.show_toast(Toast::error(tr!(save_error(error = error))));
    }
}
