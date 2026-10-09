// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One index over whatever holds the project: a folder, a zip archive, or the zip
//! a `.scrivtemplate` carries.
//!
//! Every later access resolves a wanted path through this index, never by joining
//! a string onto a filesystem path. A name out of a manifest, a checksum list or a
//! lock file can only ever find a member the walk or the archive's directory
//! already listed, so containment holds by construction, for folders and archives
//! alike.
//!
//! # What indexing reads
//!
//! Metadata only: names, kinds, sizes, modification times, placeholder flags.
//! Nothing is read until the analysis asks for a member, and pictures and research
//! files are never read here at all. A folder is walked with a loop over the
//! folders still to list, never a recursion, so its depth costs no stack; the walk
//! stops descending at [`MAX_WALK_DEPTH`], far below anything that could matter to
//! Scrivener, whose deepest file sits four folders down.
//!
//! # A consistent read
//!
//! Scrivener autosaves two seconds after the writer stops typing, and a sync
//! service can be writing too. Every member read is recorded with the size and
//! time the index saw; [`Index::changed_members`] looks at each again after the
//! analysis, and an archive is checked through its own size and time. The caller
//! reads the whole project once more when anything moved, and refuses when it moves
//! again.

use std::cell::{OnceCell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use skrib_format::zip_guard::{ZipGuard, ZipLimits, ZipRefused};
use zip::{CompressionMethod, ZipArchive};

use crate::disk::{Disk, EntryKind, Meta};
use crate::names::{folded, is_archive_litter, nfc, placeholder_target, unsafe_member_name};
use crate::outcome::Refusal;

/// The most members a project may hold.
pub(crate) const MAX_MEMBERS: u64 = 100_000;
/// The most bytes of writing and binder a project may hold, together. Pictures
/// and research files are not counted: they are never read into memory here.
pub(crate) const MAX_TEXT_BYTES: u64 = 1 << 30;
/// The most bytes one member may hold.
pub(crate) const MAX_MEMBER_BYTES: u64 = 512 << 20;
/// How many folders deep a walk descends. Scrivener's own files sit at most four
/// deep (`Files/Data/<UUID>/content.rtf`); a writer's own folders may go deeper.
pub(crate) const MAX_WALK_DEPTH: usize = 64;

/// What an archive may hold, for [`ZipGuard`]. The totals are this importer's
/// own; the ratio is the shared one, which no real project comes near (the
/// largest measured is 25).
const ZIP_LIMITS: ZipLimits = ZipLimits {
    max_entries: MAX_MEMBERS as usize,
    max_member_bytes: MAX_MEMBER_BYTES,
    max_total_bytes: MAX_TEXT_BYTES,
    max_ratio: skrib_format::zip_guard::MAX_RATIO,
};

/// A reader an archive can sit on: a file on disk or bytes in memory.
pub(crate) trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}

/// One member of the index.
#[derive(Debug, Clone)]
pub(crate) struct Entry {
    pub(crate) kind: EntryKind,
    pub(crate) size: u64,
    pub(crate) modified_ns: Option<i64>,
    pub(crate) placeholder: bool,
    location: Location,
}

#[derive(Debug, Clone)]
enum Location {
    /// A file or folder on disk.
    Disk(PathBuf),
    /// An archive member, by its index in the central directory.
    Archive {
        index: usize,
        compression: Option<String>,
    },
    /// A folder an archive lists only through the members below it.
    Implied,
}

enum Backing<'d> {
    Folder {
        disk: &'d dyn Disk,
    },
    Archive {
        archive: RefCell<ZipArchive<Box<dyn ReadSeek>>>,
        guard: RefCell<ZipGuard>,
        /// The disk the archive was opened through, asked before each member is read.
        disk: &'d dyn Disk,
    },
}

/// Why a member could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ReadError {
    /// Not in the index, or not a file.
    Missing,
    /// A link: never followed.
    Link,
    /// A cloud placeholder: reading it would download it.
    Placeholder,
    /// Larger than the read allows.
    TooLarge,
    /// The system or the archive reader could not read it. Its own words.
    Unreadable(String),
    /// It changed between the walk and the read.
    Changed,
    /// Refused outright (an archive past its limits).
    Refused(Refusal),
}

