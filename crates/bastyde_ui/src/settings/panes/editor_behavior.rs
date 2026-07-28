// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Editor Behavior — the non-typographic writing settings.

use bastyde::prelude::*;

#[allow(unused_imports)]
use super::super::*;

/// Editor ▸ Editor Behavior — the non-typographic writing settings: the
/// centered-column width + the synopsis-pane / typewriter / highlight toggles,
/// plus distraction-free mode's own column width.
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
        // Distraction-free mode's own column width — a flat pixel measure like the
        // two above (not a character-count cap: `bastyde-text`'s reachable surface
        // has no horizontal glyph-advance metrics to fake one), kept as its own
        // group + setting so widening the normal Scene column can never silently
        // widen (or narrow) the distraction-free one.
        .full_width(group(tr!(settings_page_distraction_free())))
        .line(
            field_label(tr!(settings_field_column_width())),
            slider_field(vm.distraction_free_width(), 400.0, 1200.0, 20.0, |v| {
                format!("{} px", v.round() as i32)
            }),
        )
        .full_width(hint(tr!(settings_distraction_free_width_hint())))
        .full_width(Checkbox::new(vm.synopsis_pane()).label(tr!(settings_synopsis_pane())))
        .full_width(Checkbox::new(vm.typewriter()).label(tr!(settings_typewriter())))
        .full_width(
            Checkbox::new(vm.highlight_sentence()).label(tr!(settings_highlight_sentence())),
        )
        .full_width(group(tr!(settings_group_container_views())))
        .full_width(
            Checkbox::new(vm.remember_view())
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
