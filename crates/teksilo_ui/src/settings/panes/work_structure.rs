// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Work: `<name>` ▸ Structure — the open project's chapter encoding.

use frontend::common::entities::GoalUnit;
use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton};

#[allow(unused_imports)]
use super::super::*;

/// Work: `<name>` ▸ Structure — the per-project chapter storage mode, backed by
/// the shared `SingleWork` (entity-backed, undoable via the Work's stack).
pub(in crate::settings) fn work_structure_pane(
    ctx: &mut BuildContext,
    vm: &WorkSettingsViewModel,
    crumbs: &Crumbs,
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

    // ── Counting unit ────────────────────────────────────────────────────────
    //
    // Bridged through a **local** index rather than bound straight to the entity, unlike
    // its neighbours above. Switching the unit re-reads every target this project has
    // already been given, so the writer is asked first — and a control bound to the entity
    // would already be showing the new answer while the dialog was still open, then have
    // nothing to snap back to if they said no.
    let unit_index = Signal::new(crate::goals::unit_picker::index_of(&vm.goal_unit().get()));
    {
        // External changes (a refresh, an undo, a second window) re-seed the control.
        let vm = vm.clone();
        let unit_index = unit_index.clone();
        ctx.effect(&vm.goal_unit(), move |u| {
            let seeded = crate::goals::unit_picker::index_of(u);
            if unit_index.get() != seeded {
                unit_index.set(seeded);
            }
        });
    }

    let form = FormLayout::new()
        .label(tr!(settings_page_structure()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(crate::widgets::tip::RichTip::new(
            crate::tooltip_registry::CONCEPT_CHAPTER_MODE,
            group(tr!(settings_group_chapters())),
        ))
        // In the label column with every other field on this page. A `full_width`
        // row starts at the pane's own left edge while a `.line` field starts at
        // `label_col + gap`, so mixing the two gives one page two left edges;
        // `export_styles`' editor settled this the same way. `FormLayout::line`
        // wires `access_labelled_by` itself, so `labelled_externally` only tells
        // the toggle's own assertion so.
        .line(
            field_label(tr!(settings_chapter_flat())),
            Toggle::new(flat)
                .labelled_externally()
                .rich_tooltip_content(
                    TooltipContent::new("settings.chapter_flat", tr!(new_work_chapter_scene_tip()))
                        .with_more(tr!(new_work_chapter_scene_tip_more())),
                ),
        )
        .full_width(group(tr!(settings_group_numbering())))
        .line(
            field_label(tr!(settings_number_chapters())),
            Toggle::new(numbered)
                .labelled_externally()
                .rich_tooltip_content(
                    TooltipContent::new(
                        "settings.number_chapters",
                        tr!(settings_number_chapters_tip()),
                    )
                    .with_more(tr!(settings_number_chapters_tip_more())),
                ),
        )
        .full_width(group(tr!(settings_group_goal_unit())))
        .full_width(crate::widgets::tip::RichTip::new(
            crate::tooltip_registry::GOAL_UNIT,
            crate::goals::unit_picker::goal_unit_control(unit_index.clone(), {
                let vm = vm.clone();
                let index = unit_index.clone();
                move |chosen, ctx| {
                    let current = vm.goal_unit().get();
                    if chosen != current {
                        confirm_unit_switch(vm.clone(), index.clone(), current, chosen, ctx);
                    }
                }
            }),
        ))
        .line(
            field_label(tr!(settings_part_resets_chapter())),
            Toggle::new(part_resets)
                .labelled_externally()
                .rich_tooltip_content(
                    TooltipContent::new(
                        "settings.part_resets_chapter",
                        tr!(settings_part_resets_chapter_tip()),
                    )
                    .with_more(tr!(settings_part_resets_chapter_tip_more())),
                ),
        );

    pane_frame(crumbs.of(Pane::WorkStructure), form)
}

/// Ask before switching the unit, and revert the control if the answer is no.
///
/// Nothing is destroyed either way — both numbers survive, and switching back restores the
/// original reading — but every target already entered will be *read* as a different
/// length, which is a big enough change to a manuscript's plan to be worth a sentence.
///
/// Cancel is the default and the escape action: an accidental Enter must not reinterpret
/// every target in the project.
fn confirm_unit_switch(
    vm: WorkSettingsViewModel,
    index: Signal<usize>,
    from: GoalUnit,
    to: GoalUnit,
    ctx: &mut EventContext,
) {
    let label = |u: &GoalUnit| match u {
        GoalUnit::Words => tr!(goal_unit_words()),
        GoalUnit::Characters => tr!(goal_unit_characters()),
    };
    let revert = crate::goals::unit_picker::index_of(&from);
    MessageBox::warning(tr!(settings_goal_unit_switch_title()))
        .text(tr!(settings_goal_unit_switch_text(
            from = label(&from).resolve_now(),
            to = label(&to).resolve_now()
        )))
        .informative_text(tr!(settings_goal_unit_switch_informative()))
        .buttons(MessageBoxButtons::Custom(vec![
            MessageBoxButton::standard(StandardButton::Ok)
                .label(tr!(settings_goal_unit_switch_confirm())),
            MessageBoxButton::standard(StandardButton::Cancel),
        ]))
        .default_button(StandardButton::Cancel)
        .escape_button(StandardButton::Cancel)
        .on_result(move |r, _c| {
            if r.button == StandardButton::Ok {
                vm.set_goal_unit(to.clone());
            } else {
                // Nothing was written, so putting the control back is the whole revert.
                index.set(revert);
            }
        })
        .present(ctx);
}
