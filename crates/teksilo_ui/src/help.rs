// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The help set: topics, where their prose comes from, and the registry that lets
//! something outside this workspace add one.
//!
//! ## Two tiers, one keyspace
//!
//! Skribisto already had a help system before it had a Help window:
//! [`crate::tooltip_registry`] holds a bilingual web of writing-model and target
//! concepts, each a sentence plus a longer disclosure, cross-linked by
//! `[label](:key)` and pinned by a drift test. It says so itself, in its own module
//! doc: "a lightweight, in-place substitute for a separate Help document."
//!
//! This module does not replace it. It **promotes** it. A topic is addressed by the
//! *same* key, so a concept has one identity whether the reader meets it as a tooltip
//! beside the control or as a page in the Help window:
//!
//! - [`HelpBody::Tooltip`] — the topic *is* the registered tooltip's long body,
//!   rendered full width. Costs no new prose, and cannot drift from the tooltip
//!   because it is the same string.
//! - [`HelpBody::Djot`] — a real page: headings, lists, several screens of it. This is
//!   for the answers that are genuinely procedural, where a paragraph would have to
//!   lie by omission.
//!
//! Most concepts stay in the first tier forever. The test for promotion is whether the
//! honest answer fits in two to four sentences; if it does, a page would only pad it.
//!
//! ## Locales
//!
//! A Djot body is a table of `(locale, source)` rows, not a pair of fields. Adding
//! German is adding a row: nothing in the type, the resolver or the tests counts to
//! two. Resolution tries the exact locale (`fr-CA`), then the language (`fr`), then
//! [`SOURCE_LOCALE`], and reports which happened so the page can say plainly that the
//! reader is looking at English (see [`ResolvedBody::is_fallback`]) rather than
//! silently serving it.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;

use crate::docks::LabelFn;

pub mod help_vm;
pub mod panel;
pub mod shortcuts;
pub mod window;

#[cfg(test)]
mod tests;

/// The locale every topic is authored in, and the last resort when nothing else
/// matches. Deliberately the same source locale the Fluent stack uses.
pub const SOURCE_LOCALE: &str = "en-US";

/// Where a topic sits in the Help window's table of contents.
///
/// Grouping only: a section carries no behaviour, and a topic's section can change
/// without invalidating anything that links to its key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HelpSection {
    /// First steps, and the shape of a project.
    GettingStarted,
    /// The writing model and the daily work of writing.
    Writing,
    /// Reviewing, commenting, and working with other people.
    Reviewing,
    /// Getting words in and out of Skribisto.
    Exchanging,
    /// Everything that keeps the work safe.
    Keeping,
    /// The vocabulary: one short entry per concept, browsable cold.
    ///
    /// These are the Tier-1 topics, and there are deliberately many of them. A glossary
    /// is long by nature, and the filter above the contents is what makes that fine.
    Reference,
    /// Contributed from outside this workspace.
    Extensions,
}

impl HelpSection {
    /// Every section, in the order the table of contents shows them.
    pub const ALL: &'static [HelpSection] = &[
        HelpSection::GettingStarted,
        HelpSection::Writing,
        HelpSection::Reviewing,
        HelpSection::Exchanging,
        HelpSection::Keeping,
        HelpSection::Reference,
        HelpSection::Extensions,
    ];

    /// The heading shown above this section's topics.
    pub fn label(self) -> LocalizedString {
        match self {
            HelpSection::GettingStarted => tr!(help_section_getting_started()),
            HelpSection::Writing => tr!(help_section_writing()),
            HelpSection::Reviewing => tr!(help_section_reviewing()),
            HelpSection::Exchanging => tr!(help_section_exchanging()),
            HelpSection::Keeping => tr!(help_section_keeping()),
            HelpSection::Reference => tr!(help_section_reference()),
            HelpSection::Extensions => tr!(help_section_extensions()),
        }
    }
}

/// One locale's source for a topic: the locale tag it was written for, and the Djot.
pub type LocalizedSource = (&'static str, &'static str);

