// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Dock and find-banner commands: one "show" command per panel (see
//! [`crate::docks::panels`]), the three side toggles, the search & replace dock's
//! replace mode, the per-editor find banner, this window's plain-fullscreen toggle, and
//! its distraction-free mode toggle.

use teksilo::prelude::*;
use teksilo::widgets::{DockSide, DockWidgetId, DockingModel};

use super::CommandDeps;
use crate::shared::shortcut_group::{ShortcutGroup, ShortcutGroupExt};

/// Run a dock command, unless this window's distraction-free surface is up.
///
/// Disabling a dock side does **not** disable its commands — `DockingModel::
/// reveal_dock` sets `visible = true` and picks a tab regardless — and with the
/// shell merely dormant behind the surface, nothing disables anything at all.
/// Left unguarded, a dock command fired in the mode silently rearranges the
/// desk behind the surface (e.g. the leading rail switches to Search) with no
/// visible effect until the mode is exited.
fn unless_distraction_free(focus: &crate::shared::FocusViewModel, f: impl FnOnce()) {
    if !focus.active_signal().get() {
        f();
    }
}

/// Show or hide a whole side and say so.
///
/// Toggling leaves the keyboard where it is, and a reader who is not in the side has no
/// other way to tell that anything happened: the rail's and the menu's check marks are
/// not where the focus is. Where the keyboard goes when it *was* in the side is
/// [`crate::workspace_layout::SideFocusKeeper`]'s business, because that holds for every
/// way a side can hide.
fn toggle_side(docking: &DockingModel, side: DockSide, ctx: &mut EventContext) {
    let was_visible = docking.is_side_visible(side);
    docking.toggle_side_visible(side);
    let visible = docking.is_side_visible(side);
    if visible != was_visible {
        ctx.announce(side_change_message(side, visible));
    }
}

