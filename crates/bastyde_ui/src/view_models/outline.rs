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
use bastyde::widgets::{DockOpenLocation, DockSide, DockWidgetId, DockingModel, InputDialog};

use frontend::AppContext;
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands, content_commands,
    trash_management_commands, undo_redo_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::{
    CreateBinderDto, CreateBinderItemDto, UpdateBinderDto, UpdateBinderItemDto,
};

use frontend::binder_item_management::{DuplicateDto, MoveDto, MovePlace, PromoteDto};
use frontend::trash_management::{TrashBinderDto, TrashBinderItemsDto};

use skribisto_model::{PromoteTarget, Recommendation, Relation, SubRoleExt};

use crate::app_ids::AppIds;
use crate::binder_placement;
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
        let model =
            BinderBinderItemsTreeModel::new(app_ctx.clone(), ids.work_id.clone(), filters.clone());
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

    /// The first selected **item** row's `(item_id, title)`, if the selection is
    /// an item (not a binder root). Backs the outline's "Open to the Side"
    /// keyboard shortcut.
    pub fn selected_item(&self) -> Option<(u64, String)> {
        let key = self.selection.selected_keys().first().copied()?;
        match self.node_item(key)? {
            (Some(item_id), title) => Some((item_id, title)),
            _ => None,
        }
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
    /// The app's id-only global state (shared by clone), handed to
    /// `EditorsViewModel` so its tabs (esp. the manuscript streams) can reach
    /// `work_id`/`stack_id`.
    pub fn ids(&self) -> AppIds {
        self.ids.clone()
    }

    /// Subscribe the tree to backend structural events so it re-sources on any
    /// mutation, whoever caused it (a manuscript stream, import, etc.) — not
    /// just the outline's own commands. Call once from `App::build`.
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.model.wire(ctx);
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
        self.create_item_at(binder, index, indent, role, sub_role);
    }

    /// The shared create tail: build the DTO and run the undoable create command,
    /// then reload. Both `create_item` (legacy anchor logic) and `add_recommended`
    /// (relation-aware) funnel through this so the DTO/undo/reload stay in one place.
    fn create_item_at(
        &self,
        binder: u64,
        index: usize,
        indent: i64,
        role: BinderItemRole,
        sub_role: BinderItemSubRole,
    ) {
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

    // ── context-dependent "create" recommendations ──

    /// The ordered `Recommendation`s for the current selection's first key — what
    /// the header "Create" SplitButton offers (default first). Reads live data,
    /// so it must be recomputed whenever `selection_signal()` changes.
    pub fn recommendations_for_selection(&self) -> Vec<Recommendation> {
        self.recommendations_for_key(self.selection.selected_keys().first().copied())
    }

    /// The ordered `Recommendation`s for a specific anchor row (the per-item
    /// "Add ▸" submenu). A `Binder`/absent/stale anchor falls back to the
    /// top-level set (Book first). Applies the live BookEnd gating.
    pub fn recommendations_for_key(&self, anchor: Option<BinderTreeKey>) -> Vec<Recommendation> {
        match anchor {
            Some(BinderTreeKey::Item(i)) => match self.item_dto(i) {
                Some(dto) => {
                    let mut recs = skribisto_model::recommendations(&dto.role, &dto.sub_role);
                    self.gate_book_end(i, &mut recs);
                    recs
                }
                None => skribisto_model::recommendations_root(),
            },
            _ => skribisto_model::recommendations_root(),
        }
    }

    /// Create the recommended item, relation-aware. `anchor = None` re-reads the
    /// current selection (header button); `Some(key)` anchors explicitly (context
    /// menu). The logical `CreateType` is resolved to a concrete `(role, sub_role)`
    /// via the project's chapter mode. Guarded by `validate_item`.
    pub fn add_recommended(&self, anchor: Option<BinderTreeKey>, rec: &Recommendation) {
        let anchor = anchor.or_else(|| self.selection.selected_keys().first().copied());
        let (role, sub_role) = rec.create_type.combo(self.chapter_mode());
        if skribisto_model::validate_item(&role, &sub_role, &[]).is_err() {
            return;
        }
        let Some((binder, index, indent)) = self.insertion_point_for(anchor, rec.relation) else {
            return;
        };
        self.create_item_at(binder, index, indent, role, sub_role);
    }

    // ── promote / demote (convert a binder item to its paired type) ──

    /// The paired promote target for a row, if any — `(role, sub_role)` of the
    /// type this item would become (see `skribisto_model::promote_target`).
    /// The backend context — for tests that assert on entities directly.
    #[cfg(test)]
    pub(crate) fn app_ctx(&self) -> &AppContext {
        &self.app_ctx
    }

    /// Every type `key` may be converted to, in menu order. Empty when it has none.
    pub fn promote_targets_of(&self, key: BinderTreeKey) -> Vec<PromoteTarget> {
        let BinderTreeKey::Item(item_id) = key else {
            return Vec::new();
        };
        let Some(dto) = self.item_dto(item_id) else {
            return Vec::new();
        };
        skribisto_model::promote_targets(&dto.role, &dto.sub_role)
    }

    /// The content roles whose text `key` would **lose** by becoming `target` (empty
    /// rows never count). Non-empty means the conversion is refused: a chapter holding
    /// prose cannot become a Part, which has nowhere to put it.
    pub fn promote_content_loss(
        &self,
        key: BinderTreeKey,
        target: PromoteTarget,
    ) -> Vec<ContentRole> {
        let BinderTreeKey::Item(item_id) = key else {
            return Vec::new();
        };
        let (target_role, target_sub_role) = target.combo();
        let content_ids = binder_item_commands::get_binder_item_relationship(
            &self.app_ctx,
            &item_id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        let non_empty: Vec<ContentRole> =
            content_commands::get_content_multi(&self.app_ctx, &content_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .filter(|c| !c.data.trim().is_empty())
                .map(|c| c.role)
                .collect();
        skribisto_model::promote_content_loss(&target_role, &target_sub_role, &non_empty)
    }

    /// The number of child items that block converting `key` into `target` — non-zero
    /// only when a container would become a leaf (a chapter folder → a flat chapter)
    /// while it still holds items. The caller shows a "move or trash them first" prompt.
    pub fn demote_blocked_children(&self, key: BinderTreeKey, target: PromoteTarget) -> usize {
        let BinderTreeKey::Item(item_id) = key else {
            return 0;
        };
        let Some(dto) = self.item_dto(item_id) else {
            return 0;
        };
        // Only a container → leaf conversion is gated on emptiness.
        if !(dto.role == BinderItemRole::Folder && target.combo().0 == BinderItemRole::Item) {
            return 0;
        }
        let Some(binder) = self.model.binder_of(&key) else {
            return 0;
        };
        let (order, meta) = self.ordered_meta(binder);
        let Some(pos) = order.iter().position(|&x| x == item_id) else {
            return 0;
        };
        binder_placement::subtree_end(&order, &meta, pos, dto.indent) - (pos + 1)
    }

    /// Convert a binder item to `target` (undoable). The use case re-validates the
    /// target against the item's current type and refuses a conversion that would drop
    /// text, so this is safe to call even from a stale menu. The demote-empty guard is
    /// the caller's job (`demote_blocked_children`); this trusts it.
    pub fn promote(&self, key: BinderTreeKey, target: PromoteTarget) {
        let BinderTreeKey::Item(item_id) = key else {
            return;
        };
        let dto = PromoteDto {
            item_id,
            target: target.code(),
        };
        if binder_item_management_commands::promote(&self.app_ctx, self.stack(), &dto).is_ok() {
            self.reload();
        }
    }

    /// The open project's chapter storage mode — how a `CreateType::Chapter` is
    /// encoded — read from the open Work's `chapter_mode` field (defaults to
    /// folder mode when no Work is open).
    fn chapter_mode(&self) -> skribisto_model::ChapterMode {
        self.ids
            .work_id
            .get()
            .and_then(|id| work_commands::get_work(&self.app_ctx, &id).ok().flatten())
            .map(|w| w.chapter_mode)
            .unwrap_or_default()
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
                // Through the single, so the title `Content` row follows the entity field.
                // A tree rename that wrote only `BinderItem.title` would leave the
                // manuscript compiling the old chapter title — the mirror image of the
                // editor-rename bug.
                let item = SingleBinderItem::new(self.app_ctx.clone());
                item.set_id(Some(i));
                let _ = item.set_title(title, self.stack());
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

    /// `(binder, insert_index, indent)` for a new item placed by `relation`
    /// relative to `anchor` — the relation-aware generalisation of
    /// [`insertion_point`](Self::insertion_point) (which it leaves untouched).
    ///
    /// - `Sibling` lands after the anchor's *entire subtree* at the anchor's own
    ///   indent (so a sibling of a populated folder follows its children).
    /// - `Child` appends inside a folder anchor (indent + 1), but before any direct
    ///   child that `closes_book()` (keeps a Book's `BookEnd` last).
    /// - `ParentSibling` walks up to the nearest ancestor that opens a chapter/book
    ///   and behaves as `Sibling` of it.
    fn insertion_point_for(
        &self,
        anchor: Option<BinderTreeKey>,
        relation: Relation,
    ) -> Option<(u64, usize, i64)> {
        match anchor {
            Some(BinderTreeKey::Binder(b)) => match relation {
                Relation::Child => Some((b, 0, 0)),
                _ => Some((b, self.ordered_meta(b).0.len(), 0)),
            },
            Some(BinderTreeKey::Item(i)) => {
                let binder = self.model.binder_of(&BinderTreeKey::Item(i))?;
                let (order, meta) = self.ordered_meta(binder);
                let pos = order.iter().position(|&x| x == i)?;
                let (anchor_indent, _) = *meta.get(&i)?;
                // Shared with `StreamViewModel` — see `crate::binder_placement`.
                let (index, indent) = binder_placement::insertion_point_for_item(
                    &order,
                    &meta,
                    pos,
                    anchor_indent,
                    relation,
                );
                Some((binder, index, indent))
            }
            None => {
                let b = self.first_binder()?;
                Some((b, self.ordered_meta(b).0.len(), 0))
            }
        }
    }

    /// The binder's ordered item ids plus `{id -> (indent, sub_role)}`, in one
    /// batch fetch — the data `insertion_point_for` / `gate_book_end` walk.
    fn ordered_meta(&self, binder: u64) -> (Vec<u64>, HashMap<u64, (i64, BinderItemSubRole)>) {
        let ctx = &*self.app_ctx;
        let order = binder_commands::get_binder_relationship(
            ctx,
            &binder,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default();
        let meta = binder_item_commands::get_binder_item_multi(ctx, &order)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .map(|it| (it.id, (it.indent, it.sub_role)))
            .collect();
        (order, meta)
    }

    /// `(position, indent)` of the book enclosing `order[pos]` — the anchor itself
    /// if it opens a book, else the nearest book ancestor (walking *past* any
    /// intermediate chapters). `None` if the anchor is not inside a book.
    fn enclosing_book(
        order: &[u64],
        meta: &HashMap<u64, (i64, BinderItemSubRole)>,
        pos: usize,
    ) -> Option<(usize, i64)> {
        let (ind0, sr0) = meta.get(&order[pos])?;
        let mut cur_indent = *ind0;
        if sr0.opens_book() {
            return Some((pos, cur_indent));
        }
        let mut cur = pos;
        while cur > 0 {
            cur -= 1;
            let (ind, sr) = meta.get(&order[cur])?;
            if *ind < cur_indent {
                cur_indent = *ind;
                if sr.opens_book() {
                    return Some((cur, *ind));
                }
            }
        }
        None
    }

    /// Drop any `closes_book()` recommendation (End of Book) when the anchor's
    /// enclosing book already contains one — a book has exactly one end. This is
    /// the live-data gating the pure `recommendations()` table can't do itself.
    fn gate_book_end(&self, anchor_item: u64, recs: &mut Vec<Recommendation>) {
        if !recs.iter().any(|r| r.create_type.closes_book()) {
            return;
        }
        let Some(binder) = self.model.binder_of(&BinderTreeKey::Item(anchor_item)) else {
            return;
        };
        let (order, meta) = self.ordered_meta(binder);
        let Some(pos) = order.iter().position(|&x| x == anchor_item) else {
            return;
        };
        let Some((book_pos, book_indent)) = Self::enclosing_book(&order, &meta, pos) else {
            return;
        };
        let end = binder_placement::subtree_end(&order, &meta, book_pos, book_indent);
        let has_end = order[book_pos..end]
            .iter()
            .any(|id| meta.get(id).is_some_and(|(_, sr)| sr.closes_book()));
        if has_end {
            recs.retain(|r| !r.create_type.closes_book());
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

    // The mock tree has content (2 binders, 13 items) so these assert the
    // switcher/search signals drive the model's re-source end-to-end.
    #[cfg(feature = "mocks")]
    #[test]
    fn set_binder_filter_scopes_the_tree() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        let model = outline.model();
        assert_eq!(model.visible_count(), 15); // all binders
        outline.set_binder_filter(Some(1));
        assert_eq!(model.visible_count(), 12); // Manuscript = binder + 11 items
        outline.set_binder_filter(None);
        assert_eq!(model.visible_count(), 15);
    }

    #[cfg(feature = "mocks")]
    #[test]
    fn search_query_filters_the_tree() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
        let model = outline.model();
        outline.search_query_signal().set("dawn".to_string());
        assert_eq!(model.visible_count(), 3); // Manuscript > Book One > Scene at dawn
        outline.clear_search();
        assert_eq!(model.visible_count(), 15);
    }

    // ── relation-aware creation (real backend: seed a Work/Binder/items and
    // assert where `add_recommended` lands). Gated off `mocks`, whose tree model
    // ignores `work_id` and re-sources a static fixture instead. ──
    #[cfg(not(feature = "mocks"))]
    mod recommend {
        use super::*;

        /// Seed an empty Work + Binder; return a VM wired to it (reloaded) and the
        /// binder id.
        fn seed() -> (OutlineViewModel, u64) {
            let app_ctx = Rc::new(AppContext::new());
            let work = work_commands::create_orphan_work(
                &app_ctx,
                None,
                &frontend::direct_access::CreateWorkDto::default(),
            )
            .unwrap();
            let binder = binder_commands::create_binder(
                &app_ctx,
                None,
                &CreateBinderDto {
                    name: "B".into(),
                    activated: true,
                    ..Default::default()
                },
                work.id,
                0,
            )
            .unwrap();
            let ids = AppIds::default();
            ids.work_id.set(Some(work.id));
            let outline = OutlineViewModel::new_default(app_ctx, ids);
            outline.reload();
            (outline, binder.id)
        }

        /// Append an item to `binder` at `index` (sequential = append) and reload.
        fn seed_item(
            outline: &OutlineViewModel,
            binder: u64,
            role: BinderItemRole,
            sub_role: BinderItemSubRole,
            indent: i64,
            index: i32,
        ) -> u64 {
            let dto = CreateBinderItemDto {
                title: format!("{role:?}/{sub_role:?}"),
                role,
                sub_role,
                activated: true,
                is_printable: true,
                indent,
                ..Default::default()
            };
            let id = binder_item_commands::create_binder_item(
                &outline.app_ctx,
                None,
                &dto,
                binder,
                index,
            )
            .unwrap()
            .id;
            outline.reload();
            id
        }

        fn order_of(outline: &OutlineViewModel, binder: u64) -> Vec<u64> {
            binder_commands::get_binder_relationship(
                &outline.app_ctx,
                &binder,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap()
        }

        use skribisto_model::CreateType;

        fn rec(create_type: CreateType, relation: Relation) -> Recommendation {
            Recommendation {
                create_type,
                relation,
            }
        }

        #[test]
        fn chapter_creates_a_folder_chapter_in_folder_mode() {
            let (outline, binder) = seed();
            let book = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::Book,
                0,
                0,
            );
            outline.add_recommended(
                Some(BinderTreeKey::Item(book)),
                &rec(CreateType::Chapter, Relation::Child),
            );
            let new = *order_of(&outline, binder).last().unwrap();
            let dto = outline.item_dto(new).unwrap();
            // Default (folder) mode → a Chapter is a Folder/ChapterScene.
            assert_eq!(dto.role, BinderItemRole::Folder);
            assert_eq!(dto.sub_role, BinderItemSubRole::ChapterScene);
        }

        #[test]
        fn child_appends_inside_a_book_before_its_book_end() {
            let (outline, binder) = seed();
            let book = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::Book,
                0,
                0,
            );
            let ch1 = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                1,
                1,
            );
            let end = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::BookEnd,
                1,
                2,
            );

            outline.add_recommended(
                Some(BinderTreeKey::Item(book)),
                &rec(CreateType::Chapter, Relation::Child),
            );

            let order = order_of(&outline, binder);
            assert_eq!(order.len(), 4);
            let new = order[2];
            // New chapter lands after the existing chapter but *before* BookEnd.
            assert_eq!(order, vec![book, ch1, new, end]);
            // …and at the book's child indent.
            assert_eq!(outline.item_dto(new).unwrap().indent, 1);
        }

        #[test]
        fn sibling_lands_after_the_anchor_folders_whole_subtree() {
            let (outline, binder) = seed();
            let chapter = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                0,
                0,
            );
            let s1 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
                1,
            );
            let s2 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
                2,
            );

            outline.add_recommended(
                Some(BinderTreeKey::Item(chapter)),
                &rec(CreateType::Chapter, Relation::Sibling),
            );

            let order = order_of(&outline, binder);
            let new = *order.last().unwrap();
            // After both scenes (not nested between the chapter and its children).
            assert_eq!(order, vec![chapter, s1, s2, new]);
            assert_eq!(outline.item_dto(new).unwrap().indent, 0);
        }

        #[test]
        fn parent_sibling_targets_the_enclosing_chapters_level() {
            let (outline, binder) = seed();
            let book = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::Book,
                0,
                0,
            );
            let chapter = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                1,
                1,
            );
            let s1 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                2,
                2,
            );
            let s2 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                2,
                3,
            );

            // Anchored on a deep scene: a new Chapter should start after the whole
            // enclosing chapter, at the chapter's own indent — not nested in it.
            outline.add_recommended(
                Some(BinderTreeKey::Item(s1)),
                &rec(CreateType::Chapter, Relation::ParentSibling),
            );

            let order = order_of(&outline, binder);
            let new = *order.last().unwrap();
            assert_eq!(order, vec![book, chapter, s1, s2, new]);
            assert_eq!(outline.item_dto(new).unwrap().indent, 1);
        }

        #[test]
        fn book_end_recommendation_is_gated_once_the_book_has_one() {
            let (outline, binder) = seed();
            let book = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::Book,
                0,
                0,
            );
            let _end = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::BookEnd,
                1,
                1,
            );

            let recs = outline.recommendations_for_key(Some(BinderTreeKey::Item(book)));
            assert!(
                recs.iter().all(|r| r.create_type != CreateType::EndOfBook),
                "End of Book should be hidden when the book already has one"
            );
        }

        #[test]
        fn binder_and_empty_selection_use_the_root_recommendations() {
            let (outline, binder) = seed();
            let expected = skribisto_model::recommendations_root();
            assert_eq!(
                outline.recommendations_for_key(Some(BinderTreeKey::Binder(binder))),
                expected
            );
            assert_eq!(outline.recommendations_for_key(None), expected);
        }

        #[test]
        fn promote_flips_scene_to_note() {
            let (outline, binder) = seed();
            let scene = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                0,
                0,
            );
            outline.promote(BinderTreeKey::Item(scene), PromoteTarget::Note);
            let dto = outline.item_dto(scene).unwrap();
            assert_eq!(dto.role, BinderItemRole::Item);
            assert_eq!(dto.sub_role, BinderItemSubRole::Note);
        }

        #[test]
        fn promote_flips_chapterscene_to_chapter_folder() {
            let (outline, binder) = seed();
            let cs = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::ChapterScene,
                0,
                0,
            );
            outline.promote(BinderTreeKey::Item(cs), PromoteTarget::ChapterFolder);
            let dto = outline.item_dto(cs).unwrap();
            assert_eq!(dto.role, BinderItemRole::Folder);
            assert_eq!(dto.sub_role, BinderItemSubRole::ChapterScene);
        }

        #[test]
        fn demote_blocked_children_counts_the_subtree() {
            let (outline, binder) = seed();
            let chapter = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                0,
                0,
            );
            let s1 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
                1,
            );
            let _s2 = seed_item(
                &outline,
                binder,
                BinderItemRole::Item,
                BinderItemSubRole::Scene,
                1,
                2,
            );
            // The chapter folder holds two scenes, so collapsing it into a flat chapter
            // is blocked...
            assert_eq!(
                outline.demote_blocked_children(
                    BinderTreeKey::Item(chapter),
                    PromoteTarget::FlatChapter
                ),
                2
            );
            // ...but becoming another *folder* is not: nothing is being collapsed.
            assert_eq!(
                outline.demote_blocked_children(
                    BinderTreeKey::Item(chapter),
                    PromoteTarget::PartFolder
                ),
                0
            );
            // A leaf scene has no container to empty.
            assert_eq!(
                outline.demote_blocked_children(BinderTreeKey::Item(s1), PromoteTarget::Note),
                0
            );
        }

        /// An item's name has two homes: `BinderItem.title`, which the outline tree and
        /// the tab show, and a title `Content` row, which compiles into the manuscript.
        /// **Renaming through either door must reach both.**
        ///
        /// Renaming a chapter in the tree used to write only the entity field, leaving the
        /// manuscript compiling the old title; renaming it in its editor wrote only the
        /// content row, leaving the tree and the tab showing the old name. Both now go
        /// through `SingleBinderItem::set_title`.
        #[test]
        fn a_rename_reaches_both_homes_of_the_title() {
            let (outline, binder) = seed();
            let ch = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                0,
                0,
            );

            outline.rename(BinderTreeKey::Item(ch), "The Long Road");

            // The entity field the tree and the tab read...
            assert_eq!(outline.item_dto(ch).unwrap().title, "The Long Road");
            // ...and the content row the manuscript compiles.
            let content_ids = binder_item_commands::get_binder_item_relationship(
                outline.app_ctx(),
                &ch,
                &BinderItemRelationshipField::Contents,
            )
            .unwrap();
            let chapter_title =
                content_commands::get_content_multi(outline.app_ctx(), &content_ids)
                    .unwrap()
                    .into_iter()
                    .flatten()
                    .find(|c| c.role == ContentRole::ChapterTitle)
                    .map(|c| c.data);
            assert_eq!(
                chapter_title.as_deref(),
                Some("The Long Road"),
                "a tree rename must reach the title the manuscript compiles"
            );
        }

        /// The headline of this feature: outline in bare folders, then declare what each
        /// one is. A plain folder carries only a synopsis, which every folder type allows,
        /// so every one of these conversions is lossless.
        #[test]
        fn a_plain_folder_becomes_any_other_kind_of_folder() {
            for (target, want) in [
                (
                    PromoteTarget::ChapterFolder,
                    BinderItemSubRole::ChapterScene,
                ),
                (PromoteTarget::PartFolder, BinderItemSubRole::Part),
                (PromoteTarget::BookFolder, BinderItemSubRole::Book),
                (PromoteTarget::NoteFolder, BinderItemSubRole::Note),
            ] {
                let (outline, binder) = seed();
                let f = seed_item(
                    &outline,
                    binder,
                    BinderItemRole::Folder,
                    BinderItemSubRole::None,
                    0,
                    0,
                );
                assert!(
                    outline
                        .promote_targets_of(BinderTreeKey::Item(f))
                        .contains(&target),
                    "a plain folder must offer {target:?}"
                );
                outline.promote(BinderTreeKey::Item(f), target);
                let dto = outline.item_dto(f).unwrap();
                assert_eq!(dto.role, BinderItemRole::Folder);
                assert_eq!(dto.sub_role, want, "promoting to {target:?}");
            }
        }
    }
}
