// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Work ▸ **Text replacements** — the per-project custom lexicon
//! manager ("btw" → "by the way", "--" → "—"), gated behind a leading
//! per-project master switch (`Work.custom_replacement_rules_enabled`).
//!
//! Shaped like the tag palette pane next to it: a description, the master
//! switch, then — only while it is on — a prominent two-field add row, a
//! filter + live count + Import…/Export… toolbar, and a bordered list. Each
//! row edits one rule in place: trigger, replacement, an enabled toggle
//! (deactivate without deleting), and delete.
//!
//! Like the tag palette and dictionary panes it needs generic-closure widgets
//! (`ListView`) the `bati!` DSL can't express, so it is a chained-builder
//! module.

use bastyde::core::styles::TextInputVariant;
use bastyde::data::SortFilterListModel;
use bastyde::prelude::*;
use bastyde::res;
use bastyde::tokens::{BorderRole, SurfaceRole};
use bastyde::widgets::{
    BuiltInIcons, Button, ButtonVariant, Center, Expand, FixedSize, HStack, IconButton,
    IconLocation, IconWidget, ListView, MaxSize, MinSize, Padding, Panel, SearchField, Spacer,
    Switcher, TextInput, TextWidget, Toast, Toggle, VStack, ValidationState,
};

use crate::models::TextReplacementRuleRow;
use crate::toast_scope::ToastWorkExt;
use crate::view_models::TextReplacementRulesViewModel;

const TRIGGER_COL: &str = "trigger";
const TRIGGER_FIELD_WIDTH: f32 = 140.0;
const FILTER_FIELD_MAX_WIDTH: f32 = 260.0;
const LIST_MIN_HEIGHT: f32 = 320.0;

fn add_glyph() -> IconWidget {
    (BuiltInIcons::defaults().add)().icon_size(15.0)
}
fn import_glyph() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/settings/import.svg")).icon_size(15.0)
}
fn export_glyph() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/settings/export.svg")).icon_size(15.0)
}

/// The whole pane body. The caller wraps it in `pane_frame`.
pub fn text_replacements_pane(
    ctx: &mut BuildContext,
    vm: &TextReplacementRulesViewModel,
) -> impl Widget {
    // The master switch is bridged to `Work.custom_replacement_rules_enabled` with the same
    // two-effect shape `work_structure_pane` uses for `chapter_mode`: one mirrors external
    // changes (an undo, a project switch) into the local toggle signal, the other pushes a
    // user toggle back through the write-and-persist call, which owns its own no-op guard.
    let enabled_source = vm.enabled_signal();
    let on: Signal<bool> = Signal::new(enabled_source.get());
    {
        let on = on.clone();
        ctx.effect(&enabled_source, move |v| {
            if on.get() != *v {
                on.set(*v);
            }
        });
    }
    {
        let vm = vm.clone();
        ctx.effect(&on, move |v| vm.set_enabled(*v));
    }

    let toggle = Toggle::new(on.clone()).label(tr!(settings_text_repl_enable()));

    // Only the active body needs the `BuildContext` for its own effects, but `Switcher`
    // builds both children regardless of which is shown (the same shape every empty-state
    // Switcher here uses) — cheap for a settings-sized list.
    let content = Switcher::new(on.map(|v| usize::from(*v)))
        .child(disabled_hint())
        .child(active_body(ctx, vm));

    VStack::new()
        .spacing(16.0)
        .child(TextWidget::new(tr!(settings_text_repl_desc())).color(TextRole::Secondary))
        .child(toggle)
        .child(content)
}

