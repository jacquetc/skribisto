// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Locale typography: which glyphs a language actually uses, and the stateless
//! rules that substitute them while the writer types.
//!
//! Pure Rust — no widget, no backend calls. Like
//! [`crate::replacement`] this
//! module only answers "does this text end in something that should change?";
//! *when* to ask and how to apply the answer lives in the UI's own
//! `text_replacement::session`.
//!
//! ## Two things, deliberately kept apart
//!
//! - A [`TypographyRuleset`] is **what a locale does** — French closes with `»`,
//!   Swedish opens and closes with the same `”`, Arabic writes its comma `،`.
//!   Static data, one row per locale, no user preference in it.
//! - A [`SmartPunctuationFlags`](crate::typography::engine::SmartPunctuationFlags)
//!   is **which rules this project wants on**, read
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
// The locale facts moved to the domain crate: the analysis backend needs them too, and a
// second copy there had already drifted on tag resolution. Re-exported so existing callers
// (and tests) keep their `text_replacement::typography::…` paths.
pub use super::{QuoteSystem, TypographyRuleset, mirrored_for, ruleset_for};
// The glyphs the typing rules match on. Imported rather than re-declared: a constant that is
// not in scope inside a `match` pattern becomes a catch-all binding instead of failing to
// resolve, so a second local copy is a genuinely dangerous kind of duplication here.
use super::{
    ELLIPSIS, EM_DASH, EN_DASH, LAQUO, LEFT_DOUBLE, LEFT_SINGLE, LOW_DOUBLE, LSAQUO, NNBSP, RAQUO,
    RIGHT_SINGLE,
};
// The named quote pairs a per-project house-style override selects between.
use super::{PAIR_CURLY, PAIR_GUILLEMET, PAIR_LOW_HIGH};

/// Which substitutions a project wants, read from its `SmartPunctuation` row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmartPunctuationFlags {
    pub dashes: bool,
    pub ellipsis: bool,
    pub quotes: bool,
    pub quote_style: QuoteStyle,
    pub pre_punctuation_spacing: bool,
    /// Open a paragraph typed as `- ` with the locale's dialogue dash.
    ///
    /// Unlike the four above this is a *paragraph* rule: it can only fire at the
    /// start of a paragraph, so it needs to know where that is. See
    /// [`TypographyEngine::check_paragraph`].
    pub dialogue_marker: bool,
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
            dialogue_marker: false,
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
            // Off with the spacing rule, and for a related reason: it rewrites
            // the *shape* of a line rather than one glyph inside it, and it is
            // wrong outright in the languages that quote their dialogue.
            dialogue_marker: false,
        }
    }
}

/// A fired substitution: how much of the tail to replace, and with what.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fired {
    /// How many **characters** back from the caret the substitution reaches.
    ///
    /// With [`prepend`](Self::prepend) false this is the count of tail
    /// characters *replaced* by `replacement`; with it true this is the offset
    /// back to the point `replacement` is *inserted* at, and no characters are
    /// removed.
    pub replace_chars: usize,
    /// What replaces (or is inserted at) the span.
    pub replacement: String,
    /// Exactly the characters removed, so a backspace-revert can put the
    /// writer's own keystrokes back rather than a reconstruction of them. Empty
    /// for a [`prepend`](Self::prepend), which removes nothing.
    pub typed: String,
    /// Insert `replacement` at `caret - replace_chars` **without deleting** the
    /// span between, leaving the caret where the writer left it.
    ///
    /// This is how Spanish's `¿` is applied: the mark belongs at the clause
    /// start, which can be most of a line back, but the surrounding clause must
    /// be left byte-for-byte intact — a tail *replace* that rewrote the clause
    /// as a plain string would flatten any bold/italic run inside it. The caller
    /// inserts the one mark and restores the caret to the end.
    pub prepend: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// The paragraph/clause subsystem
// ═══════════════════════════════════════════════════════════════════════════
//
// Everything above decides from the few characters behind the caret. These two
// rules cannot: a dialogue dash is only a dialogue dash at the *start of a
// paragraph*, and Spanish's `¿` has to be inserted where the **clause** began,
// which can be most of a line back and is not where the caret is.
//
// One subsystem for both, rather than a state machine per locale, because they
// need the same thing — the text of the current paragraph up to the caret — and
// that is exactly what `TextCursor::position_in_block()` bounds for free. Asking
// for `text_before(position_in_block())` cannot read past the paragraph start,
// so there is no scanning for newlines and no risk of a rule reaching into the
// paragraph above.

