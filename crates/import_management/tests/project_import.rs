// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The two project imports, end to end through the controller and a real
//! `LongOperationManager`: what they report crosses the manager's JSON round trip
//! whole, and reads back as the typed values the interface words.
//!
//! The manager stores a result as `serde_json::to_string` and completes with
//! nothing when that fails, so a result that did not survive serialisation would
//! reach the writer as an import that finished and said nothing at all. Progress
//! and failure cross as strings the same way. Each is checked here as the
//! interface receives it.

use std::sync::Arc;
use std::time::Duration;

use common::database::db_context::DbContext;
use common::event::EventHub;
use common::long_operation::{LongOperationManager, OperationStatus};
use common::types::EntityId;
use import_management::failure::{FailureReason, ProjectImportFailure};
use import_management::import_management_controller as controller;
use import_management::{
    ImportManuskriptProjectDto, ImportManuskriptProjectResultDto, ImportPlumeCreatorFileDto,
    ImportPlumeCreatorFileResultDto,
};
use skrib_format::import_notice::{ImportNotice, ProjectContainer, TextKind};
use skrib_format::importing::TITLE_SLOT;

struct Backend {
    db: DbContext,
    hub: Arc<EventHub>,
    long_ops: LongOperationManager,
}

impl Backend {
    fn new() -> Self {
        Self {
            db: DbContext::new().expect("in-memory store"),
            hub: Arc::new(EventHub::new()),
            long_ops: LongOperationManager::new(),
        }
    }

    /// Wait for `op` and return how it ended.
    fn finish(&self, op: &str) -> OperationStatus {
        assert!(
            self.long_ops
                .wait_for_operation(op, Some(Duration::from_secs(60))),
            "the import never finished"
        );
        match self.long_ops.get_operation_status(op) {
            Some(status) => status,
            None => panic!("no status for {op}"),
        }
    }

    /// The phase code the operation last reported.
    fn last_phase(&self, op: &str) -> Option<String> {
        self.long_ops.get_operation_progress(op)?.message
    }
}

/// Each item holding a scratchpad, by title, with the scratchpad as the editor reads it.
/// Each one is checked to be its own `.scratchpad.djot` file in the saved project.
fn scratchpads(bundle: &skrib_format::WorkBundle) -> Vec<(String, String)> {
    bundle
        .binders
        .iter()
        .flat_map(|b| b.items.iter())
        .filter_map(|item| {
            let pad = item
                .item
                .prose_refs
                .iter()
                .find(|p| p.role == common::entities::ContentRole::ScratchpadText)?;
            assert!(pad.path.ends_with(".scratchpad.djot"), "{}", pad.path);
            let djot = item.prose.get(&pad.file_id).expect("the file was read");
            let text = skrib_format::djot_plain_text(djot).expect("it reads").0;
            Some((item.item.title.clone(), text.trim().to_string()))
        })
        .collect()
}

/// Each item holding a scratchpad once `load_work` has opened `output`, by title, with
/// the scratchpad's stored Djot.
fn opened_scratchpads(output: &std::path::Path, media: &std::path::Path) -> Vec<(String, String)> {
    use common::direct_access::binder_item::BinderItemRelationshipField;
    use direct_access::binder_item::binder_item_controller;
    use direct_access::content::content_controller;
    use work_management::{LoadWorkDto, work_management_controller as work};
    let db = DbContext::new().expect("in-memory store");
    if let Err(e) = work::load_work(
        &db,
        &Arc::new(EventHub::new()),
        &LoadWorkDto {
            file_name: output.to_string_lossy().into_owned(),
            media_root: media.to_string_lossy().into_owned(),
        },
    ) {
        panic!("load_work refuses the imported project: {e:#}");
    }
    let rows: Vec<(EntityId, String)> = db
        .get_store()
        .binder_items
        .read()
        .expect("the store reads")
        .values()
        .map(|r| (r.id, r.title.clone()))
        .collect();
    let mut out: Vec<(String, String)> = rows
        .into_iter()
        .flat_map(|(id, title)| {
            binder_item_controller::get_relationship(
                &db,
                &id,
                &BinderItemRelationshipField::Contents,
            )
            .expect("its contents")
            .into_iter()
            .filter_map(|c| content_controller::get(&db, &c).expect("a content"))
            .filter(|c| c.role == common::entities::ContentRole::ScratchpadText)
            .map(move |c| (title.clone(), c.data))
            .collect::<Vec<_>>()
        })
        .collect();
    out.sort();
    out
}

