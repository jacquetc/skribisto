// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Keymap — rebind application shortcuts.
//!
//! Drops Bastyde's stock [`ShortcutSettings`] into the Keymap page: every
//! shortcut currently registered in the tree's `ShortcutRegistry` (the same
//! ones the menus and global actions fire) listed by category, with Rebind /
//! Unbind / Reset and conflict confirmation. A filter field above the list
//! drives `with_filter` so the writer can find a chord without scrolling the
//! whole catalogue.

use bastyde::prelude::*;
use bastyde::widgets::{Expand, SearchField, ShortcutSettings, VStack};

#[allow(unused_imports)]
use super::super::*;

/// Settings ▸ Keymap — filter + the framework's shortcut rebind panel.
pub(in crate::settings) fn keymap_pane(_ctx: &mut BuildContext) -> impl Widget {
    let filter = Signal::new(String::new());
    // Conflict confirmation is on: rebinding a chord already used elsewhere
    // shows an inline "already assigned to X — Reassign / Cancel" prompt on
    // the row rather than silently stealing it.
    let body = VStack::new()
        .spacing(12.0)
        .child(SearchField::new(filter.clone()).placeholder(tr!(settings_keymap_filter())))
        .child(
            Expand::horizontal().respect_intrinsic().child(
                ShortcutSettings::new()
                    .with_filter(filter)
                    .confirm_conflicts(true),
            ),
        );

    pane_frame(crumb(None, tr!(settings_page_keymap())), body)
}
