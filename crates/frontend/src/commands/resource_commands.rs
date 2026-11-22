// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Resource entity commands
#![allow(unused_imports, dead_code)]

use crate::app_context::AppContext;
use anyhow::{Context, Result};
use common::types::EntityId;
use direct_access::{CreateResourceDto, ResourceDto, UpdateResourceDto, resource_controller};

/// Create a new resource entity (orphan, no parent)
pub fn create_orphan_resource(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &CreateResourceDto,
) -> Result<ResourceDto> {
    let mut undo_redo_manager = common::long_operation::lock_or_recover(&ctx.undo_redo_manager);
    resource_controller::create_orphan(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dto,
    )
    .context("creating resource")
}
/// Create a new resource entity as child of owner
pub fn create_resource(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &CreateResourceDto,
    owner_id: EntityId,
    index: i32,
) -> Result<ResourceDto> {
    let mut undo_redo_manager = common::long_operation::lock_or_recover(&ctx.undo_redo_manager);
    resource_controller::create(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dto,
        owner_id,
        index,
    )
    .context("creating resource")
}
/// Create multiple resource entities (orphan, no parent)
pub fn create_orphan_resource_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dtos: &[CreateResourceDto],
) -> Result<Vec<ResourceDto>> {
    let mut undo_redo_manager = common::long_operation::lock_or_recover(&ctx.undo_redo_manager);
    resource_controller::create_orphan_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dtos,
    )
    .context("creating resource entities")
}
/// Create multiple resource entities as children of owner
pub fn create_resource_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dtos: &[CreateResourceDto],
    owner_id: EntityId,
    index: i32,
) -> Result<Vec<ResourceDto>> {
    let mut undo_redo_manager = common::long_operation::lock_or_recover(&ctx.undo_redo_manager);
    resource_controller::create_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dtos,
        owner_id,
        index,
    )
    .context("creating resource entities")
}
/// Get a resource entity by ID
pub fn get_resource(ctx: &AppContext, id: &EntityId) -> Result<Option<ResourceDto>> {
    resource_controller::get(&ctx.db_context, id).context("getting resource")
}

/// Get multiple resource entities by IDs
pub fn get_resource_multi(ctx: &AppContext, ids: &[EntityId]) -> Result<Vec<Option<ResourceDto>>> {
    resource_controller::get_multi(&ctx.db_context, ids).context("getting resource entities")
}

/// Get all resource entities.
/// Note: returns entities in database key order (by EntityId), not insertion order
/// or any user-defined sort. For ordered collections, use relationship-based
/// retrieval (e.g. get_*_relationship for ordered_one_to_many fields).
pub fn get_all_resource(ctx: &AppContext) -> Result<Vec<ResourceDto>> {
    resource_controller::get_all(&ctx.db_context).context("getting all resource entities")
}

/// Update a resource entity
pub fn update_resource(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &UpdateResourceDto,
) -> Result<ResourceDto> {
    let mut undo_redo_manager = common::long_operation::lock_or_recover(&ctx.undo_redo_manager);
    resource_controller::update(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dto,
    )
    .context("updating resource")
}

/// Update multiple resource entities
pub fn update_resource_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dtos: &[UpdateResourceDto],
) -> Result<Vec<ResourceDto>> {
    let mut undo_redo_manager = common::long_operation::lock_or_recover(&ctx.undo_redo_manager);
    resource_controller::update_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dtos,
    )
    .context("updating resource entities")
}

/// Update a resource entity with relationships
pub fn update_resource_with_relationships(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dto: &ResourceDto,
) -> Result<ResourceDto> {
    let mut undo_redo_manager = common::long_operation::lock_or_recover(&ctx.undo_redo_manager);
    resource_controller::update_with_relationships(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dto,
    )
    .context("updating resource with relationships")
}

/// Update multiple resource entities with relationships
pub fn update_resource_with_relationships_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    dtos: &[ResourceDto],
) -> Result<Vec<ResourceDto>> {
    let mut undo_redo_manager = common::long_operation::lock_or_recover(&ctx.undo_redo_manager);
    resource_controller::update_with_relationships_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        dtos,
    )
    .context("updating resource entities with relationships")
}

/// Remove a resource entity by ID
pub fn remove_resource(ctx: &AppContext, stack_id: Option<u64>, id: &EntityId) -> Result<()> {
    let mut undo_redo_manager = common::long_operation::lock_or_recover(&ctx.undo_redo_manager);
    resource_controller::remove(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        id,
    )
    .context("removing resource")
}

/// Remove multiple resource entities by IDs
pub fn remove_resource_multi(
    ctx: &AppContext,
    stack_id: Option<u64>,
    ids: &[EntityId],
) -> Result<()> {
    let mut undo_redo_manager = common::long_operation::lock_or_recover(&ctx.undo_redo_manager);
    resource_controller::remove_multi(
        &ctx.db_context,
        &ctx.event_hub,
        &mut undo_redo_manager,
        stack_id,
        ids,
    )
    .context("removing resource entities")
}
