// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `NewWorkViewModel` — the New Work dialog's business logic.
//!
//! Single-instance live state (like `EditorsViewModel`): it owns the form's
//! `Signal`s, which must survive panel rebuilds within one modal session, so it
//! is created once in `NewWorkPanel::new` and shared by `.clone()`. The dialog's
//! actions (derive the target path, assemble the `NewWorkDto`, create the work)
//! live here, not in the view's `build()`.
//!
//! Three presentation contexts, one behaviour split on [`NewWorkViewModel::create`] — see
//! [`CreateTarget`]:
//!   * **From an already-open project** (`NewWorkPanel::new` — File ▸ New
//!     Work / Ctrl+N): creates the work in place, replacing this window's
//!     project — the same "load in place" pattern as `work.open`/Ctrl+O.
//!   * **From the Launcher** (`NewWorkPanel::new_for_launcher` —
//!     `WelcomeViewModel::new_work`): creation is *deferred* to a freshly
//!     opened project window's first build
//!     ([`crate::app::PendingAction::New`]), which then closes the Launcher.
//!     Creating the work here instead — before that window's `NewWork`
//!     subscription is live — would race the event and silently skip the
//!     seed flow (`AppIds::seed`, `SingleWork::set_id`, the tree reload, …).
//!   * **From a project window that cannot replace its project in place**
//!     (`NewWorkPanel::new_beside_current` — File ▸ New Work in a window whose
//!     Work is also shown by a Work ▸ New Window sibling): same deferred
//!     creation as the Launcher, but the presenting window *stays open*. It
//!     shares one `AppIds`/`WorkSession` with its sibling, so replacing its
//!     project in place would re-point the sibling's project out from under it;
//!     the new project therefore gets a window of its own and both existing
//!     windows are left exactly as they were.
//!
//! Orthogonal to *where* the project lands is *what it is for* — see
//! [`NewWorkPurpose`]. The Launcher's "From documents…" reuses this whole form
//! for the project an import is about to fill, which changes which questions are
//! worth asking, not where the answers go.

use std::path::Path;
use std::rc::Rc;

use crate::models::{ParatextPreset, ParatextPresetsService};

use teksilo::prelude::*; // EventContext, Signal, tr!
use teksilo::widgets::{Toast, ValidationState};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::common::entities::GoalUnit;
use frontend::work_management::{NewWorkDto, NewWorkTemplate};

use crate::app::PendingAction;
use crate::shell::windows::ProjectWindowFactory;

/// Build the `NewWorkDto` for the New Work dialog.
///
/// The backend can't do i18n, so the template's human labels are resolved *here*
/// and passed in, in the exact order `work_management`'s `TemplateLabels::from_list`
/// reads them: `[Manuscript, Notes, Research, Notebook, Chapter, Scene, Note]`.
/// `language` is a locale tag (e.g. `"en-US"`) that becomes the new work's
/// `dict_language`. Parsed through the shared helper so an unset choice yields no tags
/// rather than a list holding one empty string.
#[allow(clippy::too_many_arguments)]
pub(crate) fn new_work_dto(
    file_name: String,
    is_folder: bool,
    template_kind: NewWorkTemplate,
    language: String,
    chapter_scene_mode: bool,
    paratexts: ParatextPlanDto,
    author_name: String,
    goal_unit: GoalUnit,
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
            // Appended, never inserted — the list is positional. These two name the
            // folders the paratexts land in; the item titles inside them are NOT here,
            // because they come from the preset verbatim in their own language.
            tr!(new_work_front_matter()).into(),
            tr!(new_work_back_matter()).into(),
        ],
        language: skribisto_model::language::parse_legacy_list(&language),
        chapter_scene_mode,
        // Optional: an empty string is the ordinary "not set" state, not an error.
        author_name,
        paratext_front: paratexts.front,
        paratext_back: paratexts.back,
        goal_unit,
    }
}

/// The chosen paratext structure, already resolved from its preset file by the caller.
///
/// A plain pair of title lists rather than a preset id: the backend has no business
/// parsing preset files, and this keeps `new_work_dto` a pure mapping.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParatextPlanDto {
    pub front: Vec<String>,
    pub back: Vec<String>,
}

