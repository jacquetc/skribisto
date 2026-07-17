// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Corkboard** pane — a container's contents as a grid of synopsis cards.
//!
//! One segment of a Book / Part / Chapter-folder tab. A header (breadcrumb ·
//! count · search · nested/flat · ＋New · card-size) sits over a virtualized
//! [`GridView`] of cards. The grid binds the raw backend-driven model in natural
//! order (drag-reorder + drag-out enabled) and swaps to the model's filter
//! projection while a search is active (reorder inert — you don't drag a filtered
//! view). Card size is a live slider bound to `GridView`'s reactive `.sizing`.
//!
//! Each card's cheap fields (title/type/label) come from the model; the synopsis
//! excerpt and word count are resolved lazily per **visible** tile by
//! [`SingleCorkboardCard`], so a long board stays as cheap as one viewport.

use std::rc::Rc;

use bastyde::canvas::EdgeInsets;
use bastyde::core::BindingLevel;
use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::data::ListDataSource;
use bastyde::i18n::LocalizedString;
use bastyde::prelude::*;
use bastyde::res;
use bastyde::widgets::rich_text::{RichTextEditor, ScrollPolicy};
use bastyde::widgets::{
    Badge, Breadcrumb, BreadcrumbItem, ButtonVariant, Center, DragTransferMode, Expand, FixedSize,
    GridSizing, GridView, HStack, IconButton, IconWidget, MenuItem, MenuList, Padding, Panel,
    PopoverIconButton, SearchField, Segment, SegmentedControl, Slider, Spacer, SplitButton,
    TextInput, TextWidget, TileContext, VStack,
};

// `WidgetEvent`, `Key`, `PointerButton`, `EventResponse` and the `WidgetBuilder`
// gesture/key hooks all come from `bastyde::prelude::*` above.

use frontend::AppContext;
use frontend::common::entities::BinderItemSubRole;

use crate::create_labels::{
    recommendation_label, recommendation_placement, recommendation_tooltip_key,
};
use crate::models::CorkboardCard;
use crate::singles::SingleCorkboardCard;
use crate::view_models::CorkboardViewModel;

/// The fixed "＋" glyph for the create button's main region.
fn add_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/add.svg")).icon_size(14.0)
}

/// Map the card-size slider (a minimum tile width) to an adaptive grid sizing —
/// as many ≥ `w`-wide columns as fit, stretched, with a card-shaped height.
fn sizing_for(w: f32) -> GridSizing {
    GridSizing::Adaptive {
        min_width: w,
        max_width: Some(w * 1.6),
        // A card is wider than tall (index-card proportion), enough for the header,
        // a few excerpt lines, and the footer.
        height: (w * 0.72).max(146.0),
    }
}

/// The pane: a wiring child, the header, and the grid filling the rest.
pub fn corkboard_pane(tab: &super::super::ContentTab) -> Box<dyn Widget> {
    let Some(vm) = tab.corkboard().cloned() else {
        // Non-container tabs never reach here (the segment only exists for them),
        // but keep the switcher total.
        return Box::new(VStack::new());
    };
    Box::new(
        VStack::new()
            .spacing(0.0)
            .child(WireCorkboard { vm: vm.clone() })
            .child(corkboard_header(&vm))
            .child(Expand::new().child(CorkboardGrid { vm, root: None })),
    )
}

/// Zero-size child that wires the view-model (subscribes model + probe + the
/// search/sort plumbing) on build. `wire` is idempotent per build.
struct WireCorkboard {
    vm: CorkboardViewModel,
}
impl std::fmt::Debug for WireCorkboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WireCorkboard").finish()
    }
}
impl Widget for WireCorkboard {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.wire(ctx);
        Vec::new()
    }
    fn layout_response(&self, _p: SizeProposal, _c: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }
}

// ── Header ────────────────────────────────────────────────────────────────────

