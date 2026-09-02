// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Three of the tree-walking helpers below are used only by the `mocks`-gated
// tests, so they are dead in the default build and live in the other. Deleting
// them from the default build's point of view is the mistake this comment
// exists to prevent — it was made once, and the mocks build caught it.
#![cfg_attr(not(feature = "mocks"), allow(dead_code))]

use super::*;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use teksilo::core::widget_tree::WidgetTree;

use crate::test_support::first_of_type;

/// A per-type typography set with distinguishable fonts (Scene/Synopsis =
/// Literata, Notes = Inter) so tests can assert the right bundle reaches the
/// right editor.
fn test_typography() -> EditorTypographySet {
    let bundle = |family: &str| EditorTypography {
        font_family: Signal::new(family.to_string()),
        size: Signal::new(1.0),
        line_height: Signal::new(1.5),
        first_line_indent: Signal::new(0.0),
        para_spacing_before: Signal::new(0.0),
        para_spacing_after: Signal::new(0.0),
        size_range: crate::settings::TypographySizeRange::default(),
    };
    EditorTypographySet {
        scene: bundle("Literata"),
        synopsis: bundle("Literata"),
        notes: bundle("Inter"),
        corkboard: bundle("Literata"),
        distraction_free: bundle("Literata"),
    }
}

/// Every valid `(role, sub_role)` must build a tab and lay it out headlessly
/// without panicking — the per-combination dispatch + each tab's widget tree.
/// The `bool` flags which combinations open the dual-pane prose editor (Scene /
/// ChapterScene / Note): those carry a prose kind + a main editor; the rest do
/// not.
#[test]
fn every_combination_builds_and_lays_out() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let combos = [
        (Item, Scene, true),
        (Item, ChapterScene, true),
        (Item, Note, true),
        (Item, Part, false),
        (Item, BookBegin, false),
        (Item, BookEnd, false),
        (Item, Text, false),
        (Folder, None, false),
        // A chapter folder carries its own prose, like the flat ChapterScene.
        (Folder, ChapterScene, true),
        (Folder, Part, false),
        (Folder, Book, false),
        (Folder, Note, false),
        // A paratext writes like a scene — same editor, same body — so it is
        // "prose" here even though its content role is not `SceneText`.
        (Item, Paratext, true),
        (Folder, Paratext, false),
    ];
    let ctx = Rc::new(AppContext::new());
    for (role, sub_role, is_prose) in combos {
        let tab = tab_for(
            &ctx,
            1,
            &role,
            &sub_role,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        // Prose tabs carry a kind + a main editor; every other combination has
        // neither.
        if is_prose {
            assert!(
                tab.kind().is_some(),
                "{role:?}/{sub_role:?} prose needs a kind"
            );
            assert!(
                tab.main().is_some(),
                "{role:?}/{sub_role:?} prose needs main"
            );
        } else {
            assert!(
                tab.kind().is_none(),
                "{role:?}/{sub_role:?} non-prose has no kind"
            );
        }
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
        assert!(
            tree.bounds(id).width > 0.0,
            "{role:?}/{sub_role:?} laid out to zero width"
        );
    }
}

/// The epigraph box appears on exactly the four combinations the matrix gives an
/// `EpigraphText` — both encodings of a part and of a chapter — and on no others. The
/// gate is the model, not the pane, which is what keeps `prose()` (shared with Scene,
/// Note and Paratext) from sprouting one.
#[test]
fn only_the_headed_combinations_offer_an_epigraph() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let headed = [
        (Item, Part),
        (Item, ChapterScene),
        (Folder, Part),
        (Folder, ChapterScene),
    ];
    let all = [
        (Item, Scene),
        (Item, ChapterScene),
        (Item, Note),
        (Item, Part),
        (Item, BookBegin),
        (Item, BookEnd),
        (Item, Text),
        (Folder, None),
        (Folder, ChapterScene),
        (Folder, Part),
        (Folder, Book),
        (Folder, Note),
    ];
    let ctx = Rc::new(AppContext::new());
    for (role, sub_role) in all {
        let tab = tab_for(
            &ctx,
            1,
            &role,
            &sub_role,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        let expected = headed.iter().any(|(r, s)| r == &role && s == &sub_role);
        assert_eq!(
            tab.epigraph().is_some(),
            expected,
            "{role:?}/{sub_role:?} disagrees with the matrix about the epigraph"
        );

        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
        assert_eq!(
            first_of_type(&tree, id, "Accordion").is_some(),
            expected,
            "{role:?}/{sub_role:?} disagrees about mounting the epigraph disclosure"
        );
    }
}

/// An empty epigraph starts folded and an authored one starts open, so a project that
/// never uses epigraphs carries no open box on every chapter page — and one that does
/// never has to go looking for its own text.
///
/// The empty case passes an explicit blank `EpigraphText` row rather than no rows at
/// all: a *missing* row is the one input the two builds disagree on, since the mocks
/// variant of `SingleContent::for_field` fabricates prose for it (see its `fabricate`).
/// An empty row is honoured verbatim by both, so this pins the gate itself — "is there
/// text" — in either feature set.
#[test]
fn the_epigraph_box_opens_only_when_there_is_something_in_it() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());

    let empty = tab_for(
        &ctx,
        1,
        &Folder,
        &Part,
        &[ContentDto {
            id: 76,
            role: ContentRole::EpigraphText,
            data: String::new(),
            activated: true,
            ..Default::default()
        }],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    assert!(
        !empty.epigraph_expanded.get(),
        "an empty epigraph must start folded away"
    );

    let filled = tab_for(
        &ctx,
        2,
        &Folder,
        &Part,
        &[ContentDto {
            id: 77,
            role: ContentRole::EpigraphText,
            data: "> Salt is the only honest preservative.".to_string(),
            activated: true,
            ..Default::default()
        }],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    assert!(
        filled.epigraph_expanded.get(),
        "an authored epigraph must start open"
    );
}

/// A trashed open item shows the permanent "in the Trash" warning banner above
/// its editor (gated on `open_doc.trashed`), and the tab still lays out.
#[test]
fn a_trashed_tab_shows_the_restore_banner() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.open_doc.trashed.set(true);
    let mut tree = WidgetTree::new();
    let id = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
    assert!(
        first_of_type(&tree, id, "Banner").is_some(),
        "a trashed tab must render the Trash banner"
    );
    assert!(
        tree.bounds(id).width > 0.0,
        "trashed tab laid out to zero width"
    );
}

/// The three folder containers (Book / Part / Chapter) render a `SegmentedControl`
/// bar. The per-type "last view" memory wraps that body in a `RememberSegment`
/// passthrough; this pins that the wrapper doesn't swallow the bar's layout (the
/// bar must still be present **and** lay out to a non-zero size).
#[test]
fn segmented_containers_lay_out_their_bar() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    for sub_role in [Book, Part, ChapterScene] {
        let tab = tab_for(
            &ctx,
            1,
            &Folder,
            &sub_role,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
        let bar = first_of_type(&tree, id, "SegmentedControl")
            .unwrap_or_else(|| panic!("Folder/{sub_role:?} has no SegmentedControl in its tree"));
        let b = tree.bounds(bar);
        assert!(
            b.width > 0.0 && b.height > 0.0,
            "Folder/{sub_role:?} segmented bar laid out to zero size ({b:?})"
        );
    }
}

/// A **notes folder** is segmented too, now that it offers an Overview of its
/// contents. It used to be a bare synopsis page with no bar at all, so this pins the
/// bar's existence, not only its size.
#[test]
fn a_notes_folder_lays_out_its_two_segment_bar() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Folder,
        &Note,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    let mut tree = WidgetTree::new();
    let id = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
    let bar = first_of_type(&tree, id, "SegmentedControl")
        .expect("Folder/Note has no SegmentedControl in its tree");
    let b = tree.bounds(bar);
    assert!(
        b.width > 0.0 && b.height > 0.0,
        "the notes folder's segmented bar laid out to zero size ({b:?})"
    );
}

/// The Overview view-model exists for exactly the containers that offer the segment
/// — a **wider** set than the stream's, because a notes folder has a subtree but no
/// manuscript extent.
///
/// `ContentTab` and `skribisto_model::overview_capable` must agree: a tab that built
/// no view-model would render the segment's pane as an empty `VStack`, which looks
/// like a bug in the table rather than a gate that said no.
#[test]
fn the_overview_view_model_matches_the_model_gate() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let combos = [
        (Item, Scene),
        (Item, ChapterScene),
        (Item, Note),
        (Item, Part),
        (Item, BookBegin),
        (Item, BookEnd),
        (Item, Text),
        (Folder, None),
        (Folder, ChapterScene),
        (Folder, Part),
        (Folder, Book),
        (Folder, Note),
    ];
    for (role, sub_role) in combos {
        let tab = tab_for(
            &ctx,
            1,
            &role,
            &sub_role,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        assert_eq!(
            tab.overview().is_some(),
            skribisto_model::overview_capable(&role, &sub_role),
            "{role:?}/{sub_role:?}: the tab and the model disagree about the Overview"
        );
    }
}

/// The Overview segment **mounts and lays out** — the positional
/// `SegmentedControl` ↔ `Switcher` contract, end to end.
///
/// The two are matched by index, not by name, so a segment added without its child
/// (or in the wrong order) does not fail to compile: it silently shows the *previous*
/// view under the new label. Selecting the last index and finding a real table is
/// what proves the pairing.
#[cfg(feature = "mocks")]
#[test]
fn the_overview_segment_mounts_a_table() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    // Just the container kinds now. This used to carry each one's Overview *index*
    // — (ChapterScene, 4), (Part, 4), (Book, 6), (Note, 1) — a table that existed
    // solely because the bar was positional, and that had to be corrected by hand
    // every time a segment was inserted. Addressing the segment by id retires it.
    for sub_role in [ChapterScene, Part, Book, Note] {
        let tab = tab_for(
            &ctx,
            101, // the mock Book container — its fixture subtree has rows
            &Folder,
            &sub_role,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.segment
            .set(Some(crate::tabs::shared::segments::segment_id(
                crate::tabs::shared::segments::SEG_OVERVIEW,
            )));
        // The Overview pane subscribes to backend events in its wiring child, so it
        // needs a tree that has an event source (see `crate::test_support`).
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
        let table = first_containing(&tree, id, "TreeTableView").unwrap_or_else(|| {
            panic!(
                "Folder/{sub_role:?}'s Overview segment mounted no TreeTableView — \
                     the segment and its Switcher child have drifted out of step"
            )
        });
        let b = tree.bounds(table);
        assert!(
            b.width > 0.0 && b.height > 0.0,
            "Folder/{sub_role:?}: the Overview table laid out to zero size ({b:?})"
        );
    }
}

/// The Analysis segment mounts its own pane, and only on a Book.
///
/// Same reasoning as the Overview test above, and the same trap: the bar and the
/// `Switcher` are matched by position, so an inserted segment whose child was forgotten
/// compiles cleanly and silently shows the neighbouring view under the new label. This
/// selects Analysis by index and insists on finding the pane that belongs there.
#[cfg(feature = "mocks")]
#[test]
fn only_the_book_mounts_an_analysis_segment() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());

    // Book: own page / Full Book / Full Synopsis / Pace / Analysis / Corkboard / Overview
    let tab = tab_for(
        &ctx,
        101,
        &Folder,
        &Book,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    // The Analysis segment, by id. This line previously read `set(4)` — and my first
    // pass at this migration rewrote every `set(4)` to the Overview id, because on a
    // Chapter/Part index 4 *is* Overview. On a Book it is Analysis. That is precisely
    // the confusion the keyed bar exists to make impossible, and it slipped through
    // because this test is `mocks`-gated and never ran in a default-feature run.
    tab.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_ANALYSIS,
        )));
    assert!(
        tab.analysis().is_some(),
        "a Book carries an Analysis view-model"
    );

    // The pane starts an analysis on open and subscribes to long-operation events, so
    // it needs a tree with an event source.
    let mut tree = crate::test_support::tree_with_events(&ctx);
    let id = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
    let pane = first_containing(&tree, id, "AnalysisPane").expect(
        "a Book's Analysis segment mounted no AnalysisPane — the segment and its \
             Switcher child have drifted out of step",
    );
    let b = tree.bounds(pane);
    assert!(
        b.width > 0.0 && b.height > 0.0,
        "the Analysis pane laid out to zero size ({b:?})"
    );

    // A Part has no Analysis at all: the gate is the same Book-only one Pace uses, and
    // widening it by accident would put a book-scale report on a chapter.
    let part = tab_for(
        &ctx,
        101,
        &Folder,
        &Part,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    assert!(part.analysis().is_none(), "only a Book is analysed for now");
}

/// The tags column reaches the table, and renders dots only for rows that have tags.
///
/// The Overview was the last view showing binder rows that did not surface tags (the
/// stream, corkboard and editor all did), because its column was reserved as an inert
/// seam while tags were built in a parallel worktree. This pins that the seam is
/// actually wired, not merely still reserved.
#[cfg(feature = "mocks")]
#[test]
fn the_overview_shows_tag_dots_for_tagged_rows_only() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        101,
        &Folder,
        &Book,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    // Addressed by id: what used to be "index 6" is just the Overview segment now,
    // and stays correct however many segments precede it.
    tab.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OVERVIEW,
        )));
    let mut tree = crate::test_support::tree_with_events(&ctx);
    let id = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(1200.0, 700.0));

    // Every row now hosts a `TagDotsRow` — the cell IS the picker, and an untagged one
    // has to stay clickable or a first tag can never be added from the Overview. What
    // still distinguishes tagged from untagged is one level down: only a tagged row
    // builds a `TagChipRow` of real dots; an untagged one draws the muted `+`
    // placeholder. See `TagDotsRow::offering_when_empty`.
    let rows = {
        use teksilo::data::TreeDataSource;
        tab.overview()
            .expect("a Book has an Overview")
            .rows()
            .visible_count()
    };
    let mut dot_rows = 0;
    count_containing(&tree, id, "TagDotsRow", &mut dot_rows);
    assert_eq!(
        dot_rows, rows,
        "every row must host the picker, tagged or not ({dot_rows} of {rows})"
    );

    // The fixture tags scenes 201 and 202 and leaves the rest untagged.
    let mut chip_rows = 0;
    count_containing(&tree, id, "TagChipRow", &mut chip_rows);
    assert!(
        chip_rows > 0,
        "no TagChipRow in the Overview - the tags column is not wired"
    );
    assert!(
        chip_rows < rows,
        "every one of the {rows} rows drew dots ({chip_rows}); an untagged row must draw \
             the placeholder, or the column stops distinguishing tagged from not"
    );
}

/// Count nodes at/under `root` whose type name contains `needle`.
fn count_containing(tree: &WidgetTree, root: WidgetId, needle: &str, n: &mut usize) {
    if tree
        .widget_type_name(root)
        .is_some_and(|t| t.contains(needle))
    {
        *n += 1;
    }
    for c in tree.children(root) {
        count_containing(tree, c, needle, n);
    }
}

/// F2 opens the editor on the **focused cell**, and does so exactly once.
///
/// The table used to carry its own F2 handler on top of the one
/// `TreeTableView` already implements (`EditTrigger::F2OrTypeOrDoubleClick`
/// is the default, and both editable columns opt in). Teksilo fires the
/// external handler and the widget's own with no short-circuit on
/// `Handled`, so both ran: the widget's opened the *focused* cell, mine
/// opened the first *selected* row. When those disagree the second call
/// commits and closes the first one's edit before opening its own — the
/// user gets the wrong row, or a stray undo entry, depending on ordering.
///
/// So the duplicate is gone and this pins what remains: the surviving
/// handler is the widget's, and it works.
#[cfg(feature = "mocks")]
#[test]
fn f2_opens_the_editor_on_the_focused_cell() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    use teksilo::data::TreeDataSource;
    use teksilo::widgets::TreeTableView;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        101,
        &Folder,
        &Book,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OVERVIEW,
        )));
    let mut tree = crate::test_support::tree_with_events(&ctx);
    let root = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(1200.0, 700.0));

    fn find(tree: &WidgetTree, id: WidgetId, needle: &str) -> Option<WidgetId> {
        if tree
            .widget_type_name(id)
            .is_some_and(|t| t.contains(needle))
        {
            return Some(id);
        }
        tree.children(id)
            .into_iter()
            .find_map(|c| find(tree, c, needle))
    }
    let table = find(&tree, root, "TreeTableView").expect("the Overview mounts a TreeTableView");

    tree.focus(table);
    tree.widget_as_any(table)
        .unwrap()
        .downcast_ref::<TreeTableView<crate::models::OverviewRow>>()
        .expect("the table is keyed by OverviewRow")
        .set_focused_cell(0, 0);

    let vm = tab.overview().unwrap().clone();
    // `Option::None` spelled out: the `BinderItemSubRole::*` glob above puts a
    // `None` *variant* in scope, which is what a bare `None` would resolve to.
    assert_eq!(
        vm.editing_cell().get(),
        Option::None,
        "nothing is being edited yet"
    );

    tree.press_key(Key::F2, Modifiers::NONE);

    let (uid, col) = vm.editing_cell().get().expect(
        "F2 must open an editor; removing the app-side handler must not have \
                     taken the only working one with it",
    );
    assert_eq!(col, crate::models::COL_TITLE, "F2 edits the focused column");
    assert_eq!(
        Some(uid),
        vm.rows().key_at(0),
        "F2 edits the focused row, not merely some row"
    );
}

