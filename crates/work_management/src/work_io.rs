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

/// The write surface needed to clear ONE work from the store. Implemented per use
/// case's `dyn …UnitOfWorkTrait` (the generated method names are identical), so
/// [`close_current_work`] lives **once** and is reused by `close_work` AND inline
/// at the top of `load_work`/`new_work` — opening a work closes the current one
/// without one use case calling another.
///
/// Only 3 methods, not the store-wide `get_all_<Entity>()` sweep this trait used
/// to be (see git history): the generated per-entity repositories ALREADY
/// cascade-delete correctly through every STRONG relationship a `Work` owns
/// (`Work::remove_multi` → Binders → BinderItems → Contents, Tags, DictWords,
/// TrashInfos, Paces → Holidays/Milestones — confirmed by direct read of
/// `common/src/direct_access/{work,binder,binder_item,pace}/*_repository.rs`)
/// AND reconcile the external owner (`Root.works`) on removal — there is nothing
/// left for this trait to re-derive by hand. The ONE relationship `Work` does NOT
/// own is `WorkInfo` (`WorkInfo.work` is a WEAK `many_to_one` FROM `WorkInfo`, not
/// a strong relationship owned BY `Work` — qleany.yaml's `WorkInfo` entity), so a
/// closing Work's `WorkInfo` (and, through IT, `Search`/`ProgressSnapshot`, and
/// `System.work_infos`) has to be found and removed explicitly.
pub trait WorkCloser {
    /// Every `WorkInfo` whose `.work` back-pointer is `work_id` — a reverse
    /// relationship lookup (`WorkInfo → Work` is one-way), NOT a store-wide scan:
    /// implemented via `get_work_info_relationships_from_right_ids(&Work, &[work_id])`,
    /// which only returns `WorkInfo`s that actually reference this Work.
    fn work_info_ids_for_work(&self, work_id: EntityId) -> Result<Vec<EntityId>>;
    /// Cascades to `Search` + `ProgressSnapshot`s and reconciles `System.work_infos`.
    fn remove_work_infos(&self, ids: &[EntityId]) -> Result<()>;
    /// Cascades to Binders/Tags/DictWords/TrashInfos/Paces (and, transitively,
    /// their own children) and reconciles `Root.works`.
    fn remove_works(&self, ids: &[EntityId]) -> Result<()>;
}

/// Remove exactly `work_id`'s subtree from the in-memory store — its `WorkInfo`
/// (and everything hanging off it) first, then the `Work` itself (which cascades
/// through every entity it strongly owns) — leaving every OTHER open Work, plus
/// `Root`, `System` and the `RecentWork` list, untouched. `WorkInfo` must come
/// first: it is a weak, one-way referrer of `Work`, so removing the `Work` first
/// would leave an orphaned `WorkInfo` pointing at a now-gone id. A no-op when
/// `work_id` is not present (e.g. the very first load, or a redundant close).
pub fn close_current_work<C: WorkCloser + ?Sized>(c: &C, work_id: EntityId) -> Result<()> {
    c.remove_work_infos(&c.work_info_ids_for_work(work_id)?)?;
    c.remove_works(&[work_id])?;
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
