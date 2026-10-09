// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The typed rows, field for field: every kind of refusal and notice becomes a row whose
//! key is listed, whose arguments are named as the converter names its fields and whose
//! closed choices are tokens a Fluent selector can match; items resolve to where the
//! writer knew them and to the row that holds them now.

use scrivener_import::metadata::{AuthorSource, TitleSource};
use scrivener_import::model::{Generation, ItemKey, TextRole};
use scrivener_import::outcome::{
    BackupSign, DanglingKind, ManifestDamage, ReadProblem, TemplateDamage, UnsafeName,
};
use scrivener_import::prose::{CompileTreatment, Formatting, LeftOut, RangeReading};
use scrivener_import::structure::{
    BinderKind, ContainerRung, Destination, Level, Origin, Placement, PlanError, RowRef,
    StructureNotice, TextKind, TrashChoice,
};
use scrivener_import::{
    MetadataNotice, Notice, Place, ProseNotice, Refusal, SourceNames, WrittenRow,
};
use uuid::Uuid;

use super::*;
use crate::dtos::{
    ScrivenerArg, ScrivenerBooksChoice, ScrivenerChoice, ScrivenerContainerRung,
    ScrivenerDefaultRung, ScrivenerDestination, ScrivenerFindingFamily, ScrivenerPlace,
    ScrivenerReportGroup, ScrivenerSiblingChoice, ScrivenerTextKind, ScrivenerTrashChoice,
};

fn s(text: &str) -> String {
    text.to_string()
}

/// A binder of four items: a Draft, a chapter, a scene in it, and a note beside.
fn names() -> SourceNames {
    let key = |n: u128| Some(ItemKey::Uuid(Uuid::from_u128(n)));
    SourceNames {
        items: vec![
            vec![s("Draft")],
            vec![s("Draft"), s("Chapter 1")],
            vec![s("Draft"), s("Chapter 1"), s("Arrival")],
            vec![s("Ideas")],
        ],
        keys: vec![key(1), key(2), key(3), key(3)],
        owns_key: vec![true, true, true, false],
        project_notes: vec![s("Plot")],
        // A recovered text is named by the document number its file's name gives.
        recovered: vec![vec![s("630")]],
    }
}

fn uid(n: u128) -> Uuid {
    Uuid::from_u128(n)
}

fn manuscript(row: u32) -> RowRef {
    RowRef {
        binder: BinderKind::Manuscript,
        row,
    }
}

fn notes(row: u32) -> RowRef {
    RowRef {
        binder: BinderKind::Notes,
        row,
    }
}

/// Where the four items went: the scene's notes in a row of their own, its synopsis on its
/// own row, a project note and a recovered text in the notes binder.
fn written() -> (Vec<Placement>, Vec<WrittenRow>) {
    let placements = vec![
        Placement::default(),
        Placement {
            item: Some(manuscript(1)),
            ..Placement::default()
        },
        Placement {
            item: Some(manuscript(2)),
            text: Some((
                manuscript(2),
                scrivener_import::structure::ContentRole::SceneText,
            )),
            notes: Some(manuscript(3)),
            synopsis: Some(manuscript(2)),
            ..Placement::default()
        },
        Placement {
            item: Some(notes(0)),
            ..Placement::default()
        },
    ];
    let rows = vec![
        WrittenRow {
            at: manuscript(1),
            uid: uid(11),
            origin: Origin::Item { item: 1 },
        },
        WrittenRow {
            at: manuscript(2),
            uid: uid(12),
            origin: Origin::Item { item: 2 },
        },
        WrittenRow {
            at: manuscript(3),
            uid: uid(13),
            origin: Origin::DocumentNotes { item: 2 },
        },
        WrittenRow {
            at: notes(0),
            uid: uid(20),
            origin: Origin::Item { item: 3 },
        },
        WrittenRow {
            at: notes(1),
            uid: uid(21),
            origin: Origin::ProjectNote { note: 0 },
        },
        WrittenRow {
            at: notes(2),
            uid: uid(22),
            origin: Origin::RecoveredText { text: 0 },
        },
    ];
    (placements, rows)
}

/// The parts of a reported row.
struct Row {
    key: String,
    family: ScrivenerFindingFamily,
    group: ScrivenerReportGroup,
    place: ScrivenerPlace,
    item: i64,
    path: Vec<String>,
    uid: Option<Uuid>,
    args: Vec<ScrivenerArg>,
}

fn row(finding: ScrivenerFinding) -> Row {
    match finding {
        ScrivenerFinding::Reported {
            key,
            family,
            group,
            place,
            item,
            binder_path,
            item_uid,
            args,
        } => Row {
            key,
            family,
            group,
            place,
            item,
            path: binder_path,
            uid: item_uid,
            args,
        },
        ScrivenerFinding::Empty => panic!("an empty finding was sent"),
    }
}