/// "Rename" must actually open the cell editor.
///
/// The view-model addresses an edit by `(uid, col_id)` — a durable key, because this
/// table re-sources constantly — while `CellContext::is_editing`, the only thing the
/// cell delegates consult, is the *widget's* `(row, display_pos)`. Nothing bridged the
/// two, so the menu item set the intent and no cell ever noticed: it did nothing at
/// all, silently, with no error anywhere.
///
/// Differential on purpose. The header carries a `SearchField`, which is itself a
/// `TextInput`, so an absolute count would pass on the broken code; only the change
/// across `begin_edit` is the cell editor.
#[cfg(feature = "mocks")]
#[test]
fn renaming_an_overview_row_mounts_a_cell_editor() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        101,
        &Folder,
        &Book,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OVERVIEW,
        )));

    // The pane is rebuilt at each step rather than mutated: the editing signal is
    // bound at `BindingLevel::Rebuild`, so a rebuild is exactly what the framework
    // does, and a fresh `TreeTableView` (with a fresh, empty `editing_cell`) is the
    // condition the seed has to survive.
    let inputs = |tab: &ContentTab| {
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let id = tree.add_boxed(tab_pane(tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(1200.0, 700.0));
        let mut n = 0;
        count_containing(&tree, id, "TextInput", &mut n);
        n
    };

    let idle = inputs(&tab);

    let vm = tab.overview().unwrap().clone();
    let uid = common::uid::fixture_uid(201);
    vm.begin_edit(uid, crate::models::COL_TITLE);
    let editing = inputs(&tab);
    assert!(
        editing > idle,
        "begin_edit set the view-model's editing signal but no cell editor mounted \
             ({idle} -> {editing} TextInputs)"
    );

    // Seeded from the row it targets, not left blank — an editor that opens empty
    // would silently clear the title on commit.
    assert!(
        !vm.edit_buffer().text.get().is_empty(),
        "the open editor was not seeded with the row's title"
    );

    // ...and it closes again. Without this the mount could be a one-way latch that
    // never returns the table to its normal cells.
    vm.cancel_edit();
    assert_eq!(
        inputs(&tab),
        idle,
        "cancelling the edit left the cell editor mounted"
    );
}

/// A Book's bar carries the extra "Pace" segment, so its Overview sits one further
/// along than a Chapter's or a Part's. Pinned because the index above is a magic
/// number that only the bar's construction order justifies.
#[cfg(feature = "mocks")]
#[test]
fn only_a_book_has_the_extra_book_only_segments() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let segments = |sub_role: BinderItemSubRole| {
        let tab = tab_for(
            &ctx,
            101,
            &Folder,
            &sub_role,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
        let bar = first_of_type(&tree, id, "SegmentedControl").expect("a bar");
        tree.children(bar).len()
    };
    let chapter = segments(ChapterScene);
    assert_eq!(
        segments(Part),
        chapter,
        "a Part and a Chapter offer the same views"
    );
    assert_eq!(
        segments(Book),
        chapter + 2,
        "a Book adds Pace and Analysis, which is why its Overview index is two higher"
    );
}

/// The Corkboard segment **mounts and lays out** its grid — the same positional
/// `SegmentedControl` ↔ `Switcher` contract the Overview test pins, one index
/// earlier. Without this, a segment inserted before Corkboard would silently
/// show Corkboard's grid under the new label (or Corkboard under the old one)
/// and still compile.
#[cfg(feature = "mocks")]
#[test]
fn the_corkboard_segment_mounts_a_grid() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    // (sub_role, the Corkboard's index in that container's bar) — always one
    // before the Overview, which `the_overview_segment_mounts_a_table` pins.
    // `Folder/Note` has neither (a subtree, but no manuscript extent).
    // The per-container Corkboard index table — (ChapterScene, 3), (Part, 3),
    // (Book, 5) — is gone for the same reason as the Overview one above: the segment
    // is addressed by id, so a Book's two extra segments no longer shift it.
    for sub_role in [ChapterScene, Part, Book] {
        let tab = tab_for(
            &ctx,
            101,
            &Folder,
            &sub_role,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        assert!(
            tab.corkboard().is_some(),
            "Folder/{sub_role:?} carries a Corkboard view-model"
        );
        tab.segment
            .set(Some(crate::tabs::shared::segments::segment_id(
                crate::tabs::shared::segments::SEG_CORKBOARD,
            )));
        // The board's wiring child subscribes to backend events, so a bare
        // `WidgetTree` would panic with no event source registered.
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
        let grid = first_containing(&tree, id, "GridView").unwrap_or_else(|| {
            panic!(
                "Folder/{sub_role:?}'s Corkboard segment mounted no GridView — \
                     the segment and its Switcher child have drifted out of step"
            )
        });
        let b = tree.bounds(grid);
        assert!(
            b.width > 0.0 && b.height > 0.0,
            "Folder/{sub_role:?}: the Corkboard grid laid out to zero size ({b:?})"
        );
    }
}

/// **A press inside a card's synopsis must not arm an ancestor's drag.**
///
/// The synopsis on a corkboard card is a live `RichTextEditor`, and selecting a
/// word is a press-move-release — the same gesture the `GridView` under it uses
/// to start a card drag. The editor selects through `on_pointer_event` rather
/// than a drag recognizer and returns `Ignored` on `PointerDown` (deliberately,
/// so its double/triple-tap recognizers keep working), while still *capturing*
/// the pointer for those recognizers. That combination is exactly what
/// `arm_drag_observers` walks past on its way to arming the draggable ancestor —
/// so the writer dragged the card while trying to select a word.
///
/// Composed here rather than driven through a real board: the mock
/// `OpenDocsStore` gives cards no synopsis field, so a mock corkboard renders
/// `Spacer`s where the editors would be and there is nothing to press. This
/// builds the **real** `card_synopsis_editor` under a **real** draggable
/// ancestor, which is the relationship in question.
///
/// Two editors, identical but for the wrapper, so the dead zone is the only
/// variable — and the bare one is the sensitivity control: if *it* stopped
/// arming, the assertion below would pass for the wrong reason.
#[test]
fn a_press_in_a_card_synopsis_does_not_arm_an_ancestor_drag() {
    use teksilo::canvas::Point;
    use teksilo::core::widget_builder::WidgetBuilder;
    use teksilo::prelude::PointerButton;
    use teksilo::text_document::TextDocument;

    fn synopsis() -> impl Widget {
        let doc = TextDocument::new();
        doc.set_plain_text("alpha bravo charlie delta echo")
            .unwrap();
        let typo = crate::settings::EditorTypography {
            font_family: Signal::new(String::new()),
            size: Signal::new(16.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
            size_range: crate::settings::TypographySizeRange::default(),
        };
        crate::tabs::shared::editor::card_synopsis_editor(
            doc,
            typo,
            || {},
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .0
    }

    let mut tree = WidgetTree::new();
    let card = tree.add(
        teksilo::widgets::VStack::new()
            // Control: a bare editor, as the card used to build it.
            .child(
                teksilo::widgets::FixedSize::new()
                    .height(90.0)
                    .child(synopsis()),
            )
            // Under test: what `CardSynopsis::build` composes now.
            .child(
                teksilo::widgets::FixedSize::new()
                    .height(90.0)
                    .child(teksilo::widgets::DeadZone::new().child(synopsis())),
            )
            .on_drag(|_phase, _ctx| {}),
    );
    tree.layout(teksilo::prelude::SizeProposal::exact(400.0, 200.0));
    let _ = tree.render();

    let bare = tree.child_bounds(card, 0);
    let guarded = tree.child_bounds(card, 1);
    assert!(
        bare.height > 0.0 && guarded.height > 0.0,
        "the editors laid out to zero size ({bare:?} / {guarded:?}), so the presses \
             below would be meaningless"
    );

    let press = |tree: &mut WidgetTree, r: teksilo::canvas::Rect| {
        let p = Point::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
        tree.pointer_down_button(p, PointerButton::Primary);
        let armed: Vec<_> = tree.armed_drag_observers().to_vec();
        tree.pointer_up_button(p, PointerButton::Primary);
        armed
    };

    let armed_bare = press(&mut tree, bare);
    let armed_guarded = press(&mut tree, guarded);

    assert!(
        !armed_bare.is_empty(),
        "control: a press in an unguarded synopsis must arm the card's drag — it \
             did not, so the assertion below would pass vacuously"
    );
    assert!(
        armed_guarded.is_empty(),
        "a press inside the dead-zoned synopsis armed {armed_guarded:?} — the writer \
             cannot select a word without also dragging the card"
    );
}

/// **A press inside a card's inline rename field must not arm an ancestor's drag.**
///
/// The last control on a card that was not a dead zone, and it fails exactly the way
/// the synopsis above did: selecting inside a `TextInput` is press-move-release, the
/// same gesture the `GridView` beneath uses to start a card drag, and the field selects
/// through `on_pointer_event` while returning `Ignored` on `PointerDown`. Dragging to
/// select a word in the title dragged the card instead, so the title could be replaced
/// but never edited with the mouse.
///
/// Two fields, identical but for the wrapper, so the dead zone is the only variable —
/// and the bare one is the sensitivity control.
#[test]
fn a_press_in_a_cards_rename_field_does_not_arm_the_card_drag() {
    use teksilo::core::event::PointerButton;
    use teksilo::prelude::*;

    let field =
        || teksilo::widgets::TextInput::new(Signal::new("Dans lequel Phileas Fogg".to_string()));

    let mut tree = WidgetTree::new();
    let card = tree.add(
        teksilo::widgets::VStack::new()
            // Control: a bare field, as the card used to build it.
            .child(field())
            // Under test: what `InlineTitle::build` composes now.
            .child(teksilo::widgets::DeadZone::new().child(field()))
            .on_drag(|_phase, _ctx| {}),
    );
    tree.layout(teksilo::prelude::SizeProposal::exact(400.0, 200.0));
    let _ = tree.render();

    let bare = tree.child_bounds(card, 0);
    let guarded = tree.child_bounds(card, 1);
    assert!(
        bare.height > 0.0 && guarded.height > 0.0,
        "the fields laid out to zero size ({bare:?} / {guarded:?}), so the presses \
         below would be meaningless"
    );

    let press = |tree: &mut WidgetTree, r: teksilo::canvas::Rect| {
        let p = Point::new(r.x + r.width / 2.0, r.y + r.height / 2.0);
        tree.pointer_down_button(p, PointerButton::Primary);
        let armed: Vec<_> = tree.armed_drag_observers().to_vec();
        tree.pointer_up_button(p, PointerButton::Primary);
        armed
    };

    let armed_bare = press(&mut tree, bare);
    let armed_guarded = press(&mut tree, guarded);

    assert!(
        !armed_bare.is_empty(),
        "control: a press in an unguarded rename field must arm the card's drag — it \
         did not, so the assertion below would pass vacuously"
    );
    assert!(
        armed_guarded.is_empty(),
        "a press inside the dead-zoned rename field armed {armed_guarded:?} — the \
         writer cannot select in the title without also dragging the card"
    );
}

/// A realized card still lays out after the dead-zone wrappers went in.
///
/// `DeadZone` is documented layout-transparent — it reports its child's size and
/// fills the child to its own bounds — but the card wraps four of them (synopsis,
/// expand, tag dots, kebab) inside a `CardColumn` whose middle slot is sized by
/// arithmetic on the tile height. This is the guard that the arithmetic still
/// lands on a real card rather than a collapsed one.
#[cfg(feature = "mocks")]
#[test]
fn a_realized_card_lays_out_with_its_dead_zone_wrappers() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    // 301 is the mock Part — the one container whose fixture yields cards.
    let tab = tab_for(
        &ctx,
        301,
        &Folder,
        &Part,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_CORKBOARD,
        )));
    let mut tree = crate::test_support::tree_with_events(&ctx);
    let root = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(1400.0, 900.0));
    // Tiles are virtualized — nothing exists under the grid until a render pass.
    let _ = tree.render();
    tree.layout(teksilo::prelude::SizeProposal::exact(1400.0, 900.0));

    let grid = first_containing(&tree, root, "GridView").expect("the board mounted a grid");
    let tile = first_containing(&tree, grid, "CorkboardTile")
        .expect("the mock Part's fixture realizes at least one card");
    let tb = tree.bounds(tile);
    assert!(
        tb.width > 100.0 && tb.height > 100.0,
        "a realized card collapsed to {tb:?} — a dead-zone wrapper is not being \
             sized transparently"
    );
    // And the kebab inside it, the smallest of the wrapped controls.
    let menu = first_containing(&tree, tile, "CardMenu").expect("the card carries its kebab");
    let mb = tree.bounds(menu);
    assert!(
        mb.width > 0.0 && mb.height > 0.0,
        "the card's kebab collapsed to {mb:?} inside its dead zone"
    );
}

/// A `Folder/Note` has an Overview but **no** Corkboard: it owns a subtree, yet
/// no manuscript extent, so there is nothing to lay out as cards.
#[test]
fn a_note_folder_has_no_corkboard() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Folder,
        &Note,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    assert!(
        tab.corkboard().is_none(),
        "a Folder/Note has a subtree but no manuscript extent — no Corkboard"
    );
    assert!(
        tab.overview().is_some(),
        "...but it does get an Overview, which is the wider gate"
    );
}

/// A tab with no Corkboard captures no board state, and seeding one is inert —
/// the workspace-restore path must not assume every tab has a board.
#[test]
fn a_tab_without_a_corkboard_captures_and_seeds_nothing() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    assert!(tab.capture_corkboard_state().is_none());
    tab.seed_corkboard_state(&[7, 8], &["a".into(), "b".into()], "q"); // must not panic
    assert!(tab.capture_corkboard_state().is_none());
}

/// A board sitting at its own container with nothing typed persists nothing —
/// so the overwhelmingly common case adds no bytes to `workspace.toml`.
#[cfg(feature = "mocks")]
#[test]
fn an_untouched_corkboard_captures_no_state() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        101,
        &Folder,
        &Book,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    assert!(
        tab.corkboard().is_some(),
        "precondition: a Book has a Corkboard"
    );
    assert!(
        tab.capture_corkboard_state().is_none(),
        "an untouched board has nothing worth persisting"
    );
}

/// Switching a container's view persists it per type, and a newly-opened tab of
/// the same type inherits it — the whole "remember last view" chain: the built
/// tab's `RememberSegment` effect writes `EditorViewMemory` on a segment change,
/// and `ContentTab::new` seeds a new tab's segment from it.
#[test]
fn switching_a_container_view_persists_and_a_new_tab_inherits() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let mem = crate::settings::EditorViewMemory::detached(true);
    let open = |id: u64| {
        tab_for(
            &ctx,
            id,
            &Folder,
            &ChapterScene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            mem.clone(),
            &AppIds::new(),
        )
    };
    // Open a chapter; it starts on its own page.
    let chapter1 = open(1);
    assert_eq!(
        chapter1.segment.get(),
        Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OWN
        ))
    );
    let mut tree = WidgetTree::new();
    tree.add_boxed(tab_pane(&chapter1)); // sets up the persist effect
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
    // Switch it to the manuscript stream ("Full Chapter").
    chapter1
        .segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_MANUSCRIPT,
        )));
    assert_eq!(
        mem.initial(&BinderItemRole::Folder, &ChapterScene),
        Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_MANUSCRIPT
        )),
        "the chosen view was remembered"
    );
    // A newly-opened chapter inherits it.
    assert_eq!(
        open(2).segment.get(),
        Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_MANUSCRIPT
        )),
        "a new chapter opens on Full Chapter"
    );
    // ...but a Scene (no segmented control) is unaffected.
    let scene = tab_for(
        &ctx,
        3,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        mem.clone(),
        &AppIds::new(),
    );
    // `None`, not "segment 0": a Scene has no segmented bar at all, and the keyed
    // signal can now say so. The old positional signal had to spell that as an index
    // into a control this tab does not have.
    assert_eq!(scene.segment.get(), Option::None);
}

/// Two open tabs of the same container type share one per-type memory: each of
/// their `SegmentedControl` switches writes it, and the *last switch wins* — a
/// tab's own switch never fights a peer's. This works only because `ctx.effect`
/// fires on *changes*, not on setup (proven by the counter-test above, which
/// starts each on its own page and confirms nothing is written until a switch):
/// so merely having a second same-type tab open — or a tab rebuilding — installs
/// observers that stay quiet, and only a real switch persists.
///
/// (Built at segment 0 / the own page throughout: a non-zero segment mounts the
/// manuscript-stream pane, whose event wiring needs an app-level event source
/// the headless `WidgetTree` doesn't provide — so the switches are made via the
/// signal, which fires the persist effect without swapping the mounted pane.)
#[test]
fn same_type_tabs_share_one_last_view_and_the_last_switch_wins() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let mem = crate::settings::EditorViewMemory::detached(true);
    let open = |id: u64| {
        tab_for(
            &ctx,
            id,
            &Folder,
            &ChapterScene,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            mem.clone(),
            &AppIds::new(),
        )
    };
    // Both open on their own page (memory starts at 0), so no stream mounts.
    let a = open(1);
    let b = open(2);
    assert_eq!(
        a.segment.get(),
        Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OWN
        ))
    );
    assert_eq!(
        b.segment.get(),
        Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OWN
        ))
    );
    let build = |tab: &ContentTab| {
        let mut tree = WidgetTree::new();
        tree.add_boxed(tab_pane(tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
        tree // keep alive so the effect it installed stays live
    };
    let _ta = build(&a);
    let _tb = build(&b);
    // Merely opening + building a second same-type tab wrote nothing.
    assert_eq!(
        mem.initial(&BinderItemRole::Folder, &ChapterScene),
        Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OWN
        ))
    );
    a.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_MANUSCRIPT,
        ))); // A switches → Full Chapter
    assert_eq!(
        mem.initial(&BinderItemRole::Folder, &ChapterScene),
        Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_MANUSCRIPT
        ))
    );
    b.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_SYNOPSIS,
        ))); // B → Full Synopsis: last switch wins
    assert_eq!(
        mem.initial(&BinderItemRole::Folder, &ChapterScene),
        Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_SYNOPSIS
        ))
    );
    a.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OWN,
        ))); // A switches back → its switch wins in turn
    assert_eq!(
        mem.initial(&BinderItemRole::Folder, &ChapterScene),
        Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OWN
        ))
    );
}

/// First node at/under `root` whose type name *contains* `needle` (DFS pre-order).
///
/// Separate from [`first_of_type`] because a generic widget's `type_name` carries its
/// parameters (`TreeTableView<..::OverviewRow>`), so a suffix match never fires on one.
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

// ── the margin lane, mounted ─────────────────────────────────────────────────

/// Build a real Scene tab's page in a tree that has a settings store, and hand
/// back the tree, its root and the tab.
///
/// `tree_with_settings` rather than `tree_with_events`: a lane reads the writer's
/// switches while building, and with no store to read it correctly draws nothing —
/// so a test on the plain harness would pass against a lane that never worked.
fn scene_page_with_settings() -> (WidgetTree, WidgetId, ContentTab) {
    scene_page_seeded("")
}