impl ReadError {
    /// A system error, the two this crate raises itself told apart.
    fn from_io(e: &std::io::Error) -> Self {
        if e.kind() == std::io::ErrorKind::FileTooLarge {
            ReadError::TooLarge
        } else if e
            .get_ref()
            .is_some_and(|inner| inner.is::<crate::disk::NotARegularFile>())
        {
            ReadError::Missing
        } else {
            ReadError::Unreadable(e.to_string())
        }
    }

    /// Why it failed, as a notice carries it. `None` for the two outcomes a
    /// caller handles itself: a change under the read, and a refusal.
    pub(crate) fn problem(&self) -> Option<crate::outcome::ReadProblem> {
        use crate::outcome::ReadProblem;
        Some(match self {
            ReadError::Missing => ReadProblem::NotAFile,
            ReadError::Link => ReadProblem::Link,
            ReadError::Placeholder => ReadProblem::CloudPlaceholder,
            ReadError::TooLarge => ReadProblem::TooLarge,
            ReadError::Unreadable(reason) => ReadProblem::System {
                reason: reason.clone(),
            },
            ReadError::Changed | ReadError::Refused(_) => return None,
        })
    }
}

/// One member read, as the index saw it before the read.
#[derive(Debug, Clone)]
struct ReadRecord {
    key: String,
    size: u64,
    modified_ns: Option<i64>,
    /// The bytes read, hashed: what the analysis's fingerprint says of a member it read,
    /// rather than the time the file was last written.
    hash: blake3::Hash,
}

/// The index over one container.
pub(crate) struct Index<'d> {
    entries: BTreeMap<String, Entry>,
    /// Folder → the names directly inside it (`""` is the container's top), kept
    /// sorted and unique as they arrive: a folder of a hundred thousand members
    /// must not be searched once per member to learn a name is new.
    children: BTreeMap<String, BTreeSet<String>>,
    /// Members a name-based placeholder stands in for (`X` for `.X.icloud`).
    pub(crate) placeholder_targets: BTreeSet<String>,
    /// Folders not listed because they are placeholders themselves.
    pub(crate) placeholder_folders: BTreeSet<String>,
    /// Folders that could not be listed, with the system's words.
    pub(crate) unreadable_folders: BTreeMap<String, String>,
    /// Folders not descended into, past [`MAX_WALK_DEPTH`].
    pub(crate) too_deep: u32,
    /// Archive members dropped without a word (`__MACOSX`, `._*`, `.DS_Store`).
    pub(crate) archive_litter: u32,
    backing: Backing<'d>,
    reads: RefCell<Vec<ReadRecord>>,
    nfc_keys: OnceCell<HashMap<String, Vec<String>>>,
    folded_keys: OnceCell<HashMap<String, Vec<String>>>,
    /// The archive or template file on disk, and its metadata when indexed.
    container_file: Option<(PathBuf, Meta)>,
}

impl<'d> Index<'d> {
    fn empty(backing: Backing<'d>) -> Self {
        let mut children = BTreeMap::new();
        children.insert(String::new(), BTreeSet::new());
        Index {
            entries: BTreeMap::new(),
            children,
            placeholder_targets: BTreeSet::new(),
            placeholder_folders: BTreeSet::new(),
            unreadable_folders: BTreeMap::new(),
            too_deep: 0,
            archive_litter: 0,
            backing,
            reads: RefCell::new(Vec::new()),
            nfc_keys: OnceCell::new(),
            folded_keys: OnceCell::new(),
            container_file: None,
        }
    }

