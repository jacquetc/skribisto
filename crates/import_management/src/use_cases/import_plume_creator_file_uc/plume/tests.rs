// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! End-to-end fixture tests: build a `.plume` (any version / container),
//! `import` it to a `.skrib`, read that back, and assert the full structure —
//! mirroring the old C++ `tst_plumecreatorimporter.cpp`.

use super::*;
use common::entities::BinderItemRole as Role;
use common::entities::BinderItemSubRole as SubRole;
use common::entities::ContentRole;
use skrib_format::{BundledBinder, BundledItem, read_bundle};
use std::io::Write;
use std::path::Path;

// -- fixture builders --------------------------------------------------

fn write_zip(path: &Path, members: &[(&str, &str)]) {
    let file = std::fs::File::create(path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let opts =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (name, content) in members {
        zip.start_file(*name, opts).unwrap();
        zip.write_all(content.as_bytes()).unwrap();
    }
    zip.finish().unwrap();
}

/// A modern (terminal-version) project exercising every structure.
fn terminal_members() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "info",
            r#"<!DOCTYPE plume-information><plume-information version="0.3"><prj name="Sample Novel" creationDate="2015-01-02T03:04:05" lastModified="2016-02-03T04:05:06"/></plume-information>"#,
        ),
        (
            "tree",
            r#"<!DOCTYPE plume-tree><plume-tree version="0.5" projectName="Sample">
                <book number="1" name="Book One">
                  <act number="2" name="Act I">
                    <chapter number="3" name="Chapter 1">
                      <scene number="4" name="Scene 1.1" attend="-10"/>
                      <separator number="10001" name="* * *"/>
                      <scene number="5" name="Scene 1.2"/>
                    </chapter>
                  </act>
                  <chapter number="6" name="Direct Chapter"/>
                  <chapter number="7" name="Trashed Chapter" isTrashed="yes">
                    <scene number="8" name="Trashed Scene"/>
                  </chapter>
                </book>
                <book number="9" name="Book Two">
                  <chapter number="20" name="B2 Ch"><scene number="21" name="B2 Scene"/></chapter>
                </book>
                <trash number="20000" name=""><book number="30" name="Deleted Book"/></trash>
              </plume-tree>"#,
        ),
        (
            "attendance",
            r#"<!DOCTYPE plume-attendance><plume-attendance version="0.6" box_1="Main--Secondary" box_2="None--Protagonist" spinBox_1_label="Age :">
                <group number="40" name="Characters">
                  <obj number="10" name="Alice" aliases="Al" quickDetails="The hero" box_1="0" box_2="1" spinBox_1="30"/>
                </group></plume-attendance>"#,
        ),
        ("dicts/userDict.dict_plume", "wibble;wobble;"),
        // Prose (T4 wrapped in a Qt-style doc to prove <style> stripping).
        (
            "text/T4.html",
            r#"<html><head><style type="text/css">p{margin:0}</style></head><body><p>Prose of scene 1.1.</p></body></html>"#,
        ),
        ("text/N4.html", "<p>Note for scene 1.1.</p>"),
        ("text/S2.html", "<p>The first act.</p>"),
        ("text/T6.html", "<p>Direct chapter prose.</p>"),
        ("text/N6.html", "<p>Direct chapter note.</p>"),
        ("attend/A10.html", "<p>Alice is the hero.</p>"),
    ]
}

// -- assertion helpers -------------------------------------------------

fn find<'a>(binder: &'a BundledBinder, title: &str) -> &'a BundledItem {
    binder
        .items
        .iter()
        .find(|i| i.item.title == title)
        .unwrap_or_else(|| {
            panic!(
                "no item titled '{title}' in binder '{}'",
                binder.binder.name
            )
        })
}
fn has(binder: &BundledBinder, title: &str) -> bool {
    binder.items.iter().any(|i| i.item.title == title)
}
fn prose(item: &BundledItem, role: ContentRole) -> String {
    let pr = item
        .item
        .prose_refs
        .iter()
        .find(|p| p.role == role)
        .unwrap_or_else(|| panic!("item '{}' has no prose {role:?}", item.item.title));
    item.prose.get(&pr.file_id).cloned().unwrap()
}
fn inline(item: &BundledItem, role: ContentRole) -> String {
    item.item
        .inline_contents
        .iter()
        .find(|c| c.role == role)
        .map(|c| c.text.clone())
        .unwrap_or_else(|| panic!("item '{}' has no inline {role:?}", item.item.title))
}