/// One of every refusal.
fn every_refusal() -> Vec<Refusal> {
    let examples = vec![s("Files/Data/A/content.rtf")];
    vec![
        Refusal::Missing { file: s("a.scriv") },
        Refusal::Unreadable {
            file: s("a.scriv"),
            reason: s("denied"),
        },
        Refusal::NotAScrivenerProject,
        Refusal::ManifestWithoutFiles {
            manifest: s("a.scrivx"),
        },
        Refusal::SearchTooWide { limit: 400 },
        Refusal::ScrivenerOneForMac,
        Refusal::IosStubWithoutMobileData,
        Refusal::UnknownFormatVersion { version: s("3.0") },
        Refusal::NewerFormatStamp {
            stamp: 24,
            newest: 23,
        },
        Refusal::VersionStampUnreadable,
        Refusal::DamagedManifest {
            manifest: s("a.scrivx"),
            damage: ManifestDamage::TooDeep {
                depth: 300,
                line: 4,
            },
            spare_copies: 2,
        },
        Refusal::DamagedManifest {
            manifest: s("a.scrivx"),
            damage: ManifestDamage::NotXml { reason: s("eof") },
            spare_copies: 0,
        },
        Refusal::ManifestTooLarge {
            manifest: s("a.scrivx"),
            limit_mib: 64,
        },
        Refusal::NoItemIdentifiers,
        Refusal::IncompleteCopy,
        Refusal::ArchiveUnreadable {
            file: s("a.zip"),
            reason: s("crc"),
        },
        Refusal::ArchiveUnsafeName {
            file: s("a.zip"),
            member: s("../x"),
            why: UnsafeName::ParentFolder,
        },
        Refusal::ArchiveTooManyMembers { limit: 100_000 },
        Refusal::ArchiveMemberTooLarge {
            member: s("x"),
            limit_mib: 64,
        },
        Refusal::ArchiveTooLarge { limit_mib: 512 },
        Refusal::ArchiveEncrypted { file: s("a.zip") },
        Refusal::ArchiveUnsupportedCompression {
            member: s("x"),
            method: s("lzma"),
        },
        Refusal::DuplicateArchiveMembers {
            file: s("a.zip"),
            member: Some(s("x")),
        },
        Refusal::TemplateUndecodable {
            file: s("a.scrivtemplate"),
            damage: TemplateDamage::UnknownEncoding {
                encoding: s("rot13"),
            },
        },
        Refusal::CloudPlaceholders {
            count: 3,
            examples: examples.clone(),
        },
        Refusal::UnreadableMembers { count: 1, examples },
        Refusal::NeededMemberIsLink { member: s("x") },
        Refusal::ProjectChangedDuringRead { count: 2 },
        Refusal::TooManyMembers { limit: 200_000 },
        Refusal::TooLarge { limit_mib: 1024 },
        Refusal::StructureUnmappable {
            error: PlanError::FactsMismatch { items: 3, facts: 2 },
        },
    ]
}

/// One of every notice the analysis gives, the keyed ones naming the scene.
fn every_notice() -> Vec<Notice> {
    let scene = ItemKey::Uuid(Uuid::from_u128(3));
    vec![
        Notice::ProjectFoundBelowPick { path: s("a/b") },
        Notice::SourceIsTemplate {
            title: Some(s("Novel")),
        },
        Notice::SourceIsBackup {
            sign: BackupSign::DateStamped,
        },
        Notice::ImportingPreUpgradeCopy,
        Notice::ImportedFromIosStub,
        Notice::StaleMobileBinderIgnored,
        Notice::IdenticalManifestCopies { count: 2 },
        Notice::ManifestCopiesIgnored {
            chosen: s("a.scrivx"),
            copies: vec![s("a 2.scrivx")],
        },
        Notice::PinnedManifestLooksLikeACopy {
            name: s("a 2.scrivx"),
            original: s("a.scrivx"),
        },
        Notice::PinnedManifestHasSiblings { others: 1 },
        Notice::LegacyV1FilesBeside { count: 4 },
        Notice::MixedPayloadTrees,
        Notice::IdSchemeMismatch { version: s("1.5") },
        Notice::VersionStampMismatch {
            stamp: 18,
            version: s("2.0"),
        },
        Notice::UnrecognisedWriter {
            creator: Some(s("SCRWIN-CONFLICT")),
        },
        Notice::InvalidItemIdentifier { count: 1 },
        Notice::ItemsWithoutIdentifier { count: 1 },
        Notice::DuplicateItemKey { key: scene },
        Notice::LockPresent { this_machine: true },
        Notice::ArchivedWhileOpen,
        Notice::ContentMissing {
            item: scene,
            role: TextRole::Notes,
            salvaged: true,
        },
        Notice::ContentTruncated {
            item: scene,
            role: TextRole::Text,
        },
        Notice::MissingMedia { item: scene },
        Notice::SyncConflictCopies {
            count: 2,
            original_missing: 1,
        },
        Notice::RecoveredUnreferencedText { count: 1 },
        Notice::SyncDebrisIgnored { count: 9 },
        Notice::UnrecognisedFiles {
            count: 1,
            examples: vec![s("x.tmp")],
        },
        Notice::SymlinkNotFollowed {
            count: 1,
            examples: vec![s("link")],
        },
        Notice::UnreadableFilesSkipped {
            count: 1,
            examples: vec![s("y")],
        },
        Notice::CloudPlaceholdersSkipped { count: 2 },
        Notice::NoTextFound { text_items: 3 },
        Notice::MetadataUnreadable {
            member: s("Settings/compile.xml"),
            problem: ReadProblem::TooDeep {
                depth: 300,
                line: 2,
            },
        },
        Notice::DanglingReferences {
            kind: DanglingKind::SectionType,
            count: 2,
        },
        Notice::SnapshotsNotImported {
            snapshots: 5,
            documents: 2,
            of_deleted: 1,
        },
        Notice::WritingHistoryNotImported {
            days: 3,
            first_day: Some(s("2024-03-02")),
            last_day: Some(s("2024-04-01")),
            unreadable: 0,
        },
        Notice::TitlePlaceholderRejected {
            source: TitleSource::CompileSettings,
        },
        Notice::AuthorPlaceholderRejected {
            source: AuthorSource::ProjectProperties,
        },
        Notice::TitleFromProject {
            source: TitleSource::ProjectProperties,
        },
        Notice::AuthorFromProject {
            source: AuthorSource::CompileSettings,
        },
        Notice::LanguageNotStated,
        Notice::LanguageFromScript { tag: s("ja") },
    ]
}

