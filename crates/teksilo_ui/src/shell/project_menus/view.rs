// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **View** menu — what this window shows of itself.
//!
//! Every mark here is reflect-only: it mirrors the surface's own truth without
//! writing it, and the paired intent is what changes it. A `.checkable()` row
//! would write the signal on click and fight the model that owns it.

use teksilo::prelude::*;
use teksilo::widgets::{DockSide, MenuEntry, MenuItems};

use super::ProjectMenuParts;

/// The rows of the menu, in the order they appear.
pub(super) fn menu(m: MenuItems, parts: &ProjectMenuParts) -> MenuItems {
    let outline = parts.outline.clone();
    let placement = parts.placement.clone();
    let focus = parts.focus.clone();

    // Reflect-only checkmarks: they mirror each dock's truth
    // without writing it; the toggles are driven by the
    // `outline.toggle` (F9) / `preview.toggle` (F10) intents.
    let outline = outline.clone();
    // Increment 1 of distraction-free (plain fullscreen): a
    // reflect-only checkmark straight off THIS window's own
    // `WindowState::placement()` — the same round-tripping
    // signal `view.fullscreen`'s action writes and the OS
    // reads back (see `FullscreenViewModel`'s doc), so the
    // mark stays correct even if fullscreen is left behind
    // the app's back (KDE's own shortcut, the titlebar).
    let is_fullscreen = placement.clone().map(|p| *p == WindowPlacement::Fullscreen);
    // Increment 2 of distraction-free: this checkmark reflects
    // `FocusViewModel::active_signal()` directly — unlike
    // `is_fullscreen` above, this is this window's own live
    // state, not something round-tripped off the OS, so there
    // is no fresher source to read it from.
    let is_focus_mode = focus.active_signal();

    // The bottom band has no persistent reveal affordance
    // of its own — a hidden top/bottom side collapses its
    // rail with it (unlike leading/trailing, whose rail
    // survives as the way back). So the menu entry is not
    // a convenience here, it is the only way to bring the
    // preview back once it is closed.
    let preview_visible = outline.docking().side_visible_signal(DockSide::Bottom);
    m.item(
        MenuEntry::new(tr!(menu_outline()))
            .checked(outline.is_visible())
            .intent("outline.toggle")
            .shortcut("outline.toggle"),
    )
    // Reveal the leading search & replace dock. A plain
    // action (not a reflect-only checkbox): the search
    // dock is one of two switchable leading tabs, not a
    // side that is simply shown or hidden.
    .item(
        MenuEntry::new(tr!(menu_search()))
            .intent("search.show")
            .shortcut("search.show"),
    )
    // Reveal the trash dock — like Search, a plain action
    // (a switchable leading tab, not a shown/hidden side).
    .item(MenuEntry::new(tr!(menu_trash())).intent("trash.show"))
    // Reveal the footnotes dock. Not a convenience either: a dock
    // reachable only from its rail has no way back once a saved
    // desk stops mounting it, and this is the door that recovers it.
    .item(MenuEntry::new(tr!(menu_footnotes())).intent("footnotes.show"))
    .item(
        MenuEntry::new(tr!(menu_search_preview()))
            .checked(preview_visible)
            .intent("preview.toggle")
            .shortcut("preview.toggle"),
    )
    // Reveal the project-wide Timeline band. Not a convenience: it
    // shares the bottom side with the search preview, and a hidden
    // bottom side takes its rail with it — so unlike a leading or
    // trailing dock, there is no glyph left to click. Without this
    // entry the band would be shipped and unreachable.
    .item(MenuEntry::new(tr!(menu_timeline())).intent("timeline.show"))
    .separator()
    // Increment 1 of distraction-free: plain fullscreen,
    // not the collapsed-chrome mode itself (that is a
    // later increment). F11, the platform convention.
    .item(
        MenuEntry::new(tr!(menu_fullscreen()))
            .checked(is_fullscreen)
            .intent("view.fullscreen")
            .shortcut("view.fullscreen"),
    )
    // Increment 2 of distraction-free: chrome
    // collapse + docks disabled + fullscreen,
    // together, as one per-window mode. Shift+F11.
    .item(
        MenuEntry::new(tr!(menu_focus_mode()))
            .checked(is_focus_mode)
            .intent("view.focus_mode")
            .shortcut("view.focus_mode"),
    )
    .separator()
    // Text size — the discoverable half of Ctrl+Wheel, and the reason the
    // commands resolve through the format registry's sticky latch rather than
    // live focus: opening this menu has already taken focus off the editor by
    // the time a row is clicked.
    //
    // No `enabled` gate. The actions no-op when nothing is focused, and a row
    // that greys itself out the instant the menu opens — which is exactly when
    // focus leaves the editor — would read as broken.
    .item(
        MenuEntry::new(tr!(menu_text_size_increase()))
            .intent("editor.size.increase")
            .shortcut("editor.size.increase"),
    )
    .item(
        MenuEntry::new(tr!(menu_text_size_decrease()))
            .intent("editor.size.decrease")
            .shortcut("editor.size.decrease"),
    )
    .item(
        MenuEntry::new(tr!(menu_text_size_reset()))
            .intent("editor.size.reset")
            .shortcut("editor.size.reset"),
    )
}
