// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The trigger-matching engine: given the text immediately behind the caret,
//! decide whether a lexicon rule just fired and what it expands to.
//!
//! Pure Rust — no widget, no backend, no `#[cfg]`. Everything about *when* to
//! ask and *how* to apply the answer lives in [`session`](super::session); this
//! module only answers "does this text end in a fired trigger?".
//!
//! ## How a trigger fires
//!
//! A rule fires the moment the writer types a **delimiter** after the trigger —
//! a space, a newline, a comma, a full stop. That delimiter is the last
//! character of the window handed to [`TextReplacementEngine::check`]; the
//! trigger is what sits immediately before it, and the character before *that*
//! must itself be a delimiter (or the document must start there), so `btw`
//! expands in "say btw " but not in "fumbl ".
//!
//! ## What counts as a delimiter
//!
//! Anything that is not [`char::is_alphanumeric`]. Defined by exclusion rather
//! than as a punctuation list so it is correct for every script the app
//! supports rather than for ASCII: `is_alphanumeric` is Unicode-aware, so
//! Arabic, Japanese, Greek, Cyrillic and Devanagari word characters are never
//! mistaken for delimiters, and their punctuation (、。؟ …) is never mistaken
//! for a word character.
//!
//! A consequence worth stating: the apostrophe is a delimiter. That is what
//! lets a French elision fire a rule ("l'ajd " expands `ajd`), at the cost of
//! an English contraction being two words for matching purposes ("don't" is
//! `don` + `t`). Only a pathological one-or-two-letter trigger notices, and the
//! French case is the common one.
//!
//! ## Case propagation
//!
//! Triggers match case-insensitively, and the replacement follows the case the
//! writer typed — Word's three-state rule. See [`apply_case`].

use skribisto_model::casing;

use crate::models::TextReplacementRuleRow;

/// A fired rule: what to remove and what to put in its place.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fired {
    /// How many **characters** of typed text the trigger occupies. The caller
    /// removes exactly this many, ending one character before the caret (the
    /// delimiter that fired the rule stays).
    pub trigger_chars: usize,
    /// The trigger exactly as the writer typed it — what a backspace-revert
    /// puts back, so the writer gets their own spelling rather than the
    /// lexicon's.
    pub typed: String,
    /// The replacement with the typed case propagated onto it.
    pub replacement: String,
}

/// One enabled rule, pre-lowercased for matching.
#[derive(Clone, Debug)]
struct Rule {
    /// Lowercased trigger — what the suffix match compares against.
    key: String,
    /// Character count of `key`, cached because the match needs it per
    /// candidate and `chars().count()` is O(n).
    key_chars: usize,
    /// The replacement as the writer stored it (its own casing is preserved
    /// when it carries any, see [`apply_case`]).
    replacement: String,
}

/// The compiled lexicon: enabled rules, longest trigger first.
#[derive(Clone, Debug, Default)]
pub struct TextReplacementEngine {
    /// Sorted by descending `key_chars`, so the first candidate that matches
    /// and clears the delimiter check is the longest one — "longest match
    /// wins" falls out of the iteration order.
    rules: Vec<Rule>,
    /// Character count of the longest trigger; 0 when there are no rules.
    longest: usize,
    /// The BCP-47 tag every case operation is resolved against.
    ///
    /// Turkish and Azeri need it: they treat the dotted and dotless I as
    /// different letters, so a default `to_lowercase` both mis-folds the trigger
    /// key (leaving a combining dot that never compares equal to a typed `i`)
    /// and mis-cases the replacement. See [`skribisto_model::casing`]. Empty is
    /// "no locale", which is exactly the default mapping.
    locale: String,
}

impl TextReplacementEngine {
    /// Compile the enabled rules of a lexicon. Disabled rows and blank
    /// triggers are dropped here rather than skipped per keystroke.
    ///
    /// A trigger that collides case-insensitively with an earlier one is
    /// dropped: two rules for the same trigger could never both fire, and the
    /// settings pane already refuses to create the second — this only matters
    /// for a lexicon that arrived from disk (a hand-edited `.skrib`, or an
    /// import that predates the check).
    pub fn from_rules(rows: &[TextReplacementRuleRow]) -> Self {
        Self::from_rules_for_locale(rows, "")
    }

