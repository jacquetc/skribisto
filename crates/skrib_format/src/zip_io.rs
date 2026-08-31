// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Zip read/write. The zip is a single binary file, so it is always rewritten
//! whole (diff-minimal only matters for the exploded folder under git). Open
//! extracts to a tempdir and reuses the folder reader; save writes the folder
//! form into a tempdir, packs it deterministically, and atomically renames the
//! finished archive over the target.

use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

use super::bundle::WorkBundle;
use super::folder_io::{read_folder, write_folder};
use super::writer::persist_durably;

pub fn write_zip(target: &Path, bundle: &WorkBundle) -> Result<()> {
    let staging = tempfile::tempdir().context("creating staging dir for zip save")?;
    write_folder(staging.path(), bundle)?;

    let parent = target.parent().filter(|p| !p.as_os_str().is_empty());
    if let Some(p) = parent {
        std::fs::create_dir_all(p).with_context(|| format!("creating {}", p.display()))?;
    }
    let mut tmp = match parent {
        Some(p) => NamedTempFile::new_in(p),
        None => NamedTempFile::new(),
    }
    .context("temp file for zip")?;

    // Write through the `NamedTempFile`'s own fd (not a second, independent
    // `File::create` on the same path) so the bytes we fsync in
    // `persist_durably` are demonstrably the bytes we just wrote.
    zip_dir(staging.path(), tmp.as_file_mut())?;
    persist_durably(tmp, target)
}

/// Most entries a real project has: prose, sidecars, assets, history blobs.
///
/// A 300-scene novel with a decade of history and a few hundred images lands in
/// the low thousands, so this is roughly two orders of magnitude of headroom —
/// large enough that no writer meets it, small enough that a file claiming a
/// million entries is refused before any of them is created.
const MAX_ENTRIES: usize = 200_000;

/// Ceiling on the total *uncompressed* bytes an extract will write.
///
/// Generous on purpose: an illustrated project is legitimately hundreds of
/// megabytes, and refusing to open a real book is a worse failure than a slow
/// one. The ratio guard below is what actually stops a bomb; this is the
/// backstop for the shape the ratio guard cannot see — a merely enormous file.
const MAX_TOTAL_BYTES: u64 = 8 << 30;

/// Above [`RATIO_FLOOR_BYTES`], refuse an archive that has expanded more than
/// this many times over the bytes consumed to produce it.
///
/// A zip bomb's whole trick is a ratio in the thousands. Real bundle content
/// does not come close: Djot prose deflates around 3-4x, RON manifests rather
/// more, and images are written `Stored` (ratio 1) by `zip_dir` precisely
/// because they are already compressed.
const MAX_RATIO: u64 = 200;

/// Below this much written, the ratio is not consulted — a few small, highly
/// compressible manifests can legitimately show a large ratio, and refusing a
/// 2 KB project would be absurd.
const RATIO_FLOOR_BYTES: u64 = 64 << 20;

pub fn read_zip(path: &Path) -> Result<WorkBundle> {
    let dir = tempfile::tempdir().context("creating extract dir")?;
    let file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut archive =
        zip::ZipArchive::new(file).with_context(|| format!("reading zip {}", path.display()))?;
    extract_guarded(&mut archive, dir.path())
        .with_context(|| format!("extracting zip {}", path.display()))?;
    read_folder(dir.path())
}

