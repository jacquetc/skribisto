// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `GoToButton` — the "jump to any item" popover and its trigger.
//!
//! A search field over the binder tree: empty query shows the collapsible
//! tree, typing narrows it in place while keeping each match's parent chain
//! (`BinderBinderItemsTreeModel`'s own filter is ancestor-preserving, so
//! structure never disappears out from under a match). Enter jumps.
//!
//! **One popover, three ways in.** The trigger opens it; so do the Go menu's
//! "Go to…" entry and Ctrl+G, through `PopoverWidget::open_action("go.to")` —
//! which registers a *global* action running the very same toggle closure the
//! trigger runs, so the keyboard and the pointer cannot drift apart.
//! `open_signal()` is a read-back mirror only; writing it opens nothing.
//!
//! **Escape is scoped to the popover.** The popover's own dismiss consumes it,
//! so pressing Escape here closes the list and leaves you exactly where you
//! were — it must not fall through to `App`'s contextless-Escape handler and
//! drop the writer out of distraction-free mode as well. Two things at once
//! from one keystroke is the surprise this avoids.
//!
//! Keyboard shape: focus lands in the search field (the popover focuses its
//! first focusable descendant), Down/Up move the tree's selection without
//! leaving the field, Enter activates the selected row. That is the palette
//! convention — a writer types, glances, and commits without ever reaching for
//! the mouse or Tab.

use std::rc::Rc;

use teksilo::core::overlay::OverlayPlacement;
use teksilo::data::TreeDataSource;
use teksilo::prelude::*;
use teksilo::widgets::{
    ActivateOn, Expand, IconButton, MinSize, Padding, PopoverIconButton, ScrollBarMode,
    SearchField, StandardTreeItem, TextWidget, TreeRow, TreeView, VStack,
};

use crate::go::GoToViewModel;
use crate::models::TreeNode;

/// The two per-instance action names. One `GoToButton` lives in the status bar
/// and one in the distraction-free strip, and only ever one is on screen — so
/// they must not share a name. `app/commands/go.rs` owns the public `go.to` and
/// forwards it to whichever is currently visible.
pub const GO_TO_MAIN: &str = "go.to.main";
pub const GO_TO_FOCUS: &str = "go.to.focus";

/// Popover geometry. Wide enough for a chapter title plus its label, tall
/// enough for a dozen rows — past that the tree scrolls rather than the
/// popover growing to the height of a 31-chapter binder.
const POPUP_WIDTH: f32 = 380.0;
const POPUP_HEIGHT: f32 = 360.0;

/// The popover body: search field above, filtered binder tree below.
struct GoToPalette {
    vm: GoToViewModel,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for GoToPalette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoToPalette").finish()
    }
}

impl Widget for GoToPalette {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let vm = self.vm.clone();
        let model = vm.model();

        // Enter jumps without ever touching the arrows — the common path is
        // "type three letters, press Enter", and requiring a Down first to give
        // Enter a target would make the fast case slower than the slow one.
        // `activate_current` takes the highlight if there is one and the first
        // match otherwise.
        let submit_vm = vm.clone();
        let field = SearchField::new(vm.query())
            .placeholder(tr!(go_to_placeholder()))
            .on_submit_fn(move |ctx| {
                if submit_vm.activate_current() {
                    // See the tree's `on_activate` below for why this is
                    // `dismiss_top_overlay` and not one of the scoped variants.
                    ctx.dismiss_top_overlay();
                }
            });

        let activate_model = model.clone();
        let activate_vm = vm.clone();
        let tree = TreeView::from_source_keyed(
            model.clone(),
            vm.selection(),
            move |node: &TreeNode, row: &TreeRow, selected: bool| {
                let (label, badge) = crate::models::label_and_badge(
                    &node.title,
                    node.fallback_label.as_deref(),
                    node.number,
                );
                let mut item = StandardTreeItem::new(lit!(label))
                    .depth(row.depth)
                    .has_children(row.has_children)
                    .is_expanded(row.is_expanded)
                    .selected(selected)
                    .on_toggle_rc(row.toggle_callback());
                if !node.label.is_empty() {
                    item = item.subtitle(lit!(node.label.clone()));
                }
                let icon = if node.kind == "binder" {
                    crate::binder::icons::binder_icon()
                } else {
                    crate::binder::icons::kind_sub_role_icon(&node.kind, &node.sub_role)
                };
                item = item.leading_slot(icon);
                // The chapter's ordinal, between the icon and the title — jumping by
                // number is a real workflow ("take me to chapter 19"). A separate slot,
                // never spliced into the title: the tree model's filter reads `title`,
                // `label` and `fallback_label` as three fields, and a numeral folded into
                // the first would reach every rename that seeds from it.
                if badge.is_some() {
                    item = item.center_slot(crate::widgets::StructureNumber::new(badge));
                }
                Box::new(item) as Box<dyn Widget>
            },
        )
        .auto_item_height(28.0)
        .scroll_bar_style(ScrollBarMode::Overlay)
        // Single click jumps, matching the binder tree's own convention —
        // arrow keys only move the highlight, so stepping down the list never
        // fires a jump you did not ask for.
        .activate_on(ActivateOn::SingleClick)
        .on_activate(move |idx, ctx| {
            if let Some(key) = activate_model.key_at(idx)
                && let Some((item_id, title)) = activate_model.node_of(&key)
            {
                // `node_of` yields `(Option<item_id>, title)`; rebuild the
                // shape `activate` wants and let the view-model decide. It is
                // the one that knows a binder row is not a destination.
                let jumped = activate_vm.activate(&TreeNode {
                    title,
                    item_id,
                    kind: if item_id.is_some() { "item" } else { "binder" }.to_string(),
                    ..Default::default()
                });
                // Dismiss the overlay itself, not merely the view-model's flag:
                // `GoToViewModel::hide` owns "is the popup logically open", but
                // the popover is presented by the framework and only an
                // `EventContext` can take it down. Without this the writer
                // jumps and the list stays sitting over the manuscript.
                //
                // `dismiss_top_overlay`, **not** `dismiss_self_overlay_chain`
                // or `dismiss_all_except_hosts`: both of those deliberately
                // preserve dialog-role surfaces, which is what a popover panel
                // is, so from inside one they are silent no-ops. This popover is
                // the topmost overlay whenever a row is activated in it.
                if jumped {
                    ctx.dismiss_top_overlay();
                }
            }
        });

