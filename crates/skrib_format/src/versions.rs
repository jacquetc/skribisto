// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading a project's past: one API over every place old prose is kept.
//!
//! Two sources answer different questions, and the writer should not have to know
//! which one they are looking at:
//!
//! * **backups** — deep and sparse. They reach back months, but only capture at
//!   the cadence backups run, so they answer *"what did this scene say last week"*.
//! * **the in-project [history log](crate::history)** — dense and shallow. It
//!   captures every save, so it answers *"what did it say this morning"*.
//!
//! `VersionSource` is the seam between them. A consumer asks for versions of an
//! item and gets one merged, de-duplicated timeline; adding a third source later
//! (or a fabricated one for the mocks build) means implementing this trait and
//! nothing else.
//!
//! ## Why reading a backup is cheap
//!
//! A `.skrib` is a zip, and a zip is random-access by construction. Every backup
//! already carries, in `binders/<NN-slug>/items.ron`, each row's **durable `uid`**
//! plus the bundle-root-relative `path` of each prose blob — and that path *is* the
//! zip entry name (`slug::prose_relpath` formats it with forward slashes;
//! `zip_io::zip_dir` derives entry names by the same rule). So the index of one
//! backup costs a handful of small entry reads, never `read_bundle`, which extracts
//! the whole archive to a tempdir and pulls every image into memory besides.
//!
//! Two mechanical constraints, both verified against the vendored `zip` 8.6 rather
//! than assumed, are baked into the code below:
//!
//! 1. `items.ron` cannot be fetched by name. Its directory is `binders/NN-slug`
//!    where the slug derives from the binder's *own name* — which lives inside the
//!    file being sought — and the manifest carries only `binder_order` (file ids,
//!    no names). So entries are **enumerated**, not addressed.
//! 2. `ZipArchive::file_names` borrows the archive immutably while `by_name` needs
//!    it mutably, so the names must be collected into owned `String`s first. This
//!    is a borrow-checker requirement, not a style choice.
//!
//! ## The correlation key
//!
//! Versions of "the same scene" are matched on **`(BinderItemFile.uid,
//! ContentRole)`**, resolved freshly from each version's own `items.ron` — never on
//! a path or a `file_id`. `file_id` is the store's `EntityId`, re-minted on every
//! `load_work`, and the prose file name embeds both it and the item's title slug.
//! Since backups are typically one per app session, *every* adjacent pair is a
//! cross-session pair: comparing by path would report a change at every boundary
//! and miss real ones. This is the single easiest thing to get wrong here.

use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

use super::bundle::{CommentFile, ItemsFile};
use super::history::{self, HistoryLog};
use super::shape::{SkribShape, detect_shape, folder_root};
use common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};

/// Where a version came from. Shown to the writer, because "this exists only in a
/// backup on a drive you last plugged in a month ago" is a materially different
/// promise from "this is in your project file".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SourceKind {
    /// A routine backup bundle.
    Backup,
    /// The project's own history log.
    Log,
}

/// One point in time that can be read back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionRef {
    /// The bundle to read from. For [`SourceKind::Log`] this is the live project.
    pub path: PathBuf,
    pub taken_at: DateTime<Utc>,
    pub source: SourceKind,
}

/// What a blob costs, without decompressing it.
///
/// Size only. This used to carry the zip entry's CRC-32 as well, as a pre-filter:
/// equal CRC to the previous state ⇒ unchanged, skip the read. That trusts a
/// checksum in the one direction it cannot be trusted — a mismatch proves
/// difference, a match proves nothing — and the failure it admitted was the worst
/// one here: a real edit silently folded into the state before it. It also bought
/// little, since a differing CRC still needs the hash, which is the identity the
/// sources are merged on. See [`crate::changes`].
///
/// `bytes` is not used to decide anything; it is what the list rows and the
/// timeline's sparkline show, straight from the central directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobStamp {
    pub bytes: u64,
}

