// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What a Scrivener project says about itself, beyond its binder and its prose: its title,
//! its author and its language, its labels and its status list, and its dates.
//!
//! # Proposals, not facts (D4-15)
//!
//! [`propose`] reads the title, the authors and the language a project states into
//! [`Proposals`], each with where it comes from. The interface pre-fills the Save page with
//! them and the writer confirms or changes each; the converter writes the writer's final
//! answer ([`crate::Request`]). So a template's leftover author (`OrrenQ`, `Vorname
//! Nachname`) or a placeholder title (`<$template_projectName>`) never reaches a book
//! without being seen:
//!
//! * the **title** is the compile settings' `ProjectTitle` (format 2.0), else the format
//!   1.x `ProjectProperties` title, else a `.scrivtemplate`'s own title, else the project's
//!   folder name cleaned of what backups and sync services add to it. A title holding a
//!   `<$…>` placeholder, or empty, is refused and said.
//! * the **author** is each `Author` of the compile metadata (format 2.0; an author whose
//!   text is exactly the sibling `Surname, Forename` is written `Forename Surname`, as it
//!   prints), else `Forename` and `Surname`, else the format 1.x `FullName`, else its first
//!   and last names. Never the manifest's own `Author` attribute, which names the computer's
//!   user, nor a Scrivener 2 compile's e-book authors, which are template residue.
//! * the **language** is a compile language the writer changed, else the one the script of
//!   the Draft names, else nothing: the interface then proposes its own
//!   language, as New Work does, and says the project stated none.
//!
//! # Labels and the status list (D4-2 to D4-4)
//!
//! [`vocabularies`] maps them onto a planned project: labels to tags, the status list to
//! the workflow ladder. Everything it writes into the project is either the writer's own
//! words from Scrivener, stored literally, or words the interface resolved in the writer's
//! language ([`crate::Words`]). What it did is reported as [`MetadataNotice`]s.
//!
//! # Everything else a project holds (D4-6 to D4-21)
//!
//! [`extended()`] maps the rest onto the same plan, each kind where the design puts it:
//!
//! * **keywords** become flat tags on the rows that wear them, their place in Scrivener's
//!   keyword list in the details; **list** and **checkbox** fields become tags (one per
//!   option worn, `<field>: <option>`; one per checkbox ticked); **collections** of
//!   documents become tags on their documents. Never story-bible tags, never on a row whose
//!   item no document wears, and never a name another tag holds (D4-9).
//! * **text and date fields** (and every format 1.x field) are listed: a note-like row's
//!   own values follow its own text, every other document's go to the "From Scrivener"
//!   folder.
//! * **bookmarks and references** become links only where Skribisto shows one (a note to a
//!   note, a scene to a story-bible entry: X-35); the others are listed under their
//!   document, with a 1.x reference's description.
//! * **document targets** become goals on rows whose writing a goal measures, the **draft
//!   target** the book's goal, and the project counts in its draft target's unit (else most
//!   goals', else its language's); targets on notes are listed (X-30).
//! * **a deadline** still ahead with a word target becomes the book's pace plan, every day a
//!   writing day, the deadline itself kept free when Scrivener's calculator kept it.
//! * **auto-complete** words become the project's dictionary; phrases, compile replacements,
//!   saved searches, project bookmarks, the session target and what else the project
//!   states with no field to hold it are listed.
//!
//! What has nowhere to go and is not listed (icons, script mode, a template's metadata,
//! values naming nothing) is said. The "From Scrivener" folder holds one note per kind that
//! lists something ([`import_note_sections`]); the structure plan places it, so the review
//! shows what the import writes. Every word the import adds is the interface's
//! ([`MetadataWords`]).

mod collect;
pub(crate) mod compile;
mod extended;
pub(crate) mod language;
mod listing;
mod notice;
mod vocabulary;
mod words;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::content::{ContentFacts, Letters};
use crate::model::{CompileFacts, CompileFormat, Container, ItemKind, Project};
use crate::structure::clean_title;

pub use extended::{Extended, ExtendedTag, PlannedPace, extended, foreseen};
#[doc(hidden)]
pub use notice::every_example;
pub use notice::{DeadlineReason, DraftTargetReason, Field, MetadataNotice, TargetUnit};
pub use vocabulary::{LabelTag, Rung, Vocabularies, vocabularies};
pub(crate) use vocabulary::{PALETTE, folded};
pub use words::{MetadataWords, VALUE_SLOT};

/// The notes the "From Scrivener" folder holds for `plan`: those listing something (D4-20).
/// What the structure plan places, and what the converter writes, are decided by the same
/// function over the same rows.
pub fn import_note_sections(
    project: &Project,
    plan: &crate::structure::StructurePlan,
) -> Vec<crate::structure::NoteSection> {
    collect::collect(project, plan).listing.sections()
}

