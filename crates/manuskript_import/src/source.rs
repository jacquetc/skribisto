// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Opening a Manuskript project and reading its members, from any container it
//! has ever used.
//!
//! There are three ways in, and only two of them are Manuskript's own:
//!
//! - a **zip** `.msk` — format 0 (2016, XML inside) or format 1;
//! - a **`.msk` text stub plus the folder beside it** — the modern default. The
//!   `.msk` is one byte holding `1` and carries no data at all; the project is the
//!   sibling directory of the same stem. Manuskript only ever addresses a project
//!   this way, which is why its own docs tell writers to copy both;
//! - the **project folder given directly**, which Manuskript cannot do. The import
//!   panel offers it because a writer who keeps the project in version control
//!   thinks of the folder as the project, and because a `.msk` stub is easy to
//!   lose. It is the one place this reader is deliberately wider than the source
//!   application, and it carries the staleness check below.
//!
//! # The stale-folder trap
//!
//! Switching a project from folder mode to zip mode does not clean up: the save
//! path writes the zip over the `.msk` and never removes the old sibling folder,
//! and on the next load Manuskript sees a zip and takes it. The folder then sits
//! there rotting, indistinguishable by eye from a live one — the cause of at least
//! one report of a writer recovering months-old text from it. Opening a folder
//! directly is exactly the gesture that walks into it, so [`ManuskriptSource::open`]
//! looks for a newer zip beside the folder and says so.
//!
//! # Reading is all this does
//!
//! Nothing here writes, moves or deletes anything in the source project, and that
//! is not incidental. Manuskript's own save deletes every file in the project
//! folder it did not itself write ("removing phantoms"), and one branch `rmtree`s
//! the folder outright before writing. A project is only ever read.
//!
//! `settings.pickle`, which format-0 projects carry, is **never deserialized**.
//! `pickle.loads` on it was CVE-2021-35196 — arbitrary code execution from an
//! attacker-supplied project file, live in Manuskript from 2015 until June 2021.
//! A `.msk` someone sends you is untrusted input, which is also why the member
//! count and total size below are bounded.

use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};
use chrono::{DateTime, Utc};

use crate::version::{self, FormatVersion};

/// The format-0 member holding a Python pickle. Read past, never parsed.
pub const PICKLE_MEMBER: &str = "settings.pickle";
/// The format-version marker, 0.3.0 onward.
const MARKER: &str = "MANUSKRIPT";
/// The marker's name for the few weeks between 0.2.0 and 0.3.0, zips only.
const LEGACY_MARKER: &str = "VERSION";

/// Upper bounds on what a single project may expand to.
///
/// Generous rather than tight — a real novel's `revisions.xml` has been measured
/// at 55 MB on its own — but present, because a `.msk` is a zip from an untrusted
/// source and an unbounded extract is a denial of service.
const MAX_MEMBERS: usize = 100_000;
const MAX_TOTAL_BYTES: u64 = 1 << 30; // 1 GiB

/// Which container the project was read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Container {
    /// A zipped `.msk`.
    Zip,
    /// A project folder, reached through its `.msk` stub or given directly.
    Folder,
}

impl Container {
    /// A short name for the import summary, so the writer can tell which of two
    /// copies was read.
    pub fn label(self) -> &'static str {
        match self {
            Container::Zip => "single file",
            Container::Folder => "folder",
        }
    }
}

/// A Manuskript project, read into memory.
pub struct ManuskriptSource {
    pub container: Container,
    pub format: FormatVersion,
    /// The project's own name, from the folder or the `.msk` stem. A title
    /// fallback for a project whose `infos.txt` carries none.
    pub project_name: String,
    /// The newest modification time seen in the container, so the summary can say
    /// how recent the data is.
    pub newest_modified: Option<DateTime<Utc>>,
    /// Everything worth telling the writer about the read itself.
    pub notices: Vec<String>,
    /// Members keyed by their normalised, `/`-separated relative path.
    files: BTreeMap<String, Vec<u8>>,
}

