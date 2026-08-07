// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Tags as a row of coloured dots, for the surfaces where a full chip would be too loud.
//!
//! The Inspector shows tags as named [`Pill`](crate::widgets::Pill)s because that is where
//! you go to *edit* them. Everywhere else the job is **passive awareness** — noticing that a
//! scene is still a draft without having asked. A row of dots costs almost no width, and the
//! name arrives on hover, per dot.
//!
//! Attached to the stream row header, the corkboard card and the editor subtitle.
//! Deliberately **not** the binder tree: that is the densest, most-read surface in the app
//! and already carries an icon, a title and a subtitle.
//!
//! ## Hover is per dot, click is per row
//!
//! Each dot owns a composite tooltip (its name, and its description when it has one), so
//! hovering inspects *one* tag. Clicking anywhere on the row — including on a dot — opens the
//! picker, so acting applies to *all* of them.
//!
//! That split is why the dots carry **no tap handler of their own**. `ensure_gesture_arena`
//! only installs an arena for a node that registered a gesture handler, and a descendant
//! `on_tap` *captures the pointer on PointerDown* (`bastyde-core`'s own
//! `ancestor_drag_starts_through_descendant_tap_capture` pins this: "a click on the child
//! fires the descendant tap", "a click must not start the ancestor drag"). Give the dots
//! handlers and they would swallow the row's tap, leaving the popover reachable only through
//! the slivers between them. Handler-free, the press bubbles — through several levels, per
//! `ancestor_drag_starts_through_deeply_nested_tap_capture` — to the `Popover` trigger.
//!
//! Tooltips are unaffected either way: they hang off `PointerMove` hit-testing
//! (`tooltip_pointer_enter` on the raw target), not off the gesture arena.

use bastyde::core::BindingLevel;
use bastyde::core::accesskit::Role;
use bastyde::core::overlay::TooltipPlacement;
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::tokens::{BorderRole, CornerRadius};
use bastyde::widgets::{Center, HStack, MinSize, Popover, RectWidget, TextWidget, ZStack};

use crate::models::TagRow;
use crate::tags::contrast;
use crate::tags::tag_pill_field::{SetTags, TagPicker};
use crate::tags::tag_tooltip::tag_tooltip_body;
use crate::widgets::attach_labelled_composite_tooltip;

/// Painted diameter of one dot.
///
/// Smaller than the 10 dp swatch the picker rows and tooltips use, because those sit beside
/// a name and these do not: three of them pack into a stream row's header without competing
/// with the title. Not so small that the discoverable ring around it stops reading.
const DOT: f32 = 8.0;

/// Hit area per dot. The dots tile the row at this pitch with no spacing between them, so
/// every pixel of the row belongs to some dot and there is nothing to "miss" — the whole row
/// is one target for the tap, subdivided only for hover.
///
/// 18 dp is below the 24 dp comfortable minimum on purpose: this is a *hover* subdivision on
/// a dense row, and the tap it sits inside is the full row, which is far larger.
const HIT: f32 = 18.0;

/// Gap between the fill and the discoverable ring, and the ring's own width.
const RING_GAP: f32 = 2.0;
const RING_WIDTH: f32 = 1.0;

/// Tags shown before the rest collapse into a "+N".
///
/// Per-surface because the widths differ by an order of magnitude: a corkboard card is a few
/// hundred dp, an editor subtitle spans the column.
pub const MAX_VISIBLE_STREAM: usize = 5;
pub const MAX_VISIBLE_CORKBOARD: usize = 4;
pub const MAX_VISIBLE_EDITOR: usize = 8;
/// The Overview's table cell is the tightest of the four surfaces — a fixed-width column
/// competing with five others in a pane that may be half a split window — so it shows the
/// fewest dots before collapsing to the overflow count.
pub const MAX_VISIBLE_OVERVIEW: usize = 3;

/// The dots themselves, from resolved rows — no view-model, no popover.
///
/// Split from [`TagDotsRow`] so the geometry and the accessibility tree can be tested without
/// an `AppContext`: this crate has no way to hand a `TagsViewModel` to a headless
/// `WidgetTree`, and everything worth pinning here (tiling, hit size, the ring, the overflow
/// cell, the row's label) is a pure function of the rows.
pub struct TagChipRow {
    tags: Vec<TagRow>,
    max_visible: usize,
    root_child: Option<WidgetId>,
}

