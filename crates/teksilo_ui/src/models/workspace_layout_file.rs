// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Per-work **workspace layout**: the open editor tabs (both panes + selection +
//! split) and the dock layout, so re-opening a project restores the desk exactly
//! as it was left.
//!
//! A `SettingsFile<WorkspaceLayoutFile>` at `<config_dir>/workspace.toml` holding
//! one entry per project, keyed by the project's stable **`Work.unique_id`** (not
//! its path — that survives rename/move, and is the same key `backup.toml` /
//! `search.toml` correlate on). The raw path is kept only for display/debug. This
//! is app configuration, orthogonal to the `.skrib` document, so — exactly like
//! [`SearchSettingsService`](super::SearchSettingsService) and
//! [`BackupSettingsService`](super::BackupSettingsService) — there is a **single
//! implementation (no real/mock seam)**: it opens a real `SettingsFile` in every
//! build.
//!
//! **Why ordinals, not item ids.** A tab is a `BinderItem`, but the backend store
//! is an ephemeral `HashMap` whose ids are **remapped on every load** (a `.skrib`
//! stores `file_id`s, `load_work` mints fresh store ids in binder order). Those
//! ids are not stable across a save→load cycle — an id can even shift when an
//! unrelated tag or dictionary word is added earlier in the load. So a tab is
//! persisted by its **position** in the work's ordered binder-item stream (its
//! flat, binder-major ordinal), which *does* round-trip: the save writes the
//! stream order, the load reproduces it, and the ordinal maps 1:1 to the new id.
//! See [`WorkspaceLayoutViewModel`](crate::workspace_layout::WorkspaceLayoutViewModel)
//! for the capture/restore that translates ordinals ↔ store ids.
//!
//! **Cross-process safety.** Skribisto is single-instance: normally one process
//! hosts every open project window, sharing this same `workspace.toml`. Two
//! windows (or, rarely, a `--new-instance` fallback process) writing different
//! projects' rows never clobber each other — `SettingsFile`'s locked
//! read-modify-write ([`mutate`](teksilo::settings::SettingsFile::mutate)) is the
//! only write mode. No window needs another's *live* layout (each
//! captures/restores its own at close/load), so — unlike `search.toml` — this
//! file is not registered as a `Reloadable`.

use std::collections::HashSet;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use teksilo::settings::{AppPaths, Migrator, SettingsFile, SettingsFileError, Versioned};
use teksilo::widgets::{DockLayoutState, SplitterState};
use uuid::Uuid;

/// Accepted for call-site stability only. `SettingsFile`'s writes are a
/// synchronous locked read-modify-write — there is no debounce to configure
/// (see [`SearchSettingsService`](super::SearchSettingsService)).
const SETTINGS_DEBOUNCE: Duration = Duration::from_millis(500);

/// Cap on the number of project rows kept in `workspace.toml` — a backstop
/// against unbounded growth: orphan rows for deleted projects, or a legacy
/// uid-less `.skrib` that mints a fresh `unique_id` on every open. Generous (a
/// user rarely juggles this many distinct projects); newest kept, oldest evicted.
const MAX_PROJECTS: usize = 128;

