// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What each document holds, measured once when the project is analysed.
//!
//! The structure mapping decides rows on whether a text carries anything: a
//! folder's own text gets a row of its own only when there is text, a document's
//! notes become a note beside it only when there are notes, an empty folder is
//! treated as a placeholder. A file's size cannot say that (Scrivener writes a
//! few hundred bytes of RTF for a document holding nothing), so every text and
//! notes file is read and decoded here, and the answer kept as a fact the mapping,
//! the review's preview and the conversion all share. Nothing of the writing is
//! kept but the words of the few short paragraphs that read as chapter headings,
//! which the review shows when it offers to split a one-document manuscript.
//!
//! # Failing safe
//!
//! A stream that cannot be read or decoded is [`Carries::Unknown`], and the
//! mapping treats unknown as carrying: a reader bug may add a row, never lose a
//! folder's text through a rule that routes only what it believes is there.
//!
//! "Carries" means: at least one character other than white space after
//! Scrivener's style markers (`<$Scr_Cs::1>`, `<!$Scr_Ps::0>`, `<$Scr_H::1>`,
//! `<$ScrKeepWithNext>`) and the object replacement character are removed, or a
//! picture, or a footnote or comment of its own.

use serde::{Deserialize, Serialize};

use crate::model::{BinderRow, MemberRef, Project, ProjectNote, RecoveredText, TextRole};
use crate::outcome::Refusal;
use crate::structure::lexicon;

/// Whether a stream holds anything a writer would miss.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Carries {
    /// Nothing: no file, an empty one, or only white space and markers.
    #[default]
    No,
    Yes,
    /// Could not be read or decoded: counted as carrying.
    Unknown,
}

impl Carries {
    /// Whether the mapping must find this stream a home.
    pub fn counts(self) -> bool {
        !matches!(self, Carries::No)
    }
}

/// What every document of a project holds, aligned with its binder.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentFacts {
    /// One per binder row, in the binder's order.
    pub items: Vec<ItemFacts>,
    /// One per format 1.x project note, in order.
    pub project_notes: Vec<Carries>,
    /// One per recovered text, in order.
    pub recovered: Vec<Carries>,
}

/// What one binder item's writings hold.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemFacts {
    pub text: TextFacts,
    pub notes: Carries,
    pub synopsis: Carries,
}

/// What an item's main text holds.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextFacts {
    pub carries: Carries,
    /// Words in the body, for the review's evidence and the report's count of
    /// words that stop counting as manuscript. Approximate: runs of characters
    /// between white space that hold a letter or a digit.
    pub words: u32,
    /// Top-level paragraphs that read as chapter headings, in order.
    pub headings: Vec<Heading>,
    /// Whether anything is written before the first heading (the whole text
    /// when there is no heading).
    pub lead: Carries,
    /// Its letters, by script: what the language the import proposes is read from when
    /// the project states none (I3 A8).
    #[serde(default)]
    pub letters: Letters,
}

/// Letters counted by the script they are written in. Marks, digits, punctuation and
/// white space are not letters.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Letters {
    /// Latin, with its accented and extended forms.
    pub latin: u32,
    /// Chinese characters, as Chinese and Japanese write them.
    pub han: u32,
    /// Hiragana and katakana: Japanese only.
    pub kana: u32,
    /// Korean.
    pub hangul: u32,
    pub hebrew: u32,
    /// Arabic, and the languages written in its script.
    pub arabic: u32,
    /// Every other script (Greek, Cyrillic, Devanagari…).
    pub other: u32,
}

