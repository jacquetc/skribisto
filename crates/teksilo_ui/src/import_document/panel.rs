// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Import documents** wizard: choose files, then review the tree they would
//! make before any of it exists.
//!
//! Built on Teksilo's [`Stepper`]: **Files → Review →
//! Destination**, with the framework's indicator strip and Back / Next / Finish
//! footer. Analysis progress is shown inside Review while the long op runs (not
//! as its own step). Destination is its own page so the plan tree can use the
//! full card. Form state lives on [`ImportDocumentViewModel`] as signals; each
//! step's `complete_when` gates Next/Finish from those same signals; Finish
//! applies the import — and may **refuse**: the write is one transaction, so a
//! failure returns `false` and keeps the wizard open on Destination with the
//! plan intact. See `teksilo` docs `widgets/stepper.md`.
//!
//! The review step is the point of the feature. Of the twenty-three writing tools
//! surveyed while designing this, not one shows the writer the structure it
//! inferred before committing it: they land their guesses in the manuscript and
//! leave the writer to find the damage. Everything here is editable and nothing is
//! written until Import.
//!
//! Thin, per the house rules: it binds [`ImportDocumentViewModel`]'s signals and
//! calls its methods. Every judgement lives there.

use std::path::PathBuf;

use teksilo::core::BindingLevel;
use teksilo::core::accesskit::Role;
use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::{ComboBoxVariant, PanelVariant};
use teksilo::data::{ListModel, TreeDataSource};
use teksilo::prelude::TextStyleRole;
use teksilo::prelude::*;
use teksilo::widgets::rich_text::{RichTextEditor, ScrollPolicy};
use teksilo::widgets::{
    Button, ButtonVariant, CellContext, Checkbox, Column, ColumnWidth, ComboBox, DropZone, Expand,
    FixedSize, HStack, ListView, MaxSize, Padding, Panel, ProgressBar, Spacer, Step, Stepper,
    Switcher, TextWidget, TreeTableView, VStack,
};

use skribisto_model::CreateType;

use crate::binder::create_labels::recommendation_label;
use crate::models::import_merge_source::ImportMergeSource;
use crate::models::import_plan_source::PlanRowView;
use crate::panels::import_epigraph_cell::EpigraphCell;
use skribisto_model::reconcile::{RowAction, RowStatus};

use super::import_document_vm::{
    ImportDocumentViewModel, LEVEL_TYPES, MergeRowView, ROW_TYPES, STEP_REVIEW, StrayProse,
};

const CARD_W: f32 = 920.0;
const CARD_H: f32 = 620.0;

/// The diagnostics strip's height cap — headline plus about three rows, after
/// which the list scrolls itself.
const STRIP_HEIGHT: f32 = 74.0;

/// What a caller already knows when it opens the wizard.
///
/// A struct rather than more positional parameters: there are three call sites now
/// (the File menu, the Launcher's cold start, the binder's "Import here…") and each
/// knows a different subset.
///
/// Files are deliberately **not** among them. The Launcher's cold-start door used
/// to pick them before the project existed and hand them over here, which meant
/// the writer answered "which documents?" twice — once in a bare file picker, and
/// again on this wizard's first step, which is the only place the answer can be
/// reviewed, reordered and pointed somewhere.
#[derive(Default)]
pub struct ImportDocumentOptions {
    /// Where the import should land, when the caller already knows. `None` leaves the
    /// writer to choose, which is what the File menu does.
    pub destination: Option<crate::models::BinderTreeKey>,
}

pub fn present_import_document(
    ctx: &mut EventContext,
    vm: ImportDocumentViewModel,
    options: ImportDocumentOptions,
) {
    vm.reset();
    // The destination picker was minted with the window (often while `work_id`
    // was still `None`), and unlike the outline/trash it is not reloaded on
    // LoadWork/NewWork. Without this, the review step always shows the empty
    // copy — "No binders yet" — even when the project has binders. Reload
    // *before* preselect so a remembered or "Import here…" row can resolve
    // against a filled tree rather than sitting as a held request forever.
    vm.destination().reload();
    // An explicit destination wins — "Import here…" is the writer answering the question
    // right now, which outranks what they answered last time.
    let destination = options.destination.or_else(|| {
        let uid = vm.work_uid()?;
        ctx.app_state::<crate::models::ImportPrefsService>()?
            .last_destination(&uid)
    });
    if let Some(key) = destination {
        vm.preselect_destination(key);
    }
    ctx.present_modal(
        ModalRequest::deferred(move |t| t.add(ImportDocumentPanel::new(vm.clone())))
            .presentation(ModalPresentation::InTree)
            .close_behavior(ModalCloseBehavior::Manual)
            .title(tr!(import_document_title()))
            .size(CARD_W as u32, CARD_H as u32),
    );
}

pub struct ImportDocumentPanel {
    vm: ImportDocumentViewModel,
    root_child: Option<WidgetId>,
}

