// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use crate::models::ResourceKind;

fn row(id: u64, title: &str) -> ResourceRow {
    ResourceRow {
        id,
        uid: uuid::Uuid::from_u128(u128::from(id)),
        title: title.into(),
        kind: ResourceKind::StoredFile,
        file_name: format!("{title}.pdf"),
        ..Default::default()
    }
}

fn item(id: u64, title: &str) -> AttachedItem {
    AttachedItem {
        id,
        title: title.into(),
    }
}

fn names(entries: &[FolderResource]) -> Vec<(String, Vec<u64>)> {
    entries
        .iter()
        .map(|e| (e.row.title.clone(), e.items.iter().map(|i| i.id).collect()))
        .collect()
}

#[test]
fn a_resource_on_two_items_is_listed_once_naming_both_in_binder_order() {
    let items = [item(1, "Part One"), item(2, "Chapter"), item(3, "Scene")];
    let attachments = HashMap::from([(1, vec![10]), (2, vec![11, 10]), (3, vec![12, 10, 10])]);
    let rows = vec![row(10, "Village map"), row(11, "Harbour"), row(12, "Gulls")];
    let entries = collect_in_subtree(&items, &attachments, rows);
    assert_eq!(
        names(&entries),
        [
            ("Gulls".to_string(), vec![3]),
            ("Harbour".to_string(), vec![2]),
            // The folder first, then its subtree in binder order; an item listing the
            // resource twice is still named once.
            ("Village map".to_string(), vec![1, 2, 3]),
        ]
    );
}

#[test]
fn a_resource_on_no_listed_item_is_not_on_the_page() {
    let items = [item(1, "Folder")];
    let attachments = HashMap::from([(1, vec![10]), (99, vec![11])]);
    let rows = vec![row(10, "Kept"), row(11, "Elsewhere")];
    let entries = collect_in_subtree(&items, &attachments, rows);
    assert_eq!(names(&entries), [("Kept".to_string(), vec![1])]);
}

#[test]
fn the_page_sorts_by_title_as_a_reader_looks_one_up() {
    let items = [item(1, "Folder")];
    let attachments = HashMap::from([(1, vec![1, 2, 3, 4])]);
    let mut untitled = row(4, "");
    untitled.file_name = "b-scan.png".into();
    let rows = vec![row(1, "Zebra"), row(2, "éclair"), row(3, "apple"), untitled];
    let entries = collect_in_subtree(&items, &attachments, rows);
    let titles: Vec<String> = entries.iter().map(|e| title_of(&e.row)).collect();
    assert_eq!(titles, ["apple", "b-scan.png", "éclair", "Zebra"]);
}

#[test]
fn the_layout_setting_reads_back_what_it_wrote_and_defaults_to_the_list() {
    for view in FolderView::ALL {
        assert_eq!(FolderView::from_setting(view.as_setting()), view);
        assert_eq!(FolderView::from_index(view.index()), view);
    }
    assert_eq!(
        FolderView::from_setting(crate::RESOURCES_FOLDER_VIEW_DEFAULT),
        FolderView::List
    );
    assert_eq!(FolderView::from_setting("mosaic"), FolderView::List);
    assert_eq!(FolderView::from_index(7), FolderView::List);
}

/// The mock project: the map is on the Book and on Chapter Two, inside it, so Book One's
/// page lists it once, naming both; Part One's names Chapter Two only.
#[cfg(feature = "mocks")]
#[test]
fn the_mock_books_page_lists_the_map_once_with_both_items() {
    use std::rc::Rc;
    let app_ctx = Rc::new(frontend::AppContext::new());
    let ids = crate::app_ids::AppIds::new();
    let resources = ResourcesViewModel::detached(app_ctx.clone(), ids.clone());
    let book = FolderResourcesViewModel::new(
        app_ctx.clone(),
        ids.work_id.clone(),
        101,
        Signal::new("Book One".into()),
        resources.clone(),
    );
    let entries = book.entries();
    let map = entries
        .iter()
        .find(|e| e.row.title == "Village map")
        .unwrap_or_else(|| panic!("the map is on the Book's page: {entries:?}"));
    let on: Vec<&str> = map.items.iter().map(|i| i.title.as_str()).collect();
    assert_eq!(on, ["Book One", "Chapter Two"]);
    assert!(
        !entries.iter().any(|e| e.row.title == "Keeper's cottage"),
        "a resource on no item is on no folder's page"
    );
}

#[cfg(not(feature = "mocks"))]
mod real {
    use std::path::PathBuf;
    use std::rc::Rc;

    use frontend::commands::binder_item_commands;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    use super::*;
    use crate::models::WebLink;
    use crate::resources::ResourcesSession;
    use crate::test_support::RealProject;

