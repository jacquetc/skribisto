// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The single authority on "can this build open this bundle" — both directions.
//!
//! # Why a gate at all
//!
//! The version check used to live in `migrate_bundle`, which runs only after the whole
//! bundle is parsed. That's too late: a future variant added to `ContentRole` /
//! `BinderItemSubRole` / `BinderItemRole` makes `ron::from_str` on `items.ron` hard-fail
//! with a raw `Unexpected variant named "…"` several frames before the friendly refusal
//! is ever reached. So the check moved *ahead* of parsing, into `check_version_gate`,
//! reading only the two integers it needs out of `project.skrib`.
//!
//! # Why two version numbers
//!
//! `mapping::from_entities` stamps `format_version = FORMAT_VERSION` on
//! every write, including autosave. A bare refuse-if-greater rule on that alone would make
//! a project permanently unopenable by an older build after a single autosave tick lands
//! in a newer one, whether or not anything new was actually used. So the gate judges a
//! second, content-derived number instead:
//!
//! * `format_version` — the writer's own generation. Informational only.
//! * [`format_min_read_version`](crate::ProjectManifest::format_min_read_version) — the
//!   lowest generation a reader must implement to open this bundle without loss.
//!   Recomputed fresh at every write from what the bundle actually contains (see
//!   [`compute_min_read_version`](crate::version_gate::compute_min_read_version)), never carried over from what was loaded — delete the
//!   newer content and the next save lowers the floor again.
//!
//! # The one thing that must not be duplicated
//!
//! `migrate_bundle` must **not** also compare `format_version` against `FORMAT_VERSION`:
//! it runs after the gate has already admitted the bundle on the floor, so a stamp above
//! ours is legitimate on entry there. See its own doc comment.

use anyhow::Context;
use common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use std::fs::File;
use std::io::Read;

use super::bundle::{FORMAT_VERSION, WorkBundle};
use super::errors::SkribFormatError;
use super::shape::{MANIFEST_NAME, SkribShape, folder_root};

/// The floor claimed by a bundle whose content needs nothing version-gated.
///
/// Frozen at 4 — the last format that changed a field's on-disk *representation*
/// unconditionally (v3 → v4 turned `dict_language` into a list). Nothing below 4 can
/// parse what this crate writes, so no bundle ever claims a lower floor.
///
/// Must **never** be written as `FORMAT_VERSION`: tying it to the current version would
/// raise every bundle's floor on every bump and undo the whole point of the scheme. It is
/// also not a promise that some specific historical build can read the file —
/// `BinderTagFile` dropped `text_color` without a bump, so a genuine v4 binary cannot.
const MIN_READ_BASELINE: u32 = 4;