/// One row as it existed in one version.
#[derive(Debug, Clone, PartialEq)]
pub struct VersionRow {
    pub uid: uuid::Uuid,
    pub title: String,
    /// The second name a Book carries — its subtitle.
    ///
    /// Carried for the same reason [`Self::role`] is, and it is as easy to miss:
    /// it is a plain field on `BinderItemFile`, not a prose blob, so nothing that
    /// walks `prose` would ever notice it was gone. A Book put back without it
    /// comes back with its subtitle silently blank, and the writer has no version
    /// left to read it out of.
    pub sub_title: String,
    /// Container or leaf, as of this moment.
    ///
    /// Read straight off the bundle and carried rather than derived: `sub_role`
    /// does not determine it. `Part`, `ChapterScene` and `Paratext` are each valid
    /// under **both** `Item` and `Folder` in `skribisto_model`'s constraint
    /// matrix, so a row put back without this could return as a flat marker where
    /// it was the container holding the rest of the chapter.
    ///
    /// Defaulted, not recorded, by [`LogVersions`] — the log stores prose, not
    /// metadata — which is why anything reconstructing a row from a version has to
    /// come from a backup.
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    pub indent: i64,
    /// `(role, bundle-relative blob path, stamp)` — one per prose content role.
    pub prose: Vec<(ContentRole, String, BlobStamp)>,
}

impl VersionRow {
    /// The blob path and stamp for one content role, if this row had one.
    pub fn prose_for(&self, role: &ContentRole) -> Option<(&str, BlobStamp)> {
        self.prose
            .iter()
            .find(|(r, _, _)| r == role)
            .map(|(_, p, s)| (p.as_str(), *s))
    }
}

/// Every row of one version, plus when it was taken.
#[derive(Debug, Clone, PartialEq)]
pub struct VersionIndex {
    pub taken_at: DateTime<Utc>,
    pub rows: Vec<VersionRow>,
}

impl VersionIndex {
    pub fn row(&self, uid: uuid::Uuid) -> Option<&VersionRow> {
        self.rows.iter().find(|r| r.uid == uid)
    }
}

/// What one source has to say about one row's one content role at one moment.
///
/// Three answers, not two, and the third is the load-bearing one: **"I have no
/// record of it" and "it did not exist" are different statements**, and only some
/// sources are entitled to make the second. A backup is a complete copy of the
/// project, so prose missing from it really was missing. The history log is not a
/// copy of anything — [`crate::history::thin`] drops a row's oldest states as they
/// age — so its silence about an old moment means "no surviving record", which is
/// not evidence about the row at all.
#[derive(Debug, Clone, PartialEq)]
pub enum RowAt {
    /// It was there, and this is where to read it.
    Present {
        /// Bundle-root-relative path of the prose blob.
        blob_path: String,
        stamp: BlobStamp,
        /// The row's title as of this moment, where the source records one.
        title: String,
    },
    /// Read, and it was genuinely not in it.
    Absent,
    /// Read, but this source cannot testify about this moment. Says nothing.
    Silent,
}