/// Spanish opens a question with `¿` and an exclamation with `¡`.
const SPANISH_INVERTED: &[(char, char)] = &[('?', '\u{00BF}'), ('!', '\u{00A1}')];

/// Whether `tag` writes its questions and exclamations with an opening mark.
///
/// Spanish and Asturian; **not** Catalan, Galician or Portuguese, which are
/// neighbours that do not do this. Getting that wrong would insert a character
/// no reader of those languages expects.
fn uses_inverted_marks(tag: &str) -> bool {
    let primary = tag
        .split(['-', '_'])
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    primary == "es" || primary == "ast"
}

/// Where the clause containing the caret began, as a character offset into
/// `block_before` (the paragraph up to the caret).
///
/// Scans back to the last clause boundary, then forward again over the things
/// that sit *outside* a clause but before its first word — whitespace, an
/// opening quotation mark, a dialogue dash. The inverted mark belongs after
/// those, not before them: a Spanish line of dialogue reads `—¿Qué?`, never
/// `¿—Qué?`.
///
/// `,` and `;` count as boundaries because Spanish genuinely re-opens mid
/// sentence — *Si puedes, ¿vienes?* — which is the case that makes this need a
/// clause scan at all rather than a sentence one.
///
/// `chars` is the paragraph up to the caret **excluding the mark just typed**.
/// That exclusion is load-bearing: `?` is itself a clause boundary, so scanning
/// with it included finds it, reports the clause as starting after it, and every
/// question resolves to an empty clause that never fires.
fn clause_start(chars: &[char]) -> usize {
    // A `.` or `:` sitting between two digits is a decimal point or a clock time
    // (`3.14`, `10:30`), not the end of a clause. Left to count as a boundary it
    // would open the mark mid-number — `Cuesta 3.¿14?`, `Son las 10:¿30?` — so
    // those two marks are boundaries only when they are not digit-flanked.
    let is_boundary = |i: usize| match chars[i] {
        '?' | '!' | ',' | ';' | '\u{2026}' => true,
        '.' | ':' => {
            let prev_digit = i.checked_sub(1).is_some_and(|p| chars[p].is_ascii_digit());
            let next_digit = chars.get(i + 1).is_some_and(char::is_ascii_digit);
            !(prev_digit && next_digit)
        }
        _ => false,
    };
    let boundary = (0..chars.len())
        .rev()
        .find(|&i| is_boundary(i))
        .map(|i| i + 1)
        .unwrap_or(0);
    let mut start = boundary;
    while start < chars.len() {
        let c = chars[start];
        // An existing `¿`/`¡` is deliberately NOT skipped. Skipping it puts the
        // clause start after it, so the already-opened guard below never sees
        // one and a writer who typed their own mark gets a second: `¿¿Vienes?`.
        //
        // A plain ASCII `-` is skipped alongside the real dashes: a writer whose
        // dialogue dash is still a hyphen (dash conversion off, or mid-type
        // before it fires) must still get `-¿Qué?`, not `¿-Qué?`.
        let skippable = c.is_whitespace()
            || matches!(
                c,
                LAQUO | LEFT_DOUBLE | LOW_DOUBLE | LEFT_SINGLE | LSAQUO | EM_DASH | EN_DASH | '-'
            );
        if !skippable {
            break;
        }
        start += 1;
    }
    start
}

/// The compiled per-document typography rules.
#[derive(Clone, Debug)]
pub struct TypographyEngine {
    ruleset: &'static TypographyRuleset,
    mirrored: &'static [(char, char)],
    /// Kept for the paragraph rules, which gate on the language itself rather
    /// than on anything the ruleset table carries.
    locale: String,
    flags: SmartPunctuationFlags,
    quotes: QuoteSystem,
    /// Whether to set a narrow no-break space inside guillemets (French). Cached
    /// from the locale so the per-keystroke quote path is a field read.
    guillemet_spacing: bool,
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
            locale: locale.to_string(),
            flags,
            quotes,
            guillemet_spacing: uses_guillemet_inner_spacing(locale),
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
        if self.flags.pre_punctuation_spacing
            && let Some(&(_, space)) = self
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