/// The lowest `format_version` a reader must implement to open `bundle` without loss.
///
/// Walks the **bundle**, not the store entities it was built from: `from_entities` drops
/// content that fails `skribisto_model::content_allowed` before bundling it, so scoring
/// the pre-filter input would count rows that never reach disk and needlessly refuse
/// readers that would have been fine.
///
/// Called from the one place every write path funnels through
/// (`folder_io::write_folder`'s manifest commit), so
/// no producer — not `from_entities`, not the Plume importer, not
/// `mark_existing_as_backup` — can forget to stamp it.
pub fn compute_min_read_version(bundle: &WorkBundle) -> u32 {
    let mut floor = MIN_READ_BASELINE;

    // Note templates are why v5 exists: `zip_io::write_zip` rebuilds the archive from a
    // fresh staging dir, so an older build (no `note_templates` field) would silently
    // drop every template on its first save. The floor turns that into a loud refusal
    // instead — and only for projects that actually have templates.
    if !bundle.note_templates.is_empty() {
        floor = floor.max(5);
    }

    // Statuses are why v14 exists, and for the same mechanical reason templates were why
    // v5 does. `statuses.ron` is a new root manifest: the zip writer rebuilds the archive
    // from a fresh staging directory and the exploded writer prunes what it does not
    // expect, so an older build — which has no `statuses` field at all — would delete the
    // whole ladder on its first save and leave every item's `status_id` pointing at
    // nothing. That is a vocabulary the writer named, ordered and assigned, not a
    // preference; refuse to open instead. Gated on the project actually having one, so a
    // bundle with an empty ladder stays open to every older build.
    if !bundle.statuses.is_empty() {
        floor = floor.max(14);
    }

    // Assets are why v8 exists, and for exactly the reason templates were why v5
    // does: the zip writer rebuilds the archive from a fresh staging directory,
    // and the exploded writer prunes what it does not expect. An older build has
    // neither `assets.ron` nor the `assets/` tree in its `WorkBundle`, so its
    // first save would delete every image in the project. The floor makes that a
    // refusal to open instead — and only for projects that actually have images.
    // Footnotes are why v9 exists, and for the same mechanical reason — but with
    // more at stake than either. A comment an older build cannot see is a comment
    // the writer loses a note from; a footnote is prose that belongs to the book,
    // and the first save by a build that has never heard of `.footnotes.ron` would
    // prune every one of them off disk. Refuse to open instead.
    if bundle.orphan_footnotes.is_empty()
        && !bundle
            .binders
            .iter()
            .any(|bb| bb.items.iter().any(|bi| !bi.footnotes.is_empty()))
    {
        // No footnotes anywhere: this project stays open to every v4..v8 build.
    } else {
        floor = floor.max(9);
    }

    if !bundle.assets.is_empty() {
        floor = floor.max(8);
    }

    for bb in &bundle.binders {
        for bi in &bb.items {
            floor = floor.max(binder_item_role_min_version(&bi.item.role));
            floor = floor.max(binder_item_sub_role_min_version(&bi.item.sub_role));
            for ic in &bi.item.inline_contents {
                floor = floor.max(content_role_min_version(&ic.role));
            }
            for pr in &bi.item.prose_refs {
                floor = floor.max(content_role_min_version(&pr.role));
            }
        }
    }

    floor
}

// The three matches below are deliberately exhaustive, with no wildcard arm: adding a
// variant to any of these enums must be a *compile error* here until someone decides
// which format version it belongs to. That forcing function is the whole mechanism —
// quieting it with a `_ => MIN_READ_BASELINE` arm would silently reopen the bug this
// module exists to close.

fn content_role_min_version(role: &ContentRole) -> u32 {
    match role {
        ContentRole::SceneText
        | ContentRole::NoteText
        | ContentRole::SynopsisText
        | ContentRole::BookTitle
        | ContentRole::BookSubtitle
        | ContentRole::PartTitle
        | ContentRole::ChapterTitle => MIN_READ_BASELINE,
        // v6. A build that predates epigraphs has no such variant, so `items.ron` fails
        // to deserialize outright — the raw "unexpected variant" error this module
        // exists to turn into a refusal. Only bundles that actually carry an epigraph
        // claim the floor; a project that has none stays open to every v4/v5 build.
        ContentRole::EpigraphText => 6,
        // v7. Same reasoning as the epigraph one line up: a build that predates paratexts
        // cannot deserialize the variant, so the gate refuses the bundle rather than
        // letting `items.ron` fail with a raw "unexpected variant".
        ContentRole::ParatextText => 7,
    }
}

fn binder_item_role_min_version(role: &BinderItemRole) -> u32 {
    match role {
        BinderItemRole::Item | BinderItemRole::Folder => MIN_READ_BASELINE,
    }
}

fn binder_item_sub_role_min_version(sub_role: &BinderItemSubRole) -> u32 {
    match sub_role {
        BinderItemSubRole::Text
        | BinderItemSubRole::None
        | BinderItemSubRole::Note
        | BinderItemSubRole::Book
        | BinderItemSubRole::Part
        | BinderItemSubRole::Scene
        | BinderItemSubRole::ChapterScene
        | BinderItemSubRole::BookBegin
        | BinderItemSubRole::BookEnd => MIN_READ_BASELINE,
        BinderItemSubRole::Paratext => 7,
    }
}