/// A place old prose can be read from.
pub trait VersionSource {
    /// Every readable point in time, newest first.
    fn list(&self) -> Result<Vec<VersionRef>>;
    /// Every row of one version — small reads only, never a whole-bundle parse.
    fn index(&self, v: &VersionRef) -> Result<VersionIndex>;
    /// What this source says about **one** row's **one** content role at one moment.
    ///
    /// The whole-index default is right for any source that is a complete copy of
    /// the project. A source that can answer for one row without building the whole
    /// index should override it — [`LogVersions`] does, because the default made
    /// building one row's timeline cost the *project's* whole recorded history,
    /// once per examined moment.
    fn row_at(&self, v: &VersionRef, uid: uuid::Uuid, role: &ContentRole) -> Result<RowAt> {
        let index = self.index(v)?;
        let Some(row) = index.row(uid) else {
            return Ok(RowAt::Absent);
        };
        Ok(match row.prose_for(role) {
            Some((blob_path, stamp)) => RowAt::Present {
                blob_path: blob_path.to_string(),
                stamp,
                title: row.title.clone(),
            },
            None => RowAt::Absent,
        })
    }
    /// How many recorded states of **one** row's **one** content role this source
    /// has removed as they aged.
    ///
    /// Zero by default, and that is the honest answer for a source that does not
    /// thin per row: a backup file is swept whole or not at all, so a backup that
    /// is gone took every row in it and left no per-row tally to report. Only
    /// [`LogVersions`] overrides it — [`crate::history::thin`] works one
    /// `(row, role)` at a time and now keeps the count.
    ///
    /// This is the *only* way the fact is knowable. [`RowAt::Silent`] is
    /// deliberately inert in the merge walk (see [`crate::changes`]), because a
    /// thinned moment must not read as a gap — which leaves the survivors of a
    /// sweep indistinguishable from a row that never had more. A tally recorded
    /// at the moment of the deletion is what a surface can stand behind; anything
    /// inferred from what is left would be a guess.
    fn thinned_away(&self, _uid: uuid::Uuid, _role: &ContentRole) -> u32 {
        0
    }
    /// One blob's prose.
    fn prose(&self, v: &VersionRef, blob_path: &str) -> Result<String>;
    /// The comment thread stored beside one prose blob, if any.
    ///
    /// Free, and nothing else in the category offers it: comment sidecars are
    /// derivable from the blob's own name (`…djot` → `….comments.ron`), so the
    /// same read path that recovers old prose also recovers *what the note on that
    /// paragraph said before it was resolved*.
    fn comments(&self, v: &VersionRef, blob_path: &str) -> Result<Vec<CommentFile>>;
}

// ── backups ────────────────────────────────────────────────────────────────────

/// Versions read out of a project's routine backups.
pub struct BackupVersions {
    /// Directories to scan, already resolved (an empty string means "beside the
    /// project", exactly as the backup engine reads it).
    pub directories: Vec<String>,
    pub work_unique_id: String,
    pub project_path: String,
}

impl VersionSource for BackupVersions {
    fn list(&self) -> Result<Vec<VersionRef>> {
        let mut out: Vec<VersionRef> = Vec::new();
        let mut seen: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();
        for dir in &self.directories {
            let dir = if dir.trim().is_empty() {
                match Path::new(&self.project_path).parent() {
                    Some(p) if !p.as_os_str().is_empty() => p.to_path_buf(),
                    _ => continue,
                }
            } else {
                PathBuf::from(dir)
            };
            // An unreadable destination (an unplugged stick, a revoked permission)
            // yields no versions rather than failing the whole timeline — the same
            // posture the backups browser already takes.
            let Ok(found) =
                super::retention::scan_destination(&dir, &self.work_unique_id, &self.project_path)
            else {
                continue;
            };
            for c in found {
                if seen.insert(c.path.clone()) {
                    out.push(VersionRef {
                        path: c.path,
                        taken_at: c.timestamp,
                        source: SourceKind::Backup,
                    });
                }
            }
        }
        out.sort_by_key(|v| std::cmp::Reverse(v.taken_at));
        Ok(out)
    }

    fn index(&self, v: &VersionRef) -> Result<VersionIndex> {
        let path = v.path.to_string_lossy().into_owned();
        let rows = match detect_shape(&path)? {
            SkribShape::ExplodedFolder => folder_rows(&folder_root(&path))?,
            SkribShape::ZipFile => zip_rows(&v.path)?,
            SkribShape::LegacySqlite => {
                anyhow::bail!("'{path}' is a legacy file and carries no per-row history")
            }
        };
        Ok(VersionIndex {
            taken_at: v.taken_at,
            rows,
        })
    }

    fn prose(&self, v: &VersionRef, blob_path: &str) -> Result<String> {
        read_entry(&v.path, blob_path)
    }

    fn comments(&self, v: &VersionRef, blob_path: &str) -> Result<Vec<CommentFile>> {
        let sidecar = comments_sidecar(blob_path);
        match read_entry(&v.path, &sidecar) {
            Ok(text) => Ok(ron::from_str(&text).unwrap_or_default()),
            // No sidecar means no comments — the common case, not a failure.
            Err(_) => Ok(Vec::new()),
        }
    }
}

