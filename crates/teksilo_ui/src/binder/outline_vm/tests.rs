// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
#[cfg(feature = "mocks")]
use teksilo::data::TreeDataSource; // brings `visible_count` into scope

#[test]
fn outline_init_stack_creates_a_stack_id() {
    let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
    assert!(outline.stack_id_signal().get().is_none());
    outline.init_stack();
    assert!(
        outline.stack_id_signal().get().is_some(),
        "init_stack opens the per-Work undo stack"
    );
}

/// Build a DTO with just the fields [`redundant_of`] reads.
fn dto(id: u64, title: &str, langs: &[&str]) -> frontend::direct_access::BinderItemDto {
    frontend::direct_access::BinderItemDto {
        id,
        title: title.to_string(),
        dict_language: langs.iter().map(|s| s.to_string()).collect(),
        ..Default::default()
    }
}

fn numbered(
    id: u64,
    level: skribisto_model::compile::StreamLevel,
    n: usize,
) -> (u64, skribisto_model::numbering::Numbered) {
    (
        id,
        skribisto_model::numbering::Numbered {
            level,
            book: n,
            part: n,
            chapter: n,
        },
    )
}

/// A title that only restates its own number is offered for tidying — and one that
/// says anything else, or names a *different* number, is left alone.
#[test]
fn only_titles_restating_their_own_number_are_offered() {
    use skribisto_model::compile::StreamLevel::Chapter;
    let items = vec![
        dto(1, "Chapter 3", &[]),
        dto(2, "The Storm", &[]),
        dto(3, "chapter 5.", &[]), // case + a trailing mark
        dto(4, "Chapter 9", &[]),  // names a number that is not its own
        dto(5, "", &[]),           // already blank: nothing to clear
    ];
    let numbers: HashMap<_, _> = [
        numbered(1, Chapter, 3),
        numbered(2, Chapter, 4),
        numbered(3, Chapter, 5),
        numbered(4, Chapter, 6),
        numbered(5, Chapter, 7),
    ]
    .into_iter()
    .collect();
    let got: Vec<u64> = redundant_of(&items, &numbers, &["en-US".to_string()])
        .into_iter()
        .map(|(id, _)| id)
        .collect();
    assert_eq!(got, vec![1, 3]);
}

/// Judged in the language the row is actually written in: a French chapter inside an
/// English project is recognised, and an English one is not mistaken for it.
#[test]
fn redundancy_is_judged_in_the_rows_own_language() {
    use skribisto_model::compile::StreamLevel::Chapter;
    let items = vec![
        dto(1, "Chapitre 3", &["fr-FR"]),
        dto(2, "Chapitre 4", &[]), // untagged: falls back to the Work's language
    ];
    let numbers: HashMap<_, _> = [numbered(1, Chapter, 3), numbered(2, Chapter, 4)]
        .into_iter()
        .collect();
    let got: Vec<u64> = redundant_of(&items, &numbers, &["en-US".to_string()])
        .into_iter()
        .map(|(id, _)| id)
        .collect();
    assert_eq!(got, vec![1], "only the row written in French matches");

    // …and with the Work itself in French, the untagged row matches too.
    let got: Vec<u64> = redundant_of(&items, &numbers, &["fr-FR".to_string()])
        .into_iter()
        .map(|(id, _)| id)
        .collect();
    assert_eq!(got, vec![1, 2]);
}

/// A row carrying no ordinal — a scene, a note, a prologue the writer excluded — is
/// never a candidate, however its title reads.
#[test]
fn a_row_without_an_ordinal_is_never_offered() {
    let items = vec![dto(1, "Chapter 3", &[])];
    assert!(redundant_of(&items, &HashMap::new(), &["en-US".to_string()]).is_empty());
}

#[test]
fn outline_actions_on_empty_selection_are_noops() {
    let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
    // No selection, no loaded Work — these must not panic and must do nothing.
    outline.trash_selected();
    outline.duplicate_selected();
    outline.indent_selected();
    outline.outdent_selected();
}

#[test]
fn outline_show_hide_toggle_track_side_visibility() {
    let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
    let visible = outline.is_visible();

    // Sides start hidden.
    assert!(!visible.get());

    outline.show();
    assert!(visible.get(), "show() makes the side visible");

    outline.toggle();
    assert!(!visible.get(), "toggle() hides a visible side");

    outline.toggle();
    assert!(visible.get(), "toggle() re-shows a hidden side");

    outline.hide();
    assert!(!visible.get(), "hide() hides the side");
}

// The mock tree has content (2 binders, 13 items) so these assert the
// switcher/search signals drive the model's re-source end-to-end.
#[cfg(feature = "mocks")]
#[test]
fn set_binder_filter_scopes_the_tree() {
    let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
    let model = outline.model();
    assert_eq!(model.visible_count(), 15); // all binders
    outline.set_binder_filter(Some(1));
    assert_eq!(model.visible_count(), 12); // Manuscript = binder + 11 items
    outline.set_binder_filter(None);
    assert_eq!(model.visible_count(), 15);
}

#[cfg(feature = "mocks")]
#[test]
fn search_query_filters_the_tree() {
    let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::default());
    let model = outline.model();
    outline.search_query_signal().set("dawn".to_string());
    assert_eq!(model.visible_count(), 3); // Manuscript > Book One > Scene at dawn
    outline.clear_search();
    assert_eq!(model.visible_count(), 15);
}

