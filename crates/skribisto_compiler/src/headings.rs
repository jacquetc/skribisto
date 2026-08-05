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
///
/// Re-exported from the model rather than declared here: this used to be a private
/// `Level` enum with exactly the same three variants as
/// [`skribisto_model::compile::StreamLevel`], which meant the compile spine, the UI's
/// stream view and the heading generator each named the same three-valued fact
/// separately. One name, one definition.
pub use skribisto_model::compile::StreamLevel as Level;

/// The structural word for a level in a language (primary BCP-47 subtag). Unknown
/// languages fall back to English.
pub fn word(lang: &str, level: Level) -> &'static str {
    // One tag in, by contract — the caller resolves the primary from the list.
    let base = lang.trim().split('-').next().unwrap_or("");
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

/// The byline preposition — "by Ursula K. Le Guin" under a title page's title.
///
/// Same language coverage and same English fallback as [`word`], because a title page in a
/// language we do not name is better off reading as English than as nothing.
pub fn by(lang: &str) -> &'static str {
    match lang.trim().split('-').next().unwrap_or("") {
        "fr" => "par",
        "de" => "von",
        "es" => "por",
        "it" => "di",
        "ar" => "بقلم",
        "he" => "מאת",
        _ => "by",
    }
}

/// Group a number with the language's own thousands separator.
///
/// French uses a no-break space rather than the narrow no-break space typography would
/// prefer: U+202F is missing from several of the bundled writing serifs, and a title page
/// that renders a tofu box where a space belongs is worse than one that is merely a
/// hair too wide.
fn group_thousands(n: usize, lang: &str, style: DigitStyle) -> String {
    let sep = match lang.trim().split('-').next().unwrap_or("") {
        "fr" => "\u{00A0}",
        "de" | "es" | "it" => ".",
        // U+066C ARABIC THOUSANDS SEPARATOR.
        "ar" => "\u{066C}",
        _ => ",",
    };
    let plain = n.to_string();
    let mut grouped = String::with_capacity(plain.len() + plain.len() / 3);
    for (i, ch) in plain.chars().enumerate() {
        if i > 0 && (plain.len() - i).is_multiple_of(3) {
            grouped.push_str(sep);
        }
        grouped.push(ch);
    }
    // Digit style is applied last, so the separator is chosen by language and only the
    // digits themselves change shape.
    match style {
        DigitStyle::Western => grouped,
        DigitStyle::EasternArabic => grouped
            .chars()
            .map(|c| match c {
                '0'..='9' => char::from_u32(0x0660 + (c as u32 - '0' as u32)).unwrap_or(c),
                other => other,
            })
            .collect(),
    }
}

/// Shunn's rounding: to the nearest hundred under ten thousand words, to the nearest
/// thousand above it. The point is that the number is an estimate and should look like
/// one — "about 90,000 words", never "89,417".
///
/// <https://www.shunn.net/format/novel/>
pub fn round_word_count(words: usize) -> usize {
    let step = if words < 10_000 { 100 } else { 1_000 };
    // Never round a real manuscript down to nothing: anything at all is "about `step`".
    let rounded = ((words + step / 2) / step) * step;
    if words > 0 { rounded.max(step) } else { 0 }
}

