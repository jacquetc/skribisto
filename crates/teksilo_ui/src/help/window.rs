// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Help window: one per process, opened or focused through one door.
//!
//! ## Why a window and not a modal
//!
//! Settings is a modal because you go there, change something and come back. Help is
//! the opposite: the whole reason to open it is to do something *else* while reading
//! it. A modal makes the app unusable for exactly as long as the reader needs the
//! answer, which is the one thing help must not do.
//!
//! ## One door
//!
//! [`open_or_focus_help`] is the only way in. Every entry point (F1, the Help menu, the
//! Learn pane in the Launcher, and a future context-sensitive affordance) calls it, so
//! "is Help already open" is answered in one place, by the window's own string id,
//! rather than by a side table that can disagree with the window list. Same rule the
//! project windows follow in [`crate::shell::window_ids`].

use std::rc::Rc;

use teksilo::core::window::{DecorationsMode, WindowConfig};
use teksilo::prelude::*;
use teksilo::widgets::{Center, Expand, TextWidget, TitleBar, VStack, WindowFrame};

use super::help_vm::HelpViewModel;
use super::panel::HelpPanel;

/// The Help window's stable id. Persisted geometry keys on it, so it must never change.
pub const HELP_WINDOW_ID: &str = "help";

const W: u32 = 940;
const H: u32 = 680;
const MIN_W: u32 = 620;
const MIN_H: u32 = 420;

/// Show `topic_key` in the Help window, opening it if it is not already up.
///
/// Passing [`None`] opens whatever the window was last showing (or the default topic on
/// a first open), which is what a bare "Help" command should do: a reader returning to
/// a window they left open has not asked to lose their place.
pub fn open_or_focus_help(ctx: &mut EventContext, topic_key: Option<&str>) {
    let vm = ctx
        .app_state::<HelpViewModel>()
        .cloned()
        .unwrap_or_else(|| {
            // `app_state` is seeded once in `run()`. Falling back to a fresh view-model
            // rather than panicking keeps a missing registration from taking the app
            // down; the window still works, it just does not share its place with
            // another entry point.
            eprintln!("skribisto: no HelpViewModel in app_state; using a detached one");
            HelpViewModel::new()
        });

    if let Some(key) = topic_key {
        vm.open_fresh(key);
    }

    match ctx.find_window(HELP_WINDOW_ID) {
        Some(id) => ctx.focus_window(id),
        None => {
            ctx.open_window(help_window_config(vm));
        }
    }
}

/// The Help window's configuration.
///
/// Resizable, unlike the Launcher: a topic is prose, and how wide prose should be is
/// the reader's call, not this window's.
///
/// ## The chrome is not optional
///
/// `DecorationsMode::CustomChrome` means the OS draws no frame at all, so everything a
/// window needs it has to carry itself. This shipped without any of it: no title, no
/// drag region, and **no minimize, maximize or close button** — the window could only be
/// dismissed through the compositor. `first_run_window` is the closer model than the
/// Launcher (no brand icon, no menu, no project switcher; a centred title and the
/// controls).
///
/// [`WindowFrame`] matters more here than in either of those, because Help is the only
/// one of the three that resizes: without the edge strips it installs, a reader on
/// Wayland has a resizable window with no edge to drag.
pub fn help_window_config(vm: HelpViewModel) -> WindowConfig {
    let vm = Rc::new(vm);
    WindowConfig::new()
        .id(HELP_WINDOW_ID)
        // The OS-level identity (taskbar, alt-tab) stays the application's name; the
        // *visible* title below names the screen, which is why `help-window-title`
        // exists and why it was unused until this window grew a title bar to put it in.
        .title(crate::identity::display_name())
        // Which application this window belongs to, as far as the desktop is
        // concerned. Every window in the process sends the same one. See
        // `identity::desktop_id`.
        .app_id(crate::identity::desktop_id())
        .size(W, H)
        .min_size(MIN_W, MIN_H)
        .decorations(DecorationsMode::CustomChrome)
        .activate_from_env(true)
        .root(move |tree, _state| {
            let theme = tree.theme().clone();
            let title_bar = match tree.title_bar_host() {
                Some(host) => tree.add_boxed(Box::new(teksu!(
                    TitleBar::new(host) {
                        height: crate::shell::TITLE_BAR_HEIGHT
                        background: SurfaceRole::Main
                        center: Expand::horizontal {
                            Center {
                                TextWidget::new(tr!(help_window_title())) {
                                    style: theme.typography.body_bold.clone()
                                    color: TextRole::Primary
                                }
                            }
                        }
                        close_action: |ctx| ctx.close_window()
                    }
                ))),
                // Same fallback the other two custom-chrome windows take: a plain label,
                // so a platform with no title-bar host still says what this window is.
                None => tree.add(TextWidget::new(tr!(help_window_title()))),
            };
            let body = tree.add(Expand::new().child(HelpPanel::new((*vm).clone())));
            let inner = tree.add(
                VStack::new()
                    .spacing(0.0)
                    .add_child(title_bar)
                    .add_child(body),
            );
            match tree.title_bar_host() {
                Some(host) if host.needs_custom_resize_handles() => {
                    tree.add(WindowFrame::new(host).thickness(6.0).content_id(inner))
                }
                _ => inner,
            }
        })
}
