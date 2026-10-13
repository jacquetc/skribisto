// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

use frontend::commands::{asset_commands, binder_commands, binder_item_commands, work_commands};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use frontend::direct_access::CreateBinderItemDto;

use crate::test_support::RealProject;

fn png(w: u32, h: u32, shade: u8) -> Vec<u8> {
    let mut buf = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut buf, w, h);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        let mut wr = enc.write_header().unwrap();
        wr.write_image_data(&[shade, 20, 30, 255].repeat((w * h) as usize))
            .unwrap();
    }
    buf
}

fn picture(shade: u8) -> PendingImage {
    images::examine_bytes(png(6, 9, shade), format!("front{shade}.png")).expect("decodes")
}

/// A project, its open-documents store pointed at a real media directory, and that
/// directory (kept alive by the caller).
fn project() -> (RealProject, OpenDocsStore, tempfile::TempDir) {
    let p = RealProject::empty_novel();
    let media = tempfile::tempdir().expect("media dir");
    let docs = OpenDocsStore::new(p.app_ctx.clone());
    docs.set_media_dir(media.path().to_path_buf());
    (p, docs, media)
}

/// The Book rows of the project, in binder order.
fn books(p: &RealProject) -> Vec<u64> {
    crate::models::binder_stream::ordered_binder_items(&p.app_ctx, p.work_id)
        .into_iter()
        .filter(|row| {
            binder_item_commands::get_binder_item(&p.app_ctx, &row.id)
                .ok()
                .flatten()
                .is_some_and(|i| i.sub_role == BinderItemSubRole::Book)
        })
        .map(|row| row.id)
        .collect()
}

/// A second Book, appended to the project's first binder.
fn add_book(p: &RealProject) -> u64 {
    let binder = work_commands::get_work_relationship(
        &p.app_ctx,
        &p.work_id,
        &WorkRelationshipField::Binders,
    )
    .expect("binders")[0];
    let at = binder_commands::get_binder_relationship(
        &p.app_ctx,
        &binder,
        &BinderRelationshipField::BinderItems,
    )
    .expect("items")
    .len();
    binder_item_commands::create_binder_item(
        &p.app_ctx,
        p.ids.stack_id.get(),
        &CreateBinderItemDto {
            title: "Second Book".into(),
            role: BinderItemRole::Folder,
            sub_role: BinderItemSubRole::Book,
            activated: true,
            is_exportable: true,
            ..Default::default()
        },
        binder,
        at as i32,
    )
    .expect("create a second Book")
    .id
}

fn work_assets(p: &RealProject) -> Vec<u64> {
    work_commands::get_work_relationship(&p.app_ctx, &p.work_id, &WorkRelationshipField::Assets)
        .expect("assets")
}

fn cover_of(p: &RealProject, item: u64) -> Option<u64> {
    binder_item_commands::get_binder_item(&p.app_ctx, &item)
        .unwrap()
        .unwrap()
        .cover
}

/// Choosing a cover stores the picture, records it as one of the Work's, links the Book
/// to it, and shows it; one Ctrl+Z takes all of it back.
#[test]
fn choosing_a_cover_is_shown_and_one_undo_takes_it_back() {
    let (p, docs, media) = project();
    let book = books(&p)[0];
    let vm = BookCoverViewModel::new(p.app_ctx.clone(), p.ids.clone(), docs, book);
    assert_eq!(vm.state(), CoverState::None);

    let image = picture(10);
    vm.set_cover(&image).expect("set the cover");
    let assets = work_assets(&p);
    assert_eq!(assets.len(), 1, "the picture is one of the Work's");
    assert_eq!(cover_of(&p, book), Some(assets[0]));
    match vm.state() {
        CoverState::Shown { path, .. } => {
            assert!(path.starts_with(media.path()));
            assert!(path.is_file());
        }
        other => panic!("expected the cover shown, got {other:?}"),
    }

    frontend::commands::undo_redo_commands::undo(&p.app_ctx, p.ids.stack_id.get()).expect("undo");
    vm.sync();
    assert_eq!(cover_of(&p, book), None, "one undo clears the link");
    assert!(work_assets(&p).is_empty(), "and the row the choice created");
    assert_eq!(vm.state(), CoverState::None);
    // The bytes stay: content-addressed, and reclaiming storage is always explicit.
    assert!(
        media
            .path()
            .join(image.relative_path().trim_start_matches("assets/"))
            .is_file()
    );
}

