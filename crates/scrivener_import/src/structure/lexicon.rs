// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The words the structure mapping recognises: section-type names, titles that
//! name their level, and the closed lists the design names (default Draft names,
//! story-bible folders, front and back matter, placeholder titles).
//!
//! Every list is closed and every match is anchored: a title claims a level only
//! when it *starts* with a level word followed by a number ("Chapter 7",
//! "Chapitre sept", "Part II", "Kapitel vier"), or when it is one of a few whole
//! titles ("Title Page", "Appendices"). "Chapters" (a folder of chapters) and
//! "Bibliography Management" (a manual's section) claim nothing. A word the lists
//! lack falls back to the tree's shape, which is safe (no row lost, no invalid
//! row) but coarser; the review's per-depth controls are the escape hatch.
//!
//! The words are the union of what the measured corpus holds (English, French,
//! Spanish, Portuguese, German, Italian, Chinese) and their plain translations.
//! Numbers are recognised as digits, as a canonical roman numeral, or as number
//! words, cardinal or ordinal, compounds included ("thirty-five", "vingt et un",
//! "veintiuno", "einundzwanzig", "ventitré"), so that one sibling group never
//! gets mixed answers because the list stopped at twenty-nine.

use unicode_normalization::UnicodeNormalization;

/// What a section-type name says about the level of the items wearing it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SectionClass {
    Book,
    Part,
    Chapter,
    Scene,
    Front,
    Back,
    /// Standalone text outside the flow: an introduction, a plate.
    Paratext,
    Notes,
    /// A prologue, epilogue or interlude: a chapter that is not numbered.
    Prologue,
}

/// What a title says about its level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TitleClass {
    Book,
    Part,
    Chapter,
    Scene,
    Front,
    Back,
}

/// A title that names its level, with what the sibling rules compare.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TitleMatch {
    pub(crate) class: TitleClass,
    /// The level word as written, lower-cased: siblings are grouped by it.
    pub(crate) leading: String,
    /// The number, normalised so two spellings of one value compare equal
    /// when both are digits or both roman; `None` for a whole-title match.
    pub(crate) number: Option<String>,
}

/// Which side of the book a root folder of matter belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MatterSide {
    Front,
    Back,
}

/// The form every comparison is made on: compatibility-normalised, lower-cased,
/// leading symbols dropped (a title may start with an emoji or a bullet), inner
/// whitespace collapsed.
pub(crate) fn norm(text: &str) -> String {
    trimmed(&text.nfkc().collect::<String>().to_lowercase())
}

/// [`norm`] with the letters' case kept: what the few rules that read case look at.
fn norm_cased(text: &str) -> String {
    trimmed(&text.nfkc().collect::<String>())
}

/// Leading symbols dropped, inner whitespace collapsed.
fn trimmed(folded: &str) -> String {
    let start = folded
        .char_indices()
        .find(|(_, c)| c.is_alphanumeric())
        .map_or(folded.len(), |(i, _)| i);
    let mut out = String::with_capacity(folded.len());
    let mut space = false;
    for c in folded[start..].chars() {
        if c.is_whitespace() {
            space = true;
        } else {
            if space && !out.is_empty() {
                out.push(' ');
            }
            space = false;
            out.push(c);
        }
    }
    out
}

/// The words of a normalised text: maximal runs of letters, or of digits.
/// Everything else separates, so "Chapter-01", "Ch.3" and "chapter1" all read
/// as a level word then a number.
fn tokens(normalised: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start: Option<(usize, bool)> = None;
    for (i, c) in normalised.char_indices() {
        let kind = if c.is_numeric() {
            Some(true)
        } else if c.is_alphabetic() {
            Some(false)
        } else {
            None
        };
        match (start, kind) {
            (Some((_, digits)), Some(k)) if digits == k => {}
            (Some((s, _)), Some(k)) => {
                out.push(&normalised[s..i]);
                start = Some((i, k));
            }
            (Some((s, _)), None) => {
                out.push(&normalised[s..i]);
                start = None;
            }
            (None, Some(k)) => start = Some((i, k)),
            (None, None) => {}
        }
    }
    if let Some((s, _)) = start {
        out.push(&normalised[s..]);
    }
    out
}

// ---------------------------------------------------------------- section types

/// Classify a Scrivener 3 section-type name. Section-type names are free text a
/// template or the writer typed, localised by the template, never by Scrivener:
/// "Chapter Heading", "Titre de chapitre", "H1 Capítulo". Generic names
/// ("Section", "Heading", "Text", "Level 2") and "N/A" claim nothing.
pub(crate) fn section_class(name: &str) -> Option<SectionClass> {
    let n = norm(name);
    let base = n
        .strip_suffix(" heading")
        .or_else(|| n.strip_suffix(" title"))
        .unwrap_or(&n);
    for (class, names) in SECTION_NAMES {
        if names.contains(&base) || names.contains(&n.as_str()) {
            return Some(*class);
        }
    }
    None
}

const SECTION_NAMES: &[(SectionClass, &[&str])] = &[
    (
        SectionClass::Prologue,
        &["prologue", "epilogue", "interlude", "prólogo", "epílogo"],
    ),
    (
        SectionClass::Book,
        &[
            "book",
            "livre",
            "titre du livre",
            "buch",
            "libro",
            "livro",
            "volume",
            "tome",
        ],
    ),
    (
        SectionClass::Part,
        &[
            "part",
            "act",
            "partie",
            "titre de partie",
            "teil",
            "parte",
            "acte",
            "akt",
            "acto",
            "ato",
            "atto",
        ],
    ),
    (
        SectionClass::Chapter,
        &[
            "chapter",
            "chapter + toc",
            "chapitre",
            "titre de chapitre",
            "kapitel",
            "capítulo",
            "capitulo",
            "capitolo",
            "hoofdstuk",
            "h1 capítulo",
            "h1 capitulo",
            "章",
            "章节",
        ],
    ),
    (
        SectionClass::Scene,
        &["scene", "scène", "szene", "escena", "cena", "scena", "场景"],
    ),
    (
        SectionClass::Front,
        &[
            "front matter",
            "frontmatter",
            "title page",
            "half title",
            "copyright",
            "dedication",
            "epigraph",
            "foreword",
            "preface",
            "préface",
            "prefacio",
            "vorwort",
            "avant-propos",
            "dédicace",
            "widmung",
            "metadata",
            "toc",
            "table of contents",
            "contents",
        ],
    ),
    (
        SectionClass::Back,
        &[
            "back matter",
            "backmatter",
            "appendix",
            "appendices",
            "appendice",
            "annexe",
            "anhang",
            "apéndice",
            "apendice",
            "afterword",
            "postface",
            "acknowledgments",
            "acknowledgements",
            "remerciements",
            "bibliography",
            "bibliographie",
            "references",
            "glossary",
            "index",
            "about the author",
            "endnotes",
        ],
    ),
    (
        SectionClass::Paratext,
        &["standalone", "intro", "introduction"],
    ),
    (
        SectionClass::Notes,
        &[
            "note",
            "notes",
            "notizen",
            "research",
            "character sketch",
            "setting sketch",
        ],
    ),
];

