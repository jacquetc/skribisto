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

use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;

use bastyde::core::BindingLevel;
use bastyde::prelude::*;
use bastyde::widgets::{
    Button, ButtonVariant, FixedSize, FocusScope, GroupHeader, HStack, IconWidget, MaxSize,
    MenuList, MessageBox, MessageBoxButton, MessageBoxButtons, Padding, PopoverButton,
    StandardButton, TextWidget, Toast, TraversalScopePolicy, VStack,
};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::work_management::LoadWorkDto;

use crate::models::RecentWorkListModel;
use crate::open_registry::{self, OpenEntry};
use crate::singles::{SingleWork, SingleWorkInfo};

/// Best-effort canonical form for comparing project paths across the registry
/// (which stores canonical paths) and the recents list.
fn canon(path: &str) -> String {
    std::fs::canonicalize(path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string())
}

/// Launch a fresh Skribisto process to open `path`, forwarding an activation
/// `token` (so the new window comes up focused on Wayland via the main window's
/// `activate_from_env`).
fn spawn_new_process(path: &str, token: Option<String>) {
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    let mut cmd = std::process::Command::new(exe);
    cmd.arg(path);
    if let Some(tok) = token {
        cmd.env("XDG_ACTIVATION_TOKEN", &tok);
        cmd.env("DESKTOP_STARTUP_ID", &tok);
    }
    let _ = cmd.spawn();
}

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
    let mut body = VStack::new()
        .spacing(2.0)
        .child(
            TextWidget::new(lit!(title))
                .style(TextStyleRole::BodyBold)
                .color(title_color),
        )
        .child(
            TextWidget::new(lit!(path))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary)
                .single_line()
                .overflow(TextOverflow::Ellipsis(EllipsisMode::Middle)),
        );
    if let Some(date) = date {
        body = body.child(
            TextWidget::new(lit!(date))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        );
    }
    let inner = HStack::new()
        .spacing(8.0)
        .child(marker)
        .child(MaxSize::width(360.0).child(body))
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
    app_ctx: Rc<AppContext>,
    model: RecentWorkListModel,
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
        let single_work = ctx
            .app_state::<SingleWork>()
            .cloned()
            .expect("SingleWork registered in main");
        let single_work_info = ctx
            .app_state::<SingleWorkInfo>()
            .cloned()
            .expect("SingleWorkInfo registered in main");

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

        let open_entries: Vec<OpenEntry> = open_registry::scan();
        let my_pid = open_registry::my_pid();
        let open_paths: HashSet<String> = open_entries.iter().map(|e| e.path.clone()).collect();
        let recent_not_open: Vec<_> = recents
            .iter()
            .filter(|r| !open_paths.contains(&canon(&r.absolute_path)))
            .collect();

        let has_open = !open_entries.is_empty();
        let has_recent = !recent_not_open.is_empty();

        let mut menu = MenuList::new().max_visible_items(12);

        if !has_open && !has_recent {
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
                for entry in &open_entries {
                    let is_self = entry.pid == my_pid;
                    let title = entry.title.clone();
                    let path = entry.path.clone();
                    let pid = entry.pid;
                    menu = menu.item(row(is_self, is_self, title, path, None, move |ctx| {
                        ctx.dismiss_self_overlay_chain();
                        if is_self {
                            return; // already this window
                        }
                        // Mint a token from this (focused) window, then ask the
                        // owning instance to raise itself with it.
                        ctx.request_activation_token_self(Box::new(move |tok| {
                            let _ = crate::ipc::send_raise(pid, tok);
                        }));
                    }));
                }
            }

            if has_open && has_recent {
                menu = menu.separator();
            }

            if has_recent {
                menu = menu.header(section(tr!(switcher_recent_section())));
                for dto in &recent_not_open {
                    let title = dto.title.clone();
                    let path = dto.absolute_path.clone();
                    let date = dto.last_opened_at.format("%Y-%m-%d %H:%M").to_string();
                    let app_ctx = self.app_ctx.clone();
                    menu = menu.item(row(false, false, title.clone(), path.clone(), Some(date), move |ctx| {
                        ctx.dismiss_self_overlay_chain();
                        let for_new = path.clone();
                        let for_here = path.clone();
                        let app_ctx = app_ctx.clone();
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
                                    let p = for_new.clone();
                                    ctx.request_activation_token_self(Box::new(move |tok| {
                                        spawn_new_process(&p, tok);
                                    }));
                                }
                                StandardButton::Yes => {
                                    if let Err(e) = work_management_commands::load_work(
                                        &app_ctx,
                                        &LoadWorkDto {
                                            file_name: for_here.clone(),
                                        },
                                    ) {
                                        ctx.show_toast(Toast::error(tr!(could_not_open_work(
                                            error = e.to_string()
                                        ))));
                                    }
                                }
                                _ => {}
                            })
                            .present(ctx);
                    }));
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
    /// Bumped by the popover's `on_open`; consumed by [`OpenProjectsMenu`] to
    /// re-scan the registry each time the popover shows. Owned here so it survives
    /// this widget's rebuilds.
    open_epoch: Signal<u64>,
    root_child: Option<WidgetId>,
}

impl ProjectSwitcherButton {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        Self {
            model: RecentWorkListModel::new(app_ctx.clone()),
            app_ctx,
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
        let single_work = ctx
            .app_state::<SingleWork>()
            .cloned()
            .expect("SingleWork registered in main");
        let single_work_info = ctx
            .app_state::<SingleWorkInfo>()
            .cloned()
            .expect("SingleWorkInfo registered in main");

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
            app_ctx: self.app_ctx.clone(),
            model: RecentWorkListModel::new(self.app_ctx.clone()),
            open_epoch: self.open_epoch.clone(),
            root: None,
        };

        let open_epoch = self.open_epoch.clone();
        let root = ctx.add(
            PopoverButton::new(trigger)
                .show_disclosure_caret(false)
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
