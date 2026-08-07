// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet
//! Where each kind of file dialog last opened.
//!
//! A `SettingsFile<FolderMemoryFile>` at `<config_dir>/folders.toml`, holding one
//! remembered directory per [`FolderPurpose`].
//!
//! **Why this exists.** `FileDialogRequest::starting_dir` has always been there, and
//! wired through to the native dialog — and not one of the app's file dialogs set it.
//! Every one of them opened wherever the OS last felt like, which for a writer whose
//! manuscripts live three directories deep is a small tax paid on every export, every
//! import, every dictionary they sideload.
//!
//! **Keyed by purpose, not by call site.** Ten purposes cover roughly thirty dialogs,
//! because the useful grouping is the writer's, not the code's: "where I keep my
//! projects" is one answer whether they reached the dialog from the launcher, the menu
//! or the recent list, while "where I export to" is a genuinely different place. A
//! typed enum rather than a string key so a new call site cannot invent a private
//! seventh spelling of `export` that silently never matches the other six.
//!
//! **App-global, not per project.** This is a habit, not a property of a manuscript:
//! the folder a writer exports to is the same one whichever book they are working on.
//! That is also why there is no `uid_is_usable` guard here — nothing is keyed by
//! `Work.unique_id`, so there is no empty key to defend against.
//!
//! **Single implementation (no real/mock seam)** — it is app configuration, not backend
//! data, exactly like `backup_settings_file` and `tree_expansion_file`.

use std::path::{Path, PathBuf};
use std::time::Duration;

use bastyde::settings::{AppPaths, Migrator, SettingsFile, SettingsFileError, Versioned};
use serde::{Deserialize, Serialize};

/// Accepted for call-site stability only; `SettingsFile`'s writes are a synchronous
/// locked read-modify-write with no debounce (see the siblings).
const SETTINGS_DEBOUNCE: Duration = Duration::from_millis(500);

/// What a dialog was for — the unit this file remembers a directory for.
///
/// Deliberately coarser than the call sites. The settings panes' half-dozen
/// import/export dialogs all share [`DataInterchange`](FolderPurpose::DataInterchange)
/// because they are the same act to the person doing it: moving a small sidecar file —
/// a tag palette, a dictionary, a theme — between this app and somewhere else. Splitting
/// them would remember six near-identical answers and get each one right one sixth as
/// often.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum FolderPurpose {
    /// Opening an existing `.skrib`.
    OpenProject,
    /// Where a new project's folder is created.
    NewProjectLocation,
    /// Picking documents to import (Markdown, plain text, …).
    ImportDocuments,
    /// Picking a Plume Creator project to convert.
    ImportPlume,
    /// Inserting a picture into the manuscript, and choosing a cover.
    InsertImage,
    /// Where an exported book is written.
    Export,
    /// Save As, including a backup saved out of backup mode.
    SaveAs,
    /// Adding a backup destination directory.
    BackupDestination,
    /// Sideloading a Hunspell dictionary pair.
    AddDictionary,
    /// The settings panes' import/export of tags, templates, dictionaries, text
    /// replacements, export styles and distraction-free themes.
    DataInterchange,
}

impl FolderPurpose {
    /// Every purpose, for the round-trip test — a new variant that nobody remembers to
    /// add here fails that test rather than silently going unexercised.
    pub const ALL: &'static [FolderPurpose] = &[
        FolderPurpose::OpenProject,
        FolderPurpose::NewProjectLocation,
        FolderPurpose::ImportDocuments,
        FolderPurpose::ImportPlume,
        FolderPurpose::InsertImage,
        FolderPurpose::Export,
        FolderPurpose::SaveAs,
        FolderPurpose::BackupDestination,
        FolderPurpose::AddDictionary,
        FolderPurpose::DataInterchange,
    ];
}

/// One purpose's remembered directory.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RememberedFolder {
    pub purpose: FolderPurpose,
    /// An absolute directory path. Stored as a `String` because that is what TOML round
    /// trips cleanly on every platform; resolved to a `PathBuf` on the way out.
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FolderMemoryFile {
    #[serde(default = "default_version")]
    pub version: u32,
    /// A `Vec` rather than a map: TOML round trips `[[array-of-tables]]` cleanly, and
    /// with ten entries a linear scan is not worth a thought. The same shape every
    /// sibling settings file uses.
    #[serde(default)]
    pub folders: Vec<RememberedFolder>,
}

fn default_version() -> u32 {
    FolderMemoryFile::CURRENT_VERSION
}

impl Default for FolderMemoryFile {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            folders: Vec::new(),
        }
    }
}

impl Versioned for FolderMemoryFile {
    const CURRENT_VERSION: u32 = 1;
    fn version(&self) -> u32 {
        self.version
    }
    fn set_version(&mut self, v: u32) {
        self.version = v;
    }
}