// ── relation-aware creation (real backend: seed a Work/Binder/items and
// assert where `add_recommended` lands). Gated off `mocks`, whose tree model
// ignores `work_id` and re-sources a static fixture instead. ──
#[cfg(not(feature = "mocks"))]
mod recommend {
    use frontend::commands::content_commands;
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;

    use super::*;

    /// Seed an empty Work + Binder; return a VM wired to it (reloaded) and the
    /// binder id.
    pub(super) fn seed() -> (OutlineViewModel, u64) {
        let app_ctx = Rc::new(AppContext::new());
        let work = work_commands::create_orphan_work(
            &app_ctx,
            None,
            &frontend::direct_access::CreateWorkDto::default(),
        )
        .unwrap();
        let binder = binder_commands::create_binder(
            &app_ctx,
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
        let ids = AppIds::default();
        ids.work_id.set(Some(work.id));
        let outline = OutlineViewModel::new_default(app_ctx, ids);
        outline.reload();
        (outline, binder.id)
    }

    /// Append an item to `binder` at `index` (sequential = append) and reload.
    /// The tree key for a seeded item.
    ///
    /// The seed writes straight to the backend, so the tree has to re-source before
    /// the row (and therefore its key) exists — production gets that re-source from
    /// the `BinderItem(Created)` event, which a headless test has no source for.
    /// The tree key for a seeded binder.
    pub(super) fn binder_key_of(outline: &OutlineViewModel, binder_id: u64) -> BinderTreeKey {
        outline.reload();
        outline
            .key_for_binder(binder_id)
            .expect("the seeded binder must have a row in the tree")
    }

    pub(super) fn key_of(outline: &OutlineViewModel, item_id: u64) -> BinderTreeKey {
        outline.reload();
        outline
            .key_for_item(item_id)
            .expect("the seeded item must have a row in the tree")
    }

    pub(super) fn seed_item(
        outline: &OutlineViewModel,
        binder: u64,
        role: BinderItemRole,
        sub_role: BinderItemSubRole,
        indent: i64,
        index: i32,
    ) -> u64 {
        let dto = CreateBinderItemDto {
            status: None,
            title: format!("{role:?}/{sub_role:?}"),
            role,
            sub_role,
            activated: true,
            is_exportable: true,
            indent,
            ..Default::default()
        };
        let id =
            binder_item_commands::create_binder_item(&outline.app_ctx, None, &dto, binder, index)
                .unwrap()
                .id;
        outline.reload();
        id
    }

    fn order_of(outline: &OutlineViewModel, binder: u64) -> Vec<u64> {
        binder_commands::get_binder_relationship(
            &outline.app_ctx,
            &binder,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap()
    }

    use skribisto_model::CreateType;

    fn rec(create_type: CreateType, relation: Relation) -> Recommendation {
        Recommendation {
            create_type,
            relation,
        }
    }

    #[test]
    fn chapter_creates_a_folder_chapter_in_folder_mode() {
        let (outline, binder) = seed();
        let book = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::Book,
            0,
            0,
        );
        outline.add_recommended(
            Some(key_of(&outline, book)),
            &rec(CreateType::Chapter, Relation::Child),
        );
        let new = *order_of(&outline, binder).last().unwrap();
        let dto = outline.item_dto(new).unwrap();
        // Default (folder) mode → a Chapter is a Folder/ChapterScene.
        assert_eq!(dto.role, BinderItemRole::Folder);
        assert_eq!(dto.sub_role, BinderItemSubRole::ChapterScene);
    }

    #[test]
    fn child_appends_inside_a_book_before_its_book_end() {
        let (outline, binder) = seed();
        let book = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::Book,
            0,
            0,
        );
        let ch1 = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            1,
            1,
        );
        let end = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::BookEnd,
            1,
            2,
        );

        outline.add_recommended(
            Some(key_of(&outline, book)),
            &rec(CreateType::Chapter, Relation::Child),
        );

        let order = order_of(&outline, binder);
        assert_eq!(order.len(), 4);
        let new = order[2];
        // New chapter lands after the existing chapter but *before* BookEnd.
        assert_eq!(order, vec![book, ch1, new, end]);
        // …and at the book's child indent.
        assert_eq!(outline.item_dto(new).unwrap().indent, 1);
    }

    #[test]
    fn sibling_lands_after_the_anchor_folders_whole_subtree() {
        let (outline, binder) = seed();
        let chapter = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            0,
            0,
        );
        let s1 = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            1,
            1,
        );
        let s2 = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            1,
            2,
        );

        outline.add_recommended(
            Some(key_of(&outline, chapter)),
            &rec(CreateType::Chapter, Relation::Sibling),
        );

        let order = order_of(&outline, binder);
        let new = *order.last().unwrap();
        // After both scenes (not nested between the chapter and its children).
        assert_eq!(order, vec![chapter, s1, s2, new]);
        assert_eq!(outline.item_dto(new).unwrap().indent, 0);
    }

    #[test]
    fn parent_sibling_targets_the_enclosing_chapters_level() {
        let (outline, binder) = seed();
        let book = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::Book,
            0,
            0,
        );
        let chapter = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            1,
            1,
        );
        let s1 = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            2,
            2,
        );
        let s2 = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            2,
            3,
        );

        // Anchored on a deep scene: a new Chapter should start after the whole
        // enclosing chapter, at the chapter's own indent — not nested in it.
        outline.add_recommended(
            Some(key_of(&outline, s1)),
            &rec(CreateType::Chapter, Relation::ParentSibling),
        );

        let order = order_of(&outline, binder);
        let new = *order.last().unwrap();
        assert_eq!(order, vec![book, chapter, s1, s2, new]);
        assert_eq!(outline.item_dto(new).unwrap().indent, 1);
    }

    #[test]
    fn book_end_recommendation_is_gated_once_the_book_has_one() {
        let (outline, binder) = seed();
        let book = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::Book,
            0,
            0,
        );
        let _end = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::BookEnd,
            1,
            1,
        );

        let recs = outline.recommendations_for_key(Some(key_of(&outline, book)));
        assert!(
            recs.iter().all(|r| r.create_type != CreateType::EndOfBook),
            "End of Book should be hidden when the book already has one"
        );
    }

    #[test]
    fn binder_and_empty_selection_use_the_root_recommendations() {
        let (outline, binder) = seed();
        let expected = skribisto_model::recommendations_root();
        assert_eq!(
            outline.recommendations_for_key(Some(binder_key_of(&outline, binder))),
            expected
        );
        assert_eq!(outline.recommendations_for_key(None), expected);
    }

    #[test]
    fn promote_flips_scene_to_note() {
        let (outline, binder) = seed();
        let scene = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            0,
            0,
        );
        outline.promote(key_of(&outline, scene), PromoteTarget::Note);
        let dto = outline.item_dto(scene).unwrap();
        assert_eq!(dto.role, BinderItemRole::Item);
        assert_eq!(dto.sub_role, BinderItemSubRole::Note);
    }

    #[test]
    fn promote_flips_chapterscene_to_chapter_folder() {
        let (outline, binder) = seed();
        let cs = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::ChapterScene,
            0,
            0,
        );
        outline.promote(key_of(&outline, cs), PromoteTarget::ChapterFolder);
        let dto = outline.item_dto(cs).unwrap();
        assert_eq!(dto.role, BinderItemRole::Folder);
        assert_eq!(dto.sub_role, BinderItemSubRole::ChapterScene);
    }

    #[test]
    fn demote_blocked_children_counts_the_subtree() {
        let (outline, binder) = seed();
        let chapter = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            0,
            0,
        );
        let s1 = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            1,
            1,
        );
        let _s2 = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            1,
            2,
        );
        // The chapter folder holds two scenes, so collapsing it into a flat chapter
        // is blocked...
        assert_eq!(
            outline.demote_blocked_children(key_of(&outline, chapter), PromoteTarget::FlatChapter),
            2
        );
        // ...but becoming another *folder* is not: nothing is being collapsed.
        assert_eq!(
            outline.demote_blocked_children(key_of(&outline, chapter), PromoteTarget::PartFolder),
            0
        );
        // A leaf scene has no container to empty.
        assert_eq!(
            outline.demote_blocked_children(key_of(&outline, s1), PromoteTarget::Note),
            0
        );
    }

    /// A trashed child is not "in" the chapter any more — `activated = !trashed` and a
    /// trashed row keeps its slot and indent in the binder order, so counting the raw
    /// subtree span made the guard block a chapter the writer had already emptied. The
    /// prompt itself says "move or trash them", so trashing them all must unblock it.
    #[test]
    fn trashed_children_do_not_block_the_demote() {
        let (outline, binder) = seed();
        let chapter = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            0,
            0,
        );
        let s1 = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            1,
            1,
        );
        let s2 = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            1,
            2,
        );
        let blocked = |o: &OutlineViewModel| {
            o.demote_blocked_children(key_of(o, chapter), PromoteTarget::FlatChapter)
        };
        assert_eq!(blocked(&outline), 2);

        // Trash one: the other still blocks, and the count is now honest about it.
        outline.trash_keys(&[key_of(&outline, s1)]);
        assert_eq!(blocked(&outline), 1);

        // Trash the last one: the chapter is empty as far as the writer is concerned,
        // so the conversion goes through.
        outline.trash_keys(&[key_of(&outline, s2)]);
        assert_eq!(blocked(&outline), 0);

        // And it really converts — the guard was the only thing standing in the way.
        outline.promote(key_of(&outline, chapter), PromoteTarget::FlatChapter);
        let dto = outline.item_dto(chapter).unwrap();
        assert_eq!(dto.role, BinderItemRole::Item);
    }

    /// The other half of letting trashed children through the demote guard: those rows
    /// keep their indent, so the chapter they were nested in is now a *leaf* sitting
    /// right above them. Restoring one in place would strand a live row nested under a
    /// leaf — silently, since nothing else re-checks that. `restore_items` must instead
    /// report it `orphaned` and leave it indexed, which is what makes the trash dock
    /// open its destination picker.
    #[test]
    fn restoring_into_a_demoted_chapter_is_reported_orphaned() {
        let (outline, binder) = seed();
        let chapter = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            0,
            0,
        );
        let scene = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            1,
            1,
        );
        outline.trash_keys(&[key_of(&outline, scene)]);

        let work_id = outline.ids.work_id.get().unwrap();
        let infos = || {
            work_commands::get_work_relationship(
                &outline.app_ctx,
                &work_id,
                &WorkRelationshipField::TrashInfos,
            )
            .unwrap()
        };
        let restore = || {
            trash_management_commands::restore_items(
                &outline.app_ctx,
                None,
                &frontend::trash_management::RestoreItemsDto {
                    work_id,
                    trash_info_ids: infos().iter().map(|&x| x as i64).collect(),
                },
            )
            .unwrap()
        };

        // While the chapter is still a folder, the scene restores in place as before.
        let res = restore();
        assert!(
            !res.orphaned,
            "a live folder is a valid place to restore into"
        );
        assert_eq!(res.restored_count, 1);
        assert!(outline.item_dto(scene).unwrap().activated);
        assert!(infos().is_empty(), "a consumed TrashInfo is unlinked");

        // Now trash it again and collapse the chapter under it.
        outline.trash_keys(&[key_of(&outline, scene)]);
        outline.promote(key_of(&outline, chapter), PromoteTarget::FlatChapter);
        assert_eq!(
            outline.item_dto(chapter).unwrap().role,
            BinderItemRole::Item
        );

        let res = restore();
        assert!(
            res.orphaned,
            "the chapter is a leaf now — the scene has nowhere to be restored *into*"
        );
        assert_eq!(res.restored_count, 0);
        assert!(
            !outline.item_dto(scene).unwrap().activated,
            "it must stay trashed rather than come back nested under a leaf"
        );
        assert_eq!(
            infos().len(),
            1,
            "its TrashInfo stays indexed — that is what drives the destination picker"
        );
    }

    /// An item's name has two homes: `BinderItem.title`, which the outline tree and
    /// the tab show, and a title `Content` row, which compiles into the manuscript.
    /// **Renaming through either door must reach both.**
    ///
    /// Renaming a chapter in the tree used to write only the entity field, leaving the
    /// manuscript compiling the old title; renaming it in its editor wrote only the
    /// content row, leaving the tree and the tab showing the old name. Both now go
    /// through `SingleBinderItem::set_title`.
    #[test]
    fn a_rename_reaches_both_homes_of_the_title() {
        let (outline, binder) = seed();
        let ch = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            0,
            0,
        );

        outline.rename(key_of(&outline, ch), "The Long Road");

        // The entity field the tree and the tab read...
        assert_eq!(outline.item_dto(ch).unwrap().title, "The Long Road");
        // ...and the content row the manuscript compiles.
        let content_ids = binder_item_commands::get_binder_item_relationship(
            outline.app_ctx(),
            &ch,
            &BinderItemRelationshipField::Contents,
        )
        .unwrap();
        let chapter_title = content_commands::get_content_multi(outline.app_ctx(), &content_ids)
            .unwrap()
            .into_iter()
            .flatten()
            .find(|c| c.role == ContentRole::ChapterTitle)
            .map(|c| c.data);
        assert_eq!(
            chapter_title.as_deref(),
            Some("The Long Road"),
            "a tree rename must reach the title the manuscript compiles"
        );
    }

    /// The other half of the rename story: `a_rename_reaches_both_homes_of_the_title`
    /// calls `rename` directly with a freshly-resolved key; this drives the **actual**
    /// context-menu wiring end to end — `begin_rename` (the `MenuItem`'s
    /// `on_activate_fn`) presenting the `InputDialog`, then its `on_result` closure
    /// firing *later* (after layout, after a `SetValue`/`Click` round trip) and
    /// resolving `key` again through the same live tree model.
    ///
    /// Guards the three things `BinderTreeKey`'s uid re-keying put at risk: the
    /// dialog actually appears (a modal request is queued), it's pre-filled with the
    /// row's current title, and the deferred `on_result` still finds the row and
    /// applies the edit rather than silently no-oping.
    #[test]
    fn the_rename_dialog_appears_and_a_submitted_title_actually_renames() {
        use teksilo::core::ModalContent;
        use teksilo::core::accessibility::widget_id_to_node_id;
        use teksilo::core::widget_id::WidgetId;
        use teksilo::core::widget_tree::WidgetTree;
        use teksilo::i18n::lit;
        use teksilo::widgets::Button;

        let (outline, binder) = seed();
        let item = seed_item(
            &outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            0,
            0,
        );
        let key = key_of(&outline, item);

        // Mirrors `binder_context_menu`'s "Rename" `MenuItem` exactly:
        // `.on_activate_fn(move |ctx| rename.begin_rename(key, ctx))`.
        let vm = outline.clone();
        let mut tree = WidgetTree::new().with_theme(intui::light());
        let trigger = tree
            .add(Button::new(lit!("rename")).on_activate_fn(move |ctx| vm.begin_rename(key, ctx)));
        tree.layout(SizeProposal::exact(420.0, 60.0));

        tree.dispatch_event(WidgetEvent::AccessAction {
            action: teksilo::core::accesskit::Action::Click,
            target: Some(trigger),
            target_node: widget_id_to_node_id(trigger),
            data: None,
        });

        assert!(
            tree.has_pending_modal_requests(),
            "the dialog must appear: begin_rename must queue a modal request"
        );
        let request = tree.drain_pending_modal_requests().pop().unwrap().request;
        let ModalContent::Deferred(builder) = request.content else {
            panic!("InputDialog must present as deferred content");
        };
        let content_id = builder(&mut tree);
        tree.layout(SizeProposal::exact(420.0, 180.0));

        // Collect descendants by type name — locale-independent, unlike hunting the
        // OK/Cancel buttons by their translated label.
        fn collect(tree: &WidgetTree, root: WidgetId, needle: &str, out: &mut Vec<WidgetId>) {
            if tree
                .widget_type_name(root)
                .is_some_and(|t| t.contains(needle))
            {
                out.push(root);
            }
            for c in tree.children(root) {
                collect(tree, c, needle, out);
            }
        }

        let mut fields = Vec::new();
        collect(&tree, content_id, "TextInputField", &mut fields);
        let field = *fields
            .first()
            .expect("the InputDialog must mount its text field");
        {
            let update = tree.sync_accessibility();
            let field_node = widget_id_to_node_id(field);
            let value = update
                .nodes
                .iter()
                .find(|(id, _)| *id == field_node)
                .and_then(|(_, n)| n.value());
            assert_eq!(
                value,
                Some("Item/Scene"),
                "begin_rename must read the row's current title via node_of and \
                     pre-fill the dialog with it"
            );
        }

        // Overwrite the pre-filled title (proving the field is live), the
        // AT-driven equivalent of selecting all and typing.
        tree.dispatch_event(WidgetEvent::AccessAction {
            action: teksilo::core::accesskit::Action::SetValue,
            target: Some(field),
            target_node: widget_id_to_node_id(field),
            data: Some(teksilo::core::accesskit::ActionData::Value(
                "Renamed Scene".into(),
            )),
        });

        // The field's edit is debounced onto its outer `Signal<String>` via a
        // frame-tick effect (`TextInputField`'s doc comment: "frames are
        // demand-driven"), so the OK button's `text_for_ok.get()` won't see it
        // until a frame actually ticks.
        tree.request_frame();
        tree.tick_animations(std::time::Duration::from_millis(16));
        tree.layout(SizeProposal::exact(420.0, 180.0));

        let mut buttons = Vec::new();
        collect(&tree, content_id, "Button", &mut buttons);
        // Footer order is Cancel, then OK (`InputDialogBody::build`) — the last
        // Button is OK.
        let ok = *buttons
            .last()
            .expect("the InputDialog must mount its OK/Cancel buttons");

        tree.dispatch_event(WidgetEvent::AccessAction {
            action: teksilo::core::accesskit::Action::Click,
            target: Some(ok),
            target_node: widget_id_to_node_id(ok),
            data: None,
        });

        assert_eq!(
            outline.item_dto(item).unwrap().title,
            "Renamed Scene",
            "the on_result closure must resolve the key later, through the live tree \
                 model, and actually apply the rename"
        );
    }

    /// The headline of this feature: outline in bare folders, then declare what each
    /// one is. A plain folder carries only a synopsis, which every folder type allows,
    /// so every one of these conversions is lossless.
    #[test]
    fn a_plain_folder_becomes_any_other_kind_of_folder() {
        for (target, want) in [
            (
                PromoteTarget::ChapterFolder,
                BinderItemSubRole::ChapterScene,
            ),
            (PromoteTarget::PartFolder, BinderItemSubRole::Part),
            (PromoteTarget::BookFolder, BinderItemSubRole::Book),
            (PromoteTarget::NoteFolder, BinderItemSubRole::Note),
        ] {
            let (outline, binder) = seed();
            let f = seed_item(
                &outline,
                binder,
                BinderItemRole::Folder,
                BinderItemSubRole::None,
                0,
                0,
            );
            assert!(
                outline
                    .promote_targets_of(key_of(&outline, f))
                    .contains(&target),
                "a plain folder must offer {target:?}"
            );
            outline.promote(key_of(&outline, f), target);
            let dto = outline.item_dto(f).unwrap();
            assert_eq!(dto.role, BinderItemRole::Folder);
            assert_eq!(dto.sub_role, want, "promoting to {target:?}");
        }
    }

    // ── the created row is named for its type, and is actually visible ──

    use teksilo::data::TreeDataSource;

    /// Every creatable type gets its own default title. The bug this pins:
    /// the title used to be derived from `role` alone, which cannot tell a
    /// chapter from a book from a note folder — they are all `Folder` — so
    /// six of the eight types came out as the same "New Folder".
    #[test]
    fn a_created_row_is_titled_for_its_type_not_generically() {
        let mut seen: Vec<String> = Vec::new();
        for create_type in [
            CreateType::Book,
            CreateType::Part,
            CreateType::Chapter,
            CreateType::Scene,
            CreateType::Note,
            CreateType::NoteFolder,
            CreateType::Folder,
        ] {
            let (outline, binder) = seed();
            outline.add_recommended(None, &rec(create_type, Relation::Child));
            let created = order_of(&outline, binder);
            assert_eq!(
                created.len(),
                1,
                "{create_type:?} must create exactly one row"
            );
            let title = outline.item_dto(created[0]).unwrap().title;
            assert!(
                !title.is_empty(),
                "{create_type:?} must not be created untitled"
            );
            // The row carries this type's own default, resolved to data at
            // creation — not a title derived from `role`, which collapses six
            // distinct types onto one string.
            let want: String = crate::binder::create_labels::default_title(create_type).into();
            assert_eq!(
                title, want,
                "{create_type:?} must be titled from its own vocabulary"
            );
            assert_ne!(
                title, "New Item",
                "{create_type:?} still falls back to the old generic title"
            );
            seen.push(title);
        }
        // Distinct types must read distinctly — otherwise "adapt the name to the
        // type" is satisfied only in the letter.
        let mut unique = seen.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(
            unique.len(),
            seen.len(),
            "each type needs its own title, got duplicates in {seen:?}"
        );
    }

    /// **C1a: the story-bible entry vocabulary item creates its row immediately,
    /// exactly like every other sibling, and never defers creation behind a
    /// modal.** `add_recommended_returning_id` is the mechanism the caller in
    /// `app::commands::binder` opens the story-bible configuration modal from,
    /// so this is the guarantee that no modal stands between the click and the
    /// row existing: the row is already a real, titled, correctly-typed member
    /// of the binder by the time this call returns, and the returned id names
    /// exactly that row.
    #[test]
    fn story_bible_entry_creates_immediately_and_returns_its_row_id() {
        let (outline, binder) = seed();

        let created = outline
            .add_recommended_returning_id(None, &rec(CreateType::StoryBibleEntry, Relation::Child))
            .expect("a story-bible entry must create its row immediately, not defer to a modal");

        let order = order_of(&outline, binder);
        assert_eq!(
            order,
            vec![created],
            "the row must already be in the binder"
        );

        let item = outline
            .item_dto(created)
            .expect("the row must be readable back");
        assert_eq!(item.role, BinderItemRole::Item);
        assert_eq!(item.sub_role, BinderItemSubRole::Note);
        let want_title: String =
            crate::binder::create_labels::default_title(CreateType::StoryBibleEntry).into();
        assert_eq!(
            item.title, want_title,
            "titled from its own vocabulary entry, not a generic placeholder"
        );
    }

    /// The reported bug: creating the **first** child of a container left the
    /// parent collapsed, so the new row was real, selected, and invisible.
    /// A container with no children has never been expanded — there was no
    /// twist to open — so nothing put it in the expanded set.
    #[test]
    fn creating_a_first_child_expands_the_parent_that_had_none() {
        let (outline, binder) = seed();
        let chapter = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            0,
            0,
        );
        let chapter_key = key_of(&outline, chapter);
        assert!(
            !outline.model.is_expanded(&chapter_key),
            "precondition: a childless container starts collapsed"
        );

        outline.add_recommended(Some(chapter_key), &rec(CreateType::Scene, Relation::Child));

        assert!(
            outline.model.is_expanded(&chapter_key),
            "the parent must be expanded so its first child is visible"
        );
        // …and the new row is what the writer is now pointed at.
        let scene = *order_of(&outline, binder)
            .iter()
            .find(|id| **id != chapter)
            .expect("the scene must exist");
        assert_eq!(
            outline.selection.selected_keys().first().copied(),
            Some(key_of(&outline, scene)),
            "the newly created row must be selected"
        );
    }

    /// Creating must not drag the outline dock open. The same create path
    /// serves the Corkboard and Overview header buttons, so forcing the dock
    /// forward on every create interrupts a writer who is working elsewhere.
    /// Expanding + selecting is the whole job; showing is `reveal_item`'s.
    #[test]
    fn creating_selects_without_forcing_the_outline_open() {
        let (outline, binder) = seed();
        let chapter = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            0,
            0,
        );
        outline.hide();
        assert!(
            !outline.is_visible().get(),
            "precondition: the outline is hidden"
        );

        outline.add_recommended(
            Some(key_of(&outline, chapter)),
            &rec(CreateType::Scene, Relation::Child),
        );

        assert!(
            !outline.is_visible().get(),
            "a create must not pop the outline open"
        );
        assert!(
            outline.model.is_expanded(&key_of(&outline, chapter)),
            "…but the parent must still be expanded"
        );
    }

    /// Expansion must reach *every* ancestor, not just the immediate parent —
    /// a create can land several levels below anything currently open.
    #[test]
    fn revealing_opens_the_whole_ancestor_chain() {
        let (outline, binder) = seed();
        let book = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::Book,
            0,
            0,
        );
        let part = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::Part,
            1,
            1,
        );
        let chapter = seed_item(
            &outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            2,
            2,
        );
        for k in [
            key_of(&outline, book),
            key_of(&outline, part),
            key_of(&outline, chapter),
        ] {
            outline.model.set_expanded(&k, false);
        }

        outline.add_recommended(
            Some(key_of(&outline, chapter)),
            &rec(CreateType::Scene, Relation::Child),
        );

        for (id, what) in [(book, "book"), (part, "part"), (chapter, "chapter")] {
            assert!(
                outline.model.is_expanded(&key_of(&outline, id)),
                "the {what} ancestor must be expanded too"
            );
        }
    }
}

