// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The first-run window — the settings-import offer, hosted as a real window.
//!
//! Shaped like [`super::launcher_window`]: custom chrome, fixed size, no menu.
//! It is the *first* window an edition ever shows, so it comes before the
//! Launcher and before any project window.
//!
//! **Ordering invariant, same as everywhere else in this module:** the process
//! quits when its last window closes, so `on_answer` opens the real initial
//! window *before* closing this one. Backwards, the first launch of an edition
//! ends by exiting.

use teksilo::prelude::*;
use teksilo::settings::AppPaths;
use teksilo::widgets::{Center, Expand, Padding, TextWidget, TitleBar, VStack, WindowFrame};

use crate::panels::first_run::{FirstRunPanel, OnAnswer};

pub const FIRST_RUN_WINDOW_ID: &str = "first-run";

/// The import offer as a top-level window.
///
/// `on_answer(ctx, import)` is handed the writer's choice; it is responsible for
/// performing the copy, recording the answer, opening the real initial window and
/// closing this one — in that order.
pub fn first_run_window_config(source: AppPaths, on_answer: OnAnswer) -> WindowConfig {
    const W: u32 = 560;
    const H: u32 = 380;
    WindowConfig::new()
        .id(FIRST_RUN_WINDOW_ID)
        .title(crate::identity::display_name())
        .size(W, H)
        .min_size(W, H)
        .max_size(W, H)
        .decorations(DecorationsMode::CustomChrome)
        .activate_from_env(true)
        .root(move |tree, _state| {
            let theme = tree.theme().clone();
            let title_bar = match tree.title_bar_host() {
                Some(host) => tree.add_boxed(Box::new(teksu!(
                    TitleBar::new(host) {
                        height: super::TITLE_BAR_HEIGHT
                        background: SurfaceRole::Main
                        center: Expand::horizontal {
                            Center {
                                TextWidget::new(tr!(first_run_window_title())) {
                                    style: theme.typography.body_bold.clone()
                                    color: TextRole::Primary
                                }
                            }
                        }
                        close_action: |ctx| ctx.close_window()
                    }
                ))),
                None => tree.add(TextWidget::new(tr!(first_run_window_title()))),
            };
            let body = tree.add(Expand::new().child(
                Padding::uniform(16.0).child(FirstRunPanel::new(source.clone(), on_answer.clone())),
            ));
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

/// Perform the writer's choice, then hand off to `open_initial`.
///
/// Extracted from the window so the sequence — copy, record, open, close — is
/// testable and stated once. `open_initial` is whatever the launch *would* have
/// done had there been no offer.
pub fn answer(
    ctx: &mut EventContext,
    source: &AppPaths,
    import: bool,
    open_initial: &dyn Fn(&mut EventContext),
) {
    if import {
        match crate::first_run::import(source) {
            Ok(report) => eprintln!(
                "skribisto: imported {} settings file(s) ({} bytes) from {}",
                report.files,
                report.bytes,
                source.config_dir().display()
            ),
            Err(e) => {
                // Reported, never fatal: a failed import must still let the
                // writer into the application, with defaults.
                eprintln!(
                    "skribisto: could not import settings from {}: {e}",
                    source.config_dir().display()
                );
                ctx.show_toast(Toast::error(tr!(first_run_import_failed(
                    error = e.to_string()
                ))));
            }
        }
    }
    if let Some(ours) = crate::identity::app_paths() {
        crate::first_run::mark_answered(&ours, import);
    }
    // Open before closing — the process quits with its last window.
    open_initial(ctx);
    ctx.close_window();
}
