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

/// Plume's own eight-rung ladder, as the app hands it down (already locale-resolved).
fn ladder() -> Vec<String> {
    [
        "1st draft",
        "2nd draft",
        "3rd draft",
        "1st Edit",
        "2nd Edit",
        "3rd Edit",
        "Proofread",
        "Finished",
    ]
    .map(str::to_string)
    .to_vec()
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
        &ladder(),
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
        &ladder(),
        &crate::SeededNames::default(),
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
    // Two notes are scratchpads of their own items, not items of their own.
    assert_eq!(summary.imported_items, 13);
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
        &ladder(),
        &crate::SeededNames::default(),
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
    // Scene 1.1's and Direct Chapter's notes are their scratchpads, no items of their own.
    assert_eq!(summary.imported_items, 13);
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

    // The scene's note is its own scratchpad, not a Note beside it.
    assert!(prose(scene11, ContentRole::ScratchpadText).contains("Note for scene 1.1."));
    assert!(!has(manuscript, "Scene 1.1 (notes)"));
    // A scene with no note has no scratchpad, not a blank one.
    let scene12 = find(manuscript, "Scene 1.2");
    assert!(
        scene12
            .item
            .prose_refs
            .iter()
            .all(|p| p.role != ContentRole::ScratchpadText)
    );

    // Separator → a scene-break marker appended to the PRECEDING scene's
    // prose, not an item of its own.
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
    // Its N is its scratchpad.
    assert!(prose(direct, ContentRole::ScratchpadText).contains("Direct chapter note."));
    assert!(!has(manuscript, "Direct Chapter (notes)"));

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

/// The titles the import makes up are the caller's words, in the writer's language: a
/// French interface gets a French title for a container's own text, a French name for a
/// project that names none, and nothing English.
#[test]
fn the_titles_the_import_makes_up_are_the_callers_words() {
    let slot = skrib_format::importing::TITLE_SLOT;
    // A project naming itself nowhere, and an act (a part, which holds no text) with text
    // of its own.
    let info = r#"<!DOCTYPE plume-information><plume-information version="0.3"><prj name="" creationDate="2015-01-02T03:04:05" lastModified="2016-02-03T04:05:06"/></plume-information>"#;
    let terminal = terminal_members();
    let tree = terminal[1]
        .1
        .replacen(r#"projectName="Sample""#, r#"projectName="""#, 1);
    let mut members: Vec<(&str, &str)> = vec![("info", info), ("tree", &tree)];
    members.extend(terminal.iter().skip(2).copied());
    members.push(("text/T2.html", "<p>Act prose.</p>"));
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("project.plume");
    let out = dir.path().join("out.skrib");
    write_zip(&src, &members);
    let seeded = SeededNames {
        untitled_project: "Projet importé".into(),
        content_of: format!("Texte de « {slot} »"),
        ..SeededNames::default()
    };
    import_with_progress(
        src.to_str().unwrap(),
        out.to_str().unwrap(),
        false,
        "Manuscrit",
        "Bible",
        &[],
        &seeded,
        &|_, _| {},
        &AtomicBool::new(false),
    )
    .unwrap();
    let bundle = read_bundle(out.to_str().unwrap()).unwrap();
    assert_eq!(bundle.manifest.work.title, "Projet importé");
    let manuscript = &bundle.binders[0];
    assert_eq!(manuscript.binder.name, "Manuscrit");
    assert!(has(manuscript, "Texte de « Act I »"));
    let english = manuscript
        .items
        .iter()
        .map(|i| i.item.title.as_str())
        .find(|t| t.contains("(notes)") || t.contains("(text)") || t.contains("Imported"));
    assert_eq!(english, None);
}

/// Every node's own note arrives in its scratchpad, whatever the node became: a book, an
/// act (a part), a chapter folder, a leaf chapter and a scene. No note is made beside
/// any of them, the scratchpad is written as its own `.scratchpad.djot` file, and a
/// separator's note, which has nowhere to go, is still said to be dropped.
#[test]
fn every_node_keeps_its_own_note_in_its_scratchpad() {
    let mut members = terminal_members();
    members.extend([
        ("text/N1.html", "<p>Book one plan.</p>"),
        ("text/N2.html", "<p>Act one beats.</p>"),
        ("text/N3.html", "<p>Chapter one aims.</p>"),
        ("text/N10001.html", "<p>Separator aside.</p>"),
    ]);
    let (summary, bundle) = import_zip(&members);
    let manuscript = &bundle.binders[0];
    for (title, words, sub_role) in [
        ("Book One", "Book one plan.", SubRole::Book),
        ("Act I", "Act one beats.", SubRole::Part),
        ("Chapter 1", "Chapter one aims.", SubRole::ChapterScene),
        (
            "Direct Chapter",
            "Direct chapter note.",
            SubRole::ChapterScene,
        ),
        ("Scene 1.1", "Note for scene 1.1.", SubRole::Scene),
    ] {
        let item = find(manuscript, title);
        assert_eq!(item.item.sub_role, sub_role, "{title}");
        assert!(
            prose(item, ContentRole::ScratchpadText).contains(words),
            "{title}: {:?}",
            prose(item, ContentRole::ScratchpadText)
        );
        let pad = item
            .item
            .prose_refs
            .iter()
            .find(|p| p.role == ContentRole::ScratchpadText)
            .unwrap();
        assert!(pad.path.ends_with(".scratchpad.djot"), "{}", pad.path);
    }
    assert!(
        manuscript
            .items
            .iter()
            .all(|i| i.item.sub_role != SubRole::Note),
        "no note beside anything: {:?}",
        manuscript
            .items
            .iter()
            .map(|i| &i.item.title)
            .collect::<Vec<_>>()
    );
    assert!(
        summary
            .warnings
            .contains(&ImportNotice::SeparatorTextDropped {
                name: "* * *".into()
            }),
        "{:?}",
        summary.warnings
    );
    assert_eq!(
        bundle.manifest.format_min_read_version,
        Some(18),
        "a scratchpad claims the floor that reads it"
    );
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

/// A story-bible entry's quick details are what the writer typed into a plain field.
/// Stored as Djot unescaped, `*not*` came back emphasised, `10:30:45` lost `:30:` as a
/// symbol, the quotes were curled and a detail opening `A. ` became a list item.
#[test]
fn a_story_bible_entrys_plain_details_read_back_as_typed() {
    let details = "Born at 10:30:45 -- \"Lucky\" is *not* her name";
    let second_line = "A. Smith knew her";
    let attendance = format!(
        r#"<!DOCTYPE plume-attendance><plume-attendance version="0.6" box_1="Main--Secondary" box_2="None--Protagonist" spinBox_1_label="Age :">
            <group number="40" name="Characters">
              <obj number="10" name="Alice" quickDetails="{}&#10;{}" box_1="0" box_2="1" spinBox_1="30"/>
            </group></plume-attendance>"#,
        details.replace('"', "&quot;"),
        second_line
    );
    let mut members = terminal_members();
    for member in &mut members {
        if member.0 == "attendance" {
            member.1 = attendance.as_str();
        }
    }
    let (_summary, bundle) = import_zip(&members);

    let alice = find(&bundle.binders[1], "Alice");
    let synopsis = prose(alice, ContentRole::SynopsisText);
    let (text, paragraphs) = skrib_format::djot_plain_text(&synopsis).expect("parse");
    assert_eq!(
        text,
        format!("{details}\n{second_line}\nMain · Protagonist · Age : 30"),
        "stored as {synopsis:?}"
    );
    assert_eq!(paragraphs.len(), 3, "one paragraph per line: {synopsis:?}");
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
        &ladder(),
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

/// A pre-0.3 project folder holding one scene, `LooseScene` (node 3), with no
/// `text/` folder yet. Returns the loose `.plume` tree file.
fn old_system_folder(root: &Path) -> std::path::PathBuf {
    let tree = root.join("Loose.plume");
    std::fs::write(
        &tree,
        r#"<!DOCTYPE plume><plume version="0.2" projectName="LooseProj">
            <book number="1" name="LooseBook"><chapter number="2" name="LooseChap">
              <scene number="3" name="LooseScene"/></chapter></book></plume>"#,
    )
    .unwrap();
    tree
}

/// Import the old-system project at `tree` into `root/out.skrib`.
fn import_old_system(root: &Path, tree: &Path) -> (ImportSummary, skrib_format::WorkBundle) {
    let out = root.join("out.skrib");
    let summary = import(
        tree.to_str().unwrap(),
        out.to_str().unwrap(),
        false,
        "Manuscript",
        "Story Bible",
        &ladder(),
    )
    .unwrap();
    let bundle = read_bundle(out.to_str().unwrap()).unwrap();
    (summary, bundle)
}

/// Every prose text an item holds, joined: empty when it holds none.
fn all_prose(item: &BundledItem) -> String {
    item.prose.values().cloned().collect::<Vec<_>>().join("\n")
}

fn names_unread(summary: &ImportSummary, member: &str) -> bool {
    summary
        .warnings
        .iter()
        .any(|w| matches!(w, ImportNotice::FileUnreadable { member: m, .. } if m == member))
}

/// `text/T3.html -> ~/.ssh/id_rsa` in a folder someone sent: the key must not
/// become the scene's prose. The link is named in the summary instead.
#[test]
#[cfg(unix)]
fn old_system_link_to_a_file_outside_the_project_is_not_read() {
    let dir = tempfile::tempdir().unwrap();
    let secret = dir.path().join("id_rsa");
    std::fs::write(&secret, "PRIVATE KEY").unwrap();
    let root = dir.path().join("project");
    std::fs::create_dir(&root).unwrap();
    let tree = old_system_folder(&root);
    std::fs::create_dir(root.join("text")).unwrap();
    std::os::unix::fs::symlink(&secret, root.join("text/T3.html")).unwrap();
    // A `.prjinfo` that is a link is not read either.
    std::os::unix::fs::symlink(&secret, root.join("Loose.prjinfo")).unwrap();

    let (summary, bundle) = import_old_system(&root, &tree);
    let scene = find(&bundle.binders[0], "LooseScene");
    assert!(!all_prose(scene).contains("PRIVATE KEY"));
    assert!(
        names_unread(&summary, "text/T3.html"),
        "{:?}",
        summary.warnings
    );
    assert!(
        names_unread(&summary, "Loose.prjinfo"),
        "{:?}",
        summary.warnings
    );
}

/// A `text` folder that is a link to a folder elsewhere is not walked.
#[test]
#[cfg(unix)]
fn old_system_linked_text_folder_is_not_walked() {
    let dir = tempfile::tempdir().unwrap();
    let elsewhere = dir.path().join("elsewhere");
    std::fs::create_dir(&elsewhere).unwrap();
    std::fs::write(elsewhere.join("T3.html"), "<p>Not yours.</p>").unwrap();
    let root = dir.path().join("project");
    std::fs::create_dir(&root).unwrap();
    let tree = old_system_folder(&root);
    std::os::unix::fs::symlink(&elsewhere, root.join("text")).unwrap();

    let (summary, bundle) = import_old_system(&root, &tree);
    let scene = find(&bundle.binders[0], "LooseScene");
    assert!(!all_prose(scene).contains("Not yours."));
    assert!(names_unread(&summary, "text"), "{:?}", summary.warnings);
}

/// A FIFO with no writer would block the import forever; a device would be read
/// without end. Both are left unread and named, and the rest still imports.
#[test]
#[cfg(unix)]
fn old_system_fifo_and_device_are_skipped() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("project");
    std::fs::create_dir(&root).unwrap();
    let tree = old_system_folder(&root);
    std::fs::create_dir(root.join("text")).unwrap();
    std::fs::write(root.join("text/T3.html"), "<p>Loose scene prose.</p>").unwrap();
    std::os::unix::fs::symlink("/dev/zero", root.join("text/N3.html")).unwrap();
    let fifo = root.join("text/S3.html");
    let made = std::process::Command::new("mkfifo").arg(&fifo).status();
    let have_fifo = made.is_ok_and(|s| s.success());

    let (summary, bundle) = import_old_system(&root, &tree);
    let scene = find(&bundle.binders[0], "LooseScene");
    assert!(prose(scene, ContentRole::SceneText).contains("Loose scene prose."));
    assert!(
        names_unread(&summary, "text/N3.html"),
        "{:?}",
        summary.warnings
    );
    if have_fifo {
        assert!(
            names_unread(&summary, "text/S3.html"),
            "{:?}",
            summary.warnings
        );
    }
}

/// Opening a socket fails with the system's own error, not as "not a regular
/// file", and that error used to refuse the whole project. A socket in `text/` is
/// now left unopened and named, and the rest imports.
#[test]
#[cfg(unix)]
fn old_system_socket_is_skipped_and_the_project_still_imports() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("project");
    std::fs::create_dir(&root).unwrap();
    let tree = old_system_folder(&root);
    std::fs::create_dir(root.join("text")).unwrap();
    std::fs::write(root.join("text/T3.html"), "<p>Loose scene prose.</p>").unwrap();
    let _socket = std::os::unix::net::UnixListener::bind(root.join("text/N3.html")).unwrap();

    let (summary, bundle) = import_old_system(&root, &tree);
    let scene = find(&bundle.binders[0], "LooseScene");
    assert!(prose(scene, ContentRole::SceneText).contains("Loose scene prose."));
    assert!(
        names_unread(&summary, "text/N3.html"),
        "{:?}",
        summary.warnings
    );
}

/// With `A.attend` a link and `B.attend` a real file, the first by name is the
/// link: taking it lost the characters `B.attend` holds. A regular file wins.
#[test]
#[cfg(unix)]
fn old_system_attendance_prefers_a_real_file_over_a_link_sorted_before_it() {
    let dir = tempfile::tempdir().unwrap();
    let elsewhere = dir.path().join("elsewhere.attend");
    std::fs::write(&elsewhere, "<attendance/>").unwrap();
    let root = dir.path().join("project");
    std::fs::create_dir(&root).unwrap();
    let tree = old_system_folder(&root);
    std::os::unix::fs::symlink(&elsewhere, root.join("A.attend")).unwrap();
    std::fs::write(
        root.join("B.attend"),
        r#"<!DOCTYPE plume-attendance><attendance version="0.2" levelsNames="Main" rolesNames="None">
            <char number="5" firstName="Carol" lastName="Jones"/></attendance>"#,
    )
    .unwrap();

    let (summary, bundle) = import_old_system(&root, &tree);
    assert!(
        has(&bundle.binders[1], "Carol Jones"),
        "{:?}",
        summary.warnings
    );
}

/// With only a link there, it is still named in the summary rather than passed
/// over in silence.
#[test]
#[cfg(unix)]
fn old_system_attendance_that_is_only_a_link_is_named() {
    let dir = tempfile::tempdir().unwrap();
    let elsewhere = dir.path().join("elsewhere.attend");
    std::fs::write(&elsewhere, "<attendance/>").unwrap();
    let root = dir.path().join("project");
    std::fs::create_dir(&root).unwrap();
    let tree = old_system_folder(&root);
    std::os::unix::fs::symlink(&elsewhere, root.join("A.attend")).unwrap();

    let (summary, _bundle) = import_old_system(&root, &tree);
    assert!(names_unread(&summary, "A.attend"), "{:?}", summary.warnings);
}

/// Import `src` on another thread into `out`, and fail rather than hang if the
/// import does not come back within a few seconds.
#[cfg(unix)]
fn import_or_time_out(src: &Path, out: &Path) -> anyhow::Result<ImportSummary> {
    let src = src.to_string_lossy().into_owned();
    let out = out.to_string_lossy().into_owned();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(import(
            &src,
            &out,
            false,
            "Manuscript",
            "Story Bible",
            &ladder(),
        ));
    });
    rx.recv_timeout(std::time::Duration::from_secs(5))
        .expect("the import blocked instead of returning")
}

