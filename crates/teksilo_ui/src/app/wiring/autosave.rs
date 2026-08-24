// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Frame-tick-driven wiring grouped by shape, not feature: a settings-mirror
//! toggle, a debounced dirty→disk timer and a periodic backup timer, all three
//! built on the same `wake_at`/`frame_tick` idiom and none of them owning any
//! state beyond what `App::build` hands in through [`AutosaveDeps`].
//!
//! Extracted from `App::build` so that god-function is not also the countdown
//! engine.

use std::rc::Rc;

use teksilo::prelude::*;

use frontend::AppContext;
use frontend::common::event::{Event, Origin};

use crate::app_ids::AppIds;
use crate::backup::BackupSchedulerViewModel;
use crate::editors::EditorsViewModel;
use crate::models::OpenDocsStore;
use crate::save::SaveStateViewModel;

use super::super::{mutation_ids_belong_to_work, mutation_origins};

/// Handles the install function needs from `App::build`.
pub(in crate::app) struct AutosaveDeps {
    pub comments_menu: Signal<bool>,
    /// The margin-lane switch's menu mirror, and the persisted signal behind it.
    pub margin_lane_menu: Signal<bool>,
    pub margin_lane_enabled: Signal<bool>,
    pub comments_visible: Signal<bool>,
    pub spell_docs: OpenDocsStore,
    pub editors: EditorsViewModel,
    pub save_state: SaveStateViewModel,
    pub autosave: Signal<bool>,
    pub app_ctx: Rc<AppContext>,
    pub ids: AppIds,
    pub backup_scheduler: BackupSchedulerViewModel,
}

