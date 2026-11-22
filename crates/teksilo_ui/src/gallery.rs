// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `GalleryViewModel`: the logic behind a notes folder's **Gallery**, the pictures of its
//! notes as a grid of thumbnails.
//!
//! ## The same notes as the Overview, never a second opinion
//!
//! The Gallery has no search, filter or sort of its own. It reads the folder's rows
//! through [`shaped_subtree`], the function the Overview's own rows go through, with the
//! Overview's own filter signals ([`OverviewViewModel::filters`]). So a search typed in
//! one view holds in the other, the two always agree about which notes match, and a rule
//! added to the Overview's search reaches the Gallery for free.
//!
//! ## Where a note's pictures come from
//!
//! From its own text (`NoteText`), not its synopsis: a research screenshot is the note.
//! The stored Djot is read in one batch for the whole folder, no editor built. A note
//! open in an editor is read from its live document instead, because the store lags the
//! editor until the next save and autosave can be off: a picture pasted a minute ago
//! must not be missing from the Gallery until the writer remembers to press Ctrl+S.
//! The pane rebuilds when such a note's list of pictures changes, and only then: typing
//! words beside a picture costs a comparison, not a reload.
//!
//! Plain Rust over the backend; the widgets are in [`crate::tabs::gallery`].

pub mod tiles;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use teksilo::core::ObserverHandle;
use teksilo::prelude::Signal;

use frontend::AppContext;
use frontend::commands::{binder_item_commands, content_commands};
use frontend::common::entities::{BinderItemRole, ContentRole};

use crate::models::OpenDocsStore;
use crate::models::shaped_subtree;
use crate::overview::OverviewViewModel;
use crate::picture_viewer::{ViewerEntry, ViewerItem};
use crate::shared::images::{ImageRef, parse_images};
pub use tiles::{GalleryNote, GalleryOptions, GalleryTile};

struct Inner {
    app_ctx: Rc<AppContext>,
    work_id: Signal<Option<u64>>,
    /// The folder whose tab this is.
    container_id: u64,
    overview: OverviewViewModel,
    docs: OpenDocsStore,
    /// The subfolders entered in Nested, outermost first, with their titles for the
    /// breadcrumb. Empty: the tab's own folder.
    trail: Signal<Vec<(u64, String)>>,
    /// Bumped when an open note's pictures change under the store.
    revision: Signal<u64>,
    /// The pictures each open note had when last looked at, and the observers on those
    /// notes' edits.
    live: RefCell<HashMap<u64, Vec<String>>>,
    observers: RefCell<Vec<ObserverHandle>>,
}

/// The Gallery of one notes folder tab. Cheap to clone; clones share their state.
#[derive(Clone)]
pub struct GalleryViewModel {
    inner: Rc<Inner>,
}

impl std::fmt::Debug for GalleryViewModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GalleryViewModel")
            .field("container_id", &self.inner.container_id)
            .field("trail", &self.inner.trail.get())
            .finish()
    }
}