/// One editor pane's persisted tabs.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PaneLayout {
    /// The open items' **durable uids** (`BinderItem.uid`), in tab order.
    ///
    /// This was a list of *ordinals* — positions in the work's flat binder-item stream —
    /// through v1, because nothing about a `BinderItem` was stable enough to key by. A
    /// position is not an identity: anything inserted, removed or moved in the binder
    /// between capture and the next open shifted every later ordinal, so the desk
    /// silently reopened a neighbour of each tab the writer had left open. `.skrib` v3's
    /// `uid` survives a save → load, so v2 stores that instead.
    #[serde(default)]
    pub tabs: Vec<Uuid>,
    /// The uid of the selected tab (one of [`Self::tabs`]), or `None` when the pane is
    /// empty.
    #[serde(default)]
    pub selected: Option<Uuid>,
    /// The **pinned** tabs' durable uids, in tab order — always a prefix of
    /// [`Self::tabs`], because a pin is an ordering claim: pinned tabs sort to the
    /// head of the pane and the bulk-close commands skip them.
    ///
    /// A **sidecar list**, not a `pinned` flag on [`TabViewState`], for three
    /// independent reasons — each of which has already caught someone here:
    ///
    /// * `TabViewState` is reused verbatim for the project-wide
    ///   [`PerProjectLayout::item_view_states`] roster, where a *per-pane* pin means
    ///   nothing at all: the roster knows items, not tabs, and the same item can be
    ///   open pinned in one pane and unpinned in the other. That is exactly the trap
    ///   [`corkboard`](TabViewState::corkboard) already hits, which is why
    ///   [`capture`](crate::workspace_layout::WorkspaceLayoutViewModel::capture)
    ///   zeroes that field by hand on every row it folds into the roster.
    /// * [`Self::view_states`] is built by a `filter_map` that needs **both** a
    ///   resolvable uid **and** a live view state — a strictly narrower set than
    ///   [`Self::tabs`]. A flag carried there would silently vanish for a tab that
    ///   is in `tabs`, and that tab would come back unpinned with nothing to show
    ///   why.
    /// * Restore applies `view_states` in a seeding pass that runs *after*
    ///   `set_split` and after both `open_in` loops — too late to influence
    ///   ordering, which is the one thing a pin has to influence.
    ///
    /// Additive, with a serde default, so every v7 file carries forward with no
    /// pins rather than failing to load.
    #[serde(default)]
    pub pinned: Vec<Uuid>,
    /// Where the writer was in each tab: caret offset and page scroll.
    ///
    /// A **sidecar** rather than a richer [`Self::tabs`] element, and keyed by
    /// uid rather than positional, for two independent reasons. Widening `tabs`
    /// from `Vec<Uuid>` would change its element *type*, which no old file can
    /// deserialize — this way `#[serde(default)]` carries every v3 file forward
    /// with nobody losing their tabs. And a tab list reorders (drag, or a
    /// migration between panes) without the positions meaning anything, exactly
    /// the mistake v1's ordinals made.
    ///
    /// Entries whose uid no longer resolves are simply never applied, the same
    /// way `resolve_uids` drops a tab whose item has been deleted.
    #[serde(default)]
    pub view_states: Vec<TabViewState>,
}

/// One tab's remembered caret + page scroll.
///
/// `caret` is a character offset, so it survives a typography change; `scroll`
/// is in logical pixels and is clamped to the page's real range on restore (see
/// `view_models::ViewStatePorts::apply_scroll`).
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TabViewState {
    pub uid: Uuid,
    #[serde(default)]
    pub caret: usize,
    #[serde(default)]
    pub scroll: f32,
    /// Where the writer had navigated to on this tab's Corkboard segment.
    ///
    /// Additive, with a serde default, so every v5 document still deserializes —
    /// the same shape the v3 → v4 `view_states` bump took. Not `Copy` any more
    /// (the query is a `String`); nothing depended on that.
    #[serde(default)]
    pub corkboard: CorkboardTabState,
    /// The segment (Overview / Corkboard / Stream / …) this tab was last showing,
    /// as its stable **string** id. `""` means none recorded: a segment-less item
    /// type, or a tab captured before this field existed.
    ///
    /// Persisted as the string, never the derived numeric `SegmentId`:
    /// `segments::segment_id` mints that number from the string by a one-way
    /// FNV-1a hash, so a stored number could never be resolved back to a segment
    /// after a restart; the string is the only form a later launch can look up
    /// again. Same reason `RememberSegment` (`tabs::shared::panes::remember`)
    /// persists the string half of its own `(string id, SegmentId)` table.
    #[serde(default)]
    pub segment: String,
}

/// One tab's remembered Corkboard navigation.
///
/// The board is the one editor surface with navigation of its own — drilling into
/// a folder card re-scopes it *in place* — so reopening a project put every board
/// back at its tab's own container, however deep the writer had gone.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct CorkboardTabState {
    /// The drilled-into breadcrumb trail as durable uids, root-first and
    /// root-inclusive. Empty = never drilled (the tab's own container).
    ///
    /// The *whole* trail rather than just the deepest container, because the
    /// breadcrumb is exactly this list — recovering ancestors from one uid would
    /// mean re-deriving them from indent math on restore, and getting a different
    /// answer than the trail the writer actually walked. A uid that no longer
    /// resolves truncates the trail there, the same graceful degradation
    /// `resolve_uids` applies to a tab whose item has been deleted.
    #[serde(default)]
    pub trail: Vec<Uuid>,
    /// The live filter text, restored into the search field so the board comes
    /// back filtered *and visibly so* — the query is in the box and the card
    /// count reflects it, so a short board is never unexplained.
    #[serde(default)]
    pub query: String,
}

