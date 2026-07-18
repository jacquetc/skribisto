// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Spelling ▸ Spell-checking — the app-wide master switch.

use bastyde::prelude::*;

#[allow(unused_imports)]
use super::super::*;

/// Backup & Sync ▸ Autosave — migrates the autosave preference.
/// Settings ▸ Spelling ▸ Spell-checking — the app-wide master switch, the same
/// `SPELLCHECK_ENABLED_KEY` the title-bar toggle / View menu / F7 drive. A plain
/// store-backed `Toggle`: writing the signal persists, and `App::build`'s effect turns it
/// into `set_enabled` + a re-attach. No per-language controls here — those live on the
/// Work's / an item's Language field (the hint says so).
pub(in crate::settings_panel) fn spellcheck_pane(vm: &SettingsViewModel) -> impl Widget {
    let form = FormLayout::new()
        .label(tr!(settings_page_spellcheck()))
        .label_gap(16.0)
        .row_spacing(12.0)
        .full_width(group(tr!(settings_group_spellcheck())))
        .full_width(
            Toggle::new(vm.spellcheck_enabled()).label(tr!(settings_spellcheck_enabled())),
        )
        .full_width(hint(tr!(settings_spellcheck_hint())));

    pane_frame(
        crumb(
            Some(tr!(settings_sec_spelling())),
            tr!(settings_page_spellcheck()),
        ),
        form,
    )
}
