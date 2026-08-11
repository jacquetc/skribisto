// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which heading depth means which kind of row.
//!
//! ## Why this is deliberately simple
//!
//! Every surveyed tool that guesses hard at structure — eight-pattern regex
//! cascades, word-count sizing, keyword vocabularies — is
//! guessing because it has no chance to ask. Skribisto does: nothing is committed
//! until the writer has seen the tree and can retype any row. Once a correction
//! costs one click, an elaborate heuristic buys very little and costs a great
//! deal of behaviour nobody can predict from the outside.
//!
//! So the rule is a *table*, not an algorithm: heading level → kind, seeded by
//! walking down the structural ladder from whatever the shallowest level in the
//! document is, and adjustable as a whole. That is also the only shape that
//! scales — a two-hundred-chapter import corrected row by row is two hundred
//! corrections, but "H2 means Chapter" is one.

use std::collections::BTreeMap;

use skribisto_model::CreateType;

/// The containment ladder a deeper heading steps down.
///
/// Stops at `Scene`: nothing nests inside a scene, so every level below the
/// scene level is also a scene. That is what keeps a stray `#####` from becoming
/// a phantom row — a failure documented in other converters, where a heading
/// deeper than the tool understood silently opened an unnamed scene whose body
/// began with the literal `#### `.
const LADDER: &[CreateType] = &[
    CreateType::Book,
    CreateType::Part,
    CreateType::Chapter,
    CreateType::Scene,
];

/// Whether a starting kind can hold the prose of a headingless document.
///
/// Deliberately about the *container* axis rather than the constraint matrix: asking
/// `skribisto_model::allowed_content` needs a `(role, sub_role)`, which needs the
/// project's `ChapterMode`, which this function does not have and should not need —
/// a Book holds no prose under either mode, and neither does a Part or any folder.
/// `plan::flag_illegal_combinations` still asks the matrix properly, per row, with the
/// mode in hand; this only has to avoid *seeding* a rule that it would then flag.
fn start_holds_prose(kind: CreateType) -> bool {
    !matches!(
        kind,
        CreateType::Book
            | CreateType::Part
            | CreateType::Folder
            | CreateType::NoteFolder
            | CreateType::ParatextFolder
    )
}

/// What the shallowest imported heading should become, landing at a destination.
///
/// The other half of [`infer_rules`]' `start` parameter: that function's doc states
/// the rule ("importing into a Book means the shallowest heading is a Part; importing
/// at the root means it is a Book") and this is where the rule actually lives, beside
/// the `LADDER` it steps.
///
/// `into` is "inside this container" as opposed to "beside this row". Landing beside
/// something makes the import its **sibling**, so it takes that row's own kind; landing
/// inside steps one rung down, and stops at the bottom — a Scene inside a Scene is still
/// a Scene, the same clamp that keeps a stray `#####` from inventing a phantom level.
///
/// An anchor off the ladder — a note, a paratext, a plain folder — has no rung to step
/// from. `None` says so rather than guessing; the caller decides what a destination it
/// cannot reason about should mean, and the honest answer there is "leave the import
/// where the writer put it".
pub fn start_kind_at(anchor: CreateType, into: bool) -> Option<CreateType> {
    let rung = LADDER.iter().position(|k| *k == anchor)?;
    if !into {
        return Some(anchor);
    }
    Some(LADDER[(rung + 1).min(LADDER.len() - 1)])
}

/// Heading level (1–6) → what to create for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelRules {
    by_level: BTreeMap<u8, CreateType>,
    /// Applied to a level deeper than any the table names.
    deepest: CreateType,
}

impl LevelRules {
    /// Build from an explicit table — what the review panel hands back when the
    /// writer adjusts it.
    pub fn from_table(table: BTreeMap<u8, CreateType>) -> Self {
        let deepest = table
            .values()
            .copied()
            .next_back()
            .unwrap_or(CreateType::Scene);
        LevelRules {
            by_level: table,
            deepest,
        }
    }

    /// What heading `level` creates.
    pub fn kind_for(&self, level: u8) -> CreateType {
        self.by_level
            .get(&level)
            .copied()
            .or_else(|| {
                // Deeper than the table goes: take the deepest named rule rather
                // than inventing a level.
                self.by_level
                    .range(..=level)
                    .next_back()
                    .map(|(_, kind)| *kind)
            })
            .unwrap_or(self.deepest)
    }

    /// The levels the table names, shallowest first — what the panel renders.
    pub fn levels(&self) -> impl Iterator<Item = (u8, CreateType)> + '_ {
        self.by_level.iter().map(|(l, k)| (*l, *k))
    }

    /// Retype one level, as the writer adjusting the rule table.
    pub fn set(&mut self, level: u8, kind: CreateType) {
        self.by_level.insert(level, kind);
    }
}

