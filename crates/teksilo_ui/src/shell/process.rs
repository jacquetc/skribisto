// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Path helpers shared by the surfaces that reach outside this window.
//!
//! Skribisto is single-instance: "open that project" goes through
//! [`crate::shell::windows::open_or_focus_project`], never a spawned second
//! process (a spawned child would just elect, find this process as the
//! primary, and hand the path back over a socket). What is left here are the
//! helpers that were never about spawning at all.

use std::path::Path;

/// Best-effort canonical form for comparing project paths across the open registry (which
/// stores canonical paths) and the recents list.
///
/// Falls back to the input unchanged when the path does not resolve — an unreachable network
/// mount or a deleted project still has to compare *somehow*, and comparing the raw string is
/// better than dropping the entry.
pub(crate) fn canon(path: &str) -> String {
    std::fs::canonicalize(path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string())
}

/// Open the file manager on the folder holding `path`, so `path` itself is one
/// of the entries in view.
///
/// For a **file**: an exported `.docx`, one backup among forty. Also for an
/// exploded-folder `.skrib` bundle, which is a document that happens to be a
/// directory — entering it would show the writer `project.skrib` and `binders/`
/// rather than the backup among its siblings.
///
/// To open a folder *as* a folder, call [`open_folder`]. Which of the two a
/// caller wants is something the caller knows and the filesystem cannot be
/// asked: sniffing `is_dir` here is what would break the bundle case above.
pub(crate) fn reveal_in_file_manager(path: &str) {
    open_with_desktop(&reveal_target(path));
}

/// The folder [`reveal_in_file_manager`] hands to the desktop. Pure, so the
/// rule is testable without spawning anything.
fn reveal_target(path: &str) -> std::path::PathBuf {
    Path::new(path)
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| std::path::PathBuf::from("."))
}

/// Open the file manager **on** `dir`, showing what is inside it.
///
/// The counterpart to [`reveal_in_file_manager`], and separate from it because
/// routing a directory through that one opens its *parent*: Settings ▸ Backup
/// defaults ▸ Show folder printed `<data>/skribisto/backups` and opened
/// `<data>/skribisto`, one level above the folder it had just named.
///
/// A folder the app offers to open is one the app promises exists — see
/// [`crate::backup_paths::ensure_backup_root`] for the other half of that.
pub(crate) fn open_folder(dir: &str) {
    open_with_desktop(Path::new(dir));
}

/// Hand `path` to whatever the desktop opens that kind of file with — a `.docx` to the word
/// processor, a `.pdf` to the viewer.
///
/// Best-effort and deliberately fire-and-forget: there is no portable way to learn that the
/// handler actually appeared, and a project must never be blocked waiting on one. A desktop
/// with nothing registered for the type simply does nothing, which is the same outcome as
/// not offering the button — so the failure mode costs the writer one click, not their work.
pub(crate) fn open_in_default_app(path: &str) {
    open_with_desktop(Path::new(path));
}

fn open_with_desktop(target: &Path) {
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(target).spawn();
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(target).spawn();
    #[cfg(target_os = "windows")]
    // `explorer` is the launcher on Windows for both a folder and a document; `start` is a
    // shell builtin and has no executable to spawn.
    let _ = std::process::Command::new("explorer").arg(target).spawn();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A file is revealed by opening the folder that holds it.
    #[test]
    fn a_file_is_revealed_through_its_parent() {
        let dir = tempfile::tempdir().expect("tmp");
        let file = dir.path().join("Novel-20260904-184201.skrib");
        std::fs::write(&file, b"x").expect("write");
        assert_eq!(reveal_target(&file.to_string_lossy()), dir.path());
    }

    /// An exploded-folder bundle is a document, not a folder to enter: it is
    /// revealed among its siblings like any other backup. This is why the
    /// directory case is a separate function and not an `is_dir` test here.
    #[test]
    fn a_folder_shaped_bundle_is_still_revealed_through_its_parent() {
        let dir = tempfile::tempdir().expect("tmp");
        let bundle = dir.path().join("Novel-20260904-184201.skrib");
        std::fs::create_dir_all(&bundle).expect("mkdir");
        std::fs::write(bundle.join("project.skrib"), b"x").expect("write");
        assert_eq!(
            reveal_target(&bundle.to_string_lossy()),
            dir.path(),
            "entering the bundle would show its internals, not the backup",
        );
    }

    /// **The regression the split exists for.** A folder routed through the
    /// reveal helper lands one level above itself, which is what Settings ▸
    /// Backup defaults ▸ Show folder did to the backup root.
    #[test]
    fn a_folder_routed_through_reveal_lands_above_itself() {
        let dir = tempfile::tempdir().expect("tmp");
        let backups = dir.path().join("backups");
        std::fs::create_dir_all(&backups).expect("mkdir");
        assert_eq!(
            reveal_target(&backups.to_string_lossy()),
            dir.path(),
            "which is why the backup root goes through `open_folder` instead",
        );
    }

    /// A bare filename has no parent to open; the working directory is the only
    /// honest answer, and is what the helper fell back to before as well.
    #[test]
    fn a_bare_name_falls_back_to_the_working_directory() {
        assert_eq!(reveal_target("novel.skrib"), std::path::PathBuf::from("."));
        assert_eq!(reveal_target(""), std::path::PathBuf::from("."));
    }
}
