// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What an inline image is worth to the measurement layer: nothing.
//!
//! Word counts, pace targets, repetition analysis and the search corpus all
//! read prose through the same two primitives — `count_djot` and
//! `djot_to_plain_text`. An image's alt text is a description written *for* a
//! reader who cannot see the picture; it is not manuscript prose. If it leaked
//! into either primitive a writer's daily word count would jump when they
//! inserted a photograph, their repetition analysis would flag words they never
//! wrote, and a search would match text that appears nowhere on the page.
//!
//! The parser is what enforces this — it buffers alt text into the image rather
//! than emitting it as body text — so these tests are pinning a contract that
//! lives one crate away, in exactly the place that would silently break it.

use crate::counting::{CountMethod, cached_count as count};

/// The prose either side of an image, with a wordy description in the middle.
const WITH_IMAGE: &str = "The lighthouse stood alone. ![a tall white lighthouse against a grey winter sky](assets/a.png) \
     The keeper had gone.";

/// The same sentence without the image at all.
const WITHOUT_IMAGE: &str = "The lighthouse stood alone.  The keeper had gone.";

#[test]
fn an_image_adds_no_words_to_the_manuscript() {
    let with = count(WITH_IMAGE, CountMethod::UnicodeWords);
    let without = count(WITHOUT_IMAGE, CountMethod::UnicodeWords);
    assert_eq!(
        with.words, without.words,
        "inserting a picture must not move the writer's word count"
    );
}

#[test]
fn an_image_adds_no_characters_either() {
    // Pace targets and progress snapshots track characters as well as words.
    let with = count(WITH_IMAGE, CountMethod::UnicodeWords);
    let without = count(WITHOUT_IMAGE, CountMethod::UnicodeWords);
    assert_eq!(with.chars_with_spaces, without.chars_with_spaces);
    assert_eq!(with.chars_without_spaces, without.chars_without_spaces);
}

#[test]
fn a_long_description_does_not_inflate_the_count() {
    // The failure mode is proportional to how carefully the writer described
    // the image, which would punish exactly the accessible-authoring behaviour
    // the alt field exists to encourage.
    let terse = count("a ![x](assets/a.png) b", CountMethod::UnicodeWords);
    let verbose = count(
        "a ![a very long and thorough description of what can be seen here](assets/a.png) b",
        CountMethod::UnicodeWords,
    );
    assert_eq!(terse.words, verbose.words);
}

#[test]
fn the_search_corpus_does_not_contain_alt_text() {
    // `djot_to_plain_text` is what `search_management::corpus_cache` indexes,
    // so anything it returns is findable. A writer searching for "lighthouse"
    // should not match a picture's description.
    let plain = text_document::djot_to_plain_text(WITH_IMAGE, &Default::default());
    assert!(
        !plain.contains("grey winter sky"),
        "alt text is searchable prose: {plain:?}"
    );
    assert!(plain.contains("The lighthouse stood alone"));
    assert!(plain.contains("The keeper had gone"));
}

#[test]
fn analysis_tokenisation_never_sees_the_description() {
    // Repetition, echoes and lexical diversity all tokenise this string. A word
    // used only in an image's description must not be flagged as repeated prose.
    let plain = text_document::djot_to_plain_text(WITH_IMAGE, &Default::default());
    let tokens = crate::analysis::tokens::tokenize(&plain);
    assert!(
        !tokens.iter().any(|t| t.key.as_ref() == "winter"),
        "a word only in the alt text reached the analyser"
    );
    assert!(tokens.iter().any(|t| t.key.as_ref() == "keeper"));
}
