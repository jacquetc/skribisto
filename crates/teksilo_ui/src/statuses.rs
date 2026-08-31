// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The status feature: the project's workflow ladder.
//!
//! A status is the **progress** axis of the writing model — what a row is currently *at* —
//! and it is deliberately a different shape of thing from the two axes already here:
//!
//! | axis | cardinality | vocabulary | colour |
//! |---|---|---|---|
//! | [`crate::tags`] — what a row *is* | many | project catalogue | the writer's own hex |
//! | **status** — what a row is *at* | one, **ordered** | project catalogue | an app role |
//! | `BinderItem::label` — a private note | one | ad hoc | none |
//!
//! Two levels, and the split is what makes the feature work: the writer owns a rung's
//! **name** and the ladder's **order**; the app owns its `StatusCategory`, and the
//! category — never the name — owns the glyph and the per-theme colour. That is the shape
//! Jira, Linear and Notion each arrived at independently, and here it is also forced, for
//! the reason [`glyph`] sets out in full: a status colour cannot be stored user data and
//! still clear WCAG against both of this app's themes.

mod statuses_vm;

pub mod completion;
pub mod glyph;
pub mod panel;
pub mod picker;
pub mod presets;

pub use glyph::{GLYPH_SIZE, category_role, status_glyph};
pub use picker::{SetStatus, status_picker, status_picker_dense, status_picker_marked};
pub use presets::{Preset, StatusRow};
pub use statuses_vm::{StatusRung, StatusesViewModel};
