// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Goals — word/character targets and how they are counted.

use bastyde::prelude::*;

#[allow(unused_imports)]
use super::super::*;

/// Editor ▸ Goals — the word-**counting method** (a global USER preference that
/// drives only the live status-bar count; the canonical progress snapshot always
/// counts with `Auto`) and whether the status bar shows characters beside words.
/// The 4-variant method is bridged to the `RadioGroup`'s `usize` selection with two
/// guarded effects — the shape `work_structure_pane` uses for `ChapterMode`.
pub(in crate::settings) fn goals_pane(ctx: &mut BuildContext, vm: &SettingsViewModel) -> impl Widget {
    let method = vm.counting_method();
    let index: Signal<usize> = Signal::new(method_to_index(method.get()));
    {
        let index = index.clone();
        ctx.effect(&method, move |m| {
            let i = method_to_index(*m);
            if index.get() != i {
                index.set(i);
            }
        });
    }
    {
        let method = method.clone();
        ctx.effect(&index, move |i| {
            let m = index_to_method(*i);
            if method.get() != m {
                method.set(m);
            }
        });
    }

    let form = FormLayout::new()
        .label(tr!(settings_page_goals()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_counting())))
        .full_width(
            RadioGroup::new()
                .radio(RadioButton::new(0, index.clone()).label(tr!(settings_counting_auto())))
                .radio(
                    RadioButton::new(1, index.clone())
                        .label(tr!(settings_counting_whitespace())),
                )
                .radio(
                    RadioButton::new(2, index.clone())
                        .label(tr!(settings_counting_unicode_words())),
                )
                .radio(
                    RadioButton::new(3, index.clone())
                        .label(tr!(settings_counting_cjk_hybrid())),
                ),
        )
        .full_width(hint(tr!(settings_counting_hint())))
        .full_width(group(tr!(settings_group_goals_display())))
        .full_width(Toggle::new(vm.show_characters()).label(tr!(settings_show_characters())));

    pane_frame(
        crumb(Some(tr!(settings_sec_editor())), tr!(settings_page_goals())),
        form,
    )
}
