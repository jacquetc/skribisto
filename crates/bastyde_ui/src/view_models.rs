// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Layer B — UI **view-models** (the VM in MVVM).
//!
//! A *view-model* is a cloneable handle that owns one UI feature's **state**
//! (`Signal`s, `bastyde::data` models, framework model handles) and exposes its
//! **business API** as plain methods. Widgets bind to a view-model's signals and
//! forward events to its methods; no business logic lives in `build()`.
//!
//! The three MVVM layers in `bastyde_ui`:
//!   * **Model** — `models/` (reactive `bastyde::data` adapters over the Qleany
//!     backend) + the Qleany controllers/use-cases below them.
//!   * **View** — the widgets (`app.rs`, `editor_tab.rs`, `settings_panel.rs`).
//!   * **ViewModel** — this module. Sits between the two; plain Rust, so it
//!     unit-tests headless with no `WidgetTree`/GPU.
//!
//! "Controller" is Qleany's (backend: UI → Controllers → Use Cases); view-models
//! sit *above* that line.
//!
//! One file per view-model (each self-documents its ownership shape):
//!   * [`editors`] — `EditorsViewModel`: single-instance live state (owns the tab
//!     list + selection).
//!   * [`stream`] — `StreamViewModel`: per-container-tab live state (owns the
//!     Full Chapter/Part/Book row list and the row mutations).
//!   * [`outline`] — `OutlineViewModel`: single-instance live state (owns the
//!     `DockingModel` + tree model).
//!   * [`format`] — `FormatViewModel`: single-instance live state (owns the
//!     formatting mirrors shared by the format dock, the Format menu and the
//!     editor's context-menu row).
//!   * [`settings`] — `SettingsViewModel`: store-backed facade over persisted UI
//!     settings.
//!   * [`welcome`] — `WelcomeViewModel`: store-backed facade for the start screen.
//!   * [`new_work`] — `NewWorkViewModel`: single-instance live state (owns the New
//!     Work dialog's form signals).
//!   * [`import_plume`] — `ImportPlumeViewModel`: single-instance live state (owns
//!     the Import Plume Creator dialog's form signals).
//!   * [`project_switch`] — `ProjectSwitchViewModel`: single-instance live state
//!     (owns the unsaved-changes guard every in-place project switch — New Work,
//!     Open Work, "Open here", the import toast — must pass, and the switch parked
//!     behind an in-flight save).
//!
//! Cross-view-model rules (keep the dependency graph a DAG):
//!   * A view-model may hold framework model handles and call *down* into them.
//!   * Peer view-models do **not** import each other; `App` mediates them (see the
//!     outline-selection → editor-open effect in `app.rs`).
//!   * Many-to-one / distant links graduate to the intent bus.

mod add_dictionary;
mod backup_restore;
mod backup_scheduler;
mod backup_settings;
mod backups_list;
mod binder_ops;
mod corkboard;
mod dictionaries;
mod editors;
mod export;
mod export_styles;
mod find;
mod format;
mod import_plume;
mod long_op;
mod mention_index;
mod new_work;
mod outline;
mod overview;
mod pace;
mod progress_recorder;
mod project_lifecycle;
mod project_switch;
pub mod project_switcher;
mod save_as;
mod save_queue;
mod save_status;
mod search_replace;
mod settings;
mod stream;
mod tags;
mod text_replacement_rules;
mod timers;
mod trash;
mod tree_expansion;
mod user_dictionary;
mod welcome;
mod word_count_status;
mod work_settings;
mod workspace_layout;
mod writing_session;

pub use add_dictionary::AddDictionaryViewModel;
pub use backup_restore::BackupRestoreViewModel;
pub use backup_scheduler::BackupSchedulerViewModel;
pub use backup_settings::BackupSettingsViewModel;
pub use backups_list::{BackupRow, BackupsListViewModel};
pub(crate) use binder_ops::{is_prose_bearing, is_synopsis_bearing};
pub use corkboard::CorkboardViewModel;
pub use dictionaries::{DictionariesViewModel, InstallDictError, missing_from};
pub use editors::{EditorsViewModel, Side};
pub use export::{ExportViewModel, format_label, scope_label};
pub use export_styles::ExportStylesViewModel;
pub use find::FindViewModel;
pub use format::{
    ALIGN_CENTER, ALIGN_LEFT, ALIGN_OTHER, DIR_AUTO, DIR_LTR, DIR_RTL, EditorKind, FormatSurface,
    FormatViewModel,
};
pub use import_plume::ImportPlumeViewModel;
pub use mention_index::{MentionIndex, MentionRow};
pub use new_work::NewWorkViewModel;
pub use outline::OutlineViewModel;
pub use overview::OverviewViewModel;
pub use pace::PaceViewModel;
pub use progress_recorder::ProgressRecorder;
pub use project_lifecycle::ProjectLifecycleViewModel;
pub(crate) use project_lifecycle::reload_personal_words;
pub use project_switch::{
    PendingSwitch, ProjectSwitchViewModel, UnsavedDecision, unsaved_decision,
};
pub use save_as::SaveAsViewModel;
pub(crate) use save_queue::{DeferredResume, resume_deferred};
pub use save_status::{SaveStatus, SpinnerGate, save_clickable, save_status};
pub use search_replace::SearchReplaceViewModel;
pub use settings::{
    CorkboardDefaults, EditorTypography, EditorTypographySet, EditorViewMemory, SettingsViewModel,
};
pub use stream::{SplitFlavour, StreamViewModel};
pub use tags::{TagImportSummary, TagsViewModel};
pub use text_replacement_rules::{TextReplacementImportSummary, TextReplacementRulesViewModel};
pub(crate) use timers::{AutosaveCountdown, IntervalCountdown, IntervalTick};
pub use trash::TrashViewModel;
pub use tree_expansion::TreeExpansionViewModel;
pub use user_dictionary::{ImportSummary, UserDictionaryViewModel};
pub use welcome::{DISCORD_URL, GITHUB_URL, WelcomeViewModel};
pub use word_count_status::{CountDisplay, count_display};
pub use work_settings::WorkSettingsViewModel;
pub use workspace_layout::WorkspaceLayoutViewModel;
pub use writing_session::{
    WritingSessionViewModel, format_mmss, gauge_role, remaining, words_progress,
};
