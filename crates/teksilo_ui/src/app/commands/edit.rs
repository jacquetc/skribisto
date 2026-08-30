// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **Edit ▸ Undo / Redo** — the application's one Undo.
//!
//! # Why registering `Ctrl+Z` globally is safe
//!
//! teksilo resolves registered shortcuts in *stage 1*, before any widget sees
//! the raw key, while a text widget's own `Ctrl+Z` is *stage 3* `on_key`. So
//! these preempt **every** text surface in the window — which is the point: one
//! chord, one meaning, routed by [`crate::edit::UndoGroupViewModel`] rather than
//! by whichever widget happens to catch the key first.
//!
//! Preempting every text surface also means owing every one of them an answer,
//! and an application cannot produce one from its own knowledge: it recognises
//! the editors it built and kept a handle on, and is blind to the rest — a
//! rename box in the Overview, a tag field, a search input. Guessing gets it
//! exactly backwards, undoing a *structural* command instead of the writer's
//! typing.
//!
//! So the group does not guess. It asks the framework, through
//! `teksilo::core::text_surface::TextSurfaces`: every text widget registers
//! itself on build, so "is the caret in a text surface, and which one" is
//! answered **completely** — including for widgets added after this was
//! written, which is what a list maintained here could never be.
//!
//! The second half of the guarantee is the gate. A `Shortcut` whose
//! `enabled_when` is false is treated by the dispatcher as *if it were not
//! registered*, and the keystroke falls through to the focused widget's normal
//! handling. So when the router has nothing to offer — inside a suspended
//! modal, over a frozen writing game, with nothing to undo anywhere — the
//! widget gets its own key back.
//!
//! Because they are registered, these chords also appear in **Help ▸ Keyboard
//! shortcuts** and become rebindable in **Settings ▸ Keymap** — two things
//! `Ctrl+Z` has never been in this application.

use teksilo::prelude::*;

use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    let group = deps.undo_group.clone();
    let can_undo = group.can_undo();
    let can_redo = group.can_redo();

    ctx.register_shortcut_global(
        Shortcut::new("edit.undo")
            .name(tr!(shortcut_name_edit_undo()))
            .primary(KeyStroke::ctrl(Key::Z))
            .enabled_when(can_undo.clone())
            .build(),
    );
    ctx.register_shortcut_global(
        Shortcut::new("edit.redo")
            .name(tr!(shortcut_name_edit_redo()))
            // Both of the editor's own redo chords reach the group, so a writer
            // who learned either one keeps it.
            .primary(KeyStroke::ctrl_shift(Key::Z))
            .secondary(KeyStroke::ctrl(Key::Y))
            .enabled_when(can_redo.clone())
            .build(),
    );

    // Gated at *both* ends with the same signal, the way `editor.save` is: the
    // shortcut stops matching the keystroke and the action stops matching the
    // intent, so a menu row, a chord and a scripted intent can never disagree
    // about whether there is anything to undo.
    {
        let group = group.clone();
        ctx.register_action_global(Action::new("edit.undo").enabled_when(can_undo).on_invoke(
            move |_i, c| {
                group.undo();
                // The undo may have rewritten prose behind an open tab, and
                // `set_djot` only queues a document event.
                c.request_frame();
            },
        ));
    }
    {
        let group = group.clone();
        ctx.register_action_global(Action::new("edit.redo").enabled_when(can_redo).on_invoke(
            move |_i, c| {
                group.redo();
                c.request_frame();
            },
        ));
    }

    // ── Clipboard ─────────────────────────────────────────────────────────
    //
    // Registered globally for the same reason Undo is — one meaning per chord,
    // routed by the same focus rule — and safe for the same reason: each is
    // gated, and a disabled shortcut falls through to the focused widget. So in
    // any surface the router cannot name, Ctrl+X/C/V/A behave exactly as they
    // did before.
    // The gate is what keeps that honest, and it is deliberately live rather
    // than latched: `UndoGroupViewModel` answers "is there a text surface to
    // act on" from the focused surface (or a rich editor that actually holds
    // the caret), never from the Format dock's sticky target — otherwise these
    // would stay enabled for the rest of the session after one click in a
    // scene, and Ctrl+A would stop selecting rows in every list in the window.
    let clip = [
        (
            "edit.cut",
            tr!(shortcut_name_edit_cut()),
            KeyStroke::ctrl(Key::X),
            group.can_cut(),
        ),
        (
            "edit.copy",
            tr!(shortcut_name_edit_copy()),
            KeyStroke::ctrl(Key::C),
            group.can_copy(),
        ),
        (
            "edit.paste",
            tr!(shortcut_name_edit_paste()),
            KeyStroke::ctrl(Key::V),
            group.can_paste(),
        ),
        (
            "edit.paste_plain",
            tr!(shortcut_name_edit_paste_plain()),
            KeyStroke::new(Key::V, Modifiers::CTRL | Modifiers::SHIFT),
            group.can_paste(),
        ),
        (
            "edit.select_all",
            tr!(shortcut_name_edit_select_all()),
            KeyStroke::ctrl(Key::A),
            group.can_select_all(),
        ),
    ];
    for (id, name, chord, enabled) in clip {
        ctx.register_shortcut_global(
            Shortcut::new(id)
                .name(name)
                .primary(chord)
                .enabled_when(enabled.clone())
                .build(),
        );
        let group = group.clone();
        ctx.register_action_global(Action::new(id).enabled_when(enabled).on_invoke(
            move |_i, c| {
                match id {
                    "edit.cut" => group.cut(c),
                    "edit.copy" => group.copy(c),
                    "edit.paste" => group.paste(c),
                    "edit.paste_plain" => group.paste_plain(c),
                    _ => group.select_all(),
                }
                c.request_frame();
            },
        ));
    }
}
