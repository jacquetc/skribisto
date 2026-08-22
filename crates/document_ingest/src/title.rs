// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Separating a heading's ordinal from its name.
//!
//! Skribisto renders chapter numbers itself, from the manuscript's own order, and
//! ships a *Tidy chapter titles…* command precisely because a number typed into a
//! title goes wrong the moment anything moves. Import is where such titles
//! arrive by the hundred: `## Chapter 3: The Storm` is the single most common
//! heading shape in the wild.
//!
//! So the ordinal comes off, and both halves are kept — the writer sees what was
//! removed in the review step and can put it back on any row. Removing it
//! silently would be the same mistake other importers make when splitting on the
//! word "Chapter" and deleting it from every title they produce.
//!
//! ## Three ways a number can be written, and one rule that guards them
//!
//! Arabic digits (`Chapter 3`), roman numerals (`III. The Open Road`), and — since
//! the word table arrived with `text2num` — **spelled-out** numbers, cardinal or
//! ordinal, in the seven languages that crate covers: `Chapter Three`,
//! `Chapitre premier`, `Kapitel einundzwanzig`.
//!
//! Spelled-out numbers need a guard the other two do not, because number-words are
//! also ordinary words. `# 7` can only be an ordinal, but `# One Last Thing` is a
//! title and `# Nine Princes in Amber` is a novel. So a spelled-out number is read
//! only when one of two things vouches for it:
//!
//! * **a structural keyword precedes it** — and then the keyword itself says which
//!   language to read the number in (`chapitre` ⇒ French), which is why no caller
//!   has to pass a locale; or
//! * **it is the entire heading** — `# Three`, on the same reasoning that already
//!   makes `# 7` an ordinal rather than a chapter named "7". Every supported
//!   language is tried and they must agree on the value.
//!
//! A leading number-word followed by more title is never stripped. That is the same
//! conservatism `take_number` already applies to `3rd Watch`.
//!
//! **Coverage stops at the word table.** `ru`, `ar` and `he` have keywords here but
//! no `text2num` support, so they read digits and romans only. That is a deliberate
//! stopping point rather than an oversight: Russian chapter ordinals decline
//! (`глава третья`), and Hebrew numbers its chapters with gematria letters (`פרק ג`),
//! which is a numeral system closer to the roman path than to a word list. Guessing
//! at either in a language nobody here can proof-read would be worse than the review
//! tree's one-click fix.

/// What a heading's leading ordinal turned out to be.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedOrdinal {
    /// The title with the ordinal and its separator removed. May be empty, when
    /// the heading was nothing but a number — `## 7` is a chapter marker, not a
    /// chapter called "7".
    pub remaining_title: String,
    /// The number that was found. Informational: the manuscript's own order
    /// decides the printed number, never this.
    pub numeral: Option<u32>,
    /// The structural word that preceded it, lower-cased (`"chapter"`,
    /// `"chapitre"`, `"part"`), when there was one.
    pub keyword: Option<String>,
}

/// A language whose number words [`text2num`] can read.
///
/// Six of the eight it supports; Dutch and Danish are left out on purpose (see
/// [`ALL_WORD_LANGS`]). A keyword in a language it does not cover carries an
/// empty list and still works for digits and roman numerals — see the coverage
/// note in the module doc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WordLang {
    En,
    Fr,
    De,
    Es,
    It,
    Pt,
}

impl WordLang {
    fn interpreter(self) -> text2num::Language {
        match self {
            WordLang::En => text2num::Language::english(),
            WordLang::Fr => text2num::Language::french(),
            WordLang::De => text2num::Language::german(),
            WordLang::Es => text2num::Language::spanish(),
            WordLang::It => text2num::Language::italian(),
            // ⚠ **`portugese`, one `u`, is upstream's own spelling.** It was
            // `portuguese` until `text2num` 2.8.0 renamed it under a *minor*
            // bump, so this line is the whole of what a version bump there
            // breaks — and it breaks it in a build, not at run time, which is
            // the one mercy in it. `.github/typos.toml` allows the word for
            // this call and this call only.
            WordLang::Pt => text2num::Language::portugese(),
        }
    }
}

