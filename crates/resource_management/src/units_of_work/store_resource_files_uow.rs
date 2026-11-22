// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use crate::use_cases::store_resource_files_uc::{
    StoreResourceFilesUnitOfWorkFactoryTrait, StoreResourceFilesUnitOfWorkTrait,
};
use anyhow::{Ok, Result};
use common::database::QueryUnitOfWork;
use common::database::{db_context::DbContext, transactions::Transaction};
#[allow(unused_imports)]
use common::entities::{};
use common::event::ResourceManagementEvent::StoreResourceFiles;
use common::event::{Event, EventHub, Origin};
#[allow(unused_imports)]
use common::types;
#[allow(unused_imports)]
use common::types::EntityId;
use std::sync::Arc;
use std::sync::Mutex;

pub struct StoreResourceFilesUnitOfWork {
    context: DbContext,
    transaction: Mutex<Option<Transaction>>,
    event_hub: Arc<EventHub>,
}

impl StoreResourceFilesUnitOfWork {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        StoreResourceFilesUnitOfWork {
            context: db_context.clone(),
            transaction: Mutex::new(None),
            event_hub: event_hub.clone(),
        }
    }
}

impl QueryUnitOfWork for StoreResourceFilesUnitOfWork {
    fn begin_transaction(&self) -> Result<()> {
        let mut transaction = common::long_operation::lock_or_recover(&self.transaction);
        *transaction = Some(Transaction::begin_read_transaction(&self.context)?);
        Ok(())
    }

    fn end_transaction(&self) -> Result<()> {
        let mut transaction = common::long_operation::lock_or_recover(&self.transaction);
        transaction
            .take()
            .ok_or_else(|| anyhow::anyhow!("No active transaction"))?
            .end_read_transaction()?;
        Ok(())
    }
}
//TODO: adapt entities and actions to real use :
// GetRO, GetMultiRO, GetRelationshipRO, GetRelationshipsFromRightIdsRO
//
// You have here a long operation read-only unit of work.
//
// RO means Read Only, so *RO actions should be used here.
// Don't forget to set thread_safe = true for long operation's unit of work.
// Do not mix read-only and write actions in the same unit of work.
//
// Exactly the same macros (without thread_safe) must be set in the use case file in ../use_cases/store_resource_files_uc.rs
//
impl StoreResourceFilesUnitOfWorkTrait for StoreResourceFilesUnitOfWork {
    fn publish_store_resource_files_event(&self, ids: Vec<EntityId>, data: Option<String>) {
        self.event_hub.send_event(Event {
            origin: Origin::ResourceManagement(StoreResourceFiles),
            ids,
            data,
        });
    }
}

pub struct StoreResourceFilesUnitOfWorkFactory {
    context: DbContext,
    event_hub: Arc<EventHub>,
}

impl StoreResourceFilesUnitOfWorkFactory {
    pub fn new(db_context: &DbContext, event_hub: &Arc<EventHub>) -> Self {
        StoreResourceFilesUnitOfWorkFactory {
            context: db_context.clone(),
            event_hub: event_hub.clone(),
        }
    }
}

impl StoreResourceFilesUnitOfWorkFactoryTrait for StoreResourceFilesUnitOfWorkFactory {
    fn create(&self) -> Box<dyn StoreResourceFilesUnitOfWorkTrait> {
        Box::new(StoreResourceFilesUnitOfWork::new(
            &self.context,
            &self.event_hub,
        ))
    }
}