/// Every paratext preset the picker can offer, bundled first, plus the one the interface
/// locale suggests.
///
/// Opens the **real** presets file, so a preset written in Settings is one this dialog
/// offers. Read once per dialog rather than held: the settings file is the source of
/// truth, and a dialog built after an edit must see the edit.
///
/// Falls back to a throwaway service when the config dir is unavailable, so a New Work
/// dialog never fails to open because of an optional settings file — the same degrade the
/// loader applies to a malformed entry.
fn load_paratext_presets() -> (Rc<Vec<ParatextPreset>>, Option<String>) {
    let svc = crate::identity::app_paths()
        .and_then(|paths| ParatextPresetsService::open(&paths).ok())
        .unwrap_or_else(ParatextPresetsService::in_memory_default);
    // The locale match is the service's own rule, asked once — not restated here, or the
    // two copies drift the first time the precedence changes.
    let preselected = svc
        .preselect_for_locale(&current_locale_tag().unwrap_or_default())
        .map(|p| p.id);
    (Rc::new(svc.all()), preselected)
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
    teksilo::i18n::current_locale().map(|sig| sig.get().to_string())
}

#[derive(Clone)]
pub struct NewWorkViewModel {
    /// The work's display name (drives the filename slug).
    name: Signal<String>,
    /// The author's name, written to the manifest and used by the compiler for
    /// the title page and the exported metadata. Optional — blank is normal.
    author: Signal<String>,
    /// Format segment: `0` = single `.skrib` file, `1` = bundle folder.
    format_idx: Signal<usize>,
    /// The containing folder chosen via the file picker.
    location: Signal<String>,
    /// Selected default-language locale tag (`Some("en-US")`), or `None`.
    language: Signal<Option<String>>,
    /// Template segment index (`0..=4`).
    template_idx: Signal<usize>,
    /// When set (and a manuscript template is selected), generate one flat
    /// `ChapterScene` per chapter — a chapter the user writes straight into —
    /// instead of a `Chapter` folder holding an empty `Scene`. Ignored by the
    /// non-manuscript templates. Defaults to `false` (the classic layout).
    chapter_scene: Signal<bool>,
    /// Which unit this project's targets will be counted in.
    ///
    /// Seeded from the chosen language and re-seeded while `goal_unit_touched` is false,
    /// so a writer who picks Japanese sees the picker move to characters — and a writer who
    /// set it by hand keeps their answer even if they then change the language.
    goal_unit: Signal<GoalUnit>,
    goal_unit_touched: Signal<bool>,
    /// The chosen paratext preset's id, or empty for "None" — the writer wanting no
    /// front or back matter at all, which is a first-class answer and the default when
    /// the interface locale matches no tradition.
    paratext_preset: Signal<Option<String>>,
    /// Every preset the picker can offer, bundled and user-written. Read once when the
    /// dialog is built: a preset added in Settings while the dialog is open is a case
    /// nobody meets, and re-reading per keystroke would parse every file on every frame.
    paratext_presets: Rc<Vec<ParatextPreset>>,
    app_ctx: Rc<AppContext>,
    /// Where "Create Work" puts the new project — see [`CreateTarget`].
    target: CreateTarget,
    /// What the project being created is *for* — see [`NewWorkPurpose`].
    purpose: NewWorkPurpose,
}

/// What the project this form creates is for.
///
/// The Launcher's "From documents…" makes a project whose entire content is
/// about to be imported, which silences two of the form's questions rather than
/// adding a form of its own:
///   * **Template** — every template lays down a manuscript the import is then
///     poured beside, which is the collision the writer sees as duplicate
///     chapters. `FromDocuments` pins `NewWorkTemplate::None`.
///   * **Book structure** — paratexts are only ever built around a Book row (see
///     `work_management`'s `build_template_with_paratexts`), and with no
///     template there is no book to furnish, so the picker could only lie.
///
/// The other questions all still matter: a name and a location because the file
/// must go somewhere, a language because the imported prose is spell-checked in
/// it, and flat chapters because that is `Work.chapter_mode`, which is what the
/// import resolves its own chapter rows through.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NewWorkPurpose {
    /// An ordinary project: every question is asked.
    Project,
    /// A project the Import documents wizard is about to fill — the Launcher's
    /// "From documents…". Creation is followed by that wizard opening over the
    /// new project (see [`crate::app::PendingAction::New`]'s `then_import`).
    FromDocuments,
}

