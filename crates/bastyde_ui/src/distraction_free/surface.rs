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
//! **A hand-written `Widget`, not `bati!`.** It owns a live `Option<ContentTab>`
//! and swaps it as the writer navigates, which is the same category the house
//! rule already exempts (`DockingLayout`, `TabWidget`, `FormLayout`).
//!
//! **It holds a refcount on the shared document and must give it back itself.**
//! `EditorsViewModel::release_own_open_docs` — the only release that runs on a
//! real window close — walks the two panes' tab lists, so a document only the
//! surface has open is invisible to it. Hence the `Drop` below, and hence the
//! undo-stack id snapshotted at open time rather than read live: by the time a
//! window teardown runs, `AppIds::clear()` has usually already zeroed it.

use bastyde::core::binding::BindingLevel;
use bastyde::prelude::*;
use bastyde::widgets::{Expand, HStack, MaxSize, RectWidget, Spacer, VStack, ZStack};

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

        // Outside the mode the surface holds nothing open, and **builds
        // nothing at all**.
        //
        // Building nothing is what makes it harmless when it is not in use: no
        // refcount pinned on whatever document happened to be focused, no
        // focusable node, nothing painted — and, the part that bit, **no
        // occupied slot to swallow the pointer**. This widget is a sibling of
        // the whole project shell in a `ZStack`, painted on top of it; anything
        // it leaves occupying the window while the mode is off is a full-window
        // click target sitting over the binder, the tabs and the editor. That is
        // exactly what an always-present, always-full-size `Expand` in here did:
        // the shell stayed visible (the content inside was parked) but became
        // completely unclickable.
        //
        // The travel distance the slide needs comes from this same fact rather
        // than from a filler widget: while active the content below reports the
        // full window, which is what `Slide` translates; while inactive there is
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
                        MaxSize::width(width.get() + 2.0 * PAGE_GUTTER)
                            .max_width(width.map(|w| *w + 2.0 * PAGE_GUTTER))
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
                    .child(Spacer::new()),
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
        // everything, with the strip sitting on it as the mode's one gadget row.
        //
        // **Every painted surface here is a `RectWidget`** — `Panel` resolved its
        // background differently and came out carrying the base palette's
        // `surface_main` (#F7F8FA) instead of the theme's margin colour, while the
        // card beside it themed correctly. `RectWidget` resolves its paint against
        // the live theme at paint time, which is what the surface's token override
        // needs.
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
            // body.** This surface is all-or-nothing by construction, and that
            // is a property of the surface, not of whichever of the twelve tab
            // bodies it happens to be showing.
            //
            // Asking the body instead (`ctx.child_size(child, proposal)`) is
            // what broke it: the body is a `ZStack`, and `ZStack` answers with
            // `rigid(max of its children measured at unspecified)` — it ignores
            // the proposal on purpose, so a background `RectWidget` cannot
            // inflate the stack. A `RectWidget` reports 0×0 unconstrained, so
            // the margin and the page contributed nothing and the surface
            // asked for **532×35 of a 1200×800 window** — the strip's own
            // height, with the manuscript squeezed to nothing. The parent's
            // `place_children` still handed the body the surface's full bounds,
            // so the tree looked entirely correct and only the *slot* was wrong.
            //
            // Every test but one mounts this widget as the tree root, which is
            // handed the whole window whatever it answers — which is exactly
            // why this got through them all.
            Some(_) => proposal.resolve(0.0, 0.0).into(),
            // **Rigid zero, not `proposal.resolve(0, 0)`.** Against an exact
            // proposal — which is what a `ZStack` hands every child — `resolve`
            // answers with the proposal, so the "empty" surface still claimed the
            // whole window and went on swallowing every click meant for the shell
            // underneath it. Nothing built means nothing occupied.
            None => LayoutResponse::rigid(Size::ZERO),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;
    use crate::models::{OpenDoc, OpenDocsStore, StatsModel};
    use crate::statusbar::focus_strip::FocusStripChrome;
    use crate::view_models::{
        DistractionFreeThemesViewModel, EditorTypography, EditorTypographySet, EditorsViewModel,
        FocusViewModel, GoToViewModel, SaveStateViewModel, Side, SurfaceDeps, ViewState,
        WritingSessionViewModel,
    };
    use bastyde::core::widget_tree::WidgetTree;
    use frontend::AppContext;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use skribisto_model::counting::CountingMethodSetting;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicU32, Ordering};

    const NORMAL_COLUMN: f32 = 700.0;
    const SURFACE_COLUMN: f32 = 420.0;

    fn temp_store() -> bastyde::settings::SettingsStore {
        static N: AtomicU32 = AtomicU32::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "skribisto_df_surface_test_{}_{n}.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        bastyde::settings::SettingsStore::open(path).expect("open temp settings store")
    }

    fn typography() -> EditorTypographySet {
        let bundle = |family: &str| EditorTypography {
            font_family: Signal::new(family.to_string()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
        };
        EditorTypographySet {
            scene: bundle("Literata"),
            synopsis: bundle("Literata"),
            notes: bundle("Inter"),
            corkboard: bundle("Literata"),
            distraction_free: bundle("Distraction Serif"),
        }
    }

    struct Fixture {
        vm: DistractionFreeSurfaceViewModel,
        focus: FocusViewModel,
        editors: EditorsViewModel,
        docs: OpenDocsStore,
    }

    /// A surface view-model with its dependencies attached and one Scene
    /// document (id 1) seeded into the shared store.
    fn fixture() -> Fixture {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let docs = OpenDocsStore::new(app_ctx.clone());
        let editors = EditorsViewModel::new(
            app_ctx.clone(),
            Signal::new(NORMAL_COLUMN),
            Signal::new(false),
            typography(),
            crate::view_models::TypewriterSettings::off(),
            crate::view_models::CaretHighlightSettings::off(),
            crate::view_models::EditorViewMemory::detached(false),
            crate::view_models::CorkboardDefaults::detached(),
            ids.clone(),
            docs.clone(),
            Signal::new(false),
            SaveStateViewModel::new(app_ctx.clone(), ids.clone()),
            Signal::new(false),
            crate::view_models::TreeExpansionViewModel::new(
                app_ctx.clone(),
                ids.clone(),
                crate::models::TreeExpansionService::in_memory_default(),
            ),
            Signal::new(false),
            Signal::new(SURFACE_COLUMN),
            crate::view_models::GoAvailability::new(),
            crate::view_models::FormatViewModel::detached(),
        );
        let doc = Rc::new(OpenDoc::build(
            &app_ctx,
            1,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            Signal::new(0),
        ));
        // Real prose, not an empty document: a caret is clamped to the
        // character count, so an empty scene would make every caret assertion
        // pass at 0 and prove nothing.
        doc.main
            .as_ref()
            .expect("a Scene has a main prose field")
            .doc
            .cursor_at(0)
            .insert_text("The rain kept on for three days and then it stopped.")
            .unwrap();
        docs.insert_for_test(doc);

        let focus = FocusViewModel::new();
        let vm = DistractionFreeSurfaceViewModel::new(
            app_ctx.clone(),
            focus.clone(),
            GoToViewModel::new(app_ctx.clone(), ids.clone()),
            ids,
        );
        let store = temp_store();
        let stats = StatsModel::new(
            docs.clone(),
            editors.active_item(),
            Signal::new(CountingMethodSetting::default()),
        );
        vm.attach(SurfaceDeps {
            editors: editors.clone(),
            stats: stats.clone(),
            session_vm: WritingSessionViewModel::new(stats, &store),
            has_work: Signal::new(true),
            show_characters: Signal::new(false),
            chrome: FocusStripChrome::all_shown(),
            themes: DistractionFreeThemesViewModel::new(
                crate::models::DistractionFreeThemesService::in_memory_default(),
            ),
            theme_id: Signal::new("paper".to_string()),
            settings: crate::view_models::SettingsViewModel::new(&store),
        });
        Fixture {
            vm,
            focus,
            editors,
            docs,
        }
    }

    fn mount(fx: &Fixture) -> WidgetTree {
        // `tree_with_events`: the strip's `SessionStatusItem` subscribes to
        // backend events, which panics with no event source registered.
        let ctx = Rc::new(AppContext::new());
        let mut tree = crate::test_support::tree_with_events(&ctx);
        tree.add(DistractionFreeSurface::new(fx.vm.clone()));
        tree.layout(SizeProposal::exact(1200.0, 800.0));
        tree
    }

    /// Mount the surface the way the window root does — as a `ZStack` sibling
    /// over a stand-in for the project shell — and return the surface's node.
    ///
    /// A tree *root* is always allocated the full window whatever it asks for,
    /// so mounting the surface bare cannot show whether it claims a slot. Only
    /// a parent that honours a child's answer can.
    fn mount_over_shell(fx: &Fixture) -> (WidgetTree, WidgetId) {
        let ctx = Rc::new(AppContext::new());
        let mut tree = crate::test_support::tree_with_events(&ctx);
        tree.add(
            bastyde::widgets::ZStack::new()
                .child(bastyde::widgets::Expand::new())
                .child(DistractionFreeSurface::new(fx.vm.clone())),
        );
        tree.layout(SizeProposal::exact(1200.0, 800.0));
        fn find(tree: &WidgetTree, id: WidgetId) -> Option<WidgetId> {
            if tree
                .widget_type_name(id)
                .is_some_and(|n| n.ends_with("DistractionFreeSurface"))
            {
                return Some(id);
            }
            tree.children(id).into_iter().find_map(|c| find(tree, c))
        }
        let id = find(&tree, tree.roots()[0]).expect("the surface is in the tree");
        (tree, id)
    }

    /// The widths of the writing columns the writer can actually *see*.
    ///
    /// Height-filtered on purpose: a scene's tag-dot row is centred on the tab's
    /// normal column and collapses to zero height when the scene is untagged
    /// (which most are), so a width-only walk reports a 700px "column" that is
    /// not on screen at all.
    fn column_caps(tree: &WidgetTree) -> Vec<f32> {
        fn walk(tree: &WidgetTree, id: WidgetId, out: &mut Vec<f32>) {
            if tree
                .widget_type_name(id)
                .is_some_and(|n| n.ends_with("MaxSize"))
            {
                let b = tree.bounds(id);
                if b.width > 0.0 && b.height > 0.0 {
                    out.push(b.width);
                }
            }
            for c in tree.children(id) {
                walk(tree, c, out);
            }
        }
        let mut out = Vec::new();
        for r in tree.roots() {
            walk(tree, r, &mut out);
        }
        out
    }

    /// Outside the mode the surface holds **nothing** open. A dormant widget is
    /// still a built widget, so a surface that opened the focused document
    /// eagerly would pin a refcount — and rob that document of its
    /// flush-on-evict — for every session in which the writer never once pressed
    /// Shift+F11.
    #[test]
    fn the_surface_holds_no_document_while_the_mode_is_off() {
        let fx = fixture();
        fx.editors.active_item().set(Some(1));
        let before = fx.docs.refs_for_test(1);
        let _tree = mount(&fx);
        assert_eq!(fx.docs.refs_for_test(1), before);
    }

    /// **The surface occupies nothing while the mode is off.**
    ///
    /// It is a `ZStack` sibling of the whole project shell, painted on top of
    /// it, so any slot it leaves behind is a full-window click target lying over
    /// the binder, the tabs and the editor — with nothing drawn in it to explain
    /// why the app stopped responding. That shipped once: an always-full-size
    /// `Expand` between the slide and the visibility gate (added to give the
    /// slide something to travel) made the entire main view unclickable while
    /// looking completely normal.
    ///
    /// Zero size is the whole assertion. A widget that occupies nothing cannot
    /// take a click meant for what is underneath it.
    #[test]
    fn the_surface_occupies_nothing_while_the_mode_is_off() {
        let fx = fixture();
        fx.editors.active_item().set(Some(1));
        let (mut tree, surface) = mount_over_shell(&fx);
        let b = tree.bounds(surface);
        assert_eq!(
            (b.width, b.height),
            (0.0, 0.0),
            "the surface holds a {}x{} slot over the shell while the mode is off",
            b.width,
            b.height
        );

        // …and it does take the window once the mode is on, or there would be
        // nothing for `Slide` to translate and nothing to write in.
        fx.focus.active_signal().set(true);
        tree.layout(SizeProposal::exact(1200.0, 800.0));
        let b = tree.bounds(surface);
        assert!(
            b.width > 0.0 && b.height > 0.0,
            "the surface is empty while the mode is on ({b:?})"
        );
    }

    /// **The whole window, not merely a non-zero slot.**
    ///
    /// The sibling test above asserts the surface occupies *nothing* while the
    /// mode is off, and was for a while the only one that mounted it under a
    /// parent at all — every other test uses the surface as the tree root,
    /// which is handed the full window whatever it answers. So "greater than
    /// zero" was the only claim being made about the active case, and a body
    /// that reported its *intrinsic* size passed it while rendering as a small
    /// island of content on a blank window.
    #[test]
    fn the_surface_fills_the_window_while_the_mode_is_on() {
        let fx = fixture();
        fx.editors.active_item().set(Some(1));
        fx.focus.active_signal().set(true);
        let (tree, surface) = mount_over_shell(&fx);
        let b = tree.bounds(surface);
        assert_eq!(
            (b.width, b.height),
            (1200.0, 800.0),
            "the surface took a {}x{} slot out of a 1200x800 window — the \
             margin, the page and the prose have nowhere to paint",
            b.width,
            b.height
        );
    }

    /// **The assertion this whole feature exists for.** Entering the mode must
    /// put the writer's document on the mode's own column, not the docked
    /// editor's — which is exactly what the in-place chrome collapse never did,
    /// because the typography resolved once at build time and the pane was
    /// memoized.
    #[test]
    fn entering_the_mode_mounts_the_document_on_the_distraction_free_column() {
        let fx = fixture();
        fx.editors.active_item().set(Some(1));
        fx.focus.active_signal().set(true);
        let tree = mount(&fx);

        let caps = column_caps(&tree);
        assert!(
            caps.iter().any(|w| (*w - SURFACE_COLUMN).abs() < 0.5),
            "the surface did not use the distraction-free column. Widths seen: {caps:?}"
        );
        assert!(
            !caps.iter().any(|w| (*w - NORMAL_COLUMN).abs() < 0.5),
            "the surface used the docked editor's column. Widths seen: {caps:?}"
        );
    }

    /// The refcount the surface takes is its own to give back — and it must give
    /// it back on the way out of the mode, not only when the window dies.
    /// `EditorsViewModel::release_own_open_docs` walks the two panes' tab lists,
    /// so it can never see this reference.
    #[test]
    fn leaving_the_mode_releases_the_document() {
        let fx = fixture();
        fx.editors.active_item().set(Some(1));
        let before = fx.docs.refs_for_test(1).expect("seeded");

        fx.focus.active_signal().set(true);
        let mut tree = mount(&fx);
        assert_eq!(
            fx.docs.refs_for_test(1),
            Some(before + 1),
            "entering the mode must take a reference"
        );

        fx.focus.active_signal().set(false);
        tree.layout(SizeProposal::exact(1200.0, 800.0));
        assert_eq!(
            fx.docs.refs_for_test(1),
            Some(before),
            "leaving the mode must give it straight back"
        );
    }

    /// Dropping the surface with a document still open releases it too — the
    /// window-close path, where no toggle ever runs.
    #[test]
    fn dropping_the_surface_releases_what_it_still_holds() {
        let fx = fixture();
        fx.editors.active_item().set(Some(1));
        let before = fx.docs.refs_for_test(1).expect("seeded");
        fx.focus.active_signal().set(true);

        let tree = mount(&fx);
        assert_eq!(fx.docs.refs_for_test(1), Some(before + 1));
        drop(tree);
        assert_eq!(fx.docs.refs_for_test(1), Some(before));
    }

    /// **The handoff.** A writer who presses Shift+F11 mid-sentence must carry
    /// on mid-sentence, and come back out of the mode where the surface left
    /// them — not at the top of the scene either way.
    ///
    /// Both directions matter and they use different machinery: entering
    /// *seeds* a tab that has not built yet, leaving *applies* onto a pane whose
    /// editor is dormant but still mounted.
    #[test]
    fn the_caret_survives_entering_and_leaving_the_mode() {
        let fx = fixture();
        // A pane tab on the same document, with some prose and a caret in it.
        fx.editors.open_in(Side::Primary, 1, "Scene");
        let pane_state = ViewState {
            caret: 12,
            scroll: 0.0,
        };
        fx.editors.apply_view_state(1, pane_state);
        fx.editors.seed_view_state(Side::Primary, 1, pane_state);
        fx.editors.active_item().set(Some(1));

        // Enter: the surface's own tab opens at the pane's caret.
        fx.focus.active_signal().set(true);
        let mut tree = mount(&fx);
        let surface_caret = fx
            .vm
            .open_tab(1)
            .map(|(t, _)| t.view_state().get().caret)
            .expect("the surface can open the document");
        assert_eq!(
            surface_caret, 12,
            "the surface did not pick up the pane's caret"
        );

        // Leave: whatever the surface ends on goes back to the pane.
        fx.focus.active_signal().set(false);
        tree.layout(SizeProposal::exact(1200.0, 800.0));
        assert_eq!(
            fx.editors.view_state_of(1).map(|s| s.caret),
            Some(12),
            "leaving the mode lost the caret"
        );
    }

    /// The manuscript floats on a **card**, narrower than the surface, so the
    /// theme's general-background axis has an area of its own.
    ///
    /// Before this the tab body painted its own page edge to edge and the margin
    /// colour had nowhere to show — it existed in the data and surfaced only by
    /// accident, through the idle fill of `Plain` buttons on the strip.
    #[test]
    fn a_prose_page_floats_as_a_card_narrower_than_the_surface() {
        let fx = fixture();
        fx.editors.active_item().set(Some(1));
        fx.focus.active_signal().set(true);
        // Mounted **under a parent**, not as the tree root: a root is handed the
        // whole window whatever it asks for, so measuring the card there proves
        // only that the widget exists, never that it was given room to paint.
        let (tree, _surface) = mount_over_shell(&fx);

        fn collect(tree: &WidgetTree, id: WidgetId, suffix: &str, out: &mut Vec<Rect>) {
            if tree
                .widget_type_name(id)
                .is_some_and(|n| n.ends_with(suffix))
            {
                let b = tree.bounds(id);
                if b.width > 0.0 && b.height > 0.0 {
                    out.push(b);
                }
            }
            for c in tree.children(id) {
                collect(tree, c, suffix, out);
            }
        }
        let find = |suffix: &str| {
            let mut out = Vec::new();
            for r in tree.roots() {
                collect(&tree, r, suffix, &mut out);
            }
            out
        };
        let seen = find("RectWidget");

        // The margin: the theme's general background, edge to edge behind
        // everything. Without it the axis has no area and shows up only by
        // accident, through the idle fill of the strip's controls.
        assert!(
            seen.iter()
                .any(|b| (b.width - 1200.0).abs() < 1.0 && (b.height - 800.0).abs() < 1.0),
            "no full-window margin behind the page. Rects: {seen:?}"
        );

        // The page: the writing column plus its gutters, and genuinely narrower
        // than the window or there is no margin left to see.
        let want_w = SURFACE_COLUMN + 2.0 * PAGE_GUTTER;
        let card = seen
            .iter()
            .find(|b| (b.width - want_w).abs() < 1.0)
            .copied()
            .unwrap_or_else(|| {
                panic!("no page card at the writing column's width + gutters ({want_w} px). Rects: {seen:?}")
            });
        assert!(
            card.width < 1200.0,
            "the page card is as wide as the surface — there is no margin left \
             for the theme's background to show in"
        );

        // …and it runs from the top of the window down to the control strip.
        // Pinned to the strip rather than to the window height: the page sits
        // *above* the strip rather than behind it, so hard-coding 800 here would
        // encode the strip's current height as a constant of the layout.
        let strip = find("StatusBar")
            .first()
            .copied()
            .expect("the control strip is on the surface");
        // The 2px slack is the hairline `Divider` the strip carries above its
        // `StatusBar` — the page meets the strip, it does not overlap it.
        let gap = strip.y - (card.y + card.height);
        assert!(
            card.y.abs() < 1.0 && (0.0..=2.0).contains(&gap),
            "the page card spans y {}..{} but the strip starts at {} — a page has \
             to run the full height or it reads as a floating band",
            card.y,
            card.y + card.height,
            strip.y
        );
    }

    /// A tab with no writing column gets **no** card: a narrow strip of page
    /// behind a full-width board or table would read as a rendering fault.
    #[test]
    fn only_a_prose_tab_floats_on_a_card() {
        let fx = fixture();
        fx.editors.active_item().set(Some(1));
        let tab = fx.vm.open_tab(1).expect("prose tab").0;
        assert!(tab.floats_on_a_page(), "a Scene writes on paper");
        assert_eq!(
            tab.backdrop_role(),
            bastyde::tokens::SurfaceRole::Transparent,
            "…so its body must not paint its own page over the card"
        );
    }

    /// A **chapter folder** carries scene prose — `prose_kind_for` maps
    /// `(Folder, ChapterScene)` to `Some(Scene)`, which is what gives it the
    /// right typography — but it renders through `folder_segmented`: a
    /// full-width segment bar over a `Switcher` holding a corkboard, an overview
    /// table and two manuscript streams. Floating *that* on a card the width of
    /// one writing column reads as a rendering fault.
    ///
    /// So the card cannot key off `kind` alone, tempting as that is. This is the
    /// test that says so.
    #[test]
    fn a_container_never_floats_however_much_prose_it_carries() {
        let fx = fixture();
        let app_ctx = Rc::new(AppContext::new());
        let doc = Rc::new(OpenDoc::build(
            &app_ctx,
            2,
            &BinderItemRole::Folder,
            &BinderItemSubRole::ChapterScene,
            &[],
            Signal::new(0),
        ));
        assert!(
            doc.kind.is_some(),
            "a chapter folder does carry scene prose — otherwise this test is \
             asserting nothing"
        );
        fx.docs.insert_for_test(doc);

        let tab = fx.vm.open_tab(2).expect("the container opens").0;
        assert!(
            !tab.floats_on_a_page(),
            "a segmented container was floated on a writing-column-wide card"
        );
        assert_eq!(
            tab.backdrop_role(),
            bastyde::tokens::SurfaceRole::Content,
            "…so it must keep painting its own full-bleed page"
        );
    }

    /// The theme reaches the surface **as tokens**, and the four axes stay
    /// separate — the whole reason a coloured theme is legal under a
    /// semantic-roles-only rule.
    ///
    /// Applied at the window root (`shell::windows`) rather than here, so this
    /// asserts the half that lives in this module's reach: that the view-model
    /// resolves a paintable theme and that the four colours are genuinely
    /// distinct roles rather than one colour wearing four names.
    #[test]
    fn the_surface_resolves_four_distinct_theme_axes() {
        let fx = fixture();
        let t = fx.vm.theme().expect("a theme once attached");
        let axes = [
            &t.general_background,
            &t.editor_background,
            &t.editor_text,
            &t.widget_text,
        ];
        for (i, a) in axes.iter().enumerate() {
            assert!(!a.is_empty(), "axis {i} has no colour");
        }
        assert_ne!(
            t.editor_background, t.editor_text,
            "paper and ink must differ, or there is nothing to read"
        );
        assert_ne!(
            t.editor_background, t.general_background,
            "the page and what is behind it must differ, or the theme has three axes"
        );
        assert!(
            t.meets_contrast(),
            "the default theme must be legible: prose {:.2}:1",
            t.prose_contrast()
        );
    }

    /// Switching theme is answered live, without re-entering the mode.
    #[test]
    fn changing_the_theme_id_changes_the_resolved_theme() {
        let fx = fixture();
        let before = fx.vm.theme().unwrap().id;
        let (theme_id, _) = fx.vm.theme_signals().expect("attached");
        theme_id.set("night".to_string());
        let after = fx.vm.theme().unwrap();
        assert_ne!(after.id, before);
        assert_eq!(after.id, "night");
    }

    /// A stored id naming a theme that no longer exists must still paint.
    #[test]
    fn an_unknown_theme_id_still_resolves() {
        let fx = fixture();
        let (theme_id, _) = fx.vm.theme_signals().expect("attached");
        theme_id.set("a-theme-from-another-machine".to_string());
        let t = fx.vm.theme().expect("never None once attached");
        assert!(t.builtin, "the fallback must be a shipped theme");
    }

    /// Exit is reachable even with no project open. Being unable to leave a
    /// chromeless full-screen window is the one failure this surface is shaped
    /// around, so the strip renders whether or not there is a document.
    #[test]
    fn the_strip_is_present_with_nothing_open() {
        let fx = fixture();
        fx.focus.active_signal().set(true);
        let mut tree = mount(&fx);
        let exit = tr!(statusbar_focus_exit()).resolve_now();
        let names: Vec<String> = tree
            .sync_accessibility()
            .nodes
            .iter()
            .filter_map(|(_, n)| n.label().map(|s| s.to_string()))
            .collect();
        assert!(
            names.contains(&exit),
            "no way out of the mode with nothing open. Labels seen: {names:?}"
        );
    }
}
