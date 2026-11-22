// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Custom implementation: create stored files, linked files and web links in ONE undoable
// action, attached to the items the writer dropped them on.
//
// One action because the writer did one thing: dropping five files on a scene, or
// importing a Scrivener research folder, is one Ctrl+Z, not five creations and five
// attachments. The bytes of a stored file are already in the project's resources
// directory (`store_resource_files` put them there, under the uid passed back here), so
// this touches only rows.
//
// Undo strategy: a Work-scoped snapshot/restore, as `import_tags` does. The Work's
// subtree holds the new rows, `Work.resources` and every item's `resources` junction, so
// one restore takes all three back. The stored files stay on disk until the project is
// next opened (see `skrib_format::media::prune_resources_dir`), which is what lets Redo
// bring a resource back with its bytes.
use crate::AddResourcesDto;
use crate::AddResourcesResultDto;
use anyhow::{Result, anyhow};
use common::database::CommandUnitOfWork;
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{Binder, Resource, ResourceKind, Work};
use common::snapshot::EntityTreeSnapshot;
use common::types::EntityId;
use std::collections::HashSet;

pub trait AddResourcesUnitOfWorkFactoryTrait: Send + Sync {
    fn create(&self) -> Box<dyn AddResourcesUnitOfWorkTrait>;
}

// The same macro set must appear on the impl block in
// ../units_of_work/add_resources_uow.rs.
#[macros::uow_action(entity = "Work", action = "GetAll")]
#[macros::uow_action(entity = "Work", action = "GetRelationship")]
#[macros::uow_action(entity = "Work", action = "SetRelationship")]
#[macros::uow_action(entity = "Work", action = "Snapshot")]
#[macros::uow_action(entity = "Work", action = "Restore")]
#[macros::uow_action(entity = "Binder", action = "GetMulti")]
#[macros::uow_action(entity = "Binder", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "GetRelationship")]
#[macros::uow_action(entity = "BinderItem", action = "SetRelationship")]
#[macros::uow_action(entity = "Resource", action = "CreateOrphan")]
pub trait AddResourcesUnitOfWorkTrait: CommandUnitOfWork {
    /// Read the prose a restore must not revert: see [`common::prose_keep`].
    fn keep_prose(
        &self,
        target: &common::snapshot::EntityTreeSnapshot,
        other: &common::snapshot::EntityTreeSnapshot,
    ) -> Result<common::prose_keep::KeptProse>;
    /// Write that prose back after the restore, in the same transaction.
    fn put_back_prose(&self, kept: common::prose_keep::KeptProse) -> Result<()>;

    fn publish_add_resources_event(&self, ids: Vec<EntityId>, data: Option<String>);
}

pub struct AddResourcesUseCase {
    uow_factory: Box<dyn AddResourcesUnitOfWorkFactoryTrait>,
    snap_before: Option<EntityTreeSnapshot>,
    snap_after: Option<EntityTreeSnapshot>,
}

/// The schemes a web link may have. The same as a link in the text may open
/// (`teksilo_ui::shared::external_link`), less `mailto`: an address is not a page to
/// file. Refused here rather than only at opening, so the project never holds one.
const WEB_SCHEMES: [&str; 2] = ["http", "https"];

/// Whether `url` is a web address this use case stores.
pub fn is_web_url(url: &str) -> bool {
    let url = url.trim();
    url.split_once("://").is_some_and(|(scheme, rest)| {
        WEB_SCHEMES.iter().any(|s| s.eq_ignore_ascii_case(scheme))
            && !host(rest).is_empty()
            && !has_userinfo(rest)
    })
}

/// Whether the part after `scheme://` carries a user name or password before its host.
/// Refused rather than stored: a pasted `https://user:token@host/…` would keep the token
/// in a project file that travels, and show it in a search snippet.
fn has_userinfo(after_scheme: &str) -> bool {
    let end = after_scheme
        .find(['/', '?', '#'])
        .unwrap_or(after_scheme.len());
    after_scheme[..end].contains('@')
}

/// The host of the part of a URL after `scheme://`: up to the first `/`, `?` or `#`,
/// without credentials, a port or a leading `www.`.
fn host(after_scheme: &str) -> &str {
    let end = after_scheme
        .find(['/', '?', '#'])
        .unwrap_or(after_scheme.len());
    let authority = &after_scheme[..end];
    let authority = authority.rsplit('@').next().unwrap_or(authority);
    let authority = authority.split(':').next().unwrap_or(authority);
    authority.strip_prefix("www.").unwrap_or(authority)
}

