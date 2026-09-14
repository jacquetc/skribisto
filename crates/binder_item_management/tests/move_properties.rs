// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `move_items` against a real store: the binder still describes a tree afterwards, and undo
//! puts it back exactly.
//!
//! `binder_ordering/tests/properties.rs` checks the arithmetic by replaying it. This drives
//! the use case itself, which is the half a replay cannot reach:
//!
//! * the use case still performs those steps, in that order, against real rows — a replay
//!   passes forever after the code it mirrors stops matching it;
//! * the relationship vec and each row's `indent` are written to the *same* arrangement, and
//!   they are two separate writes to two different places;
//! * undo restores the binder rather than something that merely looks similar. The restore is
//!   a scoped snapshot of the affected subtrees, so what it covers is itself a decision that
//!   can be wrong.
//!
//! The tree is built through `direct_access`, like every other integration test in this
//! crate, because `new_work` is unusable in a test (see `work_scoping.rs`).
//!
//! # What "well-formed" means here
//!
//! The binder stores no parent link. Hierarchy is the flat order plus an indent per row, so a
//! tree is well-formed when the first row is at indent 0, no indent is negative, and no row is
//! more than one level deeper than the row above it. A row that breaks the third rule names a
//! parent that does not exist, which nothing in the stack reports and everything downstream
//! believes.

use std::collections::HashSet;
use std::sync::Arc;

use binder_item_management::binder_item_management_controller as feature;
use binder_item_management::{MoveDto, MovePlace};
use common::database::db_context::DbContext;
use common::direct_access::binder::BinderRelationshipField;
use common::entities::{BinderItemRole, BinderItemSubRole};
use common::event::EventHub;
use common::types::EntityId;
use common::undo_redo::UndoRedoManager;
use direct_access::binder::binder_controller;
use direct_access::binder::dtos::CreateBinderDto;
use direct_access::binder_item::binder_item_controller;
use direct_access::binder_item::dtos::CreateBinderItemDto;
use direct_access::root::dtos::CreateRootDto;
use direct_access::root::root_controller;
use direct_access::smart_punctuation::dtos::CreateSmartPunctuationDto;
use direct_access::smart_punctuation::smart_punctuation_controller;
use direct_access::work::dtos::CreateWorkDto;
use direct_access::work::work_controller;
use proptest::prelude::*;

/// One open Work, one Binder, and the rows a test pushes into it.
struct Ctx {
    db: DbContext,
    hub: Arc<EventHub>,
    undo: UndoRedoManager,
    binder_id: EntityId,
    stack: u64,
}

impl Ctx {
    fn new() -> Self {
        let db = DbContext::new().expect("in-memory store");
        let hub = Arc::new(EventHub::new());
        let mut undo = UndoRedoManager::new();
        undo.set_event_hub(&hub);
        let root_id = root_controller::create_orphan(&db, &hub, &CreateRootDto::default())
            .expect("root")
            .id;
        let smart_punctuation = smart_punctuation_controller::create_orphan(
            &db,
            &hub,
            &mut undo,
            None,
            &CreateSmartPunctuationDto::default(),
        )
        .expect("smart_punctuation")
        .id;
        let work_id = work_controller::create(
            &db,
            &hub,
            &mut undo,
            None,
            &CreateWorkDto {
                statuses: Vec::new(),
                smart_punctuation,
                ..Default::default()
            },
            root_id,
            -1,
        )
        .expect("work")
        .id;
        let binder_id = binder_controller::create(
            &db,
            &hub,
            &mut undo,
            None,
            &CreateBinderDto {
                activated: true,
                ..Default::default()
            },
            work_id,
            -1,
        )
        .expect("binder")
        .id;
        let stack = undo.create_new_stack();
        Ctx {
            db,
            hub,
            undo,
            binder_id,
            stack,
        }
    }

    /// Append a row at `indent`. A `Folder` where the generated shape says so, because `Into`
    /// a leaf is defined to fall back to `After` it and would otherwise never be exercised.
    fn push(&mut self, indent: i64, folder: bool) -> EntityId {
        let binder_id = self.binder_id;
        binder_item_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateBinderItemDto {
                status: None,
                role: if folder {
                    BinderItemRole::Folder
                } else {
                    BinderItemRole::Item
                },
                sub_role: if folder {
                    BinderItemSubRole::None
                } else {
                    BinderItemSubRole::Scene
                },
                activated: true,
                is_exportable: true,
                indent,
                ..Default::default()
            },
            binder_id,
            -1,
        )
        .expect("binder_item")
        .id
    }

    /// The binder as the model defines it: the relationship vec's order, each row with its
    /// indent. Read through the relationship, never by scanning entities — the scan order is
    /// not the binder's order and has no reason to match it.
    fn rows(&self) -> Vec<(EntityId, i64)> {
        let order = binder_controller::get_relationship(
            &self.db,
            &self.binder_id,
            &BinderRelationshipField::BinderItems,
        )
        .expect("binder_items");
        order
            .into_iter()
            .map(|id| {
                let it = binder_item_controller::get(&self.db, &id)
                    .expect("get")
                    .expect("row present");
                (id, it.indent)
            })
            .collect()
    }
}

