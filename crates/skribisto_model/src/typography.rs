// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What a locale's typography actually is: which glyphs open and close a quotation, which
//! marks take a space before them, which dash opens a line of dialogue.
//!
//! Static data and one lookup. Domain facts about languages, not UI — which is why they live
//! here beside [`crate::language`], the other BCP-47-keyed table, rather than in the widget
//! crate that happens to have needed them first.
//!
//! ## Why this lives here, not in the UI crate
//!
//! Two features need these facts for opposite reasons: smart punctuation *inserts* the
//! glyphs as the writer types, and [`crate::analysis::prose_stats`] *recognises* them to
//! measure how much of a scene is dialogue. A single table + lookup lets both agree, not
//! just on the rows but on *resolution* (e.g. `en-US` falling back to `en`) — a duplicated
//! table can match row for row and still disagree on a tag neither copy normalises the same
//! way. The *stateful* half — which rules a project has switched on, and the per-keystroke
//! engine that applies them — stays in the UI crate. Only the facts moved.

/// The per-keystroke engine that applies the rulesets below to live text.
pub mod engine;

use crate::language;

/// How a locale opens and closes a quotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuoteSystem {
    /// Distinct opening and closing glyphs — the common case.
    Paired { open: char, close: char },
    /// The **same** glyph at both ends, as Swedish `”…”` does.
    ///
    /// Worth its own variant rather than a `Paired` with two equal fields: it is
    /// the case that breaks a naive "toggle between open and close" model, and
    /// naming it means the toggle logic cannot silently be wrong for Swedish.
    Symmetric(char),
}

impl QuoteSystem {
    /// The glyph that opens a quotation.
    pub fn open(self) -> char {
        match self {
            Self::Paired { open, .. } => open,
            Self::Symmetric(c) => c,
        }
    }

    /// The glyph that closes a quotation.
    pub fn close(self) -> char {
        match self {
            Self::Paired { close, .. } => close,
            Self::Symmetric(c) => c,
        }
    }
}

/// One locale's typographic conventions.
#[derive(Clone, Copy, Debug)]
pub struct TypographyRuleset {
    /// The BCP-47 tag this row was chosen for, for diagnostics and tests.
    pub tag: &'static str,
    /// Outer quotation marks.
    pub primary_quotes: QuoteSystem,
    /// Quotation marks *inside* an existing quotation.
    ///
    /// Consulted by the nesting rule in `TypographyEngine::check_paragraph`,
    /// but only where it is double-width (see `nests_with_double_key`);
    /// recorded for every locale regardless, because Russian inverts what Polish
    /// does and a later reader must not guess it from the primary pair.
    pub secondary_quotes: QuoteSystem,
    /// Punctuation that takes a space *before* it, and which space character.
    ///
    /// Empty for every locale but French. The space differs by mark, which is
    /// why this is pairs and not a character set plus one space: French sets a
    /// narrow no-break space before `;` `!` `?` and a full no-break space before
    /// `:`, and that is what LibreOffice's French autocorrect emits too.
    pub pre_punctuation: &'static [(char, char)],
    /// The dash that opens a line of dialogue, where the locale uses one.
    ///
    /// Read by `TypographyEngine::check_paragraph` rather than by the
    /// stateless `check` (`TypographyEngine::check`) — a dialogue dash is
    /// meaningless without knowing a paragraph just began.
    pub dialogue_dash: Option<char>,
}

// ── Glyph names, so the table below reads as prose rather than as code points ──
//
// Public because the UI's typing engine matches on them. That matters more than it looks:
// a constant used in a `match` pattern that is *not* in scope does not fail to resolve — it
// silently becomes a fresh binding that matches everything. Uppercase names are what make
// the compiler catch that, and exporting them is what keeps one spelling of each glyph.

pub const LEFT_DOUBLE: char = '\u{201C}'; // “
pub const RIGHT_DOUBLE: char = '\u{201D}'; // ”
pub const LOW_DOUBLE: char = '\u{201E}'; // „
pub const LEFT_SINGLE: char = '\u{2018}'; // ‘
pub const RIGHT_SINGLE: char = '\u{2019}'; // ’
pub const LOW_SINGLE: char = '\u{201A}'; // ‚
pub const LAQUO: char = '\u{00AB}'; // «
pub const RAQUO: char = '\u{00BB}'; // »
pub const LSAQUO: char = '\u{2039}'; // ‹
pub const RSAQUO: char = '\u{203A}'; // ›
pub const EM_DASH: char = '\u{2014}'; // —
pub const EN_DASH: char = '\u{2013}'; // –
pub const ELLIPSIS: char = '\u{2026}'; // …
pub const NBSP: char = '\u{00A0}'; // no-break space
pub const NNBSP: char = '\u{202F}'; // narrow no-break space

