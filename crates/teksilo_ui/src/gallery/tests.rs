// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Gallery against a real, in-memory project: which notes it finds, with which
//! pictures, in which order, under the Overview's own filters.
//!
//! Real backend only: under `mocks` the Overview's rows are fabricated for a Book and name
//! no store row, so there would be nothing for the Gallery to read.

use std::rc::Rc;

use frontend::AppContext;
use frontend::commands::{binder_commands, binder_item_commands, content_commands, work_commands};
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole, GoalUnit};
use frontend::direct_access::{
    CreateBinderDto, CreateBinderItemDto, CreateContentDto, CreateWorkDto,
};
use skribisto_model::counting::CountingMethodSetting;
use teksilo::prelude::Signal;

use super::tiles::{GalleryOptions, GalleryTile, build_tiles};
use super::*;
use crate::app_ids::AppIds;

const A: &str = "assets/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.png";
const B: &str = "assets/bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb.png";
const C: &str = "assets/cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc.jpg";

struct Fixture {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    binder: u64,
    folder: u64,
    next: std::cell::Cell<i32>,
}

impl Fixture {
    fn new() -> Self {
        let app_ctx = Rc::new(AppContext::new());
        let work = work_commands::create_orphan_work(&app_ctx, None, &CreateWorkDto::default())
            .unwrap_or_else(|e| panic!("work: {e}"));
        let binder = binder_commands::create_binder(
            &app_ctx,
            None,
            &CreateBinderDto {
                name: "Notes".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            0,
        )
        .unwrap_or_else(|e| panic!("binder: {e}"))
        .id;
        let ids = AppIds::new();
        ids.work_id.set(Some(work.id));
        let me = Self {
            app_ctx,
            ids,
            binder,
            folder: 0,
            next: std::cell::Cell::new(0),
        };
        let folder = me.item("Research", BinderItemRole::Folder, 0, true);
        Self { folder, ..me }
    }

    fn item(&self, title: &str, role: BinderItemRole, indent: i64, activated: bool) -> u64 {
        let index = self.next.get();
        self.next.set(index + 1);
        binder_item_commands::create_binder_item(
            &self.app_ctx,
            None,
            &CreateBinderItemDto {
                title: title.into(),
                role,
                sub_role: BinderItemSubRole::Note,
                activated,
                is_exportable: true,
                indent,
                ..Default::default()
            },
            self.binder,
            index,
        )
        .unwrap_or_else(|e| panic!("item: {e}"))
        .id
    }

    /// A note at `indent` whose own text is `djot`.
    fn note(&self, title: &str, indent: i64, djot: &str) -> u64 {
        let id = self.item(title, BinderItemRole::Item, indent, true);
        self.text(id, djot);
        id
    }

    fn text(&self, id: u64, djot: &str) {
        content_commands::create_content(
            &self.app_ctx,
            None,
            &CreateContentDto {
                role: ContentRole::NoteText,
                activated: true,
                data: djot.to_string(),
                ..Default::default()
            },
            id,
            -1,
        )
        .unwrap_or_else(|e| panic!("content: {e}"));
    }

    fn gallery(&self) -> GalleryViewModel {
        let overview = OverviewViewModel::new(
            self.app_ctx.clone(),
            self.ids.clone(),
            self.folder,
            &BinderItemRole::Folder,
            &BinderItemSubRole::Note,
            Signal::new(CountingMethodSetting::default()),
            crate::settings::TreeExpansionViewModel::new(
                self.app_ctx.clone(),
                self.ids.clone(),
                crate::models::TreeExpansionService::in_memory_default(),
            ),
            Signal::new(GoalUnit::default()),
        )
        .unwrap_or_else(|| panic!("a notes folder has an Overview"));
        GalleryViewModel::new(
            self.app_ctx.clone(),
            self.ids.work_id.clone(),
            overview,
            OpenDocsStore::new(self.app_ctx.clone()),
        )
    }
}

fn picture(src: &str) -> String {
    format!("![]({src}){{width=1600 height=900}}")
}

#[test]
fn the_gallery_reads_each_notes_own_pictures_in_binder_order() {
    let f = Fixture::new();
    f.note("Two shots", 1, &format!("{}\n\n{}", picture(A), picture(B)));
    f.note("Typed quote", 1, "Only words here.");
    f.item("Threads", BinderItemRole::Folder, 1, true);
    f.note("Deep shot", 2, &picture(C));
    let notes = f.gallery().notes();
    let shape: Vec<(String, usize, usize, bool)> = notes
        .iter()
        .map(|n| (n.title.clone(), n.depth, n.pictures.len(), n.is_folder))
        .collect();
    assert_eq!(
        shape,
        [
            ("Two shots".to_string(), 0, 2, false),
            ("Typed quote".to_string(), 0, 0, false),
            ("Threads".to_string(), 0, 0, true),
            ("Deep shot".to_string(), 1, 1, false),
        ]
    );
    assert_eq!(notes[0].pictures[1].image.src, B);
    assert_eq!(notes[1].excerpt, "Only words here.");

    let tiles = build_tiles(&notes, GalleryOptions::default());
    assert_eq!(
        tiles.len(),
        2,
        "one note tile and one folder tile: {tiles:?}"
    );
    match &tiles[1] {
        GalleryTile::Folder {
            previews, pictures, ..
        } => {
            assert_eq!(*pictures, 1);
            assert_eq!(previews[0].image.src, C);
        }
        other => panic!("a folder tile, got {other:?}"),
    }
}

#[test]
fn a_search_in_the_overview_narrows_the_gallery() {
    let f = Fixture::new();
    f.note("Gull on a post", 1, &picture(A));
    f.note("Lighthouse", 1, &picture(B));
    let gallery = f.gallery();
    gallery.overview().search_query_signal().set("gull".into());
    let titles: Vec<String> = gallery.notes().into_iter().map(|n| n.title).collect();
    assert_eq!(titles, ["Gull on a post"]);
}

#[test]
fn the_overviews_title_sort_orders_the_gallery() {
    let f = Fixture::new();
    f.note("Zebra", 1, &picture(A));
    f.note("Albatross", 1, &picture(B));
    let gallery = f.gallery();
    gallery.overview().sort_signal().set(Some((
        crate::models::COL_TITLE.to_string(),
        teksilo::data::SortDirection::Ascending,
    )));
    let titles: Vec<String> = gallery.notes().into_iter().map(|n| n.title).collect();
    assert_eq!(titles, ["Albatross", "Zebra"]);
}

#[test]
fn a_trashed_note_is_not_in_the_gallery() {
    let f = Fixture::new();
    f.note("Kept", 1, &picture(A));
    let gone = f.item("Thrown away", BinderItemRole::Item, 1, false);
    f.text(gone, &picture(B));
    let titles: Vec<String> = f.gallery().notes().into_iter().map(|n| n.title).collect();
    assert_eq!(titles, ["Kept"]);
}

#[test]
fn entering_a_subfolder_shows_what_is_inside_it() {
    let f = Fixture::new();
    f.note("Outside", 1, &picture(A));
    let threads = f.item("Threads", BinderItemRole::Folder, 1, true);
    f.note("Inside", 2, &picture(C));
    let gallery = f.gallery();
    gallery.enter(threads, "Threads".into());
    assert_eq!(gallery.current_folder(), threads);
    let notes = gallery.notes();
    assert_eq!(notes.len(), 1);
    assert_eq!((notes[0].title.as_str(), notes[0].depth), ("Inside", 0));
    gallery.leave_to(0);
    assert_eq!(gallery.current_folder(), f.folder);
}

#[test]
fn the_viewer_walks_the_picture_tiles_in_their_order() {
    let f = Fixture::new();
    f.note("Two shots", 1, &format!("{}\n\n{}", picture(A), picture(B)));
    f.note("Words", 1, "No picture.");
    f.note("One shot", 1, &picture(C));
    let gallery = f.gallery();
    let options = GalleryOptions {
        all_images: true,
        nested: true,
        text_notes: true,
    };
    let tiles = build_tiles(&gallery.notes(), options);
    let (entries, at) = gallery.viewer_entries(&tiles);
    let srcs: Vec<&str> = entries.iter().map(|e| e.src.as_str()).collect();
    assert_eq!(srcs, [A, B, C]);
    // The text tile opens no picture; the tiles after it still point at theirs.
    assert_eq!(at, [Some(0), Some(1), None, Some(2)]);
    assert_eq!(
        entries[2].item.as_ref().map(|i| i.title.as_str()),
        Some("One shot")
    );
}

// ── Picture resources ────────────────────────────────────────────────────────

/// The project's resources, with stored pictures (and one PDF) attached where asked.
struct Research {
    model: crate::models::ResourcesListModel,
    dir: tempfile::TempDir,
}

impl Research {
    fn new(f: &Fixture) -> Self {
        let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("tmp: {e}"));
        let model = crate::models::ResourcesListModel::new(f.app_ctx.clone(), f.ids.clone());
        model.set_media_root(dir.path().join("media"));
        Self { model, dir }
    }