/// Where [`NewWorkViewModel::create`] puts the project it is about to create.
///
/// One enum rather than an `Option<AppIds>` + `Option<ProjectWindowFactory>`
/// pair: the pair could represent "both" and "neither", neither of which means
/// anything, and the third context (create beside a window that must keep its
/// own project) is a genuine third case rather than a flag on one of the first
/// two.
#[derive(Clone)]
enum CreateTarget {
    /// Replace the presenting window's own project, in place. The `AppIds` is
    /// **that window's own** — read (`.work_id.get()`) at [`NewWorkViewModel::create`]
    /// time to close the outgoing Work before replacing it, never
    /// `ctx.app_state::<AppIds>()`, which is one process-wide slot fixed at
    /// builder time from the *first* window's session (see
    /// `crate::app::close_outgoing_work`'s doc): with a second Work open in a
    /// second window, that slot would name the WRONG window's Work, and closing
    /// it would tear a sibling window's live, untouched Work out from under it.
    InPlace(crate::app_ids::AppIds),
    /// Create it in a **new** project window, which performs the creation on its
    /// own first build ([`crate::app::PendingAction::New`]).
    ///
    /// `close_presenting_window` distinguishes the two callers: the Launcher
    /// exists only until a project window replaces it, so it closes; a project
    /// window whose Work is shared with a Work ▸ New Window sibling keeps its
    /// own project and simply gains a neighbour, so it stays.
    NewWindow {
        // Boxed: the factory is by far the largest thing this enum holds, and an
        // unboxed variant makes every `CreateTarget` — including the small
        // `InPlace` one — as big as the biggest.
        factory: Box<ProjectWindowFactory>,
        close_presenting_window: bool,
    },
}

#[allow(dead_code)]
impl NewWorkViewModel {
    /// For `NewWorkPanel::new` — presented over an already-open project. `ids`
    /// is THIS window's own `AppIds` — see the field's own doc for why
    /// [`Self::create`] must resolve the outgoing Work through it rather than
    /// `ctx.app_state`.
    pub fn new(app_ctx: Rc<AppContext>, ids: crate::app_ids::AppIds) -> Self {
        let (presets, preselected) = load_paratext_presets();
        Self {
            name: Signal::new(String::new()),
            author: Signal::new(String::new()),
            format_idx: Signal::new(0),
            location: Signal::new(default_location()),
            language: Signal::new(current_locale_tag()),
            template_idx: Signal::new(DEFAULT_TEMPLATE_INDEX),
            chapter_scene: Signal::new(false),
            goal_unit: Signal::new(GoalUnit::default()),
            goal_unit_touched: Signal::new(false),
            paratext_preset: Signal::new(preselected),
            paratext_presets: presets,
            app_ctx,
            target: CreateTarget::InPlace(ids),
            purpose: NewWorkPurpose::Project,
        }
    }

    /// For `NewWorkPanel::new_for_launcher` — presented from the Launcher, no
    /// project open yet (so there is nothing to close). `factory` builds the
    /// project window that "Create Work" opens once the form is submitted; the
    /// Launcher closes behind it.
    pub fn new_for_launcher(app_ctx: Rc<AppContext>, factory: ProjectWindowFactory) -> Self {
        Self::in_a_new_window(app_ctx, factory, true)
    }

    /// [`Self::new_for_launcher`] for the Launcher's **From documents…**: the
    /// same form in [`NewWorkPurpose::FromDocuments`], followed by the Import
    /// documents wizard opening over the project it creates.
    ///
    /// The files are **not** chosen here. They used to be — the door opened a
    /// file picker before the form — and the writer then met the very same
    /// question again in the import wizard afterwards, because that wizard is
    /// where files are reviewed, ordered and pointed at a destination. One
    /// question, asked once, in the place that can act on the answer.
    pub fn new_for_launcher_from_documents(
        app_ctx: Rc<AppContext>,
        factory: ProjectWindowFactory,
    ) -> Self {
        Self::in_a_new_window(app_ctx, factory, true).for_documents()
    }

