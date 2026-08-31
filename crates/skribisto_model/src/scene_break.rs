// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Scene-break markers — the vocabulary an author types into their prose, and
//! the recogniser every consumer shares.
//!
//! A scene break in Skribisto is an **explicit authorial mark**, never inferred
//! from binder structure: the binder tree is organisational, so splitting prose
//! into two Scene items is a chunking decision, not a narrative signal. Two
//! scenes may be joined by a glue paragraph and must flow continuously. The
//! author places a marker paragraph where a break belongs — including *inside* a
//! scene, which is why the mark lives in the prose rather than on an item.
//!
//! The vocabulary here is **fixed and preset-independent**. An export preset
//! decides how a break *renders* (see `skribisto_compiler`'s `SceneBreak`), so
//! changing preset never un-recognises a mark the author already typed.
//!
//! ## Two entry points, because consumers see different bytes
//!
//! Prose is stored as Djot, and Djot escapes punctuation: a typed `* * *`
//! persists in `Content.data` as `\* \* \*`. But some callers hold text that has
//! already been through the parser (a live editor buffer's `to_plain_text()`),
//! where it reads `* * *` again. Both must recognise the same marks, so this
//! module exposes a raw-Djot entry point and a plain-text one over one shared
//! vocabulary — rather than leaving each call site to roll its own and drift.
//!
//! ## Who strips, and who deliberately does not
//!
//! **Counting strips.** A break is furniture the author placed, not words they
//! wrote, so it must not inflate the manuscript's word count — least of all the
//! persisted pace/progress history, where the error would be permanent. Every
//! raw-Djot counter funnels through `counting::cached_count`, and the live
//! status-bar counter (which holds parsed text) uses the plain entry point.
//!
//! **Search does not strip.** `search_management`'s corpus is pinned as exactly
//! the text the document itself searches, so a match found there maps to a
//! replace performed on the document. Removing blocks would shift every later
//! offset and corrupt replacements. A marker is part of the manuscript as
//! written, so search and replace see it — deliberately.

use std::borrow::Cow;

/// How strong a break the author asked for.
///
/// The two-tier distinction is not a US-only idea: Shunn codifies `#` vs
/// `# # #` for minor/major, other tools separate "soft" and "hard" scene
/// breaks, and Russian editorial practice distinguishes a silent gap from a
/// graphic separator by the same logic (visible outranks invisible).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SceneBreakTier {
    /// An ordinary scene change — a shift of time, place or viewpoint.
    Minor,
    /// A stronger division: a large time skip, or a decisive viewpoint change.
    Major,
}

/// The canonical mark inserted by the "Insert scene break" commands. Authors may
/// type any form in [`MINOR_MARKERS`] / [`MAJOR_MARKERS`], but this is what the
/// app writes so a project stays internally consistent.
pub const CANONICAL_MINOR: &str = "* * *";
/// The canonical major mark. See [`CANONICAL_MINOR`].
pub const CANONICAL_MAJOR: &str = "# # #";

/// Accepted spellings of a minor break, compared after normalisation.
///
/// Deliberately liberal: a writer coming from Markdown, another editor or a plain
/// manuscript may reach for any of these, and silently treating a near-miss as
/// ordinary prose would put a stray `***` in the finished book.
///
/// `＊` (U+FF0A FULLWIDTH ASTERISK) is what Japanese web-novel practice uses, and
/// is what the `manuscript-ja-web` export preset emits for an ordinary break — so
/// an author writing to that convention must be able to type it and be understood.
pub const MINOR_MARKERS: &[&str] = &["* * *", "***", "*", "#", "＊"];

/// Accepted spellings of a major break. `⁂` (U+2042 ASTERISM) is recognised but
/// never emitted as a default — it is absent from Times New Roman, Calibri,
/// Georgia and Kindle's Bookerly, so it risks rendering as tofu.
///
/// `. . .` and `◇` are here for the same reason `＊` is above: they are the
/// stronger mark the `manuscript-ru` and `manuscript-ja-web` presets *write*, and
/// a vocabulary that can export a mark but not recognise it tells the writer their
/// own convention is prose.
///
/// The Russian mark is a spaced *row* of dots, never the single `…`. One ellipsis
/// alone on a line is an ordinary beat of silence in fiction — and smart
/// punctuation turns a typed `...` into precisely that character — so recognising
/// it would strip a writer's silence from their word count and swap it for a break
/// glyph on export. Spacing is what makes the mark a mark, exactly as it does for
/// `* * *`.
pub const MAJOR_MARKERS: &[&str] = &["# # #", "###", "⁂", ". . .", "◇"];

