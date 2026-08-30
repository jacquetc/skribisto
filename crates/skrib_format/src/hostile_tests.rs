// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A bundle that did not come from the person opening it.
//!
//! Every other test in this crate reads a bundle this crate wrote. These read
//! bundles written to be hostile, because a `.skrib` is a file that travels:
//! it is mailed to a collaborator, restored from a shared drive, handed over on
//! a stick. Until [`crate::safe_path`] landed, four of the cases below reached a
//! real `open`/`create` outside the bundle.
//!
//! The two primitives, both proved here rather than described:
//!
//! * **Read** — `templates.ron`, `assets.ron` and each `ProseRef.path` are path
//!   strings joined onto the bundle root, and `Path::join` with an absolute
//!   argument discards the base.
//! * **Write** — a zip entry name this build does not model is carried verbatim
//!   and written back through the same join on the next save, so an entry called
//!   `../../../.bashrc` lands outside the project the first time autosave runs.
//!
//! The tests assert the refusal **and** that the out-of-bundle file was not
//! touched, because a check that merely errors after doing the damage would pass
//! the first half.

use super::bundle::ShapeTag;
use super::{SkribShape, read_bundle, write_bundle};

/// Write the ordinary fixture project as an exploded folder and hand back its
/// root, so a test can corrupt one manifest and read it again.
fn folder_project() -> (tempfile::TempDir, String, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tmp");
    let target = dir.path().join("Novel");
    let path = target.to_string_lossy().into_owned();
    let fixture = super::tests::build_bundle(ShapeTag::Folder);
    write_bundle(&path, SkribShape::ExplodedFolder, &fixture).expect("write");
    let root = super::shape::folder_root(&path);
    (dir, path, root)
}

/// Repoint the first `path:` in one of the bundle's `.ron` manifests at `evil`.
fn repoint_first_path(manifest: &std::path::Path, evil: &str) {
    let text = std::fs::read_to_string(manifest).expect("read manifest");
    let start = text.find("path: \"").expect("a path field") + "path: \"".len();
    let end = start + text[start..].find('"').expect("closing quote");
    let doctored = format!("{}{}{}", &text[..start], evil, &text[end..]);
    std::fs::write(manifest, doctored).expect("write manifest");
}

/// The whole error chain of a read failure, as one string.
///
/// `SkribFormatError`'s own `Display` prints only its outermost line, and every
/// interesting cause is inside the `anyhow::Error` that `Unreadable` wraps — so
/// an assertion on the plain `to_string()` would pass or fail for reasons
/// unrelated to the rule under test.
fn chain(err: &super::SkribFormatError) -> String {
    match err {
        super::SkribFormatError::Unreadable(e) => format!("{e:#}"),
        other => other.to_string(),
    }
}

/// A file outside any bundle, standing in for whatever the attacker is after.
fn secret_outside() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tmp");
    let secret = dir.path().join("id_rsa");
    std::fs::write(&secret, b"-----BEGIN PRIVATE KEY-----\n").expect("write secret");
    (dir, secret)
}

#[test]
fn an_absolute_asset_path_cannot_read_a_file_outside_the_bundle() {
    let (_outside, secret) = secret_outside();
    // The ordinary fixture supplies no asset *bytes*, and `from_entities` drops
    // an asset row whose bytes are missing rather than writing it dangling — so
    // its `assets.ron` is empty and there would be no path to repoint.
    let dir = tempfile::tempdir().expect("tmp");
    let path = dir.path().join("Novel").to_string_lossy().into_owned();
    let fixture = super::asset_tests::bundle_with_assets();
    write_bundle(&path, SkribShape::ExplodedFolder, &fixture).expect("write");
    let root = super::shape::folder_root(&path);

    repoint_first_path(&root.join("assets.ron"), &secret.to_string_lossy());

    let err = read_bundle(&path).expect_err("an absolute asset path must be refused");
    let msg = chain(&err);
    assert!(
        msg.contains("asset"),
        "error should name what it refused: {msg}"
    );
    assert!(
        msg.contains("absolute"),
        "error should name the rule: {msg}"
    );
}

