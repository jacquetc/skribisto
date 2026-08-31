// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! First-run settings import — the IntelliJ-shaped offer.
//!
//! An edition that registers its own [`crate::identity`] gets its own config
//! directory, which is what separates it from the community build's
//! single-instance election. The cost of that separation is that it starts with
//! nothing: no recents, no window geometry, no dictionaries, none of the writer's
//! preferences. So the first time such an edition runs on a machine where the
//! community build has already been used, it offers to copy them across.
//!
//! **Copy, never move, and never delete.** The community installation must be
//! exactly as it was — still runnable, still holding every setting — after an
//! import. A writer who tries an edition and goes back must find nothing changed.
//!
//! ## The offer is made once
//!
//! [`pending`] returns a source only when **all four** hold:
//!
//! 1. this is not the community edition (it has nothing to import *from*);
//! 2. our own config directory has no `general.toml` — we have never run;
//! 3. the family config directory *does* have one — there is something to take;
//! 4. no [`MARKER`] file records an answer.
//!
//! The marker is written on **either** answer, which is what makes "Start fresh"
//! stick. An importer that re-asks every launch is the part of this pattern
//! everyone remembers hating.
//!
//! ## Why an in-place copy is enough
//!
//! Settings are opened before any window exists, so by the time the writer can
//! answer, every service already points at this edition's (empty) files. Copying
//! underneath them still works because teksilo watches the settings
//! *directories* and dispatches each changed path to the handle that owns it
//! (`teksilo-settings::watch`) — the same mechanism that keeps two running
//! processes in sync. No restart, and no re-opening of services by hand.
//!
//! Concurrent-write safety comes free from the same crate: every settings write
//! is temp-file + `sync_all` + rename under an exclusive lock, so copying a file
//! the community app is writing *right now* yields the old bytes or the new ones,
//! never a torn mix.

use std::io;
use std::path::Path;

use teksilo::settings::AppPaths;

/// Records that the offer was answered, so it is never made twice. Lives in the
/// *edition's* config directory — the one place guaranteed to be ours alone.
pub const MARKER: &str = "first_run";

/// Directories under the data dir that must never be copied.
///
/// `run/` holds this installation's IPC sockets and open-project lock files. They
/// are process-scoped and pid-keyed; copying them would plant another
/// installation's stale claims in our tree, where `scan` would faithfully reap
/// them and nothing good would come of the round trip. (On Linux they live in
/// `XDG_RUNTIME_DIR` and are not under the data dir at all; on macOS and Windows
/// they are, which is exactly why this list is not empty.)
const DATA_DIR_SKIP: &[&str] = &["run"];

/// What an import moved, for the message shown afterwards.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ImportReport {
    pub files: usize,
    pub bytes: u64,
}

/// The installation to offer an import from, or `None` when no offer should be
/// made. See the module docs for the four conditions.
///
/// ⚠ **Call once, early in [`crate::run`], before the settings services open.**
/// Opening them creates `general.toml` with defaults, after which condition 2 is
/// false and the offer would never be made again — on the very launch it was
/// meant for.
pub fn pending() -> Option<AppPaths> {
    let ours = crate::identity::app_paths()?;
    if crate::identity::is_community() {
        return None;
    }
    if answered(&ours) || ours.config_file("general").exists() {
        return None;
    }
    let family = crate::identity::family_paths()?;
    // Same directory means the edition declared paths that resolve to the
    // community's after all — there is nothing to copy and, more to the point,
    // copying a tree onto itself is not something to find out the hard way.
    if family.config_dir() == ours.config_dir() {
        return None;
    }
    family.config_file("general").exists().then_some(family)
}

/// Whether an answer has already been recorded for this edition.
pub fn answered(ours: &AppPaths) -> bool {
    ours.config_file(MARKER).exists()
}

