// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Format** menu — the same commands the Format dock carries.
//!
//! One source of truth, two surfaces: each row binds the very signal the dock's
//! button binds, so the two cannot disagree about whether the selection is bold.

use teksilo::prelude::*;
use teksilo::widgets::{MenuEntry, MenuItems};

use super::{ProjectMenuParts, command, mark};
use crate::format::{ALIGN_CENTER, ALIGN_LEFT, DIR_AUTO, DIR_LTR, DIR_RTL, FormatViewModel};

/// The rows of the menu, in the order they appear.
pub(super) fn menu(m: MenuItems, parts: &ProjectMenuParts) -> MenuItems {
    let menu_scene_focused = parts.scene_focused.clone();
    let menu_format_vm = parts.format.clone();

    // Enabled on the *sticky* target, not on live focus:
    // opening this menu moves focus to the menu overlay,
    // so an enablement keyed on focus would grey every
    // row out at the instant the user reached for one.
    // Scene breaks keep their own narrower gate — the
    // same predicate the compiler uses, so the menu can
    // never offer a mark the exporter would ignore.
    //
    // Rows stay visible when disabled: a greyed row still
    // teaches that the feature exists and what its
    // shortcut is, and still reaches the a11y tree. That
    // is the opposite of the dock, which hides what does
    // not apply — a menu is a map of what exists, a dock
    // is a set of what applies right now.
    //
    // Text-only, with no glyphs: `MenuEntry` has no
    // `.icon()`. Parity with the dock means the same
    // commands and the same state, not the same look.
    let on_scene = menu_scene_focused.clone();
    let f = menu_format_vm.clone();

    let on = f.has_target();
    // Bold/Italic/Underline are handled inside
    // `RichTextEditor`'s own key dispatch, not the
    // shortcut registry, so there is no id to bind —
    // the chord travels in the label instead.
    let mut m = m
        .item(mark(
            &f,
            tr!(menu_format_marks_bold()),
            on.clone(),
            f.bold(),
            FormatViewModel::toggle_bold,
        ))
        .item(mark(
            &f,
            tr!(menu_format_marks_italic()),
            on.clone(),
            f.italic(),
            FormatViewModel::toggle_italic,
        ))
        .item(mark(
            &f,
            tr!(menu_format_marks_underline()),
            on.clone(),
            f.underline(),
            FormatViewModel::toggle_underline,
        ))
        .item(mark(
            &f,
            tr!(menu_format_marks_strike()),
            on.clone(),
            f.strikethrough(),
            FormatViewModel::toggle_strikethrough,
        ))
        .item(mark(
            &f,
            tr!(menu_format_marks_superscript()),
            on.clone(),
            f.superscript(),
            FormatViewModel::toggle_superscript,
        ))
        .item(mark(
            &f,
            tr!(menu_format_marks_subscript()),
            on.clone(),
            f.subscript(),
            FormatViewModel::toggle_subscript,
        ))
        // Not `command(..)`: that helper takes `fn(&FormatViewModel)`, and
        // opening a dialog needs the `EventContext` only `on_activate` has.
        // It reaches the same registered action the dock button and Ctrl+K do,
        // so `MenuEntry::shortcut` can render the chord per platform rather
        // than this label hardcoding one.
        .item({
            let f = f.clone();
            // `checked`, like every mark above — reflect-only, mirroring the
            // same signal the dock's button binds, so the two surfaces agree
            // about whether the caret is on a link.
            let state = f.link();
            MenuEntry::new(tr!(menu_format_link()))
                .enabled(on.clone())
                .checked(state)
                .shortcut("format.link")
                .on_activate(move |c| crate::format::link_panel::present(&f, c))
        })
        .item(command(
            &f,
            tr!(menu_format_marks_clear()),
            on.clone(),
            FormatViewModel::clear_formatting,
        ))
        .separator();

    // Seven levels, one exclusive choice — a radio
    // group over the index the caret already reports.
    m = m.submenu(tr!(menu_format_heading()), {
        let f = f.clone();
        let on = on.clone();
        move |s| {
            let mut s = s;
            for (level, label) in [
                (0usize, tr!(menu_format_heading_normal())),
                (1, tr!(menu_format_heading_1())),
                (2, tr!(menu_format_heading_2())),
                (3, tr!(menu_format_heading_3())),
                (4, tr!(menu_format_heading_4())),
                (5, tr!(menu_format_heading_5())),
                (6, tr!(menu_format_heading_6())),
            ] {
                let f = f.clone();
                s = s.item(
                    MenuEntry::new(label)
                        .enabled(on.clone())
                        .radio(level, f.heading())
                        .on_activate(move |c| {
                            f.set_heading(level);
                            c.request_frame();
                            f.refocus(c);
                        }),
                );
            }
            s
        }
    });

    m = m.submenu(tr!(menu_format_alignment()), {
        let f = f.clone();
        let on = on.clone();
        move |s| {
            let mut s = s;
            for (idx, label) in [
                (ALIGN_LEFT, tr!(menu_format_align_left())),
                (ALIGN_CENTER, tr!(menu_format_align_center())),
            ] {
                let f = f.clone();
                s = s.item(
                    MenuEntry::new(label)
                        .enabled(on.clone())
                        .radio(idx, f.alignment())
                        .on_activate(move |c| {
                            f.set_alignment(idx);
                            c.request_frame();
                            f.refocus(c);
                        }),
                );
            }
            s
        }
    });

    // Paragraph direction gets the full three-way
    // radio the dock's single toggle cannot express:
    // "automatic" is a real state, distinct from a
    // pinned left-to-right, and worth reaching.
    m = m.submenu(tr!(menu_format_direction()), {
        let f = f.clone();
        let on = on.clone();
        move |s| {
            let mut s = s;
            for (idx, label) in [
                (DIR_AUTO, tr!(menu_format_direction_auto())),
                (DIR_LTR, tr!(menu_format_direction_ltr())),
                (DIR_RTL, tr!(menu_format_direction_rtl())),
            ] {
                let f = f.clone();
                s = s.item(
                    MenuEntry::new(label)
                        .enabled(on.clone())
                        .radio(idx, f.direction())
                        .on_activate(move |c| {
                            f.set_direction(idx);
                            c.request_frame();
                            f.refocus(c);
                        }),
                );
            }
            s
        }
    });

    m = m
        .item(mark(
            &f,
            tr!(menu_format_blockquote()),
            on.clone(),
            f.blockquote(),
            FormatViewModel::toggle_blockquote,
        ))
        .submenu(tr!(menu_format_lists()), {
            let f = f.clone();
            let on = on.clone();
            move |s| {
                s.item(command(
                    &f,
                    tr!(menu_format_list_bullet()),
                    on.clone(),
                    FormatViewModel::insert_bullet_list,
                ))
                .item(command(
                    &f,
                    tr!(menu_format_list_numbered()),
                    on.clone(),
                    FormatViewModel::insert_numbered_list,
                ))
                .separator()
                .item(command(
                    &f,
                    tr!(menu_format_indent()),
                    on.clone(),
                    FormatViewModel::indent,
                ))
                .item(command(
                    &f,
                    tr!(menu_format_outdent()),
                    on.clone(),
                    FormatViewModel::outdent,
                ))
            }
        })
        .submenu(tr!(menu_format_table()), {
            let f = f.clone();
            let on = on.clone();
            move |s| {
                // `insert_table` takes two runtime
                // numbers and `MenuEntry` has no
                // payload slot, so the sizes are
                // spelled out rather than prompted.
                let sizes = s.submenu(tr!(menu_format_table_insert()), {
                    let f = f.clone();
                    let on = on.clone();
                    move |t| {
                        let mut t = t;
                        for (n, label) in [
                            (2usize, tr!(menu_format_table_2x2())),
                            (3, tr!(menu_format_table_3x3())),
                            (4, tr!(menu_format_table_4x4())),
                        ] {
                            let f = f.clone();
                            t = t.item(MenuEntry::new(label).enabled(on.clone()).on_activate(
                                move |c| {
                                    f.insert_table(n, n);
                                    c.request_frame();
                                    f.refocus(c);
                                },
                            ));
                        }
                        t
                    }
                });
                // The row/column commands are gated
                // on the caret actually being in a
                // table — the dock hides them, a menu
                // greys them.
                let in_table = f.in_table();
                sizes
                    .separator()
                    .item(command(
                        &f,
                        tr!(menu_format_table_row_above()),
                        in_table.clone(),
                        FormatViewModel::insert_row_above,
                    ))
                    .item(command(
                        &f,
                        tr!(menu_format_table_row_below()),
                        in_table.clone(),
                        FormatViewModel::insert_row_below,
                    ))
                    .item(command(
                        &f,
                        tr!(menu_format_table_col_before()),
                        in_table.clone(),
                        FormatViewModel::insert_column_before,
                    ))
                    .item(command(
                        &f,
                        tr!(menu_format_table_col_after()),
                        in_table.clone(),
                        FormatViewModel::insert_column_after,
                    ))
                    .separator()
                    .item(command(
                        &f,
                        tr!(menu_format_table_row_delete()),
                        in_table.clone(),
                        FormatViewModel::remove_row,
                    ))
                    .item(command(
                        &f,
                        tr!(menu_format_table_col_delete()),
                        in_table.clone(),
                        FormatViewModel::remove_column,
                    ))
                    .item(command(
                        &f,
                        tr!(menu_format_table_remove()),
                        in_table,
                        FormatViewModel::remove_table,
                    ))
            }
        })
        // Undo and Redo used to sit here, acting on the focused editor's own
        // history. They now live in **Edit**, where they act on whichever
        // history the caret is in. Two Undo rows in two menus, meaning
        // different things, is exactly the confusion that work removed.
        .separator();

    m.item(
        MenuEntry::new(tr!(menu_scene_break()))
            .enabled(on_scene.clone())
            .intent("format.scene_break")
            .shortcut("format.scene_break"),
    )
    .item(
        MenuEntry::new(tr!(menu_major_scene_break()))
            .enabled(on_scene.clone())
            .intent("format.major_scene_break")
            .shortcut("format.major_scene_break"),
    )
    .separator()
    // Gated by `.enabled()`, never `.visible()`, for the
    // same reason the scene-break rows above are: a row
    // that vanishes teaches nobody the shortcut exists.
    .item(
        MenuEntry::new(tr!(comments_menu_add()))
            .enabled(on_scene.clone())
            .intent("comments.add")
            .shortcut("comments.add"),
    )
    .item(
        MenuEntry::new(tr!(comments_menu_add_paragraph()))
            .enabled(on_scene.clone())
            .intent("comments.add_paragraph")
            .shortcut("comments.add_paragraph"),
    )
}
