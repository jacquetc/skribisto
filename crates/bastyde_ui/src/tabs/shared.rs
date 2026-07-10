//! Shared building blocks for the per-combination editor tabs.
//!
//! [`editor`] holds the low-level primitives (writing / synopsis columns, the
//! title field, the live typography plumbing, the editor style); [`panes`] holds
//! the composite pane renders (heading form, dual-pane prose, no-content
//! placeholder, folder synopsis) that several `(role, sub_role)` combinations
//! share. Both are re-exported here, so every tab module calls `shared::foo`
//! without caring which file it lives in.

mod editor;
mod panes;

pub use editor::*;
pub use panes::*;