/// Assert `result` is the refusal of `file` as not a regular file: turned down
/// before it was read, not read until the system ran out of memory for it.
#[cfg(unix)]
fn assert_refused_as_not_a_regular_file(result: anyhow::Result<ImportSummary>, file: &str) {
    let refused = result.err().expect("a refusal");
    assert_eq!(
        refused.downcast_ref::<Refusal>(),
        Some(&Refusal::Unreadable {
            file: file.to_string(),
            reason: skrib_format::regular_file::not_a_regular_file().to_string(),
        }),
        "{refused:#}"
    );
}

/// A FIFO named like a project, picked by the writer (out of an unpacked
/// archive), blocked the import forever.
#[test]
#[cfg(unix)]
fn a_fifo_picked_as_the_plume_is_refused_without_blocking() {
    let dir = tempfile::tempdir().unwrap();
    let picked = dir.path().join("Novel.plume");
    let made = std::process::Command::new("mkfifo").arg(&picked).status();
    if !made.is_ok_and(|s| s.success()) {
        eprintln!("mkfifo is not available here; skipping");
        return;
    }
    let result = import_or_time_out(&picked, &dir.path().join("out.skrib"));
    assert_refused_as_not_a_regular_file(result, "Novel.plume");
}

/// A `.plume` that is a link to `/dev/zero` was read until memory ran out and the
/// app was killed.
#[test]
#[cfg(unix)]
fn a_plume_linked_to_a_device_is_refused_unread() {
    let dir = tempfile::tempdir().unwrap();
    let picked = dir.path().join("Novel.plume");
    std::os::unix::fs::symlink("/dev/zero", &picked).unwrap();
    let result = import_or_time_out(&picked, &dir.path().join("out.skrib"));
    assert_refused_as_not_a_regular_file(result, "Novel.plume");
}

