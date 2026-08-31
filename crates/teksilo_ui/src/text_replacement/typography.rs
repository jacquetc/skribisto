// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The smart-punctuation engine, re-exported from where it now lives.
//!
//! The engine moved down to [`skribisto_model::typography::engine`], beside the
//! rulesets it applies. It was always pure text in, text out — no widget, no
//! backend — and half of this file was already a facade re-exporting
//! `QuoteSystem`/`TypographyRuleset` from that very module, which is the shape a
//! thing takes when it has outgrown where it sits.
//!
//! Kept as a module here so every call site keeps spelling it
//! `crate::text_replacement::typography::…` — the same arrangement
//! `comments::anchor` has over `skribisto_model::comment_anchor`.

pub use skribisto_model::typography::engine::*;
