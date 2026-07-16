// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Welcome content — Skribisto's start screen, hosted as the Launcher
//! window's root (see [`crate::windows::launcher_window_config`]) rather than
//! a modal — the launcher-window model. Two panes: a left **sidebar** (brand
//! block + a bottom-pinned vertical nav) and a right **content** area that
//! switches with the nav selection.
//!
//! **It IS the window, it is not a card inside one.** The two columns fill the
//! Launcher edge to edge: no rounded `Panel` card, no gutter, and no inner
//! title strip — the window's own [`TitleBar`](bastyde::widgets::TitleBar)
//! carries "Welcome to Skribisto" (`welcome_title()`, set in
//! `windows::launcher_window_config`). Back when this was a modal it was a
//! fixed 780×548 card centred in the window, which — once the modal became a
//! real window — read as a second window drawn inside the first: a raised
//! rectangle floating in a 20 dp margin, under a title strip that repeated
//! what the title bar above it already said. Keep this greedy: anything that
//! pins the root to a fixed size brings the gutters back.
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

use bastyde::canvas::EdgeInsets;
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
use bastyde::widgets::styles::{RecipeButtonStyle, RecipeStandardItemStyle};
use bastyde::widgets::{
    ActivateOn, Button, ButtonVariant, Center, Divider, Expand, HStack, IconLocation, IconWidget,
    ListView, MinSize, Padding, Panel, SearchField, Spacer, StandardListItem, Switcher, TabBar,
    TabDelegate, TabId, TabIndicatorPosition, TabSizing, TextWidget, VStack,
};

use frontend::AppContext;

