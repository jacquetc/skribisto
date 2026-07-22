// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Project switcher for the title bar.
//!
//! A flat dropdown button whose main slot shows the **currently-open project's**
//! title (or "No work loaded") plus a trailing chevron. Its popover has two
//! sections:
//!
//! - **Currently open** — every project open across running instances (from the
//!   cross-process [`open_registry`]). This instance's own project is
//!   check-marked; clicking another instance's project raises that window (via an
//!   IPC "raise" message carrying a freshly-minted xdg-activation token).
//! - **Recent** — recent projects not currently open. Clicking one asks
//!   (MessageBox) whether to open it in a new window (a new process) or here.
//!
//! **On-open refresh:** the registry scan is stale the moment another instance
//! opens/closes a project, so the popover content is re-scanned **each time the
//! popover opens**. That's done by moving the content into [`OpenProjectsMenu`],
//! which rebuilds on a shared `open_epoch` signal that `PopoverButton::on_open`
//! bumps — the content subtree rebuilds in place, so the button (and the open
//! popover) is *not* recreated. The trigger label stays on `ProjectSwitcherButton`
//! and rebuilds only on the open Work's title / file_name / recents `version`.

use std::path::Path;
use std::rc::Rc;

use bastyde::core::BindingLevel;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, FixedSize, FocusScope, GroupHeader, HStack, IconWidget, MaxSize,
    MenuList, MessageBox, MessageBoxButton, MessageBoxButtons, Padding, PopoverButton, Spacer,
    StandardButton, TextWidget, TraversalScopePolicy, VStack,
};

use frontend::AppContext;

use crate::models::RecentWorkListModel;
use crate::shell::open_registry;
use crate::view_models::project_switcher as vm;
use crate::singles::{SingleWork, SingleWorkInfo};

/// Cap on a popover row's text column.
///
/// This is what makes the popover a definite, readable width. `MenuList` has no
/// width setting — it sizes to its items — and a `TextWidget` sizes to *its*
/// content, so without a cap here the rows grew to the full natural width of a
/// long path (~600px), overflowed the popover, and were clipped to an unreadable
/// middle slice. Capping the text column caps the whole popover.
const ROW_TEXT_MAX_WIDTH: f32 = 340.0;

/// A rich two/three-line popover row (checkmark column + title + path [+ date]).
fn row(
    checked: bool,
    accent: bool,
    title: String,
    path: String,
    date: Option<String>,
    on_tap: impl Fn(&mut EventContext) + 'static,
) -> impl Widget {
    let mut marker = FixedSize::new().width(16.0).height(16.0);
    if checked {
        marker = marker.child(IconWidget::checkmark(14.0));
    }
    let title_color = if accent {
        TextRole::Accent
    } else {
        TextRole::Primary
    };
    // The cap belongs on the *text*, not on the column around it. A `TextWidget`
    // sizes to its content and only ellipsizes against a bounded proposal, and
    // `MenuList` (a hugging container) measures its items unbounded — so an
    // uncapped path lays out at its full ~700px and drags the popover with it.
    // `MaxSize` proposes `min(parent, cap)` down to the text, so the ellipsis
    // engages, and reports `min(child, cap)`, so a short line still hugs.
    //
    // Capping the *column* instead cannot hug: a `VStack` fills whatever width it
    // is offered on its cross axis, so it would report the cap for every row, long
    // or short. (That is what the `FixedSize` here used to do — it pinned every row
    // to 340 and only forced the bounded proposal, engaging the ellipsis, as a
    // side effect.)
    let capped = |w: TextWidget| MaxSize::width(ROW_TEXT_MAX_WIDTH).child(w);
    let mut body = VStack::new()
        .spacing(2.0)
        // Lines are left-aligned against each other: once each line hugs its own
        // text, a centred column would stagger the title over its path.
        .alignment(bastyde::tokens::HAlignment::Leading)
        .child(capped(
            // Titles can be long too ("Faux-Semblants — brouillon 3"), and a
            // wrapped title would make rows ragged. Trailing ellipsis: a title's
            // identity is at its start.
            TextWidget::new(lit!(title))
                .style(TextStyleRole::BodyBold)
                .color(title_color)
                .single_line()
                .overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing)),
        ))
        .child(capped(
            // Middle, not trailing: a path ends in the filename, which is the
            // one part that tells two projects apart.
            TextWidget::new(lit!(path))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary)
                .single_line()
                .overflow(TextOverflow::Ellipsis(EllipsisMode::Middle)),
        ));
    if let Some(date) = date {
        body = body.child(
            TextWidget::new(lit!(date))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        );
    }
    let content = HStack::new().spacing(8.0).child(marker).child(body);
    // Rows hug their own content now, so they no longer all measure the same
    // width — and a hugging row placed in a wider list is *centred* in it, which
    // staggered the short-path rows to the right of the long ones. The trailing
    // `Spacer` absorbs that slack instead, keeping every row flush left. It sits
    // in its own zero-spacing HStack so it adds no gap to the row's natural width
    // (`content`'s own 8px spacing would otherwise apply to it too), and the tap
    // target stays the full row width rather than shrinking to the content.
    let inner = HStack::new()
        .spacing(0.0)
        .child(content)
        .child(Spacer::new())
        .cursor(CursorIcon::Pointer)
        .focusable(true)
        .on_tap(move |_event, ctx| on_tap(ctx));
    Padding::symmetric(6.0, 10.0).child(inner)
}

