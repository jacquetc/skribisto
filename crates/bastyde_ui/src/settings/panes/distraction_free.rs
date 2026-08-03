// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Distraction-free — everything the mode does differently, on one page:
//! typography (via the shared [`typography_rows`](super::typography::typography_rows)),
//! the mode's own column width, and which items its control strip keeps.
//!
//! [`distraction_free_form`] is that body. The popover reuses its *rows* rather
//! than the whole page — it is a picker you reach mid-sentence, not a settings
//! window — which is why the shared unit is a `FormLayout` you append to and not
//! a finished widget.

use bastyde::prelude::*;
use bastyde::widgets::tooltip::TooltipContent;

#[allow(unused_imports)]
use super::super::*;

/// Append every distraction-free setting to `form`: the typography bundle, the
/// mode's own column width, and which items its control strip keeps.
pub(in crate::settings) fn distraction_free_form(
    ctx: &mut BuildContext,
    form: FormLayout,
    vm: &SettingsViewModel,
    typo: &EditorTypography,
) -> FormLayout {
    let form = super::typography::typography_rows(ctx, form, typo);
    form
        // The mode's own column width — a flat pixel measure like the two on the
        // Editor Behavior page (not a character-count cap: `bastyde-text`'s
        // reachable surface has no horizontal glyph-advance metrics to fake
        // one), and its own setting so widening the normal Scene column can
        // never silently widen (or narrow) this one.
        .full_width(group(tr!(settings_group_writing_column())))
        .line(
            field_label(tr!(settings_field_column_width())),
            slider_field_tipped(
                vm.distraction_free_width(),
                400.0,
                1200.0,
                20.0,
                |v| format!("{} px", v.round() as i32),
                tr!(settings_distraction_free_width_hint()),
            ),
        )
        // Which items the control strip keeps. On = kept. There is deliberately
        // no Exit toggle — Exit always stays, and that promise is on the first
        // tip here so its absence is not read as an oversight.
        .full_width(group(tr!(settings_group_distraction_free_strip())))
        .full_width(
            Toggle::new(vm.distraction_free_title())
                .label(tr!(settings_distraction_free_title()))
                .rich_tooltip_content(TooltipContent::new(
                    "settings.df_chrome",
                    tr!(settings_distraction_free_chrome_hint()),
                )),
        )
        .full_width(
            Toggle::new(vm.distraction_free_word_count())
                .label(tr!(settings_distraction_free_word_count())),
        )
        .full_width(
            Toggle::new(vm.distraction_free_session())
                .label(tr!(settings_distraction_free_session())),
        )
        .full_width(
            Toggle::new(vm.distraction_free_go()).label(tr!(settings_distraction_free_go())),
        )
        .full_width(
            Toggle::new(vm.distraction_free_go_to()).label(tr!(settings_distraction_free_go_to())),
        )
}

/// The settings page.
pub(in crate::settings) fn distraction_free_pane(
    ctx: &mut BuildContext,
    vm: &SettingsViewModel,
    typo: &EditorTypography,
) -> impl Widget {
    let page = tr!(settings_page_distraction_free());
    let form = FormLayout::new()
        .label(page.clone())
        .label_gap(16.0)
        .row_spacing(14.0);
    let form = distraction_free_form(ctx, form, vm, typo);
    pane_frame(crumb(Some(tr!(settings_sec_editor())), page), form)
}
