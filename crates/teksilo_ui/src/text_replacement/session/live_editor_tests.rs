// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use frontend::AppContext;
use teksilo::core::widget_tree::WidgetTree;
use teksilo::prelude::SizeProposal;
use teksilo::text_document::TextDocument;
use teksilo::widgets::rich_text::{EditorHandle, RichTextEditor};

use super::*;
use crate::app_ids::AppIds;
use crate::models::TextReplacementRuleListModel;
use crate::singles::SingleWork;

/// A live editor over `text`, plus a session whose lexicon is the mock one
/// (`--` → `—`, `btw` → `by the way`, and a disabled `teh` → `the`) with the
/// project's master switch turned on.
///
/// The `WidgetTree` is returned and must be kept alive — the handle reads
/// the editor's state, and the tree owns the editor.
fn editor(
    text: &str,
) -> (
    TextDocument,
    EditorHandle,
    Rc<TextReplacementSession>,
    WidgetTree,
) {
    let doc = TextDocument::new();
    doc.set_plain_text(text).unwrap();
    let ed = RichTextEditor::editor(doc.clone());
    let handle = ed.handle();
    let mut tree = WidgetTree::new();
    tree.add(ed);
    tree.layout(SizeProposal::exact(600.0, 400.0));

    let ctx = Rc::new(AppContext::new());
    let work = SingleWork::new(ctx.clone());
    work.set_custom_replacement_rules_enabled(true);
    let ids = AppIds::new();
    let vm = TextReplacementRulesViewModel::new(
        TextReplacementRuleListModel::new(ctx, ids.clone()),
        work,
        ids,
    );
    let session = TextReplacementSession::new(vm);
    (doc, handle, session, tree)
}

/// Type `s` one character at a time at the caret, running the session after
/// each — the same cadence `on_change` delivers real typing in.
fn type_text(handle: &EditorHandle, doc: &TextDocument, session: &TextReplacementSession, s: &str) {
    // Seed the caret baseline before typing, which is what the real app
    // does for free: the frame-tick effect runs from the moment the editor
    // is built, so by the time anyone types, `tick` has already observed a
    // caret. Without this the FIRST character of each test is swallowed by
    // the caret-advanced gate — invisible to a multi-character lexicon
    // trigger, but fatal to a single-character punctuation rule.
    session.tick(handle, doc);
    for c in s.chars() {
        handle.insert_text(&c.to_string());
        session.tick(handle, doc);
    }
}

fn plain(doc: &TextDocument) -> String {
    doc.to_plain_text().unwrap_or_default()
}

/// The headline behaviour: typing the trigger and then a space expands it,
/// and the space the writer typed is still there afterwards.
#[test]
fn typing_a_trigger_then_a_space_expands_it() {
    let (doc, handle, session, _tree) = editor("");
    type_text(&handle, &doc, &session, "I saw btw ");
    assert_eq!(plain(&doc), "I saw by the way ");
    assert_eq!(
        handle.cursor_position(),
        "I saw by the way ".chars().count(),
        "the caret must end up after the delimiter, not before it"
    );
}

/// Still mid-word, nothing has fired — otherwise the expansion would happen
/// under the writer's fingers as they typed the trigger's last letter.
#[test]
fn typing_the_trigger_alone_does_not_expand() {
    let (doc, handle, session, _tree) = editor("");
    type_text(&handle, &doc, &session, "I saw btw");
    assert_eq!(plain(&doc), "I saw btw");
}

/// The word-start guard, against a real document.
#[test]
fn a_trigger_inside_a_longer_word_does_not_expand() {
    let (doc, handle, session, _tree) = editor("");
    type_text(&handle, &doc, &session, "a xbtw ");
    assert_eq!(plain(&doc), "a xbtw ");
}

/// Case propagation end to end.
#[test]
fn the_typed_case_reaches_the_document() {
    let (doc, handle, session, _tree) = editor("");
    // `--` → `—` has no letters, so it also proves a caseless rule fires.
    type_text(&handle, &doc, &session, "a -- ");
    assert_eq!(plain(&doc), "a — ");
}

