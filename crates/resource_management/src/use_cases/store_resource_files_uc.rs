// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: copy the files a writer attaches into the open project's
// resources directory, and say what each one is.
//
// A long operation because a file can be an hour of audio or a scanned book, and
// copying it must not freeze the window or be impossible to stop. It touches no entity:
// Qleany ignores `undoable` on a long operation, so the rows are `add_resources`'s, run
// once this has finished. What ties the two together is the uid: each file is stored in
// `<dest_dir>/<uid>/<name>` (see `skrib_format::media::resource_file`), and the same uid
// becomes its row's.
//
// The copy is streamed and hashed as it goes, never read whole, and lands under a
// temporary name renamed into place, so a cancelled or failed copy never leaves half a
// file where a resource would look for one. Cancelling removes every folder this run
// made.
use crate::StoreResourceFilesDto;
use crate::StoredResourceFilesDto;
use anyhow::{Context, Result, anyhow};
use common::long_operation::{LongOperation, OperationProgress};
use common::types::EntityId;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub trait StoreResourceFilesUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn StoreResourceFilesUnitOfWorkTrait>;
}

/// No store access at all: the unit of work only announces that the run finished.
pub trait StoreResourceFilesUnitOfWorkTrait: Send + Sync {
    fn publish_store_resource_files_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct StoreResourceFilesUseCase {
    uow_factory: Box<dyn StoreResourceFilesUnitOfWorkFactoryTrait>,
    dto: StoreResourceFilesDto,
}

impl StoreResourceFilesUseCase {
    pub fn new(
        uow_factory: Box<dyn StoreResourceFilesUnitOfWorkFactoryTrait>,
        dto: &StoreResourceFilesDto,
    ) -> Self {
        StoreResourceFilesUseCase {
            uow_factory,
            dto: dto.clone(),
        }
    }
}

/// What one stored file turned out to be.
struct Stored {
    uid: uuid::Uuid,
    file_name: String,
    mime_type: String,
    byte_size: u64,
    content_hash: String,
    width: u64,
    height: u64,
}

impl LongOperation for StoreResourceFilesUseCase {
    type Output = StoredResourceFilesDto;

    fn execute(
        &self,
        progress_callback: Box<dyn Fn(OperationProgress) + Send>,
        cancel_flag: Arc<AtomicBool>,
    ) -> Result<Self::Output> {
        let dest_dir = PathBuf::from(&self.dto.dest_dir);
        if self.dto.dest_dir.trim().is_empty() {
            return Err(anyhow!("store_resource_files: no resources directory"));
        }
        // A project's `resources/` that is a link is never read back (see
        // `skrib_format::media::passes_through_link`): a file stored through it would
        // land outside the project and be left out of its next save.
        if std::fs::symlink_metadata(&dest_dir).is_ok_and(|m| m.is_symlink()) {
            return Err(anyhow!(
                "store_resource_files: the resources directory {} is a link",
                dest_dir.display()
            ));
        }
        let total = self.dto.source_paths.len().max(1);
        let mut out = StoredResourceFilesDto::default();
        // Every folder this run made, removed again if it is cancelled.
        let mut made: Vec<PathBuf> = Vec::new();

        for (i, source) in self.dto.source_paths.iter().enumerate() {
            if cancel_flag.load(Ordering::Relaxed) {
                for dir in &made {
                    let _ = std::fs::remove_dir_all(dir);
                }
                return Err(anyhow!("operation cancelled"));
            }
            progress_callback(OperationProgress::new(
                100.0 * i as f32 / total as f32,
                // A phase code: this crate has no catalogue; the UI words it.
                Some("resources.storing".to_string()),
            ));
            match store_one(Path::new(source), &dest_dir, &cancel_flag) {
                Ok(stored) => {
                    made.push(dest_dir.join(stored.uid.hyphenated().to_string()));
                    out.source_paths.push(source.clone());
                    out.uids.push(stored.uid.hyphenated().to_string());
                    out.file_names.push(stored.file_name);
                    out.mime_types.push(stored.mime_type);
                    out.byte_sizes.push(stored.byte_size);
                    out.content_hashes.push(stored.content_hash);
                    out.widths.push(stored.width);
                    out.heights.push(stored.height);
                }
                Err(e) => {
                    if cancel_flag.load(Ordering::Relaxed) {
                        for dir in &made {
                            let _ = std::fs::remove_dir_all(dir);
                        }
                        return Err(anyhow!("operation cancelled"));
                    }
                    out.failed_paths.push(source.clone());
                    out.failure_reasons.push(format!("{e:#}"));
                }
            }
        }

        self.uow_factory
            .create()
            .publish_store_resource_files_event(vec![], None);
        progress_callback(OperationProgress::new(
            100.0,
            Some("resources.stored".to_string()),
        ));
        Ok(out)
    }
}

/// Copy `source` into a fresh uid folder under `dest_dir`, hashing it on the way.
fn store_one(source: &Path, dest_dir: &Path, cancel: &AtomicBool) -> Result<Stored> {
    let meta =
        std::fs::metadata(source).with_context(|| format!("reading {}", source.display()))?;
    if !meta.is_file() {
        return Err(anyhow!("{} is not a file", source.display()));
    }
    let file_name = source
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .ok_or_else(|| anyhow!("{} has no file name", source.display()))?;
    let uid = uuid::Uuid::new_v4();
    let dest = skrib_format::media::resource_file(dest_dir, uid, &file_name);
    let folder = dest
        .parent()
        .ok_or_else(|| anyhow!("no folder for {}", dest.display()))?
        .to_path_buf();
    std::fs::create_dir_all(&folder).with_context(|| format!("creating {}", folder.display()))?;

    let copied = copy_hashing(source, &dest, cancel);
    let (byte_size, content_hash) = match copied {
        Ok(done) => done,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&folder);
            return Err(e);
        }
    };
    let mime_type = mime_for(&file_name).to_string();
    let (width, height) = picture_size(&dest, &mime_type, byte_size);
    Ok(Stored {
        uid,
        file_name,
        mime_type,
        byte_size,
        content_hash,
        width,
        height,
    })
}

