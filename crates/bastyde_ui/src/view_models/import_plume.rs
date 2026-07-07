//! `ImportPlumeViewModel` — the Import Plume Creator dialog's business logic.
//!
//! Single-instance live state (like `NewWorkViewModel`): it owns the form's
//! `Signal`s (source `.plume`, destination folder, output name), so it is created
//! once in `ImportPlumePanel::new` and shared by `.clone()`. Picking a source
//! defaults the destination to the *same folder* and *same base name* (with
//! `.skrib`), still editable. On import it calls the backend command, then offers
//! to open the produced `.skrib` via the existing `load_work`.

use std::path::Path;
use std::rc::Rc;

use bastyde::prelude::*; // EventContext, Signal, tr!, lit!, FileDialogResult
use bastyde::widgets::{
    MessageBox, MessageBoxButtons, StandardButton, Toast, ToastAction, ValidationState,
};

use frontend::AppContext;
use frontend::commands::{import_management_commands, work_management_commands};
use frontend::import_management::ImportPlumeCreatorFileDto;
use frontend::work_management::LoadWorkDto;

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
    app_ctx: Rc<AppContext>,
}

#[allow(dead_code)]
impl ImportPlumeViewModel {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        Self {
            source: Signal::new(String::new()),
            location: Signal::new(String::new()),
            name: Signal::new(String::new()),
            app_ctx,
        }
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

    /// The reactive "Will create …/<name>.skrib" preview.
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
        let source_ok = self.source.map(|s| matches!(source_state(s), ValidationState::None));
        let location_ok = self.location.map(|d| matches!(location_state(d), ValidationState::None));
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

    /// Run the conversion. On success dismiss + offer to open; on failure surface
    /// the real cause in a persistent toast with a **Details** button (the raw
    /// `to_string()` of an `anyhow` error only shows the outermost context, so we
    /// use the root cause for the body and the full `{:#}` chain for the dialog).
    fn run_import(&self, ctx: &mut EventContext, overwrite: bool) {
        match import_management_commands::import_plume_creator_file(&self.app_ctx, &self.dto(overwrite))
        {
            Ok(res) => {
                // Close the import panel. `dismiss_top_overlay` (not
                // `dismiss_modal`) because on the overwrite path this runs from
                // the confirmation MessageBox's `on_result` callback, whose
                // context is anchored at the tree root (no source widget) — so
                // `dismiss_modal`'s walk to the enclosing modal would no-op. The
                // import panel is the topmost overlay in both the direct and the
                // post-confirmation paths.
                ctx.dismiss_top_overlay();
                let app_ctx = self.app_ctx.clone();
                let output = res.output_path.clone();
                let done = tr!(import_plume_done(
                    imported = res.imported_items,
                    skipped = res.skipped_trashed
                ));
                ctx.show_toast(Toast::success(done).action(ToastAction::primary(
                    tr!(import_plume_open_now()),
                    move |c| {
                        if let Err(e) = work_management_commands::load_work(
                            &app_ctx,
                            &LoadWorkDto {
                                file_name: output.clone(),
                            },
                        ) {
                            c.show_toast(Toast::error(tr!(could_not_open_work(
                                error = e.to_string()
                            ))));
                        }
                    },
                )));
            }
            Err(e) => {
                let reason = e.root_cause().to_string();
                let details = format!("{e:#}");
                ctx.show_toast(
                    Toast::error(tr!(import_plume_error_title()))
                        .body(lit!(reason))
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::prelude::*; // FileDialogResult
    use bastyde::widgets::ValidationState;
    use std::path::PathBuf;

    #[test]
    fn output_stem_strips_plume_extensions() {
        assert_eq!(output_stem("/books/Le Visiteur.plume"), "Le Visiteur");
        assert_eq!(output_stem("/books/Faux-Semblants.plume_backup"), "Faux-Semblants");
        assert_eq!(output_stem("/books/PLAIN.PLUME"), "PLAIN");
        assert_eq!(output_stem("nodir.plume"), "nodir");
    }

    #[test]
    fn build_target_appends_skrib_once() {
        assert_eq!(build_target("/books", "Le Visiteur"), "/books/Le Visiteur.skrib");
        assert_eq!(build_target("/books/", "Le Visiteur"), "/books/Le Visiteur.skrib");
        assert_eq!(build_target("/books", "Le Visiteur.skrib"), "/books/Le Visiteur.skrib");
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
