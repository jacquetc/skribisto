// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Notifications — the toast archive manager.
//!
//! Embeds Teksilo's [`NotificationLog`] (the same day-bucketed list the status
//! bar's bell popover shows) as a full settings page: mark-all-read, clear, and
//! replaying archived actions that carry an `intent_name`. The archive is the
//! process-wide one registered by `install_toast_default()` — unscoped, so the
//! writer sees every toast that has fired in this session (and, with persistent
//! archival, earlier ones too), not only the Work the opening window happens
//! to be showing.

use std::rc::Rc;

use teksilo::widgets::{Expand, NotificationArchiveModel, NotificationLog};

#[allow(unused_imports)]
use super::super::*;

/// Settings ▸ Notifications — the archive log, or an empty placeholder when
/// the toast subsystem was never installed (headless / off-screen builds).
pub(in crate::settings) fn notifications_pane(
    ctx: &mut BuildContext,
    crumbs: &Crumbs,
) -> Box<dyn Widget> {
    match ctx.app_state::<Rc<NotificationArchiveModel>>().cloned() {
        Some(archive) => {
            // Unscoped: this is the settings-level manager, not a per-window
            // bell. Action replay (`on_action_invoked`) is left unwired for now
            // — Skribisto's toasts use live closures rather than
            // `ToastAction::shortcut_id`, so archived actions already render as
            // inert past-action tags (same as the status-bar bell). When a toast
            // starts carrying a static intent name, wire the known ones here.
            let log = NotificationLog::new(archive);
            Box::new(pane_frame(
                crumbs.of(Pane::Notifications),
                // The log takes whatever the pane has left, floored by the shared
                // list floor. It used to declare 360 px of its own — 84% of the
                // viewport, so the page scrolled around a log that was itself
                // scrolling.
                crate::settings::fields::list_box(
                    Expand::horizontal().child(Expand::vertical().child(log)),
                ),
            ))
        }
        None => Box::new(empty_pane(
            crumbs,
            Pane::Notifications,
            Sec::AppearanceBehaviour.icon_svg(),
        )),
    }
}
