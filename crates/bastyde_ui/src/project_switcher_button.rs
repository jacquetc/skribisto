//! Project switcher for the title bar.
//!
//! A flat dropdown button whose main slot shows the **currently-open project's**
//! title (or a "No work loaded" fallback when nothing is open) plus a trailing
//! chevron, and whose popover lists the recent projects as rich rows — title,
//! absolute path, and last-opened date — each re-opening that project on click.
//! The row that matches the open project is marked with a leading checkmark and
//! an accent title.
//!
//! **"Open" is read from live state, never from list position.** The open
//! project's identity comes from [`SingleWorkInfo::file_name`] (its on-disk path)
//! and [`SingleWork::title`]; the recents list ([`RecentWorkListModel`], Layer A
//! over a persisted `MruList`) is *only* the history source. Marking a row
//! current is a path comparison against the open path — so at cold start (no work
//! loaded) the button reads "No work loaded" and no row is checked, even though
//! the persisted MRU is non-empty.
//!
//! `SplitButton` is deliberately *not* used here: its dropdown reuses `MenuItem`
//! verbatim (single label only), whereas we need three fields per row. The "free"
//! `MenuList::item(impl Widget)` form lets a row be an arbitrary widget tree, so
//! we wrap it in a `PopoverButton` to reproduce the split-button affordance with
//! rich rows.
//!
//! The popover content is built once per `build()`; the widget re-derives the
//! list, current title, and checkmark whenever any of three signals bump at
//! `BindingLevel::Rebuild`: the recents `version` (list changed), the open Work's
//! `title` (load/new/close), and the open `WorkInfo`'s `file_name` (load/close/
//! Save As).
//!
//! (Later phases will split the popover into "Currently open" vs "Recent (not
//! open)" sections and route clicks through new-window / switch-focus flows; this
//! file currently ships the corrected single-list behaviour only.)

use std::path::Path;
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
use crate::singles::{SingleWork, SingleWorkInfo};

/// Flat dropdown button whose main slot is the current project; its popover lists
/// recent projects.
pub struct ProjectSwitcherButton {
    app_ctx: Rc<AppContext>,
    /// Reactive recent-projects history (Layer A); its `version` signal is bound
    /// at `BindingLevel::Rebuild` so the list re-derives on load.
    model: RecentWorkListModel,
    root_child: Option<WidgetId>,
}

impl ProjectSwitcherButton {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        Self {
            model: RecentWorkListModel::new(app_ctx.clone()),
            app_ctx,
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
        // Live "what's open" state — the id-driven singles registered in `main`.
        // These, not the MRU head, define the current project.
        let single_work = ctx
            .app_state::<SingleWork>()
            .cloned()
            .expect("SingleWork registered in main");
        let single_work_info = ctx
            .app_state::<SingleWorkInfo>()
            .cloned()
            .expect("SingleWorkInfo registered in main");

        // Rebuild when: the recents history changes, the open Work's title changes
        // (load/new/close all move it), or the open WorkInfo's file_name changes
        // (load/close/Save As). The first keeps the list fresh; the latter two keep
        // the trigger label + checkmark honest about what's actually open.
        self.model.wire(ctx);
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.model
            .version_signal()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        single_work
            .title()
            .bind_to(sid, reg, BindingLevel::Rebuild);
        single_work_info
            .file_name()
            .bind_to(sid, reg, BindingLevel::Rebuild);

        // Most-recently-opened first — the history, *not* a stand-in for "open".
        let recents = self.model.items();

        // The open project's on-disk path (None ⇒ nothing loaded) and title.
        let open_path = single_work_info.file_name().get();
        let work_title = single_work.title().get();

        // A project is open iff we have a title or a path for it. The trigger label
        // prefers the matching recents row's title (so it matches the checkmarked
        // row), then the live Work title, then the file stem.
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
            for dto in recents.iter() {
                // Current ⇔ this row's path is the open project's path — never a
                // list-position guess.
                let is_current = open_path.as_deref() == Some(dto.absolute_path.as_str());
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