fn every_structure_notice() -> Vec<StructureNotice> {
    vec![
        StructureNotice::ScriptModeDraft {
            script_documents: 4,
            draft_documents: 6,
        },
        StructureNotice::OneDocumentManuscript {
            item: 2,
            headings: 3,
            split: true,
        },
        StructureNotice::TitleRepeatsNumber { item: 1 },
        StructureNotice::ClaimOverruled {
            item: 2,
            claimed: Level::Chapter,
        },
        StructureNotice::AdoptedByChapter {
            item: 2,
            chapter: 1,
        },
        StructureNotice::LooksLikeManuscript { item: 3 },
        StructureNotice::GenericSheetTitle { item: 3 },
        StructureNotice::PlaceholderTitle { item: 1 },
        StructureNotice::UntitledItem { item: 1 },
        StructureNotice::TitleCleaned { item: 1 },
        StructureNotice::OwnTextMoved { item: 1 },
        StructureNotice::TextWithChildren { item: 1 },
        StructureNotice::EmptyFolderPlain { item: 1 },
        StructureNotice::BookInsideBook { item: 1 },
        StructureNotice::PrologueNotNumbered { item: 1 },
        StructureNotice::MatterInsideBook {
            item: 3,
            back: true,
        },
        StructureNotice::ParatextNotCounted {
            documents: 2,
            words: 900,
        },
        StructureNotice::DocumentNotes { item: 2 },
        StructureNotice::DraftContentAsNote { item: 0 },
        StructureNotice::StoryBibleFolder {
            item: 3,
            template: false,
        },
        StructureNotice::TemplatesCreated { count: 2 },
        StructureNotice::TemplateNestingFlattened { item: 3 },
        StructureNotice::TemplateKeptAsNote { item: 3 },
        StructureNotice::RecoveredTexts { count: 1 },
        StructureNotice::ProjectNotes { count: 1 },
        StructureNotice::TrashBroughtAcross {
            items: 3,
            with_writing: 2,
        },
        StructureNotice::NotCompiled { item: 2 },
        StructureNotice::MediaInDraft { item: 2 },
        StructureNotice::LeftOut { item: 3, items: 5 },
        StructureNotice::TrashLeftOut {
            items: 3,
            with_writing: 1,
        },
        StructureNotice::CardImageLeftBehind { item: 2 },
        StructureNotice::DefaultTemplateNotLinked { item: 3 },
        StructureNotice::PageBreakNotKept { item: 2 },
        StructureNotice::CompileAsIsNotKept { item: 2 },
    ]
}

fn every_metadata_notice() -> Vec<MetadataNotice> {
    // One of every kind, as the converter keeps them: a kind added there is a row here.
    scrivener_import::metadata::every_example()
}

