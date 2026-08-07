// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Which heading depth means which kind of row.
//!
//! ## Why this is deliberately simple
//!
//! Every surveyed tool that guesses hard at structure — calibre's eight-pattern
//! regex cascade, its word-count sizing, Vellum's keyword vocabulary — is
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
/// a phantom row — the failure md2nw is documented to have, where a heading
/// deeper than the tool understood silently opened an unnamed scene whose body
/// began with the literal `#### `.
const LADDER: &[CreateType] = &[
    CreateType::Book,
    CreateType::Part,
    CreateType::Chapter,
    CreateType::Scene,
];

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
        // No headings at all: whatever arrives is one row of the starting kind.
        return LevelRules::from_table(BTreeMap::from([(1, start)]));
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