use crate::models::ExamplesListModel;
use crate::view_models::{DISCORD_URL, GITHUB_URL, WelcomeViewModel};

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
    /// The Welcome view-model — **single-instance live state** (it owns the
    /// search query and the recents projection wired to it), so it is created
    /// once on the first `build` (`ctx.settings()` and the window factory only
    /// exist there) and shared by `.clone()` from then on. Rebuilding it per
    /// build would hand the `SearchField` a fresh query signal every frame —
    /// the field would write into one and the list would filter on another.
    vm: Option<WelcomeViewModel>,
    examples: ExamplesListModel,
    /// Bar selection (source of truth); seeded to the first tab so Works shows.
    selected_tab: Signal<Option<TabId>>,
    /// Stable per-tab ids (index ↔ id), shared by the bar's `id_of` and the
    /// content `Switcher`'s derived index.
    tab_ids: Vec<TabId>,
    /// Keyboard/pointer cursor for the examples list. **Required for any row
    /// highlight to exist at all:** `ListView` hands its row delegate
    /// `selection.map(|s| s.is_selected(i)).unwrap_or(false)` — with no
    /// selection model attached, every row is told it is unselected forever, so
    /// arrow keys move the view's internal `focused_index` (Enter still opens
    /// the right row) while nothing on screen — and nothing in the AccessKit
    /// tree — says where the cursor is. Held on the panel, not built inside
    /// `build`, so the highlight survives a rebuild. (The recents list's cursor
    /// lives on the view-model instead — it has to move with the *filter*, not
    /// just with the pointer.)
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
            vm: None,
            examples: ExamplesListModel::new(app_ctx.clone()),
            app_ctx,
            selected_tab,
            tab_ids,
            // Single: this list is a launch target — you open one example, so a
            // multi-select cursor would be meaningless.
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
                            SearchField::new(vm.search_query()) {
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
    /// (see [`RecentRow`]) — or a muted placeholder when there are none, or when
    /// the search matched none. All three arms sit under a [`Switcher`] keyed off
    /// [`WelcomeViewModel::recents_page`], so the region resolves to one widget
    /// type (only the active page is mounted) — keep the child order in lock-step
    /// with the `RECENTS_PAGE_*` constants.
    ///
    /// **Everything here reads the view-model's *projection*, never the raw MRU.**
    /// With a query active the two disagree, and the index a `ListView` hands
    /// back is an index into what it was given — resolving it against the
    /// unfiltered model is how a click on the one visible search hit opens a
    /// different project.
    fn recents_list(&self, vm: &WelcomeViewModel, ctx: &mut BuildContext) -> impl Widget + 'static {
        // Open with the most recent work under the cursor — the launcher
        // convention: Enter resumes your last project without aiming first, and
        // the arrow keys step from a row you can actually see (`ListView` adopts
        // a preset selection as its keyboard cursor).
        vm.preselect_first();

        let row_vm = vm.clone();
        let list = ListView::from_source(vm.recents_source(), |_i, dto, selected| {
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
        .selection(vm.recents_selection())
        .auto_item_height(52.0)
        .on_activate(move |i, ctx| {
            if let Some(path) = row_vm.recent_path(i) {
                row_vm.open_work(path, ctx);
            }
        });

        // Mount the list by id so the panel can hand it back as its
        // `initial_focus_hint` — the window opens with the list already focused,
        // so Enter resumes the highlighted project with no Tab first. Published
        // only when the list is the mounted page: otherwise the Switcher shows a
        // note instead, and focusing an unmounted list would be a dead focus.
        let list_id = ctx.add(list);
        self.recents_list_id
            .set((vm.visible_recents() > 0).then_some(list_id));

        Switcher::new(vm.recents_page())
            .child(empty_note(tr!(welcome_empty_recents()))) // RECENTS_PAGE_EMPTY
            .child(empty_note(tr!(welcome_no_matches()))) // RECENTS_PAGE_NO_MATCH
            .child_id(list_id) // RECENTS_PAGE_LIST
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

        // Preselect the first example, exactly as `recents_list` does — and for
        // the same reason. Attaching a `SelectionModel` only makes a highlight
        // *possible*; something has to put the cursor somewhere for one to
        // exist. Recents get theirs seeded here; examples used to get theirs
        // from nowhere at all: rows are `ActivateOn::SingleClick`, so a click
        // opens the example and tears the Launcher down rather than leaving a
        // selected row behind, and the pane is never the window's initial focus.
        // The result was a list that never showed a cursor in any state.
        // Guarded on "nothing selected yet" so a rebuild can't yank the
        // highlight back to the top after the user has arrowed away from it.
        if !model.is_empty() && self.examples_selection.selected_indices().is_empty() {
            self.examples_selection.select(0);
        }

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

/// Width of the sidebar column, in dp.
const SIDEBAR_W: f32 = 264.0;

/// Gap between the nav and the social row beneath it, in dp.
const SOCIAL_GAP: f32 = 10.0;

/// Sidebar insets, in dp — `Padding::new`'s order: top, trailing, bottom,
/// leading. The top one replaces the height the old inner title strip used to
/// give the brand block; the side ones keep the brand — and the full-width nav
/// pills — off the window edge and the divider; the bottom one keeps the social
/// row off the window's bottom edge.
const SIDEBAR_INSETS: (f32, f32, f32, f32) = (14.0, 16.0, 14.0, 16.0);

/// Size of the two link marks, in dp.
const SOCIAL_ICON: f32 = 18.0;

/// The chrome of a flat icon button: IntUI's **Ghost** recipe (transparent fill
/// and border until hover, where it takes the standard `SurfaceRole::Hover`
/// wash) with one thing changed — its footprint.
///
/// Every stock recipe, Ghost included, carries `min_size: 72 × 24`: the width a
/// *text* button needs so that "OK" and "Cancel" line up in a dialog. On an
/// icon-only button that is nothing but dead area — invisible at rest, but the
/// hover wash paints it, so an 18 dp mark would light up a 72 dp pill on
/// mouseover, and two of them side by side would read as a button bar rather
/// than a pair of glyphs. Squaring the box to the icon + its padding makes the
/// hover state the icon's own outline, which is what "flat icon button" means
/// everywhere else.
///
/// The recipe is edited rather than written from scratch so this stays a Ghost
/// button — the fill/border/focus-ring states, and any theme that reshapes them,
/// still apply.
fn flat_icon_button_style() -> RecipeButtonStyle {
    let mut style = RecipeButtonStyle::intui();
    let ghost = style
        .recipes
        .get_mut(&ButtonVariant::Ghost)
        .expect("IntUI defines a Ghost recipe");
    // `EdgeInsets::symmetric` takes (horizontal, vertical).
    ghost.padding = EdgeInsets::symmetric(6.0, 4.0);
    ghost.min_size = Size::new(SOCIAL_ICON + 12.0, SOCIAL_ICON + 8.0);
    style
}

/// One flat icon link — an icon-only ghost [`Button`] that
/// [`WelcomeViewModel::open_link`] opens in the browser, re-declared to
/// assistive tech as a **link** rather than a button.
///
/// **Why `.access_role(Role::Link)`.** A `Button` announces as `Role::Button`
/// unconditionally ([`Button::accessibility`] hardcodes it) — but these controls
/// *navigate* to an external URL, and for a screen-reader user "link" and
/// "button" are different things they reach with different keys, from different
/// lists. The generic builder override wins here because the a11y walker applies
/// it *after* the widget's own `accessibility()`
/// (`build_overridden_builder`: `widget.accessibility(b)` then
/// `overrides.apply(b)`, whose `set_role` is unconditional), so no bespoke
/// link-with-an-icon widget — or framework change — is needed to get the right
/// role. Built as a plain builder (not a `bati!` `Button {}` block) because the
/// override wraps the button in a `WidgetWithHandlers`, which must be the last
/// call in the chain — after every `Button`-specific one.
///
/// `label` serves triple duty: the drawn content drops it
/// ([`IconLocation::IconOnly`]), but `Button` still reads it for the AT name,
/// and it is the tooltip — one string, so the tooltip and the spoken name can't
/// drift.
///
/// The chrome is [`flat_icon_button_style`]; the mark stays
/// [`IconMode::Tintable`] (see [`social_links`]).
fn link_button(
    icon: IconWidget,
    label: LocalizedString,
    on_activate: impl Fn(&mut EventContext) + 'static,
) -> impl Widget + 'static {
    Button::new(label.clone())
        .variant(ButtonVariant::Ghost)
        .style(flat_icon_button_style())
        .icon(icon, IconLocation::IconOnly)
        .tooltip(label)
        .on_activate_fn(on_activate)
        // Last: this wraps the Button, so every Button-specific call is above it.
        .access_role(bastyde::core::accesskit::Role::Link)
}

/// The project's two public links, tucked under the sidebar nav — the pair
/// v1.9.x offered: the GitHub repository and the Discord server. Each is a flat
/// icon [`link_button`].
///
/// **Both marks stay [`IconMode::Tintable`]** (the `IconWidget` default), so
/// they resolve through the button's text role: they follow the theme into dark
/// mode and pick up the same hover/press tint as everything else in the sidebar.
/// The shipped artwork cannot do that by itself — the Octicons mark is a
/// near-black `#1B1F23` silhouette (invisible on a dark sidebar) and the Discord
/// logo is brand blurple, so drawing either in its own colors would leave a
/// two-icon footer that agrees with neither the theme nor itself. Tinting costs
/// only the blurple, which is not information here: the shape already says
/// "Discord".
fn social_links(vm: &WelcomeViewModel) -> impl Widget + 'static {
    // The Discord logo's viewBox is 71×55, not square — `SvgIcon` fits it into
    // the icon box preserving aspect and centring, so it lands ~18×14 next to
    // the square GitHub mark. That is the logo's own proportion, not a squash.
    let github_icon =
        IconWidget::from_svg_icon(res!("../../resources/icons/Octicons-mark-github.svg"))
            .icon_size(SOCIAL_ICON);
    let discord_icon =
        IconWidget::from_svg_icon(res!("../../resources/icons/Discord-Logo-Color.svg"))
            .icon_size(SOCIAL_ICON);

    let github_vm = vm.clone();
    let discord_vm = vm.clone();
    bati!(
        Center {
            HStack {
                spacing: 4.0
                child: link_button(github_icon, tr!(welcome_github()), move |ctx| {
                    github_vm.open_link(GITHUB_URL, ctx)
                })
                child: link_button(discord_icon, tr!(welcome_discord()), move |ctx| {
                    discord_vm.open_link(DISCORD_URL, ctx)
                })
            }
        }
    )
}

/// The sidebar column: the brand block at the top, then — pushed to the bottom
/// by the `Spacer` between them — the nav with the social row under it.
///
/// Split out of [`WelcomePanel::build`] for the same reason as
/// [`welcome_body`]: the geometry is what regresses, and it can be laid out
/// headlessly (see this module's tests) while the real panel needs a live
/// backend. Nothing here decides what the three blocks *contain*.
///
/// The `Spacer` is what pins the bottom group, so nothing in this column may be
/// given an unbounded height — a width-only `FixedSize` around the nav (which
/// proposes `None` on the other axis) would collapse the `Spacer` and float the
/// nav up under the brand block. It is also why the nav takes no height pin at
/// all: a vertical `TabBar` already reports its own content height, and pinning
/// it would additionally hide the sidebar's width from it and defeat
/// `TabSizing::Fill`.
///
/// No "show at startup" control here — that setting lives in Settings ▸
/// Appearance & Behaviour only (a launcher-local copy would hide the very screen
/// you're looking at, with no way back).
fn welcome_sidebar(
    branding: impl Widget + 'static,
    nav: impl Widget + 'static,
    links: impl Widget + 'static,
) -> impl Widget + 'static {
    let (top, trailing, bottom, leading) = SIDEBAR_INSETS;
    bati!(
        Padding::new(top, trailing, bottom, leading) {
            VStack {
                spacing: 0.0
                child: branding
                Spacer
                child: nav
                Padding::new(SOCIAL_GAP, 0.0, 0.0, 0.0) {
                    child: links
                }
            }
        }
    )
}

