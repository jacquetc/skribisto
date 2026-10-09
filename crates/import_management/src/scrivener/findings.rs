// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Every refusal and notice the Scrivener importer can give, as the typed row the interface
//! words: a key, where it happened, its report group, and its arguments by name.
//!
//! Each converter value becomes one [`ScrivenerFinding`], field for field: a count stays a
//! count, a value from the project stays that value, a closed choice becomes a stable
//! kebab-case token a Fluent selector can match, and an item the value names by binder row
//! or by key becomes its path in Scrivener's binder, with the new row's uid once the
//! project is written. Every match is exhaustive, so a new kind of notice fails to compile
//! here until it has its row. The one exception to one value, one row: what the parts of a
//! text split into several rows each found is one row per kind ([`one_per_kind`]).
//!
//! Argument names are the converter's own field names (`count`, `this_machine`), which are
//! what the interface passes to `tr!`.

use std::collections::HashMap;

use scrivener_import::metadata::{
    AuthorSource, DeadlineReason, DraftTargetReason, Field, TargetUnit, TitleSource,
};
use scrivener_import::model::{ItemKey, TextRole};
use scrivener_import::outcome::{
    BackupSign, DanglingKind, ManifestDamage, ReadProblem, TemplateDamage, UnsafeName,
};
use scrivener_import::prose::{Formatting, LeftOut, RangeReading};
use scrivener_import::structure::{
    Level, NoteSection, Origin, Placement, PlanError, ReportGroup, RowRef, StructureNotice,
};
use scrivener_import::{
    MetadataNotice, Notice, Place, ProseNotice, Question, Refusal, SourceNames, WrittenRow,
};
use uuid::Uuid;

use crate::dtos::{
    ScrivenerArg, ScrivenerFinding, ScrivenerFindingFamily, ScrivenerPlace, ScrivenerReportGroup,
};

/// Every key a Scrivener finding or question can carry that is the importer's own, for the
/// interface's translation tests: the refusals, the questions, the source notices, and the
/// structure, prose and metadata notices. A finding of the [`ScrivenerFindingFamily::Document`]
/// family carries one of the document importer's keys instead, which that importer's own
/// translation test covers.
pub fn every_key() -> Vec<&'static str> {
    [
        Refusal::KEYS,
        Question::KEYS,
        Notice::KEYS,
        StructureNotice::KEYS,
        ProseNotice::KEYS,
        MetadataNotice::KEYS,
    ]
    .concat()
}

/// Where a finding happened, resolved.
pub(crate) struct Located {
    place: ScrivenerPlace,
    /// The Scrivener binder row, -1 when it is not a binder item.
    item: i64,
    /// Where the writer knew it in Scrivener.
    path: Vec<String>,
    /// The row of the new project that holds it, once there is one.
    uid: Option<Uuid>,
}

/// What a finding can be about, resolved: where the writer knew it in Scrivener, and which
/// row of the new project holds it once there is one.
pub(crate) struct Locator<'a> {
    names: &'a SourceNames,
    placements: &'a [Placement],
    uids: HashMap<RowRef, Uuid>,
    project_notes: HashMap<u32, Uuid>,
    recovered: HashMap<u32, Uuid>,
}

impl<'a> Locator<'a> {
    /// Before anything is written: paths only.
    pub(crate) fn before_import(names: &'a SourceNames) -> Self {
        Locator {
            names,
            placements: &[],
            uids: HashMap::new(),
            project_notes: HashMap::new(),
            recovered: HashMap::new(),
        }
    }

    /// After the import: paths, and the uid of the row that holds each thing.
    pub(crate) fn after_import(
        names: &'a SourceNames,
        placements: &'a [Placement],
        rows: &[WrittenRow],
    ) -> Self {
        let mut locator = Locator::before_import(names);
        locator.placements = placements;
        for row in rows {
            locator.uids.insert(row.at, row.uid);
            match row.origin {
                Origin::ProjectNote { note } => {
                    locator.project_notes.insert(note, row.uid);
                }
                Origin::RecoveredText { text } => {
                    locator.recovered.insert(text, row.uid);
                }
                _ => {}
            }
        }
        locator
    }

    fn path(&self, item: u32) -> Vec<String> {
        self.names
            .items
            .get(item as usize)
            .cloned()
            .unwrap_or_default()
    }

    fn uid(&self, at: Option<RowRef>) -> Option<Uuid> {
        at.and_then(|at| self.uids.get(&at).copied())
    }

    fn placement(&self, item: u32) -> Option<&Placement> {
        self.placements.get(item as usize)
    }

    /// An item as a whole: the row that represents it, else the one holding its text.
    fn item(&self, item: u32) -> Located {
        let at = self
            .placement(item)
            .and_then(|p| p.item.or(p.text.as_ref().map(|(at, _)| *at)));
        Located {
            place: ScrivenerPlace::Item,
            item: i64::from(item),
            path: self.path(item),
            uid: self.uid(at),
        }
    }

    /// An item as a whole, as `place`: its picture or file, its index-card picture.
    fn item_as(&self, item: u32, place: ScrivenerPlace) -> Located {
        Located {
            place,
            ..self.item(item)
        }
    }

