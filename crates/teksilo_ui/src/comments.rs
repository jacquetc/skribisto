// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Comments: anchoring, the two docks, and the in-editor highlight layer.
//!
//! The anchoring rules live in [`anchor`] as pure functions over plain data, so the
//! logic that decides whether a writer's note survives a rewrite is table-testable
//! without a document, a widget, or a store. [`preview`] is the same shape for a
//! different concern: turning a body's Djot into the one-line, markup-free text a
//! *summary* of it (never the card itself) is allowed to show.

pub mod anchor;
pub mod binding;
pub mod card;
pub mod layout;
pub mod margin;
pub mod pane;
pub mod preview;
pub mod session;
pub mod signature;
