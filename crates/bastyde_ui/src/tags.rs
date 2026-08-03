// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The tags feature's UI pieces: the preset catalogue, and (as they land) the pill field,
//! the compact chip renderer and its tooltip.
//!
//! Business logic lives in [`TagsViewModel`](crate::view_models::TagsViewModel); this
//! module is presentation and the data that is only meaningful to it.

pub mod alias_pill_field;
pub mod cast_add;
pub mod contrast;
pub mod mention_list;
pub mod pov;
pub mod presets;
pub mod tag_chip;
pub mod tag_pill_field;
pub mod tag_tooltip;

pub use alias_pill_field::AliasPillField;
pub use cast_add::{LiveCastOverlay, candidates_from_table, cast_add_button};
pub use mention_list::MentionList;
pub use pov::{pov_add_button, pov_chip_row, pov_chips};
pub use presets::Preset;
pub use tag_chip::TagDotsRow;
pub use tag_pill_field::TagPillField;
