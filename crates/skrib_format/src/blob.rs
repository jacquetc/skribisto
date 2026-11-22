// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A stored file's bytes, wherever they are.
//!
//! A bundle carries binary files: the pictures its prose shows, and the files a
//! writer attached to their items, which may be a scanned PDF or an hour of
//! audio. Holding those in memory was affordable while they were all pictures.
//! It is not for a project with half a gigabyte of research in it: every save,
//! save-as and backup would read all of it into RAM, and every open would too.
//!
//! So a [`Blob`] is either bytes an importer has just made ([`Blob::Bytes`]), or
//! a file already on disk ([`Blob::File`]) that is only opened when a writer
//! copies it somewhere. Reading a bundle never reads a blob; writing one streams
//! it, or links it when the source sits on the same file system.

use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tempfile::TempDir;

/// The bytes of one stored file. See the module docs.
#[derive(Clone)]
pub enum Blob {
    /// Bytes made in memory, by an importer or a test.
    Bytes(Vec<u8>),
    /// A file on disk, opened only when the bytes are needed.
    File {
        path: PathBuf,
        /// The extraction folder `path` lives in, when it is one: a zip's members
        /// are extracted to a scratch folder that has to outlive the read, and is
        /// removed once the last blob pointing into it is dropped.
        keep: Option<Arc<TempDir>>,
    },
}

impl Blob {
    /// A file on disk that belongs to someone else (the project's own folder, the
    /// media directory): nothing is removed when the blob is dropped.
    pub fn file(path: impl Into<PathBuf>) -> Self {
        Blob::File {
            path: path.into(),
            keep: None,
        }
    }

    /// A file inside a scratch folder this blob keeps alive.
    pub(crate) fn file_in(path: PathBuf, keep: Arc<TempDir>) -> Self {
        Blob::File {
            path,
            keep: Some(keep),
        }
    }

    /// The file on disk, for a blob that is one.
    pub fn path(&self) -> Option<&Path> {
        match self {
            Blob::Bytes(_) => None,
            Blob::File { path, .. } => Some(path),
        }
    }

    /// How many bytes the blob holds.
    pub fn len(&self) -> io::Result<u64> {
        match self {
            Blob::Bytes(bytes) => Ok(bytes.len() as u64),
            Blob::File { path, .. } => Ok(fs::metadata(path)?.len()),
        }
    }

    /// Whether the blob holds no bytes at all.
    pub fn is_empty(&self) -> io::Result<bool> {
        Ok(self.len()? == 0)
    }

    /// A reader over the bytes.
    pub fn open(&self) -> io::Result<Box<dyn Read + '_>> {
        match self {
            Blob::Bytes(bytes) => Ok(Box::new(bytes.as_slice())),
            Blob::File { path, .. } => Ok(Box::new(File::open(path)?)),
        }
    }

    /// All the bytes, in memory. For the callers that genuinely need them whole (a
    /// picture to decode, a test to compare); a writer streams with [`Self::open`]
    /// or [`Self::place_at`] instead.
    pub fn read(&self) -> io::Result<Vec<u8>> {
        match self {
            Blob::Bytes(bytes) => Ok(bytes.clone()),
            Blob::File { path, .. } => fs::read(path),
        }
    }

    /// Make `dest` hold these bytes, atomically and durably. Returns whether
    /// anything was written.
    ///
    /// Nothing is written when `dest` is the very file the blob reads from (a
    /// folder project saved in place), or already holds the same bytes. Otherwise
    /// the bytes go to a temporary file beside `dest`, which is renamed over it:
    /// a hard link of the source when the two share a file system (no byte is
    /// copied, which is what keeps a zip save of a large project affordable), a
    /// streamed copy when they do not.
    pub fn place_at(&self, dest: &Path) -> io::Result<bool> {
        if let Some(src) = self.path()
            && same_file(src, dest)
        {
            return Ok(false);
        }
        if dest.is_file() && self.same_contents_as(dest)? {
            return Ok(false);
        }
        let parent = dest
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("no parent folder for {}", dest.display()),
                )
            })?;
        fs::create_dir_all(parent)?;

        if let Some(src) = self.path() {
            // A hard link needs a free name: take one from `tempfile`, give it
            // back, and link onto it. `TempPath` removes it again on any way out
            // but the final rename.
            let slot = tempfile::Builder::new()
                .prefix(".blob")
                .tempfile_in(parent)?
                .into_temp_path();
            fs::remove_file(&slot)?;
            if fs::hard_link(src, &slot).is_ok() {
                return slot.persist(dest).map(|_| true).map_err(|e| e.error);
            }
            // Another file system, or one with no hard links: copy instead.
            drop(slot);
        }

        let mut tmp = tempfile::Builder::new()
            .prefix(".blob")
            .tempfile_in(parent)?;
        io::copy(&mut self.open()?, tmp.as_file_mut())?;
        tmp.as_file_mut().flush()?;
        crate::writer::sync_file(tmp.as_file())?;
        let (file, path) = tmp.into_parts();
        drop(file);
        path.persist(dest).map(|_| true).map_err(|e| e.error)
    }

    /// Whether the file at `other` holds exactly these bytes. Sizes first, so two
    /// different files of different lengths are told apart without a read.
    pub fn same_contents_as(&self, other: &Path) -> io::Result<bool> {
        if self.len()? != fs::metadata(other)?.len() {
            return Ok(false);
        }
        same_streams(&mut self.open()?, &mut File::open(other)?)
    }
}

