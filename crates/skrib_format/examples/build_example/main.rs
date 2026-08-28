// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Builds the example projects shipped in `resources/examples/`.
//!
//! ```sh
//! # a public-domain plain-text novel becomes a manuscript
//! cargo run -p skribisto-skrib-format --example build_example -- convert \
//!     resources/examples/le_tour_du_monde_en_80_jours/le-tour-du-monde-en-quatre-vingts-jours.toml \
//!     ~/Devel/livres/tdm80j.txt \
//!     resources/examples/le_tour_du_monde_en_80_jours/le-tour-du-monde-en-quatre-vingts-jours.skrib
//!
//! # editorial metadata is added to any bundle, converted or not
//! cargo run -p skribisto-skrib-format --example build_example -- enrich \
//!     resources/examples/le_tour_du_monde_en_80_jours/le-tour-du-monde-en-quatre-vingts-jours.toml \
//!     resources/examples/le_tour_du_monde_en_80_jours/le-tour-du-monde-en-quatre-vingts-jours.skrib
//! ```
//!
//! `convert` needs a source text; `enrich` does not, which is the whole reason the
//! two are separate. Starforgers has no plain-text source — it arrived as a bundle,
//! from its author — so it runs `enrich` alone, over the same TOML schema.
//!
//! ## Why a tool and not hand-edited RON
//!
//! Both halves read and write through this crate's own `read_bundle`/`write_bundle`,
//! so the result is valid by construction. Hand-editing `items.ron` produces a file
//! whose validity is discovered when a reader opens it.
//!
//! ## Tests
//!
//! `cargo test` does not run an example's tests. The parser and the Djot round-trip
//! assumptions below are covered, and CI runs them explicitly:
//!
//! ```sh
//! cargo test --workspace --examples
//! ```

mod convert;
mod enrich;
mod source_text;
mod spec;

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["convert", spec, source, out] => convert::run(spec, source, out),
        ["enrich", spec, bundle] => enrich::run(spec, bundle),
        _ => bail!(
            "usage:\n  build_example convert <spec.toml> <source.txt> <out.skrib>\n  \
             build_example enrich  <spec.toml> <bundle.skrib>"
        ),
    }
}
