// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Cutting a plain-text public-domain novel into chapters.
//!
//! Deliberately separate from bundle building, and pure: it takes a string and a
//! [`Source`] and returns chapters, so every rule below is unit-testable without a
//! `.skrib` anywhere near it. The rules are few and each earns its place against a
//! real transcription — see `[source]` in each example's TOML for which ones that
//! book turns on.
//!
//! ## Why the chapter marker is not configurable
//!
//! A chapter starts at **a line holding nothing but the Roman numeral of the chapter
//! that must come next**. Anchoring on the *expected* numeral rather than on "any
//! Roman numeral" is what makes the rule safe: `I`, `V` and `X` alone on a line are
//! plausible prose (a verse number, a regnal name), and a scanner that accepted any
//! of them would silently split a chapter in two. Requiring the next one in sequence
//! means a false positive has to be the exact numeral the scanner is already looking
//! for, in the right order — and a genuine gap becomes a hard error naming the
//! numeral it never found, instead of a book quietly missing chapter XIV.

use anyhow::{Result, bail, ensure};

use crate::spec::Source;

/// One chapter as it was found in the source.
#[derive(Debug, PartialEq)]
pub struct RawChapter {
    /// 1-based position, which is also the value its Roman numeral spelled.
    pub number: u32,
    /// The heading block that followed the numeral, joined into one line. Kept so
    /// the caller can check it against the title the TOML claims.
    pub heading: String,
    /// Body paragraphs, each already unwrapped to a single line.
    pub paragraphs: Vec<String>,
}

/// Normalise the whole source, then cut it into chapters.
pub fn parse(source: &str, cfg: &Source) -> Result<Vec<RawChapter>> {
    let text = normalise(source, cfg);
    let lines: Vec<&str> = text.lines().collect();

    let mut chapters = Vec::new();
    let mut next = 1u32;
    let mut start = None;
    for (i, line) in lines.iter().enumerate() {
        if *line == roman(next)
            && lines
                .get(i.wrapping_sub(1))
                .is_none_or(|p| p.trim().is_empty())
        {
            if let Some(from) = start.replace(i + 1) {
                chapters.push(cut(&lines[from..i], next - 1)?);
            }
            next += 1;
        }
    }
    let Some(from) = start else {
        bail!(
            "no chapter marker found: the source has no line reading exactly \"I\" \
             after a blank line"
        );
    };
    chapters.push(cut(&lines[from..], next - 1)?);
    Ok(chapters)
}

/// Everything that happens to the text before it is cut up.
fn normalise(source: &str, cfg: &Source) -> String {
    let mut text = source.replace("\r\n", "\n");
    for [from, to] in &cfg.replace {
        text = text.replace(from.as_str(), to.as_str());
    }
    if cfg.curly_apostrophes {
        // Every transcription checked spells the apostrophe `'` and the quotation
        // marks `«»`, so there is no ambiguity to resolve: a `'` in French prose set
        // this way is always an elision, never an opening quote.
        text = text.replace('\'', "\u{2019}");
    }

    // The stop markers are armed only once the book has started. They name *back*
    // matter, and a title page can legitimately carry the same words as a colophon:
    // this transcription prints the printer's mark both on page ii and on the last
    // page, so a scan that watched for it from line 1 truncated the whole novel at
    // its own title page — with no error, because "everything before chapter I is
    // front matter" then explained the empty result perfectly.
    let mut out = Vec::new();
    let mut started = false;
    let mut lines = text.lines().peekable();
    'outer: while let Some(line) = lines.next() {
        started |= line == "I";
        if started {
            for marker in &cfg.stop_at_lines_starting_with {
                if line.trim_start().starts_with(marker.as_str()) {
                    break 'outer;
                }
            }
        }
        for marker in &cfg.drop_blocks_starting_with {
            if line.trim_start().starts_with(marker.as_str()) {
                // The block runs to the line that closes the bracket, which is this
                // one unless the caption wrapped.
                if !line.contains(']') {
                    for rest in lines.by_ref() {
                        if rest.contains(']') {
                            break;
                        }
                    }
                }
                continue 'outer;
            }
        }
        out.push(line);
    }
    out.join("\n")
}

/// One chapter's lines, after its numeral and before the next one's.
///
/// The heading is "everything up to the first blank line", which is what both
/// transcription styles produce — an upper-case block of one to three lines in the
/// Hetzel setting, a single mixed-case line in the other. A chapter that opened
/// straight into prose would lose its first paragraph to this rule, so the caller
/// checks the heading against the title the TOML declares; that check is what makes
/// the rule safe rather than merely convenient.
fn cut(lines: &[&str], number: u32) -> Result<RawChapter> {
    let mut i = 0;
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }
    let heading_from = i;
    while i < lines.len() && !lines[i].trim().is_empty() {
        i += 1;
    }
    let heading = join(&lines[heading_from..i]);
    ensure!(!heading.is_empty(), "chapter {number} has no heading");

    let paragraphs = lines[i..]
        .split(|l| l.trim().is_empty())
        .map(join)
        .filter(|p| !p.is_empty())
        .collect();
    Ok(RawChapter {
        number,
        heading,
        paragraphs,
    })
}

