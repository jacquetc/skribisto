// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Work: `<name>` ▸ Punctuation — the project's typographic house style.

use frontend::common::entities::QuoteStyle;
use teksilo::prelude::*;
use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{Segment, SegmentedControl};

use crate::text_replacement::typography::{
    QuoteSystem, mirrored_for, ruleset_for, uses_guillemet_inner_spacing,
};

#[allow(unused_imports)]
use super::super::*;

/// The quote systems the override offers, in the order the control shows them.
///
/// Four rather than one per locale: this is a *house style* selector, not a
/// locale table. `LocaleDefault` is the normal answer and comes first — the
/// other three exist for the books that deliberately depart from their
/// language's convention, which is common enough in Italian (three co-existing
/// systems) that leaving it to the locale alone would be wrong.
pub(in crate::settings) const QUOTE_STYLES: [QuoteStyle; 4] = [
    QuoteStyle::LocaleDefault,
    QuoteStyle::CurlyDouble,
    QuoteStyle::Guillemets,
    QuoteStyle::LowHigh,
];

pub(in crate::settings) fn quote_style_label(style: &QuoteStyle) -> teksilo::i18n::LocalizedString {
    match style {
        QuoteStyle::LocaleDefault => tr!(settings_quote_style_locale()),
        QuoteStyle::CurlyDouble => tr!(settings_quote_style_curly()),
        QuoteStyle::Guillemets => tr!(settings_quote_style_guillemets()),
        QuoteStyle::LowHigh => tr!(settings_quote_style_low_high()),
    }
}