// ── explicit language propagation ("Apply to children") ──
//
// Nothing inherits a language from a container any more (`skribisto_model::language`
// resolves an item's own tag, else the Work's), so this button is the *only* way a
// language reaches a subtree. These assert the write itself; `skribisto_model` owns the
// resolution rule these values then feed.
#[cfg(not(feature = "mocks"))]
mod apply_language {
    use super::recommend::{seed, seed_item};
    use super::*;
    /// Space-separated in the tests, a list in storage — one parser, shared.
    /// Scoped to this module: it is the only one that uses it, and at the
    /// `tests` level it read as unused under `--features mocks`, where the
    /// module is compiled out.
    use skribisto_model::language::parse_legacy_list as tags;
    use std::collections::HashMap;

    /// Book > Chapter > Scene, plus an outsider after the chapter's subtree, so the
    /// blast radius is observable in both directions.
    fn seed_tree(outline: &OutlineViewModel, binder: u64) -> (u64, u64, u64, u64) {
        let book = seed_item(
            outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::Book,
            0,
            0,
        );
        let chapter = seed_item(
            outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            1,
            1,
        );
        let scene = seed_item(
            outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            2,
            2,
        );
        // A sibling scene back at the Book's level — NOT part of the chapter's subtree.
        let outsider = seed_item(
            outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            1,
            3,
        );
        (book, chapter, scene, outsider)
    }

