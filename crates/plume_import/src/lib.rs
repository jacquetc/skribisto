// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Plume Creator (`.plume`) → newest-version `.skrib` importer.
//!
//! One-directional, version-neutral, and a **pure file→file transform**: it reads
//! any Plume project — any historical schema version, the modern single-file zip
//! **or** the pre-0.3 old-system directory — and writes a `.skrib` zip at the
//! newest format version. The entity store is never touched; the UI loads the
//! result afterward via the existing `load_work`.
//!
//! Nothing it says is a sentence: progress is a phase code or the title of the
//! entry being mapped, what the writer should know is an [`ImportNotice`], and a
//! project it cannot import is refused with a [`Refusal`] (or one of the typed
//! refusals `skrib_format` raises on its behalf). The interface words all three in
//! the writer's language.

mod attend_parse;
pub mod diagnostics;
mod dict_parse;
mod info_parse;
mod map;
mod model;
mod source;
mod tree_parse;
mod version;

use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::{Context, Result};
use skrib_format::import_notice::ImportNotice;
use skrib_format::importing::{ImportCancelled, StagedOutput, fill_title};

pub use diagnostics::Refusal;
use model::{PlumeAttendance, PlumeInfo};
use source::PlumeSource;

/// What the import produced, for the UI's post-import summary.
pub struct ImportSummary {
    pub output_path: String,
    /// Total binder items written (both binders, including synthetic helpers).
    pub imported_items: u64,
    /// Meaningful (non-separator) Plume nodes that were trashed and skipped.
    pub skipped_trashed: u64,
    /// What the writer should know about the conversion, as data: the UI words
    /// each notice in the interface language.
    pub warnings: Vec<ImportNotice>,
}

/// Text the import writes into the project that Plume itself does not store: the
/// three story-bible groups an old (0.3 and earlier) project implies without naming,
/// the title of a project that names itself nowhere, and the titles the import makes
/// up for a sheet's or a folder's own notes and a folder's own text.
///
/// Seeded text, so not this crate's to word: the app resolves it in the interface
/// language, once, and it is stored as it is from then on. [`Default`] is the
/// English, for tests and for a caller with no catalogue. It crosses
/// `import_management`'s DTO as one list in [`SeededNames::ORDER`]; a short list
/// keeps the English for the rest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeededNames {
    pub characters_group: String,
    pub items_group: String,
    pub places_group: String,
    pub untitled_project: String,
    /// The title of a note holding a sheet's or a folder's own notes, with
    /// [`TITLE_SLOT`](skrib_format::importing::TITLE_SLOT) where its name goes.
    pub notes_of: String,
    /// The title of a scene holding a folder's own text, the same way.
    pub content_of: String,
}

impl SeededNames {
    /// The order the names travel in. Append, never reorder.
    pub const ORDER: [&'static str; 6] = [
        "characters_group",
        "items_group",
        "places_group",
        "untitled_project",
        "notes_of",
        "content_of",
    ];

    /// The names from a list in [`Self::ORDER`]; a missing or blank entry keeps its
    /// English default.
    pub fn from_list(list: &[String]) -> Self {
        let mut names = Self::default();
        let slots = [
            &mut names.characters_group,
            &mut names.items_group,
            &mut names.places_group,
            &mut names.untitled_project,
            &mut names.notes_of,
            &mut names.content_of,
        ];
        for (slot, given) in slots.into_iter().zip(list) {
            if !given.trim().is_empty() {
                *slot = given.clone();
            }
        }
        names
    }

    /// The title of the note holding `name`'s own notes.
    pub(crate) fn notes_title(&self, name: &str) -> String {
        fill_title(&self.notes_of, name)
    }

    /// The title of the scene holding `name`'s own text.
    pub(crate) fn content_title(&self, name: &str) -> String {
        fill_title(&self.content_of, name)
    }
}

impl Default for SeededNames {
    fn default() -> Self {
        Self {
            characters_group: "Characters".to_string(),
            items_group: "Items".to_string(),
            places_group: "Places".to_string(),
            untitled_project: "Imported Plume project".to_string(),
            notes_of: format!("{} (notes)", skrib_format::importing::TITLE_SLOT),
            content_of: format!("{} (text)", skrib_format::importing::TITLE_SLOT),
        }
    }
}

/// Convert the Plume project at `source_path` into a `.skrib` zip at `output_path`.
///
/// Progress-free convenience wrapper used by the fixture tests; production code
/// (the long-operation use case) calls [`import_with_progress`] directly.
#[cfg(test)]
pub fn import(
    source_path: &str,
    output_path: &str,
    overwrite: bool,
    manuscript_binder_name: &str,
    story_bible_binder_name: &str,
    status_names: &[String],
) -> Result<ImportSummary> {
    import_with_progress(
        source_path,
        output_path,
        overwrite,
        manuscript_binder_name,
        story_bible_binder_name,
        status_names,
        &SeededNames::default(),
        &|_, _| {},
        &AtomicBool::new(false),
    )
}

