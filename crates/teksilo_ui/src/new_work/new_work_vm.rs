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

use std::rc::Rc;

use crate::models::{ParatextPreset, ParatextPresetsService};

use teksilo::prelude::*; // EventContext, Signal, tr!
use teksilo::widgets::{
    MessageBox, MessageBoxButtons, StandardButton, StepStatus, StepperController, Toast,
    ValidationState,
};

use crate::shared::form_checks::{CachedValidation, DiskChecked, FolderMessages, folder_state};
use crate::shared::import_destination::{OpenRefusal, refuse_if_open_saying};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::common::entities::GoalUnit;
use frontend::work_management::{NewWorkDto, NewWorkTemplate};

use crate::app::PendingAction;
use crate::shell::windows::ProjectWindowFactory;
use crate::tags::Preset;

/// Build the `NewWorkDto` for the New Work dialog.
///
/// The backend can't do i18n, so the template's human labels are resolved *here*
/// and passed in, in the exact order `work_management`'s `TemplateLabels::from_list`
/// reads them: `[Manuscript, Notes, Research, Notebook, <retired>, Scene, Note, Front
/// matter, Back matter, Characters, Places]`.
///
/// `title` is the name the writer typed. It is **not** derivable from `file_name`, which
/// this same view-model slugified to build (`"The Long Road"` → `.../the-long-road.skrib`)
/// — reading the title back out of the path is what put `the-long-road` on the project,
/// the Book row and the exported title page.
///
/// `language` is a locale tag (e.g. `"en-US"`) that becomes the new work's
/// `dict_language`. Parsed through the shared helper so an unset choice yields no tags
/// rather than a list holding one empty string.
#[allow(clippy::too_many_arguments)]
pub(crate) fn new_work_dto(
    file_name: String,
    title: String,
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
        title,
        is_folder,
        template_kind,
        labels: vec![
            tr!(new_work_manuscript()).into(),
            tr!(new_work_notes()).into(),
            tr!(new_work_research()).into(),
            tr!(new_work_notebook()).into(),
            // Slot 4 held the word "Chapter", which the template wrote into every
            // generated chapter title. It does not any more — a chapter is named by the
            // manuscript's own numbering, in the language it is *written* in, not the
            // interface language this list is resolved in. The slot stays occupied
            // rather than being reclaimed: the list is positional, so reusing it would
            // silently retitle everything after it.
            String::new(),
            tr!(new_work_scene()).into(),
            tr!(new_work_note()).into(),
            // Appended, never inserted. These two name the folders the paratexts land
            // in; the item titles inside them are NOT here, because they come from the
            // preset verbatim in their own language.
            tr!(new_work_front_matter()).into(),
            tr!(new_work_back_matter()).into(),
            // Appended later still: the two story-bible folders the Notes binder ships
            // with, beside `new_work_research` above which names the third.
            tr!(new_work_characters()).into(),
            tr!(new_work_places()).into(),
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

/// Map the Template `RadioTileGroup` index to its `NewWorkTemplate`.
///
/// Kept in one place so the view (tile order) and the DTO stay in lockstep.
pub(crate) fn template_from_index(index: usize) -> NewWorkTemplate {
    match index {
        0 => NewWorkTemplate::None,
        1 => NewWorkTemplate::EmptyNovel,
        2 => NewWorkTemplate::LightNovel,
        4 => NewWorkTemplate::NovelInParts,
        5 => NewWorkTemplate::NoteBook,
        // 3 (Novel) is the default selection; any out-of-range index falls back
        // to it too.
        _ => NewWorkTemplate::Novel,
    }
}

/// The Template tile index for the default (Novel) selection.
pub(crate) const DEFAULT_TEMPLATE_INDEX: usize = 3;

/// How many tiles the Template step shows — one per `NewWorkTemplate`.
///
/// The panel builds its tiles as a chained `RadioTileGroup`, which cannot be counted from
/// outside, so this is the number the two agree on by hand. `every_template_is_reachable`
/// below pins that every variant is reachable exactly once within it, which catches a
/// variant added without a tile and a count bumped without a mapping.
pub(crate) const TEMPLATE_TILE_COUNT: usize = 6;

/// The tile indices that build a **book** — every novel-family template. The flat-chapter
/// toggle and the paratext picker both apply to exactly these, so the range lives here
/// rather than being spelled out at each of them (it was `1..=3` in two places, and
/// adding the parts template would have silently missed both).
const MANUSCRIPT_TEMPLATE_INDICES: std::ops::RangeInclusive<usize> = 1..=4;

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

/// The Location field's words. The folder must exist, be a directory and be
/// writable (a new work is created there); the check itself is shared with the
/// import dialogs and runs once per edit, never on a read (it writes a probe
/// file, see `shared::form_checks`).
static LOCATION: FolderMessages = FolderMessages {
    required: || tr!(new_work_location_required()),
    missing: || tr!(new_work_location_missing()),
    not_folder: || tr!(new_work_location_not_folder()),
    readonly: || tr!(new_work_location_readonly()),
};

/// The Location field's verdict, worked out whenever `location` is set.
fn location_check(location: &Signal<String>) -> CachedValidation {
    let dir = location.clone();
    CachedValidation::new(&[location], move || folder_state(&dir.get(), &LOCATION))
}

/// What is already at the target `<folder>/<slug>`, as the Name field reports it.
///
/// A file there is a project Create would replace, so it is a warning, and Create
/// asks before replacing it, as the project importers do. A folder there is refused
/// outright: a folder project cannot be replaced by writing a new one into it without
/// leaving the old one's files behind in it.
fn target_state(target: &str) -> ValidationState {
    if target.is_empty() {
        return ValidationState::None;
    }
    let path = std::path::Path::new(target);
    if path.is_dir() {
        ValidationState::Error(tr!(new_work_target_is_folder()))
    } else if path.exists() {
        ValidationState::Warning(tr!(new_work_target_exists()))
    } else {
        ValidationState::None
    }
}

/// The target's verdict, worked out whenever the folder, the name or the format is set.
fn target_check(
    location: &Signal<String>,
    name: &Signal<String>,
    format_idx: &Signal<usize>,
) -> CachedValidation {
    let (dir, stem, format) = (location.clone(), name.clone(), format_idx.clone());
    CachedValidation::new(&[location, name], move || {
        target_state(&build_target_path(&dir.get(), &stem.get(), format.get()))
    })
    .also_on(format_idx)
}

/// The words New Work refuses a target that is open in a window with.
static TARGET_OPEN: OpenRefusal = OpenRefusal {
    title: || tr!(new_work_target_open_title()),
    text: |name| tr!(new_work_target_open_text(name = name)),
};

/// One press of Create, as it was asked: the project to create and what it starts
/// with, taken from the form at that moment. The overwrite question can wait while the
/// form is edited behind it, and OK creates what was asked about.
#[derive(Clone)]
struct CreateRequest {
    dto: NewWorkDto,
    starters: crate::app::ProjectStarters,
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
    /// [`LOCATION`]'s verdict on `location`, cached.
    location_check: CachedValidation,
    /// What is already at the target the form names, cached ([`target_state`]).
    target_check: CachedValidation,
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
    /// Which tag palette the project starts with, or `None` for none at all.
    ///
    /// `None` is the default and a real answer, not an unset one: a project with no
    /// tags is a project whose writer has not decided they are keeping a story bible,
    /// and every surface that reads the palette already works with an empty one. It is
    /// also why this question can be asked here at all without being a commitment: a
    /// preset only *seeds* the palette, and every tag it lays down can be renamed,
    /// recoloured or deleted afterwards in Settings.
    tag_preset: Signal<Option<Preset>>,
    /// Which workflow ladder the project starts with. `None` means "the default one",
    /// not "none" — see `ProjectStarters::statuses`.
    status_preset: Signal<Option<crate::statuses::Preset>>,
    /// Which set of built-in note templates the project starts with, or `None` for none.
    ///
    /// The same bargain as [`Self::tag_preset`], and asked beside it: `None` is the
    /// default and a real answer, applying a set only *seeds* the list, and every
    /// template it lays down can be renamed, edited or deleted afterwards in Settings.
    /// It is here rather than left to Settings because it is the same question the
    /// template picker asks — what is already in the project on the first morning — and
    /// because the novel templates now ship the notes folders these templates fill.
    template_set: Signal<Option<crate::note_templates::StarterSet>>,
    app_ctx: Rc<AppContext>,
    /// Where "Create Work" puts the new project — see [`CreateTarget`].
    target: CreateTarget,
    /// What the project being created is *for* — see [`NewWorkPurpose`].
    purpose: NewWorkPurpose,
    /// Where the in-place path leaves [`Self::tag_preset`] and [`Self::template_set`] for
    /// the project that does not exist yet.
    ///
    /// `None` for every target that creates its project in a **different** window: that
    /// window has its own one-shot, which cannot be reached from here, so the answers
    /// travel on [`PendingAction::New`] instead and `App::build` arms them over there.
    pending_starters: Option<crate::app::PendingStarters>,
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
    pub(crate) fn new(
        app_ctx: Rc<AppContext>,
        ids: crate::app_ids::AppIds,
        pending_starters: crate::app::PendingStarters,
    ) -> Self {
        let (presets, preselected) = load_paratext_presets();
        let location = Signal::new(default_location());
        let name = Signal::new(String::new());
        let format_idx = Signal::new(0);
        Self {
            target_check: target_check(&location, &name, &format_idx),
            name,
            author: Signal::new(String::new()),
            format_idx,
            location_check: location_check(&location),
            location,
            language: Signal::new(current_locale_tag()),
            template_idx: Signal::new(DEFAULT_TEMPLATE_INDEX),
            chapter_scene: Signal::new(false),
            goal_unit: Signal::new(GoalUnit::default()),
            goal_unit_touched: Signal::new(false),
            paratext_preset: Signal::new(preselected),
            paratext_presets: presets,
            tag_preset: Signal::new(None),
            status_preset: Signal::new(None),
            template_set: Signal::new(None),
            app_ctx,
            target: CreateTarget::InPlace(ids),
            purpose: NewWorkPurpose::Project,
            pending_starters: Some(pending_starters),
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
        let location = Signal::new(default_location());
        let name = Signal::new(String::new());
        let format_idx = Signal::new(0);
        Self {
            target_check: target_check(&location, &name, &format_idx),
            name,
            author: Signal::new(String::new()),
            format_idx,
            location_check: location_check(&location),
            location,
            language: Signal::new(current_locale_tag()),
            template_idx: Signal::new(DEFAULT_TEMPLATE_INDEX),
            chapter_scene: Signal::new(false),
            goal_unit: Signal::new(GoalUnit::default()),
            goal_unit_touched: Signal::new(false),
            paratext_preset: Signal::new(preselected),
            paratext_presets: presets,
            tag_preset: Signal::new(None),
            status_preset: Signal::new(None),
            template_set: Signal::new(None),
            app_ctx,
            purpose: NewWorkPurpose::Project,
            target: CreateTarget::NewWindow {
                factory: Box::new(factory),
                close_presenting_window,
            },
            // The project is created in a window that does not exist yet, which has its
            // own one-shot; the answers ride on `PendingAction::New` instead.
            pending_starters: None,
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
        self.template_idx
            .map(|i| MANUSCRIPT_TEMPLATE_INDICES.contains(i))
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
        self.template_idx
            .map(|i| MANUSCRIPT_TEMPLATE_INDICES.contains(i))
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
    ///
    /// A usable name then reports what is already at the target it makes, from the
    /// cached check: a project Create will ask before replacing, or a folder it refuses.
    pub fn name_validation(&self) -> Signal<ValidationState> {
        self.name
            .zip(&self.target_check.signal())
            .map(|(n, target)| {
                if n.trim().is_empty() {
                    ValidationState::Error(tr!(new_work_name_required()))
                } else if slugify(n).is_empty() {
                    ValidationState::Error(tr!(new_work_name_invalid()))
                } else {
                    target.clone()
                }
            })
    }

    /// Inline validation for the Location field — the folder must exist, be a
    /// directory, and be writable. Cached: worked out when the location is set,
    /// never on a read, since the check writes a probe file into the folder.
    pub fn location_validation(&self) -> Signal<ValidationState> {
        self.location_check.signal()
    }

    /// Whether the wizard may leave its first step — a non-blank name **and** a
    /// valid location, and no folder at the target they name. Reads the cached
    /// verdicts, so painting the gate never touches the filesystem. Typing does,
    /// once per change: each keystroke in the name looks at the target on disk
    /// again, as a change of the format does, and a change of the location runs
    /// the location's own check as well, which writes a probe file.
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
        name_ok
            .and(&self.location_check.passes())
            .and(&self.target_check.passes())
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
    /// Which tag palette the new project starts with.
    pub fn tag_preset(&self) -> Signal<Option<Preset>> {
        self.tag_preset.clone()
    }

    /// Which set of built-in note templates the new project starts with.
    pub fn template_set(&self) -> Signal<Option<crate::note_templates::StarterSet>> {
        self.template_set.clone()
    }

    /// What this form asks the project to start with beyond its template.
    ///
    /// One value, because the two halves are armed and taken together — see
    /// [`crate::app::ProjectStarters`].
    fn starters(&self) -> crate::app::ProjectStarters {
        crate::app::ProjectStarters {
            tags: self.tag_preset.get(),
            templates: self.template_set.get(),
            statuses: self.status_preset.get(),
        }
    }

    /// The chosen ladder, for the New Work form's own picker.
    pub fn status_preset(&self) -> Signal<Option<crate::statuses::Preset>> {
        self.status_preset.clone()
    }

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
            // The name as typed, beside the slugified path built from it. Trimmed, so a
            // field of spaces reads as unset and the backend falls back to the file
            // stem rather than titling the book with whitespace.
            self.name.get().trim().to_string(),
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
    ///
    /// Either way the target is looked at first. A project open in a window there is
    /// refused, in the words the importers refuse one with, since creating over it
    /// would replace the project the window shows and the window's next save would
    /// write it back over the new one. A project merely there is replaced only once the
    /// writer has said so, in a question: while it is open the wizard stays on its
    /// last step (`false`), OK creates the project that was asked about, and Cancel
    /// sets `wizard`'s last step back from the error that `false` marks it with.
    ///
    /// Returns whether the work was created — the wizard's Finish gate. `false`
    /// keeps the writer on the last step with it marked in error, instead of a
    /// flow that reports itself finished over a project that does not exist.
    /// Only the in-place path can fail synchronously; the deferred ones have
    /// handed the work to another window by the time anything could go wrong.
    pub fn create(&self, ctx: &mut EventContext, wizard: Option<&StepperController>) -> bool {
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
        // The gate read verdicts cached when the fields were last set; the disk
        // may have moved on since. Both are checked again. A failure holds the
        // wizard, and since the fields saying why are on the first step while the
        // writer is on the last, the reason is also put in front of them here, the
        // way the other failure below is.
        for check in [&self.location_check, &self.target_check] {
            if !check.recheck() {
                let reason = check
                    .refusal()
                    .map(|reason| reason.resolve_now())
                    .unwrap_or_default();
                ctx.show_toast(Toast::error(tr!(could_not_create_work(error = reason))));
                return false;
            }
        }
        let request = CreateRequest {
            dto: self.dto(),
            starters: self.starters(),
        };
        let target = request.dto.file_name.clone();
        // Creating a project where one is open would replace the one a window is
        // showing, and that window's next save would write it straight back over
        // the new one: both lose work. Refused, as the importers refuse it.
        if refuse_if_open_saying(ctx, &target, &TARGET_OPEN) {
            return false;
        }
        if std::path::Path::new(&target).exists() {
            let vm = self.clone();
            let wizard = wizard.cloned();
            let name = std::path::Path::new(&target)
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| target.clone());
            MessageBox::warning(tr!(new_work_overwrite_title()))
                .text(tr!(new_work_overwrite_text(name = name)))
                .buttons(MessageBoxButtons::OkCancel)
                .on_result(move |answer, c| {
                    if answer.button != StandardButton::Ok {
                        // Declined: the wizard waits on its last step, not in error.
                        if let Some(wizard) = &wizard {
                            wizard.set_status(wizard.current(), StepStatus::Active);
                        }
                        return;
                    }
                    // The question can sit open while the writer opens that very
                    // project in another window, so the refusal is asked again at
                    // the last moment.
                    if !refuse_if_open_saying(c, &target, &TARGET_OPEN) {
                        vm.commit(c, request.clone(), Dismiss::TopOverlay);
                    }
                })
                .present(ctx);
            // Held on the last step while the question is open; OK finishes it.
            return false;
        }
        self.commit(ctx, request, Dismiss::Modal)
    }

    /// Create the project `request` names, where [`CreateTarget`] says, and close the
    /// form. `dismiss` is how: from the form's own button, its modal; from the
    /// overwrite question's answer, whose context stands at the tree's root, the
    /// topmost overlay, which is the form once the question has gone.
    fn commit(&self, ctx: &mut EventContext, request: CreateRequest, dismiss: Dismiss) -> bool {
        let close_form = |ctx: &mut EventContext| match dismiss {
            Dismiss::Modal => ctx.dismiss_modal(),
            Dismiss::TopOverlay => ctx.dismiss_top_overlay(),
        };
        match &self.target {
            CreateTarget::InPlace(ids) => {
                crate::app::close_outgoing_work(&self.app_ctx, ids.work_id.get());
                // Armed **before** the call, never applied after it. `new_work` returning
                // `Ok` does not mean `ids.work_id` names the new project: a subscription
                // event crosses `EventHubClient`'s own background thread and then the
                // winit event loop (`AppEventProxy::post_subscription_event`), so no
                // subscriber has run by the time this line is reached and `ids.work_id`
                // still holds the id of the project just closed. Writing a palette here
                // would write it against a deleted Work and lose the writer's choice
                // with nothing on screen to say so. The `NewWork` subscriber in
                // `wiring::project_events` takes this instead, once the seed has landed:
                // the same ordering, and the same reason, as the cold-start import.
                if let Some(pending) = &self.pending_starters {
                    pending.arm(request.starters);
                }
                match work_management_commands::new_work(&self.app_ctx, &request.dto) {
                    Ok(()) => close_form(ctx),
                    Err(e) => {
                        // Nothing was created, so nothing must stay armed: the next
                        // project made in this window would otherwise inherit a palette
                        // and a template set chosen for a project that never existed.
                        if let Some(pending) = &self.pending_starters {
                            pending.arm(crate::app::ProjectStarters::default());
                        }
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
                    dto: request.dto,
                    then_import: self.purpose == NewWorkPurpose::FromDocuments,
                    starters: request.starters,
                });
                ctx.open_window(config);
                if *close_presenting_window {
                    ctx.close_window();
                } else {
                    // A project window: it keeps its own project, so only the
                    // form goes away. Dismissing is not optional — the modal
                    // would otherwise stay up over a window that has just
                    // handed the user's request to a different one.
                    close_form(ctx);
                }
            }
        }
        true
    }
}

/// How [`NewWorkViewModel::commit`] closes the form.
#[derive(Clone, Copy)]
enum Dismiss {
    Modal,
    TopOverlay,
}

/// The Location field, looked at again while the wizard is on screen and it is
/// refused (see `shared::form_checks::retry_refusals`): a folder created,
/// mounted or made writable after its path was typed reopens Next without an
/// edit.
impl DiskChecked for NewWorkViewModel {
    fn disk_verdicts(&self) -> Vec<Signal<ValidationState>> {
        vec![self.location_validation(), self.target_check.signal()]
    }

    fn refused_on_disk(&self) -> bool {
        (!self.location.get().trim().is_empty() && self.location_check.refuses())
            || self.target_check.refuses()
    }

    /// Each check is looked at again only when it refuses itself. The folder's check
    /// writes a probe file into the folder, so looking at a folder it accepted, every
    /// second a folder sits at the target, would write into the writer's folder every
    /// second. The target sits in the folder, so it follows a refused folder.
    fn retry_refused(&self) {
        if !self.location.get().trim().is_empty() && self.location_check.refuses() {
            self.location_check.recheck();
            self.target_check.recheck();
        } else if self.target_check.refuses() {
            self.target_check.recheck();
        }
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
            "X".into(),
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
                "X".into(),
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

    /// **The typed name must reach the DTO as a title, not only as a path.** The form
    /// slugifies it to build `file_name`, and the backend used to read the title back out
    /// of that — so "The Long Road" became a project, a Book row and an exported title
    /// page all reading `the-long-road`, with nowhere in the app to correct it.
    #[test]
    fn the_typed_name_reaches_the_dto_as_a_title() {
        let vm = NewWorkViewModel::new(
            Rc::new(AppContext::new()),
            crate::app_ids::AppIds::new(),
            crate::app::PendingStarters::default(),
        );
        vm.location().set("/books".into());
        vm.name().set("The Long Road".into());

        let dto = vm.dto();
        assert_eq!(dto.file_name, "/books/the-long-road.skrib");
        assert_eq!(dto.title, "The Long Road");
    }

    /// Trimmed on the way out, so a field of spaces reads as unset and the backend falls
    /// back to the file stem rather than titling the book with whitespace.
    #[test]
    fn a_whitespace_only_name_arrives_as_no_title() {
        let vm = NewWorkViewModel::new(
            Rc::new(AppContext::new()),
            crate::app_ids::AppIds::new(),
            crate::app::PendingStarters::default(),
        );
        vm.location().set("/books".into());
        vm.name().set("  Tidewrack  ".into());
        assert_eq!(vm.dto().title, "Tidewrack");
    }

    /// Both starter answers travel together, so a project cannot get the palette it asked
    /// for beside the templates the *previous* project asked for.
    #[test]
    fn the_starters_carry_both_answers() {
        let vm = NewWorkViewModel::new(
            Rc::new(AppContext::new()),
            crate::app_ids::AppIds::new(),
            crate::app::PendingStarters::default(),
        );
        assert_eq!(vm.starters(), crate::app::ProjectStarters::default());

        vm.tag_preset().set(Some(crate::tags::Preset::Fantasy));
        vm.template_set()
            .set(Some(crate::note_templates::StarterSet::Essentials));
        let starters = vm.starters();
        assert_eq!(starters.tags, Some(crate::tags::Preset::Fantasy));
        assert_eq!(
            starters.templates,
            Some(crate::note_templates::StarterSet::Essentials)
        );
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
        assert_eq!(template_from_index(4), NewWorkTemplate::NovelInParts);
        assert_eq!(template_from_index(5), NewWorkTemplate::NoteBook);
        // Out-of-range falls back to the default (Novel).
        assert_eq!(template_from_index(99), NewWorkTemplate::Novel);
        assert_eq!(DEFAULT_TEMPLATE_INDEX, 3);
    }

    /// Every template is reachable from exactly one tile.
    ///
    /// The failure this guards is silent in both directions: an out-of-range index falls
    /// back to `Novel`, so a variant added without a tile is simply unreachable, and a
    /// tile added without a mapping quietly creates a second "Novel".
    #[test]
    fn every_template_is_reachable() {
        let seen: Vec<NewWorkTemplate> =
            (0..TEMPLATE_TILE_COUNT).map(template_from_index).collect();
        // Not `dedup`, which only collapses *adjacent* equals: the fallback arm maps
        // every unmapped index to `Novel`, and a stray one is rarely next to the real
        // Novel tile.
        for (i, t) in seen.iter().enumerate() {
            assert!(
                !seen[..i].contains(t),
                "tiles {} and {i} both map to {t:?}",
                seen[..i].iter().position(|s| s == t).unwrap()
            );
        }
        for wanted in [
            NewWorkTemplate::None,
            NewWorkTemplate::EmptyNovel,
            NewWorkTemplate::LightNovel,
            NewWorkTemplate::Novel,
            NewWorkTemplate::NovelInParts,
            NewWorkTemplate::NoteBook,
        ] {
            assert!(seen.contains(&wanted), "{wanted:?} has no tile");
        }
    }

    /// The tile list and `MANUSCRIPT_TEMPLATE_INDICES` must agree on which templates
    /// build a book: the flat-chapter toggle and the paratext picker are both gated on
    /// that range, and the parts template was added in the middle of it.
    #[test]
    fn every_index_that_builds_a_book_is_a_manuscript_index() {
        for idx in 0..TEMPLATE_TILE_COUNT {
            let builds_a_book = !matches!(
                template_from_index(idx),
                NewWorkTemplate::None | NewWorkTemplate::NoteBook
            );
            assert_eq!(
                MANUSCRIPT_TEMPLATE_INDICES.contains(&idx),
                builds_a_book,
                "tile {idx} disagrees with its template"
            );
        }
    }

    #[test]
    fn target_path_recomputes_on_change() {
        let vm = NewWorkViewModel::new(
            Rc::new(AppContext::new()),
            crate::app_ids::AppIds::new(),
            crate::app::PendingStarters::default(),
        );
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
        let vm = NewWorkViewModel::new(
            Rc::new(AppContext::new()),
            crate::app_ids::AppIds::new(),
            crate::app::PendingStarters::default(),
        );
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
        // The full positional list: seven original slots (one of them retired), the two
        // paratext folder names, then the two notes-folder names.
        assert_eq!(dto.labels.len(), 11);
        assert!(
            dto.labels[4].is_empty(),
            "slot 4 is retired and must stay a placeholder"
        );
        assert!(dto.chapter_scene_mode);
    }

    /// The wizard's one gate, in a real build: an untouched form cannot leave
    /// step one, and a usable name over a writable folder can.
    #[test]
    #[cfg(not(feature = "mocks"))]
    fn the_details_gate_needs_a_name_and_a_writable_folder() {
        let vm = NewWorkViewModel::new(
            Rc::new(AppContext::new()),
            crate::app_ids::AppIds::new(),
            crate::app::PendingStarters::default(),
        );
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

    /// The Location check writes a probe file into the folder, so it runs when
    /// the folder is chosen and never when the field or the gate is read. A
    /// folder removed behind the open wizard is still reported as it was, and
    /// Finish checks the disk again rather than creating into nowhere. The
    /// writer is on the last step by then, where the Location field is not on
    /// screen, so the refusal says why in a message of its own.
    #[test]
    #[cfg(not(feature = "mocks"))]
    fn the_location_is_checked_per_edit_and_again_at_finish() {
        use crate::test_support::press;
        use std::cell::Cell;
        use teksilo::core::styles::BannerSeverity;
        use teksilo::widgets::{NotificationArchiveModel, ToastInstallOptions, ToastRegistry};

        let dir = tempfile::tempdir().unwrap();
        let folder = dir.path().join("Books");
        std::fs::create_dir(&folder).unwrap();
        let vm = NewWorkViewModel::new(
            Rc::new(AppContext::new()),
            crate::app_ids::AppIds::new(),
            crate::app::PendingStarters::default(),
        );
        vm.name().set("Tidewrack".into());
        vm.location().set(folder.to_string_lossy().into_owned());
        let verdict = vm.location_validation();
        let gate = vm.can_create();
        assert!(matches!(verdict.get(), ValidationState::None));
        assert!(gate.get());

        std::fs::remove_dir(&folder).unwrap();
        for _ in 0..50 {
            assert!(matches!(verdict.get(), ValidationState::None));
            assert!(gate.get());
        }

        let created = Rc::new(Cell::new(true));
        let answer = created.clone();
        let finishing = vm.clone();
        let archive = Rc::new(NotificationArchiveModel::in_memory());
        let toasts = ToastRegistry::with_archive(
            ToastInstallOptions {
                archive: None,
                ..ToastInstallOptions::default()
            },
            archive.clone(),
        );
        let mut tree =
            crate::test_support::tree_with_toast_registry(&Rc::new(AppContext::new()), &toasts);
        press(&mut tree, move |c| answer.set(finishing.create(c, None)));
        assert!(
            !created.get(),
            "Finish must not create into a folder that is gone"
        );
        assert!(matches!(verdict.get(), ValidationState::Error(_)));
        assert!(!folder.exists(), "and nothing was created in its place");

        assert_eq!(toasts.live_count(), 1, "the writer is told why");
        let told = archive
            .entries()
            .with_item(0, |e| e.clone())
            .expect("the message is in the log");
        assert_eq!(told.severity, BannerSeverity::Error);
        let reason = tr!(new_work_location_missing()).resolve_now();
        assert!(
            told.title.contains(&reason),
            "the message names the Location's own reason: {:?}",
            told.title
        );
    }

    /// A New Work over a real, initialised backend, its form filled in with `name`
    /// in `folder`, and a tree to press its buttons in.
    #[cfg(not(feature = "mocks"))]
    fn filled_in(
        name: &str,
        folder: &std::path::Path,
    ) -> (
        NewWorkViewModel,
        Rc<AppContext>,
        teksilo::core::widget_tree::WidgetTree,
    ) {
        let app_ctx = Rc::new(AppContext::new());
        frontend::commands::handling_app_lifecycle_commands::initialize_app(&app_ctx)
            .expect("initialize the app");
        let vm = NewWorkViewModel::new(
            app_ctx.clone(),
            crate::app_ids::AppIds::new(),
            crate::app::PendingStarters::default(),
        );
        vm.location().set(folder.to_string_lossy().into_owned());
        vm.name().set(name.into());
        let tree = crate::test_support::tree_with_events(&app_ctx);
        (vm, app_ctx, tree)
    }

    /// The titles of the projects the backend holds.
    #[cfg(not(feature = "mocks"))]
    fn works(app_ctx: &AppContext) -> Vec<String> {
        frontend::commands::work_commands::get_all_work(app_ctx)
            .expect("get_all_work")
            .into_iter()
            .map(|work| work.title)
            .collect()
    }

    /// Answer the question `tree` has waiting with `button`.
    #[cfg(not(feature = "mocks"))]
    fn answer(
        tree: &mut teksilo::core::widget_tree::WidgetTree,
        question: teksilo::core::ModalRequest,
        button: StandardButton,
    ) {
        let teksilo::core::ModalContent::Deferred(builder) = question.content else {
            panic!("a MessageBox presents deferred content");
        };
        builder(tree);
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 600.0));
        let id = tree
            .find_by_label(&button.default_label().resolve_now())
            .expect("the question offers the button");
        crate::test_support::click(tree, id);
    }

    /// A project already at the target is named on the Name field, and Create asks
    /// before replacing it, as the importers do: Cancel creates nothing and leaves the
    /// wizard as it was, OK creates the project that was asked about. It used to be
    /// replaced without a word, the writer's novel and all.
    #[test]
    #[cfg(not(feature = "mocks"))]
    fn a_project_at_the_target_is_replaced_only_once_the_writer_says_so() {
        let _registry = crate::test_support::IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let existing = dir.path().join("tidewrack.skrib");
        std::fs::write(&existing, b"PK").unwrap();
        for button in [StandardButton::Cancel, StandardButton::Ok] {
            let (vm, app_ctx, mut tree) = filled_in("Tidewrack", dir.path());
            assert!(
                matches!(vm.name_validation().get(), ValidationState::Warning(_)),
                "the Name field says a project is already there"
            );
            assert!(vm.can_create().get(), "a warning, not a refusal");

            let wizard = StepperController::new(4);
            let (creating, controller) = (vm.clone(), wizard.clone());
            let created = Rc::new(std::cell::Cell::new(true));
            let answer_set = created.clone();
            crate::test_support::press(&mut tree, move |c| {
                answer_set.set(creating.create(c, Some(&controller)))
            });
            assert!(!created.get(), "the wizard waits for the answer");
            // What the Stepper does with a `false`.
            wizard.set_status(wizard.current(), StepStatus::Error);
            let question = tree
                .drain_pending_modal_requests()
                .pop()
                .expect("the writer is asked first")
                .request;
            assert_eq!(
                question.title,
                Some(tr!(new_work_overwrite_title()).resolve_now())
            );
            assert!(works(&app_ctx).is_empty(), "nothing before the answer");

            answer(&mut tree, question, button);
            if button == StandardButton::Cancel {
                assert!(works(&app_ctx).is_empty(), "Cancel creates nothing");
                assert_eq!(
                    wizard.status(wizard.current()),
                    StepStatus::Active,
                    "and leaves the wizard as it was"
                );
            } else {
                assert_eq!(works(&app_ctx), vec!["Tidewrack".to_string()]);
            }
        }
        assert_eq!(std::fs::read(&existing).unwrap(), b"PK");
    }

    /// A project open in a window is never replaced: Create refuses it with the reason,
    /// before asking anything, and again if the project is opened while the question
    /// waits.
    #[test]
    #[cfg(not(feature = "mocks"))]
    fn a_project_open_in_a_window_is_never_the_target() {
        use crate::shell::open_registry;
        let _registry = crate::test_support::IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let existing = dir.path().join("tidewrack.skrib");
        std::fs::write(&existing, b"PK").unwrap();
        let target = existing.to_string_lossy().into_owned();
        let refusal = tr!(new_work_target_open_title()).resolve_now();

        let (vm, app_ctx, mut tree) = filled_in("Tidewrack", dir.path());
        open_registry::claim(&target, "Tidewrack");
        let creating = vm.clone();
        crate::test_support::press(&mut tree, move |c| {
            assert!(!creating.create(c, None));
        });
        open_registry::release(&target);
        assert_eq!(
            crate::test_support::drain_dialog_titles(&mut tree),
            vec![refusal.clone()]
        );
        assert!(works(&app_ctx).is_empty());

        // Opened while the question waits: OK is answered with the refusal.
        let creating = vm.clone();
        crate::test_support::press(&mut tree, move |c| {
            assert!(!creating.create(c, None));
        });
        let question = tree
            .drain_pending_modal_requests()
            .pop()
            .expect("the question")
            .request;
        open_registry::claim(&target, "Tidewrack");
        answer(&mut tree, question, StandardButton::Ok);
        open_registry::release(&target);
        assert_eq!(
            crate::test_support::drain_dialog_titles(&mut tree),
            vec![refusal]
        );
        assert!(works(&app_ctx).is_empty(), "nothing is created over it");
        assert_eq!(std::fs::read(&existing).unwrap(), b"PK");
    }

    /// A project an import is still writing is not a target either: the import would
    /// replace the new project when it finished.
    #[test]
    #[cfg(not(feature = "mocks"))]
    fn a_project_an_import_is_writing_is_never_the_target() {
        use crate::shell::open_registry;
        let _registry = crate::test_support::IsolatedOpenRegistry::new();
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("tidewrack.skrib");
        let (vm, app_ctx, mut tree) = filled_in("Tidewrack", dir.path());
        let _claim = open_registry::claim_import(&target.to_string_lossy());
        let creating = vm.clone();
        crate::test_support::press(&mut tree, move |c| {
            assert!(!creating.create(c, None));
        });
        assert_eq!(
            crate::test_support::drain_dialog_titles(&mut tree),
            vec![tr!(target_importing_title()).resolve_now()]
        );
        assert!(works(&app_ctx).is_empty());
    }

    /// A folder at the target is refused on the Name field, which holds the wizard on
    /// its first step: replacing a folder project would leave its files in the new one.
    /// The format is part of the target, so switching it looks again.
    #[test]
    #[cfg(not(feature = "mocks"))]
    fn a_folder_at_the_target_is_refused_on_the_name() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("tidewrack")).unwrap();
        let (vm, _app_ctx, _tree) = filled_in("Tidewrack", dir.path());
        let (verdict, gate) = (vm.name_validation(), vm.can_create());
        assert!(matches!(verdict.get(), ValidationState::None));
        assert!(gate.get());

        vm.format_idx().set(1);
        assert!(matches!(verdict.get(), ValidationState::Error(_)));
        assert!(!gate.get(), "Next stays off over a folder");

        vm.format_idx().set(0);
        assert!(matches!(verdict.get(), ValidationState::None));
        assert!(gate.get());
    }

    /// While a folder sits at the target, the form looks at the target again every
    /// second, and only at the target: the folder's own check writes a probe file into
    /// the folder, and it accepted the folder. Before, each of those looks wrote and
    /// removed a file in the writer's folder, which a sync client or a removable drive
    /// sees. The folder is left untouched, its modification time with it.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn a_folder_at_the_target_is_looked_at_again_without_writing_to_the_folder() {
        use crate::shared::form_checks::DiskChecked;
        let dir = tempfile::tempdir().unwrap();
        let (vm, _app_ctx, _tree) = filled_in("Tidewrack", dir.path());
        vm.format_idx().set(1);
        std::fs::create_dir(dir.path().join("tidewrack")).unwrap();
        vm.format_idx().set(0);
        vm.format_idx().set(1);
        assert!(vm.refused_on_disk(), "a folder at the target is refused");
        let touched = || {
            std::fs::metadata(dir.path())
                .and_then(|m| m.modified())
                .ok()
        };
        let before = touched();
        std::thread::sleep(std::time::Duration::from_millis(20));
        for _ in 0..3 {
            vm.retry_refused();
        }
        assert_eq!(touched(), before, "nothing was written into the folder");
        assert!(
            vm.refused_on_disk(),
            "still refused while the folder is there"
        );

        std::fs::remove_dir(dir.path().join("tidewrack")).unwrap();
        vm.retry_refused();
        assert!(!vm.refused_on_disk(), "the target is looked at again");
    }

    /// Under `mocks` that gate is off, or an untouched wizard could not be
    /// walked past its first step without typing a name and pointing at a real
    /// writable folder — which is exactly what a backend-less build is for.
    #[test]
    #[cfg(feature = "mocks")]
    fn the_details_gate_is_off_under_mocks() {
        let vm = NewWorkViewModel::new(
            Rc::new(AppContext::new()),
            crate::app_ids::AppIds::new(),
            crate::app::PendingStarters::default(),
        );
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
        let vm = NewWorkViewModel::new(
            Rc::new(AppContext::new()),
            crate::app_ids::AppIds::new(),
            crate::app::PendingStarters::default(),
        )
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
        let vm = NewWorkViewModel::new(
            Rc::new(AppContext::new()),
            crate::app_ids::AppIds::new(),
            crate::app::PendingStarters::default(),
        );
        vm.paratext_preset().set(Some("us-trade-novel".into()));
        let vm = vm.for_documents();
        assert_eq!(vm.paratext_preset().get(), None);
    }

    #[test]
    fn chapter_scene_applies_only_to_manuscript_templates() {
        let vm = NewWorkViewModel::new(
            Rc::new(AppContext::new()),
            crate::app_ids::AppIds::new(),
            crate::app::PendingStarters::default(),
        );
        let applicable = vm.chapter_scene_applicable();
        // Manuscript templates (Empty Novel / Light Novel / Novel / Novel in parts).
        for idx in [1, 2, 3, 4] {
            vm.template_idx().set(idx);
            assert!(applicable.get(), "idx {idx} should enable the toggle");
        }
        // None (0) and Notebook (5) grey it out.
        for idx in [0, 5] {
            vm.template_idx().set(idx);
            assert!(!applicable.get(), "idx {idx} should disable the toggle");
        }
    }
}
