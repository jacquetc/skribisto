// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `.txt`: plain text, kept as it was typed.
//!
//! A `.txt` file is text that was never markup, so its prose goes through
//! `skrib_format::plain_text_to_djot_verbatim` and reads back character for character.
//!
//! This format used to run through the Markdown scanner, which cost a plain file three
//! ways: a `*` or `_` pair became emphasis, lines with no blank line between them ran
//! together into one paragraph, and the Djot written from the Markdown model left
//! `10:30:45`, straight quotes, `--`, `...` and a line opening `I. ` for the first load
//! to rewrite.
//!
//! ## Where a paragraph ends
//!
//! Plain text marks a paragraph in one of two ways, and a file does not say which:
//!
//! * **One paragraph per line.** Skribisto's own plain-text export writes this, with a
//!   blank line only before its list of notes, and so do most editors that wrap lines on
//!   screen. A line break is the end of a paragraph.
//! * **Wrapped at a fixed width**, a blank line between paragraphs: e-mail, e-texts, and
//!   anything written for a terminal. A line break inside a paragraph is only where the
//!   line ran out of room.
//!
//! Reading the first shape as the second runs a whole book into a handful of paragraphs,
//! which is what the Markdown route did to this app's own export; reading the second as
//! the first cuts every sentence that crosses a line end. So the file is measured
//! (`wrap_widths`). Its width is the length nine lines in ten do not exceed, so that a
//! link, a title or a line nobody wrapped does not set it, and it must be as long as a
//! wrapped line is (`WRAP_WIDTHS`). Each stretch of consecutive prose lines is then
//! measured on its own, since a file wrapped by hand, or by several tools over the years,
//! is not wrapped at one width: a stretch whose longest line comes within a few characters
//! of the file's width (`WIDTH_SLACK`) is a paragraph wrapped at the width of that line,
//! and any other stretch keeps the file's. The file is read as wrapped only when it
//! separates paragraphs with blank lines and nearly every line followed by another in the
//! same paragraph is full, meaning the next line's first word would not have fitted on it.
//! Then a full line is joined to the next with a space, and so is a line longer than the
//! file's width, which is a word too long for any line. A short line still ends its
//! paragraph, so an address, a verse or a list set line by line inside a wrapped file
//! keeps its lines. Every other file is read one paragraph per line.
//!
//! Three things are still read as structure, one line at a time, in this order:
//!
//! * a line that is scene-break vocabulary (`* * *`, `#`, `# # #`, `⁂`, …), which a
//!   writer draws in plain text as anywhere else. Vocabulary comes first, so the app's
//!   own major break `# # #` is never taken for a heading;
//! * a line of three or more `-`, `*` or `_` and nothing else but spaces, the commonest
//!   way other tools draw a break, which becomes an ordinary one;
//! * a line opening with one to six `#` and a space or a tab, the one heading syntax a
//!   plain-text writer reaches for, which is also what this format read as a heading
//!   before.
//!
//! A metadata block at the very top (`---` … `---`) is read as front matter, as it is
//! for Markdown.

use anyhow::Result;
use skribisto_model::scene_break::{self, SceneBreakTier};

use crate::block::{SourceBlock, SourceDocument};
use crate::diagnostics::ImportDiagnostic;
use crate::front_matter;
use crate::scanner::SourceScanner;
use crate::text;

pub struct PlainTextScanner;

impl SourceScanner for PlainTextScanner {
    fn extensions(&self) -> &[&str] {
        &["txt", "text"]
    }

    fn format_name(&self) -> &'static str {
        "plain-text"
    }

    fn scan(&self, bytes: &[u8], display_name: &str, origin: &str) -> Result<SourceDocument> {
        let decoded = text::decode(bytes, origin);
        let mut doc = SourceDocument::new(display_name, origin);
        doc.diagnostics.extend(decoded.diagnostics);

        if decoded.text.trim().is_empty() {
            doc.diagnostics.push(ImportDiagnostic::EmptyFile {
                path: origin.to_string(),
            });
            return Ok(doc);
        }

        let fm = front_matter::split(&decoded.text, origin);
        doc.metadata = fm.metadata;
        doc.diagnostics.extend(fm.diagnostics);
        doc.blocks = segment(&decoded.text[fm.body_offset..])?;

        if !doc
            .blocks
            .iter()
            .any(|b| matches!(b, SourceBlock::Heading { .. }))
        {
            doc.diagnostics.push(ImportDiagnostic::NoHeadings {
                path: origin.to_string(),
            });
        }
        Ok(doc)
    }
}

