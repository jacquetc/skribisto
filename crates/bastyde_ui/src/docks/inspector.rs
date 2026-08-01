// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The trailing **Inspector** dock: a context panel that adapts to the focused
//! binder item (the active editor tab). It shows the item's title and — for any
//! item with a promote pair (Chapter, ChapterScene, Scene, Note, Folder, Note
//! folder) — a "Promote to `<target>`" button, the Chapter/ChapterScene inspectors'
//! headline affordance. Rebuilds when the focused item changes.
//!
//! It reuses the shared [`promote_with_guard`] so the button behaves exactly like
//! the outline context menu (incl. the demote-empty MessageBox).

use std::rc::Rc;

use bastyde::core::BindingLevel;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, DateEdit, DockOpenLocation, DockSide, DockWidget, DockWidgetId,
    FocusScope, HStack, Padding, PopoverButton, TextWidget, Toggle, TraversalScopePolicy, VStack,
};
use jiff::civil::Date;

use frontend::AppContext;
use frontend::commands::{binder_commands, binder_item_commands, work_commands};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::BinderItemSubRole;
use frontend::common::event::{BinderItemManagementEvent, Event, Origin};

use skribisto_model::compile::{ItemMeta, StreamLevel, enclosing_head};

use crate::app_ids::AppIds;
use crate::date_convert::{jiff_to_naive, naive_to_jiff_opt};
use crate::docks::outline::promote_menu;
use crate::models::BinderTreeKey;
use crate::singles::{SingleBinderItem, SingleMilestone};
use crate::view_models::OutlineViewModel;

/// The open Work's flat, ordered `ItemMeta` stream — id/role/sub_role/indent/activated/
/// is_exportable only, **without** fetching prose (unlike `export`'s `client_gather`, which
/// needs content for its preview). Used to resolve a focused Part/Chapter's enclosing Book
/// via [`enclosing_head`]. Cheap and only walked off a focus change, mirroring
/// `export::compute_applicable`.
///
/// Resolves the Work through `ids.work_id` (the id `AppIds` already carries — the
/// Phase-1 seam) rather than `get_all_work(ctx)`'s first entry: a second open Work
/// would make "whichever the store returns first" wrong, not just imprecise.
fn live_item_metas(ctx: &AppContext, ids: &AppIds) -> Vec<ItemMeta> {
    let mut out = Vec::new();
    let Some(work_id) = ids.work_id.get() else {
        return out;
    };
    let Ok(Some(work)) = work_commands::get_work(ctx, &work_id) else {
        return out;
    };
    let Ok(binder_ids) =
        work_commands::get_work_relationship(ctx, &work.id, &WorkRelationshipField::Binders)
    else {
        return out;
    };
    for binder_id in binder_ids {
        let Ok(item_ids) = binder_commands::get_binder_relationship(
            ctx,
            &binder_id,
            &BinderRelationshipField::BinderItems,
        ) else {
            continue;
        };
        let Ok(items) = binder_item_commands::get_binder_item_multi(ctx, &item_ids) else {
            continue;
        };
        for it in items.into_iter().flatten() {
            out.push(ItemMeta {
                id: it.id,
                role: it.role,
                sub_role: it.sub_role,
                indent: it.indent as i32,
                activated: it.activated,
                is_exportable: it.is_exportable,
            });
        }
    }
    out
}

/// Package the inspector as a trailing `DockWidget`. `focus` is the active
/// editor tab's item id (the "focused" binder item).
pub fn inspector_dock(
    app_ctx: Rc<AppContext>,
    outline: OutlineViewModel,
    focus: Signal<Option<u64>>,
    dock_id: DockWidgetId,
    tags: crate::view_models::TagsViewModel,
    mention_index: crate::view_models::MentionIndex,
    open_docs: crate::models::OpenDocsStore,
) -> DockWidget {
    DockWidget::new(dock_id, tr!(inspector()), move |_id| {
        Inspector::new(
            app_ctx.clone(),
            outline.clone(),
            focus.clone(),
            tags.clone(),
            mention_index.clone(),
            open_docs.clone(),
        )
    })
    .icon(crate::icons::activity::inspector_icon)
    .default_location(DockOpenLocation::side(DockSide::Trailing))
}

