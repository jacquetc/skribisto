// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The stored files Skribisto handed to another application, and whether that
//! application changed them.
//!
//! A stored resource opens in the application the desktop uses for its type, and the
//! writer may well edit it there: annotate the map, trim the recording. Where the edit
//! lands depends on the project's shape.
//!
//! - **A folder project**'s file opens in place, in `<project>/resources/<uid>/`. The
//!   edit is already the project's; only its row (size, hash, picture size) is behind.
//! - **A single-file project**'s file lives in a working folder Skribisto prunes, so it
//!   opens as a copy under the app's data, `<data>/open/<uid>/<name>` ([`open_copy_path`]),
//!   one folder per resource so that two resources sharing a name never share a copy, and
//!   under its real name so the other application shows the name the writer knows. An
//!   edit there is not the project's until it is brought back.
//!
//! So every file handed over is remembered here with its size, modification time and
//! hash ([`OpenedFiles::remember`]), and [`OpenedFiles::changed`] reports the ones that
//! no longer match. The size and the time are the cheap test; the hash is read only when
//! one of them moved, and is what decides, so touching a file without changing it reports
//! nothing.
//!
//! The previous bytes of a stored file a copy replaces are kept in the scratch folder
//! until the project closes ([`OpenedFiles::keep_previous`]), which is what the notice's
//! Undo puts back.
//!
//! When the project closes, its copies go ([`OpenedFiles::close`]): the project's own
//! file is the one that counts, and a copy left behind would only fill the data folder.
//! The close first brings in any edit the writer saved since coming back to the window
//! (see `ResourcesSession::pickup_changes`), and a copy that changed after that last
//! look is left where it is rather than lost. What a crash or a forced quit leaves
//! behind, [`sweep_stale_copies`] removes at the next start once nobody has touched it
//! for [`STALE_COPY_AFTER`].
//!
//! Not a view-model: a list of files and the file-system work around it, testable on a
//! temporary folder.

use std::cell::RefCell;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, SystemTime};

/// Where a file handed to another application is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenedWhere {
    /// A copy under the app's data; an edit has to be brought back into the project.
    Copy,
    /// The project's own file; an edit is already in the project.
    InPlace,
}

/// A file's size and modification time: what is compared before anything is read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileStamp {
    pub len: u64,
    pub modified: Option<SystemTime>,
}

impl FileStamp {
    /// The stamp of the file at `path`, `None` when it cannot be read.
    pub fn of(path: &Path) -> Option<Self> {
        let meta = std::fs::metadata(path).ok()?;
        meta.is_file().then(|| Self {
            len: meta.len(),
            modified: meta.modified().ok(),
        })
    }
}

/// One file handed to another application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenedFile {
    pub uid: uuid::Uuid,
    pub path: PathBuf,
    pub place: OpenedWhere,
    pub stamp: FileStamp,
    /// The blake3 of its bytes, hex, as last seen.
    pub hash: String,
}

/// A file another application changed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangedFile {
    pub uid: uuid::Uuid,
    pub path: PathBuf,
    pub place: OpenedWhere,
    pub len: u64,
    pub hash: String,
}

/// The files handed over while this project is open. Cheap to clone; clones share the
/// list. One per open project (it lives on the project's session), so two projects never
/// pick up each other's files.
#[derive(Clone, Default)]
pub struct OpenedFiles {
    inner: Rc<Inner>,
}

#[derive(Default)]
struct Inner {
    files: RefCell<Vec<OpenedFile>>,
    /// Created on first use, under `skrib_format::scratch`, and removed with the last
    /// clone: the previous versions an Undo can put back.
    scratch: RefCell<Option<tempfile::TempDir>>,
}

impl std::fmt::Debug for OpenedFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenedFiles")
            .field("files", &self.inner.files.borrow().len())
            .finish()
    }
}

impl OpenedFiles {
    pub fn new() -> Self {
        Self::default()
    }

