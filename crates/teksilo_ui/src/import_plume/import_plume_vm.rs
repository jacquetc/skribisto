// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ImportPlumeViewModel` — the Import Plume Creator feature's business logic.
//!
//! Single-instance live state, created once in `main.rs` and registered as
//! app-state (so `App::build` can wire the long-operation events to it and the
//! panel can reach it). It owns the form's `Signal`s (source `.plume`,
//! destination folder, output name) **and** the in-flight import job. Picking a
//! source defaults the destination to the *same folder* and *same base name*
//! (with `.skrib`), still editable.
//!
//! Import is a **long operation**: `run_import` starts it (returning immediately),
//! closes the panel, and shows a *loading* toast with a live percentage and a
//! **Cancel** button. The backend's `Origin::LongOperation(...)` events — routed
//! here by `App::build` via `subscribe_event_with_ctx` — update that one toast in
//! place: progress ticks, then a success toast (with **Open now** → `load_work`),
//! a cancelled notice, or an error toast with **Details**. What the importer
//! could not carry across comes as a separate notice that stays until the writer
//! closes it and can be reopened from the notification log (see
//! `shared::import_warnings`).
//!
//! The destination is `shared::import_destination`, shared with the Manuskript
//! form: its checks run when a field changes rather than on every read, and an
//! import aimed at a project open in a window is refused.

use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use teksilo::prelude::*; // EventContext, Signal, tr!, lit!, FileDialogResult
use teksilo::widgets::{
    MessageBox, MessageBoxButtons, StandardButton, Toast, ToastAction, ToastPriority,
    ValidationState,
};

use frontend::AppContext;
use frontend::commands::{import_management_commands, long_operation_commands};
use frontend::common::event::{Event, LongOperationEvent, Origin};
use frontend::import_management::ImportPlumeCreatorFileDto;
use skrib_format::{XmlDeclaresEntities, XmlTooDeep};

use crate::intents::AppIntent;
use crate::shared::form_checks::{CachedValidation, DiskChecked, FolderMessages};
use crate::shared::import_destination::{
    BusyRefusal, DestinationMessages, ImportDestination, TargetHold, refuse_if_busy, refuse_if_open,
};
use crate::shared::import_failure;
use crate::shared::import_warnings::{LiveNotice, PLUME as WARNINGS};
use crate::shared::long_op::{event_id, parse_payload, payload_id};

/// Update-in-place key for the single toast the import drives through its
/// lifecycle (loading → progress → success / cancelled / error).
const IMPORT_TOAST_ID: &str = "import.plume";

/// The import's own toast, in whichever state it is in: one entry updated in
/// place under [`IMPORT_TOAST_ID`], broadcast (see [`ImportPlumeViewModel::progress_toast`]),
/// and admitted at `High` priority. At the default priority a toast reaching a
/// corner that already holds five is dropped without being logged, and this one
/// carries the only Cancel, the only Open now and the only report of a failure.
fn import_toast(toast: Toast) -> Toast {
    toast
        .id(IMPORT_TOAST_ID)
        .priority(ToastPriority::High)
        .broadcast()
}

/// The destination fields' words.
static DESTINATION: DestinationMessages = DestinationMessages {
    folder: FolderMessages {
        required: || tr!(import_plume_location_required()),
        missing: || tr!(import_plume_location_missing()),
        not_folder: || tr!(import_plume_location_not_folder()),
        readonly: || tr!(import_plume_location_readonly()),
    },
    name_required: || tr!(import_plume_name_required()),
    name_exists: || tr!(import_plume_name_exists()),
};

/// Strip a `.plume` / `.plume_backup` extension from a source path's file name,
/// yielding the default output base name (`"…/Le Visiteur.plume"` → `"Le Visiteur"`).
fn output_stem(source: &str) -> String {
    let file = Path::new(source)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let lower = file.to_ascii_lowercase();
    let stem = if let Some(base) = lower.strip_suffix(".plume_backup") {
        &file[..base.len()]
    } else if let Some(base) = lower.strip_suffix(".plume") {
        &file[..base.len()]
    } else {
        file
    };
    stem.to_string()
}

/// Validate the source: a non-blank path to an existing, readable file.
///
/// Touches the disk, so it runs in a [`CachedValidation`], never on a read.
fn source_state(source: &str) -> ValidationState {
    let s = source.trim();
    if s.is_empty() {
        return ValidationState::Error(tr!(import_plume_source_required()));
    }
    let path = Path::new(s);
    if !path.exists() {
        return ValidationState::Error(tr!(import_plume_source_missing()));
    }
    if !path.is_file() {
        return ValidationState::Error(tr!(import_plume_source_not_file()));
    }
    ValidationState::None
}

