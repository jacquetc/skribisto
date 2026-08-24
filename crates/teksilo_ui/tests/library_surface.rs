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

/// **A whole lane provider, built naming nothing but `ext`.**
///
/// `ext`'s own drift test walks `pub fn register…` declarations and asserts each
/// is re-exported. A **type** carried through one of those signatures is
/// invisible to it — which is how `WorkHandle` went a release without being
/// reachable, and how four of the margin lane's own types did: a spec is
/// declared with a `LaneColumn` and a `LaneShape`, and its closure returns
/// `LaneMark`s built on `LaneSpan`s. Ten of the lane's fourteen names were
/// exported and the four that a provider cannot be *written* without were not.
///
/// So this is the check the walk cannot be: a downstream edition's registration,
/// compiled from outside the crate, importing `ext` and nothing else. It asserts
/// almost nothing at runtime on purpose. If a type leaves `ext`, this file stops
/// compiling, and the error names it.
#[test]
fn an_extension_can_build_a_lane_provider_naming_only_ext() {
    use std::rc::Rc;
    use teksilo_ui::ext::{
        LaneColumn, LaneMark, LaneProviderSpec, LaneRefresh, LaneShape, LaneSpan, LaneSurface,
        register_lane_provider,
    };

    let spec = LaneProviderSpec {
        id: "library-surface-probe".to_string(),
        label: Rc::new(|| teksilo::prelude::lit!("Probe")),
        hint: Rc::new(|| teksilo::prelude::lit!("What the probe marks")),
        column: LaneColumn::Right,
        shape: LaneShape::Diamond,
        palette_slot: 2,
        surfaces: &[LaneSurface::Stream],
        default_on: false,
        refresh: LaneRefresh::Manual,
        marks: Rc::new(|ctx| {
            let Some(at) = (ctx.locate)(0) else {
                return Vec::new();
            };
            vec![LaneMark {
                id: ctx.item_id,
                span: LaneSpan::at(at),
                column: LaneColumn::Right,
                shape: LaneShape::Diamond,
                color: ctx.color,
                label: teksilo::prelude::lit!("a probe"),
                group: ctx.group,
            }]
        }),
    };

    let handle = register_lane_provider("test.library.surface", spec).expect("register");
    // The handle's type is nameable too — an edition has to store it in a struct
    // field for the life of the process, and one it cannot spell it cannot keep.
    let _kept: teksilo_ui::ext::LaneProviderHandle = handle;
}