    fn lang_of(outline: &OutlineViewModel, id: u64) -> Vec<String> {
        outline
            .item_dto(id)
            .map(|d| d.dict_language)
            .unwrap_or_default()
    }

    #[test]
    fn writes_every_descendant_and_leaves_everything_else_alone() {
        let (outline, binder) = seed();
        let (book, chapter, scene, outsider) = seed_tree(&outline, binder);

        outline.apply_dict_language_to_subtree(chapter, &tags("tr-TR"));

        assert_eq!(
            lang_of(&outline, scene),
            tags("tr-TR"),
            "the descendant is written"
        );
        assert_eq!(
            lang_of(&outline, book),
            tags(""),
            "an ancestor is untouched"
        );
        assert_eq!(
            lang_of(&outline, outsider),
            tags(""),
            "a non-descendant is untouched"
        );
        assert_eq!(
            lang_of(&outline, chapter),
            tags(""),
            "the item itself is untouched — the pill field beside the button owns that"
        );
    }

    /// The whole point of the composite: an over-broad apply is one Ctrl+Z, not one per
    /// descendant.
    #[test]
    fn is_a_single_undo_step() {
        let (outline, binder) = seed();
        outline.init_stack();
        let (_book, chapter, scene, _outsider) = seed_tree(&outline, binder);
        let stack = outline.stack();

        outline.apply_dict_language_to_subtree(chapter, &tags("tr-TR"));
        assert_eq!(lang_of(&outline, scene), tags("tr-TR"));

        undo_redo_commands::undo(&outline.app_ctx, stack).unwrap();
        assert_eq!(
            lang_of(&outline, scene),
            tags(""),
            "one undo reverses the whole apply, not just the last descendant"
        );

        undo_redo_commands::redo(&outline.app_ctx, stack).unwrap();
        assert_eq!(
            lang_of(&outline, scene),
            tags("tr-TR"),
            "and redo puts it back"
        );
    }

