// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The segmented control that remembers which page a container was left on.
//!
//! Builds the `SegmentedControl` and its `Switcher` from **one** ordered list in
//! one pass, so the two cannot disagree about which segment is which. It also
//! carries the `(string id, SegmentId)` table the remembered view persists
//! against: `segment_id` derives the numeric id from the string one-way, so the
//! string is what has to be written down.

use super::*;

/// Transparent passthrough that persists the container's `SegmentedControl`
/// selection into the per-type [`EditorViewMemory`] whenever it changes, so a
/// newly-opened tab of the same item type inherits it.
///
/// `SegmentedControl` has no change-callback and [`folder_segmented`] has no build
/// context, so the effect is set up here (in a widget's `build`). Mirrors
/// `editor::DirtyOnEdit`: it adds one child and forwards layout to it unchanged.
pub(super) struct RememberSegment {
    segment: Signal<Option<SegmentId>>,
    memory: EditorViewMemory,
    sub_role: BinderItemSubRole,
    /// Ordered `(string id, derived SegmentId)` for this container's segments.
    ///
    /// The derivation is one-way, so this is the only route from the `SegmentId` the
    /// control fires back to the string [`EditorViewMemory`] persists. Without it the
    /// effect below would have to store the number, which is exactly what makes a
    /// remembered view unrecoverable across a restart.
    ids: Vec<(String, SegmentId)>,
    child: Option<Box<dyn Widget>>,
    child_id: Option<WidgetId>,
}

impl RememberSegment {
    /// Build the bar and its `Switcher` from one ordered list and wrap the result.
    ///
    /// Both are built here, from the same slice, in the same pass — the pairing is
    /// positional by `SegmentedControl`'s contract, and this is what makes it true rather
    /// than merely intended. Every segment pins an explicit id: `Segment::new` would
    /// otherwise mint `SegmentId::fresh()`, a process-global counter, and a remembered
    /// view keyed on one of those could never be found again after a restart.
    pub(super) fn wrap(
        tab: &ContentTab,
        items: Vec<(&str, LocalizedString, Box<dyn Widget>)>,
        shell: impl FnOnce(crate::tabs::Boxed, crate::tabs::Boxed) -> VStack,
    ) -> Self {
        let ids: Vec<(String, SegmentId)> = items
            .iter()
            .map(|(id, _, _)| ((*id).to_string(), segments::segment_id(id)))
            .collect();
        let keys: Vec<SegmentId> = ids.iter().map(|(_, k)| *k).collect();

        let mut bar = SegmentedControl::new(tab.segment.clone());
        let mut content = Switcher::new(segmented_control::index_signal(&tab.segment, &keys));
        for ((_, label, pane), key) in items.into_iter().zip(keys.iter().copied()) {
            bar = bar.segment(Segment::new(label).id(key));
            content = content.child_boxed(pane);
        }

        let col = shell(
            crate::tabs::Boxed::new(Box::new(bar)),
            crate::tabs::Boxed::new(Box::new(content)),
        );
        Self {
            segment: tab.segment.clone(),
            memory: tab.view_memory.clone(),
            sub_role: tab.sub_role().clone(),
            ids,
            child: Some(tab_backdrop(tab.backdrop_role(), col)),
            child_id: None,
        }
    }
}

impl std::fmt::Debug for RememberSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RememberSegment").finish_non_exhaustive()
    }
}

impl Widget for RememberSegment {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let child = self.child.take().expect("RememberSegment built once");
        let id = ctx.add_boxed(child);
        self.child_id = Some(id);
        let (memory, sub_role) = (self.memory.clone(), self.sub_role.clone());
        let ids = self.ids.clone();
        // `ctx.effect` fires only on *changes*, not on setup — so a rebuild installs
        // a fresh observer that stays quiet until the user actually switches the
        // `SegmentedControl`. That's what keeps a rebuild of one tab from writing its
        // segment over the view another same-type tab just chose (regression-tested by
        // `tabs::tests::same_type_tabs_share_one_last_view_and_the_last_switch_wins`).
        ctx.effect(&self.segment, move |v| {
            // Look the fired id back up to the string it was derived from. An id this
            // container does not own (a stale remembered value from a segment that is no
            // longer registered) is simply not persisted — better than writing back a
            // string nothing will resolve next launch.
            if let Some(sel) = *v
                && let Some((id, _)) = ids.iter().find(|(_, key)| *key == sel)
            {
                memory.remember(&sub_role, id);
            }
        });
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child_id
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = bounds.size();
        }
    }

    // A filling child must be reported here too (not just from `build`), or the
    // layout pass never places it — the container's segmented bar + panes vanish.
    // (Mirrors `editor::VisibleWhen`, which wraps the same kind of boxed body.)
    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}
