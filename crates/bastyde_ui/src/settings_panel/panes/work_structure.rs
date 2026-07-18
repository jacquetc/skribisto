// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Work: `<name>` ▸ Structure — the open project's chapter encoding.

use bastyde::prelude::*;

#[allow(unused_imports)]
use super::super::*;

/// Work: `<name>` ▸ Structure — the per-project chapter storage mode, backed by
/// the shared `SingleWork` (entity-backed, undoable via the Work's stack). The
/// `Toggle` is bridged to `chapter_mode` (checked = flat) with two effects: one
/// mirrors external changes (refresh/undo) into the toggle, the other writes +
/// saves on a user toggle. Both guard on the current value to avoid a loop.
pub(in crate::settings_panel) fn work_structure_pane(
    ctx: &mut BuildContext,
    vm: &WorkSettingsViewModel,
    work_title: String,
) -> impl Widget {
    // The `Toggle` is bridged to the entity's `chapter_mode` with two effects: one mirrors
    // external changes (a refresh, an undo) into the toggle, the other pushes a user toggle
    // back. Both guard on the current value — without that they would drive each other in a
    // loop. The write-and-persist half is `WorkSettingsViewModel::set_flat_chapters`, which
    // owns its own no-op guard too, since this effect also fires on every rebuild.
    let mode = vm.chapter_mode();
    let flat: Signal<bool> = Signal::new(vm.flat_chapters());
    {
        let flat = flat.clone();
        ctx.effect(&mode, move |m| {
            let is_flat = matches!(m, ChapterMode::Flat);
            if flat.get() != is_flat {
                flat.set(is_flat);
            }
        });
    }
    {
        let vm = vm.clone();
        ctx.effect(&flat, move |f| vm.set_flat_chapters(*f));
    }

    let form = FormLayout::new()
        .label(tr!(settings_page_structure()))
        .label_gap(16.0)
        .row_spacing(12.0)
        .full_width(group(tr!(settings_group_chapters())))
        .full_width(Toggle::new(flat).label(tr!(settings_chapter_flat())))
        .full_width(hint(tr!(settings_chapter_flat_hint())));

    pane_frame(
        crumb(
            Some(lit!(format!(
                "{}: {}",
                tr!(settings_sec_work()).resolve_now(),
                work_title
            ))),
            tr!(settings_page_structure()),
        ),
        form,
    )
}
