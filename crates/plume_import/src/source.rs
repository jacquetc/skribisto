// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading a `.plume` project's raw members, from either container Plume ever
//! used:
//!  - the modern **single-file zip** (`tree`/`info`/`attendance` + `text/`,
//!    `attend/`, `dicts/` members), and
//!  - the pre-0.3 **"old system"**: a bare directory of loose `*.plume` (the tree,
//!    root `<plume>`), `*.attend`, `*.prjinfo` files plus `text/`/`attend/`/`dicts/`
//!    subfolders, opened via the loose `.plume` file.
//!
//! Detection matches Plume's own `Utils::isProjectFromOldSystem = !ZipChecker::isZip`
//! (`utils.cpp` / `zipchecker.cpp`): if the path parses as a zip it is modern,
//! otherwise it is the old loose-file layout. There is no SQLite variant — Plume
//! never used one.
//!
//! A `.plume` is tiny (tens of KB), so every member is read into memory up front;
//! callers then look prose/synopsis/notes/attendance blobs up by their numeric id.
//!
//! # Links in an old-system folder are never followed
//!
//! The loose files of an old-system project are read out of a folder that may have
//! come out of an archive someone sent, and an archive can hold links: followed,
//! `text/T1.html -> ~/.ssh/id_rsa` imports the key as a scene's prose. So no link is
//! followed, to a file or to one of the `text`, `attend` and `dicts` folders, and
//! nothing but a regular file is read (no FIFO, which would block the read forever,
//! no socket and no device): an entry that is not one, by its own type, is not even
//! opened. Each file skipped is named in a notice. A file is opened through
//! [`skrib_format::regular_file`] and read no further than the zip limits below
//! allow, so a huge loose file is refused before it is in memory.
//!
//! The `.plume` the writer picked is read the same way, after one step: a link
//! there was picked on purpose, so it is resolved once before the guarded open.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use skrib_format::import_notice::ImportNotice;
use skrib_format::regular_file;

use crate::diagnostics::{Refusal, file_name, unreadable};

/// All of a `.plume` project's text members, read into memory.
pub struct PlumeSource {
    pub tree_xml: String,
    pub attendance_xml: Option<String>,
    pub info_xml: Option<String>,
    pub dict: Option<String>,
    /// Prose / synopsis / note / attendance HTML blobs keyed by their member
    /// path, e.g. `"text/T3.html"`, `"attend/A1.html"`.
    html: HashMap<String, String>,
    /// The files of an old-system project that were left unread because they are
    /// not regular files (a link, a FIFO, a device), for the import summary.
    pub notices: Vec<ImportNotice>,
}

const DICT_MEMBER: &str = "dicts/userDict.dict_plume";

/// What a `.plume` zip may hold, before and while it inflates (see
/// [`skrib_format::zip_guard`]).
///
/// A real `.plume` is tiny — tens of kilobytes, one small XML tree and a handful of
/// HTML blobs — so these are generous headroom, not a tight fit: a novel's worth of
/// prose is a few megabytes. A `.plume` past any of them is one built to exhaust
/// memory, not one Plume Creator wrote.
const LIMITS: skrib_format::zip_guard::ZipLimits = skrib_format::zip_guard::ZipLimits {
    max_entries: 100_000,
    max_member_bytes: 256 << 20,
    max_total_bytes: 512 << 20,
    max_ratio: skrib_format::zip_guard::MAX_RATIO,
};

#[cfg(test)]
impl PlumeSource {
    /// An in-memory source carrying no HTML blobs, so the mapper can be exercised on
    /// hand-built `PlumeTree`/`PlumeAttendance` fixtures without a real `.plume` zip.
    /// Every `text`/`synopsis`/`note`/`attend_doc` lookup returns `None`, which the
    /// mapper already handles (a Plume node may legitimately have no text member).
    pub(super) fn for_tests() -> Self {
        Self {
            tree_xml: String::new(),
            attendance_xml: None,
            info_xml: None,
            dict: None,
            html: HashMap::new(),
            notices: Vec::new(),
        }
    }

