// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Dispatch a write to the right physical shape.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io;
use std::path::Path;
use tempfile::NamedTempFile;

use super::bundle::WorkBundle;
use super::shape::{SkribShape, folder_root};
use super::{folder_io, zip_io};

/// Write `bundle` to `path` in `shape`, atomically. The legacy SQLite shape is
/// never written (it is read-only legacy input).
pub fn write_bundle(path: &str, shape: SkribShape, bundle: &WorkBundle) -> Result<()> {
    match shape {
        SkribShape::ExplodedFolder => folder_io::write_folder(&folder_root(path), bundle),
        SkribShape::ZipFile => zip_io::write_zip(Path::new(path), bundle),
        SkribShape::LegacySqlite => {
            anyhow::bail!("the legacy SQLite format is read-only; save as a .skrib zip or folder")
        }
    }
}

/// Persist a [`NamedTempFile`] over `target` **durably**: sync the temp file's
/// own contents (propagating any real failure, never swallowing one), close it,
/// atomically rename it over `target`, then sync the *parent directory* so the
/// rename entry itself survives a crash.
///
/// `tempfile::persist` gives atomicity only: its own docs say "neither the
/// file contents nor the containing directory are synchronized". Without this
/// a power cut can leave a valid-looking but empty/truncated backup on disk,
/// one that retention would happily count as a real, restorable backup.
///
/// The directory step comes after the file is already in place, so what it can
/// no longer do is report a failed save for a save that landed: see
/// [`sync_directory`].
pub(crate) fn persist_durably(tmp: NamedTempFile, target: &Path) -> Result<()> {
    if let Err(e) = sync_file(tmp.as_file()) {
        discard(tmp);
        anyhow::bail!("fsyncing temp file for {}: {e}", target.display());
    }

    // Closed before the rename, not after it. macOS's WebDAV client holds a lock on a
    // file it has open, so renaming one fails: the server answers the MOVE with 424,
    // which reaches us as `EBUSY`. The contents are already synced, so nothing is lost.
    let (file, path) = tmp.into_parts();
    drop(file);
    if let Err(e) = path.persist(target) {
        let reason = e.error.to_string();
        // Nothing a caller could do differently: it is about to report the failure.
        let _ = e.path.close();
        anyhow::bail!("persisting {}: {reason}", target.display());
    }

    #[cfg(unix)]
    if let Some(parent) = target.parent().filter(|p| !p.as_os_str().is_empty()) {
        sync_directory(parent)?;
    }

    Ok(())
}

/// Remove a temp file that will not be used, closing it **first**.
///
/// Dropping a [`NamedTempFile`] removes the name while the file is still open,
/// and a network volume can refuse that: macOS's WebDAV client answers `EBUSY`,
/// so every failed save used to leave a `.tmpXXXXXX` beside the project.
pub(crate) fn discard(tmp: NamedTempFile) {
    let (file, path) = tmp.into_parts();
    drop(file);
    // Nothing a caller could do differently: it is already reporting the failure
    // that brought it here.
    let _ = path.close();
}

/// Sync `file` to its storage, as far as its file system can be asked to.
///
/// On Apple platforms [`File::sync_all`] is `fcntl(F_FULLFSYNC)` with no
/// fallback, and that is a request a file system has to implement: a WebDAV
/// mount answers `ENOTTY`, which made every save on one fail. Plain `fsync` is
/// the most such a volume offers, so it is what gets asked next, as SQLite does.
#[cfg(unix)]
fn sync_file(file: &File) -> io::Result<()> {
    sync_with_fallback(|| file.sync_all(), || plain_fsync(file))
}

/// There is only the one request to make off Unix.
#[cfg(not(unix))]
fn sync_file(file: &File) -> io::Result<()> {
    file.sync_all()
}

