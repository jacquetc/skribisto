// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One "show this panel" command per app dock: the table View ▸ Panels and the
//! commands are both built from.
//!
//! A reader who cannot see where a panel sits cannot click its glyph on the rail
//! either, so every panel needs a door of its own: a menu row that names it, and a
//! chord. Before this table, six of the twelve had neither, and the binder's only
//! door (F9) showed or hid the whole left side, whichever of its five panels happened
//! to be in front.
//!
//! One list feeds the menu and the commands, so the two cannot disagree, and
//! `every_app_dock_has_exactly_one_panel_command` fails the build for a dock added to
//! [`super::APP_DOCKS`] without a door here.
//!
//! The chords are Ctrl+Shift plus a letter (⌘⇧ on a Mac), VS Code's shape for the
//! same doors. Taken elsewhere in the app, so not offered: E (export), F is Search's
//! own and stays, H (replace), M (margin marks), N (new window), P (command palette),
//! V and Z (Paste as plain text, Redo). Avoided: U (IBus' Unicode entry on Linux takes
//! it first), S, W, Q and Y, which read as Save as, close, quit and redo elsewhere.

use teksilo::prelude::*;
use teksilo::widgets::DockSide;

use super::{
    APP_DOCKS, COMMENTS_DOCK_ID, DOC_COMMENTS_DOCK_ID, FOOTNOTES_DOCK_ID, FORMAT_DOCK_ID,
    GAMES_DOCK_ID, INSPECTOR_DOCK_ID, OUTLINE_DOCK_ID, PREVIEW_DOCK_ID, SEARCH_DOCK_ID,
    TIMELINE_DOCK_ID, TRASH_DOCK_ID, VERSIONS_DOCK_ID,
};

/// One panel's door.
pub(crate) struct PanelCommand {
    /// The intent the menu row sends and the command answers; also the shortcut's id,
    /// so a rebinding in Settings ▸ Keymap follows it.
    pub intent: &'static str,
    /// The dock it brings forward.
    pub dock: u64,
    /// The menu row's label: the dock's own title, so the row and the panel it opens
    /// cannot be called two different things.
    pub title: fn() -> LocalizedString,
    /// The command's name in the command palette and the keymap.
    pub name: fn() -> LocalizedString,
    /// The letter of its Ctrl+Shift chord.
    pub chord: Key,
}

/// Every panel, in [`super::APP_DOCKS`] order: left side, right side, bottom.
pub(crate) const PANELS: &[PanelCommand] = &[
    PanelCommand {
        intent: "binder.show",
        dock: OUTLINE_DOCK_ID,
        title: || tr!(binder()),
        name: || tr!(shortcut_name_binder_show()),
        chord: Key::B,
    },
    PanelCommand {
        intent: "search.show",
        dock: SEARCH_DOCK_ID,
        title: || tr!(search()),
        name: || tr!(shortcut_name_search_show()),
        chord: Key::F,
    },
    PanelCommand {
        intent: "trash.show",
        dock: TRASH_DOCK_ID,
        title: || tr!(trash_title()),
        name: || tr!(shortcut_name_trash_show()),
        chord: Key::T,
    },
    PanelCommand {
        intent: "comments.show",
        dock: COMMENTS_DOCK_ID,
        title: || tr!(comments_title()),
        name: || tr!(shortcut_name_comments_show()),
        chord: Key::C,
    },
    PanelCommand {
        intent: "games.show",
        dock: GAMES_DOCK_ID,
        title: || tr!(games_title()),
        name: || tr!(shortcut_name_games_show()),
        chord: Key::G,
    },
    PanelCommand {
        intent: "inspector.show",
        dock: INSPECTOR_DOCK_ID,
        title: || tr!(inspector()),
        name: || tr!(shortcut_name_inspector_show()),
        chord: Key::I,
    },
    PanelCommand {
        intent: "format.show",
        dock: FORMAT_DOCK_ID,
        title: || tr!(format_dock_title()),
        name: || tr!(shortcut_name_format_show()),
        chord: Key::O,
    },
    PanelCommand {
        intent: "comments.document.show",
        dock: DOC_COMMENTS_DOCK_ID,
        title: || tr!(comments_document_title()),
        name: || tr!(shortcut_name_document_comments_show()),
        chord: Key::D,
    },
    PanelCommand {
        intent: "footnotes.show",
        dock: FOOTNOTES_DOCK_ID,
        title: || tr!(footnotes_title()),
        name: || tr!(shortcut_name_footnotes_show()),
        chord: Key::K,
    },
    PanelCommand {
        intent: "versions.show",
        dock: VERSIONS_DOCK_ID,
        title: || tr!(versions_title()),
        name: || tr!(shortcut_name_versions_show()),
        chord: Key::R,
    },
    PanelCommand {
        intent: "preview.show",
        dock: PREVIEW_DOCK_ID,
        title: || tr!(search_preview()),
        name: || tr!(shortcut_name_preview_show()),
        chord: Key::J,
    },
    PanelCommand {
        intent: "timeline.show",
        dock: TIMELINE_DOCK_ID,
        title: || tr!(timeline_title()),
        name: || tr!(shortcut_name_timeline_show()),
        chord: Key::L,
    },
];

/// Which side `dock` mounts on, for grouping the menu. `Leading` for an id not in
/// [`super::APP_DOCKS`], which the test below makes unreachable.
pub(crate) fn side_of(dock: u64) -> DockSide {
    APP_DOCKS
        .iter()
        .find(|d| d.id == dock)
        .map(|d| d.side)
        .unwrap_or(DockSide::Leading)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn every_app_dock_has_exactly_one_panel_command() {
        let docks: Vec<u64> = APP_DOCKS.iter().map(|d| d.id).collect();
        let panels: Vec<u64> = PANELS.iter().map(|p| p.dock).collect();
        assert_eq!(
            panels, docks,
            "View ▸ Panels must list every app dock once, in APP_DOCKS order"
        );
    }

    #[test]
    fn no_two_panels_share_a_chord_or_an_intent() {
        let chords: HashSet<String> = PANELS.iter().map(|p| format!("{:?}", p.chord)).collect();
        assert_eq!(chords.len(), PANELS.len(), "two panels share a chord");
        let intents: HashSet<&str> = PANELS.iter().map(|p| p.intent).collect();
        assert_eq!(intents.len(), PANELS.len(), "two panels share an intent");
    }

    /// Every panel command reaches the command palette, which lists only the app's own
    /// namespaces: three of them once had a chord and no palette row.
    #[test]
    fn every_panel_command_is_in_the_palette() {
        for panel in PANELS {
            assert!(
                crate::commands_ext::is_palette_command(panel.intent),
                "{} is not a palette command",
                panel.intent
            );
        }
    }

    /// The letters this module's doc says are taken or avoided stay out of the table.
    #[test]
    fn no_panel_takes_a_letter_the_app_already_uses() {
        for taken in [
            Key::E,
            Key::H,
            Key::M,
            Key::N,
            Key::P,
            Key::V,
            Key::Z,
            Key::U,
            Key::S,
            Key::W,
            Key::Q,
            Key::Y,
        ] {
            assert!(
                PANELS.iter().all(|p| p.chord != taken),
                "{taken:?} is taken or avoided"
            );
        }
    }
}
