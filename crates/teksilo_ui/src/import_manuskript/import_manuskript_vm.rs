// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ImportManuskriptViewModel` — the Import Manuskript form's business logic.
//!
//! Single-instance live state, created once in `startup.rs` and registered as
//! app-state (so the panel can reach it). It owns the form's `Signal`s (the
//! source project, the destination folder, the output name).
//!
//! **The source may be a file or a folder**, which is what makes this form
//! different from the Plume one beside it. A Manuskript project in its modern
//! default is a one-byte `.msk` plus a sibling folder of the same name; in
//! single-file mode it is a zipped `.msk`; and a writer keeping the project in
//! version control thinks of the folder itself as the project. All three are
//! accepted, so the panel offers a Browse for each and this validator takes
//! either.
//!
//! Everything from the moment **Import** is pressed belongs to the shared
//! `shared::project_import::ProjectImportJob`, which this form drives through
//! [`MANUSKRIPT`]: the overwrite question, the refusal of a project that is open,
//! one import at a time, the progress notice, the result with **Open now**, the
//! report, and a failure worded in the writer's language. What stays here is what
//! only Manuskript has: its source, its request, and its words
//! (`super::messages`).
//!
//! The destination is `shared::import_destination`, shared with the Plume form:
//! its checks run when a field changes rather than on every read.

use std::path::Path;

use teksilo::prelude::*;
use teksilo::widgets::ValidationState;

use frontend::AppContext;
use frontend::commands::import_management_commands;
use frontend::import_management::ImportManuskriptProjectDto;
use skrib_format::importing::TITLE_SLOT;

use super::messages;
use crate::intents::AppIntent;
use crate::shared::form_checks::{CachedValidation, DiskChecked, FolderMessages};
use crate::shared::import_destination::{DestinationMessages, ImportDestination};
use crate::shared::import_warnings::{MANUSKRIPT as NOTICE, notice_sentence};
use crate::shared::project_import::{ImportOutcome, Importer, ProjectImportJob};

/// The Manuskript importer, as the shared job drives it.
pub(crate) static MANUSKRIPT: Importer<ImportManuskriptProjectDto> = Importer {
    toast_id: "import.manuskript",
    notice: &NOTICE,
    progress_title: || tr!(import_manuskript_progress_title()),
    own_words: messages::own_failure,
    target: |request| request.output_path.clone(),
    allow_overwrite: |request| ImportManuskriptProjectDto {
        overwrite: true,
        ..request
    },
    launch: |app_ctx, request| {
        import_management_commands::import_manuskript_project(app_ctx, request)
    },
    outcome,
    source: |request| request.source_path.clone(),
    review_again: None,
    open_now: |path| AppIntent::OpenWorkPath { path },
};

/// The finished import's result, in the writer's words.
fn outcome(app_ctx: &AppContext, op_id: &str) -> Option<ImportOutcome> {
    let result =
        import_management_commands::get_import_manuskript_project_result(app_ctx, op_id).ok()??;
    Some(ImportOutcome {
        result: tr!(import_manuskript_done(
            imported = result.imported_items,
            revisions = result.imported_revisions
        )),
        report: result
            .warnings
            .iter()
            .map(|raw| notice_sentence(raw))
            .collect(),
        output_path: result.output_path,
        persistent: false,
        headline: None,
        see_report: None,
    })
}

/// The destination fields' words.
static DESTINATION: DestinationMessages = DestinationMessages {
    folder: FolderMessages {
        required: || tr!(project_import_location_required()),
        missing: || tr!(project_import_location_missing()),
        not_folder: || tr!(project_import_location_not_folder()),
        readonly: || tr!(project_import_location_readonly()),
    },
    name_required: || tr!(project_import_name_required()),
    name_exists: || tr!(project_import_name_exists()),
    name_is_folder: || tr!(project_import_name_is_folder()),
};

/// Strip a `.msk` extension from a source path's file name, yielding the default
/// output base name.
///
/// A folder source has no extension to strip, and its own name is already the
/// project's name — the `.msk` and the folder always share a stem, which is how
/// Manuskript finds one from the other.
fn output_stem(source: &str) -> String {
    let path = Path::new(source);
    let file = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    match file.to_ascii_lowercase().strip_suffix(".msk") {
        Some(base) => file[..base.len()].to_string(),
        None => file.to_string(),
    }
}

/// Validate the source: a non-blank path to something that exists.
///
/// Deliberately accepts a directory as well as a file. The importer works out
/// which of the three shapes it is looking at, and refusing a folder here would
/// reject the one a writer with the project in git is most likely to point at.
///
/// Touches the disk, so it runs in a [`CachedValidation`], never on a read.
fn source_state(source: &str) -> ValidationState {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return ValidationState::Error(tr!(import_manuskript_source_required()));
    }
    if !Path::new(trimmed).exists() {
        return ValidationState::Error(tr!(import_manuskript_source_missing()));
    }
    ValidationState::None
}

