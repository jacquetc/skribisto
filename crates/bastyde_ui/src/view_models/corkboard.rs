// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `CorkboardViewModel` — the business logic behind one container tab's Corkboard
//! segment. One per tab, built alongside `stream`/`pace` in `ContentTab::new` and
//! gated on the same [`StreamLevel::for_container`](crate::models::StreamLevel).
//!
//! It owns the drilled-into `current_container` (a folder card drills in *in
//! place*, deepening the breadcrumb — no new tab is opened, which sidesteps the
//! absence of an "open a tab at a segment" API), the free search/sort state, and
//! the [`CorkboardCardsModel`] + its [`SortFilterListModel`] projection. Cross-
//! view-model talk stays a DAG: it never imports a peer view-model — it drills in
//! place, or fires an [`AppIntent`] on the bus (open an item, create an item).
//!
//! Plain Rust → headless-testable.

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::core::ObserverHandle;
use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::data::{SelectionMode, SelectionModel, SortDirection, SortFilterListModel};
use bastyde::prelude::*; // Signal, BuildContext, EventContext
use bastyde::text_document::{MoveMode, TextDocument};
use bastyde::widgets::InputDialog;

use frontend::AppContext;
use frontend::binder_item_management::{MergeTwoScenesDto, MoveDto, MovePlace, SplitSceneDto};
use frontend::commands::{
    binder_commands, binder_item_commands, binder_item_management_commands,
    trash_management_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::direct_access::{BinderItemDto, CreateBinderItemDto, UpdateBinderItemDto};
use frontend::trash_management::TrashBinderItemsDto;

use skribisto_model::SubRoleExt;
use skribisto_model::counting::CountingMethodSetting;
use skribisto_model::{CreateType, Recommendation, Relation};

use crate::app_ids::AppIds;
use crate::binder_placement::{self, ItemMeta};
use crate::intents::AppIntent;
use crate::models::{CorkboardCard, CorkboardCardsModel, OpenDoc, OpenDocsStore};
use crate::singles::SingleBinderItem;
use crate::view_models::EditorTypography;

/// The sort column ids bound into the projection (see [`CorkboardCardsModel::projection`]).
pub const SORT_TITLE: &str = "title";

struct Inner {
    /// The container currently shown (the tab's own container at the trail root,
    /// or a folder drilled into). Shared with the model, which re-queries on change.
    current_container: Signal<u64>,
    /// Ancestor path `(id, title)` for the breadcrumb, root-inclusive. The last
    /// entry is the current container; its title is kept fresh from `container_probe`.
    trail: Signal<Vec<(u64, String)>>,
    /// Live text filter → the projection's "text" column.
    query: Signal<String>,
    /// Active sort, or `None` for manuscript order.
    sort: Signal<Option<(String, SortDirection)>>,
    /// `!query.is_empty() || sort.is_some()` — the grid binds the projection (and
    /// disables reorder) while this is true. A real signal (not a derived one) so
    /// the grid wrapper can bind it at `BindingLevel::Rebuild`.
    projecting: Signal<bool>,
    /// Visible card count, for the header.
    count: Signal<usize>,

    // Threaded settings.
    nested: Signal<bool>,
    card_size: Signal<f32>,
    show_word_count: Signal<bool>,
    counting_method: Signal<CountingMethodSetting>,

    // Per-tab, transient (not persisted).
    selection: SelectionModel,
    scroll_y: Signal<f32>,
    /// The card whose title is being edited inline (`None` = nobody). One at a
    /// time; a card's title renders as an editable field iff its id matches.
    editing_item: Signal<Option<u64>>,
    /// The `GridView`'s widget id, published by the grid on build. The inline
    /// title editor returns focus here when it commits/cancels, so a screen
    /// reader (and keyboard nav) lands back on the card instead of the window.
    grid_id: Signal<Option<WidgetId>>,
    /// The card whose *synopsis* is being edited in place (`None` = none). One at
    /// a time; the matching card swaps its read-only viewer for a live editor.
    editing_synopsis: Signal<Option<u64>>,
    /// The shared open document backing the currently-edited synopsis — held so
    /// its refcount stays up while editing, released (and flushed) when editing
    /// ends. The document is the **same** `OpenDoc` an editor tab would use, so a
    /// card and an open tab never diverge.
    editing_doc: RefCell<Option<(u64, Rc<OpenDoc>)>>,
    /// The app-wide open-document store (shared with the editor panes).
    docs: OpenDocsStore,
    /// Synopsis typography + the editor column width, so the card's synopsis
    /// editor renders identically to the Full-Synopsis view.
    synopsis_typo: EditorTypography,
    column_width: Signal<f32>,

    cards: CorkboardCardsModel,
    projection: SortFilterListModel<CorkboardCard>,
    /// Tracks `current_container`'s dto for the current-crumb title + the "＋ Create"
    /// recommendations (which depend on the container's `(role, sub_role)`).
    container_probe: SingleBinderItem,

    app_ctx: Rc<AppContext>,
    ids: AppIds,
    /// Keeps model/probe change-observers alive for the model's lifetime.
    observers: RefCell<Vec<ObserverHandle>>,
}

#[derive(Clone)]
pub struct CorkboardViewModel {
    inner: Rc<Inner>,
}

impl CorkboardViewModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        docs: OpenDocsStore,
        container_id: u64,
        nested: Signal<bool>,
        card_size: Signal<f32>,
        show_word_count: Signal<bool>,
        counting_method: Signal<CountingMethodSetting>,
        synopsis_typo: EditorTypography,
        column_width: Signal<f32>,
    ) -> Self {
        let current_container = Signal::new(container_id);
        let cards = CorkboardCardsModel::new(
            app_ctx.clone(),
            ids.work_id.clone(),
            ids.stack_id.clone(),
            current_container.clone(),
            nested.clone(),
        );
        let projection = cards.projection();
        let container_probe = SingleBinderItem::new(app_ctx.clone());
        container_probe.set_id(Some(container_id));

        Self {
            inner: Rc::new(Inner {
                current_container,
                trail: Signal::new(vec![(container_id, String::new())]),
                query: Signal::new(String::new()),
                sort: Signal::new(None),
                projecting: Signal::new(false),
                count: Signal::new(0),
                nested,
                card_size,
                show_word_count,
                counting_method,
                selection: SelectionModel::new(SelectionMode::Multi),
                scroll_y: Signal::new(0.0),
                editing_item: Signal::new(None),
                grid_id: Signal::new(None),
                editing_synopsis: Signal::new(None),
                editing_doc: RefCell::new(None),
                docs,
                synopsis_typo,
                column_width,
                cards,
                projection,
                container_probe,
                app_ctx,
                ids,
                observers: RefCell::new(Vec::new()),
            }),
        }
    }

    /// Subscribe the model + probe and wire the search/sort → projection plumbing.
    /// Idempotent per build (mirrors the stream pane's `WireOnBuild`).
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.inner.cards.wire(ctx);
        self.inner.container_probe.wire(ctx);

        // Search text → the projection's filter (its filters_signal is observe-only,
        // so push imperatively) + recompute `projecting`.
        {
            let me = self.clone();
            ctx.effect(&self.inner.query, move |q| {
                me.inner.projection.set_filter("text", q);
                me.recompute_projecting();
            });
        }
        // Sort selection → the projection's sort + recompute `projecting`.
        {
            let me = self.clone();
            ctx.effect(&self.inner.sort, move |s| {
                match s {
                    Some((col, dir)) => me.inner.projection.set_sort(Some(col), *dir),
                    None => me.inner.projection.clear_sort(),
                }
                me.recompute_projecting();
            });
        }
        // Keep the header count in sync with the (unfiltered) card set.
        {
            let count = self.inner.count.clone();
            let list = self.inner.cards.list();
            count.set(list.len());
            let list2 = list.clone();
            let handle = list.observe_changes(move |_| count.set(list2.len()));
            self.inner.observers.borrow_mut().push(handle);
        }
    }

    // ── Navigation ──────────────────────────────────────────────────────────

    /// Drill into a folder card: deepen the breadcrumb and re-scope the grid, all
    /// in place (same tab, same segment). Only meaningful for a container card.
    pub fn drill_into(&self, folder_id: u64, title: String) {
        let mut trail = self.inner.trail.get();
        // Freeze the level we're leaving with its *resolved* title (the probe has
        // loaded it by now) so its ancestor crumb reads correctly — `get()` on the
        // probe's derived title signal is fine (only `observe`/`effect` isn't).
        if let Some(last) = trail.last_mut() {
            let live = self.inner.container_probe.title().get();
            if !live.is_empty() {
                last.1 = live;
            }
        }
        trail.push((folder_id, title));
        self.inner.trail.set(trail);
        self.enter(folder_id);
    }

    /// Jump to an ancestor crumb (index into the trail).
    pub fn go_to_crumb(&self, index: usize) {
        let mut trail = self.inner.trail.get();
        if index + 1 >= trail.len() {
            return; // already here
        }
        trail.truncate(index + 1);
        let id = trail[index].0;
        self.inner.trail.set(trail);
        self.enter(id);
    }

    fn enter(&self, container_id: u64) {
        // Leaving the level ends any in-flight synopsis edit (flush + release).
        self.end_edit_synopsis();
        self.inner.current_container.set(container_id);
        self.inner.container_probe.set_id(Some(container_id));
        self.inner.selection.clear();
        self.inner.query.set(String::new());
        self.inner.scroll_y.set(0.0);
    }

    /// Handle a tile activation (double-click / Enter): a folder in nested mode
    /// drills in; anything else opens the item in the editor via the intent bus.
    pub fn activate(&self, ctx: &mut EventContext, card: &CorkboardCard) {
        if card.is_container && self.inner.nested.get() {
            self.drill_into(card.item_id, card.title.clone());
        } else {
            ctx.send_intent(AppIntent::OpenItem {
                item_id: card.item_id,
                title: card.title.clone(),
            });
        }
    }

    // ── Create ──────────────────────────────────────────────────────────────

    /// The "＋ Create" offers for the current container: the child types the
    /// writing model recommends here. `EndOfBook` is intentionally omitted — a
    /// structural terminator is not something to add from a card board.
    pub fn create_recommendations(&self) -> Vec<Recommendation> {
        let Some(dto) = self.inner.container_probe.dto() else {
            return Vec::new();
        };
        skribisto_model::recommendations(&dto.role, &dto.sub_role)
            .into_iter()
            .filter(|r| r.relation == Relation::Child && r.create_type != CreateType::EndOfBook)
            .collect()
    }

    /// Fire a create anchored on the *current container* (not the outline selection).
    pub fn fire_create(&self, ctx: &mut EventContext, rec: Recommendation) {
        ctx.send_intent(AppIntent::NewItem {
            create_type: rec.create_type,
            relation: rec.relation,
            anchor_item_id: Some(self.inner.current_container.get()),
        });
    }

    // ── Open into the editor panes ────────────────────────────────────────────

    /// Middle-click / "open to the side": open an item card in the *other* editor
    /// pane. Only meaningful for a leaf item — a folder card has no editor.
    pub fn open_to_side(&self, ctx: &mut EventContext, card: &CorkboardCard) {
        if card.is_container {
            return;
        }
        ctx.send_intent(AppIntent::OpenItemToSide {
            item_id: card.item_id,
            title: card.title.clone(),
        });
    }

    // ── Inline rename ─────────────────────────────────────────────────────────

    /// The id of the card whose title is being edited inline (`None` = none).
    pub fn editing_item(&self) -> Signal<Option<u64>> {
        self.inner.editing_item.clone()
    }

    /// The grid's widget id (for returning focus after an inline rename).
    pub fn grid_id(&self) -> Signal<Option<WidgetId>> {
        self.inner.grid_id.clone()
    }
    /// The grid publishes its id here on build.
    pub fn set_grid_id(&self, id: WidgetId) {
        if self.inner.grid_id.get() != Some(id) {
            self.inner.grid_id.set(Some(id));
        }
    }

    /// Begin editing a card's title in place.
    pub fn begin_rename(&self, id: u64) {
        self.inner.editing_item.set(Some(id));
    }

    /// F2 / menu on the current selection: edit the first selected card's title.
    pub fn rename_selected(&self) {
        if let Some(id) = self.first_selected_item() {
            self.begin_rename(id);
        }
    }

    /// Cancel the inline edit without writing (Esc / focus loss with no commit).
    pub fn cancel_rename(&self) {
        if self.inner.editing_item.get().is_some() {
            self.inner.editing_item.set(None);
        }
    }

    /// Commit an inline rename: write `title` to the item (entity field **and** its
    /// title `Content` row — one title, two homes) and leave edit mode. A blank or
    /// *unchanged* title writes nothing — so Enter-with-no-change and the
    /// Esc→restore→blur cancel path are both quiet no-ops (no stray undo entry).
    pub fn rename(&self, _ctx: &mut EventContext, id: u64, title: &str) {
        let title = title.trim();
        let current = self.item_dto(id).map(|d| d.title).unwrap_or_default();
        if !title.is_empty() && title != current {
            let probe = SingleBinderItem::new(self.inner.app_ctx.clone());
            probe.set_id(Some(id));
            let _ = probe.set_title(title, self.stack());
        }
        self.inner.editing_item.set(None);
    }

    // ── Inline synopsis editing (one shared document with the editor panes) ───

    /// The card whose synopsis is being edited in place (`None` = none).
    pub fn editing_synopsis(&self) -> Signal<Option<u64>> {
        self.inner.editing_synopsis.clone()
    }

    /// Enter synopsis-edit mode for `id`: open the item's **shared** `OpenDoc` (so
    /// the card and any editor tab of the same item are literally one document —
    /// edits, saving and undo all match the editor) and hold its handle. Ends any
    /// prior synopsis edit first — only one card edits at a time.
    pub fn begin_edit_synopsis(&self, id: u64) {
        if self.inner.editing_synopsis.get() == Some(id) {
            return;
        }
        self.end_edit_synopsis();
        if let Some(doc) = self.inner.docs.open(id) {
            *self.inner.editing_doc.borrow_mut() = Some((id, doc));
            self.inner.editing_synopsis.set(Some(id));
        }
    }

    /// Leave synopsis-edit mode: flush the shared document to the store and release
    /// the handle (the document survives if an editor tab still references it).
    pub fn end_edit_synopsis(&self) {
        let held = self.inner.editing_doc.borrow_mut().take();
        if let Some((id, doc)) = held {
            let _ = doc.flush(self.stack());
            self.inner.docs.release(id, self.stack());
        }
        if self.inner.editing_synopsis.get().is_some() {
            self.inner.editing_synopsis.set(None);
        }
    }

    /// The shared `OpenDoc` for the card currently editing `id` (else `None`). The
    /// view reads its synopsis `TextDocument` + `mark_dirty_fn` from it.
    pub fn synopsis_open_doc(&self, id: u64) -> Option<Rc<OpenDoc>> {
        self.inner
            .editing_doc
            .borrow()
            .as_ref()
            .and_then(|(hid, doc)| (*hid == id).then(|| doc.clone()))
    }

    /// The synopsis document to edit for `id`, if it is in edit mode and carries a
    /// synopsis field (every writing row does).
    pub fn synopsis_edit_document(&self, id: u64) -> Option<TextDocument> {
        self.synopsis_open_doc(id)
            .and_then(|doc| doc.synopsis.as_ref().map(|f| f.doc.clone()))
    }

    /// The `on_change` hook for the synopsis editor: mark the shared document dirty
    /// (bumps the store's edit counter → the save indicator + autosave notice it),
    /// exactly as the main editor's own editors do.
    pub fn synopsis_on_change(&self, id: u64) -> impl Fn() + 'static {
        let doc = self.synopsis_open_doc(id);
        move || {
            if let Some(doc) = &doc {
                (doc.mark_dirty_fn())();
            }
        }
    }

    /// Synopsis typography — so the card's editor matches the Full-Synopsis view.
    pub fn synopsis_typo(&self) -> EditorTypography {
        self.inner.synopsis_typo.clone()
    }
    /// The editor column width (shared with the main editors).
    pub fn column_width(&self) -> Signal<f32> {
        self.inner.column_width.clone()
    }

    /// Whether this card can be split — only a prose-bearing scene has two halves
    /// to cut. The backend enforces the same rule.
    pub fn can_split(&self, card: &CorkboardCard) -> bool {
        is_prose_bearing(&card.role, &card.sub_role)
    }

    /// Split the item at `caret` in its **synopsis**: the synopsis text before the
    /// caret stays on the source and the rest moves to a new scene; the prose stays
    /// whole on the source. Mirrors [`StreamViewModel::split_row`] for the synopsis
    /// flavour, over the same shared `OpenDoc`.
    pub fn split_synopsis(&self, ctx: &mut EventContext, id: u64, caret: usize) {
        let Some(doc) = self.synopsis_open_doc(id) else {
            return;
        };
        let stack = self.stack();
        let _ = doc.flush(stack);
        let Some(syn) = doc.synopsis.as_ref() else {
            return;
        };
        let Ok((before_synopsis, after_synopsis)) = split_djot(&syn.doc, caret) else {
            return;
        };
        let whole_prose = doc.main.as_ref().map(|m| m.djot()).unwrap_or_default();
        let _ = binder_item_management_commands::split_scene(
            &self.inner.app_ctx,
            stack,
            &SplitSceneDto {
                source_id: id,
                before_text: whole_prose,
                after_text: String::new(),
                before_synopsis,
                after_synopsis,
                new_title: tr!(new_scene_title()).resolve_now(),
            },
        );
        // The source now holds only the before-halves — reflect that in the shared
        // doc, pumping a frame so the queued document events drain.
        doc.reload();
        ctx.request_frame();
    }

    /// The first selected card's item id (selection is index-keyed against the raw
    /// card order), or `None` when nothing is selected.
    fn first_selected_item(&self) -> Option<u64> {
        let cards = self.ordered_cards();
        self.inner
            .selection
            .selected_indices()
            .into_iter()
            .min()
            .and_then(|i| cards.get(i).map(|c| c.item_id))
    }

    // ── Drag-out / foreign receive ───────────────────────────────────────────

    /// Move cards received from another corkboard into the current container.
    pub fn receive_cards(&self, items: &[CorkboardCard]) {
        let item_ids: Vec<u64> = items.iter().map(|c| c.item_id).collect();
        if item_ids.is_empty() {
            return;
        }
        use frontend::binder_item_management::{MoveDto, MovePlace};
        let _ = frontend::commands::binder_item_management_commands::move_items(
            &self.inner.app_ctx,
            self.inner.ids.stack_id.get(),
            &MoveDto {
                item_ids,
                target_id: Some(self.inner.current_container.get()),
                target_is_binder: false,
                move_place: MovePlace::Into,
            },
        );
    }

    // ── Per-card "More actions" (mirrors the Full-Synopsis row menu) ──────────
    //
    // Each action is an undoable backend call, delegating to the same commands
    // `StreamViewModel` uses. `begin_*` presents a modal for text entry; the
    // apply method does the call. Move / merge resolve the card's neighbour from
    // the *current container's* ordered cards, so they read as manuscript order.

    /// A snapshot of the container's cards in order (what the grid shows before
    /// any filter/sort projection) — the basis for move/merge neighbour math.
    fn ordered_cards(&self) -> Vec<CorkboardCard> {
        self.inner.cards.cards()
    }

    fn card_pos(&self, id: u64) -> Option<(Vec<CorkboardCard>, usize)> {
        let cards = self.ordered_cards();
        cards
            .iter()
            .position(|c| c.item_id == id)
            .map(|pos| (cards, pos))
    }

    /// This card can be merged into the one before it: both carry scene prose
    /// and this card does not open a structural section (merging it away would
    /// delete the boundary). The backend enforces the same rule; this hides the
    /// menu item where it would be rejected.
    pub fn can_merge_into_previous(&self, id: u64) -> bool {
        let Some((cards, pos)) = self.card_pos(id) else {
            return false;
        };
        if pos == 0 {
            return false;
        }
        let this = &cards[pos];
        let prev = &cards[pos - 1];
        is_prose_bearing(&this.role, &this.sub_role)
            && !opens_a_section(&this.sub_role)
            && is_prose_bearing(&prev.role, &prev.sub_role)
    }

    // -- dialog entry points --

    pub fn begin_set_label(&self, ctx: &mut EventContext, id: u64) {
        let current = self.item_dto(id).map(|d| d.label).unwrap_or_default();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_set_label()))
            .default_text(current)
            .on_result(move |r, ctx| {
                if let Some(label) = r {
                    vm.set_label(ctx, id, label.trim());
                }
            })
            .present(ctx);
    }

    pub fn begin_insert_after(&self, ctx: &mut EventContext, id: u64) {
        let vm = self.clone();
        InputDialog::new(tr!(dialog_new_scene()))
            .placeholder(tr!(placeholder_scene_name()))
            .on_result(move |r, ctx| {
                if let Some(name) = r
                    && !name.trim().is_empty()
                {
                    vm.insert_after(ctx, id, name.trim());
                }
            })
            .present(ctx);
    }

    // -- apply methods --

    pub fn set_label(&self, _ctx: &mut EventContext, id: u64, label: &str) {
        if let Some(it) = self.item_dto(id) {
            let mut dto = update_item_dto(&it);
            dto.label = label.to_string();
            let _ =
                binder_item_commands::update_binder_item(&self.inner.app_ctx, self.stack(), &dto);
        }
    }

    /// Create the model's default recommendation for this card, placed by the
    /// relation it recommends — the same `binder_placement` math the outline and
    /// stream use, so a new item lands identically everywhere.
    pub fn insert_after(&self, _ctx: &mut EventContext, id: u64, title: &str) {
        let Some(card) = self.ordered_cards().into_iter().find(|c| c.item_id == id) else {
            return;
        };
        self.create_by_recommendation(id, &card.role, &card.sub_role, title);
    }

    pub fn move_up(&self, _ctx: &mut EventContext, id: u64) {
        if let Some((cards, pos)) = self.card_pos(id)
            && pos > 0
        {
            self.move_relative(id, cards[pos - 1].item_id, MovePlace::Before);
        }
    }

    pub fn move_down(&self, _ctx: &mut EventContext, id: u64) {
        if let Some((cards, pos)) = self.card_pos(id)
            && pos + 1 < cards.len()
        {
            self.move_relative(id, cards[pos + 1].item_id, MovePlace::After);
        }
    }

    /// Merge this card into the previous one. The backend concatenates both the
    /// prose and the synopsis into the survivor. The cards are read-only viewers
    /// (never open editors here), so there is nothing to flush first — the merge
    /// reads current content straight from the store.
    pub fn merge_into_previous(&self, _ctx: &mut EventContext, id: u64) {
        let Some((cards, pos)) = self.card_pos(id) else {
            return;
        };
        if pos == 0 || !self.can_merge_into_previous(id) {
            return;
        }
        let prev_id = cards[pos - 1].item_id;
        let _ = binder_item_management_commands::merge_two_scenes(
            &self.inner.app_ctx,
            self.stack(),
            &MergeTwoScenesDto {
                target_id: prev_id,
                source_id: id,
            },
        );
    }

    pub fn trash(&self, _ctx: &mut EventContext, id: u64) {
        if let Some((binder, _order, _pos)) = self.locate(id) {
            let _ = trash_management_commands::trash_binder_items(
                &self.inner.app_ctx,
                self.stack(),
                &TrashBinderItemsDto {
                    binder_item_ids: vec![id as i64],
                    origin_binder_id: binder as i64,
                },
            );
        }
    }

    // -- backend helpers (mirror StreamViewModel's private helpers) --

    fn stack(&self) -> Option<u64> {
        self.inner.ids.stack_id.get()
    }

    fn chapter_mode(&self) -> skribisto_model::ChapterMode {
        self.inner
            .ids
            .work_id
            .get()
            .and_then(|id| {
                work_commands::get_work(&self.inner.app_ctx, &id)
                    .ok()
                    .flatten()
            })
            .map(|w| w.chapter_mode)
            .unwrap_or_default()
    }

    fn item_dto(&self, id: u64) -> Option<BinderItemDto> {
        binder_item_commands::get_binder_item(&self.inner.app_ctx, &id)
            .ok()
            .flatten()
    }

    /// Find the binder owning `id`, its ordered items, and `id`'s position.
    fn locate(&self, id: u64) -> Option<(u64, Vec<u64>, usize)> {
        let work_id = self.inner.ids.work_id.get()?;
        let binders = work_commands::get_work_relationship(
            &self.inner.app_ctx,
            &work_id,
            &WorkRelationshipField::Binders,
        )
        .ok()?;
        for binder in binders {
            let order = binder_commands::get_binder_relationship(
                &self.inner.app_ctx,
                &binder,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            if let Some(pos) = order.iter().position(|&x| x == id) {
                return Some((binder, order, pos));
            }
        }
        None
    }

    fn item_meta(&self, order: &[u64]) -> ItemMeta {
        binder_item_commands::get_binder_item_multi(&self.inner.app_ctx, order)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .map(|it| (it.id, (it.indent, it.sub_role)))
            .collect()
    }

    fn create_by_recommendation(
        &self,
        anchor_id: u64,
        anchor_role: &BinderItemRole,
        anchor_sub_role: &BinderItemSubRole,
        title: &str,
    ) {
        let Some(rec) = skribisto_model::recommendations(anchor_role, anchor_sub_role)
            .into_iter()
            .next()
        else {
            return;
        };
        let (role, sub_role) = rec.create_type.combo(self.chapter_mode());
        if skribisto_model::validate_item(&role, &sub_role, &[]).is_err() {
            return;
        }
        let Some((binder, order, pos)) = self.locate(anchor_id) else {
            return;
        };
        let meta = self.item_meta(&order);
        let anchor_indent = meta.get(&anchor_id).map(|(i, _)| *i).unwrap_or(0);
        let (index, indent) = binder_placement::insertion_point_for_item(
            &order,
            &meta,
            pos,
            anchor_indent,
            rec.relation,
        );
        let dto = CreateBinderItemDto {
            title: title.to_string(),
            role,
            sub_role,
            activated: true,
            is_exportable: true,
            indent,
            ..Default::default()
        };
        let _ = binder_item_commands::create_binder_item(
            &self.inner.app_ctx,
            self.stack(),
            &dto,
            binder,
            index as i32,
        );
    }

    fn move_relative(&self, id: u64, target: u64, place: MovePlace) {
        let _ = binder_item_management_commands::move_items(
            &self.inner.app_ctx,
            self.stack(),
            &MoveDto {
                item_ids: vec![id],
                target_id: Some(target),
                target_is_binder: false,
                move_place: place,
            },
        );
    }

    // ── Accessors for the view ───────────────────────────────────────────────

    pub fn cards_model(&self) -> CorkboardCardsModel {
        self.inner.cards.clone()
    }
    pub fn projection(&self) -> SortFilterListModel<CorkboardCard> {
        self.inner.projection.clone()
    }
    /// Snapshot of the current cards (for resolving an activation index → card).
    pub fn cards(&self) -> Vec<CorkboardCard> {
        self.inner.cards.cards()
    }
    pub fn is_projecting(&self) -> Signal<bool> {
        self.inner.projecting.clone()
    }
    pub fn selection(&self) -> SelectionModel {
        self.inner.selection.clone()
    }
    pub fn scroll_y(&self) -> Signal<f32> {
        self.inner.scroll_y.clone()
    }
    pub fn trail_signal(&self) -> Signal<Vec<(u64, String)>> {
        self.inner.trail.clone()
    }
    pub fn container_title(&self) -> Signal<String> {
        self.inner.container_probe.title()
    }
    pub fn search_query_signal(&self) -> Signal<String> {
        self.inner.query.clone()
    }
    pub fn sort_signal(&self) -> Signal<Option<(String, SortDirection)>> {
        self.inner.sort.clone()
    }
    pub fn count_signal(&self) -> Signal<usize> {
        self.inner.count.clone()
    }
    pub fn nested(&self) -> Signal<bool> {
        self.inner.nested.clone()
    }
    pub fn card_size(&self) -> Signal<f32> {
        self.inner.card_size.clone()
    }
    pub fn show_word_count(&self) -> Signal<bool> {
        self.inner.show_word_count.clone()
    }
    pub fn counting_method(&self) -> Signal<CountingMethodSetting> {
        self.inner.counting_method.clone()
    }
    /// The app context — the tile delegate needs it to build a per-card single.
    pub fn app_ctx(&self) -> Rc<AppContext> {
        self.inner.app_ctx.clone()
    }
    /// The recommendations' anchor (current container), for the create button's title.
    pub fn current_container(&self) -> u64 {
        self.inner.current_container.get()
    }

    // ── internals ─────────────────────────────────────────────────────────────

    fn recompute_projecting(&self) {
        let projecting =
            !self.inner.query.get().trim().is_empty() || self.inner.sort.get().is_some();
        if self.inner.projecting.get() != projecting {
            self.inner.projecting.set(projecting);
        }
    }
}

