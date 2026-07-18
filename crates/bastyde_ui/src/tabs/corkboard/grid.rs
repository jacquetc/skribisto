// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The card grid itself, and its empty state.

#[allow(unused_imports)]
use super::*;

// ── Grid ──────────────────────────────────────────────────────────────────────

/// The reactive grid wrapper: binds the search-active + card-presentation signals
/// at `Rebuild`, then builds a `GridView` from either the raw model (natural
/// order → reorder + drag-out) or the filter projection (search active → reorder
/// inert). Card size drives `GridView`'s reactive `.sizing` with no rebuild.
pub(super) struct CorkboardGrid {
    pub(super) vm: CorkboardViewModel,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for CorkboardGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CorkboardGrid").finish()
    }
}
impl Widget for CorkboardGrid {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let reg = ctx.binding_registry();
        let sid = ctx.self_id();
        // Swap the bound source (raw ↔ projection) when search toggles; rebuild the
        // tiles when the card-presentation settings change (they feed the delegate).
        self.vm
            .is_projecting()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm
            .show_word_count()
            .bind_to(sid, reg, BindingLevel::Rebuild);

        let projecting = self.vm.is_projecting().get();

        // Reactive tile sizing from the card-size slider.
        let sizing = Signal::new(sizing_for(self.vm.card_size().get()));
        {
            let sizing = sizing.clone();
            ctx.effect(&self.vm.card_size(), move |w| sizing.set(sizing_for(*w)));
        }

        // Index → card, resolved against whichever source is bound (positions differ
        // once the projection filters/sorts).
        let raw = self.vm.cards_model();
        let proj = self.vm.projection();
        let read_card: Rc<dyn Fn(usize) -> Option<CorkboardCard>> = if projecting {
            let s = proj.clone();
            Rc::new(move |i| s.with_item(i, |c| c.clone()))
        } else {
            let s = raw.clone();
            Rc::new(move |i| s.with_item(i, |c| c.clone()))
        };

        let delegate = {
            let vm = self.vm.clone();
            let app_ctx = self.vm.app_ctx();
            let method = self.vm.counting_method();
            let show_wc = self.vm.show_word_count();
            let selection = self.vm.selection();
            move |tc: &TileContext<'_, CorkboardCard>| -> Box<dyn Widget> {
                Box::new(CorkboardTile {
                    vm: vm.clone(),
                    card: tc.item.clone(),
                    index: tc.index,
                    selection: selection.clone(),
                    app_ctx: app_ctx.clone(),
                    method: method.clone(),
                    show_wc: show_wc.clone(),
                    root: None,
                })
            }
        };

        let grid = if projecting {
            GridView::from_source(proj, delegate)
        } else {
            GridView::from_source(raw, delegate)
        };

        let empty_vm = self.vm.clone();
        let recv_vm = self.vm.clone();
        let act = read_card.clone();
        let act_vm = self.vm.clone();
        let type_ahead = read_card.clone();
        let f2_vm = self.vm.clone();

        let grid = grid
            .sizing(sizing)
            .spacing(14.0)
            .content_inset(EdgeInsets::uniform(16.0))
            .selection(self.vm.selection())
            .reorderable(!projecting)
            .exportable(DragTransferMode::Move)
            .accept_foreign_rows(true)
            .on_rows_received(move |items, _idx, _ctx| recv_vm.receive_cards(&items))
            .on_tile_activate(move |i, ctx| {
                if let Some(c) = act(i) {
                    act_vm.activate(ctx, &c);
                }
            })
            .type_ahead_label(move |i| type_ahead(i).map(|c| c.title).unwrap_or_default())
            .tile_a11y_label(move |i| read_card(i).map(|c| card_a11y_name(&c)).unwrap_or_default())
            .a11y_label(tr!(corkboard_grid_label()))
            .empty_view(move || Box::new(corkboard_empty(&empty_vm)));

        // F2 renames the selected card in place (the grid holds focus while the
        // writer is on the board; GridView doesn't use F2 itself). Attached last —
        // it's a `WidgetBuilder` hook, so no GridView-specific call follows it.
        let grid = grid.on_key(move |ev, _ctx| {
            if let WidgetEvent::KeyDown { key: Key::F2, .. } = ev {
                f2_vm.rename_selected();
                return EventResponse::Handled;
            }
            EventResponse::Ignored
        });

        let id = ctx.add(grid);
        // Publish the grid id so the inline title editor can return focus here on
        // commit/cancel (a11y: land back on the card, not the window root).
        self.vm.set_grid_id(id);
        self.root = Some(id);
        vec![id]
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

/// The empty-state affordance — a muted prompt plus the same "＋ New" split button.
pub(super) fn corkboard_empty(vm: &CorkboardViewModel) -> impl Widget {
    Center::new().child(
        VStack::new()
            .spacing(10.0)
            .child(
                TextWidget::new(tr!(corkboard_empty_title()))
                    .style(TextStyleRole::BodyBold)
                    .color(TextRole::Secondary),
            )
            .child(TextWidget::new(tr!(corkboard_empty_hint())).color(TextRole::Secondary))
            .child(CorkboardCreateButton {
                vm: vm.clone(),
                root: None,
            }),
    )
}
