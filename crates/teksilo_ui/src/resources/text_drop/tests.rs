// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use crate::models::ResourceKind;
use teksilo::core::widget_tree::WidgetTree;
use teksilo::widgets::rich_text::RichTextEditor;

fn row(id: u64, title: &str, file: &str) -> ResourceRow {
    ResourceRow {
        id,
        uid: uuid::Uuid::from_u128(u128::from(id)),
        title: title.into(),
        kind: ResourceKind::StoredFile,
        file_name: file.into(),
        ..Default::default()
    }
}

#[test]
fn one_picture_asks_and_everything_else_links() {
    let picture = row(1, "Lighthouse", "Lighthouse.jpg");
    assert_eq!(
        drop_action(std::slice::from_ref(&picture)),
        DropAction::AskPictureOrLink(picture.clone())
    );

    let map = row(2, "Village map", "Village map.pdf");
    assert_eq!(
        drop_action(std::slice::from_ref(&map)),
        DropAction::Links(vec![("Village map".into(), links::resource_href(map.uid))])
    );

    // Several rows at once, a picture among them: no question per row, all linked.
    let web = ResourceRow {
        kind: ResourceKind::WebLink,
        location: "https://example.org/tides".into(),
        ..row(3, "Tide tables", "")
    };
    assert_eq!(
        drop_action(&[picture.clone(), web.clone()]),
        DropAction::Links(vec![
            ("Lighthouse".into(), links::resource_href(picture.uid)),
            ("Tide tables".into(), "https://example.org/tides".into()),
        ])
    );

    // A missing picture cannot be put into the text, so it links.
    let gone = ResourceRow {
        missing: true,
        ..picture.clone()
    };
    assert!(matches!(
        drop_action(std::slice::from_ref(&gone)),
        DropAction::Links(_)
    ));
}

#[test]
fn a_drop_outside_the_lines_lands_at_the_end_of_the_text() {
    let doc = teksilo::text_document::TextDocument::new();
    doc.set_plain_text("The keeper climbed.").expect("text");
    let handle = RichTextEditor::editor(doc).handle();
    // Never laid out: the editor resolves no point, and the end is the answer.
    assert_eq!(
        drop_offset(&handle, Point::new(5.0, 5.0)),
        "The keeper climbed.".chars().count()
    );
}

/// A drop on a laid-out editor resolves the window point to the character under it,
/// not to the end: the point the target hands over is in its own space, the one the
/// editor wants is the window's, and the two agree here only because the host sits at
/// the window's origin.
#[test]
fn a_drop_on_the_first_line_lands_inside_it() {
    let doc = teksilo::text_document::TextDocument::new();
    doc.set_plain_text("The keeper climbed the stair.")
        .expect("text");
    let editor = RichTextEditor::editor(doc);
    let handle = editor.handle();
    // A text backend, so the editor lays its lines out and can hit-test a point.
    let mut tree = WidgetTree::new().with_text_backend(std::rc::Rc::new(std::cell::RefCell::new(
        teksilo::canvas::MockTextBackend::new(),
    )));
    tree.add(ResourceDropHost::new(
        Box::new(editor),
        Some((handle.downgrade(), FormatViewModel::detached())),
    ));
    for _ in 0..2 {
        tree.layout(SizeProposal::exact(600.0, 200.0));
        tree.render();
    }
    let at = drop_offset(&handle, Point::new(14.0, 14.0));
    assert!(
        at < "The keeper climbed the stair.".chars().count(),
        "landed at {at}, the end"
    );
}

/// The host lays its editor out at its own size, and keeps the drop menu out of the
/// layout until a drop asks for it.
#[test]
fn the_host_fills_its_bounds_and_holds_its_menu_dormant() {
    let doc = teksilo::text_document::TextDocument::new();
    doc.set_plain_text("Text").expect("text");
    let editor = RichTextEditor::editor(doc);
    let handle = editor.handle();
    let mut tree = WidgetTree::new();
    let host = tree.add(ResourceDropHost::new(
        Box::new(editor),
        Some((handle.downgrade(), FormatViewModel::detached())),
    ));
    tree.layout(SizeProposal::exact(400.0, 300.0));
    let b = tree.bounds(host);
    assert!(b.width > 0.0 && b.height > 0.0, "{b:?}");
    let children = tree.children(host);
    assert_eq!(children.len(), 2, "the drop target and its menu");
    let menu = tree.bounds(children[1]);
    assert_eq!(
        (menu.width, menu.height),
        (0.0, 0.0),
        "the menu takes no room"
    );
}

#[test]
fn a_picture_chosen_as_a_picture_is_parked_for_its_command() {
    let format = FormatViewModel::detached();
    assert!(format.take_dropped_resource().is_none());
    let doc = teksilo::text_document::TextDocument::new();
    let handle = RichTextEditor::editor(doc).handle();
    format.park_dropped_resource(DroppedResource {
        row: row(1, "Lighthouse", "Lighthouse.jpg"),
        handle,
        at: 3,
    });
    let taken = format.take_dropped_resource().expect("parked");
    assert_eq!((taken.row.id, taken.at), (1, 3));
    assert!(format.take_dropped_resource().is_none(), "taken once");
}
