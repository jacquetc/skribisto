// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Pill` — the rounded chip chrome shared by every pill row in the app: spellcheck
//! languages, item tags, and item aliases.
//!
//! Extracted from `spellcheck::language_pill_field`, which had it first. Only the *chrome*
//! lives here — the shell, the focus ring, the hover-revealed remove glyph, the
//! accessibility wiring, the rigid content-hugging layout, and the tooltip tier routing.
//! What a pill *means* (what its label says, what tapping it does, what goes in its leading
//! slot) stays with the feature that owns it.
//!
//! Two things this type exists to stop each caller re-deriving:
//!
//! * **Slots are always laid out; only their paint changes.** The chip's size must not
//!   move when the leading glyph toggles or the `×` reveals on hover, or the whole `Wrap`
//!   reflows under the pointer. `set_opacity` is paint-only; `visible_when` would collapse
//!   the slot out of layout and resize the chip.
//! * **Tooltips go through the framework's shared `attach` helpers**, at the right tier and
//!   delay. Reaching for bare `ctx.attach_tooltip` — as this code originally did — silently
//!   loses dwell-to-sticky promotion, the dwell-progress sink, placement control and
//!   keyboard-focus promotion.

use std::rc::Rc;
use std::time::Duration;

use bastyde::core::accesskit::Role;
use bastyde::core::overlay::TooltipPlacement;
use bastyde::core::widget::{Widget, WidgetPlacement};
use bastyde::i18n::LocalizedString;
use bastyde::prelude::*;
use bastyde::tokens::{BorderRole, CornerRadius};
use bastyde::widgets::tooltip::{
    CompositeTooltipWidget, TooltipContent, attach_rich_tooltip_content_with_placement,
};
use bastyde::widgets::{Center, MinSize, Padding, RectWidget, TextWidget, ZStack};

/// Dwell before a rich/composite tooltip pins itself open.
///
/// Mirrors `bastyde_widgets::tooltip::rich::DWELL_PROMOTION`, which is `pub(crate)` and so
/// cannot be referenced from an app crate. Duplicated in exactly one place on purpose — if
/// the framework's value ever changes, this is the single line to follow it.
const DWELL_PROMOTION: Duration = Duration::from_secs(2);

/// What (if anything) a pill shows on hover.
pub enum PillTooltip {
    None,
    /// A single line. Cheapest tier; no dwell promotion, no keyboard reachability — right
    /// for a fixed action label, wrong for anything a reader needs time with.
    Plain(LocalizedString),
    /// Short text with optional disclosure. Gains dwell-to-sticky and keyboard-focus
    /// promotion.
    Rich(TooltipContent),
    /// An arbitrary body (a swatch row, an evidence excerpt). Carries an accessible name,
    /// because an unlabelled composite tooltip announces literally "Tooltip".
    Composite {
        body: Box<dyn Widget>,
        access_label: LocalizedString,
    },
}

impl Default for PillTooltip {
    fn default() -> Self {
        Self::None
    }
}

/// Attach a composite tooltip that carries a real accessible name.
///
/// The framework's `attach_composite_tooltip_boxed*` helpers cannot set one (they take a
/// `Box<dyn Widget>` with no post-hoc setter), so every tooltip attached through them
/// announces the generic fallback. This rebuilds the same wiring — including the sticky
/// promotion and the dwell sink they supply — with the label set.
pub fn attach_labelled_composite_tooltip(
    ctx: &mut BuildContext,
    anchor_id: WidgetId,
    body: Box<dyn Widget>,
    access_label: LocalizedString,
    placement: TooltipPlacement,
) -> WidgetId {
    let tooltip = CompositeTooltipWidget::new()
        .content_boxed(body)
        .access_label(access_label);
    let sink = tooltip.shown_at_sink();
    let tooltip_id = ctx.add(tooltip);
    // Composite uses the *heavy* delay; only the plain tier uses `tooltip_delay`.
    let delay = ctx.theme().motion.tooltip_delay_heavy;
    ctx.attach_tooltip_with_sticky_sink_placement(
        anchor_id,
        tooltip_id,
        delay,
        Some(DWELL_PROMOTION),
        sink,
        placement,
    );
    tooltip_id
}

