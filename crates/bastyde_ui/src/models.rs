// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

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

mod backup_settings_file;
mod binder_binder_items_tree_model;
mod binder_stream;
mod binder_list_model;
mod dictionary_settings_file;
mod examples_list_model;
mod export_styles_file;
mod installed_dictionaries_model;
mod open_docs;
mod pace_model;
mod recent_work_list_model;
mod search_results_model;
mod search_settings_file;
mod stats_model;
mod stream_rows_model;
mod workspace_layout_file;

pub use backup_settings_file::{BackupPolicy, BackupSettingsService, RetentionMode, uid_is_usable};
pub use dictionary_settings_file::{DictionarySettingsService, UserDictionary, license_hash};
pub use installed_dictionaries_model::{
    DictOrigin, InstalledDictionariesModel, InstalledDictionaryRow,
};
pub use binder_binder_items_tree_model::{
    BinderBinderItemsTreeModel, BinderTreeKey, CommitMove, TreeFilters, TreeNode,
};
pub use binder_list_model::{BinderListModel, BinderRow};
pub use binder_stream::ordered_binder_items;
pub use examples_list_model::ExamplesListModel;
pub use export_styles_file::ExportStylesService;
pub use open_docs::{OpenDoc, OpenDocsStore};
pub use pace_model::{DailyCount, HolidayRow, MilestoneRow, PaceModel};
pub use recent_work_list_model::RecentWorkListModel;
pub use search_results_model::SearchResultsModel;
pub use stats_model::StatsModel;
pub use search_settings_file::{SearchPrefs, SearchSettingsService};
pub use stream_rows_model::{StreamLevel, StreamRow, StreamRowsModel};
pub use workspace_layout_file::{PaneLayout, PerProjectLayout, WorkspaceLayoutService};
