// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `FocusStrip` — distraction-free mode's always-visible control strip: word
//! count + writing session + Go Next/Previous + Exit, replacing the normal
//! status bar's content while the mode is active.
//!
//! **Always visible, never hover-reveal** — this app's documented EN 301 549
//! / RGAA accessibility posture rules out a hover-only strip: other writing
//! tools have filed bugs where a hover-only control panel cannot be brought
//! back once dismissed. The strip exists only while the mode's surface
//! does (`distraction_free::surface` builds it), never on hover.
//!
//! `WordCountIndicator` and `SessionStatusItem` drop in unchanged, over the
//! *same* live `StatsModel`/`WritingSessionViewModel` the normal status bar
//! uses.
//!
//! **Every item here is optional except Exit** ([`FocusStripChrome`], bound to
//! the Settings ▸ Editor ▸ Distraction-free checkboxes). Exit is not a setting
//! and must not become one: it is this strip's documented way out, and the
//! mode's other exits are a keystroke the focused editor may legitimately
//! swallow (Escape) and one the writer has to remember (Shift+F11).
//!
//! **Go reuse, not reimplementation** — the Previous/Next icon buttons fire the
//! same `go.prev`/`go.next` named actions the Go menu and the Alt+Up/Alt+Down
//! shortcut already drive (`app/commands/go.rs`), the same way Exit fires
//! `view.focus_mode` rather than duplicating `FocusViewModel::toggle`.

use teksilo::core::binding::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, IconButton, IconButtonSize, MaxSize, StatusBar,
    TextWidget, Toolbar, ToolbarAction, ToolbarItem, VStack,
};

use crate::models::StatsModel;

/// How much of the bar the item name may take before it elides.
const TITLE_MAX_WIDTH: f32 = 260.0;

// Overflow priorities — **lowest collapses first**. The two readouts keep the
// implicit 0, so they go before either navigation control; between themselves
// the later-declared (the writing session) goes first.
const GO_TO_PRIORITY: i32 = 1;
const GO_PRIORITY: i32 = 2;
use crate::settings::SettingsViewModel;
use crate::statusbar::session_status_item::SessionStatusItem;
use crate::statusbar::word_count_indicator::WordCountIndicator;
use crate::writing_session::WritingSessionViewModel;

/// Which of the strip's optional items are shown, one live setting signal each.
///
/// Exit has no field here on purpose — see this module's doc. The tab strip's
/// own toggle is *not* here either: it gates a `TabWidget` up in `App::build`,
/// not a child of this strip.
#[derive(Clone)]
pub struct FocusStripChrome {
    /// The current item's name. With the tab strip and the title bar both gone
    /// and a plain Scene carrying no title field in its pane, this is the only
    /// thing on screen that says *what* you are writing — while Alt+Up and
    /// Alt+Down step between scenes.
    pub title: Signal<bool>,
    pub word_count: Signal<bool>,
    pub session: Signal<bool>,
    pub go: Signal<bool>,
    /// The "Go to…" jump button. Its own key, not folded into `go`: the arrows
    /// step relative to where you are and this jumps anywhere, so a writer may
    /// well want one without the other.
    pub go_to: Signal<bool>,
}

impl FocusStripChrome {
    /// Read the three live setting signals off the settings view-model.
    pub fn from_settings(settings: &SettingsViewModel) -> Self {
        Self {
            title: settings.distraction_free_title(),
            word_count: settings.distraction_free_word_count(),
            session: settings.distraction_free_session(),
            go: settings.distraction_free_go(),
            go_to: settings.distraction_free_go_to(),
        }
    }

    /// Detached signals with every item shown — for headless tests that only
    /// care about the strip's other behaviour.
    #[cfg(test)]
    pub fn all_shown() -> Self {
        Self {
            title: Signal::new(true),
            word_count: Signal::new(true),
            session: Signal::new(true),
            go: Signal::new(true),
            go_to: Signal::new(true),
        }
    }
}