        // ── Quotes and the apostrophe ────────────────────────────────────────
        if self.flags.quotes {
            if last == '"' {
                // The char before the `"` is the one before `last` in `before`.
                let prev = before.chars().rev().nth(1);
                let glyph = if opens_after(prev) {
                    self.quotes.open()
                } else {
                    self.quotes.close()
                };
                return Some(fired(1, self.spaced_quote(glyph), "\""));
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

    /// The paragraph-aware rules: the two that cannot decide from the tail.
    ///
    /// `block_before` is the current paragraph up to the caret — bounded by
    /// `position_in_block()`, so it can never reach into the paragraph above.
    /// Offsets in the returned [`Fired`] are counted back from the caret exactly
    /// as the stateless rules' are, so the caller applies both the same way.
    pub fn check_paragraph(&self, block_before: &str) -> Option<Fired> {
        let chars: Vec<char> = block_before.chars().collect();
        let last = *chars.last()?;

        // ── A dialogue dash opens the paragraph ──────────────────────────────
        //
        // `- ` and nothing else before it. Deliberately the whole paragraph so
        // far: a hyphen anywhere else is a hyphen, and a rule that fired on
        // "well - " mid-line would mangle ordinary prose.
        if self.flags.dialogue_marker
            && let Some(dash) = self.ruleset.dialogue_dash
            && chars.len() == 2
            && chars[0] == '-'
            && last == ' '
        {
            return Some(fired(2, format!("{dash}\u{00A0}"), "- "));
        }

        // ── Spanish opens its questions and exclamations ─────────────────────
        //
        // Not behind a flag, for the same reason Arabic's mirrored marks are
        // not: writing `¿` is what writing the language *is*. Omitting it is a
        // spelling error in Spanish, not a stylistic choice — so the switch that
        // would turn it off is the language selector.
        if uses_inverted_marks(&self.locale)
            && let Some(&(_, opening)) = SPANISH_INVERTED.iter().find(|(c, _)| *c == last)
        {
            let start = clause_start(&chars[..chars.len() - 1]);
            let clause: String = chars[start..].iter().collect();
            // Already opened — the writer typed the mark somewhere in this
            // clause, or a prior fire did. `contains`, not `starts_with`: a `¿`
            // the writer placed mid-clause (`Es ¿que?`) is just as much an
            // existing mark, and prepending a second would stack `¿Es ¿que?`.
            if clause.contains(opening) {
                return None;
            }
            // An empty clause is a bare `?` with nothing to ask; leave it.
            if clause.chars().count() <= 1 {
                return None;
            }
            // INSERT the one mark at the clause start rather than rewriting the
            // clause: a tail replace would re-emit the clause as a plain string
            // and flatten any bold/italic run inside it. `prepend` leaves the
            // surrounding text byte-for-byte intact and the caret at the `?`.
            let offset_back = chars.len() - start;
            return Some(fired_prepend(offset_back, opening.to_string()));
        }

        // ── A quotation inside a quotation switches to the inner marks ────────
        //
        // The paragraph state the stateless quote rule cannot see: how deeply
        // nested this `"` is. French opens with `«` at the top level and `“`
        // inside one; Russian opens `«` then `„`. The writer types `"` for both,
        // and the depth decides which mark it becomes.
        //
        // Only for locales whose *secondary* quotes are double-width — the
        // guillemet family. Where the inner mark is a single curly quote
        // (English `‘…’`, Dutch, Swedish) two things rule this out: the closing
        // single quote is the apostrophe glyph, so counting depth from the
        // paragraph would be corrupted by every elision and possessive; and the
        // convention there is that the writer chooses double vs single with
        // their own key, not that a typed `"` silently becomes a `’`. Those
        // locales keep the stateless rule, which curls each `"` by context and
        // never touches depth. See [`nests_with_double_key`].
        if self.flags.quotes && last == '"' && self.can_nest() {
            let before_the_quote = &chars[..chars.len() - 1];
            let opening = opens_after(before_the_quote.last().copied());
            let depth = self.quote_depth(before_the_quote);
            let glyph = if opening {
                // Alternate outer/inner by depth: primary at an even count of
                // open quotations, secondary at an odd one.
                self.system_at(depth).open()
            } else {
                // Close the innermost open level — the one opened at `depth - 1`.
                self.system_at(depth.saturating_sub(1)).close()
            };
            return Some(fired(1, self.spaced_quote(glyph), "\""));
        }

        None
    }

    /// A quote glyph as a string, wrapping a guillemet in its French inner
    /// no-break space (`«` → `«\u{202F}`, `»` → `\u{202F}»`) where the locale
    /// calls for it. Every other glyph, and every non-French locale, is
    /// untouched.
    fn spaced_quote(&self, glyph: char) -> String {
        if self.guillemet_spacing && glyph == LAQUO {
            format!("{LAQUO}{NNBSP}")
        } else if self.guillemet_spacing && glyph == RAQUO {
            format!("{NNBSP}{RAQUO}")
        } else {
            glyph.to_string()
        }
    }

    /// Whether this engine may switch quote marks by nesting depth.
    ///
    /// Requires the locale's inner mark to be double-width
    /// ([`nests_with_double_key`]) **and** the four glyphs in play — the
    /// effective primary's open/close and the secondary's open/close — to be
    /// all distinct, so [`quote_depth`](Self::quote_depth) can tell an open from
    /// a close by glyph alone. A house-style override can break that
    /// distinctness (Low-high on French makes the primary's close `“` collide
    /// with the secondary's open `“`); when it does, nesting is declined and the
    /// stateless rule handles `"` with the effective primary, which is safe.
    fn can_nest(&self) -> bool {
        let sec = self.ruleset.secondary_quotes;
        if !nests_with_double_key(sec) {
            return false;
        }
        let pri = self.quotes;
        let g = [pri.open(), pri.close(), sec.open(), sec.close()];
        g.iter()
            .enumerate()
            .all(|(i, a)| g[i + 1..].iter().all(|b| a != b))
    }

    /// Which quote system applies at nesting `depth` — the effective primary
    /// (house-style aware) at an even depth, the locale's secondary at an odd
    /// one, alternating for deeper nesting the way every word processor does.
    fn system_at(&self, depth: usize) -> QuoteSystem {
        if depth.is_multiple_of(2) {
            self.quotes
        } else {
            self.ruleset.secondary_quotes
        }
    }

    /// How many quotations are open at the end of `before` — the nesting depth a
    /// newly-typed `"` sits at.
    ///
    /// Counts this locale's four quote glyphs: an opening mark deepens, a
    /// closing mark surfaces (saturating at 0, so a stray close cannot drive it
    /// negative). Sound only because [`can_nest`](Self::can_nest) guarantees the
    /// four are distinct and none is the apostrophe.
    ///
    /// `before` is bounded to the current paragraph by the caller, so depth
    /// counts from the paragraph start, not the document's. That is deliberate:
    /// a French or Spanish quotation running across paragraphs re-opens each
    /// paragraph with the outer mark by convention, which is exactly what a
    /// per-paragraph reset produces. Nested depth is not carried across a
    /// paragraph break — the cost of the cheap, paragraph-local read, and
    /// correct for the common single-level case.
    fn quote_depth(&self, before: &[char]) -> usize {
        let pri = self.quotes;
        let sec = self.ruleset.secondary_quotes;
        let mut depth: usize = 0;
        for &c in before {
            if c == pri.open() || c == sec.open() {
                depth += 1;
            } else if c == pri.close() || c == sec.close() {
                depth = depth.saturating_sub(1);
            }
        }
        depth
    }
}

/// Whether a quotation *opens* after `prev` (the character immediately before
/// the `"`): at the very start, or after whitespace, an opening bracket, or a
/// dash. The single source of truth for both the stateless and the nested quote
/// paths, which must agree on it.
fn opens_after(prev: Option<char>) -> bool {
    match prev {
        None => true,
        Some(c) => {
            c.is_whitespace() || matches!(c, '(' | '[' | '{' | EM_DASH | EN_DASH | '-' | LAQUO)
        }
    }
}

/// Whether the locale sets a thin no-break space inside its guillemets —
/// `« mot »` rather than `«mot»`. French practice (and Swiss French); no other
/// guillemet locale does it, so this is keyed on the French language subtag.
///
/// Reachable from the settings panes so their live sample can show the same
/// spacing the engine produces, rather than a bare `«…»` that lies about French.
pub fn uses_guillemet_inner_spacing(tag: &str) -> bool {
    tag.split(['-', '_'])
        .next()
        .unwrap_or("")
        .eq_ignore_ascii_case("fr")
}

/// Whether typing `"` inside a quotation should switch to `secondary` — i.e.
/// whether the locale's inner marks are double-width and so reached by the same
/// key as the outer ones.
///
/// True for the guillemet family, whose inner mark is a curly double `“…”` or a
/// low-high double `„…“`; false where the inner mark is a single curly quote
/// (`‘…’`), which the writer types with `'` and whose closing glyph is the
/// apostrophe — the two reasons nesting is unsafe there, spelled out at the call
/// site.
fn nests_with_double_key(secondary: QuoteSystem) -> bool {
    matches!(secondary, QuoteSystem::Paired { open, .. } if is_double_width_open(open))
}

/// Whether `c` is an opening quotation mark a writer reaches by typing `"`.
fn is_double_width_open(c: char) -> bool {
    matches!(c, LEFT_DOUBLE | LOW_DOUBLE | LAQUO)
}

fn fired(replace_chars: usize, replacement: String, typed: &str) -> Fired {
    Fired {
        replace_chars,
        replacement,
        typed: typed.to_string(),
        prepend: false,
    }
}

/// A substitution that *inserts* `mark` at `offset_back` characters before the
/// caret, removing nothing — see [`Fired::prepend`].
fn fired_prepend(offset_back: usize, mark: String) -> Fired {
    Fired {
        replace_chars: offset_back,
        replacement: mark,
        typed: String::new(),
        prepend: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Only the tests assert on this one directly.
    use crate::typography::RIGHT_DOUBLE;

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
        assert_eq!(
            ruleset_for("de-AT").tag,
            "de",
            "Austria uses the German row"
        );
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

    /// The design boundary of the nesting rule, stated as data: exactly the
    /// locales whose *inner* quotation mark is double-width nest from a typed
    /// `"`. The rest keep their single-curly inner mark on the `'` key, where
    /// swapping it automatically would collide with the apostrophe.
    #[test]
    fn only_double_width_secondaries_nest_from_the_double_key() {
        for tag in ["fr", "ru", "es", "it", "pt", "ca", "ar"] {
            assert!(
                nests_with_double_key(ruleset_for(tag).secondary_quotes),
                "{tag} has a double-width inner mark and should nest"
            );
        }
        for tag in ["en", "de", "de-CH", "pl", "nl", "pt-BR", "sv", "tr"] {
            assert!(
                !nests_with_double_key(ruleset_for(tag).secondary_quotes),
                "{tag}'s inner mark is single/symmetric and must NOT nest from `\\\"`"
            );
        }
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
        assert!(
            en().check("wait..").is_none(),
            "two dots are not an ellipsis"
        );
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
        // French sets a narrow no-break space inside its guillemets: « mot ».
        assert_eq!(f.replacement, "\u{00AB}\u{202F}");
    }

    /// The other guillemet locales do NOT take the French inner space.
    #[test]
    fn non_french_guillemets_have_no_inner_space() {
        for tag in ["es-ES", "it-IT", "pt-PT", "ru-RU", "ca"] {
            let e = TypographyEngine::new(tag, SmartPunctuationFlags::default());
            let f = e.check("dice \"").expect("fires");
            assert_eq!(f.replacement, "\u{00AB}", "{tag} must not add a space");
        }
    }

    /// A Low-high house style on French makes the primary's closing glyph `“`
    /// collide with the secondary's opening glyph `“`, so nesting is declined
    /// (rather than miscounting depth) and `"` falls back to the override
    /// primary via the stateless rule.
    #[test]
    fn a_low_high_override_on_french_declines_nesting() {
        let e = TypographyEngine::new(
            "fr-FR",
            SmartPunctuationFlags {
                quote_style: QuoteStyle::LowHigh,
                ..SmartPunctuationFlags::default()
            },
        );
        // Even inside an open low-high quote, `check_paragraph` returns None —
        // it will not risk a depth count over colliding glyphs.
        assert!(e.check_paragraph("\u{201E}mot \"").is_none());
        // So the stateless rule opens/closes with the low-high pair, never the
        // curly-double secondary.
        assert_eq!(e.check("il dit \"").expect("open").replacement, "\u{201E}");
        assert_eq!(e.check("mot\"").expect("close").replacement, "\u{201C}");
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
            dialogue_marker: false,
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
                dialogue_marker: false,
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
