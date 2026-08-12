// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Putting a footnote into the manuscript.
//!
//! One command, and a thin one: the whole act lives in
//! [`FootnotesViewModel::insert_at`](crate::footnotes::FootnotesViewModel::insert_at)
//! so it can be tested without a window. This shell only resolves *where* —
//! which editor holds the caret, and which `Content` row is behind it — and says
//! so when there is nowhere to put one.
//!
//! The chord is `Ctrl+Alt+F`, following the comment commands next door rather
//! than inventing a mnemonic. A plain `Ctrl+letter` was not available: the editor
//! owns A/C/X/V/B/I/U/Z/Y as built-ins, and a Global shortcut resolves *before*
//! the focused widget sees the key, so binding one would silently shadow it.
//!
//! Prose only, and resolved through
//! [`FormatViewModel::footnote_target`](crate::format::FormatViewModel::footnote_target)
//! — the registry that knows **which** editor holds the caret. Resolving through
//! the focused *tab* instead is what put every marker at the top of the document:
//! a tab's handle is whatever `writing_column` last attached to the find banner,
//! which reports a caret of 0 for an editor nobody is typing in.

use teksilo::prelude::*;
use teksilo::widgets::Toast;

use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    ctx.register_shortcut_global(
        Shortcut::new("editor.insert_footnote")
            .name("Insert Footnote")
            .primary(KeyStroke::new(Key::F, Modifiers::CTRL | Modifiers::ALT))
            .build(),
    );

    let format = deps.format.clone();
    let session = deps.session.clone();
    ctx.register_action_global(Action::new("editor.insert_footnote").on_invoke(
        move |_i, c: &mut EventContext| {
            let Some(footnotes) = session.open_docs.footnotes() else {
                // No project, so no `Work` to own the note. Said out loud rather
                // than swallowed: a menu item that does nothing at all reads as a
                // broken build.
                c.show_toast(Toast::warning(tr!(footnotes_no_project())));
                return;
            };
            let Some((handle, binding)) = format.footnote_target() else {
                c.show_toast(Toast::warning(tr!(footnotes_no_caret())));
                return;
            };
            if footnotes.insert_at(&handle, &binding).is_none() {
                c.show_toast(Toast::error(tr!(footnotes_not_created())));
                return;
            }
            // The menu overlay took focus when it opened; without this the writer
            // is left with no caret, exactly as the template and image commands
            // document.
            handle.focus(c);
        },
    ));
}
