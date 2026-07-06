//! Layer A — reactive list/tree models that adapt the Qleany backend to
//! `bastyde::data` models.
//!
//! Each model lives in its own self-contained file holding BOTH a real and a
//! mock definition, selected by the `mocks` feature — no `#[cfg]` gate ever
//! leaks into consuming code. This index (module declarations + re-exports) and
//! the per-model files are the reference shape for a future Qleany generator.
//!
//! **Convention.** The default real/mock shape is two `#[cfg]`-gated `mod imp`
//! blocks of the same-named type (see `singles/`). A model whose real/mock
//! difference is confined to a small *data seam* — like the tree model, whose
//! ~400-line `TreeDataSource` algorithm is identical for both — instead writes
//! that algorithm once and gates only the seam (`mod rows`), to avoid
//! duplicating (and drifting) the shared logic.

mod binder_binder_items_tree_model;
mod binder_list_model;
mod examples_list_model;
mod recent_work_list_model;

pub use binder_binder_items_tree_model::{
    BinderBinderItemsTreeModel, BinderTreeKey, CommitMove, TreeFilters, TreeNode,
};
pub use binder_list_model::{BinderListModel, BinderRow};
pub use examples_list_model::ExamplesListModel;
pub use recent_work_list_model::RecentWorkListModel;
