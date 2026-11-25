// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use crate::tabs::shared::segments::{
    SEG_ANALYSIS, SEG_CORKBOARD, SEG_MANUSCRIPT, SEG_OVERVIEW, SEG_OWN, SEG_SYNOPSIS, segment_id,
};

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
            Self {
                ctx,
                docs,
                ids: AppIds::new(),
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

        vm.view_changed(Some(f.scenes[0]), false, None);
        assert_eq!(vm.target().get(), Some(f.scenes[0]));
        assert_eq!(vm.title().get(), "Low tide");
        assert!(
            matches!(vm.state(), ScratchpadState::Ready { doc, .. } if doc.item_id == f.scenes[0])
        );

        vm.view_changed(Some(f.scenes[2]), false, Some(f.scenes[2]));
        assert_eq!(vm.target().get(), Some(f.scenes[2]));
        assert_eq!(vm.title().get(), "Fog");

        vm.view_changed(None, false, None);
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
        vm.view_changed(Some(f.chapter), true, None);
        assert_eq!(
            vm.target().get(),
            Some(f.chapter),
            "nobody has typed in a row yet"
        );

        vm.caret_moved(Some(f.scenes[1]));
        assert_eq!(vm.target().get(), Some(f.scenes[1]));
        assert_eq!(vm.title().get(), "The keeper");

        // The writer clicks into the scratchpad: the caret is in no tab editor.
        vm.caret_moved(None);
        assert_eq!(vm.target().get(), Some(f.scenes[1]), "sticky");

        vm.caret_moved(Some(f.scenes[2]));
        assert_eq!(vm.target().get(), Some(f.scenes[2]));

        vm.segment_changed(false);
        assert_eq!(
            vm.target().get(),
            Some(f.chapter),
            "the Corkboard shows the chapter"
        );
        vm.segment_changed(true);
        assert_eq!(
            vm.target().get(),
            Some(f.scenes[2]),
            "back on the stream, the row the writer left"
        );

        vm.view_changed(Some(f.scenes[0]), false, None);
        assert_eq!(vm.target().get(), Some(f.scenes[0]));
        vm.view_changed(Some(f.chapter), true, None);
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
        vm.caret_moved(Some(f.scenes[1]));
        vm.view_changed(Some(f.chapter), true, Some(f.scenes[1]));
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

        vm.view_changed(Some(a), false, None);
        assert_eq!(f.docs.refs_for_test(a), Some(2));
        assert_eq!(f.docs.refs_for_test(b), Some(1));
        vm.view_changed(Some(b), false, None);
        assert_eq!(f.docs.refs_for_test(a), Some(1));
        assert_eq!(f.docs.refs_for_test(b), Some(2));
        vm.view_changed(Some(a), false, None);
        assert_eq!(f.docs.refs_for_test(a), Some(2));
        assert_eq!(f.docs.refs_for_test(b), Some(1));
        // The same item again takes no second reference.
        vm.view_changed(Some(a), false, Some(a));
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
        vm.view_changed(Some(a), false, None);
        type_into(&field(&vm), "Check the tide tables.");
        if let Some(doc) = vm.doc() {
            (doc.mark_scratchpad_dirty_fn())();
        }

        vm.view_changed(Some(b), false, None);
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
        first.view_changed(Some(f.scenes[1]), false, None);
        second.view_changed(Some(f.chapter), true, Some(f.scenes[1]));

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
        vm.view_changed(Some(f.book_end), false, None);
        assert_eq!(vm.target().get(), Some(f.book_end));
        assert!(matches!(vm.state(), ScratchpadState::NoScratchpad));
    }

    /// A Promote rebuilds the document in place. The dock notices, once a frame, that the
    /// document it shows is no longer the store's, asks the panel to rebuild, and hands it
    /// the fresh document with a scratchpad that kept what was typed.
    #[test]
    fn a_promote_is_followed_to_the_rebuilt_document() {
        let f = Fixture::new();
        let id = f.scenes[0];
        let vm = f.vm();
        vm.view_changed(Some(id), false, None);
        let ScratchpadState::Ready { doc: before, field } = vm.state() else {
            panic!("a scene has a scratchpad");
        };
        vm.note_shown(&before);
        type_into(&field, "Move this after the storm.");
        (before.mark_scratchpad_dirty_fn())();
        vm.poll_document();
        assert_eq!(vm.generation().get(), 0, "nothing moved yet");

        let mut dto = binder_item_commands::get_binder_item(&f.ctx, &id)
            .expect("read")
            .expect("the scene");
        dto.sub_role = BinderItemSubRole::ChapterScene;
        binder_item_commands::update_binder_item(&f.ctx, None, &dto.into()).expect("promote");
        let rebuilt = f.docs.rebuild(id, None).expect("rebuilt");

        vm.poll_document();
        assert_eq!(vm.generation().get(), 1, "the panel is asked to rebuild");
        vm.poll_document();
        assert_eq!(vm.generation().get(), 1, "once");
        let ScratchpadState::Ready { doc, field } = vm.state() else {
            panic!("a flat chapter has a scratchpad too");
        };
        assert!(Rc::ptr_eq(&doc, &rebuilt), "the store's fresh document");
        assert_eq!(doc.sub_role, BinderItemSubRole::ChapterScene);
        assert_eq!(
            field.doc.to_plain_text().unwrap_or_default().trim(),
            "Move this after the storm."
        );
        assert_eq!(f.docs.refs_for_test(id), Some(1), "still one reference");
    }

    /// Typing in the scratchpad bumps the store's edit counter, which the autosave and
    /// the unsaved mark observe, and the save's flush writes the row.
    #[test]
    fn typing_marks_the_project_changed_and_the_save_writes_the_row() {
        let f = Fixture::new();
        let vm = f.vm();
        vm.view_changed(Some(f.scenes[2]), false, None);
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
        vm.view_changed(Some(f.scenes[0]), false, None);
        let mut dto = binder_item_commands::get_binder_item(&f.ctx, &f.scenes[0])
            .expect("read")
            .expect("the scene");
        dto.title = "Slack water".into();
        binder_item_commands::update_binder_item(&f.ctx, None, &dto.into()).expect("rename");
        vm.refresh_title();
        assert_eq!(vm.title().get(), "Slack water");
    }
}