#[test]
fn a_traversing_note_template_path_cannot_read_a_file_outside_the_bundle() {
    let (_outside, secret) = secret_outside();
    let (_dir, path, root) = folder_project();

    // Relative rather than absolute, so this exercises the `..` rung rather than
    // the `join`-discards-the-base one.
    let relative_escape = format!("../../{}", secret.file_name().unwrap().to_string_lossy());
    // Put the secret where that relative path actually resolves, so a test that
    // passes because the file happens not to exist cannot be mistaken for one
    // that passes because the path was refused.
    let reachable = root.parent().unwrap().parent().unwrap().join("id_rsa");
    std::fs::write(&reachable, b"-----BEGIN PRIVATE KEY-----\n").expect("plant");

    repoint_first_path(&root.join("templates.ron"), &relative_escape);

    let err = read_bundle(&path).expect_err("a traversing template path must be refused");
    let msg = chain(&err);
    assert!(msg.contains(".."), "error should name the rule: {msg}");
}

#[test]
fn a_traversing_prose_path_is_refused() {
    let (_dir, path, root) = folder_project();
    let binder_dir = {
        let mut found = None;
        for e in std::fs::read_dir(root.join("binders")).unwrap().flatten() {
            if e.path().join("items.ron").is_file() {
                found = Some(e.path());
            }
        }
        found.expect("a binder directory")
    };

    repoint_first_path(&binder_dir.join("items.ron"), "../../../../escaped.djot");

    let err = read_bundle(&path).expect_err("a traversing prose path must be refused");
    assert!(chain(&err).contains(".."), "{}", chain(&err));
}

/// The write primitive, and the one that fires on autosave rather than on open.
///
/// A carried file is an entry this build does not model, kept byte-for-byte so a
/// save cannot destroy a newer build's data. Its *name* is kept byte-for-byte
/// too, which is what made it a write primitive.
#[test]
fn a_carried_path_that_escapes_cannot_be_written_by_a_save() {
    let (outside, _secret) = secret_outside();
    let victim = outside.path().join("clobbered.txt");
    std::fs::write(&victim, b"original\n").expect("plant");

    let dir = tempfile::tempdir().expect("tmp");
    let target = dir.path().join("Novel");
    let path = target.to_string_lossy().into_owned();

    let mut fixture = super::tests::build_bundle(ShapeTag::Folder);
    fixture.carried.insert(
        victim.to_string_lossy().into_owned(),
        super::bundle::CarriedFile::new(b"clobbered by a bundle\n".to_vec()),
    );

    let err = write_bundle(&path, SkribShape::ExplodedFolder, &fixture)
        .expect_err("an escaping carried path must be refused");
    // `write_bundle` returns `anyhow::Result`, whose alternate Display already
    // walks the chain.
    assert!(format!("{err:#}").contains("carried"), "{err:#}");

    assert_eq!(
        std::fs::read(&victim).expect("victim still readable"),
        b"original\n",
        "the file outside the bundle was overwritten"
    );
}

// ── The zip door ────────────────────────────────────────────────────────────

/// One entry of a hand-built archive.
enum Entry<'a> {
    File(&'a str, &'a [u8]),
    /// A real symlink entry — `SimpleFileOptions::unix_permissions` keeps only
    /// the permission bits, so a hand-set `S_IFLNK` never reaches the archive
    /// and the entry arrives as an ordinary file.
    Symlink(&'a str, &'a str),
}

