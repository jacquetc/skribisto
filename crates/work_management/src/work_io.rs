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

use anyhow::{Context, Result, anyhow};
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
/// TextReplacementRules, NoteTemplates, SmartPunctuation, TrashInfos,
/// Paces → Holidays/Milestones, Comments → CommentReplies — confirmed by direct
/// read of
/// `common/src/direct_access/{work,binder,binder_item,pace,comment}/*_repository.rs`)
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
    /// Cascades to Binders/Tags/DictWords/TextReplacementRules/SmartPunctuation/
    /// TrashInfos/Paces/Comments (and, transitively, their own children) and
    /// reconciles `Root.works`.
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
/// Read the bytes of every gathered asset out of the project's media directory.
///
/// An asset whose file is missing is *skipped*, not an error. That is the whole
/// posture of this path: an image the writer can no longer see is a smaller
/// failure than a save that refuses to run, and `from_entities` drops the row to
/// match so the bundle never carries a dangling reference. The lost row is
/// recoverable — the prose still names the hash, so restoring the file restores
/// the picture.
pub fn read_asset_bytes(
    assets: &[common::entities::Asset],
    media_dir: &std::path::Path,
) -> std::collections::BTreeMap<String, Vec<u8>> {
    let mut out = std::collections::BTreeMap::new();
    for a in assets {
        let ext = skrib::media::extension_for(&a.mime_type);
        let path = media_dir.join(format!("{}.{ext}", a.content_hash));
        if let Ok(bytes) = std::fs::read(&path) {
            out.insert(a.content_hash.clone(), bytes);
        }
    }
    out
}

/// Write a loaded bundle's asset bytes into the project's media directory.
///
/// A no-op for an exploded-folder project, where the media directory *is* the
/// bundle's own `assets/` — `write_if_changed` would compare each file against
/// itself. Only a zip needs its images extracted to somewhere the editor can
/// read them back.
///
/// Failing here fails the load: an image the writer can see but the next save
/// cannot find would be written out of the project silently, which is the one
/// outcome worth refusing to open over.
pub fn write_asset_bytes(bundle: &skrib::WorkBundle, media_dir: &std::path::Path) -> Result<()> {
    if bundle.assets.is_empty() {
        return Ok(());
    }
    std::fs::create_dir_all(media_dir)
        .with_context(|| format!("creating media dir {}", media_dir.display()))?;
    for a in &bundle.assets {
        let ext = skrib::media::extension_for(&a.mime_type);
        let path = media_dir.join(format!("{}.{ext}", a.content_hash));
        // Content-addressed: a file already there under this name has these
        // bytes, so re-writing it would be pure cost.
        if path.exists() {
            continue;
        }
        let bytes = bundle
            .asset_bytes
            .get(&a.content_hash)
            .ok_or_else(|| anyhow!("bundle lists asset {} with no bytes", a.file_name))?;
        std::fs::write(&path, bytes)
            .with_context(|| format!("writing media file {}", path.display()))?;
    }
    Ok(())
}

/// What a write does with the project's history log.
///
/// The log is the one part of a bundle that does not come from the store, so
/// `from_entities` cannot produce it and every write path has to say what it
/// wants — see [`skrib_format::history`].
pub enum HistoryAction {
    /// Carry the log at `source` through unchanged. For `save_as` (a copy of the
    /// same state, under a new name) and `backup_now` (a snapshot of it): neither
    /// is a new edit, so neither may stamp a new entry — but both must preserve
    /// what is there, or a restore would come back with the history erased.
    Carry { source: String },
    /// Carry the log at `source` forward **and record this save into it**, then
    /// thin. Only `save_work` does this: it is the only path that represents the
    /// writer having actually changed something.
    Record {
        source: String,
        policy: skrib::retention::RetentionPolicy,
        min_keep: u32,
    },
}