/// Seed a rule table from the heading levels a set of documents actually uses.
///
/// `start` is where the ladder begins — the deepest kind that can legally
/// contain what is being imported. Importing into a Book means the shallowest
/// heading is a Part; importing at the root means it is a Book.
///
/// Levels that appear are mapped in order down the ladder, so a document using
/// `#`/`##`/`###` and one using `##`/`###` produce the same tree. Absolute depth
/// is meaningless on its own — Skribisto's own Markdown export proves it, since
/// it writes heading levels dense over whatever the export's scope happened to
/// contain, so the same `##` is a Part in one file and a Chapter in another.
pub fn infer_rules(levels_used: &[u8], start: CreateType) -> LevelRules {
    let mut used: Vec<u8> = levels_used.to_vec();
    used.sort_unstable();
    used.dedup();

    if used.is_empty() {
        // No headings at all: whatever arrives is one row — and that row holds the
        // whole file's prose, so it cannot be a kind that holds none.
        //
        // This used to be the starting kind verbatim, which was only ever safe
        // because every caller happened to pass a prose-bearing one. Deriving the
        // start from the destination stopped that being true: importing a headingless
        // file at the root starts at `Book`, and a Book with prose is a plan
        // `apply_document_import` is obliged to refuse — losing the import rather than
        // the paragraph. The same reasoning as the deepest-level re-anchor below, for
        // the case that has no levels to anchor.
        let kind = if start_holds_prose(start) {
            start
        } else {
            // The bottom of the ladder, which is prose-bearing by construction — the
            // same guarantee `kind_for(u8::MAX)` already leans on.
            *LADDER.last().unwrap_or(&CreateType::Scene)
        };
        return LevelRules::from_table(BTreeMap::from([(1, kind)]));
    }

    let start_index = LADDER.iter().position(|k| *k == start).unwrap_or(0);
    let mut table = BTreeMap::new();
    for (step, level) in used.iter().enumerate() {
        let kind = LADDER
            .get((start_index + step).min(LADDER.len() - 1))
            .copied()
            .unwrap_or(CreateType::Scene);
        table.insert(*level, kind);
    }

    // The deepest level is where the writing is, so it must land on something
    // that can hold prose. Walking the ladder by position alone does not
    // guarantee that: `# Book` / `## Chapter` uses two levels and lands on
    // Book/Part, and a Part holds no prose — which would produce a plan the
    // apply step is obliged to refuse, since `content_allowed` would otherwise
    // drop the text silently at the next save. Re-anchoring the last level onto
    // Chapter keeps the containment sensible (a Book of chapters) instead.
    let deepest_level = *used.last().expect("checked non-empty above");
    if !holds_prose(table[&deepest_level]) {
        table.insert(deepest_level, CreateType::Chapter);
    }

    LevelRules::from_table(table)
}

/// Whether a row of this kind can store prose at all.
///
/// Asked of `skribisto_model` rather than hardcoded, so the constraint matrix
/// stays the single authority. `ChapterMode::Folder` is passed because a Chapter
/// holds prose under either encoding — the mode changes how it is stored, never
/// whether it can be.
fn holds_prose(kind: CreateType) -> bool {
    use common::entities::ContentRole;
    let (role, sub_role) = kind.combo(common::entities::ChapterMode::Folder);
    skribisto_model::allowed_content(&role, &sub_role)
        .iter()
        .any(|r| {
            matches!(
                r,
                ContentRole::SceneText | ContentRole::NoteText | ContentRole::ParatextText
            )
        })
}

