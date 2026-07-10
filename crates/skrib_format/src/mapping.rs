//! Pure transforms between store entities, the on-disk [`WorkBundle`], and the
//! neutral [`LoadedWork`] graph the materialiser consumes. No I/O here.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use common::entities::{
    Binder, BinderItem, BinderTag, ChapterMode, Content, DictWord, TrashInfo, Work,
};
use skribisto_model::content_allowed;
use std::collections::BTreeMap;

use super::bundle::*;
use super::loaded::*;
use super::slug::{binder_dir_name, prose_file_name, prose_kind, prose_relpath};

fn fmt_dt(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

fn parse_dt(s: &str) -> Result<DateTime<Utc>> {
    Ok(DateTime::parse_from_rfc3339(s)
        .with_context(|| format!("parsing datetime '{s}'"))?
        .with_timezone(&Utc))
}

// ---------------------------------------------------------------------------
// store entities -> WorkBundle (save path)
// ---------------------------------------------------------------------------

/// Build a [`WorkBundle`] from already-fetched, ordered store data.
///
/// Content rows are filtered through `skribisto_model::content_allowed`, so an
/// invalid `(role, sub_role, content_role)` triple can never be written; valid
/// rows are split into inline title contents vs `.djot` prose blobs.
pub fn from_entities(
    work: &Work,
    tags: &[BinderTag],
    dict_words: &[DictWord],
    trash_infos: &[TrashInfo],
    binders: &[BinderWithItems],
    shape: ShapeTag,
) -> WorkBundle {
    let mut bundled_binders = Vec::with_capacity(binders.len());

    for (index, bwi) in binders.iter().enumerate() {
        let dir = binder_dir_name(index, &bwi.binder.name);
        let item_order: Vec<u64> = bwi.items.iter().map(|i| i.item.id).collect();

        let mut items = Vec::with_capacity(bwi.items.len());
        for iwc in &bwi.items {
            let item = &iwc.item;
            let mut inline_contents = Vec::new();
            let mut prose_refs = Vec::new();
            let mut prose = BTreeMap::new();

            for c in &iwc.contents {
                if !content_allowed(&item.role, &item.sub_role, &c.role) {
                    continue; // invalid for this item — never serialise it
                }
                match prose_kind(&c.role) {
                    None => inline_contents.push(InlineContent {
                        file_id: c.id,
                        created_at: fmt_dt(&c.created_at),
                        updated_at: fmt_dt(&c.updated_at),
                        activated: c.activated,
                        role: c.role.clone(),
                        text: c.data.clone(),
                    }),
                    Some(_) => {
                        let name = prose_file_name(c.id, &item.title, &c.role)
                            .expect("prose_kind matched");
                        prose_refs.push(ProseRef {
                            file_id: c.id,
                            created_at: fmt_dt(&c.created_at),
                            updated_at: fmt_dt(&c.updated_at),
                            activated: c.activated,
                            role: c.role.clone(),
                            path: prose_relpath(&dir, &name),
                        });
                        prose.insert(c.id, c.data.clone());
                    }
                }
            }

            items.push(BundledItem {
                item: BinderItemFile {
                    file_id: item.id,
                    created_at: fmt_dt(&item.created_at),
                    updated_at: fmt_dt(&item.updated_at),
                    title: item.title.clone(),
                    sub_title: item.sub_title.clone(),
                    role: item.role.clone(),
                    sub_role: item.sub_role.clone(),
                    label: item.label.clone(),
                    activated: item.activated,
                    is_favorite: item.is_favorite,
                    is_printable: item.is_printable,
                    indent: item.indent,
                    word_count_goal: item.word_count_goal,
                    char_count_goal: item.char_count_goal,
                    dict_language: item.dict_language.clone(),
                    inline_contents,
                    prose_refs,
                    reference_ids: item.references.clone(),
                    tag_ids: item.tags.clone(),
                },
                prose,
            });
        }

        bundled_binders.push(BundledBinder {
            binder: BinderFile {
                file_id: bwi.binder.id,
                created_at: fmt_dt(&bwi.binder.created_at),
                updated_at: fmt_dt(&bwi.binder.updated_at),
                name: bwi.binder.name.clone(),
                activated: bwi.binder.activated,
                item_order,
            },
            items,
        });
    }

    WorkBundle {
        manifest: ProjectManifest {
            format_version: FORMAT_VERSION,
            shape,
            work: WorkFile {
                file_id: work.id,
                created_at: fmt_dt(&work.created_at),
                updated_at: fmt_dt(&work.updated_at),
                title: work.title.clone(),
                author_name: work.author_name.clone(),
                dict_language: work.dict_language.clone(),
                tag_ids: work.tags.clone(),
                dict_word_ids: work.dict_words.clone(),
                unique_id: work.unique_id.clone(),
                chapter_flat: matches!(work.chapter_mode, ChapterMode::Flat),
            },
            binder_order: binders.iter().map(|b| b.binder.id).collect(),
        },
        tags: tags
            .iter()
            .map(|t| BinderTagFile {
                file_id: t.id,
                created_at: fmt_dt(&t.created_at),
                updated_at: fmt_dt(&t.updated_at),
                name: t.name.clone(),
                color: t.color.clone(),
                text_color: t.text_color.clone(),
            })
            .collect(),
        dict_words: dict_words
            .iter()
            .map(|w| DictWordFile {
                file_id: w.id,
                created_at: fmt_dt(&w.created_at),
                updated_at: fmt_dt(&w.updated_at),
                word: w.word.clone(),
            })
            .collect(),
        trash_infos: trash_infos
            .iter()
            .map(|ti| TrashInfoFile {
                file_id: ti.id,
                created_at: fmt_dt(&ti.created_at),
                updated_at: fmt_dt(&ti.updated_at),
                trashed_at: fmt_dt(&ti.trashed_at),
                origin_binder_id: ti.origin_binder_id,
                trashed_binder: ti.trashed_binder,
                trashed_binder_item: ti.trashed_binder_item,
            })
            .collect(),
        binders: bundled_binders,
    }
}

// ---------------------------------------------------------------------------
// WorkBundle -> neutral LoadedWork (load path)
// ---------------------------------------------------------------------------

/// Turn a parsed [`WorkBundle`] into the entity-typed [`LoadedWork`] graph the
/// materialiser consumes. Ids stay as file ids (remapped on materialise).
pub fn bundle_to_loaded(bundle: WorkBundle, absolute_path: &str) -> Result<LoadedWork> {
    let m = &bundle.manifest;

    let work = Work {
        id: m.work.file_id,
        created_at: parse_dt(&m.work.created_at)?,
        updated_at: parse_dt(&m.work.updated_at)?,
        title: m.work.title.clone(),
        author_name: m.work.author_name.clone(),
        dict_language: m.work.dict_language.clone(),
        // Empty for pre-v2 bundles; healed (freshly minted) in `materialize`.
        unique_id: m.work.unique_id.clone(),
        chapter_mode: if m.work.chapter_flat {
            ChapterMode::Flat
        } else {
            ChapterMode::Folder
        },
        ..Default::default()
    };

    let tags = bundle
        .tags
        .iter()
        .map(|t| {
            Ok(BinderTag {
                id: t.file_id,
                created_at: parse_dt(&t.created_at)?,
                updated_at: parse_dt(&t.updated_at)?,
                name: t.name.clone(),
                color: t.color.clone(),
                text_color: t.text_color.clone(),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let dict_words = bundle
        .dict_words
        .iter()
        .map(|w| {
            Ok(DictWord {
                id: w.file_id,
                created_at: parse_dt(&w.created_at)?,
                updated_at: parse_dt(&w.updated_at)?,
                word: w.word.clone(),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let mut references: Vec<(u64, u64)> = Vec::new();
    let mut loaded_binders = Vec::with_capacity(bundle.binders.len());

    for bb in &bundle.binders {
        let binder = Binder {
            id: bb.binder.file_id,
            created_at: parse_dt(&bb.binder.created_at)?,
            updated_at: parse_dt(&bb.binder.updated_at)?,
            name: bb.binder.name.clone(),
            activated: bb.binder.activated,
            ..Default::default()
        };

        let mut items = Vec::with_capacity(bb.items.len());
        for bi in &bb.items {
            let f = &bi.item;
            let mut contents: Vec<Content> = Vec::new();
            for ic in &f.inline_contents {
                contents.push(Content {
                    id: ic.file_id,
                    created_at: parse_dt(&ic.created_at)?,
                    updated_at: parse_dt(&ic.updated_at)?,
                    activated: ic.activated,
                    role: ic.role.clone(),
                    data: ic.text.clone(),
                });
            }
            for pr in &f.prose_refs {
                let data = bi.prose.get(&pr.file_id).cloned().ok_or_else(|| {
                    anyhow::anyhow!(
                        "missing prose blob for content {} ({})",
                        pr.file_id,
                        pr.path
                    )
                })?;
                contents.push(Content {
                    id: pr.file_id,
                    created_at: parse_dt(&pr.created_at)?,
                    updated_at: parse_dt(&pr.updated_at)?,
                    activated: pr.activated,
                    role: pr.role.clone(),
                    data,
                });
            }

            for dst in &f.reference_ids {
                references.push((f.file_id, *dst));
            }

            items.push(LoadedItem {
                item: BinderItem {
                    id: f.file_id,
                    created_at: parse_dt(&f.created_at)?,
                    updated_at: parse_dt(&f.updated_at)?,
                    title: f.title.clone(),
                    sub_title: f.sub_title.clone(),
                    role: f.role.clone(),
                    sub_role: f.sub_role.clone(),
                    label: f.label.clone(),
                    activated: f.activated,
                    is_favorite: f.is_favorite,
                    is_printable: f.is_printable,
                    indent: f.indent,
                    word_count_goal: f.word_count_goal,
                    char_count_goal: f.char_count_goal,
                    dict_language: f.dict_language.clone(),
                    ..Default::default()
                },
                contents,
                tag_ids: f.tag_ids.clone(),
            });
        }

        loaded_binders.push(LoadedBinder { binder, items });
    }

    let trash_infos = bundle
        .trash_infos
        .iter()
        .map(|ti| {
            Ok(LoadedTrash {
                created_at: parse_dt(&ti.created_at)?,
                updated_at: parse_dt(&ti.updated_at)?,
                trashed_at: parse_dt(&ti.trashed_at)?,
                origin_binder_id: ti.origin_binder_id,
                trashed_binder: ti.trashed_binder,
                trashed_binder_item: ti.trashed_binder_item,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(LoadedWork {
        work,
        tags,
        dict_words,
        binders: loaded_binders,
        trash_infos,
        references,
        absolute_path: absolute_path.to_string(),
    })
}