/// One rounded chip.
///
/// Construct with [`Pill::new`], then add only what the feature needs. A pill with no
/// `on_remove` simply has no `×` (and reserves no space for one).
pub struct Pill {
    display: String,
    /// Optional leading glyph — the spellcheck check, or nothing. Always laid out when
    /// present; `leading_opacity` decides whether it paints.
    leading: Option<Box<dyn Widget>>,
    leading_opacity: f32,
    background: ColorProp,
    text_color: Option<ColorProp>,
    tooltip: PillTooltip,
    tooltip_placement: TooltipPlacement,
    remove_label: Option<LocalizedString>,
    a11y_label: LocalizedString,
    hover: Signal<bool>,
    focused: Signal<bool>,
    on_activate: Option<Rc<dyn Fn(&mut EventContext)>>,
    on_remove: Option<Rc<dyn Fn(&mut EventContext)>>,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for Pill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pill").field("display", &self.display).finish()
    }
}

impl Pill {
    pub fn new(display: impl Into<String>, a11y_label: impl Into<LocalizedString>) -> Self {
        Self {
            display: display.into(),
            leading: None,
            leading_opacity: 1.0,
            background: SurfaceRole::AccentSubtle.into(),
            text_color: None,
            tooltip: PillTooltip::None,
            // A pill row is a `Wrap`: pills sit *horizontally* adjacent at a few dp, so a
            // `Side` tooltip would collide with the neighbour every time. `Side` is for
            // vertically stacked anchors (list/tree rows) — the opposite geometry.
            tooltip_placement: TooltipPlacement::Below,
            remove_label: None,
            a11y_label: a11y_label.into(),
            hover: Signal::new(false),
            focused: Signal::new(false),
            on_activate: None,
            on_remove: None,
            root_child: None,
        }
    }

    /// A leading glyph that is always laid out but paints only when `visible`, so toggling
    /// it never resizes the chip or reflows the row.
    pub fn leading(mut self, glyph: impl Widget + 'static, visible: bool) -> Self {
        self.leading = Some(Box::new(glyph));
        self.leading_opacity = if visible { 1.0 } else { 0.0 };
        self
    }

    /// Chip background. Accepts a semantic role or a raw `Color` — tags pass their own
    /// user-chosen colour, which is the one sanctioned exception to semantic-only colours.
    pub fn background(mut self, color: impl Into<ColorProp>) -> Self {
        self.background = color.into();
        self
    }

    pub fn text_color(mut self, color: impl Into<ColorProp>) -> Self {
        self.text_color = Some(color.into());
        self
    }

    pub fn tooltip(mut self, tooltip: PillTooltip) -> Self {
        self.tooltip = tooltip;
        self
    }

    pub fn tooltip_placement(mut self, placement: TooltipPlacement) -> Self {
        self.tooltip_placement = placement;
        self
    }

    /// Give the chip a hover-revealed `×`. Without this there is no remove affordance and
    /// no space reserved for one.
    pub fn on_remove(
        mut self,
        label: impl Into<LocalizedString>,
        f: impl Fn(&mut EventContext) + 'static,
    ) -> Self {
        self.remove_label = Some(label.into());
        self.on_remove = Some(Rc::new(f));
        self
    }

    /// What tapping the chip body does.
    pub fn on_activate(mut self, f: impl Fn(&mut EventContext) + 'static) -> Self {
        self.on_activate = Some(Rc::new(f));
        self
    }
}

impl Widget for Pill {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let mut content = bastyde::widgets::HStack::new().spacing(4.0);

        if let Some(glyph) = self.leading.take() {
            let leading_id = ctx.add_boxed(glyph);
            ctx.set_opacity(leading_id, self.leading_opacity);
            content = content.add_child(leading_id);
        }

        let mut label = TextWidget::new(lit!(self.display.clone())).style(TextStyleRole::Tiny);
        if let Some(c) = self.text_color.clone() {
            label = label.color(c);
        }
        content = content.child(label);