// ---------------------------------------------------------------- titles

const BOOK_WORDS: &[&str] = &[
    "book", "livre", "buch", "libro", "livro", "volume", "vol", "tome", "tomo",
];
const PART_WORDS: &[&str] = &[
    "part", "partie", "teil", "parte", "act", "acte", "akt", "acto", "ato", "atto",
];
const CHAPTER_WORDS: &[&str] = &[
    "chapter",
    "chapitre",
    "chap",
    "ch",
    "kapitel",
    "capítulo",
    "capitulo",
    "capitolo",
    "hoofdstuk",
];
const SCENE_WORDS: &[&str] = &["scene", "scène", "szene", "escena", "cena", "scena"];

/// Whole titles that are front matter.
const FRONT_TITLES: &[&str] = &[
    "title page",
    "front matter",
    "copyright",
    "dedication",
    "dédicace",
    "preface",
    "préface",
    "foreword",
    "avant-propos",
    "epigraph",
    "épigraphe",
    "contents",
    "table of contents",
    "table des matières",
    "sommaire",
    "inhalt",
    "inhaltsverzeichnis",
    "índice",
    "sumário",
    "half title",
];

/// Whole titles that are back matter. Anchored at both ends, so "Bibliography
/// Management" (a section of a manual) is not one.
const BACK_TITLES: &[&str] = &[
    "appendices",
    "annexes",
    "afterword",
    "postface",
    "acknowledgements",
    "acknowledgments",
    "remerciements",
    "bibliography",
    "bibliographie",
    "glossary",
    "about the author",
    "back matter",
    "endnotes",
];

/// Words that open an appendix title, alone or with a label ("Appendix A",
/// "Appendix-1", "Annexe II : données").
const APPENDIX_WORDS: &[&str] = &["appendix", "annexe", "anhang", "apéndice", "apêndice"];

/// Titles that start with one of these are prologues: they claim no level, and
/// a chapter so titled is left out of the numbering.
const PROLOGUE_WORDS: &[&str] = &[
    "prologue",
    "prolog",
    "prólogo",
    "prologo",
    "epilogue",
    "épilogue",
    "epilog",
    "epílogo",
    "epilogo",
    "interlude",
    "interludio",
    "intermezzo",
];

fn level_of(word: &str) -> Option<TitleClass> {
    if BOOK_WORDS.contains(&word) {
        Some(TitleClass::Book)
    } else if PART_WORDS.contains(&word) {
        Some(TitleClass::Part)
    } else if CHAPTER_WORDS.contains(&word) {
        Some(TitleClass::Chapter)
    } else if SCENE_WORDS.contains(&word) {
        Some(TitleClass::Scene)
    } else {
        None
    }
}

/// Classify a title by the level it names.
pub(crate) fn title_match(title: &str) -> Option<TitleMatch> {
    let n = norm(title);
    if n.is_empty() {
        return None;
    }
    if let Some((class, number, _)) = cjk_level(&n) {
        return Some(TitleMatch {
            class,
            leading: String::new(),
            number: Some(number),
        });
    }
    if FRONT_TITLES.contains(&n.as_str()) {
        return Some(whole(TitleClass::Front));
    }
    if BACK_TITLES.contains(&n.as_str()) {
        return Some(whole(TitleClass::Back));
    }
    let words = tokens(&n);
    let first = *words.first()?;
    // The same words with their case, when lower-casing kept them the same words (it
    // always does outside a few letters no level word or numeral holds).
    let cased_text = norm_cased(title);
    let cased = Some(tokens(&cased_text)).filter(|c| c.len() == words.len());
    if APPENDIX_WORDS.contains(&first)
        && (words.len() == 1 || words.get(1).is_some_and(|w| is_label(w)))
    {
        return Some(whole(TitleClass::Back));
    }
    // A level word, then a number: "Chapter 7", "Part II", "Kapitel vier".
    if let Some(class) = level_of(first) {
        let mut at = 1;
        // "Chapter No. 3", "Chapitre n° 3".
        if words
            .get(1)
            .is_some_and(|w| matches!(*w, "no" | "n" | "nº" | "nr" | "num"))
            && words.get(2).is_some_and(|w| is_digits(w))
        {
            at = 2;
        }
        let rest = &words[at..];
        let cased_word = |i: usize| cased.as_ref().and_then(|c| c.get(i).copied());
        // An abbreviation written against its number and followed by words in lower
        // case is a formula or a code, not a heading: "CH4 emissions", never chapter 4.
        // "Ch.3 The Return", "ch3" and "Chapter1 Intro" still name their level.
        let glued = n[first.len()..].starts_with(|c: char| c.is_numeric());
        if ABBREVIATED_LEVEL_WORDS.contains(&first)
            && glued
            && rest.first().is_some_and(|w| is_digits(w))
            && cased_word(at + 1).is_some_and(starts_in_lower_case)
        {
            return None;
        }
        if let Some(number) = number_at(rest, cased_word(at)) {
            return Some(TitleMatch {
                class,
                leading: first.to_string(),
                number: Some(number),
            });
        }
        return None;
    }
    // An ordinal, then a level word: "Erstes Kapitel", "Premier chapitre". A
    // cardinal never opens a title this way, since the word for one is the
    // indefinite article of five of the languages read: "Une partie de
    // cartes" is a card game, "Ein Teil von mir" a part of me.
    if is_ordinal_word(first)
        && let Some(class) = words.get(1).and_then(|w| level_of(w))
    {
        return Some(TitleMatch {
            class,
            leading: words.get(1).map(|w| w.to_string()).unwrap_or_default(),
            number: Some(first.to_string()),
        });
    }
    None
}

