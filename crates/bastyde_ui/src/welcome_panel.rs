//! The Welcome modal — Skribisto's start screen.
//!
//! Presented as an in-tree modal (see the `welcome.show` action in `app.rs`).
//! Two panes: a left **sidebar** (brand block + a bottom-pinned vertical nav) and
//! a right **content** area that switches with the nav selection.
//!
//! The nav is the framework's standalone vertical [`TabBar`] — it gives the 2 dp
//! leading accent indicator, accent-on-selected label, keyboard nav, and
//! `Role::TabList`/`Tab` accessibility for free (tablist/tab is the correct AT
//! semantics for *in-place* pane switching; see the a11y note in the plan). The
//! right pane is a sibling [`Switcher`] keyed off the bar's `selected_tab`
//! signal — we compose `[branding, Spacer, TabBar, checkbox]` ourselves so the
//! nav pins to the **bottom** (a vertical `TabWidget` keeps its tabs top-aligned
//! under the leading slot; only owning the layout lets the `Spacer` claim the
//! slack).
//!
//! All business logic lives on [`WelcomeViewModel`]; this view is thin. The
//! layout is built with the `bati!` DSL; only the nav [`TabBar`] and the content
//! [`Switcher`] stay as plain builders — they're generic over closures, which the
//! DSL can't express (same rationale as `app.rs`).

use std::rc::Rc;

use bastyde::core::BindingLevel;
use bastyde::core::styles::PanelVariant;
use bastyde::data::ListModel;
use bastyde::prelude::*;
use bastyde::res;
use bastyde::widgets::GroupHeader;
use bastyde::widgets::primitives::icon_widget::IconMode;
use bastyde::widgets::{
    Button, ButtonVariant, Center, Checkbox, Divider, Expand, FixedSize, HStack, IconButton,
    IconLocation, IconWidget, Padding, Panel, ScrollArea, SearchField, Spacer, Switcher, TabBar,
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

        // The recent-works list is data-driven (a loop over the live model
        // building `work_row` helpers); kept as a plain builder and embedded
        // into the bati! tree below via `child:`.
        let mut list = VStack::new().spacing(2.0);
        let recents = self.recents.items();
        if recents.is_empty() {
            list = list.child(
                Padding::symmetric(8.0, 10.0).child(
                    TextWidget::new(lit!("No recent works yet."))
                        .style(TextStyleRole::Body)
                        .color(TextRole::Secondary),
                ),
            );
        } else {
            for dto in recents {
                let path = dto.absolute_path.clone();
                let date = dto.last_opened_at.format("%Y-%m-%d").to_string();
                let row_vm = vm.clone();
                let icon =
                    IconWidget::from_svg_icon(res!("assets/icons/binder/book.svg")).icon_size(20.0);
                list = list.child(work_row(
                    icon,
                    dto.title.clone(),
                    path.clone(),
                    Some(date),
                    move |ctx| row_vm.open_work(path.clone(), ctx),
                ));
            }
        }

        bati!(
            VStack {
                spacing: 0.0
                Padding::symmetric(12.0, 16.0) {
                    HStack {
                        spacing: 10.0
                        Expand::horizontal {
                            SearchField::new(self.search.clone()) {
                                placeholder: lit!("Search works")
                            }
                        }
                        Button::new(lit!("Open")) {
                            variant: ButtonVariant::Plain
                            icon: open_icon, IconLocation::Leading
                            on_activate_fn: move |ctx| open_vm.pick_open(ctx)
                        }
                        Button::new(lit!("New Work")) {
                            variant: ButtonVariant::Filled
                            icon: plus_icon, IconLocation::Leading
                            on_activate_fn: move |ctx| new_vm.new_work(ctx)
                        }
                    }
                }
                Padding::symmetric(6.0, 16.0) {
                    GroupHeader::new(lit!("Recent Works")) {
                        style: TextStyleRole::SmallBold
                        color: TextRole::Secondary
                    }
                }
                Expand::vertical {
                    ScrollArea {
                        Padding::symmetric(6.0, 12.0) {
                            child: list
                        }
                    }
                }
            }
        )
    }

    /// Examples pane: the bundled example works (one today — Starforgers).
    fn examples_pane(&self, vm: &WelcomeViewModel) -> impl Widget + 'static {
        // Data-driven list (loop over the bundled examples); kept as a plain
        // builder and embedded into the bati! tree below via `child:`.
        let mut list = VStack::new().spacing(2.0);
        for ex in self.examples.items() {
            let ex_vm = vm.clone();
            let file_name = ex.file_name;
            let bytes = ex.bytes;
            let icon =
                IconWidget::from_svg_icon(res!("assets/icons/binder/book.svg")).icon_size(20.0);
            list = list.child(work_row(
                icon,
                ex.title.to_string(),
                ex.blurb.to_string(),
                None,
                move |ctx| ex_vm.open_example(file_name, bytes, ctx),
            ));
        }

        bati!(
            VStack {
                spacing: 0.0
                Padding::symmetric(12.0, 16.0) {
                    GroupHeader::new(lit!("Examples")) {
                        style: TextStyleRole::SmallBold
                        color: TextRole::Secondary
                    }
                }
                Expand::vertical {
                    ScrollArea {
                        Padding::symmetric(6.0, 12.0) {
                            child: list
                        }
                    }
                }
            }
        )
    }
}

