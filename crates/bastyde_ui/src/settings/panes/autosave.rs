// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Backup & Sync ▸ Autosave.

use bastyde::prelude::*;

#[allow(unused_imports)]
use super::super::*;

pub(in crate::settings) fn autosave_pane(vm: &SettingsViewModel) -> impl Widget {
    let form = FormLayout::new()
        .label(tr!(settings_page_autosave()))
        .label_gap(16.0)
        .row_spacing(12.0)
        .full_width(group(tr!(settings_group_autosave())))
        .full_width(Toggle::new(vm.autosave()).label(tr!(settings_autosave())))
        .full_width(hint(tr!(settings_autosave_hint())));

    pane_frame(
        crumb(
            Some(tr!(settings_sec_backup())),
            tr!(settings_page_autosave()),
        ),
        form,
    )
}
