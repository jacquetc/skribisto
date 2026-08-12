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

    // The fixture tags scenes 201 and 202 and leaves the rest untagged, so a correctly
    // wired column mounts *some* dot rows but not one per row.
    let mut dot_rows = 0;
    count_containing(&tree, id, "TagDotsRow", &mut dot_rows);
    assert!(
        dot_rows > 0,
        "no TagDotsRow in the Overview - the tags column is not wired"
    );
    let rows = {
        use teksilo::data::TreeDataSource;
        tab.overview()
            .expect("a Book has an Overview")
            .rows()
            .visible_count()
    };
    assert!(
        dot_rows < rows,
        "every one of the {rows} rows mounted a dot row ({dot_rows}); an untagged row \
             must render an empty cell, or the column stops distinguishing tagged from not"
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
        mem.initial(&ChapterScene),
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
        mem.initial(&ChapterScene),
        Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OWN
        ))
    );
    a.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_MANUSCRIPT,
        ))); // A switches → Full Chapter
    assert_eq!(
        mem.initial(&ChapterScene),
        Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_MANUSCRIPT
        ))
    );
    b.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_SYNOPSIS,
        ))); // B → Full Synopsis: last switch wins
    assert_eq!(
        mem.initial(&ChapterScene),
        Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_SYNOPSIS
        ))
    );
    a.segment
        .set(Some(crate::tabs::shared::segments::segment_id(
            crate::tabs::shared::segments::SEG_OWN,
        ))); // A switches back → its switch wins in turn
    assert_eq!(
        mem.initial(&ChapterScene),
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

/// First node at/under `root` whose fully-qualified type name ends with `suffix`
/// (DFS pre-order); type names come from `std::any::type_name`, so match the leaf.
fn first_of_type(tree: &WidgetTree, root: WidgetId, suffix: &str) -> Option<WidgetId> {
    if tree
        .widget_type_name(root)
        .is_some_and(|n| n.ends_with(suffix))
    {
        return Some(root);
    }
    tree.children(root)
        .into_iter()
        .find_map(|c| first_of_type(tree, c, suffix))
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
fn mounted_scene(paragraphs: usize) -> (ContentTab, WidgetTree) {
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
    tree.add_boxed(tab_pane(&tab));
    tree.layout(teksilo::prelude::SizeProposal::exact(900.0, 300.0));
    (tab, tree)
}

/// A mounted prose pane must publish **both** ports. They come from two
/// different widgets — the caret from the editor handle, the scroll from the
/// page `ScrollArea` — because the editors run with their own scroll bars
/// suppressed, so `RichTextEditor::scroll_y()` on a prose column is
/// permanently 0. If either port went unattached, `capture_view_state` would
/// quietly return the seed forever and nothing would ever be persisted.
#[test]
fn a_mounted_prose_pane_publishes_both_view_state_ports() {
    let (tab, _tree) = mounted_scene(4);
    let ports = tab.view_state_ports();
    assert!(ports.editor().is_some(), "the editor handle port is empty");
    assert!(
        ports.max_scroll().is_some(),
        "the page scroll port is empty — writing_page_scroll did not publish it"
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
    let (tab, _tree) = mounted_scene(2);
    let len = tab.main().unwrap().doc.character_count();
    tab.apply_view_state(crate::shared::ViewState {
        caret: len + 5_000,
        scroll: 0.0,
    });
    assert_eq!(
        tab.view_state_ports().editor().unwrap().cursor_position(),
        len
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

/// `is_stale` distinguishes "flushed and quiet" from "flushed, then edited
/// again" — the exact gap `OpenDoc::dirty`/`doc.is_modified()` leaves, since
/// both are booleans a flush clears unconditionally with no memory of which
/// edit they were cleared against.
#[cfg(not(feature = "mocks"))]
#[test]
fn is_stale_distinguishes_a_later_edit_from_flushed_and_quiet() {
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

    let field = prose_field(&ctx, item.id, ContentRole::SceneText, None);
    assert!(!field.is_stale(), "a freshly loaded field is never stale");

    // A first edit — `TextCursor::insert_text`, the same primitive live typing
    // uses, so this queues a real `ContentsChanged` and bumps `content_revision`
    // (unlike `set_djot_sync`/`set_plain_text`, which only reset the document
    // and never touch either `content_revision` or `is_modified`).
    field.doc.cursor_at(0).insert_text("First draft.").unwrap();
    assert!(field.is_stale(), "an edit must show stale");
    field.flush(None).expect("flush a real item's field");
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
