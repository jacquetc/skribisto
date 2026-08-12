// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The tags feature: business logic and UI pieces.
//!
//! [`TagsViewModel`] is the tag feature's business logic, shared by the Settings ▸ Tags
//! pane, the Inspector's tag section, and the chip popover. Everything else here —
//! the preset catalogue, the pill field, the compact chip renderer and its tooltip — is
//! presentation and the data that is only meaningful to it.

mod tags_vm;

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
pub use tags_vm::TagsViewModel;
