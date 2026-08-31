// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What "Add as note" remembers between captures, per project.
//!
//! Two facts, both the writer's own and neither derivable from the project itself:
//!
//! - **Which tags they reach for.** The capture submenu puts the discoverable tags
//!   first, because those are the story-bible ones, then the few other tags this writer
//!   actually uses. "Actually uses" is only knowable by watching, so it is remembered
//!   here rather than guessed from the palette.
//! - **Where an untagged note goes.** A note filed under no tag has no tag to carry a
//!   destination, so its folder lives here instead. Asked once, then silent.
//!
//! **Keyed on durable uids, never store ids.** A `BinderItem.uid` and a `BinderTag.uid`
//! survive save and load; an `EntityId` is re-minted by every `load_work`, so a recents
//! list built on ids would quietly name different tags after a reopen. Same rule, and
//! the same file shape, as [`crate::models::TreeExpansionService`] and
//! [`crate::models::NoteBookChoiceService`].
//!
//! **Per project, in the user's own config directory.** Which tags one writer reaches
//! for is not a fact about the manuscript, so it does not belong in the bundle: a
//! project opened by a collaborator should offer *their* habits, not the author's.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use teksilo::settings::{AppPaths, Migrator, SettingsFile, SettingsFileError, Versioned};
use uuid::Uuid;

/// Accepted for call-site stability only; `SettingsFile`'s writes are a synchronous
/// locked read-modify-write with no debounce (see the siblings this mirrors).
const SETTINGS_DEBOUNCE: Duration = Duration::from_millis(500);

/// Cap on remembered projects, matching every sibling: newest kept, oldest evicted.
const MAX_PROJECTS: usize = 128;

/// How many non-discoverable tags the submenu offers before the overflow.
///
/// Five is the tier's whole budget. A context menu that lists twenty tags is a menu the
/// writer reads instead of one they aim at, and everything past the cap is one hop away
/// under "All tags", never hidden.
pub const MAX_RECENT_TAGS: usize = 5;

/// One project's capture memory.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PerProjectNoteCapture {
    /// `Work.unique_id`, the key.
    pub work_uid: String,
    /// Display/debug only; the key is [`Self::work_uid`].
    #[serde(default)]
    pub last_path: String,
    /// Most recently used first. `BinderTag.uid`, never a store id.
    #[serde(default)]
    pub recent_tags: Vec<Uuid>,
    /// Where a note captured under no tag lands. `BinderItem.uid`.
    #[serde(default)]
    pub untagged_folder: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NoteCaptureFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub projects: Vec<PerProjectNoteCapture>,
}

fn default_version() -> u32 {
    NoteCaptureFile::CURRENT_VERSION
}

impl Default for NoteCaptureFile {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            projects: Vec::new(),
        }
    }
}

impl Versioned for NoteCaptureFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// Persistent capture memory. `SettingsFile` is `Clone` and shares its live state, so
/// cloning hands out views over the same file.
#[derive(Clone)]
pub struct NoteCaptureService {
    file: SettingsFile<NoteCaptureFile>,
}

impl NoteCaptureService {
    /// Open `note_capture.toml` under `paths` (cross-process safe).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        Self::open_with_delay(paths, SETTINGS_DEBOUNCE)
    }

    pub fn open_with_delay(paths: &AppPaths, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("note_capture"), Migrator::new())?;
        Ok(Self { file })
    }

    #[cfg(test)]
    pub fn open_at(path: std::path::PathBuf, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, Migrator::new())?;
        Ok(Self { file })
    }

    /// An in-memory stand-in for a launch with no usable config directory. The feature
    /// goes quiet rather than blocking startup: captures still work, they simply stop
    /// remembering between sessions.
    pub fn in_memory_default() -> Self {
        let file =
            super::backup_settings_file::in_memory_settings_file("note-capture", Migrator::new());
        Self { file }
    }

    /// The tags this writer has most recently captured under, newest first.
    ///
    /// Uids, so the caller resolves them against the live palette. A uid naming a tag
    /// that has since been deleted simply finds nothing and is skipped, which is why
    /// this returns what was remembered rather than pruning here: the palette is the
    /// authority on what still exists, and this file never sees a delete.
    pub fn recent_tags(&self, work_uid: &str) -> Vec<Uuid> {
        if !super::uid_is_usable(work_uid) {
            return Vec::new();
        }
        self.file
            .borrow()
            .projects
            .iter()
            .find(|p| p.work_uid == work_uid)
            .map(|p| p.recent_tags.clone())
            .unwrap_or_default()
    }

    /// Record a capture under `tag_uid`, moving it to the front.
    ///
    /// Re-capturing under a tag already in the list moves it rather than duplicating it,
    /// so the order really is "most recently used" and not "most recently added".
    pub fn note_tag_used(
        &self,
        work_uid: &str,
        last_path: &str,
        tag_uid: Uuid,
    ) -> Result<(), SettingsFileError> {
        if !super::uid_is_usable(work_uid) {
            return Ok(());
        }
        // A nil uid names no tag, so remembering it would make the list say something
        // false: every tag whose identity was never minted matches it, and the reader
        // resolving these against the palette would find them all "recently used".
        // Nothing is recorded rather than something wrong.
        if tag_uid.is_nil() {
            return Ok(());
        }
        self.file.mutate(|f| {
            f.version = NoteCaptureFile::CURRENT_VERSION;
            let row = touch(f, work_uid, last_path);
            row.recent_tags.retain(|u| *u != tag_uid);
            row.recent_tags.insert(0, tag_uid);
            // Kept a little longer than the menu shows, so a tag that falls off the
            // visible tier is not forgotten the moment it does: the writer who reaches
            // for it again finds it back at the front rather than back at the bottom of
            // "All tags".
            row.recent_tags.truncate(MAX_RECENT_TAGS * 2);
            cap(f);
        })
    }

    /// Where a note captured under no tag lands, if the writer has said.
    pub fn untagged_folder(&self, work_uid: &str) -> Option<Uuid> {
        if !super::uid_is_usable(work_uid) {
            return None;
        }
        self.file
            .borrow()
            .projects
            .iter()
            .find(|p| p.work_uid == work_uid)
            .and_then(|p| p.untagged_folder)
    }

    /// Remember where untagged notes go. Asked once, on the first untagged capture.
    pub fn set_untagged_folder(
        &self,
        work_uid: &str,
        last_path: &str,
        folder_uid: Uuid,
    ) -> Result<(), SettingsFileError> {
        if !super::uid_is_usable(work_uid) {
            return Ok(());
        }
        self.file.mutate(|f| {
            f.version = NoteCaptureFile::CURRENT_VERSION;
            touch(f, work_uid, last_path).untagged_folder = Some(folder_uid);
            cap(f);
        })
    }
}

