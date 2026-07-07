// Hand-written to match the Qleany frontend command pattern (the generator only
// wrote the feature crate; this thin wrapper is the aggregate wiring).

//! ImportManagement feature commands

use crate::app_context::AppContext;
use anyhow::{Context, Result};
use import_management::{
    ImportPlumeCreatorFileDto, ImportPlumeCreatorFileResultDto, import_management_controller,
};

/// Convert a Plume Creator (`.plume`) project into a newest-version `.skrib`
/// bundle at `dto.output_path`. Synchronous (a pure file→file transform); the UI
/// then offers to open the result via the existing `load_work` command.
pub fn import_plume_creator_file(
    ctx: &AppContext,
    dto: &ImportPlumeCreatorFileDto,
) -> Result<ImportPlumeCreatorFileResultDto> {
    import_management_controller::import_plume_creator_file(&ctx.db_context, &ctx.event_hub, dto)
        .context("import_plume_creator_file")
}