    /// Remember that `path`, whose bytes hash to `hash`, was handed to another
    /// application for the resource `uid`. A file already remembered is replaced: the
    /// latest hand-over is the one to compare with.
    pub fn remember(&self, uid: uuid::Uuid, path: &Path, place: OpenedWhere, hash: &str) {
        let Some(stamp) = FileStamp::of(path) else {
            return;
        };
        let mut files = self.inner.files.borrow_mut();
        files.retain(|f| f.path != path);
        files.push(OpenedFile {
            uid,
            path: path.to_path_buf(),
            place,
            stamp,
            hash: hash.to_string(),
        });
    }

    /// Forget every file handed over for `uid`: the resource is gone.
    pub fn forget(&self, uid: uuid::Uuid) {
        self.inner.files.borrow_mut().retain(|f| f.uid != uid);
    }

    /// What is remembered now, for a test.
    pub fn files(&self) -> Vec<OpenedFile> {
        self.inner.files.borrow().clone()
    }

    /// The files whose bytes changed since they were handed over or last reported, and
    /// remember them as they are now, so each change is reported once.
    ///
    /// A file that disappeared is forgotten silently: the other application may have
    /// saved under another name, and the project's own copy is untouched by it.
    pub fn changed(&self) -> Vec<ChangedFile> {
        let mut out = Vec::new();
        let mut files = self.inner.files.borrow_mut();
        files.retain_mut(|file| {
            let Some(now) = FileStamp::of(&file.path) else {
                return false;
            };
            if now == file.stamp {
                return true;
            }
            file.stamp = now;
            match hash_file(&file.path) {
                Ok((len, hash)) if hash != file.hash => {
                    file.hash = hash.clone();
                    out.push(ChangedFile {
                        uid: file.uid,
                        path: file.path.clone(),
                        place: file.place,
                        len,
                        hash,
                    });
                }
                _ => {}
            }
            true
        });
        out
    }

    /// The project closed: remove the copies handed over and forget every file, and drop
    /// the previous versions kept for an Undo, which can no longer be asked for.
    ///
    /// A copy whose bytes changed since the last look is kept, and said on stderr: the
    /// writer saved it after the close took in what had changed, and removing it would
    /// lose that edit for good. [`sweep_stale_copies`] removes it once it is old.
    pub fn close(&self) {
        let files = std::mem::take(&mut *self.inner.files.borrow_mut());
        for file in files {
            if file.place != OpenedWhere::Copy {
                continue;
            }
            if FileStamp::of(&file.path).is_some_and(|now| now != file.stamp)
                && hash_file(&file.path).is_ok_and(|(_, hash)| hash != file.hash)
            {
                eprintln!(
                    "resources: {} changed after the project closed; it is kept",
                    file.path.display()
                );
                continue;
            }
            if let Some(folder) = copy_folder(&file.path) {
                let _ = std::fs::remove_dir_all(folder);
            }
        }
        self.inner.scratch.borrow_mut().take();
    }

    /// Copy `stored`, about to be replaced, into the scratch folder, and say where.
    pub fn keep_previous(&self, uid: uuid::Uuid, stored: &Path) -> std::io::Result<PathBuf> {
        let dir = {
            let mut scratch = self.inner.scratch.borrow_mut();
            if scratch.is_none() {
                *scratch = Some(skrib_format::scratch::tempdir()?);
            }
            match scratch.as_ref() {
                Some(dir) => dir.path().to_path_buf(),
                None => return Err(std::io::Error::other("no scratch folder")),
            }
        };
        let folder = dir.join(uid.hyphenated().to_string());
        std::fs::create_dir_all(&folder)?;
        let n = std::fs::read_dir(&folder)?.count();
        let kept = folder.join(n.to_string());
        std::fs::copy(stored, &kept)?;
        Ok(kept)
    }
}

/// The streamed blake3 of the file at `path`, and its length.
pub fn hash_file(path: &Path) -> std::io::Result<(u64, String)> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buf = vec![0u8; 256 * 1024];
    let mut len = 0u64;
    loop {
        let n = match file.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        hasher.update(&buf[..n]);
        len += n as u64;
    }
    Ok((len, hasher.finalize().to_hex().to_string()))
}

