// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Generated structural words — "Chapter 3" / "Chapitre 3" / "الفصل ٣" — localized from
//! the scene's language and the preset's digit style.
//!
//! This is *generated furniture*: the exporter inserts these headings, so they must
//! localize. (The author's own prose is never touched.) v1 uses cardinal numbers, which
//! read correctly in every language here; gender-aware ordinals ("Chapitre premier" for
//! chapter 1) are a later refinement.

use crate::preset::DigitStyle;

/// Which structural level a generated heading names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Book,
    Part,
    Chapter,
}

/// The structural word for a level in a language (primary BCP-47 subtag). Unknown
/// languages fall back to English.
pub fn word(lang: &str, level: Level) -> &'static str {
    let primary = skribisto_model::language::primary(lang);
    let base = primary.split('-').next().unwrap_or("");
    match (base, level) {
        ("fr", Level::Book) => "Livre",
        ("fr", Level::Part) => "Partie",
        ("fr", Level::Chapter) => "Chapitre",
        ("de", Level::Book) => "Buch",
        ("de", Level::Part) => "Teil",
        ("de", Level::Chapter) => "Kapitel",
        ("es", Level::Book) => "Libro",
        ("es", Level::Part) => "Parte",
        ("es", Level::Chapter) => "Capítulo",
        ("it", Level::Book) => "Libro",
        ("it", Level::Part) => "Parte",
        ("it", Level::Chapter) => "Capitolo",
        ("ar", Level::Book) => "الكتاب",
        ("ar", Level::Part) => "الجزء",
        ("ar", Level::Chapter) => "الفصل",
        ("he", Level::Book) => "ספר",
        ("he", Level::Part) => "חלק",
        ("he", Level::Chapter) => "פרק",
        (_, Level::Book) => "Book",
        (_, Level::Part) => "Part",
        (_, Level::Chapter) => "Chapter",
    }
}

/// Render a number in the preset's digit style.
pub fn digits(n: usize, style: DigitStyle) -> String {
    match style {
        DigitStyle::Western => n.to_string(),
        DigitStyle::EasternArabic => n
            .to_string()
            .chars()
            .map(|c| match c {
                '0'..='9' => char::from_u32(0x0660 + (c as u32 - '0' as u32)).unwrap_or(c),
                other => other,
            })
            .collect(),
    }
}

/// "Chapter 3" — the structural word plus the number, in logical order (the block's text
/// direction reorders it visually for RTL).
pub fn numbered(lang: &str, level: Level, n: usize, style: DigitStyle) -> String {
    format!("{} {}", word(lang, level), digits(n, style))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_localize_and_fall_back() {
        assert_eq!(word("en-US", Level::Chapter), "Chapter");
        assert_eq!(word("fr-FR", Level::Chapter), "Chapitre");
        assert_eq!(word("de", Level::Chapter), "Kapitel");
        assert_eq!(word("ar", Level::Chapter), "الفصل");
        assert_eq!(word("he", Level::Chapter), "פרק");
        // Unknown language → English.
        assert_eq!(word("tlh", Level::Chapter), "Chapter");
    }

    #[test]
    fn eastern_arabic_digits() {
        assert_eq!(digits(3, DigitStyle::Western), "3");
        assert_eq!(digits(3, DigitStyle::EasternArabic), "٣");
        assert_eq!(digits(12, DigitStyle::EasternArabic), "١٢");
    }

    #[test]
    fn numbered_heading_is_word_then_number() {
        assert_eq!(numbered("fr", Level::Chapter, 3, DigitStyle::Western), "Chapitre 3");
        assert_eq!(numbered("ar", Level::Chapter, 3, DigitStyle::EasternArabic), "الفصل ٣");
    }
}
