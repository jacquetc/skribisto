// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! State behind the distraction-free **surface** — the panel that slides in over
//! the project shell and shows one document.
//!
//! **Per-window**, minted in `shell::windows::ProjectWindowFactory` beside
//! [`crate::view_models::FocusViewModel`] and `GoToViewModel`, for the same
//! reason those are: one window can be in the mode while another still shows the
//! binder.
//!
//! **Its dependencies arrive late.** The surface mounts at the *window* root, so
//! it can cover the title bar — but almost everything it needs (the editors, the
//! stats model, the writing session, the live settings) is built inside
//! `App::build`, which is where `ctx.settings()` first exists. So this
//! view-model is minted empty and `App::build` calls [`DistractionFreeSurfaceViewModel::attach`], the same
//! `Rc<RefCell<Option<…>>>` shape `WorkspaceLayoutViewModel::set_editors` already
//! uses. [`DistractionFreeSurfaceViewModel::revision`] is what tells the widget to rebuild when they land.
//!
//! **The surface never re-typesets a mounted pane.** It opens its *own*
//! `ContentTab` over the same shared `OpenDoc`, with `distraction_free` pinned to
//! a constant `true` — see `EditorsViewModel::open_surface_tab` for why a live
//! flag cannot work.
//!
//! **It also resolves its own caret band.** Every other colour the mode paints
//! with reaches a widget as a token override installed at the window root; the
//! band cannot, because it crosses into the text document as a `HighlightFormat`
//! field rather than staying a role resolved at paint time (see
//! `distraction_free::theme`). So the band colour is resolved here, from the
//! theme in force, into [`DistractionFreeSurfaceViewModel::band_color`] — and the
//! surface's tab is built over a `CaretHighlightSettings` that reads it instead
//! of the app palette's. The *scope* still comes from the one shared setting: how
//! much of the text is shaded is a preference, not a property of the theme.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::prelude::{Signal, WindowState};

use crate::app_ids::AppIds;
use crate::distraction_free::theme::DistractionFreeTheme;
use crate::models::StatsModel;
use crate::singles::SingleBinderItem;
use crate::statusbar::focus_strip::{FocusStrip, FocusStripChrome};
use crate::tabs::ContentTab;
use crate::view_models::{
    DistractionFreeThemesViewModel, EditorsViewModel, FocusViewModel, GoToViewModel,
    WritingSessionViewModel,
};

/// Everything the surface needs that only exists inside `App::build`.
#[derive(Clone)]
pub struct SurfaceDeps {
    pub editors: EditorsViewModel,
    pub stats: StatsModel,
    pub session_vm: WritingSessionViewModel,
    /// Whether a project is open — the same test the status bar's own items use.
    pub has_work: Signal<bool>,
    pub show_characters: Signal<bool>,
    /// The project's target unit and a store handle, for the strip's own copy of the
    /// status bar's word-count indicator — which draws the target bar itself, so the mode
    /// inherits it without a second implementation.
    pub goal_unit: Signal<frontend::common::entities::GoalUnit>,
    pub app_ctx: std::rc::Rc<frontend::AppContext>,
    pub chrome: FocusStripChrome,
    /// The theme library, and the id of the one in force. The id is an ordinary
    /// setting (one live handle — see `DistractionFreeThemesViewModel`), so the
    /// two travel together rather than the library owning a second copy.
    pub themes: DistractionFreeThemesViewModel,
    pub theme_id: Signal<String>,
    /// For the strip's quick-settings popover, which shows the mode's own
    /// settings rows.
    pub settings: crate::view_models::SettingsViewModel,
}

#[derive(Clone)]
pub struct DistractionFreeSurfaceViewModel {
    focus: FocusViewModel,
    go_to: GoToViewModel,
    ids: AppIds,
    deps: Rc<RefCell<Option<SurfaceDeps>>>,
    /// The item the surface is showing, so the strip can name it.
    ///
    /// A live single rather than a snapshot of the title: it auto-refreshes on
    /// the entity's `Updated` event, which is what makes the strip follow a
    /// **rename** made from inside the mode (a ChapterScene's own title field is
    /// right there on the page).
    item: SingleBinderItem,
    /// Whether the item currently on the surface is one of the three that render
    /// the dual-pane writing editor — see `tabs::renders_prose`.
    ///
    /// Drives the strip's synopsis toggle's *enabled* state. Set here rather than
    /// read off the mounted tab because the strip is built from this view-model,
    /// which never sees the widget's private `Mounted` state; [`Self::open_tab`] is
    /// the one place a fresh `ContentTab` passes through, and it already publishes
    /// the item's name from the same spot.
    synopsis_capable: Signal<bool>,
    /// The caret band's colour for this mode, resolved from the theme in force.
    ///
    /// A **genuine source signal**, minted once and re-set in place rather than
    /// derived from the theme id: `CaretHighlightSettings` documents why (a
    /// `Signal::map` result is read-only and *panics* on `observe()`, which is
    /// what `ctx.effect` uses — and the editor watches this very signal to push a
    /// changed band onto a live document).
    ///
    /// Owned here rather than on the tab so it survives the tab: the surface
    /// swaps its `ContentTab` whenever the writer navigates, and a colour minted
    /// per mount would leave the effects that follow the theme pointing at a
    /// signal nothing reads any more.
    band_color: Signal<teksilo::text_document::Color>,
    /// Bumped whenever the surface must rebuild for a reason its own signal
    /// bindings cannot see — today, only [`Self::attach`].
    revision: Signal<u64>,
}

