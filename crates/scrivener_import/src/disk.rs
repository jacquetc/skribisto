// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Every filesystem access the source layer makes, behind one seam.
//!
//! The reader asks three things of a disk: what is at a path (never following a
//! link), what a folder lists, and a file's bytes. [`RealDisk`] answers them with
//! the standard library. Tests answer them too, which is how a project that
//! changes while it is read, a cloud placeholder flag, or a record of every path
//! touched is tested deterministically rather than by racing a timer.
//!
//! # Links are never followed, and reading never blocks
//!
//! Metadata is always `lstat`-style: a link is reported as a link. A read opens
//! the file without following a link and without blocking (a FIFO swapped in
//! after the folder was listed would otherwise hang the read forever, beyond
//! the reach of the cancel button), then checks through the open handle that it
//! is a regular file before reading a byte.
//!
//! # Cloud placeholders are seen from metadata alone
//!
//! Reading a placeholder is what makes the operating system download it, a
//! blocking read cancel cannot stop and that fails offline. So the flags that
//! mark one are read with the metadata: `SF_DATALESS` on macOS 14 and later
//! (files and folders), and the recall and offline attributes the Windows Cloud
//! Files API sets for OneDrive, Dropbox, Google Drive and Nextcloud. The name
//! patterns older clients use (`.name.icloud`, `name.nextcloud`) are the index's
//! business, since they are names, not metadata.

use std::ffi::OsString;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// What kind of thing a path is, without following a link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EntryKind {
    File,
    Dir,
    Symlink,
    /// A FIFO, socket or device: never opened.
    Other,
}

/// What the index needs to know about a path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Meta {
    pub(crate) kind: EntryKind,
    pub(crate) len: u64,
    /// Nanoseconds since the Unix epoch, when the platform says.
    pub(crate) modified_ns: Option<i64>,
    /// A cloud placeholder: listed, but its data is not on this computer.
    pub(crate) placeholder: bool,
}

/// The source layer's view of a filesystem.
pub(crate) trait Disk {
    /// What is at `path`, not following a link.
    fn metadata(&self, path: &Path) -> io::Result<Meta>;
    /// What is at `path`, following links: only ever used for the path the
    /// writer picked, which may itself be a link they chose.
    fn metadata_following(&self, path: &Path) -> io::Result<Meta>;
    /// Where `path` leads, every link on the way followed: only ever used for a
    /// pick that is itself a link, so the project is read where it leads.
    fn resolve(&self, path: &Path) -> io::Result<PathBuf>;
    /// The entries of the folder `dir`, each with its metadata, unsorted: at most
    /// `limit + 1` of them, so a caller holding a limit of `limit` can tell a
    /// folder past it without the listing growing without bound.
    fn list(&self, dir: &Path, limit: usize) -> io::Result<Vec<(OsString, Meta)>>;
    /// The bytes of the regular file at `path`, refusing more than `limit`.
    fn read(&self, path: &Path, limit: u64) -> io::Result<Vec<u8>>;
    /// The first `count` bytes of the regular file at `path`, or fewer.
    fn read_head(&self, path: &Path, count: u64) -> io::Result<Vec<u8>>;
}

/// The filesystem, through the standard library.
pub(crate) struct RealDisk;

/// `SF_DATALESS`: the file or folder's data lives in the cloud (macOS 14 and
/// later).
#[cfg(target_os = "macos")]
const SF_DATALESS: u32 = 0x4000_0000;
/// `FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS`: not fully present locally.
#[cfg(windows)]
const RECALL_ON_DATA_ACCESS: u32 = 0x0040_0000;
/// `FILE_ATTRIBUTE_RECALL_ON_OPEN`: a folder whose listing is not local.
#[cfg(windows)]
const RECALL_ON_OPEN: u32 = 0x0004_0000;
/// `FILE_ATTRIBUTE_OFFLINE`.
#[cfg(windows)]
const OFFLINE: u32 = 0x0000_1000;

fn meta_of(md: &std::fs::Metadata) -> Meta {
    let ft = md.file_type();
    let kind = if ft.is_symlink() {
        EntryKind::Symlink
    } else if ft.is_dir() {
        EntryKind::Dir
    } else if ft.is_file() {
        EntryKind::File
    } else {
        EntryKind::Other
    };
    let modified_ns = md
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .and_then(|d| i64::try_from(d.as_nanos()).ok());
    Meta {
        kind,
        len: md.len(),
        modified_ns,
        placeholder: is_placeholder(md),
    }
}

#[cfg(target_os = "macos")]
fn is_placeholder(md: &std::fs::Metadata) -> bool {
    use std::os::macos::fs::MetadataExt;
    md.st_flags() & SF_DATALESS != 0
}

