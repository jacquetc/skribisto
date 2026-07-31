// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Guarded Work-management event subscriptions.
//!
//! Multi-window means every window hears every `LoadWork`/`NewWork`/`CloseWork`.
//! Call sites used to hand-copy the same `is_bootstrap_or_own` /
//! `is_event_for_my_work` filter — fifteen near-identical closures. These helpers
//! are the one place that filter lives.

use bastyde::prelude::*;

use frontend::common::event::{Event, Origin, WorkManagementEvent};

use crate::app_ids::AppIds;

/// Subscribe `f` to both `LoadWork` and `NewWork`, running only when the event
/// is this window's own bootstrap or in-place switch (see
/// [`AppIds::is_bootstrap_or_own`]).
pub(in crate::app) fn on_own_load_or_new(
    ctx: &mut BuildContext,
    ids: &AppIds,
    f: impl Fn(&Event) + Clone + 'static,
) {
    for event in [
        WorkManagementEvent::LoadWork,
        WorkManagementEvent::NewWork,
    ] {
        let ids = ids.clone();
        let f = f.clone();
        ctx.subscribe_event(Origin::WorkManagement(event), move |e: &Event| {
            if ids.is_bootstrap_or_own(&e.ids) {
                f(e);
            }
        });
    }
}

/// Subscribe `f` to `CloseWork`, running only when the event names this
/// window's live Work (see [`AppIds::is_event_for_my_work`]).
pub(in crate::app) fn on_own_close(
    ctx: &mut BuildContext,
    ids: &AppIds,
    f: impl Fn(&Event) + 'static,
) {
    let ids = ids.clone();
    ctx.subscribe_event(
        Origin::WorkManagement(WorkManagementEvent::CloseWork),
        move |e: &Event| {
            if ids.is_event_for_my_work(&e.ids) {
                f(e);
            }
        },
    );
}

/// Subscribe `f` to `LoadWork`/`NewWork` with an [`EventContext`] (modals, toasts).
pub(in crate::app) fn on_own_load_or_new_with_ctx(
    ctx: &mut BuildContext,
    ids: &AppIds,
    f: impl Fn(&Event, &mut EventContext) + Clone + 'static,
) {
    for event in [
        WorkManagementEvent::LoadWork,
        WorkManagementEvent::NewWork,
    ] {
        let ids = ids.clone();
        let f = f.clone();
        ctx.subscribe_event_with_ctx(Origin::WorkManagement(event), move |e: &Event, c| {
            if ids.is_bootstrap_or_own(&e.ids) {
                f(e, c);
            }
        });
    }
}
