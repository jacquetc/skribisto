// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Layer — the **per-open-Work session** seam (Option B migration, Phase 1).
//!
//! Every other shape-directory in this crate (`models/`, `singles/`,
//! `view_models/`, `docks/`, `tabs/`, `statusbar/`, `shell/`, `settings/`,
//! `panels/`) owns exactly one *kind* of thing across every feature. This
//! directory is a deliberate house-rule exception, sanctioned by the
//! multi-Work migration design doc (§9 q6): [`WorkSession`] is a genuinely new
//! *aggregate* shape — it bundles instances that already live in `view_models/`,
//! `singles/` and `models/` under one per-open-Work handle — and none of the
//! existing directories actually own "a bundle of other layers' instances,
//! keyed by which Work they belong to". Putting it at the crate root (as
//! `app_ids.rs` is) would bury a genuinely new concept inside the "small
//! standalone file" convention that root-level files otherwise follow; a named
//! directory says plainly that this is its own layer.
//!
//! [`WorkSession`] holds the state; [`WorkRegistry`] is the app-global lookup
//! table it lives in. See each type's own module doc for what moved here and
//! why, and the crate's `app_ids.rs`/`main.rs` for how Phase 1 wires them in
//! without changing any visible behaviour.

mod work_registry;
mod work_session;

pub use work_registry::{StackTeardown, WindowTeardown, WorkRegistry};
pub use work_session::WorkSession;
