//! Layer A — reactive **single-entity** handles (`SingleX`).
//!
//! A *single* mirrors Qleany's C++ `SingleWork`: it holds one entity by id,
//! exposes that entity's fields as reactive `Signal`s (plus
//! [`LoadingStatus`]/`error_message`/`dirty`), **auto-refreshes** from the
//! backend when the entity's `Updated` event fires, and writes edits back via
//! `save()`. Singles sit beside [`models`](crate::models) (which adapt
//! *collections*); together they are the reactive read/write surface the
//! view-models and widgets bind to instead of ad-hoc `get_*` command calls.
//!
//! Each single lives in its own file holding BOTH a real and a mock definition of
//! the same-named type, behind two `#[cfg]`-gated `mod imp` blocks with identical
//! public signatures, then `pub use imp::Name` — so **no `#[cfg]` ever leaks into
//! consuming code**. Parity is enforced by building both feature modes. This is
//! the reference shape for a future Qleany generator (it also generates the C++
//! singles today).
//!
//! Singles point at the ids held in [`AppIds`](crate::app_ids) and are re-pointed
//! on each `LoadWork`; their event subscriptions are installed once from a
//! long-lived widget's `build` via `wire(ctx)` (see `App::build`).

mod single_binder;
mod single_binder_item;
mod single_content;
mod single_milestone;
mod single_work;
mod single_work_info;

pub use single_binder::SingleBinder;
pub use single_binder_item::SingleBinderItem;
pub use single_content::SingleContent;
pub use single_milestone::SingleMilestone;
pub use single_work::SingleWork;
pub use single_work_info::SingleWorkInfo;

/// Loading lifecycle of a single, mirroring the C++ `SingleWork::LoadingStatus`.
///
/// `Loading`/`Error` are only reached on the real backend path; the `allow`
/// keeps the mock build (which stays `Loaded`) warning-free.
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LoadingStatus {
    /// No id set, or the entity was removed.
    #[default]
    Unloaded,
    /// A fetch is in flight.
    Loading,
    /// Data is current.
    Loaded,
    /// The last fetch or save failed (see `error_message`).
    Error,
}
