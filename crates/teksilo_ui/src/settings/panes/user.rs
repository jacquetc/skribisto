// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ User — who is using this installation.
//!
//! Two fields, both app-level and both optional. They answer "who wrote this
//! remark", which is a property of the *person at the keyboard* and not of the
//! manuscript: `Work: <name> ▸ Author` is the book's byline, it rides inside the
//! `.skrib`, and signing comments with it means an editor who opens someone
//! else's project signs their notes with the novelist's name. Hence a top-level
//! page rather than a corner of Appearance — and hence the cross-reference in
//! each field's hint, since the two are otherwise easy to confuse.
//!
//! Bound **straight** to the settings-store signals rather than mirrored into a
//! local one and committed on blur, the way `work_author` has to be: a store
//! signal is two-way and its disk write is already debounced (500 ms), so there
//! is no per-keystroke file write to defend against and no entity `save()` to
//! route through. Typing here therefore reaches the next comment immediately,
//! which is what the effect in `App::build` is watching for.

use teksilo::prelude::*;
use teksilo::widgets::TextInput;
use teksilo::widgets::tooltip::TooltipContent;

#[allow(unused_imports)]
use super::super::*;

/// Settings ▸ User — the signing name and initials.
pub(in crate::settings) fn user_pane(vm: &SettingsViewModel) -> impl Widget {
    let name = TextInput::new(vm.user_name())
        .placeholder(tr!(settings_field_user_name_placeholder()))
        .rich_tooltip_content(TooltipContent::new(
            "settings.user_name",
            tr!(settings_field_user_name_hint()),
        ));

    // Deliberately not `.max_length(3)`: `MAX_INITIALS` caps what the app
    // *derives*, not what a person may call themselves, and a field that
    // silently swallowed the fourth keystroke would be the app overruling
    // someone about their own name.
    let initials = FixedSize::new().width(120.0).child(
        TextInput::new(vm.user_initials())
            .placeholder(tr!(settings_field_user_initials_placeholder()))
            .rich_tooltip_content(TooltipContent::new(
                "settings.user_initials",
                tr!(settings_field_user_initials_hint()),
            )),
    );

    let form = FormLayout::new()
        .label(tr!(settings_page_user()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .line(field_label(tr!(settings_field_user_name())), name)
        .line(field_label(tr!(settings_field_user_initials())), initials)
        .full_width(hint(tr!(settings_field_user_hint())));

    pane_frame(crumb(None, tr!(settings_page_user())), form)
}
