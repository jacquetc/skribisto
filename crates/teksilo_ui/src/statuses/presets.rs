// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The workflow ladders a project can start from, **generated in code rather than shipped
//! as data** — the same trick, for the same reason, as [`crate::tags::presets`].
//!
//! A rung's name is *project data*: it goes through `tr!` once, here, at the moment the
//! ladder is seeded, and is stored literally from then on. That is the difference between
//! this and the anti-pattern yWriter and Plume Creator both shipped, where the names are UI
//! strings over a stored integer — so the same project reads "1st draft" to one writer and
//! "1er brouillon" to another, and neither can rename a rung without editing a locale file.
//! Here the French writer gets French rungs *and* can rename them.
//!
//! Unlike the tag palette, a ladder is seeded **unconditionally** when a project is
//! created. An empty tag palette is a project that simply has no tags yet; an empty ladder
//! is a project with no status feature at all.
//!
//! # Why the categories are assigned here and not derived
//!
//! [`StatusCategory`] owns the glyph and the per-theme colour, and the app owns the
//! category. A preset therefore has to say which bucket each rung sits in, and it is a
//! judgement per ladder — "1st Edit" is drafting in an eight-rung pass-counting ladder and
//! would be revision in a four-rung one. Deriving it from position would get the short
//! ladders right and the long ones wrong.

use common::entities::StatusCategory;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;

/// One rung, as a preset describes it: a name to resolve and the bucket it belongs to.
pub struct StatusRow {
    pub name: LocalizedString,
    pub category: StatusCategory,
}

const fn rung(name: LocalizedString, category: StatusCategory) -> StatusRow {
    StatusRow { name, category }
}

/// Which ladder the writer picked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preset {
    /// Four rungs. The default, and the shape a whole professional industry settled on:
    /// XLIFF 2.0 defines exactly four translation states (`initial` / `translated` /
    /// `reviewed` / `final`) for tooling with far more process discipline than novelists
    /// have, and extends by *refining* a rung rather than adding more.
    Drafting,
    /// Five rungs, counting revision passes. Near-identical across yWriter, oStorybook and
    /// novelibre, which makes it a genuine domain constant rather than one tool's taste.
    Passes,
    /// Plume Creator's own eight rungs, for a writer arriving from it — so an imported
    /// project's `status` indices land on rungs that mean what they meant before.
    Plume,
}

impl Preset {
    /// Every preset, in menu order.
    pub const ALL: [Preset; 3] = [Preset::Drafting, Preset::Passes, Preset::Plume];

    /// What a project gets when nobody chose.
    pub const DEFAULT: Preset = Preset::Drafting;

    pub fn label(self) -> LocalizedString {
        match self {
            Preset::Drafting => tr!(status_preset_drafting()),
            Preset::Passes => tr!(status_preset_passes()),
            Preset::Plume => tr!(status_preset_plume()),
        }
    }

    /// The rungs, in ladder order, resolved in the active locale.
    pub fn rows(self) -> Vec<StatusRow> {
        use StatusCategory as C;
        match self {
            Preset::Drafting => vec![
                rung(tr!(status_todo()), C::Planned),
                rung(tr!(status_draft()), C::Drafting),
                rung(tr!(status_revised()), C::Revised),
                rung(tr!(status_final()), C::Final),
            ],
            Preset::Passes => vec![
                rung(tr!(status_outline()), C::Planned),
                rung(tr!(status_draft()), C::Drafting),
                rung(tr!(status_first_edit()), C::Drafting),
                rung(tr!(status_second_edit()), C::Revised),
                rung(tr!(status_done()), C::Final),
            ],
            // Plume's ladder verbatim, in its own order, because a Plume project stores an
            // *index* into exactly this list. Reordering or collapsing it would silently
            // re-point every imported scene.
            Preset::Plume => vec![
                rung(tr!(status_plume_draft_1()), C::Planned),
                rung(tr!(status_plume_draft_2()), C::Drafting),
                rung(tr!(status_plume_draft_3()), C::Drafting),
                rung(tr!(status_plume_edit_1()), C::Drafting),
                rung(tr!(status_plume_edit_2()), C::Drafting),
                rung(tr!(status_plume_edit_3()), C::Revised),
                rung(tr!(status_plume_proofread()), C::Revised),
                rung(tr!(status_plume_finished()), C::Final),
            ],
        }
    }

    /// The rung names alone, resolved — what the Plume importer needs, since a `.plume`
    /// stores indices into this list and carries no names of its own.
    pub fn resolved_names(self) -> Vec<String> {
        self.rows()
            .into_iter()
            .map(|r| r.name.resolve_now())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preset_is_a_ladder_that_starts_before_it_ends() {
        for p in Preset::ALL {
            let rows = p.rows();
            assert!(rows.len() >= 2, "{p:?} is not a ladder");
            assert_eq!(
                rows.first().map(|r| &r.category),
                Some(&StatusCategory::Planned),
                "{p:?} should open on a not-started rung"
            );
            assert_eq!(
                rows.last().map(|r| &r.category),
                Some(&StatusCategory::Final),
                "{p:?} should close on a finished rung"
            );
        }
    }

    #[test]
    fn no_preset_repeats_a_name() {
        for p in Preset::ALL {
            let mut names: Vec<String> =
                p.rows().into_iter().map(|r| r.name.resolve_now()).collect();
            let total = names.len();
            names.sort();
            names.dedup();
            assert_eq!(names.len(), total, "{p:?} repeats a rung name: {names:?}");
        }
    }

    #[test]
    fn every_rung_has_a_name() {
        for p in Preset::ALL {
            for r in p.rows() {
                assert!(
                    !r.name.resolve_now().trim().is_empty(),
                    "{p:?} has a blank rung"
                );
            }
        }
    }

    /// The Plume ladder is an index space, not a menu: its length and order are a
    /// compatibility promise to `plume_import`, which maps `status="N"` straight onto it.
    #[test]
    fn the_plume_ladder_has_exactly_plumes_eight_rungs() {
        assert_eq!(Preset::Plume.rows().len(), 8);
        assert_eq!(Preset::Plume.resolved_names().len(), 8);
    }

    /// A category may repeat — an eight-rung ladder collapses onto five shapes and stays
    /// readable because the *name* is what tells two `Drafting` rungs apart.
    #[test]
    fn a_long_ladder_may_reuse_a_category() {
        let cats: Vec<_> = Preset::Plume
            .rows()
            .into_iter()
            .map(|r| r.category)
            .collect();
        assert!(
            cats.iter()
                .filter(|c| **c == StatusCategory::Drafting)
                .count()
                > 1,
            "the Plume ladder is expected to share the Drafting bucket"
        );
    }
}
