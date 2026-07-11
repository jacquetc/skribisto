//! Backup rotation: decide which of a destination's backup files to delete.
//!
//! Two policies — keep-last-N and tiered/GFS (grandfather-father-son) — over the
//! set of a project's backups in one directory. Backups are correlated **on
//! `Work.unique_id`** (survives rename/move), and only files the manifest marks
//! `kind: Backup` are ever considered — a co-located regular `.skrib`, or another
//! project's backups sharing the folder, are never touched. `min_keep` is an
//! absolute floor: the newest `min_keep` are never deleted whatever the policy math.

use anyhow::Result;
use chrono::{DateTime, Datelike, Duration, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use super::bundle::{BundleKind, ProjectManifest};
use super::reader::peek_manifest;

/// How many backups to retain, and by what shape. Persisted in the app's backup
/// settings, so it is (de)serializable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetentionPolicy {
    /// Keep the `n` most recent backups.
    KeepLastN { n: u32 },
    /// Tiered: keep 1 per bucket for the most recent `hourly` hours, `daily` days,
    /// `weekly` weeks, `monthly` months (buckets thin out as they age).
    Gfs {
        hourly: u32,
        daily: u32,
        weekly: u32,
        monthly: u32,
    },
}

/// A backup file belonging to the project being pruned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupCandidate {
    pub path: PathBuf,
    pub timestamp: DateTime<Utc>,
    pub work_unique_id: String,
}

/// Outcome of an [`apply_retention`] sweep.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RetentionReport {
    pub deleted: Vec<PathBuf>,
    pub delete_errors: Vec<(PathBuf, String)>,
}

/// List the backups in `dir` that belong to this project (by `unique_id`, or —
/// only when the current project has none — by a `backup_of == current_path`
/// match). Non-backup `.skrib` files and other projects' backups are skipped.
pub fn scan_destination(
    dir: &Path,
    current_unique_id: &str,
    current_path_fallback: &str,
) -> Result<Vec<BackupCandidate>> {
    let mut out = Vec::new();
    let entries = std::fs::read_dir(dir)
        .map_err(|e| anyhow::anyhow!("reading backup directory {}: {}", dir.display(), e))?;

    for entry in entries.flatten() {
        let path = entry.path();
        let is_zip = path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("skrib");
        let is_folder_bundle = path.is_dir() && path.join("project.skrib").is_file();
        if !is_zip && !is_folder_bundle {
            continue;
        }
        let Some(path_str) = path.to_str() else {
            continue;
        };
        let Ok(manifest) = peek_manifest(path_str) else {
            continue; // unreadable / not a bundle → ignore
        };
        if !manifest_matches(&manifest, current_unique_id, current_path_fallback) {
            continue;
        }
        out.push(BackupCandidate {
            timestamp: candidate_timestamp(&manifest, &path),
            work_unique_id: manifest.work.unique_id.clone(),
            path,
        });
    }
    Ok(out)
}

fn manifest_matches(m: &ProjectManifest, uid: &str, path_fallback: &str) -> bool {
    if m.kind != BundleKind::Backup {
        return false;
    }
    // Prefer the stable UUID. Never match on an empty uid (would collide across
    // unrelated projects) — fall back to the recorded original path only then.
    if !uid.is_empty() && m.work.unique_id == uid {
        return true;
    }
    if m.work.unique_id.is_empty()
        && !path_fallback.is_empty()
        && m.backup_of.as_deref() == Some(path_fallback)
    {
        return true;
    }
    false
}

fn candidate_timestamp(m: &ProjectManifest, path: &Path) -> DateTime<Utc> {
    if let Some(s) = &m.backup_created_at
        && let Ok(dt) = DateTime::parse_from_rfc3339(s)
    {
        return dt.with_timezone(&Utc);
    }
    if let Some(dt) = parse_stamp_from_filename(path) {
        return dt;
    }
    // Last resort: file mtime, or the epoch (treated as oldest) if even that fails.
    file_mtime(path).unwrap_or_default()
}

