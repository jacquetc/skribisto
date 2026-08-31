// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! End-to-end regression for the `Footnote.content` reanchoring finding:
//! `split_scene` and `merge_two_scenes` relocate a scene's prose between
//! `Content` rows, and `Footnote.content` — a static pointer set once at
//! creation — used to never follow it. `crate::footnote_reanchor`'s own unit
//! tests cover the *decision* logic in isolation; this file proves the two use
//! cases actually wire it up against a real backend: a real `Footnote` entity,
//! created through the same `direct_access` layer the app uses, really does end
//! up pointing at the right `Content` row after a real split or merge — and
//! that undoing the operation really does put it back, including the tricky
//! part of `split_scene`'s fix: `Footnote` hangs off `Work`, not `Binder`, so
//! its reanchor needed its own snapshot/restore pair alongside the existing
//! binder-subtree one (see `split_scene_uc.rs`'s own comment on
//! `footnote_snap_before`/`footnote_snap_after`).

use std::sync::Arc;

use binder_item_management::binder_item_management_controller;
use binder_item_management::{MergeTwoScenesDto, SplitSceneDto};
use common::database::db_context::DbContext;
use common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use common::event::EventHub;
use common::types::EntityId;
use common::undo_redo::UndoRedoManager;
use direct_access::binder::binder_controller;
use direct_access::binder::dtos::CreateBinderDto;
use direct_access::binder_item::binder_item_controller;
use direct_access::binder_item::dtos::CreateBinderItemDto;
use direct_access::content::content_controller;
use direct_access::content::dtos::CreateContentDto;
use direct_access::footnote::dtos::CreateFootnoteDto;
use direct_access::footnote::footnote_controller;
use direct_access::root::dtos::CreateRootDto;
use direct_access::root::root_controller;
use direct_access::smart_punctuation::dtos::CreateSmartPunctuationDto;
use direct_access::smart_punctuation::smart_punctuation_controller;
use direct_access::work::dtos::CreateWorkDto;
use direct_access::work::work_controller;

struct Ctx {
    db: DbContext,
    hub: Arc<EventHub>,
    undo: UndoRedoManager,
    work_id: EntityId,
    binder_id: EntityId,
}

impl Ctx {
    /// One open Work with one Binder — built straight through `direct_access`,
    /// the same bypass `tests/work_scoping.rs` uses to reach a real backend
    /// without standing up the whole `work_management` load/new flow.
    fn new() -> Self {
        let db = DbContext::new().expect("in-memory store");
        let hub = Arc::new(EventHub::new());
        let mut undo = UndoRedoManager::new();
        undo.set_event_hub(&hub);
        let root_id = root_controller::create_orphan(&db, &hub, &CreateRootDto::default())
            .expect("root")
            .id;
        let smart_punctuation_id = smart_punctuation_controller::create_orphan(
            &db,
            &hub,
            &mut undo,
            None,
            &CreateSmartPunctuationDto::default(),
        )
        .expect("smart_punctuation")
        .id;
        let work_id = work_controller::create(
            &db,
            &hub,
            &mut undo,
            None,
            &CreateWorkDto {
                statuses: Vec::new(),
                smart_punctuation: smart_punctuation_id,
                ..Default::default()
            },
            root_id,
            -1,
        )
        .expect("work")
        .id;
        let binder_id = binder_controller::create(
            &db,
            &hub,
            &mut undo,
            None,
            &CreateBinderDto {
                activated: true,
                ..Default::default()
            },
            work_id,
            -1,
        )
        .expect("binder")
        .id;
        Ctx {
            db,
            hub,
            undo,
            work_id,
            binder_id,
        }
    }

    fn new_scene(&mut self) -> EntityId {
        binder_item_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateBinderItemDto {
                status: None,
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Scene,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            self.binder_id,
            -1,
        )
        .expect("binder_item")
        .id
    }

    fn new_scene_text(&mut self, owner_item: EntityId, data: &str) -> EntityId {
        content_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateContentDto {
                activated: true,
                role: ContentRole::SceneText,
                data: data.to_string(),
                ..Default::default()
            },
            owner_item,
            -1,
        )
        .expect("content")
        .id
    }

    fn new_footnote(&mut self, content_id: EntityId, label: &str, body: &str) -> EntityId {
        footnote_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateFootnoteDto {
                content: Some(content_id),
                label: label.to_string(),
                body: body.to_string(),
                ..Default::default()
            },
            self.work_id,
            -1,
        )
        .expect("footnote")
        .id
    }

    fn footnote_content(&self, footnote_id: EntityId) -> Option<EntityId> {
        footnote_controller::get(&self.db, &footnote_id)
            .unwrap()
            .expect("footnote still exists")
            .content
    }

    fn content_data(&self, content_id: EntityId) -> String {
        content_controller::get(&self.db, &content_id)
            .unwrap()
            .expect("content still exists")
            .data
    }
}

