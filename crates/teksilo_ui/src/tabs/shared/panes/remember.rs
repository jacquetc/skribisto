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

use frontend::common::entities::BinderItemRole;
use std::cell::RefCell;
use std::rc::Rc;

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
    // Both halves of the key: `Folder/Note` and `Item/Note` share a sub-role and carry
    // different bars, so the sub-role alone would have them overwrite each other's
    // remembered view. See `EditorViewMemory::stored`.
    role: BinderItemRole,
    sub_role: BinderItemSubRole,
    /// Ordered `(string id, derived SegmentId)` for this container's segments.
    ///
    /// The derivation is one-way, so this is the only route from the `SegmentId` the
    /// control fires back to the string [`EditorViewMemory`] persists. Without it the
    /// effect below would have to store the number, which is exactly what makes a
    /// remembered view unrecoverable across a restart.
    ids: Vec<(String, SegmentId)>,
    /// The segments whose chip is live rather than always shown, by string id: exactly
    /// the `visible` table [`Self::wrap`] was called with.
    ///
    /// Kept because it is the only thing that can tell a **self-heal** from the writer's
    /// own click. `SegmentedControl` writes the same signal either way (its "select the
    /// neighbour" convention, `segmented_control.rs`), and the effect below cannot
    /// otherwise distinguish "the chip I was on has just disappeared" from "I pressed
    /// the one next to it".
    visible: Vec<(String, Prop<bool>)>,
    /// Where the segment on screen is mirrored for a capture to read.
    shown: Rc<RefCell<String>>,
    /// This tab's view-state ports, so a segment the **writer** switches to can
    /// stand down a focus request that has not been consumed yet.
    ports: Rc<crate::shared::ViewStatePorts>,
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
    ///
    /// `visible` overrides a declared segment's chip from always-shown to reactive, by
    /// string id: an entry `(id, prop)` wires `Segment::visible(prop)` onto that one
    /// segment, leaving every other segment at `Segment::new`'s own default of always
    /// visible. Empty for every caller but [`super::item_note_segmented`] today.
    ///
    /// This is deliberately **not** a reason to rebuild the list `wrap` itself is called
    /// with. A `Prop::Bound` signal drives `SegmentedControl`'s own chip live, with no
    /// rebuild of this pairing, of the `Switcher`, or of any already-mounted page inside
    /// it; see `item_note_segmented`'s own doc for why that matters here.
    pub(super) fn wrap(
        tab: &ContentTab,
        items: Vec<(&str, LocalizedString, Box<dyn Widget>)>,
        visible: &[(&str, Prop<bool>)],
        shell: impl FnOnce(crate::tabs::Boxed, crate::tabs::Boxed) -> VStack,
    ) -> Self {
        let ids: Vec<(String, SegmentId)> = items
            .iter()
            .map(|(id, _, _)| ((*id).to_string(), segments::segment_id(id)))
            .collect();
        let keys: Vec<SegmentId> = ids.iter().map(|(_, k)| *k).collect();

        // A tab restored with a segment of its own opens on that one, over the
        // per-type view `ContentTab::new` seeded from `EditorViewMemory`. Resolved
        // here because this is the only place that knows which ids this container
        // actually has: a string written down before an extension was uninstalled no
        // longer names a segment, and selecting a page at random would be worse than
        // falling back to the remembered view.
        if let Some(seed) = tab.take_segment_seed()
            && let Some((_, key)) = ids.iter().find(|(id, _)| *id == seed)
        {
            tab.segment.set(Some(*key));
        }
        // What a capture writes down. Seeded here as well as kept up to date by the
        // effect below, because `ctx.effect` fires only on a *change*: a tab the
        // writer never switches would otherwise report nothing at all.
        let shown = tab.segment_shown_sink();
        if let Some(sel) = tab.segment.get()
            && let Some((id, _)) = ids.iter().find(|(_, key)| *key == sel)
        {
            *shown.borrow_mut() = id.clone();
        }

        let mut bar = SegmentedControl::new(tab.segment.clone());
        let mut content = Switcher::new(segmented_control::index_signal(&tab.segment, &keys));
        for ((id, label, pane), key) in items.into_iter().zip(keys.iter().copied()) {
            let mut segment = Segment::new(label).id(key);
            if let Some(entry) = visible.iter().find(|entry| entry.0 == id) {
                segment = segment.visible(entry.1.clone());
            }
            bar = bar.segment(segment);
            content = content.child_boxed(pane);
        }

        let col = shell(
            crate::tabs::Boxed::new(Box::new(bar)),
            crate::tabs::Boxed::new(Box::new(content)),
        );
        Self {
            segment: tab.segment.clone(),
            memory: tab.view_memory.clone(),
            role: tab.role().clone(),
            sub_role: tab.sub_role().clone(),
            visible: visible
                .iter()
                .map(|(id, prop)| ((*id).to_string(), prop.clone()))
                .collect(),
            shown,
            ports: tab.view_state_ports(),
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
        let (memory, role, sub_role) = (
            self.memory.clone(),
            self.role.clone(),
            self.sub_role.clone(),
        );
        let ids = self.ids.clone();
        let visible = self.visible.clone();
        let shown = self.shown.clone();
        let ports = self.ports.clone();
        // Reconcile the mirror against `self.segment` **now**, not only from the effect
        // below. `ctx.add_boxed` just built the whole bar-plus-`Switcher` subtree
        // synchronously, and a `Segment` hidden behind a `visible` prop (see `wrap`'s own
        // `visible` parameter) can self-heal `self.segment` away from a seed that named it
        // as part of that very build, `SegmentedControl`'s own "select the neighbour"
        // convention. That correction is a real write to the shared signal, but it lands
        // before the effect below is registered, so without this the mirror would keep
        // reporting the hidden id forever: `EditorViewMemory::remember` is never called
        // for it (nothing here treats a self-heal as the writer's own choice), yet
        // `editors_vm::remember_position` reads this same mirror into the *per-tab*
        // workspace state, which would then keep re-seeding the same unreachable id on
        // every future restore. Deliberately **not** `memory.remember(...)` and not
        // `ports.take_focus()` here: those two belong only to a genuine switch the writer
        // made, which is exactly what the effect below still gates on.
        if let Some(sel) = self.segment.get()
            && let Some((id, _)) = ids.iter().find(|(_, key)| *key == sel)
        {
            *shown.borrow_mut() = id.clone();
        }
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
                // **A self-heal is not a choice.** A `Segment` behind a `visible` prop
                // can vanish under the writer (removing a note's last discoverable tag
                // takes the "In prose" chip away while they are reading it), and
                // `SegmentedControl` then selects the neighbour by writing this very
                // signal — indistinguishable here from a click except by this: the
                // segment we were on is no longer visible. Persisting that wrote
                // `note-details` into `editor.last_view.item_note`, so every note the
                // writer opened afterwards landed on Details instead of its own text, a
                // choice they never made. The mirror below is still corrected (the
                // segment on screen really did change); the remembered view and the
                // pending focus request, which belong to a deliberate switch, are not.
                let healed = visible
                    .iter()
                    .find(|(vid, _)| *vid == *shown.borrow())
                    .is_some_and(|(_, prop)| !prop.get());
                *shown.borrow_mut() = id.clone();
                if healed {
                    return;
                }
                memory.remember(&role, &sub_role, id);
                // The writer has just chosen a different page, which outranks a focus
                // request nobody has consumed yet: a container opened on its Overview
                // arms one that no page there can honour, and firing it later, when
                // the writer eventually opens a writing segment for their own
                // reasons, would take the caret they did not ask for.
                let _ = ports.take_focus();
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

#[cfg(test)]
mod tests {
    use super::*;

    use frontend::AppContext;
    use frontend::common::entities::BinderItemRole;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::widgets::TextWidget;

    use crate::app_ids::AppIds;
    use crate::settings::{EditorTypography, EditorTypographySet};
    use crate::tabs::tab_for;

    /// A typography bundle with no real Settings behind it. This unit exercises
    /// segment bookkeeping, never the fonts an editor draws with.
    fn test_typography() -> EditorTypographySet {
        let bundle = |family: &str| EditorTypography {
            font_family: Signal::new(family.to_string()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
            size_range: crate::settings::TypographySizeRange::default(),
        };
        EditorTypographySet {
            scene: bundle("Literata"),
            synopsis: bundle("Literata"),
            notes: bundle("Inter"),
            corkboard: bundle("Literata"),
            distraction_free: bundle("Literata"),
        }
    }

    /// A folder-container tab (a Chapter, like the "remember last view" tests in
    /// `tabs::tests`) built against `mem`, with no seed and no switch yet.
    fn container_tab(ctx: &Rc<AppContext>, mem: EditorViewMemory, item_id: u64) -> ContentTab {
        tab_for(
            ctx,
            item_id,
            &BinderItemRole::Folder,
            &BinderItemSubRole::ChapterScene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            mem,
            &AppIds::new(),
        )
    }

    /// Wrap `tab` in a three-segment `RememberSegment` over dummy panes (no stream,
    /// no corkboard, no overview: this unit tests the wrapper, not the bodies it
    /// wraps, and a plain `WidgetTree` cannot mount those without an event source),
    /// mount it and lay it out, which is what installs the effect that persists a
    /// switch and stands down a focus request.
    ///
    /// The returned tree must be kept alive by the caller (`let _tree = ...`):
    /// dropping it drops the observer `build` installed along with it.
    fn mount_remember(tab: &ContentTab) -> WidgetTree {
        let items: Vec<(&str, LocalizedString, Box<dyn Widget>)> = vec![
            (
                segments::SEG_OWN,
                LocalizedString::literal("Own"),
                Box::new(TextWidget::new(LocalizedString::literal("own page"))) as Box<dyn Widget>,
            ),
            (
                segments::SEG_MANUSCRIPT,
                LocalizedString::literal("Full"),
                Box::new(TextWidget::new(LocalizedString::literal("full manuscript")))
                    as Box<dyn Widget>,
            ),
            (
                segments::SEG_SYNOPSIS,
                LocalizedString::literal("Synopsis"),
                Box::new(TextWidget::new(LocalizedString::literal("full synopsis")))
                    as Box<dyn Widget>,
            ),
        ];
        let widget = RememberSegment::wrap(tab, items, &[], |bar, content| {
            VStack::new().spacing(0.0).child(bar).child(content)
        });
        let mut tree = WidgetTree::new();
        tree.add_boxed(Box::new(widget));
        tree.layout(teksilo::prelude::SizeProposal::exact(800.0, 600.0));
        tree
    }

    /// A tab seeded with a segment it actually has opens on that one, over the
    /// per-type `EditorViewMemory` value `ContentTab::new` already seeded from. The
    /// per-*tab* record (a workspace restore) is the more specific one and must win;
    /// without this a restored tab would silently reopen wherever its type was last
    /// left, ignoring what was written down for this particular tab.
    #[test]
    fn a_seeded_segment_wins_over_the_remembered_view() {
        let ctx = Rc::new(AppContext::new());
        let tab = container_tab(&ctx, EditorViewMemory::detached(true), 1);
        // Precondition: with no seed yet, `ContentTab::new` opened on the per-type
        // memory's own page.
        assert_eq!(
            tab.segment.get(),
            Some(segments::segment_id(segments::SEG_OWN))
        );
        tab.seed_segment(segments::SEG_SYNOPSIS);
        let _tree = mount_remember(&tab);
        assert_eq!(
            tab.segment.get(),
            Some(segments::segment_id(segments::SEG_SYNOPSIS)),
            "a tab's own seeded segment must win over the per-type memory"
        );
    }

    /// A seed naming no segment this container built (an extension uninstalled
    /// between sessions) must fall back to the remembered view rather than select a
    /// page at random. That fallback is the entire reason the seed is validated
    /// here, in the one place that knows this container's actual segment ids,
    /// instead of being applied blindly wherever it was set.
    #[test]
    fn an_unresolvable_seed_falls_back_to_the_remembered_view() {
        let ctx = Rc::new(AppContext::new());
        let tab = container_tab(&ctx, EditorViewMemory::detached(true), 1);
        assert_eq!(
            tab.segment.get(),
            Some(segments::segment_id(segments::SEG_OWN)),
            "precondition: no seed yet, own page from memory"
        );
        tab.seed_segment("extension-that-is-no-longer-installed");
        let _tree = mount_remember(&tab);
        assert_eq!(
            tab.segment.get(),
            Some(segments::segment_id(segments::SEG_OWN)),
            "an unresolvable seed must fall back to the remembered view, not pick a \
             page at random"
        );
    }

    /// The seed is consumed exactly once: a rebuild of the same tab (a Promote, a
    /// settings-driven relayout) must not re-apply it over a segment the writer has
    /// since chosen for themselves. Without this, every rebuild after a restore would
    /// yank the writer back to the restored page no matter what they had switched to.
    #[test]
    fn the_seed_is_consumed_once() {
        let ctx = Rc::new(AppContext::new());
        let tab = container_tab(&ctx, EditorViewMemory::detached(true), 1);
        tab.seed_segment(segments::SEG_SYNOPSIS);
        let _tree1 = mount_remember(&tab);
        assert_eq!(
            tab.segment.get(),
            Some(segments::segment_id(segments::SEG_SYNOPSIS)),
            "precondition: the seed was applied on the first build"
        );
        // The writer picks a different page for their own reasons.
        tab.segment
            .set(Some(segments::segment_id(segments::SEG_MANUSCRIPT)));
        // A second build of the same tab must not find the seed still waiting.
        let _tree2 = mount_remember(&tab);
        assert_eq!(
            tab.segment.get(),
            Some(segments::segment_id(segments::SEG_MANUSCRIPT)),
            "a rebuild re-applied a seed that had already been consumed"
        );
    }

    /// The segment on screen is mirrored out at wrap time, not only once the writer
    /// switches: `ctx.effect` fires only on a *change*, so without this seed a tab
    /// nobody has touched would report an empty `segment_shown` even though a
    /// segmented body is plainly mounted and showing its first page.
    #[test]
    fn segment_shown_reports_the_initial_segment_before_any_switch() {
        let ctx = Rc::new(AppContext::new());
        let tab = container_tab(&ctx, EditorViewMemory::detached(true), 1);
        assert_eq!(
            tab.segment_shown(),
            "",
            "precondition: nothing has built a segmented body yet"
        );
        let _tree = mount_remember(&tab);
        assert_eq!(
            tab.segment_shown(),
            segments::SEG_OWN,
            "the mirror must be seeded at wrap time, or an untouched tab would be \
             captured as having no segment at all"
        );
    }

    /// Switching segment as the **writer** stands down an unconsumed focus request.
    /// Without this, a container opened on its Overview (from an activation that
    /// requested focus but mounted no writing page to give it to) would park that
    /// request, and it would fire later, uninvited, the first time the writer opens a
    /// writing segment for reasons of their own.
    #[test]
    fn a_writer_switch_stands_down_an_unconsumed_focus_request() {
        let ctx = Rc::new(AppContext::new());
        let tab = container_tab(&ctx, EditorViewMemory::detached(true), 1);
        let _tree = mount_remember(&tab);
        tab.request_focus();
        assert!(
            tab.view_state_ports().wants_after_mount(),
            "precondition: a focus request is armed"
        );
        tab.segment
            .set(Some(segments::segment_id(segments::SEG_MANUSCRIPT)));
        assert!(
            !tab.view_state_ports().wants_after_mount(),
            "the writer's own switch must stand down a focus request nothing had \
             consumed yet"
        );
    }

    /// The existing "remember last view" behaviour still holds through this change:
    /// a writer's switch still writes the chosen segment into the per-type
    /// `EditorViewMemory`, so a newly-opened tab of the same type still inherits it.
    #[test]
    fn a_writer_switch_still_persists_into_the_per_type_memory() {
        let ctx = Rc::new(AppContext::new());
        let mem = EditorViewMemory::detached(true);
        let tab = container_tab(&ctx, mem.clone(), 1);
        let _tree = mount_remember(&tab);
        tab.segment
            .set(Some(segments::segment_id(segments::SEG_SYNOPSIS)));
        assert_eq!(
            mem.initial(&BinderItemRole::Folder, &BinderItemSubRole::ChapterScene),
            Some(segments::segment_id(segments::SEG_SYNOPSIS)),
            "a container's switch must still be remembered per type"
        );
    }
}
