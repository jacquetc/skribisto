// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `set_descendants_exportable` and `set_descendants_dict_language` — the two
//! "apply to children" writes, moved out of the outline view-model so a binder
//! screen that is not that outline can make the same gesture.
//!
//! What these tests pin is the part the move was *for*: the subtree walk. A
//! binder has no parent/child graph, so "beneath X" is positional — X plus the
//! following rows at a strictly greater indent — and every interesting failure
//! is a boundary in that walk rather than in the write itself. The fixture is
//! therefore shaped to have edges on both sides: a sibling after the subtree, a
//! deeper grandchild inside it, and a row that already holds the target value.

use std::sync::Arc;

use binder_item_management::binder_item_management_controller as feature;
use binder_item_management::{SetDescendantsDictLanguageDto, SetDescendantsExportableDto};
use common::database::db_context::DbContext;
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

struct Ctx {
    db: DbContext,
    hub: Arc<EventHub>,
    undo: UndoRedoManager,
    binder_id: EntityId,
    /// The feature calls' own undo stack.
    ///
    /// Separate from the default stack 0, which every `direct_access` write in
    /// this file lands on, so a test can undo the use case *without* first
    /// undoing whatever it staged afterwards — the stacks are LIFO, and
    /// `undo_follows_the_rows_it_wrote_…` depends on that staging surviving.
    stack: u64,
}

