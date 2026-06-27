//! Recent-projects picker for the title bar.
//!
//! A flat dropdown button whose main slot shows the currently-open project
//! (the most-recently-opened `RecentWork`) plus a trailing chevron, and whose
//! popover lists the recent projects as rich rows — title, absolute path, and
//! last-opened date — each re-opening that project on click. The current
//! project is marked with a leading checkmark and an accent title.
//!
//! `SplitButton` is deliberately *not* used here: its dropdown reuses
//! `MenuItem` verbatim (single label only), whereas we need three fields per
//! row. The "free" `MenuList::item(impl Widget)` form lets a row be an
//! arbitrary widget tree, so we wrap it in a `PopoverButton` to reproduce the
//! split-button affordance with rich rows.
//!
//! Recent works come from the Qleany backend (`recent_work_commands`), not a
//! Bastyde `MruList`. The popover content is built once per `build()`, so the
//! list is refreshed by rebuilding the whole widget: it subscribes once to
//! `WorkManagementEvent::LoadWork` and bumps a `version` signal bound at
//! `BindingLevel::Rebuild`.

use std::rc::Rc;

use bastyde::core::BindingLevel;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, FixedSize, FocusScope, HStack, IconWidget, MaxSize, MenuList, Padding,
    PopoverButton, TextWidget, Toast, TraversalScopePolicy, VStack,
};

use frontend::AppContext;
use frontend::commands::{recent_work_commands, work_management_commands};
use frontend::common::event::{Event, Origin, WorkManagementEvent};
use frontend::work_management::LoadWorkDto;

/// Flat dropdown button listing recent projects; the main slot is the current
/// project.
pub struct RecentProjectsButton {
    app_ctx: Rc<AppContext>,
    /// Bumped on every `LoadWork`; bound at `BindingLevel::Rebuild` so the
    /// recent list + current item re-derive after a project loads.
    version: Signal<u64>,
    /// Guards against re-subscribing on every rebuild (subscriptions don't
    /// auto-clean across builds).
    subscribed: bool,
    root_child: Option<WidgetId>,
}

impl RecentProjectsButton {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        Self {
            app_ctx,
            version: Signal::new(0),
            subscribed: false,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for RecentProjectsButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RecentProjectsButton").finish()
    }
}

impl Widget for RecentProjectsButton {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Subscribe exactly once; each project load refreshes the list.
        if !self.subscribed {
            self.subscribed = true;
            let version = self.version.clone();
            ctx.subscribe_event(
                Origin::WorkManagement(WorkManagementEvent::LoadWork),
                move |_event: &Event| version.set(version.get().wrapping_add(1)),
            );
        }
        self.version
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        // Most-recently-opened first; the head is the "current" project.
        let mut recents = recent_work_commands::get_all_recent_work(&self.app_ctx)
            .unwrap_or_default();
        recents.sort_by(|a, b| b.last_opened_at.cmp(&a.last_opened_at));

        let current_title = recents
            .first()
            .map(|r| r.title.clone())
            .unwrap_or_else(|| "No project".to_string());

        // Popover content: a MenuList of rich rows.
        let mut menu = MenuList::new().max_visible_items(10);
        if recents.is_empty() {
            menu = menu.item(
                Padding::symmetric(8.0, 12.0).child(
                    TextWidget::new(lit!("No recent projects"))
                        .style(TextStyleRole::Body)
                        .color(TextRole::Secondary),
                ),
            );
        } else {
            for (idx, dto) in recents.iter().enumerate() {
                let is_current = idx == 0;
                let path = dto.absolute_path.clone();
                let app_ctx = self.app_ctx.clone();
                let date = dto.last_opened_at.format("%Y-%m-%d %H:%M").to_string();

                // 16px leading column — checkmark on the current project.
                let mut marker = FixedSize::new().bind_width(16.0).bind_height(16.0);
                if is_current {
                    marker = marker.child(IconWidget::checkmark(14.0));
                }

                let title_color = if is_current {
                    TextRole::Accent
                } else {
                    TextRole::Primary
                };

                let body = VStack::new()
                    .spacing(2.0)
                    .child(
                        TextWidget::new(lit!(dto.title.clone()))
                            .style(TextStyleRole::BodyBold)
                            .color(title_color),
                    )
                    .child(
                        TextWidget::new(lit!(path.clone()))
                            .style(TextStyleRole::Small)
                            .color(TextRole::Secondary)
                            .single_line()
                            .overflow(TextOverflow::Ellipsis(EllipsisMode::Middle)),
                    )
                    .child(
                        TextWidget::new(lit!(date))
                            .style(TextStyleRole::Small)
                            .color(TextRole::Secondary),
                    );

                let row = HStack::new()
                    .spacing(8.0)
                    .child(marker)
                    .child(MaxSize::width(360.0).child(body))
                    .cursor(CursorIcon::Pointer)
                    .focusable(true)
                    .on_tap(move |_event, ctx| {
                        // Dismiss the popover, then (re)open the project.
                        ctx.dismiss_self_overlay_chain();
                        if let Err(e) = work_management_commands::load_work(
                            &app_ctx,
                            &LoadWorkDto { file_name: path.clone() },
                        ) {
                            ctx.show_toast(Toast::error(lit!(format!(
                                "Could not open project: {e}"
                            ))));
                        }
                    });

                menu = menu.item(Padding::symmetric(6.0, 10.0).child(row));
            }
        }

        let trigger = Button::new(lit!(current_title))
            .variant(ButtonVariant::Ghost)
            .text_style(TextStyleRole::BodyBold)
            .trailing(IconWidget::chevron_down(12.0));

        // Trap Tab inside the popover: it is an anchored (not centered)
        // overlay, so it isn't auto-confined — a Cycle scope keeps keyboard
        // navigation on the recent-project rows until the popover dismisses.
        let root = ctx.add(
            PopoverButton::new(trigger)
                .show_disclosure_caret(false)
                .content(FocusScope::new(TraversalScopePolicy::Cycle).child(menu)),
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