/// A link the writer picked on purpose is followed, once, and an old-system
/// project read where it leads: its loose files beside the tree file itself.
#[test]
#[cfg(unix)]
fn an_old_system_tree_picked_through_a_link_reads_the_folder_it_points_into() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("project");
    std::fs::create_dir(&root).unwrap();
    let tree = old_system_folder(&root);
    std::fs::create_dir(root.join("text")).unwrap();
    std::fs::write(root.join("text/T3.html"), "<p>Loose scene prose.</p>").unwrap();
    let desktop = dir.path().join("Desktop");
    std::fs::create_dir(&desktop).unwrap();
    let shortcut = desktop.join("Loose.plume");
    std::os::unix::fs::symlink(&tree, &shortcut).unwrap();

    let out = desktop.join("out.skrib");
    import_or_time_out(&shortcut, &out).unwrap();
    let bundle = read_bundle(out.to_str().unwrap()).unwrap();
    let scene = find(&bundle.binders[0], "LooseScene");
    assert!(prose(scene, ContentRole::SceneText).contains("Loose scene prose."));
}

#[test]
fn rejects_non_plume_and_refuses_overwrite() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("bad.plume");
    // A zip with no `tree` member is not a Plume project.
    write_zip(&src, &[("random", "<x/>")]);
    let out = dir.path().join("out.skrib");
    let refused = import(
        src.to_str().unwrap(),
        out.to_str().unwrap(),
        false,
        "M",
        "S",
        &ladder(),
    )
    .err()
    .expect("a zip without an outline is refused");
    assert_eq!(
        refused.downcast_ref::<Refusal>(),
        Some(&Refusal::NoOutline),
        "{refused:#}"
    );
    assert!(!out.exists(), "no output must be written on failure");

    // Overwrite guard.
    let good = dir.path().join("good.plume");
    write_zip(&good, &terminal_members());
    let out2 = dir.path().join("exists.skrib");
    std::fs::write(&out2, "sentinel").unwrap();
    let refused = import(
        good.to_str().unwrap(),
        out2.to_str().unwrap(),
        false,
        "M",
        "S",
        &ladder(),
    )
    .err()
    .expect("an existing target is refused");
    assert_eq!(
        refused.downcast_ref::<skrib_format::importing::OutputRefused>(),
        Some(&skrib_format::importing::OutputRefused::TargetExists {
            file: "exists.skrib".to_string()
        })
    );
    assert_eq!(std::fs::read(&out2).unwrap(), b"sentinel");
    // ...but succeeds with overwrite=true.
    assert!(
        import(
            good.to_str().unwrap(),
            out2.to_str().unwrap(),
            true,
            "M",
            "S",
            &ladder()
        )
        .is_ok()
    );
}