/// A sample of the prose notices: one per shape of argument, each place.
fn some_prose_notices() -> Vec<ProseNotice> {
    vec![
        ProseNotice::Unreadable {
            at: Place::Text { item: 2 },
        },
        ProseNotice::BlankLinesRemoved {
            at: Place::Notes { item: 2 },
            count: 3,
        },
        ProseNotice::RangeCommentReading {
            at: Place::Synopsis { item: 2 },
            reading: RangeReading::PicturesNotCounted,
            count: 1,
        },
        ProseNotice::LinkedPictureNotRead {
            at: Place::Media { item: 3 },
            name: s("map.png"),
        },
        ProseNotice::MarkdownAsText {
            at: Place::CardImage { item: 2 },
            paragraphs: 4,
        },
        ProseNotice::PlaceholdersInExportedText {
            at: Place::ProjectNote { note: 0 },
            kind: s("<$wc>"),
            count: 2,
        },
        ProseNotice::CompiledAwayStylePrinted {
            at: Place::Recovered { text: 0 },
            name: s("Note to self"),
            treatment: CompileTreatment::RawMarkup,
            count: 2,
        },
        ProseNotice::FormattingLost {
            at: Place::Text { item: 2 },
            what: Formatting::SmallCaps,
            count: 7,
        },
        ProseNotice::LeftOut {
            at: Place::Text { item: 2 },
            what: LeftOut::HeaderOrFooter,
            count: 1,
        },
        ProseNotice::MediaFileCopied {
            at: Place::Media { item: 3 },
            path: s("Novel research/tides.pdf"),
        },
        ProseNotice::Document {
            at: Place::Text { item: 2 },
            key: s("comment-unanchored"),
            warning: true,
            count: 1,
            detail: s("The tide"),
        },
    ]
}

fn is_snake_case(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn is_kebab_case(token: &str) -> bool {
    !token.is_empty()
        && !token.starts_with('-')
        && !token.ends_with('-')
        && token
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// A row's arguments are named in snake case, once each, and its tokens are kebab case.
fn check_args(row: &Row) {
    let mut seen = std::collections::BTreeSet::new();
    for arg in &row.args {
        let name = match arg {
            ScrivenerArg::Count { name, .. }
            | ScrivenerArg::Text { name, .. }
            | ScrivenerArg::Texts { name, .. }
            | ScrivenerArg::Flag { name, .. }
            | ScrivenerArg::Day { name, .. }
            | ScrivenerArg::Item { name, .. } => name,
            ScrivenerArg::Token { name, value } => {
                assert!(is_kebab_case(value), "{}: token {value:?}", row.key);
                name
            }
            ScrivenerArg::Empty => panic!("{}: an empty argument", row.key),
        };
        assert!(is_snake_case(name), "{}: argument {name:?}", row.key);
        assert!(seen.insert(name.clone()), "{}: {name} twice", row.key);
    }
}

#[test]
fn every_key_is_listed_once_and_is_the_importers_own() {
    let keys = every_key();
    let mut sorted = keys.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), keys.len(), "a key listed twice");
    assert!(keys.iter().all(|k| k.starts_with("scrivener-")));
    // A document importer's key never collides with one of these: one list carries both.
    assert!(!keys.contains(&"comment-unanchored"));
}

#[test]
fn every_refusal_is_a_typed_row_about_the_project() {
    let keys = every_key();
    let refusals = every_refusal();
    let mut listed: Vec<&str> = refusals.iter().map(Refusal::key).collect();
    listed.sort_unstable();
    listed.dedup();
    assert_eq!(listed.len(), Refusal::KEYS.len(), "one of every refusal");
    for refusal in refusals {
        let row = row(super::refusal(&refusal));
        assert_eq!(row.key, refusal.key());
        assert!(keys.contains(&row.key.as_str()));
        assert_eq!(row.family, ScrivenerFindingFamily::Refusal);
        assert_eq!(row.group, ScrivenerReportGroup::Unfiled);
        assert_eq!(row.place, ScrivenerPlace::Project);
        assert!(row.path.is_empty() && row.uid.is_none() && row.item == -1);
        check_args(&row);
    }
    // The damage keeps what it carries.
    let damaged = row(super::refusal(&Refusal::DamagedManifest {
        manifest: s("a.scrivx"),
        damage: ManifestDamage::TooDeep {
            depth: 300,
            line: 4,
        },
        spare_copies: 2,
    }));
    assert_eq!(
        damaged.args,
        vec![
            ScrivenerArg::Text {
                name: s("manifest"),
                value: s("a.scrivx")
            },
            ScrivenerArg::Token {
                name: s("damage"),
                value: s("too-deep")
            },
            ScrivenerArg::Count {
                name: s("depth"),
                value: 300
            },
            ScrivenerArg::Count {
                name: s("line"),
                value: 4
            },
            ScrivenerArg::Count {
                name: s("spare_copies"),
                value: 2
            },
        ]
    );
}

