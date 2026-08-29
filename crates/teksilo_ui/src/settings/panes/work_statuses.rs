// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Work ▸ **Statuses** — the per-project workflow ladder.
//!
//! This is the "user-editable" half of the two-level status model, and without it the other
//! half is a claim the app does not honour: a writer could be *given* a ladder by a preset
//! or by an import, and could file rows on it, but could not rename a rung to their own
//! vocabulary, reorder one, add the pass their process has and no preset does, or remove
//! one they never use. A vocabulary you may only choose from a catalogue is not the
//! writer's; it is the app's, handed over.
//!
//! Shaped like the tag palette next door, with three differences that all come from a
//! ladder being an **ordered, single-valued** axis rather than a set:
//!
//! * **No filter box and no sort.** The order on screen *is* the data — it is what makes
//!   "less finished than" answerable for the merge rule, the Overview sort and the
//!   completion readout's notion of "done". A control that reordered the view would be
//!   lying about the thing being edited. (It would also be pointless: ladders are four to
//!   eight rungs, not forty tags.)
//! * **Each row shows its position**, and carries move-up / move-down. That is the only
//!   way to author an order, and the number beside it is what makes the order legible as
//!   an order rather than as an arbitrary list.
//! * **The colour is not the writer's to pick.** Where the tag row has a `ColorEdit`, this
//!   one has a category picker: the writer chooses the *bucket*, and the bucket owns the
//!   glyph and the per-theme colour. `statuses::glyph` sets out why in full — a stored hex
//!   cannot clear WCAG against both of this app's themes, so a per-status colour would be
//!   a control that produces unreadable output for most of its range.
//!
//! Like the tag pane it needs generic-closure widgets (`ListView`) the `teksu!` DSL cannot
//! express, so it is a chained-builder module.

use std::rc::Rc;

use common::entities::StatusCategory;
use teksilo::core::styles::{ComboBoxVariant, TextInputVariant};
use teksilo::prelude::*;
use teksilo::tokens::{BorderRole, SurfaceRole};
use teksilo::widgets::{
    Button, ButtonVariant, ComboBox, Expand, FixedSize, HStack, IconButton, IconWidget, ListView,
    MinSize, Padding, Panel, PopoverButton, Shrinkable, Spacer, Switcher, TextInput, TextWidget,
    Toast, VStack, ValidationState,
};

use crate::models::LadderRow;
use crate::statuses::{Preset, StatusesViewModel};
use crate::toast_scope::ToastWorkExt;

const LIST_MIN_HEIGHT: f32 = 300.0;
const ROW_HEIGHT: f32 = 62.0;
const CATEGORY_MIN_WIDTH: f32 = 140.0;
/// Wide enough for a two-digit rung — Plume's ladder is eight, and a project that grew
/// past ten must not shift every glyph in the column rightwards.
const ORDINAL_WIDTH: f32 = 22.0;

fn add_glyph() -> IconWidget {
    IconWidget::from_svg_icon(teksilo::res!("assets/icons/add.svg")).icon_size(15.0)
}

/// Every category, in ladder-ish order — the order a writer would *build* a ladder in, so
/// the picker reads as a progression rather than as the enum's declaration order.
const CATEGORIES: [StatusCategory; 5] = [
    StatusCategory::Planned,
    StatusCategory::Drafting,
    StatusCategory::NeedsWork,
    StatusCategory::Revised,
    StatusCategory::Final,
];

/// What a category is called on screen.
///
/// Translated, unlike a rung's name: a category is the app's vocabulary, a rung's name is
/// the writer's. That is the whole two-level split, and it is the reason one of these is a
/// `tr!` and the other is a `lit!` two functions below.
pub(crate) fn category_label(c: StatusCategory) -> LocalizedString {
    match c {
        StatusCategory::Planned => tr!(status_category_planned()),
        StatusCategory::Drafting => tr!(status_category_drafting()),
        StatusCategory::NeedsWork => tr!(status_category_needs_work()),
        StatusCategory::Revised => tr!(status_category_revised()),
        StatusCategory::Final => tr!(status_category_final()),
    }
}

