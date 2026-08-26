// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The trailing **Inspector** dock: a context panel that adapts to the focused
//! binder item (the active editor tab). It shows the item's title and — for any
//! item with a promote pair (Chapter, ChapterScene, Scene, Note, Folder, Note
//! folder) — a "Promote to `<target>`" button, the Chapter/ChapterScene inspectors'
//! headline affordance. Rebuilds when the focused item changes.
//!
//! It reuses the shared
//! [`promote_with_guard`](crate::binder::dock::promote_with_guard) so the button behaves exactly like
//! the outline context menu (incl. the demote-empty MessageBox).

use std::rc::Rc;

mod exportable;
mod goal;
mod language;
mod milestone;
mod numbering;
mod story_bible;

#[cfg(test)]
mod tests;

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, DockOpenLocation, DockSide, DockWidget, DockWidgetId, HStack, Padding,
    PopoverButton, TextWidget, VStack,
};

use frontend::AppContext;
use frontend::commands::{binder_commands, binder_item_commands, work_commands};
use frontend::common::direct_access::binder::BinderRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, GoalUnit};
use frontend::common::event::{BinderItemManagementEvent, Event, Origin};

use skribisto_model::compile::ItemMeta;
use skribisto_model::counting::CountingMethodSetting;

use crate::app_ids::AppIds;
use crate::binder::OutlineViewModel;
use crate::binder::dock::promote_menu;
use crate::models::BinderTreeKey;
use crate::singles::SingleBinderItem;

/// The open Work's flat, ordered `ItemMeta` stream — id/role/sub_role/indent/activated/
/// is_exportable only, **without** fetching prose (unlike `export`'s `client_gather`, which
/// needs content for its preview). Used to resolve a focused Part/Chapter's enclosing Book
/// via [`skribisto_model::compile::enclosing_head`]. Cheap and only walked off a
/// focus change, mirroring
/// `export::compute_applicable`.
///
/// Resolves the Work through `ids.work_id` (the id `AppIds` already carries — the
/// Phase-1 seam) rather than `get_all_work(ctx)`'s first entry: a second open Work
/// would make "whichever the store returns first" wrong, not just imprecise.
pub(super) fn live_item_metas(ctx: &AppContext, ids: &AppIds) -> Vec<ItemMeta> {
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
                // The real binder: this stream feeds `milestone`'s `enclosing_head`, which
                // must stop at the manuscript's edge rather than claim a notes row for the
                // work's last Book.
                binder_id,
                role: it.role,
                sub_role: it.sub_role,
                indent: it.indent as i32,
                activated: it.activated,
                is_exportable: it.is_exportable,
                exclude_from_numbering: it.exclude_from_numbering,
            });
        }
    }
    out
}

/// Every live `Folder/Book` in the open Work, id and title, across every Binder in
/// binder order. This is the Books section's own candidate list, and (through its
/// length) the gate that decides whether that section renders at all: below two Books, a
/// writer sees no control, no empty picker, no chrome (see `tags::books`'s module
/// doc for why).
///
/// **Trashed excluded.** `activated` gates what a filing target may resolve to,
/// matching `reconcile_backref_binder_item_books`'s own reasoning for pruning a
/// *deleted* Book from every list naming it: a merely-trashed one has not been
/// deleted, but from where a writer is filing a new note it should look exactly as
/// absent as one that has.
///
/// **`Folder/Book` only, deliberately.** A Book also has a legacy flat encoding,
/// the top-level `Item/BookBegin` marker (see `qleany.yaml`'s own comment on the
/// `books` field), but `CreateType` no longer offers it as a creatable shape:
/// every Book a writer can make today is `Folder/Book`. A row still carrying the
/// legacy marker is not a candidate here and cannot be filed under, the same way
/// [`crate::tags::books::book_chips`] drops an id that resolves to one: filing is
/// scoped to the modern encoding, not to every row `SubRoleExt::opens_book()`
/// would admit. A Work whose Books are all still legacy rows shows no Books
/// section at all, by the same >=2 gate, until at least two are promoted.
///
/// Same shape as [`live_item_metas`]: a fresh walk of the Work's binders, not a
/// cached signal, because the Inspector already rebuilds on every focus change,
/// which is the moment this needs to be current.
///
/// `pub(crate)`, not `pub(super)`: [`crate::story_bible`]'s creation modal needs the
/// exact same candidate table, gated the exact same way, for its own Books section,
/// and a second walk written there would be one more place the >=2 gate and the
/// `activated` filter would have to be kept in step with this one by hand.
pub(crate) fn live_books(
    ctx: &AppContext,
    ids: &AppIds,
) -> Vec<crate::tags::cast_add::CastCandidate> {
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
            if it.activated
                && it.role == BinderItemRole::Folder
                && it.sub_role == BinderItemSubRole::Book
            {
                out.push(crate::tags::cast_add::CastCandidate {
                    id: it.id,
                    title: it.title,
                });
            }
        }
    }
    out
}

