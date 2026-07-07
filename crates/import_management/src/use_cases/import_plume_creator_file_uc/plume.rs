//! Plume Creator (`.plume`) → newest-version `.skrib` importer.
//!
//! One-directional, version-neutral, and a **pure file→file transform**: it reads
//! any Plume project — any historical schema version, the modern single-file zip
//! **or** the pre-0.3 old-system directory — and writes a `.skrib` zip at the
//! newest format version. The entity store is never touched; the UI loads the
//! result afterward via the existing `load_work`.

mod attend_parse;
mod dict_parse;
mod info_parse;
mod map;
mod model;
mod source;
mod tree_parse;
mod version;

use std::path::Path;

use anyhow::{Context, Result, bail};
use skrib_format::{SkribShape, write_bundle};

use model::{PlumeAttendance, PlumeInfo};
use source::PlumeSource;

/// What the import produced, for the UI's post-import summary.
pub struct ImportSummary {
    pub output_path: String,
    /// Total binder items written (both binders, including synthetic helpers).
    pub imported_items: u64,
    /// Meaningful (non-separator) Plume nodes that were trashed and skipped.
    pub skipped_trashed: u64,
    pub warnings: Vec<String>,
}

/// Convert the Plume project at `source_path` into a `.skrib` zip at `output_path`.
pub fn import(
    source_path: &str,
    output_path: &str,
    overwrite: bool,
    manuscript_binder_name: &str,
    story_bible_binder_name: &str,
) -> Result<ImportSummary> {
    if !overwrite && Path::new(output_path).exists() {
        bail!("'{output_path}' already exists (choose another name or allow overwrite)");
    }

    let src = PlumeSource::open(source_path)?;

    let tree = tree_parse::parse(&src.tree_xml).context("reading the Plume outline (tree)")?;
    let attendance = match &src.attendance_xml {
        Some(xml) => {
            attend_parse::parse(xml).context("reading the Plume story bible (attendance)")?
        }
        None => PlumeAttendance {
            spinbox_label: String::new(),
            groups: Vec::new(),
        },
    };
    // `info` is non-critical metadata (title + dates); a malformed one just falls
    // back to the tree's project name.
    let info = match &src.info_xml {
        Some(xml) => info_parse::parse(xml).unwrap_or_default(),
        None => PlumeInfo::default(),
    };
    let dict_words = src.dict.as_deref().map(dict_parse::parse).unwrap_or_default();

    let mapped = map::build_bundle(
        &tree,
        &attendance,
        &info,
        &dict_words,
        &src,
        manuscript_binder_name,
        story_bible_binder_name,
    );

    write_bundle(output_path, SkribShape::ZipFile, &mapped.bundle)
        .with_context(|| format!("writing '{output_path}'"))?;

    Ok(ImportSummary {
        output_path: output_path.to_string(),
        imported_items: mapped.imported_items,
        skipped_trashed: mapped.skipped_trashed,
        warnings: mapped.warnings,
    })
}

