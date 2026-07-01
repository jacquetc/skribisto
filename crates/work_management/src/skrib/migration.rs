//! Forward migration chain for the on-disk format, keyed on `format_version`.
//!
//! v1 → v2 added `WorkFile.unique_id`. That change is purely additive and RON is
//! name-keyed, so a v1 manifest deserializes fine via `#[serde(default)]` (empty
//! id) and the load path mints a fresh id when it's empty (see
//! `load_work_uc::materialize`) — no structural transform step is required here;
//! `migrate_bundle` just validates the range and normalizes the stamped version.

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
    // while bundle.manifest.format_version < FORMAT_VERSION { step_vN_to_vN1(bundle)?; }
    bundle.manifest.format_version = FORMAT_VERSION;
    Ok(())
}
