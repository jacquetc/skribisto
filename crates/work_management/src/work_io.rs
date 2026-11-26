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
/// Where an open project's binary files are on disk: its pictures (the media directory)
/// and its stored resources. Both resolved by `skrib_format::media` from the project's
/// path and `unique_id`, under the app's media root: inside an exploded folder, or a
/// working folder under the app's data for a zip.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectDirs {
    pub media: std::path::PathBuf,
    pub resources: std::path::PathBuf,
    /// The folder a symbolic link is judged from (see
    /// `skrib_format::media::passes_through_link`): an exploded folder's own folder, so
    /// that its `assets/` or `resources/` made a link counts as one, and for a zip the
    /// app's data root, whose working folders are the app's own.
    pub base: std::path::PathBuf,
}

impl ProjectDirs {
    /// The directories of the project at `project_path`.
    pub fn of(
        project_path: &std::path::Path,
        unique_id: &str,
        media_root: &std::path::Path,
    ) -> Self {
        Self {
            media: skrib::media::media_dir(project_path, unique_id, media_root, unique_id),
            resources: skrib::media::resources_dir(project_path, unique_id, media_root, unique_id),
            base: skrib::media::folder_project_root(project_path)
                .unwrap_or_else(|| media_root.to_path_buf()),
        }
    }
}

/// Point at the file of every stored resource in the project's resources directory,
/// keyed by the resource's `uid` as `WorkBundle::resource_blobs` is.
///
/// A missing file is skipped, as a missing picture is, but its resource is not:
/// `from_entities` still writes the row, so the card survives and shows its file as
/// missing rather than vanishing with it. So is one reached through a symbolic link, and
/// said on stderr: its bytes are wherever the link points, not the project's, and the
/// archive or backup this feeds would carry them on (see
/// `skrib_format::media::passes_through_link`).
pub fn read_resource_blobs(
    resources: &[common::entities::Resource],
    dirs: &ProjectDirs,
) -> std::collections::BTreeMap<String, skrib::Blob> {
    let mut out = std::collections::BTreeMap::new();
    for r in resources {
        if r.kind != common::entities::ResourceKind::StoredFile {
            continue;
        }
        let path = skrib::media::resource_file(&dirs.resources, r.uid, &r.file_name);
        if own_file(&dirs.base, &path) {
            out.insert(r.uid.hyphenated().to_string(), skrib::Blob::file(path));
        }
    }
    out
}

/// Whether `path`, under `dir` (a [`ProjectDirs::base`]), is a file of the project's
/// own: there, and not reached through a symbolic link. A linked one is reported.
fn own_file(dir: &std::path::Path, path: &std::path::Path) -> bool {
    if skrib::media::passes_through_link(dir, path) {
        eprintln!(
            "skrib: leaving out {}: it is a link, not a file of the project",
            path.display()
        );
        return false;
    }
    path.is_file()
}

