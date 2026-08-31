// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which copies of a manuscript have gone out for review, and what was in them.
//!
//! # What this answers that the exported file cannot
//!
//! An export written with round-trip marks carries, per row, a bookmark naming
//! that row's uid and a digest of its prose as exported — enough for a returning
//! file to be recognised and compared, with nothing stored on this side. That is
//! deliberate and it stays true.
//!
//! It stops being enough the moment **more than one copy is out at once**,
//! because a mark name is one-way: `uid_tag` is a hash and the digest a
//! truncation, so nothing here can read a name back. Three questions follow, and
//! none of them is answerable from a file:
//!
//! - *How many copies are out, and since when?* — so a writer can be told
//!   "three copies out since 14 August; four chapters have changed since".
//! - *Was this row already merged from an earlier return?*
//!   [`RowStatus::YouEdited`](skribisto_model::reconcile::RowStatus) means "you
//!   changed it and the reader did not", and after the first merge that is a
//!   lie — the change came from reader one.
//! - *Did this copy ever contain that reply?* — the difference between "the
//!   editor deleted it" and "their copy was cut before it existed", which the
//!   import currently cannot tell apart and resolves by detaching.
//!
//! # Why the writer's config, not the bundle
//!
//! Who a manuscript was sent to is a fact about the writer's working life, not
//! about the book. A collaborator opening the same project has their own
//! correspondents and should see theirs — the same reasoning that keeps
//! [`crate::models::NoteCaptureService`] out of the bundle. It also means a
//! `.skrib` handed to someone else carries no list of who else has read it.
//!
//! # What is deliberately not here
//!
//! **No prose, and no digest that is not already in the exported file.** Every
//! digest recorded was written into the document in plain sight, so this file
//! discloses nothing its recipient was not handed. A record that stored the text
//! would be a second copy of the manuscript with a different retention story,
//! and that is the shape that turns a convenience into a liability.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use teksilo::settings::{AppPaths, Migrator, SettingsFile, SettingsFileError, Versioned};

/// Accepted for call-site stability only; `SettingsFile`'s writes are a
/// synchronous locked read-modify-write with no debounce (see the siblings this
/// mirrors).
const SETTINGS_DEBOUNCE: Duration = Duration::from_millis(500);

/// Cap on remembered projects, matching every sibling: newest kept, oldest evicted.
const MAX_PROJECTS: usize = 128;

/// Cap on remembered packages per project.
///
/// A writer running a panel sends out a dozen copies; a prolific one across a
/// long revision might reach a few hundred. Keeping the newest 64 covers every
/// real correspondence while bounding a file that is only ever appended to, and
/// an evicted package costs nothing but the "already merged from an earlier
/// return" hint on rows nobody has touched in months.
const MAX_PACKAGES: usize = 64;

/// One row as it left: durable identity, and what it said at the time.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct SentRow {
    /// `BinderItem.uid`, hyphenated. Never a store id — those are re-minted by
    /// every `load_work`.
    pub item_uid: String,
    /// `round_trip::digest` of the prose that went out: the twelve hex digits the
    /// file's own bookmark carries, and the baseline a return is judged against.
    ///
    /// ⚠ This is `skribisto_model::round_trip::digest`, **not**
    /// [`crate::models::manuscript_digest`]'s. The two are different functions
    /// over different inputs and comparing one to the other reports every row as
    /// edited. This one is the one that travels in the file.
    pub digest: String,
}

/// One copy that went out.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct SentPackage {
    /// When it left, RFC 3339 UTC — the "since when" of the writer's readout.
    pub sent_at: String,
    /// The file that was written. Display only: it may have been moved, renamed
    /// or deleted, and none of that invalidates the record.
    pub output_path: String,
    /// The format's name as `ExportFormat`'s `Debug` spells it.
    pub format: String,
    /// The rows that carried a mark, in export order.
    pub rows: Vec<SentRow>,
    /// The `Comment.uid`s that went out, hyphenated.
    ///
    /// What lets a return be asked "did your copy ever have this thread?" — the
    /// question that separates a reply the editor deleted from one their copy was
    /// cut before.
    pub comment_uids: Vec<String>,
}