#[derive(Clone)]
pub struct ImportManuskriptViewModel {
    /// The chosen project: a `.msk` of either kind, or a project folder.
    source: Signal<String>,
    /// [`source_state`] of `source`, worked out when it changes.
    source_check: CachedValidation,
    /// The destination folder and output name, with their checks.
    destination: ImportDestination,
    /// The process's one project-import job, shared with every other importer.
    job: ProjectImportJob,
}

impl ImportManuskriptViewModel {
    pub(crate) fn new(job: ProjectImportJob) -> Self {
        let source = Signal::new(String::new());
        let source_check = {
            let picked = source.clone();
            CachedValidation::new(&[&source], move || source_state(&picked.get()))
        };
        Self {
            source,
            source_check,
            destination: ImportDestination::new(&DESTINATION),
            job,
        }
    }

    /// Clear the form fields — called when the panel is (re)opened so a previous
    /// session's paths don't linger. The running import, if any, is the job's.
    pub fn reset_form(&self) {
        self.source.set(String::new());
        self.destination.clear();
    }

    // ── Signal accessors (bound by the view) ───────────────────────────────
    pub fn source(&self) -> Signal<String> {
        self.source.clone()
    }
    pub fn location(&self) -> Signal<String> {
        self.destination.location()
    }
    pub fn name(&self) -> Signal<String> {
        self.destination.name()
    }

    /// React to either source picker: default the destination folder and name
    /// from what was picked.
    ///
    /// Both shapes are handled, because both buttons write into the same field. A
    /// picked **file** defaults the destination beside it; a picked **folder**
    /// defaults beside its parent, not inside itself, so the new `.skrib` lands
    /// next to the project it came from rather than inside it.
    pub fn apply_source_defaults(&self, res: &FileDialogResult) {
        let picked = match res {
            FileDialogResult::File(Some(path)) => Some(path.clone()),
            FileDialogResult::Folder(Some(path)) => Some(path.clone()),
            _ => None,
        };
        let Some(path) = picked else { return };
        self.source.set(path.to_string_lossy().into_owned());
        self.destination.default_from(
            path.parent().and_then(|p| p.to_str()),
            output_stem(&path.to_string_lossy()),
        );
    }

    /// The reactive "Will create `…/<name>.skrib`" preview.
    pub fn target_path(&self) -> Signal<String> {
        self.destination.target_path()
    }

    /// The source field's verdict, cached: reading it never touches the disk.
    pub fn source_validation(&self) -> Signal<ValidationState> {
        self.source_check.signal()
    }
    /// The destination folder's verdict, cached: its check writes a probe file,
    /// which a derived signal would do on every frame the field is painted.
    pub fn location_validation(&self) -> Signal<ValidationState> {
        self.destination.location_validation()
    }

    /// Inline validation for the file-name field: blank → error; a name whose
    /// target `.skrib` already exists → a *warning* (import still proceeds, after
    /// an overwrite confirmation). Cached, like the folder's.
    pub fn name_validation(&self) -> Signal<ValidationState> {
        self.destination.name_validation()
    }

    /// Whether "Import" may fire: a valid source **and** a valid destination
    /// **and** a non-blank name. Derived from the cached verdicts only.
    pub fn can_import(&self) -> Signal<bool> {
        self.source_check.passes().and(&self.destination.is_ready())
    }

    /// Whether the source names a path the disk refused. The cached verdict
    /// only; a blank source is not a refusal the disk could lift.
    fn source_refused(&self) -> bool {
        !self.source.get().trim().is_empty() && self.source_check.refuses()
    }

    fn dto(&self, overwrite: bool) -> ImportManuskriptProjectDto {
        ImportManuskriptProjectDto {
            source_path: self.source.get(),
            output_path: self.destination.target(),
            overwrite,
            // Manuskript has no binders and no story-bible groups, so it stores
            // none of these names. Resolve them here, in the writer's locale, and
            // hand them down — the same treatment the Plume importer's two binder
            // names get.
            manuscript_binder_name: tr!(import_manuskript_manuscript_binder()).into(),
            story_bible_binder_name: tr!(import_manuskript_story_bible_binder()).into(),
            characters_group_name: tr!(import_manuskript_characters_group()).into(),
            world_group_name: tr!(import_manuskript_world_group()).into(),
            plots_group_name: tr!(import_manuskript_plots_group()).into(),
            project_info_note_name: tr!(import_manuskript_project_info_note()).into(),
            summary_note_name: tr!(import_manuskript_summary_note()).into(),
            // Lowest rung first. Manuskript stores an importance as 0, 1 or 2 and
            // names them in its UI, so the file carries no names at all.
            importance_names: vec![
                tr!(import_manuskript_importance_minor()).into(),
                tr!(import_manuskript_importance_secondary()).into(),
                tr!(import_manuskript_importance_main()).into(),
            ],
            // The headings of the notes the import writes and the titles it makes up,
            // in the order the importer reads them back. Seeded text, so the
            // interface's language, once.
            section_names: manuskript_import::map::SectionNames::ORDER
                .iter()
                .map(|key| section_name(key))
                .collect(),
        }
    }

