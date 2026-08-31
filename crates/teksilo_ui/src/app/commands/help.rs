// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Help commands: the Help window, the keyboard-shortcuts sheet, the command palette,
//! and the two outward links the Help menu owes a reader who is stuck.
//!
//! Every one is registered with `register_action_global` rather than
//! `register_action`, for the reason `App::build` documents at length: intents walk
//! source-widget to root, and the title-bar menu renders in an overlay that is a
//! *sibling* of `App`, so a plain registration is never reached from a menu row.
//!
//! ## Why the palette is registered here and not with the feature it opens
//!
//! It opens nothing of this app's. `CommandPalette` is a teksilo widget that reads the
//! tree's own `ShortcutRegistry`, so the palette's contents are whatever this app (and
//! any extension) registered as a shortcut. Wiring it is four lines and belongs next to
//! the other "how do I find things" commands.

use teksilo::prelude::*;
use teksilo::widgets::CommandPalette;

use super::CommandDeps;
use crate::help::window::open_or_focus_help;

/// Commands that must never appear as rows in the palette itself.
///
/// Opening the palette from the palette is a loop with nothing at the end of it, and
/// "Command palette" is the one row a reader who is already looking at the palette
/// certainly does not need.
const PALETTE_HIDDEN: &[&str] = &["app.command_palette"];

/// Whether a registered shortcut belongs in the palette.
///
/// The palette reads the whole `ShortcutRegistry`, which in a debug build also holds
/// teksilo's own inspector bindings (Toggle Picker, Cycle Bounds Overlay, Next Tab).
/// Those are development tools; a writer searching for a command should not meet them,
/// and having no category they sort to the top of an empty query.
fn belongs_in_palette(cmd: &teksilo::widgets::PaletteCommand) -> bool {
    !PALETTE_HIDDEN.contains(&cmd.id) && crate::commands_ext::is_palette_command(cmd.id)
}

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    // F1 is the platform convention on Windows and Linux and is unbound in this app
    // (F2, F3, F7 and F9 through F11 are taken; F1 was not). macOS has no F1 help
    // convention, but leaving it bound there costs nothing and keeps one description of
    // the shortcut across platforms.
    ctx.register_shortcut_global(
        Shortcut::new("help.topics")
            .name(tr!(shortcut_name_help_topics()))
            .category("Help")
            .primary(KeyStroke::new(Key::F1, Modifiers::NONE))
            .build(),
    );
    ctx.register_action_global(Action::new("help.topics").on_invoke(|_i, c| {
        // `None`: a reader returning to a Help window they left open has not asked to
        // lose their place. See `open_or_focus_help`.
        open_or_focus_help(c, None);
    }));

    // No chord: this is a command you find by name, and every chord spent is one the
    // writer cannot have. It is still rebindable in Settings ▸ Keymap, because the
    // registry lists it either way.
    ctx.register_shortcut_global(
        Shortcut::new("help.shortcuts")
            .name(tr!(shortcut_name_help_shortcuts()))
            .category("Help")
            .build(),
    );
    ctx.register_action_global(
        Action::new("help.shortcuts")
            .on_invoke(|_i, c| crate::help::shortcuts::present_shortcuts(c)),
    );

    ctx.register_shortcut_global(
        Shortcut::new("help.website")
            .name(tr!(shortcut_name_help_website()))
            .category("Help")
            .build(),
    );
    ctx.register_action_global(Action::new("help.website").on_invoke(|_i, c| {
        crate::shared::external_link::open_external_link(
            crate::shared::project_links::GITHUB_URL,
            c,
        );
    }));

    ctx.register_shortcut_global(
        Shortcut::new("help.report")
            .name(tr!(shortcut_name_help_report()))
            .category("Help")
            .build(),
    );
    ctx.register_action_global(Action::new("help.report").on_invoke(|_i, c| {
        crate::shared::external_link::open_external_link(
            crate::shared::project_links::ISSUES_URL,
            c,
        );
    }));

    // Ctrl+Shift+P: the convention every editor with a palette uses, and unbound here.
    ctx.register_shortcut_global(
        Shortcut::new("app.command_palette")
            .name(tr!(shortcut_name_command_palette()))
            .category("Help")
            .primary(KeyStroke::ctrl_shift(Key::P))
            .build(),
    );
    ctx.register_action_global(Action::new("app.command_palette").on_invoke(|_i, c| {
        CommandPalette::new().include(belongs_in_palette).present(c);
    }));

    // Settings, opened straight at Keymap. A separate action rather than a payload on
    // `app.settings`, which is how this app already addresses a specific settings page
    // (see `app.settings.games`). Fired by the shortcuts sheet's own Rebind button,
    // which promises that page by name.
    {
        let session = deps.session.clone();
        let undo = deps.undo_group.clone();
        ctx.register_action_global(Action::new("app.settings.keymap").on_invoke(move |_i, c| {
            let session = session.clone();
            let undo = undo.clone();
            crate::settings::present(c, move || {
                crate::settings::SettingsPanel::open_to_keymap(session, &undo)
            });
        }));
    }
}