/// `12-the-lamp.scene.djot` → `12-the-lamp.scene.comments.ron`, the naming
/// `folder_io::comments_file_name` already writes.
fn comments_sidecar(blob_path: &str) -> String {
    let stem = blob_path.strip_suffix(".djot").unwrap_or(blob_path);
    format!("{stem}.comments.ron")
}

/// Read one entry out of a bundle, whatever shape it is, without parsing the rest.
fn read_entry(bundle: &Path, rel: &str) -> Result<String> {
    let path = bundle.to_string_lossy().into_owned();
    match detect_shape(&path)? {
        SkribShape::ExplodedFolder => {
            let full = folder_root(&path).join(rel);
            std::fs::read_to_string(&full).with_context(|| format!("reading {}", full.display()))
        }
        SkribShape::ZipFile => {
            let file = std::fs::File::open(bundle)
                .with_context(|| format!("opening {}", bundle.display()))?;
            let mut archive = zip::ZipArchive::new(file)
                .with_context(|| format!("reading zip {}", bundle.display()))?;
            let mut entry = archive
                .by_name(rel)
                .with_context(|| format!("no '{rel}' in {}", bundle.display()))?;
            let mut text = String::new();
            entry.read_to_string(&mut text)?;
            Ok(text)
        }
        SkribShape::LegacySqlite => anyhow::bail!("'{path}' is a legacy file"),
    }
}

/// Rows of an exploded-folder bundle: read each binder's `items.ron` and stat its
/// blobs.
fn folder_rows(root: &Path) -> Result<Vec<VersionRow>> {
    let mut rows = Vec::new();
    let binders_dir = root.join("binders");
    let Ok(entries) = std::fs::read_dir(&binders_dir) else {
        return Ok(rows); // a bundle with no binders is legal, if odd
    };
    let mut dirs: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    dirs.sort();
    for d in dirs {
        let items_path = d.join("items.ron");
        let Ok(text) = std::fs::read_to_string(&items_path) else {
            continue;
        };
        let itf: ItemsFile =
            ron::from_str(&text).with_context(|| format!("parsing {}", items_path.display()))?;
        for item in itf.items {
            let prose = item
                .prose_refs
                .iter()
                .map(|pr| {
                    let bytes = std::fs::metadata(root.join(&pr.path))
                        .map(|m| m.len())
                        .unwrap_or(0);
                    (pr.role.clone(), pr.path.clone(), BlobStamp { bytes })
                })
                .collect();
            rows.push(row_from(item, prose)?);
        }
    }
    Ok(rows)
}

/// Rows of a zip bundle, read from the central directory plus the `items.ron`
/// entries — never an extraction.
fn zip_rows(path: &Path) -> Result<Vec<VersionRow>> {
    let file = std::fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut archive =
        zip::ZipArchive::new(file).with_context(|| format!("reading zip {}", path.display()))?;

    // Collect first: `file_names` borrows the archive immutably and `by_name`
    // needs it mutably, so the two cannot overlap.
    let mut items_entries: Vec<String> = archive
        .file_names()
        .filter(|n| n.starts_with("binders/") && n.ends_with("/items.ron"))
        .map(str::to_string)
        .collect();
    items_entries.sort();

    let mut stamps: BTreeMap<String, BlobStamp> = BTreeMap::new();
    let mut rows = Vec::new();
    for name in &items_entries {
        let mut text = String::new();
        archive
            .by_name(name)
            .with_context(|| format!("reading {name} from {}", path.display()))?
            .read_to_string(&mut text)?;
        let itf: ItemsFile = ron::from_str(&text)
            .with_context(|| format!("parsing {name} from {}", path.display()))?;
        for item in itf.items {
            let mut prose = Vec::new();
            for pr in &item.prose_refs {
                // Opening an entry seeks to its header; it does not decompress.
                // `.size()` is the accessor — there is no `uncompressed_size()`.
                let stamp = match stamps.get(&pr.path) {
                    Some(s) => *s,
                    None => {
                        let s = archive
                            .by_name(&pr.path)
                            .map(|f| BlobStamp { bytes: f.size() })
                            .unwrap_or(BlobStamp { bytes: 0 });
                        stamps.insert(pr.path.clone(), s);
                        s
                    }
                };
                prose.push((pr.role.clone(), pr.path.clone(), stamp));
            }
            rows.push(row_from(item, prose)?);
        }
    }
    Ok(rows)
}

