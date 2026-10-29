// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The one-time offer of the screen-reader guide.
//!
//! When Skribisto starts with a screen reader running, the first window to come up asks
//! whether to open *Help ▸ Using a screen reader*. Once per run, and never again once the
//! writer ticks "Do not show this again" ([`crate::SCREEN_READER_OFFER_KEY`]). The Help
//! menu row stays the way back to the page, so the checkbox cannot lock it away.
//!
//! The state is asked once, after the first window has mounted, through teksilo's
//! public `AccessibilityPreferences::query`: the platform's own flag, which NVDA, JAWS
//! and Narrator set on Windows (`SPI_GETSCREENREADER`), Orca on Linux (AT-SPI
//! `ScreenReaderEnabled`) and VoiceOver on macOS. A screen reader started later is not
//! noticed, which suits an offer made at startup. teksilo pushes the same state into each
//! window's tree but gives a build or an event no way to read it, so this asks for
//! itself, synchronously, as teksilo does when a window takes focus.

use std::sync::atomic::{AtomicBool, Ordering};

use teksilo::core::ScreenReaderState;
use teksilo::prelude::*;
use teksilo::widgets::{MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton};

/// Whether this run has already offered (or decided not to). Process-wide: a second
/// window, or the project window that replaces the Launcher, must not ask again.
static OFFERED: AtomicBool = AtomicBool::new(false);

/// Offer the guide from the window being built, if this run has not offered yet.
///
/// Call from the `build` of a window's root. The question waits until the window is
/// mounted, because a dialog needs a window to sit in.
pub fn offer_once(ctx: &mut BuildContext) {
    if OFFERED.swap(true, Ordering::Relaxed) {
        return;
    }
    ctx.run_after_mount(|c| {
        let enabled = c.settings().signal(crate::SCREEN_READER_OFFER_KEY, true);
        let state = teksilo::platform::AccessibilityPreferences::query().screen_reader;
        if should_offer(state, enabled.get()) {
            present(c, enabled);
        }
    });
}

/// Offer only to a screen reader the platform says is running, and only while the
/// writer has not asked never to be offered again. An `Unknown` state is not a yes:
/// asking a sighted writer about a screen reader they do not use is noise.
fn should_offer(state: ScreenReaderState, enabled: bool) -> bool {
    enabled && state == ScreenReaderState::Active
}

fn present(ctx: &mut EventContext, enabled: Signal<bool>) {
    MessageBox::information(tr!(screen_reader_offer_title()))
        .text(tr!(screen_reader_offer_text()))
        .buttons(MessageBoxButtons::Custom(vec![
            MessageBoxButton::standard(StandardButton::Ok).label(tr!(screen_reader_offer_open())),
            MessageBoxButton::standard(StandardButton::Close),
        ]))
        // Opening the guide is what a reader who hears this is most likely to want, and
        // it changes nothing: Enter is safe.
        .default_button(StandardButton::Ok)
        .escape_button(StandardButton::Close)
        .show_again_checkbox(tr!(screen_reader_offer_never()))
        .on_result(move |result, c| {
            if result.checkbox_checked {
                enabled.set(false);
            }
            if result.button == StandardButton::Ok {
                crate::help::window::open_or_focus_help(c, Some(crate::help::SCREEN_READER_TOPIC));
            }
        })
        .present(ctx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_running_screen_reader_is_offered_the_guide_and_only_until_declined() {
        assert!(should_offer(ScreenReaderState::Active, true));
        assert!(
            !should_offer(ScreenReaderState::Active, false),
            "the writer said not to show it again"
        );
        assert!(!should_offer(ScreenReaderState::Inactive, true));
        assert!(
            !should_offer(ScreenReaderState::Unknown, true),
            "an unknown state is not a screen reader"
        );
    }
}