/// Convert the Plume project at `source_path` into a `.skrib` zip at
/// `output_path`, reporting progress and honouring cancellation.
///
/// `report(percent, label)` drives the UI's progress toast. `label` is either a
/// phase code (`import.opening`, `import.outline`, …), which the UI translates —
/// this crate has no catalogue — or the title of the entry being mapped, shown
/// as data. `cancel` is polled at every phase boundary and once per mapped node.
/// Cancellation leaves nothing on disk: the `.skrib` is staged beside its target
/// and moved into place only after the final cancel check (see
/// `skrib_format::importing::StagedOutput`), so an existing `output_path`
/// (overwrite) survives an aborted or failed import untouched, and no partial
/// file is left on any way out.
#[allow(clippy::too_many_arguments)]
pub fn import_with_progress(
    source_path: &str,
    output_path: &str,
    overwrite: bool,
    manuscript_binder_name: &str,
    story_bible_binder_name: &str,
    status_names: &[String],
    seeded: &SeededNames,
    report: &dyn Fn(f32, &str),
    cancel: &AtomicBool,
) -> Result<ImportSummary> {
    let output = StagedOutput::begin(output_path, overwrite)?;

    report(2.0, "import.opening");
    let src = PlumeSource::open(source_path)?;
    bail_if_cancelled(cancel)?;

    // Everything from the first parse to the finished bundle runs on the parser
    // stack. The mapper recurses once per level of the outline, about 6.7 KB of
    // stack a level in a debug build, so a tree nested to `MAX_XML_DEPTH` needs
    // most of a long operation's 2 MiB on its own; progress is relayed back here.
    let mapped = skrib_format::xml_depth::on_parser_stack_reporting(report, |report| {
        read_and_map(
            &src,
            manuscript_binder_name,
            story_bible_binder_name,
            status_names,
            seeded,
            report,
            cancel,
        )
    })
    .map_err(anyhow::Error::new)??;
    bail_if_cancelled(cancel)?;

    // Written beside the target, then moved over it: an existing target
    // (overwrite) is only replaced once the write fully succeeded and no late
    // cancel arrived. Every return before `commit` removes what was written.
    report(92.0, "import.writing");
    output.write(&mapped.bundle)?;
    bail_if_cancelled(cancel)?;
    output.commit()?;

    report(100.0, "import.done");
    Ok(ImportSummary {
        output_path: output_path.to_string(),
        imported_items: mapped.imported_items,
        skipped_trashed: mapped.skipped_trashed,
        warnings: mapped.warnings,
    })
}

/// Parse every member and map the project to a bundle: the part of an import that
/// is as deep as the project it reads, run on the parser stack by
/// [`import_with_progress`].
fn read_and_map(
    src: &PlumeSource,
    manuscript_binder_name: &str,
    story_bible_binder_name: &str,
    status_names: &[String],
    seeded: &SeededNames,
    report: &dyn Fn(f32, &str),
    cancel: &AtomicBool,
) -> Result<map::Mapped> {
    report(12.0, "import.outline");
    let tree = tree_parse::parse(&src.tree_xml).context("reading the Plume outline (tree)")?;
    let attendance = match &src.attendance_xml {
        Some(xml) => attend_parse::parse(xml, seeded)
            .context("reading the Plume story bible (attendance)")?,
        None => PlumeAttendance {
            spinbox_label: String::new(),
            groups: Vec::new(),
        },
    };
    // `info` is non-critical metadata (title + dates); a malformed one just falls
    // back to the tree's project name. One nested past the XML ceiling, or one
    // declaring an entity, is not malformed metadata but a file built to crash its
    // reader, and refuses the project exactly as it would in `tree` or
    // `attendance`: no Plume version writes anything within two hundred levels of
    // the ceiling, nor any entity declaration.
    let info = match &src.info_xml {
        Some(xml) => match info_parse::parse(xml) {
            Ok(info) => info,
            Err(e)
                if skrib_format::xml_depth::too_deep(&e).is_some()
                    || skrib_format::xml_depth::declares_entities(&e).is_some() =>
            {
                return Err(e.context("reading the Plume project information (info)"));
            }
            Err(_) => PlumeInfo::default(),
        },
        None => PlumeInfo::default(),
    };
    let dict_words = src
        .dict
        .as_deref()
        .map(dict_parse::parse)
        .unwrap_or_default();
    bail_if_cancelled(cancel)?;

    report(20.0, "import.converting");
    Ok(map::build_bundle(
        &tree,
        &attendance,
        &info,
        &dict_words,
        src,
        manuscript_binder_name,
        story_bible_binder_name,
        status_names,
        seeded,
        report,
        cancel,
    ))
}

/// Abort the import if the cancel token has been set, before any file is written.
fn bail_if_cancelled(cancel: &AtomicBool) -> Result<()> {
    if cancel.load(Ordering::Relaxed) {
        return Err(anyhow::Error::new(ImportCancelled));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