/// A disabled rule must not fire even with the project switch on.
#[test]
fn a_disabled_rule_does_not_expand() {
    let (doc, handle, session, _tree) = editor("");
    type_text(&handle, &doc, &session, "teh ");
    assert_eq!(plain(&doc), "teh ");
}

/// The document's language must actually reach the matcher.
///
/// The mock lexicon ships `teh` → `the` (disabled) and `btw` → `by the way`.
/// Under `tr`, typing the SHOUTED form of `btw` is `BTW` either way — `b`,
/// `t` and `w` have no dotted-I problem — so this uses a rule that does:
/// it adds one, then checks that the Turkish fold matches where the default
/// fold would not.
#[test]
fn the_documents_locale_reaches_the_matcher() {
    let (doc, handle, session, _tree) = editor("");
    // A trigger whose uppercase differs between Turkish and the default.
    session.vm_for_test().create("ii", "iyi", true);
    session.set_locale("tr-TR");
    // `İİ` is the Turkish uppercase of `ii`; under the default fold it would
    // carry a combining dot and never match.
    type_text(&handle, &doc, &session, "İİ ");
    assert_eq!(plain(&doc), "İYİ ", "got {:?}", plain(&doc));
}

/// And the same input under a non-Turkish locale must NOT expand — `İİ` is
/// not the uppercase of `ii` anywhere else.
#[test]
fn a_non_turkish_locale_does_not_get_the_turkish_fold() {
    let (doc, handle, session, _tree) = editor("");
    session.vm_for_test().create("ii", "iyi", true);
    session.set_locale("en-US");
    type_text(&handle, &doc, &session, "İİ ");
    assert_eq!(plain(&doc), "İİ ", "got {:?}", plain(&doc));
}

/// Backspace immediately after an expansion puts the writer's own spelling
/// back — the escape hatch, driven through the real handle.
#[test]
fn backspace_right_after_an_expansion_reverts_it() {
    let (doc, handle, session, _tree) = editor("");
    type_text(&handle, &doc, &session, "I saw btw ");
    assert_eq!(plain(&doc), "I saw by the way ");

    // The delimiter the expansion re-added is what Backspace removes.
    let end = handle.cursor_position();
    handle.replace_range(end - 1, end, "");
    session.tick(&handle, &doc);
    assert_eq!(plain(&doc), "I saw btw");
}

/// And having reverted, re-typing the delimiter must NOT expand it again —
/// the writer already said no. (The one-shot suppression.)
#[test]
fn re_typing_the_delimiter_after_a_revert_does_not_re_expand() {
    let (doc, handle, session, _tree) = editor("");
    type_text(&handle, &doc, &session, "I saw btw ");
    let end = handle.cursor_position();
    handle.replace_range(end - 1, end, "");
    session.tick(&handle, &doc);
    assert_eq!(plain(&doc), "I saw btw");

    type_text(&handle, &doc, &session, " ");
    assert_eq!(
        plain(&doc),
        "I saw btw ",
        "the rule the writer just rejected must not fire again at that spot"
    );
}

/// The suppression is one-shot: a *different* occurrence later still fires,
/// or rejecting one expansion would disable the rule for the session.
#[test]
fn a_later_occurrence_still_expands_after_a_revert() {
    let (doc, handle, session, _tree) = editor("");
    type_text(&handle, &doc, &session, "I saw btw ");
    let end = handle.cursor_position();
    handle.replace_range(end - 1, end, "");
    session.tick(&handle, &doc);
    type_text(&handle, &doc, &session, " and btw ");
    assert!(
        plain(&doc).ends_with("and by the way "),
        "got {:?}",
        plain(&doc)
    );
}

