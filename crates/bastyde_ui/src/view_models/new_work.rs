//! `NewWorkViewModel` — the New Work dialog's business logic.
//!
//! Single-instance live state (like `EditorsViewModel`): it owns the form's
//! `Signal`s, which must survive panel rebuilds within one modal session, so it
//! is created once in `NewWorkPanel::new` and shared by `.clone()`. The dialog's
//! actions (derive the target path, assemble the `NewWorkDto`, create the work)
//! live here, not in the view's `build()`.

use std::path::Path;
use std::rc::Rc;

use bastyde::prelude::*; // EventContext, Signal, tr!
use bastyde::widgets::{Toast, ValidationState};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::work_management::{NewWorkDto, NewWorkTemplate};

/// Build the `NewWorkDto` for the New Work dialog.
///
/// The backend can't do i18n, so the template's human labels are resolved *here*
/// and passed in, in the exact order `work_management`'s `TemplateLabels::from_list`
/// reads them: `[Manuscript, Notes, Research, Notebook, Chapter, Scene, Note]`.
/// `language` is a locale tag (e.g. `"en-US"`) that becomes the new work's
/// `dict_language`.
pub(crate) fn new_work_dto(
    file_name: String,
    is_folder: bool,
    template_kind: NewWorkTemplate,
    language: String,
) -> NewWorkDto {
    NewWorkDto {
        file_name,
        is_folder,
        template_kind,
        labels: vec![
            tr!(new_work_manuscript()).into(),
            tr!(new_work_notes()).into(),
            tr!(new_work_research()).into(),
            tr!(new_work_notebook()).into(),
            tr!(new_work_chapter()).into(),
            tr!(new_work_scene()).into(),
            tr!(new_work_note()).into(),
        ],
        language,
    }
}

/// Map the Template `SegmentedControl` index to its `NewWorkTemplate`.
///
/// Kept in one place so the view (segment order) and the DTO stay in lockstep.
pub(crate) fn template_from_index(index: usize) -> NewWorkTemplate {
    match index {
        0 => NewWorkTemplate::None,
        1 => NewWorkTemplate::EmptyNovel,
        2 => NewWorkTemplate::LightNovel,
        4 => NewWorkTemplate::NoteBook,
        // 3 (Novel) is the default selection; any out-of-range index falls back
        // to it too.
        _ => NewWorkTemplate::Novel,
    }
}

/// The Template segment index for the default (Novel) selection.
pub(crate) const DEFAULT_TEMPLATE_INDEX: usize = 3;

/// Derive the on-disk target from the folder + name + format.
///
/// Single file → `<dir>/<slug>.skrib`; Bundle → `<dir>/<slug>` (a folder). The
/// slug is the trimmed, lowercased name with inner whitespace collapsed to `-`.
/// An empty name yields `""` (nothing to create yet).
fn build_target_path(dir: &str, name: &str, format_idx: usize) -> String {
    let slug = slugify(name);
    if slug.is_empty() {
        return String::new();
    }
    let dir = dir.trim_end_matches(['/', '\\']);
    let sep = if dir.is_empty() { "" } else { "/" };
    if format_idx == 1 {
        // Bundle: a folder named after the work.
        format!("{dir}{sep}{slug}")
    } else {
        format!("{dir}{sep}{slug}.skrib")
    }
}

/// Filesystem-forbidden characters (POSIX separators + the Windows set); each
/// is replaced by a `-` so the name is always a safe single path component.
const FORBIDDEN_CHARS: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

/// Turn a work name into a safe filename stem: lowercase, whitespace / control /
/// forbidden characters collapsed to a single `-`, with leading/trailing `-` and
/// `.` trimmed (a trailing dot is illegal on Windows). `"Tidewrack"` →
/// `"tidewrack"`, `"The Long Road"` → `"the-long-road"`, `"Book: A/B?"` →
/// `"book-a-b"`. Returns `""` when nothing usable remains.
fn slugify(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut prev_dash = false;
    for ch in name.trim().to_lowercase().chars() {
        if ch.is_control() || ch.is_whitespace() || FORBIDDEN_CHARS.contains(&ch) {
            if !out.is_empty() && !prev_dash {
                out.push('-');
                prev_dash = true;
            }
        } else {
            out.push(ch);
            prev_dash = false;
        }
    }
    out.trim_matches(|c: char| c == '-' || c == '.').to_string()
}