/// Record that the writer answered, so the offer is never repeated.
///
/// Written on **both** answers. Failure is reported and swallowed: a marker that
/// could not be written means the offer comes back next launch, which is a
/// nuisance rather than a hazard, and is not worth refusing to start over.
pub fn mark_answered(ours: &AppPaths, imported: bool) {
    let path = ours.config_file(MARKER);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let body = format!(
        "# Written once, the first time this edition ran.\n\
         # Delete this file to be offered the import again.\n\
         answered = true\nimported = {imported}\n"
    );
    if let Err(e) = std::fs::write(&path, body) {
        eprintln!(
            "skribisto: could not record the first-run answer at {}: {e} — the import will be \
             offered again next launch",
            path.display()
        );
    }
}

/// Copy `from`'s settings into this edition's directories.
///
/// Overwrites: by the time this runs the settings services have already created
/// their files with defaults, and an import that lost to a default it was meant
/// to replace would be worse than useless.
///
/// Partial failure is **reported, not fatal** — a single unreadable file should
/// not cost the writer every other setting. The count returned is what actually
/// landed.
pub fn import(from: &AppPaths) -> io::Result<ImportReport> {
    let ours = crate::identity::app_paths()
        .ok_or_else(|| io::Error::other("no config directory for the running edition"))?;

    let mut report = ImportReport::default();
    copy_tree(from.config_dir(), ours.config_dir(), &[], &mut report)?;
    // The data dir holds the recents list the writer asked to carry over, plus
    // window geometry — but not this installation's live sockets.
    if from.data_dir() != from.config_dir() {
        copy_tree(from.data_dir(), ours.data_dir(), DATA_DIR_SKIP, &mut report)?;
    }
    Ok(report)
}

/// Recursively copy `src` into `dst`, skipping top-level entries named in `skip`.
///
/// A missing source is not an error — an installation that never wrote a data dir
/// simply has nothing there to take.
fn copy_tree(src: &Path, dst: &Path, skip: &[&str], report: &mut ImportReport) -> io::Result<()> {
    if !src.is_dir() {
        return Ok(());
    }
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        if skip.iter().any(|s| std::ffi::OsStr::new(s) == name) {
            continue;
        }
        // Never carry the source's own answer marker across: it would tell this
        // edition it had already answered an offer it has not yet been made.
        if name == std::ffi::OsStr::new(&format!("{MARKER}.toml")) {
            continue;
        }
        let target = dst.join(&name);
        let path = entry.path();
        if path.is_dir() {
            copy_tree(&path, &target, &[], report)?;
        } else {
            match std::fs::copy(&path, &target) {
                Ok(bytes) => {
                    report.files += 1;
                    report.bytes += bytes;
                }
                Err(e) => eprintln!(
                    "skribisto: could not import {}: {e} — every other setting was still copied",
                    path.display()
                ),
            }
        }
    }
    Ok(())
}

/// A short, human description of where an import would come from, for the offer.
pub fn source_label(from: &AppPaths) -> String {
    from.config_dir().display().to_string()
}

