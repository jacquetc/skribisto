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
    typo: EditorTypography,
) -> (impl Widget, teksilo::widgets::rich_text::EditorHandle) {
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
        typo,
        on_change,
        split,
        spell,
        open_doc.replacement_synopsis(),
        Some(vm.format()),
        Some(vm.caret_band()),
        open_doc.images(),
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
                // A **gesture dead zone** around the editor.
                //
                // Selecting a word is press-move-release — the same gesture the
                // `GridView` beneath uses to start a card drag. The editor does its
                // selection through `on_pointer_event` rather than a drag
                // recognizer and returns `Ignored` on `PointerDown` (deliberately,
                // so its double/triple-tap recognizers keep working), which is
                // exactly what `arm_drag_observers` walks straight past on its way
                // to arming the tile above. The writer then dragged the card while
                // trying to select a word.
                //
                // `DeadZone` stops that walk **structurally** — no ancestor above
                // this node is ever armed — rather than by winning a gesture race,
                // and it is layout-transparent, so `place_children` below still
                // sizes the editor exactly as before. The card stays draggable by
                // its header, its status line and its padding.
                {
                    let (editor, _handle) =
                        synopsis_editor(&self.vm, &self.card, doc, self.vm.synopsis_typo());
                    ctx.add(DeadZone::new().child(editor))
                }
            }
            // No synopsis field (or unreadable): an empty filler so the card still
            // lays out.
            _ => ctx.add(teksilo::widgets::Spacer::new()),
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
        // Escape **or** an outside click, the ordinary modal contract — clicking
        // away from a thing you opened should put it away, and needing to find
        // Escape for a surface opened by mouse is a small papercut every time.
        //
        // (The previous note here worried that a right-click in the editor — to
        // reach Split/Cut/Paste — would read as an outside click and dismiss the
        // modal out from under its own menu. The context menu opens as an overlay
        // *above* the modal, and the press that opens it lands inside the modal's
        // surface, so it is not an outside click; the menu's own dismissal is a
        // separate overlay pop. Worth re-checking by hand if that menu ever starts
        // closing its host.)
        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
        // Roomy enough to actually draft in: the old 720×560 left the editor a
        // 500px box, which is a handful of lines once the chrome and padding are
        // out — the writer opened "expand" and got barely more than the card.
        .size(980, 760),
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
                // The expanded editor gets its **own** size scale (Settings ▸ Corkboard):
                // the card's is chosen to be scannable in a tile, this one to be written in.
                let (editor, handle) =
                    synopsis_editor(&self.vm, &self.card, &doc, self.vm.modal_typo());
                // `FixedSize` → `Padding` forward an *exact* bounded height to the
                // greedy editor, which is what makes it consume that height and
                // scroll. An `Expand` in between would measure the editor with an
                // unspecified height (its 100px fallback), so it never learns the
                // box height — the editor then overflows, centered, scrollbar pinned.
                // A titled, raised frame. Without it the modal is a bare white slab
                // over the board: `ModalRequest::title` names the *native-window*
                // presentation and this one is `InTree`, so the surface owns its own
                // chrome, exactly as `MoveTargetPanel` does.
                let root = ctx.add(
                    Panel::new()
                        .variant(PanelVariant::Raised)
                        .corner_radius(10.0)
                        .padding(0.0)
                        .child(
                            VStack::new()
                                .spacing(0.0)
                                .child(
                                    Padding::symmetric(8.0, 14.0).child(
                                        TextWidget::new(tr!(corkboard_synopsis_modal_title()))
                                            .style(TextStyleRole::Small)
                                            .color(TextRole::Secondary),
                                    ),
                                )
                                .child(Divider::new())
                                .child(
                                    FixedSize::new()
                                        .width(940.0)
                                        .height(660.0)
                                        .child(Padding::uniform(16.0).child(editor)),
                                ),
                        ),
                );
                // Put the caret in the editor as the modal opens. A writer who chose
                // "expand synopsis" wants to type, and an unfocused editor draws no
                // caret at all — the surface reads as broken rather than unfocused.
                //
                // `run_after_mount`, not a focus call here: at `build` time this
                // subtree's children have not built yet, so the editor's focusable
                // node does not exist and any descendant walk comes back empty (which
                // is exactly what the framework's own `first_focusable_descendant`
                // fallback hits, and why the modal opened caretless). The same idiom
                // `distraction_free::surface` uses to hand its editor the caret.
                ctx.run_after_mount(move |ctx| handle.focus(ctx));
                root
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