/// The canonical mark for `tier` as **literal text** — what the author sees.
///
/// This is what an insert command types into the editor: the editor holds parsed
/// text, and the Djot escaping that keeps the mark from parsing as a thematic
/// break is applied by the exporter on save. Writing [`canonical_djot`] into a
/// live editor instead would show the author a stray backslash.
pub fn canonical_plain(tier: SceneBreakTier) -> &'static str {
    match tier {
        SceneBreakTier::Minor => CANONICAL_MINOR,
        SceneBreakTier::Major => CANONICAL_MAJOR,
    }
}

/// The canonical mark for `tier` as **Djot source**, escaped.
///
/// The escaping is not optional: bare `* * *` is a Djot *thematic break*, which
/// the document model cannot represent and the parser discards — an unescaped
/// marker written into prose would simply vanish on the next load. This is the
/// exact byte sequence the editor itself persists when an author types the mark,
/// so importers and insert commands should write it rather than roll their own.
pub fn canonical_djot(tier: SceneBreakTier) -> &'static str {
    match tier {
        SceneBreakTier::Minor => "\\* \\* \\*",
        SceneBreakTier::Major => "\\# # #",
    }
}

/// Undo Djot's punctuation escaping.
///
/// text-document escapes on the way out — every `\ * _ ` ~ ^ [ ] ( ) { } | <`
/// plus a leading `# > - + :` — so the persisted form of a typed `* * *` is
/// `\* \* \*`. Djot's rule is that a backslash escapes any ASCII punctuation, so
/// inverting it needs no table of its own (and cannot drift out of step with the
/// escaper's exact character set).
fn unescape_djot(s: &str) -> Cow<'_, str> {
    if !s.contains('\\') {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            // An escaped punctuation mark: keep the mark, drop the backslash.
            Some(next) if next.is_ascii_punctuation() => out.push(next),
            // A backslash before anything else is literal.
            Some(next) => {
                out.push('\\');
                out.push(next);
            }
            None => out.push('\\'),
        }
    }
    Cow::Owned(out)
}

/// Reduce a block to the form the vocabulary is written in: unescaped, trimmed,
/// and with internal *spaces* collapsed — so `*  *  *` and `* * *` are the same
/// mark, which is what an author means by them.
///
/// Returns `None` for anything spanning more than one line. A mark is a single
/// line by definition, and collapsing newlines too would silently eat a genuine
/// paragraph such as `*\n*\n*` — or a marker-looking line inside a fenced code
/// block, which arrives here as one chunk with its fences attached.
fn normalize(block: &str) -> Option<String> {
    let trimmed = block.trim();
    if trimmed.contains('\n') {
        return None;
    }
    let unescaped = unescape_djot(trimmed);
    Some(unescaped.split_whitespace().collect::<Vec<_>>().join(" "))
}

/// The tier of an already-normalised candidate, or `None` if it is ordinary prose.
fn tier_of_normalized(candidate: &str) -> Option<SceneBreakTier> {
    if candidate.is_empty() {
        return None;
    }
    if MAJOR_MARKERS.contains(&candidate) {
        return Some(SceneBreakTier::Major);
    }
    if MINOR_MARKERS.contains(&candidate) {
        return Some(SceneBreakTier::Minor);
    }
    None
}

/// Classify one **raw Djot block** — the escaped form as stored in
/// `Content.data`. Returns `None` for ordinary prose.
pub fn tier_of_djot_block(block: &str) -> Option<SceneBreakTier> {
    tier_of_normalized(&normalize(block)?)
}

