// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `clear_titles` — blanking a title in both the places a title lives.
//!
//! The whole reason this is a use case is that a title has two homes:
//! `BinderItem.title`, which the outline and the tab read, and the title
//! `Content` row the constraint matrix gives that combination, which is what
//! the exporter compiles. Every test here is therefore about the *pair* — that
//! both are written, that a combination carrying no title row is still handled,
//! and that undo brings both back together. A test that checked only the entity
//! field would pass against the bug this exists to prevent.

use std::sync::Arc;

use binder_item_management::ClearTitlesDto;
use binder_item_management::binder_item_management_controller as feature;
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
    binder_id: EntityId,
    /// The feature's own undo stack — see `subtree_writes.rs` for why the
    /// `direct_access` seeding must not share it.
    stack: u64,
}

impl Ctx {
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
        let stack = undo.create_new_stack();
        Ctx {
            db,
            hub,
            undo,
            binder_id,
            stack,
        }
    }

    /// A row of the given type, titled, plus its matching title `Content` row
    /// holding the same text — the state the new-project template leaves behind.
    fn titled(
        &mut self,
        sub_role: BinderItemSubRole,
        content_role: Option<ContentRole>,
        title: &str,
    ) -> EntityId {
        let binder_id = self.binder_id;
        let item = binder_item_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateBinderItemDto {
                role: BinderItemRole::Folder,
                sub_role,
                activated: true,
                title: title.to_string(),
                ..Default::default()
            },
            binder_id,
            -1,
        )
        .expect("binder_item")
        .id;
        if let Some(role) = content_role {
            content_controller::create(
                &self.db,
                &self.hub,
                &mut self.undo,
                None,
                &CreateContentDto {
                    role,
                    data: title.to_string(),
                    ..Default::default()
                },
                item,
                -1,
            )
            .expect("content");
        }
        item
    }

    fn title(&self, id: EntityId) -> String {
        binder_item_controller::get(&self.db, &id)
            .expect("get")
            .expect("row still present")
            .title
    }

    /// The data of the item's `Content` row for `role`, or `None` if it has none.
    fn content(&self, id: EntityId, role: &ContentRole) -> Option<String> {
        let ids = binder_item_controller::get_relationship(
            &self.db,
            &id,
            &common::direct_access::binder_item::BinderItemRelationshipField::Contents,
        )
        .expect("contents");
        content_controller::get_multi(&self.db, &ids)
            .expect("get_multi")
            .into_iter()
            .flatten()
            .find(|c| &c.role == role)
            .map(|c| c.data)
    }

    fn clear(&mut self, ids: &[EntityId]) -> Vec<EntityId> {
        let stack = Some(self.stack);
        feature::clear_titles(
            &self.db,
            &self.hub,
            &mut self.undo,
            stack,
            &ClearTitlesDto {
                item_ids: ids.to_vec(),
            },
        )
        .expect("clear_titles")
        .cleared_ids
    }
}

#[test]
fn clears_both_homes_of_the_title() {
    let mut ctx = Ctx::new();
    let chapter = ctx.titled(
        BinderItemSubRole::ChapterScene,
        Some(ContentRole::ChapterTitle),
        "Chapter 3",
    );

    ctx.clear(&[chapter]);

    assert_eq!(ctx.title(chapter), "");
    assert_eq!(
        ctx.content(chapter, &ContentRole::ChapterTitle).as_deref(),
        Some(""),
        "the row the exporter compiles must not keep a heading the binder dropped"
    );
}

#[test]
fn a_combination_with_no_title_row_clears_its_entity_field_alone() {
    let mut ctx = Ctx::new();
    // `Folder/None` is a plain organising folder: the matrix gives it no title
    // `Content` role at all, so `title_role_of` returns `None`.
    let folder = ctx.titled(BinderItemSubRole::None, None, "Act One");

    let cleared = ctx.clear(&[folder]);

    assert_eq!(cleared, vec![folder]);
    assert_eq!(ctx.title(folder), "");
}

