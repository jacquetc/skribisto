// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Commands over the manuscript's typography: inserting a scene break, and
//! linking.
//!
//! A scene break is an **explicit authorial mark** — the binder is
//! organisational, so item adjacency never implies one. These commands place the
//! canonical mark at the caret; an export preset decides how it prints (a Shunn
//! `#`, a dinkus, or a bare gap, per region).
//!
//! The link command lives here rather than with the other marks because it is
//! the only one that needs an [`EventContext`] — it opens a dialog instead of
//! flipping a property — and the dock's `command_button` cannot supply one. One
//! action serves all three doors (dock button, Format menu, Ctrl+K), so they
//! cannot drift.

use skribisto_model::scene_break::SceneBreakTier;
use teksilo::prelude::*;

use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    // Enter is the paragraph key, so the scene break sits one rung further up
    // the same ladder: Enter makes a paragraph, Ctrl+Enter forces a block (both
    // `RichTextEditor` built-ins), Ctrl+Shift+Enter marks an ordinary scene
    // break and Ctrl+Alt+Enter a major one. Chosen over a Ctrl+letter because
    // every one of those would shadow a built-in (A/C/X/V/B/I/U/Z/Y), and over a
    // function key because those are opaque; Enter is also layout-independent,
    // unlike any punctuation-based mnemonic on AZERTY or QWERTZ.
    //
    // A Global shortcut resolves before the focused widget sees the key, so the
    // editor's own `Ctrl+Enter` arm (which matches on ctrl regardless of shift)
    // never swallows these.
    ctx.register_shortcut_global(
        Shortcut::new("format.scene_break")
            .name(tr!(shortcut_name_format_scene_break()))
            .primary(KeyStroke::new(
                Key::Enter,
                Modifiers::CTRL | Modifiers::SHIFT,
            ))
            .build(),
    );
    ctx.register_shortcut_global(
        Shortcut::new("format.major_scene_break")
            .name(tr!(shortcut_name_format_major_scene_break()))
            .primary(KeyStroke::new(Key::Enter, Modifiers::CTRL | Modifiers::ALT))
            .build(),
    );

    {
        let editors = deps.editors.clone();
        ctx.register_action_global(
            Action::new("format.scene_break")
                .on_invoke(move |_i, c| editors.insert_scene_break(SceneBreakTier::Minor, c)),
        );
    }
    {
        let editors = deps.editors.clone();
        ctx.register_action_global(
            Action::new("format.major_scene_break")
                .on_invoke(move |_i, c| editors.insert_scene_break(SceneBreakTier::Major, c)),
        );
    }

    // Ctrl+K is the near-universal binding for this, and it shadows nothing:
    // it is absent from the app's global set and from `RichTextEditor`'s own
    // key handling (whose Ctrl+letter arms are A/C/X/V/B/I/U/Z/Y).
    ctx.register_shortcut_global(
        Shortcut::new("format.link")
            .name(tr!(shortcut_name_format_link()))
            .primary(KeyStroke::new(Key::K, Modifiers::CTRL))
            .build(),
    );
    {
        let format = deps.format.clone();
        ctx.register_action_global(
            Action::new("format.link")
                .on_invoke(move |_i, c| crate::format::link_panel::present(&format, c)),
        );
    }
}
