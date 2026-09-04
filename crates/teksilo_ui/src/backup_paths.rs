// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where this installation keeps a project's backups by default.
//!
//! The sibling of [`crate::media_paths`], and it exists for the same reason: the
//! backend takes a resolved path, and only the UI knows how to resolve it (the
//! platform's data directory is a `teksilo::settings::AppPaths` question, and
//! `skrib_format` is deliberately store-free and platform-agnostic).
//!
//! ## Why not next to the project
//!
//! Until this module existed, an empty destination list meant "write the backup
//! beside the `.skrib`". That has three problems, and the third is the one that
//! matters for version history:
//!
//! 1. it litters the writer's working folder with `MyNovel-20260806-211817.skrib`
//!    siblings;
//! 2. a project inside a synced folder gets every backup synced too, multiplying
//!    storage and sync churn on exactly the mechanism most often blamed for
//!    corrupting writers' projects; and
//! 3. it dies with the folder — move, rename or delete the project directory and
//!    the history goes with it.
//!
//! A version timeline is only honest if its source is reliably *there*, so the
//! default destination is app-managed and always present. Off-machine copies
//! (a stick, a synced folder) remain worth having and stay a user choice — they
//! answer a different question, "the drive died", not "what did this scene say
//! last week".
//!
//! ## The symbolic default
//!
//! [`BackupPolicy::destinations`](crate::models::BackupPolicy) stays **empty** by
//! default; what changed is what empty *resolves to*. Nothing absolute is ever
//! persisted, because an absolute path baked into `backup.toml` would break the
//! moment the same user ran a Flatpak build instead of a native one (different
//! data root) or moved their home directory. An explicit `""` entry still means
//! "beside the project" — the option survives, it just stopped being the default.
//!
//! Not a view-model — a path helper with no state.

use std::path::PathBuf;

/// The app's default backup root, `<data_dir>/backups`.
///
/// `data_dir` rather than `cache_dir`, for the reason
/// [`crate::media_paths::media_root`] already gives about images: a cache
/// directory is something the OS may reclaim, and reclaiming a writer's only
/// history is precisely the failure this feature exists to prevent.
///
/// Returns an empty path when the platform offers no data directory.
/// [`resolve_destinations`] treats that as "fall back to beside the project", so
/// backups never silently stop on such a platform.
pub fn backup_root() -> PathBuf {
    crate::identity::app_paths()
        .map(|paths| paths.data_dir().join("backups"))
        .unwrap_or_default()
}

/// [`backup_root`] as the `String` the DTOs and settings carry.
pub fn backup_root_string() -> String {
    backup_root().to_string_lossy().into_owned()
}

/// Create the app's backup root if it is not there yet, once per launch.
///
/// The module doc above calls the default destination "app-managed and always
/// present", and until this existed it was neither: nothing created the folder
/// until the *first backup ran*, which on a Flatpak install is
/// `~/.var/app/eu.skribisto.skribisto/data/skribisto/backups`. So a writer who
/// opened Settings ▸ Backup defaults on a fresh install read a path that was not
/// on the disk, and reaching for it in a file manager — through the Show folder
/// button or by hand — found nothing there. "Where does my work get backed up" answered with
/// a path that does not exist reads as a broken setting, not as an empty one.
///
/// Idempotent and best-effort: a failure is reported and nothing else changes.
/// The write path does **not** depend on this — `backup_now` creates the
/// destination itself, and must, since a configured destination can be a stick
/// that was unplugged since. This only makes the app's own default visible
/// before the first backup lands in it.
///
/// Returns whether the root exists afterwards.
pub fn ensure_backup_root() -> bool {
    ensure_root(&backup_root())
}

/// The `root`-injected half of [`ensure_backup_root`], so the rule is testable
/// without creating a folder in the developer's own data directory.
///
/// An empty `root` is "this platform has no data directory": there is nothing to
/// make, and [`resolve_destinations`] already falls back to writing beside the
/// project, so it is a `false` rather than an error.
fn ensure_root(root: &std::path::Path) -> bool {
    if root.as_os_str().is_empty() {
        return false;
    }
    match std::fs::create_dir_all(root) {
        Ok(()) => true,
        Err(e) => {
            eprintln!(
                "skribisto: could not create the backup folder {}: {e}",
                root.display()
            );
            false
        }
    }
}

