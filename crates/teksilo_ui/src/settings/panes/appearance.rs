// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Appearance & Behaviour ▸ Appearance — interface language, theme, text size, welcome-at-startup.

use teksilo::prelude::*;
use teksilo::widgets::tooltip::TooltipContent;

#[allow(unused_imports)]
use super::super::*;

/// Appearance & Behaviour ▸ Appearance — interface language, the app-wide
/// **Theme** and **Interface text size** (both relocated here from the old
/// Manuscript pane, where "Editor theme"/"Text size" were misnomers for
/// app-wide controls), and the "show welcome at startup" preference.
pub(in crate::settings) fn appearance_pane(
    vm: &SettingsViewModel,
    scale: Signal<f32>,
) -> impl Widget {
    let form = FormLayout::new()
        .label(tr!(settings_page_appearance()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_language())))
        .line(
            field_label(tr!(settings_field_language())),
            FixedSize::new().width(240.0).child(LanguageSwitcher::new()),
        )
        .full_width(group(tr!(settings_group_theme())))
        .line(
            field_label(tr!(settings_field_app_theme())),
            FixedSize::new()
                .width(240.0)
                .child(ThemeSwitcher::new().system(true)),
        )
        .line(
            field_label(tr!(settings_field_text_scale())),
            FixedSize::new()
                .width(300.0)
                .child(TextScaleControl::new(scale)),
        )
        .full_width(group(tr!(settings_group_startup())))
        .full_width(
            Toggle::new(vm.show_welcome())
                .label(tr!(settings_show_welcome()))
                .rich_tooltip_content(TooltipContent::new(
                    "settings.show_welcome",
                    tr!(settings_show_welcome_tip()),
                )),
        );

    pane_frame(
        crumb(
            Some(tr!(settings_sec_appearance_behaviour())),
            tr!(settings_page_appearance()),
        ),
        form,
    )
}
