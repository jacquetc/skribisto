// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! "Where should this land?" — the live binder tree, single-select, read only.
//!
//! Extracted from the **Restore to…** modal once document import needed the same
//! question asked. The machinery was never trash-specific: a
//! [`BinderBinderItemsTreeModel`] over the open Work, a
//! [`KeyedSelectionModel`] keyed by durable uid, and the small piece of judgement
//! that turns "the writer clicked this row" into a place — a binder, an anchor,
//! and whether the new material goes *inside* the row or *after* it.
//!
//! Only the tree excludes trashed rows, which both callers want for the same
//! reason: a destination must be somewhere the writer can actually see.
//!
//! ## Handle and view
//!
//! [`DestinationPicker`] is a cheap-to-clone handle; [`DestinationPicker::view`]
//! builds the widget. The split exists because a confirm button needs to read the
//! selection *after* the tree has been handed to the widget tree, which a single
//! owning widget could not allow. It mirrors the view-model/view split the rest of
//! the app uses, at widget scale.

use bastyde::data::{KeyedSelectionModel, SelectionMode, TreeDataSource};
use bastyde::prelude::TextStyleRole;
use bastyde::prelude::*;
use bastyde::widgets::{
    Padding, ScrollBarMode, StandardTreeItem, Switcher, TextWidget, TreeRow, TreeView,
};

use frontend::AppContext;
use frontend::trash_management::DropPosition;

use crate::models::{BinderBinderItemsTreeModel, BinderTreeKey, TreeFilters, TreeNode};

/// A place in the binder, as the writer pointed at it.
///
/// `DropPosition` rather than a local enum of the same shape: it is already this
/// app's general drop vocabulary — three models and three view-models speak it —
/// so inventing a parallel one here would mean every caller translating between
/// two words for the same idea.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinderDestination {
    pub binder_id: u64,
    /// The row to land relative to, or `None` when a whole binder was chosen.
    pub anchor_item_id: Option<u64>,
    pub position: DropPosition,
    /// What the writer will see it called, for a confirmation sentence.
    pub title: String,
}

/// A handle on the picker's live state. Cheap to clone; clone it into the
/// handlers that need to read the selection.
#[derive(Clone)]
pub struct DestinationPicker {
    model: BinderBinderItemsTreeModel,
    selection: KeyedSelectionModel<BinderTreeKey>,
}

impl DestinationPicker {
    /// A picker over `work_id`'s binders.
    ///
    /// Its own tree model rather than the outline's: the outline carries the
    /// writer's expansion state, search query and selection, and borrowing it
    /// would let a transient modal reorganise the window behind it.
    pub fn new(app_ctx: std::rc::Rc<AppContext>, work_id: Signal<Option<u64>>) -> Self {
        let filters = TreeFilters {
            binder: Signal::new(None),
            query: Signal::new(String::new()),
            match_counts: Signal::new((0, 0)),
            all_binders: Signal::new(false),
        };
        Self {
            model: BinderBinderItemsTreeModel::new(app_ctx, work_id, filters),
            selection: KeyedSelectionModel::new(SelectionMode::Single),
        }
    }

    /// True while a destination is chosen — bind a confirm button's `enabled` to it.
    pub fn has_selection(&self) -> impl Into<Prop<bool>> + use<> {
        self.selection.selection_signal().map(|s| !s.is_empty())
    }

    /// Where the writer pointed, or `None` if nowhere yet.
    ///
    /// A binder row means "into this binder"; a container row means "inside it";
    /// anything else means "after it". That last distinction is the whole reason
    /// this is not just a selection model: clicking a scene means *beside* that
    /// scene, and clicking a chapter means *within* it, which is what the writer
    /// is pointing at in either case.
    pub fn selected(&self) -> Option<BinderDestination> {
        let key = self.selection.selected_keys().first().copied()?;
        let binder_id = self.model.binder_of(&key).unwrap_or(0);
        let title = self.model.node_of(&key).map(|(_, t)| t).unwrap_or_default();

        let (anchor_item_id, position) = match self.model.item_id_of(&key) {
            // A binder row, or one that has vanished under us.
            None => (None, DropPosition::Into),
            Some(item_id) if self.model.node_is_folder(&key) => (Some(item_id), DropPosition::Into),
            Some(item_id) => (Some(item_id), DropPosition::After),
        };

        Some(BinderDestination {
            binder_id,
            anchor_item_id,
            position,
            title,
        })
    }