/// What a reader hears when `side` has just been shown or hidden, in the words of the
/// *View* menu's rows.
fn side_change_message(side: DockSide, visible: bool) -> String {
    match (side, visible) {
        (DockSide::Leading, true) => tr!(side_start_shown()),
        (DockSide::Leading, false) => tr!(side_start_hidden()),
        (DockSide::Trailing, true) => tr!(side_end_shown()),
        (DockSide::Trailing, false) => tr!(side_end_hidden()),
        (_, true) => tr!(side_bottom_shown()),
        (_, false) => tr!(side_bottom_hidden()),
    }
    .into()
}

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    // F9, not Ctrl+B: Ctrl+B is the editor's built-in bold command, and a Global shortcut is
    // resolved *before* the focused widget sees the raw key — so a Ctrl+B binding here would
    // shadow `RichTextEditor`'s bold.
    ctx.register_shortcut_global(
        Shortcut::new("outline.toggle")
            .group(ShortcutGroup::View)
            .name(tr!(shortcut_name_outline_toggle()))
            .primary(KeyStroke::new(Key::F9, Modifiers::NONE))
            .build(),
    );
    // Shows or hides the bottom band. Unlike F9, which only relayouts, this parks and
    // unparks the bottom content via `visible_when`. No default chord: it had F10, which
    // the menu bar takes first on Linux and Windows (and screen-reader users reach the
    // menus with), so it never fired there while the View menu went on advertising it.
    // The search preview's own door is `preview.show` (Ctrl+Shift+J).
    ctx.register_shortcut_global(
        Shortcut::new("preview.toggle")
            .group(ShortcutGroup::View)
            .name(tr!(shortcut_name_preview_toggle()))
            .build(),
    );
    // The right side's counterpart of F9, with no default chord: the status bar's
    // button is its other door, and every panel on that side has its own.
    ctx.register_shortcut_global(
        Shortcut::new("view.right_panel")
            .group(ShortcutGroup::View)
            .name(tr!(shortcut_name_side_end_toggle()))
            .build(),
    );
    {
        let docking = deps.outline.docking();
        let focus = deps.focus.clone();
        ctx.register_action_global(Action::new("view.right_panel").on_invoke(move |_i, c| {
            unless_distraction_free(&focus, || toggle_side(&docking, DockSide::Trailing, c));
        }));
    }
    register_panel_commands(ctx, deps);
    // Every dock command below runs through `unless_distraction_free` — see its own doc.
    {
        let docking = deps.outline.docking();
        let focus = deps.focus.clone();
        ctx.register_action_global(Action::new("preview.toggle").on_invoke(move |_i, c| {
            unless_distraction_free(&focus, || toggle_side(&docking, DockSide::Bottom, c));
        }));
    }
    {
        let docking = deps.outline.docking();
        let focus = deps.focus.clone();
        ctx.register_action_global(Action::new("outline.toggle").on_invoke(move |_i, c| {
            unless_distraction_free(&focus, || toggle_side(&docking, DockSide::Leading, c));
        }));
    }

    // Ctrl+Shift+M, not a function key: the lane is a writing-surface toggle
    // rather than a window-chrome one, and the free function keys are spoken
    // for (F9 outline, F10 the menu bar, F11 distraction-free). M for "marks"; Ctrl+M
    // alone is a plain Return in several editors, so the Shift is not
    // decorative.
    ctx.register_shortcut_global(
        Shortcut::new("view.margin_lane")
            .group(ShortcutGroup::View)
            .name(tr!(shortcut_name_view_margin_lane()))
            .primary(KeyStroke::new(Key::M, Modifiers::CTRL | Modifiers::SHIFT))
            .build(),
    );
    {
        // The setting *is* the state: the menu row reflects this signal and the
        // action writes it, so the toggle and Settings can never disagree about
        // whether the lane is on. Same pairing every other reflect-only row in
        // this menu uses.
        let enabled = ctx.settings().signal(
            crate::MARGIN_LANE_ENABLED_KEY,
            crate::MARGIN_LANE_ENABLED_DEFAULT,
        );
        ctx.register_action_global(Action::new("view.margin_lane").on_invoke(move |_i, _c| {
            enabled.set(!enabled.get());
        }));
    }

    // Shift+F11. Plain F11 is the platform convention for fullscreen, but here it
    // belongs to distraction-free mode below: the writer reaches for that one far
    // more often, so it gets the single key. A Global shortcut so it fires
    // regardless of which widget has focus, same rationale as F9 above.
    ctx.register_shortcut_global(
        Shortcut::new("view.fullscreen")
            .group(ShortcutGroup::View)
            .name(tr!(shortcut_name_view_fullscreen()))
            .primary(KeyStroke::new(Key::F11, Modifiers::SHIFT))
            .build(),
    );
    {
        let fullscreen = deps.fullscreen.clone();
        ctx.register_action_global(Action::new("view.fullscreen").on_invoke(move |_i, c| {
            // Resolve THIS event's own window rather than a captured handle —
            // correct with several project windows open. No-op in the (never
            // reachable from a real project window) headless case where
            // `ctx.window()` is `None`.
            if let Some(window) = c.window() {
                fullscreen.toggle(window);
            }
        }));
    }

    // F11 — Increment 2 of distraction-free: chrome collapse + docks
    // disabled + fullscreen, together, as one per-window mode independent of
    // the Shift+F11 fullscreen toggle above (see `FocusViewModel`'s module doc for why
    // the two never share placement memory). Global for the same reason as
    // Shift+F11/F9.
    ctx.register_shortcut_global(
        Shortcut::new("view.focus_mode")
            .group(ShortcutGroup::View)
            .name(tr!(shortcut_name_view_focus_mode()))
            .primary(KeyStroke::new(Key::F11, Modifiers::NONE))
            .build(),
    );
    {
        let focus = deps.focus.clone();
        let seed_synopsis = deps.editors.show_synopsis();
        ctx.register_action_global(Action::new("view.focus_mode").on_invoke(move |_i, c| {
            // Same resolve-the-firing-window rationale as `view.fullscreen`
            // above; also the target of the strip's Exit button, which fires
            // this same named intent (`toggle` is always a clean exit there —
            // the button only renders while the mode is active). The
            // contextless-Escape handler in `App::build` calls
            // `FocusViewModel::exit` directly instead, for the stronger
            // idempotency guarantee that method carries (see its doc) — a
            // raw key handler, not a discoverable command, same precedent as
            // the find banner's own local Escape handling.
            if let Some(window) = c.window() {
                // Seed the mode's synopsis from the ordinary editor preference, so
                // entering distraction-free looks like the editor the writer just
                // left rather than silently overriding a choice they made.
                focus.toggle(window, seed_synopsis.get());
            }
        }));
    }

    // Text size: the keyboard half of Ctrl+Wheel. The wheel is already standing
    // on the editor it means; these have to be told, so they resolve through the
    // format registry's focused/sticky editor — which is what makes them work
    // from the View menu too, where a menu opened from the keyboard has taken
    // focus off the editor.
    //
    // Global for the same reason as every chord above: resolved before the
    // focused widget sees the key, so `RichTextEditor` cannot eat them.
    ctx.register_shortcut_global(
        Shortcut::new("editor.size.increase")
            .group(ShortcutGroup::View)
            .name(tr!(shortcut_name_editor_size_increase()))
            .primary(KeyStroke::ctrl(Key::Character('=')))
            // Ctrl+Plus as an alias, because '+' is Shift+'=' on a US/ISO
            // keyboard and half of users press it that way — the same pair
            // every browser accepts.
            .secondary(KeyStroke::ctrl(Key::Character('+')))
            .build(),
    );
    ctx.register_shortcut_global(
        Shortcut::new("editor.size.decrease")
            .group(ShortcutGroup::View)
            .name(tr!(shortcut_name_editor_size_decrease()))
            .primary(KeyStroke::ctrl(Key::Character('-')))
            .build(),
    );
    ctx.register_shortcut_global(
        Shortcut::new("editor.size.reset")
            .group(ShortcutGroup::View)
            .name(tr!(shortcut_name_editor_size_reset()))
            .primary(KeyStroke::ctrl(Key::Character('0')))
            .build(),
    );
    for (id, notches) in [
        ("editor.size.increase", Some(1)),
        ("editor.size.decrease", Some(-1)),
        ("editor.size.reset", None),
    ] {
        let format = deps.format.clone();
        ctx.register_action_global(Action::new(id).on_invoke(move |_i, c| {
            // No editor focused and none latched — or a surface with no size
            // preference of its own — means there is nothing to resize. A
            // no-op, like every other command that acts through this registry.
            let Some(typo) = format.focused_typography() else {
                return;
            };
            match notches {
                Some(n) => crate::shared::editor_size::step_and_announce(c, &typo, n),
                None => crate::shared::editor_size::reset_and_announce(c, &typo),
            }
        }));
    }

    // Ctrl+F opens the per-editor find banner in the focused pane's active tab (its
    // `FindViewModel`). A *global* shortcut is resolved before the focused widget sees the
    // key — the editor must not eat Ctrl+F — but the action reads which tab is focused, so
    // it targets the right editor even in a split view.
    ctx.register_shortcut_global(
        Shortcut::new("editor.find")
            .group(ShortcutGroup::Edit)
            .name(tr!(shortcut_name_editor_find()))
            .primary(KeyStroke::ctrl(Key::F))
            .build(),
    );
    {
        let editors = deps.editors.clone();
        ctx.register_action_global(
            Action::new("editor.find").on_invoke(move |_i, _c| editors.open_find()),
        );
    }
    // Ctrl+R opens the find banner in replace mode; F3 / Shift+F3 step through matches — the
    // common find-bar chords, all targeting the focused tab.
    ctx.register_shortcut_global(
        Shortcut::new("editor.replace")
            .group(ShortcutGroup::Edit)
            .name(tr!(shortcut_name_editor_replace()))
            .primary(KeyStroke::ctrl(Key::R))
            .build(),
    );
    ctx.register_shortcut_global(
        Shortcut::new("editor.find_next")
            .group(ShortcutGroup::Edit)
            .name(tr!(shortcut_name_editor_find_next()))
            .primary(KeyStroke::new(Key::F3, Modifiers::NONE))
            .build(),
    );
    ctx.register_shortcut_global(
        Shortcut::new("editor.find_prev")
            .group(ShortcutGroup::Edit)
            .name(tr!(shortcut_name_editor_find_prev()))
            .primary(KeyStroke::new(Key::F3, Modifiers::SHIFT))
            .build(),
    );
    {
        let editors = deps.editors.clone();
        ctx.register_action_global(
            Action::new("editor.replace").on_invoke(move |_i, _c| editors.open_find_replace()),
        );
    }
    {
        let editors = deps.editors.clone();
        ctx.register_action_global(
            Action::new("editor.find_next").on_invoke(move |_i, c| editors.find_next(c)),
        );
    }
    {
        let editors = deps.editors.clone();
        ctx.register_action_global(
            Action::new("editor.find_prev").on_invoke(move |_i, c| editors.find_prev(c)),
        );
    }

    // Ctrl+Shift+H reveals the search & replace dock *and* discloses the replace row
    // (Ctrl+Shift+F, the dock alone, is one of the panel commands). Global (resolved
    // before a focused editor), and Shift-qualified so it shadows neither Ctrl+H nor an
    // editor chord. Like `work.open` / `editor.save`, a global action with no `AppIntent`
    // variant.
    ctx.register_shortcut_global(
        Shortcut::new("search.replace")
            .group(ShortcutGroup::Edit)
            .name(tr!(shortcut_name_search_replace()))
            .primary(KeyStroke::new(Key::H, Modifiers::CTRL | Modifiers::SHIFT))
            .build(),
    );
    {
        let docking = deps.outline.docking();
        let search_dock = deps.search_dock;
        let search = deps.search.clone();
        let focus = deps.focus.clone();
        ctx.register_action_global(Action::new("search.replace").on_invoke(move |_i, _c| {
            unless_distraction_free(&focus, || {
                docking.focus_dock(search_dock);
                search.set_show_replace(true);
            });
        }));
    }
    /// The least height the Timeline band can say what it is in.
    ///
    /// The chart wants about 120 dp to be a chart rather than a row of axis labels;
    /// the coverage line, slider, caption and filter row take the rest. Measured
    /// against the real band, not guessed: at 160 dp the chart got 31 dp and drew
    /// nothing.
    const TIMELINE_MIN_HEIGHT: f32 = 300.0;

    // `timeline.show`, the panel command `register_panel_commands` leaves to this
    // block. Its door matters more than most. The band shares the bottom side
    // with the search preview, and the bottom side starts hidden — and hiding a
    // top/bottom side collapses its rail with it, unlike leading and trailing,
    // whose rail survives as the way back. So there is no glyph to click, and
    // without this the band would be built, mounted, and unreachable.
    {
        let docking = deps.outline.docking();
        let timeline_dock = deps.timeline_dock;
        let focus = deps.focus.clone();
        ctx.register_action_global(Action::new("timeline.show").on_invoke(move |_i, _c| {
            unless_distraction_free(&focus, || {
                // The band carries a chart, and a chart needs height. The bottom
                // side's stored size is whatever the search preview was last
                // dragged to — often around 160 dp, which leaves the chart about
                // 30 dp once the coverage line, the slider, the caption and the
                // filter row have taken theirs. That is enough for tilted axis
                // labels and no bars at all.
                //
                // Raised only when it is *below* the floor, and only on the way
                // in: a writer who has already made the side taller keeps their
                // height, and one who drags it back down afterwards keeps that
                // too — the workspace layout persists it either way.
                if docking.side_size(DockSide::Bottom) < TIMELINE_MIN_HEIGHT {
                    docking.set_side_size(DockSide::Bottom, TIMELINE_MIN_HEIGHT);
                }
                docking.focus_dock(timeline_dock);
            });
        }));
    }
}

