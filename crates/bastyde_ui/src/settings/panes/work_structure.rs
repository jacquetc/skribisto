// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Work: `<name>` ▸ Structure — the open project's chapter encoding.

use bastyde::prelude::*;
use bastyde::widgets::tooltip::TooltipContent;

#[allow(unused_imports)]
use super::super::*;

/// Work: `<name>` ▸ Structure — the per-project chapter storage mode, backed by
/// the shared `SingleWork` (entity-backed, undoable via the Work's stack).
pub(in crate::settings) fn work_structure_pane(
    ctx: &mut BuildContext,
    vm: &WorkSettingsViewModel,
    work_title: String,
) -> impl Widget {
    // The `Toggle` is bridged to the entity's `chapter_mode` (checked = flat) with two
    // guarded effects — one mirrors external changes (refresh/undo) in, the other pushes a
    // user toggle back via `WorkSettingsViewModel::set_flat_chapters` — to avoid a loop.
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

    // The two numbering settings, bridged the same guarded way as `flat` above: the
    // view-model's setters no-op when the value is unchanged, so the effect firing on every
    // rebuild costs neither an undo entry nor a disk save.
    let numbered = vm.number_chapters();
    {
        let vm = vm.clone();
        ctx.effect(&numbered, move |on| vm.set_number_chapters(*on));
    }
    let part_resets = vm.part_resets_chapter();
    {
        let vm = vm.clone();
        ctx.effect(&part_resets, move |on| vm.set_part_resets_chapter(*on));
    }

    let form = FormLayout::new()
        .label(tr!(settings_page_structure()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_chapters())))
        .full_width(
            Toggle::new(flat)
                .label(tr!(settings_chapter_flat()))
                .rich_tooltip_content(
                    TooltipContent::new("settings.chapter_flat", tr!(new_work_chapter_scene_tip()))
                        .with_more(tr!(new_work_chapter_scene_tip_more())),
                ),
        )
        .full_width(group(tr!(settings_group_numbering())))
        .full_width(
            Toggle::new(numbered)
                .label(tr!(settings_number_chapters()))
                .rich_tooltip_content(
                    TooltipContent::new(
                        "settings.number_chapters",
                        tr!(settings_number_chapters_tip()),
                    )
                    .with_more(tr!(settings_number_chapters_tip_more())),
                ),
        )
        .full_width(
            Toggle::new(part_resets)
                .label(tr!(settings_part_resets_chapter()))
                .rich_tooltip_content(
                    TooltipContent::new(
                        "settings.part_resets_chapter",
                        tr!(settings_part_resets_chapter_tip()),
                    )
                    .with_more(tr!(settings_part_resets_chapter_tip_more())),
                ),
        );

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