    /// [`Self::for_tests`], carrying the HTML members given as `(member path, html)`,
    /// e.g. `("text/T1.html", "<p>Words.</p>")`.
    pub(super) fn for_tests_with_html(members: &[(&str, &str)]) -> Self {
        Self {
            html: members
                .iter()
                .map(|(path, html)| (path.to_string(), html.to_string()))
                .collect(),
            ..Self::for_tests()
        }
    }
}

impl PlumeSource {
    /// Open the project at `source_path` (a `.plume`/`.plume_backup` zip, or the
    /// loose `.plume` tree file of an old-system project).
    pub fn open(source_path: &str) -> Result<Self> {
        let path = Path::new(source_path);
        // A picked link is followed once, and an old-system project read where it
        // leads: its loose files sit beside the tree file itself, not beside the link.
        let (resolved, bytes) = read_picked(path)?;

        // Modern zip? (Plume's own zip-vs-loose discriminator.)
        match skrib_format::zip_guard::ZipGuard::open(LIMITS, std::io::Cursor::new(&bytes[..])) {
            Ok((archive, guard)) => Self::from_zip(path, archive, guard),
            Err(_) => Self::from_old_system(&resolved, &bytes),
        }
    }

    fn from_zip(
        path: &Path,
        mut archive: zip::ZipArchive<std::io::Cursor<&[u8]>>,
        mut guard: skrib_format::zip_guard::ZipGuard,
    ) -> Result<Self> {
        // An archive that opens but cannot be read through is damaged, and says so
        // by name. A size refusal from the guard stays what it is: the typed
        // `ZipRefused` the interface words itself.
        let damaged = |reason: String| {
            anyhow::Error::new(Refusal::Damaged {
                file: file_name(path),
                reason,
            })
        };
        // A `.plume` is a zip from an untrusted source, so every member inflates under
        // the shared guard — a bomb or an over-declared member refused before it fills
        // memory.
        guard.check_directory(&mut archive, |_| true)?;

        let mut tree_xml = None;
        let mut attendance_xml = None;
        let mut info_xml = None;
        let mut dict = None;
        let mut html = HashMap::new();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| damaged(e.to_string()))?;
            if !file.is_file() {
                continue;
            }
            let name = file
                .name()
                .map_err(|e| damaged(format!("entry {i}: {e}")))?
                .trim_start_matches('/')
                .to_string();
            let buf = guard.read(&mut file).map_err(|e| {
                if skrib_format::zip_guard::refused(&e).is_some() {
                    e
                } else {
                    damaged(format!("{e:#}"))
                }
            })?;
            let text = String::from_utf8_lossy(&buf).into_owned();
            match name.as_str() {
                "tree" => tree_xml = Some(text),
                "attendance" => attendance_xml = Some(text),
                "info" => info_xml = Some(text),
                DICT_MEMBER => dict = Some(text),
                _ if name.starts_with("text/") || name.starts_with("attend/") => {
                    html.insert(name, text);
                }
                _ => {}
            }
        }

        let tree_xml = tree_xml.ok_or_else(|| anyhow::Error::new(Refusal::NoOutline))?;
        Ok(Self {
            tree_xml,
            attendance_xml,
            info_xml,
            dict,
            html,
            notices: Vec::new(),
        })
    }

    fn from_old_system(tree_file: &Path, tree_bytes: &[u8]) -> Result<Self> {
        // A file that was just read always has a folder, if only the current one
        // (`Path::parent` answers "" for a bare name); `None` is the root itself.
        let dir = tree_file.parent().unwrap_or(Path::new("."));
        let tree_xml = String::from_utf8_lossy(tree_bytes).into_owned();
        let mut loose = LooseReader::default();

        // Locate the loose sibling metadata files by extension.
        let attendance_xml = match first_file_with_ext(dir, "attend") {
            Some((p, kind)) => loose.read_entry(&p, kind, file_name_of(&p))?,
            None => None,
        };
        let info_xml = match first_file_with_ext(dir, "prjinfo") {
            Some((p, kind)) => loose.read_entry(&p, kind, file_name_of(&p))?,
            None => None,
        };
        let dict = if loose.folder(dir, "dicts") {
            let p = dir.join(DICT_MEMBER);
            match fs::symlink_metadata(&p) {
                Ok(md) => loose.read_entry(&p, Ok(md.file_type()), DICT_MEMBER.to_string())?,
                Err(_) => None,
            }
        } else {
            None
        };

        // Load the text/ and attend/ subfolders into the html map.
        let mut html = HashMap::new();
        for sub in ["text", "attend"] {
            if !loose.folder(dir, sub) {
                continue;
            }
            let subdir = dir.join(sub);
            for entry in fs::read_dir(&subdir).map_err(|e| unreadable(&subdir, &e))? {
                let entry = entry.map_err(|e| unreadable(&subdir, &e))?;
                // The entry's own type: a link is a link here, never what it points at.
                let kind = entry.file_type();
                if kind.as_ref().is_ok_and(|t| t.is_dir()) {
                    continue;
                }
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let member = format!("{sub}/{name}");
                    if let Some(text) = loose.read_entry(&path, kind, member.clone())? {
                        html.insert(member, text);
                    }
                }
            }
        }

        if tree_xml.trim().is_empty() {
            return Err(anyhow::Error::new(Refusal::EmptyOutline));
        }
        Ok(Self {
            tree_xml,
            attendance_xml,
            info_xml,
            dict,
            html,
            notices: loose.notices,
        })
    }

    /// Manuscript prose for tree node `n` (`text/T{n}.html`).
    pub fn text(&self, n: u32) -> Option<&str> {
        self.html
            .get(&format!("text/T{n}.html"))
            .map(String::as_str)
    }
    /// Synopsis for tree node `n` (`text/S{n}.html`).
    pub fn synopsis(&self, n: u32) -> Option<&str> {
        self.html
            .get(&format!("text/S{n}.html"))
            .map(String::as_str)
    }
    /// Note for tree node `n` (`text/N{n}.html`).
    pub fn note(&self, n: u32) -> Option<&str> {
        self.html
            .get(&format!("text/N{n}.html"))
            .map(String::as_str)
    }
    /// Detail sheet for attendance node `n` (`attend/A{n}.html`).
    pub fn attend_doc(&self, n: u32) -> Option<&str> {
        self.html
            .get(&format!("attend/A{n}.html"))
            .map(String::as_str)
    }
}