/// Build a zip with no writer of ours in the way, so an entry can be shaped in
/// ways `zip_dir` would never produce.
fn hostile_zip(path: &std::path::Path, entries: &[Entry<'_>]) {
    use std::io::Write;
    use zip::write::SimpleFileOptions;

    let file = std::fs::File::create(path).expect("create zip");
    let mut zw = zip::ZipWriter::new(file);
    let opts = SimpleFileOptions::default();
    for entry in entries {
        match entry {
            Entry::File(name, bytes) => {
                zw.start_file(*name, opts).expect("start entry");
                zw.write_all(bytes).expect("write entry");
            }
            Entry::Symlink(name, target) => {
                zw.add_symlink(*name, *target, opts).expect("symlink entry");
            }
        }
    }
    zw.finish().expect("finish zip");
}

#[test]
fn a_zip_entry_that_traverses_is_refused_before_it_is_written() {
    let dir = tempfile::tempdir().expect("tmp");
    let archive = dir.path().join("Novel.skrib");
    // The manifest name makes `detect_shape` treat it as a real bundle, so the
    // refusal comes from the extractor rather than from shape detection.
    hostile_zip(
        &archive,
        &[
            Entry::File("project.skrib", b"(format_version: 14)"),
            Entry::File("../../../escaped.txt", b"pwned\n"),
        ],
    );

    let err = read_bundle(&archive.to_string_lossy()).expect_err("traversal must be refused");
    assert!(chain(&err).contains(".."), "{}", chain(&err));

    let escaped = dir.path().parent().map(|p| p.join("escaped.txt"));
    if let Some(p) = escaped {
        assert!(!p.exists(), "the entry was written outside the extract dir");
    }
}

#[test]
fn a_symlink_entry_is_refused() {
    let dir = tempfile::tempdir().expect("tmp");
    let archive = dir.path().join("Novel.skrib");
    hostile_zip(
        &archive,
        &[
            Entry::File("project.skrib", b"(format_version: 14)"),
            Entry::Symlink("assets", "/etc"),
        ],
    );

    let err = read_bundle(&archive.to_string_lossy()).expect_err("a symlink must be refused");
    assert!(chain(&err).contains("symbolic link"), "{}", chain(&err));
}

#[test]
fn a_decompression_bomb_is_refused_rather_than_filling_the_disk() {
    let dir = tempfile::tempdir().expect("tmp");
    let archive = dir.path().join("Novel.skrib");

    // 512 MiB of zeroes deflates to a few hundred KiB — a ratio in the
    // thousands, far past the 200x limit, and well past the 64 MiB floor below
    // which the ratio is not consulted.
    let zeroes = vec![0u8; 512 << 20];
    hostile_zip(
        &archive,
        &[
            Entry::File("project.skrib", b"(format_version: 14)"),
            Entry::File("bomb.bin", &zeroes),
        ],
    );

    let err = read_bundle(&archive.to_string_lossy()).expect_err("a bomb must be refused");
    assert!(chain(&err).contains("expands"), "{}", chain(&err));
}

/// The one that is not a file-system escape but a process kill: the Djot parser
/// recurses per nested container with no limit, and a stack overflow aborts
/// rather than unwinding — so every unsaved document in the process is lost.
#[test]
fn prose_nested_deeply_enough_to_crash_the_parser_is_refused_at_the_bundle() {
    let (_dir, path, root) = folder_project();
    let binder_dir = std::fs::read_dir(root.join("binders"))
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .find(|p| p.join("items.ron").is_file())
        .expect("a binder directory");

    // Rewrite the first prose blob the binder points at.
    let items = std::fs::read_to_string(binder_dir.join("items.ron")).unwrap();
    let start = items.find("path: \"").expect("a prose path") + "path: \"".len();
    let end = start + items[start..].find('"').unwrap();
    let rel = &items[start..end];
    std::fs::write(root.join(rel), format!("{}deep\n", ">".repeat(4_000))).unwrap();

    let err = read_bundle(&path).expect_err("unbounded nesting must be refused");
    let msg = chain(&err);
    assert!(msg.contains("nests"), "{msg}");
    assert!(msg.contains(rel), "the error should name the file: {msg}");
}
