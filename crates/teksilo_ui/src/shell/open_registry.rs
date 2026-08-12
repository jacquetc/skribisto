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

/// One open project, as advertised by its owning instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenEntry {
    pub pid: u32,
    /// Canonicalized absolute path of the `.skrib`.
    pub path: String,
    pub title: String,
}

thread_local! {
    /// This process's claims: canonical path -> the lock file that represents it.
    /// A map (not a single slot) because one process may hold several projects
    /// open at once.
    static CLAIMED: RefCell<HashMap<String, PathBuf>> = RefCell::new(HashMap::new());

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
/// only).
#[cfg(test)]
fn set_dir_override(path: Option<PathBuf>) {
    DIR_OVERRIDE.with(|d| *d.borrow_mut() = path);
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

pub(crate) fn canonical(path: &str) -> String {
    std::fs::canonicalize(path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string())
}

/// The lock file naming `pid`'s claim on `project_path`. Pid is a parameter (not
/// always [`my_pid`]) so tests can fake a foreign process's lock without
/// spawning one.
fn lock_path_for(pid: u32, project_path: &str) -> Option<PathBuf> {
    let d = dir()?;
    let mut h = DefaultHasher::new();
    canonical(project_path).hash(&mut h);
    Some(d.join(format!("open-{pid}-{:016x}.lock", h.finish())))
}

/// Claim `path` as open by this process, in addition to any claims already
/// held — opening a second project does *not* drop the first. A window
/// replacing its own previous project in place (Load/New/Save-As/Restore, no
/// `CloseWork` in between) should [`release`] its own previous path first,
/// then call this — never [`release_all`], which drops every claim the whole
/// *process* holds, including a sibling window's untouched, still-open Work
/// (see `view_models::project_lifecycle::ProjectLifecycleViewModel::claim`'s doc).
pub fn claim(path: &str, title: &str) {
    let canon = canonical(path);
    let Some(lock) = lock_path_for(my_pid(), path) else {
        return;
    };
    let entry = OpenEntry {
        pid: my_pid(),
        path: canon.clone(),
        title: title.to_string(),
    };
    if let Ok(json) = serde_json::to_string(&entry)
        && std::fs::write(&lock, json).is_ok()
    {
        CLAIMED.with(|c| {
            c.borrow_mut().insert(canon, lock);
        });
    }
}

/// Release this process's claim on `path`, if any. Leaves every other claim this
/// process holds untouched. Idempotent — safe to call on `CloseWork` and again
/// at shutdown, and it is the right way to drop a WINDOW's own previous claim
/// before it claims a new path in place (see [`claim`]'s doc).
pub fn release(path: &str) {
    let canon = canonical(path);
    let lock = CLAIMED.with(|c| c.borrow_mut().remove(&canon));
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
        PID_OVERRIDE.with(|m| m.borrow_mut().clear());
        d
    }

    /// Write a lock file as if `pid` (not this process) claimed `path`.
    fn write_foreign_lock(pid: u32, path: &str, title: &str) -> PathBuf {
        let lock = lock_path_for(pid, path).expect("dir available");
        let entry = OpenEntry {
            pid,
            path: canonical(path),
            title: title.to_string(),
        };
        std::fs::write(&lock, serde_json::to_string(&entry).unwrap()).unwrap();
        lock
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