#[test]
fn every_notice_is_a_typed_row_named_where_the_writer_knew_it() {
    let names = names();
    let (placements, rows) = written();
    let keys = every_key();
    let notices = every_notice();
    let mut listed: Vec<&str> = notices.iter().map(Notice::key).collect();
    listed.sort_unstable();
    listed.dedup();
    assert_eq!(listed.len(), Notice::KEYS.len(), "one of every notice");
    for stage in [
        Locator::before_import(&names),
        Locator::after_import(&names, &placements, &rows),
    ] {
        for notice in every_notice() {
            let row = row(super::notice(&notice, &stage));
            assert_eq!(row.key, notice.key());
            assert!(keys.contains(&row.key.as_str()));
            assert_eq!(row.family, ScrivenerFindingFamily::Source);
            assert_eq!(row.group, ScrivenerReportGroup::Unfiled);
            check_args(&row);
        }
    }
    let after = Locator::after_import(&names, &placements, &rows);
    // Missing notes of the scene: its path, and the row that holds its notes.
    let missing = row(super::notice(
        &Notice::ContentMissing {
            item: ItemKey::Uuid(Uuid::from_u128(3)),
            role: TextRole::Notes,
            salvaged: true,
        },
        &after,
    ));
    assert_eq!(missing.place, ScrivenerPlace::Notes);
    assert_eq!(missing.item, 2, "the binder row, for the report's order");
    assert_eq!(missing.path, vec![s("Draft"), s("Chapter 1"), s("Arrival")]);
    assert_eq!(missing.uid, Some(uid(13)));
    // The duplicate is the item that carries the key without owning it.
    let duplicate = row(super::notice(
        &Notice::DuplicateItemKey {
            key: ItemKey::Uuid(Uuid::from_u128(3)),
        },
        &after,
    ));
    assert_eq!(duplicate.place, ScrivenerPlace::Item);
    assert_eq!(duplicate.path, vec![s("Ideas")]);
    assert_eq!(duplicate.uid, Some(uid(20)));
    // A key the binder does not list is kept as an argument, never dropped.
    let unknown = row(super::notice(
        &Notice::MissingMedia {
            item: ItemKey::Number(77),
        },
        &after,
    ));
    assert_eq!(unknown.place, ScrivenerPlace::Project);
    assert!(unknown.args.contains(&ScrivenerArg::Text {
        name: s("item_key"),
        value: s("77")
    }));
}

#[test]
fn every_structure_notice_keeps_its_group_and_its_item() {
    let names = names();
    let (placements, rows) = written();
    let after = Locator::after_import(&names, &placements, &rows);
    let keys = every_key();
    assert_eq!(every_structure_notice().len(), StructureNotice::KEYS.len());
    for notice in every_structure_notice() {
        let row = row(super::structure_notice(&notice, &after));
        assert_eq!(row.key, notice.key());
        assert!(keys.contains(&row.key.as_str()));
        assert_eq!(row.family, ScrivenerFindingFamily::Structure);
        assert_eq!(row.group, findings::group(notice.group()));
        match notice.item() {
            Some(item) => {
                assert_eq!(row.place, ScrivenerPlace::Item);
                assert_eq!(row.item, i64::from(item));
                assert_eq!(row.path, names.items[item as usize]);
            }
            None => {
                assert_eq!(row.place, ScrivenerPlace::Project);
                assert_eq!(row.item, -1);
            }
        }
        check_args(&row);
    }
    // The chapter that adopts a scene is an item argument, with its row.
    let adopted = row(super::structure_notice(
        &StructureNotice::AdoptedByChapter {
            item: 2,
            chapter: 1,
        },
        &after,
    ));
    assert_eq!(adopted.uid, Some(uid(12)));
    assert_eq!(
        adopted.args,
        vec![ScrivenerArg::Item {
            name: s("chapter"),
            binder_path: vec![s("Draft"), s("Chapter 1")],
            item_uid: Some(uid(11)),
        }]
    );
}

#[test]
fn every_metadata_notice_is_a_row_about_the_project() {
    let keys = every_key();
    assert_eq!(every_metadata_notice().len(), MetadataNotice::KEYS.len());
    for notice in every_metadata_notice() {
        let row = row(super::metadata_notice(&notice));
        assert_eq!(row.key, notice.key());
        assert!(keys.contains(&row.key.as_str()));
        assert_eq!(row.family, ScrivenerFindingFamily::Metadata);
        assert_eq!(row.group, findings::group(notice.group()));
        assert_eq!(row.place, ScrivenerPlace::Project);
        check_args(&row);
    }
}

