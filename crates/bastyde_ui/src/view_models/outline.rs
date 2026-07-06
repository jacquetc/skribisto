//! `OutlineViewModel` — the binder/outline dock: its tree, its selection, and its
//! visibility. Visibility is *delegated* to the `DockingModel` (which is itself a
//! cloneable model handle); the view-model adds no visibility state of its own —
//! it only binds the dock id + side and speaks Skribisto vocabulary.
//!
//! Single-instance live state: owns the `DockingModel` and the tree model.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use bastyde::data::{DropPosition, KeyedSelectionModel, SelectionMode};
use bastyde::prelude::*; // EventContext, Signal, tr!
use bastyde::widgets::{
    DockOpenLocation, DockSide, DockWidgetId, DockingModel, InputDialog,
};

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands,
    trash_management_commands, undo_redo_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::direct_access::{
    CreateBinderDto, CreateBinderItemDto, UpdateBinderDto, UpdateBinderItemDto,
};

use frontend::binder_item_management::{DuplicateDto, MoveDto, MovePlace};
use frontend::trash_management::{TrashBinderDto, TrashBinderItemsDto};

use crate::app_ids::AppIds;
use crate::models::{BinderBinderItemsTreeModel, BinderTreeKey, CommitMove, TreeFilters};
use crate::singles::{SingleBinder, SingleBinderItem};

#[derive(Clone)]
pub struct OutlineViewModel {
    app_ctx: Rc<AppContext>,
    model: BinderBinderItemsTreeModel,
    selection: KeyedSelectionModel<BinderTreeKey>,
    docking: DockingModel,
    dock_id: DockWidgetId,
    /// The app's id-only global state (root/work/work-info/undo-stack ids).
    /// Binder-tree mutations run on `ids.stack_id` so one Ctrl+Z history reverses
    /// them all; shared by clone with `EditorsViewModel` and the singles.
    ids: AppIds,
    /// Reactive read handles (Layer A) re-pointed to fetch the current state of
    /// the item/binder a mutation is about to update — replacing ad-hoc `get_*`.
    item_probe: SingleBinderItem,
    binder_probe: SingleBinder,
    /// The binder-switcher + search filter signals, shared (by clone) with the
    /// tree model, which observes them and re-sources itself on change.
    filters: TreeFilters,
}