/// The destinations a backup run should actually write to.
///
/// Pure, and `default_root`-injected, so the policy → directories mapping is
/// unit-testable without touching the OS. [`effective_destinations`] is the
/// production wrapper.
///
/// * a non-empty configured list is used verbatim (including any explicit `""`,
///   which the engine resolves to the project's own folder);
/// * an empty list resolves to `default_root`;
/// * an empty list *and* an unavailable `default_root` falls back to the historical
///   beside-the-project behaviour (`vec![String::new()]`), because a backup written
///   somewhere odd beats no backup at all.
pub fn resolve_destinations(configured: &[String], default_root: &str) -> Vec<String> {
    if !configured.is_empty() {
        return configured.to_vec();
    }
    if default_root.trim().is_empty() {
        vec![String::new()]
    } else {
        vec![default_root.to_string()]
    }
}

/// [`resolve_destinations`] against this installation's real data directory.
pub fn effective_destinations(configured: &[String]) -> Vec<String> {
    resolve_destinations(configured, &backup_root_string())
}

/// Every directory a **reader** of this project's past must look in.
///
/// Strictly wider than [`effective_destinations`], and the difference is not
/// cosmetic. That one answers "where should the next backup be *written*"; this
/// answers "where might a backup already *be*", and the two diverged the moment
/// the default destination changed:
///
/// * the app's backup root — where an unconfigured project's backups land now;
/// * the project's own folder (`""`) — where they landed **before** that default
///   changed, so a reader that stopped looking there would hide a writer's
///   entire existing history behind an empty pane.
///
/// Both are unconditional, and neither is redundant. Duplicates are dropped, so
/// naming a directory twice costs nothing; every consumer of this list
/// de-duplicates by *resolved* path as well.
///
/// One function rather than the rule spelled out at each call site: the backups
/// browser, the Versions dock and the Timeline band all have to agree about
/// where history lives, and the second and third were written months after the
/// first learned this the hard way.
pub fn search_destinations(configured: &[String]) -> Vec<String> {
    let mut dirs = effective_destinations(configured);
    for extra in [backup_root_string(), String::new()] {
        if !dirs.contains(&extra) {
            dirs.push(extra);
        }
    }
    dirs
}

/// What the app's backup root currently holds.
///
/// Every project's backups share one directory (they are told apart by
/// `Work.unique_id` in each manifest, never by filename), so this is deliberately
/// an app-wide total rather than a per-project one: it answers "how much disk is
/// Skribisto using for backups", which is the question a writer has once the files
/// stop being visible in their own folder.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RootUsage {
    /// Bundles found — a zip `.skrib` file or an exploded-folder bundle each count once.
    pub count: usize,
    pub bytes: u64,
    /// The oldest bundle's date, as `YYYY-MM-DD`, or empty when nothing is kept.
    ///
    /// Here for reassurance rather than for arithmetic. The strongest finding in
    /// the research behind this feature is that version history gets built and
    /// then forgotten — *"I mostly take snapshots and never look at them again, I
    /// just like knowing I have all my versions."* Most writers will open these
    /// surfaces two or three times a year; what they want the rest of the time is
    /// a quiet sentence saying how far back they are covered.
    ///
    /// Read from the filename stamp where there is one, and from the file's
    /// modification time otherwise — the same order `retention` resolves a
    /// candidate's timestamp in, so the two never disagree about which backup is
    /// the oldest.
    pub oldest: String,
}

/// Measure [`backup_root`]. Blocking filesystem I/O — call it off the UI thread.
///
/// A missing or unreadable root is reported as an empty usage rather than an error:
/// "no backups yet" and "the directory does not exist yet" are the same fact to a
/// reader, and the first run is exactly when it does not exist.
pub fn scan_root_usage(root: &std::path::Path) -> RootUsage {
    let mut usage = RootUsage::default();
    let Ok(entries) = std::fs::read_dir(root) else {
        return usage;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let is_zip = path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("skrib");
        // Mirrors `retention::scan_destination`'s notion of a folder bundle, so the
        // two never disagree about what counts as a backup.
        let is_folder_bundle = path.is_dir() && path.join("project.skrib").is_file();
        if !is_zip && !is_folder_bundle {
            continue;
        }
        usage.count += 1;
        usage.bytes += byte_size(&path);
        if let Some(day) = bundle_day(&path)
            && (usage.oldest.is_empty() || day < usage.oldest)
        {
            usage.oldest = day;
        }
    }
    usage
}

