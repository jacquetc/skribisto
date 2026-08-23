// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Outline dock's header controls: a **binder switcher** and a **tree
//! search**, both on one row above the tree.
//!
//! [`BinderSwitcherButton`] is a flat dropdown whose main slot shows the current
//! binder (or "All Binders") and whose popover lists the Work's binders — each
//! with an item count, the current one check-marked — plus a "Show all binders"
//! entry and a "New binder…" entry. Right-clicking a binder offers **Send to
//! Trash** behind a confirmation (`MessageBox` → `AppIntent::TrashBinder` → the
//! `binder.trash` global action). Modelled on
//! [`ProjectSwitcherButton`](crate::shell::project_switcher_button): a `PopoverButton`
//! over rich `MenuList` rows, refreshed by rebinding a `version` signal.
//!
//! [`binder_search_button`] is a ghost `PopoverIconButton` (magnifier) whose
//! popover holds a `SearchField` — two-way bound to the outline's live query
//! signal so the tree filters as you type — and a toggleable `IconButton`
//! switching the search scope between the current binder and all binders. The
//! filtering itself lives in the tree model (`TreeRowFilter`); these widgets
//! only drive the [`OutlineViewModel`]'s filter signals.

use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, FixedSize, HStack, IconButton, IconWidget, MenuItem, MenuList,
    MessageBox, MessageBoxButtons, Padding, PopoverButton, PopoverIconButton, SearchField, Spacer,
    StandardButton, TextWidget,
};

use frontend::AppContext;

use crate::binder::OutlineViewModel;
use crate::binder::icons::binder_icon;
use crate::intents::AppIntent;
use crate::models::{BinderListModel, BinderRow};

/// Row width for the popover, so the trailing item counts align.
const ROW_WIDTH: f32 = 236.0;

/// Flat dropdown selecting which binder the outline tree shows.
pub struct BinderSwitcherButton {
    outline: OutlineViewModel,
    /// Reactive binder list (Layer A); its `version` signal is bound at
    /// `BindingLevel::Rebuild` so the list + current-binder label re-derive.
    model: BinderListModel,
    root_child: Option<WidgetId>,
}

impl BinderSwitcherButton {
    pub fn new(outline: OutlineViewModel, app_ctx: Rc<AppContext>) -> Self {
        let model = BinderListModel::new(app_ctx, outline.work_id_signal());
        Self {
            outline,
            model,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for BinderSwitcherButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinderSwitcherButton").finish()
    }
}

impl Widget for BinderSwitcherButton {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Refresh the list on binder/item events, and re-derive the label +
        // checkmarks when the displayed binder changes. Both at Rebuild level.
        self.model.wire(ctx);
        self.model.version_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        self.outline.binder_filter_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        let current = self.outline.binder_filter_signal().get();
        let binders = self.model.items();

        // ── Popover: "Show all binders", each binder, then "New binder…". ──
        let mut menu = MenuList::new().max_visible_items(12);
        menu = menu.item(show_all_row(&self.outline, current.is_none()));
        menu = menu.separator();
        for b in &binders {
            menu = menu.item(binder_row(&self.outline, b, current == Some(b.id)));
        }
        menu = menu.separator();
        menu = menu.item(new_binder_row(&self.outline));

        // ── Trigger: current binder name (or "All Binders") + a chevron. ──
        let label = current
            .and_then(|id| binders.iter().find(|b| b.id == id))
            .map(|b| b.name.clone());
        let trigger = Button::new(match label {
            Some(name) => lit!(name),
            None => tr!(binder_all()),
        })
        .variant(ButtonVariant::Ghost)
        .text_style(TextStyleRole::BodyBold)
        .trailing(IconWidget::chevron_down(12.0));

        let root = ctx.add(
            PopoverButton::new(trigger)
                .show_disclosure_caret(false)
                .content(menu),
        );
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0))
            .into()
    }
}

/// A 16 px leading column holding a checkmark when `current`.
fn marker(current: bool) -> FixedSize {
    let mut m = FixedSize::new().width(16.0).height(16.0);
    if current {
        m = m.child(IconWidget::checkmark(14.0));
    }
    m
}