    /// One of an item's writings: the row holding it, else the item's own row.
    fn writing(&self, item: u32, role: TextRole) -> Located {
        let placement = self.placement(item);
        let own = placement.and_then(|p| p.item);
        let (place, at) = match role {
            TextRole::Text => (
                ScrivenerPlace::Text,
                placement.and_then(|p| p.text.as_ref().map(|(at, _)| *at)),
            ),
            TextRole::Notes => (ScrivenerPlace::Notes, placement.and_then(|p| p.notes)),
            TextRole::Synopsis => (ScrivenerPlace::Synopsis, placement.and_then(|p| p.synopsis)),
        };
        Located {
            place,
            item: i64::from(item),
            path: self.path(item),
            uid: self.uid(at.or(own)),
        }
    }

    /// Where a prose notice happened.
    fn place(&self, place: Place) -> Located {
        match place {
            Place::Text { item } => self.writing(item, TextRole::Text),
            Place::Notes { item } => self.writing(item, TextRole::Notes),
            Place::Synopsis { item } => self.writing(item, TextRole::Synopsis),
            Place::Media { item } => self.item_as(item, ScrivenerPlace::Media),
            Place::CardImage { item } => self.item_as(item, ScrivenerPlace::CardImage),
            Place::ProjectNote { note } => Located {
                place: ScrivenerPlace::ProjectNote,
                item: -1,
                path: vec![
                    self.names
                        .project_notes
                        .get(note as usize)
                        .cloned()
                        .unwrap_or_default(),
                ],
                uid: self.project_notes.get(&note).copied(),
            },
            Place::Recovered { text } => Located {
                place: ScrivenerPlace::Recovered,
                item: -1,
                path: self
                    .names
                    .recovered
                    .get(text as usize)
                    .cloned()
                    .unwrap_or_default(),
                uid: self.recovered.get(&text).copied(),
            },
        }
    }

    /// The item a key names: the one owning it when `owner`, else one carrying it without.
    fn keyed(&self, key: ItemKey, owner: bool) -> Option<u32> {
        self.names.item_by_key(key, owner)
    }

    /// Another item, as an argument.
    fn item_arg(&self, name: &str, item: u32) -> ScrivenerArg {
        let located = self.item(item);
        ScrivenerArg::Item {
            name: name.to_string(),
            binder_path: located.path,
            item_uid: located.uid,
        }
    }
}

// ── arguments ────────────────────────────────────────────────────────────────────────────

fn count(name: &str, value: impl TryInto<i64>) -> ScrivenerArg {
    ScrivenerArg::Count {
        name: name.to_string(),
        value: value.try_into().unwrap_or(i64::MAX),
    }
}

fn text(name: &str, value: &str) -> ScrivenerArg {
    ScrivenerArg::Text {
        name: name.to_string(),
        value: value.to_string(),
    }
}

fn texts(name: &str, values: &[String]) -> ScrivenerArg {
    ScrivenerArg::Texts {
        name: name.to_string(),
        values: values.to_vec(),
    }
}

fn flag(name: &str, value: bool) -> ScrivenerArg {
    ScrivenerArg::Flag {
        name: name.to_string(),
        value,
    }
}

fn token(name: &str, value: &str) -> ScrivenerArg {
    ScrivenerArg::Token {
        name: name.to_string(),
        value: value.to_string(),
    }
}

fn day(name: &str, value: &str) -> ScrivenerArg {
    ScrivenerArg::Day {
        name: name.to_string(),
        value: value.to_string(),
    }
}

/// A finding about the project as a whole.
fn whole(
    key: &str,
    family: ScrivenerFindingFamily,
    group: ScrivenerReportGroup,
    args: Vec<ScrivenerArg>,
) -> ScrivenerFinding {
    ScrivenerFinding::Reported {
        key: key.to_string(),
        family,
        group,
        place: ScrivenerPlace::Project,
        item: -1,
        binder_path: Vec::new(),
        item_uid: None,
        args,
    }
}

/// A finding about one place.
fn at(
    key: &str,
    family: ScrivenerFindingFamily,
    group: ScrivenerReportGroup,
    located: Located,
    args: Vec<ScrivenerArg>,
) -> ScrivenerFinding {
    ScrivenerFinding::Reported {
        key: key.to_string(),
        family,
        group,
        place: located.place,
        item: located.item,
        binder_path: located.path,
        item_uid: located.uid,
        args,
    }
}

pub(crate) fn group(group: ReportGroup) -> ScrivenerReportGroup {
    match group {
        ReportGroup::CouldNotBeRead => ScrivenerReportGroup::CouldNotBeRead,
        ReportGroup::CheckThese => ScrivenerReportGroup::CheckThese,
        ReportGroup::BroughtAcrossDifferently => ScrivenerReportGroup::BroughtAcrossDifferently,
        ReportGroup::LeftBehind => ScrivenerReportGroup::LeftBehind,
    }
}

// ── tokens ───────────────────────────────────────────────────────────────────────────────

fn backup_sign(sign: BackupSign) -> &'static str {
    match sign {
        BackupSign::BackupsFolder => "backups-folder",
        BackupSign::BakArchive => "bak-archive",
        BackupSign::DateStamped => "date-stamped",
    }
}

fn dangling(kind: DanglingKind) -> &'static str {
    match kind {
        DanglingKind::Keyword => "keyword",
        DanglingKind::Label => "label",
        DanglingKind::Status => "status",
        DanglingKind::SectionType => "section-type",
        DanglingKind::CustomField => "custom-field",
        DanglingKind::Item => "item",
    }
}

fn title_source(source: TitleSource) -> &'static str {
    match source {
        TitleSource::CompileSettings => "compile-settings",
        TitleSource::ProjectProperties => "project-properties",
        TitleSource::Template => "template",
        TitleSource::FolderName => "folder-name",
    }
}