/// Import `members` (as a zip) and read the produced `.skrib` back.
fn import_zip(members: &[(&str, &str)]) -> (ImportSummary, skrib_format::WorkBundle) {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("project.plume");
    let out = dir.path().join("out.skrib");
    write_zip(&src, members);
    let summary = import(
        src.to_str().unwrap(),
        out.to_str().unwrap(),
        false,
        "Manuscript",
        "Story Bible",
    )
    .unwrap();
    let bundle = read_bundle(out.to_str().unwrap()).unwrap();
    (summary, bundle)
}

// -- tests -------------------------------------------------------------

#[test]
fn progress_is_reported_monotonically_to_completion() {
    use std::cell::RefCell;
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("p.plume");
    let out = dir.path().join("o.skrib");
    write_zip(&src, &terminal_members());

    let seen = RefCell::new(Vec::<f32>::new());
    let never = AtomicBool::new(false);
    let summary = import_with_progress(
        src.to_str().unwrap(),
        out.to_str().unwrap(),
        false,
        "M",
        "S",
        &|pct, _label| seen.borrow_mut().push(pct),
        &never,
    )
    .unwrap();

    let seen = seen.into_inner();
    assert!(!seen.is_empty(), "progress must be reported");
    assert!(
        seen.windows(2).all(|w| w[1] >= w[0]),
        "progress must be monotonic non-decreasing: {seen:?}"
    );
    assert!(
        seen.first().copied().unwrap() <= 20.0,
        "first report should be an early phase"
    );
    assert_eq!(
        seen.last().copied().unwrap(),
        100.0,
        "a successful import must finish at 100%"
    );
    assert_eq!(summary.imported_items, 15);
}

#[test]
fn cancel_during_mapping_leaves_no_output_and_preserves_target() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("p.plume");
    let out = dir.path().join("o.skrib");
    write_zip(&src, &terminal_members());
    // An existing target opened with overwrite=true must survive a cancel.
    std::fs::write(&out, b"ORIGINAL").unwrap();

    // Trip the cancel token as soon as the mapping phase begins reporting.
    let cancel = AtomicBool::new(false);
    let res = import_with_progress(
        src.to_str().unwrap(),
        out.to_str().unwrap(),
        true,
        "M",
        "S",
        &|pct, _| {
            if pct >= 20.0 {
                cancel.store(true, Ordering::Relaxed);
            }
        },
        &cancel,
    );

    assert!(res.is_err(), "a cancelled import must return Err");
    assert_eq!(
        std::fs::read(&out).unwrap(),
        b"ORIGINAL",
        "the overwrite target must be untouched after a cancel"
    );
    let tmp = format!("{}.importing", out.to_str().unwrap());
    assert!(
        !Path::new(&tmp).exists(),
        "no temp file may be left behind on cancel"
    );
}