impl Letters {
    /// Count the letters of `text` onto these.
    pub fn add(&mut self, text: &str) {
        for c in text.chars().filter(|c| c.is_alphabetic()) {
            let slot = match u32::from(c) {
                0x41..=0x5A | 0x61..=0x7A | 0xAA | 0xBA | 0xC0..=0x24F | 0x1E00..=0x1EFF => {
                    &mut self.latin
                }
                0x3005
                | 0x3007
                | 0x3021..=0x3029
                | 0x3400..=0x4DBF
                | 0x4E00..=0x9FFF
                | 0xF900..=0xFAFF
                | 0x20000..=0x2FA1F => &mut self.han,
                0x3041..=0x309F | 0x30A0..=0x30FF | 0x31F0..=0x31FF | 0xFF66..=0xFF9F => {
                    &mut self.kana
                }
                0x1100..=0x11FF
                | 0x3131..=0x318E
                | 0xA960..=0xA97F
                | 0xAC00..=0xD7AF
                | 0xFFA0..=0xFFDC => &mut self.hangul,
                0x0590..=0x05FF | 0xFB1D..=0xFB4F => &mut self.hebrew,
                0x0600..=0x06FF
                | 0x0750..=0x077F
                | 0x08A0..=0x08FF
                | 0xFB50..=0xFDFF
                | 0xFE70..=0xFEFF => &mut self.arabic,
                _ => &mut self.other,
            };
            *slot = slot.saturating_add(1);
        }
    }

    /// These and `other`'s, together.
    pub fn plus(self, other: Letters) -> Letters {
        Letters {
            latin: self.latin.saturating_add(other.latin),
            han: self.han.saturating_add(other.han),
            kana: self.kana.saturating_add(other.kana),
            hangul: self.hangul.saturating_add(other.hangul),
            hebrew: self.hebrew.saturating_add(other.hebrew),
            arabic: self.arabic.saturating_add(other.arabic),
            other: self.other.saturating_add(other.other),
        }
    }

    /// Every letter counted.
    pub fn total(&self) -> u64 {
        [
            self.latin,
            self.han,
            self.kana,
            self.hangul,
            self.hebrew,
            self.arabic,
            self.other,
        ]
        .iter()
        .map(|n| u64::from(*n))
        .sum()
    }
}

/// A paragraph that reads as a chapter heading.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Heading {
    /// Its index among the document's top-level blocks (paragraphs and tables),
    /// in the order RTF writes them: where a split starts the next chapter.
    pub block: u32,
    /// Its words, markers removed and white space collapsed.
    pub title: String,
}

/// The most headings kept for one document: enough for any real manuscript, and
/// a bound on what a crafted one costs.
const MAX_HEADINGS: usize = 2_000;

impl ContentFacts {
    /// Facts from the model alone, for a caller that cannot read the project:
    /// every file that is not empty counts as carrying, unread. Fails safe; the
    /// plan it gives may hold more rows than the measured one, never fewer.
    pub fn assumed(project: &Project) -> ContentFacts {
        let member = |m: &Option<MemberRef>, salvaged: &Option<String>| match m {
            Some(m) if m.size > 0 => Carries::Unknown,
            Some(_) => Carries::No,
            None => plain(salvaged.as_deref().unwrap_or_default()).carries,
        };
        ContentFacts {
            items: project
                .binder
                .iter()
                .map(|row| {
                    let carries = member(&row.payload.text, &row.payload.salvaged.text);
                    ItemFacts {
                        text: TextFacts {
                            carries,
                            words: 0,
                            headings: Vec::new(),
                            lead: carries,
                            letters: Letters::default(),
                        },
                        notes: member(&row.payload.notes, &row.payload.salvaged.notes),
                        synopsis: member(&row.payload.synopsis, &row.payload.salvaged.synopsis),
                    }
                })
                .collect(),
            project_notes: project
                .project_notes
                .iter()
                .map(|n| member(&n.file, &None))
                .collect(),
            recovered: project
                .recovered
                .iter()
                .map(|r| member(&Some(r.member.clone()), &None))
                .collect(),
        }
    }
}

/// How reading one member ended.
pub(crate) enum Read {
    Bytes(Vec<u8>),
    /// Could not be read: counted as carrying.
    Unreadable,
}

