// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Notifications — the update check, and the toast archive manager.
//!
//! Embeds Teksilo's [`NotificationLog`] (the same day-bucketed list the status
//! bar's bell popover shows) as a full settings page: mark-all-read, clear, and
//! replaying archived actions that carry an `intent_name`. The archive is the
//! process-wide one registered by `install_toast_default()` — unscoped, so the
//! writer sees every toast that has fired in this session (and, with persistent
//! archival, earlier ones too), not only the Work the opening window happens
//! to be showing.
//!
//! ## Why the update toggle lives here
//!
//! It is the one preference that governs whether the application tells the
//! writer something unprompted, which is what this page is about. It is
//! deliberately not on Appearance & Behaviour beside "show the Launcher at
//! startup": that row is about what the application does with its own windows,
//! this one is about whether it speaks at all.
//!
//! The row is **absent**, not disabled, on a channel that never checks. A
//! Flathub install is kept current by the software centre, so a switch there
//! would claim an effect it does not have; the explanation belongs in About,
//! next to the version, where a reader wondering about their version is already
//! looking.

use std::rc::Rc;

use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{Expand, NotificationArchiveModel, NotificationLog};

#[allow(unused_imports)]
use super::super::*;

/// Settings ▸ Notifications — the update-check preference above the archive log,
/// or an empty placeholder when the toast subsystem was never installed
/// (headless / off-screen builds).
pub(in crate::settings) fn notifications_pane(
    ctx: &mut BuildContext,
    crumbs: &Crumbs,
    vm: &SettingsViewModel,
) -> Box<dyn Widget> {
    let updates = crate::updates::view_model();
    let shows_updates = updates.shows_update_state();

    // Turning the check off has to clear what the last one found, not merely
    // stop the next one: a reader who says "stop telling me" and goes on being
    // told has not been listened to. `forget_if_disabled` is idempotent and
    // cheap, so reacting to the signal is enough; there is no separate
    // "changed" path to keep in step.
    if shows_updates {
        let enabled = vm.check_for_updates();
        let vm_for_effect = updates.clone();
        ctx.effect(&enabled, move |on| vm_for_effect.forget_if_disabled(*on));
    }

    let toggle_row = shows_updates.then(|| {
        FormLayout::new()
            .label(tr!(settings_page_notifications()))
            .label_gap(16.0)
            .row_spacing(14.0)
            .full_width(group(tr!(settings_group_updates())))
            .line(
                field_label(tr!(settings_check_for_updates())),
                Toggle::new(vm.check_for_updates())
                    .labelled_externally()
                    .rich_tooltip_content(TooltipContent::new(
                        "settings.check_for_updates",
                        tr!(settings_check_for_updates_tip()),
                    )),
            )
    });

    match ctx.app_state::<Rc<NotificationArchiveModel>>().cloned() {
        Some(archive) => {
            // Unscoped: this is the settings-level manager, not a per-window
            // bell. Action replay (`on_action_invoked`) is left unwired for now
            // — Skribisto's toasts use live closures rather than
            // `ToastAction::shortcut_id`, so archived actions already render as
            // inert past-action tags (same as the status-bar bell). When a toast
            // starts carrying a static intent name, wire the known ones here.
            let log = NotificationLog::new(archive);
            // The log takes whatever the pane has left, floored by the shared
            // list floor. It used to declare 360 px of its own — 84% of the
            // viewport, so the page scrolled around a log that was itself
            // scrolling. The toggle sits above it and takes its natural height,
            // so the log keeps the rest.
            // `list_box` is ALREADY an `Expand::vertical().respect_intrinsic()`,
            // so it goes into the column as-is. Wrapping it in a second, plain
            // `Expand::vertical` is what the sibling panes warn about: a bare
            // `Expand` reports 0 on its flex axis during intrinsic measurement,
            // and inside the settings scroll that starves everything above it —
            // the toggle row rendered at zero height and simply was not there.
            let log_box = crate::settings::fields::list_box(Expand::horizontal().child(log));
            let mut column = VStack::new().spacing(16.0);
            if let Some(row) = toggle_row {
                column = column.child(row);
            }
            Box::new(pane_frame(
                crumbs.of(Pane::Notifications),
                column.child(log_box),
            ))
        }
        None => Box::new(empty_pane(
            crumbs,
            Pane::Notifications,
            Sec::AppearanceBehaviour.icon_svg(),
        )),
    }
}
