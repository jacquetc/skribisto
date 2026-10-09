// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

use frontend::AppContext;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::import_management::{
    ScrivenerAnalysisDto, ScrivenerAnalysisOutcome, ScrivenerArg, ScrivenerBinder,
    ScrivenerControl, ScrivenerDestination, ScrivenerFinding, ScrivenerFindingFamily,
    ScrivenerItemRef, ScrivenerMobileEdits, ScrivenerPlace, ScrivenerPreviewRow, ScrivenerQuestion,
    ScrivenerReportGroup, ScrivenerStructure, ScrivenerTitleKind, ScrivenerTrashChoice,
};
use teksilo::core::widget_tree::WidgetTree;

use crate::models::scrivener_inspection::InspectState;
use crate::shared::project_import::ProjectImportJob;

/// The tallest review the measured corpus holds: twelve folders beside the manuscript,
/// a manuscript of three levels, and a Trash.
const ROOTS: i64 = 12;

fn notice(key: &str, args: Vec<ScrivenerArg>) -> ScrivenerFinding {
    ScrivenerFinding::Reported {
        key: key.into(),
        family: ScrivenerFindingFamily::Source,
        group: ScrivenerReportGroup::Unfiled,
        place: ScrivenerPlace::Project,
        item: -1,
        binder_path: Vec::new(),
        item_uid: None,
        args,
    }
}

fn analysis() -> ScrivenerAnalysisDto {
    use BinderItemRole::{Folder, Item};
    use BinderItemSubRole as S;
    let row = |binder, depth, role, sub_role, title: &str, item| ScrivenerPreviewRow::Row {
        binder,
        depth,
        role,
        sub_role,
        title: title.into(),
        title_kind: ScrivenerTitleKind::Own,
        exportable: true,
        activated: true,
        item,
        why: Default::default(),
    };
    let mut rows = vec![row(
        ScrivenerBinder::Manuscript,
        0,
        Folder,
        S::Book,
        "Book",
        0,
    )];
    for chapter in 1..=8 {
        rows.push(row(
            ScrivenerBinder::Manuscript,
            1,
            Folder,
            S::ChapterScene,
            &format!("Chapter {chapter}"),
            chapter,
        ));
        rows.push(row(
            ScrivenerBinder::Manuscript,
            2,
            Item,
            S::Scene,
            "A scene",
            100 + chapter,
        ));
    }
    let mut controls = Vec::new();
    for root in 0..ROOTS {
        let item = 200 + root;
        let title = format!("A folder beside the manuscript with a long name {root}");
        rows.push(row(
            ScrivenerBinder::Notes,
            0,
            Folder,
            S::Note,
            &title,
            item,
        ));
        controls.push(ScrivenerControl::Root {
            item: ScrivenerItemRef::Item {
                item,
                binder_path: vec![title],
            },
            items: 4,
            detected: ScrivenerDestination::Notes,
            chosen: None,
            looks_like_manuscript: root == 0,
        });
    }
    controls.push(ScrivenerControl::Trash {
        items: 3,
        with_writing: 2,
        chosen: ScrivenerTrashChoice::LeaveOut,
    });
    ScrivenerAnalysisDto {
        outcome: ScrivenerAnalysisOutcome::Read,
        notices: vec![
            notice(
                "scrivener-notice-lock-present",
                vec![ScrivenerArg::Flag {
                    name: "this_machine".into(),
                    value: true,
                }],
            ),
            notice(
                "scrivener-notice-sync-debris-ignored",
                vec![ScrivenerArg::Count {
                    name: "count".into(),
                    value: 4,
                }],
            ),
            notice("scrivener-notice-language-not-stated", Vec::new()),
        ],
        pending: vec![ScrivenerQuestion::UnmergedMobileEdits {
            source: ScrivenerMobileEdits::MobileFolder,
        }],
        folder_title: "The Lighthouse Keeper".into(),
        writer_version: "3.3.6".into(),
        documents: 8,
        manuscript_documents: 8,
        research_files: 3,
        structure: ScrivenerStructure::Planned {
            rows,
            controls,
            notices: vec![ScrivenerFinding::Reported {
                key: "scrivener-structure-trash-left-out".into(),
                family: ScrivenerFindingFamily::Structure,
                group: ScrivenerReportGroup::LeftBehind,
                place: ScrivenerPlace::Project,
                item: -1,
                binder_path: Vec::new(),
                item_uid: None,
                args: vec![ScrivenerArg::Count {
                    name: "items".into(),
                    value: 3,
                }],
            }],
            choices: Vec::new(),
            set_aside_trashed: 0,
            set_aside_recovered: 0,
            shape: Default::default(),
            chapter_encoding: Default::default(),
            templates: Vec::new(),
            tags: Vec::new(),
        },
        ..ScrivenerAnalysisDto::default()
    }
}