/// Why measuring stopped before it was done.
pub(crate) enum Interrupted {
    /// A member moved under the read.
    Changed,
    /// The container refused a read outright (an archive past its limits).
    Refused(Refusal),
}

/// Measure every document the model refers to, reading each through `read`.
pub(crate) fn measure(
    rows: &[BinderRow],
    project_notes: &[ProjectNote],
    recovered: &[RecoveredText],
    read: &mut dyn FnMut(&MemberRef) -> Result<Read, Interrupted>,
) -> Result<ContentFacts, Interrupted> {
    let mut facts = ContentFacts::default();
    for row in rows {
        let text = match (&row.payload.text, &row.payload.salvaged.text) {
            (Some(m), _) => match read(m)? {
                Read::Bytes(bytes) => rtf(&bytes),
                Read::Unreadable => TextFacts::unknown(),
            },
            (None, Some(salvaged)) => plain(salvaged),
            (None, None) => TextFacts::default(),
        };
        let notes = match (&row.payload.notes, &row.payload.salvaged.notes) {
            (Some(m), _) => match read(m)? {
                Read::Bytes(bytes) => rtf(&bytes).carries,
                Read::Unreadable => Carries::Unknown,
            },
            (None, Some(salvaged)) => plain(salvaged).carries,
            (None, None) => Carries::No,
        };
        let synopsis = match (&row.payload.synopsis, &row.payload.salvaged.synopsis) {
            (Some(m), _) => match read(m)? {
                Read::Bytes(bytes) => plain(&String::from_utf8_lossy(&bytes)).carries,
                Read::Unreadable => Carries::Unknown,
            },
            (None, Some(salvaged)) => plain(salvaged).carries,
            (None, None) => Carries::No,
        };
        facts.items.push(ItemFacts {
            text,
            notes,
            synopsis,
        });
    }
    for note in project_notes {
        facts.project_notes.push(match &note.file {
            Some(m) => match read(m)? {
                Read::Bytes(bytes) => rtf(&bytes).carries,
                Read::Unreadable => Carries::Unknown,
            },
            None => Carries::No,
        });
    }
    for text in recovered {
        facts.recovered.push(match read(&text.member)? {
            Read::Bytes(bytes) if text.role == TextRole::Synopsis => {
                plain(&String::from_utf8_lossy(&bytes)).carries
            }
            Read::Bytes(bytes) => rtf(&bytes).carries,
            Read::Unreadable => Carries::Unknown,
        });
    }
    Ok(facts)
}

impl TextFacts {
    fn unknown() -> TextFacts {
        TextFacts {
            carries: Carries::Unknown,
            words: 0,
            headings: Vec::new(),
            lead: Carries::Unknown,
            letters: Letters::default(),
        }
    }
}

/// What plain text holds (a synopsis, or text Scrivener's search index kept).
pub(crate) fn plain(text: &str) -> TextFacts {
    let carries = if text.chars().any(|c| !c.is_whitespace() && c != '\u{FFFC}') {
        Carries::Yes
    } else {
        Carries::No
    };
    let mut letters = Letters::default();
    letters.add(text);
    TextFacts {
        carries,
        words: count_words(text),
        headings: Vec::new(),
        lead: carries,
        letters,
    }
}