/// `vm` is threaded from the OPENING WINDOW's `WorkSession` rather than read off
/// `app_state`, the same discipline every other Work-scoped pane here follows and for the
/// reason `settings::content` records: an `app_state` lookup resolves to whichever Work's
/// session registered first, not this window's.
pub fn work_statuses_pane(ctx: &mut BuildContext, vm: &StatusesViewModel) -> impl Widget {
    VStack::new()
        .spacing(16.0)
        .child(add_row(ctx, vm))
        .child(Expand::horizontal().child(LadderList {
            vm: vm.clone(),
            root_child: None,
        }))
}

/// Name field + category + "Apply a preset…" + Add.
///
/// The category picker sits in the add row rather than defaulting silently, because the
/// category is what gives a new rung its glyph: a rung added without one would arrive
/// wearing the first bucket's mark and look like a mistake the writer has to go and undo.
fn add_row(ctx: &mut BuildContext, vm: &StatusesViewModel) -> impl Widget {
    let text = Signal::new(String::new());
    // Owned, not derived: `.validation()` treats a bound signal as a shared *write* target,
    // and a mapped signal is lazy and read-only.
    let validation = Signal::new(ValidationState::None);
    let category = Signal::new(StatusCategory::Drafting);

    {
        // Live as the writer types. `TextInput` has no change hook, so the warning is
        // pushed from an effect — the same imperative shape the tag pane uses.
        let vm = vm.clone();
        let validation = validation.clone();
        ctx.effect(&text, move |typed| {
            validation.set(match vm.duplicate_name(typed, None) {
                Some(existing) => {
                    ValidationState::Warning(tr!(settings_statuses_duplicate(name = existing)))
                }
                None => ValidationState::None,
            });
        });
    }

    let commit = {
        let vm = vm.clone();
        let text = text.clone();
        let validation = validation.clone();
        let category = category.clone();
        move |ctx: &mut EventContext| {
            let name = text.get().trim().to_string();
            if name.is_empty() {
                return;
            }
            // Refused, not merely warned about, on the button path: two rungs with one
            // name make the ladder unreadable in the picker, where the name is all there
            // is to tell them apart.
            if let Some(existing) = vm.duplicate_name(&name, None) {
                ctx.show_toast(
                    Toast::warning(tr!(settings_statuses_duplicate(name = existing)))
                        .scoped_id("statuses.duplicate", vm.work_id())
                        .target_work(vm.work_id()),
                );
                return;
            }
            if vm.create(&name, category.get(), "").is_some() {
                ctx.show_toast(
                    Toast::info(tr!(settings_statuses_added(name = name.clone())))
                        .scoped_id("statuses.added", vm.work_id())
                        .target_work(vm.work_id()),
                );
            }
            text.set(String::new());
            validation.set(ValidationState::None);
        }
    };

    let field = {
        let commit = commit.clone();
        TextInput::new(text.clone())
            .leading_slot(add_glyph().color(TextRole::Secondary))
            .placeholder(tr!(settings_statuses_add_placeholder()))
            .validation(validation.clone())
            .rich_tooltip_content(teksilo::widgets::tooltip::TooltipContent::new(
                "settings.statuses",
                tr!(settings_statuses_desc()),
            ))
            .on_submit_fn(move |ctx| commit(ctx))
    };

    let category_pick = {
        let selected = Signal::new(Some(StatusCategory::Drafting));
        let category = category.clone();
        ComboBox::from_items(CATEGORIES.to_vec(), selected, |c: &StatusCategory| {
            category_label(c.clone())
        })
        .variant(ComboBoxVariant::Plain)
        .on_select(move |c: &StatusCategory, _| category.set(c.clone()))
    };

    let can_add = text.map(|t| !t.trim().is_empty());
    let add_btn = Button::new(tr!(settings_statuses_add()))
        .variant(ButtonVariant::Filled)
        .enabled(can_add)
        .on_activate_fn(move |ctx| commit(ctx));

    HStack::new()
        .spacing(10.0)
        .child(Expand::horizontal().child(field))
        .child(
            Shrinkable::new()
                .min_width(CATEGORY_MIN_WIDTH)
                .child(category_pick),
        )
        .child(preset_button(vm))
        .child(add_btn)
}

