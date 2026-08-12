// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use std::collections::BTreeMap;

/// Every default must survive its own validator. A mismatch here means the table
/// declares a type it does not actually hold — which would surface as a startup
/// rejection of a perfectly good pins file.
#[test]
fn defaults_are_valid() {
    for spec in SETTINGS {
        let value = (spec.default)();
        assert!(
            (spec.check)(&value).is_ok(),
            "{}: default {value} fails its own check ({})",
            spec.key,
            (spec.check)(&value).unwrap_err()
        );
    }
}

#[test]
fn keys_are_unique() {
    let mut seen = BTreeMap::new();
    for (i, spec) in SETTINGS.iter().enumerate() {
        if let Some(first) = seen.insert(spec.key, i) {
            panic!("{} is registered twice (entries {first} and {i})", spec.key);
        }
    }
}

/// Collect every `*_KEY: &str = "…"` declared anywhere under `src/`.
///
/// A directory walk rather than a fixed `include_str!` list: the point of the drift
/// test is to catch a key added in a file nobody thought to add here, and a hard-coded
/// file list is blind to exactly that case. Handles the multi-line form
/// (`pub const X: &str =\n    "…";`) by scanning forward to the next string literal.
fn declared_keys() -> BTreeMap<String, String> {
    fn walk(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }

    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    walk(&src, &mut files);

    let mut found = BTreeMap::new();
    for file in files {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        for (index, _) in text.match_indices("_KEY: &str =") {
            // A declaration, not prose that quotes the pattern — including this
            // scanner's own doc comment and the literal on the line above. The
            // previous guard skipped the file called `settings_keys.rs`, which
            // stopped working the moment the scanner moved into a `tests.rs`
            // beside it, and never covered a comment in any other file at all.
            let line_start = text[..index].rfind('\n').map_or(0, |i| i + 1);
            let before = text[line_start..index].trim_start();
            if !before.starts_with("const ")
                && !before.starts_with("pub const ")
                && !before.starts_with("pub(crate) const ")
            {
                continue;
            }
            let tail = &text[index..];
            let Some(open) = tail.find('"') else { continue };
            let Some(close) = tail[open + 1..].find('"') else {
                continue;
            };
            let key = &tail[open + 1..open + 1 + close];
            let name_start = text[..index]
                .rfind(|c: char| c.is_whitespace())
                .map(|i| i + 1)
                .unwrap_or(0);
            let name = format!("{}_KEY", &text[name_start..index]);
            found.insert(key.to_string(), name);
        }
    }
    found
}

/// The registry is hand-written and the app's keys are not: this is what keeps them
/// in step. A key declared in the crate but missing here is unpinnable and invisible
/// to `--dump-config`, silently — the exact failure mode this module exists to remove.
#[test]
fn every_declared_key_is_registered() {
    let declared = declared_keys();
    assert!(
        declared.len() > 50,
        "the source scan found only {} keys — the scan itself is broken",
        declared.len()
    );

    let missing: Vec<String> = declared
        .iter()
        .filter(|(key, _)| spec(key).is_none())
        .map(|(key, name)| format!("  {key}  (declared as {name})"))
        .collect();

    assert!(
        missing.is_empty(),
        "these settings keys are declared in the crate but missing from SETTINGS:\n{}\n\
             Add a SettingSpec for each, or --config and --dump-config will not see them.",
        missing.join("\n")
    );
}

/// The other direction. A registered key that no longer exists in the app is dead
/// weight that `--dump-config` still advertises as settable.
///
/// Five keys are legitimately not declared as `*_KEY` constants and are exempt: the
/// four `editor.last_view.*` (inline literals in `EditorViewMemory::new`) and
/// teksilo's own `accessibility.text_scale` (a `SettingsKey<f32>` in the framework).
#[test]
fn every_registered_key_still_exists() {
    let declared = declared_keys();
    let exempt = [
        "editor.last_view.book",
        "editor.last_view.part",
        "editor.last_view.chapter",
        "editor.last_view.note",
        teksilo::settings::TEXT_SCALE_KEY.key,
    ];

    let orphans: Vec<&str> = SETTINGS
        .iter()
        .map(|s| s.key)
        .filter(|k| !declared.contains_key(*k) && !exempt.contains(k))
        .collect();

    assert!(
        orphans.is_empty(),
        "these keys are registered but no longer declared anywhere in the crate: {orphans:?}"
    );
}

#[test]
fn a_dotted_key_round_trips_through_the_tree() {
    let mut root = toml::Value::Table(toml::Table::new());
    insert(&mut root, "editor.scene.size", toml::Value::Float(1.25));
    insert(&mut root, "editor.autosave", toml::Value::Boolean(true));

    assert_eq!(
        lookup(&root, "editor.scene.size"),
        Some(&toml::Value::Float(1.25))
    );
    assert_eq!(
        lookup(&root, "editor.autosave"),
        Some(&toml::Value::Boolean(true))
    );
    // Sibling keys under one prefix must coexist — the whole file is one tree.
    assert!(lookup(&root, "editor").unwrap().is_table());
}

