// Hand-written to match the Qleany frontend command pattern (the generator only
// wrote the feature crate; this thin wrapper is the aggregate wiring). Mirrors
// `export_management_commands` — a long-operation feature: start returns an
// operation id, progress/result are polled by id, and the operation is driven on
// a background thread by the shared `LongOperationManager`.

//! ImportManagement feature commands

use crate::app_context::AppContext;
use anyhow::{Context, Result};
use import_management::{
    ImportPlumeCreatorFileDto, ImportPlumeCreatorFileResultDto, import_management_controller,
};

use common::long_operation::OperationProgress;

/// Start converting a Plume Creator (`.plume`) project into a newest-version
/// `.skrib` bundle at `dto.output_path`. Long operation: returns the operation
/// id immediately; the work runs on a background thread and reports progress via
/// `Origin::LongOperation(...)` events. The UI drives a progress + cancel toast
/// from those events, then opens the result via the existing `load_work`.
pub fn import_plume_creator_file(
    ctx: &AppContext,
    dto: &ImportPlumeCreatorFileDto,
) -> Result<String> {
    import_management_controller::import_plume_creator_file(
        &ctx.db_context,
        &ctx.event_hub,
        &mut ctx.long_operation_manager.lock().unwrap(),
        dto,
    )
    .context("import_plume_creator_file")
}

/// Get the progress of an `import_plume_creator_file` operation.
pub fn get_import_plume_creator_file_progress(
    ctx: &AppContext,
    operation_id: &str,
) -> Option<OperationProgress> {
    import_management_controller::get_import_plume_creator_file_progress(
        &ctx.long_operation_manager.lock().unwrap(),
        operation_id,
    )
}

/// Get the result of a completed `import_plume_creator_file` operation.
pub fn get_import_plume_creator_file_result(
    ctx: &AppContext,
    operation_id: &str,
) -> Result<Option<ImportPlumeCreatorFileResultDto>> {
    import_management_controller::get_import_plume_creator_file_result(
        &ctx.long_operation_manager.lock().unwrap(),
        operation_id,
    )
    .context("getting import_plume_creator_file result")
}
