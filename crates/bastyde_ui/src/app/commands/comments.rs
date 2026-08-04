// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Commands that create comments on the manuscript.
//!
//! Both act on the **focused prose editor**, resolved live through
//! `EditorsViewModel` rather than from a cached handle — a tab rebuild mints a
//! fresh editor, so a stored handle would address the one the writer *used* to be
//! typing in.
//!
//! The chords follow Google Docs (`Ctrl+Alt+M` for a comment) rather than
//! inventing a mnemonic. A plain `Ctrl+letter` was not available: the editor owns
//! A/C/X/V/B/I/U/Z/Y as built-ins, and a Global shortcut resolves *before* the
//! focused widget sees the key, so binding one would silently shadow it — which
//! is exactly why the outline toggle is F9 and not Ctrl+B.

use bastyde::prelude::*;

use super::CommandDeps;
use crate::view_models::SettingsViewModel;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    ctx.register_shortcut_global(
        Shortcut::new("comments.add")
            .name("Add Comment")
            .primary(KeyStroke::new(Key::M, Modifiers::CTRL | Modifiers::ALT))
            .build(),
    );
    ctx.register_shortcut_global(
        Shortcut::new("comments.add_paragraph")
            .name("Comment on Paragraph")
            .primary(KeyStroke::new(
                Key::M,
                Modifiers::CTRL | Modifiers::ALT | Modifiers::SHIFT,
            ))
            .build(),
    );

    {
        let editors = deps.editors.clone();
        ctx.register_action_global(
            Action::new("comments.add").on_invoke(move |_i, c| editors.add_comment_at_selection(c)),
        );
    }
    {
        let editors = deps.editors.clone();
        ctx.register_action_global(
            Action::new("comments.add_paragraph")
                .on_invoke(move |_i, c| editors.add_paragraph_comment(c)),
        );
    }

    // Tools ▸ Comments — show or hide the marks and the margin.
    //
    // Flips the *setting* and nothing else, exactly like `spellcheck.toggle`: the
    // effect in `App::build` owns the application, so there is one path from the menu
    // to `set_comments_visible` whatever fired it. No shortcut: F7 has thirty years of
    // spell-check behind it and there is no comparable convention for this, so an
    // invented chord would only be one more Global binding shadowing the editor's own
    // keys for a command reached once a session.
    {
        let visible = SettingsViewModel::new(ctx.settings()).comments_visible();
        ctx.register_action_global(Action::new("comments.toggle").on_invoke(move |_i, _c| {
            let now = !visible.get();
            visible.set(now);
        }));
    }
}
