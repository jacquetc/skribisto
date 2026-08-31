// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The settings window's own small vocabulary of view helpers.
//!
//! Nothing here knows which page it is on: a field label, a slider with a live
//! readout, the breadcrumb frame a pane's body sits in, the centred placeholder
//! a category with no settings yet shows. They sit together because they are
//! narrower than [`crate::shared`] — which is for what several *features* spell
//! the same way — and wider than any one pane.

use std::rc::Rc;

use skribisto_model::counting::CountingMethodSetting;
use teksilo::canvas::svg::SvgIcon;
use teksilo::i18n::{LocalizedString, localized};
use teksilo::prelude::*;
use teksilo::widgets::{
    Breadcrumb, BreadcrumbItem, Center, Divider, Expand, FixedSize, GroupHeader, HStack,
    IconWidget, MinSize, Padding, ScrollArea, Slider, TextWidget, VStack,
};

use super::nav::{Navigator, Pane, Root, Sec, ancestors_of};

// ── The pane's own measure ───────────────────────────────────────────────────

/// A pane body's horizontal inset, per side — the `24.0` half of
/// [`pane_frame`]'s `Padding::symmetric(20.0, 24.0)`.
pub(crate) const PANE_H_INSET: f32 = 24.0;

/// **The width a pane's content column actually gets**, derived rather than
/// measured by hand.
///
/// The card is [`super::CARD_W`] wide, the category rail takes
/// [`super::TREE_W`] of it, a 1 px rule separates the two, and [`pane_frame`]
/// insets the body by [`PANE_H_INSET`] on each side. Every number in that chain
/// already exists exactly once; this is their sum, so a card that is re-measured
/// moves the panes with it.
///
/// It exists because the alternative had already gone wrong: a pane's own
/// regression test hard-coded `760.0` as "the row's width", which is 151 px more
/// than the window has ever offered, so the test passed while both of the row's
/// dropdowns sat clamped at their floor in the shipping build. A width a test
/// asserts against must be *this* one.
///
/// `settings.rs` owns the column arithmetic as [`super::pane_width`], which the
/// live card drives from its *clamped* width; this constant is that same formula
/// evaluated at the preferred [`super::CARD_W`], minus the frame's inset. The
/// three terms below are `settings.rs`'s own constants, never re-typed here —
/// there must not be two definitions of this number.
///
/// Read only by tests today, and deliberately so: the window does not *lay out*
/// from this number (the card, the rail and the frame each place themselves from
/// their own constants). It exists so an assertion about how wide a pane is has
/// one honest source instead of a fourth hand-copy — which is exactly the bug it
/// was introduced to fix.
#[allow(dead_code)]
pub(crate) const PANE_W: f32 = super::CARD_W - super::TREE_W - super::RULE_W - 2.0 * PANE_H_INSET;

/// The floor a managed list never shrinks below, whatever else shares its pane.
///
/// Low on purpose: it is not the list's *intended* height (that is whatever
/// [`list_box`] can claim from the pane, which depends on the window), only the
/// point below which a list stops being usable. The constants it replaces were
/// 300–320 px, which is more than a pane's whole viewport once the add row and
/// toolbar above it are counted — so the page scrolled at *minimum* content while
/// the list scrolled inside it, two nested scrollbars over one list.
pub(crate) const LIST_MIN_HEIGHT: f32 = 168.0;