/// Every language with a word table, for the no-keyword case.
///
/// Dutch and Danish are deliberately absent even though `text2num` supports both:
/// nothing here carries a structural keyword in either, so including them would
/// only widen what a bare heading can be mistaken for, with nothing asking for it.
/// Danish arrived with `text2num` 2.8.0 and is left out for the same reason Dutch
/// always was, rather than because it was not noticed.
const ALL_WORD_LANGS: &[WordLang] = &[
    WordLang::En,
    WordLang::Fr,
    WordLang::De,
    WordLang::Es,
    WordLang::It,
    WordLang::Pt,
];

/// Structural words worth recognising at the head of a title, across the
/// languages the app already ships headings for. Matched case-insensitively.
///
/// Each carries the languages whose number words may follow it, which is how a
/// spelled-out ordinal is read without anyone passing a locale: `chapitre` can only
/// be followed by a French number. Some words are genuinely shared — `parte` is both
/// Spanish and Portuguese, `capítulo` likewise — and those list both; the reader then
/// requires every language that parses to agree on the value, so an ambiguous keyword
/// costs precision only where the languages genuinely disagree.
///
/// An empty list means "recognised as a keyword, but no word table" — the row still
/// strips `Глава 3`, it just cannot read `Глава третья`.
const KEYWORDS: &[(&str, &[WordLang])] = &[
    // en
    ("chapter", &[WordLang::En]),
    ("part", &[WordLang::En]),
    ("book", &[WordLang::En]),
    ("volume", &[WordLang::En]),
    ("prologue", &[WordLang::En]),
    ("epilogue", &[WordLang::En]),
    // fr
    ("chapitre", &[WordLang::Fr]),
    ("partie", &[WordLang::Fr]),
    ("livre", &[WordLang::Fr]),
    ("tome", &[WordLang::Fr]),
    // de
    ("kapitel", &[WordLang::De]),
    ("teil", &[WordLang::De]),
    ("buch", &[WordLang::De]),
    // es / pt — `capítulo` and `parte` are both, so both are offered
    ("capítulo", &[WordLang::Es, WordLang::Pt]),
    ("capitulo", &[WordLang::Es, WordLang::Pt]),
    ("parte", &[WordLang::Es, WordLang::Pt]),
    ("libro", &[WordLang::Es]),
    // it
    ("capitolo", &[WordLang::It]),
    // ru — keyword only; see the module doc on declension
    ("глава", &[]),
    ("часть", &[]),
    // ar / he keep the app's existing coverage, digits and romans only
    ("فصل", &[]),
    ("פרק", &[]),
];

/// Pull a leading ordinal off `heading`, if it has one.
///
/// Recognises `Chapter 3`, `Chapter 3: The Storm`, `3. The Storm`, `III — The
/// Storm`, `7`, and the same with any keyword above. Returns `None` when the
/// heading is an ordinary title, which is the common case and must stay cheap.
pub fn extract_leading_ordinal(heading: &str) -> Option<ExtractedOrdinal> {
    let trimmed = heading.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut rest = trimmed;
    let mut keyword = None;
    let mut keyword_langs: Option<&[WordLang]> = None;

    // An optional structural word first.
    for (kw, langs) in KEYWORDS {
        if let Some(after) = strip_prefix_case_insensitive(trimmed, kw) {
            // Must be a whole word: "Chapterhouse" is a title, not a chapter.
            if after.is_empty() || !after.starts_with(|c: char| c.is_alphanumeric()) {
                keyword = Some((*kw).to_string());
                keyword_langs = Some(langs);
                rest = after.trim_start();
                break;
            }
        }
    }

    let (numeral, after_number) = match take_number(rest) {
        (Some(n), after) => (Some(n), after),
        // No digits and no roman numeral. A spelled-out number may still be here,
        // but only where something vouches for it — see the module doc.
        (None, _) => take_number_words(rest, keyword_langs),
    };

    // A keyword with no number ("Prologue") is a title in its own right, and a
    // heading with neither is an ordinary title.
    numeral?;

    let remaining = after_number.trim_start_matches(is_ordinal_separator).trim();

    Some(ExtractedOrdinal {
        remaining_title: remaining.to_string(),
        numeral,
        keyword,
    })
}