fn whole(class: TitleClass) -> TitleMatch {
    TitleMatch {
        class,
        leading: String::new(),
        number: None,
    }
}

/// `第…章` and its kin: `第`, a number in digits or in Chinese numerals, then
/// the level mark right after it. "第一次见面的章节" (the first meeting's
/// chapter) is not chapter one: `次` ends the number, and no mark follows.
/// The level, the number as written, and the byte offset just past the mark.
fn cjk_level(n: &str) -> Option<(TitleClass, String, usize)> {
    let rest = n.strip_prefix('第')?;
    let mut number = String::new();
    for (at, c) in rest.char_indices() {
        if c.is_numeric() || CJK_NUMERALS.contains(c) {
            if number.chars().count() == 8 {
                return None;
            }
            number.push(c);
            continue;
        }
        if number.is_empty() {
            return None;
        }
        let class = match c {
            '章' => TitleClass::Chapter,
            '部' | '幕' => TitleClass::Part,
            '卷' | '册' => TitleClass::Book,
            _ => return None,
        };
        let end = '第'.len_utf8() + at + c.len_utf8();
        return Some((class, number, end));
    }
    None
}

/// Chinese numerals, everyday and financial forms. Digits, full-width ones
/// included, are numeric already.
const CJK_NUMERALS: &str = "〇零一二三四五六七八九十百千万两壹贰叁肆伍陆柒捌玖拾佰仟";

/// A title's first word, normalised: siblings are grouped by it.
pub(crate) fn first_word(title: &str) -> Option<String> {
    let n = norm(title);
    tokens(&n).first().map(|w| w.to_string())
}

/// Whether a title starts with a prologue word.
pub(crate) fn is_prologue_title(title: &str) -> bool {
    let n = norm(title);
    tokens(&n)
        .first()
        .is_some_and(|w| PROLOGUE_WORDS.contains(w))
}

/// What a paragraph of a document names, when it reads as a chapter heading:
/// short, naming a chapter or a part the way a title would ("CHAPTER II",
/// "Capitulo 3"), and not a sentence ([`reads_as_sentence`]). A manual's
/// section headings ("Toolbar Quick Search Field") do not, which is what keeps
/// them from being offered as chapters.
pub(crate) fn chapter_heading(paragraph: &str) -> Option<TitleMatch> {
    if paragraph.chars().count() > 60 || reads_as_sentence(paragraph) {
        return None;
    }
    title_match(paragraph).filter(|m| matches!(m.class, TitleClass::Chapter | TitleClass::Part))
}

/// Whether a paragraph that opens like a heading is a sentence: it ends as a
/// sentence ends, and the first word past its level word and its number is
/// written in lower case ("Chapter 2 reviews the literature.", "Part two was
/// worse."), or, in Chinese, words follow the level mark ("第二章介绍相关工作。").
/// A thesis outlining its chapters is not a book kept in one document. A
/// heading may still end with a full stop: "CHAPTER I.", "Chapter One.",
/// "Part One: Before the Storm."
fn reads_as_sentence(paragraph: &str) -> bool {
    let end = paragraph.trim_end().trim_end_matches(|c: char| {
        matches!(
            c,
            '"' | '\'' | '\u{201D}' | '\u{2019}' | '\u{00BB}' | '\u{203A}' | ')' | ']'
        )
    });
    let ends_as_sentence = end.chars().last().is_some_and(|c| {
        matches!(
            c,
            '.' | '!'
                | '?'
                | '\u{2026}'
                | ':'
                | ';'
                | '\u{3002}'
                | '\u{FF01}'
                | '\u{FF1F}'
                | '\u{FF1B}'
                | '\u{FF1A}'
        )
    });
    if !ends_as_sentence {
        return false;
    }
    let n = norm(paragraph);
    if let Some((_, _, after)) = cjk_level(&n) {
        return n[after..].chars().any(char::is_alphanumeric);
    }
    paragraph
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty())
        .find(|w| {
            let lower = w.to_lowercase();
            !(level_of(&lower).is_some()
                || is_digits(&lower)
                || roman_value(&lower).is_some()
                || is_number_word(&lower)
                || is_ordinal_word(&lower)
                || NUMBER_JOINERS.contains(&lower.as_str())
                || matches!(lower.as_str(), "no" | "n" | "nº" | "nr" | "num")
                || tokens(&lower)
                    .iter()
                    .all(|t| is_digits(t) || level_of(t).is_some()))
        })
        .is_some_and(|w| w.chars().next().is_some_and(char::is_lowercase))
}

// ---------------------------------------------------------------- numbers

fn is_digits(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_ascii_digit())
}

/// A label after "Appendix": a number, a single letter, a roman numeral.
fn is_label(word: &str) -> bool {
    is_digits(word) || word.chars().count() == 1 || roman_value(word).is_some()
}

/// The number the words at the start of `rest` spell, normalised, or `None`
/// when they do not start with one. `cased_first` is the first word as written,
/// when known: a roman numeral is written in one case ("XIV", "xiv"), so a word
/// written as a name ("Mix", "Li", "Di") is a word, whatever its letters spell.
fn number_at(rest: &[&str], cased_first: Option<&str>) -> Option<String> {
    let first = *rest.first()?;
    if is_digits(first) {
        return Some(first.trim_start_matches('0').to_string());
    }
    let written_as_numeral = cased_first.is_none_or(|w| !is_mixed_case(w));
    if let Some(value) = roman_value(first).filter(|v| *v <= MAX_ROMAN_LEVEL)
        && written_as_numeral
    {
        return Some(value.to_string());
    }
    if !is_number_word(first) {
        return None;
    }
    // A compound spelled over several words ("thirty five", "vingt et un").
    let mut parts = vec![first];
    for word in &rest[1..] {
        if is_number_word(word) || NUMBER_JOINERS.contains(word) {
            parts.push(word);
        } else {
            break;
        }
    }
    while parts.last().is_some_and(|w| NUMBER_JOINERS.contains(w)) {
        parts.pop();
    }
    Some(parts.join("-"))
}