/// A managed list, sized to whatever height the pane has left over.
///
/// Panes that are mostly one long list (tags, statuses, templates, replacements,
/// the personal dictionary, export styles, distraction-free themes) used to pin
/// that list to a hand-tuned constant — a `MinSize` floor taller than the
/// viewport, or a `MaxSize` cap chosen before the pane grew a toolbar. Neither
/// number had any relation to the space actually available, and both produced the
/// same defect: the pane scrolled *and* the list scrolled inside it.
///
/// Here the height is not a number at all. [`pane_frame`]'s `ScrollArea` is
/// `widget_resizable`, so content shorter than the viewport is *placed* at the
/// viewport's height; the `Expand::vertical` below then claims that slack, and
/// the leftover goes to the list. Only when the rest of the pane plus
/// [`LIST_MIN_HEIGHT`] genuinely exceeds the viewport does the page scroll —
/// which is the one case where a second scroll region is the honest answer.
///
/// `respect_intrinsic` is load-bearing: an `Expand` with the default zero basis
/// reports **0** on its flex axis during intrinsic measurement, so the pane would
/// measure as if the list were not there and `ScrollArea` would never learn it
/// had overflowed.
pub(crate) fn list_box(child: impl Widget + 'static) -> impl Widget + 'static {
    Expand::vertical()
        .respect_intrinsic()
        .child(MinSize::new(0.0, LIST_MIN_HEIGHT).child(child))
}

// ── Small view helpers ───────────────────────────────────────────────────────

/// A left-column field label (dimmed, small) — matches the design's `--tx2`.
/// `pub(crate)` so `panes::backup` shares the exact same row-label style as
/// every built-in pane.
pub(crate) use crate::shared::text::{field_label, hint};

/// A lowercase-titled section header + trailing rule (design's group headers).
/// `pub(crate)` so the backup panes reuse the identical group-header treatment.
pub(crate) fn group(text: LocalizedString) -> GroupHeader {
    GroupHeader::new(text)
        .style(TextStyleRole::SmallBold)
        .color(TextRole::Secondary)
}

/// A slider with a live value readout on its right, filling the field slot it
/// is placed in.
pub(crate) fn slider_field(
    value: Signal<f32>,
    min: f32,
    max: f32,
    step: f32,
    fmt: impl Fn(f32) -> String + 'static,
) -> impl Widget + 'static {
    slider_field_inner(value, min, max, step, fmt, None)
}

/// Like [`slider_field`], with a plain tooltip on the slider (for short
/// explanations that used to be body-copy hints under the control).
pub(crate) fn slider_field_tipped(
    value: Signal<f32>,
    min: f32,
    max: f32,
    step: f32,
    fmt: impl Fn(f32) -> String + 'static,
    tip: LocalizedString,
) -> impl Widget + 'static {
    slider_field_inner(value, min, max, step, fmt, Some(tip))
}

fn slider_field_inner(
    value: Signal<f32>,
    min: f32,
    max: f32,
    step: f32,
    fmt: impl Fn(f32) -> String + 'static,
    tip: Option<LocalizedString>,
) -> impl Widget + 'static {
    let seed = fmt(value.get());
    let text = value.map(move |v| fmt(*v));
    let readout = TextWidget::new(LocalizedString::literal(seed))
        .text(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary);
    let mut slider = Slider::new(value, min, max).step(step);
    if let Some(tip) = tip {
        slider = slider.tooltip(tip);
    }
    // No `FixedSize::width` here. `FormLayout` places its field slot at
    // `field_col_width` unconditionally and a `FixedSize` in that slot passes the
    // slot's width straight back out, so the 300 px this used to declare was never
    // once honoured — it only stated an intent the layout discards. **A form
    // field's width is the layout's to decide**, and what it decides is a function
    // of the longest *label* on the page: ~401 px in English, ~353 in French, ~313
    // on the footnote-typography page. Every settings field is like this; this is
    // the one place it is written down.
    HStack::new()
        .spacing(12.0)
        .child(Expand::horizontal().child(slider))
        .child(FixedSize::new().width(52.0).child(readout))
}

/// What activating an ancestor crumb runs — see [`Crumbs::jump`].
///
/// Named, because `BreadcrumbItem::on_activate_fn` takes it by value and a test
/// has to hold one to prove the link does anything at all.
pub(crate) type CrumbJump = Box<dyn Fn(&mut EventContext)>;

