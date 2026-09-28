// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Cross-instance "open project" registry.
//!
//! Skribisto is **single-instance by default**: one process hosts every project
//! window, and a second launch hands off over IPC. Multiple processes still
//! appear under `--new-instance` or when a wedged primary degrades — this
//! registry is what those peers use to list what is open and to raise each
//! other. This module does **not** assume "at most one project per process": a
//! process may hold any number of claims at once, and two different processes
//! may legitimately claim the *same* path for a moment (e.g. one closing while
//! another opens it) — both must be visible to [`scan`] so callers like
//! `BackupRestoreViewModel::check_open_elsewhere` can tell a peer still has the
//! project open before overwriting it.
//!
//! While a project is open, the claiming process writes a small lock file naming
//! that project + its own pid into a shared runtime directory. The file name
//! embeds **both** the pid and a hash of the canonical path
//! (`open-{pid}-{hash}.lock`), so two processes claiming the same path get two
//! distinct files — one can never clobber or delete the other's claim. Other
//! instances [`scan`] these lock files to show which projects are open elsewhere
//! (the ProjectSwitcher "Currently open" section) and to reach the owning
//! process — its IPC socket is derived from the pid ([`socket_name`]) — to ask it
//! to raise its window.
//!
//! The shared directory is **namespaced by installation identity** — see
//! [`namespace_for`]. `XDG_RUNTIME_DIR` is per-login-session, not
//! per-installation, so without that suffix a sandboxed run (the automation
//! scripts override `XDG_CONFIG_HOME`/`XDG_DATA_HOME`/`HOME` but not the runtime
//! dir) would share locks, sockets and the primary election with the developer's
//! own running copy.
//!
//! Crash-safe by construction: a lock whose pid is no longer alive is reaped on
//! [`scan`]; the mere presence of a file never means "open". [`scan`] also reaps
//! this instance's IPC socket files (`ipc-{pid}.sock`) once their owning pid is
//! dead — sockets are not otherwise cleaned up by anything else, so on macOS
//! (whose fallback directory is persistent) they would otherwise accumulate
//! without bound. On Windows there is nothing to reap: a named pipe is not a
//! filesystem object and cannot outlive its server. These are OS-level functions,
//! independent of the (mock or real) backend, so they are not `#[cfg]`-gated.

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

mod spelling;

pub(crate) use spelling::PathStyle;

/// One open project, as advertised by its owning instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenEntry {
    pub pid: u32,
    /// Absolute path of the `.skrib`, as [`canonical`] spells it. Compare it with
    /// another path through [`same_project`], never as a string: an earlier build
    /// wrote whatever spelling it was handed.
    pub path: String,
    pub title: String,
    /// Not open in a window: an import is writing the project there, and nothing may
    /// open or write it until the import has finished ([`claim_import`]). Absent from
    /// the lock files of builds before it, which read as `false`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub importing: bool,
}

thread_local! {
    /// This process's claims: the spelling [`canonical`] gave each project when it
    /// was claimed -> the lock file that represents it. A map (not a single slot)
    /// because one process may hold several projects open at once.
    ///
    /// Keyed by that spelling, never by [`project_key`]. The key folds case by the
    /// platform's default, and a volume formatted case-sensitive (APFS offers it,
    /// Windows sets it per folder) holds two projects whose names differ only in
    /// case: one slot, and one lock file, for both let the first to close take the
    /// other's claim with it, and every other copy of Skribisto then saw the one
    /// still open as closed. The key is for comparing only.
    static CLAIMED: RefCell<HashMap<String, PathBuf>> = RefCell::new(HashMap::new());

    /// The paths this process's imports are writing: [`project_key`] -> how many
    /// [`ImportClaim`]s hold it. Kept whether or not a lock file could be written, so
    /// this process refuses them even with no lock directory at all.
    static IMPORTING: RefCell<HashMap<String, usize>> = RefCell::new(HashMap::new());

    /// Test-only override for [`dir`], so tests never touch the real
    /// `XDG_RUNTIME_DIR` / app data dir.
    static DIR_OVERRIDE: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

#[cfg(test)]
thread_local! {
    /// Test-only override for [`pid_alive`], keyed by pid, so tests can fake a
    /// foreign pid as alive (to keep its lock from being reaped mid-assertion) or
    /// dead (to exercise the stale-reap paths) without touching real processes.
    static PID_OVERRIDE: RefCell<HashMap<u32, bool>> = RefCell::new(HashMap::new());

    /// Test-only override for the rules [`project_key`] compares by, so a test on
    /// one platform proves what another compares.
    static STYLE_OVERRIDE: std::cell::Cell<Option<PathStyle>> =
        const { std::cell::Cell::new(None) };
}

/// This process's pid — the "is this my own window?" discriminator for the UI.
pub fn my_pid() -> u32 {
    std::process::id()
}

/// The per-installation namespace suffix for [`dir`] — 16 hex chars of blake3
/// over `config_dir`.
///
/// **Why this exists.** `XDG_RUNTIME_DIR` is a *per-login-session* directory, not
/// a per-installation one, and nothing that sandboxes Skribisto sandboxes it: the
/// automation scripts override `XDG_CONFIG_HOME`/`XDG_DATA_HOME`/`HOME` and leave
/// the runtime dir pointing at the real `/run/user/{uid}`. Without a suffix, a
/// sandboxed run and the developer's own running copy share one lock directory,
/// one set of IPC sockets, and — via `shell::instance` — one *primary
/// election*: the first automation run to start would become the primary for the
/// real app, and every later launch would hand its project off into a tempdir.
/// Keying on the config dir makes each sandbox its own instance universe, while a
/// normal user (one config dir) sees exactly one namespace and no change at all.
///
/// **Hashed, not canonicalized.** `std::fs::canonicalize` answers differently
/// before and after the directory first exists, so a canonicalizing namespace
/// would silently change on the run that creates the config dir — orphaning every
/// lock and socket minted before that moment. The literal path `AppPaths` returns
/// is already deterministic for a given environment, which is the whole
/// requirement here.
///
/// **blake3, not `DefaultHasher`** — same rationale `shell::windows::window_id_for`
/// documents: `DefaultHasher`'s algorithm is explicitly not stable across Rust
/// releases, so a toolchain bump would move every live instance to a fresh
/// namespace and strand the locks in the old one. blake3 is already in this
/// workspace's dependency graph.
fn namespace_for(config_dir: &Path) -> String {
    let bytes = config_dir.as_os_str().as_encoded_bytes();
    blake3::hash(bytes).to_hex()[..16].to_string()
}

/// This installation's namespace, or `None` when no home directory is
/// detectable. Also used to name Windows pipes (see [`socket_name`]).
///
/// ⚠ **Keyed to the family, not to the running edition** — `family_paths`, never
/// `app_paths`. Every edition installed on a machine must land in one lock
/// directory so each can see what the others hold open; keying this to the
/// edition would give the community build and an extension build separate
/// universes, and `BackupRestoreViewModel::check_open_elsewhere` would stop
/// refusing to restore a backup over a project the other edition has open — the
/// other then autosaves its stale in-memory state back over the restored file,
/// silently.
///
/// The elections still separate, because the *socket name* carries the edition
/// even though the directory does not. See [`SocketId::leaf`].
///
/// Sandbox isolation is unaffected: a sandbox overrides `XDG_CONFIG_HOME`, which
/// moves the family config dir too, so the whole namespace still moves with it —
/// which is the property this function exists for.
pub fn namespace() -> Option<String> {
    crate::identity::family_paths().map(|p| namespace_for(p.config_dir()))
}

/// The shared directory holding every instance's lock + socket files. Prefers an
/// ephemeral runtime dir (`XDG_RUNTIME_DIR`); falls back to the app data dir on
/// platforms without one (macOS/Windows). That fallback is persistent, so unlike
/// the tmpfs case a reboot does not clear it — stale lock files are still reaped
/// by pid on every [`scan`], but see the module doc for why sockets needed an
/// explicit reap too.
///
/// **Only the `XDG_RUNTIME_DIR` branch carries the namespace suffix**, because it
/// is the only one that needs it: that directory is per-login-session and shared
/// by every installation and sandbox on the machine. The fallback is already
/// rooted inside *this* installation's own data dir, so it is per-installation for
/// free.
///
/// Namespacing the fallback too breaks macOS outright: there is no
/// `XDG_RUNTIME_DIR` there, so the path is
/// `~/Library/Application Support/eu.skribisto.Skribisto/run/…`, and a Unix
/// domain socket's `sun_path` holds only **104 bytes** on Darwin — the 17-byte
/// `-{16 hex}` suffix pushes every username over the cap and `bind()` fails
/// with `ENAMETOOLONG`. `the_macos_socket_path_fits_in_sun_path` pins the budget.
pub fn dir() -> Option<PathBuf> {
    if let Some(over) = DIR_OVERRIDE.with(|d| d.borrow().clone()) {
        std::fs::create_dir_all(&over).ok()?;
        return Some(over);
    }
    let session_dir: Option<PathBuf> = {
        #[cfg(unix)]
        {
            std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from)
        }
        #[cfg(not(unix))]
        {
            None
        }
    };
    let d = match session_dir {
        // Shared across installations and sandboxes — must be namespaced. No
        // `AppPaths` means no detectable home directory, hence no installation
        // identity to key on; fall back to the unsuffixed name this function used
        // before namespacing so that degraded case behaves as it always did.
        Some(base) => base.join(match namespace() {
            Some(ns) => format!("skribisto-{ns}"),
            None => "skribisto".to_string(),
        }),
        // Already inside this installation's own data dir. Every byte spent here
        // comes out of the macOS `sun_path` budget, so spend none.
        None => crate::identity::family_paths()?.data_dir().join("run"),
    };
    std::fs::create_dir_all(&d).ok()?;
    Some(d)
}

