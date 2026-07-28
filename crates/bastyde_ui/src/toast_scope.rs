// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Route a toast to the Work it is about.
//!
//! bastyde's window-scoped toast routing (see the framework's
//! `Toast::target`/`ToastAudience`) needs an app-chosen `u64` token per
//! audience. Skribisto's natural audience is the open `Work`: two windows
//! showing two different Works must never leak a backup/export/save toast
//! into each other's corner, and a Work's own bell must show only its own
//! history plus anything genuinely app-wide. The `Work`'s id already
//! uniquely and durably names it (see `app_ids::AppIds`), so it doubles as
//! the [`ToastAudience`] token with nothing new to mint or keep in sync —
//! `App::build`'s `LoadWork`/`NewWork` handlers call
//! `ToastRegistry::set_window_audience(window_id, Some(ToastAudience::new(work_id)))`
//! with the very same id every per-Work view-model uses here.
//!
//! Every per-Work view-model already resolves its own `AppIds::work_id`
//! before raising a toast — the "no open project" guards that predate this
//! module (e.g. `TrashViewModel::restore`'s early return) read it for
//! exactly that reason. [`ToastWorkExt::target_work`] is the one-line bridge
//! from that `Option<u64>` to the framework's routing, so a Work-scoped
//! call site reads the same way everywhere: `.target_work(self.ids.work_id.get())`.
//!
//! `None` (no Work open yet, or a call site that legitimately has none to
//! give — the Launcher, a New Work failure before any Work exists) leaves
//! the toast at the framework's own default: the presenting window only.
//! That is the correct fallback, not a degraded case — those toasts were
//! never "about" a Work in the first place.

use bastyde::widgets::{Toast, ToastAudience};

/// Extension on [`Toast`] used by every Work-scoped call site instead of the
/// bare `.target()`, so "which Work does this toast concern" reads the same
/// way everywhere.
pub trait ToastWorkExt {
    /// Route to `work_id`'s audience when one is known; otherwise leave the
    /// toast's target unset (framework default: the presenting window).
    fn target_work(self, work_id: Option<u64>) -> Self;
}

impl ToastWorkExt for Toast {
    fn target_work(self, work_id: Option<u64>) -> Self {
        match work_id {
            Some(id) => self.target(ToastAudience::new(id)),
            None => self,
        }
    }
}