    /// As [`from_rules`](Self::from_rules), for a document written in `locale`
    /// (a BCP-47 tag; `""` for none). The locale decides how triggers are folded
    /// for matching and how case is propagated onto the replacement.
    pub fn from_rules_for_locale(rows: &[TextReplacementRuleRow], locale: &str) -> Self {
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut rules: Vec<Rule> = Vec::new();
        for row in rows {
            if !row.enabled {
                continue;
            }
            let trimmed = row.trigger.trim();
            if trimmed.is_empty() {
                continue;
            }
            let key = casing::fold_key(trimmed, locale);
            if !seen.insert(key.clone()) {
                continue;
            }
            rules.push(Rule {
                key_chars: key.chars().count(),
                key,
                replacement: row.replacement.clone(),
            });
        }
        rules.sort_by(|a, b| b.key_chars.cmp(&a.key_chars).then_with(|| a.key.cmp(&b.key)));
        let longest = rules.first().map(|r| r.key_chars).unwrap_or(0);
        Self { rules, longest, locale: locale.to_string() }
    }

    /// Whether there is anything to match at all. The session skips the whole
    /// read-the-text-behind-the-caret step when this is true, so an empty or
    /// all-disabled lexicon costs nothing per keystroke.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// How many characters of text before the caret [`check`](Self::check)
    /// needs: the longest trigger, plus the delimiter that fires it, plus the
    /// one character before the trigger whose delimiter-ness proves the
    /// trigger starts a word.
    ///
    /// Reading exactly this much is what makes the empty-prefix case in
    /// `check` unambiguous — see its documentation.
    pub fn window_chars(&self) -> usize {
        if self.rules.is_empty() {
            0
        } else {
            self.longest + 2
        }
    }

    /// Test the text behind the caret for a fired rule.
    ///
    /// `window` MUST be the last [`window_chars`](Self::window_chars)
    /// characters before the caret — or the entire document prefix when the
    /// document is shorter than that. That contract is what lets an empty
    /// prefix be read as "the trigger starts the document": with a full-length
    /// window there is always at least one character left over after the
    /// longest possible trigger and its delimiter, so nothing but a genuine
    /// document start can leave the prefix empty.
    pub fn check(&self, window: &str) -> Option<Fired> {
        if self.rules.is_empty() {
            return None;
        }
        let chars: Vec<char> = window.chars().collect();
        // The rule fires on the delimiter the writer just typed, which is the
        // last thing in the window. Anything else means they are still inside a
        // word, so there is nothing to expand yet.
        if !is_delimiter(*chars.last()?) {
            return None;
        }
        let typed = &chars[..chars.len() - 1];

        for rule in &self.rules {
            if rule.key_chars > typed.len() {
                continue;
            }
            let start = typed.len() - rule.key_chars;
            let candidate: String = typed[start..].iter().collect();
            if casing::fold_key(&candidate, &self.locale) != rule.key {
                continue;
            }
            // The trigger has to START a word too, or "fumbl " would expand a
            // "mbl" rule. Keep looking rather than bailing out: a shorter rule
            // may still start on a delimiter here.
            if start > 0 && !is_delimiter(typed[start - 1]) {
                continue;
            }
            return Some(Fired {
                trigger_chars: rule.key_chars,
                replacement: apply_case(&rule.replacement, &candidate, &self.locale),
                typed: candidate,
            });
        }
        None
    }
}

/// Whether `c` ends a word for matching purposes.
///
/// Defined by exclusion so it holds for every script — see the module
/// documentation for why, and for what it means for the apostrophe.
fn is_delimiter(c: char) -> bool {
    !c.is_alphanumeric()
}

