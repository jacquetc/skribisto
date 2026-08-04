// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Forward migration chain for the on-disk format, keyed on `format_version`.
//!
//! v1 → v2 added `WorkFile.unique_id`. That change is purely additive and RON is
//! name-keyed, so a v1 manifest deserializes fine via `#[serde(default)]` (empty
//! id) and the load path mints a fresh id when it's empty (see
//! `load_work_uc::materialize`) — no structural transform step was required.
//!
//! v2 → v3 added `BinderFile.uid` / `BinderItemFile.uid`. Also additive, but it
//! cannot be healed the same way: `materialize` heals ONE work-level id, while
//! this needs a fresh id per row, and every row must get one before anything
//! keys off it. So this is the chain's first real step — the `while` loop below
//! finally runs.
//!
//! **The legacy SQLite path does not come through here.** It builds its graph in
//! `load_work_uc::legacy_to_loaded` without ever constructing a `WorkBundle`, so
//! it mints its own uids; a fix confined to this file would silently miss every
//! legacy project.

use anyhow::Result;

use super::bundle::{FORMAT_VERSION, WorkBundle};

/// Walk `bundle` forward to [`FORMAT_VERSION`], one arm per transition.
///
/// **This does not decide whether the bundle is too new** — [`crate::version_gate`] does,
/// before any parsing, and it is the sole authority. A ceiling check here used to exist
/// and had to go: the gate judges `format_min_read_version` (the content-derived floor),
/// not the raw writer stamp, so `format_version` can legitimately exceed `FORMAT_VERSION`
/// on entry. That is the entire point of the floor scheme — a newer build saving a
/// project that contains nothing new stamps a higher version but a floor we understand.
/// Re-checking the stamp here would refuse exactly the file the gate just admitted, after
/// paying the full parse cost.
///
/// The loop below correctly no-ops for such a bundle, and the next save re-stamps
/// `format_version` to ours regardless (`folder_io::write_folder` writes the manifest,
/// `mapping::from_entities` builds it), so a from-the-future in-memory value never
/// reaches disk.
pub fn migrate_bundle(bundle: &mut WorkBundle) -> Result<()> {
    let v = bundle.manifest.format_version;
    if v == 0 {
        // The gate catches this pre-parse for every `read_bundle` call; kept here so the
        // function's own contract holds for a caller that builds a bundle in memory.
        anyhow::bail!("invalid .skrib format_version 0");
    }
    // One arm per transition, so the chain reads as the sequence it is and a
    // future v3→v4 step cannot be bolted onto an arm that already means
    // something else. A version with no arm fails loudly rather than being
    // silently stamped as current.
    while bundle.manifest.format_version < FORMAT_VERSION {
        match bundle.manifest.format_version {
            1 => step_v1_to_v2(bundle),
            2 => step_v2_to_v3(bundle),
            3 => step_v3_to_v4(bundle),
            4 => step_v4_to_v5(bundle),
            5 => step_v5_to_v6(bundle),
            6 => step_v6_to_v7(bundle),
            other => anyhow::bail!("no migration step from .skrib format_version {other}"),
        }
        bundle.manifest.format_version += 1;
    }
    Ok(())
}

/// v3 → v4 turned `dict_language` from a space-separated string into a real list.
///
/// The split itself happens in the deserializer (`bundle::tags_or_legacy_string`), because a
/// type change has to be tolerated at *parse* time — this chain runs afterwards, and a v3
/// file would never reach it. So this arm only advances the stamp, exactly as v1 → v2 does
/// for a field healed elsewhere. It still has to exist: a version with no arm fails loudly.
fn step_v3_to_v4(_bundle: &mut WorkBundle) {}

/// v4 → v5 added the note templates. Nothing to heal: a v4 bundle simply had none, and
/// `read_folder` already yields an empty list for the absent `templates.ron`. The bump
/// exists to stop an *older* build opening (and then silently re-saving without) a
/// project that has templates — see [`FORMAT_VERSION`].
fn step_v4_to_v5(_bundle: &mut WorkBundle) {}

/// v5 → v6 added epigraphs. Nothing to heal in this direction either: a v5 bundle simply
/// has no `EpigraphText` rows, and an absent `*.epigraph.djot` is indistinguishable from a
/// project that never wrote one. The bump exists for the *other* direction — an older
/// build cannot deserialize the new `ContentRole` variant at all, so
/// [`version_gate`](crate::version_gate) refuses the bundle up front instead of letting
/// `items.ron` fail with a raw "unexpected variant".
fn step_v5_to_v6(_bundle: &mut WorkBundle) {}

/// v6 → v7 added paratexts. Nothing to heal: a v6 bundle simply has none. The bump exists
/// so an older build refuses the file rather than failing to deserialize the two new enum
/// variants — the same reason v6 exists.
fn step_v6_to_v7(_bundle: &mut WorkBundle) {}

/// Mint a durable `uid` for every binder and item that lacks one.
///
/// Idempotent: a row that already carries a uid keeps it, so re-running the
/// step (or meeting a partially-migrated bundle) never re-mints and never
/// breaks an existing reference. v1 bundles pass through here too — they are
/// missing the field for the same reason v2 ones are.
fn step_v2_to_v3(bundle: &mut WorkBundle) {
    for bb in &mut bundle.binders {
        bb.binder.uid = common::uid::heal_uid(bb.binder.uid);
        for bi in &mut bb.items {
            bi.item.uid = common::uid::heal_uid(bi.item.uid);
        }
    }
}

/// v1 → v2 added `WorkFile.unique_id`, healed downstream by
/// `load_work_uc::materialize` rather than here, so this step only advances the
/// stamp. It exists so the chain has one arm per transition: a v1 bundle must
/// still pass through v2 on its way to v3.
fn step_v1_to_v2(_bundle: &mut WorkBundle) {}
