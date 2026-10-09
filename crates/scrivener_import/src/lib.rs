// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Panic hygiene: this crate is at zero `unwrap()`/`expect()`/`panic!` outside tests, so the
// lint is switched on here to keep it that way. See the note in the workspace `Cargo.toml`
// for why this is per crate and not workspace-wide.
#![warn(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

//! Reading a Scrivener project: which one the writer picked, which generation of
//! Scrivener wrote it, what is safe to read, and one version-neutral model of it.
//!
//! The Scrivener importer: its source layer, its structure mapping, its prose, its
//! metadata and its converter. [`analyze()`] turns a path a writer picked into an
//! [`Analysis`]: a [`ReadProject`] (the [`model::Project`], what each document holds as
//! [`ContentFacts`], the [`structure::StructurePlan`] the review step shows, the title,
//! authors and language the project states as [`Proposals`] for the Save page, what the
//! writer should know as [`Notice`]s, what they must still accept), a [`Question`] the
//! writer must answer first, or a [`Refusal`] saying why the project cannot be
//! imported. [`convert()`] reads the project again with the writer's choices and
//! writes a newest-version `.skrib` of it: every row the plan lists, its prose read
//! through Scrivener's own dialect ([`prose`]), its footnotes, comments and pictures,
//! the story bible's tags, the labels as tags and the status list as the workflow
//! ladder ([`metadata`]), every row dated as Scrivener dated it, the note templates
//! and the Trash. Snapshots and the daily writing history stay in the Scrivener
//! project, counted in the report. Store-free, like the other whole-project
//! importers: the interface opens the result with the ordinary `load_work`.
//!
//! ## What it reads
//!
//! Every generation Scrivener has written since 2010, and every container writers
//! hold them in:
//!
//! * format 2.0 (Scrivener 3 on Mac and Windows), format 1.5 (Mac 2.7 and 2.8,
//!   Windows 1.9, the Linux beta 1.9) and format 1.0 (Mac 2.0 to 2.6, Windows 1.0
//!   to 1.8, the Linux beta up to 1.7), and projects created on an iPhone or iPad
//!   and never opened on a desktop;
//! * a project folder, the folder around one, a folder or a manifest inside one,
//!   a zip archive (Scrivener's backups, iOS exports, Finder's "Compress"), and a
//!   `.scrivtemplate` in either of its encodings.
//!
//! Scrivener 1 for Mac (2007 to 2010) is recognised and refused, by decision.
//!
//! ## How it reads
//!
//! Only ever reading: nothing in the source is created, changed or deleted, not
//! even Scrivener's lock file. Everything is found through one index of the
//! project, so a name out of a manifest can never reach a file outside
//! it; links are never followed; archives are bounded before and while they
//! inflate; XML is measured before it is parsed; nothing recurses on the input's
//! nesting; and a project that changes while it is read is read again, then
//! refused. Pictures and research files are located, never read, until the
//! conversion brings the pictures in; a research file Skribisto cannot open stays
//! in the Scrivener project, and its note names where. A picture a text links to on
//! the writer's disk is never read here either: only its file name is reported.
//!
//! ## Nothing it says is a sentence
//!
//! Refusals, questions and notices are values with stable keys; the interface
//! words them in the writer's language. Nothing personal (the host and user in a
//! lock file, the device a manifest names) reaches a notice; the [`Card`] may show
//! it before the import and nothing keeps it.
//!
//! ## Provenance
//!
//! Written for Skribisto from Scrivener projects observed directly (every
//! generation, harvested from public repositories and measured locally, never
//! redistributed) and from Literature & Latte's published manuals. No other
//! Scrivener reader's code was used or adapted: several published ones encode
//! format details real files contradict.
//!
//! ## Maintenance
//!
//! Scrivener's format will move again. The gate fails closed above the newest
//! known version ([`NEWEST_STAMP`]); raising it means adding a fixture of the new
//! shape to `scrivener_fixtures`, re-running the opt-in corpus test
//! (`tests/all/corpus.rs`, which says how to rebuild the corpus), and only then
//! moving the ceiling.
//!
//! The structure mapping recognises levels by closed word lists
//! (`structure/lexicon.rs`). A word added there changes which items claim a
//! level: add a unit test for it, then run the corpus test's structure half and
//! compare the per-project counts it prints with the previous run.

pub mod analyze;
mod completeness;
pub mod content;
pub mod convert;
mod dates;
mod disk;
mod index;
mod layout;
mod lock;
mod manifest;
pub mod metadata;
pub mod model;
mod names;
pub mod outcome;
mod pick;
pub mod prose;
pub mod structure;
mod template;
mod xml;

pub use analyze::{
    Analysis, Answers, Card, NEWEST_STAMP, Options, ReadProject, SiblingChoice, analyze,
};
pub use content::ContentFacts;
pub use convert::{
    ConvertError, DefaultRung, Request, Summary, TITLE_SLOT, Totals, Words, convert,
};
pub use lock::this_host;
pub use metadata::{MetadataNotice, Proposals};
pub use outcome::{Notice, Question, Refusal, Tier};
pub use prose::{CompileTreatment, LeftOut, Place, ProseChoices, ProseNotice};

use std::path::Path;

/// Whether `path` looks like a Scrivener project, from a listing or the first
/// bytes of a file and nothing more: a project folder, a manifest, or a
/// `.scrivtemplate`. For a caller that must tell a Scrivener project from a
/// Skribisto one before choosing how to open it.
pub fn looks_like_a_scrivener_project(path: &Path) -> bool {
    let disk = disk::RealDisk;
    let Ok(meta) = disk::Disk::metadata_following(&disk, path) else {
        return false;
    };
    // A picked link is judged by where it leads, as `analyze` reads it.
    let resolved;
    let path =
        if disk::Disk::metadata(&disk, path).is_ok_and(|m| m.kind == disk::EntryKind::Symlink) {
            match disk::Disk::resolve(&disk, path) {
                Ok(target) => {
                    resolved = target;
                    resolved.as_path()
                }
                Err(_) => return false,
            }
        } else {
            path
        };
    match meta.kind {
        disk::EntryKind::Dir => pick::list_disk(&disk, path)
            .is_ok_and(|listing| listing.strength() == pick::Strength::Strong),
        disk::EntryKind::File => disk::Disk::read_head(&disk, path, 4096).is_ok_and(|head| {
            matches!(
                pick::sniff(&head),
                pick::Sniffed::Manifest | pick::Sniffed::Template
            )
        }),
        _ => false,
    }
}
