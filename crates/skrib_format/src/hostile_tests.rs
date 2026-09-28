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
///
/// `evil` goes in as the string it is, escaped the way RON reads a string back:
/// a Windows path is full of backslashes, and spliced in raw they are escapes the
/// parser refuses, so the manifest would fail to parse before the rule under test
/// was ever reached.
fn repoint_first_path(manifest: &std::path::Path, evil: &str) {
    let text = std::fs::read_to_string(manifest).expect("read manifest");
    let start = text.find("path: \"").expect("a path field") + "path: \"".len();
    let end = start + text[start..].find('"').expect("closing quote");
    let doctored = format!("{}{}{}", &text[..start], ron_escaped(evil), &text[end..]);
    std::fs::write(manifest, doctored).expect("write manifest");
}

/// `text` as it goes between the quotes of a RON string: every backslash and
/// every quote behind a backslash of its own.
fn ron_escaped(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}

/// What [`repoint_first_path`] splices in reads back as the string it was, so the
/// fixtures that name a file by its absolute path test the rule they are about on
/// Windows too, where that path is full of backslashes.
#[test]
fn a_spliced_path_reads_back_as_the_string_it_was() {
    for evil in [
        r"C:\Users\writer\AppData\Local\Temp\.tmpA1b2C3\id_rsa",
        r"\\server\share\id_rsa",
        r#"a "quoted" name"#,
        "../../../../escaped.djot",
    ] {
        let literal = format!("\"{}\"", ron_escaped(evil));
        assert_eq!(ron::from_str::<String>(&literal).as_deref(), Ok(evil));
    }
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

/// Whether a read failure is the typed zip refusal, through the `Unreadable` wrapper
/// `read_bundle` returns it in.
fn is_zip_refusal(err: &super::SkribFormatError) -> bool {
    match err {
        super::SkribFormatError::Unreadable(e) => super::zip_guard::refused(e).is_some(),
        _ => false,
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
    // thousands, far past any reader's limit, and well past the 64 MiB floor below
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
    assert!(
        is_zip_refusal(&err),
        "the bomb must be refused as a typed zip refusal: {}",
        chain(&err)
    );
    assert!(chain(&err).contains("unpacks"), "{}", chain(&err));
}

/// A project that unpacks to about a hundred times its size is refused, although a
/// Word or OpenDocument file may: the shared ratio of 200 let a crafted `.skrib` of about
/// 40 MB unpack to 8 GiB, every byte of which a load reads into memory. Real projects
/// unpack to a few times their size (see `zip_io::MAX_RATIO`).
#[test]
fn a_project_expanding_far_past_any_real_one_is_refused_below_the_shared_ratio() {
    let dir = tempfile::tempdir().expect("tmp");
    let archive = dir.path().join("Novel.skrib");

    // One byte that deflate cannot predict, then 255 zeroes, over 96 MiB: past the
    // 64 MiB floor, at a ratio between a project's and the shared one.
    let mut scene = vec![0u8; 96 << 20];
    let mut seed: u32 = 0x9E37_79B9;
    for chunk in scene.chunks_mut(256) {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        chunk[0] = (seed >> 24) as u8;
    }
    hostile_zip(
        &archive,
        &[
            Entry::File("project.skrib", b"(format_version: 14)"),
            Entry::File("binders/01-manuscript/scene.djot", &scene),
        ],
    );

    // The case tests nothing unless the shared ratio would have let it through.
    let file = std::fs::File::open(&archive).expect("open archive");
    let mut zip = zip::ZipArchive::new(file).expect("read archive");
    let member = zip
        .by_name("binders/01-manuscript/scene.djot")
        .expect("scene member");
    let ratio = member.size() / member.compressed_size().max(1);
    drop(member);
    assert!(
        ratio > super::zip_io::MAX_RATIO && ratio < super::zip_guard::MAX_RATIO,
        "the scene unpacks {ratio} times over, outside the range this case covers"
    );

    let err = read_bundle(&archive.to_string_lossy())
        .expect_err("a project expanding a hundred times over must be refused");
    assert!(
        is_zip_refusal(&err),
        "it must be refused as a typed zip refusal: {}",
        chain(&err)
    );
    assert!(chain(&err).contains("unpacks"), "{}", chain(&err));
}

/// A zip64 header declaring a member's size as an enormous number it does not hold
/// — the shape that aborts a reader reserving the declared size — is refused, before
/// a byte is inflated.
///
/// The over-declared member is the manifest itself, so the refusal comes from the
/// version gate that reads it (through the same guard), which is the very first thing
/// `read_bundle` does — proving the door is bounded even before extraction.
#[test]
fn a_zip64_over_declared_member_is_refused_before_inflating() {
    let dir = tempfile::tempdir().expect("tmp");
    let archive = dir.path().join("Novel.skrib");
    let bytes = super::zip_guard::fixtures::over_declared_zip(MANIFEST_NAME_FOR_TEST, 16 << 30);
    std::fs::write(&archive, &bytes).unwrap();

    let err = read_bundle(&archive.to_string_lossy())
        .expect_err("an over-declared manifest must be refused");
    assert!(
        is_zip_refusal(&err),
        "must be a typed zip refusal: {}",
        chain(&err)
    );
}

/// The manifest entry name, as [`crate::shape::MANIFEST_NAME`] spells it.
const MANIFEST_NAME_FOR_TEST: &str = "project.skrib";

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
    std::fs::write(root.join(rel), format!("{}deep\n", "> ".repeat(4_000))).unwrap();

    let err = read_bundle(&path).expect_err("unbounded nesting must be refused");
    let msg = chain(&err);
    assert!(msg.contains("nests"), "{msg}");
    assert!(msg.contains(rel), "the error should name the file: {msg}");
}

/// Where a test plants hostile prose in a bundle.
#[derive(Debug, Clone, Copy)]
enum PlantedIn {
    /// The first row's first prose blob.
    Prose,
    /// The first note template's body.
    TemplateBody,
    /// The first comment's body in a prose blob's `.comments.ron` sidecar.
    CommentBody,
    /// The first reply's body in that sidecar.
    ReplyBody,
    /// The first footnote's body in a prose blob's `.footnotes.ron` sidecar.
    FootnoteBody,
    /// The first comment's body in `orphan_comments.ron`.
    OrphanCommentBody,
    /// The body of a reply to that comment.
    OrphanReplyBody,
    /// The first footnote's body in `orphan_footnotes.ron`.
    OrphanFootnoteBody,
}

impl PlantedIn {
    /// Every place a bundle stores Djot that something later parses.
    const ALL: [PlantedIn; 8] = [
        PlantedIn::Prose,
        PlantedIn::TemplateBody,
        PlantedIn::CommentBody,
        PlantedIn::ReplyBody,
        PlantedIn::FootnoteBody,
        PlantedIn::OrphanCommentBody,
        PlantedIn::OrphanReplyBody,
        PlantedIn::OrphanFootnoteBody,
    ];
}

/// The fixture every planting test starts from: the ordinary project, plus a comment
/// thread and a footnote beside a row's prose and one of each in the orphanages.
fn fixture() -> super::WorkBundle {
    super::tests::build_bundle_with_footnotes(ShapeTag::Folder)
}

/// Where a planted body landed: the bundle-relative path of the file it is written
/// to, and the words naming it inside that file. A refusal has to say both.
struct Planted {
    file: String,
    what: String,
}

impl Planted {
    fn file(file: &str) -> Self {
        Self {
            file: file.to_string(),
            what: String::new(),
        }
    }
}

/// The sidecar beside the prose blob `content` names, as the bundle records it.
fn sidecar_of(item: &super::BundledItem, content: u64, name: fn(&str) -> String) -> String {
    let Some(prose_ref) = item.item.prose_refs.iter().find(|p| p.file_id == content) else {
        panic!("the fixture's sidecar must sit beside a prose blob");
    };
    name(&prose_ref.path)
}

/// Replace the prose at `place` in `bundle` with `text`, and say where it went.
fn plant(bundle: &mut super::WorkBundle, place: PlantedIn, text: &str) -> Planted {
    use super::folder_io::{comments_file_name, footnotes_file_name};
    match place {
        PlantedIn::Prose => {
            for binder in &mut bundle.binders {
                for item in &mut binder.items {
                    if let Some(prose_ref) = item.item.prose_refs.first() {
                        item.prose.insert(prose_ref.file_id, text.to_string());
                        return Planted::file(&prose_ref.path);
                    }
                }
            }
            panic!("the fixture must hold a prose blob");
        }
        PlantedIn::TemplateBody => {
            let Some(template) = bundle.note_templates.first() else {
                panic!("the fixture must hold a note template");
            };
            bundle
                .note_template_bodies
                .insert(template.file_id, text.to_string());
            Planted::file(&template.path)
        }
        PlantedIn::CommentBody | PlantedIn::ReplyBody => {
            let reply = matches!(place, PlantedIn::ReplyBody);
            for binder in &mut bundle.binders {
                for item in &mut binder.items {
                    let found = item.comments.iter_mut().find_map(|(content, thread)| {
                        let (index, comment) = thread
                            .iter_mut()
                            .enumerate()
                            .find(|(_, c)| !reply || !c.replies.is_empty())?;
                        Some((*content, index, comment))
                    });
                    let Some((content, index, comment)) = found else {
                        continue;
                    };
                    let what = if reply {
                        comment.replies[0].body = text.to_string();
                        format!("reply 1 to comment {}", index + 1)
                    } else {
                        comment.body = text.to_string();
                        format!("comment {}", index + 1)
                    };
                    return Planted {
                        file: sidecar_of(item, content, comments_file_name),
                        what,
                    };
                }
            }
            panic!("the fixture must hold a comment thread with a reply beside its prose");
        }
        PlantedIn::FootnoteBody => {
            for binder in &mut bundle.binders {
                for item in &mut binder.items {
                    let found = item.footnotes.iter_mut().find_map(|(content, notes)| {
                        notes.first_mut().map(|note| (*content, note))
                    });
                    let Some((content, note)) = found else {
                        continue;
                    };
                    note.body = text.to_string();
                    return Planted {
                        file: sidecar_of(item, content, footnotes_file_name),
                        what: "footnote 1".to_string(),
                    };
                }
            }
            panic!("the fixture must hold a footnote beside its prose");
        }
        PlantedIn::OrphanCommentBody => {
            let Some(comment) = bundle.orphan_comments.first_mut() else {
                panic!("the fixture must hold an orphaned comment");
            };
            comment.body = text.to_string();
            Planted {
                file: "orphan_comments.ron".to_string(),
                what: "comment 1".to_string(),
            }
        }
        PlantedIn::OrphanReplyBody => {
            let Some(comment) = bundle.orphan_comments.first_mut() else {
                panic!("the fixture must hold an orphaned comment");
            };
            comment.replies.push(super::CommentReplyFile {
                file_id: 9_901,
                uid: common::uid::fixture_uid(9_901),
                created_at: comment.created_at.clone(),
                updated_at: comment.updated_at.clone(),
                author_name: "Marc".to_string(),
                author_initials: "M".to_string(),
                body: text.to_string(),
            });
            Planted {
                file: "orphan_comments.ron".to_string(),
                what: format!("reply {} to comment 1", comment.replies.len()),
            }
        }
        PlantedIn::OrphanFootnoteBody => {
            let Some(note) = bundle.orphan_footnotes.first_mut() else {
                panic!("the fixture must hold an orphaned footnote");
            };
            note.body = text.to_string();
            Planted {
                file: "orphan_footnotes.ron".to_string(),
                what: "footnote 1".to_string(),
            }
        }
    }
}

/// Every piece of Djot `bundle` holds: rows, template bodies, and the bodies of
/// every comment, reply and footnote, anchored or orphaned.
fn every_stored_djot(bundle: &super::WorkBundle) -> Vec<&str> {
    fn threads(list: &[super::CommentFile]) -> impl Iterator<Item = &str> {
        list.iter().flat_map(|c| {
            std::iter::once(c.body.as_str()).chain(c.replies.iter().map(|r| r.body.as_str()))
        })
    }
    let mut out: Vec<&str> = Vec::new();
    for item in bundle.binders.iter().flat_map(|binder| &binder.items) {
        out.extend(item.prose.values().map(String::as_str));
        for list in item.comments.values() {
            out.extend(threads(list));
        }
        for notes in item.footnotes.values() {
            out.extend(notes.iter().map(|n| n.body.as_str()));
        }
    }
    out.extend(bundle.note_template_bodies.values().map(String::as_str));
    out.extend(threads(&bundle.orphan_comments));
    out.extend(bundle.orphan_footnotes.iter().map(|n| n.body.as_str()));
    out
}

/// Parse every piece of Djot `bundle` holds on a long operation's stack: what
/// opening, exporting or searching the project does, and what the comment and
/// footnote cards do with a body.
fn parse_every_prose(bundle: &super::WorkBundle) {
    for text in every_stored_djot(bundle) {
        let parsed = super::djot_depth::tests::parse_on_a_long_operation_stack(text.to_string());
        if let Err(e) = parsed {
            panic!("the parser refused the prose: {e}");
        }
    }
}

/// Every shape of nesting a container can take, in every place a bundle stores Djot
/// (a row's prose, a note template's body, and the body of a comment, a reply or a
/// footnote, beside the prose or in an orphanage), in a folder and in a zip: refused
/// at the bundle, naming the file and the line. Before the guard counted every kind
/// of container, every shape but the first loaded from the prose and the templates,
/// and before it read the sidecars every shape loaded from a comment, a reply or a
/// footnote; the first parse aborted the process. This test then aborts in
/// `parse_every_prose`.
#[test]
fn every_shape_of_nesting_past_the_ceiling_is_refused_at_the_bundle_by_name() {
    for (name, hostile) in super::djot_depth::tests::past_the_parsers_limit() {
        for shape in [SkribShape::ExplodedFolder, SkribShape::ZipFile] {
            for place in PlantedIn::ALL {
                let mut bundle = fixture();
                let planted = plant(&mut bundle, place, &hostile);
                let dir = tempfile::tempdir().expect("tmp");
                let path = dir
                    .path()
                    .join("Novel.skrib")
                    .to_string_lossy()
                    .into_owned();
                write_bundle(&path, shape, &bundle).expect("write");

                match read_bundle(&path) {
                    Err(err) => {
                        let msg = chain(&err);
                        assert!(msg.contains("nests"), "{name}, {shape:?}, {place:?}: {msg}");
                        assert!(
                            msg.contains(&planted.file),
                            "{name}, {place:?}: names the file: {msg}"
                        );
                        assert!(
                            msg.contains(&planted.what),
                            "{name}, {place:?}: names the body in it: {msg}"
                        );
                        assert!(msg.contains("at line "), "{name}: names the line: {msg}");
                    }
                    Ok(loaded) => {
                        parse_every_prose(&loaded);
                        panic!("{name}, {shape:?}, {place:?}: loaded past the ceiling");
                    }
                }
            }
        }
    }
}

/// Djot the parser cannot be given though it nests nothing (a heading deeper than it
/// counts, which panics it on the thread that opens the row, paragraphs of openers
/// nothing closes, which it takes minutes to read, and paragraphs that keep something
/// open over thousands of lines, which overflow its stack), in every place a bundle
/// stores Djot and in both shapes: refused at the bundle, naming the file, the body and
/// the line.
#[test]
fn djot_the_parser_cannot_be_given_is_refused_at_the_bundle_by_name() {
    for (name, hostile, says) in super::djot_depth::tests::beyond_the_parser() {
        for shape in [SkribShape::ExplodedFolder, SkribShape::ZipFile] {
            for place in PlantedIn::ALL {
                let mut bundle = fixture();
                let planted = plant(&mut bundle, place, &hostile);
                let dir = tempfile::tempdir().expect("tmp");
                let path = dir
                    .path()
                    .join("Novel.skrib")
                    .to_string_lossy()
                    .into_owned();
                write_bundle(&path, shape, &bundle).expect("write");

                match read_bundle(&path) {
                    Err(err) => {
                        let msg = chain(&err);
                        assert!(msg.contains(says), "{name}, {shape:?}, {place:?}: {msg}");
                        assert!(msg.contains(&planted.file), "{name}, {place:?}: {msg}");
                        assert!(msg.contains(&planted.what), "{name}, {place:?}: {msg}");
                        assert!(msg.contains("at line "), "{name}: names the line: {msg}");
                    }
                    Ok(loaded) => {
                        parse_every_prose(&loaded);
                        panic!("{name}, {shape:?}, {place:?}: loaded");
                    }
                }
            }
        }
    }
}

/// Paragraphs holding more lines than the ceiling, which the load joins into one line
/// rather than refusing, in every place a bundle stores Djot, in a folder and in a zip:
/// the project opens, each body joined, and every piece of Djot it holds parses from a
/// long operation's stack. Before, each was refused by name, and the project with it.
#[test]
fn a_paragraph_the_load_joins_opens_from_every_place() {
    for (name, held, _) in super::djot_depth::tests::joined_by_the_load() {
        let Ok(joined) = super::djot_depth::admit(held.clone()) else {
            panic!("{name}: the load joins it");
        };
        for shape in [SkribShape::ExplodedFolder, SkribShape::ZipFile] {
            let mut bundle = fixture();
            for place in PlantedIn::ALL {
                plant(&mut bundle, place, &held);
            }
            let dir = tempfile::tempdir().expect("tmp");
            let path = dir
                .path()
                .join("Novel.skrib")
                .to_string_lossy()
                .into_owned();
            write_bundle(&path, shape, &bundle).expect("write");
            let loaded = match read_bundle(&path) {
                Ok(loaded) => loaded,
                Err(err) => panic!("{name}, {shape:?}: {}", chain(&err)),
            };
            let stored = every_stored_djot(&loaded);
            assert_eq!(
                stored.iter().filter(|djot| **djot == joined).count(),
                PlantedIn::ALL.len(),
                "{name}, {shape:?}: every body is kept, joined"
            );
            assert!(stored.iter().all(|djot| *djot != held), "{name}, {shape:?}");
            parse_every_prose(&loaded);
        }
    }
}

/// What the editor writes for a preformatted passage of six hundred lines, pasted and
/// formatted from end to end (in italics after the paste, pasted in italics, made a
/// link...), saved as a row's prose and as a comment's body: the project opens again,
/// in either shape, and the editor reads the prose it gets back as the prose it wrote.
/// Before, the next load refused the project from 129 lines on.
#[test]
fn a_formatted_pasted_passage_the_editor_saved_opens_again() {
    for (name, djot) in super::djot_depth::tests::formatted_passages(600) {
        for shape in [SkribShape::ExplodedFolder, SkribShape::ZipFile] {
            let mut bundle = fixture();
            let planted = [PlantedIn::Prose, PlantedIn::CommentBody];
            for place in planted {
                plant(&mut bundle, place, &djot);
            }
            let dir = tempfile::tempdir().expect("tmp");
            let path = dir
                .path()
                .join("Novel.skrib")
                .to_string_lossy()
                .into_owned();
            write_bundle(&path, shape, &bundle).expect("write");
            let loaded = match read_bundle(&path) {
                Ok(loaded) => loaded,
                Err(err) => panic!("{name}, {shape:?}: {}", chain(&err)),
            };
            let Ok(joined) = super::djot_depth::admit(djot.clone()) else {
                panic!("{name}: the load joins it");
            };
            let stored = every_stored_djot(&loaded);
            assert_eq!(
                stored.iter().filter(|stored| **stored == joined).count(),
                planted.len(),
                "{name}, {shape:?}"
            );
            assert_eq!(
                super::djot_depth::tests::reading(joined),
                super::djot_depth::tests::reading(djot.clone()),
                "{name}: the editor reads it as it wrote it"
            );
            parse_every_prose(&loaded);
        }
    }
}

/// The same shapes at the ceiling load from every place, and every piece of Djot the
/// bundle holds parses from a long operation's stack.
#[test]
fn every_shape_of_nesting_at_the_ceiling_loads_and_parses() {
    for marker in ["- ", "1. ", "(iv) ", "- [ ] ", "[^a]: ", ": ", "> "] {
        let text = super::djot_depth::tests::one_line(marker, super::MAX_DJOT_DEPTH);
        for shape in [SkribShape::ExplodedFolder, SkribShape::ZipFile] {
            let mut bundle = fixture();
            for place in PlantedIn::ALL {
                plant(&mut bundle, place, &text);
            }
            let dir = tempfile::tempdir().expect("tmp");
            let path = dir
                .path()
                .join("Novel.skrib")
                .to_string_lossy()
                .into_owned();
            write_bundle(&path, shape, &bundle).expect("write");
            let loaded = match read_bundle(&path) {
                Ok(loaded) => loaded,
                Err(err) => panic!("{marker:?}, {shape:?}: {}", chain(&err)),
            };
            let stored = every_stored_djot(&loaded);
            assert_eq!(
                stored.iter().filter(|djot| **djot == text).count(),
                PlantedIn::ALL.len(),
                "{marker:?}, {shape:?}: every body planted at the ceiling loads as it was"
            );
            parse_every_prose(&loaded);
        }
    }
}

/// Before v12 a comment's body and a reply's were stored as plain text, which nothing
/// parses as it stands: the load rewrites each one as the Djot that reads back as the
/// same words. So a remark in an older project that only looks nested opens, and comes
/// back as those words, within the ceiling and parseable from a long operation's stack.
/// A footnote's body was Djot from the day footnotes existed, and in the same project it
/// is refused by name, like the prose.
#[test]
fn a_pre_v12_remark_that_looks_nested_opens_as_its_words() {
    const REMARKS: [PlantedIn; 4] = [
        PlantedIn::CommentBody,
        PlantedIn::ReplyBody,
        PlantedIn::OrphanCommentBody,
        PlantedIn::OrphanReplyBody,
    ];
    let write_v11 = |bundle: &mut super::WorkBundle, shape: SkribShape| {
        bundle.manifest.format_version = 11;
        let dir = tempfile::tempdir().expect("tmp");
        let path = dir
            .path()
            .join("Novel.skrib")
            .to_string_lossy()
            .into_owned();
        write_bundle(&path, shape, bundle).expect("write");
        (dir, path)
    };

    for (name, hostile) in super::djot_depth::tests::past_the_parsers_limit() {
        let words = super::plain_text_to_djot_verbatim(&hostile);
        assert!(super::djot_depth::check(&words).is_ok(), "{name}");
        for shape in [SkribShape::ExplodedFolder, SkribShape::ZipFile] {
            // Every remark in one project: none of them may be refused.
            let mut bundle = fixture();
            for place in REMARKS {
                plant(&mut bundle, place, &hostile);
            }
            let (_dir, path) = write_v11(&mut bundle, shape);
            let loaded = match read_bundle(&path) {
                Ok(loaded) => loaded,
                Err(err) => panic!(
                    "{name}, {shape:?}: a plain-text remark was refused: {}",
                    chain(&err)
                ),
            };
            let stored = every_stored_djot(&loaded)
                .into_iter()
                .filter(|djot| *djot == words)
                .count();
            assert_eq!(
                stored,
                REMARKS.len(),
                "{name}, {shape:?}: every remark is stored as its words"
            );
            parse_every_prose(&loaded);

            for place in [PlantedIn::FootnoteBody, PlantedIn::OrphanFootnoteBody] {
                let mut bundle = fixture();
                let planted = plant(&mut bundle, place, &hostile);
                let (_dir, path) = write_v11(&mut bundle, shape);
                match read_bundle(&path) {
                    Err(err) => {
                        let msg = chain(&err);
                        assert!(msg.contains("nests"), "{name}, {shape:?}, {place:?}: {msg}");
                        assert!(msg.contains(&planted.file), "{name}, {place:?}: {msg}");
                    }
                    Ok(loaded) => {
                        parse_every_prose(&loaded);
                        panic!("{name}, {shape:?}, {place:?}: loaded past the ceiling");
                    }
                }
            }
        }
    }
}

/// Not a hostile bundle: the writer's own. The editor stores a paragraph's leading
/// spaces and tabs as they were typed, so a paragraph typed after four hundred of them
/// is saved exactly so, and the load used to count half a level of nesting for every
/// one of them and refuse the whole project, which then would not open at all. Typed
/// into every place a bundle stores Djot, saved in both shapes, it opens, every body
/// comes back as it was saved, and every one parses from a long operation's stack.
#[test]
fn a_paragraph_typed_after_four_hundred_spaces_or_tabs_opens() {
    for blank in [" ", "\t", " \t"] {
        let typed = format!(
            "{}Set far in.\n\nThen back at the margin.\n\n{}\n",
            blank.repeat(400 / blank.len()),
            blank.repeat(400 / blank.len())
        );
        let doc = text_document::TextDocument::new();
        doc.set_plain_text(&typed).expect("type");
        let saved = doc
            .to_djot()
            .expect("the editor writes its document as Djot");
        assert!(
            saved.starts_with(&blank.repeat(400 / blank.len())),
            "{blank:?}: the editor stores the paragraph's leading blanks as typed: {saved:.40?}"
        );
        for shape in [SkribShape::ExplodedFolder, SkribShape::ZipFile] {
            let mut bundle = fixture();
            for place in PlantedIn::ALL {
                plant(&mut bundle, place, &saved);
            }
            let dir = tempfile::tempdir().expect("tmp");
            let path = dir
                .path()
                .join("Novel.skrib")
                .to_string_lossy()
                .into_owned();
            write_bundle(&path, shape, &bundle).expect("write");
            let loaded = match read_bundle(&path) {
                Ok(loaded) => loaded,
                Err(err) => panic!(
                    "{blank:?}, {shape:?}: the project does not open: {}",
                    chain(&err)
                ),
            };
            assert_eq!(
                every_stored_djot(&loaded)
                    .iter()
                    .filter(|djot| **djot == saved)
                    .count(),
                PlantedIn::ALL.len(),
                "{blank:?}, {shape:?}: every body comes back as it was saved"
            );
            parse_every_prose(&loaded);
        }
    }
}
