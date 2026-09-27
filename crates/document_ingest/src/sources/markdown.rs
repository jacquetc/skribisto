// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Markdown scanner.
//!
//! ## Why this owns its own parse
//!
//! Prose conversion goes through `text-document` — it parses into a model that
//! records *"this run is italic"* rather than which delimiter said so, which is
//! what stops CommonMark and Djot's swapped emphasis markers (`*x*` is emphasis
//! in one and strong in the other) from silently bolding every italicised word in
//! an imported novel. But `text-document` exposes no source offsets, and it
//! *drops* thematic breaks outright — its Markdown reader has no arm for
//! `Event::Rule`. So structure has to be found here, on the raw text, before a
//! byte of it is handed over.
//!
//! ## The one rule that matters
//!
//! **A scene-break match wins over whatever the parser called the block.** Run
//! against real `pulldown-cmark` output, the app's own break glyphs classify as:
//!
//! | source line | parsed as |
//! |---|---|
//! | `* * *`, `***`, `---`, `___` | `Rule` |
//! | `*` | a `List` with one **empty** item |
//! | `#`, `###` | a `Heading` with **no text child** |
//! | `# # #` | a `Heading` whose text is `#` |
//! | `⁂` `…` `＊` `◇` | a `Paragraph` |
//!
//! So Skribisto's own major break, `# # #`, is a level-1 heading to any
//! CommonMark parser. Classify headings first and re-importing the app's own
//! export invents a near-empty Book called `#` — and since a detected break is
//! preserved inline rather than proposed for review, nothing downstream would
//! catch it. Hence: match the vocabulary against each block's **whole raw source
//! span** first, and only then ask what the parser thought.
//!
//! Comparing the raw span, never the parser's inner text, is load-bearing too:
//! `# # #` yields a text event of just `#`, which is indistinguishable from a
//! genuine one-character heading.

use anyhow::Result;
use pulldown_cmark::{BrokenLink, Event, Options, Parser, RefDefs, Tag, TagEnd};
use skribisto_model::scene_break::{self, SceneBreakTier};

use crate::block::{SourceBlock, SourceDocument};
use crate::diagnostics::ImportDiagnostic;
use crate::front_matter;
use crate::scanner::SourceScanner;
use crate::sources::MAX_LIST_LEVELS;
use crate::text;

/// Matches what `text-document`'s own Markdown reader enables, so this scan and
/// the conversion that follows it cannot disagree about where a block begins.
fn options() -> Options {
    Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES | Options::ENABLE_TASKLISTS
}

pub struct MarkdownScanner;

impl SourceScanner for MarkdownScanner {
    fn extensions(&self) -> &[&str] {
        &["md", "markdown", "mdown", "mkd"]
    }

    fn format_name(&self) -> &'static str {
        "markdown"
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
        let body = &decoded.text[fm.body_offset..];