    /// Link a real PNG called `name`, written beside the project, to `items`. Returns its
    /// path and the resource's id.
    fn link_picture(&self, items: &[u64], name: &str) -> (std::path::PathBuf, u64) {
        let path = self.dir.path().join(name);
        let icon = teksilo::canvas::RasterIcon::from_raw(vec![120; 8 * 6 * 4], 8, 6);
        let png = crate::shared::images::encode_png(&icon).unwrap_or_default();
        std::fs::write(&path, png).unwrap_or_else(|e| panic!("write: {e}"));
        let id = self
            .model
            .add(items, &Default::default(), std::slice::from_ref(&path), &[])
            .unwrap_or_else(|e| panic!("add: {e}"))[0];
        (path, id)
    }

    /// Attach a stored file called `name` to `items`.
    fn attach(&self, items: &[u64], name: &str) -> u64 {
        let picture = crate::resources::file_kind::is_viewable_picture(name);
        let stored = frontend::resource_management::StoredResourceFilesDto {
            uids: vec![uuid::Uuid::new_v4().hyphenated().to_string()],
            file_names: vec![name.to_string()],
            mime_types: vec![String::new()],
            byte_sizes: vec![1024],
            content_hashes: vec!["d".repeat(64)],
            widths: vec![if picture { 1600 } else { 0 }],
            heights: vec![if picture { 900 } else { 0 }],
            ..Default::default()
        };
        self.model
            .add(items, &stored, &[], &[])
            .unwrap_or_else(|e| panic!("add: {e}"))[0]
    }
}