    /// "Import": check every field against the disk again, then hand the request
    /// to the shared job, which refuses a second import and a target that is
    /// open, confirms an overwrite, then starts.
    pub fn import(&self, ctx: &mut EventContext) {
        // The verdicts on screen were worked out when the fields last changed,
        // and the disk may have moved on since. Both checks run, so every field
        // shows its fresh verdict, before either answer is acted on.
        let source_ok = self.source_check.recheck();
        let destination_ok = self.destination.recheck();
        if !(source_ok && destination_ok) {
            return;
        }
        self.job.import(ctx, &MANUSKRIPT, self.dto(false));
    }
}

/// The form's disk-checked fields, looked at again while the panel is on screen
/// and one of them is refused (see `shared::form_checks::retry_refusals`): a
/// source that appears or a folder created after its path was typed reopens
/// Import without an edit.
impl DiskChecked for ImportManuskriptViewModel {
    fn disk_verdicts(&self) -> Vec<Signal<ValidationState>> {
        vec![self.source_validation(), self.location_validation()]
    }

    fn refused_on_disk(&self) -> bool {
        self.source_refused() || self.destination.folder_refused()
    }

    fn retry_refused(&self) {
        if self.source_refused() {
            self.source_check.recheck();
        }
        self.destination.retry_refused_folder();
    }
}

