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
//! into THAT id too, via [`ToastWorkExt::scoped_id`], or two Works sharing one
//! static id collide in the registry regardless of how correctly their
//! routes are set — see that method's own doc for the exact failure mode,
//! and [`work_scoped_toast_id`]'s doc for the `None` case specifically (F2).

use bastyde::widgets::{Toast, ToastAudience};

/// Extension on [`Toast`] used by every Work-scoped call site instead of the
/// bare `.target()`/`.id()`, so "which Work does this toast concern" reads
/// the same way everywhere.
pub trait ToastWorkExt {
    /// Route to `work_id`'s audience when one is known; otherwise leave the
    /// toast's target unset (framework default: the presenting window).
    ///
    /// Generic over `impl Into<Option<u64>>` (not just a bare `Option<u64>`)
    /// so a long-operation view-model's `view_models::long_op::CapturedWork`
    /// — the Work an operation started for, captured once and never re-read
    /// live — plugs straight in here with no `.into()` at the call site.
    fn target_work(self, work_id: impl Into<Option<u64>>) -> Self;

    /// Fold `work_id` into a static dedup id (`base`, e.g. `"backup.now"`)
    /// and attach it — **or don't**, when `work_id` is `None` — this is the
    /// *only* sanctioned way to pair a static id with `.target_work(...)`.
    ///
    /// See [`work_scoped_toast_id`] for the full "why `None` skips the id
    /// entirely" rationale (F2). This method exists so a call site can never
    /// reproduce that bug by hand: there is no `.id(work_scoped_toast_id(...))`
    /// spelling left to get wrong, because `work_scoped_toast_id` itself
    /// returns `Option<String>` — not a `String` that quietly degrades to a
    /// collidable `"{base}.0"` — and this is the one place that unwraps it,
    /// correctly, every time.
    fn scoped_id(self, base: &str, work_id: impl Into<Option<u64>>) -> Self;

    /// [`Self::scoped_id`] for a toast that follows **one long operation** from
    /// start to finish (loading → progress → success/cancelled/error): the op's
    /// own id joins `work_id` in the dedup key.
    ///
    /// Work-scoping alone was enough while one Work meant one window, because
    /// the view-models that drive these toasts (`ExportViewModel`,
    /// `SaveAsViewModel`) are per **window** and each allows one operation at a
    /// time — so "this Work's export" and "this window's export" were the same
    /// thing. Work ▸ New Window separates them: two windows on one Work can each
    /// start an export, and with only the Work in the key the second one's
    /// enqueue finds the first's still-live entry, overwrites it in place and
    /// retargets it — the first export's progress bar and its Cancel button
    /// vanish mid-flight while the operation itself carries on running,
    /// uncancellable.
    ///
    /// The op id is unique per operation, so it distinguishes the two without
    /// the view-model needing to know anything about windows — and, being the
    /// *same* string for every update of one operation, it keeps the
    /// update-in-place behaviour those toasts depend on.
    ///
    /// A caller with no operation (nothing was ever started — "no project is
    /// open to export") has no op id and keeps [`Self::scoped_id`]: there is no
    /// operation for a second toast to be confused with.
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

/// Fold `work_id` into a static `Toast::id(...)` base, the dedup-id half of
/// Work-scoping — [`ToastWorkExt::target_work`] is the *routing* half,
/// [`ToastWorkExt::scoped_id`] is the sanctioned way to apply this to a
/// `Toast` under construction.
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
/// the same per-Work identity the route already gets. So for a known
/// `work_id`, this folds it in: `"export.work"` for Work 7 and Work 9 become
/// distinct `"export.work.7"` / `"export.work.9"` entries in the registry,
/// never colliding.
///
/// # F2 — `None` returns `None`, not a collidable id
///
/// The first cut of this function collapsed `None` (no Work open yet, or a
/// call site with no Work to give) to a fixed `"{base}.0"` suffix
/// (`work_id.unwrap_or_default()`). That was itself the bug F2 found: **every
/// window with no Work open shares that one id.** Two project windows opened
/// in quick succession both have `backup.now`'s "nothing open"/"already
/// running" toasts live *before* their own `LoadWork`/`NewWork` resolves a
/// `work_id` — both build the same `"backup.now.0"`, and the second window's
/// enqueue finds the first's still-live entry and steals it, exactly the
/// [`ToastWorkExt::target_work`] hazard this function exists to prevent, just
/// smuggled back in through the one case that looked "safe" to default.
///
/// The honest fix: **there is no Work to dedup against, so don't invent an
/// id that pretends there is.** Returning `None` here — and having
/// [`ToastWorkExt::scoped_id`] skip the `.id(...)` call entirely for it —
/// leaves such a toast with no dedup id at all, which is `Toast`'s own
/// default for a toast that was never given one: it never merges with
/// anything (see `Toast::id`'s doc), which is the correct behaviour for two
/// windows independently discovering "no Work is open" — those are two
/// unrelated, simultaneously true facts about two different windows, not one
/// operation that a second call should update in place. A *stale* progress
/// toast for a Work that has since closed is the one case this could, in
/// principle, leave un-deduped against a fresh one for the same still-`None`
/// state — accepted, because with no Work there is no operation identity
/// for two calls to even conceptually be "the same" logical toast.
///
/// Returning `Option<String>` (rather than `String`) is deliberate too: it
/// makes the `None` case a type-level fact a caller has to handle, not a
/// value a caller can silently misuse by feeding it straight into `.id(...)`
/// — that misuse (`.id(work_scoped_toast_id(base, work_id))`, unwrapping the
/// old `unwrap_or_default()` fallback back in) is exactly what produced F2.
/// Every call site in this crate goes through [`ToastWorkExt::scoped_id`]
/// instead, which is the only place that unwraps this `Option`.
///
/// **Do not use this for a `.broadcast()` toast.** An app-wide toast SHOULD
/// dedup across every window — e.g. "downloading a dictionary…" showing
/// twice because two windows are both waiting on the same download would be
/// the bug, not the fix — so broadcast call sites (`dictionaries.rs`,
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

    /// The pure check above only proves the helper itself no longer invents a
    /// collidable id — it never touches an actual `.id(...)`/`.scoped_id(...)`
    /// call site, so a reverted call site (back to
    /// `.id(work_scoped_toast_id(base, work_id).unwrap_or_default())`, say)
    /// would still leave it green. This drives two toasts through a real
    /// `ToastRegistry` via a wired `Button` + [`crate::test_support::click`]
    /// (the same idiom every other Phase-3 regression test in this crate
    /// uses), both built the way `BackupSchedulerViewModel`'s "nothing open"
    /// branch does — `.scoped_id("backup.now", None::<u64>)` — and asserts
    /// both stay live: two windows with no Work open yet must never let the
    /// second one's "no Work open" toast steal the first's slot.
    #[test]
    fn two_no_work_toasts_via_scoped_id_both_stay_live_in_a_real_registry() {
        use bastyde::i18n::lit;
        use bastyde::prelude::SizeProposal;
        use bastyde::widgets::{Button, EventContextToastExt, ToastInstallOptions, ToastRegistry};

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