pub struct FocusStrip {
    go_to_vm: crate::go::GoToViewModel,
    /// Settings + the theme library, for the quick-access popover. `None` in the
    /// widget tests, which build a strip with no app around it.
    quick: Option<(
        crate::settings::SettingsViewModel,
        crate::distraction_free::DistractionFreeThemesViewModel,
    )>,
    /// The current item's name, live off a `SingleBinderItem` so a rename made
    /// from inside the mode reaches the strip.
    title: Signal<String>,
    stats: StatsModel,
    session_vm: WritingSessionViewModel,
    has_work: Signal<bool>,
    show_characters: Signal<bool>,
    /// The project's target unit and a store handle — the two things
    /// [`WordCountIndicator`] needs to draw its target bar. Threaded through rather than
    /// resolved here, so the strip's copy of the indicator is the same widget the status
    /// bar builds and shows the same bar.
    goal_unit: Signal<frontend::common::entities::GoalUnit>,
    app_ctx: std::rc::Rc<frontend::AppContext>,
    chrome: FocusStripChrome,
    /// Whether the synopsis shows inside the mode — this window's own flag, not
    /// the global preference (see `FocusViewModel::synopsis_visible_signal`).
    synopsis_visible: Signal<bool>,
    /// Whether the open item even *has* a toggleable synopsis. The one control
    /// here that is disabled rather than absent: unlike the four chrome-gated
    /// gadgets it is not a preference the writer turned off, it is a thing this
    /// particular document cannot do — and a button that vanished as you navigated
    /// between a scene and a folder would read as the strip glitching.
    synopsis_capable: Signal<bool>,
    root_child: Option<WidgetId>,
}

impl FocusStrip {
    // Each argument is a distinct live model this strip binds — bundling them
    // into a config struct would add a type whose only job is to be
    // destructured immediately. Same call as `writing_column`'s.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        go_to_vm: crate::go::GoToViewModel,
        quick: Option<(
            crate::settings::SettingsViewModel,
            crate::distraction_free::DistractionFreeThemesViewModel,
        )>,
        title: Signal<String>,
        stats: StatsModel,
        session_vm: WritingSessionViewModel,
        has_work: Signal<bool>,
        show_characters: Signal<bool>,
        goal_unit: Signal<frontend::common::entities::GoalUnit>,
        app_ctx: std::rc::Rc<frontend::AppContext>,
        chrome: FocusStripChrome,
        synopsis_visible: Signal<bool>,
        synopsis_capable: Signal<bool>,
    ) -> Self {
        Self {
            go_to_vm,
            quick,
            title,
            stats,
            session_vm,
            has_work,
            show_characters,
            goal_unit,
            app_ctx,
            chrome,
            synopsis_visible,
            synopsis_capable,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for FocusStrip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocusStrip").finish()
    }
}

impl Widget for FocusStrip {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let self_id = ctx.self_id();
        // The four optional items are settings, and a writer flips them while
        // sitting *in* the mode. Rebuilding on a change (rather than wrapping
        // each item in a `VisibleWhen`) is what lets each one be a real
        // `Toolbar` item with its own overflow form: a hidden-but-present item
        // would still occupy a slot in the overflow accounting, and an item
        // that has to survive being hidden cannot also declare how it collapses.
        for sig in [
            &self.chrome.title,
            &self.chrome.word_count,
            &self.chrome.session,
            &self.chrome.go,
            &self.chrome.go_to,
        ] {
            sig.bind_to(self_id, ctx.binding_registry(), BindingLevel::Rebuild);
        }

