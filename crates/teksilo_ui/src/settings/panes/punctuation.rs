// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Punctuation — the application-level smart-punctuation preference.

use teksilo::prelude::*;
use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{Segment, SegmentedControl};

#[allow(unused_imports)]
use super::super::*;

use super::work_punctuation::{QUOTE_STYLES, language_sample, quote_style_label};

/// Editor ▸ Punctuation — what every project does unless it takes an override
/// of its own in Work ▸ Punctuation.
///
/// The same controls as the per-project pane minus its master switch: this tier
/// *is* the default, so there is nothing for it to override. It is also
/// store-backed rather than entity-backed, which is the right home — this is a
/// preference of the person at the keyboard, and it follows them across
/// projects. The per-project row is the opposite: a property of the manuscript,
/// travelling inside the `.skrib` to whoever opens it next.
pub(in crate::settings) fn punctuation_pane(
    ctx: &mut BuildContext,
    vm: &SettingsViewModel,
) -> impl Widget {
    // The sample needs a language to resolve against, and this tier has no
    // project. It shows the *interface* language, which is the only one on hand
    // and is very often the one the writer works in — the per-project pane shows
    // the real answer for a given book.
    let ui_lang = vec![vm.locale().get()];

    let style_index = Signal::new(
        QUOTE_STYLES
            .iter()
            .position(|s| *s == vm.punct_quote_style().get())
            .unwrap_or(0),
    );
    {
        let entity = vm.punct_quote_style();
        let style_index = style_index.clone();
        ctx.effect(&entity, move |s| {
            let want = QUOTE_STYLES.iter().position(|q| q == s).unwrap_or(0);
            if style_index.get() != want {
                style_index.set(want);
            }
        });
    }
    {
        let entity = vm.punct_quote_style();
        ctx.effect(&style_index, move |i| {
            let want = QUOTE_STYLES[*i].clone();
            if entity.get() != want {
                entity.set(want);
            }
        });
    }

    // React to BOTH the quote style and the interface language: switching the UI
    // language changes which quotes "your language" produces, and the sample has
    // to follow — a snapshot taken once at build would freeze it on the language
    // that happened to be active when the pane was first shown.
    let sample = vm
        .locale()
        .zip(&vm.punct_quote_style())
        .map(|(lang, style)| language_sample(std::slice::from_ref(lang), style));

    let form = FormLayout::new()
        .label(tr!(settings_page_punctuation()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_punctuation())))
        .full_width(
            Toggle::new(vm.punct_dashes())
                .label(tr!(settings_punctuation_dashes()))
                .rich_tooltip_content(TooltipContent::new(
                    "settings.punct_app",
                    tr!(settings_punctuation_app_hint()),
                )),
        )
        .full_width(Toggle::new(vm.punct_ellipsis()).label(tr!(settings_punctuation_ellipsis())))
        .full_width(Toggle::new(vm.punct_quotes()).label(tr!(settings_punctuation_quotes())))
        .line(
            field_label(tr!(settings_quote_style())),
            QUOTE_STYLES
                .iter()
                .fold(SegmentedControl::new(style_index), |c, s| {
                    c.segment(Segment::new(quote_style_label(s)))
                })
                .enabled(vm.punct_quotes()),
        )
        .line(
            field_label(tr!(settings_punctuation_sample())),
            TextWidget::new(lit!(language_sample(
                &ui_lang,
                &vm.punct_quote_style().get()
            )))
            .text(sample),
        )
        .full_width(
            Toggle::new(vm.punct_spacing())
                .label(tr!(settings_punctuation_spacing()))
                .rich_tooltip_content(TooltipContent::new(
                    "settings.punct_spacing",
                    tr!(settings_punctuation_spacing_hint()),
                )),
        )
        .full_width(
            Toggle::new(vm.punct_dialogue())
                .label(tr!(settings_punctuation_dialogue()))
                .rich_tooltip_content(TooltipContent::new(
                    "settings.punct_dialogue",
                    tr!(settings_punctuation_dialogue_hint()),
                )),
        );

    pane_frame(
        crumb(
            Some(tr!(settings_sec_editor())),
            tr!(settings_page_punctuation()),
        ),
        form,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    struct PaneHost {
        vm: Option<SettingsViewModel>,
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
            let body = punctuation_pane(ctx, &vm);
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

    /// The pane mounts, lays out and builds an accessibility tree — the check
    /// that fails if any control here loses its accessible name.
    #[test]
    fn the_pane_mounts_and_lays_out() {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        let mut tree = crate::test_support::tree_with_settings(&ctx);
        let store = tree
            .app_context()
            .app_state::<teksilo::settings::SettingsStore>()
            .expect("tree_with_settings registers a store")
            .clone();
        tree.add(PaneHost {
            vm: Some(SettingsViewModel::new(&store)),
            root_child: None,
        });
        tree.layout(SizeProposal::exact(760.0, 620.0));
        tree.sync_accessibility();
    }
}