/// The heading of one section of an imported note, by its key in
/// `SectionNames::ORDER`. Goal and Conflict are the note templates' own words, so an
/// imported character sheet reads like one made from the Character template. A key
/// this does not know is left blank, which the importer fills with its English.
fn section_name(key: &str) -> String {
    let name = match key {
        "paragraph_summary" => tr!(import_manuskript_section_paragraph_summary()),
        "full_summary" => tr!(import_manuskript_section_full_summary()),
        "motivation" => tr!(import_manuskript_section_motivation()),
        "goal" => tr!(note_template_field_goal()),
        "conflict" => tr!(note_template_field_conflict()),
        "epiphany" => tr!(import_manuskript_section_epiphany()),
        "notes" => tr!(import_manuskript_section_notes()),
        "description" => tr!(import_manuskript_section_description()),
        "passion" => tr!(import_manuskript_section_passion()),
        "result" => tr!(import_manuskript_section_result()),
        "summary" => tr!(import_manuskript_section_summary()),
        "meta" => tr!(import_manuskript_section_meta()),
        "series" => tr!(import_manuskript_section_series()),
        "volume" => tr!(import_manuskript_section_volume()),
        "genre" => tr!(import_manuskript_section_genre()),
        "license" => tr!(import_manuskript_section_license()),
        "email" => tr!(import_manuskript_section_email()),
        "situation" => tr!(import_manuskript_section_situation()),
        "sentence" => tr!(import_manuskript_section_sentence()),
        "paragraph" => tr!(import_manuskript_section_paragraph()),
        "page" => tr!(import_manuskript_section_page()),
        "full" => tr!(import_manuskript_section_full()),
        "untitled_project" => tr!(import_manuskript_untitled_project()),
        // The pattern's argument is the slot the converter fills with the row's title.
        "notes_of" => tr!(import_manuskript_notes_of(title = TITLE_SLOT.to_string())),
        _ => return String::new(),
    };
    name.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every heading the importer can write has a translation here: a key with no
    /// arm in `section_name` would quietly fall back to English in every language.
    #[test]
    fn every_section_heading_is_named() {
        for key in manuskript_import::map::SectionNames::ORDER {
            assert!(!section_name(key).trim().is_empty(), "{key} has no name");
        }
    }
    use std::any::{Any, TypeId};
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::time::{Duration, Instant};

    use frontend::commands::long_operation_commands;
    use teksilo::widgets::{StandardButton, Toast};

    use teksilo::core::ModalContent;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::widgets::{
        NotificationArchiveModel, NotificationEntry, ToastInstallOptions, ToastRegistry,
    };

    use frontend::common::event::{Event, LongOperationEvent, Origin};

    use crate::sessions::{WorkRegistry, WorkSession};
    use crate::shell::open_registry;
    use crate::test_support::{IsolatedOpenRegistry, click, drain_dialog_titles, press};

    fn fixture() -> String {
        format!(
            "{}/../manuskript_import/tests/fixtures/tour-du-monde.msk",
            env!("CARGO_MANIFEST_DIR")
        )
    }

    /// A tree whose app state holds `state`, as the running app's would.
    fn tree(app_ctx: &Rc<AppContext>, state: Vec<(TypeId, Box<dyn Any>)>) -> WidgetTree {
        crate::test_support::tree_with_app_state(
            app_ctx,
            state.into_iter().collect::<HashMap<_, _>>(),
        )
    }

    /// A form filled in with the fixture and `<dir>/novel.skrib`.
    fn filled(app_ctx: &Rc<AppContext>, dir: &Path) -> ImportManuskriptViewModel {
        let vm = form(app_ctx);
        vm.source().set(fixture());
        vm.location().set(dir.to_string_lossy().into_owned());
        vm.name().set("novel".into());
        vm
    }

    /// A form over a job of its own, as each running copy of Skribisto has one.
    fn form(app_ctx: &Rc<AppContext>) -> ImportManuskriptViewModel {
        ImportManuskriptViewModel::new(ProjectImportJob::new(app_ctx.clone()))
    }

    /// Every word the import writes into the project is resolved in the interface's
    /// language, in both shipped locales: never a message id, and the notes pattern holds
    /// the slot the row's title goes into exactly once.
    #[test]
    fn the_words_written_into_the_project_are_the_interface_s() {
        for locale in ["en-US", "fr-FR"] {
            crate::test_support::with_shipped_messages(locale, || {
                let dto = form(&Rc::new(AppContext::new())).dto(false);
                let mut words = vec![
                    &dto.manuscript_binder_name,
                    &dto.story_bible_binder_name,
                    &dto.characters_group_name,
                    &dto.world_group_name,
                    &dto.plots_group_name,
                    &dto.project_info_note_name,
                    &dto.summary_note_name,
                ];
                words.extend(dto.importance_names.iter());
                let order = manuskript_import::map::SectionNames::ORDER;
                assert_eq!(dto.section_names.len(), order.len());
                let mut notes_of = None;
                for (key, word) in order.iter().zip(&dto.section_names) {
                    if *key == "notes_of" {
                        notes_of = Some(word);
                    } else {
                        words.push(word);
                    }
                }
                for word in words {
                    assert!(
                        !word.trim().is_empty() && !word.starts_with("import-manuskript-"),
                        "{locale}: {word:?}"
                    );
                }
                let notes_of = notes_of.expect("the notes title is seeded");
                assert_eq!(
                    notes_of.matches(TITLE_SLOT).count(),
                    1,
                    "{locale}: {notes_of:?}"
                );
            });
        }
    }

    /// The title of the one dialog a press raised, if it raised one.
    fn only_dialog_title(tree: &mut WidgetTree) -> Option<String> {
        let titles = drain_dialog_titles(tree);
        assert!(titles.len() <= 1, "one dialog at most: {titles:?}");
        titles.into_iter().next()
    }

    #[test]
    fn output_stem_strips_a_msk_extension_only() {
        assert_eq!(output_stem("/books/Tour du monde.msk"), "Tour du monde");
        assert_eq!(output_stem("/books/TOUR.MSK"), "TOUR");
        assert_eq!(output_stem("/books/Tour du monde"), "Tour du monde");
    }

    /// Reading the form, as its fields and its Import button do on every frame,
    /// never touches the disk: a folder deleted behind the open dialog is still
    /// reported as it was until something is typed. Before, the folder check
    /// ran on each read, writing and deleting a probe file every time.
    #[test]
    fn reading_the_form_never_touches_the_disk() {
        let dir = tempfile::tempdir().unwrap();
        let vm = filled(&Rc::new(AppContext::new()), dir.path());
        let location = vm.location_validation();
        let source = vm.source_validation();
        let can_import = vm.can_import();
        assert!(matches!(location.get(), ValidationState::None));
        assert!(can_import.get());

        std::fs::remove_dir_all(dir.path()).unwrap();
        for _ in 0..50 {
            assert!(matches!(location.get(), ValidationState::None));
            assert!(matches!(source.get(), ValidationState::None));
            assert!(can_import.get());
        }

        // Typing is what reruns a check.
        vm.location().set(dir.path().to_string_lossy().into_owned());
        assert!(matches!(location.get(), ValidationState::Error(_)));
        assert!(!can_import.get());
    }

    /// Import checks the disk again before anything starts, and the fields show
    /// the fresh verdict.
    #[test]
    fn import_rechecks_the_disk_first() {
        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let vm = filled(&app_ctx, dir.path());
        std::fs::remove_dir_all(dir.path()).unwrap();

        let mut tree = tree(&app_ctx, Vec::new());
        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));

        assert!(vm.job.active_operation().is_none(), "nothing may start");
        assert!(matches!(
            vm.location_validation().get(),
            ValidationState::Error(_)
        ));
        assert_eq!(only_dialog_title(&mut tree), None);
    }

    /// The project the import would replace is open in a window of this
    /// process: the import is refused with a message saying so, before the
    /// overwrite question is even asked, and nothing starts.
    #[test]
    fn an_import_over_a_project_open_here_is_refused() {
        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("novel.skrib");
        std::fs::write(&target, b"PK").unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let vm = filled(&app_ctx, dir.path());

        let works = WorkRegistry::new();
        let session = WorkSession::for_test();
        session
            .single_work_info
            .file_name()
            .set(Some(target.to_string_lossy().into_owned()));
        works.register(7, session);
        let mut tree = tree(
            &app_ctx,
            vec![(TypeId::of::<WorkRegistry>(), Box::new(works))],
        );

        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));

        assert_eq!(
            only_dialog_title(&mut tree),
            Some(tr!(import_target_open_title()).resolve_now()),
            "the refusal, not the overwrite confirmation"
        );
        assert!(vm.job.active_operation().is_none(), "nothing may start");
        assert_eq!(
            std::fs::read(&target).unwrap(),
            b"PK",
            "the project is untouched"
        );
    }

    /// The same project held by another running copy of Skribisto, which only
    /// the open registry's lock files know about.
    #[test]
    fn an_import_over_a_project_open_elsewhere_is_refused() {
        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("novel.skrib");
        std::fs::write(&target, b"PK").unwrap();
        open_registry::claim(&target.to_string_lossy(), "Novel");
        let app_ctx = Rc::new(AppContext::new());
        let vm = filled(&app_ctx, dir.path());
        let mut tree = tree(&app_ctx, Vec::new());

        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        open_registry::release(&target.to_string_lossy());

        assert_eq!(
            only_dialog_title(&mut tree),
            Some(tr!(import_target_open_title()).resolve_now())
        );
        assert!(vm.job.active_operation().is_none());
    }

    /// A target that exists but is open nowhere still gets the ordinary
    /// question: the refusal is about open projects, not existing files.
    #[test]
    fn a_closed_existing_target_is_still_offered_the_overwrite() {
        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("novel.skrib"), b"PK").unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let vm = filled(&app_ctx, dir.path());

        let works = WorkRegistry::new();
        let session = WorkSession::for_test();
        session.single_work_info.file_name().set(Some(
            dir.path()
                .join("other.skrib")
                .to_string_lossy()
                .into_owned(),
        ));
        works.register(7, session);
        let mut tree = tree(
            &app_ctx,
            vec![(TypeId::of::<WorkRegistry>(), Box::new(works))],
        );

        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        assert_eq!(
            only_dialog_title(&mut tree),
            Some(tr!(project_import_overwrite_title()).resolve_now())
        );
    }

    /// A tree whose app state holds a toast registry filing into an in-memory
    /// notification archive, as the running app's does.
    fn tree_with_archive(
        app_ctx: &Rc<AppContext>,
    ) -> (WidgetTree, ToastRegistry, Rc<NotificationArchiveModel>) {
        let archive = Rc::new(NotificationArchiveModel::in_memory());
        let toasts = ToastRegistry::with_archive(
            ToastInstallOptions {
                archive: None,
                ..ToastInstallOptions::default()
            },
            archive.clone(),
        );
        let tree = tree(
            app_ctx,
            vec![(TypeId::of::<ToastRegistry>(), Box::new(toasts.clone()))],
        );
        (tree, toasts, archive)
    }

    /// Every row of `archive`, oldest first.
    fn archived(archive: &NotificationArchiveModel) -> Vec<NotificationEntry> {
        (0..archive.entries().len())
            .filter_map(|i| archive.entries().with_item(i, |e| e.clone()))
            .collect()
    }

    /// Whether `row` is an import's warnings notice.
    fn is_notice(row: &NotificationEntry) -> bool {
        row.actions
            .iter()
            .any(|a| a.intent_name.as_deref() == Some(NOTICE.action))
    }

    /// Press Import on a filled form, wait for the long operation, and deliver
    /// its completion as the app's wiring would.
    fn import_to_completion(
        tree: &mut WidgetTree,
        app_ctx: &Rc<AppContext>,
        vm: &ImportManuskriptViewModel,
    ) {
        let importing = vm.clone();
        press(tree, move |c| importing.import(c));
        let op_id = vm.job.active_operation().expect("the import started");
        let deadline = Instant::now() + Duration::from_secs(60);
        while long_operation_commands::is_operation_finished(app_ctx, &op_id) != Some(true) {
            assert!(Instant::now() < deadline, "the import never finished");
            std::thread::sleep(Duration::from_millis(20));
        }
        let completed = Event {
            origin: Origin::LongOperation(LongOperationEvent::Completed),
            ids: Vec::new(),
            data: Some(format!(r#"{{"id":"{op_id}"}}"#)),
        };
        let finishing = vm.clone();
        press(tree, move |c| {
            finishing.job.on_long_op_completed(c, &completed)
        });
    }

    /// The target is held while the conversion runs, so no window opens it and nothing
    /// else writes it, and let go of once the import is done.
    #[test]
    fn the_target_is_held_while_the_import_runs() {
        use crate::shell::open_registry;
        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let target = dir
            .path()
            .join("novel.skrib")
            .to_string_lossy()
            .into_owned();
        let app_ctx = Rc::new(AppContext::new());
        let vm = filled(&app_ctx, dir.path());
        let (mut tree, _toasts, _archive) = tree_with_archive(&app_ctx);

        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        let op_id = vm.job.active_operation().expect("the import started");
        assert!(open_registry::importing(&target), "held while it runs");
        let deadline = Instant::now() + Duration::from_secs(60);
        while long_operation_commands::is_operation_finished(&app_ctx, &op_id) != Some(true) {
            assert!(Instant::now() < deadline, "the import never finished");
            std::thread::sleep(Duration::from_millis(20));
        }
        let completed = Event {
            origin: Origin::LongOperation(LongOperationEvent::Completed),
            ids: Vec::new(),
            data: Some(format!(r#"{{"id":"{op_id}"}}"#)),
        };
        let finishing = vm.clone();
        press(&mut tree, move |c| {
            finishing.job.on_long_op_completed(c, &completed)
        });
        assert!(!open_registry::importing(&target), "let go of once done");
    }

    /// A window that started loading the target after Import was pressed claimed it
    /// before loading: the import, which looks again once it holds the target, backs
    /// out and says the project is open.
    #[test]
    fn an_import_backs_out_when_a_load_claimed_its_target_since() {
        use crate::shell::open_registry;
        use crate::test_support::drain_dialog_titles;
        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let vm = filled(&app_ctx, dir.path());
        let (mut tree, _toasts, _archive) = tree_with_archive(&app_ctx);
        let request = vm.dto(false);
        let target = request.output_path.clone();

        let Some(loading) = open_registry::claim_for_load(&target) else {
            panic!("nothing stands in the way of the load");
        };
        let starting = vm.clone();
        press(&mut tree, move |c| {
            starting.job.start(c, &MANUSKRIPT, request.clone())
        });
        assert_eq!(
            drain_dialog_titles(&mut tree),
            vec![tr!(import_target_open_title()).resolve_now()]
        );
        assert!(vm.job.active_operation().is_none(), "nothing started");
        assert!(!open_registry::importing(&target), "the hold was let go of");
        drop(loading);
    }

    /// A second import from the form, to another file, while the first still runs is
    /// refused, when Import is pressed and at an overwrite question's OK: the first
    /// keeps its target, its notice and its Cancel button until it is done.
    #[test]
    fn a_second_import_waits_until_the_first_has_finished() {
        use crate::shell::open_registry;
        use crate::test_support::drain_dialog_titles;
        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let target_in = |name: &str| {
            dir.path()
                .join(format!("{name}.skrib"))
                .to_string_lossy()
                .into_owned()
        };
        let (first, second) = (target_in("novel"), target_in("second"));
        let app_ctx = Rc::new(AppContext::new());
        let vm = filled(&app_ctx, dir.path());
        let (mut tree, _toasts, _archive) = tree_with_archive(&app_ctx);

        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        let op_id = vm.job.active_operation().expect("the first import started");
        let busy = tr!(project_import_busy_title()).resolve_now();

        vm.name().set("second".into());
        let pressing = vm.clone();
        press(&mut tree, move |c| pressing.import(c));
        assert!(
            drain_dialog_titles(&mut tree).contains(&busy),
            "Import is refused"
        );
        let request = vm.dto(true);
        assert_eq!(request.output_path, second);
        let confirming = vm.clone();
        press(&mut tree, move |c| {
            confirming.job.start(c, &MANUSKRIPT, request.clone())
        });
        assert!(
            drain_dialog_titles(&mut tree).contains(&busy),
            "an overwrite question's OK is refused too"
        );
        assert_eq!(
            vm.job.active_operation(),
            Some(op_id.clone()),
            "the first is still the one"
        );
        assert!(
            open_registry::importing(&first),
            "the first target is still held"
        );
        assert!(!open_registry::importing(&second));

        let deadline = Instant::now() + Duration::from_secs(60);
        while long_operation_commands::is_operation_finished(&app_ctx, &op_id) != Some(true) {
            assert!(Instant::now() < deadline, "the import never finished");
            std::thread::sleep(Duration::from_millis(20));
        }
        let completed = Event {
            origin: Origin::LongOperation(LongOperationEvent::Completed),
            ids: Vec::new(),
            data: Some(format!(r#"{{"id":"{op_id}"}}"#)),
        };
        let finishing = vm.clone();
        press(&mut tree, move |c| {
            finishing.job.on_long_op_completed(c, &completed)
        });
        assert!(!open_registry::importing(&first));
        assert!(
            !std::path::Path::new(&second).exists(),
            "nothing was written there"
        );
    }

    /// A writer importing one project after another, each with something to
    /// report. Every list stays in the log, but only the latest notice stays on
    /// screen: notices that wait for the writer would otherwise fill the corner
    /// until a later import's own result had no room left and was dropped
    /// unseen and unlogged.
    #[test]
    fn earlier_notices_never_crowd_out_an_import_result() {
        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let vm = filled(&app_ctx, dir.path());
        let (mut tree, toasts, archive) = tree_with_archive(&app_ctx);

        // One more import than the corner has room for.
        for round in 0..6 {
            vm.name().set(format!("novel-{round}"));
            import_to_completion(&mut tree, &app_ctx, &vm);
        }

        let rows = archived(&archive);
        assert_eq!(
            rows.iter().filter(|e| is_notice(e)).count(),
            6,
            "every import's list is in the log"
        );
        // One row per id: the log updates the row the progress notice opened
        // with each later state of the notice, so its title is what shows which
        // state was admitted last.
        let progress = tr!(import_manuskript_progress_title()).resolve_now();
        let result = rows
            .iter()
            .find(|e| e.dedup_id.as_deref() == Some(MANUSKRIPT.toast_id))
            .expect("the import's own toast is in the log");
        assert_ne!(
            result.title, progress,
            "the last import's result reached the writer, not only its progress toast"
        );
        assert_eq!(
            toasts.live_count(),
            2,
            "on screen: the last result and the last notice"
        );
    }

    /// A failure is the one thing an import must always get in front of the
    /// writer, however many other messages are already up: dropped for want of
    /// room, it would be reported nowhere, not even in the log. It arrives as a
    /// sentence in the writer's language, never the backend's chain, which stays
    /// behind Details.
    #[test]
    fn a_failure_reaches_the_writer_however_full_the_corner() {
        use frontend::common::long_operation::OperationStatus;

        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        // A folder holding nothing that makes a Manuskript project.
        let source = dir.path().join("Not a project");
        std::fs::create_dir(&source).unwrap();
        std::fs::write(source.join("notes.txt"), "Just notes.").unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let vm = form(&app_ctx);
        vm.source().set(source.to_string_lossy().into_owned());
        vm.location().set(dir.path().to_string_lossy().into_owned());
        vm.name().set("novel".into());
        let (mut tree, _toasts, archive) = tree_with_archive(&app_ctx);
        press(&mut tree, |c| {
            for i in 0..5 {
                c.show_toast(
                    Toast::info(lit!(format!("Something else {i}")))
                        .id(format!("elsewhere.{i}"))
                        .broadcast(),
                );
            }
        });

        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        let op_id = vm.job.active_operation().expect("the import started");
        let deadline = Instant::now() + Duration::from_secs(60);
        while long_operation_commands::is_operation_finished(&app_ctx, &op_id) != Some(true) {
            assert!(Instant::now() < deadline, "the import never finished");
            std::thread::sleep(Duration::from_millis(20));
        }
        let Some(OperationStatus::Failed(message)) =
            long_operation_commands::get_operation_status(&app_ctx, &op_id)
        else {
            panic!("a folder that is not a project is refused");
        };
        let failed = Event {
            origin: Origin::LongOperation(LongOperationEvent::Failed),
            ids: Vec::new(),
            data: Some(serde_json::json!({"id": op_id, "error": message}).to_string()),
        };
        let failing = vm.clone();
        press(&mut tree, move |c| {
            failing.job.on_long_op_failed(c, &failed)
        });

        let error = archived(&archive)
            .into_iter()
            .find(|e| e.dedup_id.as_deref() == Some(MANUSKRIPT.toast_id))
            .expect("the failure was admitted, so it is in the log");
        // One row per id: the log updates the row the progress notice opened
        // with the failure, so its title and body are what show it arrived.
        assert_eq!(error.title, tr!(project_import_error_title()).resolve_now());
        assert_eq!(
            error.body.as_deref(),
            Some(
                tr!(import_manuskript_not_a_project())
                    .resolve_now()
                    .as_str()
            ),
            "the reason, in the writer's words"
        );
    }

    /// The overwrite question can sit open while the writer opens that very
    /// project in a window. Answering it is the last moment before the file is
    /// replaced, so the refusal is asked again there.
    #[test]
    fn a_project_opened_while_the_overwrite_question_waits_is_refused() {
        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("novel.skrib");
        std::fs::write(&target, b"PK").unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let vm = filled(&app_ctx, dir.path());
        let works = WorkRegistry::new();
        let mut tree = tree(
            &app_ctx,
            vec![(TypeId::of::<WorkRegistry>(), Box::new(works.clone()))],
        );

        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        let request = tree
            .drain_pending_modal_requests()
            .pop()
            .expect("an existing target is asked about first")
            .request;
        assert_eq!(
            request.title,
            Some(tr!(project_import_overwrite_title()).resolve_now())
        );

        // Meanwhile, the writer opens the project in a window of this process.
        let session = WorkSession::for_test();
        session
            .single_work_info
            .file_name()
            .set(Some(target.to_string_lossy().into_owned()));
        works.register(7, session);

        let ModalContent::Deferred(builder) = request.content else {
            panic!("a MessageBox presents deferred content");
        };
        builder(&mut tree);
        tree.layout(SizeProposal::exact(900.0, 600.0));
        let ok = tree
            .find_by_label(&StandardButton::Ok.default_label().resolve_now())
            .expect("the question offers OK");
        click(&mut tree, ok);

        assert_eq!(
            only_dialog_title(&mut tree),
            Some(tr!(import_target_open_title()).resolve_now()),
            "OK is answered with the refusal"
        );
        assert!(vm.job.active_operation().is_none(), "nothing may start");
        assert_eq!(std::fs::read(&target).unwrap(), b"PK");
    }

    /// The overwrite question is modal in its own window only, and the form is
    /// one view-model every window shares: while the question waits, another
    /// window can open this importer and fill the form in afresh, even with the
    /// name of a project open in a window. OK imports the file that was asked
    /// about and checked, never the one the form names by then.
    #[test]
    fn answering_the_overwrite_question_imports_what_was_asked_about() {
        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let asked = dir.path().join("first.skrib");
        let open = dir.path().join("open.skrib");
        std::fs::write(&asked, b"PK").unwrap();
        std::fs::write(&open, b"PK").unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let works = WorkRegistry::new();
        let session = WorkSession::for_test();
        session
            .single_work_info
            .file_name()
            .set(Some(open.to_string_lossy().into_owned()));
        works.register(7, session);
        let mut tree = tree(
            &app_ctx,
            vec![(TypeId::of::<WorkRegistry>(), Box::new(works))],
        );

        let vm = filled(&app_ctx, dir.path());
        vm.name().set("first".into());
        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        let request = tree
            .drain_pending_modal_requests()
            .pop()
            .expect("an existing target is asked about first")
            .request;
        assert_eq!(
            request.title,
            Some(tr!(project_import_overwrite_title()).resolve_now())
        );

        // Meanwhile, another window opens the importer and fills it in again.
        vm.reset_form();
        vm.source().set(fixture());
        vm.location().set(dir.path().to_string_lossy().into_owned());
        vm.name().set("open".into());

        let ModalContent::Deferred(builder) = request.content else {
            panic!("a MessageBox presents deferred content");
        };
        builder(&mut tree);
        tree.layout(SizeProposal::exact(900.0, 600.0));
        let ok = tree
            .find_by_label(&StandardButton::Ok.default_label().resolve_now())
            .expect("the question offers OK");
        click(&mut tree, ok);
        assert_eq!(
            only_dialog_title(&mut tree),
            None,
            "the file asked about is open nowhere"
        );

        let op_id = vm
            .job
            .active_operation()
            .expect("the confirmed import started");
        let deadline = Instant::now() + Duration::from_secs(60);
        while long_operation_commands::is_operation_finished(&app_ctx, &op_id) != Some(true) {
            assert!(Instant::now() < deadline, "the import never finished");
            std::thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(
            std::fs::read(&open).unwrap(),
            b"PK",
            "the project open in a window is untouched"
        );
        assert_ne!(
            std::fs::read(&asked).unwrap(),
            b"PK",
            "the file asked about holds the import"
        );
    }

    /// End to end: a real import of a project Manuskript leaves ambiguous, and
    /// its completion. The warnings arrive as their own notice, archived with the
    /// whole list and a Details action the notification log can replay (the
    /// notice's own lifetime is pinned in `shared::import_warnings`).
    #[test]
    fn a_completed_import_files_its_warnings_where_they_can_be_reopened() {
        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let vm = filled(&app_ctx, dir.path());
        let (mut tree, _toasts, archive) = tree_with_archive(&app_ctx);

        import_to_completion(&mut tree, &app_ctx, &vm);

        let rows = archived(&archive);
        let notice = rows
            .iter()
            .find(|e| {
                e.actions
                    .iter()
                    .any(|a| a.intent_name.as_deref() == Some(NOTICE.action))
            })
            .expect("the warnings notice is archived with a replayable Details");
        let body = notice.body.clone().unwrap_or_default();
        assert!(body.contains("no folder.txt"), "{body}");
        assert!(body.contains("no ID of their own"), "{body}");
        assert!(
            dir.path().join("novel.skrib").exists(),
            "and the import landed"
        );
    }

    /// The report reaches a French writer in French: the notices are typed on the
    /// wire and worded when the import completes, in the interface language. It
    /// used to be English in every locale.
    #[test]
    fn a_french_writer_reads_the_report_in_french() {
        crate::test_support::with_shipped_messages("fr-FR", || {
            let _registry = IsolatedOpenRegistry::new();
            let dir = tempfile::tempdir().unwrap();
            let app_ctx = Rc::new(AppContext::new());
            let vm = filled(&app_ctx, dir.path());
            let (mut tree, _toasts, archive) = tree_with_archive(&app_ctx);

            import_to_completion(&mut tree, &app_ctx, &vm);

            let notice = archived(&archive)
                .into_iter()
                .find(|e| {
                    e.actions
                        .iter()
                        .any(|a| a.intent_name.as_deref() == Some(NOTICE.action))
                })
                .expect("the report is archived");
            let body = notice.body.unwrap_or_default();
            assert!(
                body.contains("copie"),
                "which copy was read, in French: {body}"
            );
            assert!(body.contains("folder.txt"), "{body}");
            assert!(
                !body.contains("was imported") && !body.contains("Read the"),
                "no English line: {body}"
            );
            let lines = body.lines().filter(|l| !l.trim().is_empty()).count() as i64;
            assert_eq!(
                notice.title,
                tr!(project_import_report(count = lines)).resolve_now(),
                "the title is French too"
            );
        });
    }
}