struct Inspector {
    app_ctx: Rc<AppContext>,
    outline: OutlineViewModel,
    focus: Signal<Option<u64>>,
    probe: SingleBinderItem,
    /// Rebuild trigger bumped on binder moves — a same-indent cross-container move can
    /// change a Part/Chapter's enclosing Book (or slide a Book boundary past it) without
    /// touching the item's own row, so nothing else here would re-resolve the milestone.
    moves: Signal<u64>,
    root_child: Option<WidgetId>,
    /// Tier-2 (per-open-Work) handles, threaded in from the owning window's own
    /// `sessions::WorkSession` — **not** looked up via `ctx.app_state::<T>()`, the
    /// multi-Work migration's whole point (see `sessions::WorkSession`'s module
    /// doc): an `app_state` slot is one process-wide value, so with a second Work
    /// open in a second window, that lookup would silently answer with whichever
    /// Work's session was registered first — this dock's tag picker would then
    /// attach a *different* Work's tag id onto this window's own item.
    tags: crate::view_models::TagsViewModel,
    mention_index: crate::view_models::MentionIndex,
    open_docs: crate::models::OpenDocsStore,
    /// Debounced live prose for cast suggestions — never bound at Rebuild to
    /// open-doc edit counters (typing must not rebuild this dock). Frame-tick
    /// and edit effects are re-registered every cast-scope build: bastyde drops
    /// `ctx.effect` on rebuild (same rule as the move subscription above).
    live_cast: crate::tags::LiveCastOverlay,
}

impl Inspector {
    fn new(
        app_ctx: Rc<AppContext>,
        outline: OutlineViewModel,
        focus: Signal<Option<u64>>,
        tags: crate::view_models::TagsViewModel,
        mention_index: crate::view_models::MentionIndex,
        open_docs: crate::models::OpenDocsStore,
    ) -> Self {
        Self {
            probe: SingleBinderItem::new(app_ctx.clone()),
            app_ctx,
            outline,
            focus,
            moves: Signal::new(0),
            root_child: None,
            tags,
            mention_index,
            open_docs,
            live_cast: crate::tags::LiveCastOverlay::new(),
        }
    }
}

impl std::fmt::Debug for Inspector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inspector").finish()
    }
}

impl Widget for Inspector {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Rebuild when focus moves to a different item...
        self.focus
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        // ...and when the focused item *itself* changes under us. Promote rewrites its
        // `(role, sub_role)` and a rename its title, neither of which moves focus — so
        // without this the panel kept offering the conversions of the item's *old* type.
        self.probe.dto_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
        // Re-subscribe on **every** build, not once: `BuildContext::subscribe_event`
        // scopes the subscription to the widget's *current* build and drops it on the
        // next one. Guarding this with a "wired" flag made the panel deaf the moment it
        // first rebuilt — which is exactly when it needed to keep listening.
        self.probe.wire(ctx);
        // A same-indent cross-container move can change a focused Part/Chapter's enclosing
        // Book without touching its own row (neither `focus` nor the probe's dto fires),
        // so the milestone row below would keep targeting the old Book. Moves publish
        // `BinderItemManagement(MoveItems)` (not a per-relationship `Binder(Updated)`), so
        // subscribe to that and bump a rebuild trigger — cheap, and moves are rare.
        self.moves
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        {
            let moves = self.moves.clone();
            ctx.subscribe_event(
                Origin::BinderItemManagement(BinderItemManagementEvent::MoveItems),
                move |_e: &Event| moves.set(moves.get().wrapping_add(1)),
            );
        }

        let item_id = self.focus.get();
        // Only re-point on a genuine focus change: `set_id` re-reads synchronously and
        // writes the dto signal, which is now a rebuild trigger.
        if self.probe.id() != item_id {
            self.probe.set_id(item_id);
        }
        let dto = item_id.and_then(|_| self.probe.dto());

