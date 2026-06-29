//! The only mutable state the app itself holds: entity **ids**.
//!
//! Per the writing-model architecture, the UI reads entity *data* reactively
//! through [`singles`](crate::singles) and [`models`](crate::models); the app
//! itself keeps only the handful of ids those handles point at — the current
//! `Root`, the open `Work`, its `WorkInfo`, and the per-`Work` undo stack.
//!
//! Seeded once per `LoadWork` (see `App::build`), shared everywhere by clone, and
//! registered as `app_state` so any widget can reach it via
//! `ctx.app_state::<AppIds>()`. Singles/models read these signals to know what to
//! point at and refresh themselves on fine-grained backend events thereafter.

use bastyde::prelude::Signal;

use frontend::AppContext;
use frontend::commands::{root_commands, undo_redo_commands, work_commands, work_info_commands};

/// The app's id-only global state. Cloneable (every field is an `Rc`-backed
/// `Signal`), so all clones share one live state.
#[derive(Clone)]
pub struct AppIds {
    pub root_id: Signal<Option<u64>>,
    pub work_id: Signal<Option<u64>>,
    pub work_info_id: Signal<Option<u64>>,
    /// Per-`Work` undo stack id — one Ctrl+Z history for the whole undoable trunk.
    pub stack_id: Signal<Option<u64>>,
}

impl Default for AppIds {
    fn default() -> Self {
        Self {
            root_id: Signal::new(None),
            work_id: Signal::new(None),
            work_info_id: Signal::new(None),
            stack_id: Signal::new(None),
        }
    }
}

impl AppIds {
    pub fn new() -> Self {
        Self::default()
    }

    /// Bootstrap the entity ids from the freshly-loaded project. The single
    /// `get_all_*` sweep run on each `LoadWork`; thereafter everything downstream
    /// is id-driven (singles/models read these signals and self-refresh on
    /// entity events).
    pub fn seed(&self, ctx: &AppContext) {
        self.root_id
            .set(first_id(root_commands::get_all_root(ctx).ok(), |r| r.id));
        self.work_id
            .set(first_id(work_commands::get_all_work(ctx).ok(), |w| w.id));
        self.work_info_id.set(first_id(
            work_info_commands::get_all_work_info(ctx).ok(),
            |wi| wi.id,
        ));
    }

    /// Open a fresh per-`Work` undo stack and record its id. Call on `LoadWork`.
    pub fn open_stack(&self, ctx: &AppContext) {
        let id = undo_redo_commands::create_new_stack(ctx);
        self.stack_id.set(Some(id));
    }

    /// Forget all ids — no work is open. Call on `CloseWork`.
    pub fn clear(&self) {
        self.root_id.set(None);
        self.work_id.set(None);
        self.work_info_id.set(None);
        self.stack_id.set(None);
    }
}

/// The id of the first element of a `get_all_*` result, if any.
fn first_id<T>(list: Option<Vec<T>>, id_of: impl Fn(&T) -> u64) -> Option<u64> {
    list?.first().map(id_of)
}
