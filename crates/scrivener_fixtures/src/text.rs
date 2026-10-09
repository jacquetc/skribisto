// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Invented English prose for the fixtures.
//!
//! Written for these tests and owned by the project: no novel, template,
//! tutorial or manual text is ever copied into a fixture. A line of it is picked
//! by a seed, so every fixture reads the same on every run.

/// Sentences a lighthouse novel might hold.
const SENTENCES: &[&str] = &[
    "The keeper counted the ships before the tide turned.",
    "Mara climbed the iron stairs with the lamp oil in both hands.",
    "Nobody in the harbour remembered who had painted the door blue.",
    "Fog came in from the west and stayed for three days.",
    "He wrote the weather in the logbook, then crossed it out.",
    "The ferry was late again, and the letters with it.",
    "At night the beam swept the water like a slow broom.",
    "She kept the key to the lantern room on a string.",
    "Gulls argued on the roof until the rain silenced them.",
    "The inspector arrived on Tuesday with a list of complaints.",
    "Salt had eaten through the railing on the north side.",
    "They agreed to meet at the chapel when the bells stopped.",
];

/// One sentence, picked by `seed`.
pub fn sentence(seed: usize) -> &'static str {
    SENTENCES[seed % SENTENCES.len()]
}

/// `count` sentences starting at `seed`, joined into one paragraph.
pub fn paragraph(seed: usize, count: usize) -> String {
    (0..count)
        .map(|i| sentence(seed + i))
        .collect::<Vec<_>>()
        .join(" ")
}

/// `paragraphs` paragraphs of `sentences` sentences each.
pub fn paragraphs(seed: usize, paragraphs: usize, sentences: usize) -> Vec<String> {
    (0..paragraphs)
        .map(|i| paragraph(seed + i * sentences, sentences))
        .collect()
}
