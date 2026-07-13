//! The Welcome content — Skribisto's start screen, hosted as the Launcher
//! window's root (see [`crate::windows::launcher_window_config`]) rather than
//! a modal — the launcher-window model. Two panes: a left **sidebar** (brand
//! block + a bottom-pinned vertical nav) and a right **content** area that
//! switches with the nav selection.
//!
//! The nav is the framework's standalone vertical [`TabBar`] — it gives the 2 dp
//! leading accent indicator, accent-on-selected label, keyboard nav, and
//! `Role::TabList`/`Tab` accessibility for free (tablist/tab is the correct AT
//! semantics for *in-place* pane switching; see the a11y note in the plan). The
//! right pane is a sibling [`Switcher`] keyed off the bar's `selected_tab`
//! signal — we compose `[branding, Spacer, TabBar]` ourselves so the nav pins
//! to the **bottom** (a vertical `TabWidget` keeps its tabs top-aligned under
//! the leading slot; only owning the layout lets the `Spacer` claim the
//! slack). No inline "show at startup" control here — that setting lives in
//! Settings ▸ Appearance & Behaviour (a launcher-local copy would be a
//! footgun: it would hide the very screen you're looking at).
//!
//! All business logic lives on [`WelcomeViewModel`]; this view is thin. The
//! layout is built with the `bati!` DSL; only the nav [`TabBar`] and the content
//! [`Switcher`] stay as plain builders — they're generic over closures, which the
//! DSL can't express (same rationale as `app.rs`).

use std::rc::Rc;

use bastyde::core::styles::PanelVariant;
use bastyde::data::ListModel;
use bastyde::prelude::*;
use bastyde::res;
use bastyde::widgets::GroupHeader;
use bastyde::widgets::primitives::icon_widget::IconMode;
use bastyde::widgets::{
    ActivateOn, Button, ButtonVariant, Center, Divider, Expand, FixedSize, HStack, IconLocation,
    IconWidget, ListView, Padding, Panel, SearchField, Spacer, StandardListItem, Switcher, TabBar,
    TabDelegate, TabId, TabIndicatorPosition, TextWidget, VStack,
};

use frontend::AppContext;

use crate::models::{ExamplesListModel, RecentWorkListModel};
use crate::view_models::WelcomeViewModel;

/// The four left-rail sections, in order.
#[derive(Clone, Copy)]
enum NavItem {
    Works,
    Examples,
    Learn,
    About,
}

impl NavItem {
    fn icon(self) -> IconWidget {
        let svg = match self {
            NavItem::Works => res!("assets/icons/binder/book.svg"),
            NavItem::Examples => res!("assets/icons/binder/binder.svg"),
            NavItem::Learn => res!("assets/icons/welcome/learn.svg"),
            NavItem::About => res!("assets/icons/welcome/info.svg"),
        };
        IconWidget::from_svg_icon(svg).icon_size(16.0)
    }
}

pub struct WelcomePanel {
    app_ctx: Rc<AppContext>,
    recents: RecentWorkListModel,
    examples: ExamplesListModel,
    /// Bar selection (source of truth); seeded to the first tab so Works shows.
    selected_tab: Signal<Option<TabId>>,
    /// Stable per-tab ids (index ↔ id), shared by the bar's `id_of` and the
    /// content `Switcher`'s derived index.
    tab_ids: Vec<TabId>,
    search: Signal<String>,
    root_child: Option<WidgetId>,
}

