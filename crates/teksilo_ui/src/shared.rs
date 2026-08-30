// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Code that belongs to no one feature.
//!
//! The bar for living here is **two or more independent callers**, per the house
//! rule: one call site does not qualify, however convenient. Everything below
//! arrived either as two or three byte-identical private copies in unrelated
//! files — the shape a codebase takes when there is nowhere obvious to put a
//! four-line helper, and the copies had already started to drift apart in their
//! doc comments if not yet in their behaviour — or as a view-model/support
//! module read by several features once the great `view_models/` → feature-dir
//! move reached it (`CaretHighlightSettings`, `FocusViewModel`,
//! `FullscreenViewModel`, `TypewriterSettings`, `SynopsisPlacement`,
//! `ViewState`, `ProgressRecorder`, `images`, `long_op`).
//!
//! This is deliberately *not* `widgets/`. That directory holds app-level shared
//! **widgets** (`Pill`, `DestinationPicker`, …) — named types a feature composes
//! into its own tree. Most of what was here originally is smaller than a
//! widget: formatting rules and one-expression builders that several features
//! happen to spell the same way; the view-model arrivals are bigger, but the
//! same multi-caller bar decided their home the same way.
//!
//! A feature-local `shared` module is still the right home for something two
//! submodules of *one* feature share; `tabs/shared/` is the standing example, and
//! nothing here supersedes it.

/// Shared binder plumbing for the item-editing view-models (outline, stream,
/// corkboard, overview) — crate-internal, not exposed past the extension seam.
pub(crate) mod binder_ops;

mod caret_highlight;
pub mod editor_size;
pub mod external_link;
mod focus_vm;
mod fullscreen_vm;
pub mod images;
mod item_view_states;
pub mod list_naming;
/// Shared `Origin::LongOperation` event-parsing + Work-capture helpers, used
/// by every long-operation view-model in the crate — `pub(crate)` (not just
/// `mod`) because those view-models live across many feature directories
/// rather than as descendants of this module.
pub(crate) mod long_op;
mod progress_recorder;
pub mod project_links;
pub mod slug;
pub mod stamps;
mod synopsis_placement;
pub mod text;
mod typewriter;
/// The **Undo** button a toast offers for one specific operation — shared by
/// the eight destructive-op toasts so they cannot drift apart again.
pub(crate) mod undo_toast;
mod view_state;

pub(crate) use binder_ops::{is_prose_bearing, is_synopsis_bearing};
pub use caret_highlight::{CaretBand, CaretHighlightSettings, HighlightScope};
pub use focus_vm::FocusViewModel;
pub use fullscreen_vm::FullscreenViewModel;
pub use item_view_states::ItemViewStates;
pub use progress_recorder::ProgressRecorder;
pub use synopsis_placement::SynopsisPlacement;
pub use typewriter::{TypewriterAnchor, TypewriterSettings};
pub use view_state::{ViewState, ViewStateBinding, ViewStatePorts};