#[cfg(test)]
mod tests {
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
        let opts = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        for (name, content) in members {
            zip.start_file(*name, opts).unwrap();
            zip.write_all(content.as_bytes()).unwrap();
        }
        zip.finish().unwrap();
    }

    /// A modern (terminal-version) project exercising every structure.
    fn terminal_members() -> Vec<(&'static str, &'static str)> {
        vec![
            ("info", r#"<!DOCTYPE plume-information><plume-information version="0.3"><prj name="Sample Novel" creationDate="2015-01-02T03:04:05" lastModified="2016-02-03T04:05:06"/></plume-information>"#),
            ("tree", r#"<!DOCTYPE plume-tree><plume-tree version="0.5" projectName="Sample">
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
              </plume-tree>"#),
            ("attendance", r#"<!DOCTYPE plume-attendance><plume-attendance version="0.6" box_1="Main--Secondary" box_2="None--Protagonist" spinBox_1_label="Age :">
                <group number="40" name="Characters">
                  <obj number="10" name="Alice" aliases="Al" quickDetails="The hero" box_1="0" box_2="1" spinBox_1="30"/>
                </group></plume-attendance>"#),
            ("dicts/userDict.dict_plume", "wibble;wobble;"),
            // Prose (T4 wrapped in a Qt-style doc to prove <style> stripping).
            ("text/T4.html", r#"<html><head><style type="text/css">p{margin:0}</style></head><body><p>Prose of scene 1.1.</p></body></html>"#),
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
            .unwrap_or_else(|| panic!("no item titled '{title}' in binder '{}'", binder.binder.name))
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
    fn terminal_project_full_structure() {
        let (summary, bundle) = import_zip(&terminal_members());

        // Trashed: Trashed Chapter + Trashed Scene + Deleted Book = 3.
        assert_eq!(summary.skipped_trashed, 3);
        assert_eq!(summary.imported_items, 16);
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
        assert_eq!((book1.item.role.clone(), book1.item.sub_role.clone()), (Role::Folder, SubRole::Book));
        assert_eq!(book1.item.indent, 0);
        assert_eq!(inline(book1, ContentRole::BookTitle), "Book One");

        let act = find(manuscript, "Act I");
        assert_eq!((act.item.role.clone(), act.item.sub_role.clone()), (Role::Folder, SubRole::Part));
        assert_eq!(act.item.indent, 1);
        assert_eq!(inline(act, ContentRole::PartTitle), "Act I");
        assert!(prose(act, ContentRole::SynopsisText).contains("The first act."));

        let chap1 = find(manuscript, "Chapter 1");
        assert_eq!((chap1.item.role.clone(), chap1.item.sub_role.clone()), (Role::Folder, SubRole::Chapter));
        assert_eq!(chap1.item.indent, 2);

        let scene11 = find(manuscript, "Scene 1.1");
        assert_eq!((scene11.item.role.clone(), scene11.item.sub_role.clone()), (Role::Item, SubRole::Scene));
        assert_eq!(scene11.item.indent, 3);
        assert!(prose(scene11, ContentRole::SceneText).contains("Prose of scene 1.1."));
        // The Qt <style> block must not leak into the prose.
        assert!(!prose(scene11, ContentRole::SceneText).contains("margin"));

        // Scene note became a following-sibling Note.
        let scene11_notes = find(manuscript, "Scene 1.1 (notes)");
        assert_eq!(scene11_notes.item.sub_role, SubRole::Note);
        assert_eq!(scene11_notes.item.indent, 3);
        assert!(prose(scene11_notes, ContentRole::NoteText).contains("Note for scene 1.1."));

        // Separator → a titled Text marker with no content.
        let sep = find(manuscript, "* * *");
        assert_eq!(sep.item.sub_role, SubRole::Text);
        assert!(sep.item.prose_refs.is_empty() && sep.item.inline_contents.is_empty());

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
        let book_ends = manuscript.items.iter().filter(|i| i.item.sub_role == SubRole::BookEnd).count();
        assert_eq!(book_ends, 2);

        // Trashed nodes are absent.
        assert!(!has(manuscript, "Trashed Chapter"));
        assert!(!has(manuscript, "Trashed Scene"));
        assert!(!has(manuscript, "Deleted Book"));

        // --- story bible + cross-links ---
        let characters = find(story, "Characters");
        assert_eq!((characters.item.role.clone(), characters.item.sub_role.clone()), (Role::Folder, SubRole::None));
        let alice = find(story, "Alice");
        assert_eq!(alice.item.sub_role, SubRole::Note);
        assert!(prose(alice, ContentRole::NoteText).contains("Alice is the hero."));
        let syn = prose(alice, ContentRole::SynopsisText);
        assert!(syn.contains("The hero") && syn.contains("Al") && syn.contains("Main")
            && syn.contains("Protagonist") && syn.contains("Age : 30"), "synopsis was: {syn:?}");

        // Scene 1.1's attend="-10" resolved to Alice's item id.
        assert_eq!(scene11.item.reference_ids, vec![alice.item.file_id]);

        // Dictionary.
        let words: Vec<&str> = bundle.dict_words.iter().map(|w| w.word.as_str()).collect();
        assert!(words.contains(&"wibble") && words.contains(&"wobble"));
    }

    #[test]
    fn old_zip_schema_is_normalized() {
        // tree 0.4 (root plume-tree, no <trash>, separator without number),
        // attendance 0.3 (legacy char/item/place + level/role, no <group>).
        let members = vec![
            ("info", r#"<!DOCTYPE plume-information><plume-information version="0.3"><prj name="Old Project"/></plume-information>"#),
            ("tree", r#"<!DOCTYPE plume-tree><plume-tree version="0.4" projectName="Old">
                <book number="1" name="OldBook"><chapter number="2" name="OldChap">
                  <scene number="3" name="OldScene"/></chapter></book></plume-tree>"#),
            ("attendance", r#"<!DOCTYPE plume-attendance><attendance version="0.3" levelsNames="Main--Secondary" rolesNames="None--Protagonist">
                <char number="5" firstName="Bob" lastName="Smith" level="1" role="1"/>
                <place number="6" name="Town"/></attendance>"#),
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
        assert!(syn.contains("Secondary") && syn.contains("Protagonist"), "synopsis was: {syn:?}");
        assert!(has(story, "Town"));
    }

    #[test]
    fn old_system_bare_directory_is_imported() {
        // Pre-0.3 layout: loose *.plume (root <plume>) + *.attend + *.prjinfo + text/.
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::write(root.join("Loose.plume"), r#"<!DOCTYPE plume><plume version="0.2" projectName="LooseProj">
            <book number="1" name="LooseBook"><chapter number="2" name="LooseChap">
              <scene number="3" name="LooseScene"/></chapter></book></plume>"#).unwrap();
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
        assert!(import(src.to_str().unwrap(), out.to_str().unwrap(), false, "M", "S").is_err());
        assert!(!out.exists(), "no output must be written on failure");

        // Overwrite guard.
        let good = dir.path().join("good.plume");
        write_zip(&good, &terminal_members());
        let out2 = dir.path().join("exists.skrib");
        std::fs::write(&out2, "sentinel").unwrap();
        assert!(import(good.to_str().unwrap(), out2.to_str().unwrap(), false, "M", "S").is_err());
        // ...but succeeds with overwrite=true.
        assert!(import(good.to_str().unwrap(), out2.to_str().unwrap(), true, "M", "S").is_ok());
    }
}