fn corkboard_header(vm: &CorkboardViewModel) -> impl Widget {
    let top = HStack::new()
        .spacing(10.0)
        .child(Expand::horizontal().child(CorkboardBreadcrumb {
            vm: vm.clone(),
            root: None,
        }))
        .child(CorkboardCount {
            vm: vm.clone(),
            root: None,
        })
        .child(NestedFlatToggle {
            vm: vm.clone(),
            index: Signal::new(0),
            root: None,
        })
        .child(CorkboardCreateButton {
            vm: vm.clone(),
            root: None,
        });

    let bottom = HStack::new()
        .spacing(10.0)
        .child(
            Expand::horizontal().child(
                SearchField::new(vm.search_query_signal())
                    .placeholder(tr!(corkboard_search_placeholder())),
            ),
        )
        .child(TextWidget::new(tr!(corkboard_card_size())).color(TextRole::Secondary))
        .child(
            FixedSize::new().width(170.0).child(
                Slider::new(
                    vm.card_size(),
                    crate::CORKBOARD_CARD_SIZE_MIN,
                    crate::CORKBOARD_CARD_SIZE_MAX,
                )
                .step(crate::CORKBOARD_CARD_SIZE_STEP)
                .label(tr!(corkboard_card_size())),
            ),
        );

    Panel::new().background(SurfaceRole::Raised).child(
        Padding::symmetric(14.0, 8.0).child(VStack::new().spacing(8.0).child(top).child(bottom)),
    )
}

/// The breadcrumb trail — rebuilds when navigation (drill in/out) changes it.
struct CorkboardBreadcrumb {
    vm: CorkboardViewModel,
    root: Option<WidgetId>,
}
impl std::fmt::Debug for CorkboardBreadcrumb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CorkboardBreadcrumb").finish()
    }
}
impl Widget for CorkboardBreadcrumb {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.trail_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        // The current crumb's title tracks the container probe, so also rebuild
        // when it resolves/changes.
        self.vm.container_title().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let trail = self.vm.trail_signal().get();
        let current_title = self.vm.container_title().get();
        let last = trail.len().saturating_sub(1);
        let mut bc = Breadcrumb::new().label(tr!(corkboard_grid_label()));
        for (i, (_id, title)) in trail.iter().enumerate() {
            if i == last {
                // The current container — its title comes from the live probe.
                let t = if current_title.is_empty() {
                    title.clone()
                } else {
                    current_title.clone()
                };
                bc = bc.item(BreadcrumbItem::current(lit!(t)));
            } else {
                let vm = self.vm.clone();
                let title = title.clone();
                bc = bc.item(
                    BreadcrumbItem::new(lit!(title)).on_activate_fn(move |_ctx| vm.go_to_crumb(i)),
                );
            }
        }
        let id = ctx.add(bc);
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

/// The "N cards" count — rebuilds when the card set changes.
struct CorkboardCount {
    vm: CorkboardViewModel,
    root: Option<WidgetId>,
}
impl std::fmt::Debug for CorkboardCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CorkboardCount").finish()
    }
}
impl Widget for CorkboardCount {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.count_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let n = self.vm.count_signal().get();
        let id = ctx.add(
            TextWidget::new(tr!(corkboard_card_count(count = n as i64))).color(TextRole::Secondary),
        );
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

/// Nested ⇄ Flat mode toggle — a two-segment control bridged to the VM's `nested`
/// bool (0 = Nested, 1 = Flat) via two guarded effects, the shape `goals_pane` uses.
struct NestedFlatToggle {
    vm: CorkboardViewModel,
    index: Signal<usize>,
    root: Option<WidgetId>,
}
impl std::fmt::Debug for NestedFlatToggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NestedFlatToggle").finish()
    }
}
impl Widget for NestedFlatToggle {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let nested = self.vm.nested();
        self.index.set(if nested.get() { 0 } else { 1 });
        {
            let index = self.index.clone();
            ctx.effect(&nested, move |n| {
                let i = if *n { 0 } else { 1 };
                if index.get() != i {
                    index.set(i);
                }
            });
        }
        {
            let nested = nested.clone();
            ctx.effect(&self.index, move |i| {
                let n = *i == 0;
                if nested.get() != n {
                    nested.set(n);
                }
            });
        }
        let ctrl = SegmentedControl::new(self.index.clone())
            .segment(Segment::new(tr!(corkboard_view_nested())))
            .segment(Segment::new(tr!(corkboard_view_flat())));
        let id = ctx.add(ctrl);
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

/// The "＋ New" split button: offers the container's recommended child types,
/// anchored on the *current* (drilled-into) container. Mirrors the outline dock's
/// `CreateSplitButton`; rebuilds when the container changes.
struct CorkboardCreateButton {
    vm: CorkboardViewModel,
    root: Option<WidgetId>,
}
impl std::fmt::Debug for CorkboardCreateButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CorkboardCreateButton").finish()
    }
}
impl Widget for CorkboardCreateButton {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Recommendations depend on the container's (role, sub_role) — rebuild when
        // the container (hence its title) changes.
        self.vm.container_title().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let recs = self.vm.create_recommendations();
        let anchor_title = self.vm.container_title().get();