/// Package the inspector as a trailing `DockWidget`. `focus` is the active
/// editor tab's item id (the "focused" binder item).
///
/// Every argument is a distinct live handle this dock binds — the Tier-2 ones deliberately
/// threaded in rather than looked up (see the struct's own note). Bundling them into a
/// config struct would add a type whose only job is to be destructured immediately.
#[allow(clippy::too_many_arguments)]
pub fn inspector_dock(
    app_ctx: Rc<AppContext>,
    outline: OutlineViewModel,
    focus: Signal<Option<u64>>,
    dock_id: DockWidgetId,
    tags: crate::tags::TagsViewModel,
    mention_index: crate::mentions::MentionIndex,
    open_docs: crate::models::OpenDocsStore,
    counting_method: Signal<CountingMethodSetting>,
    goal_unit: Signal<GoalUnit>,
) -> DockWidget {
    DockWidget::new(dock_id, tr!(inspector()), move |_id| {
        Inspector::new(
            app_ctx.clone(),
            outline.clone(),
            focus.clone(),
            tags.clone(),
            mention_index.clone(),
            open_docs.clone(),
            counting_method.clone(),
            goal_unit.clone(),
        )
    })
    .icon(crate::icons::activity::inspector_icon)
    .default_location(DockOpenLocation::side(DockSide::Trailing))
}

pub(super) struct Inspector {
    app_ctx: Rc<AppContext>,
    outline: OutlineViewModel,
    focus: Signal<Option<u64>>,
    probe: SingleBinderItem,
    /// Rebuild trigger bumped on binder moves — a same-indent cross-container move can
    /// change a Part/Chapter's enclosing Book (or slide a Book boundary past it) without
    /// touching the item's own row, so nothing else here would re-resolve the milestone.
    moves: Signal<u64>,
    root_child: Option<WidgetId>,
    /// The counting method the rest of this window displays with, so the target readout
    /// cannot disagree with the number the status bar prints for the same scene.
    counting_method: Signal<CountingMethodSetting>,
    /// The project's target unit — Tier 2, from this window's own `WorkSession`'s
    /// `SingleWork`, for the same reason the handles below are threaded in.
    goal_unit: Signal<GoalUnit>,
    /// Tier-2 (per-open-Work) handles, threaded in from the owning window's own
    /// `sessions::WorkSession` — **not** looked up via `ctx.app_state::<T>()`, the
    /// multi-Work migration's whole point (see `sessions::WorkSession`'s module
    /// doc): an `app_state` slot is one process-wide value, so with a second Work
    /// open in a second window, that lookup would silently answer with whichever
    /// Work's session was registered first — this dock's tag picker would then
    /// attach a *different* Work's tag id onto this window's own item.
    tags: crate::tags::TagsViewModel,
    mention_index: crate::mentions::MentionIndex,
    open_docs: crate::models::OpenDocsStore,
    /// Debounced live prose for cast suggestions — never bound at Rebuild to
    /// open-doc edit counters (typing must not rebuild this dock). Frame-tick
    /// and edit effects are re-registered every cast-scope build: teksilo drops
    /// `ctx.effect` on rebuild (same rule as the move subscription above).
    live_cast: crate::tags::LiveCastOverlay,
}

