// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use crate::use_cases::add_resources_uc::{
    AddResourcesUnitOfWorkFactoryTrait, AddResourcesUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::CommandUnitOfWork;
use common::database::write_guard::WriteTransactionGuard;
use common::database::{db_context::DbContext, transactions::Transaction};
#[allow(unused_imports)]
use common::entities::{Binder, BinderItem, Resource, Work};
use common::event::ResourceManagementEvent::AddResources;
use common::event::{AllEvent, DirectAccessEntity, Event, EventBuffer, EventHub, Origin};
#[allow(unused_imports)]
use common::types;
#[allow(unused_imports)]
use common::types::EntityId;
use std::cell::RefCell;
use std::sync::Arc;

// Unit of work for AddResources

pub struct AddResourcesUnitOfWork {
    context: DbContext,
    transaction: Option<Transaction>,
    // RAII: acquired in `begin_transaction`, released via `commit`/`rollback`
    // clearing it — see `common::database::write_guard`'s module doc for the
    // single-write-transaction-per-store invariant this enforces.
    write_guard: Option<WriteTransactionGuard>,
    event_hub: Arc<EventHub>,
    event_buffer: RefCell<EventBuffer>,
}

impl AddResourcesUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        AddResourcesUnitOfWork {
            context: db_context.clone(),
            transaction: None,
            write_guard: None,
            event_hub: event_hub.clone(),
            event_buffer: RefCell::new(EventBuffer::new()),
        }
    }
}

impl CommandUnitOfWork for AddResourcesUnitOfWork {
    fn begin_transaction(&mut self) -> Result<()> {
        // Acquire BEFORE the write transaction itself — see
        // `common::database::write_guard`'s module doc.
        self.write_guard = Some(WriteTransactionGuard::acquire(
            &self.context,
            "add_resources",
        )?);
        self.transaction = Some(Transaction::begin_write_transaction(&self.context)?);
        self.event_buffer.get_mut().begin_buffering();
        Ok(())
    }

    fn commit(&mut self) -> Result<()> {
        self.transaction
            .take()
            .ok_or_else(|| anyhow::anyhow!("No active transaction"))?
            .commit()?;
        self.write_guard = None;
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
        self.write_guard = None;
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

        // Discard buffered events — savepoint restore invalidated them
        self.event_buffer.get_mut().discard();

        // Send Reset immediately (not buffered — UI must refresh now)
        self.event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::All(AllEvent::Reset)),
            ids: vec![],
            data: None,
        });

        // Recreate the transaction after restoring to savepoint
        self.transaction = Some(transaction);

        Ok(())
    }
}

//TODO: adapt entities and actions to real use :
// Create, CreateMulti, Get, GetMulti, Update (scalar-only), UpdateMulti (scalar-only),
// UpdateWithRelationships, UpdateWithRelationshipsMulti,
// Remove, RemoveMulti, GetRelationship, GetRelationshipsFromRightIds,
// SetRelationship, SetRelationshipMulti, Snapshot, Restore
//
// You have here a read-write unit of work.
//
// RO means Read Only, so *RO actions should not used be here.
// Don't forget to set thread_safe = true for long operation's unit of work.
// Do not mix read-only and write actions in the same unit of work.
//
// Exactly the same macros must be set in the use case uow trait file in ../use_cases/add_resources_uc.rs
//
// The same macro set must appear on the trait in ../use_cases/add_resources_uc.rs.
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "Work", action = "Snapshot")]
#[macros::uow_action(entity = "Work", action = "Restore")]
#[macros::uow_action(entity = "Binder", action = "GetMulti")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "SetRelationship")]
#[macros::uow_action(entity = "Resource", action = "CreateOrphan")]
impl AddResourcesUnitOfWorkTrait for AddResourcesUnitOfWork {
    fn keep_prose(
        &self,
        target: &common::snapshot::EntityTreeSnapshot,
        other: &common::snapshot::EntityTreeSnapshot,
    ) -> Result<common::prose_keep::KeptProse> {
        common::prose_keep::keep(self.transaction.as_ref(), target, other)
    }

    fn put_back_prose(&self, kept: common::prose_keep::KeptProse) -> Result<()> {
        common::prose_keep::put_back(self.transaction.as_ref(), &self.event_buffer, kept)
    }

    fn publish_add_resources_event(&self, ids: Vec<EntityId>, data: Option<String>) {
        self.event_hub.send_event(Event {
            origin: Origin::ResourceManagement(AddResources),
            ids,
            data,
        });
    }
}

pub struct AddResourcesUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl AddResourcesUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        AddResourcesUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl AddResourcesUnitOfWorkFactoryTrait for AddResourcesUnitOfWorkFactory {
    fn create(&self) -> Box<dyn AddResourcesUnitOfWorkTrait> {
        Box::new(AddResourcesUnitOfWork::new(&self.context, &self.event_hub))
    }
}
