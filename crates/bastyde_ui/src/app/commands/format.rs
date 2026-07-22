// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Commands over the manuscript's typography: inserting a scene break.
//!
//! A scene break is an **explicit authorial mark** — the binder is
//! organisational, so item adjacency never implies one. These commands place the
//! canonical mark at the caret; an export preset decides how it prints (a Shunn
//! `#`, a dinkus, or a bare gap, per region).

use bastyde::prelude::*;
use skribisto_model::scene_break::SceneBreakTier;

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
            .name("Insert Scene Break")
            .primary(KeyStroke::new(
                Key::Enter,
                Modifiers::CTRL | Modifiers::SHIFT,
            ))
            .build(),
    );
    ctx.register_shortcut_global(
        Shortcut::new("format.major_scene_break")
            .name("Insert Major Scene Break")
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
}
