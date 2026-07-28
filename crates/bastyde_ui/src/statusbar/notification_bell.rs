// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `NotificationBell` — the status bar's bell, scoped to the Work this
//! window currently shows.
//!
//! `bastyde`'s `NotificationCenterButton::for_audience` bakes its scope in at
//! construction — it is a plain field, not itself signal-driven — so a
//! window that switches Work in place (File ▸ New Work / Open Work, the
//! ProjectSwitcher's "Open here") needs its bell rebuilt with the new scope,
//! not just its data re-rendered. This wrapper does exactly that: it binds
//! `AppIds.work_id` at `BindingLevel::Rebuild` and reconstructs the inner
//! `NotificationCenterButton` on every change, scoped via
//! `for_audience(ToastAudience::new(work_id))` — the same `work_id` →
//! `ToastAudience` mapping `App::build` uses for this window's `ToastHost`
//! (`ToastRegistry::set_window_audience`; see `crate::toast_scope`'s module
//! doc), so the bell and the toast corner always agree on which Work this
//! window is showing.
//!
//! Before any Work is open (`work_id` is `None` — the moment between a
//! window opening and its own bootstrap Load/New resolving), the bell is
//! left unscoped: it shows the whole shared archive, matching
//! `NotificationCenterButton`'s own documented "no scope set" behaviour —
//! never *hiding* a legitimate notification during that brief window.

use std::rc::Rc;

use bastyde::core::binding::BindingLevel;
use bastyde::prelude::*;
use bastyde::widgets::{
    IconButtonSize, NotificationArchiveModel, NotificationCenterButton, ToastAudience,
};

pub struct NotificationBell {
    archive: Rc<NotificationArchiveModel>,
    work_id: Signal<Option<u64>>,
    size: IconButtonSize,
    root_child: Option<WidgetId>,
}

impl NotificationBell {
    pub fn new(archive: Rc<NotificationArchiveModel>, work_id: Signal<Option<u64>>) -> Self {
        Self {
            archive,
            work_id,
            size: IconButtonSize::Toolbar,
            root_child: None,
        }
    }

    /// Bell-icon size — forwarded to the inner `NotificationCenterButton`.
    pub fn size(mut self, size: IconButtonSize) -> Self {
        self.size = size;
        self
    }
}

impl std::fmt::Debug for NotificationBell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotificationBell").finish()
    }
}