/// A sample of what the project's language actually produces, given the chosen
/// quote style — the answer to "I set the language to Spanish and nothing in
/// this pane changed".
///
/// Nothing here *is* language-dependent: the switches say which rules run, and
/// the locale table says what each one produces, so the pane legitimately looks
/// identical in every language. That is defensible and still unhelpful — a
/// writer cannot see what "Language default" means for their book without
/// typing a quotation mark into a scene and looking. This line shows it.
///
/// Built as data (`lit!`), not a translated string: it is glyphs, and the same
/// glyphs whatever the interface language.
pub(in crate::settings) fn language_sample(langs: &[String], style: &QuoteStyle) -> String {
    let tag = skribisto_model::language::primary(langs);
    let ruleset = ruleset_for(tag);
    let quotes = match style {
        QuoteStyle::LocaleDefault => ruleset.primary_quotes,
        QuoteStyle::CurlyDouble => QuoteSystem::Paired {
            open: '\u{201C}',
            close: '\u{201D}',
        },
        QuoteStyle::Guillemets => QuoteSystem::Paired {
            open: '\u{00AB}',
            close: '\u{00BB}',
        },
        QuoteStyle::LowHigh => QuoteSystem::Paired {
            open: '\u{201E}',
            close: '\u{201C}',
        },
    };
    // Guillemets carry a thin no-break space inside them in French (and only
    // French), added by the engine along with the marks themselves — so the
    // sample has to show `« … »`, not a bare `«…»` that misrepresents the output.
    let (open, close) = (quotes.open(), quotes.close());
    let inner = if uses_guillemet_inner_spacing(tag) && open == '\u{00AB}' && close == '\u{00BB}' {
        format!("{open}\u{202F}\u{2026}\u{202F}{close}")
    } else {
        format!("{open}\u{2026}{close}")
    };
    let mut out = inner;
    // The two rules that are language-gated rather than switch-gated, and so
    // cannot be inferred from anything else on this pane.
    if !ruleset.pre_punctuation.is_empty() {
        out.push_str("   mot\u{202F}?");
    }
    for (ascii, localized) in mirrored_for(tag) {
        out.push_str(&format!("   {ascii}\u{2192}{localized}"));
    }
    out
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
    let dialogue = bridge(ctx, vm.dialogue_marker(), {
        let vm = vm.clone();
        move |v| vm.set_dialogue_marker(v)
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
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_punctuation())))
        .full_width(
            Toggle::new(over.clone())
                .label(tr!(settings_punctuation_override()))
                .rich_tooltip_content(TooltipContent::new(
                    "settings.punct_override",
                    tr!(settings_punctuation_override_hint()),
                )),
        )
        .full_width(
            Toggle::new(dashes)
                .label(tr!(settings_punctuation_dashes()))
                .enabled(over.clone()),
        )
        .full_width(
            Toggle::new(ellipsis)
                .label(tr!(settings_punctuation_ellipsis()))
                .enabled(over.clone()),
        )
        .full_width(
            Toggle::new(quotes)
                .label(tr!(settings_punctuation_quotes()))
                .enabled(over.clone()),
        )
        .line(
            field_label(tr!(settings_quote_style())),
            QUOTE_STYLES
                .iter()
                .fold(SegmentedControl::indexed(style_index), |c, s| {
                    c.segment(Segment::new(quote_style_label(s)))
                })
                .enabled(style_enabled),
        )
        .line(
            field_label(tr!(settings_punctuation_sample())),
            TextWidget::new(lit!(language_sample(
                &vm.dict_language().get(),
                &vm.quote_style().get()
            )))
            .text(
                vm.dict_language()
                    .zip(&vm.quote_style())
                    .map(|(l, s)| language_sample(l, s)),
            ),
        )
        .full_width(
            Toggle::new(spacing)
                .label(tr!(settings_punctuation_spacing()))
                .enabled(over.clone())
                .rich_tooltip_content(TooltipContent::new(
                    "settings.punct_spacing_work",
                    tr!(settings_punctuation_spacing_hint()),
                )),
        )
        .full_width(
            Toggle::new(dialogue)
                .label(tr!(settings_punctuation_dialogue()))
                .enabled(over.clone())
                .rich_tooltip_content(TooltipContent::new(
                    "settings.punct_dialogue_work",
                    tr!(settings_punctuation_dialogue_hint()),
                )),
        );

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
    use frontend::AppContext;
    use std::rc::Rc;
    use teksilo::core::widget_tree::WidgetTree;

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

    /// **The row that answers "I changed the language and nothing happened".**
    ///
    /// Nothing on this pane is language-dependent — the switches say which rules
    /// run, the locale table says what each produces — so the pane legitimately
    /// looks identical in every language. This sample is the one place the
    /// difference is visible, so it is pinned per language.
    #[test]
    fn the_sample_shows_what_each_language_produces() {
        let l = |s: &str| vec![s.to_string()];
        let d = QuoteStyle::LocaleDefault;

        assert_eq!(language_sample(&l("en-US"), &d), "\u{201C}\u{2026}\u{201D}");
        assert_eq!(language_sample(&l("es-ES"), &d), "\u{00AB}\u{2026}\u{00BB}");
        assert_eq!(language_sample(&l("de-DE"), &d), "\u{201E}\u{2026}\u{201C}");
        // Swedish opens and closes with the same glyph.
        assert_eq!(language_sample(&l("sv-SE"), &d), "\u{201D}\u{2026}\u{201D}");

        // French guillemets carry their thin no-break space inside the marks —
        // `« … »`, not `«…»` — because the engine adds it along with the marks,
        // and the sample has to show what the writer will actually get.
        let fr = language_sample(&l("fr-FR"), &d);
        assert!(
            fr.starts_with("\u{00AB}\u{202F}\u{2026}\u{202F}\u{00BB}"),
            "French shows the inner thin space: {fr:?}"
        );
        // And its pre-punctuation spacing example — that rule is language-gated
        // too, so it cannot be inferred from the switches.
        assert!(
            fr.contains("mot\u{202F}?"),
            "French shows its thin space before ?: {fr:?}"
        );

        // Arabic adds its mirrored marks, which appear nowhere else in the pane.
        let ar = language_sample(&l("ar"), &d);
        for want in [
            "?\u{2192}\u{061F}",
            ",\u{2192}\u{060C}",
            ";\u{2192}\u{061B}",
        ] {
            assert!(ar.contains(want), "Arabic shows {want}: {ar:?}");
        }

        // Hebrew is right-to-left and must NOT show mirrored marks.
        let he = language_sample(&l("he-IL"), &d);
        assert!(!he.contains("\u{061F}"), "Hebrew keeps ASCII: {he:?}");
    }

    /// An explicit house style overrides the language in the sample too — what
    /// the writer sees has to match what they will get.
    #[test]
    fn an_explicit_style_overrides_the_language_in_the_sample() {
        let l = vec!["en-US".to_string()];
        assert_eq!(
            language_sample(&l, &QuoteStyle::Guillemets),
            "\u{00AB}\u{2026}\u{00BB}"
        );
        assert_eq!(
            language_sample(&l, &QuoteStyle::LowHigh),
            "\u{201E}\u{2026}\u{201C}"
        );
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