impl WelcomePanel {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        let tab_ids: Vec<TabId> = (0..4).map(|_| TabId::fresh()).collect();
        let selected_tab = Signal::new(Some(tab_ids[0]));
        Self {
            recents: RecentWorkListModel::new(app_ctx.clone()),
            examples: ExamplesListModel::new(app_ctx.clone()),
            app_ctx,
            selected_tab,
            tab_ids,
            search: Signal::new(String::new()),
            root_child: None,
        }
    }

    /// Works pane: search + Open/New-Work actions, then the recent-works list.
    fn works_pane(&self, vm: &WelcomeViewModel) -> impl Widget + 'static {
        let open_vm = vm.clone();
        let new_vm = vm.clone();
        let open_icon =
            IconWidget::from_svg_icon(res!("assets/icons/binder/folder.svg")).icon_size(16.0);
        let plus_icon =
            IconWidget::from_svg_icon(res!("assets/icons/welcome/plus.svg")).icon_size(16.0);

        bati!(
            VStack {
                spacing: 0.0
                Padding::symmetric(12.0, 16.0) {
                    HStack {
                        spacing: 10.0
                        Expand::horizontal {
                            SearchField::new(self.search.clone()) {
                                placeholder: tr!(welcome_search())
                            }
                        }
                        Button::new(tr!(welcome_open())) {
                            variant: ButtonVariant::Plain
                            icon: open_icon, IconLocation::Leading
                            on_activate_fn: move |ctx| open_vm.pick_open(ctx)
                        }
                        Button::new(tr!(welcome_new_work())) {
                            variant: ButtonVariant::Filled
                            icon: plus_icon, IconLocation::Leading
                            on_activate_fn: move |ctx| new_vm.new_work(ctx)
                        }
                    }
                }
                Padding::symmetric(6.0, 16.0) {
                    GroupHeader::new(tr!(welcome_recent_works())) {
                        style: TextStyleRole::SmallBold
                        color: TextRole::Secondary
                    }
                }
                Expand::vertical {
                    child: self.recents_list(vm)
                }
            }
        )
    }

    /// The recent-works region: a virtualized [`ListView`] of hand-rolled rows
    /// (see [`recent_row`]) — or a muted placeholder when there are none. Both
    /// arms sit under a constant-index [`Switcher`] so the region resolves to
    /// one widget type (only the active page is mounted).
    fn recents_list(&self, vm: &WelcomeViewModel) -> impl Widget + 'static {
        let model = self.recents.list_model();

        // Reactive page index: re-derived on every `refresh()` (which bumps
        // `version` *after* `reconcile_by_key`), so the list replaces the empty-note
        // placeholder as soon as the first recent work arrives. A plain
        // build-time `model.is_empty()` snapshot fed to `Signal::new(..)` would
        // leave the Switcher stuck on the empty page for this panel instance's
        // whole lifetime (mirrors how the nav Switcher below derives its index).
        let idx_model = model.clone();
        let switch_index = self
            .recents
            .version_signal()
            .map(move |_: &u64| if idx_model.is_empty() { 0usize } else { 1usize });

        // `on_activate` hands back only the row index, so the open path reads the
        // file path back out of a second cheap-clone handle on the same model.
        let open_model = model.clone();
        let row_vm = vm.clone();
        let list = ListView::new(model, |_i, dto, selected| {
            let icon =
                IconWidget::from_svg_icon(res!("assets/icons/binder/book.svg")).icon_size(20.0);
            let date = dto.last_opened_at.format("%Y-%m-%d").to_string();
            Box::new(recent_row(
                icon,
                dto.title.clone(),
                dto.absolute_path.clone(),
                date,
                selected,
            ))
        })
        // Single-click to open (these rows are launch targets, not multi-select
        // list items); arrow keys still move the highlight without opening.
        .activate_on(ActivateOn::SingleClick)
        .auto_item_height(52.0)
        .on_activate(move |i, ctx| {
            if let Some(path) = open_model.with_item(i, |d| d.absolute_path.clone()) {
                row_vm.open_work(path, ctx);
            }
        });

        Switcher::new(switch_index)
            .child(empty_note(tr!(welcome_empty_recents())))
            .child(list)
    }

    /// Examples pane: the bundled example works (one today — Starforgers).
    fn examples_pane(&self, vm: &WelcomeViewModel) -> impl Widget + 'static {
        bati!(
            VStack {
                spacing: 0.0
                Padding::symmetric(12.0, 16.0) {
                    GroupHeader::new(tr!(nav_examples())) {
                        style: TextStyleRole::SmallBold
                        color: TextRole::Secondary
                    }
                }
                Expand::vertical {
                    child: self.examples_list(vm)
                }
            }
        )
    }

    /// The examples region — same [`ListView`] + [`StandardListItem`] shape as
    /// [`recents_list`](Self::recents_list). Always non-empty (the bundled set),
    /// so no placeholder branch.
    fn examples_list(&self, vm: &WelcomeViewModel) -> impl Widget + 'static {
        let model = self.examples.list_model();
        let open_model = model.clone();
        let ex_vm = vm.clone();
        ListView::new(model, |_i, ex, selected| {
            let icon =
                IconWidget::from_svg_icon(res!("assets/icons/binder/book.svg")).icon_size(20.0);
            Box::new(
                StandardListItem::new(lit!(ex.title))
                    .subtitle(lit!(ex.blurb))
                    .leading_slot(icon)
                    .selected(selected),
            )
        })
        .activate_on(ActivateOn::SingleClick)
        .auto_item_height(52.0)
        .on_activate(move |i, ctx| {
            if let Some((file_name, bytes)) = open_model.with_item(i, |e| (e.file_name, e.bytes)) {
                ex_vm.open_example(file_name, bytes, ctx);
            }
        })
    }
}