    /// The tree widget. `empty_text` is the caller's, because "nothing to restore
    /// into" and "this project has no binders yet" are different situations that
    /// happen to render the same way.
    pub fn view(&self, empty_text: impl Into<LocalizedString>) -> DestinationPickerView {
        DestinationPickerView {
            picker: self.clone(),
            empty_text: empty_text.into(),
            root_child: None,
        }
    }
}

/// The picker's widget half. Build one with [`DestinationPicker::view`].
pub struct DestinationPickerView {
    picker: DestinationPicker,
    empty_text: LocalizedString,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for DestinationPickerView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DestinationPickerView").finish()
    }
}

impl Widget for DestinationPickerView {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Keep the tree live for as long as it is on screen.
        self.picker.model.wire(ctx);

        // Read-only and single-select: no reorder, no context menu, no
        // open-on-activate. Picking a place must not edit the thing being pointed at.
        let tree = TreeView::from_source_keyed(
            self.picker.model.clone(),
            self.picker.selection.clone(),
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
                // These are live rows, so their chapter numbers are current and
                // mean something — unlike the trashed rows in the trash dock.
                if badge.is_some() {
                    item = item.center_slot(crate::widgets::StructureNumber::new(badge));
                }
                Box::new(item) as Box<dyn Widget>
            },
        )
        .auto_item_height(28.0)
        .scroll_bar_style(ScrollBarMode::Overlay)
        .row_click_expands(false);

        let empty_model = self.picker.model.clone();
        let switch = self
            .picker
            .model
            .version_signal()
            .map(move |_| usize::from(empty_model.visible_count() != 0));

        // Plain builders, not `bati!`: `Switcher` takes its children as ordered
        // closure-built slots, which the macro cannot express — the same reason
        // the DockingLayout and TabWidget call sites use builders.
        let empty_text = self.empty_text.clone();
        let switcher = Switcher::new(switch)
            .child(
                Padding::symmetric(24.0, 40.0).child(
                    TextWidget::new(empty_text)
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
            )
            .child(tree);
        let root = ctx.add(switcher);
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::prelude::SizeProposal;
    use std::rc::Rc;

    fn picker() -> DestinationPicker {
        DestinationPicker::new(Rc::new(AppContext::new()), Signal::new(None))
    }

    /// The point of the extraction: the picker stands up on its own, with no
    /// `TrashViewModel` anywhere near it. If this ever needs one again, the seam
    /// has moved back to where it was.
    #[test]
    fn the_picker_builds_and_lays_out_without_any_feature_view_model() {
        let app_ctx = Rc::new(AppContext::new());
        let picker = DestinationPicker::new(app_ctx.clone(), Signal::new(None));

        // The tree model subscribes to backend events, so it needs a tree that has
        // an event source (see `crate::test_support`).
        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        let id = tree.add_boxed(Box::new(picker.view(lit!("Nothing here yet"))));
        tree.layout(SizeProposal::exact(560.0, 420.0));

        let bounds = tree.bounds(id);
        assert!(
            bounds.width > 0.0 && bounds.height > 0.0,
            "the picker laid out to nothing ({bounds:?})"
        );
    }

    #[test]
    fn nothing_is_selected_until_the_writer_picks_something() {
        assert_eq!(picker().selected(), None);
    }

    /// Two independent pickers must not share a selection — a modal opened twice
    /// would otherwise start out pointing wherever the last one did.
    #[test]
    fn two_pickers_do_not_share_state() {
        let a = picker();
        let b = picker();
        assert_eq!(a.selected(), None);
        assert_eq!(b.selected(), None);
    }
}
