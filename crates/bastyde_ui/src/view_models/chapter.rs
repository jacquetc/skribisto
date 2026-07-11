//! `ChapterViewModel` — business logic for the Chapter folder tab's **Full
//! Chapter** view: the ordered scene list + per-scene documents, and the
//! scene/chapter mutations (rename, set label, insert / add / move / merge /
//! split / trash).
//!
//! One instance **per open chapter tab** (each chapter tab has independent live
//! scene documents — the same per-tab ownership shape as `ContentTab`'s
//! `ProseField`s). Created in `OpenDoc::build` for a `Folder/Chapter` item and
//! stored on the `ContentTab`. All name entry is a modal `InputDialog` (mirrors
//! `OutlineViewModel::begin_rename`); each `begin_*` presents the dialog and the
//! matching apply-method does the undoable backend call. Plain Rust →
//! headless-testable.

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
    binder_commands, binder_item_commands, binder_item_management_commands, content_commands,
    trash_management_commands, work_commands,
};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
use frontend::direct_access::{BinderItemDto, CreateBinderItemDto, UpdateBinderItemDto};
use frontend::trash_management::TrashBinderItemsDto;

use crate::app_ids::AppIds;
use crate::models::{ChapterScenesModel, SceneRow};
use crate::singles::{SingleBinderItem, SingleScene};

struct Inner {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    chapter_id: u64,
    model: ChapterScenesModel,
    chapter_probe: SingleBinderItem,
    /// One `SingleScene` per id, created lazily and reused across list refreshes
    /// so an edited scene's document survives structural changes.
    scenes: RefCell<HashMap<u64, SingleScene>>,
    subscribed: Cell<bool>,
}

#[derive(Clone)]
pub struct ChapterViewModel {
    inner: Rc<Inner>,
}