/// Where a topic's prose comes from.
#[derive(Clone)]
pub enum HelpBody {
    /// Render the long body of a registered rich tooltip, by its registry key.
    ///
    /// The key is usually the topic's own key, and the two being the same string is
    /// the point: one concept, one identity, two renderings.
    Tooltip(&'static str),
    /// Render a Djot document, chosen from this table by the active locale.
    ///
    /// Every row is `(locale_tag, source)`. There is no required length and no
    /// required order beyond [`SOURCE_LOCALE`] being present, which the
    /// `every_djot_topic_has_a_source_locale_body` drift test enforces.
    Djot(&'static [LocalizedSource]),
}

/// A topic body resolved for one locale.
pub struct ResolvedBody {
    /// The Djot source to render.
    pub source: &'static str,
    /// The locale the source was actually written for.
    pub locale: &'static str,
    /// True when the reader asked for a locale this topic has not been translated into
    /// yet, so [`Self::source`] is the source-locale text.
    ///
    /// The window shows a line saying so. A silent fallback would be the same bug this
    /// project keeps finding in its own surfaces: presenting something as an answer to
    /// a question it did not actually answer.
    pub is_fallback: bool,
}

/// One entry in the Help window.
#[derive(Clone)]
pub struct HelpTopicSpec {
    /// Stable, and the same key the tooltip registry uses where the concept has one.
    /// Never renumbered: it is what `[label](:key)` links resolve against, and what a
    /// context-sensitive Help entry point names.
    pub key: &'static str,
    /// Which group of the table of contents this sits in.
    pub section: HelpSection,
    /// Resolved per build, so a runtime language switch reaches it. (Storing a
    /// `LocalizedString` here would pin the title to whatever locale was active when
    /// the topic was registered.)
    pub title: LabelFn,
    /// Where the prose comes from.
    pub body: HelpBody,
}

impl HelpTopicSpec {
    /// Pick the body for `locale`, falling back by language and then to the source
    /// locale.
    ///
    /// `None` only for a [`HelpBody::Tooltip`] topic, whose text comes from the Fluent
    /// stack instead and is already locale-resolved by `tr!`.
    pub fn resolve(&self, locale: &str) -> Option<ResolvedBody> {
        let HelpBody::Djot(table) = &self.body else {
            return None;
        };
        Some(resolve_source(table, locale))
    }
}

/// Choose a source from `table` for `locale`: exact tag, then language, then source
/// locale, then whatever the table's first row is.
///
/// The last resort exists so a contributed topic that forgot the source locale still
/// renders something rather than an empty page. The drift test refuses that shape for
/// built-in topics, so it is only ever reached by a registration from outside.
fn resolve_source(table: &'static [LocalizedSource], locale: &str) -> ResolvedBody {
    let wanted_language = language_of(locale);

    if let Some((tag, source)) = table.iter().find(|(tag, _)| *tag == locale) {
        return ResolvedBody {
            source,
            locale: tag,
            is_fallback: false,
        };
    }
    if let Some((tag, source)) = table
        .iter()
        .find(|(tag, _)| language_of(tag) == wanted_language)
    {
        return ResolvedBody {
            source,
            locale: tag,
            // Same language, different region: `fr-CA` reading the `fr-FR` page is not
            // the reader being handed a language they did not ask for.
            is_fallback: false,
        };
    }
    let (tag, source) = table
        .iter()
        .find(|(tag, _)| *tag == SOURCE_LOCALE)
        .or_else(|| table.first())
        .copied()
        .unwrap_or((SOURCE_LOCALE, ""));
    ResolvedBody {
        source,
        locale: tag,
        is_fallback: true,
    }
}

/// The language subtag of a locale tag: `fr-FR` and `fr-CA` are both `fr`.
fn language_of(tag: &str) -> &str {
    tag.split(['-', '_']).next().unwrap_or(tag)
}

// ── The registry ────────────────────────────────────────────────────────────

struct RegisteredTopic {
    namespace: String,
    spec: HelpTopicSpec,
}

// Thread-local for the same reason the analysis-category registry is: a spec holds an
// `Rc` label closure, and only the UI thread ever reads one.
thread_local! {
    static EXTENSION_TOPICS: RefCell<Vec<RegisteredTopic>> = const { RefCell::new(Vec::new()) };
}

/// Add a topic to the Help window.
///
/// Refuses a key a built-in already uses, or one another namespace registered: the key
/// is what a link resolves against, so two claimants make that lookup ambiguous.
/// Registering the same namespace again replaces its previous topics.
///
/// ⚠ Like every other seam in this app, the registry is read when the Help window is
/// built, not subscribed to. Register at startup, before any window exists.
pub fn register_topics(
    namespace: impl Into<String>,
    specs: Vec<HelpTopicSpec>,
) -> Result<TopicsHandle, String> {
    let namespace = namespace.into();
    for spec in &specs {
        if builtin_topics().iter().any(|t| t.key == spec.key) {
            return Err(format!("help topic key '{}' is a built-in", spec.key));
        }
    }
    EXTENSION_TOPICS.with(|reg| {
        let mut reg = reg.borrow_mut();
        for spec in &specs {
            if let Some(other) = reg
                .iter()
                .find(|r| r.spec.key == spec.key && r.namespace != namespace)
            {
                return Err(format!(
                    "help topic key '{}' is already registered by '{}'",
                    spec.key, other.namespace
                ));
            }
        }
        reg.retain(|r| r.namespace != namespace);
        for spec in specs {
            reg.push(RegisteredTopic {
                namespace: namespace.clone(),
                spec,
            });
        }
        Ok(TopicsHandle {
            namespace: namespace.clone(),
        })
    })
}

/// Unregisters its topics when dropped.
#[derive(Debug)]
pub struct TopicsHandle {
    namespace: String,
}

impl Drop for TopicsHandle {
    fn drop(&mut self) {
        // `try_with`: a handle dropped during thread teardown must not panic.
        let _ = EXTENSION_TOPICS.try_with(|reg| {
            reg.borrow_mut().retain(|r| r.namespace != self.namespace);
        });
    }
}

/// Every topic, built-in first, in table-of-contents order.
pub fn all_topics() -> Vec<HelpTopicSpec> {
    let mut topics = builtin_topics();
    EXTENSION_TOPICS.with(|reg| {
        topics.extend(reg.borrow().iter().map(|r| r.spec.clone()));
    });
    topics
}

/// The topic with this key, if any. This is what a `[label](:key)` link resolves
/// through, and what a context-sensitive entry point names.
pub fn topic(key: &str) -> Option<HelpTopicSpec> {
    all_topics().into_iter().find(|t| t.key == key)
}

/// The key the Help window opens on when nothing more specific was asked for.
pub const DEFAULT_TOPIC: &str = "help-getting-started";

// ── The built-in topics ─────────────────────────────────────────────────────

/// One `(locale, source)` table per topic, built by including every locale's file.
///
/// A new locale is one line per topic here. The paths are literal because
/// `include_str!` needs them to be; the drift test walks the directory instead of this
/// list, so a file added without a line here fails the build rather than shipping
/// unreachable.
macro_rules! djot_topic {
    ($file:literal) => {
        HelpBody::Djot(&[
            (
                "en-US",
                include_str!(concat!("../help/en-US/", $file, ".djot")),
            ),
            (
                "fr-FR",
                include_str!(concat!("../help/fr-FR/", $file, ".djot")),
            ),
        ])
    };
}

/// The concepts the rich-tooltip registry already teaches, as browsable topics.
///
/// This is the Tier-1 half of the design, and the reason the cost argument in the plan
/// holds: **not one word here is new.** Each entry renders the tooltip already written
/// for it, in both locales, so a concept explains itself identically whether the reader
/// meets it beside the control (`.rich_tooltip(key)`) or looks it up cold in this
/// window. The two cannot drift, because they are the same `LocalizedString`.
///
/// The title is not new either: where the concept is something you can create, it
/// reuses the very label the "＋ Create" menu offers, so the glossary and the menu can
/// never call the same thing two names. A handful of concepts are not create rows and
/// have no such label; those get a title of their own, and only those.
///
/// Order is the order the contents shows: the writing vocabulary first, then the
/// target and pace vocabulary, matching the two webs `tooltip_registry` documents.
fn concept_topics() -> Vec<HelpTopicSpec> {
    use crate::tooltip_registry as tips;

    // (tooltip key, title). The title closure is resolved per build, so a locale switch
    // reaches it.
    let rows: Vec<(&'static str, LabelFn)> = vec![
        (tips::WM_BOOK, Rc::new(|| tr!(create_book()))),
        (tips::WM_PART, Rc::new(|| tr!(create_part()))),
        (tips::WM_CHAPTER, Rc::new(|| tr!(create_chapter()))),
        (tips::WM_SCENE, Rc::new(|| tr!(create_scene()))),
        (tips::WM_NOTE, Rc::new(|| tr!(create_note()))),
        (tips::WM_NOTE_FOLDER, Rc::new(|| tr!(create_note_folder()))),
        (tips::WM_FOLDER, Rc::new(|| tr!(create_folder()))),
        (tips::WM_PARATEXT, Rc::new(|| tr!(create_paratext()))),
        (
            tips::WM_PARATEXT_FOLDER,
            Rc::new(|| tr!(create_paratext_folder())),
        ),
        (tips::WM_END_OF_BOOK, Rc::new(|| tr!(create_book_end()))),
        (
            tips::WM_STORY_BIBLE_ENTRY,
            Rc::new(|| tr!(create_story_bible_entry())),
        ),
        (tips::WM_SYNOPSIS, Rc::new(|| tr!(synopsis()))),
        (
            tips::SCENE_BREAK_MINOR,
            Rc::new(|| tr!(help_concept_scene_break())),
        ),
        (
            tips::SCENE_BREAK_MAJOR,
            Rc::new(|| tr!(help_concept_major_scene_break())),
        ),
        (
            tips::WM_FIND_IN_PROSE,
            Rc::new(|| tr!(help_concept_find_in_prose())),
        ),
        (
            tips::WM_STORY_BIBLE,
            Rc::new(|| tr!(help_concept_story_bible())),
        ),
        (tips::GOAL_TARGET, Rc::new(|| tr!(inspector_goal()))),
        (tips::GOAL_UNIT, Rc::new(|| tr!(help_concept_goal_unit()))),
        (
            tips::GOAL_PROGRESS,
            Rc::new(|| tr!(help_concept_goal_progress())),
        ),
        (
            tips::GOAL_MANUSCRIPT_WORDS,
            Rc::new(|| tr!(help_concept_manuscript_words())),
        ),
        (
            tips::GOAL_EXPORTABLE,
            Rc::new(|| tr!(help_concept_exportable())),
        ),
        (
            tips::GOAL_DISTRIBUTE,
            Rc::new(|| tr!(help_concept_distribute())),
        ),
        (
            tips::GOAL_SUBTREE_TOTAL,
            Rc::new(|| tr!(help_concept_subtree_total())),
        ),
        (
            tips::GOAL_MILESTONE,
            Rc::new(|| tr!(help_concept_milestone())),
        ),
        (tips::PACE_PLAN, Rc::new(|| tr!(help_concept_pace_plan()))),
        (tips::CONCEPT_TAG, Rc::new(|| tr!(help_concept_tag()))),
        (tips::CONCEPT_LABEL, Rc::new(|| tr!(help_concept_label()))),
        (
            tips::CONCEPT_POINT_OF_VIEW,
            Rc::new(|| tr!(help_concept_point_of_view())),
        ),
        (
            tips::CONCEPT_EPIGRAPH,
            Rc::new(|| tr!(help_concept_epigraph())),
        ),
        (
            tips::CONCEPT_FOOTNOTE,
            Rc::new(|| tr!(help_concept_footnote())),
        ),
        (
            tips::CONCEPT_CHAPTER_MODE,
            Rc::new(|| tr!(help_concept_chapter_mode())),
        ),
        (
            tips::CONCEPT_COMMENT,
            Rc::new(|| tr!(help_concept_comment())),
        ),
        (tips::CONCEPT_BACKUP, Rc::new(|| tr!(help_concept_backup()))),
        (
            tips::CONCEPT_VERSION,
            Rc::new(|| tr!(help_concept_version())),
        ),
        (tips::CONCEPT_TRASH, Rc::new(|| tr!(help_concept_trash()))),
        (
            tips::CONCEPT_SPELLCHECK,
            Rc::new(|| tr!(help_concept_spellcheck())),
        ),
        (
            tips::CONCEPT_SEARCH_REPLACE,
            Rc::new(|| tr!(help_concept_search_replace())),
        ),
        (
            tips::CONCEPT_NOTE_TEMPLATE,
            Rc::new(|| tr!(help_concept_note_template())),
        ),
        (
            tips::CONCEPT_TEXT_REPLACEMENT,
            Rc::new(|| tr!(help_concept_text_replacement())),
        ),
        (
            tips::CONCEPT_SMART_PUNCTUATION,
            Rc::new(|| tr!(help_concept_smart_punctuation())),
        ),
        (
            tips::CONCEPT_EXPORT_STYLE,
            Rc::new(|| tr!(help_concept_export_style())),
        ),
        (
            tips::CONCEPT_ROUND_TRIP_MARKS,
            Rc::new(|| tr!(help_concept_round_trip_marks())),
        ),
    ];

    rows.into_iter()
        .map(|(key, title)| HelpTopicSpec {
            // The topic key *is* the tooltip key. One concept, one identity: a
            // `[label](:wm-scene)` link written for a tooltip resolves here unchanged.
            key,
            section: HelpSection::Reference,
            title,
            body: HelpBody::Tooltip(key),
        })
        .collect()
}

/// The topics this application ships.
///
/// Order within a section is the order they appear. Keep the first one the one a
/// reader who opened Help by accident should land on.
pub fn builtin_topics() -> Vec<HelpTopicSpec> {
    vec![
        HelpTopicSpec {
            key: "help-getting-started",
            section: HelpSection::GettingStarted,
            title: Rc::new(|| tr!(help_topic_getting_started())),
            body: djot_topic!("getting-started"),
        },
        HelpTopicSpec {
            key: "help-writing-model",
            section: HelpSection::Writing,
            title: Rc::new(|| tr!(help_topic_writing_model())),
            body: djot_topic!("writing-model"),
        },
        HelpTopicSpec {
            key: "help-goals-and-pace",
            section: HelpSection::Writing,
            title: Rc::new(|| tr!(help_topic_goals_and_pace())),
            body: djot_topic!("goals-and-pace"),
        },
        HelpTopicSpec {
            key: "help-comments",
            section: HelpSection::Reviewing,
            title: Rc::new(|| tr!(help_topic_comments())),
            body: djot_topic!("comments"),
        },
        HelpTopicSpec {
            key: "help-round-trip",
            section: HelpSection::Reviewing,
            title: Rc::new(|| tr!(help_topic_round_trip())),
            body: djot_topic!("round-trip"),
        },
        HelpTopicSpec {
            key: "help-export",
            section: HelpSection::Exchanging,
            title: Rc::new(|| tr!(help_topic_export())),
            body: djot_topic!("export"),
        },
        HelpTopicSpec {
            key: "help-import-documents",
            section: HelpSection::Exchanging,
            title: Rc::new(|| tr!(help_topic_import_documents())),
            body: djot_topic!("import-documents"),
        },
        HelpTopicSpec {
            key: "help-backups-and-versions",
            section: HelpSection::Keeping,
            title: Rc::new(|| tr!(help_topic_backups_and_versions())),
            body: djot_topic!("backups-and-versions"),
        },
    ]
    .into_iter()
    .chain(concept_topics())
    .collect()
}