        // A `Toolbar`, not a hand-laid `HStack` + `Spacer`. It fits itself to the
        // width it is offered and re-decides every layout pass which items stay
        // inline and which collapse into a trailing chevron menu — so a narrow
        // window can no longer push controls off the end of the bar.
        //
        // Collapse order is by **priority, lowest first** (ties: last declared).
        // The readouts go first because they are glanceable extras; the
        // navigation goes last because it is what the mode leans on, the tab
        // strip being gone. Exit and the title never go at all — they are
        // *pinned* items, which the toolbar never collapses.
        let mut bar = Toolbar::new();

        if self.chrome.word_count.get() {
            let (stats, has_work, chars, unit, app_ctx) = (
                self.stats.clone(),
                self.has_work.clone(),
                self.show_characters.clone(),
                self.goal_unit.clone(),
                self.app_ctx.clone(),
            );
            bar = bar.item(
                ToolbarItem::custom(WordCountIndicator::new(
                    stats.clone(),
                    has_work.clone(),
                    chars.clone(),
                    unit.clone(),
                    app_ctx.clone(),
                ))
                // A live widget in the menu, not a one-shot row: a word count
                // that stopped counting once it collapsed would be a worse
                // answer than hiding it.
                .overflow_widget(move || {
                    Box::new(WordCountIndicator::new(
                        stats.clone(),
                        has_work.clone(),
                        chars.clone(),
                        unit.clone(),
                        app_ctx.clone(),
                    )) as Box<dyn Widget>
                }),
            );
        }
        if self.chrome.session.get() {
            let (vm, has_work) = (self.session_vm.clone(), self.has_work.clone());
            bar = bar.item(
                ToolbarItem::custom(SessionStatusItem::new(vm.clone(), has_work.clone()))
                    .overflow_widget(move || {
                        Box::new(SessionStatusItem::new(vm.clone(), has_work.clone()))
                            as Box<dyn Widget>
                    }),
            );
        }

        bar = bar.item(ToolbarItem::flexible_space());

        // The synopsis toggle, before the Go cluster. Not gated by a
        // `FocusStripChrome` field, and deliberately: the other four gadgets are
        // ambient readouts a writer may not want, while this is the only way to
        // reach the synopsis at all once the chrome is gone — the same reason Exit
        // has no setting either.
        //
        // **Pinned**, not a collapsible `.action`. A `ToolbarAction` renders its
        // overflow form as a `MenuItem`, and a menu row cannot carry an icon *and*
        // a checkmark — so a toggling action with a glyph is rejected outright by
        // the framework. Pinning it also matches what it is: like Exit, a control
        // that only exists inside this mode and must stay reachable.
        bar = bar.item(ToolbarItem::custom(
            IconButton::new(crate::icons::editor::synopsis_side())
                .size(IconButtonSize::Compact)
                .toggle(self.synopsis_visible.clone())
                .enabled(self.synopsis_capable.clone())
                .tooltip(tr!(statusbar_focus_synopsis())),
        ));

        if self.chrome.go.get() {
            // Plain actions: an icon button inline, a real menu row when
            // collapsed. Both fire the same named commands the Go menu and
            // Alt+Up/Alt+Down already drive, so there is one implementation.
            bar = bar
                .action(
                    ToolbarAction::new(tr!(statusbar_focus_go_prev()), crate::icons::go::prev_icon)
                        .priority(GO_PRIORITY)
                        .on_activate(|ctx| ctx.send_intent(Intent::new("go.prev"))),
                )
                .action(
                    ToolbarAction::new(tr!(statusbar_focus_go_next()), crate::icons::go::next_icon)
                        .priority(GO_PRIORITY)
                        .on_activate(|ctx| ctx.send_intent(Intent::new("go.next"))),
                );
        }
        if self.chrome.go_to.get() {
            // "Go to…" — the jump the arrows cannot do. After them because it is
            // the less-used of the two and the eye reads the pair first.
            //
            // Its inline form is the popover trigger; its overflow form is a
            // plain row firing `go.to`, which opens the same popup Ctrl+G does.
            // A popover trigger buried inside another popover would be a menu
            // that opens a menu.
            bar = bar.item(
                ToolbarItem::custom(crate::statusbar::go_to_button::GoToButton::new(
                    self.go_to_vm.clone(),
                    crate::statusbar::go_to_button::GO_TO_FOCUS,
                ))
                .overflow_as(
                    ToolbarAction::new(tr!(statusbar_go_to()), crate::icons::go::go_to_icon)
                        .priority(GO_TO_PRIORITY)
                        .on_activate(|ctx| ctx.send_intent(Intent::new("go.to"))),
                ),
            );
        }

