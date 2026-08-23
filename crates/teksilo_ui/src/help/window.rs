// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Help window: one per process, opened or focused through one door.
//!
//! ## Why a window and not a modal
//!
//! Settings is a modal because you go there, change something and come back. Help is
//! the opposite: the whole reason to open it is to do something *else* while reading
//! it. A modal makes the app unusable for exactly as long as the reader needs the
//! answer, which is the one thing help must not do.
//!
//! ## One door
//!
//! [`open_or_focus_help`] is the only way in. Every entry point (F1, the Help menu, the
//! Learn pane in the Launcher, and a future context-sensitive affordance) calls it, so
//! "is Help already open" is answered in one place, by the window's own string id,
//! rather than by a side table that can disagree with the window list. Same rule the
//! project windows follow in [`crate::shell::window_ids`].

use std::rc::Rc;

use teksilo::core::window::{DecorationsMode, WindowConfig};
use teksilo::prelude::*;

use super::help_vm::HelpViewModel;
use super::panel::HelpPanel;

/// The Help window's stable id. Persisted geometry keys on it, so it must never change.
pub const HELP_WINDOW_ID: &str = "help";

const W: u32 = 940;
const H: u32 = 680;
const MIN_W: u32 = 620;
const MIN_H: u32 = 420;

/// Show `topic_key` in the Help window, opening it if it is not already up.
///
/// Passing [`None`] opens whatever the window was last showing (or the default topic on
/// a first open), which is what a bare "Help" command should do: a reader returning to
/// a window they left open has not asked to lose their place.
pub fn open_or_focus_help(ctx: &mut EventContext, topic_key: Option<&str>) {
    let vm = ctx
        .app_state::<HelpViewModel>()
        .cloned()
        .unwrap_or_else(|| {
            // `app_state` is seeded once in `run()`. Falling back to a fresh view-model
            // rather than panicking keeps a missing registration from taking the app
            // down; the window still works, it just does not share its place with
            // another entry point.
            eprintln!("skribisto: no HelpViewModel in app_state; using a detached one");
            HelpViewModel::new()
        });

    if let Some(key) = topic_key {
        vm.open_fresh(key);
    }

    match ctx.find_window(HELP_WINDOW_ID) {
        Some(id) => ctx.focus_window(id),
        None => {
            ctx.open_window(help_window_config(vm));
        }
    }
}

/// The Help window's configuration.
///
/// Resizable, unlike the Launcher: a topic is prose, and how wide prose should be is
/// the reader's call, not this window's.
pub fn help_window_config(vm: HelpViewModel) -> WindowConfig {
    let vm = Rc::new(vm);
    WindowConfig::new()
        .id(HELP_WINDOW_ID)
        .title(crate::identity::display_name())
        .size(W, H)
        .min_size(MIN_W, MIN_H)
        .decorations(DecorationsMode::CustomChrome)
        .activate_from_env(true)
        .root(move |tree, _state| tree.add(HelpPanel::new((*vm).clone())))
}