/// What a project states about itself, for the Save page to pre-fill.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proposals {
    pub title: TitleProposal,
    /// `None` when the project names no author.
    pub author: Option<AuthorProposal>,
    /// `None` when the project states no language: the interface proposes its own.
    pub language: Option<LanguageProposal>,
}

/// The project's title, and where it was read.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TitleProposal {
    /// Cleaned as titles are (D3.18). Empty only when even the folder has no name.
    pub title: String,
    pub source: TitleSource,
}

/// Where a title was read.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitleSource {
    /// The compile settings' metadata (format 2.0).
    CompileSettings,
    /// The project properties (format 1.x).
    ProjectProperties,
    /// A `.scrivtemplate`'s own title.
    Template,
    /// The project folder's name, or the archive's, cleaned.
    #[default]
    FolderName,
}

/// The project's authors, and where they were read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorProposal {
    /// Each author's name as it prints, in order. The interface joins several with its
    /// language's list separator.
    pub names: Vec<String>,
    pub source: AuthorSource,
}

/// Where the authors were read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorSource {
    /// The compile settings' metadata (format 2.0).
    CompileSettings,
    /// The project properties (format 1.x).
    ProjectProperties,
}

/// The manuscript's language, and what it was read from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageProposal {
    /// A language tag (`fr`, `pt-BR`).
    pub tag: String,
    pub source: LanguageSource,
}

/// What a language was read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LanguageSource {
    /// A compile language the writer set.
    CompileSettings,
    /// The script the Draft is written in (I3 A8).
    Script,
}

/// A value the import refused to propose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Refused {
    Title(TitleSource),
    Author(AuthorSource),
}

/// Whether a stated title or name is a template's placeholder rather than words: it holds a
/// `<$…>` code (a name after `<$`, closed on the same line), or is one `<…>` in angle
/// brackets (`<Proposal Title>`). A title that only holds `<$` (`Deals under <$5`) is words.
pub(crate) fn is_placeholder(value: &str) -> bool {
    let trimmed = value.trim();
    let holds_code = trimmed.match_indices("<$").any(|(at, _)| {
        let code = &trimmed[at + 2..];
        code.starts_with(|c: char| c.is_alphabetic() || c == '_' || c == '!')
            && code
                .find(['>', '<', '\n'])
                .is_some_and(|end| code[end..].starts_with('>'))
    });
    holds_code
        || (trimmed.starts_with('<') && trimmed.ends_with('>') && trimmed.chars().count() > 2)
}

/// A stated value, cleaned, or `None` when absent, empty or a placeholder (`refused` then
/// says so, for a value present and refused).
fn stated(raw: Option<&str>, refused: &mut bool) -> Option<String> {
    let raw = raw?;
    let cleaned = clean_title(Some(raw)).0;
    if cleaned.is_empty() {
        return None;
    }
    if is_placeholder(&cleaned) {
        *refused = true;
        return None;
    }
    Some(cleaned)
}

/// The title the project states in its settings, when it states one that is words: the
/// compile settings' first, then the project properties'. Refused placeholders are pushed
/// to `refusals`.
pub(crate) fn stated_title(
    project: &Project,
    refusals: &mut Vec<Refused>,
) -> Option<(String, TitleSource)> {
    for (raw, source) in [
        (
            project.compile.title.as_deref(),
            TitleSource::CompileSettings,
        ),
        (
            project.properties.title.as_deref(),
            TitleSource::ProjectProperties,
        ),
    ] {
        let mut refused = false;
        if let Some(title) = stated(raw, &mut refused) {
            return Some((title, source));
        }
        if refused {
            refusals.push(Refused::Title(source));
        }
    }
    None
}

/// The project's title: a template's own title, the one its card shows, since its
/// settings state the name of the project it was saved from (C13); else what its settings
/// state; else its folder's name.
fn title(project: &Project, refusals: &mut Vec<Refused>) -> TitleProposal {
    let template = project
        .source
        .template_title
        .as_deref()
        .map(|t| clean_title(Some(t)).0)
        .filter(|t| !t.is_empty() && !is_placeholder(t));
    let is_template = matches!(project.source.container, Container::Template(_));
    if let Some(title) = template.clone().filter(|_| is_template) {
        return TitleProposal {
            title,
            source: TitleSource::Template,
        };
    }
    if let Some((title, source)) = stated_title(project, refusals) {
        return TitleProposal { title, source };
    }
    if let Some(title) = template {
        return TitleProposal {
            title,
            source: TitleSource::Template,
        };
    }
    TitleProposal {
        title: folder_title(project),
        source: TitleSource::FolderName,
    }
}

/// The folder's name as a title: the analysis cleaned it; a model from elsewhere may only
/// carry the raw name, whose extension goes.
pub(crate) fn folder_title(project: &Project) -> String {
    if !project.source.folder_title.is_empty() {
        return project.source.folder_title.clone();
    }
    let name = &project.source.folder_name;
    let stem = if name.to_ascii_lowercase().ends_with(".scriv") {
        &name[..name.len() - ".scriv".len()]
    } else {
        name.as_str()
    };
    clean_title(Some(stem)).0
}

