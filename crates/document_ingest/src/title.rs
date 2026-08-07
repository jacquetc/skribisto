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
//! silently would be the same mistake Scrivener makes when splitting on the word
//! "Chapter" and deleting it from every title it produced.
//!
//! **Arabic digits and roman numerals only.** Spelled-out numbers ("Chapter
//! Three") are real but a minority, and recognising them well means a
//! per-language word table for every language the app supports — the review tree
//! makes that a one-click fix instead, which is the better trade until there is
//! evidence otherwise.

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

/// Structural words worth recognising at the head of a title, across the
/// languages the app already ships headings for. Matched case-insensitively.
const KEYWORDS: &[&str] = &[
    // en
    "chapter",
    "part",
    "book",
    "volume",
    "prologue",
    "epilogue", // fr
    "chapitre",
    "partie",
    "livre",
    "tome", // de
    "kapitel",
    "teil",
    "buch", // es
    "capítulo",
    "capitulo",
    "parte",
    "libro",    // it
    "capitolo", // pt
    "capítulo",
    "parte", // ru
    "глава",
    "часть", // ar / he keep the app's existing coverage
    "فصل",
    "פרק",
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

    let lower = trimmed.to_lowercase();
    let mut rest = trimmed;
    let mut keyword = None;

    // An optional structural word first.
    for kw in KEYWORDS {
        if lower.starts_with(kw) {
            let after = &trimmed[kw.len()..];
            // Must be a whole word: "Chapterhouse" is a title, not a chapter.
            if after.is_empty() || !after.starts_with(|c: char| c.is_alphanumeric()) {
                keyword = Some((*kw).to_string());
                rest = after.trim_start();
                break;
            }
        }
    }

    let (numeral, after_number) = take_number(rest);

    // A keyword with no number ("Prologue") is a title in its own right, and a
    // heading with neither is an ordinary title.
    numeral?;

    let remaining = after_number
        .trim_start_matches(|c: char| {
            matches!(c, ':' | '.' | ')' | '-' | '—' | '–' | '·' | '|') || c.is_whitespace()
        })
        .trim();

    Some(ExtractedOrdinal {
        remaining_title: remaining.to_string(),
        numeral,
        keyword,
    })
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
}
