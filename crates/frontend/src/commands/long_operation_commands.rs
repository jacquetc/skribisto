// Hand-written: a thin, feature-agnostic wrapper over the shared
// `LongOperationManager`. Long operations are keyed by an opaque operation id
// (returned by each feature's start command), so cancellation is generic — it
// doesn't belong to any one feature. `cancel_operation` sets the operation's
// cancel flag and the manager emits an `Origin::LongOperation(Cancelled)` event,
// which the UI observes to tear down its progress toast.

//! Cross-cutting long-operation commands (cancellation).

use crate::app_context::AppContext;

/// Request cancellation of the long operation with `operation_id`. Returns
/// `true` if the operation existed (was still tracked). The running use case
/// polls its cancel flag and stops at the next checkpoint; the manager emits a
/// `Cancelled` event either way.
pub fn cancel_operation(ctx: &AppContext, operation_id: &str) -> bool {
    ctx.long_operation_manager
        .lock()
        .unwrap()
        .cancel_operation(operation_id)
}
