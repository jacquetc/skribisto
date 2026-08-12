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
use teksilo::widgets::{TextWidget, VStack};

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
            .child(
                TextWidget::new(tr!(inspector_tags()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            )
            .child(crate::tags::TagPillField::new(
                tag_value.clone(),
                set_tags,
                tags_vm.clone(),
            ));

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
            col = col
                .child(
                    TextWidget::new(tr!(inspector_aliases()))
                        .style(TextStyleRole::Tiny)
                        .color(TextRole::Secondary),
                )
                .child(crate::tags::AliasPillField::new(alias_value, set_aliases));
        }

        // Cast / Présence: references-first story-bible pins + scan suggestions.
        // Hidden only when the project has no discoverable tags — nothing can be
        // cast material. Always shown for Scene / Note / ChapterScene so the
        // writer can Add even before any prose names anyone.
        if !discoverable.is_empty() {
            let index = panel.mention_index.clone();
            index.changed_signal().bind_to(
                ctx.self_id(),
                ctx.binding_registry(),
                BindingLevel::Rebuild,
            );
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
                let cast = index.cast_for(d.id, live_prose.as_deref(), &d.references, &extra);
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
                        Some(pin.clone()),
                        Some(unpin),
                        open.clone(),
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

                    col = col.child(
                        TextWidget::new(tr!(pov_section()))
                            .style(TextStyleRole::Tiny)
                            .color(TextRole::Secondary),
                    );

                    let pov_ids = d.point_of_view.clone();
                    if pov_ids.is_empty() {
                        col = col.child(
                            TextWidget::new(tr!(pov_empty()))
                                .style(TextStyleRole::Tiny)
                                .color(TextRole::Secondary),
                        );
                    } else {
                        col = col.child(crate::tags::pov_chip_row(
                            crate::tags::pov_chips(&table, &pov_ids),
                            clear_pov,
                        ));
                        // Two viewpoints in one scene is head-hopping. Stated as
                        // an observation, not a warning: writers do it on purpose,
                        // and the schema allows it precisely so it can be seen
                        // rather than blocked.
                        if pov_ids.len() > 1 {
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

                // Backlinks, on a discoverable item: where this character appears.
                // No pin — that would write the wrong item's references.
                if tag_value.get().iter().any(|id| discoverable.contains(id)) {
                    let backlinks = index.backlinks_for(d.id);
                    if !backlinks.is_empty() {
                        col = col
                            .child(
                                TextWidget::new(tr!(mentions_backlinks()))
                                    .style(TextStyleRole::Tiny)
                                    .color(TextRole::Secondary),
                            )
                            .child(crate::tags::MentionList::new(backlinks, None, None, open));
                    }
                }
            } else if tag_value.get().iter().any(|id| discoverable.contains(id)) {
                // Out of cast scope but still story-bible: show Appears in only.
                let open: crate::tags::mention_list::OpenTarget =
                    Rc::new(move |item_id, title, c: &mut EventContext| {
                        c.send_intent(crate::intents::AppIntent::OpenItemToSide { item_id, title });
                    });
                let backlinks = index.backlinks_for(d.id);
                if !backlinks.is_empty() {
                    col = col
                        .child(
                            TextWidget::new(tr!(mentions_backlinks()))
                                .style(TextStyleRole::Tiny)
                                .color(TextRole::Secondary),
                        )
                        .child(crate::tags::MentionList::new(backlinks, None, None, open));
                }
            }
        }
    }
    col
}