/// Build a [`VersionRow`], refusing a row with no durable identity.
///
/// A nil `uid` cannot be correlated with anything, and quietly keeping such rows
/// would let several unrelated items collapse onto one timeline — showing a writer
/// another scene's prose as their own history. Better to fail the whole index and
/// say why.
fn row_from(
    item: super::bundle::BinderItemFile,
    prose: Vec<(ContentRole, String, BlobStamp)>,
) -> Result<VersionRow> {
    anyhow::ensure!(
        !item.uid.is_nil(),
        "row '{}' carries no uid; this bundle predates durable row identity and has \
         no per-row history that can be correlated",
        item.title,
    );
    Ok(VersionRow {
        uid: item.uid,
        title: item.title,
        sub_title: item.sub_title,
        role: item.role,
        sub_role: item.sub_role,
        indent: item.indent,
        prose,
    })
}

// ── the in-project log ─────────────────────────────────────────────────────────

/// Versions read out of the project's own [history log](crate::history).
///
/// Denser than backups and always present, but shallower: [`history::thin`] keeps
/// a bounded past. Its "rows" are synthesised from log entries rather than read
/// from an `items.ron`, so a version here knows the row's uid and role but not the
/// title it had at the time — the log records prose, not metadata.
pub struct LogVersions {
    pub log: HistoryLog,
    pub project_path: String,
    /// Each entry's parsed timestamp, by entry index. `None` for an unparseable
    /// one, which is then invisible to every query here — the same treatment
    /// [`crate::history::thin`] gives it.
    ///
    /// Parsed **once**, at [`Self::open`]. It used to be re-parsed inside both
    /// `list` and `index`, and `index` runs once per examined moment, so a project
    /// with S saves and E entries parsed S×E timestamps to build one row's
    /// timeline.
    stamps: Vec<Option<DateTime<Utc>>>,
    /// Entry indices per `(row uid, prose kind)`, in `entries` order.
    ///
    /// This is what makes one row's history cost that row's history. Without it
    /// every query walked the whole project's log — and the log pools every row's
    /// past, so the cost of opening the Versions dock on a single scene grew with
    /// how much the writer had edited *everything else*.
    ///
    /// Keyed at `(uid, kind)` and not at `uid`, because that is the granularity
    /// [`crate::history::thin`] works at, and therefore the granularity at which
    /// "the log has nothing here" has to be interpreted. See [`Self::row_at`].
    by_key: BTreeMap<(uuid::Uuid, &'static str), Vec<usize>>,
}

/// The log's own name for a content role — the on-disk authority, and what
/// [`crate::history`] keys its retention on.
fn kind_of(role: &ContentRole) -> &'static str {
    super::slug::prose_kind(role).unwrap_or("other")
}

