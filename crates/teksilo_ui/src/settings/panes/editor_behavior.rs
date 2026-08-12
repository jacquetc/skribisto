// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Editor Behavior — the non-typographic writing settings. Distraction-free
//! mode's own settings (typography, column width, strip toggles) live on
//! `panes::distraction_free` instead, so its quick-access popover has one body to reuse.

use teksilo::prelude::*;
use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{ComboBox, Segment, SegmentedControl};

use crate::shared::SynopsisPlacement;

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

/// The synopsis control's three choices — one control over the two settings it
/// writes (whether there is a synopsis, and where it goes).
///
/// A checkbox plus a two-way placement control encoded three choices in four
/// states, one of which ("hidden, but beside") means nothing — and the checkbox's
/// label had to claim a position, so it read wrongly the moment the synopsis could
/// sit anywhere else.
const SYNOPSIS_CHOICES: [Option<SynopsisPlacement>; 3] = [
    Option::None,
    Some(SynopsisPlacement::Top),
    Some(SynopsisPlacement::Side),
];

fn synopsis_choice_label(choice: Option<SynopsisPlacement>) -> LocalizedString {
    match choice {
        Option::None => tr!(settings_synopsis_placement_none()),
        Some(SynopsisPlacement::Top) => tr!(settings_synopsis_placement_top()),
        Some(SynopsisPlacement::Side) => tr!(settings_synopsis_placement_side()),
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

/// Which segment a `(shown, placement)` pair selects.
fn synopsis_choice_of(shown: bool, placement: SynopsisPlacement) -> usize {
    if shown { placement.to_index() + 1 } else { 0 }
}

/// Bridge the synopsis `SegmentedControl`'s index to the **two** settings it writes —
/// whether there is a synopsis, and where it goes — and return the index signal.
///
/// The two stay separate because distraction-free mode needs the boolean axis on its own
/// (its strip toggle shows and hides; placement there is always Beside). Only the control
/// is unified; the stored state stays two answers.
///
/// Unlike the caret band's single-enum bridge, this index maps to a *pair* that cannot be
/// written atomically: a click changing both (e.g. off → "Side") sets `shown` first, which
/// would re-enter `sync` on the half-applied pair and briefly publish a selection nobody
/// chose. `settling` suppresses that re-entrant sync — the same guard `SideSync::suppress`
/// and `WidthProbe::last_mode` use elsewhere in this feature.
fn bridge_synopsis_choice(
    ctx: &mut BuildContext,
    shown: Signal<bool>,
    placement: Signal<SynopsisPlacement>,
) -> Signal<usize> {
    let choice_index: Signal<usize> = Signal::new(synopsis_choice_of(shown.get(), placement.get()));
    let settling = std::rc::Rc::new(std::cell::Cell::new(false));
    {
        let sync = {
            let (choice_index, placement, shown, settling) = (
                choice_index.clone(),
                placement.clone(),
                shown.clone(),
                settling.clone(),
            );
            std::rc::Rc::new(move || {
                if settling.get() {
                    return;
                }
                let want = synopsis_choice_of(shown.get(), placement.get());
                if choice_index.get() != want {
                    choice_index.set(want);
                }
            })
        };
        let also = sync.clone();
        ctx.effect(&shown, move |_| also());
        ctx.effect(&placement, move |_| sync());
    }
    {
        ctx.effect(&choice_index, move |i| {
            settling.set(true);
            // "None" leaves the placement alone rather than resetting it, so
            // switching the synopsis back on returns it to the side it was on.
            let want_shown = *i > 0;
            if shown.get() != want_shown {
                shown.set(want_shown);
            }
            if let Some(p) = SYNOPSIS_CHOICES.get(*i).copied().flatten()
                && placement.get() != p
            {
                placement.set(p);
            }
            settling.set(false);
        });
    }
    choice_index
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
    let choice_index = bridge_synopsis_choice(ctx, vm.synopsis_pane(), vm.synopsis_placement());
    let synopsis_control = SYNOPSIS_CHOICES.into_iter().fold(
        SegmentedControl::indexed(choice_index),
        |control, choice| control.segment(Segment::new(synopsis_choice_label(choice))),
    );

    let highlight_control = HighlightScope::all().into_iter().fold(
        SegmentedControl::indexed(scope_index),
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
        // These three are general writing-surface options and apply in every mode; their
        // own group heading keeps them from reading as part of "Distraction-free" below.
        .full_width(group(tr!(settings_group_writing_view())))
        .line(
            field_label(tr!(settings_synopsis_placement())),
            synopsis_control,
        )
        .full_width(
            Toggle::new(vm.typewriter())
                .label(tr!(settings_typewriter()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::core::{LayoutContext, LayoutResponse, Widget, WidgetId};

    /// Registers the bridge inside a real `BuildContext` and hands the index signal back,
    /// so a test can act as the `SegmentedControl` does: write the index, observe the pair.
    struct BridgeHost {
        shown: Signal<bool>,
        placement: Signal<SynopsisPlacement>,
        out: Rc<std::cell::RefCell<Option<Signal<usize>>>>,
    }
    impl std::fmt::Debug for BridgeHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BridgeHost").finish()
        }
    }
    impl Widget for BridgeHost {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let index = bridge_synopsis_choice(ctx, self.shown.clone(), self.placement.clone());
            *self.out.borrow_mut() = Some(index);
            Vec::new()
        }
        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            proposal.resolve(0.0, 0.0).into()
        }
    }

    /// A live bridge plus everything a test needs to drive and watch it.
    struct Bridge {
        shown: Signal<bool>,
        placement: Signal<SynopsisPlacement>,
        index: Signal<usize>,
        /// Writes to `index` *after* construction — that is, every write the bridge makes
        /// on top of the caller's own. A click is one; anything more is the write-back
        /// cascade the guard exists to stop.
        writes: Rc<Cell<usize>>,
        /// Both kept alive deliberately: `ctx.effect`'s observers are owned by the tree,
        /// so dropping it would quietly unregister the bridge under test and leave every
        /// assertion below passing against nothing.
        _tree: WidgetTree,
        _observer: teksilo::core::signal::ObserverHandle,
    }

    fn bridge(shown: bool, placement: SynopsisPlacement) -> Bridge {
        let shown = Signal::new(shown);
        let placement = Signal::new(placement);
        let out = Rc::new(std::cell::RefCell::new(None));
        let mut tree = WidgetTree::new();
        tree.add(BridgeHost {
            shown: shown.clone(),
            placement: placement.clone(),
            out: out.clone(),
        });
        tree.layout(SizeProposal::exact(100.0, 100.0));
        let index: Signal<usize> = out.borrow().clone().expect("the bridge was built");

        let writes = Rc::new(Cell::new(0usize));
        let w = writes.clone();
        let observer = index.observe(move |_| w.set(w.get() + 1));
        Bridge {
            shown,
            placement,
            index,
            writes,
            _tree: tree,
            _observer: observer,
        }
    }

    /// The one-axis clicks were never at risk; assert them anyway so the guard cannot be
    /// "fixed" by breaking the ordinary path.
    #[test]
    fn a_single_axis_click_applies_cleanly() {
        // Off → "Top": only `shown` changes.
        let b = bridge(false, SynopsisPlacement::Top);
        b.index.set(1);
        assert!(b.shown.get());
        assert_eq!(b.placement.get(), SynopsisPlacement::Top);
        assert_eq!(b.index.get(), 1);
        assert_eq!(b.writes.get(), 1, "only the click itself");
    }

    /// Turning the synopsis off must leave the remembered placement alone, so switching
    /// it back on returns it to the side it was on.
    #[test]
    fn turning_it_off_remembers_where_it_was() {
        let b = bridge(true, SynopsisPlacement::Side);
        b.index.set(0);
        assert!(!b.shown.get());
        assert_eq!(
            b.placement.get(),
            SynopsisPlacement::Side,
            "placement must survive the round trip"
        );
        b.index.set(2);
        assert!(b.shown.get());
        assert_eq!(b.placement.get(), SynopsisPlacement::Side);
    }

    /// The regression. Off → "Side" changes **both** settings in one click, and the pair
    /// cannot be written atomically: without the `settling` guard, `b.shown.set(true)`
    /// re-enters the sync before the placement lands, so the sync derives index 1 (the
    /// remembered Top) and writes it back — publishing a segment the writer never chose —
    /// before the placement write drags it to 2 again.
    #[test]
    fn a_two_axis_click_does_not_write_the_index_back() {
        let b = bridge(false, SynopsisPlacement::Top);

        b.index.set(2); // the writer clicks "Side" while the synopsis is off

        assert!(b.shown.get());
        assert_eq!(b.placement.get(), SynopsisPlacement::Side);
        assert_eq!(b.index.get(), 2, "the chosen segment must stick");
        assert_eq!(
            b.writes.get(),
            1,
            "the click is the only write; a higher count is the write-back cascade"
        );
    }

    /// The other two-axis case, reached from a remembered placement: off-with-Side → Top.
    #[test]
    fn a_two_axis_click_from_a_remembered_placement_is_also_clean() {
        let b = bridge(false, SynopsisPlacement::Side);

        b.index.set(1); // "Top", from off-but-remembering-Side

        assert!(b.shown.get());
        assert_eq!(b.placement.get(), SynopsisPlacement::Top);
        assert_eq!(b.index.get(), 1);
        assert_eq!(b.writes.get(), 1);
    }

    /// The guard must not deafen the bridge to changes made *elsewhere* — the
    /// distraction-free strip toggles `shown` on its own, and the control has to follow.
    #[test]
    fn an_external_change_still_moves_the_control() {
        let b = bridge(true, SynopsisPlacement::Side);
        assert_eq!(b.index.get(), 2);

        b.shown.set(false); // as the distraction-free strip toggle does
        assert_eq!(b.index.get(), 0, "the control must follow the setting");

        b.shown.set(true);
        assert_eq!(b.index.get(), 2, "…back to the remembered placement");
        assert_eq!(b.placement.get(), SynopsisPlacement::Side);
    }
}