impl CorkboardTabState {
    /// Nothing worth persisting (never drilled, nothing typed).
    pub fn is_empty(&self) -> bool {
        self.trail.is_empty() && self.query.is_empty()
    }
}

impl PaneLayout {
    /// Nothing worth restoring (no open tabs).
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
}

/// Cap on [`PerProjectLayout::item_view_states`], the same kind of backstop as
/// [`MAX_PROJECTS`]: a project can have more distinct items ever opened than
/// anyone keeps a caret position worth restoring for. Newest kept (the front of
/// the roster, see [`touch`]), oldest evicted.
const MAX_ITEM_VIEW_STATES: usize = 256;

/// Upsert `state` into `roster` by [`TabViewState::uid`], moving it to the
/// **front** (newest first), then truncate to [`MAX_ITEM_VIEW_STATES`] so the
/// oldest entries fall off.
///
/// Moving to the front rather than leaving it in place is what makes the
/// eviction axis "longest since visited" instead of "first ever opened": a
/// plain push-and-truncate would evict by insertion order, which for an item
/// the writer keeps coming back to is exactly the wrong one, since the entry
/// with the oldest insertion could be the item open right now.
pub fn touch(roster: &mut Vec<TabViewState>, state: TabViewState) {
    roster.retain(|s| s.uid != state.uid);
    roster.insert(0, state);
    roster.truncate(MAX_ITEM_VIEW_STATES);
}

/// Drop every entry whose uid is not in `live`: an item trashed or deleted
/// since it was last recorded.
///
/// No cascade reaches this roster the way trashing sweeps a plan's bindings
/// elsewhere: it lives beside the binder, not inside it, so without an explicit
/// prune a project's item history would only ever grow, quietly pushing
/// genuinely-live entries out past [`MAX_ITEM_VIEW_STATES`] ahead of the dead
/// ones that no longer mean anything.
pub fn prune(roster: &mut Vec<TabViewState>, live: &HashSet<Uuid>) {
    roster.retain(|s| live.contains(&s.uid));
}

/// One project's complete workspace layout.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PerProjectLayout {
    pub work_uid: String,
    /// Display/debug only (the key is [`Self::work_uid`]).
    #[serde(default)]
    pub last_path: String,
    /// The primary (always-present) editor pane.
    #[serde(default)]
    pub primary: PaneLayout,
    /// The secondary (side) editor pane.
    #[serde(default)]
    pub secondary: PaneLayout,
    /// The secondary pane was the focused one (drives the open-item marker).
    #[serde(default)]
    pub focus_secondary: bool,
    /// The two editor panes' splitter ratio, if the split was shown.
    #[serde(default)]
    pub editor_splitter: Option<SplitterState>,
    /// The dock layout (per-side size / presentation / selected-tab / arrangement /
    /// corners). `None` for a legacy entry written before docks were persisted, or
    /// a blob this build can't read (see [`lenient_docks`]).
    #[serde(default, deserialize_with = "lenient_docks")]
    pub docks: Option<DockLayoutState>,
    /// Every dock id the build that wrote [`Self::docks`] **knew about**, whether or
    /// not it was open at the time.
    ///
    /// This is what makes "absent from the snapshot" readable. A `DockLayoutState`
    /// is a closed list and `import_state` rebuilds the rail purely from it, so a
    /// dock the app registers but the snapshot never mentions is silently dropped.
    /// Without this field the two reasons a dock can be missing are
    /// indistinguishable:
    ///
    /// * **the user closed it** — `close_dock` removes it from the layout; it must
    ///   stay closed, or every launch would overrule that choice; versus
    /// * **it did not exist yet** — a dock shipped after this desk was last
    ///   captured; it must be mounted, or the feature is invisible to everyone with
    ///   an existing project.
    ///
    /// Recording the roster resolves it by subtraction: *listed but absent* is the
    /// first case, *unlisted* is the second. That is what let v5 replace the
    /// blunt "drop every saved arrangement" migration the Format dock needed at
    /// v2 → v3 (see [`migrator`]), and what makes the next dock need no migration
    /// at all.
    #[serde(default)]
    pub known_docks: Vec<u64>,
    /// The **per-`BinderItem`** roster: where the writer was in an item, whether
    /// or not a tab on it is open right now. Newest-first (see [`touch`]).
    ///
    /// Distinct from [`PaneLayout::view_states`], which only ever knows about a
    /// pane's *currently open* tabs and stays authoritative for restoring one:
    /// the same item open in both split panes legitimately has two carets, and
    /// only the per-pane roster can tell them apart. This roster is what lets
    /// reopening an item the writer had **closed**, this session or a later one,
    /// come back to where they left it; a closed tab is in no pane, so no
    /// per-pane list can ever remember it. Capped at [`MAX_ITEM_VIEW_STATES`] and
    /// swept of dead uids by [`prune`], since nothing else ever removes an entry
    /// on this item's behalf.
    #[serde(default)]
    pub item_view_states: Vec<TabViewState>,
}