/// The largest roman numeral read as a level's number. No book has four hundred
/// chapters, and the canonical numerals past it spell words and years more often
/// than levels: "Chapter MIX" is not chapter 1009, "Book MCMXCIX" is a year.
const MAX_ROMAN_LEVEL: u32 = 399;

/// The level words that are abbreviations: written against a number, they also
/// begin formulas and codes ("CH4", "Vol2x").
const ABBREVIATED_LEVEL_WORDS: &[&str] = &["ch", "chap", "vol"];

/// A word of two letters or more holding both capitals and small letters: written as
/// a name or a word, never as a roman numeral.
fn is_mixed_case(word: &str) -> bool {
    word.chars().count() >= 2
        && word.chars().any(char::is_uppercase)
        && word.chars().any(char::is_lowercase)
}

/// Whether a word starts with a small letter.
fn starts_in_lower_case(word: &str) -> bool {
    word.chars().next().is_some_and(char::is_lowercase)
}

/// The value of a roman numeral written canonically, or `None`. Canonical only:
/// "iiii" or "vx" are words, not numbers.
fn roman_value(word: &str) -> Option<u32> {
    if word.is_empty() || word.len() > 15 {
        return None;
    }
    let digit = |c: char| match c {
        'i' => Some(1),
        'v' => Some(5),
        'x' => Some(10),
        'l' => Some(50),
        'c' => Some(100),
        'd' => Some(500),
        'm' => Some(1000),
        _ => None,
    };
    let values: Vec<i64> = word.chars().map(digit).collect::<Option<_>>()?;
    // A smaller digit before a larger one subtracts ("iv"); the canonical
    // re-spelling below refuses every other order.
    let mut total: i64 = 0;
    for (i, v) in values.iter().enumerate() {
        if values.get(i + 1).is_some_and(|next| next > v) {
            total -= v;
        } else {
            total += v;
        }
    }
    let total = u32::try_from(total)
        .ok()
        .filter(|t| (1..4000).contains(t))?;
    (to_roman(total) == word).then_some(total)
}

fn to_roman(mut value: u32) -> String {
    const TABLE: &[(u32, &str)] = &[
        (1000, "m"),
        (900, "cm"),
        (500, "d"),
        (400, "cd"),
        (100, "c"),
        (90, "xc"),
        (50, "l"),
        (40, "xl"),
        (10, "x"),
        (9, "ix"),
        (5, "v"),
        (4, "iv"),
        (1, "i"),
    ];
    let mut out = String::new();
    for (n, s) in TABLE {
        while value >= *n {
            out.push_str(s);
            value -= n;
        }
    }
    out
}

/// Words that join the parts of a number spelled over several words.
const NUMBER_JOINERS: &[&str] = &["and", "et", "y", "e", "und"];

/// Cardinal and ordinal number words, whole: English, French, Spanish,
/// Portuguese, German and Italian. Compounds written as one word are handled by
/// [`is_number_word`].
const NUMBER_WORDS: &[&str] = &[
    // English
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
    "hundred",
    "thousand",
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
    "thirteenth",
    "fourteenth",
    "fifteenth",
    "sixteenth",
    "seventeenth",
    "eighteenth",
    "nineteenth",
    "twentieth",
    "thirtieth",
    "fortieth",
    "fiftieth",
    "sixtieth",
    "seventieth",
    "eightieth",
    "ninetieth",
    "hundredth",
    // French
    "un",
    "une",
    "deux",
    "trois",
    "quatre",
    "cinq",
    "sept",
    "huit",
    "neuf",
    "dix",
    "onze",
    "douze",
    "treize",
    "quatorze",
    "quinze",
    "seize",
    "vingt",
    "vingts",
    "trente",
    "quarante",
    "cinquante",
    "soixante",
    "cent",
    "cents",
    "mille",
    "premier",
    "première",
    "premiere",
    "seconde",
    "deuxième",
    "deuxieme",
    "troisième",
    "troisieme",
    "quatrième",
    "quatrieme",
    "cinquième",
    "cinquieme",
    "sixième",
    "sixieme",
    "septième",
    "septieme",
    "huitième",
    "huitieme",
    "neuvième",
    "neuvieme",
    "dixième",
    "dixieme",
    "onzième",
    "onzieme",
    "douzième",
    "douzieme",
    "treizième",
    "quatorzième",
    "quinzième",
    "seizième",
    "vingtième",
    "vingtieme",
    "trentième",
    "quarantième",
    "cinquantième",
    "soixantième",
    "centième",
    "unième",
    // Spanish
    "uno",
    "dos",
    "dós",
    "tres",
    "trés",
    "cuatro",
    "cinco",
    "seis",
    "séis",
    "siete",
    "ocho",
    "nueve",
    "diez",
    "once",
    "doce",
    "trece",
    "catorce",
    "quince",
    "dieciséis",
    "dieciseis",
    "diecisiete",
    "dieciocho",
    "diecinueve",
    "veinte",
    "treinta",
    "cuarenta",
    "cincuenta",
    "sesenta",
    "setenta",
    "ochenta",
    "noventa",
    "cien",
    "ciento",
    "primero",
    "primera",
    "primer",
    "segundo",
    "segunda",
    "tercero",
    "tercera",
    "tercer",
    "cuarto",
    "cuarta",
    "quinto",
    "quinta",
    "sexto",
    "sexta",
    "séptimo",
    "septimo",
    "séptima",
    "octavo",
    "octava",
    "noveno",
    "novena",
    "décimo",
    "decimo",
    "décima",
    // Portuguese
    "um",
    "uma",
    "dois",
    "duas",
    "três",
    "sete",
    "oito",
    "nove",
    "dez",
    "doze",
    "treze",
    "quatorze",
    "dezesseis",
    "dezasseis",
    "dezessete",
    "dezassete",
    "dezoito",
    "dezenove",
    "dezanove",
    "vinte",
    "trinta",
    "quarenta",
    "cinquenta",
    "sessenta",
    "oitenta",
    "noventa",
    "cem",
    "cento",
    "primeiro",
    "primeira",
    "terceiro",
    "terceira",
    "sétimo",
    "oitavo",
    "nono",
    "nona",
    // German
    "eins",
    "ein",
    "eine",
    "zwei",
    "drei",
    "vier",
    "fünf",
    "fuenf",
    "sechs",
    "sieben",
    "acht",
    "neun",
    "zehn",
    "elf",
    "zwölf",
    "zwoelf",
    "dreizehn",
    "vierzehn",
    "fünfzehn",
    "sechzehn",
    "siebzehn",
    "achtzehn",
    "neunzehn",
    "zwanzig",
    "dreißig",
    "dreissig",
    "vierzig",
    "fünfzig",
    "sechzig",
    "siebzig",
    "achtzig",
    "neunzig",
    "hundert",
    // Italian
    "due",
    "tre",
    "quattro",
    "cinque",
    "sei",
    "sette",
    "otto",
    "dieci",
    "undici",
    "dodici",
    "tredici",
    "quattordici",
    "quindici",
    "sedici",
    "diciassette",
    "diciotto",
    "diciannove",
    "venti",
    "trenta",
    "quaranta",
    "sessanta",
    "settanta",
    "ottanta",
    "novanta",
    "primo",
    "prima",
    "secondo",
    "terzo",
    "terza",
    "sesto",
    "settimo",
    "ottavo",
    "decimo",
];

