// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The crate is reachable from outside itself.
//!
//! An integration test compiles as its **own crate** linked against `teksilo_ui`,
//! which is what makes this a real check rather than a restatement of the unit
//! tests: everything named below has to be reachable the way a downstream
//! extension crate would reach it. While this was a `[[bin]]`-only target none of
//! it was — not one type — and that is the single fact that blocked the whole UI
//! half of the extension seam.
//!
//! If someone reverts the split, or turns a `pub mod` back into `mod`, this file
//! stops compiling. That is the entire point; there is deliberately very little
//! runtime assertion here.

/// The entry point the binary is now a one-line wrapper around.
#[test]
fn run_is_callable_from_outside_the_crate() {
    // Not invoked — it opens windows and elects a single instance. Taking its
    // address is what proves it is public and correctly typed.
    let entry: fn() = teksilo_ui::run;
    assert!(
        !std::ptr::eq(entry as *const (), std::ptr::null()),
        "run() must be a real, externally-callable entry point"
    );
}

/// The modules an extension has to reach to contribute anything at all.
///
/// Named individually rather than with a glob so that losing one is a compile
/// error naming *that* module, instead of a silent narrowing nobody notices.
#[test]
fn the_extension_facing_modules_are_public() {
    // Docks: the roster and the stable ids an extension dock must not collide with.
    let _: u64 = teksilo_ui::docks::OUTLINE_DOCK_ID;
    let roster_len = teksilo_ui::docks::APP_DOCKS.len();
    assert!(roster_len > 0, "the app must declare at least one dock");

    // The rest of the seam's surface, reached as a downstream crate would.
    #[allow(unused_imports)]
    use teksilo_ui::{
        app, app_ids, docks, editors, export, icons, intents, models, panels, sessions, settings,
        settings_keys, shell, singles, statusbar, tabs, tags, widgets,
    };
}