    /// Index the folder `root` and everything below it, never following a link.
    pub(crate) fn of_folder(disk: &'d dyn Disk, root: &Path) -> Result<Self, Refusal> {
        let mut index = Index::empty(Backing::Folder { disk });
        let mut members: u64 = 0;
        // Every folder listed and not yet walked: its key, its path, its depth.
        let mut pending: Vec<(String, PathBuf, usize)> =
            vec![(String::new(), root.to_path_buf(), 0)];
        while let Some((key, dir, depth)) = pending.pop() {
            let room = usize::try_from(MAX_MEMBERS.saturating_sub(members)).unwrap_or(usize::MAX);
            let listing = match disk.list(&dir, room) {
                Ok(listing) => listing,
                Err(e) if key.is_empty() => {
                    return Err(Refusal::Unreadable {
                        file: file_name(root),
                        reason: e.to_string(),
                    });
                }
                Err(e) => {
                    index.unreadable_folders.insert(key, e.to_string());
                    continue;
                }
            };
            for (name, meta) in listing {
                let name = name.to_string_lossy().into_owned();
                let child_key = join(&key, &name);
                let path = dir.join(&name);
                members += 1;
                if members > MAX_MEMBERS {
                    return Err(Refusal::TooManyMembers { limit: MAX_MEMBERS });
                }
                if meta.kind == EntryKind::File
                    && let Some(target) = placeholder_target(&name)
                {
                    index.placeholder_targets.insert(join(&key, &target));
                }
                index.insert(
                    child_key.clone(),
                    Entry {
                        kind: meta.kind,
                        size: meta.len,
                        modified_ns: meta.modified_ns,
                        placeholder: meta.placeholder,
                        location: Location::Disk(path.clone()),
                    },
                );
                if meta.kind != EntryKind::Dir {
                    continue;
                }
                if meta.placeholder {
                    // Listing a dataless folder is what downloads it.
                    index.placeholder_folders.insert(child_key);
                } else if name == ".git" {
                    // Version control's own tree: never part of the project.
                } else if depth + 1 > MAX_WALK_DEPTH {
                    index.too_deep += 1;
                } else {
                    pending.push((child_key, path, depth + 1));
                }
            }
        }
        Ok(index)
    }

    /// Index an archive on disk.
    pub(crate) fn of_archive_file(disk: &'d dyn Disk, path: &Path) -> Result<Self, Refusal> {
        let label = file_name(path);
        let meta = disk.metadata(path).map_err(|e| Refusal::Unreadable {
            file: label.clone(),
            reason: e.to_string(),
        })?;
        let file = std::fs::File::open(path).map_err(|e| Refusal::Unreadable {
            file: label.clone(),
            reason: e.to_string(),
        })?;
        let mut index = Index::of_archive(disk, Box::new(file), label)?;
        index.container_file = Some((path.to_path_buf(), meta));
        Ok(index)
    }

    /// Index an archive held in memory (a template's project, a spare binder), its
    /// members read only while `disk` says reading may go on.
    pub(crate) fn of_archive_bytes(
        disk: &'d dyn Disk,
        bytes: Vec<u8>,
        label: String,
    ) -> Result<Self, Refusal> {
        Index::of_archive(disk, Box::new(std::io::Cursor::new(bytes)), label)
    }