    /// The empty novel's Book, its chapter folder and its scene, and one more scene.
    struct Fixture {
        project: RealProject,
        resources: ResourcesViewModel,
        book: u64,
        chapter: u64,
        scene: u64,
        second: u64,
    }

    fn fixture() -> Fixture {
        let project = RealProject::empty_novel();
        let (scene, _) = project.scenes()[0];
        let (second, _) = project.add_scene_after(scene, "The quay");
        let items =
            crate::models::binder_stream::ordered_flat_items(&project.app_ctx, project.work_id);
        let of = |sub: BinderItemSubRole| {
            items
                .iter()
                .find(|(_, it)| it.role == BinderItemRole::Folder && it.sub_role == sub)
                .map(|(_, it)| it.id)
                .unwrap_or_else(|| panic!("the novel has a {sub:?} folder"))
        };
        let (book, chapter) = (
            of(BinderItemSubRole::Book),
            of(BinderItemSubRole::ChapterScene),
        );
        let session = ResourcesSession::new(project.app_ctx.clone(), project.ids.clone());
        let resources = ResourcesViewModel::new(
            project.app_ctx.clone(),
            project.ids.clone(),
            session,
            Signal::new(None),
            Signal::new(None),
            Signal::new(String::new()),
        );
        Fixture {
            project,
            resources,
            book,
            chapter,
            scene,
            second,
        }
    }

    fn link(fx: &Fixture, items: &[u64], title: &str) -> u64 {
        fx.resources
            .session()
            .model()
            .add(
                items,
                &Default::default(),
                &[] as &[PathBuf],
                &[WebLink {
                    url: format!("https://example.org/{}", title.to_lowercase()),
                    title: title.into(),
                }],
            )
            .unwrap_or_else(|e| panic!("add: {e}"))[0]
    }

    fn page(fx: &Fixture, folder: u64, title: &str) -> FolderResourcesViewModel {
        FolderResourcesViewModel::new(
            fx.project.app_ctx.clone(),
            fx.project.ids.work_id.clone(),
            folder,
            Signal::new(title.into()),
            fx.resources.clone(),
        )
    }

    #[test]
    fn the_folders_page_lists_its_own_and_its_subtrees_once_each_trashed_left_out() {
        let fx = fixture();
        link(&fx, &[fx.book, fx.scene], "Map");
        link(&fx, &[fx.chapter], "Tides");
        let gone = link(&fx, &[fx.second], "Gulls");
        // The second scene goes to the trash: its resource leaves the page with it.
        let mut dto = binder_item_commands::get_binder_item(&fx.project.app_ctx, &fx.second)
            .ok()
            .flatten()
            .unwrap_or_else(|| panic!("the second scene"));
        dto.activated = false;
        binder_item_commands::update_binder_item(
            &fx.project.app_ctx,
            None,
            &frontend::direct_access::UpdateBinderItemDto::from(dto),
        )
        .unwrap_or_else(|e| panic!("trash: {e}"));

        let entries = page(&fx, fx.book, "Book").entries();
        let shape: Vec<(String, Vec<u64>)> = entries
            .iter()
            .map(|e| (e.row.title.clone(), e.items.iter().map(|i| i.id).collect()))
            .collect();
        assert_eq!(
            shape,
            [
                ("Map".to_string(), vec![fx.book, fx.scene]),
                ("Tides".to_string(), vec![fx.chapter]),
            ]
        );
        assert!(!entries.iter().any(|e| e.row.id == gone));
        assert_eq!(
            entries[0].items[0].title, "Book",
            "the folder by its tab's title"
        );

        // The chapter's page: the map through its scene, not through the Book above it.
        let chapter = page(&fx, fx.chapter, "Chapter").entries();
        let shape: Vec<(String, Vec<u64>)> = chapter
            .iter()
            .map(|e| (e.row.title.clone(), e.items.iter().map(|i| i.id).collect()))
            .collect();
        assert_eq!(
            shape,
            [
                ("Map".to_string(), vec![fx.scene]),
                ("Tides".to_string(), vec![fx.chapter]),
            ]
        );
    }

    #[test]
    fn a_link_added_from_the_page_is_attached_to_the_folder() {
        let fx = fixture();
        let page = page(&fx, fx.chapter, "Chapter");
        let mut tree = crate::test_support::tree_with_events(&fx.project.app_ctx);
        let added = Rc::new(std::cell::Cell::new(false));
        {
            let (page, added) = (page.clone(), added.clone());
            crate::test_support::press(&mut tree, move |c| {
                added.set(page.add_dropped_text(c, "see https://example.org/harbour please"));
            });
        }
        assert!(added.get());
        let rows = fx.resources.session().model().rows_for(fx.chapter);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].location, "https://example.org/harbour");
        assert!(fx.resources.session().model().rows_for(fx.scene).is_empty());
    }
}