/// `text` with a leading `prefix` removed, matched case-insensitively — or
/// `None` if `text` doesn't start with it.
///
/// Walks `text`'s own characters and accumulates the byte offset from *their*
/// lengths, never `prefix`'s. `&text[prefix.len()..]` looks equivalent and is
/// not: a `prefix` compared against `text.to_lowercase()` matches on lower-cased
/// bytes, but a handful of Unicode compatibility characters change byte length
/// under `to_lowercase` (the Kelvin sign U+212A, 3 bytes, lower-cases to plain
/// ASCII 'k', 1 byte). Slicing `text` at `prefix`'s byte count would then land
/// off one of `text`'s own char boundaries — not just wrong, but a panic.
fn strip_prefix_case_insensitive<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    let mut text_chars = text.chars();
    let mut prefix_chars = prefix.chars();
    let mut consumed = 0usize;
    loop {
        let Some(pc) = prefix_chars.next() else {
            return Some(&text[consumed..]);
        };
        let tc = text_chars.next()?;
        if !tc.to_lowercase().eq(pc.to_lowercase()) {
            return None;
        }
        consumed += tc.len_utf8();
    }
}

/// A leading arabic or roman numeral, and what follows it.
fn take_number(s: &str) -> (Option<u32>, &str) {
    let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    if !digits.is_empty() {
        let rest = &s[digits.len()..];
        // A digit run glued to letters ("3rd Watch") is part of the title.
        if rest.starts_with(|c: char| c.is_alphabetic()) {
            return (None, s);
        }
        return (digits.parse().ok(), rest);
    }

    let roman_len = s
        .chars()
        .take_while(|c| {
            matches!(
                c.to_ascii_uppercase(),
                'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M'
            )
        })
        .map(char::len_utf8)
        .sum::<usize>();
    if roman_len == 0 {
        return (None, s);
    }
    let (roman, rest) = s.split_at(roman_len);
    // Only a roman numeral standing alone as a token counts. Without this, "Mine"
    // begins with M and "Ill met by moonlight" begins with I-l-l.
    if rest.starts_with(|c: char| c.is_alphanumeric()) {
        return (None, s);
    }
    match roman_value(roman) {
        Some(n) => (Some(n), rest),
        None => (None, s),
    }
}

/// The most word-tokens a spelled-out number may occupy.
///
/// Four covers everything a chapter heading realistically reaches:
/// `quatre-vingt-dix-neuf` is one hyphenated token, `eighty five` is two, and the
/// longest ordinary form in the covered languages — Spanish `ciento veinticinco mil`
/// — is three. The cap is what stops the reader walking an entire title looking for
/// a number that is not there.
const MAX_NUMBER_WORDS: usize = 4;

/// A leading *spelled-out* number, and what follows it — or `(None, s)`.
///
/// `langs` is the candidate set: `Some(langs)` when a structural keyword named them
/// (and `Some(&[])` when the keyword's language has no word table, which reads as "no"),
/// `None` when there was no keyword at all.
///
/// The rule this enforces is the module doc's: with a keyword, the number may be
/// followed by the rest of the title; without one, the number must **be** the whole
/// heading. `One Last Thing` and `Nine Princes in Amber` are titles, and no amount of
/// language support should make them chapters.
fn take_number_words<'a>(s: &'a str, langs: Option<&[WordLang]>) -> (Option<u32>, &'a str) {
    let (candidates, must_be_whole) = match langs {
        Some([]) => return (None, s),
        Some(langs) => (langs, false),
        None => (ALL_WORD_LANGS, true),
    };

    let ends = leading_word_token_ends(s);
    // Longest first: "twenty one" is 21, not 20 followed by a title called "one".
    for end in ends.into_iter().rev() {
        let candidate = &s[..end];
        // A single letter is never a number word. Without this, English "a" reads as
        // one in the crate's vocabulary, and "A Study in Scarlet" would lose its "A".
        if candidate.chars().count() < 2 {
            continue;
        }
        let Some(value) = read_in_every_language(candidate, candidates) else {
            continue;
        };
        let after = &s[end..];
        if must_be_whole && !after.trim_matches(is_ordinal_separator).trim().is_empty() {
            // A number-word opening a longer title. Not an ordinal.
            return (None, s);
        }
        return (Some(value), after);
    }
    (None, s)
}