/// Deleting text that happens to leave a fired trigger behind the caret is
/// not typing, and must not expand — the caret-advanced gate.
#[test]
fn a_deletion_that_exposes_a_trigger_does_not_expand() {
    let (doc, handle, session, _tree) = editor("say btw x ");
    // Put the caret after the "x " and delete the "x", leaving "say btw  ".
    handle.select_range(9, 9);
    session.tick(&handle, &doc); // seed the caret baseline
    handle.replace_range(8, 9, "");
    session.tick(&handle, &doc);
    assert!(
        !plain(&doc).contains("by the way"),
        "a deletion must not fire a rule, got {:?}",
        plain(&doc)
    );
}

// ── Typography, through the same live editor ─────────────────────────────

/// Turn on the punctuation rules for `locale` on an existing session.
fn punctuate(session: &TextReplacementSession, locale: &str) {
    session.set_locale(locale);
    session.set_punctuation(Some(SmartPunctuationFlags::default()));
}

#[test]
fn typing_three_dots_produces_an_ellipsis() {
    let (doc, handle, session, _tree) = editor("");
    punctuate(&session, "en-US");
    type_text(&handle, &doc, &session, "wait...");
    assert_eq!(plain(&doc), "wait…");
}

/// The chained case, through a real document: two hyphens become an en dash
/// and the third has to upgrade it rather than sit beside it.
#[test]
fn typing_three_hyphens_climbs_to_an_em_dash() {
    let (doc, handle, session, _tree) = editor("");
    punctuate(&session, "en-US");
    type_text(&handle, &doc, &session, "a--");
    assert_eq!(plain(&doc), "a–", "two hyphens make an en dash");
    type_text(&handle, &doc, &session, "-");
    assert_eq!(plain(&doc), "a—", "the third upgrades it");
}

#[test]
fn quotes_curl_by_side_against_a_real_document() {
    let (doc, handle, session, _tree) = editor("");
    punctuate(&session, "en-US");
    type_text(&handle, &doc, &session, "he said \"yes\"");
    assert_eq!(plain(&doc), "he said “yes”");
}

/// French takes guillemets and a narrow no-break space before its question
/// mark — both rules on one line of prose.
#[test]
fn french_gets_its_guillemets_and_its_thin_space() {
    let (doc, handle, session, _tree) = editor("");
    session.set_locale("fr-FR");
    session.set_punctuation(Some(SmartPunctuationFlags {
        pre_punctuation_spacing: true,
        ..SmartPunctuationFlags::default()
    }));
    type_text(&handle, &doc, &session, "\"Quoi ?");
    // « takes its inner thin space too, and ? takes its space-before.
    assert_eq!(plain(&doc), "«\u{202F}Quoi\u{202F}?");
}

/// Arabic mirroring, end to end.
#[test]
fn arabic_punctuation_is_mirrored_in_the_document() {
    let (doc, handle, session, _tree) = editor("");
    punctuate(&session, "ar");
    type_text(&handle, &doc, &session, "كيف?");
    assert_eq!(plain(&doc), "كيف؟");
}

/// The reason mirroring is gated on script rather than direction — Hebrew is
/// right-to-left and keeps its ASCII question mark.
#[test]
fn hebrew_keeps_its_ascii_question_mark_in_the_document() {
    let (doc, handle, session, _tree) = editor("");
    punctuate(&session, "he-IL");
    type_text(&handle, &doc, &session, "מה?");
    assert_eq!(plain(&doc), "מה?");
}

/// Both engines can match the same keystroke. The lexicon wins, because it
/// is the writer's own instruction where typography is a house convention —
/// and only one fires, or Backspace would undo the wrong half.
#[test]
fn the_lexicon_wins_a_keystroke_both_engines_could_claim() {
    let (doc, handle, session, _tree) = editor("");
    punctuate(&session, "en-US");
    // `btw` + `.` ends a lexicon trigger; the `.` could also begin an
    // ellipsis. Only the expansion may happen.
    type_text(&handle, &doc, &session, "I saw btw.");
    assert_eq!(plain(&doc), "I saw by the way.");
}

