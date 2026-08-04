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
mod binder_list_model;
mod binder_stream;
mod comments_list_model;
mod corkboard_cards_model;
mod dict_word_list_model;
mod dictionary_settings_file;
mod distraction_free_themes_file;
mod examples_list_model;
mod export_styles_file;
mod installed_dictionaries_model;
mod open_docs;
mod overview_rows_model;
mod pace_model;
mod paratext_presets;
mod recent_work_list_model;
mod repetition_tree_model;
mod row_search;
mod search_results_model;
mod search_settings_file;
mod stats_model;
mod stream_rows_model;
mod text_replacement_rule_list_model;
mod trash_tree_model;
mod tree_expansion_file;
mod work_note_templates_list_model;
mod work_tags_list_model;
mod workspace_layout_file;

pub use backup_settings_file::{BackupPolicy, BackupSettingsService, RetentionMode, uid_is_usable};
pub use binder_binder_items_tree_model::{
    BinderBinderItemsTreeModel, BinderTreeKey, CommitMove, TreeFilters, TreeNode,
};
pub use binder_list_model::{BinderListModel, BinderRow};
pub use binder_stream::{BinderItemRef, ordered_binder_items};
#[allow(unused_imports)]
pub use comments_list_model::{CommentRow, CommentsListModel, ReplyRow};
pub use corkboard_cards_model::{CorkboardCard, CorkboardCardsModel};
pub use dict_word_list_model::{DictWordListModel, DictWordRow};
pub use dictionary_settings_file::{DictionarySettingsService, UserDictionary, license_hash};
pub use distraction_free_themes_file::DistractionFreeThemesService;
pub use examples_list_model::ExamplesListModel;
pub use export_styles_file::ExportStylesService;
pub use installed_dictionaries_model::{
    DictOrigin, InstalledDictionariesModel, InstalledDictionaryRow,
};
pub use open_docs::{OpenDoc, OpenDocsStore, SynopsisViewerGuard};
pub use overview_rows_model::{
    COL_LABEL, COL_OPEN_COMMENTS, COL_OWN_WORDS, COL_TAGS, COL_TITLE, COL_TOTAL_COMMENTS,
    COL_TOTAL_WORDS, COL_TYPE, OverviewFilters, OverviewRow, OverviewRowsModel,
};
pub use pace_model::{DailyCount, HolidayRow, MilestoneRow, PaceModel};
pub use paratext_presets::{NEW_PRESET_TEMPLATE, ParatextPreset, ParatextPresetsService};
pub use recent_work_list_model::RecentWorkListModel;
pub use repetition_tree_model::{RepetitionNode, RepetitionTreeKey, RepetitionTreeModel};
pub use search_results_model::SearchResultsModel;
pub use search_settings_file::{SearchPrefs, SearchSettingsService};
pub use stats_model::StatsModel;
pub use stream_rows_model::{StreamLevel, StreamRow, StreamRowsModel};
pub use text_replacement_rule_list_model::{
    TextReplacementRuleListModel, TextReplacementRuleRow, trigger_key,
};
pub use trash_tree_model::{TrashNode, TrashRootKind, TrashTreeKey, TrashTreeModel};
pub use tree_expansion_file::TreeExpansionService;
pub use work_note_templates_list_model::{
    TemplateRow, WorkNoteTemplatesListModel, moved_index, starred_first,
};
pub use work_tags_list_model::{TagRow, WorkTagsListModel, name_key, sort_rows};
pub use workspace_layout_file::{
    CorkboardTabState, PaneLayout, PerProjectLayout, TabViewState, WorkspaceLayoutService,
};