impl Fixture {
    fn gallery_with(&self, research: &Research) -> GalleryViewModel {
        self.gallery()
            .with_resources(research.model.clone(), Signal::new("Research".into()))
    }
}

/// What each note holds, as `title: picture names`, a resource by its file name.
fn holdings(notes: &[GalleryNote]) -> Vec<String> {
    notes
        .iter()
        .map(|n| {
            let pictures: Vec<String> = n
                .pictures
                .iter()
                .map(|p| match &p.resource {
                    Some(r) => r.file_name.clone(),
                    None => p.image.src[7..9].to_string(),
                })
                .collect();
            format!("{}: {}", n.title, pictures.join(" "))
        })
        .collect()
}

#[test]
fn a_picture_resource_is_a_picture_of_its_note_after_those_of_its_text() {
    let f = Fixture::new();
    let shots = f.note("Two shots", 1, &format!("{}\n\n{}", picture(A), picture(B)));
    let quote = f.note("Typed quote", 1, "Only words here.");
    let threads = f.item("Threads", BinderItemRole::Folder, 1, true);
    f.note("Deep shot", 2, &picture(C));
    let research = Research::new(&f);
    research.attach(&[shots], "Quay.jpg");
    research.attach(&[quote], "Gull.png");
    research.attach(&[quote], "Notes.pdf");
    research.attach(&[threads], "Lamp.jpg");
    research.attach(&[f.folder], "Coast.jpg");
    let notes = f.gallery_with(&research).notes();
    assert_eq!(
        holdings(&notes),
        [
            // The folder's own, first: the folder itself, not a subfolder tile.
            "Research: Coast.jpg",
            "Two shots: aa bb Quay.jpg",
            // A PDF is a resource but not a picture.
            "Typed quote: Gull.png",
            "Threads: Lamp.jpg",
            "Deep shot: cc",
        ]
    );
    assert!(!notes[0].is_folder && notes[3].is_folder);

    // Every mode stays coherent: First image counts the resource; All images shows it.
    let first = build_tiles(&notes, GalleryOptions::default());
    let more: Vec<(u64, usize)> = first
        .iter()
        .filter_map(|t| match t {
            GalleryTile::Picture { item_id, more, .. } => Some((*item_id, *more)),
            _ => None,
        })
        .collect();
    assert_eq!(more, [(f.folder, 0), (shots, 2), (quote, 0)]);
    match &first[3] {
        GalleryTile::Folder {
            pictures, previews, ..
        } => {
            assert_eq!(
                *pictures, 2,
                "the subfolder's own resource and its note's picture"
            );
            assert!(previews[0].is_resource());
        }
        other => panic!("the subfolder's tile, got {other:?}"),
    }
    let all = build_tiles(
        &notes,
        GalleryOptions {
            all_images: true,
            nested: false,
            text_notes: false,
        },
    );
    assert_eq!(all.len(), 7, "coast, a, b, quay, gull, lamp, c: {all:?}");
    assert!(
        !tiles::has_text_only_notes(&notes),
        "a note whose picture is a resource is not a text note"
    );
}