impl Ctx {
    /// One open Work with one Binder, built straight through `direct_access`
    /// (see `work_scoping.rs` for why `new_work` is unusable in a test).
    fn new() -> Self {
        let db = DbContext::new().expect("in-memory store");
        let hub = Arc::new(EventHub::new());
        let mut undo = UndoRedoManager::new();
        undo.set_event_hub(&hub);
        let root_id = root_controller::create_orphan(&db, &hub, &CreateRootDto::default())
            .expect("root")
            .id;
        let smart_punctuation_id = smart_punctuation_controller::create_orphan(
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
                smart_punctuation: smart_punctuation_id,
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

    /// Append a row at `indent`, starting exportable unless told otherwise.
    fn push(&mut self, indent: i64, exportable: bool) -> EntityId {
        let binder_id = self.binder_id;
        binder_item_controller::create(
            &self.db,
            &self.hub,
            &mut self.undo,
            None,
            &CreateBinderItemDto {
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Scene,
                activated: true,
                is_exportable: exportable,
                indent,
                ..Default::default()
            },
            binder_id,
            -1,
        )
        .expect("binder_item")
        .id
    }

    fn exportable(&self, id: EntityId) -> bool {
        binder_item_controller::get(&self.db, &id)
            .expect("get")
            .expect("row still present")
            .is_exportable
    }

    fn language(&self, id: EntityId) -> Vec<String> {
        binder_item_controller::get(&self.db, &id)
            .expect("get")
            .expect("row still present")
            .dict_language
    }

    fn apply_language(&mut self, item_id: EntityId, tags: &[&str]) -> Vec<EntityId> {
        let stack = Some(self.stack);
        feature::set_descendants_dict_language(
            &self.db,
            &self.hub,
            &mut self.undo,
            stack,
            &SetDescendantsDictLanguageDto {
                item_id,
                tags: tags.iter().map(|s| s.to_string()).collect(),
            },
        )
        .expect("set_descendants_dict_language")
        .changed_ids
    }

    fn apply(&mut self, item_id: EntityId, exportable: bool) -> Vec<EntityId> {
        let stack = Some(self.stack);
        feature::set_descendants_exportable(
            &self.db,
            &self.hub,
            &mut self.undo,
            stack,
            &SetDescendantsExportableDto {
                item_id,
                exportable,
            },
        )
        .expect("set_descendants_exportable")
        .changed_ids
    }
}

/// ```text
/// before      indent 0   (a sibling ahead of the subtree)
/// root        indent 0
///   child_a   indent 1
///     grand   indent 2   (already false — the "no write needed" case)
///   child_b   indent 1
/// after       indent 0   (the row the walk must stop at)
/// ```
struct Tree {
    before: EntityId,
    root: EntityId,
    child_a: EntityId,
    grand: EntityId,
    child_b: EntityId,
    after: EntityId,
}

fn tree(ctx: &mut Ctx) -> Tree {
    Tree {
        before: ctx.push(0, true),
        root: ctx.push(0, true),
        child_a: ctx.push(1, true),
        grand: ctx.push(2, false),
        child_b: ctx.push(1, true),
        after: ctx.push(0, true),
    }
}

#[test]
fn writes_every_descendant_and_leaves_the_root_alone() {
    let mut ctx = Ctx::new();
    let t = tree(&mut ctx);

    ctx.apply(t.root, false);

    assert!(
        ctx.exportable(t.root),
        "the root carries its own toggle; this gesture is the one beside it"
    );
    assert!(!ctx.exportable(t.child_a));
    assert!(!ctx.exportable(t.grand), "depth is not a boundary");
    assert!(!ctx.exportable(t.child_b));
}

#[test]
fn the_walk_stops_at_the_first_row_back_up_the_indent() {
    let mut ctx = Ctx::new();
    let t = tree(&mut ctx);

    ctx.apply(t.root, false);

    assert!(
        ctx.exportable(t.after),
        "`after` is at the root's own indent, so the subtree ended before it"
    );
    assert!(
        ctx.exportable(t.before),
        "the walk runs forward from the root and must never reach behind it"
    );
}

#[test]
fn changed_ids_reports_the_rows_actually_written() {
    let mut ctx = Ctx::new();
    let t = tree(&mut ctx);

    // `grand` is already false, so turning the subtree off cannot write it.
    let changed = ctx.apply(t.root, false);
    assert_eq!(
        changed,
        vec![t.child_a, t.child_b],
        "in document order, and excluding the row that already agreed"
    );

    // Nothing left to do — an empty report is a success, not a failure.
    assert!(ctx.apply(t.root, false).is_empty());
}

#[test]
fn undo_restores_and_redo_reapplies() {
    let mut ctx = Ctx::new();
    let t = tree(&mut ctx);
    ctx.apply(t.root, false);

    ctx.undo.undo(Some(ctx.stack)).expect("undo");
    assert!(ctx.exportable(t.child_a));
    assert!(ctx.exportable(t.child_b));
    assert!(
        !ctx.exportable(t.grand),
        "`grand` was already false and was never written, so undo must not turn it on"
    );

    ctx.undo.redo(Some(ctx.stack)).expect("redo");
    assert!(!ctx.exportable(t.child_a));
    assert!(!ctx.exportable(t.child_b));
}

#[test]
fn undo_follows_the_rows_it_wrote_even_after_one_leaves_the_subtree() {
    let mut ctx = Ctx::new();
    let t = tree(&mut ctx);
    ctx.apply(t.root, false);

    // Outdent `child_b` to the root's own level: it is no longer beneath the
    // root, but this step still turned it off and still owns putting it back.
    let mut moved = binder_item_controller::get(&ctx.db, &t.child_b)
        .unwrap()
        .unwrap();
    moved.indent = 0;
    binder_item_controller::update(&ctx.db, &ctx.hub, &mut ctx.undo, None, &moved.into())
        .expect("outdent");

    // The outdent stays in place — it went to stack 0, and only the feature's
    // own stack is unwound here. Were the inverse a fresh subtree walk, it
    // would no longer find `child_b` beneath the root and would leave it off.
    ctx.undo
        .undo(Some(ctx.stack))
        .expect("undo the subtree write");

    assert!(
        ctx.exportable(t.child_b),
        "the inverse is keyed to the rows the forward pass wrote, not to a re-walk"
    );
}

#[test]
fn a_leaf_is_a_no_op_rather_than_an_error() {
    let mut ctx = Ctx::new();
    let t = tree(&mut ctx);

    assert!(ctx.apply(t.child_b, false).is_empty());
    assert!(
        ctx.exportable(t.child_b),
        "a leaf's own flag is not its own descendant"
    );
}

#[test]
fn an_id_no_binder_holds_is_a_no_op() {
    let mut ctx = Ctx::new();
    let t = tree(&mut ctx);

    // An absent item has no descendants — the same instruction a leaf gives.
    assert!(ctx.apply(t.after + 9_999, false).is_empty());
    assert!(ctx.exportable(t.root));
}

// ── set_descendants_dict_language ────────────────────────────────────────────
//
// The walk is shared with `set_descendants_exportable` (`crate::subtree`), so
// these do not re-test the boundaries above. What is different here is the
// inverse: the field is a list, so the prior value cannot be derived from the
// new one the way a boolean's can, and each row's own former list has to be
// carried.

#[test]
fn language_reaches_every_descendant_and_not_the_root() {
    let mut ctx = Ctx::new();
    let t = tree(&mut ctx);

    ctx.apply_language(t.root, &["tr-TR"]);

    assert!(
        ctx.language(t.root).is_empty(),
        "the pill field beside the button owns the root"
    );
    assert_eq!(ctx.language(t.child_a), vec!["tr-TR"]);
    assert_eq!(ctx.language(t.grand), vec!["tr-TR"]);
    assert_eq!(ctx.language(t.child_b), vec!["tr-TR"]);
    assert!(ctx.language(t.after).is_empty());
}

#[test]
fn an_empty_tag_list_clears_rather_than_being_ignored() {
    let mut ctx = Ctx::new();
    let t = tree(&mut ctx);
    ctx.apply_language(t.root, &["tr-TR"]);

    // With no container scope in the language model, this is the only way back
    // from an over-broad apply — so it must be a write, not a no-op guard.
    let cleared = ctx.apply_language(t.root, &[]);

    assert_eq!(
        cleared.len(),
        3,
        "all three descendants were carrying a tag"
    );
    assert!(ctx.language(t.child_a).is_empty());
    assert!(ctx.language(t.grand).is_empty());
}

#[test]
fn undo_restores_each_row_to_its_own_previous_list() {
    let mut ctx = Ctx::new();
    let t = tree(&mut ctx);

    // Give one descendant a language of its own first, so a blanket restore to
    // any single value would be caught.
    ctx.apply_language(t.child_a, &["el-GR"]); // reaches nothing: child_a is a leaf
    let mut lone = binder_item_controller::get(&ctx.db, &t.grand)
        .unwrap()
        .unwrap();
    lone.dict_language = vec!["el-GR".to_string()];
    binder_item_controller::update(&ctx.db, &ctx.hub, &mut ctx.undo, None, &lone.into())
        .expect("seed a differing row");

    ctx.apply_language(t.root, &["tr-TR"]);
    ctx.undo.undo(Some(ctx.stack)).expect("undo");

    assert!(ctx.language(t.child_a).is_empty());
    assert_eq!(
        ctx.language(t.grand),
        vec!["el-GR"],
        "restored to what this row held, not to what its siblings held"
    );
    assert!(ctx.language(t.child_b).is_empty());
}

#[test]
fn language_redo_reapplies() {
    let mut ctx = Ctx::new();
    let t = tree(&mut ctx);
    ctx.apply_language(t.root, &["tr-TR"]);
    ctx.undo.undo(Some(ctx.stack)).expect("undo");

    ctx.undo.redo(Some(ctx.stack)).expect("redo");

    assert_eq!(ctx.language(t.child_a), vec!["tr-TR"]);
    assert_eq!(ctx.language(t.grand), vec!["tr-TR"]);
    assert_eq!(ctx.language(t.child_b), vec!["tr-TR"]);
}

#[test]
fn language_changed_ids_skips_rows_that_already_agree() {
    let mut ctx = Ctx::new();
    let t = tree(&mut ctx);

    assert_eq!(ctx.apply_language(t.root, &["tr-TR"]).len(), 3);
    assert!(
        ctx.apply_language(t.root, &["tr-TR"]).is_empty(),
        "a second identical apply writes nothing"
    );
}