/// Typography must not disturb the lexicon's own backspace-revert.
#[test]
fn backspace_revert_still_works_with_punctuation_on() {
    let (doc, handle, session, _tree) = editor("");
    punctuate(&session, "en-US");
    type_text(&handle, &doc, &session, "I saw btw ");
    assert_eq!(plain(&doc), "I saw by the way ");
    let end = handle.cursor_position();
    handle.replace_range(end - 1, end, "");
    session.tick(&handle, &doc);
    assert_eq!(plain(&doc), "I saw btw");
}

/// A punctuation substitution leaves NO pending revert: one glyph replaced
/// one glyph, so Backspace must delete it like any character rather than
/// restoring the literal and needing a second press.
#[test]
fn backspace_after_a_substitution_just_deletes_it() {
    let (doc, handle, session, _tree) = editor("");
    punctuate(&session, "en-US");
    // A space before it, so the quote opens rather than closes — the side
    // is decided by what precedes, and `a"` would legitimately give `a”`.
    type_text(&handle, &doc, &session, "a \"");
    assert_eq!(plain(&doc), "a “");
    let end = handle.cursor_position();
    handle.replace_range(end - 1, end, "");
    session.tick(&handle, &doc);
    assert_eq!(
        plain(&doc),
        "a ",
        "the quote is gone, not turned back into a literal one"
    );
}

/// Until the project's row resolves, nothing is substituted — a document
/// must never be rewritten under a guess about settings still loading.
#[test]
fn nothing_is_substituted_before_the_settings_resolve() {
    let (doc, handle, session, _tree) = editor("");
    session.set_locale("en-US");
    // `set_punctuation` deliberately not called.
    type_text(&handle, &doc, &session, "wait... \"no\"");
    assert_eq!(plain(&doc), "wait... \"no\"");
}

/// And an explicitly all-off row substitutes nothing either, while the
/// lexicon carries on working — the two switches are independent.
#[test]
fn punctuation_off_leaves_the_lexicon_running() {
    let (doc, handle, session, _tree) = editor("");
    session.set_locale("en-US");
    session.set_punctuation(Some(SmartPunctuationFlags::all_off()));
    type_text(&handle, &doc, &session, "wait... btw ");
    assert_eq!(plain(&doc), "wait... by the way ");
}

/// **The document's language picks the quotation marks**, through the same
/// live editor a writer types into.
///
/// This is the test that answers "does it adapt to the project's language,
/// or is it really just English and French?" — every row here is a locale
/// the ruleset table carries, and each opens with its own glyph. Dashes and
/// the ellipsis are deliberately absent: they are locale-independent, which
/// is exactly why the feature can *look* English-only until someone types a
/// quotation mark.
#[test]
fn the_documents_language_picks_the_quotation_marks() {
    for (locale, want) in [
        ("en-US", "\u{201C}"),         // “
        ("fr-FR", "\u{00AB}\u{202F}"), // « + its inner thin space
        ("de-DE", "\u{201E}"),         // „
        ("de-CH", "\u{00AB}"),         // « — Switzerland departs from German
        ("es-ES", "\u{00AB}"),
        ("it-IT", "\u{00AB}"),
        ("pt-PT", "\u{00AB}"),
        ("pt-BR", "\u{201C}"), // Brazil departs from Portugal
        ("nl-NL", "\u{201C}"),
        ("pl-PL", "\u{201E}"),
        ("ru-RU", "\u{00AB}"),
        ("sv-SE", "\u{201D}"), // ” at BOTH ends
        ("tr-TR", "\u{201C}"),
        ("ar", "\u{00AB}"),
    ] {
        let (doc, handle, session, _tree) = editor("");
        punctuate(&session, locale);
        type_text(&handle, &doc, &session, "x \"");
        assert_eq!(
            plain(&doc),
            format!("x {want}"),
            "{locale} must open its quotation with {want}"
        );
    }
}