/// Parse a UTC timestamp out of a `…-YYYYMMDD-HHMMSS[-N].skrib` file name.
fn parse_stamp_from_filename(path: &Path) -> Option<DateTime<Utc>> {
    let file = path.file_name()?.to_str()?;
    let stem = file.strip_suffix(".skrib").unwrap_or(file);
    let parts: Vec<&str> = stem.split('-').collect();
    let n = parts.len();
    for tail_len in [2usize, 3usize] {
        if n <= tail_len {
            continue;
        }
        let (date, time) = if tail_len == 2 {
            (parts[n - 2], parts[n - 1])
        } else {
            let extra = parts[n - 1];
            if extra.is_empty() || !extra.bytes().all(|b| b.is_ascii_digit()) {
                continue;
            }
            (parts[n - 3], parts[n - 2])
        };
        if date.len() == 8
            && time.len() == 6
            && date.bytes().all(|b| b.is_ascii_digit())
            && time.bytes().all(|b| b.is_ascii_digit())
            && let Ok(ndt) = NaiveDateTime::parse_from_str(&format!("{date}{time}"), "%Y%m%d%H%M%S")
        {
            return Some(DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc));
        }
    }
    None
}

fn file_mtime(path: &Path) -> Option<DateTime<Utc>> {
    let modified = std::fs::metadata(path).ok()?.modified().ok()?;
    Some(DateTime::<Utc>::from(modified))
}

/// Pure: given candidates + policy + `min_keep` floor + `now`, return which paths
/// to delete. No filesystem access — fully unit-testable.
pub fn plan_deletions(
    candidates: &[BackupCandidate],
    policy: &RetentionPolicy,
    min_keep: u32,
    now: DateTime<Utc>,
) -> Vec<PathBuf> {
    let mut sorted = candidates.to_vec();
    sorted.sort_by(|a, b| b.timestamp.cmp(&a.timestamp).then(a.path.cmp(&b.path)));

    let mut keep: HashSet<PathBuf> = HashSet::new();
    // Absolute floor first: the `min_keep` newest are never deleted.
    for c in sorted.iter().take(min_keep as usize) {
        keep.insert(c.path.clone());
    }

    match policy {
        RetentionPolicy::KeepLastN { n } => {
            for c in sorted.iter().take(*n as usize) {
                keep.insert(c.path.clone());
            }
        }
        RetentionPolicy::Gfs {
            hourly,
            daily,
            weekly,
            monthly,
        } => {
            keep_one_per_bucket(
                &sorted,
                |ts| ts.timestamp().div_euclid(3600),
                |ts| now.signed_duration_since(*ts) < Duration::hours(24),
                *hourly,
                &mut keep,
            );
            keep_one_per_bucket(
                &sorted,
                |ts| ts.timestamp().div_euclid(86_400),
                |ts| now.signed_duration_since(*ts) < Duration::days(7),
                *daily,
                &mut keep,
            );
            keep_one_per_bucket(
                &sorted,
                |ts| ts.timestamp().div_euclid(604_800),
                |ts| now.signed_duration_since(*ts) < Duration::weeks(4),
                *weekly,
                &mut keep,
            );
            keep_one_per_bucket(
                &sorted,
                |ts| ts.year() as i64 * 12 + ts.month0() as i64,
                |_| true,
                *monthly,
                &mut keep,
            );
        }
    }

    sorted
        .into_iter()
        .filter(|c| !keep.contains(&c.path))
        .map(|c| c.path)
        .collect()
}