/// Remembered-folder service. `SettingsFile` is `Clone` (it shares the in-memory state +
/// writer), so cloning hands out views over the same live file.
#[derive(Clone)]
pub struct FolderMemoryService {
    file: SettingsFile<FolderMemoryFile>,
}

impl FolderMemoryService {
    /// Open `folders.toml` under `paths` (cross-process safe).
    pub fn open(paths: &AppPaths) -> Result<Self, SettingsFileError> {
        Self::open_with_delay(paths, SETTINGS_DEBOUNCE)
    }

    /// `delay` is accepted for call-site stability but has no effect.
    pub fn open_with_delay(paths: &AppPaths, _delay: Duration) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(paths.config_file("folders"), Migrator::new())?;
        Ok(Self { file })
    }

    /// Open at an explicit path. Test-only: production reaches the file through
    /// [`open`](Self::open) or [`in_memory_default`](Self::in_memory_default).
    #[cfg(test)]
    pub fn open_at(path: std::path::PathBuf) -> Result<Self, SettingsFileError> {
        let file = SettingsFile::load(path, Migrator::new())?;
        Ok(Self { file })
    }

    /// Graceful fallback when the config dir is unavailable: a throwaway per-process temp
    /// file, so the app still runs — the dialogs just start cold, which is exactly where
    /// they started before this file existed. Shares its retry/uniqueness logic with every
    /// sibling via [`in_memory_settings_file`](super::backup_settings_file::in_memory_settings_file).
    pub fn in_memory_default() -> Self {
        let file = super::backup_settings_file::in_memory_settings_file("folders", Migrator::new());
        Self { file }
    }

    /// Where a dialog for `purpose` should open, if there is a usable answer.
    ///
    /// A remembered directory that no longer exists returns `None` rather than being
    /// handed to the dialog: an external drive that is not mounted, or a folder since
    /// deleted, would otherwise take a native dialog somewhere it cannot go — and the
    /// platforms do not agree on what happens then. Falling back to the OS default is
    /// always safe and is what happened before anything was remembered.
    pub fn last(&self, purpose: FolderPurpose) -> Option<PathBuf> {
        let f = self.file.borrow();
        let path = f
            .folders
            .iter()
            .find(|r| r.purpose == purpose)
            .map(|r| PathBuf::from(&r.path))?;
        path.is_dir().then_some(path)
    }

    /// Remember the directory `chosen` sits in — for a dialog that picked a *file*.
    ///
    /// The parent, not the file: reopening a picker inside the folder the writer last
    /// took something from is the useful behaviour, and a file path is not a directory a
    /// dialog can open at.
    pub fn remember_file(&self, purpose: FolderPurpose, chosen: &Path) {
        if let Some(parent) = chosen.parent() {
            self.remember_dir(purpose, parent);
        }
    }

    /// Remember `dir` itself — for a dialog that picked a *directory*.
    ///
    /// A blank or relative path is ignored rather than stored: it could not be handed
    /// back to a dialog usefully, and writing it would only push a real answer out.
    pub fn remember_dir(&self, purpose: FolderPurpose, dir: &Path) {
        if !dir.is_absolute() {
            return;
        }
        let Some(path) = dir.to_str().map(str::to_string) else {
            return;
        };
        if path.trim().is_empty() {
            return;
        }
        let _ = self.file.mutate(|f| {
            match f.folders.iter_mut().find(|r| r.purpose == purpose) {
                Some(row) => row.path = path,
                None => f.folders.push(RememberedFolder { purpose, path }),
            }
            f.version = FolderMemoryFile::CURRENT_VERSION;
        });
    }
}

/// Open `req` where this purpose was last answered, if there is a usable answer.
///
/// A free function taking the context rather than a method on the service, so a call
/// site is one line and needs no plumbing: the service is Tier 1 (one per process, app
/// configuration — the one tier `app_state` is genuinely right for, unlike the
/// per-Work and per-window state whose doc comments warn against it).
///
/// Silently a no-op when no service is registered, which is what makes it safe to write
/// unconditionally at every call site: a headless test builds no settings file and must
/// not have to care.
pub fn dialog_start_in(
    ctx: &bastyde::prelude::EventContext,
    purpose: FolderPurpose,
    req: bastyde::prelude::FileDialogRequest,
) -> bastyde::prelude::FileDialogRequest {
    match ctx
        .app_state::<FolderMemoryService>()
        .and_then(|svc| svc.last(purpose))
    {
        Some(dir) => req.starting_dir(dir),
        None => req,
    }
}

