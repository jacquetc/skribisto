// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Corkboard's business logic: [`CorkboardViewModel`].
//!
//! One per container tab (Chapter/Part/Book), built alongside the stream and Pace
//! view-models in `ContentTab::new` and gated on the same
//! [`StreamLevel::for_container`](crate::models::StreamLevel::for_container). It owns
//! the drilled-into navigation (a folder card drills in *in place*, no new tab), the
//! free search/sort state, and the per-card synopsis documents shared with every
//! other open view of the same item. The grid, header and card views that consume it
//! live under [`crate::tabs::corkboard`].

mod corkboard_vm;

pub use corkboard_vm::{CorkboardViewModel, EDIT_SYNOPSIS_COMMAND, SORT_TITLE};

use teksilo::core::shortcut::{KeyStroke, Shortcut};
use teksilo::prelude::*;

use crate::shared::shortcut_group::{ShortcutGroup, ShortcutGroupExt};

/// The chord [`EDIT_SYNOPSIS_COMMAND`] answers to until the writer rebinds it.
///
/// `Shift+F2` because the board's keys around it are already spoken for: `Enter`, with
/// any modifier, opens the card (`GridView` reads the key without looking at modifiers,
/// and a hook on the grid cannot stop it), and plain `F2` renames it, as it does on every
/// other row in the app. Taking `F2` for the synopsis would have made a rename on the
/// board mean something else than a rename in the binder.
pub(crate) fn edit_synopsis_default_key() -> KeyStroke {
    KeyStroke::new(Key::F2, Modifiers::SHIFT)
}

/// One row in Settings ▸ Keymap, the shortcuts sheet and the palette, however many
/// boards are open.
///
/// Registered by the window rather than by a board: a board is built only when its
/// segment is shown, and a writer looking for the chord should find it before that.
/// Each board answers through its own `Action`, on the walk from the focused widget, so
/// the chord does nothing outside a board and the key then reaches whatever has focus.
pub(crate) fn edit_synopsis_shortcut() -> Shortcut {
    Shortcut::new(EDIT_SYNOPSIS_COMMAND)
        .group(ShortcutGroup::Document)
        .name(tr!(shortcut_name_corkboard_edit_synopsis()))
        .primary(edit_synopsis_default_key())
        .build()
}
