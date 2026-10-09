// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The manuscript's language, which Scrivener never records as such.
//!
//! No Scrivener file stores a spelling language (D4-15, checked again by its critique over
//! every settings file). Two things come near, and each is a proposal the writer confirms on
//! the Save page, never a fact:
//!
//! * **A compile language the writer changed.** Every compile file carries one, and in the
//!   corpus every one is the value Scrivener ships (`en` in `compile.xml` and
//!   `compile.plist`, `English` in `compile.ini`), including in French, Spanish and Chinese
//!   projects. So the shipped value says nothing; any other value is the writer's choice,
//!   and proposed.
//! * **The script the manuscript is written in** (I3 A8). Not a statistical guess at the
//!   language: when more than three letters in ten of the Draft are Chinese characters and
//!   kana, the manuscript is Japanese if one in ten of them at least is kana (Japanese
//!   writes its grammar in kana; Chinese holds a few only when it quotes Japanese) and
//!   Chinese otherwise; Hangul, Hebrew and Arabic likewise propose Korean, Hebrew and
//!   Arabic. These decide how Skribisto counts words and quotes, where a Latin-script
//!   default would be wrong. Latin, Cyrillic and the rest are shared by too many languages
//!   to name one.
//!
//! When neither says anything, the import proposes nothing and the interface fills in its
//! own language, as New Work does.

use crate::content::Letters;
use crate::model::{CompileFile, CompileLanguage};

/// The share of the Draft's letters a script must pass for its language to be proposed.
const SCRIPT_SHARE: f64 = 0.3;

/// The share of the Chinese characters and kana that kana must reach for the manuscript to
/// be Japanese rather than Chinese.
const KANA_SHARE: f64 = 0.1;

/// The first language a compile file states that is not the one Scrivener ships, as a
/// language tag.
pub(crate) fn from_compile(languages: &[CompileLanguage]) -> Option<String> {
    languages.iter().find_map(|stated| match stated.file {
        CompileFile::Xml | CompileFile::Plist => {
            let tag = canonical_tag(&stated.value)?;
            (!tag.eq_ignore_ascii_case("en")).then_some(tag)
        }
        CompileFile::Ini => {
            let name = stated.value.trim();
            if name.eq_ignore_ascii_case("English") {
                return None;
            }
            QT_LANGUAGE_NAMES
                .iter()
                .find(|(qt, _)| qt.eq_ignore_ascii_case(name))
                .map(|(_, tag)| (*tag).to_string())
        }
    })
}

/// The language the script of the Draft's letters names, when one script passes the share.
pub(crate) fn from_scripts(letters: &Letters) -> Option<&'static str> {
    let total = letters.total();
    if total == 0 {
        return None;
    }
    let share = |n: u32| f64::from(n) / total as f64;
    let cjk = letters.han.saturating_add(letters.kana);
    // Japanese prose writes its grammar in kana, a third of its CJK letters or more even
    // where kanji are densest; Chinese prose holds a few only when it quotes Japanese.
    let japanese = f64::from(letters.kana) >= f64::from(cjk) * KANA_SHARE;
    let candidates = [
        (share(cjk), if japanese { "ja" } else { "zh" }),
        (share(letters.hangul), "ko"),
        (share(letters.hebrew), "he"),
        (share(letters.arabic), "ar"),
    ];
    candidates
        .into_iter()
        .filter(|(s, _)| *s > SCRIPT_SHARE)
        .max_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, tag)| tag)
}

/// A language tag as written in a compile file, in its canonical case (`pt_br` →
/// `pt-BR`, `ZH-hans` → `zh-Hans`), or `None` when it is not one: a primary language of two
/// or three letters, then subtags of one to eight letters and digits.
pub(crate) fn canonical_tag(raw: &str) -> Option<String> {
    let raw = raw.trim().replace('_', "-");
    let mut parts = raw.split('-');
    let primary = parts.next()?;
    if !(2..=3).contains(&primary.len()) || !primary.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let mut out = primary.to_ascii_lowercase();
    for part in parts {
        if part.is_empty() || part.len() > 8 || !part.chars().all(|c| c.is_ascii_alphanumeric()) {
            return None;
        }
        out.push('-');
        if part.len() == 2 && part.chars().all(|c| c.is_ascii_alphabetic()) {
            out.push_str(&part.to_ascii_uppercase());
        } else if part.len() == 4 && part.chars().all(|c| c.is_ascii_alphabetic()) {
            let lower = part.to_ascii_lowercase();
            let mut chars = lower.chars();
            if let Some(first) = chars.next() {
                out.push(first.to_ascii_uppercase());
                out.extend(chars);
            }
        } else {
            out.push_str(&part.to_ascii_lowercase());
        }
    }
    Some(out)
}