/// Find or create this project's row and move it to the end, so [`cap`] can never evict
/// the project currently in use. Touch order, as every sibling's own cap works.
fn touch<'a>(
    f: &'a mut NoteCaptureFile,
    work_uid: &str,
    last_path: &str,
) -> &'a mut PerProjectNoteCapture {
    match f.projects.iter().position(|p| p.work_uid == work_uid) {
        Some(pos) => {
            let mut row = f.projects.remove(pos);
            row.last_path = last_path.to_string();
            f.projects.push(row);
        }
        None => f.projects.push(PerProjectNoteCapture {
            work_uid: work_uid.to_string(),
            last_path: last_path.to_string(),
            ..Default::default()
        }),
    }
    f.projects.last_mut().expect("just pushed or moved")
}

fn cap(f: &mut NoteCaptureFile) {
    let len = f.projects.len();
    if len > MAX_PROJECTS {
        f.projects.drain(0..len - MAX_PROJECTS);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn service() -> (NoteCaptureService, tempfile::TempDir) {
        let dir = tempdir().expect("tempdir");
        let svc =
            NoteCaptureService::open_at(dir.path().join("note_capture.toml"), SETTINGS_DEBOUNCE)
                .expect("open");
        (svc, dir)
    }

    fn uid(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    fn a_project_with_no_history_offers_nothing_rather_than_guessing() {
        let (svc, _d) = service();
        assert!(svc.recent_tags("work-a").is_empty());
        assert_eq!(svc.untagged_folder("work-a"), None);
    }

    /// Most recently used, not most recently added: re-capturing under a tag already in
    /// the list has to move it to the front rather than leave it where it was, or the
    /// tier stops meaning what its name says.
    #[test]
    fn re_using_a_tag_moves_it_to_the_front_rather_than_duplicating_it() {
        let (svc, _d) = service();
        svc.note_tag_used("w", "/p", uid(1)).unwrap();
        svc.note_tag_used("w", "/p", uid(2)).unwrap();
        svc.note_tag_used("w", "/p", uid(1)).unwrap();
        assert_eq!(svc.recent_tags("w"), vec![uid(1), uid(2)]);
    }

    /// The list is kept longer than the menu shows, so a tag that scrolls off the
    /// visible tier is not forgotten the instant it does.
    #[test]
    fn history_outlives_the_visible_tier_but_is_still_bounded() {
        let (svc, _d) = service();
        for n in 0..40u128 {
            svc.note_tag_used("w", "/p", uid(n)).unwrap();
        }
        let recents = svc.recent_tags("w");
        assert!(
            recents.len() > MAX_RECENT_TAGS,
            "more is remembered than shown"
        );
        assert_eq!(recents.len(), MAX_RECENT_TAGS * 2, "but it is bounded");
        assert_eq!(recents[0], uid(39), "newest first");
    }

    /// Two projects must not share a writer's habits: the same tag uid in each is two
    /// different tags, and a recents list that leaked across would offer one project's
    /// palette in the other.
    #[test]
    fn projects_do_not_share_their_history() {
        let (svc, _d) = service();
        svc.note_tag_used("work-a", "/a", uid(1)).unwrap();
        svc.set_untagged_folder("work-a", "/a", uid(9)).unwrap();
        assert!(svc.recent_tags("work-b").is_empty());
        assert_eq!(svc.untagged_folder("work-b"), None);
        assert_eq!(svc.recent_tags("work-a"), vec![uid(1)]);
    }

    /// An unsaved project has no usable `unique_id`, and every project without one would
    /// otherwise share a single row: one writer's habits leaking into every new file.
    #[test]
    fn an_unusable_work_uid_is_never_written() {
        let (svc, _d) = service();
        svc.note_tag_used("", "/p", uid(1)).unwrap();
        assert!(svc.recent_tags("").is_empty());
    }

    /// A nil uid names no tag. Writing one would make the file claim a capture under
    /// every tag whose identity was never minted, so it is not written at all, and the
    /// history it would have displaced is left intact.
    #[test]
    fn a_nil_tag_uid_is_never_remembered() {
        let (svc, _d) = service();
        svc.note_tag_used("w", "/p", uid(1)).unwrap();
        svc.note_tag_used("w", "/p", Uuid::nil()).unwrap();
        assert_eq!(
            svc.recent_tags("w"),
            vec![uid(1)],
            "nothing recorded, and the real history untouched"
        );
    }

    #[test]
    fn the_untagged_folder_round_trips() {
        let (svc, _d) = service();
        svc.set_untagged_folder("w", "/p", uid(7)).unwrap();
        assert_eq!(svc.untagged_folder("w"), Some(uid(7)));
    }
}
