// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use crate::settings::EditorTypography;
use crate::tabs;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

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

fn editors() -> EditorsViewModel {
    let app_ctx = Rc::new(AppContext::new());
    let save_state = SaveStateViewModel::new(app_ctx.clone(), AppIds::new());
    editors_with(app_ctx, save_state)
}

/// Build an `EditorsViewModel` over a caller-supplied `app_ctx` + save state,
/// so a test can construct **two** instances sharing the same
/// `SaveStateViewModel` — modelling two windows onto one project.
fn editors_with(app_ctx: Rc<AppContext>, save_state: SaveStateViewModel) -> EditorsViewModel {
    let ids = AppIds::new();
    let docs = OpenDocsStore::new(app_ctx.clone());
    let tree_expansion = crate::settings::TreeExpansionViewModel::new(
        app_ctx.clone(),
        ids.clone(),
        crate::models::TreeExpansionService::in_memory_default(),
    );
    EditorsViewModel::new(
        app_ctx,
        Signal::new(700.0),
        Signal::new(true),
        Signal::new(crate::shared::SynopsisPlacement::default()),
        Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
        test_typography(),
        crate::shared::TypewriterSettings::off(),
        crate::shared::CaretHighlightSettings::off(),
        crate::settings::EditorViewMemory::detached(false),
        crate::settings::CorkboardDefaults::detached(),
        ids,
        docs,
        Signal::new(false),
        save_state,
        Signal::new(false),
        tree_expansion,
        Signal::new(false),
        Signal::new(620.0),
        crate::go::GoAvailability::new(),
        crate::format::FormatViewModel::detached(),
        crate::writing_session::WritingGamesViewModel::detached(),
        Signal::new(GoalUnit::default()),
    )
}

/// Push a tab directly into `side` (bypassing the backend / store) so tab
/// management can be tested without a loaded project.
fn push_tab(vm: &EditorsViewModel, side: Side, item_id: u64) -> TabId {
    let id = TabId::fresh();
    let tab = tabs::tab_for(
        &vm.app_ctx,
        item_id,
        &BinderItemRole::Item,
        &BinderItemSubRole::Scene,
        &[],
        vm.column_width.clone(),
        vm.show_synopsis.clone(),
        vm.typography.clone(),
        vm.view_memory.clone(),
        &vm.ids,
    );
    vm.pane(side).tabs.push(TabHandle::dynamic(
        id,
        "editor",
        TabInfo::new().closable(true),
        tab,
    ));
    id
}

/// Seed the store with a Scene document for `item_id`, bypassing the backend.
fn seed_doc(vm: &EditorsViewModel, item_id: u64) {
    use crate::models::OpenDoc;
    vm.docs.insert_for_test(Rc::new(OpenDoc::build(
        &vm.app_ctx,
        item_id,
        &BinderItemRole::Item,
        &BinderItemSubRole::Scene,
        &[],
        Signal::new(0),
        std::path::Path::new(""),
    )));
}

/// The pane tab and the surface tab differ on exactly one axis, and it is the
/// one that decides which typography bundle and which column width the
/// writer actually sees.
///
/// This is the shape that replaces the defect: the flag is fixed for the
/// lifetime of the tab that reads it, rather than a live signal a mounted
/// pane was supposed to re-read and never did.
#[test]
fn a_surface_tab_is_distraction_free_and_a_pane_tab_never_is() {
    let app_ctx = Rc::new(AppContext::new());
    let save_state = SaveStateViewModel::new(app_ctx.clone(), AppIds::new());
    let vm = editors_with(app_ctx, save_state);
    // The shared fixture gives every bundle the same face; give the
    // distraction-free one its own so the two are distinguishable.
    vm.typography
        .distraction_free
        .font_family
        .set("Distraction Serif".to_string());
    seed_doc(&vm, 1);

    let surface = vm
        .open_surface_tab(
            1,
            vm.show_synopsis(),
            crate::shared::CaretHighlightSettings::off(),
        )
        .expect("the store holds item 1");
    assert_eq!(
        surface.main_typography().font_family.get(),
        "Distraction Serif"
    );
    assert_eq!(surface.main_column_width().get(), 620.0);

    let pane = vm.make_tab(
        vm.docs.peek(1).unwrap(),
        vm.distraction_free.clone(),
        vm.show_synopsis.clone(),
        vm.synopsis_placement.clone(),
        vm.caret_highlight.clone(),
    );
    assert_eq!(pane.main_typography().font_family.get(), "Literata");
    assert_eq!(pane.main_column_width().get(), 700.0);
}