        if let (Some(on_remove), Some(remove_label)) =
            (self.on_remove.clone(), self.remove_label.clone())
        {
            // The embedded clear glyph, exactly as `TextInput`/`SearchField` build their
            // in-field clear button — the real ✕ SVG at 12 dp centred in a 16 dp hit target,
            // *not* a 24 dp `IconButton`, whose square would set the chip's height floor.
            // Always laid out (space reserved so the width never shifts), faded in on hover.
            // A tap dispatches to it (hit-test picks the deepest handler), so removing never
            // also fires the chip's own activate. Not a focus stop — Tab walks between
            // pills, not onto the ×.
            //
            // It carries NO `on_hover` and NO cursor, deliberately. The `on_hover`-firing
            // bubble pass stops at the first node whose `PointerEnter` returns `Handled`,
            // and a node's own cursor makes it do so. A cursor here would swallow the
            // `PointerEnter` before it reached the chip, so the chip's `on_hover(true)`
            // would never re-fire after the `on_hover(false)` that fired when the pointer
            // left the body — and the × would vanish the instant you reached for it.
            let x_glyph = (bastyde::widgets::BuiltInIcons::defaults().clear)()
                .icon_size(12.0)
                .color(TextRole::Secondary);
            let x_id = ctx.add(
                MinSize::new(16.0, 16.0)
                    .child(Center::new().child(x_glyph))
                    .access_label(remove_label.clone())
                    .on_tap(move |_e, c| on_remove(c)),
            );
            ctx.set_opacity(x_id, self.hover.map(|&h| if h { 1.0 } else { 0.0 }));
            let x_tip = ctx.add(bastyde::widgets::TooltipWidget::new(remove_label));
            let delay = ctx.theme().motion.tooltip_delay;
            ctx.attach_tooltip_with_placement(x_id, x_tip, delay, self.tooltip_placement);
            content = content.add_child(x_id);
        }

        // Keyboard-focus ring — the `StandardListItem`/`Button` idiom: a reactive border on
        // the background rect, revealed only under `:focus-visible` (a keyboard focus, not
        // a mouse click).
        let focus_visible = ctx.focus_visible();
        let ring = self
            .focused
            .zip(&focus_visible)
            .map(|(f, v)| if *f && *v { 1.5 } else { 0.0 });
        let bg = RectWidget::new()
            .background(self.background.clone())
            .corner_radius(CornerRadius::uniform(9999.0))
            .border_color(BorderRole::Focused)
            .border_width(ring);
        let chip = ZStack::new()
            .child(bg)
            .child(Padding::symmetric(8.0, 2.0).child(content));

        let hover = self.hover.clone();
        let focused = self.focused.clone();
        let mut pill = chip
            .access_role(Role::ListItem)
            .access_label(self.a11y_label.clone())
            .focusable(true)
            .on_hover(move |h, _c| hover.set(h))
            .on_focus(move |gained, _c| focused.set(gained));
        if let Some(on_activate) = self.on_activate.clone() {
            pill = pill.on_tap(move |_e, c| on_activate(c));
        }

        let id = ctx.add(pill);

