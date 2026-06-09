//! Layer A — reactive list/tree models that adapt the Qleany backend to
//! `bastyde::data` models.
//!
//! Each model lives in its own self-contained file holding BOTH a real and a
//! mock definition of the same type, selected by the `mocks` feature — no
//! `#[cfg]` gate ever leaks into consuming code. This index (module declarations
//! + re-exports) and the per-model files are the reference shape for a future
//! Qleany generator.

mod binder_binder_items_tree_model;

pub use binder_binder_items_tree_model::{BinderBinderItemsTreeModel, TreeNode};
