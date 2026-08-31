// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Route a toast to the Work it is about.
//!
//! teksilo's window-scoped toast routing (`Toast::target`/`ToastAudience`)
//! needs an app-chosen `u64` token per audience. Skribisto's natural audience
//! is the open `Work`: two windows on two different Works must never leak a
//! backup/export/save toast into each other's corner. The `Work`'s id already
//! uniquely names it, so it doubles as the [`ToastAudience`] token —
//! `App::build`'s `LoadWork`/`NewWork` handlers call
//! `ToastRegistry::set_window_audience(window_id, Some(ToastAudience::new(work_id)))`
//! with the same id every per-Work view-model uses here via
//! [`ToastWorkExt::target_work`]: `.target_work(self.ids.work_id.get())`.
//!
//! `None` (no Work open, or a call site with none to give) leaves the toast
//! at the framework's default: the presenting window only.
//!
//! **Routing is only half of Work-scoping.** A call site that also passes a
//! static [`Toast::id`] (for update-in-place dedup — a progress toast, a
//! burst guard) must fold `work_id` into THAT id too, via
//! [`ToastWorkExt::scoped_id`], or two Works sharing one static id collide in
//! the registry regardless of how correctly their routes are set — see
//! [`work_scoped_toast_id`]'s doc for the `None` case specifically (F2).

use teksilo::widgets::{Toast, ToastAudience};

/// Extension on [`Toast`] used by every Work-scoped call site instead of the
/// bare `.target()`/`.id()`, so "which Work does this toast concern" reads
/// the same way everywhere.
pub trait ToastWorkExt {
    /// Route to `work_id`'s audience when one is known; otherwise leave the
    /// toast's target unset (framework default: the presenting window).
    ///
    /// Generic over `impl Into<Option<u64>>` so a long-operation view-model's
    /// `view_models::long_op::CapturedWork` plugs straight in with no
    /// `.into()` at the call site.
    fn target_work(self, work_id: impl Into<Option<u64>>) -> Self;

    /// Fold `work_id` into a static dedup id (`base`, e.g. `"backup.now"`)
    /// and attach it — or don't, when `work_id` is `None`. The only
    /// sanctioned way to pair a static id with `.target_work(...)`; see
    /// [`work_scoped_toast_id`] for why `None` must skip the id entirely (F2).
    fn scoped_id(self, base: &str, work_id: impl Into<Option<u64>>) -> Self;

    /// [`Self::scoped_id`] for a toast that follows **one long operation**
    /// from start to finish: the op's own id joins `work_id` in the dedup
    /// key, so two windows on the same Work (Work ▸ New Window) can each run
    /// one without the second's enqueue retargeting/killing the first's
    /// still-live progress toast. A caller with no operation at all (nothing
    /// was ever started) keeps [`Self::scoped_id`].
    fn scoped_op_id(self, base: &str, work_id: impl Into<Option<u64>>, op_id: &str) -> Self;
}

impl ToastWorkExt for Toast {
    fn target_work(self, work_id: impl Into<Option<u64>>) -> Self {
        match work_id.into() {
            Some(id) => self.target(ToastAudience::new(id)),
            None => self,
        }
    }

    fn scoped_id(self, base: &str, work_id: impl Into<Option<u64>>) -> Self {
        match work_scoped_toast_id(base, work_id) {
            Some(id) => self.id(id),
            None => self,
        }
    }

    fn scoped_op_id(self, base: &str, work_id: impl Into<Option<u64>>, op_id: &str) -> Self {
        // Unlike `scoped_id`, a `None` Work is not a reason to drop the id: the
        // op id alone already identifies exactly one operation, which is the
        // whole thing this toast tracks. F2's hazard (every window with no Work
        // open sharing one id) cannot arise from a key no two operations share.
        match work_scoped_toast_id(base, work_id) {
            Some(id) => self.id(format!("{id}.{op_id}")),
            None => self.id(format!("{base}.{op_id}")),
        }
    }
}

