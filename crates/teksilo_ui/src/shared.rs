// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Code that belongs to no one feature.
//!
//! The bar for living here is **two or more independent callers**, per the house
//! rule: one call site does not qualify, however convenient. Everything below
//! arrived as two or three byte-identical private copies in unrelated files —
//! the shape a codebase takes when there is nowhere obvious to put a four-line
//! helper — and the copies had already started to drift apart in their doc
//! comments if not yet in their behaviour.
//!
//! This is deliberately *not* `widgets/`. That directory holds app-level shared
//! **widgets** (`Pill`, `DestinationPicker`, …) — named types a feature composes
//! into its own tree. What is here is smaller than a widget: formatting rules and
//! one-expression builders that several features happen to spell the same way.
//!
//! A feature-local `shared` module is still the right home for something two
//! submodules of *one* feature share; `tabs/shared/` is the standing example, and
//! nothing here supersedes it.

/// Shared binder plumbing for the item-editing view-models (outline, stream,
/// corkboard, overview) — crate-internal, not exposed past the extension seam.
pub(crate) mod binder_ops;
pub mod list_naming;
pub mod slug;
pub mod stamps;
pub mod text;

pub(crate) use binder_ops::{is_prose_bearing, is_synopsis_bearing};
