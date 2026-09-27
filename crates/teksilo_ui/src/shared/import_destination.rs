// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The destination half of the two project importers' forms, Plume Creator and
//! Manuskript: a folder and a file name that become `<folder>/<name>.skrib`.
//!
//! Shared because both forms ask the same two questions of the same disk in the
//! same way; each feature supplies only its own words ([`DestinationMessages`]).
//! Besides the path arithmetic, two rules live here:
//!
//! * **The checks are cached.** The folder check writes a probe file, and a
//!   derived signal would run it on every read (see [`super::form_checks`]).
//!   Both verdicts are worked out when the folder or the name is set,
//!   [`ImportDestination::recheck`] runs them again at the moment of importing,
//!   and [`ImportDestination::retry_refused_folder`] looks at a refused folder
//!   again while the dialog is up.
//! * **An open project is never a destination** ([`refuse_if_open`]). An import
//!   writes a whole new `.skrib` over its target. When that target is a project
//!   open in a window, the window keeps the old project in memory: until its
//!   next save the writer's file on disk is the import rather than what they are
//!   looking at, and that save then writes the old project straight back over
//!   the import. Both halves lose work, so the import refuses and says why. The
//!   same goes the other way for as long as the import runs: it holds its target
//!   ([`crate::shell::open_registry::claim_import`]), and the doors that open a
//!   project refuse one an import is writing.

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::{MessageBox, MessageBoxButtons, ValidationState};

use crate::sessions::WorkRegistry;
use crate::shared::form_checks::{CachedValidation, FolderMessages, folder_state};
use crate::shell::open_registry;

/// What a destination can say, in the words of the importer that owns it.
pub(crate) struct DestinationMessages {
    pub(crate) folder: FolderMessages,
    pub(crate) name_required: fn() -> LocalizedString,
    /// A warning, not an error: the import proceeds after an overwrite
    /// confirmation.
    pub(crate) name_exists: fn() -> LocalizedString,
}

/// Build the target `<dir>/<name>.skrib` (empty when the name is blank). A
/// trailing `.skrib` the writer typed is not doubled.
pub(crate) fn build_target(dir: &str, name: &str) -> String {
    let name = name.trim();
    let name = name.strip_suffix(".skrib").unwrap_or(name).trim();
    if name.is_empty() {
        return String::new();
    }
    let dir = dir.trim().trim_end_matches(['/', '\\']);
    let sep = if dir.is_empty() { "" } else { "/" };
    format!("{dir}{sep}{name}.skrib")
}

fn name_state(target: &str, messages: &DestinationMessages) -> ValidationState {
    if target.is_empty() {
        ValidationState::Error((messages.name_required)())
    } else if Path::new(target).exists() {
        ValidationState::Warning((messages.name_exists)())
    } else {
        ValidationState::None
    }
}

/// The destination folder and file name of an import form, with their checks.
///
/// Cheap to clone; clones share every signal.
#[derive(Clone)]
pub(crate) struct ImportDestination {
    location: Signal<String>,
    name: Signal<String>,
    location_check: CachedValidation,
    name_check: CachedValidation,
}

impl ImportDestination {
    pub(crate) fn new(messages: &'static DestinationMessages) -> Self {
        let location = Signal::new(String::new());
        let name = Signal::new(String::new());
        let location_check = {
            let dir = location.clone();
            CachedValidation::new(&[&location], move || {
                folder_state(&dir.get(), &messages.folder)
            })
        };
        // Over both fields: the file that may already exist is `<folder>/<name>`,
        // so a new folder can make the same name collide.
        let name_check = {
            let (dir, stem) = (location.clone(), name.clone());
            CachedValidation::new(&[&location, &name], move || {
                name_state(&build_target(&dir.get(), &stem.get()), messages)
            })
        };
        Self {
            location,
            name,
            location_check,
            name_check,
        }
    }

    /// The destination folder, bound by the view's folder picker.
    pub(crate) fn location(&self) -> Signal<String> {
        self.location.clone()
    }

    /// The output base name (a `.skrib` is appended), bound by the view.
    pub(crate) fn name(&self) -> Signal<String> {
        self.name.clone()
    }

    /// Empty both fields, for a form being opened afresh.
    pub(crate) fn clear(&self) {
        self.location.set(String::new());
        self.name.set(String::new());
    }

    /// Default both fields from a picked source: its folder and `stem`.
    pub(crate) fn default_from(&self, folder: Option<&str>, stem: String) {
        if let Some(folder) = folder {
            self.location.set(folder.to_string());
        }
        self.name.set(stem);
    }

