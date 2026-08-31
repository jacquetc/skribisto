// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

/// Space-separated in the tests, a list in storage — one parser, shared.
use skribisto_model::language::parse_legacy_list as tags;

use super::*;
use spellcheck_engine::{MAX_SUGGESTIONS, bounded_levenshtein, detect_encoding, merge_suggestions};

// ── the master switch (Settings ▸ Spelling / the title-bar toggle / F7) ──

/// A service whose `en-US` dictionary is already in the cache, so these tests never touch
/// the disk and never depend on what this machine happens to have installed.
fn service_with_tiny_dict() -> SpellcheckService {
    let svc = SpellcheckService::new();
    let dict = spellbook::Dictionary::new("SET UTF-8\n", "2\nhello\nworld\n").unwrap();
    svc.inner
        .cache
        .borrow_mut()
        .insert("en-US".to_string(), Some(Arc::new(dict)));
    svc
}

/// Spell-check is on out of the box — the switch is an escape hatch, not an opt-in.
#[test]
fn spellcheck_is_enabled_by_default() {
    assert!(SpellcheckService::new().is_enabled());
}

#[test]
fn set_enabled_reports_change_only_on_a_real_flip() {
    let svc = SpellcheckService::new();
    assert!(!svc.set_enabled(true), "already on — not a change");
    assert!(svc.set_enabled(false), "on -> off is a change");
    assert!(!svc.set_enabled(false), "already off — not a change");
    assert!(svc.set_enabled(true), "off -> on is a change");
}

/// The pills bind `mute_version` and nothing else, so a flip must bump it or they keep
/// showing live green checks while the switch is off.
#[test]
fn set_enabled_bumps_mute_version_on_a_real_flip() {
    let svc = SpellcheckService::new();
    let v = svc.mute_version();
    let before = v.get();
    svc.set_enabled(false);
    assert_eq!(v.get(), before + 1, "a real flip rebuilds the pill field");
    svc.set_enabled(false);
    assert_eq!(v.get(), before + 1, "a no-op flip must not churn the UI");
}

/// The one gate: off means every document's checker is `None`, which is the same degrade
/// path a missing dictionary already takes.
#[test]
fn build_checker_short_circuits_when_disabled_and_resumes_when_re_enabled() {
    let svc = service_with_tiny_dict();
    assert!(
        svc.build_checker(&tags("en-US"), Some(1)).is_some(),
        "on by default"
    );

    svc.set_enabled(false);
    assert!(
        svc.build_checker(&tags("en-US"), Some(1)).is_none(),
        "off — no checker at all"
    );

    svc.set_enabled(true);
    let checker = svc.build_checker(&tags("en-US"), Some(1)).expect("back on");
    assert!(checker.misspelled("helo"), "and it checks again");
}

/// Off must beat everything downstream — a document that would otherwise be checked (an
/// installed, unmuted language) still gets nothing.
#[test]
fn the_master_switch_overrides_an_otherwise_checkable_document() {
    let svc = service_with_tiny_dict();
    assert!(
        !svc.is_muted("en-US", Some(1)),
        "precondition: nothing muted"
    );
    svc.set_enabled(false);
    assert!(
        svc.build_checker(&tags("en-US"), Some(1)).is_none(),
        "an installed, unmuted language is still not checked when the switch is off"
    );
}

/// `clear(work_id)` is `close_work`: it drops that Work's **own** project-scoped state
/// (session mutes + personal words) — never the shared dictionary cache, and never a
/// different, still-open Work's state. The master switch is an app-wide preference and
/// must survive — a writer who turned spell-check off does not expect the next project to
/// turn it back on.
#[test]
fn close_work_does_not_reset_the_master_switch() {
    let svc = SpellcheckService::new();
    svc.set_enabled(false);
    svc.clear(1);
    assert!(
        !svc.is_enabled(),
        "the switch is app-wide, not project state"
    );
}

// ── Multi-Work isolation (Phase 2) ──────────────────────────────────────