fn author_source(source: AuthorSource) -> &'static str {
    match source {
        AuthorSource::CompileSettings => "compile-settings",
        AuthorSource::ProjectProperties => "project-properties",
    }
}

fn level(level: Level) -> &'static str {
    match level {
        Level::Book => "book",
        Level::Part => "part",
        Level::Chapter => "chapter",
        Level::Scene => "scene",
        Level::Paratext => "paratext",
        Level::Note => "note",
    }
}

fn field(field: Field) -> &'static str {
    match field {
        Field::Label => "label",
        Field::Status => "status",
        Field::Keyword => "keyword",
        Field::CustomField => "custom-field",
        Field::Bookmark => "bookmark",
        Field::Target => "target",
    }
}

fn target_unit(unit: TargetUnit) -> &'static str {
    match unit {
        TargetUnit::Words => "words",
        TargetUnit::Characters => "characters",
        TargetUnit::Pages => "pages",
    }
}

fn draft_target_reason(reason: DraftTargetReason) -> &'static str {
    match reason {
        DraftTargetReason::Pages => "pages",
        DraftTargetReason::NoSingleBook => "no-single-book",
    }
}

fn deadline_reason(reason: DeadlineReason) -> &'static str {
    match reason {
        DeadlineReason::Passed => "passed",
        DeadlineReason::NoWritingDayLeft => "no-writing-day-left",
        DeadlineReason::NoWordGoal => "no-word-goal",
        DeadlineReason::CharacterGoal => "character-goal",
        DeadlineReason::NoSingleBook => "no-single-book",
        DeadlineReason::Unreadable => "unreadable",
        DeadlineReason::Template => "template",
    }
}

fn note_section(section: NoteSection) -> &'static str {
    section.token()
}

/// A day the converter gives as the Unix timestamp of its midnight, as the report names
/// days: `YYYY-MM-DD`.
fn unix_day(name: &str, seconds: i64) -> ScrivenerArg {
    let value = chrono::DateTime::from_timestamp(seconds, 0)
        .map(|at| at.format("%Y-%m-%d").to_string())
        .unwrap_or_default();
    day(name, &value)
}

fn range_reading(reading: RangeReading) -> &'static str {
    match reading {
        RangeReading::MarkersRemoved => "markers-removed",
        RangeReading::MarkersCounted => "markers-counted",
        RangeReading::PicturesNotCounted => "pictures-not-counted",
    }
}

fn formatting(what: Formatting) -> &'static str {
    match what {
        Formatting::Colour => "colour",
        Formatting::Highlight => "highlight",
        Formatting::SmallCaps => "small-caps",
        Formatting::AllCaps => "all-caps",
        Formatting::Hidden => "hidden",
    }
}

fn left_out(what: LeftOut) -> &'static str {
    match what {
        LeftOut::AfterDocument => "after-document",
        LeftOut::HeaderOrFooter => "header-or-footer",
        LeftOut::ListLabelPicture => "list-label-picture",
    }
}

/// Why a file of the project could not be read: the token, and what it carries.
fn read_problem(problem: &ReadProblem) -> Vec<ScrivenerArg> {
    let problem_token = |value: &str| token("problem", value);
    match problem {
        ReadProblem::Link => vec![problem_token("link")],
        ReadProblem::CloudPlaceholder => vec![problem_token("cloud-placeholder")],
        ReadProblem::TooLarge => vec![problem_token("too-large")],
        ReadProblem::NotAFile => vec![problem_token("not-a-file")],
        ReadProblem::NotText => vec![problem_token("not-text")],
        ReadProblem::TooDeep { depth, line } => vec![
            problem_token("too-deep"),
            count("depth", *depth),
            count("line", *line),
        ],
        ReadProblem::DeclaresEntities => vec![problem_token("declares-entities")],
        ReadProblem::Malformed { reason } => {
            vec![problem_token("malformed"), text("reason", reason)]
        }
        ReadProblem::System { reason } => vec![problem_token("system"), text("reason", reason)],
    }
}

fn manifest_damage(damage: &ManifestDamage) -> Vec<ScrivenerArg> {
    let damage_token = |value: &str| token("damage", value);
    match damage {
        ManifestDamage::NotText => vec![damage_token("not-text")],
        ManifestDamage::NotXml { reason } => {
            vec![damage_token("not-xml"), text("reason", reason)]
        }
        ManifestDamage::TooDeep { depth, line } => vec![
            damage_token("too-deep"),
            count("depth", *depth),
            count("line", *line),
        ],
        ManifestDamage::DeclaresEntities => vec![damage_token("declares-entities")],
        ManifestDamage::TooManyItems { limit } => {
            vec![damage_token("too-many-items"), count("limit", *limit)]
        }
        ManifestDamage::NotAManifest => vec![damage_token("not-a-manifest")],
        ManifestDamage::EmptyBinder => vec![damage_token("empty-binder")],
        ManifestDamage::NewProjectPlaceholder => vec![damage_token("new-project-placeholder")],
    }
}

fn unsafe_name(why: UnsafeName) -> &'static str {
    match why {
        UnsafeName::Empty => "empty",
        UnsafeName::Absolute => "absolute",
        UnsafeName::ParentFolder => "parent-folder",
        UnsafeName::ControlCharacter => "control-character",
    }
}