#[cfg(windows)]
fn is_placeholder(md: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    md.file_attributes() & (RECALL_ON_DATA_ACCESS | RECALL_ON_OPEN | OFFLINE) != 0
}

#[cfg(not(any(target_os = "macos", windows)))]
fn is_placeholder(_md: &std::fs::Metadata) -> bool {
    // Linux has no placeholder flag: Nextcloud and ownCloud virtual files are
    // recognised by name, and FUSE mounts (rclone, onedriver) are transparent.
    false
}

impl Disk for RealDisk {
    fn metadata(&self, path: &Path) -> io::Result<Meta> {
        std::fs::symlink_metadata(path).map(|md| meta_of(&md))
    }

    fn metadata_following(&self, path: &Path) -> io::Result<Meta> {
        std::fs::metadata(path).map(|md| meta_of(&md))
    }

    fn resolve(&self, path: &Path) -> io::Result<PathBuf> {
        std::fs::canonicalize(path)
    }

    fn list(&self, dir: &Path, limit: usize) -> io::Result<Vec<(OsString, Meta)>> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            if out.len() > limit {
                break;
            }
            let entry = entry?;
            // `DirEntry::metadata` does not follow a link, on every platform.
            let md = entry.metadata()?;
            out.push((entry.file_name(), meta_of(&md)));
        }
        Ok(out)
    }

    fn read(&self, path: &Path, limit: u64) -> io::Result<Vec<u8>> {
        let file = open_regular(path)?;
        let mut out = Vec::new();
        file.take(limit.saturating_add(1)).read_to_end(&mut out)?;
        if out.len() as u64 > limit {
            return Err(io::Error::from(io::ErrorKind::FileTooLarge));
        }
        Ok(out)
    }

    fn read_head(&self, path: &Path, count: u64) -> io::Result<Vec<u8>> {
        let file = open_regular(path)?;
        let mut out = Vec::new();
        file.take(count).read_to_end(&mut out)?;
        Ok(out)
    }
}

/// Open `path` for reading without following a link or blocking, and make sure
/// what was opened is a regular file.
fn open_regular(path: &Path) -> io::Result<std::fs::File> {
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    #[cfg(not(unix))]
    {
        // Windows opens a link's target; check what is at the path first.
        let md = std::fs::symlink_metadata(path)?;
        if !md.file_type().is_file() {
            return Err(not_a_regular_file());
        }
    }
    let file = options.open(path)?;
    if !file.metadata()?.file_type().is_file() {
        return Err(not_a_regular_file());
    }
    Ok(file)
}

/// The error a read returns for something that is not a regular file, told
/// apart from the system's own by its payload.
#[derive(Debug)]
pub(crate) struct NotARegularFile;

impl std::fmt::Display for NotARegularFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("not a regular file")
    }
}

impl std::error::Error for NotARegularFile {}

fn not_a_regular_file() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, NotARegularFile)
}

#[cfg(test)]
pub(crate) mod testing {
    //! Disks for tests: one that remembers every path it was asked about, one
    //! that makes files look changed, one that marks files as placeholders.

    use super::*;
    use std::cell::RefCell;
    use std::collections::{BTreeSet, HashMap};

    /// The real disk, remembering every path the reader touched.
    #[derive(Default)]
    pub(crate) struct Recording {
        pub(crate) touched: RefCell<BTreeSet<PathBuf>>,
    }

    impl Recording {
        fn note(&self, path: &Path) {
            self.touched.borrow_mut().insert(path.to_path_buf());
        }
    }

    impl Disk for Recording {
        fn metadata(&self, path: &Path) -> io::Result<Meta> {
            self.note(path);
            RealDisk.metadata(path)
        }
        fn metadata_following(&self, path: &Path) -> io::Result<Meta> {
            self.note(path);
            RealDisk.metadata_following(path)
        }
        fn resolve(&self, path: &Path) -> io::Result<PathBuf> {
            self.note(path);
            RealDisk.resolve(path)
        }
        fn list(&self, dir: &Path, limit: usize) -> io::Result<Vec<(OsString, Meta)>> {
            self.note(dir);
            RealDisk.list(dir, limit)
        }
        fn read(&self, path: &Path, limit: u64) -> io::Result<Vec<u8>> {
            self.note(path);
            RealDisk.read(path, limit)
        }
        fn read_head(&self, path: &Path, count: u64) -> io::Result<Vec<u8>> {
            self.note(path);
            RealDisk.read_head(path, count)
        }
    }