/// Shown while the master switch is off: no add row, no list — just what
/// turning it on will do.
fn disabled_hint() -> impl Widget {
    Center::new().child(
        Padding::symmetric(0.0, 24.0).child(
            TextWidget::new(tr!(settings_text_repl_disabled_hint()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        ),
    )
}

/// The add row, toolbar and bordered list — everything that only makes sense
/// once the lexicon is active.
fn active_body(ctx: &mut BuildContext, vm: &TextReplacementRulesViewModel) -> impl Widget {
    let filtered = SortFilterListModel::new(vm.list_model()).with_predicate(TRIGGER_COL, |text| {
        let needle = text.trim().to_lowercase();
        Box::new(move |row: &TextReplacementRuleRow| {
            needle.is_empty()
                || row.trigger.to_lowercase().contains(&needle)
                || row.replacement.to_lowercase().contains(&needle)
        })
    });
    let query = Signal::new(String::new());
    {
        // Pushed imperatively — `filters_signal` would `observe` a derived signal.
        let filter_view = filtered.clone();
        ctx.effect(&query, move |q| filter_view.set_filter(TRIGGER_COL, q));
    }

    let list_vm = vm.clone();
    let list = ListView::from_source(
        filtered,
        move |_i, row: &TextReplacementRuleRow, _selected| {
            Box::new(RuleRowView {
                vm: list_vm.clone(),
                row: row.clone(),
                root_child: None,
            })
        },
    )
    .auto_item_height(46.0);

    let empty_idx = {
        let vm = vm.clone();
        vm.changed_signal().map(move |_| usize::from(vm.is_empty()))
    };
    let list_card = Panel::new()
        .background(SurfaceRole::Content)
        .border_color(BorderRole::Default)
        .border_width(1.0)
        .corner_radius(8.0)
        .padding(0.0)
        .child(
            // The floor goes on the card, not the list: a `Switcher` reports its active
            // child's size, and a virtualised `ListView` given unbounded height inside the
            // pane's own scroll reports ~nothing.
            MinSize::new(0.0, LIST_MIN_HEIGHT).child(
                Switcher::new(empty_idx)
                    .child(Expand::vertical().child(list))
                    .child(empty_state()),
            ),
        );

    VStack::new()
        .spacing(16.0)
        .child(add_row(ctx, vm))
        .child(toolbar_row(vm, query))
        .child(Expand::horizontal().child(list_card))
}

/// The prominent add row: trigger field, an arrow, the replacement field, and a filled button.
fn add_row(ctx: &mut BuildContext, vm: &TextReplacementRulesViewModel) -> impl Widget {
    let trigger = Signal::new(String::new());
    let replacement = Signal::new(String::new());
    // Owned rather than derived: `.validation()` treats a bound signal as a shared *write*
    // target, and a `.map()` signal is lazy and read-only.
    let validation = Signal::new(ValidationState::None);

    {
        // Live as the writer types, not on commit — the same imperative-push shape the
        // filter and the tag-name warning use.
        let vm = vm.clone();
        let validation = validation.clone();
        ctx.effect(&trigger, move |typed| {
            validation.set(match vm.duplicate_trigger(typed, None) {
                Some(existing) => {
                    ValidationState::Warning(tr!(settings_text_repl_duplicate(trigger = existing)))
                }
                None => ValidationState::None,
            });
        });
    }

    let commit = {
        let vm = vm.clone();
        let trigger = trigger.clone();
        let replacement = replacement.clone();
        let validation = validation.clone();
        move |ctx: &mut EventContext| {
            let t = trigger.get().trim().to_string();
            if t.is_empty() {
                return;
            }
            // Only clear on a real add. The Add *button* is gated by `can_add`, but
            // Enter in either field is not gated by anything — so a writer who types
            // an existing trigger and hits Enter used to lose both fields with no
            // explanation. The inline warning is already on screen saying why; leaving
            // the text in place is what lets them act on it.
            if vm.create(&t, &replacement.get(), true).is_none() {
                return;
            }
            ctx.show_toast(
                Toast::info(tr!(settings_text_repl_added(trigger = t.clone())))
                    .scoped_id("text_repl.added", vm.work_id())
                    .target_work(vm.work_id()),
            );
            trigger.set(String::new());
            replacement.set(String::new());
            validation.set(ValidationState::None);
        }
    };

    let trigger_field = {
        let commit = commit.clone();
        TextInput::new(trigger.clone())
            .leading_slot(add_glyph().color(TextRole::Secondary))
            .placeholder(tr!(settings_text_repl_trigger_placeholder()))
            .validation(validation.clone())
            .on_submit_fn(move |ctx| commit(ctx))
    };
    let replacement_field = {
        let commit = commit.clone();
        TextInput::new(replacement.clone())
            .placeholder(tr!(settings_text_repl_replacement_placeholder()))
            .on_submit_fn(move |ctx| commit(ctx))
    };

    let can_add = {
        let vm = vm.clone();
        trigger
            .zip(&vm.changed_signal())
            .map(move |(t, _)| vm.can_add(t))
    };
    let add_btn = Button::new(tr!(settings_text_repl_add()))
        .variant(ButtonVariant::Filled)
        .enabled(can_add)
        .on_activate_fn(move |ctx| commit(ctx));

    HStack::new()
        .spacing(10.0)
        .child(
            FixedSize::new()
                .width(TRIGGER_FIELD_WIDTH)
                .child(trigger_field),
        )
        .child(TextWidget::new(lit!("→")).color(TextRole::Secondary))
        .child(Expand::horizontal().child(replacement_field))
        .child(add_btn)
}

/// Filter + live count on the left, Import…/Export… pushed to the right.
fn toolbar_row(vm: &TextReplacementRulesViewModel, query: Signal<String>) -> impl Widget {
    let count = {
        let vm = vm.clone();
        vm.changed_signal()
            .map(move |_| tr!(settings_text_repl_count(n = vm.rows().len() as i64)).resolve_now())
    };

    HStack::new()
        .spacing(10.0)
        .child(
            MaxSize::width(FILTER_FIELD_MAX_WIDTH)
                .child(SearchField::new(query).placeholder(tr!(settings_text_repl_filter()))),
        )
        .child(
            TextWidget::new(lit!(""))
                .text(count)
                .color(TextRole::Secondary),
        )
        .child(Expand::horizontal().child(Spacer::new()))
        .child(import_button(vm))
        .child(export_button(vm))
}

fn import_button(vm: &TextReplacementRulesViewModel) -> impl Widget {
    let vm = vm.clone();
    Button::new(tr!(settings_text_repl_import()))
        .variant(ButtonVariant::Plain)
        .icon(import_glyph(), IconLocation::Leading)
        .on_activate_fn(move |ctx| {
            let vm = vm.clone();
            let req = FileDialogRequest::pick_file()
                .title(tr!(settings_text_repl_import()))
                .add_filter(tr!(settings_text_repl_csv_filter()).resolve_now(), &["csv"]);
            let _ = ctx.pick_file(req, move |res, c| {
                if let FileDialogResult::File(Some(path)) = res {
                    match vm.import_from(&path) {
                        Ok(s) => {
                            c.show_toast(
                                Toast::info(tr!(settings_text_repl_imported(
                                    added = s.added as i64,
                                    skipped = (s.duplicates + s.malformed) as i64
                                )))
                                .scoped_id("text_repl.imported", vm.work_id())
                                .target_work(vm.work_id()),
                            );
                        }
                        Err(e) => {
                            c.show_toast(
                                Toast::info(lit!(format!("{e:#}")))
                                    .scoped_id("text_repl.error", vm.work_id())
                                    .target_work(vm.work_id()),
                            );
                        }
                    }
                }
            });
        })
}

fn export_button(vm: &TextReplacementRulesViewModel) -> impl Widget {
    let vm = vm.clone();
    Button::new(tr!(settings_text_repl_export()))
        .variant(ButtonVariant::Plain)
        .icon(export_glyph(), IconLocation::Leading)
        .on_activate_fn(move |ctx| {
            let vm = vm.clone();
            let req = FileDialogRequest::save_file()
                .title(tr!(settings_text_repl_export()))
                .default_file_name("text-replacements.csv".to_string())
                .add_filter(tr!(settings_text_repl_csv_filter()).resolve_now(), &["csv"]);
            let _ = ctx.save_file(req, move |res, c| {
                if let FileDialogResult::Saved(Some(mut path)) = res {
                    if path.extension().and_then(|e| e.to_str()) != Some("csv") {
                        path.set_extension("csv");
                    }
                    match vm.export_to(&path) {
                        Ok(n) => {
                            c.show_toast(
                                Toast::info(tr!(settings_text_repl_exported(n = n as i64)))
                                    .scoped_id("text_repl.exported", vm.work_id())
                                    .target_work(vm.work_id()),
                            );
                        }
                        Err(e) => {
                            c.show_toast(
                                Toast::info(lit!(format!("{e:#}")))
                                    .scoped_id("text_repl.error", vm.work_id())
                                    .target_work(vm.work_id()),
                            );
                        }
                    }
                }
            });
        })
}

/// One lexicon row: trigger, an arrow, replacement, an enabled toggle, delete.
///
/// A real `Widget` rather than a plain builder function, for the same reason
/// `TagRowView` is: `Toggle` and `TextInput` both write only to their
/// signals, so persisting an edit means observing those signals from an
/// effect, and `ctx` is the only place effects can be installed.
struct RuleRowView {
    vm: TextReplacementRulesViewModel,
    row: TextReplacementRuleRow,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for RuleRowView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuleRowView")
            .field("trigger", &self.row.trigger)
            .finish()
    }
}

impl Widget for RuleRowView {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let id = self.row.id;

        // Trigger, with the duplicate warning live as it is typed.
        let trigger = Signal::new(self.row.trigger.clone());
        let validation = Signal::new(ValidationState::None);
        {
            let vm = self.vm.clone();
            let validation = validation.clone();
            ctx.effect(&trigger, move |typed| {
                // `Some(id)` excludes this row — a rule never collides with itself.
                validation.set(match vm.duplicate_trigger(typed, Some(id)) {
                    Some(existing) => ValidationState::Warning(tr!(settings_text_repl_duplicate(
                        trigger = existing
                    ))),
                    None => ValidationState::None,
                });
            });
        }
        // Both fields commit on Enter **and on blur** — the same reasoning as
        // `TagRowView`'s name field: nothing about a bare inline field says it must be
        // confirmed, so tabbing away or closing Settings must not silently discard it.
        // Each commit compares against the model first so an unmodified pass through the
        // field doesn't push a no-op undo step.
        let trigger_field = {
            let vm = self.vm.clone();
            let sig = trigger.clone();
            let commit = move || {
                let typed = sig.get();
                let current = vm
                    .rows()
                    .into_iter()
                    .find(|r| r.id == id)
                    .map(|r| r.trigger);
                if current.as_deref() == Some(typed.as_str()) {
                    return;
                }
                // Blank or already taken: the view-model refuses it (two rules on one
                // trigger is incoherent, not merely untidy). Put the stored trigger back
                // in the field rather than leaving the rejected text sitting there — an
                // inline field that keeps showing what was NOT saved is how a writer ends
                // up believing a rule exists under a trigger it does not.
                if !vm.set_trigger(id, &typed)
                    && let Some(stored) = current
                {
                    sig.set(stored);
                }
            };
            let on_blur = commit.clone();
            TextInput::new(trigger.clone())
                .variant(TextInputVariant::Bare)
                .validation(validation)
                .on_submit_fn(move |_c| commit())
                .on_blur_fn(move |_c| on_blur())
        };

        let replacement = Signal::new(self.row.replacement.clone());
        let replacement_field = {
            let vm = self.vm.clone();
            let sig = replacement.clone();
            let commit = move || {
                let typed = sig.get();
                let current = vm
                    .rows()
                    .into_iter()
                    .find(|r| r.id == id)
                    .map(|r| r.replacement);
                // Blank IS meaningful here — it makes the rule delete the trigger outright.
                if current.as_deref() != Some(typed.as_str()) {
                    vm.set_replacement(id, &typed);
                }
            };
            let on_blur = commit.clone();
            TextInput::new(replacement.clone())
                .variant(TextInputVariant::Bare)
                .on_submit_fn(move |_c| commit())
                .on_blur_fn(move |_c| on_blur())
        };

        // The toggle writes only to its signal, so the write-back rides an effect, guarded
        // against echoing its own write back in on the next refresh.
        let enabled = Signal::new(self.row.enabled);
        {
            let vm = self.vm.clone();
            ctx.effect(&enabled, move |on| {
                let current = vm
                    .rows()
                    .into_iter()
                    .find(|r| r.id == id)
                    .map(|r| r.enabled);
                if current != Some(*on) {
                    vm.set_rule_enabled(id, *on);
                }
            });
        }
        // `.label(..)`, not `.tooltip(..)`: a tooltip is not an accessible name, and
        // `Toggle` asserts on a nameless switch — a screen reader would otherwise
        // announce bare "switch" for every row. It crashed the pane outright the
        // first time a rule existed to render.
        let toggle = Toggle::new(enabled).label(tr!(settings_text_repl_row_enabled()));

        let delete = {
            let vm = self.vm.clone();
            let trigger_label = self.row.trigger.clone();
            IconButton::clear()
                .embedded()
                .tooltip(tr!(settings_text_repl_delete(
                    trigger = trigger_label.clone()
                )))
                .on_activate_fn(move |c| {
                    vm.delete(&[id]);
                    c.show_toast(
                        Toast::info(tr!(settings_text_repl_deleted(
                            trigger = trigger_label.clone()
                        )))
                        .scoped_id("text_repl.deleted", vm.work_id())
                        .target_work(vm.work_id()),
                    );
                })
        };

        let body = Padding::symmetric(6.0, 12.0).child(
            HStack::new()
                .spacing(8.0)
                .child(
                    FixedSize::new()
                        .width(TRIGGER_FIELD_WIDTH)
                        .child(trigger_field),
                )
                .child(TextWidget::new(lit!("→")).color(TextRole::Secondary))
                .child(Expand::horizontal().child(replacement_field))
                .child(toggle)
                .child(delete),
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

/// Shown instead of an empty bordered box, which reads as broken.
fn empty_state() -> impl Widget {
    Center::new().child(
        VStack::new().spacing(4.0).child(
            TextWidget::new(tr!(settings_text_repl_empty()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        ),
    )
}

/// Headless layout tests for the pane and its rows.
///
/// These exist because of a real crash: `RuleRowView`'s enable switch was built
/// with `.tooltip(..)` and no `.label(..)`, and `Toggle::accessibility` asserts
/// on a switch with no accessible name. Nothing in the suite ever *built* a row,
/// so every engine, view-model and live-editor test passed while the pane took
/// the whole app down the moment a project had one rule to render.
///
/// The lesson generalises past that one widget: a11y assertions, missing
/// `BuildContext` wiring and layout panics only fire when a widget is actually
/// mounted and laid out. So mount them.
#[cfg(all(test, feature = "mocks"))]
mod tests {
    use std::rc::Rc;

    use bastyde::prelude::SizeProposal;
    use frontend::AppContext;

    use super::*;
    use crate::app_ids::AppIds;
    use crate::models::TextReplacementRuleListModel;
    use crate::singles::SingleWork;

    fn vm(enabled: bool) -> (Rc<AppContext>, TextReplacementRulesViewModel) {
        let ctx = Rc::new(AppContext::new());
        let work = SingleWork::new(ctx.clone());
        work.set_custom_replacement_rules_enabled(enabled);
        let ids = AppIds::new();
        let vm = TextReplacementRulesViewModel::new(
            TextReplacementRuleListModel::new(ctx.clone(), ids.clone()),
            work,
            ids,
        );
        (ctx, vm)
    }

    /// A row must mount and lay out. This is the test that fails — by panicking
    /// inside `Toggle` — if the enable switch ever loses its accessible label
    /// again.
    #[test]
    fn a_rule_row_mounts_and_lays_out() {
        let (ctx, vm) = vm(true);
        let row = vm
            .rows()
            .into_iter()
            .next()
            .expect("the mock lexicon ships rules");
        let mut tree = crate::test_support::tree_with_settings(&ctx);
        tree.add(RuleRowView {
            vm: vm.clone(),
            row,
            root_child: None,
        });
        tree.layout(SizeProposal::exact(760.0, 46.0));
        // The assertion lives in `Toggle::accessibility`, which only runs when
        // the AccessKit tree is built — laying out alone would not reach it.
        tree.sync_accessibility();
    }

    /// Every row, not just the first: the mock lexicon carries an enabled rule, a
    /// caseless one and a disabled one, and a row's own state feeds its widgets.
    #[test]
    fn every_rule_row_mounts() {
        let (ctx, vm) = vm(true);
        for row in vm.rows() {
            let mut tree = crate::test_support::tree_with_settings(&ctx);
            tree.add(RuleRowView {
                vm: vm.clone(),
                row: row.clone(),
                root_child: None,
            });
            tree.layout(SizeProposal::exact(760.0, 46.0));
            tree.sync_accessibility();
        }
    }

    /// Hosts the pane so it can be mounted: `text_replacements_pane` needs a
    /// `&mut BuildContext`, which only exists inside a `Widget::build`.
    struct PaneHost {
        vm: Option<TextReplacementRulesViewModel>,
        root_child: Option<WidgetId>,
    }

    impl std::fmt::Debug for PaneHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PaneHost").finish()
        }
    }

    impl Widget for PaneHost {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let vm = self.vm.take().expect("built once");
            let body = text_replacements_pane(ctx, &vm);
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

    /// Mount the pane and return how many text inputs the accessibility tree
    /// exposes — the add row contributes two (trigger + replacement), so this is
    /// a direct read of whether the master switch actually gated the body.
    fn mount_pane_inputs(enabled: bool) -> usize {
        use bastyde::core::accesskit::Role;
        let (ctx, vm) = vm(enabled);
        let mut tree = crate::test_support::tree_with_settings(&ctx);
        tree.add(PaneHost {
            vm: Some(vm),
            root_child: None,
        });
        tree.layout(SizeProposal::exact(760.0, 520.0));
        let update = tree.sync_accessibility();
        update
            .nodes
            .iter()
            .filter(|(_, n)| n.role() == Role::TextInput)
            .count()
    }

    fn mount_pane(enabled: bool) {
        let _ = mount_pane_inputs(enabled);
    }

    /// Flipping the switch on an ALREADY-MOUNTED pane must re-gate the body.
    ///
    /// The build-time test above passes even when the `Switcher` is stuck,
    /// because it mounts a fresh pane per state. This is the path a writer takes:
    /// open the pane, click the switch, and expect the add row to appear or go
    /// away under them.
    #[test]
    fn flipping_the_master_switch_re_gates_a_mounted_pane() {
        use bastyde::core::accesskit::Role;
        let (ctx, vm) = vm(true);
        let mut tree = crate::test_support::tree_with_settings(&ctx);
        tree.add(PaneHost {
            vm: Some(vm.clone()),
            root_child: None,
        });
        tree.layout(SizeProposal::exact(760.0, 520.0));

        let inputs = |tree: &mut bastyde::core::widget_tree::WidgetTree| {
            tree.sync_accessibility()
                .nodes
                .iter()
                .filter(|(_, n)| n.role() == Role::TextInput)
                .count()
        };
        assert!(
            inputs(&mut tree) >= 2,
            "starts on, so the add row is present"
        );

        vm.set_enabled(false);
        tree.layout(SizeProposal::exact(760.0, 520.0));
        assert_eq!(
            inputs(&mut tree),
            0,
            "switching off must take the add row away, not leave it editable"
        );

        vm.set_enabled(true);
        tree.layout(SizeProposal::exact(760.0, 520.0));
        assert!(
            inputs(&mut tree) >= 2,
            "switching back on must bring the add row back"
        );
    }

    /// The master switch must actually gate the body, in BOTH directions. The
    /// `Switcher` is driven by a mapped signal off the toggle, and a stuck one
    /// would leave the add row and the list on screen for a project that has the
    /// feature switched off — offering edits to a lexicon that will never fire.
    #[test]
    fn the_master_switch_gates_the_add_row() {
        assert_eq!(
            mount_pane_inputs(false),
            0,
            "switched off, the pane must show only the hint — no add row"
        );
        assert!(
            mount_pane_inputs(true) >= 2,
            "switched on, the add row's trigger and replacement fields must be present"
        );
    }

    /// The whole pane, switched on — the add row, the toolbar and a populated
    /// list, which is the state a writer actually sees.
    #[test]
    fn the_pane_mounts_with_the_switch_on() {
        mount_pane(true);
    }

    /// And switched off, where the `Switcher` shows the explanatory hint instead.
    #[test]
    fn the_pane_mounts_with_the_switch_off() {
        mount_pane(false);
    }
}
