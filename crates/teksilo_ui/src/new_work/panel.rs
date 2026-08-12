// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The New Work modal — create a work, as a three-step wizard.
//!
//! Built on Teksilo's [`Stepper`], the same shape as
//! the Import documents wizard: **Details → Language & structure → Template**,
//! with the framework's indicator strip and Cancel / Back / Next / Create Work
//! footer. See `teksilo` docs `widgets/stepper.md`.
//!
//! The Launcher's **From documents…** reuses this same wizard in
//! [`NewWorkPurpose::FromDocuments`]: same first step, a language-only second
//! step, and a third step that replaces the template picker with what happens
//! next — the import wizard opening over the project this one creates. Its
//! template is pinned to None, because every template lays down a manuscript the
//! import would then be poured beside.
//!
//! That branch is **four declared steps**, the last two gated on the purpose
//! with `Step::visible_when` — the hidden one leaves navigation, the indicator
//! strip and the a11y tree, so either door is three pages long. Declaring both
//! keeps one builder and lets a test ask which step the flow actually reaches;
//! the alternative (a `match` producing two step lists) put the shared builder
//! inside the branch and made that question unaskable.
//!
//! The split follows what the writer must decide *before* anything can be
//! created versus what merely shapes the project:
//!   1. **Details** — name, author, format, where it lands. The only step with a
//!      gate: `complete_when(can_create)` keeps Next off until the name slugifies
//!      to something and the folder exists and is writable, so the two error
//!      states that can stop creation are met on the first page rather than at
//!      the end of the flow.
//!   2. **Language & structure** — the project's default writing language and the
//!      paratext tradition its book opens and closes with.
//!   3. **Template** — how much book to generate, and whether its chapters are
//!      flat. Finish (`Create Work`) commits.
//!
//! No step past the first carries a gate: every control there is a picker with a
//! valid default, so there is nothing left to be invalid about. Finish itself
//! can still refuse — `create` returns `false` when the project could not be
//! written, which holds the wizard on its last step and marks it in error rather
//! than closing over a project that does not exist.
//!
//! Enter advances (the `Stepper`'s default): step one is a form, and a focused
//! control that claims the key still wins.
//!
//! Under `--features mocks` that one gate is off and Finish creates nothing (see
//! [`NewWorkViewModel::can_create`] and `create`), so the whole flow can be
//! walked for layout work and automation without a backend — the same bypass the
//! import wizard carries.
//!
//! Each step body is a two-column [`FormLayout`] (auto-sized label column,
//! `Role::Form` a11y) inside a [`ScrollArea`]. All business logic lives on
//! [`NewWorkViewModel`]; this view is thin — it binds the VM's signals and hands
//! the stepper's Finish to its `create`.
//!
//! Format is two [`RadioTile`]s in a row; Template is a vertical
//! [`RadioTileGroup`] of compact rows (radio · icon · title · trailing count),
//! one per `NewWorkTemplate`. Location uses a [`FilePickerField`] with its
//! embedded browse affordance — no separate Browse button. The only
//! runtime-computed string is the "Will create `…/<slug>.skrib`" path preview
//! (`text` on the VM's derived signal); every other string is `tr!`-localized.

use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use teksilo::core::accesskit::Role;
use teksilo::core::styles::PanelVariant;
use teksilo::i18n::{LocalizedString, localized};
use teksilo::prelude::*;
use teksilo::res;
use teksilo::widgets::stepper::ChromePosition;
use teksilo::widgets::tooltip::TooltipContent;
use teksilo::widgets::{
    ComboBox, Divider, Expand, FilePickerField, FilePickerKind, FixedSize, FormLayout, HStack,
    IconWidget, Padding, Panel, RadioTile, RadioTileGroup, ScrollArea, Step, Stepper,
    StepperController, TextInput, TextWidget, TileLayout, Toggle, VStack,
};

use frontend::AppContext;

use crate::new_work::{NewWorkPurpose, NewWorkViewModel};

