// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Focus: the settings that keep the writer's eye on the line being written,
//! which are typewriter scrolling, the highlight around the caret, and the fade over the
//! rest of the text.
//!
//! Split off the old Editor Behavior page, where these six rows sat under a "Writing
//! view" heading beside the synopsis position and the large-image policy. Three of them
//! are disabled while the switch above them is off, and that reads clearly only on a
//! page where nothing else is in between.

use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{ComboBox, Segment, SegmentedControl};

use crate::shared::{DimScope, DimStrength};

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

/// Display name for each unit the caret fade keeps bright.
fn dim_scope_label(scope: DimScope) -> LocalizedString {
    match scope {
        DimScope::Sentence => tr!(settings_dim_scope_sentence()),
        DimScope::Paragraph => tr!(settings_dim_scope_paragraph()),
    }
}

/// What each unit does, as a per-segment tooltip, the shape the band's control uses.
fn dim_scope_tip(scope: DimScope) -> TooltipContent {
    let (key, text) = match scope {
        DimScope::Sentence => (
            "settings.dim_scope.sentence",
            tr!(settings_dim_scope_tip_sentence()),
        ),
        DimScope::Paragraph => (
            "settings.dim_scope.paragraph",
            tr!(settings_dim_scope_tip_paragraph()),
        ),
    };
    TooltipContent::new(key, text)
}

/// Display name for each fade strength.
fn dim_strength_label(strength: DimStrength) -> LocalizedString {
    match strength {
        DimStrength::Light => tr!(settings_dim_strength_light()),
        DimStrength::Medium => tr!(settings_dim_strength_medium()),
        DimStrength::Strong => tr!(settings_dim_strength_strong()),
    }
}

/// What each strength looks like, as a per-segment tooltip.
fn dim_strength_tip(strength: DimStrength) -> TooltipContent {
    let (key, text) = match strength {
        DimStrength::Light => (
            "settings.dim_strength.light",
            tr!(settings_dim_strength_tip_light()),
        ),
        DimStrength::Medium => (
            "settings.dim_strength.medium",
            tr!(settings_dim_strength_tip_medium()),
        ),
        DimStrength::Strong => (
            "settings.dim_strength.strong",
            tr!(settings_dim_strength_tip_strong()),
        ),
    };
    TooltipContent::new(key, text)
}

/// Bridge the caret band's stored scope to its `SegmentedControl`'s `usize` selection:
/// two guarded effects — the shape `goals_pane` uses for `CountingMethodSetting` and
/// `work_structure_pane` for `ChapterMode`. The `!=` guards are what stop the two signals
/// from writing each other back and forth for ever.
fn bridge_highlight_scope(ctx: &mut BuildContext, scope: Signal<HighlightScope>) -> Signal<usize> {
    let index: Signal<usize> = Signal::new(scope.get().to_index());
    {
        let index = index.clone();
        ctx.effect(&scope, move |s: &HighlightScope| {
            let i = s.to_index();
            if index.get() != i {
                index.set(i);
            }
        });
    }
    ctx.effect(&index, move |i| {
        let s = HighlightScope::from_index(*i);
        if scope.get() != s {
            scope.set(s);
        }
    });
    index
}

/// Bridge the stored fade scope to its `SegmentedControl`'s `usize`, the band's shape.
fn bridge_dim_scope(ctx: &mut BuildContext, scope: Signal<DimScope>) -> Signal<usize> {
    let index: Signal<usize> = Signal::new(scope.get().to_index());
    {
        let index = index.clone();
        ctx.effect(&scope, move |s: &DimScope| {
            let i = s.to_index();
            if index.get() != i {
                index.set(i);
            }
        });
    }
    ctx.effect(&index, move |i| {
        let s = DimScope::from_index(*i);
        if scope.get() != s {
            scope.set(s);
        }
    });
    index
}

/// Bridge the stored fade strength to its `SegmentedControl`'s `usize`, the same way.
fn bridge_dim_strength(ctx: &mut BuildContext, strength: Signal<DimStrength>) -> Signal<usize> {
    let index: Signal<usize> = Signal::new(strength.get().to_index());
    {
        let index = index.clone();
        ctx.effect(&strength, move |s: &DimStrength| {
            let i = s.to_index();
            if index.get() != i {
                index.set(i);
            }
        });
    }
    ctx.effect(&index, move |i| {
        let s = DimStrength::from_index(*i);
        if strength.get() != s {
            strength.set(s);
        }
    });
    index
}