/// One shortcut and one command per panel, from [`crate::docks::panels::PANELS`]: the
/// door a keyboard reader has to a panel they cannot click on the rail.
///
/// `focus_dock`, never `toggle_dock`: someone reaching for a panel wants it in front of
/// them, and a toggle that hides a panel they cannot see would be the opposite of the
/// point. It is also the recovery door for a panel a saved desk has lost: a desk saved
/// with an inconsistent roster stops mounting a dock, the reconcile reads the absence as
/// "the writer closed this", and a rail-only dock then had no way back short of deleting
/// `workspace.toml`.
///
/// The keyboard focus goes into the panel (where it last was there, else its first
/// control) and a screen reader hears the panel's title. Revealing alone left the
/// focus in the text, and a reader who pressed the chord heard nothing at all.
fn register_panel_commands(ctx: &mut BuildContext, deps: &CommandDeps) {
    for panel in crate::docks::panels::PANELS {
        ctx.register_shortcut_global(
            Shortcut::new(panel.intent)
                .group(ShortcutGroup::View)
                .name((panel.name)())
                .primary(KeyStroke::ctrl_shift(panel.chord))
                .build(),
        );
        // The timeline raises the bottom band before revealing it: its own action is
        // registered in `register`, after this.
        if panel.dock == crate::docks::TIMELINE_DOCK_ID {
            continue;
        }
        let docking = deps.outline.docking();
        let dock = DockWidgetId::from_raw(panel.dock);
        let focus = deps.focus.clone();
        ctx.register_action_global(Action::new(panel.intent).on_invoke(move |_i, _c| {
            unless_distraction_free(&focus, || docking.focus_dock(dock));
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;
    use crate::binder::OutlineViewModel;
    use crate::shared::FocusViewModel;
    use frontend::AppContext;
    use std::rc::Rc;

    /// A dock command fired while the surface is up must leave the desk exactly
    /// as the writer left it.
    ///
    /// Driven through a real `OutlineViewModel`/`DockingModel` rather than a
    /// restatement of the `if`: the bug this guards against was never in the
    /// condition, it was in the assumption that a *disabled* side could not be
    /// rearranged behind your back.
    #[test]
    fn a_dock_command_does_nothing_while_the_surface_is_up() {
        let outline = OutlineViewModel::new_default(Rc::new(AppContext::new()), AppIds::new());
        let focus = FocusViewModel::new();
        let visible_before = outline.is_visible().get();

        focus.active_signal().set(true);
        unless_distraction_free(&focus, || outline.toggle());
        assert_eq!(
            outline.is_visible().get(),
            visible_before,
            "the outline moved while the distraction-free surface was up"
        );

        // …and works normally the moment the writer is back on the desk.
        focus.active_signal().set(false);
        unless_distraction_free(&focus, || outline.toggle());
        assert_ne!(outline.is_visible().get(), visible_before);
    }

    /// Each of the three toggles says what it just did, in both directions. The second
    /// F9 used to say nothing at all: the keyboard was not on the side, so nothing a
    /// screen reader follows had changed.
    #[test]
    fn toggling_a_side_says_which_side_and_which_way() {
        use teksilo::core::NoopWindowOps;
        use teksilo::core::widget_tree::WidgetTree;

        crate::test_support::with_real_messages(|| {
            let docking = DockingModel::new();
            for side in [DockSide::Leading, DockSide::Trailing, DockSide::Bottom] {
                docking.set_side_visible(side, true);
            }
            let mut tree = WidgetTree::new();
            tree.add(teksilo::widgets::Spacer::new());
            tree.set_records_announcements(true);
            let said = |side: DockSide, tree: &mut WidgetTree| {
                let mark = tree.announcements_since(0).last().map_or(0, |a| a.seq);
                let docking = docking.clone();
                tree.run_with_event_context(&mut NoopWindowOps, move |c| {
                    toggle_side(&docking, side, c)
                });
                for _ in 0..2 {
                    tree.layout(SizeProposal::exact(400.0, 300.0));
                    let _ = tree.sync_accessibility();
                }
                tree.announcements_since(mark)
                    .into_iter()
                    .map(|a| a.text)
                    .collect::<Vec<_>>()
            };
            for (side, hidden, shown) in [
                (DockSide::Leading, "Left panel hidden", "Left panel shown"),
                (
                    DockSide::Trailing,
                    "Right panel hidden",
                    "Right panel shown",
                ),
                (
                    DockSide::Bottom,
                    "Bottom panel hidden",
                    "Bottom panel shown",
                ),
            ] {
                assert_eq!(said(side, &mut tree), vec![hidden.to_string()]);
                assert_eq!(said(side, &mut tree), vec![shown.to_string()]);
            }
        });
    }
}