/// Where the copy of a single-file project's stored resource opens: one folder per
/// resource under `<data>/open/`, the file under its own (made-safe) name.
pub fn open_copy_path(data_dir: &Path, uid: uuid::Uuid, file_name: &str) -> PathBuf {
    data_dir
        .join("open")
        .join(uid.hyphenated().to_string())
        .join(skrib_format::slug::safe_file_name(file_name))
}

/// The `<data>/open/<uid>` folder holding the copy at `path`, or `None` for a path that is
/// not one [`open_copy_path`] made: nothing else is ever removed.
fn copy_folder(path: &Path) -> Option<&Path> {
    let folder = path.parent()?;
    let named = folder.file_name()?.to_str()?;
    let open = folder.parent()?;
    (uuid::Uuid::parse_str(named).is_ok() && open.file_name()? == "open").then_some(folder)
}

/// How long a copy nobody touched stays in `<data>/open/` after a close that could not
/// remove it (a crash, a forced quit, a copy edited after the close).
pub const STALE_COPY_AFTER: Duration = Duration::from_secs(30 * 24 * 60 * 60);

/// Remove the copies under `<data_dir>/open/` that nothing changed for `max_age` at `now`.
///
/// Only the per-resource folders [`open_copy_path`] makes, judged by the newest of the
/// folder and what it holds, so a copy edited in place last week is not old. Run once at
/// start, by the first instance only: a project another one has open may still be
/// reading its copies.
pub fn sweep_stale_copies(data_dir: &Path, now: SystemTime, max_age: Duration) {
    let Ok(entries) = std::fs::read_dir(data_dir.join("open")) else {
        return;
    };
    for entry in entries.flatten() {
        let folder = entry.path();
        let is_copy_folder = entry.file_type().is_ok_and(|t| t.is_dir())
            && entry
                .file_name()
                .to_str()
                .is_some_and(|n| uuid::Uuid::parse_str(n).is_ok());
        if !is_copy_folder {
            continue;
        }
        let stale = newest_change(&folder)
            .and_then(|newest| now.duration_since(newest).ok())
            .is_some_and(|age| age > max_age);
        if stale {
            let _ = std::fs::remove_dir_all(&folder);
        }
    }
}

/// The latest modification time of `folder` and the files directly in it; `None` when
/// the folder's own cannot be read.
fn newest_change(folder: &Path) -> Option<SystemTime> {
    let own = std::fs::metadata(folder).ok()?.modified().ok()?;
    let inside = std::fs::read_dir(folder)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| e.metadata().ok()?.modified().ok());
    Some(inside.fold(own, SystemTime::max))
}

/// Write the copy of `stored` that opens for the resource `uid`, replacing an earlier
/// one, and return its path.
///
/// Through a temporary file renamed into place, so an application still holding the
/// previous copy open never reads half of the new one.
pub fn write_open_copy(
    stored: &Path,
    data_dir: &Path,
    uid: uuid::Uuid,
    file_name: &str,
) -> std::io::Result<PathBuf> {
    let target = open_copy_path(data_dir, uid, file_name);
    let folder = target
        .parent()
        .ok_or_else(|| std::io::Error::other("no folder for the copy"))?;
    std::fs::create_dir_all(folder)?;
    replace_file(stored, &target)?;
    Ok(target)
}