/// Point [`dir`] at a temp directory for the duration of a test (this thread
/// only). Crate-visible so a test elsewhere that consults the registry (the
/// import dialogs' open-project refusal) never reads or reaps the real one.
#[cfg(test)]
pub(crate) fn set_dir_override(path: Option<PathBuf>) {
    DIR_OVERRIDE.with(|d| *d.borrow_mut() = path);
}

/// Compare project paths in `style`'s rules instead of this platform's, for the
/// duration of a test (this thread only). Crate-visible so the doors that ask the
/// registry (New Work, the importers) can prove on Linux what Windows compares.
#[cfg(test)]
pub(crate) fn set_path_style_override(style: Option<PathStyle>) {
    STYLE_OVERRIDE.with(|s| s.set(style));
}

/// The rules [`project_key`] compares by: this platform's, unless a test set others.
fn path_style() -> PathStyle {
    #[cfg(test)]
    {
        if let Some(style) = STYLE_OVERRIDE.with(std::cell::Cell::get) {
            return style;
        }
    }
    PathStyle::HOST
}

/// Which of this installation's two socket kinds a caller means.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocketId {
    /// The well-known socket the single-instance election runs on. Exactly one
    /// live instance **of one edition** owns it — see [`SocketId::leaf`] for why
    /// this is the one name that carries an edition suffix while the directory
    /// around it does not.
    Primary,
    /// A specific instance's own socket, reachable by pid — how a peer process
    /// (a `--new-instance` sibling, or an instance of *another edition*) is asked
    /// to raise a window.
    Pid(u32),
}

impl SocketId {
    /// The leaf file name on Unix, and the distinguishing part of the pipe name
    /// on Windows.
    ///
    /// **The primary socket is per-edition; everything else is family-shared.**
    /// That split is the whole design: [`dir`] is keyed to the *family* so every
    /// edition installed on a machine reads the same lock files and can tell that
    /// a peer holds a project open (without which
    /// `BackupRestoreViewModel::check_open_elsewhere` would let one edition
    /// restore a backup over a project another has open, and the other would then
    /// autosave its stale state back over it). But the *election* must not be
    /// shared: an extension build that finds a community primary hands over its
    /// project and exits in ~half a second, and the writer gets a window with
    /// none of the extension in it. Different socket name, separate elections,
    /// same directory.
    ///
    /// The community edition keeps the bare name `primary` it has always used, so
    /// upgrading an existing install does not briefly elect two primaries while
    /// old and new processes look for different names.
    ///
    /// ⚠ A **six-hex slug**, not the readable organization name. See
    /// [`crate::identity::AppIdentity::slug`]: this lands in a path that on macOS
    /// must fit Darwin's 104-byte `sun_path`, and `primary-skribisto-pro` does
    /// not. Pinned by `the_macos_socket_path_fits_in_sun_path`.
    fn leaf(self) -> String {
        match self {
            SocketId::Primary if crate::identity::is_community() => "primary".to_string(),
            SocketId::Primary => format!("primary-{}", crate::identity::current().slug()),
            SocketId::Pid(pid) => format!("ipc-{pid}"),
        }
    }
}

/// The **path** backing `id`, on platforms where a socket is a file.
///
/// `None` on Windows, where a named pipe is not a filesystem object at all: it
/// has no path to unlink, no path to stat, and it ceases to exist when its server
/// does. Callers use this only for the file-ish chores (unlinking a stale socket,
/// reaping one whose owner died); the actual bind/connect goes through
/// [`socket_name`], which is the platform-correct address either way.
#[cfg(unix)]
pub fn socket_path(id: SocketId) -> Option<PathBuf> {
    Some(dir()?.join(format!("{}.sock", id.leaf())))
}

#[cfg(not(unix))]
pub fn socket_path(_id: SocketId) -> Option<PathBuf> {
    None
}

