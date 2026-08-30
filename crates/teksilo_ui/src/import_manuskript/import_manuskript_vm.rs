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
//! now**), a cancelled notice, or an error toast with **Details**.

use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use teksilo::prelude::*;
use teksilo::widgets::{
    MessageBox, MessageBoxButtons, StandardButton, Toast, ToastAction, ValidationState,
};

use frontend::AppContext;
use frontend::commands::{import_management_commands, long_operation_commands};
use frontend::common::event::{Event, LongOperationEvent, Origin};
use frontend::import_management::ImportManuskriptProjectDto;

use crate::intents::AppIntent;
use crate::shared::long_op::{event_id, parse_payload, payload_id};

/// Update-in-place key for the single toast the import drives through its
/// lifecycle (loading → progress → success / cancelled / error).
const IMPORT_TOAST_ID: &str = "import.manuskript";
/// The warnings notice rides its own id so it does not replace — nor get replaced
/// by — the progress/result toast, while still being replaceable by a later
/// import's warnings instead of stacking.
const IMPORT_WARNINGS_TOAST_ID: &str = "import.manuskript.warnings";

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

/// Build the target `<dir>/<name>.skrib` (empty when the name is blank). A
/// trailing `.skrib` the user typed is not doubled.
fn build_target(dir: &str, name: &str) -> String {
    let name = name.trim();
    let name = name.strip_suffix(".skrib").unwrap_or(name).trim();
    if name.is_empty() {
        return String::new();
    }
    let dir = dir.trim().trim_end_matches(['/', '\\']);
    let sep = if dir.is_empty() { "" } else { "/" };
    format!("{dir}{sep}{name}.skrib")
}

/// Validate the source: a non-blank path to something that exists.
///
/// Deliberately accepts a directory as well as a file. The importer works out
/// which of the three shapes it is looking at, and refusing a folder here would
/// reject the one a writer with the project in git is most likely to point at.
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

/// Validate the destination folder — it must exist, be a directory, and be
/// writable.
fn location_state(dir: &str) -> ValidationState {
    let trimmed = dir.trim();
    if trimmed.is_empty() {
        return ValidationState::Error(tr!(import_manuskript_location_required()));
    }
    let path = Path::new(trimmed);
    if !path.exists() {
        return ValidationState::Error(tr!(import_manuskript_location_missing()));
    }
    if !path.is_dir() {
        return ValidationState::Error(tr!(import_manuskript_location_not_folder()));
    }
    if !dir_writable(path) {
        return ValidationState::Error(tr!(import_manuskript_location_readonly()));
    }
    ValidationState::None
}

