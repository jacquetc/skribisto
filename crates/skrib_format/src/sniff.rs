// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Classify whether a `.skrib` path is a point-in-time **backup** copy.
//!
//! The authoritative signal is the manifest's [`BundleKind::Backup`] marker
//! ([`super::reader::peek_manifest`], cheap — no full extract). The
//! `<stem>-<YYYYMMDD-HHMMSS>[-N].skrib` filename is only a **fallback**, used when
//! the manifest can't be read at all (legacy SQLite / corrupt bundle) — never to
//! override a readable `Regular` manifest, so a real project that merely happens
//! to be named like a backup is never mistaken for one.
//!
//! Even in the fallback case, a filename match alone is not enough: it is only
//! accepted when the *guessed original* path actually exists on disk. A legacy
//! project genuinely named e.g. `draft-20260615-100000.skrib` (unreadable, so
//! the manifest can't rule it out) would otherwise be forced into read-only
//! backup mode forever, with no original to point back to. A backup whose
//! original is gone is indistinguishable from a project that merely looks like
//! one, so — since we can't even parse the file — the safe default is "this is
//! a normal project": wrongly entering read-only backup mode is far worse for
//! the user than missing the banner.

use std::path::Path;

use super::bundle::BundleKind;
use super::reader::peek_manifest;

/// The outcome of sniffing a path for "is this a backup?".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupSniff {
    pub is_backup: bool,
    /// The original project's path — from the manifest (`backup_of`), or guessed
    /// from the filename fallback (same directory, `-<stamp>` stripped).
    pub backup_of: Option<String>,
    /// The backup's own creation timestamp (RFC3339), when the manifest carried it.
    pub backup_created_at: Option<String>,
    /// `true` when the manifest's `kind: Backup` (or a readable `Regular`) decided
    /// it; `false` when only the filename pattern could be consulted (a guess).
    pub authoritative: bool,
}

impl BackupSniff {
    fn not_backup(authoritative: bool) -> Self {
        BackupSniff {
            is_backup: false,
            backup_of: None,
            backup_created_at: None,
            authoritative,
        }
    }
}

/// Classify `path`.
///
/// - Readable manifest `kind: Backup` → an authoritative backup.
/// - Readable manifest `Regular` → authoritatively **not** a backup (a real
///   project is never dragged into backup mode by its name).
/// - Manifest unreadable (legacy SQLite / corrupt) → fall back to the
///   `<stem>-<YYYYMMDD-HHMMSS>[-N].skrib` filename convention, but only when
///   the guessed original path **actually exists on disk**; a match whose
///   original is gone is indistinguishable from a project that merely looks
///   like one, so it is treated as "not a backup" rather than trapping the
///   user in permanent read-only mode. A match with an existing original is a
///   non-authoritative guess (the restore flow re-confirms the original path).
///
/// Never errors — an odd file is simply "not a backup".
pub fn sniff_backup(path: &str) -> BackupSniff {
    match peek_manifest(path) {
        Ok(m) if m.kind == BundleKind::Backup => BackupSniff {
            is_backup: true,
            backup_of: m.backup_of,
            backup_created_at: m.backup_created_at,
            authoritative: true,
        },
        Ok(_) => BackupSniff::not_backup(true),
        Err(_) => match sniff_backup_filename(path) {
            Some(backup_of) if Path::new(&backup_of).exists() => BackupSniff {
                is_backup: true,
                backup_of: Some(backup_of),
                backup_created_at: None,
                authoritative: false,
            },
            _ => BackupSniff::not_backup(false),
        },
    }
}

/// If `path`'s file name matches `<name>-<YYYYMMDD>-<HHMMSS>[-<N>].skrib`, return
/// the guessed original path (`<same dir>/<name>.skrib`); else `None`.
///
/// Public so callers with only a path (no manifest) can guess an original, but
/// **never** used for retention deletion — that correlates strictly on the
/// manifest marker + `unique_id`.
pub fn sniff_backup_filename(path: &str) -> Option<String> {
    let p = Path::new(path);
    let file = p.file_name()?.to_str()?;
    let stem = file.strip_suffix(".skrib").unwrap_or(file);
    let parts: Vec<&str> = stem.split('-').collect();
    let n = parts.len();

    // Two tail shapes: `.. date time` (2) and `.. date time collision-N` (3).
    for tail_len in [2usize, 3usize] {
        if n <= tail_len {
            continue;
        }
        let (date, time, extra_ok) = if tail_len == 2 {
            (parts[n - 2], parts[n - 1], true)
        } else {
            let extra = parts[n - 1];
            (
                parts[n - 3],
                parts[n - 2],
                !extra.is_empty() && extra.bytes().all(|b| b.is_ascii_digit()),
            )
        };
        if is_digits(date, 8) && is_digits(time, 6) && extra_ok {
            let name = parts[..n - tail_len].join("-");
            if name.is_empty() {
                return None;
            }
            let dir = p.parent().unwrap_or_else(|| Path::new(""));
            return Some(
                dir.join(format!("{name}.skrib"))
                    .to_string_lossy()
                    .into_owned(),
            );
        }
    }
    None
}

fn is_digits(s: &str, len: usize) -> bool {
    s.len() == len && s.bytes().all(|b| b.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filename_fallback_matches_stamped_backup() {
        assert_eq!(
            sniff_backup_filename("/home/u/backups/mynovel-20260101-153000.skrib"),
            Some("/home/u/backups/mynovel.skrib".to_string())
        );
    }

    #[test]
    fn filename_fallback_matches_collision_suffix() {
        assert_eq!(
            sniff_backup_filename("/b/mynovel-20260101-153000-2.skrib"),
            Some("/b/mynovel.skrib".to_string())
        );
    }

    #[test]
    fn filename_fallback_keeps_hyphenated_name() {
        assert_eq!(
            sniff_backup_filename("/b/my-great-novel-20260101-153000.skrib"),
            Some("/b/my-great-novel.skrib".to_string())
        );
    }

    #[test]
    fn filename_fallback_rejects_plain_project() {
        assert_eq!(sniff_backup_filename("/b/mynovel.skrib"), None);
        // date-like but wrong separators / widths
        assert_eq!(sniff_backup_filename("/b/report-2026-01-01.skrib"), None);
        // no name before the stamp
        assert_eq!(sniff_backup_filename("/b/20260101-153000.skrib"), None);
    }
}