/// Propagate the case the writer typed onto the replacement — Word's rule.
///
/// A replacement that carries **any** uppercase of its own is used verbatim:
/// the writer wrote "by the way" (or "PhD", or "iPhone") deliberately,
/// and re-casing it from the trigger would destroy that. Only an all-lowercase
/// replacement follows the trigger, and then only into the three states a
/// writer can express by typing: lowercase, Capitalized, ALL CAPS. Anything
/// else they typed (`bTw`) carries no clear intent, so the replacement is left
/// alone.
///
/// Casing uses Rust's default Unicode mappings, which are locale-independent.
/// That is correct everywhere except Turkish, where `I` should lowercase to `ı`
/// rather than `i`; locale-tailored casing belongs with the typography rules
/// rather than here, where it would need a locale this engine is not given.
fn apply_case(replacement: &str, typed: &str, locale: &str) -> String {
    if replacement.chars().any(char::is_uppercase) {
        return replacement.to_string();
    }
    match classify(typed) {
        TypedCase::Capitalized => casing::capitalize_first(replacement, locale),
        TypedCase::Upper => casing::to_uppercase(replacement, locale),
        TypedCase::AsTyped => replacement.to_string(),
    }
}

/// The three case shapes a writer can express, plus "no clear intent".
enum TypedCase {
    /// All lowercase, mixed in a way that says nothing, or no cased letters at
    /// all (a `--` trigger) — the replacement is left as stored.
    AsTyped,
    /// First letter capitalized, the rest lowercase.
    Capitalized,
    /// Every letter uppercase, and at least two of them: one uppercase letter
    /// reads as Capitalized, not as shouting.
    Upper,
}