/// The notices a result carries, read back as the interface reads them.
fn decoded(warnings: &[String]) -> Vec<ImportNotice> {
    warnings
        .iter()
        .map(|raw| match ImportNotice::decode(raw) {
            Some(notice) => notice,
            None => panic!("not an encoded notice: {raw}"),
        })
        .collect()
}

/// An old-style Plume project (a loose `.plume` outline file) holding one chapter
/// with nothing but a separator in it, which the importer has to drop and say so.
fn lonely_plume(dir: &std::path::Path) -> String {
    let path = dir.join("Lonely.plume");
    std::fs::write(
        &path,
        r#"<!DOCTYPE plume-tree><plume-tree version="0.5" projectName="Lonely">
            <book number="1" name="Book">
              <chapter number="2" name="Only a break">
                <separator number="10001" name="* * *"/>
              </chapter>
            </book>
          </plume-tree>"#,
    )
    .expect("fixture");
    path.to_string_lossy().into_owned()
}

fn plume_request(source: String, output: &std::path::Path) -> ImportPlumeCreatorFileDto {
    ImportPlumeCreatorFileDto {
        source_path: source,
        output_path: output.to_string_lossy().into_owned(),
        overwrite: false,
        manuscript_binder_name: "Manuscript".into(),
        story_bible_binder_name: "Story Bible".into(),
        status_names: Vec::new(),
        seeded_names: Vec::new(),
    }
}

#[test]
fn a_plume_import_s_notices_cross_the_result_channel_typed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut backend = Backend::new();
    let request = plume_request(lonely_plume(dir.path()), &dir.path().join("out.skrib"));
    let op = controller::import_plume_creator_file(
        &backend.db,
        &backend.hub,
        &mut backend.long_ops,
        &request,
    )
    .expect("started");
    assert_eq!(backend.finish(&op), OperationStatus::Completed);
    assert_eq!(backend.last_phase(&op).as_deref(), Some("import.done"));

    let result = controller::get_import_plume_creator_file_result(&backend.long_ops, &op)
        .expect("the result reads back")
        .expect("and there is one");
    let notices = decoded(&result.warnings);
    assert!(
        notices.contains(&ImportNotice::ChapterWithoutScenes {
            name: "Only a break".to_string()
        }),
        "{notices:?}"
    );
    assert!(dir.path().join("out.skrib").is_file());
}

#[test]
fn a_manuskript_import_s_notices_cross_the_result_channel_typed() {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../manuskript_import/tests/fixtures/tour-du-monde");
    let dir = tempfile::tempdir().expect("tempdir");
    let mut backend = Backend::new();
    let request = ImportManuskriptProjectDto {
        source_path: fixture.to_string_lossy().into_owned(),
        output_path: dir.path().join("out.skrib").to_string_lossy().into_owned(),
        overwrite: false,
        manuscript_binder_name: "Manuscript".into(),
        story_bible_binder_name: "Story bible".into(),
        characters_group_name: "Characters".into(),
        world_group_name: "World".into(),
        plots_group_name: "Plots".into(),
        project_info_note_name: "Project information".into(),
        summary_note_name: "Summary".into(),
        importance_names: vec!["Minor".into(), "Secondary".into(), "Main".into()],
        section_names: Vec::new(),
    };
    let op = controller::import_manuskript_project(
        &backend.db,
        &backend.hub,
        &mut backend.long_ops,
        &request,
    )
    .expect("started");
    assert_eq!(backend.finish(&op), OperationStatus::Completed);
    assert_eq!(backend.last_phase(&op).as_deref(), Some("import.done"));

    let result = controller::get_import_manuskript_project_result(&backend.long_ops, &op)
        .expect("the result reads back")
        .expect("and there is one");
    let notices = decoded(&result.warnings);
    assert!(
        matches!(
            notices.first(),
            Some(ImportNotice::ReadCopy {
                container: ProjectContainer::Folder,
                day: Some(_),
            })
        ),
        "the copy read comes first: {notices:?}"
    );
    assert!(
        notices
            .iter()
            .any(|n| matches!(n, ImportNotice::FolderWithoutMetadata { .. })),
        "{notices:?}"
    );
    // Twelve chapters' notes name the plot "Le pari": the notes are each chapter's
    // scratchpad, and the links they made are said to be lost.
    assert!(
        notices.contains(&ImportNotice::NoteLinksNotKept { count: 12 }),
        "{notices:?}"
    );
}

