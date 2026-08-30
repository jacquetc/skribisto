// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading `characters/*.txt`.
//!
//! These files use the same metadata header as an outline item, but their keys are
//! **human-readable display strings** rather than field names — `Phrase Summary`,
//! not `summarySentence`. Two conventions in one format, and the display strings
//! are hard-coded English that Manuskript never translates, so a French project's
//! character files say `Phrase Summary` while its `status.txt` says `Premier jet`.
//! Matching on English is correct here and wrong there.
//!
//! ⚠ `Color` is genuinely ambiguous, by design. The **first** one is the swatch
//! beside the name; a later one is a field the writer added called "Color". The
//! writer emits the swatch before the user fields, so first-wins is the rule, and
//! it is the rule Manuskript reads by.
//!
//! A user field whose name collides with a built-in ("Goal", "Notes") overwrites
//! that built-in on read, in Manuskript as here — the format cannot tell them
//! apart, so neither can a reader.

use crate::mmd;
use crate::model::Character;
use crate::source::ManuskriptSource;

/// The directory every character file lives in.
pub const CHARACTERS_DIR: &str = "characters/";
/// The swatch key, and the one key whose first occurrence wins.
const COLOR_KEY: &str = "Color";

/// The on-disk key for each built-in field, in the writer's own order.
const BUILT_IN: &[&str] = &[
    "Name",
    "ID",
    "Importance",
    "POV",
    "Motivation",
    "Goal",
    "Conflict",
    "Epiphany",
    "Phrase Summary",
    "Paragraph Summary",
    "Full Summary",
    "Notes",
];

/// Read every character, in the order Manuskript numbered them.
///
/// By the leading number and not by the member path: a character file is
/// `{ID}-{slug}.txt` with **no zero padding**, unlike an outline row, so a plain
/// string sort puts the tenth character between the first and the second. The
/// same comparator the outline uses, for the same reason.
pub fn read(src: &ManuskriptSource, notices: &mut Vec<String>) -> Vec<Character> {
    let mut out = Vec::new();
    let mut members = src.members_under(CHARACTERS_DIR);
    members.sort_by(|a, b| {
        let name = |m: &str| m.rsplit('/').next().unwrap_or(m).to_string();
        super::outline::sibling_order(&name(a), &name(b))
    });
    for member in members {
        if !member.ends_with(".txt") {
            notices.push(format!(
                "'{member}' is not a Manuskript character file and was left out."
            ));
            continue;
        }
        let Some(text) = src.text(member) else {
            continue;
        };
        let file = mmd::parse(&text);
        if file.recovered_trailing_key {
            notices.push(format!(
                "'{member}' ended without a blank line after its metadata. Its last field was \
                 kept; Manuskript loses it."
            ));
        }
        out.push(from_file(&file));
    }
    out
}

fn from_file(file: &mmd::MmdFile) -> Character {
    let mut c = Character::default();
    let mut seen_color = false;
    for entry in &file.entries {
        let key = entry.key.as_str();
        let value = entry.value.clone();
        match key {
            "Name" => c.name = value,
            "ID" => c.id = Some(value).filter(|s| !s.is_empty()),
            "Importance" => c.importance = crate::xml::importance(Some(&value)),
            // Whether Manuskript offers this character in its POV picker. Added in
            // 0.12.0; absent in an older project, which is not the same as false,
            // so it stays an `Option`.
            "POV" => c.pov_enabled = parse_python_bool(&value),
            "Motivation" => c.motivation = value,
            "Goal" => c.goal = value,
            "Conflict" => c.conflict = value,
            "Epiphany" => c.epiphany = value,
            "Phrase Summary" => c.summary_sentence = value,
            "Paragraph Summary" => c.summary_paragraph = value,
            "Full Summary" => c.summary_full = value,
            "Notes" => c.notes = value,
            COLOR_KEY if !seen_color => {
                seen_color = true;
                c.color = value;
            }
            // Everything else, including a second `Color`, is a field the writer
            // added. Kept in file order, and never merged with the built-ins.
            _ => c.infos.push((entry.key.clone(), value)),
        }
    }
    c
}

/// Read the string Python's `str(bool)` produces.
///
/// `None` for anything else, including an absent key: "this project predates the
/// field" and "this character may not hold the camera" are different answers.
fn parse_python_bool(raw: &str) -> Option<bool> {
    match raw.trim() {
        "True" => Some(true),
        "False" => Some(false),
        _ => None,
    }
}

/// Every key a character file may carry that is not a user field.
pub fn known_keys() -> Vec<&'static str> {
    let mut keys = BUILT_IN.to_vec();
    keys.push(COLOR_KEY);
    keys
}

#[cfg(test)]
mod tests {