/// A region with no row of its own inherits its language's typography
/// rather than falling back to English — `fr-CA` is French.
#[test]
fn an_unlisted_region_inherits_its_language() {
    let (doc, handle, session, _tree) = editor("");
    punctuate(&session, "fr-CA");
    type_text(&handle, &doc, &session, "il dit \"");
    assert_eq!(plain(&doc), "il dit \u{00AB}\u{202F}");
}

/// **One Backspace undoes a spaced guillemet whole.** French opens with `«`
/// plus an inner narrow no-break space, so the substitution puts TWO
/// characters in for the one `"` the writer typed. A plain per-character
/// Backspace would delete only the trailing space (its own grapheme cluster)
/// and leave a bare `«` — or delete the `«` and strand an invisible space.
/// The collapse revert restores the single `"` the writer actually typed.
#[test]
fn one_backspace_collapses_a_spaced_guillemet_to_the_typed_quote() {
    let (doc, handle, session, _tree) = editor("");
    punctuate(&session, "fr-FR");
    type_text(&handle, &doc, &session, "il dit \"");
    assert_eq!(plain(&doc), "il dit \u{00AB}\u{202F}");

    // Backspace removes the trailing no-break space; the tick that follows
    // collapses the whole substitution back to the `"`.
    let end = handle.cursor_position();
    handle.replace_range(end - 1, end, "");
    session.tick(&handle, &doc);
    assert_eq!(
        plain(&doc),
        "il dit \"",
        "one Backspace must undo the auto-guillemet, not strand its inner space"
    );
}

/// The collapse revert is armed for exactly one keystroke. If the writer
/// keeps typing inside the fresh guillemet instead of backspacing, a later
/// Backspace is an ordinary character delete, not a collapse.
#[test]
fn typing_on_after_a_guillemet_disarms_the_collapse() {
    let (doc, handle, session, _tree) = editor("");
    punctuate(&session, "fr-FR");
    type_text(&handle, &doc, &session, "il dit \"a");
    assert_eq!(plain(&doc), "il dit \u{00AB}\u{202F}a");

    // Backspace now deletes the "a" as any character, leaving the guillemet
    // and its space intact — the collapse window closed when "a" was typed.
    let end = handle.cursor_position();
    handle.replace_range(end - 1, end, "");
    session.tick(&handle, &doc);
    assert_eq!(plain(&doc), "il dit \u{00AB}\u{202F}");
}

// ── The paragraph/clause subsystem ───────────────────────────────────────

fn spanish(session: &TextReplacementSession) {
    session.set_locale("es-ES");
    session.set_punctuation(Some(SmartPunctuationFlags::default()));
}

/// **The rule that cannot work from the tail.** Spanish opens a question
/// where the *clause* began, which is most of a line behind the caret.
#[test]
fn spanish_opens_its_question_at_the_start_of_the_clause() {
    let (doc, handle, session, _tree) = editor("");
    spanish(&session);
    type_text(&handle, &doc, &session, "Que hora es?");
    assert_eq!(plain(&doc), "\u{00BF}Que hora es?");
    assert_eq!(
        handle.cursor_position(),
        "\u{00BF}Que hora es?".chars().count(),
        "the caret stays after the `?` — it must NOT jump back to the mark"
    );
}

#[test]
fn spanish_opens_an_exclamation_too() {
    let (doc, handle, session, _tree) = editor("");
    spanish(&session);
    type_text(&handle, &doc, &session, "Que bien!");
    assert_eq!(plain(&doc), "\u{00A1}Que bien!");
}

/// The case that makes this a *clause* scan and not a sentence one: Spanish
/// re-opens mid sentence.
#[test]
fn spanish_reopens_after_a_comma_mid_sentence() {
    let (doc, handle, session, _tree) = editor("");
    spanish(&session);
    type_text(&handle, &doc, &session, "Si puedes, vienes?");
    assert_eq!(plain(&doc), "Si puedes, \u{00BF}vienes?");
}