/// As [`scene_page_with_settings`], with prose already in the document.
///
/// Written **before** the tree is built, deliberately: an editor's text layout is
/// driven by the frame loop, and a document filled after the first frame does not
/// reflow from a bare `tree.layout()` — the page then measures at its `min_lines`
/// floor and reports no content height at all, which is indistinguishable from a
/// lane that is broken.
fn scene_page_seeded(text: &str) -> (WidgetTree, WidgetId, ContentTab) {
    let ctx = Rc::new(AppContext::new());
    let open_doc = Rc::new(OpenDoc::build(
        &ctx,
        1,
        &BinderItemRole::Item,
        &BinderItemSubRole::Scene,
        &[],
        Signal::new(0),
        std::path::Path::new(""),
    ));
    if let Some(field) = open_doc.main.as_ref().filter(|_| !text.is_empty()) {
        field.doc.set_plain_text(text).unwrap();
    }
    let tab = ContentTab::new(
        ctx.clone(),
        AppIds::new(),
        OpenDocsStore::new(ctx.clone()),
        open_doc,
        Signal::new(700.0),
        Signal::new(true),
        Signal::new(SynopsisPlacement::default()),
        Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
        test_typography(),
        crate::shared::TypewriterSettings::off(),
        crate::shared::CaretHighlightSettings::off(),
        crate::settings::EditorViewMemory::detached(false),
        crate::settings::CorkboardDefaults::detached(),
        crate::settings::TreeExpansionViewModel::new(
            ctx.clone(),
            AppIds::new(),
            crate::models::TreeExpansionService::in_memory_default(),
        ),
        Signal::new(false),
        Signal::new(crate::DISTRACTION_FREE_WIDTH_DEFAULT),
        crate::format::FormatViewModel::detached(),
        crate::writing_session::WritingGamesViewModel::detached(),
        crate::save::WorkHandle::detached(ctx.clone(), AppIds::new()),
        Signal::new(GoalUnit::default()),
        crate::tags::TagsViewModel::detached(ctx.clone(), AppIds::new()),
        crate::statuses::StatusesViewModel::new(ctx.clone(), AppIds::new()),
        crate::mentions::MentionIndex::new(ctx.clone(), AppIds::new()),
    );
    let mut tree = crate::test_support::tree_with_settings(&ctx);
    let root = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 400.0));
    let _ = tree.render();
    (tree, root, tab)
}

/// **The lane is actually on the page**, and it did not take the manuscript's
/// measure to get there.
///
/// The second half is the one that matters. A writing surface's column width is a
/// setting the writer chose and a number the whole application centres text on; a
/// strip that ate into it would move every line of every book by twelve pixels, and
/// nothing else in this suite would have noticed.
#[test]
fn a_writing_page_carries_a_margin_lane_without_narrowing_the_prose() {
    let (tree, root, _tab) = scene_page_with_settings();

    let lane = first_of_type(&tree, root, "MarginLane").expect("the page carries a lane");
    let lane_bounds = tree.bounds(lane);
    assert!(
        (lane_bounds.width - crate::widgets::DEFAULT_LANE_WIDTH).abs() < 0.01,
        "the lane took {} px, not its declared width",
        lane_bounds.width
    );
    assert!(lane_bounds.height > 0.0, "and it has height");

    let scroll = first_of_type(&tree, root, "ScrollArea").expect("the page scrolls");
    let scroll_bounds = tree.bounds(scroll);
    assert!(
        scroll_bounds.width > 900.0,
        "the prose side kept the page: the scroll area got {} of 1000",
        scroll_bounds.width
    );
    assert!(
        scroll_bounds.x + scroll_bounds.width <= lane_bounds.x + 0.01,
        "the lane belongs beside the page, not over it: page ends at {}, lane starts at {}",
        scroll_bounds.x + scroll_bounds.width,
        lane_bounds.x
    );
}

/// **The marks actually resolve on a tab**, which the test above cannot see: it
/// asserts the strip is there and the right width, and passes against a lane that
/// never produces a single mark.
///
/// This is where that gap showed. With the geometry left out of the recompute guard
/// the first frame had no laid-out text, the second was indistinguishable from it,
/// and the lane stayed empty for the life of the tab — twelve correct pixels of
/// nothing.
#[test]
fn a_tabs_lane_resolves_marks_against_the_prose_on_the_page() {
    use crate::margin_lane::{LaneQuery, LaneQuerySource, set_active_query};

    let _providers = crate::margin_lane::install_builtin_providers();

    // Paragraphs of very unequal length, with the searched word only in the long
    // final one: a lane placing marks by character fraction would spread them
    // further up the strip than the text goes.
    let mut text = String::new();
    for i in 0..20 {
        text.push_str(&format!("Line {i}.\n\n"));
    }
    text.push_str(&"the ferry left before the light did ".repeat(40));

    set_active_query(Some(LaneQuery {
        text: "ferry".into(),
        case_sensitive: false,
        whole_word: true,
        diacritic_sensitive: false,
        source: LaneQuerySource::Project,
        current: Option::None,
    }));

    let (mut tree, root, _tab) = scene_page_seeded(&text);
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 400.0));
    let _ = tree.render();

    let lane_id = first_of_type(&tree, root, "MarginLane").expect("the page carries a lane");
    let lane_bounds = tree.bounds(lane_id);
    let marks = tree
        .widget_as_any(lane_id)
        .and_then(|a| a.downcast_ref::<crate::widgets::MarginLane>())
        .expect("MarginLane opts into as_any")
        .resolve_marks(lane_bounds);

    // Counted through `merged`, not through the list length: forty hits inside one
    // paragraph are three pixels apart on a four-hundred-pixel strip, and the lane
    // folds anything under its minimum mark height into one mark that says how many
    // it stands for. That folding is the feature working, not a shortfall.
    let hits: usize = marks.iter().map(|m| m.merged).sum();
    assert!(
        hits >= 30,
        "forty occurrences of 'ferry' resolved to {hits} across {} marks",
        marks.len()
    );
    let highest = marks.iter().map(|m| m.top).fold(f32::INFINITY, f32::min);
    assert!(
        highest > lane_bounds.height * 0.3,
        "the marks belong where the word is, not where its character offsets are: \
         highest at {highest} of {}",
        lane_bounds.height
    );

    set_active_query(Option::None);
}

/// **A lane that is not drawn does no work**, which is a stronger claim than "draws
/// nothing" and the one the settings page's own wording makes.
///
/// `place_children` runs even for a widget with no children, so a host that only
/// stopped *building* the strip would go on resolving marks and walking documents
/// on every keystroke for a column that is not on screen. Asserted by watching the
/// marks the lane would have published: with the switch off they stay empty however
/// much the page is laid out.
#[test]
fn a_lane_the_writer_turned_off_stops_resolving_as_well_as_drawing() {
    use crate::margin_lane::{LaneQuery, LaneQuerySource, set_active_query};

    let _providers = crate::margin_lane::install_builtin_providers();
    let mut text = String::new();
    for i in 0..20 {
        text.push_str(&format!("the ferry left before the light did, {i}.\n\n"));
    }
    set_active_query(Some(LaneQuery {
        text: "ferry".into(),
        case_sensitive: false,
        whole_word: true,
        diacritic_sensitive: false,
        source: LaneQuerySource::Project,
        current: Option::None,
    }));

    let (mut tree, root, _tab) = scene_page_seeded(&text);
    let store = tree
        .app_context()
        .app_state::<teksilo::settings::SettingsStore>()
        .expect("the harness registered a store")
        .clone();

    // On: the marks are there.
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 400.0));
    let _ = tree.render();
    let lane_id = first_of_type(&tree, root, "MarginLane").expect("the page carries a lane");
    let resolved = |tree: &WidgetTree, id| {
        tree.widget_as_any(id)
            .and_then(|a| a.downcast_ref::<crate::widgets::MarginLane>())
            .map(|l| l.resolve_marks(tree.bounds(id)).len())
            .unwrap_or(0)
    };
    assert!(resolved(&tree, lane_id) > 0, "the marks were there to lose");

    // Off, and then laid out again: the whole column goes, so nothing is left to
    // resolve against.
    store
        .signal(
            crate::MARGIN_LANE_ENABLED_KEY,
            crate::MARGIN_LANE_ENABLED_DEFAULT,
        )
        .set(false);
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 400.0));
    let _ = tree.render();
    assert!(
        first_of_type(&tree, root, "MarginLane").is_none(),
        "the strip outlived the switch"
    );

    set_active_query(Option::None);
}

/// The View menu's switch is a live preference, not a construction-time decision.
///
/// A writer flipping it must see the strip go, without the tab being rebuilt for
/// some unrelated reason first.
#[test]
fn turning_the_lane_off_takes_the_column_off_the_page() {
    let (mut tree, root, _tab) = scene_page_with_settings();
    assert!(first_of_type(&tree, root, "MarginLane").is_some());

    let store = tree
        .app_context()
        .app_state::<teksilo::settings::SettingsStore>()
        .expect("the harness registered a store")
        .clone();
    store
        .signal(
            crate::MARGIN_LANE_ENABLED_KEY,
            crate::MARGIN_LANE_ENABLED_DEFAULT,
        )
        .set(false);
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 400.0));

    assert!(
        first_of_type(&tree, root, "MarginLane").is_none(),
        "the strip outlived the switch"
    );
}

/// A Scene tab built with the given typewriter setting, laid out, plus the
/// page `ScrollArea`'s maximum scroll offset.
///
/// The pin lives on the editors, but the *range* that lets the last line
/// reach it lives on the page — this is the link between them.
fn scene_page_max_scroll(typewriter: crate::shared::TypewriterSettings) -> (f32, f32) {
    use teksilo::widgets::ScrollArea;
    let ctx = Rc::new(AppContext::new());
    let open_doc = Rc::new(OpenDoc::build(
        &ctx,
        1,
        &BinderItemRole::Item,
        &BinderItemSubRole::Scene,
        &[],
        Signal::new(0),
        std::path::Path::new(""),
    ));
    let tab = ContentTab::new(
        ctx.clone(),
        AppIds::new(),
        OpenDocsStore::new(ctx.clone()),
        open_doc,
        Signal::new(700.0),
        Signal::new(true),
        Signal::new(SynopsisPlacement::default()),
        Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
        test_typography(),
        typewriter,
        crate::shared::CaretHighlightSettings::off(),
        crate::settings::EditorViewMemory::detached(false),
        crate::settings::CorkboardDefaults::detached(),
        crate::settings::TreeExpansionViewModel::new(
            ctx.clone(),
            AppIds::new(),
            crate::models::TreeExpansionService::in_memory_default(),
        ),
        Signal::new(false),
        Signal::new(crate::DISTRACTION_FREE_WIDTH_DEFAULT),
        crate::format::FormatViewModel::detached(),
        crate::writing_session::WritingGamesViewModel::detached(),
        crate::save::WorkHandle::detached(ctx.clone(), AppIds::new()),
        Signal::new(GoalUnit::default()),
        crate::tags::TagsViewModel::detached(ctx.clone(), AppIds::new()),
        crate::statuses::StatusesViewModel::new(ctx.clone(), AppIds::new()),
        crate::mentions::MentionIndex::new(ctx.clone(), AppIds::new()),
    );
    let mut tree = crate::test_support::tree_with_events(&ctx);
    let root = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 400.0));
    let sa_id = first_of_type(&tree, root, "ScrollArea").expect("the page scrolls");
    let max_scroll = tree
        .widget_as_any(sa_id)
        .and_then(|a| a.downcast_ref::<ScrollArea>())
        .expect("ScrollArea opts into as_any")
        .max_scroll_y_signal()
        .get();
    (max_scroll, tree.bounds(sa_id).height)
}

/// With typewriter scrolling on, the writing page must be able to scroll
/// *past* its last line — otherwise the pin silently stops working over the
/// final page, which is exactly where a writer spends their time. With it
/// off, the page must stop at its content like any other.
///
/// This is the one link the unit tests either side of it cannot cover: that
/// every writing surface really does go through `writing_page_scroll`.
#[test]
fn the_writing_page_buys_scroll_range_only_while_pinning() {
    use crate::shared::{TypewriterAnchor, TypewriterSettings};

    let on = |a: TypewriterAnchor| {
        scene_page_max_scroll(TypewriterSettings::new(
            Signal::new(true),
            Signal::new(Some(a)),
        ))
    };

    let (off, _) = scene_page_max_scroll(TypewriterSettings::off());
    let (middle, viewport) = on(TypewriterAnchor::Middle);
    let (top_third, _) = on(TypewriterAnchor::TopThird);
    let (bottom_quarter, _) = on(TypewriterAnchor::BottomQuarter);

    assert!(
        middle > off,
        "pinning must buy scroll range past the last line (off={off}, on={middle})"
    );

    // A higher pin needs more room beneath it, so the range grows as the
    // anchor rises. Ordering alone would pass on a constant offset — the
    // exact differences below are what tie the range to the anchor.
    assert!(top_third > middle && middle > bottom_quarter);

    // Each enabled page differs from the next by exactly the difference in
    // their scroll-past-end fractions, scaled by the viewport. Asserted as a
    // *difference* so it holds whatever the tab chrome leaves for content —
    // an absolute expectation would encode this tab's incidental layout.
    let expected = |a: TypewriterAnchor, b: TypewriterAnchor| {
        (a.scroll_past_end() - b.scroll_past_end()) * viewport
    };
    assert!(
        (top_third - middle - expected(TypewriterAnchor::TopThird, TypewriterAnchor::Middle)).abs()
            < 0.5,
        "top-third vs middle: got {}, expected {}",
        top_third - middle,
        expected(TypewriterAnchor::TopThird, TypewriterAnchor::Middle)
    );
    assert!(
        (middle
            - bottom_quarter
            - expected(TypewriterAnchor::Middle, TypewriterAnchor::BottomQuarter))
        .abs()
            < 0.5,
        "middle vs bottom-quarter: got {}, expected {}",
        middle - bottom_quarter,
        expected(TypewriterAnchor::Middle, TypewriterAnchor::BottomQuarter)
    );
}

/// The Book's Pace dashboard **must reflow with width**. `pace_pane` drops the
/// prose-column `centered()` wrapper and flows its section panels through a `ColumnFlow`
/// under the scroll viewport, with the empty/planner swap done by a *layout-forwarding*
/// composing widget (`PaceBody`), not a `Switcher`. This pins that the resulting chain —
/// `ScrollArea > Padding > VStack > (forwarding swap) > ColumnFlow` — carries the
/// viewport's **bounded** width all the way to the flow, so it packs into several short
/// columns when wide and one tall column when narrow.
///
/// Two traps this guards, both of which reported one column at *every* width: the old
/// `centered()` measured the content at its **hugging** width, and a `Switcher` measures
/// *all* its children with an **unbounded** width to size to the largest. Five 200px
/// panels: wide ≈ two rows, narrow ≈ five.
#[test]
fn column_flow_reflows_in_the_pace_scaffold() {
    use teksilo::widgets::{ColumnFlow, FixedSize, Padding, RectWidget, ScrollArea, VStack};

    let flow_height = |width: f32| -> f32 {
        let mut flow = ColumnFlow::new()
            .min_column_width(300.0)
            .max_columns(3)
            .column_spacing(12.0)
            .item_spacing(12.0);
        for _ in 0..5 {
            flow = flow.child(FixedSize::new().height(200.0).child(RectWidget::new()));
        }
        // `Forwarder` stands in for `pace_pane`'s `PaceBody` swap: it forwards layout to
        // its one child, so (unlike a `Switcher`) the parent's bounded width reaches it.
        let body = VStack::new().child(Forwarder {
            child: Some(Box::new(flow)),
            root: None,
        });
        let scaffold = ScrollArea::new().child(Padding::symmetric(0.0, 24.0).child(body));

        let mut tree = WidgetTree::new();
        let root = tree.add_boxed(Box::new(scaffold));
        tree.layout(teksilo::prelude::SizeProposal::exact(width, 4000.0));
        first_of_type(&tree, root, "ColumnFlow")
            .map(|id| tree.bounds(id).height)
            .expect("the scaffold contains a ColumnFlow")
    };

    let wide = flow_height(1200.0);
    let narrow = flow_height(360.0);
    assert!(
        wide > 0.0 && narrow > 0.0,
        "scaffold laid out (wide {wide}, narrow {narrow})"
    );
    // Wide packs five panels into three columns (≈ two rows); narrow stacks all five.
    // A generous margin, not a pixel assertion — it only has to have reflowed at all.
    assert!(
        wide * 1.5 < narrow,
        "wide dashboard ({wide}px) must reflow far shorter than narrow ({narrow}px) — \
             the viewport's bounded width did not reach the ColumnFlow"
    );
}

/// A one-child composing widget that forwards its layout to that child — the layout
/// shape of `pace_pane`'s `PaceBody`. Proves the empty/planner swap does not, unlike a
/// `Switcher`, drop the parent's bounded width proposal.
struct Forwarder {
    child: Option<Box<dyn Widget>>,
    root: Option<WidgetId>,
}
impl std::fmt::Debug for Forwarder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Forwarder").finish()
    }
}
impl Widget for Forwarder {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let id = ctx.add_boxed(self.child.take().expect("Forwarder built once"));
        self.root = Some(id);
        vec![id]
    }
    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// A writing item exposes the fields the matrix allows: a Scene gets main +
/// synopsis prose; a ChapterScene adds a title; a BookBegin gets two titles
/// plus the book's synopsis (symmetric with the Folder/Book container).
#[test]
fn tab_for_loads_allowed_fields() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let mk = |sr: BinderItemSubRole| {
        tab_for(
            &ctx,
            1,
            &Item,
            &sr,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        )
    };
    let scene = mk(Scene);
    assert!(scene.main().is_some() && scene.synopsis().is_some() && scene.title().is_none());

    let cs = mk(ChapterScene);
    assert!(cs.main().is_some() && cs.synopsis().is_some() && cs.title().is_some());

    let bb = mk(BookBegin);
    assert!(
        bb.title().is_some()
            && bb.subtitle().is_some()
            && bb.synopsis().is_some()
            && bb.main().is_none()
    );

    let end = mk(BookEnd);
    assert!(end.main().is_none() && end.synopsis().is_none() && end.title().is_none());
}

/// A tab hands out the **same** backend handles it was built with.
///
/// This is what makes the `container.segments` slot usable at all. A
/// registered segment receives nothing but the tab, and an extension cannot
/// capture an `AppContext` at registration time — the app builds its own
/// inside `run`, long afterwards, so a captured one is a second, permanently
/// empty store. A segment reading through it would render a convincing
/// "nothing here" forever, with no error and nothing to grep for.
#[test]
fn a_tab_publishes_the_backend_handles_it_was_built_with() {
    let ctx = Rc::new(AppContext::new());
    let ids = AppIds::new();
    ids.work_id.set(Some(4242));
    let tab = tab_for(
        &ctx,
        1,
        &BinderItemRole::Item,
        &BinderItemSubRole::Scene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &ids,
    );

    assert!(
        Rc::ptr_eq(&tab.app_ctx(), &ctx),
        "the tab handed back a different AppContext than it was built with"
    );
    assert_eq!(
        tab.ids().work_id.get(),
        Some(4242),
        "the tab handed back a different AppIds than it was built with"
    );
    // And it is the live handle, not a snapshot: an id set afterwards has to
    // be visible through it, or a segment built early would read a stale work.
    ids.work_id.set(Some(99));
    assert_eq!(tab.ids().work_id.get(), Some(99));
}

