//! Read the open Work tree into one ordered, relationship-hydrated snapshot.
//!
//! Relocated out of `work_management::work_io` so it can be shared **without a
//! feature-to-feature dependency**: `save_work` / `save_as` / `backup_now` (in
//! `work_management`) and `export_work` (in `export_management`) each implement the thin
//! [`TreeReader`] over their own generated UoW trait, and one [`gather`] does the ordered
//! read + relationship hydration once. This is the same "shared crate, no feature
//! coupling" precedent as the rest of `skrib_format`.
//!
//! Export lists fewer entities than save (no `WorkInfo` / `TrashInfo` / `DictWord`), so
//! [`TreeReader::all_work_info`], [`TreeReader::all_trash_info`] and
//! [`TreeReader::dict_multi`] have **default no-op** bodies — an implementor that has no
//! such generated getter simply omits them.

use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Result, anyhow};
use common::direct_access::binder::BinderRelationshipField;
use common::direct_access::binder_item::BinderItemRelationshipField;
use common::direct_access::work::WorkRelationshipField;
use common::entities::{
    Binder, BinderItem, BinderTag, Content, DictWord, TrashInfo, Work, WorkInfo,
};
use common::long_operation::OperationProgress;
use common::types::EntityId;

use crate::{BinderWithItems, ItemWithContents};

/// The read surface needed to snapshot the Work subtree. Implemented for each use case's
/// `dyn …UnitOfWorkTrait` (the generated method names are identical).
///
/// The three defaulted methods cover entities that only *save* lists; an implementor that
/// does not read them (e.g. `export_work`) leaves them defaulted.
pub trait TreeReader {
    fn all_work(&self) -> Result<Vec<Work>>;
    fn all_work_info(&self) -> Result<Vec<WorkInfo>> {
        Ok(Vec::new())
    }
    fn all_trash_info(&self) -> Result<Vec<TrashInfo>> {
        Ok(Vec::new())
    }
    fn work_rel(&self, id: &EntityId, field: &WorkRelationshipField) -> Result<Vec<EntityId>>;
    fn binder_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Binder>>>;
    fn binder_rel(&self, id: &EntityId, field: &BinderRelationshipField) -> Result<Vec<EntityId>>;
    fn item_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<BinderItem>>>;
    fn item_rel(&self, id: &EntityId, field: &BinderItemRelationshipField)
    -> Result<Vec<EntityId>>;
    fn tag_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<BinderTag>>>;
    fn dict_multi(&self, _ids: &[EntityId]) -> Result<Vec<Option<DictWord>>> {
        Ok(Vec::new())
    }
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

/// Read the single open Work, its tags/dict-words/trash, and every binder → item →
/// content in order, hydrating each entity's relationship id vectors (which `get` does not
/// populate). Honours `cancel`; reports `progress`.
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