/// Every heading level used across a set of documents.
pub fn levels_used(docs: &[crate::block::SourceDocument]) -> Vec<u8> {
    let mut levels: Vec<u8> = docs
        .iter()
        .flat_map(|d| d.blocks.iter())
        .filter_map(|b| match b {
            crate::block::SourceBlock::Heading { level, .. } => Some(*level),
            _ => None,
        })
        .collect();
    levels.sort_unstable();
    levels.dedup();
    levels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_map_down_the_ladder_from_the_starting_kind() {
        let rules = infer_rules(&[1, 2, 3], CreateType::Book);
        assert_eq!(rules.kind_for(1), CreateType::Book);
        assert_eq!(rules.kind_for(2), CreateType::Part);
        assert_eq!(rules.kind_for(3), CreateType::Chapter);
    }

    /// The deepest level is where the prose is, so it must land somewhere prose
    /// can live. Two levels from Book walk the ladder to Book/Part — and a Part
    /// holds none, which would build a plan the apply step has to refuse.
    #[test]
    fn the_deepest_level_always_lands_where_prose_can_live() {
        let rules = infer_rules(&[1, 2], CreateType::Book);
        assert_eq!(rules.kind_for(1), CreateType::Book);
        assert_eq!(
            rules.kind_for(2),
            CreateType::Chapter,
            "a Book of chapters, not a Book of prose-less Parts"
        );
    }

    #[test]
    fn a_single_level_holding_prose_is_a_chapter_not_a_book() {
        assert_eq!(
            infer_rules(&[1], CreateType::Book).kind_for(1),
            CreateType::Chapter
        );
    }

    #[test]
    fn a_run_that_already_ends_in_prose_is_left_alone() {
        let rules = infer_rules(&[1, 2], CreateType::Chapter);
        assert_eq!(rules.kind_for(1), CreateType::Chapter);
        assert_eq!(rules.kind_for(2), CreateType::Scene);
    }

    /// Absolute depth means nothing — only the order levels appear in. The app's
    /// own export writes levels dense over the export's scope, so `##` is a Part
    /// in one file and a Chapter in another.
    #[test]
    fn a_document_starting_at_h2_gets_the_same_tree_as_one_starting_at_h1() {
        let from_h1 = infer_rules(&[1, 2], CreateType::Part);
        let from_h2 = infer_rules(&[2, 3], CreateType::Part);
        assert_eq!(from_h1.kind_for(1), from_h2.kind_for(2));
        assert_eq!(from_h1.kind_for(2), from_h2.kind_for(3));
    }

    #[test]
    fn the_ladder_stops_at_scene_rather_than_inventing_depth() {
        let rules = infer_rules(&[1, 2, 3, 4, 5, 6], CreateType::Book);
        assert_eq!(rules.kind_for(4), CreateType::Scene);
        assert_eq!(rules.kind_for(5), CreateType::Scene);
        assert_eq!(rules.kind_for(6), CreateType::Scene);
    }

    #[test]
    fn a_level_deeper_than_the_table_takes_the_deepest_rule() {
        let rules = infer_rules(&[1, 2], CreateType::Chapter);
        assert_eq!(rules.kind_for(2), CreateType::Scene);
        assert_eq!(rules.kind_for(5), CreateType::Scene);
    }

    #[test]
    fn no_headings_yields_one_rule_of_the_starting_kind() {
        let rules = infer_rules(&[], CreateType::Scene);
        assert_eq!(rules.kind_for(1), CreateType::Scene);
    }

    #[test]
    fn the_writer_can_retype_a_level() {
        // Three levels, so the deepest already lands on Chapter and the
        // re-anchoring in `infer_rules` leaves the middle level alone — this test
        // is about `set`, not about inference.
        let mut rules = infer_rules(&[1, 2, 3], CreateType::Book);
        rules.set(1, CreateType::Chapter);
        assert_eq!(rules.kind_for(1), CreateType::Chapter);
        assert_eq!(
            rules.kind_for(2),
            CreateType::Part,
            "other levels untouched"
        );
    }
}

#[cfg(test)]
mod start_kind_tests {
    use super::*;

    /// Inside a container, the import starts one rung deeper — the rule
    /// `infer_rules`' own doc states.
    #[test]
    fn landing_inside_a_container_steps_one_rung_down() {
        assert_eq!(
            start_kind_at(CreateType::Book, true),
            Some(CreateType::Part)
        );
        assert_eq!(
            start_kind_at(CreateType::Part, true),
            Some(CreateType::Chapter)
        );
        assert_eq!(
            start_kind_at(CreateType::Chapter, true),
            Some(CreateType::Scene)
        );
    }

    /// …and stops at the bottom rather than inventing a level below Scene.
    #[test]
    fn the_ladder_has_a_floor() {
        assert_eq!(
            start_kind_at(CreateType::Scene, true),
            Some(CreateType::Scene)
        );
    }

    /// Beside a row means a sibling of it, at its own rung.
    #[test]
    fn landing_beside_a_row_matches_it() {
        for kind in [
            CreateType::Book,
            CreateType::Part,
            CreateType::Chapter,
            CreateType::Scene,
        ] {
            assert_eq!(start_kind_at(kind, false), Some(kind));
        }
    }

    /// A destination the ladder does not describe gets no answer, not a plausible one.
    #[test]
    fn an_off_ladder_destination_has_no_rung() {
        for kind in [
            CreateType::Note,
            CreateType::NoteFolder,
            CreateType::Folder,
            CreateType::Paratext,
            CreateType::ParatextFolder,
            CreateType::EndOfBook,
        ] {
            assert_eq!(start_kind_at(kind, true), None, "for {kind:?}");
            assert_eq!(start_kind_at(kind, false), None, "for {kind:?}");
        }
    }
}

#[cfg(test)]
mod headingless_start_tests {
    use super::*;

    /// **Regression.** A file with no headings is one row holding all its prose, so
    /// that row must be able to hold prose — whatever the destination started at.
    ///
    /// Importing a headingless `.md` at the root starts the ladder at `Book`, and a
    /// Book with prose is a plan `apply_document_import` refuses outright: the writer
    /// loses the whole import, not the paragraph. It went unseen while every caller
    /// passed a prose-bearing start by hand.
    #[test]
    fn a_headingless_import_never_lands_on_a_kind_that_holds_no_prose() {
        for start in [
            CreateType::Book,
            CreateType::Part,
            CreateType::Folder,
            CreateType::NoteFolder,
            CreateType::ParatextFolder,
        ] {
            let rules = infer_rules(&[], start);
            let kind = rules.kind_for(u8::MAX);
            assert!(
                start_holds_prose(kind),
                "starting at {start:?} produced {kind:?}, which holds no prose"
            );
        }
    }

    /// A start that already holds prose is left exactly as it is.
    #[test]
    fn a_prose_bearing_start_is_kept() {
        for start in [CreateType::Scene, CreateType::Chapter, CreateType::Note] {
            assert_eq!(infer_rules(&[], start).kind_for(u8::MAX), start);
        }
    }
}