fn template_damage(damage: &TemplateDamage) -> Vec<ScrivenerArg> {
    let damage_token = |value: &str| token("damage", value);
    match damage {
        TemplateDamage::NotXml => vec![damage_token("not-xml")],
        TemplateDamage::NotATemplate => vec![damage_token("not-a-template")],
        TemplateDamage::NoProjectData => vec![damage_token("no-project-data")],
        TemplateDamage::UnknownEncoding { encoding } => {
            vec![damage_token("unknown-encoding"), text("encoding", encoding)]
        }
        TemplateDamage::Undecodable => vec![damage_token("undecodable")],
        TemplateDamage::NotAZip => vec![damage_token("not-a-zip")],
        TemplateDamage::TooLarge { limit_mib } => {
            vec![damage_token("too-large"), count("limit_mib", *limit_mib)]
        }
    }
}

/// A broken structure invariant: which kind, and the technical account for **Details**.
fn plan_error(error: &PlanError) -> Vec<ScrivenerArg> {
    let kind = match error {
        PlanError::Shape(_) => "shape",
        PlanError::FactsMismatch { .. } => "facts-mismatch",
        PlanError::Choice(_) => "choice",
        PlanError::Violation(_) => "violation",
    };
    vec![token("error", kind), text("detail", &error.to_string())]
}

// ── refusals ─────────────────────────────────────────────────────────────────────────────

/// Why the project cannot be imported, as the row the interface words: on the card, when
/// the analysis refuses, and in the failure, when the import does.
pub fn refusal(refusal: &Refusal) -> ScrivenerFinding {
    let args = match refusal {
        Refusal::Missing { file } => vec![text("file", file)],
        Refusal::Unreadable { file, reason } => vec![text("file", file), text("reason", reason)],
        Refusal::NotAScrivenerProject => Vec::new(),
        Refusal::ManifestWithoutFiles { manifest } => vec![text("manifest", manifest)],
        Refusal::SearchTooWide { limit } => vec![count("limit", *limit)],
        Refusal::ScrivenerOneForMac => Vec::new(),
        Refusal::IosStubWithoutMobileData => Vec::new(),
        Refusal::UnknownFormatVersion { version } => vec![text("version", version)],
        Refusal::NewerFormatStamp { stamp, newest } => {
            vec![count("stamp", *stamp), count("newest", *newest)]
        }
        Refusal::VersionStampUnreadable => Vec::new(),
        Refusal::DamagedManifest {
            manifest,
            damage,
            spare_copies,
        } => {
            let mut args = vec![text("manifest", manifest)];
            args.extend(manifest_damage(damage));
            args.push(count("spare_copies", *spare_copies));
            args
        }
        Refusal::ManifestTooLarge {
            manifest,
            limit_mib,
        } => vec![text("manifest", manifest), count("limit_mib", *limit_mib)],
        Refusal::NoItemIdentifiers => Vec::new(),
        Refusal::IncompleteCopy => Vec::new(),
        Refusal::ArchiveUnreadable { file, reason } => {
            vec![text("file", file), text("reason", reason)]
        }
        Refusal::ArchiveUnsafeName { file, member, why } => vec![
            text("file", file),
            text("member", member),
            token("why", unsafe_name(*why)),
        ],
        Refusal::ArchiveTooManyMembers { limit } => vec![count("limit", *limit)],
        Refusal::ArchiveMemberTooLarge { member, limit_mib } => {
            vec![text("member", member), count("limit_mib", *limit_mib)]
        }
        Refusal::ArchiveTooLarge { limit_mib } => vec![count("limit_mib", *limit_mib)],
        Refusal::ArchiveEncrypted { file } => vec![text("file", file)],
        Refusal::ArchiveUnsupportedCompression { member, method } => {
            vec![text("member", member), text("method", method)]
        }
        Refusal::DuplicateArchiveMembers { file, member } => {
            let mut args = vec![text("file", file)];
            if let Some(member) = member {
                args.push(text("member", member));
            }
            args
        }
        Refusal::TemplateUndecodable { file, damage } => {
            let mut args = vec![text("file", file)];
            args.extend(template_damage(damage));
            args
        }
        Refusal::CloudPlaceholders { count: n, examples } => {
            vec![count("count", *n), texts("examples", examples)]
        }
        Refusal::UnreadableMembers { count: n, examples } => {
            vec![count("count", *n), texts("examples", examples)]
        }
        Refusal::NeededMemberIsLink { member } => vec![text("member", member)],
        Refusal::ProjectChangedDuringRead { count: n } => vec![count("count", *n)],
        Refusal::TooManyMembers { limit } => vec![count("limit", *limit)],
        Refusal::TooLarge { limit_mib } => vec![count("limit_mib", *limit_mib)],
        Refusal::StructureUnmappable { error } => plan_error(error),
    };
    whole(
        refusal.key(),
        ScrivenerFindingFamily::Refusal,
        ScrivenerReportGroup::Unfiled,
        args,
    )
}

// ── what the analysis found ──────────────────────────────────────────────────────────────