/// A stale scalar where a table now belongs is overwritten, not fatal.
#[test]
fn a_scalar_in_the_way_is_replaced_by_a_table() {
    let mut root = toml::Value::Table(toml::Table::new());
    insert(&mut root, "editor", toml::Value::Boolean(true));
    insert(&mut root, "editor.autosave", toml::Value::Boolean(true));
    assert_eq!(
        lookup(&root, "editor.autosave"),
        Some(&toml::Value::Boolean(true))
    );
}

#[test]
fn flatten_produces_dotted_keys() {
    let parsed: toml::Value = toml::from_str(
        "[editor]\nautosave = true\n[editor.scene]\nsize = 1.5\n[ui]\ndark = true\n",
    )
    .unwrap();
    let mut out = Vec::new();
    flatten("", &parsed, &mut out);
    let keys: Vec<&str> = out.iter().map(|(k, _)| k.as_str()).collect();
    assert!(keys.contains(&"editor.autosave"));
    assert!(keys.contains(&"editor.scene.size"));
    assert!(keys.contains(&"ui.dark"));
    assert_eq!(keys.len(), 3, "only leaves, never the tables above them");
}

/// Both spellings a caller might reach for must parse to the same pins, or the
/// dotted-key form `dump` emits would not be re-readable by `--config`.
#[test]
fn dotted_and_sectioned_pins_are_equivalent() {
    let dir = tempfile::tempdir().unwrap();

    let dotted = dir.path().join("dotted.toml");
    std::fs::write(&dotted, "editor.autosave = true\nui.dark = true\n").unwrap();

    let sectioned = dir.path().join("sectioned.toml");
    std::fs::write(&sectioned, "[editor]\nautosave = true\n[ui]\ndark = true\n").unwrap();

    assert_eq!(load_pins(&dotted).unwrap(), load_pins(&sectioned).unwrap());
}

#[test]
fn an_unknown_key_is_rejected_with_a_suggestion() {
    let dir = tempfile::tempdir().unwrap();
    let pins = dir.path().join("pins.toml");
    std::fs::write(&pins, "ui.local = \"fr-FR\"\n").unwrap();

    let err = load_pins(&pins).unwrap_err();
    assert!(err.contains("unknown setting `ui.local`"), "{err}");
    assert!(err.contains("did you mean `ui.locale`"), "{err}");
}

/// Nothing in the table is within typo range of this, so the message must stand on
/// its own rather than suggest an unrelated key.
#[test]
fn a_wild_key_is_rejected_without_a_bogus_suggestion() {
    let dir = tempfile::tempdir().unwrap();
    let pins = dir.path().join("pins.toml");
    std::fs::write(&pins, "backup.retention.daily = 7\n").unwrap();

    let err = load_pins(&pins).unwrap_err();
    assert!(
        err.contains("unknown setting `backup.retention.daily`"),
        "{err}"
    );
    assert!(
        !err.contains("did you mean"),
        "no near neighbour exists: {err}"
    );
}

/// A bare `1` for a float key is accepted, because `SettingsStore::signal`'s
/// `T::deserialize` accepts it. The check must not be stricter than the store, or it
/// would refuse a pins file that would have worked perfectly.
#[test]
fn an_integer_is_accepted_where_the_store_accepts_one() {
    let dir = tempfile::tempdir().unwrap();
    let pins = dir.path().join("pins.toml");
    std::fs::write(
        &pins,
        "editor.scene.size = 1\naccessibility.text_scale = 2\n",
    )
    .unwrap();
    assert!(load_pins(&pins).is_ok(), "{:?}", load_pins(&pins).err());
}

#[test]
fn a_mistyped_value_is_rejected_naming_the_expected_type() {
    let dir = tempfile::tempdir().unwrap();
    let pins = dir.path().join("pins.toml");
    std::fs::write(&pins, "editor.autosave = \"yes\"\n").unwrap();

    let err = load_pins(&pins).unwrap_err();
    assert!(err.contains("`editor.autosave` expects bool"), "{err}");
}

/// An enum takes its variant name; a wrong one must name the legal set, since that is
/// the whole reason `ty` is prose and not a Rust type name.
#[test]
fn an_unknown_enum_variant_is_rejected_naming_the_variants() {
    let dir = tempfile::tempdir().unwrap();
    let pins = dir.path().join("pins.toml");
    std::fs::write(&pins, "editor.highlight_scope = \"Line\"\n").unwrap();

    let err = load_pins(&pins).unwrap_err();
    assert!(
        err.contains("None | Sentence | Paragraph"),
        "the error must list the legal variants: {err}"
    );
}

/// Every problem at once, so a pins file takes one round trip to fix rather than one
/// per mistake.
#[test]
fn every_problem_is_reported_together() {
    let dir = tempfile::tempdir().unwrap();
    let pins = dir.path().join("pins.toml");
    std::fs::write(
        &pins,
        "ui.dark = \"yes\"\nui.nonsense = 1\neditor.autosave = 3\n",
    )
    .unwrap();

    let err = load_pins(&pins).unwrap_err();
    assert!(err.contains("ui.dark"), "{err}");
    assert!(err.contains("ui.nonsense"), "{err}");
    assert!(err.contains("editor.autosave"), "{err}");
}