        if self.chrome.title.get() {
            // **Pinned and width-capped, both.** Pinned because a name that
            // disappears into a chevron menu answers nothing — it is here
            // precisely for the moment after Alt+Down, when the writer needs to
            // know where they landed. Capped because a pinned item is never
            // collapsed, only *accommodated*: an uncapped 200-character scene
            // title would squeeze every other gadget out and push Exit off the
            // bar, which the `Toolbar` cannot rescue you from.
            bar = bar.item(ToolbarItem::custom(
                MaxSize::width(TITLE_MAX_WIDTH).child(
                    TextWidget::new(lit!(String::new()))
                        .text(self.title.clone())
                        .color(TextRole::Secondary)
                        .single_line(),
                ),
            ));
        }

        // The way to the mode's own settings without leaving it — the menu bar
        // is parked dormant behind the surface, so this is the only route that
        // does not put the full Settings modal over the manuscript. Pinned for
        // the same reason Exit is: a setting you can only reach by first undoing
        // the thing you wanted to configure is not a setting.
        if let Some((settings, themes)) = self.quick.clone() {
            bar = bar.item(ToolbarItem::custom(
                crate::distraction_free::quick_settings::quick_settings_button(
                    ctx, settings, themes,
                ),
            ));
        }

        // Never gated, and **pinned** so the toolbar can never collapse it: this
        // strip's documented way out of the mode. A pinned item reduces the room
        // the collapsible ones have and is never itself put in the menu, which
        // makes "Exit always stays" a property of the layout rather than only a
        // consequence of no setting gating it.
        //
        // Fires the same named intent Shift+F11 and the View menu use — and this
        // button only ever exists while the mode is active, so the toggle is
        // always a clean exit here, never a re-entry.
        //
        // **Ghost, not Plain.** A `Plain` button's idle fill is
        // `SurfaceRole::Main`, and on this surface that role carries the theme's
        // *general background* — the margin colour — so Exit painted a slab of
        // the wrong colour onto a page-coloured strip. `Ghost` is
        // transparent-idle, so it simply sits on whatever the strip is, which is
        // what "follows the theme" means here. It still lights up on hover.
        bar = bar.item(ToolbarItem::custom(
            Button::new(tr!(statusbar_focus_exit()))
                .variant(ButtonVariant::Ghost)
                .on_activate_fn(|ctx| ctx.send_intent(Intent::new("view.focus_mode"))),
        ));

        // On the **margin**, not the page: the surface floats the manuscript as a
        // card on `SurfaceRole::Main` (see `distraction_free::surface`), and the
        // strip is chrome beside the paper rather than a footer printed on it. A
        // hairline above is all the separation it needs. Semantic roles only, so
        // light and dark both keep working — and so a theme's own colours reach
        // it through the surface's token override.
        let bar = VStack::new().spacing(0.0).child(Divider::new()).child(
            StatusBar::new()
                .background(SurfaceRole::Main)
                // `Expand::horizontal` is load-bearing, not decoration.
                // `Toolbar` fills the width it is *offered* — but offered an
                // unbounded one it falls back to its content width, which
                // leaves its `flexible_space` no slack to claim and packs
                // every gadget against the leading edge. This guarantees a
                // bounded proposal, so the spacer can push Exit and its
                // neighbours to the trailing side.
                .child(Expand::horizontal().child(bar)),
        );