pub(in crate::app) fn install(ctx: &mut BuildContext, deps: &AutosaveDeps) {
    // ── Tools ▸ Comments ─────────────────────────────────────────────────
    // Same shape as the spell-check switch above, and deliberately the same scope: one
    // persisted app-wide key, mirrored into the menu's checkmark and pushed into the
    // store, which owns both halves of the hide (the view-model flag the margin binds,
    // and every open document's highlight layer).
    //
    // Presentation only. The docks keep listing every thread and the AccessKit
    // annotations keep announcing them — a writer who decluttered the page must still
    // be able to act on the notes they hid, and a screen-reader user gets nothing from
    // losing them.
    {
        deps.comments_menu.set(deps.comments_visible.get());
        let menu = deps.comments_menu.clone();
        let docs = deps.spell_docs.clone();
        // Seeded before the first document opens, so a launch with comments hidden
        // never paints a frame of ochre before the effect catches up.
        docs.set_comments_visible(deps.comments_visible.get());
        ctx.effect(&deps.comments_visible, move |on| {
            menu.set(*on);
            docs.set_comments_visible(*on);
        });
    }
    // ── View ▸ Margin marks ──────────────────────────────────────────────
    // The persisted key is the single source; the menu row's checkmark is a
    // mirror of it, pushed one way. The intent writes the key rather than the
    // mirror, so the menu and Settings ▸ Editor ▸ Margin marks cannot disagree
    // about whether the lane is on — the same shape the Comments switch above
    // uses, and for the same reason.
    {
        deps.margin_lane_menu.set(deps.margin_lane_enabled.get());
        let menu = deps.margin_lane_menu.clone();
        ctx.effect(&deps.margin_lane_enabled, move |on| menu.set(*on));
    }

    // Synopsis spell dormancy is not wired here: a doc can be on screen
    // several times at once (split pane, distraction-free), so "may this
    // session sleep?" is answered by counting the views that actually show
    // it — see `OpenDoc::acquire_synopsis_viewer`, held by the mounted pane.
    // Dirty tracking + debounced autosave-to-disk. Every mutation (editor
    // typing via the editors' `edited` signal, plus tree/metadata events)
    // marks the work `unsaved` and — when autosave is on — (re)schedules a
    // one-shot wake ~1.5 s out. `wake_at` keeps the loop asleep until the
    // deadline (no 60 fps drain); the `frame_tick` effect only runs on the
    // frames that actually pump, and fires the save when the deadline passes.
    // The countdown policy lives in `crate::save::timers` (pure, `now`-injected,
    // unit-tested); `App` keeps only the two effects it must own — arming the
    // framework's `wake_at` and actually writing to disk.
    {
        use std::time::Instant;
        let countdown = Rc::new(crate::save::AutosaveCountdown::new());
        let wake = ctx.wake_at_handle();
        let autosave = deps.autosave.clone();

        // Autosave rearms off `dirty_seq` **itself**, not off each site that
        // bumps it. Every source — this window's typing, the
        // whitelisted entity events, and an extension's
        // `WorkHandle::mark_changed` — therefore rearms for free, with no
        // per-source special-casing. Without this an extension-only edit
        // would still gate close/quit/switch/Ctrl+S (those read `unsaved`
        // directly) but would never trigger a timed write, so a crash before
        // the next manual save would lose it.
        {
            let countdown = countdown.clone();
            let wake = wake.clone();
            let autosave = autosave.clone();
            ctx.effect(&deps.save_state.dirty_seq(), move |_| {
                if let Some(at) = countdown.on_mutation(Instant::now(), autosave.get()) {
                    wake.set(Some(at));
                }
            });
        }

        let on_mutation = {
            let save_state = deps.save_state.clone();
            Rc::new(move || {
                // Bump the shared edit sequence: this mutation is now ahead of
                // whatever the last save covered, so the derived `unsaved` goes
                // true for every window onto this project — and stays true if
                // the save in flight (if any) predates it. The rearm rides on
                // the bump, above.
                save_state.bump_dirty();
            })
        };
        {
            let oc = on_mutation.clone();
            ctx.effect(&deps.editors.edited_signal(), move |_| oc());
        }
        // Guarded: a sibling Work's mutation must not mark THIS window's Work
        // dirty or re-arm its autosave timer. Unlike `LoadWork`/`NewWork`/
        // `CloseWork`, none of these entity kinds' events carry a
        // `work_id` — see `mutation_ids_belong_to_work`'s docs for the
        // relationship walk each one needs.
        for origin in mutation_origins() {
            let oc = on_mutation.clone();
            let app_ctx = deps.app_ctx.clone();
            let my_ids = deps.ids.clone();
            ctx.subscribe_event(origin, move |e: &Event| {
                let Origin::DirectAccess(entity) = e.origin.clone() else {
                    return;
                };
                let Some(my_work_id) = my_ids.work_id.get() else {
                    return; // nothing open here — cannot be my mutation
                };
                if mutation_ids_belong_to_work(&app_ctx, my_work_id, entity, &e.ids) {
                    oc();
                }
            });
        }
        {
            let editors = deps.editors.clone();
            let autosave = autosave.clone();
            let tick = ctx.frame_tick();
            ctx.effect(&tick, move |_| {
                let (save, resleep) = countdown.tick(Instant::now(), autosave.get());
                if save {
                    editors.save_to_disk();
                }
                if let Some(at) = resleep {
                    wake.set(Some(at));
                }
            });
        }
    }

    // Periodic "every N hours" backup while a project is open. Mirrors the
    // autosave timer: a `wake_at` deadline keeps the loop asleep; the
    // `frame_tick` effect fires `interval_tick` when the deadline passes and
    // re-arms. The interval (and whether it's on) comes from the open project's
    // effective policy via the scheduler; `None` disarms it.
    {
        use crate::save::IntervalTick;
        use std::time::{Duration, Instant};
        let countdown =
            crate::save::IntervalCountdown::new(deps.backup_scheduler.completed_epoch());
        let wake = ctx.wake_at_handle();
        let scheduler = deps.backup_scheduler.clone();
        let tick = ctx.frame_tick();
        ctx.effect(&tick, move |_| {
            let interval = scheduler.interval_secs().map(Duration::from_secs);
            match countdown.tick(Instant::now(), interval, scheduler.completed_epoch()) {
                IntervalTick::Disarmed => {}
                IntervalTick::Sleep(at) => wake.set(Some(at)),
                IntervalTick::Fire(next) => {
                    scheduler.interval_tick();
                    wake.set(Some(next));
                }
            }
        });
    }
}