/// The bug this migration fixed: closing one Work must never wipe a different, still-open
/// Work's session mutes or personal words. Before partitioning by `work_id`, `clear()` was
/// one flat set shared by every open Work.
#[test]
fn closing_one_work_leaves_a_different_open_works_mutes_and_personal_words_intact() {
    let svc = service_with_tiny_dict();
    let (work_a, work_b) = (1u64, 2u64);

    svc.set_muted("en-US", true, Some(work_a));
    let mut personal_a = HashSet::new();
    personal_a.insert("Skribisto".to_string());
    svc.set_personal(work_a, personal_a);

    let mut personal_b = HashSet::new();
    personal_b.insert("Teksilo".to_string());
    svc.set_personal(work_b, personal_b);

    // Work B closes.
    svc.clear(work_b);

    assert!(
        svc.is_muted("en-US", Some(work_a)),
        "Work A's own mute must survive Work B's close"
    );
    assert!(
        !svc.is_muted("en-US", Some(work_b)),
        "Work B's mute is gone, as expected"
    );
    let checker_a = svc
        .build_checker(&tags("fr-FR"), Some(work_a))
        .expect("Work A still has an active (unmuted) dictionary/personal word");
    assert!(
        !checker_a.misspelled("Skribisto"),
        "Work A's personal word survives Work B's close"
    );
}

/// Two simultaneously-open Works never see each other's personal words or mutes — the
/// live, non-close-related half of the same isolation guarantee.
#[test]
fn two_open_works_never_share_personal_words_or_mutes() {
    let svc = service_with_tiny_dict();
    let (work_a, work_b) = (1u64, 2u64);

    let mut personal_a = HashSet::new();
    personal_a.insert("Skribisto".to_string());
    svc.set_personal(work_a, personal_a);
    svc.set_muted("en-US", true, Some(work_a));

    // Work B has its own, disjoint personal set and no mutes.
    let mut personal_b = HashSet::new();
    personal_b.insert("Teksilo".to_string());
    svc.set_personal(work_b, personal_b);

    assert!(
        !svc.is_muted("en-US", Some(work_b)),
        "Work B never inherits Work A's mute"
    );
    let checker_b = svc
        .build_checker(&tags("en-US"), Some(work_b))
        .expect("Work B's own dictionary is still active — it never muted en-US");
    assert!(
        checker_b.misspelled("Skribisto"),
        "Work B's checker must not know Work A's personal word"
    );
    assert!(
        !checker_b.misspelled("Teksilo"),
        "Work B's checker knows its own personal word"
    );
}

/// Char offsets are correct through accented text (a byte offset would be wrong here).
#[test]
fn word_positions_uses_char_offsets() {
    // "éàî mot" — the first word is 3 chars (6 bytes); "mot" starts at char 4.
    let got = word_positions("éàî mot");
    assert_eq!(got[0], (0, 3, "éàî"));
    assert_eq!(got[1], (4, 3, "mot"));
}

/// Contractions and elisions stay one token, for both apostrophes.
#[test]
fn word_positions_keeps_apostrophes() {
    let straight: Vec<&str> = word_positions("don't").iter().map(|(_, _, w)| *w).collect();
    assert_eq!(straight, ["don't"]);
    let curly: Vec<&str> = word_positions("l\u{2019}auteur")
        .iter()
        .map(|(_, _, w)| *w)
        .collect();
    assert_eq!(curly, ["l\u{2019}auteur"]);
}

/// A tiny real dictionary flags the misspelling and leaves the good word and the number.
#[test]
fn highlighter_flags_only_the_misspelling() {
    let dict = spellbook::Dictionary::new("SET UTF-8\n", "2\nhello\nworld\n")
        .expect("tiny dictionary parses");
    let hl = SpellChecker::new(vec![Arc::new(dict)], HashSet::new());
    assert!(hl.misspelled("helo"), "a misspelling is flagged");
    assert!(!hl.misspelled("hello"), "a good word is not");
    assert!(!hl.misspelled("world"), "another good word is not");
    assert!(!hl.misspelled("123"), "a number is never a misspelling");
}

