// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The control strip's **quick-access popover** — the mode's settings, reachable
//! without leaving it.
//!
//! Otherwise they are not reachable at all: the menu bar is parked dormant
//! behind the surface, so the only route to Settings would be Ctrl+, putting the
//! full modal over the manuscript.
//!
//! Deliberately a **reduced** view, not the settings page. Two tabs:
//!
//! 1. **Typography** — the same rows the page shows, shared through
//!    `settings::panes::typography::typography_rows`, plus the mode's column
//!    width. What a writer actually reaches for mid-sentence.
//! 2. **Themes** — the list, and one button through to the real Settings page.
//!    Creating, importing, editing and deleting stay there; this is for
//!    *picking* while writing.
//!
//! Height-capped: a `FontPicker` plus sliders plus a theme list will otherwise
//! outgrow the window it is anchored in.

use teksilo::data::ListModel;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, HStack, IconButton, ListView, MaxSize, PopoverIconButton, ScrollArea,
    StandardListItem, TabInfo, TabWidget, VStack,
};

use crate::distraction_free::theme::DistractionFreeTheme;
use crate::view_models::{DistractionFreeThemesViewModel, SettingsViewModel};

/// Cap on the popover so it cannot outgrow a small window.
const MAX_HEIGHT: f32 = 460.0;
const MAX_WIDTH: f32 = 420.0;

/// The gear pinned in the control strip.
pub fn quick_settings_button(
    ctx: &mut BuildContext,
    settings: SettingsViewModel,
    themes: DistractionFreeThemesViewModel,
) -> impl Widget {
    let body = body(ctx, settings, themes);
    PopoverIconButton::new(
        IconButton::new(crate::icons::session::gear())
            .toolbar()
            .tooltip(tr!(statusbar_focus_settings())),
    )
    .content(body)
}

fn body(
    ctx: &mut BuildContext,
    settings: SettingsViewModel,
    themes: DistractionFreeThemesViewModel,
) -> impl Widget {
    let typo = settings.editor_typography();
    // The *same rows* the settings page builds, not a second copy of them: a
    // slider added there has to appear here without anybody remembering to.
    let form = crate::settings::panes::typography::typography_rows(
        ctx,
        teksilo::widgets::FormLayout::new()
            .label_gap(12.0)
            .row_spacing(10.0),
        &typo.distraction_free,
    );
    let form = form.line(
        crate::settings::field_label(tr!(settings_field_column_width())),
        crate::settings::slider_field(
            settings.distraction_free_width(),
            400.0,
            1200.0,
            20.0,
            |v| format!("{} px", v.round() as i32),
        ),
    );

    let selected: Signal<Option<teksilo::widgets::TabId>> = Signal::new(None);
    let tabs = TabWidget::new(selected)
        .static_tab(
            TabInfo::new().title(tr!(settings_group_typography())),
            ScrollArea::new().child(form),
        )
        .static_tab(
            TabInfo::new().title(tr!(settings_page_distraction_free_themes())),
            themes_tab(ctx, themes, settings.distraction_free_theme()),
        )
        .compact_bar();

    MaxSize::new(MAX_WIDTH, MAX_HEIGHT).child(tabs)
}

/// The theme list, plus the one way through to the full library.
fn themes_tab(
    ctx: &mut BuildContext,
    themes: DistractionFreeThemesViewModel,
    current: Signal<String>,
) -> impl Widget {
    let model = ListModel::from_vec(themes.all_themes());
    {
        // The library is app-wide: another window can import or delete while this
        // popover exists.
        let model = model.clone();
        let themes = themes.clone();
        ctx.effect(&themes.changed_signal(), move |_| {
            model.replace_all(themes.all_themes());
        });
    }
    let cur = current.clone();
    let rows = themes.clone();
    let list = ListView::new(model, move |_i, t: &DistractionFreeTheme, _sel| {
        let id_state = t.id.clone();
        Box::new(
            StandardListItem::new(lit!(t.name.clone()))
                .selected(cur.map(move |c| c.as_str() == id_state.as_str())),
        )
    })
    .auto_item_height(36.0)
    // Single click: this is a picker reached mid-sentence, not a file list —
    // asking for a double click to change theme would be one gesture too many.
    .activate_on(teksilo::widgets::ActivateOn::SingleClick)
    .on_activate(move |i, _ctx| {
        if let Some(t) = rows.all_themes().get(i) {
            current.set(t.id.clone());
        }
    });

    // Everything the popover deliberately does *not* do lives one click away.
    // Ghost for the same reason Exit is: a `Plain` fill is `SurfaceRole::Main`,
    // which on this surface is the theme's general background, not the surface
    // the button actually sits on.
    let manage = Button::new(tr!(statusbar_focus_manage_themes()))
        .variant(ButtonVariant::Ghost)
        .on_activate_fn(|ctx| ctx.send_intent(Intent::new("app.settings")));

    VStack::new()
        .spacing(6.0)
        .child(teksilo::widgets::Expand::new().child(list))
        .child(
            HStack::new()
                .child(teksilo::widgets::Spacer::new())
                .child(manage),
        )
}