    /// The `.skrib` the import would write, as of now.
    pub(crate) fn target(&self) -> String {
        build_target(&self.location.get(), &self.name.get())
    }

    /// The reactive "Will create `…/<name>.skrib`" preview. Pure string work.
    pub(crate) fn target_path(&self) -> Signal<String> {
        self.location
            .zip(&self.name)
            .map(|(dir, name)| build_target(dir, name))
    }

    /// The folder field's cached verdict.
    pub(crate) fn location_validation(&self) -> Signal<ValidationState> {
        self.location_check.signal()
    }

    /// The name field's cached verdict: blank is an error, an existing target a
    /// warning.
    pub(crate) fn name_validation(&self) -> Signal<ValidationState> {
        self.name_check.signal()
    }

    /// Whether both verdicts let the import proceed. Derived from the cached
    /// verdicts only, so reading it touches nothing.
    pub(crate) fn is_ready(&self) -> Signal<bool> {
        self.location_check.passes().and(&self.name_check.passes())
    }

    /// Check both fields against the disk again, now, and report whether the
    /// import may go ahead. The fields show the fresh verdicts either way.
    pub(crate) fn recheck(&self) -> bool {
        let folder_ok = self.location_check.recheck();
        let name_ok = self.name_check.recheck();
        folder_ok && name_ok
    }

    /// Whether the folder field names a folder the disk refused (missing, not
    /// a folder, read-only). The cached verdict only; a blank field is not a
    /// refusal the disk could lift.
    pub(crate) fn folder_refused(&self) -> bool {
        !self.location.get().trim().is_empty() && self.location_check.refuses()
    }

    /// Look at a refused folder again. The name's verdict follows it, since
    /// whether `<folder>/<name>.skrib` already exists depends on the folder.
    pub(crate) fn retry_refused_folder(&self) {
        if self.folder_refused() {
            self.location_check.recheck();
            self.name_check.recheck();
        }
    }
}

/// The target of the import a form is running, held for as long as it runs: see
/// [`open_registry::claim_import`]. Cheap to clone; clones share the hold.
#[derive(Clone, Default)]
pub(crate) struct TargetHold(Rc<RefCell<Option<open_registry::ImportClaim>>>);

impl TargetHold {
    /// Hold `target` for an import about to start, in place of anything held before.
    pub(crate) fn hold(&self, target: &str) {
        let claim = open_registry::claim_import(target);
        self.0.replace(Some(claim));
    }

    /// Hold `target` for an import about to start, then look again for a window
    /// holding it open, or opening it: when one does, let go, say why in the importers'
    /// words, and return `false`.
    ///
    /// The look before (when Import was pressed) cannot see a load another copy of
    /// Skribisto started since, and a load claims its project before it starts
    /// (`open_registry::claim_for_load`). Claiming first and looking second, as that
    /// load does the other way round, is what guarantees one of the two sees the other.
    pub(crate) fn hold_unless_open(&self, ctx: &mut EventContext, target: &str) -> bool {
        self.hold(target);
        if open_project_at(ctx, target).is_none() {
            return true;
        }
        self.release();
        present_refusal(ctx, target, &IMPORT_OPEN);
        false
    }

    /// Let go of the target: the import completed, failed, was cancelled or never
    /// started.
    pub(crate) fn release(&self) {
        self.0.replace(None);
    }
}

/// Refuse to start an import while `active`, the operation the same form started last,
/// still runs: tell the writer why and return `true`. Returns `false`, having shown
/// nothing, when it does not.
///
/// A form tracks one import at a time: its progress notice, its Cancel button and the
/// hold on its target ([`TargetHold`]) all belong to that one. A second started beside
/// it would take all three over, and the first import's target would be let go of while
/// that import still meant to replace it. Asked when Import is pressed, and again at the
/// last moment, since an overwrite question can wait while another window starts one.
///
/// `words` are the form's own: a title and a text, the text taking no argument.
pub(crate) fn refuse_if_busy(
    ctx: &mut EventContext,
    active: &Signal<Option<String>>,
    words: &BusyRefusal,
) -> bool {
    if active.get().is_none() {
        return false;
    }
    MessageBox::warning((words.title)())
        .text((words.text)())
        .buttons(MessageBoxButtons::Ok)
        .present(ctx);
    true
}

/// The words [`refuse_if_busy`] is said in, by the form that refuses.
pub(crate) struct BusyRefusal {
    pub(crate) title: fn() -> LocalizedString,
    pub(crate) text: fn() -> LocalizedString,
}