/// The English names Qt gives languages (`QLocale::languageToString`), as Scrivener for
/// Windows 1.x writes them into `compile.ini`, with their language tags: the one-word names
/// of the languages writers most often compile in. The corpus only ever holds `English`, so
/// the other names are Qt's, not observed; a name not listed proposes nothing.
const QT_LANGUAGE_NAMES: &[(&str, &str)] = &[
    ("Arabic", "ar"),
    ("Catalan", "ca"),
    ("Chinese", "zh"),
    ("Czech", "cs"),
    ("Danish", "da"),
    ("Dutch", "nl"),
    ("Finnish", "fi"),
    ("French", "fr"),
    ("German", "de"),
    ("Greek", "el"),
    ("Hebrew", "he"),
    ("Hungarian", "hu"),
    ("Italian", "it"),
    ("Japanese", "ja"),
    ("Korean", "ko"),
    ("Polish", "pl"),
    ("Portuguese", "pt"),
    ("Romanian", "ro"),
    ("Russian", "ru"),
    ("Spanish", "es"),
    ("Swedish", "sv"),
    ("Turkish", "tr"),
    ("Ukrainian", "uk"),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn stated(file: CompileFile, value: &str) -> CompileLanguage {
        CompileLanguage {
            file,
            value: value.to_string(),
        }
    }

    #[test]
    fn the_language_scrivener_ships_says_nothing() {
        assert_eq!(from_compile(&[stated(CompileFile::Xml, "en")]), None);
        assert_eq!(from_compile(&[stated(CompileFile::Plist, " EN ")]), None);
        assert_eq!(from_compile(&[stated(CompileFile::Ini, "English")]), None);
        assert_eq!(from_compile(&[]), None);
    }

    #[test]
    fn a_language_the_writer_chose_is_proposed() {
        assert_eq!(
            from_compile(&[stated(CompileFile::Xml, "fr")]).as_deref(),
            Some("fr")
        );
        assert_eq!(
            from_compile(&[stated(CompileFile::Xml, "en_GB")]).as_deref(),
            Some("en-GB"),
            "a region the writer chose is a choice too"
        );
        assert_eq!(
            from_compile(&[
                stated(CompileFile::Xml, "en"),
                stated(CompileFile::Plist, "pt_br")
            ])
            .as_deref(),
            Some("pt-BR"),
            "the shipped value in one file does not hide a choice in another"
        );
        assert_eq!(
            from_compile(&[stated(CompileFile::Ini, "French")]).as_deref(),
            Some("fr")
        );
        assert_eq!(from_compile(&[stated(CompileFile::Ini, "Klingon")]), None);
        assert_eq!(from_compile(&[stated(CompileFile::Xml, "français")]), None);
    }

    #[test]
    fn tags_come_out_in_their_canonical_case() {
        assert_eq!(canonical_tag("ZH-hans-cn").as_deref(), Some("zh-Hans-CN"));
        assert_eq!(canonical_tag("de").as_deref(), Some("de"));
        assert_eq!(canonical_tag("e"), None);
        assert_eq!(canonical_tag("en--US"), None);
        assert_eq!(canonical_tag("<$lang>"), None);
    }

    #[test]
    fn a_script_names_a_language_only_past_its_share() {
        let chinese = Letters {
            han: 400,
            latin: 100,
            ..Letters::default()
        };
        assert_eq!(from_scripts(&chinese), Some("zh"));
        let japanese = Letters {
            han: 200,
            kana: 250,
            latin: 50,
            ..Letters::default()
        };
        assert_eq!(from_scripts(&japanese), Some("ja"));
        let hebrew = Letters {
            hebrew: 80,
            latin: 20,
            ..Letters::default()
        };
        assert_eq!(from_scripts(&hebrew), Some("he"));
        let arabic = Letters {
            arabic: 40,
            latin: 60,
            ..Letters::default()
        };
        assert_eq!(from_scripts(&arabic), Some("ar"));
        let korean = Letters {
            hangul: 900,
            han: 30,
            ..Letters::default()
        };
        assert_eq!(from_scripts(&korean), Some("ko"));
        // A thesis quoting a little Chinese is not in Chinese; Latin names no language.
        let quoting = Letters {
            han: 20,
            latin: 1000,
            ..Letters::default()
        };
        assert_eq!(from_scripts(&quoting), None);
        assert_eq!(from_scripts(&Letters::default()), None);
        let russian = Letters {
            other: 1000,
            ..Letters::default()
        };
        assert_eq!(from_scripts(&russian), None);
    }

    /// A Chinese novel that quotes a Japanese name, or writes の as Chinese web prose
    /// does, is still Chinese: kana name Japanese when they are a real share of the
    /// Chinese characters and kana, as in any Japanese prose, however dense in kanji.
    #[test]
    fn a_few_kana_in_chinese_prose_do_not_make_it_japanese() {
        let chinese = Letters {
            han: 80_000,
            kana: 40,
            latin: 500,
            ..Letters::default()
        };
        assert_eq!(from_scripts(&chinese), Some("zh"));
        let dense_japanese = Letters {
            han: 6_000,
            kana: 4_000,
            ..Letters::default()
        };
        assert_eq!(from_scripts(&dense_japanese), Some("ja"));
        let katakana_heavy = Letters {
            han: 100,
            kana: 900,
            ..Letters::default()
        };
        assert_eq!(from_scripts(&katakana_heavy), Some("ja"));
    }
}
