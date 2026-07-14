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

use std::cell::Cell;
use std::rc::Rc;

use bastyde::core::styles::{PanelVariant, SharedStandardItemStyle, StandardItemStyleConfig};
use bastyde::core::widget_builder::HandlerSet;
use bastyde::data::{ListModel, SelectionMode, SelectionModel};
use bastyde::prelude::*;
use bastyde::res;
use bastyde::widgets::GroupHeader;
use bastyde::widgets::primitives::icon_widget::IconMode;
// `InteractionState` (the hover/press state the standard-item chrome recipe
// consumes) is not re-exported at the widgets root — reach it by module path.
use bastyde::widgets::button::InteractionState;
use bastyde::widgets::styles::RecipeStandardItemStyle;
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
    /// Keyboard/pointer cursor for each list. **Required for any row highlight
    /// to exist at all:** `ListView` hands its row delegate
    /// `selection.map(|s| s.is_selected(i)).unwrap_or(false)` — with no
    /// selection model attached, every row is told it is unselected forever, so
    /// arrow keys move the view's internal `focused_index` (Enter still opens
    /// the right row) while nothing on screen — and nothing in the AccessKit
    /// tree — says where the cursor is. Held on the panel, not built inside
    /// `build`, so the highlight survives a rebuild.
    recents_selection: SelectionModel,
    examples_selection: SelectionModel,
    /// The recents `ListView`'s id, republished on each build as this panel's
    /// [`Widget::initial_focus_hint`] so the Launcher window opens with the list
    /// focused (Enter then opens the highlighted project straight away).
    /// `None` when there are no recents — see [`Self::recents_list`].
    recents_list_id: Cell<Option<WidgetId>>,
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
            // Single: these lists are launch targets — you open one project, so
            // a multi-select cursor would be meaningless.
            recents_selection: SelectionModel::new(SelectionMode::Single),
            examples_selection: SelectionModel::new(SelectionMode::Single),
            recents_list_id: Cell::new(None),
            root_child: None,
        }
    }

    /// Works pane: search + Open/New-Work actions, then the recent-works list.
    fn works_pane(&self, vm: &WelcomeViewModel, ctx: &mut BuildContext) -> impl Widget + 'static {
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
                    child: self.recents_list(vm, ctx)
                }
            }
        )
    }

    /// The recent-works region: a virtualized [`ListView`] of hand-rolled rows
    /// (see [`RecentRow`]) — or a muted placeholder when there are none. Both
    /// arms sit under a constant-index [`Switcher`] so the region resolves to
    /// one widget type (only the active page is mounted).
    fn recents_list(&self, vm: &WelcomeViewModel, ctx: &mut BuildContext) -> impl Widget + 'static {
        let model = self.recents.list_model();

        // Open with the most recent work under the cursor — the launcher
        // convention: Enter resumes your last project without aiming first, and
        // the arrow keys step from a row you can actually see (`ListView` adopts
        // a preset selection as its keyboard cursor). Guarded on "nothing
        // selected yet" so a rebuild never yanks the highlight back to the top
        // after the user has moved it.
        if !model.is_empty() && self.recents_selection.selected_indices().is_empty() {
            self.recents_selection.select(0);
        }

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
            Box::new(RecentRow::new(
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
        .selection(self.recents_selection.clone())
        .auto_item_height(52.0)
        .on_activate(move |i, ctx| {
            if let Some(path) = open_model.with_item(i, |d| d.absolute_path.clone()) {
                row_vm.open_work(path, ctx);
            }
        });

        // Mount the list by id so the panel can hand it back as its
        // `initial_focus_hint` — the window opens with the list already focused,
        // so Enter resumes the highlighted project with no Tab first. Published
        // only when there ARE recents: with none, the Switcher shows the empty
        // note instead and focusing an unmounted list would be a dead focus.
        let list_id = ctx.add(list);
        self.recents_list_id
            .set((!self.recents.list_model().is_empty()).then_some(list_id));

        Switcher::new(switch_index)
            .child(empty_note(tr!(welcome_empty_recents())))
            .child_id(list_id)
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
        .selection(self.examples_selection.clone())
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
/// **The layout is hand-rolled; the chrome is not.** The selection background,
/// corner radius, hover wash, WCAG selection edge and `:focus-visible` keyboard
/// ring all come from the *same* [`StandardItemStyle`] recipe
/// [`StandardListItem`] uses (`theme.style_slots.standard_item`), so a
/// highlighted recents row is indistinguishable from a highlighted Examples row
/// — in both themes, and including the muted `SelectedInactive` wash when focus
/// sits elsewhere. That seam is exactly what `StandardItemStyleConfig` is for:
/// its own docs say the style owns "the chrome (selection background, corner
/// radius, padding) but **not** row-internal layout". The recipe already pads
/// and insets its content, so the row hands it the bare `HStack` (as
/// `StandardListItem` does) and adds no `Padding` of its own.
///
/// Why not simply *be* a [`StandardListItem`], then? Its `label_column` is a
/// plain (non-flex) child of the row's outer `HStack`, measured at its own
/// intrinsic/natural width before that HStack's trailing `Spacer` claims the
/// rest. `Expand`'s default flex basis is **zero** (it contributes nothing to
/// an intrinsic-size query), so nesting an `Expand` inside
/// `subtitle_leading_slot` never widens `label_column` itself — it can only
/// ever fill whatever (already-narrow, title-width-sized) box
/// `StandardListItem` handed it. Owning the row lets the path's `Expand`
/// compete for the *row's* remaining width directly, so a long path (e.g.
/// `/home/cyril/Nextcloud/…/Faux-semblants.skrib`) stays one line, on one row
/// height, with the filename still readable — the "wrap in `Expand` (fill mode)
/// to make it fill a box" layout gotcha, applied at the right level. Borrowing
/// the chrome recipe keeps that layout freedom *and* the stock look.
struct RecentRow {
    /// `Option` only so `build` can move it into the tree (built once).
    icon: Option<IconWidget>,
    title: String,
    path: String,
    date: String,
    selected: Signal<bool>,
    /// Hover/press state feeding the recipe's non-selected chrome. Rows are
    /// rebuilt on selection change, so a per-build `Signal` is enough.
    interaction: Signal<InteractionState>,
    root_child: Option<WidgetId>,
}

impl RecentRow {
    fn new(icon: IconWidget, title: String, path: String, date: String, selected: bool) -> Self {
        Self {
            icon: Some(icon),
            title,
            path,
            date,
            selected: Signal::new(selected),
            interaction: Signal::new(InteractionState::Idle),
            root_child: None,
        }
    }
}

impl std::fmt::Debug for RecentRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RecentRow")
            .field("title", &self.title)
            .finish()
    }
}