/// Everything a pane needs to print its own breadcrumb: the tree it sits in,
/// the open project's title, and the way to jump to a page.
///
/// Threaded down to every pane builder from [`super::content::build`] rather
/// than each page naming its own ancestors, because a hand-written trail is a
/// second copy of the tree's shape — the very thing [`super::nav`]'s header
/// warns against. Every trail here is derived from the same `spec` the rail is
/// built from, so a page moved between sections is re-breadcrumbed by that one
/// edit.
///
/// `nav` is `None` only where there is genuinely nothing to navigate (a
/// headless test); the window always has one, so its ancestors are live links.
#[derive(Clone)]
pub(crate) struct Crumbs {
    spec: Rc<Vec<Root>>,
    /// The open project's title — the "Work: `<title>`" half of the Work
    /// section's crumb. Empty when nothing is open, which is also when no Work
    /// page is reachable.
    work_title: Rc<str>,
    nav: Option<Navigator>,
}

impl Crumbs {
    pub(crate) fn new(spec: Rc<Vec<Root>>, work_title: &str, nav: Option<Navigator>) -> Self {
        Self {
            spec,
            work_title: Rc::from(work_title),
            nav,
        }
    }

    /// The label a crumb prints for `pane` — its own, except the open project's
    /// section, which reads "Work: `<title>`".
    pub(crate) fn label(&self, pane: Pane) -> LocalizedString {
        match pane {
            Pane::Section(sec) => section_title(sec, &self.work_title),
            other => other.label(),
        }
    }

    /// `pane`'s full ancestry, outermost first, each with the label to print.
    ///
    /// Derived from [`ancestors_of`], so a leaf under Editor ▸ Typography
    /// carries *both* of its ancestors. Printing only the nearest is what made
    /// every typography page read "Editor › Scene" and never name the group it
    /// is actually in.
    pub(crate) fn trail(&self, pane: Pane) -> Vec<(Pane, LocalizedString)> {
        ancestors_of(&self.spec, pane)
            .into_iter()
            .map(|a| (a, self.label(a)))
            .collect()
    }

    /// `pane`'s breadcrumb: its ancestry as links, then its own name.
    pub(crate) fn of(&self, pane: Pane) -> Breadcrumb {
        self.titled(pane, self.label(pane))
    }

    /// The same, with the current crumb's text supplied by the caller — for a
    /// page whose printed name is not the enum's to know (an extension's, and
    /// the open project's section page).
    pub(crate) fn titled(&self, pane: Pane, current: LocalizedString) -> Breadcrumb {
        let mut b = Breadcrumb::new();
        for (ancestor, label) in self.trail(pane) {
            let item = BreadcrumbItem::new(label);
            b = b.item(match self.jump(ancestor) {
                Some(go) => item.on_activate_fn(go),
                None => item,
            });
        }
        b.item(BreadcrumbItem::current(current))
    }

    /// What an ancestor crumb *does* when it is activated.
    ///
    /// A named handle rather than a closure written inline in [`Self::titled`],
    /// because it is the half that kept going missing: a `BreadcrumbItem` with
    /// no action still reports `Role::Link` **and a name** to accessibility
    /// (`teksilo-widgets/src/breadcrumb.rs`, `BreadcrumbSegment::accessibility`),
    /// so a screen reader announced a link that could not be activated while a
    /// sighted reader clicked "Editor ›" to no effect at all. Only
    /// `is_interactive()` — which is exactly "carries an action" — makes the
    /// segment focusable, hand-cursored and accent-coloured.
    ///
    /// `None` only where there is nothing to navigate: a [`Crumbs`] built
    /// without a [`Navigator`], which is a headless test and never the window.
    pub(crate) fn jump(&self, pane: Pane) -> Option<CrumbJump> {
        let nav = self.nav.clone()?;
        Some(Box::new(move |_ctx: &mut EventContext| nav.go(pane)))
    }
}

/// Height of the breadcrumb band above a pane's body.
///
/// 34, not the 46 it was: the trail's own measured ink is 28 px, and a
/// `BreadcrumbItem` already carries a 10 px leading inset of its own. At 46 the
/// band's `Padding::symmetric(13.0, 22.0)` put the text at x = 32 while the pane
/// body starts at x = 24 and the footer at x = 22 — 8 px of drift across three
/// bands that are meant to line up. 34 with a 14 px inset lands the text at
/// exactly 24 and leaves all 28 px of ink intact.
pub(crate) const CRUMB_BAND_H: f32 = 34.0;