/// What the analysis found that the writer should know. The report group is the
/// interface's to decide (D7-12): these are the project card's notices first.
pub(crate) fn notice(notice: &Notice, locator: &Locator<'_>) -> ScrivenerFinding {
    use ScrivenerFindingFamily::Source;
    use ScrivenerReportGroup::Unfiled;
    let key = notice.key();
    let project = |args: Vec<ScrivenerArg>| whole(key, Source, Unfiled, args);
    // A notice naming an item by key: where the item is, else the project as a whole with
    // the key as its argument, so nothing it said is lost.
    let keyed = |item_key: ItemKey,
                 owner: bool,
                 role: Option<TextRole>,
                 mut args: Vec<ScrivenerArg>| match locator.keyed(item_key, owner) {
        Some(item) => {
            let place = match role {
                Some(role) => locator.writing(item, role),
                None => locator.item(item),
            };
            at(key, Source, Unfiled, place, args)
        }
        None => {
            args.push(text("item_key", &item_key_text(item_key)));
            whole(key, Source, Unfiled, args)
        }
    };
    match notice {
        Notice::ProjectFoundBelowPick { path } => project(vec![text("path", path)]),
        Notice::SourceIsTemplate { title } => project(
            title
                .iter()
                .map(|title| text("title", title))
                .collect::<Vec<_>>(),
        ),
        Notice::SourceIsBackup { sign } => project(vec![token("sign", backup_sign(*sign))]),
        Notice::ImportingPreUpgradeCopy
        | Notice::ImportedFromIosStub
        | Notice::StaleMobileBinderIgnored
        | Notice::MixedPayloadTrees
        | Notice::ArchivedWhileOpen
        | Notice::LanguageNotStated => project(Vec::new()),
        Notice::IdenticalManifestCopies { count: n } => project(vec![count("count", *n)]),
        Notice::ManifestCopiesIgnored { chosen, copies } => {
            project(vec![text("chosen", chosen), texts("copies", copies)])
        }
        Notice::PinnedManifestLooksLikeACopy { name, original } => {
            project(vec![text("name", name), text("original", original)])
        }
        Notice::PinnedManifestHasSiblings { others } => project(vec![count("others", *others)]),
        Notice::LegacyV1FilesBeside { count: n } => project(vec![count("count", *n)]),
        Notice::IdSchemeMismatch { version } => project(vec![text("version", version)]),
        Notice::VersionStampMismatch { stamp, version } => {
            project(vec![count("stamp", *stamp), text("version", version)])
        }
        Notice::UnrecognisedWriter { creator } => project(
            creator
                .iter()
                .map(|creator| text("creator", creator))
                .collect::<Vec<_>>(),
        ),
        Notice::InvalidItemIdentifier { count: n }
        | Notice::ItemsWithoutIdentifier { count: n } => project(vec![count("count", *n)]),
        Notice::DuplicateItemKey { key: item_key } => keyed(*item_key, false, None, Vec::new()),
        Notice::LockPresent { this_machine } => project(vec![flag("this_machine", *this_machine)]),
        Notice::ContentMissing {
            item,
            role,
            salvaged,
        } => keyed(*item, true, Some(*role), vec![flag("salvaged", *salvaged)]),
        Notice::ContentTruncated { item, role } => keyed(*item, true, Some(*role), Vec::new()),
        Notice::MissingMedia { item } => match locator.keyed(*item, true) {
            Some(index) => at(
                key,
                Source,
                Unfiled,
                locator.item_as(index, ScrivenerPlace::Media),
                Vec::new(),
            ),
            None => project(vec![text("item_key", &item_key_text(*item))]),
        },
        Notice::SyncConflictCopies {
            count: n,
            original_missing,
        } => project(vec![
            count("count", *n),
            count("original_missing", *original_missing),
        ]),
        Notice::RecoveredUnreferencedText { count: n }
        | Notice::SyncDebrisIgnored { count: n }
        | Notice::CloudPlaceholdersSkipped { count: n } => project(vec![count("count", *n)]),
        Notice::UnrecognisedFiles { count: n, examples }
        | Notice::SymlinkNotFollowed { count: n, examples }
        | Notice::UnreadableFilesSkipped { count: n, examples } => {
            project(vec![count("count", *n), texts("examples", examples)])
        }
        Notice::NoTextFound { text_items } => project(vec![count("text_items", *text_items)]),
        Notice::MetadataUnreadable { member, problem } => {
            let mut args = vec![text("member", member)];
            args.extend(read_problem(problem));
            project(args)
        }
        Notice::DanglingReferences { kind, count: n } => {
            project(vec![token("kind", dangling(*kind)), count("count", *n)])
        }
        Notice::SnapshotsNotImported {
            snapshots,
            documents,
            of_deleted,
        } => project(vec![
            count("snapshots", *snapshots),
            count("documents", *documents),
            count("of_deleted", *of_deleted),
        ]),
        Notice::WritingHistoryNotImported {
            days,
            first_day,
            last_day,
            unreadable,
        } => {
            let mut args = vec![count("days", *days)];
            if let Some(first) = first_day {
                args.push(day("first_day", first));
            }
            if let Some(last) = last_day {
                args.push(day("last_day", last));
            }
            args.push(count("unreadable", *unreadable));
            project(args)
        }
        Notice::TitlePlaceholderRejected { source } | Notice::TitleFromProject { source } => {
            project(vec![token("source", title_source(*source))])
        }
        Notice::AuthorPlaceholderRejected { source } | Notice::AuthorFromProject { source } => {
            project(vec![token("source", author_source(*source))])
        }
        Notice::LanguageFromScript { tag } => project(vec![text("tag", tag)]),
    }
}

/// An item key as text, for the rare notice whose item the binder no longer lists.
fn item_key_text(key: ItemKey) -> String {
    match key {
        ItemKey::Uuid(uuid) => uuid.hyphenated().to_string(),
        ItemKey::Number(number) => number.to_string(),
    }
}

// ── the structure mapping ────────────────────────────────────────────────────────────────