/// A non-navigable section caption for the popover.
fn section(label: bastyde::i18n::LocalizedString) -> impl Widget {
    Padding::symmetric(6.0, 8.0).child(GroupHeader::new(label))
}

/// The popover content: the two-section menu, re-scanned on `open_epoch` bumps
/// (fired by the button's `on_open`) so it reflects other instances' current
/// state each time it is shown — without rebuilding the enclosing button.
struct OpenProjectsMenu {
    model: RecentWorkListModel,
    /// The current Work's title/path, for the "Currently open" section's
    /// check-marked row — a constructor parameter (from `ProjectSwitcherButton`,
    /// which is itself handed these by `ProjectWindowFactory::window_config`),
    /// not an independent `ctx.app_state` lookup: see `sessions::WorkSession`'s
    /// module doc on why "the Work" should be resolved through the seam a
    /// window was actually given, not re-fetched from scratch.
    single_work: SingleWork,
    single_work_info: SingleWorkInfo,
    open_epoch: Signal<u64>,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for OpenProjectsMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenProjectsMenu").finish()
    }
}

impl Widget for OpenProjectsMenu {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let single_work = self.single_work.clone();
        let single_work_info = self.single_work_info.clone();

        self.model.wire(ctx);
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.model
            .version_signal()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        single_work.title().bind_to(sid, reg, BindingLevel::Rebuild);
        single_work_info
            .file_name()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        // Re-scan the registry whenever the popover opens.
        self.open_epoch.bind_to(sid, reg, BindingLevel::Rebuild);

        let recents = self.model.items();

        // Which project belongs in which section is derived (and tested) in
        // `view_models::project_switcher`; this build only renders the answer.
        let split = vm::sections(open_registry::scan(), &recents, open_registry::my_pid());
        let has_open = !split.open.is_empty();
        let has_recent = !split.recent.is_empty();

        let mut menu = MenuList::new().max_visible_items(12);