/// The address to bind or connect `id` on, in whatever form this platform's local
/// sockets actually take.
///
/// **Unix** — a filesystem path under [`dir`], mapped with `GenericFilePath`.
///
/// **Windows** — a *named pipe*, mapped with `GenericNamespaced`, which prepends
/// `\\.\pipe\`. Not a stylistic choice: `GenericFilePath` on Windows accepts
/// only paths that already begin `\\.\pipe\`, and our path lives under
/// `%APPDATA%` — mapping it with `GenericFilePath` fails every `to_fs_name`
/// call, so both `try_connect`/`try_bind` fail and the election falls through
/// to `Standalone`, silently disabling single-instance (and cross-process
/// raise) on Windows.
///
/// The namespace hash moves into the pipe *name* on Windows, since there is no
/// directory to put it in — the pipe namespace is machine-global, so two sandboxes
/// would otherwise collide exactly as they did on Linux.
pub fn socket_name(id: SocketId) -> Option<interprocess::local_socket::Name<'static>> {
    #[cfg(unix)]
    {
        use interprocess::local_socket::{GenericFilePath, ToFsName};
        socket_path(id)?.to_fs_name::<GenericFilePath>().ok()
    }
    #[cfg(not(unix))]
    {
        use interprocess::local_socket::{GenericNamespaced, ToNsName};
        let ns = namespace().unwrap_or_else(|| "default".to_string());
        // No `.sock` suffix: this is a pipe, not a file, and naming it after a
        // filesystem object it is not would mislead anyone reading `\\.\pipe\`.
        format!("skribisto-{ns}-{}", id.leaf())
            .to_ns_name::<GenericNamespaced>()
            .ok()
    }
}

/// The spelling a claim records: the path the filesystem gives the project, as far
/// as the project exists.
///
/// A folder project's two spellings (`…/Novel`, `…/Novel/project.skrib`) are
/// collapsed first, as `shell::process::canon` does, or a second instance opens the
/// project a second time. Then the filesystem canonicalises the path or, when it
/// does not exist, its folder, and the name is appended as written. A target an
/// import or New Work is about to write does not exist yet, but its folder does
/// (every door that writes one checks it), so this spells it the way it will be
/// spelled once written: a claim made before the file appears and a check made
/// after it (or the reverse) agree, and a folder reached through a symbolic link, a
/// relative path or, on Windows, a short name such as `RUNNER~1` is the folder
/// itself. When neither resolves, such as on an unreachable network share, the path
/// is only made absolute, and unchanged if even that fails.
///
/// Compare two paths through [`same_project`], never by this alone: it keeps the
/// case and the separators the writer or the filesystem gave it.
pub(crate) fn canonical(path: &str) -> String {
    let path = skrib_format::canonical_project_path(path);
    resolved(Path::new(&path))
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or(path)
}

/// `path` made absolute and canonicalised by the filesystem, or when it does not
/// exist, its folder canonicalised and its name appended, or when that does not
/// either, only made absolute. `None` when even that fails (an empty path).
///
/// Only the folder is tried, not every ancestor up to the root: on an unreachable
/// network share each attempt can wait out a timeout, and a target is only ever
/// written into a folder that exists.
fn resolved(path: &Path) -> Option<PathBuf> {
    let absolute = std::path::absolute(path).ok()?;
    if let Ok(full) = std::fs::canonicalize(&absolute) {
        return Some(full);
    }
    let in_its_folder = absolute
        .file_name()
        .zip(absolute.parent())
        .and_then(|(name, folder)| {
            std::fs::canonicalize(folder)
                .ok()
                .map(|folder| folder.join(name))
        });
    Some(in_its_folder.unwrap_or(absolute))
}

/// What every spelling of one project's path has in common: [`canonical`], then
/// the separators, verbatim prefix, case and Unicode normalisation this platform's
/// filesystem does not tell apart (see [`spelling`]). The one thing a door
/// compares.
///
/// Never where a claim is stored: the key can be one for two files (see
/// [`CLAIMED`]).
pub(crate) fn project_key(path: &str) -> String {
    key_of(&canonical(path))
}

/// [`project_key`] of a path [`canonical`] has already spelled, so a caller that
/// needs both touches the filesystem once.
fn key_of(spelled: &str) -> String {
    spelling::spelling_key(spelled, path_style())
}

/// Whether `a` and `b` name the same project, whatever their spelling.
pub(crate) fn same_project(a: &str, b: &str) -> bool {
    project_key(a) == project_key(b)
}

/// The lock file naming `pid`'s claim on the project [`canonical`] spells
/// `spelled`. Named after the spelling, not the key, for the reason [`CLAIMED`] is
/// keyed by it: two files must never share a lock file. Pid is a parameter (not
/// always [`my_pid`]) so tests can fake a foreign process's lock without spawning
/// one.
fn lock_path_for(pid: u32, spelled: &str) -> Option<PathBuf> {
    let d = dir()?;
    let mut h = DefaultHasher::new();
    spelled.hash(&mut h);
    Some(d.join(format!("open-{pid}-{:016x}.lock", h.finish())))
}

/// The spelling this process claimed the project [`canonical`] spells `spelled`
/// under, if it holds it: that spelling itself, or one that resolves to it now.
///
/// A claim can be made before its file exists (New Work claims the project it is
/// about to write) and let go of once it does, and the filesystem may spell the
/// file then otherwise than the claim did: HFS+ stores a name decomposed. So a claim
/// is also found by resolving its own spelling again. Never by [`project_key`]
/// alone: two files can share a key, and letting go of the other one's claim is the
/// loss [`CLAIMED`] is keyed by spelling to prevent. Holding on to a claim too long
/// only has other copies see the project open until this one exits.
///
/// Only the claims sharing the key are resolved again, which in practice is none:
/// resolving every claim would have each release wait on the disk of every project
/// open, a network share that has gone away among them.
fn claimed_spelling(claimed: &HashMap<String, PathBuf>, spelled: &str) -> Option<String> {
    if claimed.contains_key(spelled) {
        return Some(spelled.to_string());
    }
    let key = key_of(spelled);
    claimed
        .keys()
        .filter(|held| key_of(held) == key)
        .find(|held| canonical(held) == spelled)
        .cloned()
}

/// Claim `path` as open by this process, in addition to any claims already
/// held — opening a second project does *not* drop the first. A window
/// replacing its own previous project in place (Load/New/Save-As/Restore, no
/// `CloseWork` in between) should [`release`] its own previous path first,
/// then call this — never [`release_all`], which drops every claim the whole
/// *process* holds, including a sibling window's untouched, still-open Work
/// (see `view_models::project_lifecycle::ProjectLifecycleViewModel::claim`'s doc).
pub fn claim(path: &str, title: &str) {
    let spelled = canonical(path);
    let Some(lock) = lock_path_for(my_pid(), &spelled) else {
        return;
    };
    let entry = OpenEntry {
        pid: my_pid(),
        path: spelled.clone(),
        title: title.to_string(),
        importing: false,
    };
    if let Ok(json) = serde_json::to_string(&entry)
        && std::fs::write(&lock, json).is_ok()
    {
        CLAIMED.with(|c| {
            c.borrow_mut().insert(spelled, lock);
        });
    }
}

