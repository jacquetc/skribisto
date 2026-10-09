// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Import a Scrivener project** wizard: **Project → Review → Save** (D7-3).
//!
//! Built on Teksilo's `Stepper`, the document wizard's shape, so the app's two review-first
//! importers read alike. The Project page reads the project the moment one is chosen and
//! shows what it found; the Review page shows the binder the import will make, with the
//! choices that change it beside it; the Save page names the new project. Nothing is
//! written until **Import**, which hands over to the shared project-import job and its
//! notice.
//!
//! Thin, per the house rules: it binds [`ImportScrivenerViewModel`]'s signals and models
//! and calls its methods. Every judgement lives there. Lists are teksilo-data widgets: a
//! `Repeater` for the few, unvirtualized rows of the card and the choices (each carries a
//! live control), a `TreeTableView` for the preview, which a 2,500-item project needs
//! virtualized.

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use teksilo::core::accesskit::{Live, Role};
use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::{ComboBoxVariant, PanelVariant};
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::{
    Banner, Button, ButtonVariant, CellContext, Checkbox, Column, ColumnWidth, ComboBox, DropZone,
    Expand, FilePickerField, FilePickerKind, FixedSize, FormLayout, HStack, MaxSize, MessageBox,
    MessageBoxButtons, Padding, Panel, ProgressBar, RadioButton, RadioGroup, Repeater, ScrollArea,
    Spacer, Step, Stepper, Switcher, TabTraversal, TextInput, TextWidget, TreeTableView, VStack,
};

use super::import_scrivener_vm::{
    HELP_TOPIC, ImportScrivenerViewModel, NoticeView, QuestionView, RefusalView, ReviewControl,
    ReviewRow,
};
use super::preview_source::PreviewRowView;
use crate::shared::form_checks::{RETRY_REFUSED_AFTER, retry_refusals};

/// The card's preferred size. Preferred, not fixed: the Launcher is a fixed 820 × 590
/// and a project window may be 800 × 600, so the card shrinks into the window it is
/// opened over ([`card_size`]) rather than losing its footer, Import included.
const CARD_W: f32 = 920.0;
const CARD_H: f32 = 620.0;
/// The floor the card does not shrink past: below it the Review page's two columns stop
/// being two columns.
const MIN_CARD_W: f32 = 640.0;
const MIN_CARD_H: f32 = 420.0;
/// The gutter left between a shrunk card and the window's edge.
const VIEWPORT_MARGIN: f32 = 24.0;

/// The card's size in a slot of `slot_w` × `slot_h`: the preferred size when it fits,
/// else the slot less the gutter, never under the floor nor over the slot.
fn card_size(slot_w: f32, slot_h: f32) -> (f32, f32) {
    fn fit(slot: f32, preferred: f32, floor: f32) -> f32 {
        if slot >= preferred {
            preferred
        } else {
            (slot - VIEWPORT_MARGIN).max(floor).min(slot)
        }
    }
    (
        fit(slot_w, CARD_W, MIN_CARD_W),
        fit(slot_h, CARD_H, MIN_CARD_H),
    )
}

/// The Review page's left column: the choices.
const CHOICES_W: f32 = 340.0;

/// The "before you import" strip's cap: its heading and about four lines, after which it
/// scrolls.
const STRIP_HEIGHT: f32 = 104.0;

/// Open the wizard over the window that asked for it, with `vm` (that window's own)
/// emptied first. With `source`, the wizard opens already reading it, with what the writer
/// chose for the import that failed on it: **Review again** after an import that found the
/// project changed.
pub fn present_import_scrivener(
    ctx: &mut EventContext,
    vm: ImportScrivenerViewModel,
    source: Option<String>,
) {
    vm.reset();
    vm.set_remembered_folder(
        ctx.app_state::<crate::models::FolderMemoryService>()
            .and_then(|svc| svc.last(crate::models::FolderPurpose::NewProjectLocation)),
    );
    if let Some(source) = source {
        vm.review_again(&source);
    }
    ctx.present_modal(
        ModalRequest::deferred(move |t| t.add(ImportScrivenerPanel::new(vm.clone())))
            .presentation(ModalPresentation::InTree)
            .close_behavior(ModalCloseBehavior::Manual)
            .title(tr!(import_scrivener_title()))
            .size(CARD_W as u32, CARD_H as u32),
    );
}

