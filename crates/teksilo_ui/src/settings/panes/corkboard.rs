// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Corkboard — card size, what a card shows, and how the grid behaves.

use teksilo::prelude::*;
use teksilo::widgets::tooltip::TooltipContent;

#[allow(unused_imports)]
use super::super::*;

/// Editor ▸ Corkboard — the card-board defaults: nested-vs-flat mode, card
/// size, and what a card shows. All store-backed, so a change fans out live to
/// every open board. The nested/flat radio bridges a `usize` selection to the
/// `corkboard_nested` bool via two guarded effects — the shape `goals_pane` uses.
pub(in crate::settings) fn corkboard_pane(
    ctx: &mut BuildContext,
    vm: &SettingsViewModel,
) -> impl Widget {
    let nested = vm.corkboard_nested();
    // 0 = Nested, 1 = Flat.
    let index: Signal<usize> = Signal::new(if nested.get() { 0 } else { 1 });
    {
        let index = index.clone();
        ctx.effect(&nested, move |n| {
            let i = if *n { 0 } else { 1 };
            if index.get() != i {
                index.set(i);
            }
        });
    }
    {
        let nested = nested.clone();
        ctx.effect(&index, move |i| {
            let n = *i == 0;
            if nested.get() != n {
                nested.set(n);
            }
        });
    }

    let typo = vm.corkboard_typo();
    let form = FormLayout::new()
        .label(tr!(settings_page_corkboard()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_corkboard_layout())))
        .full_width(
            RadioGroup::new()
                .radio(
                    RadioButton::new(0, index.clone())
                        .label(tr!(corkboard_view_nested()))
                        .rich_tooltip_content(TooltipContent::new(
                            "settings.corkboard_layout",
                            tr!(corkboard_layout_hint()),
                        )),
                )
                .radio(
                    RadioButton::new(1, index.clone())
                        .label(tr!(corkboard_view_flat()))
                        .rich_tooltip_content(TooltipContent::new(
                            "settings.corkboard_layout_flat",
                            tr!(corkboard_layout_hint()),
                        )),
                ),
        )
        .full_width(group(tr!(settings_group_corkboard_cards())))
        // Label lives on the FormLayout row (via `field_label` +
        // `access_labelled_by`); do not also put `.label()` on the Slider —
        // that would name the control twice for AT.
        .line(
            field_label(tr!(corkboard_card_size())),
            Slider::new(
                vm.corkboard_card_size(),
                crate::CORKBOARD_CARD_SIZE_MIN,
                crate::CORKBOARD_CARD_SIZE_MAX,
            )
            .step(crate::CORKBOARD_CARD_SIZE_STEP),
        )
        .full_width(
            Toggle::new(vm.corkboard_show_word_count()).label(tr!(corkboard_show_word_count())),
        )
        .full_width(
            Toggle::new(vm.corkboard_show_card_numbers()).label(tr!(corkboard_show_card_numbers())),
        )
        // The **expanded** editor's own size, beside the card's. The writer opens
        // "expand synopsis" to draft rather than to scan, so it is the one place on
        // the board where a comfortable reading size beats fitting a tile — which is
        // exactly why it cannot share the card's scale.
        .line(
            field_label(tr!(corkboard_modal_size())),
            // The same constants `CorkboardViewModel::modal_typo` gives the
            // expanded editor's bundle, so this slider and Ctrl+Wheel over that
            // editor agree on its wider ceiling.
            slider_field(
                vm.corkboard_modal_size(),
                crate::CORKBOARD_MODAL_SIZE_MIN,
                crate::CORKBOARD_MODAL_SIZE_MAX,
                crate::EDITOR_TYPO_SIZE_STEP,
                |v| format!("{:.0}%", v * 100.0),
            ),
        )
        // The card's own synopsis typography — mirrors the Scene / Synopsis / Notes
        // pages, so cards can read distinctly from the Full-Synopsis pane.
        .full_width(group(tr!(settings_group_typography())))
        .line(
            field_label(tr!(settings_field_typeface())),
            super::typography::font_picker(ctx, typo.font_family.clone()),
        )
        .line(
            field_label(tr!(settings_field_size())),
            slider_field(
                typo.size.clone(),
                typo.size_range.min,
                typo.size_range.max,
                typo.size_range.step,
                |v| format!("{:.0}%", v * 100.0),
            ),
        )
        .line(
            field_label(tr!(settings_field_line_height())),
            slider_field(typo.line_height.clone(), 1.0, 2.4, 0.02, |v| {
                format!("{v:.2}")
            }),
        )
        .line(
            field_label(tr!(settings_field_first_line_indent())),
            slider_field(typo.first_line_indent.clone(), 0.0, 60.0, 2.0, |v| {
                format!("{} px", v.round() as i32)
            }),
        )
        .line(
            field_label(tr!(settings_field_paragraph_spacing_before())),
            slider_field(typo.para_spacing_before.clone(), 0.0, 40.0, 2.0, |v| {
                format!("{} px", v.round() as i32)
            }),
        )
        .line(
            field_label(tr!(settings_field_paragraph_spacing_after())),
            slider_field(typo.para_spacing_after.clone(), 0.0, 40.0, 2.0, |v| {
                format!("{} px", v.round() as i32)
            }),
        );

    pane_frame(
        crumb(
            Some(tr!(settings_sec_editor())),
            tr!(settings_page_corkboard()),
        ),
        form,
    )
}