impl TagChipRow {
    pub fn new(tags: Vec<TagRow>, max_visible: usize) -> Self {
        Self {
            tags,
            max_visible: max_visible.max(1),
            root_child: None,
        }
    }

    /// What a screen reader hears for the whole row. The dots are decoration; this is the
    /// one place the tag names exist as text.
    fn row_label(&self) -> String {
        self.tags
            .iter()
            .map(|t| t.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// How many dots are drawn, and which tags the "+N" cell is hiding.
    ///
    /// Split out of `build` so the claim can be tested: the overflow cell names what it is
    /// hiding (the whole reason it carries a tooltip), and an off-by-one here would silently
    /// produce a "+2" that names three tags, or names the wrong two.
    fn split_at_cap(&self) -> (usize, Vec<String>) {
        let shown = self.tags.len().min(self.max_visible);
        let hidden = self.tags[shown..].iter().map(|t| t.name.clone()).collect();
        (shown, hidden)
    }
}

impl std::fmt::Debug for TagChipRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagChipRow")
            .field("tags", &self.tags.len())
            .finish()
    }
}

impl Widget for TagChipRow {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Zero spacing: the hit cells must abut, or the gaps between them would be dead
        // zones for hover (the tap still works there, since it belongs to the row).
        let mut row = HStack::new().spacing(0.0);

        let (shown, rest) = self.split_at_cap();
        for tag in self.tags.iter().take(shown) {
            let cell = ctx.add(dot_cell(tag));
            // Per dot, so hovering one never shows another's. `tag_tooltip_body` omits the
            // description line when the tag has none, so a bare tag gets a one-line tip
            // rather than one with a hole in it.
            attach_labelled_composite_tooltip(
                ctx,
                cell,
                Box::new(tag_tooltip_body(tag)),
                lit!(tag.name.clone()),
                TooltipPlacement::Below,
            );
            row = row.add_child(cell);
        }

        let hidden = rest.len();
        if hidden > 0 {
            let label = tr!(tags_chip_more(n = hidden as i64));
            let cell = ctx.add(
                MinSize::new(HIT, HIT).child(
                    Center::new().child(
                        TextWidget::new(lit!(format!("+{hidden}")))
                            .style(TextStyleRole::Tiny)
                            .color(TextRole::Secondary),
                    ),
                ),
            );
            // The overflow cell names what it is hiding, so the count is not a dead end.
            attach_labelled_composite_tooltip(
                ctx,
                cell,
                Box::new(
                    TextWidget::new(lit!(rest.join(", ")))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::TooltipText),
                ),
                label,
                TooltipPlacement::Below,
            );
            row = row.add_child(cell);
        }

        let id = ctx.add(row);
        self.root_child = Some(id);
        vec![id]
    }

    /// The row is the meaningful unit, not the dots: one node naming every tag. The dots
    /// themselves add no nodes — a screen reader walking eight anonymous graphics would be
    /// worse than one label that reads them out.
    ///
    /// A **`Label`, not a `Button`**, even though the row is clickable. The `Popover` wraps
    /// its trigger in an `OverlayTrigger`, which already contributes `Role::Button` with
    /// `has_popup` and the live `expanded` state — better button semantics than this widget
    /// could supply. Claiming `Role::Button` here too put two buttons at identical bounds,
    /// so the control was announced twice. This node's job is only to say *which tags*,
    /// inside the button the framework provides.
    fn accessibility(&self, builder: &mut bastyde::core::accessibility::AccessNodeBuilder) {
        builder.set_role(Role::Label);
        builder.set_name(self.row_label());
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

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }
}