/// The surface's refcount is its own to give back.
///
/// `release_own_open_docs` — the only release that runs on a real window
/// close — walks the two panes' tab lists, so a document the surface holds
/// open is invisible to it. If the surface ever forgets to release, that
/// item is pinned in the store forever and never gets its flush-on-evict.
#[test]
fn a_surface_tab_takes_a_refcount_that_release_gives_back() {
    let app_ctx = Rc::new(AppContext::new());
    let save_state = SaveStateViewModel::new(app_ctx.clone(), AppIds::new());
    let vm = editors_with(app_ctx, save_state);
    seed_doc(&vm, 1);
    let before = vm.docs.refs_for_test(1).expect("seeded");

    let tab = vm
        .open_surface_tab(
            1,
            vm.show_synopsis(),
            crate::shared::CaretHighlightSettings::off(),
        )
        .expect("the store holds item 1");
    assert_eq!(
        vm.docs.refs_for_test(1),
        Some(before + 1),
        "opening a surface tab must take a reference"
    );

    drop(tab);
    vm.release_surface_tab(1, None);
    assert_eq!(
        vm.docs.refs_for_test(1),
        Some(before),
        "releasing must give exactly one reference back, not more or fewer"
    );
}

/// The caption rule itself, at the seam every tab caption goes through.
#[test]
fn a_row_with_no_title_of_its_own_is_captioned_by_its_generated_name() {
    let chapter_3 = || Some("Chapter 3".to_string());
    // A title the writer typed wins over anything derived.
    assert_eq!(
        EditorsViewModel::caption_of("The Storm", chapter_3()).resolve_now(),
        "The Storm"
    );
    // **The bug this fixes.** An untitled chapter is named by its ordinal here, as it
    // already was in the binder, the stream, the corkboard and the exported book.
    assert_eq!(
        EditorsViewModel::caption_of("", chapter_3()).resolve_now(),
        "Chapter 3"
    );
    // Whitespace is not a title — `fallback_label_for` trims too, so treating it as
    // one here would caption the tab " " while every other surface says "Chapter 3".
    assert_eq!(
        EditorsViewModel::caption_of("   ", chapter_3()).resolve_now(),
        "Chapter 3"
    );
    // A scene or a note has no name to generate. The tree leaves that row blank; a
    // tab cannot, or there would be nothing left to click.
    assert_eq!(
        EditorsViewModel::caption_of("", None).resolve_now(),
        tr!(untitled()).resolve_now()
    );
}

