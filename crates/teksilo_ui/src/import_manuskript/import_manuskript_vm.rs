// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ImportManuskriptViewModel` — the Import Manuskript feature's business logic.
//!
//! Single-instance live state, created once in `startup.rs` and registered as
//! app-state (so `App::build` can wire the long-operation events to it and the
//! panel can reach it). It owns the form's `Signal`s (the source project, the
//! destination folder, the output name) **and** the in-flight import job.
//!
//! **The source may be a file or a folder**, which is what makes this form
//! different from the Plume one beside it. A Manuskript project in its modern
//! default is a one-byte `.msk` plus a sibling folder of the same name; in
//! single-file mode it is a zipped `.msk`; and a writer keeping the project in
//! version control thinks of the folder itself as the project. All three are
//! accepted, so the panel offers a Browse for each and this validator takes
//! either.
//!
//! Import is a **long operation**: starting it returns
//! immediately), closes the panel, and shows a *loading* toast with a live
//! percentage and a **Cancel** button. The backend's `Origin::LongOperation(...)`
//! events — routed here by `app::wiring::long_ops` and by the Launcher — update
//! that one toast in place: progress ticks, then a success toast (with **Open
//! now**), a cancelled notice, or an error toast with **Details**. What the
//! importer could not carry across comes as a separate notice that stays until
//! the writer closes it and can be reopened from the notification log (see
//! `shared::import_warnings`).
//!
//! The destination is `shared::import_destination`, shared with the Plume form:
//! its checks run when a field changes rather than on every read, and an import
//! aimed at a project open in a window is refused.

use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use teksilo::prelude::*;
use teksilo::widgets::{
    MessageBox, MessageBoxButtons, StandardButton, Toast, ToastAction, ToastPriority,
    ValidationState,
};

use frontend::AppContext;
use frontend::commands::{import_management_commands, long_operation_commands};
use frontend::common::event::{Event, LongOperationEvent, Origin};
use frontend::import_management::ImportManuskriptProjectDto;
use skrib_format::{FoldersTooDeep, XmlTooDeep};

use crate::intents::AppIntent;
use crate::shared::form_checks::{CachedValidation, DiskChecked, FolderMessages};
use crate::shared::import_destination::{
    BusyRefusal, DestinationMessages, ImportDestination, TargetHold, refuse_if_busy, refuse_if_open,
};
use crate::shared::import_failure;
use crate::shared::import_warnings::{LiveNotice, MANUSKRIPT as WARNINGS};
use crate::shared::long_op::{event_id, parse_payload, payload_id};

/// Update-in-place key for the single toast the import drives through its
/// lifecycle (loading → progress → success / cancelled / error).
const IMPORT_TOAST_ID: &str = "import.manuskript";

/// The import's own toast, in whichever state it is in: one entry updated in
/// place under [`IMPORT_TOAST_ID`], broadcast (see
/// [`ImportManuskriptViewModel::progress_toast`]), and admitted at `High`
/// priority. At the default priority a toast reaching a corner that already
/// holds five is dropped without being logged, and this one carries the only
/// Cancel, the only Open now and the only report of a failure.
fn import_toast(toast: Toast) -> Toast {
    toast
        .id(IMPORT_TOAST_ID)
        .priority(ToastPriority::High)
        .broadcast()
}

