//! Forward migration chain for the on-disk format, keyed on `format_version`.
//! v1 is the only version today; the seam is here for future steps.

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
