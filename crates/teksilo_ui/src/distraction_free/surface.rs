// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The distraction-free **surface**: one document, the control strip, and
//! nothing else.
//!
//! It renders through [`crate::tabs::tab_pane`] — the same dispatch the editor
//! panes use — so all twelve `(role, sub_role)` combinations work here,
//! including the segmented container tabs, and there is no second renderer to
//! drift out of step with `skribisto_model::COMBINATIONS`.
//!
//! **A hand-written `Widget`, not `teksu!`.** It owns a live `Option<ContentTab>`
//! and swaps it as the writer navigates, which is the same category the house
//! rule already exempts (`DockingLayout`, `TabWidget`, `FormLayout`).
//!
//! **It holds a refcount on the shared document and must give it back itself.**
//! `EditorsViewModel::release_own_open_docs` — the only release that runs on a
//! real window close — walks the two panes' tab lists, so a document only the
//! surface has open is invisible to it. Hence the `Drop` below, and hence the
//! undo-stack id snapshotted at open time rather than read live: by the time a
//! window teardown runs, `AppIds::clear()` has usually already zeroed it.

use teksilo::core::binding::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{Expand, FixedSize, HStack, MaxSize, RectWidget, Spacer, VStack, ZStack};

use crate::tabs::{Boxed, ContentTab, tab_pane};
use crate::view_models::DistractionFreeSurfaceViewModel;

/// Breathing room between the writing column and the edge of the page it floats
/// on, so the text is not flush against the paper's edge.
const PAGE_GUTTER: f32 = 56.0;

/// The document the surface currently has open, and what it needs to let go of.
struct Mounted {
    item_id: u64,
    /// Snapshotted when the tab was opened — see this module's doc.
    stack: Option<u64>,
    tab: ContentTab,
    /// Whether this document's editor has already been given keyboard focus.
    /// Guards `run_after_mount`, which is a *per-enqueue* one-shot: the surface
    /// rebuilds for reasons that are not a new document (the settings the strip
    /// reads, for one), and stealing focus back on each of those would fight the
    /// writer if they had clicked into the strip.
    focused: bool,
}

pub struct DistractionFreeSurface {
    vm: DistractionFreeSurfaceViewModel,
    mounted: Option<Mounted>,
    root_child: Option<WidgetId>,
}

impl DistractionFreeSurface {
    pub fn new(vm: DistractionFreeSurfaceViewModel) -> Self {
        Self {
            vm,
            mounted: None,
            root_child: None,
        }
    }

    /// Point the surface at `want`, opening and releasing documents as needed.
    /// The id check first is what keeps an incidental rebuild from tearing down
    /// and re-taking the same reference — and, more to the point, from throwing
    /// away the caret while it does.
    fn sync_mounted(&mut self, want: Option<u64>) {
        if self.mounted.as_ref().map(|m| m.item_id) == want {
            return;
        }
        self.let_go();
        if let Some(id) = want
            && let Some((tab, stack)) = self.vm.open_tab(id)
        {
            self.mounted = Some(Mounted {
                item_id: id,
                stack,
                tab,
                focused: false,
            });
        }
    }

    /// Release the mounted document, **handing its caret and scroll back to the
    /// pane underneath first**.
    ///
    /// The pane's own editor is dormant, not destroyed, so it takes the position
    /// in place with no rebuild — which is what makes leaving the mode land the
    /// writer exactly where they were writing rather than at the top of the
    /// scene.
    ///
    /// The band the mode painted comes off with the editors that drew it —
    /// `TypographyBoundEditor`'s `Drop`, not anything here. It has to be there
    /// rather than here: the state carrying the band can be a *stale* one this
    /// surface never had a handle on, and by the time the mode is off its band
    /// effects are gone, so nothing reachable from this side can retire it.
    fn let_go(&mut self) {
        if let Some(old) = self.mounted.take() {
            self.vm.hand_back(old.item_id, old.tab.capture_view_state());
            self.vm.release_tab(old.item_id, old.stack);
        }
    }
}

impl Drop for DistractionFreeSurface {
    fn drop(&mut self) {
        self.let_go();
    }
}

impl std::fmt::Debug for DistractionFreeSurface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DistractionFreeSurface").finish()
    }
}