/// The card size. Wider and slightly shorter than the old single-column form:
/// the indicator strip and footer take the height the scrolling form used to,
/// and three shorter pages need less of it than one long one.
const CARD_W: f32 = 640.0;
const CARD_H: f32 = 620.0;

/// Steps **declared**, which is not how many a given wizard shows: the two last
/// ones are mutually exclusive, gated on [`NewWorkPurpose`] (see `build`).
const STEP_COUNT: usize = 4;

/// The declared step indices. The last two are the branch: only one is ever
/// visible, so the flow is always three pages long.
// Step ordinals the wizard's own step-order test names; the panel itself walks
// the steps rather than indexing them.
#[cfg(test)]
const STEP_TEMPLATE: usize = 2;
// See STEP_TEMPLATE.
#[cfg(test)]
const STEP_IMPORT: usize = 3;

pub struct NewWorkPanel {
    /// Owns the form's signals for this modal session (created once in `new`).
    vm: NewWorkViewModel,
    root_child: Option<WidgetId>,
    /// The Work-name field, captured during `build` and handed to the modal
    /// pipeline by [`Widget::initial_focus_hint`] — see the note there.
    name_field: std::cell::Cell<Option<WidgetId>>,
    /// Held rather than left to the `Stepper` so the flow can be driven and
    /// inspected from outside the widget — which step is current, which are
    /// reachable. The branch below is only observable through it.
    controller: StepperController,
}

impl NewWorkPanel {
    /// Presented over an already-open project (File ▸ New Work / Ctrl+N):
    /// creates the work in place, replacing this window's project. `ids` is
    /// THIS window's own `AppIds` — see [`NewWorkViewModel::new`]'s doc for why
    /// "Create Work" must close the outgoing Work through it.
    pub fn new(app_ctx: Rc<AppContext>, ids: crate::app_ids::AppIds) -> Self {
        Self {
            vm: NewWorkViewModel::new(app_ctx, ids),
            root_child: None,
            name_field: std::cell::Cell::new(None),
            controller: StepperController::new(STEP_COUNT),
        }
    }

    /// Presented from a project window whose Work is shared with a Work ▸ New
    /// Window sibling: the new project opens in its own window and this one
    /// keeps the project it is showing — see
    /// [`crate::new_work::NewWorkViewModel::new_beside_current`].
    pub fn new_beside_current(
        app_ctx: Rc<AppContext>,
        factory: crate::shell::windows::ProjectWindowFactory,
    ) -> Self {
        Self {
            vm: NewWorkViewModel::new_beside_current(app_ctx, factory),
            root_child: None,
            name_field: std::cell::Cell::new(None),
            controller: StepperController::new(STEP_COUNT),
        }
    }

    /// Presented from the Launcher (`WelcomeViewModel::new_work`): creation is
    /// deferred to a freshly-opened project window, which then closes the
    /// Launcher — see [`crate::new_work::NewWorkViewModel::new_for_launcher`].
    pub fn new_for_launcher(
        app_ctx: Rc<AppContext>,
        factory: crate::shell::windows::ProjectWindowFactory,
    ) -> Self {
        Self {
            vm: NewWorkViewModel::new_for_launcher(app_ctx, factory),
            root_child: None,
            name_field: std::cell::Cell::new(None),
            controller: StepperController::new(STEP_COUNT),
        }
    }

    /// [`Self::new_for_launcher`] for the Launcher's **From documents…**: the
    /// same wizard without the questions an import answers for itself — see
    /// [`crate::new_work::NewWorkViewModel::new_for_launcher_from_documents`].
    pub fn new_for_launcher_from_documents(
        app_ctx: Rc<AppContext>,
        factory: crate::shell::windows::ProjectWindowFactory,
    ) -> Self {
        Self {
            vm: NewWorkViewModel::new_for_launcher_from_documents(app_ctx, factory),
            root_child: None,
            name_field: std::cell::Cell::new(None),
            controller: StepperController::new(STEP_COUNT),
        }
    }