impl HistoryAction {
    /// The bundle this write derives from. Both variants have one, and the
    /// carry-through read needs it without caring which variant it is.
    pub fn source(&self) -> &str {
        match self {
            Self::Carry { source } | Self::Record { source, .. } => source,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// "Every `BinderItem` uid this project holds" — the prune list a
// [`crate::lifecycle`] listener works from, since no cascade ever reaches one.
//
// **Both forms live here, side by side, on purpose.** A project's item tree is
// reachable through two different types — `Gathered` on the way out and
// `LoadedWork` on the way in — so one function cannot serve both without a trait
// neither shape otherwise needs. Co-locating them is what stops the pair
// drifting: changing what "live" means (excluding trashed items, say, or adding
// a new child collection) has to be done twice, and a reader who finds one
// cannot miss the other.
//
// There is a third answer, deliberately not here: `new_work_uc` collects uids as
// it *creates* the items, because a brand-new project has no tree to walk yet.
// ─────────────────────────────────────────────────────────────────────────────

/// From the ordered read a save is built from.
pub(crate) fn live_binder_item_uids(g: &Gathered) -> Vec<uuid::Uuid> {
    g.binders
        .iter()
        .flat_map(|b| b.items.iter().map(|i| i.item.uid))
        .collect()
}

/// From the neutral graph a load materialises. These are the uids the bundle
/// carries, which the materialiser preserves verbatim.
pub(crate) fn loaded_binder_item_uids(loaded: &skrib_format::LoadedWork) -> Vec<uuid::Uuid> {
    loaded
        .binders
        .iter()
        .flat_map(|b| b.items.iter().map(|i| i.item.uid))
        .collect()
}

pub fn serialize_and_write(
    g: &Gathered,
    target: String,
    shape: SkribShape,
    shape_tag: ShapeTag,
    media_dir: &std::path::Path,
    history: HistoryAction,
    kind: crate::lifecycle::SaveKind,
) -> Result<String> {
    let mut bundle = skrib::from_entities(
        &g.work,
        &g.tags,
        &g.dict_words,
        &g.text_replacement_rules,
        &g.note_templates,
        &g.assets,
        read_asset_bytes(&g.assets, media_dir),
        g.smart_punctuation.as_ref(),
        &g.trash_infos,
        &g.paces,
        &g.progress_snapshots,
        &g.comments,
        &g.footnotes,
        &g.binders,
        shape_tag,
    );
    // Asked here, merged below. The gap is only an economy: `collect` clones the
    // bundle to fingerprint it, and doing that before `carry::load` means not
    // copying a project's unmodelled files for nothing. Correctness does not
    // rest on it — `manuscript_fingerprint` drops `carried` wherever it is asked,
    // which is what stops an extension's own bytes from looking like an edit to
    // the book.
    let contributed = crate::bundle_contributors::collect(&bundle, &g.work.unique_id, kind);

    // Files the format does not model travel with the project, on every write
    // path. Read from the *source* bundle, never the target: `save_as` and
    // `backup_now` must bring the original's unmodelled files with them, and
    // reading the target would carry a backup's own stale contents instead.
    //
    // Both `HistoryAction` variants name the same source for the same reason,
    // so this is lifted out of the match rather than repeated inside it — one
    // write path forgetting the call is how a writer loses data silently.
    bundle.carried = skrib::carry::load(history.source());

    // The contributors' files then land **over** the on-disk read. That
    // direction is the whole point of the hook and is the only correct one:
    // what a contributor holds in memory is current, what `carry::load` found
    // on disk is whatever the last save left there. Merging disk second would
    // write the stale copy back over every live change, which is the exact bug
    // this hook exists to prevent.
    for (path, bytes) in contributed {
        bundle.carried.insert(path, skrib::CarriedFile::new(bytes));
    }

    match history {
        HistoryAction::Carry { source } => {
            bundle.history = skrib::history::load(&source);
        }
        HistoryAction::Record {
            source,
            policy,
            min_keep,
        } => {
            let now = chrono::Utc::now();
            bundle.history = skrib::history::load(&source);
            skrib::history::record(&mut bundle, now);
            skrib::history::thin(&mut bundle.history, &policy, min_keep, now);
        }
    }
    skrib::write_bundle(&target, shape, &bundle).map_err(|e| anyhow!("writing '{target}': {e}"))?;
    // The backend seam's "a write landed" hook, beside the contributor call it
    // mirrors: same function, same `unique_id`, same once-per-write cardinality
    // that both get here for free.
    if crate::lifecycle::has_listeners() {
        crate::lifecycle::notify(crate::lifecycle::LifecycleEvent::Saved {
            unique_id: g.work.unique_id.clone(),
            path: target.clone(),
            kind,
            live_binder_item_uids: live_binder_item_uids(g),
        });
    }
    Ok(target)
}