/// Ordinal number words, whole: English, French, Spanish, Portuguese and
/// Italian. German ordinals are recognised by their stems and endings
/// ([`german_ordinal`]).
const ORDINAL_WORDS: &[&str] = &[
    // English
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
    "thirteenth",
    "fourteenth",
    "fifteenth",
    "sixteenth",
    "seventeenth",
    "eighteenth",
    "nineteenth",
    "twentieth",
    "thirtieth",
    "fortieth",
    "fiftieth",
    "sixtieth",
    "seventieth",
    "eightieth",
    "ninetieth",
    "hundredth",
    // French
    "premier",
    "première",
    "premiere",
    "second",
    "seconde",
    "deuxième",
    "deuxieme",
    "troisième",
    "troisieme",
    "quatrième",
    "quatrieme",
    "cinquième",
    "cinquieme",
    "sixième",
    "sixieme",
    "septième",
    "septieme",
    "huitième",
    "huitieme",
    "neuvième",
    "neuvieme",
    "dixième",
    "dixieme",
    "onzième",
    "onzieme",
    "douzième",
    "douzieme",
    "treizième",
    "quatorzième",
    "quinzième",
    "seizième",
    "vingtième",
    "vingtieme",
    "trentième",
    "quarantième",
    "cinquantième",
    "soixantième",
    "centième",
    // Spanish
    "primero",
    "primera",
    "primer",
    "segundo",
    "segunda",
    "tercero",
    "tercera",
    "tercer",
    "cuarto",
    "cuarta",
    "quinto",
    "quinta",
    "sexto",
    "sexta",
    "séptimo",
    "septimo",
    "séptima",
    "octavo",
    "octava",
    "noveno",
    "novena",
    "décimo",
    "decimo",
    "décima",
    // Portuguese
    "primeiro",
    "primeira",
    "terceiro",
    "terceira",
    "quarto",
    "quarta",
    "sétimo",
    "sétima",
    "oitavo",
    "oitava",
    "nono",
    "nona",
    // Italian
    "primo",
    "prima",
    "secondo",
    "seconda",
    "terzo",
    "terza",
    "sesto",
    "sesta",
    "settimo",
    "settima",
    "ottavo",
    "ottava",
    "decima",
];

/// Whether `word` is an ordinal: listed, a German ordinal ("erstes",
/// "zweite"), or a German compound one ("einundzwanzigstes").
fn is_ordinal_word(word: &str) -> bool {
    ORDINAL_WORDS.contains(&word)
        || german_ordinal(word)
        || word.split_once("und").is_some_and(|(unit, tens)| {
            !unit.is_empty() && NUMBER_WORDS.contains(&unit) && german_ordinal(tens)
        })
}

/// German ordinal stems, before their ending: "erst-e", "zweit-es".
const GERMAN_ORDINAL_STEMS: &[&str] = &[
    "erst", "zweit", "dritt", "viert", "fünft", "sechst", "siebt", "acht", "neunt", "zehnt",
    "elft", "zwölft",
];

/// Spanish and Italian tens that fuse with a unit: "veintiuno", "ventitré",
/// "trentuno".
const FUSING_TENS: &[&str] = &[
    "veinti",
    "venti",
    "vent",
    "trenta",
    "trent",
    "quaranta",
    "quarant",
    "cinquanta",
    "cinquant",
    "sessanta",
    "sessant",
    "settanta",
    "settant",
    "ottanta",
    "ottant",
    "novanta",
    "novant",
];

/// Units a fused ten takes.
const FUSED_UNITS: &[&str] = &[
    "uno", "un", "dos", "dós", "tres", "trés", "cuatro", "cinco", "seis", "séis", "siete", "ocho",
    "nueve", "due", "tre", "tré", "quattro", "cinque", "sei", "sette", "otto", "nove",
];

/// Whether `word` is a number word: listed, an ordinal, a fused compound, or a
/// German compound or ordinal.
pub(crate) fn is_number_word(word: &str) -> bool {
    if NUMBER_WORDS.contains(&word) || ORDINAL_WORDS.contains(&word) {
        return true;
    }
    if FUSING_TENS.iter().any(|tens| {
        word.strip_prefix(tens)
            .is_some_and(|unit| FUSED_UNITS.contains(&unit))
    }) {
        return true;
    }
    // German: "einundzwanzig", and its ordinal "einundzwanzigste".
    if let Some((unit, tens)) = word.split_once("und")
        && !unit.is_empty()
        && (NUMBER_WORDS.contains(&unit))
        && (NUMBER_WORDS.contains(&tens) || german_ordinal(tens))
    {
        return true;
    }
    german_ordinal(word)
}