/// Editor ▸ Focus: typewriter scrolling, the caret band and the fade.
pub(in crate::settings) fn focus_pane(
    ctx: &mut BuildContext,
    crumbs: &Crumbs,
    vm: &SettingsViewModel,
) -> impl Widget {
    let highlight_control = HighlightScope::all().into_iter().fold(
        SegmentedControl::indexed(bridge_highlight_scope(ctx, vm.highlight_scope())),
        |control, s| {
            control.segment(
                Segment::new(highlight_scope_label(s)).rich_tooltip_content(highlight_scope_tip(s)),
            )
        },
    );

    let dim_scope_control = DimScope::all().into_iter().fold(
        SegmentedControl::indexed(bridge_dim_scope(ctx, vm.dim_scope())),
        |control, s| {
            control.segment(Segment::new(dim_scope_label(s)).rich_tooltip_content(dim_scope_tip(s)))
        },
    );
    let dim_strength_control = DimStrength::all().into_iter().fold(
        SegmentedControl::indexed(bridge_dim_strength(ctx, vm.dim_strength())),
        |control, s| {
            control.segment(
                Segment::new(dim_strength_label(s)).rich_tooltip_content(dim_strength_tip(s)),
            )
        },
    );

    let form = FormLayout::new()
        .label(tr!(settings_page_focus()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_scrolling())))
        // In the label column with every other field on this page, not spanning
        // both: a `full_width` row starts at the pane's own left edge while a
        // `.line` field starts at `label_col + gap`, and mixing the two gives one
        // page two left edges. `export_styles`' editor settled this the same way.
        // `FormLayout::line` wires `access_labelled_by` itself, so
        // `labelled_externally` only tells the toggle's own assertion so.
        .line(
            field_label(tr!(settings_typewriter())),
            Toggle::new(vm.typewriter())
                .labelled_externally()
                .rich_tooltip_content(TooltipContent::new(
                    "settings.typewriter",
                    tr!(settings_typewriter_tip()),
                )),
        )
        // Where the pinned line sits. Presets rather than a percentage slider,
        // following the established convention. Disabled — not hidden — while the
        // toggle above is off, so the choice stays visible as part of what the
        // feature offers instead of appearing out of nowhere when it is enabled.
        .line(
            field_label(tr!(settings_typewriter_position())),
            ComboBox::from_items(
                TypewriterAnchor::all(),
                vm.typewriter_anchor(),
                typewriter_anchor_label,
            )
            .enabled(vm.typewriter()),
        )
        .full_width(group(tr!(settings_group_highlight_fade())))
        // How much of the text around the caret is shaded while you write. A three-way choice
        // rather than a toggle: a sentence is the unit you shape word by word, a paragraph the
        // one you shape whole, and neither subsumes the other.
        .line(
            field_label(tr!(settings_highlight_scope())),
            highlight_control,
        )
        // The fade: everything but the text being written drawn lighter. Its own switch
        // rather than a fourth segment above, because it combines with the band (a sentence
        // band over a paragraph fade) and because F8 flips it without forgetting the unit and
        // the strength below, which stay visible but disabled while it is off, as the
        // typewriter position does.
        .line(
            field_label(tr!(settings_dim_text())),
            Toggle::new(vm.dim_enabled())
                .labelled_externally()
                .rich_tooltip_content(TooltipContent::new(
                    "settings.dim_text",
                    tr!(settings_dim_text_tip()),
                )),
        )
        .line(
            field_label(tr!(settings_dim_scope())),
            dim_scope_control.enabled(vm.dim_enabled()),
        )
        .line(
            field_label(tr!(settings_dim_strength())),
            dim_strength_control.enabled(vm.dim_enabled()),
        );

    pane_frame(crumbs.of(Pane::Focus), form)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::core::{LayoutContext, LayoutResponse, Widget, WidgetId};

    /// Hosts [`bridge_highlight_scope`] in a real `BuildContext` and hands the index
    /// signal back, so a test can act as the `SegmentedControl` does.
    struct ScopeHost {
        scope: Signal<HighlightScope>,
        out: Rc<std::cell::RefCell<Option<Signal<usize>>>>,
    }
    impl std::fmt::Debug for ScopeHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ScopeHost").finish()
        }
    }
    impl Widget for ScopeHost {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            *self.out.borrow_mut() = Some(bridge_highlight_scope(ctx, self.scope.clone()));
            Vec::new()
        }
        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            proposal.resolve(0.0, 0.0).into()
        }
    }

    /// The caret band's bridge, moved out of the page body when this page was split
    /// off: a segment click writes the stored scope, and a scope written elsewhere
    /// (Reset, the View menu) moves the control.
    #[test]
    fn the_highlight_scope_round_trips_through_the_control() {
        let scope = Signal::new(HighlightScope::None);
        let out = Rc::new(std::cell::RefCell::new(None));
        let mut tree = WidgetTree::new();
        tree.add(ScopeHost {
            scope: scope.clone(),
            out: out.clone(),
        });
        tree.layout(SizeProposal::exact(100.0, 100.0));
        let index: Signal<usize> = out.borrow().clone().expect("the bridge was built");
        assert_eq!(index.get(), HighlightScope::None.to_index());

        index.set(HighlightScope::Paragraph.to_index());
        assert_eq!(scope.get(), HighlightScope::Paragraph);

        scope.set(HighlightScope::Sentence); // as Reset to defaults does
        assert_eq!(
            index.get(),
            HighlightScope::Sentence.to_index(),
            "the control must follow the setting"
        );
        drop(tree);
    }
}
