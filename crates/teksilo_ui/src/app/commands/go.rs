// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Go menu's commands: prev/next Scene, Chapter and Note, scoped to the focused
//! item's own binder — Increment 4 of distraction-free.
//!
//! Eight bare-named global actions, none with an `AppIntent` variant — the same
//! "fixed behaviour, keystroke/menu only" shape `editor.find_next` and
//! `format.scene_break` already use. Six are kind-specific (menu rows only, no
//! individual shortcut — see the Go menu built in `shell/windows.rs`); the other two,
//! `go.next`/`go.prev`, are the generic pair the distraction-free strip's own
//! Next/Previous buttons fire, bound to Alt+Down/Alt+Up (verified free — no
//! Alt-only chord is registered anywhere else — leaving Alt+Left/Alt+Right free for a
//! possible future history feature). They read the *focused tab's own kind* at
//! dispatch time and delegate to the matching kind-specific answer.
//!
//! Every handler resolves its target through [`EditorsViewModel::go`], which is the
//! **existing** `open_or_focus` path — never a bespoke `OpenDocsStore::open` — so
//! autosave, dirty tracking and `StatsModel::active_item` all see a Go jump exactly
//! like any ordinary binder navigation. A target that does not exist (nothing
//! focused, or nothing of that kind in that direction — no wraparound) is a quiet
//! no-op, the same shape `format.scene_break`'s own internal guard already uses.

use skribisto_model::{GoDirection, GoKind};
use teksilo::prelude::*;

use crate::editors::EditorsViewModel;

use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    register_kind(ctx, deps, "go.next_scene", GoKind::Scene, GoDirection::Next);
    register_kind(
        ctx,
        deps,
        "go.prev_scene",
        GoKind::Scene,
        GoDirection::Previous,
    );
    register_kind(
        ctx,
        deps,
        "go.next_chapter",
        GoKind::Chapter,
        GoDirection::Next,
    );
    register_kind(
        ctx,
        deps,
        "go.prev_chapter",
        GoKind::Chapter,
        GoDirection::Previous,
    );
    register_kind(ctx, deps, "go.next_note", GoKind::Note, GoDirection::Next);
    register_kind(
        ctx,
        deps,
        "go.prev_note",
        GoKind::Note,
        GoDirection::Previous,
    );

    // The generic pair: keyboard/strip-only, never a menu row (the six kind-specific
    // rows above already cover the menu). Alt+Down steps forward, Alt+Up backward —
    // Down/Up rather than Left/Right so a future history feature can still claim
    // Alt+Left/Alt+Right, matching the prior-art shape one axis over.
    ctx.register_shortcut_global(
        Shortcut::new("go.next")
            .name("Next")
            .primary(KeyStroke::new(Key::ArrowDown, Modifiers::ALT))
            .build(),
    );
    ctx.register_shortcut_global(
        Shortcut::new("go.prev")
            .name("Previous")
            .primary(KeyStroke::new(Key::ArrowUp, Modifiers::ALT))
            .build(),
    );
    {
        let editors = deps.editors.clone();
        ctx.register_action_global(Action::new("go.next").on_invoke(move |_i, _c| {
            if let Some(kind) = editors.focused_go_kind() {
                editors.go(kind, GoDirection::Next);
            }
        }));
    }
    {
        let editors = deps.editors.clone();
        ctx.register_action_global(Action::new("go.prev").on_invoke(move |_i, _c| {
            if let Some(kind) = editors.focused_go_kind() {
                editors.go(kind, GoDirection::Previous);
            }
        }));
    }

    // `go.to` — the shortcut only. Its **action** is registered by the
    // `GoToButton` widget itself, through `PopoverWidget::open_action`, because
    // opening a popover needs the `EventContext` that only the widget's own
    // toggle closure has (see that widget's module doc). Registering a second
    // action here under the same name would shadow it with something that could
    // not actually present the overlay.
    ctx.register_shortcut_global(
        Shortcut::new("go.to")
            .name("Go to")
            .primary(KeyStroke::ctrl(Key::G))
            .build(),
    );
    // `go.to` is the name the menu and the shortcut use; it forwards to
    // whichever `GoToButton` is actually on screen. There are two per window —
    // the status bar's and the distraction-free strip's — and only one is ever
    // visible. Letting both claim the same action meant Ctrl+G could be
    // answered by the *hidden* one, whose trigger has no live bounds, so the
    // popover opened anchored to nothing in the top-left corner.
    {
        let focus = deps.focus.clone();
        ctx.register_action_global(Action::new("go.to").on_invoke(move |_i, c| {
            let target = if focus.active_signal().get() {
                crate::statusbar::go_to_button::GO_TO_FOCUS
            } else {
                crate::statusbar::go_to_button::GO_TO_MAIN
            };
            c.send_intent(Intent::new(target));
        }));
    }
}

/// Register one kind-specific Go action: fixed `kind`/`direction`, no payload, no
/// shortcut — reached only via its Go-menu row (`shell/windows.rs`) or a scripted
/// intent by name.
fn register_kind(
    ctx: &mut BuildContext,
    deps: &CommandDeps,
    name: &'static str,
    kind: GoKind,
    direction: GoDirection,
) {
    let editors: EditorsViewModel = deps.editors.clone();
    ctx.register_action_global(Action::new(name).on_invoke(move |_i, _c| {
        editors.go(kind, direction);
    }));
}