/// A second question in the same paragraph opens its own clause, not the
/// first one again.
#[test]
fn a_second_question_opens_its_own_clause() {
    let (doc, handle, session, _tree) = editor("");
    spanish(&session);
    type_text(&handle, &doc, &session, "Vienes? Cuando?");
    assert_eq!(plain(&doc), "\u{00BF}Vienes? \u{00BF}Cuando?");
}

/// A writer who typed the mark themselves must not get a second one.
#[test]
fn an_already_opened_question_is_left_alone() {
    let (doc, handle, session, _tree) = editor("");
    spanish(&session);
    type_text(&handle, &doc, &session, "\u{00BF}Vienes?");
    assert_eq!(plain(&doc), "\u{00BF}Vienes?");
}

/// And a `¿` the writer placed *mid*-clause is still an existing mark: the
/// guard checks the whole clause, not just its first character, so no second
/// `¿` is prepended to give `¿Es ¿que?`.
#[test]
fn a_mid_clause_question_mark_is_not_doubled() {
    let (doc, handle, session, _tree) = editor("");
    spanish(&session);
    type_text(&handle, &doc, &session, "Es \u{00BF}que?");
    assert_eq!(plain(&doc), "Es \u{00BF}que?");
}

/// A decimal point does not end the clause: the `.` in `3.14` is part of a
/// number, so the mark opens the whole question, not the fraction — `¿Cuesta
/// 3.14?`, never `Cuesta 3.¿14?`.
#[test]
fn a_decimal_point_does_not_split_the_clause() {
    let (doc, handle, session, _tree) = editor("");
    spanish(&session);
    type_text(&handle, &doc, &session, "Cuesta 3.14?");
    assert_eq!(plain(&doc), "\u{00BF}Cuesta 3.14?");
}

/// Nor does the colon of a clock time: `10:30` is one token, so the question
/// opens before it — `¿Son las 10:30?`, not `Son las 10:¿30?`.
#[test]
fn a_clock_time_does_not_split_the_clause() {
    let (doc, handle, session, _tree) = editor("");
    spanish(&session);
    type_text(&handle, &doc, &session, "Son las 10:30?");
    assert_eq!(plain(&doc), "\u{00BF}Son las 10:30?");
}

/// A colon that is genuinely punctuation — not between digits — still ends
/// the clause, so the question that follows opens after it.
#[test]
fn a_real_colon_still_opens_the_next_clause() {
    let (doc, handle, session, _tree) = editor("");
    spanish(&session);
    type_text(&handle, &doc, &session, "Dice: vienes?");
    assert_eq!(plain(&doc), "Dice: \u{00BF}vienes?");
}

/// A dialogue dash left as a plain hyphen — the conversion off, or the raw
/// character — still sits *outside* the clause, so the opening mark lands
/// after it: `-¿Vienes?`, never `¿-Vienes?`.
#[test]
fn a_plain_hyphen_dash_keeps_the_mark_after_it() {
    let (doc, handle, session, _tree) = editor("");
    spanish(&session);
    type_text(&handle, &doc, &session, "-Vienes?");
    assert_eq!(plain(&doc), "-\u{00BF}Vienes?");
}

/// Neighbours that do NOT invert. Catalan and Portuguese sit next to Spanish
/// in the locale table and share its guillemets — inserting `¿` into either
/// would be a character no reader of them expects.
#[test]
fn the_neighbouring_languages_do_not_invert() {
    for locale in ["ca", "pt-PT", "pt-BR", "fr-FR", "it-IT", "en-US"] {
        let (doc, handle, session, _tree) = editor("");
        session.set_locale(locale);
        session.set_punctuation(Some(SmartPunctuationFlags::default()));
        type_text(&handle, &doc, &session, "Que tal?");
        assert_eq!(plain(&doc), "Que tal?", "{locale} must not invert");
    }
}

