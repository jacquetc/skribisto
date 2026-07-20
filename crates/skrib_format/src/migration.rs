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

pub fn migrate_bundle(bundle: &mut WorkBundle) -> Result<()> {
    let v = bundle.manifest.format_version;
    if v == 0 {
        anyhow::bail!("invalid .skrib format_version 0");
    }
    if v > FORMAT_VERSION {
        anyhow::bail!(
            "this .skrib was written by a newer Skribisto (format_version {v} > {FORMAT_VERSION}); please upgrade"
        );
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
            other => anyhow::bail!("no migration step from .skrib format_version {other}"),
        }
        bundle.manifest.format_version += 1;
    }
    Ok(())
}

/// Mint a durable `uid` for every binder and item that lacks one.
///
/// Idempotent: a row that already carries a uid keeps it, so re-running the
/// step (or meeting a partially-migrated bundle) never re-mints and never
/// breaks an existing reference. v1 bundles pass through here too — they are
/// missing the field for the same reason v2 ones are.
/// v3 → v4 turned `dict_language` from a space-separated string into a real list.
///
/// The split itself happens in the deserializer (`bundle::tags_or_legacy_string`), because a
/// type change has to be tolerated at *parse* time — this chain runs afterwards, and a v3
/// file would never reach it. So this arm only advances the stamp, exactly as v1 → v2 does
/// for a field healed elsewhere. It still has to exist: a version with no arm fails loudly.
fn step_v3_to_v4(_bundle: &mut WorkBundle) {}

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