fn german_ordinal(word: &str) -> bool {
    ["e", "er", "es", "en"].iter().any(|ending| {
        word.strip_suffix(ending).is_some_and(|stem| {
            GERMAN_ORDINAL_STEMS.contains(&stem)
                || stem
                    .strip_suffix("st")
                    .is_some_and(|cardinal| NUMBER_WORDS.contains(&cardinal))
        })
    })
}

// ---------------------------------------------------------------- closed lists

/// Names Scrivener or a template gives the Draft folder, and placeholders: a
/// Draft still wearing one says nothing about the book's title.
const DEFAULT_DRAFT_NAMES: &[&str] = &[
    "",
    "draft",
    "manuscript",
    "manuscrit",
    "ébauche",
    "ebauche",
    "brouillon",
    "borrador",
    "manuscrito",
    "rascunho",
    "entwurf",
    "manuskript",
    "bozza",
    "manoscritto",
    "concept",
    "szkic",
    "черновик",
    "草稿",
    "手稿",
    "原稿",
    "script",
    "screenplay",
    "comic script",
    "recipes",
    "short story",
    "lecture steps",
    "essay",
    "hausarbeit",
    "novel",
    "manual",
    "project",
    "ideas",
    "innhold",
    "text",
    "main content",
    "content",
    "contents",
    "thesis",
    "dissertation",
];

/// Whether a Draft title still says nothing about the book: a default name, or
/// a bracketed placeholder such as `<Proposal Title>`.
pub(crate) fn is_default_draft_name(title: &str) -> bool {
    let n = norm(title);
    let trimmed = title.trim();
    DEFAULT_DRAFT_NAMES.contains(&n.as_str())
        || (trimmed.starts_with('<') && trimmed.ends_with('>'))
        || (trimmed.starts_with('[') && trimmed.ends_with(']'))
}

/// Story-bible folder titles, in the languages the corpus holds.
const BIBLE_TITLES: &[&str] = &[
    "characters",
    "character",
    "cast",
    "dramatis personae",
    "personnages",
    "personajes",
    "personagens",
    "personaggi",
    "figuren",
    "charaktere",
    "personen",
    "人物",
    "角色",
    "places",
    "locations",
    "location",
    "settings",
    "setting",
    "lieux",
    "lugares",
    "locais",
    "luoghi",
    "orte",
    "schauplätze",
    "schauplatze",
    "地点",
    "场景设定",
    "worldbuilding",
    "world building",
    "monde",
    "world",
];

/// An icon name without its image extension, lower-cased.
fn icon_base(icon: &str) -> String {
    let lower = icon.trim().to_lowercase();
    for ext in [".tiff", ".tif", ".png", ".jpeg", ".jpg", ".gif"] {
        if let Some(stem) = lower.strip_suffix(ext) {
            return stem.to_string();
        }
    }
    lower
}

/// Scrivener's built-in character and place icons ("Characters (Photo)",
/// "Locations (Map)"). Built-in icon names are resource names, the same in
/// every interface language, which is why they are asked first.
pub(crate) fn is_bible_icon(icon: &str) -> bool {
    let base = icon_base(icon);
    base.starts_with("characters") || base.starts_with("locations")
}

pub(crate) fn is_bible_title(title: &str) -> bool {
    BIBLE_TITLES.contains(&norm(title).as_str())
}

/// Scrivener's "Front Matter" and "Back Matter" icons.
pub(crate) fn matter_icon(icon: &str) -> Option<MatterSide> {
    let base = icon_base(icon);
    if base.starts_with("front matter") {
        Some(MatterSide::Front)
    } else if base.starts_with("back matter") {
        Some(MatterSide::Back)
    } else {
        None
    }
}

/// Root folder titles that name front or back matter outright.
pub(crate) fn matter_title(title: &str) -> Option<MatterSide> {
    match norm(title).as_str() {
        "front matter" | "deckblatt" | "pages liminaires" => Some(MatterSide::Front),
        "back matter" | "anhang" | "annexes" => Some(MatterSide::Back),
        _ => None,
    }
}

/// Titles that only name a level: stored blank on a structural row, where the
/// export's generated heading already says "Chapter 3" and would print the
/// word a second time.
const PLACEHOLDER_TITLES: &[&str] = &[
    "chapter",
    "part",
    "section",
    "scene",
    "book",
    "act",
    "volume",
    "chapter title",
    "part title",
    "book title",
    "chapter heading",
    "part heading",
    "new chapter",
    "new part",
    "chapitre",
    "partie",
    "livre",
    "acte",
    "scène",
    "nouveau chapitre",
    "titre du chapitre",
    "capítulo",
    "capitulo",
    "parte",
    "sección",
    "seccion",
    "escena",
    "libro",
    "acto",
    "seção",
    "secção",
    "cena",
    "livro",
    "ato",
    "kapitel",
    "teil",
    "abschnitt",
    "szene",
    "buch",
    "akt",
    "capitolo",
    "sezione",
    "scena",
    "atto",
    // The level words the title lexicon reads that the list above lacked, and
    // "new chapter" in each language it reads.
    "tome",
    "tomo",
    "vol",
    "chap",
    "hoofdstuk",
    "nouvelle partie",
    "neues kapitel",
    "neuer teil",
    "nuovo capitolo",
    "nuova parte",
    "nuevo capítulo",
    "nuevo capitulo",
    "nueva parte",
    "novo capítulo",
    "novo capitulo",
    "nova parte",
    "章",
    "章节",
    "部分",
    "场景",
    "卷",
];

pub(crate) fn is_placeholder_title(title: &str) -> bool {
    let n = norm(title);
    let n = n.trim_end_matches(|c: char| !c.is_alphanumeric());
    PLACEHOLDER_TITLES.contains(&n)
}