#[test]
fn a_missing_content_row_is_not_minted_to_hold_nothing() {
    let mut ctx = Ctx::new();
    // A book that carries a title in the entity field but has no title row yet.
    let book = ctx.titled(BinderItemSubRole::Book, None, "Untitled Book");

    ctx.clear(&[book]);

    assert_eq!(ctx.title(book), "");
    assert!(
        ctx.content(book, &ContentRole::BookTitle).is_none(),
        "an empty row the writer never authored would be carried into the bundle"
    );
}

#[test]
fn cleared_ids_reports_only_rows_that_had_something_to_clear() {
    let mut ctx = Ctx::new();
    let titled = ctx.titled(
        BinderItemSubRole::ChapterScene,
        Some(ContentRole::ChapterTitle),
        "Chapter 1",
    );
    let already_blank = ctx.titled(BinderItemSubRole::ChapterScene, None, "");

    let cleared = ctx.clear(&[titled, already_blank]);

    assert_eq!(cleared, vec![titled]);
}

#[test]
fn a_row_whose_two_homes_disagree_is_still_cleared() {
    let mut ctx = Ctx::new();
    // Exactly the drift this use case exists to stop: the entity field was
    // blanked at some point but the compiled heading was left behind.
    let chapter = ctx.titled(
        BinderItemSubRole::ChapterScene,
        Some(ContentRole::ChapterTitle),
        "Chapter 7",
    );
    let mut item = binder_item_controller::get(&ctx.db, &chapter)
        .unwrap()
        .unwrap();
    item.title.clear();
    binder_item_controller::update(&ctx.db, &ctx.hub, &mut ctx.undo, None, &item.into())
        .expect("blank only the entity field");

    let cleared = ctx.clear(&[chapter]);

    assert_eq!(
        cleared,
        vec![chapter],
        "an empty entity field must not be read as 'nothing to do' while the row still holds a heading"
    );
    assert_eq!(
        ctx.content(chapter, &ContentRole::ChapterTitle).as_deref(),
        Some("")
    );
}

#[test]
fn undo_brings_both_homes_back_and_redo_clears_them_again() {
    let mut ctx = Ctx::new();
    let chapter = ctx.titled(
        BinderItemSubRole::ChapterScene,
        Some(ContentRole::ChapterTitle),
        "Chapter 3",
    );
    ctx.clear(&[chapter]);

    ctx.undo.undo(Some(ctx.stack)).expect("undo");
    assert_eq!(ctx.title(chapter), "Chapter 3");
    assert_eq!(
        ctx.content(chapter, &ContentRole::ChapterTitle).as_deref(),
        Some("Chapter 3"),
        "the scoped snapshot covers the Content rows because BinderItem owns them"
    );

    ctx.undo.redo(Some(ctx.stack)).expect("redo");
    assert_eq!(ctx.title(chapter), "");
    assert_eq!(
        ctx.content(chapter, &ContentRole::ChapterTitle).as_deref(),
        Some("")
    );
}

#[test]
fn several_rows_clear_as_one_step() {
    let mut ctx = Ctx::new();
    let a = ctx.titled(
        BinderItemSubRole::ChapterScene,
        Some(ContentRole::ChapterTitle),
        "Chapter 1",
    );
    let b = ctx.titled(
        BinderItemSubRole::ChapterScene,
        Some(ContentRole::ChapterTitle),
        "Chapter 2",
    );

    assert_eq!(ctx.clear(&[a, b]), vec![a, b]);

    // One undo, not two — the point of moving the loop below the UI.
    ctx.undo.undo(Some(ctx.stack)).expect("undo");
    assert_eq!(ctx.title(a), "Chapter 1");
    assert_eq!(ctx.title(b), "Chapter 2");
}

#[test]
fn an_empty_request_is_a_no_op() {
    let mut ctx = Ctx::new();
    assert!(ctx.clear(&[]).is_empty());
}