/// `candidate`'s value when every candidate language that can read it agrees — or
/// `None` when none can, or when two disagree.
///
/// The agreement rule matters for the genuinely shared keywords (`parte`, `capítulo`):
/// where Spanish and Portuguese read a word the same way the answer is safe, and where
/// they would differ, saying nothing is better than picking one at random.
fn read_in_every_language(candidate: &str, langs: &[WordLang]) -> Option<u32> {
    let mut agreed: Option<u32> = None;
    for lang in langs {
        let Ok(digits) = text2num::text2digits(candidate, &lang.interpreter()) else {
            continue;
        };
        let Some(value) = leading_value(&digits) else {
            continue;
        };
        match agreed {
            None => agreed = Some(value),
            Some(seen) if seen == value => {}
            Some(_) => return None,
        }
    }
    agreed
}

/// The number in one of `text2num`'s rendered forms.
///
/// It returns the *formatted* value, not a bare integer, and the format carries the
/// language's own ordinal morphology: `premier` comes back as `"1er"`, `first` as
/// `"1st"`, German `erste` as `"1."`, and Spanish/Italian/Portuguese `primero`/`primo`/
/// `primeiro` as `"1º"`. Only the leading digit run is the number; parsing the whole
/// string would silently reject every ordinal and quietly leave the feature reading
/// cardinals alone.
fn leading_value(rendered: &str) -> Option<u32> {
    let digits: String = rendered.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

/// Byte offsets at which each of the leading word tokens ends, up to
/// [`MAX_NUMBER_WORDS`].
///
/// A token is a run of alphabetic characters, hyphens or apostrophes — hyphens
/// included so `quatre-vingt-dix-neuf` and `einundzwanzig`'s hyphenated cousins stay
/// one token. Scanning stops at the first character that is neither a token character
/// nor a single separating space, so a heading's punctuation (`Three: The Storm`)
/// bounds the run without being read as part of it.
fn leading_word_token_ends(s: &str) -> Vec<usize> {
    let is_token_char = |c: char| c.is_alphabetic() || c == '-' || c == '\'' || c == '\u{2019}';

    let mut ends = Vec::new();
    let mut offset = 0usize;
    let mut chars = s.char_indices().peekable();

    while ends.len() < MAX_NUMBER_WORDS {
        let start = offset;
        while let Some(&(i, c)) = chars.peek() {
            if is_token_char(c) {
                offset = i + c.len_utf8();
                chars.next();
            } else {
                break;
            }
        }
        if offset == start {
            break; // not a token character — nothing more to take
        }
        ends.push(offset);

        // Exactly one space may separate two tokens of the same number.
        match chars.peek() {
            Some(&(i, ' ')) => {
                offset = i + 1;
                chars.next();
            }
            _ => break,
        }
    }
    ends
}

/// The punctuation that may sit between an ordinal and the title it introduces.
fn is_ordinal_separator(c: char) -> bool {
    matches!(c, ':' | '.' | ')' | '-' | '—' | '–' | '·' | '|') || c.is_whitespace()
}

/// Strict roman-numeral value, or `None` if `s` is not one.
fn roman_value(s: &str) -> Option<u32> {
    let value = |c: char| match c.to_ascii_uppercase() {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    };
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return None;
    }
    let mut total = 0u32;
    for (i, c) in chars.iter().enumerate() {
        let v = value(*c);
        if v == 0 {
            return None;
        }
        let next = chars.get(i + 1).map(|c| value(*c)).unwrap_or(0);
        if v < next {
            total = total.checked_sub(v)?;
        } else {
            total = total.checked_add(v)?;
        }
    }
    // Reject a lone "I" only when it is more plausibly the English pronoun; a
    // heading of exactly "I" is a chapter number in a great many novels, so it
    // stays. Values of zero cannot happen given the table above.
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract(s: &str) -> Option<(String, Option<u32>)> {
        extract_leading_ordinal(s).map(|e| (e.remaining_title, e.numeral))
    }

    #[test]
    fn a_keyword_and_a_number_come_off_together() {
        assert_eq!(
            extract("Chapter 3: The Storm"),
            Some(("The Storm".into(), Some(3)))
        );
        assert_eq!(
            extract("Chapitre 12 — La tempête"),
            Some(("La tempête".into(), Some(12)))
        );
        assert_eq!(extract("Kapitel 4"), Some((String::new(), Some(4))));
    }

    #[test]
    fn a_bare_number_is_an_ordinal_not_a_title() {
        assert_eq!(extract("7"), Some((String::new(), Some(7))));
        assert_eq!(extract("3. The Storm"), Some(("The Storm".into(), Some(3))));
    }

    #[test]
    fn roman_numerals_are_recognised() {
        assert_eq!(
            extract("III. The Open Road"),
            Some(("The Open Road".into(), Some(3)))
        );
        assert_eq!(extract("XIV"), Some((String::new(), Some(14))));
    }

    #[test]
    fn an_ordinary_title_is_left_entirely_alone() {
        for title in [
            "The Storm",
            "Prologue",
            "Chapterhouse",
            "3rd Watch",
            "Ill met by moonlight",
            "Mine",
            "A Study in Scarlet",
        ] {
            assert_eq!(extract(title), None, "{title:?} should not be an ordinal");
        }
    }

    #[test]
    fn the_keyword_is_reported_so_the_writer_can_see_what_went() {
        let e = extract_leading_ordinal("Chapter 3: The Storm").unwrap();
        assert_eq!(e.keyword.as_deref(), Some("chapter"));
        assert_eq!(e.numeral, Some(3));
    }

    #[test]
    fn a_keyword_and_a_spelled_out_number_come_off_together() {
        assert_eq!(
            extract("Chapter Three: The Storm"),
            Some(("The Storm".into(), Some(3)))
        );
        assert_eq!(
            extract("Chapitre premier"),
            Some((String::new(), Some(1))),
            "the standard French form for chapter one is a word, not a digit"
        );
        assert_eq!(
            extract("Kapitel einundzwanzig"),
            Some((String::new(), Some(21)))
        );
        assert_eq!(
            extract("Capitolo dodici — La tempesta"),
            Some(("La tempesta".into(), Some(12)))
        );
    }

    /// Multi-token numbers must be read whole. Taking the longest match is what stops
    /// "twenty one" becoming chapter 20 with a title called "one".
    #[test]
    fn a_spelled_out_number_spanning_several_words_is_read_whole() {
        assert_eq!(
            extract("Chapter twenty one"),
            Some((String::new(), Some(21)))
        );
        assert_eq!(
            extract("Chapitre quatre-vingt-dix-neuf"),
            Some((String::new(), Some(99)))
        );
    }

    /// The same reasoning that already makes a bare `7` an ordinal.
    #[test]
    fn a_heading_that_is_only_a_spelled_out_number_is_an_ordinal() {
        assert_eq!(extract("Three"), Some((String::new(), Some(3))));
        assert_eq!(extract("Trois"), Some((String::new(), Some(3))));
    }

    /// The guard the whole spelled-out path hangs on. Number words are ordinary words,
    /// and without a keyword vouching for them a leading one is part of the title.
    #[test]
    fn a_spelled_out_number_starting_a_real_title_is_left_alone() {
        for title in [
            "One Last Thing",
            "Three Musketeers",
            "Nine Princes in Amber",
            "Two Towers",
            "Un homme et son péché",
            "Cent ans de solitude",
        ] {
            assert_eq!(
                extract(title),
                None,
                "{title:?} is a title, not an ordinal and a title"
            );
        }
    }

    /// No caller passes a locale, because the keyword already is one.
    #[test]
    fn the_keyword_decides_which_language_reads_the_number() {
        // "drei" is German for three, and means nothing in English.
        assert_eq!(extract("Kapitel drei"), Some((String::new(), Some(3))));
        assert_eq!(
            extract("Chapter drei"),
            None,
            "an English keyword must not reach for the German word table"
        );
    }

    /// `leading_value` exists because `text2num` renders a number the way its language
    /// writes it, ordinal morphology and all. Pin the real forms, per language, against
    /// the real crate: if an upgrade ever returned a bare integer — or a differently
    /// decorated one — every ordinal would silently stop being recognised, and nothing
    /// else in the suite would notice.
    #[test]
    fn an_ordinal_is_read_through_whatever_suffix_its_language_renders() {
        use text2num::{Language, text2digits};

        let cases: &[(Language, &str, u32)] = &[
            (Language::french(), "premier", 1),
            (Language::french(), "première", 1),
            (Language::english(), "first", 1),
            (Language::german(), "erste", 1),
            (Language::spanish(), "primero", 1),
            (Language::italian(), "primo", 1),
            (Language::portugese(), "primeiro", 1),
        ];

        for (lang, word, expected) in cases {
            let rendered = text2digits(word, lang).unwrap_or_else(|e| {
                panic!("{word:?} must still read as a number, got {e:?}");
            });
            assert!(
                rendered.starts_with(char::is_numeric),
                "{word:?} rendered as {rendered:?}, which starts with no digit at all"
            );
            assert_eq!(
                leading_value(&rendered),
                Some(*expected),
                "{word:?} rendered as {rendered:?}"
            );
        }

        // And a plain cardinal, which carries no suffix to strip.
        assert_eq!(leading_value("99"), Some(99));
    }

    /// Russian, Arabic and Hebrew keywords keep working for the two numeral systems
    /// that need no word table — the coverage line the module doc draws.
    #[test]
    fn a_language_with_no_word_table_still_reads_digits_and_romans() {
        assert_eq!(extract("Глава 3"), Some((String::new(), Some(3))));
        assert_eq!(extract("פרק 12"), Some((String::new(), Some(12))));
        assert_eq!(extract("فصل 7"), Some((String::new(), Some(7))));
        assert_eq!(
            extract("Глава третья"),
            None,
            "a declined Russian ordinal is out of scope, and must not be half-read"
        );
    }

    /// A heading beginning with a Unicode compatibility character whose
    /// lower-cased form is shorter than its own encoding (the Kelvin sign
    /// U+212A, 3 UTF-8 bytes, lower-cases to plain ASCII 'k', 1 byte) used to
    /// slice `trimmed` at a byte offset measured on the *lower-cased* copy —
    /// which does not land on one of `trimmed`'s own char boundaries and can
    /// panic. It must not, whether or not it happens to read as a keyword.
    #[test]
    fn a_heading_with_a_length_changing_unicode_case_fold_does_not_panic() {
        let kelvin = '\u{212A}'; // KELVIN SIGN, lower-cases to ASCII 'k'
        let heading = format!("{kelvin}apitel 3: Der Sturm");
        // Must not panic; whatever it decides is secondary to that.
        let _ = extract_leading_ordinal(&heading);

        let angstrom = '\u{212B}'; // ANGSTROM SIGN, lower-cases to ASCII 'å'
        let heading = format!("{angstrom}ngstrom Readings");
        let _ = extract_leading_ordinal(&heading);
    }
}
