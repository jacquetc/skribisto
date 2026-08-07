// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Import documents** wizard: choose files, then review the tree they would
//! make before any of it exists.
//!
//! Two steps behind one `Switcher`. Step one is a drop zone and the files it has
//! collected, in the order they will be read. Step two is the plan — every row the
//! import would create, with the type it resolved to, editable — plus the
//! heading-level rules that produced those types and the destination it lands in.
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

use bastyde::core::accesskit::Role;
use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::core::styles::{ComboBoxVariant, PanelVariant};
use bastyde::data::{ListModel, TreeDataSource};
use bastyde::prelude::TextStyleRole;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, CellContext, Checkbox, Column, ColumnWidth, ComboBox, Divider, DropZone,
    Expand, FixedSize, HStack, IconButton, ListView, Padding, Panel, ProgressBar, ScrollArea,
    Spacer, Switcher, TextWidget, Toast, TreeTableView, VStack,
};

use skribisto_model::CreateType;

use crate::binder::create_labels::recommendation_label;
use crate::models::import_plan_source::PlanRowView;
use crate::view_models::import_document::{
    ImportDocumentViewModel, LEVEL_TYPES, ROW_TYPES, STEP_ANALYSING, STEP_FILES, STEP_REVIEW,
};

const CARD_W: f32 = 920.0;
const CARD_H: f32 = 620.0;

/// Present the wizard over the current window, optionally pre-loaded with files.
///
/// `sources` is what the cold-start door hands over: a writer who chose
/// "New from documents…" on the Launcher already picked their files before the
/// project existed, and asking again would be the wizard forgetting what it was
/// opened for. Every other door passes an empty slice.
/// What a caller already knows when it opens the wizard.
///
/// A struct rather than more positional parameters: there are three call sites now
/// (the File menu, the Launcher's cold start, the binder's "Import here…") and each
/// knows a different subset, which reads badly as a second bare `Vec` argument.
#[derive(Default)]
pub struct ImportDocumentOptions {
    /// Files already chosen — the Launcher's cold-start path picks them first.
    pub sources: Vec<PathBuf>,
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
    vm.add_files(options.sources);
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
        // Plain builders around the Switcher: it takes ordered closure-built slots
        // the `bati!` macro cannot express, as with DockingLayout and TabWidget.
        // Children are positional — their order is the `STEP_*` constants.
        let body = Switcher::new(self.vm.step())
            .child(files_step(ctx, &self.vm))
            .child(review_step(&self.vm))
            .child(analysing_step(&self.vm));

