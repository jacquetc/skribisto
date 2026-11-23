// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What one row puts into the exported book, decided apart from how it is written.
//!
//! The compiler's row loop used to make these decisions inline, between the lines that
//! write the djot: whether a note or a paratext is left out, which heading a chapter
//! opens on, whether its epigraph prints and on which side of that heading, whether the
//! synopsis rides along. Anything else that wanted to show the book "as exported" would
//! have had to copy them, and a copy is a second answer waiting to drift from the first.
//!
//! So the decision lives here, as a pure function over plain facts, and both readers ask
//! it: `render::assemble`, which writes the file, and the UI's Full Chapter / Part / Book
//! read as exported, which lays the same rows out on screen. That is the rule chapter
//! numbering already follows, and for the same reason: the two must agree by
//! construction, not by care.
//!
//! What this deliberately does **not** decide is anything about pages and type: where a
//! break lands in a particular format, the title page, covers, fonts. Those stay in the
//! renderer, because no screen can show them faithfully and the reading view does not
//! claim to.

use common::entities::{BinderItemSubRole, ContentRole};
use skribisto_model::SubRoleExt;
use skribisto_model::numbering;

use crate::headings::{self, Level};
use crate::preset::{EpigraphPlacement, HeadingScheme, Preset};

/// What the decision needs to know about one row. Facts only, so the backend's frozen tree
/// and the UI's live store can both supply them.
#[derive(Clone, Copy, Debug)]
pub struct RowFacts<'a> {
    pub sub_role: &'a BinderItemSubRole,
    /// The row's title, trimmed, or `None` when it is blank. For a structural row this is
    /// its title content when it has one, else the item's own title; a scene only has
    /// the latter.
    pub title: Option<&'a str>,
    /// The row's ordinal in the manuscript, or `None` when it holds none (a prologue
    /// excluded from numbering, an unnumbered book, a scene).
    pub number: Option<usize>,
    /// The language the structural words are written in, with the style's heading
    /// language already applied.
    pub heading_lang: &'a str,
    /// The row carries an epigraph that is not blank.
    pub has_epigraph: bool,
    /// The row holds its main prose: the scene's, note's or paratext's own text.
    pub has_main: bool,
    /// The row holds a synopsis.
    pub has_synopsis: bool,
}

/// Which kind of heading a row opens on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadingKind {
    /// A book, part or chapter heading.
    Structure(Level),
    /// A scene's own title, kept by the style in place of a scene break.
    SceneTitle,
}

/// The heading a row prints, as the reader will read it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RowHeading {
    pub kind: HeadingKind,
    pub text: String,
}

/// What one row prints, in the order it prints it: the epigraph when it leads, the
/// heading, the epigraph when it follows, the prose, the synopsis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RowPlan {
    /// The style opens a new page here: at a structural level it breaks on, or at a
    /// paratext.
    pub starts_page: bool,
    /// Where the epigraph prints relative to the heading, or `None` when it does not.
    pub epigraph: Option<EpigraphPlacement>,
    pub heading: Option<RowHeading>,
    /// The row's main prose prints.
    pub prose: bool,
    /// The row's synopsis prints.
    pub synopsis: bool,
}

impl RowPlan {
    /// Whether the row puts anything at all on the page. A plain folder, a front-matter
    /// folder or a book opener under a title page is in the scope and prints nothing.
    pub fn prints_anything(&self) -> bool {
        self.heading.is_some() || self.epigraph.is_some() || self.prose || self.synopsis
    }
}

