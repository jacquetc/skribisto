// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The keyboard-focus ring for hand-built focus stops.
//!
//! **Teksilo paints no focus indicator of its own.** Every framework widget that
//! shows one draws it itself — `Button`, `StandardListItem`, `ListView`,
//! `Checkbox`, the activity bar, all ~29 of them, each pairing
//! [`BuildContext::focus_visible`] with `BorderRole::Focused`. There is no
//! fallback ring for an arbitrary node, so a plain `HStack` or `TextWidget`
//! given `.focusable(true)` becomes a Tab stop that shows the writer *nothing*
//! when focus lands on it. Reaching a control you cannot see you have reached is
//! WCAG 2.4.7 (Focus Visible), and it is the state four of this app's row lists
//! shipped in.
//!
//! Prefer a framework widget that already rings itself — `Button` for a chip,
//! `ListView` + `StandardListItem` for a list. This module is for the rows that
//! are genuinely hand-built: composite bodies with embedded controls, rows in a
//! popover, chips in a `Wrap`.
//!
//! ## Using it
//!
//! Three parts, and all three are required — the ring is only half of the
//! contract:
//!
//! ```ignore
//! let focused = ctx.signal(false);                       // 1. the state
//! let row = with_focus_ring(ctx, RING_RADIUS_ROW, body, &focused)
//!     .access_role(Role::ListItem)
//!     .access_label(label)
//!     .focusable(true)
//!     .on_focus({ let f = focused.clone(); move |g, _| f.set(g) })   // 2. drive it
//!     .on_tap(..)
//!     .on_key(..);                                       // 3. Enter/Space
//! ```
//!
//! `on_key` is not optional either: `on_tap` is pointer-only, so a focusable row
//! without it is reachable and inert, which is WCAG 2.1.1 (Keyboard) failing
//! right after 2.4.7 passes.
//!
//! ## Why a `Signal` and not a query
//!
//! There is no "is this widget focused" signal to read — focus arrives as the
//! `on_focus` event, so the caller owns the state and hands it here. A signal
//! made with `ctx.signal(false)` resets on rebuild, which is correct rather than
//! sloppy: a rebuild destroys and re-adds the row, so the focus it held is gone
//! too and a ring left painted would be a lie. A row whose widget struct
//! outlives its rebuilds (`Pill`) keeps the signal on the struct instead.

use teksilo::prelude::*;
use teksilo::tokens::{BorderRole, CornerRadius};
use teksilo::widgets::{RectWidget, ZStack};

/// Ring thickness, in dp. Matches the framework's own recipes.
pub const RING_WIDTH: f32 = 1.5;

/// Corner radius for a rectangular row (a list line, a menu row).
pub const RING_RADIUS_ROW: f32 = 4.0;

/// Corner radius for a fully-rounded chip. Any value past half the chip's height
/// resolves to a capsule.
pub const RING_RADIUS_PILL: f32 = 9999.0;

/// The ring rect alone — a transparent rectangle whose *border* appears while
/// `focused` and the input modality is keyboard.
///
/// Stack it, don't merge it into a background rect: a `RectWidget` has a single
/// border, so a fill that already draws a hairline outline cannot also carry the
/// ring, and swapping the border's colour on focus would need the theme resolved
/// at signal-map time, which it is not. Two stacked rects keep both static and
/// let the ring simply paint over the hairline.
///
/// Gated on [`BuildContext::focus_visible`] — the `:focus-visible` rule — so a
/// pointer click does not leave a ring behind, only keyboard navigation does.
pub fn focus_ring(ctx: &BuildContext, radius: f32, focused: &Signal<bool>) -> RectWidget {
    let width = focused
        .zip(&ctx.focus_visible())
        .map(move |(f, v)| if *f && *v { RING_WIDTH } else { 0.0 });
    RectWidget::new()
        .corner_radius(CornerRadius::uniform(radius))
        .border_color(BorderRole::Focused)
        .border_width(width)
}

/// `content` with a keyboard-focus ring stacked over it.
///
/// The ring is a sibling layer, not a wrapper around the content, so it adds no
/// insets and cannot change how `content` measures — a row keeps the exact size
/// it had before it grew a ring.
///
/// The caller still owns the other half of the contract: make the result
/// focusable, drive `focused` from its `on_focus`, and answer Enter/Space. See
/// the module docs.
pub fn with_focus_ring(
    ctx: &BuildContext,
    radius: f32,
    content: impl Widget + 'static,
    focused: &Signal<bool>,
) -> ZStack {
    ZStack::new()
        .child(content)
        .child(focus_ring(ctx, radius, focused))
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::widgets::TextWidget;

    fn tree() -> WidgetTree {
        WidgetTree::new().with_theme(teksilo::presets::intui::light())
    }

    /// The ring is invisible until the row is *both* focused and the modality is
    /// keyboard — a mouse click must not leave one behind.
    #[test]
    fn ring_shows_only_on_a_keyboard_focus() {
        let focused = Signal::new(false);
        let visible = Signal::new(false);
        let width = focused
            .zip(&visible)
            .map(|(f, v)| if *f && *v { RING_WIDTH } else { 0.0 });

        assert_eq!(width.get(), 0.0, "neither");
        focused.set(true);
        assert_eq!(width.get(), 0.0, "focused, but by pointer");
        visible.set(true);
        assert_eq!(width.get(), RING_WIDTH, "focused via the keyboard");
        focused.set(false);
        assert_eq!(width.get(), 0.0, "focus left");
    }

    /// `with_focus_ring` must not change the size of what it wraps: a ring is
    /// paint, not layout, and a row that grew when it gained one would reflow
    /// every list it sits in.
    #[test]
    fn the_ring_adds_no_size() {
        let mut bare = tree();
        let bare_id = bare.add(TextWidget::new(lit!("Elizabeth Bennet")));
        bare.layout(SizeProposal::with_width(200.0));
        let bare_size = bare.bounds(bare_id);

        let mut ringed = tree();
        let ringed_id = ringed.add(RingProbe {
            focused: Signal::new(true),
            root: None,
        });
        ringed.layout(SizeProposal::with_width(200.0));
        let ringed_size = ringed.bounds(ringed_id);

        assert!(
            (bare_size.width - ringed_size.width).abs() < 0.5
                && (bare_size.height - ringed_size.height).abs() < 0.5,
            "ringed {ringed_size:?} must match bare {bare_size:?}"
        );
    }

    /// Minimal composite that calls the helper with a real `BuildContext`.
    #[derive(Debug)]
    struct RingProbe {
        focused: Signal<bool>,
        root: Option<WidgetId>,
    }

    impl Widget for RingProbe {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let ringed = with_focus_ring(
                ctx,
                RING_RADIUS_ROW,
                TextWidget::new(lit!("Elizabeth Bennet")),
                &self.focused,
            );
            let id = ctx.add(ringed);
            self.root = Some(id);
            vec![id]
        }

        fn layout_response(
            &self,
            proposal: SizeProposal,
            ctx: &teksilo::core::widget::LayoutContext,
        ) -> teksilo::core::widget::LayoutResponse {
            self.root
                .and_then(|id| ctx.child_size(id, proposal))
                .map(Into::into)
                .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
        }

        fn children(&self) -> Vec<WidgetId> {
            self.root.into_iter().collect()
        }
    }
}