/// Does this `(role, sub_role)` carry scene prose? The constraint matrix decides
/// (not `SubRoleExt`), matching the backend gate on merge — a card can only be
/// merged where there is prose to concatenate.
fn is_prose_bearing(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> bool {
    skribisto_model::content_allowed(role, sub_role, &ContentRole::SceneText)
}

/// Split `doc` at char offset `caret` into two Djot strings, preserving inline
/// formatting (mirrors the stream's helper) — used by the synopsis "Split scene".
fn split_djot(doc: &TextDocument, caret: usize) -> anyhow::Result<(String, String)> {
    let n = doc.character_count();
    let caret = caret.min(n);
    let extract = |from: usize, to: usize| -> anyhow::Result<String> {
        let c = doc.cursor();
        c.set_position(from, MoveMode::MoveAnchor);
        c.set_position(to, MoveMode::KeepAnchor);
        let frag = c.selection();
        let tmp = TextDocument::new();
        tmp.cursor().insert_fragment(&frag)?;
        Ok(tmp.to_djot()?)
    };
    let before = extract(0, caret)?;
    let after = extract(caret, n)?;
    Ok((before, after))
}

/// Does this row open a structural section? Such a card must never be merged
/// *away* — it would delete the boundary (and orphan a chapter folder's scenes).
fn opens_a_section(sub_role: &BinderItemSubRole) -> bool {
    sub_role.opens_chapter() || sub_role.opens_part() || sub_role.opens_book()
}

/// Build a scalar-only `UpdateBinderItemDto` from a fetched item (mirrors the
/// stream/outline helper) — used to patch a single field (the label) back.
fn update_item_dto(it: &BinderItemDto) -> UpdateBinderItemDto {
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
        is_exportable: it.is_exportable,
        indent: it.indent,
        word_count_goal: it.word_count_goal,
        char_count_goal: it.char_count_goal,
        dict_language: it.dict_language.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typo() -> EditorTypography {
        EditorTypography {
            font_family: Signal::new(String::new()),
            size: Signal::new(16.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
        }
    }

    fn vm() -> CorkboardViewModel {
        // An empty AppContext: the model/probe reads find nothing (graceful) — the
        // navigation logic under test manipulates only signals.
        let ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let docs = OpenDocsStore::new(ctx.clone());
        CorkboardViewModel::new(
            ctx,
            ids,
            docs,
            100,
            Signal::new(true),
            Signal::new(200.0),
            Signal::new(true),
            Signal::new(CountingMethodSetting::default()),
            typo(),
            Signal::new(600.0),
        )
    }

    #[test]
    fn starts_at_the_tab_container() {
        let vm = vm();
        assert_eq!(vm.current_container(), 100);
        let trail = vm.trail_signal().get();
        assert_eq!(trail.len(), 1);
        assert_eq!(trail[0].0, 100);
    }

    #[test]
    fn drill_in_deepens_the_trail_and_rescopes() {
        let vm = vm();
        vm.drill_into(200, "Part Two".to_string());
        assert_eq!(vm.current_container(), 200);
        let trail = vm.trail_signal().get();
        assert_eq!(trail.len(), 2);
        assert_eq!(trail[1], (200, "Part Two".to_string()));

        vm.drill_into(300, "Chapter Five".to_string());
        assert_eq!(vm.current_container(), 300);
        assert_eq!(vm.trail_signal().get().len(), 3);
    }

    #[test]
    fn crumb_navigation_truncates_back_to_an_ancestor() {
        let vm = vm();
        vm.drill_into(200, "Part Two".to_string());
        vm.drill_into(300, "Chapter Five".to_string());
        // Jump back to the root crumb.
        vm.go_to_crumb(0);
        assert_eq!(vm.current_container(), 100);
        assert_eq!(vm.trail_signal().get().len(), 1);
    }

    #[test]
    fn crumb_navigation_to_the_current_level_is_a_noop() {
        let vm = vm();
        vm.drill_into(200, "Part Two".to_string());
        vm.go_to_crumb(1); // already here
        assert_eq!(vm.current_container(), 200);
        assert_eq!(vm.trail_signal().get().len(), 2);
    }
}
