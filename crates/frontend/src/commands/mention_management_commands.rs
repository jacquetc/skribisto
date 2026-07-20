// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! MentionManagement feature commands
//!
//! Shaped exactly like `progress_management_commands`: one long operation, started by id and
//! polled for progress and result. The scan is read-only, so there is no write command here
//! — pinning a suggestion goes through the `references` relationship on `BinderItem`, not
//! through this feature.

use crate::app_context::AppContext;
use anyhow::{Context, Result};
use mention_management::{MentionScanResultDto, mention_management_controller};

use common::long_operation::OperationProgress;

/// scan_mentions (long operation)
pub fn scan_mentions(ctx: &AppContext) -> Result<String> {
    mention_management_controller::scan_mentions(
        &ctx.db_context,
        &ctx.event_hub,
        &mut ctx.long_operation_manager.lock().unwrap(),
    )
    .context("scan_mentions")
}

/// Get the progress of a scan_mentions operation
pub fn get_scan_mentions_progress(
    ctx: &AppContext,
    operation_id: &str,
) -> Option<OperationProgress> {
    mention_management_controller::get_scan_mentions_progress(
        &ctx.long_operation_manager.lock().unwrap(),
        operation_id,
    )
}

/// Get the result of a scan_mentions operation
pub fn get_scan_mentions_result(
    ctx: &AppContext,
    operation_id: &str,
) -> Result<Option<MentionScanResultDto>> {
    mention_management_controller::get_scan_mentions_result(
        &ctx.long_operation_manager.lock().unwrap(),
        operation_id,
    )
    .context("getting scan_mentions result")
}
