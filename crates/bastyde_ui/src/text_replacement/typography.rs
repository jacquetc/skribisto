// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Locale typography: which glyphs a language actually uses, and the stateless
//! rules that substitute them while the writer types.
//!
//! Pure Rust — no widget, no backend calls. Like [`engine`](super::engine) this
//! module only answers "does this text end in something that should change?";
//! *when* to ask and how to apply the answer lives in [`session`](super::session).
//!
//! ## Two things, deliberately kept apart
//!
//! - A [`TypographyRuleset`] is **what a locale does** — French closes with `»`,
//!   Swedish opens and closes with the same `”`, Arabic writes its comma `،`.
//!   Static data, one row per locale, no user preference in it.
//! - A [`SmartPunctuationFlags`] is **which rules this project wants on**, read
//!   from the per-Work `SmartPunctuation` entity that travels in the `.skrib`.
//!
//! Keeping them apart is what lets a house style ("this book uses guillemets
//! even though it is in English") be a four-value override rather than a fork of
//! the locale table.
//!
//! ## Why the rules here are stateless
//!
//! Every rule in this module decides from the handful of characters immediately
//! behind the caret and nothing else. That is what makes them safe to run on
//! every keystroke and trivial to test. The rules that genuinely cannot work
//! that way — a dialogue dash needs to know a paragraph just started, Spanish
//! `¿` needs to know where the clause began — are **not** here; they need the
//! shared paragraph/clause subsystem and are tracked separately.
//!
//! ## Reversibility
//!
//! Every substitution goes through the same backspace-revert path as a lexicon
//! expansion, so a writer who wanted a literal `--` gets it back by pressing
//! backspace once. That is the reason these fire immediately rather than
//! waiting for the end of the word: an immediate change the writer can see and
//! undo beats a delayed one that surprises them two words later.

use common::entities::QuoteStyle;
use skribisto_model::language;

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
    /// Not yet consulted by a rule — nesting depth is paragraph state, not tail
    /// state — but recorded per locale because the data is the hard part to get
    /// right and Russian inverts what Polish does, so a later reader must not
    /// guess it from the primary pair.
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
    /// Recorded here, applied by the paragraph-aware subsystem rather than by
    /// [`TypographyEngine`] — a dialogue dash is meaningless without knowing a
    /// paragraph just began.
    pub dialogue_dash: Option<char>,
    /// ASCII punctuation this locale writes with its own glyph.
    ///
    /// Only ever non-empty for Arabic-script locales. Gated on
    /// [`language::uses_arabic_script`] rather than on `is_rtl`, because Hebrew
    /// is right-to-left and keeps the ASCII marks — mirroring off `is_rtl` would
    /// corrupt Hebrew prose.
    pub mirrored_punctuation: &'static [(char, char)],
}

// ── Glyph names, so the table below reads as prose rather than as code points ──

const LEFT_DOUBLE: char = '\u{201C}'; // “
const RIGHT_DOUBLE: char = '\u{201D}'; // ”
const LOW_DOUBLE: char = '\u{201E}'; // „
const LEFT_SINGLE: char = '\u{2018}'; // ‘
const RIGHT_SINGLE: char = '\u{2019}'; // ’
const LOW_SINGLE: char = '\u{201A}'; // ‚
const LAQUO: char = '\u{00AB}'; // «
const RAQUO: char = '\u{00BB}'; // »
const LSAQUO: char = '\u{2039}'; // ‹
const RSAQUO: char = '\u{203A}'; // ›
const EM_DASH: char = '\u{2014}'; // —
const EN_DASH: char = '\u{2013}'; // –
const ELLIPSIS: char = '\u{2026}'; // …
const NBSP: char = '\u{00A0}'; // no-break space
const NNBSP: char = '\u{202F}'; // narrow no-break space

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
    mirrored_punctuation: NO_MIRRORING,
};