impl GalleryViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        work_id: Signal<Option<u64>>,
        overview: OverviewViewModel,
        docs: OpenDocsStore,
    ) -> Self {
        let container_id = overview.container_id();
        Self {
            inner: Rc::new(Inner {
                app_ctx,
                work_id,
                container_id,
                overview,
                docs,
                trail: Signal::new(Vec::new()),
                revision: Signal::new(0),
                live: RefCell::new(HashMap::new()),
                observers: RefCell::new(Vec::new()),
            }),
        }
    }

    pub fn app_ctx(&self) -> Rc<AppContext> {
        self.inner.app_ctx.clone()
    }

    pub fn overview(&self) -> &OverviewViewModel {
        &self.inner.overview
    }

    pub fn trail(&self) -> Signal<Vec<(u64, String)>> {
        self.inner.trail.clone()
    }

    pub fn revision(&self) -> Signal<u64> {
        self.inner.revision.clone()
    }

    pub fn work_id(&self) -> Signal<Option<u64>> {
        self.inner.work_id.clone()
    }

    pub fn media_dir(&self) -> std::path::PathBuf {
        self.inner.docs.media_dir()
    }

    /// The folder shown now: the last one entered, else the tab's own.
    pub fn current_folder(&self) -> u64 {
        self.inner
            .trail
            .get()
            .last()
            .map_or(self.inner.container_id, |(id, _)| *id)
    }

    /// Enter the subfolder `item_id` (a folder tile was activated in Nested).
    pub fn enter(&self, item_id: u64, title: String) {
        let mut trail = self.inner.trail.get();
        trail.push((item_id, title));
        self.inner.trail.set(trail);
    }

    /// Go back to the folder at `depth` in the breadcrumb: `0` is the tab's own.
    pub fn leave_to(&self, depth: usize) {
        let mut trail = self.inner.trail.get();
        if depth < trail.len() {
            trail.truncate(depth);
            self.inner.trail.set(trail);
        }
    }

    /// The notes of the folder shown now, as the Overview shows them: its search, filters
    /// and order, each note with its pictures and its opening words.
    ///
    /// Also leaves an observer on every note in it that is open in an editor, so that a
    /// picture added or removed there before any save still reaches the Gallery.
    pub fn notes(&self) -> Vec<GalleryNote> {
        let folder = self.current_folder();
        let rows = shaped_subtree(
            &self.inner.app_ctx,
            &self.inner.work_id,
            folder,
            &self.inner.overview.filters(),
        );
        if rows.is_empty() {
            self.watch_open_notes(&[]);
            return Vec::new();
        }
        let ids: Vec<u64> = rows.iter().map(|r| r.item.item_id).collect();
        let texts = self.note_texts(&ids);
        // The shaped rows' depths are relative to the folder whose subtree was loaded.
        let base = rows.iter().map(|r| r.depth).min().unwrap_or(0);
        let notes: Vec<GalleryNote> = rows
            .into_iter()
            .map(|row| {
                let item = row.item;
                let text = texts.get(&item.item_id).cloned().unwrap_or_default();
                let pictures = parse_images(&text);
                let plain = skrib_format::djot_plain_text(&text)
                    .map(|(plain, _)| plain)
                    .unwrap_or_default();
                GalleryNote {
                    item_id: item.item_id,
                    title: item.title,
                    depth: row.depth.saturating_sub(base),
                    is_folder: item.role == BinderItemRole::Folder,
                    pictures,
                    excerpt: tiles::excerpt(&plain),
                }
            })
            .collect();
        self.watch_open_notes(&notes);
        notes
    }

    /// Each note's own text as Djot: the live document for a note open in an editor,
    /// the store's copy for the rest, read in one batch.
    fn note_texts(&self, ids: &[u64]) -> HashMap<u64, String> {
        let mut texts = HashMap::new();
        let mut stored = Vec::new();
        for &id in ids {
            match self.live_text(id) {
                Some(djot) => {
                    texts.insert(id, djot);
                }
                None => stored.push(id),
            }
        }
        let items = binder_item_commands::get_binder_item_multi(&self.inner.app_ctx, &stored)
            .unwrap_or_default();
        let mut owner: HashMap<u64, u64> = HashMap::new();
        for item in items.into_iter().flatten() {
            for content in item.contents {
                owner.insert(content, item.id);
            }
        }
        let content_ids: Vec<u64> = owner.keys().copied().collect();
        let contents = content_commands::get_content_multi(&self.inner.app_ctx, &content_ids)
            .unwrap_or_default();
        for content in contents.into_iter().flatten() {
            if content.role != ContentRole::NoteText {
                continue;
            }
            if let Some(item) = owner.get(&content.id) {
                texts.insert(*item, content.data);
            }
        }
        texts
    }

    /// The live Djot of `item_id`'s own text, when it is open in an editor.
    fn live_text(&self, item_id: u64) -> Option<String> {
        let doc = self.inner.docs.peek(item_id)?;
        doc.main.as_ref()?.doc.to_djot().ok()
    }

    /// Watch the open notes among `notes`, replacing the previous watch.
    fn watch_open_notes(&self, notes: &[GalleryNote]) {
        let mut observers = Vec::new();
        let mut live = HashMap::new();
        for note in notes.iter().filter(|n| !n.is_folder) {
            let Some(doc) = self.inner.docs.peek(note.item_id) else {
                continue;
            };
            live.insert(note.item_id, srcs(&note.pictures));
            let me = self.clone();
            let item_id = note.item_id;
            observers.push(doc.edit_gen.observe(move |_| me.note_edited(item_id)));
        }
        *self.inner.live.borrow_mut() = live;
        *self.inner.observers.borrow_mut() = observers;
    }

    /// An open note was edited: ask for a rebuild only if its pictures changed.
    fn note_edited(&self, item_id: u64) {
        let now = self
            .live_text(item_id)
            .map(|djot| srcs(&parse_images(&djot)))
            .unwrap_or_default();
        let changed = self
            .inner
            .live
            .borrow()
            .get(&item_id)
            .is_none_or(|before| *before != now);
        if changed {
            self.inner.live.borrow_mut().insert(item_id, now);
            self.inner
                .revision
                .set(self.inner.revision.get().wrapping_add(1));
        }
    }

    /// The viewer's list for `tiles`: every picture tile, in order, and for each tile the
    /// entry it opens on (a text or folder tile opens none).
    pub fn viewer_entries(&self, tiles: &[GalleryTile]) -> (Vec<ViewerEntry>, Vec<Option<usize>>) {
        let mut entries = Vec::new();
        let mut at = Vec::with_capacity(tiles.len());
        for tile in tiles {
            match tile {
                GalleryTile::Picture {
                    item_id,
                    title,
                    image,
                    ..
                } => {
                    at.push(Some(entries.len()));
                    entries.push(ViewerEntry {
                        src: image.src.clone(),
                        alt: image.alt.clone(),
                        item: Some(ViewerItem {
                            item_id: *item_id,
                            title: title.clone(),
                        }),
                        file_name: None,
                        file: None,
                    });
                }
                _ => at.push(None),
            }
        }
        (
            crate::picture_viewer::with_file_names(&self.inner.app_ctx, entries),
            at,
        )
    }
}

fn srcs(pictures: &[ImageRef]) -> Vec<String> {
    pictures.iter().map(|p| p.src.clone()).collect()
}

#[cfg(all(test, not(feature = "mocks")))]
mod tests;