/// `full`, then `plain` only if the file system does not implement `full`. A
/// real failure (an I/O error, a full disk) is the answer and is never retried.
#[cfg(unix)]
fn sync_with_fallback(
    full: impl FnOnce() -> io::Result<()>,
    plain: impl FnOnce() -> io::Result<()>,
) -> io::Result<()> {
    match full() {
        Err(e) if sync_not_implemented(&e) => plain(),
        other => other,
    }
}

/// The answers meaning "this file system has no such request", as opposed to
/// "the request failed".
#[cfg(unix)]
fn sync_not_implemented(e: &io::Error) -> bool {
    // Not a `match`: `ENOTSUP` and `EOPNOTSUPP` are one value on Linux and two
    // on Apple platforms.
    e.raw_os_error().is_some_and(|code| {
        [
            libc::ENOTTY,
            libc::ENOTSUP,
            libc::EOPNOTSUPP,
            libc::ENOSYS,
            libc::EINVAL,
        ]
        .contains(&code)
    })
}

#[cfg(unix)]
fn plain_fsync(file: &File) -> io::Result<()> {
    use std::os::fd::AsRawFd;
    // SAFETY: `file` is borrowed for the whole call, so the descriptor is open,
    // and `fsync` reads nothing but that integer.
    if unsafe { libc::fsync(file.as_raw_fd()) } == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

/// Ask for the rename in `dir` to survive a crash. By now the file is in place,
/// so being unable to *ask* is no failure of the save: a folder macOS refuses
/// to open (a File Provider drive the writer denied) or a file system with no
/// directory sync (WebDAV, some network mounts) changes nothing for what was
/// written, and calling it failed would leave the project marked unsaved and the
/// writer unable to quit. A real failure of the sync is still reported.
///
/// Directory sync is a POSIX-only concept (Windows has no directory handle to
/// sync), so there the rename is atomic but not additionally journaled by us.
#[cfg(unix)]
fn sync_directory(dir: &Path) -> Result<()> {
    let Ok(handle) = File::open(dir) else {
        return Ok(());
    };
    match sync_file(&handle) {
        Err(e) if !sync_not_implemented(&e) => {
            anyhow::bail!("fsyncing directory {}: {e}", dir.display())
        }
        _ => Ok(()),
    }
}

/// Verify that the bundle at `path` really is a backup, and (unless
/// `expect_unique_id` is empty) that it backs up the project with that
/// `unique_id`.
///
/// A backup nobody has ever read back is a hypothesis, not a backup: this is
/// the cheap verify-after-write check a backup routine should run immediately
/// after writing, so a corrupt/truncated/wrongly-tagged file is caught the
/// moment it's produced rather than the moment someone needs it. It uses the
/// existing [`super::peek_manifest`] (reads only the manifest entry — no full
/// extract), so it stays cheap enough to run on every backup.
///
/// Errors, with a clear message, when: the manifest can't be read or parsed,
/// `kind != BundleKind::Backup`, or (when `expect_unique_id` is non-empty)
/// `work.unique_id != expect_unique_id`.
pub fn verify_backup_at(path: &str, expect_unique_id: &str) -> Result<()> {
    let manifest = super::peek_manifest(path)
        .with_context(|| format!("verifying backup at '{path}': manifest unreadable"))?;

    if manifest.kind != super::BundleKind::Backup {
        anyhow::bail!(
            "verifying backup at '{path}': expected kind Backup, found {:?}",
            manifest.kind
        );
    }

    if !expect_unique_id.is_empty() && manifest.work.unique_id != expect_unique_id {
        anyhow::bail!(
            "verifying backup at '{path}': unique_id mismatch (expected '{expect_unique_id}', found '{}')",
            manifest.work.unique_id
        );
    }

    Ok(())
}

/// Mark an *existing* bundle at `path` as a backup of `backup_of`, in place.
///
/// The restore flow's "safety copy" (the copy of the current project taken
/// before overwriting it with the restored one) is a raw byte copy of the
/// original file, so as written it still carries `kind: Regular` — meaning
/// retention never prunes it and the backups list never shows it: every
/// restore would otherwise leave a permanent, invisible orphan on disk.
///
/// This does a read-modify-write: [`super::read_bundle`] the bundle,
/// apply the same marking [`super::mark_as_backup`] uses, then
/// [`write_bundle`] it back in its original physical shape
/// ([`super::detect_shape`]). Restores are rare, so the cost of a full
/// read+rewrite (rather than a cheap manifest-only patch) is acceptable, and
/// it reuses the exact same marking logic as a freshly-taken backup instead of
/// duplicating it.
pub fn mark_existing_as_backup(path: &str, backup_of: &str, when: DateTime<Utc>) -> Result<()> {
    let shape = super::detect_shape(path)
        .with_context(|| format!("detecting shape of '{path}' to mark it as a backup"))?;
    let mut bundle = super::read_bundle(path)
        .with_context(|| format!("reading '{path}' to mark it as a backup"))?;
    super::mark_as_backup(&mut bundle, backup_of.to_string(), when);
    write_bundle(path, shape, &bundle)
        .with_context(|| format!("writing '{path}' back as a marked backup"))
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::os::unix::fs::PermissionsExt;

    fn os(code: i32) -> io::Result<()> {
        Err(io::Error::from_raw_os_error(code))
    }

    #[test]
    fn a_file_system_without_the_full_sync_gets_a_plain_one() {
        for code in [
            libc::ENOTTY,
            libc::ENOTSUP,
            libc::EOPNOTSUPP,
            libc::ENOSYS,
            libc::EINVAL,
        ] {
            let plain = Cell::new(false);
            let got = sync_with_fallback(
                || os(code),
                || {
                    plain.set(true);
                    Ok(())
                },
            );
            assert!(got.is_ok(), "errno {code}: {got:?}");
            assert!(plain.get(), "errno {code} never reached the plain sync");
        }
    }

    #[test]
    fn a_real_sync_failure_is_never_retried_or_swallowed() {
        for code in [libc::EIO, libc::ENOSPC, libc::EDQUOT, libc::EROFS] {
            let got = sync_with_fallback(|| os(code), || panic!("errno {code} was retried"));
            assert_eq!(got.err().and_then(|e| e.raw_os_error()), Some(code));
        }
    }

    #[test]
    fn a_plain_sync_that_fails_is_the_answer() {
        let got = sync_with_fallback(|| os(libc::ENOTTY), || os(libc::EIO));
        assert_eq!(got.err().and_then(|e| e.raw_os_error()), Some(libc::EIO));
    }

    /// The File Provider case: the rename landed, then the folder cannot be
    /// opened for the directory sync. The save must be reported as what it was.
    #[test]
    fn a_save_that_landed_is_not_failed_by_a_folder_that_cannot_be_opened() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("Novel.skrib");
        let mut tmp = NamedTempFile::new_in(dir.path()).unwrap();
        io::Write::write_all(&mut tmp, b"the edit").unwrap();

        // Write and enter, but not read: `open` on the folder is refused, while
        // the rename inside it still works. Running as root the folder opens,
        // and this then proves less but still holds.
        let set = |mode| {
            std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(mode)).unwrap()
        };
        set(0o300);
        let got = persist_durably(tmp, &target);
        set(0o700);

        assert!(got.is_ok(), "{got:?}");
        assert_eq!(std::fs::read(&target).unwrap(), b"the edit");
    }

    /// A failure must not leave the temp file beside the project.
    #[test]
    fn a_failed_persist_leaves_no_temp_file_behind() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("taken");
        std::fs::create_dir(&target).unwrap();
        std::fs::write(target.join("keeps it non-empty"), b"x").unwrap();
        let tmp = NamedTempFile::new_in(dir.path()).unwrap();

        let err = persist_durably(tmp, &target).unwrap_err().to_string();

        assert!(err.contains("persisting"), "{err}");
        let left: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| Some(e.ok()?.file_name().to_str()?.to_string()))
            .collect();
        assert_eq!(left, ["taken"]);
    }
}