    /// An empty list is a legitimate value to push: it resets the subtree to inheriting
    /// the Work's language — the only way to undo an over-broad apply after the fact
    /// without visiting every child by hand.
    #[test]
    fn an_empty_list_resets_descendants_to_the_work_language() {
        let (outline, binder) = seed();
        let (_book, chapter, scene, _outsider) = seed_tree(&outline, binder);
        outline.apply_dict_language_to_subtree(chapter, &tags("tr-TR"));
        assert_eq!(lang_of(&outline, scene), tags("tr-TR"));

        outline.apply_dict_language_to_subtree(chapter, &tags(""));
        assert_eq!(
            lang_of(&outline, scene),
            tags(""),
            "cleared back to inheriting"
        );

        // And the resolver then hands it the Work's language — the two halves meeting.
        let items = vec![frontend::common::entities::BinderItem {
            id: scene,
            dict_language: lang_of(&outline, scene),
            ..Default::default()
        }];
        let mut out = HashMap::new();
        skribisto_model::language::tags_in_binder(&tags("en-US"), &items, &mut out);
        assert_eq!(out[&scene], tags("en-US"));
    }

    /// A leaf has no subtree, so the button is never offered — and the call is inert if
    /// it somehow fires anyway.
    #[test]
    fn a_leaf_has_no_descendants_and_the_call_is_a_noop() {
        let (outline, binder) = seed();
        let (_book, _chapter, scene, _outsider) = seed_tree(&outline, binder);
        assert!(
            outline.subtree_descendants(scene).is_empty(),
            "the button's own gate"
        );
        outline.apply_dict_language_to_subtree(scene, &tags("tr-TR"));
        assert_eq!(
            lang_of(&outline, scene),
            tags(""),
            "an inert call writes nothing"
        );
    }
}