#[cfg(test)]
impl ManuskriptSource {
    /// An in-memory source built from `(member, content)` pairs, so a reader can
    /// be exercised without a project on disk.
    ///
    /// Bypasses [`Self::open`] deliberately: what it skips — container sniffing,
    /// version resolution, the staleness check — has its own tests, and a reader
    /// test should fail for a reason in the reader.
    pub(crate) fn for_tests(members: &[(&str, &str)]) -> Self {
        Self {
            container: Container::Folder,
            format: FormatVersion::V1,
            project_name: "Fixture".to_string(),
            newest_modified: None,
            notices: Vec::new(),
            files: members
                .iter()
                .map(|(k, v)| ((*k).to_string(), v.as_bytes().to_vec()))
                .collect(),
        }
    }
}

impl ManuskriptSource {
    /// Open the project at `path`: a zipped `.msk`, a `.msk` stub beside its
    /// folder, or the folder itself.
    pub fn open(path: &str) -> Result<Self> {
        let p = Path::new(path);
        if p.is_dir() {
            let mut src = Self::from_folder(p)?;
            src.warn_if_a_newer_zip_sits_beside(p);
            return Ok(src);
        }
        if !p.exists() {
            bail!("'{path}' does not exist");
        }
        if looks_like_a_zip(p)? {
            return Self::from_zip(p);
        }
        Self::from_stub(p)
    }