/// The open project sitting at `target`, if any.
///
/// Compared by the one spelling every door agrees on (`open_registry::canonical`:
/// `skrib_format::canonical_project_path`, then the filesystem's own canonical
/// form), so a folder project named by its `project.skrib` and a path reached
/// through a symlink are still recognised. Two sources, because neither alone is
/// complete: this process's Work registry holds every project open in one of its
/// windows, whether or not its lock file could be written; the open registry's
/// lock files add every other running copy of Skribisto.
///
/// Reads the lock directory, so it runs when the writer presses Import, never in
/// a derived signal.
pub(crate) fn open_project_at(ctx: &EventContext, target: &str) -> Option<String> {
    let here = ctx
        .app_state::<WorkRegistry>()
        .map(open_in_this_process)
        .unwrap_or_default();
    let elsewhere = open_registry::scan_open()
        .into_iter()
        .map(|entry| entry.path);
    matching_open_project(target, here.into_iter().chain(elsewhere))
}

/// The file path of every Work open in a window of this process. An unsaved
/// project has none and is skipped.
fn open_in_this_process(registry: &WorkRegistry) -> Vec<String> {
    registry
        .open_work_ids()
        .into_iter()
        .filter_map(|work_id| registry.session_for(work_id))
        .filter_map(|session| session.single_work_info.file_name().get())
        .collect()
}

/// The first of `open` that names the same project as `target`.
fn matching_open_project(target: &str, open: impl IntoIterator<Item = String>) -> Option<String> {
    let target = target.trim();
    if target.is_empty() {
        return None;
    }
    let wanted = open_registry::canonical(target);
    open.into_iter()
        .filter(|path| !path.trim().is_empty())
        .find(|path| open_registry::canonical(path) == wanted)
}

/// Refuse an import aimed at a project that is open: tell the writer why, and
/// return `true`. Returns `false`, having shown nothing, when `target` is free.
pub(crate) fn refuse_if_open(ctx: &mut EventContext, target: &str) -> bool {
    refuse_if_open_saying(ctx, target, &IMPORT_OPEN)
}

/// The words a refusal of an open target is said in: a title, and a text naming the
/// project by its file name.
pub(crate) struct OpenRefusal {
    pub(crate) title: fn() -> LocalizedString,
    pub(crate) text: fn(String) -> LocalizedString,
}

static IMPORT_OPEN: OpenRefusal = OpenRefusal {
    title: || tr!(import_target_open_title()),
    text: |name| tr!(import_target_open_text(name = name)),
};

/// [`refuse_if_open`], in the words of whatever else would write a whole project over
/// `target`: New Work creating one in its place, for one.
///
/// A target an import is still writing is refused the same way, in words of its own
/// ([`refuse_if_importing`]).
pub(crate) fn refuse_if_open_saying(
    ctx: &mut EventContext,
    target: &str,
    words: &OpenRefusal,
) -> bool {
    if refuse_if_importing(ctx, target, &TARGET_IMPORTING) {
        return true;
    }
    if open_project_at(ctx, target).is_none() {
        return false;
    }
    present_refusal(ctx, target, words);
    true
}

/// Refuse to start writing a project over `target` while an import is writing it
/// ([`open_registry::importing`]): tell the writer why, in `words`, and return `true`.
/// Returns `false`, having shown nothing, when no import is.
///
/// The import replaces its target when it finishes, whatever was written there
/// meanwhile. Every door that writes a whole project to a path the writer chose asks
/// this first: New Work and the importers (through [`refuse_if_open_saying`]), Save As
/// and a backup's restore.
pub(crate) fn refuse_if_importing(
    ctx: &mut EventContext,
    target: &str,
    words: &OpenRefusal,
) -> bool {
    if !open_registry::importing(target) {
        return false;
    }
    present_refusal(ctx, target, words);
    true
}

/// The refusal of a target an import is writing, for a door where the writer can
/// pick another name.
pub(crate) static TARGET_IMPORTING: OpenRefusal = OpenRefusal {
    title: || tr!(target_importing_title()),
    text: |name| tr!(target_importing_text(name = name)),
};