/// A refused project fails with the typed reason, readable by the interface from
/// the one string the manager keeps, and with the technical chain behind it.
#[test]
fn a_refusal_crosses_the_failure_channel_typed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let source = dir.path().join("Empty.plume");
    std::fs::write(&source, "   ").expect("fixture");
    let mut backend = Backend::new();
    let request = plume_request(
        source.to_string_lossy().into_owned(),
        &dir.path().join("out.skrib"),
    );
    let op = controller::import_plume_creator_file(
        &backend.db,
        &backend.hub,
        &mut backend.long_ops,
        &request,
    )
    .expect("started");
    let OperationStatus::Failed(message) = backend.finish(&op) else {
        panic!("an empty outline must be refused");
    };
    let failure = ProjectImportFailure::from_any_message(&message);
    assert_eq!(failure.reason, FailureReason::PlumeEmptyOutline);
    assert!(failure.details.contains("empty"), "{}", failure.details);
    assert!(
        !dir.path().join("out.skrib.importing").exists() && !dir.path().join("out.skrib").exists(),
        "nothing was written"
    );
}

/// Every notice either importer can send survives the JSON round trip the
/// manager puts a result through, field for field: encoded into the result's
/// `warnings`, serialised with it, and decoded back as the interface decodes it.
#[test]
fn every_notice_survives_the_json_round_trip() {
    let s = |t: &str| t.to_string();
    let plume_notices = vec![
        ImportNotice::FlattenedRichTexts { count: 2 },
        ImportNotice::FreeTextBadges { count: 4 },
        ImportNotice::SeparatorTextDropped { name: s("* * *") },
        ImportNotice::SeparatorWithoutScene { name: s("~") },
        ImportNotice::ChapterWithoutScenes {
            name: s("Une « pause »"),
        },
        ImportNotice::RichTextUnconverted {
            error: s("unexpected \"</p>\""),
        },
        ImportNotice::UnresolvedCrossLink { number: 7 },
    ];
    let plume = ImportPlumeCreatorFileResultDto {
        output_path: s("/out/novel.skrib"),
        imported_items: 3,
        skipped_trashed: 1,
        warnings: plume_notices.iter().map(ImportNotice::encode).collect(),
    };
    let json = serde_json::to_string(&plume).expect("serialises");
    let back: ImportPlumeCreatorFileResultDto = serde_json::from_str(&json).expect("reads back");
    assert_eq!(back, plume);
    assert_eq!(decoded(&back.warnings), plume_notices);

    let manuskript_notices = vec![
        ImportNotice::ReadCopy {
            container: ProjectContainer::SingleFile,
            day: Some(s("2026-03-04")),
        },
        ImportNotice::ReadCopy {
            container: ProjectContainer::Folder,
            day: None,
        },
        ImportNotice::OriginalFormat,
        ImportNotice::NewerSingleFileBeside {
            sibling: s("A Novel.msk"),
            sibling_day: s("2026-03-06"),
            folder_day: s("2026-03-04"),
        },
        ImportNotice::PickleSkipped {
            member: s("settings.pickle"),
        },
        ImportNotice::FileUnreadable {
            member: s("a.txt"),
            error: s("denied"),
        },
        ImportNotice::NotUtf8 { member: s("b.txt") },
        ImportNotice::TableUnreadable {
            member: s("perso.xml"),
            error: s("bad"),
        },
        ImportNotice::OutlineMissing {
            member: s("outline.xml"),
        },
        ImportNotice::OutlineUnreadable {
            member: s("outline.xml"),
            error: s("bad"),
        },
        ImportNotice::NamedNoneRow {
            member: s("labels.xml"),
            name: s("Draft"),
        },
        ImportNotice::UnknownField {
            member: s("infos.txt"),
            field: s("Mood"),
        },
        ImportNotice::ColourlessLabels {
            member: s("labels.txt"),
            count: 2,
        },
        ImportNotice::SettingsUnreadable {
            member: s("settings.txt"),
            error: s("EOF"),
        },
        ImportNotice::RevisionsUnreadable {
            member: s("revisions.xml"),
            error: s("bad"),
        },
        ImportNotice::ForeignFiles {
            count: 7,
            shown: vec![s("cover.png"), s("notes/idea.rtf")],
        },
        ImportNotice::NotACharacterFile {
            member: s("characters/x.png"),
        },
        ImportNotice::NotATextItem {
            member: s("outline/x.rtf"),
        },
        ImportNotice::MissingBlankLine {
            member: s("outline/0-A.md"),
        },
        ImportNotice::FolderWithoutMetadata {
            member: s("outline/0-Part"),
            metadata: s("folder.txt"),
        },
        ImportNotice::RowsGivenIds { count: 1 },
        ImportNotice::PlotsUnreadable {
            member: s("plots.xml"),
            error: s("bad"),
        },
        ImportNotice::WorldUnreadable {
            member: s("world.opml"),
            error: s("bad"),
        },
        ImportNotice::WorldNotOpml {
            member: s("world.opml"),
        },
        ImportNotice::TextFlattened {
            title: s("Opening"),
        },
        ImportNotice::TextUnconverted {
            title: s("Opening"),
            error: s("bad"),
        },
        ImportNotice::TextWithoutFileName {
            title: s("A"),
            text: TextKind::Scene,
        },
        ImportNotice::TextWithoutFileName {
            title: s("B"),
            text: TextKind::Scratchpad,
        },
        ImportNotice::UnknownLabel {
            title: s("A"),
            index: 9,
        },
        ImportNotice::UnknownStatus {
            title: s("A"),
            index: 9,
        },
        ImportNotice::UnknownPointOfView { title: s("A") },
        ImportNotice::UnresolvedReferences {
            title: s("A"),
            error: s("bad"),
        },
        ImportNotice::CustomIcons { count: 3 },
        ImportNotice::CharacterColours { count: 2 },
        ImportNotice::VersionsKept {
            found: 40,
            kept: 12,
        },
        ImportNotice::VersionsUnplaced { count: 1 },
        ImportNotice::NoteLinksNotKept { count: 12 },
    ];
    let manuskript = ImportManuskriptProjectResultDto {
        output_path: s("/out/novel.skrib"),
        imported_items: 12,
        imported_revisions: 4,
        warnings: manuskript_notices
            .iter()
            .map(ImportNotice::encode)
            .collect(),
    };
    let json = serde_json::to_string(&manuskript).expect("serialises");
    let back: ImportManuskriptProjectResultDto = serde_json::from_str(&json).expect("reads back");
    assert_eq!(back, manuskript);
    assert_eq!(decoded(&back.warnings), manuskript_notices);
}