/// Stream `source` to `dest` through a temporary file beside it, hashing the bytes as
/// they pass. Returns the size and the blake3, hex.
fn copy_hashing(source: &Path, dest: &Path, cancel: &AtomicBool) -> Result<(u64, String)> {
    let folder = dest
        .parent()
        .ok_or_else(|| anyhow!("no folder for {}", dest.display()))?;
    let mut input =
        std::fs::File::open(source).with_context(|| format!("opening {}", source.display()))?;
    let mut tmp = tempfile::Builder::new()
        .prefix(".storing")
        .tempfile_in(folder)
        .with_context(|| format!("temporary file in {}", folder.display()))?;
    let mut hasher = blake3::Hasher::new();
    let mut buf = vec![0u8; 256 * 1024];
    let mut size: u64 = 0;
    loop {
        if cancel.load(Ordering::Relaxed) {
            return Err(anyhow!("operation cancelled"));
        }
        let n = match input.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e).with_context(|| format!("reading {}", source.display())),
        };
        hasher.update(&buf[..n]);
        tmp.write_all(&buf[..n])
            .with_context(|| format!("writing into {}", folder.display()))?;
        size += n as u64;
    }
    tmp.as_file_mut()
        .sync_all()
        .with_context(|| format!("writing into {}", folder.display()))?;
    tmp.persist(dest)
        .map_err(|e| anyhow!("placing {}: {}", dest.display(), e.error))?;
    Ok((size, hasher.finalize().to_hex().to_string()))
}

/// A stored picture's size, read from its header, for a grid to lay it out before a
/// thumbnail exists. 0 by 0 for anything else, and for a picture the editor could not
/// show: its tile then takes a file-type icon, like any other file.
fn picture_size(path: &Path, mime_type: &str, byte_size: u64) -> (u64, u64) {
    if !matches!(mime_type, "image/png" | "image/jpeg" | "image/webp")
        || byte_size > document_ingest::pictures::MAX_PICTURE_BYTES
    {
        return (0, 0);
    }
    // The header first: a picture says its size in its opening bytes (a JPEG's frame
    // header and orientation sit before its image data), so the whole file is read
    // only when its head is not enough.
    let head = read_head(path, PICTURE_HEAD_BYTES);
    head.and_then(|bytes| document_ingest::pictures::displayable_size(&bytes).ok())
        .or_else(|| {
            std::fs::read(path)
                .ok()
                .and_then(|bytes| document_ingest::pictures::displayable_size(&bytes).ok())
        })
        .map_or((0, 0), |(w, h)| (u64::from(w), u64::from(h)))
}

/// How much of a picture is read for its size before the whole of it is.
const PICTURE_HEAD_BYTES: u64 = 512 * 1024;

/// The first `limit` bytes of the file at `path`, or all of it if it is smaller.
fn read_head(path: &Path, limit: u64) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    std::fs::File::open(path)
        .ok()?
        .take(limit)
        .read_to_end(&mut out)
        .ok()?;
    Some(out)
}

/// The media type of a file, from its extension: the one table every way of storing a
/// resource shares (see `skrib_format::media::mime_for`).
pub use skrib_format::media::mime_for;

#[cfg(test)]
mod tests {
    use super::*;

    struct NoEvents;
    impl StoreResourceFilesUnitOfWorkTrait for NoEvents {
        fn publish_store_resource_files_event(&self, _ids: Vec<EntityId>, _data: Option<String>) {}
    }
    struct Factory;
    impl StoreResourceFilesUnitOfWorkFactoryTrait for Factory {
        fn create(&self) -> Box<dyn StoreResourceFilesUnitOfWorkTrait> {
            Box::new(NoEvents)
        }
    }

    fn run(dest: &Path, sources: Vec<String>, cancel: bool) -> Result<StoredResourceFilesDto> {
        let uc = StoreResourceFilesUseCase::new(
            Box::new(Factory),
            &StoreResourceFilesDto {
                dest_dir: dest.to_string_lossy().into_owned(),
                source_paths: sources,
            },
        );
        uc.execute(Box::new(|_| {}), Arc::new(AtomicBool::new(cancel)))
    }

