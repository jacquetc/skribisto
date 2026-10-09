// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Writes the Scrivener projects the live-app probe `scripts/automation_import_scrivener.py`
//! imports, into a folder it names:
//!
//! ```text
//! cargo run -p skribisto-scrivener-fixtures --example write_fixtures -- <folder> [--scale N]
//! ```
//!
//! One project per situation the wizard has to meet, each in a folder of its own, and a
//! line per project on standard output, `<name>\t<path>`:
//!
//! * `novel`: the Scrivener 3 novel, with metadata, one of whose scenes holds the phrases a
//!   Djot round trip once changed (`10:30:45`, `--`, straight quotes, a paragraph opening
//!   with a letter and a full stop), for the probe's edit-and-reopen check;
//! * `manifests`: the same project with two binder files and nothing to tell them apart;
//! * `upgrade`: the copy Scrivener kept from before an upgrade, beside the upgraded
//!   project (`upgrade-current`), which the wizard reads in its place;
//! * `ios`: a project with edits from an iPhone or iPad not yet merged;
//! * `scrivener1`: a Scrivener 1 for Mac project, which the import refuses;
//! * `large`: the novel with `N` more scenes (500 by default; the probe asks for 3,000),
//!   long enough to cancel;
//! * `s2`: the Scrivener 3 novel with every kind of metadata the second half of the mapping
//!   reads ([`Spec::with_s2`]): keywords, custom metadata, collections, bookmarks, targets, a
//!   deadline still ahead, an auto-complete list, compile replacements and icons.
//!
//! Test tooling: every project is invented ([`scrivener_fixtures::text`]), nothing is
//! copied from a real one.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use scrivener_fixtures::{Files, Generation, Item, Kind, Platform, Spec};

/// The phrases the editor's Djot round trip once rewrote on the first save of an edited
/// scene, each where it would trip it.
const TRIGGERS: [&str; 3] = [
    "We met at 10:30:45 sharp, and he wrote--with care--a short note.",
    "He said \"hi\" and 'bye' before the ferry left.",
    "S. Russell's view of the harbour was plain enough.",
];

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(out) = args.next().map(PathBuf::from) else {
        eprintln!("usage: write_fixtures <folder> [--scale N]");
        return ExitCode::from(2);
    };
    let mut scale = 500usize;
    while let Some(arg) = args.next() {
        match (arg.as_str(), args.next().map(|n| n.parse::<usize>())) {
            ("--scale", Some(Ok(n))) => scale = n,
            _ => {
                eprintln!("usage: write_fixtures <folder> [--scale N]");
                return ExitCode::from(2);
            }
        }
    }
    match write_all(&out, scale) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("writing the fixtures into {}: {e}", out.display());
            ExitCode::FAILURE
        }
    }
}

fn write_all(out: &Path, scale: usize) -> std::io::Result<()> {
    let novel = novel();
    say(
        "novel",
        &write(&novel.build(), out, "novel", &novel.folder())?,
    );

    let mut files = novel.build();
    files.remove("Files/binder.autosave");
    let manifest = files.remove(&novel.manifest_name()).unwrap_or_default();
    let text = String::from_utf8_lossy(&manifest).into_owned();
    // Neither copy carries the folder's name, nor says when it was saved.
    let undated = |t: &str| t.replace("Modified=\"2024-06-02 18:40:12 +0200\" ", "");
    files.put("Draft A.scrivx", undated(&text).into_bytes());
    files.put(
        "Draft B.scrivx",
        undated(&text.replace("<Title>Arrival</Title>", "<Title>Arrival B</Title>")).into_bytes(),
    );
    say(
        "manifests",
        &write(&files, out, "manifests", &novel.folder())?,
    );

    let mut old = Spec::novel(Generation::Format10, Platform::Mac);
    old.name = "Lighthouse Backup".to_string();
    old.manifest_stem = Some("Lighthouse".to_string());
    let upgraded = Spec::novel(Generation::Format20, Platform::Mac);
    // The upgraded project is a source the wizard reads too, through its question: the
    // probe digests it with the others.
    say(
        "upgrade-current",
        &write(&upgraded.build(), out, "upgrade", &upgraded.folder())?,
    );
    say(
        "upgrade",
        &write(&old.build(), out, "upgrade", &old.folder())?,
    );

    let pending = Spec::novel(Generation::Format15, Platform::Mac);
    let mut files = pending.build();
    let ipad = Spec::novel(Generation::IosStub, Platform::Ios).build();
    files.put(
        "Mobile/binder.mob",
        ipad.get("Mobile/binder.mob").unwrap_or_default().to_vec(),
    );
    say("ios", &write(&files, out, "ios", &pending.folder())?);

    let one = Spec::novel(Generation::ScrivenerOne, Platform::Mac);
    say(
        "scrivener1",
        &write(&one.build(), out, "scrivener1", &one.folder())?,
    );

    let s2 = Spec::novel(Generation::Format20, Platform::Mac)
        .with_metadata()
        .with_s2();
    say("s2", &write(&s2.build(), out, "s2", &s2.folder())?);

    let large = large(scale);
    say(
        "large",
        &write(&large.build(), out, "large", &large.folder())?,
    );
    Ok(())
}

/// The Scrivener 3 novel with its metadata, *The Harbour* holding the trigger phrases.
fn novel() -> Spec {
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac).with_metadata();
    if let Some(index) = spec.index_of("The Harbour") {
        spec.items[index].text = TRIGGERS.iter().map(|p| p.to_string()).collect();
    }
    spec
}

/// The novel with `scale` more scenes in a third chapter.
fn large(scale: usize) -> Spec {
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    let at = spec.index_of("Research").unwrap_or(spec.items.len());
    let mut more = vec![Item::new(2, Kind::Folder, "Chapter 3").compiled()];
    more.extend((0..scale).map(|i| {
        Item::new(3, Kind::Text, &format!("Crossing {i}"))
            .compiled()
            .with_text(i, 8)
            .with_synopsis(i)
    }));
    spec.items.splice(at..at, more);
    spec
}

/// Write `files` as `<out>/<group>/<folder>`, and return where.
fn write(files: &Files, out: &Path, group: &str, folder: &str) -> std::io::Result<PathBuf> {
    let path = out.join(group).join(folder);
    files.write_to(&path)?;
    Ok(path)
}

fn say(name: &str, path: &Path) {
    println!("{name}\t{}", path.display());
}
