//! Shared "read the open Work tree and serialise it" support for `save_work`
//! and the two `migrate_to_*` use cases. Each use case's UoW trait exposes the
//! same generated read methods, so a thin [`TreeReader`] impl per UoW lets one
//! [`gather`] do the ordered read + relationship hydration once.

use crate::skrib::{self, BinderWithItems, ItemWithContents, ShapeTag, SkribShape};
use anyhow::{Result, anyhow};
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{
    Binder, BinderItem, BinderTag, Content, DictWord, TrashInfo, Work, WorkInfo, WorkShape,
};
use common::long_operation::OperationProgress;
use common::types::EntityId;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

/// Generate a fresh, stable project identity string (UUID v4).
///
/// Used to mint a `Work.unique_id` for brand-new projects and to *heal* a load
/// whose source carries none (a legacy `.skrib` without
/// `t_project_unique_identifier`, or a pre-v2 bundle lacking the field). Legacy
/// ids that *are* present are preserved verbatim — this only fills the gaps.
pub(crate) fn new_unique_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// The read surface needed to serialise the Work subtree. Implemented for each
/// use case's `dyn …UnitOfWorkTrait` (the generated method names are identical).
pub trait TreeReader {
    fn all_work(&self) -> Result<Vec<Work>>;
    fn all_work_info(&self) -> Result<Vec<WorkInfo>>;
    fn all_trash_info(&self) -> Result<Vec<TrashInfo>>;
    fn work_rel(&self, id: &EntityId, field: &WorkRelationshipField) -> Result<Vec<EntityId>>;
    fn binder_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Binder>>>;
    fn binder_rel(&self, id: &EntityId, field: &BinderRelationshipField) -> Result<Vec<EntityId>>;
    fn item_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<BinderItem>>>;
    fn item_rel(&self, id: &EntityId, field: &BinderItemRelationshipField)
    -> Result<Vec<EntityId>>;
    fn tag_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<BinderTag>>>;
    fn dict_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<DictWord>>>;
    fn content_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Content>>>;
}

pub struct Gathered {
    pub work: Work,
    pub tags: Vec<BinderTag>,
    pub dict_words: Vec<DictWord>,
    pub trash_infos: Vec<TrashInfo>,
    pub binders: Vec<BinderWithItems>,
    pub work_info: Option<WorkInfo>,
}

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
    fn work_info_ids(&self) -> Result<Vec<EntityId>>;
    fn remove_works(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_binders(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_items(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_contents(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_tags(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_dicts(&self, ids: &[EntityId]) -> Result<()>;
    fn remove_trashes(&self, ids: &[EntityId]) -> Result<()>;
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
    c.remove_work_infos(&c.work_info_ids()?)?;
    c.remove_works(&c.work_ids()?)?;
    Ok(())
}

/// Read the single open Work, its tags/dict-words/trash, and every binder →
/// item → content in order, hydrating each entity's relationship id vectors
/// (which `get` does not populate). Honours `cancel`; reports `progress`.
pub fn gather<R: TreeReader + ?Sized>(
    reader: &R,
    progress: &(dyn Fn(OperationProgress) + Send),
    cancel: &AtomicBool,
) -> Result<Gathered> {
    let mut work = reader
        .all_work()?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("no open work"))?;
    let work_info = reader.all_work_info()?.into_iter().next();
    let work_id = work.id;

    work.tags = reader.work_rel(&work_id, &WorkRelationshipField::Tags)?;
    work.dict_words = reader.work_rel(&work_id, &WorkRelationshipField::DictWords)?;
    work.binders = reader.work_rel(&work_id, &WorkRelationshipField::Binders)?;

    let tags = fetch_multi(&work.tags, |ids| reader.tag_multi(ids))?;
    let dict_words = fetch_multi(&work.dict_words, |ids| reader.dict_multi(ids))?;
    let trash_infos = reader.all_trash_info()?;

    let binder_entities = fetch_multi(&work.binders, |ids| reader.binder_multi(ids))?;
    let count = binder_entities.len().max(1);
    let mut binders = Vec::with_capacity(binder_entities.len());
    for (idx, mut binder) in binder_entities.into_iter().enumerate() {
        if cancel.load(Ordering::Relaxed) {
            return Err(anyhow!("operation cancelled"));
        }
        let item_ids = reader.binder_rel(&binder.id, &BinderRelationshipField::BinderItems)?;
        binder.binder_items = item_ids.clone();

        let item_entities = fetch_multi(&item_ids, |ids| reader.item_multi(ids))?;
        let mut items = Vec::with_capacity(item_entities.len());
        for mut item in item_entities {
            item.contents = reader.item_rel(&item.id, &BinderItemRelationshipField::Contents)?;
            item.references =
                reader.item_rel(&item.id, &BinderItemRelationshipField::References)?;
            item.tags = reader.item_rel(&item.id, &BinderItemRelationshipField::Tags)?;
            let contents = fetch_multi(&item.contents, |ids| reader.content_multi(ids))?;
            items.push(ItemWithContents { item, contents });
        }
        binders.push(BinderWithItems { binder, items });
        progress(OperationProgress::new(
            10.0 + 80.0 * (idx as f32 + 1.0) / count as f32,
            Some("Reading project…".to_string()),
        ));
    }

    Ok(Gathered {
        work,
        tags,
        dict_words,
        trash_infos,
        binders,
        work_info,
    })
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
        &g.binders,
        shape_tag,
    );
    skrib::write_bundle(&target, shape, &bundle).map_err(|e| anyhow!("writing '{target}': {e}"))?;
    Ok(target)
}

fn fetch_multi<T>(
    ids: &[EntityId],
    get: impl FnOnce(&[EntityId]) -> Result<Vec<Option<T>>>,
) -> Result<Vec<T>> {
    let fetched = get(ids)?;
    let mut out = Vec::with_capacity(fetched.len());
    for (id, opt) in ids.iter().zip(fetched) {
        out.push(opt.ok_or_else(|| anyhow!("entity {id} vanished mid-read"))?);
    }
    Ok(out)
}