/// What an RTF document holds.
pub fn rtf(bytes: &[u8]) -> TextFacts {
    if bytes.iter().all(|b| b.is_ascii_whitespace()) {
        return TextFacts::default();
    }
    let Ok(doc) = rtf_reader::parse(bytes) else {
        return TextFacts::unknown();
    };
    let mut facts = TextFacts::default();
    let mut candidates: Vec<Candidate> = Vec::new();
    let mut first_carrying: Option<u32> = None;
    for (index, block) in doc.body.iter().enumerate() {
        let mut block_carries = false;
        for paragraph in rtf_reader::paragraphs(std::slice::from_ref(block)) {
            let mut raw = String::new();
            for inline in &paragraph.inlines {
                match inline {
                    rtf_reader::Inline::Text(run) => raw.push_str(&run.text),
                    // A line break separates words as a space does: a heading written
                    // as its number, a break and its name is titled with both, apart.
                    rtf_reader::Inline::Break(_) => raw.push(' '),
                    rtf_reader::Inline::Picture(_)
                    | rtf_reader::Inline::NoteReference(_)
                    | rtf_reader::Inline::AnnotationReference(_) => block_carries = true,
                    _ => {}
                }
            }
            let text = strip_markers(&raw);
            if text.chars().any(|c| !c.is_whitespace() && c != '\u{FFFC}') {
                block_carries = true;
            }
            facts.words = facts.words.saturating_add(count_words(&text));
            facts.letters.add(&text);
            if let rtf_reader::Block::Paragraph(_) = block
                && candidates.len() < MAX_HEADINGS
                && let Some(candidate) = heading(index as u32, &text)
            {
                candidates.push(candidate);
            }
        }
        if block_carries && first_carrying.is_none() {
            first_carrying = Some(index as u32);
        }
    }
    facts.headings = chapters(candidates);
    // What comes before the first heading that opens a chapter: a heading that
    // only repeats its chapter's number is text of the part before it.
    facts.lead = match (first_carrying, facts.headings.first()) {
        (Some(first), Some(opening)) if first < opening.block => Carries::Yes,
        (Some(_), None) => Carries::Yes,
        _ => Carries::No,
    };
    // A footnote or comment written with no text around it is still writing.
    let any = first_carrying.is_some() || !doc.notes.is_empty() || !doc.annotations.is_empty();
    facts.carries = if any { Carries::Yes } else { Carries::No };
    facts
}

/// A paragraph that reads as a chapter or part heading, before the document's
/// headings are compared with each other.
struct Candidate {
    heading: Heading,
    part: bool,
    number: Option<String>,
}

/// The heading a paragraph is, if it is one: a short paragraph naming a chapter
/// or a part the way a title does ("CHAPTER II", "Capitulo 3"), and not a
/// sentence opening with those words ("Chapter 2 reviews the literature.").
///
/// Only the words decide. A paragraph Scrivener styled as a heading is not
/// enough: manuals and tutorials hold dozens of documents of section headings
/// ("Toolbar Quick Search Field") that must not be split into chapters, and the
/// measured one-document manuscripts all name their chapters in words (I3, A2).
fn heading(block: u32, text: &str) -> Option<Candidate> {
    let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let found = lexicon::chapter_heading(&cleaned)?;
    Some(Candidate {
        heading: Heading {
            block,
            title: cleaned,
        },
        part: found.class == lexicon::TitleClass::Part,
        number: found.number,
    })
}

/// The headings that open a chapter or a part. A heading repeating the number
/// of the one before it at its level names a scene of that chapter ("Chapter
/// 1 - Scene 2" after "Chapter 1 - Scene 1"), the rule sibling titles follow
/// too; a part heading starts the chapter numbering again.
fn chapters(candidates: Vec<Candidate>) -> Vec<Heading> {
    let mut out = Vec::with_capacity(candidates.len());
    let mut last_part: Option<Option<String>> = None;
    let mut last_chapter: Option<Option<String>> = None;
    for candidate in candidates {
        let last = if candidate.part {
            &mut last_part
        } else {
            &mut last_chapter
        };
        if candidate.number.is_some() && last.as_ref() == Some(&candidate.number) {
            continue;
        }
        *last = Some(candidate.number);
        if candidate.part {
            last_chapter = None;
        }
        out.push(candidate.heading);
    }
    out
}

