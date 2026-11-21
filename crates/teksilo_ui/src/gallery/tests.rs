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
    assert_eq!(notes[0].pictures[1].src, B);
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
            assert_eq!(previews[0].src, C);
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