/// A day the converter gives as a timestamp reaches the report as the writer's calendar
/// day; a closed choice as a token.
#[test]
fn a_deadline_reaches_the_report_as_a_day_and_a_reason() {
    use scrivener_import::metadata::DeadlineReason;
    let planned = row(super::metadata_notice(&MetadataNotice::PacePlanned {
        day: 4_099_766_400,
        day_off: true,
    }));
    assert_eq!(
        planned.args,
        vec![
            ScrivenerArg::Day {
                name: s("day"),
                value: s("2099-12-01")
            },
            ScrivenerArg::Flag {
                name: s("day_off"),
                value: true
            },
        ]
    );
    let passed = row(super::metadata_notice(
        &MetadataNotice::DeadlineNotPlanned {
            reason: DeadlineReason::Passed,
            day: None,
        },
    ));
    assert_eq!(
        passed.args,
        vec![ScrivenerArg::Token {
            name: s("reason"),
            value: s("passed")
        }]
    );
}
#[test]
fn a_prose_notice_names_the_writing_it_is_about_and_the_row_holding_it() {
    let names = names();
    let (placements, rows) = written();
    let after = Locator::after_import(&names, &placements, &rows);
    let keys = every_key();
    let expected = [
        (ScrivenerPlace::Text, Some(uid(12))),
        (ScrivenerPlace::Notes, Some(uid(13))),
        (ScrivenerPlace::Synopsis, Some(uid(12))),
        (ScrivenerPlace::Media, Some(uid(20))),
        (ScrivenerPlace::CardImage, Some(uid(12))),
        (ScrivenerPlace::ProjectNote, Some(uid(21))),
        (ScrivenerPlace::Recovered, Some(uid(22))),
    ];
    let notices = some_prose_notices();
    for (notice, (place, row_uid)) in notices.iter().zip(expected) {
        let row = row(super::prose_notice(notice, &after));
        assert_eq!(row.place, place, "{notice:?}");
        assert_eq!(row.uid, row_uid, "{notice:?}");
        check_args(&row);
    }
    for notice in &notices {
        let row = row(super::prose_notice(notice, &after));
        assert_eq!(row.group, findings::group(notice.group()));
        match notice {
            ProseNotice::Document { .. } => {
                assert_eq!(row.family, ScrivenerFindingFamily::Document);
                assert_eq!(row.key, "comment-unanchored");
            }
            _ => {
                assert_eq!(row.family, ScrivenerFindingFamily::Prose);
                assert!(keys.contains(&row.key.as_str()));
            }
        }
    }
    let note = row(super::prose_notice(&notices[5], &after));
    assert_eq!(note.path, vec![s("Plot")]);
    assert_eq!(note.item, -1, "a project note is no binder item");
    let recovered = row(super::prose_notice(&notices[6], &after));
    assert_eq!(recovered.path, vec![s("630")]);
    assert_eq!(recovered.key, "scrivener-prose-compile-raw-style-printed");
}

#[test]
fn nothing_is_resolved_to_a_row_before_the_import() {
    let names = names();
    let before = Locator::before_import(&names);
    for notice in some_prose_notices() {
        assert!(row(super::prose_notice(&notice, &before)).uid.is_none());
    }
}

#[test]
fn the_choices_cross_both_ways() {
    let list = vec![
        ScrivenerChoice::Empty,
        ScrivenerChoice::Containers {
            depth: 1,
            rung: ScrivenerContainerRung::Plain,
        },
        ScrivenerChoice::Texts {
            depth: 2,
            kind: ScrivenerTextKind::Chapters,
        },
        ScrivenerChoice::Root {
            item: 9,
            destination: ScrivenerDestination::LeaveOut,
        },
        ScrivenerChoice::Books {
            choice: ScrivenerBooksChoice::Several,
        },
        ScrivenerChoice::Split {
            item: 4,
            split: false,
        },
        ScrivenerChoice::Trash {
            choice: ScrivenerTrashChoice::LeaveOut,
        },
    ];
    let choices = choices_from_dto(&list).unwrap_or_default();
    assert_eq!(choices.containers.get(&1), Some(&ContainerRung::Plain));
    assert_eq!(choices.texts.get(&2), Some(&TextKind::Chapters));
    assert_eq!(choices.roots.get(&9), Some(&Destination::LeaveOut));
    assert_eq!(
        choices.books,
        Some(scrivener_import::structure::BooksChoice::Several)
    );
    assert_eq!(choices.splits.get(&4), Some(&false));
    assert_eq!(choices.trash, TrashChoice::LeaveOut);
    // And back: what the plan reports is what the wizard sends again.
    assert_eq!(
        choices_from_dto(&plan::choices_to_dto(&choices)),
        Ok(choices)
    );
    // A later choice for one depth replaces an earlier one, as the review's own does.
    let twice = choices_from_dto(&[
        ScrivenerChoice::Containers {
            depth: 1,
            rung: ScrivenerContainerRung::Part,
        },
        ScrivenerChoice::Containers {
            depth: 1,
            rung: ScrivenerContainerRung::Chapter,
        },
    ]);
    assert_eq!(
        twice.map(|c| c.containers.get(&1).copied()),
        Ok(Some(ContainerRung::Chapter))
    );
}

#[test]
fn a_choice_out_of_every_range_is_refused_by_name() {
    assert_eq!(
        choices_from_dto(&[ScrivenerChoice::Containers {
            depth: -1,
            rung: ScrivenerContainerRung::Part,
        }]),
        Err(ChoiceOutOfRange::Depth(-1))
    );
    assert_eq!(
        choices_from_dto(&[ScrivenerChoice::Texts {
            depth: 70_000,
            kind: ScrivenerTextKind::Scenes,
        }]),
        Err(ChoiceOutOfRange::Depth(70_000))
    );
    assert_eq!(
        choices_from_dto(&[ScrivenerChoice::Split {
            item: -2,
            split: true,
        }]),
        Err(ChoiceOutOfRange::Item(-2))
    );
}