// The visibility / reveal / action methods are the feature's public API
// (callable from menus, shortcuts, other view-models); wired incrementally, so
// not every one has a caller yet.
#[allow(dead_code)]
impl OutlineViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        model: BinderBinderItemsTreeModel,
        docking: DockingModel,
        dock_id: DockWidgetId,
        filters: TreeFilters,
    ) -> Self {
        let vm = Self {
            item_probe: SingleBinderItem::new(app_ctx.clone()),
            binder_probe: SingleBinder::new(app_ctx.clone()),
            app_ctx,
            model,
            // Multi: Ctrl/Shift-click, Shift+arrow and Ctrl+A extend the set;
            // the batch actions (trash/duplicate/indent) already act on the
            // whole selection and `reload`'s `prune_missing` handles a multi-set.
            selection: KeyedSelectionModel::new(SelectionMode::Multi),
            docking,
            dock_id,
            ids,
            filters,
        };
        vm.install_reorder();
        vm
    }

    /// The current `BinderItem` DTO, read through the reactive single (Layer A) —
    /// the source of truth when a tree mutation builds its update.
    fn item_dto(&self, id: u64) -> Option<frontend::direct_access::BinderItemDto> {
        self.item_probe.set_id(Some(id));
        self.item_probe.dto()
    }

    /// The current `Binder` DTO, read through the reactive single (Layer A).
    fn binder_dto(&self, id: u64) -> Option<frontend::direct_access::BinderDto> {
        self.binder_probe.set_id(Some(id));
        self.binder_probe.dto()
    }

    /// Build the outline view-model with its standard leading-dock presentation
    /// (280 px side + 48 px activity rail). The single place that knows the
    /// outline's dock geometry — so `App` and the title-bar menu share one
    /// handle without either re-stating layout constants.
    pub fn new_default(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        // The switcher/search filter signals — the single source of truth, shared
        // (by clone) into the tree model and read back by the header widgets.
        let filters = TreeFilters {
            binder: Signal::new(None),
            query: Signal::new(String::new()),
            all_binders: Signal::new(false),
        };
        let model = BinderBinderItemsTreeModel::new(
            app_ctx.clone(),
            ids.work_id.clone(),
            filters.clone(),
        );
        let docking = DockingModel::new();
        docking.set_side_size(DockSide::Leading, 280.0);
        // A non-zero rail thickness switches the leading side to Rail
        // presentation (a `DockActivityBar` icon rail); the layout sizes the
        // rail from the `DockRail` config.
        docking.set_side_rail(DockSide::Leading, 48.0);
        Self::new(app_ctx, ids, model, docking, DockWidgetId::fresh(), filters)
    }

    /// Inject the model's drag-reorder closure. **Cycle-safety:** it captures
    /// only `app_ctx` + the `stack_id` signal and calls the free `apply_move`
    /// helper — NOT `self` (capturing the vm would form model → closure → vm →
    /// model, the `Rc` cycle the designer deliberately avoids).
    fn install_reorder(&self) {
        let app_ctx = self.app_ctx.clone();
        let stack_id = self.ids.stack_id.clone();
        let commit: CommitMove = Rc::new(move |dragged, target, place| {
            apply_move(&app_ctx, stack_id.get(), dragged, target, place).is_ok()
        });
        self.model.set_reorder(commit);
    }

    // ── handles for wiring the widgets / layout ──
    /// The `TreeDataSource` to hand to `TreeView::from_source_keyed`.
    pub fn model(&self) -> BinderBinderItemsTreeModel {
        self.model.clone()
    }
    pub fn selection(&self) -> KeyedSelectionModel<BinderTreeKey> {
        self.selection.clone()
    }
    pub fn selection_signal(&self) -> Signal<HashSet<BinderTreeKey>> {
        self.selection.selection_signal()
    }
    pub fn docking(&self) -> DockingModel {
        self.docking.clone()
    }
    pub fn dock_id(&self) -> DockWidgetId {
        self.dock_id
    }

    // ── binder switcher + search filters (shared with the tree model) ──
    /// Binder display scope — `None` = all binders, `Some(id)` = that binder.
    /// Bind the switcher label to it; the tree model re-sources on change.
    pub fn binder_filter_signal(&self) -> Signal<Option<u64>> {
        self.filters.binder.clone()
    }
    /// The live text-search query — hand a clone to `SearchField::new`.
    pub fn search_query_signal(&self) -> Signal<String> {
        self.filters.query.clone()
    }
    /// Search scope toggle — `false` = current binder, `true` = all binders.
    pub fn search_all_signal(&self) -> Signal<bool> {
        self.filters.all_binders.clone()
    }
    /// Switch the displayed binder (the tree model observes this and re-sources).
    pub fn set_binder_filter(&self, binder: Option<u64>) {
        self.filters.binder.set(binder);
    }
    /// The open Work id — for models that key on it (e.g. the binder list).
    pub fn work_id_signal(&self) -> Signal<Option<u64>> {
        self.ids.work_id.clone()
    }
    /// Clear the text search (e.g. on project load).
    pub fn clear_search(&self) {
        self.filters.query.set(String::new());
    }

    /// Resolve a tree key to its `(item_id, title)` (binder rows: `item_id` None).
    pub fn node_item(&self, key: BinderTreeKey) -> Option<(Option<u64>, String)> {
        self.model.node_of(&key)
    }

    // ── visibility: pure delegation to the DockingModel ──
    //
    // The outline lives on a *rail* side, so "visible" means the side panel is
    // shown (the rail itself persists) — i.e. side visibility, the same concept
    // the activity-rail's click-to-hide drives. We toggle/reflect side
    // visibility, NOT dock open/close (which would add/remove the rail icon).

    /// Show the outline panel.
    pub fn show(&self) {
        self.docking.set_side_visible(DockSide::Leading, true);
    }
    /// Hide the outline panel (the rail stays).
    pub fn hide(&self) {
        self.docking.set_side_visible(DockSide::Leading, false);
    }
    /// Flip the outline panel's visibility.
    pub fn toggle(&self) {
        let visible = self.docking.is_side_visible(DockSide::Leading);
        self.docking.set_side_visible(DockSide::Leading, !visible);
    }
    /// Reactive shown/hidden state — bind a menu check or toolbar toggle to this.
    /// Reflects the side's visibility (also flips when the rail hides the side).
    pub fn is_visible(&self) -> Signal<bool> {
        self.docking.side_visible_signal(DockSide::Leading)
    }

    /// Register the outline dock on its side (called once, at layout build).
    pub fn open_in_layout(&self) {
        self.docking
            .open_dock(self.dock_id, DockOpenLocation::side(DockSide::Leading));
    }

    /// Rebuild the tree from the backend (e.g. on project load), pruning a
    /// now-stale selection (a trashed item drops out of the tree).
    pub fn reload(&self) {
        self.model.reload();
        let model = self.model.clone();
        self.selection.prune_missing(move |k| model.contains(k));
    }

    /// Bring the outline forward AND select a row ("reveal in outline").
    pub fn reveal_item(&self, key: BinderTreeKey) {
        self.show();
        self.selection.select(key);
    }

    // ── undo stack (one per loaded Work) ──

    /// Create the per-`Work` undo stack. Call on `LoadWork`.
    pub fn init_stack(&self) {
        self.ids.open_stack(&self.app_ctx);
    }
    fn stack(&self) -> Option<u64> {
        self.ids.stack_id.get()
    }
    /// The shared undo-stack signal, handed to `EditorsViewModel` so editor
    /// write-back lands on the same Ctrl+Z history as tree edits.
    pub fn stack_id_signal(&self) -> Signal<Option<u64>> {
        self.ids.stack_id.clone()
    }

    // ── actions (each: backend command on the undo stack, then reload) ──

    /// Create a new item/folder at the insertion point derived from selection.
    /// A "folder" is just `role = Folder`; there is no separate `new_folder`.
    pub fn new_item(&self, role: BinderItemRole, sub_role: BinderItemSubRole) {
        let anchor = self.selection.selected_keys().first().copied();
        self.create_item(anchor, role, sub_role);
    }

    /// Create a new item/folder anchored at a specific row (context-menu entry —
    /// does not touch the selection, so it never opens an editor).
    pub fn new_item_at(
        &self,
        anchor: BinderTreeKey,
        role: BinderItemRole,
        sub_role: BinderItemSubRole,
    ) {
        self.create_item(Some(anchor), role, sub_role);
    }

    fn create_item(
        &self,
        anchor: Option<BinderTreeKey>,
        role: BinderItemRole,
        sub_role: BinderItemSubRole,
    ) {
        if skribisto_model::validate_item(&role, &sub_role, &[]).is_err() {
            return;
        }
        let Some((binder, index, indent)) = self.insertion_point(anchor) else {
            return;
        };
        let title = match role {
            BinderItemRole::Folder => "New Folder",
            BinderItemRole::Item => "New Item",
        };
        let dto = CreateBinderItemDto {
            title: title.to_string(),
            role,
            sub_role,
            activated: true,
            is_printable: true,
            indent,
            ..Default::default()
        };
        if binder_item_commands::create_binder_item(
            &self.app_ctx,
            self.stack(),
            &dto,
            binder,
            index as i32,
        )
        .is_ok()
        {
            self.reload();
        }
    }

    /// Begin a rename: present a modal `InputDialog`, applying `rename` on OK.
    pub fn begin_rename(&self, key: BinderTreeKey, ctx: &mut EventContext) {
        let current = self.model.node_of(&key).map(|(_, t)| t).unwrap_or_default();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_rename()))
            .default_text(current)
            .on_result(move |result, _ctx| {
                if let Some(name) = result
                    && !name.trim().is_empty()
                {
                    vm.rename(key, &name);
                }
            })
            .present(ctx);
    }

    /// Apply a rename to a binder (name) or item (title).
    pub fn rename(&self, key: BinderTreeKey, title: &str) {
        let ctx = &*self.app_ctx;
        match key {
            BinderTreeKey::Binder(b) => {
                if let Some(binder) = self.binder_dto(b) {
                    let dto = UpdateBinderDto {
                        id: b,
                        created_at: binder.created_at,
                        updated_at: binder.updated_at,
                        name: title.to_string(),
                        activated: binder.activated,
                    };
                    let _ = binder_commands::update_binder(ctx, self.stack(), &dto);
                }
            }
            BinderTreeKey::Item(i) => {
                if let Some(it) = self.item_dto(i) {
                    let mut dto = update_item_dto(&it);
                    dto.title = title.to_string();
                    let _ = binder_item_commands::update_binder_item(ctx, self.stack(), &dto);
                }
            }
        }
        self.reload();
    }

    /// Rename the first selected row (the `binder.rename` command entry point).
    pub fn rename_selected(&self, ctx: &mut EventContext) {
        if let Some(key) = self.selection.selected_keys().first().copied() {
            self.begin_rename(key, ctx);
        }
    }

    /// Move the selected items to trash (the `binder.trash_selected` command).
    pub fn trash_selected(&self) {
        let sel = self.selection.selected_keys();
        self.trash_keys(&sel);
    }

    /// Move the given keys to trash (binders and items both supported). Used by
    /// the context menu (operates on the right-clicked row, not the selection).
    pub fn trash_keys(&self, sel: &[BinderTreeKey]) {
        let ctx = &*self.app_ctx;
        if sel.is_empty() {
            return;
        }
        let stack = self.stack();
        let composite = sel.len() > 1;
        if composite {
            let _ = undo_redo_commands::begin_composite(ctx, stack);
        }
        // Whole binders.
        for key in sel {
            if let BinderTreeKey::Binder(b) = key {
                let _ = trash_management_commands::trash_binder(
                    ctx,
                    stack,
                    &TrashBinderDto {
                        binder_id: *b as i64,
                    },
                );
            }
        }
        // Items, grouped by their origin binder.
        let mut by_binder: HashMap<u64, Vec<i64>> = HashMap::new();
        for key in sel {
            if let BinderTreeKey::Item(i) = key
                && let Some(b) = self.model.binder_of(key)
            {
                by_binder.entry(b).or_default().push(*i as i64);
            }
        }
        for (binder, ids) in by_binder {
            let _ = trash_management_commands::trash_binder_items(
                ctx,
                stack,
                &TrashBinderItemsDto {
                    binder_item_ids: ids,
                    origin_binder_id: binder as i64,
                },
            );
        }
        if composite {
            undo_redo_commands::end_composite(ctx);
        }
        self.reload();
    }

    /// Create a new binder in the open Work, switch the switcher to it, and open
    /// the rename dialog so the user names it. Backs the popover's "New binder…".
    pub fn new_binder(&self, ctx: &mut EventContext) {
        let Some(work_id) = self.ids.work_id.get() else {
            return; // no project open
        };
        let dto = CreateBinderDto {
            name: "New Binder".to_string(),
            activated: true,
            ..Default::default()
        };
        // `-1` appends to the Work's ordered binder list; undoable on the stack.
        if let Ok(binder) =
            binder_commands::create_binder(&self.app_ctx, self.stack(), &dto, work_id, -1)
        {
            // Show the new binder (this re-sources the tree so its row exists),
            // then rename it in place — `begin_rename` reads the row's name.
            self.set_binder_filter(Some(binder.id));
            self.begin_rename(BinderTreeKey::Binder(binder.id), ctx);
        }
    }

    /// Trash a whole binder by id — the switcher context-menu action, reached via
    /// `AppIntent::TrashBinder` → the `binder.trash` global action. If it was the
    /// displayed binder, revert to "all binders" so the tree isn't left filtered
    /// to a now-trashed binder.
    pub fn trash_binder(&self, id: u64) {
        self.trash_keys(&[BinderTreeKey::Binder(id)]);
        if self.filters.binder.get() == Some(id) {
            self.set_binder_filter(None);
        }
    }

    /// Duplicate the selected items (the `binder.duplicate` command).
    pub fn duplicate_selected(&self) {
        let sel = self.selection.selected_keys();
        self.duplicate_keys(&sel);
    }

    /// Duplicate the given keys' item subtrees (binder keys ignored). Used by the
    /// context menu (operates on the right-clicked row, not the selection).
    pub fn duplicate_keys(&self, keys: &[BinderTreeKey]) {
        let item_ids: Vec<u64> = keys
            .iter()
            .filter_map(|k| match k {
                BinderTreeKey::Item(i) => Some(*i),
                _ => None,
            })
            .collect();
        if item_ids.is_empty() {
            return;
        }
        let _ = binder_item_management_commands::duplicate(
            &self.app_ctx,
            self.stack(),
            &DuplicateDto { item_ids },
        );
        self.reload();
    }

    /// Move one dragged item relative to a target (drag-drop and keyboard moves
    /// share the same `apply_move` helper).
    pub fn move_item(&self, dragged: BinderTreeKey, target: BinderTreeKey, place: DropPosition) {
        if apply_move(&self.app_ctx, self.stack(), dragged, target, place).is_ok() {
            self.reload();
        }
    }

    pub fn indent_selected(&self) {
        self.reindent(1);
    }
    pub fn outdent_selected(&self) {
        self.reindent(-1);
    }

    // ── private helpers ──

    /// `(binder, insert_index, indent)` for a new item, relative to `anchor`.
    fn insertion_point(&self, anchor: Option<BinderTreeKey>) -> Option<(u64, usize, i64)> {
        let ctx = &*self.app_ctx;
        match anchor {
            Some(BinderTreeKey::Binder(b)) => Some((b, 0, 0)),
            Some(BinderTreeKey::Item(i)) => {
                let binder = self.model.binder_of(&BinderTreeKey::Item(i))?;
                let order = binder_commands::get_binder_relationship(
                    ctx,
                    &binder,
                    &BinderRelationshipField::BinderItems,
                )
                .ok()?;
                let pos = order.iter().position(|&x| x == i)?;
                let it = self.item_dto(i)?;
                let indent = if it.role == BinderItemRole::Folder {
                    it.indent + 1 // first child of the folder
                } else {
                    it.indent // following sibling
                };
                Some((binder, pos + 1, indent))
            }
            None => Some((self.first_binder()?, 0, 0)),
        }
    }

    fn first_binder(&self) -> Option<u64> {
        let ctx = &*self.app_ctx;
        // ids-only global state: the open Work's id is known; no `get_all_work`.
        let work_id = self.ids.work_id.get()?;
        work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
            .ok()?
            .into_iter()
            .next()
    }

    fn reindent(&self, delta: i64) {
        let ctx = &*self.app_ctx;
        let items: Vec<u64> = self
            .selection
            .selected_keys()
            .iter()
            .filter_map(|k| match k {
                BinderTreeKey::Item(i) => Some(*i),
                _ => None,
            })
            .collect();
        if items.is_empty() {
            return;
        }
        let stack = self.stack();
        let composite = items.len() > 1;
        if composite {
            let _ = undo_redo_commands::begin_composite(ctx, stack);
        }
        for i in items {
            let Some(binder) = self.model.binder_of(&BinderTreeKey::Item(i)) else {
                continue;
            };
            let Ok(order) = binder_commands::get_binder_relationship(
                ctx,
                &binder,
                &BinderRelationshipField::BinderItems,
            ) else {
                continue;
            };
            let Some(pos) = order.iter().position(|&x| x == i) else {
                continue;
            };
            let Some(it) = self.item_dto(i) else {
                continue;
            };
            let new_indent = if delta > 0 {
                // Indent ≤ predecessor sibling's indent + 1 (can't indent the
                // first child of a level).
                let pred = if pos == 0 {
                    -1
                } else {
                    self.item_dto(order[pos - 1])
                        .map(|p| p.indent)
                        .unwrap_or(-1)
                };
                (it.indent + 1).min(pred + 1).max(0)
            } else {
                (it.indent - 1).max(0)
            };
            if new_indent != it.indent {
                let mut dto = update_item_dto(&it);
                dto.indent = new_indent;
                let _ = binder_item_commands::update_binder_item(ctx, stack, &dto);
            }
        }
        if composite {
            undo_redo_commands::end_composite(ctx);
        }
        self.reload();
    }
}