/// One dot in its hit cell: the fill with its derived hairline, plus a concentric ring when
/// the tag is story-bible material.
fn dot_cell(tag: &TagRow) -> impl Widget {
    let fill = contrast::parse(&tag.color);
    // The hairline is derived from the fill rather than taken from a border token, because
    // no token clears SC 1.4.11's 3:1 against an arbitrary user-chosen colour — see
    // `contrast::outline_on`. Without it a near-white tag is an invisible dot.
    let core = RectWidget::new()
        .background(fill)
        .corner_radius(CornerRadius::uniform(9999.0))
        .border_color(contrast::outline_on(fill))
        .border_width(1.0);

    let dot: Box<dyn Widget> = if tag.discoverable {
        // A second, larger, concentric ring — a separate rect because one rect draws one
        // border, and the inner hairline must not be spent on this (it is doing the
        // accessibility work). Chrome-coloured, not the tag's colour: "this tag is in the
        // story bible" must read identically for every tag, or it is not a signal.
        Box::new(
            ZStack::new()
                .child(
                    RectWidget::new()
                        .corner_radius(CornerRadius::uniform(9999.0))
                        .border_color(BorderRole::Strong)
                        .border_width(RING_WIDTH),
                )
                .child(Center::new().child(MinSize::new(DOT, DOT).child(core))),
        )
    } else {
        Box::new(MinSize::new(DOT, DOT).child(core))
    };

    let painted = if tag.discoverable {
        DOT + 2.0 * (RING_GAP + RING_WIDTH)
    } else {
        DOT
    };

    MinSize::new(HIT, HIT).child(Center::new().child(FixedDot {
        size: painted,
        child: Some(dot),
        child_id: None,
    }))
}

/// Pins a dot to an exact square. `MinSize` only floors a size, so inside a row it would
/// stretch to the row height and paint an oval — the bug the settings pane's swatch already
/// hit once.
struct FixedDot {
    size: f32,
    child: Option<Box<dyn Widget>>,
    child_id: Option<WidgetId>,
}

impl std::fmt::Debug for FixedDot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FixedDot")
            .field("size", &self.size)
            .finish()
    }
}

impl Widget for FixedDot {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let id = match self.child.take() {
            Some(w) => ctx.add_boxed(w),
            None => return Vec::new(),
        };
        self.child_id = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        proposal.resolve(self.size, self.size).into()
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = Size::new(self.size, self.size);
        }
    }
}

/// The mounted unit: resolves ids against the palette and puts the picker behind a tap.
///
/// Renders nothing at all when the item has no tags — an empty row would otherwise reserve
/// 18 dp of height on every untagged item in the binder, which is most of them, and offer a
/// popover with no visible affordance.
///
/// It reads `TagsViewModel` from `app_state` itself rather than taking one. All three call
/// sites (the stream row header, the corkboard card, the editor subtitle) are plain
/// composition functions with no `BuildContext`, so a passed-in view-model would have to be
/// threaded down from `App::build` through `EditorsViewModel` and `ContentTab`'s
/// already-nine-parameter constructor. This widget's own `build` has the context those
/// functions lack, and the view-model is a documented singleton, so fetching it here is both
/// shorter and impossible to get wrong.
pub struct TagDotsRow {
    value: Signal<Vec<u64>>,
    set: SetTags,
    max_visible: usize,
    root_child: Option<WidgetId>,
}

impl TagDotsRow {
    pub fn new(value: Signal<Vec<u64>>, set: SetTags, max_visible: usize) -> Self {
        Self {
            value,
            set,
            max_visible,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for TagDotsRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagDotsRow").finish()
    }
}

impl Widget for TagDotsRow {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let Some(vm) = ctx
            .app_state::<crate::view_models::TagsViewModel>()
            .cloned()
        else {
            // No work open: nothing to resolve ids against.
            self.root_child = None;
            return Vec::new();
        };

        // Deliberately binds NOTHING. This widget owns the `Popover`, and a rebuild here
        // replaces that popover with a fresh one — tearing down the open overlay on every tag
        // toggle. The reactive part lives one level down in [`ChipDots`], so the dots repaint
        // while the popover above them stays open.
        let chips = ChipDots {
            value: self.value.clone(),
            vm: vm.clone(),
            max_visible: self.max_visible,
            root_child: None,
        };
        let picker = TagPicker::new(self.value.clone(), self.set.clone(), vm);
        let id = ctx.add(
            Popover::new(tr!(tags_pill_list()))
                .trigger(chips)
                .content(picker),
        );
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        match self.root_child {
            Some(id) => ctx
                .child_size(id, proposal)
                .map(LayoutResponse::from)
                .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into()),
            // No tags: take no space at all.
            None => proposal.resolve(0.0, 0.0).into(),
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }
}

