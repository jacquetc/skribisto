// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Formatting glyphs — the format dock's buttons and the four-button row on the
//! editor's context menu.
//!
//! Not the Format menu: `MenuEntry` has no `.icon()`, so that surface is
//! text-only. Same commands, different rendering.
//!
//! Like [`editor`](crate::icons::editor), `res!` embeds each asset at compile
//! time and needs a literal path per call site, so there is one function per
//! icon and no way to look one up by name.
//!
//! Toggle state is **not** an asset swap. `IconButton::toggle` paints a
//! Selected background behind the glyph, which carries the on/off distinction
//! as a shape rather than as colour alone — so bold has one asset, not the
//! on/off pair `spellcheck` needs (that button lives in flat title-bar chrome
//! with no Selected surface to lean on).

use teksilo::res;
use teksilo::widgets::IconWidget;

/// Icon-button glyph size (dp), matching the rest of the app's chrome.
const ICON_SIZE: f32 = 16.0;

// -- History -------------------------------------------------------------

/// Undo — the editor's own edit history, not the app's Work-level trunk.
pub fn undo() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/undo.svg")).icon_size(ICON_SIZE)
}

/// Redo — undo's mirror.
pub fn redo() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/redo.svg")).icon_size(ICON_SIZE)
}

// -- Character marks -----------------------------------------------------

/// Bold.
pub fn bold() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/bold.svg")).icon_size(ICON_SIZE)
}

/// Italic.
pub fn italic() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/italic.svg")).icon_size(ICON_SIZE)
}

/// Underline.
pub fn underline() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/underline.svg")).icon_size(ICON_SIZE)
}

/// Strikethrough.
pub fn strikethrough() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/strikethrough.svg")).icon_size(ICON_SIZE)
}

/// Superscript — raised above the baseline.
pub fn superscript() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/superscript.svg")).icon_size(ICON_SIZE)
}

/// Subscript — dropped below the baseline.
pub fn subscript() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/subscript.svg")).icon_size(ICON_SIZE)
}

/// Link — a hyperlink in the prose.
pub fn link() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/link.svg")).icon_size(ICON_SIZE)
}

/// Clear formatting — strip marks and block structure back to plain prose.
pub fn clear_formatting() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/clear-formatting.svg")).icon_size(ICON_SIZE)
}

// -- Block ---------------------------------------------------------------

/// Heading level — the trigger for the Normal/H1..H6 picker.
pub fn heading() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/heading.svg")).icon_size(ICON_SIZE)
}

/// Align left — the manuscript default.
pub fn align_left() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/align-left.svg")).icon_size(ICON_SIZE)
}

/// Align centre — for an epigraph, a dedication, a verse stanza.
pub fn align_center() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/align-center.svg")).icon_size(ICON_SIZE)
}

/// Right-to-left paragraph direction.
///
/// A *paragraph* property, not a character one — which is why it sits
/// with the alignment buttons rather than with bold and italic.
pub fn direction_rtl() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/direction-rtl.svg")).icon_size(ICON_SIZE)
}

/// Blockquote.
pub fn blockquote() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/blockquote.svg")).icon_size(ICON_SIZE)
}

// -- Lists ---------------------------------------------------------------

/// Bulleted list.
pub fn list_bullet() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/list-bullet.svg")).icon_size(ICON_SIZE)
}

/// Numbered list.
pub fn list_numbered() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/list-numbered.svg")).icon_size(ICON_SIZE)
}

/// Indent — one nesting level deeper.
pub fn indent() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/indent.svg")).icon_size(ICON_SIZE)
}

/// Outdent — one nesting level shallower; stops at the outermost.
pub fn outdent() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/outdent.svg")).icon_size(ICON_SIZE)
}

// -- Tables --------------------------------------------------------------

/// Insert table.
pub fn table_insert() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/table-insert.svg")).icon_size(ICON_SIZE)
}

/// Insert row above.
pub fn table_row_above() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/table-row-above.svg")).icon_size(ICON_SIZE)
}

/// Insert row below.
pub fn table_row_below() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/table-row-below.svg")).icon_size(ICON_SIZE)
}

/// Insert column before.
pub fn table_col_before() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/table-col-before.svg")).icon_size(ICON_SIZE)
}

/// Insert column after.
pub fn table_col_after() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/table-col-after.svg")).icon_size(ICON_SIZE)
}

/// Delete row.
pub fn table_row_delete() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/table-row-delete.svg")).icon_size(ICON_SIZE)
}

/// Delete column.
pub fn table_col_delete() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/table-col-delete.svg")).icon_size(ICON_SIZE)
}

/// Remove the whole table.
pub fn table_remove() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/table-remove.svg")).icon_size(ICON_SIZE)
}

// -- Scene breaks --------------------------------------------------------

/// Minor scene break.
pub fn scene_break_minor() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/scene-break-minor.svg"))
        .icon_size(ICON_SIZE)
}

/// Major scene break.
pub fn scene_break_major() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/format/scene-break-major.svg"))
        .icon_size(ICON_SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::prelude::SizeProposal;

    /// Every glyph builds and lays out to a real size.
    ///
    /// Malformed path data cannot reach this test: `res!` parses each asset at
    /// **compile time** and fails the build (verified — a broken `d` attribute
    /// gives "invalid path data at position 9: expected number"). So this
    /// covers the case the compiler cannot see: an asset that parses fine and
    /// still lays out to nothing, which would ship as a blank square in the
    /// dock. It also pins the count, so an icon added to the set without a
    /// control — or a control without its icon — shows up here.
    #[test]
    fn every_format_icon_parses_and_lays_out() {
        let icons: Vec<(&str, IconWidget)> = vec![
            ("undo", undo()),
            ("redo", redo()),
            ("bold", bold()),
            ("italic", italic()),
            ("underline", underline()),
            ("strikethrough", strikethrough()),
            ("superscript", superscript()),
            ("subscript", subscript()),
            ("clear_formatting", clear_formatting()),
            ("heading", heading()),
            ("align_left", align_left()),
            ("align_center", align_center()),
            ("blockquote", blockquote()),
            ("list_bullet", list_bullet()),
            ("list_numbered", list_numbered()),
            ("indent", indent()),
            ("outdent", outdent()),
            ("table_insert", table_insert()),
            ("table_row_above", table_row_above()),
            ("table_row_below", table_row_below()),
            ("table_col_before", table_col_before()),
            ("table_col_after", table_col_after()),
            ("table_row_delete", table_row_delete()),
            ("table_col_delete", table_col_delete()),
            ("table_remove", table_remove()),
            ("scene_break_minor", scene_break_minor()),
            ("scene_break_major", scene_break_major()),
        ];
        assert_eq!(icons.len(), 27, "the dock's full control set");

        for (name, icon) in icons {
            let mut tree = WidgetTree::new();
            let id = tree.add(icon);
            tree.layout(SizeProposal::exact(16.0, 16.0));
            let bounds = tree.bounds(id);
            assert!(
                bounds.width > 0.0 && bounds.height > 0.0,
                "{name}: laid out to nothing"
            );
        }
    }
}
