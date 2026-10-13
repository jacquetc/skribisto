// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `BookCoverViewModel` — one Book's cover: what it is, choosing one, taking it off.
//!
//! A cover is a link from the Book row (`BinderItem.cover`) to one of the Work's pictures
//! (`Asset`). Choosing one is three writes that must undo as one: the picture's bytes go
//! into the media directory (content-addressed, so never undone and never duplicated), its
//! `Asset` row is found or created, and the Book is pointed at it. The last two run inside
//! one composite, so a single Ctrl+Z takes the cover back off.
//!
//! Removing a cover only drops the link. The picture stays in the project, for the reason
//! [`crate::shared::images::store`] gives: reclaiming storage is always an explicit act.

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use teksilo::prelude::*;
use teksilo::widgets::Toast;

use frontend::AppContext;
use frontend::asset::dtos::{AssetDto, CreateAssetDto};
use frontend::commands::{asset_commands, binder_item_commands, undo_redo_commands, work_commands};
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin, UndoRedoEvent};
use frontend::direct_access::BinderItemRelationshipDto;

use crate::app_ids::AppIds;
use crate::models::OpenDocsStore;
use crate::shared::images::{self, ImageError, PendingImage};
use crate::singles::{SingleAsset, SingleBinderItem};
use crate::toast_scope::ToastWorkExt;

/// What a Book's cover currently is.
#[derive(Debug, Clone, PartialEq)]
pub enum CoverState {
    /// No cover chosen.
    None,
    /// A cover whose picture is in the project's media directory.
    Shown { asset: AssetDto, path: PathBuf },
    /// A cover the Book still names, whose file is gone from the media directory. Said
    /// rather than drawn as an empty box, so the writer knows to choose it again.
    Missing { asset: AssetDto },
}

/// Why choosing a cover failed, each answered by its own toast.
#[derive(Debug)]
pub enum CoverError {
    /// No project is open, so there is no media directory to store the picture in.
    NoProject,
    /// The picture could not be written into the media directory.
    NotStored(ImageError),
    /// The picture is stored but the Book could not be pointed at it.
    NotRecorded(anyhow::Error),
}

struct Inner {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    docs: OpenDocsStore,
    item_id: u64,
    item: SingleBinderItem,
    asset: SingleAsset,
    /// Bumped whenever the cover may have changed; the view rebuilds on it.
    revision: Signal<u64>,
    /// What the view last showed: the cover's row and the Book's title (its alt text).
    /// `revision` moves only when this does, so an undo elsewhere in the project, which
    /// every cover block hears, rebuilds none of them.
    shown: RefCell<Option<(Option<AssetDto>, String)>>,
}

/// One Book's cover. Cheap to clone; every clone is the same view-model.
#[derive(Clone)]
pub struct BookCoverViewModel {
    inner: Rc<Inner>,
}

