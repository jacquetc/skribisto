// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Report a Translation Error window: one per process, opened or focused through
//! one door.
//!
//! A window rather than a modal for the reason Help is one: the writer reporting a
//! string usually wants to go and look at it again (open the menu, hover the
//! tooltip) and come back to the report, which a modal would forbid.
//!
//! [`open_or_focus_translation_report`] is the only way in. The Help menu, the
//! command palette, the Launcher's Learn pane, Settings ▸ Appearance and the Help
//! window's footer all call it, so "is the report already open" is answered by the
//! window's own string id, as for every other window.

use std::rc::Rc;

use teksilo::core::window::{DecorationsMode, WindowConfig};
use teksilo::prelude::*;
use teksilo::widgets::{Center, Expand, TextWidget, TitleBar, VStack, WindowFrame};

use super::panel::TranslationReportPanel;
use super::translation_report_vm::{Scope, TranslationReportViewModel};

/// The window's stable id. Persisted geometry keys on it, so it must never change.
pub const TRANSLATION_REPORT_WINDOW_ID: &str = "translation-report";

const W: u32 = 980;
const H: u32 = 720;
const MIN_W: u32 = 760;
const MIN_H: u32 = 540;

/// Show the report window, opening it if it is not already up.
///
/// `scope` narrows the search to one help page, from the Help window's footer;
/// `None` keeps whatever the window holds, so a writer who left a report half
/// written finds it as they left it.
pub fn open_or_focus_translation_report(ctx: &mut EventContext, scope: Option<Scope>) {
    let vm = ctx
        .app_state::<TranslationReportViewModel>()
        .cloned()
        .unwrap_or_else(|| {
            // `app_state` is seeded once in `run()`, beside the Help window's view-model.
            // A missing registration degrades to a window that does not keep its state
            // between openings, rather than to no window.
            eprintln!(
                "skribisto: no TranslationReportViewModel in app_state; using a detached one"
            );
            TranslationReportViewModel::new()
        });
    if scope.is_some() {
        vm.open_for(scope);
    }
    match ctx.find_window(TRANSLATION_REPORT_WINDOW_ID) {
        Some(id) => ctx.focus_window(id),
        None => {
            ctx.open_window(translation_report_window_config(vm));
        }
    }
}

/// The window's configuration: custom chrome with a title, the window controls and,
/// where the platform needs them, resize edges, as the Help window has.
pub fn translation_report_window_config(vm: TranslationReportViewModel) -> WindowConfig {
    let vm = Rc::new(vm);
    WindowConfig::new()
        .id(TRANSLATION_REPORT_WINDOW_ID)
        .title(
            tr!(window_title(
                title = tr!(translation_report_window_title()).resolve_now(),
                app = crate::identity::display_name()
            ))
            .resolve_now(),
        )
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
                                TextWidget::new(tr!(translation_report_window_title())) {
                                    style: theme.typography.body_bold.clone()
                                    color: TextRole::Primary
                                }
                            }
                        }
                        close_action: |ctx| ctx.close_window()
                    }
                ))),
                None => tree.add(TextWidget::new(tr!(translation_report_window_title()))),
            };
            let body = tree.add(Expand::new().child(TranslationReportPanel::new((*vm).clone())));
            let inner = tree.add(VStack::new().spacing(0.0).child(title_bar).child(body));
            match tree.title_bar_host() {
                Some(host) if host.needs_custom_resize_handles() => {
                    tree.add(WindowFrame::new(host).thickness(6.0).content(inner))
                }
                _ => inner,
            }
        })
}