/// Validate the chosen Location folder — it must exist, be a directory, and be
/// writable (we actually create a new work there). Returns the field's inline
/// [`ValidationState`]; `None` means valid.
fn location_state(dir: &str) -> ValidationState {
    let trimmed = dir.trim();
    if trimmed.is_empty() {
        return ValidationState::Error(tr!(new_work_location_required()));
    }
    let path = Path::new(trimmed);
    if !path.exists() {
        return ValidationState::Error(tr!(new_work_location_missing()));
    }
    if !path.is_dir() {
        return ValidationState::Error(tr!(new_work_location_not_folder()));
    }
    if !dir_writable(path) {
        return ValidationState::Error(tr!(new_work_location_readonly()));
    }
    ValidationState::None
}

/// True when `location_state` reports no error (used to gate "Create Work").
fn location_ok(dir: &str) -> bool {
    matches!(location_state(dir), ValidationState::None)
}

/// Is `dir` writable *by us*? Probe with a uniquely-named temp file (owner
/// mode-bits alone don't say whether the current user may write), then remove
/// it. Only ever called for a path already known to be an existing directory,
/// so it does not run on every keystroke of a half-typed path.
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

/// The user's home directory (`$HOME` / `%USERPROFILE%`), or `""` — a starting
/// point for the Location field; the picker lets them choose any folder.
fn default_location() -> String {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_default()
}

/// The active UI locale tag (e.g. `"en-US"`), used to seed the default language.
fn current_locale_tag() -> Option<String> {
    bastyde::i18n::current_locale().map(|sig| sig.get().to_string())
}

#[derive(Clone)]
pub struct NewWorkViewModel {
    /// The work's display name (drives the filename slug).
    name: Signal<String>,
    /// Format segment: `0` = single `.skrib` file, `1` = bundle folder.
    format_idx: Signal<usize>,
    /// The containing folder chosen via the file picker.
    location: Signal<String>,
    /// Selected default-language locale tag (`Some("en-US")`), or `None`.
    language: Signal<Option<String>>,
    /// Template segment index (`0..=4`).
    template_idx: Signal<usize>,
    app_ctx: Rc<AppContext>,
}

