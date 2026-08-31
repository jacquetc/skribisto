// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Tags, aliases, cast and point of view — the story-bible half of the panel.
//!
//! The largest of the sections and the only one that reads prose: the cast
//! suggestions come off a debounced overlay of what is actually written, so the
//! panel can offer a name it has just seen without rebuilding on every keystroke.

use std::rc::Rc;

use teksilo::core::BindingLevel;

use frontend::common::entities::BinderItemSubRole;
use frontend::direct_access::BinderItemDto;
use teksilo::prelude::*;
use teksilo::widgets::{Button, ButtonVariant, TextWidget, VStack};

use super::Inspector;
use crate::singles::SingleBinderItem;

pub(super) fn section(
    mut col: VStack,
    panel: &Inspector,
    ctx: &mut BuildContext,
    d: &BinderItemDto,
) -> VStack {
    // Tags, and — only for story-bible material — the other names this item
    // answers to in prose.
    //
    // Same shape as the language section below: a local mirror signal plus a
    // probe fixed to *this* item, so a write started here still lands on the
    // right item after focus moves on. The two writers differ underneath though:
    // `set_tags` writes a relationship (already undoable on its own), while
    // `set_aliases` is a scalar patch — see `SingleBinderItem`.
    {
        let tags_vm = panel.tags.clone();
        let stack = panel.outline.ids().stack_id.get();

        let tag_value = Signal::new(d.tags.clone());
        let tag_probe = SingleBinderItem::new(panel.app_ctx.clone());
        tag_probe.set_id(Some(d.id));
        let set_tags: crate::tags::tag_pill_field::SetTags = {
            let mirror = tag_value.clone();
            // Only echo the write into the mirror once it actually lands —
            // e.g. the item was trashed by another window between the click
            // and the commit. Setting it unconditionally would show the new
            // pill row while the store never got it, reverting only on the
            // next unrelated refresh with no clue why.
            Rc::new(
                move |ids: Vec<u64>, _c| match tag_probe.set_tags(&ids, stack) {
                    Ok(()) => mirror.set(ids),
                    Err(e) => eprintln!("inspector: set tags failed: {e}"),
                },
            )
        };
        col = col
            .child(crate::widgets::tip::RichTip::new(
                crate::tooltip_registry::CONCEPT_TAG,
                TextWidget::new(tr!(inspector_tags()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            ))
            .child(crate::tags::TagPillField::new(
                tag_value.clone(),
                set_tags,
                tags_vm.clone(),
            ));

        // **Bound before anything reads them, and outside every gate below.**
        //
        // Both of these arrive *after* a project opens: the palette when the Work's tags
        // load, the index when the first scan lands. Every story-bible section here is
        // gated on one or the other, and the bindings used to sit *inside* those gates,
        // so on a project opened straight onto a Note the panel built with an empty
        // palette, took the "nothing is discoverable" branch, and therefore never
        // subscribed to the thing that would have told it otherwise. It stayed blank
        // until the writer moved focus to another row and back, which rebuilds the dock
        // for an unrelated reason and by then finds both loaded.
        //
        // A guard must never decide whether to subscribe to the data the guard itself
        // reads.
        tags_vm.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        panel.mention_index.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        // Aliases only make sense on an item the mention index will actually
        // scan for: a discoverable tag is what puts it in that set, so aliases on
        // anything else would be indexed against nothing. Gating on the tags the
        // *mirror* holds (not the fetched DTO) means the field appears the moment
        // a discoverable tag is added, without waiting for a refetch.
        let discoverable: std::collections::HashSet<u64> = tags_vm
            .rows()
            .into_iter()
            .filter(|t| t.discoverable)
            .map(|t| t.id)
            .collect();
        if tag_value.get().iter().any(|id| discoverable.contains(id)) {
            let alias_value = Signal::new(d.aliases.clone());
            let alias_probe = SingleBinderItem::new(panel.app_ctx.clone());
            alias_probe.set_id(Some(d.id));
            let set_aliases: crate::tags::alias_pill_field::SetAliases = {
                let mirror = alias_value.clone();
                // Same "only echo a landed write" reasoning as `set_tags`
                // above.
                Rc::new(move |names: Vec<String>, _c| {
                    match alias_probe.set_aliases(&names, stack) {
                        Ok(()) => mirror.set(names),
                        Err(e) => eprintln!("inspector: set aliases failed: {e}"),
                    }
                })
            };
            // Armed with the story-bible table so the "+" popover can name any other
            // item already answering to the alias being typed, live, as a fact rather
            // than a warning: see `AliasPillField::collision_lookup`.
            let alias_table = panel.mention_index.discoverable_table();
            col = col
                .child(
                    TextWidget::new(tr!(inspector_aliases()))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary),
                )
                .child(
                    crate::tags::AliasPillField::new(alias_value, set_aliases)
                        .collision_lookup(alias_table, d.id),
                );
        }

        // Books: which Book or Books this note is declared to belong to. This is
        // the writer's own filing, never a position or a scan result. Only a Note or
        // Note folder needs it: a scene's Book is already derivable from where it
        // sits in the binder, so this is gated through `search_facet_of`, the same
        // constraint-matrix predicate that already unions `Folder/Note` and
        // `Item/Note` under one "Note" facet for search, not a hand-rolled
        // sub_role list.
        //
        // **Offered from the first Book, not the second.** This was gated at two, on
        // the reasoning that a one-Book writer has nothing to declare. That is true of a
        // *filter* — a chip row offering one choice asks a question nobody has — and
        // false of a *field*: filing says which Book an entry is part of, and the model
        // refuses to infer it (empty `books` is "not yet filed", never "every book").
        // Gated at two, a one-Book project could not file anything at all, so every
        // entry read as unfiled for ever, and adding a second Book handed the writer a
        // whole cast to file after the fact. Nothing is offered with no Book at all.
        if matches!(
            skribisto_model::search_facet_of(&d.role, &d.sub_role),
            Some(skribisto_model::SearchFacet::Note)
        ) {
            let candidates = super::live_books(&panel.app_ctx, &panel.outline.ids());
            if !candidates.is_empty() {
                let stack = panel.outline.ids().stack_id.get();
                let books_value = Signal::new(d.books.clone());
                let books_probe = SingleBinderItem::new(panel.app_ctx.clone());
                books_probe.set_id(Some(d.id));

                let set_book: crate::tags::mention_list::PinReference = {
                    let mirror = books_value.clone();
                    let probe = books_probe.clone();
                    // Only echo the write into the mirror once it actually lands:
                    // same reasoning as `set_tags` above.
                    Rc::new(move |target, _c| {
                        let mut next = probe.dto().map(|x| x.books).unwrap_or_default();
                        if !next.contains(&target) {
                            next.push(target);
                        }
                        match probe.set_books(&next, stack) {
                            Ok(()) => mirror.set(next),
                            Err(e) => eprintln!("inspector: file under book failed: {e}"),
                        }
                    })
                };
                let clear_book: crate::tags::books::ClearBook = {
                    let mirror = books_value.clone();
                    let probe = books_probe.clone();
                    Rc::new(move |target: u64, _c: &mut EventContext| {
                        let next: Vec<u64> = probe
                            .dto()
                            .map(|x| x.books)
                            .unwrap_or_default()
                            .into_iter()
                            .filter(|&id| id != target)
                            .collect();
                        match probe.set_books(&next, stack) {
                            Ok(()) => mirror.set(next),
                            Err(e) => eprintln!("inspector: remove book filing failed: {e}"),
                        }
                    })
                };

                col = col.child(
                    TextWidget::new(tr!(books_section()))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary),
                );

                let book_ids = books_value.get();
                if book_ids.is_empty() {
                    col = col.child(
                        TextWidget::new(tr!(books_empty()))
                            .style(TextStyleRole::Tiny)
                            .color(TextRole::Secondary),
                    );
                } else {
                    col = col.child(crate::tags::book_chip_row(
                        crate::tags::book_chips(&candidates, &book_ids),
                        clear_book,
                    ));
                }

                col = col.child(crate::tags::book_add_button(
                    candidates, book_ids, d.id, set_book,
                ));

                // "Apply to children": push this item's current filing onto its
                // whole subtree in one undo step, matching the export toggle and
                // the language field's own "Apply to children" gate. Shown only
                // when the item actually has a subtree (always empty for
                // `Item/Note`, since a leaf is not its own descendant).
                if !panel.outline.subtree_descendants(d.id).is_empty() {
                    let outline = panel.outline.clone();
                    let id = d.id;
                    let books_value = books_value.clone();
                    col = col.child(
                        Button::new(tr!(books_apply_to_children()))
                            .variant(ButtonVariant::Plain)
                            .on_activate_fn(move |_c| {
                                outline.apply_books_to_subtree(id, &books_value.get())
                            }),
                    );
                }
            }
        }

        // Cast / Présence: references-first story-bible pins + scan suggestions.
        // Hidden only when the project has no discoverable tags — nothing can be
        // cast material. Always shown for Scene / Note / ChapterScene so the
        // writer can Add even before any prose names anyone.
        if !discoverable.is_empty() {
            let index = panel.mention_index.clone();
            // Live overlay version only — not open_docs.edited (that would rebuild
            // the whole Inspector on every keystroke).
            panel.live_cast.version().bind_to(
                ctx.self_id(),
                ctx.binding_registry(),
                BindingLevel::Rebuild,
            );

            let cast_scope = matches!(
                d.sub_role,
                BinderItemSubRole::Scene
                    | BinderItemSubRole::Note
                    | BinderItemSubRole::ChapterScene
            );

            if cast_scope {
                // Debounced live prose: re-register effects every build
                // (teksilo drops them on rebuild — a one-shot "wired" flag
                // freezes live suggestions after the first version bump).
                // Frame tick runs to_djot after idle; edit uses this item's
                // edit_gen only (not store-wide edited_any).
                let wake = ctx.wake_at_handle();
                let live = panel.live_cast.clone();
                let docs = panel.open_docs.clone();
                let tick = ctx.frame_tick();
                ctx.effect(&tick, {
                    let live = live.clone();
                    let docs = docs.clone();
                    move |_| {
                        live.tick(std::time::Instant::now(), |id| {
                            docs.peek(id).and_then(|doc| {
                                doc.main.as_ref().and_then(|m| m.doc.to_djot().ok())
                            })
                        });
                    }
                });
                // Per-item edit gen when the focused cast item is open —
                // typing in a side tab must not re-export this scene.
                if let Some(doc) = panel.open_docs.peek(d.id) {
                    let edit_gen = doc.edit_gen.clone();
                    let live = panel.live_cast.clone();
                    let wake = wake.clone();
                    ctx.effect(&edit_gen, move |g| {
                        live.on_edit_gen(*g, &wake);
                    });
                }
                panel.live_cast.on_focus(d.id, &wake);

                let open: crate::tags::mention_list::OpenTarget =
                    Rc::new(move |item_id, title, c: &mut EventContext| {
                        c.send_intent(crate::intents::AppIntent::OpenItemToSide { item_id, title });
                    });

                let extra: Vec<u64> = if matches!(d.sub_role, BinderItemSubRole::ChapterScene) {
                    panel.outline.subtree_descendants(d.id)
                } else {
                    Vec::new()
                };
                // prose_for already drops empty strings; cast_for treats
                // empty as batch-only as well.
                let live_prose = panel.live_cast.prose_for(d.id);
                let cast = index.cast_for(
                    d.id,
                    live_prose.as_deref(),
                    &d.references,
                    &d.point_of_view,
                    &extra,
                );
                let cast_empty = cast.is_empty();

                // Read current refs from the probe on each click — a frozen
                // snapshot would drop prior pins when Add is used twice before
                // the next rebuild lands.
                let pin_probe = SingleBinderItem::new(panel.app_ctx.clone());
                pin_probe.set_id(Some(d.id));
                let index_for_filter = index.clone();
                let owner_id = d.id;
                let pin: crate::tags::mention_list::PinReference = {
                    let pin_probe = pin_probe.clone();
                    let index_for_filter = index_for_filter.clone();
                    Rc::new(move |target, _c| {
                        let mut next = pin_probe.dto().map(|x| x.references).unwrap_or_default();
                        if !next.contains(&target) {
                            next.push(target);
                        }
                        let next = index_for_filter.filter_cast_targets(owner_id, &next);
                        if let Err(e) = pin_probe.set_references(&next, stack) {
                            eprintln!("inspector: pin cast reference failed: {e}");
                        }
                    })
                };
                let unpin: crate::tags::mention_list::UnpinReference = {
                    let pin_probe = pin_probe.clone();
                    let index_for_filter = index_for_filter.clone();
                    Rc::new(move |target, _c| {
                        let next: Vec<u64> = pin_probe
                            .dto()
                            .map(|x| x.references)
                            .unwrap_or_default()
                            .into_iter()
                            .filter(|&id| id != target)
                            .collect();
                        let next = index_for_filter.filter_cast_targets(owner_id, &next);
                        if let Err(e) = pin_probe.set_references(&next, stack) {
                            eprintln!("inspector: unpin cast reference failed: {e}");
                        }
                    })
                };

                col = col
                    .child(
                        TextWidget::new(tr!(cast_section()))
                            .style(TextStyleRole::Tiny)
                            .color(TextRole::Secondary),
                    )
                    .child(crate::tags::MentionList::new(
                        cast,
                        crate::tags::MentionNaming::Target,
                        Some(pin.clone()),
                        Some(unpin),
                        open,
                    ));

                if cast_empty {
                    col = col.child(
                        TextWidget::new(tr!(cast_empty()))
                            .style(TextStyleRole::Tiny)
                            .color(TextRole::Secondary),
                    );
                }

                // Cloned once per rebuild, not once per consumer: the POV
                // section below needs the same table, and `discoverable_table()`
                // deep-clones every story-bible entry each call.
                let table = index.discoverable_table();
                let candidates = crate::tags::candidates_from_table(&table);
                col = col.child(crate::tags::cast_add_button(
                    candidates.clone(),
                    d.references.clone(),
                    d.id,
                    pin,
                ));

                // Point of view: whose eyes this scene is told through.
                //
                // Its own section rather than a badge on a cast row, because it
                // answers a different question — the cast is who appears, the POV
                // is who holds the camera — and a scene routinely has one without
                // the other. Candidates are the full discoverable table, not just
                // the confirmed cast, which is what makes picking a POV able to
                // add someone to the cast rather than requiring them there first.
                //
                // Rendered as a chip row rather than a `MentionList`: that row
                // type carries scan evidence (hit counts, the matched name, the
                // sentence it was found in) which means nothing for a POV, since
                // a scene told in deep POV may never name its own viewpoint
                // character at all.
                {
                    let pov_probe = SingleBinderItem::new(panel.app_ctx.clone());
                    pov_probe.set_id(Some(d.id));
                    let set_pov: crate::tags::mention_list::PinReference = {
                        let pov_probe = pov_probe.clone();
                        Rc::new(move |target, _c| {
                            let mut next =
                                pov_probe.dto().map(|x| x.point_of_view).unwrap_or_default();
                            if !next.contains(&target) {
                                next.push(target);
                            }
                            if let Err(e) = pov_probe.set_point_of_view(&next, stack) {
                                eprintln!("inspector: set point of view failed: {e}");
                            }
                        })
                    };
                    let clear_pov = {
                        let pov_probe = pov_probe.clone();
                        Rc::new(move |target: u64, _c: &mut EventContext| {
                            let next: Vec<u64> = pov_probe
                                .dto()
                                .map(|x| x.point_of_view)
                                .unwrap_or_default()
                                .into_iter()
                                .filter(|&id| id != target)
                                .collect();
                            if let Err(e) = pov_probe.set_point_of_view(&next, stack) {
                                eprintln!("inspector: clear point of view failed: {e}");
                            }
                        })
                    };

                    col = col.child(crate::widgets::tip::RichTip::new(
                        crate::tooltip_registry::CONCEPT_POINT_OF_VIEW,
                        TextWidget::new(tr!(pov_section()))
                            .style(TextStyleRole::Tiny)
                            .color(TextRole::Secondary),
                    ));

                    let pov_ids = d.point_of_view.clone();
                    if pov_ids.is_empty() {
                        col = col.child(
                            TextWidget::new(tr!(pov_empty()))
                                .style(TextStyleRole::Tiny)
                                .color(TextRole::Secondary),
                        );
                    } else {
                        let pov_chips = crate::tags::pov_chips(&table, &pov_ids);
                        if !pov_chips.is_empty() {
                            col =
                                col.child(crate::tags::pov_chip_row(pov_chips.clone(), clear_pov));
                        }
                        // A pin that no longer resolves (the target was trashed, or
                        // lost its discoverable tag) must not look identical to no
                        // point of view ever having been set: see
                        // `tags::pov::pov_has_unresolved`.
                        if crate::tags::pov_has_unresolved(&table, &pov_ids) {
                            col = col.child(
                                TextWidget::new(tr!(pov_unresolved()))
                                    .style(TextStyleRole::Tiny)
                                    .color(TextRole::Secondary),
                            );
                        }
                        // Two viewpoints in one scene is head-hopping. Stated as an
                        // observation, not a warning: writers do it on purpose, and
                        // the schema allows it precisely so it can be seen rather
                        // than blocked. Counted on what actually resolves, not on
                        // the raw pin count: a pin pointing at a gone target is not
                        // a second viewpoint the reader can ever meet.
                        if pov_chips.len() > 1 {
                            col = col.child(
                                TextWidget::new(tr!(pov_multiple()))
                                    .style(TextStyleRole::Tiny)
                                    .color(TextRole::Secondary),
                            );
                        }
                    }

                    col = col.child(crate::tags::pov_add_button(
                        candidates, pov_ids, d.id, set_pov,
                    ));
                }

                // **No backlinks list here.** "Appears in" belongs to the entry's own
                // Details page (`tabs::note_details`), which has the width to show a
                // document, its matched name and the sentence it was found in. In a
                // 300 dp rail it was a column of ellipsised titles, and it is the one
                // thing the Inspector showed that the writer was not looking *at*: this
                // panel is about the focused row, and a backlink list is about
                // everything else. `MentionList`'s confirm control keeps working there,
                // which is the direction that write was always meant to be made from.
            }
        }
    }
    col
}