/// One project's outgoing history.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct PerProjectExchange {
    /// `Work.unique_id`, the key.
    pub work_uid: String,
    /// Display/debug only; the key is [`Self::work_uid`].
    pub title: String,
    /// Newest last.
    pub packages: Vec<SentPackage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ExchangeFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub projects: Vec<PerProjectExchange>,
}

fn default_version() -> u32 {
    ExchangeFile::CURRENT_VERSION
}

impl Default for ExchangeFile {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            projects: Vec::new(),
        }
    }
}

impl Versioned for ExchangeFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// The persistent record of copies sent out. `SettingsFile` is `Clone` and shares
/// its live state, so cloning hands out views over the same file.
#[derive(Clone)]
pub struct ExchangeService {
    file: SettingsFile<ExchangeFile>,
}

impl ExchangeService {
    /// Open `exchange.toml` under `paths` (cross-process safe).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        Self::open_with_delay(paths, SETTINGS_DEBOUNCE)
    }

    pub fn open_with_delay(paths: &AppPaths, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("exchange"), Migrator::new())?;
        Ok(Self { file })
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn open_at(path: std::path::PathBuf, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, Migrator::new())?;
        Ok(Self { file })
    }

    /// An in-memory stand-in for a launch with no usable config directory. The
    /// feature goes quiet rather than blocking startup: exports still work, they
    /// simply stop being remembered between sessions.
    pub fn in_memory_default() -> Self {
        Self {
            file: super::backup_settings_file::in_memory_settings_file("exchange", Migrator::new()),
        }
    }

    /// Record a copy that has just gone out.
    ///
    /// A package with no rows is **not** recorded: an export written without
    /// round-trip marks — a PDF for a reader, a plain-text draft — carries
    /// nothing a return could be matched on, so listing it would tell the writer
    /// a copy is "out" in a sense the software cannot act on.
    pub fn record(&self, work_uid: &str, title: &str, package: SentPackage) {
        if !super::uid_is_usable(work_uid) || package.rows.is_empty() {
            return;
        }
        let _ = self.file.mutate(|f| {
            let project = project_mut(f, work_uid);
            project.title = title.to_string();
            project.packages.push(package);
            let overflow = project.packages.len().saturating_sub(MAX_PACKAGES);
            if overflow > 0 {
                project.packages.drain(..overflow);
            }
            let overflow = f.projects.len().saturating_sub(MAX_PROJECTS);
            if overflow > 0 {
                f.projects.drain(..overflow);
            }
        });
    }

    /// Every copy sent out of this project, oldest first.
    pub fn packages(&self, work_uid: &str) -> Vec<SentPackage> {
        if !super::uid_is_usable(work_uid) {
            return Vec::new();
        }
        let f = self.file.borrow();
        f.projects
            .iter()
            .find(|p| p.work_uid == work_uid)
            .map(|p| p.packages.clone())
            .unwrap_or_default()
    }

    /// The digest this row carried when it last went out, if it ever did.
    ///
    /// The newest package wins: a row sent three times is being asked about the
    /// copy most likely still in someone's hands.
    pub fn last_sent_digest(&self, work_uid: &str, item_uid: &str) -> Option<String> {
        self.packages(work_uid).iter().rev().find_map(|p| {
            p.rows
                .iter()
                .find(|r| r.item_uid == item_uid)
                .map(|r| r.digest.clone())
        })
    }

    /// Whether any copy that went out carried this comment thread.
    ///
    /// The question the import needs to tell "the editor deleted this reply" from
    /// "their copy was cut before it existed". A thread no copy ever carried
    /// cannot have been deleted in one.
    pub fn was_sent(&self, work_uid: &str, comment_uid: &str) -> bool {
        self.packages(work_uid)
            .iter()
            .any(|p| p.comment_uids.iter().any(|u| u == comment_uid))
    }
}

