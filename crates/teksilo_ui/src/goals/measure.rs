// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! "How many manuscript words are under this item?" — asked once, answered here.
//!
//! Four surfaces need that number (the Inspector's readout, a per-item milestone's
//! progress, the Book's informational target total, and the weights a
//! [`distribute`](super::distribute) uses), and the app has a history of surfaces each
//! deriving their own and quietly disagreeing. This is the one walk.
//!
//! **The admission gate is `counts_prose && activated && is_exportable`, per item, with no
//! cascade** — byte for byte what `progress_management::count_words_uc::fold_counts` uses,
//! and what `skribisto_model::compile::push_swept` uses when resolving an export scope. A
//! non-exportable *chapter folder* does not exclude the scenes inside it; each row answers
//! for itself. That is exactly why the Inspector ships an "Apply to children" button beside
//! the toggle: exclusion is pushed down by hand, never inferred. Getting this wrong would
//! make a goal bar disagree with the export it is supposed to be measuring.
//!
//! **Counting method.** The caller passes the method its own surface displays with, so a
//! bar never contradicts the number printed beside it. That is deliberately *not* the
//! always-`Auto` discipline `ProgressSnapshot` uses: a persisted historical series must not
//! be re-based when a display preference changes, but a live readout must agree with its
//! own label.
//!
//! Read-only, one batched pass, no reactive state — call it from a view-model or an effect,
//! not from a paint. Two `mod imp` variants share one surface, like every other Layer A
//! reader; the mock's numbers line up with the mock binder fixture so the mocks build reads
//! naturally.

use frontend::common::entities::GoalUnit;

/// A length, in both units at once.
///
/// Both are always computed — the counter produces them together and characters cost
/// nothing extra — so a surface can render the unit its project asked for without a second
/// walk, and switching the project's unit needs no recount.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub words: usize,
    /// Characters **including spaces**: every character convention that is an industry
    /// standard counts them (a German Normseite is 1500 *inkl. Leerzeichen*, a French
    /// feuillet 1500 *signes espaces compris*, a 原稿用紙 sheet 400 squares including the
    /// blank ones). Characters-without-spaces stays an informational stat elsewhere and is
    /// never a target unit.
    pub chars: usize,
}

impl Counts {
    /// The number this project's unit asks for.
    pub fn by(&self, unit: &GoalUnit) -> usize {
        match unit {
            GoalUnit::Words => self.words,
            GoalUnit::Characters => self.chars,
        }
    }
}

/// What one item and its subtree measure.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Measured {
    /// This row's *own* scene prose, or `None` when its `(role, sub_role)` carries none.
    ///
    /// The same `Option` contract the Overview column uses: `None` is "no prose slot
    /// here", `Some(0)` is "prose-bearing and empty". Unaffected by `is_exportable` — an
    /// excluded scene still has a length, and hiding it would make this answer the wrong
    /// question.
    pub own: Option<Counts>,
    /// This row plus its whole subtree, under the manuscript gate above. What a container's
    /// target is measured against.
    pub subtree: Counts,
    /// Sum of the **descendants'** targets (never the row's own), across manuscript rows
    /// only, in the unit the measurement was asked for.
    ///
    /// Purely informational: this app never derives a container's target from its children
    /// (see [`crate::goals`]). It exists so a writer who set thirty chapter targets can see
    /// what they add up to, without that sum ever pretending to be a target.
    pub descendant_goal_total: i64,
    /// How many descendants contributed to `descendant_goal_total`.
    pub descendant_goal_items: usize,
}