/// The reactive half of [`TagDotsRow`]: resolves ids to palette rows and rebuilds when
/// either changes.
///
/// Separate from `TagDotsRow` purely for lifetime reasons — it is the `Popover`'s trigger,
/// so it may rebuild freely, whereas rebuilding its parent would close the popover.
struct ChipDots {
    value: Signal<Vec<u64>>,
    vm: crate::view_models::TagsViewModel,
    max_visible: usize,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for ChipDots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChipDots").finish()
    }
}

impl Widget for ChipDots {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // The item's own tags, and the palette behind them — a rename or recolour in
        // Settings must repaint every dot on screen.
        self.value
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        self.vm.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        let assigned = self.value.get();
        // Resolve through the id → row map (`WorkTagsListModel::lookup_signal`), not by
        // scanning the palette: `rows()` clones every row's three Strings, and doing that per
        // chip per rebuild on every row of a stream is an O(rows × tags) scan with a full
        // clone on top.
        //
        // Ids with no palette row are skipped rather than drawn as a placeholder: that only
        // happens mid-delete, and a phantom dot would outlive the tag.
        let lookup = self.vm.lookup_signal().get();
        let mut rows: Vec<TagRow> = assigned
            .iter()
            .filter_map(|id| lookup.get(id).cloned())
            .collect();
        // Palette order (alphabetical), not assignment order, so the same set of tags always
        // looks the same wherever it is shown. A HashMap has no order, so this has to be
        // restored explicitly — through the model's own comparator, since the ordering is
        // load-bearing (it is what makes `status/…` cluster) and a second spelling of it
        // would be a second ordering.
        crate::models::sort_rows(&mut rows);
        if rows.is_empty() {
            self.root_child = None;
            return Vec::new();
        }

        let id = ctx.add(TagChipRow::new(rows, self.max_visible));
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        match self.root_child {
            Some(id) => ctx
                .child_size(id, proposal)
                .map(LayoutResponse::from)
                .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into()),
            // No tags: take no space at all, so an untagged item is unchanged.
            None => proposal.resolve(0.0, 0.0).into(),
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;

    fn tag(id: u64, name: &str, color: &str, discoverable: bool) -> TagRow {
        TagRow {
            id,
            name: name.to_string(),
            color: color.to_string(),
            details: String::new(),
            discoverable,
        }
    }

    fn three() -> Vec<TagRow> {
        vec![
            tag(1, "status/draft", "#2e7d32", false),
            tag(2, "character", "#1565c0", true),
            tag(3, "needs research", "#ffffff", false),
        ]
    }

    /// The cells the row actually placed (the `HStack`'s children).
    fn cells(tree: &WidgetTree, root: WidgetId) -> Vec<WidgetId> {
        let row = tree.children(root)[0];
        tree.children(row)
    }