/// The preset catalogue.
///
/// Unlike the tag palette's, applying one is **refused** over a ladder that already has
/// rungs rather than merging into it. A preset is an *order*, and merging two orders has no
/// right answer: the tag version can skip names it already has because a tag is identified
/// by its name and carries no position, while every rung's meaning is partly its place.
fn preset_button(vm: &StatusesViewModel) -> impl Widget {
    let mut menu = teksilo::widgets::MenuList::new();
    for preset in Preset::ALL {
        let vm = vm.clone();
        menu = menu.item(
            teksilo::widgets::MenuItem::new(preset.label()).on_activate_fn(move |c| {
                let added = vm.seed(preset);
                // Say what happened either way: a silent refusal over a non-empty ladder
                // reads as a broken menu item.
                let toast = if added > 0 {
                    Toast::info(tr!(settings_statuses_preset_applied(added = added as i64)))
                } else {
                    Toast::warning(tr!(settings_statuses_preset_refused()))
                };
                c.show_toast(
                    toast
                        .scoped_id("statuses.preset", vm.work_id())
                        .target_work(vm.work_id()),
                );
            }),
        );
    }

    PopoverButton::new(
        Button::new(tr!(settings_statuses_apply_preset())).variant(ButtonVariant::Plain),
    )
    .bare()
    .content(menu)
}

/// The bordered ladder, as a widget of its own.
///
/// A widget rather than a builder function for the reason `TagList` records: the rows have
/// to be re-derived when the ladder changes, and re-deriving means a `BindingLevel::Rebuild`
/// against an id — and `BuildContext::self_id()` inside the pane function is the *Settings
/// window*, not the pane, so binding there would throw away the add row's half-typed name
/// on every edit.
///
/// It also owns the `ListModel`. The view-model deliberately reads its ladder through
/// rather than caching one (see `StatusesViewModel`), so the list the `ListView` binds to is
/// built here and re-seeded on each rebuild — which is exactly what a rebuild is for.
struct LadderList {
    vm: StatusesViewModel,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for LadderList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LadderList").finish()
    }
}

impl Widget for LadderList {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.wire(ctx);

        // **Deliberately no `BindingLevel::Rebuild` on the ladder's revision.** The
        // `ListView` binds the live model instead, so a rename reconciles one row and
        // rebuilds one delegate. Rebuilding the whole list on every ladder change — which
        // is what a revision binding does — destroys and recreates every inline field,
        // including the one the writer just tabbed into: a blur-commit in row A would
        // silently swallow the first characters typed into row B.
        //
        // Nothing below goes stale as a result. `usage` is captured once and is right for
        // the life of this page (Settings is modal, so no item can change its rung while it
        // is open; a rung *added* here is new and correctly reads zero, and a deleted one
        // takes its row with it), and the position and "is this the last rung" both come
        // from the model at delegate-build time.
        let usage = Rc::new(self.vm.usage_counts());
        let model = self.vm.list_model();
        let empty_idx = self.vm.revision().map({
            let model = model.clone();
            move |_| usize::from(model.is_empty())
        });

        let list_vm = self.vm.clone();
        let list = ListView::from_source(model.clone(), move |i, row: &LadderRow, _selected| {
            Box::new(RungRowView {
                vm: list_vm.clone(),
                row: row.clone(),
                position: i,
                is_last: i + 1 >= model.len(),
                usage: usage.get(&row.id).copied().unwrap_or(0),
                root_child: None,
            })
        })
        .auto_item_height(ROW_HEIGHT);

        let card = Panel::new()
            .background(SurfaceRole::Content)
            .border_color(BorderRole::Default)
            .border_width(1.0)
            .corner_radius(8.0)
            .padding(0.0)
            .child(
                // The floor goes on the card, not the list: a `Switcher` reports its active
                // child's size, and a virtualised `ListView` given unbounded height inside
                // the pane's own scroll reports ~nothing.
                MinSize::new(0.0, LIST_MIN_HEIGHT).child(
                    Switcher::new(empty_idx)
                        .child(Expand::vertical().child(list))
                        .child(empty_state()),
                ),
            );
        let root = ctx.add(card);
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// One rung: position · glyph · name · category · move · delete, with the description under.
///
/// A real `Widget` rather than a builder function because a row needs a `BuildContext`:
/// `TextInput` writes only to its signal, so persisting an edit means observing that signal
/// from an effect, and `ctx` is the only place effects can be installed.
struct RungRowView {
    vm: StatusesViewModel,
    row: LadderRow,
    position: usize,
    is_last: bool,
    /// How many binder items wear this rung — quoted by the delete tooltip so the writer
    /// knows what they are about to unfile before they do it, not after.
    usage: usize,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for RungRowView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RungRowView")
            .field("name", &self.row.name)
            .finish()
    }
}

