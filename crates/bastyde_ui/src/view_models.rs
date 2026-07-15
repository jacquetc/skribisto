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
mod backup_scheduler;
mod backup_settings;
mod dictionaries;
mod editors;
mod export;
mod find;
mod import_plume;
mod long_op;
mod new_work;
mod outline;
mod project_switch;
mod restore;
mod save_as;
mod save_queue;
mod search_replace;
mod save_status;
mod settings;
mod stream;
mod welcome;

pub use add_dictionary::AddDictionaryViewModel;
pub use backup_scheduler::BackupSchedulerViewModel;
pub use backup_settings::BackupSettingsViewModel;
pub use dictionaries::{DictionariesViewModel, InstallDictError, missing_from};
pub use editors::{EditorsViewModel, Side};
pub use export::{ExportViewModel, format_label, scope_label};
pub use find::FindViewModel;
pub use import_plume::ImportPlumeViewModel;
pub use new_work::NewWorkViewModel;
pub use outline::OutlineViewModel;
pub use project_switch::{
    PendingSwitch, ProjectSwitchViewModel, UnsavedDecision, unsaved_decision,
};
pub use restore::RestoreViewModel;
pub use save_as::SaveAsViewModel;
pub use save_status::{SaveStatus, SpinnerGate, save_clickable, save_status};
pub use search_replace::SearchReplaceViewModel;
pub use settings::{EditorTypography, EditorTypographySet, SettingsViewModel};
pub use stream::{SplitFlavour, StreamViewModel};
pub(crate) use stream::{is_prose_bearing, is_synopsis_bearing};
pub use welcome::{DISCORD_URL, GITHUB_URL, WelcomeViewModel};