/// The Welcome body: fixed-width **sidebar** · vertical rule · flexible
/// **content pane**, all three filling whatever the window offers — the whole
/// layout, in one expression. Split out of [`WelcomePanel::build`] so the
/// geometry can be laid out headlessly (see this module's tests): the panel
/// itself needs a live backend and the project-window factory, its two panes
/// don't decide any of the column geometry, and the column geometry is exactly
/// what regressed.
///
/// **Why `MinSize::width` and not `FixedSize::width` for the sidebar.** A
/// `FixedSize` proposes `None` on the axis it doesn't bind, so a width-only one
/// would hand the sidebar an *unbounded height* — its `Spacer` would collapse
/// and the nav would ride up under the brand block instead of pinning to the
/// bottom. `MinSize` clamps the axis it constrains and passes the other one
/// through untouched, so the sidebar is measured at 264 × the body height. It
/// can't overflow that width either: the brand text wraps
/// (`TextOverflow::Wrap`) against the 264 dp it is offered.
///
/// Each column claims the body height without a pin: `HStack` offers its height
/// to every child (and defaults to `VAlignment::Center`, so a *content*-sized
/// column would float in the middle instead). `Expand::vertical` takes that
/// offered height while reporting `flex = 0` on the horizontal axis the stack
/// is distributing — it claims height without stealing width from the content
/// pane, which is the one column that does compete for it
/// (`Expand::horizontal`).
fn welcome_body(
    sidebar: impl Widget + 'static,
    content: impl Widget + 'static,
) -> impl Widget + 'static {
    bati!(
        HStack {
            spacing: 0.0
            MinSize::width(SIDEBAR_W) {
                child: sidebar
            }
            // Vertical rule between the sidebar and the content pane.
            Expand::vertical {
                Divider::vertical()
            }
            Expand::horizontal {
                child: content
            }
        }
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
        // All logic on the view-model — created **once** (it owns the live search
        // query and the recents projection reading it; a per-build instance would
        // give the `SearchField` a new query signal every frame, so typing would
        // filter a list nobody is looking at) and shared by clone from then on.
        // The project-window factory is process-wide `app_state` (registered once
        // in `main`), shared by clone rather than reconstructed.
        let factory = ctx
            .app_state::<crate::windows::ProjectWindowFactory>()
            .cloned()
            .expect("ProjectWindowFactory registered in main");
        let app_ctx = self.app_ctx.clone();
        let vm = self
            .vm
            .get_or_insert_with(|| WelcomeViewModel::new(ctx.settings(), app_ctx, factory))
            .clone();

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
        vm.wire(ctx);
        self.examples.wire(ctx);

        // ── Brand block (top of the sidebar) ───────────────────────────────
        let mut title_style = ctx.theme().typography.body_bold.clone();
        title_style.size = 22.0;
        // The tagline is set in one of the bundled writing serifs, italic, so the
        // brand block closes on a line that looks *written* rather than chromed.
        // The italic run comes from the `*…*` in the tagline's ftl value, which
        // only means anything with `markup: true` below — without it the asterisks
        // render literally. It needs the family's italic face: every serif in
        // `register_editor_fonts` ships one, upright and italic under the same
        // family name. EB Garamond is the calligraphic one of the three.
        // `small` (12 px) is too small for Garamond's short x-height, hence 15.
        let mut tagline_style = ctx.theme().typography.small.clone();
        tagline_style.family = "EB Garamond".to_string();
        tagline_style.size = 15.0;
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
                    TextWidget::new(tr!(welcome_version(version = crate::version::app_version()))) {
                        style: TextStyleRole::Small
                        color: TextRole::Secondary
                    }
                    TextWidget::new(tr!(welcome_tagline())) {
                        style: tagline_style
                        color: TextRole::Secondary
                        markup: true
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
        // Nav-rail sizing: the pills span the sidebar instead of shrinking to
        // their label ("Learn" would otherwise be a stub next to "Examples").
        // The default `Shared` fits the widest label; `Fill` takes the width
        // the sidebar column offers — which is why the bar must not be wrapped
        // in a height-only `FixedSize` (that proposes `width: None` and there
        // would be nothing to fill).
        .tab_sizing(TabSizing::Fill)
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

        // ── Sidebar: brand block, a Spacer, the bottom-pinned nav + links ───
        let sidebar = welcome_sidebar(branding, bar, social_links(&vm));

        // Two-tone: the content pane sits on a darker (Sunken) base, against the
        // sidebar's window surface (`surface_main` — the same fill the title bar
        // above it paints, which is what makes the sidebar read as part of the
        // window chrome rather than as a card on top of it). A `Panel` stretches
        // its child (unlike `ZStack`, which centres and collapses the greedy
        // content).
        let content_pane = bati!(
            Panel {
                variant: PanelVariant::Sunken
                corner_radius: 0.0
                padding: 0.0
                child: content
            }
        );

        let root = ctx.add(welcome_body(sidebar, content_pane));
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
        // Delegate to the body: it is greedy, and the Launcher wants it that way
        // — this panel *is* the window's content, so it takes the whole
        // proposal. (Filling like this is the trap `SettingsPanel` documents,
        // where a modal must NOT fill its host; here it is the requirement.)
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;
    use bastyde::widgets::FixedSize;

    /// A stand-in block of a known size — the sidebar test cares where the three
    /// blocks land, not what they draw.
    fn block(w: f32, h: f32) -> FixedSize {
        FixedSize::new().width(w).height(h).child(Spacer::new())
    }

    /// **The two link marks announce to assistive tech as *links*, not buttons.**
    /// They navigate to an external URL, so "link" is the role a screen-reader
    /// user expects — and it is what `.access_role(Role::Link)` in
    /// [`link_button`] declares, overriding the `Role::Button` a `Button` emits
    /// by default. This is the crux of the whole helper; assert the role really
    /// reaches the AT tree (and that the overridden Button role leaves nothing
    /// behind on a sibling node).
    #[test]
    fn link_button_announces_as_a_link() {
        use bastyde::core::accessibility::widget_id_to_node_id;
        use bastyde::core::accesskit::Role;

        let icon =
            IconWidget::from_svg_icon(res!("../../resources/icons/Octicons-mark-github.svg"))
                .icon_size(SOCIAL_ICON);

        let mut tree = WidgetTree::new().with_theme(bastyde::presets::intui::light());
        let id = tree.add_boxed(Box::new(link_button(icon, lit!("GitHub"), |_| {})));
        tree.layout(SizeProposal::exact(120.0, 40.0));
        let _ = tree.render();
        let update = tree.sync_accessibility();

        let (_, node) = update
            .nodes
            .iter()
            .find(|(nid, _)| *nid == widget_id_to_node_id(id))
            .expect("the link button emits an AT node");
        assert_eq!(
            node.role(),
            Role::Link,
            "it opens a URL — a link, not a button"
        );
        assert_eq!(node.label(), Some("GitHub"), "…named by its label");
        assert!(
            !update.nodes.iter().any(|(_, n)| n.role() == Role::Button),
            "the Button role is overridden in place, not left on a sibling node"
        );
    }

    /// **The social links sit under the nav, and the pair stays pinned to the
    /// sidebar's bottom.** The `Spacer` above them is the only thing holding
    /// them there, so this is the piece that breaks if anything in the column is
    /// ever handed an unbounded height (see [`welcome_sidebar`]): the group
    /// would ride up under the brand block instead.
    ///
    /// Laid out on stand-in blocks — the nav's own height is `TabBar`'s business,
    /// and the column geometry must not depend on it.
    #[test]
    fn the_social_links_sit_below_the_bottom_pinned_nav() {
        const H: f32 = 546.0;
        const NAV_H: f32 = 136.0; // 4 pills × 34 dp, as the real bar reports
        const LINKS_H: f32 = 30.0;
        let (top, _trailing, bottom, leading) = SIDEBAR_INSETS;

        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(welcome_sidebar(
            block(120.0, 110.0),
            block(SIDEBAR_W, NAV_H),
            block(60.0, LINKS_H),
        )));
        tree.layout(SizeProposal::exact(SIDEBAR_W, H));

        // Padding ▸ VStack ▸ [branding, Spacer, nav, Padding ▸ links]
        let column = tree.children(id)[0];
        let rows = tree.children(column);
        assert_eq!(rows.len(), 4, "branding · Spacer · nav · the links row");
        let branding = tree.bounds(rows[0]);
        let nav = tree.bounds(rows[2]);
        let links = tree.bounds(rows[3]);

        assert_eq!(branding.y, top, "the brand block stays at the top");
        assert_eq!(branding.x, leading, "…inside the leading inset");

        assert_eq!(
            links.y,
            nav.bottom(),
            "the links row starts where the nav ends (its own top gap is inside it)"
        );
        assert_eq!(
            links.bottom(),
            H - bottom,
            "the links row — not the nav — is what now rides the bottom inset"
        );
        assert_eq!(
            nav.bottom(),
            H - bottom - LINKS_H - SOCIAL_GAP,
            "the nav is pushed up by exactly the links row and its gap"
        );
        assert!(
            nav.y > branding.bottom(),
            "the Spacer still holds the bottom group away from the brand block"
        );
    }

    /// **The Welcome content is the window, not a card inside it.**
    ///
    /// It used to be a fixed 780×548 `Panel` centred in the 820×590 Launcher —
    /// a leftover from its modal days. Once the modal became a real window that
    /// read as a window drawn inside a window: a raised, rounded rectangle
    /// floating in a ~20 dp gutter, topped by its own title strip repeating the
    /// title bar right above it. So: the body starts at x = 0, the content pane
    /// ends at the right edge, and both columns are as tall as the window.
    ///
    /// Laid out on stand-in panes — the columns' geometry is what is under
    /// test, and neither pane has a say in it. The sidebar stub keeps a
    /// `Spacer`, like the real one: it is the piece that needs a *bounded*
    /// height to push the nav to the bottom, so a sidebar that failed to fill
    /// the body would collapse here rather than pass by accident.
    #[test]
    fn the_body_fills_the_window_edge_to_edge() {
        const W: f32 = 820.0;
        const H: f32 = 546.0;

        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(welcome_body(
            VStack::new().spacing(0.0).child(Spacer::new()),
            Spacer::new(),
        )));
        tree.layout(SizeProposal::exact(W, H));

        let body = tree.bounds(id);
        assert_eq!(body.x, 0.0, "the body starts at the window's left edge");
        assert_eq!(body.width, W, "the body spans the window's full width");

        let cols = tree.children(id);
        assert_eq!(cols.len(), 3, "sidebar · rule · content pane");
        let sidebar = tree.bounds(cols[0]);
        let rule = tree.bounds(cols[1]);
        let pane = tree.bounds(cols[2]);

        assert_eq!(sidebar.x, 0.0, "no gutter to the left of the sidebar");
        assert_eq!(sidebar.width, SIDEBAR_W);
        assert_eq!(sidebar.height, H, "the sidebar fills the body height");
        assert_eq!(rule.x, sidebar.right(), "the rule abuts the sidebar");
        assert_eq!(rule.height, H, "the rule runs the full height");
        assert_eq!(pane.x, rule.right(), "the pane abuts the rule");
        assert_eq!(
            pane.right(),
            W,
            "no gutter to the right of the content pane"
        );
        assert_eq!(pane.height, H, "the content pane fills the body height");
    }
}