/// Build a scalar-only `UpdateBinderItemDto` from a fetched item.
fn update_item_dto(it: &frontend::direct_access::BinderItemDto) -> UpdateBinderItemDto {
    UpdateBinderItemDto {
        id: it.id,
        created_at: it.created_at,
        updated_at: it.updated_at,
        title: it.title.clone(),
        sub_title: it.sub_title.clone(),
        role: it.role.clone(),
        sub_role: it.sub_role.clone(),
        label: it.label.clone(),
        activated: it.activated,
        is_favorite: it.is_favorite,
        is_printable: it.is_printable,
        indent: it.indent,
        word_count_goal: it.word_count_goal,
        char_count_goal: it.char_count_goal,
        dict_language: it.dict_language.clone(),
    }
}

/// Apply a single-item move through the backend (undoable). Free function so the
/// model's reorder closure can call it without capturing the view-model (which
/// would create an `Rc` cycle model → closure → vm → model).
pub(crate) fn apply_move(
    ctx: &AppContext,
    stack: Option<u64>,
    dragged: BinderTreeKey,
    target: BinderTreeKey,
    place: DropPosition,
) -> anyhow::Result<()> {
    let item_id = match dragged {
        BinderTreeKey::Item(i) => i,
        BinderTreeKey::Binder(_) => anyhow::bail!("only items can be moved"),
    };
    let (target_id, target_is_binder) = match target {
        BinderTreeKey::Binder(b) => (b, true),
        BinderTreeKey::Item(i) => (i, false),
    };
    let move_place = match place {
        DropPosition::Before => MovePlace::Before,
        DropPosition::After => MovePlace::After,
        DropPosition::Into => MovePlace::Into,
    };
    binder_item_management_commands::move_items(
        ctx,
        stack,
        &MoveDto {
            item_ids: vec![item_id],
            target_id: Some(target_id),
            target_is_binder,
            move_place,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "mocks")]
    use bastyde::data::TreeDataSource; // brings `visible_count` into scope

    #[test]
    fn outline_init_stack_creates_a_stack_id() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        assert!(outline.stack_id_signal().get().is_none());
        outline.init_stack();
        assert!(
            outline.stack_id_signal().get().is_some(),
            "init_stack opens the per-Work undo stack"
        );
    }

    #[test]
    fn outline_actions_on_empty_selection_are_noops() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        // No selection, no loaded Work — these must not panic and must do nothing.
        outline.trash_selected();
        outline.duplicate_selected();
        outline.indent_selected();
        outline.outdent_selected();
    }

    #[test]
    fn outline_show_hide_toggle_track_side_visibility() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        let visible = outline.is_visible();

        // Sides start hidden.
        assert!(!visible.get());

        outline.show();
        assert!(visible.get(), "show() makes the side visible");

        outline.toggle();
        assert!(!visible.get(), "toggle() hides a visible side");

        outline.toggle();
        assert!(visible.get(), "toggle() re-shows a hidden side");

        outline.hide();
        assert!(!visible.get(), "hide() hides the side");
    }

    // The mock tree has content (2 binders, 7 items) so these assert the
    // switcher/search signals drive the model's re-source end-to-end.
    #[cfg(feature = "mocks")]
    #[test]
    fn set_binder_filter_scopes_the_tree() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        let model = outline.model();
        assert_eq!(model.visible_count(), 9); // all binders
        outline.set_binder_filter(Some(1));
        assert_eq!(model.visible_count(), 6); // Manuscript = binder + 5 items
        outline.set_binder_filter(None);
        assert_eq!(model.visible_count(), 9);
    }

    #[cfg(feature = "mocks")]
    #[test]
    fn search_query_filters_the_tree() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        let model = outline.model();
        outline.search_query_signal().set("dawn".to_string());
        assert_eq!(model.visible_count(), 3); // Manuscript > Book One > Scene at dawn
        outline.clear_search();
        assert_eq!(model.visible_count(), 9);
    }
}