/// **The bug this pins.** The order the real app pushes state in is: the
/// document's language first, its punctuation flags later (they come from a
/// row that loads asynchronously). Between the two, the session's flags are
/// `None` — and Spanish's `¿` does not need them, only the locale. A cache
/// key that dropped the locale while the flags were `None` left the engine
/// pinned to locale `""`, so `¿` never fired in exactly this window.
#[test]
fn spanish_fires_when_the_language_arrives_before_the_flags() {
    let (doc, handle, session, _tree) = editor("");
    // Language known; flags NOT yet resolved — `set_punctuation` never
    // called, so `punctuation` is `None`.
    session.set_locale("es-ES");
    type_text(&handle, &doc, &session, "Hola?");
    assert_eq!(
        plain(&doc),
        "\u{00BF}Hola?",
        "the opening mark must fire on the locale alone, before any flags load"
    );
}

/// And a language *change* while the flags stay unresolved must re-reach the
/// engine — the same key bug, in its other guise.
#[test]
fn a_language_change_reaches_the_engine_with_no_flags_set() {
    let (doc, handle, session, _tree) = editor("");
    session.set_locale("en-US");
    type_text(&handle, &doc, &session, "Hola?");
    assert_eq!(plain(&doc), "Hola?", "English does not invert");

    session.set_locale("es-ES");
    type_text(&handle, &doc, &session, " Que?");
    assert!(
        plain(&doc).ends_with("\u{00BF}Que?"),
        "the switch to Spanish must take effect, got {:?}",
        plain(&doc)
    );
}

/// The dialogue dash opens a paragraph typed as `- `.
#[test]
fn a_dialogue_dash_opens_the_paragraph() {
    let (doc, handle, session, _tree) = editor("");
    session.set_locale("fr-FR");
    session.set_punctuation(Some(SmartPunctuationFlags {
        dialogue_marker: true,
        ..SmartPunctuationFlags::default()
    }));
    type_text(&handle, &doc, &session, "- ");
    assert_eq!(plain(&doc), "\u{2014}\u{00A0}");
}

/// And a hyphen anywhere else is a hyphen. A rule that fired mid-line would
/// mangle ordinary prose, which is why it matches the whole paragraph so far
/// rather than just the two characters behind the caret.
#[test]
fn a_hyphen_mid_paragraph_is_left_alone() {
    let (doc, handle, session, _tree) = editor("");
    session.set_locale("fr-FR");
    session.set_punctuation(Some(SmartPunctuationFlags {
        dialogue_marker: true,
        ..SmartPunctuationFlags::default()
    }));
    type_text(&handle, &doc, &session, "eh bien - ");
    assert_eq!(plain(&doc), "eh bien - ");
}

/// The dialogue dash IS behind a flag, unlike Spanish's marks — it is a
/// convention some books follow and others do not, where writing `¿` is
/// simply writing the language.
#[test]
fn the_dialogue_dash_is_off_unless_asked_for() {
    let (doc, handle, session, _tree) = editor("");
    session.set_locale("fr-FR");
    session.set_punctuation(Some(SmartPunctuationFlags::default()));
    type_text(&handle, &doc, &session, "- ");
    assert_eq!(plain(&doc), "- ", "default is off");
}

/// A language with no dialogue dash in its table gets none even when asked.
#[test]
fn a_language_without_a_dialogue_dash_gets_none() {
    let (doc, handle, session, _tree) = editor("");
    session.set_locale("en-US");
    session.set_punctuation(Some(SmartPunctuationFlags {
        dialogue_marker: true,
        ..SmartPunctuationFlags::default()
    }));
    type_text(&handle, &doc, &session, "- ");
    assert_eq!(plain(&doc), "- ");
}

// ── Nested quotes ────────────────────────────────────────────────────────

fn quotes_for(session: &TextReplacementSession, locale: &str) {
    session.set_locale(locale);
    session.set_punctuation(Some(SmartPunctuationFlags::default()));
}