impl LogVersions {
    /// Read the log out of the project at `project_path`.
    pub fn open(project_path: &str) -> Self {
        let log = history::load(project_path);
        let stamps: Vec<Option<DateTime<Utc>>> = log
            .entries
            .iter()
            .map(|e| {
                DateTime::parse_from_rfc3339(&e.at)
                    .ok()
                    .map(|d| d.with_timezone(&Utc))
            })
            .collect();
        let mut by_key: BTreeMap<(uuid::Uuid, &'static str), Vec<usize>> = BTreeMap::new();
        for (i, e) in log.entries.iter().enumerate() {
            by_key
                .entry((e.item_uid, kind_of(&e.role)))
                .or_default()
                .push(i);
        }
        Self {
            log,
            project_path: project_path.to_string(),
            stamps,
            by_key,
        }
    }

    /// The newest recorded entry for one `(row, prose kind)` at or before `at`.
    ///
    /// Compares timestamps rather than trusting `entries` to be in order: it is
    /// append-only in normal operation, but a log carried through a restore has no
    /// such guarantee, and "last one wins" would then pick an older state.
    fn newest_for(
        &self,
        key: (uuid::Uuid, &'static str),
        at: DateTime<Utc>,
    ) -> Option<&super::history::HistoryEntry> {
        let mut newest: Option<(DateTime<Utc>, &super::history::HistoryEntry)> = None;
        for &i in self.by_key.get(&key)? {
            let Some(stamp) = self.stamps[i] else {
                continue;
            };
            if stamp > at {
                continue;
            }
            // `>=` so an equal stamp lets the later entry win, which is what the
            // append order meant before this compared timestamps at all.
            if newest.is_none_or(|(seen, _)| stamp >= seen) {
                newest = Some((stamp, &self.log.entries[i]));
            }
        }
        newest.map(|(_, e)| e)
    }

    /// The entry this log wrote for one `(row, prose kind)` **at exactly** `at`, if
    /// it wrote one. The moments [`Self::list`] offers are these moments, so this
    /// is an equality and not a search backwards. See [`Self::row_at`] for why the
    /// difference matters.
    fn recorded_at(
        &self,
        key: (uuid::Uuid, &'static str),
        at: DateTime<Utc>,
    ) -> Option<&super::history::HistoryEntry> {
        // Last one wins, matching `record`'s append order in the pathological case
        // of two entries for one key at one instant.
        self.by_key
            .get(&key)?
            .iter()
            .rev()
            .find(|&&i| self.stamps[i] == Some(at))
            .map(|&i| &self.log.entries[i])
    }
}

fn prose_of(e: &super::history::HistoryEntry) -> (ContentRole, String, BlobStamp) {
    (
        e.role.clone(),
        history::blob_relpath(&e.hash),
        BlobStamp { bytes: e.bytes },
    )
}

/// A synthesised row: the log stores prose, not metadata, so it has no record of
/// what the row was *called* at the time. Callers pair a log version with the live
/// tree for titles.
fn log_row(uid: uuid::Uuid, prose: Vec<(ContentRole, String, BlobStamp)>) -> VersionRow {
    VersionRow {
        uid,
        title: String::new(),
        sub_title: String::new(),
        role: BinderItemRole::default(),
        sub_role: BinderItemSubRole::default(),
        indent: 0,
        prose,
    }
}

impl VersionSource for LogVersions {
    fn list(&self) -> Result<Vec<VersionRef>> {
        let mut stamps: Vec<DateTime<Utc>> = self.stamps.iter().flatten().copied().collect();
        stamps.sort_unstable();
        stamps.dedup();
        stamps.reverse();
        Ok(stamps
            .into_iter()
            .map(|taken_at| VersionRef {
                path: PathBuf::from(&self.project_path),
                taken_at,
                source: SourceKind::Log,
            })
            .collect())
    }

    fn index(&self, v: &VersionRef) -> Result<VersionIndex> {
        // Every row's newest state at or before this moment — which is what "the
        // project as of then" means for an append-only log.
        let mut by_uid: BTreeMap<uuid::Uuid, Vec<(ContentRole, String, BlobStamp)>> =
            BTreeMap::new();
        for &key in self.by_key.keys() {
            if let Some(e) = self.newest_for(key, v.taken_at) {
                by_uid.entry(key.0).or_default().push(prose_of(e));
            }
        }

        Ok(VersionIndex {
            taken_at: v.taken_at,
            rows: by_uid.into_iter().map(|(uid, p)| log_row(uid, p)).collect(),
        })
    }

    /// One row's one role, without building an index of the whole project — and
    /// **only for the moments this log actually recorded**.
    ///
    /// This is deliberately a narrower question than [`Self::index`] answers, and
    /// the difference is the whole point. `index` reconstructs "the project as of
    /// then" by carrying each row's newest earlier state forward, which is what the
    /// Timeline band wants. A per-row timeline wants something else: **testimony**,
    /// and this log is only a witness to the moments it wrote an entry at.
    ///
    /// Carrying a state forward here, or reading silence as absence, both turn a
    /// record of *changes* into a claim about *existence* that it cannot support:
    ///
    /// * Silence before a row's oldest surviving entry may mean the row did not
    ///   exist yet — or that [`crate::history::thin`] dropped its older states, as
    ///   it is designed to. Answering [`RowAt::Absent`] there tells a writer
    ///   "Didn't exist yet on `<date>`" about a scene they wrote years earlier.
    /// * A row that is *deleted* keeps its recorded states forever (`thin` never
    ///   empties a key that had entries), so carrying the newest one forward
    ///   reports the row as present at every later save — which erased the
    ///   "deleted after `<date>`" a backup had correctly established.
    ///
    /// [`RowAt::Silent`] is the honest answer to both, and it costs nothing: a
    /// backup **is** a complete copy of the project, so absence and deletion are
    /// established from backups, which run on close and every couple of hours by
    /// default. What the log adds — the dense record of *when the prose actually
    /// changed*, at save granularity rather than backup granularity — is exactly
    /// what it can prove, and is unaffected.
    fn row_at(&self, v: &VersionRef, uid: uuid::Uuid, role: &ContentRole) -> Result<RowAt> {
        Ok(match self.recorded_at((uid, kind_of(role)), v.taken_at) {
            Some(e) => RowAt::Present {
                blob_path: history::blob_relpath(&e.hash),
                stamp: BlobStamp { bytes: e.bytes },
                // The log stores prose, not metadata: it has no record of what the
                // row was called at the time.
                title: String::new(),
            },
            None => RowAt::Silent,
        })
    }

    /// The tally [`crate::history::thin`] left behind for this `(row, role)`.
    ///
    /// The counterpart to [`Self::row_at`]'s [`RowAt::Silent`]: that answer keeps
    /// the log from *claiming* anything about a moment it no longer holds, and
    /// this one is how the same silence can still be accounted for out loud.
    fn thinned_away(&self, uid: uuid::Uuid, role: &ContentRole) -> u32 {
        self.log.thinned_away(uid, role)
    }

    fn prose(&self, _v: &VersionRef, blob_path: &str) -> Result<String> {
        let hash = Path::new(blob_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        self.log
            .blobs
            .get(hash)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("history blob {hash} is not in the log"))
    }

    fn comments(&self, _v: &VersionRef, _blob_path: &str) -> Result<Vec<CommentFile>> {
        // The log deliberately stores prose only. Comments live in sidecars beside
        // the *bundle's* blobs, which a backup carries and this log does not —
        // recording them here would double the log's size for a far rarer question.
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_comment_sidecar_is_derived_from_its_blob_name() {
        assert_eq!(
            comments_sidecar("binders/01-manuscript/text/12-the-lamp.scene.djot"),
            "binders/01-manuscript/text/12-the-lamp.scene.comments.ron",
        );
        // Defensive: a name without the expected suffix still produces something
        // addressable rather than panicking.
        assert_eq!(comments_sidecar("odd"), "odd.comments.ron");
    }

    /// A nil uid must fail the index loudly.
    ///
    /// Silently keeping such a row would let several unrelated items collapse onto
    /// one key and show a writer another scene's prose as their own past — the
    /// worst possible failure for a feature whose entire promise is fidelity.
    #[test]
    fn a_row_without_a_durable_uid_is_refused_rather_than_silently_merged() {
        let bundle = crate::tests::build_bundle(crate::bundle::ShapeTag::Zip);
        let mut item = bundle.binders[0].items[0].item.clone();
        assert!(!item.uid.is_nil(), "the fixture itself must carry a uid");
        assert!(row_from(item.clone(), Vec::new()).is_ok());

        item.uid = uuid::Uuid::nil();
        let err = row_from(item, Vec::new()).expect_err("a nil uid must not produce a row");
        assert!(
            err.to_string().contains("no uid"),
            "the refusal must say why, got: {err}",
        );
    }
}
