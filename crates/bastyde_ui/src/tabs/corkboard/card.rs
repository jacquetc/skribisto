// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One card: its tile, and the inline title editor.

#[allow(unused_imports)]
use super::*;

// ── Tile ──────────────────────────────────────────────────────────────────────

/// One card. Owns a [`SingleCorkboardCard`] for the lazy excerpt + word count,
/// wired here so an edit elsewhere refreshes it. Built per **realized** tile.
pub(super) struct CorkboardTile {
    pub(super) vm: CorkboardViewModel,
    pub(super) card: CorkboardCard,
    /// This tile's position in the bound source — the key selection is indexed
    /// by (GridView computes `is_selected` from the same index).
    pub(super) index: usize,
    pub(super) selection: bastyde::data::SelectionModel,
    pub(super) app_ctx: Rc<AppContext>,
    pub(super) method: Signal<skribisto_model::counting::CountingMethodSetting>,
    pub(super) show_wc: Signal<bool>,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for CorkboardTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CorkboardTile").finish()
    }
}
impl Widget for CorkboardTile {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let single = SingleCorkboardCard::new(self.app_ctx.clone(), self.method.clone());
        single.set_card(self.card.item_id);
        single.wire(ctx);

        // Header: type icon, the (inline-editable) title, type badge, and the
        // "More actions" menu. Double-click the title — or F2 / ⋮ Rename — to edit
        // it in place.
        let header = HStack::new()
            .spacing(7.0)
            .child(crate::binder::icons::sub_role_icon(&self.card.sub_role).icon_size(15.0))
            .child(Expand::horizontal().child(InlineTitle {
                vm: self.vm.clone(),
                item_id: self.card.item_id,
                title: self.card.title.clone(),
                root: None,
            }))
            .child(
                Badge::new(sub_role_badge_label(&self.card.sub_role))
                    .text_role(TextRole::Secondary),
            )
            .child(card_menu(&self.vm, &self.card));

        // The synopsis: a *live* editor over the item's **shared** `OpenDoc` (the
        // same document an editor tab of this item uses — one source of truth), plus
        // an expand button that opens a roomier editor in a modal. Opened once per
        // container visit and reused across tile rebuilds; the VM flushes + releases
        // it when the board is left.
        //
        // The synopsis is the middle slot of a `CardColumn` (see it) — the header +
        // footer take their intrinsic height and the synopsis fills the exact rest,
        // scrolling its overflow. `CardColumn` measures the header, so the title
        // swapping in its taller edit field just shrinks the synopsis instead of
        // overflowing the card.
        let synopsis = CardSynopsis {
            vm: self.vm.clone(),
            card: self.card.clone(),
            open_doc: self.vm.synopsis_doc_for(self.card.item_id),
            root: None,
        };

        // Footer: an "expand synopsis" button at the bottom-left, then the count
        // pushed to the bottom-right (pinned there by the filling synopsis above).
        // The label is no longer here — it reads directly under the title now.
        let expand = {
            let vm = self.vm.clone();
            let card = self.card.clone();
            IconButton::expand()
                .embedded()
                .tooltip(tr!(corkboard_expand_synopsis()))
                .on_activate_fn(move |ctx| present_synopsis_modal(&vm, &card, ctx))
        };
        let footer = HStack::new()
            .spacing(8.0)
            .child(expand)
            .child(Spacer::new())
            .child(FooterCount {
                is_container: self.card.is_container,
                child_count: self.card.child_count,
                word_count: single.word_count(),
                show_wc: self.show_wc.clone(),
                root: None,
            });