        let mut btn = SplitButton::new_static()
            .variant(ButtonVariant::Tinted)
            .icon(add_icon());
        for rec in &recs {
            let placement =
                recommendation_placement(Some(anchor_title.as_str()), rec.relation).resolve_now();
            let create_type = rec.create_type;
            let relation = rec.relation;
            let vm = self.vm.clone();
            btn = btn.item(
                MenuItem::new(recommendation_label(rec.create_type))
                    .icon(crate::binder_icons::create_type_icon(rec.create_type))
                    .shortcut_label(placement)
                    .rich_tooltip(recommendation_tooltip_key(rec.create_type))
                    .on_activate_fn(move |ctx| {
                        vm.fire_create(
                            ctx,
                            skribisto_model::Recommendation {
                                create_type,
                                relation,
                            },
                        )
                    }),
            );
        }
        if let Some(top) = recs.first() {
            btn = btn.rich_tooltip(recommendation_tooltip_key(top.create_type));
        }
        let id = ctx.add(btn);
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

// ── Grid ──────────────────────────────────────────────────────────────────────

/// The reactive grid wrapper: binds the search-active + card-presentation signals
/// at `Rebuild`, then builds a `GridView` from either the raw model (natural
/// order → reorder + drag-out) or the filter projection (search active → reorder
/// inert). Card size drives `GridView`'s reactive `.sizing` with no rebuild.
struct CorkboardGrid {
    vm: CorkboardViewModel,
    root: Option<WidgetId>,
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
fn corkboard_empty(vm: &CorkboardViewModel) -> impl Widget {
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

// ── Tile ──────────────────────────────────────────────────────────────────────

/// One card. Owns a [`SingleCorkboardCard`] for the lazy excerpt + word count,
/// wired here so an edit elsewhere refreshes it. Built per **realized** tile.
struct CorkboardTile {
    vm: CorkboardViewModel,
    card: CorkboardCard,
    /// This tile's position in the bound source — the key selection is indexed
    /// by (GridView computes `is_selected` from the same index).
    index: usize,
    selection: bastyde::data::SelectionModel,
    app_ctx: Rc<AppContext>,
    method: Signal<skribisto_model::counting::CountingMethodSetting>,
    show_wc: Signal<bool>,
    root: Option<WidgetId>,
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
            .child(crate::binder_icons::sub_role_icon(&self.card.sub_role).icon_size(15.0))
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

        // The synopsis: read-only by default, editable in place on click (over the
        // *shared* document, so a card and an editor tab never diverge), plus an
        // expand button that opens a larger editor in a modal. Fills the card's
        // middle and pins the footer below it.
        let body = Expand::new().child(CardSynopsis {
            vm: self.vm.clone(),
            card: self.card.clone(),
            read_doc: single.synopsis_doc(),
            root: None,
        });

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

        // header → the free-text label (its "status", directly under the title) →
        // the scrollable synopsis filling the middle (pins the footer to the
        // card's bottom) → the footer. So the word count is always bottom-right.
        let mut inner = VStack::new().spacing(6.0).child(header);
        if !self.card.label.is_empty() {
            inner = inner.child(
                TextWidget::new(lit!(self.card.label.clone()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary)
                    .max_lines(1),
            );
        }
        let inner = inner.child(body).child(footer);

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

        // A Panel proposes its child the (bounded) tile height, so the VStack's
        // Spacer fills — the whole card fills its tile slot rather than shrinking.
        let card = Panel::new()
            .background(SurfaceRole::Content)
            .corner_radius(10.0)
            .border_color(border_role)
            .border_width(1.0)
            .child(Expand::new().child(Padding::uniform(12.0).child(inner)))
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
fn refocus_grid(vm: &CorkboardViewModel, ctx: &mut EventContext) {
    if let Some(gid) = vm.grid_id().get() {
        ctx.request_focus(gid);
    }
}

/// The card's title, editable in place. Normally a one-line label; double-click it
/// (or F2 / the ⋮ menu on the selection) swaps in a focused text field. Enter or
/// clicking away commits; Esc restores the old name. Only the card whose id matches
/// the view-model's `editing_item` is in edit mode, so exactly one edits at a time.
struct InlineTitle {
    vm: CorkboardViewModel,
    item_id: u64,
    title: String,
    root: Option<WidgetId>,
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
                    .on_double_tap(move |_tap, _ctx| vm.begin_rename(item_id)),
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

/// A caret-aware "Split scene" for the card's synopsis editor — only offered on a
/// prose-bearing scene, and routed to the same `split_scene` use case the Full view
/// uses (so the card and the stream split identically).
fn synopsis_split_fn(
    vm: &CorkboardViewModel,
    card: &CorkboardCard,
) -> Option<super::editor::SplitFn> {
    if !vm.can_split(card) {
        return None;
    }
    let vm = vm.clone();
    let id = card.item_id;
    Some(Rc::new(move |ctx: &mut EventContext, caret: usize| {
        vm.split_synopsis(ctx, id, caret)
    }))
}

/// Build the shared editable synopsis column. It uses the **scene main editor**'s
/// column (`writing_column`), so the synopsis edits *borderless* — like the main
/// prose editor, not the Full-Synopsis view's bordered box — while still carrying
/// the same typography, spell-check, and the right-click menu with **Split scene**.
/// `width` caps the writing column (narrow on the card, wide in a modal).
fn synopsis_editor(
    vm: &CorkboardViewModel,
    card: &CorkboardCard,
    doc: &bastyde::text_document::TextDocument,
    width: Signal<f32>,
) -> super::editor::CenterColumnFlowing {
    let on_change = vm.synopsis_on_change(card.item_id);
    let split = synopsis_split_fn(vm, card);
    let spell = vm
        .synopsis_open_doc(card.item_id)
        .and_then(|d| d.spell_synopsis());
    super::editor::writing_column(
        doc,
        &width,
        &vm.synopsis_typo(),
        1,
        on_change,
        split,
        None,
        spell,
    )
}

/// The card's synopsis body: a read-only viewer that becomes an inline editor when
/// this card is the one being edited (`editing_synopsis`), plus an expand button
/// that opens the same editor in a roomier modal.
struct CardSynopsis {
    vm: CorkboardViewModel,
    card: CorkboardCard,
    /// The read-only document for view mode (lazy, from `SingleCorkboardCard`).
    read_doc: bastyde::text_document::TextDocument,
    root: Option<WidgetId>,
}
impl std::fmt::Debug for CardSynopsis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CardSynopsis").finish()
    }
}
impl Widget for CardSynopsis {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.editing_synopsis().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        let id = self.card.item_id;
        let editing = self.vm.editing_synopsis().get() == Some(id);

        // `ctx.add` each concrete branch (no boxing): the editable column, or a
        // read-only, hit-transparent viewer under a tap-catcher — a click anywhere
        // on the synopsis enters inline edit mode.
        let body = if editing {
            match self.vm.synopsis_edit_document(id) {
                Some(doc) => ctx.add(synopsis_editor(
                    &self.vm,
                    &self.card,
                    &doc,
                    self.vm.column_width(),
                )),
                None => ctx.add(
                    RichTextEditor::read_only(self.read_doc.clone())
                        .v_scroll_policy(ScrollPolicy::Auto),
                ),
            }
        } else {
            let vm = self.vm.clone();
            ctx.add(
                Expand::new()
                    .child(
                        RichTextEditor::read_only(self.read_doc.clone())
                            .v_scroll_policy(ScrollPolicy::Auto)
                            .hit_transparent(true),
                    )
                    .on_tap(move |_tap, _ctx| vm.begin_edit_synopsis(id)),
            )
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
    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// Open the synopsis in a roomier modal editor over the **same** shared document as
/// the inline card editor (edits reflect in both). Entering edit mode first ensures
/// the document is open.
fn present_synopsis_modal(vm: &CorkboardViewModel, card: &CorkboardCard, ctx: &mut EventContext) {
    vm.begin_edit_synopsis(card.item_id);
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
struct SynopsisModal {
    vm: CorkboardViewModel,
    card: CorkboardCard,
    root: Option<WidgetId>,
}
impl std::fmt::Debug for SynopsisModal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SynopsisModal").finish()
    }
}
impl Widget for SynopsisModal {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let id = self.card.item_id;
        // The self-centering writing column needs a *bounded* width proposal. The
        // in-tree modal sizes to its content, so declare the surface size explicitly
        // (an `Expand` would leave the column an unbounded width and it collapses to
        // one word per line). The column scrolls internally for a long synopsis.
        let cid = match self.vm.synopsis_edit_document(id) {
            Some(doc) => {
                let editor = synopsis_editor(&self.vm, &self.card, &doc, Signal::new(600.0));
                ctx.add(
                    FixedSize::new()
                        .width(680.0)
                        .height(500.0)
                        .child(Padding::uniform(16.0).child(Expand::new().child(editor))),
                )
            }
            None => {
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

/// A card's footer count: a container shows its child count; a leaf shows its word
/// count (reactive — it loads lazily), gated by the show-word-count setting.
struct FooterCount {
    is_container: bool,
    child_count: usize,
    word_count: Signal<Option<usize>>,
    show_wc: Signal<bool>,
    root: Option<WidgetId>,
}
impl std::fmt::Debug for FooterCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FooterCount").finish()
    }
}
impl Widget for FooterCount {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.word_count
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        self.show_wc
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let label = if self.is_container {
            tr!(corkboard_child_count(count = self.child_count as i64))
        } else if self.show_wc.get() {
            match self.word_count.get() {
                Some(n) => tr!(statusbar_word_count(count = n as i64)),
                None => lit!(""),
            }
        } else {
            lit!("")
        };
        let id = ctx.add(TextWidget::new(label).color(TextRole::Secondary));
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

/// The per-card "More actions" menu — the same actions as the Full-Synopsis row
/// menu (insert · set label · move up / down · merge · trash), minus rename. Merge
/// is offered only where the model allows it. Bare kebab `IconButton`.
fn card_menu(vm: &CorkboardViewModel, card: &CorkboardCard) -> impl Widget {
    let id = card.item_id;
    let mk = |f: fn(&CorkboardViewModel, &mut EventContext, u64)| {
        let vm = vm.clone();
        move |ctx: &mut EventContext| f(&vm, ctx, id)
    };

    let mut list = MenuList::new()
        // Rename opens the inline title editor on this card (same as F2 /
        // double-clicking the title) — no modal.
        .item(MenuItem::new(tr!(rename())).on_activate_fn(mk(|v, _c, id| v.begin_rename(id))))
        .item(
            MenuItem::new(insert_label(card))
                .on_activate_fn(mk(|v, c, id| v.begin_insert_after(c, id))),
        )
        .item(
            MenuItem::new(tr!(set_label())).on_activate_fn(mk(|v, c, id| v.begin_set_label(c, id))),
        )
        .separator()
        .item(MenuItem::new(tr!(move_up())).on_activate_fn(mk(|v, c, id| v.move_up(c, id))))
        .item(MenuItem::new(tr!(move_down())).on_activate_fn(mk(|v, c, id| v.move_down(c, id))));

    if vm.can_merge_into_previous(id) {
        list = list.item(
            MenuItem::new(tr!(merge_with_previous()))
                .on_activate_fn(mk(|v, c, id| v.merge_into_previous(c, id))),
        );
    }

    list = list.separator().item(
        MenuItem::new(tr!(move_to_trash()))
            .text_role(TextRole::Error)
            .on_activate_fn(mk(|v, c, id| v.trash(c, id))),
    );

    PopoverIconButton::new(IconButton::more())
        .bare()
        .content(list)
}

/// What "Insert …" on a card creates — the model's default recommendation for it
/// — so the menu item names that type rather than always saying "scene".
fn insert_label(card: &CorkboardCard) -> LocalizedString {
    let recommended = skribisto_model::recommendations(&card.role, &card.sub_role)
        .first()
        .map(|r| r.create_type);
    match recommended {
        Some(skribisto_model::CreateType::Chapter) => tr!(insert_chapter()),
        _ => tr!(insert_scene()),
    }
}

/// The concise accessible name a screen reader announces for a card's `GridCell`:
/// a superset of the visible title (Label-in-Name) — "Title, Type[, status]" —
/// not the whole synopsis (which is scanned visually / read on demand).
fn card_a11y_name(card: &CorkboardCard) -> String {
    let mut name = format!(
        "{}, {}",
        card.title,
        sub_role_badge_label(&card.sub_role).resolve_now()
    );
    if !card.label.is_empty() {
        name = format!("{name}, {}", card.label);
    }
    name
}

/// A short, sentence-case badge for a card's type. No existing sub_role→text map
/// (the icons are `binder_icons`), so it lives here, the only consumer.
fn sub_role_badge_label(sub_role: &BinderItemSubRole) -> LocalizedString {
    use BinderItemSubRole::*;
    match sub_role {
        Scene => tr!(corkboard_badge_scene()),
        ChapterScene => tr!(corkboard_badge_chapter()),
        Part => tr!(corkboard_badge_part()),
        Book | BookBegin => tr!(corkboard_badge_book()),
        Note => tr!(corkboard_badge_note()),
        None => tr!(corkboard_badge_folder()),
        BookEnd => tr!(corkboard_badge_end()),
        Text => tr!(corkboard_badge_text()),
    }
}