/// The widest right edge anywhere under `id`, with the widget that owns it — the
/// name matters, because "something overflows" is useless without "what".
fn widest_right(tree: &WidgetTree, id: WidgetId) -> (f32, String) {
    let mut worst = (
        tree.bounds(id).right(),
        tree.widget_type_name(id).unwrap_or("?").to_string(),
    );
    for child in tree.children(id) {
        let got = widest_right(tree, child);
        if got.0 > worst.0 {
            worst = got;
        }
    }
    worst
}

/// **The writing column must shrink with the window, not overflow it.**
///
/// The column grows up to the width set in Settings, but below that it has to
/// follow the window down. It did not: `centered()` laid its cap out inside an
/// `HStack` + `Spacer`, and an alignment parent measures its child with an
/// **unbounded** proposal — so `MaxSize` always reported its full cap and never
/// shrank. Narrow the window under the column width and the tab overflowed to the
/// right by the difference, for the entire height of the document.
///
/// That overhang is what froze the app: the inspector painted hazard stripes over
/// an overflow strip as tall as the whole scene, and a single 45° band across it
/// became a 7573x7563 path — a 229 MB rasterization too big for the path atlas to
/// store, so it was rebuilt and thrown away on every frame at 100% CPU. The
/// renderer and the overlay are both hardened now, but the layout is where the
/// absurd geometry was born, so it is pinned here too.
#[test]
fn a_window_narrower_than_the_column_does_not_overflow() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;

    let ctx = Rc::new(AppContext::new());
    // The settings column cap is far wider than the window we lay out in.
    const CAP: f32 = 700.0;
    const BOX_W: f32 = 300.0;

    for (role, sub_role) in [
        (Item, Scene),
        (Item, ChapterScene),
        (Item, Note),
        (Folder, Book),
        (Folder, ChapterScene),
    ] {
        let tab = tab_for(
            &ctx,
            1,
            &role,
            &sub_role,
            &[],
            Signal::new(CAP),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        // A real text backend is required for a faithful narrow-window
        // check: `single_line` labels (the segment bar) only report a shrink
        // weight — so an over-constrained stack truncates them with an
        // ellipsis instead of overflowing — through the real text-layout
        // path. The no-backend 8px/char fallback returns a rigid size, so
        // the bar's labels would spill exactly as they never do in the live
        // app (which always has a backend).
        let mut tree = WidgetTree::new().with_text_backend(std::rc::Rc::new(
            std::cell::RefCell::new(teksilo::canvas::MockTextBackend::new()),
        ));
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(BOX_W, 700.0));

        let (right, who) = widest_right(&tree, id);
        assert!(
            right <= BOX_W + 0.5,
            "{role:?}/{sub_role:?}: `{who}` reaches x={right} in a {BOX_W}px window \
                 (column cap {CAP}) — the writing column must shrink with the window, \
                 not overhang it by {:.0}px for the full height of the scene",
            right - BOX_W
        );
    }
}

/// …but it stops shrinking at a floor, so the column never collapses to nothing.
#[test]
fn the_writing_column_shrinks_no_further_than_its_floor() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;

    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    let mut tree = WidgetTree::new();
    let id = tree.add_boxed(tab_pane(&tab));
    // Absurdly narrow — far below the floor.
    tree.layout(teksilo::prelude::SizeProposal::exact(20.0, 400.0));

    let (right, _who) = widest_right(&tree, id);
    assert!(
        right >= shared::editor::MIN_COLUMN_WIDTH - 0.5,
        "the column collapsed to {right}px; it must bottom out at \
             {}px rather than shrink to nothing",
        shared::editor::MIN_COLUMN_WIDTH
    );
}

/// Every **on-screen** prose editor's rect, in tree order.
///
/// Two filters, both load-bearing. Dormant subtrees are skipped: a `Switcher`
/// keeps the page it switched away from mounted, and a parked node still
/// reports the bounds it had when it was last laid out — so a naive walk sees
/// the Top *and* Side layouts at once and counts four editors where the writer
/// sees two. And recursion stops at an editor, because each one nests its own
/// padded viewport under the same type name and would otherwise be counted
/// twice.
fn editor_rects(tree: &WidgetTree, id: WidgetId, out: &mut Vec<teksilo::prelude::Rect>) {
    if !tree.is_active(id) {
        return;
    }
    let bounds = tree.bounds(id);
    let is_editor = tree
        .widget_type_name(id)
        .is_some_and(|t| t.contains("RichTextEditor"));
    if is_editor && bounds.height > 0.0 && bounds.width > 0.0 {
        out.push(bounds);
        return;
    }
    for c in tree.children(id) {
        editor_rects(tree, c, out);
    }
}

/// Mount a Side-placed scene at `width` and report the laid-out editor rects.
///
/// Settling takes more than one pass on purpose. The breakpoint is decided
/// during layout and published to a signal the `Switcher` consumes as a
/// *deferred* rebuild, and showing a splitter pane is an animated tween — so a
/// single `layout()` reads a half-open divider, which is exactly the trap a
/// test written by analogy to the instant `VisibleWhen` hide would fall into.
fn side_scene_editors(width: f32, collapsed: bool) -> Vec<teksilo::prelude::Rect> {
    use BinderItemRole::*;
    use BinderItemSubRole::*;

    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.synopsis_placement
        .set(crate::shared::SynopsisPlacement::Side);
    if collapsed {
        tab.side_splitter
            .set_collapsed(crate::tabs::shared::editor::SYNOPSIS_PANE, true);
    }

    let mut tree = WidgetTree::new();
    let id = tree.add_boxed(tab_pane(&tab));
    for _ in 0..4 {
        tree.layout(teksilo::prelude::SizeProposal::exact(width, 700.0));
    }
    tree.tick_animations(std::time::Duration::from_millis(400));
    tree.layout(teksilo::prelude::SizeProposal::exact(width, 700.0));

    let mut rects = Vec::new();
    editor_rects(&tree, id, &mut rects);
    rects
}

/// Side placement puts the synopsis in its own column **beside** the prose —
/// the whole point of the setting.
#[test]
fn side_placement_seats_the_synopsis_left_of_the_manuscript() {
    let rects = side_scene_editors(1200.0, false);
    assert_eq!(
        rects.len(),
        2,
        "a Side scene lays out both its synopsis and its prose, got {rects:?}"
    );
    let (synopsis, prose) = (rects[0], rects[1]);
    assert!(
        synopsis.x < prose.x,
        "the synopsis column must sit to the left of the manuscript \
             (synopsis at x={}, prose at x={})",
        synopsis.x,
        prose.x
    );
    assert!(
        synopsis.x + synopsis.width <= prose.x + 1.0,
        "the two columns must not overlap — the divider separates them"
    );
}

/// …but only where there is room for it. The editor can be split down to a
/// 320px pane, and a 280px synopsis taken out of that leaves a prose column
/// nobody can write in. Below the threshold the tab renders Top instead —
/// stacked, not side by side — rather than honouring the setting into
/// uselessness.
#[test]
fn a_pane_too_narrow_for_two_columns_falls_back_to_the_top_layout() {
    let rects = side_scene_editors(460.0, false);
    assert_eq!(
        rects.len(),
        2,
        "the synopsis is still shown — only its placement changed, got {rects:?}"
    );
    let (synopsis, prose) = (rects[0], rects[1]);
    assert!(
        (synopsis.x - prose.x).abs() < 60.0,
        "the fallback must stack the two in one column, not seat them side by \
             side (synopsis at x={}, prose at x={})",
        synopsis.x,
        prose.x
    );
    assert!(
        synopsis.y < prose.y,
        "stacked means the synopsis is above the prose"
    );
}

/// Count the laid-out splitter dividers under `id`.
fn gutters(tree: &WidgetTree, id: WidgetId, out: &mut usize) {
    if !tree.is_active(id) {
        return;
    }
    if tree
        .widget_type_name(id)
        .is_some_and(|t| t.contains("SplitterHandleBody"))
        && tree.bounds(id).width > 0.0
    {
        *out += 1;
    }
    for c in tree.children(id) {
        gutters(tree, c, out);
    }
}

/// The Side pane's own fold-away control gives the width back to the
/// manuscript — **and leaves a way to get it back.**
///
/// The first cut of this folded with `set_pane_visible(false)`, which removes
/// the pane *and its divider*. The only control that could restore the column
/// lived inside the column, so folding it was a one-way door: the writer was
/// left with no affordance at all and no way back short of the Settings window.
/// Folding is a **collapse** instead, which is the framework's other state
/// precisely because it keeps the divider — draggable, double-clickable and
/// keyboard-reachable — as the way back.
#[test]
fn folding_the_side_column_leaves_the_divider_as_the_way_back() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;

    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.synopsis_placement
        .set(crate::shared::SynopsisPlacement::Side);

    let mut tree = WidgetTree::new();
    let root = tree.add_boxed(tab_pane(&tab));
    // Collapsing a pane is an animated tween, and the breakpoint decision is
    // published from layout for the `Switcher` to pick up on the *next* pass —
    // so settling means interleaving layouts and clock ticks until both have
    // finished, not one of each.
    //
    // Run to a **fixed point**, not a fixed number of rounds. How many passes
    // this takes is not a property of the tween: the breakpoint decision is
    // published from layout for the `Switcher` to pick up on the *next* pass,
    // and any pass spent absorbing a backend event that lands mid-settle is a
    // pass neither of them gets. A hard-coded count is therefore a guess about
    // machine load, and this one guessed wrong often enough to fail the suite
    // outright once the runner filled all 24 cores.
    //
    // Idle animations *and* unchanged geometry, floored by `MIN_ROUNDS` so a
    // tween that has not started yet is never mistaken for one that has
    // finished. `MAX_ROUNDS` only bounds genuine non-convergence: hitting it
    // leaves the geometry unsettled and fails exactly as a wrong layout would.
    //
    // Frame-sized steps are safe again. They were not while `WidgetTree::layout`
    // stamped a newly armed animation on the wall clock and `tick_animations`
    // ticked the scheduler on a simulated one — a tween then only progressed
    // while simulated time stayed ahead of real time, and froze outright when
    // it did not. That was a framework bug, and it is fixed in teksilo
    // (`WidgetTree::animation_clock`) rather than papered over here with
    // implausibly large time steps.
    let settle = |tree: &mut WidgetTree| {
        const MIN_ROUNDS: usize = 8;
        const MAX_ROUNDS: usize = 64;
        let mut previous = String::new();
        for round in 0..MAX_ROUNDS {
            tree.layout(teksilo::prelude::SizeProposal::exact(1200.0, 700.0));
            tree.tick_animations(std::time::Duration::from_millis(120));
            let mut rects = Vec::new();
            editor_rects(tree, root, &mut rects);
            let current = format!("{rects:?}");
            if round >= MIN_ROUNDS && !tree.has_active_animations() && current == previous {
                break;
            }
            previous = current;
        }
        tree.layout(teksilo::prelude::SizeProposal::exact(1200.0, 700.0));
    };
    let survey = |tree: &WidgetTree| {
        let (mut rects, mut n) = (Vec::new(), 0);
        editor_rects(tree, root, &mut rects);
        gutters(tree, root, &mut n);
        (rects, n)
    };

    settle(&mut tree);
    let (open, open_gutters) = survey(&tree);
    assert_eq!(open.len(), 2, "open: synopsis + manuscript");
    assert_eq!(open_gutters, 1, "one divider between the two columns");

    tab.side_splitter
        .set_collapsed(crate::tabs::shared::editor::SYNOPSIS_PANE, true);
    settle(&mut tree);
    let (folded, folded_gutters) = survey(&tree);
    assert_eq!(
        folded.len(),
        1,
        "folded: the synopsis column is gone, got {folded:?}"
    );
    assert_eq!(
        folded_gutters, 1,
        "the divider must SURVIVE the fold — it is the only thing left to \
             pull the column back with"
    );
    // The writing column is width-capped by design, so it does not get *wider*
    // — it re-centres in the space the synopsis gave back.
    assert!(
        folded[0].x < open[1].x,
        "the manuscript reclaims the space and re-centres (x {} -> {})",
        open[1].x,
        folded[0].x
    );

    tab.side_splitter
        .set_collapsed(crate::tabs::shared::editor::SYNOPSIS_PANE, false);
    settle(&mut tree);
    let (restored, _) = survey(&tree);
    assert_eq!(
        restored.len(),
        2,
        "and dragging it back open restores the column, got {restored:?}"
    );
}

/// Dragging the divider persists the new width; the app's own show/hide
/// bookkeeping does not.
///
/// The divider's `version` signal is one coarse notification bumped by every
/// mutation there is — including the two the show/hide dance makes on every
/// toggle. Writing back on every bump would let hiding the synopsis overwrite
/// the width the writer chose with whatever the model happened to hold
/// mid-dance, so the next tab would open at a width nobody picked.
#[test]
fn only_a_real_drag_persists_the_synopsis_column_width() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;

    let ctx = Rc::new(AppContext::new());
    let show = Signal::new(true);
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        show.clone(),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.synopsis_placement
        .set(crate::shared::SynopsisPlacement::Side);
    let stored = tab.synopsis_side_width.clone();

    let mut tree = WidgetTree::new();
    tree.add_boxed(tab_pane(&tab));
    let settle = |tree: &mut WidgetTree| {
        for _ in 0..4 {
            tree.layout(teksilo::prelude::SizeProposal::exact(1200.0, 700.0));
        }
        tree.tick_animations(std::time::Duration::from_millis(400));
    };
    settle(&mut tree);
    assert_eq!(
        stored.get(),
        crate::SYNOPSIS_SIDE_WIDTH_DEFAULT,
        "merely showing the column is not the writer choosing a width"
    );

    // What a drag does to the model.
    tab.side_splitter.set_stored_size(0, 350.0);
    assert_eq!(stored.get(), 350.0, "a drag is the one thing that persists");

    // …and the dance that runs when the synopsis is folded away must leave it.
    show.set(false);
    settle(&mut tree);
    assert_eq!(
        stored.get(),
        350.0,
        "hiding the column must not overwrite the width the writer chose"
    );

    show.set(true);
    settle(&mut tree);
    assert_eq!(stored.get(), 350.0, "nor must showing it again");
}

/// A synopsis spell session is awake exactly while some mounted view is
/// showing it — no more, and no less.
///
/// This used to be one global flag pushed into every open document, which was
/// the right shape only while "is the synopsis visible?" had a single
/// app-wide answer. It no longer does: a tab can fold its Side synopsis away
/// on its own, and distraction-free mode has its own toggle. So the question
/// became a count, and the thing that must not happen is a **leak** — a tab
/// closed while its synopsis was up, pinning a session awake for the rest of
/// the session with nothing on screen to justify it.
#[test]
fn a_synopsis_session_sleeps_unless_a_mounted_view_is_showing_it() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;

    let ctx = Rc::new(AppContext::new());
    let show = Signal::new(true);
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        show.clone(),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    let doc = tab.open_doc.clone();
    let layout =
        |tree: &mut WidgetTree| tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 700.0));

    assert_eq!(
        doc.synopsis_viewers(),
        0,
        "a document nobody has mounted yet is not being shown"
    );

    let mut tree = WidgetTree::new();
    tree.add_boxed(tab_pane(&tab));
    layout(&mut tree);
    assert_eq!(doc.synopsis_viewers(), 1, "the mounted pane shows it");

    show.set(false);
    layout(&mut tree);
    assert_eq!(doc.synopsis_viewers(), 0, "hidden — the session may sleep");

    show.set(true);
    layout(&mut tree);
    assert_eq!(doc.synopsis_viewers(), 1, "shown again — awake again");

    drop(tree);
    assert_eq!(
        doc.synopsis_viewers(),
        0,
        "a pane torn down while the synopsis was showing must release its \
             claim — otherwise closing a tab pins the session awake forever"
    );
}

/// The same document open twice (both panes of a split, or a pane plus the
/// distraction-free surface) is shown once as far as its spell session is
/// concerned — and closing *one* of the two must not put it to sleep while the
/// other is still displaying it.
#[test]
fn two_views_of_one_document_count_as_one_awake_synopsis() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;

    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    let doc = tab.open_doc.clone();

    let mut both = WidgetTree::new();
    both.add_boxed(tab_pane(&tab));
    both.add_boxed(tab_pane(&tab));
    both.layout(teksilo::prelude::SizeProposal::exact(900.0, 700.0));
    assert_eq!(doc.synopsis_viewers(), 2, "two mounted views, two claims");

    drop(both);
    assert_eq!(doc.synopsis_viewers(), 0);
}

/// Every tab is born with a Side-synopsis divider, and it must be **weightless**
/// until something shows it.
///
/// A `Splitter` folds every pane's `min_size` into its own intrinsic minimum
/// whether or not that pane is visible. So a synopsis pane parked at a real
/// minimum would put a floor under the width of *every* tab — including the Top
/// ones that never draw it, and including a secondary editor pane that is only
/// 320px wide to begin with. Hidden means `min_size` 0; the width is raised only
/// while the pane is actually on screen.
#[test]
fn a_new_tabs_side_divider_starts_hidden_and_weightless() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;

    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    let m = &tab.side_splitter;

    assert_eq!(m.pane_count(), 2, "synopsis | manuscript");
    assert!(
        !m.is_pane_visible(0),
        "the synopsis pane must start hidden — placement is Top by default"
    );
    assert_eq!(
        m.min_size(0),
        0.0,
        "a hidden synopsis pane must contribute no minimum, or it widens every tab"
    );
    assert!(
        m.min_size(1) > 0.0,
        "the manuscript pane keeps a real floor so Side can never squeeze it away"
    );
    assert_eq!(
        m.stored_size(0),
        crate::SYNOPSIS_SIDE_WIDTH_DEFAULT,
        "hidden, but seeded at the persisted width so showing it opens where the \
             writer left it"
    );
    assert!(
        m.is_collapsible(0),
        "the synopsis pane must be collapsible, or folding it away would leave \
             no divider to pull it back with"
    );
    assert!(!m.is_collapsed(0), "a fresh tab has not been folded away");
}