#[test]
fn the_viewer_opens_a_resource_from_its_file_in_the_gallerys_order() {
    let f = Fixture::new();
    let shots = f.note("Two shots", 1, &picture(A));
    let research = Research::new(&f);
    let quay = research.attach(&[shots], "Quay.jpg");
    let gallery = f.gallery_with(&research);
    let options = GalleryOptions {
        all_images: true,
        ..GalleryOptions::default()
    };
    let tiles = build_tiles(&gallery.notes(), options);
    let (entries, at) = gallery.viewer_entries(&tiles);
    assert_eq!(at, [Some(0), Some(1)]);
    assert_eq!(entries[0].src, A);
    let row = research
        .model
        .by_id(quay)
        .unwrap_or_else(|| panic!("the resource"));
    assert_eq!(
        entries[1].src,
        crate::resources::links::resource_href(row.uid)
    );
    assert_eq!(entries[1].file_name.as_deref(), Some("Quay.jpg"));
    assert_eq!(
        entries[1].file.as_ref().map(|file| file.path.clone()),
        row.path,
        "the viewer reads the resource's own file"
    );
}

#[test]
fn a_filter_keeps_the_folders_own_resources_out_and_a_filtered_notes_with_it() {
    let f = Fixture::new();
    let gull = f.note("Gull on a post", 1, &picture(A));
    let lamp = f.note("Lighthouse", 1, &picture(B));
    let research = Research::new(&f);
    research.attach(&[f.folder], "Coast.jpg");
    research.attach(&[gull], "Gull.png");
    research.attach(&[lamp], "Lamp.jpg");
    let gallery = f.gallery_with(&research);
    gallery.overview().search_query_signal().set("gull".into());
    assert_eq!(holdings(&gallery.notes()), ["Gull on a post: aa Gull.png"]);
}

#[test]
fn a_gallery_with_no_resources_model_shows_the_pictures_of_the_text_only() {
    let f = Fixture::new();
    let shots = f.note("Two shots", 1, &picture(A));
    let research = Research::new(&f);
    research.attach(&[shots, f.folder], "Quay.jpg");
    assert_eq!(holdings(&f.gallery().notes()), ["Two shots: aa"]);
}