/// The first widget under `root` whose type name holds `needle`: generic widgets carry
/// their type parameters in their names.
fn first_of_type(tree: &WidgetTree, root: WidgetId, needle: &str) -> Option<WidgetId> {
    if tree
        .widget_type_name(root)
        .is_some_and(|n| n.contains(needle))
    {
        return Some(root);
    }
    tree.children(root)
        .into_iter()
        .find_map(|c| first_of_type(tree, c, needle))
}

/// A wizard on its own tree (the inspection subscribes to backend events), with
/// `state` as the inspection's answer, laid out at the card's size.
fn mount(state: Option<InspectState>) -> (WidgetTree, WidgetId, ImportScrivenerViewModel) {
    let app_ctx = std::rc::Rc::new(AppContext::new());
    let vm = ImportScrivenerViewModel::new(app_ctx.clone(), ProjectImportJob::new(app_ctx.clone()));
    // Text measured the way the app measures it: with no backend, text never wraps and
    // only clamps to the proposal, so an overflow could not show.
    let mut tree =
        crate::test_support::tree_with_events(&app_ctx).with_text_backend(std::rc::Rc::new(
            std::cell::RefCell::new(teksilo::canvas::MockTextBackend::new()),
        ));
    let id = tree.add_boxed(Box::new(ImportScrivenerPanel::new(vm.clone())));
    if let Some(state) = state {
        vm.inspection().state().set(state);
    }
    tree.layout(SizeProposal::exact(CARD_W, CARD_H));
    (tree, id, vm)
}

fn go_to(tree: &mut WidgetTree, vm: &ImportScrivenerViewModel, step: usize) {
    vm.controller().go_to(step);
    tree.layout(SizeProposal::exact(CARD_W, CARD_H));
}

/// No widget the step shows reaches past the card's right edge.
fn assert_inside_the_card(tree: &WidgetTree, root: WidgetId) {
    let card = tree.bounds(root);
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        let b = tree.bounds(id);
        assert!(
            b.x + b.width <= card.x + card.width + 0.5,
            "{} reaches past the card: {b:?} in {card:?}",
            tree.widget_type_name(id).unwrap_or_default()
        );
        stack.extend(tree.children(id));
    }
}

#[test]
fn the_wizard_opens_on_the_project_step_with_a_way_to_choose() {
    let (tree, id, _vm) = mount(None);
    let bounds = tree.bounds(id);
    assert_eq!((bounds.width, bounds.height), (CARD_W, CARD_H));
    assert!(first_of_type(&tree, id, "DropZone").is_some());
    assert!(first_of_type(&tree, id, "Stepper").is_some());
    assert_inside_the_card(&tree, id);
}

/// The card, the question to accept and the notices all mount on the Project page.
#[test]
fn a_read_project_shows_its_card_question_and_notices() {
    crate::test_support::with_real_messages(|| {
        let (tree, id, vm) = mount(Some(InspectState::Read(std::rc::Rc::new(analysis()))));
        assert_eq!(
            vm.inspection_page().get(),
            crate::import_scrivener::import_scrivener_vm::PAGE_CARD
        );
        assert!(
            tree.find_by_label("The Lighthouse Keeper").is_some(),
            "the card's name"
        );
        assert!(
            tree.find_by_label(&tr!(import_scrivener_ios_pending_accept()).resolve_now())
                .is_some(),
            "the checkbox that accepts the iPad question"
        );
        assert!(first_of_type(&tree, id, "Banner").is_some());
        assert_inside_the_card(&tree, id);
    });
}

/// The Review page at the measured tail: twelve folders beside the manuscript. The
/// choices scroll in their own column, so the preview keeps the card's full height and
/// shows far more than its header and three rows.
#[test]
fn the_review_keeps_the_preview_tall_at_twelve_top_level_folders() {
    crate::test_support::with_real_messages(|| {
        let (mut tree, id, vm) = mount(Some(InspectState::Read(std::rc::Rc::new(analysis()))));
        go_to(&mut tree, &vm, 1);
        let Some(preview) = first_of_type(&tree, id, "TreeTableView") else {
            panic!("the review mounted no preview");
        };
        let height = tree.bounds(preview).height;
        assert!(height >= 28.0 * 8.0, "the preview is {height} px tall");
        assert!(
            first_of_type(&tree, id, "Repeater").is_some(),
            "the choices"
        );
        assert_inside_the_card(&tree, id);
    });
}

