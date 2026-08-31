// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The status bar's writing-game warning: a standing reminder, while "Always
//! forward" is being played, that deleting is off.
//!
//! **Why the app warns at all.** A mode that silently swallows Backspace is the
//! single most-reported complaint about every tool that ships this feature —
//! a writer who has forgotten they switched it on reads a dead key as a broken
//! keyboard, not as a rule they set for themselves. So the state is always
//! visible while it lasts, and the badge is also the way out: clicking it stops
//! the game.
//!
//! **Why a badge and not a toast.** The game lasts a whole drafting session and
//! the fact stays true the entire time; a toast appears once and is gone by the
//! time the writer hits the key it was explaining. This sits beside the save
//! glyph, which makes the same argument about a different always-true fact.
//!
//! Takes no width at all when no game is on — a `VisibleWhen` rather than a
//! dimmed placeholder, because an inert reminder about a rule nobody is playing
//! under is just clutter in a bar the writer glances at.

use teksilo::prelude::*;
use teksilo::widgets::{HStack, IconButton, IconButtonSize, TextWidget};

use crate::tabs::shared::VisibleWhen;
use crate::writing_session::WritingGamesViewModel;

/// Build the status-bar game warning.
///
/// `has_work` is the same "is a project open" test the save glyph and the word
/// count already use: with no project there is no `Work` to be playing, and the
/// signal is threaded rather than re-derived so the three cannot disagree.
pub fn game_indicator(
    games: &WritingGamesViewModel,
    has_work: Signal<bool>,
    backup_mode: Signal<bool>,
) -> impl Widget {
    // Shown while the game is on *and* there is a normal project to play it in.
    // Backup mode is excluded for the same reason Save is: that window is a
    // read-only look at an old copy, and offering a drafting game there would be
    // promising something about a file the writer cannot keep.
    let visible = games
        .always_forward()
        .zip3(&has_work, &backup_mode)
        .map(|(on, work, backup)| *on && *work && !*backup);

    let stop = games.clone();
    VisibleWhen::new(
        visible,
        HStack::new()
            .spacing(4.0)
            .child(
                IconButton::new(crate::icons::editor::forward_only())
                    .size(IconButtonSize::Compact)
                    // Amber, not red: playing a writing game is a deliberate,
                    // harmless state — the same reasoning the save glyph uses for
                    // "unsaved". The error role's job is to be alarming, and this
                    // is a rule the writer chose.
                    .icon_role(ColorProp::undimmed(TextRole::Warning))
                    .tooltip(tr!(statusbar_games_forward_tooltip()))
                    .on_activate_fn(move |_| stop.set_always_forward(false)),
            )
            // The name beside the glyph, because a lone unfamiliar icon is
            // exactly what a writer would fail to connect to their dead
            // Backspace key — which is the whole failure this widget exists to
            // prevent.
            .child(
                TextWidget::new(tr!(statusbar_games_forward()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Warning),
            ),
    )
}