        let body: Box<dyn Widget> = match dto {
            None => Box::new(
                Padding::symmetric(16.0, 16.0)
                    .child(TextWidget::new(tr!(inspector_empty())).color(TextRole::Secondary)),
            ),
            Some(d) => {
                let mut col = VStack::new()
                    .spacing(12.0)
                    .child(TextWidget::new(lit!(d.title.clone())).style(TextStyleRole::BodyBold));
                // The headline affordance: convert this item to another type. A folder
                // can become any other kind of folder, so it is a menu, not a button.
                let key = BinderTreeKey::Item(d.uid);
                let targets = self.outline.promote_targets_of(key);
                if !targets.is_empty() {
                    let outline = self.outline.clone();
                    col = col.child(
                        PopoverButton::new(
                            Button::new(tr!(inspector_promote())).variant(ButtonVariant::Tinted),
                        )
                        // Trap Tab inside the anchored overlay, as every popover must.
                        .content(
                            FocusScope::new(TraversalScopePolicy::Cycle)
                                .child(promote_menu(outline, key)),
                        ),
                    );
                }
                // Tags, and — only for story-bible material — the other names this item
                // answers to in prose.
                //
                // Same shape as the language section below: a local mirror signal plus a
                // probe fixed to *this* item, so a write started here still lands on the
                // right item after focus moves on. The two writers differ underneath though:
                // `set_tags` writes a relationship (already undoable on its own), while
                // `set_aliases` is a scalar patch — see `SingleBinderItem`.
                {
                    let tags_vm = self.tags.clone();
                    let stack = self.outline.ids().stack_id.get();

                    let tag_value = Signal::new(d.tags.clone());
                    let tag_probe = SingleBinderItem::new(self.app_ctx.clone());
                    tag_probe.set_id(Some(d.id));
                    let set_tags: crate::tags::tag_pill_field::SetTags = {
                        let mirror = tag_value.clone();
                        Rc::new(move |ids: Vec<u64>, _c| {
                            let _ = tag_probe.set_tags(&ids, stack);
                            mirror.set(ids);
                        })
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
                        let alias_probe = SingleBinderItem::new(self.app_ctx.clone());
                        alias_probe.set_id(Some(d.id));
                        let set_aliases: crate::tags::alias_pill_field::SetAliases = {
                            let mirror = alias_value.clone();
                            Rc::new(move |names: Vec<String>, _c| {
                                let _ = alias_probe.set_aliases(&names, stack);
                                mirror.set(names);
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
                        let index = self.mention_index.clone();
                        index.changed_signal().bind_to(
                            ctx.self_id(),
                            ctx.binding_registry(),
                            BindingLevel::Rebuild,
                        );
                        // Live overlay version only — not open_docs.edited (that would rebuild
                        // the whole Inspector on every keystroke).
                        self.live_cast.version().bind_to(
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
                            // (bastyde drops them on rebuild — a one-shot "wired" flag
                            // freezes live suggestions after the first version bump).
                            // Frame tick runs to_djot after idle; edit uses this item's
                            // edit_gen only (not store-wide edited_any).
                            let wake = ctx.wake_at_handle();
                            let live = self.live_cast.clone();
                            let docs = self.open_docs.clone();
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
                            if let Some(doc) = self.open_docs.peek(d.id) {
                                let edit_gen = doc.edit_gen.clone();
                                let live = self.live_cast.clone();
                                let wake = wake.clone();
                                ctx.effect(&edit_gen, move |g| {
                                    live.on_edit_gen(*g, &wake);
                                });
                            }
                            self.live_cast.on_focus(d.id, &wake);

                            let open: crate::tags::mention_list::OpenTarget =
                                Rc::new(move |item_id, title, c: &mut EventContext| {
                                    c.send_intent(crate::intents::AppIntent::OpenItemToSide {
                                        item_id,
                                        title,
                                    });
                                });

                            let extra: Vec<u64> =
                                if matches!(d.sub_role, BinderItemSubRole::ChapterScene) {
                                    self.outline.subtree_descendants(d.id)
                                } else {
                                    Vec::new()
                                };
                            // prose_for already drops empty strings; cast_for treats
                            // empty as batch-only as well.
                            let live_prose = self.live_cast.prose_for(d.id);
                            let cast = index.cast_for(
                                d.id,
                                live_prose.as_deref(),
                                &d.references,
                                &extra,
                            );
                            let cast_empty = cast.is_empty();

                            // Read current refs from the probe on each click — a frozen
                            // snapshot would drop prior pins when Add is used twice before
                            // the next rebuild lands.
                            let pin_probe = SingleBinderItem::new(self.app_ctx.clone());
                            pin_probe.set_id(Some(d.id));
                            let index_for_filter = index.clone();
                            let owner_id = d.id;
                            let pin: crate::tags::mention_list::PinReference = {
                                let pin_probe = pin_probe.clone();
                                let index_for_filter = index_for_filter.clone();
                                Rc::new(move |target, _c| {
                                    let mut next = pin_probe
                                        .dto()
                                        .map(|x| x.references)
                                        .unwrap_or_default();
                                    if !next.contains(&target) {
                                        next.push(target);
                                    }
                                    let next =
                                        index_for_filter.filter_cast_targets(owner_id, &next);
                                    let _ = pin_probe.set_references(&next, stack);
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
                                    let next =
                                        index_for_filter.filter_cast_targets(owner_id, &next);
                                    let _ = pin_probe.set_references(&next, stack);
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

                            let candidates = crate::tags::candidates_from_table(
                                &index.discoverable_table(),
                            );
                            col = col.child(crate::tags::cast_add_button(
                                candidates,
                                d.references.clone(),
                                d.id,
                                pin,
                            ));

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
                                        .child(crate::tags::MentionList::new(
                                            backlinks, None, None, open,
                                        ));
                                }
                            }
                        } else if tag_value.get().iter().any(|id| discoverable.contains(id)) {
                            // Out of cast scope but still story-bible: show Appears in only.
                            let open: crate::tags::mention_list::OpenTarget =
                                Rc::new(move |item_id, title, c: &mut EventContext| {
                                    c.send_intent(crate::intents::AppIntent::OpenItemToSide {
                                        item_id,
                                        title,
                                    });
                                });
                            let backlinks = index.backlinks_for(d.id);
                            if !backlinks.is_empty() {
                                col = col
                                    .child(
                                        TextWidget::new(tr!(mentions_backlinks()))
                                            .style(TextStyleRole::Tiny)
                                            .color(TextRole::Secondary),
                                    )
                                    .child(crate::tags::MentionList::new(
                                        backlinks, None, None, open,
                                    ));
                            }
                        }
                    }
                }

                // Per-item language override (Step 9): the pill field over this item's own
                // `dict_language`, with the Work's list as the placeholder shown when the item
                // declares none of its own. Nothing inherits from a container — an item's tag
                // reaches only that item (see `skribisto_model::language`) — so "Apply to
                // children" beside it is the *only* way a language spreads down a subtree.
                if let Some(spell) = ctx
                    .app_state::<crate::spellcheck::SpellcheckService>()
                    .cloned()
                {
                    let inherited = Some(self.open_docs.effective_language(d.id));
                    let value = Signal::new(d.dict_language.clone());
                    // A probe fixed to *this* item, so the write targets it even after focus
                    // moves on (unlike the shared `self.probe`).
                    let item_probe = SingleBinderItem::new(self.app_ctx.clone());
                    item_probe.set_id(Some(d.id));
                    let stack = self.outline.ids().stack_id.get();
                    let set: crate::spellcheck::language_pill_field::SetLanguages = {
                        let value = value.clone();
                        Rc::new(move |new: Vec<String>, _c| {
                            let _ = item_probe.set_dict_language(&new, stack);
                            value.set(new);
                        })
                    };
                    col = col
                        .child(
                            TextWidget::new(tr!(inspector_dict_language()))
                                .style(TextStyleRole::Tiny)
                                .color(TextRole::Secondary),
                        )
                        .child(crate::spellcheck::language_pill_field::LanguagePillField::new(
                            value.clone(),
                            set,
                            spell,
                            inherited.clone(),
                            self.open_docs.clone(),
                        ));
                    // Push this language down the subtree, one undo step (shown only when the
                    // item actually has a subtree — the same gate the export toggle uses).
                    //
                    // It applies the list the pills **display**, not the raw field: when the
                    // item declares nothing of its own the pills show the Work's language, and
                    // stamping something else than what the writer is looking at would be a
                    // lie. The consequence is deliberate — the descendants end up carrying a
                    // real tag, so a later change to the Work's language no longer reaches
                    // them. That is what "apply" means here, and one Undo takes it back.
                    if !self.outline.subtree_descendants(d.id).is_empty() {
                        let outline = self.outline.clone();
                        let id = d.id;
                        let value = value.clone();
                        let placeholder = self.open_docs.effective_language(d.id);
                        col = col.child(
                            Button::new(tr!(inspector_apply_language_to_children()))
                                .variant(ButtonVariant::Plain)
                                .on_activate_fn(move |_c| {
                                    let raw = value.get();
                                    let tags = if raw.iter().all(|t| t.trim().is_empty()) {
                                        placeholder.clone()
                                    } else {
                                        raw
                                    };
                                    outline.apply_dict_language_to_subtree(id, &tags);
                                }),
                        );
                    }
                }
                // Per-item **export** toggle (M3): whether this item is included when a
                // structural scope (Book / Chapter / Folder) sweeps it in. On by default; an
                // explicit Export Scene/Note or a checked Choose… item overrides it. Beside
                // it, "Apply to children" pushes this value across the whole subtree in one
                // undo step (shown only when the item actually has a subtree).
                {
                    let value = Signal::new(d.is_exportable);
                    let item_probe = SingleBinderItem::new(self.app_ctx.clone());
                    item_probe.set_id(Some(d.id));
                    let stack = self.outline.ids().stack_id.get();
                    {
                        let probe = item_probe.clone();
                        // Write only on a genuine change — never on the initial seed nor the
                        // post-write echo (the entity Updated event rebuilds this panel), so
                        // the toggle can't feed back into itself.
                        ctx.effect(&value, move |on| {
                            if probe.dto().map(|d| d.is_exportable) != Some(*on) {
                                let _ = probe.set_exportable(*on, stack);
                            }
                        });
                    }
                    col = col.child(
                        TextWidget::new(tr!(inspector_export()))
                            .style(TextStyleRole::Tiny)
                            .color(TextRole::Secondary),
                    );
                    col = col.child(Toggle::new(value.clone()).label(tr!(inspector_exportable())));
                    if !self.outline.subtree_descendants(d.id).is_empty() {
                        let outline = self.outline.clone();
                        let id = d.id;
                        col = col.child(
                            Button::new(tr!(inspector_apply_to_children()))
                                .variant(ButtonVariant::Plain)
                                .on_activate_fn(move |_c| {
                                    outline.apply_exportable_to_subtree(id, value.get())
                                }),
                        );
                    }
                }
                // Per-Part/Chapter **milestone** (M5): a target date pinned on the Book's
                // Pace timeline, set right where the writer plans the section. A milestone
                // only makes sense for a compile-stream Part or Chapter, and only inside a
                // Book — so gate on the sub_role, then resolve the enclosing Book head.
                if matches!(
                    d.sub_role,
                    BinderItemSubRole::Part | BinderItemSubRole::ChapterScene
                ) {
                    let metas = live_item_metas(&self.app_ctx, &self.outline.ids());
                    if let Some(book_id) = metas
                        .iter()
                        .position(|m| m.id == d.id)
                        .and_then(|pos| enclosing_head(&metas, pos, StreamLevel::Book))
                        .map(|head| metas[head].id)
                    {
                        let probe = SingleMilestone::new(self.app_ctx.clone(), self.outline.ids());
                        probe.set_book_and_item(Some(book_id), Some(d.id));
                        probe.wire(ctx);
                        // jiff mirror for the DateEdit; two guarded effects bridge the
                        // entity's chrono date <-> the widget's jiff date, each writing only
                        // on a genuine change so neither echoes the other into a loop.
                        let local: Signal<Option<Date>> =
                            Signal::new(naive_to_jiff_opt(probe.date_signal().get()));
                        {
                            let local = local.clone();
                            ctx.effect(&probe.date_signal(), move |d| {
                                let jd = naive_to_jiff_opt(*d);
                                if local.get() != jd {
                                    local.set(jd);
                                }
                            });
                        }
                        {
                            let probe = probe.clone();
                            ctx.effect(&local, move |jd| {
                                let want = jd.map(jiff_to_naive);
                                if probe.date() != want {
                                    probe.set_date(want);
                                }
                            });
                        }
                        let has_date = local.map(|d| d.is_some());
                        col = col
                            .child(
                                TextWidget::new(tr!(inspector_milestone()))
                                    .style(TextStyleRole::Tiny)
                                    .color(TextRole::Secondary),
                            )
                            .child(
                                HStack::new()
                                    .spacing(8.0)
                                    .child(
                                        DateEdit::new(local.clone())
                                            .placeholder(tr!(inspector_milestone_none())),
                                    )
                                    .child(
                                        Button::new(tr!(inspector_milestone_clear()))
                                            .variant(ButtonVariant::Plain)
                                            .enabled(has_date)
                                            .on_activate_fn({
                                                let local = local.clone();
                                                move |_c| local.set(None)
                                            }),
                                    ),
                            );
                    }
                }
                Box::new(Padding::symmetric(16.0, 16.0).child(col))
            }
        };

        let id = ctx.add_boxed(body);
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0))
            .into()
    }
}