/// The path an import would copy *into* — shown beside the source so the writer
/// can see the copy is a copy.
pub fn destination_label() -> String {
    crate::identity::app_paths()
        .map(|p| p.config_dir().display().to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sandbox(name: &str) -> std::path::PathBuf {
        let dir =
            std::env::temp_dir().join(format!("skribisto-first-run-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn seeded(root: &Path) -> AppPaths {
        std::fs::create_dir_all(root).unwrap();
        std::fs::write(root.join("general.toml"), "ui.locale = \"fr-FR\"\n").unwrap();
        std::fs::write(
            root.join("recents.toml"),
            "[[entries]]\npath = \"/x.skrib\"\n",
        )
        .unwrap();
        std::fs::create_dir_all(root.join("run")).unwrap();
        std::fs::write(root.join("run").join("primary.sock"), "not a real socket").unwrap();
        AppPaths::for_testing(root)
    }

    /// The whole promise: after an import the source is byte-for-byte what it
    /// was. A writer who tries an edition and goes back must find nothing moved.
    #[test]
    fn an_import_copies_and_never_moves() {
        let root = sandbox("copies");
        let src = seeded(&root.join("community"));
        let dst_dir = root.join("edition");
        let dst = AppPaths::for_testing(&dst_dir);

        let mut report = ImportReport::default();
        copy_tree(src.config_dir(), dst.config_dir(), &[], &mut report).unwrap();

        assert!(
            report.files >= 2,
            "expected the seeded files, got {report:?}"
        );
        assert!(
            src.config_file("general").exists() && src.config_file("recents").exists(),
            "the source installation must be untouched by an import"
        );
        assert_eq!(
            std::fs::read_to_string(dst_dir.join("general.toml")).unwrap(),
            "ui.locale = \"fr-FR\"\n",
            "the imported settings must arrive intact"
        );
        assert!(
            dst_dir.join("recents.toml").exists(),
            "recents are part of what the writer asked to carry over"
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    /// `run/` holds pid-keyed sockets and open-project claims belonging to
    /// another installation; importing them plants stale claims in our tree.
    #[test]
    fn an_import_leaves_the_instance_directory_behind() {
        let root = sandbox("skips-run");
        let src = seeded(&root.join("community"));
        let dst_dir = root.join("edition");

        let mut report = ImportReport::default();
        copy_tree(src.config_dir(), &dst_dir, DATA_DIR_SKIP, &mut report).unwrap();

        assert!(
            !dst_dir.join("run").exists(),
            "another installation's sockets and lock files must not be imported"
        );
        assert!(dst_dir.join("general.toml").exists());
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Importing the source's own marker would tell this edition it had already
    /// answered an offer it has not yet been made.
    #[test]
    fn an_import_never_carries_the_answer_marker_across() {
        let root = sandbox("marker");
        let src_dir = root.join("community");
        let _ = seeded(&src_dir);
        std::fs::write(src_dir.join(format!("{MARKER}.toml")), "answered = true\n").unwrap();
        let dst_dir = root.join("edition");

        let mut report = ImportReport::default();
        copy_tree(&src_dir, &dst_dir, &[], &mut report).unwrap();

        assert!(
            !dst_dir.join(format!("{MARKER}.toml")).exists(),
            "the source's answer marker must not be imported"
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    /// An answer sticks. This is what stops the offer reappearing every launch.
    #[test]
    fn an_answer_is_recorded_and_recognised() {
        let root = sandbox("answered");
        let ours = AppPaths::for_testing(&root);
        assert!(!answered(&ours));
        mark_answered(&ours, false);
        assert!(
            answered(&ours),
            "declining the import must be remembered, or it is offered again forever"
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    /// The community edition has nothing to import from — itself least of all.
    #[test]
    fn the_community_edition_is_never_offered_an_import() {
        let _serial = crate::identity::lock_for_test();
        assert!(
            pending().is_none(),
            "the community build must never be offered its own settings"
        );
    }

    /// An edition whose declared paths happen to resolve to the community's is
    /// the community installation wearing a different name; copying its tree
    /// onto itself is not a discovery to make at runtime.
    #[test]
    fn an_edition_sharing_the_community_directories_is_not_offered_an_import() {
        let _serial = crate::identity::lock_for_test();
        let _h = crate::identity::register(
            crate::identity::AppIdentity::community().with_display_name("Renamed"),
        );
        assert!(pending().is_none());
    }

    #[test]
    fn a_report_counts_what_landed() {
        let root = sandbox("report");
        let src = root.join("a");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("one.toml"), "xx").unwrap();
        std::fs::write(src.join("two.toml"), "yyy").unwrap();

        let mut report = ImportReport::default();
        copy_tree(&src, &root.join("b"), &[], &mut report).unwrap();

        assert_eq!(report.files, 2);
        assert_eq!(report.bytes, 5);
        let _ = std::fs::remove_dir_all(&root);
    }
}
