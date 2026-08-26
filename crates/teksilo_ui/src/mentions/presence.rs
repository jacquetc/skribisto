// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! **Confirming a presence from the entry's own side.**
//!
//! The cast list on a scene asks "who is here" and pins an entry into *this scene's*
//! `references`. A story-bible entry's own "Appears in" reading asks the mirror question,
//! "where does she turn up", and the writer working through it wants the same decision
//! from the other end: yes, that mention is really her.
//!
//! It is the **same relationship**, written from the other direction — the note goes into
//! the mentioning document's `references`, never the other way round. That is the whole
//! reason this is not simply [`crate::tags::MentionList`]'s own pin callback reused: a
//! pin keyed on the row's target would write the entry's own references and record the
//! backwards claim, which is why the backlink direction has always passed `None` there.
//!
//! ## Confirm only
//!
//! There is no decline. A suggestion the writer ignores stays a suggestion, ghosted, and
//! costs nothing; a *denied* mention would be a third persisted state, and the schema has
//! two. It would also be a promise this feature cannot keep — the scan re-runs on every
//! edit, and "not her" would have to survive a rewritten sentence, a renamed alias and a
//! moved scene to mean anything at all.
//!
//! ## One press, one undo
//!
//! Confirming a whole reading writes one relationship per mentioning document, and a
//! writer who confirmed eleven appearances with one press must undo them with one Ctrl+Z
//! rather than eleven. The composite group is what makes that true, exactly as
//! `SingleBinderItem::set_point_of_view` uses it to keep a point of view and its cast
//! entry inseparable.

use frontend::AppContext;
use frontend::commands::{binder_item_commands, undo_redo_commands};
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::direct_access::BinderItemRelationshipDto;

/// Write `entry` into the `references` of every document in `owners`, as one undoable
/// action. Returns how many documents actually changed.
///
/// Documents that already reference `entry` are skipped rather than rewritten, so
/// confirming a reading twice is a no-op and puts nothing on the undo stack — which is
/// what makes a "confirm every appearance" button safe to press again after adding a
/// scene.
#[must_use]
pub fn confirm(ctx: &AppContext, owners: &[u64], entry: u64, stack: Option<u64>) -> usize {
    // An entry cannot appear in itself, and a doubled id must not be written twice.
    let mut wanted: Vec<u64> = Vec::with_capacity(owners.len());
    for &owner in owners {
        if owner != entry && !wanted.contains(&owner) {
            wanted.push(owner);
        }
    }
    if wanted.is_empty() {
        return 0;
    }

    // One batched read rather than one per document: a character can appear in fifty
    // scenes, and this runs from a button press on the UI thread.
    let Ok(current) = binder_item_commands::get_binder_item_relationship_many(
        ctx,
        &wanted,
        &BinderItemRelationshipField::References,
    ) else {
        return 0;
    };

    let pending: Vec<(u64, Vec<u64>)> = wanted
        .into_iter()
        .filter_map(|owner| {
            let mut refs = current.get(&owner).cloned().unwrap_or_default();
            if refs.contains(&entry) {
                return None;
            }
            refs.push(entry);
            Some((owner, refs))
        })
        .collect();
    if pending.is_empty() {
        return 0;
    }

    let grouped = pending.len() > 1;
    if grouped {
        let _ = undo_redo_commands::begin_composite(ctx, stack);
    }
    let mut wrote = 0;
    for (owner, right_ids) in pending {
        match binder_item_commands::set_binder_item_relationship(
            ctx,
            stack,
            &BinderItemRelationshipDto {
                id: owner,
                field: BinderItemRelationshipField::References,
                right_ids,
            },
        ) {
            Ok(()) => wrote += 1,
            Err(e) => eprintln!("story bible: confirming a presence in {owner} failed: {e}"),
        }
    }
    // Closed on every path, so a failed write never leaves the group open — the rule
    // `set_point_of_view` records for the same call pair.
    if grouped {
        undo_redo_commands::end_composite(ctx);
    }
    wrote
}

