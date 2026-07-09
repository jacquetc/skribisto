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
//!   * [`outline`] — `OutlineViewModel`: single-instance live state (owns the
//!     `DockingModel` + tree model).
//!   * [`settings`] — `SettingsViewModel`: store-backed facade over persisted UI
//!     settings.
//!   * [`welcome`] — `WelcomeViewModel`: store-backed facade for the start screen.
//!   * [`new_work`] — `NewWorkViewModel`: single-instance live state (owns the New
//!     Work dialog's form signals).
//!   * [`import_plume`] — `ImportPlumeViewModel`: single-instance live state (owns
//!     the Import Plume Creator dialog's form signals).
//!
//! Cross-view-model rules (keep the dependency graph a DAG):
//!   * A view-model may hold framework model handles and call *down* into them.
//!   * Peer view-models do **not** import each other; `App` mediates them (see the
//!     outline-selection → editor-open effect in `app.rs`).
//!   * Many-to-one / distant links graduate to the intent bus.

mod chapter;
mod editors;
mod import_plume;
mod new_work;
mod outline;
mod settings;
mod welcome;

pub use chapter::ChapterViewModel;
pub use editors::EditorsViewModel;
pub use import_plume::ImportPlumeViewModel;
pub use new_work::NewWorkViewModel;
pub use outline::OutlineViewModel;
pub use settings::SettingsViewModel;
pub use welcome::WelcomeViewModel;
