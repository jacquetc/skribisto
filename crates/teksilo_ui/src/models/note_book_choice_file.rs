// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet
//! Remembered Book choice for a story-bible note's "In prose" segment.
//!
//! A `SettingsFile<NoteBookChoiceFile>` at `<config_dir>/note_book_choice.toml`, holding
//! one row per project (keyed by `Work.unique_id`), each naming the durable
//! `BinderItem.uid` of the Book every `Item/Note` tab's "In prose" segment opens on.
//!
//! **Per project, not per note, and not per tab.** A writer reading where "Devon" appears
//! and a writer reading where "Hap" appears in the same sitting are almost always reading
//! the same Book (the one they are drafting right now), and a choice that reset with
//! every note they opened would have them re-pick it a dozen times a session for no
//! reason. So the file holds exactly one Book per project, applied to every `Item/Note`
//! tab at the moment it is opened; it does not track which note asked, and a note's own
//! pane is free to switch it locally (see `tabs::note_in_prose`) without disturbing this
//! remembered default until the writer's next choice is captured.
//!
//! **Why a durable uid and not a store id.** The same reasoning
//! [`crate::models::tree_expansion_file`] states for its own container uids applies
//! unchanged here: `EntityId` is a position in an ephemeral `HashMap` that `load_work`
//! re-mints on every open, and a `.skrib`'s `file_id`s are only those ids at save time.
//! Keyed by either, a remembered Book would silently resolve to whatever item happened to
//! inherit that number on the next open, pointing the reading at a scene, or at nothing,
//! rather than at the Book the writer meant. `BinderItem.uid` (`.skrib` v3) is the one
//! identity that survives a save, then a load.
//!
//! **A sibling of `tree_expansion_file` / `workspace_layout_file` / `search_settings_file`
//! / `backup_settings_file`**, and deliberately its own file rather than a new column on
//! any of them: those three are written at their own doors (a container's expand state on
//! project-leave, the desk layout on project-leave, search preferences on every search) on
//! their own schedules, and a corrupt or future-schema'd blob on one must never quarantine
//! an unrelated one. This file's own schedule is different again: read once per `Item/Note`
//! tab opened, written once per Book the writer actually picks, so it gets the same
//! isolation the others already have from each other.
//!
//! **Single implementation, no real/mock seam**: a config file, not backend data, exactly
//! like every sibling named above.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use teksilo::settings::{AppPaths, Migrator, SettingsFile, SettingsFileError, Versioned};
use uuid::Uuid;

/// Accepted for call-site stability only; `SettingsFile`'s writes are a synchronous
/// locked read-modify-write with no debounce (see the siblings this mirrors).
const SETTINGS_DEBOUNCE: Duration = Duration::from_millis(500);

/// Cap on remembered projects: a backstop against unbounded growth from deleted
/// projects and from legacy uid-less `.skrib`s that mint a fresh `unique_id` on every
/// open. Newest kept, oldest evicted, exactly as `tree_expansion_file`'s own cap works.
const MAX_PROJECTS: usize = 128;

/// One project's remembered Book choice.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PerProjectNoteBookChoice {
    /// `Work.unique_id`, the key. A `String`, not a `Uuid`, because that field still is
    /// one (unlike the Book row's own uid below).
    pub work_uid: String,
    /// Display/debug only; the key is [`Self::work_uid`].
    #[serde(default)]
    pub last_path: String,
    /// The chosen Book's durable `BinderItem.uid`.
    pub book_uid: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NoteBookChoiceFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub projects: Vec<PerProjectNoteBookChoice>,
}

fn default_version() -> u32 {
    NoteBookChoiceFile::CURRENT_VERSION
}

impl Default for NoteBookChoiceFile {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            projects: Vec::new(),
        }
    }
}

impl Versioned for NoteBookChoiceFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// Persistent Book-choice service. `SettingsFile` is `Clone` (it shares the in-memory
/// state + writer), so cloning hands out views over the same live file, the same
/// contract [`crate::models::TreeExpansionService`] and
/// [`crate::models::WorkspaceLayoutService`] already give their own callers.
#[derive(Clone)]
pub struct NoteBookChoiceService {
    file: SettingsFile<NoteBookChoiceFile>,
}