/// Remove Scrivener's style markers, which are text inside the RTF:
/// `<$Scr_Cs::1>`, `<!$Scr_Ps::0>`, `<$ScrKeepWithNext>`.
pub(crate) fn strip_markers(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find('<') {
        out.push_str(&rest[..at]);
        let after = &rest[at + 1..];
        let body = after.strip_prefix('!').unwrap_or(after);
        let marker_len = body.strip_prefix("$Scr").and_then(|tail| {
            let end = tail.find('>')?;
            let inner = &tail[..end];
            (end <= 40
                && inner
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':'))
            .then_some(end)
        });
        match marker_len {
            Some(end) => {
                // Skip `<`, an optional `!`, `$Scr`, the name and `>`.
                let skipped = 1 + (after.len() - body.len()) + 4 + end + 1;
                rest = &rest[at + skipped..];
            }
            None => {
                out.push('<');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

fn count_words(text: &str) -> u32 {
    text.split_whitespace()
        .filter(|w| w.chars().any(char::is_alphanumeric))
        .count()
        .min(u32::MAX as usize) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markers_are_not_writing() {
        assert_eq!(strip_markers("<$Scr_Cs::1>Hello<!$Scr_Cs::1>"), "Hello");
        assert_eq!(strip_markers("<!$Scr_Ps::0>"), "");
        assert_eq!(strip_markers("a<$ScrKeepWithNext>b"), "ab");
        assert_eq!(strip_markers("x < y and <b>"), "x < y and <b>");
        assert_eq!(strip_markers("<$Scr_H::1>Chapter 1"), "Chapter 1");
        assert_eq!(strip_markers("<"), "<");
    }

    #[test]
    fn an_rtf_shell_carries_nothing() {
        let shell = br"{\rtf1\ansi\ansicpg1252\cocoartf2580{\fonttbl\f0\fnil Palatino;}\pard\f0 <!$Scr_Ps::0>\par}";
        assert_eq!(rtf(shell).carries, Carries::No);
        assert_eq!(rtf(b"").carries, Carries::No);
        assert_eq!(rtf(b"   ").carries, Carries::No);
    }

    #[test]
    fn text_carries_and_is_counted() {
        let doc = br"{\rtf1\ansi Once upon a time.\par There was a keeper.\par}";
        let facts = rtf(doc);
        assert_eq!(facts.carries, Carries::Yes);
        assert_eq!(facts.words, 8);
        assert!(facts.headings.is_empty());
        assert_eq!(facts.lead, Carries::Yes);
    }

    #[test]
    fn undecodable_is_unknown_and_counts() {
        let facts = rtf(b"not rtf at all");
        assert_eq!(facts.carries, Carries::Unknown);
        assert!(facts.carries.counts());
    }

    #[test]
    fn chapter_headings_are_found_with_their_blocks() {
        let doc = br"{\rtf1\ansi A word first.\par CHAPTER I\par It began.\par CHAPTER II\par It went on.\par Chapter Three\par It ended.\par}";
        let facts = rtf(doc);
        let titles: Vec<&str> = facts.headings.iter().map(|h| h.title.as_str()).collect();
        assert_eq!(titles, ["CHAPTER I", "CHAPTER II", "Chapter Three"]);
        assert_eq!(
            facts.headings.iter().map(|h| h.block).collect::<Vec<_>>(),
            [1, 3, 5]
        );
        assert_eq!(facts.lead, Carries::Yes);
    }

    #[test]
    fn nothing_before_the_first_heading() {
        let doc = br"{\rtf1\ansi \par Chapter 1\par Text.\par}";
        let facts = rtf(doc);
        assert_eq!(facts.headings.len(), 1);
        assert_eq!(facts.lead, Carries::No);
    }

    /// A heading written as its number, a line break and its name keeps a space where the
    /// break was: the row it opens is titled "Chapter 1 The Storm", never "Chapter 1The
    /// Storm".
    #[test]
    fn a_line_break_in_a_heading_is_a_space_in_its_title() {
        let doc = br"{\rtf1\ansi Chapter 1\line The Storm\par It began.\par Chapter 2\line\line Calm\par It went on.\par}";
        let facts = rtf(doc);
        let titles: Vec<&str> = facts.headings.iter().map(|h| h.title.as_str()).collect();
        assert_eq!(titles, ["Chapter 1 The Storm", "Chapter 2 Calm"]);
        assert_eq!(facts.words, 12, "a break between two words keeps them two");
    }

    #[test]
    fn a_heading_style_alone_is_not_a_chapter() {
        let doc = br"{\rtf1\ansi <$Scr_H::1>Toolbar Quick Search Field\par Text.\par <$Scr_H::1>Chapter 2\par More.\par}";
        let facts = rtf(doc);
        let titles: Vec<&str> = facts.headings.iter().map(|h| h.title.as_str()).collect();
        assert_eq!(titles, ["Chapter 2"]);
    }

    #[test]
    fn scene_titles_are_not_chapter_headings() {
        // "Chapter 1 - Scene 2" names a scene of chapter 1: only the first
        // heading of a run with one number opens that chapter.
        let doc = br"{\rtf1\ansi Chapter 1 - Scene 1\par It began.\par Chapter 1 - Scene 2\par It went on.\par Chapter 1 - Scene 3\par It ended.\par}";
        let facts = rtf(doc);
        let titles: Vec<&str> = facts.headings.iter().map(|h| h.title.as_str()).collect();
        assert_eq!(titles, ["Chapter 1 - Scene 1"]);
        assert_eq!(facts.lead, Carries::No);
        // Numbering that starts again under each part is not a repeat.
        let doc = br"{\rtf1\ansi PART ONE\par Chapter 1\par a\par Chapter 2\par b\par PART TWO\par Chapter 1\par c\par}";
        assert_eq!(rtf(doc).headings.len(), 5);
    }

    #[test]
    fn a_sentence_that_starts_with_a_level_is_not_a_heading() {
        // A thesis introduction's outline, and a line of dialogue: sentences,
        // not headings.
        let doc = br"{\rtf1\ansi This thesis has five chapters.\par Chapter 2 reviews the literature.\par Chapter 3 describes the method.\par Chapter 4 presents the results.\par Chapter 5 concludes.\par}";
        let facts = rtf(doc);
        assert!(facts.headings.is_empty(), "{:?}", facts.headings);
        assert_eq!(facts.lead, Carries::Yes);
        let doc = br"{\rtf1\ansi Il faisait froid.\par Une partie de cartes.\par Ils jouaient.\par Une partie de chasse.\par Ils couraient.\par Une partie de dames.\par Fin.\par}";
        assert!(rtf(doc).headings.is_empty());
        // A heading may still end with a full stop.
        let doc = br"{\rtf1\ansi CHAPTER I.\par a\par Chapter Two.\par b\par CHAPTER III. INTRODUCTORY.\par c\par}";
        assert_eq!(rtf(doc).headings.len(), 3);
    }

    #[test]
    fn letters_are_counted_by_their_script() {
        let mut letters = Letters::default();
        letters.add("Été 1: 夏の日, 여름, שלום, سلام, Лето!");
        assert_eq!(letters.latin, 3);
        assert_eq!(letters.han, 2, "夏 and 日");
        assert_eq!(letters.kana, 1, "の");
        assert_eq!(letters.hangul, 2);
        assert_eq!(letters.hebrew, 4);
        assert_eq!(letters.arabic, 4);
        assert_eq!(letters.other, 4, "Cyrillic");
        assert_eq!(letters.total(), 20);
        let doc = br"{\rtf1\ansi Once upon a time.\par}";
        assert_eq!(rtf(doc).letters.latin, 13);
    }

    #[test]
    fn a_picture_alone_carries() {
        let doc = br"{\rtf1\ansi {\pict\pngblip\picw1\pich1 89504e47}\par}";
        assert_eq!(rtf(doc).carries, Carries::Yes);
    }

    #[test]
    fn plain_text() {
        assert_eq!(plain("  \n ").carries, Carries::No);
        assert_eq!(plain("A synopsis.").carries, Carries::Yes);
        assert_eq!(plain("\u{FFFC}").carries, Carries::No);
    }
}