/// What the structure mapping decided that the writer should know.
pub(crate) fn structure_notice(
    notice: &StructureNotice,
    locator: &Locator<'_>,
) -> ScrivenerFinding {
    use ScrivenerFindingFamily::Structure;
    let key = notice.key();
    let group = group(notice.group());
    let project = |args: Vec<ScrivenerArg>| whole(key, Structure, group.clone(), args);
    let item = |item: u32, args: Vec<ScrivenerArg>| {
        at(key, Structure, group.clone(), locator.item(item), args)
    };
    match notice {
        StructureNotice::ScriptModeDraft {
            script_documents,
            draft_documents,
        } => project(vec![
            count("script_documents", *script_documents),
            count("draft_documents", *draft_documents),
        ]),
        StructureNotice::OneDocumentManuscript {
            item: i,
            headings,
            split,
        } => item(
            *i,
            vec![count("headings", *headings), flag("split", *split)],
        ),
        StructureNotice::ClaimOverruled { item: i, claimed } => {
            item(*i, vec![token("claimed", level(*claimed))])
        }
        StructureNotice::AdoptedByChapter { item: i, chapter } => {
            item(*i, vec![locator.item_arg("chapter", *chapter)])
        }
        StructureNotice::MatterInsideBook { item: i, back } => item(*i, vec![flag("back", *back)]),
        StructureNotice::StoryBibleFolder { item: i, template } => {
            item(*i, vec![flag("template", *template)])
        }
        StructureNotice::LeftOut { item: i, items } => item(*i, vec![count("items", *items)]),
        StructureNotice::TitleRepeatsNumber { item: i }
        | StructureNotice::LooksLikeManuscript { item: i }
        | StructureNotice::GenericSheetTitle { item: i }
        | StructureNotice::PlaceholderTitle { item: i }
        | StructureNotice::UntitledItem { item: i }
        | StructureNotice::TitleCleaned { item: i }
        | StructureNotice::OwnTextMoved { item: i }
        | StructureNotice::TextWithChildren { item: i }
        | StructureNotice::EmptyFolderPlain { item: i }
        | StructureNotice::BookInsideBook { item: i }
        | StructureNotice::PrologueNotNumbered { item: i }
        | StructureNotice::DocumentNotes { item: i }
        | StructureNotice::DraftContentAsNote { item: i }
        | StructureNotice::TemplateNestingFlattened { item: i }
        | StructureNotice::TemplateKeptAsNote { item: i }
        | StructureNotice::NotCompiled { item: i }
        | StructureNotice::MediaInDraft { item: i }
        | StructureNotice::CardImageLeftBehind { item: i }
        | StructureNotice::DefaultTemplateNotLinked { item: i }
        | StructureNotice::PageBreakNotKept { item: i }
        | StructureNotice::CompileAsIsNotKept { item: i } => item(*i, Vec::new()),
        StructureNotice::ParatextNotCounted { documents, words } => {
            project(vec![count("documents", *documents), count("words", *words)])
        }
        StructureNotice::TemplatesCreated { count: n }
        | StructureNotice::RecoveredTexts { count: n }
        | StructureNotice::ProjectNotes { count: n } => project(vec![count("count", *n)]),
        StructureNotice::TrashBroughtAcross {
            items,
            with_writing,
        }
        | StructureNotice::TrashLeftOut {
            items,
            with_writing,
        } => project(vec![
            count("items", *items),
            count("with_writing", *with_writing),
        ]),
    }
}

// ── the prose ────────────────────────────────────────────────────────────────────────────