#[test]
fn every_word_reaches_the_converter_as_given() {
    let slot = TITLE_SLOT;
    let dto = crate::dtos::ImportScrivenerProjectDto {
        manuscript_binder_name: s("Manuscrit"),
        notes_binder_name: s("Notes et recherches"),
        notes_of_pattern: format!("Notes de {slot}"),
        project_notes_name: s("Notes du projet"),
        recovered_name: s("Retrouvé dans Scrivener"),
        untitled_template_name: s("Modèle sans titre"),
        research_folder_pattern: format!("{slot} documentation"),
        list_separator: s(" et "),
        label_details_pattern: format!("Champ Scrivener : {slot}"),
        status_details_pattern: format!("Statut Scrivener : {slot}"),
        unnamed_label_pattern: format!("Étiquette {slot}"),
        unnamed_status_pattern: format!("Statut {slot}"),
        label_tag_pattern: format!("{slot} (étiquette)"),
        untitled_tag_name: s("Sans titre"),
        default_statuses: vec![
            ScrivenerDefaultRung::Empty,
            ScrivenerDefaultRung::Rung {
                name: s("Prévu"),
                category: common::entities::StatusCategory::Planned,
            },
        ],
        ..crate::dtos::ImportScrivenerProjectDto::default()
    };
    let words = words(&dto);
    assert_eq!(words.manuscript_binder, "Manuscrit");
    assert_eq!(words.notes_binder, "Notes et recherches");
    assert_eq!(words.notes_of, format!("Notes de {slot}"));
    assert_eq!(words.project_notes, "Notes du projet");
    assert_eq!(words.recovered, "Retrouvé dans Scrivener");
    assert_eq!(words.untitled_template, "Modèle sans titre");
    assert_eq!(words.research_folder, format!("{slot} documentation"));
    assert_eq!(words.list_separator, " et ");
    assert_eq!(words.label_details, format!("Champ Scrivener : {slot}"));
    assert_eq!(words.status_details, format!("Statut Scrivener : {slot}"));
    assert_eq!(words.unnamed_label, format!("Étiquette {slot}"));
    assert_eq!(words.unnamed_status, format!("Statut {slot}"));
    assert_eq!(words.label_tag, format!("{slot} (étiquette)"));
    assert_eq!(words.untitled_tag, "Sans titre");
    assert_eq!(
        words.default_statuses,
        vec![scrivener_import::DefaultRung {
            name: s("Prévu"),
            category: common::entities::StatusCategory::Planned,
        }]
    );
}

#[test]
fn an_answer_left_empty_is_no_answer() {
    let dto = crate::dtos::AnalyzeScrivenerProjectDto {
        chosen_manifest: s("Novel.scrivx"),
        upgraded_sibling: ScrivenerSiblingChoice::Backup,
        accept_no_text: true,
        ..crate::dtos::AnalyzeScrivenerProjectDto::default()
    };
    let answers = analysis_answers(&dto);
    assert_eq!(answers.project, None);
    assert_eq!(answers.manifest.as_deref(), Some("Novel.scrivx"));
    assert_eq!(
        answers.upgraded_sibling,
        Some(scrivener_import::SiblingChoice::Backup)
    );
    assert!(answers.accept_no_text && !answers.accept_unmerged_mobile_edits);
    assert_eq!(
        analysis_answers(&crate::dtos::AnalyzeScrivenerProjectDto::default()),
        scrivener_import::Answers::default()
    );
}

/// The computers a project names are shown on the card and withheld from every `{:?}`.
#[test]
fn the_writers_computers_never_reach_a_log() {
    let dto = crate::dtos::ScrivenerAnalysisDto {
        computers: ScrivenerComputers::Named {
            device: Some(s("Tavs-MacBook-Air-4")),
            lock_host: Some(s("studio.local")),
        },
        ..crate::dtos::ScrivenerAnalysisDto::default()
    };
    let logged = format!("{dto:?}");
    assert!(!logged.contains("Tavs-MacBook"), "{logged}");
    assert!(!logged.contains("studio.local"), "{logged}");
    assert!(logged.contains("<withheld>"));
}