impl From<Vec<u8>> for Blob {
    fn from(bytes: Vec<u8>) -> Self {
        Blob::Bytes(bytes)
    }
}

/// Two blobs are equal when they hold the same bytes, wherever those are. This
/// reads both, so it is for tests and for the rare whole-bundle comparison, never
/// for a save path.
impl PartialEq for Blob {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Blob::Bytes(a), Blob::Bytes(b)) => a == b,
            _ => match (self.open(), other.open(), self.len(), other.len()) {
                (Ok(mut a), Ok(mut b), Ok(la), Ok(lb)) => {
                    la == lb && same_streams(&mut a, &mut b).unwrap_or(false)
                }
                _ => false,
            },
        }
    }
}

impl Eq for Blob {}

impl fmt::Debug for Blob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Blob::Bytes(bytes) => write!(f, "Blob::Bytes({} bytes)", bytes.len()),
            Blob::File { path, .. } => write!(f, "Blob::File({})", path.display()),
        }
    }
}

/// Whether `a` and `b` name the same file on disk, through whatever spelling
/// (a relative path, a symlinked folder, a different Unicode normalisation that
/// the file system folds).
fn same_file(a: &Path, b: &Path) -> bool {
    match (fs::canonicalize(a), fs::canonicalize(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

/// Whether two readers yield the same bytes, compared a block at a time.
fn same_streams(a: &mut dyn Read, b: &mut dyn Read) -> io::Result<bool> {
    const BLOCK: usize = 64 * 1024;
    let mut left = vec![0u8; BLOCK];
    let mut right = vec![0u8; BLOCK];
    loop {
        let na = fill(a, &mut left)?;
        let nb = fill(b, &mut right)?;
        if na != nb || left[..na] != right[..nb] {
            return Ok(false);
        }
        if na < BLOCK {
            return Ok(true);
        }
    }
}

/// Read until `buf` is full or the stream ends; how many bytes were read.
fn fill(r: &mut dyn Read, buf: &mut [u8]) -> io::Result<usize> {
    let mut n = 0;
    while n < buf.len() {
        match r.read(&mut buf[n..]) {
            Ok(0) => break,
            Ok(k) => n += k,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_blob_placed_on_the_file_it_reads_from_writes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.png");
        fs::write(&path, b"pixels").unwrap();
        let blob = Blob::file(&path);
        assert!(!blob.place_at(&path).unwrap());
        assert_eq!(fs::read(&path).unwrap(), b"pixels");
    }

    #[test]
    fn a_blob_placed_over_the_same_bytes_writes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.png");
        fs::write(&path, b"pixels").unwrap();
        assert!(!Blob::from(b"pixels".to_vec()).place_at(&path).unwrap());
    }

    #[test]
    fn a_file_blob_is_placed_with_its_bytes_and_its_source_kept() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("src.pdf");
        fs::write(&src, b"%PDF-1.7 research").unwrap();
        let dest = dir.path().join("out").join("copy.pdf");
        assert!(Blob::file(&src).place_at(&dest).unwrap());
        assert_eq!(fs::read(&dest).unwrap(), b"%PDF-1.7 research");
        assert_eq!(fs::read(&src).unwrap(), b"%PDF-1.7 research");
    }

    #[test]
    fn different_bytes_of_the_same_length_replace_the_old_ones() {
        let dir = tempfile::tempdir().unwrap();
        let dest = dir.path().join("a.bin");
        fs::write(&dest, b"aaaa").unwrap();
        assert!(Blob::from(b"bbbb".to_vec()).place_at(&dest).unwrap());
        assert_eq!(fs::read(&dest).unwrap(), b"bbbb");
    }

    #[test]
    fn a_comparison_spans_more_than_one_block() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("big.bin");
        let mut bytes = vec![7u8; 200 * 1024];
        fs::write(&path, &bytes).unwrap();
        assert!(Blob::from(bytes.clone()).same_contents_as(&path).unwrap());
        // One byte different, in the last block.
        if let Some(last) = bytes.last_mut() {
            *last = 8;
        }
        assert!(!Blob::from(bytes).same_contents_as(&path).unwrap());
    }

    #[test]
    fn blobs_compare_by_their_bytes_wherever_they_are() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.png");
        fs::write(&path, b"pixels").unwrap();
        assert_eq!(Blob::file(&path), Blob::from(b"pixels".to_vec()));
        assert_ne!(Blob::file(&path), Blob::from(b"pixel".to_vec()));
    }

    #[test]
    fn a_scratch_folder_lives_as_long_as_a_blob_in_it() {
        let dir = Arc::new(tempfile::tempdir().unwrap());
        let path = dir.path().join("a.png");
        fs::write(&path, b"pixels").unwrap();
        let root = dir.path().to_path_buf();
        let blob = Blob::file_in(path.clone(), Arc::clone(&dir));
        drop(dir);
        assert_eq!(blob.read().unwrap(), b"pixels");
        drop(blob);
        assert!(!root.exists());
    }
}
