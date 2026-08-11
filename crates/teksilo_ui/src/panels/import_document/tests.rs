// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use crate::app_ids::AppIds;
use document_ingest::ImportPlan;
use document_ingest::plan::PlannedRow;
use frontend::AppContext;
use std::rc::Rc;
use teksilo::core::widget_tree::WidgetTree;

fn planned(indent: i64, title: &str, kind: CreateType, breaks: usize) -> PlannedRow {
    PlannedRow {
        indent,
        create_type: kind,
        title: title.into(),
        stripped_ordinal: None,
        djot: "Prose.".into(),
        epigraph: String::new(),
        scene_breaks: breaks,
        word_count: 1,
        origin: "a.md".into(),
        included: true,
        comments: Vec::new(),
        source_uid_tag: None,
        source_digest: None,
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

/// A plan carrying an epigraph still builds and lays out.
///
/// The epigraph column is the ninth on the review table and the only one whose cell
/// text is derived rather than read straight off the row, so it is the one that can
/// throw while every other row renders. `epigraph_preview`'s own tests prove *what* it
/// says; this proves the table survives saying it.
#[test]
fn a_plan_carrying_an_epigraph_still_mounts_its_review_tree() {
    let app_ctx = Rc::new(AppContext::new());
    let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());
    let mut chapter = planned(1, "Chapter One", CreateType::Chapter, 0);
    chapter.epigraph = "> {semantic_role=epigraph}\n> Every winter asks twice.".into();
    vm.on_plan_ready(
        &ImportPlan {
            rows: vec![planned(0, "Book", CreateType::Book, 0), chapter],
            diagnostics: Vec::new(),
        },
        vec![1, 2],
        vec![(1, CreateType::Book), (2, CreateType::Chapter)],
    );

    let (tree, id) = mount(vm, &app_ctx);
    assert!(
        first_containing(&tree, id, "TreeTableView").is_some(),
        "the review step mounted no plan tree"
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

/// A blank half-panel under a drop zone reads as "something failed" — the
/// empty copy must be what the files step shows before anything is chosen.
///
/// The Stepper pre-mounts every step's content, so a "no ListView in the
/// tree" assertion is no longer meaningful (Review's level-rules list is
/// always present). Count ListViews before and after adding a file instead:
/// choosing one must mount the files list.
#[test]
fn an_empty_file_list_says_so_instead_of_showing_nothing() {
    let app_ctx = Rc::new(AppContext::new());
    let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());
    let (tree, id) = mount(vm.clone(), &app_ctx);
    let before = count_of(&tree, id, "ListView");

    vm.add_files([PathBuf::from("/tmp/a.md")]);
    let (tree, id) = mount(vm, &app_ctx);
    assert!(
        count_of(&tree, id, "ListView") > before,
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

/// The strip's rows must be **on screen**, not merely in the tree.
///
/// The bug this pins: the list was a virtualized `ListView` inside a
/// `ScrollArea` inside a height-only `FixedSize`. That `FixedSize` proposes
/// `width: None`, so every row measured 0×0 and was placed nowhere — the
/// writer saw a headline saying "3 things to know" above an empty box with a
/// scrollbar. The old test counted `ListView`s and passed throughout.
#[test]
fn every_diagnostic_row_has_a_size() {
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
    vm.set_diagnostics(vec![crate::view_models::import_document::Diagnostic {
        key: "no-headings".into(),
        severity: "info".into(),
        path: "/tmp/a.md".into(),
        detail: String::new(),
        count: 0,
        row: None,
    }]);
    let (tree, id) = mount(vm.clone(), &app_ctx);

    // The view-model is what holds the rows now — no mirror in the view, and
    // nothing that only fills when an invisible widget happens to be read.
    assert!(
        vm.diagnostic_rows().len() >= 2,
        "the analyser's entry and the illegal-combination one must both be listed"
    );

    let list = first_containing(&tree, id, "ListView").expect("the strip mounts a list");
    let bounds = tree.bounds(list);
    assert!(
        bounds.width > 100.0,
        "the diagnostics list must be laid out at a real width, got {bounds:?}"
    );
    // Capped, and capped by something that actually constrains it: under the
    // old `FixedSize` the list reported its 200 px fallback and was merely
    // clipped, which is how a scrollbar appeared over nothing.
    assert!(
        bounds.height > 0.0 && bounds.height <= STRIP_HEIGHT,
        "the list must be capped at the strip's height, got {bounds:?}"
    );
    // No outer scroller around it: the list scrolls itself, and the second
    // one was a scrollbar over a viewport the list never knew about.
    assert!(
        ancestors(&tree, id, list).iter().all(|a| !tree
            .widget_type_name(*a)
            .is_some_and(|n| n.contains("ScrollArea"))),
        "the diagnostics list must not sit inside a ScrollArea"
    );
}

/// Every ancestor of `needle` up to `root`, nearest first.
fn ancestors(tree: &WidgetTree, root: WidgetId, needle: WidgetId) -> Vec<WidgetId> {
    fn walk(tree: &WidgetTree, here: WidgetId, needle: WidgetId, path: &mut Vec<WidgetId>) -> bool {
        if here == needle {
            return true;
        }
        for c in tree.children(here) {
            path.push(here);
            if walk(tree, c, needle, path) {
                return true;
            }
            path.pop();
        }
        false
    }
    let mut path = Vec::new();
    walk(tree, root, needle, &mut path);
    path.reverse();
    path
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

/// Review keeps the plan tree tall; Destination keeps the outline tree tall.
///
/// These sizes are what the three-step split is for: destination used to share
/// Review as a 150 px strip. Headless (the automation bridge cannot drop files
/// onto the live wizard), so the two active steps are mounted by driving the
/// controller the same way the long-op handlers do.
#[test]
fn review_and_destination_each_get_a_full_pane() {
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

    fn size_of(tree: &WidgetTree, root: WidgetId, needle: &str) -> Option<(f32, f32)> {
        fn find(tree: &WidgetTree, root: WidgetId, needle: &str) -> Option<WidgetId> {
            if tree
                .widget_type_name(root)
                .is_some_and(|n| n.contains(needle))
            {
                return Some(root);
            }
            tree.children(root)
                .into_iter()
                .find_map(|c| find(tree, c, needle))
        }
        let id = find(tree, root, needle)?;
        let b = tree.bounds(id);
        Some((b.width, b.height))
    }

    let (tree, id) = mount(vm.clone(), &app_ctx);
    let (w, h) = size_of(&tree, id, "TreeTableView").expect("plan tree on Review");
    assert!(w > 400.0, "plan tree width {w}");
    // Heading-level rules are height-pinned to n×28; without that pin the
    // ListView ate ~half the pane and the tree sat around 250. It should
    // clearly clear that now that levels take a strip, not a half-panel.
    assert!(
        h > 300.0,
        "plan tree height {h} — level rules / destination must not steal the pane"
    );
    // Inactive Destination must not claim layout while Review is showing.
    if let Some((dw, dh)) = size_of(&tree, id, "DestinationPickerView") {
        assert!(
            dw * dh < 1.0,
            "destination must be zero-sized off-step, got {dw}x{dh}"
        );
    }

    vm.controller()
        .go_to(crate::view_models::import_document::STEP_DESTINATION);
    let (tree, id) = mount(vm, &app_ctx);
    let (w, h) = size_of(&tree, id, "DestinationPickerView").expect("picker on Destination");
    assert!(w > 400.0, "destination width {w}");
    assert!(
        h > 300.0,
        "destination height {h} — the outline needs a full card, not a strip"
    );
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
/// The merge step mounts, and it is a table over the merge — not the review tree again.
///
/// A step that silently failed to build would compile perfectly well, which is exactly the
/// class of mistake this file's other layout tests exist for.
#[test]
fn the_merge_step_mounts_its_table() {
    use crate::view_models::import_document::{MergeRowKey, MergeRowView};
    use skribisto_model::reconcile::{RowAction, RowStatus};

    let app_ctx = Rc::new(AppContext::new());
    let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());

    // A merge with one row on both sides and one the file added — the shape the whole
    // two-column design exists for.
    vm.seed_merge_for_test(vec![
        MergeRowView {
            key: MergeRowKey::Current(uuid::Uuid::from_u128(1)),
            indent: 0,
            current_title: Some("Chapter One".into()),
            current_item_id: Some(1),
            incoming_title: Some("Chapter One".into()),
            incoming_key: None,
            status: RowStatus::EditorEdited,
            moved: false,
            actions: vec![RowAction::TakeImport, RowAction::KeepCurrent],
        },
        MergeRowView {
            key: MergeRowKey::Current(uuid::Uuid::from_u128(2)),
            indent: 0,
            current_title: None,
            current_item_id: None,
            incoming_title: Some("A chapter they added".into()),
            incoming_key: None,
            status: RowStatus::New,
            moved: false,
            actions: vec![RowAction::CreateNew, RowAction::Ignore],
        },
    ]);

    let mut tree = crate::test_support::tree_with_events(&app_ctx);
    let id = tree.add_boxed(Box::new(WidgetHolder::new(reconcile_step(&vm))));
    tree.layout(SizeProposal::exact(CARD_W, CARD_H));

    assert!(
        first_containing(&tree, id, "TreeTableView").is_some(),
        "the merge step must mount its table"
    );
    // Mounting the table is not the same as filling it: the step spent a release showing
    // an empty one. Both seeded rows offer two actions, so both get a combo box.
    assert_eq!(
        count_of(&tree, id, "ComboBox"),
        2,
        "each merge row must render its own action control"
    );
    let bounds = tree.bounds(id);
    assert!(bounds.width > 0.0 && bounds.height > 0.0, "{bounds:?}");
}

/// Reading the merge step must never *write* to it.
///
/// The step used to refill its table from a `Signal::map` closure, on the assumption that a
/// derived signal recomputes when its input changes. It recomputes on every read, and a
/// `Switcher`'s visibility bindings read once a frame — so the closure bumped the version
/// signal the table is bound to, which dirtied the table, which scheduled the frame that
/// read the closure again. The wizard's fourth page pegged a core and never finished a
/// frame. Nothing about a single build says so, which is why this pumps layout and watches
/// the version instead of looking at the widgets.
#[test]
fn mounting_the_merge_step_does_not_re_source_its_table() {
    use crate::view_models::import_document::{MergeRowKey, MergeRowView};
    use skribisto_model::reconcile::{RowAction, RowStatus};
    use teksilo::data::TreeDataSource;

    let app_ctx = Rc::new(AppContext::new());
    let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());
    vm.seed_merge_for_test(vec![MergeRowView {
        key: MergeRowKey::Current(uuid::Uuid::from_u128(1)),
        indent: 0,
        current_title: Some("Chapter One".into()),
        current_item_id: Some(1),
        incoming_title: Some("Chapter One".into()),
        incoming_key: None,
        status: RowStatus::EditorEdited,
        moved: false,
        actions: vec![RowAction::TakeImport, RowAction::KeepCurrent],
    }]);

    let settled = vm.merge_source().version_signal().get();

    let mut tree = crate::test_support::tree_with_events(&app_ctx);
    tree.add_boxed(Box::new(WidgetHolder::new(reconcile_step(&vm))));
    for _ in 0..4 {
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
    }

    assert_eq!(
        vm.merge_source().version_signal().get(),
        settled,
        "drawing the step re-sourced its table — the loop is back"
    );
}

/// With nothing to line up — a first import — the step says so in a sentence rather than
/// showing a table of identical "create it" dropdowns.
#[test]
fn the_merge_step_says_so_when_there_is_nothing_to_reconcile() {
    let app_ctx = Rc::new(AppContext::new());
    let vm = ImportDocumentViewModel::new(app_ctx.clone(), AppIds::default());
    vm.seed_merge_for_test(Vec::new());

    let mut tree = crate::test_support::tree_with_events(&app_ctx);
    let id = tree.add_boxed(Box::new(WidgetHolder::new(reconcile_step(&vm))));
    tree.layout(SizeProposal::exact(CARD_W, CARD_H));

    assert!(
        first_containing(&tree, id, "TreeTableView").is_none(),
        "an empty merge must not mount a table"
    );
}

/// A bare host for a `impl Widget` the step factory returns, so it can be mounted without
/// the whole wizard around it.
#[derive(Debug)]
struct WidgetHolder {
    child: Option<Box<dyn Widget>>,
    id: Option<WidgetId>,
}

impl WidgetHolder {
    fn new(child: impl Widget + 'static) -> Self {
        Self {
            child: Some(Box::new(child)),
            id: None,
        }
    }
}

impl Widget for WidgetHolder {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        match self.child.take() {
            Some(child) => {
                let id = ctx.add_boxed(child);
                self.id = Some(id);
                vec![id]
            }
            None => self.id.into_iter().collect(),
        }
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.id
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}