/// A web link's title when the writer gave none: its host, which is what they would
/// recognise it by in a list.
fn title_of_url(url: &str) -> String {
    url.trim()
        .split_once("://")
        .map(|(_, rest)| host(rest).to_string())
        .unwrap_or_default()
}

/// A file's title when the writer gave none: its name without the extension.
fn title_of_file(file_name: &str) -> String {
    match file_name.rsplit_once('.') {
        Some((stem, _)) if !stem.trim().is_empty() => stem.trim().to_string(),
        _ => file_name.trim().to_string(),
    }
}

impl AddResourcesUseCase {
    pub fn new(uow_factory: Box<dyn AddResourcesUnitOfWorkFactoryTrait>) -> Self {
        AddResourcesUseCase {
            uow_factory,
            snap_before: None,
            snap_after: None,
        }
    }

    pub fn execute(&mut self, dto: &AddResourcesDto) -> Result<AddResourcesResultDto> {
        // The stored-file lists are one table transposed into columns (a DTO carries only
        // primitives), as are the two web-link lists. A length mismatch means the caller
        // built them inconsistently, and would otherwise pair a title with the wrong file.
        let n = dto.stored_uids.len();
        let columns = [
            dto.stored_titles.len(),
            dto.stored_file_names.len(),
            dto.stored_mime_types.len(),
            dto.stored_byte_sizes.len(),
            dto.stored_content_hashes.len(),
            dto.stored_widths.len(),
            dto.stored_heights.len(),
        ];
        if columns.iter().any(|&c| c != n) {
            return Err(anyhow!(
                "add_resources: stored-file columns differ in length ({n} uids, {columns:?})"
            ));
        }
        if dto.web_titles.len() != dto.web_urls.len() {
            return Err(anyhow!(
                "add_resources: {} web titles for {} addresses",
                dto.web_titles.len(),
                dto.web_urls.len()
            ));
        }
        if n + dto.linked_paths.len() + dto.web_urls.len() == 0 {
            return Err(anyhow!("add_resources: nothing to add"));
        }
        if let Some(bad) = dto.web_urls.iter().find(|u| !is_web_url(u)) {
            return Err(anyhow!("add_resources: '{bad}' is not a web address"));
        }
        if let Some(bad) = dto
            .linked_paths
            .iter()
            .find(|p| !std::path::Path::new(p).is_absolute())
        {
            return Err(anyhow!(
                "add_resources: linked path '{bad}' is not absolute"
            ));
        }
        let mut stored_uids = Vec::with_capacity(n);
        for raw in &dto.stored_uids {
            let uid = uuid::Uuid::parse_str(raw)
                .map_err(|e| anyhow!("add_resources: bad uid '{raw}': {e}"))?;
            if uid.is_nil() {
                return Err(anyhow!("add_resources: a stored file needs its own uid"));
            }
            stored_uids.push(uid);
        }

        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;

        // Into the caller-named Work only — several may be open at once.
        let work_id = dto.work_id as EntityId;
        uow.get_all_work()?
            .into_iter()
            .find(|w| w.id == work_id)
            .ok_or_else(|| anyhow!("work {work_id} is not open"))?;

        // Every item named must be this Work's: an id from another open project would
        // otherwise hold a link into this one.
        let mut own_items: HashSet<EntityId> = HashSet::new();
        let binder_ids = uow.get_work_relationship(&work_id, &WorkRelationshipField::Binders)?;
        for binder in uow.get_binder_multi(&binder_ids)?.into_iter().flatten() {
            own_items.extend(
                uow.get_binder_relationship(&binder.id, &BinderRelationshipField::BinderItems)?,
            );
        }
        let mut item_ids: Vec<EntityId> = Vec::new();
        for &id in &dto.item_ids {
            let id = id as EntityId;
            if !own_items.contains(&id) {
                return Err(anyhow!("add_resources: item {id} is not in work {work_id}"));
            }
            if !item_ids.contains(&id) {
                item_ids.push(id);
            }
        }

        let snap_before = uow.snapshot_work(&[work_id])?;
        let now = chrono::Utc::now();
        let base = Resource {
            created_at: now,
            updated_at: now,
            ..Default::default()
        };

        let mut created_ids: Vec<EntityId> = Vec::new();
        for i in 0..n {
            let file_name = dto.stored_file_names[i].clone();
            let title = match dto.stored_titles[i].trim() {
                "" => title_of_file(&file_name),
                t => t.to_string(),
            };
            let created = uow.create_orphan_resource(&Resource {
                uid: stored_uids[i],
                title,
                kind: ResourceKind::StoredFile,
                file_name,
                mime_type: dto.stored_mime_types[i].clone(),
                byte_size: dto.stored_byte_sizes[i],
                content_hash: dto.stored_content_hashes[i].clone(),
                width: dto.stored_widths[i],
                height: dto.stored_heights[i],
                ..base.clone()
            })?;
            created_ids.push(created.id);
        }
        for path in &dto.linked_paths {
            let file_name = std::path::Path::new(path)
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let created = uow.create_orphan_resource(&Resource {
                uid: common::uid::new_uid(),
                title: title_of_file(&file_name),
                kind: ResourceKind::LinkedFile,
                location: path.clone(),
                file_name,
                ..base.clone()
            })?;
            created_ids.push(created.id);
        }
        for (url, title) in dto.web_urls.iter().zip(&dto.web_titles) {
            let url = url.trim().to_string();
            let title = match title.trim() {
                "" => title_of_url(&url),
                t => t.to_string(),
            };
            let created = uow.create_orphan_resource(&Resource {
                uid: common::uid::new_uid(),
                title,
                kind: ResourceKind::WebLink,
                location: url,
                ..base.clone()
            })?;
            created_ids.push(created.id);
        }

        let mut work_resources =
            uow.get_work_relationship(&work_id, &WorkRelationshipField::Resources)?;
        work_resources.extend(created_ids.iter().copied());
        uow.set_work_relationship(&work_id, &WorkRelationshipField::Resources, &work_resources)?;
        for item_id in &item_ids {
            let mut attached =
                uow.get_binder_item_relationship(item_id, &BinderItemRelationshipField::Resources)?;
            attached.extend(created_ids.iter().copied());
            uow.set_binder_item_relationship(
                item_id,
                &BinderItemRelationshipField::Resources,
                &attached,
            )?;
        }

        let snap_after = uow.snapshot_work(&[work_id])?;
        uow.commit()?;
        uow.publish_add_resources_event(created_ids.clone(), None);

        self.snap_before = Some(snap_before);
        self.snap_after = Some(snap_after);
        Ok(AddResourcesResultDto { created_ids })
    }
}