/// Shorthand for a row that differs from [`DEFAULT_RULESET`] only in the fields
/// given — keeps the table below readable.
const fn ruleset(
    tag: &'static str,
    primary: QuoteSystem,
    secondary: QuoteSystem,
    pre_punctuation: &'static [(char, char)],
    dialogue_dash: Option<char>,
    mirrored_punctuation: &'static [(char, char)],
) -> TypographyRuleset {
    TypographyRuleset {
        tag,
        primary_quotes: primary,
        secondary_quotes: secondary,
        pre_punctuation,
        dialogue_dash,
        mirrored_punctuation,
    }
}

const PAIR_CURLY: QuoteSystem = QuoteSystem::Paired {
    open: LEFT_DOUBLE,
    close: RIGHT_DOUBLE,
};
const PAIR_SINGLE_CURLY: QuoteSystem = QuoteSystem::Paired {
    open: LEFT_SINGLE,
    close: RIGHT_SINGLE,
};
const PAIR_GUILLEMET: QuoteSystem = QuoteSystem::Paired {
    open: LAQUO,
    close: RAQUO,
};
const PAIR_SINGLE_GUILLEMET: QuoteSystem = QuoteSystem::Paired {
    open: LSAQUO,
    close: RSAQUO,
};
/// German-style: low opening, high closing.
const PAIR_LOW_HIGH: QuoteSystem = QuoteSystem::Paired {
    open: LOW_DOUBLE,
    close: LEFT_DOUBLE,
};
const PAIR_SINGLE_LOW_HIGH: QuoteSystem = QuoteSystem::Paired {
    open: LOW_SINGLE,
    close: LEFT_SINGLE,
};
/// Polish: low opening, but a *right* double closing — deliberately not
/// German's pair, which is the mistake this constant exists to prevent.
const PAIR_POLISH: QuoteSystem = QuoteSystem::Paired {
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
    ruleset("en", PAIR_CURLY, PAIR_SINGLE_CURLY, NO_SPACING, None, NO_MIRRORING),
    // ── French: the one locale with pre-punctuation spacing ──────────────────
    ruleset(
        "fr",
        PAIR_GUILLEMET,
        PAIR_CURLY,
        FRENCH_SPACING,
        Some(EM_DASH),
        NO_MIRRORING,
    ),
    // ── German: „…“, except Switzerland, where it is officially prohibited ───
    ruleset("de", PAIR_LOW_HIGH, PAIR_SINGLE_LOW_HIGH, NO_SPACING, None, NO_MIRRORING),
    ruleset(
        "de-CH",
        PAIR_GUILLEMET,
        PAIR_SINGLE_GUILLEMET,
        NO_SPACING,
        None,
        NO_MIRRORING,
    ),
    // ── Iberian and Italian ──────────────────────────────────────────────────
    ruleset("es", PAIR_GUILLEMET, PAIR_CURLY, NO_SPACING, Some(EM_DASH), NO_MIRRORING),
    ruleset("ca", PAIR_GUILLEMET, PAIR_CURLY, NO_SPACING, Some(EM_DASH), NO_MIRRORING),
    // Italian genuinely has three co-existing systems; guillemets are the
    // literary default and the house-style override covers the other two.
    ruleset("it", PAIR_GUILLEMET, PAIR_CURLY, NO_SPACING, Some(EM_DASH), NO_MIRRORING),
    // Portugal and Brazil diverge on quotes and agree on the travessão.
    ruleset("pt", PAIR_GUILLEMET, PAIR_CURLY, NO_SPACING, Some(EM_DASH), NO_MIRRORING),
    ruleset("pt-BR", PAIR_CURLY, PAIR_SINGLE_CURLY, NO_SPACING, Some(EM_DASH), NO_MIRRORING),
    // ── Dutch ────────────────────────────────────────────────────────────────
    ruleset("nl", PAIR_CURLY, PAIR_SINGLE_CURLY, NO_SPACING, None, NO_MIRRORING),
    // ── Slavic ───────────────────────────────────────────────────────────────
    // Polish opens low and closes high-right — NOT the German pair.
    ruleset("pl", PAIR_POLISH, PAIR_SINGLE_LOW_HIGH, NO_SPACING, Some(EM_DASH), NO_MIRRORING),
    // Russian nests the reverse of Polish: guillemets outside, low-high inside.
    ruleset("ru", PAIR_GUILLEMET, PAIR_LOW_HIGH, NO_SPACING, Some(EM_DASH), NO_MIRRORING),
    // ── Swedish: the same glyph both ends ────────────────────────────────────
    ruleset(
        "sv",
        QuoteSystem::Symmetric(RIGHT_DOUBLE),
        QuoteSystem::Symmetric(RIGHT_SINGLE),
        NO_SPACING,
        Some(EM_DASH),
        NO_MIRRORING,
    ),
    // ── Turkish: double quotes, em-dash dialogue, and explicitly NO French
    //    spacing — porting that rule across would be an actual error here. ────
    ruleset("tr", PAIR_CURLY, PAIR_SINGLE_CURLY, NO_SPACING, Some(EM_DASH), NO_MIRRORING),
    // ── Arabic: the punctuation-mirroring row ────────────────────────────────
    ruleset(
        "ar",
        PAIR_GUILLEMET,
        PAIR_CURLY,
        NO_SPACING,
        Some(EM_DASH),
        ARABIC_MIRRORING,
    ),
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

/// Which substitutions a project wants, read from its `SmartPunctuation` row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmartPunctuationFlags {
    pub dashes: bool,
    pub ellipsis: bool,
    pub quotes: bool,
    pub quote_style: QuoteStyle,
    pub pre_punctuation_spacing: bool,
}