/// What converting the prose did. The document importer's own findings keep its key, in
/// the [`ScrivenerFindingFamily::Document`] family, so their sentences are the ones the
/// document import already has.
pub(crate) fn prose_notice(notice: &ProseNotice, locator: &Locator<'_>) -> ScrivenerFinding {
    let family = match notice {
        ProseNotice::Document { .. } => ScrivenerFindingFamily::Document,
        _ => ScrivenerFindingFamily::Prose,
    };
    let args = match notice {
        ProseNotice::Unreadable { .. }
        | ProseNotice::ReadAsPlainText { .. }
        | ProseNotice::CommentsUnreadable { .. }
        | ProseNotice::HeadingRepeatsTitle { .. }
        | ProseNotice::AnnotationOnlyDocumentKept { .. } => Vec::new(),
        ProseNotice::InlineNoteUnmatched { count: n, .. }
        | ProseNotice::StyleUnresolved { count: n, .. }
        | ProseNotice::StylesLostInExport { count: n, .. }
        | ProseNotice::CommentAnchorWithoutEntry { count: n, .. }
        | ProseNotice::CommentEntryWithoutAnchor { count: n, .. }
        | ProseNotice::FootnoteEntryWithoutAnchor { count: n, .. }
        | ProseNotice::RangeCommentApproximate { count: n, .. }
        | ProseNotice::DocumentLinkUnresolved { count: n, .. }
        | ProseNotice::OrnamentLinesKept { count: n, .. }
        | ProseNotice::PicturesWithoutDescription { count: n, .. }
        | ProseNotice::StruckTextCompiledAway { count: n, .. }
        | ProseNotice::InlineNotesMerged { count: n, .. }
        | ProseNotice::InlineNotesJoinedAcrossParagraphs { count: n, .. }
        | ProseNotice::BlankNotesDropped { count: n, .. }
        | ProseNotice::AnnotationsBecameComments { count: n, .. }
        | ProseNotice::PreserveFormattingPassages { count: n, .. }
        | ProseNotice::NestedNoteUnwrapped { count: n, .. }
        | ProseNotice::DocumentLinksDropped { count: n, .. }
        | ProseNotice::DocumentLinksKept { count: n, .. }
        | ProseNotice::OtherLinksKeptAsText { count: n, .. }
        | ProseNotice::SoftBreaksSplit { count: n, .. }
        | ProseNotice::TypedIndentsRemoved { count: n, .. }
        | ProseNotice::LeftIndentsNotKept { count: n, .. }
        | ProseNotice::BlankLinesRemoved { count: n, .. }
        | ProseNotice::VerseStanzaGapsLost { count: n, .. }
        | ProseNotice::BlankLinesBecameSceneBreaks { count: n, .. }
        | ProseNotice::OrnamentsBecameSceneBreaks { count: n, .. }
        | ProseNotice::TableFirstRowHeader { count: n, .. }
        | ProseNotice::TableCellsJoined { count: n, .. }
        | ProseNotice::JustifiedTextNotKept { count: n, .. }
        | ProseNotice::CommentColoursLost { count: n, .. }
        | ProseNotice::ControlCharactersRemoved { count: n, .. }
        | ProseNotice::UndecodableCharacters { count: n, .. }
        | ProseNotice::NoteOnHeadingLost { count: n, .. } => vec![count("count", *n)],
        ProseNotice::RangeCommentReading {
            reading, count: n, ..
        } => vec![
            token("reading", range_reading(*reading)),
            count("count", *n),
        ],
        ProseNotice::LinkedPictureNotRead { name, .. }
        | ProseNotice::MediaFileNotCarried { name, .. } => vec![text("name", name)],
        ProseNotice::MarkdownAsText { paragraphs, .. }
        | ProseNotice::CitationKeysAsText { paragraphs, .. }
        | ProseNotice::LongDocument { paragraphs, .. } => vec![count("paragraphs", *paragraphs)],
        ProseNotice::PlaceholdersInExportedText { kind, count: n, .. }
        | ProseNotice::PlaceholdersInOtherText { kind, count: n, .. } => {
            vec![text("placeholder", kind), count("count", *n)]
        }
        // The treatment is in the key: the two are two sentences.
        ProseNotice::CompiledAwayStylePrinted { name, count: n, .. }
        | ProseNotice::StyleNotCarried { name, count: n, .. } => {
            vec![text("name", name), count("count", *n)]
        }
        ProseNotice::FormattingLost { what, count: n, .. } => {
            vec![token("what", formatting(*what)), count("count", *n)]
        }
        ProseNotice::LeftOut { what, count: n, .. } => {
            vec![token("what", left_out(*what)), count("count", *n)]
        }
        ProseNotice::MediaFileCopied { path, .. } => vec![text("path", path)],
        ProseNotice::Document {
            warning,
            count: n,
            detail,
            ..
        } => vec![
            flag("warning", *warning),
            count("count", *n),
            text("detail", detail),
        ],
    };
    at(
        notice.key(),
        family,
        group(notice.group()),
        locator.place(notice.place()),
        args,
    )
}

/// One row per kind and place: a text the import split into several rows (a one-document
/// manuscript cut at its chapter headings) is converted part by part, and each part says
/// what it found about the same text. The converter's own prose findings count quantities
/// (characters, paragraphs, pictures), so the parts' rows become one, their counts added,
/// in the place the first one had. Every other family is kept row for row: a structure,
/// source or metadata finding is one per item already, and a document importer's finding's
/// number is not always a quantity (a nesting ceiling, a heading level).
pub(crate) fn one_per_kind(findings: Vec<ScrivenerFinding>) -> Vec<ScrivenerFinding> {
    let mut out: Vec<ScrivenerFinding> = Vec::with_capacity(findings.len());
    let mut first: HashMap<String, usize> = HashMap::new();
    for finding in findings {
        let ScrivenerFinding::Reported {
            family: ScrivenerFindingFamily::Prose,
            ..
        } = &finding
        else {
            out.push(finding);
            continue;
        };
        let identity = identity(&finding);
        match first.get(&identity).and_then(|at| out.get_mut(*at)) {
            Some(ScrivenerFinding::Reported { args, .. }) => add_counts(args, &finding),
            _ => {
                first.insert(identity, out.len());
                out.push(finding);
            }
        }
    }
    out
}

/// What makes two findings the same row: all of it but the counts.
fn identity(finding: &ScrivenerFinding) -> String {
    let mut without_counts = finding.clone();
    if let ScrivenerFinding::Reported { args, .. } = &mut without_counts {
        for arg in args {
            if let ScrivenerArg::Count { value, .. } = arg {
                *value = 0;
            }
        }
    }
    format!("{without_counts:?}")
}

/// `other`'s counts added to `args`, argument by argument: the two are the same kind, so
/// their arguments come in the same order under the same names.
fn add_counts(args: &mut [ScrivenerArg], other: &ScrivenerFinding) {
    let ScrivenerFinding::Reported { args: more, .. } = other else {
        return;
    };
    for (arg, more) in args.iter_mut().zip(more) {
        if let (
            ScrivenerArg::Count { name, value },
            ScrivenerArg::Count {
                name: other_name,
                value: other_value,
            },
        ) = (arg, more)
            && name == other_name
        {
            *value = value.saturating_add(*other_value);
        }
    }
}

// ── the labels, the status list and the dates ────────────────────────────────────────────