impl BookCoverViewModel {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds, docs: OpenDocsStore, item_id: u64) -> Self {
        let vm = Self {
            inner: Rc::new(Inner {
                item: SingleBinderItem::new(app_ctx.clone()),
                asset: SingleAsset::new(app_ctx.clone()),
                app_ctx,
                ids,
                docs,
                item_id,
                revision: Signal::new(0),
                shown: RefCell::new(None),
            }),
        };
        vm.sync();
        vm
    }

    /// Follow the Book and its picture: a cover set or cleared from anywhere (this page,
    /// an undo, a promote that turned the Book into something else) and a picture row
    /// that changed or went away. Call from `build`, every build.
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.inner.asset.wire(ctx);
        let item_id = self.inner.item_id;
        let vm = self.clone();
        ctx.subscribe_event(
            Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
            move |event: &Event| {
                if event.ids.contains(&item_id) {
                    vm.sync();
                }
            },
        );
        for event in [
            Origin::DirectAccess(DirectAccessEntity::Asset(EntityEvent::Updated)),
            Origin::DirectAccess(DirectAccessEntity::Asset(EntityEvent::Removed)),
            Origin::UndoRedo(UndoRedoEvent::Undone),
            Origin::UndoRedo(UndoRedoEvent::Redone),
        ] {
            let vm = self.clone();
            ctx.subscribe_event(event, move |_: &Event| vm.sync());
        }
    }

    /// Moves whenever the cover may have changed.
    pub fn revision(&self) -> Signal<u64> {
        self.inner.revision.clone()
    }

    pub fn item_id(&self) -> u64 {
        self.inner.item_id
    }

    /// The Book's own title, for the picture's alt text.
    pub fn book_title(&self) -> String {
        self.inner.item.dto().map(|d| d.title).unwrap_or_default()
    }

    /// What the cover is right now.
    pub fn state(&self) -> CoverState {
        let Some(asset) = self.inner.asset.dto() else {
            return CoverState::None;
        };
        let media_dir = self.inner.docs.media_dir();
        let ext = skrib_format::media::extension_for(&asset.mime_type);
        let path = media_dir.join(format!("{}.{ext}", asset.content_hash));
        if !media_dir.as_os_str().is_empty() && path.is_file() {
            CoverState::Shown { asset, path }
        } else {
            CoverState::Missing { asset }
        }
    }

    /// Make `image` this Book's cover: stored in the media directory, recorded as one of
    /// the Work's pictures (the same picture already in the project is reused, never
    /// copied), and linked from the Book. One undo entry.
    pub fn set_cover(&self, image: &PendingImage) -> Result<(), CoverError> {
        let Some(work_id) = self.inner.ids.work_id.get() else {
            return Err(CoverError::NoProject);
        };
        let media_dir = self.inner.docs.media_dir();
        if media_dir.as_os_str().is_empty() {
            return Err(CoverError::NoProject);
        }
        // A cover is stored as the writer's file, never downscaled: it is the one picture
        // in the project whose resolution is the whole point, and an ereader shows it
        // full-screen.
        images::store(image, &media_dir).map_err(CoverError::NotStored)?;

        let ctx = &self.inner.app_ctx;
        let stack = self.inner.ids.stack_id.get();
        // This Work's pictures, never `get_all_asset`: one process holds every open
        // project in one store, and an `Asset` carries no back-link to its Work.
        let existing =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Assets)
                .and_then(|ids| asset_commands::get_asset_multi(ctx, &ids))
                .map_err(CoverError::NotRecorded)?
                .into_iter()
                .flatten()
                .find(|a| a.content_hash == image.content_hash);
        if let Some(a) = &existing
            && self.inner.item.dto().and_then(|d| d.cover) == Some(a.id)
        {
            // Already this Book's cover: nothing to record, and no empty undo entry.
            return Ok(());
        }

        undo_redo_commands::begin_composite(ctx, stack).map_err(CoverError::NotRecorded)?;
        let linked = (|| -> anyhow::Result<()> {
            let asset_id = match existing {
                Some(a) => a.id,
                None => {
                    asset_commands::create_asset(
                        ctx,
                        stack,
                        &CreateAssetDto {
                            created_at: chrono::Utc::now(),
                            updated_at: chrono::Utc::now(),
                            content_hash: image.content_hash.clone(),
                            file_name: image.file_name.clone(),
                            mime_type: image.mime_type.clone(),
                            width: u64::from(image.width),
                            height: u64::from(image.height),
                            byte_size: image.bytes.len() as u64,
                            alt: String::new(),
                        },
                        work_id,
                        -1,
                    )?
                    .id
                }
            };
            self.link(Some(asset_id))
        })();
        match linked {
            Ok(()) => undo_redo_commands::end_composite(ctx),
            // Cancelled, not ended: a group left open would swallow every later edit of
            // the session into one undo entry.
            Err(e) => {
                undo_redo_commands::cancel_composite(ctx);
                self.sync();
                return Err(CoverError::NotRecorded(e));
            }
        }
        self.sync();
        Ok(())
    }

    /// Take the cover off this Book. The picture stays in the project.
    pub fn remove(&self) -> anyhow::Result<()> {
        if self.inner.item.dto().and_then(|d| d.cover).is_none() {
            return Ok(());
        }
        self.link(None)?;
        self.sync();
        Ok(())
    }

    /// The whole "Choose…" gesture: the file dialog, then [`Self::set_cover`], then the
    /// toast that says how it went.
    pub fn choose(&self, c: &mut EventContext) {
        let work_id = self.inner.ids.work_id.get();
        if self.inner.docs.media_dir().as_os_str().is_empty() {
            c.show_toast(
                Toast::warning(tr!(image_no_project())).scoped_id("images.no_project", work_id),
            );
            return;
        }
        let req = crate::models::dialog_start_in(
            c,
            crate::models::FolderPurpose::InsertImage,
            FileDialogRequest::pick_file()
                .title(tr!(cover_choose_title()))
                .add_filter(
                    tr!(image_filter_label()).resolve_now(),
                    &["png", "jpg", "jpeg", "webp"],
                ),
        );
        let vm = self.clone();
        let _ = c.pick_file(req, move |res, c| {
            let FileDialogResult::File(Some(path)) = res else {
                return;
            };
            crate::models::remember_dialog_file(
                c,
                crate::models::FolderPurpose::InsertImage,
                &path,
            );
            let work_id = vm.inner.ids.work_id.get();
            let outcome = images::examine(&path)
                .map_err(CoverError::NotStored)
                .and_then(|image| vm.set_cover(&image));
            let toast = match outcome {
                Ok(()) => Toast::success(tr!(cover_set())).scoped_id("images.cover", work_id),
                Err(CoverError::NoProject) => {
                    Toast::warning(tr!(image_no_project())).scoped_id("images.no_project", work_id)
                }
                Err(CoverError::NotStored(e)) => {
                    Toast::error(lit!(e.to_string())).scoped_id("images.failed", work_id)
                }
                Err(CoverError::NotRecorded(e)) => {
                    eprintln!("book cover: recording the cover failed: {e:#}");
                    Toast::error(tr!(image_not_recorded())).scoped_id("images.failed", work_id)
                }
            };
            c.show_toast(toast);
        });
    }

    /// [`Self::remove`], with the toast that says so.
    pub fn remove_and_report(&self, c: &mut EventContext) {
        let work_id = self.inner.ids.work_id.get();
        let toast = match self.remove() {
            Ok(()) => Toast::success(tr!(cover_cleared())).scoped_id("images.cover", work_id),
            Err(e) => {
                eprintln!("book cover: removing the cover failed: {e:#}");
                Toast::error(tr!(image_not_recorded())).scoped_id("images.failed", work_id)
            }
        };
        c.show_toast(toast);
    }

    fn link(&self, asset: Option<u64>) -> anyhow::Result<()> {
        binder_item_commands::set_binder_item_relationship(
            &self.inner.app_ctx,
            self.inner.ids.stack_id.get(),
            &BinderItemRelationshipDto {
                id: self.inner.item_id,
                field: BinderItemRelationshipField::Cover,
                right_ids: asset.into_iter().collect(),
            },
        )
    }

    /// Re-read the Book and the picture it names, and tell the view.
    fn sync(&self) {
        self.inner.item.set_id(Some(self.inner.item_id));
        let cover = self.inner.item.dto().and_then(|d| d.cover);
        self.inner.asset.set_id(cover);
        let now = Some((self.inner.asset.dto(), self.book_title()));
        if *self.inner.shown.borrow() != now {
            *self.inner.shown.borrow_mut() = now;
            self.inner
                .revision
                .set(self.inner.revision.get().wrapping_add(1));
        }
    }
}

#[cfg(all(test, not(feature = "mocks")))]
mod tests;