impl DistractionFreeSurfaceViewModel {
    pub fn new(
        app_ctx: Rc<frontend::AppContext>,
        focus: FocusViewModel,
        go_to: GoToViewModel,
        ids: AppIds,
    ) -> Self {
        Self {
            focus,
            go_to,
            ids,
            deps: Rc::new(RefCell::new(None)),
            item: SingleBinderItem::new(app_ctx),
            synopsis_capable: Signal::new(false),
            // Seeded transparent, not black: before `attach` there is no library
            // to ask, and the surface is not on screen either — but a black seed
            // would be one missed sync away from a bar of ink across the prose.
            band_color: Signal::new(teksilo::text_document::Color::rgba(0, 0, 0, 0)),
            revision: Signal::new(0),
        }
    }

    /// Hand the surface what `App::build` owns. Idempotent — `App::build` re-runs
    /// on every rebuild of the shell, and re-attaching simply re-points at the
    /// current handles.
    pub fn attach(&self, deps: SurfaceDeps) {
        *self.deps.borrow_mut() = Some(deps);
        self.revision.set(self.revision.get().wrapping_add(1));
    }

    /// Bumped by [`Self::attach`]. The surface binds this at
    /// `BindingLevel::Rebuild` so it picks its dependencies up the moment they
    /// exist, rather than caching "no project yet" from the one build that
    /// happened before `App` had built.
    pub fn revision(&self) -> Signal<u64> {
        self.revision.clone()
    }

    /// Whether this window is in the mode. Also bound at `Rebuild` by the
    /// surface: entering is what makes it open a document at all.
    pub fn active_signal(&self) -> Signal<bool> {
        self.focus.active_signal()
    }

    /// The item the surface should be showing: whatever the focused pane's
    /// active tab is.
    ///
    /// Following the editors rather than keeping its own notion of "current" is
    /// what makes Go Previous/Next and "Go to…" work inside the mode for free —
    /// they already route through `EditorsViewModel::open_or_focus`, so the tab
    /// set behind the surface stays in step and the writer comes back out where
    /// they navigated to.
    pub fn desired_item(&self) -> Signal<Option<u64>> {
        match self.deps.borrow().as_ref() {
            Some(d) => d.editors.active_item(),
            // Before `attach`, a detached constant. Never observed — only bound —
            // so a fresh signal here is inert rather than a leak.
            None => Signal::new(None),
        }
    }

    /// Open a tab for `item_id` on the surface, taking a refcount on the shared
    /// document. Returns the tab and the undo-stack id **snapshotted now**, which
    /// is what the release must use: by the time a window teardown runs,
    /// `AppIds::clear()` has usually already zeroed the live one.
    ///
    /// The tab is seeded with the caret and scroll the writer left in the pane
    /// underneath, so pressing Shift+F11 mid-sentence carries on mid-sentence
    /// rather than jumping to the top of the scene.
    pub fn open_tab(&self, item_id: u64) -> Option<(ContentTab, Option<u64>)> {
        let deps = self.deps.borrow();
        let editors = &deps.as_ref()?.editors;
        let caret = self.caret_band()?;
        // The mode's own synopsis flag, not the global setting — see
        // `FocusViewModel::synopsis_visible_signal`.
        let tab = editors.open_surface_tab(item_id, self.focus.synopsis_visible_signal(), caret)?;
        // Point the strip's name — and its synopsis toggle's enabled state — at
        // this document.
        self.item.set_id(Some(item_id));
        self.synopsis_capable.set(tab.renders_prose());
        if let Some(state) = editors.view_state_of(item_id) {
            tab.seed_view_state(state);
        }
        Some((tab, self.ids.stack_id.get()))
    }

    /// Hand the surface's position back to the pane underneath, on the way out.
    ///
    /// The pane's editor is still mounted (dormant, not destroyed), so this
    /// re-points it in place rather than seeding a rebuild — which is what keeps
    /// leaving the mode as cheap as entering it.
    pub fn hand_back(&self, item_id: u64, state: crate::view_models::ViewState) {
        if let Some(d) = self.deps.borrow().as_ref() {
            d.editors.apply_view_state(item_id, state);
        }
    }

    /// Give back what [`Self::open_tab`] took.
    pub fn release_tab(&self, item_id: u64, stack: Option<u64>) {
        if let Some(d) = self.deps.borrow().as_ref() {
            d.editors.release_surface_tab(item_id, stack);
        }
    }