/// Deserialize the embedded [`DockLayoutState`] **tolerantly**: on any error, drop
/// it to `None` rather than failing the whole `WorkspaceLayoutFile`.
///
/// `DockLayoutState` is an *external*, independently-`Versioned` type with its own
/// migrator — but embedded here as a plain field it bypasses that migrator (only
/// the outer file's version is walked). Serde fails a whole container if any one
/// element fails, so without this a future non-additive `DockLayoutState` schema
/// change (its own tests show a required-field rename is a real precedent) would
/// fail-load *every* project's row, silently disabling all workspace persistence
/// until the file is hand-fixed. Parsing through a `toml::Value` and swallowing the
/// error contains the blast radius to that one project's dock layout — its tabs and
/// every other project's desk still load.
fn lenient_docks<'de, D>(d: D) -> Result<Option<DockLayoutState>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = toml::Value::deserialize(d)?;
    match DockLayoutState::deserialize(value) {
        Ok(state) => Ok(Some(state)),
        // Log the dropped blob so a *genuine* serialization bug is distinguishable
        // from the intended tolerance of a future-incompatible schema.
        Err(e) => {
            eprintln!("skribisto: workspace layout: unreadable dock state dropped ({e})");
            Ok(None)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WorkspaceLayoutFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub projects: Vec<PerProjectLayout>,
}

/// The `version` field's serde default, for a `WorkspaceLayoutFile`
/// deserialized **outside** `SettingsFile`'s load path. It is not what decides
/// how a versionless file on disk is read.
///
/// **A file with no `version` key is treated as v1, not as current**, and every
/// migration step runs against it — including v1 → v2, which drops its tab lists.
/// That is `Migrator::run`'s own rule (`peek_version(&raw).unwrap_or(1)`, then
/// walk to `CURRENT_VERSION`), and every load here goes through it
/// (`SettingsFile::load`). The walk also stamps a `version` into the raw table
/// after each step, so on that path this default is never even reached; it exists
/// so a value built straight from a `toml::Value` still carries a coherent
/// version rather than 0.
///
/// This doc claimed the exact opposite until v8 — "assumed already current, so no
/// migration runs for it" — and reasoned at length from that. Nothing in the tree
/// ever behaved that way: `the_v1_migration_drops_ordinal_tabs_and_keeps_the_rest`
/// passes because its fixture stamps `version = 1` by hand, so the claim was
/// never under test. Believing it would mislead the next schema bump into
/// thinking a legacy file is safe from its own steps.
fn default_version() -> u32 {
    WorkspaceLayoutFile::CURRENT_VERSION
}

impl Default for WorkspaceLayoutFile {
    fn default() -> Self {
        WorkspaceLayoutFile {
            version: WorkspaceLayoutFile::CURRENT_VERSION,
            projects: Vec::new(),
        }
    }
}

impl Versioned for WorkspaceLayoutFile {
    /// **v2** re-keyed the persisted tabs from stream ordinals to durable
    /// `BinderItem.uid`s (see [`PaneLayout::tabs`]).
    ///
    /// **v3** drops the persisted dock arrangement so the Format dock reaches
    /// projects saved before it existed (see [`migrator`]).
    ///
    /// **v4** adds per-tab caret + scroll ([`PaneLayout::view_states`]). Purely
    /// additive — no data is reshaped and nothing is dropped.
    ///
    /// **v5** records [`PerProjectLayout::known_docks`] so a dock added after a desk
    /// was captured can be told apart from one the user closed (see [`migrator`]).
    /// Additive too — and it is what retires the drop-everything approach v3 took.
    ///
    /// **v6** records each tab's Corkboard navigation in
    /// [`TabViewState::corkboard`]. Additive, with a serde default, exactly like
    /// v4's `view_states`: a v5 document already deserializes correctly, and the
    /// step exists only to stamp the version so an older build is *refused* by the
    /// `Migrator` rather than silently rewriting the file and dropping the field.
    ///
    /// **v7** adds the per-`BinderItem` roster
    /// ([`PerProjectLayout::item_view_states`]) and each tab's last segment
    /// ([`TabViewState::segment`]). Additive, with serde defaults, exactly like
    /// v4's `view_states` and v6's `corkboard`: a v6 document already deserializes
    /// correctly, and the step exists only to stamp the version so an older build
    /// is *refused* by the `Migrator` rather than silently rewriting the file and
    /// dropping the roster.
    ///
    /// **v8** records which of a pane's tabs were **pinned**
    /// ([`PaneLayout::pinned`]). Additive, with a serde default, exactly like v4's
    /// `view_states`, v6's `corkboard` and v7's roster: a v7 document already
    /// deserializes correctly under v8 and there is nothing to transform. The step
    /// exists only to stamp the version so an older build is *refused* by the
    /// `Migrator` rather than silently rewriting the file and unpinning every tab.
    const CURRENT_VERSION: u32 = 8;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// The migrator for `workspace.toml`.
///
/// **v1 → v2 drops every persisted tab list.** v1 stored stream *ordinals*; v2 stores
/// durable uids, and the two cannot be translated here — an ordinal only means anything
/// against the item stream of a *loaded* project, which a settings migration does not
/// have (and there is one file for every project the user has ever opened, not one).
///
/// So the tabs are cleared and everything else in the row is kept. The user-visible cost
/// is exactly one launch on which the remembered open tabs do not come back; the docks,
/// splitter and focused pane all survive, and the next close re-captures the tabs by uid.
/// Translating wrongly — reopening a *neighbour* of each tab — is the failure this whole
/// change exists to remove, so dropping is the honest option.
///
/// **v2 → v3 drops the persisted dock arrangement**, for a related reason. A saved
/// arrangement is an exported blob listing the docks that existed when it was written, and
/// `import_state` restores exactly that — so a dock added later is simply never mounted,
/// and the Format dock would stay invisible to every project the user had already opened.
/// Re-asserting it after the import is not an option either: `close_dock` *removes* a dock
/// from the layout, so "absent" is indistinguishable from "the user closed it", and
/// re-opening would silently overrule that choice every launch.
///
/// Dropping the arrangement makes each project fall back to the default docks once, which
/// now include Format. The cost is one launch on which a customised dock layout returns to
/// the default; tabs, splitter and focused pane are untouched, and the next close
/// re-captures the arrangement. Cheap at alpha, and honest — the alternative is a feature
/// nobody with an existing project can find.
///
/// **v4 → v5 keeps everything**, and retires that trade-off. The two comments docks hit
/// exactly the v2 → v3 problem — every project captured before they shipped restored
/// without them — but this time the fix records *why* a dock is missing instead of
/// destroying the evidence: each row is stamped with the roster its author knew, so the
/// restore-time reconcile can mount what is genuinely new and leave closed what the user
/// closed (see [`PerProjectLayout::known_docks`]). Nothing is dropped; a customised dock
/// layout survives, and the next dock added needs no migration step at all.
fn migrator() -> Migrator<WorkspaceLayoutFile> {
    Migrator::new()
        .step(1, |mut raw| {
            if let Some(projects) = raw.get_mut("projects").and_then(|p| p.as_array_mut()) {
                for project in projects.iter_mut() {
                    for pane in ["primary", "secondary"] {
                        if let Some(t) = project.get_mut(pane).and_then(|p| p.as_table_mut()) {
                            t.remove("tabs");
                            t.remove("selected");
                        }
                    }
                }
            }
            Ok(raw)
        })
        .step(2, |mut raw| {
            if let Some(projects) = raw.get_mut("projects").and_then(|p| p.as_array_mut()) {
                for project in projects.iter_mut() {
                    if let Some(t) = project.as_table_mut() {
                        t.remove("docks");
                    }
                }
            }
            Ok(raw)
        })
        // **v3 → v4 is the identity.** `view_states` is a brand-new field with a
        // serde default, so a v3 document already deserializes correctly under
        // v4 and there is nothing to transform. The step exists only to stamp
        // the version, which is what makes a *downgrade* safe: an older build
        // meeting a v4 file is refused by the `Migrator` rather than silently
        // rewriting it and dropping everyone's caret positions. Same shape as
        // `dictionary_settings_file`'s own additive bump.
        .step(3, Ok)
        // **v4 → v5: stamp each row with the roster its author knew.**
        //
        // These ids are written out as literals, and deliberately *not* read from
        // `crate::docks`, because they are a historical fact rather than a current
        // one: they are the docks a build that could produce a v4 file knew about —
        // outline, search, inspector, preview, trash, format. `docks::APP_DOCKS`
        // describes today's app and will keep growing; pointing at it here would
        // silently rewrite history on every future release, telling the reconcile a
        // v4 desk already knew about docks that did not exist when it was saved —
        // which is precisely the "silently never mounted" bug this exists to fix. A
        // migration step must stay frozen at the moment it describes.
        //
        // Absent from this list, and so correctly seen as new by the reconcile:
        // `COMMENTS_DOCK_ID` (0xD0C_0007) and `DOC_COMMENTS_DOCK_ID` (0xD0C_0008).
        .step(4, |mut raw| {
            const V4_DOCKS: [i64; 6] = [
                0xD0C_0001, 0xD0C_0002, 0xD0C_0003, 0xD0C_0004, 0xD0C_0005, 0xD0C_0006,
            ];
            if let Some(projects) = raw.get_mut("projects").and_then(|p| p.as_array_mut()) {
                for project in projects.iter_mut() {
                    if let Some(t) = project.as_table_mut() {
                        // A row with no `docks` blob restores from the *defaults*,
                        // which already carry the whole current roster — stamping it
                        // would be a claim about a snapshot that does not exist, and
                        // would then suppress the reconcile for a row that never
                        // needed it.
                        if !t.contains_key("docks") {
                            continue;
                        }
                        t.insert(
                            "known_docks".to_string(),
                            toml::Value::Array(
                                V4_DOCKS.iter().copied().map(toml::Value::Integer).collect(),
                            ),
                        );
                    }
                }
            }
            Ok(raw)
        })
        // **v5 → v6 is the identity**, for the same reason v3 → v4 was:
        // `TabViewState::corkboard` is a brand-new field with a serde default, so a
        // v5 document already deserializes correctly under v6 and there is nothing
        // to transform. The step stamps the version, which is what makes a
        // *downgrade* safe — an older build meeting a v6 file is refused by the
        // `Migrator` rather than silently rewriting it and dropping every board's
        // remembered navigation.
        .step(5, Ok)
        // **v6 → v7 is the identity**, for the same reason v3 → v4 and v5 → v6
        // were: `PerProjectLayout::item_view_states` and `TabViewState::segment`
        // are both brand-new fields with serde defaults, so a v6 document already
        // deserializes correctly under v7 and there is nothing to transform. The
        // step stamps the version, which is what makes a *downgrade* safe: an
        // older build meeting a v7 file is refused by the `Migrator` rather than
        // silently rewriting it and dropping the per-item roster.
        .step(6, Ok)
        // **v7 → v8 is the identity**, for the same reason v3 → v4, v5 → v6 and
        // v6 → v7 were: `PaneLayout::pinned` is a brand-new field with a serde
        // default, so a v7 document already deserializes correctly under v8 and
        // there is nothing to transform. What the step buys is the *stamp*, which
        // is the only **downgrade** protection there is: an older build meeting a
        // v8 file is refused outright by the `Migrator`
        // (`MigrationError::NewerThanCurrent`) instead of reading it, ignoring a
        // field it has never heard of, and unpinning every tab on its next
        // capture.
        .step(7, Ok)
}

/// Persistent workspace-layout service. `SettingsFile` is `Clone` (shares the
/// in-memory state + writer), so cloning hands out views over the same live file.
#[derive(Clone)]
pub struct WorkspaceLayoutService {
    file: SettingsFile<WorkspaceLayoutFile>,
}

impl WorkspaceLayoutService {
    /// Open `workspace.toml` under `paths` (cross-process safe).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        Self::open_with_delay(paths, SETTINGS_DEBOUNCE)
    }

    /// `delay` is accepted for call-site stability but has no effect (writes are a
    /// synchronous locked read-modify-write, no debounce).
    pub fn open_with_delay(paths: &AppPaths, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("workspace"), migrator())?;
        Ok(Self { file })
    }

    /// Open at an explicit path — used by tests.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn open_at(path: std::path::PathBuf, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, migrator())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable: a throwaway per-process
    /// temp file, so the app still runs (the layout just won't persist across
    /// restarts). This one is opened *eagerly, before any window* (see `app.rs`), so it
    /// is the one call site where an infallible fallback matters most — a fallback that
    /// could itself fail would kill every launch, not just this one setting. Shares its
    /// retry/uniqueness logic with every sibling via
    /// [`in_memory_settings_file`](super::backup_settings_file::in_memory_settings_file).
    pub fn in_memory_default() -> Self {
        let file = super::backup_settings_file::in_memory_settings_file("workspace", migrator());
        Self { file }
    }

    /// This project's saved layout, if any.
    pub fn get(&self, work_uid: &str) -> Option<PerProjectLayout> {
        if !super::uid_is_usable(work_uid) {
            return None;
        }
        self.file
            .borrow()
            .projects
            .iter()
            .find(|p| p.work_uid == work_uid)
            .cloned()
    }

    /// Persist `layout` as its project's entry (upsert). A no-op when:
    ///  - the uid is empty (a brand-new unsaved project — keying `""` would
    ///    collide across unrelated projects); or
    ///  - the stored row is already byte-identical (an autosave whose desk hasn't
    ///    changed must not rewrite the whole file).
    ///
    /// Otherwise it replaces any prior row for this **uid** *and* any row for the
    /// same **path** — a legacy uid-less `.skrib` mints a fresh uid every open, so
    /// the path is what actually identifies the file, and de-duping on it keeps one
    /// row per project instead of one per open. The row list is bounded at
    /// [`MAX_PROJECTS`] (oldest evicted).
    pub fn set(&self, layout: PerProjectLayout) -> Result<(), SettingsFileError> {
        if !super::uid_is_usable(&layout.work_uid) {
            return Ok(());
        }
        // Fast-path: skip a no-op write (unchanged desk on an autosave tick).
        if self
            .file
            .borrow()
            .projects
            .iter()
            .find(|p| p.work_uid == layout.work_uid)
            == Some(&layout)
        {
            return Ok(());
        }
        self.file.mutate(|f| {
            let path = layout.last_path.clone();
            f.projects.retain(|p| {
                p.work_uid != layout.work_uid && (path.is_empty() || p.last_path != path)
            });
            f.projects.push(layout); // newest at the end
            let len = f.projects.len();
            if len > MAX_PROJECTS {
                f.projects.drain(0..len - MAX_PROJECTS); // evict the oldest
            }
        })
    }

    /// Drop `work_uid`'s entry (project deleted / no longer wanted). No-op if absent.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn forget(&self, work_uid: &str) -> Result<(), SettingsFileError> {
        self.file
            .mutate(|f| f.projects.retain(|p| p.work_uid != work_uid))
    }

    pub fn flush_now(&self) -> Result<(), SettingsFileError> {
        self.file.flush_now()
    }
}

#[cfg(test)]
mod tests;
