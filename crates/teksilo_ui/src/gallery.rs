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
//! ## Picture resources
//!
//! A photograph attached to a note as a resource, not put into its text, is a picture of
//! that note too, after those of its text ([`tiles`] says how each mode shows it). A
//! folder's own picture resources are its pictures; those of the folder the Gallery shows
//! come first. They open in the image viewer with the others, in the Gallery's order, and
//! the viewer's Open in another app goes through the Resources feature for them
//! ([`crate::resources::ResourcesViewModel::viewer_opener`]).
//!
//! ## A search finds a picture resource by its name
//!
//! The Overview's search matches a row's title and label, the names its table shows. The
//! Gallery shows more: each picture resource, named on its tile. So here a search also
//! finds a picture resource by its title, its file name or its description, the way the
//! Resources dock names it. A note the search does not match, but which carries such a
//! picture, is shown holding that picture only (its other pictures are not what was
//! asked for), with the folders around it kept as the Overview keeps them; the folder's
//! own picture resources stay when they match. The tag and status filters are still the
//! Overview's alone: they ask about notes, a picture has neither, and the folder's own
//! pictures leave while one narrows the notes.
//!
//! ## An empty Gallery says why
//!
//! A Gallery with no tile tells a folder with no pictures from one whose pictures the
//! filters hide ([`GalleryEmpty`]). Filtered, it says no picture matches and offers to
//! clear the filters, the Overview's with them. Otherwise it offers to add pictures to
//! the folder shown, as resources, from a file dialog or by a drop: the folder's own
//! pictures are tiles, so they appear at once.
//!
//! Plain Rust over the backend; the widgets are in [`crate::tabs::gallery`].

pub mod tiles;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use teksilo::core::ObserverHandle;
use teksilo::prelude::Signal;

use frontend::AppContext;
use frontend::commands::{binder_item_commands, content_commands};
use frontend::common::entities::{BinderItemRole, ContentRole};

use crate::models::{OpenDocsStore, ResourceRow, ResourcesListModel};
use crate::models::{row_matches, search_needle, shaped_subtree};
use crate::overview::OverviewViewModel;
use crate::picture_viewer::{ViewerEntry, ViewerFile, ViewerItem};
use crate::shared::images::{ImageRef, parse_images};
pub use tiles::{GalleryNote, GalleryOptions, GalleryPicture, GalleryTile};

/// Why a Gallery shows no tile, from [`GalleryViewModel::empty_reason`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GalleryEmpty {
    /// The search, a tag or a status hides every picture. Saying "no images here" would
    /// make a writer think their photographs were lost: the page says the filters match
    /// none, and offers to clear them.
    NoMatch,
    /// The folder's notes hold only text, and *Notes without images* is off.
    TextNotesHidden,
    /// Nothing in the folder to show: the page offers to add pictures to it.
    NoPictures,
}

impl GalleryEmpty {
    /// Whether the page offers to add pictures to the folder, by a button or a drop. Not
    /// while filtered: the folder may well have pictures, and one added there might not
    /// match the filters either.
    pub fn offers_adding(self) -> bool {
        !matches!(self, Self::NoMatch)
    }
}

/// The files among `paths` the Gallery can show, the others left out: a file dropped on
/// an empty Gallery is attached to its folder only when it would then appear there.
pub fn pictures_among(paths: Vec<std::path::PathBuf>) -> Vec<std::path::PathBuf> {
    paths
        .into_iter()
        .filter(|p| {
            p.file_name().is_some_and(|name| {
                crate::resources::file_kind::is_viewable_picture(&name.to_string_lossy())
            })
        })
        .collect()
}

/// A row of a folder's subtree, as the Overview shapes it.
type OverviewTreeRow = teksilo::data::TreeRow<uuid::Uuid, crate::models::OverviewRow>;

