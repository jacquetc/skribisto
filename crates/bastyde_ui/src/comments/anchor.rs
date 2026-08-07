// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where a comment points, and how it keeps pointing there.
//!
//! The rules themselves now live in [`skribisto_model::comment_anchor`], one layer
//! down, because the document importer has to capture an anchor for an editor's
//! comment arriving in a `.docx` or `.odt` — and a second copy of the capture rules
//! beside the matcher that resolves them is how the two drift apart without anyone
//! noticing. They qualified for the move on exactly the grounds `scene_break` and
//! `mentions` did: pure functions over plain data, no widgets, no document, no
//! store, table-testable in isolation.
//!
//! This module stays as the name every comment surface in the UI already imports.
//! `capture` / `resolve` / `shift_range` / `block_extent` / `block_of` /
//! `for_display`, [`Anchor`] and [`Resolution`] read exactly as they did before.
//!
//! `CONTEXT_CHARS` and `MAX_EXACT_CHARS` are deliberately not re-exported: nothing
//! in the UI reads them — they are the capture rules, and capture is what this
//! module delegates. A name re-exported for nobody is a name that drifts.

pub use skribisto_model::comment_anchor::{
    Anchor, Resolution, block_extent, block_of, capture, for_display, resolve, shift_range,
};