/// Hiding the synopsis pane must give its space back to the prose.
///
/// It used to be a `Switcher` (which parks a zero-size page); it is now
/// `VisibleWhen`, which sends the node *dormant* — out of layout entirely. This
/// pins the property that actually matters to the writer: with the pane off, the
/// tab is shorter by the height of the synopsis, rather than leaving a gap where
/// it used to be.
#[test]
fn hiding_the_synopsis_pane_reclaims_its_height() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;

    let ctx = Rc::new(AppContext::new());
    let height_with = |show: bool| {
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &Scene,
            &[],
            Signal::new(700.0),
            Signal::new(show),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(tab_pane(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 700.0));

        // Count the editors that actually take up space. The tab always fills its
        // 700px box (the outer ScrollArea fills), so the tab's own height says
        // nothing; what changes is whether the synopsis editor is laid out at all.
        fn editors_with_height(tree: &WidgetTree, id: WidgetId, n: &mut usize) {
            let is_editor = tree
                .widget_type_name(id)
                .is_some_and(|t| t.contains("RichTextEditor"));
            if is_editor && tree.bounds(id).height > 0.0 {
                *n += 1;
            }
            for c in tree.children(id) {
                editors_with_height(tree, c, n);
            }
        }
        let mut n = 0;
        editors_with_height(&tree, id, &mut n);
        n
    };

    let shown = height_with(true);
    let hidden = height_with(false);
    assert!(
        hidden > 0,
        "the prose editor must still be laid out with the synopsis hidden"
    );
    assert!(
        hidden < shown,
        "{shown} editor nodes take space with the synopsis shown and {hidden} with it \
             hidden — hiding it must send the pane DORMANT and drop it out of layout, not \
             park an empty row where it used to be"
    );
}

/// The editor half of "one title, two homes": typing a name into a container's own
/// page and committing it (blur / Enter) must reach **both** `BinderItem.title` —
/// what the outline tree and the tab caption show — and the title `Content` row that
/// compiles into the manuscript.
///
/// It used to write only the content row, which is why renaming a chapter in its
/// editor left the tree and the tab showing the old name.
#[cfg(not(feature = "mocks"))]
#[test]
fn committing_a_title_reaches_both_of_its_homes() {
    use frontend::commands::{binder_commands, binder_item_commands, content_commands};
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};

    let ctx = Rc::new(AppContext::new());
    let work = frontend::commands::work_commands::create_orphan_work(
        &ctx,
        None,
        &CreateWorkDto::default(),
    )
    .unwrap();
    let binder = binder_commands::create_binder(
        &ctx,
        None,
        &CreateBinderDto {
            name: "B".into(),
            activated: true,
            ..Default::default()
        },
        work.id,
        0,
    )
    .unwrap();
    let item = binder_item_commands::create_binder_item(
        &ctx,
        None,
        &CreateBinderItemDto {
            status: None,
            title: "Old name".into(),
            role: BinderItemRole::Folder,
            sub_role: BinderItemSubRole::ChapterScene,
            activated: true,
            is_exportable: true,
            ..Default::default()
        },
        binder.id,
        -1,
    )
    .unwrap();

    let tab = tab_for(
        &ctx,
        item.id,
        &BinderItemRole::Folder,
        &BinderItemSubRole::ChapterScene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    let title = tab.title().expect("a chapter folder has a title field");
    assert_eq!(
        title.value.get(),
        "Old name",
        "the field is seeded from the entity — the name the writer sees in the tree"
    );

    title.value.set("The Long Road".to_string());
    tab.commit_names_fn()(); // what blur / Enter fires

    // Home 1: the entity field the tree and the tab caption read.
    let dto = binder_item_commands::get_binder_item(&ctx, &item.id)
        .unwrap()
        .unwrap();
    assert_eq!(dto.title, "The Long Road");

    // Home 2: the content row the manuscript compiles.
    let content_ids = binder_item_commands::get_binder_item_relationship(
        &ctx,
        &item.id,
        &BinderItemRelationshipField::Contents,
    )
    .unwrap();
    let chapter_title = content_commands::get_content_multi(&ctx, &content_ids)
        .unwrap()
        .into_iter()
        .flatten()
        .find(|c| c.role == ContentRole::ChapterTitle)
        .map(|c| c.data);
    assert_eq!(chapter_title.as_deref(), Some("The Long Road"));
}

/// BUG 1's fix. `note_details::note_details_pane` used to resolve its tag
/// palette via `ctx.app_state::<TagsViewModel>()`, which on the app's own
/// first-launch fallback (see `startup.rs`'s throwaway `WorkSession`, built on
/// a fresh, never-seeded `AppIds`) is bound to no `work_id` at all. So the
/// Details segment's "+" popover called `TagsViewModel::create`, which needs a
/// real `work_id` (see `WorkTagsListModel::create`'s own doc), and it silently
/// created nothing.
///
/// This builds the pane with **no `app_state` registered at all**, the same
/// "nothing to find" state that lookup was reaching in the wild, and proves
/// it does not matter any more: the tab's own [`ContentTab::tags`] is what the
/// pane is built from now, and that handle genuinely creates a tag against the
/// tab's real, open Work.
///
/// `#[cfg(not(feature = "mocks"))]`: the mocks palette's `create` ignores its
/// `owner_id` entirely (see `WorkTagsListModel::create`'s mock `impl`) and
/// always succeeds, so it cannot tell a Work-bound handle from an unbound one.
/// Only the real backend's `owner_id?` early-return actually exercises the
/// bug this test pins.
#[cfg(not(feature = "mocks"))]
#[test]
fn the_details_segment_creates_a_tag_against_the_tabs_own_work() {
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};

    let ctx = Rc::new(AppContext::new());
    let work = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default()).unwrap();
    let binder = binder_commands::create_binder(
        &ctx,
        None,
        &CreateBinderDto {
            name: "B".into(),
            activated: true,
            ..Default::default()
        },
        work.id,
        0,
    )
    .unwrap();
    let item = binder_item_commands::create_binder_item(
        &ctx,
        None,
        &CreateBinderItemDto {
            status: None,
            title: "A note".into(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Note,
            activated: true,
            is_exportable: true,
            ..Default::default()
        },
        binder.id,
        -1,
    )
    .unwrap();

    // The tab's own ids, bound to the real Work: the handle `note_details_pane`
    // must reach through `ContentTab::tags()`, not through whatever (if
    // anything) `app_state` happens to hold.
    let ids = AppIds::new();
    ids.work_id.set(Some(work.id));
    let tab = tab_for(
        &ctx,
        item.id,
        &BinderItemRole::Item,
        &BinderItemSubRole::Note,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &ids,
    );

    // The pane must build with no `app_state` registered at all. Before the fix,
    // an unregistered `TagsViewModel` made the whole Tags section render nothing
    // (the `if let Some(vm) = &tags_vm` gate this module's doc used to describe);
    // it must now render regardless, since the handle comes off the tab.
    let mut tree = crate::test_support::tree_with_events(&ctx);
    let root = tree.add_boxed(note_details::note_details_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 900.0));
    let _ = tree.render();
    assert!(
        first_of_type(&tree, root, "TagPillField").is_some(),
        "the Details segment built no Tags section at all with nothing in `app_state`"
    );

    // The exact handle the pane's Tags section was built from: it must be able
    // to create a tag, which it can do only if it actually knows the tab's Work.
    let id = tab.tags().create("Protagonist", "#607d8b", "", true);
    assert!(
        id.is_some(),
        "the Details segment's tags handle could not create a tag against its own Work"
    );
}

/// A segment registered for `BinderItemSubRole::Note` actually appears on a
/// `Folder/Note` tab's bar.
///
/// `folder_synopsis_with_overview` used to be hardcoded to exactly Notes and
/// Overview and never read `segments::registered_for` at all.
/// `register_container_segment`'s `shows_on` gate accepts `BinderItemSubRole::Note`
/// with no error, so a registration landing there believed it had succeeded and
/// the segment simply never showed up. This is the regression test: it registers a
/// segment, points a real `Folder/Note` tab's `SegmentedControl` at it, and insists
/// the widget that segment's `view` builds is actually the one that gets laid out,
/// not merely that the registry's own filtered list contains the id, which the
/// broken code also passed.
#[test]
fn a_segment_registered_for_a_note_folder_appears_on_its_bar() {
    use crate::tabs::shared::segments::{self, ContainerSegmentSpec};
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};
    use teksilo::widgets::FixedSize;

    let ctx = Rc::new(AppContext::new());
    let work = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default()).unwrap();
    let binder = binder_commands::create_binder(
        &ctx,
        None,
        &CreateBinderDto {
            name: "Notes".into(),
            activated: true,
            ..Default::default()
        },
        work.id,
        0,
    )
    .unwrap();
    let item = binder_item_commands::create_binder_item(
        &ctx,
        None,
        &CreateBinderItemDto {
            status: None,
            title: "Research".into(),
            role: BinderItemRole::Folder,
            sub_role: BinderItemSubRole::Note,
            activated: true,
            is_exportable: true,
            ..Default::default()
        },
        binder.id,
        -1,
    )
    .unwrap();

    // A `FixedSize` at a height nothing else on this tab lays out to, so its
    // presence in the tree is unambiguous evidence that the registered `view` ran
    // and was actually mounted, not merely that some segment button exists.
    const MARKER_HEIGHT: f32 = 444.0;
    const MARKER_ID: &str = "test.notes-marker";
    let _handle = segments::register_container_segment(
        "test.notes_folder_segment",
        ContainerSegmentSpec {
            id: MARKER_ID.to_string(),
            label: Rc::new(|| lit!("Marker".to_string())),
            view: Rc::new(|_tab| {
                Box::new(FixedSize::new().width(10.0).height(MARKER_HEIGHT)) as Box<dyn Widget>
            }),
            shows_on: Rc::new(|s| matches!(s, BinderItemSubRole::Note)),
        },
    )
    .expect("a free namespace and a free id");

    let tab = tab_for(
        &ctx,
        item.id,
        &BinderItemRole::Folder,
        &BinderItemSubRole::Note,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    // Selected before the tab is built: `Switcher` mounts only the child whose
    // index matches at build time, so this is what makes the registered segment
    // the one that is actually laid out rather than merely present in the list.
    tab.segment.set(Some(segments::segment_id(MARKER_ID)));

    let mut tree = WidgetTree::new();
    let root = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(800.0, 600.0));

    fn any_child_at_height(tree: &WidgetTree, id: WidgetId, height: f32) -> bool {
        if (tree.bounds(id).height - height).abs() < 0.5 {
            return true;
        }
        tree.children(id)
            .into_iter()
            .any(|c| any_child_at_height(tree, c, height))
    }
    assert!(
        any_child_at_height(&tree, root, MARKER_HEIGHT),
        "a segment registered for BinderItemSubRole::Note did not appear on a \
         Folder/Note tab's segment bar"
    );
}

/// Scene, ChapterScene and Note are no longer collapsed into one prose kind:
/// `tab_for` tags each, and `main_typography` resolves the right bundle
/// (Scene → Scene font, Note → Notes font).
#[test]
fn prose_kind_distinguishes_scene_from_note() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let mk = |sr: BinderItemSubRole| {
        tab_for(
            &ctx,
            1,
            &Item,
            &sr,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        )
    };
    assert_eq!(mk(Scene).kind(), Some(ProseKind::Scene));
    assert_eq!(mk(ChapterScene).kind(), Some(ProseKind::Scene));
    assert_eq!(mk(Note).kind(), Some(ProseKind::Note));
    assert_eq!(mk(Part).kind(), Option::None); // Item/Part → heading, no prose kind

    // `main_typography` picks the bundle by kind.
    assert_eq!(mk(Scene).main_typography().font_family.get(), "Literata");
    assert_eq!(mk(Note).main_typography().font_family.get(), "Inter");
}

/// Distraction-free mode is a *window-mode* axis, not a content-type one:
/// `main_typography` must return the distraction-free bundle for BOTH a
/// Scene and a Note tab the instant this tab's window enters the mode, and
/// must fall back to the normal Scene/Notes split the instant it leaves —
/// the branch `ContentTab::new`'s `distraction_free` flag exists for.
/// `main_column_width` rides the very same flag, so it is pinned here too:
/// this is the regression test for the column-width slider that used to be
/// a complete no-op (nothing downstream of `ContentTab::column_width` ever
/// read `distraction_free_width`).
#[test]
fn distraction_free_overrides_prose_kind_typography_while_active() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let mut typo = test_typography();
    typo.distraction_free.font_family = Signal::new("Distraction Serif".to_string());
    let distraction_free = Signal::new(false);
    let mk = |sr: BinderItemSubRole,
              typo: &EditorTypographySet,
              df: &Signal<bool>,
              df_width: &Signal<f32>| {
        let open_doc = Rc::new(OpenDoc::build(
            &ctx,
            1,
            &Item,
            &sr,
            &[],
            Signal::new(0),
            std::path::Path::new(""),
        ));
        ContentTab::new(
            ctx.clone(),
            AppIds::new(),
            OpenDocsStore::new(ctx.clone()),
            open_doc,
            Signal::new(700.0),
            Signal::new(true),
            Signal::new(SynopsisPlacement::default()),
            Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
            typo.clone(),
            crate::shared::TypewriterSettings::off(),
            crate::shared::CaretHighlightSettings::off(),
            crate::settings::EditorViewMemory::detached(false),
            crate::settings::CorkboardDefaults::detached(),
            crate::settings::TreeExpansionViewModel::new(
                ctx.clone(),
                AppIds::new(),
                crate::models::TreeExpansionService::in_memory_default(),
            ),
            df.clone(),
            df_width.clone(),
            crate::format::FormatViewModel::detached(),
            crate::writing_session::WritingGamesViewModel::detached(),
            crate::save::WorkHandle::detached(ctx.clone(), AppIds::new()),
            Signal::new(GoalUnit::default()),
            crate::tags::TagsViewModel::detached(ctx.clone(), AppIds::new()),
            crate::statuses::StatusesViewModel::new(ctx.clone(), AppIds::new()),
            crate::mentions::MentionIndex::new(ctx.clone(), AppIds::new()),
        )
    };
    let distraction_free_width = Signal::new(620.0);

    // Inactive: the normal Scene/Note split still applies, and the column
    // stays at the normal width.
    let scene = mk(Scene, &typo, &distraction_free, &distraction_free_width);
    let note = mk(Note, &typo, &distraction_free, &distraction_free_width);
    assert_eq!(scene.main_typography().font_family.get(), "Literata");
    assert_eq!(note.main_typography().font_family.get(), "Inter");
    assert_eq!(scene.main_column_width().get(), 700.0);
    assert_eq!(note.main_column_width().get(), 700.0);

    // Active: both read the distraction-free bundle — and the
    // distraction-free width — instead.
    distraction_free.set(true);
    assert_eq!(
        scene.main_typography().font_family.get(),
        "Distraction Serif"
    );
    assert_eq!(
        note.main_typography().font_family.get(),
        "Distraction Serif"
    );
    assert_eq!(scene.main_column_width().get(), 620.0);
    assert_eq!(note.main_column_width().get(), 620.0);

    // The distraction-free width is itself live, exactly like every other
    // Settings-backed signal — dragging the slider must reach an already
    // built tab, not just a freshly opened one.
    distraction_free_width.set(500.0);
    assert_eq!(scene.main_column_width().get(), 500.0);

    // Deactivated again: back to the normal split/width — the flag is
    // live, not baked in at construction time.
    distraction_free.set(false);
    assert_eq!(scene.main_typography().font_family.get(), "Literata");
    assert_eq!(scene.main_column_width().get(), 700.0);
}

/// **The lane over a stream maps every row that is on the page**, each onto its own
/// slice of one strip.
///
/// The hardest thing this feature does, and the one nothing else can stand in for.
/// A tab's lane maps one document; a Full Chapter's maps as many as the writer has
/// scrolled to, and it has to know where each of them landed — which nothing in the
/// row-list layer can say, because typing in row 1 moves row 2 and fires no event
/// any of those view-models watch.
///
/// Asserted through the **boundaries** provider, which needs no text: it puts one
/// rule at the top of each mapped document. That makes the assertion exactly "did
/// every row get its own place on the strip", with nothing about the prose in the
/// way.
#[cfg(feature = "mocks")]
#[test]
fn a_stream_lane_marks_every_row_on_its_own_slice() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;

    let _providers = crate::margin_lane::install_builtin_providers();
    let ctx = Rc::new(AppContext::new());
    let open_doc = Rc::new(OpenDoc::build(
        &ctx,
        101,
        &Folder,
        &ChapterScene,
        &[],
        Signal::new(0),
        std::path::Path::new(""),
    ));
    let tab = ContentTab::new(
        ctx.clone(),
        AppIds::new(),
        OpenDocsStore::new(ctx.clone()),
        open_doc,
        Signal::new(700.0),
        Signal::new(true),
        Signal::new(SynopsisPlacement::default()),
        Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
        test_typography(),
        crate::shared::TypewriterSettings::off(),
        crate::shared::CaretHighlightSettings::off(),
        crate::settings::EditorViewMemory::detached(false),
        crate::settings::CorkboardDefaults::detached(),
        crate::settings::TreeExpansionViewModel::new(
            ctx.clone(),
            AppIds::new(),
            crate::models::TreeExpansionService::in_memory_default(),
        ),
        Signal::new(false),
        Signal::new(crate::DISTRACTION_FREE_WIDTH_DEFAULT),
        crate::format::FormatViewModel::detached(),
        crate::writing_session::WritingGamesViewModel::detached(),
        crate::save::WorkHandle::detached(ctx.clone(), AppIds::new()),
        Signal::new(GoalUnit::default()),
        crate::tags::TagsViewModel::detached(ctx.clone(), AppIds::new()),
        crate::statuses::StatusesViewModel::new(ctx.clone(), AppIds::new()),
        crate::mentions::MentionIndex::new(ctx.clone(), AppIds::new()),
    );
    tab.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_MANUSCRIPT,
        )));

    let mut tree = crate::test_support::tree_with_settings(&ctx);
    let root = tree.add_boxed(tab_pane(&tab));
    // Two full frames, and both halves of each matter. `render` is what runs the
    // editors' own text layout, and until it has there is no geometry to convert an
    // offset against — a lane resolved before it correctly reports nothing. And the
    // second frame is needed because the rows are placed *after* their sibling lane
    // in the first: that gap is exactly why the host binds the extents' generation
    // at `Relayout` rather than below it.
    tree.layout(teksilo::prelude::SizeProposal::exact(1400.0, 900.0));
    let _ = tree.render();
    tree.layout(teksilo::prelude::SizeProposal::exact(1400.0, 900.0));
    let _ = tree.render();

    let lane_id = first_of_type(&tree, root, "MarginLane").expect("the stream carries a lane");
    let lane_bounds = tree.bounds(lane_id);
    let marks = tree
        .widget_as_any(lane_id)
        .and_then(|a| a.downcast_ref::<crate::widgets::MarginLane>())
        .expect("MarginLane opts into as_any")
        .resolve_marks(lane_bounds);

    let rules: Vec<_> = marks
        .iter()
        .filter(|m| m.shape == crate::widgets::LaneShape::Rule)
        .collect();
    assert!(
        rules.len() > 1,
        "a stream of several documents produced {} boundary marks",
        rules.len()
    );
    assert!(
        rules.windows(2).all(|w| w[0].top <= w[1].top + 0.01),
        "boundaries must come down the strip in document order: {rules:?}"
    );
    assert!(
        rules.last().unwrap().top - rules[0].top > lane_bounds.height * 0.2,
        "every row landed in one slice: {:?}",
        rules.iter().map(|m| m.top).collect::<Vec<_>>()
    );
}