#[cfg(test)]
mod tests {
    use super::*;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};
    use std::rc::Rc;

    /// A Work with a binder, its own undo stack, and a maker for rows in it.
    ///
    /// The stack is minted rather than assumed: `begin_composite` refuses a stack that
    /// does not exist, which is the same thing `AppIds::adopt_stack` does for a window.
    fn seed() -> (
        Rc<AppContext>,
        Option<u64>,
        impl FnMut(&str, BinderItemSubRole) -> u64,
    ) {
        let ctx = Rc::new(AppContext::new());
        let work = work_commands::create_orphan_work(&ctx, None, &CreateWorkDto::default())
            .expect("create work");
        let binder = binder_commands::create_binder(
            &ctx,
            None,
            &CreateBinderDto {
                name: "Manuscript".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            0,
        )
        .expect("create binder")
        .id;
        let c = ctx.clone();
        let mut index = 0i32;
        let add = move |title: &str, sub_role: BinderItemSubRole| {
            let id = binder_item_commands::create_binder_item(
                &c,
                None,
                &CreateBinderItemDto {
                    title: title.into(),
                    role: BinderItemRole::Item,
                    sub_role,
                    activated: true,
                    is_exportable: true,
                    ..Default::default()
                },
                binder,
                index,
            )
            .expect("create item")
            .id;
            index += 1;
            id
        };
        let stack = Some(undo_redo_commands::create_new_stack(&ctx));
        (ctx, stack, add)
    }

    fn cast_of(ctx: &AppContext, owner: u64) -> Vec<u64> {
        binder_item_commands::get_binder_item_relationship(
            ctx,
            &owner,
            &BinderItemRelationshipField::References,
        )
        .expect("read references")
    }

    /// **The claim goes into the mentioning document, never into the entry.**
    ///
    /// The whole reason this is not the roster's pin callback reused: a pin is keyed on
    /// the row's target and writes the list owner's references, which read from a backlink
    /// list would record "the character references the scene" — the relationship
    /// backwards, and invisible until some other surface read it back.
    #[test]
    fn confirming_writes_the_entry_into_the_documents_that_mention_it() {
        let (ctx, stack, mut add) = seed();
        let elena = add("Elena", BinderItemSubRole::Note);
        let dusk = add("Dusk", BinderItemSubRole::Scene);
        let dawn = add("Dawn", BinderItemSubRole::Scene);

        assert_eq!(confirm(&ctx, &[dusk, dawn], elena, stack), 2);
        assert_eq!(cast_of(&ctx, dusk), vec![elena]);
        assert_eq!(cast_of(&ctx, dawn), vec![elena]);
        assert!(
            cast_of(&ctx, elena).is_empty(),
            "the entry's own references must not have been touched"
        );
    }

    /// **One press, one undo.** A writer who confirmed a whole reading with one button
    /// must take it back with one Ctrl+Z, not one per scene.
    #[test]
    fn confirming_a_whole_reading_undoes_as_one_action() {
        let (ctx, stack, mut add) = seed();
        let elena = add("Elena", BinderItemSubRole::Note);
        let scenes: Vec<u64> = ["One", "Two", "Three"]
            .into_iter()
            .map(|t| add(t, BinderItemSubRole::Scene))
            .collect();

        assert_eq!(confirm(&ctx, &scenes, elena, stack), 3);
        undo_redo_commands::undo(&ctx, stack).expect("undo");
        for scene in &scenes {
            assert!(
                cast_of(&ctx, *scene).is_empty(),
                "one undo must have reverted every document the press wrote"
            );
        }
    }

    /// Confirming twice is a no-op, so the button is safe to press again after a scene is
    /// added — and it puts nothing on the undo stack the second time, which is what stops
    /// a stray press from eating a writer's real Ctrl+Z.
    #[test]
    fn confirming_what_is_already_confirmed_writes_nothing() {
        let (ctx, stack, mut add) = seed();
        let elena = add("Elena", BinderItemSubRole::Note);
        let dusk = add("Dusk", BinderItemSubRole::Scene);
        let dawn = add("Dawn", BinderItemSubRole::Scene);

        assert_eq!(confirm(&ctx, &[dusk], elena, stack), 1);
        assert_eq!(
            confirm(&ctx, &[dusk, dawn], elena, stack),
            1,
            "only the document that had not agreed yet"
        );
        assert_eq!(cast_of(&ctx, dusk), vec![elena], "and not written twice");

        assert_eq!(confirm(&ctx, &[dusk, dawn], elena, stack), 0);
        // The last press wrote nothing, so the undo above it is still the one that
        // added `dawn` — not an empty group swallowing the writer's Ctrl+Z.
        undo_redo_commands::undo(&ctx, stack).expect("undo");
        assert!(cast_of(&ctx, dawn).is_empty());
        assert_eq!(cast_of(&ctx, dusk), vec![elena]);
    }

    /// A document already in the cast keeps the company it had: confirming appends, it
    /// does not replace the list.
    #[test]
    fn confirming_appends_rather_than_replacing_a_cast() {
        let (ctx, stack, mut add) = seed();
        let elena = add("Elena", BinderItemSubRole::Note);
        let marcus = add("Marcus", BinderItemSubRole::Note);
        let dusk = add("Dusk", BinderItemSubRole::Scene);

        assert_eq!(confirm(&ctx, &[dusk], marcus, stack), 1);
        assert_eq!(confirm(&ctx, &[dusk], elena, stack), 1);
        assert_eq!(cast_of(&ctx, dusk), vec![marcus, elena]);
    }

    /// An entry cannot appear in itself, and a doubled id must not be written twice —
    /// both reachable from a caller that hands over whatever the row list gave it.
    #[test]
    fn an_entry_is_never_confirmed_into_its_own_cast() {
        let (ctx, stack, mut add) = seed();
        let elena = add("Elena", BinderItemSubRole::Note);
        let dusk = add("Dusk", BinderItemSubRole::Scene);

        assert_eq!(confirm(&ctx, &[elena], elena, stack), 0);
        assert!(cast_of(&ctx, elena).is_empty());
        assert_eq!(confirm(&ctx, &[dusk, dusk], elena, stack), 1);
        assert_eq!(cast_of(&ctx, dusk), vec![elena]);
    }
}