// ── explicit Book-filing propagation ("Apply to children") ──
//
// `books` is a **relationship** (a junction table), not a scalar list like
// `dict_language`, so unlike `apply_language` above this also proves the
// write actually reaches the junction rather than the entity's own scalar
// field; see `set_descendants_books_uc`'s own header for why the scalar
// `GetMulti`/`UpdateMulti` path would silently no-op for this field.
#[cfg(not(feature = "mocks"))]
mod apply_books {
    use super::recommend::{seed, seed_item};
    use super::*;

    /// Book > Chapter > Scene, plus an outsider after the chapter's subtree.
    /// Same shape as `apply_language`'s `seed_tree`, plus two more `Folder/
    /// Book` rows to serve as filing targets. Any live `BinderItem` id is a
    /// legal target for `books` (nothing in the schema enforces that a
    /// target actually resolves to a `Folder/Book`; see the field's own doc
    /// comment), but using real Book rows keeps the fixture honest.
    fn seed_tree(outline: &OutlineViewModel, binder: u64) -> (u64, u64, u64, u64, u64, u64) {
        let book = seed_item(
            outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::Book,
            0,
            0,
        );
        let chapter = seed_item(
            outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::ChapterScene,
            1,
            1,
        );
        let scene = seed_item(
            outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            2,
            2,
        );
        // A sibling scene back at the Book's level: NOT part of the chapter's subtree.
        let outsider = seed_item(
            outline,
            binder,
            BinderItemRole::Item,
            BinderItemSubRole::Scene,
            1,
            3,
        );
        let book_one = seed_item(
            outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::Book,
            0,
            4,
        );
        let book_two = seed_item(
            outline,
            binder,
            BinderItemRole::Folder,
            BinderItemSubRole::Book,
            0,
            5,
        );
        (book, chapter, scene, outsider, book_one, book_two)
    }