/// Release this process's claim on `path`, if any. Leaves every other claim this
/// process holds untouched. Idempotent — safe to call on `CloseWork` and again
/// at shutdown, and it is the right way to drop a WINDOW's own previous claim
/// before it claims a new path in place (see [`claim`]'s doc).
pub fn release(path: &str) {
    let spelled = canonical(path);
    let lock = CLAIMED.with(|c| {
        let mut claimed = c.borrow_mut();
        let held = claimed_spelling(&claimed, &spelled)?;
        claimed.remove(&held)
    });
    if let Some(lock) = lock {
        remove_own_lock(&lock);
    }
}

/// Release every claim this process holds — process exit only. Never call this
/// to "swap the one open project": with two Works open in two windows of one
/// process, it would drop a sibling window's untouched, still-open Work's
/// claim too. Use [`release`] (this window's own previous path) + [`claim`]
/// (its new one) instead.
pub fn release_all() {
    let locks: Vec<PathBuf> = CLAIMED.with(|c| c.borrow_mut().drain().map(|(_, l)| l).collect());
    for lock in locks {
        remove_own_lock(&lock);
    }
}

/// Delete `lock` only if it still names *this* process's pid. Defends against
/// deleting a peer's claim: even though the pid is now baked into the file name
/// so two processes can never share one file, this guard keeps `release`/
/// `release_all` safe against any future path that hands them a lock path they
/// didn't mint themselves.
fn remove_own_lock(lock: &Path) {
    if let Ok(text) = std::fs::read_to_string(lock)
        && let Ok(entry) = serde_json::from_str::<OpenEntry>(&text)
        && entry.pid == my_pid()
    {
        let _ = std::fs::remove_file(lock);
    }
}

/// An import writing the project at a path, for as long as this lives.
///
/// # The race this closes
///
/// An import checks that its target is not open when Import (or the overwrite
/// question's OK) is pressed, then converts for as long as the project takes, then
/// replaces the file. A window that opened the old project in between would keep it
/// in memory, and its next save would write it straight back over the import. With
/// the target claimed for the whole of that time, the load doors
/// ([`importing`]'s callers) refuse to open it, and New Work and the other importers
/// refuse to write it, here and in every other running copy of Skribisto.
///
/// A lock file like an open project's, marked [`OpenEntry::importing`], so another
/// instance sees the claim; released, file and all, when the claim is dropped, which
/// an import does as it completes, fails or is cancelled.
#[must_use = "the claim lasts only as long as it is held"]
#[derive(Debug)]
pub struct ImportClaim {
    key: String,
    lock: Option<PathBuf>,
}

impl Drop for ImportClaim {
    fn drop(&mut self) {
        IMPORTING.with(|importing| {
            let mut importing = importing.borrow_mut();
            if let Some(count) = importing.get_mut(&self.key) {
                *count = count.saturating_sub(1);
                if *count == 0 {
                    importing.remove(&self.key);
                }
            }
        });
        if let Some(lock) = &self.lock {
            remove_own_lock(lock);
        }
    }
}

/// The lock file naming `pid`'s import claim on the project [`canonical`] spells
/// `spelled`: beside its open claims, under another name, so a claim of either kind
/// never overwrites the other. Named after the spelling, as [`lock_path_for`] is:
/// two imports writing two files whose names differ only in case, on a
/// case-sensitive volume, must not share one lock file, or the first to finish
/// deletes the other's.
fn import_lock_path_for(pid: u32, spelled: &str) -> Option<PathBuf> {
    let d = dir()?;
    let mut h = DefaultHasher::new();
    spelled.hash(&mut h);
    Some(d.join(format!("import-{pid}-{:016x}.lock", h.finish())))
}

/// Claim `path` for an import writing it. See [`ImportClaim`].
pub fn claim_import(path: &str) -> ImportClaim {
    let spelled = canonical(path);
    let key = key_of(&spelled);
    IMPORTING.with(|importing| *importing.borrow_mut().entry(key.clone()).or_insert(0) += 1);
    let lock = import_lock_path_for(my_pid(), &spelled);
    let entry = OpenEntry {
        pid: my_pid(),
        path: spelled,
        title: String::new(),
        importing: true,
    };
    let lock = lock.filter(|lock| {
        serde_json::to_string(&entry).is_ok_and(|json| std::fs::write(lock, json).is_ok())
    });
    ImportClaim { key, lock }
}

/// A claim on a project a window is about to load, made before the load starts and
/// kept once it has succeeded: see [`claim_for_load`].
#[must_use = "the claim is released when this is dropped, unless it is kept"]
#[derive(Debug)]
pub struct LoadClaim {
    /// The path claimed, when this made the claim and has to let go of it.
    made: Option<String>,
}

impl LoadClaim {
    /// The load succeeded: the claim stays, as the claim of the project now open, and
    /// is released with it (on `CloseWork`, or at exit).
    pub fn keep(mut self) {
        self.made = None;
    }
}

impl Drop for LoadClaim {
    fn drop(&mut self) {
        if let Some(path) = self.made.take() {
            release(&path);
        }
    }
}

/// Claim `path` as open before loading it, or `None` when an import is writing it.
///
/// # Claim, then check
///
/// A load takes seconds (twenty for a large project in a debug build), and a window
/// claimed its project only once the load was done. An import started in another
/// copy of Skribisto in the meantime saw no claim, went ahead, and replaced the file
/// under a window that went on to write the old project back over it. So a load
/// claims first and then looks for an import, and an import claims first and then
/// looks for a project open or being opened (`shared::import_destination`). Whichever
/// of the two looks second sees the other's claim and backs out: both are never
/// missed.
///
/// The claim is titled after the file until the load names the project. When this
/// process already holds `path` open, nothing new is claimed and nothing is released.
pub fn claim_for_load(path: &str) -> Option<LoadClaim> {
    if importing(path) {
        return None;
    }
    let spelled = canonical(path);
    let held = CLAIMED.with(|claimed| claimed_spelling(&claimed.borrow(), &spelled).is_some());
    let claim = if held {
        LoadClaim { made: None }
    } else {
        let title = Path::new(path)
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
            .unwrap_or_default();
        claim(path, &title);
        LoadClaim {
            made: Some(path.to_string()),
        }
    };
    // Dropped on the way out when an import claimed the path meanwhile.
    (!importing(path)).then_some(claim)
}

/// Whether an import, in this instance or another, is writing the project at `path`.
///
/// Reads the lock directory, so it runs when a project is about to be opened or
/// written, never in a derived signal.
pub fn importing(path: &str) -> bool {
    let key = project_key(path);
    IMPORTING.with(|importing| importing.borrow().contains_key(&key))
        || scan()
            .into_iter()
            .any(|entry| entry.importing && project_key(&entry.path) == key)
}

/// [`scan`], without the projects an import is writing: every project open in a
/// window of some instance.
pub fn scan_open() -> Vec<OpenEntry> {
    scan()
        .into_iter()
        .filter(|entry| !entry.importing)
        .collect()
}

