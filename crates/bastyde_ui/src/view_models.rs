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
//!   * **View** — the widgets (`app.rs`, `tabs/`, `settings.rs`, the `panels/` modals).
//!   * **ViewModel** — this module. Sits between the two; plain Rust, so it
//!     unit-tests headless with no `WidgetTree`/GPU.
//!
//! "Controller" is Qleany's (backend: UI → Controllers → Use Cases); view-models
//! sit *above* that line.
//!
//! One file per view-model — most are `<name>::<Name>ViewModel`; a couple ([`go`]'s
//! `GoAvailability`, [`quit_sequencer`]'s `QuitSequencer`) skip the suffix but are
//! view-models all the same. This list is not exhaustive — each file self-documents
//! its own ownership shape — it highlights the shapes worth knowing before adding one:
//!   * [`editors`] — `EditorsViewModel`: single-instance live state (owns the tab
//!     list + selection).
//!   * [`stream`] — `StreamViewModel`: per-container-tab live state (owns the
//!     Full Chapter/Part/Book row list and the row mutations).
//!   * [`outline`] — `OutlineViewModel`: single-instance live state (owns the
//!     `DockingModel` + tree model).
//!   * [`mod@format`] — `FormatViewModel`: single-instance live state (owns the
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
//!   * [`save_state`] — `SaveStateViewModel`: **Work**-scoped, not per-window,
//!     live state (owns `dirty_seq`/`saved_seq`/`saving` and the `SaveQueue`).
//!     Created once in `main`, shared by every window's `App`/`EditorsViewModel`
//!     — a per-window copy of any of this breaks the moment a second window
//!     exists (see its module docs).
//!   * [`fullscreen`] — `FullscreenViewModel`: per-window live state (remembers
//!     the placement to restore when this window's F11/View ▸ Fullscreen
//!     toggle leaves fullscreen) — minted fresh per window, never shared.
//!   * [`focus`] — `FocusViewModel`: per-window live state (Increment 2 of
//!     distraction-free — whether this window's chrome is collapsed, plus its
//!     own independent placement memory for the Shift+F11 toggle) — minted
//!     fresh per window, reset on Close-Work/Load-Work like `AppIds`.
//!   * [`go`] — `GoAvailability`: per-window live state (Increment 4's Go menu
//!     — the six Next/Previous × Scene/Chapter/Note rows' live "is there a
//!     target" mirrors) — minted fresh per window alongside `scene_focused`,
//!     for the same reason `fullscreen`/`focus` are: a shared instance would
//!     let a second project window's Go menu reflect the wrong window's
//!     focused item.
//!
//! Not every file here is a view-model — pure support/data, owned by no widget:
//!   * [`view_state`] — `ViewState` + the ports a mounted pane publishes: shared
//!     by the distraction-free surface's caret handoff and `workspace.toml`'s
//!     per-tab restore, neither of which owns it.
//!   * [`long_op`], [`save_queue`], [`mod@save_status`], [`open_failure`] — shared
//!     `Origin::LongOperation` payload parsing, the save-coalescing state
//!     machine, the save indicator's pure decision table, and the "couldn't
//!     open" toast text.
//!   * [`binder_ops`], [`project_switcher`] — shared plumbing for the four
//!     binder-editing view-models, and the pure functions behind the
//!     project-switcher popover.
//!   * [`caret_highlight`], [`typewriter`], [`synopsis_placement`],
//!     [`word_count_status`] — pure preference vocabulary read by both a
//!     settings pane and the editor/status-bar wiring (each says so in its own
//!     module doc).
//!   * [`progress_recorder`], [`mention_index`] — app-level services with no
//!     view of their own (word-count history, the cross-work mention index),
//!     wired once in `App::build` and registered as `app_state`.
//!   * [`timers`] — the pure autosave/backup countdown policy.
//!
//! Cross-view-model rules (keep the dependency graph a DAG):
//!   * A view-model may hold framework model handles and call *down* into them.
//!   * Peer view-models do **not** import each other; `App` mediates them (see the
//!     outline-selection → editor-open effect in `app.rs`).
//!   * Many-to-one / distant links graduate to the intent bus.