    /// A `.msk` that is not a zip is the one-byte stub naming the folder beside it.
    fn from_stub(stub: &Path) -> Result<Self> {
        let raw = fs::read(stub).with_context(|| format!("reading '{}'", stub.display()))?;
        // The stub holds a bare number. Anything longer is not one, and saying so
        // is more useful than failing later on a missing folder.
        if raw.len() > 32 {
            bail!(
                "'{}' is neither a Manuskript archive nor a project file",
                stub.display()
            );
        }
        let stem = stub
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("'{}' has no file name", stub.display()))?;
        let parent = stub
            .parent()
            .ok_or_else(|| anyhow!("'{}' has no parent directory", stub.display()))?;
        let folder = parent.join(stem);
        if !folder.is_dir() {
            bail!(
                "'{}' is a pointer to the project folder beside it, and '{}' is missing. \
                 A Manuskript project in folder mode is both, and they have to travel together.",
                stub.display(),
                folder.display()
            );
        }
        let stub_marker = String::from_utf8_lossy(&raw).trim().to_string();
        let mut src = Self::from_folder(&folder)?;
        // The folder's own marker wins; the stub answers for a folder that lost it.
        if !src.files.contains_key(MARKER) && !stub_marker.is_empty() {
            src.format = version::resolve(Some(&stub_marker))?;
        }
        Ok(src)
    }

    fn from_folder(folder: &Path) -> Result<Self> {
        let mut files = BTreeMap::new();
        let mut notices = Vec::new();
        let mut newest: Option<DateTime<Utc>> = None;
        let mut total: u64 = 0;
        read_dir_into(
            folder,
            folder,
            &mut files,
            &mut notices,
            &mut newest,
            &mut total,
        )?;

        let project_name = folder
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Manuskript project")
            .to_string();
        Self::finish(Container::Folder, project_name, files, notices, newest)
    }

    fn from_zip(archive_path: &Path) -> Result<Self> {
        let file = fs::File::open(archive_path)
            .with_context(|| format!("opening '{}'", archive_path.display()))?;
        let mut archive = zip::ZipArchive::new(file)
            .with_context(|| format!("reading '{}' as an archive", archive_path.display()))?;

        let mut files = BTreeMap::new();
        let mut notices = Vec::new();
        let mut total: u64 = 0;
        if archive.len() > MAX_MEMBERS {
            bail!(
                "'{}' holds {} entries, past the {MAX_MEMBERS} this importer will read",
                archive_path.display(),
                archive.len()
            );
        }
        for i in 0..archive.len() {
            let mut entry = archive
                .by_index(i)
                .with_context(|| format!("reading entry {i} of '{}'", archive_path.display()))?;
            if !entry.is_file() {
                continue;
            }
            let Some(name) = normalise_member(entry.name()) else {
                continue;
            };
            total = total.saturating_add(entry.size());
            if total > MAX_TOTAL_BYTES {
                bail!(
                    "'{}' expands past the {} MiB this importer will read",
                    archive_path.display(),
                    MAX_TOTAL_BYTES / (1 << 20)
                );
            }
            let mut buf = Vec::new();
            entry
                .read_to_end(&mut buf)
                .with_context(|| format!("reading '{name}' from '{}'", archive_path.display()))?;
            note_if_not_utf8(&name, &buf, &mut notices);
            files.insert(name, buf);
        }

        let project_name = archive_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Manuskript project")
            .to_string();
        let newest = modified_at(archive_path);
        Self::finish(Container::Zip, project_name, files, notices, newest)
    }

    fn finish(
        container: Container,
        project_name: String,
        files: BTreeMap<String, Vec<u8>>,
        mut notices: Vec<String>,
        newest_modified: Option<DateTime<Utc>>,
    ) -> Result<Self> {
        let marker = files
            .get(MARKER)
            .or_else(|| files.get(LEGACY_MARKER))
            .map(|b| String::from_utf8_lossy(b).into_owned());
        let format = version::resolve(marker.as_deref())?;

        let looks_like_v1 = files.keys().any(|k| k.starts_with("outline/"))
            || files.contains_key("infos.txt")
            || marker.is_some();
        let looks_like_v0 =
            files.contains_key("outline.xml") || files.contains_key("flatModel.xml");
        if !looks_like_v1 && !looks_like_v0 {
            bail!(
                "this does not look like a Manuskript project: no MANUSKRIPT marker, no \
                 outline/ folder and no outline.xml"
            );
        }

        if files.contains_key(PICKLE_MEMBER) {
            notices.push(format!(
                "'{PICKLE_MEMBER}' was left unread. It is a Python pickle, and loading one from \
                 a project file was a remote-code-execution flaw in Manuskript itself \
                 (CVE-2021-35196); the settings it holds are display preferences, not writing."
            ));
        }

        Ok(Self {
            container,
            format,
            project_name,
            newest_modified,
            notices,
            files,
        })
    }

    /// Warn when a newer zip sits beside a folder opened directly.
    ///
    /// Only reachable through the folder door, because through the `.msk` the zip
    /// would have been what we opened.
    fn warn_if_a_newer_zip_sits_beside(&mut self, folder: &Path) {
        let Some(name) = folder.file_name().and_then(|s| s.to_str()) else {
            return;
        };
        let Some(parent) = folder.parent() else {
            return;
        };
        let sibling = parent.join(format!("{name}.msk"));
        if !sibling.is_file() {
            return;
        }
        // A stub is a handful of bytes; only an actual archive means the project
        // has moved to single-file mode and left this folder behind.
        if !matches!(looks_like_a_zip(&sibling), Ok(true)) {
            return;
        }
        let (Some(zip_at), Some(folder_at)) = (modified_at(&sibling), self.newest_modified) else {
            return;
        };
        if zip_at > folder_at {
            self.notices.push(format!(
                "'{}' is newer than this folder ({} against {}). Manuskript leaves the old \
                 folder in place when a project moves to single-file mode, so this may be an \
                 abandoned copy; importing the .msk instead would read the newer one.",
                sibling.display(),
                zip_at.format("%Y-%m-%d"),
                folder_at.format("%Y-%m-%d"),
            ));
        }
    }

    /// A member's bytes, if it is present.
    pub fn bytes(&self, member: &str) -> Option<&[u8]> {
        self.files.get(member).map(Vec::as_slice)
    }

    /// A member decoded as text, lossily.
    ///
    /// Lossy because Manuskript's own zip branch decodes unconditionally and
    /// throws on a bad byte, which is a project that will not open at all; a
    /// replacement character in one scene is the better failure. Every member that
    /// needed it is already in [`Self::notices`].
    pub fn text(&self, member: &str) -> Option<String> {
        self.bytes(member)
            .map(|b| String::from_utf8_lossy(b).into_owned())
    }

    pub fn has(&self, member: &str) -> bool {
        self.files.contains_key(member)
    }

    /// Every member whose path starts with `prefix`, in sorted path order.
    ///
    /// Sorted, because that is the only thing carrying outline order: Manuskript
    /// records sibling position in a zero-padded filename prefix and keeps no index
    /// file, and its own loader relies on the same sort.
    pub fn members_under(&self, prefix: &str) -> Vec<&str> {
        self.files
            .keys()
            .filter(|k| k.starts_with(prefix))
            .map(String::as_str)
            .collect()
    }

    /// Every member, sorted. Used to report what the format did not account for.
    pub fn members(&self) -> Vec<&str> {
        self.files.keys().map(String::as_str).collect()
    }
}