    fn of_archive(
        disk: &'d dyn Disk,
        mut reader: Box<dyn ReadSeek>,
        label: String,
    ) -> Result<Self, Refusal> {
        let unreadable = |reason: String| Refusal::ArchiveUnreadable {
            file: label.clone(),
            reason,
        };
        // (`reason` is always the archive reader's or the system's own words.)
        let declared = declared_entries(&mut reader).map_err(|e| unreadable(e.to_string()))?;
        let (mut archive, guard) =
            ZipGuard::open(ZIP_LIMITS, reader).map_err(|e| unreadable(e.to_string()))?;
        guard
            .check_directory(&mut archive, |_| false)
            .map_err(|refusal| from_zip_refusal(&refusal))?;
        // The zip reader keys members by name and keeps the last of two with one
        // name, silently. An archive listing more entries than it has names holds
        // duplicates, and which one a reader sees is an accident.
        if let Some(declared) = declared
            && declared > archive.len() as u64
        {
            return Err(Refusal::DuplicateArchiveMembers {
                file: label.clone(),
                member: None,
            });
        }
        let mut index = Index::empty(Backing::Folder {
            disk: &crate::disk::RealDisk,
        });
        for i in 0..archive.len() {
            let entry = archive
                .by_index_raw(i)
                .map_err(|e| unreadable(e.to_string()))?;
            let raw = entry.name().replace('\\', "/");
            if entry.encrypted() {
                return Err(Refusal::ArchiveEncrypted {
                    file: label.clone(),
                });
            }
            let trimmed = raw.trim_end_matches('/');
            if let Some(why) = unsafe_member_name(trimmed) {
                return Err(Refusal::ArchiveUnsafeName {
                    file: label.clone(),
                    member: raw.clone(),
                    why,
                });
            }
            let parts: Vec<&str> = trimmed
                .split('/')
                .filter(|p| !p.is_empty() && *p != ".")
                .collect();
            if parts.is_empty() {
                continue;
            }
            let key = parts.join("/");
            if is_archive_litter(&key) {
                index.archive_litter += 1;
                continue;
            }
            if index
                .entries
                .get(&key)
                .is_some_and(|e| !matches!(e.location, Location::Implied))
            {
                return Err(Refusal::DuplicateArchiveMembers {
                    file: label.clone(),
                    member: Some(key),
                });
            }
            let is_link = entry
                .unix_mode()
                .is_some_and(|mode| mode & 0o170_000 == 0o120_000);
            let kind = if entry.is_dir() || raw.ends_with('/') {
                EntryKind::Dir
            } else if is_link {
                EntryKind::Symlink
            } else {
                EntryKind::File
            };
            let compression = match entry.compression() {
                CompressionMethod::Stored | CompressionMethod::Deflated => None,
                other => Some(compression_name(other)),
            };
            index.insert(
                key,
                Entry {
                    kind,
                    size: entry.size(),
                    modified_ns: None,
                    placeholder: false,
                    location: Location::Archive {
                        index: i,
                        compression,
                    },
                },
            );
        }
        index.backing = Backing::Archive {
            archive: RefCell::new(archive),
            guard: RefCell::new(guard),
            disk,
        };
        Ok(index)
    }

    /// Record `entry` under `key`, and every folder above it an archive only
    /// implies.
    fn insert(&mut self, key: String, entry: Entry) {
        let mut child = key.clone();
        let mut parent = parent_of(&key).to_string();
        loop {
            let names = self.children.entry(parent.clone()).or_default();
            let name = child.rsplit('/').next().unwrap_or_default().to_string();
            names.insert(name);
            if parent.is_empty() || self.entries.contains_key(&parent) {
                break;
            }
            self.entries.insert(
                parent.clone(),
                Entry {
                    kind: EntryKind::Dir,
                    size: 0,
                    modified_ns: None,
                    placeholder: false,
                    location: Location::Implied,
                },
            );
            child = parent.clone();
            parent = parent_of(&parent).to_string();
        }
        if entry.kind == EntryKind::Dir {
            self.children.entry(key.clone()).or_default();
        }
        match self.entries.get(&key) {
            // A folder an archive listed after the members below it replaces the
            // implied one.
            Some(existing) if matches!(existing.location, Location::Implied) => {
                self.entries.insert(key, entry);
            }
            Some(_) => {}
            None => {
                self.entries.insert(key, entry);
            }
        }
    }

    /// The member at exactly `key`.
    pub(crate) fn entry(&self, key: &str) -> Option<&Entry> {
        self.entries.get(key)
    }