        if split.is_empty() {
            menu = menu.item(
                Padding::symmetric(8.0, 12.0).child(
                    TextWidget::new(tr!(no_recent_works()))
                        .style(TextStyleRole::Body)
                        .color(TextRole::Secondary),
                ),
            );
        } else {
            if has_open {
                menu = menu.header(section(tr!(switcher_open_section())));
                for entry in &split.open {
                    let is_self = entry.is_self;
                    let title = entry.title.clone();
                    let path = entry.path.clone();
                    let pid = entry.pid;
                    menu = menu.item(row(is_self, is_self, title, path, None, move |ctx| {
                        ctx.dismiss_self_overlay_chain();
                        if is_self {
                            return; // already this window
                        }
                        vm::raise_instance(ctx, pid);
                    }));
                }
            }

            if has_open && has_recent {
                menu = menu.separator();
            }

            if has_recent {
                menu = menu.header(section(tr!(switcher_recent_section())));
                for dto in &split.recent {
                    let title = dto.title.clone();
                    let path = dto.absolute_path.clone();
                    let date = dto.last_opened_at.format("%Y-%m-%d %H:%M").to_string();
                    menu = menu.item(row(
                        false,
                        false,
                        title.clone(),
                        path.clone(),
                        Some(date),
                        move |ctx| {
                            ctx.dismiss_self_overlay_chain();
                            let for_new = path.clone();
                            let for_here = path.clone();
                            MessageBox::question(tr!(open_project_title()))
                                .text(tr!(open_project_question(title = title.clone())))
                                .buttons(MessageBoxButtons::Custom(vec![
                                    MessageBoxButton::standard(StandardButton::Open)
                                        .label(tr!(open_in_new_window())),
                                    MessageBoxButton::standard(StandardButton::Yes)
                                        .label(tr!(open_here())),
                                    MessageBoxButton::standard(StandardButton::Cancel),
                                ]))
                                .default_button(StandardButton::Open)
                                .escape_button(StandardButton::Cancel)
                                .on_result(move |r, ctx| match r.button {
                                    StandardButton::Open => {
                                        vm::open_in_new_window(ctx, &for_new);
                                    }
                                    // "Open here" *replaces* this window's project, so
                                    // it goes through the `work.open_path` intent →
                                    // the unsaved-changes guard, which loads it once
                                    // the open project is saved or explicitly
                                    // discarded. It used to call `load_work` outright
                                    // and bin those edits without asking.
                                    StandardButton::Yes => vm::open_here(ctx, &for_here),
                                    _ => {}
                                })
                                .present(ctx);
                        },
                    ));
                }
            }
        }

        // Trap Tab inside the anchored (non-centered) overlay.
        let root = ctx.add(FocusScope::new(TraversalScopePolicy::Cycle).child(menu));
        self.root = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0))
            .into()
    }
}

/// Flat dropdown button: current project in the main slot, a two-section popover
/// (open elsewhere / recent) listing everything else.
pub struct ProjectSwitcherButton {
    app_ctx: Rc<AppContext>,
    model: RecentWorkListModel,
    /// The open Work's title/path — a constructor parameter, not an
    /// independent `ctx.app_state` lookup; see [`OpenProjectsMenu`]'s doc.
    single_work: SingleWork,
    single_work_info: SingleWorkInfo,
    /// Bumped by the popover's `on_open`; consumed by [`OpenProjectsMenu`] to
    /// re-scan the registry each time the popover shows. Owned here so it survives
    /// this widget's rebuilds.
    open_epoch: Signal<u64>,
    root_child: Option<WidgetId>,
}

impl ProjectSwitcherButton {
    pub fn new(
        app_ctx: Rc<AppContext>,
        single_work: SingleWork,
        single_work_info: SingleWorkInfo,
    ) -> Self {
        Self {
            model: RecentWorkListModel::new(app_ctx.clone()),
            app_ctx,
            single_work,
            single_work_info,
            open_epoch: Signal::new(0),
            root_child: None,
        }
    }
}

impl std::fmt::Debug for ProjectSwitcherButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectSwitcherButton").finish()
    }
}

impl Widget for ProjectSwitcherButton {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let single_work = self.single_work.clone();
        let single_work_info = self.single_work_info.clone();

        // Trigger label rebuilds on load/title/close — NOT on popover open (that
        // would recreate the PopoverButton and close the just-opened popover).
        self.model.wire(ctx);
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.model
            .version_signal()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        single_work.title().bind_to(sid, reg, BindingLevel::Rebuild);
        single_work_info
            .file_name()
            .bind_to(sid, reg, BindingLevel::Rebuild);

        let recents = self.model.items();
        let open_path = single_work_info.file_name().get();
        let work_title = single_work.title().get();

        let is_open = !work_title.is_empty() || open_path.is_some();
        let current_title: Option<String> = if is_open {
            open_path
                .as_deref()
                .and_then(|p| {
                    recents
                        .iter()
                        .find(|r| r.absolute_path == p)
                        .map(|r| r.title.clone())
                })
                .or_else(|| (!work_title.is_empty()).then(|| work_title.clone()))
                .or_else(|| {
                    open_path.as_deref().and_then(|p| {
                        Path::new(p)
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .map(str::to_string)
                    })
                })
        } else {
            None
        };