impl Widget for RungRowView {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let id = self.row.id;

        // The position, as a plain ordinal. This is the one number on the page that is not
        // stored anywhere: it is the relationship's index, which is the ladder.
        let ordinal = FixedSize::new().width(ORDINAL_WIDTH).child(
            TextWidget::new(lit!((self.position + 1).to_string()))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Secondary)
                .single_line(),
        );

        // The glyph, in its category's live role — the same mark this rung wears in the
        // picker, the Overview and the completion readout, so the writer is choosing the
        // thing they will actually see rather than a word that maps to it invisibly.
        let glyph = crate::statuses::status_glyph(&self.row.category);

        let name = Signal::new(self.row.name.clone());
        let validation = Signal::new(ValidationState::None);
        {
            let vm = self.vm.clone();
            let validation = validation.clone();
            ctx.effect(&name, move |typed| {
                // `Some(id)` excludes this row — a rung never collides with itself.
                validation.set(match vm.duplicate_name(typed, Some(id)) {
                    Some(existing) => {
                        ValidationState::Warning(tr!(settings_statuses_duplicate(name = existing)))
                    }
                    None => ValidationState::None,
                });
            });
        }

        // Commits on Enter **and on blur**, guarded against a no-op write. Both halves
        // matter: nothing about a bare inline field says it must be confirmed, so a rename
        // followed by closing Settings would otherwise be thrown away — and an unguarded
        // blur would push an identical value through the command stack (an undo step that
        // changes nothing) every time focus merely passed through.
        let name_field = {
            let vm = self.vm.clone();
            let sig = name.clone();
            let commit = move || {
                let typed = sig.get();
                let current = vm.rung(id).map(|r| r.name);
                // A blank name would leave a rung nothing can identify; keep the old one.
                if !typed.trim().is_empty() && current.as_deref() != Some(typed.as_str()) {
                    vm.rename(id, &typed);
                }
            };
            let on_blur = commit.clone();
            TextInput::new(name.clone())
                .variant(TextInputVariant::Bare)
                .validation(validation)
                .on_submit_fn(move |_c| commit())
                .on_blur_fn(move |_c| on_blur())
        };

        let category = {
            let vm = self.vm.clone();
            let selected = Signal::new(Some(self.row.category.clone()));
            ComboBox::from_items(CATEGORIES.to_vec(), selected, |c: &StatusCategory| {
                category_label(c.clone())
            })
            .variant(ComboBoxVariant::Plain)
            .on_select(move |c: &StatusCategory, _| vm.set_category(id, c.clone()))
        };

        let up = {
            let vm = self.vm.clone();
            IconButton::new(
                IconWidget::from_svg_icon(teksilo::res!("assets/icons/templates/move-up.svg"))
                    .icon_size(14.0),
            )
            .embedded()
            .enabled(self.position > 0)
            .tooltip(tr!(settings_statuses_move_up()))
            .on_activate_fn(move |_| vm.nudge(id, -1))
        };
        let down = {
            let vm = self.vm.clone();
            IconButton::new(
                IconWidget::from_svg_icon(teksilo::res!("assets/icons/templates/move-down.svg"))
                    .icon_size(14.0),
            )
            .embedded()
            .enabled(!self.is_last)
            .tooltip(tr!(settings_statuses_move_down()))
            .on_activate_fn(move |_| vm.nudge(id, 1))
        };

        let delete = {
            let vm = self.vm.clone();
            let name = self.row.name.clone();
            let usage = self.usage;
            IconButton::clear()
                .embedded()
                // The count is in the *tooltip*, before the click — a confirmation that
                // only tells you what you disturbed after you disturbed it is not a
                // confirmation. Undo covers the rest, which is why this is not a modal.
                .tooltip(if usage == 0 {
                    tr!(settings_statuses_delete(name = name.clone()))
                } else {
                    tr!(settings_statuses_delete_in_use(
                        name = name.clone(),
                        count = usage as i64
                    ))
                })
                .on_activate_fn(move |c| {
                    vm.delete(id);
                    c.show_toast(
                        Toast::info(if usage == 0 {
                            tr!(settings_statuses_deleted(name = name.clone()))
                        } else {
                            tr!(settings_statuses_deleted_in_use(
                                name = name.clone(),
                                count = usage as i64
                            ))
                        })
                        .scoped_id("statuses.deleted", vm.work_id())
                        .target_work(vm.work_id()),
                    );
                })
        };

        let details = Signal::new(self.row.details.clone());
        let details_field = {
            let vm = self.vm.clone();
            let sig = details.clone();
            let commit = move || {
                let typed = sig.get();
                let current = vm.rung(id).map(|r| r.details);
                // Blank IS meaningful here — it clears the description.
                if current.as_deref() != Some(typed.as_str()) {
                    vm.set_details(id, &typed);
                }
            };
            let on_blur = commit.clone();
            TextInput::new(details.clone())
                .variant(TextInputVariant::Bare)
                .placeholder(tr!(settings_statuses_details_placeholder()))
                .on_submit_fn(move |_c| commit())
                .on_blur_fn(move |_c| on_blur())
        };

        let body = Padding::symmetric(6.0, 10.0).child(
            VStack::new()
                .spacing(2.0)
                .child(
                    HStack::new()
                        .spacing(8.0)
                        .child(ordinal)
                        .child(glyph)
                        .child(Expand::horizontal().child(name_field))
                        .child(
                            Shrinkable::new()
                                .min_width(CATEGORY_MIN_WIDTH)
                                .child(category),
                        )
                        .child(up)
                        .child(down)
                        .child(delete),
                )
                .child(Padding::new(0.0, 0.0, 0.0, ORDINAL_WIDTH + 8.0).child(details_field)),
        );
        let root = ctx.add(body);
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// A project whose ladder is empty. Says what a ladder is *for* rather than only that
/// there isn't one — this is the one screen where the concept is introduced.
fn empty_state() -> impl Widget {
    Padding::symmetric(24.0, 24.0).child(
        VStack::new()
            .spacing(6.0)
            .child(
                TextWidget::new(tr!(settings_statuses_empty_title()))
                    .style(TextStyleRole::BodyBold),
            )
            .child(
                TextWidget::new(tr!(settings_statuses_empty_body()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .child(Spacer::new()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::app_ids::AppIds;

    /// Stands in for the Settings panel: it hosts the pane exactly as
    /// `settings::content::build` does, by calling it with its own `BuildContext`.
    struct Host {
        vm: StatusesViewModel,
        child: Option<WidgetId>,
    }

    impl std::fmt::Debug for Host {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Host").finish()
        }
    }

    impl Widget for Host {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let pane = work_statuses_pane(ctx, &self.vm);
            let id = ctx.add(pane);
            self.child = Some(id);
            vec![id]
        }

        fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
            self.child
                .and_then(|id| ctx.child_size(id, proposal))
                .map(LayoutResponse::from)
                .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
        }

        fn children(&self) -> Vec<WidgetId> {
            self.child.into_iter().collect()
        }
    }

    /// Count widgets below `id` whose type name satisfies `is_match`.
    ///
    /// The tree exposes no rendered text, so the shape of the pane is asserted through its
    /// controls — which is the stronger claim anyway: a rung the writer cannot *edit* is
    /// the failure worth catching.
    fn count(
        tree: &teksilo::core::widget_tree::WidgetTree,
        id: WidgetId,
        is_match: &dyn Fn(&str) -> bool,
    ) -> usize {
        let mine = usize::from(tree.widget_type_name(id).is_some_and(is_match));
        tree.children(id)
            .into_iter()
            .map(|c| count(tree, c, is_match))
            .sum::<usize>()
            + mine
    }

    /// A plain type, matched exactly. `contains` would double-count every one of these:
    /// a `TextInput` mounts an inner widget whose type name also contains `::TextInput`.
    fn plain(tree: &teksilo::core::widget_tree::WidgetTree, id: WidgetId, ty: &str) -> usize {
        let suffix = format!("::{ty}");
        count(tree, id, &|n| n.ends_with(&suffix))
    }

    /// A **generic** type. Its name ends in the parameter
    /// (`…::ComboBox<…StatusCategory>`), so an exact match silently counts zero of them —
    /// which reads exactly like a pane that rendered no rows.
    fn generic(tree: &teksilo::core::widget_tree::WidgetTree, id: WidgetId, ty: &str) -> usize {
        let open = format!("::{ty}<");
        count(tree, id, &|n| n.contains(&open))
    }

    fn lay_out() -> (
        teksilo::core::widget_tree::WidgetTree,
        WidgetId,
        StatusesViewModel,
    ) {
        let app_ctx = std::rc::Rc::new(frontend::AppContext::new());
        let vm = StatusesViewModel::new(app_ctx.clone(), AppIds::new());
        let mut tree = crate::test_support::tree_with_settings(&app_ctx);
        let root = tree.add_boxed(Box::new(Host {
            vm: vm.clone(),
            child: None,
        }));
        tree.layout(SizeProposal::with_width(720.0));
        (tree, root, vm)
    }

    /// The pane builds and lays out to a real size.
    ///
    /// Under `mocks` the ladder is the fabricated four-rung one, so the rows are on screen;
    /// under the real backend there is no project, so the empty state is — and **both** have
    /// to survive a layout, because the empty one is what a writer sees the first time they
    /// open the page.
    #[test]
    fn the_pane_lays_out() {
        let (tree, root, _vm) = lay_out();
        let bounds = tree.bounds(root);
        assert!(
            bounds.width > 0.0 && bounds.height > 0.0,
            "the pane collapsed to {bounds:?}"
        );
        // The add row is always there, ladder or not: it is the only way to start one.
        assert!(
            plain(&tree, root, "TextInput") >= 1,
            "no add field on the page"
        );
    }

    /// **Every rung gets its own editable row**, with the three controls that make the
    /// ladder editable at all: a name field, a category picker, and move/delete buttons.
    ///
    /// Counted rather than read, because the widget tree exposes no rendered text. The
    /// arithmetic is the point: one `ComboBox` per rung *plus one* for the add row, so a
    /// pane that rendered the add row and no rungs — the shape this feature had before —
    /// would read 1 and fail.
    #[cfg(feature = "mocks")]
    #[test]
    fn every_rung_gets_an_editable_row() {
        let (tree, root, vm) = lay_out();
        let rungs = vm.ladder().len();
        assert_eq!(rungs, 4, "the fabricated ladder is four rungs");
        assert_eq!(
            generic(&tree, root, "ComboBox"),
            rungs + 1,
            "one category picker per rung, plus the add row's"
        );
        assert_eq!(
            plain(&tree, root, "TextInput"),
            rungs * 2 + 1,
            "name + description per rung, plus the add row's name field"
        );
        // Move up, move down, delete — the three that make the order and the membership
        // the writer's rather than the app's.
        assert!(
            plain(&tree, root, "IconButton") >= rungs * 3,
            "a rung is missing its move or delete control"
        );
    }

    /// A project with no ladder shows the empty state and **no rung rows** — not a table
    /// of blank ones.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn an_empty_ladder_shows_no_rung_rows() {
        let (tree, root, vm) = lay_out();
        assert!(vm.ladder().is_empty(), "no project, so no ladder");
        assert_eq!(
            generic(&tree, root, "ComboBox"),
            1,
            "only the add row's category picker"
        );
    }

    /// **The picker offers one entry per app-owned category.**
    ///
    /// Pinned rather than derived, the same way `skribisto_model::COMBINATIONS` pins its
    /// own length: a category missing from `CATEGORIES` is a glyph and a colour no writer
    /// can ever reach, and nothing else in the build would notice. Adding a
    /// `StatusCategory` variant already breaks `category_label`'s exhaustive match — this
    /// is the reminder that the array and both `.ftl` files need the same entry.
    #[test]
    fn the_category_picker_offers_every_bucket() {
        assert_eq!(CATEGORIES.len(), 5, "one entry per StatusCategory variant");
        let mut labels: Vec<String> = CATEGORIES
            .iter()
            .map(|c| category_label(c.clone()).resolve_now())
            .collect();
        labels.sort();
        labels.dedup();
        assert_eq!(
            labels.len(),
            CATEGORIES.len(),
            "two categories share a label, so the picker cannot tell them apart"
        );
        assert!(
            labels.iter().all(|l| !l.trim().is_empty()),
            "a category with no label is an unpickable row"
        );
    }
}