/// Every project currently open across live instances (undeduped — the same
/// path may legitimately appear twice if two processes both hold it), reaping
/// any lock or IPC socket whose owning process is gone. Includes this
/// instance's own project(s) (compare `entry.pid == my_pid()`).
pub fn scan() -> Vec<OpenEntry> {
    let mut out = Vec::new();
    let Some(d) = dir() else {
        return out;
    };
    let Ok(rd) = std::fs::read_dir(&d) else {
        return out;
    };
    for ent in rd.flatten() {
        let p = ent.path();
        match p.extension().and_then(|e| e.to_str()) {
            Some("lock") => {
                let Ok(text) = std::fs::read_to_string(&p) else {
                    continue;
                };
                match serde_json::from_str::<OpenEntry>(&text) {
                    Ok(entry) if pid_alive(entry.pid) => out.push(entry),
                    // Unparsable or dead-owner ⇒ stale; reap it.
                    _ => {
                        let _ = std::fs::remove_file(&p);
                    }
                }
            }
            Some("sock") => {
                // `ipc-{pid}.sock`: unlike locks these carry no JSON payload, so
                // the pid comes from the file name itself.
                if let Some(pid) = socket_pid(&p)
                    && !pid_alive(pid)
                {
                    let _ = std::fs::remove_file(&p);
                }
            }
            _ => {}
        }
    }
    out
}

/// Parse the pid out of an `ipc-{pid}.sock` file name.
fn socket_pid(p: &Path) -> Option<u32> {
    let stem = p.file_stem()?.to_str()?;
    stem.strip_prefix("ipc-")?.parse().ok()
}

/// [`pid_is_alive`], but test-overridable so tests can fake a foreign pid as
/// alive or dead without depending on real process state.
fn pid_alive(pid: u32) -> bool {
    #[cfg(test)]
    {
        if let Some(v) = PID_OVERRIDE.with(|m| m.borrow().get(&pid).copied()) {
            return v;
        }
    }
    pid_is_alive(pid)
}

/// Is `pid` a live process? Used only to reap stale locks/sockets — a false
/// "alive" (PID reuse) merely keeps a stale entry one scan longer, never a
/// correctness issue.
#[cfg(unix)]
fn pid_is_alive(pid: u32) -> bool {
    // Signal 0 sends nothing; it just does the existence/permission check.
    unsafe { libc::kill(pid as libc::pid_t, 0) == 0 }
}

#[cfg(windows)]
fn pid_is_alive(pid: u32) -> bool {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Threading::{
        GetExitCodeProcess, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
    };
    const STILL_ACTIVE: u32 = 259;
    unsafe {
        let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) else {
            return false;
        };
        let mut code = 0u32;
        let alive = GetExitCodeProcess(handle, &mut code).is_ok() && code == STILL_ACTIVE;
        let _ = CloseHandle(handle);
        alive
    }
}