/// Find-or-create this project's row.
fn project_mut<'a>(f: &'a mut ExchangeFile, work_uid: &str) -> &'a mut PerProjectExchange {
    if let Some(i) = f.projects.iter().position(|p| p.work_uid == work_uid) {
        return &mut f.projects[i];
    }
    f.projects.push(PerProjectExchange {
        work_uid: work_uid.to_string(),
        ..Default::default()
    });
    f.projects.last_mut().expect("just pushed")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn svc() -> ExchangeService {
        ExchangeService::in_memory_default()
    }

    fn package(sent_at: &str, rows: &[(&str, &str)], comments: &[&str]) -> SentPackage {
        SentPackage {
            sent_at: sent_at.into(),
            output_path: "/tmp/book.docx".into(),
            format: "Docx".into(),
            rows: rows
                .iter()
                .map(|(uid, digest)| SentRow {
                    item_uid: (*uid).into(),
                    digest: (*digest).into(),
                })
                .collect(),
            comment_uids: comments.iter().map(|c| (*c).to_string()).collect(),
        }
    }

    #[test]
    fn a_recorded_package_comes_back() {
        let s = svc();
        s.record(
            "uid-1",
            "The Lighthouse",
            package(
                "2026-08-14T10:00:00Z",
                &[("row-a", "0123456789ab")],
                &["c-1"],
            ),
        );
        let packages = s.packages("uid-1");
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].rows[0].digest, "0123456789ab");
        assert!(s.was_sent("uid-1", "c-1"));
        assert!(!s.was_sent("uid-1", "c-2"));
    }

    /// The empty uid belongs to an unsaved project, which has no durable identity
    /// to key anything by — the rule every sibling enforces.
    #[test]
    fn an_unusable_work_uid_is_never_written() {
        let s = svc();
        s.record(
            "",
            "Untitled",
            package("2026-08-14T10:00:00Z", &[("row-a", "aaa")], &[]),
        );
        s.record(
            "   ",
            "Untitled",
            package("2026-08-14T10:00:00Z", &[("row-a", "aaa")], &[]),
        );
        assert!(s.packages("").is_empty());
        assert!(s.packages("   ").is_empty());
    }

    /// An export that carried no marks cannot be matched on the way home, so
    /// recording it would report a copy as "out" in a sense nothing can act on.
    #[test]
    fn an_export_with_no_marked_rows_is_not_recorded() {
        let s = svc();
        s.record("uid-1", "T", package("2026-08-14T10:00:00Z", &[], &["c-1"]));
        assert!(s.packages("uid-1").is_empty());
    }

    /// A row sent more than once reports the copy most likely still out there.
    #[test]
    fn the_newest_package_wins_for_a_row_sent_twice() {
        let s = svc();
        s.record(
            "uid-1",
            "T",
            package("2026-08-01T10:00:00Z", &[("row-a", "old000000000")], &[]),
        );
        s.record(
            "uid-1",
            "T",
            package("2026-08-20T10:00:00Z", &[("row-a", "new000000000")], &[]),
        );
        assert_eq!(
            s.last_sent_digest("uid-1", "row-a").as_deref(),
            Some("new000000000")
        );
        assert_eq!(s.last_sent_digest("uid-1", "row-b"), None);
    }

    #[test]
    fn two_projects_never_clobber_each_other() {
        let s = svc();
        s.record(
            "uid-1",
            "One",
            package("2026-08-01T10:00:00Z", &[("a", "111111111111")], &[]),
        );
        s.record(
            "uid-2",
            "Two",
            package("2026-08-02T10:00:00Z", &[("b", "222222222222")], &[]),
        );
        assert_eq!(s.packages("uid-1").len(), 1);
        assert_eq!(s.packages("uid-2").len(), 1);
        assert_eq!(s.packages("uid-1")[0].rows[0].item_uid, "a");
        assert_eq!(s.packages("uid-2")[0].rows[0].item_uid, "b");
    }

    /// The per-project cap evicts oldest-first, so the newest correspondence is
    /// the one that survives.
    #[test]
    fn packages_are_capped_oldest_first() {
        let s = svc();
        for i in 0..(MAX_PACKAGES + 5) {
            s.record(
                "uid-1",
                "T",
                package(
                    &format!("2026-08-01T10:00:{i:02}Z"),
                    &[("a", "111111111111")],
                    &[],
                ),
            );
        }
        let packages = s.packages("uid-1");
        assert_eq!(packages.len(), MAX_PACKAGES);
        assert_eq!(packages[0].sent_at, "2026-08-01T10:00:05Z");
    }
}