/// French switches to curly doubles inside its guillemets, and the writer
/// types `"` for every level.
#[test]
fn french_quotes_nest_from_guillemets_to_curly_doubles() {
    let (doc, handle, session, _tree) = editor("");
    quotes_for(&session, "fr-FR");
    type_text(&handle, &doc, &session, "\"a \"b\" c\"");
    // Outer guillemets carry their French inner space; the inner curly
    // doubles do not.
    assert_eq!(
        plain(&doc),
        "\u{00AB}\u{202F}a \u{201C}b\u{201D} c\u{202F}\u{00BB}"
    );
}

/// Russian nests guillemets into low-high doubles — a different inner mark,
/// proving the rule reads the locale's own secondary rather than a constant.
#[test]
fn russian_quotes_nest_from_guillemets_to_low_high() {
    let (doc, handle, session, _tree) = editor("");
    quotes_for(&session, "ru-RU");
    type_text(&handle, &doc, &session, "\"a \"b\" c\"");
    assert_eq!(plain(&doc), "\u{00AB}a \u{201E}b\u{201C} c\u{00BB}");
}

/// Three levels deep, the marks alternate back to the outer style — the
/// same as every word processor.
#[test]
fn a_third_level_alternates_back_to_the_primary() {
    let (doc, handle, session, _tree) = editor("");
    quotes_for(&session, "fr-FR");
    type_text(&handle, &doc, &session, "\"a \"b \"c\"");
    // « (spaced) then “ then « (spaced) again at depth 2, closing » (spaced).
    assert_eq!(
        plain(&doc),
        "\u{00AB}\u{202F}a \u{201C}b \u{00AB}\u{202F}c\u{202F}\u{00BB}"
    );
}

/// **The reason nesting is gated on double-width secondaries.** English's
/// inner mark is a single curly quote, which the writer reaches with `'`,
/// not `"`. Typing `"` inside a quote must stay a double, curled by context
/// — exactly what Word does — not silently turn into a `’`.
#[test]
fn english_quotes_do_not_switch_to_single_when_nested() {
    let (doc, handle, session, _tree) = editor("");
    quotes_for(&session, "en-US");
    type_text(&handle, &doc, &session, "\"a \"b\" c\"");
    assert_eq!(plain(&doc), "\u{201C}a \u{201C}b\u{201D} c\u{201D}");
}

/// And the apostrophe hazard the gate exists to avoid: an elision inside a
/// French quotation must not be counted as a closing mark and throw the
/// depth off. `l'ami` carries an apostrophe; the closing `"` must still land
/// on the guillemet.
#[test]
fn an_apostrophe_inside_a_quote_does_not_corrupt_the_depth() {
    let (doc, handle, session, _tree) = editor("");
    quotes_for(&session, "fr-FR");
    type_text(&handle, &doc, &session, "\"l'ami\"");
    // «…» (spaced) — the apostrophe curled to ’, and the close is still a
    // guillemet, not thrown off by counting the apostrophe as a quote.
    assert_eq!(plain(&doc), "\u{00AB}\u{202F}l\u{2019}ami\u{202F}\u{00BB}");
}

/// With the project's master switch off, nothing expands at all.
#[test]
fn the_master_switch_off_disables_every_rule() {
    let doc = TextDocument::new();
    let ed = RichTextEditor::editor(doc.clone());
    let handle = ed.handle();
    let mut tree = WidgetTree::new();
    tree.add(ed);
    tree.layout(SizeProposal::exact(600.0, 400.0));

    let ctx = Rc::new(AppContext::new());
    let work = SingleWork::new(ctx.clone());
    // Left at its default: off.
    let ids = AppIds::new();
    let vm = TextReplacementRulesViewModel::new(
        TextReplacementRuleListModel::new(ctx, ids.clone()),
        work,
        ids,
    );
    let session = TextReplacementSession::new(vm);
    type_text(&handle, &doc, &session, "I saw btw ");
    assert_eq!(plain(&doc), "I saw btw ");
    drop(tree);
}