/// Reads the loose files of an old-system project: regular files only, under the
/// same limits a `.plume` zip's members inflate under, noting each file left unread.
#[derive(Default)]
struct LooseReader {
    /// Bytes read so far, against [`LIMITS`]`.max_total_bytes`.
    total: u64,
    notices: Vec<ImportNotice>,
}

impl LooseReader {
    /// The file at `path`, read as text, or `None` (and a notice naming `member`)
    /// when it is not a regular file. A file that cannot be read, or one past the
    /// limits, refuses the project as it always has.
    fn read(&mut self, path: &Path, member: String) -> Result<Option<String>> {
        let file = match regular_file::open_regular(path) {
            Ok(file) => file,
            Err(e) if regular_file::is_not_a_regular_file(&e) => {
                self.notices.push(ImportNotice::FileUnreadable {
                    member,
                    error: e.to_string(),
                });
                return Ok(None);
            }
            Err(e) => return Err(unreadable(path, &e)),
        };
        let left = LIMITS.max_total_bytes.saturating_sub(self.total);
        let limit = LIMITS.max_member_bytes.min(left);
        let bytes = regular_file::read_limited(file, limit).map_err(|e| unreadable(path, &e))?;
        self.total = self.total.saturating_add(bytes.len() as u64);
        Ok(Some(String::from_utf8_lossy(&bytes).into_owned()))
    }

