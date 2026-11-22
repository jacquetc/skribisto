// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use crate::AddResourcesDto;
use crate::AddResourcesResultDto;
use crate::StoreResourceFilesDto;
use crate::StoredResourceFilesDto;
use crate::units_of_work::add_resources_uow::AddResourcesUnitOfWorkFactory;
use crate::units_of_work::store_resource_files_uow::StoreResourceFilesUnitOfWorkFactory;
use crate::use_cases::add_resources_uc::AddResourcesUseCase;
use crate::use_cases::store_resource_files_uc::StoreResourceFilesUseCase;
use anyhow::Result;

use common::long_operation::{LongOperationManager, OperationProgress};
use common::undo_redo::UndoRedoManager;
use common::{database::db_context::DbContext, event::EventHub};
use std::sync::Arc;

pub fn store_resource_files(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    long_operation_manager: &mut LongOperationManager,
    dto: &StoreResourceFilesDto,
) -> Result<String> {
    let uow_context = StoreResourceFilesUnitOfWorkFactory::new(db_context, event_hub);
    let uc = StoreResourceFilesUseCase::new(Box::new(uow_context), dto);
    let operation_id = long_operation_manager.start_operation(uc);
    Ok(operation_id)
}

pub fn get_store_resource_files_progress(
    long_operation_manager: &LongOperationManager,
    operation_id: &str,
) -> Option<OperationProgress> {
    long_operation_manager.get_operation_progress(operation_id)
}

pub fn get_store_resource_files_result(
    long_operation_manager: &LongOperationManager,
    operation_id: &str,
) -> Result<Option<StoredResourceFilesDto>> {
    // Get the operation result as a JSON string
    let result_json = long_operation_manager.get_operation_result(operation_id);

    // If there's no result, return None
    if result_json.is_none() {
        return Ok(None);
    }
    // Parse the JSON string into a StoredResourceFilesDto
    let result_dto: StoredResourceFilesDto = serde_json::from_str(&result_json.unwrap())?;

    Ok(Some(result_dto))
}

pub fn add_resources(
    db_context: &DbContext,
    event_hub: &Arc<EventHub>,
    undo_redo_manager: &mut UndoRedoManager,
    stack_id: Option<u64>,
    dto: &AddResourcesDto,
) -> Result<AddResourcesResultDto> {
    let uow_context = AddResourcesUnitOfWorkFactory::new(db_context, event_hub);
    let mut uc = AddResourcesUseCase::new(Box::new(uow_context));
    let return_dto = uc.execute(dto)?;
    undo_redo_manager.add_command_to_stack(Box::new(uc), stack_id)?;
    Ok(return_dto)
}
