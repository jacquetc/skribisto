// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Turning a name the writer typed into an id a config file can key on.

/// Every non-alphanumeric run becomes `-`, the ends are trimmed, and an empty
/// result falls back to `fallback`.
///
/// The fallback is a parameter because that is the *only* thing the two callers
/// disagreed about: the export-styles pane said `"export-style"` and the
/// distraction-free themes pane said `"theme"`, and the rest of both functions
/// was identical.
///
/// Note this is **not** the rule `new_work` uses to turn a project name into a
/// folder name. That one is a different algorithm for a different problem — it
/// works against a set of characters the filesystem refuses rather than towards
/// something readable in a TOML key — and merging the two would break one of
/// them. It stays where it is, under its own name.
pub fn slugify(name: &str, fallback: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    let trimmed = s.trim_matches('-').to_string();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spaces_and_punctuation_become_single_trimmed_dashes() {
        assert_eq!(slugify("My Style!", "x"), "My-Style");
        assert_eq!(
            slugify("  leading and trailing  ", "x"),
            "leading-and-trailing"
        );
    }

    #[test]
    fn a_name_with_nothing_alphanumeric_falls_back_to_the_callers_word() {
        assert_eq!(slugify("---", "export-style"), "export-style");
        assert_eq!(slugify("", "theme"), "theme");
    }

    #[test]
    fn non_ascii_letters_are_kept_because_they_are_alphanumeric() {
        assert_eq!(slugify("Épreuve finale", "x"), "Épreuve-finale");
    }
}
