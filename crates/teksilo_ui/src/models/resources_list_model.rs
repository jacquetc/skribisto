// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The open Work's resources: the files and web links its items carry, as rows a dock
//! can list, and every write the Resources feature makes to them.
//!
//! One model per open project (it lives on the project's
//! [`WorkSession`](crate::sessions::WorkSession)), project-wide: [`rows_for`] answers
//! for one item, [`attachments`] for many at once (a folder's subtree), [`unattached`]
//! for the resources no item carries, [`all`] for the whole project. A dock filters by
//! the item in front of the writer, so two windows on one project read one model.
//!
//! A row carries what the backend stores plus two facts only this machine can answer:
//! where the file is ([`ResourceRow::path`]: the stored file in the project's resources
//! directory, or the linked file), and whether it is there ([`ResourceRow::missing`]).
//! The resources directory is resolved each time, as
//! [`crate::media_paths::project_media_dir`] resolves the media directory, because a
//! Save As moves it.
//!
//! Two `#[cfg]`-gated `mod imp` share one public surface (see [`crate::models`]); the
//! mock one holds realistic rows in memory and applies the writes to them, because the
//! automation scripts drive the mocks build.
//!
//! [`rows_for`]: imp::ResourcesListModel::rows_for
//! [`attachments`]: imp::ResourcesListModel::attachments
//! [`unattached`]: imp::ResourcesListModel::unattached
//! [`all`]: imp::ResourcesListModel::all

use std::path::PathBuf;

pub use frontend::common::entities::ResourceKind;
pub use frontend::resource_management::StoredResourceFilesDto;

/// One resource, as the dock shows it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResourceRow {
    pub id: u64,
    pub uid: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub kind: ResourceKind,
    /// The absolute path of a linked file; the address of a web link; empty for a
    /// stored file.
    pub location: String,
    pub file_name: String,
    pub mime_type: String,
    pub byte_size: u64,
    pub content_hash: String,
    pub width: u64,
    pub height: u64,
    /// Where the bytes are on this machine: the stored file in the project's resources
    /// directory, or the linked file. `None` for a web link.
    pub path: Option<PathBuf>,
    /// A file this row names is not there: a linked file moved or deleted, a stored
    /// file whose bytes were lost. Never true for a web link.
    pub missing: bool,
}

impl ResourceRow {
    pub fn is_web_link(&self) -> bool {
        self.kind == ResourceKind::WebLink
    }

    pub fn is_stored(&self) -> bool {
        self.kind == ResourceKind::StoredFile
    }

    pub fn is_linked(&self) -> bool {
        self.kind == ResourceKind::LinkedFile
    }

    /// The name of the file, or the address of a web link.
    pub fn display_name(&self) -> &str {
        if self.is_web_link() {
            &self.location
        } else {
            &self.file_name
        }
    }
}

/// A stored file's facts after an application outside changed it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FileFacts {
    pub byte_size: u64,
    pub content_hash: String,
    pub width: u64,
    pub height: u64,
}

impl FileFacts {
    /// The facts `row` records now.
    pub fn of(row: &ResourceRow) -> Self {
        Self {
            byte_size: row.byte_size,
            content_hash: row.content_hash.clone(),
            width: row.width,
            height: row.height,
        }
    }
}

/// How a request to store files began.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StoreStart {
    /// A long operation is copying them; its result arrives with its completion.
    Running(String),
    /// Finished already (the mock model, which has no backend to wait on).
    Done(Box<StoredResourceFilesDto>),
}

