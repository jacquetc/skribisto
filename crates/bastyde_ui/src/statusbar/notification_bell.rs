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