/// Start `field` where this purpose was last answered.
///
/// The build-time twin of [`dialog_start_in`], for [`FilePickerField`], which builds
/// its dialog itself when the writer presses Browse and so must be told the directory
/// up front — there is no `EventContext` at that point, only the `BuildContext` the
/// widget is being assembled in.
pub fn picker_starts_in(
    ctx: &bastyde::core::build_context::BuildContext,
    purpose: FolderPurpose,
    field: bastyde::widgets::FilePickerField,
) -> bastyde::widgets::FilePickerField {
    match ctx
        .app_state::<FolderMemoryService>()
        .and_then(|svc| svc.last(purpose))
    {
        Some(dir) => field.starting_dir(dir),
        None => field,
    }
}

/// Remember the folder a *file* was picked from or saved to.
pub fn remember_dialog_file(
    ctx: &bastyde::prelude::EventContext,
    purpose: FolderPurpose,
    chosen: &Path,
) {
    if let Some(svc) = ctx.app_state::<FolderMemoryService>() {
        svc.remember_file(purpose, chosen);
    }
}

/// Remember a picked *directory*.
pub fn remember_dialog_dir(
    ctx: &bastyde::prelude::EventContext,
    purpose: FolderPurpose,
    dir: &Path,
) {
    if let Some(svc) = ctx.app_state::<FolderMemoryService>() {
        svc.remember_dir(purpose, dir);
    }
}

