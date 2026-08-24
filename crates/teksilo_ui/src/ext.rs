// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The extension seam, in one place.
//!
//! Every module in this crate is `pub` (see the note in `lib.rs`) because the
//! crate *is* the application and an extension has to reach into it. That stays
//! true and nothing here takes it away. What this module adds is a **stable
//! address**: an out-of-tree edition should `use teksilo_ui::ext::*` and name
//! nothing else, so that moving a view from `docks/comments.rs` into
//! `comments/dock.rs` is an internal change rather than a downstream break.
//!
//! There are nine registration slots and five context types. Each slot is a
//! namespaced registration returning a **drop-handle**, refusing an id that is
//! built-in or already held by another namespace.
//!
//! | Slot | Read at |
//! |---|---|
//! | [`register_dock`] | a window's shell is built |
//! | [`register_inspector_section`] | the Inspector renders |
//! | [`register_container_segment`] | a container tab is built |
//! | [`register_category`] | the Analysis pane is built |
//! | [`register_topics`] | the Help window is built |
//! | [`register_lane_provider`] | a text surface's margin lane is built |
//! | [`register_command`] | `App::build`, and each window's menu build |
//! | [`register_settings`] | every `spec`/`dump`/`load_pins` lookup |
//! | [`register_page`] | the Settings window is built |
//! | [`register_locales`] | **once**, in `run()` |
//! | [`register`] (identity) | **earliest of all**, before `AppContext` exists |
//!
//! Two rules that the table cannot show, and that have both gone wrong before:
//!
//! * **Every registry is a snapshot, not a subscription.** Register on the main
//!   thread *before* `run()`. Identity is the strictest — it is read before
//!   `AppContext::new()`, so a late registration is worse than a no-op: the
//!   process has already elected, bound a socket and resolved its settings under
//!   the old identity.
//! * **Never capture an `AppContext` at registration.** The app builds its own
//!   inside `run`, so a captured one is a second, permanently empty store and the
//!   panel renders a convincing "nothing here" forever. Every slot hands the live
//!   handles to the *view* instead, through the context types below.
//!
//! The backend-side slots are not re-exported here because they live in another
//! crate entirely: `work_management::{bundle_contributors, project_store,
//! lifecycle}`.

// ── Docks ────────────────────────────────────────────────────────────────────
pub use crate::docks::{DockContext, DockHandle, ExtensionDock, register_dock};

// ── Marking a project dirty ──────────────────────────────────────────────────
//
// Published because both context types above hand one out — `DockContext::work`
// and `ContentTab::work()` — and an extension that receives a value it cannot
// name has to reach past this façade to spell its type, which is the one thing
// the façade exists to prevent. It went unnoticed until the downstream edition
// was compiled against a release that had moved it; the drift test walks
// registration functions, and a *type* handed out by one is invisible to it.
pub use crate::save::WorkHandle;

// ── Inspector sections ───────────────────────────────────────────────────────
pub use crate::docks::inspector_sections::{
    InspectorContext, InspectorSectionHandle, InspectorSectionSpec, register_inspector_section,
};

// ── Container segments ───────────────────────────────────────────────────────
pub use crate::tabs::ContentTab;
pub use crate::tabs::shared::segments::{
    ContainerSegmentHandle, ContainerSegmentSpec, register_container_segment, segment_id,
};

// ── The margin lane ──────────────────────────────────────────────────────────
/// The label closure every registration's `label`/`hint` fields are typed as.
///
/// Declared next to the dock seam because that is where it first appeared, and
/// shared verbatim by the inspector, segment, category and lane specs.
pub use crate::docks::LabelFn;
/// The margin lane's provider seam — a source of positional marks on the strip
/// beside a text surface's scroll area.
///
/// An extension registers what it knows how to find and where in the document it
/// is; the lane decides how it looks, so a provider cannot paint a red stripe
/// down the side of a manuscript or ship a mark that fails contrast on a theme
/// it never saw.
///
/// [`active_query`] is here for the same reason [`CommentAnchor`] is on the
/// context: a provider that wanted to mark what the writer is *looking for* has no
/// other route to it, and re-deriving one would mean guessing which of the two
/// searches was used last.
pub use crate::margin_lane::{
    CommentAnchor, LaneContext, LaneMarksFn, LaneProviderHandle, LaneProviderSpec, LaneQuery,
    LaneRefresh, LaneSurface, active_query, register_lane_provider,
};
/// What a provider actually returns, and the two enums its spec is declared with.
///
/// These live in `widgets` because the lane draws them, and every one of them is
/// in [`register_lane_provider`]'s reachable signature: a spec names a
/// [`LaneColumn`] and a [`LaneShape`], and its closure returns [`LaneMark`]s
/// built on [`LaneSpan`]s. Without them here, an edition that took this module's
/// own advice — *name nothing else* — could register a provider and not write
/// one.
///
/// ⚠ The gap was invisible to `ext`'s drift test, which walks `pub fn register…`
/// declarations: a **type** carried through a slot's signature is not a
/// registrar, and the same blind spot is what took `WorkHandle` a release to
/// find. `library_surface::an_extension_can_build_a_lane_provider_naming_only_ext`
/// is the check that does see it, because it is compiled from outside the crate
/// against nothing but this module.
pub use crate::widgets::{LaneColumn, LaneMark, LaneShape, LaneSpan};

// ── Analysis categories ──────────────────────────────────────────────────────
pub use crate::analysis::AnalysisViewModel;
/// Contribute a page to the Help window.
///
/// Registered topics land in the fixed **Extensions** section of the table of contents,
/// for the same reason a registered settings page does: letting a registration address
/// the app's own sections would make that tree's shape a compatibility promise.
///
/// ⚠ Read when the Help window is built, so register before `run()` like every other
/// slot. A topic whose handle drops while the window is open leaves the reader on a
/// "no longer available" page rather than a stale one.
pub use crate::help::{
    HelpBody, HelpSection, HelpTopicSpec, LocalizedSource, ResolvedBody, TopicsHandle,
    register_topics,
};
pub use crate::tabs::analysis::{AnalysisCategorySpec, CategoryHandle, register_category};

// ── Commands, shortcuts, the Tools row ───────────────────────────────────────
pub use crate::commands_ext::{
    CommandHandle, ExtensionCommand, MenuRow, SeamContext, ShortcutSpec, register_command,
};

// ── Settings keys and pages ──────────────────────────────────────────────────
pub use crate::settings_ext::{
    PageHandle, SettingsHandle, SettingsPage, register_page, register_settings,
};

// ── `App`'s own `BuildContext`, for state rather than a verb ─────────────────
// The other half of what `commands_ext` solves. Registering a settings key an
// extension's own save hook cannot read was possible before this; it is the
// bridge from a `Rc`-backed `SettingsStore` on the UI thread to the `Send + Sync`
// hooks that actually obey it.
pub use crate::app_wiring::{Wiring, WiringHandle, register_wiring};

// ── Locales ──────────────────────────────────────────────────────────────────
pub use crate::locales::{LocaleBundle, LocaleHandle, register_locales};

// ── Application identity ─────────────────────────────────────────────────────
pub use crate::identity::{AppIdentity, IdentityHandle, app_paths, register};

// ── What the writer is looking at ────────────────────────────────────────────
pub use crate::active_context::{ActiveContext, ActiveItem, ActivePane};
pub use crate::read_signal::ReadSignal;

#[cfg(test)]
mod tests;