/// **The split_scene failure scenario from the finding, end to end.** A
/// footnote's citation sits right after the caret; splitting the scene there
/// must reparent the footnote onto the new, after-caret row — not leave it
/// pointing at the source, which the citation no longer lives in.
#[test]
fn splitting_at_the_footnote_reparents_it_to_the_new_scene() {
    let mut ctx = Ctx::new();
    let source = ctx.new_scene();
    let source_content =
        ctx.new_scene_text(source, "The letter arrived at dawn[^1]. She read it twice.");
    let footnote_id = ctx.new_footnote(source_content, "1", "A note on the letter.");

    binder_item_management_controller::split_scene(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &SplitSceneDto {
            source_id: source,
            before_text: "The letter arrived at dawn.".to_string(),
            after_text: "[^1]. She read it twice.".to_string(),
            before_synopsis: String::new(),
            after_synopsis: String::new(),
            new_title: "Split part".to_string(),
        },
    )
    .expect("split_scene");

    let new_content = ctx
        .footnote_content(footnote_id)
        .expect("the footnote must still be anchored to something");
    assert_ne!(
        new_content, source_content,
        "the citation moved to the new scene — the anchor must move with it"
    );
    assert_eq!(
        ctx.content_data(new_content),
        "[^1]. She read it twice.",
        "the footnote must be anchored to the row that actually holds its \
         citation now"
    );

    // And undo must put it back — `Footnote` is not part of the binder-subtree
    // snapshot `split_scene` otherwise scopes its undo to (see
    // `split_scene_uc.rs`'s own comment on why it carries a second, independent
    // snapshot pair just for this).
    ctx.undo.undo(None).expect("undo");
    assert_eq!(
        ctx.footnote_content(footnote_id),
        Some(source_content),
        "undoing the split must restore the footnote's original anchor, not \
         leave it pointing at a row the binder-subtree undo already deleted"
    );
}

/// A footnote whose citation stayed on the before-caret half must not move —
/// it was already correctly anchored.
#[test]
fn splitting_after_the_footnote_leaves_it_anchored_to_the_source() {
    let mut ctx = Ctx::new();
    let source = ctx.new_scene();
    let source_content =
        ctx.new_scene_text(source, "The letter[^1] arrived at dawn. She read it twice.");
    let footnote_id = ctx.new_footnote(source_content, "1", "A note on the letter.");

    binder_item_management_controller::split_scene(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &SplitSceneDto {
            source_id: source,
            before_text: "The letter[^1] arrived at dawn.".to_string(),
            after_text: "She read it twice.".to_string(),
            before_synopsis: String::new(),
            after_synopsis: String::new(),
            new_title: "Split part".to_string(),
        },
    )
    .expect("split_scene");

    assert_eq!(
        ctx.footnote_content(footnote_id),
        Some(source_content),
        "the citation stayed on the source's own half — the anchor must not move"
    );
}

/// **The merge_two_scenes failure scenario from the finding, end to end.** The
/// source's prose (and its footnote's citation) is folded into the target;
/// the footnote must follow it, or it would resolve to the target's own
/// content, plus a redundant one for the source, but instead used to keep
/// pointing at the source row — which the merge then trashes, dropping the
/// note from every default (non-`include_trashed`) view even though its
/// citation renders live in the surviving scene.
#[test]
fn merging_reparents_the_sources_footnote_onto_the_target() {
    let mut ctx = Ctx::new();
    let target = ctx.new_scene();
    let target_content = ctx.new_scene_text(target, "First part.");
    let source = ctx.new_scene();
    let source_content = ctx.new_scene_text(source, "Second part[^1].");
    let footnote_id = ctx.new_footnote(source_content, "1", "A note on the second part.");

    binder_item_management_controller::merge_two_scenes(
        &ctx.db,
        &ctx.hub,
        &mut ctx.undo,
        None,
        &MergeTwoScenesDto {
            work_id: ctx.work_id,
            target_id: target,
            source_id: source,
        },
    )
    .expect("merge_two_scenes");

    assert_eq!(
        ctx.footnote_content(footnote_id),
        Some(target_content),
        "the citation was folded into the target's row — the anchor must follow it"
    );
    assert_eq!(
        ctx.content_data(target_content),
        "First part.\n\nSecond part[^1].",
        "sanity: the merge actually folded the text this test is anchoring against"
    );

    // Undo is Work-scoped for this use case already (`snapshot_work`/
    // `restore_work`), which is exactly what makes it able to absorb the
    // reanchor for free — no second snapshot pair needed, unlike `split_scene`.
    ctx.undo.undo(None).expect("undo");
    assert_eq!(
        ctx.footnote_content(footnote_id),
        Some(source_content),
        "undoing the merge must restore the footnote's original anchor"
    );
}
