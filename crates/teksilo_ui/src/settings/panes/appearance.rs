// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Appearance & Behaviour ▸ Appearance — interface language, theme, text size, welcome-at-startup.

use teksilo::widgets::tooltip::TooltipContent;

#[allow(unused_imports)]
use super::super::*;

/// Appearance & Behaviour ▸ Appearance — interface language, the app-wide
/// **Theme** and **Interface text size** (both relocated here from the old
/// Manuscript pane, where "Editor theme"/"Text size" were misnomers for
/// app-wide controls), and the "show welcome at startup" preference.
pub(in crate::settings) fn appearance_pane(
    crumbs: &Crumbs,
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
            LanguageSwitcher::new(),
        )
        .full_width(group(tr!(settings_group_theme())))
        .line(field_label(tr!(settings_field_app_theme())), {
            // Light / Dark / System — the same three entries as ever, but
            // built from the run's own design language rather than from
            // `ThemeSwitcher::new()`'s hardcoded IntUI pair. Under
            // `--style fluent` those defaults would match no active theme
            // (the combo shows nothing, since it matches by `ThemeId`) and
            // picking Light would drop the window out of Fluent for good.
            //
            // `.system(true)` keeps the follow-OS entry, which is the one
            // that does leave the style by design — see `crate::style`.
            ThemeSwitcher::new()
                .themes([
                    (tr!(settings_theme_light()), crate::style::light()),
                    (tr!(settings_theme_dark()), crate::style::dark()),
                ])
                .system(true)
        })
        .line(
            field_label(tr!(settings_field_text_scale())),
            TextScaleControl::new(scale),
        )
        .full_width(group(tr!(settings_group_startup())))
        // In the label column with every other field on the page, not spanning
        // both. A `full_width` row starts at the pane's own left edge while a
        // `.line` field starts at `label_col + gap`, so mixing the two gives one
        // page two left edges; `export_styles`' editor already settled this the
        // same way. `FormLayout::line` wires `access_labelled_by` itself, so
        // `labelled_externally` only tells the toggle's own assertion so — the
        // relation is pushed after `accessibility()` has already run.
        .line(
            field_label(tr!(settings_show_welcome())),
            Toggle::new(vm.show_welcome())
                .labelled_externally()
                .rich_tooltip_content(TooltipContent::new(
                    "settings.show_welcome",
                    tr!(settings_show_welcome_tip()),
                )),
        );

    pane_frame(crumbs.of(Pane::Appearance), form)
}
