// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A card's synopsis surface — the inline editor and its expand modal.

#[allow(unused_imports)]
use super::*;

/// A caret-aware "Split scene" for the card's synopsis editor — only offered on a
/// prose-bearing scene, and routed to the same `split_scene` use case the Full view
/// uses (so the card and the stream split identically).
pub(super) fn synopsis_split_fn(
    vm: &CorkboardViewModel,
    card: &CorkboardCard,
) -> Option<crate::tabs::shared::editor::SplitFn> {
    if !vm.can_split(card) {
        return None;
    }
    let vm = vm.clone();
    let id = card.item_id;
    Some(Rc::new(move |ctx: &mut EventContext, caret: usize| {
        vm.split_synopsis(ctx, id, caret)
    }))
}

/// Build the card's editable synopsis over its shared `OpenDoc`. It uses the
/// borderless, greedy, internally-scrolling `card_synopsis_editor` — like the scene
/// main editor, but bounded so a long synopsis scrolls (and follows the caret)
/// instead of overflowing the card/modal — with the same typography, spell-check and
/// right-click menu (incl. **Split scene**). Edits write straight through to the
/// shared doc: `on_change` marks it dirty (autosave + the save indicator see it).
/// Wrap the result in an `Expand` (or a `FixedSize`) so the target box bounds it.
pub(super) fn synopsis_editor(
    vm: &CorkboardViewModel,
    card: &CorkboardCard,
    open_doc: &Rc<OpenDoc>,
) -> impl Widget {
    // Every writing row has a `SynopsisText` field; the `None` fallback is only the
    // defensive path (a card whose type carries no synopsis) — the callers already
    // gate on `synopsis.is_some()`.
    let doc = open_doc
        .synopsis
        .as_ref()
        .map(|f| f.doc.clone())
        .unwrap_or_default();
    let on_change = open_doc.mark_dirty_fn();
    let split = synopsis_split_fn(vm, card);
    let spell = open_doc.spell_synopsis();
    crate::tabs::shared::editor::card_synopsis_editor(
        doc,
        vm.synopsis_typo(),
        on_change,
        split,
        spell,
        open_doc.replacement_synopsis(),
        Some(vm.format()),
        Some(vm.caret_band()),
    )
}

/// A card's vertical layout: `top` (header + optional status label) and `bottom`
/// (footer) take their intrinsic height; `middle` (the synopsis editor) fills the
/// **exact** remaining height and is *proposed* that height — so the greedy editor
/// consumes it, fills the card width, and scrolls its overflow.
///
/// Why not `VStack` + `Expand`? Two reasons this layout has to be bespoke:
/// - `Expand` proposes an *unspecified* height to its flex child, so a greedy editor
///   collapses to its ~100px fallback (overflow, centred, scrollbar pinned).
/// - The card's height must come from the size slider (`total`), because the
///   fixed-height GridView tile only ever proposes a *width* to a card.
///
/// Measuring the header (rather than assuming a fixed chrome height) is what keeps it
/// correct when the title swaps its one-line label for its taller edit field: the
/// synopsis simply shrinks to fit, instead of the footer overflowing the card.
pub(super) struct CardColumn {
    /// The card's inner content height (tile height minus its frame), from the slider.
    pub(super) total: Signal<f32>,
    pub(super) spacing: f32,
    pub(super) top: Option<Box<dyn Widget>>,
    pub(super) middle: Option<Box<dyn Widget>>,
    pub(super) bottom: Option<Box<dyn Widget>>,
    pub(super) ids: Vec<WidgetId>,
}
impl CardColumn {
    pub(super) fn new(
        total: Signal<f32>,
        spacing: f32,
        top: impl Widget + 'static,
        middle: impl Widget + 'static,
        bottom: impl Widget + 'static,
    ) -> Self {
        Self {
            total,
            spacing,
            top: Some(Box::new(top)),
            middle: Some(Box::new(middle)),
            bottom: Some(Box::new(bottom)),
            ids: Vec::new(),
        }
    }
    /// Intrinsic height of `top`/`bottom` at width `w`; the middle gets the exact
    /// remainder of `avail` (min 24px). Used with `total` at measure time and with the
    /// real placed height at place time, so the column fills whatever height it lands in.
    fn slot_heights(&self, w: Option<f32>, avail: f32, ctx: &LayoutContext) -> (f32, f32, f32) {
        let intrinsic = |id: WidgetId| {
            ctx.child_size(
                id,
                SizeProposal {
                    width: w,
                    height: None,
                },
            )
            .map(|s| s.height)
            .unwrap_or(0.0)
        };
        let top_h = self.ids.first().copied().map(intrinsic).unwrap_or(0.0);
        let bot_h = self.ids.get(2).copied().map(intrinsic).unwrap_or(0.0);
        let mid_h = (avail - top_h - bot_h - 2.0 * self.spacing).max(24.0);
        (top_h, mid_h, bot_h)
    }
}
impl std::fmt::Debug for CardColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardColumn").finish()
    }
}
impl Widget for CardColumn {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.ids = vec![
            ctx.add_boxed(self.top.take().expect("built once")),
            ctx.add_boxed(self.middle.take().expect("built once")),
            ctx.add_boxed(self.bottom.take().expect("built once")),
        ];
        self.total.bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Relayout,
        );
        self.ids.clone()
    }
    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // The tile proposes only a width, so take the height from `total` (the slider).
        let avail = self.total.get().max(0.0);
        let (_, mid_h, _) = self.slot_heights(proposal.width, avail, ctx);
        // Hand the middle its exact height (and forward the card width) so the greedy
        // editor bounds itself and scrolls.
        if let Some(&mid) = self.ids.get(1) {
            let _ = ctx.child_size(
                mid,
                SizeProposal {
                    width: proposal.width,
                    height: Some(mid_h),
                },
            );
        }
        Size::new(proposal.width.unwrap_or(0.0), avail).into()
    }
    fn place_children(
        &self,
        bounds: Rect,
        _p: SizeProposal,
        children: &mut [WidgetPlacement],
        ctx: &LayoutContext,
    ) {
        // Fill the *actual* placed height, so a frame/rounding difference never leaves
        // the footer overflowing or floating.
        let (top_h, mid_h, bot_h) = self.slot_heights(Some(bounds.width), bounds.height, ctx);
        let heights = [top_h, mid_h, bot_h];
        let mut y = bounds.y;
        for (i, child) in children.iter_mut().enumerate() {
            let h = heights.get(i).copied().unwrap_or(0.0);
            child.origin = Point::new(bounds.x, y);
            child.size = Size::new(bounds.width, h);
            y += h + self.spacing;
        }
    }
    fn children(&self) -> Vec<WidgetId> {
        self.ids.clone()
    }
}

