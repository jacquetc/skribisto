// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the format must be true of for **every** project, not for the one fixture.
//!
//! `tests.rs` builds one carefully-shaped bundle — every combination of the constraint
//! matrix, every field deliberately off its default — and checks a write/read cycle against
//! it. That fixture is the right way to prove no *field* is dropped, and it cannot prove
//! anything about the axis where this format has actually gone wrong: the **names**. A
//! project's file names come from its titles, and a title is whatever the writer typed, in
//! whatever script, under whatever Unicode normalisation the machine that last touched the
//! folder happened to use.
//!
//! Three shipped faults sit on that axis, and each is a property rather than a case:
//!
//! * a folder handed over with decomposed accented names would not open, and a read-only
//!   tolerance for it would have had the next save write a precomposed twin beside the file
//!   and prune the original;
//! * a `.skrib` that ended up inside a bundle was carried, so every save copied it forward
//!   and every backup then held all the earlier ones;
//! * an `assets.ron` naming an absolute path read that file into the project.
//!
//! # The fixpoint
//!
//! The strongest statement available here is not that a round trip preserves the data. It is
//! that **saving a project you have not edited changes nothing on disk** — write, read, write
//! again, and the bytes are the same bytes. That is what the exploded shape exists for: a git
//! history where editing one scene touches one file. Anything that accumulates, renames or
//! duplicates across a no-op save breaks it, whatever the in-memory bundle says, which is why
//! the disk is compared directly and not only the value read back.
//!
//! # Why the generator tweaks a fixture rather than building from nothing
//!
//! A `WorkBundle` is only valid if every row satisfies `skribisto_model`'s constraint matrix,
//! so a freely generated one would be rejected for reasons that have nothing to do with what
//! is being tested. `tests::build_bundle_with` supplies the valid scaffolding and this varies
//! what the properties are about: titles, prose, depths, names.

use proptest::prelude::*;
use std::collections::BTreeMap;

use super::bundle::{FORMAT_VERSION, ShapeTag, WorkBundle};
use super::{SkribShape, read_bundle, write_bundle};

/// Strings chosen for where they land rather than for variety.
///
/// Every one of these is a real title someone would type, and each reaches a different part
/// of the naming path: the accented pair is the same text under NFC and NFD, which are two
/// different file names on ext4 and one on APFS; the scripts prove a slug keeps its own
/// characters rather than folding a non-Latin book into `item-item-item`; the punctuation and
/// separators are what a path is not allowed to contain; and the empty and whitespace-only
/// cases are what an untitled scene actually carries.
const TITLES: &[&str] = &[
    "Chapter One",
    "Rapha\u{eb}l et Mire\u{ef}a",     // NFC: ë, ï
    "Raphae\u{308}l et Mirei\u{308}a", // NFD: e + combining diaeresis
    "\u{e9}t\u{e9}",                   // NFC
    "e\u{301}te\u{301}",               // NFD
    "",
    "   ",
    "../../etc/passwd",
    "C:\\Windows\\system32",
    "a/b/c",
    ".",
    "..",
    "CON",
    "\u{6d4b}\u{8bd5}\u{7ae0}\u{8282}",           // Chinese
    "\u{627}\u{644}\u{641}\u{635}\u{644}",        // Arabic
    "\u{5e8}\u{5d0}\u{5e9}\u{5d5}\u{5df}",        // Hebrew
    "\u{41f}\u{440}\u{43e}\u{43b}\u{43e}\u{433}", // Cyrillic
    "\u{1f4d8} Emoji title",
    "trailing space ",
    " leading space",
    "dots...everywhere",
    "a\tb",
    "very long title that goes on and on and on and really ought to be truncated somewhere",
];

fn title_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(TITLES).prop_map(str::to_string)
}

/// Prose worth writing to a file: Djot markers, blank lines, the characters a writer's
/// typography actually produces, and the object replacement character an inline image is.
const PROSE: &[&str] = &[
    "Plain prose.",
    "",
    "# Not a heading, a line of prose\n\nSecond block.",
    "\u{ab} Que dire ? \u{bb} demanda-t-elle.",
    "A line with \u{fffc} an image anchor in it.",
    "* * *",
    "Ligne un\nLigne deux\n\nBloc deux",
    "\u{65e5}\u{672c}\u{8a9e}\u{306e}\u{6587}\u{7ae0}\u{3002}",
    "Trailing whitespace   \n\n   leading",
];

