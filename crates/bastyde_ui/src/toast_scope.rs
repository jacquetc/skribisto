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
//!
//! **Routing is only half of Work-scoping.** A call site that also passes a
//! static [`Toast::id`] (for the framework's update-in-place dedup — a
//! progress toast, a "replace, don't stack" burst guard) must fold `work_id`
//! into THAT id too, via [`work_scoped_toast_id`], or two Works sharing one
//! static id collide in the registry regardless of how correctly their
//! routes are set — see that function's own doc for the exact failure mode.

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

/// Fold `work_id` into a static `Toast::id(...)` base, the dedup-id half of
/// Work-scoping — [`ToastWorkExt::target_work`] is the *routing* half.
///
/// `ToastRegistry::enqueue` (bastyde) finds an update-in-place match by dedup
/// id **alone** — no route check — and then overwrites the matched entry's
/// route with the new toast's. That is deliberate upstream (a progress toast
/// whose audience becomes known partway through must retarget in place; see
/// `Toast::id`'s own doc for the framework's reasoning and the exact hazard
/// this creates). It means a *static* id shared by every window is an
/// app-level bug the moment two windows can raise "the same" toast about two
/// different Works at once: window B's enqueue finds window A's still-live
/// entry (same id), mutates it in place, and retargets it to B's audience —
/// A's toast silently vanishes, mis-attributed to B. This is exactly
/// [`ToastWorkExt::target_work`]'s own scenario, one layer up: the id needs
/// the same per-Work identity the route already gets.
///
/// Every call site that pairs a static id with `.target_work(...)` should
/// route the id through here instead of using the base string directly —
/// `format!("{base}.{}", work_id.unwrap_or_default())`, so `"export.work"`
/// for Work 7 and Work 9 become distinct `"export.work.7"` / `"export.work.9"`
/// entries in the registry, never colliding. `None` (no Work open, or a call
/// site that legitimately has none to give) collapses to the same `.0`
/// suffix for every such toast — an accepted, pre-existing simplification
/// (mirrors `Option::unwrap_or_default`'s own precedent in
/// `BackupSchedulerViewModel::toast_id`, which this function replaces): the
/// call sites that ever pass `None` here already have no Work to distinguish
/// by, so there is nothing more specific to fold in.
///
/// **Do not use this for a `.broadcast()` toast.** An app-wide toast SHOULD
/// dedup across every window — e.g. "downloading a dictionary…" showing
/// twice because two windows are both waiting on the same download would be
/// the bug, not the fix — so broadcast call sites (`dictionaries.rs`,
/// `import_plume.rs`) keep their bare static id.
pub fn work_scoped_toast_id(base: &str, work_id: Option<u64>) -> String {
    format!("{base}.{}", work_id.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_works_never_share_a_scoped_toast_id() {
        assert_ne!(
            work_scoped_toast_id("export.work", Some(1)),
            work_scoped_toast_id("export.work", Some(2)),
            "two different Works raising the same logical toast must never collide \
             in the shared ToastRegistry"
        );
    }

    #[test]
    fn the_same_work_always_gets_the_same_scoped_toast_id() {
        assert_eq!(
            work_scoped_toast_id("export.work", Some(7)),
            work_scoped_toast_id("export.work", Some(7)),
            "the update-in-place merge (a progress tick replacing the previous \
             percentage) needs a *stable* id across calls for one Work"
        );
    }

    #[test]
    fn different_bases_never_collide_for_the_same_work() {
        assert_ne!(
            work_scoped_toast_id("export.work", Some(1)),
            work_scoped_toast_id("backup.now", Some(1)),
            "two unrelated operations on the same Work must not share a dedup id \
             just because they share a work_id"
        );
    }
}
