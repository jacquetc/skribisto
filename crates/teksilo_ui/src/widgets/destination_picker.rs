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

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::data::{KeyedSelectionModel, SelectionMode, TreeDataSource};
use teksilo::prelude::TextStyleRole;
use teksilo::prelude::*;
use teksilo::widgets::{
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
    /// A destination asked for before the tree was able to hold it.
    ///
    /// [`preselect`](DestinationPicker::preselect) is called at construction — from a
    /// right-clicked binder row, or from the project's remembered destination — but the
    /// tree fills from backend events, so at that moment it is usually still empty and
    /// the key resolves to nothing. Holding the request here and retrying as the model
    /// reloads is what makes "open the wizard already pointing at this chapter" work
    /// without a sleep or a poll.
    pending: Rc<RefCell<Option<BinderTreeKey>>>,
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
            pending: Rc::new(RefCell::new(None)),
        }
    }

    /// Open pointing at `key`, as if the writer had clicked that row.
    ///
    /// The ancestors are expanded so the row is actually on screen — a selection the
    /// writer cannot see reads as no selection at all, and they would have no way to
    /// tell what the confirm button is about to do.
    ///
    /// Safe to call before the tree has loaded: the request is held and applied on the
    /// first reload that can satisfy it. If the tree loads without that row — it was
    /// trashed, or the project changed under a remembered destination — the request is
    /// dropped and the picker simply opens with nothing chosen, which is the same state
    /// it has always had.
    pub fn preselect(&self, key: BinderTreeKey) {
        *self.pending.borrow_mut() = Some(key);
        self.apply_pending();
    }

    /// Load the tree now, rather than waiting for a backend event.
    ///
    /// Needed whenever the picker outlives the moment `work_id` becomes real:
    /// the model sources rows once at construction and then only on filter
    /// changes or structural events, and a LoadWork that bulk-fills the store
    /// does not re-fire Binder Created events. The Import documents wizard is
    /// the main caller — its view-model is minted with the window, often while
    /// no project is open yet.
    ///
    /// A headless caller has no frame loop and no event pump either, so without
    /// this the tree is permanently empty and a [`preselect`](Self::preselect)
    /// can never resolve. Always follows the reload with
    /// [`apply_pending`](Self::apply_pending) so a destination asked for against
    /// an empty tree still lands once the rows arrive.
    pub(crate) fn reload(&self) {
        self.model.reload();
        self.apply_pending();
    }

    /// Try to satisfy a held [`preselect`](Self::preselect). Cheap and idempotent —
    /// called once at request time and again on every reload until it resolves.
    fn apply_pending(&self) {
        let Some(key) = *self.pending.borrow() else {
            return;
        };
        if self.model.contains(&key) {
            self.model.expand_ancestors(&key);
            self.selection.select(key);
            *self.pending.borrow_mut() = None;
        } else if self.model.visible_count() != 0 {
            // The tree has rows and this is not one of them, so waiting longer cannot
            // help. Dropping it now stops a stale request from firing much later, if
            // that row is ever restored from the trash.
            *self.pending.borrow_mut() = None;
        }
    }

    /// True while a destination is chosen — bind a confirm button's `enabled` to it.
    pub fn has_selection(&self) -> impl Into<Prop<bool>> + use<> {
        self.selection.selection_signal().map(|s| !s.is_empty())
    }

    /// The chosen row as its durable key — what a caller persists, since a
    /// `BinderDestination`'s ids are re-minted by every `load_work`.
    pub fn selected_key(&self) -> Option<BinderTreeKey> {
        self.selection.selected_keys().first().copied()
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

        // A destination asked for before the tree could hold it. `wire` above is what
        // starts the rows arriving, so this is the earliest point a held request can
        // be satisfied — and the model's version signal is what says "the rows just
        // changed", which is exactly when it is worth trying again.
        let picker = self.picker.clone();
        let version = self.picker.model.version_signal();
        ctx.effect(&version, move |_| picker.apply_pending());

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

        // Plain builders, not `teksu!`: `Switcher` takes its children as ordered
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
    use teksilo::prelude::SizeProposal;
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

/// The picker's behaviour over a **real** loaded project.
///
/// Separated and gated the way `footnotes_list_model`'s backend tests are: under
/// `--features mocks` the tree model's row seam fabricates a binder, so "the tree is
/// empty until a project arrives" is false and a uid read from the store names no row
/// the mock tree has. Both premises here are about the real backend, so this is where
/// they belong rather than being weakened until they pass in both.
#[cfg(all(test, not(feature = "mocks")))]
mod real_backend_tests {
    use super::*;
    use teksilo::prelude::SizeProposal;
    use std::rc::Rc;

    /// A picker over the shipped fixture, mounted so its tree is live.
    ///
    /// The mount matters: the model only starts filling once `wire` has run, which
    /// happens in `build`. A picker that is never put on screen has an empty tree, and
    /// every assertion about resolving a row against it would pass or fail for the
    /// wrong reason.
    fn load_fixture(app_ctx: &Rc<AppContext>) -> u64 {
        frontend::commands::work_management_commands::load_work(
            app_ctx,
            &frontend::work_management::LoadWorkDto {
                media_root: crate::media_paths::media_root_string(),
                file_name: format!(
                    "{}/../../resources/test/skribisto_test_project.skrib",
                    env!("CARGO_MANIFEST_DIR")
                ),
            },
        )
        .expect("load fixture");
        frontend::commands::work_commands::get_all_work(app_ctx)
            .expect("work")
            .first()
            .expect("one work")
            .id
    }

    /// The lowest-numbered live item's uid — deterministic, so the assertions below
    /// never depend on a `HashMap`'s iteration order.
    fn live_item_uid(app_ctx: &Rc<AppContext>) -> uuid::Uuid {
        frontend::commands::binder_item_commands::get_all_binder_item(app_ctx)
            .expect("items")
            .into_iter()
            .filter(|it| it.activated)
            .map(|it| it.uid)
            .min()
            .expect("the fixture has a live item")
    }

    fn loaded_picker() -> (
        DestinationPicker,
        Rc<AppContext>,
        teksilo::core::widget_tree::WidgetTree,
    ) {
        let app_ctx = Rc::new(AppContext::new());
        let work_id = load_fixture(&app_ctx);

        let picker = DestinationPicker::new(app_ctx.clone(), Signal::new(Some(work_id)));
        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        tree.add_boxed(Box::new(picker.view(lit!("Nothing here yet"))));
        tree.layout(SizeProposal::exact(560.0, 420.0));
        (picker, app_ctx, tree)
    }

    /// "Import here…" and the remembered destination both open the wizard already
    /// pointing at a row, named the durable way.
    #[test]
    fn a_preselected_row_is_the_one_that_ends_up_chosen() {
        let (picker, app_ctx, _tree) = loaded_picker();
        let wanted = live_item_uid(&app_ctx);

        picker.preselect(BinderTreeKey::Item(wanted));

        let chosen = picker.selected().expect("the preselect must have resolved");
        assert_eq!(
            chosen.anchor_item_id,
            picker.model.item_id_of(&BinderTreeKey::Item(wanted)),
            "the row that was asked for is the row that ended up chosen"
        );
    }

    /// The case the held request exists for: both real callers construct the picker and
    /// ask for a destination in the same breath, and a window opening on a project that
    /// is still loading has no rows to resolve against yet. The request must survive
    /// that and land on the reload that can satisfy it — without a poll or a sleep.
    #[test]
    fn a_preselect_asked_for_before_the_tree_exists_still_lands() {
        let app_ctx = Rc::new(AppContext::new());
        let work = Signal::new(None);
        let picker = DestinationPicker::new(app_ctx.clone(), work.clone());

        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        tree.add_boxed(Box::new(picker.view(lit!("Nothing here yet"))));
        tree.layout(SizeProposal::exact(560.0, 420.0));

        // Nothing is open, so nothing can resolve — and the request must be *kept*.
        // Discarding it here is the bug this whole mechanism exists to avoid: the
        // wizard would open pointing nowhere and the writer would never know it had
        // been told where to point.
        picker.preselect(BinderTreeKey::Item(uuid::Uuid::from_u128(0x1234)));
        assert_eq!(picker.selected(), None);
        assert!(
            picker.pending.borrow().is_some(),
            "an unresolvable request over an empty tree must be held, not discarded"
        );

        // The project arrives, and the request names a row it really has.
        let real = load_fixture(&app_ctx);
        *picker.pending.borrow_mut() = Some(BinderTreeKey::Item(live_item_uid(&app_ctx)));
        work.set(Some(real));
        picker.model.reload();
        tree.layout(SizeProposal::exact(560.0, 420.0));

        assert!(
            picker.selected().is_some(),
            "the held request must resolve on the reload that can satisfy it"
        );
        assert!(
            picker.pending.borrow().is_none(),
            "and must be cleared, so a later reload cannot re-apply it over a newer choice"
        );
    }

    /// A remembered destination outlives the row it names — the writer trashes that
    /// chapter, or opens a different project. The picker must open empty, not wrong.
    #[test]
    fn a_preselect_for_a_row_that_is_not_there_is_dropped() {
        let (picker, _app_ctx, _tree) = loaded_picker();
        picker.preselect(BinderTreeKey::Item(uuid::Uuid::from_u128(0xdead_beef)));
        assert_eq!(
            picker.selected(),
            None,
            "a destination that is no longer in the tree must select nothing"
        );
        assert!(
            picker.pending.borrow().is_none(),
            "and the request must be dropped rather than left to fire on a later reload"
        );
    }
}
