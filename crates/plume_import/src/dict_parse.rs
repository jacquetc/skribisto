// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Parse `dicts/userDict.dict_plume` — Plume's personal spell-check word list,
//! a `;`-separated string.

/// Split the raw dictionary file into de-duplicated, non-empty words.
pub fn parse(raw: &str) -> Vec<String> {
    let mut words = Vec::new();
    for token in raw.split(';') {
        let word = token.trim();
        if !word.is_empty() && !words.iter().any(|w| w == word) {
            words.push(word.to_string());
        }
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_and_trims() {
        assert_eq!(
            parse("nanites;*$;"),
            vec!["nanites".to_string(), "*$".to_string()]
        );
        assert_eq!(parse(""), Vec::<String>::new());
        assert_eq!(parse("  a ; b ;a;"), vec!["a".to_string(), "b".to_string()]);
    }
}
