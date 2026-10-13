// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which menu a keyboard shortcut is filed under.
//!
//! Settings ▸ Keymap, Help ▸ Keyboard shortcuts and the command palette group
//! shortcuts by their category. A shortcut is filed under the menu that holds
//! its command, so a writer who knows where a command sits in the menus finds
//! its chord in the same place.
//!
//! teksilo keeps the two halves of a category apart: the **key** orders and
//! groups, so it must not change with the language, and the **label** is what a
//! reader sees and searches. The keys here are numbered in menu-bar order so the
//! groups read Work, Edit, View… as the bar does, never alphabetically in
//! whichever language is showing. Nobody sees a key.

use teksilo::core::shortcut::{Shortcut, ShortcutBuilder};
use teksilo::prelude::*;

/// A menu of the project window's menu bar, as a shortcut group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShortcutGroup {
    Work,
    Edit,
    View,
    Document,
    Format,
    Go,
    Tools,
    Help,
}

impl ShortcutGroup {
    /// The stable key, in menu-bar order.
    pub(crate) fn key(self) -> &'static str {
        match self {
            ShortcutGroup::Work => "1-work",
            ShortcutGroup::Edit => "2-edit",
            ShortcutGroup::View => "3-view",
            ShortcutGroup::Document => "4-document",
            ShortcutGroup::Format => "5-format",
            ShortcutGroup::Go => "6-go",
            ShortcutGroup::Tools => "7-tools",
            ShortcutGroup::Help => "8-help",
        }
    }

    /// What the group's header says: the menu's own name, without its mnemonic.
    pub(crate) fn label(self) -> LocalizedString {
        match self {
            ShortcutGroup::Work => tr!(shortcut_group_work()),
            ShortcutGroup::Edit => tr!(shortcut_group_edit()),
            ShortcutGroup::View => tr!(shortcut_group_view()),
            ShortcutGroup::Document => tr!(shortcut_group_document()),
            ShortcutGroup::Format => tr!(shortcut_group_format()),
            ShortcutGroup::Go => tr!(shortcut_group_go()),
            ShortcutGroup::Tools => tr!(shortcut_group_tools()),
            ShortcutGroup::Help => tr!(shortcut_group_help()),
        }
    }
}

/// `.group(ShortcutGroup::Edit)` on a shortcut being built: sets both halves of
/// its category at once, so the two can never disagree.
pub(crate) trait ShortcutGroupExt {
    fn group(self, group: ShortcutGroup) -> Self;
}

impl ShortcutGroupExt for ShortcutBuilder {
    fn group(self, group: ShortcutGroup) -> Self {
        self.category(group.key()).category_label(group.label())
    }
}

/// The same for a shortcut someone else built — teksilo's standard edit
/// commands, re-filed under the menu that holds them.
impl ShortcutGroupExt for Shortcut {
    fn group(mut self, group: ShortcutGroup) -> Self {
        self.category = Some(group.key());
        self.category_label = Some(group.label().into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::shortcut::Shortcut;

    const ALL: [ShortcutGroup; 8] = [
        ShortcutGroup::Work,
        ShortcutGroup::Edit,
        ShortcutGroup::View,
        ShortcutGroup::Document,
        ShortcutGroup::Format,
        ShortcutGroup::Go,
        ShortcutGroup::Tools,
        ShortcutGroup::Help,
    ];

    #[test]
    fn the_keys_sort_in_menu_bar_order() {
        let keys: Vec<&str> = ALL.iter().map(|g| g.key()).collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        assert_eq!(keys, sorted);
    }

    #[test]
    fn a_grouped_shortcut_shows_its_label_not_its_key() {
        let sc = Shortcut::new("x").group(ShortcutGroup::Help).build();
        assert_eq!(sc.category, Some("8-help"));
        assert_eq!(sc.category_text(), Some("Help".to_string()));
    }

    /// The label is the menu's own title with its mnemonic taken out, so the
    /// two cannot drift apart in en-US.
    #[test]
    fn each_label_is_its_menu_title() {
        let titles = [
            tr!(menu_work()),
            tr!(menu_edit()),
            tr!(menu_view()),
            tr!(menu_document()),
            tr!(menu_format()),
            tr!(menu_go()),
            tr!(menu_tools()),
            tr!(menu_help()),
        ];
        for (group, title) in ALL.iter().zip(titles) {
            assert_eq!(
                group.label().resolve_now(),
                title.resolve_now().replace('&', ""),
                "{group:?}"
            );
        }
    }
}