/// Tab captions over a **real** manuscript: what an untitled chapter's tab is called,
/// and what renames it. Gated off `mocks`, whose every fixture row is titled (and whose
/// item probe answers for ids no test seeded).
#[cfg(not(feature = "mocks"))]
mod captions {
    use super::*;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};

    /// A numbering manuscript with one binder, and `vm` pointed at it.
    fn seed_work(vm: &EditorsViewModel) -> u64 {
        let work = work_commands::create_orphan_work(
            &vm.app_ctx,
            None,
            &CreateWorkDto {
                // The two fields naming derives from. Numbering *off* is a real state
                // too — covered by `an_unnumbered_untitled_chapter_says_what_it_is`.
                number_chapters: true,
                dict_language: vec!["en".into()],
                ..Default::default()
            },
        )
        .unwrap();
        vm.ids.work_id.set(Some(work.id));
        binder_commands::create_binder(
            &vm.app_ctx,
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
        .id
    }

    /// Append (or, with an explicit `index`, insert) an item into `binder`.
    fn seed_item(
        vm: &EditorsViewModel,
        binder: u64,
        title: &str,
        sub_role: BinderItemSubRole,
        index: i32,
    ) -> u64 {
        binder_item_commands::create_binder_item(
            &vm.app_ctx,
            None,
            &CreateBinderItemDto {
                title: title.into(),
                role: BinderItemRole::Item,
                sub_role,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            binder,
            index,
        )
        .unwrap()
        .id
    }

    /// The caption an open tab is showing, resolved.
    ///
    /// `TabInfo` keeps its title field crate-private and exposes no getter, but its
    /// `Debug` renders the *resolved* string — so this reads it back from there rather
    /// than asserting on the helper that produced it, which would pass even if
    /// `open_in` stopped calling that helper at all. Swap this for
    /// `TabInfo::title_text()` if teksilo ever grows one.
    fn caption_of_tab(vm: &EditorsViewModel, side: Side, item_id: u64) -> Option<String> {
        let pane = vm.pane(side);
        let rendered = (0..pane.tabs.len()).find_map(|i| {
            pane.tabs
                .with_item(i, |h| {
                    h.payload
                        .downcast_ref::<ContentTab>()
                        .filter(|t| t.item_id() == item_id)
                        .map(|_| format!("{:?}", h.info))
                })
                .flatten()
        })?;
        let open = rendered.find("resolved: \"")? + "resolved: \"".len();
        let rest = &rendered[open..];
        Some(rest[..rest.find('"')?].to_string())
    }

    /// **The bug.** Opening a chapter the writer never named used to caption its tab
    /// "Untitled" — so a manuscript of unnamed chapters (which is what a new project
    /// is, since numbering stopped writing "Chapter N" into titles) gave a strip of
    /// identical tabs with nothing to tell them apart.
    #[test]
    fn an_untitled_chapter_opens_in_a_tab_named_by_its_number() {
        let vm = editors();
        let binder = seed_work(&vm);
        let first = seed_item(&vm, binder, "", BinderItemSubRole::ChapterScene, -1);
        let second = seed_item(&vm, binder, "", BinderItemSubRole::ChapterScene, -1);

        vm.open_in(Side::Primary, first, "");
        vm.open_in(Side::Primary, second, "");
        assert_eq!(
            caption_of_tab(&vm, Side::Primary, first).as_deref(),
            Some("Chapter 1")
        );
        assert_eq!(
            caption_of_tab(&vm, Side::Primary, second).as_deref(),
            Some("Chapter 2")
        );
    }

    /// A title the writer typed is still the tab's name — the fallback must not
    /// overwrite one.
    #[test]
    fn a_named_chapter_keeps_the_name_its_writer_gave_it() {
        let vm = editors();
        let binder = seed_work(&vm);
        let id = seed_item(
            &vm,
            binder,
            "The Storm",
            BinderItemSubRole::ChapterScene,
            -1,
        );
        vm.open_in(Side::Primary, id, "The Storm");
        assert_eq!(
            caption_of_tab(&vm, Side::Primary, id).as_deref(),
            Some("The Storm")
        );
    }

    /// A scene has no generated name, so an unnamed one keeps the "Untitled" fallback.
    #[test]
    fn an_untitled_scene_still_falls_back_to_untitled() {
        let vm = editors();
        let binder = seed_work(&vm);
        let id = seed_item(&vm, binder, "", BinderItemSubRole::Scene, -1);
        vm.open_in(Side::Primary, id, "");
        assert_eq!(
            caption_of_tab(&vm, Side::Primary, id),
            Some(tr!(untitled()).resolve_now())
        );
    }

    /// With numbering off the row has no ordinal, so its tab says what it *is* —
    /// the same bare structural word the binder shows, not "Untitled".
    #[test]
    fn an_unnumbered_untitled_chapter_says_what_it_is() {
        let vm = editors();
        let binder = seed_work(&vm);
        let work_id = vm.ids.work_id.get().unwrap();
        let mut work = work_commands::get_work(&vm.app_ctx, &work_id)
            .unwrap()
            .unwrap();
        work.number_chapters = false;
        work_commands::update_work(&vm.app_ctx, None, &work.into()).unwrap();

        let id = seed_item(&vm, binder, "", BinderItemSubRole::ChapterScene, -1);
        vm.open_in(Side::Primary, id, "");
        assert_eq!(
            caption_of_tab(&vm, Side::Primary, id).as_deref(),
            Some("Chapter")
        );
    }

    /// **Renumbering renames.** Inserting a chapter above an open untitled one changes
    /// its name without touching the item at all — no rename, no `Updated` for it — so
    /// a caption pushed only on rename would go on saying "Chapter 1" while the binder,
    /// the stream and the book all said 2.
    #[test]
    fn inserting_a_chapter_above_renames_the_open_tab() {
        let vm = editors();
        let binder = seed_work(&vm);
        let open = seed_item(&vm, binder, "", BinderItemSubRole::ChapterScene, -1);
        vm.open_in(Side::Primary, open, "");
        assert_eq!(
            caption_of_tab(&vm, Side::Primary, open).as_deref(),
            Some("Chapter 1")
        );

        seed_item(&vm, binder, "", BinderItemSubRole::ChapterScene, 0);
        vm.refresh_captions();
        assert_eq!(
            caption_of_tab(&vm, Side::Primary, open).as_deref(),
            Some("Chapter 2"),
            "the open tab kept a number the manuscript no longer gives it"
        );
    }

    /// Clearing a chapter's title hands it back to the generated name; the sweep must
    /// therefore *replace* a caption, not merely fill an empty one.
    #[test]
    fn clearing_a_title_hands_the_tab_back_to_its_number() {
        let vm = editors();
        let binder = seed_work(&vm);
        let id = seed_item(
            &vm,
            binder,
            "The Storm",
            BinderItemSubRole::ChapterScene,
            -1,
        );
        vm.open_in(Side::Primary, id, "The Storm");

        let mut it = binder_item_commands::get_binder_item(&vm.app_ctx, &id)
            .unwrap()
            .unwrap();
        it.title = String::new();
        binder_item_commands::update_binder_item(&vm.app_ctx, None, &it.into()).unwrap();
        vm.items_updated(&[id]);

        assert_eq!(
            caption_of_tab(&vm, Side::Primary, id).as_deref(),
            Some("Chapter 1")
        );
    }

    /// The seam's richer view of the focused item, over a **real** manuscript:
    /// `binder_ops::item_dto` reads the store, so the uid and the
    /// `(role, sub_role)` only exist where real rows do. Gated off `mocks` for
    /// the same reason `captions` is.
    #[cfg(not(feature = "mocks"))]
    mod active_context {
        use super::captions::{seed_item, seed_work};
        use super::*;
        use frontend::commands::binder_item_commands;
        use frontend::direct_access::UpdateBinderItemDto;

        /// The dock-facing view must name the focused pane's item, carry its
        /// **durable** uid (an `EntityId` is re-minted by every `load_work`), and
        /// follow focus across a split.
        #[test]
        fn it_reflects_the_focused_panes_item() {
            let vm = editors();
            let binder = seed_work(&vm);
            let first = seed_item(&vm, binder, "One", BinderItemSubRole::Scene, -1);
            let second = seed_item(&vm, binder, "Two", BinderItemSubRole::Note, -1);

            assert_eq!(
                vm.active_context().get(),
                None,
                "nothing open, nothing focused"
            );

            vm.open_in(Side::Primary, first, "One");
            let a = vm.active_context().get().expect("an item is focused");
            assert_eq!(a.id, first);
            assert_eq!(a.sub_role, BinderItemSubRole::Scene);
            assert!(
                !a.uid.is_nil(),
                "a durable uid is the only key an extension may persist"
            );

            vm.set_split(true);
            vm.open_in(Side::Secondary, second, "Two");
            vm.set_focused(Side::Secondary);
            let b = vm.active_context().get().expect("the side pane is focused");
            assert_eq!(b.id, second);
            assert_eq!(b.sub_role, BinderItemSubRole::Note);
            assert_ne!(a.uid, b.uid);

            vm.close_all();
            assert_eq!(
                vm.active_context().get(),
                None,
                "closing every tab unfocuses"
            );
        }

        /// Two answers to one question must not drift: whatever else changes,
        /// the id in the seam's view is the id the app itself calls active.
        #[test]
        fn it_never_disagrees_with_active_item() {
            let vm = editors();
            let binder = seed_work(&vm);
            let one = seed_item(&vm, binder, "One", BinderItemSubRole::Scene, -1);
            let two = seed_item(&vm, binder, "Two", BinderItemSubRole::Scene, -1);
            let agree = |vm: &EditorsViewModel| {
                assert_eq!(
                    vm.active_item().get(),
                    vm.active_context().get().map(|c| c.id),
                    "the two views of `what is focused` disagree"
                );
            };

            agree(&vm);
            vm.open_in(Side::Primary, one, "One");
            agree(&vm);
            vm.set_split(true);
            vm.open_in(Side::Secondary, two, "Two");
            agree(&vm);
            vm.set_focused(Side::Primary);
            agree(&vm);
            vm.close_all();
            agree(&vm);
        }

        /// **The staleness Promote causes.** Retyping an item rewrites its
        /// `sub_role` in place: the id does not move, the pane does not change,
        /// the selection does not change. A `sub_role` cached on focus change
        /// alone would stay wrong until the writer happened to click elsewhere.
        #[test]
        fn retyping_the_focused_item_refreshes_its_sub_role() {
            let vm = editors();
            let binder = seed_work(&vm);
            let id = seed_item(&vm, binder, "One", BinderItemSubRole::Scene, -1);
            vm.open_in(Side::Primary, id, "One");
            assert_eq!(
                vm.active_context().get().map(|c| c.sub_role),
                Some(BinderItemSubRole::Scene)
            );

            binder_item_commands::update_binder_item(
                &vm.app_ctx,
                None,
                &UpdateBinderItemDto {
                    id,
                    sub_role: BinderItemSubRole::Note,
                    ..Default::default()
                },
            )
            .unwrap();
            // What `App`'s `BinderItem::Updated` subscription calls.
            vm.sync_active_item();

            assert_eq!(
                vm.active_context().get().map(|c| c.sub_role),
                Some(BinderItemSubRole::Note),
                "a Promote left the seam reporting the item's old type"
            );
        }
    }
}

/// [`EditorsViewModel::release_own_open_docs`] is the on_removed-driven
/// teardown's release step: it must release exactly the items *this*
/// window's own tabs (both panes) held open, and never touch what a
/// sibling window sharing the same `OpenDocsStore` (Phase 3's
/// `AttachExisting`) has open — the whole reason it exists instead of
/// `close_all`'s `docs.clear()`.
#[test]
fn release_own_open_docs_releases_only_this_windows_items_never_a_siblings() {
    use crate::models::OpenDoc;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    let app_ctx = Rc::new(AppContext::new());
    let save_state = SaveStateViewModel::new(app_ctx.clone(), AppIds::new());
    // One shared `OpenDocsStore` (Tier 2) — as it would be for two windows
    // onto the same Work.
    let docs = OpenDocsStore::new(app_ctx.clone());
    let ids_a = AppIds::new();
    let tree_expansion_a = crate::settings::TreeExpansionViewModel::new(
        app_ctx.clone(),
        ids_a.clone(),
        crate::models::TreeExpansionService::in_memory_default(),
    );
    let window_a = EditorsViewModel::new(
        app_ctx.clone(),
        Signal::new(700.0),
        Signal::new(true),
        Signal::new(crate::shared::SynopsisPlacement::default()),
        Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
        test_typography(),
        crate::shared::TypewriterSettings::off(),
        crate::shared::CaretHighlightSettings::off(),
        crate::settings::EditorViewMemory::detached(false),
        crate::settings::CorkboardDefaults::detached(),
        ids_a,
        docs.clone(),
        Signal::new(false),
        save_state.clone(),
        Signal::new(false),
        tree_expansion_a,
        Signal::new(false),
        Signal::new(620.0),
        crate::go::GoAvailability::new(),
        crate::format::FormatViewModel::detached(),
        crate::writing_session::WritingGamesViewModel::detached(),
        Signal::new(GoalUnit::default()),
    );
    let ids_b = AppIds::new();
    let tree_expansion_b = crate::settings::TreeExpansionViewModel::new(
        app_ctx.clone(),
        ids_b.clone(),
        crate::models::TreeExpansionService::in_memory_default(),
    );
    let window_b = EditorsViewModel::new(
        app_ctx.clone(),
        Signal::new(700.0),
        Signal::new(true),
        Signal::new(crate::shared::SynopsisPlacement::default()),
        Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
        test_typography(),
        crate::shared::TypewriterSettings::off(),
        crate::shared::CaretHighlightSettings::off(),
        crate::settings::EditorViewMemory::detached(false),
        crate::settings::CorkboardDefaults::detached(),
        ids_b,
        docs.clone(),
        Signal::new(false),
        save_state,
        Signal::new(false),
        tree_expansion_b,
        Signal::new(false),
        Signal::new(620.0),
        crate::go::GoAvailability::new(),
        crate::format::FormatViewModel::detached(),
        crate::writing_session::WritingGamesViewModel::detached(),
        Signal::new(GoalUnit::default()),
    );

    // Window A has item 1 in its primary pane and item 2 in its side pane;
    // window B (a sibling on the same Work) has item 3.
    push_tab(&window_a, Side::Primary, 1);
    push_tab(&window_a, Side::Secondary, 2);
    push_tab(&window_b, Side::Primary, 3);
    for id in [1u64, 2, 3] {
        docs.insert_for_test(Rc::new(OpenDoc::build(
            &app_ctx,
            id,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            docs.edited_any(),
            std::path::Path::new(""),
        )));
    }

    window_a.release_own_open_docs(None);

    assert!(
        docs.refs_for_test(1).is_none(),
        "window A's own primary-pane item must be released"
    );
    assert!(
        docs.refs_for_test(2).is_none(),
        "window A's own side-pane item must be released"
    );
    assert_eq!(
        docs.refs_for_test(3),
        Some(1),
        "window B's item must be untouched"
    );
}

#[test]
fn open_or_focus_dedupes_within_the_primary_pane() {
    let vm = editors();
    let id = push_tab(&vm, Side::Primary, 42);
    assert_eq!(vm.tabs(Side::Primary).len(), 1);
    vm.open_or_focus(42, "Scene"); // already open → focuses, no backend hit
    assert_eq!(vm.tabs(Side::Primary).len(), 1);
    assert_eq!(vm.selected(Side::Primary).get(), Some(id));
}

/// The segment signal of the open tab for `item_id` in `side`, if any.
#[cfg(feature = "mocks")]
fn tab_segment(
    vm: &EditorsViewModel,
    side: Side,
    item_id: u64,
) -> Option<Signal<Option<teksilo::widgets::SegmentId>>> {
    let pane = vm.pane(side);
    (0..pane.tabs.len()).find_map(|i| {
        pane.tabs
            .with_item(i, |h| {
                h.payload
                    .downcast_ref::<ContentTab>()
                    .filter(|t| t.item_id() == item_id)
                    .map(|t| t.segment.clone())
            })
            .flatten()
    })
}

/// Editing one item's scalar (a word-count goal, a rename) fires `BinderItem::Updated`
/// → `items_updated` → `retype`. `retype` must rebuild **only the updated item's own
/// tab, and only if that item's type actually changed** — never a tab merely because
/// some *other* open tab has a different type. This regressions the "set a Book's goal
/// and the Pace segment jumps back to Book" bug: with a differently-typed tab (a Scene)
/// also open, the goal edit rebuilt the Book's tab (fresh `ContentTab`, segment → 0).
#[cfg(feature = "mocks")]
#[test]
fn updating_an_item_does_not_rebuild_a_differently_typed_open_tab() {
    let vm = editors();
    vm.open_or_focus(101, "Book One"); // Folder/Book (mock fixture)
    vm.open_or_focus(103, "Scene at dawn"); // Item/Scene — a *different* type
    // Put the Book tab on the Pace segment — addressed by id, so this stays correct
    // however many segments precede it.
    let pace = Some(crate::tabs::shared::segments::segment_id(
        crate::tabs::shared::segments::SEG_PACE,
    ));
    tab_segment(&vm, Side::Primary, 101)
        .expect("book tab open")
        .set(pace);
    // Editing the Book's word-count goal fires `BinderItem::Updated` for the Book.
    vm.items_updated(&[101]);
    // The Book's tab must be the SAME one (its type did not change), so its segment
    // must still be Pace — not reset to 0 by a spurious rebuild.
    assert_eq!(
        tab_segment(&vm, Side::Primary, 101).map(|s| s.get()),
        Some(pace),
        "editing the Book's goal rebuilt its tab (segment reset) because another \
             differently-typed tab was open"
    );
}

#[test]
fn set_split_toggles_visibility_and_focus() {
    let vm = editors();
    assert!(!vm.split_active().get());
    vm.set_split(true);
    assert!(vm.split_active().get());
    assert!(vm.splitter().is_pane_visible(1));
    // Collapsing with a side tab open closes it and hides the pane.
    push_tab(&vm, Side::Secondary, 7);
    assert_eq!(vm.tabs(Side::Secondary).len(), 1);
    vm.set_split(false);
    assert!(!vm.split_active().get());
    assert!(!vm.splitter().is_pane_visible(1));
    assert_eq!(vm.tabs(Side::Secondary).len(), 0);
}

#[test]
fn set_focused_switches_active_item_between_panes() {
    let vm = editors();
    vm.set_split(true);
    let p = push_tab(&vm, Side::Primary, 42);
    vm.pane(Side::Primary).selected.set(Some(p));
    let s = push_tab(&vm, Side::Secondary, 7);
    vm.pane(Side::Secondary).selected.set(Some(s));

    // Focusing a pane (e.g. clicking into its editor) makes its selected item
    // the active "inspected" item — the split-view Inspector focus fix. In the
    // app, `focus_within` on each pane drives `set_focused`.
    vm.set_focused(Side::Primary);
    assert_eq!(vm.active_item().get(), Some(42));
    vm.set_focused(Side::Secondary);
    assert_eq!(vm.active_item().get(), Some(7));
    vm.set_focused(Side::Primary);
    assert_eq!(vm.active_item().get(), Some(42));
}

/// Trashing a row puts it away, and the tab has to go with it. The app used
/// to leave the tab open and merely turn it read-only behind a banner, which
/// left the writer looking at a document they had just deleted.
#[test]
fn trashing_an_item_closes_every_tab_showing_it() {
    let vm = editors();
    vm.set_split(true);
    push_tab(&vm, Side::Primary, 42);
    push_tab(&vm, Side::Secondary, 42);
    // A tab for something else, to prove this is aimed and not a sweep.
    push_tab(&vm, Side::Primary, 43);

    vm.close_trashed_tabs(42);

    assert!(
        vm.find_open(Side::Primary, 42).is_none(),
        "the trashed row is still open in the primary pane",
    );
    assert!(
        vm.find_open(Side::Secondary, 42).is_none(),
        "both panes have to let go, not just the focused one",
    );
    assert!(
        vm.find_open(Side::Primary, 43).is_some(),
        "trashing one row must not close its neighbour's tab",
    );
}

/// An item with no tab open is the ordinary case — trashing from the binder
/// without having opened it — and must not disturb anything.
#[test]
fn trashing_something_that_was_never_open_changes_nothing() {
    let vm = editors();
    push_tab(&vm, Side::Primary, 7);
    vm.close_trashed_tabs(99);
    assert_eq!(vm.tabs(Side::Primary).len(), 1);
}

#[test]
fn closing_last_side_tab_auto_collapses() {
    let vm = editors();
    vm.set_split(true);
    let id = push_tab(&vm, Side::Secondary, 9);
    vm.close_in(Side::Secondary, id);
    assert_eq!(vm.tabs(Side::Secondary).len(), 0);
    assert!(
        !vm.split_active().get(),
        "emptying the side pane collapses the split"
    );
}

#[test]
fn transfer_out_of_last_side_tab_collapses_the_split() {
    let vm = editors();
    vm.set_split(true);
    let id = push_tab(&vm, Side::Secondary, 3);
    // Simulate the framework's on_transfer_out (the tab dragged to the other
    // pane): it must empty the side pane AND collapse the split, like a close.
    vm.transfer_out(Side::Secondary, id);
    assert_eq!(vm.tabs(Side::Secondary).len(), 0);
    assert!(
        !vm.split_active().get(),
        "dragging out the last side tab collapses the split"
    );
}

#[test]
fn receive_tab_dedupes_against_the_target_pane() {
    let vm = editors();
    // The same item is open in both panes (two tabs, one item).
    push_tab(&vm, Side::Primary, 5);
    let existing = push_tab(&vm, Side::Secondary, 5);
    // Migrating the primary's tab into the side pane (which already has it)
    // must not create a duplicate; it focuses the existing side tab.
    let migrating = TabHandle::dynamic(
        TabId::fresh(),
        "editor",
        TabInfo::new().closable(true),
        tabs::tab_for(
            &vm.app_ctx,
            5,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            vm.column_width.clone(),
            vm.show_synopsis.clone(),
            vm.typography.clone(),
            vm.view_memory.clone(),
            &vm.ids,
        ),
    );
    vm.receive_tab(Side::Secondary, migrating);
    assert_eq!(
        vm.tabs(Side::Secondary).len(),
        1,
        "no duplicate in the side pane"
    );
    assert_eq!(vm.selected(Side::Secondary).get(), Some(existing));
}

#[test]
fn snapshot_helpers_report_order_selection_and_focus() {
    let vm = editors();
    // Primary: three tabs in order; select the middle one.
    let a = push_tab(&vm, Side::Primary, 10);
    let _b = push_tab(&vm, Side::Primary, 20);
    let _c = push_tab(&vm, Side::Primary, 30);
    vm.pane(Side::Primary).selected.set(Some(a));
    assert_eq!(
        vm.tab_item_ids(Side::Primary),
        vec![10, 20, 30],
        "tab order"
    );
    assert_eq!(
        vm.selected_item(Side::Primary),
        Some(10),
        "selected tab's item"
    );

    // Secondary pane + focus tracking.
    vm.set_split(true);
    let s = push_tab(&vm, Side::Secondary, 99);
    vm.pane(Side::Secondary).selected.set(Some(s));
    assert_eq!(vm.tab_item_ids(Side::Secondary), vec![99]);
    assert_eq!(vm.selected_item(Side::Secondary), Some(99));

    vm.set_focused(Side::Secondary);
    assert_eq!(vm.focused_side(), Side::Secondary);
    vm.set_focused(Side::Primary);
    assert_eq!(vm.focused_side(), Side::Primary);
}

#[test]
fn select_item_reselects_the_tab_for_an_item() {
    // The restore step: after re-opening a pane's tabs, re-mark the one that was
    // selected — by item id, since the `TabId`s are freshly minted on restore.
    let vm = editors();
    let first = push_tab(&vm, Side::Primary, 10);
    let _second = push_tab(&vm, Side::Primary, 20);
    // Currently the first is selected; ask to select item 20's tab.
    vm.pane(Side::Primary).selected.set(Some(first));
    vm.select_item(Side::Primary, 20);
    assert_eq!(vm.selected_item(Side::Primary), Some(20));
    // An item with no open tab is a no-op (selection unchanged).
    vm.select_item(Side::Primary, 12345);
    assert_eq!(vm.selected_item(Side::Primary), Some(20));
}

#[test]
fn close_all_empties_both_panes_and_resets_split() {
    let vm = editors();
    push_tab(&vm, Side::Primary, 1);
    vm.set_split(true);
    push_tab(&vm, Side::Secondary, 2);
    vm.close_all();
    assert_eq!(vm.tabs(Side::Primary).len(), 0);
    assert_eq!(vm.tabs(Side::Secondary).len(), 0);
    assert!(!vm.split_active().get());
    assert_eq!(vm.active_item().get(), None);
}

// ── Shared save state (the multi-window fix) ────────────────────────────

/// Single-window behaviour is unchanged: a clean editor reports `!is_unsaved`,
/// and `mark_clean` settles it after edits land.
#[test]
fn mark_clean_settles_the_dirty_flag() {
    let app_ctx = Rc::new(AppContext::new());
    let save_state = SaveStateViewModel::new(app_ctx.clone(), AppIds::new());
    let vm = editors_with(app_ctx, save_state.clone());

    assert!(!vm.is_unsaved(), "a freshly built editor starts clean");
    save_state.bump_dirty();
    assert!(vm.is_unsaved());
    vm.mark_clean();
    assert!(!vm.is_unsaved());
    assert_eq!(vm.saved_seq().get(), save_state.dirty_seq().get());
}

/// **Regression test for the actual multi-window bug.** Two
/// `EditorsViewModel`s — one per window — built over the *same*
/// `SaveStateViewModel` must agree on dirty/clean: whichever window's
/// `ProjectLifecycleViewModel` happens to call `mark_clean` (on load/new/close)
/// settles it for *every* window, not just its own.
///
/// Before this fix, each `EditorsViewModel` minted its own `saved_seq`/queue
/// from a `dirty_seq: Signal<u64>` that only happened to be shared: calling
/// `window_a.mark_clean()` would leave `window_b`'s own `saved_seq` stuck at
/// its old value, so `window_b.is_unsaved()` stayed `true` forever — a
/// permanent spurious "unsaved" for every window but the one that last
/// settled it.
#[test]
fn two_editors_view_models_sharing_one_save_state_agree_on_dirty_and_clean() {
    let app_ctx = Rc::new(AppContext::new());
    let save_state = SaveStateViewModel::new(app_ctx.clone(), AppIds::new());
    let window_a = editors_with(app_ctx.clone(), save_state.clone());
    let window_b = editors_with(app_ctx, save_state.clone());

    save_state.bump_dirty();
    assert!(window_a.is_unsaved());
    assert!(
        window_b.is_unsaved(),
        "both windows see the same dirty flag"
    );

    // Window A's lifecycle settles it (e.g. its `ProjectLifecycleViewModel`
    // ran `on_load`/`on_new`/`on_close`) — window B must see the same answer,
    // not a stale "unsaved" from a `saved_seq` nobody ever advanced for it.
    window_a.mark_clean();
    assert!(!window_a.is_unsaved());
    assert!(
        !window_b.is_unsaved(),
        "mark_clean is Work-scoped, not a per-window flag"
    );
    assert_eq!(window_a.saved_seq().get(), window_b.saved_seq().get());
}

/// **The production wiring of the extension seam's focus view, exercised.**
///
/// `ActiveContext::for_window` exists so `App::build` cannot choose which
/// signals to bridge; this proves the choice it makes is the live one, against
/// a real `EditorsViewModel`.
///
/// Without it the only coverage was `ActiveContext::new` over signals a test
/// made up — which passes just as happily if the call site hands over a fresh
/// `Signal::new(None)`, leaving every dock's focus tracking stuck on its
/// initial value with nothing to see.
#[test]
fn for_window_bridges_the_seam_to_this_windows_editors() {
    use crate::active_context::ActivePane;
    use crate::binder::OutlineViewModel;

    let vm = editors();
    let outline = OutlineViewModel::new_default(vm.app_ctx.clone(), vm.ids.clone());
    let cx = crate::active_context::ActiveContext::for_window(&vm, &outline);

    assert_eq!(cx.active_pane().get(), ActivePane::Primary);
    assert_eq!(cx.active_item().get(), None);

    // Drive the real view-model, not a stand-in.
    vm.set_focused(Side::Secondary);
    assert_eq!(
        cx.active_pane().get(),
        ActivePane::Secondary,
        "the seam context is not bridged to this window's editors"
    );

    vm.set_focused(Side::Primary);
    assert_eq!(cx.active_pane().get(), ActivePane::Primary);
}