/// Replace `target` with a copy of `source`, through a temporary file beside `target`.
pub fn replace_file(source: &Path, target: &Path) -> std::io::Result<()> {
    let folder = target
        .parent()
        .ok_or_else(|| std::io::Error::other("no folder for the file"))?;
    std::fs::create_dir_all(folder)?;
    let tmp = tempfile::Builder::new()
        .prefix(".replacing")
        .tempfile_in(folder)?;
    std::fs::copy(source, tmp.path())?;
    tmp.persist(target).map_err(|e| e.error)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uid(n: u128) -> uuid::Uuid {
        uuid::Uuid::from_u128(n)
    }

    /// Set a file's modification time a little later, so a test does not depend on how
    /// fine the file system's clock is.
    fn touch_later(path: &Path) {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .open(path)
            .expect("open");
        let later = SystemTime::now() + std::time::Duration::from_secs(5);
        file.set_modified(later).expect("set modified");
    }

    #[test]
    fn two_resources_with_one_name_never_share_a_copy() {
        let data = Path::new("/data");
        let a = open_copy_path(data, uid(1), "map.pdf");
        let b = open_copy_path(data, uid(2), "map.pdf");
        assert_ne!(a, b);
        assert_eq!(a.file_name().and_then(|n| n.to_str()), Some("map.pdf"));
        assert!(a.starts_with("/data/open"));
    }

    #[test]
    fn a_copy_keeps_its_real_name_and_its_bytes() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"map v1").expect("write");
        let copy = write_open_copy(&stored, dir.path(), uid(7), "Village map.pdf").expect("copy");
        assert_eq!(
            copy.file_name().and_then(|n| n.to_str()),
            Some("Village map.pdf")
        );
        assert_eq!(std::fs::read(&copy).expect("read"), b"map v1");
        // Opening again replaces it rather than failing or making a second one.
        std::fs::write(&stored, b"map v2").expect("write");
        let again = write_open_copy(&stored, dir.path(), uid(7), "Village map.pdf").expect("copy");
        assert_eq!(again, copy);
        assert_eq!(std::fs::read(&copy).expect("read"), b"map v2");
    }

    #[test]
    fn an_untouched_file_reports_nothing() {
        let dir = tempfile::tempdir().expect("tmp");
        let path = dir.path().join("map.pdf");
        std::fs::write(&path, b"map").expect("write");
        let (_, hash) = hash_file(&path).expect("hash");
        let opened = OpenedFiles::new();
        opened.remember(uid(1), &path, OpenedWhere::Copy, &hash);
        assert!(opened.changed().is_empty());
    }

    #[test]
    fn a_touched_file_with_the_same_bytes_reports_nothing() {
        let dir = tempfile::tempdir().expect("tmp");
        let path = dir.path().join("map.pdf");
        std::fs::write(&path, b"map").expect("write");
        let (_, hash) = hash_file(&path).expect("hash");
        let opened = OpenedFiles::new();
        opened.remember(uid(1), &path, OpenedWhere::Copy, &hash);
        touch_later(&path);
        assert!(
            opened.changed().is_empty(),
            "the hash decides, not the clock"
        );
    }

    #[test]
    fn an_edited_file_is_reported_once_with_its_new_hash() {
        let dir = tempfile::tempdir().expect("tmp");
        let path = dir.path().join("map.pdf");
        std::fs::write(&path, b"map").expect("write");
        let (_, hash) = hash_file(&path).expect("hash");
        let opened = OpenedFiles::new();
        opened.remember(uid(1), &path, OpenedWhere::InPlace, &hash);

        std::fs::write(&path, b"map, annotated").expect("write");
        touch_later(&path);
        let changed = opened.changed();
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0].uid, uid(1));
        assert_eq!(changed[0].place, OpenedWhere::InPlace);
        assert_eq!(changed[0].len, 14);
        assert_eq!(changed[0].hash, hash_file(&path).expect("hash").1);
        assert_ne!(changed[0].hash, hash);

        assert!(
            opened.changed().is_empty(),
            "reported once, not every focus"
        );
    }

    #[test]
    fn a_vanished_file_is_forgotten() {
        let dir = tempfile::tempdir().expect("tmp");
        let path = dir.path().join("map.pdf");
        std::fs::write(&path, b"map").expect("write");
        let opened = OpenedFiles::new();
        opened.remember(uid(1), &path, OpenedWhere::Copy, "x");
        std::fs::remove_file(&path).expect("remove");
        assert!(opened.changed().is_empty());
        assert!(opened.files().is_empty());
    }

    #[test]
    fn closing_removes_the_copies_and_nothing_else() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"map").expect("write");
        let copy = write_open_copy(&stored, dir.path(), uid(4), "map.pdf").expect("copy");
        let (_, hash) = hash_file(&copy).expect("hash");
        let opened = OpenedFiles::new();
        opened.remember(uid(4), &copy, OpenedWhere::Copy, &hash);
        // A folder project's file opens in place: the project's own, never removed.
        let in_place = dir.path().join("in place.pdf");
        std::fs::write(&in_place, b"plan").expect("write");
        opened.remember(uid(5), &in_place, OpenedWhere::InPlace, "x");

        opened.close();
        assert!(!copy.exists(), "the copy is gone");
        assert!(
            !dir.path().join("open").join(uid(4).to_string()).exists(),
            "with its folder"
        );
        assert!(dir.path().join("open").is_dir(), "the open folder stays");
        assert!(in_place.exists(), "a file opened in place is the project's");
        assert!(stored.exists());
        assert!(opened.files().is_empty(), "nothing is remembered any more");
    }

    #[test]
    fn closing_keeps_a_copy_edited_since_the_last_look() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"map").expect("write");
        let copy = write_open_copy(&stored, dir.path(), uid(6), "map.pdf").expect("copy");
        let (_, hash) = hash_file(&copy).expect("hash");
        let opened = OpenedFiles::new();
        opened.remember(uid(6), &copy, OpenedWhere::Copy, &hash);
        std::fs::write(&copy, b"map, annotated").expect("write");
        touch_later(&copy);

        opened.close();
        assert_eq!(
            std::fs::read(&copy).expect("still there"),
            b"map, annotated"
        );
        assert!(opened.files().is_empty());
    }

    #[test]
    fn only_a_copy_path_names_a_folder_to_remove() {
        let data = Path::new("/data");
        let copy = open_copy_path(data, uid(1), "map.pdf");
        assert_eq!(copy_folder(&copy), copy.parent());
        assert_eq!(copy_folder(Path::new("/home/me/research/map.pdf")), None);
        assert_eq!(
            copy_folder(
                &Path::new("/data/elsewhere")
                    .join(uid(1).to_string())
                    .join("a")
            ),
            None
        );
    }

    #[test]
    fn the_sweep_removes_old_copies_only() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"map").expect("write");
        let old = write_open_copy(&stored, dir.path(), uid(8), "old.pdf").expect("copy");
        let fresh = write_open_copy(&stored, dir.path(), uid(9), "fresh.pdf").expect("copy");
        let other = dir.path().join("open").join("not a copy");
        std::fs::create_dir_all(&other).expect("mkdir");

        // A month and a day later, the fresh copy was edited yesterday.
        let later = SystemTime::now() + STALE_COPY_AFTER + Duration::from_secs(24 * 60 * 60);
        std::fs::OpenOptions::new()
            .write(true)
            .open(&fresh)
            .expect("open")
            .set_modified(later - Duration::from_secs(60 * 60))
            .expect("set modified");
        sweep_stale_copies(dir.path(), later, STALE_COPY_AFTER);

        assert!(!old.exists(), "untouched for a month");
        assert!(fresh.exists(), "edited in place an hour ago");
        assert!(other.is_dir(), "not a folder the copies made");
    }

    #[test]
    fn the_previous_bytes_are_kept_for_an_undo() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("map.pdf");
        std::fs::write(&stored, b"before").expect("write");
        let opened = OpenedFiles::new();
        let kept = opened.keep_previous(uid(3), &stored).expect("keep");
        std::fs::write(&stored, b"after").expect("write");
        replace_file(&kept, &stored).expect("put back");
        assert_eq!(std::fs::read(&stored).expect("read"), b"before");
        // A second change keeps its own previous version beside the first.
        let second = opened.keep_previous(uid(3), &stored).expect("keep");
        assert_ne!(kept, second);
    }
}