#[test]
fn terminal_project_full_structure() {
    let (summary, bundle) = import_zip(&terminal_members());

    // Trashed: Trashed Chapter + Trashed Scene + Deleted Book = 3.
    assert_eq!(summary.skipped_trashed, 3);
    assert_eq!(summary.imported_items, 15);
    assert_eq!(bundle.binders.len(), 2);

    let manuscript = &bundle.binders[0];
    let story = &bundle.binders[1];
    assert_eq!(manuscript.binder.name, "Manuscript");
    assert_eq!(story.binder.name, "Story Bible");

    // Work metadata: title + preserved creation date.
    assert_eq!(bundle.manifest.work.title, "Sample Novel");
    assert!(bundle.manifest.work.created_at.starts_with("2015-01-02"));

    // --- manuscript tree: roles / sub-roles / indents ---
    let book1 = find(manuscript, "Book One");
    assert_eq!(
        (book1.item.role.clone(), book1.item.sub_role.clone()),
        (Role::Folder, SubRole::Book)
    );
    assert_eq!(book1.item.indent, 0);
    assert_eq!(inline(book1, ContentRole::BookTitle), "Book One");

    let act = find(manuscript, "Act I");
    assert_eq!(
        (act.item.role.clone(), act.item.sub_role.clone()),
        (Role::Folder, SubRole::Part)
    );
    assert_eq!(act.item.indent, 1);
    assert_eq!(inline(act, ContentRole::PartTitle), "Act I");
    assert!(prose(act, ContentRole::SynopsisText).contains("The first act."));

    let chap1 = find(manuscript, "Chapter 1");
    assert_eq!(
        (chap1.item.role.clone(), chap1.item.sub_role.clone()),
        (Role::Folder, SubRole::ChapterScene)
    );
    assert_eq!(chap1.item.indent, 2);

    let scene11 = find(manuscript, "Scene 1.1");
    assert_eq!(
        (scene11.item.role.clone(), scene11.item.sub_role.clone()),
        (Role::Item, SubRole::Scene)
    );
    assert_eq!(scene11.item.indent, 3);
    assert!(prose(scene11, ContentRole::SceneText).contains("Prose of scene 1.1."));
    // The Qt <style> block must not leak into the prose.
    assert!(!prose(scene11, ContentRole::SceneText).contains("margin"));

    // Scene note became a following-sibling Note.
    let scene11_notes = find(manuscript, "Scene 1.1 (notes)");
    assert_eq!(scene11_notes.item.sub_role, SubRole::Note);
    assert_eq!(scene11_notes.item.indent, 3);
    assert!(prose(scene11_notes, ContentRole::NoteText).contains("Note for scene 1.1."));

    // Separator → a scene-break marker appended to the PRECEDING scene's
    // prose, not an item of its own. Note that `Scene 1.1` pushed a sibling
    // Note after itself, so this also exercises the backward walk past it.
    assert!(
        !manuscript.items.iter().any(|i| i.item.title == "* * *"),
        "a separator must no longer occupy a binder slot"
    );
    assert!(
        prose(scene11, ContentRole::SceneText)
            .trim_end()
            .ends_with("\\* \\* \\*"),
        "the separator must land as an escaped marker at the end of Scene 1.1: {:?}",
        prose(scene11, ContentRole::SceneText)
    );

    // Leaf chapter (no scenes) → ChapterScene carrying its own prose.
    let direct = find(manuscript, "Direct Chapter");
    assert_eq!(direct.item.sub_role, SubRole::ChapterScene);
    assert_eq!(direct.item.indent, 1);
    assert_eq!(inline(direct, ContentRole::ChapterTitle), "Direct Chapter");
    assert!(prose(direct, ContentRole::SceneText).contains("Direct chapter prose."));
    assert!(has(manuscript, "Direct Chapter (notes)")); // its N → sibling note

    // Multi-book + BookEnd markers (one per book).
    assert!(has(manuscript, "Book Two"));
    assert!(has(manuscript, "B2 Scene"));
    let book_ends = manuscript
        .items
        .iter()
        .filter(|i| i.item.sub_role == SubRole::BookEnd)
        .count();
    assert_eq!(book_ends, 2);

    // Trashed nodes are absent.
    assert!(!has(manuscript, "Trashed Chapter"));
    assert!(!has(manuscript, "Trashed Scene"));
    assert!(!has(manuscript, "Deleted Book"));

    // --- story bible + cross-links ---
    let characters = find(story, "Characters");
    assert_eq!(
        (
            characters.item.role.clone(),
            characters.item.sub_role.clone()
        ),
        (Role::Folder, SubRole::None)
    );
    let alice = find(story, "Alice");
    assert_eq!(alice.item.sub_role, SubRole::Note);
    assert!(prose(alice, ContentRole::NoteText).contains("Alice is the hero."));
    let syn = prose(alice, ContentRole::SynopsisText);
    assert!(
        syn.contains("The hero")
            && syn.contains("Main")
            && syn.contains("Protagonist")
            && syn.contains("Age : 30"),
        "synopsis was: {syn:?}"
    );
    // `aliases="Al"` is structured data now, not the first cell of the synopsis
    // metadata line — that is what lets the mention index find "Al" in prose.
    assert_eq!(alice.item.aliases, vec!["Al".to_string()]);
    assert!(
        !syn.contains(" Al ") && !syn.starts_with("Al"),
        "the alias must not be duplicated back into the synopsis: {syn:?}"
    );
    // The story-bible group became a discoverable tag, and Alice carries it. Read off
    // the re-read bundle, so this also proves the tags survive the zip round-trip.
    let characters_tag = bundle
        .tags
        .iter()
        .find(|t| t.name == "Characters")
        .expect("a tag named after the story-bible group");
    assert!(characters_tag.discoverable);
    assert_eq!(alice.item.tag_ids, vec![characters_tag.file_id]);

    // Scene 1.1's attend="-10" resolved to Alice's item id.
    assert_eq!(scene11.item.reference_ids, vec![alice.item.file_id]);

    // Dictionary.
    let words: Vec<&str> = bundle.dict_words.iter().map(|w| w.word.as_str()).collect();
    assert!(words.contains(&"wibble") && words.contains(&"wobble"));
}