impl Widget for DistractionFreeSurface {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let self_id = ctx.self_id();
        self.vm.wire(ctx);
        // Three reasons to rebuild, all structural:
        //   * the mode turned on or off — it is what decides whether a document
        //     is open at all, so an inactive surface holds no refcount;
        //   * the focused pane's active tab changed — Go Previous/Next, "Go
        //     to…" and Ctrl+Tab all move it, and the surface follows;
        //   * `App::build` handed over its dependencies (see the view-model).
        // Bound, never observed: two of the three are plain mutable signals but
        // binding is what a structural change wants, and it costs nothing while
        // the surface is dormant (a parked node is filtered out of the rebuild
        // walk).
        let active = self.vm.active_signal();
        active.bind_to(self_id, ctx.binding_registry(), BindingLevel::Rebuild);
        self.vm
            .desired_item()
            .bind_to(self_id, ctx.binding_registry(), BindingLevel::Rebuild);
        self.vm
            .revision()
            .bind_to(self_id, ctx.binding_registry(), BindingLevel::Rebuild);
        // A theme change is a **repaint** of this subtree, not a rebuild: the
        // colours live in a token override installed at the window root
        // (`shell::windows`), whose closure is consulted afresh on every
        // resolve. All this has to do is make sure a resolve happens — which is
        // why `SubtreeRepaint` rather than `RepaintOnly`: the page, the prose and
        // the strip are all descendants, and only the marked subtree is
        // re-resolved.
        if let Some((theme_id, library)) = self.vm.theme_signals() {
            theme_id.bind_to(
                self_id,
                ctx.binding_registry(),
                BindingLevel::SubtreeRepaint,
            );
            library.bind_to(
                self_id,
                ctx.binding_registry(),
                BindingLevel::SubtreeRepaint,
            );
        }

        // Outside the mode the surface builds **nothing at all**: no refcount
        // pinned on whatever document happened to be focused, no focusable
        // node, nothing painted — and no occupied slot to swallow the
        // pointer. This widget is a `ZStack` sibling of the whole project
        // shell, painted on top of it, so anything it leaves occupying the
        // window while the mode is off is a full-window click target over
        // the binder, the tabs and the editor.
        //
        // The slide's travel distance comes from this same fact rather than a
        // filler widget: while active the content below reports the full
        // window, which is what `Slide` translates; while inactive there is
        // no child, so the slot collapses to nothing.
        let want = if active.get() {
            self.vm.desired_item().get()
        } else {
            None
        };
        self.sync_mounted(want);
        if !active.get() {
            self.root_child = None;
            return Vec::new();
        }

        let page_card = self.mounted.as_ref().and_then(|m| {
            m.tab
                .floats_on_a_page()
                .then(|| m.tab.main_column_width().clone())
        });
        // How much wider than the manuscript the card has to be to seat the
        // synopsis column beside it — zero unless one is actually showing.
        let side = self
            .mounted
            .as_ref()
            .map(|m| m.tab.side_pane_extent())
            .unwrap_or_else(|| Signal::new(0.0));
        let mut manuscript: Option<Box<dyn Widget>> = None;
        if let Some(m) = &self.mounted {
            manuscript = Some(tab_pane(&m.tab));

            // Two things that can only happen once the pane below has actually
            // built, and so cannot be done inline here.
            let ports = m.tab.view_state_ports();

            // 1. Restore the page scroll. `ScrollArea` clamps any offset to its
            //    maximum, and that maximum is 0 until the content has been laid
            //    out — so a scroll written at build time is silently dropped.
            //    Waiting on the maximum instead is the only way to land it.
            //    One-shot: `pending` is cleared on the first application, so a
            //    later reflow (a wider window, an edit) never yanks the writer
            //    back to where they came in.
            let pending = m.tab.view_state().get();
            if pending.scroll > 0.0
                && let Some(max) = ports.max_scroll()
            {
                let ports_for_scroll = ports.clone();
                let done = std::cell::Cell::new(false);
                ctx.effect(&max, move |m: &f32| {
                    if !done.get() && *m > 0.0 {
                        done.set(true);
                        ports_for_scroll.apply_scroll(pending.scroll);
                    }
                });
            }

            // 2. Take keyboard focus, so the writer can just carry on typing.
            //    Also what re-points the Format menu and dock: their target is a
            //    sticky latch keyed by `WidgetId`, released only on a real
            //    `Drop`, so the pane's editor — dormant, not destroyed — would
            //    otherwise stay the target and the formatting surfaces would
            //    show the frozen state of an editor that is not on screen.
            if !m.focused {
                if let Some(m) = self.mounted.as_mut() {
                    m.focused = true;
                }
                ctx.run_after_mount(move |ctx| {
                    if let Some(handle) = ports.editor() {
                        handle.focus(ctx);
                    }
                });
            }
        }