        let root = bati!(ctx => FixedSize {
            width: CARD_W
            height: CARD_H
            Panel {
                variant: PanelVariant::Raised
                corner_radius: 10.0
                padding: 0.0
                VStack {
                    spacing: 0.0
                    Expand::horizontal { child: header(&self.vm) }
                    Expand::horizontal { Divider }
                    Expand::vertical { child: body }
                    Expand::horizontal { Divider }
                    Expand::horizontal { child: footer(&self.vm) }
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
    /// `Role::Dialog` node, and the title strip is a `TextWidget`, which carries
    /// no accessible name of its own. Without this, a screen reader met a panel
    /// that had appeared over everything and could not say what it was.
    fn accessibility(&self, builder: &mut bastyde::core::accessibility::AccessNodeBuilder) {
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

fn header(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
    // Which step the writer is on, as a word rather than a counter: two steps do
    // not need a progress apparatus. A `Switcher` rather than a mapped string,
    // because a `LocalizedString` is resolved when it is rendered — flattening it
    // to a `String` here would freeze it in whatever locale was current at build.
    let where_am_i = Switcher::new(vm.step())
        .child(
            TextWidget::new(tr!(import_document_step_files()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
        .child(
            TextWidget::new(tr!(import_document_step_review()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
        .child(
            TextWidget::new(tr!(import_document_step_analysing()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        );

    FixedSize::new().height(44.0).child(
        Padding::symmetric(8.0, 14.0).child(
            HStack::new()
                .spacing(8.0)
                .child(
                    Expand::horizontal().child(
                        TextWidget::new(tr!(import_document_title()))
                            .style(TextStyleRole::Small)
                            .color(TextRole::Secondary),
                    ),
                )
                .child(where_am_i)
                .child(
                    IconButton::clear()
                        .tooltip(tr!(import_document_close()))
                        .on_activate_fn(|ctx| ctx.dismiss_modal()),
                ),
        ),
    )
}

/// Step one: collect the files.
///
/// `ctx` only to open Browse where the writer last imported from. The zone builds its
/// own file dialog internally, so — like `FilePickerField` — it has to be told the
/// directory here rather than at the click.
fn files_step(ctx: &BuildContext, vm: &ImportDocumentViewModel) -> impl Widget + use<> {
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
    if let Some(dir) = ctx
        .app_state::<crate::models::FolderMemoryService>()
        .and_then(|svc| svc.last(crate::models::FolderPurpose::ImportDocuments))
    {
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

/// Between the two: the analysis, running.
///
/// Its own step rather than a toast, because the wizard is modal — a progress
/// surface *behind* a dialog nobody can dismiss would be chrome the writer can
/// see and not reach. Cancel lives here for the same reason: it is the only
/// thing there is to do while this is on screen.
fn analysing_step(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
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
                // Which file the backend is reading. Blank until the first
                // tick, which is honest: nothing is known yet.
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

/// Step two: the plan.
fn review_step(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
    Padding::symmetric(12.0, 12.0).child(
        VStack::new()
            .spacing(10.0)
            .child(level_rules(vm))
            .child(Expand::vertical().child(plan_tree(vm)))
            .child(diagnostics_strip(vm))
            .child(
                TextWidget::new(tr!(import_document_destination()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            // Same shape as the drop zone above, and the same trap: the
            // `Expand` must sit above the height pin, not under it.
            .child(
                Expand::horizontal().child(
                    FixedSize::new().height(150.0).child(
                        vm.destination()
                            .view(tr!(import_document_destination_empty())),
                    ),
                ),
            ),
    )
}

/// What the importer had to decide, lose or guess — said out loud, before Import.
///
/// This is the feature's whole reason for existing. The prior-art survey behind
/// it found that manuscript import fails *silently* everywhere: Scrivener's split
/// deleting the word it split on, Joplin dropping every relative-path image for
/// years. `document_ingest` names fourteen such moments and attaches each to a
/// file or a row — and until this strip existed the UI collected every one of
/// them and showed none, which is the same failure with extra steps.
///
/// Shown only when there is something to say. An empty box announcing that
/// nothing is wrong costs a third of the tree's height to say nothing.
fn diagnostics_strip(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
    let rows: ListModel<(String, LocalizedString)> = ListModel::new();
    let counts = Signal::new((0usize, 0usize));

    // Refilled whenever a fresh analysis lands. `diagnostics()` is the signal the
    // view-model writes; everything below reads its own mirror of it.
    let mirror = rows.clone();
    let counts_sink = counts.clone();
    let source_vm = vm.clone();
    let refresh = vm.diagnostics().map(move |all| {
        mirror.replace_all(source_vm.diagnostic_messages());
        counts_sink.set(source_vm.diagnostic_counts());
        all.len()
    });

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
            HStack::new()
                .spacing(6.0)
                .child(
                    TextWidget::new(lit!(""))
                        .text(headline)
                        .style(TextStyleRole::SmallBold)
                        .color(headline_color),
                )
                .child(Spacer::new())
                // Invisible; it exists so the list refills when a fresh analysis
                // replaces the diagnostics.
                .child(
                    FixedSize::new()
                        .width(0.0)
                        .child(TextWidget::new(lit!("")).text(refresh.map(|n| n.to_string()))),
                ),
        )
        .child(Expand::vertical().child(ScrollArea::new().child(list)));

    // Height-pinned and scrollable rather than growing: a file with forty image
    // references must not push the tree off the card.
    Switcher::new(vm.diagnostics().map(|d| usize::from(!d.is_empty())))
        .child(Spacer::new())
        .child(Expand::horizontal().child(FixedSize::new().height(74.0).child(body)))
}

/// One combo per heading level the documents actually used.
///
/// The affordance that makes a two-hundred-chapter import survivable: retyping a
/// *level* is one decision where retyping its rows is two hundred. A row the
/// writer already retyped by hand is left alone — the view-model pins it.
fn level_rules(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
    let rules = vm.level_rules();
    let rules_vm = vm.clone();

    // A `ListModel` over the levels, so adding a level after a fresh analysis
    // rebuilds the row without this function knowing how many there are.
    let levels: ListModel<(u8, CreateType)> = ListModel::new();
    let mirror = levels.clone();
    let refresh = rules.map(move |entries| {
        mirror.replace_all(entries.clone());
        entries.len()
    });

    HStack::new()
        .spacing(8.0)
        .child(
            TextWidget::new(tr!(import_document_level_rules()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
        .child(
            Expand::horizontal().child(
                ListView::new(levels, move |_index, entry: &(u8, CreateType), _sel| {
                    let (level, kind) = *entry;
                    let apply = rules_vm.clone();
                    Box::new(
                        HStack::new()
                            .spacing(4.0)
                            .child(
                                TextWidget::new(tr!(import_document_level_n(level = level as i64)))
                                    .style(TextStyleRole::Small),
                            )
                            .child(
                                ComboBox::from_items(
                                    LEVEL_TYPES.to_vec(),
                                    Signal::new(Some(kind)),
                                    |kind: &CreateType| recommendation_label(*kind),
                                )
                                .variant(ComboBoxVariant::Plain)
                                .on_select(
                                    move |kind: &CreateType, _ctx| {
                                        apply.set_level_rule(level, *kind)
                                    },
                                ),
                            ),
                    ) as Box<dyn Widget>
                })
                .item_height(28.0),
            ),
        )
        .child(
            // Nothing visible; it exists so the level list re-fills when a fresh
            // analysis changes which levels the documents used.
            FixedSize::new()
                .width(0.0)
                .child(TextWidget::new(lit!("")).text(refresh.map(|n| n.to_string()))),
        )
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
    .width(ColumnWidth::Fixed(130.0));

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
    .width(ColumnWidth::Fixed(80.0));

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
    .width(ColumnWidth::Fixed(80.0));

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
    .width(ColumnWidth::Fixed(80.0));

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
                .add_column(origin_col)
                .add_column(words)
                .add_column(breaks)
                .add_column(comments)
                .row_height(30.0),
        )
}

fn footer(vm: &ImportDocumentViewModel) -> impl Widget + use<> {
    let step = vm.step();

    // The summary describes whatever step is on screen. It used to describe the
    // *plan* always, so a writer still choosing files read "0 rows · 0 scene
    // breaks" — a count of something they had not asked for yet, sitting under
    // a drop zone, reading like a failure.
    //
    // Resolved to `String` rather than carried as a `LocalizedString` because
    // `TextWidget::text` takes a reactive string; each map re-runs on its own
    // source, and a locale change rebuilds this parent anyway.
    let files_summary = vm
        .file_count()
        .map(|n| tr!(import_document_file_count(count = *n as i64)).resolve_now());

    let summary_source = vm.plan();
    let plan_summary = summary_source.version_signal().map(move |_| {
        let rows = summary_source.rows();
        let breaks: usize = rows.iter().map(|r| r.scene_breaks).sum();
        tr!(import_document_summary(
            rows = rows.len() as i64,
            breaks = breaks as i64
        ))
        .resolve_now()
    });

    // Positional, like every other `Switcher` here — the `STEP_*` order.
    let summary = Switcher::new(step.clone())
        .child(
            TextWidget::new(lit!(""))
                .text(files_summary)
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
        .child(
            TextWidget::new(lit!(""))
                .text(plan_summary)
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
        .child(
            TextWidget::new(tr!(import_document_analysing()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        );

    let back_vm = vm.clone();
    let analyse_vm = vm.clone();
    let import_vm = vm.clone();

    // Analyse needs both a step and something to read; the two greying rules
    // are zipped rather than checked in the handler, so an empty file list
    // reads as "not yet" instead of erroring on click.
    let can_analyse = step
        .zip(&vm.file_count())
        .map(|(s, n)| *s == STEP_FILES && *n > 0);

    FixedSize::new().height(52.0).child(
        Padding::symmetric(10.0, 22.0).child(
            HStack::new()
                .spacing(8.0)
                .child(
                    Button::new(tr!(import_document_back()))
                        .variant(ButtonVariant::Plain)
                        .enabled(step.map(|s| *s == STEP_REVIEW))
                        .on_activate_fn(move |_| back_vm.back_to_files()),
                )
                .child(Expand::horizontal().child(summary))
                .child(
                    Button::new(tr!(import_document_cancel()))
                        .variant(ButtonVariant::Plain)
                        // Not while the analysis runs: closing the wizard would
                        // leave the operation running with nowhere to report.
                        // The analysing step's own Cancel stops it first.
                        .enabled(step.map(|s| *s != STEP_ANALYSING))
                        .on_activate_fn(|ctx| ctx.dismiss_modal()),
                )
                .child(
                    Button::new(tr!(import_document_analyse()))
                        .variant(ButtonVariant::Filled)
                        .enabled(can_analyse)
                        .on_activate_fn(move |ctx| {
                            if let Err(e) = analyse_vm.start_analysis() {
                                ctx.show_toast(Toast::error(lit!(format!("{e:#}"))));
                            }
                        }),
                )
                .child(
                    Button::new(tr!(import_document_import()))
                        .variant(ButtonVariant::Filled)
                        .enabled(step.map(|s| *s == STEP_REVIEW))
                        .on_activate_fn(move |ctx| {
                            // Read before applying: `apply` is what the wizard closes on,
                            // and the picker's selection goes with it.
                            let landed =
                                import_vm.work_uid().zip(import_vm.chosen_destination_key());
                            match import_vm.apply() {
                                Ok(created) => {
                                    if let Some((uid, key)) = landed
                                        && let Some(prefs) =
                                            ctx.app_state::<crate::models::ImportPrefsService>()
                                    {
                                        prefs.remember_destination(&uid, key);
                                    }
                                    // Dismiss first, then toast: the toast would
                                    // otherwise become the topmost overlay and the
                                    // dismissal would take it instead of the modal.
                                    ctx.dismiss_modal();
                                    import_vm.offer_undo(ctx, created.len());
                                }
                                // A refused import leaves the wizard up, with the
                                // plan the writer can still fix — and nothing is
                                // remembered, because nothing landed.
                                Err(e) => import_vm.report_failure(ctx, &e),
                            }
                        }),
                ),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;
    use bastyde::core::widget_tree::WidgetTree;
    use document_ingest::ImportPlan;
    use document_ingest::plan::PlannedRow;
    use frontend::AppContext;
    use std::rc::Rc;

    fn planned(indent: i64, title: &str, kind: CreateType, breaks: usize) -> PlannedRow {
        PlannedRow {
            indent,
            create_type: kind,
            title: title.into(),
            stripped_ordinal: None,
            djot: "Prose.".into(),
            scene_breaks: breaks,
            word_count: 1,
            origin: "a.md".into(),
            included: true,
            comments: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    fn first_containing(tree: &WidgetTree, root: WidgetId, needle: &str) -> Option<WidgetId> {
        if tree
            .widget_type_name(root)
            .is_some_and(|n| n.contains(needle))
        {
            return Some(root);
        }
        tree.children(root)
            .into_iter()
            .find_map(|c| first_containing(tree, c, needle))
    }

    fn mount(vm: ImportDocumentViewModel, app_ctx: &Rc<AppContext>) -> (WidgetTree, WidgetId) {
        // The destination picker and the plan tree both subscribe to backend
        // events, so a bare `WidgetTree` would panic (see `crate::test_support`).
        let mut tree = crate::test_support::tree_with_events(app_ctx);
        let id = tree.add_boxed(Box::new(ImportDocumentPanel::new(vm)));
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        (tree, id)
    }

    #[test]
    fn the_wizard_opens_on_the_file_step_with_a_drop_zone() {
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());
        let (tree, id) = mount(vm, &app_ctx);

        assert!(
            first_containing(&tree, id, "DropZone").is_some(),
            "step one must offer somewhere to drop files"
        );
        let bounds = tree.bounds(id);
        assert!(bounds.width > 0.0 && bounds.height > 0.0, "{bounds:?}");
    }

    /// The review step is the whole point of the feature, so a panel that
    /// silently failed to mount its tree would be the worst possible regression —
    /// and would compile perfectly well.
    #[test]
    fn a_ready_plan_mounts_the_review_tree_and_the_destination_picker() {
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());
        vm.on_plan_ready(
            &ImportPlan {
                rows: vec![
                    planned(0, "Book", CreateType::Book, 0),
                    planned(1, "Chapter One", CreateType::Chapter, 3),
                    planned(2, "Scene A", CreateType::Scene, 0),
                ],
                diagnostics: Vec::new(),
            },
            vec![1, 2, 3],
            vec![
                (1, CreateType::Book),
                (2, CreateType::Chapter),
                (3, CreateType::Scene),
            ],
        );

        let (tree, id) = mount(vm, &app_ctx);
        assert!(
            first_containing(&tree, id, "TreeTableView").is_some(),
            "the review step mounted no plan tree"
        );
        assert!(
            first_containing(&tree, id, "DestinationPickerView").is_some(),
            "the review step mounted no destination picker"
        );
    }

    /// The drop zone must span the card.
    ///
    /// A height-only `FixedSize` proposes `width: None`, and a `DropZone`
    /// measured against no width shrinks to its own text — which shipped: a
    /// ~310px box hugging the left edge of a 920px card, as a drag target. The
    /// fix is an `Expand::horizontal` *inside* the height pin, and nothing but a
    /// laid-out tree can tell the two apart.
    #[test]
    fn the_drop_zone_spans_the_card_rather_than_hugging_its_own_text() {
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());
        let (tree, id) = mount(vm, &app_ctx);

        let zone = first_containing(&tree, id, "DropZone").expect("step one mounts a drop zone");
        let width = tree.bounds(zone).width;
        assert!(
            width > CARD_W * 0.8,
            "the drop zone is {width}px of a {CARD_W}px card — it collapsed to its content"
        );
    }

    /// A blank half-panel under a drop zone reads as "something failed".
    #[test]
    fn an_empty_file_list_says_so_instead_of_showing_nothing() {
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());
        let (tree, id) = mount(vm.clone(), &app_ctx);
        assert!(
            first_containing(&tree, id, "ListView").is_none(),
            "with no files there is nothing to list"
        );

        vm.add_files([PathBuf::from("/tmp/a.md")]);
        let (tree, id) = mount(vm, &app_ctx);
        assert!(
            first_containing(&tree, id, "ListView").is_some(),
            "a chosen file must appear in a list"
        );
    }

    /// The strip appears only when there is something to say, and appears when
    /// there is.
    ///
    /// The failure it guards is the one this whole surface was added to fix: the
    /// analysis collected fourteen kinds of diagnostic, the view-model stored
    /// them, and no widget read them — an import that quietly lost footnotes,
    /// images and whole unreadable files while reporting success. A strip that
    /// silently failed to mount would restore that exactly, and would compile.
    #[test]
    fn diagnostics_reach_the_screen_but_an_empty_box_never_does() {
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());
        vm.on_plan_ready(
            &ImportPlan {
                rows: vec![planned(0, "Book", CreateType::Book, 0)],
                diagnostics: Vec::new(),
            },
            vec![1],
            vec![(1, CreateType::Book)],
        );

        let (tree, id) = mount(vm.clone(), &app_ctx);
        let before = count_of(&tree, id, "ListView");

        vm.set_diagnostics(vec![crate::view_models::import_document::Diagnostic {
            key: "no-headings".into(),
            severity: "info".into(),
            path: "/tmp/a.md".into(),
            detail: String::new(),
            count: 0,
            row: None,
        }]);
        let (tree, id) = mount(vm, &app_ctx);

        assert!(
            count_of(&tree, id, "ListView") > before,
            "a diagnostic must reach the screen"
        );
    }

    fn count_of(tree: &WidgetTree, root: WidgetId, needle: &str) -> usize {
        let here = usize::from(
            tree.widget_type_name(root)
                .is_some_and(|n| n.contains(needle)),
        );
        here + tree
            .children(root)
            .into_iter()
            .map(|c| count_of(tree, c, needle))
            .sum::<usize>()
    }

    /// An empty plan must say so rather than render a blank table.
    #[test]
    fn an_empty_plan_shows_its_own_message_instead_of_a_tree() {
        let app_ctx = Rc::new(AppContext::new());
        let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());
        vm.on_plan_ready(&ImportPlan::default(), Vec::new(), Vec::new());

        let (tree, id) = mount(vm, &app_ctx);
        assert!(
            first_containing(&tree, id, "TreeTableView").is_none(),
            "an empty plan must not mount a table"
        );
    }
}