fn prose_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(PROSE).prop_map(str::to_string)
}

fn shape_strategy() -> impl Strategy<Value = (ShapeTag, SkribShape)> {
    prop_oneof![
        Just((ShapeTag::Zip, SkribShape::ZipFile)),
        Just((ShapeTag::Folder, SkribShape::ExplodedFolder)),
    ]
}

/// A valid bundle with generated titles, prose and depths.
///
/// The per-row values are drawn once and applied by index, so a shrunk failure names a
/// specific row rather than a reseeded random stream.
///
/// `accented` forces the first row's title to carry a precomposed accent. The normalisation
/// property needs a file name there is something to decompose *in*, and drawing one by chance
/// would let that property pass on a project whose titles happened to be pure ASCII — a green
/// run proving nothing, which is the failure mode a property test is supposed to remove.
fn bundle_strategy() -> impl Strategy<Value = (WorkBundle, ShapeTag, SkribShape)> {
    bundle_strategy_inner(false)
}

fn accented_bundle_strategy() -> impl Strategy<Value = (WorkBundle, ShapeTag, SkribShape)> {
    bundle_strategy_inner(true)
}

fn bundle_strategy_inner(
    accented: bool,
) -> impl Strategy<Value = (WorkBundle, ShapeTag, SkribShape)> {
    (
        shape_strategy(),
        prop::collection::vec(title_strategy(), 1..=14),
        prop::collection::vec(prose_strategy(), 1..=14),
        title_strategy(),
        title_strategy(),
    )
        .prop_map(
            move |((tag, shape), mut titles, proses, work_title, author)| {
                if accented {
                    titles[0] = "Rapha\u{eb}l et Mire\u{ef}a".to_string();
                }
                let bundle = super::tests::build_bundle_with(tag, |s| {
                    s.work.title = work_title;
                    s.work.author_name = author;
                    let mut n = 0usize;
                    for binder in s.binders.iter_mut() {
                        for item in binder.items.iter_mut() {
                            item.item.title = titles[n % titles.len()].clone();
                            for content in item.contents.iter_mut() {
                                content.data = proses[n % proses.len()].clone();
                            }
                            n += 1;
                        }
                    }
                });
                (bundle, tag, shape)
            },
        )
}

/// Every file under `root`, keyed by its path relative to `root` with `/` separators.
///
/// The comparison unit for the fixpoint. Paths are captured as the **raw bytes on disk**, not
/// normalised, because two spellings of one name is precisely the failure being looked for: a
/// save that writes the precomposed twin of a decomposed file leaves both here and is caught,
/// whereas a normalised key would quietly merge them.
fn snapshot_folder(root: &std::path::Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    for entry in walkdir::WalkDir::new(root).sort_by_file_name() {
        let Ok(entry) = entry else { continue };
        if !entry.file_type().is_file() {
            continue;
        }
        let Ok(rel) = entry.path().strip_prefix(root) else {
            continue;
        };
        let key = rel.to_string_lossy().replace('\\', "/");
        let bytes = std::fs::read(entry.path()).unwrap_or_default();
        out.insert(key, bytes);
    }
    out
}

/// Rewrite every path component under `root` into its decomposed (NFD) spelling.
///
/// What a synchronisation client, an HFS+ volume or a third-party zip tool hands another
/// machine. Deepest first, so renaming a directory never invalidates the paths still to come.
///
/// Returns how many names it rewrote, so a test can refuse to pass on a project that had no
/// accented name in it to begin with.
fn decompose_names(root: &std::path::Path) -> usize {
    use unicode_normalization::UnicodeNormalization;
    let mut paths: Vec<std::path::PathBuf> = walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .map(|e| e.path().to_path_buf())
        .filter(|p| p != root)
        .collect();
    paths.sort_by_key(|p| std::cmp::Reverse(p.components().count()));
    let mut renamed = 0usize;
    for path in paths {
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let nfd: String = name.nfd().collect();
        if nfd == name {
            continue;
        }
        let Some(parent) = path.parent() else {
            continue;
        };
        if std::fs::rename(&path, parent.join(nfd)).is_ok() {
            renamed += 1;
        }
    }
    renamed
}

/// Write `b` at `path` in `shape` and read it straight back.
fn cycle(path: &str, shape: SkribShape, b: &WorkBundle) -> WorkBundle {
    write_bundle(path, shape, b).expect("write");
    read_bundle(path).expect("read")
}

