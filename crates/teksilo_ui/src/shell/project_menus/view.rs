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

    // The sides, shown or hidden. Reflect-only marks, like every row here.
    let left_visible = outline.is_visible();
    let right_visible = outline.docking().side_visible_signal(DockSide::Trailing);
    // The bottom band has no persistent reveal affordance of its own: a hidden
    // top/bottom side collapses its rail with it (unlike leading/trailing, whose rail
    // survives as the way back). So this row, and the two bottom panels' own rows, are
    // not a convenience: they are the only way to bring the band back once it is closed.
    let bottom_visible = outline.docking().side_visible_signal(DockSide::Bottom);

    // Every panel first, in a submenu of its own: they are places to go, and the rest
    // of this menu changes how the window looks. One row per panel, grouped by side,
    // built from the same table as the commands, so a panel cannot have a chord and no
    // row, or a row and no chord.
    m.submenu(tr!(menu_panels()), |s| {
        let mut s = s;
        let mut previous_side = None;
        for panel in crate::docks::panels::PANELS {
            let side = crate::docks::panels::side_of(panel.dock);
            if previous_side.is_some_and(|p| p != side) {
                s = s.separator();
            }
            previous_side = Some(side);
            s = s.item(
                MenuEntry::new((panel.title)())
                    .intent(panel.intent)
                    .shortcut(panel.intent),
            );
        }
        s
    })
    .separator()
    .item(
        MenuEntry::new(tr!(menu_side_start()))
            .checked(left_visible)
            .intent("outline.toggle")
            .shortcut("outline.toggle"),
    )
    .item(
        MenuEntry::new(tr!(menu_side_end()))
            .checked(right_visible)
            .intent("view.right_panel")
            .shortcut("view.right_panel"),
    )
    .item(
        MenuEntry::new(tr!(menu_side_bottom()))
            .checked(bottom_visible)
            .intent("preview.toggle")
            .shortcut("preview.toggle"),
    )
    .separator()
    // The margin lane. Reflect-only, like the sides above it: the row shows the
    // setting and the intent writes it, so the menu and Settings ▸ Editor ▸ Margin
    // marks are the same switch rather than two that can drift.
    .item(
        MenuEntry::new(tr!(menu_margin_lane()))
            .checked(parts.margin_lane_menu.clone())
            .intent("view.margin_lane")
            .shortcut("view.margin_lane"),
    )
    .separator()
    // Increment 1 of distraction-free: plain fullscreen,
    // not the collapsed-chrome mode itself (that is a
    // later increment). Shift+F11: plain F11 is distraction-free's.
    .item(
        MenuEntry::new(tr!(menu_fullscreen()))
            .checked(is_fullscreen)
            .intent("view.fullscreen")
            .shortcut("view.fullscreen"),
    )
    // Increment 2 of distraction-free: chrome
    // collapse + docks disabled + fullscreen,
    // together, as one per-window mode. F11.
    .item(
        MenuEntry::new(tr!(menu_focus_mode()))
            .checked(is_focus_mode)
            .intent("view.focus_mode")
            .shortcut("view.focus_mode"),
    )
    .separator()
    // Text size — the discoverable half of Ctrl+Wheel, and the reason the
    // commands resolve through the format registry's sticky latch rather than
    // live focus: a menu opened from the keyboard (F10, Alt) has taken focus
    // off the editor while its rows are on screen.
    //
    // No `enabled` gate. The actions no-op when nothing is focused, and a row
    // that greys itself out the instant the menu opens from the keyboard (which
    // is exactly when focus leaves the editor) would read as broken.
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

#[cfg(test)]
mod tests {
    /// The Panels rows are labelled with each dock's own title. The timeline's used to
    /// be a separate key that had to be renamed by hand with the dock's ("Timeline" to
    /// "Go back in time"); pinned here so a row can never again name a place the panel
    /// does not.
    #[test]
    fn a_panel_row_names_the_same_place_as_its_dock() {
        let timeline = crate::docks::panels::PANELS
            .iter()
            .find(|p| p.dock == crate::docks::TIMELINE_DOCK_ID)
            .expect("the timeline has a panel command");
        assert_eq!(
            (timeline.title)().resolve_now(),
            tr!(timeline_title()).resolve_now()
        );
    }

    use teksilo::prelude::*;
}