/// However an import ends before its project is in place, nothing is left
/// beside the target: here a folder with files in it sits where the project
/// would go, which no final move can replace, so it is refused before anything
/// is read or written. (The final move failing is `skrib_format::importing`'s to
/// test: a folder appearing there while the import runs.)
#[test]
fn a_folder_at_the_target_leaves_no_partial_file() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("p.plume");
    write_zip(&src, &terminal_members());
    let out = dir.path().join("o.skrib");
    std::fs::create_dir(&out).unwrap();
    std::fs::write(out.join("keep"), b"x").unwrap();

    let refused = import(
        src.to_str().unwrap(),
        out.to_str().unwrap(),
        true,
        "M",
        "S",
        &ladder(),
    )
    .err()
    .expect("a folder at the target is refused");
    assert!(
        matches!(
            refused.downcast_ref::<skrib_format::importing::OutputRefused>(),
            Some(skrib_format::importing::OutputRefused::TargetIsFolder { .. })
        ),
        "{refused:#}"
    );
    let tmp = format!("{}.importing", out.to_str().unwrap());
    assert!(!Path::new(&tmp).exists(), "no partial file is left");
    assert!(out.join("keep").is_file(), "what was there is untouched");
}

/// A version newer than the importer knows is refused as that, naming the part
/// and both versions, so the writer can be told which Plume wrote it.
#[test]
fn a_newer_tree_is_refused_as_too_new() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("p.plume");
    write_zip(
        &src,
        &[(
            "tree",
            "<!DOCTYPE plume-tree><plume-tree version=\"0.9\" projectName=\"P\"/>",
        )],
    );
    let out = dir.path().join("o.skrib");
    let refused = import(
        src.to_str().unwrap(),
        out.to_str().unwrap(),
        false,
        "M",
        "S",
        &ladder(),
    )
    .err()
    .expect("refused");
    assert_eq!(
        refused.downcast_ref::<Refusal>(),
        Some(&Refusal::TooNew {
            part: "tree".to_string(),
            version: "0.9".to_string(),
            supported: "0.5".to_string(),
        })
    );
}
