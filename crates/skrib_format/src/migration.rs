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
    while bundle.manifest.format_version < FORMAT_VERSION {
        match bundle.manifest.format_version {
            1 | 2 => step_v2_to_v3(bundle),
            other => anyhow::bail!("no migration step from .skrib format_version {other}"),
        }
        bundle.manifest.format_version += 1;
    }
    bundle.manifest.format_version = FORMAT_VERSION;
    Ok(())
}

/// Mint a durable `uid` for every binder and item that lacks one.
///
/// Idempotent: a row that already carries a uid keeps it, so re-running the
/// step (or meeting a partially-migrated bundle) never re-mints and never
/// breaks an existing reference. v1 bundles pass through here too — they are
/// missing the field for the same reason v2 ones are.
fn step_v2_to_v3(bundle: &mut WorkBundle) {
    for bb in &mut bundle.binders {
        if bb.binder.uid.is_empty() {
            bb.binder.uid = crate::new_unique_id();
        }
        for bi in &mut bb.items {
            if bi.item.uid.is_empty() {
                bi.item.uid = crate::new_unique_id();
            }
        }
    }
}
