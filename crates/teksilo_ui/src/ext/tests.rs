// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use std::path::{Path, PathBuf};

/// Registration functions that are **not** extension slots, with the reason.
///
/// Anything `pub fn register…` in this crate that is not listed here is assumed
/// to be a slot and must be reachable through `ext`. Getting on this list should
/// take an argument, which is why each entry carries one.
const NOT_A_SLOT: &[(&str, &str)] = &[
    (
        "register_all_extension_commands",
        "the app calls this to install what extensions registered; an extension never calls it",
    ),
    (
        "register_referenced",
        "image reference counting inside the editor, unrelated to the seam",
    ),
];

fn crate_src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Every `pub fn register…` declared at the top level of a module, paired with
/// the file that declares it.
fn declared_registrars() -> Vec<(String, PathBuf)> {
    let mut files = Vec::new();
    rust_files(&crate_src(), &mut files);
    files.sort();

    let mut found = Vec::new();
    for file in files {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        for line in text.lines() {
            // A declaration at column zero. `pub use` re-exports (this module is
            // nothing but those) do not match, and neither does an indented
            // method on some unrelated builder.
            let Some(rest) = line.strip_prefix("pub fn register") else {
                continue;
            };
            // A word boundary, or the readers next door (`registered_pages`,
            // `registered_for`, …) come back as slots. They are the app's side
            // of each registry, not an extension's.
            if !rest.starts_with('(') && !rest.starts_with('_') {
                continue;
            }
            let name: String = std::iter::once("register")
                .chain(std::iter::once(
                    rest.split(|c: char| !c.is_alphanumeric() && c != '_')
                        .next()
                        .unwrap_or(""),
                ))
                .collect();
            found.push((name, file.clone()));
        }
    }
    found
}

/// A slot nobody re-exported is a slot a downstream edition has to reach by its
/// internal path — which is exactly what `ext` exists to stop, and exactly what
/// happens by default, since adding a registry and updating this module are two
/// separate acts. One of the nine arrived while this refactor was in flight.
#[test]
fn every_registration_slot_is_reachable_through_ext() {
    let ext = std::fs::read_to_string(crate_src().join("ext.rs")).expect("read ext.rs");
    let excused: Vec<&str> = NOT_A_SLOT.iter().map(|(n, _)| *n).collect();

    let missing: Vec<String> = declared_registrars()
        .into_iter()
        .filter(|(name, _)| !excused.contains(&name.as_str()))
        .filter(|(name, _)| !ext.contains(name.as_str()))
        .map(|(name, file)| format!("  {name}  (declared in {})", file.display()))
        .collect();

    assert!(
        missing.is_empty(),
        "these registration slots are not re-exported from `ext`:\n{}\n\
         Add a `pub use` for each, or list it in NOT_A_SLOT with the reason it \
         is not part of the seam.",
        missing.join("\n")
    );
}

/// The excuse list must not outlive what it excuses: a stale entry silently
/// widens the check's blind spot.
#[test]
fn nothing_on_the_not_a_slot_list_has_been_deleted() {
    let declared: Vec<String> = declared_registrars().into_iter().map(|(n, _)| n).collect();
    for (name, _why) in NOT_A_SLOT {
        assert!(
            declared.contains(&name.to_string()),
            "NOT_A_SLOT still excuses `{name}`, which no longer exists"
        );
    }
}

/// The scan is the whole test; if it stops finding anything it passes silently.
#[test]
fn the_scan_itself_finds_the_slots() {
    let names: Vec<String> = declared_registrars().into_iter().map(|(n, _)| n).collect();
    assert!(
        names.len() >= 9,
        "the scan found only {} registration functions, so it is broken: {names:?}",
        names.len()
    );
    for expected in [
        "register_dock",
        "register_inspector_section",
        "register_container_segment",
        "register_category",
        "register_command",
        "register_settings",
        "register_page",
        "register_locales",
    ] {
        assert!(
            names.contains(&expected.to_string()),
            "scan missed {expected}"
        );
    }
}