    #[test]
    fn one_cell_per_tag() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(TagChipRow::new(three(), 10)));
        tree.layout(SizeProposal::exact(400.0, 40.0));
        assert_eq!(cells(&tree, id).len(), 3);
    }

    /// Hover subdivides the row, so a gap between cells would be a spot where hovering shows
    /// nothing while the pixels either side name a tag.
    #[test]
    fn hit_cells_tile_the_row_without_gaps() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(TagChipRow::new(three(), 10)));
        tree.layout(SizeProposal::exact(400.0, 40.0));
        let cs = cells(&tree, id);
        for pair in cs.windows(2) {
            let left = tree.bounds(pair[0]);
            let right = tree.bounds(pair[1]);
            let gap = right.origin().x - (left.origin().x + left.width);
            assert!(
                gap.abs() < 0.51,
                "cells must abut, found a {gap} dp gap between them"
            );
        }
    }

    #[test]
    fn every_hit_cell_is_at_least_the_hit_size() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(TagChipRow::new(three(), 10)));
        tree.layout(SizeProposal::exact(400.0, 40.0));
        for c in cells(&tree, id) {
            let b = tree.bounds(c);
            assert!(
                b.width >= HIT - 0.51 && b.height >= HIT - 0.51,
                "hit cell {}x{} is under {HIT} dp",
                b.width,
                b.height
            );
        }
    }

    /// The dot must stay round. `MinSize` alone would let it stretch to the row height,
    /// which is exactly how the settings pane's swatch once became a bar.
    #[test]
    fn a_dot_stays_square_in_a_tall_row() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(TagChipRow::new(
            vec![tag(1, "a", "#2e7d32", false)],
            10,
        )));
        tree.layout(SizeProposal::exact(400.0, 200.0));
        let cell = cells(&tree, id)[0];
        // cell -> Center -> FixedDot
        let centre = tree.children(cell)[0];
        let fixed = tree.children(centre)[0];
        let b = tree.bounds(fixed);
        assert!(
            (b.width - b.height).abs() < 0.51,
            "dot is {}x{}, not square",
            b.width,
            b.height
        );
        assert!(
            (b.width - DOT).abs() < 0.51,
            "dot should be {DOT} dp, got {}",
            b.width
        );
    }

    #[test]
    fn a_discoverable_tag_paints_a_ring_around_a_same_sized_fill() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(TagChipRow::new(
            vec![
                tag(1, "plain", "#2e7d32", false),
                tag(2, "bible", "#2e7d32", true),
            ],
            10,
        )));
        tree.layout(SizeProposal::exact(400.0, 40.0));
        let cs = cells(&tree, id);
        let painted = |cell: WidgetId| {
            let centre = tree.children(cell)[0];
            let fixed = tree.children(centre)[0];
            tree.bounds(fixed).width
        };
        let plain = painted(cs[0]);
        let bible = painted(cs[1]);
        assert!(
            bible > plain,
            "a discoverable dot ({bible}) should be wider than a plain one ({plain})"
        );
        assert!(
            (bible - (DOT + 2.0 * (RING_GAP + RING_WIDTH))).abs() < 0.51,
            "discoverable dot should be fill + ring, got {bible}"
        );
    }

    #[test]
    fn a_row_within_the_cap_has_no_overflow_cell() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(TagChipRow::new(three(), 5)));
        tree.layout(SizeProposal::exact(400.0, 40.0));
        assert_eq!(cells(&tree, id).len(), 3);
    }

    /// Over the cap the row must not simply grow: a corkboard card would be pushed apart by
    /// an item carrying a dozen tags.
    #[test]
    fn a_row_over_the_cap_collapses_the_rest_into_one_cell() {
        let tags: Vec<TagRow> = (1..=7)
            .map(|i| tag(i, &format!("t{i}"), "#2e7d32", false))
            .collect();
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(TagChipRow::new(tags, 4)));
        tree.layout(SizeProposal::exact(400.0, 40.0));
        let cs = cells(&tree, id);
        assert_eq!(cs.len(), 5, "4 dots + one overflow cell");
        let b = tree.bounds(cs[4]);
        assert!(
            b.width >= HIT - 0.51,
            "the overflow cell is a hit target too, got {}",
            b.width
        );
    }

    /// The "+N" cell names **exactly** the tags it is hiding — no more, no fewer, in order.
    ///
    /// This is what stops the count being a dead end, and it is the one part of the overflow
    /// that a live probe cannot easily reach (it needs a real tooltip on a real hover, on an
    /// item carrying more tags than any surface's cap). An off-by-one here yields a "+2" that
    /// lists three names, or lists the wrong two, and the cell that exists to explain itself
    /// quietly misleads instead.
    #[test]
    fn the_overflow_cell_names_exactly_the_hidden_tags() {
        let tags: Vec<TagRow> = (1..=6)
            .map(|i| tag(i, &format!("t{i}"), "#2e7d32", false))
            .collect();

        // The corkboard's cap: 4 shown, 2 hidden.
        let (shown, hidden) = TagChipRow::new(tags.clone(), MAX_VISIBLE_CORKBOARD).split_at_cap();
        assert_eq!(shown, 4);
        assert_eq!(
            hidden,
            vec!["t5".to_string(), "t6".to_string()],
            "the hidden list is the tail, in palette order — not the head, and not resorted"
        );

        // The stream's cap on the same tags: one fewer hidden. Proves the split follows the
        // cap it is given rather than a constant.
        let (shown, hidden) = TagChipRow::new(tags.clone(), MAX_VISIBLE_STREAM).split_at_cap();
        assert_eq!(shown, 5);
        assert_eq!(hidden, vec!["t6".to_string()]);

        // Under the editor's cap nothing is hidden, so no cell and nothing to name.
        let (shown, hidden) = TagChipRow::new(tags, MAX_VISIBLE_EDITOR).split_at_cap();
        assert_eq!(shown, 6);
        assert!(
            hidden.is_empty(),
            "with room for every tag there is no overflow cell to explain"
        );
    }

    /// The dots carry no text, so if the row does not name them the tags are invisible to a
    /// screen reader entirely.
    #[test]
    fn the_row_announces_every_tag_name() {
        let row = TagChipRow::new(three(), 10);
        let label = row.row_label();
        for name in ["status/draft", "character", "needs research"] {
            assert!(label.contains(name), "{label:?} should mention {name}");
        }
    }

    /// The row names the tags but must NOT also claim to be a button.
    ///
    /// In the app it is the trigger of a `Popover`, whose `OverlayTrigger` already emits
    /// `Role::Button` with `has_popup` and `expanded` at the very same bounds. Claiming the
    /// role here too announced the control twice — caught by reading the live AT tree, not
    /// by any test, which is why this one exists.
    #[test]
    fn the_row_is_not_a_second_button() {
        use bastyde::core::accessibility::widget_id_to_node_id;

        let mut tree = WidgetTree::new().with_theme(bastyde::presets::intui::light());
        let id = tree.add_boxed(Box::new(TagChipRow::new(three(), 10)));
        tree.layout(SizeProposal::exact(400.0, 40.0));
        let _ = tree.render();
        let update = tree.sync_accessibility();

        let (_, node) = update
            .nodes
            .iter()
            .find(|(nid, _)| *nid == widget_id_to_node_id(id))
            .expect("the row emits an AT node");
        assert_eq!(
            node.role(),
            Role::Label,
            "the surrounding OverlayTrigger owns the button role"
        );
        // A `Label`-role node carries its text in `value`, not `label` — the same shape
        // the running app reports for every GroupHeader.
        let name = node
            .value()
            .map(str::to_string)
            .or_else(|| node.label().map(str::to_string))
            .unwrap_or_default();
        for tag in ["status/draft", "character", "needs research"] {
            assert!(name.contains(tag), "{name:?} should name {tag}");
        }
        assert!(
            !update.nodes.iter().any(|(_, n)| n.role() == Role::Button),
            "no button node: the popover trigger supplies it, and two would be announced twice"
        );
    }

    /// The dots come out in palette order whatever order the ids were assigned in — resolved
    /// through the id → row `HashMap`, which has no order of its own, so the ordering has to
    /// be restored explicitly. Without this test the regression shows up only as dots that
    /// shuffle between two rows carrying the same tags.
    #[test]
    fn dots_follow_palette_order_not_assignment_order() {
        let mut rows = vec![
            tag(3, "status/draft", "#00f", false),
            tag(1, "character", "#f00", true),
            tag(2, "place", "#0f0", true),
        ];
        crate::models::sort_rows(&mut rows);
        let names: Vec<&str> = rows.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["character", "place", "status/draft"],
            "the shared comparator sorts case-insensitively by name"
        );

        // And the row announces them in that order — the row label is the one place the
        // names exist as text, so it is where the order is observable.
        assert_eq!(
            TagChipRow::new(rows, 8).row_label(),
            "character, place, status/draft",
        );
    }

    #[test]
    fn a_zero_cap_still_shows_one_dot() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(TagChipRow::new(three(), 0)));
        tree.layout(SizeProposal::exact(400.0, 40.0));
        let cs = cells(&tree, id);
        assert_eq!(cs.len(), 2, "one dot plus the overflow cell");
    }
}