/// Where the Project page leaves the path field's id once it is built: the
/// `TextInput`'s own cell, which it fills when it mounts.
type PathFieldSlot = Rc<RefCell<Option<Rc<std::cell::Cell<Option<WidgetId>>>>>>;

pub struct ImportScrivenerPanel {
    vm: ImportScrivenerViewModel,
    root_child: Option<WidgetId>,
    /// The card's live size, written where the panel learns the room it has
    /// ([`Widget::layout_response`]) and read by the card's `FixedSize` in the same pass.
    card_w: Signal<f32>,
    card_h: Signal<f32>,
    /// The path field's id, filled when the Project page is built: the field the wizard
    /// opens focused on (see [`Widget::initial_focus_hint`]).
    path_field: PathFieldSlot,
}

impl ImportScrivenerPanel {
    pub fn new(vm: ImportScrivenerViewModel) -> Self {
        Self {
            vm,
            root_child: None,
            card_w: Signal::new(CARD_W),
            card_h: Signal::new(CARD_H),
            path_field: Rc::new(RefCell::new(None)),
        }
    }
}

impl std::fmt::Debug for ImportScrivenerPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportScrivenerPanel").finish()
    }
}

impl Widget for ImportScrivenerPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // A refused path or folder is looked at again while the wizard is up, so a drive
        // mounted or a folder created afterwards reopens the way without an edit.
        retry_refusals(ctx, self.vm.clone(), RETRY_REFUSED_AFTER);

        let project_vm = self.vm.clone();
        let review_vm = self.vm.clone();
        let save_vm = self.vm.clone();
        let finish_vm = self.vm.clone();
        let cancel_vm = self.vm.clone();
        let path_field = self.path_field.clone();

        // `Stepper` stays a builder: its steps are an accumulator of generic closures.
        let stepper = Stepper::new()
            .controller(self.vm.controller())
            // Enter belongs to the control that has it: a list picks its answer, the path
            // field reads its project. The Stepper acts on the key's release, which a
            // control that took the press lets through, so Enter picking a language
            // would start the import and one picking a choice would leave the Review.
            .enter_advances(false)
            .back_label(tr!(import_scrivener_back()))
            .next_label(tr!(import_scrivener_next()))
            .finish_label(tr!(import_scrivener_import()))
            .help(tr!(import_scrivener_help()), |c, _| {
                crate::help::window::open_or_focus_help(c, Some(HELP_TOPIC))
            })
            .cancel(tr!(import_scrivener_cancel()), move |c, _| {
                cancel_vm.cancel_or_dismiss(c)
            })
            .step(
                Step::new(tr!(import_scrivener_step_project()))
                    .content(move || project_step(&project_vm, &path_field))
                    .complete_when(self.vm.can_leave_project_step()),
            )
            .step(
                Step::new(tr!(import_scrivener_step_review()))
                    .content(move || review_step(&review_vm))
                    .complete_when(self.vm.can_leave_review_step()),
            )
            .step(
                Step::new(tr!(import_scrivener_step_save()))
                    .content(move || save_step(&save_vm))
                    .complete_when(self.vm.can_import()),
            )
            // Import hands over to the shared job, which closes the wizard when the import
            // starts. `false` keeps it open on Save, with the fields saying why.
            .on_finish(move |c, _| finish_vm.import(c));

        let root = teksu!(ctx => FixedSize {
            width: self.card_w.clone()
            height: self.card_h.clone()
            Panel {
                variant: PanelVariant::Raised
                corner_radius: 10.0
                padding: 12.0
                Expand::vertical {
                    child: stepper
                }
            }
        });
        self.root_child = Some(root);
        vec![root]
    }

    /// Open with the path field focused, so the wizard is typeable the moment it appears.
    fn initial_focus_hint(&self) -> Option<WidgetId> {
        self.path_field
            .borrow()
            .as_ref()
            .and_then(|field| field.get())
    }

    /// Announce the wizard as a named dialog: `present_modal` does not wrap a hand-drawn
    /// panel in a container that would.
    fn accessibility(&self, builder: &mut teksilo::core::accessibility::AccessNodeBuilder) {
        builder.set_role(Role::Dialog);
        builder.set_name(tr!(import_scrivener_title()).resolve_now());
    }

    /// Always the preferred size, so the slot the overlay hands back is
    /// `min(preferred, window)` on every pass; the card then takes what fits of it.
    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        if let (Some(slot_w), Some(slot_h)) = (proposal.width, proposal.height) {
            let (w, h) = card_size(slot_w, slot_h);
            self.card_w.set_if_changed(w);
            self.card_h.set_if_changed(h);
        }
        Size::new(CARD_W, CARD_H).into()
    }

    /// Centre the card in its slot, at the size that fits.
    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [teksilo::core::widget::WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        let (w, h) = card_size(bounds.width, bounds.height);
        let origin = Point::new(
            bounds.x + ((bounds.width - w) / 2.0).max(0.0),
            bounds.y + ((bounds.height - h) / 2.0).max(0.0),
        );
        for child in children.iter_mut() {
            child.origin = origin;
            child.size = Size::new(w, h);
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

// ── Project ──────────────────────────────────────────────────────────────────────────────

/// Step one: choose the project, and see what it holds.
fn project_step(vm: &ImportScrivenerViewModel, path_field: &PathFieldSlot) -> impl Widget + use<> {
    let drop_vm = vm.clone();
    let submit_vm = vm.clone();
    let blur_vm = vm.clone();
    let file_vm = vm.clone();
    let folder_vm = vm.clone();
    let input = TextInput::new(vm.source_text())
        .label(tr!(import_scrivener_source()))
        .placeholder(tr!(import_scrivener_source_placeholder()))
        .validation(vm.source_validation())
        .on_submit_fn(move |_| submit_vm.submit_source())
        // Leaving the field reads what was typed; the same project is not read twice.
        .on_blur_fn(move |_| blur_vm.submit_source());
    *path_field.borrow_mut() = Some(input.field_id());

    teksu!(Padding::symmetric(16.0, 18.0) {
        VStack {
            spacing: 10.0
            // `Expand` above the height pin: a height-only `FixedSize` proposes no width,
            // and a drop zone measured against none hugs its own text.
            Expand::horizontal {
                FixedSize {
                    height: 76.0
                    DropZone::new(tr!(import_scrivener_drop_title())) {
                        subtitle: tr!(import_scrivener_drop_hint())
                        // The two buttons below are the keyboard route, with titles in
                        // the writer's language and a folder picker the zone lacks.
                        show_browse_button: false
                        allow_multiple: false
                        on_files_dropped: move |paths: Vec<PathBuf>, _c: &mut EventContext| {
                            drop_vm.accept_dropped(&paths)
                        }
                    }
                }
            }
            HStack {
                spacing: 8.0
                Expand::horizontal {
                    child: input
                }
                Button::new(tr!(import_scrivener_choose_file())) {
                    variant: ButtonVariant::Plain
                    on_activate_fn: move |c| file_vm.pick_file(c)
                }
                Button::new(tr!(import_scrivener_choose_folder())) {
                    variant: ButtonVariant::Plain
                    on_activate_fn: move |c| folder_vm.pick_folder(c)
                }
            }
            TextWidget::new(vm.source_hint()) {
                style: TextStyleRole::Small
                color: TextRole::Secondary
            }
            Expand::vertical {
                Switcher::new(vm.inspection_page()) {
                    child: empty_hint()
                    child: reading_body(vm)
                    child: project_card(vm)
                    child: refusal_body(vm)
                    child: failure_body(vm)
                }
            }
        }
    })
}

fn empty_hint() -> impl Widget + use<> {
    teksu!(Padding::symmetric(8.0, 24.0) {
        TextWidget::new(tr!(import_scrivener_empty_hint())) {
            style: TextStyleRole::Small
            color: TextRole::Secondary
        }
    })
}

/// While the project is read: what is happening, how far, and **Stop reading**.
fn reading_body(vm: &ImportScrivenerViewModel) -> impl Widget + use<> {
    let stop_vm = vm.clone();
    teksu!(Padding::symmetric(40.0, 24.0) {
        VStack {
            spacing: 12.0
            TextWidget::new(tr!(import_scrivener_reading())) {
                style: TextStyleRole::Body
                color: TextRole::Primary
                access_live: Live::Polite
            }
            ProgressBar::new(0.0) {
                value: vm.inspect_progress()
                thickness: 6.0
                label: tr!(import_scrivener_reading())
            }
            HStack {
                Spacer
                Button::new(tr!(import_scrivener_stop_reading())) {
                    variant: ButtonVariant::Plain
                    on_activate_fn: move |_| stop_vm.stop_reading()
                }
            }
        }
    })
}

/// What the project is, then the questions on it and what to know about it.
fn project_card(vm: &ImportScrivenerViewModel) -> impl Widget + use<> {
    teksu!(ScrollArea {
        Padding::symmetric(4.0, 6.0) {
            VStack {
                spacing: 12.0
                // Empty until a project reads: a question asked before anything could be
                // read has no card above it.
                Switcher::new(vm.has_card()) {
                    Spacer
                    VStack {
                        spacing: 4.0
                        TextWidget::new(lit!("")) {
                            text: vm.card_name()
                            style: TextStyleRole::BodyBold
                            color: TextRole::Primary
                            // The first line a screen reader hears once the project is read.
                            access_live: Live::Polite
                        }
                        TextWidget::new(lit!("")) {
                            text: vm.card_made_with()
                            style: TextStyleRole::Small
                            color: TextRole::Secondary
                        }
                        TextWidget::new(lit!("")) {
                            text: vm.card_saved_on()
                            style: TextStyleRole::Small
                            color: TextRole::Secondary
                        }
                        TextWidget::new(lit!("")) {
                            text: vm.card_counts()
                            style: TextStyleRole::Small
                            color: TextRole::Secondary
                        }
                    }
                }
                child: questions(vm)
                Repeater::new(vm.notices(), |n: &NoticeView| Box::new(notice_banner(n)) as Box<dyn Widget>) {
                    spacing: 8.0
                }
            }
        }
    })
}

/// A notice as a banner: a warning, or something to know.
fn notice_banner(notice: &NoticeView) -> Banner {
    let banner = if notice.warning {
        Banner::warning(notice.title.clone())
    } else {
        Banner::info(notice.title.clone())
    };
    match &notice.body {
        Some(body) => banner.description(body.clone()),
        None => banner,
    }
}

/// A question: a radio group to choose with, or a warning with the checkbox that accepts
/// it. A disabled Next is always explained by one of these, on this page.
fn question(vm: &ImportScrivenerViewModel, q: &QuestionView) -> Box<dyn Widget> {
    match &q.accept {
        Some(accept) => {
            let accept_vm = vm.clone();
            let key = q.key.clone();
            let banner = Banner::warning(q.title.clone()).description(q.body.clone());
            Box::new(teksu!(VStack {
                spacing: 6.0
                child: banner
                Checkbox::new(q.accepted.clone()) {
                    label: accept.clone()
                    on_change: move |yes: bool, _c: &mut EventContext| accept_vm.accept(&key, yes)
                }
            }))
        }
        None => {
            let index = q.index;
            let chosen = q.chosen.clone();
            let radios: Vec<RadioButton> = q
                .options
                .iter()
                .enumerate()
                .map(|(i, label)| {
                    let choose_vm = vm.clone();
                    RadioButton::new(i, chosen.clone())
                        .label(label.clone())
                        .on_change(move |option, _| choose_vm.choose(index, option))
                })
                .collect();
            Box::new(teksu!(VStack {
                spacing: 6.0
                TextWidget::new(q.title.clone()) {
                    style: TextStyleRole::SmallBold
                    color: TextRole::Primary
                }
                TextWidget::new(q.body.clone()) {
                    style: TextStyleRole::Small
                    color: TextRole::Secondary
                }
                RadioGroup {
                    label: q.title.clone()
                    spacing: 4.0
                    radios: radios
                }
            }))
        }
    }
}

/// Why the project cannot be imported, and what to do: **Check again** when the cause may
/// go away by itself, **Details** for a fault of the importer's own.
fn refusal_body(vm: &ImportScrivenerViewModel) -> impl Widget + use<> {
    let refusal_vm = vm.clone();
    teksu!(ScrollArea {
        Padding::symmetric(4.0, 6.0) {
            VStack {
                spacing: 8.0
                Repeater::new(vm.refusal(), move |r: &RefusalView| refusal_banner(&refusal_vm, r)) {
                    spacing: 8.0
                }
                // The answer that led here, to choose another project or copy.
                child: questions(vm)
            }
        }
    })
}

/// The questions met so far, each with its answer.
fn questions(vm: &ImportScrivenerViewModel) -> Repeater<QuestionView> {
    let question_vm = vm.clone();
    Repeater::new(vm.questions(), move |q: &QuestionView| {
        question(&question_vm, q)
    })
    .spacing(10.0)
}

fn refusal_banner(vm: &ImportScrivenerViewModel, refusal: &RefusalView) -> Box<dyn Widget> {
    let banner = Banner::error(refusal.title.clone());
    let banner = match &refusal.body {
        Some(body) => banner.description(body.clone()),
        None => banner,
    };
    let banner = if refusal.retry {
        let again_vm = vm.clone();
        banner.action(
            Button::new(tr!(import_scrivener_check_again()))
                .variant(ButtonVariant::Plain)
                .on_activate_fn(move |_| again_vm.check_again()),
        )
    } else if let Some(details) = refusal.details.clone() {
        banner.action(
            Button::new(tr!(import_scrivener_details()))
                .variant(ButtonVariant::Plain)
                .on_activate_fn(move |c| show_details(c, &details)),
        )
    } else {
        banner
    };
    Box::new(banner)
}

/// The reading itself failed: say so, with the technical account behind **Details**.
fn failure_body(vm: &ImportScrivenerViewModel) -> impl Widget + use<> {
    let details_vm = vm.clone();
    let again_vm = vm.clone();
    let banner = Banner::error(tr!(import_scrivener_read_failed()))
        .description(tr!(import_scrivener_read_failed_body()));
    teksu!(ScrollArea {
        Padding::symmetric(4.0, 6.0) {
            VStack {
                spacing: 8.0
                child: banner
                HStack {
                    spacing: 8.0
                    Spacer
                    Button::new(tr!(import_scrivener_check_again())) {
                        variant: ButtonVariant::Plain
                        on_activate_fn: move |_| again_vm.check_again()
                    }
                    Button::new(tr!(import_scrivener_details())) {
                        variant: ButtonVariant::Plain
                        on_activate_fn: move |c| {
                            let details = details_vm.failure().get().unwrap_or_default();
                            show_details(c, &details)
                        }
                    }
                }
                // The answer that led here, to choose another project or copy.
                child: questions(vm)
            }
        }
    })
}

fn show_details(ctx: &mut EventContext, details: &str) {
    MessageBox::warning(tr!(import_scrivener_details()))
        .text(lit!(details.to_string()))
        .buttons(MessageBoxButtons::Ok)
        .present(ctx);
}

// ── Review ───────────────────────────────────────────────────────────────────────────────

/// Step two: the choices on the left, the binder the import makes on the right at full
/// height, and under it what the import does that the writer should know first.
fn review_step(vm: &ImportScrivenerViewModel) -> impl Widget + use<> {
    let row_vm = vm.clone();
    let error = vm
        .review_error()
        .map(|e| e.as_ref().map(|e| e.resolve_now()).unwrap_or_default());
    let has_error = vm.review_error().map(|e| usize::from(e.is_some()));
    teksu!(Padding::symmetric(12.0, 12.0) {
        HStack {
            spacing: 14.0
            MaxSize::width(CHOICES_W) {
                VStack {
                    spacing: 8.0
                    TextWidget::new(tr!(import_scrivener_review_hint())) {
                        style: TextStyleRole::Small
                        color: TextRole::Secondary
                    }
                    Switcher::new(has_error) {
                        Spacer
                        TextWidget::new(lit!("")) {
                            text: error
                            style: TextStyleRole::Small
                            color: TextRole::Error
                        }
                    }
                    Expand::vertical {
                        ScrollArea {
                            Padding::symmetric(0.0, 2.0) {
                                Repeater::new(vm.review_rows(), move |row: &ReviewRow| review_row(&row_vm, row)) {
                                    spacing: 6.0
                                }
                            }
                        }
                    }
                }
            }
            Expand::horizontal {
                VStack {
                    spacing: 8.0
                    Expand::vertical {
                        child: preview_tree(vm)
                    }
                    child: before_strip(vm)
                }
            }
        }
    })
}

/// One row of choices: a heading, a pick, or a checkbox.
fn review_row(vm: &ImportScrivenerViewModel, row: &ReviewRow) -> Box<dyn Widget> {
    let detail = row.detail.clone().unwrap_or_else(|| lit!(""));
    match &row.control {
        ReviewControl::Heading => Box::new(teksu!(Padding::symmetric(6.0, 0.0) {
            TextWidget::new(row.label.clone()) {
                style: TextStyleRole::SmallBold
                color: TextRole::Primary
            }
        })),
        ReviewControl::Pick {
            target,
            options,
            selected,
        } => {
            let (target, pick_vm, labels) = (*target, vm.clone(), options.clone());
            let combo = ComboBox::from_items(
                (0..options.len()).collect::<Vec<usize>>(),
                selected.clone(),
                move |i: &usize| labels.get(*i).cloned().unwrap_or_else(|| lit!("")),
            )
            .variant(ComboBoxVariant::Plain)
            // Named by its row: a screen reader says which level or folder the answer is
            // about, not only the answer.
            .label(row.label.clone())
            .on_select(move |i: &usize, _c| pick_vm.pick(target, *i));
            let label = TextWidget::new(row.label.clone()).single_line();
            // Wrapped, never cut: the example and the reason are why the answer was
            // suggested.
            let detail = TextWidget::new(detail)
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary);
            Box::new(teksu!(HStack {
                spacing: 8.0
                Expand::horizontal {
                    VStack {
                        spacing: 1.0
                        child: label
                        child: detail
                    }
                }
                FixedSize {
                    width: 168.0
                    child: combo
                }
            }))
        }
        ReviewControl::Toggle { target, on } => {
            let (target, toggle_vm) = (*target, vm.clone());
            let detail_line = row.detail.is_some().then(|| {
                Padding::symmetric(0.0, 28.0).child(
                    TextWidget::new(detail)
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                )
            });
            Box::new(teksu!(VStack {
                spacing: 1.0
                Checkbox::new(on.clone()) {
                    label: row.label.clone()
                    on_change: move |yes: bool, _c: &mut EventContext| toggle_vm.toggle(target, yes)
                }
                child_opt: detail_line
            }))
        }
    }
}

/// The binder the import makes, before any of it exists: each binder a band, its rows
/// under it, what each becomes, and whether it is in exports.
fn preview_tree(vm: &ImportScrivenerViewModel) -> impl Widget + use<> {
    let title = Column::new(
        "title",
        tr!(import_scrivener_col_title()),
        |row: &PreviewRowView, _cx: &CellContext| {
            let (style, color) = if row.band {
                (TextStyleRole::SmallBold, TextRole::Primary)
            } else if row.title_is_placeholder || row.trashed {
                (TextStyleRole::Body, TextRole::Secondary)
            } else {
                (TextStyleRole::Body, TextRole::Primary)
            };
            Box::new(
                TextWidget::new(row.title.clone())
                    .style(style)
                    .color(color)
                    .single_line(),
            ) as Box<dyn Widget>
        },
    );
    let becomes = Column::new(
        "becomes",
        tr!(import_scrivener_col_becomes()),
        |row: &PreviewRowView, _cx: &CellContext| {
            Box::new(
                TextWidget::new(row.becomes.clone())
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary)
                    .single_line(),
            ) as Box<dyn Widget>
        },
    )
    .width(ColumnWidth::Fixed(112.0));
    // Words rather than a tick, so the column reads the same to every reader.
    let exports = Column::new(
        "exports",
        tr!(import_scrivener_col_exports()),
        |row: &PreviewRowView, _cx: &CellContext| {
            if row.band {
                return Box::new(Spacer::new()) as Box<dyn Widget>;
            }
            let label = if row.in_exports {
                tr!(import_scrivener_in_exports_yes())
            } else {
                tr!(import_scrivener_in_exports_no())
            };
            Box::new(
                TextWidget::new(label)
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary)
                    .single_line(),
            )
        },
    )
    .width(ColumnWidth::Fixed(108.0));

    TreeTableView::from_source(vm.preview())
        .add_column(title)
        .add_column(becomes)
        .add_column(exports)
        .row_height(28.0)
        // Tab walks out of a read-only preview once at its end, rather than holding the
        // focus on its last cell: the wizard's buttons come after it.
        .tab_traversal(TabTraversal::OutOfTable)
        .a11y_label(tr!(import_scrivener_preview_label()))
}

