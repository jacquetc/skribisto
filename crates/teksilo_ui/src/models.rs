// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Layer A — reactive list/tree models that adapt the Qleany backend to
//! `teksilo::data` models.
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
/// `pub`: `ordered_binder_items` + `BinderItemRef` are how anything outside this
/// crate reads the work's flat item stream — and the `(id, uid, title)` triple is
/// the only correct way to key extension data to a row, since `EntityId` is
/// re-minted on every load.
pub mod binder_stream;
mod coalesced_reload;
mod comments_list_model;
mod corkboard_cards_model;
mod dict_word_list_model;
mod dictionary_settings_file;
mod distraction_free_themes_file;
mod examples_list_model;
mod export_styles_file;
mod folder_memory_file;
mod footnote_numbering;
mod footnotes_list_model;
pub(crate) mod import_merge_source;
pub(crate) mod import_plan_source;
mod import_prefs_file;
mod installed_dictionaries_model;
mod manuscript_digest;
mod numbering;
mod open_docs;
mod overview_rows_model;
mod pace_model;
mod paratext_presets;
mod recent_work_list_model;
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
// `UNNUMBERED_MARKER` is used by the tests and by the dock's own reasoning about
// what an unnumbered reference draws; re-exported beside `marker_for` so the two
// are found together.
#[allow(unused_imports)]
pub use footnote_numbering::{NotePlacement, UNNUMBERED_MARKER, marker_for};
pub use footnotes_list_model::{FootnoteRow, FootnotesListModel};
pub use installed_dictionaries_model::{
    DictOrigin, InstalledDictionariesModel, InstalledDictionaryRow,
};
// The `--features mocks` build fabricates its rows rather than numbering a real
// manuscript, so several of these have no consumer there.
pub use folder_memory_file::{
    FolderMemoryService, FolderPurpose, dialog_start_in, picker_starts_in, remember_dialog_dir,
    remember_dialog_file, remember_pick,
};
pub use import_prefs_file::ImportPrefsService;
pub use manuscript_digest::{LiveRow, digest_of, live_manuscript};
#[allow(unused_imports)]
pub use numbering::{
    NameContext, fallback_label_for, item_meta_of, label_and_badge, numbers_for_items,
    numbers_for_work, ordered_item_dtos, work_language_tags,
};
pub use open_docs::{OpenDoc, OpenDocsStore, SynopsisViewerGuard};
pub use overview_rows_model::{
    COL_LABEL, COL_OPEN_COMMENTS, COL_OWN_WORDS, COL_TAGS, COL_TITLE, COL_TOTAL_COMMENTS,
    COL_TOTAL_WORDS, COL_TYPE, OverviewFilters, OverviewRow, OverviewRowsModel,
};
pub use pace_model::{DailyCount, HolidayRow, MilestoneRow, PaceModel};
pub use paratext_presets::{NEW_PRESET_TEMPLATE, ParatextPreset, ParatextPresetsService};
pub use recent_work_list_model::RecentWorkListModel;
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