/// Place a loaded bundle's stored resources into the project's resources directory,
/// and remove what no resource names any more (see `media::prune_resources_dir`).
///
/// For an exploded folder every blob already is the file it would be placed on, so
/// only the prune does anything, except for a resource whose uid the load gave anew
/// (`WorkBundle::heal_resource_uids`): its blob is the file in the folder of its old
/// uid, copied here into the folder of its new one. A resource whose file the bundle
/// lacks is left as it is: missing, shown so, never a reason to refuse the project.
///
/// Every file is placed before the prune runs. The prune keeps only the folders of
/// today's uids, so run first it would delete the old uid's folder, and with it the
/// only copy of the file the copy below reads.
///
/// Rows that shared one file each get a file of their own, never two names for one
/// file (a hard link), so an application editing one does not change the other.
pub fn write_resource_blobs(
    bundle: &skrib::WorkBundle,
    resources_dir: &std::path::Path,
) -> Result<()> {
    // The files already placed from, in this pass.
    let mut placed_from: std::collections::BTreeSet<std::path::PathBuf> = Default::default();
    for r in &bundle.resources {
        let Some(blob) = bundle.resource_blobs.get(&r.uid.hyphenated().to_string()) else {
            continue;
        };
        let path = skrib::media::resource_file(resources_dir, r.uid, &r.file_name);
        // A folder project's blob already is the file in this folder, and nothing is
        // written: it may be under the name an earlier build gave it, which the prune
        // below renames to today's (see `media::prune_resources_dir`).
        if blob.path().is_some_and(|src| src.parent() == path.parent()) {
            continue;
        }
        let from_this_project = blob
            .path()
            .is_some_and(|src| src.starts_with(resources_dir));
        // From another uid's folder of this project: a row whose uid the load gave anew.
        // The uid is derived from the row, the same at every open until a save writes it
        // (see `heal_resource_uids`), so a file already in its folder is the copy an
        // earlier open made, perhaps edited since: it is the row's own, and kept.
        if from_this_project && path.is_file() {
            continue;
        }
        // From a zip's extraction, thrown away after the load: a link is safe, for the
        // first row placed from that file. From another uid's folder of this project,
        // or from a file an earlier row of this pass was placed from: a copy.
        let first_from_extraction = !from_this_project
            && blob
                .path()
                .is_none_or(|src| placed_from.insert(src.to_path_buf()));
        let placed = if first_from_extraction {
            blob.link_or_place_at(&path)
        } else {
            blob.place_at(&path)
        };
        placed.with_context(|| format!("writing resource file {}", path.display()))?;
    }
    let keep: Vec<(uuid::Uuid, String)> = bundle
        .resources
        .iter()
        .filter(|r| r.kind == common::entities::ResourceKind::StoredFile)
        .map(|r| (r.uid, r.file_name.clone()))
        .collect();
    skrib::media::prune_resources_except(
        resources_dir,
        &keep,
        &bundle.manifest.legacy_resource_dirs,
    );
    Ok(())
}

/// Copy the open project's pictures and stored resources from `from` to `to`, for a
/// Save As that moves the project somewhere its files are not.
///
/// After a Save As the window is the new file, and every later save reads its files
/// from the new file's directories. A folder project saved as a single file keeps them
/// in the folder it came from, so without this the first save of the copy found none
/// and wrote it without its pictures. Linked when the two share a disk, copied
/// otherwise; nothing done when the directories are the same.
pub fn carry_project_files(g: &Gathered, from: &ProjectDirs, to: &ProjectDirs) -> Result<()> {
    if from.media != to.media {
        for (hash, blob) in read_asset_blobs(&g.assets, from) {
            let Some(a) = g.assets.iter().find(|a| a.content_hash == hash) else {
                continue;
            };
            let ext = skrib::media::extension_for(&a.mime_type);
            let dest = to.media.join(format!("{hash}.{ext}"));
            blob.place_at(&dest)
                .with_context(|| format!("copying media file to {}", dest.display()))?;
        }
    }
    if from.resources != to.resources {
        for r in &g.resources {
            let src = skrib::media::resource_file(&from.resources, r.uid, &r.file_name);
            if r.kind != common::entities::ResourceKind::StoredFile || !own_file(&from.base, &src) {
                continue;
            }
            let dest = skrib::media::resource_file(&to.resources, r.uid, &r.file_name);
            skrib::Blob::file(src)
                .place_at(&dest)
                .with_context(|| format!("copying resource file to {}", dest.display()))?;
        }
    }
    Ok(())
}

/// Point at the file of every gathered asset in the project's media directory.
///
/// Nothing is read: each [`skrib::Blob`] names the file, and the writer streams or
/// links it into place, so a save never holds the project's pictures in memory.
///
/// An asset whose file is missing is *skipped*, not an error. That is the whole
/// posture of this path: an image the writer can no longer see is a smaller
/// failure than a save that refuses to run, and `from_entities` drops the row to
/// match so the bundle never carries a dangling reference. The lost row is
/// recoverable — the prose still names the hash, so restoring the file restores
/// the picture. A file reached through a symbolic link is skipped the same way, and
/// said: a picture is the project's own bytes, never a door to a file elsewhere.
pub fn read_asset_blobs(
    assets: &[common::entities::Asset],
    dirs: &ProjectDirs,
) -> std::collections::BTreeMap<String, skrib::Blob> {
    let mut out = std::collections::BTreeMap::new();
    for a in assets {
        let ext = skrib::media::extension_for(&a.mime_type);
        let path = dirs.media.join(format!("{}.{ext}", a.content_hash));
        if own_file(&dirs.base, &path) {
            out.insert(a.content_hash.clone(), skrib::Blob::file(path));
        }
    }
    out
}

