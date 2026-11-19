// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Appearance & Behaviour ▸ Appearance — interface language, theme, text size, text
//! contrast, glyph rendering, welcome-at-startup.
//!
//! The two text-drawing rows write teksilo's own settings keys
//! (`TEXT_CONTRAST_KEY`, `GLYPH_RENDERING_KEY`), which `run()` binds and follows
//! in every window, so a control here is all either needs.

use teksilo::prelude::{
    EffectiveGlyphRendering, GlyphRendering, GlyphRenderingPreference, TextContrastLevel,
};
use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{Segment, SegmentedControl};

use std::rc::Rc;

#[allow(unused_imports)]
use super::super::*;

/// The segment for one text contrast level: its name and what it does. `None`
/// for a level teksilo adds after this was written (its enum is
/// `#[non_exhaustive]`): it stays off the page until it is given words here,
/// rather than appearing under another level's name.
fn text_contrast_segment(level: TextContrastLevel) -> Option<Segment> {
    let (label, key, tip) = match level {
        TextContrastLevel::Standard => (
            tr!(settings_text_contrast_standard()),
            "settings.text_contrast.standard",
            tr!(settings_text_contrast_tip_standard()),
        ),
        TextContrastLevel::Stronger => (
            tr!(settings_text_contrast_stronger()),
            "settings.text_contrast.stronger",
            tr!(settings_text_contrast_tip_stronger()),
        ),
        TextContrastLevel::Strongest => (
            tr!(settings_text_contrast_strongest()),
            "settings.text_contrast.strongest",
            tr!(settings_text_contrast_tip_strongest()),
        ),
        _ => return None,
    };
    Some(Segment::new(label).rich_tooltip_content(TooltipContent::new(key, tip)))
}

/// "System", or, while System is the stored choice, "System (sharp)" or
/// "System (smooth)": what following the desktop resolved to, so a writer can
/// tell whether picking Sharp would change anything.
fn system_label_text(choice: GlyphRenderingPreference, now: GlyphRendering) -> LocalizedString {
    if choice != GlyphRenderingPreference::FollowDesktop {
        return tr!(settings_glyph_rendering_system());
    }
    match now {
        GlyphRendering::Crisp => tr!(settings_glyph_rendering_system_sharp()),
        _ => tr!(settings_glyph_rendering_system_smooth()),
    }
}

/// [`system_label_text`], kept current: re-resolved when the stored choice or
/// the rendering in effect (teksilo's `EffectiveGlyphRendering`) changes.
fn system_label(
    stored: &Signal<GlyphRenderingPreference>,
    now: &Signal<GlyphRendering>,
) -> LocalizedString {
    let (choice, read) = (stored.clone(), now.clone());
    teksilo::i18n::localized(move || system_label_text(choice.get(), read.get()).resolve_now())
        .also_observing(stored)
        .also_observing(now)
}

/// The segment for one glyph rendering, as [`text_contrast_segment`] builds
/// one. `system` replaces System's plain name when the window can report what
/// following the desktop resolved to ([`system_label`]).
fn glyph_rendering_segment(
    choice: GlyphRenderingPreference,
    system: Option<LocalizedString>,
) -> Option<Segment> {
    let (label, key, tip) = match choice {
        GlyphRenderingPreference::FollowDesktop => (
            system.unwrap_or_else(|| tr!(settings_glyph_rendering_system())),
            "settings.glyph_rendering.system",
            tr!(settings_glyph_rendering_tip_system()),
        ),
        GlyphRenderingPreference::Natural => (
            tr!(settings_glyph_rendering_smooth()),
            "settings.glyph_rendering.smooth",
            tr!(settings_glyph_rendering_tip_smooth()),
        ),
        GlyphRenderingPreference::Crisp => (
            tr!(settings_glyph_rendering_sharp()),
            "settings.glyph_rendering.sharp",
            tr!(settings_glyph_rendering_tip_sharp()),
        ),
        _ => return None,
    };
    Some(Segment::new(label).rich_tooltip_content(TooltipContent::new(key, tip)))
}

/// A `SegmentedControl` over `segments`, bound to `stored` through
/// [`bridge_choice`]: the choices and their segments come from one list, so a
/// segment's position is always its choice's index.
fn choice_control<T: Copy + PartialEq + 'static>(
    ctx: &mut BuildContext,
    stored: Signal<T>,
    segments: Vec<(T, Segment)>,
) -> SegmentedControl {
    let options: Rc<[T]> = segments.iter().map(|(choice, _)| *choice).collect();
    let index = bridge_choice(ctx, stored, options);
    segments
        .into_iter()
        .fold(SegmentedControl::indexed(index), |control, (_, segment)| {
            control.segment(segment)
        })
}