/// One immediate child a target can be distributed to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Child {
    pub id: u64,
    pub title: String,
    /// Its current target in the project's unit (`0` = none).
    pub goal: i64,
    /// Manuscript length under it, the default distribution weight.
    pub subtree: Counts,
    /// Manuscript rows under it (including itself), the fallback weight when nothing is
    /// written yet.
    pub subtree_rows: usize,
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::collections::HashMap;

    use frontend::AppContext;
    use frontend::commands::{
        binder_commands, binder_item_commands, content_commands, work_commands,
    };
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::{ContentRole, GoalUnit};
    use frontend::direct_access::BinderItemDto;

    use skribisto_model::counting::{self, CountMethod, CountingMethodSetting};
    use skribisto_model::{compile, counts_prose, language};

    use super::{Child, Counts, Measured};

    /// Every binder item of `work_id`, binder-major, in each binder's stored relationship
    /// order, paired with its owning binder.
    ///
    /// **Trashed rows are kept.** Filtering them before the indent walk would let a
    /// subtree run past a trashed container into rows that are not its descendants; they
    /// are dropped at counting time instead. Trashing does cascade today
    /// (`trash_binder_items_uc` trashes the whole contiguous subtree), so this is belt and
    /// braces — but it costs one predicate and removes a whole class of wrong number.
    fn flat_items(ctx: &AppContext, work_id: u64) -> Vec<(u64, BinderItemDto)> {
        let mut out = Vec::new();
        let binder_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();
        for binder_id in binder_ids {
            let item_ids = binder_item_ids(ctx, binder_id);
            let by_id: HashMap<u64, BinderItemDto> =
                binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                    .map(|it| (it.id, it))
                    .collect();
            for id in item_ids {
                if let Some(it) = by_id.get(&id) {
                    out.push((binder_id, it.clone()));
                }
            }
        }
        out
    }

    fn binder_item_ids(ctx: &AppContext, binder_id: u64) -> Vec<u64> {
        binder_commands::get_binder_relationship(
            ctx,
            &binder_id,
            &BinderRelationshipField::BinderItems,
        )
        .unwrap_or_default()
    }

    /// The contiguous descendants of `item_id`: every following row deeper than it,
    /// stopping at the first row back at its level or shallower — **or at the first row of
    /// a different binder**, because indents are scoped per binder and the flat stream has
    /// no separator between them.
    fn descendants(
        flat: &[(u64, BinderItemDto)],
        item_id: u64,
    ) -> Option<(&BinderItemDto, &[(u64, BinderItemDto)])> {
        let pos = flat.iter().position(|(_, it)| it.id == item_id)?;
        let (binder, root) = (&flat[pos].0, &flat[pos].1);
        let after = &flat[pos + 1..];
        let end = after
            .iter()
            .position(|(b, it)| it.indent <= root.indent || b != binder)
            .unwrap_or(after.len());
        Some((root, &after[..end]))
    }

    /// Does this row's prose count toward a manuscript total? Per item, never inherited.
    fn counted(it: &BinderItemDto) -> bool {
        it.activated && it.is_exportable && counts_prose(&it.role, &it.sub_role)
    }

    /// Scene prose per item, in one batched read for the whole slice.
    fn scene_text(ctx: &AppContext, items: &[&BinderItemDto]) -> HashMap<u64, String> {
        let mut owner: HashMap<u64, u64> = HashMap::new();
        let mut ids: Vec<u64> = Vec::new();
        for it in items {
            for cid in &it.contents {
                owner.insert(*cid, it.id);
                ids.push(*cid);
            }
        }
        let mut out = HashMap::new();
        for c in content_commands::get_content_multi(ctx, &ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
        {
            if c.role == ContentRole::SceneText
                && let Some(item) = owner.get(&c.id)
            {
                out.insert(*item, c.data);
            }
        }
        out
    }

    fn counts_of(
        prose: &HashMap<u64, String>,
        it: &BinderItemDto,
        method: CountingMethodSetting,
    ) -> Counts {
        let m = counting::resolve_method(
            method,
            CountMethod::UnicodeWords,
            language::primary(&it.dict_language),
        );
        prose.get(&it.id).map_or(Counts::default(), |d| {
            // Content-addressed cache, so a subtree walked twice in a row (a focus change
            // and back) re-counts nothing.
            let c = counting::cached_count(d, m);
            Counts {
                words: c.words,
                chars: c.chars_with_spaces,
            }
        })
    }

    /// This item's target in `unit` — one of the two long-standing fields, chosen by the
    /// project's own setting and never converted into the other.
    fn goal_in(it: &BinderItemDto, unit: &GoalUnit) -> i64 {
        match unit {
            GoalUnit::Words => it.word_count_goal,
            GoalUnit::Characters => it.char_count_goal,
        }
    }

    pub fn measure(
        ctx: &AppContext,
        work_id: u64,
        item_id: u64,
        method: CountingMethodSetting,
        unit: &GoalUnit,
    ) -> Option<Measured> {
        let flat = flat_items(ctx, work_id);
        let (root, kids) = descendants(&flat, item_id)?;

        let mut prose_rows: Vec<&BinderItemDto> = Vec::new();
        if counts_prose(&root.role, &root.sub_role) {
            prose_rows.push(root);
        }
        for (_, it) in kids {
            if counted(it) {
                prose_rows.push(it);
            }
        }
        let prose = scene_text(ctx, &prose_rows);

        let own = counts_prose(&root.role, &root.sub_role).then(|| counts_of(&prose, root, method));
        let mut subtree = if counted(root) {
            own.unwrap_or_default()
        } else {
            Counts::default()
        };
        let mut descendant_goal_total = 0i64;
        let mut descendant_goal_items = 0usize;
        for (_, it) in kids {
            if counted(it) {
                let c = counts_of(&prose, it, method);
                subtree.words += c.words;
                subtree.chars += c.chars;
            }
            // Informational only, and restricted to manuscript rows: a note's target is
            // real, but summing it under a *book's* target would answer a question nobody
            // asked. `activated` still gates it — a trashed chapter's target is not part
            // of the plan.
            let goal = goal_in(it, unit);
            if it.activated && goal > 0 && compile::is_row(&it.sub_role) {
                descendant_goal_total += goal;
                descendant_goal_items += 1;
            }
        }
        Some(Measured {
            own,
            subtree,
            descendant_goal_total,
            descendant_goal_items,
        })
    }

    pub fn children(
        ctx: &AppContext,
        work_id: u64,
        item_id: u64,
        method: CountingMethodSetting,
        unit: &GoalUnit,
    ) -> Vec<Child> {
        let flat = flat_items(ctx, work_id);
        let Some((root, kids)) = descendants(&flat, item_id) else {
            return Vec::new();
        };
        let depth = root.indent + 1;

        let prose_rows: Vec<&BinderItemDto> = kids
            .iter()
            .map(|(_, it)| it)
            .filter(|it| counted(it))
            .collect();
        let prose = scene_text(ctx, &prose_rows);

        let mut out: Vec<Child> = Vec::new();
        for (i, (_, it)) in kids.iter().enumerate() {
            if it.indent != depth || !it.activated || !compile::is_row(&it.sub_role) {
                continue;
            }
            // This child's own contiguous run, found the same way as the parent's.
            let end = kids[i + 1..]
                .iter()
                .position(|(_, x)| x.indent <= it.indent)
                .map_or(kids.len(), |n| i + 1 + n);
            let mut subtree = Counts::default();
            let mut subtree_rows = 0usize;
            for (_, row) in &kids[i..end] {
                if counted(row) {
                    let c = counts_of(&prose, row, method);
                    subtree.words += c.words;
                    subtree.chars += c.chars;
                }
                if row.activated && compile::is_row(&row.sub_role) {
                    subtree_rows += 1;
                }
            }
            out.push(Child {
                id: it.id,
                title: it.title.clone(),
                goal: goal_in(it, unit),
                subtree,
                subtree_rows: subtree_rows.max(1),
            });
        }
        out
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use frontend::AppContext;
    use frontend::common::entities::GoalUnit;
    use skribisto_model::counting::CountingMethodSetting;

    use super::{Child, Counts, Measured};

    /// Roughly five characters per word plus a space, which is what English prose runs at.
    /// Good enough for a fabricated fixture, and it keeps the mocks build's two units in a
    /// believable relationship instead of showing the same number twice.
    fn counts(words: usize) -> Counts {
        Counts {
            words,
            chars: words * 6,
        }
    }

    /// Fabricated to agree with the mock binder fixture in
    /// `models::overview_rows_model` — Book One (101) holds Part One (301) and the loose
    /// chapters, Part One holds chapter folder 104 and flat chapter 302, and 104 holds
    /// scenes 201/202 plus note 203. Keeping the two tables in step is what lets the mocks
    /// build show a coherent set of numbers across the Overview, the Inspector and the
    /// status bar at once.
    pub fn measure(
        _ctx: &AppContext,
        _work_id: u64,
        item_id: u64,
        _method: CountingMethodSetting,
        unit: &GoalUnit,
    ) -> Option<Measured> {
        let (own, subtree, goal_total, goal_items) = match item_id {
            101 => (None, 4879, 22_000, 3),    // Book One
            301 => (None, 2665, 12_000, 2),    // Part One
            104 => (Some(90), 1910, 4_000, 2), // Chapter Two (a folder with its own prose)
            201 => (Some(1180), 1180, 0, 0),
            202 => (Some(640), 640, 0, 0),
            203 => (None, 0, 0, 0), // a note carries no manuscript prose
            302 => (Some(755), 755, 0, 0),
            303 => (Some(300), 300, 0, 0),
            105 => (Some(1502), 1502, 0, 0),
            103 => (Some(412), 412, 0, 0),
            _ => (None, 0, 0, 0),
        };
        // The fixture's targets are word figures; a character-unit project's fabricated
        // targets scale with the fabricated prose so the bars still read sensibly.
        let scale = match unit {
            GoalUnit::Words => 1,
            GoalUnit::Characters => 6,
        };
        Some(Measured {
            own: own.map(counts),
            subtree: counts(subtree),
            descendant_goal_total: goal_total * scale,
            descendant_goal_items: goal_items,
        })
    }

    pub fn children(
        _ctx: &AppContext,
        _work_id: u64,
        item_id: u64,
        _method: CountingMethodSetting,
        unit: &GoalUnit,
    ) -> Vec<Child> {
        let rows: &[(u64, &str, i64, usize, usize)] = match item_id {
            101 => &[
                (103, "Scene at dawn", 0, 412, 1),
                (301, "Part One — Arrival", 12_000, 2665, 5),
                (303, "The light returns", 0, 300, 1),
                (105, "Confrontation", 10_000, 1502, 1),
            ],
            301 => &[
                (104, "Chapter Two", 4_000, 1910, 3),
                (302, "Into the Dark", 0, 755, 1),
            ],
            104 => &[
                (201, "Scene 1", 2_000, 1180, 1),
                (202, "Scene 2", 2_000, 640, 1),
            ],
            _ => &[],
        };
        let scale = match unit {
            GoalUnit::Words => 1,
            GoalUnit::Characters => 6,
        };
        rows.iter()
            .map(|(id, title, goal, words, count)| Child {
                id: *id,
                title: (*title).to_string(),
                goal: *goal * scale,
                subtree: counts(*words),
                subtree_rows: *count,
            })
            .collect()
    }
}

pub use imp::{children, measure};