/// The right-pane frame: breadcrumb header · rule · scrollable content.
pub(crate) fn pane_frame(
    breadcrumb: impl Widget + 'static,
    content: impl Widget + 'static,
) -> impl Widget {
    VStack::new()
        .spacing(0.0)
        .child(
            // `Expand::horizontal` around the band, not a bare height-only
            // `FixedSize`: a `FixedSize` with only its height set reports its
            // CHILD's intrinsic width, so the strip measured 184 px inside a
            // 657 px pane. The same trap is recorded (and fixed the same way) at
            // `backup/list_panel.rs`. It also blocks the obvious next use of the
            // band — `Breadcrumb::trailing_slot` for a page-level action, which
            // needs the full width to sit at the far end of.
            Expand::horizontal().child(
                FixedSize::new()
                    .height(CRUMB_BAND_H)
                    .child(Padding::symmetric(3.0, 14.0).child(breadcrumb)),
            ),
        )
        .child(Expand::horizontal().child(Divider::new()))
        .child(
            Expand::vertical().child(
                ScrollArea::new()
                    // Content shorter than the viewport is *placed* at the
                    // viewport's height rather than its own, which is what gives
                    // [`list_box`]'s `Expand::vertical` something to claim. Without
                    // it a `ScrollArea` proposes `height: None` for ever and a list
                    // can only ever be as tall as a hand-written constant.
                    .widget_resizable(true)
                    .child(Padding::symmetric(20.0, PANE_H_INSET).child(content)),
            ),
        )
}

/// The centered placeholder body for a not-yet-implemented category.
pub(crate) fn empty_content(icon: &'static SvgIcon) -> impl Widget {
    Center::new().child(
        VStack::new()
            .spacing(10.0)
            .child(
                IconWidget::from_svg_icon(icon)
                    .icon_size(40.0)
                    .color(TextRole::Disabled),
            )
            .child(
                TextWidget::new(tr!(settings_empty_title()))
                    .style(TextStyleRole::Body)
                    .color(TextRole::Secondary),
            )
            .child(hint(tr!(settings_empty_hint()))),
    )
}

/// A section's displayed title — static, except the open project's, which reads
/// "Work: `<title>`".
///
/// Composed per resolve rather than once, so a runtime locale switch reaches the
/// "Work" half of it (the project's own title is data and stays as typed).
pub(crate) fn section_title(sec: Sec, work_title: &str) -> LocalizedString {
    match sec {
        Sec::Work => {
            let title = work_title.to_string();
            localized(move || format!("{}: {}", tr!(settings_sec_work()).resolve_now(), title))
        }
        other => other.label(),
    }
}

/// A full empty pane (breadcrumb + placeholder).
///
/// Breadcrumbed from the same [`Crumbs`] a real page uses, so the placeholder a
/// Work page falls back to when no project is open still names where it sits —
/// and its ancestors are still links out of a page with nothing on it, which is
/// the one page a reader most wants to leave.
pub(crate) fn empty_pane(crumbs: &Crumbs, pane: Pane, icon: &'static SvgIcon) -> impl Widget {
    pane_frame(crumbs.of(pane), empty_content(icon))
}

/// `CountingMethodSetting` ⟷ the `RadioGroup`'s `usize` selection. The order here
/// is the radio order in `goals_pane`; keep the two in step.
pub(crate) fn method_to_index(m: CountingMethodSetting) -> usize {
    match m {
        CountingMethodSetting::Auto => 0,
        CountingMethodSetting::Whitespace => 1,
        CountingMethodSetting::UnicodeWords => 2,
        CountingMethodSetting::CjkHybrid => 3,
    }
}

