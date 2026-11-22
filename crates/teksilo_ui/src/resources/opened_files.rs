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
//!   opens as a copy under the app's data, `<data>/open/<key>/<uid>/<name>`
//!   ([`open_copy_path`]). An edit there is not the project's until it is brought back.
//!   The `<key>` is the project's: the name of its working folder, its `unique_id` and a
//!   hash of its path (see `skrib_format::media::resources_dir`). A Save As, a backup
//!   and a file-manager copy keep the resource uids, so two projects open at once may
//!   well hold the same `<uid>`, and keyed by the uid alone they shared one copy: opening
//!   it from one overwrote what the writer had annotated from the other, and closing one
//!   deleted the file the other had handed over. Under the key, one folder per resource
//!   keeps two resources sharing a name apart, and the real name lets the other
//!   application show the name the writer knows.
//!
//! So every file handed over is remembered here with what the project holds for it: the
//! hash of those bytes, and the size and modification time at which the file last had
//! them ([`OpenedFiles::remember`]). [`OpenedFiles::changed`] reports the ones that no
//! longer match. The size and the time are the cheap test; the hash is read only when
//! one of them moved, and is what decides, so touching a file without changing it
//! reports nothing.
//!
//! A change reported is not yet seen. It counts as seen only once the project has taken
//! it in, which the caller says with [`OpenedFiles::settle`]. A change the caller could
//! not take in (the stored file could not be replaced, the row could not be written), or
//! a file that could not be read at all (another application holding it on Windows), is
//! reported again at the next look.
//!
//! The previous bytes of a stored file a copy replaces are kept in the scratch folder
//! until the project closes ([`OpenedFiles::keep_previous`]), which is what the notice's
//! Undo puts back. The Undo writes the restored bytes back into the copy as well and
//! settles them ([`OpenedFiles::rewrite_copies`]), so the other application's next save
//! is a new change rather than the one just undone.
//!
//! When the project closes, its copies go ([`OpenedFiles::close`]): the project's own
//! file is the one that counts, and a copy left behind would only fill the data folder.
//! The close first brings in any edit the writer saved since coming back to the window
//! (see `ResourcesSession::pickup_changes`). Only the copy itself is removed, and only
//! when its bytes are the ones the project holds: a copy changed after that last look,
//! one whose change could not be taken in and one that cannot be read are all left where
//! they are, and so is anything else the other application saved beside it. A resource
//! removed from the project loses its copies the same way, at once
//! ([`OpenedFiles::forget`]), rather than leaving them for the sweep.
//!
//! A copy left that way is not lost the next time the same project opens the resource:
//! [`write_open_copy`] finds an earlier copy whose bytes differ from the stored file,
//! leaves it as it is ([`OpenCopy::Pending`]), and the project takes it in at once, with
//! the usual notice and its Undo (see `ResourcesSession::open_copy`). What a crash or a
//! forced quit leaves behind, and a copy no open ever came back for,
//! [`sweep_stale_copies`] removes at the next start once nobody has touched it for
//! [`STALE_COPY_AFTER`].
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
    /// The file's stamp when its bytes were last found to be the ones the project holds;
    /// `None` while they never were, so it is read at every look.
    pub stamp: Option<FileStamp>,
    /// The blake3 of the bytes the project holds for it, hex.
    pub hash: String,
}