    /// The names directly inside folder `key`, sorted.
    pub(crate) fn children(&self, key: &str) -> Vec<String> {
        self.children
            .get(key)
            .map(|names| names.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Every member strictly below the folder `prefix` (`""` for everything),
    /// sorted.
    pub(crate) fn keys_under(&self, prefix: &str) -> Vec<String> {
        if prefix.is_empty() {
            return self.entries.keys().cloned().collect();
        }
        // Everything below `a/b` sorts from `a/b/` up to, not including, `a/b0`:
        // `0` is the character right after `/`. A plain "starts with" scan from
        // `a/b` would stop at a sibling such as `a/b-c`, which sorts in between.
        let low = format!("{prefix}/");
        let high = format!("{prefix}0");
        self.entries
            .range::<str, _>((
                std::ops::Bound::Included(low.as_str()),
                std::ops::Bound::Excluded(high.as_str()),
            ))
            .map(|(k, _)| k.clone())
            .collect()
    }

    /// Resolve a wanted path to the member present: exactly, then by NFC, then
    /// by NFC case-folded. A case-folded key two members share resolves to
    /// neither.
    pub(crate) fn lookup(&self, wanted: &str) -> Option<String> {
        if self.entries.contains_key(wanted) {
            return Some(wanted.to_string());
        }
        let nfc_keys = self.nfc_keys.get_or_init(|| {
            let mut map: HashMap<String, Vec<String>> = HashMap::new();
            for key in self.entries.keys() {
                map.entry(nfc(key)).or_default().push(key.clone());
            }
            map
        });
        if let Some([only]) = nfc_keys.get(&nfc(wanted)).map(Vec::as_slice) {
            return Some(only.clone());
        }
        let folded_keys = self.folded_keys.get_or_init(|| {
            let mut map: HashMap<String, Vec<String>> = HashMap::new();
            for key in self.entries.keys() {
                map.entry(folded(key)).or_default().push(key.clone());
            }
            map
        });
        match folded_keys.get(&folded(wanted)).map(Vec::as_slice) {
            Some([only]) => Some(only.clone()),
            _ => None,
        }
    }

    /// Whether `key` is a placeholder, or sits in a placeholder folder, or has a
    /// name-based placeholder standing in for it.
    pub(crate) fn is_placeholder(&self, key: &str) -> bool {
        if self.entries.get(key).is_some_and(|e| e.placeholder) {
            return true;
        }
        if self.placeholder_targets.contains(key) {
            return true;
        }
        ancestors(key).any(|folder| self.placeholder_folders.contains(folder))
    }

    /// The folder that could not be listed and that holds `key` (or is `key`), if
    /// any.
    pub(crate) fn unreadable_folder_of(&self, key: &str) -> Option<&str> {
        // Each of `key`'s own folders is asked after, never every unreadable
        // folder in turn: a project can hold thousands, and thousands of
        // documents are looked for.
        std::iter::once(key)
            .chain(ancestors(key))
            .find_map(|folder| self.unreadable_folders.get_key_value(folder))
            .map(|(folder, _)| folder.as_str())
    }

    /// Read the member `key` whole, refusing more than `limit` bytes.
    pub(crate) fn read(&self, key: &str, limit: u64) -> Result<Vec<u8>, ReadError> {
        let entry = self.entries.get(key).ok_or(ReadError::Missing)?;
        match entry.kind {
            EntryKind::File => {}
            EntryKind::Symlink => return Err(ReadError::Link),
            EntryKind::Dir | EntryKind::Other => return Err(ReadError::Missing),
        }
        if entry.placeholder {
            return Err(ReadError::Placeholder);
        }
        if entry.size > limit {
            return Err(ReadError::TooLarge);
        }
        let bytes = match (&self.backing, &entry.location) {
            (Backing::Folder { disk }, Location::Disk(path)) => {
                let bytes = disk.read(path, limit).map_err(|e| ReadError::from_io(&e))?;
                if bytes.len() as u64 != entry.size {
                    return Err(ReadError::Changed);
                }
                bytes
            }
            (
                Backing::Archive {
                    archive,
                    guard,
                    disk,
                },
                Location::Archive { index, compression },
            ) => {
                // A member comes out of the archive's own handle, not through the disk: a
                // cancel is honoured here, before the next one is inflated.
                disk.go_on().map_err(|e| ReadError::from_io(&e))?;
                if let Some(method) = compression {
                    return Err(ReadError::Refused(Refusal::ArchiveUnsupportedCompression {
                        member: key.to_string(),
                        method: method.clone(),
                    }));
                }
                let mut archive = archive.borrow_mut();
                let mut member = archive
                    .by_index(*index)
                    .map_err(|e| ReadError::Unreadable(e.to_string()))?;
                let bytes = guard.borrow_mut().read(&mut member).map_err(|e| {
                    match skrib_format::zip_guard::refused(&e) {
                        Some(refusal) => ReadError::Refused(from_zip_refusal(refusal)),
                        None => ReadError::Unreadable(e.to_string()),
                    }
                })?;
                if bytes.len() as u64 > limit {
                    return Err(ReadError::TooLarge);
                }
                bytes
            }
            _ => return Err(ReadError::Missing),
        };
        self.reads.borrow_mut().push(ReadRecord {
            key: key.to_string(),
            size: entry.size,
            modified_ns: entry.modified_ns,
            hash: blake3::hash(&bytes),
        });
        Ok(bytes)
    }

    /// The hash of the bytes each member read so far held, by key: the last read of a
    /// member read twice, which the consistent read makes the same bytes.
    pub(crate) fn read_hashes(&self) -> HashMap<String, blake3::Hash> {
        self.reads
            .borrow()
            .iter()
            .map(|record| (record.key.clone(), record.hash))
            .collect()
    }

    /// Members read whose size or time moved after they were read, or the
    /// container itself when it moved: Scrivener or a sync service writing while
    /// the project was read.
    pub(crate) fn changed_members(&self) -> Vec<String> {
        let mut moved = Vec::new();
        match &self.backing {
            Backing::Folder { disk } => {
                for record in self.reads.borrow().iter() {
                    let Some(Entry {
                        location: Location::Disk(path),
                        ..
                    }) = self.entries.get(&record.key)
                    else {
                        continue;
                    };
                    match disk.metadata(path) {
                        Ok(meta)
                            if meta.len == record.size
                                && meta.modified_ns == record.modified_ns => {}
                        _ => moved.push(record.key.clone()),
                    }
                }
            }
            Backing::Archive { .. } => {}
        }
        if let (Some((path, before)), Backing::Archive { .. }) =
            (&self.container_file, &self.backing)
        {
            match crate::disk::RealDisk.metadata(path) {
                Ok(now) if now.len == before.len && now.modified_ns == before.modified_ns => {}
                _ => moved.push(file_name(path)),
            }
        }
        moved.sort();
        moved.dedup();
        moved
    }

    /// Record a container file to re-check after the read (a template on disk).
    pub(crate) fn set_container_file(&mut self, path: PathBuf, meta: Meta) {
        self.container_file = Some((path, meta));
    }

    /// The archive or template file the project came in, by name, with its size
    /// and time when it was opened.
    pub(crate) fn container_facts(&self) -> Option<(String, u64, Option<i64>)> {
        self.container_file
            .as_ref()
            .map(|(path, meta)| (file_name(path), meta.len, meta.modified_ns))
    }

    /// The disk the index reads through: an archive nested in it (a spare binder) is read
    /// through the same one, so a cancel reaches it too.
    pub(crate) fn disk(&self) -> &'d dyn Disk {
        match &self.backing {
            Backing::Folder { disk } | Backing::Archive { disk, .. } => *disk,
        }
    }

    /// Whether the index is over an archive rather than a folder.
    pub(crate) fn is_archive(&self) -> bool {
        matches!(self.backing, Backing::Archive { .. })
    }
}

/// A member's key inside its folder's key.
pub(crate) fn join(parent: &str, name: &str) -> String {
    if parent.is_empty() {
        name.to_string()
    } else {
        format!("{parent}/{name}")
    }
}

/// The folder key holding `key` (`""` at the top).
pub(crate) fn parent_of(key: &str) -> &str {
    key.rfind('/').map_or("", |slash| &key[..slash])
}

/// The folders above `key`, nearest first, the container's top (`""`) excluded.
fn ancestors(key: &str) -> impl Iterator<Item = &str> {
    std::iter::successors(Some(parent_of(key)), |folder| {
        (!folder.is_empty()).then(|| parent_of(folder))
    })
    .filter(|folder| !folder.is_empty())
}

/// The name `path` ends in, with `..` and `.` taken as the path says: never the
/// path itself, which names the writer's folders, and empty for a path that
/// ends in no name (the root, `.`).
pub(crate) fn file_name(path: &Path) -> String {
    let mut names = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::Normal(name) => names.push(name),
            std::path::Component::ParentDir => {
                names.pop();
            }
            _ => {}
        }
    }
    names
        .last()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// An archive guard's refusal as this importer's.
