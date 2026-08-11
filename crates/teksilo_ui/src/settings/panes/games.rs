// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Writing games — self-imposed drafting constraints.
//!
//! One game today: **Always forward**. The pane's unusual shape follows from the
//! feature's one unusual property — *the switch at the top is not a setting*.
//! Playing is per-session state living on the project's `WorkSession`, while the
//! two "where it applies" boxes under it are ordinary persisted preferences. The
//! pane says so in as many words rather than leaving the writer to discover it
//! when the game is gone after a restart.

use teksilo::prelude::*;
use teksilo::widgets::VStack;

use crate::tabs::shared::VisibleWhen;

#[allow(unused_imports)]
use super::super::*;

/// Editor ▸ Writing games.
///
/// Takes the whole [`WritingGamesViewModel`](crate::view_models::WritingGamesViewModel)
/// rather than four loose signals: the "is it bound to nothing" warning is a
/// question about the combination, and answering it here would duplicate a rule
/// the view-model already owns.
pub(in crate::settings) fn games_pane(
    ctx: &mut BuildContext,
    games: &crate::view_models::WritingGamesViewModel,
) -> impl Widget {
    // `is_inert` reads three signals, so it needs re-evaluating whenever any of
    // them moves. A derived signal would recompute on every read (and panics
    // under `observe`), so this mirrors into a plain signal from three effects —
    // the same shape the editor wiring uses to push the command filter.
    let inert = Signal::new(games.is_inert());
    {
        let (g, out) = (games.clone(), inert.clone());
        let refresh = move || {
            let now = g.is_inert();
            if out.get() != now {
                out.set(now);
            }
        };
        {
            let refresh = refresh.clone();
            ctx.effect(&games.always_forward(), move |_| refresh());
        }
        {
            let refresh = refresh.clone();
            ctx.effect(&games.forward_in_prose(), move |_| refresh());
        }
        ctx.effect(&games.forward_in_synopsis(), move |_| refresh());
    }

    let form = FormLayout::new()
        .label(tr!(settings_page_games()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_games_forward())))
        .full_width(Toggle::new(games.always_forward()).label(tr!(settings_games_forward_toggle())))
        .full_width(hint(tr!(settings_games_forward_hint())))
        // The session warning is the whole reason this pane reads differently
        // from every other one: everything else in this window is remembered.
        .full_width(
            TextWidget::new(tr!(settings_games_session_warning()))
                .style(TextStyleRole::Small)
                .color(TextRole::Warning),
        )
        .full_width(group(tr!(settings_group_games_scope())))
        .full_width(Toggle::new(games.forward_in_prose()).label(tr!(settings_games_in_prose())))
        .full_width(
            Toggle::new(games.forward_in_synopsis()).label(tr!(settings_games_in_synopsis())),
        )
        .full_width(hint(tr!(settings_games_scope_hint())))
        // Shown only when the writer has switched the game on and then turned
        // every surface off — a real state, and one where nothing at all would
        // happen as they type. Warned about rather than prevented: they may be
        // half-way through choosing.
        .full_width(VisibleWhen::new(
            inert,
            TextWidget::new(tr!(settings_games_inert_warning()))
                .style(TextStyleRole::Small)
                .color(TextRole::Warning),
        ));

    pane_frame(
        crumb(Some(tr!(settings_sec_editor())), tr!(settings_page_games())),
        VStack::new().child(form),
    )
}