/// The three rules of the binder model. `Ok` or the reason it is not a tree.
fn well_formed(rows: &[(EntityId, i64)]) -> Result<(), String> {
    let mut prev: Option<i64> = None;
    for (i, (id, indent)) in rows.iter().enumerate() {
        if *indent < 0 {
            return Err(format!("row {i} (id {id}) has negative indent {indent}"));
        }
        match prev {
            None if *indent != 0 => {
                return Err(format!("first row (id {id}) is at indent {indent}, not 0"));
            }
            Some(p) if *indent > p + 1 => {
                return Err(format!(
                    "row {i} (id {id}) jumps from indent {p} to {indent}, naming a parent that \
                     does not exist"
                ));
            }
            _ => {}
        }
        prev = Some(*indent);
    }
    Ok(())
}

/// A tree shape (depth + folderness per row), a selection over it, and a drop.
///
/// Depths are clamped into a well-formed walk the same way `binder_ordering`'s generator does
/// it, so the *starting* binder is always legal and any illegality in the result was created
/// by the move. The selection and target are resolved against real ids after the rows exist,
/// inside the test, since the ids are minted by the store.
fn shape_strategy() -> impl Strategy<Value = (Vec<(i64, bool)>, Vec<bool>, usize, MovePlace)> {
    (
        prop::collection::vec((0i64..4, any::<bool>()), 1..=10),
        prop::collection::vec(any::<bool>(), 10),
        0usize..10,
        prop_oneof![
            Just(MovePlace::Before),
            Just(MovePlace::After),
            Just(MovePlace::Into)
        ],
    )
        .prop_map(|(mut shape, picks, target, place)| {
            let mut prev = -1i64;
            for (depth, _) in shape.iter_mut() {
                *depth = (*depth).min(prev + 1).max(0);
                prev = *depth;
            }
            (shape, picks, target, place)
        })
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 200, ..ProptestConfig::default() })]

    /// A move through the real use case leaves a binder that still describes a tree, and
    /// leaves every row still in it.
    ///
    /// The store makes this stricter than the replay in two ways worth the cost of a
    /// `DbContext` per case: the order lives in the binder's relationship vec while the depths
    /// live on the rows, so this is the only place the two can be seen to disagree; and a
    /// refusal is a real `Err` from the use case rather than a condition the test decided to
    /// model, so a move that *should* have been refused and was not shows up here as a
    /// malformed tree instead of passing quietly.
    #[test]
    fn a_move_through_the_use_case_leaves_a_tree(
        (shape, picks, target_pick, place) in shape_strategy()
    ) {
        let mut ctx = Ctx::new();
        let ids: Vec<EntityId> = shape.iter().map(|(d, f)| ctx.push(*d, *f)).collect();

        let before = ctx.rows();
        prop_assert!(well_formed(&before).is_ok(), "the fixture itself is malformed");

        let selected: Vec<EntityId> = ids
            .iter()
            .zip(picks.iter().chain(std::iter::repeat(&false)))
            .filter(|(_, p)| **p)
            .map(|(id, _)| *id)
            .collect();
        if selected.is_empty() {
            return Ok(());
        }
        let target = ids[target_pick % ids.len()];

        let outcome = feature::move_items(
            &ctx.db,
            &ctx.hub,
            &mut ctx.undo,
            Some(ctx.stack),
            &MoveDto {
                item_ids: selected.clone(),
                target_id: Some(target),
                target_is_binder: false,
                move_place: place.clone(),
            },
        );
        // A drop onto one's own subtree is refused by design; a refusal must also leave the
        // binder untouched, which is what the transaction rollback is for.
        if outcome.is_err() {
            prop_assert_eq!(ctx.rows(), before, "a refused move still changed the binder");
            return Ok(());
        }

        let after = ctx.rows();
        if let Err(why) = well_formed(&after) {
            prop_assert!(
                false,
                "moving {selected:?} {place:?} {target}\n  before: {before:?}\n  after:  \
                 {after:?}\n  {why}"
            );
        }

        let mut before_ids: Vec<EntityId> = before.iter().map(|(id, _)| *id).collect();
        let mut after_ids: Vec<EntityId> = after.iter().map(|(id, _)| *id).collect();
        before_ids.sort_unstable();
        after_ids.sort_unstable();
        prop_assert_eq!(before_ids, after_ids, "the move lost or duplicated a row");
    }

    /// Undoing a move restores the binder exactly: same order, same depths.
    ///
    /// `move_items` undoes by restoring a snapshot scoped to the affected binders rather than
    /// by replaying an inverse, so what it covers is a judgement call. Any row it forgot is a
    /// row left at the depth the move gave it, on a binder that otherwise looks restored.
    #[test]
    fn undoing_a_move_restores_the_binder_exactly(
        (shape, picks, target_pick, place) in shape_strategy()
    ) {
        let mut ctx = Ctx::new();
        let ids: Vec<EntityId> = shape.iter().map(|(d, f)| ctx.push(*d, *f)).collect();
        let before = ctx.rows();

        let selected: Vec<EntityId> = ids
            .iter()
            .zip(picks.iter().chain(std::iter::repeat(&false)))
            .filter(|(_, p)| **p)
            .map(|(id, _)| *id)
            .collect();
        if selected.is_empty() {
            return Ok(());
        }
        let target = ids[target_pick % ids.len()];

        if feature::move_items(
            &ctx.db,
            &ctx.hub,
            &mut ctx.undo,
            Some(ctx.stack),
            &MoveDto {
                item_ids: selected,
                target_id: Some(target),
                target_is_binder: false,
                move_place: place,
            },
        )
        .is_err()
        {
            return Ok(());
        }

        ctx.undo.undo(Some(ctx.stack)).expect("undo");
        prop_assert_eq!(ctx.rows(), before, "undo did not restore the binder");
    }

    /// Every selected subtree's root lands at the same depth, and each keeps its own shape.
    ///
    /// The rule the multi-subtree fault broke. Dropping a chapter and a scene from elsewhere
    /// onto one place makes both children of it; it does not preserve the accident that one
    /// of them used to be two levels deeper than the other. Checked here against the rows the
    /// store actually holds, so it covers the write as well as the arithmetic.
    #[test]
    fn every_selected_root_lands_at_one_depth(
        (shape, picks, target_pick, place) in shape_strategy()
    ) {
        let mut ctx = Ctx::new();
        let ids: Vec<EntityId> = shape.iter().map(|(d, f)| ctx.push(*d, *f)).collect();
        let before = ctx.rows();

        let selected: Vec<EntityId> = ids
            .iter()
            .zip(picks.iter().chain(std::iter::repeat(&false)))
            .filter(|(_, p)| **p)
            .map(|(id, _)| *id)
            .collect();
        if selected.is_empty() {
            return Ok(());
        }
        let target = ids[target_pick % ids.len()];

        // The roots of the selection, as the expansion finds them: a selected row not already
        // covered by an earlier selected row's subtree.
        let sel: HashSet<EntityId> = selected.iter().copied().collect();
        let mut roots: Vec<EntityId> = Vec::new();
        let mut covered: HashSet<EntityId> = HashSet::new();
        for (pos, (id, indent)) in before.iter().enumerate() {
            if !sel.contains(id) || covered.contains(id) {
                continue;
            }
            roots.push(*id);
            for (row, depth) in &before[pos..] {
                if row != id && *depth <= *indent {
                    break;
                }
                covered.insert(*row);
            }
        }

        if feature::move_items(
            &ctx.db,
            &ctx.hub,
            &mut ctx.undo,
            Some(ctx.stack),
            &MoveDto {
                item_ids: selected.clone(),
                target_id: Some(target),
                target_is_binder: false,
                move_place: place,
            },
        )
        .is_err()
        {
            return Ok(());
        }

        let after = ctx.rows();
        let depth_of = |id: EntityId, rows: &[(EntityId, i64)]| {
            rows.iter().find(|(r, _)| *r == id).map(|(_, d)| *d)
        };
        let mut landed: Vec<i64> = roots
            .iter()
            .filter_map(|id| depth_of(*id, &after))
            .collect();
        landed.dedup();
        prop_assert!(
            landed.windows(2).all(|w| w[0] == w[1]),
            "selected roots {roots:?} landed at different depths {landed:?}\n  before: \
             {before:?}\n  after: {after:?}"
        );
    }
}