/// Titles a template gives a character or place sheet. As a story-bible entry
/// such a title becomes a name the mention index looks for in the prose.
const GENERIC_SHEET_TITLES: &[&str] = &[
    "protagonist",
    "antagonist",
    "hero",
    "heroine",
    "villain",
    "sidekick",
    "mentor",
    "love interest",
    "main character",
    "new character",
    "untitled",
    "character sketch",
    "setting sketch",
    "character",
    "place",
    "location",
    "setting",
    "protagoniste",
    "antagoniste",
    "héros",
    "personnage",
    "lieu",
    "sans titre",
];

pub(crate) fn is_generic_sheet_title(title: &str) -> bool {
    GENERIC_SHEET_TITLES.contains(&norm(title).as_str())
}

/// Words in a root folder's title that say it holds manuscript: a folder the
/// Draft does not contain but the writer may have meant as more of the book.
const MANUSCRIPT_WORDS: &[&str] = &[
    "draft",
    "drafts",
    "manuscript",
    "manuscripts",
    "manuscrit",
    "manuscrits",
    "episode",
    "episodes",
    "épisode",
    "épisodes",
    "chapter",
    "chapters",
    "chapitre",
    "chapitres",
    "capítulo",
    "capítulos",
    "capitulo",
    "capitulos",
    "kapitel",
    "book",
    "books",
    "livre",
    "livres",
];