/// Why a second import is refused while this form's first still runs.
static BUSY: BusyRefusal = BusyRefusal {
    title: || tr!(import_plume_busy_title()),
    text: || tr!(import_plume_busy_text()),
};

#[derive(Clone)]
pub struct ImportPlumeViewModel {
    /// The chosen `.plume` / `.plume_backup` source path.
    source: Signal<String>,
    /// [`source_state`] of `source`, worked out when it changes.
    source_check: CachedValidation,
    /// The destination folder and output name, with their checks.
    destination: ImportDestination,
    /// The long-operation id of the import running right now, if any — set on
    /// start, cleared when it completes / is cancelled / fails. Drives event
    /// filtering (only events for *this* op touch the toast) and the Cancel button.
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
impl ImportPlumeViewModel {
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

    /// React to the source file picker: default the destination folder + name
    /// from the picked `.plume` (same folder, same base name → `.skrib`). Only
    /// overwrites the destination fields — the source signal is already set by
    /// the picker.
    pub fn apply_source_defaults(&self, res: &FileDialogResult) {
        if let FileDialogResult::File(Some(path)) = res {
            let source = path.to_string_lossy().into_owned();
            self.destination
                .default_from(path.parent().and_then(|p| p.to_str()), output_stem(&source));
        }
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

    fn dto(&self, overwrite: bool) -> ImportPlumeCreatorFileDto {
        ImportPlumeCreatorFileDto {
            // Plume stores a per-node status as an INDEX into its own fixed eight-rung
            // ladder and translates the names at display time, so the `.plume` carries no
            // names at all. Resolve them here, in the writer's locale, and hand them down
            // — the same treatment the two binder names above get. The Plume preset is
            // ordered to match Plume's list exactly, which is what makes the index map
            // straight across.
            status_names: crate::statuses::Preset::Plume.resolved_names(),
            source_path: self.source.get(),
            output_path: self.destination.target(),
            overwrite,
            manuscript_binder_name: tr!(import_plume_manuscript_binder()).into(),
            story_bible_binder_name: tr!(import_plume_story_bible_binder()).into(),
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
            MessageBox::warning(tr!(import_plume_overwrite_title()))
                .text(tr!(import_plume_overwrite_text(name = fname)))
                .buttons(MessageBoxButtons::OkCancel)
                .on_result(move |r, c| {
                    // The confirmation can sit open while the writer opens that
                    // very project in another window, so the refusal is asked
                    // again at the last moment.
                    if r.button == StandardButton::Ok && !refuse_if_open(c, &target) {
                        vm.run_import(
                            c,
                            ImportPlumeCreatorFileDto {
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

    /// Start the conversion of `request` (a long operation). Returns immediately
    /// with the operation id; the panel closes and a loading toast takes over,
    /// driven by the `Origin::LongOperation(...)` events routed to
    /// `on_long_op_*`. Only a failure to *start* is handled inline; the actual
    /// import errors arrive as a `Failed` event.
    fn run_import(&self, ctx: &mut EventContext, request: ImportPlumeCreatorFileDto) {
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
        match import_management_commands::import_plume_creator_file(&self.app_ctx, &request) {
            Ok(op_id) => {
                self.active.set(Some(op_id));
                // Close the import panel. `dismiss_top_overlay` (not
                // `dismiss_modal`) because on the overwrite path this runs from
                // the confirmation MessageBox's `on_result` callback, whose
                // context is anchored at the tree root (no source widget) — so
                // `dismiss_modal`'s walk to the enclosing modal would no-op. The
                // import panel is the topmost overlay in both paths.
                ctx.dismiss_top_overlay();
                // Show the progress toast right away (before the first event).
                ctx.show_toast(self.progress_toast(0.0, ""));
            }
            Err(e) => {
                self.target_hold.release();
                self.show_error(ctx, &format!("{e:#}"));
            }
        }
    }

    /// The loading toast the import lives in: spinner + a `NN% · message` body +
    /// a **Cancel** button. Re-shown (same id) on every progress tick so the one
    /// surface updates in place.
    ///
    /// Broadcast, like every toast this Tier-1, single-instance view-model
    /// raises (see the module doc): the import isn't scoped to any open
    /// Work — it produces a brand-new `.skrib` nobody has opened yet — and
    /// `ImportPlumeViewModel` is one shared instance every window's
    /// `App::build` wires the same long-operation events to, so a
    /// window-scoped default would create one redundant toast entry per
    /// open window instead of the one shared surface every window should
    /// show.
    fn progress_toast(&self, percent: f32, message: &str) -> Toast {
        let vm = self.clone();
        let body = if message.is_empty() {
            format!("{percent:.0}%")
        } else {
            format!("{percent:.0}% · {message}")
        };
        import_toast(Toast::loading(tr!(import_plume_progress_title())))
            .body(lit!(body))
            .action(
                ToastAction::destructive(tr!(import_plume_cancel_import()), move |c| vm.cancel(c))
                    .closes_toast(false),
            )
    }

    /// Cancel the running import (Cancel button). Sets the operation's cancel
    /// flag; the backend stops at the next checkpoint and the manager emits a
    /// `Cancelled` event, which `on_long_op_cancelled` turns into the final toast.
    pub fn cancel(&self, _ctx: &mut EventContext) {
        if let Some(op_id) = self.active.get() {
            long_operation_commands::cancel_operation(&self.app_ctx, &op_id);
        }
    }

    // ── Long-operation event handlers (wired in `App::build`) ───────────────
    // Each event is generic across all long operations, so every handler first
    // matches the payload's `id` against the in-flight job — events for another
    // op (or a stale one) are ignored.

    /// Subscribe a widget tree to this import's four long-operation events.
    ///
    /// **Called once per window that can start an import**, and there are two:
    /// `app::wiring::long_ops` for a project window, and `WelcomePanel::build`
    /// for the Launcher. The Launcher's is not optional — event subscriptions
    /// are per `WidgetTree`, so an import started from a Launcher with no
    /// project window open would otherwise sit under a progress toast that
    /// never ticked, never resolved, and never offered **Open now**.
    ///
    /// Subscribing from several windows at once is safe by construction, and
    /// already is: every handler below filters on the operation id this
    /// view-model started ([`Self::active`]) and clears it on the first terminal
    /// event, so whichever subscriber runs second finds nothing to do. Two
    /// project windows have always relied on exactly that.
    pub fn wire_long_operation(&self, ctx: &mut BuildContext) {
        type Handler = fn(&ImportPlumeViewModel, &mut EventContext, &Event);
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

    /// A progress tick: update the loading toast's percentage + message in place.
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

    /// The operation finished: fetch the result and replace the loading toast
    /// with a success toast offering **Open now** (loads the produced `.skrib`).
    pub fn on_long_op_completed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = self.active.get() else {
            return;
        };
        if event_id(event) != Some(op_id.clone()) {
            return;
        }
        self.active.set(None);
        self.target_hold.release();
        match import_management_commands::get_import_plume_creator_file_result(
            &self.app_ctx,
            &op_id,
        ) {
            Ok(Some(res)) => {
                let output = res.output_path.clone();
                let done = tr!(import_plume_done(
                    imported = res.imported_items,
                    skipped = res.skipped_trashed
                ));
                ctx.show_toast(import_toast(Toast::success(done.clone())).action(
                    ToastAction::primary(tr!(import_plume_open_now()), move |c| {
                        // Opening the imported project *replaces* the one in this
                        // window, so this goes through the `work.open_path` intent →
                        // the unsaved-changes guard, rather than calling `load_work`
                        // outright and silently discarding this window's unsaved
                        // edits.
                        c.send_intent(AppIntent::OpenWorkPath {
                            path: output.clone(),
                        });
                    }),
                ));
                // The mapper records what it could not carry over: prose on a
                // separator, a separator with no scene to attach to, unresolved
                // cross-links. Read by nobody, that is an import that quietly lost
                // data, so they get a notice of their own that waits for the
                // writer and can be reopened from the notification log. Raised
                // after the result, which it comments on, and which its spoken
                // name repeats: a screen reader hears the notice over it.
                WARNINGS.show(ctx, &res.warnings, &done, &self.warnings_notice);
            }
            // Completed without a recoverable result (shouldn't happen) — clear
            // the loading toast with a neutral, self-dismissing notice.
            Ok(None) | Err(_) => {
                ctx.show_toast(
                    import_toast(Toast::info(tr!(import_plume_progress_title())))
                        .auto_dismiss_after(Duration::from_secs(4)),
                );
            }
        }
    }

    /// The operation was cancelled: replace the loading toast with a neutral,
    /// self-dismissing notice.
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
            import_toast(Toast::info(tr!(import_plume_cancelled())))
                .auto_dismiss_after(Duration::from_secs(4)),
        );
    }

    /// The operation failed: replace the loading toast with an error toast whose
    /// body is the failure reason and whose **Details** button shows the full
    /// message (the operation's error string is the flattened `{:#}` chain).
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
    /// **Details** (persistent — the user dismisses it).
    fn show_error(&self, ctx: &mut EventContext, message: &str) {
        let (body, details) =
            import_failure::failure_text(message, nested_too_deep, declares_entities);
        ctx.show_toast(
            import_toast(Toast::error(tr!(import_plume_error_title())))
                .body(body)
                .persistent()
                .action(ToastAction::primary(
                    tr!(import_plume_error_details()),
                    move |c| {
                        MessageBox::warning(tr!(import_plume_error_title()))
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
impl DiskChecked for ImportPlumeViewModel {
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

/// The error toast's sentence for a project refused because one of its parts
/// nests its XML past the ceiling (see `shared::import_failure`).
fn nested_too_deep(refused: &XmlTooDeep) -> LocalizedString {
    tr!(import_plume_nested_too_deep(
        part = refused.part.clone(),
        limit = refused.limit() as i64
    ))
}

/// The error toast's sentence for a project refused because one of its parts
/// declares an XML entity, which no Plume Creator project does (see
/// `shared::import_failure`).
fn declares_entities(refused: &XmlDeclaresEntities) -> LocalizedString {
    tr!(import_plume_declares_entities(part = refused.part.clone()))
}

#[cfg(test)]
mod tests {
    use super::*; // brings `FileDialogResult` in via the parent's prelude glob
    use std::path::PathBuf;
    use teksilo::widgets::ValidationState;

    /// The refusal reaches the writer as a sentence in their language, naming the
    /// part, in both shipped locales, with every argument filled in.
    #[test]
    fn a_nesting_refusal_is_worded_for_the_writer_in_both_locales() {
        crate::shared::import_failure::assert_worded_in_both_locales("tree", nested_too_deep);
    }

    /// A project refused for declaring an entity reaches the writer as a sentence
    /// in their language, naming the part, in both shipped locales.
    #[test]
    fn an_entity_refusal_is_worded_for_the_writer_in_both_locales() {
        crate::shared::import_failure::assert_entity_refusal_worded_in_both_locales(
            "tree",
            declares_entities,
        );
    }

    #[test]
    fn output_stem_strips_plume_extensions() {
        assert_eq!(output_stem("/books/Le Visiteur.plume"), "Le Visiteur");
        assert_eq!(
            output_stem("/books/Faux-Semblants.plume_backup"),
            "Faux-Semblants"
        );
        assert_eq!(output_stem("/books/PLAIN.PLUME"), "PLAIN");
        assert_eq!(output_stem("nodir.plume"), "nodir");
    }

    #[test]
    fn picking_a_source_defaults_destination() {
        let vm = ImportPlumeViewModel::new(Rc::new(AppContext::new()));
        let res = FileDialogResult::File(Some(PathBuf::from("/books/My Novel.plume")));
        vm.apply_source_defaults(&res);
        assert_eq!(vm.location().get(), "/books");
        assert_eq!(vm.name().get(), "My Novel");
        assert_eq!(
            vm.target_path().get(),
            PathBuf::from("/books")
                .join("My Novel.skrib")
                .to_string_lossy()
        );
    }

    #[test]
    fn name_validation_flags_blank_and_existing_target() {
        let dir = tempfile::tempdir().unwrap();
        let vm = ImportPlumeViewModel::new(Rc::new(AppContext::new()));
        let v = vm.name_validation();

        vm.location().set(dir.path().to_string_lossy().into_owned());
        // Blank name → error.
        assert!(matches!(v.get(), ValidationState::Error(_)));
        // A name whose target does not exist → clean.
        vm.name().set("brand-new".into());
        assert!(matches!(v.get(), ValidationState::None));
        // Create the target, point a fresh name at it → warning (not error: the
        // import still proceeds, after an overwrite confirmation).
        std::fs::write(dir.path().join("existing.skrib"), b"x").unwrap();
        vm.name().set("existing".into());
        assert!(matches!(v.get(), ValidationState::Warning(_)));
    }

    #[test]
    fn dto_carries_overwrite_flag_and_target() {
        let vm = ImportPlumeViewModel::new(Rc::new(AppContext::new()));
        vm.source().set("/x/a.plume".into());
        vm.location().set("/out".into());
        vm.name().set("a".into());
        assert_eq!(vm.dto(false).source_path, "/x/a.plume");
        assert_eq!(
            vm.dto(false).output_path,
            PathBuf::from("/out").join("a.skrib").to_string_lossy()
        );
        assert!(!vm.dto(false).overwrite);
        assert!(vm.dto(true).overwrite);
    }

    /// A form filled in with a stand-in source file and `<dir>/novel.skrib`.
    /// The source only has to pass the form's own check (an existing file); the
    /// tests below never let an import start.
    fn filled(app_ctx: &Rc<AppContext>, dir: &std::path::Path) -> ImportPlumeViewModel {
        let source = dir.join("Le Visiteur.plume");
        std::fs::write(&source, b"PK").unwrap();
        let vm = ImportPlumeViewModel::new(app_ctx.clone());
        vm.source().set(source.to_string_lossy().into_owned());
        vm.location().set(dir.to_string_lossy().into_owned());
        vm.name().set("novel".into());
        vm
    }

    /// Reading the form, as its fields and its Import button do on every frame,
    /// never touches the disk. Before, the folder check wrote and deleted a
    /// probe file on each read.
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
        vm.source().set(vm.source().get());
        assert!(matches!(source.get(), ValidationState::Error(_)));
        assert!(!can_import.get());
    }

    /// Import checks the disk again before anything starts, field by field,
    /// and each field shows its fresh verdict. The verdicts on screen were
    /// worked out when the fields last changed, so a folder or a source that
    /// has gone since is only noticed here.
    #[test]
    fn import_rechecks_the_disk_first() {
        use crate::test_support::{IsolatedOpenRegistry, drain_dialog_titles, press};

        let _registry = IsolatedOpenRegistry::new();
        let sources = tempfile::tempdir().unwrap();
        let destination = tempfile::tempdir().unwrap();
        let source = sources.path().join("Le Visiteur.plume");
        std::fs::write(&source, b"PK").unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportPlumeViewModel::new(app_ctx.clone());
        vm.source().set(source.to_string_lossy().into_owned());
        vm.location()
            .set(destination.path().to_string_lossy().into_owned());
        vm.name().set("novel".into());
        assert!(vm.can_import().get());
        let mut tree = crate::test_support::tree_with_events(&app_ctx);

        // The destination folder goes while the dialog is open.
        std::fs::remove_dir(destination.path()).unwrap();
        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        assert!(vm.active.get().is_none(), "nothing may start");
        assert!(matches!(
            vm.location_validation().get(),
            ValidationState::Error(_)
        ));
        assert!(drain_dialog_titles(&mut tree).is_empty());

        // The folder is back, and now the source goes.
        std::fs::create_dir(destination.path()).unwrap();
        std::fs::remove_file(&source).unwrap();
        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        assert!(vm.active.get().is_none(), "nothing may start");
        assert!(matches!(
            vm.source_validation().get(),
            ValidationState::Error(_)
        ));
        assert!(
            matches!(vm.location_validation().get(), ValidationState::None),
            "the folder was looked at again too"
        );
        assert!(drain_dialog_titles(&mut tree).is_empty());
    }

    /// The project the import would replace is open in a window of this
    /// process: refused before the overwrite question, and nothing starts.
    #[test]
    fn an_import_over_a_project_open_here_is_refused() {
        use crate::sessions::{WorkRegistry, WorkSession};
        use crate::test_support::{IsolatedOpenRegistry, drain_dialog_titles, press};
        use std::any::{Any, TypeId};

        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("novel.skrib");
        std::fs::write(&target, b"PK").unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let vm = filled(&app_ctx, dir.path());

        let works = WorkRegistry::new();
        let session = WorkSession::for_test();
        // Spelled differently from the form's target on purpose: the check
        // compares canonical paths, not strings.
        session
            .single_work_info
            .file_name()
            .set(Some(format!("{}/./novel.skrib", dir.path().display())));
        works.register(3, session);
        let state: std::collections::HashMap<TypeId, Box<dyn Any>> = [(
            TypeId::of::<WorkRegistry>(),
            Box::new(works) as Box<dyn Any>,
        )]
        .into();
        let mut tree = crate::test_support::tree_with_app_state(&app_ctx, state);

        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));

        assert_eq!(
            drain_dialog_titles(&mut tree),
            vec![tr!(import_target_open_title()).resolve_now()],
            "the refusal, not the overwrite confirmation"
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
        use crate::sessions::{WorkRegistry, WorkSession};
        use crate::test_support::{IsolatedOpenRegistry, click, drain_dialog_titles, press};
        use std::any::{Any, TypeId};
        use std::time::Instant;
        use teksilo::core::ModalContent;

        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let source = lonely_plume(dir.path());
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
        works.register(3, session);
        let state: std::collections::HashMap<TypeId, Box<dyn Any>> = [(
            TypeId::of::<WorkRegistry>(),
            Box::new(works) as Box<dyn Any>,
        )]
        .into();
        let mut tree = crate::test_support::tree_with_app_state(&app_ctx, state);

        let vm = ImportPlumeViewModel::new(app_ctx.clone());
        let fill = |name: &str| {
            vm.source().set(source.to_string_lossy().into_owned());
            vm.location().set(dir.path().to_string_lossy().into_owned());
            vm.name().set(name.into());
        };
        fill("first");
        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        let request = tree
            .drain_pending_modal_requests()
            .pop()
            .expect("an existing target is asked about first")
            .request;
        assert_eq!(
            request.title,
            Some(tr!(import_plume_overwrite_title()).resolve_now())
        );

        // Meanwhile, another window opens the importer and fills it in again.
        vm.reset_form();
        fill("open");

        let ModalContent::Deferred(builder) = request.content else {
            panic!("a MessageBox presents deferred content");
        };
        builder(&mut tree);
        tree.layout(SizeProposal::exact(900.0, 600.0));
        let ok = tree
            .find_by_label(&StandardButton::Ok.default_label().resolve_now())
            .expect("the question offers OK");
        click(&mut tree, ok);
        assert!(
            drain_dialog_titles(&mut tree).is_empty(),
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

    /// A Plume project with a chapter that holds nothing but a separator, which
    /// the importer has to drop and says so: an import that always has one
    /// warning to report.
    fn lonely_plume(dir: &std::path::Path) -> PathBuf {
        use std::io::Write;
        let source = dir.join("Lonely.plume");
        let mut zip = zip::ZipWriter::new(std::fs::File::create(&source).unwrap());
        zip.start_file("tree", zip::write::SimpleFileOptions::default())
            .unwrap();
        zip.write_all(
            br#"<!DOCTYPE plume-tree><plume-tree version="0.5" projectName="Lonely">
                <book number="1" name="Book">
                  <chapter number="2" name="Only a break">
                    <separator number="10001" name="* * *"/>
                  </chapter>
                </book>
              </plume-tree>"#,
        )
        .unwrap();
        zip.finish().unwrap();
        source
    }

    /// A tree whose app state holds a toast registry filing into an in-memory
    /// notification archive, as the running app's does.
    fn tree_with_archive(
        app_ctx: &Rc<AppContext>,
    ) -> (
        teksilo::core::widget_tree::WidgetTree,
        teksilo::widgets::ToastRegistry,
        Rc<teksilo::widgets::NotificationArchiveModel>,
    ) {
        use std::any::{Any, TypeId};
        use teksilo::widgets::{NotificationArchiveModel, ToastInstallOptions, ToastRegistry};
        let archive = Rc::new(NotificationArchiveModel::in_memory());
        let toasts = ToastRegistry::with_archive(
            ToastInstallOptions {
                archive: None,
                ..ToastInstallOptions::default()
            },
            archive.clone(),
        );
        let state: std::collections::HashMap<TypeId, Box<dyn Any>> = [(
            TypeId::of::<ToastRegistry>(),
            Box::new(toasts.clone()) as Box<dyn Any>,
        )]
        .into();
        let tree = crate::test_support::tree_with_app_state(app_ctx, state);
        (tree, toasts, archive)
    }

    /// Every row of `archive`, oldest first.
    fn archived(
        archive: &teksilo::widgets::NotificationArchiveModel,
    ) -> Vec<teksilo::widgets::NotificationEntry> {
        (0..archive.entries().len())
            .filter_map(|i| archive.entries().with_item(i, |e| e.clone()))
            .collect()
    }

    /// Press Import on a filled form, wait for the long operation, and deliver
    /// its completion as the app's wiring would.
    fn import_to_completion(
        tree: &mut teksilo::core::widget_tree::WidgetTree,
        app_ctx: &Rc<AppContext>,
        vm: &ImportPlumeViewModel,
    ) {
        use crate::test_support::press;
        use frontend::common::event::{Event, LongOperationEvent, Origin};
        use std::time::Instant;

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

    /// For as long as the conversion runs, its target is held: no window opens it and
    /// nothing else writes it, in this Skribisto or another one, since the import
    /// replaces it when it finishes. A second import to the same file is refused while
    /// the first runs. The hold goes once the import is done, and the project it wrote
    /// then opens as any other.
    #[test]
    fn the_target_is_held_while_the_import_runs() {
        use crate::shell::open_registry;
        use crate::test_support::{IsolatedOpenRegistry, drain_dialog_titles, press};
        use frontend::common::event::{Event, LongOperationEvent, Origin};
        use std::time::Instant;

        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let source = lonely_plume(dir.path());
        let target = dir
            .path()
            .join("novel.skrib")
            .to_string_lossy()
            .into_owned();
        let app_ctx = Rc::new(AppContext::new());
        let filled_for = |vm: &ImportPlumeViewModel| {
            vm.source().set(source.to_string_lossy().into_owned());
            vm.location().set(dir.path().to_string_lossy().into_owned());
            vm.name().set("novel".into());
        };
        let vm = ImportPlumeViewModel::new(app_ctx.clone());
        filled_for(&vm);
        let (mut tree, _toasts, _archive) = tree_with_archive(&app_ctx);
        assert!(!open_registry::importing(&target));

        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        let op_id = vm.active.get().expect("the import started");
        assert!(open_registry::importing(&target), "held while it runs");

        let other = ImportPlumeViewModel::new(app_ctx.clone());
        filled_for(&other);
        let second = other.clone();
        press(&mut tree, move |c| second.import(c));
        assert!(
            drain_dialog_titles(&mut tree).contains(&tr!(target_importing_title()).resolve_now()),
            "a second import to the same file is refused"
        );
        assert!(other.active.get().is_none());

        let deadline = Instant::now() + Duration::from_secs(60);
        while long_operation_commands::is_operation_finished(&app_ctx, &op_id) != Some(true) {
            assert!(Instant::now() < deadline, "the import never finished");
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(
            open_registry::importing(&target),
            "held until the writer is told it is done"
        );
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

    /// A second import from the form, to another file, while the first still runs is
    /// refused, when Import is pressed and at the last moment (an overwrite question's
    /// OK): it would take the first one's progress notice and Cancel button over, and
    /// let go of the first one's target while that import still meant to replace it.
    /// The first keeps its target until it is done.
    #[test]
    fn a_second_import_waits_until_the_first_has_finished() {
        use crate::shell::open_registry;
        use crate::test_support::{IsolatedOpenRegistry, drain_dialog_titles, press};
        use frontend::common::event::{Event, LongOperationEvent, Origin};
        use std::time::Instant;

        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let source = lonely_plume(dir.path());
        let target_in = |name: &str| {
            dir.path()
                .join(format!("{name}.skrib"))
                .to_string_lossy()
                .into_owned()
        };
        let (first, second) = (target_in("novel"), target_in("second"));
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportPlumeViewModel::new(app_ctx.clone());
        vm.source().set(source.to_string_lossy().into_owned());
        vm.location().set(dir.path().to_string_lossy().into_owned());
        vm.name().set("novel".into());
        let (mut tree, _toasts, _archive) = tree_with_archive(&app_ctx);

        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        let op_id = vm.active.get().expect("the first import started");
        let busy = tr!(import_plume_busy_title()).resolve_now();

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

    /// A window that started loading the target after Import was pressed claimed it
    /// before loading (`open_registry::claim_for_load`). The import, which looks again
    /// once it holds the target, backs out and says the project is open, rather than
    /// replacing the file under that window. Before, nothing looked after the press.
    #[test]
    fn an_import_backs_out_when_a_load_claimed_its_target_since() {
        use crate::shell::open_registry;
        use crate::test_support::{IsolatedOpenRegistry, drain_dialog_titles, press};

        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let source = lonely_plume(dir.path());
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportPlumeViewModel::new(app_ctx.clone());
        vm.source().set(source.to_string_lossy().into_owned());
        vm.location().set(dir.path().to_string_lossy().into_owned());
        vm.name().set("novel".into());
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

    /// A cancelled import lets go of its target as well.
    #[test]
    fn a_cancelled_import_lets_go_of_its_target() {
        use crate::shell::open_registry;
        use crate::test_support::{IsolatedOpenRegistry, press};
        use frontend::common::event::{Event, LongOperationEvent, Origin};

        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let source = lonely_plume(dir.path());
        let target = dir
            .path()
            .join("novel.skrib")
            .to_string_lossy()
            .into_owned();
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportPlumeViewModel::new(app_ctx.clone());
        vm.source().set(source.to_string_lossy().into_owned());
        vm.location().set(dir.path().to_string_lossy().into_owned());
        vm.name().set("novel".into());
        let (mut tree, _toasts, _archive) = tree_with_archive(&app_ctx);

        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        let op_id = vm.active.get().expect("the import started");
        assert!(open_registry::importing(&target));
        let cancelled = Event {
            origin: Origin::LongOperation(LongOperationEvent::Cancelled),
            ids: Vec::new(),
            data: Some(format!(r#"{{"id":"{op_id}"}}"#)),
        };
        let cancelling = vm.clone();
        press(&mut tree, move |c| {
            cancelling.on_long_op_cancelled(c, &cancelled)
        });
        assert!(!open_registry::importing(&target));
    }

    /// End to end: a real import of a Plume project the importer has to report
    /// on. The warning arrives as its own notice, archived with the whole list
    /// and a Details action the notification log can replay (the notice's own
    /// lifetime is pinned in `shared::import_warnings`).
    #[test]
    fn a_completed_import_files_its_warnings_where_they_can_be_reopened() {
        use crate::test_support::IsolatedOpenRegistry;

        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let source = lonely_plume(dir.path());
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportPlumeViewModel::new(app_ctx.clone());
        vm.source().set(source.to_string_lossy().into_owned());
        vm.location().set(dir.path().to_string_lossy().into_owned());
        vm.name().set("novel".into());
        let (mut tree, _toasts, archive) = tree_with_archive(&app_ctx);

        import_to_completion(&mut tree, &app_ctx, &vm);

        let notice = archived(&archive)
            .into_iter()
            .find(|e| {
                e.actions
                    .iter()
                    .any(|a| a.intent_name.as_deref() == Some(WARNINGS.action))
            })
            .expect("the warnings notice is archived with a replayable Details");
        let body = notice.body.unwrap_or_default();
        assert!(body.contains("has no scenes"), "{body}");
        assert!(
            dir.path().join("novel.skrib").exists(),
            "and the import landed"
        );
    }

    /// A writer importing one project after another, each with something to
    /// report. Every list stays in the log, but only the latest notice stays on
    /// screen: notices that wait for the writer would otherwise fill the corner
    /// until a later import's own result had no room left and was dropped
    /// unseen and unlogged.
    #[test]
    fn earlier_notices_never_crowd_out_an_import_result() {
        use crate::test_support::IsolatedOpenRegistry;

        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let source = lonely_plume(dir.path());
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportPlumeViewModel::new(app_ctx.clone());
        vm.source().set(source.to_string_lossy().into_owned());
        vm.location().set(dir.path().to_string_lossy().into_owned());
        let (mut tree, toasts, archive) = tree_with_archive(&app_ctx);

        // One more import than the corner has room for.
        for round in 0..6 {
            vm.name().set(format!("novel-{round}"));
            import_to_completion(&mut tree, &app_ctx, &vm);
        }

        let rows = archived(&archive);
        let notices = rows
            .iter()
            .filter(|e| {
                e.actions
                    .iter()
                    .any(|a| a.intent_name.as_deref() == Some(WARNINGS.action))
            })
            .count();
        assert_eq!(notices, 6, "every import's list is in the log");
        // The log merges a re-raised id into its first row, updating the title
        // and body only, so the title is what shows which state was admitted
        // last.
        let progress = tr!(import_plume_progress_title()).resolve_now();
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
        use crate::test_support::press;
        use teksilo::core::styles::BannerSeverity;

        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportPlumeViewModel::new(app_ctx.clone());
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
    /// project in another window. Answering it is the last moment before the
    /// file is replaced, so the refusal is asked again there.
    #[test]
    fn a_project_opened_while_the_overwrite_question_waits_is_refused() {
        use crate::shell::open_registry;
        use crate::test_support::{IsolatedOpenRegistry, click, drain_dialog_titles, press};
        use teksilo::core::ModalContent;
        use teksilo::widgets::StandardButton;

        let _registry = IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("novel.skrib");
        std::fs::write(&target, b"PK").unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let vm = filled(&app_ctx, dir.path());
        let mut tree = crate::test_support::tree_with_events(&app_ctx);

        let importing = vm.clone();
        press(&mut tree, move |c| importing.import(c));
        let request = tree
            .drain_pending_modal_requests()
            .pop()
            .expect("an existing target is asked about first")
            .request;
        assert_eq!(
            request.title,
            Some(tr!(import_plume_overwrite_title()).resolve_now())
        );

        // Meanwhile, another window opens the project.
        open_registry::claim(&target.to_string_lossy(), "Novel");

        let ModalContent::Deferred(builder) = request.content else {
            panic!("a MessageBox presents deferred content");
        };
        builder(&mut tree);
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 600.0));
        let ok = tree
            .find_by_label(&StandardButton::Ok.default_label().resolve_now())
            .expect("the question offers OK");
        click(&mut tree, ok);
        open_registry::release(&target.to_string_lossy());

        assert_eq!(
            drain_dialog_titles(&mut tree),
            vec![tr!(import_target_open_title()).resolve_now()],
            "OK is answered with the refusal"
        );
        assert!(vm.active.get().is_none(), "nothing may start");
        assert_eq!(std::fs::read(&target).unwrap(), b"PK");
    }
}