impl SmartPunctuationFlags {
    /// Every rule off — what a session uses before its project's row has
    /// resolved, and what a project that follows the app default with nothing
    /// configured amounts to.
    pub fn all_off() -> Self {
        Self {
            dashes: false,
            ellipsis: false,
            quotes: false,
            quote_style: QuoteStyle::LocaleDefault,
            pre_punctuation_spacing: false,
        }
    }
}

impl Default for SmartPunctuationFlags {
    /// Everything on but the French spacing.
    ///
    /// Dashes, ellipsis and quotes are what a writer means by "smart
    /// punctuation" and are safe in every locale. Pre-punctuation spacing is off
    /// because it is French-only and inserts an invisible character; a writer
    /// who wants it should be able to point at the setting they turned on.
    fn default() -> Self {
        Self {
            dashes: true,
            ellipsis: true,
            quotes: true,
            quote_style: QuoteStyle::LocaleDefault,
            pre_punctuation_spacing: false,
        }
    }
}

/// A fired substitution: how much of the tail to replace, and with what.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fired {
    /// How many **characters** at the end of the inspected text to remove.
    pub replace_chars: usize,
    /// What replaces them.
    pub replacement: String,
    /// Exactly the characters removed, so a backspace-revert can put the
    /// writer's own keystrokes back rather than a reconstruction of them.
    pub typed: String,
}

/// The compiled per-document typography rules.
#[derive(Clone, Debug)]
pub struct TypographyEngine {
    ruleset: &'static TypographyRuleset,
    mirrored: &'static [(char, char)],
    flags: SmartPunctuationFlags,
    quotes: QuoteSystem,
}

impl Default for TypographyEngine {
    fn default() -> Self {
        Self::new("", SmartPunctuationFlags::default())
    }
}

impl TypographyEngine {
    /// Compile the rules for a document written in `locale`.
    pub fn new(locale: &str, flags: SmartPunctuationFlags) -> Self {
        let ruleset = ruleset_for(locale);
        let quotes = match flags.quote_style {
            QuoteStyle::LocaleDefault => ruleset.primary_quotes,
            QuoteStyle::CurlyDouble => PAIR_CURLY,
            QuoteStyle::Guillemets => PAIR_GUILLEMET,
            QuoteStyle::LowHigh => PAIR_LOW_HIGH,
        };
        Self {
            ruleset,
            mirrored: mirrored_for(locale),
            flags,
            quotes,
        }
    }