/// Decide what one row of an export puts into the book, or `None` when the style leaves
/// the row out whole.
///
/// `explicit_selection` is the export's own flag of that name: an item the writer pointed
/// at (Export Note, a checked Choose… item) prints even when the style drops its kind.
pub fn plan_row(facts: &RowFacts, preset: &Preset, explicit_selection: bool) -> Option<RowPlan> {
    // A note swept into a structural scope is dropped unless the style keeps notes, or
    // the selection was explicit: the writer pointed at it, which overrides the toggle.
    if matches!(facts.sub_role, BinderItemSubRole::Note)
        && !preset.include_notes
        && !explicit_selection
    {
        return None;
    }
    // A paratext is the author's, but it is not the story. The style can leave it out:
    // the clean-submission case, where an editor wants the manuscript and not the
    // acknowledgements.
    //
    // Note that a paratext prints **no heading of its own**. Its title is a binder label
    // the writer chose to find it by ("Copyright", "front matter (draft)"), not a line of
    // the book. A writer who wants a heading on the page writes one in the prose, where
    // they control its wording and its level. The same reasoning keeps a
    // `Folder/Paratext` from printing anything at all.
    let is_paratext = matches!(facts.sub_role, BinderItemSubRole::Paratext);
    if is_paratext && !preset.include_paratexts {
        return None;
    }

    let level = numbering::level_of(facts.sub_role);
    let starts_page = (is_paratext && preset.paratext_starts_page)
        || level.is_some_and(|level| match level {
            Level::Book => preset.book_starts_page,
            Level::Part => preset.part_starts_page,
            Level::Chapter => preset.chapter_starts_page,
        });

    let heading = match level {
        Some(level) => {
            // A book is titled, not numbered, and the title page (if on) already carries
            // it, so the opener then prints nothing. Parts and chapters use their schemes.
            let scheme = match level {
                Level::Book if preset.book_title_page => HeadingScheme::None,
                Level::Book => HeadingScheme::TitleOnly,
                Level::Part => preset.part_heading,
                Level::Chapter => preset.chapter_heading,
            };
            heading_text(
                facts.title,
                level,
                facts.number,
                facts.heading_lang,
                preset,
                scheme,
            )
            .map(|text| RowHeading {
                kind: HeadingKind::Structure(level),
                text,
            })
        }
        // A plain titled scene (never a chapter scene: that opened on its chapter heading
        // above). The title stands in for the scene break.
        None if preset.include_scene_titles && facts.sub_role.carries_scene() => {
            facts.title.map(|t| RowHeading {
                kind: HeadingKind::SceneTitle,
                text: t.to_string(),
            })
        }
        None => None,
    };

    Some(RowPlan {
        starts_page,
        // Every convention that writes the rule down puts the epigraph after the title;
        // above it is a designer's choice with real currency, so the style decides.
        epigraph: (preset.include_epigraphs && facts.has_epigraph)
            .then_some(preset.epigraph_placement),
        heading,
        prose: main_prose_role(facts.sub_role).is_some() && facts.has_main,
        synopsis: preset.include_synopses && facts.has_synopsis,
    })
}

/// The content role that holds a row's main prose: a scene's text, a note's, a
/// paratext's. `None` for a row with no prose of its own.
pub fn main_prose_role(sr: &BinderItemSubRole) -> Option<ContentRole> {
    if sr.carries_scene() {
        Some(ContentRole::SceneText)
    } else if matches!(sr, BinderItemSubRole::Note) {
        Some(ContentRole::NoteText)
    } else if matches!(sr, BinderItemSubRole::Paratext) {
        Some(ContentRole::ParatextText)
    } else {
        None
    }
}