/// Bridge a stored choice to a `SegmentedControl`'s `usize` over `options`, and
/// return the index signal: two guarded effects, whose `!=` guards stop the two
/// signals writing each other back and forth for ever. The stored side moves the
/// control too, which is how Reset to defaults reaches it. A stored value missing
/// from `options` shows as the first segment and is left alone until one is
/// picked; an index past the end writes nothing.
fn bridge_choice<T: Copy + PartialEq + 'static>(
    ctx: &mut BuildContext,
    stored: Signal<T>,
    options: Rc<[T]>,
) -> Signal<usize> {
    let to_index = |options: &[T], v: T| options.iter().position(|o| *o == v).unwrap_or(0);
    let index: Signal<usize> = Signal::new(to_index(&options, stored.get()));
    {
        let index = index.clone();
        let options = options.clone();
        ctx.effect(&stored, move |v: &T| {
            let i = to_index(&options, *v);
            if index.get() != i {
                index.set(i);
            }
        });
    }
    ctx.effect(&index, move |i: &usize| {
        if let Some(v) = options.get(*i).copied()
            && stored.get() != v
        {
            stored.set(v);
        }
    });
    index
}

/// Appearance & Behaviour ▸ Appearance — interface language, the app-wide
/// **Theme** and **Interface text size** (both relocated here from the old
/// Manuscript pane, where "Editor theme"/"Text size" were misnomers for
/// app-wide controls), and the "show welcome at startup" preference.
pub(in crate::settings) fn appearance_pane(
    ctx: &mut BuildContext,
    crumbs: &Crumbs,
    vm: &SettingsViewModel,
    scale: Signal<f32>,
) -> impl Widget {
    let mut form = FormLayout::new()
        .label(tr!(settings_page_appearance()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_language())))
        .line(
            field_label(tr!(settings_field_language())),
            LanguageSwitcher::new(),
        );
    // Under the language picker, where someone who has just switched language and
    // met a wrong word looks. Only where the running edition takes such reports.
    if crate::identity::links().translation_form.is_some() {
        form = form.line(
            TextWidget::new(lit!(String::new())),
            Button::new(tr!(settings_report_translation()))
                .variant(ButtonVariant::Link)
                .on_activate_fn(|ctx| {
                    crate::translation_report::window::open_or_focus_translation_report(ctx, None)
                }),
        );
    }
    let form = form
        .full_width(group(tr!(settings_group_theme())))
        .line(field_label(tr!(settings_field_app_theme())), {
            // Light / Dark / System — the same three entries as ever, but
            // built from the run's own design language rather than from
            // `ThemeSwitcher::new()`'s hardcoded IntUI pair. Under
            // `--style fluent` those defaults would match no active theme
            // (the combo shows nothing, since it matches by `ThemeId`) and
            // picking Light would drop the window out of Fluent for good.
            //
            // `.system(true)` keeps the follow-OS entry, which is the one
            // that does leave the style by design — see `crate::style`.
            ThemeSwitcher::new()
                .themes([
                    (tr!(settings_theme_light()), crate::style::light()),
                    (tr!(settings_theme_dark()), crate::style::dark()),
                ])
                .system(true)
        })
        .line(
            field_label(tr!(settings_field_text_scale())),
            TextScaleControl::new(scale),
        )
        // How text is drawn, in every window at once (teksilo redraws them all from
        // the one signal `shared::TextRendering` holds).
        .full_width(group(tr!(settings_group_text_rendering())))
        .line(
            field_label(tr!(settings_text_contrast())),
            text_contrast_control(ctx, vm),
        )
        .line(
            field_label(tr!(settings_glyph_rendering())),
            glyph_rendering_control(ctx, vm),
        )
        .full_width(hint(tr!(settings_hint_text_rendering())))
        .full_width(group(tr!(settings_group_startup())))
        // In the label column with every other field on the page, not spanning
        // both. A `full_width` row starts at the pane's own left edge while a
        // `.line` field starts at `label_col + gap`, so mixing the two gives one
        // page two left edges; `export_styles`' editor already settled this the
        // same way. `FormLayout::line` wires `access_labelled_by` itself, so
        // `labelled_externally` only tells the toggle's own assertion so — the
        // relation is pushed after `accessibility()` has already run.
        .line(
            field_label(tr!(settings_show_welcome())),
            Toggle::new(vm.show_welcome())
                .labelled_externally()
                .rich_tooltip_content(TooltipContent::new(
                    "settings.show_welcome",
                    tr!(settings_show_welcome_tip()),
                )),
        );

    pane_frame(crumbs.of(Pane::Appearance), form)
}

/// Standard / Stronger / Strongest, in teksilo's order, bound to its text
/// contrast key.
fn text_contrast_control(ctx: &mut BuildContext, vm: &SettingsViewModel) -> SegmentedControl {
    let segments = TextContrastLevel::all()
        .iter()
        .filter_map(|&level| text_contrast_segment(level).map(|s| (level, s)))
        .collect();
    choice_control(ctx, vm.text_contrast(), segments)
}

