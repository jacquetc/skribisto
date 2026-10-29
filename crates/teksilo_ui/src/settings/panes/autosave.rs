// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Backup & Sync ▸ Autosave.

use teksilo::widgets::tooltip::TooltipContent;

#[allow(unused_imports)]
use super::super::*;

pub(in crate::settings) fn autosave_pane(crumbs: &Crumbs, vm: &SettingsViewModel) -> impl Widget {
    let form = FormLayout::new()
        .label(tr!(settings_page_autosave()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_autosave())))
        .line(
            field_label(tr!(settings_autosave())),
            Toggle::new(vm.autosave())
                .labelled_externally()
                .rich_tooltip_content(TooltipContent::new(
                    "settings.autosave",
                    tr!(settings_autosave_hint()),
                )),
        );

    pane_frame(crumbs.of(Pane::Autosave), form)
}