    /// The locale row in force, for tests and diagnostics.
    pub fn ruleset(&self) -> &'static TypographyRuleset {
        self.ruleset
    }

    /// How many characters of trailing context [`check`](Self::check) needs.
    ///
    /// Three: the longest pattern is `...`, and every other rule looks at the
    /// typed character plus at most one before it.
    pub fn window_chars(&self) -> usize {
        3
    }

    /// Does `before` — the text up to and including the character just typed —
    /// end in something this locale would set differently?
    ///
    /// Rules are tried longest-pattern first so `...` wins over any single-
    /// character rule, and `---` over `--`.
    pub fn check(&self, before: &str) -> Option<Fired> {
        if before.is_empty() {
            return None;
        }
        let last = before.chars().next_back()?;

        // ── Ellipsis: three full stops become one glyph ──────────────────────
        if self.flags.ellipsis && before.ends_with("...") {
            return Some(fired(3, ELLIPSIS.to_string(), "..."));
        }

        // ── Dashes ───────────────────────────────────────────────────────────
        //
        // Three shapes rather than two, because the second dash has already
        // been converted by the time the writer types the third: `--` became
        // `–`, so a literal `---` never reaches this function. Handling `–-`
        // is what makes typing three hyphens produce an em dash.
        if self.flags.dashes {
            if before.ends_with("---") {
                return Some(fired(3, EM_DASH.to_string(), "---"));
            }
            let en_then_hyphen: String = [EN_DASH, '-'].iter().collect();
            if before.ends_with(&en_then_hyphen) {
                return Some(fired(2, EM_DASH.to_string(), &en_then_hyphen));
            }
            if before.ends_with("--") {
                return Some(fired(2, EN_DASH.to_string(), "--"));
            }
        }

        // ── Arabic-script punctuation ────────────────────────────────────────
        //
        // Before the spacing rule, though they never collide in practice: no
        // Arabic-script locale carries a `pre_punctuation` table, and no
        // French-spacing locale carries a mirroring one.
        if let Some(&(_, localized)) = self.mirrored.iter().find(|(ascii, _)| *ascii == last) {
            return Some(fired(1, localized.to_string(), &last.to_string()));
        }

        // ── French pre-punctuation spacing ───────────────────────────────────
        //
        // Fires on the punctuation mark, replacing the ordinary space the
        // writer typed before it. Only an ASCII space is consumed: if the
        // preceding character is already a no-break space the rule has already
        // run, and re-firing would loop.
        if self.flags.pre_punctuation_spacing {
            if let Some(&(_, space)) = self
                .ruleset
                .pre_punctuation
                .iter()
                .find(|(mark, _)| *mark == last)
            {
                let mut chars = before.chars().rev();
                let _mark = chars.next();
                if chars.next() == Some(' ') {
                    let typed: String = [' ', last].iter().collect();
                    let replacement: String = [space, last].iter().collect();
                    return Some(fired(2, replacement, &typed));
                }
            }
        }

        // ── Quotes and the apostrophe ────────────────────────────────────────
        if self.flags.quotes {
            if last == '"' {
                let glyph = if self.opens_here(before) {
                    self.quotes.open()
                } else {
                    self.quotes.close()
                };
                return Some(fired(1, glyph.to_string(), "\""));
            }
            if last == '\'' {
                // Always the right single quote: in running prose an ASCII
                // apostrophe is overwhelmingly an elision or a possessive, not
                // an opening single quotation mark, and getting the common case
                // right matters more than the rare one.
                return Some(fired(1, RIGHT_SINGLE.to_string(), "'"));
            }
        }

        None
    }

    /// Whether a `"` at the end of `before` opens rather than closes.
    ///
    /// Opens at the very start of the text, and after whitespace or an opening
    /// bracket or dash; closes otherwise. For a [`QuoteSystem::Symmetric`]
    /// locale the answer changes nothing — both ends are the same glyph — which
    /// is exactly why Swedish needs no special case anywhere else.
    fn opens_here(&self, before: &str) -> bool {
        let mut chars = before.chars().rev();
        let _quote = chars.next();
        match chars.next() {
            None => true,
            Some(prev) => {
                prev.is_whitespace()
                    || matches!(prev, '(' | '[' | '{' | EM_DASH | EN_DASH | '-' | LAQUO)
            }
        }
    }
}

