// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Layer B — UI **view-models** (the VM in MVVM).
//!
//! A *view-model* is a cloneable handle that owns one UI feature's **state**
//! (`Signal`s, `teksilo::data` models, framework model handles) and exposes its
//! **business API** as plain methods. Widgets bind to a view-model's signals and
//! forward events to its methods; no business logic lives in `build()`.
//!
//! The three MVVM layers in `teksilo_ui`:
//!   * **Model** — `models/` (reactive `teksilo::data` adapters over the Qleany
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
//!   * [`crate::stream::StreamViewModel`] — per-container-tab live state (owns the
//!     Full Chapter/Part/Book row list and the row mutations).
//!   * [`crate::binder::OutlineViewModel`] — single-instance live state (owns the
//!     `DockingModel` + tree model).
//!   * [`mod@format`] — `FormatViewModel`: single-instance live state (owns the
//!     formatting mirrors shared by the format dock, the Format menu and the
//!     editor's context-menu row).
//!   * [`settings`] — `SettingsViewModel`: store-backed facade over persisted UI
//!     settings.
//!   * [`crate::new_work::NewWorkViewModel`] — single-instance live state (owns the New
//!     Work dialog's form signals).
//!   * [`crate::import_plume::ImportPlumeViewModel`] — single-instance live state (owns
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
//!   * [`crate::shared::binder_ops`] — shared plumbing for the four
//!     binder-editing view-models.
//!   * [`project_switcher`] — the pure functions behind the project-switcher
//!     popover.
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

pub mod analysis;
mod caret_highlight;
mod distraction_free_surface;
mod distraction_free_themes;
mod editors;
mod focus;
mod format;
mod fullscreen;
mod go;
mod go_to;
pub mod images;
/// Shared `Origin::LongOperation` event-parsing + Work-capture helpers, used
/// by every long-operation view-model in the crate — `pub(crate)` (not just
/// `mod`) because several of those view-models now live in their own feature
/// directories (e.g. `crate::backup`) rather than as descendants of this
/// module.
pub(crate) mod long_op;
mod mention_index;
mod open_failure;
mod progress_recorder;
mod project_lifecycle;
mod project_switch;
pub mod project_switcher;
mod quit_sequencer;
mod save_as;
mod save_queue;
mod save_state;
mod save_status;
mod settings;
mod synopsis_placement;
mod text_replacement_rules;
mod timers;
mod tree_expansion;
mod typewriter;
mod view_state;
mod word_count_status;
mod work_settings;
mod workspace_layout;
/// Self-imposed drafting constraints — currently "Always forward".
pub mod writing_games;
mod writing_session;

pub use analysis::{AnalysisCategory, AnalysisState, AnalysisViewModel};
pub use caret_highlight::{CaretBand, CaretHighlightSettings, HighlightScope};
pub use distraction_free_surface::{DistractionFreeSurfaceViewModel, SurfaceDeps};
pub use distraction_free_themes::DistractionFreeThemesViewModel;
pub use editors::{EditorsViewModel, Side};
pub use focus::FocusViewModel;
pub use format::{
    ALIGN_CENTER, ALIGN_LEFT, DIR_AUTO, DIR_LTR, DIR_RTL, EditorKind, FormatSurface,
    FormatViewModel,
};
pub use fullscreen::FullscreenViewModel;
pub use go::GoAvailability;
pub use go_to::GoToViewModel;
pub use mention_index::{MentionIndex, MentionRow};
pub use open_failure::open_failure_toast;
pub use progress_recorder::ProgressRecorder;
pub use project_lifecycle::ProjectLifecycleViewModel;
pub(crate) use project_lifecycle::reload_personal_words;
pub use project_switch::{
    PendingSwitch, ProjectSwitchViewModel, UnsavedDecision, unsaved_decision,
};
pub use quit_sequencer::QuitSequencer;
pub use save_as::SaveAsViewModel;
pub(crate) use save_queue::{DeferredResume, resume_deferred};
pub use save_state::{SaveStateViewModel, WorkHandle};
pub use save_status::{SaveStatus, SpinnerGate, save_clickable, save_status};
pub use settings::{
    CorkboardDefaults, EditorTypography, EditorTypographySet, EditorViewMemory, SettingsViewModel,
};
pub use synopsis_placement::SynopsisPlacement;
pub use text_replacement_rules::TextReplacementRulesViewModel;
pub(crate) use timers::{AutosaveCountdown, IntervalCountdown, IntervalTick};
pub use tree_expansion::TreeExpansionViewModel;
pub use typewriter::{TypewriterAnchor, TypewriterSettings};
pub use view_state::{ViewState, ViewStateBinding, ViewStatePorts};
pub use word_count_status::{CountDisplay, GoalDisplay, count_display, goal_display};
pub use work_settings::WorkSettingsViewModel;
pub use workspace_layout::WorkspaceLayoutViewModel;
pub use writing_games::{
    FORWARD_PROSE_DEFAULT, FORWARD_SYNOPSIS_DEFAULT, WritingGameOptions, WritingGamesViewModel,
};
pub use writing_session::{
    WritingSessionViewModel, format_mmss, gauge_role, remaining, words_progress,
};
