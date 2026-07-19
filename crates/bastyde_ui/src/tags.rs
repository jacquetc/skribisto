// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The tags feature's UI pieces: the preset catalogue, and (as they land) the pill field,
//! the compact chip renderer and its tooltip.
//!
//! Business logic lives in [`TagsViewModel`](crate::view_models::TagsViewModel); this
//! module is presentation and the data that is only meaningful to it.

pub mod presets;

pub use presets::Preset;