fn from_zip_refusal(refusal: &ZipRefused) -> Refusal {
    match refusal {
        ZipRefused::TooManyEntries { limit, .. } => Refusal::ArchiveTooManyMembers {
            limit: *limit as u64,
        },
        ZipRefused::MemberTooLarge { part, limit, .. } => Refusal::ArchiveMemberTooLarge {
            member: part.clone(),
            limit_mib: limit / (1 << 20),
        },
        ZipRefused::ArchiveTooLarge { limit } => Refusal::ArchiveTooLarge {
            limit_mib: limit / (1 << 20),
        },
    }
}

/// The name a compression method goes by, for the refusal that names it.
fn compression_name(method: CompressionMethod) -> String {
    let named = [
        (CompressionMethod::DEFLATE64, "Deflate64"),
        (CompressionMethod::BZIP2, "bzip2"),
        (CompressionMethod::LZMA, "LZMA"),
        (CompressionMethod::ZSTD, "Zstandard"),
        (CompressionMethod::XZ, "XZ"),
        (CompressionMethod::PPMD, "PPMd"),
    ];
    named
        .iter()
        .find(|(known, _)| *known == method)
        .map(|(_, name)| (*name).to_string())
        .unwrap_or_else(|| format!("{method:?}"))
}

/// How many entries the archive's end-of-central-directory record declares, or
/// `None` when it cannot be found (the zip reader then refuses the archive
/// itself).
fn declared_entries(reader: &mut Box<dyn ReadSeek>) -> std::io::Result<Option<u64>> {
    let len = reader.seek(SeekFrom::End(0))?;
    // The record is 22 bytes plus a comment of at most 65,535.
    let tail_len = len.min(22 + 65_535);
    reader.seek(SeekFrom::Start(len - tail_len))?;
    let mut tail = vec![0u8; tail_len as usize];
    reader.read_exact(&mut tail)?;
    reader.seek(SeekFrom::Start(0))?;
    let Some(at) = (0..tail.len().saturating_sub(21))
        .rev()
        .find(|&i| tail[i..i + 4] == *b"PK\x05\x06")
    else {
        return Ok(None);
    };
    let total = u16::from_le_bytes([tail[at + 10], tail[at + 11]]);
    if total != u16::MAX {
        return Ok(Some(u64::from(total)));
    }
    // Zip64: the locator sits just before the record and points at the zip64
    // record, which holds the real count.
    if at < 20 || tail[at - 20..at - 16] != *b"PK\x06\x07" {
        return Ok(None);
    }
    let mut offset = [0u8; 8];
    offset.copy_from_slice(&tail[at - 12..at - 4]);
    let record = u64::from_le_bytes(offset);
    if record.saturating_add(56) > len {
        return Ok(None);
    }
    reader.seek(SeekFrom::Start(record))?;
    let mut head = [0u8; 40];
    reader.read_exact(&mut head)?;
    reader.seek(SeekFrom::Start(0))?;
    if head[..4] != *b"PK\x06\x06" {
        return Ok(None);
    }
    let mut total = [0u8; 8];
    total.copy_from_slice(&head[32..40]);
    Ok(Some(u64::from_le_bytes(total)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::disk::{Cancellable, RealDisk};
    use std::sync::atomic::{AtomicBool, Ordering};

    /// An archive's members come out of its own handle, not through the disk: once the
    /// writer cancels, the next one is refused rather than inflated, so a large backup or
    /// template is not read to its end after the import was stopped.
    #[test]
    fn a_cancel_stops_an_archive_at_its_next_member() {
        let mut files = scrivener_fixtures::Files::new();
        files.put("Novel.scriv/Novel.scrivx", "<ScrivenerProject/>");
        files.put("Novel.scriv/Files/Data/a.rtf", "{\\rtf1 One.}");
        let bytes = match files.zip(None) {
            Ok(bytes) => bytes,
            Err(e) => panic!("the archive: {e}"),
        };
        let cancel = AtomicBool::new(false);
        let disk = Cancellable {
            inner: &RealDisk,
            cancel: &cancel,
        };
        let index = match Index::of_archive_bytes(&disk, bytes, "Backup.zip".to_string()) {
            Ok(index) => index,
            Err(refusal) => panic!("the archive indexes: {refusal:?}"),
        };
        assert!(index.read("Novel.scriv/Novel.scrivx", 1 << 20).is_ok());
        cancel.store(true, Ordering::Relaxed);
        assert!(matches!(
            index.read("Novel.scriv/Files/Data/a.rtf", 1 << 20),
            Err(ReadError::Unreadable(_))
        ));
        assert_eq!(
            index.read_hashes().len(),
            1,
            "nothing read after the cancel"
        );
    }
}