/// Probe writability with a uniquely-named temp file (owner mode bits alone don't
/// prove the current user may write), then remove it.
fn dir_writable(dir: &Path) -> bool {
    let probe = dir.join(format!(".skribisto-writetest-{}", std::process::id()));
    match std::fs::File::create(&probe) {
        Ok(_) => {
            let _ = std::fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}

#[derive(Clone)]
pub struct ImportManuskriptViewModel {
    /// The chosen project: a `.msk` of either kind, or a project folder.
    source: Signal<String>,
    /// The destination folder for the produced `.skrib`.
    location: Signal<String>,
    /// The output base name (a `.skrib` is appended).
    name: Signal<String>,
    /// The long-operation id of the import running right now, if any — set on
    /// start, cleared when it completes / is cancelled / fails. Drives event
    /// filtering (only events for *this* op touch the toast) and Cancel.
    active: Signal<Option<String>>,
    app_ctx: Rc<AppContext>,
}

#[allow(dead_code)]
impl ImportManuskriptViewModel {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        Self {
            source: Signal::new(String::new()),
            location: Signal::new(String::new()),
            name: Signal::new(String::new()),
            active: Signal::new(None),
            app_ctx,
        }
    }

    /// Clear the form fields — called when the panel is (re)opened so a previous
    /// session's paths don't linger. The in-flight job is independent.
    pub fn reset_form(&self) {
        self.source.set(String::new());
        self.location.set(String::new());
        self.name.set(String::new());
    }

    // ── Signal accessors (bound by the view) ───────────────────────────────
    pub fn source(&self) -> Signal<String> {
        self.source.clone()
    }
    pub fn location(&self) -> Signal<String> {
        self.location.clone()
    }
    pub fn name(&self) -> Signal<String> {
        self.name.clone()
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
        if let Some(parent) = path.parent().and_then(|p| p.to_str()) {
            self.location.set(parent.to_string());
        }
        self.name.set(output_stem(&path.to_string_lossy()));
    }

    /// The reactive "Will create `…/<name>.skrib`" preview.
    pub fn target_path(&self) -> Signal<String> {
        self.location
            .zip(&self.name)
            .map(|(dir, name)| build_target(dir, name))
    }

    pub fn source_validation(&self) -> Signal<ValidationState> {
        self.source.map(|s| source_state(s))
    }
    pub fn location_validation(&self) -> Signal<ValidationState> {
        self.location.map(|d| location_state(d))
    }

    /// Whether "Import" may fire: a valid source **and** a valid destination
    /// **and** a non-blank name.
    pub fn can_import(&self) -> Signal<bool> {
        let source_ok = self
            .source
            .map(|s| matches!(source_state(s), ValidationState::None));
        let location_ok = self
            .location
            .map(|d| matches!(location_state(d), ValidationState::None));
        let name_ok = self.name.map(|n| !build_target("x", n).is_empty());
        source_ok.and(&location_ok).and(&name_ok)
    }

    fn dto(&self, overwrite: bool) -> ImportManuskriptProjectDto {
        ImportManuskriptProjectDto {
            source_path: self.source.get(),
            output_path: build_target(&self.location.get(), &self.name.get()),
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
    /// an overwrite confirmation).
    pub fn name_validation(&self) -> Signal<ValidationState> {
        self.location.zip(&self.name).map(|(loc, name)| {
            let target = build_target(loc, name);
            if target.is_empty() {
                ValidationState::Error(tr!(import_manuskript_name_required()))
            } else if Path::new(&target).exists() {
                ValidationState::Warning(tr!(import_manuskript_name_exists()))
            } else {
                ValidationState::None
            }
        })
    }

    /// "Import" — if the target `.skrib` already exists, confirm overwrite first;
    /// otherwise import straight away.
    pub fn import(&self, ctx: &mut EventContext) {
        let target = build_target(&self.location.get(), &self.name.get());
        if !target.is_empty() && Path::new(&target).exists() {
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
                    if r.button == StandardButton::Ok {
                        vm.run_import(c, true);
                    }
                })
                .present(ctx);
        } else {
            self.run_import(ctx, false);
        }
    }

    /// Start the conversion (a long operation). Returns immediately; the panel
    /// closes and a loading toast takes over. Only a failure to *start* is
    /// handled inline — the import's own errors arrive as a `Failed` event.
    fn run_import(&self, ctx: &mut EventContext, overwrite: bool) {
        let dto = self.dto(overwrite);
        match import_management_commands::import_manuskript_project(&self.app_ctx, &dto) {
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
        Toast::loading(tr!(import_manuskript_progress_title()))
            .id(IMPORT_TOAST_ID)
            .body(lit!(body))
            .broadcast()
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
                // The importer records everything it could not carry across, and
                // everything the source itself is ambiguous about. Collected and
                // then never read would be an import that quietly lost data, so
                // they get a Details action — never the headline.
                if !res.warnings.is_empty() {
                    let detail = res.warnings.join("\n");
                    let count = res.warnings.len() as i64;
                    ctx.show_toast(
                        Toast::warning(tr!(import_manuskript_warnings(count = count)))
                            .id(IMPORT_WARNINGS_TOAST_ID)
                            .broadcast()
                            .action(ToastAction::primary(
                                tr!(import_manuskript_details()),
                                move |c| {
                                    MessageBox::warning(tr!(import_manuskript_warnings_title()))
                                        .text(lit!(detail.clone()))
                                        .buttons(MessageBoxButtons::Ok)
                                        .present(c);
                                },
                            )),
                    );
                }
                ctx.show_toast(Toast::success(done).id(IMPORT_TOAST_ID).broadcast().action(
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
            }
            // Completed with no recoverable result (should not happen): clear the
            // loading toast with a neutral, self-dismissing notice rather than
            // leaving a spinner up for ever.
            Ok(None) | Err(_) => {
                ctx.show_toast(
                    Toast::info(tr!(import_manuskript_progress_title()))
                        .id(IMPORT_TOAST_ID)
                        .auto_dismiss_after(Duration::from_secs(4))
                        .broadcast(),
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
        ctx.show_toast(
            Toast::info(tr!(import_manuskript_cancelled()))
                .id(IMPORT_TOAST_ID)
                .auto_dismiss_after(Duration::from_secs(4))
                .broadcast(),
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
        let details = message.to_string();
        ctx.show_toast(
            Toast::error(tr!(import_manuskript_error_title()))
                .id(IMPORT_TOAST_ID)
                .body(lit!(message.to_string()))
                .persistent()
                .broadcast()
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