#[test]
fn the_save_page_lays_out_inside_the_card() {
    crate::test_support::with_real_messages(|| {
        let (mut tree, id, vm) = mount(Some(InspectState::Read(std::rc::Rc::new(analysis()))));
        go_to(&mut tree, &vm, 2);
        assert!(first_of_type(&tree, id, "FilePickerField").is_some());
        assert!(
            tree.find_by_label(&tr!(import_scrivener_copy_research_label()).resolve_now())
                .is_some(),
            "three research files: the copy is offered"
        );
        assert_inside_the_card(&tree, id);
    });
}

/// A refusal replaces the card with its banner.
#[test]
fn a_refusal_shows_its_banner() {
    crate::test_support::with_real_messages(|| {
        let refused = ScrivenerAnalysisDto {
            outcome: ScrivenerAnalysisOutcome::Refused,
            refusal: ScrivenerFinding::Reported {
                key: "scrivener-refusal-cloud-placeholders".into(),
                family: ScrivenerFindingFamily::Refusal,
                group: ScrivenerReportGroup::Unfiled,
                place: ScrivenerPlace::Project,
                item: -1,
                binder_path: Vec::new(),
                item_uid: None,
                args: vec![ScrivenerArg::Count {
                    name: "count".into(),
                    value: 2,
                }],
            },
            ..ScrivenerAnalysisDto::default()
        };
        let (tree, id, _vm) = mount(Some(InspectState::Read(std::rc::Rc::new(refused))));
        assert!(first_of_type(&tree, id, "Banner").is_some());
        assert!(
            tree.find_by_label(&tr!(import_scrivener_check_again()).resolve_now())
                .is_some(),
            "a refusal that may go away offers Check again"
        );
        assert_inside_the_card(&tree, id);
    });
}

/// The wizard announces itself as a named dialog.
#[test]
fn the_wizard_is_a_named_dialog() {
    crate::test_support::with_real_messages(|| {
        let (tree, _id, _vm) = mount(None);
        let dialog = tree.find_by_role(Role::Dialog);
        let name = dialog
            .map(|d| {
                tree.accessibility_node(d)
                    .name()
                    .unwrap_or_default()
                    .to_string()
            })
            .unwrap_or_default();
        assert_eq!(name, "Import a Scrivener project");
    });
}

/// The Launcher is a fixed 820 × 590 and cannot be resized, and a project window may be
/// 800 × 600: the card shrinks into either, Import included, rather than clipping.
#[test]
fn the_card_fits_the_smallest_windows() {
    use crate::shell::launcher_window::{LAUNCHER_H, LAUNCHER_W};
    crate::test_support::with_real_messages(|| {
        for (w, h) in [(LAUNCHER_W as f32, LAUNCHER_H as f32), (800.0, 600.0)] {
            let (mut tree, id, vm) = mount(Some(InspectState::Read(std::rc::Rc::new(analysis()))));
            vm.controller().go_to(2);
            tree.layout(SizeProposal::exact(w, h));
            let Some(card) = tree.children(id).first().copied() else {
                panic!("the panel has its card");
            };
            let b = tree.bounds(card);
            assert!(b.x >= 0.0 && b.y >= 0.0, "{w}x{h}: {b:?}");
            assert!(b.x + b.width <= w && b.y + b.height <= h, "{w}x{h}: {b:?}");
            let import = tree
                .find_by_label(&tr!(import_scrivener_import()).resolve_now())
                .map(|i| tree.bounds(i));
            assert!(
                import.is_some_and(|i| i.y + i.height <= b.y + b.height),
                "{w}x{h}: Import is inside the card: {import:?} in {b:?}"
            );
        }
    });
}

// ── the keyboard, and what each control says ─────────────────────────────────────────────

/// The analysis with nothing left to accept: Next and Import open as soon as it reads.
fn ready() -> ScrivenerAnalysisDto {
    let mut ready = analysis();
    ready.pending.clear();
    ready
}

/// Every widget under `root` whose type name holds `needle`, in tree order.
fn all_of_type(tree: &WidgetTree, root: WidgetId, needle: &str) -> Vec<WidgetId> {
    let mut found = Vec::new();
    if tree
        .widget_type_name(root)
        .is_some_and(|n| n.contains(needle))
    {
        found.push(root);
    }
    for child in tree.children(root) {
        found.extend(all_of_type(tree, child, needle));
    }
    found
}