/// A recent-work row: icon, title + middle-ellipsized path, trailing date.
///
/// Hand-rolled rather than [`StandardListItem`] — its subtitle line has no
/// overflow override (always wraps) *and*, more fundamentally, its
/// `label_column` is a plain (non-flex) child of the row's outer `HStack`,
/// measured at its own intrinsic/natural width before that HStack's trailing
/// `Spacer` claims the rest. `Expand`'s default flex basis is **zero** (it
/// contributes nothing to an intrinsic-size query), so nesting an `Expand`
/// inside `subtitle_leading_slot` never widens `label_column` itself — it can
/// only ever fill whatever (already-narrow, title-width-sized) box
/// `StandardListItem` handed it. Owning the whole row lets the path's
/// `Expand` compete for the *row's* remaining width directly, so a long path
/// (e.g. `/home/cyril/Nextcloud/…/Faux-semblants.skrib`) stays one line, on
/// one row height, with the filename still readable — the "wrap in `Expand`
/// (fill mode) to make it fill a box" layout gotcha, applied at the right
/// level this time.
///
/// No selection background (StandardListItem's rounded-rect chrome isn't
/// reproduced here — these rows are launch targets that navigate away
/// immediately on click, not a persistent multi-select list); the title
/// accents on selection instead, mirroring `project_switcher_button.rs`'s
/// own hand-rolled row.
fn recent_row(
    icon: IconWidget,
    title: String,
    path: String,
    date: String,
    selected: bool,
) -> impl Widget + 'static {
    let title_role = if selected {
        TextRole::Accent
    } else {
        TextRole::Primary
    };
    let body = VStack::new()
        .spacing(2.0)
        .child(
            TextWidget::new(lit!(title.clone()))
                .style(TextStyleRole::Body)
                .color(title_role)
                .single_line(),
        )
        .child(
            TextWidget::new(lit!(path.clone()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary)
                .overflow(TextOverflow::Ellipsis(EllipsisMode::Middle)),
        );
    Padding::symmetric(6.0, 10.0).child(
        HStack::new()
            .spacing(10.0)
            .child(icon)
            .child(Expand::horizontal().child(body))
            .child(
                TextWidget::new(lit!(date))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .access_label_literal(title)
            .access_description_literal(path),
    )
}

/// Muted top-aligned note shown in place of a list when it has no rows.
fn empty_note(text: impl Into<LocalizedString>) -> impl Widget + 'static {
    bati!(
        Padding::symmetric(8.0, 10.0) {
            TextWidget::new(text) {
                style: TextStyleRole::Body
                color: TextRole::Secondary
            }
        }
    )
}

/// Centered muted placeholder for the not-yet-designed Learn/About panes.
fn placeholder(text: impl Into<LocalizedString>) -> impl Widget + 'static {
    bati!(
        Center {
            TextWidget::new(text) {
                style: TextStyleRole::Body
                color: TextRole::Secondary
            }
        }
    )
}

impl std::fmt::Debug for WelcomePanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WelcomePanel").finish()
    }
}