        // The `top` slot: the header, then the free-text status label directly under
        // the title (when present). `CardColumn` measures this and the footer, so the
        // synopsis in the middle fills exactly the rest.
        let mut top = VStack::new().spacing(6.0).child(header);
        if !self.card.label.is_empty() {
            top = top.child(
                TextWidget::new(lit!(self.card.label.clone()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary)
                    .max_lines(1),
            );
        }
        // The card's inner content height (tile height minus its `Padding`), from the
        // size slider — the fixed-height GridView tile only proposes a card its width.
        let total = Signal::new(card_tile_height(self.vm.card_size().get()) - CARD_PADDING);
        {
            let t = total.clone();
            ctx.effect(&self.vm.card_size(), move |w| {
                t.set(card_tile_height(*w) - CARD_PADDING)
            });
        }
        let inner = CardColumn::new(total, 6.0, top, synopsis, footer);

        // Selection shows as the border accent *only* — no fill, no stripe.
        // Reactive (RepaintOnly, no tile rebuild): a click repaints the border
        // immediately, since GridView never rebuilds a tile on selection. The
        // keyboard focus ring is drawn separately by GridView's own overlay, so
        // both "selected" and "focused" read as an accented border and nothing
        // else. Keyed on this tile's index — exactly how GridView derives
        // `is_selected`.
        let index = self.index;
        let border_role = self.selection.selection_signal().map(move |sel| {
            if sel.contains(&index) {
                BorderRole::Accent
            } else {
                BorderRole::Default
            }
        });

        // Middle-click a leaf card → open it in the *other* editor pane (mirrors
        // the outline's middle-click "open to side"). Consumes only the middle
        // button so primary-click selection / drag / double-click-open are intact.
        let mid_vm = self.vm.clone();
        let mid_card = self.card.clone();

        // `Padding` insets the content; `CardColumn` inside sizes itself from the
        // slider (the tile forces the Panel to the card height, so no `Expand` needed).
        let card = Panel::new()
            .background(SurfaceRole::Content)
            .corner_radius(10.0)
            .border_color(border_role)
            .border_width(1.0)
            .child(Padding::uniform(12.0).child(inner))
            .on_pointer_event(move |ev, ctx| {
                if let WidgetEvent::PointerDown {
                    button: PointerButton::Middle,
                    ..
                } = ev
                {
                    mid_vm.open_to_side(ctx, &mid_card);
                    return EventResponse::Handled;
                }
                EventResponse::Ignored
            });

        // The concise per-cell accessible name is set on the GridCell wrapper via
        // `.tile_a11y_label` (see `card_a11y_name`); the card body stays unlabelled
        // so a screen reader reads the tidy name, then the synopsis on demand.
        let id = ctx.add(card);
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

/// Return focus to the grid after an inline rename commits/cancels, so a screen
/// reader and keyboard navigation land back on the card — not the window root,
/// where focus would otherwise fall when the editor field is removed.
pub(super) fn refocus_grid(vm: &CorkboardViewModel, ctx: &mut EventContext) {
    if let Some(gid) = vm.grid_id().get() {
        ctx.request_focus(gid);
    }
}

/// The card's title, editable in place. Normally a one-line label; a single click on
/// it (or F2 / the ⋮ menu on the selection) swaps in a focused text field. Enter or
/// clicking away commits; Esc restores the old name; a blank name is rejected (the
/// old title stands). Only the card whose id matches the view-model's `editing_item`
/// is in edit mode, so exactly one edits at a time.
pub(super) struct InlineTitle {
    pub(super) vm: CorkboardViewModel,
    pub(super) item_id: u64,
    pub(super) title: String,
    pub(super) root: Option<WidgetId>,
}
impl std::fmt::Debug for InlineTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InlineTitle").finish()
    }
}
impl Widget for InlineTitle {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.editing_item().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let editing = self.vm.editing_item().get() == Some(self.item_id);
        let id = if editing {
            // A live buffer seeded with the current title. Enter / blur commit it;
            // Esc restores the original so the following blur-commit is a no-op.
            let buffer = Signal::new(self.title.clone());
            let original = self.title.clone();
            let field = TextInput::new(buffer.clone())
                // Accessible name: a screen reader announces "Rename item, edit
                // text, <current title>" instead of a bare "edit text". The
                // visible title is unchanged (this is a11y-only).
                .label(tr!(corkboard_rename_field()))
                .on_submit_fn({
                    let vm = self.vm.clone();
                    let buffer = buffer.clone();
                    let item_id = self.item_id;
                    move |ctx| vm.rename(ctx, item_id, &buffer.get())
                })
                .on_blur_fn({
                    let vm = self.vm.clone();
                    let buffer = buffer.clone();
                    let item_id = self.item_id;
                    move |ctx| vm.rename(ctx, item_id, &buffer.get())
                })
                .on_key({
                    let vm = self.vm.clone();
                    let buffer = buffer.clone();
                    let item_id = self.item_id;
                    move |ev, ctx| {
                        if let WidgetEvent::KeyDown { key, .. } = ev {
                            match key {
                                // Commit here (not just via `on_submit`) and mark
                                // the key Handled — otherwise Enter bubbles up to
                                // the GridView, which activates the focused tile and
                                // opens the item. Committing + consuming keeps Enter
                                // a pure "confirm the rename".
                                Key::Enter => {
                                    vm.rename(ctx, item_id, &buffer.get());
                                    refocus_grid(&vm, ctx);
                                    return EventResponse::Handled;
                                }
                                Key::Escape => {
                                    buffer.set(original.clone());
                                    vm.cancel_rename();
                                    refocus_grid(&vm, ctx);
                                    return EventResponse::Handled;
                                }
                                _ => {}
                            }
                        }
                        EventResponse::Ignored
                    }
                });
            let fid = ctx.add(field);
            // Focus the field so the writer can type immediately.
            ctx.focus(fid);
            fid
        } else {
            let vm = self.vm.clone();
            let item_id = self.item_id;
            ctx.add(
                TextWidget::new(lit!(self.title.clone()))
                    .style(TextStyleRole::SmallBold)
                    .max_lines(1)
                    // A text cursor advertises the click-to-edit affordance on hover.
                    .cursor(CursorIcon::Text)
                    // A single primary click enters rename — and is **consumed**, so
                    // it never reaches the GridView. Otherwise the click would select
                    // the tile and a second one would activate it, opening the card in
                    // a tab (the double-click-opens-while-editing bug). F2 and the ⋮
                    // menu's Rename are the other ways in.
                    .on_pointer_event(move |ev, _ctx| {
                        if let WidgetEvent::PointerDown {
                            button: PointerButton::Primary,
                            ..
                        } = ev
                        {
                            vm.begin_rename(item_id);
                            return EventResponse::Handled;
                        }
                        EventResponse::Ignored
                    }),
            )
        };
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
