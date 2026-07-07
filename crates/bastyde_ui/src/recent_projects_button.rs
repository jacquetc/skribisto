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
//! Recent works come from the reactive
//! [`RecentWorkListModel`](crate::models::RecentWorkListModel) (Layer A), which is
//! backed by a **persisted** Bastyde `MruList` (so recents survive restarts;
//! unreachable projects are hidden but kept). The popover content is built once
//! per `build()`, so the list is refreshed by rebuilding the whole widget: the
//! model self-subscribes to `LoadWork`/`NewWork` and bumps a `version` signal this
//! widget binds at `BindingLevel::Rebuild`.

use std::rc::Rc;

use bastyde::core::BindingLevel;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, FixedSize, FocusScope, HStack, IconWidget, MaxSize, MenuList, Padding,
    PopoverButton, TextWidget, Toast, TraversalScopePolicy, VStack,
};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::work_management::LoadWorkDto;

use crate::models::RecentWorkListModel;

/// Flat dropdown button listing recent projects; the main slot is the current
/// project.
pub struct RecentProjectsButton {
    app_ctx: Rc<AppContext>,
    /// Reactive recent-projects list (Layer A); its `version` signal is bound at
    /// `BindingLevel::Rebuild` so the list + current item re-derive on load.
    model: RecentWorkListModel,
    root_child: Option<WidgetId>,
}

impl RecentProjectsButton {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        Self {
            model: RecentWorkListModel::new(app_ctx.clone()),
            app_ctx,
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
        // The model subscribes once and bumps `version` on each load; bind it at
        // Rebuild level so this widget re-derives the list then.
        self.model.wire(ctx);
        self.model.version_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        // Most-recently-opened first; the head is the "current" project.
        let recents = self.model.items();

        // The current project's title (data), or a translated "No work" fallback.
        let current_title = recents.first().map(|r| r.title.clone());

        // Popover content: a MenuList of rich rows.
        let mut menu = MenuList::new().max_visible_items(10);
        if recents.is_empty() {
            menu = menu.item(
                Padding::symmetric(8.0, 12.0).child(
                    TextWidget::new(tr!(no_recent_works()))
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
                let mut marker = FixedSize::new().width(16.0).height(16.0);
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
                            &LoadWorkDto {
                                file_name: path.clone(),
                            },
                        ) {
                            ctx.show_toast(Toast::error(tr!(could_not_open_work(
                                error = e.to_string()
                            ))));
                        }
                    });

                menu = menu.item(Padding::symmetric(6.0, 10.0).child(row));
            }
        }

        let trigger = Button::new(match current_title {
            Some(t) => lit!(t),
            None => tr!(no_work()),
        })
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
