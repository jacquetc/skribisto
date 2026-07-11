//! `StreamViewModel` — business logic for a container tab's **manuscript streams**:
//! the Full Chapter / Full Part / Full Book view and its Full Synopsis twin.
//!
//! It owns the container's ordered row list ([`StreamRowsModel`]) and the row
//! mutations (rename, set label, insert / add / move / merge / split / trash). The
//! two stream *flavours* — prose and synopsis — are the same rows with a different
//! editor per row, so one view-model drives both; [`SplitFlavour`] names which one an
//! action came from.
//!
//! **One instance per open container tab**, created by `ContentTab::new` — *not* by
//! `OpenDoc::build`. That placement is load-bearing:
//!
//! * This view-model holds the [`OpenDocsStore`] (it opens its rows' documents through
//!   it), and the store owns every `OpenDoc`. Hanging the view-model off an `OpenDoc`
//!   would close an `Rc` cycle — and worse, `OpenDocsStore::clear()` drops its
//!   `OpenDoc`s *while holding the map's `RefCell` borrow*, so a `Drop` that released
//!   row refs would re-enter `borrow_mut()` and panic. On `ContentTab` the chain
//!   `TabHandle → ContentTab → StreamViewModel → OpenDocsStore` runs one way, and
//!   nothing the store owns points back.
//! * Sharing it across panes is no longer needed for what that used to buy: two panes
//!   on the same container get two view-models, but both call `store.open(row_id)` for
//!   the same ids, so their rows are the **same live documents** regardless.
//!
//! Rows take their documents from the store, so a scene open as its own tab *and*
//! shown inside a stream is one document — which is what the old per-view `SingleScene`
//! got wrong (two `TextDocument`s over one `Content` row, silently diverging).
//!
//! All name entry is a modal `InputDialog` (mirrors `OutlineViewModel::begin_rename`);
//! each `begin_*` presents the dialog and the matching apply-method does the undoable
//! backend call. Plain Rust → headless-testable.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use bastyde::data::ListModel;
use bastyde::prelude::*; // EventContext, Signal, BuildContext, tr!
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
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
use frontend::direct_access::{BinderItemDto, CreateBinderItemDto, UpdateBinderItemDto};
use frontend::trash_management::TrashBinderItemsDto;

use skribisto_model::SubRoleExt;

use crate::app_ids::AppIds;
use crate::binder_placement::{self, ItemMeta};
use crate::models::{OpenDoc, OpenDocsStore, StreamLevel, StreamRow, StreamRowsModel};
use crate::singles::SingleBinderItem;

/// Which of a row's two writing surfaces an action came from. The stream pane picks
/// its body with this, and a split cuts *that* role at the caret.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SplitFlavour {
    Prose,
    Synopsis,
}

/// One cached row: its shared document plus a probe for the reactive title / label.
#[derive(Clone)]
struct RowHandle {
    doc: Rc<OpenDoc>,
    probe: SingleBinderItem,
    /// The `sub_role` the `doc` was built from. An `OpenDoc`'s fields are decided once,
    /// at construction, from the constraint matrix — so if a Promote changes the row's
    /// type, the cached doc is stale and must be evicted and reopened.
    sub_role: BinderItemSubRole,
}

struct Inner {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    docs: OpenDocsStore,
    container_id: u64,
    container_sub_role: BinderItemSubRole,
    level: StreamLevel,
    rows: StreamRowsModel,
    container_probe: SingleBinderItem,
    /// One handle per row, created lazily and reused across list refreshes so an edited
    /// row's document survives structural changes.
    row_handles: RefCell<HashMap<u64, RowHandle>>,
    subscribed: Cell<bool>,
}

/// Release every document this stream still holds, when the tab closes (the
/// `ContentTab` drops, taking the last `Rc<Inner>` with it).
///
/// This is *why* `wire`'s closures capture a `Weak`: an `Rc` capture would keep `Inner`
/// alive forever, this `Drop` would never run, and every row the stream ever opened
/// would leak. Calling `release` here is safe — `OpenDocsStore::release` drops the
/// evicted `Rc<OpenDoc>` *after* letting go of the map's borrow, and an `OpenDoc` is a
/// leaf that points at nothing.
impl Drop for Inner {
    fn drop(&mut self) {
        let stack = self.ids.stack_id.get();
        for id in self.row_handles.borrow().keys() {
            self.docs.release(*id, stack);
        }
    }
}