/// A resource reaches the Gallery, and so the viewer's "Open in another app", by the
/// file it opens: a link named like a picture but pointing at a launcher is not one. An
/// honest one is shown, and its tile has a picture to decode, though a linked file never
/// has a content hash recorded.
#[test]
fn a_link_named_like_a_picture_but_opening_something_else_is_not_shown() {
    use crate::models::{ResourceKind, ResourceRow};
    use crate::shared::picture_cache::{PictureCache, PictureKey, PictureState, file_key_hash};
    let disguised = ResourceRow {
        kind: ResourceKind::LinkedFile,
        file_name: "lighthouse.png".into(),
        path: Some(std::path::PathBuf::from("/home/someone/evil.desktop")),
        ..Default::default()
    };
    assert!(!super::is_picture_resource(&disguised));
    let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("tmp: {e}"));
    let path = dir.path().join("lighthouse.png");
    let icon = teksilo::canvas::RasterIcon::from_raw(vec![120; 4 * 4 * 4], 4, 4);
    std::fs::write(
        &path,
        crate::shared::images::encode_png(&icon).unwrap_or_default(),
    )
    .unwrap_or_else(|e| panic!("write: {e}"));
    let honest = ResourceRow {
        kind: ResourceKind::LinkedFile,
        file_name: "lighthouse.png".into(),
        path: Some(path.clone()),
        ..Default::default()
    };
    assert!(super::is_picture_resource(&honest));
    let hash = file_key_hash(&path, &honest.content_hash)
        .unwrap_or_else(|| panic!("a linked picture has a key"));
    let slot = PictureCache::new(1 << 20, None).request(None, path, PictureKey { hash, edge: 64 });
    assert!(matches!(slot.get(), PictureState::Ready(_)), "it decodes");
}

/// A picture linked rather than stored, as a Scrivener bookmark to a local image is too,
/// has no content hash. The Gallery still decodes its tile, and the viewer opened on it
/// still has a picture to show.
#[test]
fn a_linked_picture_has_a_tile_and_opens_in_the_viewer() {
    use crate::shared::picture_cache::{PictureCache, PictureState};
    let f = Fixture::new();
    let shots = f.note("Harbour", 1, "Words only.");
    let research = Research::new(&f);
    let (path, id) = research.link_picture(&[shots], "Quay.png");
    let row = research
        .model
        .by_id(id)
        .unwrap_or_else(|| panic!("the resource"));
    assert!(row.content_hash.is_empty(), "a linked file is never hashed");
    let gallery = f.gallery_with(&research);
    let tiles = build_tiles(&gallery.notes(), GalleryOptions::default());
    let shown = tiles
        .iter()
        .find_map(GalleryTile::resource)
        .unwrap_or_else(|| panic!("the linked picture has a tile: {tiles:?}"));
    assert_eq!(shown.path.as_deref(), Some(path.as_path()));
    let (entries, at) = gallery.viewer_entries(&tiles);
    let viewer = crate::picture_viewer::PictureViewerViewModel::new(
        entries,
        at.iter().flatten().copied().next().unwrap_or_default(),
        std::path::PathBuf::new(),
    );
    assert_eq!(viewer.current_path(), Some(path.clone()));
    let key = viewer
        .current_key(64)
        .unwrap_or_else(|| panic!("the viewer has a key for it"));
    let slot = PictureCache::new(1 << 20, None).request(None, path, key);
    assert!(
        matches!(slot.get(), PictureState::Ready(_)),
        "and it decodes"
    );
}

/// An item listing one resource twice, apart (`[A, B, A]`), shows it once.
#[test]
fn a_resource_listed_twice_on_an_item_is_one_picture() {
    assert_eq!(super::once_each(&[7, 3, 7, 9, 3]), [7, 3, 9]);
    assert!(super::once_each(&[]).is_empty());
}