/// The importer builds its `ProjectManifest` as a hand-written literal, never through
/// `from_entities` — exactly the kind of second construction site that gets forgotten
/// when a new manifest field lands. It does not stamp the read floor itself, and must
/// not have to: `folder_io::write_folder` computes it at the manifest commit, which
/// every write path (this one included) funnels through. This is the test that says so.
///
/// The expected value is the floor for template-free content — Plume has no template
/// concept — so an imported project stays openable by the widest range of builds.
#[test]
fn an_imported_bundle_gets_a_correct_read_floor_without_the_importer_stamping_it() {
    let (_summary, bundle) = import_zip(&terminal_members());

    assert!(
        bundle.note_templates.is_empty(),
        "Plume has no templates to import"
    );
    assert_eq!(
        bundle.manifest.format_min_read_version,
        Some(skrib_format::version_gate::compute_min_read_version(
            &bundle
        )),
        "the writer must have stamped the content-derived floor on the way out"
    );
    assert!(
        bundle.manifest.format_min_read_version <= Some(skrib_format::FORMAT_VERSION),
        "an importer must never produce a file this very build could not reopen"
    );
}

#[test]
fn imported_bundle_carries_a_distinct_uid_for_every_row_on_disk() {
    // The importer WRITES a v3 bundle, so it must mint identities itself
    // rather than lean on the loader's heal-on-nil path. Without this the
    // on-disk artifact would be wrong while the app still looked fine,
    // because `materialize` would paper over it at load time.
    let (_summary, bundle) = import_zip(&terminal_members());

    assert_eq!(
        bundle.manifest.format_version,
        skrib_format::FORMAT_VERSION,
        "the importer must write the current format"
    );

    let mut seen = std::collections::HashSet::new();
    let mut rows = 0usize;
    for b in &bundle.binders {
        assert!(
            !b.binder.uid.is_nil(),
            "binder '{}' written without an identity",
            b.binder.name
        );
        assert!(seen.insert(b.binder.uid), "two rows share a uid");
        rows += 1;
        for i in &b.items {
            assert!(
                !i.item.uid.is_nil(),
                "item '{}' written without an identity",
                i.item.title
            );
            assert!(seen.insert(i.item.uid), "two rows share a uid");
            rows += 1;
        }
    }
    assert!(rows > 1, "fixture must produce several rows");
}