/// One recent/example row: 36 dp icon chip · (title / subtitle) · optional
/// trailing meta. Whole row is the click target.
fn work_row(
    icon: IconWidget,
    title: String,
    subtitle: String,
    trailing: Option<String>,
    on_click: impl Fn(&mut EventContext) + 'static,
) -> impl Widget + 'static {
    // Surface the title as the row's accessible name (a bare HStack has none).
    let name = title.clone();
    bati!(
        Padding::symmetric(8.0, 10.0) {
            HStack {
                spacing: 12.0
                // Whole row is the click target. These `WidgetBuilder` methods
                // attach last regardless of source order (bati! reorders them).
                cursor: CursorIcon::Pointer
                focusable: true
                access_label_literal: name
                on_tap: move |_event, ctx| on_click(ctx)
                // 36 dp icon chip.
                FixedSize {
                    bind_width: 36.0
                    bind_height: 36.0
                    Center {
                        child: icon
                    }
                }
                // Title / subtitle body, claiming the slack.
                Expand::horizontal {
                    VStack {
                        spacing: 2.0
                        TextWidget::new(lit!(title)) {
                            style: TextStyleRole::BodyBold
                            color: TextRole::Primary
                            single_line
                            overflow: TextOverflow::Ellipsis(EllipsisMode::Trailing)
                        }
                        TextWidget::new(lit!(subtitle)) {
                            style: TextStyleRole::Small
                            color: TextRole::Secondary
                            single_line
                            overflow: TextOverflow::Ellipsis(EllipsisMode::Middle)
                        }
                    }
                }
                // Optional trailing meta (e.g. last-opened date).
                if let Some(t) = trailing {
                    TextWidget::new(lit!(t)) {
                        style: TextStyleRole::Small
                        color: TextRole::Secondary
                    }
                }
            }
        }
    )
}

/// Centered muted placeholder for the not-yet-designed Learn/About panes.
fn placeholder(text: &'static str) -> impl Widget + 'static {
    bati!(
        Center {
            TextWidget::new(lit!(text)) {
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
        // All logic on the view-model; rebuilt here from the live store.
        let vm = WelcomeViewModel::new(ctx.settings(), self.app_ctx.clone());

        // Refresh the recents list when a work loads (the model bumps `version`).
        self.recents.wire(ctx);
        self.recents.version_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );
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
                    TextWidget::new(lit!("A quiet place to write long things.")) {
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
            NavItem::Works => lit!("Works"),
            NavItem::Examples => lit!("Examples"),
            NavItem::Learn => lit!("Learn"),
            NavItem::About => lit!("About"),
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
            .child(placeholder("Guides and tips are coming soon."))
            .child(placeholder(
                "Skribisto — a Rust + Bastyde rewrite of the writing app.",
            ));

        // Deterministic heights: `HStack` defaults to `VAlignment::Center` (no
        // stretch), so a content-sized sidebar would float in the middle. Both
        // columns are pinned to the body height (548 − 44 header − 1 divider =
        // 503) so they fill it exactly and the sidebar's `Spacer` can push the
        // nav to the bottom.
        const BODY_H: f32 = 503.0;

        // Hard-bound the whole card to the modal size so the greedy inner
        // `Expand`s fill exactly 780×548 (otherwise they stretch to the window
        // height and the sidebar overflows below the card).
        let root = bati!(ctx =>
            FixedSize {
                bind_width: 780.0
                bind_height: 548.0
                // The Raised card is the modal's lighter surface.
                Panel {
                    variant: PanelVariant::Raised
                    corner_radius: 10.0
                    padding: 0.0
                    VStack {
                        spacing: 0.0
                        // ── Header strip: title + close (full width, fixed 44 dp).
                        // `Expand::horizontal` claims the VStack's full width; the
                        // height-only `FixedSize` alone would leave the strip at its
                        // natural (collapsed) width and squash the title.
                        Expand::horizontal {
                            FixedSize {
                                bind_height: 44.0
                                Padding::symmetric(8.0, 14.0) {
                                    HStack {
                                        spacing: 8.0
                                        Expand::horizontal {
                                            TextWidget::new(lit!("Welcome to Skribisto")) {
                                                style: TextStyleRole::Small
                                                color: TextRole::Secondary
                                            }
                                        }
                                        IconButton::clear() {
                                            tooltip: lit!("Close")
                                            on_activate_fn: |ctx| ctx.dismiss_modal()
                                        }
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
                            // Sidebar: brand block, a Spacer, the bottom-pinned nav,
                            // then the startup checkbox.
                            FixedSize {
                                bind_width: 264.0
                                bind_height: BODY_H
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
                                            bind_height: 136.0
                                            child: bar
                                        }
                                        // Inline "show at startup" checkbox (binds the
                                        // same persisted setting as the Settings panel).
                                        Padding::symmetric(8.0, 8.0) {
                                            Checkbox::new(vm.show_welcome()) {
                                                label: lit!("Show at startup")
                                            }
                                        }
                                    }
                                }
                            }
                            // Vertical rule between the sidebar and the content pane.
                            FixedSize {
                                bind_height: BODY_H
                                Divider::vertical()
                            }
                            // Two-tone: the content pane sits on a darker (Sunken)
                            // base. A `Panel` stretches its child (unlike `ZStack`,
                            // which centres and collapses the greedy content).
                            Expand::horizontal {
                                FixedSize {
                                    bind_height: BODY_H
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
