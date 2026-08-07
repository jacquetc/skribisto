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
//! See [`WorkspaceLayoutViewModel`](crate::view_models::WorkspaceLayoutViewModel)
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

use std::time::Duration;

use teksilo::settings::{AppPaths, Migrator, SettingsFile, SettingsFileError, Versioned};
use teksilo::widgets::{DockLayoutState, SplitterState};
use serde::{Deserialize, Serialize};
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

/// A file with no `version` key is assumed already current, so **no migration
/// runs for it**.
///
/// That is the long-standing behaviour and it stays, because the two cases it
/// conflates are indistinguishable from here: a hand-edited current file and a
/// legacy one written before the field existed. Assuming *old* instead would
/// re-run every step against files that are already correct, which for v1 means
/// silently dropping their tab lists.
///
/// The cost is now user-visible rather than theoretical: such a file keeps its
/// stale `docks` blob, so a dock added later (Format, in v3) never appears for
/// it. If that is ever reported, the fix is a repair pass keyed on content —
/// "no `docks` entry mentions the format dock" — not a change to this default.
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
    const CURRENT_VERSION: u32 = 6;
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
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn svc(dir: &std::path::Path) -> WorkspaceLayoutService {
        WorkspaceLayoutService::open_at(dir.join("workspace.toml"), Duration::ZERO).unwrap()
    }

    fn u(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    fn sample(uid: &str) -> PerProjectLayout {
        PerProjectLayout {
            work_uid: uid.to_string(),
            // Distinct paths per uid — a real project is one file (dedup-by-path).
            last_path: format!("/x/{uid}.skrib"),
            primary: PaneLayout {
                tabs: vec![u(3), u(7), u(1)],
                selected: Some(u(7)),
                view_states: Vec::new(),
            },
            secondary: PaneLayout {
                tabs: vec![u(9)],
                selected: Some(u(9)),
                view_states: Vec::new(),
            },
            focus_secondary: false,
            editor_splitter: None,
            docks: Some(DockLayoutState::default()),
            known_docks: vec![1, 2, 3],
        }
    }

    /// A v1 file's ordinal tab lists are **dropped**, not translated — an ordinal only
    /// means something against a loaded project's item stream, which a settings migration
    /// does not have. Everything else in the row survives, so the user loses exactly one
    /// launch's remembered tabs rather than reopening the wrong items forever.
    #[test]
    fn the_v1_migration_drops_ordinal_tabs_and_keeps_the_rest() {
        let d = tempdir().unwrap();
        let path = d.path().join("workspace.toml");
        std::fs::write(
            &path,
            r#"version = 1
[[projects]]
work_uid = "uid-A"
last_path = "/x/a.skrib"
focus_secondary = true
[projects.primary]
tabs = [3, 7, 1]
selected = 7
[projects.secondary]
tabs = [9]
selected = 9
"#,
        )
        .unwrap();

        let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
        let got = s.get("uid-A").expect("the row survived the migration");
        assert!(
            got.primary.tabs.is_empty() && got.primary.selected.is_none(),
            "v1 ordinals cannot be translated, so they are dropped"
        );
        assert!(got.secondary.tabs.is_empty());
        assert_eq!(got.last_path, "/x/a.skrib", "the rest of the row is kept");
        assert!(got.focus_secondary, "including the focused pane");
    }

    /// **v3 → v4 loses nothing.** Unlike v1→v2 and v2→v3, which both had to drop
    /// data they could not translate, `view_states` is a brand-new field: an
    /// existing file loads with every tab, selection, splitter and dock intact
    /// and simply no remembered caret positions yet. The version bump exists so
    /// an *older* build meeting a v4 file is refused rather than silently
    /// rewriting it.
    #[test]
    fn the_v4_migration_is_additive_and_keeps_every_tab() {
        let d = tempdir().unwrap();
        let path = d.path().join("workspace.toml");
        std::fs::write(
            &path,
            r#"version = 3
[[projects]]
work_uid = "uid-A"
last_path = "/x/a.skrib"
focus_secondary = true
[projects.primary]
tabs = ["00000000-0000-0000-0000-000000000003"]
selected = "00000000-0000-0000-0000-000000000003"
[projects.secondary]
tabs = []
"#,
        )
        .unwrap();

        let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
        let got = s.get("uid-A").expect("the row survived the migration");
        assert_eq!(got.primary.tabs.len(), 1, "a v3 file keeps its tabs");
        assert!(got.primary.selected.is_some());
        assert_eq!(got.last_path, "/x/a.skrib");
        assert!(got.focus_secondary);
        assert!(
            got.primary.view_states.is_empty(),
            "nothing remembered yet, but the field must exist rather than fail the load"
        );
    }

    /// **v5 → v6 is additive and keeps every tab and caret.** Same shape as v4:
    /// `TabViewState::corkboard` is a brand-new field with a serde default, so a v5
    /// document loads whole and merely gains an empty board state. The step exists
    /// so an *older* build meeting a v6 file is refused rather than silently
    /// rewriting it and dropping every board's remembered navigation.
    #[test]
    fn the_v6_migration_is_additive_and_keeps_every_tab_and_caret() {
        let d = tempdir().unwrap();
        let path = d.path().join("workspace.toml");
        std::fs::write(
            &path,
            r#"version = 5
[[projects]]
work_uid = "uid-A"
last_path = "/x/a.skrib"
known_docks = [13631489]
[projects.primary]
tabs = ["00000000-0000-0000-0000-000000000003"]
selected = "00000000-0000-0000-0000-000000000003"
[[projects.primary.view_states]]
uid = "00000000-0000-0000-0000-000000000003"
caret = 412
scroll = 96.5
[projects.secondary]
tabs = []
"#,
        )
        .unwrap();

        let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
        let got = s.get("uid-A").expect("the row survived the migration");
        assert_eq!(got.primary.tabs.len(), 1, "a v5 file keeps its tabs");
        assert_eq!(got.last_path, "/x/a.skrib");
        assert_eq!(got.known_docks, vec![13631489], "and its dock roster");
        let vs = got.primary.view_states.first().expect("the caret survived");
        assert_eq!(vs.caret, 412);
        assert_eq!(vs.scroll, 96.5);
        assert!(
            vs.corkboard.is_empty(),
            "no board navigation remembered yet, but the field must exist rather \
             than fail the load"
        );
    }

    /// A drilled board's trail and filter survive a real write and re-read — the
    /// half of the round trip the in-memory view-model test cannot cover.
    #[test]
    fn corkboard_board_state_round_trips_through_disk() {
        let d = tempdir().unwrap();
        let mut rec = sample("uid-A");
        rec.primary.view_states = vec![TabViewState {
            uid: u(3),
            caret: 0,
            scroll: 0.0,
            corkboard: CorkboardTabState {
                trail: vec![u(1), u(2), u(3)],
                query: "ferry".into(),
            },
        }];
        {
            let s = svc(d.path());
            s.set(rec).unwrap();
        }
        let s = svc(d.path());
        let got = s.get("uid-A").expect("row");
        let board = &got.primary.view_states[0].corkboard;
        assert_eq!(board.trail, vec![u(1), u(2), u(3)]);
        assert_eq!(board.query, "ferry");
    }

    /// **v4 → v5 stamps the roster and destroys nothing.** This is the migration that
    /// let the comments docks reach existing projects *without* repeating v2 → v3's
    /// blanket "drop every saved arrangement": the row keeps its docks, tabs, splitter
    /// and focused pane, and merely gains the list of docks its author knew.
    #[test]
    fn the_v5_migration_stamps_the_v4_roster_and_keeps_the_saved_docks() {
        let d = tempdir().unwrap();
        let path = d.path().join("workspace.toml");
        // Build a genuine v4 document: a real serialized `DockLayoutState` (a
        // hand-written partial one is silently dropped by `lenient_docks`, which
        // would make this test pass for the wrong reason), stamped back to version 4
        // with the v5-only key removed.
        let mut f = WorkspaceLayoutFile {
            version: 4,
            projects: vec![PerProjectLayout {
                focus_secondary: true,
                ..sample("uid-A")
            }],
        };
        f.projects[0].known_docks.clear();
        let text = toml::to_string(&f)
            .unwrap()
            .replace("known_docks = []\n", "");
        assert!(
            !text.contains("known_docks"),
            "the fixture must be a real v4 file — no v5 key"
        );
        std::fs::write(&path, text).unwrap();

        let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
        let got = s.get("uid-A").expect("the row survived the migration");
        assert_eq!(
            got.known_docks,
            vec![
                0xD0C_0001, 0xD0C_0002, 0xD0C_0003, 0xD0C_0004, 0xD0C_0005, 0xD0C_0006
            ],
            "stamped with the six docks a v4-era build knew — comments (7, 8) deliberately absent"
        );
        assert!(
            got.docks.is_some(),
            "unlike v2 -> v3, the saved arrangement is KEPT"
        );
        assert_eq!(
            got.primary.tabs,
            vec![u(3), u(7), u(1)],
            "and so are the tabs, in order"
        );
        assert_eq!(got.secondary.tabs, vec![u(9)]);
        assert!(got.focus_secondary, "and the focused pane");
    }

    /// A row with **no** `docks` blob is not stamped. It restores from the pristine
    /// defaults, which already carry the whole current roster, so claiming it knew
    /// only the v4 six would be false — and would then suppress a reconcile for a row
    /// that never needed one.
    #[test]
    fn the_v5_migration_does_not_stamp_a_row_that_saved_no_docks() {
        let d = tempdir().unwrap();
        let path = d.path().join("workspace.toml");
        std::fs::write(
            &path,
            "version = 4\n\n[[projects]]\nwork_uid = \"uid-A\"\n[projects.primary]\n\
             tabs = [\"00000000-0000-0000-0000-000000000001\"]\n",
        )
        .unwrap();
        let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
        let got = s.get("uid-A").unwrap();
        assert!(got.docks.is_none());
        assert!(
            got.known_docks.is_empty(),
            "no snapshot to describe, so no claim about what its author knew"
        );
    }

    /// A caret + scroll pair survives a real write and re-read.
    #[test]
    fn view_states_round_trip_through_disk() {
        let d = tempdir().unwrap();
        let mut rec = sample("uid-A");
        rec.primary.view_states = vec![TabViewState {
            uid: u(3),
            caret: 412,
            scroll: 96.5,
            corkboard: CorkboardTabState {
                trail: vec![u(9), u(10)],
                query: "keep".into(),
            },
        }];
        {
            let s = svc(d.path());
            s.set(rec).unwrap();
        }
        let s = svc(d.path());
        let got = s.get("uid-A").expect("row");
        assert_eq!(
            got.primary.view_states,
            vec![TabViewState {
                uid: u(3),
                caret: 412,
                scroll: 96.5,
                corkboard: CorkboardTabState {
                    trail: vec![u(9), u(10)],
                    query: "keep".into(),
                },
            }]
        );
    }

    #[test]
    fn round_trips_through_toml() {
        let d = tempdir().unwrap();
        {
            let s = svc(d.path());
            s.set(sample("uid-A")).unwrap();
            s.flush_now().unwrap();
        }
        // Reopen from disk — the layout survived the TOML round-trip.
        let s = svc(d.path());
        let got = s.get("uid-A").expect("entry present");
        assert_eq!(got.primary.tabs, vec![u(3), u(7), u(1)]);
        assert_eq!(got.primary.selected, Some(u(7)));
        assert_eq!(got.secondary.tabs, vec![u(9)]);
        assert!(got.docks.is_some());
    }

    #[test]
    fn dedup_by_path_keeps_one_row_per_file() {
        // A legacy uid-less .skrib mints a fresh uid every open, so the same path
        // arrives under a new uid each time — keep one row (the latest), not one
        // orphan per open.
        let d = tempdir().unwrap();
        let s = svc(d.path());
        let mut open1 = sample("uid-open-1");
        open1.last_path = "/x/legacy.skrib".to_string();
        let mut open2 = sample("uid-open-2");
        open2.last_path = "/x/legacy.skrib".to_string();
        s.set(open1).unwrap();
        s.set(open2).unwrap();
        assert_eq!(s.file.borrow().projects.len(), 1, "one row per file path");
        assert!(
            s.get("uid-open-1").is_none(),
            "the earlier open's orphan row is gone"
        );
        assert!(s.get("uid-open-2").is_some());
    }

    #[test]
    fn no_op_write_is_skipped() {
        // Re-setting an identical desk must not grow or reorder the file.
        let d = tempdir().unwrap();
        let s = svc(d.path());
        s.set(sample("uid-A")).unwrap();
        s.set(sample("uid-A")).unwrap();
        s.set(sample("uid-A")).unwrap();
        assert_eq!(s.file.borrow().projects.len(), 1);
    }

    #[test]
    fn row_count_is_capped() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        for i in 0..(MAX_PROJECTS + 20) {
            s.set(sample(&format!("uid-{i}"))).unwrap();
        }
        let f = s.file.borrow();
        assert_eq!(f.projects.len(), MAX_PROJECTS, "bounded at the cap");
        // Oldest evicted, newest kept.
        assert!(f.projects.iter().all(|p| p.work_uid != "uid-0"));
        assert!(
            f.projects
                .iter()
                .any(|p| p.work_uid == format!("uid-{}", MAX_PROJECTS + 19))
        );
    }

    #[test]
    fn upsert_replaces_and_isolates_per_uid() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        s.set(sample("uid-A")).unwrap();
        s.set(sample("uid-B")).unwrap();

        // Overwrite A with a different desk.
        let mut a2 = sample("uid-A");
        a2.primary.tabs = vec![u(42)];
        a2.primary.selected = Some(u(42));
        s.set(a2).unwrap();

        assert_eq!(s.get("uid-A").unwrap().primary.tabs, vec![u(42)]);
        assert_eq!(
            s.get("uid-B").unwrap().primary.tabs,
            vec![u(3), u(7), u(1)],
            "B untouched"
        );
        assert_eq!(s.file.borrow().projects.len(), 2, "no duplicate row for A");
    }

    #[test]
    fn an_empty_uid_never_persists() {
        // A brand-new unsaved project has no unique_id; keying "" would collide
        // across unrelated projects, so a set on it is silently dropped.
        let d = tempdir().unwrap();
        let s = svc(d.path());
        s.set(sample("")).unwrap();
        assert!(s.get("").is_none(), "empty uid must not persist");
        assert!(s.file.borrow().projects.is_empty());
    }

    #[test]
    fn forget_removes_only_the_named_entry() {
        let d = tempdir().unwrap();
        let s = svc(d.path());
        s.set(sample("uid-A")).unwrap();
        s.set(sample("uid-B")).unwrap();
        s.forget("uid-A").unwrap();
        assert!(s.get("uid-A").is_none());
        assert!(s.get("uid-B").is_some());
    }

    #[test]
    fn two_shared_services_over_one_file_do_not_clobber_each_others_projects() {
        // Two processes (one per project) sharing one workspace.toml, each writing
        // a different project's layout. The locked read-modify-write keeps the
        // second write's stale snapshot from dropping the first.
        let d = tempdir().unwrap();
        let path = d.path().join("workspace.toml");
        let a = WorkspaceLayoutService::open_at(path.clone(), Duration::ZERO).unwrap();
        let b = WorkspaceLayoutService::open_at(path.clone(), Duration::ZERO).unwrap();
        a.set(sample("uid-A")).unwrap();
        b.set(sample("uid-B")).unwrap();

        let c = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
        assert!(c.get("uid-A").is_some());
        assert!(c.get("uid-B").is_some());
    }

    #[test]
    fn an_unreadable_docks_blob_drops_to_none_without_failing_the_file() {
        // A project row whose embedded `docks` is a garbage/future-incompatible
        // table must load with `docks: None` and its tabs intact — and must not
        // take down a sibling project's row (blast-radius containment).
        let d = tempdir().unwrap();
        let path = d.path().join("workspace.toml");
        std::fs::write(
            &path,
            r#"
version = 2

[[projects]]
work_uid = "uid-good"
[projects.primary]
tabs = ["00000000-0000-0000-0000-000000000001", "00000000-0000-0000-0000-000000000004"]
selected = "00000000-0000-0000-0000-000000000004"

[[projects]]
work_uid = "uid-bad-docks"
[projects.primary]
tabs = ["00000000-0000-0000-0000-000000000002"]
[projects.docks]
this_is_not = "a valid DockLayoutState"
leading = 12345
"#,
        )
        .unwrap();
        let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
        // The good row is untouched.
        let good = s.get("uid-good").expect("good row present");
        assert_eq!(good.primary.tabs, vec![u(1), u(4)]);
        assert!(good.docks.is_none());
        // The bad-docks row still loads; only its docks dropped to None.
        let bad = s.get("uid-bad-docks").expect("bad-docks row still loaded");
        assert_eq!(
            bad.primary.tabs,
            vec![u(2)],
            "tabs survive an unreadable docks blob"
        );
        assert!(
            bad.docks.is_none(),
            "unreadable docks blob -> None, not a load failure"
        );
    }

    #[test]
    fn missing_optional_fields_default_cleanly() {
        // A minimal legacy-shaped row (only work_uid + a couple of tabs) must load,
        // the rest defaulting — additive schema evolution safety.
        let d = tempdir().unwrap();
        let path = d.path().join("workspace.toml");
        std::fs::write(
            &path,
            "version = 2\n\n[[projects]]\nwork_uid = \"uid-A\"\n[projects.primary]\n\
             tabs = [\"00000000-0000-0000-0000-000000000001\"]\n",
        )
        .unwrap();
        let s = WorkspaceLayoutService::open_at(path, Duration::ZERO).unwrap();
        let got = s.get("uid-A").unwrap();
        assert_eq!(got.primary.tabs, vec![u(1)]);
        assert_eq!(got.primary.selected, None);
        assert!(!got.focus_secondary);
        assert!(got.docks.is_none());
        assert!(got.secondary.is_empty());
    }
}
