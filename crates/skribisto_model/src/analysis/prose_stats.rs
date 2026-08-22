// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Per-scene shape: sentence lengths, paragraph lengths, punctuation density, dialogue share.
//!
//! Every figure here is **descriptive**. There is no correct sentence length and no correct
//! dialogue ratio, so nothing in this module carries a threshold, a grade or a verdict — it
//! reports what is there and leaves the comparison to the caller, which compares a scene
//! against the rest of *this* manuscript and never against an external norm.
//!
//! ## Dialogue is a caller-supplied convention
//!
//! Which glyphs open a line of dialogue is a per-language fact the app already curates once,
//! in the typography rules that drive smart punctuation. Rather than grow a second, silently
//! diverging copy, [`DialogueMarkers`] is a parameter: the caller reads the writer's language
//! from the existing per-item/per-Work chain and hands the answer down. A language with no
//! curated convention gets [`DialogueMarkers::none`] and an honest
//! [`ProseStats::dialogue`] of `None` — not a zero, which would read as "no dialogue here"
//! rather than "not measurable in this language".

use common::entities::QuoteStyle;
use text_document::sentences;

use super::stats;
use super::tokens::tokenize;

/// How one language marks speech. Built by the caller from its typography table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DialogueMarkers {
    /// The glyph that opens a quotation, if the language quotes.
    pub open_quote: Option<char>,
    /// The glyph that closes it. Equal to `open_quote` for symmetric conventions.
    pub close_quote: Option<char>,
    /// The dash that opens a line of dialogue, where the language uses one.
    pub dash: Option<char>,
}

impl DialogueMarkers {
    /// No convention known for this language — dialogue is not measurable, and saying so is
    /// the point.
    pub fn none() -> Self {
        Self::default()
    }

    /// Whether anything here lets us recognise speech at all.
    pub fn is_measurable(&self) -> bool {
        self.open_quote.is_some() || self.dash.is_some()
    }
}

/// How this language marks speech, under a project's house style.
///
/// Resolves through [`crate::typography::quotes_for`] — **the same function**
/// [`TypographyEngine::new`](crate::typography::engine::TypographyEngine::new) uses to decide which
/// glyphs to *insert* as the writer types. Sharing the resolver, rather than reading the
/// locale table twice, is what keeps recognising dialogue and producing it in agreement.
///
/// `quote_style` is the project's own override. It is a required argument rather than an
/// `Option` with a default precisely because the default is what went wrong before: this
/// function used to read the locale row directly and never saw the override, so a project
/// set to guillemets under an `en-US` locale had `« »` inserted and `" "` looked for, and
/// every paragraph measured as zero spoken words. Passing [`QuoteStyle::LocaleDefault`] is
/// still available and still correct — it just has to be *said*.
///
/// A tag with no curated row resolves to the default English-ish ruleset, which is right for
/// *inserting* punctuation (better than nothing) and wrong for *measuring* it (a guess is not
/// a measurement) — so that case returns [`DialogueMarkers::none`] and everything downstream
/// reports dialogue as not measurable rather than as zero.
pub fn markers_for(tag: &str, quote_style: QuoteStyle) -> DialogueMarkers {
    let row = crate::typography::ruleset_for(tag);

    // Unmeasurable means "we would be guessing", and an explicit house style is
    // not a guess. A locale with no curated row falls back to a default ruleset
    // that is fine for *inserting* punctuation and wrong for *measuring* it — but
    // that reasoning only applies when the quotes came from the locale. A writer
    // in an uncurated language who set the project to guillemets gets guillemets
    // inserted, so recognising them is exactly as well-founded as it is for a
    // curated language, and reporting "not measurable" there would hide dialogue
    // the editor itself is producing.
    if row.tag.is_empty() && quote_style == QuoteStyle::LocaleDefault {
        return DialogueMarkers::none();
    }

    let quotes = crate::typography::quotes_for(tag, quote_style);
    DialogueMarkers {
        open_quote: Some(quotes.open()),
        close_quote: Some(quotes.close()),
        // No override exists for the dialogue dash, so an uncurated row
        // legitimately contributes none.
        dash: row.dialogue_dash,
    }
}