struct Inner {
    app_ctx: Rc<AppContext>,
    work_id: Signal<Option<u64>>,
    /// The folder whose tab this is.
    container_id: u64,
    /// The project's resources, and the title of the tab's own folder, which a picture
    /// resource on it is filed under. `None` until [`GalleryViewModel::with_resources`]:
    /// a Gallery with no resources model shows the pictures of the text only.
    resources: Option<(ResourcesListModel, Signal<String>)>,
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
                resources: None,
                overview,
                docs,
                trail: Signal::new(Vec::new()),
                revision: Signal::new(0),
                live: RefCell::new(HashMap::new()),
                observers: RefCell::new(Vec::new()),
            }),
        }
    }

    /// The same Gallery, showing the picture resources of `resources` beside the pictures
    /// of the text; `container_title` names the tab's own folder. Call before cloning.
    pub fn with_resources(
        mut self,
        resources: ResourcesListModel,
        container_title: Signal<String>,
    ) -> Self {
        if let Some(inner) = Rc::get_mut(&mut self.inner) {
            inner.resources = Some((resources, container_title));
        }
        self
    }

    /// Bumps when a resource or an attachment may have changed: `None` with no resources.
    pub fn resources_version(&self) -> Option<Signal<u64>> {
        self.inner
            .resources
            .as_ref()
            .map(|(model, _)| model.version_signal())
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

    /// Whether the search, a tag or a status is narrowing the folder now. The order is not
    /// a filter: it hides nothing.
    pub fn is_filtered(&self) -> bool {
        let filters = self.inner.overview.filters();
        search_needle(&filters.query.get()).is_some()
            || !filters.tag_filter.get().is_empty()
            || !filters.status_filter.get().is_empty()
    }

    /// Drop the search and the tag and status filters, in the Overview too, since they are
    /// its own. The order stays.
    pub fn clear_filters(&self) {
        let filters = self.inner.overview.filters();
        if !filters.query.get().is_empty() {
            filters.query.set(String::new());
        }
        if !filters.tag_filter.get().is_empty() {
            filters.tag_filter.set(Vec::new());
        }
        if !filters.status_filter.get().is_empty() {
            filters.status_filter.set(Vec::new());
        }
    }

    /// Why the Gallery has no tile for `notes` (what [`Self::notes`] gave) under
    /// `options`: what its empty page says, and what it offers.
    pub fn empty_reason(&self, notes: &[GalleryNote], options: GalleryOptions) -> GalleryEmpty {
        if self.is_filtered() {
            GalleryEmpty::NoMatch
        } else if !options.text_notes && tiles::has_text_only_notes(notes) {
            GalleryEmpty::TextNotesHidden
        } else {
            GalleryEmpty::NoPictures
        }
    }

    /// The notes of the folder shown now, as the Overview shows them: its search, filters
    /// and order, each note with its pictures and its opening words.
    ///
    /// Also leaves an observer on every note in it that is open in an editor, so that a
    /// picture added or removed there before any save still reaches the Gallery.
    pub fn notes(&self) -> Vec<GalleryNote> {
        let folder = self.current_folder();
        let filters = self.inner.overview.filters();
        let needle = search_needle(&filters.query.get());
        // A tag or status filter asks about notes, which the folder is not.
        let tags = filters.tag_filter.get();
        let statuses = filters.status_filter.get();
        let narrowing_notes = !tags.is_empty() || !statuses.is_empty();
        let (rows, by_picture) = self.shaped_rows(folder, &filters, needle.as_deref());
        let mut ids: Vec<u64> = rows.iter().map(|r| r.item.item_id).collect();
        if !narrowing_notes {
            ids.push(folder);
        }
        let mut resources = self.picture_resources(&ids);
        if let Some(needle) = &needle
            && let Some(pictures) = resources.get_mut(&folder)
        {
            pictures.retain(|p| picture_matches(p, needle));
        }
        let own = (!narrowing_notes)
            .then(|| resources.remove(&folder))
            .flatten()
            .filter(|p| !p.is_empty())
            .map(|pictures| GalleryNote {
                item_id: folder,
                title: self.folder_title(folder),
                depth: 0,
                // The folder itself, not one of its subfolders: its pictures are tiles.
                is_folder: false,
                pictures,
                excerpt: String::new(),
            });
        if rows.is_empty() && own.is_none() {
            self.watch_open_notes(&[]);
            return Vec::new();
        }
        let texts = self.note_texts(&ids);
        // The shaped rows' depths are relative to the folder whose subtree was loaded.
        let base = rows.iter().map(|r| r.depth).min().unwrap_or(0);
        let notes: Vec<GalleryNote> = own
            .into_iter()
            .chain(rows.into_iter().map(|row| {
                let item = row.item;
                let is_folder = item.role == BinderItemRole::Folder;
                // A folder's own synopsis is not its text: it has no pictures of its own
                // but its resources.
                // A note found by a picture shows that picture, not its text's.
                let text = if is_folder || by_picture.contains(&item.item_id) {
                    String::new()
                } else {
                    texts.get(&item.item_id).cloned().unwrap_or_default()
                };
                let mut pictures: Vec<GalleryPicture> = parse_images(&text)
                    .into_iter()
                    .map(GalleryPicture::in_text)
                    .collect();
                // Ancestors kept to show a match in context contribute no pictures of their own.
                if item.matches_tags_and_status(&tags, &statuses) {
                    let mut attached = resources.remove(&item.item_id).unwrap_or_default();
                    if let Some(needle) = &needle
                        && !row_matches(needle, &[&item.title, &item.label])
                    {
                        attached.retain(|picture| picture_matches(picture, needle));
                    }
                    pictures.extend(attached);
                }
                let plain = skrib_format::djot_plain_text(&text)
                    .map(|(plain, _)| plain)
                    .unwrap_or_default();
                GalleryNote {
                    item_id: item.item_id,
                    title: item.title,
                    depth: row.depth.saturating_sub(base),
                    is_folder,
                    pictures,
                    excerpt: tiles::excerpt(&plain),
                }
            }))
            .collect();
        self.watch_open_notes(&notes);
        notes
    }

    /// The rows of `folder` as the Overview shapes them, and, under a search, also the
    /// notes the search does not match but whose picture resources it does, with the rows
    /// above them; those notes are the second half of the answer. In the Overview's order
    /// either way.
    fn shaped_rows(
        &self,
        folder: u64,
        filters: &crate::models::OverviewFilters,
        needle: Option<&str>,
    ) -> (Vec<OverviewTreeRow>, HashSet<u64>) {
        let ctx = &self.inner.app_ctx;
        let work_id = &self.inner.work_id;
        let matched = shaped_subtree(ctx, work_id, folder, filters);
        let Some(needle) = needle.filter(|_| self.inner.resources.is_some()) else {
            return (matched, HashSet::new());
        };
        // The same rows with every filter but the search, so the order and the tag and
        // status filters are the Overview's.
        let unsearched = crate::models::OverviewFilters {
            query: Signal::new(String::new()),
            ..filters.clone()
        };
        let all = shaped_subtree(ctx, work_id, folder, &unsearched);
        let kept: HashSet<u64> = matched.iter().map(|r| r.item.item_id).collect();
        let tags = filters.tag_filter.get();
        let statuses = filters.status_filter.get();
        let candidates: Vec<u64> = all
            .iter()
            .filter(|row| row.item.matches_tags_and_status(&tags, &statuses))
            .map(|r| r.item.item_id)
            .filter(|id| !kept.contains(id))
            .collect();
        let by_picture: HashSet<u64> = self
            .picture_resources(&candidates)
            .into_iter()
            .filter(|(_, pictures)| pictures.iter().any(|p| picture_matches(p, needle)))
            .map(|(id, _)| id)
            .collect();
        if by_picture.is_empty() {
            return (matched, by_picture);
        }
        // Keep each found note's ancestors, as the Overview's search keeps a match's: the
        // rows before it, in depth-first order, each shallower than the last.
        let mut keep = kept;
        let mut above: Vec<(usize, u64)> = Vec::new();
        for row in &all {
            while above.last().is_some_and(|(depth, _)| *depth >= row.depth) {
                above.pop();
            }
            if by_picture.contains(&row.item.item_id) {
                keep.extend(above.iter().map(|(_, id)| *id));
                keep.insert(row.item.item_id);
            }
            above.push((row.depth, row.item.item_id));
        }
        let rows = all
            .into_iter()
            .filter(|r| keep.contains(&r.item.item_id))
            .collect();
        (rows, by_picture)
    }

    /// The picture resources of each of `item_ids`, each item's by title. Empty with no
    /// resources model.
    fn picture_resources(&self, item_ids: &[u64]) -> HashMap<u64, Vec<GalleryPicture>> {
        let Some((model, _)) = &self.inner.resources else {
            return HashMap::new();
        };
        let attachments = model.attachments(item_ids).unwrap_or_else(|e| {
            eprintln!("resources: could not read what the items carry: {e}");
            HashMap::new()
        });
        let mut wanted: Vec<u64> = attachments.values().flatten().copied().collect();
        wanted.sort_unstable();
        wanted.dedup();
        let rows: HashMap<u64, ResourceRow> = model
            .by_ids(&wanted)
            .into_iter()
            .filter(is_picture_resource)
            .map(|r| (r.id, r))
            .collect();
        attachments
            .into_iter()
            .filter_map(|(item, ids)| {
                let found: Vec<ResourceRow> = once_each(&ids)
                    .iter()
                    .filter_map(|id| rows.get(id).cloned())
                    .collect();
                let found = crate::resources::sorted_by_title(found);
                (!found.is_empty()).then(|| {
                    let pictures = found
                        .into_iter()
                        .enumerate()
                        .map(|(i, row)| GalleryPicture::of_resource(row, i))
                        .collect();
                    (item, pictures)
                })
            })
            .collect()
    }

    /// The title of `folder`: the tab's own, or the subfolder entered last.
    fn folder_title(&self, folder: u64) -> String {
        if folder == self.inner.container_id {
            return self
                .inner
                .resources
                .as_ref()
                .map(|(_, title)| title.get())
                .unwrap_or_default();
        }
        self.inner
            .trail
            .get()
            .into_iter()
            .find(|(id, _)| *id == folder)
            .map(|(_, title)| title)
            .unwrap_or_default()
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
            live.insert(note.item_id, text_srcs(&note.pictures));
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
    /// entry it opens on (a text or folder tile opens none). A picture resource's entry
    /// names its file, as the dock's own viewer does.
    pub fn viewer_entries(&self, tiles: &[GalleryTile]) -> (Vec<ViewerEntry>, Vec<Option<usize>>) {
        let mut entries = Vec::new();
        let mut at = Vec::with_capacity(tiles.len());
        for tile in tiles {
            match tile {
                GalleryTile::Picture {
                    item_id,
                    title,
                    picture,
                    ..
                } => {
                    at.push(Some(entries.len()));
                    let resource = picture.resource.as_deref();
                    entries.push(ViewerEntry {
                        src: picture.image.src.clone(),
                        alt: picture.image.alt.clone(),
                        item: Some(ViewerItem {
                            item_id: *item_id,
                            title: title.clone(),
                        }),
                        file_name: resource.map(|r| r.file_name.clone()),
                        file: resource.and_then(|r| {
                            Some(ViewerFile {
                                path: r.path.clone()?,
                                hash: r.content_hash.clone(),
                            })
                        }),
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

/// The pictures of a note's own text, as [`srcs`] lists them: what an edit in an open
/// note is compared against (its resources do not move when its text does).
fn text_srcs(pictures: &[GalleryPicture]) -> Vec<String> {
    pictures
        .iter()
        .filter(|p| !p.is_resource())
        .map(|p| p.image.src.clone())
        .collect()
}

/// Whether a search for `needle` (lower-cased) names this picture: a resource by its
/// title, its file name or its description. A picture of the text has no name of its own.
fn picture_matches(picture: &GalleryPicture, needle: &str) -> bool {
    picture
        .resource
        .as_ref()
        .is_some_and(|r| row_matches(needle, &[&r.title, &r.file_name, &r.description]))
}

/// `ids` with each kept once, where it first appears, wherever it repeats.
///
/// The store refuses a resource listed twice on an item, and loading a project collapses
/// the repeats, so this only stands between a list that somehow holds one and a picture
/// shown twice. Not `dedup`, which collapses only neighbours: `[A, B, A]` would stay.
fn once_each(ids: &[u64]) -> Vec<u64> {
    let mut seen = HashSet::new();
    ids.iter().copied().filter(|id| seen.insert(*id)).collect()
}

/// A resource the Gallery shows: a file in a picture format the editor paints. A web
/// link is never one, whatever its address ends with.
fn is_picture_resource(row: &ResourceRow) -> bool {
    // Judged by the file it opens, never only by its name: see `name_to_judge`.
    !row.is_web_link()
        && crate::resources::file_kind::is_viewable_picture(&crate::resources::name_to_judge(row))
}

#[cfg(all(test, not(feature = "mocks")))]
mod tests;
