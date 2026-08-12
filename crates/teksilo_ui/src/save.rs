// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Saving: the one `.skrib` write path every open window shares.
//!
//! [`SaveStateViewModel`] owns the serialized, sequence-tracked save state
//! (`dirty_seq`/`saved_seq`/`saving` and the [`save_queue`]'s coalescing) —
//! **Work**-scoped, not per-window, since a `Work` can span several windows
//! but the save op is global (one `.skrib`, at most one `save_work` at a
//! time). [`SaveAsViewModel`] is Save As / the zip↔folder switch, reusing the
//! same atomic-replace machinery backups restore through.
//! [`save_status`](mod@save_status) is the status-bar glyph's pure decision table (dot =
//! unsaved, check = on disk, spinner = a slow write past 200 ms); a **failed**
//! save is a toast instead, always naming the deferred close/switch it
//! dropped. [`timers`] is the pure countdown policy behind the debounced
//! autosave.

mod save_as_vm;
mod save_queue;
mod save_state_vm;
mod save_status;
mod timers;

pub use save_as_vm::SaveAsViewModel;
pub(crate) use save_queue::{DeferredResume, resume_deferred};
pub use save_state_vm::{SaveLanded, SaveStateViewModel, WorkHandle};
pub use save_status::{SaveStatus, SpinnerGate, save_clickable, save_status};
pub(crate) use timers::{AutosaveCountdown, IntervalCountdown, IntervalTick};