/// One paragraph's contribution, in the order the paragraphs appear.
///
/// The per-paragraph half of what [`measure`] already computed and used to discard: the
/// aggregate `dialogue` share is the sum of `spoken` over the sum of `words`. Kept because a
/// reader hears dialogue as a *shape* down the page — a ladder of speech against a wall of
/// narration — and a single ratio for the whole scene cannot show that.
///
/// `spoken` is `0` for a language with no curated convention, which the caller must read
/// alongside [`ProseStats::dialogue`] being `None` rather than as "no dialogue here".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ParagraphStats {
    /// Words in the paragraph. Never `0` — empty paragraphs are not reported.
    pub words: usize,
    /// How many of those words sit inside speech.
    pub spoken: usize,
}

/// The measured shape of one scene's prose.
#[derive(Debug, Clone, PartialEq)]
pub struct ProseStats {
    pub words: usize,
    /// Word counts of each sentence, in order.
    pub sentence_words: Vec<usize>,
    /// Word counts of each paragraph, in order.
    pub paragraph_words: Vec<usize>,
    /// Per-paragraph words and spoken words, in the same order as
    /// [`paragraph_words`](Self::paragraph_words) and index-aligned with it.
    pub paragraphs: Vec<ParagraphStats>,
    /// Sentence-ending and clause-separating marks per 1,000 words.
    pub punctuation_per_1k: f64,
    /// Share of words inside speech, `0.0..=1.0` — `None` when the language has no curated
    /// dialogue convention, which is different from "no dialogue".
    pub dialogue: Option<f64>,
}

impl ProseStats {
    pub fn mean_sentence_words(&self) -> Option<f64> {
        stats::mean(&as_f64(&self.sentence_words))
    }

    /// Population standard deviation of sentence length — the "rhythm" figure. A manuscript
    /// whose sentences are all the same length reads flat, and this is what shows it.
    pub fn sentence_words_stddev(&self) -> Option<f64> {
        stats::population_stddev(&as_f64(&self.sentence_words))
    }

    pub fn mean_paragraph_words(&self) -> Option<f64> {
        stats::mean(&as_f64(&self.paragraph_words))
    }
}

fn as_f64(xs: &[usize]) -> Vec<f64> {
    xs.iter().map(|&n| n as f64).collect()
}

/// Marks that end a sentence or separate a clause. Deliberately narrow: this measures how
/// heavily punctuated the prose is, so apostrophes and hyphens (which are *inside* words)
/// must not count, or every text scores by how many contractions it happens to use.
const PUNCTUATION: &[char] = &[
    '.', ',', ';', ':', '!', '?', '—', '–', '…', '(', ')', '«', '»', '"', '\u{201C}', '\u{201D}',
];

/// Measure one scene's plain text.
///
/// `locale` is the BCP-47 tag the sentence splitter tailors to (`None` falls back to plain
/// UAX #29, which is a real fallback and not a stub — see `text_document`'s sentence module).
pub fn measure(text: &str, locale: Option<&str>, markers: DialogueMarkers) -> ProseStats {
    // One pass. This used to be two: `paragraph_words` tokenized every paragraph, and then
    // `dialogue_share` walked the same paragraphs and tokenized them all over again to reach
    // the same counts. Both halves are computed here, once, and the aggregate share is
    // derived from them rather than measured separately — so the total and the per-paragraph
    // breakdown are arithmetically the same number and cannot disagree.
    let measurable = markers.is_measurable();
    let per_paragraph: Vec<ParagraphStats> = paragraphs(text)
        .filter_map(|para| {
            let words = tokenize(para).len();
            if words == 0 {
                return None;
            }
            let spoken = if !measurable {
                0
            } else if markers.dash.is_some_and(|d| para.starts_with(d)) {
                // The dash convention is per *paragraph*: there is no closing dash to look
                // for, so an opening one makes the whole paragraph speech.
                words
            } else {
                quoted_words(para, markers)
            };
            Some(ParagraphStats { words, spoken })
        })
        .collect();

    let paragraph_words: Vec<usize> = per_paragraph.iter().map(|p| p.words).collect();

    // Sentence splitting is block-scoped by design, so it is applied per paragraph rather
    // than to the whole scene — handing it the joined text would let one paragraph's last
    // sentence run into the next paragraph's first.
    let mut sentence_words = Vec::new();
    for para in paragraphs(text) {
        for s in sentences(para, locale) {
            let n = tokenize(s.text).len();
            if n > 0 {
                sentence_words.push(n);
            }
        }
    }

    let words: usize = paragraph_words.iter().sum();
    let marks = text.chars().filter(|c| PUNCTUATION.contains(c)).count();
    let punctuation_per_1k = if words == 0 {
        0.0
    } else {
        marks as f64 * 1000.0 / words as f64
    };

    let spoken: usize = per_paragraph.iter().map(|p| p.spoken).sum();

    ProseStats {
        words,
        sentence_words,
        paragraph_words,
        paragraphs: per_paragraph,
        punctuation_per_1k,
        // Both conditions matter. No convention means "not measurable in this language";
        // no words means "nothing to measure" — and a `Some(0.0)` for either would draw a
        // real 0% bar, which this module's own docs say must never stand in for absence.
        dialogue: (measurable && words > 0).then(|| spoken as f64 / words as f64),
    }
}