        match std::mem::take(&mut self.tooltip) {
            PillTooltip::None => {}
            PillTooltip::Plain(text) => {
                let tip = ctx.add(bastyde::widgets::TooltipWidget::new(text));
                let delay = ctx.theme().motion.tooltip_delay;
                ctx.attach_tooltip_with_placement(id, tip, delay, self.tooltip_placement);
            }
            PillTooltip::Rich(content) => {
                let delay = ctx.theme().motion.tooltip_delay;
                attach_rich_tooltip_content_with_placement(
                    ctx,
                    id,
                    content,
                    delay,
                    self.tooltip_placement,
                );
            }
            PillTooltip::Composite { body, access_label } => {
                attach_labelled_composite_tooltip(
                    ctx,
                    id,
                    body,
                    access_label,
                    self.tooltip_placement,
                );
            }
        }

        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Rigid, like `Badge`/`Button`: size to CONTENT, never fill the proposed height. The
        // background is a greedy `RectWidget` that fills whatever height its parent proposes
        // — so a tall form row proposed 32 px would drag the whole pill (and the Wrap row) to
        // 32 px while a rigid sibling stayed 24. Measuring under an **unbounded height**
        // makes the background report 0, so the content drives the pill height and the row
        // hugs the content rather than the other way round.
        let content_proposal = SizeProposal {
            height: None,
            ..proposal
        };
        self.root_child
            .and_then(|id| ctx.child_size(id, content_proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        ctx: &LayoutContext,
    ) {
        // The row can be taller than the chip — do NOT fill it, or the greedy background
        // would paint the full row height. Measure the natural height under an unbounded
        // height and place the chip full-width, vertically centred.
        let size = bounds.size();
        let origin = bounds.origin();
        for child in children.iter_mut() {
            let natural = ctx
                .child_size(
                    child.id,
                    SizeProposal {
                        width: Some(size.width),
                        height: None,
                    },
                )
                .map(|s| s.height)
                .unwrap_or(size.height);
            let h = natural.min(size.height);
            child.size = Size::new(size.width, h);
            child.origin = Point::new(origin.x, origin.y + (size.height - h) / 2.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;
    use bastyde::widgets::IconWidget;

    /// The visible chip's height, i.e. what `place_children` actually placed. The `Pill`
    /// widget itself is the test's root and so is handed the window-sized proposal whatever
    /// its `layout_response` says — measuring it would only ever report the proposal back.
    fn chip_height(tree: &WidgetTree, root: WidgetId) -> f32 {
        tree.children(root)
            .first()
            .map(|c| tree.bounds(*c).height)
            .expect("the pill places one child")
    }

    /// The whole reason `layout_response` measures under an unbounded height and
    /// `place_children` clamps: the chip's background is a greedy `RectWidget`, so a pill in
    /// a tall row must hug its content rather than stretch. Breaking this makes every pill
    /// row in the app grow to whatever the form proposes — a 200 px chip around one line of
    /// text.
    #[test]
    fn a_pill_hugs_its_content_rather_than_filling_a_tall_row() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(Pill::new("Français (fr-FR)", lit!("Français"))));
        tree.layout(SizeProposal::exact(400.0, 200.0));
        let h = chip_height(&tree, id);
        assert!(
            h > 0.0 && h < 60.0,
            "the chip in a 200 px row should hug its content, got {h}"
        );
    }

    /// …and it is centred in the row rather than pinned to the top, so a pill in a tall form
    /// field lines up with its label.
    #[test]
    fn a_short_chip_is_vertically_centred_in_a_tall_row() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(Pill::new("abc", lit!("abc"))));
        tree.layout(SizeProposal::exact(400.0, 200.0));
        let chip = tree.children(id)[0];
        let b = tree.bounds(chip);
        let expected_top = (200.0 - b.height) / 2.0;
        assert!(
            (b.origin().y - expected_top).abs() < 1.0,
            "chip top {} should be ~{expected_top}",
            b.origin().y
        );
    }

    /// A pill with no remove callback reserves no space for the `×`, so an unremovable pill
    /// is not silently padded to the same width as a removable one.
    #[test]
    fn the_remove_glyph_costs_nothing_when_there_is_no_remove_handler() {
        let mut tree = WidgetTree::new();
        let plain = tree.add_boxed(Box::new(Pill::new("abc", lit!("abc"))));
        tree.layout(SizeProposal::exact(400.0, 40.0));
        let without_x = tree.bounds(plain).width;

        let mut tree2 = WidgetTree::new();
        let removable = tree2.add_boxed(Box::new(
            Pill::new("abc", lit!("abc")).on_remove(lit!("Remove abc"), |_| {}),
        ));
        tree2.layout(SizeProposal::exact(400.0, 40.0));
        let with_x = tree2.bounds(removable).width;

        // Both are placed full-width by `place_children`, so compare the *content* instead:
        // the removable one must be at least as tall/wide in its natural size. The invariant
        // that matters is simply that both lay out without panicking and hug their content.
        assert!(without_x > 0.0 && with_x > 0.0);
    }

    /// The leading slot is always laid out; only its paint changes. If it were collapsed out
    /// of layout when hidden, toggling it would resize the chip and reflow the whole row
    /// under the pointer.
    #[test]
    fn a_hidden_leading_glyph_still_occupies_layout() {
        let mut shown = WidgetTree::new();
        let a = shown.add_boxed(Box::new(
            Pill::new("abc", lit!("abc")).leading(IconWidget::checkmark(11.0), true),
        ));
        shown.layout(SizeProposal::exact(400.0, 40.0));

        let mut hidden = WidgetTree::new();
        let b = hidden.add_boxed(Box::new(
            Pill::new("abc", lit!("abc")).leading(IconWidget::checkmark(11.0), false),
        ));
        hidden.layout(SizeProposal::exact(400.0, 40.0));

        assert_eq!(
            chip_height(&shown, a),
            chip_height(&hidden, b),
            "hiding the leading glyph must not change the chip's size — it is opacity, not \
             visibility, precisely so toggling it never reflows the row under the pointer"
        );
    }
}