/// The title page's word-count line, rounded and localized: "about 90,000 words".
pub fn word_count_note(lang: &str, words: usize, style: DigitStyle) -> String {
    let n = group_thousands(round_word_count(words), lang, style);
    match lang.trim().split('-').next().unwrap_or("") {
        "fr" => format!("environ {n} mots"),
        "de" => format!("ca. {n} Wörter"),
        "es" => format!("unas {n} palabras"),
        "it" => format!("circa {n} parole"),
        "ar" => format!("نحو {n} كلمة"),
        "he" => format!("כ-{n} מילים"),
        _ => format!("about {n} words"),
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

/// Fold a title for redundancy comparison: lowercased, interior runs of any whitespace
/// (including the no-break spaces an autocorrect leaves behind) collapsed to one space,
/// and one trailing `.`/`:`/`-`/`—` dropped.
///
/// The trailing-punctuation strip runs *after* the trim, so `"Chapter 3 ."` folds the same
/// way `"Chapter 3."` does.
fn fold_for_comparison(s: &str) -> String {
    let collapsed: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
    let trimmed = collapsed
        .trim_end_matches(['.', ':', '-', '—', '–'])
        .trim_end();
    trimmed.to_lowercase()
}

/// Whether `title` is already saying nothing but this row's own number.
///
/// The writer typed "Chapter 3" as the title because until now the app never showed them
/// that it already knew. Appending a generated "Chapter 3" to that gives the doubled
/// heading this whole feature exists to remove — but only when the two really are the same
/// fact, which is a judgement the old byte-for-byte comparison could not make.
///
/// Recognised as redundant, against `n` and the **row's own** resolved language:
/// the full localized phrase in any case (`"chapter 3"`, `"CHAPTER 3"`), with an interior
/// NBSP or a doubled space, with one trailing `.`/`:`/dash, and the bare numeral
/// (`"3"`, `"3."`) — which is what a writer numbering by hand most often types.
///
/// Deliberately **not** recognised: roman numerals, spelled-out numbers, abbreviations
/// like `"Ch. 3"`, and — most importantly — a title naming a *different* number than the
/// counter computed. Those are not redundancy. A title reading "Chapter 3" on the row the
/// manuscript makes chapter four is a real contradiction between what the writer believes
/// and where the row actually sits, and hiding half of it would hide the disagreement.
/// Showing "Chapter 4 — Chapter 3" is ugly, and it is *supposed* to be: it is the writer's
/// signal that something moved. Silently wrong is worse than visibly wrong.
pub fn is_redundant_number_title(title: &str, lang: &str, level: Level, n: usize) -> bool {
    let folded = fold_for_comparison(title);
    if folded.is_empty() {
        // Blank, or whitespace-only — nothing to append either way. Callers treat this as
        // "no title", so answering `true` keeps the dangling-dash heading from forming.
        return true;
    }
    // Compared in western digits regardless of the preset's digit style: the writer typed
    // their title on a keyboard, and `numbered()` may have rendered "٣".
    let plain = n.to_string();
    folded == fold_for_comparison(&format!("{} {}", word(lang, level), plain))
        || folded == plain
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
    fn bylines_localize_and_fall_back() {
        assert_eq!(by("en-GB"), "by");
        assert_eq!(by("fr"), "par");
        assert_eq!(by("de"), "von");
        assert_eq!(by("tlh"), "by");
    }

    /// Shunn rounds so the number reads as the estimate it is.
    #[test]
    fn word_counts_round_the_way_shunn_asks() {
        // Under ten thousand: nearest hundred.
        assert_eq!(round_word_count(7_432), 7_400);
        assert_eq!(round_word_count(7_450), 7_500);
        // Above: nearest thousand.
        assert_eq!(round_word_count(89_417), 89_000);
        assert_eq!(round_word_count(89_600), 90_000);
    }

    /// A short piece must not round away to "about 0 words"; an empty export still reports
    /// nothing, because there is genuinely nothing.
    #[test]
    fn a_short_manuscript_rounds_up_rather_than_to_nothing() {
        assert_eq!(round_word_count(12), 100);
        assert_eq!(round_word_count(1), 100);
        assert_eq!(round_word_count(0), 0);
    }

    #[test]
    fn word_count_notes_group_and_localize() {
        assert_eq!(
            word_count_note("en", 89_417, DigitStyle::Western),
            "about 89,000 words"
        );
        assert_eq!(
            word_count_note("fr", 89_417, DigitStyle::Western),
            "environ 89\u{00A0}000 mots"
        );
        assert_eq!(
            word_count_note("de", 89_417, DigitStyle::Western),
            "ca. 89.000 Wörter"
        );
        // Eastern digits keep the language's own separator and only change shape.
        assert_eq!(
            word_count_note("ar", 89_417, DigitStyle::EasternArabic),
            "نحو ٨٩\u{066C}٠٠٠ كلمة"
        );
    }

    /// Every shape of "the title is just the number again" the old byte-comparison let
    /// through. The NBSP case is the one that mattered most: it looked de-duplicated to
    /// anyone reading the export, because the character is invisible.
    #[test]
    fn a_title_that_only_restates_the_number_is_redundant() {
        for title in [
            "Chapter 3",
            "chapter 3",
            "CHAPTER 3",
            "Chapter 3.",
            "Chapter 3:",
            "Chapter 3 —",
            "  Chapter 3  ",
            "Chapter\u{00A0}3", // no-break space
            "Chapter  3",       // doubled space
            "3",
            "3.",
        ] {
            assert!(
                is_redundant_number_title(title, "en", Level::Chapter, 3),
                "{title:?} should read as redundant"
            );
        }
        // …and in the row's own language, which is what the writer typed in.
        assert!(is_redundant_number_title(
            "Chapitre 3",
            "fr",
            Level::Chapter,
            3
        ));
        assert!(is_redundant_number_title("Partie 2", "fr", Level::Part, 2));
    }

    /// A real title is never swallowed — including the ones that merely *contain* a
    /// number, and the ones naming a different number than the counter reached.
    #[test]
    fn a_real_title_is_never_treated_as_redundant() {
        for title in [
            "Welcome",
            "Chapter 3 — The Storm",
            "The Storm",
            "Three",     // spelled out: deliberately not recognised
            "III",       // roman: deliberately not recognised
            "Ch. 3",     // abbreviation: deliberately not recognised
            "Chapter 4", // a DIFFERENT number — a contradiction, not a duplicate
            "33",
            "Chapter 33",
        ] {
            assert!(
                !is_redundant_number_title(title, "en", Level::Chapter, 3),
                "{title:?} must survive as a title"
            );
        }
    }

    /// A blank or whitespace-only title behaves as no title at all, so a heading can never
    /// come out as "Chapter 3 —    " with a dangling dash.
    #[test]
    fn a_blank_title_counts_as_no_title() {
        assert!(is_redundant_number_title("", "en", Level::Chapter, 3));
        assert!(is_redundant_number_title("   ", "en", Level::Chapter, 3));
        assert!(is_redundant_number_title(
            "\u{00A0}\t ",
            "en",
            Level::Chapter,
            3
        ));
    }

    /// The writer types western digits whatever the preset renders, so an Eastern-Arabic
    /// export must still recognise its own number written the ordinary way.
    #[test]
    fn digit_style_does_not_defeat_the_guard() {
        assert_eq!(numbered("ar", Level::Chapter, 3, DigitStyle::EasternArabic), "الفصل ٣");
        assert!(is_redundant_number_title(
            "الفصل 3",
            "ar",
            Level::Chapter,
            3
        ));
    }

    #[test]
    fn numbered_heading_is_word_then_number() {
        assert_eq!(
            numbered("fr", Level::Chapter, 3, DigitStyle::Western),
            "Chapitre 3"
        );
        assert_eq!(
            numbered("ar", Level::Chapter, 3, DigitStyle::EasternArabic),
            "الفصل ٣"
        );
    }
}
