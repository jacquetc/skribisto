// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Hang a registered rich tooltip on a widget that has no `.rich_tooltip(..)` of its own.
//!
//! Controls carry that builder — a `Button`, a `SpinBox`, a `Segment`. Plain content does
//! not: `TextWidget`, an icon, a row of them. That is usually right, because a tooltip
//! belongs on something you can point at and act on. It stops being right for the places
//! where the *reading* is the thing that needs explaining: a dimmed number in a table cell,
//! a line of prose stating what some targets add up to. Those have nowhere to put an
//! explanation, and they are exactly the ones a writer would otherwise have to guess at.
//!
//! Registry keys only, deliberately. Inline content would let the same concept be worded
//! two ways in two cells; a key resolves to one entry, participates in the `[label](:key)`
//! cascade, and is covered by `tooltip_registry`'s drift test.

use teksilo::prelude::*;
use teksilo::widgets::tooltip::attach_rich_tooltip;

/// A transparent wrapper: lays out exactly as its child, and answers hover with `key`'s
/// registered explainer.
pub struct RichTip<W: Widget + 'static> {
    child: Option<W>,
    key: &'static str,
    id: Option<WidgetId>,
}

impl<W: Widget + 'static> RichTip<W> {
    pub fn new(key: &'static str, child: W) -> Self {
        Self {
            child: Some(child),
            key,
            id: None,
        }
    }
}

impl<W: Widget + 'static> std::fmt::Debug for RichTip<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RichTip").field("key", &self.key).finish()
    }
}

impl<W: Widget + 'static> Widget for RichTip<W> {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let Some(child) = self.child.take() else {
            // A rebuild after the child was consumed: keep the previous id rather than
            // dropping the subtree, which would blank the cell on every repaint.
            return self.id.into_iter().collect();
        };
        let id = ctx.add(child);
        let delay = ctx.theme().motion.tooltip_delay;
        attach_rich_tooltip(ctx, id, self.key, delay);
        self.id = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.id
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.id.into_iter().collect()
    }
}
