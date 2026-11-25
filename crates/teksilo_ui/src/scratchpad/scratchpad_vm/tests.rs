// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use crate::tabs::shared::segments::{
    SEG_ANALYSIS, SEG_CORKBOARD, SEG_MANUSCRIPT, SEG_OVERVIEW, SEG_OWN, SEG_SYNOPSIS, segment_id,
};

/// A caret in an editor of the tab showing `tab`, in `item`'s text.
#[cfg(not(feature = "mocks"))]
fn caret(item: u64, tab: u64) -> Option<CaretItem> {
    Some(CaretItem {
        item,
        tab: Some(tab),
    })
}

/// The rule, on its own: a stream follows the row, every other view the tab's item.
#[test]
fn the_target_is_the_row_in_a_stream_and_the_tab_item_elsewhere() {
    assert_eq!(resolve_target(None, false, None), None, "no tab, no target");
    assert_eq!(
        resolve_target(None, true, Some(7)),
        None,
        "a row without a tab"
    );
    assert_eq!(resolve_target(Some(3), false, None), Some(3));
    assert_eq!(
        resolve_target(Some(3), false, Some(7)),
        Some(3),
        "outside a stream the caret's row does not matter"
    );
    assert_eq!(resolve_target(Some(3), true, Some(7)), Some(7));
    assert_eq!(
        resolve_target(Some(3), true, None),
        Some(3),
        "a stream nobody has typed in yet shows the container"
    );
}

/// The two streams count; the container's own page, the Corkboard, the Overview and the
/// Analysis do not, and neither does a bar that has chosen nothing yet.
#[test]
fn only_the_full_text_and_the_full_synopsis_are_streams() {
    assert!(is_stream_segment(Some(segment_id(SEG_MANUSCRIPT))));
    assert!(is_stream_segment(Some(segment_id(SEG_SYNOPSIS))));
    for other in [SEG_OWN, SEG_CORKBOARD, SEG_OVERVIEW, SEG_ANALYSIS] {
        assert!(!is_stream_segment(Some(segment_id(other))), "{other}");
    }
    assert!(!is_stream_segment(None));
}

/// A search hit waits for the panel to show its item: asked for another item it stays,
/// taken for its own it is handed over, and it is taken only once. The project closing
/// forgets one still waiting. Whether its words are still there is the editor's
/// `Destination` to check (`search::editor_result`).
#[test]
fn a_search_hit_waits_for_its_item_and_is_taken_once() {
    use crate::search::editor_result::EditorHit;
    use frontend::common::entities::MatchField;

    let ctx = Rc::new(AppContext::new());
    let vm = ScratchpadViewModel::new(
        ctx.clone(),
        OpenDocsStore::new(ctx),
        crate::app_ids::AppIds::new(),
    );
    let hit = EditorHit {
        field: MatchField::Scratchpad,
        start: 7,
        matched: "keeper".into(),
    };
    let requests = vm.hit_requests().get();

    vm.select_when_shown(9, hit.clone());
    assert_ne!(vm.hit_requests().get(), requests, "the panel is told");
    assert_eq!(vm.take_hit_for(4), None, "not for another item");
    assert_eq!(vm.take_hit_for(9), Some(hit.clone()));
    assert_eq!(vm.take_hit_for(9), None, "taken once");

    vm.select_when_shown(9, hit);
    vm.release();
    assert_eq!(vm.take_hit_for(9), None, "the project closed");
}

