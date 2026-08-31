// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Writing games** dock (leading rail): the writer's self-imposed drafting
//! constraints, switched on and off where they are being played rather than
//! buried in a preferences window.
//!
//! One game today — **Always forward** — presented as a card: name, what it does
//! to you, a switch, and a live line saying what it currently covers. The
//! Settings page (Editor ▸ Writing games) owns the *options*; this dock owns the
//! *playing*, which is the thing a writer reaches for mid-session and should not
//! have to open a modal window to reach.
//!
//! Deliberately a rail dock rather than a menu item: the roster is meant to grow
//! (the roadmap has more games in it), and a panel that lists them with a
//! sentence each is how a writer discovers a rule they might want to play under.

use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, DockOpenLocation, DockSide, DockWidget, DockWidgetId, Expand, Padding,
    ScrollArea, Switcher, TextWidget, Toggle, VStack,
};

use teksilo::widgets::tooltip::TooltipContent;

use crate::writing_session::WritingGamesViewModel;

/// Build the writing-games panel as a leading-side `DockWidget`.
///
/// `on_settings` opens Settings at the games page — passed in rather than fired
/// from here so the dock does not need to know how this app opens its settings
/// window, which is `App`'s business.
pub fn games_dock(
    games: WritingGamesViewModel,
    dock_id: DockWidgetId,
    on_settings: std::rc::Rc<dyn Fn(&mut EventContext)>,
) -> DockWidget {
    DockWidget::new(dock_id, tr!(games_title()), move |_id| {
        games_panel(games.clone(), on_settings.clone())
    })
    .icon(crate::icons::activity::games_icon)
    .show_header(true)
    .default_location(DockOpenLocation::side(DockSide::Leading))
}

/// The panel body: the game cards, then the way to their options.
fn games_panel(
    games: WritingGamesViewModel,
    on_settings: std::rc::Rc<dyn Fn(&mut EventContext)>,
) -> impl Widget {
    let body = VStack::new()
        .spacing(10.0)
        .child(always_forward_card(games))
        .child(
            Button::new(tr!(games_settings_link()))
                .variant(ButtonVariant::Plain)
                .on_activate_fn(move |ctx| on_settings(ctx)),
        );

    Expand::new().child(ScrollArea::new().child(Padding::symmetric(10.0, 10.0).child(body)))
}

/// The one game: its name, what playing costs you, the switch, and what it
/// currently covers.
fn always_forward_card(games: WritingGamesViewModel) -> impl Widget {
    // Reactive *localized* text is swapped by a `Switcher` rather than pushed
    // through `.text()`: that setter takes a plain `String` prop, so a mapped
    // signal of `LocalizedString` cannot drive it — and going through `String`
    // would resolve the translation once and then stop following a locale change.
    //
    // Playing or not, in words. Same signal the toggle drives, so the line beside
    // the switch cannot lag behind it.
    let status = Switcher::new(games.always_forward().map(|on| usize::from(*on)))
        .child(
            TextWidget::new(tr!(games_forward_idle()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
        .child(
            TextWidget::new(tr!(games_forward_playing()))
                .style(TextStyleRole::Small)
                // Amber while playing — the same colour, and the same argument,
                // as the status-bar badge: a deliberate, harmless state.
                .color(TextRole::Warning),
        );

    // What the game covers, from the two options — so a card that said "applies
    // to your prose" cannot survive the writer turning prose off. Reads the same
    // whether the game is on or off: switched off it answers "what would happen
    // if I switched this on", which is what someone reading the card wants.
    let scope_index = games
        .forward_in_prose()
        .zip(&games.forward_in_synopsis())
        .map(|(prose, synopsis)| match (*prose, *synopsis) {
            (true, true) => 0usize,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        });
    let scope_line = |text: LocalizedString| {
        TextWidget::new(text)
            .style(TextStyleRole::Small)
            .color(TextRole::Secondary)
    };
    let scope = Switcher::new(scope_index)
        .child(scope_line(tr!(games_scope_prose_and_synopsis())))
        .child(scope_line(tr!(games_scope_prose())))
        .child(scope_line(tr!(games_scope_synopsis())))
        .child(scope_line(tr!(games_scope_nothing())));

    VStack::new()
        .spacing(6.0)
        // No separate heading above the switch: the toggle already carries the
        // game's name, and a card that says "Always forward" twice reads as a
        // layout mistake. The label is on the *control* rather than over it so a
        // screen reader announces the name with the state it applies to.
        //
        // What playing costs you lives in the toggle's own rich tooltip rather
        // than as a paragraph above it: the card is a control, and a permanent
        // wall of explanation is read once and then only ever in the way. The
        // writer who wants the rule hovers the switch that applies it.
        .child(
            Toggle::new(games.always_forward())
                .label(tr!(games_forward_name()))
                .rich_tooltip_content(TooltipContent::new(
                    "games.always_forward",
                    tr!(games_forward_blurb()),
                )),
        )
        .child(status)
        .child(scope)
        // The one thing a writer must know before switching it on, and the thing
        // no other toggle in this app does: it is not remembered.
        .child(
            TextWidget::new(tr!(games_session_note()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
}