/// Two Books, two covers, and neither touches the other. The bug the feature was rebuilt
/// for: a single project-wide cover meant setting one replaced the other.
#[test]
fn each_book_keeps_its_own_cover() {
    let (p, docs, _media) = project();
    let first = books(&p)[0];
    let second = add_book(&p);
    let a = BookCoverViewModel::new(p.app_ctx.clone(), p.ids.clone(), docs.clone(), first);
    let b = BookCoverViewModel::new(p.app_ctx.clone(), p.ids.clone(), docs, second);

    a.set_cover(&picture(10)).expect("first cover");
    b.set_cover(&picture(200)).expect("second cover");

    let (ca, cb) = (cover_of(&p, first), cover_of(&p, second));
    assert!(ca.is_some() && cb.is_some());
    assert_ne!(ca, cb, "each Book points at its own picture");
    assert_eq!(work_assets(&p).len(), 2);
}

/// The same picture chosen again, for this Book or another, is the same row: never a
/// second copy, and choosing a Book's current cover again records nothing.
#[test]
fn the_same_picture_is_reused_not_copied() {
    let (p, docs, _media) = project();
    let first = books(&p)[0];
    let second = add_book(&p);
    let a = BookCoverViewModel::new(p.app_ctx.clone(), p.ids.clone(), docs.clone(), first);
    let b = BookCoverViewModel::new(p.app_ctx.clone(), p.ids.clone(), docs, second);

    a.set_cover(&picture(10)).expect("first");
    let can_undo_before =
        frontend::commands::undo_redo_commands::can_undo(&p.app_ctx, p.ids.stack_id.get());
    a.set_cover(&picture(10)).expect("again");
    b.set_cover(&picture(10)).expect("the other Book");
    assert_eq!(work_assets(&p).len(), 1, "one picture, one row");
    assert_eq!(cover_of(&p, first), cover_of(&p, second));
    assert!(can_undo_before);
}

/// Removing a cover drops the link and keeps the picture in the project.
#[test]
fn removing_a_cover_keeps_the_picture() {
    let (p, docs, _media) = project();
    let book = books(&p)[0];
    let vm = BookCoverViewModel::new(p.app_ctx.clone(), p.ids.clone(), docs, book);
    vm.set_cover(&picture(10)).expect("set");
    let asset = cover_of(&p, book).expect("linked");

    vm.remove().expect("remove");
    assert_eq!(cover_of(&p, book), None);
    assert_eq!(vm.state(), CoverState::None);
    assert!(
        asset_commands::get_asset(&p.app_ctx, &asset)
            .unwrap()
            .is_some()
    );
    // And removing nothing is not an error.
    vm.remove().expect("remove again");
}

/// A cover the Book names whose file is gone reads as missing, not as no cover and not as
/// an empty picture.
#[test]
fn a_cover_whose_file_is_gone_reads_as_missing() {
    let (p, docs, media) = project();
    let book = books(&p)[0];
    let vm = BookCoverViewModel::new(p.app_ctx.clone(), p.ids.clone(), docs, book);
    let image = picture(10);
    vm.set_cover(&image).expect("set");
    std::fs::remove_file(
        media
            .path()
            .join(image.relative_path().trim_start_matches("assets/")),
    )
    .expect("delete the file");
    assert!(matches!(vm.state(), CoverState::Missing { .. }));
}

/// With no project there is nowhere to put the picture, and nothing is written.
#[test]
fn no_project_is_refused_before_anything_is_written() {
    let (p, _docs, _media) = project();
    let book = books(&p)[0];
    let unplaced = OpenDocsStore::new(p.app_ctx.clone());
    let vm = BookCoverViewModel::new(p.app_ctx.clone(), p.ids.clone(), unplaced, book);
    assert!(matches!(
        vm.set_cover(&picture(10)),
        Err(CoverError::NoProject)
    ));
    assert!(work_assets(&p).is_empty());
}

/// The view is told when the cover changes, and only then: every cover block hears every
/// undo in the project, and one that rebuilt on each would redraw for nothing.
#[test]
fn the_view_is_told_only_when_the_cover_changes() {
    let (p, docs, _media) = project();
    let book = books(&p)[0];
    let vm = BookCoverViewModel::new(p.app_ctx.clone(), p.ids.clone(), docs, book);
    let before = vm.revision().get();
    vm.sync();
    assert_eq!(
        vm.revision().get(),
        before,
        "nothing changed, nothing to redraw"
    );
    vm.set_cover(&picture(10)).expect("set");
    let after = vm.revision().get();
    assert_ne!(after, before, "a new cover is redrawn");
    vm.sync();
    assert_eq!(vm.revision().get(), after);
}