#[cfg(not(feature = "mocks"))]
mod real {
    use super::*;
    use frontend::commands::{
        binder_commands, binder_item_commands, content_commands, work_commands,
    };
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};

    /// A real project: a chapter folder, three scenes and an end-of-book marker.
    struct Fixture {
        ctx: Rc<AppContext>,
        docs: OpenDocsStore,
        ids: AppIds,
        binder: u64,
        chapter: u64,
        scenes: Vec<u64>,
        book_end: u64,
    }

    impl Fixture {
        fn new() -> Self {
            let ctx = Rc::new(AppContext::new());
            let work = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default())
                .expect("a work")
                .id;
            let binder = binder_commands::create_binder(
                &ctx,
                None,
                &CreateBinderDto {
                    name: "Manuscript".into(),
                    activated: true,
                    ..Default::default()
                },
                work,
                0,
            )
            .expect("a binder");
            let item = |title: &str, role, sub_role, indent| {
                binder_item_commands::create_binder_item(
                    &ctx,
                    None,
                    &CreateBinderItemDto {
                        title: title.to_string(),
                        role,
                        sub_role,
                        activated: true,
                        is_exportable: true,
                        indent,
                        ..Default::default()
                    },
                    binder.id,
                    -1,
                )
                .expect("an item")
                .id
            };
            let chapter = item(
                "The harbour",
                BinderItemRole::Folder,
                BinderItemSubRole::ChapterScene,
                0,
            );
            let scenes = ["Low tide", "The keeper", "Fog"]
                .into_iter()
                .map(|t| item(t, BinderItemRole::Item, BinderItemSubRole::Scene, 1))
                .collect();
            let book_end = item("", BinderItemRole::Item, BinderItemSubRole::BookEnd, 0);
            let docs = OpenDocsStore::new(ctx.clone());
            let ids = AppIds::new();
            ids.work_id.set(Some(work));
            Self {
                ctx,
                docs,
                ids,
                binder: binder.id,
                chapter,
                scenes,
                book_end,
            }
        }

        fn vm(&self) -> ScratchpadViewModel {
            ScratchpadViewModel::new(self.ctx.clone(), self.docs.clone(), self.ids.clone())
        }

        /// The item's stored scratchpad text, if it has a row.
        fn stored_scratchpad(&self, item: u64) -> Option<String> {
            let ids = binder_item_commands::get_binder_item_relationship(
                &self.ctx,
                &item,
                &BinderItemRelationshipField::Contents,
            )
            .expect("contents");
            content_commands::get_content_multi(&self.ctx, &ids)
                .expect("rows")
                .into_iter()
                .flatten()
                .find(|c| c.role == ContentRole::ScratchpadText)
                .map(|c| c.data.trim().to_string())
        }
    }

    fn field(vm: &ScratchpadViewModel) -> Rc<ProseField> {
        match vm.state() {
            ScratchpadState::Ready { field, .. } => field,
            other => panic!("expected a scratchpad, got {other:?}"),
        }
    }

    fn type_into(field: &ProseField, text: &str) {
        field
            .doc
            .cursor_at(0)
            .insert_text(text)
            .expect("typing into the scratchpad");
    }

    /// In a scene tab the dock shows that scene's scratchpad, and follows the writer to
    /// the next tab.
    #[test]
    fn the_dock_follows_the_tab_in_front() {
        let f = Fixture::new();
        let vm = f.vm();
        assert!(matches!(vm.state(), ScratchpadState::NoItem));

        vm.view_changed(FrontView::page(f.scenes[0]), None);
        assert_eq!(vm.target().get(), Some(f.scenes[0]));
        assert_eq!(vm.title().get(), "Low tide");
        assert!(
            matches!(vm.state(), ScratchpadState::Ready { doc, .. } if doc.item_id == f.scenes[0])
        );

        vm.view_changed(
            FrontView::page(f.scenes[2]),
            caret(f.scenes[2], f.scenes[2]),
        );
        assert_eq!(vm.target().get(), Some(f.scenes[2]));
        assert_eq!(vm.title().get(), "Fog");

        vm.view_changed(FrontView::NOTHING, None);
        assert_eq!(vm.target().get(), None, "no tab, nothing shown");
        assert!(matches!(vm.state(), ScratchpadState::NoItem));
        assert_eq!(vm.title().get(), "");
    }

    /// In a Full Chapter the dock follows the row the caret is in, and stays on it while
    /// the caret is in the scratchpad itself (or any other dock): that reads as "not in a
    /// tab", which never moves the row. Leaving the stream for the Corkboard shows the
    /// chapter; coming back shows the row again; another tab forgets it.
    #[test]
    fn in_a_stream_the_dock_follows_the_carets_row_and_keeps_it() {
        let f = Fixture::new();
        let vm = f.vm();
        vm.view_changed(FrontView::stream(f.chapter), None);
        assert_eq!(
            vm.target().get(),
            Some(f.chapter),
            "nobody has typed in a row yet"
        );

        vm.caret_moved(caret(f.scenes[1], f.chapter));
        assert_eq!(vm.target().get(), Some(f.scenes[1]));
        assert_eq!(vm.title().get(), "The keeper");

        // The writer clicks into the scratchpad: the caret is in no tab editor.
        vm.caret_moved(None);
        assert_eq!(vm.target().get(), Some(f.scenes[1]), "sticky");

        vm.caret_moved(caret(f.scenes[2], f.chapter));
        assert_eq!(vm.target().get(), Some(f.scenes[2]));

        vm.segment_changed(Some(segment_id(SEG_CORKBOARD)));
        assert_eq!(
            vm.target().get(),
            Some(f.chapter),
            "the Corkboard shows the chapter"
        );
        vm.segment_changed(Some(segment_id(SEG_MANUSCRIPT)));
        assert_eq!(
            vm.target().get(),
            Some(f.scenes[2]),
            "back on the stream, the row the writer left"
        );

        vm.view_changed(FrontView::page(f.scenes[0]), None);
        assert_eq!(vm.target().get(), Some(f.scenes[0]));
        vm.view_changed(FrontView::stream(f.chapter), None);
        assert_eq!(
            vm.target().get(),
            Some(f.chapter),
            "another tab in between forgot the row"
        );
    }

    /// A change of view takes the caret where it is at that moment as the row, so a click
    /// that moved the pane and the caret in one gesture is followed whichever of the two
    /// signals fired first.
    #[test]
    fn a_change_of_view_reads_the_caret_where_it_is() {
        let f = Fixture::new();
        let vm = f.vm();
        vm.caret_moved(caret(f.scenes[1], f.chapter));
        vm.view_changed(FrontView::stream(f.chapter), caret(f.scenes[1], f.chapter));
        assert_eq!(vm.target().get(), Some(f.scenes[1]));
    }

    /// One reference while shown, given back before the next is taken, and nothing left
    /// over after A, B, A and a release. A tab's own reference is never touched.
    #[test]
    fn switching_items_keeps_the_reference_counts_balanced() {
        let f = Fixture::new();
        let (a, b) = (f.scenes[0], f.scenes[1]);
        // Both are open in tabs.
        let _tab_a = f.docs.open(a).expect("tab A");
        let _tab_b = f.docs.open(b).expect("tab B");
        let vm = f.vm();

        vm.view_changed(FrontView::page(a), None);
        assert_eq!(f.docs.refs_for_test(a), Some(2));
        assert_eq!(f.docs.refs_for_test(b), Some(1));
        vm.view_changed(FrontView::page(b), None);
        assert_eq!(f.docs.refs_for_test(a), Some(1));
        assert_eq!(f.docs.refs_for_test(b), Some(2));
        vm.view_changed(FrontView::page(a), None);
        assert_eq!(f.docs.refs_for_test(a), Some(2));
        assert_eq!(f.docs.refs_for_test(b), Some(1));
        // The same item again takes no second reference.
        vm.view_changed(FrontView::page(a), caret(a, a));
        assert_eq!(f.docs.refs_for_test(a), Some(2));

        vm.release();
        assert_eq!(f.docs.refs_for_test(a), Some(1));
        assert_eq!(f.docs.refs_for_test(b), Some(1));
        assert_eq!(vm.target().get(), None);
    }

    /// When the dock was the last to hold a document, moving on flushes the scratchpad
    /// typed into and lets the document go: nothing typed there is lost, and the store
    /// does not keep it open for ever.
    #[test]
    fn moving_on_from_a_document_only_the_dock_held_writes_it_and_lets_it_go() {
        let f = Fixture::new();
        let (a, b) = (f.scenes[0], f.scenes[1]);
        let vm = f.vm();
        vm.view_changed(FrontView::page(a), None);
        type_into(&field(&vm), "Check the tide tables.");
        if let Some(doc) = vm.doc() {
            (doc.mark_scratchpad_dirty_fn())();
        }

        vm.view_changed(FrontView::page(b), None);
        assert_eq!(f.docs.refs_for_test(a), None, "let go");
        assert_eq!(f.docs.refs_for_test(b), Some(1));
        assert_eq!(
            f.stored_scratchpad(a).as_deref(),
            Some("Check the tide tables."),
            "flushed on the way out"
        );
    }

    /// Two windows on one Work showing the same item edit one document: what one types,
    /// the other shows.
    #[test]
    fn two_windows_share_one_scratchpad() {
        let f = Fixture::new();
        let first = f.vm();
        let second = f.vm();
        first.view_changed(FrontView::page(f.scenes[1]), None);
        second.view_changed(FrontView::stream(f.chapter), caret(f.scenes[1], f.chapter));

        let (one, two) = (field(&first), field(&second));
        assert!(Rc::ptr_eq(&one, &two), "one field");
        assert_eq!(f.docs.refs_for_test(f.scenes[1]), Some(2));
        type_into(&one, "Ask about the keeper.");
        assert_eq!(
            two.doc.to_plain_text().unwrap_or_default().trim(),
            "Ask about the keeper."
        );

        first.release();
        assert_eq!(
            f.docs.refs_for_test(f.scenes[1]),
            Some(1),
            "the other still holds it"
        );
        second.release();
        assert_eq!(f.docs.refs_for_test(f.scenes[1]), None);
    }

    /// The caret is remembered per item, and a release forgets them all: the ids mean
    /// nothing in the next project.
    #[test]
    fn the_caret_is_remembered_per_item_until_release() {
        let f = Fixture::new();
        let vm = f.vm();
        vm.remember_caret(f.scenes[0], (4, 9));
        vm.remember_caret(f.scenes[1], (12, 12));
        vm.remember_caret(f.scenes[0], (5, 5));
        assert_eq!(vm.remembered_caret(f.scenes[0]), Some((5, 5)));
        assert_eq!(vm.remembered_caret(f.scenes[1]), Some((12, 12)));
        assert_eq!(vm.remembered_caret(f.scenes[2]), None);
        vm.release();
        assert_eq!(vm.remembered_caret(f.scenes[0]), None);
    }

    /// The end-of-book marker carries no text, so it has no scratchpad to show.
    #[test]
    fn a_marker_has_no_scratchpad() {
        let f = Fixture::new();
        let vm = f.vm();
        vm.view_changed(FrontView::page(f.book_end), None);
        assert_eq!(vm.target().get(), Some(f.book_end));
        assert!(matches!(vm.state(), ScratchpadState::NoScratchpad));
    }

    /// A Promote rebuilds the document in place. The store says so once the swap is
    /// done, the dock sees that the document it shows is no longer the store's, asks the
    /// panel to rebuild, and hands it the fresh document with a scratchpad that kept
    /// what was typed. No polling: the rebuild itself reaches the panel.
    #[test]
    fn a_promote_is_followed_to_the_rebuilt_document() {
        let f = Fixture::new();
        let id = f.scenes[0];
        let vm = f.vm();
        vm.view_changed(FrontView::page(id), None);
        let ScratchpadState::Ready { doc: before, field } = vm.state() else {
            panic!("a scene has a scratchpad");
        };
        vm.note_shown(&before);
        type_into(&field, "Move this after the storm.");
        (before.mark_scratchpad_dirty_fn())();
        vm.document_rebuilt();
        assert_eq!(vm.generation().get(), 0, "nothing moved yet");

        let mut dto = binder_item_commands::get_binder_item(&f.ctx, &id)
            .expect("read")
            .expect("the scene");
        dto.sub_role = BinderItemSubRole::ChapterScene;
        binder_item_commands::update_binder_item(&f.ctx, None, &dto.into()).expect("promote");
        let rebuilt = f.docs.rebuild(id, None).expect("rebuilt");
        assert_eq!(
            vm.generation().get(),
            1,
            "the rebuild itself asks the panel to rebuild"
        );
        vm.document_rebuilt();
        assert_eq!(vm.generation().get(), 1, "once");

        let ScratchpadState::Ready { doc, field } = vm.state() else {
            panic!("a flat chapter has a scratchpad too");
        };
        vm.note_shown(&doc);
        assert!(Rc::ptr_eq(&doc, &rebuilt), "the store's fresh document");
        assert_eq!(doc.sub_role, BinderItemSubRole::ChapterScene);
        assert_eq!(
            field.doc.to_plain_text().unwrap_or_default().trim(),
            "Move this after the storm."
        );
        assert_eq!(f.docs.refs_for_test(id), Some(1), "still one reference");

        // Another item's rebuild leaves the panel alone.
        let _other = f.docs.open(f.scenes[1]).expect("opens");
        f.docs.rebuild(f.scenes[1], None).expect("rebuilt");
        assert_eq!(vm.generation().get(), 1, "not this panel's document");
        f.docs.release(f.scenes[1], None);
    }

    /// Typing in the scratchpad bumps the store's edit counter, which the autosave and
    /// the unsaved mark observe, and the save's flush writes the row.
    #[test]
    fn typing_marks_the_project_changed_and_the_save_writes_the_row() {
        let f = Fixture::new();
        let vm = f.vm();
        vm.view_changed(FrontView::page(f.scenes[2]), None);
        let edited = f.docs.edited_any().get();
        let ScratchpadState::Ready { doc, field } = vm.state() else {
            panic!("a scene has a scratchpad");
        };
        type_into(&field, "The fog horn, twice.");
        (doc.mark_scratchpad_dirty_fn())();
        assert!(doc.dirty.get());
        assert_ne!(f.docs.edited_any().get(), edited);
        assert_eq!(f.stored_scratchpad(f.scenes[2]), None, "not written yet");

        assert!(f.docs.flush_all(None).is_empty());
        assert_eq!(
            f.stored_scratchpad(f.scenes[2]).as_deref(),
            Some("The fog horn, twice.")
        );
    }

    /// The header's name follows a rename.
    #[test]
    fn the_title_follows_a_rename() {
        let f = Fixture::new();
        let vm = f.vm();
        vm.view_changed(FrontView::page(f.scenes[0]), None);
        let mut dto = binder_item_commands::get_binder_item(&f.ctx, &f.scenes[0])
            .expect("read")
            .expect("the scene");
        dto.title = "Slack water".into();
        binder_item_commands::update_binder_item(&f.ctx, None, &dto.into()).expect("rename");
        vm.refresh_title();
        assert_eq!(vm.title().get(), "Slack water");
    }

    /// Scenario A. The caret is in row S of the Full Chapter; the writer types in the
    /// scratchpad (the caret is in no tab editor), then clicks back into the stream on
    /// a row's title or in the margin. That click is a focus edge of the pane, which the
    /// wiring pushes with the view unchanged and the caret, still read last frame, in no
    /// tab editor. The dock stays on S: it used to forget the row and jump to the
    /// chapter.
    #[test]
    fn clicking_back_into_the_stream_keeps_the_row() {
        let f = Fixture::new();
        let vm = f.vm();
        let s = f.scenes[1];
        vm.view_changed(FrontView::stream(f.chapter), None);
        vm.caret_moved(caret(s, f.chapter));
        assert_eq!(vm.target().get(), Some(s));

        // Into the scratchpad: next frame the caret reads as nowhere.
        vm.caret_moved(None);
        // Back into the pane, on a title: the focus edge, then the frame.
        vm.view_changed(FrontView::stream(f.chapter), None);
        vm.caret_moved(None);
        assert_eq!(
            vm.target().get(),
            Some(s),
            "still the row the writer was in"
        );
        assert_eq!(vm.title().get(), "The keeper");

        // The same edge from the other pane is a real change of view.
        let other = FrontView {
            pane: crate::editors::Side::Secondary,
            ..FrontView::stream(f.chapter)
        };
        vm.view_changed(other, None);
        assert_eq!(
            vm.target().get(),
            Some(f.chapter),
            "another view forgets it"
        );
    }

    /// Scenario B. The caret is in scene tab X; the writer clicks chapter C's tab, which
    /// shows its Full Chapter. The tab edge arrives while the caret, read once a frame,
    /// still names X's own editor; the frame after may still name it. Neither makes X
    /// the row of C's stream: X's editor is not one of C's tab.
    #[test]
    fn a_caret_left_in_another_tab_is_not_a_row_of_this_one() {
        let f = Fixture::new();
        let vm = f.vm();
        // X is a scene of the chapter itself, the case a stream-membership check alone
        // would get wrong.
        let x = f.scenes[0];
        vm.view_changed(FrontView::page(x), None);
        vm.caret_moved(caret(x, x));
        assert_eq!(vm.target().get(), Some(x));

        vm.view_changed(FrontView::stream(f.chapter), caret(x, x));
        assert_eq!(
            vm.target().get(),
            Some(f.chapter),
            "the stale caret is ignored"
        );
        vm.caret_moved(caret(x, x));
        assert_eq!(
            vm.target().get(),
            Some(f.chapter),
            "and so is the next frame's"
        );

        // The writer clicks into a row of C's stream: that one is followed.
        vm.caret_moved(caret(f.scenes[2], f.chapter));
        assert_eq!(vm.target().get(), Some(f.scenes[2]));
    }

    /// The row the dock follows is trashed, moved to another chapter, or deleted while
    /// the stream stays in front: it is no longer a row of that stream, so the dock goes
    /// back to the chapter rather than keep showing it.
    #[test]
    fn a_row_that_leaves_the_stream_is_let_go() {
        for how in ["trash", "move", "delete"] {
            let f = Fixture::new();
            let vm = f.vm();
            let row = f.scenes[1];
            vm.view_changed(FrontView::stream(f.chapter), None);
            vm.caret_moved(caret(row, f.chapter));
            assert_eq!(vm.target().get(), Some(row), "{how}");
            vm.structure_changed();
            assert_eq!(vm.target().get(), Some(row), "{how}: nothing moved yet");

            match how {
                "trash" => {
                    // What the Trash does to an item: it is no longer activated.
                    let mut dto = binder_item_commands::get_binder_item(&f.ctx, &row)
                        .expect("read")
                        .expect("the scene");
                    dto.activated = false;
                    binder_item_commands::update_binder_item(&f.ctx, None, &dto.into())
                        .expect("trash");
                }
                "move" => {
                    // Out to the top level, after the end-of-book marker.
                    let mut dto = binder_item_commands::get_binder_item(&f.ctx, &row)
                        .expect("read")
                        .expect("the scene");
                    dto.indent = 0;
                    binder_item_commands::update_binder_item(&f.ctx, None, &dto.into())
                        .expect("reindent");
                    let binder = f.binder;
                    let mut order = binder_commands::get_binder_relationship(
                        &f.ctx,
                        &binder,
                        &frontend::common::direct_access::binder::BinderRelationshipField::BinderItems,
                    )
                    .expect("order");
                    order.retain(|id| *id != row);
                    order.push(row);
                    binder_commands::set_binder_relationship(
                        &f.ctx,
                        None,
                        &frontend::direct_access::BinderRelationshipDto {
                            id: binder,
                            field: frontend::common::direct_access::binder::BinderRelationshipField::BinderItems,
                            right_ids: order,
                        },
                    )
                    .expect("move");
                }
                _ => {
                    f.docs.flush_all(None);
                    binder_item_commands::remove_binder_item(&f.ctx, None, &row).expect("delete");
                }
            }
            vm.structure_changed();
            assert_eq!(
                vm.target().get(),
                Some(f.chapter),
                "{how}: back to the chapter"
            );
        }
    }

    /// Merging the scene the dock follows into the one before it: the notes it showed
    /// are now on the scene that stays, so the dock goes there, and not back to the
    /// chapter as for a scene trashed, moved or deleted. The merge's entity events come
    /// first and empty the row (the merged scene is in the Trash), so the merge's own
    /// event is what names where it went; both orders are covered.
    #[test]
    fn a_merged_row_hands_the_dock_to_the_scene_it_went_into() {
        for entity_events_first in [true, false] {
            let f = Fixture::new();
            let vm = f.vm();
            let (survivor, merged) = (f.scenes[0], f.scenes[1]);
            vm.view_changed(FrontView::stream(f.chapter), None);
            vm.caret_moved(caret(merged, f.chapter));
            assert_eq!(vm.target().get(), Some(merged));

            frontend::commands::binder_item_management_commands::merge_two_scenes(
                &f.ctx,
                None,
                &frontend::binder_item_management::MergeTwoScenesDto {
                    work_id: f.ids.work_id.get().expect("a work"),
                    target_id: survivor,
                    source_id: merged,
                },
            )
            .expect("merge");
            if entity_events_first {
                vm.items_updated(&[merged]);
                vm.structure_changed();
                assert_eq!(
                    vm.target().get(),
                    Some(f.chapter),
                    "the trashed row alone sends it to the chapter"
                );
                vm.scenes_merged(survivor, merged);
            } else {
                vm.scenes_merged(survivor, merged);
                vm.structure_changed();
            }
            assert_eq!(
                vm.target().get(),
                Some(survivor),
                "the dock follows the notes into the scene that stays"
            );
        }
    }

    /// A merge of two other scenes leaves the dock where it was.
    #[test]
    fn a_merge_elsewhere_leaves_the_dock_alone() {
        let f = Fixture::new();
        let vm = f.vm();
        vm.view_changed(FrontView::stream(f.chapter), None);
        vm.caret_moved(caret(f.scenes[2], f.chapter));
        vm.scenes_merged(f.scenes[0], f.scenes[1]);
        assert_eq!(vm.target().get(), Some(f.scenes[2]));
    }

    /// An untitled chapter is named by its number. A chapter inserted before it renumbers
    /// it, an event about the binder and not about the chapter: the header follows.
    #[test]
    fn the_title_follows_a_renumbering() {
        let f = Fixture::new();
        // The chapter loses its title, so the manuscript names it by its number.
        let mut dto = binder_item_commands::get_binder_item(&f.ctx, &f.chapter)
            .expect("read")
            .expect("the chapter");
        dto.title = String::new();
        binder_item_commands::update_binder_item(&f.ctx, None, &dto.into()).expect("untitle");
        let work_id = f.ids.work_id.get().expect("the fixture's work");
        let mut work = work_commands::get_work(&f.ctx, &work_id)
            .expect("read")
            .expect("the work");
        work.number_chapters = true;
        work_commands::update_work(&f.ctx, None, &work.into()).expect("number the chapters");
        let vm = f.vm();
        crate::test_support::with_real_messages(|| {
            vm.view_changed(FrontView::page(f.chapter), None);
            let first = vm.title().get();
            assert!(first.contains('1'), "named by its number: {first}");

            // A new chapter at the top of the binder.
            binder_item_commands::create_binder_item(
                &f.ctx,
                None,
                &CreateBinderItemDto {
                    title: "Prologue at sea".into(),
                    role: BinderItemRole::Folder,
                    sub_role: BinderItemSubRole::ChapterScene,
                    activated: true,
                    is_exportable: true,
                    ..Default::default()
                },
                f.binder,
                0,
            )
            .expect("a chapter before it");
            vm.structure_changed();
            let second = vm.title().get();
            assert!(second.contains('2'), "renumbered: {second}");
        });
    }

    /// A search hit waiting for one item is dropped when the dock moves on to another:
    /// the writer went elsewhere, and selecting it the next time they pass by would be a
    /// surprise. Passing through nothing (no tab) keeps it.
    #[test]
    fn a_waiting_hit_is_dropped_when_the_dock_goes_elsewhere() {
        use crate::search::editor_result::EditorHit;
        use frontend::common::entities::MatchField;

        let f = Fixture::new();
        let vm = f.vm();
        let hit = EditorHit {
            field: MatchField::Scratchpad,
            start: 0,
            matched: "tide".into(),
        };
        vm.view_changed(FrontView::page(f.scenes[0]), None);
        vm.select_when_shown(f.scenes[2], hit.clone());
        vm.view_changed(FrontView::NOTHING, None);
        vm.view_changed(FrontView::page(f.scenes[2]), None);
        assert_eq!(
            vm.take_hit_for(f.scenes[2]),
            Some(hit.clone()),
            "it got there"
        );

        vm.select_when_shown(f.scenes[2], hit);
        vm.view_changed(FrontView::page(f.scenes[1]), None);
        vm.view_changed(FrontView::page(f.scenes[2]), None);
        assert_eq!(vm.take_hit_for(f.scenes[2]), None, "dropped on the way");
    }

    /// Releasing on a given stack flushes the held document on that stack, whatever the
    /// live id says: a window's teardown snapshots its stack before it begins, and by the
    /// time the dock lets go the live id may be cleared or already another project's.
    /// The title typed into the held document is undoable on the snapshotted stack.
    #[test]
    fn release_on_flushes_on_the_stack_it_is_given() {
        use frontend::commands::undo_redo_commands;

        let f = Fixture::new();
        let stack = undo_redo_commands::create_new_stack(&f.ctx);
        let vm = f.vm();
        let chapter = f.chapter;
        vm.view_changed(FrontView::page(chapter), None);
        let doc = vm.doc().expect("held");
        let title = doc.title.as_ref().expect("a chapter has a title field");
        title.value.set("The harbour at dawn".into());
        drop(doc);
        f.ids.stack_id.set(None);
        assert!(!undo_redo_commands::can_undo(&f.ctx, Some(stack)));

        vm.release_on(Some(stack));
        assert_eq!(f.docs.refs_for_test(chapter), None, "let go");
        assert!(
            undo_redo_commands::can_undo(&f.ctx, Some(stack)),
            "the rename went onto the stack it was given"
        );
    }
}