impl ImportDocumentPanel {
    pub fn new(vm: ImportDocumentViewModel) -> Self {
        Self {
            vm,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for ImportDocumentPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportDocumentPanel").finish()
    }
}

impl Widget for ImportDocumentPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Leaving Review while an analysis is still running (Back, or a jump)
        // must cancel it — otherwise a late completion would fill a plan the
        // writer walked away from. Success and abandon clear `busy` *before*
        // they leave Review, so this does not re-cancel them.
        {
            let vm = self.vm.clone();
            let current = self.vm.controller().current_step_signal();
            ctx.effect(&current, move |step| {
                if *step != STEP_REVIEW && vm.busy().get() {
                    vm.cancel_analysis();
                }
            });
        }

        // `Step::content` is a factory with no `BuildContext`, so the Browse
        // start directory is resolved once here and captured into the zone.
        let start_dir = ctx
            .app_state::<crate::models::FolderMemoryService>()
            .and_then(|svc| svc.last(crate::models::FolderPurpose::ImportDocuments));

        let files_vm = self.vm.clone();
        let analyse_vm = self.vm.clone();
        let review_vm = self.vm.clone();
        let dest_vm = self.vm.clone();
        let merge_vm = self.vm.clone();
        let reconcile_vm = self.vm.clone();
        let finish_vm = self.vm.clone();
        let cancel_vm = self.vm.clone();

        let stepper = Stepper::new()
            .controller(self.vm.controller())
            .back_label(tr!(import_document_back()))
            .next_label(tr!(import_document_analyse()))
            .finish_label(tr!(import_document_import()))
            .cancel(tr!(import_document_cancel()), move |ctx, _ctrl| {
                // While the long op runs (on Review), Cancel means "stop reading".
                if cancel_vm.busy().get() {
                    cancel_vm.cancel_analysis();
                } else {
                    ctx.dismiss_modal();
                }
            })
            .step(
                Step::new(tr!(import_document_step_files()))
                    .content(move || files_step(&files_vm, start_dir.clone()))
                    .complete_when(self.vm.can_analyse_signal())
                    .validate_on_next({
                        let vm = analyse_vm;
                        // Real: start analysis (progress on Review while busy).
                        // `mocks`: plant a plan and advance — no file drop needed.
                        move || vm.try_advance_from_files()
                    }),
            )
            .step(
                Step::new(tr!(import_document_step_review()))
                    .content(move || review_step(&review_vm))
                    // Next stays off until the long op finishes; Destination is
                    // the page after that.
                    .complete_when(self.vm.can_proceed_from_review_signal()),
            )
            .step(
                Step::new(tr!(import_document_step_destination()))
                    .content(move || destination_step(&dest_vm))
                    .complete_when(self.vm.can_apply_signal())
                    .validate_on_next({
                        // The merge is computed here rather than in the next step's content
                        // factory, because the factory may run once while this runs on every
                        // advance — and the whole question is about the destination that was
                        // *just* chosen.
                        let vm = merge_vm;
                        move || {
                            vm.rebuild_merge();
                            true
                        }
                    }),
            )
            .step(
                Step::new(tr!(import_document_step_reconcile()))
                    .content(move || reconcile_step(&reconcile_vm)),
            )
            // Import. The write is one transaction, so it either lands whole or
            // not at all — and when it does not, returning `false` keeps the
            // wizard open on Destination with the step marked in error, beside
            // the failure toast. The plan is still there to fix or retry;
            // reporting a finished flow over an import that never happened would
            // have closed it.
            .on_finish(move |ctx, _ctrl| {
                let landed = finish_vm.work_uid().zip(finish_vm.chosen_destination_key());
                match finish_vm.apply() {
                    Ok(created) => {
                        if let Some((uid, key)) = landed
                            && let Some(prefs) =
                                ctx.app_state::<crate::models::ImportPrefsService>()
                        {
                            prefs.remember_destination(&uid, key);
                        }
                        // Dismiss first, then toast: the toast would otherwise
                        // become the topmost overlay and the dismissal would take
                        // it instead of the modal.
                        ctx.dismiss_modal();
                        finish_vm.offer_undo(ctx, created.len());
                        true
                    }
                    Err(e) => {
                        finish_vm.report_failure(ctx, &e);
                        false
                    }
                }
            });