/// A file another application changed, not yet taken in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangedFile {
    pub uid: uuid::Uuid,
    pub path: PathBuf,
    pub place: OpenedWhere,
    /// The stamp read before the bytes were: what [`OpenedFiles::settle`] records.
    pub stamp: FileStamp,
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

    /// Remember that `path`, whose bytes hash to `hash`, the bytes the project holds,
    /// was handed to another application for the resource `uid`. A file already
    /// remembered is replaced: the latest hand-over is the one to compare with.
    pub fn remember(&self, uid: uuid::Uuid, path: &Path, place: OpenedWhere, hash: &str) {
        let Some(stamp) = FileStamp::of(path) else {
            return;
        };
        self.put(OpenedFile {
            uid,
            path: path.to_path_buf(),
            place,
            stamp: Some(stamp),
            hash: hash.to_string(),
        });
    }

    /// Remember `path` as handed over for `uid` while its bytes are not the ones the
    /// project holds, which hash to `hash`: the next [`Self::changed`] reports it. For
    /// an earlier copy found holding an edit ([`OpenCopy::Pending`]).
    pub fn remember_unsettled(&self, uid: uuid::Uuid, path: &Path, place: OpenedWhere, hash: &str) {
        self.put(OpenedFile {
            uid,
            path: path.to_path_buf(),
            place,
            stamp: None,
            hash: hash.to_string(),
        });
    }

    fn put(&self, file: OpenedFile) {
        let mut files = self.inner.files.borrow_mut();
        files.retain(|f| f.path != file.path);
        files.push(file);
    }

    /// The resource `uid` was removed from the project: forget every file handed over
    /// for it, and remove its copies as [`Self::close`] does, each only when its bytes
    /// are the ones the project held. One holding an edit stays where it is: the
    /// removal's Undo brings the resource back, and the next open of it from this
    /// project takes that edit in ([`OpenCopy::Pending`]).
    pub fn forget(&self, uid: uuid::Uuid) {
        let gone: Vec<OpenedFile> = {
            let mut files = self.inner.files.borrow_mut();
            let (gone, kept) = std::mem::take(&mut *files)
                .into_iter()
                .partition(|f| f.uid == uid);
            *files = kept;
            gone
        };
        for file in gone {
            remove_copy(&file);
        }
    }

    /// What is remembered now, for a test.
    pub fn files(&self) -> Vec<OpenedFile> {
        self.inner.files.borrow().clone()
    }

    /// The files whose bytes differ from what the project holds for them. Nothing is
    /// marked seen here: a change is reported again at every look until the caller has
    /// taken it in and said so with [`Self::settle`].
    ///
    /// A file touched with its bytes unchanged is settled on the spot, so it is not read
    /// again. One that cannot be read now, or that is not there (the other application
    /// may be in the middle of saving it under a temporary name), stays remembered and
    /// is looked at again next time.
    pub fn changed(&self) -> Vec<ChangedFile> {
        let mut out = Vec::new();
        for file in self.inner.files.borrow_mut().iter_mut() {
            let Some(now) = FileStamp::of(&file.path) else {
                continue;
            };
            if file.stamp == Some(now) {
                continue;
            }
            match hash_file(&file.path) {
                Ok((_, hash)) if hash == file.hash => file.stamp = Some(now),
                Ok((len, hash)) => out.push(ChangedFile {
                    uid: file.uid,
                    path: file.path.clone(),
                    place: file.place,
                    stamp: now,
                    len,
                    hash,
                }),
                Err(e) => {
                    eprintln!("resources: could not read {}: {e}", file.path.display());
                }
            }
        }
        out
    }

    /// The change [`Self::changed`] reported for `path` is now the project's: its bytes,
    /// `hash`, are what the project holds, as of `stamp`.
    pub fn settle(&self, path: &Path, stamp: FileStamp, hash: &str) {
        if let Some(file) = self
            .inner
            .files
            .borrow_mut()
            .iter_mut()
            .find(|f| f.path == path)
        {
            file.stamp = Some(stamp);
            file.hash = hash.to_string();
        }
    }

    /// Write `stored` back into every copy handed over for `uid`, and settle them on its
    /// bytes: an Undo just put them back into the project, and the copy still showed
    /// what was undone. The other application's next save is then a change of its own.
    ///
    /// A copy that cannot be written (held open on Windows) is settled as it is, on the
    /// stored file's hash: the bytes it holds were just undone and are not taken in
    /// again, a later save there is, and the close keeps it, since it differs.
    pub fn rewrite_copies(&self, uid: uuid::Uuid, stored: &Path) -> std::io::Result<()> {
        let (_, hash) = hash_file(stored)?;
        let copies: Vec<PathBuf> = self
            .inner
            .files
            .borrow()
            .iter()
            .filter(|f| f.uid == uid && f.place == OpenedWhere::Copy)
            .map(|f| f.path.clone())
            .collect();
        let mut result = Ok(());
        for copy in copies {
            if let Err(e) = replace_file(stored, &copy) {
                result = Err(e);
            }
            if let Some(stamp) = FileStamp::of(&copy) {
                self.settle(&copy, stamp, &hash);
            }
        }
        result
    }

    /// The project closed: remove the copies handed over and forget every file, and drop
    /// the previous versions kept for an Undo, which can no longer be asked for.
    ///
    /// A copy goes only when its bytes are the ones the project holds, read now rather
    /// than trusted from the last look. One that differs (saved after the close took in
    /// what had changed, or a change that could not be taken in) or cannot be read is
    /// kept, and said on stderr: removing it would lose that edit for good. The next open
    /// of the same resource from the same project takes it in ([`OpenCopy::Pending`]),
    /// and [`sweep_stale_copies`] removes it once it is old. Only the copy is removed,
    /// then its folders if that left them empty, so a file the other application saved
    /// beside it stays.
    pub fn close(&self) {
        let files = std::mem::take(&mut *self.inner.files.borrow_mut());
        for file in files {
            remove_copy(&file);
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

/// Remove `file`, when it is a copy holding the bytes the project holds, then its folders
/// if that left them empty. See [`OpenedFiles::close`].
fn remove_copy(file: &OpenedFile) {
    if file.place != OpenedWhere::Copy {
        return;
    }
    let gone = matches!(
        std::fs::metadata(&file.path),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound
    );
    if !gone {
        let held = hash_file(&file.path).is_ok_and(|(_, hash)| hash == file.hash);
        if !held {
            eprintln!(
                "resources: {} holds changes the project does not; it is kept",
                file.path.display()
            );
            return;
        }
        if let Err(e) = std::fs::remove_file(&file.path) {
            eprintln!("resources: could not remove {}: {e}", file.path.display());
            return;
        }
    }
    if let Some(folder) = copy_folder(&file.path) {
        // Each only when empty: what else is in them is not ours to remove.
        if std::fs::remove_dir(folder).is_ok()
            && let Some(key) = folder.parent()
        {
            let _ = std::fs::remove_dir(key);
        }
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

/// Where the copy of a single-file project's stored resource opens: under the project's
/// `key` (see the module docs), one folder per resource, the file under its own
/// (made-safe) name. `None` for a key that is not a single folder name.
///
/// Laid out as a resources folder is, so a copy an earlier build left under its earlier
/// made-safe name (`skrib_format::slug::legacy_safe_file_name`) is still found.
pub fn open_copy_path(
    data_dir: &Path,
    key: &str,
    uid: uuid::Uuid,
    file_name: &str,
) -> Option<PathBuf> {
    let one_name =
        !key.contains(['/', '\\']) && Path::new(key).file_name().is_some_and(|n| n == key);
    one_name.then(|| {
        skrib_format::media::resource_file(&data_dir.join("open").join(key), uid, file_name)
    })
}

/// The `<data>/open/<key>/<uid>` folder holding the copy at `path`, or `None` for a path
/// that is not one [`open_copy_path`] made: nothing else is ever removed.
fn copy_folder(path: &Path) -> Option<&Path> {
    let folder = path.parent()?;
    let named = folder.file_name()?.to_str()?;
    let open = folder.parent()?.parent()?;
    (uuid::Uuid::parse_str(named).is_ok() && open.file_name()? == "open").then_some(folder)
}

/// Whether `name`, under `<data>/open/`, is a project's key: a zip project's working
/// folder name, `<unique_id>-<16 hex digits of its path's hash>`.
fn is_project_key(name: &str) -> bool {
    name.rsplit_once('-').is_some_and(|(id, place)| {
        !id.is_empty()
            && place.len() == 16
            && place
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}

/// How long a copy nobody touched stays in `<data>/open/` after a close that could not
/// remove it (a crash, a forced quit, a copy edited after the close).
pub const STALE_COPY_AFTER: Duration = Duration::from_secs(30 * 24 * 60 * 60);

/// Remove the copies under `<data_dir>/open/` that nothing changed for `max_age` at `now`.
///
/// Only the per-resource folders [`open_copy_path`] makes, `<key>/<uid>`, and the
/// project's `<key>` folder once that left it empty; also the `<uid>` folders right under
/// `open/` that copies were made in before they were kept per project. Each is judged by
/// the newest of the folder and what it holds, so a copy edited in place last week is
/// not old. Run once at start, by the first instance only: a project another one has
/// open may still be reading its copies.
pub fn sweep_stale_copies(data_dir: &Path, now: SystemTime, max_age: Duration) {
    let Ok(entries) = std::fs::read_dir(data_dir.join("open")) else {
        return;
    };
    for entry in entries.flatten() {
        if !entry.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }
        let folder = entry.path();
        let Some(name) = entry.file_name().to_str().map(str::to_string) else {
            continue;
        };
        if uuid::Uuid::parse_str(&name).is_ok() {
            sweep_copy_folder(&folder, now, max_age);
        } else if is_project_key(&name) {
            for inner in std::fs::read_dir(&folder).into_iter().flatten().flatten() {
                let is_copy_folder = inner.file_type().is_ok_and(|t| t.is_dir())
                    && inner
                        .file_name()
                        .to_str()
                        .is_some_and(|n| uuid::Uuid::parse_str(n).is_ok());
                if is_copy_folder {
                    sweep_copy_folder(&inner.path(), now, max_age);
                }
            }
            // Only when empty: a copy still fresh, or anything else, keeps it.
            let _ = std::fs::remove_dir(&folder);
        }
    }
}

/// Remove the copy folder `folder` if nothing in it changed for `max_age` at `now`.
fn sweep_copy_folder(folder: &Path, now: SystemTime, max_age: Duration) {
    let stale = newest_change(folder)
        .and_then(|newest| now.duration_since(newest).ok())
        .is_some_and(|age| age > max_age);
    if stale {
        let _ = std::fs::remove_dir_all(folder);
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

/// The copy [`write_open_copy`] has ready to hand over.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpenCopy {
    /// A copy with the stored file's bytes, written now or already there.
    Fresh(PathBuf),
    /// An earlier copy whose bytes differ from the stored file: an edit the writer saved
    /// that the project never took in (see the module docs). Left as it is, for the
    /// caller to take in.
    Pending(PathBuf),
}

/// Make the copy of `stored` that opens for the resource `uid` of the project `key`.
///
/// An earlier copy already there with the same bytes is used as it is. One with other
/// bytes is never overwritten: it holds an edit the project does not have, and is
/// returned as [`OpenCopy::Pending`]. Otherwise the copy is written through a temporary
/// file renamed into place, so an application still holding a previous copy open never
/// reads half of the new one.
pub fn write_open_copy(
    stored: &Path,
    data_dir: &Path,
    key: &str,
    uid: uuid::Uuid,
    file_name: &str,
) -> std::io::Result<OpenCopy> {
    let target = open_copy_path(data_dir, key, uid, file_name)
        .ok_or_else(|| std::io::Error::other("no folder for the copy"))?;
    match std::fs::metadata(&target) {
        Ok(_) => {
            let (_, theirs) = hash_file(&target)?;
            let (_, ours) = hash_file(stored)?;
            return Ok(if theirs == ours {
                OpenCopy::Fresh(target)
            } else {
                OpenCopy::Pending(target)
            });
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }
    replace_file(stored, &target)?;
    Ok(OpenCopy::Fresh(target))
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

    /// A project's key, as a zip project's working folder is named.
    const PROJECT: &str = "5e5a7c1e-0000-4000-8000-000000000001-0123456789abcdef";
    /// Another project carrying the same resource uids: its Save As copy.
    const SAVED_AS: &str = "5e5a7c1e-0000-4000-8000-000000000001-fedcba9876543210";

    fn copy_at(stored: &Path, data: &Path, key: &str, n: u128, name: &str) -> PathBuf {
        match write_open_copy(stored, data, key, uid(n), name).expect("copy") {
            OpenCopy::Fresh(path) => path,
            OpenCopy::Pending(path) => {
                panic!("{} was not expected to hold an edit", path.display())
            }
        }
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
        let a = open_copy_path(data, PROJECT, uid(1), "map.pdf").expect("a path");
        let b = open_copy_path(data, PROJECT, uid(2), "map.pdf").expect("a path");
        assert_ne!(a, b);
        assert_eq!(a.file_name().and_then(|n| n.to_str()), Some("map.pdf"));
        assert!(a.starts_with(Path::new("/data/open").join(PROJECT)));
    }

    /// A Save As keeps the resource uids: the copy and its original, open together,
    /// each open the resource through a copy of their own.
    #[test]
    fn a_copy_an_earlier_build_named_otherwise_is_still_found() {
        let dir = tempfile::tempdir().expect("tmp");
        let earlier = dir
            .path()
            .join("open")
            .join("key")
            .join(uid(3).to_string())
            .join("COM¹.pdf");
        std::fs::create_dir_all(earlier.parent().expect("folder")).expect("mkdir");
        std::fs::write(&earlier, b"map, annotated").expect("write");
        assert_eq!(
            open_copy_path(dir.path(), "key", uid(3), "COM¹.pdf"),
            Some(earlier)
        );
        let today = open_copy_path(dir.path(), "key", uid(4), "COM¹.pdf").expect("path");
        assert_eq!(
            today.file_name().and_then(|n| n.to_str()),
            Some("_COM¹.pdf")
        );
    }

    #[test]
    fn two_projects_with_one_resource_never_share_a_copy() {
        let data = Path::new("/data");
        let a = open_copy_path(data, PROJECT, uid(1), "map.pdf").expect("a path");
        let b = open_copy_path(data, SAVED_AS, uid(1), "map.pdf").expect("a path");
        assert_ne!(a.parent(), b.parent());
    }

    #[test]
    fn a_key_is_one_folder_name() {
        let data = Path::new("/data");
        for bad in ["", ".", "..", "a/b", "a\\b", "../escape"] {
            assert_eq!(
                open_copy_path(data, bad, uid(1), "map.pdf"),
                None,
                "{bad:?}"
            );
        }
    }

    #[test]
    fn a_copy_keeps_its_real_name_and_its_bytes() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"map v1").expect("write");
        let copy = copy_at(&stored, dir.path(), PROJECT, 7, "Village map.pdf");
        assert_eq!(
            copy.file_name().and_then(|n| n.to_str()),
            Some("Village map.pdf")
        );
        assert_eq!(std::fs::read(&copy).expect("read"), b"map v1");
        // Opening again finds it rather than failing or making a second one.
        let again = copy_at(&stored, dir.path(), PROJECT, 7, "Village map.pdf");
        assert_eq!(again, copy);
    }

    /// A copy the project did not take in (changed after the close's last look) is not
    /// overwritten by the next open of the same resource: it is handed back as pending.
    #[test]
    fn an_earlier_copy_with_other_bytes_is_never_overwritten() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"map").expect("write");
        let copy = copy_at(&stored, dir.path(), PROJECT, 7, "map.pdf");
        std::fs::write(&copy, b"map, annotated").expect("edit");

        let again = write_open_copy(&stored, dir.path(), PROJECT, uid(7), "map.pdf").expect("copy");
        assert_eq!(again, OpenCopy::Pending(copy.clone()));
        assert_eq!(std::fs::read(&copy).expect("read"), b"map, annotated");
        // Another project with the same resource has its own copy, from its own file.
        let other = copy_at(&stored, dir.path(), SAVED_AS, 7, "map.pdf");
        assert_ne!(other, copy);
        assert_eq!(std::fs::read(&other).expect("read"), b"map");
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
        assert_eq!(
            opened.files()[0].stamp,
            FileStamp::of(&path),
            "and the new time is settled, so it is not read again"
        );
    }

    /// Reported until it is taken in, and not once settled.
    #[test]
    fn an_edited_file_is_reported_until_it_is_settled() {
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

        assert_eq!(
            opened.changed(),
            changed,
            "not taken in yet: reported again at the next look"
        );
        opened.settle(&path, changed[0].stamp, &changed[0].hash);
        assert!(
            opened.changed().is_empty(),
            "taken in: reported once, not every focus"
        );
    }

    /// A file that could not be read (another application holding it) is not marked
    /// seen: it is reported once it can be, and the close keeps it until then.
    #[cfg(unix)]
    #[test]
    fn a_copy_that_cannot_be_read_is_retried_and_kept_at_close() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"map").expect("write");
        let copy = copy_at(&stored, dir.path(), PROJECT, 3, "map.pdf");
        let (_, hash) = hash_file(&copy).expect("hash");
        let opened = OpenedFiles::new();
        opened.remember(uid(3), &copy, OpenedWhere::Copy, &hash);
        std::fs::write(&copy, b"map, annotated").expect("edit");
        touch_later(&copy);

        let set_mode = |mode| {
            std::fs::set_permissions(&copy, std::fs::Permissions::from_mode(mode)).expect("chmod");
        };
        set_mode(0o000);
        if hash_file(&copy).is_ok() {
            // Running as root: permissions do not stop a read, so there is no failure
            // to test.
            set_mode(0o644);
            return;
        }
        assert!(opened.changed().is_empty(), "nothing could be read");
        set_mode(0o644);
        assert_eq!(opened.changed().len(), 1, "read now, so reported now");

        set_mode(0o000);
        opened.close();
        set_mode(0o644);
        assert_eq!(
            std::fs::read(&copy).expect("still there"),
            b"map, annotated"
        );
    }

    /// A removed resource's copy goes with it when it holds what the project held, and
    /// stays when it holds an edit; the project's other copies are not touched.
    #[test]
    fn forgetting_a_resource_removes_its_unedited_copies_only() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"map").expect("write");
        let (_, hash) = hash_file(&stored).expect("hash");
        let removed = copy_at(&stored, dir.path(), PROJECT, 1, "map.pdf");
        let edited = copy_at(&stored, dir.path(), PROJECT, 2, "map.pdf");
        let other = copy_at(&stored, dir.path(), PROJECT, 3, "map.pdf");
        let opened = OpenedFiles::new();
        for (n, copy) in [(1, &removed), (2, &edited), (3, &other)] {
            opened.remember(uid(n), copy, OpenedWhere::Copy, &hash);
        }
        std::fs::write(&edited, b"map, annotated").expect("edit");

        opened.forget(uid(1));
        assert!(!removed.exists(), "its bytes were the project's: it goes");
        assert!(
            removed.parent().is_some_and(|f| !f.exists()),
            "and its folder with it"
        );
        opened.forget(uid(2));
        assert_eq!(
            std::fs::read(&edited).expect("kept"),
            b"map, annotated",
            "an edit is never deleted"
        );
        assert!(other.exists());
        let left: Vec<uuid::Uuid> = opened.files().iter().map(|f| f.uid).collect();
        assert_eq!(left, [uid(3)]);
    }

    #[test]
    fn a_vanished_file_reports_nothing_until_it_is_back() {
        let dir = tempfile::tempdir().expect("tmp");
        let path = dir.path().join("map.pdf");
        std::fs::write(&path, b"map").expect("write");
        let (_, hash) = hash_file(&path).expect("hash");
        let opened = OpenedFiles::new();
        opened.remember(uid(1), &path, OpenedWhere::Copy, &hash);
        // The other application saves through a temporary file renamed over it.
        std::fs::remove_file(&path).expect("remove");
        assert!(opened.changed().is_empty());
        std::fs::write(&path, b"map, annotated").expect("write");
        touch_later(&path);
        assert_eq!(opened.changed().len(), 1, "still remembered");
    }

    #[test]
    fn closing_removes_the_copies_and_nothing_else() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"map").expect("write");
        let copy = copy_at(&stored, dir.path(), PROJECT, 4, "map.pdf");
        let (_, hash) = hash_file(&copy).expect("hash");
        let opened = OpenedFiles::new();
        opened.remember(uid(4), &copy, OpenedWhere::Copy, &hash);
        // A folder project's file opens in place: the project's own, never removed.
        let in_place = dir.path().join("in place.pdf");
        std::fs::write(&in_place, b"plan").expect("write");
        opened.remember(uid(5), &in_place, OpenedWhere::InPlace, "x");
        // The same resource, opened from another project.
        let theirs = copy_at(&stored, dir.path(), SAVED_AS, 4, "map.pdf");

        opened.close();
        assert!(!copy.exists(), "the copy is gone");
        let open = dir.path().join("open");
        assert!(!open.join(PROJECT).exists(), "with its folders");
        assert!(open.is_dir(), "the open folder stays");
        assert!(theirs.exists(), "another project's copy is not this one's");
        assert!(in_place.exists(), "a file opened in place is the project's");
        assert!(stored.exists());
        assert!(opened.files().is_empty(), "nothing is remembered any more");
    }

    /// What the other application saved beside the copy (another format, a second
    /// version) is not the project's to remove.
    #[test]
    fn closing_leaves_what_was_saved_beside_a_copy() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"notes").expect("write");
        let copy = copy_at(&stored, dir.path(), PROJECT, 4, "notes.docx");
        let (_, hash) = hash_file(&copy).expect("hash");
        let beside = copy.with_file_name("notes.odt");
        std::fs::write(&beside, b"notes, kept as odt").expect("write");
        let opened = OpenedFiles::new();
        opened.remember(uid(4), &copy, OpenedWhere::Copy, &hash);

        opened.close();
        assert!(!copy.exists());
        assert_eq!(
            std::fs::read(&beside).expect("still there"),
            b"notes, kept as odt"
        );
    }

    #[test]
    fn closing_keeps_a_copy_edited_since_the_last_look() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"map").expect("write");
        let copy = copy_at(&stored, dir.path(), PROJECT, 6, "map.pdf");
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

    /// The change was reported but never taken in (the caller could not write the
    /// stored file): the close must not take the last look for a settled one.
    #[test]
    fn closing_keeps_a_copy_whose_change_was_reported_but_not_taken_in() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"map").expect("write");
        let copy = copy_at(&stored, dir.path(), PROJECT, 6, "map.pdf");
        let (_, hash) = hash_file(&copy).expect("hash");
        let opened = OpenedFiles::new();
        opened.remember(uid(6), &copy, OpenedWhere::Copy, &hash);
        std::fs::write(&copy, b"map, annotated").expect("write");
        touch_later(&copy);
        assert_eq!(opened.changed().len(), 1);

        opened.close();
        assert_eq!(
            std::fs::read(&copy).expect("still there"),
            b"map, annotated"
        );
    }

    /// Undo of a pickup writes the restored bytes into the copy and settles them, so the
    /// copy no longer shows what was undone, and the next save there is a new change.
    #[test]
    fn rewriting_a_copy_settles_it_on_the_restored_bytes() {
        let dir = tempfile::tempdir().expect("tmp");
        let stored = dir.path().join("stored.bin");
        std::fs::write(&stored, b"map").expect("write");
        let copy = copy_at(&stored, dir.path(), PROJECT, 2, "map.pdf");
        let opened = OpenedFiles::new();
        opened.remember(uid(2), &copy, OpenedWhere::Copy, "the annotated hash");

        opened.rewrite_copies(uid(2), &stored).expect("rewritten");
        assert_eq!(std::fs::read(&copy).expect("read"), b"map");
        assert!(opened.changed().is_empty());
        std::fs::write(&copy, b"map, annotated again").expect("edit");
        touch_later(&copy);
        assert_eq!(opened.changed().len(), 1);
    }

    #[test]
    fn only_a_copy_path_names_a_folder_to_remove() {
        let data = Path::new("/data");
        let copy = open_copy_path(data, PROJECT, uid(1), "map.pdf").expect("a path");
        assert_eq!(copy_folder(&copy), copy.parent());
        assert_eq!(copy_folder(Path::new("/home/me/research/map.pdf")), None);
        assert_eq!(
            copy_folder(
                &Path::new("/data/elsewhere")
                    .join(PROJECT)
                    .join(uid(1).to_string())
                    .join("a")
            ),
            None
        );
        assert_eq!(
            copy_folder(
                &Path::new("/data/open")
                    .join(PROJECT)
                    .join("not a uid")
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
        let old = copy_at(&stored, dir.path(), PROJECT, 8, "old.pdf");
        let fresh = copy_at(&stored, dir.path(), PROJECT, 9, "fresh.pdf");
        let old_alone = copy_at(&stored, dir.path(), SAVED_AS, 8, "old.pdf");
        // Where copies were made before they were kept per project.
        let flat = dir
            .path()
            .join("open")
            .join(uid(10).to_string())
            .join("flat.pdf");
        replace_file(&stored, &flat).expect("flat copy");
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
        assert!(
            !dir.path().join("open").join(SAVED_AS).exists(),
            "a project's folder goes with its last copy"
        );
        assert!(!old_alone.exists());
        assert!(!flat.exists(), "the earlier layout is swept too");
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