    /// Switch this form to [`NewWorkPurpose::FromDocuments`].
    ///
    /// Separate from the constructor because the purpose is orthogonal to where
    /// the project lands ([`CreateTarget`]) — only the Launcher door needs it
    /// today, but nothing about it is Launcher-specific.
    pub(crate) fn for_documents(mut self) -> Self {
        self.purpose = NewWorkPurpose::FromDocuments;
        // No template, and therefore no paratexts: the import supplies the
        // structure. Pinned on the state rather than only in the view, so the
        // DTO is right even though the form never shows these controls.
        self.template_idx.set(0);
        self.paratext_preset.set(None);
        self
    }

    /// For `NewWorkPanel::new_beside_current` — presented from a project window
    /// that must **not** replace its own project: its Work is also shown by a
    /// Work ▸ New Window sibling, and the two share one `AppIds`/`WorkSession`,
    /// so an in-place replace would re-point the sibling's project too. The new
    /// project opens in its own window and the presenting window stays exactly
    /// as it was.
    pub fn new_beside_current(app_ctx: Rc<AppContext>, factory: ProjectWindowFactory) -> Self {
        Self::in_a_new_window(app_ctx, factory, false)
    }

    fn in_a_new_window(
        app_ctx: Rc<AppContext>,
        factory: ProjectWindowFactory,
        close_presenting_window: bool,
    ) -> Self {
        let (presets, preselected) = load_paratext_presets();
        Self {
            name: Signal::new(String::new()),
            author: Signal::new(String::new()),
            format_idx: Signal::new(0),
            location: Signal::new(default_location()),
            language: Signal::new(current_locale_tag()),
            template_idx: Signal::new(DEFAULT_TEMPLATE_INDEX),
            chapter_scene: Signal::new(false),
            goal_unit: Signal::new(GoalUnit::default()),
            goal_unit_touched: Signal::new(false),
            paratext_preset: Signal::new(preselected),
            paratext_presets: presets,
            app_ctx,
            purpose: NewWorkPurpose::Project,
            target: CreateTarget::NewWindow {
                factory: Box::new(factory),
                close_presenting_window,
            },
        }
    }

    /// What this project is for — the view asks so it can drop the questions
    /// [`NewWorkPurpose::FromDocuments`] answers on the writer's behalf.
    pub fn purpose(&self) -> NewWorkPurpose {
        self.purpose
    }