    /// A character file is `{ID}-{slug}.txt` with no zero padding, so a plain
    /// string sort lists the tenth character between the first and the second.
    #[test]
    fn characters_come_back_in_their_own_numeric_order() {
        let members: Vec<(String, String)> = (0..12)
            .map(|n| {
                (
                    format!("characters/{n}-Person.txt"),
                    format!("Name:                Person {n}\nID:                  {n}\n"),
                )
            })
            .collect();
        let borrowed: Vec<(&str, &str)> = members
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let src = ManuskriptSource::for_tests(&borrowed);
        let mut notices = Vec::new();
        let names: Vec<String> = read(&src, &mut notices)
            .into_iter()
            .map(|c| c.name)
            .collect();
        assert_eq!(names.first().map(String::as_str), Some("Person 0"));
        assert_eq!(names.get(1).map(String::as_str), Some("Person 1"));
        assert_eq!(
            names.get(2).map(String::as_str),
            Some("Person 2"),
            "not Person 10"
        );
        assert_eq!(names.last().map(String::as_str), Some("Person 11"));
    }

    use super::*;

    /// The shape `formatMetaData(key, value, 20)` writes.
    fn file(pairs: &[(&str, &str)]) -> mmd::MmdFile {
        let text: String = pairs
            .iter()
            .map(|(k, v)| format!("{k}:{}{v}\n", " ".repeat(20usize.saturating_sub(k.len()))))
            .collect();
        mmd::parse(&text)
    }

    #[test]
    fn the_display_string_keys_map_onto_the_fields_they_name() {
        let c = from_file(&file(&[
            ("Name", "Peter"),
            ("ID", "0"),
            ("Importance", "2"),
            ("POV", "True"),
            ("Motivation", "To be understood"),
            ("Goal", "Preach"),
            ("Conflict", "His own doubt"),
            ("Epiphany", "The sheet from heaven"),
            ("Phrase Summary", "A fisherman."),
            ("Paragraph Summary", "A fisherman who leads."),
            ("Full Summary", "A long telling."),
            ("Notes", "Speaks first, thinks later."),
            ("Color", "#ff0000"),
        ]));
        assert_eq!(c.name, "Peter");
        assert_eq!(c.id.as_deref(), Some("0"));
        assert_eq!(c.importance, Some(2));
        assert_eq!(c.pov_enabled, Some(true));
        assert_eq!(c.color, "#ff0000");
        assert_eq!(c.motivation, "To be understood");
        assert_eq!(c.epiphany, "The sheet from heaven");
        assert_eq!(c.summary_sentence, "A fisherman.");
        assert_eq!(c.summary_paragraph, "A fisherman who leads.");
        assert_eq!(c.summary_full, "A long telling.");
        assert_eq!(c.notes, "Speaks first, thinks later.");
        assert!(c.infos.is_empty());
    }

    /// The ambiguity the format cannot resolve any other way: the first `Color` is
    /// the swatch, a later one is a field the writer added.
    #[test]
    fn the_first_colour_is_the_swatch_and_a_later_one_is_a_user_field() {
        let c = from_file(&file(&[
            ("Name", "Alice"),
            ("Color", "#00ff00"),
            ("Color", "blue eyes"),
        ]));
        assert_eq!(c.color, "#00ff00");
        assert_eq!(c.infos, [("Color".to_string(), "blue eyes".to_string())]);
    }

    #[test]
    fn user_fields_keep_their_names_and_their_order() {
        let c = from_file(&file(&[
            ("Name", "Alice"),
            ("Age", "30"),
            ("Color", "#00ff00"),
            ("Hometown", "Bristol"),
        ]));
        assert_eq!(
            c.infos,
            [
                ("Age".to_string(), "30".to_string()),
                ("Hometown".to_string(), "Bristol".to_string()),
            ]
        );
    }

    /// Added in 0.12.0, so an older project has no answer — which is not "no".
    #[test]
    fn an_absent_pov_flag_is_unknown_rather_than_false() {
        assert_eq!(from_file(&file(&[("Name", "A")])).pov_enabled, None);
        assert_eq!(
            from_file(&file(&[("Name", "A"), ("POV", "False")])).pov_enabled,
            Some(false)
        );
        // Anything that is not Python's own spelling is unknown, not false.
        assert_eq!(
            from_file(&file(&[("Name", "A"), ("POV", "yes")])).pov_enabled,
            None
        );
    }

    /// The format cannot tell a user field named "Goal" from the built-in one, and
    /// neither can Manuskript. Recorded so the behaviour is deliberate.
    #[test]
    fn a_user_field_colliding_with_a_built_in_overwrites_it() {
        let c = from_file(&file(&[("Goal", "Preach"), ("Goal", "Rest")]));
        assert_eq!(c.goal, "Rest");
        assert!(c.infos.is_empty());
    }

    #[test]
    fn an_importance_outside_the_scale_is_no_importance() {
        assert_eq!(from_file(&file(&[("Importance", "9")])).importance, None);
        assert_eq!(from_file(&file(&[("Importance", "")])).importance, None);
        assert_eq!(from_file(&file(&[("Importance", "0")])).importance, Some(0));
    }

    #[test]
    fn the_known_key_set_covers_every_built_in_and_the_swatch() {
        let keys = known_keys();
        assert!(keys.contains(&"Phrase Summary"));
        assert!(keys.contains(&COLOR_KEY));
        assert_eq!(keys.len(), BUILT_IN.len() + 1);
    }
}