/// What the import does with the binder, or leaves behind, one line per kind: hidden when
/// there is nothing to say.
fn before_strip(vm: &ImportScrivenerViewModel) -> impl Widget + use<> {
    let shown = vm.has_before_you_import().map(|has| usize::from(*has));
    teksu!(Switcher::new(shown) {
        Spacer
        // `MaxSize`, not a height-only `FixedSize`: the latter proposes no width, and the
        // lines would measure with none.
        Expand::horizontal {
            MaxSize::height(STRIP_HEIGHT) {
                VStack {
                    spacing: 4.0
                    TextWidget::new(tr!(import_scrivener_before_title())) {
                        style: TextStyleRole::SmallBold
                        color: TextRole::Primary
                    }
                    Expand::vertical {
                        ScrollArea {
                            // Clear of the scroll bar on the right.
                            Padding::new(0.0, 14.0, 0.0, 0.0) {
                                Repeater::new(vm.before_you_import(), |line: &LocalizedString| {
                                    Box::new(
                                        TextWidget::new(line.clone())
                                            .style(TextStyleRole::Small)
                                            .color(TextRole::Secondary),
                                    ) as Box<dyn Widget>
                                }) {
                                    spacing: 2.0
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

// ── Save ─────────────────────────────────────────────────────────────────────────────────

/// Step three: where the new project goes, what it is called, and what it states.
fn save_step(vm: &ImportScrivenerViewModel) -> impl Widget + use<> {
    let research = vm.research_files().map(|n| usize::from(*n > 0));
    let research_detail = vm
        .research_files()
        .map(|n| tr!(import_scrivener_copy_research(count = *n)).resolve_now());
    let busy = vm.job_running().map(|running| usize::from(*running));
    let busy_banner =
        Banner::info(tr!(project_import_busy_title())).description(tr!(project_import_busy_text()));
    teksu!(ScrollArea {
        Padding::symmetric(16.0, 20.0) {
            VStack {
                spacing: 14.0
                TextWidget::new(tr!(import_scrivener_save_hint())) {
                    style: TextStyleRole::Small
                    color: TextRole::Secondary
                }
                child: destination_form(vm)
                // What the project states about the title, author and language, under
                // the fields it is about.
                Repeater::new(vm.save_hints(), |hint: &LocalizedString| {
                    Box::new(
                        TextWidget::new(hint.clone())
                            .style(TextStyleRole::Small)
                            .color(TextRole::Secondary),
                    ) as Box<dyn Widget>
                }) {
                    spacing: 4.0
                }
                Switcher::new(research) {
                    Spacer
                    VStack {
                        spacing: 2.0
                        Checkbox::new(vm.copy_research_files()) {
                            label: tr!(import_scrivener_copy_research_label())
                        }
                        Padding::symmetric(0.0, 28.0) {
                            TextWidget::new(lit!("")) {
                                text: research_detail
                                style: TextStyleRole::Small
                                color: TextRole::Secondary
                            }
                        }
                    }
                }
                Switcher::new(busy) {
                    Spacer
                    child: busy_banner
                }
            }
        }
    })
}

/// The form: a builder, because `FormLayout` rows are a two-argument accumulator whose
/// fields carry method chains.
fn destination_form(vm: &ImportScrivenerViewModel) -> impl Widget + use<> {
    let language = ComboBox::from_model(vm.languages(), vm.language(), |tag: &String| {
        lit!(ImportScrivenerViewModel::language_label(tag))
    })
    .placeholder(tr!(import_scrivener_language()));

    FormLayout::new()
        .label(tr!(import_scrivener_step_save()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .line(
            field_label(tr!(project_import_location())),
            FilePickerField::new(vm.location())
                .kind(FilePickerKind::PickFolder)
                .dialog_title(tr!(project_import_location_title()))
                .validation(vm.location_validation()),
        )
        .line(
            field_label(tr!(project_import_name())),
            VStack::new()
                .spacing(6.0)
                .child(
                    TextInput::new(vm.name())
                        .placeholder(tr!(project_import_name_placeholder()))
                        .validation(vm.name_validation()),
                )
                .child(
                    HStack::new()
                        .spacing(7.0)
                        .child(
                            TextWidget::new(tr!(project_import_will_create()))
                                .style(TextStyleRole::Small)
                                .color(TextRole::Secondary),
                        )
                        // Wrapped, never cut: the file name, at the end, is what this line
                        // is there to confirm, as the other importers' forms show it.
                        .child(
                            Expand::horizontal().child(
                                TextWidget::new(lit!(""))
                                    .text(vm.target_path())
                                    .style(TextStyleRole::Small)
                                    .color(TextRole::Accent),
                            ),
                        ),
                ),
        )
        .line(
            field_label(tr!(import_scrivener_title_field())),
            TextInput::new(vm.title()),
        )
        .line(
            field_label(tr!(import_scrivener_author())),
            TextInput::new(vm.author()),
        )
        .line(field_label(tr!(import_scrivener_language())), language)
}

fn field_label(text: LocalizedString) -> TextWidget {
    TextWidget::new(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary)
}

#[cfg(test)]
mod tests;
