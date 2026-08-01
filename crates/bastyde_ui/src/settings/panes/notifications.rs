// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Notifications — the toast archive manager.
//!
//! Embeds Bastyde's [`NotificationLog`] (the same day-bucketed list the status
//! bar's bell popover shows) as a full settings page: mark-all-read, clear, and
//! replaying archived actions that carry an `intent_name`. The archive is the
//! process-wide one registered by `install_toast_default()` — unscoped, so the
//! writer sees every toast that has fired in this session (and, with persistent
//! archival, earlier ones too), not only the Work the opening window happens
//! to be showing.

use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::{Expand, MinSize, NotificationArchiveModel, NotificationLog};

#[allow(unused_imports)]
use super::super::*;

/// Floor height for the log so its internal `ScrollArea` has a bounded slot
/// to fill inside the settings pane's own scroll (same trap `user_dictionary`
/// documents for a virtualised list).
const LOG_MIN_HEIGHT: f32 = 360.0;

/// Settings ▸ Notifications — the archive log, or an empty placeholder when
/// the toast subsystem was never installed (headless / off-screen builds).
pub(in crate::settings) fn notifications_pane(ctx: &mut BuildContext) -> Box<dyn Widget> {
    let ab = tr!(settings_sec_appearance_behaviour());
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
                crumb(Some(ab), tr!(settings_page_notifications())),
                Expand::horizontal().child(
                    MinSize::new(0.0, LOG_MIN_HEIGHT).child(Expand::vertical().child(log)),
                ),
            ))
        }
        None => Box::new(empty_pane(
            Some(ab),
            tr!(settings_page_notifications()),
            Sec::AppearanceBehaviour.icon_svg(),
        )),
    }
}