    /// The real disk, except that the files whose name ends with `suffix` report
    /// a new modification time every time they are looked at after being read:
    /// a project something keeps writing to.
    pub(crate) struct Restless {
        pub(crate) suffix: String,
        pub(crate) bumps: RefCell<HashMap<PathBuf, i64>>,
        pub(crate) read: RefCell<BTreeSet<PathBuf>>,
        /// Stop moving after this many bumps (`None`: never stop).
        pub(crate) settle_after: Option<usize>,
        pub(crate) bumped: RefCell<usize>,
    }

    impl Restless {
        pub(crate) fn new(suffix: &str, settle_after: Option<usize>) -> Self {
            Self {
                suffix: suffix.to_string(),
                bumps: RefCell::new(HashMap::new()),
                read: RefCell::new(BTreeSet::new()),
                settle_after,
                bumped: RefCell::new(0),
            }
        }

        fn adjust(&self, path: &Path, mut meta: Meta) -> Meta {
            if !path.to_string_lossy().ends_with(&self.suffix) {
                return meta;
            }
            let was_read = self.read.borrow().contains(path);
            let settled = self
                .settle_after
                .is_some_and(|n| *self.bumped.borrow() >= n);
            if was_read && !settled {
                *self.bumped.borrow_mut() += 1;
                let mut bumps = self.bumps.borrow_mut();
                let bump = bumps.entry(path.to_path_buf()).or_insert(0);
                *bump += 1_000_000_000;
            }
            if let Some(bump) = self.bumps.borrow().get(path) {
                meta.modified_ns = meta.modified_ns.map(|t| t + bump);
            }
            meta
        }
    }

    impl Disk for Restless {
        fn metadata(&self, path: &Path) -> io::Result<Meta> {
            let meta = RealDisk.metadata(path)?;
            Ok(self.adjust(path, meta))
        }
        fn metadata_following(&self, path: &Path) -> io::Result<Meta> {
            RealDisk.metadata_following(path)
        }
        fn resolve(&self, path: &Path) -> io::Result<PathBuf> {
            RealDisk.resolve(path)
        }
        fn list(&self, dir: &Path, limit: usize) -> io::Result<Vec<(OsString, Meta)>> {
            let mut out = RealDisk.list(dir, limit)?;
            for (name, meta) in &mut out {
                let path = dir.join(&*name);
                if let Some(bump) = self.bumps.borrow().get(&path) {
                    meta.modified_ns = meta.modified_ns.map(|t| t + bump);
                }
            }
            Ok(out)
        }
        fn read(&self, path: &Path, limit: u64) -> io::Result<Vec<u8>> {
            self.read.borrow_mut().insert(path.to_path_buf());
            RealDisk.read(path, limit)
        }
        fn read_head(&self, path: &Path, count: u64) -> io::Result<Vec<u8>> {
            self.read.borrow_mut().insert(path.to_path_buf());
            RealDisk.read_head(path, count)
        }
    }

    /// The real disk, except that paths ending with one of `suffixes` are cloud
    /// placeholders, and folders among them are not listed (listing one is what
    /// downloads it).
    pub(crate) struct Clouded {
        pub(crate) suffixes: Vec<String>,
    }

    impl Clouded {
        fn marked(&self, path: &Path) -> bool {
            let text = path.to_string_lossy().replace('\\', "/");
            self.suffixes.iter().any(|s| text.ends_with(s.as_str()))
        }
    }

    impl Disk for Clouded {
        fn metadata(&self, path: &Path) -> io::Result<Meta> {
            let mut meta = RealDisk.metadata(path)?;
            meta.placeholder |= self.marked(path);
            Ok(meta)
        }
        fn metadata_following(&self, path: &Path) -> io::Result<Meta> {
            RealDisk.metadata_following(path)
        }
        fn resolve(&self, path: &Path) -> io::Result<PathBuf> {
            RealDisk.resolve(path)
        }
        fn list(&self, dir: &Path, limit: usize) -> io::Result<Vec<(OsString, Meta)>> {
            if self.marked(dir) {
                return Err(io::Error::other("a placeholder folder was listed"));
            }
            let mut out = RealDisk.list(dir, limit)?;
            for (name, meta) in &mut out {
                meta.placeholder |= self.marked(&dir.join(&*name));
            }
            Ok(out)
        }
        fn read(&self, path: &Path, limit: u64) -> io::Result<Vec<u8>> {
            if self.marked(path) {
                return Err(io::Error::other("a placeholder was read"));
            }
            RealDisk.read(path, limit)
        }
        fn read_head(&self, path: &Path, count: u64) -> io::Result<Vec<u8>> {
            if self.marked(path) {
                return Err(io::Error::other("a placeholder was read"));
            }
            RealDisk.read_head(path, count)
        }
    }
}
