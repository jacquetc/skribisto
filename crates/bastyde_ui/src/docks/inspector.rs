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
    Button, ButtonVariant, DateEdit, DockOpenLocation, DockSide, DockWidget, DockWidgetId, HStack,
    Padding, PopoverButton, TextWidget, Toggle, VStack,
};
use jiff::civil::Date;

use frontend::AppContext;
use frontend::commands::{binder_commands, binder_item_commands, work_commands};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::BinderItemSubRole;
use frontend::common::event::{BinderItemManagementEvent, Event, Origin};

use skribisto_model::compile::{ItemMeta, StreamLevel, enclosing_head};

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
fn live_item_metas(ctx: &AppContext) -> Vec<ItemMeta> {
    let mut out = Vec::new();
    let Some(work) = work_commands::get_all_work(ctx).ok().and_then(|w| w.into_iter().next()) else {
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
) -> DockWidget {
    DockWidget::new(dock_id, tr!(inspector()), move |_id| {
        Inspector::new(app_ctx.clone(), outline.clone(), focus.clone())
    })
    .icon(crate::activity_icons::inspector_icon)
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
}

impl Inspector {
    fn new(app_ctx: Rc<AppContext>, outline: OutlineViewModel, focus: Signal<Option<u64>>) -> Self {
        Self {
            probe: SingleBinderItem::new(app_ctx.clone()),
            app_ctx,
            outline,
            focus,
            moves: Signal::new(0),
            root_child: None,
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
                let key = BinderTreeKey::Item(d.id);
                let targets = self.outline.promote_targets_of(key);
                if !targets.is_empty() {
                    let outline = self.outline.clone();
                    col = col.child(
                        PopoverButton::new(
                            Button::new(tr!(inspector_promote())).variant(ButtonVariant::Tinted),
                        )
                        .content(promote_menu(outline, key)),
                    );
                }
                // Per-item language override (Step 9): the pill field over this item's own
                // `dict_language`, with the inherited list (item → Book → Work) as the
                // placeholder shown when the item declares none of its own.
                if let Some(spell) = ctx.app_state::<crate::spellcheck::SpellcheckService>().cloned()
                {
                    let inherited = ctx
                        .app_state::<crate::models::OpenDocsStore>()
                        .map(|s| s.effective_language(d.id));
                    let value = Signal::new(d.dict_language.clone());
                    // A probe fixed to *this* item, so the write targets it even after focus
                    // moves on (unlike the shared `self.probe`).
                    let item_probe = SingleBinderItem::new(self.app_ctx.clone());
                    item_probe.set_id(Some(d.id));
                    let stack = self.outline.ids().stack_id.get();
                    let set: crate::language_pill_field::SetLanguages = {
                        let value = value.clone();
                        Rc::new(move |new: String, _c| {
                            let _ = item_probe.set_dict_language(&new, stack);
                            value.set(new);
                        })
                    };
                    col = col
                        .child(TextWidget::new(tr!(inspector_dict_language())).style(TextStyleRole::Tiny).color(TextRole::Secondary))
                        .child(crate::language_pill_field::LanguagePillField::new(
                            value, set, spell, inherited,
                        ));
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
                if matches!(d.sub_role, BinderItemSubRole::Part | BinderItemSubRole::ChapterScene) {
                    let metas = live_item_metas(&self.app_ctx);
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
