// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! App-level widgets shared by more than one feature.
//!
//! The bar for living here is genuine reuse across features, not "it is a widget" — a
//! control used by exactly one feature belongs with that feature. [`pill::Pill`] qualifies:
//! spellcheck languages, item tags and item aliases all render one.

pub mod pill;

pub use pill::{Pill, PillTooltip, attach_labelled_composite_tooltip};
