// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Cross-instance "open project" registry.
//!
//! Skribisto is multi-process today (each running instance is its own window),
//! and moving toward one instance holding several projects at once (the
//! `System.work_info` -> `work_infos` fan-out already landed). This module does
//! **not** assume "at most one project per process": a process may hold any
//! number of claims at once, and two different processes may legitimately claim
//! the *same* path for a moment (e.g. one closing while another opens it) — both
//! must be visible to [`scan`] so callers like `BackupRestoreViewModel::check_open_elsewhere`
//! can tell a peer still has the project open before overwriting it.
//!
//! While a project is open, the claiming process writes a small lock file naming
//! that project + its own pid into a shared runtime directory. The file name
//! embeds **both** the pid and a hash of the canonical path
//! (`open-{pid}-{hash}.lock`), so two processes claiming the same path get two
//! distinct files — one can never clobber or delete the other's claim. Other
//! instances [`scan`] these lock files to show which projects are open elsewhere
//! (the ProjectSwitcher "Currently open" section) and to reach the owning
//! process — its IPC socket path is derived from the pid
//! ([`ipc_socket_for_pid`]) — to ask it to raise its window.
//!
//! Crash-safe by construction: a lock whose pid is no longer alive is reaped on
//! [`scan`]; the mere presence of a file never means "open". [`scan`] also reaps
//! this instance's IPC socket files (`ipc-{pid}.sock`) once their owning pid is
//! dead — sockets are not otherwise cleaned up by anything else, so on platforms
//! without an ephemeral runtime dir (the macOS/Windows fallback below) they would
//! otherwise accumulate without bound. These are OS-level functions, independent
//! of the (mock or real) backend, so they are not `#[cfg]`-gated.

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

/// The shared directory holding every instance's lock + socket files. Prefers an
/// ephemeral runtime dir (`XDG_RUNTIME_DIR`); falls back to the app data dir on
/// platforms without one (macOS/Windows). That fallback is persistent, so unlike
/// the tmpfs case a reboot does not clear it — stale lock files are still reaped
/// by pid on every [`scan`], but see the module doc for why sockets needed an
/// explicit reap too.
pub fn dir() -> Option<PathBuf> {
    if let Some(over) = DIR_OVERRIDE.with(|d| d.borrow().clone()) {
        std::fs::create_dir_all(&over).ok()?;
        return Some(over);
    }
    let base: Option<PathBuf> = {
        #[cfg(unix)]
        {
            std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from)
        }
        #[cfg(not(unix))]
        {
            None
        }
    };
    let base = base.or_else(|| {
        bastyde::settings::AppPaths::new("eu", "skribisto", "Skribisto")
            .map(|p| p.data_dir().join("run"))
    })?;
    let d = base.join("skribisto");
    std::fs::create_dir_all(&d).ok()?;
    Some(d)
}

/// Point [`dir`] at a temp directory for the duration of a test (this thread
/// only).
#[cfg(test)]
fn set_dir_override(path: Option<PathBuf>) {
    DIR_OVERRIDE.with(|d| *d.borrow_mut() = path);
}

/// The IPC socket path for the instance owning `pid`.
pub fn ipc_socket_for_pid(pid: u32) -> Option<PathBuf> {
    Some(dir()?.join(format!("ipc-{pid}.sock")))
}

/// This instance's own IPC socket path (where its listener binds).
pub fn my_ipc_socket() -> Option<PathBuf> {
    ipc_socket_for_pid(my_pid())
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
/// (see `view_models::project_lifecycle::ProjectLifecycleViewModel::claim`'s
/// doc — this crate used to have a `replace_claim` helper doing exactly that
/// blanket release; it was Phase 3's own migration bug and was removed once
/// its three call sites were fixed to the release-then-claim pair instead).
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
