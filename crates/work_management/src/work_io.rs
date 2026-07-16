// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Save-side helpers for the open Work tree.
//!
//! The ordered *read* — [`TreeReader`] / [`Gathered`] / [`gather`] — now lives in
//! `skrib_format::tree_read` so `export_management` can share it without a
//! feature-to-feature dependency; it is re-exported here so `save_work` / `save_as` /
//! `backup_now` are unchanged. The *write* half ([`resolve_target`],
//! [`serialize_and_write`]) and the store-clearing [`WorkCloser`] / [`close_current_work`]
//! (used by load / new / close) stay here — they are `.skrib`-specific and save-only.

use std::path::Path;

use anyhow::{Result, anyhow};
use common::entities::{WorkInfo, WorkShape};
use common::types::EntityId;
use skrib_format::{self as skrib, ShapeTag, SkribShape};

// Relocated to `skrib_format::tree_read`; re-exported so existing callers are unchanged.
pub use skrib_format::tree_read::{Gathered, TreeReader, gather};

/// The write surface needed to clear the open work from the store: list every id
/// of each entity type and remove them. Implemented per use case's
/// `dyn …UnitOfWorkTrait` (the generated `get_all_*` / `remove_*_multi` names are
/// identical), so [`close_current_work`] lives **once** and is reused by
/// `close_work` AND inline at the top of `load_work` — opening a work closes the
/// current one without one use case calling another.
pub trait WorkCloser {
    fn work_ids(&self) -> Result<Vec<EntityId>>;
    fn binder_ids(&self) -> Result<Vec<EntityId>>;
    fn item_ids(&self) -> Result<Vec<EntityId>>;
    fn content_ids(&self) -> Result<Vec<EntityId>>;
    fn tag_ids(&self) -> Result<Vec<EntityId>>;
    fn dict_ids(&self) -> Result<Vec<EntityId>>;
    fn trash_ids(&self) -> Result<Vec<EntityId>>;
    fn pace_ids(&self) -> Result<Vec<EntityId>>;
    fn holiday_ids(&self) -> Result<Vec<EntityId>>;
    fn milestone_ids(&self) -> Result<Vec<EntityId>>;
    fn progress_snapshot_ids(&self) -> Result<Vec<EntityId>>;
    fn work_info_ids(&self) -> Result<Vec<EntityId>>;
    fn remove_works(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_binders(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_items(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_contents(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_tags(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_dicts(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_trashes(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_paces(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_holidays(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_milestones(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_progress_snapshots(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_work_infos(&self, ids: &[EntityId]) -> Result<()>;
}

/// Clear the open work from the in-memory store — every entity under `Root→Work`
/// plus the System-side `WorkInfo`/`TrashInfo` — leaving `Root`, `System` and the
/// `RecentWork` list intact. `remove_multi` cleans each parent's backward
/// junction, so no dangling relationship remains. Children are removed before
/// parents. A no-op when the store is empty (the first load).
pub fn close_current_work<C: WorkCloser + ?Sized>(c: &C) -> Result<()> {
    c.remove_contents(&c.content_ids()?)?;
    c.remove_items(&c.item_ids()?)?;
    c.remove_binders(&c.binder_ids()?)?;
    c.remove_tags(&c.tag_ids()?)?;
    c.remove_dicts(&c.dict_ids()?)?;
    c.remove_trashes(&c.trash_ids()?)?;
    // Pace children before Pace before Work (children-before-parent).
    c.remove_milestones(&c.milestone_ids()?)?;
    c.remove_holidays(&c.holiday_ids()?)?;
    c.remove_paces(&c.pace_ids()?)?;
    // ProgressSnapshots hang off WorkInfo — remove them before it.
    c.remove_progress_snapshots(&c.progress_snapshot_ids()?)?;
    c.remove_work_infos(&c.work_info_ids()?)?;
    c.remove_works(&c.work_ids()?)?;
    Ok(())
}

/// Decide the output path + shape. `forced` pins the shape (the migrate cases);
/// otherwise an existing new-format target wins, then the recorded WorkInfo
/// shape (a legacy target falls through → migration-on-save), default Zip.
pub fn resolve_target(
    dto_path: &str,
    work_info: Option<&WorkInfo>,
    forced: Option<SkribShape>,
) -> Result<(String, SkribShape, ShapeTag)> {
    let target = if !dto_path.is_empty() {
        dto_path.to_string()
    } else {
        work_info
            .and_then(|wi| wi.file_name.clone())
            .ok_or_else(|| anyhow!("no target path to write to"))?
    };

    let shape = match forced {
        Some(s) => s,
        None => {
            let existing = if Path::new(&target).exists() {
                skrib::detect_shape(&target).ok()
            } else {
                None
            };
            match existing {
                Some(SkribShape::ExplodedFolder) => SkribShape::ExplodedFolder,
                Some(SkribShape::ZipFile) => SkribShape::ZipFile,
                _ => match work_info.map(|wi| wi.shape.clone()) {
                    Some(WorkShape::Folder) => SkribShape::ExplodedFolder,
                    _ => SkribShape::ZipFile,
                },
            }
        }
    };
    let tag = match shape {
        SkribShape::ExplodedFolder => ShapeTag::Folder,
        _ => ShapeTag::Zip,
    };
    Ok((target, shape, tag))
}

/// Serialise `g` and write it to `target` in `shape`. Returns `target`.
pub fn serialize_and_write(
    g: &Gathered,
    target: String,
    shape: SkribShape,
    shape_tag: ShapeTag,
) -> Result<String> {
    let bundle = skrib::from_entities(
        &g.work,
        &g.tags,
        &g.dict_words,
        &g.trash_infos,
        &g.paces,
        &g.progress_snapshots,
        &g.binders,
        shape_tag,
    );
    skrib::write_bundle(&target, shape, &bundle).map_err(|e| anyhow!("writing '{target}': {e}"))?;
    Ok(target)
}
