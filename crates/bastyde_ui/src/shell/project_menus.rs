// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Format-menu helpers shared by the project window chrome.
//!
//! The dock and the Format menu bind the same view-model signals; these two
//! constructors keep the menu rows requesting a frame and restoring focus the
//! same way the dock's buttons do.

use bastyde::prelude::*;
use bastyde::widgets::MenuEntry;

use crate::view_models::FormatViewModel;

/// A Format-menu row that reflects document state: a reflect-only checkmark
/// mirroring the same signal the dock's button binds, so the two surfaces
/// cannot disagree about whether the selection is bold.
///
/// `checked` and not `checkable`: the mark mirrors the document read-only, and
/// the command is what changes it. A checkable row would write the signal on
/// click and fight the value the editor reports back.
pub(crate) fn mark(
    vm: &FormatViewModel,
    label: bastyde::i18n::LocalizedString,
    enabled: Signal<bool>,
    state: Signal<bool>,
    run: fn(&FormatViewModel),
) -> MenuEntry {
    let vm = vm.clone();
    MenuEntry::new(label)
        .enabled(enabled)
        .checked(state)
        .on_activate(move |c| {
            run(&vm);
            c.request_frame();
            vm.refocus(c);
        })
}

/// A Format-menu row that just runs a command.
///
/// Like the dock's buttons it requests a frame: the pointer is on the menu
/// overlay and the editor is unfocused, so nothing else schedules the repaint
/// that shows the edit. It then puts focus back where the writer was typing —
/// reaching a menu item took it away, and the dock and context menu do not have
/// that problem (the dock's buttons are non-focusable, and dismissing the
/// context menu restores focus by itself).
pub(crate) fn command(
    vm: &FormatViewModel,
    label: bastyde::i18n::LocalizedString,
    enabled: Signal<bool>,
    run: fn(&FormatViewModel),
) -> MenuEntry {
    let vm = vm.clone();
    MenuEntry::new(label)
        .enabled(enabled)
        .on_activate(move |c| {
            run(&vm);
            c.request_frame();
            vm.refocus(c);
        })
}