fn fired(replace_chars: usize, replacement: String, typed: &str) -> Fired {
    Fired {
        replace_chars,
        replacement,
        typed: typed.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn en() -> TypographyEngine {
        TypographyEngine::new("en-US", SmartPunctuationFlags::default())
    }

    fn french() -> TypographyEngine {
        TypographyEngine::new(
            "fr-FR",
            SmartPunctuationFlags {
                pre_punctuation_spacing: true,
                ..SmartPunctuationFlags::default()
            },
        )
    }

    // ── The locale table ─────────────────────────────────────────────────────

    #[test]
    fn a_region_falls_back_to_its_language_but_an_exact_row_wins() {
        assert_eq!(ruleset_for("fr-CA").tag, "fr", "no fr-CA row, inherit fr");
        assert_eq!(ruleset_for("de-AT").tag, "de", "Austria uses the German row");
        assert_eq!(ruleset_for("de-CH").tag, "de-CH", "Switzerland has its own");
        assert_eq!(ruleset_for("pt-BR").tag, "pt-BR");
        assert_eq!(ruleset_for("pt-PT").tag, "pt");
    }

    #[test]
    fn an_unknown_locale_still_gets_the_default_row() {
        for tag in ["", "  ", "xx", "klingon-PQ"] {
            assert_eq!(ruleset_for(tag).tag, "", "{tag} should fall back");
        }
    }

    #[test]
    fn tag_lookup_is_case_and_separator_tolerant() {
        assert_eq!(ruleset_for("FR_fr").tag, "fr");
        assert_eq!(ruleset_for("DE_ch").tag, "de-CH");
    }

    /// Polish and German both open low, and close differently. Getting this
    /// wrong is the specific error the design doc warned about.
    #[test]
    fn polish_is_not_german() {
        let pl = ruleset_for("pl").primary_quotes;
        let de = ruleset_for("de").primary_quotes;
        assert_eq!(pl.open(), de.open(), "both open with the low double");
        assert_ne!(pl.close(), de.close(), "but they close differently");
        assert_eq!(pl.close(), RIGHT_DOUBLE);
        assert_eq!(de.close(), LEFT_DOUBLE);
    }

    /// Russian nests the reverse of Polish — guillemets outside, low-high in.
    #[test]
    fn russian_nesting_inverts_polish() {
        let ru = ruleset_for("ru");
        assert_eq!(ru.primary_quotes, PAIR_GUILLEMET);
        assert_eq!(ru.secondary_quotes, PAIR_LOW_HIGH);
    }

    /// The case that breaks a naive open/close toggle.
    #[test]
    fn swedish_uses_one_glyph_at_both_ends() {
        let sv = ruleset_for("sv").primary_quotes;
        assert!(matches!(sv, QuoteSystem::Symmetric(_)));
        assert_eq!(sv.open(), sv.close());
    }

    // ── Ellipsis and dashes ──────────────────────────────────────────────────

    #[test]
    fn three_dots_become_an_ellipsis() {
        let f = en().check("wait...").expect("should fire");
        assert_eq!(f.replace_chars, 3);
        assert_eq!(f.replacement, "…");
        assert_eq!(f.typed, "...");
        assert!(en().check("wait..").is_none(), "two dots are not an ellipsis");
    }

    /// Typing three hyphens has to reach an em dash even though the engine
    /// already rewrote the first two into an en dash.
    #[test]
    fn hyphens_climb_from_en_dash_to_em_dash() {
        let e = en();
        let first = e.check("a--").expect("two hyphens fire");
        assert_eq!(first.replacement, "–", "two hyphens make an en dash");
        assert_eq!(first.replace_chars, 2);

        // What the buffer actually looks like once that has been applied.
        let second = e.check("a–-").expect("en dash plus hyphen fires");
        assert_eq!(second.replacement, "—", "the third hyphen makes an em dash");
        assert_eq!(second.replace_chars, 2);
    }

    /// A literal `---` can only arrive from a paste or an import, but it must
    /// still resolve to an em dash rather than to an en dash plus a stray.
    #[test]
    fn a_literal_triple_hyphen_becomes_an_em_dash() {
        let f = en().check("a---").expect("should fire");
        assert_eq!(f.replacement, "—");
        assert_eq!(f.replace_chars, 3);
    }

    #[test]
    fn a_single_hyphen_is_left_alone() {
        assert!(en().check("well-known").is_none());
    }

    // ── Quotes ───────────────────────────────────────────────────────────────

    #[test]
    fn a_quote_opens_at_the_start_and_after_a_space() {
        for text in ["\"", "he said \"", "(\"", "—\""] {
            let f = en().check(text).unwrap_or_else(|| panic!("{text} fires"));
            assert_eq!(f.replacement, "“", "{text} should open");
        }
    }

    #[test]
    fn a_quote_closes_after_a_word_or_its_punctuation() {
        for text in ["word\"", "end.\"", "yes!\""] {
            let f = en().check(text).unwrap_or_else(|| panic!("{text} fires"));
            assert_eq!(f.replacement, "”", "{text} should close");
        }
    }

    /// Swedish emits the same glyph either way — the toggle is a no-op there,
    /// and that must not read as a bug.
    #[test]
    fn a_symmetric_locale_emits_one_glyph_whichever_side_it_is() {
        let sv = TypographyEngine::new("sv-SE", SmartPunctuationFlags::default());
        let opening = sv.check("han sa \"").expect("fires");
        let closing = sv.check("ord\"").expect("fires");
        assert_eq!(opening.replacement, "”");
        assert_eq!(closing.replacement, opening.replacement);
    }

    #[test]
    fn french_opens_with_a_guillemet() {
        let f = french().check("il dit \"").expect("fires");
        assert_eq!(f.replacement, "«");
    }

    /// The text handed to `check` ends at the character just typed — so the
    /// apostrophe is the *last* character, not one in the middle of a word that
    /// was finished long ago.
    #[test]
    fn an_apostrophe_becomes_the_typographic_one() {
        let f = en().check("don'").expect("fires");
        assert_eq!(f.replacement, "’");
        assert_eq!(f.replace_chars, 1);
        assert!(
            en().check("don't").is_none(),
            "a finished word is not re-examined"
        );
    }

    /// The house-style override replaces the locale's own pair outright.
    #[test]
    fn a_house_style_override_beats_the_locale() {
        let english_with_guillemets = TypographyEngine::new(
            "en-US",
            SmartPunctuationFlags {
                quote_style: QuoteStyle::Guillemets,
                ..SmartPunctuationFlags::default()
            },
        );
        let f = english_with_guillemets.check("he said \"").expect("fires");
        assert_eq!(f.replacement, "«", "the override wins over en-US curly");
    }

    // ── French spacing ───────────────────────────────────────────────────────

    #[test]
    fn french_puts_a_thin_space_before_a_question_mark() {
        let f = french().check("Quoi ?").expect("fires");
        assert_eq!(f.replace_chars, 2, "the space and the mark are replaced");
        assert_eq!(f.replacement, "\u{202F}?");
        assert_eq!(f.typed, " ?");
    }

    /// The colon is genuinely the exception — a full no-break space, not a thin
    /// one. Spelled out because it looks like an inconsistency otherwise.
    #[test]
    fn french_uses_a_full_space_before_a_colon_only() {
        let colon = french().check("ceci :").expect("fires");
        assert_eq!(colon.replacement, "\u{00A0}:");
        let semicolon = french().check("ceci ;").expect("fires");
        assert_eq!(semicolon.replacement, "\u{202F};");
    }

    /// Without a preceding ordinary space there is nothing to upgrade, and once
    /// upgraded the rule must not fire again — that would loop forever.
    #[test]
    fn french_spacing_needs_a_plain_space_and_never_re_fires() {
        assert!(french().check("Quoi?").is_none(), "no space, nothing to do");
        assert!(
            french().check("Quoi\u{202F}?").is_none(),
            "already a thin space — re-firing would loop"
        );
    }

    #[test]
    fn other_locales_get_no_pre_punctuation_spacing() {
        for tag in ["en-US", "de-DE", "tr-TR", "ru-RU"] {
            let e = TypographyEngine::new(
                tag,
                SmartPunctuationFlags {
                    pre_punctuation_spacing: true,
                    ..SmartPunctuationFlags::default()
                },
            );
            assert!(
                e.check("Quoi ?").is_none(),
                "{tag} must not take French spacing"
            );
        }
    }

    // ── Arabic ───────────────────────────────────────────────────────────────

    #[test]
    fn arabic_mirrors_its_punctuation() {
        let ar = TypographyEngine::new("ar", SmartPunctuationFlags::default());
        for (typed, want) in [("كيف?", "؟"), ("كيف,", "،"), ("كيف;", "؛")] {
            let f = ar.check(typed).unwrap_or_else(|| panic!("{typed} fires"));
            assert_eq!(f.replacement, want);
            assert_eq!(f.replace_chars, 1);
        }
    }

    #[test]
    fn persian_and_urdu_mirror_too() {
        for tag in ["fa", "ur", "ps", "ckb"] {
            let e = TypographyEngine::new(tag, SmartPunctuationFlags::default());
            let f = e.check("x?").unwrap_or_else(|| panic!("{tag} fires"));
            assert_eq!(f.replacement, "؟", "{tag} mirrors the question mark");
        }
    }

    /// The whole reason mirroring is gated on script rather than on direction.
    #[test]
    fn hebrew_keeps_its_ascii_punctuation() {
        let he = TypographyEngine::new("he-IL", SmartPunctuationFlags::default());
        assert!(he.check("מה?").is_none(), "Hebrew does not mirror `?`");
        assert!(he.check("מה,").is_none(), "…nor its comma");
    }

    #[test]
    fn the_full_stop_is_never_mirrored() {
        let ar = TypographyEngine::new("ar", SmartPunctuationFlags::default());
        // A lone `.` must be inert — `...` is the ellipsis rule, tested above.
        assert!(ar.check("كيف.").is_none());
    }

    // ── Flags ────────────────────────────────────────────────────────────────

    #[test]
    fn every_rule_can_be_switched_off_independently() {
        let off = SmartPunctuationFlags {
            dashes: false,
            ellipsis: false,
            quotes: false,
            quote_style: QuoteStyle::LocaleDefault,
            pre_punctuation_spacing: false,
        };
        let e = TypographyEngine::new("fr-FR", off);
        assert!(e.check("wait...").is_none());
        assert!(e.check("a--").is_none());
        assert!(e.check("il dit \"").is_none());
        assert!(e.check("Quoi ?").is_none());
    }

    /// Turning off the dash rule must not take the ellipsis with it.
    #[test]
    fn switching_off_one_rule_leaves_the_others_alone() {
        let e = TypographyEngine::new(
            "en-US",
            SmartPunctuationFlags {
                dashes: false,
                ..SmartPunctuationFlags::default()
            },
        );
        assert!(e.check("a--").is_none(), "dashes off");
        assert!(e.check("wait...").is_some(), "ellipsis still on");
    }

    /// Mirroring is not behind a flag of its own: it is what writing the
    /// language *is*, not a stylistic preference, so it stays on even when the
    /// stylistic rules are all off.
    #[test]
    fn arabic_mirroring_is_not_a_stylistic_preference() {
        let ar = TypographyEngine::new(
            "ar",
            SmartPunctuationFlags {
                dashes: false,
                ellipsis: false,
                quotes: false,
                quote_style: QuoteStyle::LocaleDefault,
                pre_punctuation_spacing: false,
            },
        );
        assert_eq!(ar.check("كيف?").expect("still fires").replacement, "؟");
    }

    // ── Nothing fires on ordinary prose ──────────────────────────────────────

    #[test]
    fn ordinary_text_is_left_completely_alone() {
        let e = en();
        for text in ["hello", "a", "word ", "1234", "naïve", "日本語", "", "x."] {
            assert!(e.check(text).is_none(), "{text:?} must not fire");
        }
    }
}