    /// The wizard's own title — the modal draws it, because `present_modal`
    /// ignores `ModalRequest::title` for an `InTree` presentation and a bare
    /// indicator strip does not say what flow you are in. It is also the one
    /// visible difference between the two doors before their third step.
    fn title(&self) -> LocalizedString {
        match self.vm.purpose() {
            NewWorkPurpose::Project => tr!(new_work_title()),
            NewWorkPurpose::FromDocuments => tr!(new_work_documents_title()),
        }
    }
}

/// A left-column field label (dimmed, small).
use crate::shared::text::{field_label, hint};

/// A 16 dp tile icon (monochrome, follows the theme text color).
fn tile_icon(icon: &'static teksilo::canvas::svg::SvgIcon) -> IconWidget {
    IconWidget::from_svg_icon(icon).icon_size(16.0)
}

/// The rich tooltip for the "write directly in chapters" toggle: a primary
/// body contrasting the two encodings, plus a `more` accordion with the
/// writing-model rationale. Inline content (no boot-time registry needed).
fn chapter_scene_tooltip() -> TooltipContent {
    TooltipContent::new("new-work-chapter-scene", tr!(new_work_chapter_scene_tip()))
        .with_more(tr!(new_work_chapter_scene_tip_more()))
}

/// The Words/Characters picker, over a local index kept in step with the view-model both
/// ways: the writer's choice latches it (so the language stops overriding), and a language
/// change re-seeds it while it has not been latched.
fn goal_unit_field(vm: &NewWorkViewModel) -> impl Widget + use<> {
    GoalUnitField {
        vm: vm.clone(),
        index: Signal::new(crate::goals::unit_picker::index_of(&vm.goal_unit().get())),
        root: None,
    }
}

/// The picker plus the two effects that keep it and the view-model agreeing.
///
/// A widget rather than two loose `ctx.effect` calls, because a `Step::content` factory has
/// no `BuildContext` — the effects have to be registered by something that gets one.
struct GoalUnitField {
    vm: NewWorkViewModel,
    index: Signal<usize>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for GoalUnitField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoalUnitField").finish()
    }
}