/// Paragraphs of plain text: blank-line separated, blanks dropped.
///
/// `djot_to_plain_text` emits one `\n` per block, so a "blank line" is really "an empty
/// line between two non-empty ones" — splitting on `\n\n` alone would return the whole
/// scene as one paragraph.
fn paragraphs(text: &str) -> impl Iterator<Item = &str> {
    text.lines().map(str::trim).filter(|l| !l.is_empty())
}

/// Words inside quotation marks in one paragraph.
///
/// Tracks open/close as a toggle rather than as a nesting depth: prose nests quotations at
/// most one level deep and a toggle degrades gracefully on the unbalanced quotes that real
/// drafts contain, where a depth counter would go negative and stay wrong for the rest of
/// the scene.
fn quoted_words(para: &str, markers: DialogueMarkers) -> usize {
    let (Some(open), Some(close)) = (markers.open_quote, markers.close_quote) else {
        return 0;
    };

    let mut inside = false;
    let mut spans: Vec<(usize, usize)> = Vec::new();
    let mut span_start = 0usize;

    for (at, ch) in para.char_indices() {
        if open == close {
            // A symmetric mark cannot say which end it is; alternate.
            if ch == open {
                if inside {
                    spans.push((span_start, at));
                    inside = false;
                } else {
                    inside = true;
                    span_start = at + ch.len_utf8();
                }
            }
        } else if ch == open && !inside {
            inside = true;
            span_start = at + ch.len_utf8();
        } else if ch == close && inside {
            spans.push((span_start, at));
            inside = false;
        }
    }
    // An unclosed quotation runs to the end of the paragraph, which is what a writer
    // mid-draft almost always means.
    if inside {
        spans.push((span_start, para.len()));
    }

    spans
        .iter()
        .map(|&(s, e)| tokenize(&para[s..e]).len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EN: DialogueMarkers = DialogueMarkers {
        open_quote: Some('\u{201C}'),
        close_quote: Some('\u{201D}'),
        dash: None,
    };
    const FR: DialogueMarkers = DialogueMarkers {
        open_quote: Some('«'),
        close_quote: Some('»'),
        dash: Some('—'),
    };

    #[test]
    fn paragraphs_are_blank_line_separated() {
        let s = measure("One two three.\n\nFour five.", Some("en"), EN);
        assert_eq!(s.paragraph_words, [3, 2]);
        assert_eq!(s.words, 5);
    }

    #[test]
    fn sentences_are_counted_per_paragraph() {
        let s = measure("He ran. She walked.\n\nThen night fell.", Some("en"), EN);
        assert_eq!(s.sentence_words, [2, 2, 3]);
    }

    /// A sentence must not run across a paragraph break even when the first paragraph ends
    /// without a full stop — a scene-break line or a line of verse would otherwise swallow
    /// the paragraph after it.
    #[test]
    fn a_paragraph_break_always_ends_a_sentence() {
        let s = measure("No full stop here\n\nAnd a new thought.", Some("en"), EN);
        assert_eq!(s.sentence_words, [4, 4]);
    }

    #[test]
    fn mean_and_stddev_describe_the_rhythm() {
        let s = measure("A b.\n\nC d e f g h.", Some("en"), EN);
        assert_eq!(s.sentence_words, [2, 6]);
        assert_eq!(s.mean_sentence_words(), Some(4.0));
        assert_eq!(s.sentence_words_stddev(), Some(2.0));
    }

    #[test]
    fn one_sentence_has_a_mean_but_no_deviation() {
        let s = measure("Only the one sentence.", Some("en"), EN);
        assert_eq!(s.mean_sentence_words(), Some(4.0));
        assert_eq!(
            s.sentence_words_stddev(),
            None,
            "deviation of one sample is not a number"
        );
    }

    #[test]
    fn empty_prose_measures_as_empty_not_as_zero_rhythm() {
        let s = measure("   \n\n  ", Some("en"), EN);
        assert_eq!(s.words, 0);
        assert_eq!(s.mean_sentence_words(), None);
        assert_eq!(s.punctuation_per_1k, 0.0);
    }

    /// A scene with no prose (a freshly created one) must report no dialogue share at all,
    /// not `Some(0.0)` — the latter draws a real 0% bar and reads as a finding.
    #[test]
    fn prose_with_no_words_has_no_dialogue_share_to_report() {
        assert_eq!(measure("   \n\n  ", Some("en"), EN).dialogue, None);
        assert_eq!(measure("", Some("en"), EN).dialogue, None);
        // ...while a scene that genuinely contains no speech still reports zero, because
        // that IS a measurement.
        assert_eq!(
            measure("The room was cold.", Some("en"), EN).dialogue,
            Some(0.0)
        );
    }

    #[test]
    fn punctuation_density_is_per_thousand_words() {
        // 4 words, 2 marks → 500 per 1k.
        let s = measure("Yes, he left.", Some("en"), EN);
        assert_eq!(s.words, 3);
        assert!((s.punctuation_per_1k - 2000.0 / 3.0).abs() < 1e-9);
    }

    /// An apostrophe is inside a word, not punctuation between clauses; counting it would
    /// score a contraction-heavy voice as heavily punctuated.
    #[test]
    fn apostrophes_are_not_punctuation() {
        let plain = measure("She could not go", Some("en"), EN);
        let contracted = measure("She couldn't go", Some("en"), EN);
        assert_eq!(contracted.punctuation_per_1k, 0.0);
        assert_eq!(plain.punctuation_per_1k, 0.0);
    }

    // ── dialogue ──

    #[test]
    fn quoted_speech_counts_as_dialogue() {
        let s = measure("\u{201C}Come here,\u{201D} she said.", Some("en"), EN);
        // 2 spoken of 4 total.
        assert_eq!(s.dialogue, Some(0.5));
    }

    #[test]
    fn narration_alone_is_no_dialogue() {
        let s = measure("The room was cold and empty.", Some("en"), EN);
        assert_eq!(s.dialogue, Some(0.0));
    }

    #[test]
    fn a_french_dialogue_dash_makes_the_paragraph_speech() {
        let s = measure("— Tu viens ?\n\nIl ne répondit pas.", Some("fr"), FR);
        // "tu viens" spoken of "tu viens il ne répondit pas" — the dash and the question
        // mark are punctuation, not words.
        assert_eq!(s.words, 6);
        assert_eq!(s.dialogue, Some(2.0 / 6.0));
    }

    #[test]
    fn french_guillemets_are_recognised() {
        let s = measure("« Vraiment ? » demanda-t-elle.", Some("fr"), FR);
        assert!(s.dialogue.is_some_and(|d| d > 0.0));
    }

    /// An unbalanced quote is ordinary in a draft. It must not poison the rest of the scene,
    /// and it must not be silently ignored either.
    #[test]
    fn an_unclosed_quotation_runs_to_the_end_of_its_paragraph_only() {
        let s = measure(
            "\u{201C}Wait, he began but stopped\n\nThe room stayed silent and cold.",
            Some("en"),
            EN,
        );
        let d = s.dialogue.unwrap();
        assert!(
            d > 0.0 && d < 1.0,
            "one runaway quote must not mark the whole scene: {d}"
        );
    }

    /// The honest answer for a language with no curated convention. A `0.0` here would read
    /// as "this scene has no dialogue", which is a different and false claim.
    #[test]
    fn an_unsupported_language_reports_no_measurement_rather_than_zero() {
        let s = measure(
            "\u{201C}Whatever this is,\u{201D} said someone.",
            None,
            DialogueMarkers::none(),
        );
        assert_eq!(s.dialogue, None);
        assert!(s.words > 0, "the rest of the measurement still works");
    }

    // ── the language table ──

    #[test]
    fn every_curated_language_is_measurable() {
        for tag in [
            "en", "fr", "de", "de-CH", "es", "ca", "it", "pt", "pt-BR", "nl", "pl", "ru", "sv",
            "tr", "ar",
        ] {
            assert!(
                markers_for(tag, QuoteStyle::LocaleDefault).is_measurable(),
                "{tag} must be measurable"
            );
        }
    }

    /// An explicit house style is not a guess, so it must be measurable even
    /// where the locale itself carries no curated row.
    ///
    /// The editor inserts the override's glyphs for any language; reporting
    /// "not measurable" for those same glyphs would hide dialogue the app is
    /// itself producing.
    #[test]
    fn an_override_is_measurable_even_for_an_uncurated_language() {
        for tag in ["ja", "fi", "cs", "he", "zxx"] {
            assert!(
                !markers_for(tag, QuoteStyle::LocaleDefault).is_measurable(),
                "{tag} has no curated row, so the locale default is a guess"
            );
            for style in [
                QuoteStyle::CurlyDouble,
                QuoteStyle::Guillemets,
                QuoteStyle::LowHigh,
            ] {
                let markers = markers_for(tag, style.clone());
                assert!(
                    markers.is_measurable(),
                    "{tag} / {style:?}: the editor inserts these glyphs, so they must be recognised"
                );
                let engine = crate::typography::engine::TypographyEngine::new(
                    tag,
                    crate::typography::engine::SmartPunctuationFlags {
                        quote_style: style.clone(),
                        ..Default::default()
                    },
                );
                assert_eq!(markers.open_quote, Some(engine.quotes().open()));
                assert_eq!(markers.close_quote, Some(engine.quotes().close()));
            }
        }
    }

    /// The per-paragraph breakdown is the point of the single-pass rewrite, and
    /// it must be arithmetically the same measurement as the aggregate — not a
    /// second, separately-derived one that can drift.
    #[test]
    fn the_paragraph_breakdown_sums_to_the_aggregate() {
        let text = "\u{201c}Come inside,\u{201d} she said.\n\
                    The door stood open on a cold hallway.\n\
                    \u{201c}I would rather not.\u{201d}";
        let stats = measure(
            text,
            Some("en"),
            markers_for("en", QuoteStyle::LocaleDefault),
        );

        assert_eq!(
            stats.paragraphs.len(),
            3,
            "one entry per non-empty paragraph"
        );
        assert_eq!(
            stats.paragraphs.iter().map(|p| p.words).collect::<Vec<_>>(),
            stats.paragraph_words,
            "the breakdown and paragraph_words must agree, index for index"
        );
        assert_eq!(
            stats.paragraphs.iter().map(|p| p.words).sum::<usize>(),
            stats.words
        );

        let spoken: usize = stats.paragraphs.iter().map(|p| p.spoken).sum();
        let expected = spoken as f64 / stats.words as f64;
        assert!(
            (stats.dialogue.unwrap() - expected).abs() < f64::EPSILON,
            "the aggregate share must be the breakdown's own sum"
        );

        // The middle paragraph is narration; the outer two are speech.
        assert_eq!(stats.paragraphs[1].spoken, 0);
        assert!(stats.paragraphs[0].spoken > 0);
        assert!(stats.paragraphs[2].spoken > 0);
    }

    /// **The bug this module shipped, pinned.**
    ///
    /// `markers_for` used to read the locale row directly and never saw the project's house
    /// quote style, while the editor's `TypographyEngine` did. A project set to guillemets
    /// under an `en-US` locale therefore had `« »` inserted and `" "` looked for, so every
    /// paragraph measured as zero spoken words while reading as dialogue on the page.
    /// Nothing errored; the number was quietly wrong.
    ///
    /// Crossing every curated locale with every override is what makes that unrepresentable,
    /// and it fails loudly if the two ever resolve separately again.
    #[test]
    fn measurement_and_insertion_agree_on_every_locale_and_override() {
        use crate::typography::engine::{SmartPunctuationFlags, TypographyEngine};

        for tag in [
            "en", "fr", "de", "de-CH", "es", "ca", "it", "pt", "pt-BR", "nl", "pl", "ru", "sv",
            "tr", "ar",
        ] {
            for style in [
                QuoteStyle::LocaleDefault,
                QuoteStyle::CurlyDouble,
                QuoteStyle::Guillemets,
                QuoteStyle::LowHigh,
            ] {
                let flags = SmartPunctuationFlags {
                    quote_style: style.clone(),
                    ..SmartPunctuationFlags::default()
                };
                let inserts = TypographyEngine::new(tag, flags).quotes();
                let recognises = markers_for(tag, style.clone());

                assert_eq!(
                    recognises.open_quote,
                    Some(inserts.open()),
                    "{tag} / {style:?}: the editor inserts {:?} to open a quotation but \
                     dialogue measurement looks for {:?}",
                    inserts.open(),
                    recognises.open_quote,
                );
                assert_eq!(
                    recognises.close_quote,
                    Some(inserts.close()),
                    "{tag} / {style:?}: insertion and measurement disagree on the closing glyph",
                );
            }
        }
    }

    /// A house style that is not the locale's own must actually change what counts as speech.
    ///
    /// The agreement test above would still pass if `quotes_for` ignored the override in
    /// *both* halves, so this pins the other half: an English scene written in guillemets
    /// measures as dialogue only when the project says guillemets.
    #[test]
    fn a_house_style_override_changes_what_measures_as_speech() {
        let text = "\u{ab}Come inside,\u{bb} she said.";

        let under_default = measure(
            text,
            Some("en"),
            markers_for("en", QuoteStyle::LocaleDefault),
        );
        let under_guillemets = measure(text, Some("en"), markers_for("en", QuoteStyle::Guillemets));

        assert_eq!(
            under_default.dialogue,
            Some(0.0),
            "curly quotes are what en expects, so guillemetted speech reads as narration"
        );
        assert!(
            under_guillemets.dialogue.is_some_and(|d| d > 0.0),
            "with the project set to guillemets the same words must measure as speech, got {:?}",
            under_guillemets.dialogue
        );
    }

    /// `language::primary` hands over the writer's tag as written (e.g. `en-US`), so a
    /// regional tag must resolve through its base language rather than report unmeasurable.
    #[test]
    fn a_regional_tag_resolves_through_its_language() {
        for tag in [
            "en-US", "en_US", "EN-us", " en-GB ", "fr-CA", "fr_FR", "pt-PT",
        ] {
            assert!(
                markers_for(tag, QuoteStyle::LocaleDefault).is_measurable(),
                "{tag} must resolve to its language's convention"
            );
        }
        assert_eq!(
            markers_for("en-US", QuoteStyle::LocaleDefault),
            markers_for("en", QuoteStyle::LocaleDefault)
        );
        assert_eq!(
            markers_for("fr-CA", QuoteStyle::LocaleDefault),
            markers_for("fr", QuoteStyle::LocaleDefault)
        );
    }

    /// A region with its own row must win over its base language, in both directions.
    #[test]
    fn a_region_with_its_own_row_beats_its_base_language() {
        assert_eq!(
            markers_for("de-CH", QuoteStyle::LocaleDefault),
            markers_for("de-ch", QuoteStyle::LocaleDefault)
        );
        assert_ne!(
            markers_for("de-CH", QuoteStyle::LocaleDefault),
            markers_for("de", QuoteStyle::LocaleDefault)
        );
        assert_ne!(
            markers_for("pt-BR", QuoteStyle::LocaleDefault),
            markers_for("pt", QuoteStyle::LocaleDefault)
        );
        // ...while a region with no row of its own inherits.
        assert_eq!(
            markers_for("de-AT", QuoteStyle::LocaleDefault),
            markers_for("de", QuoteStyle::LocaleDefault)
        );
    }

    /// The mistake the Polish row exists to prevent: it opens low like German but closes
    /// with a *right* double, and conflating them would mis-measure every Polish scene.
    #[test]
    fn polish_is_not_german() {
        assert_ne!(
            markers_for("pl", QuoteStyle::LocaleDefault),
            markers_for("de", QuoteStyle::LocaleDefault)
        );
        assert_eq!(
            markers_for("pl", QuoteStyle::LocaleDefault).close_quote,
            Some('\u{201D}')
        );
        assert_eq!(
            markers_for("de", QuoteStyle::LocaleDefault).close_quote,
            Some('\u{201C}')
        );
    }

    #[test]
    fn a_region_that_diverges_from_its_base_language_gets_its_own_row() {
        assert_ne!(
            markers_for("de", QuoteStyle::LocaleDefault),
            markers_for("de-CH", QuoteStyle::LocaleDefault)
        );
        assert_ne!(
            markers_for("pt", QuoteStyle::LocaleDefault),
            markers_for("pt-BR", QuoteStyle::LocaleDefault)
        );
    }

    #[test]
    fn an_uncurated_language_is_honestly_unmeasurable() {
        for tag in ["ja", "zh", "he", "fi", "cs", ""] {
            let m = markers_for(tag, QuoteStyle::LocaleDefault);
            assert!(
                !m.is_measurable(),
                "{tag} has no curated convention and must say so"
            );
        }
    }

    #[test]
    fn symmetric_quotes_alternate_open_and_close() {
        let straight = DialogueMarkers {
            open_quote: Some('"'),
            close_quote: Some('"'),
            dash: None,
        };
        let s = measure("\"Come here,\" she said.", Some("en"), straight);
        assert_eq!(s.dialogue, Some(0.5));
    }
}