mod add_dictionary;
pub mod analysis;
mod backup_restore;
mod backup_scheduler;
mod backup_settings;
mod backups_list;
mod binder_ops;
mod caret_highlight;
mod comments;
mod corkboard;
mod dictionaries;
mod distraction_free_surface;
mod distraction_free_themes;
mod editors;
mod export;
mod export_styles;
mod find;
mod focus;
mod format;
mod fullscreen;
mod go;
mod go_to;
pub mod images;
mod import_plume;
mod long_op;
mod mention_index;
mod new_work;
mod note_templates;
mod open_failure;
mod outline;
mod overview;
mod pace;
mod paratext_presets;
mod progress_recorder;
mod project_lifecycle;
mod project_switch;
pub mod project_switcher;
mod quit_sequencer;
mod save_as;
mod save_queue;
mod save_state;
mod save_status;
mod search_replace;
mod settings;
mod stream;
mod synopsis_placement;
mod tags;
mod text_replacement_rules;
mod timers;
mod trash;
mod tree_expansion;
mod typewriter;
mod user_dictionary;
mod view_state;
mod welcome;
mod word_count_status;
mod work_settings;
mod workspace_layout;
mod writing_session;

pub use add_dictionary::AddDictionaryViewModel;
pub use analysis::{AnalysisCategory, AnalysisState, AnalysisViewModel};
pub use backup_restore::BackupRestoreViewModel;
pub use backup_scheduler::BackupSchedulerViewModel;
pub use backup_settings::BackupSettingsViewModel;
pub use backups_list::{BackupRow, BackupsListViewModel};
pub(crate) use binder_ops::{is_prose_bearing, is_synopsis_bearing};
pub use caret_highlight::{CaretBand, CaretHighlightSettings, HighlightScope};
pub use comments::{CommentFilter, CommentPalette, CommentSort, CommentsViewModel, ThreadEntry};
pub use corkboard::{CorkboardViewModel, SORT_TITLE as CORKBOARD_SORT_TITLE};
pub use dictionaries::{DictionariesViewModel, InstallDictError};
pub use distraction_free_surface::{DistractionFreeSurfaceViewModel, SurfaceDeps};
pub use distraction_free_themes::DistractionFreeThemesViewModel;
pub use editors::{EditorsViewModel, Side};
pub use export::{ExportViewModel, format_label, scope_label};
pub use export_styles::ExportStylesViewModel;
pub use find::FindViewModel;
pub use focus::FocusViewModel;
pub use format::{
    ALIGN_CENTER, ALIGN_LEFT, DIR_AUTO, DIR_LTR, DIR_RTL, EditorKind, FormatSurface,
    FormatViewModel,
};
pub use fullscreen::FullscreenViewModel;
pub use go::GoAvailability;
pub use go_to::GoToViewModel;
pub use import_plume::ImportPlumeViewModel;
pub use mention_index::{MentionIndex, MentionRow};
pub use new_work::NewWorkViewModel;
pub use note_templates::NoteTemplatesViewModel;
pub use open_failure::open_failure_toast;
pub use outline::OutlineViewModel;
pub use overview::OverviewViewModel;
pub use pace::PaceViewModel;
pub use paratext_presets::{ParatextPresetsViewModel, PresetRow};
pub use progress_recorder::ProgressRecorder;
pub use project_lifecycle::ProjectLifecycleViewModel;
pub(crate) use project_lifecycle::reload_personal_words;
pub use project_switch::{
    PendingSwitch, ProjectSwitchViewModel, UnsavedDecision, unsaved_decision,
};
pub use quit_sequencer::QuitSequencer;
pub use save_as::SaveAsViewModel;
pub(crate) use save_queue::{DeferredResume, resume_deferred};
pub use save_state::SaveStateViewModel;
pub use save_status::{SaveStatus, SpinnerGate, save_clickable, save_status};
pub use search_replace::SearchReplaceViewModel;
pub use settings::{
    CorkboardDefaults, EditorTypography, EditorTypographySet, EditorViewMemory, SettingsViewModel,
};
pub use stream::{SplitFlavour, StreamViewModel};
pub use synopsis_placement::SynopsisPlacement;
pub use tags::TagsViewModel;
pub use text_replacement_rules::TextReplacementRulesViewModel;
pub(crate) use timers::{AutosaveCountdown, IntervalCountdown, IntervalTick};
pub use trash::TrashViewModel;
pub use tree_expansion::TreeExpansionViewModel;
pub use typewriter::{TypewriterAnchor, TypewriterSettings};
pub use user_dictionary::UserDictionaryViewModel;
pub use view_state::{ViewState, ViewStateBinding, ViewStatePorts};
pub use welcome::{DISCORD_URL, GITHUB_URL, WelcomeViewModel};
pub use word_count_status::{CountDisplay, count_display};
pub use work_settings::WorkSettingsViewModel;
pub use workspace_layout::WorkspaceLayoutViewModel;
pub use writing_session::{
    WritingSessionViewModel, format_mmss, gauge_role, remaining, words_progress,
};
