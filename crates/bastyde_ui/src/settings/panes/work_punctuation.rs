// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Work: `<name>` ▸ Punctuation — the project's typographic house style.

use bastyde::prelude::*;
use bastyde::widgets::{Segment, SegmentedControl};
use frontend::common::entities::QuoteStyle;

#[allow(unused_imports)]
use super::super::*;

/// The quote systems the override offers, in the order the control shows them.
///
/// Four rather than one per locale: this is a *house style* selector, not a
/// locale table. `LocaleDefault` is the normal answer and comes first — the
/// other three exist for the books that deliberately depart from their
/// language's convention, which is common enough in Italian (three co-existing
/// systems) that leaving it to the locale alone would be wrong.
const QUOTE_STYLES: [QuoteStyle; 4] = [
    QuoteStyle::LocaleDefault,
    QuoteStyle::CurlyDouble,
    QuoteStyle::Guillemets,
    QuoteStyle::LowHigh,
];

fn quote_style_label(style: &QuoteStyle) -> bastyde::i18n::LocalizedString {
    match style {
        QuoteStyle::LocaleDefault => tr!(settings_quote_style_locale()),
        QuoteStyle::CurlyDouble => tr!(settings_quote_style_curly()),
        QuoteStyle::Guillemets => tr!(settings_quote_style_guillemets()),
        QuoteStyle::LowHigh => tr!(settings_quote_style_low_high()),
    }
}

/// Bridge a `bool` on the entity to a `Signal<bool>` the widget owns.
///
/// The same two-effect dance `work_structure_pane` uses, and for the same
/// reasons: one effect mirrors external changes (a refresh, an undo, the other
/// settings surface) into the widget, the other pushes a user change back. Both
/// halves guard on the current value — without that they drive each other in a
/// loop — and the write half guards again inside the view-model, because this
/// effect also fires on every rebuild.
fn bridge(
    ctx: &mut BuildContext,
    entity: Signal<bool>,
    write: impl Fn(bool) + 'static,
) -> Signal<bool> {
    let mirror = Signal::new(entity.get());
    {
        let mirror = mirror.clone();
        ctx.effect(&entity, move |v| {
            if mirror.get() != *v {
                mirror.set(*v);
            }
        });
    }
    ctx.effect(&mirror, move |v| write(*v));
    mirror
}

/// Work: `<name>` ▸ Punctuation — the per-project smart-punctuation rules,
/// backed by the shared `SingleSmartPunctuation` (entity-backed, undoable on the
/// Work's stack, and travelling inside the `.skrib`).
///
/// The five switches are disabled until the project takes the override, which is
/// what the stored `override_app_default` means: off is "follow the application
/// preference", not "every rule off". Their values are kept while disabled, so
/// taking the override back gives the writer what they had configured.
pub(in crate::settings) fn work_punctuation_pane(
    ctx: &mut BuildContext,
    vm: &WorkSettingsViewModel,
    work_title: String,
) -> impl Widget {
    let over = bridge(ctx, vm.punctuation_override(), {
        let vm = vm.clone();
        move |v| vm.set_punctuation_override(v)
    });
    let dashes = bridge(ctx, vm.smart_dashes(), {
        let vm = vm.clone();
        move |v| vm.set_smart_dashes(v)
    });
    let ellipsis = bridge(ctx, vm.smart_ellipsis(), {
        let vm = vm.clone();
        move |v| vm.set_smart_ellipsis(v)
    });
    let quotes = bridge(ctx, vm.smart_quotes(), {
        let vm = vm.clone();
        move |v| vm.set_smart_quotes(v)
    });
    let spacing = bridge(ctx, vm.pre_punctuation_spacing(), {
        let vm = vm.clone();
        move |v| vm.set_pre_punctuation_spacing(v)
    });

    // The quote-style control, bridged the same way but over an index.
    let style_entity = vm.quote_style();
    let style_index = Signal::new(
        QUOTE_STYLES
            .iter()
            .position(|s| *s == style_entity.get())
            .unwrap_or(0),
    );
    {
        let style_index = style_index.clone();
        ctx.effect(&style_entity, move |s| {
            let want = QUOTE_STYLES.iter().position(|q| q == s).unwrap_or(0);
            if style_index.get() != want {
                style_index.set(want);
            }
        });
    }
    {
        let vm = vm.clone();
        ctx.effect(&style_index, move |i| {
            vm.set_quote_style(QUOTE_STYLES[*i].clone())
        });
    }

    // Curling quotes is what makes a quote *style* mean anything, so the style
    // control follows that switch rather than the master one alone.
    let style_enabled = over.zip(&quotes).map(|(o, q)| *o && *q);

    let form = FormLayout::new()
        .label(tr!(settings_page_punctuation()))
        .label_gap(16.0)
        .row_spacing(12.0)
        .full_width(group(tr!(settings_group_punctuation())))
        .full_width(Toggle::new(over.clone()).label(tr!(settings_punctuation_override())))
        .full_width(hint(tr!(settings_punctuation_override_hint())))
        .full_width(
            Checkbox::new(dashes)
                .label(tr!(settings_punctuation_dashes()))
                .enabled(over.clone()),
        )
        .full_width(
            Checkbox::new(ellipsis)
                .label(tr!(settings_punctuation_ellipsis()))
                .enabled(over.clone()),
        )
        .full_width(
            Checkbox::new(quotes)
                .label(tr!(settings_punctuation_quotes()))
                .enabled(over.clone()),
        )
        .line(
            field_label(tr!(settings_quote_style())),
            QUOTE_STYLES
                .iter()
                .fold(SegmentedControl::new(style_index), |c, s| {
                    c.segment(Segment::new(quote_style_label(s)))
                })
                .enabled(style_enabled),
        )
        .full_width(
            Checkbox::new(spacing)
                .label(tr!(settings_punctuation_spacing()))
                .enabled(over.clone()),
        )
        .full_width(hint(tr!(settings_punctuation_spacing_hint())));

    // NOTE: the entity also carries `dialogue_marker`, and this pane deliberately
    // does not offer it. A dialogue dash has to know that a paragraph just began,
    // which the stateless rules cannot; it waits on the shared paragraph/clause
    // subsystem. A switch that persisted a preference nothing acts on would read
    // as a broken feature rather than an unbuilt one.

    pane_frame(
        crumb(
            Some(lit!(format!(
                "{}: {}",
                tr!(settings_sec_work()).resolve_now(),
                work_title
            ))),
            tr!(settings_page_punctuation()),
        ),
        form,
    )
}