        doc.blocks = segment(body, origin, &mut doc.diagnostics)?;

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

/// What a top-level block turned out to be.
enum Boundary {
    /// Text is not carried here: a heading's own text arrives as later `Text`
    /// events, so only its depth is known at the block's start event.
    Heading {
        level: u8,
    },
    SceneBreak(SceneBreakTier),
    /// Ordinary content: extend the prose run.
    Prose,
}

/// Walk the document's top-level blocks, cutting it into headings, scene breaks
/// and the prose runs between them.
fn segment(
    body: &str,
    origin: &str,
    diagnostics: &mut Vec<ImportDiagnostic>,
) -> Result<Vec<SourceBlock>> {
    let mut blocks = Vec::new();
    // The half-open byte range of every top-level block of the prose accumulated since
    // the last boundary, in order.
    let mut prose: Vec<(usize, usize)> = Vec::new();
    let mut depth = 0usize;
    let mut pending_heading: Option<(u8, usize, usize)> = None;
    let mut heading_text = String::new();
    let mut html_blocks = 0usize;
    let mut nested_rules = 0usize;
    let mut images = Vec::new();
    // What the conversions of the prose runs reported, summed.
    let mut counts = RunCounts::default();
    // How many lists are open at the current event, and the most that were open at once
    // since the prose run began: a run holding a list nested past `MAX_LIST_LEVELS` is
    // converted with its lists held to that depth.
    let mut open_lists = 0usize;
    let mut deepest_list = 0usize;

    // Every link reference definition of the file, found by the label a run cites. A
    // definition is no block of its own to the parser, so a run holds only those that come
    // before its last block; the ones it cites from elsewhere are handed over with it
    // (see `convert`).
    let lookup = Parser::new_ext(body, options());
    let definitions = lookup.reference_definitions();

    let flush_prose = |prose: &mut Vec<(usize, usize)>,
                       blocks: &mut Vec<SourceBlock>,
                       counts: &mut RunCounts,
                       deepest_list: &mut usize| {
        let parts = std::mem::take(prose);
        let deep_lists = std::mem::take(deepest_list) > MAX_LIST_LEVELS;
        let (Some((start, _)), Some((_, end))) = (parts.first(), parts.last()) else {
            return Ok(());
        };
        if !body[*start..*end].trim().is_empty() {
            let run = convert_run(body, &parts, definitions, deep_lists)?;
            counts.flattened += run.flattened;
            counts.lists_held += run.lists_held;
            if !run.djot.trim().is_empty() {
                blocks.push(SourceBlock::Prose {
                    djot: run.djot,
                    text: run.text,
                });
            }
        }
        Ok::<(), anyhow::Error>(())
    };

    for (event, range) in Parser::new_ext(body, options()).into_offset_iter() {
        match &event {
            Event::Start(Tag::List(_)) => {
                open_lists += 1;
                deepest_list = deepest_list.max(open_lists);
            }
            Event::End(TagEnd::List(_)) => open_lists = open_lists.saturating_sub(1),
            _ => {}
        }
        match &event {
            Event::Start(tag) => {
                if depth == 0 {
                    match classify(&body[range.clone()], tag) {
                        Boundary::SceneBreak(tier) => {
                            flush_prose(&mut prose, &mut blocks, &mut counts, &mut deepest_list)?;
                            blocks.push(SourceBlock::SceneBreak { tier });
                        }
                        Boundary::Heading { level } => {
                            flush_prose(&mut prose, &mut blocks, &mut counts, &mut deepest_list)?;
                            heading_text.clear();
                            pending_heading = Some((level, range.start, range.end));
                        }
                        Boundary::Prose => {
                            extend(&mut prose, line_start(body, range.start), range.end)
                        }
                    }
                }
                depth += 1;
            }
            Event::End(_) => {
                depth = depth.saturating_sub(1);
                if depth == 0
                    && let Some((level, _, _)) = pending_heading.take()
                {
                    blocks.push(SourceBlock::Heading {
                        level,
                        text: heading_text.trim().to_string(),
                    });
                }
            }
            // A thematic break is never anything but a scene break — it is the
            // most common way a manuscript from another tool spells one, and the
            // conversion layer would delete it if it were left in the prose.
            Event::Rule if depth == 0 => {
                flush_prose(&mut prose, &mut blocks, &mut counts, &mut deepest_list)?;
                let tier = scene_break::tier_of_plain_line(body[range.clone()].trim())
                    .unwrap_or(SceneBreakTier::Minor);
                blocks.push(SourceBlock::SceneBreak { tier });
            }
            // A thematic break *inside* another construct (a block quote, a list
            // item) is not caught by the depth == 0 arm above, so it rides along
            // inside that construct's own merged prose span and reaches
            // `markdown_to_djot` verbatim — which has no arm for `Event::Rule`
            // either, and drops it just as silently. Counted rather than fixed:
            // recognising a break nested inside arbitrary containers is a bigger
            // change than reporting that one was lost.
            Event::Rule if depth > 0 => {
                nested_rules += 1;
            }
            Event::Text(t) | Event::Code(t) if pending_heading.is_some() => {
                heading_text.push_str(t);
            }
            Event::Html(_) if depth == 0 => {
                html_blocks += 1;
                extend(&mut prose, line_start(body, range.start), range.end);
            }
            _ => {}
        }

        if let Event::Start(Tag::Image { dest_url, .. }) = &event {
            images.push(dest_url.to_string());
        }
    }
    flush_prose(&mut prose, &mut blocks, &mut counts, &mut deepest_list)?;

    if html_blocks > 0 {
        diagnostics.push(ImportDiagnostic::RawHtmlDropped {
            path: origin.to_string(),
            count: html_blocks,
        });
    }
    if nested_rules > 0 {
        diagnostics.push(ImportDiagnostic::NestedBreakDropped {
            path: origin.to_string(),
            count: nested_rules,
        });
    }
    if counts.flattened > 0 {
        diagnostics.push(ImportDiagnostic::ProseNotVerbatim {
            path: origin.to_string(),
            count: counts.flattened,
        });
    }
    if counts.lists_held > 0 {
        diagnostics.push(ImportDiagnostic::ListNestingFlattened {
            path: origin.to_string(),
            count: counts.lists_held,
            limit: MAX_LIST_LEVELS,
        });
    }
    for target in images {
        diagnostics.push(ImportDiagnostic::ImageNotIngested {
            path: origin.to_string(),
            target,
        });
    }
    let footnotes = count_footnote_definitions(body);
    if footnotes > 0 {
        diagnostics.push(ImportDiagnostic::FootnotesDegraded {
            path: origin.to_string(),
            count: footnotes,
        });
    }

    Ok(blocks)
}

/// Decide what one top-level block is, vocabulary first.
fn classify(raw_span: &str, tag: &Tag<'_>) -> Boundary {
    if let Some(tier) = scene_break::tier_of_plain_line(raw_span.trim()) {
        return Boundary::SceneBreak(tier);
    }
    match tag {
        Tag::Heading { level, .. } => Boundary::Heading {
            level: *level as u8,
        },
        _ => Boundary::Prose,
    }
}

/// Where the line holding `at` starts, when nothing but spaces and tabs come before `at`
/// on it, and `at` itself otherwise.
///
/// A top-level block's range starts after its indentation, and the indentation is part of
/// what the block is: an indented code block handed over from its first character is a
/// paragraph. Converted alone, as a prose run's first block is, its code became prose, and
/// a bracket in it a link.
fn line_start(body: &str, at: usize) -> usize {
    let start = body[..at].rfind('\n').map_or(0, |newline| newline + 1);
    if body[start..at].bytes().all(|b| b == b' ' || b == b'\t') {
        start
    } else {
        at
    }
}

/// Add the top-level block at `start..end` to the prose run. A range overlapping the last
/// one widens it instead, so the parts stay in order and apart, and the run still spans
/// from the first block's start to the furthest end any of them reached.
fn extend(prose: &mut Vec<(usize, usize)>, start: usize, end: usize) {
    match prose.last_mut() {
        Some((_, last_end)) if start < *last_end => *last_end = (*last_end).max(end),
        _ => prose.push((start, end)),
    }
}

/// What converting the prose runs reported, summed over the file.
#[derive(Default)]
struct RunCounts {
    /// Paragraphs stored as plain text because their markup nested past what a project
    /// may hold (see `skrib_format::markdown_to_djot_and_text`).
    flattened: usize,
    /// List items nested past [`MAX_LIST_LEVELS`], stored at that level.
    lists_held: usize,
}

/// A prose run as it is stored.
struct ConvertedRun {
    djot: String,
    text: String,
    /// How many of its paragraphs were stored as plain text.
    flattened: usize,
    /// How many of its list items were nested past [`MAX_LIST_LEVELS`], and stored at
    /// that level, their words, marker and formatting kept.
    lists_held: usize,
}

/// Convert the prose run made of the top-level blocks `parts` of `body`.
///
/// Both answers come from one parse: the Djot that gets stored, and the plain text an
/// annotation would be measured against. Markdown carries no annotations, but a block must
/// describe itself the same way whichever scanner made it.
///
/// The run is converted whole, which keeps what one paragraph borrows from another (a link
/// whose reference is defined further down). The definitions it cites from elsewhere in
/// the file, found in `definitions`, go with it: one written after the run's last
/// paragraph, at the end of a chapter or all together at the end of the file as Markdown
/// writers often put them, is outside every run, and the link citing it used to arrive as
/// its brackets.
///
/// Markup nesting past what a project may hold makes the converter keep only the words of
/// everything it was handed (`skrib_format::markdown_to_djot_and_text`), so a run where
/// that happened is converted again a top-level block at a time: only the blocks nested
/// too deep lose their formatting, and a chapter keeps its italics around one pathological
/// quotation. A run holding a list nested past [`MAX_LIST_LEVELS`] (`deep_lists`) has its
/// deeper items written at that level, as a Word or OpenDocument list is.
fn convert_run(
    body: &str,
    parts: &[(usize, usize)],
    definitions: &RefDefs<'_>,
    deep_lists: bool,
) -> Result<ConvertedRun> {
    let (Some((start, _)), Some((_, end))) = (parts.first(), parts.last()) else {
        return Ok(ConvertedRun {
            djot: String::new(),
            text: String::new(),
            flattened: 0,
            lists_held: 0,
        });
    };
    let (whole, lists_held) = convert(body, *start..*end, definitions, deep_lists)?;
    // One paragraph per line of what a flattened conversion kept.
    let whole_run = |whole: skrib_format::ConvertedDjot, lists_held: usize| {
        let flattened = if whole.flattened {
            whole.text.lines().count()
        } else {
            0
        };
        ConvertedRun {
            djot: whole.djot,
            text: whole.text,
            flattened,
            lists_held,
        }
    };
    if !whole.flattened || parts.len() == 1 {
        return Ok(whole_run(whole, lists_held));
    }

    let mut djot: Vec<String> = Vec::with_capacity(parts.len());
    let mut flattened = 0usize;
    let mut held = 0usize;
    for (start, end) in parts {
        let (part, part_held) = convert(body, *start..*end, definitions, deep_lists)?;
        if part.flattened {
            flattened += part.text.lines().count();
        }
        held += part_held;
        if !part.djot.trim().is_empty() {
            djot.push(part.djot);
        }
    }
    let djot = djot.join("\n\n");
    // Each part is within the ceiling on its own, and blocks separated by a blank line do
    // not nest inside one another; should a part leave something open that the next one
    // closes over, the words of the whole run are what is stored. Lines a load would join
    // are joined here the same way.
    let Ok(djot) = skrib_format::djot_depth::admit(djot) else {
        return Ok(whole_run(whole, 0));
    };
    let (text, _) = skrib_format::djot_plain_text(&djot)?;
    Ok(ConvertedRun {
        djot,
        text,
        flattened,
        lists_held: held,
    })
}

/// Convert `body[range]` with the link reference definitions it cites from elsewhere in
/// the file ([`cited_elsewhere`]), and hold its lists to [`MAX_LIST_LEVELS`] when
/// `deep_lists`. The second answer is how many list items that moved.
///
/// Those definitions are appended after a blank line, where they add no text: a
/// definition prints nothing. They are left off when the range leaves a block open that
/// would take them in as its own text (a fenced code block never closed, say), which the
/// parse shows by reaching into them, and when the range is stored as its words alone,
/// where no link is kept for them to serve and their lines would be words too.
fn convert(
    body: &str,
    range: std::ops::Range<usize>,
    definitions: &RefDefs<'_>,
    deep_lists: bool,
) -> Result<(skrib_format::ConvertedDjot, usize)> {
    let markdown_to_djot = |markdown: &str| {
        if deep_lists {
            skrib_format::markdown_to_djot_within_list_levels(markdown, MAX_LIST_LEVELS)
        } else {
            Ok((skrib_format::markdown_to_djot_and_text(markdown)?, 0))
        }
    };
    let own = &body[range.clone()];
    let elsewhere: Vec<&str> = cited_elsewhere(body, range, definitions)
        .into_iter()
        .map(|(start, end)| body[start..end].trim())
        .collect();
    if !elsewhere.is_empty() {
        let joined = format!("{own}\n\n{}\n", elsewhere.join("\n"));
        let tail = own.len() + 2;
        let absorbed = Parser::new_ext(&joined, options())
            .into_offset_iter()
            .any(|(_, range)| range.end > tail);
        if !absorbed {
            let converted = markdown_to_djot(&joined)?;
            if !converted.0.flattened {
                return Ok(converted);
            }
        }
    }
    markdown_to_djot(own)
}

/// The byte ranges in `body` of the link reference definitions `body[range]` cites and
/// does not hold, in the file's order, each once.
///
/// Only those: a file whose links are all defined together at its end would otherwise
/// hand every definition to every chapter, and parse and convert them all again for each,
/// which made importing it several times slower. A citation is what the parser, reading
/// the range alone, finds no definition for; the file's own table (`definitions`, the
/// first definition of each label, matched as the parser matches labels) says where it
/// is.
fn cited_elsewhere(
    body: &str,
    range: std::ops::Range<usize>,
    definitions: &RefDefs<'_>,
) -> Vec<(usize, usize)> {
    let mut cited: std::collections::BTreeSet<(usize, usize)> = Default::default();
    let note = |link: BrokenLink<'_>| {
        if let Some(definition) = definitions.get(link.reference.as_ref()) {
            let (start, end) = (definition.span.start, definition.span.end);
            if end <= range.start || start >= range.end {
                cited.insert((start, end));
            }
        }
        None
    };
    Parser::new_with_broken_link_callback(&body[range.clone()], options(), Some(note))
        .for_each(drop);
    cited.into_iter().collect()
}