impl Widget for RecentRow {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let body = VStack::new()
            .spacing(2.0)
            .child(
                TextWidget::new(lit!(self.title.clone()))
                    .style(TextStyleRole::Body)
                    .color(TextRole::Primary)
                    .single_line(),
            )
            .child(
                TextWidget::new(lit!(self.path.clone()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary)
                    .overflow(TextOverflow::Ellipsis(EllipsisMode::Middle)),
            );
        let row = HStack::new()
            .spacing(10.0)
            .child(self.icon.take().expect("RecentRow builds once"))
            .child(Expand::horizontal().child(body))
            .child(
                TextWidget::new(lit!(self.date.clone()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .access_label_literal(self.title.clone())
            .access_description_literal(self.path.clone());
        let content = ctx.add(row);

        // The stock chrome, driven by this row's own state signals — the theme's
        // slot when one is installed, else the default recipe (mirrors
        // `StandardListItem::build_with_background`).
        let style: SharedStandardItemStyle = ctx
            .theme()
            .style_slots
            .standard_item
            .clone()
            .unwrap_or_else(|| Rc::new(RecipeStandardItemStyle::default()));
        let cfg = StandardItemStyleConfig {
            content,
            is_selected: self.selected.clone(),
            is_hovered: self
                .interaction
                .map(|s| matches!(s, InteractionState::Hovered)),
            is_pressed: self
                .interaction
                .map(|s| matches!(s, InteractionState::Pressed)),
            // Resolves this row's focus scope — the enclosing `ListView` — so a
            // selected row pales to `SelectedInactive` when focus leaves it.
            is_focused: ctx.view_focus_active(),
            is_focus_visible: ctx.focus_visible(),
            is_disabled: Signal::new(false),
            is_window_active: ctx.window_active_signal(),
        };
        let root = style.make_body(&cfg, ctx);

        let interaction = self.interaction.clone();
        ctx.apply_self_handlers(HandlerSet::new().on_hover(
            move |entered: bool, _ctx: &mut EventContext| {
                interaction.set(if entered {
                    InteractionState::Hovered
                } else {
                    InteractionState::Idle
                });
            },
        ));

        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
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
            .child(self.works_pane(&vm, ctx))
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

    /// Open the Launcher with keyboard focus already on the recent-works list,
    /// so **Enter opens the highlighted project immediately** — no Tab hunt
    /// first. The window machinery consults this after the root is built (it
    /// walks descendants, so the hint is found under the title bar / resize
    /// frame chrome) and focuses what we point at. Paired with the preselected
    /// top row in [`Self::recents_list`]: highlight *and* focus, or Enter would
    /// still go nowhere.
    ///
    /// `None` when there are no recents — the list isn't the mounted page then,
    /// and the window falls back to opening with nothing focused, as before.
    fn initial_focus_hint(&self) -> Option<WidgetId> {
        self.recents_list_id.get()
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
