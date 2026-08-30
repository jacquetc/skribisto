// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Editor Behavior — the non-typographic writing settings. Distraction-free
//! mode's own settings (typography, column width, strip toggles) live on
//! `panes::distraction_free` instead, so its quick-access popover has one body to reuse.

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

/// The three values `editor.image_size_policy` takes, in the order they are
/// offered. The strings are the setting's own storage vocabulary — the same three
/// `app::commands::images::stored_policy` reads back — so they are literals here
/// rather than an enum: an enum would be a fourth place the vocabulary is written
/// down, and the two sides could then disagree silently.
const IMAGE_SIZE_POLICIES: [&str; 3] = ["ask", "keep", "downscale"];

/// Display name for each large-image policy.
fn image_policy_label(value: &str) -> LocalizedString {
    match value {
        "keep" => tr!(settings_image_policy_keep()),
        "downscale" => tr!(settings_image_policy_downscale()),
        _ => tr!(settings_image_policy_ask()),
    }
}

/// Which segment a stored policy selects.
///
/// Anything unrecognised lands on "Ask each time" — the same fallback
/// `app::commands::images::stored_policy` applies when it reads the key back, so
/// a value hand-written into `general.toml` cannot make the control disagree with
/// what the insert path will actually do.
fn image_policy_index(value: &str) -> usize {
    IMAGE_SIZE_POLICIES
        .iter()
        .position(|p| *p == value)
        .unwrap_or(0)
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

/// Bridge the stored `editor.image_size_policy` string to the
/// `SegmentedControl`'s `usize`, and return the index signal.
///
/// Two guarded effects, the same shape as the caret band's bridge: the `!=`
/// guards are what stop the two signals from writing each other back and forth
/// for ever. The stored form stays a string because it is the vocabulary the
/// *insert* path reads (`app::commands::images`), and a settings page must not
/// invent a second one.
fn bridge_image_policy(ctx: &mut BuildContext, policy: Signal<String>) -> Signal<usize> {
    let index: Signal<usize> = Signal::new(image_policy_index(&policy.get()));
    {
        let index = index.clone();
        ctx.effect(&policy, move |v: &String| {
            let i = image_policy_index(v);
            if index.get() != i {
                index.set(i);
            }
        });
    }
    {
        let policy = policy.clone();
        ctx.effect(&index, move |i| {
            let want = IMAGE_SIZE_POLICIES
                .get(*i)
                .copied()
                .unwrap_or(IMAGE_SIZE_POLICIES[0])
                .to_string();
            if policy.get() != want {
                policy.set(want);
            }
        });
    }
    index
}

/// Editor ▸ Editor Behavior — the non-typographic writing settings: the
/// centered-column width, the writing-view toggles, and the container-view
/// memory.
pub(in crate::settings) fn editor_behavior_pane(
    ctx: &mut BuildContext,
    crumbs: &Crumbs,
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
    // Through the view-model, not `ctx.settings()` directly: this row and the
    // Reset button must move the *same* signal, or a reset would put the store
    // back to `ask` while the segmented control kept showing what it had. The
    // accessor is lazy on purpose (it seeds the key only where it is about to be
    // written), so reading it here — inside a pane the writer opened — is the
    // moment it is meant to be read.
    let image_policy = vm.image_size_policy();
    let policy_index = bridge_image_policy(ctx, image_policy);
    let image_policy_control = IMAGE_SIZE_POLICIES
        .iter()
        .fold(SegmentedControl::indexed(policy_index), |control, value| {
            control.segment(Segment::new(image_policy_label(value)))
        });

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
        // How much of the text around the caret is shaded while you write. A three-way choice
        // rather than a toggle: a sentence is the unit you shape word by word, a paragraph the
        // one you shape whole, and neither subsumes the other.
        .line(
            field_label(tr!(settings_highlight_scope())),
            highlight_control,
        )
        // The way back out of a one-way door. The insert-an-image prompt offers
        // "don't ask again", and ticking it writes `keep` or `downscale` into
        // `editor.image_size_policy` for good — after which every later insert is
        // silently handled that way and *no page in the window said so*, which
        // left hand-editing `general.toml` as the only undo.
        .line(
            field_label(tr!(settings_field_image_size_policy())),
            image_policy_control,
        )
        .full_width(hint(tr!(settings_hint_image_size_policy())))
        .full_width(group(tr!(settings_group_container_views())))
        .line(
            field_label(tr!(settings_remember_view())),
            Toggle::new(vm.remember_view())
                .labelled_externally()
                .rich_tooltip_content(
                    TooltipContent::new(
                        "settings.remember_view",
                        tr!(settings_remember_view_tip()),
                    )
                    .with_more(tr!(settings_remember_view_tip_more())),
                ),
        );

    pane_frame(crumbs.of(Pane::EditorBehavior), form)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::core::{LayoutContext, LayoutResponse, Widget, WidgetId};

    /// Hosts [`bridge_image_policy`] in a real `BuildContext` and hands the index
    /// signal back, so a test can act as the `SegmentedControl` does.
    struct PolicyHost {
        policy: Signal<String>,
        out: Rc<std::cell::RefCell<Option<Signal<usize>>>>,
    }
    impl std::fmt::Debug for PolicyHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PolicyHost").finish()
        }
    }
    impl Widget for PolicyHost {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            *self.out.borrow_mut() = Some(bridge_image_policy(ctx, self.policy.clone()));
            Vec::new()
        }
        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            proposal.resolve(0.0, 0.0).into()
        }
    }

    /// A live policy bridge. The tree is kept alive deliberately: `ctx.effect`'s
    /// observers are owned by it, and dropping it would unregister the bridge
    /// under test and leave every assertion passing against nothing.
    struct Policy {
        policy: Signal<String>,
        index: Signal<usize>,
        _tree: WidgetTree,
    }

    fn policy_bridge(stored: &str) -> Policy {
        let policy = Signal::new(stored.to_string());
        let out = Rc::new(std::cell::RefCell::new(None));
        let mut tree = WidgetTree::new();
        tree.add(PolicyHost {
            policy: policy.clone(),
            out: out.clone(),
        });
        tree.layout(SizeProposal::exact(100.0, 100.0));
        let index = out.borrow().clone().expect("the bridge was built");
        Policy {
            policy,
            index,
            _tree: tree,
        }
    }

    /// The vocabulary this page writes is the one the *insert* path reads.
    ///
    /// `editor.image_size_policy` had no page at all: the insert prompt's "don't
    /// ask again" box wrote `keep` or `downscale` and nothing in the window could
    /// write it back. A page that offered its own spelling of those three would
    /// have been worse than none — it would look like an undo and change nothing.
    #[test]
    fn the_offered_policies_are_the_ones_the_insert_path_stores() {
        assert_eq!(IMAGE_SIZE_POLICIES, ["ask", "keep", "downscale"]);
    }

    /// Picking a segment writes the stored string, in both directions.
    #[test]
    fn the_large_image_policy_round_trips_through_the_control() {
        let b = policy_bridge("ask");
        assert_eq!(b.index.get(), 0);

        b.index.set(2); // the writer picks "Optimise"
        assert_eq!(b.policy.get(), "downscale");

        b.index.set(1);
        assert_eq!(b.policy.get(), "keep");

        // …and a value written from elsewhere (the insert prompt's "don't ask
        // again" box) moves the control.
        b.policy.set("ask".to_string());
        assert_eq!(b.index.get(), 0, "the control must follow the setting");
    }

    /// A value nothing recognises reads as "ask" — the same fallback
    /// `stored_policy` applies — so a hand-edited `general.toml` cannot make the
    /// page disagree with what an insert will actually do.
    #[test]
    fn an_unrecognised_stored_policy_shows_as_ask() {
        assert_eq!(image_policy_index("shrink-a-bit"), 0);
        assert_eq!(image_policy_index(""), 0);
        let b = policy_bridge("shrink-a-bit");
        assert_eq!(b.index.get(), 0);
    }

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
