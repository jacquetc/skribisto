// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Layer-B view-model setup: the window title/OS-title sync, the editors
//! view-model (created once and re-pointed every build), its Format-menu
//! attachment and the per-work workspace-layout hookup.
//!
//! Extracted from `App::build` so that function is not also the place these
//! six handles are minted.

use std::rc::Rc;

use teksilo::prelude::*;

use crate::sessions::WorkSession;
use crate::view_models::{
    EditorsViewModel, SaveStateViewModel, SettingsViewModel, WorkspaceLayoutViewModel,
};

use super::App;

/// Handles [`App::build_layer_b_view_models`] mints, for the rest of `App::build` to share.
pub(super) struct LayerBViewModels {
    pub settings: SettingsViewModel,
    pub window_id: Option<TeksiloWindowId>,
    pub session: WorkSession,
    pub save_state: SaveStateViewModel,
    pub editors: EditorsViewModel,
    pub workspace_layout: Option<WorkspaceLayoutViewModel>,
}

impl App {
    /// Layer-B view-models: created once, then shared by clone.
    pub(super) fn build_layer_b_view_models(&mut self, ctx: &mut BuildContext) -> LayerBViewModels {
        let settings = SettingsViewModel::new(ctx.settings());

        // This window's own id, read once per build — `None` only in a
        // headless/off-screen build context (never a real project window; see
        // the backup flush-hook registration below, this crate's first consumer
        // of `ctx.window()`). Used both there and by the window-teardown
        // registration further down (`WorkRegistry::register_window`), which is
        // what teksilo's `on_removed` hook (wired in `shell::windows`) looks
        // this same window up by once it is confirmed gone.
        let window_id = ctx.window().map(|w| w.id());

        // Scope D — window titles. Push every change of this window's live
        // title text (`Self::title_text`, already bound to the drawn custom
        // title bar in `shell::windows`) to the OS-level title too, via
        // `WindowState::title()` — its own doc: an app-side `.set()` here
        // round-trips through the window manager into a real OS window-title
        // call, the half a KWin rule (matching a window by its title text)
        // actually needs. A no-op in a headless/off-screen build context
        // (`ctx.window()` is `None` there — same guard as `window_id` above).

        // The Tier-2 per-open-Work bundle — see `sessions::WorkSession`'s module
        // doc and this struct's own field doc for why `App::build` reads these
        // straight off `session` instead of doing its own `ctx.app_state::<T>()`
        // lookup per field, the way the rest of this function used to.
        let session = self.session.clone();

        if let Some(window) = ctx.window() {
            let os_title = window.title().clone();
            // Observe the two MUTABLE sources, never `self.title_text` itself:
            // that is `single_work.title().zip(ordinal).map(..)`
            // (`shell::windows::window_title_text`), and a zip/map signal is
            // lazy and read-only — `ctx.effect` observes, and `observe()`
            // panics on a derived signal. Reading its value inside the closure
            // is fine; only observing it is not. Both arms recompute the whole
            // title, so either source changing pushes the same correct text.
            let title_text = self.title_text.clone();
            let title = session.single_work.title();
            {
                let os_title = os_title.clone();
                let title_text = title_text.clone();
                ctx.effect(&title, move |_: &String| {
                    os_title.set(title_text.get());
                });
            }
            ctx.effect(&self.window_ordinal, move |_: &usize| {
                os_title.set(title_text.get());
            });
        }
        let app_ctx = self.app_ctx.clone();
        let column_width = settings.column_width();
        let show_synopsis = settings.synopsis_pane();
        let synopsis_placement = settings.synopsis_placement();
        let synopsis_side_width = settings.synopsis_side_width();
        let typography = settings.editor_typography();
        // Typewriter scrolling: the two source signals, straight from the store,
        // so a settings change reaches every open tab's editors and its page's
        // scroll range together.
        let typewriter = crate::view_models::TypewriterSettings::new(
            settings.typewriter(),
            settings.typewriter_anchor(),
        );
        // The caret band. Its colour is a *resolved* theme colour, because it crosses into the
        // document as a `HighlightFormat` field rather than staying a paintable role — the same
        // trip the spell-check squiggle colour makes — so the bundle owns a source signal kept
        // current by a theme effect. Built through the shared constructor, which the Search &
        // Replace preview dock also uses; duplicating it here is how one of the two ends up not
        // following a light/dark switch.
        let caret_highlight = crate::view_models::CaretHighlightSettings::from_context(ctx);
        let view_memory = crate::view_models::EditorViewMemory::new(ctx.settings());
        let corkboard_defaults = settings.corkboard_defaults();
        let ids = self.outline.ids();
        let docs = session.open_docs.clone();
        // Work-scoped, not per-window: created once in `main` (inside the
        // `WorkSession` bundle) and shared by every window onto this project —
        // see `SaveStateViewModel`'s module docs for why a per-window copy of
        // `dirty_seq`/`saved_seq`/`saving`/the `SaveQueue` is a bug the moment a
        // second window exists. Unlike every other Tier-2 field, this one is
        // *not* also reachable via `ctx.app_state::<SaveStateViewModel>()` —
        // `main.rs` deliberately stopped registering it there once `App` itself
        // started carrying the session (see `main.rs`'s comment at its
        // construction site).
        let save_state = session.save_state.clone();
        let backup_mode_for_editors = self.backup_mode.clone();
        let save_state_for_editors = save_state.clone();
        let scene_focused_for_editors = self.scene_focused.clone();
        // A **pane** tab is never distraction-free — a constant `false`, not
        // `self.focus.active_signal()`. `TabWidget` memoizes its panes and
        // `DockingLayout` preserves its centre across rebuilds, so a signal here
        // would never actually be re-read once a pane is built: a scene opened
        // before entering the mode would keep its normal typeface/column for the
        // whole session. The mode instead mounts its own surface with its own
        // tab, whose flag is a constant `true` (`EditorsViewModel::open_surface_tab`).
        let distraction_free_for_editors = Signal::new(false);
        let distraction_free_width = settings.distraction_free_width();
        let go_for_editors = self.go.clone();
        let format_for_editors = self.format.clone();
        // The writing games: this project's session-only activation (Tier 2, off
        // `WorkSession`, so a second window on the same Work agrees and a second
        // open project does not) paired with the two app-global "which surfaces"
        // settings. Assembled here because this is the one place both halves are
        // in hand — the same shape as the typewriter and caret-band bundles above.
        let writing_games_for_editors = crate::view_models::WritingGamesViewModel::new(
            session.always_forward.clone(),
            crate::view_models::WritingGameOptions::new(
                settings.games_forward_prose(),
                settings.games_forward_synopsis(),
            ),
        );
        let editors = self
            .editors
            .get_or_insert_with(|| {
                EditorsViewModel::new(
                    app_ctx,
                    column_width,
                    show_synopsis,
                    synopsis_placement,
                    synopsis_side_width,
                    typography,
                    typewriter,
                    caret_highlight,
                    view_memory,
                    corkboard_defaults,
                    ids,
                    docs,
                    backup_mode_for_editors,
                    save_state_for_editors,
                    scene_focused_for_editors,
                    session.tree_expansion.clone(),
                    distraction_free_for_editors,
                    distraction_free_width,
                    go_for_editors,
                    format_for_editors,
                    writing_games_for_editors,
                    session.single_work.goal_unit(),
                )
            })
            .clone();

        // This window's own format VM (minted in the factory with the menu bar).
        // Re-pointed at the editors here, on every build — idempotent.
        let format = self.format.clone();
        {
            let target = editors.clone();
            format.attach(Rc::new(move || {
                use crate::view_models::FormatSurface;
                // One walk answers both halves, at deliberately different
                // strictnesses.
                //
                // The *target* is sticky: it stays the tab's editor whether or
                // not it holds focus this instant. Opening the Format menu moves
                // focus to the menu overlay, so a target gated on live focus
                // would vanish exactly when the user reached for a command.
                //
                // The *surface* is live: click into the binder and there is
                // genuinely nothing to format, so the dock drops to its empty
                // state rather than offering controls for a caret that is no
                // longer anywhere. (The dock's own buttons are
                // `focusable(false)`, so pressing one never blurs the editor out
                // from under itself.)
                let Some((handle, is_synopsis, focused)) = target.format_target() else {
                    return (None, FormatSurface::None);
                };
                if !focused {
                    return (Some(handle), FormatSurface::None);
                }
                if is_synopsis {
                    return (Some(handle), FormatSurface::Synopsis);
                }
                // The same predicate the compiler uses to decide what it scans,
                // so the dock cannot offer a scene break where the exporter
                // would ignore one.
                let surface = if target.focused_carries_scene() {
                    FormatSurface::Scene
                } else {
                    FormatSurface::Note
                };
                (Some(handle), surface)
            }));
        }

        // Pull the focused editor's formatting into the mirrors once per frame,
        // here rather than in the dock: the Format menu binds the same signals,
        // and the dock is only one of two trailing rail tabs — driven from
        // there, the menu's checkmarks would freeze whenever the user switched
        // the rail to the Inspector.
        //
        // Deliberately not an effect on the editor's `format_version`: that
        // signal is written from inside the editor's own `state.borrow_mut()`
        // and observers fire synchronously there, so reading the state back
        // would panic on an already-borrowed cell. A frame tick fires outside
        // any borrow, and `refresh` short-circuits when nothing has moved.
        {
            let format = format.clone();
            let tick = ctx.frame_tick();
            ctx.effect(&tick, move |_| format.refresh());
        }

        // Hand the editors to the per-work workspace-layout restore. It was created
        // in `main` (inside the `WorkSession` bundle, before any `ctx.settings()`),
        // so it starts editor-less and is wired here, on every build — idempotent
        // (`set_editors` just re-points). Kept as a local `Option` (rather than
        // `session.workspace_layout` directly) so the Load/New subscribers below,
        // which pre-date this field always being present, don't need reshaping.
        //
        // **`None` for an attached window** — see [`WindowRole::owns_desk`].
        let workspace_layout = self
            .role
            .owns_desk()
            .then(|| session.workspace_layout.clone());
        if let Some(layout) = &workspace_layout {
            layout.set_editors(editors.clone());
            // Same idempotent re-point, for `capture_tree_expansion`'s own use of
            // this window's outline (Scope C fix — see that method's doc).
            layout.set_outline(self.outline.clone());
        }

        LayerBViewModels {
            settings,
            window_id,
            session,
            save_state,
            editors,
            workspace_layout,
        }
    }
}