/// Unwrap a hard-wrapped block into one line.
///
/// The wrap is a property of the transcription's 72-column terminal, not of the
/// prose, and it has to go: Gutenberg italics span the break (`_Institution royale\n
/// de la Grande-Bretagne_`), so a converter that kept the newlines would leave a
/// stray underscore at each end of a third of the book's emphasis.
fn join(lines: &[&str]) -> String {
    lines
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// `1 -> "I"`, `37 -> "XXXVII"`. Upper case, the only spelling either source uses.
fn roman(mut n: u32) -> String {
    const TABLE: &[(u32, &str)] = &[
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut out = String::new();
    for (value, glyph) in TABLE {
        while n >= *value {
            out.push_str(glyph);
            n -= value;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> Source {
        Source {
            drop_blocks_starting_with: vec!["[Illustration".to_string()],
            stop_at_lines_starting_with: vec!["TABLE DES".to_string()],
            replace: vec![["--".to_string(), "\u{2014}".to_string()]],
            curly_apostrophes: true,
            footnotes: false,
        }
    }

    const SAMPLE: &str = "\
  LE TITRE

  [Illustration]

I

PREMIER CHAPITRE.

Une phrase qui parle de l'Inde
et se poursuit ici.

Un second paragraphe.

II

SECOND CHAPITRE,
SUR DEUX LIGNES.

[Illustration: une vue
sur deux lignes.]

Le corps du second.

TABLE DES MATIERES

I.--Premier
";

    #[test]
    fn cuts_on_the_expected_numeral_only() {
        let chapters = parse(SAMPLE, &cfg()).unwrap();
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0].number, 1);
        assert_eq!(chapters[0].heading, "PREMIER CHAPITRE.");
        assert_eq!(chapters[1].heading, "SECOND CHAPITRE, SUR DEUX LIGNES.");
    }

    #[test]
    fn unwraps_paragraphs_and_curls_apostrophes() {
        let chapters = parse(SAMPLE, &cfg()).unwrap();
        assert_eq!(
            chapters[0].paragraphs,
            vec![
                "Une phrase qui parle de l\u{2019}Inde et se poursuit ici.".to_string(),
                "Un second paragraphe.".to_string(),
            ]
        );
    }

    #[test]
    fn drops_illustrations_including_wrapped_captions() {
        let chapters = parse(SAMPLE, &cfg()).unwrap();
        assert_eq!(chapters[1].paragraphs, vec!["Le corps du second."]);
    }

    /// The rule that cost the whole novel once: a colophon printed on the title page
    /// as well as on the last page must not truncate the book at page ii.
    #[test]
    fn a_stop_marker_in_the_front_matter_is_not_a_stop() {
        let text = "\
  Paris.--Imp. GAUTHIER-VILLARS.

I

TITRE.

Le corps du livre.

Paris.--Imp. GAUTHIER-VILLARS.
";
        let cfg = Source {
            stop_at_lines_starting_with: vec!["Paris.--Imp.".to_string()],
            ..Source::default()
        };
        let chapters = parse(text, &cfg).unwrap();
        assert_eq!(chapters.len(), 1);
        assert_eq!(chapters[0].paragraphs, vec!["Le corps du livre."]);
    }

    #[test]
    fn stops_before_the_back_matter() {
        // The trailing table of contents holds a line starting `I.--Premier`, which
        // is not a bare numeral — but the *stop* rule is what guarantees no back
        // matter can ever reach a chapter body, whatever it happens to contain.
        let chapters = parse(SAMPLE, &cfg()).unwrap();
        assert!(!chapters[1].paragraphs.iter().any(|p| p.contains("Premier")));
    }

    #[test]
    fn a_numeral_inside_a_paragraph_is_not_a_chapter() {
        // "V" ending a line mid-paragraph, and "II" out of sequence: neither may cut.
        let text = "I\n\nTITRE.\n\nHenri\nV\nregnait alors.\n\nUn autre.\n";
        let chapters = parse(text, &Source::default()).unwrap();
        assert_eq!(chapters.len(), 1);
        assert_eq!(chapters[0].paragraphs.len(), 2);
    }

    #[test]
    fn a_source_with_no_chapters_is_an_error() {
        assert!(parse("juste de la prose\n", &Source::default()).is_err());
    }

    #[test]
    fn roman_numerals_round_trip() {
        for (n, s) in [(1, "I"), (4, "IV"), (9, "IX"), (29, "XXIX"), (37, "XXXVII")] {
            assert_eq!(roman(n), s);
        }
    }
}