/// The manuscript stream honours the tab's **main** typography and column,
/// not the Scene bundle and the normal column it used to hardcode.
///
/// This is what stops the distraction-free surface's whole point from ending
/// at a container's first segment: switch a Full Chapter into the surface and
/// the prose must be typeset like the mode's single-scene view. The row
/// *headers* deliberately stay on the tab's normal column — page furniture,
/// not manuscript.
#[cfg(feature = "mocks")]
#[test]
fn the_manuscript_stream_follows_the_tabs_main_typography_and_column() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let mut typo = test_typography();
    typo.distraction_free.font_family = Signal::new("Distraction Serif".to_string());

    // A container tab built the way the distraction-free surface builds one:
    // the flag pinned true for the life of the tab.
    let open_doc = Rc::new(OpenDoc::build(
        &ctx,
        101,
        &Folder,
        &ChapterScene,
        &[],
        Signal::new(0),
        std::path::Path::new(""),
    ));
    let tab = ContentTab::new(
        ctx.clone(),
        AppIds::new(),
        OpenDocsStore::new(ctx.clone()),
        open_doc,
        Signal::new(700.0),
        Signal::new(true),
        Signal::new(SynopsisPlacement::default()),
        Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
        typo,
        crate::shared::TypewriterSettings::off(),
        crate::shared::CaretHighlightSettings::off(),
        crate::settings::EditorViewMemory::detached(false),
        crate::settings::CorkboardDefaults::detached(),
        crate::settings::TreeExpansionViewModel::new(
            ctx.clone(),
            AppIds::new(),
            crate::models::TreeExpansionService::in_memory_default(),
        ),
        Signal::new(true),
        Signal::new(420.0),
        crate::format::FormatViewModel::detached(),
        crate::writing_session::WritingGamesViewModel::detached(),
        crate::save::WorkHandle::detached(ctx.clone(), AppIds::new()),
        Signal::new(GoalUnit::default()),
        crate::tags::TagsViewModel::detached(ctx.clone(), AppIds::new()),
        crate::statuses::StatusesViewModel::new(ctx.clone(), AppIds::new()),
        crate::mentions::MentionIndex::new(ctx.clone(), AppIds::new()),
    );

    // Segment 1 is the manuscript stream (own page / manuscript / Full
    // Synopsis / Corkboard / Overview).
    tab.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_MANUSCRIPT,
        )));
    let mut tree = crate::test_support::tree_with_events(&ctx);
    let id = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(1400.0, 900.0));

    // The stream's prose columns are capped at the distraction-free width.
    // The header columns keep the normal 700 — both must be present, which is
    // what proves the two widths did not collapse back into one.
    let caps = all_max_size_widths(&tree, id);
    assert!(
        caps.iter().any(|w| (*w - 420.0).abs() < 0.5),
        "no stream column at the distraction-free width — the stream is still \
             hardcoded to the tab's normal column. Widths seen: {caps:?}"
    );
    assert!(
        caps.iter().any(|w| (*w - 700.0).abs() < 0.5),
        "no stream column at the normal width — the row headers should stay on \
             the tab's own column. Widths seen: {caps:?}"
    );
}

/// Every laid-out `MaxSize` cap in the subtree — the writing columns.
fn all_max_size_widths(tree: &WidgetTree, root: WidgetId) -> Vec<f32> {
    let mut out = Vec::new();
    fn walk(tree: &WidgetTree, id: WidgetId, out: &mut Vec<f32>) {
        if tree
            .widget_type_name(id)
            .is_some_and(|n| n.ends_with("MaxSize"))
        {
            let b = tree.bounds(id);
            if b.width > 0.0 {
                out.push(b.width);
            }
        }
        for c in tree.children(id) {
            walk(tree, c, out);
        }
    }
    walk(tree, root, &mut out);
    out
}

// ── per-document view state (caret + page scroll) ─────────────────────

/// Build a Scene tab whose prose is `paragraphs` lines long, mount its pane,
/// and return both. A long document is what gives the page `ScrollArea` a
/// non-zero maximum, without which a restored scroll offset is clamped
/// straight back to 0 and the test would prove nothing.
fn mounted_scene(paragraphs: usize) -> (ContentTab, WidgetTree, WidgetId) {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        Signal::new(false),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    let text = "The rain kept on.\n".repeat(paragraphs);
    tab.main()
        .expect("a Scene has a main prose field")
        .doc
        .cursor_at(0)
        .insert_text(&text)
        .unwrap();
    // A real text backend, for the same reason
    // `a_window_narrower_than_the_column_does_not_overflow` needs one: the
    // no-backend fallback does not give the prose a faithful height, so the
    // page would have nothing to scroll and every offset would clamp to 0.
    let mut tree = WidgetTree::new().with_text_backend(std::rc::Rc::new(std::cell::RefCell::new(
        teksilo::canvas::MockTextBackend::new(),
    )));
    let root = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 300.0));
    (tab, tree, root)
}

/// A mounted prose pane must publish **both** ports. They come from two
/// different widgets — the caret from the editor handle, the scroll from the
/// page `ScrollArea` — because the editors run with their own scroll bars
/// suppressed, so `RichTextEditor::scroll_y()` on a prose column is
/// permanently 0. If either port went unattached, `capture_view_state` would
/// quietly return the seed forever and nothing would ever be persisted.
#[test]
fn a_mounted_prose_pane_publishes_both_view_state_ports() {
    let (tab, _tree, _root) = mounted_scene(4);
    let ports = tab.view_state_ports();
    assert!(ports.editor().is_some(), "the editor handle port is empty");
    assert!(
        ports.max_scroll().is_some(),
        "the page scroll port is empty — writing_page_scroll did not publish it"
    );
}

/// **A page that is built but never shown must not claim the tab's editor.**
///
/// A prose tab constructs *both* of its layouts in one pass: the Top flowing page,
/// and the Side splitter `WidthProbe` shows instead when the writer asked for it and
/// the window is wide enough. Only one is ever mounted. While each column attached
/// its handle to the tab as it was *constructed*, the Side layout, built second, won
/// the slot on every prose tab in the app whether or not anything ever showed it.
///
/// Nothing caught that, because both layouts edit the same document, so every
/// assertion about a caret held either way. It mattered all the same: the caret the
/// tab captured belonged to a widget nobody was looking at, and
/// `EditorHandle::focus` on an editor that never built is a silent no-op, so the
/// click that was supposed to put the caret in the prose did nothing at all.
///
/// Asserted at construction rather than through the mounted widget deliberately.
/// This is the exact moment the old code published, and it needs no layout, no text
/// backend and no synthesised input to be decisive.
#[test]
fn building_a_tabs_pages_publishes_no_editor_until_one_is_mounted() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        Signal::new(false),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );

    let _body = tab_pane(&tab);
    assert!(
        tab.view_state_ports().editor().is_none(),
        "building a tab's pages published an editor handle before any of them was \
         mounted, so the tab holds whichever page happened to be constructed last"
    );

    let (mounted, _tree, _root) = mounted_scene(4);
    assert!(
        mounted.view_state_ports().editor().is_some(),
        "and mounting one must publish it, or nothing is ever captured"
    );
}

/// A heading tab (Item/Part, Item/BookBegin) publishes an **editor** port off
/// its synopsis, the field that *is* the page there, since neither
/// combination has a prose column at all. Before `synopsis_column` grew its
/// `view_state` parameter nothing on this tab ever called `attach_editor`, so
/// `view_state_ports().editor()` was permanently `None` and a restored caret
/// had nothing to land on: the synopsis always opened at 0, whatever
/// `seed_view_state` said.
#[test]
fn a_heading_tabs_editor_port_is_its_synopsis() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    for sub_role in [Part, BookBegin] {
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &sub_role,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.synopsis()
            .expect("a heading tab has a synopsis")
            .doc
            .cursor_at(0)
            .insert_text("A heading's own synopsis, long enough to hold a caret.")
            .unwrap();
        // Seeded BEFORE the pane exists, exactly like the workspace-restore path.
        tab.seed_view_state(crate::shared::ViewState {
            caret: 12,
            scroll: 0.0,
        });

        let mut tree = WidgetTree::new();
        tree.add_boxed(tab_pane(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 600.0));

        let handle = tab
            .view_state_ports()
            .editor()
            .unwrap_or_else(|| panic!("Item/{sub_role:?} published no editor port at all"));
        assert_eq!(
            handle.cursor_position(),
            12,
            "Item/{sub_role:?}'s synopsis did not open at the seeded caret, \
                 nothing attached its handle to the tab's view-state ports"
        );
    }
}

/// A synopsis-only folder (Folder/None, and Folder/Note's own "Notes" page)
/// likewise publishes an editor port off its synopsis, the same reasoning as
/// the heading tabs above, for the two container combinations whose own page
/// has nothing else to write in.
#[test]
fn a_synopsis_only_folders_editor_port_is_its_synopsis() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    for sub_role in [None, Note] {
        let tab = tab_for(
            &ctx,
            1,
            &Folder,
            &sub_role,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        tab.synopsis()
            .expect("a synopsis-only folder has a synopsis")
            .doc
            .cursor_at(0)
            .insert_text("A folder's synopsis, long enough to hold a caret position.")
            .unwrap();
        tab.seed_view_state(crate::shared::ViewState {
            caret: 15,
            scroll: 0.0,
        });

        // Folder/Note opens on its "Notes" segment by default, the same own
        // page `folder_synopsis_body` builds for a plain Folder/None, so no
        // event source is needed for either: nothing on this page subscribes
        // to backend events (unlike the Overview segment beside it).
        let mut tree = WidgetTree::new();
        tree.add_boxed(tab_pane(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 600.0));

        let handle = tab
            .view_state_ports()
            .editor()
            .unwrap_or_else(|| panic!("Folder/{sub_role:?} published no editor port at all"));
        assert_eq!(
            handle.cursor_position(),
            15,
            "Folder/{sub_role:?}'s synopsis did not open at the seeded caret, \
                 nothing attached its handle to the tab's view-state ports"
        );
    }
}

/// A Scene tab's editor port is its **prose** editor, never the compact
/// synopsis box that sits above it.
///
/// Proved by identity of behaviour, not by hoping: the two documents are
/// deliberately different lengths, and the seeded caret is valid in the prose
/// (80 characters) but past the end of the synopsis (5). `writing_column`
/// clamps a seeded caret to *its own* document's length before opening the
/// editor there, so if the compact synopsis box ever held this port instead
/// (a regression that would compile cleanly, since both are `ProseField`s over
/// a `TextDocument`), the reported caret would clamp down to 5 rather than
/// land on 50.
#[test]
fn a_scene_tabs_editor_port_is_its_prose_not_its_synopsis_box() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        // Synopsis shown, so the compact box actually mounts rather than
        // sitting dormant behind `VisibleWhen`, the port it must NOT hold
        // has to exist for this to prove anything.
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.main()
        .unwrap()
        .doc
        .cursor_at(0)
        .insert_text(&"x".repeat(80))
        .unwrap();
    tab.synopsis()
        .unwrap()
        .doc
        .cursor_at(0)
        .insert_text("short")
        .unwrap();
    tab.seed_view_state(crate::shared::ViewState {
        caret: 50,
        scroll: 0.0,
    });

    let mut tree = crate::test_support::tree_with_events(&ctx);
    tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 600.0));

    let handle = tab
        .view_state_ports()
        .editor()
        .expect("a Scene publishes an editor port");
    assert_eq!(
        handle.cursor_position(),
        50,
        "the editor port reported a caret clamped to the synopsis box's 5 \
             characters instead of 50, it is wired to the compact synopsis, \
             not the prose editor"
    );
}

/// A **chapter folder**'s own page (Folder/ChapterScene, on its "Chapter"
/// segment) uses its own prose for the editor port too, for the same reason
/// as the Scene tab above: a chapter folder carries `SceneText` exactly like
/// the flat chapter it promotes to, and `folder_own_pane` gives the view-state
/// binding to its main editor whenever one exists, never to the synopsis
/// beside it.
#[test]
fn a_chapter_folders_editor_port_is_its_own_prose_not_its_synopsis() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Folder,
        &ChapterScene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.main()
        .expect("a chapter folder carries its own prose")
        .doc
        .cursor_at(0)
        .insert_text(&"x".repeat(80))
        .unwrap();
    tab.synopsis()
        .unwrap()
        .doc
        .cursor_at(0)
        .insert_text("short")
        .unwrap();
    tab.seed_view_state(crate::shared::ViewState {
        caret: 50,
        scroll: 0.0,
    });

    // The default "Chapter" (own) segment, no event source needed, exactly
    // as `segmented_containers_lay_out_their_bar` establishes for this body.
    let mut tree = WidgetTree::new();
    tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 600.0));

    let handle = tab
        .view_state_ports()
        .editor()
        .expect("a chapter folder's own page publishes an editor port");
    assert_eq!(
        handle.cursor_position(),
        50,
        "the editor port reported a caret clamped to the synopsis's 5 \
             characters instead of 50, the chapter folder's own page is \
             remembering its synopsis instead of its own prose"
    );
}

/// Item/Text and Item/BookEnd, the two contentless placeholders, publish no
/// editor port at all, so nothing can arm a focus request that no page could
/// ever honour: `placeholder` has no writing surface, and never calls
/// `writing_page_scroll` either.
#[test]
fn placeholder_tabs_publish_no_editor_port() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    for sub_role in [Text, BookEnd] {
        let tab = tab_for(
            &ctx,
            1,
            &Item,
            &sub_role,
            &[],
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::settings::EditorViewMemory::detached(false),
            &AppIds::new(),
        );
        let mut tree = WidgetTree::new();
        tree.add_boxed(tab_pane(&tab));
        tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 600.0));
        assert!(
            tab.view_state_ports().editor().is_none(),
            "Item/{sub_role:?} must publish no editor port, there is no page \
                 here for a focus request to land on"
        );
    }
}

/// Every node at/under `root` whose type name *ends with* `suffix`, like
/// [`first_of_type`], but returns every match instead of only the first.
/// Needed wherever more than one instance of a widget kind can be mounted at
/// once, which a container tab's `Switcher` guarantees the moment a second
/// segment has ever been selected (it keeps every page it has built).
fn all_of_type(tree: &WidgetTree, root: WidgetId, suffix: &str, out: &mut Vec<WidgetId>) {
    if tree
        .widget_type_name(root)
        .is_some_and(|n| n.ends_with(suffix))
    {
        out.push(root);
    }
    for c in tree.children(root) {
        all_of_type(tree, c, suffix, out);
    }
}