/// A personal word overrides the dictionary — checked first, so no dictionary mutation.
#[test]
fn personal_words_are_the_final_fallback() {
    // The project's own word list is the last checker: a word no installed dictionary knows is
    // rescued by the personal set; an unknown word absent from both stays flagged.
    let dict = spellbook::Dictionary::new("SET UTF-8\n", "1\nhello\n").unwrap();
    let mut personal = HashSet::new();
    personal.insert("Skribisto".to_string());
    let hl = SpellChecker::new(vec![Arc::new(dict)], personal);
    assert!(!hl.misspelled("Skribisto"), "a personal word is accepted");
    assert!(hl.misspelled("Skrib"), "but not an unrelated unknown word");
}

// ── Suggestions (the context-menu corrections) ──

/// The installed dictionary corrects a typo of an ordinary word.
#[test]
fn suggest_offers_dictionary_corrections() {
    let dict = spellbook::Dictionary::new("SET UTF-8\n", "2\nhello\nworld\n").unwrap();
    let hl = SpellChecker::new(vec![Arc::new(dict)], HashSet::new());
    let got = hl.suggest("helo");
    assert!(
        got.contains(&"hello".to_string()),
        "expected 'hello' in {got:?}"
    );
    assert!(
        got.len() <= MAX_SUGGESTIONS,
        "capped at {MAX_SUGGESTIONS}: {got:?}"
    );
    // Nothing alphabetic is not correctable.
    assert!(hl.suggest("123").is_empty(), "a number has no corrections");
}

/// **The personal-dictionary suggestion.** `spellbook` cannot see the personal set, so a typo
/// of a project's coined term is only ever corrected by our own near-match pass — and it must
/// outrank the dictionary's guesses for an invented word.
#[test]
fn suggest_offers_personal_words_the_dictionary_cannot_know() {
    let dict = spellbook::Dictionary::new("SET UTF-8\n", "2\nhello\nworld\n").unwrap();
    let mut personal = HashSet::new();
    personal.insert("Skribisto".to_string());
    let hl = SpellChecker::new(vec![Arc::new(dict)], personal);
    // Sanity: the dictionary alone knows nothing of it.
    let mut raw = Vec::new();
    hl.dictionaries()[0].suggest("Skibisto", &mut raw);
    assert!(
        !raw.contains(&"Skribisto".to_string()),
        "precondition: spellbook cannot suggest a personal word ({raw:?})"
    );
    // But we can — and it comes first, being one edit away.
    let got = hl.suggest("Skibisto");
    assert_eq!(
        got.first().map(String::as_str),
        Some("Skribisto"),
        "a one-edit personal word leads the list, got {got:?}"
    );
}

/// A personal word differing only in **casing** is a distance-0 match. This is the everyday
/// case: the personal set is matched exact-case, so `skribisto` is genuinely flagged, and the
/// correction to offer is the project's own casing.
#[test]
fn suggest_corrects_the_casing_of_a_personal_word() {
    let dict = spellbook::Dictionary::new("SET UTF-8\n", "1\nhello\n").unwrap();
    let mut personal = HashSet::new();
    personal.insert("Skribisto".to_string());
    let hl = SpellChecker::new(vec![Arc::new(dict)], personal);
    assert!(
        hl.misspelled("skribisto"),
        "precondition: exact-case matching flags it"
    );
    assert_eq!(
        hl.suggest("skribisto").first().map(String::as_str),
        Some("Skribisto"),
        "the stored casing is offered"
    );
}

/// A personal word is never suggested for itself, and a distant one is not suggested at all.
#[test]
fn suggest_skips_the_typed_word_and_distant_personal_words() {
    let dict = spellbook::Dictionary::new("SET UTF-8\n", "1\nhello\n").unwrap();
    let mut personal = HashSet::new();
    personal.insert("Skribisto".to_string());
    personal.insert("Teksilo".to_string());
    let hl = SpellChecker::new(vec![Arc::new(dict)], personal);
    // "Skribisto" itself is not flagged, but even asked directly it must not echo back.
    assert!(
        !hl.suggest("Skribisto").contains(&"Skribisto".to_string()),
        "a word is never its own correction"
    );
    // "Teksilo" is far from "Skibisto" — beyond the typo radius, so it is not offered.
    assert!(
        !hl.suggest("Skibisto").contains(&"Teksilo".to_string()),
        "an unrelated personal word is not a correction"
    );
}