impl Widget for GoalUnitField {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // The language moved: re-seed, unless the writer has already spoken. The
        // view-model owns that decision — this only tells it the language changed.
        {
            let vm = self.vm.clone();
            let index = self.index.clone();
            ctx.effect(&self.vm.language(), move |_| {
                vm.language_changed();
                let seeded = crate::goals::unit_picker::index_of(&vm.goal_unit().get());
                if index.get() != seeded {
                    index.set(seeded);
                }
            });
        }
        // Wrapped rather than given a builder tooltip: `SegmentedControl` has none, and
        // the explanation is about the *choice*, not about either option.
        let id = ctx.add(crate::widgets::tip::RichTip::new(
            crate::tooltip_registry::GOAL_UNIT,
            FixedSize::new()
                .width(240.0)
                .child(crate::goals::unit_picker::goal_unit_control(
                    self.index.clone(),
                    {
                        // The writer moved the control: latch their choice, so the language
                        // stops overriding it from here on.
                        let vm = self.vm.clone();
                        move |unit, _ctx| vm.set_goal_unit(unit)
                    },
                )),
        ));
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// The project language dropdown, populated from the **dictionary registry** (the languages
/// a writer can spell-check in) — NOT the app's UI-translation locales, which are a
/// different, unrelated list (a French-UI user may well write an English novel). Bound to
/// the VM's language tag; a plain `ComboBox`, not `LanguageSwitcher` (which would change the
/// app's own UI locale). Display names are data → `localized(..)`, never `tr!`.
fn language_combo(vm: &NewWorkViewModel) -> ComboBox<String> {
    let entries = crate::spellcheck::dictionary_registry::entries();
    let tags: Vec<String> = entries.iter().map(|e| e.id.clone()).collect();
    let labels: HashMap<String, String> = entries
        .iter()
        .map(|e| (e.id.clone(), format!("{} ({})", e.display_name, e.id)))
        .collect();
    ComboBox::from_items(tags, vm.language(), move |tag: &String| {
        let display = labels.get(tag).cloned().unwrap_or_else(|| tag.clone());
        localized(move || display.clone())
    })
    .placeholder(tr!(new_work_language()))
}

/// The paratext-structure picker. Each preset names itself in its own language —
/// "Roman français" is not translated, because it is the name of a tradition, not a
/// label. Empty selection means no front or back matter, which is a first-class
/// answer and what a locale matching no tradition starts on.
fn paratext_combo(vm: &NewWorkViewModel) -> ComboBox<String> {
    let presets = vm.paratext_presets();
    let ids: Vec<String> = presets.iter().map(|p| p.id.clone()).collect();
    let names: HashMap<String, String> = presets
        .iter()
        .map(|p| (p.id.clone(), p.name.clone()))
        .collect();
    ComboBox::from_items(ids, vm.paratext_preset(), move |id: &String| {
        let display = names.get(id).cloned().unwrap_or_else(|| id.clone());
        localized(move || display.clone())
    })
    .placeholder(tr!(new_work_paratext_none()))
}

/// The reactive "Will create …" preview — the one runtime-computed string.
fn path_preview(vm: &NewWorkViewModel) -> impl Widget + use<> {
    HStack::new()
        .spacing(7.0)
        .child(
            TextWidget::new(tr!(new_work_will_create()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
        .child(
            // The target path lives on the VM as a derived `Signal<String>`;
            // this is the sole `lit!`/`text` runtime string.
            TextWidget::new(lit!(""))
                .text(vm.target_path())
                .style(TextStyleRole::Small)
                .color(TextRole::Accent),
        )
}

/// Wrap a step body in the scrolling, padded page every step shares.
fn step_page<W: Widget + 'static>(body: W) -> impl Widget + use<W> {
    ScrollArea::new().child(Padding::symmetric(4.0, 6.0).child(body))
}

/// Step one — name, author, format, destination: everything creation cannot
/// proceed without.
///
/// `start_dir` is resolved once by the panel (a `Step::content` factory has no
/// `BuildContext`, so `models::picker_starts_in` cannot be used here) so Browse
/// opens where the writer last put a project.
fn details_step(vm: &NewWorkViewModel, start_dir: Option<PathBuf>) -> impl Widget + use<> {
    let mut picker = FilePickerField::new(vm.location())
        .kind(FilePickerKind::PickFolder)
        .validation(vm.location_validation())
        .on_pick(|res, ctx| {
            crate::models::remember_pick(ctx, crate::models::FolderPurpose::NewProjectLocation, res)
        });
    if let Some(dir) = start_dir {
        picker = picker.starting_dir(dir);
    }

    step_page(
        FormLayout::new()
            .label(tr!(new_work_step_details()))
            .label_gap(16.0)
            .row_spacing(18.0)
            // ── Work name (required — inline error while blank) ────────────
            .line(
                field_label(tr!(new_work_name())),
                TextInput::new(vm.name())
                    .placeholder(tr!(new_work_name_placeholder()))
                    .validation(vm.name_validation()),
            )
            // ── Author (optional — no validation; blank is a normal state) ──
            .line(
                field_label(tr!(new_work_author())),
                TextInput::new(vm.author()).placeholder(tr!(new_work_author_placeholder())),
            )
            // ── Format: two selectable cards + the "convert later" note ────
            .line(
                field_label(tr!(new_work_format())),
                VStack::new()
                    .spacing(8.0)
                    .child(
                        RadioTileGroup::new(vm.format_idx())
                            .layout(TileLayout::Row)
                            .tile(
                                RadioTile::new()
                                    .icon(tile_icon(res!("assets/icons/new_work/single-file.svg")))
                                    .title(tr!(new_work_single_file()))
                                    .description(tr!(new_work_single_file_desc())),
                            )
                            .tile(
                                RadioTile::new()
                                    .icon(tile_icon(res!("assets/icons/new_work/bundle.svg")))
                                    .title(tr!(new_work_bundle()))
                                    .description(tr!(new_work_bundle_desc())),
                            ),
                    )
                    .child(hint(tr!(new_work_convert_later()))),
            )
            // ── Location (FilePicker, browse embedded) + path preview ──────
            // Same inline error indicator as the name field: the folder must
            // exist and be writable.
            .line(
                field_label(tr!(new_work_location())),
                VStack::new()
                    .spacing(6.0)
                    .child(picker)
                    .child(path_preview(vm)),
            ),
    )
}

/// Step two — the project's default writing language, and (for an ordinary
/// project) its paratext tradition.
///
/// The paratext picker greys out for the templates that build no book (None,
/// Notebook). That choice is made on the *next* step, so the control can turn
/// inapplicable behind the writer's back — greyed rather than hidden, so at
/// worst they come back and find the answer they gave no longer applies, never
/// a control that has vanished.
///
/// It is absent altogether in [`NewWorkPurpose::FromDocuments`], whose template
/// is pinned to None: paratexts are only ever built around a Book row, so the
/// picker there could not do anything at all — and a permanently greyed control
/// is worse than no control.
fn language_step(vm: &NewWorkViewModel) -> impl Widget + use<> {
    let form = FormLayout::new()
        .label(tr!(new_work_step_language()))
        .label_gap(16.0)
        .row_spacing(18.0)
        // ── Default language ──────────────────────────────────────────────
        .line(
            field_label(tr!(new_work_language())),
            VStack::new()
                .spacing(6.0)
                .child(FixedSize::new().width(240.0).child(language_combo(vm)))
                .child(hint(tr!(new_work_language_hint()))),
        )
        // ── Counting unit ────────────────────────────────────────────────
        //
        // Beside the language, not over on the template step with the chapter
        // toggle, because it is *derived* from the language: a control whose
        // default moves has to be where the writer can see it move. Picking
        // Japanese and then finding, two steps later, that the unit had quietly
        // become characters would be the same information delivered as a surprise.
        .line(
            field_label(tr!(new_work_goal_unit())),
            VStack::new()
                .spacing(6.0)
                .child(goal_unit_field(vm))
                .child(hint(tr!(new_work_goal_unit_hint()))),
        );

    let form = match vm.purpose() {
        NewWorkPurpose::FromDocuments => form,
        // ── Paratext structure: the front and back matter a tradition opens and
        // closes a book with. Orthogonal to the template — how much book, and
        // which tradition, are two questions. ─────────────────────────────
        NewWorkPurpose::Project => form.full_width(Divider::new()).line(
            field_label(tr!(new_work_paratext())),
            VStack::new()
                .spacing(6.0)
                .child(
                    FixedSize::new()
                        .width(240.0)
                        .child(paratext_combo(vm).enabled(vm.paratext_applicable())),
                )
                .child(hint(tr!(new_work_paratext_hint()))),
        ),
    };
    step_page(form)
}

/// Step three — how much book to generate, and whether its chapters are flat.
/// Finish commits from here.
fn template_step(vm: &NewWorkViewModel) -> impl Widget + use<> {
    step_page(
        FormLayout::new()
            .label(tr!(new_work_step_template()))
            .label_gap(16.0)
            .row_spacing(18.0)
            // ── Template: a vertical list of compact rows (radio · icon ·
            // title · trailing count), one per NewWorkTemplate. ────────────
            .line(
                field_label(tr!(new_work_template())),
                RadioTileGroup::new(vm.template_idx())
                    .layout(TileLayout::Vertical)
                    .tile(
                        RadioTile::new()
                            .icon(tile_icon(res!("assets/icons/new_work/none.svg")))
                            .title(tr!(new_work_template_none()))
                            .trailing(tr!(new_work_template_none_count())),
                    )
                    .tile(
                        RadioTile::new()
                            .icon(tile_icon(res!("assets/icons/new_work/empty-novel.svg")))
                            .title(tr!(new_work_template_empty_novel()))
                            .trailing(tr!(new_work_template_empty_novel_count())),
                    )
                    .tile(
                        RadioTile::new()
                            .icon(tile_icon(res!("assets/icons/new_work/light-novel.svg")))
                            .title(tr!(new_work_template_light_novel()))
                            .trailing(tr!(new_work_template_light_novel_count())),
                    )
                    .tile(
                        RadioTile::new()
                            .icon(tile_icon(res!("assets/icons/new_work/novel.svg")))
                            .title(tr!(new_work_template_novel()))
                            .trailing(tr!(new_work_template_novel_count())),
                    )
                    .tile(
                        RadioTile::new()
                            .icon(tile_icon(res!("assets/icons/new_work/notebook.svg")))
                            .title(tr!(new_work_template_notebook()))
                            .trailing(tr!(new_work_template_notebook_count())),
                    ),
            )
            // ── ChapterScene mode: write directly in chapters (novel templates
            // only; greyed otherwise). The rich tooltip explains both modes. ──
            .full_width(Divider::new())
            .full_width(
                Toggle::new(vm.chapter_scene())
                    .label(tr!(new_work_chapter_scene()))
                    .enabled(vm.chapter_scene_applicable())
                    .rich_tooltip_content(chapter_scene_tooltip()),
            ),
    )
}

/// Step three for [`NewWorkPurpose::FromDocuments`] — what happens after
/// "Create & import", in place of the template picker.
///
/// The page exists because the flow has a seam the writer would otherwise walk
/// into blind: creating the project and importing into it are two operations,
/// and the second one opens a wizard of its own the moment the first finishes.
/// Saying so here is the difference between "why is it asking me for files
/// again?" and "right, that is the part I was told about".
///
/// It keeps the flat-chapters toggle: that is `Work.chapter_mode`, which every
/// chapter the import creates is resolved through, so it is the one structural
/// question this form must still ask.
fn import_next_step(vm: &NewWorkViewModel) -> impl Widget + use<> {
    step_page(
        VStack::new()
            .spacing(12.0)
            .child(
                TextWidget::new(tr!(new_work_documents_next_title()))
                    .style(TextStyleRole::BodyBold),
            )
            .child(TextWidget::new(tr!(new_work_documents_next_body())))
            .child(
                TextWidget::new(tr!(new_work_documents_no_template()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .child(Divider::new())
            .child(
                Toggle::new(vm.chapter_scene())
                    .label(tr!(new_work_chapter_scene()))
                    .enabled(vm.chapter_scene_applicable())
                    .rich_tooltip_content(chapter_scene_tooltip()),
            )
            .child(
                TextWidget::new(tr!(new_work_documents_chapter_scene_hint()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            ),
    )
}

impl std::fmt::Debug for NewWorkPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewWorkPanel").finish()
    }
}

impl Widget for NewWorkPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // `Step::content` is a factory with no `BuildContext`, so the Browse
        // start directory is resolved once here and captured into the step.
        let start_dir = ctx
            .app_state::<crate::models::FolderMemoryService>()
            .and_then(|svc| svc.last(crate::models::FolderPurpose::NewProjectLocation));

        let details_vm = self.vm.clone();
        let language_vm = self.vm.clone();
        let template_vm = self.vm.clone();
        let import_vm = self.vm.clone();
        let create_vm = self.vm.clone();
        let purpose = self.vm.purpose();

        // The last step and the button that leaves it are the whole difference
        // between the two doors: an ordinary project chooses a template and
        // Creates; a from-documents project is told what comes next and Creates
        // & imports.
        let finish_label = match purpose {
            NewWorkPurpose::Project => tr!(new_work_create()),
            NewWorkPurpose::FromDocuments => tr!(new_work_create_and_import()),
        };

        // Both last steps are **declared**, each gated on the purpose that
        // selects it (`Step::visible_when`): the hidden one drops out of
        // navigation, out of the indicator strip and out of the a11y tree, and
        // the flow ends on whichever survives. Two `.step(..)` arms in a `match`
        // was the old shape, and it put the shared builder inside the branch.
        // Not `content_boxed` on one step: the *title* differs too ("Template"
        // vs "Import"), and a title is not reactive.
        let is_project = purpose == NewWorkPurpose::Project;

        let stepper = Stepper::new()
            .controller(self.controller.clone())
            .back_label(tr!(new_work_back()))
            .next_label(tr!(new_work_next()))
            .finish_label(finish_label)
            .cancel(tr!(new_work_cancel()), |ctx, _ctrl| ctx.dismiss_modal())
            .step(
                Step::new(tr!(new_work_step_details()))
                    .content(move || details_step(&details_vm, start_dir.clone()))
                    // The only gate in the flow: a name that slugifies to
                    // something, and a folder that exists and is writable.
                    .complete_when(self.vm.can_create()),
            )
            .step(
                Step::new(tr!(new_work_step_language()))
                    .content(move || language_step(&language_vm)),
            )
            .step(
                Step::new(tr!(new_work_step_template()))
                    .visible_when(is_project)
                    .content(move || template_step(&template_vm)),
            )
            .step(
                Step::new(tr!(new_work_step_import()))
                    .visible_when(!is_project)
                    .content(move || import_next_step(&import_vm)),
            )
            // Create Work. `create` dismisses on success; on failure it toasts
            // and returns `false`, which keeps the wizard on its last step and
            // marks that step in error rather than reporting a flow that
            // finished when nothing was created. From documents, the import
            // wizard opens over the new project the moment it exists
            // (`PendingAction::New`'s `then_import`).
            .on_finish(move |ctx, _ctrl| create_vm.create(ctx));

        // Which flow this is, in the Stepper's own chrome slot. `Top` is not the
        // default — chrome is QWizard's watermark slot, so it lands in a leading
        // sidebar column unless told otherwise, and a one-line title there is a
        // 230 dp margin holding four words. The strip names the steps but not
        // the wizard, and "From documents…" differs from plain New Work in ways
        // the writer should be able to see before its last page.
        let stepper = stepper
            .chrome(TextWidget::new(self.title()).style(TextStyleRole::BodyBold))
            .chrome_position(ChromePosition::Top);

        // Add the stepper first so its first focusable descendant — the Work
        // name field on step one — can be captured for `initial_focus_hint`
        // (the chrome banner is a label, so it does not take the slot).
        let stepper_id = ctx.add(stepper);
        self.name_field
            .set(ctx.first_focusable_descendant(stepper_id));

        let root = teksu!(ctx => FixedSize {
                width: CARD_W
                height: CARD_H
                Panel {
                    variant: PanelVariant::Raised
                    corner_radius: 10.0
                    padding: 12.0
                    Expand::vertical {
                        child_id: stepper_id
                    }
                }
            }
        );
        self.root_child = Some(root);
        vec![root]
    }

    /// Announce the wizard as a named dialog.
    ///
    /// `ctx.present_modal` does not wrap a hand-drawn panel in a
    /// `ModalContainer`, so nothing else here would emit a `Role::Dialog` node —
    /// the same gap the Import documents wizard closes this way.
    fn accessibility(&self, builder: &mut teksilo::core::accessibility::AccessNodeBuilder) {
        builder.set_role(Role::Dialog);
        builder.set_name(self.title().resolve_now());
    }

    /// Open with the Work name field focused, so the dialog is typeable the moment
    /// it appears.
    ///
    /// The linear indicator strip is not focusable and the footer is built after
    /// the content, so the modal pipeline's own fallback would land here anyway —
    /// but only by accident of tree order. Pinning it keeps the first keystroke
    /// going into the name field whatever the stepper's chrome grows into.
    fn initial_focus_hint(&self) -> Option<WidgetId> {
        self.name_field.get()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Delegate to the fixed-size root (bounds the greedy inner `Expand`s);
        // delegating to the inner `Panel` would fill the window — the
        // modal-centering trap noted in `WelcomePanel`.
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::widget_tree::WidgetTree;

    /// The whole panel — the stepper's indicator strip, the first step's
    /// `FormLayout` body with its `RadioTileGroup` / `FilePickerField`, and the
    /// footer — must build and lay out headlessly without panicking. This
    /// exercises the full widget tree (any wrong builder/DSL usage would panic
    /// here), and the panel's own `layout_response` must report the fixed card
    /// size the modal host uses to size/centre it (not zero, not a stretch).
    #[test]
    fn panel_builds_and_lays_out() {
        let ctx = Rc::new(AppContext::new());
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(NewWorkPanel::new(
            ctx,
            crate::app_ids::AppIds::new(),
        )));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        let b = tree.bounds(id);
        assert_eq!(
            (b.width, b.height),
            (CARD_W, CARD_H),
            "panel fills the card"
        );
    }

    /// Every step body must build and lay out on its own — the stepper only
    /// builds the active one, so a broken step two or three would otherwise sit
    /// undetected until a writer clicked Next.
    #[test]
    fn every_step_builds_and_lays_out() {
        let vm = NewWorkViewModel::new(Rc::new(AppContext::new()), crate::app_ids::AppIds::new());
        let documents =
            NewWorkViewModel::new(Rc::new(AppContext::new()), crate::app_ids::AppIds::new())
                .for_documents();
        for (name, page) in [
            (
                "details",
                Box::new(details_step(&vm, None)) as Box<dyn Widget>,
            ),
            ("language", Box::new(language_step(&vm))),
            ("template", Box::new(template_step(&vm))),
            // The from-documents variants: a language step with no paratext row,
            // and the "what happens next" page in place of the templates.
            ("language/documents", Box::new(language_step(&documents))),
            ("import-next", Box::new(import_next_step(&documents))),
        ] {
            let mut tree = WidgetTree::new();
            let id = tree.add_boxed(page);
            tree.layout(SizeProposal::exact(CARD_W, CARD_H));
            let b = tree.bounds(id);
            assert!(
                b.width > 0.0 && b.height > 0.0,
                "step {name} laid out to nothing"
            );
        }
    }

    /// The from-documents wizard is a different set of steps and a different
    /// finish label, assembled in `build` — so it gets its own build/layout pass
    /// rather than being assumed to follow from the ordinary one.
    #[test]
    fn the_from_documents_panel_builds_and_lays_out() {
        let (_ctrl, tree, id) = mounted(true);
        let b = tree.bounds(id);
        assert_eq!((b.width, b.height), (CARD_W, CARD_H));
    }

    /// Mount a wizard of either purpose and hand back its controller.
    fn mounted(from_documents: bool) -> (StepperController, WidgetTree, WidgetId) {
        let mut panel =
            NewWorkPanel::new(Rc::new(AppContext::new()), crate::app_ids::AppIds::new());
        if from_documents {
            panel.vm = panel.vm.clone().for_documents();
        }
        let controller = panel.controller.clone();
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(panel));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        (controller, tree, id)
    }

    /// Both last steps are declared; only the one this door needs is reachable.
    ///
    /// The wizard is three pages long either way — walking it must never land on
    /// the branch that does not apply, and Back must not find it either. (The
    /// two used to be separate step *lists*, which made "is the other one
    /// reachable?" an unaskable question.)
    #[test]
    fn only_the_last_step_this_purpose_needs_is_reachable() {
        for (from_documents, wanted, hidden) in [
            (false, STEP_TEMPLATE, STEP_IMPORT),
            (true, STEP_IMPORT, STEP_TEMPLATE),
        ] {
            let (ctrl, _tree, _id) = mounted(from_documents);
            assert_eq!(ctrl.current(), 0, "wizards open on Details");

            ctrl.next();
            assert_eq!(ctrl.current(), 1, "then Language & structure");

            ctrl.next();
            assert_eq!(
                ctrl.current(),
                wanted,
                "from_documents={from_documents}: the flow must end on its own last step"
            );
            assert_ne!(ctrl.current(), hidden, "and never on the hidden one");

            // Back retraces the pages actually visited — the hidden step is not
            // one of them.
            ctrl.back();
            assert_eq!(ctrl.current(), 1);
        }
    }
}
