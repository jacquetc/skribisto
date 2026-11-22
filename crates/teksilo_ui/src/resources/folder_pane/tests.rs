// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use std::rc::Rc;

use super::*;
use crate::resources::ResourcesViewModel;

fn body(vm: FolderResourcesViewModel, view: FolderView) -> FolderResourcesBody {
    FolderResourcesBody {
        vm,
        view: Signal::new(view.as_setting().to_string()),
        selection: SelectionModel::new(SelectionMode::Single),
        root: None,
    }
}

/// Lay `widget` out in a tab-sized box and say how big it came out.
fn laid_out(app_ctx: &Rc<frontend::AppContext>, widget: impl Widget + 'static) -> Rect {
    let mut tree = crate::test_support::tree_with_settings(app_ctx);
    let id = tree.add_boxed(Box::new(widget));
    tree.layout(SizeProposal::exact(900.0, 700.0));
    tree.bounds(id)
}

/// A folder with nothing filed in it says so, in either layout, and the page around it
/// (header and drop target) builds.
#[test]
fn an_empty_folder_says_so_in_both_layouts() {
    let app_ctx = Rc::new(frontend::AppContext::new());
    let ids = crate::app_ids::AppIds::new();
    let vm = FolderResourcesViewModel::new(
        app_ctx.clone(),
        ids.work_id.clone(),
        // An id no mock fixture and no real project knows.
        424_242,
        Signal::new("Empty".into()),
        ResourcesViewModel::detached(app_ctx.clone(), ids),
    );
    assert!(vm.entries().is_empty());
    for view in FolderView::ALL {
        let b = laid_out(&app_ctx, body(vm.clone(), view));
        assert!(b.width > 0.0 && b.height > 0.0, "{view:?}: {b:?}");
    }
    let b = laid_out(&app_ctx, FolderResourcesPane::new(vm));
    assert!(b.width > 0.0 && b.height > 0.0, "the page: {b:?}");
}

#[cfg(feature = "mocks")]
fn mock_book() -> (Rc<frontend::AppContext>, FolderResourcesViewModel) {
    let app_ctx = Rc::new(frontend::AppContext::new());
    let ids = crate::app_ids::AppIds::new();
    let vm = FolderResourcesViewModel::new(
        app_ctx.clone(),
        ids.work_id.clone(),
        101,
        Signal::new("Book One".into()),
        ResourcesViewModel::detached(app_ctx.clone(), ids),
    );
    (app_ctx, vm)
}

/// The list's rows are the dock's, each also naming the items it is on: what a screen
/// reader hears for the map names both the Book and the chapter inside it.
#[cfg(feature = "mocks")]
#[test]
fn the_list_names_every_item_a_resource_is_on() {
    let (app_ctx, vm) = mock_book();
    let mut tree = crate::test_support::tree_with_settings(&app_ctx);
    tree.add_boxed(Box::new(body(vm, FolderView::List)));
    tree.layout(SizeProposal::exact(900.0, 700.0));
    let names = crate::test_support::list_row_names(&mut tree);
    let map = names
        .iter()
        .find(|n| n.contains("Village map"))
        .unwrap_or_else(|| panic!("the map is listed: {names:?}"));
    assert!(
        map.contains("Book One") && map.contains("Chapter Two"),
        "{map}"
    );
}

/// The grid builds a tile per resource, a picture's laid out from its stored size and
/// everything else as its icon and title, at a tab's width.
#[cfg(feature = "mocks")]
#[test]
fn the_grid_lays_out_its_tiles_within_the_tab() {
    let (app_ctx, vm) = mock_book();
    assert!(
        vm.entries().len() >= 3,
        "the Book's subtree carries several"
    );
    let b = laid_out(&app_ctx, body(vm, FolderView::Grid));
    assert!(b.width > 0.0 && b.height > 0.0, "{b:?}");
    assert!(b.width <= 900.5, "the grid stays in the tab: {b:?}");
}

/// One tile of each kind, on its own, at a tile's width: a picture with a stored size, a
/// picture with none, a file, a missing linked file and a web link.
#[test]
fn every_kind_of_tile_builds_at_a_tiles_width() {
    use crate::models::ResourceKind;
    let app_ctx = Rc::new(frontend::AppContext::new());
    let picture = ResourceRow {
        id: 1,
        title: "Lighthouse".into(),
        kind: ResourceKind::StoredFile,
        file_name: "Lighthouse.jpg".into(),
        content_hash: "a".repeat(64),
        width: 1600,
        height: 900,
        path: Some("/nowhere/Lighthouse.jpg".into()),
        ..Default::default()
    };
    for row in [
        picture.clone(),
        ResourceRow {
            width: 0,
            height: 0,
            ..picture.clone()
        },
        ResourceRow {
            file_name: "Map.pdf".into(),
            ..picture.clone()
        },
        ResourceRow {
            kind: ResourceKind::LinkedFile,
            missing: true,
            ..picture.clone()
        },
        ResourceRow {
            kind: ResourceKind::WebLink,
            location: "https://example.org/a/long/address".into(),
            ..picture
        },
    ] {
        let mut tree = crate::test_support::tree_with_events(&app_ctx);
        let id = tree.add_boxed(Box::new(ResourceTile {
            row: row.clone(),
            rt: None,
            root: None,
        }));
        tree.layout(SizeProposal::with_width(TILE));
        let b = tree.bounds(id);
        assert!(b.height > 0.0, "{}: {b:?}", row.file_name);
        assert!(b.width <= TILE + 0.5, "{}: {b:?}", row.file_name);
    }
}
