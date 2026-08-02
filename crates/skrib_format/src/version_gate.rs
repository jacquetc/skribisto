// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The single authority on "can this build open this bundle" — both directions.
//!
//! # Why a gate at all
//!
//! [`migration::migrate_bundle`](crate::migration::migrate_bundle) used to own the
//! "written by a newer Skribisto" refusal, and it runs **after** the whole bundle is
//! parsed (for a zip, after the entire archive is extracted to a tempdir). That made
//! the check reachable only for a bump that changes nothing but the integer — the one
//! kind of bump nobody makes. The moment a future version adds a variant to
//! `ContentRole` / `BinderItemSubRole` / `BinderItemRole`, `ron::from_str` on
//! `items.ron` hard-fails several call frames earlier with a raw
//! `Unexpected variant named "…"`, and the friendly message is never reached.
//!
//! So the version check moved *ahead* of parsing, into [`check_version_gate`], and it
//! reads only the two integers it needs out of `project.skrib`.
//!
//! # Why two version numbers
//!
//! [`mapping::from_entities`](crate::mapping) stamps `format_version = FORMAT_VERSION`
//! on **every** write, unconditionally — including autosave, which fires every few
//! seconds. Under a bare refuse-if-greater rule, opening a project once in a newer
//! build and letting a single autosave tick land makes it permanently unopenable by the
//! previous build, *whether or not anything new was used*. That is a real cliff for
//! anyone running stable next to a beta, or rolling back.
//!
//! Hence the pair, as in EBML's `DocTypeVersion`/`DocTypeReadVersion`, glTF's
//! `asset.version`/`asset.minVersion`, and SQLite's write/read-version header bytes:
//!
//! * `format_version` — the writer's own generation. Informational for the gate.
//! * [`format_min_read_version`](crate::ProjectManifest::format_min_read_version) — the
//!   **content-derived floor**: the lowest generation a reader must implement to open
//!   this bundle without losing anything. Recomputed fresh at every write from what the
//!   bundle actually contains (see [`compute_min_read_version`]), never carried over
//!   from what was loaded.
//!
//! The gate judges the floor, not the stamp. A newer build that saves a project
//! containing nothing new writes a floor this build understands, so the project stays
//! openable here indefinitely, across any number of open/edit/save cycles. Delete the
//! newer content and the next save recomputes a lower floor — nothing is sticky.
//!
//! # The one thing that must not be duplicated
//!
//! `migrate_bundle` must **not** also compare `format_version` against `FORMAT_VERSION`.
//! It runs after the gate has already admitted the bundle on the floor, so a stamp above
//! ours is legitimate on entry — refusing there would reject exactly the file the gate
//! just let through, after paying the full parse cost. See its own doc comment.

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
/// unconditionally (v3 → v4 turned `dict_language` from a space-separated string into a
/// list, which every write emits whether or not the project uses it). Nothing below 4
/// can parse what this crate writes, so no bundle ever claims a lower floor.
///
/// This is a historical constant and must **never** be written as `FORMAT_VERSION`:
/// tying it to the current version would raise every bundle's floor on every bump and
/// silently undo the whole point of the scheme.
///
/// It is not, and does not pretend to be, a promise that some specific historical build
/// can read the file — `BinderTagFile` dropped `text_color` without a bump (see its doc
/// comment), so a genuine v4-vintage binary cannot. The floor is only ever *read* by
/// builds that implement this gate; for them, 4 is simply "needs nothing newer".
const MIN_READ_BASELINE: u32 = 4;

/// The lowest `format_version` a reader must implement to open `bundle` without loss.
///
/// Walks the **bundle**, not the store entities it was built from: `from_entities` drops
/// content that fails `skribisto_model::content_allowed` before bundling it, so scoring
/// the pre-filter input would count rows that never reach disk and needlessly refuse
/// readers that would have been fine.
///
/// Called from the one place every write path funnels through
/// ([`folder_io::write_folder`](crate::folder_io::write_folder)'s manifest commit), so
/// no producer — not `from_entities`, not the Plume importer, not
/// `mark_existing_as_backup` — can forget to stamp it.
pub fn compute_min_read_version(bundle: &WorkBundle) -> u32 {
    let mut floor = MIN_READ_BASELINE;

    // Note templates are the reason v5 exists at all, and the reason is a *write*
    // hazard rather than a read one: `zip_io::write_zip` rebuilds the archive from a
    // fresh staging dir, so an older build — whose `WorkBundle` has no `note_templates`
    // field — would silently drop every template the first time it saved. Refusing the
    // open is what turns that into a loud, recoverable error, and that refusal only
    // happens if the floor says so. A project with no templates is unaffected, which is
    // exactly the precision the two-number scheme buys.
    if !bundle.note_templates.is_empty() {
        floor = floor.max(5);
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
    }
}

/// The two integers the gate needs, and nothing else.
///
/// Deliberately **not** [`peek_manifest`](crate::peek_manifest), which parses the whole
/// [`ProjectManifest`](crate::ProjectManifest) — including `shape: ShapeTag` and
/// `kind: BundleKind`, plain derived enums with no `#[serde(other)]`. A future third
/// `ShapeTag` or a new `BundleKind` would hard-fail the probe itself, reopening this
/// very bug one level up.
///
/// Serde's derived struct visitor *skips the raw value* of any key it does not declare,
/// without trying to interpret its shape, and there is no `deny_unknown_fields` anywhere
/// in this crate. So a manifest from an arbitrarily distant future — new fields, new
/// nested types, new enum values in fields this struct doesn't name — still yields its
/// two integers. The probe is safe by construction, not by discipline;
/// `probe_survives_an_unrecognised_shape_tag_variant` pins that difference.
///
/// # Why the rename
///
/// That "unknown keys are skipped" guarantee covers *fields*, not the struct **name**.
/// `folder_io::to_ron` serialises with `PrettyConfig::struct_names(true)`, so every
/// manifest on disk opens with the literal `ProjectManifest(`, and RON checks that name
/// before it looks at a single field: without the rename, every real bundle fails the
/// probe with `Expected struct 'VersionProbe' but found 'ProjectManifest'`. RON is
/// lenient the other way — a hand-written manifest that omits the name still parses — so
/// the rename costs nothing and covers both spellings.
///
/// This does couple the probe to `ProjectManifest`'s *type name*. That coupling is
/// deliberately loud here and pinned by `the_probe_reads_a_real_serialized_manifest`,
/// which round-trips an actual manifest rather than a hand-written string.
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
