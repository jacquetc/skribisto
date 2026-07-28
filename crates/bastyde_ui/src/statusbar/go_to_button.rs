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
//! trigger runs, so the keyboard and the pointer cannot drift apart. (That
//! mechanism did not exist until this feature needed it; `open_signal()` is a
//! read-back mirror and writing it opens nothing.)
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

use bastyde::core::overlay::OverlayPlacement;
use bastyde::data::TreeDataSource;
use bastyde::prelude::*;
use bastyde::widgets::{
    ActivateOn, Button, ButtonVariant, Expand, FocusScope, MinSize, Padding, PopoverButton,
    ScrollBarMode, SearchField, StandardTreeItem, TextWidget, TraversalScopePolicy, TreeRow,
    TreeView, VStack,
};

use crate::models::TreeNode;
use crate::view_models::GoToViewModel;

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

        let field = SearchField::new(vm.query()).placeholder(tr!(go_to_placeholder()));

        let activate_model = model.clone();
        let activate_vm = vm.clone();
        let tree = TreeView::from_source_keyed(
            model.clone(),
            vm.selection(),
            move |node: &TreeNode, row: &TreeRow, selected: bool| {
                let mut item = StandardTreeItem::new(lit!(node.title.clone()))
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
                    crate::binder::icons::sub_role_icon(&node.sub_role)
                };
                Box::new(item.leading_slot(icon)) as Box<dyn Widget>
            },
        )
        .auto_item_height(28.0)
        .scroll_bar_style(ScrollBarMode::Overlay)
        // Single click jumps, matching the binder tree's own convention —
        // arrow keys only move the highlight, so stepping down the list never
        // fires a jump you did not ask for.
        .activate_on(ActivateOn::SingleClick)
        .on_activate(move |idx| {
            if let Some(key) = activate_model.key_at(idx)
                && let Some((item_id, title)) = activate_model.node_of(&key)
            {
                // `node_of` yields `(Option<item_id>, title)`; rebuild the
                // shape `activate` wants and let the view-model decide. It is
                // the one that knows a binder row is not a destination.
                activate_vm.activate(&TreeNode {
                    title,
                    item_id,
                    kind: if item_id.is_some() { "item" } else { "binder" }.to_string(),
                    ..Default::default()
                });
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
            .visible_when(empty_query.zip(&model.version_signal()).map(move |(q, _)| {
                !q.trim().is_empty() && count_model.visible_count() == 0
            }));

        // Trap Tab inside the popover: a keyboard-only writer must be able to
        // reach the search field and the list without tabbing straight back out
        // into the window behind (WCAG 2.1.1). `Cycle`, matching
        // `ProjectSwitcherButton`; the crate's own `a11y` lint enforces this for
        // every file that opens a popover, and it caught this one.
        let body = FocusScope::new(TraversalScopePolicy::Cycle).child(
            MinSize::new(POPUP_WIDTH, 0.0).child(
            VStack::new()
                .spacing(6.0)
                .child(Padding::symmetric(0.0, 2.0).child(field))
                .child(MinSize::new(POPUP_WIDTH, POPUP_HEIGHT).child(Expand::new().child(tree)))
                .child(empty),
            ),
        );
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
    root: Option<WidgetId>,
}

impl GoToButton {
    pub fn new(vm: GoToViewModel) -> Self {
        Self { vm, root: None }
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
            PopoverButton::new(
                Button::new(tr!(statusbar_go_to()))
                    .variant(ButtonVariant::Plain)
                    .tooltip(tr!(tooltip_go_to())),
            )
            .content(content)
            .show_disclosure_caret(false)
            // The trigger sits at the very bottom of the window (status bar, or
            // the distraction-free strip on a fullscreen screen), so the list
            // opens upward. Anchored below would land off-screen.
            .placement(OverlayPlacement::Above)
            .open_action("go.to")
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
