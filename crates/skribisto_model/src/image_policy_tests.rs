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

/// Every test here counts through the **process-wide** cache, so each takes the
/// shared guard: `counting`'s own test asserts on the whole cache's heap, and an
/// entry landing there mid-assertion fails it for a reason that has nothing to do
/// with either test.
fn exclusive() -> std::sync::MutexGuard<'static, ()> {
    crate::counting::GLOBAL_CACHE_TESTS
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

/// The prose either side of an image, with a wordy description in the middle.
const WITH_IMAGE: &str = "The lighthouse stood alone. ![a tall white lighthouse against a grey winter sky](assets/a.png) \
     The keeper had gone.";

/// The same sentence without the image at all.
const WITHOUT_IMAGE: &str = "The lighthouse stood alone.  The keeper had gone.";

#[test]
fn an_image_adds_no_words_to_the_manuscript() {
    let _exclusive = exclusive();
    let with = count(WITH_IMAGE, CountMethod::UnicodeWords);
    let without = count(WITHOUT_IMAGE, CountMethod::UnicodeWords);
    assert_eq!(
        with.words, without.words,
        "inserting a picture must not move the writer's word count"
    );
}

#[test]
fn an_image_adds_no_characters_either() {
    let _exclusive = exclusive();
    // Pace targets and progress snapshots track characters as well as words.
    let with = count(WITH_IMAGE, CountMethod::UnicodeWords);
    let without = count(WITHOUT_IMAGE, CountMethod::UnicodeWords);
    assert_eq!(with.chars_with_spaces, without.chars_with_spaces);
    assert_eq!(with.chars_without_spaces, without.chars_without_spaces);
}

#[test]
fn a_long_description_does_not_inflate_the_count() {
    let _exclusive = exclusive();
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

// ── The same contract, for the other inline object ───────────────────────────
//
// A footnote reference is `U+FFFC` in the addressable view exactly as an image
// is, and reaches the measurement layer through the same two primitives. It is
// pinned here rather than in a file of its own because the sentinel is *shared*:
// a change that puts an image back into the word count puts a footnote there in
// the same line of code, and the next person needs to see both from one place.

/// Prose either side of a footnote reference.
const WITH_NOTE: &str = "The lighthouse stood alone.[^fn1] The keeper had gone.";
/// The same sentence with no note in it.
const WITHOUT_NOTE: &str = "The lighthouse stood alone. The keeper had gone.";

#[test]
fn a_footnote_reference_adds_no_words_or_characters() {
    let _exclusive = exclusive();
    let with = count(WITH_NOTE, CountMethod::UnicodeWords);
    let without = count(WITHOUT_NOTE, CountMethod::UnicodeWords);
    assert_eq!(
        with.words, without.words,
        "annotating a sentence must not move the writer's word count"
    );
    assert_eq!(
        with.chars_with_spaces, without.chars_with_spaces,
        "nor their character count — a marker is not a character they typed"
    );
    assert_eq!(with.chars_without_spaces, without.chars_without_spaces);
}

/// The note's **body** is not in the prose at all: it lives on its own entity,
/// and the reference in the text names it. So a long note cannot inflate the
/// scene it annotates — which is what keeps a well-annotated chapter from
/// reading as a longer one.
#[test]
fn a_notes_body_is_not_counted_with_the_scene() {
    let _exclusive = exclusive();
    let terse = count("a[^fn1] b", CountMethod::UnicodeWords);
    let same_with_a_longer_label = count("a[^fn117] b", CountMethod::UnicodeWords);
    assert_eq!(terse.words, same_with_a_longer_label.words);
    assert_eq!(
        terse.chars_with_spaces, same_with_a_longer_label.chars_with_spaces,
        "the label is machinery, and its length is nobody's business but the format's"
    );
}