        // The manuscript layer, and — for a prose tab — the page it sits on.
        //
        // A tab normally paints its own `SurfaceRole::Content` edge to edge, and
        // on this surface that left the theme's *general background* with no area
        // of its own — the axis existed in the data and showed up nowhere, except
        // by accident through control fills.
        //
        // The tab's own backdrop goes `Transparent` to make room; one predicate,
        // `ContentTab::floats_on_a_page`, decides that *and* whether a card is
        // drawn here, so the two can never disagree and a card can never appear
        // behind a body still painting its own page.
        let manuscript: Box<dyn Widget> = match (page_card, manuscript) {
            // **The body is confined to the card, not merely backed by it.** A
            // pane laid out at the full window width keeps centring its writing
            // column, so the prose looked right — but a section's caption and
            // rule ("Synopsis", "Text") are left-aligned to the *pane*, so they
            // stretched across the whole window and stranded the captions out on
            // the margin, detached from the page they label. Narrowing the pane
            // to the card puts the whole composite — captions, rules, the find
            // banner, the scroll bar — on the paper, in exactly the relationship
            // they already have in the docked editor.
            (Some(width), Some(pane)) => Box::new(
                HStack::new()
                    .child(Spacer::new())
                    .child(
                        MaxSize::width(width.get() + 2.0 * PAGE_GUTTER + side.get())
                            .max_width(
                                width
                                    .zip(&side)
                                    .map(|(w, side)| *w + 2.0 * PAGE_GUTTER + *side),
                            )
                            // `Expand` between the cap and the stack on purpose:
                            // `ZStack` answers with `rigid(max of its children
                            // measured unconstrained)`, so without it the card
                            // would be as tall as the prose rather than the page.
                            .child(
                                Expand::new().child(
                                    ZStack::new()
                                        .child(RectWidget::new().background(SurfaceRole::Content))
                                        .child(Boxed::new(pane)),
                                ),
                            ),
                    )
                    .child(Spacer::new())
                    // **The manuscript must not move when the synopsis appears.**
                    // Two equal spacers centre the card, so a card that grew on its
                    // left by the width of the synopsis column would slide the prose
                    // right by half of it — re-centring the page under the writer's
                    // cursor every time they glanced at their synopsis. Taking the
                    // same amount back off the trailing side cancels that exactly:
                    // the manuscript half of the card stays where it was, and the
                    // synopsis grows into the margin beside it.
                    .child(FixedSize::new().width(side.clone())),
            ),
            // A corkboard, an overview table or a segmented container has no
            // column to float — a narrow card behind a full-width board would
            // read as a rendering fault — so it keeps its full-bleed page and
            // paints its own background.
            (None, Some(pane)) => pane,
            // No project, or nothing open. The strip still renders below, so Exit
            // is reachable — being unable to leave a chromeless full-screen
            // window is the one failure this whole surface is shaped around.
            (_, None) => Box::new(Expand::new()),
        };

        let mut col = VStack::new()
            .spacing(0.0)
            .child(Expand::new().child(Boxed::new(manuscript)));
        if let Some(strip) = self.vm.strip() {
            col = col.child(strip);
        }

        // The margin: the theme's general background, edge to edge behind
        // everything, with the strip sitting on it as the mode's one gadget
        // row. Every painted surface here is a `RectWidget`, not a `Panel` —
        // it resolves its paint against the live theme at paint time, which
        // the surface's token override needs.
        let body: Box<dyn Widget> = Box::new(
            ZStack::new()
                .child(RectWidget::new().background(SurfaceRole::Main))
                .child(col),
        );

        // Escape leaves the mode. A widget-level key handler, not a global
        // shortcut: globals are resolved *before* the focused widget sees the
        // key, so a global Escape would fire underneath whatever the editor just
        // did with the same keypress. Raw keys bubble from the focused widget up
        // through its ancestors, so this only ever sees an Escape nothing more
        // local already claimed.
        //
        // It lives here rather than on the project shell's root because that
        // subtree is **dormant** while the surface is up, and a dormant widget
        // receives no events at all — an Escape handler left there would be dead
        // code pretending to be a way out.
        let vm = self.vm.clone();
        self.root_child = Some(ctx.add(Boxed::new(body).on_key(move |ev, ctx| match ev {
            WidgetEvent::KeyDown {
                key: Key::Escape, ..
            } => {
                if let Some(window) = ctx.window() {
                    vm.exit(window);
                }
                EventResponse::Handled
            }
            _ => EventResponse::Ignored,
        })));
        self.root_child.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        match self.root_child {
            // **The whole window, taken from the proposal — never from the
            // body.** This surface is all-or-nothing by construction,
            // regardless of which of the twelve tab bodies it shows.
            // Delegating to the body (`ctx.child_size`) is wrong: the body is
            // a `ZStack`, which answers `rigid(max of its children measured
            // unconstrained)` — a background `RectWidget` reports 0×0 and
            // can't inflate it, so the surface would ask for far less than
            // the window (the strip's own height, with nothing else counted).
            Some(_) => proposal.resolve(0.0, 0.0).into(),
            // **Rigid zero, not `proposal.resolve(0, 0)`.** Against an exact
            // proposal — what a `ZStack` hands every child — `resolve` echoes
            // the proposal back, so an "empty" surface would still claim the
            // whole window and swallow every click meant for the shell below.
            None => LayoutResponse::rigid(Size::ZERO),
        }
    }
}

#[cfg(test)]
mod tests;
