//! Dispatch a write to the right physical shape.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
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

/// Persist a [`NamedTempFile`] over `target` **durably**: `sync_all()` the temp
/// file's own contents (propagating any error — never swallow a failed fsync),
/// atomically rename it over `target` via [`NamedTempFile::persist`], then
/// fsync the *parent directory* so the rename entry itself survives a crash.
///
/// `tempfile::persist` gives atomicity only: its own docs say "neither the
/// file contents nor the containing directory are synchronized". Without this
/// a power cut can leave a valid-looking but empty/truncated backup on disk —
/// one that retention would happily count as a real, restorable backup.
///
/// Directory fsync is a POSIX-only concept (Windows has no directory handle to
/// sync), so that step is `#[cfg(unix)]` and silently a no-op elsewhere — the
/// rename is still atomic there, just not additionally journaled by us.
pub(crate) fn persist_durably(tmp: NamedTempFile, target: &Path) -> Result<()> {
    tmp.as_file()
        .sync_all()
        .with_context(|| format!("fsyncing temp file for {}", target.display()))?;

    tmp.persist(target)
        .map_err(|e| anyhow::anyhow!("persisting {}: {}", target.display(), e))?;

    #[cfg(unix)]
    {
        if let Some(parent) = target.parent().filter(|p| !p.as_os_str().is_empty()) {
            let dir = std::fs::File::open(parent)
                .with_context(|| format!("opening {} for fsync", parent.display()))?;
            dir.sync_all()
                .with_context(|| format!("fsyncing directory {}", parent.display()))?;
        }
    }

    Ok(())
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