impl NoteBookChoiceService {
    /// Open `note_book_choice.toml` under `paths` (cross-process safe).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        Self::open_with_delay(paths, SETTINGS_DEBOUNCE)
    }

    /// `delay` is accepted for call-site stability but has no effect. See
    /// [`SETTINGS_DEBOUNCE`].
    pub fn open_with_delay(paths: &AppPaths, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("note_book_choice"), Migrator::new())?;
        Ok(Self { file })
    }

    /// Open at an explicit path. Test-only: production reaches the file through
    /// [`open`](Self::open) (the config dir) or [`in_memory_default`](Self::in_memory_default).
    #[cfg(test)]
    pub fn open_at(path: std::path::PathBuf, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, Migrator::new())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable: a throwaway per-process temp
    /// file, so the app still runs (the choice just won't survive a restart). Mirrors
    /// [`crate::models::TreeExpansionService::in_memory_default`] and
    /// [`crate::models::WorkspaceLayoutService::in_memory_default`], which is also why
    /// every call site can register this unconditionally rather than carrying an `Option`
    /// around. Shares its retry/uniqueness logic with every sibling via
    /// [`crate::models::backup_settings_file::in_memory_settings_file`].
    pub fn in_memory_default() -> Self {
        let file = super::backup_settings_file::in_memory_settings_file(
            "note-book-choice",
            Migrator::new(),
        );
        Self { file }
    }

    /// The remembered Book for `work_uid`, or `None` when nothing has been chosen yet (a
    /// fresh project, or one whose config dir was unavailable when it was last open).
    ///
    /// The caller still has to resolve this against the Work's **live** Book list. See
    /// [`crate::models::resolve_book_choice`], because nothing here knows whether the
    /// Book this uid once named still exists.
    pub fn book(&self, work_uid: &str) -> Option<Uuid> {
        if !super::uid_is_usable(work_uid) {
            return None;
        }
        self.file
            .borrow()
            .projects
            .iter()
            .find(|p| p.work_uid == work_uid)
            .map(|p| p.book_uid)
    }

    /// Remember `book_uid` as the Book every `Item/Note` tab in this project opens its
    /// "In prose" segment against, from now on.
    pub fn set_book(
        &self,
        work_uid: &str,
        last_path: &str,
        book_uid: Uuid,
    ) -> Result<(), SettingsFileError> {
        if !super::uid_is_usable(work_uid) {
            return Ok(());
        }
        self.file.mutate(|f| {
            f.version = NoteBookChoiceFile::CURRENT_VERSION;
            match f.projects.iter().position(|p| p.work_uid == work_uid) {
                Some(pos) => {
                    f.projects[pos].last_path = last_path.to_string();
                    f.projects[pos].book_uid = book_uid;
                    // Touch order, as every sibling's own cap does: the row just written
                    // is the newest, so the cap below can never evict the project in use.
                    let row = f.projects.remove(pos);
                    f.projects.push(row);
                }
                None => f.projects.push(PerProjectNoteBookChoice {
                    work_uid: work_uid.to_string(),
                    last_path: last_path.to_string(),
                    book_uid,
                }),
            }
            let len = f.projects.len();
            if len > MAX_PROJECTS {
                f.projects.drain(0..len - MAX_PROJECTS);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn svc(dir: &std::path::Path) -> NoteBookChoiceService {
        NoteBookChoiceService::open_at(dir.join("note_book_choice.toml"), Duration::ZERO).unwrap()
    }

    fn uid(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    fn a_captured_choice_round_trips() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        s.set_book("work-1", "/p.skrib", uid(10)).unwrap();
        assert_eq!(s.book("work-1"), Some(uid(10)));
    }

    #[test]
    fn an_unknown_project_reads_as_none() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        s.set_book("work-1", "", uid(10)).unwrap();
        assert_eq!(s.book("other-work"), None);
    }

    /// A blank `Work.unique_id` must not become a shared key: a brand-new unsaved
    /// project has none, and every one of them would otherwise read and write the same
    /// row.
    #[test]
    fn a_blank_work_uid_is_neither_written_nor_read() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        s.set_book("", "/p.skrib", uid(10)).unwrap();
        assert_eq!(s.book(""), None);
        s.set_book("  ", "", uid(10)).unwrap();
        assert_eq!(s.book("  "), None);
    }

    /// A later pick replaces the remembered Book rather than adding a second row for the
    /// same project: there is exactly one current choice per project, never a history.
    #[test]
    fn a_later_pick_replaces_the_remembered_book() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        s.set_book("w", "", uid(10)).unwrap();
        s.set_book("w", "", uid(20)).unwrap();
        assert_eq!(s.book("w"), Some(uid(20)));
        assert_eq!(s.file.borrow().projects.len(), 1);
    }

    /// Projects are capped by **touch order**, so the project just written can never be
    /// the one evicted: the bug a naive "drain the front" cap has, and the same property
    /// `tree_expansion_file`'s own cap tests guard.
    #[test]
    fn projects_are_capped_newest_first() {
        let dir = tempdir().unwrap();
        let s = svc(dir.path());
        for i in 0..(MAX_PROJECTS + 20) {
            s.set_book(&format!("w-{i}"), "", uid(1)).unwrap();
        }
        assert_eq!(s.file.borrow().projects.len(), MAX_PROJECTS);
        assert_eq!(
            s.book(&format!("w-{}", MAX_PROJECTS + 19)),
            Some(uid(1)),
            "the most recently written project survives the cap"
        );
        assert_eq!(
            s.book("w-0"),
            None,
            "the oldest project was evicted to make room"
        );
    }
}