/// One bundle's date as `YYYY-MM-DD`, from its `-YYYYMMDD-HHMMSS` stamp when it
/// carries one and from its modification time otherwise.
///
/// The name is read by `retention::parse_stamp_from_filename` rather than by a
/// second parser here — the same fallback order `retention::candidate_timestamp`
/// uses, so the settings pane and the retention sweep can never disagree about
/// which backup is the oldest. A looser local reading (any eight digits, without
/// checking that a six-digit time follows) dated `foo-12345678-1.skrib` from a
/// segment that was never a timestamp at all.
fn bundle_day(path: &std::path::Path) -> Option<String> {
    let when = skrib_format::retention::parse_stamp_from_filename(path).or_else(|| {
        let modified = std::fs::metadata(path).ok()?.modified().ok()?;
        Some(chrono::DateTime::<chrono::Utc>::from(modified))
    })?;
    Some(when.format("%Y-%m-%d").to_string())
}

/// Recursive size of a file or directory; unreadable entries contribute nothing.
fn byte_size(path: &std::path::Path) -> u64 {
    let Ok(meta) = std::fs::metadata(path) else {
        return 0;
    };
    if meta.is_file() {
        return meta.len();
    }
    let Ok(entries) = std::fs::read_dir(path) else {
        return 0;
    };
    entries.flatten().map(|e| byte_size(&e.path())).sum()
}