        // "No matches" only once a query has been typed **and** the projection
        // really is empty. Zipping the model's version signal is what makes it
        // reactive: the row set changes when the model re-sources, which the
        // query signal alone does not announce.
        let count_model = model.clone();
        let empty_query = vm.query();
        let empty = TextWidget::new(tr!(go_to_no_matches()))
            .color(TextRole::Secondary)
            .visible_when(
                empty_query
                    .zip(&model.version_signal())
                    .map(move |(q, _)| !q.trim().is_empty() && count_model.visible_count() == 0),
            );

        // Down/Up move the highlight through the list **while focus stays in the
        // field**, so a writer never has to Tab out of what they are typing to
        // choose a result. Handled on the wrapping column so it applies whether
        // focus is still in the field or has moved into the tree.
        let keys_vm = vm.clone();
        let body = MinSize::new(POPUP_WIDTH, 0.0)
            .child(
                VStack::new()
                    .spacing(6.0)
                    .child(Padding::symmetric(0.0, 2.0).child(field))
                    .child(MinSize::new(POPUP_WIDTH, POPUP_HEIGHT).child(Expand::new().child(tree)))
                    .child(empty),
            )
            .on_key(move |ev, _ctx| match ev {
                WidgetEvent::KeyDown {
                    key: Key::ArrowDown,
                    ..
                } => {
                    keys_vm.step_selection(1);
                    EventResponse::Handled
                }
                WidgetEvent::KeyDown {
                    key: Key::ArrowUp, ..
                } => {
                    keys_vm.step_selection(-1);
                    EventResponse::Handled
                }
                _ => EventResponse::Ignored,
            });
        let id = ctx.add_boxed(Box::new(body));
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// The status-bar / distraction-free-strip trigger plus its popover.
pub struct GoToButton {
    vm: GoToViewModel,
    /// The named action that opens *this* instance.
    ///
    /// Two instances exist per window — one in the status bar, one in the
    /// distraction-free strip — and only ever one of them is on screen. They
    /// must therefore not share a name: a single `go.to` registered twice is
    /// answered by whichever registration wins, and if that is the hidden one
    /// its trigger has no live bounds, so the popover opens anchored to
    /// nothing — in the top-left corner of the screen. `app/commands/go.rs`
    /// owns the public `go.to` and forwards it to whichever of these is
    /// currently visible.
    action: &'static str,
    root: Option<WidgetId>,
}

impl GoToButton {
    pub fn new(vm: GoToViewModel, action: &'static str) -> Self {
        Self {
            vm,
            action,
            root: None,
        }
    }
}

impl std::fmt::Debug for GoToButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoToButton").finish()
    }
}

impl Widget for GoToButton {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let vm = self.vm.clone();
        let content = GoToPalette {
            vm: vm.clone(),
            root: None,
        };
        // `on_open` clears the stale query (see `GoToViewModel::show`'s doc):
        // reopening pre-filtered to a search you cannot see the cause of reads
        // as a binder that lost most of its rows.
        let opening = vm.clone();
        let closing = vm.clone();
        let root = ctx.add(
            // A flat icon trigger, matching the strip's own ‹ › pair and the
            // status bar's other controls — a text button was the odd one out in
            // both places, and in the strip it competed with Exit, the one
            // control there that must read as the primary action.
            PopoverIconButton::new(
                IconButton::new(crate::icons::go::go_to_icon())
                    .toolbar()
                    .tooltip(tr!(tooltip_go_to())),
            )
            .content(content)
            // `IconButton` triggers default the disclosure caret ON; off here,
            // because the strip is a row of bare glyphs and one carrying a
            // chevron would read as a different kind of control.
            .show_disclosure_caret(false)
            // The trigger sits at the very bottom of the window (status bar, or
            // the distraction-free strip on a fullscreen screen), so the list
            // opens upward. Anchored below would land off-screen.
            .placement(OverlayPlacement::Above)
            .open_action(self.action)
            .on_open(move || opening.show())
            .on_close(move || closing.hide()),
        );
        self.root = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// The shared `Rc` shape `App` injects so the popup can open a document
/// without importing `EditorsViewModel` (see the view-model's own doc).
pub type OpenItemFn = Rc<dyn Fn(u64, &str)>;