#[derive(Clone)]
pub struct StreamViewModel {
    inner: Rc<Inner>,
}

// Not every affordance has a caller until every stream surface is wired.
#[allow(dead_code)]
impl StreamViewModel {
    /// A stream for `container_id`, or `None` if this `(role, sub_role)` has none. The
    /// gate is [`StreamLevel::for_container`] — the *same* one `ContentTab::new` uses,
    /// so there is no second partial function to keep in sync by hand.
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        docs: OpenDocsStore,
        container_id: u64,
        container_role: &BinderItemRole,
        container_sub_role: &BinderItemSubRole,
    ) -> Option<Self> {
        let level = StreamLevel::for_container(container_role, container_sub_role)?;
        let rows = StreamRowsModel::new(app_ctx.clone(), ids.work_id.clone(), container_id, level);
        let container_probe = SingleBinderItem::new(app_ctx.clone());
        container_probe.set_id(Some(container_id));
        Some(Self {
            inner: Rc::new(Inner {
                app_ctx,
                ids,
                docs,
                container_id,
                container_sub_role: container_sub_role.clone(),
                level,
                rows,
                container_probe,
                row_handles: RefCell::new(HashMap::new()),
                subscribed: Cell::new(false),
            }),
        })
    }

    /// Subscribe once (row list + per-row metadata) and fill the list.
    ///
    /// Both closures capture a **`Weak`**, never `self`. They are *stored* — the removal
    /// callback for the row model's lifetime, the event subscription for the widget
    /// tree's — and the row model is itself owned by this view-model, so an `Rc` capture
    /// would close the cycle `Inner → rows → stored closure → Inner`. `Drop` would never
    /// run and every row this stream opened would leak. A stale `Weak` simply fails to
    /// `upgrade()` once the tab is gone.
    pub fn wire(&self, ctx: &mut BuildContext) {
        // Rows that left the stream (trashed, merged away, moved out): drop our
        // reference to their shared documents.
        let weak = Rc::downgrade(&self.inner);
        self.inner.rows.wire(ctx, move |removed: &[u64]| {
            let Some(inner) = weak.upgrade() else {
                return;
            };
            let stack = inner.ids.stack_id.get();
            let gone: Vec<u64> = {
                let mut cache = inner.row_handles.borrow_mut();
                removed
                    .iter()
                    .filter(|id| cache.remove(id).is_some())
                    .copied()
                    .collect()
            };
            for id in gone {
                inner.docs.release(id, stack);
            }
        });

        if !self.inner.subscribed.replace(true) {
            let weak = Rc::downgrade(&self.inner);
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
                move |event: &Event| {
                    let Some(inner) = weak.upgrade() else {
                        return;
                    };
                    if event.ids.contains(&inner.container_id) {
                        inner.container_probe.set_id(Some(inner.container_id));
                    }
                    let want: HashMap<u64, BinderItemSubRole> = inner
                        .rows
                        .rows()
                        .into_iter()
                        .map(|r| (r.item_id, r.sub_role))
                        .collect();
                    // A Promote rewrites a row's type in place. That row's `OpenDoc`
                    // decided its fields at construction, from the *old* type, so it is
                    // now stale: evict it and let the next `row_doc()` rebuild it.
                    let stale: Vec<u64> = {
                        let mut cache = inner.row_handles.borrow_mut();
                        let mut stale = Vec::new();
                        for id in &event.ids {
                            let Some(h) = cache.get(id) else { continue };
                            h.probe.set_id(Some(*id));
                            if want.get(id).is_some_and(|sr| *sr != h.sub_role) {
                                stale.push(*id);
                            }
                        }
                        for id in &stale {
                            cache.remove(id);
                        }
                        stale
                    };
                    let stack = inner.ids.stack_id.get();
                    for id in stale {
                        inner.docs.release(id, stack);
                    }
                },
            );
        }
    }

    // ── reactive reads ──

    /// The `Repeater`'s data source.
    pub fn list(&self) -> ListModel<StreamRow> {
        self.inner.rows.list()
    }

    /// A snapshot of the rows — for the synchronous gating below, which needs each row's
    /// `(role, sub_role)`, not just its id.
    pub fn rows(&self) -> Vec<StreamRow> {
        self.inner.rows.rows()
    }

    pub fn level(&self) -> StreamLevel {
        self.inner.level
    }

    pub fn container_title(&self) -> Signal<String> {
        self.inner.container_probe.title()
    }

    /// The row's shared live documents, opened through the store — so a row and a
    /// standalone tab on the same item are one document. Cached and reused.
    pub fn row_doc(&self, id: u64) -> Option<Rc<OpenDoc>> {
        self.handle(id).map(|h| h.doc)
    }

    pub fn row_title(&self, id: u64) -> Signal<String> {
        match self.handle(id) {
            Some(h) => h.probe.title(),
            None => Signal::new(String::new()),
        }
    }

    pub fn row_label(&self, id: u64) -> Signal<String> {
        match self.handle(id) {
            Some(h) => h
                .probe
                .dto_signal()
                .map(|d| d.as_ref().map(|x| x.label.clone()).unwrap_or_default()),
            None => Signal::new(String::new()),
        }
    }

    /// Get-or-open the row's handle. Opening refs the document in the store; the ref is
    /// dropped when the row leaves the stream (see [`wire`](Self::wire)) or the tab
    /// closes (see `Drop for Inner`).
    fn handle(&self, id: u64) -> Option<RowHandle> {
        if let Some(h) = self.inner.row_handles.borrow().get(&id) {
            return Some(h.clone());
        }
        let doc = self.inner.docs.open(id)?;
        let probe = SingleBinderItem::new(self.inner.app_ctx.clone());
        probe.set_id(Some(id));
        let h = RowHandle {
            sub_role: doc.sub_role.clone(),
            doc,
            probe,
        };
        self.inner.row_handles.borrow_mut().insert(id, h.clone());
        Some(h)
    }

    // ── gating (what the row menus offer) ──

    /// This row carries scene prose that can be cut in two.
    pub fn can_split(&self, id: u64) -> bool {
        can_split_row(&self.rows(), id)
    }

    /// This row can be merged into the previous one.
    pub fn can_merge_into_previous(&self, id: u64) -> bool {
        can_merge_row(&self.rows(), id)
    }

    pub fn can_move_up(&self, id: u64) -> bool {
        matches!(self.row_pos(id), Some(p) if p > 0)
    }
    pub fn can_move_down(&self, id: u64) -> bool {
        let ids = self.inner.rows.ids();
        matches!(ids.iter().position(|&x| x == id), Some(p) if p + 1 < ids.len())
    }

    // ── dialog entry points (present an InputDialog, apply on OK) ──

    pub fn begin_rename_container(&self, ctx: &mut EventContext) {
        let current = self.inner.container_probe.title().get();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_rename()))
            .default_text(current)
            .on_result(move |r, ctx| {
                if let Some(name) = r
                    && !name.trim().is_empty()
                {
                    vm.rename_container(ctx, name.trim());
                }
            })
            .present(ctx);
    }

    pub fn begin_rename_row(&self, ctx: &mut EventContext, id: u64) {
        let current = self.row_title(id).get();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_rename()))
            .default_text(current)
            .on_result(move |r, ctx| {
                if let Some(name) = r
                    && !name.trim().is_empty()
                {
                    vm.rename_row(ctx, id, name.trim());
                }
            })
            .present(ctx);
    }

    pub fn begin_set_label(&self, ctx: &mut EventContext, id: u64) {
        let current = self.row_label(id).get();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_set_label()))
            .default_text(current)
            .on_result(move |r, ctx| {
                if let Some(label) = r {
                    vm.set_row_label(ctx, id, label.trim());
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

    pub fn begin_add_row(&self, ctx: &mut EventContext) {
        let vm = self.clone();
        InputDialog::new(tr!(dialog_new_scene()))
            .placeholder(tr!(placeholder_scene_name()))
            .on_result(move |r, ctx| {
                if let Some(name) = r
                    && !name.trim().is_empty()
                {
                    vm.add_row(ctx, name.trim());
                }
            })
            .present(ctx);
    }

    // ── apply methods (undoable backend calls; unit-tested) ──

    pub fn rename_container(&self, _ctx: &mut EventContext, title: &str) {
        if let Some(it) = self.item_dto(self.inner.container_id) {
            let mut dto = update_item_dto(&it);
            dto.title = title.to_string();
            let _ =
                binder_item_commands::update_binder_item(&self.inner.app_ctx, self.stack(), &dto);
            self.inner
                .container_probe
                .set_id(Some(self.inner.container_id));
        }
    }

    pub fn rename_row(&self, _ctx: &mut EventContext, id: u64, title: &str) {
        if let Some(it) = self.item_dto(id) {
            let mut dto = update_item_dto(&it);
            dto.title = title.to_string();
            let _ =
                binder_item_commands::update_binder_item(&self.inner.app_ctx, self.stack(), &dto);
            if let Some(h) = self.handle(id) {
                h.probe.set_id(Some(id));
            }
        }
    }

    pub fn set_row_label(&self, _ctx: &mut EventContext, id: u64, label: &str) {
        if let Some(it) = self.item_dto(id) {
            let mut dto = update_item_dto(&it);
            dto.label = label.to_string();
            let _ =
                binder_item_commands::update_binder_item(&self.inner.app_ctx, self.stack(), &dto);
            if let Some(h) = self.handle(id) {
                h.probe.set_id(Some(id));
            }
        }
    }

    /// Create the recommended type after `id`, placed by the **relation** the model
    /// recommends. Not "right after the anchor, at the anchor's own indent": in a Full
    /// Part / Full Book stream the anchor may be a chapter head, whose default
    /// recommendation is `Child`, and that shortcut would drop the new scene *outside*
    /// the chapter it was added to.
    pub fn insert_after(&self, _ctx: &mut EventContext, id: u64, title: &str) {
        let Some(row) = self.rows().into_iter().find(|r| r.item_id == id) else {
            return;
        };
        self.create_by_recommendation(id, &row.role, &row.sub_role, title);
    }

    /// Append to the stream: after its last row if it has one, else as the container's
    /// first child.
    pub fn add_row(&self, ctx: &mut EventContext, title: &str) {
        if let Some(last) = self.rows().last().map(|r| r.item_id) {
            self.insert_after(ctx, last, title);
        } else {
            let sub_role = self.inner.container_sub_role.clone();
            self.create_by_recommendation(
                self.inner.container_id,
                &BinderItemRole::Folder,
                &sub_role,
                title,
            );
        }
    }

    pub fn move_row_up(&self, _ctx: &mut EventContext, id: u64) {
        let ids = self.inner.rows.ids();
        let Some(pos) = ids.iter().position(|&x| x == id) else {
            return;
        };
        if pos == 0 {
            return;
        }
        self.move_relative(id, ids[pos - 1], MovePlace::Before);
    }

    pub fn move_row_down(&self, _ctx: &mut EventContext, id: u64) {
        let ids = self.inner.rows.ids();
        let Some(pos) = ids.iter().position(|&x| x == id) else {
            return;
        };
        if pos + 1 >= ids.len() {
            return;
        }
        self.move_relative(id, ids[pos + 1], MovePlace::After);
    }

    /// Merge this row into the previous one. The backend concatenates **both** writing
    /// roles, so the survivor must reload both — the old chapter view reloaded only the
    /// prose and left a stale synopsis on screen.
    pub fn merge_into_previous(&self, ctx: &mut EventContext, id: u64) {
        if !self.can_merge_into_previous(id) {
            return;
        }
        let rows = self.rows();
        let Some(pos) = rows.iter().position(|r| r.item_id == id) else {
            return;
        };
        let prev_id = rows[pos - 1].item_id;
        let (Some(prev), Some(cur)) = (self.row_doc(prev_id), self.row_doc(id)) else {
            return;
        };

        // Merge reads content from the store — flush both rows' docs first.
        let stack = self.stack();
        let _ = prev.flush(stack);
        let _ = cur.flush(stack);
        let _ = binder_item_management_commands::merge_two_scenes(
            &self.inner.app_ctx,
            stack,
            &MergeTwoScenesDto {
                target_id: prev_id,
                source_id: id,
            },
        );
        // `prev` absorbed `id`'s prose *and* synopsis — reflect both in the (reused)
        // editors. `set_djot` only queues a document event, so pump a frame for the
        // editors the user did not touch.
        prev.reload();
        ctx.request_frame();
    }

    /// Split the row at `caret` into two. `which` says which editor the caret is in:
    /// that role is cut there, and the *other* role is passed whole to the source and
    /// empty to the new scene — which is exactly "it stays entirely on the original
    /// scene". See `split_scene_uc`, which applies both roles uniformly.
    pub fn split_row(&self, ctx: &mut EventContext, id: u64, which: SplitFlavour, caret: usize) {
        if !self.can_split(id) {
            return;
        }
        let Some(doc) = self.row_doc(id) else { return };
        let stack = self.stack();
        let _ = doc.flush(stack);

        let (prose, synopsis) = (doc.main.as_ref(), doc.synopsis.as_ref());
        let (before_text, after_text, before_synopsis, after_synopsis) = match which {
            SplitFlavour::Prose => {
                let Some(m) = prose else { return };
                let Ok((before, after)) = split_djot(&m.doc, caret) else {
                    return;
                };
                let whole = synopsis.map(|s| s.djot()).unwrap_or_default();
                (before, after, whole, String::new())
            }
            SplitFlavour::Synopsis => {
                let Some(s) = synopsis else { return };
                let Ok((before, after)) = split_djot(&s.doc, caret) else {
                    return;
                };
                let whole = prose.map(|m| m.djot()).unwrap_or_default();
                (whole, String::new(), before, after)
            }
        };

        let _ = binder_item_management_commands::split_scene(
            &self.inner.app_ctx,
            stack,
            &SplitSceneDto {
                source_id: id,
                before_text,
                after_text,
                before_synopsis,
                after_synopsis,
                new_title: tr!(new_scene_title()).resolve_now(),
            },
        );
        // `id` now holds only the before-halves — reflect them in the (reused) editors,
        // pumping a frame so the queued document events are drained.
        doc.reload();
        ctx.request_frame();
    }

    pub fn trash_row(&self, _ctx: &mut EventContext, id: u64) {
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

    // ── helpers ──

    fn stack(&self) -> Option<u64> {
        self.inner.ids.stack_id.get()
    }

    /// The open project's chapter storage mode (mirrors `OutlineViewModel`).
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

    fn row_pos(&self, id: u64) -> Option<usize> {
        self.inner.rows.ids().iter().position(|&x| x == id)
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

    /// `{id -> (indent, sub_role)}` for a binder's items — the data `binder_placement`
    /// walks.
    fn item_meta(&self, order: &[u64]) -> ItemMeta {
        binder_item_commands::get_binder_item_multi(&self.inner.app_ctx, order)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .map(|it| (it.id, (it.indent, it.sub_role)))
            .collect()
    }

    /// Create the model's default recommendation for the anchor, placed by the relation
    /// it recommends — through the shared `binder_placement` math, so the stream and the
    /// outline place a new item identically.
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
            is_printable: true,
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
}

/// Can the row `id` be cut in two? Only a prose-bearing row has anything to split, and
/// the backend rejects anything else.
///
/// Pure over the row list so the rule is unit-testable without a backend.
fn can_split_row(rows: &[StreamRow], id: u64) -> bool {
    rows.iter()
        .any(|r| r.item_id == id && is_prose_bearing(&r.role, &r.sub_role))
}

/// Can the row `id` be merged **into the previous row**? Three conditions, all necessary:
///
/// 1. it is prose-bearing — there is something to merge;
/// 2. it does not open a chapter/part/book — merging it away would delete that boundary
///    (and, for a chapter *folder*, orphan its child scenes);
/// 3. the immediately preceding **row** is prose-bearing — which is what stops a scene
///    being merged backwards across the heading that starts its chapter, now that a Full
///    Part / Full Book stream shows several chapters at once.
///
/// The backend enforces the same invariants; this is what hides the menu item.
/// Pure over the row list so the rule is unit-testable without a backend.
fn can_merge_row(rows: &[StreamRow], id: u64) -> bool {
    let Some(pos) = rows.iter().position(|r| r.item_id == id) else {
        return false;
    };
    if pos == 0 {
        return false;
    }
    let (this, prev) = (&rows[pos], &rows[pos - 1]);
    is_prose_bearing(&this.role, &this.sub_role)
        && !opens_a_section(&this.sub_role)
        && is_prose_bearing(&prev.role, &prev.sub_role)
}

/// Does this `(role, sub_role)` carry scene prose? The **constraint matrix** decides —
/// not `SubRoleExt::carries_scene()` — because the matrix is the source of truth and
/// the backend gates split/merge on exactly this predicate.
pub(crate) fn is_prose_bearing(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> bool {
    skribisto_model::content_allowed(role, sub_role, &ContentRole::SceneText)
}

/// Does this `(role, sub_role)` carry a synopsis? Same rule, other role — it is what
/// decides whether a row gets an editor in the Full Synopsis flavour.
pub(crate) fn is_synopsis_bearing(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> bool {
    skribisto_model::content_allowed(role, sub_role, &ContentRole::SynopsisText)
}

/// Does this row open a structural section? Such a row must never be merged *away*: it
/// would delete the boundary (and, for a chapter folder, orphan its child scenes).
fn opens_a_section(sub_role: &BinderItemSubRole) -> bool {
    sub_role.opens_chapter() || sub_role.opens_part() || sub_role.opens_book()
}

/// Build a scalar-only `UpdateBinderItemDto` from a fetched item (mirrors the outline's
/// helper).
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
        is_printable: it.is_printable,
        indent: it.indent,
        word_count_goal: it.word_count_goal,
        char_count_goal: it.char_count_goal,
        dict_language: it.dict_language.clone(),
    }
}

/// Split `doc` at char offset `caret` into two Djot strings, preserving inline
/// formatting, via fragment extraction into fresh documents.
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

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::common::entities::BinderItemRole::{Folder, Item};
    use frontend::common::entities::BinderItemSubRole::{Book, ChapterScene, Note, Part, Scene};

    fn vm(role: BinderItemRole, sub_role: BinderItemSubRole) -> Option<StreamViewModel> {
        let ctx = Rc::new(AppContext::new());
        let docs = OpenDocsStore::new(ctx.clone());
        StreamViewModel::new(ctx, AppIds::new(), docs, 1, &role, &sub_role)
    }

    /// Only folder containers host a stream — a flat chapter keeps its dual-pane editor.
    /// `ContentTab::new` gates on the same `StreamLevel::for_container`, so the two
    /// surfaces cannot drift apart.
    #[test]
    fn only_folder_containers_get_a_view_model() {
        assert_eq!(
            vm(Folder, ChapterScene).map(|v| v.level()),
            Some(StreamLevel::Chapter)
        );
        assert_eq!(vm(Folder, Part).map(|v| v.level()), Some(StreamLevel::Part));
        assert_eq!(vm(Folder, Book).map(|v| v.level()), Some(StreamLevel::Book));
        assert!(
            vm(Item, ChapterScene).is_none(),
            "the flat chapter is not a container"
        );
        assert!(vm(Item, Scene).is_none());
        assert!(vm(Folder, Note).is_none());
    }

    /// The prose/synopsis predicates read the constraint matrix, so they agree with what
    /// the backend will accept: both encodings of a chapter carry prose; a part and a
    /// book carry only a synopsis.
    #[test]
    fn editor_visibility_follows_the_constraint_matrix() {
        assert!(is_prose_bearing(&Item, &Scene));
        assert!(is_prose_bearing(&Item, &ChapterScene));
        assert!(is_prose_bearing(&Folder, &ChapterScene));
        assert!(!is_prose_bearing(&Folder, &Part));
        assert!(!is_prose_bearing(&Folder, &Book));

        // Every stream row has a synopsis — that is what makes the Full Synopsis stream
        // a complete outline with no holes.
        assert!(is_synopsis_bearing(&Item, &Scene));
        assert!(is_synopsis_bearing(&Item, &ChapterScene));
        assert!(is_synopsis_bearing(&Folder, &ChapterScene));
        assert!(is_synopsis_bearing(&Folder, &Part));
        assert!(is_synopsis_bearing(&Folder, &Book));
    }

    /// A row that opens a section can never be merged away: it would delete a
    /// chapter/part/book boundary (and orphan a chapter folder's children).
    #[test]
    fn structural_openers_are_never_merged_away() {
        assert!(opens_a_section(&ChapterScene));
        assert!(opens_a_section(&Part));
        assert!(opens_a_section(&Book));
        assert!(!opens_a_section(&Scene));
    }

    #[test]
    fn split_djot_cuts_at_the_caret() {
        let doc = TextDocument::new();
        let _ = doc.set_djot("HelloWorld").and_then(|op| op.wait());
        let (before, after) = split_djot(&doc, 5).expect("split");
        assert!(before.contains("Hello") && !before.contains("World"));
        assert!(after.contains("World") && !after.contains("Hello"));
    }

    fn row(item_id: u64, role: BinderItemRole, sub_role: BinderItemSubRole) -> StreamRow {
        StreamRow {
            item_id,
            role,
            sub_role,
        }
    }

    /// A Full Book stream over: Part(1) → chapter folder(2) → scenes 3,4 → flat
    /// chapter(5) → scene 6. The merge rule has to hold across every adjacency in it.
    fn book_rows() -> Vec<StreamRow> {
        vec![
            row(1, Folder, Part),
            row(2, Folder, ChapterScene),
            row(3, Item, Scene),
            row(4, Item, Scene),
            row(5, Item, ChapterScene),
            row(6, Item, Scene),
        ]
    }

    #[test]
    fn merge_is_offered_only_between_adjacent_prose_rows() {
        let rows = book_rows();
        // Scene after scene, inside one chapter: the ordinary case.
        assert!(can_merge_row(&rows, 4));
        // The chapter's *first* scene may merge into the chapter folder itself — the
        // folder carries prose, so this is the exact inverse of splitting it.
        assert!(can_merge_row(&rows, 3));
    }

    #[test]
    fn merge_never_crosses_a_structural_boundary() {
        let rows = book_rows();
        // Scene 6 is the first scene of the flat chapter 5 — and 5 *is* prose-bearing,
        // so this merges into its own chapter head, not across a boundary. Allowed.
        assert!(can_merge_row(&rows, 6));
        // But chapter 5 itself must NOT be merged into scene 4: it opens a chapter, and
        // merging it away would delete that boundary and swallow the chapter title.
        assert!(
            !can_merge_row(&rows, 5),
            "a chapter head must never be merged away"
        );
        // Nor may the chapter folder be merged into the part heading above it.
        assert!(
            !can_merge_row(&rows, 2),
            "a chapter folder must never be merged away (it would orphan its scenes)"
        );
        // The first row has nothing before it.
        assert!(!can_merge_row(&rows, 1));
        // A part heading carries no prose at all.
        let flat = vec![row(1, Folder, Part), row(2, Folder, Part)];
        assert!(!can_merge_row(&flat, 2));
    }

    #[test]
    fn split_is_offered_only_on_prose_rows() {
        let rows = book_rows();
        assert!(can_split_row(&rows, 3), "a scene splits");
        assert!(can_split_row(&rows, 5), "a flat chapter splits");
        assert!(
            can_split_row(&rows, 2),
            "a chapter folder's own prose splits — the matrix gives it a SceneText"
        );
        assert!(
            !can_split_row(&rows, 1),
            "a part heading carries no prose to split"
        );
        assert!(!can_split_row(&rows, 999), "unknown row");
    }
}
