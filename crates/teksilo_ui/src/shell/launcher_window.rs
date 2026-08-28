// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Launcher window — Welcome UI as a real top-level window.

use std::rc::Rc;

use teksilo::prelude::*;
use teksilo::widgets::{
    Center, CollapsePolicy, Expand, HStack, IconButtonSize, MenuBar, NativeMenuMode, Padding,
    TextWidget, TitleBar, VStack, WindowFrame,
};

use frontend::AppContext;

use crate::shell::window_ids::LAUNCHER_WINDOW_ID;
use crate::welcome::panel::WelcomePanel;

/// The Launcher window: the Welcome UI hosted as a real top-level window
/// (not a modal), reusing [`WelcomePanel`]/`WelcomeViewModel` verbatim — no
/// duplicated recents/examples UI.
///
/// Skribisto's windows are all `DecorationsMode::CustomChrome` — the OS draws
/// no title bar at all, so `TitleBar` + `WindowFrame` *are* the window chrome
/// (a `Native`-decorated window would be the odd one out: no consistent
/// drag/resize/traffic-light behaviour with the rest of the app). "Appropriate
/// chrome" for the launcher means a **leaner** `TitleBar` — the drag region, the
/// brand mark and hamburger, the window title, and the window controls
/// (min/max/close) — with no `ProjectSwitcherButton`, no spell-check toggle and
/// no Export control (all three assume an open project). Mirrors
/// `ProjectWindowFactory::window_config`'s `title_bar`/`WindowFrame` composition,
/// menu bar included: same `leading` slot, same collapse policy, same
/// suppress-on-macOS flag — over a much shorter model
/// ([`crate::shell::launcher_menu`]).
///
/// No close guard: the Launcher holds no unsaved state, so closing it (its
/// own title-bar close button, Alt+F4, or the panel's inline close button)
/// always succeeds — and since it is Skribisto's other-than-a-project window,
/// closing it while it is the only window quits the process. That is the
/// intended "close the launcher to exit" behaviour.
pub fn launcher_window_config(app_ctx: Rc<AppContext>) -> WindowConfig {
    // The Launcher is deliberately not resizable (min == max): `WelcomePanel`
    // fills it edge to edge, so this is the one place its proportions — the
    // 264 dp sidebar against the recents list — are decided.
    const W: u32 = 820;
    const H: u32 = 590;
    WindowConfig::new()
        .id(LAUNCHER_WINDOW_ID)
        // The OS-level title (taskbar, alt-tab, window list) stays the app's
        // name — that identifies the *process*. The visible custom title bar
        // below says "Welcome to Skribisto": that names the *screen*, and it
        // is the reason the Welcome content no longer carries a title strip of
        // its own.
        .title(crate::identity::display_name())
        .size(W, H)
        .min_size(W, H)
        .max_size(W, H)
        .decorations(DecorationsMode::CustomChrome)
        // Consume an xdg-activation startup token (set by the desktop, or by
        // another instance's "open in new window") so a bare launch comes up
        // focused on Wayland — same rationale as the project window.
        .activate_from_env(true)
        .root(move |tree, _state| {
            let theme = tree.theme().clone();
            // Leaner title bar: brand icon + hamburger in the leading slot, the
            // window title centred, drag region, and the platform's window
            // controls — no project switcher. Falls back to a plain label on any
            // platform whose host is unavailable (mirrors the project window's
            // fallback).
            let title_bar = match tree.title_bar_host() {
                Some(host) => {
                    // Leading inset: the icon is the first thing in the title
                    // bar's leading slot, which starts at the window's left edge
                    // — bare, it sits flush against it. The project window uses
                    // the same padding on its brand icon (which likewise leads
                    // its hamburger).
                    let brand_icon = Padding::new(0.0, 0.0, 0.0, 8.0)
                        .child(crate::identity::brand_mark().widget(25.0));
                    // Same three settings as the project window's bar, for the
                    // same three reasons: `Toolbar` (30 dp) because the strip is
                    // `TITLE_BAR_HEIGHT` tall and does not grow for an oversized
                    // child; `Always` because this app's menu is a hamburger, not
                    // a strip of labels; `Suppress` because on macOS the same
                    // `MenuModel` is mirrored into the global bar at the top of
                    // the screen (`install_native_menu` in `lib.rs`) and a second
                    // copy in-window, where no Mac user looks, is all `Coexist`
                    // would buy. The flag is inert off macOS.
                    let menubar = MenuBar::from_model(super::launcher_menu::build_launcher_menu())
                        .native_on_macos(NativeMenuMode::Suppress)
                        .collapse_policy(CollapsePolicy::Always)
                        .hamburger_size(IconButtonSize::Toolbar);
                    // The menu bar goes in `leading`, never in `center`: the
                    // centre slot lives inside the TitleBar's DragRegion, which is
                    // published to the OS as the window caption, and the OS owns
                    // caption pixels outright — a hamburger there would drag the
                    // window instead of opening. (The project window carves its
                    // switcher back out with a `DeadZone` for exactly that reason;
                    // the leading slot needs no such rescue.)
                    let leading = teksu!(
                        HStack {
                            spacing: 5.0
                            alignment: teksilo::tokens::VAlignment::Center
                            child: brand_icon
                            child: menubar
                        }
                    );
                    tree.add_boxed(Box::new(teksu!(
                    TitleBar::new(host) {
                        height: super::TITLE_BAR_HEIGHT
                        background: SurfaceRole::Main
                        leading: leading
                        center: Expand::horizontal {
                            Center {
                                // The window's title bar names the screen
                                // ("Welcome to Skribisto"), so the Welcome
                                // content below needs no title strip of its
                                // own — the two together were a window
                                // inside a window. Project windows keep the
                                // bare app name here.
                                TextWidget::new(tr!(welcome_title())) {
                                    style: theme.typography.body_bold.clone()
                                    color: TextRole::Primary
                                }
                            }
                        }
                        close_action: |ctx| ctx.close_window()
                    }
                    )))
                }
                None => tree.add(TextWidget::new(tr!(welcome_title()))),
            };
            // No `Center`: the Welcome content fills the window (see
            // `welcome_panel`'s module docs) — centring a fixed-size card in
            // here is what put a gutter down each side of it.
            let body = tree.add(Expand::new().child(WelcomePanel::new(app_ctx.clone())));
            let inner = tree.add(
                VStack::new()
                    .spacing(0.0)
                    .add_child(title_bar)
                    .add_child(body),
            );

            // Edge resize handles, same as the project window (skipped where
            // the host doesn't need them, e.g. macOS).
            match tree.title_bar_host() {
                Some(host) if host.needs_custom_resize_handles() => {
                    tree.add(WindowFrame::new(host).thickness(6.0).content_id(inner))
                }
                _ => inner,
            }
        })
}
