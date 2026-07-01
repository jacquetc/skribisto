//! Layer-A model: the bundled example works listed in the Welcome dialog's
//! "Examples" pane.
//!
//! Owns a `bastyde::data::ListModel<ExampleEntry>` so the Welcome panel renders
//! examples through a `ListView` — with `Role::List` / `Role::ListItem`
//! accessibility and keyboard navigation — exactly like recents, sharing the
//! same `StandardListItem` row.
//!
//! Static — there is no backend for examples — so unlike `RecentWorkListModel`
//! this has **no real/mock split**: the data (one entry today, *Starforgers*)
//! is byte-identical in both builds, so a second `mod imp` would only duplicate
//! the embedded payload (the data-seam exception in `models.rs`, taken to its
//! limit: the *whole* model is the shared part). The `.skrib` bytes are
//! **embedded** in the binary so an example opens from any working directory
//! and in a shipped build; the Welcome view-model extracts them to a writable
//! temp copy before loading (the repo original is read-only). The public shape
//! mirrors `RecentWorkListModel` so the Welcome panel treats recents and
//! examples uniformly.

use std::rc::Rc;

use bastyde::data::ListModel;
use bastyde::prelude::*;

use frontend::AppContext;

/// One bundled example work.
#[derive(Clone)]
pub struct ExampleEntry {
    /// Display title.
    pub title: &'static str,
    /// One-line description shown under the title.
    pub blurb: &'static str,
    /// File name for the extracted temp copy (e.g. `"Starforgers.skrib"`).
    pub file_name: &'static str,
    /// Embedded `.skrib` bytes (legacy SQLite; opened via the load-work upgrader).
    pub bytes: &'static [u8],
}

/// The bundled examples. One today — add further entries here.
const EXAMPLES: &[ExampleEntry] = &[ExampleEntry {
    title: "Starforgers",
    blurb: "A bundled example novel — a tour of the writing model.",
    file_name: "Starforgers.skrib",
    bytes: include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../resources/examples/Starforgers.skrib"
    )),
}];

/// Reactive list model over the static bundled examples.
#[derive(Clone)]
pub struct ExamplesListModel {
    /// Single source of truth — the reactive list a `ListView` binds to.
    model: ListModel<ExampleEntry>,
    version: Signal<u64>,
}

#[allow(dead_code)]
impl ExamplesListModel {
    pub fn new(_ctx: Rc<AppContext>) -> Self {
        Self {
            model: ListModel::from_vec(EXAMPLES.to_vec()),
            version: Signal::new(0),
        }
    }

    /// No backend events to subscribe to — examples are static.
    pub fn wire(&self, _ctx: &mut BuildContext) {}

    /// The reactive model to bind a `ListView` to.
    pub fn list_model(&self) -> ListModel<ExampleEntry> {
        self.model.clone()
    }

    /// Constant (examples never change at runtime); present for API symmetry
    /// with `RecentWorkListModel` so the panel can bind it the same way.
    pub fn version_signal(&self) -> Signal<u64> {
        self.version.clone()
    }

    /// `Vec` snapshot of the bundled examples — for non-`ListView` consumers.
    pub fn items(&self) -> Vec<ExampleEntry> {
        EXAMPLES.to_vec()
    }
}