/// The two integers the gate needs, and nothing else.
///
/// Deliberately **not** [`peek_manifest`](crate::peek_manifest), which parses the whole
/// [`ProjectManifest`](crate::ProjectManifest) including `shape`/`kind` — plain derived
/// enums with no `#[serde(other)]`, so a future variant there would hard-fail the probe
/// itself. Serde's derived struct visitor skips any key it doesn't declare (no
/// `deny_unknown_fields` in this crate), so a manifest from an arbitrarily distant future
/// still yields its two integers; `probe_survives_an_unrecognised_shape_tag_variant` pins
/// this.
///
/// # Why the rename
///
/// The "unknown keys are skipped" guarantee covers *fields*, not the struct name.
/// `folder_io::to_ron` serialises with `PrettyConfig::struct_names(true)`, so every
/// manifest opens with the literal `ProjectManifest(`, and RON checks that name before
/// looking at any field — without the rename every real bundle fails the probe. This
/// couples the probe to `ProjectManifest`'s type name, pinned by
/// `the_probe_reads_a_real_serialized_manifest`.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename = "ProjectManifest")]
struct VersionProbe {
    format_version: u32,
    #[serde(default)]
    format_min_read_version: Option<u32>,
}

/// Read just the manifest text, without extracting anything else.
fn probe_text(path: &str, shape: SkribShape) -> anyhow::Result<String> {
    match shape {
        SkribShape::ExplodedFolder => {
            let manifest_path = folder_root(path).join(MANIFEST_NAME);
            std::fs::read_to_string(&manifest_path)
                .with_context(|| format!("reading {}", manifest_path.display()))
        }
        SkribShape::ZipFile => {
            // Streams ONLY the `project.skrib` entry, mirroring `peek_manifest`'s zip
            // arm. Never `.extract()` — the cost of extracting a whole archive before
            // discovering it cannot be opened is precisely what this gate exists to
            // avoid.
            let file = File::open(path).with_context(|| format!("opening '{path}'"))?;
            let mut archive =
                zip::ZipArchive::new(file).with_context(|| format!("reading zip '{path}'"))?;
            let mut entry = archive
                .by_name(MANIFEST_NAME)
                .with_context(|| format!("no {MANIFEST_NAME} entry in '{path}'"))?;
            let mut text = String::new();
            entry
                .read_to_string(&mut text)
                .with_context(|| format!("reading {MANIFEST_NAME} from '{path}'"))?;
            Ok(text)
        }
        SkribShape::LegacySqlite => {
            // Not reachable through `read_bundle`, which rejects this shape first. An
            // error rather than a panic: a read path in a library has no business
            // aborting the process over a caller's mistake.
            anyhow::bail!("'{path}' is a legacy SQLite file; it has no manifest to probe")
        }
    }
}

/// Judge, **before parsing anything**, whether this build can open the bundle at `path`.
///
/// The sole authority on "too new to open" — see the module docs for why
/// `migrate_bundle` must not second-guess it.
pub(crate) fn check_version_gate(path: &str, shape: SkribShape) -> Result<(), SkribFormatError> {
    let text = probe_text(path, shape).map_err(SkribFormatError::Unreadable)?;
    let probe: VersionProbe = ron::from_str(&text)
        .with_context(|| format!("parsing {MANIFEST_NAME} version fields from '{path}'"))
        .map_err(SkribFormatError::Unreadable)?;

    if probe.format_version == 0 {
        return Err(SkribFormatError::InvalidVersion);
    }

    // Absent on every manifest written before this field existed — i.e. every file any
    // real user holds, all of which are `format_version <= 2`. Falling back to the raw
    // stamp reproduces exactly the refuse-if-greater rule the crate had, so those files
    // open precisely as they always did. No backfill, no migration.
    let floor = probe
        .format_min_read_version
        .unwrap_or(probe.format_version);

    if floor > FORMAT_VERSION {
        return Err(SkribFormatError::TooNew {
            written_by: probe.format_version,
            requires_at_least: floor,
            supported: FORMAT_VERSION,
        });
    }

    Ok(())
}
