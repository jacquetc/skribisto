// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Dock and find-banner commands: the outline rail, the bottom preview band, the search &
//! replace dock, the per-editor find banner, this window's plain-fullscreen toggle, and its
//! distraction-free mode toggle.

use bastyde::prelude::*;
use bastyde::widgets::DockSide;

use super::CommandDeps;

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    // F9, not Ctrl+B: Ctrl+B is the editor's built-in bold command, and a Global shortcut is
    // resolved *before* the focused widget sees the raw key — so a Ctrl+B binding here would
    // shadow `RichTextEditor`'s bold.
    ctx.register_shortcut_global(
        Shortcut::new("outline.toggle")
            .name("Toggle Outline")
            .primary(KeyStroke::new(Key::F9, Modifiers::NONE))
            .build(),
    );
    // Phase 0.2 stub — F10 collapses/reveals the BOTTOM band, so the probe can exercise the
    // `visible_when` park/unpark path that dock content takes when its side hides. (F9 only
    // relayouts; it never parks the bottom content.)
    ctx.register_shortcut_global(
        Shortcut::new("preview.toggle")
            .name("Toggle Preview Band")
            .primary(KeyStroke::new(Key::F10, Modifiers::NONE))
            .build(),
    );
    {
        let docking = deps.outline.docking();
        ctx.register_action_global(Action::new("preview.toggle").on_invoke(move |_i, _c| {
            docking.toggle_side_visible(DockSide::Bottom);
        }));
    }
    {
        let outline = deps.outline.clone();
        ctx.register_action_global(
            Action::new("outline.toggle").on_invoke(move |_i, _c| outline.toggle()),
        );
    }

    // F11, the platform convention — free (no other command claims it; see
    // this increment's ground-truth sweep). A Global shortcut so it fires
    // regardless of which widget has focus, same rationale as F9/F10 above.
    ctx.register_shortcut_global(
        Shortcut::new("view.fullscreen")
            .name("Toggle Fullscreen")
            .primary(KeyStroke::new(Key::F11, Modifiers::NONE))
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

    // Shift+F11 — Increment 2 of distraction-free: chrome collapse + docks
    // disabled + fullscreen, together, as one per-window mode independent of
    // the plain F11 toggle above (see `FocusViewModel`'s module doc for why
    // the two never share placement memory). Global for the same reason as
    // F11/F9/F10.
    ctx.register_shortcut_global(
        Shortcut::new("view.focus_mode")
            .name("Toggle Distraction-free Mode")
            .primary(KeyStroke::new(Key::F11, Modifiers::SHIFT))
            .build(),
    );
    {
        let focus = deps.focus.clone();
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
                focus.toggle(window);
            }
        }));
    }

    // Ctrl+F opens the per-editor find banner in the focused pane's active tab (its
    // `FindViewModel`). A *global* shortcut is resolved before the focused widget sees the
    // key — the editor must not eat Ctrl+F — but the action reads which tab is focused, so
    // it targets the right editor even in a split view.
    ctx.register_shortcut_global(
        Shortcut::new("editor.find")
            .name("Find")
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
            .name("Replace")
            .primary(KeyStroke::ctrl(Key::R))
            .build(),
    );
    ctx.register_shortcut_global(
        Shortcut::new("editor.find_next")
            .name("Next Match")
            .primary(KeyStroke::new(Key::F3, Modifiers::NONE))
            .build(),
    );
    ctx.register_shortcut_global(
        Shortcut::new("editor.find_prev")
            .name("Previous Match")
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

    // Ctrl+Shift+F reveals the search & replace dock; Ctrl+Shift+H reveals it *and* discloses
    // the replace row. Global (resolved before a focused editor), and Shift-qualified so
    // neither shadows Ctrl+F (find banner) or an editor chord. Only ever fired by keystroke,
    // so — like `work.open` / `editor.save` — they are global actions with no `AppIntent`
    // variant.
    ctx.register_shortcut_global(
        Shortcut::new("search.show")
            .name("Search in Project")
            .primary(KeyStroke::new(Key::F, Modifiers::CTRL | Modifiers::SHIFT))
            .build(),
    );
    ctx.register_shortcut_global(
        Shortcut::new("search.replace")
            .name("Replace in Project")
            .primary(KeyStroke::new(Key::H, Modifiers::CTRL | Modifiers::SHIFT))
            .build(),
    );
    {
        let docking = deps.outline.docking();
        let search_dock = deps.search_dock;
        ctx.register_action_global(Action::new("search.show").on_invoke(move |_i, _c| {
            docking.reveal_dock(search_dock);
        }));
    }
    {
        let docking = deps.outline.docking();
        let search_dock = deps.search_dock;
        let search = deps.search.clone();
        ctx.register_action_global(Action::new("search.replace").on_invoke(move |_i, _c| {
            docking.reveal_dock(search_dock);
            search.set_show_replace(true);
        }));
    }
}