/// The destination fields' words.
static DESTINATION: DestinationMessages = DestinationMessages {
    folder: FolderMessages {
        required: || tr!(import_manuskript_location_required()),
        missing: || tr!(import_manuskript_location_missing()),
        not_folder: || tr!(import_manuskript_location_not_folder()),
        readonly: || tr!(import_manuskript_location_readonly()),
    },
    name_required: || tr!(import_manuskript_name_required()),
    name_exists: || tr!(import_manuskript_name_exists()),
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

/// Why a second import is refused while this form's first still runs.
static BUSY: BusyRefusal = BusyRefusal {
    title: || tr!(import_manuskript_busy_title()),
    text: || tr!(import_manuskript_busy_text()),
};

#[derive(Clone)]
pub struct ImportManuskriptViewModel {
    /// The chosen project: a `.msk` of either kind, or a project folder.
    source: Signal<String>,
    /// [`source_state`] of `source`, worked out when it changes.
    source_check: CachedValidation,
    /// The destination folder and output name, with their checks.
    destination: ImportDestination,
    /// The long-operation id of the import running right now, if any — set on
    /// start, cleared when it completes / is cancelled / fails. Drives event
    /// filtering (only events for *this* op touch the toast) and Cancel.
    active: Signal<Option<String>>,
    /// The target the running import is writing, held until it completes, fails or
    /// is cancelled, so nothing opens or writes it meanwhile.
    target_hold: TargetHold,
    /// The warnings notice of the latest import that had any, while it is on
    /// screen: the next one takes its place rather than piling up beside it.
    warnings_notice: LiveNotice,
    app_ctx: Rc<AppContext>,
}

#[allow(dead_code)]
impl ImportManuskriptViewModel {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        let source = Signal::new(String::new());
        let source_check = {
            let picked = source.clone();
            CachedValidation::new(&[&source], move || source_state(&picked.get()))
        };
        Self {
            source,
            source_check,
            destination: ImportDestination::new(&DESTINATION),
            active: Signal::new(None),
            target_hold: TargetHold::default(),
            warnings_notice: LiveNotice::default(),
            app_ctx,
        }
    }

    /// Clear the form fields — called when the panel is (re)opened so a previous
    /// session's paths don't linger. The in-flight job is independent.
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
        }
    }

    /// Inline validation for the file-name field: blank → error; a name whose
    /// target `.skrib` already exists → a *warning* (import still proceeds, after
    /// an overwrite confirmation). Cached, like the folder's.
    pub fn name_validation(&self) -> Signal<ValidationState> {
        self.destination.name_validation()
    }

    /// "Import": check every field against the disk again, refuse a target that
    /// is a project open in a window, confirm an overwrite, then start.
    pub fn import(&self, ctx: &mut EventContext) {
        if refuse_if_busy(ctx, &self.active, &BUSY) {
            return;
        }
        // The verdicts on screen were worked out when the fields last changed,
        // and the disk may have moved on since. Both checks run, so every field
        // shows its fresh verdict, before either answer is acted on.
        let source_ok = self.source_check.recheck();
        let destination_ok = self.destination.recheck();
        if !(source_ok && destination_ok) {
            return;
        }
        // Everything below acts on this one request, taken from the form now.
        // The form is one view-model every window shares, and the overwrite
        // question is modal in its own window only: while it waits, another
        // window can open this importer and fill the form in afresh. OK then
        // imports the file that was asked about and checked, never the one the
        // form names by that time.
        let request = self.dto(false);
        let target = request.output_path.clone();
        if refuse_if_open(ctx, &target) {
            return;
        }
        if Path::new(&target).exists() {
            let vm = self.clone();
            let fname = Path::new(&target)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string();
            MessageBox::warning(tr!(import_manuskript_overwrite_title()))
                .text(tr!(import_manuskript_overwrite_text(name = fname)))
                .buttons(MessageBoxButtons::OkCancel)
                .on_result(move |r, c| {
                    // The confirmation can sit open while the writer opens that
                    // very project in another window, so the refusal is asked
                    // again at the last moment.
                    if r.button == StandardButton::Ok && !refuse_if_open(c, &target) {
                        vm.run_import(
                            c,
                            ImportManuskriptProjectDto {
                                overwrite: true,
                                ..request.clone()
                            },
                        );
                    }
                })
                .present(ctx);
        } else {
            self.run_import(ctx, request);
        }
    }

    /// Start the conversion of `request` (a long operation). Returns
    /// immediately; the panel closes and a loading toast takes over. Only a
    /// failure to *start* is handled inline; the import's own errors arrive as
    /// a `Failed` event.
    fn run_import(&self, ctx: &mut EventContext, request: ImportManuskriptProjectDto) {
        // Asked again here: an overwrite question can wait while another window
        // starts an import from this same form, which this one would take over.
        if refuse_if_busy(ctx, &self.active, &BUSY) {
            return;
        }
        // Held before the conversion starts, so there is no moment it runs unheld,
        // and looked at again once held, for a load another copy started meanwhile.
        if !self.target_hold.hold_unless_open(ctx, &request.output_path) {
            return;
        }
        match import_management_commands::import_manuskript_project(&self.app_ctx, &request) {
            Ok(op_id) => {
                self.active.set(Some(op_id));
                // `dismiss_top_overlay`, not `dismiss_modal`: on the overwrite
                // path this runs from the confirmation MessageBox's `on_result`,
                // whose context is anchored at the tree root, so `dismiss_modal`'s
                // walk to the enclosing modal would no-op. The import panel is the
                // topmost overlay either way.
                ctx.dismiss_top_overlay();
                ctx.show_toast(self.progress_toast(0.0, ""));
            }
            Err(e) => {
                self.target_hold.release();
                self.show_error(ctx, &format!("{e:#}"));
            }
        }
    }

    /// The loading toast the import lives in: spinner, a `NN% · message` body and
    /// a **Cancel** button, re-shown under the same id so one surface updates in
    /// place.
    ///
    /// Broadcast, for the same reason the Plume import's is: the job belongs to no
    /// open Work — it produces a `.skrib` nobody has opened yet — and this one
    /// shared view-model is wired from every window, so a window-scoped toast
    /// would put one redundant entry in each.
    fn progress_toast(&self, percent: f32, message: &str) -> Toast {
        let vm = self.clone();
        let body = if message.is_empty() {
            format!("{percent:.0}%")
        } else {
            format!("{percent:.0}% · {message}")
        };
        import_toast(Toast::loading(tr!(import_manuskript_progress_title())))
            .body(lit!(body))
            .action(
                ToastAction::destructive(tr!(import_manuskript_cancel_import()), move |c| {
                    vm.cancel(c)
                })
                .closes_toast(false),
            )
    }

    /// Cancel the running import. The backend stops at its next checkpoint and
    /// emits `Cancelled`, which becomes the final toast.
    pub fn cancel(&self, _ctx: &mut EventContext) {
        if let Some(op_id) = self.active.get() {
            long_operation_commands::cancel_operation(&self.app_ctx, &op_id);
        }
    }

    /// Subscribe a widget tree to this import's four long-operation events.
    ///
    /// **Called once per window that can start an import**, and there are two:
    /// `app::wiring::long_ops` for a project window, and `WelcomePanel::build`
    /// for the Launcher. The Launcher's is not optional — subscriptions are per
    /// `WidgetTree`, so an import started with no project window open would
    /// otherwise sit under a toast that never ticked and never offered **Open
    /// now**.
    ///
    /// Subscribing from several windows at once is safe by construction: every
    /// handler filters on the operation id this view-model started and clears it
    /// on the first terminal event, so whichever subscriber runs second finds
    /// nothing to do.
    pub fn wire_long_operation(&self, ctx: &mut BuildContext) {
        type Handler = fn(&ImportManuskriptViewModel, &mut EventContext, &Event);
        const HANDLERS: &[(LongOperationEvent, Handler)] = &[
            (LongOperationEvent::Progress, |v, c, e| {
                v.on_long_op_progress(c, e)
            }),
            (LongOperationEvent::Completed, |v, c, e| {
                v.on_long_op_completed(c, e)
            }),
            (LongOperationEvent::Cancelled, |v, c, e| {
                v.on_long_op_cancelled(c, e)
            }),
            (LongOperationEvent::Failed, |v, c, e| {
                v.on_long_op_failed(c, e)
            }),
        ];
        for (event, handler) in HANDLERS {
            let vm = self.clone();
            let handler = *handler;
            ctx.subscribe_event_with_ctx(
                Origin::LongOperation(event.clone()),
                move |e: &Event, c| handler(&vm, c, e),
            );
        }
    }

    /// A progress tick: update the loading toast in place.
    pub fn on_long_op_progress(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = self.active.get() else {
            return;
        };
        let Some(payload) = parse_payload(event) else {
            return;
        };
        if payload_id(&payload) != Some(op_id.as_str()) {
            return;
        }
        let percent = payload
            .get("percentage")
            .and_then(|p| p.as_f64())
            .unwrap_or(0.0) as f32;
        let message = payload
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("");
        ctx.show_toast(self.progress_toast(percent, message));
    }

    /// The operation finished: replace the loading toast with a success toast
    /// offering **Open now**.
    pub fn on_long_op_completed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = self.active.get() else {
            return;
        };
        if event_id(event) != Some(op_id.clone()) {
            return;
        }
        self.active.set(None);
        self.target_hold.release();
        match import_management_commands::get_import_manuskript_project_result(
            &self.app_ctx,
            &op_id,
        ) {
            Ok(Some(res)) => {
                let output = res.output_path.clone();
                let done = tr!(import_manuskript_done(
                    imported = res.imported_items,
                    revisions = res.imported_revisions
                ));
                ctx.show_toast(import_toast(Toast::success(done.clone())).action(
                    ToastAction::primary(tr!(import_manuskript_open_now()), move |c| {
                        // Opening the imported project *replaces* the one in this
                        // window, so this goes through the `work.open_path` intent
                        // → the unsaved-changes guard, rather than calling
                        // `load_work` outright and discarding unsaved edits.
                        c.send_intent(AppIntent::OpenWorkPath {
                            path: output.clone(),
                        });
                    }),
                ));
                // The importer records everything it could not carry across, and
                // everything the source itself is ambiguous about. Collected and
                // then never read would be an import that quietly lost data, so
                // they get a notice of their own that waits for the writer and can
                // be reopened from the notification log. Raised after the result,
                // which it comments on, and which its spoken name repeats: a
                // screen reader hears the notice over it.
                WARNINGS.show(ctx, &res.warnings, &done, &self.warnings_notice);
            }
            // Completed with no recoverable result (should not happen): clear the
            // loading toast with a neutral, self-dismissing notice rather than
            // leaving a spinner up for ever.
            Ok(None) | Err(_) => {
                ctx.show_toast(
                    import_toast(Toast::info(tr!(import_manuskript_progress_title())))
                        .auto_dismiss_after(Duration::from_secs(4)),
                );
            }
        }
    }

    /// Cancelled: a neutral, self-dismissing notice.
    pub fn on_long_op_cancelled(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = self.active.get() else {
            return;
        };
        if event_id(event) != Some(op_id.clone()) {
            return;
        }
        self.active.set(None);
        self.target_hold.release();
        ctx.show_toast(
            import_toast(Toast::info(tr!(import_manuskript_cancelled())))
                .auto_dismiss_after(Duration::from_secs(4)),
        );
    }

    /// Failed: an error toast whose body is the reason and whose **Details**
    /// shows the whole flattened chain.
    pub fn on_long_op_failed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = self.active.get() else {
            return;
        };
        let Some(payload) = parse_payload(event) else {
            return;
        };
        if payload_id(&payload) != Some(op_id.as_str()) {
            return;
        }
        self.active.set(None);
        self.target_hold.release();
        let error = payload
            .get("error")
            .and_then(|e| e.as_str())
            .unwrap_or_default()
            .to_string();
        self.show_error(ctx, &error);
    }

    /// Replace/raise the error toast: reason in the body, full message behind
    /// **Details**, persistent so the writer dismisses it themselves.
    fn show_error(&self, ctx: &mut EventContext, message: &str) {
        let (body, details) = failure_text(message);
        ctx.show_toast(
            import_toast(Toast::error(tr!(import_manuskript_error_title())))
                .body(body)
                .persistent()
                .action(ToastAction::primary(
                    tr!(import_manuskript_error_details()),
                    move |c| {
                        MessageBox::warning(tr!(import_manuskript_error_title()))
                            .text(lit!(details.clone()))
                            .buttons(MessageBoxButtons::Ok)
                            .present(c);
                    },
                )),
        );
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

/// What the error toast says for a failed import's `message`, and what its
/// **Details** shows: `shared::import_failure`'s answer, plus the one refusal
/// only a Manuskript project can raise, a file sitting in folders nested past the
/// ceiling (format 1 keeps the outline as folders, one per level).
fn failure_text(message: &str) -> (LocalizedString, String) {
    match FoldersTooDeep::from_failure_message(message) {
        Some(refused) => (folders_too_deep(&refused), refused.to_string()),
        None => import_failure::failure_text(message, nested_too_deep),
    }
}

/// The error toast's sentence for a project refused because one of its files
/// nests its XML past the ceiling (see `shared::import_failure`).
fn nested_too_deep(refused: &XmlTooDeep) -> LocalizedString {
    tr!(import_manuskript_nested_too_deep(
        part = refused.part.clone(),
        limit = refused.limit() as i64
    ))
}

/// The error toast's sentence for a project refused because one of its files
/// sits more folders deep than the ceiling. The path itself goes behind
/// **Details**: at that depth it runs to hundreds of folder names.
fn folders_too_deep(refused: &FoldersTooDeep) -> LocalizedString {
    tr!(import_manuskript_folders_too_deep(
        limit = refused.limit() as i64
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::{Any, TypeId};
    use std::collections::HashMap;
    use std::time::Instant;

    use teksilo::core::ModalContent;
    use teksilo::core::styles::BannerSeverity;
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
        let vm = ImportManuskriptViewModel::new(app_ctx.clone());
        vm.source().set(fixture());
        vm.location().set(dir.to_string_lossy().into_owned());
        vm.name().set("novel".into());
        vm
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

        assert!(vm.active.get().is_none(), "nothing may start");
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
        assert!(vm.active.get().is_none(), "nothing may start");
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
        assert!(vm.active.get().is_none());
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
            Some(tr!(import_manuskript_overwrite_title()).resolve_now())
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
            .any(|a| a.intent_name.as_deref() == Some(WARNINGS.action))
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
        let op_id = vm.active.get().expect("the import started");
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
        press(tree, move |c| finishing.on_long_op_completed(c, &completed));
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
        let op_id = vm.active.get().expect("the import started");
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
            finishing.on_long_op_completed(c, &completed)
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
        press(&mut tree, move |c| starting.run_import(c, request.clone()));
        assert_eq!(
            drain_dialog_titles(&mut tree),
            vec![tr!(import_target_open_title()).resolve_now()]
        );
        assert!(vm.active.get().is_none(), "nothing started");
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
        let op_id = vm.active.get().expect("the first import started");
        let busy = tr!(import_manuskript_busy_title()).resolve_now();

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
            confirming.run_import(c, request.clone())
        });
        assert!(
            drain_dialog_titles(&mut tree).contains(&busy),
            "an overwrite question's OK is refused too"
        );
        assert_eq!(
            vm.active.get(),
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
            finishing.on_long_op_completed(c, &completed)
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
        // The log merges a re-raised id into its first row, updating the title
        // and body only, so the title is what shows which state was admitted
        // last.
        let progress = tr!(import_manuskript_progress_title()).resolve_now();
        let result = rows
            .iter()
            .find(|e| e.dedup_id.as_deref() == Some(IMPORT_TOAST_ID))
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
    /// room, it would be reported nowhere, not even in the log.
    #[test]
    fn a_failure_reaches_the_writer_however_full_the_corner() {
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportManuskriptViewModel::new(app_ctx.clone());
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

        let failing = vm.clone();
        press(&mut tree, move |c| {
            failing.show_error(c, "No space left on device")
        });

        let error = archived(&archive)
            .into_iter()
            .find(|e| e.dedup_id.as_deref() == Some(IMPORT_TOAST_ID))
            .expect("the failure was admitted, so it is in the log");
        assert_eq!(error.severity, BannerSeverity::Error);
        assert_eq!(error.body.as_deref(), Some("No space left on device"));
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
            Some(tr!(import_manuskript_overwrite_title()).resolve_now())
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
        assert!(vm.active.get().is_none(), "nothing may start");
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
            Some(tr!(import_manuskript_overwrite_title()).resolve_now())
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

        let op_id = vm.active.get().expect("the confirmed import started");
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
                    .any(|a| a.intent_name.as_deref() == Some(WARNINGS.action))
            })
            .expect("the warnings notice is archived with a replayable Details");
        let body = notice.body.clone().unwrap_or_default();
        assert!(body.contains("no folder.txt"), "{body}");
        assert!(body.contains("no ID of its own"), "{body}");
        assert!(
            dir.path().join("novel.skrib").exists(),
            "and the import landed"
        );
    }

    /// The refusal reaches the writer as a sentence in their language, naming the
    /// file, in both shipped locales, with every argument filled in.
    #[test]
    fn a_nesting_refusal_is_worded_for_the_writer_in_both_locales() {
        crate::shared::import_failure::assert_worded_in_both_locales("world.opml", nested_too_deep);
    }

    /// A project refused for its folders reaches the writer as a sentence in
    /// their language naming the ceiling, in both shipped locales, with the path
    /// behind **Details** and never the wire form in the body.
    #[test]
    fn a_folder_refusal_is_worded_for_the_writer_in_both_locales() {
        let refused = FoldersTooDeep {
            part: format!(
                "outline/{}0-Scene.md",
                "0-Part/".repeat(skrib_format::MAX_XML_DEPTH)
            ),
            depth: skrib_format::MAX_XML_DEPTH + 1,
        };
        let message = refused.failure_message();
        for locale in ["en-US", "fr-FR"] {
            crate::test_support::with_shipped_messages(locale, || {
                let (body, details) = failure_text(&message);
                let body = body.resolve_now();
                assert!(
                    body.contains(&skrib_format::MAX_XML_DEPTH.to_string()),
                    "{locale}: {body}"
                );
                assert!(
                    !body.contains("{$") && !body.contains("{ $"),
                    "{locale} left an argument unfilled: {body}"
                );
                assert!(
                    !body.contains("folders-nested-too-deep") && !body.contains("0-Part/"),
                    "{locale} showed the wire form or the path: {body}"
                );
                assert_eq!(details, refused.to_string(), "{locale}");
            });
        }
    }

    /// The folder refusal is recognised ahead of the shared wording, and every
    /// other failure still goes through it unchanged.
    #[test]
    fn any_other_failure_is_worded_as_the_shared_helper_words_it() {
        let (body, details) = failure_text("reading 'x.msk': permission denied");
        assert_eq!(body.resolve_now(), "reading 'x.msk': permission denied");
        assert_eq!(details, "reading 'x.msk': permission denied");
    }
}