/// **The container-tab regression.** A `Folder/Book` tab builds its own page,
/// its manuscript stream and its Full Synopsis stream from one
/// `folder_segmented` call, and each of the three owns a `PageScrollPort`.
/// Before that port re-attached on activation, each one published to the
/// tab's view-state ports the moment it was *constructed* rather than the
/// moment it was *shown*, so a container tab reported, and restored, the
/// scroll of whichever segment `folder_segmented` happened to build last
/// (Full Synopsis), whatever the writer actually had on screen.
///
/// This mounts the Book on its own page, switches to Full Synopsis and back,
/// and at each stop writes a distinguishing offset through the tab-level port
/// (`apply_view_state`) and confirms it landed on *that segment's own*
/// `ScrollArea` and nowhere else, proving the port always names the page
/// actually on screen, not merely some page.
#[test]
fn a_container_tabs_page_scroll_port_follows_the_visible_segment() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    use teksilo::widgets::ScrollArea;

    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Folder,
        &Book,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OWN,
        )));
    // Enough synopsis text to overflow a small viewport on BOTH the container's
    // own page and the Full Synopsis stream (which shows this very same field
    // as its own content), without this every offset below clamps to 0 on
    // both pages, and the assertions would pass whichever page the port
    // actually named.
    tab.synopsis()
        .expect("a Book has a synopsis")
        .doc
        .cursor_at(0)
        .insert_text(&"A paragraph of synopsis text, long enough to wrap. ".repeat(20))
        .unwrap();

    // `tree_with_events`: the Full Synopsis stream's wiring child subscribes to
    // backend events. A real text backend: the no-backend fallback does not
    // give the synopsis a faithful height, and this test rests entirely on the
    // page actually needing to scroll.
    let mut tree = crate::test_support::tree_with_events(&ctx).with_text_backend(std::rc::Rc::new(
        std::cell::RefCell::new(teksilo::canvas::MockTextBackend::new()),
    ));
    let root = tree.add_boxed(tab_pane(&tab));
    let small = teksilo::prelude::SizeProposal::exact(600.0, 120.0);
    tree.layout(small);

    // The `Switcher` behind `RememberSegment` is lazy, so only the default
    // segment (the own page) has ever been mounted, exactly one `ScrollArea`
    // exists in the whole tree at this point.
    let mut areas = Vec::new();
    all_of_type(&tree, root, "ScrollArea", &mut areas);
    assert_eq!(
        areas.len(),
        1,
        "the own page is the only segment selected so far, so only its \
             ScrollArea should exist yet, got {areas:?}"
    );
    let own_area = areas[0];

    let offset = |tree: &WidgetTree, id: WidgetId| -> f32 {
        tree.widget_as_any(id)
            .and_then(|a| a.downcast_ref::<ScrollArea>())
            .expect("ScrollArea opts into as_any")
            .scroll_y_signal()
            .get()
    };
    let max = |tree: &WidgetTree, id: WidgetId| -> f32 {
        tree.widget_as_any(id)
            .and_then(|a| a.downcast_ref::<ScrollArea>())
            .expect("ScrollArea opts into as_any")
            .max_scroll_y_signal()
            .get()
    };
    assert!(
        max(&tree, own_area) > 0.0,
        "the own page must actually be scrollable, or the offsets below prove \
             nothing"
    );

    tab.apply_view_state(crate::shared::ViewState {
        caret: 0,
        scroll: 40.0,
    });
    assert_eq!(
        offset(&tree, own_area),
        40.0,
        "the port's write did not land on the own page's ScrollArea"
    );

    // Switch to Full Synopsis and give the tree a layout pass to mount it:
    // `segment.set` alone only fires `RememberSegment`'s persist effect, the
    // same reason `switching_a_container_view_persists_and_a_new_tab_inherits`
    // has to drive an actual page swap through a layout pass rather than the
    // signal write alone.
    tab.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_SYNOPSIS,
        )));
    tree.layout(small);

    areas.clear();
    all_of_type(&tree, root, "ScrollArea", &mut areas);
    assert_eq!(
        areas.len(),
        2,
        "Full Synopsis must have mounted its own ScrollArea alongside the own \
             page's (lazily, on first selection), got {areas:?}"
    );
    let synopsis_area = *areas
        .iter()
        .find(|&&id| id != own_area)
        .expect("a second, distinct ScrollArea for Full Synopsis");
    assert!(
        max(&tree, synopsis_area) > 0.0,
        "Full Synopsis must be scrollable too, for the same reason as above"
    );

    tab.apply_view_state(crate::shared::ViewState {
        caret: 0,
        scroll: 25.0,
    });
    assert_eq!(
        offset(&tree, synopsis_area),
        25.0,
        "the port's write must land on the NOW-visible Full Synopsis page"
    );
    assert_eq!(
        offset(&tree, own_area),
        40.0,
        "...and must not disturb the page that just left the screen"
    );

    // Back to the own page. No third `ScrollArea` appears, the `Switcher`
    // keeps every page it has ever mounted; only visibility changes.
    tab.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OWN,
        )));
    tree.layout(small);
    areas.clear();
    all_of_type(&tree, root, "ScrollArea", &mut areas);
    assert_eq!(areas.len(), 2, "switching back mounts no new page");

    tab.apply_view_state(crate::shared::ViewState {
        caret: 0,
        scroll: 10.0,
    });
    assert_eq!(
        offset(&tree, own_area),
        10.0,
        "the port must reattach to the own page on the way back, this is \
             exactly the case `PageScrollPort`'s activation effect exists for"
    );
    assert_eq!(
        offset(&tree, synopsis_area),
        25.0,
        "...and the page just switched away from keeps whatever it was left at"
    );
}

/// The round trip the whole feature rests on: a seeded caret reaches the
/// editor as it builds, and the caret the writer actually leaves behind is
/// what comes back out — not the seed.
#[test]
fn a_seeded_caret_reaches_the_editor_and_the_live_one_comes_back() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        Signal::new(false),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.main()
        .unwrap()
        .doc
        .cursor_at(0)
        .insert_text("The rain kept on for three days.")
        .unwrap();
    // Seeded BEFORE the pane exists — the workspace-restore and
    // distraction-free-entry path.
    tab.seed_view_state(crate::shared::ViewState {
        caret: 9,
        scroll: 0.0,
    });

    let mut tree = crate::test_support::tree_with_events(&ctx);
    tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 600.0));

    let handle = tab.view_state_ports().editor().unwrap();
    assert_eq!(
        handle.cursor_position(),
        9,
        "the editor did not open at the seeded caret"
    );

    // The writer moves. Capture must report that, not the seed.
    handle.select_range(20, 20);
    assert_eq!(tab.capture_view_state().caret, 20);
}

/// A caret past the end of the document is clamped rather than landing
/// somewhere arbitrary. Reachable whenever a document was edited in another
/// window (or another split pane) between capture and restore — `New Window`
/// on the same project makes that an ordinary thing to do, not a corner case.
#[test]
fn a_stale_caret_past_the_end_is_clamped_to_the_document() {
    let (tab, _tree, _root) = mounted_scene(2);
    let doc = &tab.main().unwrap().doc;
    // The document's own maximum cursor position, which is what the clamp is
    // measured against and is not the character count. A cursor may sit on the
    // separator between each pair of blocks, so the two numbers differ by
    // `block_count() - 1`, and they coincide only in a single-block document.
    //
    // Asserting `character_count()` here would be asserting the bug
    // `tabs::shared::editor` records having fixed: clamping against the smaller
    // of the two walks the caret back one character per paragraph, so restoring
    // the end of a ninety block chapter left the writer ninety characters short.
    let last = doc.character_count() + doc.block_count().saturating_sub(1);
    tab.apply_view_state(crate::shared::ViewState {
        caret: last + 5_000,
        scroll: 0.0,
    });
    assert_eq!(
        tab.view_state_ports().editor().unwrap().cursor_position(),
        last,
        "a caret past the end must clamp to the last position a cursor can hold, \
         not to the character count"
    );
}

/// A rebuild that has nothing to do with the writer — a Promote, a
/// settings-driven relayout — mints a fresh editor over the same document.
/// It must not throw the caret back to wherever the tab was first opened,
/// which is what a naive "seed on every build" would do.
#[test]
fn rebuilding_a_pane_carries_the_caret_over() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        1,
        &Item,
        &Scene,
        &[],
        Signal::new(700.0),
        Signal::new(false),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.main()
        .unwrap()
        .doc
        .cursor_at(0)
        .insert_text("The rain kept on for three days.")
        .unwrap();

    let mut tree = crate::test_support::tree_with_events(&ctx);
    tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 600.0));
    tab.view_state_ports()
        .editor()
        .unwrap()
        .select_range(17, 17);

    // Rebuild, exactly as the tab factory would.
    let mut tree2 = crate::test_support::tree_with_events(&ctx);
    tree2.add_boxed(tab_pane(&tab));
    tree2.layout(teksilo::prelude::SizeProposal::exact(900.0, 600.0));

    assert_eq!(
        tab.view_state_ports().editor().unwrap().cursor_position(),
        17,
        "the rebuild reset the caret instead of carrying it over"
    );
}

/// A real (non-mock) `AppContext` holding one Work → Binder → Scene, plus that
/// scene's freshly loaded prose field. Shared by the two flush-gating tests
/// below, which both need a genuine `Content` row to write to and read back.
#[cfg(not(feature = "mocks"))]
fn scene_prose_field() -> (Rc<AppContext>, ProseField) {
    use frontend::commands::binder_commands;
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};

    let ctx = Rc::new(AppContext::new());
    let work = frontend::commands::work_commands::create_orphan_work(
        &ctx,
        None,
        &CreateWorkDto::default(),
    )
    .unwrap();
    let binder = binder_commands::create_binder(
        &ctx,
        None,
        &CreateBinderDto {
            name: "B".into(),
            activated: true,
            ..Default::default()
        },
        work.id,
        0,
    )
    .unwrap();
    let item = frontend::commands::binder_item_commands::create_binder_item(
        &ctx,
        None,
        &CreateBinderItemDto {
            status: None,
            title: "Scene".into(),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
            activated: true,
            ..Default::default()
        },
        binder.id,
        -1,
    )
    .unwrap();

    // Its own backend: this helper builds one field with nothing to share with.
    // The documents built in it keep it alive.
    let field = prose_field(
        &ctx,
        &teksilo::text_document::DocumentBackend::new(),
        item.id,
        ContentRole::SceneText,
        None,
    );
    (ctx, field)
}

/// `is_stale` distinguishes "flushed and quiet" from "flushed, then edited
/// again" — the exact gap `OpenDoc::dirty`/`doc.is_modified()` leaves, since
/// both are booleans a flush clears unconditionally with no memory of which
/// edit they were cleared against.
#[cfg(not(feature = "mocks"))]
#[test]
fn is_stale_distinguishes_a_later_edit_from_flushed_and_quiet() {
    let (_ctx, field) = scene_prose_field();
    assert!(!field.is_stale(), "a freshly loaded field is never stale");

    // A first edit — `TextCursor::insert_text`, the same primitive live typing
    // uses, so this queues a real `ContentsChanged` and bumps `content_revision`
    // (unlike `set_djot_sync`/`set_plain_text`, which only reset the document
    // and never touch either `content_revision` or `is_modified`).
    field.doc.cursor_at(0).insert_text("First draft.").unwrap();
    assert!(field.is_stale(), "an edit must show stale");
    field.flush().expect("flush a real item's field");
    assert!(!field.is_stale(), "flush must clear staleness");
    assert!(
        !field.doc.is_modified(),
        "flush must also clear the coarse flag"
    );

    // A SECOND edit after that flush: `content_revision` moves again, so
    // `is_stale` still catches it — exactly the case a boolean
    // `dirty`/`is_modified` flag cannot distinguish from "flushed and quiet"
    // the instant after any flush clears it.
    field
        .doc
        .cursor_at(0)
        .insert_text("Second draft, written after the save. ")
        .unwrap();
    assert!(
        field.is_stale(),
        "an edit made after the last flush must be detected, even though \
             right after any flush it looks identical to 'flushed and quiet' from \
             `is_modified()`'s point of view"
    );
}

/// An editor undo made *after* a flush must still reach the store.
///
/// The regression this pins lost work silently. `flush` used to gate on
/// `doc.is_modified()`, a flag it clears itself, and `TextDocument::undo` never
/// set it — so: type, autosave (flag cleared), Ctrl+Z (buffer reverts, flag
/// stays false), and every later flush returned early. The screen showed the
/// undone text while the `.skrib` kept the pre-undo version, and the next time
/// the tab was rebuilt it re-read the stale row and the undo was gone for good.
///
/// Both halves of the fix are exercised here at once: `TextDocument::undo` now
/// marks the document modified, and `flush` gates on the exact revision
/// comparison rather than the flag.
#[cfg(not(feature = "mocks"))]
#[test]
fn an_undo_after_a_flush_still_reaches_the_store() {
    use frontend::commands::content_commands;

    let (ctx, field) = scene_prose_field();

    field.doc.cursor_at(0).insert_text("First draft.").unwrap();
    field.flush().expect("the first flush persists the typing");
    let content_id = field
        .content_id()
        .expect("flushing a field creates its Content row");
    let persisted = |id: u64| -> String {
        content_commands::get_content(&ctx, &id)
            .expect("the row reads back")
            .expect("the row exists")
            .data
    };
    assert!(
        persisted(content_id).contains("First draft."),
        "the typing reached the store"
    );

    // Stand exactly where autosave leaves the field: flushed, flag cleared.
    assert!(!field.doc.is_modified());
    assert!(!field.is_stale());

    field.doc.undo().expect("undo the typing");
    assert_eq!(
        field.doc.to_plain_text().unwrap().trim(),
        "",
        "the buffer really did revert"
    );
    assert!(
        field.is_stale(),
        "the store is now a revision behind the buffer"
    );

    field.flush().expect("the second flush persists the undo");
    assert!(
        !persisted(content_id).contains("First draft."),
        "the undone text must not survive in the store — this is the silent \
         data loss the flag-based gate caused"
    );
}

/// Autosaving prose must not put anything on the project's undo history.
///
/// It used to put a whole-Djot `Content::update` there every few seconds, so
/// the structural history was mostly prose snapshots interleaved with the
/// commands a writer would actually want back — which is what let a toast's
/// Undo pop a flush instead of the trash it named, and what made a
/// project-wide Ctrl+Z unsafe to offer. The buffer's own word-level history is
/// this text's undo; the mirror write is a timer going off.
#[cfg(not(feature = "mocks"))]
#[test]
fn flushing_prose_records_nothing_on_the_project_history() {
    use frontend::commands::undo_redo_commands;

    let (ctx, field) = scene_prose_field();
    let stack = undo_redo_commands::create_new_stack(&ctx);
    let depth = || undo_redo_commands::get_stack_size(&ctx, stack);
    // Measured as a *delta*, not against zero: the fixture builds a Work, a
    // Binder and an item with `stack_id: None`, and `None` is stack 0 — which
    // is precisely the trap being pinned here, so the baseline is not empty.
    let global_before = undo_redo_commands::get_stack_size(&ctx, 0);
    assert_eq!(depth(), 0);

    // Ten autosave cycles' worth of typing and flushing.
    for i in 0..10 {
        field
            .doc
            .cursor_at(0)
            .insert_text(&format!("Sentence {i}. "))
            .unwrap();
        field.flush().expect("flush");
    }

    assert_eq!(
        depth(),
        0,
        "ten flushes must leave the history exactly as they found it"
    );
    assert!(
        !undo_redo_commands::can_undo(&ctx, Some(stack)),
        "and nothing to undo"
    );
    assert_eq!(
        undo_redo_commands::get_stack_size(&ctx, 0),
        global_before,
        "nor may it leak into the global stack 0, which is undeletable and \
         which nothing ever clears — `stack_id: None` reroutes there rather \
         than opting out, which is the trap this fix exists to close"
    );
    // The prose still reached the store — untracked means unrecorded, not unwritten.
    let id = field.content_id().expect("the row exists");
    let stored = frontend::commands::content_commands::get_content(&ctx, &id)
        .unwrap()
        .unwrap()
        .data;
    assert!(stored.contains("Sentence 9."), "the text was persisted");
}

// ── The epigraph's attribution line ─────────────────────────────────────────
//
// Every writer keys the attribution off `Alignment::Right` inside a
// `SemanticRole::Epigraph` — DOCX and ODT give it the `EpigraphAttribution` named
// style, LaTeX and Typst their own slot. Nothing in the application could set that
// alignment, so the branch was dead in every format. These pin the two halves the
// application owns: that the alignment can be set at all, and that it survives into the
// stored prose, which is the only form any writer ever sees.

#[test]
fn marking_a_line_as_the_source_reaches_the_stored_prose() {
    use teksilo::text_document::{Alignment, TextDocument};
    use teksilo::widgets::rich_text::RichTextEditor;

    let doc = TextDocument::new();
    // Not "M. Ferrand": Djot reads a leading Roman numeral plus a full stop as an
    // ordered-list marker (M is 1000), so that fixture silently became a list and came
    // back renumbered as "I. Ferrand". An attribution line is a paragraph.
    doc.set_djot_sync("Salt is the only honest preservative.\n\nMarguerite Ferrand")
        .expect("the fixture parses");

    let editor = RichTextEditor::editor(doc.clone());
    // Put the caret in the second block, the way the writer would before pressing the
    // control, then mark it.
    let text = doc.to_plain_text().unwrap_or_default();
    editor.set_caret_position(text.len());
    editor.set_alignment(Alignment::Right);

    assert_eq!(
        editor.get_alignment(),
        Alignment::Right,
        "the caret's block must report the alignment that was just set"
    );

    // The stored form is what every exporter reads. `Content.data` is Djot, so the mark
    // has to survive `to_djot`; if it does not, the control changes the screen and
    // nothing else.
    let djot = doc.to_djot().expect("the document serialises");
    assert!(
        djot.contains("alignment=right"),
        "the attribution mark must reach the stored Djot, or no exporter can see it; got: {djot:?}"
    );
}

#[test]
fn marking_is_a_toggle_that_leaves_the_other_lines_alone() {
    use teksilo::text_document::{Alignment, TextDocument};
    use teksilo::widgets::rich_text::RichTextEditor;

    let doc = TextDocument::new();
    doc.set_djot_sync("The quotation.\n\nThe source")
        .expect("the fixture parses");
    let editor = RichTextEditor::editor(doc.clone());

    let text = doc.to_plain_text().unwrap_or_default();
    editor.set_caret_position(text.len());
    editor.set_alignment(Alignment::Right);
    editor.set_alignment(Alignment::Left);
    assert_eq!(
        editor.get_alignment(),
        Alignment::Left,
        "marking a line already marked must give it back, which is what the control's \
         toggle depends on"
    );

    // The quotation above it never carried the mark and must not have gained one.
    editor.set_caret_position(0);
    assert_eq!(
        editor.get_alignment(),
        Alignment::Left,
        "the mark is a block-level statement about one line, not the whole epigraph"
    );
}