    #[test]
    fn a_file_is_stored_in_its_uid_folder_with_its_hash() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("Village map.pdf");
        std::fs::write(&src, b"%PDF-1.7 map").unwrap();
        let dest = dir.path().join("resources");
        let out = run(&dest, vec![src.to_string_lossy().into_owned()], false).unwrap();

        assert_eq!(out.uids.len(), 1);
        assert_eq!(out.file_names, vec!["Village map.pdf"]);
        assert_eq!(out.mime_types, vec!["application/pdf"]);
        assert_eq!(out.byte_sizes, vec![12]);
        assert_eq!(
            out.content_hashes,
            vec![blake3::hash(b"%PDF-1.7 map").to_hex().to_string()]
        );
        let uid = uuid::Uuid::parse_str(&out.uids[0]).unwrap();
        let stored = skrib_format::media::resource_file(&dest, uid, "Village map.pdf");
        assert_eq!(std::fs::read(stored).unwrap(), b"%PDF-1.7 map");
        // The source is the writer's and is left as it was.
        assert_eq!(std::fs::read(&src).unwrap(), b"%PDF-1.7 map");
    }

    #[test]
    fn a_file_that_cannot_be_read_is_reported_and_the_others_stored() {
        let dir = tempfile::tempdir().unwrap();
        let good = dir.path().join("notes.txt");
        std::fs::write(&good, b"words").unwrap();
        let missing = dir.path().join("gone.pdf");
        let out = run(
            &dir.path().join("resources"),
            vec![
                missing.to_string_lossy().into_owned(),
                good.to_string_lossy().into_owned(),
            ],
            false,
        )
        .unwrap();
        assert_eq!(out.file_names, vec!["notes.txt"]);
        assert_eq!(
            out.failed_paths,
            vec![missing.to_string_lossy().into_owned()]
        );
        assert_eq!(out.failure_reasons.len(), 1);
    }

    #[test]
    fn a_run_cancelled_after_a_file_was_stored_leaves_nothing_behind() {
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("a.pdf");
        let b = dir.path().join("b.pdf");
        std::fs::write(&a, b"x").unwrap();
        std::fs::write(&b, b"y").unwrap();
        let dest = dir.path().join("resources");
        let cancel = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&cancel);
        let calls = std::sync::atomic::AtomicUsize::new(0);
        let uc = StoreResourceFilesUseCase::new(
            Box::new(Factory),
            &StoreResourceFilesDto {
                dest_dir: dest.to_string_lossy().into_owned(),
                source_paths: vec![
                    a.to_string_lossy().into_owned(),
                    b.to_string_lossy().into_owned(),
                ],
            },
        );
        // Cancelled once the first file is in: the second progress report.
        let progress = Box::new(move |_| {
            if calls.fetch_add(1, Ordering::Relaxed) == 1 {
                flag.store(true, Ordering::Relaxed);
            }
        });
        assert!(uc.execute(progress, cancel).is_err());
        let left = std::fs::read_dir(&dest).map(|d| d.count()).unwrap_or(0);
        assert_eq!(left, 0, "the first file's folder must be removed too");
    }

    #[cfg(unix)]
    #[test]
    fn nothing_is_stored_through_a_linked_resources_directory() {
        let dir = tempfile::tempdir().unwrap();
        let elsewhere = dir.path().join("elsewhere");
        std::fs::create_dir(&elsewhere).unwrap();
        let linked = dir.path().join("resources");
        std::os::unix::fs::symlink(&elsewhere, &linked).unwrap();
        let src = dir.path().join("map.pdf");
        std::fs::write(&src, b"map").unwrap();

        assert!(run(&linked, vec![src.to_string_lossy().into_owned()], false).is_err());
        assert_eq!(
            std::fs::read_dir(&elsewhere).unwrap().count(),
            0,
            "nothing written where the link points"
        );
    }

    #[test]
    fn a_picture_says_its_size() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("dot.png");
        // A real 1×1 PNG (signature, IHDR, one IDAT, IEND), the smallest a decoder takes.
        let png: Vec<u8> = [
            "89504E470D0A1A0A0000000D49484452000000010000000108060000001F15C489",
            "0000000A49444154789C63000100000500010D0A2DB40000000049454E44AE426082",
        ]
        .concat()
        .as_bytes()
        .chunks(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect();
        std::fs::write(&src, &png).unwrap();
        let out = run(
            &dir.path().join("resources"),
            vec![src.to_string_lossy().into_owned()],
            false,
        )
        .unwrap();
        assert_eq!(out.mime_types, vec!["image/png"]);
        assert_eq!((out.widths[0], out.heights[0]), (1, 1));
    }

    #[test]
    fn a_type_comes_from_the_extension_whatever_its_case() {
        assert_eq!(mime_for("Scan.PDF"), "application/pdf");
        assert_eq!(mime_for("talk.mp3"), "audio/mpeg");
        assert_eq!(mime_for("no-extension"), "application/octet-stream");
        assert_eq!(mime_for("evil.desktop"), "application/octet-stream");
    }
}