/// A dictionary chatty enough to fill every slot on its own — what Hunspell's ngram search
/// does for a coined word nothing knows. Synthesised, because a test-sized dictionary has no
/// `TRY` table and cannot be provoked into guessing that freely.
fn chatty_dict() -> Vec<String> {
    ["aaa", "bbb", "ccc", "ddd", "eee", "fff", "ggg", "hhh"]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

/// **The reserved slots.** Without [`PERSONAL_SUGGESTION_FLOOR`] a two-edit personal match is
/// appended past the cap and truncated away — so for `Skiibsto` the writer would get six
/// unrelated English guesses and never the project's own term.
#[test]
fn a_distant_personal_word_survives_a_talkative_dictionary() {
    let personal = vec![(2usize, "Helios".to_string())];
    let got = merge_suggestions("helo", personal, chatty_dict().into_iter());
    assert!(
        got.contains(&"Helios".to_string()),
        "the 2-edit personal word keeps its reserved slot, got {got:?}"
    );
    assert_eq!(
        got.len(),
        MAX_SUGGESTIONS,
        "and the list is still full: {got:?}"
    );
    assert_eq!(
        got.last().map(String::as_str),
        Some("Helios"),
        "it trails the dictionary's own ranked guesses"
    );
}

/// No personal matches ⇒ nothing is reserved and the dictionary gets every slot.
#[test]
fn without_personal_matches_the_dictionary_fills_the_list() {
    let got = merge_suggestions("helo", vec![], chatty_dict().into_iter());
    assert_eq!(
        got.len(),
        MAX_SUGGESTIONS,
        "the floor must not shrink the list when there is nothing to reserve for: {got:?}"
    );
}

/// A one-edit personal match leads *and* still leaves the dictionary its slots.
#[test]
fn a_near_personal_word_leads_without_reserving() {
    let personal = vec![(1usize, "Helo2".to_string())];
    let got = merge_suggestions("helo", personal, chatty_dict().into_iter());
    assert_eq!(got.first().map(String::as_str), Some("Helo2"), "{got:?}");
    assert_eq!(got.len(), MAX_SUGGESTIONS);
}

/// **`near` is not exempt from the ceiling.** A glossary of similar short terms (a
/// Kai/Kal/Kar naming family) can yield more one-edit matches than the menu holds; without
/// the ceiling those would crowd out the reserved `far` slots.
#[test]
fn a_crowd_of_near_personal_words_cannot_evict_the_reserved_far_slots() {
    let personal = vec![
        (1usize, "Kai".to_string()),
        (1, "Kal".to_string()),
        (1, "Kar".to_string()),
        (1, "Kaz".to_string()),
        (1, "Kay".to_string()),
        (1, "Kah".to_string()),
        (2, "Kaito".to_string()),
        (2, "Kalim".to_string()),
    ];
    let got = merge_suggestions("Kax", personal, chatty_dict().into_iter());
    assert_eq!(got.len(), MAX_SUGGESTIONS, "{got:?}");
    assert!(
        got.contains(&"Kaito".to_string()) && got.contains(&"Kalim".to_string()),
        "both reserved far slots survive a crowd of near matches, got {got:?}"
    );
    assert_eq!(
        got.iter().filter(|w| w.len() == 3).count(),
        4,
        "near is capped at the ceiling (6 - 2 reserved), got {got:?}"
    );
}

/// **A wasted reserved slot is given back.** When a far word merely repeats something already
/// listed, `push_unique` drops it — the slot must go back to the dictionary rather than hand
/// back a short menu while suggestions remain unfetched.
#[test]
fn a_far_word_colliding_with_a_dictionary_suggestion_backfills() {
    // "aaa" is both the project's own term and the dictionary's first guess.
    let personal = vec![(2usize, "aaa".to_string())];
    let got = merge_suggestions("typed", personal, chatty_dict().into_iter());
    assert_eq!(
        got.len(),
        MAX_SUGGESTIONS,
        "the collided slot is refilled from the dictionary, got {got:?}"
    );
    assert_eq!(
        got.iter().filter(|w| *w == "aaa").count(),
        1,
        "and not duplicated"
    );
}

/// The dictionary is never pulled past the budget — the early-exit that keeps a second
/// language's ngram search from running once the list is full.
#[test]
fn the_dictionary_is_not_pulled_past_the_budget() {
    let pulled = Cell::new(0usize);
    let dict = (0..100).map(|i| {
        pulled.set(pulled.get() + 1);
        format!("w{i}")
    });
    let got = merge_suggestions("helo", vec![(2, "Helios".into())], dict);
    assert_eq!(got.len(), MAX_SUGGESTIONS);
    // 5 dictionary slots (6 minus the one reserved), so the 6th pull never happens.
    assert_eq!(
        pulled.get(),
        MAX_SUGGESTIONS - 1,
        "the suggester must stop at the budget, not run dry"
    );
}

/// No source may echo the typed word back as its own correction — a menu item that edits
/// nothing. The personal set was already filtered; the dictionaries were not.
#[test]
fn the_typed_word_is_never_its_own_correction() {
    let dict = vec!["helo".to_string(), "hello".to_string()];
    let got = merge_suggestions("helo", vec![(0, "helo".into())], dict.into_iter());
    assert_eq!(
        got,
        ["hello"],
        "the input is dropped from every source: {got:?}"
    );
}

/// Ties are ordered deterministically — `personal` is a `HashSet`, and a menu that reshuffles
/// between right-clicks is unusable.
#[test]
fn personal_suggestions_are_stable_across_runs() {
    let dict = spellbook::Dictionary::new("SET UTF-8\n", "1\nhello\n").unwrap();
    // Three terms all exactly one edit from "Xan" — the tie the HashSet would shuffle.
    let personal: HashSet<String> = ["Xen", "Xin", "Xon"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let hl = SpellChecker::new(vec![Arc::new(dict)], personal);
    let first = hl.suggest("Xan");
    assert_eq!(
        first,
        ["Xen", "Xin", "Xon"],
        "equal-distance ties sort alphabetically"
    );
    for _ in 0..5 {
        assert_eq!(
            hl.suggest("Xan"),
            first,
            "the order must not vary between calls"
        );
    }
}

/// The bounded edit distance itself: case is folded by the caller, the cap is honoured, and
/// accented words measure in characters.
#[test]
fn bounded_levenshtein_measures_chars_and_honours_the_cap() {
    let d = |a: &str, b: &str| {
        let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
        bounded_levenshtein(&a, &b, 2)
    };
    assert_eq!(d("abc", "abc"), Some(0));
    assert_eq!(d("abc", "abd"), Some(1)); // substitution
    assert_eq!(d("abc", "ab"), Some(1)); // deletion
    assert_eq!(d("abc", "abcd"), Some(1)); // insertion
    assert_eq!(d("abc", "xyz"), None, "3 edits exceeds the cap");
    // A length gap alone exceeds the cap — rejected without building the matrix.
    assert_eq!(d("a", "abcdef"), None);
    // "café" vs "cafe" is one char edit, not two bytes' worth.
    assert_eq!(d("café", "cafe"), Some(1));
    // Empty on either side is the other's length, still subject to the cap.
    assert_eq!(d("", "ab"), Some(2));
    assert_eq!(d("ab", ""), Some(2));
    assert_eq!(d("", ""), Some(0));
}

/// The length gate must not reject a candidate whose *lower-cased* form is within the cap
/// even though its raw form is not — `İ` lower-cases to two chars, so counting the raw word
/// would measure the wrong length.
#[test]
fn a_candidate_is_gated_on_its_lower_cased_length() {
    // "İ" (U+0130) lower-cases to "i̇" — 1 char becomes 2.
    assert_eq!("İ".chars().count(), 1);
    assert_eq!("İ".to_lowercase().chars().count(), 2);
    // A personal word whose lower-cased form is exactly the typed word must be found.
    let hl = SpellChecker::from_word_lists(&["hello"], &["İstanbul"]);
    let got = hl.suggest("i\u{307}stanbul"); // the lower-cased spelling, typed by the writer
    assert_eq!(
        got.first().map(String::as_str),
        Some("İstanbul"),
        "the stored casing is offered, got {got:?}"
    );
}

/// The multi-dictionary union: a word only one language knows is still accepted.
#[test]
fn a_word_any_active_dictionary_knows_is_accepted() {
    let en = spellbook::Dictionary::new("SET UTF-8\n", "1\nhello\n").unwrap();
    let fr = spellbook::Dictionary::new("SET UTF-8\n", "1\nbonjour\n").unwrap();
    let hl = SpellChecker::new(vec![Arc::new(en), Arc::new(fr)], HashSet::new());
    assert!(!hl.misspelled("hello"), "English word accepted");
    assert!(!hl.misspelled("bonjour"), "French word accepted");
    assert!(hl.misspelled("guten"), "a word neither knows is flagged");
}

/// `validate_dictionary_files` accepts a real pair and rejects garbage / missing files.
#[test]
fn validate_accepts_good_rejects_bad() {
    let dir = std::env::temp_dir().join(format!("skrib-valdict-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let aff = dir.join("ok.aff");
    let dic = dir.join("ok.dic");
    std::fs::write(&aff, "SET UTF-8\n").unwrap();
    std::fs::write(&dic, "1\nhello\n").unwrap();
    assert!(
        validate_dictionary_files(&aff, &dic).is_ok(),
        "a real pair validates"
    );

    // A missing file is a read error, not a panic.
    assert!(validate_dictionary_files(&dir.join("nope.aff"), &dic).is_err());

    // A .dic whose count line is nonsense fails to parse (spellbook rejects it).
    let bad = dir.join("bad.dic");
    std::fs::write(&bad, "not-a-count\n\0\0garbage").unwrap();
    assert!(
        validate_dictionary_files(&aff, &bad).is_err(),
        "garbage is rejected"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// The ISO-8859-1 transcode path: a `SET ISO8859-1` `.aff` decodes its bytes correctly.
#[test]
fn detect_and_decode_latin1() {
    let enc = detect_encoding(b"SET ISO8859-1\nTRY esiat\n");
    assert_eq!(enc.name(), "windows-1252"); // encoding_rs maps ISO-8859-1 to its superset
    // 0xE9 is 'é' in Latin-1.
    let (decoded, _, _) = enc.decode(b"caf\xe9");
    assert_eq!(decoded, "café");
}

// ── SpellSession (the caret-aware range highlighter) ──

fn tiny_doc(text: &str) -> TextDocument {
    let d = TextDocument::new();
    d.set_plain_text(text).unwrap();
    d
}

/// A checker knowing `hello`/`world` — so `helo`/`wrld` are misspelled.
fn en_checker() -> SpellChecker {
    let dict = spellbook::Dictionary::new("SET UTF-8\n", "2\nhello\nworld\n").unwrap();
    SpellChecker::new(vec![Arc::new(dict)], HashSet::new())
}

/// Focus a single view whose caret is read from `cell` (the closure production supplies is
/// `move || handle.cursor_position()`; a test injects a plain cell instead).
fn focus_at(session: &SpellSession, cell: &Rc<Cell<usize>>) {
    let c = cell.clone();
    session.on_focus(WidgetId::default(), Rc::new(move || c.get()));
}

fn starts(session: &SpellSession) -> Vec<usize> {
    session
        .last_ranges
        .borrow()
        .iter()
        .map(|r| r.start)
        .collect()
}

#[test]
fn session_exempts_the_caret_word_and_flags_the_rest() {
    let doc = tiny_doc("helo wrld"); // both misspelled
    let session = SpellSession::new(&doc);
    let caret = Rc::new(Cell::new(2usize)); // inside "helo" [0,4]
    focus_at(&session, &caret);
    session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
    assert_eq!(
        starts(&session),
        vec![5],
        "caret word exempt; only wrld (char 5) flagged"
    );
}

#[test]
fn no_focused_view_flags_every_misspelling() {
    let doc = tiny_doc("helo wrld");
    let session = SpellSession::new(&doc);
    session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
    assert_eq!(
        starts(&session),
        vec![0, 5],
        "no exemption without a focused caret"
    );
}

#[test]
fn caret_at_the_word_end_keeps_it_exempt() {
    let doc = tiny_doc("helo wrld");
    let session = SpellSession::new(&doc);
    let caret = Rc::new(Cell::new(4usize)); // the END of "helo" [0,4] — still typing it
    focus_at(&session, &caret);
    session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
    assert_eq!(
        starts(&session),
        vec![5],
        "inclusive end keeps the just-typed word exempt"
    );
}

#[test]
fn caret_exempts_at_most_one_word_at_a_zero_gap_boundary() {
    // Two TOUCHING misspelled ranges — [0,1) and [1,2) — as adjacent CJK/Hiragana characters
    // produce (UAX#29 splits them, and no CJK dictionary means both are "misspelled"). A caret
    // exactly on the shared boundary (char 1) is inclusive-in both; only the first must drop.
    let doc = tiny_doc("ab");
    let session = SpellSession::new(&doc);
    let fmt = || spell_format(Color::rgb(220, 50, 50));
    *session.all_ranges.borrow_mut() = vec![
        RangeHighlight {
            start: 0,
            length: 1,
            format: fmt(),
        },
        RangeHighlight {
            start: 1,
            length: 1,
            format: fmt(),
        },
    ];
    let caret = Rc::new(Cell::new(1usize));
    focus_at(&session, &caret);
    session.apply_exemption(true); // forced: `all_ranges` was poked in directly
    assert_eq!(
        starts(&session),
        vec![1],
        "only the first touching word is exempt, not both"
    );
}

#[test]
fn moving_the_caret_reveals_the_word_left_and_hides_the_word_entered() {
    let doc = tiny_doc("helo wrld");
    let session = SpellSession::new(&doc);
    let caret = Rc::new(Cell::new(2usize)); // in "helo"
    focus_at(&session, &caret);
    session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
    assert_eq!(starts(&session), vec![5], "helo exempt, wrld flagged");

    caret.set(6); // move into "wrld" [5,9]
    session.on_caret(WidgetId::default());
    session.tick();
    assert_eq!(
        starts(&session),
        vec![0],
        "now helo is flagged and wrld exempt"
    );
}

/// The fast path: a caret move that stays inside the same exempted word must not
/// re-run the O(all_ranges) filter+clone. Typing within a word ticks the caret on every
/// keystroke, so on a densely-flagged document that clone was the per-keystroke cost.
#[test]
fn a_same_word_caret_move_skips_the_filter() {
    let doc = tiny_doc("helo wrld"); // both misspelled
    let session = SpellSession::new(&doc);
    let caret = Rc::new(Cell::new(1usize)); // inside "helo" [0,4]
    focus_at(&session, &caret);
    session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
    let after_first = session.exemption_recomputes.get();
    assert_eq!(starts(&session), vec![5], "helo exempt, wrld flagged");

    // Three caret moves that never leave "helo" [0,4] — the exempted range is unchanged.
    for pos in [0usize, 2, 4] {
        caret.set(pos);
        session.on_caret(WidgetId::default());
        session.tick();
    }
    assert_eq!(
        session.exemption_recomputes.get(),
        after_first,
        "staying inside the exempt word takes the fast path — no re-filter"
    );
    assert_eq!(starts(&session), vec![5], "and the pushed set is unchanged");

    // Crossing into "wrld" [5,9] genuinely changes the exemption, so it must recompute.
    caret.set(6);
    session.on_caret(WidgetId::default());
    session.tick();
    assert_eq!(
        session.exemption_recomputes.get(),
        after_first + 1,
        "leaving the word recomputes exactly once"
    );
    assert_eq!(starts(&session), vec![0], "wrld now exempt, helo flagged");
}

/// An inactive (hidden) session defers the eager rebuild `set_checker` would do, then
/// catches up on the first tick after it is shown. This is what stops a re-attach
/// (dictionary install / mute / language change) from re-tokenising a hidden 20k-word
/// synopsis nobody can see.
#[test]
fn a_hidden_session_defers_its_rebuild_until_shown() {
    let doc = tiny_doc("helo wrld"); // both misspelled
    let session = SpellSession::new(&doc);
    session.set_active(false);

    // A checker arrives while hidden — stored, but no ranges computed yet.
    session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
    assert!(
        session.all_ranges.borrow().is_empty(),
        "a hidden pane does not tokenise on set_checker"
    );
    assert!(
        session.last_ranges.borrow().is_empty(),
        "and nothing is pushed"
    );

    // A tick while still hidden stays a no-op (and must not consume the owed rebuild).
    session.tick();
    assert!(
        session.all_ranges.borrow().is_empty(),
        "still nothing while hidden"
    );

    // Shown again → the next tick performs the deferred rebuild.
    session.set_active(true);
    session.tick();
    assert_eq!(
        starts(&session),
        vec![0, 5],
        "the catch-up rebuild runs once the pane is shown (no focused caret → both flagged)"
    );
}

#[test]
fn a_content_edit_re_derives_on_the_next_tick() {
    let doc = tiny_doc("hello"); // correct → nothing flagged
    let session = SpellSession::new(&doc);
    session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
    assert!(
        session.last_ranges.borrow().is_empty(),
        "correct prose has no squiggle"
    );

    doc.set_plain_text("helo").unwrap(); // now misspelled — fires an offset-moving event
    session.tick();
    assert_eq!(
        starts(&session),
        vec![0],
        "the edit is picked up on the tick"
    );
}

#[test]
fn char_offsets_are_document_absolute_through_accents() {
    // "café" is correct; "wrld" is the misspelling. A byte offset would place it at 6 (é is two
    // bytes); the char offset is 5.
    let dict = spellbook::Dictionary::new("SET UTF-8\n", "2\ncafé\nworld\n").unwrap();
    let checker = SpellChecker::new(vec![Arc::new(dict)], HashSet::new());
    let doc = tiny_doc("café wrld");
    let session = SpellSession::new(&doc);
    session.set_checker(Some(checker), Color::rgb(220, 50, 50));
    assert_eq!(starts(&session), vec![5], "char offset, not byte offset");
}

#[test]
fn a_second_paragraph_gets_absolute_offsets() {
    let doc = tiny_doc("hello\nwrld"); // block 2 ("wrld") starts one past block 1 ("hello")
    let session = SpellSession::new(&doc);
    session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
    assert_eq!(
        starts(&session),
        vec![6],
        "wrld sits at char 6 (5 + the 1-char block gap)"
    );
}

#[test]
fn no_checker_clears_the_squiggles() {
    let doc = tiny_doc("helo wrld");
    let session = SpellSession::new(&doc);
    session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
    assert!(!session.last_ranges.borrow().is_empty());
    session.set_checker(None, Color::rgb(220, 50, 50)); // degrade
    assert!(
        session.last_ranges.borrow().is_empty(),
        "no checker → no ranges, session kept"
    );
}

#[test]
fn an_idle_tick_is_a_no_op() {
    let doc = tiny_doc("helo wrld");
    let session = SpellSession::new(&doc);
    session.set_checker(Some(en_checker()), Color::rgb(220, 50, 50));
    let before = session.last_ranges.borrow().clone();
    session.tick(); // nothing dirty
    assert_eq!(
        *session.last_ranges.borrow(),
        before,
        "an idle tick changes nothing"
    );
}