use common::undo_redo::UndoRedoCommand;
use std::any::Any;
impl UndoRedoCommand for AddResourcesUseCase {
    fn undo(&mut self) -> Result<()> {
        let snap = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("add_resources: nothing to undo"))?;
        let other = self
            .snap_after
            .as_ref()
            .ok_or_else(|| anyhow!("add_resources: the other end of this command is missing"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let kept = uow.keep_prose(snap, other)?;
        uow.restore_work(snap)?;
        uow.put_back_prose(kept)?;
        uow.commit()?;
        Ok(())
    }

    fn redo(&mut self) -> Result<()> {
        let snap = self
            .snap_after
            .as_ref()
            .ok_or_else(|| anyhow!("add_resources: nothing to redo"))?;
        let other = self
            .snap_before
            .as_ref()
            .ok_or_else(|| anyhow!("add_resources: the other end of this command is missing"))?;
        let mut uow = self.uow_factory.create();
        uow.begin_transaction()?;
        let kept = uow.keep_prose(snap, other)?;
        uow.restore_work(snap)?;
        uow.put_back_prose(kept)?;
        uow.commit()?;
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_web_page_is_a_web_link() {
        assert!(is_web_url("https://example.org/tanneries"));
        assert!(is_web_url("  HTTP://example.org  "));
        assert!(!is_web_url("file:///etc/passwd"));
        assert!(!is_web_url("javascript:alert(1)"));
        assert!(!is_web_url("mailto:someone@example.org"));
        assert!(!is_web_url("https://"));
        assert!(!is_web_url("https://user:token@example.org/x"));
        assert!(is_web_url("https://example.org/a@b"));
    }

    #[test]
    fn a_web_link_with_no_title_is_named_by_its_host() {
        assert_eq!(
            title_of_url("https://www.example.org/a/b?c#d"),
            "example.org"
        );
        assert_eq!(
            title_of_url("https://user:pw@archive.org:8080/x"),
            "archive.org"
        );
    }

    #[test]
    fn a_file_with_no_title_is_named_without_its_extension() {
        assert_eq!(title_of_file("Village map.pdf"), "Village map");
        assert_eq!(title_of_file(".profile"), ".profile");
        assert_eq!(title_of_file("README"), "README");
    }
}