/// True when the file starts with a zip local-file or end-of-central-directory
/// signature — the same sniff `skrib_format::detect_shape` uses, and the same
/// question Manuskript answers by trying to open it as an archive.
fn looks_like_a_zip(path: &Path) -> Result<bool> {
    let mut file = fs::File::open(path).with_context(|| format!("opening '{}'", path.display()))?;
    let mut magic = [0u8; 4];
    let mut read = 0usize;
    while read < magic.len() {
        match file.read(&mut magic[read..]) {
            Ok(0) => break,
            Ok(n) => read += n,
            Err(e) => return Err(e).with_context(|| format!("reading '{}'", path.display())),
        }
    }
    if read < 4 {
        return Ok(false);
    }
    Ok(&magic == b"PK\x03\x04" || &magic == b"PK\x05\x06")
}

/// Normalise a zip member name, or `None` for one we will not read.
///
/// Manuskript builds member names with `os.path.join`, so a `.msk` written on
/// Windows carries backslashes and one written on Linux carries forward slashes.
/// Its own loader split on the *local* separator, which is why a project written
/// on one platform opened with an empty outline on the other — and then saved the
/// emptiness back over it. Both separators are accepted here.
///
/// Directory entries are dropped (Manuskript writes none, but 7-Zip and friends
/// add them), as are dot-files, which Manuskript has skipped since 0.7.0 after a
/// stray `.DS_Store` crashed its load, and any `..` component.
fn normalise_member(raw: &str) -> Option<String> {
    let unified = raw.replace('\\', "/");
    let trimmed = unified.trim_start_matches('/');
    if trimmed.is_empty() || trimmed.ends_with('/') {
        return None;
    }
    let mut parts: Vec<&str> = Vec::new();
    for part in trimmed.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." || part.starts_with('.') {
            return None;
        }
        parts.push(part);
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join("/"))
    }
}

/// Walk a project folder into the member map, keyed exactly as the zip path keys.
fn read_dir_into(
    root: &Path,
    dir: &Path,
    files: &mut BTreeMap<String, Vec<u8>>,
    notices: &mut Vec<String>,
    newest: &mut Option<DateTime<Utc>>,
    total: &mut u64,
) -> Result<()> {
    let entries = fs::read_dir(dir).with_context(|| format!("reading '{}'", dir.display()))?;
    for entry in entries {
        let entry = entry.with_context(|| format!("reading an entry of '{}'", dir.display()))?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        // Dot-entries are skipped for the same reason the zip path skips them.
        if name.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            read_dir_into(root, &path, files, notices, newest, total)?;
            continue;
        }
        let Ok(relative) = path.strip_prefix(root) else {
            continue;
        };
        let Some(key) = normalise_member(&relative.to_string_lossy()) else {
            continue;
        };
        if files.len() >= MAX_MEMBERS {
            bail!("this project holds more than {MAX_MEMBERS} files");
        }
        // A file that cannot be read costs that file and nothing else. A
        // project folder is a directory on someone's disk: it collects broken
        // symlinks, half-synced files and things the current user may not read,
        // and none of that is a reason to refuse a manuscript.
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(e) => {
                notices.push(format!(
                    "'{key}' could not be read ({e}) and was skipped; everything else was \
                     imported."
                ));
                continue;
            }
        };
        *total = total.saturating_add(bytes.len() as u64);
        if *total > MAX_TOTAL_BYTES {
            bail!(
                "this project is larger than the {} MiB this importer will read",
                MAX_TOTAL_BYTES / (1 << 20)
            );
        }
        if let Some(at) = modified_at(&path)
            && newest.map(|n| at > n).unwrap_or(true)
        {
            *newest = Some(at);
        }
        note_if_not_utf8(&key, &bytes, notices);
        files.insert(key, bytes);
    }
    Ok(())
}

/// Record a member that is not valid UTF-8, so the summary can name it.
///
/// The pickle is exempt: it is binary by definition and is reported separately,
/// as something deliberately not read rather than something read imperfectly.
fn note_if_not_utf8(member: &str, bytes: &[u8], notices: &mut Vec<String>) {
    if member == PICKLE_MEMBER || std::str::from_utf8(bytes).is_ok() {
        return;
    }
    notices.push(format!(
        "'{member}' is not valid UTF-8; unreadable bytes were replaced rather than failing \
         the import."
    ));
}