    // ── Signal accessors (bound by the view) ───────────────────────────────
    pub fn name(&self) -> Signal<String> {
        self.name.clone()
    }
    pub fn author(&self) -> Signal<String> {
        self.author.clone()
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
    pub fn chapter_scene(&self) -> Signal<bool> {
        self.chapter_scene.clone()
    }

    pub fn goal_unit(&self) -> Signal<GoalUnit> {
        self.goal_unit.clone()
    }

    /// The picker's own change handler: record that the writer has spoken, so the
    /// language no longer overrides it.
    pub fn set_goal_unit(&self, unit: GoalUnit) {
        self.goal_unit_touched.set(true);
        self.goal_unit.set(unit);
    }

    /// Re-seed the unit from the language, unless the writer has already chosen one.
    ///
    /// Called from an effect on the language field rather than derived, because "has the
    /// writer touched it" is state and a derived signal cannot hold any.
    pub fn language_changed(&self) {
        if self.goal_unit_touched.get() {
            return;
        }
        let tag = self.language.get().unwrap_or_default();
        self.goal_unit
            .set(skribisto_model::goal_unit::default_unit_for_language(&tag));
    }

    /// Whether the "write directly in chapters" toggle applies to the current
    /// selection — true only for the three manuscript templates (Empty Novel,
    /// Light Novel, Novel = indices 1/2/3). Drives the toggle's `enabled` state
    /// so it greys out for None (0) / Notebook (4).
    ///
    /// Always true for [`NewWorkPurpose::FromDocuments`], whose template is
    /// pinned to None: the flag is not really about the template but about
    /// `Work.chapter_mode`, and the import resolves every chapter row it creates
    /// through that mode. Greying it there would be the form refusing to ask the
    /// one structural question the import actually obeys.
    pub fn chapter_scene_applicable(&self) -> Signal<bool> {
        if self.purpose == NewWorkPurpose::FromDocuments {
            return Signal::new(true);
        }
        self.template_idx.map(|i| matches!(*i, 1..=3))
    }

    /// Whether a paratext structure can be applied — the same three manuscript templates,
    /// because front and back matter are the furniture of a book and the other templates
    /// build none. Greyed rather than hidden, so the control does not appear and vanish
    /// as the template changes.
    ///
    /// Never for [`NewWorkPurpose::FromDocuments`] — its template is None, so
    /// there is no Book row to furnish and the picker is not shown at all.
    pub fn paratext_applicable(&self) -> Signal<bool> {
        if self.purpose == NewWorkPurpose::FromDocuments {
            return Signal::new(false);
        }
        self.template_idx.map(|i| matches!(*i, 1..=3))
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

    /// Whether the wizard may leave its first step — a non-blank name **and** a
    /// valid location. Split into two per-field booleans so typing the name
    /// doesn't re-probe the filesystem (the location check only reruns on a
    /// location change).
    ///
    /// This is the Stepper's only gate: it sits on the Details step, which is
    /// where both of those fields live, so Next stays off until creation could
    /// actually succeed. The later steps are pickers with valid defaults and
    /// gate nothing.
    ///
    /// Under `--features mocks` the gate is off: a mocks build has no backend
    /// and its "location" probe would still write to the real filesystem, so
    /// requiring a typed name and a writable folder would stop layout work and
    /// automation from walking past step one. Same bypass as the import
    /// wizard's `can_analyse_signal`.
    pub fn can_create(&self) -> Signal<bool> {
        if cfg!(feature = "mocks") {
            return Signal::new(true);
        }
        let name_ok = self.name.map(|n| !slugify(n).is_empty());
        let location_ok = self.location.map(|dir| location_ok(dir));
        name_ok.and(&location_ok)
    }

    /// The paratext titles the chosen preset asks for, verbatim.
    ///
    /// Empty for "None", and empty for a preset that has since been deleted or broken —
    /// creating a project with no front matter is always better than refusing to create
    /// one.
    fn paratext_plan(&self) -> ParatextPlanDto {
        let Some(id) = self.paratext_preset.get() else {
            return ParatextPlanDto::default();
        };
        self.paratext_presets
            .iter()
            .find(|p| p.id == id)
            .map(|p| ParatextPlanDto {
                front: p.front.clone(),
                back: p.back.clone(),
            })
            .unwrap_or_default()
    }

    /// Every preset, for the picker. Each names itself, in its own language.
    pub fn paratext_presets(&self) -> Rc<Vec<ParatextPreset>> {
        self.paratext_presets.clone()
    }

    /// The chosen preset, or `None` for no structure at all.
    pub fn paratext_preset(&self) -> Signal<Option<String>> {
        self.paratext_preset.clone()
    }

    /// Build the DTO from the current form state.
    fn dto(&self) -> NewWorkDto {
        new_work_dto(
            build_target_path(
                &self.location.get(),
                &self.name.get(),
                self.format_idx.get(),
            ),
            self.format_idx.get() == 1,
            template_from_index(self.template_idx.get()),
            self.language.get().unwrap_or_default(),
            self.chapter_scene.get(),
            self.paratext_plan(),
            // Trimmed so a field containing only spaces reads as unset rather
            // than putting whitespace on the title page.
            self.author.get().trim().to_string(),
            self.goal_unit.get(),
        )
    }

    /// "Create Work".
    ///
    /// Already in a project window (`launcher_factory` is `None`): close the
    /// open project's own backend subtree — `crate::app::close_outgoing_work`,
    /// see its doc — then create the new work in place, then dismiss. This is
    /// the actual point of no return (the unsaved-changes guard already
    /// resolved before this form was ever shown, and Cancel up to this exact
    /// click leaves the outgoing project untouched), so closing it right here,
    /// immediately before `new_work`, never leaves the window showing nothing
    /// for longer than this one synchronous call. On failure the toast
    /// surfaces the error and the dialog stays open to retry — the outgoing
    /// project is already closed by then (its file on disk is unaffected; the
    /// unsaved-changes guard already saved or discarded anything in memory
    /// before offering this form), so a retry creates fresh rather than
    /// resuming a still-open one.
    ///
    /// From the Launcher (`launcher_factory` is `Some`): don't touch the
    /// backend here — open a project window carrying this DTO as its
    /// `PendingAction::New` (it creates the work on its own first build, once
    /// its `NewWork` subscription is live), then close the Launcher. There is
    /// no synchronous failure to report inline in this path; a creation error
    /// there is `eprintln!`-only (see `App::build`), matching the argv/Open
    /// path's existing error handling. Nothing is open in the Launcher window,
    /// so there is nothing to close.
    /// Returns whether the work was created — the wizard's Finish gate. `false`
    /// keeps the writer on the last step with it marked in error, instead of a
    /// flow that reports itself finished over a project that does not exist.
    /// Only the in-place path can fail synchronously; the deferred ones have
    /// handed the work to another window by the time anything could go wrong.
    pub fn create(&self, ctx: &mut EventContext) -> bool {
        if cfg!(feature = "mocks") {
            // A mocks build has no backend to create into, and the gate that
            // normally guarantees a usable name and a writable folder is off
            // (see [`Self::can_create`]) — so the real call would try to write
            // a project at whatever the untouched form says and toast a
            // failure. Finish just closes the wizard, the same "walk the flow,
            // touch nothing" bargain the import wizard's mock bypass makes.
            ctx.dismiss_modal();
            return true;
        }
        match &self.target {
            CreateTarget::InPlace(ids) => {
                crate::app::close_outgoing_work(&self.app_ctx, ids.work_id.get());
                match work_management_commands::new_work(&self.app_ctx, &self.dto()) {
                    Ok(()) => ctx.dismiss_modal(),
                    Err(e) => {
                        ctx.show_toast(Toast::error(tr!(could_not_create_work(
                            error = e.to_string()
                        ))));
                        return false;
                    }
                }
            }
            CreateTarget::NewWindow {
                factory,
                close_presenting_window,
            } => {
                // The returned `InitialWindowState` is only kept by `main.rs`'s
                // very first window (see `window_config`'s doc) — every later
                // window, like this one, discards it.
                let (config, _state) = factory.window_config(PendingAction::New {
                    dto: self.dto(),
                    then_import: self.purpose == NewWorkPurpose::FromDocuments,
                });
                ctx.open_window(config);
                if *close_presenting_window {
                    ctx.close_window();
                } else {
                    // A project window: it keeps its own project, so only the
                    // form goes away. Dismissing is not optional — the modal
                    // would otherwise stay up over a window that has just
                    // handed the user's request to a different one.
                    ctx.dismiss_modal();
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The author typed into the New Work form must reach the DTO — otherwise the
    /// field is decorative and every project starts unattributed.
    #[test]
    fn the_author_field_reaches_the_new_work_dto() {
        let dto = new_work_dto(
            "/tmp/x.skrib".into(),
            false,
            NewWorkTemplate::Novel,
            "en-US".into(),
            false,
            ParatextPlanDto::default(),
            "A. Writer".into(),
            GoalUnit::default(),
        );
        assert_eq!(dto.author_name, "A. Writer");
    }

    /// The name is optional. Left blank — or filled with only spaces, which the
    /// view-model trims — it must arrive empty rather than as whitespace that
    /// would print as a blank line on the title page.
    #[test]
    fn an_unset_author_arrives_empty() {
        for typed in ["", "   "] {
            let dto = new_work_dto(
                "/tmp/x.skrib".into(),
                false,
                NewWorkTemplate::Novel,
                "en-US".into(),
                false,
                ParatextPlanDto::default(),
                typed.trim().to_string(),
                GoalUnit::default(),
            );
            assert_eq!(dto.author_name, "", "{typed:?} must arrive as unset");
        }
    }

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
        assert_eq!(
            build_target_path("~/Novels", "Tidewrack", 1),
            "~/Novels/tidewrack"
        );
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
        let vm = NewWorkViewModel::new(Rc::new(AppContext::new()), crate::app_ids::AppIds::new());
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
        let vm = NewWorkViewModel::new(Rc::new(AppContext::new()), crate::app_ids::AppIds::new());
        vm.location().set("~/Books".into());
        vm.name().set("Tidewrack".into());
        vm.format_idx().set(1); // bundle
        vm.template_idx().set(1); // Empty Novel
        vm.language().set(Some("fr-FR".into()));
        vm.chapter_scene().set(true);

        let dto = vm.dto();
        assert_eq!(dto.file_name, "~/Books/tidewrack");
        assert!(dto.is_folder);
        assert_eq!(dto.template_kind, NewWorkTemplate::EmptyNovel);
        assert_eq!(dto.language, vec!["fr-FR".to_string()]);
        // Seven binder names plus the two paratext folder names, appended.
        assert_eq!(dto.labels.len(), 9);
        assert!(dto.chapter_scene_mode);
    }

    /// The wizard's one gate, in a real build: an untouched form cannot leave
    /// step one, and a usable name over a writable folder can.
    #[test]
    #[cfg(not(feature = "mocks"))]
    fn the_details_gate_needs_a_name_and_a_writable_folder() {
        let vm = NewWorkViewModel::new(Rc::new(AppContext::new()), crate::app_ids::AppIds::new());
        let gate = vm.can_create();
        vm.location()
            .set(std::env::temp_dir().to_string_lossy().to_string());
        assert!(!gate.get(), "a blank name must hold the wizard on step one");
        vm.name().set("Tidewrack".into());
        assert!(
            gate.get(),
            "a usable name + a writable folder must open Next"
        );
        // A name that slugifies to nothing is as unusable as a blank one.
        vm.name().set("///".into());
        assert!(!gate.get());
        // …and so is a folder that does not exist.
        vm.name().set("Tidewrack".into());
        vm.location().set("/nonexistent-skribisto-probe".into());
        assert!(!gate.get());
    }

    /// Under `mocks` that gate is off, or an untouched wizard could not be
    /// walked past its first step without typing a name and pointing at a real
    /// writable folder — which is exactly what a backend-less build is for.
    #[test]
    #[cfg(feature = "mocks")]
    fn the_details_gate_is_off_under_mocks() {
        let vm = NewWorkViewModel::new(Rc::new(AppContext::new()), crate::app_ids::AppIds::new());
        let gate = vm.can_create();
        assert!(gate.get(), "an untouched mocks form must still advance");
        vm.location().set("/nonexistent-skribisto-probe".into());
        assert!(gate.get(), "no filesystem probe may gate a mocks build");
    }

    /// The Launcher's "From documents…" must create an **empty** project: a
    /// template would lay down a manuscript beside the one about to be imported,
    /// which is exactly the duplication the flow exists to avoid. Paratexts go
    /// with it — they are only ever built around a Book row, which no longer
    /// exists.
    #[test]
    fn a_from_documents_project_carries_no_template_and_no_paratexts() {
        let vm = NewWorkViewModel::new(Rc::new(AppContext::new()), crate::app_ids::AppIds::new())
            .for_documents();
        vm.location().set("~/Books".into());
        vm.name().set("Tidewrack".into());

        let dto = vm.dto();
        assert_eq!(dto.template_kind, NewWorkTemplate::None);
        assert!(dto.paratext_front.is_empty() && dto.paratext_back.is_empty());
        // The picker is not shown, and could do nothing if it were.
        assert!(!vm.paratext_applicable().get());
        // …but flat chapters still matter: that is `Work.chapter_mode`, which
        // every chapter the import creates is resolved through.
        assert!(vm.chapter_scene_applicable().get());
        vm.chapter_scene().set(true);
        assert!(vm.dto().chapter_scene_mode);
    }

    /// A paratext preselected from the interface locale must not sneak into a
    /// from-documents project: `for_documents` clears the choice, so nothing
    /// depends on the preset file happening to match no locale.
    #[test]
    fn from_documents_clears_a_preselected_paratext() {
        let vm = NewWorkViewModel::new(Rc::new(AppContext::new()), crate::app_ids::AppIds::new());
        vm.paratext_preset().set(Some("us-trade-novel".into()));
        let vm = vm.for_documents();
        assert_eq!(vm.paratext_preset().get(), None);
    }

    #[test]
    fn chapter_scene_applies_only_to_manuscript_templates() {
        let vm = NewWorkViewModel::new(Rc::new(AppContext::new()), crate::app_ids::AppIds::new());
        let applicable = vm.chapter_scene_applicable();
        // Manuscript templates (Empty Novel / Light Novel / Novel).
        for idx in [1, 2, 3] {
            vm.template_idx().set(idx);
            assert!(applicable.get(), "idx {idx} should enable the toggle");
        }
        // None (0) and Notebook (4) grey it out.
        for idx in [0, 4] {
            vm.template_idx().set(idx);
            assert!(!applicable.get(), "idx {idx} should disable the toggle");
        }
    }
}