/// Every title an import makes up is the interface's word, in its language: with a French
/// interface, a Plume project that names itself nowhere arrives with French titles, and
/// nothing English is written into either import. The notes of a Plume scene and of a
/// Manuskript row make up no title: each arrives in its own item's scratchpad, saved as
/// that item's `.scratchpad.djot` file.
#[test]
fn the_titles_an_import_makes_up_are_the_interface_s_words() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut backend = Backend::new();

    // An old-style Plume project with no name anywhere, a scene with notes of its own.
    let plume = dir.path().join("plume");
    std::fs::create_dir_all(plume.join("text")).expect("fixture");
    std::fs::write(
        plume.join("Loose.plume"),
        r#"<!DOCTYPE plume><plume version="0.2" projectName="">
            <book number="1" name="Livre"><chapter number="2" name="Chapitre">
              <scene number="3" name="Scène"/></chapter></book></plume>"#,
    )
    .expect("fixture");
    std::fs::write(
        plume.join("Loose.attend"),
        r#"<!DOCTYPE plume-attendance><attendance version="0.2" levelsNames="Principal" rolesNames="Aucun">
            <char number="5" firstName="Carole" lastName="Martin"/>
            <place number="6" name="Paris"/></attendance>"#,
    )
    .expect("fixture");
    std::fs::write(plume.join("text/T3.html"), "<p>Des mots.</p>").expect("fixture");
    std::fs::write(plume.join("text/N3.html"), "<p>Une note.</p>").expect("fixture");
    let output = dir.path().join("plume.skrib");
    // In `plume_import::SeededNames::ORDER`.
    let seeded = |key: &str| match key {
        "characters_group" => "Personnages".to_string(),
        "items_group" => "Objets".to_string(),
        "places_group" => "Lieux".to_string(),
        "untitled_project" => "Projet Plume importé".to_string(),
        "content_of" => format!("{TITLE_SLOT} (texte)"),
        other => panic!("a seeded name this test does not know: {other}"),
    };
    let request = ImportPlumeCreatorFileDto {
        manuscript_binder_name: "Manuscrit".into(),
        story_bible_binder_name: "Personnages et lieux".into(),
        seeded_names: plume_import::SeededNames::ORDER
            .iter()
            .map(|key| seeded(key))
            .collect(),
        ..plume_request(
            plume.join("Loose.plume").to_string_lossy().into_owned(),
            &output,
        )
    };
    let op = controller::import_plume_creator_file(
        &backend.db,
        &backend.hub,
        &mut backend.long_ops,
        &request,
    )
    .expect("started");
    assert_eq!(backend.finish(&op), OperationStatus::Completed);
    let bundle = skrib_format::read_bundle(&output.to_string_lossy()).expect("reads back");
    assert_eq!(bundle.manifest.work.title, "Projet Plume importé");
    let titles: Vec<&str> = bundle
        .binders
        .iter()
        .flat_map(|b| b.items.iter())
        .map(|i| i.item.title.as_str())
        .collect();
    assert!(
        !titles.iter().any(|t| t.contains("notes")),
        "no note beside the scene: {titles:?}"
    );
    assert_eq!(
        scratchpads(&bundle),
        [("Scène".to_string(), "Une note.".to_string())],
        "the scene's note is its scratchpad"
    );
    let opened = opened_scratchpads(&output, &dir.path().join("media-plume"));
    assert_eq!(opened.len(), 1, "{opened:?}");
    assert_eq!(opened[0].0, "Scène");
    assert!(opened[0].1.contains("Une note."), "{opened:?}");
    assert_eq!(bundle.binders[0].binder.name, "Manuscrit");
    // The story bible's groups of a project older than 0.4, which named none of them.
    let tags: Vec<&str> = bundle.tags.iter().map(|t| t.name.as_str()).collect();
    assert!(
        tags.contains(&"Personnages") && tags.contains(&"Lieux"),
        "{tags:?}"
    );
    for english in ["Characters", "Items", "Places"] {
        assert!(
            !tags.contains(&english) && !titles.contains(&english),
            "{english}"
        );
    }

    // A Manuskript project whose rows carry notes.
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../manuskript_import/tests/fixtures/tour-du-monde");
    let output = dir.path().join("manuskript.skrib");
    let request = ImportManuskriptProjectDto {
        source_path: fixture.to_string_lossy().into_owned(),
        output_path: output.to_string_lossy().into_owned(),
        overwrite: false,
        manuscript_binder_name: "Manuscrit".into(),
        story_bible_binder_name: "Bible de l’histoire".into(),
        characters_group_name: "Personnages".into(),
        world_group_name: "Univers".into(),
        plots_group_name: "Intrigues".into(),
        project_info_note_name: "Fiche du projet".into(),
        summary_note_name: "Résumé".into(),
        importance_names: vec!["Mineur".into(), "Secondaire".into(), "Principal".into()],
        // In `manuskript_import::map::SectionNames::ORDER`; a blank keeps the English
        // heading, which none of these titles uses.
        section_names: manuskript_import::map::SectionNames::ORDER
            .iter()
            .map(|key| match *key {
                "untitled_project" => "Projet Manuskript importé".to_string(),
                _ => String::new(),
            })
            .collect(),
    };
    let op = controller::import_manuskript_project(
        &backend.db,
        &backend.hub,
        &mut backend.long_ops,
        &request,
    )
    .expect("started");
    assert_eq!(backend.finish(&op), OperationStatus::Completed);
    let bundle = skrib_format::read_bundle(&output.to_string_lossy()).expect("reads back");
    let titles: Vec<String> = bundle
        .binders
        .iter()
        .flat_map(|b| b.items.iter())
        .map(|i| i.item.title.clone())
        .collect();
    assert!(
        !titles.iter().any(|t| t.contains("notes")),
        "no note beside a row: {titles:?}"
    );
    let pads = scratchpads(&bundle);
    assert_eq!(pads.len(), 12, "every chapter with notes: {pads:?}");
    for (title, text) in &pads {
        assert_eq!(
            text, "Le pari traverse ce chapitre.",
            "{title}: the marker is written out as the plot's name"
        );
    }
    let opened = opened_scratchpads(&output, &dir.path().join("media-manuskript"));
    assert_eq!(opened.len(), 12, "{opened:?}");
    for words in [&titles, &vec![bundle.manifest.work.title.clone()]] {
        assert!(
            !words
                .iter()
                .any(|t| t.contains("(notes)") || t.contains("(text)") || t.contains("Imported")),
            "{words:?}"
        );
    }
}