/// The widget of type `needle` under `root` that a screen reader names `name`.
fn named(tree: &WidgetTree, root: WidgetId, needle: &str, name: &str) -> Option<WidgetId> {
    all_of_type(tree, root, needle)
        .into_iter()
        .find(|id| tree.accessibility_node(*id).name() == Some(name))
}

fn relayout(tree: &mut WidgetTree) {
    tree.layout(SizeProposal::exact(CARD_W, CARD_H));
}

/// Enter that picks an answer in a choice's list picks it, and stays on the Review page.
#[test]
fn enter_in_a_choice_stays_on_the_review() {
    crate::test_support::with_real_messages(|| {
        let (mut tree, id, vm) = mount(Some(InspectState::Read(std::rc::Rc::new(ready()))));
        go_to(&mut tree, &vm, 1);
        let Some(combo) = all_of_type(&tree, id, "ComboBox").first().copied() else {
            panic!("the review has its picks");
        };
        tree.focus(combo);
        tree.press_key(Key::Enter, Modifiers::NONE);
        relayout(&mut tree);
        tree.press_key(Key::Enter, Modifiers::NONE);
        relayout(&mut tree);
        assert_eq!(vm.controller().current(), 1, "still on the Review page");
    });
}

/// Enter that picks a language picks it, and does not start the import.
#[test]
fn enter_in_the_language_does_not_import() {
    let _registry = crate::test_support::IsolatedOpenRegistry::new();
    crate::test_support::with_real_messages(|| {
        let (mut tree, id, vm) = mount(Some(InspectState::Read(std::rc::Rc::new(ready()))));
        let dir = tempfile::tempdir().unwrap();
        vm.location().set(dir.path().to_string_lossy().into_owned());
        vm.name().set("Keeper".into());
        go_to(&mut tree, &vm, 2);
        assert!(vm.can_import().get(), "Import is open");
        let Some(language) = all_of_type(&tree, id, "ComboBox").last().copied() else {
            panic!("the Save page has its language");
        };
        tree.focus(language);
        for key in [Key::Enter, Key::ArrowDown, Key::Enter] {
            tree.press_key(key, Modifiers::NONE);
            relayout(&mut tree);
        }
        assert!(!vm.job_running().get(), "nothing was imported");
        assert_eq!(vm.controller().current(), 2);
    });
}

/// Tab walks into the preview and out of it again: a read-only tree of 2,500 rows is
/// not a place to be held.
#[test]
fn tab_leaves_the_preview() {
    crate::test_support::with_real_messages(|| {
        let (mut tree, id, vm) = mount(Some(InspectState::Read(std::rc::Rc::new(ready()))));
        go_to(&mut tree, &vm, 1);
        let Some(preview) = first_of_type(&tree, id, "TreeTableView") else {
            panic!("the review mounted no preview");
        };
        tree.click(preview);
        relayout(&mut tree);
        assert!(
            tree.focused()
                .is_some_and(|f| tree.is_descendant_of(f, preview)),
            "the preview takes the focus"
        );
        let mut left = false;
        for _ in 0..400 {
            tree.press_key(Key::Tab, Modifiers::NONE);
            relayout(&mut tree);
            if tree
                .focused()
                .is_some_and(|f| !tree.is_descendant_of(f, preview))
            {
                left = true;
                break;
            }
        }
        assert!(left, "Tab never left the preview");
    });
}

/// Each pick is named by its row, so a screen reader says which level or folder it is
/// about, not only the answer it holds.
#[test]
fn a_pick_is_named_by_its_row() {
    crate::test_support::with_real_messages(|| {
        let (mut tree, id, vm) = mount(Some(InspectState::Read(std::rc::Rc::new(ready()))));
        go_to(&mut tree, &vm, 1);
        let picks = all_of_type(&tree, id, "ComboBox");
        assert!(!picks.is_empty());
        for row in 0..ROOTS {
            let name = format!("A folder beside the manuscript with a long name {row}");
            assert!(
                named(&tree, id, "ComboBox", &name).is_some(),
                "no pick is named {name:?}"
            );
        }
    });
}