/// One line of the body, as [`segment`] reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Line<'a> {
    Blank,
    SceneBreak(SceneBreakTier),
    Heading {
        level: u8,
        text: String,
    },
    /// Prose: the line as the file has it, without its trailing whitespace, which is what a
    /// wrapped line's width is measured on.
    Prose(&'a str),
}

impl<'a> Line<'a> {
    /// A prose line's length in characters, what a wrapped line's width is measured in.
    fn prose_len(&self) -> Option<usize> {
        match self {
            Line::Prose(text) => Some(text.chars().count()),
            _ => None,
        }
    }

    fn read(line: &'a str) -> Self {
        let trimmed = skrib_format::trim_djot_whitespace(line);
        if trimmed.is_empty() {
            Line::Blank
        } else if let Some(tier) = scene_break::tier_of_plain_line(trimmed) {
            Line::SceneBreak(tier)
        } else if is_drawn_rule(trimmed) {
            Line::SceneBreak(SceneBreakTier::Minor)
        } else if let Some((level, text)) = heading(trimmed) {
            Line::Heading { level, text }
        } else {
            Line::Prose(line.trim_end_matches(skrib_format::is_djot_whitespace))
        }
    }
}

/// Cut the text into headings, scene breaks and the prose between them, a line at a time.
///
/// Scene-break vocabulary is read before a heading, so the app's own major break `# # #`
/// is never taken for one. Consecutive prose lines are one paragraph each, unless the file
/// is wrapped at a fixed width, where a full line runs on into the next (see the module
/// documentation).
fn segment(body: &str) -> Result<Vec<SourceBlock>> {
    // `\r\n` and a lone `\r` end a line too.
    let lines: Vec<Line<'_>> = body
        .split('\n')
        .flat_map(|line| line.strip_suffix('\r').unwrap_or(line).split('\r'))
        .map(Line::read)
        .collect();
    let widths = wrap_widths(&lines);

    let mut blocks = Vec::new();
    let mut prose = String::new();
    for (index, line) in lines.iter().enumerate() {
        match line {
            Line::Blank => {}
            Line::SceneBreak(tier) => {
                flush_prose(&mut prose, &mut blocks)?;
                blocks.push(SourceBlock::SceneBreak { tier: *tier });
            }
            Line::Heading { level, text } => {
                flush_prose(&mut prose, &mut blocks)?;
                blocks.push(SourceBlock::Heading {
                    level: *level,
                    text: text.clone(),
                });
            }
            Line::Prose(text) => {
                prose.push_str(skrib_format::trim_djot_whitespace(text));
                let runs_on = match (&widths, lines.get(index + 1)) {
                    (Some(widths), Some(Line::Prose(next))) => widths
                        .get(index)
                        .is_some_and(|width| is_full(text, next, *width)),
                    _ => false,
                };
                prose.push(if runs_on { ' ' } else { '\n' });
            }
        }
    }
    flush_prose(&mut prose, &mut blocks)?;
    Ok(blocks)
}

/// How long a line wrapped at a fixed width can be, in characters. Below this a file is
/// short lines, not wrapped ones; above it, lines are paragraphs a screen wraps.
const WRAP_WIDTHS: std::ops::RangeInclusive<usize> = 40..=100;

/// How far under the file's width a stretch of lines can stop and still be a paragraph
/// wrapped at a width of its own, in characters.
///
/// A wrapped paragraph's longest line falls short of its width by less than the word that
/// did not fit; a line set on its own, one line of an address or of a verse, most often
/// falls much further short.
const WIDTH_SLACK: usize = 10;