/// Fold `work_id` into a static `Toast::id(...)` base — the dedup-id half of
/// Work-scoping. `ToastRegistry::enqueue` matches an update-in-place toast by
/// dedup id alone, no route check, and overwrites the match's route with the
/// new toast's — so a static id shared by every window lets window B's
/// enqueue steal window A's still-live entry the moment two windows raise
/// "the same" toast about two different Works. Folding `work_id` in
/// (`"export.work.7"` vs `"export.work.9"`) fixes that the same way
/// [`ToastWorkExt::target_work`] fixes routing.
///
/// # F2 — `None` returns `None`, not a collidable id
///
/// `None` (no Work open yet) must NOT collapse to a fixed `"{base}.0"`
/// suffix — that was itself a real bug: two project windows opened in quick
/// succession, both with no `work_id` resolved yet, would build the same
/// `"backup.now.0"` and the second's enqueue would steal the first's toast.
/// Returning `Option<String>` (not `String`) makes the `None` case something
/// a caller must handle rather than silently defaulting away — only
/// [`ToastWorkExt::scoped_id`] unwraps it, and correctly skips `.id(...)`
/// entirely when it's `None`, leaving no dedup id at all (which never merges
/// with anything, the correct behaviour when there is no Work to dedup
/// against).
///
/// **Do not use this for a `.broadcast()` toast** — an app-wide toast SHOULD
/// dedup across every window, so broadcast call sites (`dictionaries.rs`,
/// `import_plume.rs`) keep their bare static id.
pub fn work_scoped_toast_id(base: &str, work_id: impl Into<Option<u64>>) -> Option<String> {
    work_id.into().map(|id| format!("{base}.{id}"))
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

    // ── F2: the `None` case must never collide ─────────────────────────────

    #[test]
    fn no_work_open_yields_no_dedup_id_at_all() {
        assert_eq!(
            work_scoped_toast_id("backup.now", None),
            None,
            "F2: with no Work open there is no operation identity to dedup \
             against — this must NOT collapse to a fixed \"{{base}}.0\" id that \
             every no-Work window would share"
        );
    }

    /// The pure check above doesn't touch a real `.scoped_id(...)` call site,
    /// so this drives two toasts through a real `ToastRegistry` via a wired
    /// `Button` + [`crate::test_support::click`] and asserts both stay live —
    /// two windows with no Work open must never let the second one's toast
    /// steal the first's slot.
    #[test]
    fn two_no_work_toasts_via_scoped_id_both_stay_live_in_a_real_registry() {
        use teksilo::i18n::lit;
        use teksilo::prelude::SizeProposal;
        use teksilo::widgets::{Button, EventContextToastExt, ToastInstallOptions, ToastRegistry};

        let app_ctx = std::rc::Rc::new(frontend::AppContext::new());
        let registry = ToastRegistry::new(ToastInstallOptions {
            archive: None,
            ..ToastInstallOptions::default()
        });
        let mut tree = crate::test_support::tree_with_toast_registry(&app_ctx, &registry);

        let btn_a = tree.add(Button::new(lit!("a")).on_activate_fn(move |ctx| {
            ctx.show_toast(
                Toast::warning(lit!("nothing open")).scoped_id("backup.now", None::<u64>),
            );
        }));
        let btn_b = tree.add(Button::new(lit!("b")).on_activate_fn(move |ctx| {
            ctx.show_toast(
                Toast::warning(lit!("nothing open")).scoped_id("backup.now", None::<u64>),
            );
        }));
        tree.layout(SizeProposal::exact(200.0, 80.0));

        crate::test_support::click(&mut tree, btn_a);
        crate::test_support::click(&mut tree, btn_b);

        assert_eq!(
            registry.live_count(),
            2,
            "F2: two windows with no Work open must both keep their own \"nothing \
             open\" toast — a shared \"backup.now.0\" id would let window B's \
             enqueue find window A's still-live entry and steal it"
        );
    }
}