/// What a web link is made of, for [`imp::ResourcesListModel::add`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WebLink {
    pub url: String,
    pub title: String,
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::collections::{HashMap, HashSet};
    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{
        binder_item_commands, long_operation_commands, resource_commands,
        resource_management_commands, undo_redo_commands, work_commands,
    };
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::event::{
        BinderItemManagementEvent, DirectAccessEntity, EntityEvent, Origin,
        ResourceManagementEvent, TrashManagementEvent, UndoRedoEvent, WorkManagementEvent,
    };
    use frontend::direct_access::{BinderItemRelationshipDto, ResourceDto, UpdateResourceDto};
    use frontend::resource_management::{AddResourcesDto, StoreResourceFilesDto};

    use super::{
        FileFacts, ResourceKind, ResourceRow, StoreStart, StoredResourceFilesDto, WebLink,
    };
    use crate::app_ids::AppIds;

    struct Inner {
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        /// Bumped on every refresh; bind at `BindingLevel::Rebuild`.
        version: Signal<u64>,
        /// The media root the resources directory is resolved under, when it is not
        /// the app's own: a test's temporary folder.
        media_root: std::cell::RefCell<Option<PathBuf>>,
    }

    #[derive(Clone)]
    pub struct ResourcesListModel {
        inner: Rc<Inner>,
    }

    impl ResourcesListModel {
        pub fn new(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
            Self {
                inner: Rc::new(Inner {
                    app_ctx,
                    ids,
                    version: Signal::new(0),
                    media_root: std::cell::RefCell::new(None),
                }),
            }
        }

        /// Resolve the resources directory under `root` rather than the app's media
        /// root, so a test stores its files in a folder of its own.
        pub fn set_media_root(&self, root: PathBuf) {
            *self.inner.media_root.borrow_mut() = Some(root);
        }

        /// Subscribe so the rows follow the backend. Call from each window's build; a
        /// subscription is scoped to the build that made it.
        pub fn wire(&self, ctx: &mut BuildContext) {
            use DirectAccessEntity::{BinderItem, Resource, Work};
            use EntityEvent::{Created, Removed, Updated};
            let origins = [
                Origin::DirectAccess(Resource(Created)),
                Origin::DirectAccess(Resource(Updated)),
                Origin::DirectAccess(Resource(Removed)),
                // An attachment or a detachment is the item's relationship moving.
                Origin::DirectAccess(BinderItem(Updated)),
                Origin::DirectAccess(Work(Updated)),
                Origin::ResourceManagement(ResourceManagementEvent::AddResources),
                // A folder's Resources page lists its subtree's: a move, a trash or a
                // restore changes which items that is.
                Origin::DirectAccess(BinderItem(Removed)),
                Origin::BinderItemManagement(BinderItemManagementEvent::MoveItems),
                Origin::BinderItemManagement(BinderItemManagementEvent::Duplicate),
                Origin::BinderItemManagement(BinderItemManagementEvent::Promote),
                Origin::TrashManagement(TrashManagementEvent::TrashBinderItems),
                Origin::TrashManagement(TrashManagementEvent::TrashBinder),
                Origin::TrashManagement(TrashManagementEvent::RestoreItems),
                Origin::TrashManagement(TrashManagementEvent::EmptyTrash),
                Origin::UndoRedo(UndoRedoEvent::Undone),
                Origin::UndoRedo(UndoRedoEvent::Redone),
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                Origin::WorkManagement(WorkManagementEvent::NewWork),
                // A save can move the resources directory (a new folder project's
                // first save, a Save As), which moves every row's path.
                Origin::WorkManagement(WorkManagementEvent::SaveWork),
                Origin::WorkManagement(WorkManagementEvent::SaveAs),
            ];
            let me = self.clone();
            crate::models::coalesced_reload::reload_on_events(ctx, origins, move || me.reload());
        }

        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        /// Ask every reader to look again, now.
        pub fn reload(&self) {
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }

        fn stack(&self) -> Option<u64> {
            self.inner.ids.stack_id.get()
        }

        fn work_id(&self) -> Option<u64> {
            self.inner.ids.work_id.get()
        }

        /// Where this project's stored files are now: see the module docs.
        pub fn resources_dir(&self) -> PathBuf {
            match self.work_id() {
                Some(work_id) => crate::media_paths::project_resources_dir(
                    &self.inner.app_ctx,
                    work_id,
                    &self
                        .inner
                        .media_root
                        .borrow()
                        .clone()
                        .unwrap_or_else(crate::media_paths::media_root),
                ),
                None => PathBuf::new(),
            }
        }

        /// The resources attached to `item_id`, in the order they were attached.
        pub fn rows_for(&self, item_id: u64) -> Vec<ResourceRow> {
            let ids = binder_item_commands::get_binder_item_relationship(
                &self.inner.app_ctx,
                &item_id,
                &BinderItemRelationshipField::Resources,
            )
            .unwrap_or_default();
            self.rows_of(&ids)
        }

        /// Every resource of the project.
        pub fn all(&self) -> Vec<ResourceRow> {
            let Some(work_id) = self.work_id() else {
                return Vec::new();
            };
            let ids = work_commands::get_work_relationship(
                &self.inner.app_ctx,
                &work_id,
                &WorkRelationshipField::Resources,
            )
            .unwrap_or_default();
            self.rows_of(&ids)
        }

        /// The resource whose durable id is `uid`, if this project has one.
        pub fn by_uid(&self, uid: uuid::Uuid) -> Option<ResourceRow> {
            self.all().into_iter().find(|r| r.uid == uid)
        }

        /// The resource `id`, if this project has it.
        pub fn by_id(&self, id: u64) -> Option<ResourceRow> {
            self.rows_of(&[id]).into_iter().next()
        }

        /// The resources `ids` this project has, in no particular order.
        pub fn by_ids(&self, ids: &[u64]) -> Vec<ResourceRow> {
            self.rows_of(ids)
        }

        /// The title of the binder item `item_id`, empty when it has none or is gone:
        /// what a toast names the item by when a resource is dropped on it.
        pub fn item_title(&self, item_id: u64) -> String {
            binder_item_commands::get_binder_item(&self.inner.app_ctx, &item_id)
                .ok()
                .flatten()
                .map(|item| item.title)
                .unwrap_or_default()
        }

        /// The resource ids attached to each of `item_ids`, in the order they were
        /// attached. An item with none is absent or maps to an empty list.
        pub fn attachments(&self, item_ids: &[u64]) -> HashMap<u64, Vec<u64>> {
            if item_ids.is_empty() {
                return HashMap::new();
            }
            binder_item_commands::get_binder_item_relationship_many(
                &self.inner.app_ctx,
                item_ids,
                &BinderItemRelationshipField::Resources,
            )
            .unwrap_or_default()
        }

        /// The project's resources that no item carries, trashed items included: a
        /// resource on an item in the trash comes back with it, so it is not loose.
        pub fn unattached(&self) -> Vec<ResourceRow> {
            let Some(work_id) = self.work_id() else {
                return Vec::new();
            };
            let items: Vec<u64> =
                crate::models::binder_stream::ordered_all_items(&self.inner.app_ctx, work_id)
                    .into_iter()
                    .map(|(_, item)| item.id)
                    .collect();
            let held: HashSet<u64> = self.attachments(&items).into_values().flatten().collect();
            self.all()
                .into_iter()
                .filter(|r| !held.contains(&r.id))
                .collect()
        }

        /// Attach the resources `ids` to every item of `item_ids`, keeping what each
        /// already carries: one undo step.
        pub fn attach(&self, item_ids: &[u64], ids: &[u64]) -> Result<(), String> {
            let ctx = &self.inner.app_ctx;
            let field = BinderItemRelationshipField::Resources;
            let current = self.attachments(item_ids);
            let writes: Vec<BinderItemRelationshipDto> = item_ids
                .iter()
                .filter_map(|item| {
                    let mut right_ids = current.get(item).cloned().unwrap_or_default();
                    let before = right_ids.len();
                    for id in ids {
                        if !right_ids.contains(id) {
                            right_ids.push(*id);
                        }
                    }
                    (right_ids.len() != before).then(|| BinderItemRelationshipDto {
                        id: *item,
                        field: field.clone(),
                        right_ids,
                    })
                })
                .collect();
            if writes.is_empty() {
                return Ok(());
            }
            let stack = self.stack();
            let grouped = writes.len() > 1;
            if grouped {
                undo_redo_commands::begin_composite(ctx, stack).map_err(|e| format!("{e:#}"))?;
            }
            let wrote = writes.iter().try_for_each(|dto| {
                binder_item_commands::set_binder_item_relationship(ctx, stack, dto)
            });
            // Closed on every path, so a failed write never leaves the group open.
            if grouped {
                undo_redo_commands::end_composite(ctx);
            }
            wrote.map_err(|e| format!("{e:#}"))
        }

        fn rows_of(&self, ids: &[u64]) -> Vec<ResourceRow> {
            if ids.is_empty() {
                return Vec::new();
            }
            let dir = self.resources_dir();
            resource_commands::get_resource_multi(&self.inner.app_ctx, ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .map(|dto| row_of(dto, &dir))
                .collect()
        }

        /// Start copying `paths` into `dest_dir`, one uid folder each.
        pub fn start_store(
            &self,
            dest_dir: &Path,
            paths: &[PathBuf],
        ) -> Result<StoreStart, String> {
            resource_management_commands::store_resource_files(
                &self.inner.app_ctx,
                &StoreResourceFilesDto {
                    dest_dir: dest_dir.to_string_lossy().into_owned(),
                    source_paths: paths
                        .iter()
                        .map(|p| p.to_string_lossy().into_owned())
                        .collect(),
                },
            )
            .map(StoreStart::Running)
            .map_err(|e| format!("{e:#}"))
        }

        /// What a finished store run copied.
        pub fn store_result(&self, op_id: &str) -> Option<StoredResourceFilesDto> {
            resource_management_commands::get_store_resource_files_result(
                &self.inner.app_ctx,
                op_id,
            )
            .ok()
            .flatten()
        }

        pub fn cancel_store(&self, op_id: &str) {
            long_operation_commands::cancel_operation(&self.inner.app_ctx, op_id);
        }

        /// Create the resources `stored` (from a store run), `linked` and `web`, list them
        /// on the project and attach them to `item_ids`: one undo step. Returns the new
        /// rows' ids, in that order.
        pub fn add(
            &self,
            item_ids: &[u64],
            stored: &StoredResourceFilesDto,
            linked: &[PathBuf],
            web: &[WebLink],
        ) -> Result<Vec<u64>, String> {
            let work_id = self
                .work_id()
                .ok_or_else(|| "no project is open".to_string())?;
            let n = stored.uids.len();
            resource_management_commands::add_resources(
                &self.inner.app_ctx,
                self.stack(),
                &AddResourcesDto {
                    work_id,
                    item_ids: item_ids.to_vec(),
                    stored_uids: stored.uids.clone(),
                    stored_titles: vec![String::new(); n],
                    stored_file_names: stored.file_names.clone(),
                    stored_mime_types: stored.mime_types.clone(),
                    stored_byte_sizes: stored.byte_sizes.clone(),
                    stored_content_hashes: stored.content_hashes.clone(),
                    stored_widths: stored.widths.clone(),
                    stored_heights: stored.heights.clone(),
                    linked_paths: linked
                        .iter()
                        .map(|p| p.to_string_lossy().into_owned())
                        .collect(),
                    web_urls: web.iter().map(|w| w.url.clone()).collect(),
                    web_titles: web.iter().map(|w| w.title.clone()).collect(),
                },
            )
            .map(|r| r.created_ids)
            .map_err(|e| format!("{e:#}"))
        }

        fn update(
            &self,
            id: u64,
            stack: Option<u64>,
            edit: impl FnOnce(&mut UpdateResourceDto),
        ) -> Result<(), String> {
            let dto = resource_commands::get_resource(&self.inner.app_ctx, &id)
                .map_err(|e| format!("{e:#}"))?
                .ok_or_else(|| format!("resource {id} is gone"))?;
            let mut update = UpdateResourceDto::from(dto);
            edit(&mut update);
            update.updated_at = chrono::Utc::now();
            resource_commands::update_resource(&self.inner.app_ctx, stack, &update)
                .map(|_| ())
                .map_err(|e| format!("{e:#}"))
        }

        /// A new title and description, as one undo step.
        pub fn edit_text(&self, id: u64, title: &str, description: &str) -> Result<(), String> {
            self.update(id, self.stack(), |u| {
                u.title = title.to_string();
                u.description = description.to_string();
            })
        }

        /// Record what a stored file is after another application changed it.
        ///
        /// Off the writer's undo stack: the bytes on disk changed outside it, and a
        /// Ctrl+Z that took the row back without the bytes would leave the two
        /// disagreeing. The notice's own Undo puts both back.
        pub fn set_file_facts(&self, id: u64, facts: &FileFacts) -> Result<(), String> {
            self.update(
                id,
                Some(frontend::common::undo_redo::UNTRACKED_STACK_ID),
                |u| {
                    u.byte_size = facts.byte_size;
                    u.content_hash = facts.content_hash.clone();
                    u.width = facts.width;
                    u.height = facts.height;
                },
            )
        }

        /// Point the linked file `id` at `path`, as one undo step.
        pub fn relink(&self, id: u64, path: &Path) -> Result<(), String> {
            let file_name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            self.update(id, self.stack(), |u| {
                u.location = path.to_string_lossy().into_owned();
                u.file_name = file_name;
            })
        }

        /// Take the resource `id` off `item_id`, leaving it in the project.
        pub fn detach(&self, item_id: u64, id: u64) -> Result<(), String> {
            let ctx = &self.inner.app_ctx;
            let field = BinderItemRelationshipField::Resources;
            let mut attached =
                binder_item_commands::get_binder_item_relationship(ctx, &item_id, &field)
                    .map_err(|e| format!("{e:#}"))?;
            attached.retain(|r| *r != id);
            binder_item_commands::set_binder_item_relationship(
                ctx,
                self.stack(),
                &BinderItemRelationshipDto {
                    id: item_id,
                    field,
                    right_ids: attached,
                },
            )
            .map_err(|e| format!("{e:#}"))
        }

        /// Remove the resources `ids` from the project, as one undo step. Their stored
        /// files stay on disk until the project is next opened, so an Undo finds them.
        pub fn remove(&self, ids: &[u64]) -> Result<(), String> {
            resource_commands::remove_resource_multi(&self.inner.app_ctx, self.stack(), ids)
                .map_err(|e| format!("{e:#}"))
        }
    }

    fn row_of(dto: ResourceDto, dir: &Path) -> ResourceRow {
        let path = match dto.kind {
            ResourceKind::StoredFile if !dir.as_os_str().is_empty() => Some(
                skrib_format::media::resource_file(dir, dto.uid, &dto.file_name),
            ),
            ResourceKind::StoredFile => None,
            ResourceKind::LinkedFile => Some(PathBuf::from(&dto.location)),
            ResourceKind::WebLink => None,
        };
        let missing = match dto.kind {
            ResourceKind::WebLink => false,
            _ => !path.as_deref().is_some_and(Path::is_file),
        };
        ResourceRow {
            id: dto.id,
            uid: dto.uid,
            title: dto.title,
            description: dto.description,
            kind: dto.kind,
            location: dto.location,
            file_name: dto.file_name,
            mime_type: dto.mime_type,
            byte_size: dto.byte_size,
            content_hash: dto.content_hash,
            width: dto.width,
            height: dto.height,
            path,
            missing,
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    use teksilo::prelude::*;

    use frontend::AppContext;

    use super::{
        FileFacts, ResourceKind, ResourceRow, StoreStart, StoredResourceFilesDto, WebLink,
    };
    use crate::app_ids::AppIds;

    struct Inner {
        version: Signal<u64>,
        rows: RefCell<Vec<ResourceRow>>,
        /// `(item, resource)` pairs, in the order they were attached.
        attached: RefCell<Vec<(u64, u64)>>,
        next_id: RefCell<u64>,
    }

    #[derive(Clone)]
    pub struct ResourcesListModel {
        inner: Rc<Inner>,
    }

    fn mock_dir() -> PathBuf {
        crate::media_paths::media_root().join("mock-resources")
    }

    fn uid(n: u128) -> uuid::Uuid {
        uuid::Uuid::from_u128(0x5e5_0000_0000_4000_8000_0000_0000_0000 + n)
    }

    /// Where the seeded resources are attached, as `(item, resource)` pairs over the mock
    /// binder's items: the map on the Book and on Chapter Two (one resource on two
    /// items), the photograph on a scene and on the character sketch, the recording on
    /// Scene 3, the moved tide tables on Confrontation, the web page on Chapter Two. The
    /// cottage photograph and the lighthouse page are attached to nothing, for the
    /// Work's Resources settings page.
    fn seed_attachments() -> Vec<(u64, u64)> {
        vec![
            (101, 9001),
            (104, 9001),
            (201, 9002),
            (106, 9002),
            (203, 9003),
            (105, 9004),
            (104, 9005),
        ]
    }

    /// A writer's research for a seaside novel: a map and two photographs stored in
    /// the project, a recording, a linked file that moved, and two web pages.
    fn seed() -> Vec<ResourceRow> {
        let stored = |id: u64, n: u128, title: &str, file: &str, mime: &str, size: u64| {
            let uid = uid(n);
            ResourceRow {
                id,
                uid,
                title: title.to_string(),
                kind: ResourceKind::StoredFile,
                file_name: file.to_string(),
                mime_type: mime.to_string(),
                byte_size: size,
                content_hash: format!("{:064x}", n),
                path: Some(skrib_format::media::resource_file(&mock_dir(), uid, file)),
                ..Default::default()
            }
        };
        vec![
            ResourceRow {
                description: "The harbour and the two roads up to the lighthouse.".into(),
                ..stored(
                    9001,
                    1,
                    "Village map",
                    "Village map.pdf",
                    "application/pdf",
                    2_621_440,
                )
            },
            ResourceRow {
                width: 1600,
                height: 1067,
                ..stored(
                    9002,
                    2,
                    "Lighthouse at dusk",
                    "Lighthouse at dusk.jpg",
                    "image/jpeg",
                    482_304,
                )
            },
            stored(
                9003,
                3,
                "Interview with the keeper",
                "Keeper interview.mp3",
                "audio/mpeg",
                18_874_368,
            ),
            ResourceRow {
                id: 9004,
                uid: uid(4),
                title: "Tide tables 1887".into(),
                kind: ResourceKind::LinkedFile,
                location: "/home/writer/Research/Tide tables 1887.pdf".into(),
                file_name: "Tide tables 1887.pdf".into(),
                path: Some(PathBuf::from("/home/writer/Research/Tide tables 1887.pdf")),
                missing: true,
                ..Default::default()
            },
            ResourceRow {
                id: 9005,
                uid: uid(5),
                title: "Harbour records".into(),
                description: "Ship arrivals, 1880 to 1895.".into(),
                kind: ResourceKind::WebLink,
                location: "https://example.org/harbour-records".into(),
                ..Default::default()
            },
            ResourceRow {
                width: 1200,
                height: 900,
                ..stored(
                    9006,
                    6,
                    "Keeper's cottage",
                    "Keeper's cottage.jpg",
                    "image/jpeg",
                    356_352,
                )
            },
            ResourceRow {
                id: 9007,
                uid: uid(7),
                title: "Lighthouse history".into(),
                kind: ResourceKind::WebLink,
                location: "https://example.org/lighthouse-history".into(),
                ..Default::default()
            },
        ]
    }

    impl ResourcesListModel {
        pub fn new(_app_ctx: Rc<AppContext>, _ids: AppIds) -> Self {
            Self {
                inner: Rc::new(Inner {
                    version: Signal::new(0),
                    rows: RefCell::new(seed()),
                    attached: RefCell::new(seed_attachments()),
                    next_id: RefCell::new(9100),
                }),
            }
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}

        /// No backend directory to resolve: the mock folder stays where it is.
        pub fn set_media_root(&self, _root: PathBuf) {}

        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        pub fn reload(&self) {
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }

        pub fn resources_dir(&self) -> PathBuf {
            mock_dir()
        }

        /// The resources attached to `item_id`, in the order they were attached.
        pub fn rows_for(&self, item_id: u64) -> Vec<ResourceRow> {
            let ids: Vec<u64> = self
                .inner
                .attached
                .borrow()
                .iter()
                .filter(|(item, _)| *item == item_id)
                .map(|(_, id)| *id)
                .collect();
            let rows = self.inner.rows.borrow();
            ids.iter()
                .filter_map(|id| rows.iter().find(|r| r.id == *id).cloned())
                .collect()
        }

        pub fn all(&self) -> Vec<ResourceRow> {
            self.inner.rows.borrow().clone()
        }

        pub fn by_uid(&self, uid: uuid::Uuid) -> Option<ResourceRow> {
            self.all().into_iter().find(|r| r.uid == uid)
        }

        pub fn by_id(&self, id: u64) -> Option<ResourceRow> {
            self.all().into_iter().find(|r| r.id == id)
        }

        pub fn by_ids(&self, ids: &[u64]) -> Vec<ResourceRow> {
            self.all()
                .into_iter()
                .filter(|r| ids.contains(&r.id))
                .collect()
        }

        pub fn item_title(&self, item_id: u64) -> String {
            format!("Item {item_id}")
        }

        pub fn attachments(&self, item_ids: &[u64]) -> HashMap<u64, Vec<u64>> {
            let mut out: HashMap<u64, Vec<u64>> = HashMap::new();
            for (item, id) in self.inner.attached.borrow().iter() {
                if item_ids.contains(item) {
                    out.entry(*item).or_default().push(*id);
                }
            }
            out
        }

        pub fn unattached(&self) -> Vec<ResourceRow> {
            let attached = self.inner.attached.borrow();
            self.all()
                .into_iter()
                .filter(|r| !attached.iter().any(|(_, id)| *id == r.id))
                .collect()
        }

        pub fn attach(&self, item_ids: &[u64], ids: &[u64]) -> Result<(), String> {
            {
                let mut attached = self.inner.attached.borrow_mut();
                for item in item_ids {
                    for id in ids {
                        if !attached.contains(&(*item, *id)) {
                            attached.push((*item, *id));
                        }
                    }
                }
            }
            self.reload();
            Ok(())
        }

        /// No backend to copy through: say what the files are, as the store run would.
        pub fn start_store(
            &self,
            _dest_dir: &Path,
            paths: &[PathBuf],
        ) -> Result<StoreStart, String> {
            let mut out = StoredResourceFilesDto::default();
            for path in paths {
                let source = path.to_string_lossy().into_owned();
                match crate::resources::opened_files::hash_file(path) {
                    Ok((len, hash)) => {
                        let name = path
                            .file_name()
                            .map(|n| n.to_string_lossy().into_owned())
                            .unwrap_or_default();
                        out.source_paths.push(source);
                        out.uids.push(uuid::Uuid::new_v4().hyphenated().to_string());
                        out.mime_types.push(String::new());
                        out.file_names.push(name);
                        out.byte_sizes.push(len);
                        out.content_hashes.push(hash);
                        out.widths.push(0);
                        out.heights.push(0);
                    }
                    Err(e) => {
                        out.failed_paths.push(source);
                        out.failure_reasons.push(e.to_string());
                    }
                }
            }
            Ok(StoreStart::Done(Box::new(out)))
        }

        pub fn store_result(&self, _op_id: &str) -> Option<StoredResourceFilesDto> {
            None
        }

        pub fn cancel_store(&self, _op_id: &str) {}

        fn next_id(&self) -> u64 {
            let mut next = self.inner.next_id.borrow_mut();
            *next += 1;
            *next
        }

        pub fn add(
            &self,
            item_ids: &[u64],
            stored: &StoredResourceFilesDto,
            linked: &[PathBuf],
            web: &[WebLink],
        ) -> Result<Vec<u64>, String> {
            let mut created = Vec::new();
            let mut rows = Vec::new();
            for (i, raw) in stored.uids.iter().enumerate() {
                let uid = uuid::Uuid::parse_str(raw).map_err(|e| e.to_string())?;
                let name = stored.file_names[i].clone();
                rows.push(ResourceRow {
                    id: self.next_id(),
                    uid,
                    title: title_of_file(&name),
                    kind: ResourceKind::StoredFile,
                    path: Some(skrib_format::media::resource_file(&mock_dir(), uid, &name)),
                    file_name: name,
                    byte_size: stored.byte_sizes[i],
                    content_hash: stored.content_hashes[i].clone(),
                    ..Default::default()
                });
            }
            for path in linked {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                rows.push(ResourceRow {
                    id: self.next_id(),
                    uid: uuid::Uuid::new_v4(),
                    title: title_of_file(&name),
                    kind: ResourceKind::LinkedFile,
                    location: path.to_string_lossy().into_owned(),
                    file_name: name,
                    path: Some(path.clone()),
                    missing: !path.is_file(),
                    ..Default::default()
                });
            }
            for link in web {
                rows.push(ResourceRow {
                    id: self.next_id(),
                    uid: uuid::Uuid::new_v4(),
                    title: if link.title.trim().is_empty() {
                        link.url.clone()
                    } else {
                        link.title.clone()
                    },
                    kind: ResourceKind::WebLink,
                    location: link.url.clone(),
                    ..Default::default()
                });
            }
            for row in rows {
                created.push(row.id);
                self.inner
                    .attached
                    .borrow_mut()
                    .extend(item_ids.iter().map(|item| (*item, row.id)));
                self.inner.rows.borrow_mut().push(row);
            }
            self.reload();
            Ok(created)
        }

        fn edit(&self, id: u64, f: impl FnOnce(&mut ResourceRow)) -> Result<(), String> {
            {
                let mut rows = self.inner.rows.borrow_mut();
                let row = rows
                    .iter_mut()
                    .find(|r| r.id == id)
                    .ok_or_else(|| format!("resource {id} is gone"))?;
                f(row);
            }
            self.reload();
            Ok(())
        }

        pub fn edit_text(&self, id: u64, title: &str, description: &str) -> Result<(), String> {
            self.edit(id, |r| {
                r.title = title.to_string();
                r.description = description.to_string();
            })
        }

        pub fn set_file_facts(&self, id: u64, facts: &FileFacts) -> Result<(), String> {
            self.edit(id, |r| {
                r.byte_size = facts.byte_size;
                r.content_hash = facts.content_hash.clone();
                r.width = facts.width;
                r.height = facts.height;
            })
        }

        pub fn relink(&self, id: u64, path: &Path) -> Result<(), String> {
            self.edit(id, |r| {
                r.location = path.to_string_lossy().into_owned();
                r.file_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                r.path = Some(path.to_path_buf());
                r.missing = !path.is_file();
            })
        }

        pub fn detach(&self, item_id: u64, id: u64) -> Result<(), String> {
            self.inner
                .attached
                .borrow_mut()
                .retain(|pair| *pair != (item_id, id));
            self.reload();
            Ok(())
        }

        pub fn remove(&self, ids: &[u64]) -> Result<(), String> {
            self.inner
                .rows
                .borrow_mut()
                .retain(|r| !ids.contains(&r.id));
            self.inner
                .attached
                .borrow_mut()
                .retain(|(_, id)| !ids.contains(id));
            self.reload();
            Ok(())
        }
    }

    fn title_of_file(name: &str) -> String {
        match name.rsplit_once('.') {
            Some((stem, _)) if !stem.trim().is_empty() => stem.trim().to_string(),
            _ => name.to_string(),
        }
    }
}

pub use imp::ResourcesListModel;