/// The width each line is measured against, by line index, or `None` when the file is read
/// one paragraph per line.
///
/// The file's width is the length at least nine prose lines in ten do not exceed, and it
/// must fall within [`WRAP_WIDTHS`]. The longest line would do only for a file wrapped to
/// the character: one link or one unwrapped line would set it, and then no other line would
/// come near it. A line longer than the width is [full](is_full) whatever follows it, which
/// is right for a word too long for any line.
///
/// Each stretch of consecutive prose lines is measured against its own longest line that
/// is not longer than the file's width, when that line comes within [`WIDTH_SLACK`] of it.
/// A file wrapped by hand, or by several tools over the years, runs a little narrower or
/// wider from one paragraph to the next, and a paragraph wrapped at some width fills its
/// lines to just under it. A stretch that stops further short, an address or a verse,
/// keeps the file's width, so its short lines still end their paragraphs.
///
/// Wrapped also means that a blank line separates two stretches of prose somewhere, and
/// that at least three in four of the lines followed by another in a stretch measured on
/// its own are full. That count is what keeps a file of one paragraph per line from being
/// read as wrapped: its paragraphs are of every length, and few come near the longest
/// around them. A stretch's longest line is full at its own width by definition, so the
/// count measures it against the file's width instead; otherwise a short reply under a
/// longer line, one paragraph per line, would prove its own stretch wrapped.
fn wrap_widths(lines: &[Line<'_>]) -> Option<Vec<usize>> {
    let mut lengths: Vec<usize> = lines.iter().filter_map(Line::prose_len).collect();
    lengths.sort_unstable();
    // Nearest rank: the smallest length that at least nine lines in ten do not exceed.
    let rank = (lengths.len() * 9).div_ceil(10);
    let width = *lengths.get(rank.checked_sub(1)?)?;
    if !WRAP_WIDTHS.contains(&width) {
        return None;
    }

    let mut separated = false;
    let mut prose_seen = false;
    let mut blank_since_prose = false;
    for line in lines {
        match line {
            Line::Blank => blank_since_prose = prose_seen,
            Line::Prose(_) => {
                separated |= blank_since_prose;
                prose_seen = true;
            }
            Line::SceneBreak(_) | Line::Heading { .. } => {}
        }
    }

    let mut widths = vec![width; lines.len()];
    let mut followed = 0usize;
    let mut full = 0usize;
    for stretch in stretches(lines) {
        let longest = stretch
            .clone()
            .filter_map(|index| Some((index, lines[index].prose_len()?)))
            .filter(|(_, len)| *len <= width)
            .max_by_key(|(_, len)| *len);
        let Some((longest, own)) = longest else {
            continue;
        };
        if own + WIDTH_SLACK < width {
            continue;
        }
        widths[stretch.clone()].fill(own);
        for index in stretch.start..stretch.end - 1 {
            if let (Line::Prose(text), Line::Prose(next)) = (&lines[index], &lines[index + 1]) {
                followed += 1;
                let against = if index == longest { width } else { own };
                if is_full(text, next, against) {
                    full += 1;
                }
            }
        }
    }
    (separated && followed > 0 && full * 4 >= followed * 3).then_some(widths)
}

/// Each run of consecutive prose lines, as a range of line indices.
fn stretches(lines: &[Line<'_>]) -> Vec<std::ops::Range<usize>> {
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    for (index, line) in lines.iter().enumerate() {
        match (line, start) {
            (Line::Prose(_), None) => start = Some(index),
            (Line::Prose(_), Some(_)) => {}
            (_, Some(from)) => {
                out.push(from..index);
                start = None;
            }
            (_, None) => {}
        }
    }
    if let Some(from) = start {
        out.push(from..lines.len());
    }
    out
}

/// Whether `line` ran out of room: the first word of `next` would have made it longer
/// than `width`.
fn is_full(line: &str, next: &str, width: usize) -> bool {
    let first_word = next
        .split(skrib_format::is_djot_whitespace)
        .find(|word| !word.is_empty())
        .map_or(0, |word| word.chars().count());
    line.chars().count() + 1 + first_word > width
}

/// Turn the lines gathered since the last boundary into one prose block.
///
/// The plain text beside the Djot comes from parsing that Djot, so a block describes
/// itself in the coordinates the editor will read it in, whichever scanner made it.
fn flush_prose(prose: &mut String, blocks: &mut Vec<SourceBlock>) -> Result<()> {
    let djot = skrib_format::plain_text_to_djot_verbatim(prose);
    prose.clear();
    if djot.is_empty() {
        return Ok(());
    }
    let (text, _) = skrib_format::djot_plain_text(&djot)?;
    blocks.push(SourceBlock::Prose { djot, text });
    Ok(())
}

/// Three or more of one of `-`, `*` and `_`, with nothing else on the line but spaces or
/// tabs between them.
fn is_drawn_rule(line: &str) -> bool {
    let mut marks = line.chars().filter(|c| !matches!(c, ' ' | '\t'));
    let Some(first) = marks.next() else {
        return false;
    };
    matches!(first, '-' | '*' | '_') && marks.clone().all(|c| c == first) && marks.count() >= 2
}

/// A line opening with one to six `#` and then a space or a tab, as its level and text.
///
/// A closing run of `#` after a space is dropped, as Markdown drops it, so `## Title ##`
/// is the heading "Title".
fn heading(line: &str) -> Option<(u8, String)> {
    let level = line.chars().take_while(|c| *c == '#').count();
    if level == 0 || level > 6 {
        return None;
    }
    // `#` is one byte, so the count is also where the rest starts.
    let rest = &line[level..];
    if !rest.is_empty() && !rest.starts_with([' ', '\t']) {
        return None;
    }
    let text = rest.trim_matches([' ', '\t']);
    let without_closing = text.trim_end_matches('#');
    let text = if without_closing.is_empty() {
        ""
    } else if without_closing.ends_with([' ', '\t']) {
        without_closing.trim_end_matches([' ', '\t'])
    } else {
        text
    };
    Some((u8::try_from(level).ok()?, text.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(src: &str) -> SourceDocument {
        PlainTextScanner
            .scan(src.as_bytes(), "s", "s.txt")
            .expect("scan")
    }

    /// Every prose block's Djot, read back through the parser the editor uses.
    fn prose_read_back(doc: &SourceDocument) -> Vec<String> {
        doc.blocks
            .iter()
            .filter_map(|b| match b {
                SourceBlock::Prose { djot, text } => {
                    let read = skrib_format::djot_plain_text(djot).expect("parse").0;
                    assert_eq!(&read, text, "a block's text is its own Djot's parse");
                    Some(read)
                }
                _ => None,
            })
            .collect()
    }

    #[test]
    fn unstructured_text_becomes_one_row() {
        let doc = scan("She turned the corner and the street was gone.");
        assert!(matches!(doc.blocks.as_slice(), [SourceBlock::Prose { .. }]));
    }

    #[test]
    fn a_drawn_break_in_plain_text_is_still_a_break() {
        let doc = scan("Before.\n\n* * *\n\nAfter.");
        assert!(doc.blocks.iter().any(|b| matches!(
            b,
            SourceBlock::SceneBreak {
                tier: SceneBreakTier::Minor
            }
        )));
    }

    /// The strings the Markdown route changed, each kept as typed.
    #[test]
    fn plain_text_reads_back_character_for_character() {
        let typed = "Meet at 10:30:45, don't be late -- \"sharp\"...\n\
                     I. Introduction\n\
                     *not* emphasis and snake_case_name\n\
                     - not a list item\n\
                     A. Smith said so.";
        let doc = scan(typed);
        assert_eq!(prose_read_back(&doc), vec![typed.to_string()]);
    }

    /// One paragraph per line, which is what Skribisto's own `.txt` export writes: the
    /// Markdown route ran lines with no blank line between them into one paragraph.
    #[test]
    fn every_line_is_its_own_paragraph() {
        let doc = scan("First paragraph.\r\nSecond paragraph.\rThird.\n\n\nFourth.");
        let [SourceBlock::Prose { djot, .. }] = doc.blocks.as_slice() else {
            panic!("one prose run: {:?}", doc.blocks);
        };
        let (text, starts) = skrib_format::djot_plain_text(djot).expect("parse");
        assert_eq!(text, "First paragraph.\nSecond paragraph.\nThird.\nFourth.");
        assert_eq!(starts.len(), 4);
    }

    /// `paragraph` wrapped the way a text editor or a mail client wraps it: as many words
    /// on a line as fit in `width`.
    fn wrap(paragraph: &str, width: usize) -> String {
        let mut lines: Vec<String> = vec![String::new()];
        for word in paragraph.split(' ') {
            let line = lines.last_mut().expect("one line at least");
            if line.is_empty() {
                line.push_str(word);
            } else if line.chars().count() + 1 + word.chars().count() <= width {
                line.push(' ');
                line.push_str(word);
            } else {
                lines.push(word.to_string());
            }
        }
        lines.join("\n")
    }

    /// Every paragraph of the scanned prose, as the editor reads them.
    fn paragraphs(doc: &SourceDocument) -> Vec<String> {
        prose_read_back(doc)
            .iter()
            .flat_map(|text| text.split('\n').map(str::to_string).collect::<Vec<_>>())
            .collect()
    }

    const FIRST: &str = "It was a dark and stormy night; the rain fell in torrents, except at \
                         occasional intervals, when it was checked by a violent gust of wind \
                         which swept up the streets. They met at 10:30:45 -- \"sharp\", as \
                         agreed... I. Introduction was never read aloud.";
    const SECOND: &str = "The second paragraph is wrapped at the same width, so its lines run \
                          on into one another as well, and a line that happens to open with \
                          - a dash or 2. a number is still the middle of a sentence.";

    /// A file wrapped at a fixed width, a blank line between paragraphs, keeps each
    /// paragraph whole, every character as typed. Reading it one paragraph per line cut
    /// each sentence that crossed a line end.
    #[test]
    fn a_hard_wrapped_file_keeps_its_paragraphs_whole() {
        for width in [60, 72, 79] {
            for ending in ["\n", "\r\n"] {
                let file = format!("{}\n\n{}\n", wrap(FIRST, width), wrap(SECOND, width))
                    .replace('\n', ending);
                assert!(file.lines().count() > 6, "the fixture is wrapped: {file}");
                assert_eq!(
                    paragraphs(&scan(&file)),
                    vec![FIRST.to_string(), SECOND.to_string()],
                    "wrapped at {width}, lines ending {ending:?}"
                );
            }
        }
    }

    /// Inside a wrapped file, a line that did not run out of room ends its paragraph, so an
    /// address or a verse set line by line keeps its lines.
    #[test]
    fn a_short_line_in_a_wrapped_file_keeps_its_break() {
        let file = format!(
            "{}\n\nMr. Sherlock Holmes\n221B Baker Street\nLondon\n\n{}",
            wrap(FIRST, 52),
            wrap(SECOND, 52)
        );
        assert_eq!(
            paragraphs(&scan(&file)),
            vec![
                FIRST,
                "Mr. Sherlock Holmes",
                "221B Baker Street",
                "London",
                SECOND
            ]
        );
    }

    const THIRD: &str = "A third paragraph gives the file a little more body, so the measure \
                         has lines enough to judge, and it goes on for a while longer than \
                         the others did.";
    const FOURTH: &str = "Nobody on the quay had seen the ferry leave, and nobody could say \
                          when it would be back, though everyone had an opinion on the matter \
                          and most of them were shared loudly, in the rain, with whoever stood \
                          nearest.";
    const FIFTH: &str = "She kept the letter folded in her coat for a week before she read it, \
                         and when she finally did, standing under the awning of the closed \
                         bakery with the street empty around her, the ink had already begun \
                         to run.";

    /// One line longer than the rest does not decide the width the file is wrapped at: a
    /// link that cannot be broken, a line nobody wrapped, a title set in the middle of the
    /// line. The paragraphs around it are still joined back together.
    #[test]
    fn a_line_longer_than_the_rest_does_not_unwrap_the_file() {
        for line in [
            "See https://www.example.org/a/rather/long/path/to/a/page/that/does/not/wrap/because/urls/do/not.html",
            "\"You will find me,\" she wrote at the foot of the page, \"at the old house by the quay.\"",
            "                         THE THIRD CHAPTER, IN WHICH NOTHING AT ALL HAPPENS",
        ] {
            let file = format!(
                "{}\n\n{line}\n\n{}\n\n{}\n",
                wrap(FIRST, 72),
                wrap(SECOND, 72),
                wrap(THIRD, 72)
            );
            assert_eq!(
                paragraphs(&scan(&file)),
                vec![FIRST, line.trim_start(), SECOND, THIRD],
                "{line:?}"
            );
        }
    }

    /// A link too long for any line sits on a line of its own inside its paragraph, since
    /// a wrapper cannot break it. That line ran out of room like the others, so it joins
    /// the lines around it.
    #[test]
    fn a_link_too_long_for_its_line_stays_inside_its_paragraph() {
        let with_link = "Read the notice at \
                         https://www.example.org/a/rather/long/path/to/a/page/that/does/not/wrap.html \
                         before you go any further with this.";
        let file = format!(
            "{}\n\n{}\n\n{}\n",
            wrap(FIRST, 72),
            wrap(with_link, 72),
            wrap(SECOND, 72)
        );
        assert!(
            file.lines().any(|line| line.starts_with("https://")),
            "the link has a line of its own: {file}"
        );
        assert_eq!(paragraphs(&scan(&file)), vec![FIRST, with_link, SECOND]);
    }

    /// A file wrapped by hand, or by several tools over the years, is not wrapped at one
    /// width: most of its lines cluster a few characters under one width, with a tail of
    /// longer ones. Each paragraph is measured against its own lines, so none is cut
    /// because it ran a little narrower or wider than the rest.
    #[test]
    fn paragraphs_wrapped_at_slightly_different_widths_are_each_kept_whole() {
        let wrapped = [
            (FIRST, 72),
            (SECOND, 70),
            (THIRD, 76),
            (FOURTH, 68),
            (FIFTH, 78),
            (FIRST, 71),
        ];
        let file = wrapped
            .iter()
            .map(|(paragraph, width)| wrap(paragraph, *width))
            .collect::<Vec<_>>()
            .join("\n\n");
        let longest = file.lines().map(|l| l.chars().count()).max();
        assert_eq!(longest, Some(78), "the fixture has its longer tail");
        assert_eq!(
            paragraphs(&scan(&file)),
            wrapped
                .iter()
                .map(|(paragraph, _)| *paragraph)
                .collect::<Vec<_>>()
        );
    }

    /// Verse set line by line between wrapped paragraphs keeps its lines, and does not stop
    /// the paragraphs around it from being read as wrapped.
    #[test]
    fn verse_between_wrapped_paragraphs_keeps_its_lines() {
        let verse = [
            "Shall I compare thee to a summer's day?",
            "Thou art more lovely and more temperate:",
            "Rough winds do shake the darling buds of May,",
            "And summer's lease hath all too short a date;",
        ];
        let file = format!(
            "{}\n\n{}\n\n{}\n",
            wrap(FIRST, 72),
            verse.join("\n"),
            wrap(SECOND, 72)
        );
        let mut expected = vec![FIRST];
        expected.extend(verse);
        expected.push(SECOND);
        assert_eq!(paragraphs(&scan(&file)), expected);
    }

    /// One paragraph per line with a blank line here and there, a reply under its question
    /// and a stretch of narration on its own, is not a wrapped file: no line in it ran out
    /// of room.
    #[test]
    fn one_line_paragraphs_between_blank_lines_are_not_read_as_wrapped() {
        let file = "\"Where were you last night?\"\n\
                    \"Out.\"\n\
                    \n\
                    She waited for more, and none came. The kettle began to sing on the stove behind her.\n\
                    \n\
                    \"Out where?\" she asked.\n\
                    He shrugged as if the question bored him, then picked up his coat and left.\n\
                    \n\
                    Nobody followed him down the stairs.";
        let expected: Vec<&str> = file.lines().filter(|l| !l.is_empty()).collect();
        assert_eq!(paragraphs(&scan(file)), expected);
    }

    /// Skribisto's own plain-text export, notes included: one paragraph per line, and the
    /// blank line before the notes does not make the paragraphs above it into wrapped lines,
    /// even when every paragraph is short enough to pass for one.
    #[test]
    fn the_apps_own_export_is_read_one_paragraph_per_line() {
        let paragraphs_in = [
            "The ferry was late, as it always was on a Tuesday in the rain.[^n1]",
            "\"Yes,\" she said.",
            "He did not answer. The engines turned over twice and caught at last.",
            "Nobody moved.",
            "Below them the harbour lights came on one at a time, then all at once.",
            "\"Now,\" he said.",
        ];
        let djot = format!("{}\n\n[^n1]: It always is.", paragraphs_in.join("\n\n"));
        let doc = text_document::TextDocument::new();
        doc.set_djot(&djot).expect("parse").wait().expect("parse");
        let exported = doc
            .to_plain_text_with(text_document::PlainTextExportOptions::presentation())
            .expect("export");
        assert!(exported.contains("\n\n1. It always is."), "{exported:?}");

        let read = paragraphs(&scan(&exported));
        let expected: Vec<String> = exported
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(str::to_string)
            .collect();
        assert_eq!(read, expected);
        assert_eq!(read.len(), paragraphs_in.len() + 1);
    }

    /// Lines of every length with a blank line here and there are not a wrapped file.
    #[test]
    fn a_file_of_short_paragraphs_is_not_read_as_wrapped() {
        let file = "A line of some length that could pass for a wrapped one at first.\n\
                    Short.\n\
                    Another line, of a different length again, then a blank one.\n\
                    \n\
                    And a last line.";
        assert_eq!(
            paragraphs(&scan(file)),
            vec![
                "A line of some length that could pass for a wrapped one at first.",
                "Short.",
                "Another line, of a different length again, then a blank one.",
                "And a last line.",
            ]
        );
    }

    #[test]
    fn a_hash_line_is_a_heading_and_the_apps_own_break_is_not() {
        let doc = scan("# Book\n\nOpening.\n\n## Chapter One ##\n\nWords.\n\n# # #\n\nMore.");
        let headings: Vec<(u8, &str)> = doc
            .blocks
            .iter()
            .filter_map(|b| match b {
                SourceBlock::Heading { level, text } => Some((*level, text.as_str())),
                _ => None,
            })
            .collect();
        assert_eq!(headings, vec![(1, "Book"), (2, "Chapter One")]);
        assert!(doc.blocks.iter().any(|b| matches!(
            b,
            SourceBlock::SceneBreak {
                tier: SceneBreakTier::Major
            }
        )));
        assert!(
            !doc.diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::NoHeadings { .. }))
        );
    }

    #[test]
    fn a_hash_with_no_space_is_prose() {
        let doc = scan("#hashtag stays a word");
        assert_eq!(prose_read_back(&doc), vec!["#hashtag stays a word"]);
    }

    /// A line of dashes, asterisks or underscores is how other tools draw a break. A line
    /// under a paragraph is still a break here, never the underline of a heading.
    #[test]
    fn a_ruled_line_is_an_ordinary_break() {
        for rule in ["---", "___", "- - -", "*****"] {
            let doc = scan(&format!("Before.\n{rule}\nAfter."));
            let breaks = doc
                .blocks
                .iter()
                .filter(|b| matches!(b, SourceBlock::SceneBreak { .. }))
                .count();
            assert_eq!(breaks, 1, "{rule:?}: {:?}", doc.blocks);
            assert_eq!(prose_read_back(&doc), vec!["Before.", "After."], "{rule:?}");
        }
        assert!(!is_drawn_rule("-*-"), "mixed marks draw nothing");
        assert!(!is_drawn_rule("--"), "two marks are not a rule");
    }

    #[test]
    fn front_matter_is_metadata_not_prose() {
        let doc = scan("---\ntitle: The Book\n---\nFirst line.");
        assert_eq!(doc.metadata.title.as_deref(), Some("The Book"));
        assert_eq!(prose_read_back(&doc), vec!["First line."]);
    }
}
