// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Font-byte selection for the PDF exporter.
//!
//! text-document's PDF backend renders with ONLY the font bytes it is handed (no system-font
//! search), so the compiler gathers the right set here: the preset's body serif, plus the
//! bundled Noto RTL faces when the export contains right-to-left scenes. Bytes come from the
//! shared [`skribisto_fonts`] crate (serifs) and [`teksilo_text`] (Noto) — the same blobs the
//! editor shapes with, so an exported PDF matches what the writer saw on screen.

use std::collections::BTreeSet;

use skribisto_model::language;

use crate::preset::{DirectionMode, Preset};

/// The font bytes to feed Typst for a PDF export: the preset's body serif (substituting a
/// bundled face when the named family has no bundled bytes — e.g. "Times New Roman" /
/// "Courier New"), plus the Noto Arabic + Hebrew faces when any included scene is right-to-left
/// (Typst's own font fallback then picks the correct face per glyph).
pub fn pdf_font_bytes(preset: &Preset, langs: &BTreeSet<String>) -> Vec<Vec<u8>> {
    let mut fonts = Vec::new();

    // Body serif: the preset's family if bundled, else EB Garamond as a deterministic
    // substitute (so a system-font preset still exports a real serif rather than Typst's
    // built-in default face). Substitution is a fixed choice, not a silent platform lookup.
    let body = skribisto_fonts::by_family(&preset.font_family)
        .unwrap_or_else(skribisto_fonts::eb_garamond);
    fonts.push(body.to_vec());

    let has_rtl =
        preset.direction == DirectionMode::ForceRtl || langs.iter().any(|t| language::is_rtl(t));
    if has_rtl {
        // Feed both RTL faces when any scene is RTL; Typst's fallback selects Arabic vs Hebrew
        // per glyph, which avoids guessing the script from a language tag.
        fonts.push(teksilo_text::noto_sans_arabic_bytes().to_vec());
        fonts.push(teksilo_text::noto_sans_hebrew_bytes().to_vec());
    }
    fonts
}

/// The body font *family name* that matches the bytes [`pdf_font_bytes`] feeds — the preset's
/// own family when it's bundled, otherwise "EB Garamond" (the substitute). Typst's
/// `#set text(font:)` must name the face whose bytes it actually has, or it silently falls back.
pub fn pdf_body_family(preset: &Preset) -> String {
    if skribisto_fonts::by_family(&preset.font_family).is_some() {
        preset.font_family.clone()
    } else {
        "EB Garamond".to_string()
    }
}