fn title_color(current: bool) -> TextRole {
    if current {
        TextRole::Accent
    } else {
        TextRole::Primary
    }
}

/// The "Show all binders" popover entry — clears the display filter.
fn show_all_row(outline: &OutlineViewModel, current: bool) -> impl Widget {
    let outline = outline.clone();
    let row = HStack::new()
        .spacing(8.0)
        .child(marker(current))
        .child(
            TextWidget::new(tr!(binder_show_all()))
                .style(TextStyleRole::Body)
                .color(title_color(current)),
        )
        .cursor(CursorIcon::Pointer)
        .focusable(true)
        .on_tap(move |_event, ctx| {
            ctx.dismiss_self_overlay_chain();
            outline.set_binder_filter(None);
        });
    Padding::symmetric(6.0, 8.0).child(FixedSize::new().width(ROW_WIDTH).child(row))
}

/// One binder entry — pick it on tap; right-click → Send to Trash (confirmed).
fn binder_row(outline: &OutlineViewModel, b: &BinderRow, current: bool) -> impl Widget {
    let id = b.id;
    let name = b.name.clone();

    let pick = outline.clone();
    let row = HStack::new()
        .spacing(8.0)
        .child(marker(current))
        .child(binder_icon())
        .child(
            TextWidget::new(lit!(b.name.clone()))
                .style(TextStyleRole::BodyBold)
                .color(title_color(current))
                .single_line()
                .overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing)),
        )
        .child(Spacer::new())
        .child(
            TextWidget::new(tr!(binder_item_count(count = b.item_count as i64)))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
        .cursor(CursorIcon::Pointer)
        .focusable(true)
        .on_tap(move |_event, ctx| {
            ctx.dismiss_self_overlay_chain();
            pick.set_binder_filter(Some(id));
        })
        // Right-click → confirm → intent → the `binder.trash` global action.
        .context_menu(move |_pos, _ctx| {
            let name = name.clone();
            let menu =
                MenuList::new().item(MenuItem::new(tr!(ctx_trash())).on_activate_fn(move |ctx| {
                    let name = name.clone();
                    MessageBox::question(tr!(binder_trash_confirm_title()))
                        .text(tr!(binder_trash_confirm_text(name = name)))
                        .buttons(MessageBoxButtons::OkCancel)
                        .on_result(move |r, ctx| {
                            if r.button == StandardButton::Ok {
                                ctx.send_intent(AppIntent::TrashBinder {
                                    binder_id: id as i64,
                                });
                            }
                        })
                        .present(ctx);
                }));
            Some(Box::new(menu) as Box<dyn Widget>)
        });
    Padding::symmetric(6.0, 8.0).child(FixedSize::new().width(ROW_WIDTH).child(row))
}

/// The "New binder…" popover entry — create + rename dialog.
fn new_binder_row(outline: &OutlineViewModel) -> impl Widget {
    let outline = outline.clone();
    let row = HStack::new()
        .spacing(8.0)
        .child(marker(false))
        .child(
            TextWidget::new(tr!(binder_new()))
                .style(TextStyleRole::Body)
                .color(TextRole::Secondary),
        )
        .cursor(CursorIcon::Pointer)
        .focusable(true)
        .on_tap(move |_event, ctx| {
            ctx.dismiss_self_overlay_chain();
            outline.new_binder(ctx);
        });
    Padding::symmetric(6.0, 8.0).child(FixedSize::new().width(ROW_WIDTH).child(row))
}

/// The ghost search button: a magnifier whose popover filters the tree live.
pub fn binder_search_button(outline: OutlineViewModel) -> impl Widget {
    let panel = Padding::symmetric(8.0, 8.0).child(
        HStack::new()
            .spacing(6.0)
            .child(
                FixedSize::new().width(200.0).child(
                    SearchField::new(outline.search_query_signal())
                        .placeholder(tr!(binder_search_placeholder())),
                ),
            )
            .child(
                // Bistate: surface-tint "on" == search across all binders.
                IconButton::new(binder_icon())
                    .toggle(outline.search_all_signal())
                    .tooltip(tr!(binder_search_scope())),
            ),
    );
    PopoverIconButton::new(IconButton::search().toolbar())
        .show_disclosure_caret(false)
        .content(panel)
}
