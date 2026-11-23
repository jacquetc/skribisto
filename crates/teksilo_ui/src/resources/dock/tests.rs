// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use std::rc::Rc;

use super::*;
use crate::app_ids::AppIds;
use crate::models::ResourceKind;
use crate::resources::ResourcesSession;
use crate::resources::file_kind::FileKind;
use crate::resources::row::{detail_line, kind_of, row_name};

fn vm(app_ctx: &Rc<frontend::AppContext>, focus: Option<u64>) -> ResourcesViewModel {
    let ids = AppIds::new();
    ResourcesViewModel::new(
        app_ctx.clone(),
        ids.clone(),
        ResourcesSession::new(app_ctx.clone(), ids),
        Signal::new(focus),
        Signal::new(None),
        Signal::new(String::new()),
    )
}

/// `s` without the bidi isolation marks Fluent puts around each placeable.
fn plain(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, '\u{2068}' | '\u{2069}'))
        .collect()
}

fn stored(title: &str, file: &str, size: u64) -> ResourceRow {
    ResourceRow {
        id: 1,
        uid: uuid::Uuid::from_u128(1),
        title: title.into(),
        kind: ResourceKind::StoredFile,
        file_name: file.into(),
        byte_size: size,
        ..Default::default()
    }
}

/// The dock binds a model that subscribes to backend events, so a bare tree would
/// panic in `subscribe_event`: hence `tree_with_events`.
#[test]
fn the_panel_builds_and_lays_out_with_and_without_an_item_in_both_views() {
    for (focus, view) in [None, Some(10)]
        .into_iter()
        .flat_map(|f| DockView::ALL.map(|v| (f, v)))
    {
        let ctx = Rc::new(frontend::AppContext::new());
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let vm = vm(&ctx, focus);
        vm.view().set(view);
        let id = tree.add_boxed(Box::new(resources_panel(vm)));
        tree.layout(SizeProposal::exact(300.0, 600.0));
        let b = tree.bounds(id);
        assert!(
            b.width > 0.0 && b.height > 0.0,
            "laid out to nothing ({b:?})"
        );
    }
}

#[test]
fn the_panel_says_which_of_its_four_states_it_is_in() {
    let item = DockView::Item;
    assert_eq!(
        page(item, false, false),
        0,
        "no item in front of the writer"
    );
    assert_eq!(page(item, false, true), 0);
    assert_eq!(page(item, true, false), 1, "an item with nothing attached");
    assert_eq!(page(item, true, true), 2, "the list");
    let loose = DockView::Unattached;
    assert_eq!(page(loose, false, true), 2, "listed with no item open");
    assert_eq!(page(loose, true, true), 2);
    assert_eq!(page(loose, false, false), 3, "every resource is on an item");
    assert_eq!(page(loose, true, false), 3);
}

#[test]
fn a_row_builds_at_the_trailing_docks_width() {
    let ctx = Rc::new(frontend::AppContext::new());
    for row in [
        stored("Village map", "Village map.pdf", 2_621_440),
        ResourceRow {
            missing: true,
            kind: ResourceKind::LinkedFile,
            location: "/nowhere/Tide tables 1887.pdf".into(),
            path: Some("/nowhere/Tide tables 1887.pdf".into()),
            ..stored("Tide tables 1887", "Tide tables 1887.pdf", 0)
        },
        ResourceRow {
            kind: ResourceKind::WebLink,
            location: "https://example.org/a/very/long/address/that/has/to/elide".into(),
            ..stored("Harbour records", "", 0)
        },
    ] {
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let id = tree.add_boxed(Box::new(resource_row(
            &row,
            vm(&ctx, Some(10)),
            DetachFrom::Focus,
        )));
        tree.layout(SizeProposal::exact(260.0, 80.0));
        let b = tree.bounds(id);
        assert!(b.width > 0.0 && b.height > 0.0, "{}: {b:?}", row.title);
        assert!(
            b.width <= 260.5,
            "{} runs out of the dock: {b:?}",
            row.title
        );
    }
}

#[test]
fn the_second_line_is_the_size_or_the_address() {
    crate::test_support::with_real_messages(|| {
        let file = stored("Village map", "Village map.pdf", 2_621_440);
        assert_eq!(
            detail_line(&file),
            crate::shared::units::byte_size(2_621_440)
        );
        let empty = stored("Blank", "Blank.txt", 0);
        assert_eq!(
            detail_line(&empty),
            "Blank.txt",
            "no size to show: the name"
        );
        let web = ResourceRow {
            kind: ResourceKind::WebLink,
            location: "https://example.org/harbour".into(),
            ..stored("Harbour records", "", 0)
        };
        assert_eq!(detail_line(&web), "https://example.org/harbour");
        let linked = ResourceRow {
            kind: ResourceKind::LinkedFile,
            path: Some("/nowhere/x.pdf".into()),
            ..stored("X", "x.pdf", 0)
        };
        assert_eq!(plain(&detail_line(&linked)), "Linked file");
    });
}

/// A screen reader hears the type, the size and the missing state, not a colour.
#[test]
fn a_rows_name_carries_its_type_size_and_whether_it_is_missing() {
    crate::test_support::with_real_messages(|| {
        let file = stored("Village map", "Village map.pdf", 2_621_440);
        let name = plain(&row_name(&file));
        assert!(name.starts_with("Village map, PDF document, "), "{name}");
        assert!(!name.contains("not found"), "{name}");
        let missing = ResourceRow {
            missing: true,
            ..file
        };
        assert!(plain(&row_name(&missing)).ends_with(", not found"));
        let picture = stored("Lighthouse", "Lighthouse.jpg", 1024);
        assert!(plain(&row_name(&picture)).contains("picture"));
    });
}

#[test]
fn a_web_link_is_its_own_kind_whatever_its_address_ends_with() {
    let web = ResourceRow {
        kind: ResourceKind::WebLink,
        location: "https://example.org/map.pdf".into(),
        ..stored("Map page", "", 0)
    };
    assert_eq!(kind_of(&web), FileKind::WebLink);
    assert_eq!(kind_of(&stored("Map", "map.PDF", 1)), FileKind::Pdf);
}