#[cfg(not(any(unix, windows)))]
fn pid_is_alive(_pid: u32) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    /// Fresh temp dir + reset thread-local state for one test. Each `#[test]`
    /// runs on its own thread under the default harness, so the `thread_local`s
    /// are naturally isolated, but we clear them anyway to be independent of
    /// harness threading details.
    fn setup(tag: &str) -> PathBuf {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let d = std::env::temp_dir().join(format!(
            "skribisto-open-registry-test-{tag}-{}-{n}",
            std::process::id()
        ));
        std::fs::create_dir_all(&d).unwrap();
        set_dir_override(Some(d.clone()));
        CLAIMED.with(|c| c.borrow_mut().clear());
        IMPORTING.with(|c| c.borrow_mut().clear());
        PID_OVERRIDE.with(|m| m.borrow_mut().clear());
        STYLE_OVERRIDE.with(|s| s.set(None));
        d
    }

    /// Write a lock file as if `pid` (not this process) claimed `path`.
    fn write_foreign_lock(pid: u32, path: &str, title: &str) -> PathBuf {
        let lock = lock_path_for(pid, &canonical(path)).expect("dir available");
        let entry = OpenEntry {
            pid,
            path: canonical(path),
            title: title.to_string(),
            importing: false,
        };
        std::fs::write(&lock, serde_json::to_string(&entry).unwrap()).unwrap();
        lock
    }

    /// An import claim is seen by this instance and by any other through its lock
    /// file, is not counted as a project open in a window, and goes, file and all,
    /// when it is dropped.
    #[test]
    fn an_import_claim_lasts_as_long_as_it_is_held() {
        setup("import-claim");
        let path = "/tmp/skribisto-registry-test-import.skrib";
        assert!(!importing(path));
        let claim = claim_import(path);
        assert!(importing(path));
        let seen = scan();
        assert_eq!(seen.len(), 1);
        assert!(
            seen[0].importing,
            "another instance reads the lock as an import"
        );
        assert!(scan_open().is_empty(), "no window holds it");
        drop(claim);
        assert!(!importing(path));
        assert!(scan().is_empty(), "the lock file went with it");
    }

    /// A load claims its project before it starts, so an import started while it runs
    /// sees a project open there. The claim goes when the load fails, and stays, as the
    /// open project's own claim, once it has succeeded.
    #[test]
    fn a_load_claims_its_project_before_it_starts() {
        setup("load-claim");
        let path = "/tmp/skribisto-registry-test-load.skrib";
        let Some(failed) = claim_for_load(path) else {
            panic!("nothing stands in the way");
        };
        let seen = scan_open();
        assert_eq!(seen.len(), 1, "seen as open while it loads");
        assert_eq!(seen[0].title, "skribisto-registry-test-load");
        drop(failed);
        assert!(scan_open().is_empty(), "a failed load lets go of it");

        let Some(loaded) = claim_for_load(path) else {
            panic!("nothing stands in the way");
        };
        loaded.keep();
        assert_eq!(scan_open().len(), 1, "a load that succeeded keeps it");
        release(path);
        assert!(scan_open().is_empty());
    }

    /// A load of a project an import holds, here or in another copy, is refused and
    /// leaves no claim behind; so is one whose claim an import's lands beside before it
    /// looks, which the second look finds.
    #[test]
    fn a_load_of_a_project_an_import_holds_is_refused_and_claims_nothing() {
        let dir = setup("load-while-importing");
        let path = "/tmp/skribisto-registry-test-load-importing.skrib";
        let import = claim_import(path);
        assert!(claim_for_load(path).is_none());
        assert!(scan_open().is_empty(), "no claim is left behind");
        drop(import);

        let fake_pid = 999_203;
        PID_OVERRIDE.with(|m| m.borrow_mut().insert(fake_pid, true));
        let entry = OpenEntry {
            pid: fake_pid,
            path: canonical(path),
            title: String::new(),
            importing: true,
        };
        std::fs::write(
            dir.join(format!("import-{fake_pid}-0.lock")),
            serde_json::to_string(&entry).unwrap(),
        )
        .unwrap();
        assert!(claim_for_load(path).is_none(), "a peer's import too");
        assert!(scan_open().is_empty());
        PID_OVERRIDE.with(|m| m.borrow_mut().insert(fake_pid, false));
    }

    /// A project this process already holds open is not claimed again, and a load of it
    /// that fails does not let go of the claim the open window holds.
    #[test]
    fn a_load_of_a_project_already_open_here_keeps_its_claim() {
        setup("load-held");
        let path = "/tmp/skribisto-registry-test-load-held.skrib";
        claim(path, "Held");
        let Some(failed) = claim_for_load(path) else {
            panic!("nothing stands in the way");
        };
        drop(failed);
        let seen = scan_open();
        assert_eq!(seen.len(), 1, "the open window's claim stays");
        assert_eq!(seen[0].title, "Held");
        release(path);
    }

    /// A hold on a target not written yet blocks every spelling of it. The filesystem
    /// cannot canonicalise a file that is not there, so each of these used to compare
    /// as another string, and a load, a New Work or a Save As into the file the import
    /// was about to replace went ahead: a separator doubled, a `.` component, a
    /// trailing separator, and the folder reached through a symbolic link, either way
    /// round.
    #[test]
    fn a_hold_blocks_every_spelling_of_a_target_not_written_yet() {
        setup("import-spellings");
        let dir = tempfile::tempdir().unwrap();
        let books = dir.path().join("Books");
        std::fs::create_dir(&books).unwrap();
        let target = books.join("Novel.skrib").to_string_lossy().into_owned();
        let folder = books.to_string_lossy().into_owned();
        let sep = std::path::MAIN_SEPARATOR;
        let hold = claim_import(&target);
        for spelling in [
            format!("{folder}{sep}{sep}Novel.skrib"),
            format!("{folder}{sep}.{sep}Novel.skrib"),
            format!("{target}{sep}"),
        ] {
            assert!(importing(&spelling), "{spelling}");
            assert!(claim_for_load(&spelling).is_none(), "{spelling}");
        }
        assert!(!importing(&books.join("Other.skrib").to_string_lossy()));
        drop(hold);
        #[cfg(unix)]
        {
            let shelf = dir.path().join("Shelf");
            std::os::unix::fs::symlink(&books, &shelf).unwrap();
            let through_link = shelf.join("Novel.skrib").to_string_lossy().into_owned();
            let hold = claim_import(&target);
            assert!(importing(&through_link), "the link names the same folder");
            drop(hold);
            let hold = claim_import(&through_link);
            assert!(importing(&target), "and the other way round");
            drop(hold);
        }
        assert!(scan().is_empty(), "every hold went with its lock file");
    }

    /// A hold made under one Windows spelling of a target blocks the others: the other
    /// separator (the import forms built `C:\Books/Novel.skrib` where a file dialog
    /// says `C:\Books\Novel.skrib`), another case, which NTFS does not tell apart, and
    /// a trailing separator; in this instance, and in another one whose lock file
    /// carries the spelling an earlier build wrote. Proved in Windows' rules whatever
    /// the platform running the test.
    #[test]
    fn a_hold_blocks_the_windows_spellings_of_its_target() {
        let dir = setup("import-windows-spellings");
        let _style = crate::test_support::ForeignPathStyle::new(PathStyle::Windows);
        let books = tempfile::tempdir().unwrap();
        let folder = books.path().to_string_lossy().into_owned();
        let hold = claim_import(&format!("{folder}/Second.skrib"));
        for spelling in [
            format!("{folder}\\Second.skrib"),
            format!("{folder}\\second.skrib"),
            format!("{folder}/SECOND.SKRIB"),
            format!("{folder}\\\\Second.skrib\\"),
        ] {
            assert!(importing(&spelling), "{spelling}");
        }
        assert!(!importing(&format!("{folder}\\Other.skrib")));
        drop(hold);

        let fake_pid = 999_204;
        PID_OVERRIDE.with(|m| m.borrow_mut().insert(fake_pid, true));
        let entry = OpenEntry {
            pid: fake_pid,
            path: format!("{folder}/Third.skrib"),
            title: String::new(),
            importing: true,
        };
        std::fs::write(
            dir.join(format!("import-{fake_pid}-0.lock")),
            serde_json::to_string(&entry).unwrap(),
        )
        .unwrap();
        assert!(importing(&format!("{folder}\\third.skrib")), "a peer's too");
        PID_OVERRIDE.with(|m| m.borrow_mut().insert(fake_pid, false));
    }

    /// A volume formatted case-sensitive (APFS offers it, Windows sets it per folder)
    /// holds two projects whose names differ only in case. Their keys are one, since
    /// the key follows the platform's default, but they are two files, each with its
    /// own claim: closing one, or finishing the import writing one, leaves the other
    /// advertised to every other copy of Skribisto. Proved in macOS' rules on this
    /// case-sensitive test filesystem.
    #[test]
    fn two_projects_whose_names_differ_only_in_case_keep_two_claims() {
        setup("case-sensitive-volume");
        let _style = crate::test_support::ForeignPathStyle::new(PathStyle::Mac);
        let books = tempfile::tempdir().unwrap();
        let upper = books.path().join("Novel.skrib");
        let lower = books.path().join("novel.skrib");
        std::fs::write(&upper, b"PK").unwrap();
        if lower.exists() {
            // A volume that ignores case, the default on macOS and Windows, holds one
            // file under both names, so the two projects this proves cannot be made.
            return;
        }
        std::fs::write(&lower, b"PK").unwrap();
        let (upper, lower) = (
            upper.to_string_lossy().into_owned(),
            lower.to_string_lossy().into_owned(),
        );

        claim(&upper, "Upper");
        claim(&lower, "Lower");
        release(&lower);
        // Released twice, as every window on a Work releases it when it closes.
        release(&lower);
        let titles: Vec<String> = scan_open().into_iter().map(|e| e.title).collect();
        assert_eq!(
            titles,
            vec!["Upper".to_string()],
            "closing one keeps the other"
        );
        release(&upper);
        assert!(scan_open().is_empty());

        let upper_import = claim_import(&upper);
        let lower_import = claim_import(&lower);
        drop(lower_import);
        let held: Vec<String> = scan().into_iter().map(|e| e.path).collect();
        assert_eq!(
            held,
            vec![canonical(&upper)],
            "the import still running is seen"
        );
        drop(upper_import);
        assert!(scan().is_empty());
    }

    /// A claim is let go of once the filesystem spells its project otherwise than it
    /// did when the claim was made. New Work claims the file it is about to write,
    /// and HFS+ then stores its name decomposed, so the path the window releases
    /// resolves to a spelling the claim never had. Here, in macOS' rules, the name
    /// claimed precomposed comes to be a symbolic link to the decomposed one: the
    /// same change of spelling on this filesystem.
    #[cfg(unix)]
    #[test]
    fn a_claim_is_let_go_of_once_the_filesystem_spells_it_otherwise() {
        setup("release-respelled");
        let _style = crate::test_support::ForeignPathStyle::new(PathStyle::Mac);
        let books = tempfile::tempdir().unwrap();
        let typed = books.path().join("Rapha\u{eb}l.skrib");
        let stored = books.path().join("Raphae\u{308}l.skrib");
        let path = typed.to_string_lossy().into_owned();
        claim(&path, "Novel");
        assert_eq!(scan_open().len(), 1);
        std::fs::write(&stored, b"PK").unwrap();
        if typed.exists() {
            // APFS and HFS+ look both spellings up as one file, so the respelling this
            // proves cannot be staged with a link there; the filesystem does it itself.
            return;
        }
        std::os::unix::fs::symlink(&stored, &typed).unwrap();
        assert_ne!(
            canonical(&path),
            path,
            "the filesystem spells it otherwise now"
        );
        release(&path);
        assert!(scan_open().is_empty(), "the claim went with its release");
        assert!(CLAIMED.with(|c| c.borrow().is_empty()));
    }

    /// Another instance's import is refused here too, through its lock file alone.
    #[test]
    fn a_peers_import_claim_is_seen_here() {
        let dir = setup("peer-import");
        let path = "/tmp/skribisto-registry-test-peer-import.skrib";
        let fake_pid = 999_202;
        PID_OVERRIDE.with(|m| m.borrow_mut().insert(fake_pid, true));
        let entry = OpenEntry {
            pid: fake_pid,
            path: canonical(path),
            title: String::new(),
            importing: true,
        };
        std::fs::write(
            dir.join(format!("import-{fake_pid}-0.lock")),
            serde_json::to_string(&entry).unwrap(),
        )
        .unwrap();
        assert!(importing(path));
        PID_OVERRIDE.with(|m| m.borrow_mut().insert(fake_pid, false));
        assert!(!importing(path), "a dead peer's claim is reaped");
    }

    /// A lock file written before the field existed reads as a project open in a
    /// window, as it was.
    #[test]
    fn a_lock_without_the_import_mark_is_an_open_project() {
        let entry: OpenEntry =
            serde_json::from_str(r#"{"pid":1,"path":"/a.skrib","title":"A"}"#).unwrap();
        assert!(!entry.importing);
        let written = serde_json::to_string(&OpenEntry {
            pid: 1,
            path: "/a.skrib".into(),
            title: "A".into(),
            importing: false,
        })
        .unwrap();
        assert!(!written.contains("importing"), "{written}");
    }

    /// **The macOS budget.** There is no `XDG_RUNTIME_DIR` on macOS, so the
    /// socket lives under `~/Library/Application Support/…`, and a Unix domain
    /// socket's `sun_path` holds only **104 bytes** on Darwin. An earlier
    /// revision namespaced that branch too, for the tidiness of one naming rule,
    /// and every username went over — `bind()` failed with `ENAMETOOLONG` and
    /// single-instance silently never engaged on macOS at all.
    ///
    /// Computed rather than measured: this suite does not run on Darwin, and the
    /// failure it guards is a silent degradation to `Standalone`, not a crash
    /// anyone would notice. `etcetera`'s Apple strategy puts the data dir at
    /// `~/Library/Application Support/{tld}.{author}.{app}`.
    #[test]
    fn the_macos_socket_path_fits_in_sun_path() {
        // `SocketId::leaf` reads the process-wide identity static; see
        // `identity::lock_for_test`'s own doc for why every test that reads it,
        // not only the ones that register a different one, needs this lock.
        let _serial = crate::identity::lock_for_test();
        const DARWIN_SUN_PATH: usize = 104;
        // Generous: longer than almost any real macOS short name.
        let long_user = "jean-baptiste-de-la";
        // The directory is the **family** one whatever edition runs (see
        // `namespace`), so an edition with a longer application name does not
        // lengthen this path — only its socket leaf.
        for user in ["bo", "cyril", long_user] {
            let dir =
                format!("/Users/{user}/Library/Application Support/eu.skribisto.Skribisto/run");
            for leaf in [
                SocketId::Primary.leaf(),
                SocketId::Pid(4_294_967_295).leaf(),
                // The worst case an extension edition can produce. Computed the
                // same way `SocketId::leaf` computes it, rather than hardcoded,
                // so shortening or lengthening the slug moves this budget with it.
                format!(
                    "primary-{}",
                    crate::identity::AppIdentity::new("eu", "skribisto-pro", "Skribisto Pro")
                        .slug()
                ),
            ] {
                let path = format!("{dir}/{leaf}.sock");
                assert!(
                    path.len() < DARWIN_SUN_PATH,
                    "{path} is {} bytes; Darwin's sun_path holds {DARWIN_SUN_PATH} \
                     including the NUL, so bind() would fail with ENAMETOOLONG — which does \
                     not crash, it degrades silently to Standalone and single-instance never \
                     engages. This is why the edition suffix is a six-hex slug and not the \
                     readable organization name.",
                    path.len()
                );
            }
        }
    }

    /// Two editions must **elect separately** — this is the half of the design
    /// that stops an extension build handing its project to a community primary
    /// and exiting.
    #[test]
    fn editions_elect_on_different_primary_sockets() {
        // `crate::identity::register` mutates the process-wide identity static;
        // this lock is what keeps that mutation from racing every other test
        // that reads or writes it, in this module and every other one. See
        // `identity::lock_for_test`'s own doc for the intermittent-failure
        // history this exists to prevent.
        let _serial = crate::identity::lock_for_test();
        let community = SocketId::Primary.leaf();
        let _h = crate::identity::register(crate::identity::AppIdentity::new(
            "eu",
            "skribisto-pro",
            "Skribisto Pro",
        ));
        let edition = SocketId::Primary.leaf();

        assert_ne!(
            community, edition,
            "an edition sharing the community's primary socket shares its election, and hands \
             every project it is launched with to a window that has none of the extension in it"
        );
        assert_eq!(
            community, "primary",
            "the community edition must keep the bare name it has always used, so an upgrade \
             does not briefly run two primaries"
        );
    }

    /// …and must **share a lock directory**, which is the other half: it is what
    /// lets `check_open_elsewhere` see that another edition holds a project open
    /// before a backup restore overwrites it.
    #[test]
    fn editions_share_one_lock_directory() {
        let _serial = crate::identity::lock_for_test();
        let community = namespace();
        let _h = crate::identity::register(crate::identity::AppIdentity::new(
            "eu",
            "skribisto-pro",
            "Skribisto Pro",
        ));
        assert_eq!(
            community,
            namespace(),
            "the lock directory must not follow the edition, or one edition can restore a \
             backup over a project another has open and never know"
        );
    }

    /// The per-pid socket is what a cross-edition raise travels over, so it must
    /// stay edition-independent: the peer whose window we want to raise is
    /// identified by pid alone.
    #[test]
    fn a_peer_socket_is_addressed_by_pid_alone() {
        let _serial = crate::identity::lock_for_test();
        let before = SocketId::Pid(4242).leaf();
        let _h = crate::identity::register(crate::identity::AppIdentity::new(
            "eu",
            "skribisto-pro",
            "Skribisto Pro",
        ));
        assert_eq!(before, SocketId::Pid(4242).leaf());
    }

    /// The suffix belongs to the `XDG_RUNTIME_DIR` branch alone — that directory
    /// is per-login-session and shared by every installation on the machine. The
    /// data-dir fallback is already inside this installation's own tree, and
    /// every byte spent there comes out of the macOS budget above.
    #[test]
    fn only_the_shared_session_dir_pays_for_a_namespace() {
        let ns = namespace_for(Path::new("/home/writer/.config/Skribisto"));
        let session = PathBuf::from("/run/user/1000").join(format!("skribisto-{ns}"));
        let fallback = PathBuf::from("/home/writer/.local/share/Skribisto").join("run");

        assert!(
            session.to_string_lossy().contains(&ns),
            "the shared session dir must be namespaced"
        );
        assert!(
            !fallback.to_string_lossy().contains(&ns),
            "the per-installation fallback must not spend bytes on a suffix it does not need"
        );
    }

    /// Socket leaves must stay short and free of path separators: on Unix they
    /// are a file name inside `dir()`, on Windows they are spliced into a
    /// `\\.\pipe\` name, and neither tolerates a `/`.
    #[test]
    fn socket_leaves_are_short_and_flat() {
        let _serial = crate::identity::lock_for_test();
        for leaf in [
            SocketId::Primary.leaf(),
            SocketId::Pid(4_294_967_295).leaf(),
        ] {
            assert!(
                !leaf.contains('/') && !leaf.contains('\\'),
                "{leaf} has a separator"
            );
            assert!(leaf.len() <= 16, "{leaf} is {} bytes", leaf.len());
        }
    }

    #[test]
    fn two_config_dirs_get_two_namespaces() {
        let a = namespace_for(Path::new("/home/writer/.config/Skribisto"));
        let b = namespace_for(Path::new("/tmp/skribisto_sandbox_1/config/Skribisto"));
        assert_ne!(
            a, b,
            "a sandboxed run must not share the real installation's instance universe"
        );
    }

    #[test]
    fn the_same_config_dir_always_gets_the_same_namespace() {
        // The suffix keys lock files, IPC sockets and the primary election
        // (`shell::instance`). An unstable answer would strand every one of
        // them in a directory nothing looks at any more.
        let p = Path::new("/home/writer/.config/Skribisto");
        assert_eq!(namespace_for(p), namespace_for(p));
        assert_eq!(namespace_for(p), namespace_for(&PathBuf::from(p)));
    }

    #[test]
    fn the_namespace_is_a_fixed_width_hex_suffix() {
        let ns = namespace_for(Path::new("/home/writer/.config/Skribisto"));
        assert_eq!(ns.len(), 16, "matches the `work-{{:016x}}` id width");
        assert!(ns.chars().all(|c| c.is_ascii_hexdigit()));
    }

    /// The namespace's whole point: a claim minted under one installation
    /// identity must be invisible to another. Exercised through `scan` with two
    /// distinct directory overrides, which is what the suffix resolves to.
    #[test]
    fn a_claim_in_one_namespace_is_invisible_in_another() {
        let ns_a = setup("namespace-a");
        let path = "/tmp/skribisto-registry-test-project-ns.skrib";
        claim(path, "Mine");
        assert_eq!(scan().len(), 1, "visible in its own namespace");

        // A different namespace = a different directory, exactly as the
        // blake3 suffix produces for a different config dir.
        let ns_b = std::env::temp_dir().join(format!(
            "skribisto-open-registry-test-namespace-b-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&ns_b).unwrap();
        set_dir_override(Some(ns_b.clone()));
        assert!(
            scan().is_empty(),
            "a peer namespace must not see this installation's claims"
        );

        set_dir_override(Some(ns_a));
        release_all();
        let _ = std::fs::remove_dir_all(&ns_b);
    }

    #[test]
    fn two_different_pids_claiming_same_path_both_appear() {
        setup("two-pids-same-path");
        let path = "/tmp/skribisto-registry-test-project-a.skrib";
        let fake_pid = 999_101;
        PID_OVERRIDE.with(|m| m.borrow_mut().insert(fake_pid, true));
        write_foreign_lock(fake_pid, path, "Peer's copy");

        claim(path, "My copy");

        let matching: Vec<_> = scan()
            .into_iter()
            .filter(|e| e.path == canonical(path))
            .collect();
        assert_eq!(
            matching.len(),
            2,
            "both this process's and the peer's claim on the same path must be visible"
        );
        assert!(matching.iter().any(|e| e.pid == fake_pid));
        assert!(matching.iter().any(|e| e.pid == my_pid()));

        release_all();
    }

    #[test]
    fn one_pid_claiming_two_paths_yields_two_entries() {
        setup("one-pid-two-paths");
        let a = "/tmp/skribisto-registry-test-project-b.skrib";
        let b = "/tmp/skribisto-registry-test-project-c.skrib";

        claim(a, "B");
        claim(b, "C");

        let entries = scan();
        assert_eq!(entries.len(), 2);
        assert!(entries.iter().any(|e| e.path == canonical(a)));
        assert!(entries.iter().any(|e| e.path == canonical(b)));
        assert!(entries.iter().all(|e| e.pid == my_pid()));

        release_all();
    }

    #[test]
    fn release_drops_only_that_path_leaving_others_intact() {
        setup("release-one-of-many");
        let a = "/tmp/skribisto-registry-test-project-d.skrib";
        let b = "/tmp/skribisto-registry-test-project-e.skrib";
        claim(a, "D");
        claim(b, "E");

        release(a);

        let entries = scan();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, canonical(b));

        release_all();
    }

    #[test]
    fn release_never_deletes_a_lock_owned_by_another_pid() {
        setup("release-foreign-guard");
        let path = "/tmp/skribisto-registry-test-project-f.skrib";
        let fake_pid = 999_102;
        let lock = write_foreign_lock(fake_pid, path, "Foreign");

        // Exercise the exact defensive guard `release`/`release_all` rely on.
        remove_own_lock(&lock);

        assert!(
            lock.exists(),
            "a lock naming another pid must never be deleted"
        );
        let _ = std::fs::remove_file(&lock);
    }

    #[test]
    fn dead_pids_stale_socket_is_reaped_by_scan() {
        let d = setup("socket-reap");
        let dead_pid = 999_103;
        PID_OVERRIDE.with(|m| m.borrow_mut().insert(dead_pid, false));
        let sock = d.join(format!("ipc-{dead_pid}.sock"));
        std::fs::write(&sock, b"").unwrap();

        let _ = scan();

        assert!(
            !sock.exists(),
            "a dead pid's stale ipc socket must be reaped by scan()"
        );
    }

    #[test]
    fn live_pids_socket_is_left_alone() {
        let d = setup("socket-alive");
        let alive_pid = 999_104;
        PID_OVERRIDE.with(|m| m.borrow_mut().insert(alive_pid, true));
        let sock = d.join(format!("ipc-{alive_pid}.sock"));
        std::fs::write(&sock, b"").unwrap();

        let _ = scan();

        assert!(
            sock.exists(),
            "a live pid's socket must not be touched by scan()"
        );
        let _ = std::fs::remove_file(&sock);
    }
}