/// What mapping the metadata did. None of these is about one item.
pub(crate) fn metadata_notice(notice: &MetadataNotice) -> ScrivenerFinding {
    let args = match notice {
        MetadataNotice::StatusCategoriesGuessed { rungs } => vec![texts("rungs", rungs)],
        MetadataNotice::DatesUnreadable { items } => vec![count("items", *items)],
        MetadataNotice::LabelsBecameTags { tags, rows } => {
            vec![count("tags", *tags), count("rows", *rows)]
        }
        MetadataNotice::StatusesBecameLadder { rungs, rows } => {
            vec![count("rungs", *rungs), count("rows", *rows)]
        }
        MetadataNotice::LadderSeeded { rungs } => vec![count("rungs", *rungs)],
        MetadataNotice::FieldRenamed { field: f, title } => {
            vec![token("field", field(*f)), text("title", title)]
        }
        MetadataNotice::NoneValueCollapsed {
            field: f,
            name,
            items,
        } => vec![
            token("field", field(*f)),
            text("name", name),
            count("items", *items),
        ],
        MetadataNotice::UnnamedValue { field: f, id } => {
            vec![token("field", field(*f)), count("id", *id)]
        }
        MetadataNotice::TagNameDisambiguated { names } => vec![texts("names", names)],
        MetadataNotice::RungNamesDisambiguated { names } => vec![texts("names", names)],
        MetadataNotice::UnusedLabelsSkipped { names } => vec![texts("names", names)],
        MetadataNotice::ValuesWithoutRow { field: f, items } => {
            vec![token("field", field(*f)), count("items", *items)]
        }
        MetadataNotice::PovHint { values, matches } => {
            vec![texts("values", values), texts("matches", matches)]
        }
        MetadataNotice::CheckboxDefaultUnknown { field: f } => vec![text("field", f)],
        MetadataNotice::GoalsInOtherUnit { count: n } => vec![count("count", *n)],
        MetadataNotice::CountingDiffers => Vec::new(),
        MetadataNotice::WritingDaysNotRead { raw } => vec![text("raw", raw)],
        MetadataNotice::ScriptModeRows { rows } => vec![count("rows", *rows)],
        MetadataNotice::KeywordsBecameTags { tags, rows } => {
            vec![count("tags", *tags), count("rows", *rows)]
        }
        MetadataNotice::KeywordHierarchyFlattened { keywords } => {
            vec![count("keywords", *keywords)]
        }
        MetadataNotice::FieldsBecameTags { fields, tags, rows } => vec![
            count("fields", *fields),
            count("tags", *tags),
            count("rows", *rows),
        ],
        MetadataNotice::CollectionsBecameTags { tags, rows } => {
            vec![count("tags", *tags), count("rows", *rows)]
        }
        MetadataNotice::CollectionOrderLost { names } => vec![texts("names", names)],
        MetadataNotice::BookmarksBecameLinks { count: n } => vec![count("count", *n)],
        MetadataNotice::GoalsKept { rows } => vec![count("rows", *rows)],
        MetadataNotice::DraftTargetKept { value, unit } => {
            vec![count("value", *value), token("unit", target_unit(*unit))]
        }
        MetadataNotice::PacePlanned { day: d, day_off } => {
            vec![unix_day("day", *d), flag("day_off", *day_off)]
        }
        MetadataNotice::DictionaryWords { count: n } => vec![count("count", *n)],
        MetadataNotice::ListedInImportNote { section, count: n } => {
            vec![token("section", note_section(*section)), count("count", *n)]
        }
        MetadataNotice::TargetsOnNotes { count: n } => vec![count("count", *n)],
        MetadataNotice::BookmarkEndsWithoutRow { count: n } => vec![count("count", *n)],
        MetadataNotice::UnusedKeywordsSkipped { names } => vec![texts("names", names)],
        MetadataNotice::ListValueInvalid { field: f, count: n } => {
            vec![text("field", f), count("count", *n)]
        }
        MetadataNotice::UndefinedCustomField { ids, values } => {
            vec![texts("ids", ids), count("values", *values)]
        }
        MetadataNotice::CollectionMembersMissing { name, count: n } => {
            vec![text("name", name), count("count", *n)]
        }
        MetadataNotice::EmptyCollectionsSkipped { names } => vec![texts("names", names)],
        MetadataNotice::BookmarksUnresolved { count: n } => vec![count("count", *n)],
        MetadataNotice::TargetsInPages { count: n }
        | MetadataNotice::TargetsUnreadable { count: n }
        | MetadataNotice::TargetsOnSplitDocuments { count: n } => vec![count("count", *n)],
        MetadataNotice::DraftTargetNotKept {
            reason,
            value,
            unit,
        } => vec![
            token("reason", draft_target_reason(*reason)),
            count("value", *value),
            token("unit", target_unit(*unit)),
        ],
        MetadataNotice::DeadlineNotPlanned { reason, day: d } => {
            let mut args = vec![token("reason", deadline_reason(*reason))];
            if let Some(d) = d {
                args.push(unix_day("day", *d));
            }
            args
        }
        MetadataNotice::SessionTargetNotImported {
            value,
            unit,
            calculated,
        } => vec![
            count("value", *value),
            token("unit", target_unit(*unit)),
            flag("calculated", *calculated),
        ],
        MetadataNotice::IconsDropped { rows, names } => {
            vec![count("rows", *rows), texts("names", names)]
        }
    };
    whole(
        notice.key(),
        ScrivenerFindingFamily::Metadata,
        group(notice.group()),
        args,
    )
}