impl Inspector {
    #[allow(clippy::too_many_arguments)]
    fn new(
        app_ctx: Rc<AppContext>,
        outline: OutlineViewModel,
        focus: Signal<Option<u64>>,
        tags: crate::tags::TagsViewModel,
        mention_index: crate::mentions::MentionIndex,
        open_docs: crate::models::OpenDocsStore,
        counting_method: Signal<CountingMethodSetting>,
        goal_unit: Signal<GoalUnit>,
    ) -> Self {
        Self {
            probe: SingleBinderItem::new(app_ctx.clone()),
            app_ctx,
            outline,
            focus,
            counting_method,
            goal_unit,
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
                // The identity header: the item's name, and — for a structural row — the
                // ordinal it carries in the book. Read-only here (renaming happens in the
                // tree or the tab), so the two can sit side by side without any risk of the
                // number reaching an edit buffer.
                // Only a row that opens a structural level can carry one, so the
                // whole-Work walk is skipped entirely for a Scene or a Note — which is most
                // of what the Inspector is ever focused on. `metas` is fetched at most once
                // per build and reused by the milestone lookup further down, which used to
                // walk every binder a second time for the same Work.
                let metas = skribisto_model::numbering::level_of(&d.sub_role)
                    .is_some()
                    .then(|| live_item_metas(&self.app_ctx, &self.outline.ids()));
                let work_id = self.outline.ids().work_id.get();
                let numbered = metas.as_ref().and_then(|m| {
                    work_id.and_then(|id| {
                        crate::models::numbers_for_work(&self.app_ctx, id, m)
                            .get(&d.id)
                            .copied()
                    })
                });
                // **Named the way the tree names it**, through the one resolver both the
                // outline row and the outline card already go through. An untitled
                // structural row is not nameless: it answers to its ordinal, in its own
                // language, and `label_and_badge` also *drops* the separate number badge in
                // that case, because "Chapter 7" already carries the number and showing both
                // reads as a stutter.
                //
                // Resolved here rather than rendered raw because the Inspector was doing
                // exactly what `fallback_label_for`'s own doc warns about: showing a bare
                // "2." with nothing beside it, which reads as a broken row rather than an
                // untitled chapter. Two surfaces disagreeing about what a row is *called* is
                // the same drift this crate guards everywhere else, so this calls the shared
                // resolver rather than growing a third answer of its own.
                let work_langs = work_id
                    .map(|id| crate::models::work_language_tags(&self.app_ctx, id))
                    .unwrap_or_default();
                let fallback =
                    crate::models::fallback_label_for(&d, numbered.as_ref(), &work_langs);
                let (name, ordinal) = crate::models::label_and_badge(
                    &d.title,
                    fallback.as_deref(),
                    numbered.map(|n| n.number()),
                );
                let mut header = HStack::new().spacing(6.0);
                if ordinal.is_some() {
                    header = header.child(crate::widgets::StructureNumber::new(ordinal));
                }
                header = header.child(TextWidget::new(lit!(name)).style(TextStyleRole::BodyBold));
                let mut col = VStack::new().spacing(12.0).child(header);
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
                        .content(promote_menu(outline, key)),
                    );
                }
                col = story_bible::section(col, self, ctx, &d);
                col = language::section(col, self, ctx, &d);
                col = exportable::section(col, self, ctx, &d);
                col = goal::section(col, self, ctx, &d);
                col = numbering::section(col, self, ctx, &d);
                col = milestone::section(col, self, ctx, &d, &metas);
                // Contributed sections last, after everything this application
                // builds. Same order as the container bar and the Analysis bar:
                // a registration adds to the panel, it never displaces what the
                // writer already knows is at the top of it.
                //
                // Each is given the focused item whole and the live handles —
                // never anything captured at registration, which would be a
                // second, permanently empty store. Filtered by `shows_on`, so a
                // section says nothing under a row it has nothing to say about.
                let cx = crate::docks::inspector_sections::InspectorContext {
                    app_ctx: &self.app_ctx,
                    ids: &self.outline.ids(),
                    item: &d,
                };
                for section in crate::docks::inspector_sections::registered_for(&d.sub_role) {
                    col = col
                        .child(
                            TextWidget::new((section.label)())
                                .style(TextStyleRole::Tiny)
                                .color(TextRole::Secondary),
                        )
                        .child(crate::tabs::Boxed::new((section.view)(&cx)));
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

/// The line under the Inspector's target field: how much is written, against the target.
///
/// A widget of its own, and deferred to its first paint, for the same reason the outline
/// card's word line is: the measurement walks the item's whole subtree, and the Inspector
/// rebuilds on every focus change. Doing it inline would mean counting an entire book
/// synchronously each time a writer clicks its folder. Deferring it costs one extra frame
/// and is paid only by a panel actually on screen.
struct GoalReadout {
    app_ctx: Rc<AppContext>,
    work_id: Signal<Option<u64>>,
    item_id: u64,
    goal: i64,
    method: Signal<CountingMethodSetting>,
    unit: GoalUnit,
    /// The measurement, `None` until the first paint has paid for it.
    measured: Signal<Option<crate::goals::Measured>>,
    loaded: std::cell::Cell<bool>,
    root: Option<WidgetId>,
}

impl GoalReadout {
    fn new(
        app_ctx: Rc<AppContext>,
        work_id: Signal<Option<u64>>,
        item_id: u64,
        goal: i64,
        method: Signal<CountingMethodSetting>,
        unit: GoalUnit,
    ) -> Self {
        Self {
            app_ctx,
            work_id,
            item_id,
            goal,
            method,
            unit,
            measured: Signal::new(None),
            loaded: std::cell::Cell::new(false),
            root: None,
        }
    }

    fn load(&self) {
        let Some(work_id) = self.work_id.get() else {
            return;
        };
        self.measured.set(crate::goals::measure::measure(
            &self.app_ctx,
            work_id,
            self.item_id,
            self.method.get(),
            &self.unit,
        ));
    }
}

impl std::fmt::Debug for GoalReadout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoalReadout").finish()
    }
}

impl Widget for GoalReadout {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.measured
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        let Some(m) = self.measured.get() else {
            self.root = None;
            return Vec::new();
        };
        // A row with prose of its own is measured against *its own* length; a container
        // against everything beneath it. That is the whole of "a level encompasses its
        // children": the progress rolls up, the targets never do.
        let written = m.own.unwrap_or(m.subtree).by(&self.unit);
        let id = ctx.add(crate::goals::readout::line(written, self.goal, &self.unit));
        self.root = Some(id);
        vec![id]
    }

    fn paint(&self, _bounds: Rect, _canvas: &mut Canvas, _ctx: &PaintContext) {
        if !self.loaded.replace(true) {
            self.load();
        }
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        match self.root {
            Some(id) => ctx
                .child_size(id, proposal)
                .map(LayoutResponse::from)
                .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into()),
            None => proposal.resolve(0.0, 0.0).into(),
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}