impl Widget for WelcomePanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // All logic on the view-model; rebuilt here from the live store. The
        // project-window factory is process-wide `app_state` (registered once
        // in `main`), shared by clone rather than reconstructed.
        let factory = ctx
            .app_state::<crate::windows::ProjectWindowFactory>()
            .cloned()
            .expect("ProjectWindowFactory registered in main");
        let vm = WelcomeViewModel::new(ctx.settings(), self.app_ctx.clone(), factory);

        // Ctrl+Q / File ▸ Quit on the bare Launcher (no project open, hence no
        // `App`, no unsaved state, no `on_close_requested` guard — this window's
        // own doc comment already states that closing it while it's the only
        // open window quits the process, by design). A plain guarded
        // `close_window()` is correct here: there is nothing to veto. This is
        // deliberately simpler than the project window's `app.quit` (which runs
        // through `guard_unsaved_exit` — see `app.rs`) — registering *that*
        // version here would have zero effect anyway: each `WidgetTree` (one per
        // OS window) has its own independent `global_actions`/`shortcut_registry`,
        // so `App::build`'s own `app.quit` registration is unreachable from a
        // window that never builds an `App`. Must be registered inside a widget
        // that is actually built into the Launcher's own window tree — this one.
        ctx.register_shortcut_global(
            Shortcut::new("app.quit")
                .name("Quit")
                .primary(KeyStroke::ctrl(Key::Q))
                .build(),
        );
        ctx.register_action_global(Action::new("app.quit").on_invoke(|_i, c| c.close_window()));

        // The lists are reactive `ListView`s bound to Layer-A `ListModel`s, so
        // they refresh themselves on `LoadWork` — no widget rebuild needed here.
        // `wire` just subscribes each model to the backend (examples: a no-op).
        self.recents.wire(ctx);
        self.examples.wire(ctx);

        // ── Brand block (top of the sidebar) ───────────────────────────────
        let mut title_style = ctx.theme().typography.body_bold.clone();
        title_style.size = 22.0;
        let logo = IconWidget::from_raster(res!("../../resources/icons/skribisto.png"), 60.0)
            .mode(IconMode::FullColor);
        let branding = bati!(
            Padding::symmetric(8.0, 4.0) {
                VStack {
                    spacing: 6.0
                    child: logo
                    TextWidget::new(lit!("Skribisto")) {
                        style: title_style
                        color: TextRole::Primary
                    }
                    TextWidget::new(lit!("Version 1.9.43 · Bastyde")) {
                        style: TextStyleRole::Small
                        color: TextRole::Secondary
                    }
                    TextWidget::new(tr!(welcome_tagline())) {
                        style: TextStyleRole::Small
                        color: TextRole::Secondary
                    }
                }
            }
        );

        // ── Vertical nav TabBar (bottom of the sidebar) ─────────────────────
        let nav_model = ListModel::from_vec(vec![
            NavItem::Works,
            NavItem::Examples,
            NavItem::Learn,
            NavItem::About,
        ]);
        let delegate = TabDelegate::new(|_, item: &NavItem| match item {
            NavItem::Works => tr!(nav_works()),
            NavItem::Examples => tr!(nav_examples()),
            NavItem::Learn => tr!(nav_learn()),
            NavItem::About => tr!(nav_about()),
        })
        .icon(|_, item: &NavItem| Some(item.icon()));

        let ids_for_idof = self.tab_ids.clone();
        let bar = TabBar::vertical(
            nav_model,
            delegate,
            self.selected_tab.clone(),
            move |i, _item| ids_for_idof[i],
        )
        .selected_text_role(TextRole::Accent)
        .idle_text_role(TextRole::Secondary)
        .selected_tab_background(SurfaceRole::AccentSubtle)
        .active_indicator(TabIndicatorPosition::OuterEdge)
        .tab_bar_height(34.0) // compact pills (~design's 30 dp), not the 50 dp default
        .show_scroll_arrows(false)
        .show_overflow_dropdown(false)
        .access_label_literal("Welcome sections");

        // ── Right pane: a Switcher keyed off the bar's selection ────────────
        // `TabBar` and `Switcher` are generic over closures, so they stay plain
        // builders and join the bati! tree below via `child:`.
        let ids_for_idx = self.tab_ids.clone();
        let switch_index = self.selected_tab.map(move |opt: &Option<TabId>| {
            (*opt)
                .and_then(|tid| ids_for_idx.iter().position(|t| *t == tid))
                .unwrap_or(0)
        });
        let content = Switcher::new(switch_index)
            .child(self.works_pane(&vm))
            .child(self.examples_pane(&vm))
            .child(placeholder(tr!(welcome_learn_soon())))
            .child(placeholder(tr!(welcome_about_blurb())));

        // Deterministic heights: `HStack` defaults to `VAlignment::Center` (no
        // stretch), so a content-sized sidebar would float in the middle. Both
        // columns are pinned to the body height (548 − 44 header − 1 divider =
        // 503) so they fill it exactly and the sidebar's `Spacer` can push the
        // nav to the bottom.
        const BODY_H: f32 = 503.0;

        // Hard-bound the whole card to the modal size so the greedy inner
        // `Expand`s fill exactly 780×548 (otherwise they stretch to the window
        // height and the sidebar overflows below the card).
        let root = bati!(ctx => FixedSize {
                width: 780.0
                height: 548.0
                // The Raised card is the modal's lighter surface.
                Panel {
                    variant: PanelVariant::Raised
                    corner_radius: 10.0
                    padding: 0.0
                    VStack {
                        spacing: 0.0
                        // ── Header strip: title (full width, fixed 44 dp). No
                        // close button here any more — the Launcher is a real
                        // window now, and its own `TitleBar` (see
                        // `windows.rs::launcher_window_config`) already has the
                        // window controls; a second, inner ✕ would be redundant
                        // and ambiguous about what it even closes.
                        // `Expand::horizontal` claims the VStack's full width; the
                        // height-only `FixedSize` alone would leave the strip at its
                        // natural (collapsed) width and squash the title.
                        Expand::horizontal {
                            FixedSize {
                                height: 44.0
                                Padding::symmetric(8.0, 14.0) {
                                    TextWidget::new(tr!(welcome_title())) {
                                        style: TextStyleRole::Small
                                        color: TextRole::Secondary
                                    }
                                }
                            }
                        }
                        // Full-width rule between the title strip and the body.
                        Expand::horizontal {
                            Divider
                        }
                        // ── Body: sidebar · vertical rule · content pane.
                        HStack {
                            spacing: 0.0
                            // Sidebar: brand block, a Spacer, the bottom-pinned
                            // nav. No "show at startup" control here — that
                            // setting lives in Settings ▸ Appearance & Behaviour
                            // only (a launcher-local copy would hide the very
                            // screen you're looking at, with no way back).
                            FixedSize {
                                width: 264.0
                                height: BODY_H
                                // Left margin so the brand/nav don't hug the modal edge.
                                Padding::new(0.0, 0.0, 0.0, 16.0) {
                                    VStack {
                                        spacing: 0.0
                                        child: branding
                                        Spacer
                                        // A vertical TabBar's scroll area is greedily
                                        // `Expand::vertical`, so next to a flexible
                                        // `Spacer` it collapses to height 0 and its
                                        // pills overflow. Pin it to its intrinsic
                                        // extent (4 tabs × 34 dp) so the `Spacer` above
                                        // can push the whole nav to the sidebar bottom.
                                        FixedSize {
                                            height: 136.0
                                            child: bar
                                        }
                                    }
                                }
                            }
                            // Vertical rule between the sidebar and the content pane.
                            FixedSize {
                                height: BODY_H
                                Divider::vertical()
                            }
                            // Two-tone: the content pane sits on a darker (Sunken)
                            // base. A `Panel` stretches its child (unlike `ZStack`,
                            // which centres and collapses the greedy content).
                            Expand::horizontal {
                                FixedSize {
                                    height: BODY_H
                                    Panel {
                                        variant: PanelVariant::Sunken
                                        corner_radius: 0.0
                                        padding: 0.0
                                        child: content
                                    }
                                }
                            }
                        }
                    }
                }
            }
        );
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Delegate to the fixed-size (non-greedy) root: it reports a bounded
        // 780×548 card and bounds the inner greedy `Expand`s. Delegating to the
        // inner `Panel` instead would fill the window (the modal-centering trap
        // noted in `SettingsPanel`); the `FixedSize` avoids it.
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}
