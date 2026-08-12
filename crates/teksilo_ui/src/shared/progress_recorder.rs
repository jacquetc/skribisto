// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ProgressRecorder` - the app-level cadence that keeps the `ProgressSnapshot`
//! history current, feeding the Book's Pace charts.
//!
//! It is *not* a view-model (no view); like `save::save_queue` it is a small
//! orchestration service wired once in `App::build`. On each `save_work` it fires
//! the read-only `count_words` long operation (throttled, and never overlapping
//! itself), and when that operation completes it records one `ProgressSnapshot`
//! for today - upserted by day, so many saves in a day just refresh today's row.
//!
//! Triggering only on `SaveWork` (never directly on `LoadWork`/`NewWork`) means
//! the recount always runs *after* the project's ids are seeded, so the
//! session-guard id it captures is correct; a brand-new project's baseline is
//! covered by the `save_to_disk` that `NewWork` issues, and a loaded project that
//! is merely read (never saved) correctly gets no new point - its historical
//! snapshots are already in the store from `load_work`.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

use chrono::Utc;

use frontend::AppContext;
use frontend::commands::progress_management_commands;
use frontend::common::event::Event;
use frontend::progress_management::{CountWordsDto, RecordProgressSnapshotDto};

use crate::app_ids::AppIds;

use super::long_op::event_id;

/// Don't recount more than once per minute on the save path - autosave fires
/// every few seconds, and today's snapshot only needs to be roughly current.
const THROTTLE: Duration = Duration::from_secs(60);

struct Inner {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    /// The in-flight `count_words` operation id, if one is running.
    active: RefCell<Option<String>>,
    /// The `WorkInfo` id captured when the count was fired - the completion only
    /// records if the same project is still open (a close/switch mid-count must
    /// not land project A's total on project B).
    fired_for: Cell<Option<u64>>,
    /// When the last recount was fired, for the save-path throttle.
    last_fire: Cell<Option<Instant>>,
}

/// Cloneable handle; one instance is shared app-wide via `app_state`.
#[derive(Clone)]
pub struct ProgressRecorder {
    inner: Rc<Inner>,
}

impl ProgressRecorder {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        Self {
            inner: Rc::new(Inner {
                app_ctx,
                ids,
                active: RefCell::new(None),
                fired_for: Cell::new(None),
                last_fire: Cell::new(None),
            }),
        }
    }

    /// Recount now, unconditionally - for an explicit "refresh" affordance
    /// (wired by the Pace pane in M4c).
    #[allow(dead_code)]
    pub fn recount(&self) {
        self.fire();
    }

    /// Recount on a save, unless one is already running or we recounted within
    /// the last [`THROTTLE`].
    pub fn recount_throttled(&self) {
        if self.inner.active.borrow().is_some() {
            return;
        }
        if let Some(last) = self.inner.last_fire.get()
            && last.elapsed() < THROTTLE
        {
            return;
        }
        self.fire();
    }

    fn fire(&self) {
        // No open project → nothing to count (also keeps the mock build, which
        // never seeds a real Work, inert).
        let Some(work_info_id) = self.inner.ids.work_info_id.get() else {
            return;
        };
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return;
        };
        let dto = CountWordsDto { work_id };
        if let Ok(op_id) = progress_management_commands::count_words(&self.inner.app_ctx, &dto) {
            *self.inner.active.borrow_mut() = Some(op_id);
            self.inner.fired_for.set(Some(work_info_id));
            self.inner.last_fire.set(Some(Instant::now()));
        }
    }

    /// A long operation completed - if it is our in-flight `count_words`, record
    /// today's snapshot. Ignores every other long op (save / import / export /
    /// backup) by matching the operation id.
    pub fn on_completed(&self, event: &Event) {
        let Some(op_id) = self.take_if_ours(event) else {
            return;
        };
        // Session guard: only record while the same project is still open.
        let fired_for = self.inner.fired_for.get();
        if fired_for.is_none() || self.inner.ids.work_info_id.get() != fired_for {
            return;
        }
        // The guard above already proves the same project (by WorkInfo) is
        // still open, so its Work id is fetched fresh here rather than
        // threaded through `fired_for` — the two ids are seeded together.
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return;
        };
        let Ok(Some(res)) =
            progress_management_commands::get_count_words_result(&self.inner.app_ctx, &op_id)
        else {
            return;
        };
        let day = Utc::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .expect("midnight is valid")
            .and_utc();
        let dto = RecordProgressSnapshotDto {
            work_id,
            day,
            total_word_count: res.total_word_count,
            total_char_count: res.total_char_count,
            book_item_ids: res.book_item_ids,
            book_word_counts: res.book_word_counts,
        };
        let _ = progress_management_commands::record_progress_snapshot(&self.inner.app_ctx, &dto);
    }

    /// A long operation failed or was cancelled - clear our in-flight marker if
    /// it was ours, so the next save can recount again.
    pub fn on_failed_or_cancelled(&self, event: &Event) {
        let _ = self.take_if_ours(event);
    }

    /// If `event` is our in-flight operation's completion, clear the marker and
    /// return its id; otherwise leave the marker and return `None`.
    fn take_if_ours(&self, event: &Event) -> Option<String> {
        let ours = {
            let active = self.inner.active.borrow();
            match active.as_deref() {
                Some(id) => event_id(event).as_deref() == Some(id),
                None => false,
            }
        };
        if ours {
            self.inner.active.borrow_mut().take()
        } else {
            None
        }
    }
}