/// The one field the writer legitimately computes rather than copies, neutralised so two
/// bundles can be compared for equality. See `tests::assert_round_trip`.
fn comparable(b: &WorkBundle) -> WorkBundle {
    let mut out = b.clone();
    out.manifest.format_min_read_version = None;
    out
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 48, ..ProptestConfig::default() })]

    /// A project written and read back is the project that was written.
    ///
    /// The fixture in `tests.rs` states this for one set of titles. Here the titles are the
    /// variable, because they are what the file names are made of: the row whose prose a save
    /// cannot find again is the row whose title produced a name the reader spells differently.
    #[test]
    fn a_project_survives_a_write_and_read((bundle, _tag, shape) in bundle_strategy()) {
        let dir = tempfile::tempdir().expect("tmp");
        let path = dir.path().join("Novel.skrib").to_string_lossy().into_owned();
        let back = cycle(&path, shape, &bundle);

        prop_assert_eq!(
            back.manifest.format_min_read_version,
            Some(super::version_gate::compute_min_read_version(&bundle)),
            "the writer must stamp the content-derived read floor"
        );
        prop_assert_eq!(comparable(&back), comparable(&bundle), "the project changed in transit");
    }

    /// **Saving an unedited project changes nothing on disk.**
    ///
    /// The fixpoint, and the strongest thing this format can claim. The exploded shape exists
    /// so that editing one scene touches one file in a git history; a save that rewrites,
    /// renames or accumulates anything it was not asked to breaks that even when every field
    /// still round-trips. Both known instances were invisible in the bundle and visible only
    /// here: a `.skrib` that ended up inside the bundle was carried forward into every later
    /// save, each of which then held all the earlier ones, and a normalisation mismatch had a
    /// save write a second copy of a prose file beside the first.
    ///
    /// Compared as raw on-disk names, deliberately: two spellings of one name is the failure,
    /// so normalising the keys would merge exactly what must be seen apart.
    #[test]
    fn saving_an_unedited_project_is_a_no_op((bundle, _tag, _shape) in bundle_strategy()) {
        let dir = tempfile::tempdir().expect("tmp");
        let path = dir.path().join("Novel").to_string_lossy().into_owned();

        let first = cycle(&path, SkribShape::ExplodedFolder, &bundle);
        let after_first = snapshot_folder(std::path::Path::new(&path));

        write_bundle(&path, SkribShape::ExplodedFolder, &first).expect("rewrite");
        let after_second = snapshot_folder(std::path::Path::new(&path));

        let added: Vec<&String> = after_second
            .keys()
            .filter(|k| !after_first.contains_key(*k))
            .collect();
        let removed: Vec<&String> = after_first
            .keys()
            .filter(|k| !after_second.contains_key(*k))
            .collect();
        prop_assert!(added.is_empty(), "a no-op save added files: {added:?}");
        prop_assert!(removed.is_empty(), "a no-op save removed files: {removed:?}");
        prop_assert_eq!(after_first, after_second, "a no-op save rewrote a file");

        let second = read_bundle(&path).expect("reread");
        prop_assert_eq!(comparable(&second), comparable(&first), "the project drifted on resave");
    }

    /// A folder whose names arrived decomposed still opens, and saving into it renames nothing.
    ///
    /// `slugify` keeps a title's accented letters, so a French or Catalan project has accented
    /// file names, spelled precomposed by the machine that wrote them. Another machine can be
    /// handed the decomposed spelling, and on ext4 or NTFS those are two different names.
    ///
    /// The second half is the part a reader-only tolerance gets wrong, and it is why the
    /// writer goes through `locate` too: if a save wrote the precomposed name it would leave a
    /// twin beside the file and prune the original, and on a normalisation-insensitive
    /// filesystem those two names are one file — whether the prose survived would come down to
    /// the order a sync client replayed a create and a delete.
    #[test]
    fn a_decomposed_folder_opens_and_resaves_in_place(
        (bundle, _tag, _shape) in accented_bundle_strategy()
    ) {
        let dir = tempfile::tempdir().expect("tmp");
        let path = dir.path().join("Novel").to_string_lossy().into_owned();
        let written = cycle(&path, SkribShape::ExplodedFolder, &bundle);

        let renamed = decompose_names(std::path::Path::new(&path));
        prop_assert!(
            renamed > 0,
            "nothing was decomposed, so this case proves nothing about normalisation"
        );
        let after_rename = snapshot_folder(std::path::Path::new(&path));

        let reopened = read_bundle(&path).expect("a decomposed folder must still open");
        prop_assert_eq!(
            comparable(&reopened),
            comparable(&written),
            "a decomposed folder read back as a different project"
        );

        write_bundle(&path, SkribShape::ExplodedFolder, &reopened).expect("resave");
        let after_resave = snapshot_folder(std::path::Path::new(&path));
        let twins: Vec<&String> = after_resave
            .keys()
            .filter(|k| !after_rename.contains_key(*k))
            .collect();
        prop_assert!(
            twins.is_empty(),
            "saving into a decomposed folder wrote precomposed twins: {twins:?}"
        );
        prop_assert_eq!(
            after_resave.keys().collect::<Vec<_>>(),
            after_rename.keys().collect::<Vec<_>>(),
            "saving into a decomposed folder renamed files"
        );
    }

    /// Every path a bundle names stays inside the bundle.
    ///
    /// The whole of `safe_path`'s contract, over arbitrary strings rather than a list of
    /// attacks. `Path::join` with an absolute argument discards the base, so a manifest naming
    /// `/home/someone/.ssh/id_rsa` read that file into the project — which needed no malicious
    /// writer, only a bundle that arrived from somewhere, as a mailed or restored `.skrib`
    /// already has.
    #[test]
    fn a_bundle_path_never_escapes_the_bundle(raw in r"[\PC]{0,40}") {
        let root = std::path::Path::new("/tmp/bundle-root");
        match super::safe_path::bundle_relative(&raw) {
            Err(_) => {}
            Ok(rel) => {
                prop_assert!(rel.is_relative(), "{raw:?} was accepted as an absolute path");
                let joined = root.join(&rel);
                prop_assert!(
                    joined.starts_with(root),
                    "{raw:?} joined to {joined:?}, outside the bundle"
                );
                prop_assert!(
                    !rel.components().any(|c| matches!(
                        c,
                        std::path::Component::ParentDir | std::path::Component::CurDir
                    )),
                    "{raw:?} was accepted with a traversal component"
                );
            }
        }
    }

    /// A slug says the same thing whichever normalisation the title arrived in, and is never
    /// empty.
    ///
    /// The file name is derived from the title, so a slug that differed by normalisation would
    /// give one title two file names depending on which machine typed it. Never empty because
    /// a slug is half of a path segment; the other half, the uid prefix, is what makes it
    /// unique, so the slug is free to collide but not to vanish.
    #[test]
    fn a_slug_is_normalisation_insensitive_and_never_empty(title in title_strategy()) {
        use unicode_normalization::UnicodeNormalization;
        let nfc: String = title.nfc().collect();
        let nfd: String = title.nfd().collect();
        prop_assert_eq!(
            super::slug::slugify(&nfc),
            super::slug::slugify(&nfd),
            "{:?} slugs differently under NFC and NFD",
            title
        );
        let slug = super::slug::slugify(&title);
        prop_assert!(!slug.is_empty(), "{title:?} produced an empty slug");
        prop_assert!(
            !slug.contains('/') && !slug.contains('\\'),
            "{title:?} produced a slug with a separator in it: {slug:?}"
        );
    }

    /// Migrating is idempotent and always lands on the current version.
    ///
    /// The chain is one arm per transition and each arm rewrites data, so an arm that runs
    /// twice is an arm that has applied its rewrite twice. Nothing downstream would report it:
    /// the version stamp says current either way. Running the whole chain over an
    /// already-current bundle must therefore be a no-op, and running it over any stamped
    /// version must end at `FORMAT_VERSION` rather than short of it.
    #[test]
    fn migrating_is_idempotent_and_reaches_the_current_version(
        (bundle, _tag, _shape) in bundle_strategy()
    ) {
        let mut once = bundle.clone();
        super::migration::migrate_bundle(&mut once).expect("migrate");
        prop_assert_eq!(
            once.manifest.format_version,
            FORMAT_VERSION,
            "the chain stopped short of the current version"
        );

        let mut twice = once.clone();
        super::migration::migrate_bundle(&mut twice).expect("migrate again");
        prop_assert_eq!(twice, once, "migrating an already-current bundle changed it");
    }
}