pub(crate) fn has_manuscript_word(title: &str) -> bool {
    let n = norm(title);
    tokens(&n).iter().any(|w| MANUSCRIPT_WORDS.contains(w))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn class(title: &str) -> Option<TitleClass> {
        title_match(title).map(|m| m.class)
    }

    #[test]
    fn titles_claim_their_level_with_any_number() {
        for title in [
            "Chapter 7",
            "Chapter Twelve",
            "Chapter Thirty-Five",
            "chapter fifty",
            "Chapitre sept",
            "Chapitre vingt et un",
            "Chapitre onze",
            "Capítulo cuatro",
            "Capítulo diez",
            "Capítulo primero",
            "Capitulo 3",
            "Kapitel vier",
            "Kapitel fünf",
            "Kapitel einundzwanzig",
            "Erstes Kapitel",
            "Premier chapitre",
            "Chapter-01",
            "Ch. 3",
            "chapter1",
            "CHAPTER II",
            "Chapter No. 4",
            "Capitolo ventitré",
            "第三章",
            "第1章 开始",
        ] {
            assert_eq!(class(title), Some(TitleClass::Chapter), "{title}");
        }
        assert_eq!(class("Part II: The Return"), Some(TitleClass::Part));
        assert_eq!(class("Act 1"), Some(TitleClass::Part));
        assert_eq!(class("Partie deux"), Some(TitleClass::Part));
        assert_eq!(class("Book One"), Some(TitleClass::Book));
        assert_eq!(class("Volume 2"), Some(TitleClass::Book));
        assert_eq!(class("Scene 4"), Some(TitleClass::Scene));
    }

    #[test]
    fn words_that_only_look_like_levels_claim_nothing() {
        for title in [
            "Chapters",
            "Chapter: Ironmongery",
            "Chapter Mixup",
            "Book notes",
            "Partial draft",
            "Atom",
            "Bibliography Management",
            "Glossary Support",
            "The Chapter",
            "",
            "Part",
        ] {
            assert_eq!(class(title), None, "{title}");
        }
    }

    /// A word whose letters happen to spell a roman numeral, and a formula that starts
    /// with a level's abbreviation, name no level; the numerals and abbreviations
    /// writers do use still do.
    #[test]
    fn a_word_or_a_formula_is_not_a_number() {
        for title in [
            "Chapter Mix",
            "Chapter Mi amor",
            "CHAPTER MIX",
            "Book MCMXCIX",
            "CH4 emissions",
            "ch4 and co2",
            "Vol2 notes",
        ] {
            assert_eq!(class(title), None, "{title}");
        }
        for (title, number) in [
            ("Chapter XIV", "14"),
            ("chapter xiv", "14"),
            ("CHAPTER CCCXCIX", "399"),
            ("Chapter I", "1"),
            ("Ch4", "4"),
            ("CH4", "4"),
            ("Ch.4 the return", "4"),
            ("Ch4 - The Storm", "4"),
            ("Ch4 The Storm", "4"),
            ("Chapter1 intro", "1"),
        ] {
            let found = title_match(title);
            assert_eq!(
                found.as_ref().map(|m| m.class),
                Some(TitleClass::Chapter),
                "{title}"
            );
            assert_eq!(
                found.and_then(|m| m.number).as_deref(),
                Some(number),
                "{title}"
            );
        }
    }

    #[test]
    fn an_article_or_a_phrase_is_not_a_number() {
        // The indefinite article of French, Spanish, Portuguese, German and
        // Italian is the word for one: "Une partie de cartes" is a card game,
        // "Ein Teil von mir" a part of me. Only an ordinal opens a numbered
        // title ("Premier chapitre", "Second Act").
        for title in [
            "Une partie de campagne",
            "Une partie d'échecs",
            "Un chapitre de ma vie",
            "Un acte manqué",
            "Un acto de fe",
            "Un libro abierto",
            "Uma parte de mim",
            "Um ato de amor",
            "Ein Teil von mir",
            "Ein Buch für Kinder",
            "Sei parte di me",
            "Un atto d'amore",
            "第一次见面的章节",
        ] {
            assert_eq!(class(title), None, "{title}");
        }
        assert_eq!(class("Premier acte"), Some(TitleClass::Part));
        assert_eq!(class("Second Act"), Some(TitleClass::Part));
        assert_eq!(class("Primera parte"), Some(TitleClass::Part));
        assert_eq!(class("Terzo atto"), Some(TitleClass::Part));
        assert_eq!(class("Zweiter Teil"), Some(TitleClass::Part));
        assert_eq!(
            class("Einundzwanzigstes Kapitel"),
            Some(TitleClass::Chapter)
        );
        assert_eq!(class("第十二章"), Some(TitleClass::Chapter));
        assert_eq!(class("第二部 归来"), Some(TitleClass::Part));
    }

    #[test]
    fn a_title_naming_only_its_level_is_a_placeholder_in_every_language_read() {
        for title in [
            "Tome",
            "Tomo",
            "Vol.",
            "Chap.",
            "Hoofdstuk",
            "章",
            "章节",
            "场景",
            "Nouvelle partie",
            "Neues Kapitel",
            "Nuovo capitolo",
            "Nuevo capítulo",
            "Novo capítulo",
        ] {
            assert!(is_placeholder_title(title), "{title}");
        }
        assert!(!is_placeholder_title("Band"));
        assert!(!is_placeholder_title("Volume 2"));
    }

    #[test]
    fn front_and_back_titles_are_whole() {
        assert_eq!(class("Title Page"), Some(TitleClass::Front));
        assert_eq!(class("Table des matières"), Some(TitleClass::Front));
        assert_eq!(class("Appendix"), Some(TitleClass::Back));
        assert_eq!(class("Appendix-1"), Some(TitleClass::Back));
        assert_eq!(class("Appendix A: Data"), Some(TitleClass::Back));
        assert_eq!(class("Annexe II"), Some(TitleClass::Back));
        assert_eq!(class("Acknowledgements"), Some(TitleClass::Back));
        assert_eq!(class("Appendix of doom"), None);
        assert_eq!(class("Title Page draft"), None);
    }

    #[test]
    fn numbers_are_normalised_for_comparison() {
        let number = |t: &str| title_match(t).and_then(|m| m.number);
        assert_eq!(number("Chapter 01"), Some("1".to_string()));
        assert_eq!(number("Chapter IV"), Some("4".to_string()));
        assert_eq!(number("Chapter twenty one"), Some("twenty-one".to_string()));
        assert_eq!(
            number("Chapitre vingt et un"),
            Some("vingt-et-un".to_string())
        );
        assert_eq!(number("Chapter 1 - Scene 2"), Some("1".to_string()));
    }

    #[test]
    fn roman_numerals_are_canonical_only() {
        assert_eq!(roman_value("xiv"), Some(14));
        assert_eq!(roman_value("mcmxcix"), Some(1999));
        assert_eq!(roman_value("iiii"), None);
        assert_eq!(roman_value("vx"), None);
        assert_eq!(roman_value("mixup"), None);
        assert_eq!(roman_value("did"), None);
    }

    #[test]
    fn section_type_names_classify_with_their_decorations() {
        assert_eq!(
            section_class("Chapter Heading"),
            Some(SectionClass::Chapter)
        );
        assert_eq!(
            section_class("Titre de chapitre"),
            Some(SectionClass::Chapter)
        );
        assert_eq!(section_class("H1 Capítulo"), Some(SectionClass::Chapter));
        assert_eq!(section_class("Part Title"), Some(SectionClass::Part));
        assert_eq!(section_class("Book Title"), Some(SectionClass::Book));
        assert_eq!(section_class("Scene"), Some(SectionClass::Scene));
        assert_eq!(section_class("Front Matter"), Some(SectionClass::Front));
        assert_eq!(section_class("Appendix"), Some(SectionClass::Back));
        assert_eq!(section_class("Standalone"), Some(SectionClass::Paratext));
        assert_eq!(section_class("Intro"), Some(SectionClass::Paratext));
        assert_eq!(section_class("Notes"), Some(SectionClass::Notes));
        assert_eq!(section_class("Prologue"), Some(SectionClass::Prologue));
        for generic in ["Section", "Heading", "Text", "N/A", "As-Is", "Level 2", ""] {
            assert_eq!(section_class(generic), None, "{generic}");
        }
        // "Chapters" is a folder of chapters, not a chapter.
        assert_eq!(section_class("Chapters"), None);
    }

    #[test]
    fn chapter_headings_are_short() {
        assert!(chapter_heading("CHAPTER VIII").is_some());
        assert!(chapter_heading("Capitulo 4").is_some());
        assert!(chapter_heading("Toolbar Quick Search Field").is_none());
        let long = format!("Chapter 3 {}", "and so on ".repeat(8));
        assert!(chapter_heading(&long).is_none());
    }

    #[test]
    fn closed_lists() {
        assert!(is_default_draft_name("Draft"));
        assert!(is_default_draft_name("  Manuscrit "));
        assert!(is_default_draft_name("<Proposal Title>"));
        assert!(is_default_draft_name("Short Story"));
        assert!(!is_default_draft_name("Bowl of Salt"));
        assert!(is_bible_icon("Characters (Photo).tiff"));
        assert!(is_bible_icon("Locations (Map)"));
        assert!(!is_bible_icon("Book (Blue Book)"));
        assert!(is_bible_title("Personnages"));
        assert!(is_bible_title("Cast"));
        assert!(!is_bible_title("Characters and places"));
        assert_eq!(matter_icon("Front Matter.tiff"), Some(MatterSide::Front));
        assert_eq!(matter_icon("Back Matter"), Some(MatterSide::Back));
        assert_eq!(matter_title("Pages liminaires"), Some(MatterSide::Front));
        assert_eq!(matter_title("Anhang"), Some(MatterSide::Back));
        assert!(is_placeholder_title("Chapter"));
        assert!(is_placeholder_title("Chapitre"));
        assert!(is_placeholder_title("New Chapter"));
        assert!(is_placeholder_title("Part:"));
        assert!(!is_placeholder_title("Chapter 1"));
        assert!(is_generic_sheet_title("Hero"));
        assert!(is_generic_sheet_title("Untitled"));
        assert!(!is_generic_sheet_title("Mara"));
        assert!(has_manuscript_word("Draft - Cenas"));
        assert!(has_manuscript_word("Book 2"));
        assert!(!has_manuscript_word("Bookkeeping"));
        assert!(is_prologue_title("Prologue: the storm"));
        assert!(is_prologue_title("Épilogue"));
        assert!(!is_prologue_title("Prologues and other beginnings"));
    }

    #[test]
    fn number_words() {
        for word in [
            "thirty",
            "vingt",
            "veintiuno",
            "veintidós",
            "einundzwanzig",
            "einundzwanzigste",
            "erstes",
            "zweite",
            "ventitré",
            "trentuno",
            "dezoito",
            "quinzième",
        ] {
            assert!(is_number_word(word), "{word}");
        }
        for word in ["the", "und", "mix", "venture"] {
            assert!(!is_number_word(word), "{word}");
        }
    }
}