/// French spacing: a narrow no-break space before `;` `!` `?` and `»`, a full
/// no-break space before `:`.
///
/// The `:` really is the odd one out — it is set with a full space in French
/// practice while the others take a thin one, and both LibreOffice and the
/// Imprimerie nationale's rules agree on the distinction.
const FRENCH_SPACING: &[(char, char)] = &[
    (';', NNBSP),
    ('!', NNBSP),
    ('?', NNBSP),
    (RAQUO, NNBSP),
    (':', NBSP),
];

/// Arabic-script punctuation. The question mark, comma and semicolon each have
/// their own glyph; the full stop does **not** change (see the module note in
/// [`mirrored_for`]).
const ARABIC_MIRRORING: &[(char, char)] = &[
    ('?', '\u{061F}'), // ؟
    (',', '\u{060C}'), // ،
    (';', '\u{061B}'), // ؛
];

const NO_SPACING: &[(char, char)] = &[];
const NO_MIRRORING: &[(char, char)] = &[];

/// The default row: straight-ish English convention, used for any tag with no
/// entry of its own. Never fails, so an unknown locale still gets ellipsis and
/// dash conversion rather than nothing.
const DEFAULT_RULESET: TypographyRuleset = TypographyRuleset {
    tag: "",
    primary_quotes: QuoteSystem::Paired {
        open: LEFT_DOUBLE,
        close: RIGHT_DOUBLE,
    },
    secondary_quotes: QuoteSystem::Paired {
        open: LEFT_SINGLE,
        close: RIGHT_SINGLE,
    },
    pre_punctuation: NO_SPACING,
    dialogue_dash: None,
};

/// Shorthand for a row that differs from [`DEFAULT_RULESET`] only in the fields
/// given — keeps the table below readable.
const fn ruleset(
    tag: &'static str,
    primary: QuoteSystem,
    secondary: QuoteSystem,
    pre_punctuation: &'static [(char, char)],
    dialogue_dash: Option<char>,
) -> TypographyRuleset {
    TypographyRuleset {
        tag,
        primary_quotes: primary,
        secondary_quotes: secondary,
        pre_punctuation,
        dialogue_dash,
    }
}

pub const PAIR_CURLY: QuoteSystem = QuoteSystem::Paired {
    open: LEFT_DOUBLE,
    close: RIGHT_DOUBLE,
};
pub const PAIR_SINGLE_CURLY: QuoteSystem = QuoteSystem::Paired {
    open: LEFT_SINGLE,
    close: RIGHT_SINGLE,
};
pub const PAIR_GUILLEMET: QuoteSystem = QuoteSystem::Paired {
    open: LAQUO,
    close: RAQUO,
};
pub const PAIR_SINGLE_GUILLEMET: QuoteSystem = QuoteSystem::Paired {
    open: LSAQUO,
    close: RSAQUO,
};
/// German-style: low opening, high closing.
pub const PAIR_LOW_HIGH: QuoteSystem = QuoteSystem::Paired {
    open: LOW_DOUBLE,
    close: LEFT_DOUBLE,
};
pub const PAIR_SINGLE_LOW_HIGH: QuoteSystem = QuoteSystem::Paired {
    open: LOW_SINGLE,
    close: LEFT_SINGLE,
};
/// Polish: low opening, but a *right* double closing — deliberately not
/// German's pair, which is the mistake this constant exists to prevent.
pub const PAIR_POLISH: QuoteSystem = QuoteSystem::Paired {
    open: LOW_DOUBLE,
    close: RIGHT_DOUBLE,
};