/// Say why `target` is refused, naming it by its file name.
fn present_refusal(ctx: &mut EventContext, target: &str, words: &OpenRefusal) {
    let name = Path::new(target)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| target.to_string());
    MessageBox::warning((words.title)())
        .text((words.text)(name))
        .buttons(MessageBoxButtons::Ok)
        .present(ctx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::i18n::lit;

    static MESSAGES: DestinationMessages = DestinationMessages {
        folder: FolderMessages {
            required: || lit!("required"),
            missing: || lit!("missing"),
            not_folder: || lit!("not a folder"),
            readonly: || lit!("read-only"),
        },
        name_required: || lit!("name required"),
        name_exists: || lit!("exists"),
    };

    #[test]
    fn build_target_appends_skrib_once() {
        assert_eq!(
            build_target("/books", "Le Visiteur"),
            "/books/Le Visiteur.skrib"
        );
        assert_eq!(
            build_target("/books/", "Le Visiteur"),
            "/books/Le Visiteur.skrib"
        );
        assert_eq!(
            build_target("/books", "Le Visiteur.skrib"),
            "/books/Le Visiteur.skrib"
        );
        assert_eq!(build_target("/books", "   "), "");
    }

    /// The name's verdict follows both fields: a name that is free in one folder
    /// collides in the folder that already holds that file.
    #[test]
    fn the_name_verdict_follows_the_folder_too() {
        let empty = tempfile::tempdir().unwrap();
        let full = tempfile::tempdir().unwrap();
        std::fs::write(full.path().join("novel.skrib"), b"x").unwrap();
        let dest = ImportDestination::new(&MESSAGES);
        let verdict = dest.name_validation();

        dest.location()
            .set(empty.path().to_string_lossy().into_owned());
        assert!(matches!(verdict.get(), ValidationState::Error(_)));
        dest.name().set("novel".into());
        assert!(matches!(verdict.get(), ValidationState::None));
        dest.location()
            .set(full.path().to_string_lossy().into_owned());
        assert!(matches!(verdict.get(), ValidationState::Warning(_)));
        assert!(dest.is_ready().get(), "a warning does not block the import");
    }

    /// Reading the verdicts never touches the disk: a file created behind the
    /// form's back is not noticed until something is typed or a recheck runs.
    #[test]
    fn reading_the_verdicts_never_touches_the_disk() {
        let dir = tempfile::tempdir().unwrap();
        let dest = ImportDestination::new(&MESSAGES);
        dest.location()
            .set(dir.path().to_string_lossy().into_owned());
        dest.name().set("novel".into());
        let name_verdict = dest.name_validation();
        let ready = dest.is_ready();
        assert!(matches!(name_verdict.get(), ValidationState::None));

        std::fs::write(dir.path().join("novel.skrib"), b"x").unwrap();
        std::fs::remove_dir_all(dir.path()).unwrap();
        for _ in 0..10 {
            assert!(matches!(name_verdict.get(), ValidationState::None));
            assert!(ready.get());
        }
        assert!(!dest.recheck(), "the folder is gone");
        assert!(!ready.get());
    }

    /// Canonical comparison: the folder spelling and the `project.skrib`
    /// spelling of one folder project are one project, and a path through a
    /// symlink is the file it points at.
    #[test]
    fn an_open_project_is_recognised_under_any_spelling() {
        let dir = tempfile::tempdir().unwrap();
        let zip = dir.path().join("novel.skrib");
        std::fs::write(&zip, b"PK").unwrap();
        let folder = dir.path().join("Folder.skrib");
        std::fs::create_dir(&folder).unwrap();
        std::fs::write(folder.join("project.skrib"), b"(manifest)").unwrap();
        let zip_s = zip.to_string_lossy().into_owned();
        let folder_s = folder.to_string_lossy().into_owned();

        assert_eq!(
            matching_open_project(&zip_s, [String::new(), zip_s.clone()]),
            Some(zip_s.clone())
        );
        assert_eq!(
            matching_open_project(
                &folder_s,
                [folder.join("project.skrib").to_string_lossy().into_owned()]
            ),
            Some(folder.join("project.skrib").to_string_lossy().into_owned()),
            "the manifest spelling is the folder project"
        );
        assert_eq!(
            matching_open_project(
                &format!("{}/./novel.skrib", dir.path().display()),
                [zip_s.clone()]
            ),
            Some(zip_s.clone())
        );
        #[cfg(unix)]
        {
            let link = dir.path().join("link.skrib");
            std::os::unix::fs::symlink(&zip, &link).unwrap();
            assert_eq!(
                matching_open_project(&link.to_string_lossy(), [zip_s.clone()]),
                Some(zip_s.clone())
            );
        }
        assert_eq!(
            matching_open_project(&dir.path().join("other.skrib").to_string_lossy(), [zip_s]),
            None
        );
        assert_eq!(matching_open_project("", [String::new()]), None);
    }
}