/// System / Smooth / Sharp, in teksilo's order, bound to its glyph rendering key.
/// System names what the desktop resolved to while it is the choice.
fn glyph_rendering_control(ctx: &mut BuildContext, vm: &SettingsViewModel) -> SegmentedControl {
    let stored = vm.glyph_rendering();
    // Present whenever teksilo's `run` built the window; a headless tree has none,
    // and its System segment is then plain "System".
    let effective = ctx
        .app_state::<EffectiveGlyphRendering>()
        .map(EffectiveGlyphRendering::signal);
    let segments = GlyphRenderingPreference::all()
        .iter()
        .filter_map(|&choice| {
            let system = effective.as_ref().map(|now| system_label(&stored, now));
            glyph_rendering_segment(choice, system).map(|s| (choice, s))
        })
        .collect();
    choice_control(ctx, stored, segments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use teksilo::core::widget_tree::WidgetTree;

    /// Hosts [`bridge_choice`] in a real `BuildContext`.
    struct BridgeHost {
        stored: Signal<TextContrastLevel>,
        out: Rc<RefCell<Option<Signal<usize>>>>,
    }
    impl std::fmt::Debug for BridgeHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BridgeHost").finish()
        }
    }
    impl Widget for BridgeHost {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let options: Rc<[TextContrastLevel]> = TextContrastLevel::all().into();
            *self.out.borrow_mut() = Some(bridge_choice(ctx, self.stored.clone(), options));
            Vec::new()
        }
        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            proposal.resolve(0.0, 0.0).into()
        }
    }

    /// Picking a segment writes the stored level, a level written elsewhere (Reset
    /// to defaults) moves the control, and an index past the end writes nothing.
    #[test]
    fn the_text_contrast_round_trips_through_the_control() {
        let stored = Signal::new(TextContrastLevel::Stronger);
        let out = Rc::new(RefCell::new(None));
        let mut tree = WidgetTree::new();
        tree.add(BridgeHost {
            stored: stored.clone(),
            out: out.clone(),
        });
        tree.layout(SizeProposal::exact(100.0, 100.0));
        let index = out.borrow().clone().expect("the bridge was built");
        assert_eq!(index.get(), 1, "Stronger is the second segment");

        index.set(2);
        assert_eq!(stored.get(), TextContrastLevel::Strongest);
        index.set(0);
        assert_eq!(stored.get(), TextContrastLevel::Standard);
        index.set(9);
        assert_eq!(stored.get(), TextContrastLevel::Standard);

        stored.set(TextContrastLevel::Strongest); // as Reset does, the other way
        assert_eq!(index.get(), 2, "the control must follow the setting");
        drop(tree);
    }

    /// Every level and rendering teksilo lists today has words here, so the page
    /// offers all of them. A variant teksilo adds later reddens this until it is
    /// given a name and a tooltip, instead of silently staying off the page.
    #[test]
    fn every_choice_teksilo_lists_is_offered() {
        for &level in TextContrastLevel::all() {
            assert!(
                text_contrast_segment(level).is_some(),
                "{level:?} has no segment"
            );
        }
        for &choice in GlyphRenderingPreference::all() {
            assert!(
                glyph_rendering_segment(choice, None).is_some(),
                "{choice:?} has no segment"
            );
        }
    }

    /// The System segment names what the desktop resolved to while System is
    /// chosen, follows a change of either, and is plain "System" otherwise.
    #[test]
    fn the_system_segment_names_the_rendering_in_effect() {
        let text = |l: LocalizedString| l.resolve_now();
        let follow = GlyphRenderingPreference::FollowDesktop;
        assert_eq!(
            text(system_label_text(follow, GlyphRendering::Crisp)),
            text(tr!(settings_glyph_rendering_system_sharp()))
        );
        assert_eq!(
            text(system_label_text(follow, GlyphRendering::Natural)),
            text(tr!(settings_glyph_rendering_system_smooth()))
        );
        assert_eq!(
            text(system_label_text(
                GlyphRenderingPreference::Crisp,
                GlyphRendering::Crisp
            )),
            text(tr!(settings_glyph_rendering_system())),
            "with a fixed choice, System has nothing to report"
        );

        // And the bound label follows both signals.
        let stored = Signal::new(follow);
        let now = Signal::new(GlyphRendering::Natural);
        let shown = system_label(&stored, &now).to_signal();
        assert_eq!(
            shown.get(),
            text(tr!(settings_glyph_rendering_system_smooth()))
        );
        now.set(GlyphRendering::Crisp);
        assert_eq!(
            shown.get(),
            text(tr!(settings_glyph_rendering_system_sharp()))
        );
        stored.set(GlyphRenderingPreference::Natural);
        assert_eq!(shown.get(), text(tr!(settings_glyph_rendering_system())));
    }
}
