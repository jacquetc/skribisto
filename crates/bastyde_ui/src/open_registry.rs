//! Cross-instance "open project" registry.
//!
//! Skribisto is multi-process: each running instance holds at most one project,
//! in its own window. While a project is open, the instance writes a small lock
//! file naming that project + its own pid into a shared runtime directory. Other
//! instances [`scan`] these to show which projects are open elsewhere (the
//! ProjectSwitcher "Currently open" section) and to reach the owning process —
//! its IPC socket path is derived from the pid ([`ipc_socket_for_pid`]) — to ask
//! it to raise its window.
//!
//! Crash-safe by construction: a lock whose pid is no longer alive is reaped on
//! [`scan`]; the mere presence of a file never means "open". These are OS-level
//! functions, independent of the (mock or real) backend, so they are not
//! `#[cfg]`-gated.

use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

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
    /// The lock file this process currently holds (if a project is open), so it
    /// can be removed on the next claim / on release without re-deriving it.
    static CLAIMED: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

/// This process's pid — the "is this my own window?" discriminator for the UI.
pub fn my_pid() -> u32 {
    std::process::id()
}

/// The shared directory holding every instance's lock + socket files. Prefers an
/// ephemeral runtime dir (`XDG_RUNTIME_DIR`); falls back to the app data dir on
/// platforms without one (macOS/Windows) — stale entries are reaped by pid, so
/// surviving a reboot in the fallback case is harmless.
pub fn dir() -> Option<PathBuf> {
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

fn lock_path_for(project_path: &str) -> Option<PathBuf> {
    let d = dir()?;
    let mut h = DefaultHasher::new();
    canonical(project_path).hash(&mut h);
    Some(d.join(format!("open-{:016x}.lock", h.finish())))
}

/// Claim `path` as open by this process. Releases any prior claim first — opening
/// a *different* project does not emit `CloseWork`, so the previous lock must be
/// dropped here rather than relying on a close event.
pub fn claim(path: &str, title: &str) {
    release();
    let Some(lock) = lock_path_for(path) else {
        return;
    };
    let entry = OpenEntry {
        pid: my_pid(),
        path: canonical(path),
        title: title.to_string(),
    };
    if let Ok(json) = serde_json::to_string(&entry)
        && std::fs::write(&lock, json).is_ok()
    {
        CLAIMED.with(|c| *c.borrow_mut() = Some(lock));
    }
}

/// Remove this process's lock file, if any. Idempotent — safe to call on
/// `CloseWork` and again at shutdown.
pub fn release() {
    CLAIMED.with(|c| {
        if let Some(lock) = c.borrow_mut().take() {
            let _ = std::fs::remove_file(lock);
        }
    });
}

/// Every project currently open across live instances, reaping any lock whose
/// owning process is gone. Includes this instance's own project (compare
/// `entry.pid == my_pid()`).
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
        if p.extension().and_then(|e| e.to_str()) != Some("lock") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&p) else {
            continue;
        };
        match serde_json::from_str::<OpenEntry>(&text) {
            Ok(entry) if pid_is_alive(entry.pid) => out.push(entry),
            // Unparsable or dead-owner ⇒ stale; reap it.
            _ => {
                let _ = std::fs::remove_file(&p);
            }
        }
    }
    out
}

/// Is `pid` a live process? Used only to reap stale locks — a false "alive" (PID
/// reuse) merely keeps a stale entry one scan longer, never a correctness issue.
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
