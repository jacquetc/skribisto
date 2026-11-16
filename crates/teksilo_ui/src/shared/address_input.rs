// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A field that holds an address or a path: it reads left to right in every
//! interface.
//!
//! Every text field reads in the direction of its own text (teksilo's
//! `TextBaseDirection::Auto`): Arabic typed in a French window reads right to left,
//! and an empty field in an Arabic window opens with its caret on the right. That is
//! right for a title or a name, and wrong for an address. A browser keeps its
//! address bar left to right for the same reason: a path's structure (the root, the
//! separators, the extension) runs left to right whatever its folder names are
//! written in, and a field that opens on the right jumps to the left at the first
//! letter of `https`.
//!
//! Used by the link dialog's address and by the Manuskript and Scrivener importers'
//! source paths.

use teksilo::prelude::*;
use teksilo::widgets::{TextBaseDirection, TextInput};

/// A text input for an address or a path, left to right whatever it holds.
pub(crate) fn address_input(value: Signal<String>) -> TextInput {
    TextInput::new(value).base_direction(TextBaseDirection::LeftToRight)
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::LayoutDirection;
    use teksilo::core::accessibility::audit::text_infos;
    use teksilo::core::accesskit::TextDirection;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::widgets::VStack;

    /// A path whose folder names are Arabic, typed in an Arabic window.
    const ARABIC_PATH: &str = "روايات/الرواية.skrib";

    /// The directions the tree's text fields are announced with, in tree order.
    fn announced(field: impl Widget + 'static) -> Vec<TextDirection> {
        let mut tree = WidgetTree::new();
        tree.set_layout_direction(LayoutDirection::RightToLeft);
        tree.add(field);
        tree.layout(SizeProposal::exact(400.0, 100.0));
        let update = tree.sync_accessibility();
        let mut directions: Vec<_> = text_infos(&update)
            .into_values()
            .filter(|info| info.document_text == ARABIC_PATH)
            .filter_map(|info| info.direction)
            .collect();
        directions.dedup();
        directions
    }

    #[test]
    fn an_address_reads_left_to_right_in_a_right_to_left_window() {
        let field = address_input(Signal::new(ARABIC_PATH.to_string()));
        assert_eq!(
            announced(VStack::new().child(field)),
            vec![TextDirection::LeftToRight]
        );
    }

    /// The control: the same text in an ordinary field reads right to left, so the
    /// test above holds because of `address_input` and not by accident.
    #[test]
    fn the_same_text_in_an_ordinary_field_reads_right_to_left() {
        let field = TextInput::new(Signal::new(ARABIC_PATH.to_string()));
        assert_eq!(
            announced(VStack::new().child(field)),
            vec![TextDirection::RightToLeft]
        );
    }
}