#[test]
fn valid_pins_survive_the_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let pins = dir.path().join("pins.toml");
    std::fs::write(
        &pins,
        "ui.dark = true\nui.locale = \"fr-FR\"\neditor.scene.size = 1.25\n\
             editor.highlight_scope = \"Paragraph\"\n",
    )
    .unwrap();

    let general = dir.path().join("general.toml");
    merge_into(&general, &load_pins(&pins).unwrap()).unwrap();

    let written: toml::Value = toml::from_str(&std::fs::read_to_string(&general).unwrap()).unwrap();
    assert_eq!(
        lookup(&written, "ui.dark"),
        Some(&toml::Value::Boolean(true))
    );
    assert_eq!(
        lookup(&written, "ui.locale"),
        Some(&toml::Value::String("fr-FR".into()))
    );
    assert_eq!(
        lookup(&written, "editor.scene.size"),
        Some(&toml::Value::Float(1.25))
    );
    assert_eq!(
        lookup(&written, "editor.highlight_scope"),
        Some(&toml::Value::String("Paragraph".into()))
    );
}

/// Pins overwrite the keys they name and leave every other key alone — the file is a
/// merge target, not a replacement.
#[test]
fn merging_preserves_unrelated_keys() {
    let dir = tempfile::tempdir().unwrap();
    let general = dir.path().join("general.toml");
    std::fs::write(
        &general,
        "[ui]\ndark = false\nlocale = \"en-US\"\n[editor]\nautosave = true\n",
    )
    .unwrap();

    let pins = dir.path().join("pins.toml");
    std::fs::write(&pins, "ui.dark = true\n").unwrap();
    merge_into(&general, &load_pins(&pins).unwrap()).unwrap();

    let written: toml::Value = toml::from_str(&std::fs::read_to_string(&general).unwrap()).unwrap();
    assert_eq!(
        lookup(&written, "ui.dark"),
        Some(&toml::Value::Boolean(true))
    );
    assert_eq!(
        lookup(&written, "ui.locale"),
        Some(&toml::Value::String("en-US".into())),
        "an unpinned key must survive"
    );
    assert_eq!(
        lookup(&written, "editor.autosave"),
        Some(&toml::Value::Boolean(true)),
        "so must a key in another section"
    );
}

/// A missing settings file is the normal case for a fresh sandbox, not an error.
#[test]
fn merging_into_a_missing_file_creates_it() {
    let dir = tempfile::tempdir().unwrap();
    let general = dir.path().join("nested").join("general.toml");

    let pins = dir.path().join("pins.toml");
    std::fs::write(&pins, "ui.dark = true\n").unwrap();
    merge_into(&general, &load_pins(&pins).unwrap()).unwrap();

    let written: toml::Value = toml::from_str(&std::fs::read_to_string(&general).unwrap()).unwrap();
    assert_eq!(
        lookup(&written, "ui.dark"),
        Some(&toml::Value::Boolean(true))
    );
}

/// The load-bearing property of `dump`: its output is a legal pins file. If it ever
/// stops being one, the documented workflow ("dump, delete lines, pass it back")
/// silently breaks.
#[test]
fn a_dump_is_a_valid_pins_file() {
    let _guard = crate::settings_ext::lock_registry();
    let dir = tempfile::tempdir().unwrap();
    let general = dir.path().join("general.toml");
    std::fs::write(&general, "[ui]\ndark = true\n").unwrap();

    let rendered = dir.path().join("dumped.toml");
    std::fs::write(&rendered, dump(&general)).unwrap();

    let pins = load_pins(&rendered).expect("a dump must be re-readable as pins");
    assert_eq!(
        pins.len(),
        SETTINGS.len(),
        "a dump lists every key exactly once"
    );
    let dark = pins.iter().find(|(k, _)| k == crate::DARK_KEY).unwrap();
    assert_eq!(dark.1, toml::Value::Boolean(true), "the disk value wins");
}

#[test]
fn a_dump_marks_which_values_came_from_disk() {
    let dir = tempfile::tempdir().unwrap();
    let general = dir.path().join("general.toml");
    std::fs::write(&general, "[ui]\ndark = true\n").unwrap();

    let text = dump(&general);
    let dark_line = format!("{} = true", crate::DARK_KEY);
    let dark_at = text.find(&dark_line).expect("ui.dark must be listed");
    assert!(
        text[..dark_at].ends_with("bool — set\n"),
        "a key present on disk is marked `set`"
    );
    assert!(
        text.contains("— default"),
        "keys absent from disk are marked `default`"
    );
}

/// A dump against a config directory that does not exist yet must still work: it is
/// the first thing an agent runs, quite possibly before ever launching the app.
#[test]
fn a_dump_of_a_missing_file_is_all_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let text = dump(&dir.path().join("absent.toml"));
    assert!(!text.contains("— set"), "nothing can have been set");
    assert!(text.contains(&format!("{} = false", crate::DARK_KEY)));
}