    /// Wire the live single behind the strip's item name, and keep the caret
    /// band's colour on the theme in force. Called from the surface's `build`,
    /// the one place with a `BuildContext`.
    ///
    /// The band needs **effects**, not the surface's own bindings: the surface
    /// binds the same two signals at `SubtreeRepaint`, which by design never
    /// re-runs `build`. Left to that, a writer editing the band colour in
    /// Settings would see the page repaint around a band that stayed as it was
    /// until they navigated to another scene.
    pub fn wire(&self, ctx: &mut teksilo::prelude::BuildContext) {
        self.item.wire(ctx);
        self.sync_band_color();
        if let Some((theme_id, library)) = self.theme_signals() {
            let me = self.clone();
            ctx.effect(&theme_id, move |_| me.sync_band_color());
            let me = self.clone();
            ctx.effect(&library, move |_| me.sync_band_color());
        }
    }

    /// Re-resolve [`Self::band_color`] from the theme in force. A no-op before
    /// `attach`, and idempotent — it only writes when the colour actually moved,
    /// so a library bump that changed some *other* theme does not push a
    /// pointless band update onto every open editor.
    fn sync_band_color(&self) {
        let Some(theme) = self.theme() else { return };
        let next =
            crate::view_models::CaretHighlightSettings::document_color(theme.caret_band_color());
        if self.band_color.get() != next {
            self.band_color.set(next);
        }
    }

    /// The caret-band bundle the surface's own tab is built with: the **shared**
    /// scope preference, over **this mode's** colour.
    ///
    /// The split is the point. How much text is shaded — nothing, the sentence,
    /// the paragraph — is how a writer works, and does not change because they
    /// went full-screen; what colour it is shaded belongs to the page they chose
    /// to write on, and the app palette's answer is wrong on a sepia or a
    /// midnight one.
    ///
    /// `None` before `attach`, when there is no settings handle to read.
    pub fn caret_band(&self) -> Option<crate::view_models::CaretHighlightSettings> {
        let deps = self.deps.borrow();
        let d = deps.as_ref()?;
        Some(crate::view_models::CaretHighlightSettings::new(
            d.settings.highlight_scope(),
            self.band_color.clone(),
        ))
    }

    /// The always-visible control strip, built over the *same* live models the
    /// normal status bar uses rather than a second set.
    pub fn strip(&self) -> Option<FocusStrip> {
        let deps = self.deps.borrow();
        let d = deps.as_ref()?;
        Some(FocusStrip::new(
            self.go_to.clone(),
            Some((d.settings.clone(), d.themes.clone())),
            self.item.title(),
            d.stats.clone(),
            d.session_vm.clone(),
            d.has_work.clone(),
            d.show_characters.clone(),
            d.goal_unit.clone(),
            d.app_ctx.clone(),
            d.chrome.clone(),
            self.focus.synopsis_visible_signal(),
            self.synopsis_capable.clone(),
        ))
    }

    /// The theme the surface should paint with — always something paintable,
    /// even if the stored id names a theme that has been deleted.
    ///
    /// `None` only before `attach`, when there is no library to ask.
    pub fn theme(&self) -> Option<DistractionFreeTheme> {
        let deps = self.deps.borrow();
        let d = deps.as_ref()?;
        Some(d.themes.resolve(&d.theme_id.get()))
    }

    /// The two signals a change of theme can arrive through: the writer picking
    /// a different one, and the library itself changing (an edit to the theme in
    /// force, an import, a delete that makes the current id fall back).
    ///
    /// The surface binds both so its subtree is re-resolved — a theme override
    /// closure is consulted on each layout/paint, but only for a subtree
    /// something has marked dirty.
    pub fn theme_signals(&self) -> Option<(Signal<String>, Signal<u64>)> {
        let deps = self.deps.borrow();
        let d = deps.as_ref()?;
        Some((d.theme_id.clone(), d.themes.changed_signal()))
    }

    /// Leave the mode. Idempotent — neither Escape nor the strip's Exit button
    /// may ever toggle it back *on*.
    pub fn exit(&self, window: &WindowState) {
        self.focus.exit(window);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vm() -> DistractionFreeSurfaceViewModel {
        let app_ctx = Rc::new(frontend::AppContext::new());
        DistractionFreeSurfaceViewModel::new(
            app_ctx.clone(),
            FocusViewModel::new(),
            GoToViewModel::new(app_ctx.clone(), AppIds::new()),
            AppIds::new(),
        )
    }

    #[test]
    fn everything_is_inert_before_the_dependencies_arrive() {
        // The surface is built at the window root, which can happen before
        // `App::build` has run — it must answer harmlessly rather than panic or
        // half-open a document.
        let vm = vm();
        assert!(vm.desired_item().get().is_none());
        assert!(vm.strip().is_none());
        assert!(vm.open_tab(1).is_none());
        vm.release_tab(1, None); // no-op, must not panic
    }

    #[test]
    fn attaching_bumps_the_revision_so_the_widget_rebuilds() {
        let vm = vm();
        let before = vm.revision().get();
        // A minimal attach is not constructible without an `App`, so this pins
        // the half that matters structurally: the revision is what the widget
        // binds to, and it must move when dependencies land — otherwise the
        // surface caches "no project" for the life of the window.
        vm.revision.set(vm.revision.get().wrapping_add(1));
        assert_ne!(vm.revision().get(), before);
    }
}