    fn books_of(outline: &OutlineViewModel, id: u64) -> Vec<u64> {
        outline.item_dto(id).map(|d| d.books).unwrap_or_default()
    }

    #[test]
    fn writes_every_descendant_and_leaves_everything_else_alone() {
        let (outline, binder) = seed();
        let (book, chapter, scene, outsider, book_one, book_two) = seed_tree(&outline, binder);

        outline.apply_books_to_subtree(chapter, &[book_one, book_two]);

        assert_eq!(
            books_of(&outline, scene),
            vec![book_one, book_two],
            "the descendant is written"
        );
        assert!(
            books_of(&outline, book).is_empty(),
            "an ancestor is untouched"
        );
        assert!(
            books_of(&outline, outsider).is_empty(),
            "a non-descendant is untouched"
        );
        assert!(
            books_of(&outline, chapter).is_empty(),
            "the item itself is untouched: the Inspector's own Books section owns that"
        );
    }

    /// The whole point of the composite: an over-broad apply is one Ctrl+Z, not
    /// one per descendant.
    #[test]
    fn is_a_single_undo_step() {
        let (outline, binder) = seed();
        outline.init_stack();
        let (_book, chapter, scene, _outsider, book_one, _book_two) = seed_tree(&outline, binder);
        let stack = outline.stack();

        outline.apply_books_to_subtree(chapter, &[book_one]);
        assert_eq!(books_of(&outline, scene), vec![book_one]);

        undo_redo_commands::undo(&outline.app_ctx, stack).unwrap();
        assert!(
            books_of(&outline, scene).is_empty(),
            "one undo reverses the whole apply, not just the last descendant"
        );

        undo_redo_commands::redo(&outline.app_ctx, stack).unwrap();
        assert_eq!(
            books_of(&outline, scene),
            vec![book_one],
            "and redo puts it back"
        );
    }