/// The card's synopsis body: a **live editor** over the item's shared `OpenDoc`
/// (no read-only ⇄ editable swap — the editor *is* the card, so there is no font /
/// size / scroll jump on click), plus an expand button that opens the same editor in
/// a roomier modal. A quiet placeholder if the item couldn't be opened.
pub(super) struct CardSynopsis {
    pub(super) vm: CorkboardViewModel,
    pub(super) card: CorkboardCard,
    /// The shared open document for this card's synopsis (from the VM's per-card
    /// store). `None` only if the item couldn't be opened.
    pub(super) open_doc: Option<Rc<OpenDoc>>,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for CardSynopsis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardSynopsis").finish()
    }
}
impl Widget for CardSynopsis {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Always the editor over the shared doc — one widget, one document, so
        // clicking to type never swaps typography or resets the scroll position.
        // The caller wraps this in a `FixedSize`, which hands the greedy editor an
        // *exact* height to consume (so it scrolls + follows the caret); this widget
        // just force-fills that box (see `place_children`).
        let body = match &self.open_doc {
            Some(doc) if doc.synopsis.is_some() => {
                ctx.add(synopsis_editor(&self.vm, &self.card, doc))
            }
            // No synopsis field (or unreadable): an empty filler so the card still
            // lays out.
            _ => ctx.add(bastyde::widgets::Spacer::new()),
        };
        self.root = Some(body);
        vec![body]
    }
    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or_else(|| p.resolve(0.0, 0.0))
            .into()
    }
    fn place_children(
        &self,
        bounds: Rect,
        _p: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        // Force the editor to fill our (FixedSize-bounded) box — a no-op default
        // place would leave the greedy editor at its measured fallback height, the
        // very bug that made the text overflow, centered, scrollbar pinned.
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }
    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// Open the synopsis in a roomier modal editor over the **same** shared document as
/// the inline card editor — the card holds it open, so this is the very same
/// `OpenDoc` and edits reflect in both instantly.
pub(super) fn present_synopsis_modal(
    vm: &CorkboardViewModel,
    card: &CorkboardCard,
    ctx: &mut EventContext,
) {
    let vm = vm.clone();
    let card = card.clone();
    ctx.present_modal(
        ModalRequest::deferred(move |t| {
            t.add(SynopsisModal {
                vm: vm.clone(),
                card: card.clone(),
                root: None,
            })
        })
        .presentation(ModalPresentation::InTree)
        .title(tr!(corkboard_synopsis_modal_title()).resolve_now())
        // Escape / the title-bar close only — NOT click-outside: a right-click in
        // the editor (to reach the Split/Cut/Paste menu) would otherwise be read as
        // an outside click and dismiss the modal out from under the menu.
        .close_behavior(ModalCloseBehavior::EscapeKey)
        .size(720, 560),
    );
}

/// The modal's content: the shared synopsis editor in a wide column.
pub(super) struct SynopsisModal {
    pub(super) vm: CorkboardViewModel,
    pub(super) card: CorkboardCard,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for SynopsisModal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SynopsisModal").finish()
    }
}
impl Widget for SynopsisModal {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let id = self.card.item_id;
        // The editor needs a *bounded* box: the in-tree modal sizes to its content,
        // so declare the surface size explicitly (a plain `Expand` would leave the
        // greedy editor no height to consume). `FixedSize` gives it 680×500; the
        // editor fills it and scrolls internally for a long synopsis.
        let cid = match self.vm.synopsis_doc_for(id) {
            Some(doc) if doc.synopsis.is_some() => {
                let editor = synopsis_editor(&self.vm, &self.card, &doc);
                // `FixedSize` → `Padding` forward an *exact* bounded height to the
                // greedy editor, which is what makes it consume that height and
                // scroll. An `Expand` in between would measure the editor with an
                // unspecified height (its 100px fallback), so it never learns the
                // box height — the editor then overflows, centered, scrollbar pinned.
                ctx.add(
                    FixedSize::new()
                        .width(680.0)
                        .height(500.0)
                        .child(Padding::uniform(16.0).child(editor)),
                )
            }
            _ => {
                ctx.add(Padding::uniform(16.0).child(TextWidget::new(tr!(corkboard_empty_hint()))))
            }
        };
        self.root = Some(cid);
        vec![cid]
    }
    fn layout_response(&self, p: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, p))
            .unwrap_or_else(|| p.resolve(0.0, 0.0))
            .into()
    }
    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}