/// Compose one structural heading.
///
/// `number` is `None` when the row holds no ordinal — because the writer excluded it from
/// numbering (a prologue), or because the manuscript is unnumbered. Every scheme then
/// falls back to the title, and a scheme that cannot produce anything at all returns
/// `None` rather than inventing a numeral.
fn heading_text(
    title: Option<&str>,
    level: Level,
    number: Option<usize>,
    lang: &str,
    preset: &Preset,
    scheme: HeadingScheme,
) -> Option<String> {
    let numbered = number.map(|n| headings::numbered(lang, level, n, preset.digit_style));
    match scheme {
        HeadingScheme::None => None,
        // An unnumbered row under a numbers-only scheme still needs naming — a prologue
        // typeset as a blank chapter opener would be a hole in the book.
        HeadingScheme::Numbered => numbered.or_else(|| title.map(str::to_string)),
        // `TitleOnly` degrading to a bare number when the title is empty is deliberate and
        // long-standing — an untitled chapter is better opened by "Chapter 3" than by
        // nothing at all — but it *is* a degrade, and named as one here rather than left
        // for a reader of this match to infer from an `or_else`.
        HeadingScheme::TitleOnly => title.map(str::to_string).or(numbered),
        HeadingScheme::NumberAndTitle => match (numbered, title) {
            // A title that already *is* the number is not a title to append: writers name
            // their chapters "Chapter 5" (and every project this app creates starts out
            // that way) purely because nothing else ever showed them the number. See
            // `headings::is_redundant_number_title` for exactly what counts as saying the
            // same thing, and for why a *different* number is left visibly doubled.
            (Some(n), Some(t))
                if !headings::is_redundant_number_title(t, lang, level, number.unwrap_or(0)) =>
            {
                // Folded to spaces: a heading is one line (see `Preset::heading_separator`).
                let sep = preset.heading_separator.replace(['\n', '\r'], " ");
                Some(format!("{n}{sep}{t}"))
            }
            (Some(n), _) => Some(n),
            (None, Some(t)) => Some(t.to_string()),
            (None, None) => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin_presets;
    use BinderItemSubRole as SR;

    fn preset() -> Preset {
        builtin_presets()
            .into_iter()
            .next()
            .unwrap_or_else(|| unreachable!("there is always a built-in style"))
    }

    fn facts(sub_role: &SR) -> RowFacts<'_> {
        RowFacts {
            sub_role,
            title: None,
            number: None,
            heading_lang: "en",
            has_epigraph: false,
            has_main: true,
            has_synopsis: false,
        }
    }

    #[test]
    fn a_paratext_prints_its_page_and_never_a_heading() {
        let mut p = preset();
        p.include_paratexts = true;
        p.paratext_starts_page = true;
        let sr = SR::Paratext;
        let plan = plan_row(
            &RowFacts {
                title: Some("Dedication"),
                ..facts(&sr)
            },
            &p,
            false,
        )
        .unwrap_or_else(|| unreachable!("kept by the style"));
        assert!(plan.prose && plan.starts_page);
        assert_eq!(
            plan.heading, None,
            "its title is a binder label, not a line"
        );

        p.include_paratexts = false;
        assert_eq!(plan_row(&facts(&sr), &p, false), None);
    }

    #[test]
    fn a_note_is_left_out_unless_kept_or_pointed_at() {
        let mut p = preset();
        p.include_notes = false;
        let sr = SR::Note;
        assert_eq!(plan_row(&facts(&sr), &p, false), None);
        assert!(plan_row(&facts(&sr), &p, true).is_some_and(|plan| plan.prose));
    }

    #[test]
    fn the_epigraph_follows_the_style_and_needs_text() {
        let mut p = preset();
        p.include_epigraphs = true;
        p.epigraph_placement = EpigraphPlacement::BeforeHeading;
        let sr = SR::ChapterScene;
        let with = RowFacts {
            has_epigraph: true,
            ..facts(&sr)
        };
        assert_eq!(
            plan_row(&with, &p, false).and_then(|plan| plan.epigraph),
            Some(EpigraphPlacement::BeforeHeading)
        );
        assert_eq!(
            plan_row(&facts(&sr), &p, false).and_then(|plan| plan.epigraph),
            None,
            "a blank epigraph prints nothing"
        );
        p.include_epigraphs = false;
        assert_eq!(
            plan_row(&with, &p, false).and_then(|plan| plan.epigraph),
            None
        );
    }

    #[test]
    fn a_chapter_opens_on_the_heading_its_scheme_composes() {
        let mut p = preset();
        p.chapter_heading = HeadingScheme::NumberAndTitle;
        p.heading_separator = ": ".into();
        let sr = SR::ChapterScene;
        let plan = plan_row(
            &RowFacts {
                title: Some("The Storm"),
                number: Some(3),
                ..facts(&sr)
            },
            &p,
            false,
        );
        assert_eq!(
            plan.and_then(|plan| plan.heading),
            Some(RowHeading {
                kind: HeadingKind::Structure(Level::Chapter),
                text: "Chapter 3: The Storm".into(),
            })
        );
    }

    #[test]
    fn a_scene_title_prints_only_when_the_style_keeps_it() {
        let mut p = preset();
        let sr = SR::Scene;
        let titled = RowFacts {
            title: Some("Dawn"),
            ..facts(&sr)
        };
        p.include_scene_titles = false;
        assert_eq!(
            plan_row(&titled, &p, false).and_then(|plan| plan.heading),
            None
        );
        p.include_scene_titles = true;
        assert_eq!(
            plan_row(&titled, &p, false).and_then(|plan| plan.heading),
            Some(RowHeading {
                kind: HeadingKind::SceneTitle,
                text: "Dawn".into(),
            })
        );
    }

    #[test]
    fn a_book_under_a_title_page_prints_nothing_of_its_own() {
        let mut p = preset();
        p.book_title_page = true;
        let sr = SR::Book;
        let plan = plan_row(
            &RowFacts {
                title: Some("Starforgers"),
                has_main: false,
                ..facts(&sr)
            },
            &p,
            false,
        )
        .unwrap_or_else(|| unreachable!("a book is never left out"));
        assert!(!plan.prints_anything());
        p.book_title_page = false;
        assert_eq!(
            plan_row(
                &RowFacts {
                    title: Some("Starforgers"),
                    has_main: false,
                    ..facts(&sr)
                },
                &p,
                false
            )
            .and_then(|plan| plan.heading)
            .map(|h| h.text),
            Some("Starforgers".into())
        );
    }
}
