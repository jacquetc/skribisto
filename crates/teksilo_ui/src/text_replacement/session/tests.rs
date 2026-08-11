// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

/// The revert gate, stated as the pure predicate the impure path applies.
/// `try_revert` is not directly testable headless (it needs a live
/// `EditorHandle`), but its decision is exactly these two comparisons, and
/// they are what a wrong caret or an unrelated same-length edit has to fail.
fn revert_applies(pending: &PendingRevert, caret: usize, before: &str) -> bool {
    caret == pending.span_start + pending.replacement_chars && before == pending.replacement
}

fn pending() -> PendingRevert {
    PendingRevert {
        span_start: 4,
        replacement_chars: 10,
        replacement: "by the way".into(),
        typed: "btw".into(),
        revision: 7,
        collapse: false,
    }
}

/// The case the feature exists for: "say btw " expanded, the writer hit
/// Backspace, so the delimiter is gone and the caret sits at the end of the
/// replacement.
#[test]
fn deleting_the_delimiter_right_after_a_fire_reverts() {
    assert!(revert_applies(&pending(), 14, "by the way"));
}

/// The writer kept typing instead — the caret is past where a revert could
/// apply, so the expansion stands.
#[test]
fn typing_on_after_a_fire_does_not_revert() {
    assert!(!revert_applies(&pending(), 15, "by the way "));
}

/// A caret in the right place is not enough on its own: an edit elsewhere
/// can leave it there, and reverting then would rewrite text the fire never
/// touched.
#[test]
fn a_matching_caret_over_different_text_does_not_revert() {
    assert!(!revert_applies(&pending(), 14, "by the wax"));
}

/// Deleting further back moves the caret out of the span.
#[test]
fn deleting_into_the_replacement_does_not_revert() {
    assert!(!revert_applies(&pending(), 13, "by the wa"));
}

/// The suppression is keyed case-insensitively, matching the engine: the
/// writer who reverted "BTW" must not have "btw" re-expand at that spot.
#[test]
fn the_suppression_key_is_case_insensitive() {
    let s = Suppressed {
        span_start: 4,
        key: "btw".into(),
    };
    assert_eq!(s.key, "BTW".to_lowercase());
}

/// The revert restores the trigger *without* the delimiter the backspace
/// consumed. That is what keeps the restore from re-firing: the text now
/// ends in a word character, so the engine has nothing to match.
#[test]
fn the_restored_text_cannot_immediately_re_fire() {
    use crate::models::TextReplacementRuleRow;
    let engine = TextReplacementEngine::from_rules(&[TextReplacementRuleRow {
        id: 0,
        trigger: "btw".into(),
        replacement: "by the way".into(),
        enabled: true,
    }]);
    // What the document reads as after a revert of "say btw ".
    assert_eq!(engine.check("say btw"), None);
}