        let root = teksu!(ctx => FixedSize {
            width: CARD_W
            height: CARD_H
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

    /// Announce the wizard as a named dialog.
    ///
    /// `ctx.present_modal` does not wrap a hand-drawn panel in a
    /// `ModalContainer` — that is the shape every custom modal in this app has
    /// (settings, new work, import Plume) — so nothing else would emit a
    /// `Role::Dialog` node. Without this, a screen reader met a panel that had
    /// appeared over everything and could not say what it was.
    fn accessibility(&self, builder: &mut teksilo::core::accessibility::AccessNodeBuilder) {
        builder.set_role(Role::Dialog);
        builder.set_name(tr!(import_document_title()).resolve_now());
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// Step one: collect the files.
///
/// `start_dir` is resolved once by the panel (the content factory has no
/// `BuildContext`) so Browse opens where the writer last imported from.
fn files_step(
    vm: &ImportDocumentViewModel,
    start_dir: Option<std::path::PathBuf>,
) -> impl Widget + use<> {
    let drop_vm = vm.clone();
    let mut zone = DropZone::new(tr!(import_document_drop_title()))
        .subtitle(tr!(import_document_drop_hint()))
        .accept_extensions(ImportDocumentViewModel::accepted_extensions())
        .allow_multiple(true)
        .browse_label(tr!(import_document_browse()))
        .on_files_dropped(move |paths, c| {
            if let Some(first) = paths.first() {
                crate::models::remember_dialog_file(
                    c,
                    crate::models::FolderPurpose::ImportDocuments,
                    first,
                );
            }
            drop_vm.add_files(paths);
        });
    if let Some(dir) = start_dir {
        zone = zone.starting_dir(dir);
    }

    let files: ListModel<PathBuf> = vm.files();
    let row_vm = vm.clone();
    let list = ListView::new(files, move |index, path: &PathBuf, _selected| {
        // Only the file's own name: the writer chose these, and a column of
        // absolute paths would bury the one part that distinguishes them.
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());
        let up = row_vm.clone();
        let down = row_vm.clone();
        let remove = row_vm.clone();
        Box::new(
            Padding::symmetric(16.0, 2.0).child(
                HStack::new()
                    .spacing(6.0)
                    .child(Expand::horizontal().child(TextWidget::new(lit!(name))))
                    .child(
                        Button::new(tr!(import_document_move_up()))
                            .variant(ButtonVariant::Plain)
                            .enabled(index > 0)
                            .on_activate_fn(move |_| up.move_file(index, -1)),
                    )
                    .child(
                        Button::new(tr!(import_document_move_down()))
                            .variant(ButtonVariant::Plain)
                            .on_activate_fn(move |_| down.move_file(index, 1)),
                    )
                    .child(
                        Button::new(tr!(import_document_remove_file()))
                            .variant(ButtonVariant::Plain)
                            .on_activate_fn(move |_| remove.remove_file(index)),
                    ),
            ),
        ) as Box<dyn Widget>
    })
    .item_height(30.0);

    // An empty list has to say so. A blank half-panel under a drop zone reads as
    // "something failed", which is exactly the wrong first impression for a
    // surface whose entire job is to look trustworthy before it writes anything.
    let listing = Switcher::new(vm.file_count().map(|n| usize::from(*n > 0)))
        .child(
            Padding::symmetric(24.0, 16.0).child(
                TextWidget::new(tr!(import_document_no_files()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            ),
        )
        .child(list);

    Padding::symmetric(16.0, 20.0).child(
        VStack::new()
            .spacing(12.0)
            // `Expand::horizontal` *outside* the height pin, not inside it.
            //
            // A height-only `FixedSize` proposes `width: None` to its child, and
            // a `DropZone` measured against no width shrinks to its own text — a
            // ~310px box hugging the left of a 920px card, as a drag target.
            // Wrapping the zone in an `Expand` does not help, because that
            // `Expand` is measured against the same `None`; the `Expand` has to
            // sit above the pin, where the `VStack` gives it a real width to
            // hand down.
            .child(Expand::horizontal().child(FixedSize::new().height(150.0).child(zone)))
            .child(Expand::vertical().child(listing)),
    )
}

/// Progress while `analyze_document_import` runs — shown *inside* the Review
/// step, not as its own indicator entry, so the strip stays Files / Review /
/// Destination.
fn analysing_body(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
    let cancel_vm = vm.clone();
    Padding::symmetric(40.0, 40.0).child(
        VStack::new()
            .spacing(14.0)
            .child(Spacer::new())
            .child(
                TextWidget::new(tr!(import_document_analysing()))
                    .style(TextStyleRole::Body)
                    .color(TextRole::Primary),
            )
            .child(
                ProgressBar::new(0.0)
                    .value(vm.progress())
                    .thickness(6.0)
                    .label(tr!(import_document_analysing())),
            )
            .child(
                TextWidget::new(lit!(""))
                    .text(vm.progress_message())
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .child(
                HStack::new().spacing(8.0).child(Spacer::new()).child(
                    Button::new(tr!(import_document_cancel_analysis()))
                        .variant(ButtonVariant::Plain)
                        .on_activate_fn(move |_| cancel_vm.cancel_analysis()),
                ),
            )
            .child(Spacer::new()),
    )
}

/// Step two: the plan (and analysis progress while it is still being built).
///
/// Destination lives on the next step — pinning it here left the tree a
/// ~150px strip that could not show a real outline.
fn review_step(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
    let plan = Padding::symmetric(12.0, 12.0).child(
        VStack::new()
            .spacing(10.0)
            .child(level_rules(vm))
            .child(Expand::vertical().child(plan_tree(vm)))
            .child(diagnostics_strip(vm)),
    );

    // Index 0 = plan, 1 = progress. `busy` is set before Next advances onto
    // this step, so the first paint is the progress body.
    Switcher::new(vm.busy().map(|b| usize::from(*b)))
        .child(plan)
        .child(analysing_body(vm))
}

/// Step three: where the import lands — the full card, not a footer strip.
fn destination_step(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
    Padding::symmetric(16.0, 16.0).child(
        VStack::new()
            .spacing(10.0)
            .child(
                TextWidget::new(tr!(import_document_destination()))
                    .style(TextStyleRole::Body)
                    .color(TextRole::Primary),
            )
            .child(
                TextWidget::new(tr!(import_document_destination_hint()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .child(
                Expand::vertical().child(
                    vm.destination()
                        .view(tr!(import_document_destination_empty())),
                ),
            ),
    )
}

/// Step four: what the returning file does to the book it left from.
///
/// Shown only when there is something to line up. A first import into an empty destination
/// matches nothing, and a table of identical "create it" dropdowns would be a page of ceremony
/// saying what the row count already said — so that case gets one sentence instead.
fn reconcile_step(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
    // The merge lives on the view-model and is re-sourced there, by `rebuild_merge`, on every
    // entry to this step — the writer may have gone back and chosen a different destination.
    //
    // Nothing here may refill it. This step once built its own source and re-sourced it from a
    // `Signal::map` closure, on the assumption that a derived signal recomputes when its input
    // changes. It recomputes on every *read* — including every visibility evaluation the
    // `Switcher` below performs, which is once a frame. Writing the table's rows from there
    // bumped the version signal the table is bound to, so reading the step dirtied it and
    // dirtying it scheduled the frame that read it again: the wizard's fourth page pegged a
    // core and never finished a frame, showing its header and an empty table.
    let source = vm.merge_source();

    let has_matches = vm.anything_to_reconcile().map(|m| usize::from(*m));

    // Deliberately not a live count of what matched. `TextWidget`'s reactive setter takes a
    // plain `String`, so a translated plural cannot be bound to a signal — and a count read
    // once at build time would go stale the moment the writer went back and chose a different
    // destination. The table below says which rows matched, in more detail than a number
    // could, so the header says what the step is *for* and leaves the counting to it.
    let header = TextWidget::new(tr!(import_document_reconcile_hint()))
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary);

    let body = VStack::new()
        .spacing(8.0)
        .child(header)
        .child(Expand::vertical().child(merge_tree(vm, source)));

    Padding::symmetric(16.0, 12.0).child(
        Switcher::new(has_matches)
            .child(
                Padding::symmetric(8.0, 40.0).child(
                    TextWidget::new(tr!(import_document_reconcile_all_new()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                ),
            )
            .child(body),
    )
}

/// The merge table: the project's tree down the left, the returning file beside it.
fn merge_tree(vm: &ImportDocumentViewModel, source: ImportMergeSource) -> impl Widget + use<> {
    // Column 0 is the tree, and it is the **project's** tree — blank on a row only the file
    // has, which is what makes an inserted chapter read as the gap it is.
    let current = Column::new(
        "current",
        tr!(import_document_col_current()),
        move |row: &MergeRowView, _cx: &CellContext| match &row.current_title {
            Some(title) => Box::new(TextWidget::new(lit!(title.clone()))) as Box<dyn Widget>,
            None => Box::new(Spacer::new()),
        },
    );

    let incoming = Column::new(
        "incoming",
        tr!(import_document_col_incoming()),
        move |row: &MergeRowView, _cx: &CellContext| match &row.incoming_title {
            Some(title) => Box::new(TextWidget::new(lit!(title.clone())).color(
                if row.current_title.is_some() {
                    TextRole::Primary
                } else {
                    // A row only the file has: named in the colour of something being added,
                    // so the eye finds the insertions without reading the Status column.
                    TextRole::Success
                },
            )) as Box<dyn Widget>,
            None => Box::new(Spacer::new()),
        },
    );

    let status = Column::new(
        "status",
        tr!(import_document_col_status()),
        move |row: &MergeRowView, _cx: &CellContext| {
            Box::new(
                TextWidget::new(status_label(row.status, row.moved))
                    .style(TextStyleRole::Small)
                    .color(if row.status.needs_attention() {
                        TextRole::Warning
                    } else {
                        TextRole::Secondary
                    }),
            ) as Box<dyn Widget>
        },
    )
    .width(ColumnWidth::Fixed(150.0));

    // `CellContext` carries a flat row index and no node identity, so a per-row control has to
    // resolve its own key through the source — a delegate that closed over `cx.row_index`
    // would act on whatever row happened to sit at that position after a collapse.
    let action_source = source.clone();
    let action_vm = vm.clone();
    let action = Column::new(
        "action",
        tr!(import_document_col_action()),
        move |row: &MergeRowView, cx: &CellContext| {
            let Some(key) = action_source.key_at(cx.row_index) else {
                return Box::new(Spacer::new()) as Box<dyn Widget>;
            };
            if row.actions.len() < 2 {
                // Exactly one thing may happen to this row — a missing row can only be kept.
                // A combo box offering one choice is a control that cannot be used.
                return Box::new(
                    TextWidget::new(action_label(
                        row.actions.first().copied().unwrap_or(RowAction::Ignore),
                    ))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
                );
            }
            let selected = Signal::new(Some(action_vm.action_for(row)));
            let pick = action_vm.clone();
            Box::new(
                ComboBox::from_items(row.actions.clone(), selected, |a: &RowAction| {
                    action_label(*a)
                })
                .variant(ComboBoxVariant::Plain)
                .on_select(move |a: &RowAction, _ctx| pick.set_action(key, *a)),
            )
        },
    )
    .width(ColumnWidth::Fixed(190.0));

    let compare_source = source.clone();
    let compare_vm = vm.clone();
    let compare = Column::new(
        "compare",
        lit!(""),
        move |row: &MergeRowView, cx: &CellContext| {
            if !row.can_compare() {
                return Box::new(Spacer::new()) as Box<dyn Widget>;
            }
            let Some(key) = compare_source.key_at(cx.row_index) else {
                return Box::new(Spacer::new());
            };
            let open = compare_vm.clone();
            Box::new(
                Button::new(tr!(import_document_compare()))
                    .variant(ButtonVariant::Ghost)
                    .on_activate_fn(move |ctx| {
                        if let Some(row) = open.merge_row(key) {
                            show_compare(ctx, &open, &row);
                        }
                    }),
            )
        },
    )
    .width(ColumnWidth::Fixed(110.0));

    TreeTableView::from_source(source)
        .add_column(current)
        .add_column(incoming)
        .add_column(status)
        .add_column(action)
        .add_column(compare)
        .row_height(30.0)
}

/// What becomes of prose on a row that cannot hold it.
fn stray_prose_label(choice: StrayProse) -> LocalizedString {
    match choice {
        StrayProse::AsParatext => tr!(import_document_stray_as_paratext()),
        StrayProse::Discard => tr!(import_document_stray_discard()),
    }
}

fn status_label(status: RowStatus, moved: bool) -> LocalizedString {
    if moved {
        return tr!(import_document_status_moved());
    }
    match status {
        RowStatus::Identical => tr!(import_document_status_identical()),
        RowStatus::EditorEdited => tr!(import_document_status_editor_edited()),
        RowStatus::YouEdited => tr!(import_document_status_you_edited()),
        RowStatus::Conflict => tr!(import_document_status_conflict()),
        RowStatus::Different => tr!(import_document_status_different()),
        RowStatus::New => tr!(import_document_status_new()),
        RowStatus::Missing => tr!(import_document_status_missing()),
    }
}

fn action_label(action: RowAction) -> LocalizedString {
    match action {
        RowAction::CommentsOnly => tr!(import_document_action_comments_only()),
        RowAction::TakeImport => tr!(import_document_action_take_import()),
        RowAction::KeepCurrent => tr!(import_document_action_keep_current()),
        RowAction::CreateNew => tr!(import_document_action_create_new()),
        RowAction::Ignore => tr!(import_document_action_ignore()),
    }
}

/// The two sides of one row, side by side and read-only.
///
/// Through the same renderer the Versions dock uses, over the shared [`crate::widgets::diff_pane::DiffPane`] — one
/// rendering with two sources rather than two implementations of "show me a diff".
fn show_compare(ctx: &mut EventContext, vm: &ImportDocumentViewModel, row: &MergeRowView) {
    let (current, incoming) = vm.compare_prose(row);
    let title = row
        .current_title
        .clone()
        .or_else(|| row.incoming_title.clone())
        .unwrap_or_default();
    ctx.present_modal(
        ModalRequest::deferred(move |t| {
            t.add(ComparePanel::new(current.clone(), incoming.clone()))
        })
        .presentation(ModalPresentation::InTree)
        .title(lit!(title))
        .size(COMPARE_W as u32, COMPARE_H as u32)
        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

const COMPARE_W: f32 = 720.0;
const COMPARE_H: f32 = 520.0;

/// One row's two versions, rendered by the same machinery the Versions dock uses.
///
/// A widget rather than an inline tree because `ModalRequest::deferred` builds its content
/// into the host's own tree — and because the [`crate::widgets::diff_pane::DiffPane`] has to outlive a rebuild, or the
/// comparison would reload and lose its scroll position on every repaint.
struct ComparePanel {
    pane: crate::widgets::DiffPane,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for ComparePanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComparePanel").finish()
    }
}

impl ComparePanel {
    fn new(current: String, incoming: String) -> Self {
        let pane = crate::widgets::DiffPane::new();
        let diff = crate::versions::version_diff::diff_djot(&current, &incoming);
        pane.show(&crate::versions::version_diff::render(&diff, None, &|n| {
            format!("[{n}]")
        }));
        Self {
            pane,
            root_child: None,
        }
    }
}

impl Widget for ComparePanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let editor = RichTextEditor::read_only(self.pane.doc.clone())
            .content_padding_symmetric(6.0, 8.0)
            .h_scroll_policy(ScrollPolicy::AlwaysOff);

        let root = teksu!(ctx => FixedSize {
            width: COMPARE_W
            height: COMPARE_H
            Panel {
                variant: PanelVariant::Raised
                corner_radius: 10.0
                padding: 14.0
                VStack {
                    spacing: 8.0
                    TextWidget::new(tr!(import_document_compare_legend())) {
                        style: TextStyleRole::Small
                        color: TextRole::Secondary
                    }
                    Expand::vertical {
                        child: editor
                    }
                    HStack {
                        Spacer
                        Button::new(tr!(import_document_compare_close())) {
                            on_activate_fn: |ctx| ctx.dismiss_modal()
                        }
                    }
                }
            }
        });
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// What the importer had to decide, lose or guess — said out loud, before Import.
///
/// This is the feature's whole reason for existing. The prior-art survey behind
/// it found that manuscript import fails *silently* everywhere: a splitter
/// deleting the word it split on, an importer dropping every relative-path image
/// for years. `document_ingest` names fourteen such moments and attaches each to a
/// file or a row — and until this strip existed the UI collected every one of
/// them and showed none, which is the same failure with extra steps.
///
/// Shown only when there is something to say. An empty box announcing that
/// nothing is wrong costs a third of the tree's height to say nothing.
fn diagnostics_strip(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
    // The view-model owns the rows and refills them on every write to
    // `diagnostics` — see `ImportDocumentViewModel::diagnostic_rows`. This used
    // to be a local `ListModel` filled by a side effect *inside* a mapped
    // signal, kept alive by a zero-width label whose only job was to read it:
    // the rows never reached the screen (they were built but never measured, so
    // every one laid out 0×0) and the label leaked its counter as a stray number
    // beside the headline.
    let rows = vm.diagnostic_rows();
    let counting_vm = vm.clone();
    let counts = vm
        .diagnostics()
        .map(move |_| counting_vm.diagnostic_counts());

    let headline = counts.map(|(errors, warnings)| {
        tr!(import_document_diagnostics(
            errors = *errors as i64,
            warnings = *warnings as i64
        ))
        .resolve_now()
    });
    // Red only when a file was lost outright. A warning is the ordinary case —
    // most imports have one — and colouring those red would train the writer to
    // ignore the colour by the second import.
    let headline_color = counts.map(|(errors, _)| {
        if *errors > 0 {
            TextRole::Error
        } else {
            TextRole::Warning
        }
    });

    let list = ListView::new(rows, |_i, entry: &(String, LocalizedString), _sel| {
        let (severity, message) = entry.clone();
        let color = match severity.as_str() {
            "error" => TextRole::Error,
            "warning" => TextRole::Warning,
            _ => TextRole::Secondary,
        };
        Box::new(
            Padding::symmetric(2.0, 0.0).child(
                TextWidget::new(message)
                    .style(TextStyleRole::Small)
                    .color(color),
            ),
        ) as Box<dyn Widget>
    })
    .item_height(20.0);

    let body = VStack::new()
        .spacing(2.0)
        .child(
            TextWidget::new(lit!(""))
                .text(headline)
                .style(TextStyleRole::SmallBold)
                .color(headline_color),
        )
        // The `ListView` scrolls itself — wrapping it in a `ScrollArea` gave the
        // strip a second, outer scrollbar over a viewport the list never knew
        // about, which is the empty scroller this box used to be. `Expand` so the
        // list is *allocated* the leftover height: a virtualized list sizes its
        // viewport from an allocation, and one that is only measured falls back
        // to a 200 px window it then gets clipped out of.
        .child(Expand::vertical().child(list));

    // Height-capped rather than growing: a file with forty image references must
    // not push the tree off the card. `MaxSize`, not `FixedSize`: a height-only
    // `FixedSize` proposes `width: None` to its child, and a virtualized list
    // measured with no width places every row at 0×0 — visible in the a11y tree,
    // invisible on screen. `MaxSize` forwards the width it was given.
    Switcher::new(vm.diagnostics().map(|d| usize::from(!d.is_empty())))
        .child(Spacer::new())
        .child(Expand::horizontal().child(MaxSize::height(STRIP_HEIGHT).child(body)))
}

/// One combo per heading level the documents actually used.
///
/// The affordance that makes a two-hundred-chapter import survivable: retyping a
/// *level* is one decision where retyping its rows is two hundred. A row the
/// writer already retyped by hand is left alone — the view-model pins it.
///
/// Built as its own widget that **rebuilds** when `level_rules` changes (same
/// pattern as the tag alias chips). A `ListView` row factory under a height pin
/// kept laying out the "Heading N" labels at zero width and the combos without
/// a readable selected value — fine for hundreds of rows, wrong for six levels.
fn level_rules(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
    LevelRulesStrip {
        vm: vm.clone(),
        root_child: None,
    }
}

/// Compact heading-level → type map. Not a `ListView`: the rule table is tiny
/// (a handful of levels) and must size to its labels, not to a virtualized
/// viewport.
struct LevelRulesStrip {
    vm: ImportDocumentViewModel,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for LevelRulesStrip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LevelRulesStrip").finish()
    }
}

impl Widget for LevelRulesStrip {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Rebuild when the level table or the plan shape changes (including a
        // synthetic top-level insert, which renumbers every row).
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.vm
            .level_rules()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        self.vm
            .plan()
            .version_signal()
            .bind_to(sid, reg, BindingLevel::Rebuild);

        // Nothing to map and nothing to wrap — stay invisible until analysis
        // (or a mock seed) has produced a plan.
        if self.vm.plan().is_empty() {
            let id = ctx.add(Spacer::new());
            self.root_child = Some(id);
            return vec![id];
        }

        let entries = self.vm.level_rules().get();

        // One tight row per level: label takes its text width, combo is rigid
        // (~120 px min) — no Expand between them, so neither can zero the other.
        let mut rows = VStack::new().spacing(4.0);
        for (level, kind) in entries {
            let apply = self.vm.clone();
            let selected = Signal::new(Some(kind));
            rows = rows.child(
                HStack::new()
                    .spacing(8.0)
                    .child(
                        TextWidget::new(tr!(import_document_level_n(level = level as i64)))
                            .style(TextStyleRole::Small)
                            .color(TextRole::Secondary),
                    )
                    .child(
                        ComboBox::from_items(
                            LEVEL_TYPES.to_vec(),
                            selected,
                            |kind: &CreateType| recommendation_label(*kind),
                        )
                        .variant(ComboBoxVariant::Plain)
                        .on_select(move |kind: &CreateType, _ctx| {
                            apply.set_level_rule(level, *kind)
                        }),
                    ),
            );
        }

        let add_vm = self.vm.clone();
        // Several chapter-files with no Book heading: the writer adds the
        // container here, then every analysed row sits under it. Shown even
        // when there are no heading-level rules (headingless files).
        let add = Button::new(tr!(import_document_add_top_level()))
            .variant(ButtonVariant::Plain)
            .tooltip(tr!(import_document_add_top_level_tooltip()))
            .on_activate_fn(move |_| add_vm.add_top_level_header());

        let id = ctx.add(
            HStack::new()
                .spacing(10.0)
                .child(
                    TextWidget::new(tr!(import_document_level_rules()))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary),
                )
                .child(Expand::horizontal().child(rows))
                .child(add),
        );
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// The plan itself.
///
/// `from_source` over the in-memory plan, which the framework virtualises — an
/// eight-hundred-row manuscript builds only the rows on screen. The type column's
/// combo reads and writes that row's own signal, because the framework's cell
/// editing reports *where* an edit happened and never *what* it was.
fn plan_tree(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
    let source = vm.plan();

    let tick_source = source.clone();
    let tick_vm = vm.clone();
    let included = Column::new(
        "included",
        tr!(import_document_col_included()),
        move |_row: &PlanRowView, cx: &CellContext| {
            let Some(key) = tick_source.key_at(cx.row_index) else {
                return Box::new(Spacer::new()) as Box<dyn Widget>;
            };
            let Some(signal) = tick_source.included_signal(key) else {
                return Box::new(Spacer::new());
            };
            // Enabled only while every ancestor is coming too, so a ticked row
            // under an unticked chapter reads as held back rather than as a
            // contradiction.
            Box::new(
                Checkbox::new(signal)
                    .labels_hidden(true)
                    .enabled(tick_vm.ancestors_included(key)),
            )
        },
    )
    .width(ColumnWidth::Fixed(64.0));

    let title = Column::new(
        "title",
        tr!(import_document_col_title()),
        move |row: &PlanRowView, _cx: &CellContext| {
            Box::new(TextWidget::new(lit!(row.title.clone()))) as Box<dyn Widget>
        },
    );

    let type_source = source.clone();
    let type_vm = vm.clone();
    let kind = Column::new(
        "type",
        tr!(import_document_col_type()),
        move |_row: &PlanRowView, cx: &CellContext| {
            let Some(key) = type_source.key_at(cx.row_index) else {
                return Box::new(Spacer::new()) as Box<dyn Widget>;
            };
            // The row's own live signal, never a snapshot of it — a bulk level
            // rule writes this signal, and a cell bound to a copy would go on
            // showing the type the row had when its cell happened to be built.
            let Some(selected) = type_source.type_signal(key) else {
                return Box::new(Spacer::new());
            };
            let retype = type_vm.clone();
            Box::new(
                ComboBox::from_items(ROW_TYPES.to_vec(), selected, |kind: &CreateType| {
                    recommendation_label(*kind)
                })
                .variant(ComboBoxVariant::Plain)
                .on_select(move |kind: &CreateType, _ctx| retype.retype_row(key, *kind)),
            )
        },
    )
    .width(ColumnWidth::Fixed(160.0));

    // Which file a row came from. With eight files imported at once, this is the
    // first thing you need when a row looks wrong — and the column's own ftl key
    // had been sitting unused since M4, which is how it stayed missing.
    let origin_col = Column::new(
        "source",
        tr!(import_document_col_source()),
        move |row: &PlanRowView, _cx: &CellContext| {
            let name = std::path::Path::new(&row.origin)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| row.origin.clone());
            Box::new(
                TextWidget::new(lit!(name))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            ) as Box<dyn Widget>
        },
    )
    // 130 → 100: thirty of the eighty pixels the epigraph column needs. A file *name* in
    // Small/Secondary that was already ellipsising at 130 loses nothing legible by it, and
    // the alternative was a tenth column the writer has to scroll to reach.
    .width(ColumnWidth::Fixed(100.0));

    // A row the importer had to say something about. The strip below carries the
    // sentence; this is what tells you *which* row it was about without reading
    // the strip and hunting for the title.
    let flag_vm = vm.clone();
    let flag_source = source.clone();
    let flag = Column::new(
        "flag",
        lit!(""),
        move |_row: &PlanRowView, cx: &CellContext| {
            let Some(key) = flag_source.key_at(cx.row_index) else {
                return Box::new(Spacer::new()) as Box<dyn Widget>;
            };
            let found = flag_vm.diagnostics_for_row(key);
            if found.is_empty() {
                return Box::new(Spacer::new());
            }
            let color = if found.iter().any(|d| d.is_error()) {
                TextRole::Error
            } else {
                TextRole::Warning
            };
            Box::new(TextWidget::new(lit!("!")).color(color))
        },
    )
    .width(ColumnWidth::Fixed(24.0));

    let words = Column::new(
        "words",
        tr!(import_document_col_words()),
        move |row: &PlanRowView, _cx: &CellContext| {
            Box::new(TextWidget::new(lit!(row.word_count.to_string()))) as Box<dyn Widget>
        },
    )
    // 80 → 73: the last twenty-one pixels, so the flexible **title** column ends up
    // exactly as wide as it was before the epigraph column existed. The header here
    // already ellipsised at 80 ("Comme…"), so the width was never carrying it — what
    // has to fit is a per-row count, which is one to four digits.
    .width(ColumnWidth::Fixed(73.0));

    // The break count is what tells the writer, before they commit, that a
    // four-thousand-word chapter is arriving as one row and not as four.
    let breaks = Column::new(
        "breaks",
        tr!(import_document_col_breaks()),
        move |row: &PlanRowView, _cx: &CellContext| {
            let text = if row.scene_breaks == 0 {
                String::new()
            } else {
                row.scene_breaks.to_string()
            };
            Box::new(TextWidget::new(lit!(text))) as Box<dyn Widget>
        },
    )
    // 80 → 73: the last twenty-one pixels, so the flexible **title** column ends up
    // exactly as wide as it was before the epigraph column existed. The header here
    // already ellipsised at 80 ("Comme…"), so the width was never carrying it — what
    // has to fit is a per-row count, which is one to four digits.
    .width(ColumnWidth::Fixed(73.0));

    // How many editors' notes come with this row. Threads, not turns: a comment
    // with four replies is one note, and the number a writer wants before pressing
    // Import is "how many things is somebody asking me about", not "how many
    // paragraphs of conversation".
    //
    // Blank rather than 0 for a row with none, matching the breaks column beside
    // it: a column of zeros reads as a measurement that failed.
    let comments = Column::new(
        "comments",
        tr!(import_document_col_comments()),
        move |row: &PlanRowView, _cx: &CellContext| {
            let text = if row.comments.is_empty() {
                String::new()
            } else {
                row.comments.len().to_string()
            };
            Box::new(TextWidget::new(lit!(text))) as Box<dyn Widget>
        },
    )
    // 80 → 73: the last twenty-one pixels, so the flexible **title** column ends up
    // exactly as wide as it was before the epigraph column existed. The header here
    // already ellipsised at 80 ("Comme…"), so the width was never carrying it — what
    // has to fit is a per-row count, which is one to four digits.
    .width(ColumnWidth::Fixed(73.0));

    // The quotation this row is headed by, when the source named one — a mark here and the
    // words themselves on hover. See `import_epigraph_cell` for why it is a marker rather
    // than a preview (the table was already exactly as wide as its card) and why the
    // tooltip still has to be readable (an epigraph is recognised from a paragraph style,
    // so "did it read the right paragraph" is the question a tick cannot answer).
    let epigraph = Column::new(
        "epigraph",
        tr!(import_document_col_epigraph()),
        move |row: &PlanRowView, _cx: &CellContext| {
            Box::new(EpigraphCell::new(row.epigraph.clone())) as Box<dyn Widget>
        },
    )
    .width(ColumnWidth::Fixed(80.0));

    // Only ever populated for a row carrying prose its own type cannot hold — a Book, a Part
    // or a folder, which store none. Blank everywhere else, because a column of empty combo
    // boxes would suggest a decision exists on every row when it exists on almost none.
    let stray_source = source.clone();
    let stray_vm = vm.clone();
    let stray = Column::new(
        "stray",
        tr!(import_document_col_stray_prose()),
        move |_row: &PlanRowView, cx: &CellContext| {
            let Some(key) = stray_source.key_at(cx.row_index) else {
                return Box::new(Spacer::new()) as Box<dyn Widget>;
            };
            let Some(choice) = stray_vm.stray_prose_for(key) else {
                return Box::new(Spacer::new());
            };
            let selected = Signal::new(Some(choice));
            let pick = stray_vm.clone();
            Box::new(
                ComboBox::from_items(StrayProse::ALL.to_vec(), selected, |c: &StrayProse| {
                    stray_prose_label(*c)
                })
                .variant(ComboBoxVariant::Plain)
                .on_select(move |c: &StrayProse, _ctx| pick.set_stray_prose(key, *c)),
            )
        },
    )
    // 170 → 140: the other thirty. This combo is empty on almost every row (see above) and
    // its longest label still fits — the width was headroom, and the epigraph column is a
    // better use of it than headroom on a control most imports never show.
    .width(ColumnWidth::Fixed(140.0));

    let has_rows = {
        let s = source.clone();
        source
            .version_signal()
            .map(move |_| usize::from(!s.is_empty()))
    };

    Switcher::new(has_rows)
        .child(
            Padding::symmetric(24.0, 40.0).child(
                TextWidget::new(tr!(import_document_plan_empty()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            ),
        )
        .child(
            TreeTableView::from_source(source)
                .add_column(included)
                .add_column(flag)
                .add_column(title)
                .add_column(kind)
                .add_column(stray)
                .add_column(origin_col)
                .add_column(words)
                .add_column(breaks)
                .add_column(comments)
                .add_column(epigraph)
                .row_height(30.0),
        )
}

#[cfg(test)]
mod tests;