#[allow(dead_code)]
impl NewWorkViewModel {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        Self {
            name: Signal::new(String::new()),
            format_idx: Signal::new(0),
            location: Signal::new(default_location()),
            language: Signal::new(current_locale_tag()),
            template_idx: Signal::new(DEFAULT_TEMPLATE_INDEX),
            app_ctx,
        }
    }

    // ── Signal accessors (bound by the view) ───────────────────────────────
    pub fn name(&self) -> Signal<String> {
        self.name.clone()
    }
    pub fn format_idx(&self) -> Signal<usize> {
        self.format_idx.clone()
    }
    pub fn location(&self) -> Signal<String> {
        self.location.clone()
    }
    pub fn language(&self) -> Signal<Option<String>> {
        self.language.clone()
    }
    pub fn template_idx(&self) -> Signal<usize> {
        self.template_idx.clone()
    }

    /// The reactive "Will create …" path — recomputes as name/location/format
    /// change. This is the dialog's one runtime-computed (`lit!`) string.
    pub fn target_path(&self) -> Signal<String> {
        self.location
            .zip3(&self.name, &self.format_idx)
            .map(|(dir, name, fmt)| build_target_path(dir, name, *fmt))
    }

    /// Inline validation for the Work name field: blank → "enter a name"; a name
    /// made only of forbidden/whitespace characters (which would slugify to an
    /// empty, fileless stem) → "no usable characters".
    pub fn name_validation(&self) -> Signal<ValidationState> {
        self.name.map(|n| {
            if n.trim().is_empty() {
                ValidationState::Error(tr!(new_work_name_required()))
            } else if slugify(n).is_empty() {
                ValidationState::Error(tr!(new_work_name_invalid()))
            } else {
                ValidationState::None
            }
        })
    }

    /// Inline validation for the Location field — the folder must exist, be a
    /// directory, and be writable. Recomputes only when the location changes.
    pub fn location_validation(&self) -> Signal<ValidationState> {
        self.location.map(|dir| location_state(dir))
    }

    /// Whether "Create Work" may fire: a non-blank name **and** a valid
    /// location. Split into two per-field booleans so typing the name doesn't
    /// re-probe the filesystem (the location check only reruns on a location
    /// change).
    pub fn can_create(&self) -> Signal<bool> {
        let name_ok = self.name.map(|n| !slugify(n).is_empty());
        let location_ok = self.location.map(|dir| location_ok(dir));
        name_ok.and(&location_ok)
    }

    /// Build the DTO from the current form state.
    fn dto(&self) -> NewWorkDto {
        new_work_dto(
            build_target_path(&self.location.get(), &self.name.get(), self.format_idx.get()),
            self.format_idx.get() == 1,
            template_from_index(self.template_idx.get()),
            self.language.get().unwrap_or_default(),
        )
    }

    /// "Create Work" — create the work from the current form, then dismiss. On
    /// failure the toast surfaces the error and the dialog stays open to retry.
    pub fn create(&self, ctx: &mut EventContext) {
        match work_management_commands::new_work(&self.app_ctx, &self.dto()) {
            Ok(()) => ctx.dismiss_modal(),
            Err(e) => {
                ctx.show_toast(Toast::error(tr!(could_not_create_work(error = e.to_string()))));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_sanitizes_forbidden_chars() {
        // Path separators and the Windows-reserved set collapse to single dashes.
        assert_eq!(slugify("Book: A/B?"), "book-a-b");
        assert_eq!(slugify(r#"a\b:c*d"e<f>g|h"#), "a-b-c-d-e-f-g-h");
        // Leading/trailing separators and dots are trimmed (no dotfiles / no
        // trailing-dot names).
        assert_eq!(slugify("  ...Weird**Name??  "), "weird-name");
        assert_eq!(slugify(".hidden."), "hidden");
        // A name made only of forbidden/space chars slugifies to nothing.
        assert_eq!(slugify("///"), "");
        assert_eq!(slugify("   "), "");
    }

    #[test]
    fn slug_and_target_path() {
        assert_eq!(slugify("Tidewrack"), "tidewrack");
        assert_eq!(slugify("  The Long Road  "), "the-long-road");
        assert_eq!(slugify(""), "");

        // Single file appends `.skrib`; bundle is a bare folder.
        assert_eq!(
            build_target_path("~/Novels", "Tidewrack", 0),
            "~/Novels/tidewrack.skrib"
        );
        assert_eq!(build_target_path("~/Novels", "Tidewrack", 1), "~/Novels/tidewrack");
        // A trailing separator on the folder is not doubled.
        assert_eq!(
            build_target_path("~/Novels/", "Tidewrack", 0),
            "~/Novels/tidewrack.skrib"
        );
        // Empty name → nothing to create yet.
        assert_eq!(build_target_path("~/Novels", "   ", 0), "");
    }

    #[test]
    fn template_index_maps_to_all_variants() {
        assert_eq!(template_from_index(0), NewWorkTemplate::None);
        assert_eq!(template_from_index(1), NewWorkTemplate::EmptyNovel);
        assert_eq!(template_from_index(2), NewWorkTemplate::LightNovel);
        assert_eq!(template_from_index(3), NewWorkTemplate::Novel);
        assert_eq!(template_from_index(4), NewWorkTemplate::NoteBook);
        // Out-of-range falls back to the default (Novel).
        assert_eq!(template_from_index(99), NewWorkTemplate::Novel);
        assert_eq!(DEFAULT_TEMPLATE_INDEX, 3);
    }

    #[test]
    fn target_path_recomputes_on_change() {
        let vm = NewWorkViewModel::new(Rc::new(AppContext::new()));
        let path = vm.target_path();
        vm.location().set("~/Books".into());
        vm.name().set("Tidewrack".into());
        assert_eq!(path.get(), "~/Books/tidewrack.skrib");
        // Switching to bundle drops the extension.
        vm.format_idx().set(1);
        assert_eq!(path.get(), "~/Books/tidewrack");
    }

    #[test]
    fn dto_carries_form_choices() {
        let vm = NewWorkViewModel::new(Rc::new(AppContext::new()));
        vm.location().set("~/Books".into());
        vm.name().set("Tidewrack".into());
        vm.format_idx().set(1); // bundle
        vm.template_idx().set(1); // Empty Novel
        vm.language().set(Some("fr-FR".into()));

        let dto = vm.dto();
        assert_eq!(dto.file_name, "~/Books/tidewrack");
        assert!(dto.is_folder);
        assert_eq!(dto.template_kind, NewWorkTemplate::EmptyNovel);
        assert_eq!(dto.language, "fr-FR");
        assert_eq!(dto.labels.len(), 7);
    }
}
