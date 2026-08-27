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
//! **Thirteen registration slots.** Each is a namespaced registration returning a
//! **drop-handle**, refusing an id that is built-in or already held by another
//! namespace, and each hands the live application handles to the *view* it
//! registers rather than to the registration itself — through the context type
//! named beside it.
//!
//! The table is the count: `ext/tests.rs` fails the build if a slot is missing a
//! row, which is how the prose here came to say "nine" over a table of twelve.
//!
//! | Slot | Read at | Its view is handed |
//! |---|---|---|
//! | [`register_dock`] | a window's shell is built | [`DockContext`] |
//! | [`register_inspector_section`] | the Inspector renders | [`InspectorContext`] |
//! | [`register_container_segment`] | a container tab is built | [`ContentTab`] |
//! | [`register_note_details_section`] | an entry's Details page is built | [`NoteSectionContext`] |
//! | [`register_category`] | the Analysis pane is built | [`AnalysisViewModel`] |
//! | [`register_topics`] | the Help window is built | nothing; a topic is content |
//! | [`register_lane_provider`] | a text surface's margin lane is built | [`LaneContext`] |
//! | [`register_command`] | `App::build`, and each window's menu build | [`SeamContext`] |
//! | [`register_settings`] | every `spec`/`dump`/`load_pins` lookup | nothing; a key is data |
//! | [`register_page`] | the Settings window is built | [`SeamContext`] |
//! | [`register_wiring`] | every `App::build`, beside the extension commands | `&mut BuildContext` |
//! | [`register_locales`] | **once**, in `run()` | nothing; a bundle is data |
//! | [`register`] (identity) | **earliest of all**, before `AppContext` exists | nothing; it *is* the answer |
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
pub use crate::docks::{
    DockContext, DockHandle, ExtensionDock, LiveProse, LiveProseFn, register_dock,
};
// The placement an `ExtensionDock` carries, and the band its id has to sit in.
//
// `ExtensionDock::placement` is an `AppDock`, so a registration cannot be written without
// naming this type: leaving it out of the façade means every extension reaches past `ext`
// for the one field it cannot avoid. The two bounds go with it because an id outside them
// is refused at registration, at runtime, in whichever window opened first, and a caller
// that can name them can check at compile time instead.
pub use crate::docks::{APP_DOCK_ID_CEILING, AppDock, EXTENSION_DOCK_ID_FLOOR};

// ── Marking a project dirty ──────────────────────────────────────────────────
//
// Published because both context types above hand one out — `DockContext::work`
// and `ContentTab::work()` — and an extension that receives a value it cannot
// name has to reach past this façade to spell its type, which is the one thing
// the façade exists to prevent. It went unnoticed until the downstream edition
// was compiled against a release that had moved it; the drift test walks
// registration functions, and a *type* handed out by one is invisible to it.
pub use crate::save::WorkHandle;

// ── The mention index ────────────────────────────────────────────────────────
//
// Who is named where, across the whole Work. Kept current by
// `WorkManagementEvent::LoadWork`/`NewWork`, a binder tag or item being created,
// updated or removed, and a throttled rescan on save (`app/wiring/long_ops.rs`).
//
// **Take it from the context the slot hands you, never from `app_state`.**
// `DockContext::mention_index`, `ContentTab::mention_index()` and
// `NoteSectionContext::mention_index` are all *this Work's* index, the same one
// every roster and backlink list in the app resolves through. There is also a
// `MentionIndex` published as `app_state` in `App::build`, and it is the one slot
// that cannot answer this correctly: it is fixed at process start and holds
// whatever the *bootstrap* session built. Launch with a project on the command
// line, which is how a developer runs the app, and the bootstrap session is that
// project, so the slot happens to hold the right index and everything works.
// Open the app first and pick a project from Recents, which is what most writers
// do, and it holds the throwaway session's index, bound to a `work_id` of `None`
// and empty for the life of the process. A reading built on it then states an
// absence rather than an error: a character named in twenty-two scenes reads as
// "not named in any scene yet". A downstream edition shipped exactly that.
//
// A window an extension opens for itself is neither a dock nor a tab, so no
// context reaches inside it. Hand the index down from whichever slot opened the
// window, the way `view_model_setup` threads it into every tab, rather than
// reaching for `ctx.app_state::<MentionIndex>()` from the window's widget tree.
//
// Published for the same reason `WorkHandle` is above: an extension that receives
// a value it cannot name has to reach past this façade to spell its type, which is
// the one thing the façade exists to prevent. `MentionRow`'s fields are already
// `pub` and the type is already `Clone`, so this is a pure visibility fix, nothing
// more.
//
// `skribisto_model::mentions::DiscoverableEntity`, what
// `MentionIndex::discoverable_table` hands back, is deliberately *not*
// re-exported here, unlike the margin lane's own types below. A lane provider
// has no other route to `LaneColumn`/`LaneShape`/`LaneMark`/`LaneSpan`; a
// downstream edition already depends on `skribisto_model` directly for its own
// counting and drift math, so it can already name `DiscoverableEntity` on its own
// crate's dependency, and re-exporting it here would only be a second name for
// the same type.
pub use crate::mentions::{MentionIndex, MentionRow};

// ── Inspector sections ───────────────────────────────────────────────────────
pub use crate::docks::inspector_sections::{
    InspectorContext, InspectorSectionHandle, InspectorSectionSpec, register_inspector_section,
};

// ── Container segments ───────────────────────────────────────────────────────
pub use crate::tabs::ContentTab;
pub use crate::tabs::shared::segments::{
    ContainerSegmentHandle, ContainerSegmentSpec, register_container_segment, segment_id,
};

// ── Chart sizing ─────────────────────────────────────────────────────────────
//
// Not a registration slot: there is nothing per-extension to key, which is the
// same shape as `app_paths` and `active_query` below.
//
// Pace and Analysis already share this exact sizing so that one manuscript reads
// as one shape on both of their charts (see the module doc on
// `tabs::shared::charts` for why a chart is sized from its data rather than its
// viewport). An out-of-tree edition adding a third chart to the same tab bar has
// no other route to that shape, and re-deriving the formula rather than calling
// it drifts silently the first time `BAR_PITCH` or either height moves.
//
// ⚠ These five are frozen the way a registration id is frozen: their values are
// a visual contract between charts that sit beside each other, not an
// implementation detail.
pub use crate::tabs::shared::charts::{
    BAR_PITCH, CHART_HEIGHT, STRIP_HEIGHT, content_width, wide_chart,
};

// ── One entry's Details page ─────────────────────────────────────────────────
//
// A *per-entry* door, deliberately not a segment: a reading about one story-bible
// entry belongs on that entry's own page, where the app already knows which entry
// it is about. See `tabs::shared::note_sections` for what an extension had to do
// without one.
pub use crate::tabs::shared::note_sections::{
    NoteSectionContext, NoteSectionHandle, NoteSectionSpec, register_note_details_section,
    registered_note_sections,
};

// ── The margin lane ──────────────────────────────────────────────────────────
/// The label closure every registration's `label`/`hint` fields are typed as.
///
/// Declared next to the dock seam because that is where it first appeared, and
/// shared verbatim by the inspector, segment, category and lane specs.
pub use crate::docks::LabelFn;
/// Which field of an item a lane is mapping, carried on [`LaneContext::kind`].
///
/// A stream is one [`LaneSurface`] with two flavours, a Book's prose and the
/// same Book's synopsis cards, and this is what tells them apart.
pub use crate::format::EditorKind;
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