#[test]
fn old_zip_schema_is_normalized() {
    // tree 0.4 (root plume-tree, no <trash>, separator without number),
    // attendance 0.3 (legacy char/item/place + level/role, no <group>).
    let members = vec![
        (
            "info",
            r#"<!DOCTYPE plume-information><plume-information version="0.3"><prj name="Old Project"/></plume-information>"#,
        ),
        (
            "tree",
            r#"<!DOCTYPE plume-tree><plume-tree version="0.4" projectName="Old">
                <book number="1" name="OldBook"><chapter number="2" name="OldChap">
                  <scene number="3" name="OldScene"/></chapter></book></plume-tree>"#,
        ),
        (
            "attendance",
            r#"<!DOCTYPE plume-attendance><attendance version="0.3" levelsNames="Main--Secondary" rolesNames="None--Protagonist">
                <char number="5" firstName="Bob" lastName="Smith" level="1" role="1"/>
                <place number="6" name="Town"/></attendance>"#,
        ),
        ("text/T3.html", "<p>Old scene prose.</p>"),
    ];
    let (_summary, bundle) = import_zip(&members);
    let manuscript = &bundle.binders[0];
    assert!(has(manuscript, "OldBook") && has(manuscript, "OldChap"));
    let scene = find(manuscript, "OldScene");
    assert!(prose(scene, ContentRole::SceneText).contains("Old scene prose."));

    let story = &bundle.binders[1];
    // Legacy <char> folded into a synthesized "Characters" group, name from
    // firstName+lastName, level/role resolved via the legacy catalogs.
    assert!(has(story, "Characters") && has(story, "Places"));
    let bob = find(story, "Bob Smith");
    let syn = prose(bob, ContentRole::SynopsisText);
    assert!(
        syn.contains("Secondary") && syn.contains("Protagonist"),
        "synopsis was: {syn:?}"
    );
    assert!(has(story, "Town"));
}

#[test]
fn old_system_bare_directory_is_imported() {
    // Pre-0.3 layout: loose *.plume (root <plume>) + *.attend + *.prjinfo + text/.
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(
        root.join("Loose.plume"),
        r#"<!DOCTYPE plume><plume version="0.2" projectName="LooseProj">
            <book number="1" name="LooseBook"><chapter number="2" name="LooseChap">
              <scene number="3" name="LooseScene"/></chapter></book></plume>"#,
    )
    .unwrap();
    std::fs::write(root.join("Loose.attend"), r#"<!DOCTYPE plume-attendance><attendance version="0.2" levelsNames="Main" rolesNames="None">
            <char number="5" firstName="Carol" lastName="Jones"/></attendance>"#).unwrap();
    std::fs::write(root.join("Loose.prjinfo"), r#"<!DOCTYPE plume-information><plume-information version="0.2"><prj name="Loose Project"/></plume-information>"#).unwrap();
    std::fs::create_dir(root.join("text")).unwrap();
    std::fs::write(root.join("text/T3.html"), "<p>Loose scene prose.</p>").unwrap();

    let out = root.join("out.skrib");
    let summary = import(
        root.join("Loose.plume").to_str().unwrap(),
        out.to_str().unwrap(),
        false,
        "Manuscript",
        "Story Bible",
    )
    .unwrap();
    assert_eq!(summary.skipped_trashed, 0);

    let bundle = read_bundle(out.to_str().unwrap()).unwrap();
    assert_eq!(bundle.manifest.work.title, "Loose Project");
    let manuscript = &bundle.binders[0];
    assert!(has(manuscript, "LooseBook") && has(manuscript, "LooseScene"));
    let scene = find(manuscript, "LooseScene");
    assert!(prose(scene, ContentRole::SceneText).contains("Loose scene prose."));
    assert!(has(&bundle.binders[1], "Carol Jones"));
}

#[test]
fn rejects_non_plume_and_refuses_overwrite() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("bad.plume");
    // A zip with no `tree` member is not a Plume project.
    write_zip(&src, &[("random", "<x/>")]);
    let out = dir.path().join("out.skrib");
    assert!(
        import(
            src.to_str().unwrap(),
            out.to_str().unwrap(),
            false,
            "M",
            "S"
        )
        .is_err()
    );
    assert!(!out.exists(), "no output must be written on failure");

    // Overwrite guard.
    let good = dir.path().join("good.plume");
    write_zip(&good, &terminal_members());
    let out2 = dir.path().join("exists.skrib");
    std::fs::write(&out2, "sentinel").unwrap();
    assert!(
        import(
            good.to_str().unwrap(),
            out2.to_str().unwrap(),
            false,
            "M",
            "S"
        )
        .is_err()
    );
    // ...but succeeds with overwrite=true.
    assert!(
        import(
            good.to_str().unwrap(),
            out2.to_str().unwrap(),
            true,
            "M",
            "S"
        )
        .is_ok()
    );
}