    /// The folder entry at `path`, whose own type is `kind` (a link's type is that
    /// of a link, never of what it points at): read as [`Self::read`] reads it when
    /// it is a regular file, else left unopened and named in a notice.
    ///
    /// Only a regular file is opened at all. Opening a socket fails with the
    /// system's own error rather than as "not a regular file", and that error
    /// refused the whole project over a file it never needed.
    fn read_entry(
        &mut self,
        path: &Path,
        kind: std::io::Result<fs::FileType>,
        member: String,
    ) -> Result<Option<String>> {
        match kind {
            Ok(kind) if kind.is_file() => self.read(path, member),
            Ok(_) => {
                self.skip(member, &regular_file::not_a_regular_file());
                Ok(None)
            }
            Err(e) => {
                self.skip(member, &e);
                Ok(None)
            }
        }
    }

    /// Name `member` in a notice as left unread, for `error`.
    fn skip(&mut self, member: String, error: &std::io::Error) {
        self.notices.push(ImportNotice::FileUnreadable {
            member,
            error: error.to_string(),
        });
    }

    /// Whether `dir/name` is a folder to read: a real one, not a link to one. A link
    /// is named in a notice; nothing there at all is no notice.
    fn folder(&mut self, dir: &Path, name: &str) -> bool {
        match fs::symlink_metadata(dir.join(name)) {
            Ok(md) if md.is_dir() => true,
            Ok(md) if md.file_type().is_symlink() => {
                self.skip(name.to_string(), &regular_file::not_a_regular_file());
                false
            }
            _ => false,
        }
    }
}

fn file_name_of(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// The entry in `dir` whose extension is `ext` (case-insensitive) to read, with its
/// own type, if there is one: the first regular file by name, else the first entry
/// that is not a folder.
///
/// A link is listed as a link, not as what it points at, and
/// [`LooseReader::read_entry`] then leaves it unread. So a regular file is preferred
/// over one: with `A.attend` a link and `B.attend` a real file, the first by name is
/// the link, and taking it would lose the attendance `B.attend` holds. Only when no
/// regular file is there is the link (or FIFO, or device) returned, to be named in
/// a notice.
fn first_file_with_ext(dir: &Path, ext: &str) -> Option<(PathBuf, std::io::Result<fs::FileType>)> {
    let mut matches: Vec<(PathBuf, std::io::Result<fs::FileType>)> = fs::read_dir(dir)
        .ok()?
        .filter_map(Result::ok)
        .map(|e| (e.path(), e.file_type()))
        .filter(|(_, kind)| !kind.as_ref().is_ok_and(|t| t.is_dir()))
        .filter(|(p, _)| {
            p.extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e.eq_ignore_ascii_case(ext))
        })
        .collect();
    let is_regular =
        |kind: &std::io::Result<fs::FileType>| kind.as_ref().is_ok_and(|t| t.is_file());
    // Regular files first, then by name.
    matches.sort_by(|(a, a_kind), (b, b_kind)| {
        (!is_regular(a_kind), a).cmp(&(!is_regular(b_kind), b))
    });
    matches.into_iter().next()
}

/// Read the `.plume` file the writer picked, no more than a `.plume` may hold.
///
/// A link there was picked on purpose and is followed, once: the path is resolved
/// first, and what it resolves to is opened through the guard, which follows
/// nothing further and refuses anything but a regular file. So a `.plume` that is a
/// FIFO, or a link to `/dev/zero` out of an unpacked archive, is refused before a
/// byte is read and without waiting on a writer, and a file past the limit is
/// refused before it is in memory.
///
/// Returns the resolved path with the bytes.
fn read_picked(path: &Path) -> Result<(PathBuf, Vec<u8>)> {
    let resolved = fs::canonicalize(path).map_err(|e| unreadable(path, &e))?;
    let bytes = regular_file::read_regular(&resolved, LIMITS.max_total_bytes)
        .map_err(|e| unreadable(path, &e))?;
    Ok((resolved, bytes))
}