/// Keep the newest candidate in each of the most recent `count` distinct buckets
/// (among those matching `eligible`). `sorted` must be newest-first.
fn keep_one_per_bucket<B, E>(
    sorted: &[BackupCandidate],
    bucket: B,
    eligible: E,
    count: u32,
    keep: &mut HashSet<PathBuf>,
) where
    B: Fn(&DateTime<Utc>) -> i64,
    E: Fn(&DateTime<Utc>) -> bool,
{
    if count == 0 {
        return;
    }
    let mut seen: HashSet<i64> = HashSet::new();
    for c in sorted {
        if !eligible(&c.timestamp) {
            continue;
        }
        let key = bucket(&c.timestamp);
        if seen.contains(&key) {
            continue; // an already-kept bucket keeps only its newest
        }
        if seen.len() as u32 >= count {
            break; // have the `count` most-recent buckets already (newest-first)
        }
        seen.insert(key);
        keep.insert(c.path.clone());
    }
}

/// Scan `dir`, plan deletions, and delete — best-effort: a failed delete is
/// collected, never panics, and never aborts the rest of the sweep.
pub fn apply_retention(
    dir: &Path,
    current_unique_id: &str,
    current_path_fallback: &str,
    policy: &RetentionPolicy,
    min_keep: u32,
) -> Result<RetentionReport> {
    let candidates = scan_destination(dir, current_unique_id, current_path_fallback)?;
    let to_delete = plan_deletions(&candidates, policy, min_keep, Utc::now());

    let mut report = RetentionReport::default();
    for p in to_delete {
        let result = if p.is_dir() {
            std::fs::remove_dir_all(&p)
        } else {
            std::fs::remove_file(&p)
        };
        match result {
            Ok(()) => report.deleted.push(p),
            Err(e) => report.delete_errors.push((p, e.to_string())),
        }
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle::{FORMAT_VERSION, ProjectManifest, ShapeTag, WorkFile};
    use std::io::Write;

    fn cand(path: &str, ts: &str) -> BackupCandidate {
        BackupCandidate {
            path: PathBuf::from(path),
            timestamp: DateTime::parse_from_rfc3339(ts)
                .unwrap()
                .with_timezone(&Utc),
            work_unique_id: "uid".into(),
        }
    }

    fn now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-06-15T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    #[test]
    fn keep_last_n_deletes_the_rest() {
        let c = vec![
            cand("/b/a-20260615-100000.skrib", "2026-06-15T10:00:00Z"),
            cand("/b/a-20260614-100000.skrib", "2026-06-14T10:00:00Z"),
            cand("/b/a-20260613-100000.skrib", "2026-06-13T10:00:00Z"),
            cand("/b/a-20260612-100000.skrib", "2026-06-12T10:00:00Z"),
        ];
        let del = plan_deletions(&c, &RetentionPolicy::KeepLastN { n: 2 }, 0, now());
        assert_eq!(
            del,
            vec![
                PathBuf::from("/b/a-20260613-100000.skrib"),
                PathBuf::from("/b/a-20260612-100000.skrib"),
            ]
        );
    }

    #[test]
    fn min_keep_floor_overrides_a_smaller_policy() {
        let c = vec![
            cand("/b/a-20260615-100000.skrib", "2026-06-15T10:00:00Z"),
            cand("/b/a-20260614-100000.skrib", "2026-06-14T10:00:00Z"),
            cand("/b/a-20260613-100000.skrib", "2026-06-13T10:00:00Z"),
        ];
        // Policy says keep 1, floor says keep 3 → nothing deleted.
        let del = plan_deletions(&c, &RetentionPolicy::KeepLastN { n: 1 }, 3, now());
        assert!(del.is_empty());
    }

    #[test]
    fn gfs_thins_older_buckets() {
        // Three today (same day, different hours), plus older days/weeks/months.
        let c = vec![
            cand("/b/a-20260615-110000.skrib", "2026-06-15T11:00:00Z"),
            cand("/b/a-20260615-100000.skrib", "2026-06-15T10:00:00Z"),
            cand("/b/a-20260615-090000.skrib", "2026-06-15T09:00:00Z"),
            cand("/b/a-20260610-090000.skrib", "2026-06-10T09:00:00Z"), // this week, older day
            cand("/b/a-20260501-090000.skrib", "2026-05-01T09:00:00Z"), // last month
            cand("/b/a-20260401-090000.skrib", "2026-04-01T09:00:00Z"), // two months ago
        ];
        let policy = RetentionPolicy::Gfs {
            hourly: 2,
            daily: 3,
            weekly: 2,
            monthly: 2,
        };
        let del = plan_deletions(&c, &policy, 0, now());
        // hourly (2 most-recent hour-buckets today) keeps 11:00 & 10:00; today's 09:00
        // falls to the daily tier but that day-bucket is already covered by 11:00, so
        // 09:00 is dropped. daily also keeps 06-10. weekly adds nothing new. monthly=2
        // keeps the two most-recent months (June via 11:00, May via 05-01), so April's
        // 04-01 is dropped. Net deletions: today's 09:00 and 04-01.
        assert!(del.contains(&PathBuf::from("/b/a-20260615-090000.skrib")));
        assert!(del.contains(&PathBuf::from("/b/a-20260401-090000.skrib")));
        assert_eq!(del.len(), 2);
    }

    // --- scan_destination cross-project isolation (real zips with a manifest) ---

    fn write_backup_zip(
        dir: &Path,
        name: &str,
        kind: BundleKind,
        uid: &str,
        backup_of: Option<&str>,
        stamp: &str,
    ) -> PathBuf {
        let manifest = ProjectManifest {
            format_version: FORMAT_VERSION,
            shape: ShapeTag::Zip,
            work: WorkFile {
                file_id: 1,
                created_at: String::new(),
                updated_at: String::new(),
                title: "T".into(),
                author_name: String::new(),
                dict_language: String::new(),
                tag_ids: vec![],
                dict_word_ids: vec![],
                unique_id: uid.into(),
                chapter_flat: false,
            },
            binder_order: vec![],
            kind,
            backup_of: backup_of.map(String::from),
            backup_created_at: Some(stamp.to_string()),
        };
        let ron = ron::ser::to_string(&manifest).unwrap();
        let path = dir.join(name);
        let file = std::fs::File::create(&path).unwrap();
        let mut zw = zip::ZipWriter::new(file);
        zw.start_file("project.skrib", zip::write::SimpleFileOptions::default())
            .unwrap();
        zw.write_all(ron.as_bytes()).unwrap();
        zw.finish().unwrap();
        path
    }

    #[test]
    fn scan_isolates_by_unique_id_in_a_shared_folder() {
        let dir = tempfile::tempdir().unwrap();
        // Two backups for project A, one for project B (same stem "novel"),
        // plus a regular (non-backup) file that shares the naming.
        write_backup_zip(
            dir.path(),
            "novel-20260615-100000.skrib",
            BundleKind::Backup,
            "A",
            Some("/x/novel.skrib"),
            "2026-06-15T10:00:00Z",
        );
        write_backup_zip(
            dir.path(),
            "novel-20260614-100000.skrib",
            BundleKind::Backup,
            "A",
            Some("/x/novel.skrib"),
            "2026-06-14T10:00:00Z",
        );
        write_backup_zip(
            dir.path(),
            "novel-20260613-100000.skrib",
            BundleKind::Backup,
            "B",
            Some("/y/novel.skrib"),
            "2026-06-13T10:00:00Z",
        );
        write_backup_zip(
            dir.path(),
            "novel-20260612-100000.skrib",
            BundleKind::Regular,
            "A",
            None,
            "2026-06-12T10:00:00Z",
        );

        let found = scan_destination(dir.path(), "A", "/x/novel.skrib").unwrap();
        assert_eq!(found.len(), 2, "only project A's two backups");
        assert!(found.iter().all(|c| c.work_unique_id == "A"));

        // keep-last-1 deletes the older A backup, never B's or the regular file.
        let del = plan_deletions(&found, &RetentionPolicy::KeepLastN { n: 1 }, 0, now());
        assert_eq!(del, vec![dir.path().join("novel-20260614-100000.skrib")]);
    }
}