/// Classify one **plain-text line** — text that has already been through the
/// Djot parser, such as a live editor buffer's `to_plain_text()`. Returns `None`
/// for ordinary prose.
pub fn tier_of_plain_line(line: &str) -> Option<SceneBreakTier> {
    // `normalize` un-escapes defensively: plain text has no backslash escapes,
    // but running the same path keeps the two entry points impossible to skew.
    tier_of_normalized(&normalize(line)?)
}

/// Remove scene-break markers from a **raw Djot** string, so they are not
/// counted as words or indexed as prose. Blocks are separated by a blank line.
///
/// Borrows unchanged when there is nothing to strip, which is the common case.
pub fn strip_markers_djot(djot: &str) -> Cow<'_, str> {
    if !might_contain_marker(djot) {
        return Cow::Borrowed(djot);
    }
    let kept: Vec<&str> = djot
        .split("\n\n")
        .filter(|block| tier_of_djot_block(block).is_none())
        .collect();
    Cow::Owned(kept.join("\n\n"))
}

/// Remove scene-break markers from **plain text**, where each block is its own
/// line. The counterpart of [`strip_markers_djot`] for callers holding parsed
/// text rather than Djot source.
pub fn strip_markers_plain(text: &str) -> Cow<'_, str> {
    if !might_contain_marker(text) {
        return Cow::Borrowed(text);
    }
    let kept: Vec<&str> = text
        .lines()
        .filter(|line| tier_of_plain_line(line).is_none())
        .collect();
    Cow::Owned(kept.join("\n"))
}