/// The authors the project names.
fn author(project: &Project, refusals: &mut Vec<Refused>) -> Option<AuthorProposal> {
    let compile = &project.compile;
    let mut refused = false;
    let surname = stated(compile.surname.as_deref(), &mut refused);
    let forename = stated(compile.forename.as_deref(), &mut refused);
    let mut names = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for author in compile.authors.iter().take(compile::MAX_AUTHORS) {
        let role = author.role.as_deref().map(str::trim).unwrap_or_default();
        if !(role.is_empty() || role.eq_ignore_ascii_case("aut")) {
            continue;
        }
        let Some(name) = stated(Some(&author.name), &mut refused) else {
            continue;
        };
        // "Hollest, Oskel" beside Surname Hollest and Forename Oskel, with no FileAs: the
        // sorting form, written as it prints.
        let flipped = match (&surname, &forename, &author.file_as) {
            (Some(s), Some(f), None) if name == format!("{s}, {f}") => format!("{f} {s}"),
            _ => name,
        };
        if seen.insert(flipped.clone()) {
            names.push(flipped);
        }
    }
    if names.is_empty() {
        let joined: Vec<&str> = [forename.as_deref(), surname.as_deref()]
            .into_iter()
            .flatten()
            .collect();
        if !joined.is_empty() {
            names.push(joined.join(" "));
        }
    }
    if refused {
        refusals.push(Refused::Author(AuthorSource::CompileSettings));
    }
    if !names.is_empty() {
        return Some(AuthorProposal {
            names,
            source: AuthorSource::CompileSettings,
        });
    }
    let properties = &project.properties;
    let mut refused = false;
    let full = stated(properties.full_name.as_deref(), &mut refused);
    let parts: Vec<String> = [
        stated(properties.first_name.as_deref(), &mut refused),
        stated(properties.last_name.as_deref(), &mut refused),
    ]
    .into_iter()
    .flatten()
    .collect();
    if refused {
        refusals.push(Refused::Author(AuthorSource::ProjectProperties));
    }
    let name = full.or_else(|| (!parts.is_empty()).then(|| parts.join(" ")))?;
    Some(AuthorProposal {
        names: vec![name],
        source: AuthorSource::ProjectProperties,
    })
}

/// The letters of the Draft's own texts.
fn draft_letters(project: &Project, content: &ContentFacts) -> Letters {
    let parents = project.parents();
    let mut in_draft = vec![false; project.binder.len()];
    let mut total = Letters::default();
    for (i, row) in project.binder.iter().enumerate() {
        in_draft[i] = row.kind == ItemKind::DraftFolder
            || parents[i].is_some_and(|p| in_draft.get(p).copied().unwrap_or(false));
        if in_draft[i]
            && let Some(facts) = content.items.get(i)
        {
            total = total.plus(facts.text.letters);
        }
    }
    total
}

/// The manuscript's language, as far as the project says it.
fn language(project: &Project, content: &ContentFacts) -> Option<LanguageProposal> {
    if let Some(tag) = language::from_compile(&project.compile.languages) {
        return Some(LanguageProposal {
            tag,
            source: LanguageSource::CompileSettings,
        });
    }
    language::from_scripts(&draft_letters(project, content)).map(|tag| LanguageProposal {
        tag: tag.to_string(),
        source: LanguageSource::Script,
    })
}

/// The styles, by name, whose text the project's compile leaves out (`DeleteText`) or
/// passes through as raw markup (`RawMarkup`): as the compile format Scrivener last compiled
/// with says, when the project holds it, else as any of the project's own formats says (a
/// format the writer made for this project is one they compile with). A format kept in the
/// writer's own library, or one of Scrivener's, is not in the project and says nothing.
pub(crate) fn compiled_away(compile: &CompileFacts) -> (Vec<String>, Vec<String>) {
    let last = compile
        .last_format
        .as_deref()
        .and_then(|id| compile.formats.iter().find(|f| f.id == id));
    let formats: Vec<&CompileFormat> = match last {
        Some(format) => vec![format],
        None => compile.formats.iter().collect(),
    };
    let mut deleted: Vec<String> = formats
        .iter()
        .flat_map(|f| f.deleted_styles.iter().cloned())
        .collect();
    let mut raw: Vec<String> = formats
        .iter()
        .flat_map(|f| f.raw_styles.iter().cloned())
        .collect();
    deleted.sort();
    deleted.dedup();
    raw.sort();
    raw.dedup();
    (deleted, raw)
}

/// What `project` states about itself, and what it stated that was refused.
pub fn propose(project: &Project, content: &ContentFacts) -> (Proposals, Vec<Refused>) {
    let mut refusals = Vec::new();
    let proposals = Proposals {
        title: title(project, &mut refusals),
        author: author(project, &mut refusals),
        language: language(project, content),
    };
    (proposals, refusals)
}

#[cfg(test)]
mod tests;