/// Place a loaded bundle's asset files into the project's media directory.
///
/// A no-op for an exploded-folder project, where the media directory *is* the
/// bundle's own `assets/`: each blob already points at the file it would be
/// placed on. Only a zip needs its images moved out of the extraction to
/// somewhere the editor can read them back, which is a hard link when the
/// extraction and the media directory share a disk.
///
/// Failing here fails the load: an image the writer can see but the next save
/// cannot find would be written out of the project silently, which is the one
/// outcome worth refusing to open over.
pub fn write_asset_blobs(bundle: &skrib::WorkBundle, media_dir: &std::path::Path) -> Result<()> {
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
        let blob = bundle
            .asset_blobs
            .get(&a.content_hash)
            .ok_or_else(|| anyhow!("bundle lists asset {} with no bytes", a.file_name))?;
        // From a zip's extraction, thrown away after the load: a link is safe.
        blob.link_or_place_at(&path)
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

/// Whether the bundle at `source` is another project than the one being written,
/// whose `unique_id` is `unique_id`.
///
/// A save reads the history log and the files the format does not model from its
/// source, which for an ordinary save is the project's own file. When a New Work
/// the writer agreed to let replace a project is first saved, that file is the
/// project it replaces, and carrying its log forward would put every version of the
/// replaced manuscript inside the new one, to travel with it wherever it is sent.
/// Only a readable manifest naming another project counts: a source with no manifest
/// (none yet, or a legacy file) has nothing of another project's to carry.
fn holds_another_project(source: &str, unique_id: &str) -> bool {
    if unique_id.is_empty() {
        return false;
    }
    match skrib::peek_manifest(source) {
        Ok(manifest) => {
            let theirs = &manifest.work.unique_id;
            !theirs.is_empty() && theirs != unique_id
        }
        Err(_) => false,
    }
}

// Eight, one over clippy's line: `work` and `kind` both answer the seam's "which
// write is this, for which open project", and are passed straight through to
// `bundle_contributors::collect` and `LifecycleEvent::Saved`.
#[allow(clippy::too_many_arguments)]
pub fn serialize_and_write(
    g: &Gathered,
    work: common::open_work::OpenWork,
    target: String,
    shape: SkribShape,
    shape_tag: ShapeTag,
    dirs: &ProjectDirs,
    history: HistoryAction,
    kind: crate::lifecycle::SaveKind,
) -> Result<String> {
    let mut bundle = skrib::from_entities(
        &g.work,
        &g.tags,
        &g.dict_words,
        &g.text_replacement_rules,
        &g.note_templates,
        &g.statuses,
        &g.assets,
        read_asset_blobs(&g.assets, dirs),
        &g.resources,
        read_resource_blobs(&g.resources, dirs),
        g.smart_punctuation.as_ref(),
        &g.trash_infos,
        &g.paces,
        &g.progress_snapshots,
        &g.comments,
        &g.footnotes,
        &g.binders,
        shape_tag,
    );
    // A linked file's path is stored relative to the folder holding the project, so the
    // link moves with the project and its research. Made relative to where THIS bundle
    // goes: a Save As elsewhere stores the paths as seen from there.
    skrib::media::relativise_links(
        &mut bundle,
        skrib::media::link_base(std::path::Path::new(&target)).as_deref(),
    );
    // Asked here, merged below. The gap is only an economy: `collect` clones the
    // bundle to fingerprint it, and doing that before `carry::load` means not
    // copying a project's unmodelled files for nothing. Correctness does not
    // rest on it — `manuscript_fingerprint` drops `carried` wherever it is asked,
    // which is what stops an extension's own bytes from looking like an edit to
    // the book.
    let contributed = crate::bundle_contributors::collect(&bundle, work, &g.work.unique_id, kind);

    // A source holding another project is one this write replaces, not one it goes
    // on from: its history and the files it carries are that project's own.
    let foreign = holds_another_project(history.source(), &g.work.unique_id);

    // Files the format does not model travel with the project, on every write
    // path. Read from the *source* bundle, never the target: `save_as` and
    // `backup_now` must bring the original's unmodelled files with them, and
    // reading the target would carry a backup's own stale contents instead.
    //
    // Both `HistoryAction` variants name the same source for the same reason,
    // so this is lifted out of the match rather than repeated inside it — one
    // write path forgetting the call is how a writer loses data silently.
    bundle.carried = if foreign {
        Default::default()
    } else {
        skrib::carry::load_for_save(history.source(), &bundle.resources)?
    };

    // The contributors' files then land **over** the on-disk read. That
    // direction is the whole point of the hook and is the only correct one:
    // what a contributor holds in memory is current, what `carry::load` found
    // on disk is whatever the last save left there. Merging disk second would
    // write the stale copy back over every live change, which is the exact bug
    // this hook exists to prevent.
    for (path, bytes) in contributed {
        bundle.carried.insert(path, skrib::CarriedFile::new(bytes));
    }

    let past = |source: &str| {
        if foreign {
            skrib::history::HistoryLog::default()
        } else {
            skrib::history::load(source)
        }
    };
    match history {
        HistoryAction::Carry { source } => {
            bundle.history = past(&source);
        }
        HistoryAction::Record {
            source,
            policy,
            min_keep,
        } => {
            let now = chrono::Utc::now();
            bundle.history = past(&source);
            skrib::history::record(&mut bundle, now);
            skrib::history::thin(&mut bundle.history, &policy, min_keep, now);
        }
    }
    skrib::write_bundle(&target, shape, &bundle)
        .map_err(|e| anyhow!("writing '{target}': {e:#}"))?;
    // The backend seam's "a write landed" hook, beside the contributor call it
    // mirrors: same function, same `unique_id`, same once-per-write cardinality
    // that both get here for free.
    if crate::lifecycle::has_listeners() {
        crate::lifecycle::notify(crate::lifecycle::LifecycleEvent::Saved {
            work,
            unique_id: g.work.unique_id.clone(),
            path: target.clone(),
            kind,
            live_binder_item_uids: live_binder_item_uids(g),
        });
    }
    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::entities::{Asset, Resource, ResourceKind};

    fn stored(uid: u128, file_name: &str) -> Resource {
        Resource {
            uid: uuid::Uuid::from_u128(uid),
            kind: ResourceKind::StoredFile,
            file_name: file_name.into(),
            ..Default::default()
        }
    }

    /// A stored row of a bundle, with `uid`, as a folder project lists it.
    fn stored_row(file_id: u64, uid: uuid::Uuid, file_name: &str) -> skrib::ResourceFile {
        skrib::ResourceFile {
            file_id,
            uid,
            created_at: String::new(),
            updated_at: String::new(),
            title: file_name.into(),
            description: String::new(),
            kind: ResourceKind::StoredFile,
            location: String::new(),
            file_name: file_name.into(),
            mime_type: "application/pdf".into(),
            byte_size: 8,
            content_hash: String::new(),
            width: 0,
            height: 0,
            path: skrib::slug::resource_relpath(uid, file_name),
        }
    }

    /// A folder project whose rows name the stored file in the folder of `uid`, with
    /// that file on disk, healed the way a load heals it. Returns the bundle and the
    /// file's path.
    fn healed_folder_project(
        resources_dir: &std::path::Path,
        uid: uuid::Uuid,
        rows: usize,
    ) -> (skrib::WorkBundle, std::path::PathBuf) {
        let name = "Village map.pdf";
        let file = skrib::media::resource_file(resources_dir, uid, name);
        std::fs::create_dir_all(file.parent().expect("parent")).expect("folder");
        std::fs::write(&file, b"%PDF-1.7").expect("write");
        let mut bundle = crate::save_load_test::sample_bundle();
        bundle.resources = (0..rows)
            .map(|i| stored_row(800 + i as u64, uid, name))
            .collect();
        bundle.resource_blobs = std::collections::BTreeMap::from([(
            uid.hyphenated().to_string(),
            skrib::Blob::file(file.clone()),
        )]);
        bundle.heal_resource_uids();
        (bundle, file)
    }

    /// A stored resource with no uid gets one as the project opens, and its file moves
    /// to the new uid's folder.
    ///
    /// The prune used to run first: it kept only today's uids, so it deleted the old
    /// folder, and the copy then read the file it had just deleted. The open failed and
    /// the only copy of the file was gone.
    #[test]
    fn a_resource_given_a_new_uid_on_open_keeps_its_file() {
        let dir = tempfile::tempdir().expect("tmp");
        let resources_dir = dir.path().join("resources");
        let (bundle, old) = healed_folder_project(&resources_dir, uuid::Uuid::nil(), 1);
        let r = &bundle.resources[0];
        assert!(!r.uid.is_nil());

        write_resource_blobs(&bundle, &resources_dir).expect("the open places the file");

        let new = skrib::media::resource_file(&resources_dir, r.uid, &r.file_name);
        assert_eq!(std::fs::read(&new).ok().as_deref(), Some(&b"%PDF-1.7"[..]));
        assert!(
            !old.exists(),
            "the nil folder, which no row names now, is pruned"
        );
    }

    /// Two rows that shared one uid each keep a copy of the file, not two names for one
    /// file: an application editing one must not change the other.
    #[test]
    fn rows_that_shared_a_uid_each_keep_their_own_copy() {
        let dir = tempfile::tempdir().expect("tmp");
        let resources_dir = dir.path().join("resources");
        let shared = uuid::Uuid::from_u128(7);
        let (bundle, kept) = healed_folder_project(&resources_dir, shared, 2);
        let other = bundle
            .resources
            .iter()
            .find(|r| r.uid != shared)
            .expect("the second row got a uid of its own");

        write_resource_blobs(&bundle, &resources_dir).expect("the open places the file");

        let copy = skrib::media::resource_file(&resources_dir, other.uid, &other.file_name);
        assert_eq!(std::fs::read(&copy).ok().as_deref(), Some(&b"%PDF-1.7"[..]));
        // Rewritten in place, as an editor saving over the file does.
        std::fs::write(&kept, b"%PDF-2.0 edited").expect("edit");
        assert_eq!(
            std::fs::read(&copy).ok().as_deref(),
            Some(&b"%PDF-1.7"[..]),
            "editing one row's file changed the other's"
        );
    }

    /// A folder project whose `resources.ron` gives a stored file no uid, opened again and
    /// again with nothing saved in between: the file is there after every open.
    ///
    /// An open gives the row a uid and copies the file into that uid's folder, but marks
    /// nothing unsaved, so `resources.ron` keeps no uid. Each open used to give another
    /// one: the second open pruned the folder the first had copied the file into, and the
    /// nil folder was already gone, so the file was lost.
    #[test]
    fn a_healed_resource_keeps_its_file_through_opens_without_a_save() {
        let dir = tempfile::tempdir().expect("tmp");
        let project = dir.path().join("Novel");
        let path = project.to_str().expect("utf-8").to_string();
        let name = "Village map.pdf";
        let nil = uuid::Uuid::nil();
        let mut bundle = crate::save_load_test::sample_bundle();
        bundle.resources = vec![stored_row(800, nil, name)];
        bundle.resource_blobs = std::collections::BTreeMap::from([(
            nil.hyphenated().to_string(),
            skrib::Blob::from(b"%PDF-1.7".to_vec()),
        )]);
        skrib::write_bundle(&path, SkribShape::ExplodedFolder, &bundle).expect("write");
        let resources_dir = project.join(skrib::slug::RESOURCES_DIR);

        let mut uids = Vec::new();
        for open in 1..=3 {
            let mut loaded = skrib::read_bundle(&path).expect("open");
            assert_eq!(loaded.resources[0].uid, nil, "nothing saved the healed uid");
            loaded.heal_resource_uids();
            write_resource_blobs(&loaded, &resources_dir).expect("the open places the file");
            let r = &loaded.resources[0];
            let file = skrib::media::resource_file(&resources_dir, r.uid, &r.file_name);
            assert_eq!(
                std::fs::read(&file).ok().as_deref(),
                Some(&b"%PDF-1.7"[..]),
                "the file is gone after open {open}"
            );
            uids.push(r.uid);
        }
        assert!(uids.windows(2).all(|w| w[0] == w[1]), "{uids:?}");
    }

    /// Two rows of a single-file project that shared one uid: placed from the one file
    /// the archive was extracted to, each gets a file of its own, not a second name for
    /// the first one's, so editing one does not change the other.
    #[test]
    fn rows_that_shared_a_uid_in_a_zip_each_get_their_own_file() {
        let dir = tempfile::tempdir().expect("tmp");
        let extraction = dir.path().join("extraction");
        let shared = uuid::Uuid::from_u128(7);
        let name = "Village map.pdf";
        let src = skrib::media::resource_file(&extraction, shared, name);
        std::fs::create_dir_all(src.parent().expect("parent")).expect("folder");
        std::fs::write(&src, b"%PDF-1.7").expect("write");
        let mut bundle = crate::save_load_test::sample_bundle();
        bundle.resources = vec![stored_row(800, shared, name), stored_row(801, shared, name)];
        bundle.resource_blobs = std::collections::BTreeMap::from([(
            shared.hyphenated().to_string(),
            skrib::Blob::file(src),
        )]);
        bundle.heal_resource_uids();
        let resources_dir = dir.path().join("media").join("resources");

        write_resource_blobs(&bundle, &resources_dir).expect("the open places the files");

        let files: Vec<std::path::PathBuf> = bundle
            .resources
            .iter()
            .map(|r| skrib::media::resource_file(&resources_dir, r.uid, &r.file_name))
            .collect();
        // Rewritten in place, as an application saving over the file does.
        std::fs::write(&files[0], b"%PDF-2.0 edited").expect("edit");
        assert_eq!(
            std::fs::read(&files[1]).ok().as_deref(),
            Some(&b"%PDF-1.7"[..]),
            "editing one row's file changed the other's"
        );
    }

    /// A save, a backup and a Save As read the project's stored files and pictures
    /// through these, and a link among them points at bytes that are not the project's:
    /// a folder project from elsewhere could otherwise carry `~/.ssh/id_rsa` into the
    /// archive or backup the writer makes of it.
    #[cfg(unix)]
    #[test]
    fn a_stored_file_or_picture_that_is_a_link_is_not_read() {
        let dir = tempfile::tempdir().expect("tmp");
        let secret = dir.path().join("id_rsa");
        std::fs::write(&secret, b"PRIVATE KEY").expect("write");
        let resources_dir = dir.path().join("resources");
        let dirs = ProjectDirs {
            media: dir.path().join("assets"),
            resources: resources_dir.clone(),
            base: dir.path().to_path_buf(),
        };
        let own = stored(1, "Village map.pdf");
        let linked = stored(2, "Tide tables.pdf");
        for r in [&own, &linked] {
            let folder = resources_dir.join(r.uid.hyphenated().to_string());
            std::fs::create_dir_all(&folder).expect("folder");
        }
        std::fs::write(
            skrib::media::resource_file(&resources_dir, own.uid, &own.file_name),
            b"%PDF",
        )
        .expect("write");
        std::os::unix::fs::symlink(
            &secret,
            skrib::media::resource_file(&resources_dir, linked.uid, &linked.file_name),
        )
        .expect("link");

        let blobs = read_resource_blobs(&[own.clone(), linked.clone()], &dirs);
        assert_eq!(
            blobs.keys().cloned().collect::<Vec<_>>(),
            vec![own.uid.hyphenated().to_string()]
        );

        let media_dir = dir.path().join("assets");
        std::fs::create_dir_all(&media_dir).expect("folder");
        let picture = Asset {
            content_hash: "hash-of-map".into(),
            mime_type: "image/png".into(),
            ..Default::default()
        };
        std::os::unix::fs::symlink(&secret, media_dir.join("hash-of-map.png")).expect("link");
        assert!(read_asset_blobs(&[picture], &dirs).is_empty());
    }

    /// A Save As that moves the project's files copies only its own, never a linked one.
    #[cfg(unix)]
    #[test]
    fn a_save_as_does_not_copy_a_linked_stored_file() {
        let dir = tempfile::tempdir().expect("tmp");
        let secret = dir.path().join("id_rsa");
        std::fs::write(&secret, b"PRIVATE KEY").expect("write");
        let from = ProjectDirs {
            media: dir.path().join("from-media"),
            resources: dir.path().join("from"),
            base: dir.path().to_path_buf(),
        };
        let to = ProjectDirs {
            media: dir.path().join("to-media"),
            resources: dir.path().join("to"),
            base: dir.path().to_path_buf(),
        };
        let linked = stored(2, "Tide tables.pdf");
        let src = skrib::media::resource_file(&from.resources, linked.uid, &linked.file_name);
        std::fs::create_dir_all(src.parent().expect("parent")).expect("folder");
        std::os::unix::fs::symlink(&secret, &src).expect("link");
        let g = Gathered {
            work: Default::default(),
            tags: Vec::new(),
            dict_words: Vec::new(),
            text_replacement_rules: Vec::new(),
            note_templates: Vec::new(),
            statuses: Vec::new(),
            assets: Vec::new(),
            resources: vec![linked.clone()],
            smart_punctuation: None,
            trash_infos: Vec::new(),
            paces: Vec::new(),
            progress_snapshots: Vec::new(),
            comments: Vec::new(),
            footnotes: Vec::new(),
            binders: Vec::new(),
            work_info: None,
        };
        carry_project_files(&g, &from, &to).expect("carried");
        assert!(
            !skrib::media::resource_file(&to.resources, linked.uid, &linked.file_name).exists()
        );
    }

    /// A folder project whose whole `resources/` or `assets/` folder is a link: the
    /// link is judged from the project folder, not from the folder it replaces, so a
    /// save or a backup reads none of what it points at.
    #[cfg(unix)]
    #[test]
    fn a_folder_projects_linked_resources_or_assets_folder_is_not_read() {
        let dir = tempfile::tempdir().expect("tmp");
        let elsewhere = dir.path().join("elsewhere");
        let r = stored(3, "Thesis.odt");
        let uid_folder = elsewhere.join(r.uid.hyphenated().to_string());
        std::fs::create_dir_all(&uid_folder).expect("folder");
        std::fs::write(uid_folder.join("Thesis.odt"), b"outside").expect("write");
        std::fs::write(elsewhere.join("hash-of-map.png"), b"outside").expect("write");
        let project = dir.path().join("Novel");
        std::fs::create_dir_all(&project).expect("folder");
        std::fs::write(project.join("project.skrib"), b"").expect("manifest");
        std::os::unix::fs::symlink(&elsewhere, project.join("resources")).expect("link");
        std::os::unix::fs::symlink(&elsewhere, project.join("assets")).expect("link");

        let dirs = ProjectDirs::of(&project, "uid", &dir.path().join("data"));
        assert_eq!(dirs.base, project);
        assert!(read_resource_blobs(std::slice::from_ref(&r), &dirs).is_empty());
        let picture = Asset {
            content_hash: "hash-of-map".into(),
            mime_type: "image/png".into(),
            ..Default::default()
        };
        assert!(read_asset_blobs(&[picture], &dirs).is_empty());
    }
}