/// Human-readable byte count for the settings readout.
pub fn human_bytes(bytes: u64) -> String {
    const MB: u64 = 1_048_576;
    const GB: u64 = 1_073_741_824;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else {
        format!("{} kB", bytes.div_ceil(1024))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── where to *read* history, as against where to *write* it ────────────

    /// **The regression this exists for.** Backups made before the default
    /// destination moved sit beside the project. A reader built from the write
    /// destinations alone never looks there, so every project with a history
    /// showed an empty pane until its owner happened to take a fresh backup.
    #[test]
    fn a_reader_always_looks_beside_the_project_as_well() {
        let dirs = search_destinations(&[]);
        assert!(
            dirs.iter().any(|d| d.is_empty()),
            "the project's own folder is where backups landed before the default \
             moved, and it is not optional for a reader: {dirs:?}",
        );
        assert!(
            dirs.iter().any(|d| d == &backup_root_string()),
            "…and the app root is where they land now: {dirs:?}",
        );
    }

    /// A configured destination is still honoured, and still joined by both.
    #[test]
    fn a_configured_destination_is_kept_and_widened_never_replaced() {
        let dirs = search_destinations(&["/media/stick".to_string()]);
        assert_eq!(dirs.first().map(String::as_str), Some("/media/stick"));
        assert!(dirs.iter().any(|d| d.is_empty()));
        assert!(dirs.iter().any(|d| d == &backup_root_string()));
    }

    #[test]
    fn naming_a_directory_twice_costs_nothing() {
        let root = backup_root_string();
        let dirs = search_destinations(&[root.clone(), String::new()]);
        assert_eq!(
            dirs.iter().filter(|d| *d == &root).count(),
            1,
            "the root must appear once, however it was reached: {dirs:?}",
        );
        assert_eq!(dirs.iter().filter(|d| d.is_empty()).count(), 1);
    }

    /// Writing stays narrow: a backup goes to one place, not to three.
    #[test]
    fn the_write_destinations_are_not_widened() {
        let write = effective_destinations(&["/media/stick".to_string()]);
        assert_eq!(write, vec!["/media/stick".to_string()]);
    }

    /// The oldest-backup date reads a name by the **same** rule retention does.
    /// A looser second parser dated a file from an eight-digit run that was never
    /// a timestamp, so the settings pane claimed cover reaching back to a day
    /// nothing was written.
    #[test]
    fn a_bundles_date_is_read_by_the_same_rule_retention_uses() {
        let dir = tempfile::tempdir().expect("tmp");
        let real = dir.path().join("Novel-20260314-093000.skrib");
        std::fs::write(&real, b"x").expect("write");
        assert_eq!(bundle_day(&real).as_deref(), Some("2026-03-14"));

        // Eight digits, but the segment after them is not a `HHMMSS` time — so
        // this is not a stamped backup name and must fall back to the mtime,
        // never to "2026-08-07" invented out of `12345678`.
        let bogus = dir.path().join("foo-12345678-1.skrib");
        std::fs::write(&bogus, b"x").expect("write");
        let day = bundle_day(&bogus).expect("a date from the file's own mtime");
        assert_ne!(
            day, "1234-56-78",
            "an eight-digit run is not a date just because it is eight digits",
        );
        assert!(
            day.starts_with("20"),
            "the fallback is the modification time, which is a real date: {day}",
        );
    }

    #[test]
    fn an_empty_policy_resolves_to_the_app_default_root() {
        assert_eq!(
            resolve_destinations(&[], "/data/backups"),
            vec!["/data/backups".to_string()],
        );
    }

    #[test]
    fn a_configured_list_is_used_verbatim() {
        let configured = vec!["/stick".to_string(), "/nas".to_string()];
        assert_eq!(
            resolve_destinations(&configured, "/data/backups"),
            configured,
            "an explicit destination list must never be second-guessed",
        );
    }

    #[test]
    fn an_explicit_empty_entry_still_means_beside_the_project() {
        // The option survives the default changing: `""` is how a writer asks for
        // the pre-existing behaviour, and it must not be swallowed by the new root.
        let configured = vec![String::new()];
        assert_eq!(
            resolve_destinations(&configured, "/data/backups"),
            configured
        );
    }

    #[test]
    fn usage_counts_zip_and_folder_bundles_but_not_strays() {
        let d = tempfile::tempdir().unwrap();
        let root = d.path();

        std::fs::write(root.join("novel-20260806-101010.skrib"), b"0123456789").unwrap();

        // An exploded-folder bundle: a directory carrying a project.skrib manifest.
        let folder = root.join("other-20260806-111111.skrib");
        std::fs::create_dir_all(folder.join("binders")).unwrap();
        std::fs::write(folder.join("project.skrib"), b"01234").unwrap();
        std::fs::write(folder.join("binders/items.ron"), b"012345").unwrap();

        // Neither of these is a backup, and neither may be counted or measured.
        std::fs::write(root.join("notes.txt"), b"xxxxxxxxxxxxxxxxxxxx").unwrap();
        std::fs::create_dir_all(root.join("scratch")).unwrap();

        let usage = scan_root_usage(root);
        assert_eq!(usage.count, 2, "one zip bundle and one folder bundle");
        assert_eq!(
            usage.bytes,
            10 + 5 + 6,
            "a folder bundle is measured recursively, and strays contribute nothing",
        );
    }

    /// **The regression this exists for.** The backup root used to appear only
    /// when the first backup was written into it, so Settings ▸ Backup defaults
    /// named a folder that was not on the disk on every fresh install.
    #[test]
    fn the_backup_root_is_made_before_the_first_backup_is_written() {
        let d = tempfile::tempdir().unwrap();
        // Nested, because the data directory itself may not exist yet either on
        // the launch this runs for.
        let root = d.path().join("skribisto").join("backups");
        assert!(!root.exists());
        assert!(
            ensure_root(&root),
            "the app's own default must be creatable"
        );
        assert!(root.is_dir(), "{} must be a directory", root.display());
        // Idempotent: a second launch finds it and does not report a failure.
        assert!(ensure_root(&root));
    }

    /// A platform with no data directory has nothing to create, and says so
    /// rather than trying to make a folder called "".
    #[test]
    fn no_data_dir_has_no_root_to_make() {
        assert!(!ensure_root(std::path::Path::new("")));
    }

    /// A root that cannot be made is reported, not panicked on — the write path
    /// creates its own destination anyway, so a backup still happens.
    #[test]
    fn an_uncreatable_root_is_reported_not_fatal() {
        let d = tempfile::tempdir().unwrap();
        let blocker = d.path().join("backups");
        std::fs::write(&blocker, b"not a directory").unwrap();
        assert!(!ensure_root(&blocker));
    }

    #[test]
    fn usage_of_a_missing_root_is_empty_not_an_error() {
        // The first run is exactly when the directory does not exist yet.
        let d = tempfile::tempdir().unwrap();
        assert_eq!(
            scan_root_usage(&d.path().join("never-created")),
            RootUsage::default()
        );
    }

    #[test]
    fn human_bytes_reads_naturally_at_each_scale() {
        assert_eq!(human_bytes(0), "0 kB");
        assert_eq!(
            human_bytes(1),
            "1 kB",
            "a non-empty backup never reads as 0"
        );
        assert_eq!(human_bytes(1_048_576), "1.0 MB");
        assert_eq!(human_bytes(1_073_741_824), "1.0 GB");
    }

    #[test]
    fn no_data_dir_falls_back_to_beside_the_project() {
        // A platform with no home directory still gets backups, rather than
        // silently getting none.
        assert_eq!(resolve_destinations(&[], ""), vec![String::new()]);
        assert_eq!(resolve_destinations(&[], "   "), vec![String::new()]);
    }
}
