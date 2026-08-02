// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Comments: anchoring, the two docks, and the in-editor highlight layer.
//!
//! The anchoring rules live in [`anchor`] as pure functions over plain data, so the
//! logic that decides whether a writer's note survives a rewrite is table-testable
//! without a document, a widget, or a store.

pub mod anchor;
pub mod binding;
pub mod card;
pub mod layout;
pub mod margin;
pub mod pane;
pub mod session;