/// A choice changes the plan, not the controls: the one the writer used keeps the focus.
#[test]
fn a_choice_keeps_the_focus_where_it_was() {
    crate::test_support::with_real_messages(|| {
        let (mut tree, id, vm) = mount(Some(InspectState::Read(std::rc::Rc::new(ready()))));
        go_to(&mut tree, &vm, 1);
        for (label, target) in [
            (
                tr!(import_scrivener_choice_blank_lines()).resolve_now(),
                crate::import_scrivener::import_scrivener_vm::ToggleTarget::BlankLines,
            ),
            (
                tr!(import_scrivener_choice_ornaments()).resolve_now(),
                crate::import_scrivener::import_scrivener_vm::ToggleTarget::Ornaments,
            ),
        ] {
            let Some(checkbox) = named(&tree, id, "Checkbox", &label) else {
                panic!("no checkbox {label:?}");
            };
            tree.focus(checkbox);
            assert_eq!(tree.focused(), Some(checkbox));
            vm.toggle(target, true);
            relayout(&mut tree);
            assert_eq!(tree.focused(), Some(checkbox), "{label}");
        }
    });
}

/// The "Will create" line shows the whole path, file name included, however deep the
/// folder is.
#[test]
fn the_new_projects_path_shows_its_file_name() {
    crate::test_support::with_real_messages(|| {
        let (mut tree, _id, vm) = mount(Some(InspectState::Read(std::rc::Rc::new(ready()))));
        let deep = "/home/writer/Documents/Writing/Novels in progress/The lighthouse series/Drafts imported from Scrivener";
        vm.location().set(deep.into());
        vm.name().set("The Lighthouse Keeper".into());
        go_to(&mut tree, &vm, 2);
        let path = vm.target_path().get();
        assert!(path.ends_with("The Lighthouse Keeper.skrib"));
        let Some(line) = tree.find_by_label(&path) else {
            panic!("no line says {path:?}");
        };
        let one_line = tree
            .find_by_label(&tr!(project_import_will_create()).resolve_now())
            .map(|w| tree.bounds(w).height)
            .unwrap_or(0.0);
        assert!(
            tree.bounds(line).height > one_line * 1.5,
            "the path is cut to one line: {:?}",
            tree.bounds(line)
        );
    });
}

/// A choice's second line, how many items and an example, is shown in full: it is why
/// the choice was suggested.
#[test]
fn a_choices_detail_is_shown_in_full() {
    crate::test_support::with_shipped_messages("fr-FR", || {
        let (mut tree, _id, vm) = mount(Some(InspectState::Read(std::rc::Rc::new(ready()))));
        go_to(&mut tree, &vm, 1);
        let detail = tr!(import_scrivener_root_items_manuscript(count = 4_i64)).resolve_now();
        let Some(line) = tree.find_by_label(&detail) else {
            panic!("no line says {detail:?}");
        };
        let one_line = tree
            .find_by_label("A folder beside the manuscript with a long name 0")
            .map(|w| tree.bounds(w).height)
            .unwrap_or(0.0);
        assert!(
            tree.bounds(line).height > one_line * 1.5,
            "{detail:?} is cut to one line: {:?}",
            tree.bounds(line)
        );
    });
}

/// A refusal reached through an answer keeps the question on the page: the writer can
/// choose the other project from there.
#[test]
fn a_refusal_after_an_answer_shows_the_question() {
    crate::test_support::with_real_messages(|| {
        let several = ScrivenerAnalysisDto {
            outcome: ScrivenerAnalysisOutcome::Choose,
            question: ScrivenerQuestion::SeveralProjects {
                candidates: vec!["Book One.scriv".into(), "Book Two.scriv".into()],
            },
            ..ScrivenerAnalysisDto::default()
        };
        let (mut tree, _id, vm) = mount(Some(InspectState::Read(std::rc::Rc::new(several))));
        vm.choose(0, 1);
        let refused = ScrivenerAnalysisDto {
            outcome: ScrivenerAnalysisOutcome::Refused,
            refusal: ScrivenerFinding::Reported {
                key: "scrivener-refusal-scrivener-one-for-mac".into(),
                family: ScrivenerFindingFamily::Refusal,
                group: ScrivenerReportGroup::Unfiled,
                place: ScrivenerPlace::Project,
                item: -1,
                binder_path: Vec::new(),
                item_uid: None,
                args: Vec::new(),
            },
            ..ScrivenerAnalysisDto::default()
        };
        vm.inspection()
            .state()
            .set(InspectState::Read(std::rc::Rc::new(refused)));
        relayout(&mut tree);
        assert_eq!(
            vm.inspection_page().get(),
            crate::import_scrivener::import_scrivener_vm::PAGE_REFUSED
        );
        let page = tree
            .find_by_label("This project was made by Scrivener 1 for Mac")
            .map(|banner| tree.bounds(banner));
        assert!(page.is_some(), "the refusal");
        assert!(
            tree.find_by_label("Book One.scriv").is_some(),
            "the other project, to choose instead"
        );
    });
}
