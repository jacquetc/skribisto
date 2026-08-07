// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Markdown (and CommonMark-shaped plain text) scanner.
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
use pulldown_cmark::{Event, Options, Parser, Tag};
use skribisto_model::scene_break::{self, SceneBreakTier};

use crate::block::{SourceBlock, SourceDocument};
use crate::diagnostics::ImportDiagnostic;
use crate::front_matter;
use crate::scanner::SourceScanner;
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
    // The half-open byte range of the prose accumulated since the last boundary.
    let mut prose: Option<(usize, usize)> = None;
    let mut depth = 0usize;
    let mut pending_heading: Option<(u8, usize, usize)> = None;
    let mut heading_text = String::new();
    let mut html_blocks = 0usize;
    let mut nested_rules = 0usize;
    let mut images = Vec::new();

    let flush_prose = |prose: &mut Option<(usize, usize)>, blocks: &mut Vec<SourceBlock>| {
        if let Some((start, end)) = prose.take() {
            let raw = &body[start..end];
            if !raw.trim().is_empty() {
                // Both answers from one parse: the Djot that gets stored, and the
                // plain text an annotation would be measured against. Markdown
                // carries no annotations, but a block must describe itself the
                // same way whichever scanner made it.
                let (djot, text) = skrib_format::markdown_to_djot_and_text(raw)?;
                if !djot.trim().is_empty() {
                    blocks.push(SourceBlock::Prose { djot, text });
                }
            }
        }
        Ok::<(), anyhow::Error>(())
    };

    for (event, range) in Parser::new_ext(body, options()).into_offset_iter() {
        match &event {
            Event::Start(tag) => {
                if depth == 0 {
                    match classify(&body[range.clone()], tag) {
                        Boundary::SceneBreak(tier) => {
                            flush_prose(&mut prose, &mut blocks)?;
                            blocks.push(SourceBlock::SceneBreak { tier });
                        }
                        Boundary::Heading { level } => {
                            flush_prose(&mut prose, &mut blocks)?;
                            heading_text.clear();
                            pending_heading = Some((level, range.start, range.end));
                        }
                        Boundary::Prose => extend(&mut prose, range.start, range.end),
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
                flush_prose(&mut prose, &mut blocks)?;
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
                extend(&mut prose, range.start, range.end);
            }
            _ => {}
        }

        if let Event::Start(Tag::Image { dest_url, .. }) = &event {
            images.push(dest_url.to_string());
        }
    }
    flush_prose(&mut prose, &mut blocks)?;

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

fn extend(prose: &mut Option<(usize, usize)>, start: usize, end: usize) {
    match prose {
        Some((_, e)) => *e = (*e).max(end),
        None => *prose = Some((start, end)),
    }
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
}