/// Cheap pre-filter: prose with no line that *could* be a marker is handled
/// verbatim, without splitting it into blocks at all. The compiler relies on this
/// to leave ordinary prose completely untouched.
///
/// A marker is a whole line by definition, so the test is line-anchored rather
/// than a bare `contains`. That matters most for `.`: a substring test would
/// report "might contain a marker" for every page of prose ever written, costing
/// the fast path exactly where it is most wanted, where a line built only of dots
/// and spaces is vanishingly rare. Anchoring also tightens the older characters —
/// a paragraph mentioning `*` no longer forces a block split either.
///
/// False positives are free (they only cost the walk the caller would have done);
/// a false negative would silently leave a marker in the finished book, so the
/// test admits any line built solely from marker characters, whitespace, and the
/// backslash Djot escapes them with.
pub fn might_contain_marker(s: &str) -> bool {
    s.lines().any(|line| {
        let trimmed = line.trim();
        !trimmed.is_empty()
            && trimmed.chars().all(|c| {
                matches!(c, '*' | '#' | '⁂' | '.' | '＊' | '◇' | '\\') || c.is_whitespace()
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_canonical_marks_are_recognised_at_their_own_tier() {
        assert_eq!(
            tier_of_plain_line(CANONICAL_MINOR),
            Some(SceneBreakTier::Minor)
        );
        assert_eq!(
            tier_of_plain_line(CANONICAL_MAJOR),
            Some(SceneBreakTier::Major)
        );
    }

    #[test]
    fn the_real_escaped_forms_from_disk_are_recognised() {
        // THE test that matters. `Content.data` never holds `* * *` — it holds
        // `\* \* \*`, because text-document escapes every asterisk on save, and
        // a leading `#` is guarded too. A recogniser written against idealised
        // raw strings would silently match nothing in a real project.
        assert_eq!(
            tier_of_djot_block("\\* \\* \\*"),
            Some(SceneBreakTier::Minor)
        );
        assert_eq!(tier_of_djot_block("\\#"), Some(SceneBreakTier::Minor));
        assert_eq!(tier_of_djot_block("\\# # #"), Some(SceneBreakTier::Major));
        assert_eq!(tier_of_djot_block("\\*\\*\\*"), Some(SceneBreakTier::Minor));
    }

    #[test]
    fn every_listed_spelling_resolves_to_its_tier() {
        for m in MINOR_MARKERS {
            assert_eq!(
                tier_of_plain_line(m),
                Some(SceneBreakTier::Minor),
                "minor marker {m:?}"
            );
        }
        for m in MAJOR_MARKERS {
            assert_eq!(
                tier_of_plain_line(m),
                Some(SceneBreakTier::Major),
                "major marker {m:?}"
            );
        }
    }

    #[test]
    fn spacing_variants_are_the_same_mark() {
        assert_eq!(tier_of_plain_line("*  *  *"), Some(SceneBreakTier::Minor));
        assert_eq!(tier_of_plain_line("  * * *  "), Some(SceneBreakTier::Minor));
        assert_eq!(tier_of_plain_line("#\t#\t#"), Some(SceneBreakTier::Major));
    }

    #[test]
    fn ordinary_prose_is_never_a_marker() {
        for line in [
            "",
            "   ",
            "She left without a word.",
            "He rated it 5 stars: *****!",
            "The * marks a footnote.",
            "#hashtag",
            "* a bullet-looking line",
        ] {
            assert_eq!(tier_of_plain_line(line), None, "prose {line:?}");
        }
    }

    #[test]
    fn stripping_djot_removes_only_the_marker_block() {
        let src = "First scene ends.\n\n\\* \\* \\*\n\nSecond scene begins.";
        assert_eq!(
            strip_markers_djot(src),
            "First scene ends.\n\nSecond scene begins."
        );
    }

    #[test]
    fn stripping_plain_text_removes_only_the_marker_line() {
        let src = "First scene ends.\n* * *\nSecond scene begins.";
        assert_eq!(
            strip_markers_plain(src),
            "First scene ends.\nSecond scene begins."
        );
    }

    #[test]
    fn stripping_prose_without_markers_borrows_unchanged() {
        let src = "Nothing to strip here at all.";
        assert!(matches!(strip_markers_djot(src), Cow::Borrowed(_)));
        assert!(matches!(strip_markers_plain(src), Cow::Borrowed(_)));
    }

    #[test]
    fn stripping_does_not_touch_prose_that_merely_contains_an_asterisk() {
        let src = "He was *emphatic* about it.";
        assert_eq!(strip_markers_djot(src), src);
    }

    #[test]
    fn unescaping_leaves_a_backslash_before_a_letter_alone() {
        // Djot only escapes punctuation, so `\n` here is a literal backslash-n,
        // not an escape — dropping the backslash would corrupt the prose.
        assert_eq!(unescape_djot("path\\name"), "path\\name");
        assert_eq!(unescape_djot("a \\* b"), "a * b");
    }

    /// A lone ellipsis is a beat of silence, not furniture.
    ///
    /// It was briefly in `MAJOR_MARKERS`, because the Russian preset emitted one.
    /// That made a writer's own trailing-off paragraph vanish from their word
    /// count and come back as a break glyph on export — and smart punctuation
    /// turns a typed `...` into exactly that character, so it was not even a rare
    /// spelling. The preset now emits a spaced row of dots instead, and this pins
    /// the difference.
    #[test]
    fn a_lone_ellipsis_is_prose_and_a_row_of_dots_is_a_break() {
        assert_eq!(tier_of_plain_line("…"), None);
        assert_eq!(tier_of_plain_line("..."), None);
        assert_eq!(
            tier_of_plain_line(". . ."),
            Some(SceneBreakTier::Major),
            "the Russian preset's own mark must be recognised"
        );
    }

    /// The word count is where the old behaviour did its damage.
    #[test]
    fn a_silence_beat_survives_stripping_but_a_real_mark_does_not() {
        let prose = "He waited.\n\n…\n\nShe never came.";
        assert_eq!(
            strip_markers_djot(prose),
            prose,
            "an ellipsis paragraph is the writer's, not the app's"
        );
        assert_eq!(
            strip_markers_djot("He waited.\n\n. . .\n\nShe never came."),
            "He waited.\n\nShe never came."
        );
    }

    /// The cheap pre-filter must not be tripped by ordinary sentences now that
    /// `.` is a marker character — it is line-anchored precisely for this.
    #[test]
    fn ordinary_prose_never_reaches_the_marker_matcher() {
        assert!(!might_contain_marker(
            "She turned the corner. The street was gone. Nothing moved."
        ));
        assert!(might_contain_marker("Before.\n\n. . .\n\nAfter."));
    }
}