/// Remember wherever a dialog's answer landed, whatever shape the answer took.
///
/// The **write twin of [`picker_starts_in`]**, and the reason it exists is that the
/// two were not twins before: a `FilePickerField` is handed its starting directory
/// at build time, so recording where the writer actually went is a *separate* call
/// on a *separate* hook (`on_pick`) — easy to add the reading half and never notice
/// the writing half is missing. Six of the app's dialogs were exactly that: they
/// opened where you last were and then forgot where you went, so the memory could
/// only ever be updated by some *other* dialog sharing the purpose.
///
/// One function over the whole `FileDialogResult` rather than a `match` at each
/// call site: every site then reads `.on_pick(move |res, ctx| remember_pick(ctx, P, res))`,
/// and a new dialog that forgets it is visibly missing a line rather than subtly
/// missing an arm. Cancellation (`None` / an empty `Vec`) and `Error` record
/// nothing — the writer did not choose a folder, so there is nothing to learn.
pub fn remember_pick(
    ctx: &bastyde::prelude::EventContext,
    purpose: FolderPurpose,
    result: &bastyde::prelude::FileDialogResult,
) {
    use bastyde::prelude::FileDialogResult as R;
    match result {
        R::File(Some(path)) | R::Saved(Some(path)) => remember_dialog_file(ctx, purpose, path),
        R::Files(paths) => {
            if let Some(path) = paths.first() {
                remember_dialog_file(ctx, purpose, path);
            }
        }
        R::Folder(Some(dir)) => remember_dialog_dir(ctx, purpose, dir),
        R::File(None) | R::Saved(None) | R::Folder(None) | R::Error(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service() -> (FolderMemoryService, tempfile::TempDir) {
        let dir = tempfile::tempdir().expect("tempdir");
        let svc =
            FolderMemoryService::open_at(dir.path().join("folders.toml")).expect("open settings");
        (svc, dir)
    }

    #[test]
    fn nothing_is_remembered_until_something_is_chosen() {
        let (svc, _d) = service();
        for purpose in FolderPurpose::ALL {
            assert_eq!(
                svc.last(*purpose),
                None,
                "{purpose:?} started with an answer"
            );
        }
    }

    /// Every purpose must survive the TOML round trip. A variant whose serde name
    /// collides with another's, or that the file cannot represent, would read back as the
    /// wrong folder — and the only symptom would be one dialog opening in another's
    /// directory.
    #[test]
    fn every_purpose_remembers_its_own_directory() {
        let (svc, d) = service();
        let mut expected = Vec::new();
        for (i, purpose) in FolderPurpose::ALL.iter().enumerate() {
            let sub = d.path().join(format!("dir{i}"));
            std::fs::create_dir_all(&sub).expect("mkdir");
            svc.remember_dir(*purpose, &sub);
            expected.push((*purpose, sub));
        }

        // Re-opened from disk, so this is the file talking and not the in-memory copy.
        let reopened =
            FolderMemoryService::open_at(d.path().join("folders.toml")).expect("reopen settings");
        for (purpose, dir) in expected {
            assert_eq!(
                reopened.last(purpose),
                Some(dir),
                "{purpose:?} did not survive the round trip"
            );
        }
    }

    #[test]
    fn a_picked_file_remembers_the_folder_it_came_from() {
        let (svc, d) = service();
        let sub = d.path().join("manuscripts");
        std::fs::create_dir_all(&sub).expect("mkdir");

        svc.remember_file(FolderPurpose::OpenProject, &sub.join("novel.skrib"));

        assert_eq!(
            svc.last(FolderPurpose::OpenProject),
            Some(sub),
            "a file pick must remember its parent directory, not the file"
        );
    }

    #[test]
    fn choosing_again_replaces_rather_than_appends() {
        let (svc, d) = service();
        let first = d.path().join("one");
        let second = d.path().join("two");
        std::fs::create_dir_all(&first).expect("mkdir");
        std::fs::create_dir_all(&second).expect("mkdir");

        svc.remember_dir(FolderPurpose::Export, &first);
        svc.remember_dir(FolderPurpose::Export, &second);

        assert_eq!(svc.last(FolderPurpose::Export), Some(second));
        assert_eq!(
            svc.file.borrow().folders.len(),
            1,
            "one purpose is one row, however many times it is answered"
        );
    }

    /// The case that would otherwise send a native dialog somewhere it cannot go.
    #[test]
    fn a_remembered_folder_that_has_gone_away_is_not_offered() {
        let (svc, d) = service();
        let gone = d.path().join("on-a-detached-drive");
        std::fs::create_dir_all(&gone).expect("mkdir");
        svc.remember_dir(FolderPurpose::Export, &gone);
        assert!(svc.last(FolderPurpose::Export).is_some());

        std::fs::remove_dir(&gone).expect("rmdir");
        assert_eq!(
            svc.last(FolderPurpose::Export),
            None,
            "an unreachable directory must fall back to the OS default, not be handed over"
        );
    }

    /// A relative path cannot be handed to a dialog, and storing one would evict the real
    /// answer for that purpose.
    #[test]
    fn a_relative_path_is_not_remembered() {
        let (svc, d) = service();
        let real = d.path().join("real");
        std::fs::create_dir_all(&real).expect("mkdir");
        svc.remember_dir(FolderPurpose::SaveAs, &real);

        svc.remember_dir(FolderPurpose::SaveAs, Path::new("../elsewhere"));

        assert_eq!(
            svc.last(FolderPurpose::SaveAs),
            Some(real),
            "a relative path must be ignored rather than overwrite a usable answer"
        );
    }
}

/// A source sweep, not a behaviour test — the defect it guards is *omission*, and
/// no behaviour test can see a call site that was never written.
#[cfg(test)]
mod pairing_tests {
    /// Every dialog that opens where the writer last was must also record where
    /// they went.
    ///
    /// **Regression.** `picker_starts_in` is called at *build* time and
    /// `remember_pick` on the `on_pick` hook, so the two halves live in different
    /// places and adding only the first is invisible: the dialog opens in the right
    /// folder and simply never learns a new one. Six surfaces shipped that way —
    /// the export destination, the new-project location, the Plume source, both
    /// Hunspell pickers, and the template import — each of which could only ever be
    /// updated by some *other* dialog that happened to share its `FolderPurpose`.
    ///
    /// Reads the sources rather than exercising the widgets because that is where
    /// the fault is. `picker_starts_in` wraps a `FilePickerField`; this asserts the
    /// same expression also carries an `on_pick`. Crude, and it catches exactly the
    /// mistake that was made five more times than anybody noticed.
    #[test]
    fn every_picker_that_starts_in_a_remembered_folder_also_records_the_new_one() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut offenders: Vec<String> = Vec::new();

        for path in walk(&root) {
            let text = std::fs::read_to_string(&path).expect("read source");
            // Each `picker_starts_in(` call, to the end of its argument list.
            for (index, _) in text.match_indices("picker_starts_in(") {
                // `models/folder_memory_file.rs` declares it; it does not call it.
                if path.ends_with("folder_memory_file.rs") {
                    continue;
                }
                let rest = &text[index..];
                let end = balanced_end(rest).unwrap_or(rest.len());
                if !rest[..end].contains("on_pick") {
                    let line = text[..index].matches('\n').count() + 1;
                    offenders.push(format!(
                        "{}:{line}",
                        path.strip_prefix(&root).unwrap_or(&path).display()
                    ));
                }
            }
        }

        assert!(
            offenders.is_empty(),
            "these pickers start in a remembered folder but never record the one the \
             writer chose — add `.on_pick(|res, ctx| remember_pick(ctx, <purpose>, res))`:\n  {}",
            offenders.join("\n  ")
        );
    }

    /// Byte offset just past the `(` … `)` opened at the start of `text`.
    fn balanced_end(text: &str) -> Option<usize> {
        let open = text.find('(')?;
        let mut depth = 0i32;
        for (i, ch) in text[open..].char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(open + i + 1);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn walk(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
        let mut out = Vec::new();
        let Ok(entries) = std::fs::read_dir(dir) else {
            return out;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                out.extend(walk(&path));
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
        out
    }
}
