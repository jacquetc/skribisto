// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Bind a live window to a Work in the registry — the shared half of Load,
//! New, and Attach seeding.
//!
//! Load and New used to each open-code register_window + ordinal write-back +
//! toast audience. Attach is the same bind with a reserved ordinal and without
//! a session `register` (attach already bumped the refcount). One function so
//! the three paths cannot drift.

use bastyde::prelude::{BastydeWindowId, Signal};
use bastyde::widgets::{ToastAudience, ToastRegistry};

use crate::sessions::{StackTeardown, WindowTeardown, WorkRegistry};

/// Register this window against `work_id` and bind its toast/bell audience.
///
/// `reserved_ordinal` is `Some` only for Work ▸ New Window (the ordinal was
/// reserved before the window existed so its persistence id could be derived).
/// Load/New pass `None` and accept whatever ordinal `register_window` assigns.
///
/// Returns the ordinal actually assigned (and writes it to `window_ordinal`).
pub(in crate::app) fn bind_window_to_work(
    registry: &WorkRegistry,
    window_id: BastydeWindowId,
    work_id: u64,
    reserved_ordinal: Option<usize>,
    stack_teardown: StackTeardown,
    window_teardown: WindowTeardown,
    window_ordinal: &Signal<usize>,
    toast_registry: &Option<ToastRegistry>,
) -> usize {
    let ordinal = registry.register_window(
        window_id,
        work_id,
        reserved_ordinal,
        stack_teardown,
        window_teardown,
    );
    window_ordinal.set(ordinal);
    if let Some(reg) = toast_registry {
        reg.set_window_audience(window_id, Some(ToastAudience::new(work_id)));
    }
    ordinal
}
