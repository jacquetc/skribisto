// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Editor Behavior — the non-typographic writing settings.
//!
//! Distraction-free mode's own settings used to live here as a group, while its
//! typography lived on a separate page. Nothing could then be handed whole to
//! the mode's quick-access popover, and a writer looking for "the
//! distraction-free settings" had to know to look in two places. They are all on
//! `panes::distraction_free` now.

use bastyde::prelude::*;
use bastyde::widgets::{ComboBox, Segment, SegmentedControl};
use bastyde::widgets::tooltip::TooltipContent;

#[allow(unused_imports)]
use super::super::*;

/// Display name for each pinned-line position.
fn typewriter_anchor_label(anchor: &TypewriterAnchor) -> LocalizedString {
    match anchor {
        TypewriterAnchor::TopThird => tr!(settings_typewriter_position_top_third()),
        TypewriterAnchor::Middle => tr!(settings_typewriter_position_middle()),
        TypewriterAnchor::BottomQuarter => tr!(settings_typewriter_position_bottom_quarter()),
    }
}

/// Display name for each caret-band scope.
fn highlight_scope_label(scope: HighlightScope) -> LocalizedString {
    match scope {
        HighlightScope::None => tr!(settings_highlight_scope_none()),
        HighlightScope::Sentence => tr!(settings_highlight_scope_sentence()),
        HighlightScope::Paragraph => tr!(settings_highlight_scope_paragraph()),
    }
}

/// What each scope does, as a per-segment tooltip — the self-documenting shape the backup
/// pane's retention control uses, and the reason the three choices need no prose beside them.
fn highlight_scope_tip(scope: HighlightScope) -> TooltipContent {
    let (key, text) = match scope {
        HighlightScope::None => (
            "settings.highlight_scope.none",
            tr!(settings_highlight_scope_tip_none()),
        ),
        HighlightScope::Sentence => (
            "settings.highlight_scope.sentence",
            tr!(settings_highlight_scope_tip_sentence()),
        ),
        HighlightScope::Paragraph => (
            "settings.highlight_scope.paragraph",
            tr!(settings_highlight_scope_tip_paragraph()),
        ),
    };
    TooltipContent::new(key, text)
}

/// Editor ▸ Editor Behavior — the non-typographic writing settings: the
/// centered-column width, the writing-view toggles, and the container-view
/// memory.
pub(in crate::settings) fn editor_behavior_pane(
    ctx: &mut BuildContext,
    vm: &SettingsViewModel,
) -> impl Widget {
    // The caret band's scope ⟷ the `SegmentedControl`'s `usize` selection, bridged by two
    // guarded effects — the shape `goals_pane` uses for `CountingMethodSetting` and
    // `work_structure_pane` for `ChapterMode`. The `!=` guards are what stop the two signals
    // from writing each other back and forth forever.
    let scope = vm.highlight_scope();
    let scope_index: Signal<usize> = Signal::new(scope.get().to_index());
    {
        let scope_index = scope_index.clone();
        ctx.effect(&scope, move |s| {
            let i = s.to_index();
            if scope_index.get() != i {
                scope_index.set(i);
            }
        });
    }
    {
        let scope = scope.clone();
        ctx.effect(&scope_index, move |i| {
            let s = HighlightScope::from_index(*i);
            if scope.get() != s {
                scope.set(s);
            }
        });
    }
    let highlight_control = HighlightScope::all().into_iter().fold(
        SegmentedControl::new(scope_index),
        |control, s| {
            control.segment(
                Segment::new(highlight_scope_label(s)).rich_tooltip_content(highlight_scope_tip(s)),
            )
        },
    );

    let form = FormLayout::new()
        .label(tr!(settings_page_editor_behavior()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_writing_column())))
        .line(
            field_label(tr!(settings_text_width())),
            slider_field(vm.column_width(), 400.0, 1200.0, 20.0, |v| {
                format!("{} px", v.round() as i32)
            }),
        )
        .line(
            field_label(tr!(settings_preview_width())),
            slider_field(vm.preview_width(), 400.0, 1200.0, 20.0, |v| {
                format!("{} px", v.round() as i32)
            }),
        )
        // These three are general writing-surface options and apply in every
        // mode. They carry their own group heading so they cannot be read as
        // belonging to the "Distraction-free" one below — which is exactly how
        // they rendered before, the group heading having been inserted above
        // them when the distraction-free column width was added.
        .full_width(group(tr!(settings_group_writing_view())))
        .full_width(Toggle::new(vm.synopsis_pane()).label(tr!(settings_synopsis_pane())))
        .full_width(
            Toggle::new(vm.typewriter())
                .label(tr!(settings_typewriter()))
                .rich_tooltip_content(TooltipContent::new(
                    "settings.typewriter",
                    tr!(settings_typewriter_tip()),
                )),
        )
        // Where the pinned line sits. Presets rather than a percentage slider,
        // following Scrivener and Ulysses. Disabled — not hidden — while the
        // toggle above is off, so the choice stays visible as part of what the
        // feature offers instead of appearing out of nowhere when it is enabled.
        .line(
            field_label(tr!(settings_typewriter_position())),
            FixedSize::new().width(240.0).child(
                ComboBox::from_items(
                    TypewriterAnchor::all(),
                    vm.typewriter_anchor(),
                    typewriter_anchor_label,
                )
                .enabled(vm.typewriter()),
            ),
        )
        // How much of the text around the caret is shaded while you write. A three-way choice
        // rather than a toggle: a sentence is the unit you shape word by word, a paragraph the
        // one you shape whole, and neither subsumes the other.
        .line(
            field_label(tr!(settings_highlight_scope())),
            highlight_control,
        )
        .full_width(group(tr!(settings_group_container_views())))
        .full_width(
            Toggle::new(vm.remember_view())
                .label(tr!(settings_remember_view()))
                .rich_tooltip_content(
                    TooltipContent::new(
                        "settings.remember_view",
                        tr!(settings_remember_view_tip()),
                    )
                    .with_more(tr!(settings_remember_view_tip_more())),
                ),
        );

    pane_frame(
        crumb(
            Some(tr!(settings_sec_editor())),
            tr!(settings_page_editor_behavior()),
        ),
        form,
    )
}