fn classify(typed: &str) -> TypedCase {
    let cased: Vec<char> = typed.chars().filter(|c| c.is_alphabetic()).collect();
    let Some(first) = cased.first() else {
        return TypedCase::AsTyped;
    };
    if cased.iter().all(|c| c.is_lowercase()) {
        return TypedCase::AsTyped;
    }
    if cased.len() >= 2 && cased.iter().all(|c| c.is_uppercase()) {
        return TypedCase::Upper;
    }
    if first.is_uppercase() && cased[1..].iter().all(|c| c.is_lowercase()) {
        return TypedCase::Capitalized;
    }
    TypedCase::AsTyped
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(trigger: &str, replacement: &str) -> TextReplacementRuleRow {
        TextReplacementRuleRow {
            id: 0,
            trigger: trigger.into(),
            replacement: replacement.into(),
            enabled: true,
        }
    }

    fn engine(rows: &[TextReplacementRuleRow]) -> TextReplacementEngine {
        TextReplacementEngine::from_rules(rows)
    }

    /// Build the window the session would hand `check` for `text` — the last
    /// `window_chars()` characters, or all of it when it is shorter. Every test
    /// goes through this so none of them can accidentally violate the window
    /// contract that the empty-prefix rule depends on.
    fn window(e: &TextReplacementEngine, text: &str) -> String {
        let chars: Vec<char> = text.chars().collect();
        let want = e.window_chars();
        let start = chars.len().saturating_sub(want);
        chars[start..].iter().collect()
    }

    fn fire(e: &TextReplacementEngine, text: &str) -> Option<Fired> {
        e.check(&window(e, text))
    }

    #[test]
    fn an_exact_trigger_followed_by_a_space_fires() {
        let e = engine(&[rule("btw", "by the way")]);
        let f = fire(&e, "I saw btw ").expect("the rule must fire");
        assert_eq!(f.replacement, "by the way");
        assert_eq!(f.trigger_chars, 3);
        assert_eq!(f.typed, "btw");
    }

    /// The delimiter is what fires the rule, so mid-word typing must stay quiet
    /// — otherwise every keystroke of "btw" would expand as it was typed.
    #[test]
    fn a_trigger_not_yet_followed_by_a_delimiter_does_not_fire() {
        let e = engine(&[rule("btw", "by the way")]);
        assert_eq!(fire(&e, "I saw btw"), None);
    }

    /// The whole point of the start-of-word check: a trigger that happens to be
    /// the tail of a longer word must not expand.
    #[test]
    fn a_trigger_that_is_only_the_tail_of_a_word_does_not_fire() {
        let e = engine(&[rule("mbl", "Mumble")]);
        assert_eq!(fire(&e, "I fumbl "), None);
    }

    #[test]
    fn any_punctuation_fires_the_rule_not_just_a_space() {
        let e = engine(&[rule("btw", "by the way")]);
        for text in ["btw,", "btw.", "btw!", "btw\n", "btw\t", "btw)", "btw—"] {
            assert!(
                fire(&e, &format!("I saw {text}")).is_some(),
                "{text:?} must fire the rule"
            );
        }
    }

    // ── Case propagation ────────────────────────────────────────────────────

    #[test]
    fn a_lowercase_replacement_follows_the_typed_case() {
        let e = engine(&[rule("teh", "the")]);
        assert_eq!(fire(&e, "teh ").unwrap().replacement, "the");
        assert_eq!(fire(&e, "Teh ").unwrap().replacement, "The");
        assert_eq!(fire(&e, "TEH ").unwrap().replacement, "THE");
    }

    /// A replacement the writer capitalised themselves is theirs — re-casing it
    /// from the trigger would turn "PhD" into "Phdonagall".
    #[test]
    fn a_replacement_with_its_own_uppercase_is_used_verbatim() {
        let e = engine(&[rule("phd", "PhD")]);
        assert_eq!(fire(&e, "phd ").unwrap().replacement, "PhD");
        assert_eq!(fire(&e, "PHD ").unwrap().replacement, "PhD");
        assert_eq!(fire(&e, "Phd ").unwrap().replacement, "PhD");
    }

    /// A single uppercase letter reads as Capitalized rather than as shouting —
    /// otherwise a one-letter trigger could never produce a capitalised word.
    #[test]
    fn a_one_letter_uppercase_trigger_capitalizes_rather_than_shouts() {
        let e = engine(&[rule("b", "by the way")]);
        assert_eq!(fire(&e, "B ").unwrap().replacement, "By the way");
    }

    /// Case the writer cannot have meant carries no instruction.
    #[test]
    fn a_mixed_case_trigger_leaves_the_replacement_alone() {
        let e = engine(&[rule("btw", "by the way")]);
        assert_eq!(fire(&e, "bTw ").unwrap().replacement, "by the way");
    }

    /// A trigger with no letters at all has no case to propagate.
    #[test]
    fn a_caseless_trigger_leaves_the_replacement_alone() {
        let e = engine(&[rule("--", "—")]);
        assert_eq!(fire(&e, "a -- ").unwrap().replacement, "—");
    }

    /// Unicode expansion still applies to the replacement — one character can
    /// uppercase into two. (`capitalize_first` itself now lives in
    /// `skribisto_model::casing` and is tested exhaustively there; this is the
    /// engine-level proof that the path reaches it.)
    #[test]
    fn capitalizing_handles_a_character_that_uppercases_into_two() {
        let e = engine(&[rule("bt", "ßeta")]);
        assert_eq!(fire(&e, "Bt ").unwrap().replacement, "SSeta");
    }

    // ── Turkish: the dotted/dotless I ───────────────────────────────────────

    /// Turkish treats the dotted and dotless I as different letters, so a
    /// default `to_lowercase` leaves a combining dot on `İ` that never compares
    /// equal to a typed `i` — the trigger would silently never match. The
    /// engine folds through the locale to close that.
    #[test]
    fn a_turkish_trigger_matches_across_both_capital_is() {
        let e = TextReplacementEngine::from_rules_for_locale(
            &[rule("ist", "İstanbul")],
            "tr-TR",
        );
        assert_eq!(fire(&e, "ist ").unwrap().replacement, "İstanbul");
        assert_eq!(
            fire(&e, "İST ").unwrap().replacement,
            "İstanbul",
            "the dotted capital must fold to the same key as `i`"
        );
    }

    /// Case propagation follows the locale too: an all-lowercase replacement
    /// shouted in Turkish gains DOTTED capitals.
    ///
    /// And the negative half is the more interesting one. Shouting `ii` in
    /// Turkish is `İİ`, not `II` — `II` is the uppercase of the *dotless* `ıı`,
    /// a different word — so `II` must NOT fire this rule. Under the default
    /// mapping it would, which is precisely the silent mis-expansion the
    /// tailoring exists to prevent.
    #[test]
    fn turkish_case_propagation_uses_the_dotted_capital() {
        let e = TextReplacementEngine::from_rules_for_locale(&[rule("ii", "iyi")], "tr");
        assert_eq!(fire(&e, "İİ ").unwrap().replacement, "İYİ");
        assert_eq!(fire(&e, "İi ").unwrap().replacement, "İyi");
        assert_eq!(
            fire(&e, "II "),
            None,
            "`II` is the uppercase of `ıı` in Turkish — a different word"
        );
    }

    /// And the tailoring must NOT leak anywhere else — applying it to English
    /// would be its own bug.
    #[test]
    fn the_turkish_tailoring_does_not_leak_into_other_locales() {
        let e = TextReplacementEngine::from_rules_for_locale(&[rule("ii", "iyi")], "en-US");
        assert_eq!(fire(&e, "II ").unwrap().replacement, "IYI");
        let d = engine(&[rule("ii", "iyi")]);
        assert_eq!(fire(&d, "II ").unwrap().replacement, "IYI", "no locale = default");
    }

    // ── Longest match wins ──────────────────────────────────────────────────

    #[test]
    fn the_longest_matching_trigger_wins() {
        // Both are viable here: "e g" starts after the space before "e", and "g"
        // starts after the space before it, so each on its own would fire. The
        // longer must win.
        let e = engine(&[rule("e g", "for example"), rule("g", "gram")]);
        assert_eq!(fire(&e, "an e g ").unwrap().replacement, "for example");
    }

    /// The longest candidate failing the start-of-word check must not stop a
    /// shorter one that passes it — a `continue`, not a bail-out.
    #[test]
    fn a_shorter_rule_still_fires_when_the_longest_fails_the_word_start_check() {
        // In "xa b " the longer rule "a b" DOES match the text behind the caret,
        // but starts mid-word (preceded by "x"), so it must be skipped rather
        // than end the search — "b" starts right after the space and fires.
        let e = engine(&[rule("a b", "alpha beta"), rule("b", "beta")]);
        assert_eq!(fire(&e, "xa b ").unwrap().replacement, "beta");
    }

    // ── Document-start edges ────────────────────────────────────────────────

    /// A trigger typed as the very first thing in the document has no character
    /// before it — that must read as "starts a word", not as a failed check.
    #[test]
    fn a_trigger_at_the_very_start_of_the_document_fires() {
        let e = engine(&[rule("btw", "by the way")]);
        let f = fire(&e, "btw ").expect("a trigger at position 0 must fire");
        assert_eq!(f.replacement, "by the way");
    }

    /// The window is shorter than `window_chars()` near the start of a
    /// document; the match must not depend on it being full.
    #[test]
    fn a_window_shorter_than_requested_still_matches() {
        let e = engine(&[rule("verylongtrigger", "x"), rule("btw", "by the way")]);
        assert!(e.window_chars() > "btw ".chars().count());
        assert_eq!(fire(&e, "btw ").unwrap().replacement, "by the way");
    }

    // ── Compilation ─────────────────────────────────────────────────────────

    #[test]
    fn a_disabled_rule_never_fires() {
        let mut row = rule("btw", "by the way");
        row.enabled = false;
        let e = engine(&[row]);
        assert!(e.is_empty());
        assert_eq!(fire(&e, "say btw "), None);
    }

    #[test]
    fn a_blank_trigger_is_dropped_rather_than_matching_everything() {
        let e = engine(&[rule("   ", "boom")]);
        assert!(e.is_empty());
        assert_eq!(e.window_chars(), 0);
    }

    /// A lexicon that arrived from disk can hold a case-insensitive duplicate
    /// the settings pane would have refused; the first one wins deterministically.
    #[test]
    fn a_case_insensitive_duplicate_trigger_is_dropped() {
        let e = engine(&[rule("btw", "by the way"), rule("BTW", "Different")]);
        assert_eq!(fire(&e, "say btw ").unwrap().replacement, "by the way");
    }

    #[test]
    fn an_empty_lexicon_asks_for_no_window_and_never_fires() {
        let e = engine(&[]);
        assert!(e.is_empty());
        assert_eq!(e.window_chars(), 0);
        assert_eq!(e.check(""), None);
        assert_eq!(e.check("anything at all "), None);
    }

    /// The trigger is trimmed when compiled, so a stored trigger with stray
    /// space still matches what the writer actually types.
    #[test]
    fn a_stored_trigger_is_matched_trimmed() {
        let e = engine(&[rule("  btw  ", "by the way")]);
        assert_eq!(fire(&e, "say btw ").unwrap().replacement, "by the way");
    }

    // ── Non-Latin scripts ───────────────────────────────────────────────────

    /// `is_delimiter` is defined by exclusion precisely so this works: Cyrillic
    /// and Greek letters are word characters, not delimiters.
    #[test]
    fn a_non_latin_trigger_fires_and_propagates_case() {
        let e = engine(&[rule("привет", "здравствуйте")]);
        assert_eq!(
            fire(&e, "привет ").unwrap().replacement,
            "здравствуйте"
        );
        assert_eq!(
            fire(&e, "Привет ").unwrap().replacement,
            "Здравствуйте"
        );
    }

    /// A CJK full stop is punctuation, so it fires the rule; a CJK ideograph is
    /// a word character, so it does not.
    #[test]
    fn cjk_punctuation_delimits_but_ideographs_do_not() {
        assert!(is_delimiter('。'));
        assert!(is_delimiter('、'));
        assert!(!is_delimiter('日'));
        assert!(!is_delimiter('ひ'));
    }

    /// Arabic script is right-to-left but the matching is purely by logical
    /// order, which is what the document stores — so nothing special is needed.
    #[test]
    fn an_arabic_trigger_fires() {
        let e = engine(&[rule("سلام", "السلام عليكم")]);
        assert_eq!(fire(&e, "سلام ").unwrap().replacement, "السلام عليكم");
        assert!(is_delimiter('؟'), "the Arabic question mark delimits");
    }

    /// The French elision case the apostrophe-is-a-delimiter choice is for.
    #[test]
    fn a_trigger_after_a_french_elision_fires() {
        let e = engine(&[rule("ajd", "aujourd’hui")]);
        assert_eq!(fire(&e, "l'ajd ").unwrap().replacement, "aujourd’hui");
        assert_eq!(fire(&e, "l’ajd ").unwrap().replacement, "aujourd’hui");
    }

    /// A multi-character trigger whose characters are multi-byte must count in
    /// characters, not bytes, or the caller would delete the wrong span.
    #[test]
    fn trigger_chars_counts_characters_not_bytes() {
        let e = engine(&[rule("café", "coffee")]);
        let f = fire(&e, "un café ").unwrap();
        assert_eq!(f.trigger_chars, 4, "é is one character but two bytes");
        assert_eq!(f.typed, "café");
    }

    /// A trigger may contain a space; nothing about the matching forbids it,
    /// and the delimiter-before check still holds at its start.
    #[test]
    fn a_multi_word_trigger_fires() {
        let e = engine(&[rule("e g", "for example")]);
        assert_eq!(fire(&e, "an e g ").unwrap().replacement, "for example");
    }

    /// An empty replacement is legal — it makes the rule delete the trigger.
    #[test]
    fn an_empty_replacement_is_allowed() {
        let e = engine(&[rule("xx", "")]);
        let f = fire(&e, "a xx ").expect("an erasing rule still fires");
        assert_eq!(f.replacement, "");
        assert_eq!(f.trigger_chars, 2);
    }
}