/// Footnote *definitions* — lines like `[^1]: the note`.
///
/// Counted from the raw text rather than from parser events because the parse
/// deliberately leaves `ENABLE_FOOTNOTES` off, to stay in step with the
/// conversion layer. They survive as literal text either way; this is what makes
/// that visible instead of a surprise in the finished book.
fn count_footnote_definitions(body: &str) -> usize {
    body.lines()
        .filter(|line| {
            let t = line.trim_start();
            t.starts_with("[^") && t.split_once("]:").is_some()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(src: &str) -> SourceDocument {
        MarkdownScanner
            .scan(src.as_bytes(), "fixture", "fixture.md")
            .expect("scan")
    }

    fn headings(doc: &SourceDocument) -> Vec<(u8, &str)> {
        doc.blocks
            .iter()
            .filter_map(|b| match b {
                SourceBlock::Heading { level, text } => Some((*level, text.as_str())),
                _ => None,
            })
            .collect()
    }

    fn breaks(doc: &SourceDocument) -> Vec<SceneBreakTier> {
        doc.blocks
            .iter()
            .filter_map(|b| match b {
                SourceBlock::SceneBreak { tier } => Some(*tier),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn headings_become_boundaries_with_their_depth() {
        let doc = scan("# Book\n\nprose\n\n## Chapter One\n\nmore prose\n");
        assert_eq!(headings(&doc), vec![(1, "Book"), (2, "Chapter One")]);
    }

    /// The whole reason this scanner owns its own parse.
    #[test]
    fn every_break_spelling_is_recognised_and_never_becomes_a_heading() {
        for (src, expected) in [
            ("* * *", SceneBreakTier::Minor),
            ("***", SceneBreakTier::Minor),
            ("*", SceneBreakTier::Minor),
            ("#", SceneBreakTier::Minor),
            ("###", SceneBreakTier::Major),
            ("# # #", SceneBreakTier::Major),
            ("⁂", SceneBreakTier::Major),
            (". . .", SceneBreakTier::Major),
            ("＊", SceneBreakTier::Minor),
            ("◇", SceneBreakTier::Major),
            // Not in any preset, but the commonest spelling from other tools.
            ("---", SceneBreakTier::Minor),
            ("___", SceneBreakTier::Minor),
        ] {
            let doc = scan(&format!("Before.\n\n{src}\n\nAfter."));
            assert_eq!(breaks(&doc), vec![expected], "for {src:?}");
            assert!(headings(&doc).is_empty(), "{src:?} must not be a heading");
        }
    }

    /// Re-importing Skribisto's own Glyph-preset export. `# # #` is an ATX
    /// heading to CommonMark, so getting this wrong invents a spurious Book.
    #[test]
    fn the_apps_own_major_glyph_survives_a_round_trip() {
        let doc = scan("## Chapter One\n\nProse.\n\n# # #\n\nMore prose.");
        assert_eq!(headings(&doc), vec![(2, "Chapter One")]);
        assert_eq!(breaks(&doc), vec![SceneBreakTier::Major]);
    }

    #[test]
    fn prose_that_merely_looks_like_a_marker_stays_prose() {
        for src in [
            // A beat of silence, not furniture — and the shape smart punctuation
            // produces from a typed `...`, so it is the common spelling rather
            // than an exotic one.
            "…",
            "...",
            "*word*",
            "* a bullet-looking line",
            "# Chapter One",
            "He rated it 5 stars: *****!",
            "#hashtag",
        ] {
            let doc = scan(src);
            assert!(breaks(&doc).is_empty(), "{src:?} was read as a scene break");
        }
    }

    #[test]
    fn a_hash_inside_a_fenced_code_block_is_not_a_heading() {
        let doc = scan("Prose.\n\n```\n# not a heading\n```\n");
        assert!(headings(&doc).is_empty());
    }

    /// `---` under a text line is a setext H2, and CommonMark says so — the
    /// parser resolves the ambiguity, so no hand-rolled guard is needed.
    #[test]
    fn a_setext_underline_is_a_heading_not_a_break() {
        let doc = scan("Chapter One\n---\n\nProse.");
        assert_eq!(headings(&doc), vec![(2, "Chapter One")]);
        assert!(breaks(&doc).is_empty());
    }

    #[test]
    fn emphasis_survives_as_djot_rather_than_flipping_to_strong() {
        let doc = scan("He was *utterly* lost.");
        let SourceBlock::Prose { djot, .. } = &doc.blocks[0] else {
            panic!("expected prose, got {:?}", doc.blocks);
        };
        assert!(djot.contains("_utterly_"), "got {djot:?}");
    }

    #[test]
    fn front_matter_supplies_the_title_and_never_reaches_the_prose() {
        let doc = scan("---\ntitle: The Storm\norder: 2\n---\n\nProse.");
        assert_eq!(doc.metadata.title.as_deref(), Some("The Storm"));
        assert_eq!(doc.metadata.order_hint, Some(2));
        assert_eq!(doc.effective_title(), "The Storm");
        let joined = format!("{:?}", doc.blocks);
        assert!(!joined.contains("title"), "front matter leaked: {joined}");
    }

    #[test]
    fn a_file_without_headings_says_so_and_still_imports() {
        let doc = scan("Just some prose, no structure at all.");
        assert!(headings(&doc).is_empty());
        assert!(matches!(doc.blocks.as_slice(), [SourceBlock::Prose { .. }]));
        assert!(
            doc.diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::NoHeadings { .. }))
        );
    }

    #[test]
    fn footnotes_and_images_are_reported_rather_than_vanishing() {
        let doc = scan("Text.[^1]\n\n![a photo](images/x.png)\n\n[^1]: The note.");
        assert!(
            doc.diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::FootnotesDegraded { count: 1, .. }))
        );
        assert!(
            doc.diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::ImageNotIngested { target, .. } if target == "images/x.png"))
        );
    }

    #[test]
    fn an_empty_file_is_reported_and_yields_nothing() {
        let doc = scan("   \n\n  ");
        assert!(doc.blocks.is_empty());
        assert!(
            doc.diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::EmptyFile { .. }))
        );
    }

    #[test]
    fn a_break_glued_to_prose_still_separates_it() {
        let doc = scan("Prose ends here.\n* * *\nMore prose.");
        assert_eq!(breaks(&doc), vec![SceneBreakTier::Minor]);
        assert_eq!(doc.blocks.len(), 3);
    }

    /// A thematic break *inside* a block quote is not a top-level boundary, so it
    /// rides along inside the quote's own merged prose span and reaches
    /// `markdown_to_djot` verbatim — which drops it, the same silent loss the
    /// depth == 0 handling exists to prevent. It must at least be named.
    #[test]
    fn a_thematic_break_nested_inside_a_block_quote_is_reported_not_silently_dropped() {
        let doc = scan("Prose.\n\n> Quoted.\n>\n> ***\n>\n> More quoted.\n\nAfter.");
        assert!(
            doc.diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::NestedBreakDropped { count: 1, .. })),
            "a break nested inside a block quote must be named, not silently \
             swallowed: {:?}",
            doc.diagnostics
        );
        // It must not have been promoted to a real scene break either — only a
        // top-level break is structural.
        assert!(breaks(&doc).is_empty());
    }

    /// A quotation nested two hundred deep converts to Djot the next load of the project
    /// would refuse. Its words are stored as plain text instead, the paragraphs around it
    /// keep their formatting, and the writer is told how many paragraphs lost theirs.
    #[test]
    fn markdown_nested_past_what_a_project_holds_arrives_as_words_and_is_reported() {
        let src = format!(
            "# Chapter\n\nAbove, *leaning*.\n\n{}Deep words.\n\nBelow, **firmly**.\n",
            "> ".repeat(200)
        );
        let doc = scan(&src);
        let prose: Vec<(&str, &str)> = doc
            .blocks
            .iter()
            .filter_map(|b| match b {
                SourceBlock::Prose { djot, text } => Some((djot.as_str(), text.as_str())),
                _ => None,
            })
            .collect();
        let [(djot, text)] = prose.as_slice() else {
            panic!("one prose block: {:?}", doc.blocks);
        };
        assert!(
            skrib_format::djot_depth::check(djot).is_ok(),
            "the next load must accept {djot:?}"
        );
        assert_eq!(*text, "Above, leaning.\nDeep words.\nBelow, firmly.");
        assert!(
            djot.contains("_leaning_") && djot.contains("*firmly*"),
            "the paragraphs around it keep their formatting: {djot:?}"
        );
        assert!(
            doc.diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::ProseNotVerbatim { count: 1, .. })),
            "{:?}",
            doc.diagnostics
        );
    }

    #[test]
    fn a_top_level_break_does_not_trigger_the_nested_diagnostic() {
        let doc = scan("Before.\n\n* * *\n\nAfter.");
        assert!(
            !doc.diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::NestedBreakDropped { .. })),
            "a top-level break is handled at depth == 0 and must not also be \
             counted as a dropped nested one: {:?}",
            doc.diagnostics
        );
    }

    /// Each prose run's Djot, in order.
    fn prose_djot(doc: &SourceDocument) -> Vec<&str> {
        doc.blocks
            .iter()
            .filter_map(|b| match b {
                SourceBlock::Prose { djot, .. } => Some(djot.as_str()),
                _ => None,
            })
            .collect()
    }

    /// A link reference definition prints nothing, so the parser gives it no block, and a
    /// definition after a passage's last paragraph fell outside the run converted. The
    /// link arrived as its brackets. Writers put them at the end of a chapter, or all
    /// together at the end of the file.
    #[test]
    fn a_link_defined_after_its_passage_keeps_its_address() {
        let doc =
            scan("# Chapter\n\nSee [the site][ref] now, *really*.\n\n[ref]: https://example.org\n");
        assert_eq!(
            prose_djot(&doc),
            vec!["See [the site](https://example.org) now, _really_."]
        );

        let doc = scan(
            "# One\n\nSee [a][r1] here.\n\n# Two\n\nAnd [b][r2] there.\n\n\
             [r1]: https://one.example\n[r2]: https://two.example \"Two\"\n",
        );
        assert_eq!(
            prose_djot(&doc),
            vec![
                "See [a](https://one.example) here.",
                "And [b](https://two.example) there."
            ]
        );
    }

    /// A definition handed to a passage that leaves a block open would become that
    /// block's text: a fence never closed runs to the end of the file, and whatever
    /// follows it is code. The passage cites a label defined in the chapter before, so a
    /// definition is handed to it; it is then converted without, its link arriving as the
    /// brackets it was written with.
    #[test]
    fn a_definition_never_becomes_the_text_of_an_open_block() {
        let doc = scan(
            "# Zero\n\n[r]: https://one.example\n\n# One\n\nSee [a][r].\n\n```\nnever closed\n",
        );
        let prose = prose_djot(&doc);
        assert_eq!(
            prose,
            vec!["See \\[a\\]\\[r\\].\n\n```\nnever closed\n```"],
            "the code holds only its own line"
        );
    }

    /// An indented code block opening a passage is code: the passage starts at the line
    /// its first block is on, indentation included. It started at the block's first
    /// character, which made the code a paragraph, and a bracket in it a link to whatever
    /// the file defined under that label.
    #[test]
    fn an_indented_code_block_opening_a_passage_stays_code() {
        let doc = scan(
            "# One\n\n    indented code [x][r1]\n\n# Two\n\nWords.\n\n[r1]: https://one.example\n",
        );
        assert_eq!(
            prose_djot(&doc),
            vec!["```\nindented code [x][r1]\n```", "Words."]
        );
    }

    /// A passage is handed the definitions it cites and holds nothing of its own for, each
    /// once, a label matched as the parser matches one (whatever its case), and no other.
    /// Every definition of the file went to every passage, and a book with its links
    /// defined at the end took several times longer to import.
    #[test]
    fn a_passage_is_handed_only_the_definitions_it_cites() {
        let body = "# One\n\nSee [a][r1], [again][r1] and [here][Own].\n\n[own]: https://own.example\n\n\
                    # Two\n\nAnd [b][R2] and [c][r1], [not a link].\n\n\
                    [r1]: https://one.example\n[r2]: https://two.example\n[r3]: https://three.example\n";
        let lookup = Parser::new_ext(body, options());
        let definitions = lookup.reference_definitions();
        let cited_by = |passage: &str| {
            let start = body.find(passage).expect("the passage is in the file");
            let found = cited_elsewhere(body, start..start + passage.len(), definitions);
            found
                .into_iter()
                .map(|(start, end)| body[start..end].trim())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            cited_by("See [a][r1], [again][r1] and [here][Own].\n\n[own]: https://own.example"),
            vec!["[r1]: https://one.example"]
        );
        assert_eq!(
            cited_by("And [b][R2] and [c][r1], [not a link]."),
            vec!["[r1]: https://one.example", "[r2]: https://two.example"]
        );
    }

    /// A list nested more than [`MAX_LIST_LEVELS`] deep arrives with its deeper items at
    /// the deepest level kept, words and formatting kept, and the wizard says how many:
    /// the rule a Word or OpenDocument list follows. Markdown's list used to arrive at
    /// whatever depth it had, and say nothing.
    #[test]
    fn a_list_nested_past_sixteen_levels_is_held_there_and_reported() {
        let mut src = String::from("Intro.\n\n");
        for level in 0..20 {
            src.push_str(&" ".repeat(level * 2));
            src.push_str(&format!("- item *{level}*\n"));
        }
        src.push_str("\nAfter.\n");
        let doc = scan(&src);
        let prose = prose_djot(&doc);
        let [djot] = prose.as_slice() else {
            panic!("one prose run: {:?}", doc.blocks);
        };
        let deepest = djot
            .lines()
            .filter(|line| line.trim_start().starts_with("- "))
            .map(|line| line.len() - line.trim_start().len())
            .max()
            .unwrap_or_default();
        assert_eq!(deepest, (MAX_LIST_LEVELS - 1) * 2, "{djot}");
        for level in 0..20 {
            assert!(
                djot.contains(&format!("- item _{level}_")),
                "{level}: {djot}"
            );
        }
        assert!(
            doc.diagnostics.iter().any(|d| matches!(
                d,
                ImportDiagnostic::ListNestingFlattened {
                    count: 4,
                    limit: MAX_LIST_LEVELS,
                    ..
                }
            )),
            "{:?}",
            doc.diagnostics
        );

        // Sixteen levels are kept as they are, and nothing is said.
        let within: String = (0..MAX_LIST_LEVELS)
            .map(|level| format!("{}- item {level}\n", " ".repeat(level * 2)))
            .collect();
        let doc = scan(&within);
        assert!(
            !doc.diagnostics
                .iter()
                .any(|d| matches!(d, ImportDiagnostic::ListNestingFlattened { .. })),
            "{:?}",
            doc.diagnostics
        );
    }
}