fn modified_at(path: &Path) -> Option<DateTime<Utc>> {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .map(DateTime::<Utc>::from)
}

#[cfg(test)]
mod tests {

    /// A project folder is a directory on someone's disk: it collects broken
    /// symlinks and half-synced files, and none of that is a reason to refuse a
    /// manuscript.
    #[test]
    #[cfg(unix)]
    fn a_file_that_cannot_be_read_is_skipped_rather_than_fatal() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().join("A Novel");
        folder(&root, &v1_members());
        std::os::unix::fs::symlink(root.join("nowhere"), root.join("dangling.txt"))
            .expect("dangling symlink");

        let src = ManuskriptSource::open(&root.to_string_lossy()).expect("open");
        assert!(
            src.notices.iter().any(|n| n.contains("dangling.txt")),
            "{:?}",
            src.notices
        );
        // And everything else still arrived.
        assert!(src.has("MANUSKRIPT"));
        assert_eq!(src.members_under("outline/").len(), 2);
    }

    use super::*;
    use std::io::Write;
    use std::time::{Duration, SystemTime};

    /// Write a project folder from `(relative path, content)` pairs.
    fn folder(root: &Path, members: &[(&str, &str)]) {
        for (name, content) in members {
            let path = root.join(name);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("fixture dir");
            }
            fs::write(&path, content).expect("fixture file");
        }
    }

    /// Write a zip whose member names are used verbatim, so a test can pin the
    /// separator a Windows-written archive carries.
    fn zip_raw(path: &Path, members: &[(&str, &[u8])]) {
        let file = fs::File::create(path).expect("fixture zip");
        let mut zip = zip::ZipWriter::new(file);
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        for (name, content) in members {
            zip.start_file(*name, opts).expect("zip entry");
            zip.write_all(content).expect("zip write");
        }
        zip.finish().expect("zip finish");
    }

    fn zip_of(path: &Path, members: &[(&str, &str)]) {
        let owned: Vec<(&str, &[u8])> = members.iter().map(|(n, c)| (*n, c.as_bytes())).collect();
        zip_raw(path, &owned);
    }

    /// The smallest thing that is recognisably a format-1 project.
    fn v1_members() -> Vec<(&'static str, &'static str)> {
        vec![
            ("MANUSKRIPT", "1"),
            ("infos.txt", "Title:          A Novel\n"),
            (
                "outline/0-Chapter_One/folder.txt",
                "title:          Chapter One\nID:             1\ntype:           folder\n",
            ),
            (
                "outline/0-Chapter_One/0-Opening.md",
                "title:          Opening\nID:             2\ntype:           md\n\n\nProse.",
            ),
        ]
    }

    fn set_modified(path: &Path, at: SystemTime) {
        let file = fs::File::options()
            .write(true)
            .open(path)
            .expect("open for touch");
        file.set_modified(at).expect("set mtime");
    }

    #[test]
    fn a_folder_project_opens_as_format_one() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().join("A Novel");
        folder(&root, &v1_members());
        let src = ManuskriptSource::open(&root.to_string_lossy()).expect("open");
        assert_eq!(src.container, Container::Folder);
        assert_eq!(src.format, FormatVersion::V1);
        assert_eq!(src.project_name, "A Novel");
        assert_eq!(
            src.members_under("outline/"),
            [
                "outline/0-Chapter_One/0-Opening.md",
                "outline/0-Chapter_One/folder.txt"
            ]
        );
        assert!(src.newest_modified.is_some());
    }

    #[test]
    fn a_stub_msk_resolves_to_the_folder_beside_it() {
        let dir = tempfile::tempdir().expect("tempdir");
        folder(&dir.path().join("A Novel"), &v1_members());
        let stub = dir.path().join("A Novel.msk");
        fs::write(&stub, "1").expect("stub");
        let src = ManuskriptSource::open(&stub.to_string_lossy()).expect("open");
        assert_eq!(src.container, Container::Folder);
        assert_eq!(src.format, FormatVersion::V1);
        assert_eq!(src.project_name, "A Novel");
    }

    #[test]
    fn a_stub_whose_folder_is_missing_says_both_names() {
        let dir = tempfile::tempdir().expect("tempdir");
        let stub = dir.path().join("Gone.msk");
        fs::write(&stub, "1").expect("stub");
        let err = match ManuskriptSource::open(&stub.to_string_lossy()) {
            Ok(_) => panic!("expected a refusal"),
            Err(e) => e.to_string(),
        };
        assert!(err.contains("Gone.msk"), "{err}");
        assert!(err.contains("Gone"), "{err}");
        assert!(err.contains("travel together"), "{err}");
    }

    #[test]
    fn a_zipped_project_opens_and_keeps_its_stem_as_a_name() {
        let dir = tempfile::tempdir().expect("tempdir");
        let archive = dir.path().join("Zipped Novel.msk");
        zip_of(&archive, &v1_members());
        let src = ManuskriptSource::open(&archive.to_string_lossy()).expect("open");
        assert_eq!(src.container, Container::Zip);
        assert_eq!(src.format, FormatVersion::V1);
        assert_eq!(src.project_name, "Zipped Novel");
        assert_eq!(src.text("MANUSKRIPT").as_deref(), Some("1"));
    }

    /// A zip with no marker at all is format 0, which is what Manuskript's own
    /// dispatcher concludes.
    #[test]
    fn a_markerless_zip_holding_outline_xml_is_format_zero() {
        let dir = tempfile::tempdir().expect("tempdir");
        let archive = dir.path().join("Old.msk");
        zip_of(
            &archive,
            &[
                ("outline.xml", "<outlineItem title=\"Root\" ID=\"0\"/>"),
                ("flatModel.xml", "<model/>"),
            ],
        );
        let src = ManuskriptSource::open(&archive.to_string_lossy()).expect("open");
        assert_eq!(src.format, FormatVersion::V0);
    }

    /// The cross-platform bug that emptied outlines: a `.msk` written on Windows
    /// carries backslashes, and splitting on the local separator finds nothing.
    #[test]
    fn a_windows_written_archive_reads_on_a_unix_host() {
        let dir = tempfile::tempdir().expect("tempdir");
        let archive = dir.path().join("Windows.msk");
        zip_of(
            &archive,
            &[
                ("MANUSKRIPT", "1"),
                (
                    "outline\\0-Chapter_One\\folder.txt",
                    "title:          Chapter One\nID:             1\ntype:           folder\n",
                ),
            ],
        );
        let src = ManuskriptSource::open(&archive.to_string_lossy()).expect("open");
        assert_eq!(
            src.members_under("outline/"),
            ["outline/0-Chapter_One/folder.txt"]
        );
    }

    #[test]
    fn directory_entries_dot_files_and_parent_paths_are_skipped() {
        let dir = tempfile::tempdir().expect("tempdir");
        let archive = dir.path().join("Noisy.msk");
        zip_of(
            &archive,
            &[
                ("MANUSKRIPT", "1"),
                ("infos.txt", "Title:          X\n"),
                ("outline/", ""),
                (".DS_Store", "junk"),
                ("outline/.hidden", "junk"),
                ("../escape.txt", "junk"),
            ],
        );
        let src = ManuskriptSource::open(&archive.to_string_lossy()).expect("open");
        assert_eq!(src.members(), ["MANUSKRIPT", "infos.txt"]);
    }

    /// A dot-file in a folder project is skipped too, and so the whole read
    /// survives one that is not text at all.
    #[test]
    fn a_folder_project_skips_dot_entries() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().join("A Novel");
        folder(&root, &v1_members());
        fs::write(root.join(".DS_Store"), [0u8, 159, 146, 150]).expect("junk");
        let src = ManuskriptSource::open(&root.to_string_lossy()).expect("open");
        assert!(!src.members().iter().any(|m| m.contains("DS_Store")));
        assert!(src.notices.is_empty(), "{:?}", src.notices);
    }

    #[test]
    fn the_pickle_is_reported_and_never_parsed() {
        let dir = tempfile::tempdir().expect("tempdir");
        let archive = dir.path().join("Old.msk");
        // A real pickle opcode stream, which we must never hand to a deserializer.
        zip_raw(
            &archive,
            &[
                ("outline.xml", b"<outlineItem title=\"Root\" ID=\"0\"/>"),
                (PICKLE_MEMBER, b"\x80\x03}q\x00."),
            ],
        );
        let src = ManuskriptSource::open(&archive.to_string_lossy()).expect("open");
        assert_eq!(src.format, FormatVersion::V0);
        let joined = src.notices.join(" ");
        assert!(joined.contains("CVE-2021-35196"), "{joined}");
        // Present and readable as bytes, but nothing ever decodes or executes it.
        assert!(src.bytes(PICKLE_MEMBER).is_some());
        // It is binary, and must NOT also be reported as a bad-UTF-8 read.
        assert_eq!(src.notices.len(), 1, "{:?}", src.notices);
    }

    #[test]
    fn a_member_that_is_not_utf8_is_reported_and_still_readable() {
        let dir = tempfile::tempdir().expect("tempdir");
        let archive = dir.path().join("Latin1.msk");
        zip_raw(
            &archive,
            &[
                ("MANUSKRIPT", b"1"),
                ("infos.txt", b"Title:          Caf\xe9\n"),
            ],
        );
        let src = ManuskriptSource::open(&archive.to_string_lossy()).expect("open");
        assert!(
            src.notices.iter().any(|n| n.contains("infos.txt")),
            "{:?}",
            src.notices
        );
        assert!(src.text("infos.txt").unwrap_or_default().contains("Caf"));
    }

    #[test]
    fn a_newer_zip_beside_a_folder_is_reported_as_a_possible_abandoned_copy() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().join("A Novel");
        folder(&root, &v1_members());
        let archive = dir.path().join("A Novel.msk");
        zip_of(&archive, &v1_members());

        let old = SystemTime::UNIX_EPOCH + Duration::from_secs(1_500_000_000);
        for member in v1_members() {
            set_modified(&root.join(member.0), old);
        }
        set_modified(&archive, old + Duration::from_secs(60 * 60 * 24 * 365));

        let src = ManuskriptSource::open(&root.to_string_lossy()).expect("open");
        let joined = src.notices.join(" ");
        assert!(joined.contains("abandoned copy"), "{joined}");
        assert!(joined.contains("A Novel.msk"), "{joined}");
    }

    /// The same two files, opened through the `.msk`, are not a staleness case:
    /// the zip is what gets read.
    #[test]
    fn opening_the_newer_zip_itself_reports_no_staleness() {
        let dir = tempfile::tempdir().expect("tempdir");
        folder(&dir.path().join("A Novel"), &v1_members());
        let archive = dir.path().join("A Novel.msk");
        zip_of(&archive, &v1_members());
        let src = ManuskriptSource::open(&archive.to_string_lossy()).expect("open");
        assert_eq!(src.container, Container::Zip);
        assert!(src.notices.is_empty(), "{:?}", src.notices);
    }

    /// A stub beside a folder is not an archive, so it must not be mistaken for
    /// one and reported as a stale-copy warning.
    #[test]
    fn a_plain_stub_beside_its_own_folder_is_not_a_staleness_warning() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().join("A Novel");
        folder(&root, &v1_members());
        fs::write(dir.path().join("A Novel.msk"), "1").expect("stub");
        let src = ManuskriptSource::open(&root.to_string_lossy()).expect("open");
        assert!(src.notices.is_empty(), "{:?}", src.notices);
    }

    #[test]
    fn a_newer_format_marker_is_refused() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().join("Future");
        folder(
            &root,
            &[("MANUSKRIPT", "2"), ("infos.txt", "Title:          X\n")],
        );
        let err = match ManuskriptSource::open(&root.to_string_lossy()) {
            Ok(_) => panic!("expected a refusal"),
            Err(e) => e.to_string(),
        };
        assert!(err.contains('2'), "{err}");
    }

    #[test]
    fn a_folder_that_is_not_a_project_is_refused_by_name() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path().join("Photos");
        folder(&root, &[("holiday.txt", "not a project")]);
        let err = match ManuskriptSource::open(&root.to_string_lossy()) {
            Ok(_) => panic!("expected a refusal"),
            Err(e) => e.to_string(),
        };
        assert!(
            err.contains("does not look like a Manuskript project"),
            "{err}"
        );
    }

    #[test]
    fn a_path_that_does_not_exist_says_so() {
        let dir = tempfile::tempdir().expect("tempdir");
        let missing = dir.path().join("nope.msk");
        let err = match ManuskriptSource::open(&missing.to_string_lossy()) {
            Ok(_) => panic!("expected a refusal"),
            Err(e) => e.to_string(),
        };
        assert!(err.contains("does not exist"), "{err}");
    }
}