/// The curated locale table.
///
/// Scoped to the locales Skribisto ships a spellcheck dictionary for, plus
/// Turkish and Arabic. Those two carry no curated dictionary but do carry
/// correctness requirements of their own — Turkish because its dotted/dotless I
/// breaks case-insensitive matching outright, Arabic because its punctuation
/// genuinely differs — and typography support was never gated on a dictionary
/// existing.
///
/// Matched most-specific first: an exact tag, then the bare language. So
/// `de-CH` finds its guillemets while `de-BE` falls back to the German row.
const RULESETS: &[TypographyRuleset] = &[
    // ── English ──────────────────────────────────────────────────────────────
    ruleset("en", PAIR_CURLY, PAIR_SINGLE_CURLY, NO_SPACING, None),
    // British publishing sets speech in singles and nests doubles inside — the
    // mirror of the American row above. Most-specific-first lookup means `en-GB`
    // finds this while `en-US`, `en-CA` and a bare `en` keep the row above; the
    // Australian and Canadian conventions are their own question, so neither
    // gets a row here rather than being guessed at.
    ruleset("en-GB", PAIR_SINGLE_CURLY, PAIR_CURLY, NO_SPACING, None),
    // ── French: the one locale with pre-punctuation spacing ──────────────────
    ruleset(
        "fr",
        PAIR_GUILLEMET,
        PAIR_CURLY,
        FRENCH_SPACING,
        Some(EM_DASH),
    ),
    // ── German: „…“, except Switzerland, where it is officially prohibited ───
    ruleset("de", PAIR_LOW_HIGH, PAIR_SINGLE_LOW_HIGH, NO_SPACING, None),
    ruleset(
        "de-CH",
        PAIR_GUILLEMET,
        PAIR_SINGLE_GUILLEMET,
        NO_SPACING,
        None,
    ),
    // ── Iberian and Italian ──────────────────────────────────────────────────
    ruleset("es", PAIR_GUILLEMET, PAIR_CURLY, NO_SPACING, Some(EM_DASH)),
    ruleset("ca", PAIR_GUILLEMET, PAIR_CURLY, NO_SPACING, Some(EM_DASH)),
    // Italian genuinely has three co-existing systems; guillemets are the
    // literary default and the house-style override covers the other two.
    ruleset("it", PAIR_GUILLEMET, PAIR_CURLY, NO_SPACING, Some(EM_DASH)),
    // Portugal and Brazil diverge on quotes and agree on the travessão.
    ruleset("pt", PAIR_GUILLEMET, PAIR_CURLY, NO_SPACING, Some(EM_DASH)),
    ruleset(
        "pt-BR",
        PAIR_CURLY,
        PAIR_SINGLE_CURLY,
        NO_SPACING,
        Some(EM_DASH),
    ),
    // ── Dutch ────────────────────────────────────────────────────────────────
    ruleset("nl", PAIR_CURLY, PAIR_SINGLE_CURLY, NO_SPACING, None),
    // ── Slavic ───────────────────────────────────────────────────────────────
    // Polish opens low and closes high-right — NOT the German pair.
    ruleset(
        "pl",
        PAIR_POLISH,
        PAIR_SINGLE_LOW_HIGH,
        NO_SPACING,
        Some(EM_DASH),
    ),
    // Russian nests the reverse of Polish: guillemets outside, low-high inside.
    ruleset(
        "ru",
        PAIR_GUILLEMET,
        PAIR_LOW_HIGH,
        NO_SPACING,
        Some(EM_DASH),
    ),
    // ── Swedish: the same glyph both ends ────────────────────────────────────
    ruleset(
        "sv",
        QuoteSystem::Symmetric(RIGHT_DOUBLE),
        QuoteSystem::Symmetric(RIGHT_SINGLE),
        NO_SPACING,
        Some(EM_DASH),
    ),
    // ── Turkish: double quotes, em-dash dialogue, and explicitly NO French
    //    spacing — porting that rule across would be an actual error here. ────
    ruleset(
        "tr",
        PAIR_CURLY,
        PAIR_SINGLE_CURLY,
        NO_SPACING,
        Some(EM_DASH),
    ),
    // ── Arabic: the punctuation-mirroring row ────────────────────────────────
    ruleset("ar", PAIR_GUILLEMET, PAIR_CURLY, NO_SPACING, Some(EM_DASH)),
];

/// The ruleset for `tag`, most-specific match first, never failing.
///
/// `fr-CA` finds the French row through its language subtag even though only
/// `fr` is listed, which is the intent: a locale with no row of its own should
/// inherit its language's typography rather than fall back to English.
pub fn ruleset_for(tag: &str) -> &'static TypographyRuleset {
    let normalized = tag.trim().replace('_', "-").to_ascii_lowercase();
    if normalized.is_empty() {
        return &DEFAULT_RULESET;
    }
    // Exact tag first, so `de-CH` beats `de` and `pt-BR` beats `pt`.
    if let Some(found) = RULESETS
        .iter()
        .find(|r| r.tag.to_ascii_lowercase() == normalized)
    {
        return found;
    }
    let primary = normalized.split('-').next().unwrap_or("");
    RULESETS
        .iter()
        .find(|r| r.tag == primary)
        .unwrap_or(&DEFAULT_RULESET)
}

/// The mirroring table that applies to `tag`, or empty when it is not an
/// Arabic-script locale.
///
/// Split out from the table lookup so the script gate is stated once. The full
/// stop is deliberately **not** mirrored: Urdu does write `۔` for it, but a rule
/// that rewrites every typed `.` would also rewrite the ones in decimals,
/// abbreviations and file names, and there is no tail-local way to tell those
/// apart.
pub fn mirrored_for(tag: &str) -> &'static [(char, char)] {
    if language::uses_arabic_script(tag) {
        ARABIC_MIRRORING
    } else {
        NO_MIRRORING
    }
}
