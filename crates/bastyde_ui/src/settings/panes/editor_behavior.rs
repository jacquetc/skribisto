// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Editor Behavior — the non-typographic writing settings.
//!
//! Distraction-free mode's own settings used to live here as a group, while its
//! typography lived on a separate page. Nothing could then be handed whole to
//! the mode's quick-access popover, and a writer looking for "the
//! distraction-free settings" had to know to look in two places. They are all on
//! `panes::distraction_free` now.

use bastyde::prelude::*;
use bastyde::widgets::ComboBox;
use bastyde::widgets::tooltip::TooltipContent;

#[allow(unused_imports)]
use super::super::*;

/// Display name for each pinned-line position.
fn typewriter_anchor_label(anchor: &TypewriterAnchor) -> LocalizedString {
    match anchor {
        TypewriterAnchor::TopThird => tr!(settings_typewriter_position_top_third()),
        TypewriterAnchor::Middle => tr!(settings_typewriter_position_middle()),
        TypewriterAnchor::BottomQuarter => tr!(settings_typewriter_position_bottom_quarter()),
    }
}

/// Editor ▸ Editor Behavior — the non-typographic writing settings: the
/// centered-column width, the writing-view toggles, and the container-view
/// memory.
pub(in crate::settings) fn editor_behavior_pane(vm: &SettingsViewModel) -> impl Widget {
    let form = FormLayout::new()
        .label(tr!(settings_page_editor_behavior()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_writing_column())))
        .line(
            field_label(tr!(settings_text_width())),
            slider_field(vm.column_width(), 400.0, 1200.0, 20.0, |v| {
                format!("{} px", v.round() as i32)
            }),
        )
        .line(
            field_label(tr!(settings_preview_width())),
            slider_field(vm.preview_width(), 400.0, 1200.0, 20.0, |v| {
                format!("{} px", v.round() as i32)
            }),
        )
        // These three are general writing-surface options and apply in every
        // mode. They carry their own group heading so they cannot be read as
        // belonging to the "Distraction-free" one below — which is exactly how
        // they rendered before, the group heading having been inserted above
        // them when the distraction-free column width was added.
        .full_width(group(tr!(settings_group_writing_view())))
        .full_width(Toggle::new(vm.synopsis_pane()).label(tr!(settings_synopsis_pane())))
        .full_width(
            Toggle::new(vm.typewriter())
                .label(tr!(settings_typewriter()))
                .rich_tooltip_content(TooltipContent::new(
                    "settings.typewriter",
                    tr!(settings_typewriter_tip()),
                )),
        )
        // Where the pinned line sits. Presets rather than a percentage slider,
        // following Scrivener and Ulysses. Disabled — not hidden — while the
        // toggle above is off, so the choice stays visible as part of what the
        // feature offers instead of appearing out of nowhere when it is enabled.
        .line(
            field_label(tr!(settings_typewriter_position())),
            FixedSize::new().width(240.0).child(
                ComboBox::from_items(
                    TypewriterAnchor::all(),
                    vm.typewriter_anchor(),
                    typewriter_anchor_label,
                )
                .enabled(vm.typewriter()),
            ),
        )
        .full_width(Toggle::new(vm.highlight_sentence()).label(tr!(settings_highlight_sentence())))
        .full_width(group(tr!(settings_group_container_views())))
        .full_width(
            Toggle::new(vm.remember_view())
                .label(tr!(settings_remember_view()))
                .rich_tooltip_content(
                    TooltipContent::new(
                        "settings.remember_view",
                        tr!(settings_remember_view_tip()),
                    )
                    .with_more(tr!(settings_remember_view_tip_more())),
                ),
        );

    pane_frame(
        crumb(
            Some(tr!(settings_sec_editor())),
            tr!(settings_page_editor_behavior()),
        ),
        form,
    )
}