pub(crate) fn index_to_method(i: usize) -> CountingMethodSetting {
    match i {
        1 => CountingMethodSetting::Whitespace,
        2 => CountingMethodSetting::UnicodeWords,
        3 => CountingMethodSetting::CjkHybrid,
        _ => CountingMethodSetting::Auto,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::widgets::RectWidget;

    /// The content viewport a pane is laid out in, as the window builds it: the
    /// card minus the rail and the rule, minus the band, the divider and the
    /// body's own vertical insets.
    const VIEWPORT_W: f32 = PANE_W + 2.0 * PANE_H_INSET;
    const VIEWPORT_H: f32 = 519.0;

    /// Walk `pane_frame`'s own shape down to the body a caller passed in.
    ///
    /// Spelled out rather than searched for: if the frame's shape changes, these
    /// tests must fail loudly rather than quietly measure some other widget.
    fn body_of(tree: &WidgetTree, frame: WidgetId) -> WidgetId {
        let bands = tree.children(frame);
        assert_eq!(bands.len(), 3, "band · rule · scroll");
        let scroll = tree.children(bands[2])[0];
        let padding = tree.children(scroll)[0];
        tree.children(padding)[0]
    }

    fn frame_with(body: impl Widget + 'static, height: f32) -> (WidgetTree, WidgetId) {
        let mut tree = WidgetTree::new();
        // A root page's crumb: no ancestors, nothing to navigate to.
        let crumbs = Crumbs::new(Rc::new(Vec::new()), "", None);
        let frame = tree.add(pane_frame(crumbs.titled(Pane::Keymap, lit!("Page")), body));
        tree.layout(SizeProposal::exact(VIEWPORT_W, height));
        (tree, frame)
    }

    /// A list takes the height the pane has left over, not a hand-written one.
    #[test]
    fn a_list_box_claims_the_leftover_height_of_the_pane() {
        let above = 100.0;
        let (tree, frame) = frame_with(
            VStack::new()
                .child(FixedSize::new().height(above).child(RectWidget::new()))
                .child(list_box(RectWidget::new())),
            VIEWPORT_H,
        );
        let body = body_of(&tree, frame);
        let list = tree.children(body)[1];
        let got = tree.bounds(list).height;
        let want = VIEWPORT_H - CRUMB_BAND_H - 1.0 - 2.0 * 20.0 - above;
        assert!(
            (got - want).abs() < 1.5,
            "the list should fill the {want} px the pane has left, got {got}"
        );
        assert!(
            got > LIST_MIN_HEIGHT,
            "…and that is more than its floor, or the helper bought nothing"
        );
    }

    /// …and stops at its floor when the rest of the pane genuinely fills the
    /// viewport, which is the one case where letting the page scroll is honest.
    #[test]
    fn a_list_box_stops_at_its_floor_when_the_pane_is_already_full() {
        let (tree, frame) = frame_with(
            VStack::new()
                .child(FixedSize::new().height(800.0).child(RectWidget::new()))
                .child(list_box(RectWidget::new())),
            VIEWPORT_H,
        );
        let body = body_of(&tree, frame);
        let got = tree.bounds(tree.children(body)[1]).height;
        assert!(
            (got - LIST_MIN_HEIGHT).abs() < 1.5,
            "expected the floor {LIST_MIN_HEIGHT}, got {got}"
        );
    }

    /// The breadcrumb band spans the pane and stands 34 px tall.
    ///
    /// The width half is the regression: a height-only `FixedSize` reports its
    /// *child's* intrinsic width, so the strip measured about 184 px inside a
    /// 657 px pane — which is also what blocked putting a page action at its
    /// trailing end.
    #[test]
    fn the_breadcrumb_band_spans_the_pane_at_its_own_height() {
        let (tree, frame) = frame_with(RectWidget::new(), VIEWPORT_H);
        let band = tree.bounds(tree.children(frame)[0]);
        assert!(
            (band.width - VIEWPORT_W).abs() < 0.5,
            "the band should span the pane's {VIEWPORT_W} px, got {}",
            band.width
        );
        assert!(
            (band.height - CRUMB_BAND_H).abs() < 0.5,
            "expected a {CRUMB_BAND_H} px band, got {}",
            band.height
        );
    }

    /// The pane width is the card's arithmetic, not a fourth hand-copy of 609.
    #[test]
    fn the_pane_width_is_derived_from_the_card() {
        assert_eq!(
            PANE_W,
            super::super::CARD_W - super::super::TREE_W - 1.0 - 2.0 * PANE_H_INSET
        );
    }
}
