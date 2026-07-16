// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom UoW for SplitScene — action macros must match the trait in
// ../use_cases/split_scene_uc.rs.

use crate::use_cases::split_scene_uc::{
    SplitSceneUnitOfWorkFactoryTrait, SplitSceneUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::CommandUnitOfWork;
use common::database::{db_context::DbContext, transactions::Transaction};
#[allow(unused_imports)]
use common::entities::{Binder, BinderItem, Content};
use common::event::BinderItemManagementEvent::SplitScene;
use common::event::{AllEvent, DirectAccessEntity, Event, EventBuffer, EventHub, Origin};
#[allow(unused_imports)]
use common::types;
#[allow(unused_imports)]
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

// Unit of work for SplitScene

pub struct SplitSceneUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    event_hub: Arc<EventHub>,
    event_buffer: RefCell<EventBuffer>,
}

impl SplitSceneUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        SplitSceneUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            event_hub: event_hub.clone(),
            event_buffer: RefCell::new(EventBuffer::new()),
        }
    }
}

impl CommandUnitOfWork for SplitSceneUnitOfWork {
    fn begin_transaction(&mut self) -> Result<()> {
        self.transaction = Some(Transaction::begin_write_transaction(&self.context)?);
        self.event_buffer.get_mut().begin_buffering();
        Ok(())
    }

    fn commit(&mut self) -> Result<()> {
        self.transaction
            .take()
            .ok_or_else(|| anyhow::anyhow!("No active transaction"))?
            .commit()?;
        for event in self.event_buffer.get_mut().flush() {
            self.event_hub.send_event(event);
        }
        Ok(())
    }

    fn rollback(&mut self) -> Result<()> {
        self.transaction
            .take()
            .ok_or_else(|| anyhow::anyhow!("No active transaction"))?
            .rollback()?;
        self.event_buffer.get_mut().discard();
        Ok(())
    }

    fn create_savepoint(&self) -> Result<types::Savepoint> {
        self.transaction
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No active transaction"))?
            .create_savepoint()
    }

    fn restore_to_savepoint(&mut self, savepoint: types::Savepoint) -> Result<()> {
        let mut transaction = self
            .transaction
            .take()
            .ok_or_else(|| anyhow::anyhow!("No active transaction"))?;
        transaction.restore_to_savepoint(savepoint)?;
        self.event_buffer.get_mut().discard();
        self.event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::All(AllEvent::Reset)),
            ids: vec![],
            data: None,
        });
        self.transaction = Some(transaction);
        Ok(())
    }
}

// Same macro set as the trait in ../use_cases/split_scene_uc.rs.
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "Binder", action = "GetRelationshipsFromRightIds")]
#[macros::uow_action(entity = "Binder", action = "SetRelationship")]
#[macros::uow_action(entity = "Binder", action = "Snapshot")]
#[macros::uow_action(entity = "Binder", action = "Restore")]
#[macros::uow_action(entity = "BinderItem", action = "GetMulti")]
#[macros::uow_action(entity = "BinderItem", action = "CreateOrphan")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "SetRelationship")]
#[macros::uow_action(entity = "Content", action = "GetMulti")]
#[macros::uow_action(entity = "Content", action = "Update")]
#[macros::uow_action(entity = "Content", action = "CreateOrphan")]
impl SplitSceneUnitOfWorkTrait for SplitSceneUnitOfWork {
    fn publish_split_scene_event(&self, ids: Vec<EntityId>, data: Option<String>) {
        self.event_hub.send_event(Event {
            origin: Origin::BinderItemManagement(SplitScene),
            ids,
            data,
        });
    }
}

pub struct SplitSceneUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl SplitSceneUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        SplitSceneUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl SplitSceneUnitOfWorkFactoryTrait for SplitSceneUnitOfWorkFactory {
    fn create(&self) -> Box<dyn SplitSceneUnitOfWorkTrait> {
        Box::new(SplitSceneUnitOfWork::new(&self.context, &self.event_hub))
    }
}
