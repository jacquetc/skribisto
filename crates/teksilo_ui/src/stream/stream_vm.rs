// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

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

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use teksilo::data::ListModel;
use teksilo::prelude::*; // EventContext, Signal, BuildContext, tr!
use teksilo::text_document::TextDocument;
use teksilo::widgets::InputDialog;

use crate::comments::binding::CommentBinding;

use frontend::AppContext;
use frontend::binder_item_management::{MergeTwoScenesDto, MovePlace, SplitSceneDto};
use frontend::commands::{
    binder_item_commands, binder_item_management_commands, trash_management_commands,
};
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
use frontend::trash_management::TrashSelectionDto;

use crate::app_ids::AppIds;
use crate::models::{OpenDoc, OpenDocsStore, StreamLevel, StreamRow, StreamRowsModel};
use crate::singles::SingleBinderItem;

use crate::shared::binder_ops::{
    self, is_prose_bearing, opens_a_section, split_djot, update_item_dto,
};

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

        // Every build, not once — see `StreamRowsModel::wire`: a subscription is scoped
        // to the widget's current build and dropped on the next one.
        {
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
                    // now stale. Dropping our cached handle is not enough: while any other
                    // holder (a standalone tab, the other pane) still references the entry,
                    // the store keeps the stale doc and hands it straight back. Rebuild it
                    // in place instead, so every holder sees the fresh one.
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
                        inner.docs.rebuild(id, stack);
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

    /// Every document this stream is **already showing**, in the order it shows them —
    /// what a find spanning the whole page searches.
    ///
    /// Deliberately non-opening: it reads the row cache directly rather than going
    /// through [`row_doc`](Self::row_doc), which on a miss runs a full synchronous Djot
    /// import — the thing that once made switching a Book to Full Book freeze for
    /// seconds. A row the page has not opened yet is a row the reader is not looking at,
    /// and it contributes nothing rather than being imported on a keystroke.
    ///
    /// In practice that excludes nothing on a mounted page: `Repeater` is not
    /// virtualized, so every row of the stream has a live editor and its document is
    /// opened by the row factory. What it does exclude is the moment *before* that — the
    /// banner is the page's parent and builds first — which the per-frame re-sync in
    /// [`FindViewModel::tick`](crate::search::FindViewModel::tick) picks up.
    pub fn page_documents(&self, flavour: SplitFlavour) -> Vec<(u64, TextDocument)> {
        let cache = self.inner.row_handles.borrow();
        self.inner
            .rows
            .ids()
            .into_iter()
            .filter_map(|id| {
                let handle = cache.get(&id)?;
                let field = match flavour {
                    SplitFlavour::Prose => handle.doc.main.as_ref(),
                    SplitFlavour::Synopsis => handle.doc.synopsis.as_ref(),
                }?;
                Some((id, field.doc.clone()))
            })
            .collect()
    }

    /// This row's door to the comment feature, for the surface `flavour` names.
    ///
    /// A stream row is a perfectly ordinary commentable editor: `row_doc` opens
    /// through the **shared** store, so the row and a standalone tab on the same
    /// item are one document, and the binding it mints is the same one that tab
    /// would get. What a row genuinely cannot have is a *view-state* binding —
    /// "the caret of this tab" has no answer with twelve editors on the page — and
    /// the two were conflated when the streams were first written, which is why
    /// comments were washed into stream prose that offered no way to read them.
    pub fn row_comments(&self, id: u64, flavour: SplitFlavour) -> Option<CommentBinding> {
        let doc = self.row_doc(id)?;
        match flavour {
            SplitFlavour::Prose => doc.comment_binding_main(),
            SplitFlavour::Synopsis => doc.comment_binding_synopsis(),
        }
    }

    /// Any row's comments view-model, for a page that needs to watch the store but
    /// whose container has no commentable surface of its own (a Part or a Book has
    /// no prose). Every binding on the page shares one view-model, so the first
    /// that resolves answers for all of them.
    pub fn row_comments_any_view_model(
        &self,
        flavour: SplitFlavour,
    ) -> Option<crate::comments::CommentsViewModel> {
        self.inner
            .rows
            .ids()
            .into_iter()
            .find_map(|id| self.row_comments(id, flavour))
            .map(|b| b.view_model())
            // A container with no rows *yet* — a freshly created Chapter, a Part
            // whose scenes have not been written — has no binding to ask, but its
            // page still has to watch the comment store: rows and their comments
            // arrive later, and whatever subscribed at build time is all this page
            // will ever have. Fall back to the store's project-wide handle, which
            // exists from `App::build` onward and does not depend on a row.
            .or_else(|| self.inner.docs.comments())
    }

    /// Does any row on this page carry a live comment?
    ///
    /// The page-level question behind the gutter reservation — see
    /// [`ColumnWithMargin::reserve`](crate::comments::pane::ColumnWithMargin::reserve).
    /// Walks the rows rather than the comment store because it is the *rows on this
    /// page* that decide, and a container's stream is a small slice of a project's
    /// comments.
    pub fn any_row_has_comments(&self, flavour: SplitFlavour) -> bool {
        self.inner.rows.ids().into_iter().any(|id| {
            self.row_comments(id, flavour)
                .is_some_and(|b| b.has_live_cards())
        })
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

    /// The row's tag ids, live.
    ///
    /// Read from the row's own probe rather than baked into `StreamRow`, because
    /// `StreamRowsModel` deliberately does not watch `BinderItem(Updated)` — a baked field
    /// would go stale the moment a tag was assigned. This is the same shape as
    /// [`row_label`](Self::row_label), and for the same reason.
    pub fn row_tags(&self, id: u64) -> Signal<Vec<u64>> {
        match self.handle(id) {
            Some(h) => h
                .probe
                .dto_signal()
                .map(|d| d.as_ref().map(|x| x.tags.clone()).unwrap_or_default()),
            None => Signal::new(Vec::new()),
        }
    }

    /// This project's workflow ladder.
    ///
    /// Built from `app_ctx` + `ids` rather than threaded from the `WorkSession`, and that is
    /// safe *here* only because `StatusesViewModel` caches nothing: `ladder()` reads through
    /// the relationship on every call, so this handle and the session's can never disagree
    /// about what the rungs are. The one thing it does not share is the session handle's
    /// `revision` signal, which nothing on a stream row binds to — a row rebuilds on its own
    /// `BinderItem::Updated`, and the picker's list is rebuilt with it.
    pub fn statuses(&self) -> crate::statuses::StatusesViewModel {
        crate::statuses::StatusesViewModel::new(self.inner.app_ctx.clone(), self.inner.ids.clone())
    }

    /// The row's current rung, live. `None` is "no status" — and so is a rung that no
    /// longer resolves, since the reference is weak by design.
    pub fn row_status(&self, id: u64) -> Signal<Option<u64>> {
        match self.handle(id) {
            Some(h) => h
                .probe
                .dto_signal()
                .map(|d| d.as_ref().and_then(|x| x.status)),
            None => Signal::new(None),
        }
    }

    /// Persist a row's tag ids.
    ///
    /// Tags are a *relationship*, not a scalar on the DTO, so this cannot go through
    /// `update_binder_item` the way [`set_row_label`](Self::set_row_label) does — it writes
    /// through the probe's `set_tags`, which issues the relationship command.
    pub fn set_row_tags(&self, id: u64, tags: &[u64]) {
        if let Some(h) = self.handle(id) {
            let _ = h.probe.set_tags(tags, self.stack());
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

    /// Rename the container. Through the single, so its title `Content` row follows the
    /// entity field — one title, two homes (see [`SingleBinderItem`]).
    pub fn rename_container(&self, _ctx: &mut EventContext, title: &str) {
        let _ = self.inner.container_probe.set_title(title, self.stack());
    }

    pub fn rename_row(&self, _ctx: &mut EventContext, id: u64, title: &str) {
        if let Some(h) = self.handle(id) {
            let _ = h.probe.set_title(title, self.stack());
        }
    }

    pub fn set_row_label(&self, _ctx: &mut EventContext, id: u64, label: &str) {
        if let Some(it) = binder_ops::item_dto(&self.inner.app_ctx, id) {
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
        binder_ops::create_by_recommendation(
            &self.inner.app_ctx,
            &self.inner.ids,
            id,
            &row.role,
            &row.sub_role,
            title,
        );
    }

    /// Append to the stream: after its last row if it has one, else as the container's
    /// first child.
    pub fn add_row(&self, ctx: &mut EventContext, title: &str) {
        if let Some(last) = self.rows().last().map(|r| r.item_id) {
            self.insert_after(ctx, last, title);
        } else {
            let sub_role = self.inner.container_sub_role.clone();
            binder_ops::create_by_recommendation(
                &self.inner.app_ctx,
                &self.inner.ids,
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
        binder_ops::move_relative(
            &self.inner.app_ctx,
            &self.inner.ids,
            id,
            ids[pos - 1],
            MovePlace::Before,
        );
    }

    pub fn move_row_down(&self, _ctx: &mut EventContext, id: u64) {
        let ids = self.inner.rows.ids();
        let Some(pos) = ids.iter().position(|&x| x == id) else {
            return;
        };
        if pos + 1 >= ids.len() {
            return;
        }
        binder_ops::move_relative(
            &self.inner.app_ctx,
            &self.inner.ids,
            id,
            ids[pos + 1],
            MovePlace::After,
        );
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
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return; // no project open
        };

        // Merge reads content from the store — flush both rows' docs first.
        let stack = self.stack();
        let _ = prev.flush(stack);
        let _ = cur.flush(stack);
        let _ = binder_item_management_commands::merge_two_scenes(
            &self.inner.app_ctx,
            stack,
            &MergeTwoScenesDto {
                work_id,
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
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return; // no project open
        };
        // The origin binder is resolved by the use case; the stream does not
        // need to locate the row first.
        let _ = trash_management_commands::trash_selection(
            &self.inner.app_ctx,
            self.stack(),
            &TrashSelectionDto {
                work_id,
                binder_ids: Vec::new(),
                binder_item_ids: vec![id],
            },
        );
    }

    // ── helpers ──

    fn stack(&self) -> Option<u64> {
        self.inner.ids.stack_id.get()
    }

    fn row_pos(&self, id: u64) -> Option<usize> {
        self.inner.rows.ids().iter().position(|&x| x == id)
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

    // The `is_prose_bearing` / `is_synopsis_bearing` / `opens_a_section` / `split_djot`
    // tests moved to `shared::binder_ops` along with the functions themselves. What
    // stays here is what stays here: the `can_split_row` / `can_merge_row` row-list rules,
    // which are the stream's own.

    fn row(item_id: u64, role: BinderItemRole, sub_role: BinderItemSubRole) -> StreamRow {
        StreamRow {
            item_id,
            role,
            sub_role,
            number: None,
            fallback_label: None,
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
