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
//! a cancelled notice, or an error toast with **Details**.

use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use bastyde::prelude::*; // EventContext, Signal, tr!, lit!, FileDialogResult
use bastyde::widgets::{
    MessageBox, MessageBoxButtons, StandardButton, Toast, ToastAction, ValidationState,
};

use frontend::AppContext;
use frontend::commands::{import_management_commands, long_operation_commands};
use frontend::common::event::Event;
use frontend::import_management::ImportPlumeCreatorFileDto;

use super::long_op::{event_id, parse_payload, payload_id};
use crate::intents::AppIntent;

/// Update-in-place key for the single toast the import drives through its
/// lifecycle (loading → progress → success / cancelled / error).
const IMPORT_TOAST_ID: &str = "import.plume";
/// The warnings notice rides its own id so it does not replace — nor get
/// replaced by — the progress/result toast, while still being replaceable by a
/// later import's warnings instead of stacking.
const IMPORT_WARNINGS_TOAST_ID: &str = "import.plume.warnings";

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

/// Validate the source: a non-blank path to an existing, readable file.
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

/// Validate the destination folder — it must exist, be a directory, and be writable.
fn location_state(dir: &str) -> ValidationState {
    let trimmed = dir.trim();
    if trimmed.is_empty() {
        return ValidationState::Error(tr!(import_plume_location_required()));
    }
    let path = Path::new(trimmed);
    if !path.exists() {
        return ValidationState::Error(tr!(import_plume_location_missing()));
    }
    if !path.is_dir() {
        return ValidationState::Error(tr!(import_plume_location_not_folder()));
    }
    if !dir_writable(path) {
        return ValidationState::Error(tr!(import_plume_location_readonly()));
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
pub struct ImportPlumeViewModel {
    /// The chosen `.plume` / `.plume_backup` source path.
    source: Signal<String>,
    /// The destination folder for the produced `.skrib`.
    location: Signal<String>,
    /// The output base name (a `.skrib` is appended).
    name: Signal<String>,
    /// The long-operation id of the import running right now, if any — set on
    /// start, cleared when it completes / is cancelled / fails. Drives event
    /// filtering (only events for *this* op touch the toast) and the Cancel button.
    active: Signal<Option<String>>,
    app_ctx: Rc<AppContext>,
}

#[allow(dead_code)]
impl ImportPlumeViewModel {
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

    /// React to the source file picker: default the destination folder + name
    /// from the picked `.plume` (same folder, same base name → `.skrib`). Only
    /// overwrites the destination fields — the source signal is already set by
    /// the picker.
    pub fn apply_source_defaults(&self, res: &FileDialogResult) {
        if let FileDialogResult::File(Some(path)) = res {
            let source = path.to_string_lossy().into_owned();
            if let Some(parent) = path.parent().and_then(|p| p.to_str()) {
                self.location.set(parent.to_string());
            }
            self.name.set(output_stem(&source));
        }
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

    fn dto(&self, overwrite: bool) -> ImportPlumeCreatorFileDto {
        ImportPlumeCreatorFileDto {
            source_path: self.source.get(),
            output_path: build_target(&self.location.get(), &self.name.get()),
            overwrite,
            manuscript_binder_name: tr!(import_plume_manuscript_binder()).into(),
            story_bible_binder_name: tr!(import_plume_story_bible_binder()).into(),
        }
    }

    /// Inline validation for the file-name field: blank → error; a name whose
    /// target `.skrib` already exists → a *warning* (import still proceeds, after
    /// an overwrite confirmation).
    pub fn name_validation(&self) -> Signal<ValidationState> {
        self.location.zip(&self.name).map(|(loc, name)| {
            let target = build_target(loc, name);
            if target.is_empty() {
                ValidationState::Error(tr!(import_plume_name_required()))
            } else if Path::new(&target).exists() {
                ValidationState::Warning(tr!(import_plume_name_exists()))
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
            MessageBox::warning(tr!(import_plume_overwrite_title()))
                .text(tr!(import_plume_overwrite_text(name = fname)))
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

    /// Start the conversion (a long operation). Returns immediately with the
    /// operation id; the panel closes and a loading toast takes over, driven by
    /// the `Origin::LongOperation(...)` events routed to `on_long_op_*`. Only a
    /// failure to *start* is handled inline — the actual import errors arrive as
    /// a `Failed` event.
    fn run_import(&self, ctx: &mut EventContext, overwrite: bool) {
        let dto = self.dto(overwrite);
        match import_management_commands::import_plume_creator_file(&self.app_ctx, &dto) {
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
        Toast::loading(tr!(import_plume_progress_title()))
            .id(IMPORT_TOAST_ID)
            .body(lit!(body))
            .broadcast()
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
                // The mapper records what it could not carry over — prose on a
                // separator, a separator with no scene to attach to, unresolved
                // cross-links. These were collected end to end and then never
                // read by anything, so an import quietly lost data. Surface them
                // as a Details action, never as the headline.
                if !res.warnings.is_empty() {
                    let detail = res.warnings.join("\n");
                    let count = res.warnings.len() as i64;
                    // Carries its own id so a second import replaces this rather
                    // than stacking another undismissable toast on top of it.
                    ctx.show_toast(
                        Toast::warning(tr!(import_plume_warnings(count = count)))
                            .id(IMPORT_WARNINGS_TOAST_ID)
                            .broadcast()
                            .action(ToastAction::primary(
                                tr!(import_plume_details()),
                                move |c| {
                                    MessageBox::warning(tr!(import_plume_warnings_title()))
                                        .text(lit!(detail.clone()))
                                        .buttons(MessageBoxButtons::Ok)
                                        .present(c);
                                },
                            )),
                    );
                }
                ctx.show_toast(Toast::success(done).id(IMPORT_TOAST_ID).broadcast().action(
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
            }
            // Completed without a recoverable result (shouldn't happen) — clear
            // the loading toast with a neutral, self-dismissing notice.
            Ok(None) | Err(_) => {
                ctx.show_toast(
                    Toast::info(tr!(import_plume_progress_title()))
                        .id(IMPORT_TOAST_ID)
                        .auto_dismiss_after(Duration::from_secs(4))
                        .broadcast(),
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
        ctx.show_toast(
            Toast::info(tr!(import_plume_cancelled()))
                .id(IMPORT_TOAST_ID)
                .auto_dismiss_after(Duration::from_secs(4))
                .broadcast(),
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
        let details = message.to_string();
        ctx.show_toast(
            Toast::error(tr!(import_plume_error_title()))
                .id(IMPORT_TOAST_ID)
                .body(lit!(message.to_string()))
                .persistent()
                .broadcast()
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

#[cfg(test)]
mod tests {
    use super::*; // brings `FileDialogResult` in via the parent's prelude glob
    use bastyde::widgets::ValidationState;
    use std::path::PathBuf;

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

    #[test]
    fn picking_a_source_defaults_destination() {
        let vm = ImportPlumeViewModel::new(Rc::new(AppContext::new()));
        let res = FileDialogResult::File(Some(PathBuf::from("/books/My Novel.plume")));
        vm.apply_source_defaults(&res);
        assert_eq!(vm.location().get(), "/books");
        assert_eq!(vm.name().get(), "My Novel");
        assert_eq!(vm.target_path().get(), "/books/My Novel.skrib");
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
        assert_eq!(vm.dto(false).output_path, "/out/a.skrib");
        assert!(!vm.dto(false).overwrite);
        assert!(vm.dto(true).overwrite);
    }
}