#[test]
fn every_question_has_a_listed_key_and_a_tier() {
    use scrivener_import::Question;
    use scrivener_import::outcome::{
        ManifestCandidate, ManifestEvidence, MobileEdits, ProjectCandidate,
    };
    let questions = [
        Question::SeveralProjects {
            candidates: vec![ProjectCandidate { path: s("a") }],
        },
        Question::SeveralManifests {
            candidates: vec![ManifestCandidate {
                name: s("a.scrivx"),
                creator: None,
                version: Some(s("2.0")),
                modified: Some(1_700_000_000),
                items: Some(40),
                matches_autosave: true,
                identical_to_preselected: false,
            }],
            preselected: Some(s("a.scrivx")),
            evidence: vec![ManifestEvidence::MatchesAutosave],
        },
        Question::EnclosingProject {
            project: s("Novel.scriv"),
            levels_up: 2,
        },
        Question::UpgradedSibling {
            backup: s("Novel Backup.scriv"),
            sibling: s("Novel.scriv"),
            sibling_generation: Some(Generation::Format20),
            preselect_sibling: true,
        },
        Question::UnmergedMobileEdits {
            source: MobileEdits::MobileFolder,
        },
        Question::NoTextInCopy { text_items: 30 },
    ];
    let keys = every_key();
    for asked in &questions {
        let dto = question(asked);
        assert_eq!(dto.key(), asked.key());
        assert!(keys.contains(&dto.key()));
        let expected = match asked.tier() {
            scrivener_import::Tier::Choose => ScrivenerQuestionTier::Choose,
            scrivener_import::Tier::Accept => ScrivenerQuestionTier::Accept,
        };
        assert_eq!(dto.tier(), expected);
    }
}

#[test]
fn review_data_that_is_not_an_analysis_is_refused() {
    assert!(matches!(
        ScrivenerReview::open("not json"),
        Err(ScrivenerReviewError::Unreadable)
    ));
    assert!(matches!(
        ScrivenerReview::open("{}"),
        Err(ScrivenerReviewError::Unreadable)
    ));
}

/// The prose findings the parts of a split text each make about it are one row per kind,
/// their counts added, in the first one's place; a finding that differs in anything but its
/// counts stays a row of its own, and so does every finding of another family, whose
/// number is not always a quantity.
#[test]
fn a_split_text_s_prose_findings_are_one_row_per_kind() {
    let names = names();
    let (placements, rows) = written();
    let after = Locator::after_import(&names, &placements, &rows);
    let at = Place::Text { item: 2 };
    let found = |notice: ProseNotice| super::prose_notice(&notice, &after);
    let findings = vec![
        found(ProseNotice::PicturesWithoutDescription { at, count: 2 }),
        found(ProseNotice::FormattingLost {
            at,
            what: Formatting::Colour,
            count: 4,
        }),
        found(ProseNotice::PicturesWithoutDescription { at, count: 3 }),
        found(ProseNotice::FormattingLost {
            at,
            what: Formatting::Highlight,
            count: 1,
        }),
        found(ProseNotice::LongDocument {
            at,
            paragraphs: 700,
        }),
        found(ProseNotice::LongDocument {
            at,
            paragraphs: 800,
        }),
        found(ProseNotice::Document {
            at,
            key: "list-nesting-flattened".to_string(),
            warning: true,
            count: 4,
            detail: s("16"),
        }),
        found(ProseNotice::Document {
            at,
            key: "list-nesting-flattened".to_string(),
            warning: true,
            count: 5,
            detail: s("16"),
        }),
        found(ProseNotice::PicturesWithoutDescription {
            at: Place::Notes { item: 2 },
            count: 1,
        }),
    ];
    let merged = one_per_kind(findings);
    let summary: Vec<(String, ScrivenerPlace, Vec<ScrivenerArg>)> = merged
        .into_iter()
        .map(|f| {
            let row = row(f);
            (row.key, row.place, row.args)
        })
        .collect();
    let count = |name: &str, value: i64| ScrivenerArg::Count {
        name: s(name),
        value,
    };
    let what = |value: &str| ScrivenerArg::Token {
        name: s("what"),
        value: s(value),
    };
    assert_eq!(
        summary,
        vec![
            (
                s("scrivener-prose-pictures-without-description"),
                ScrivenerPlace::Text,
                vec![count("count", 5)],
            ),
            (
                s("scrivener-prose-formatting-lost"),
                ScrivenerPlace::Text,
                vec![what("colour"), count("count", 4)],
            ),
            (
                s("scrivener-prose-formatting-lost"),
                ScrivenerPlace::Text,
                vec![what("highlight"), count("count", 1)],
            ),
            (
                s("scrivener-prose-long-document"),
                ScrivenerPlace::Text,
                vec![count("paragraphs", 1500)],
            ),
            (
                s("list-nesting-flattened"),
                ScrivenerPlace::Text,
                vec![
                    ScrivenerArg::Flag {
                        name: s("warning"),
                        value: true,
                    },
                    count("count", 4),
                    ScrivenerArg::Text {
                        name: s("detail"),
                        value: s("16"),
                    },
                ],
            ),
            (
                s("list-nesting-flattened"),
                ScrivenerPlace::Text,
                vec![
                    ScrivenerArg::Flag {
                        name: s("warning"),
                        value: true,
                    },
                    count("count", 5),
                    ScrivenerArg::Text {
                        name: s("detail"),
                        value: s("16"),
                    },
                ],
            ),
            (
                s("scrivener-prose-pictures-without-description"),
                ScrivenerPlace::Notes,
                vec![count("count", 1)],
            ),
        ]
    );
}