/// Extract every entry of `archive` under `dest`, refusing anything a `.skrib`
/// has no business containing.
///
/// This replaces `ZipArchive::extract`, which is written for archives in general
/// and therefore does two things a project bundle must not inherit. It **creates
/// symlinks with unvalidated targets** — containment is only checked when some
/// *later* entry traverses one, so an archive can plant a link and a subsequent
/// read through it escapes; and it has **no ceiling of any kind**, so a
/// decompression bomb is bounded only by the disk.
///
/// A `.skrib` is written by [`zip_dir`], which emits directories and regular
/// files and nothing else, so refusing every other entry kind costs no
/// legitimate bundle anything — including one written by the Plume or Manuskript
/// importers, which build a `WorkBundle` and hand it to the same writer.
fn extract_guarded<R: std::io::Read + std::io::Seek>(
    archive: &mut zip::ZipArchive<R>,
    dest: &Path,
) -> Result<()> {
    use std::io::Read;

    if archive.len() > MAX_ENTRIES {
        anyhow::bail!(
            "archive declares {} entries, more than the {MAX_ENTRIES} a project may hold",
            archive.len()
        );
    }

    let mut written: u64 = 0;
    let mut compressed: u64 = 0;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;

        // `enclosed_name` is the zip crate's own "is this name safe to join"
        // predicate; `bundle_relative` is this crate's, and is stricter (it also
        // refuses backslashes and colons, which matter for a bundle that must
        // mean the same tree on Windows and Unix). Requiring both is deliberate:
        // they disagree only on names no writer here produces.
        let raw = entry.name().to_string();
        let is_dir = entry.is_dir();
        let name = raw.strip_suffix('/').unwrap_or(&raw);

        if entry.enclosed_name().is_none() {
            anyhow::bail!("entry '{}' escapes the archive root", raw.escape_debug());
        }
        let rel = crate::safe_path::bundle_relative(name)?;
        let target = dest.join(&rel);

        if is_dir {
            std::fs::create_dir_all(&target)
                .with_context(|| format!("creating {}", target.display()))?;
            continue;
        }

        // A symlink entry carries its target as the entry's *contents*, so it is
        // indistinguishable from a regular file except by mode bits. Refuse it
        // outright rather than resolving it: nothing here writes one, and a link
        // is the one entry kind whose meaning depends on the filesystem it lands
        // on rather than on the archive.
        if let Some(mode) = entry.unix_mode()
            && mode & 0xF000 == 0xA000
        {
            anyhow::bail!("entry '{}' is a symbolic link", raw.escape_debug());
        }

        let declared = entry.size();
        if written.saturating_add(declared) > MAX_TOTAL_BYTES {
            anyhow::bail!("archive expands past the {MAX_TOTAL_BYTES} byte ceiling");
        }

        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }

        // The budget is bounded **per entry**, not only in total, and by this
        // entry's own compressed size rather than by what is left of the
        // ceiling. A between-entries ratio check cannot see a bomb that is one
        // entry: a single ~8 MB deflate stream expanding to 8 GiB would be
        // written out in full — into `$TMPDIR`, which is commonly tmpfs, i.e.
        // RAM — and only refused afterwards.
        //
        // `take` is what actually bounds it: `entry.size()` is a claim by the
        // archive, so it decides nothing on its own.
        let entry_budget = RATIO_FLOOR_BYTES
            .max(entry.compressed_size().saturating_mul(MAX_RATIO))
            .min(MAX_TOTAL_BYTES - written);
        let mut out =
            File::create(&target).with_context(|| format!("creating {}", target.display()))?;
        let copied = std::io::copy(&mut (&mut entry).take(entry_budget + 1), &mut out)
            .with_context(|| format!("writing {}", target.display()))?;
        if copied > entry_budget {
            // Remove the partial file rather than leaving it for `read_folder`
            // to meet as a truncated manifest.
            let _ = std::fs::remove_file(&target);
            anyhow::bail!(
                "entry '{}' expands past the {MAX_RATIO}x limit on its compressed size",
                raw.escape_debug()
            );
        }

        written += copied;
        compressed += entry.compressed_size();

        if written > RATIO_FLOOR_BYTES && written > compressed.max(1).saturating_mul(MAX_RATIO) {
            anyhow::bail!(
                "archive expands {}x over its compressed size, past the {MAX_RATIO}x limit",
                written / compressed.max(1)
            );
        }
    }

    Ok(())
}

/// Pack `src_dir`'s tree into a fresh zip, written through `target` (an
/// already-open file — typically a [`NamedTempFile`]'s own fd, so the bytes
/// fsynced afterwards are demonstrably the bytes just written, rather than
/// through a second independent handle on the same path).
pub fn zip_dir(src_dir: &Path, target: &mut File) -> Result<()> {
    let mut zw = zip::ZipWriter::new(target);
    let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    // Already-compressed payloads are stored, not deflated again.
    //
    // A JPEG, PNG or WebP is entropy-coded already: running deflate over one
    // spends real CPU to gain a fraction of a percent, occasionally growing the
    // entry. That cost lands on the autosave path, which rebuilds the whole
    // archive every few seconds, so for a project with photographs in it this is
    // the difference between an autosave that is proportional to the prose and
    // one that is proportional to the pictures.
    let stored = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for entry in WalkDir::new(src_dir).sort_by_file_name() {
        let entry = entry.context("walking staging dir")?;
        let path = entry.path();
        let rel = path
            .strip_prefix(src_dir)
            .context("stripping staging prefix")?;
        if rel.as_os_str().is_empty() {
            continue;
        }
        let name = rel
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("non-UTF8 path {}", rel.display()))?
            .replace('\\', "/");
        if entry.file_type().is_dir() {
            zw.add_directory(format!("{name}/"), opts)?;
        } else {
            let already_compressed = matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("png" | "jpg" | "jpeg" | "webp" | "gif")
            );
            zw.start_file(name, if already_compressed { stored } else { opts })?;
            let bytes =
                std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
            zw.write_all(&bytes)?;
        }
    }
    zw.finish()?;
    Ok(())
}