        let trigger = Button::new(match current_title {
            Some(t) => lit!(t),
            None => tr!(no_work()),
        })
        .variant(ButtonVariant::Ghost)
        .text_style(TextStyleRole::BodyBold)
        .trailing(IconWidget::chevron_down(12.0));

        let content = OpenProjectsMenu {
            model: RecentWorkListModel::new(self.app_ctx.clone()),
            single_work: self.single_work.clone(),
            single_work_info: self.single_work_info.clone(),
            open_epoch: self.open_epoch.clone(),
            root: None,
        };

        let open_epoch = self.open_epoch.clone();
        let root = ctx.add(
            PopoverButton::new(trigger)
                .show_disclosure_caret(false)
                .bare()
                .content(content)
                // Re-scan the registry each time the popover opens.
                .on_open(move || open_epoch.set(open_epoch.get().wrapping_add(1))),
        );
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0))
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;

    /// A stand-in for [`OpenProjectsMenu`]: focusable rows in a `MenuList`,
    /// rebuilt whenever `epoch` bumps. That is the whole shape that matters
    /// here — the real menu's registry scan is irrelevant to focus.
    struct EpochRows {
        epoch: Signal<u64>,
        root: Option<WidgetId>,
    }

    impl std::fmt::Debug for EpochRows {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("EpochRows").finish()
        }
    }

    impl Widget for EpochRows {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let sid = ctx.self_id();
            let reg = ctx.binding_registry();
            self.epoch.bind_to(sid, reg, BindingLevel::Rebuild);

            let mut menu = MenuList::new().max_visible_items(12);
            for i in 0..3 {
                menu = menu.item(row(
                    false,
                    false,
                    format!("Project {i}"),
                    "/tmp/p.skrib".to_string(),
                    None,
                    |_| {},
                ));
            }
            let root = ctx.add(FocusScope::new(TraversalScopePolicy::Cycle).child(menu));
            self.root = Some(root);
            vec![root]
        }

        fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
            self.root
                .and_then(|id| ctx.child_size(id, proposal))
                .unwrap_or_else(|| proposal.resolve(0.0, 0.0))
                .into()
        }
    }

    /// Clicking the switcher must leave keyboard focus **inside** the popover.
    ///
    /// `PopoverButton` focuses the first focusable descendant of its content
    /// when it opens — but our `on_open` bumps `open_epoch` to re-scan the
    /// registry, and that `BindingLevel::Rebuild` runs in the *next* layout
    /// pass, destroying the very row that was just focused. The tree then drops
    /// focus (`revalidate_interaction_state`), so the popup opens unfocused:
    /// no arrow-key navigation, no Enter.
    #[test]
    fn opening_the_popover_puts_focus_inside_it() {
        let mut tree = WidgetTree::new();
        let epoch = Signal::new(0u64);
        let bump = epoch.clone();

        let pb = PopoverButton::new(Button::new(lit!("Switch")))
            .show_disclosure_caret(false)
            .bare()
            .content(EpochRows {
                epoch: epoch.clone(),
                root: None,
            })
            .on_open(move || bump.set(bump.get().wrapping_add(1)));
        let id = tree.add(pb);
        tree.layout(SizeProposal::exact(600.0, 400.0));

        let trigger = tree
            .first_focusable_descendant(id)
            .expect("PopoverButton exposes a focusable trigger");
        let b = tree.bounds(trigger);
        let center = Point::new(b.x + b.width / 2.0, b.y + b.height / 2.0);
        tree.pointer_down_button(center, PointerButton::Primary);
        tree.pointer_up_button(center, PointerButton::Primary);

        // The open lands focus on the first row here...
        let after_click = tree.focused();
        assert!(
            after_click.is_some_and(|f| f != trigger),
            "the popover should focus its content on open, got {after_click:?}"
        );

        // ...and the on_open-driven content rebuild happens in this pass.
        tree.layout(SizeProposal::exact(600.0, 400.0));

        let focused = tree.focused().expect(
            "focus must stay inside the open popover; the on_open re-scan rebuilt the \
             content and destroyed the focused row, so focus was dropped entirely",
        );
        assert_ne!(
            focused, trigger,
            "focus must be inside the popover, not back on the trigger"
        );
        assert!(
            tree.is_active(focused),
            "the focused widget must be a live node"
        );
    }

    /// A popover row must stay within its cap no matter how long the path is.
    ///
    /// A hugging container measures its items with an unbounded proposal to learn
    /// their natural width — and a `TextWidget` sizes to its content, so an
    /// uncapped path would lay out at its full ~700px and drag the popover with
    /// it. The text column is therefore capped (`ROW_TEXT_MAX_WIDTH`), which makes
    /// the ellipsis engage and gives the row a predictable natural width.
    ///
    /// Remove that cap and this test fails.
    #[test]
    fn a_long_path_cannot_blow_out_the_popover_row() {
        let long = "/home/cyril/Nextcloud/Documents/Livres/Faux-Semblants/\
                    deeply/nested/for/the/purposes/of/this/test/Faux-semblants.skrib";

        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(row(
            false,
            false,
            "A Very Long Project Title That Would Also Overflow".to_string(),
            long.to_string(),
            Some("2026-07-13 10:00".to_string()),
            |_| {},
        )));
        // Propose far more width than the row should take: a correct row clamps
        // itself; the broken one grew to the text's natural width.
        // A hugging container (MenuList) measures its items with an UNBOUNDED
        // proposal to learn their natural width. That is the measurement that
        // decides the popover's width, so it is the one to assert on.
        tree.layout(SizeProposal {
            width: None,
            height: None,
        });
        let b = tree.bounds(id);
        eprintln!("NATURAL ROW WIDTH = {}", b.width);

        // marker (16) + spacing (8) + text column + horizontal padding (2 x 10).
        let ceiling = ROW_TEXT_MAX_WIDTH + 16.0 + 8.0 + 20.0 + 1.0;
        assert!(
            b.width <= ceiling,
            "row must hug to its bounded text column, got {} (ceiling {}). \
             Without a bound the path lays out at full natural width and the \
             popover is dragged out with it.",
            b.width,
            ceiling
        );
        assert!(b.width > 100.0, "row should not collapse either");
    }

    /// The text column is a **cap, not a pin**: a row whose title and path both
    /// fit must hug its content instead of stretching to the cap.
    ///
    /// This is the difference between `MaxSize` (which reports `min(child, cap)`)
    /// and the `FixedSize` it replaced (which reported the cap unconditionally,
    /// and only bounded the proposal — hence engaged the ellipsis — as a side
    /// effect). Swap `MaxSize` back for `FixedSize` and this test fails.
    #[test]
    fn a_short_path_hugs_instead_of_stretching_to_the_cap() {
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(row(
            false,
            false,
            "Novel".to_string(),
            "/tmp/n.skrib".to_string(),
            None,
            |_| {},
        )));
        tree.layout(SizeProposal {
            width: None,
            height: None,
        });
        let b = tree.bounds(id);

        // marker (16) + spacing (8) + text column + horizontal padding (2 x 10).
        let pinned = ROW_TEXT_MAX_WIDTH + 16.0 + 8.0 + 20.0;
        assert!(
            b.width < pinned - 1.0,
            "a short row must hug its content, not stretch to the cap: got {} \
             (a pinned column would report {})",
            b.width,
            pinned
        );
        assert!(b.width > 60.0, "…but it must still hold its content");
    }

    /// A short row sitting under a long one stays flush left with it.
    ///
    /// Rows hug their content, so they no longer all measure the same width — and
    /// the `MenuList` stretches each item to the list width, which *centres* a
    /// narrow row's content and staggered the short-path rows to the right of the
    /// long ones. The trailing `Spacer` absorbs that slack instead. Drop it and
    /// this test fails.
    ///
    /// Asserted through a `MenuList`, because that is where the stretching (and so
    /// the centring) happens — a row laid out on its own fills and aligns leading
    /// either way, and would pass even while the popover was visibly staggered.
    #[test]
    fn a_short_row_stays_flush_left_with_a_long_one() {
        let long = "/home/cyril/Nextcloud/Documents/Livres/Faux-Semblants/\
                    deeply/nested/Faux-semblants.skrib";
        let menu = MenuList::new()
            .max_visible_items(12)
            .item(row(
                false,
                false,
                "Long".to_string(),
                long.to_string(),
                None,
                |_| {},
            ))
            .item(row(
                false,
                false,
                "Short".to_string(),
                "/tmp/n.skrib".to_string(),
                None,
                |_| {},
            ));

        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(menu));
        tree.layout(SizeProposal {
            width: None,
            height: None,
        });

        // Each row's leading edge is its 16x16 marker column (empty when the row
        // is unchecked). Both must land at the same x.
        let mut ids = Vec::new();
        collect(&tree, id, &mut ids);
        let markers: Vec<f32> = ids
            .iter()
            .map(|i| tree.bounds(*i))
            .filter(|b| (b.width - 16.0).abs() < 0.5 && (b.height - 16.0).abs() < 0.5)
            .map(|b| b.x)
            .collect();

        assert_eq!(markers.len(), 2, "expected one marker per row");
        assert!(
            (markers[0] - markers[1]).abs() < 0.5,
            "both rows must start at the same x — got {:?}. A centred short row \
             is staggered to the right of the long one.",
            markers
        );
    }

    fn collect(tree: &WidgetTree, id: WidgetId, out: &mut Vec<WidgetId>) {
        for child in tree.children(id) {
            out.push(child);
            collect(tree, child, out);
        }
    }

    /// Measure the whole `MenuList` the popover actually shows — the row is only
    /// half the story, and the row alone measured fine even when the popover was
    /// visibly broken.
    #[test]
    fn menu_list_of_rows_reports_its_content_width() {
        let long = "/home/cyril/Nextcloud/Documents/Livres/Faux-Semblants/\
                    deeply/nested/Faux-semblants.skrib";
        let mut menu = MenuList::new().max_visible_items(12);
        menu = menu.header(section(lit!("Currently open".to_string())));
        for i in 0..3 {
            menu = menu.item(row(
                i == 0,
                i == 0,
                format!("Project {i}"),
                long.to_string(),
                None,
                |_| {},
            ));
        }
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(menu));
        tree.layout(SizeProposal {
            width: None,
            height: None,
        });
        let b = tree.bounds(id);
        eprintln!("NATURAL MENULIST WIDTH (3 items, no scroll) = {}", b.width);
        assert!(
            b.width >= ROW_TEXT_MAX_WIDTH,
            "the MenuList must be at least as wide as its rows' text column, \
             got {} (rows hug to {}). If it is narrower, the popover clips its \
             own content and the user sees a middle slice of each path.",
            b.width,
            ROW_TEXT_MAX_WIDTH
        );
    }

    /// The popover scrolls once it exceeds `max_visible_items`. Does engaging the
    /// scroll viewport change how the menu measures its width?
    ///
    /// This is the difference between "worked before" and "broken now": recents
    /// grew past 12 entries, so the scrollbar appeared.
    #[test]
    fn a_scrolling_menu_still_reports_its_content_width() {
        let long = "/home/cyril/Nextcloud/Documents/Livres/Faux-Semblants/\
                    deeply/nested/Faux-semblants.skrib";
        let mut menu = MenuList::new().max_visible_items(12);
        // 30 items — well past the 12-item scroll threshold.
        for i in 0..30 {
            menu = menu.item(row(
                false,
                false,
                format!("P{i}"),
                long.to_string(),
                None,
                |_| {},
            ));
        }
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(menu));
        tree.layout(SizeProposal {
            width: None,
            height: None,
        });
        let b = tree.bounds(id);
        eprintln!("SCROLLING MENULIST WIDTH (30 items) = {}", b.width);
        assert!(
            b.width >= ROW_TEXT_MAX_WIDTH,
            "a scrolling menu collapsed to {} (rows hug to {}) — the scroll \
             viewport is squeezing the content width",
            b.width,
            ROW_TEXT_MAX_WIDTH
        );
    }
}
