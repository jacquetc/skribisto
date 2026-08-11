// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! App-level widgets shared by more than one feature.
//!
//! The bar for living here is genuine reuse across features, not "it is a widget" — a
//! control used by exactly one feature belongs with that feature. [`pill::Pill`] qualifies:
//! spellcheck languages, item tags and item aliases all render one. So does
//! [`structure_number::StructureNumber`]: the outline tree, the Overview table, the stream
//! headings, the corkboard, the Inspector and the export scope tree all show a chapter's
//! ordinal beside its title. And so does [`destination_picker::DestinationPicker`]: both
//! restoring from the trash and importing a document have to ask where the result should
//! land, and it is the same question with the same answer shape.

pub mod destination_picker;
pub mod diff_pane;
/// The word/character target readout. Four surfaces draw it and must not draw it
/// differently: the Inspector, a container page's header, the status bar, and the
/// Distribute preview.
pub mod goal_progress;
/// Words or characters: the project's counting unit, in New Work and in Settings.
pub mod goal_unit_picker;
pub mod pill;
pub mod structure_number;
/// A registry rich tooltip on content that has no builder of its own.
pub mod tip;

pub use destination_picker::DestinationPicker;
pub(crate) use diff_pane::DiffPane;
pub use pill::{Pill, PillTooltip, attach_labelled_composite_tooltip};
pub use structure_number::StructureNumber;