#[cfg(all(test, feature = "mocks"))]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;
    use frontend::AppContext;
    use std::rc::Rc;

    use crate::singles::{SingleSmartPunctuation, SingleWork};

    fn vm() -> (Rc<AppContext>, WorkSettingsViewModel) {
        let ctx = Rc::new(AppContext::new());
        let work = SingleWork::new(ctx.clone());
        let punctuation = SingleSmartPunctuation::new(ctx.clone());
        let vm = WorkSettingsViewModel::new(work, punctuation, Signal::new(None));
        (ctx, vm)
    }

    /// Hosts the pane so it can be mounted: `work_punctuation_pane` needs a
    /// `&mut BuildContext`, which only exists inside a `Widget::build`.
    struct PaneHost {
        vm: Option<WorkSettingsViewModel>,
        root_child: Option<WidgetId>,
    }

    impl std::fmt::Debug for PaneHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PaneHost").finish()
        }
    }

    impl Widget for PaneHost {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let vm = self.vm.take().expect("built once");
            let body = work_punctuation_pane(ctx, &vm, "Starforgers".to_string());
            let root = ctx.add(body);
            self.root_child = Some(root);
            vec![root]
        }

        fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
            self.root_child
                .and_then(|id| ctx.child_size(id, proposal))
                .map(LayoutResponse::from)
                .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
        }

        fn children(&self) -> Vec<WidgetId> {
            self.root_child.into_iter().collect()
        }
    }

    fn mount(vm: WorkSettingsViewModel, ctx: &Rc<AppContext>) -> WidgetTree {
        let mut tree = crate::test_support::tree_with_settings(ctx);
        tree.add(PaneHost {
            vm: Some(vm),
            root_child: None,
        });
        tree.layout(SizeProposal::exact(760.0, 620.0));
        // The a11y assertions live in each widget's `accessibility`, which only
        // runs when the AccessKit tree is built — laying out alone never reaches
        // them, which is exactly how a `Toggle` shipped with no accessible label
        // and took the whole pane down on first open.
        tree.sync_accessibility();
        tree
    }

    /// The pane mounts, lays out and builds an accessibility tree. This is the
    /// test that panics if any control here loses its accessible name.
    #[test]
    fn the_pane_mounts_and_lays_out() {
        let (ctx, vm) = vm();
        let _tree = mount(vm, &ctx);
    }

    /// And it mounts with the override off, where every switch is disabled — a
    /// different widget state, and the one a project gets before anyone opens
    /// this pane.
    #[test]
    fn the_pane_mounts_with_the_override_off() {
        let (ctx, vm) = vm();
        vm.set_punctuation_override(false);
        let _tree = mount(vm, &ctx);
    }

    /// The switches write through to the entity, and are idempotent — the pane
    /// drives them from effects that fire on every rebuild, so a redundant write
    /// must not queue an undo entry.
    #[test]
    fn a_switch_writes_through_and_is_idempotent() {
        let (_ctx, vm) = vm();
        vm.set_punctuation_override(true);
        vm.set_smart_dashes(false);
        assert!(!vm.smart_dashes().get());
        vm.set_smart_dashes(true);
        assert!(vm.smart_dashes().get());
        // Writing the same value again is a no-op rather than a second edit.
        vm.set_smart_dashes(true);
        assert!(vm.smart_dashes().get());
    }

    /// The quote style round-trips through the view-model.
    #[test]
    fn the_quote_style_writes_through() {
        let (_ctx, vm) = vm();
        vm.set_quote_style(QuoteStyle::LowHigh);
        assert_eq!(vm.quote_style().get(), QuoteStyle::LowHigh);
    }

    /// Turning the override off must **keep** the switches, not clear them:
    /// off means "follow the application preference", and a writer who toggles
    /// it back must get their configuration rather than a blank slate.
    #[test]
    fn dropping_the_override_keeps_the_configured_rules() {
        let (_ctx, vm) = vm();
        vm.set_punctuation_override(true);
        vm.set_smart_dashes(true);
        vm.set_quote_style(QuoteStyle::Guillemets);

        vm.set_punctuation_override(false);
        assert!(vm.smart_dashes().get(), "the rule is remembered");
        assert_eq!(vm.quote_style().get(), QuoteStyle::Guillemets);
    }
}
