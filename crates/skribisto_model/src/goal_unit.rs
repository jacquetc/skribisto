// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which unit a new project should count its targets in, from the language it is written
//! in.
//!
//! A suggestion, not a rule: it seeds the picker in the New Work panel and the writer can
//! change it there or later. It exists because the alternative — everyone starts in words —
//! is wrong for a large part of the world in a way that is tedious to notice and fix, and
//! because no other tool in this field adapts the unit to the manuscript at all.

pub use common::entities::GoalUnit;

/// The unit to start a project in, from the primary subtag of its writing language.
///
/// Unlisted or unset languages get [`GoalUnit::Words`].
pub fn default_unit_for_language(tag: &str) -> GoalUnit {
    // The primary subtag only, lowercased: `zh-Hans-CN` and `ZH` are the same answer.
    let primary = tag
        .trim()
        .split(['-', '_'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    match primary.as_str() {
        // No inter-word spacing at all, and character count is the native unit of both
        // publishing traditions: Japanese counts 原稿用紙 sheets of 400 characters, Chinese
        // pays and measures by the 千字, the thousand characters.
        "zh" | "ja" => GoalUnit::Characters,
        // Korean *is* space-delimited, so a word count is computable — which is why
        // `counting::is_cjk_language` deliberately excludes it from CJK counting. It is
        // here anyway because 원고지 manuscript paper and the major web-fiction platforms
        // both report characters, so that is what a Korean writer expects to be asked for.
        // The weakest row in this table, and the one most likely to want revisiting.
        "ko" => GoalUnit::Characters,
        // Thai writes with no spaces between words at all, so a word count needs
        // dictionary-based segmentation that nothing in this workspace does. Characters is
        // as much "the only number we can honestly produce" as it is a convention.
        "th" => GoalUnit::Characters,
        // Several languages measure prose in characters *by convention* and are still
        // defaulted to words here, deliberately:
        //   de  Normseite, about 1500 characters including spaces
        //   fr  feuillet,  1500 signes espaces compris
        // Both are units of *delivered pages*, quoted in publishing and translation
        // contracts at the end of the work. A drafting target is still set in words, and a
        // writer who wants the contract unit switches it per project in
        // Settings ▸ Work ▸ Structure. Contrast zh/ja/ko/th above, where characters are not
        // a convention layered on top of words but the only unit the language really has.
        //
        // Vietnamese (`vi`) is here too, and is the other row worth knowing about: it puts
        // a space between every *syllable*, not every word, so its space-delimited count
        // reads high against true lexical words. It stays on words because that is the
        // number Vietnamese word processors report and call "từ" — matching expectation
        // beats linguistic precision when the writer is comparing against another tool.
        _ => GoalUnit::Words,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_scripts_with_no_word_spacing_start_in_characters() {
        for tag in ["ja", "ja-JP", "zh", "zh-Hans-CN", "ZH", "ko", "ko_KR", "th"] {
            assert_eq!(
                default_unit_for_language(tag),
                GoalUnit::Characters,
                "{tag} should start in characters"
            );
        }
    }

    #[test]
    fn everything_else_starts_in_words() {
        for tag in [
            "en-US", "fr-FR", "de-DE", "es", "ru", "ar", "he", "vi", "pl",
        ] {
            assert_eq!(
                default_unit_for_language(tag),
                GoalUnit::Words,
                "{tag} should start in words"
            );
        }
    }

    /// A project with no language set is a project with nothing to infer from.
    #[test]
    fn an_unset_or_unknown_language_starts_in_words() {
        assert_eq!(default_unit_for_language(""), GoalUnit::Words);
        assert_eq!(default_unit_for_language("   "), GoalUnit::Words);
        assert_eq!(default_unit_for_language("xx-YY"), GoalUnit::Words);
    }

    /// The region subtag must not change the answer, and neither must the separator a
    /// legacy tag used.
    #[test]
    fn the_region_and_the_separator_are_ignored() {
        assert_eq!(
            default_unit_for_language("zh_TW"),
            default_unit_for_language("zh-TW")
        );
        assert_eq!(
            default_unit_for_language("ja-JP-u-ca-japanese"),
            GoalUnit::Characters
        );
    }
}