// Not every method has a caller until the view is wired.
#[allow(dead_code)]
impl ChapterViewModel {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds, chapter_id: u64) -> Self {
        let model = ChapterScenesModel::new(app_ctx.clone(), ids.work_id.clone(), chapter_id);
        let chapter_probe = SingleBinderItem::new(app_ctx.clone());
        chapter_probe.set_id(Some(chapter_id));
        Self {
            inner: Rc::new(Inner {
                app_ctx,
                ids,
                chapter_id,
                model,
                chapter_probe,
                scenes: RefCell::new(HashMap::new()),
                subscribed: Cell::new(false),
            }),
        }
    }

    /// Subscribe once (scene list + per-scene metadata) and fill the list.
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.inner.model.wire(ctx);
        if !self.inner.subscribed.replace(true) {
            let me = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
                move |event: &Event| {
                    if event.ids.contains(&me.inner.chapter_id) {
                        me.inner.chapter_probe.set_id(Some(me.inner.chapter_id));
                    }
                    let map = me.inner.scenes.borrow();
                    for id in &event.ids {
                        if let Some(s) = map.get(id) {
                            s.refresh_meta();
                        }
                    }
                },
            );
        }
    }

    // ── reactive reads ──

    pub fn scenes(&self) -> ListModel<SceneRow> {
        self.inner.model.list()
    }

    pub fn chapter_title(&self) -> Signal<String> {
        self.inner.chapter_probe.title()
    }

    /// The per-scene handle (title / label / document), cached and reused.
    pub fn scene(&self, id: u64) -> SingleScene {
        let mut map = self.inner.scenes.borrow_mut();
        map.entry(id)
            .or_insert_with(|| SingleScene::new(self.inner.app_ctx.clone(), id))
            .clone()
    }

    /// True when a previous / next scene exists (for the row menu's enable state).
    pub fn can_move_up(&self, id: u64) -> bool {
        matches!(self.scene_pos(id), Some(p) if p > 0)
    }
    pub fn can_move_down(&self, id: u64) -> bool {
        let ids = self.inner.model.ids();
        matches!(ids.iter().position(|&x| x == id), Some(p) if p + 1 < ids.len())
    }

    // ── dialog entry points (present an InputDialog, apply on OK) ──

    pub fn begin_rename_chapter(&self, ctx: &mut EventContext) {
        let current = self.inner.chapter_probe.title().get();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_rename()))
            .default_text(current)
            .on_result(move |r, ctx| {
                if let Some(name) = r
                    && !name.trim().is_empty()
                {
                    vm.rename_chapter(ctx, name.trim());
                }
            })
            .present(ctx);
    }

    pub fn begin_rename_scene(&self, ctx: &mut EventContext, id: u64) {
        let current = self.scene(id).title().get();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_rename()))
            .default_text(current)
            .on_result(move |r, ctx| {
                if let Some(name) = r
                    && !name.trim().is_empty()
                {
                    vm.rename_scene(ctx, id, name.trim());
                }
            })
            .present(ctx);
    }

    pub fn begin_set_label(&self, ctx: &mut EventContext, id: u64) {
        let current = self.scene(id).label().get();
        let vm = self.clone();
        InputDialog::new(tr!(dialog_set_label()))
            .default_text(current)
            .on_result(move |r, ctx| {
                if let Some(label) = r {
                    vm.set_scene_label(ctx, id, label.trim());
                }
            })
            .present(ctx);
    }

    pub fn begin_insert_scene_after(&self, ctx: &mut EventContext, id: u64) {
        let vm = self.clone();
        InputDialog::new(tr!(dialog_new_scene()))
            .placeholder(tr!(placeholder_scene_name()))
            .on_result(move |r, ctx| {
                if let Some(name) = r
                    && !name.trim().is_empty()
                {
                    vm.insert_scene_after(ctx, id, name.trim());
                }
            })
            .present(ctx);
    }

    pub fn begin_add_scene(&self, ctx: &mut EventContext) {
        let vm = self.clone();
        InputDialog::new(tr!(dialog_new_scene()))
            .placeholder(tr!(placeholder_scene_name()))
            .on_result(move |r, ctx| {
                if let Some(name) = r
                    && !name.trim().is_empty()
                {
                    vm.add_scene(ctx, name.trim());
                }
            })
            .present(ctx);
    }

    // ── apply methods (undoable backend calls; unit-tested) ──

    pub fn rename_chapter(&self, _ctx: &mut EventContext, title: &str) {
        if let Some(it) = self.item_dto(self.inner.chapter_id) {
            let mut dto = update_item_dto(&it);
            dto.title = title.to_string();
            let _ =
                binder_item_commands::update_binder_item(&self.inner.app_ctx, self.stack(), &dto);
            self.inner.chapter_probe.set_id(Some(self.inner.chapter_id));
        }
    }

    pub fn rename_scene(&self, _ctx: &mut EventContext, id: u64, title: &str) {
        if let Some(it) = self.item_dto(id) {
            let mut dto = update_item_dto(&it);
            dto.title = title.to_string();
            let _ =
                binder_item_commands::update_binder_item(&self.inner.app_ctx, self.stack(), &dto);
            self.scene(id).refresh_meta();
        }
    }

    pub fn set_scene_label(&self, _ctx: &mut EventContext, id: u64, label: &str) {
        if let Some(it) = self.item_dto(id) {
            let mut dto = update_item_dto(&it);
            dto.label = label.to_string();
            let _ =
                binder_item_commands::update_binder_item(&self.inner.app_ctx, self.stack(), &dto);
            self.scene(id).refresh_meta();
        }
    }

    pub fn insert_scene_after(&self, _ctx: &mut EventContext, id: u64, title: &str) {
        if let Some((binder, _order, pos)) = self.locate(id) {
            let indent = self.item_dto(id).map(|it| it.indent).unwrap_or(0);
            self.create_scene(binder, (pos + 1) as i32, indent, title);
        }
    }

    pub fn add_scene(&self, ctx: &mut EventContext, title: &str) {
        // Append after the chapter's last scene, else right after the chapter head.
        if let Some(&last) = self.inner.model.ids().last() {
            self.insert_scene_after(ctx, last, title);
        } else if let Some((binder, _order, pos)) = self.locate(self.inner.chapter_id) {
            let indent = self
                .item_dto(self.inner.chapter_id)
                .map(|it| it.indent)
                .unwrap_or(0)
                + 1;
            self.create_scene(binder, (pos + 1) as i32, indent, title);
        }
    }

    pub fn move_scene_up(&self, _ctx: &mut EventContext, id: u64) {
        let ids = self.inner.model.ids();
        let Some(pos) = ids.iter().position(|&x| x == id) else {
            return;
        };
        if pos == 0 {
            return;
        }
        self.move_relative(id, ids[pos - 1], MovePlace::Before);
    }

    pub fn move_scene_down(&self, _ctx: &mut EventContext, id: u64) {
        let ids = self.inner.model.ids();
        let Some(pos) = ids.iter().position(|&x| x == id) else {
            return;
        };
        if pos + 1 >= ids.len() {
            return;
        }
        self.move_relative(id, ids[pos + 1], MovePlace::After);
    }

    pub fn merge_into_previous(&self, ctx: &mut EventContext, id: u64) {
        let ids = self.inner.model.ids();
        let Some(pos) = ids.iter().position(|&x| x == id) else {
            return;
        };
        if pos == 0 {
            return;
        }
        let prev = ids[pos - 1];
        // Merge reads content from the store — flush both live docs first.
        let stack = self.stack();
        let _ = self.scene(prev).flush(stack);
        let _ = self.scene(id).flush(stack);
        let _ = binder_item_management_commands::merge_two_scenes(
            &self.inner.app_ctx,
            stack,
            &MergeTwoScenesDto {
                target_id: prev,
                source_id: id,
            },
        );
        // `prev` absorbed `id`'s text — reflect it in the (reused) editor. The
        // `set_djot` only queues a document event; pump a frame so that editor
        // (which the user did not interact with) drains it and repaints.
        self.scene(prev).reload_doc();
        ctx.request_frame();
    }

    /// Split the scene at `caret` into two (the editor context-menu action).
    pub fn split_scene(&self, ctx: &mut EventContext, id: u64, caret: usize) {
        let scene = self.scene(id);
        let doc = scene.main_doc();
        let Ok((before, after)) = split_djot(&doc, caret) else {
            return;
        };
        // Splitting from the prose editor: the synopsis is not cut — it goes whole
        // to the source and empty to the new scene (generalized in `StreamViewModel`).
        // `split_scene` *reassigns* SynopsisText, so passing the empty string here
        // would wipe the source's synopsis.
        let synopsis = self.synopsis_djot(id);
        let _ = binder_item_management_commands::split_scene(
            &self.inner.app_ctx,
            self.stack(),
            &SplitSceneDto {
                source_id: id,
                before_text: before,
                after_text: after,
                before_synopsis: synopsis,
                after_synopsis: String::new(),
                new_title: "New Scene".to_string(),
            },
        );
        // `id` now holds only the before-text — reflect it in the (reused) editor,
        // pumping a frame so the queued document event is drained.
        scene.reload_doc();
        ctx.request_frame();
    }

    pub fn trash_scene(&self, _ctx: &mut EventContext, id: u64) {
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

    /// Persist every loaded scene's edits (called from `ContentTab::flush`).
    pub fn flush_all(&self, stack: Option<u64>) -> anyhow::Result<()> {
        for s in self.inner.scenes.borrow().values() {
            s.flush(stack)?;
        }
        Ok(())
    }

    // ── helpers ──

    fn stack(&self) -> Option<u64> {
        self.inner.ids.stack_id.get()
    }

    /// The scene's persisted `SynopsisText`, so a prose split can pass it back
    /// unchanged. `SingleScene` only owns the `SceneText` document; this whole
    /// view-model is superseded by `StreamViewModel`, whose rows hold the shared
    /// `OpenDoc` (both roles) and need no such lookup.
    fn synopsis_djot(&self, id: u64) -> String {
        let content_ids = binder_item_commands::get_binder_item_relationship(
            &self.inner.app_ctx,
            &id,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap_or_default();
        content_commands::get_content_multi(&self.inner.app_ctx, &content_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .find(|c| c.role == ContentRole::SynopsisText)
            .map(|c| c.data)
            .unwrap_or_default()
    }

    fn item_dto(&self, id: u64) -> Option<BinderItemDto> {
        binder_item_commands::get_binder_item(&self.inner.app_ctx, &id)
            .ok()
            .flatten()
    }

    fn scene_pos(&self, id: u64) -> Option<usize> {
        self.inner.model.ids().iter().position(|&x| x == id)
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

    fn create_scene(&self, binder: u64, index: i32, indent: i64, title: &str) {
        let dto = CreateBinderItemDto {
            title: title.to_string(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
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
            index,
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

/// Build a scalar-only `UpdateBinderItemDto` from a fetched item (mirrors the
/// outline's helper).
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