impl Widget for NotificationBell {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.work_id
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let mut bell = NotificationCenterButton::new(self.archive.clone()).size(self.size);
        if let Some(work_id) = self.work_id.get() {
            bell = bell.for_audience(ToastAudience::new(work_id));
        }
        let root = ctx.add(bell);
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(30.0, 30.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::styles::BannerSeverity;
    use bastyde::core::widget_tree::WidgetTree;
    use bastyde::presets::intui;
    use bastyde::widgets::{NotificationEntry, ToastPriority, ToastRoute};

    /// A minimal archived entry routed to `route` — mirrors bastyde's own
    /// `NotificationCenterButton` test helper (`entry_with_route`), not
    /// itself exported, so rebuilt here from `NotificationEntry`'s public
    /// fields.
    fn entry(title: &str, route: ToastRoute) -> NotificationEntry {
        NotificationEntry {
            id: 0,
            severity: BannerSeverity::Info,
            priority: ToastPriority::Normal,
            title: title.to_string(),
            body: None,
            actions: Vec::new(),
            timestamp: jiff::Timestamp::UNIX_EPOCH,
            group: None,
            source: None,
            read: false,
            dedup_id: None,
            updates: Vec::new(),
            route,
        }
    }

    /// Four entries: Work 1's own, two of Work 2's own, and one broadcast —
    /// enough to tell "scoped to 1" (1 + broadcast = 2), "scoped to 2" (2 +
    /// broadcast = 3), and "unscoped" (all 4) apart by badge count alone.
    fn seeded_archive() -> Rc<NotificationArchiveModel> {
        let archive = Rc::new(NotificationArchiveModel::in_memory());
        archive.push(entry(
            "for work 1",
            ToastRoute::Audience(ToastAudience::new(1)),
        ));
        archive.push(entry(
            "for work 2 (a)",
            ToastRoute::Audience(ToastAudience::new(2)),
        ));
        archive.push(entry(
            "for work 2 (b)",
            ToastRoute::Audience(ToastAudience::new(2)),
        ));
        archive.push(entry("everyone", ToastRoute::Broadcast));
        archive
    }

    /// With a Work open, the bell must apply `for_audience(work_id)` — proven
    /// through the ONE externally observable effect that scoping has: the
    /// badge counts only entries routed to that audience plus `Broadcast`,
    /// not the whole shared archive (`NotificationCenterButton` itself has no
    /// public getter for its own `route_scope`, so this is the only way to
    /// tell the branch was taken without reaching into a different crate's
    /// private field).
    #[test]
    fn with_a_work_open_the_bell_scopes_to_its_audience() {
        let archive = seeded_archive();
        let mut tree = WidgetTree::new().with_theme(intui::light());
        tree.add(NotificationBell::new(archive, Signal::new(Some(1))));
        tree.layout(SizeProposal::exact(120.0, 60.0));
        assert!(
            tree.find_by_label("2").is_some(),
            "Work 1's own entry + the broadcast one = 2 — proving for_audience(1) was \
             applied, not the whole shared archive's count of 4"
        );
    }

    /// With no Work open (the moment between a window opening and its own
    /// bootstrap Load/New resolving), the bell must fall back to unscoped —
    /// the whole shared archive, never hiding a legitimate notification.
    #[test]
    fn with_no_work_open_the_bell_falls_back_to_the_whole_archive() {
        let archive = seeded_archive();
        let mut tree = WidgetTree::new().with_theme(intui::light());
        tree.add(NotificationBell::new(archive, Signal::new(None)));
        tree.layout(SizeProposal::exact(120.0, 60.0));
        assert!(
            tree.find_by_label("4").is_some(),
            "no Work open ⇒ unscoped bell ⇒ every entry counts, matching \
             NotificationCenterButton's own documented 'no scope set' behaviour"
        );
    }

    /// The core F5 regression: an in-place project switch reseeds the SAME
    /// `AppIds.work_id` signal this bell is bound to (see the module doc) —
    /// the whole point of the `BindingLevel::Rebuild` binding is to
    /// reconstruct the inner `NotificationCenterButton` with the new scope.
    /// A refactor that dropped that binding, flipped the `if let Some(work_id)`
    /// branch, or changed the fallback would compile cleanly and leave the
    /// badge frozen at the old Work's count — exactly the silent regression
    /// this test exists to catch (it would only otherwise surface in manual
    /// QA, per the review finding this closes).
    #[test]
    fn the_bell_rebuilds_its_scope_when_work_id_changes() {
        let archive = seeded_archive();
        let work_id = Signal::new(Some(1));
        let mut tree = WidgetTree::new().with_theme(intui::light());
        tree.add(NotificationBell::new(archive, work_id.clone()));
        tree.layout(SizeProposal::exact(120.0, 60.0));
        assert!(
            tree.find_by_label("2").is_some(),
            "starts scoped to Work 1 (its own entry + broadcast = 2)"
        );

        work_id.set(Some(2));
        tree.layout(SizeProposal::exact(120.0, 60.0));
        assert!(
            tree.find_by_label("3").is_some(),
            "must now show Work 2's own two entries + broadcast = 3 — proving the bell \
             actually rebuilt with the new scope, not merely kept rendering the old one"
        );
        assert!(
            tree.find_by_label("2").is_none(),
            "the stale Work-1-scoped badge ('2') must be gone, not lingering alongside \
             the new one"
        );
    }
}