/// **Ctrl+F on a stream reads the whole stream.**
///
/// A Full Chapter, Part or Book is one manuscript to the writer in front of it, and a
/// find that stopped at the first row's last line would be answering a question nobody
/// asked. The banner over the page therefore searches the container's own prose and
/// every row's as one run, and its count is the page's, not a document's.
///
/// Two things are pinned here that no unit test can reach. The banner must resolve to
/// the **page** on screen — a segmented tab has two, and the segment bar is what says
/// which — and the documents must arrive from the rows this page actually mounted, which
/// is a handshake between three files.
///
/// `#[cfg(not(feature = "mocks"))]`: the mock `StreamRowsModel` fabricates a stream for
/// every container and ignores the head id, so it cannot show that the real subtree walk
/// reached these rows.
#[cfg(not(feature = "mocks"))]
#[test]
fn a_streams_find_banner_searches_every_row_of_the_page() {
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};

    let ctx = Rc::new(AppContext::new());
    let work = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default()).unwrap();
    let binder = binder_commands::create_binder(
        &ctx,
        None,
        &CreateBinderDto {
            name: "Manuscript".into(),
            activated: true,
            ..Default::default()
        },
        work.id,
        0,
    )
    .unwrap()
    .id;
    let mut index = 0i32;
    let mut add = |title: &str, role: BinderItemRole, sub_role: BinderItemSubRole, indent: i64| {
        let id = binder_item_commands::create_binder_item(
            &ctx,
            None,
            &CreateBinderItemDto {
                status: None,
                title: title.into(),
                role,
                sub_role,
                activated: true,
                is_exportable: true,
                indent,
                ..Default::default()
            },
            binder,
            index,
        )
        .unwrap()
        .id;
        index += 1;
        id
    };
    let chapter = add(
        "The crossing",
        BinderItemRole::Folder,
        BinderItemSubRole::ChapterScene,
        0,
    );
    let first = add("Dusk", BinderItemRole::Item, BinderItemSubRole::Scene, 1);
    let second = add("Dawn", BinderItemRole::Item, BinderItemSubRole::Scene, 1);

    let ids = AppIds::new();
    ids.work_id.set(Some(work.id));
    let tab = tab_for(
        &ctx,
        chapter,
        &BinderItemRole::Folder,
        &BinderItemSubRole::ChapterScene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &ids,
    );

    // On the container's own page there is nothing multi-document to search, so Ctrl+F
    // means the tab's own banner — a chapter folder's prose — exactly as before.
    tab.segment.set(Some(shared::segments::segment_id(
        shared::segments::SEG_OWN,
    )));
    assert!(
        tab.active_find()
            .is_some_and(|f| f.editor_handle().is_none() && f.has_documents()),
        "the container's own page resolves to its own single-document banner"
    );

    tab.segment.set(Some(shared::segments::segment_id(
        shared::segments::SEG_MANUSCRIPT,
    )));
    let mut tree = crate::test_support::tree_with_events(&ctx);
    let root = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));

    // The prose the reader is looking at: the chapter's own — held by the tab itself,
    // which is what the page renders at the top — then its two scenes, held by the
    // shared store the rows open through.
    tab.main()
        .expect("a chapter folder carries its own prose")
        .doc
        .set_plain_text("the ferry was late")
        .unwrap();
    for (id, text) in [
        (first, "no boats at all"),
        (second, "the ferry, and then the ferry again"),
    ] {
        tab.docs()
            .open(id)
            .and_then(|d| d.main.as_ref().map(|m| m.doc.clone()))
            .expect("a prose-bearing row")
            .set_plain_text(text)
            .unwrap();
    }

    let find = tab
        .active_find()
        .cloned()
        .expect("the manuscript stream has a banner");
    find.ensure_session(
        teksilo::text_document::HighlightFormat::default(),
        teksilo::text_document::HighlightFormat::default(),
    );
    find.open();
    find.query_signal().set("ferry".into());
    find.refresh_query();

    assert!(find.visible_signal().get(), "there was something to search");
    assert_eq!(
        find.count_signal().get(),
        3,
        "one in the chapter's own prose and two in the last scene — the page, not a row"
    );
    assert_eq!(find.current_signal().get(), 1);
    assert!(
        !find.single_document_signal().get(),
        "and the replace half is withheld over a page"
    );

    // The lane is told which row holds the cursor, so exactly one mark on the strip
    // reads as current while every row still shows its own hits.
    let q = crate::margin_lane::active_query()
        .get()
        .expect("the lane was told");
    assert_eq!(
        q.source,
        crate::margin_lane::LaneQuerySource::Editor(chapter),
        "the first hit on the page is in the chapter's own prose"
    );
    assert_eq!(
        q.current_in(second),
        None,
        "the last scene shows hits, none current"
    );

    // And the strip is actually on the page rather than only in the view-model: before
    // this, a stream had no banner mounted at all, so Ctrl+F opened nothing anywhere.
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
    let banner =
        first_of_type(&tree, root, "FindBanner").expect("the stream page carries a find banner");
    assert!(
        tree.bounds(banner).height > 0.0,
        "an opened banner takes height above the manuscript"
    );

    find.close();
    crate::margin_lane::set_active_query(None);
}

/// **A chapter folder's own page can be searched.**
///
/// It carries its own prose — a `Folder/ChapterScene` is a chapter a writer can type
/// straight into — and Ctrl+F has always resolved to its banner. That banner was never
/// mounted, so the shortcut opened a view-model, ran the query, published the hits to the
/// margin lane, and put nothing on screen: the silent half of a feature. A Part or a Book
/// has no prose on this page and still gets no banner, which is the honest answer rather
/// than an empty strip.
#[test]
fn a_chapter_folders_own_page_carries_the_banner_ctrl_f_opens() {
    let ctx = Rc::new(AppContext::new());
    let own = shared::segments::segment_id(shared::segments::SEG_OWN);

    let tab = tab_for(
        &ctx,
        1,
        &BinderItemRole::Folder,
        &BinderItemSubRole::ChapterScene,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.segment.set(Some(own));
    let find = tab
        .active_find()
        .cloned()
        .expect("a chapter folder's own prose is searchable");

    let mut tree = crate::test_support::tree_with_settings(&ctx);
    let root = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
    assert!(
        first_of_type(&tree, root, "FindBanner").is_none_or(|b| tree.bounds(b).height == 0.0),
        "a closed banner takes no height, so the prose sits flush at the top"
    );

    find.open();
    tree.layout(teksilo::prelude::SizeProposal::exact(1000.0, 700.0));
    let banner = first_of_type(&tree, root, "FindBanner")
        .expect("the container's own page carries a find banner");
    assert!(
        tree.bounds(banner).height > 0.0,
        "and Ctrl+F puts it on screen"
    );
    find.close();

    // A Book's own page is a title and a synopsis — no prose, so nothing to find.
    let book = tab_for(
        &ctx,
        2,
        &BinderItemRole::Folder,
        &BinderItemSubRole::Book,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    book.segment.set(Some(own));
    assert!(
        book.active_find().is_none(),
        "no prose on a Book's own page, so Ctrl+F is a no-op there"
    );
}

/// **An open cell editor holds the keyboard, so Escape cancels and Enter commits.**
///
/// `TreeTableView` never moved focus into the editor a cell delegate swapped in
/// (`TableView` always did; the line was left behind when the tree table was split out
/// of it). Focus stayed on the table, so every keystroke went to the table's own key
/// handler: Escape cancelled nothing, Enter *activated the row* — opening the item in an
/// editor tab in the middle of a rename — and the character that type-to-edit was
/// supposed to seed the field with was swallowed too. It only ever looked like it worked
/// because clicking into the field focuses it by hand.
///
/// Fixed in teksilo's `TreeBodyPane` (`ctx.focus_into` on the editing cell). This pins
/// the app-side half: the `on_key` Escape handler in `columns::cell_editor` is the thing
/// that finally receives a key, and `cancel_edit` is what it does.
#[cfg(feature = "mocks")]
#[test]
fn escape_cancels_an_open_overview_cell_editor() {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    use teksilo::core::event::{Key, Modifiers};
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        101,
        &Folder,
        &Book,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OVERVIEW,
        )));

    let vm = tab.overview().unwrap().clone();
    let uid = common::uid::fixture_uid(201);
    vm.begin_edit(uid, crate::models::COL_TITLE);

    let mut tree = crate::test_support::tree_with_events(&ctx);
    let root = tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(1200.0, 700.0));

    // No `tree.focus(..)` anywhere: mounting the editor is what must move the keyboard
    // into it. Focusing it by hand here would test the mouse path and pass on the bug.
    let focused = tree.focused().expect("something holds focus");
    // Inside the *table*, specifically: the header above it carries a `SearchField`,
    // which is itself a `TextInputField`, so "focus is on a text field" would pass with
    // the keyboard sitting in the filter box.
    // `contains`, not `first_of_type`'s `ends_with`: the node's type name is
    // `TreeTableView<..::OverviewRow>`.
    fn find(tree: &WidgetTree, id: WidgetId, needle: &str) -> Option<WidgetId> {
        if tree
            .widget_type_name(id)
            .is_some_and(|t| t.contains(needle))
        {
            return Some(id);
        }
        tree.children(id)
            .into_iter()
            .find_map(|c| find(tree, c, needle))
    }
    let table = find(&tree, root, "TreeTableView").expect("the Overview's table");
    assert!(
        tree.is_descendant_of(focused, table),
        "the cell editor never took the keyboard; focus sits on {:?} outside the table",
        tree.widget_type_name(focused)
    );
    assert!(
        tree.widget_type_name(focused)
            .is_some_and(|t| t.contains("TextInput")),
        "focus landed in the table but not in the editor: {:?}",
        tree.widget_type_name(focused)
    );

    tree.press_key(Key::Escape, Modifiers::NONE);
    assert_eq!(
        vm.editing_cell().get(),
        Option::None,
        "Escape must cancel the edit and close the editor"
    );

    // **And the keyboard must land somewhere.** Closing the editor destroys the widget
    // that was holding focus, so this is where focus can silently go nowhere — after
    // which Tab and the arrow keys reach nothing and the table is unusable without a
    // mouse (WCAG 2.4.3). It belongs back on the table the writer was working in.
    tree.layout(teksilo::prelude::SizeProposal::exact(1200.0, 700.0));
    // Re-resolved, not reused: closing the editor rebuilds `OverviewTable`, which mints
    // a fresh node — the id from before the keystroke is stale by now.
    let table = find(&tree, root, "TreeTableView").expect("the table survived the cancel");
    let after = tree
        .focused()
        .expect("focus was lost when the editor closed");
    assert!(
        after == table || tree.is_descendant_of(after, table),
        "focus left the table when the editor closed; it is on {:?}",
        tree.widget_type_name(after)
    );
}

/// **Clicking away from an open cell editor closes it, keeping what was typed.**
///
/// This is the "focus loss commits" half of the editing contract, and it had never once
/// run. It was written as `TextInput::new(..).on_focus(..)`, which compiles, reads
/// correctly and fires *never*: the focusable node is the inner `TextInputField`, which
/// registers an `on_focus` of its own for its caret, and a handler that fires answers
/// `Handled` — so the bubble stops one node below the wrapper the app's closure hangs on.
/// Neither gain nor loss ever arrived.
///
/// The visible result was an editor that could not be dismissed by clicking anywhere,
/// and, because it went on holding the keyboard, one that swallowed everything after it.
#[cfg(feature = "mocks")]
#[test]
fn clicking_another_row_closes_the_open_cell_editor() {
    let (mut tree, root, vm, p) = overview_with_open_label_editor();

    let table = find_containing(&tree, root, "TreeTableView").expect("the table");
    let target = a_cell_on_a_later_row(&tree, root, table);
    tree.click(target);
    tree.layout(p);

    assert_eq!(
        vm.editing_cell().get(),
        Option::None,
        "clicking another row left the editor open — and holding the keyboard, so every \
         later click and keystroke went to it instead of the table"
    );
}

/// ...but an **unrelated rebuild does not**.
///
/// The pane rebuilds constantly — a selection change, the other pane's autosave firing
/// `Content(Updated)`, an undo, a rename elsewhere — and each one destroys and re-creates
/// the editor widget. If "the editor lost focus" were read as "the writer left", every
/// one of those would commit and close a half-typed rename: exactly the loss
/// `edit_buffer`'s own doc says the buffer lives on the view-model to prevent.
///
/// Driven through a selection change made on the **model**, not with a click: a click
/// legitimately moves focus, which is the case above.
#[cfg(feature = "mocks")]
#[test]
fn an_unrelated_rebuild_leaves_the_open_editor_alone() {
    let (mut tree, _root, vm, p) = overview_with_open_label_editor();
    let before = vm.editing_cell().get();
    assert!(before.is_some(), "the editor is open to begin with");

    vm.edit_buffer().text.set("half-typed".to_string());
    for i in 0..3 {
        vm.selection().select(common::uid::fixture_uid(201 + i));
        tree.layout(p);
    }

    assert_eq!(
        vm.editing_cell().get(),
        before,
        "an unrelated rebuild closed the editor"
    );
    assert_eq!(
        vm.edit_buffer().text.get(),
        "half-typed",
        "an unrelated rebuild discarded what was typed"
    );
}

/// A mounted Overview with the Label editor open on the first fixture row, plus the
/// proposal the tree was laid out under.
#[cfg(feature = "mocks")]
fn overview_with_open_label_editor() -> (
    WidgetTree,
    WidgetId,
    crate::overview::OverviewViewModel,
    teksilo::prelude::SizeProposal,
) {
    use BinderItemRole::*;
    use BinderItemSubRole::*;
    let ctx = Rc::new(AppContext::new());
    let tab = tab_for(
        &ctx,
        101,
        &Folder,
        &Book,
        &[],
        Signal::new(700.0),
        Signal::new(true),
        test_typography(),
        crate::settings::EditorViewMemory::detached(false),
        &AppIds::new(),
    );
    tab.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OVERVIEW,
        )));

    let vm = tab.overview().unwrap().clone();
    vm.begin_edit(common::uid::fixture_uid(201), crate::models::COL_LABEL);

    let mut tree = crate::test_support::tree_with_events(&ctx);
    let root = tree.add_boxed(tab_pane(&tab));
    let p = teksilo::prelude::SizeProposal::exact(1200.0, 700.0);
    tree.layout(p);
    (tree, root, vm, p)
}

/// First node at/under `root` whose type name *contains* `needle` — `first_of_type`'s
/// `ends_with` misses every generic (`TreeTableView<..::OverviewRow>`).
fn find_containing(tree: &WidgetTree, id: WidgetId, needle: &str) -> Option<WidgetId> {
    if tree
        .widget_type_name(id)
        .is_some_and(|t| t.contains(needle))
    {
        return Some(id);
    }
    tree.children(id)
        .into_iter()
        .find_map(|c| find_containing(tree, c, needle))
}

/// A wide cell on a row below the first one, well right of the tree column — somewhere a
/// click means "select that row" and nothing else.
#[cfg(feature = "mocks")]
fn a_cell_on_a_later_row(tree: &WidgetTree, root: WidgetId, table: WidgetId) -> WidgetId {
    fn collect(tree: &WidgetTree, id: WidgetId, out: &mut Vec<WidgetId>) {
        if tree
            .widget_type_name(id)
            .is_some_and(|t| t.contains("CellA11y"))
        {
            out.push(id);
        }
        for c in tree.children(id) {
            collect(tree, c, out);
        }
    }
    let mut cells = Vec::new();
    collect(tree, root, &mut cells);
    let top = cells
        .iter()
        .map(|c| tree.bounds(*c).y)
        .fold(f32::INFINITY, f32::min);
    let left = tree.bounds(table).x;
    cells
        .into_iter()
        .find(|c| {
            let b = tree.bounds(*c);
            b.y > top + 40.0 && b.x > left + 130.0 && b.width > 40.0
        })
        .expect("a cell on a later row")
}

/// **A press on the empty band below the rows closes the editor too.**
///
/// Not covered by the per-cell rule: there is no cell down there to carry it, so the
/// dismissal is mounted on the table root as well. The band is most of the pane on a
/// short container, and it is where a writer clicks to mean "never mind".
#[cfg(feature = "mocks")]
#[test]
fn clicking_the_empty_band_below_the_rows_closes_the_open_cell_editor() {
    let (mut tree, root, vm, p) = overview_with_open_label_editor();
    let table = find_containing(&tree, root, "TreeTableView").expect("the table");
    let b = tree.bounds(table);

    // Well below the last row, still inside the table.
    let empty = teksilo::canvas::Point::new(b.x + b.width * 0.5, b.y + b.height - 8.0);
    press_at(&mut tree, empty);
    tree.layout(p);

    assert_eq!(
        vm.editing_cell().get(),
        Option::None,
        "clicking the empty band below the rows left the editor open"
    );
}

/// ...but a press **inside the open editor** does not.
///
/// The counter-case that keeps the rule honest: moving the caret, selecting a word and
/// dragging over text are all presses on the field, and every one of them would close the
/// editor under the pointer if the dismissal did not exempt them. This is what
/// `press_claimed_by_interactive_child` buys at the table root.
#[cfg(feature = "mocks")]
#[test]
fn clicking_inside_the_open_editor_keeps_it_open() {
    let (mut tree, root, vm, p) = overview_with_open_label_editor();
    let before = vm.editing_cell().get();
    assert!(before.is_some(), "the editor is open to begin with");

    let field = find_containing(&tree, root, "TextInputField").expect("a text field");
    // The header's search box is a `TextInputField` too; take the one in the table.
    let table = find_containing(&tree, root, "TreeTableView").expect("the table");
    let field = if tree.is_descendant_of(field, table) {
        field
    } else {
        fn collect(tree: &WidgetTree, id: WidgetId, out: &mut Vec<WidgetId>) {
            if tree
                .widget_type_name(id)
                .is_some_and(|t| t.contains("TextInputField"))
            {
                out.push(id);
            }
            for c in tree.children(id) {
                collect(tree, c, out);
            }
        }
        let mut all = Vec::new();
        collect(&tree, root, &mut all);
        all.into_iter()
            .find(|f| tree.is_descendant_of(*f, table))
            .expect("the cell editor's field")
    };

    let at = tree.bounds(field).center();
    press_at(&mut tree, at);
    tree.layout(p);

    assert_eq!(
        vm.editing_cell().get(),
        before,
        "clicking into the open editor closed it — the caret can never be moved"
    );
}

/// A primary press + release at an absolute point.
fn press_at(tree: &mut WidgetTree, at: teksilo::canvas::Point) {
    use teksilo::core::event::{Modifiers, PointerButton, WidgetEvent};
    tree.dispatch_event(WidgetEvent::PointerDown {
        position: at,
        button: PointerButton::Primary,
        modifiers: Modifiers::NONE,
    });
    tree.dispatch_event(WidgetEvent::PointerUp {
        position: at,
        button: PointerButton::Primary,
        modifiers: Modifiers::NONE,
    });
}

/// **Escape still cancels after the writer has clicked into the field.**
///
/// The path a person actually takes — open the cell, click to place the caret, change
/// their mind — and the one reported as broken. Headlessly it always worked; live it did
/// not, because a tooltip was up and *consumed* the keystroke before the editor ever saw
/// it (fixed in teksilo: `WidgetTree::tooltip_escape_pressed` retires the tip without
/// swallowing the key). This pins the app-side half of that path, which the tooltip bug
/// was hiding: focus stays in the field across the click, so the editor's own `on_key`
/// is still the thing Escape reaches.
#[cfg(feature = "mocks")]
#[test]
fn escape_cancels_after_clicking_into_the_open_editor() {
    use teksilo::core::event::{Key, Modifiers};
    let (mut tree, root, vm, p) = overview_with_open_label_editor();
    let table = find_containing(&tree, root, "TreeTableView").expect("the table");

    fn collect(tree: &WidgetTree, id: WidgetId, needle: &str, out: &mut Vec<WidgetId>) {
        if tree
            .widget_type_name(id)
            .is_some_and(|t| t.contains(needle))
        {
            out.push(id);
        }
        for c in tree.children(id) {
            collect(tree, c, needle, out);
        }
    }
    let mut fields = Vec::new();
    collect(&tree, root, "TextInputField", &mut fields);
    let field = fields
        .into_iter()
        .find(|f| tree.is_descendant_of(*f, table))
        .expect("the cell editor's field");

    let at = tree.bounds(field).center();
    press_at(&mut tree, at);
    tree.layout(p);

    tree.press_key(Key::Escape, Modifiers::NONE);
    tree.layout(p);
    assert_eq!(vm.editing_cell().get(), Option::None, "Escape did nothing");
}