    /// An empty list is a legitimate value to push, matching
    /// `apply_dict_language_to_subtree`'s own documented behaviour: it clears
    /// the subtree back to "not yet filed," the only way to undo an
    /// over-broad apply after the fact without visiting every child by hand.
    #[test]
    fn an_empty_list_clears_the_subtree() {
        let (outline, binder) = seed();
        let (_book, chapter, scene, _outsider, book_one, _book_two) = seed_tree(&outline, binder);
        outline.apply_books_to_subtree(chapter, &[book_one]);
        assert_eq!(books_of(&outline, scene), vec![book_one]);

        outline.apply_books_to_subtree(chapter, &[]);
        assert!(
            books_of(&outline, scene).is_empty(),
            "cleared back to not filed"
        );
    }

    /// A descendant that already carries its own filing loses it to the
    /// parent's current value: `apply` overwrites, it does not merge.
    #[test]
    fn overwrites_a_descendant_that_already_had_its_own_filing() {
        let (outline, binder) = seed();
        let (_book, chapter, scene, _outsider, book_one, book_two) = seed_tree(&outline, binder);
        outline.apply_books_to_subtree(chapter, &[book_one]);
        assert_eq!(books_of(&outline, scene), vec![book_one]);

        outline.apply_books_to_subtree(chapter, &[book_two]);
        assert_eq!(
            books_of(&outline, scene),
            vec![book_two],
            "the parent's current value replaces the child's own, it does not join it"
        );
    }

    /// A leaf has no subtree, so the button is never offered. The call is
    /// inert too, if it somehow fires anyway.
    #[test]
    fn a_leaf_has_no_descendants_and_the_call_is_a_noop() {
        let (outline, binder) = seed();
        let (_book, _chapter, scene, _outsider, book_one, _book_two) = seed_tree(&outline, binder);
        assert!(
            outline.subtree_descendants(scene).is_empty(),
            "the button's own gate"
        );
        outline.apply_books_to_subtree(scene, &[book_one]);
        assert!(
            books_of(&outline, scene).is_empty(),
            "an inert call writes nothing"
        );
    }
}