        self.root_child = Some(ctx.add(bar));
        self.root_child.into_iter().collect()
    }

    /// Zero-sized when hidden (the enclosing `VisibleWhen` gate never even
    /// builds this while the mode is inactive), matching every other
    /// status-bar item's own dormancy contract.
    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{OpenDocsStore, StatsModel};
    use frontend::AppContext;
    use skribisto_model::counting::CountingMethodSetting;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicU32, Ordering};
    use teksilo::core::accesskit::Role;
    use teksilo::core::widget_tree::WidgetTree;

    fn temp_store() -> teksilo::settings::SettingsStore {
        static N: AtomicU32 = AtomicU32::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "skribisto_focus_strip_test_{}_{n}.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        teksilo::settings::SettingsStore::open(path).expect("open temp settings store")
    }

    /// Mount a strip with `chrome` and return its live tree plus the accessible
    /// names the accessibility layer exposes. `tree_with_events` (not a bare
    /// `WidgetTree`): `SessionStatusItem` subscribes to backend events, which
    /// panics with no event source registered.
    fn mount_at(
        chrome: FocusStripChrome,
        title: &str,
        width: f32,
    ) -> (WidgetTree, FocusStripChrome) {
        let ctx = Rc::new(AppContext::new());
        let store = temp_store();
        let stats = StatsModel::new(
            OpenDocsStore::new(ctx.clone()),
            Signal::new(None),
            Signal::new(CountingMethodSetting::default()),
        );
        let session_vm = WritingSessionViewModel::new(stats.clone(), &store);
        let mut tree = crate::test_support::tree_with_events(&ctx);
        tree.add(FocusStrip::new(
            crate::go::GoToViewModel::new(ctx.clone(), crate::app_ids::AppIds::new()),
            // No quick-settings popover here: it needs a `SettingsViewModel` and
            // the theme library, and every assertion in this module is about the
            // strip's own layout and gating.
            Option::None,
            Signal::new(title.to_string()),
            stats,
            session_vm,
            Signal::new(true),
            Signal::new(false),
            Signal::new(frontend::common::entities::GoalUnit::default()),
            ctx.clone(),
            chrome.clone(),
            Signal::new(false),
            Signal::new(true),
        ));
        tree.layout(SizeProposal::exact(width, 40.0));
        (tree, chrome)
    }

    fn mount(chrome: FocusStripChrome) -> (WidgetTree, FocusStripChrome) {
        mount_at(chrome, "Prologue", 900.0)
    }

    fn button_names(tree: &mut WidgetTree) -> Vec<String> {
        tree.sync_accessibility()
            .nodes
            .iter()
            .filter(|(_, n)| n.role() == Role::Button)
            .filter_map(|(_, n)| n.label().map(|s| s.to_string()))
            .collect()
    }

    /// The invariant the whole strip rests on: **Exit survives every
    /// combination of the settings**. Distraction-free mode's other two exits
    /// are a keystroke the focused editor may legitimately swallow (Escape)
    /// and one the writer has to remember (Shift+F11), so a settings choice
    /// that could take this button away would be a way to get stranded in
    /// fullscreen with no visible way out — the exact failure other writing
    /// tools have filed bugs for.
    #[test]
    fn exit_survives_every_combination_of_the_chrome_settings() {
        let exit = tr!(statusbar_focus_exit()).resolve_now();
        for title in [false, true] {
            for wc in [false, true] {
                for session in [false, true] {
                    for go in [false, true] {
                        let (mut tree, _) = mount(FocusStripChrome {
                            title: Signal::new(title),
                            word_count: Signal::new(wc),
                            session: Signal::new(session),
                            go: Signal::new(go),
                            go_to: Signal::new(go),
                        });
                        assert!(
                            button_names(&mut tree).contains(&exit),
                            "Exit must be present with title={title}, word_count={wc}, \
                             session={session}, go={go}"
                        );
                    }
                }
            }
        }
    }

    /// **A narrow window must not do what a settings choice may not.** The
    /// toolbar collapses its gadgets into the chevron menu as room runs out;
    /// Exit is a *pinned* item, so it is never a candidate — which is what makes
    /// "there is always a way out" a property of the layout rather than a
    /// promise the settings happen to keep.
    #[test]
    fn the_strip_collapses_its_gadgets_but_never_exit() {
        let exit = tr!(statusbar_focus_exit()).resolve_now();
        let prev = tr!(statusbar_focus_go_prev()).resolve_now();

        let (mut wide, _) = mount_at(FocusStripChrome::all_shown(), "Prologue", 1400.0);
        let wide_names = button_names(&mut wide);
        assert!(wide_names.contains(&prev), "the arrows fit at 1400px");
        assert!(wide_names.contains(&exit));

        // Punishingly narrow: the collapsible gadgets have to give way.
        let (mut narrow, _) = mount_at(FocusStripChrome::all_shown(), "Prologue", 220.0);
        let narrow_names = button_names(&mut narrow);
        assert!(
            narrow_names.contains(&exit),
            "Exit was collapsed out of a narrow strip — it must be pinned. Buttons: {narrow_names:?}"
        );
    }

    /// A long scene title elides rather than shoving Exit off the bar. The
    /// title is *pinned*, so the toolbar will never collapse it to make room —
    /// it can only accommodate it, which is exactly why it needs its own cap.
    #[test]
    fn a_long_title_does_not_displace_the_exit_button() {
        let exit = tr!(statusbar_focus_exit()).resolve_now();
        let long = "The Chapter In Which A Great Many Things Happen At Once And Nobody \
                    Is Entirely Sure Why, Least Of All The Narrator Himself";
        let (mut tree, _) = mount_at(FocusStripChrome::all_shown(), long, 900.0);
        assert!(
            button_names(&mut tree).contains(&exit),
            "a long item name pushed Exit off the strip"
        );
    }

    /// The Go pair is one affordance: both arrows go, or neither does. A strip
    /// offering only Previous would be a worse answer than offering neither.
    #[test]
    fn the_go_pair_appears_and_disappears_together() {
        let prev = tr!(statusbar_focus_go_prev()).resolve_now();
        let next = tr!(statusbar_focus_go_next()).resolve_now();

        let (mut on, _) = mount(FocusStripChrome::all_shown());
        let names = button_names(&mut on);
        assert!(names.contains(&prev) && names.contains(&next), "both shown");

        let (mut off, _) = mount(FocusStripChrome {
            go: Signal::new(false),
            ..FocusStripChrome::all_shown()
        });
        let names = button_names(&mut off);
        assert!(
            !names.contains(&prev) && !names.contains(&next),
            "neither shown"
        );
    }

    /// Flipping a setting on an ALREADY-MOUNTED strip re-gates it. The
    /// per-combination tests above each mount a fresh strip, so they would pass
    /// even if the gate were read once at build time — but the writer's path is
    /// to be *in* the mode, open settings, untick a box and expect the strip to
    /// change under them.
    #[test]
    fn flipping_a_setting_re_gates_a_mounted_strip() {
        let prev = tr!(statusbar_focus_go_prev()).resolve_now();
        let (mut tree, chrome) = mount(FocusStripChrome::all_shown());
        assert!(button_names(&mut tree).contains(&prev), "starts shown");

        chrome.go.set(false);
        tree.layout(SizeProposal::exact(900.0, 40.0));
        assert!(
            !button_names(&mut tree).contains(&prev),
            "unticking Go must hide the arrows without re-entering the mode"
        );

        chrome.go.set(true);
        tree.layout(SizeProposal::exact(900.0, 40.0));
        assert!(
            button_names(&mut tree).contains(&prev),
            "and ticking it must bring them back"
        );
    }
}
