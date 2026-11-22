// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! ResourceManagement feature commands

use crate::app_context::AppContext;
use anyhow::{Context, Result};
use resource_management::{
    AddResourcesDto, AddResourcesResultDto, StoreResourceFilesDto, StoredResourceFilesDto,
    resource_management_controller,
};

use common::long_operation::OperationProgress;

/// store_resource_files (long operation)
pub fn store_resource_files(ctx: &AppContext, dto: &StoreResourceFilesDto) -> Result<String> {
    resource_management_controller::store_resource_files(
        &ctx.db_context,
        &ctx.event_hub,
        &mut common::long_operation::lock_or_recover(&ctx.long_operation_manager),
        dto,
    )
    .context("store_resource_files")
}

/// Get the progress of a store_resource_files operation
pub fn get_store_resource_files_progress(
    ctx: &AppContext,
    operation_id: &str,
) -> Option<OperationProgress> {
    resource_management_controller::get_store_resource_files_progress(
        &common::long_operation::lock_or_recover(&ctx.long_operation_manager),
        operation_id,
    )
}

/// Get the result of a store_resource_files operation
pub fn get_store_resource_files_result(
    ctx: &AppContext,
    operation_id: &str,
) -> Result<Option<StoredResourceFilesDto>> {
    resource_management_controller::get_store_resource_files_result(
        &common::long_operation::lock_or_recover(&ctx.long_operation_manager),
        operation_id,
    )
    .context("getting store_resource_files result")
}

pub fn add_resources(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &AddResourcesDto,
) -> Result<AddResourcesResultDto> {
    let mut undo_redo_manager = common::long_operation::lock_or_recover(&ctx.undo_redo_manager);
    resource_management_controller::add_resources(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dto,
    )
    .context("add_resources")
}
