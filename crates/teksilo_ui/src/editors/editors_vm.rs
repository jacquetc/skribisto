// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `EditorsViewModel` — the split editor: two panes of open tabs (a primary and a
//! secondary/side pane) over the shared [`OpenDocsStore`].
//!
//! Single-instance live state: owns the two panes' `ListModel`s + selection
//! signals, the split state, and the horizontal `SplitterModel`; `App` creates
//! exactly one and shares it by clone. All document ownership + write-back lives
//! in the store, so opening the same item in both panes yields two tabs over one
//! live document.

use std::rc::Rc;

use skribisto_model::SubRoleExt;
use skribisto_model::scene_break::{self, SceneBreakTier};
use teksilo::data::ListModel;
use teksilo::prelude::*; // Signal, tr!, lit!
use teksilo::widgets::{Orientation, PaneDescriptor, SplitterModel, TabHandle, TabId, TabInfo};

use frontend::AppContext;
use frontend::common::entities::GoalUnit;
use frontend::direct_access::BinderItemDto;

use frontend::common::event::{Event, Origin};

use crate::app_ids::AppIds;
use crate::models::{OpenDoc, OpenDocsStore};
use crate::singles::SingleBinderItem;
use crate::tabs::ContentTab;

use crate::go::GoAvailability;
use crate::save::{SaveLanded, SaveStateViewModel};
use crate::settings::EditorTypographySet;
use crate::shared::binder_ops;

/// Which editor pane. `Primary` is always present; `Secondary` is the side pane,
/// revealed by the split view.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    Primary,
    Secondary,
}

impl Side {
    /// The other pane. Used wherever "look at the focused side first, then the
    /// other one" is the right search order — the same item can be open in both.
    pub fn other(self) -> Self {
        match self {
            Side::Primary => Side::Secondary,
            Side::Secondary => Side::Primary,
        }
    }
}

/// One editor pane: its dynamic tab model + selection.
#[derive(Clone)]
struct Pane {
    tabs: ListModel<TabHandle>,
    selected: Signal<Option<TabId>>,
}

impl Pane {
    fn new() -> Self {
        Self {
            tabs: ListModel::from_vec(Vec::new()),
            selected: Signal::new(None),
        }
    }
}

/// Minimum width of a pane, so the splitter can't crush an editor to nothing.
const PANE_MIN_WIDTH: f32 = 320.0;

#[derive(Clone)]
pub struct EditorsViewModel {
    app_ctx: Rc<AppContext>,
    primary: Pane,
    secondary: Pane,
    /// `true` when the side pane is shown. Drives the split button visual, the
    /// splitter's pane-1 visibility, and the primary drop-target's side zone.
    split_active: Signal<bool>,
    /// The horizontal splitter behind the two panes; pane 1 starts hidden.
    splitter: SplitterModel,
    /// Which pane's selection feeds `active_item` (the binder's open-item marker).
    focused_side: Signal<Side>,
    /// The `BinderItem` of the focused pane's active tab — the "open document".
    active_item: Signal<Option<u64>>,
    /// The same item, resolved to what the extension seam publishes: durable
    /// `uid` plus `role`/`sub_role` alongside the store id. Kept beside
    /// `active_item` (rather than derived from it on demand) because
    /// `crate::active_context::ActiveContext` bridges it by *observation*, and a
    /// derived signal cannot be observed.
    active_ctx: Signal<Option<crate::active_context::ActiveItem>>,
    /// Whether the focused pane's active tab edits a scene's own prose — the
    /// live form of [`Self::focused_carries_scene`], for menu enablement. Kept
    /// as a signal (not a derived map) because it is computed by walking the
    /// tab list, which is not itself in the signal graph.
    scene_focused: Signal<bool>,
    /// Live "is there a target" mirrors for the six Go-menu rows — recomputed
    /// alongside `scene_focused` in [`Self::sync_active_item`]. See
    /// [`GoAvailability`]'s own doc for why this is threaded in rather than owned:
    /// `shell/windows.rs` builds the Go menu before this view-model exists.
    go: GoAvailability,
    column_width: Signal<f32>,
    show_synopsis: Signal<bool>,
    /// Synopsis placement (above vs beside the manuscript), shared live from
    /// Settings into every `ContentTab`.
    synopsis_placement: Signal<crate::shared::SynopsisPlacement>,
    /// Width of the Side synopsis column, shared live from Settings into every
    /// `ContentTab`, which seeds its own divider from it.
    synopsis_side_width: Signal<f32>,
    typography: EditorTypographySet,
    /// Typewriter scrolling, shared live from Settings into every `ContentTab`.
    typewriter: crate::shared::TypewriterSettings,
    /// The ambient caret band, shared live from Settings into every `ContentTab`.
    caret_highlight: crate::shared::CaretHighlightSettings,
    /// The flag every **pane** tab is built with — a constant `false`, supplied
    /// by `App::build`. A pane is never distraction-free: the mode mounts its
    /// own surface with its own tab (see [`Self::open_surface_tab`]) rather than
    /// re-typesetting a mounted pane, which cannot be made to work — the
    /// typography resolves once at build time and panes are memoized.
    ///
    /// Kept as a field rather than inlined because it is one of
    /// `ContentTab::new`'s arguments and [`Self::make_tab`] is the single place
    /// those are assembled.
    distraction_free: Signal<bool>,
    /// The distraction-free writing column's own width (Settings ▸ Editor),
    /// threaded into every `ContentTab` for `ContentTab::main_column_width()`.
    /// Only the surface's tab ever reads it, but it is live: dragging the slider
    /// while the mode is open reaches the surface without re-entering it.
    distraction_free_width: Signal<f32>,
    /// Per-container-type "last view" memory, threaded into every `ContentTab`.
    view_memory: crate::settings::EditorViewMemory,
    /// Corkboard default presentation, threaded into every container `ContentTab`.
    corkboard_defaults: crate::settings::CorkboardDefaults,
    /// Id-only global state (work + undo-stack ids); write-back lands on
    /// `ids.stack_id` so it shares the tree edits' Ctrl+Z history.
    ids: AppIds,
    /// The shared holder of open documents (app-state clone), refcounted per item.
    docs: OpenDocsStore,
    /// `true` while a *backup file* is open: disk saves are inert (the file is
    /// read-only; the content stays editable and can only be kept via Save As).
    backup_mode: Signal<bool>,
    /// The **Work**-scoped save-tracking state (`dirty_seq`/`saved_seq`/`saving` +
    /// the `SaveQueue`) — shared with every other window onto this project, not
    /// owned here. See [`SaveStateViewModel`]'s module docs for why a per-window
    /// copy of any of this is a bug the moment a second window exists.
    save_state: SaveStateViewModel,
    /// Threaded into every `ContentTab` (its Overview pane's remembered chevron
    /// expansion) — see `overview::OverviewViewModel::restore_expansion`'s
    /// doc for why this is a constructor-threaded handle, not an
    /// `OverviewViewModel`-local `ctx.app_state` lookup.
    tree_expansion: crate::settings::TreeExpansionViewModel,
    /// This window's Format surfaces — threaded into every `ContentTab` so
    /// editors register with the right registry (never process-wide app_state).
    format: crate::format::FormatViewModel,
    /// The writing games this project is playing (per-`Work` activation +
    /// app-global options), handed to every tab this view-model builds.
    writing_games: crate::writing_session::WritingGamesViewModel,
    /// The open project's target unit (Tier 2, its `WorkSession`'s `SingleWork`), threaded
    /// into every `ContentTab` for the same reason as the handles above: a second window on
    /// a second project must not read the first one's answer.
    goal_unit: Signal<GoalUnit>,
}

impl EditorsViewModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        column_width: Signal<f32>,
        show_synopsis: Signal<bool>,
        synopsis_placement: Signal<crate::shared::SynopsisPlacement>,
        synopsis_side_width: Signal<f32>,
        typography: EditorTypographySet,
        typewriter: crate::shared::TypewriterSettings,
        caret_highlight: crate::shared::CaretHighlightSettings,
        view_memory: crate::settings::EditorViewMemory,
        corkboard_defaults: crate::settings::CorkboardDefaults,
        ids: AppIds,
        docs: OpenDocsStore,
        backup_mode: Signal<bool>,
        save_state: SaveStateViewModel,
        // Shared with the title-bar's Format menu — see `scene_focused_signal`.
        scene_focused: Signal<bool>,
        tree_expansion: crate::settings::TreeExpansionViewModel,
        distraction_free: Signal<bool>,
        distraction_free_width: Signal<f32>,
        // Shared with the title-bar's Go menu — see `GoAvailability`'s own doc.
        go: GoAvailability,
        format: crate::format::FormatViewModel,
        writing_games: crate::writing_session::WritingGamesViewModel,
        goal_unit: Signal<GoalUnit>,
    ) -> Self {
        // Two equal panes; the side pane starts hidden (no divider) until split.
        // The Splitter sums *every* pane's `min_size` into its own intrinsic
        // minimum regardless of visibility, so the hidden side pane starts at
        // min_size 0 (raised to PANE_MIN_WIDTH only while shown, in `set_split`) —
        // otherwise it would inflate the editor area's minimum width when unsplit.
        let splitter = SplitterModel::from_panes(
            vec![
                PaneDescriptor::new().stretch(1.0).min_size(PANE_MIN_WIDTH),
                PaneDescriptor::new()
                    .stretch(1.0)
                    .min_size(0.0)
                    .visible(false),
            ],
            Orientation::Horizontal,
        );
        Self {
            app_ctx,
            primary: Pane::new(),
            secondary: Pane::new(),
            split_active: Signal::new(false),
            splitter,
            focused_side: Signal::new(Side::Primary),
            active_item: Signal::new(None),
            active_ctx: Signal::new(None),
            scene_focused,
            go,
            format,
            writing_games,
            column_width,
            show_synopsis,
            synopsis_placement,
            synopsis_side_width,
            typography,
            typewriter,
            caret_highlight,
            distraction_free,
            distraction_free_width,
            view_memory,
            corkboard_defaults,
            ids,
            docs,
            backup_mode,
            save_state,
            tree_expansion,
            goal_unit,
        }
    }

    // ── View handles ────────────────────────────────────────────────────────

    /// The "an edit happened" signal — bind the debounced autosave to it.
    pub fn edited_signal(&self) -> Signal<u64> {
        self.docs.edited_any()
    }

    /// The dynamic-tab model for a pane's `TabWidget::dynamic_model`.
    pub fn tabs(&self, side: Side) -> ListModel<TabHandle> {
        self.pane(side).tabs.clone()
    }

    /// The selection signal for a pane's `TabWidget::new`.
    pub fn selected(&self, side: Side) -> Signal<Option<TabId>> {
        self.pane(side).selected.clone()
    }

    /// Whether the side pane is shown (drives the split button + side drop zone).
    pub fn split_active(&self) -> Signal<bool> {
        self.split_active.clone()
    }

    /// The splitter behind the two panes (hand to `Splitter::new`).
    pub fn splitter(&self) -> SplitterModel {
        self.splitter.clone()
    }

    /// The currently-open item id (focused pane's active tab). Bind a binder row's
    /// "open document" accent to this.
    pub fn active_item(&self) -> Signal<Option<u64>> {
        self.active_item.clone()
    }

    /// The same item with its durable `uid` and its `(role, sub_role)` — the raw
    /// material [`crate::active_context::ActiveContext`] publishes to a dock.
    /// In-crate only; the seam sees the read-only projection.
    pub(crate) fn active_context(&self) -> Signal<Option<crate::active_context::ActiveItem>> {
        self.active_ctx.clone()
    }

    /// Which pane has focus, live. In-crate only, for the same reason.
    pub(crate) fn focused_side_signal(&self) -> Signal<Side> {
        self.focused_side.clone()
    }

    // ── Focus / active item ─────────────────────────────────────────────────

    /// Mark `side` as the focused pane and refresh the open-item marker. Called
    /// from `App`'s per-pane selection effects.
    pub fn set_focused(&self, side: Side) {
        self.focused_side.set(side);
        self.sync_active_item();
    }

    /// Recompute `active_item` from the focused pane's selected tab.
    pub fn sync_active_item(&self) {
        let side = self.focused_side.get();
        let active = self
            .pane(side)
            .selected
            .get()
            .and_then(|tab| self.item_of_tab(side, tab));
        if self.active_item.get() != active {
            self.active_item.set(active);
        }
        // The seam's richer view of the same item. Resolved here rather than
        // derived on demand so it is a mutable signal an observer can bridge, and
        // recomputed unconditionally (not only when `active` changed) because
        // `promote_uc` retypes an item **in place**: the id is unmoved, the
        // `sub_role` is not. `binder_ops::item_dto` is the same read
        // `sync_go_targets` below already pays on every focus change.
        let ctx_item = active.and_then(|id| {
            binder_ops::item_dto(&self.app_ctx, id).map(|dto| crate::active_context::ActiveItem {
                id,
                uid: dto.uid,
                role: dto.role,
                sub_role: dto.sub_role,
            })
        });
        if self.active_ctx.get() != ctx_item {
            self.active_ctx.set(ctx_item);
        }
        let carries = self.focused_carries_scene();
        if self.scene_focused.get() != carries {
            self.scene_focused.set(carries);
        }
        self.sync_go_targets();
    }

    /// Recompute the six Go-menu row mirrors from `active_item`'s position in its own
    /// binder — one backend read (`binder_ops::go_targets`) answers all six. `None`
    /// (nothing focused) mirrors every row as unavailable, exactly like a located item
    /// with nothing in a given direction.
    fn sync_go_targets(&self) {
        use skribisto_model::{GoDirection::*, GoKind::*};
        let targets = self
            .active_item
            .get()
            .map(|id| binder_ops::go_targets(&self.app_ctx, &self.ids, id))
            .unwrap_or_default();
        self.go.set(Scene, Next, targets.next_scene.is_some());
        self.go.set(Scene, Previous, targets.prev_scene.is_some());
        self.go.set(Chapter, Next, targets.next_chapter.is_some());
        self.go
            .set(Chapter, Previous, targets.prev_chapter.is_some());
        self.go.set(Note, Next, targets.next_note.is_some());
        self.go.set(Note, Previous, targets.prev_note.is_some());
    }

    /// Jump to the Next/Previous item of `kind` from the focused pane's active tab,
    /// through [`Self::open_or_focus`] — the exact same path the
    /// `AppIntent::OpenItem` handler calls, **never** a bypassing direct
    /// `OpenDocsStore::open` — so autosave, dirty tracking and `StatsModel::active_item`
    /// all see the jump like any ordinary binder navigation. A safe no-op when nothing
    /// is focused, or nothing of `kind` lies in that direction (no wraparound).
    pub fn go(&self, kind: skribisto_model::GoKind, direction: skribisto_model::GoDirection) {
        let Some(focused) = self.active_item.get() else {
            return;
        };
        let targets = binder_ops::go_targets(&self.app_ctx, &self.ids, focused);
        let Some(target) = targets.get(kind, direction) else {
            return;
        };
        let title = binder_ops::item_dto(&self.app_ctx, target)
            .map(|it| it.title)
            .unwrap_or_default();
        self.open_or_focus(target, &title);
    }

    /// The `GoKind` of the focused pane's active tab, if it is a writing tab at all —
    /// what the generic `go.next`/`go.prev` pair (the distraction-free strip's own
    /// Next/Previous buttons) resolves at dispatch time to pick which kind-specific
    /// answer to delegate to. `None` for a non-writing tab (a folder container, a
    /// heading, a placeholder) — the pair is then a no-op, same as any Go row with
    /// nothing to jump to.
    pub fn focused_go_kind(&self) -> Option<skribisto_model::GoKind> {
        self.with_focused_tab(|t| skribisto_model::go_kind_of(t.role(), t.sub_role()))
    }

    /// Open the find banner (Ctrl+F) in the **focused** pane's active tab, if that
    /// tab has a prose surface to search. A no-op on a heading / folder / empty
    /// tab. The banner's own state lives on the tab's `FindViewModel`, so a split
    /// view's two editors keep independent finds.
    pub fn open_find(&self) {
        if let Some(find) = self.focused_find() {
            find.open();
        }
    }

    /// Open the find banner in **replace** mode in the focused tab (Ctrl+R).
    pub fn open_find_replace(&self) {
        if let Some(find) = self.focused_find() {
            find.open_with_replace();
        }
    }

    /// Step to the next / previous match in the focused tab's find (F3 / Shift+F3).
    /// A no-op when no find banner is open there.
    pub fn find_next(&self, ctx: &mut teksilo::prelude::EventContext) {
        if let Some(find) = self.focused_find() {
            find.next(ctx);
        }
    }
    pub fn find_prev(&self, ctx: &mut teksilo::prelude::EventContext) {
        if let Some(find) = self.focused_find() {
            find.prev(ctx);
        }
    }

    /// The focused prose editor's handle — `None` when nothing is open there or
    /// the active tab has no main prose field. See
    /// [`crate::search::FindViewModel::editor_handle`] for why the handle lives there.
    pub fn focused_prose_handle(&self) -> Option<teksilo::widgets::rich_text::EditorHandle> {
        self.focused_find()?.editor_handle()
    }

    /// Insert a scene break of `tier` at the caret of the focused prose editor.
    ///
    /// A break is a paragraph of its own, so this splits the block at the caret,
    /// types the canonical mark into the new block, and splits again — leaving
    /// the prose that followed the caret in its own paragraph. Plain text is
    /// right here: the mark is literal, and the Djot escaping that stops it
    /// parsing as a (dropped) thematic break is applied on save.
    ///
    /// Deliberately prose-only. A synopsis has its own editor and a scene break
    /// means nothing there, so there is no focused-editor ambiguity to resolve.
    ///
    /// Restricted to **scene-bearing** items. A Note has a main prose field (and
    /// therefore a handle) but the compiler never scans notes for markers, so
    /// inserting one there would write a mark the exporter silently ignores —
    /// an action that appears to work and does nothing.
    pub fn insert_scene_break(
        &self,
        tier: SceneBreakTier,
        ctx: &mut teksilo::prelude::EventContext,
    ) {
        if !self.focused_carries_scene() {
            return;
        }
        let Some(handle) = self.focused_prose_handle() else {
            return;
        };
        // The gate above is per-TAB; this one is per-EDITOR. A Scene tab also
        // hosts a synopsis box, and the handle here is always the *main prose*
        // editor (only `writing_column` calls `attach_handle`). Without this,
        // typing in the synopsis and pressing the shortcut would edit the
        // manuscript prose instead, at whatever stale caret it still held.
        if !handle.focused_signal().get() {
            return;
        }
        // One atomic call rather than `insert_block` + `insert_text` +
        // `insert_block`: three separate entries into the widget give the
        // application three change notifications to react to mid-edit, and a
        // rebuild between them leaves the rest of the sequence addressing a
        // handle that no longer points at the mounted widget.
        if !handle.insert_paragraph(scene_break::canonical_plain(tier)) {
            // The edit did not land, so do not steal focus as if it had —
            // leaving the caret where the writer left it is the honest failure.
            return;
        }
        // Invoked from the menubar, focus is on the menu overlay, so the edit
        // lands but nothing schedules the repaint that would show it — the mark
        // only appeared once the writer clicked back into the prose. Focusing
        // after the edit fixes that, and leaves the caret where writing resumes.
        handle.focus(ctx);
    }

    /// Whether the focused pane's active tab edits a scene's own prose — the
    /// same predicate `skribisto_compiler` uses to decide what to scan, so the
    /// command surface and the exporter cannot disagree about where a scene
    /// break is meaningful.
    /// The focused tab's main-prose comment binding, if it has one.
    ///
    /// Resolves through the focused *pane's selected tab* rather than any cached
    /// handle: a tab rebuild mints a fresh `ContentTab`, and the `OpenDoc` behind
    /// it is the shared, refcounted one, so this always reaches the live document.
    fn focused_comment_binding(&self) -> Option<crate::comments::binding::CommentBinding> {
        let side = self.focused_side.get();
        let pane = self.pane(side);
        let tab_id = pane.selected.get()?;
        (0..pane.tabs.len()).find_map(|i| {
            pane.tabs
                .with_item(i, |h| {
                    if h.id == tab_id {
                        h.payload
                            .downcast_ref::<ContentTab>()
                            .and_then(|t| t.open_doc.comment_binding_main())
                    } else {
                        None
                    }
                })
                .flatten()
        })
    }

    /// Comment on the focused prose editor's selection.
    ///
    /// Prose-only and selection-only, deliberately: the synopsis has its own
    /// editor (and its own `Content` row), and a zero-width range has nothing to
    /// anchor to. The same per-EDITOR focus gate `insert_scene_break` documents
    /// applies — without it, typing in the synopsis and pressing the shortcut
    /// would annotate the manuscript prose at whatever stale caret it still held.
    ///
    /// Returns whether a thread was actually created, so the caller can follow up
    /// on it — today, the "these comments are unsigned" nudge. Every early return
    /// above is a real no-op (no focus, no selection), and warning about the
    /// signature of a comment that was never made would be noise.
    pub fn add_comment_at_selection(&self, _ctx: &mut teksilo::prelude::EventContext) -> bool {
        let Some(handle) = self.focused_prose_handle() else {
            return false;
        };
        if !handle.focused_signal().get() {
            return false;
        }
        let Some(binding) = self.focused_comment_binding() else {
            return false;
        };
        let (a, p) = handle.selection();
        binding.add_range(a.min(p), a.max(p)).is_some()
    }

    /// Comment on the paragraph the focused prose caret is in.
    ///
    /// Returns whether a thread was created — see
    /// [`add_comment_at_selection`](Self::add_comment_at_selection).
    pub fn add_paragraph_comment(&self, _ctx: &mut teksilo::prelude::EventContext) -> bool {
        let Some(handle) = self.focused_prose_handle() else {
            return false;
        };
        if !handle.focused_signal().get() {
            return false;
        }
        let Some(binding) = self.focused_comment_binding() else {
            return false;
        };
        let (a, p) = handle.selection();
        binding.add_paragraph(a.min(p), a.max(p)).is_some()
    }

    pub fn focused_carries_scene(&self) -> bool {
        let side = self.focused_side.get();
        let pane = self.pane(side);
        let Some(tab_id) = pane.selected.get() else {
            return false;
        };
        (0..pane.tabs.len())
            .find_map(|i| {
                pane.tabs
                    .with_item(i, |h| {
                        if h.id == tab_id {
                            h.payload
                                .downcast_ref::<ContentTab>()
                                .map(|t| t.sub_role().carries_scene())
                        } else {
                            None
                        }
                    })
                    .flatten()
            })
            .unwrap_or(false)
    }

    /// The editor the formatting surfaces act on, what kind of text it holds,
    /// and whether it currently has keyboard focus.
    ///
    /// The focus flag is reported rather than used as a filter, because the two
    /// surfaces need different answers from one walk. The dock wants *live*
    /// focus — click into the binder and there is nothing to format, so it says
    /// so. The Format menu cannot: opening it moves focus to the menu overlay,
    /// so a menu that resolved its target the dock's way would disable every
    /// item at the instant the user reached for one, and its commands would
    /// find nothing to act on. The menu therefore keeps acting on the tab's
    /// editor whether or not it holds focus this instant.
    ///
    /// Prefers the tab's prose editor, falling back to its synopsis. `None` for
    /// a stream row's synopsis or a corkboard card: those build many editors per
    /// tab, so no single per-tab handle can say which one. Those are not
    /// unreachable — they register themselves with `FormatViewModel`, which
    /// prefers whichever registered editor holds focus over this answer.
    pub fn format_target(&self) -> Option<(teksilo::widgets::rich_text::EditorHandle, bool, bool)> {
        self.with_focused_tab(|tab| {
            let prose = tab.find().and_then(|f| f.editor_handle());
            let synopsis = tab.synopsis_handle();
            // Whichever holds focus wins; with neither focused the prose editor
            // is the tab's primary surface and the better default.
            if let Some(h) = &synopsis
                && h.focused_signal().get()
            {
                return Some((h.clone(), true, true));
            }
            if let Some(h) = &prose {
                return Some((h.clone(), false, h.focused_signal().get()));
            }
            synopsis.map(|h| (h, true, false))
        })
    }

    /// Run `read` against the focused pane's active `ContentTab`.
    fn with_focused_tab<R>(&self, read: impl Fn(&ContentTab) -> Option<R>) -> Option<R> {
        let side = self.focused_side.get();
        let pane = self.pane(side);
        let tab_id = pane.selected.get()?;
        (0..pane.tabs.len()).find_map(|i| {
            pane.tabs
                .with_item(i, |h| {
                    if h.id == tab_id {
                        h.payload.downcast_ref::<ContentTab>().and_then(&read)
                    } else {
                        None
                    }
                })
                .flatten()
        })
    }

    /// The `FindViewModel` of the focused pane's active tab — `None` when nothing
    /// is open there or the active tab has no main prose field.
    fn focused_find(&self) -> Option<crate::search::FindViewModel> {
        self.with_focused_tab(|t| t.find().cloned())
    }

    // ── Open ────────────────────────────────────────────────────────────────

    /// Open (or focus) `item_id`'s tab in `side`. The view is chosen per
    /// `(role, sub_role)`; the document is shared through the store, so the same
    /// item can be open once in each pane over one live document.
    pub fn open_in(&self, side: Side, item_id: u64, title: &str) {
        if let Some(tid) = self.find_open(side, item_id) {
            self.pane(side).selected.set(Some(tid));
            self.set_focused(side);
            return;
        }
        let Some(doc) = self.docs.open(item_id) else {
            return;
        };
        let sub_role = doc.sub_role.clone();
        // A trashed item can be open (from the trash dock) — tint its tab icon
        // warning-orange, reactively (flips live on trash/restore, no rebuild).
        let icon_color = doc.trashed.map(|t| {
            if *t {
                TextRole::Warning
            } else {
                TextRole::Primary
            }
        });
        let tab = self.make_tab(
            doc,
            self.distraction_free.clone(),
            self.show_synopsis.clone(),
            self.synopsis_placement.clone(),
            self.caret_highlight.clone(),
        );
        let tab_title = self.caption(item_id, title);
        let id = TabId::fresh();
        self.pane(side).tabs.push(TabHandle::dynamic(
            id,
            "editor",
            TabInfo::new()
                .title(tab_title)
                .closable(true)
                .icon(move || {
                    crate::binder::icons::sub_role_icon(&sub_role).color(icon_color.clone())
                }),
            tab,
        ));
        self.pane(side).selected.set(Some(id));
        self.set_focused(side);
    }

    /// This window's own synopsis-visibility signal — what a **pane** tab is
    /// built with. The distraction-free surface passes its own instead.
    pub fn show_synopsis(&self) -> Signal<bool> {
        self.show_synopsis.clone()
    }

    /// Build a `ContentTab` over `doc` with this window's shared settings
    /// handles. The one place `ContentTab::new`'s arguments are assembled, so a
    /// tab opened in a pane, one rebuilt by a Promote, and one opened by the
    /// distraction-free surface cannot drift apart on anything but the axes they
    /// are meant to differ on — which the caller passes in.
    fn make_tab(
        &self,
        doc: Rc<OpenDoc>,
        distraction_free: Signal<bool>,
        show_synopsis: Signal<bool>,
        synopsis_placement: Signal<crate::shared::SynopsisPlacement>,
        caret_highlight: crate::shared::CaretHighlightSettings,
    ) -> ContentTab {
        ContentTab::new(
            self.app_ctx.clone(),
            self.ids.clone(),
            self.docs.clone(),
            doc,
            self.column_width.clone(),
            show_synopsis,
            synopsis_placement,
            self.synopsis_side_width.clone(),
            self.typography.clone(),
            self.typewriter.clone(),
            caret_highlight,
            self.view_memory.clone(),
            self.corkboard_defaults.clone(),
            self.tree_expansion.clone(),
            distraction_free,
            self.distraction_free_width.clone(),
            self.format.clone(),
            self.writing_games.clone(),
            // The **shared** Work save state, not a fresh one: a segment's edit
            // must bump the counter this window's close guard reads.
            self.save_state.handle(),
            self.goal_unit.clone(),
        )
    }

    /// Build a tab for the **distraction-free surface**: the same shared
    /// `OpenDoc` a pane tab would use, but with `distraction_free` pinned to a
    /// constant `true`.
    ///
    /// A constant, not this window's live flag, is the whole point. The bundle
    /// and column that `ContentTab::main_typography()`/`main_column_width()`
    /// resolve are read once as the pane builds, and panes are memoized — so a
    /// flag that *changes* under a mounted pane silently does nothing (which is
    /// the defect this surface replaces). A flag that is fixed for the lifetime
    /// of the widget tree that reads it is correct by construction.
    ///
    /// Takes a refcount on the shared document, which the caller **must** give
    /// back with [`Self::release_surface_tab`] — `release_own_open_docs` only
    /// walks the two panes' tab lists and cannot see a reference the surface
    /// holds.
    ///
    /// `show_synopsis` is the surface's **own** flag rather than this window's
    /// setting: revealing the synopsis while writing full-screen is a thing you do
    /// for the next few minutes, not a preference change that should follow you
    /// back out and into every other window.
    ///
    /// `caret` is likewise the surface's own bundle rather than
    /// `self.caret_highlight`: the band's colour is a *resolved* colour that
    /// crosses into the document as data, so it cannot ride the mode's token
    /// override and has to be resolved against the distraction-free theme
    /// instead of the app palette — see `DistractionFreeSurfaceViewModel::caret_band`.
    /// It still shares the one scope setting, so how much text is shaded is the
    /// same inside the mode and out.
    pub fn open_surface_tab(
        &self,
        item_id: u64,
        show_synopsis: Signal<bool>,
        caret: crate::shared::CaretHighlightSettings,
    ) -> Option<ContentTab> {
        let doc = self.docs.open(item_id)?;
        // Placement is pinned to Side in this mode rather than following the
        // setting. The mode has one document, a whole screen, and no docks — the
        // reasons a writer chooses Top (a narrow pane, a busy window) are all
        // absent, and the strip's toggle is the only way in, so "shown" and
        // "beside" are the same gesture here.
        Some(self.make_tab(
            doc,
            Signal::new(true),
            show_synopsis,
            Signal::new(crate::shared::SynopsisPlacement::Side),
            caret,
        ))
    }

    /// Give back the refcount [`Self::open_surface_tab`] took.
    pub fn release_surface_tab(&self, item_id: u64, stack: Option<u64>) {
        self.docs.release(item_id, stack);
    }

    /// Seed the caret + page scroll a **not-yet-built** tab will open at.
    ///
    /// Called right after `open_in` during a workspace restore: `open_in` only
    /// pushes a `TabHandle`, so the pane widget has not built yet and the seed
    /// is read once when it does.
    pub fn seed_view_state(&self, side: Side, item_id: u64, state: crate::shared::ViewState) {
        self.with_tab(side, item_id, |t| t.seed_view_state(state));
    }

    /// The live caret + page scroll of whichever open tab shows `item_id`.
    ///
    /// The **focused** side is asked first: with the same item open in both split
    /// panes, "where the writer is" is where they were last typing, not whichever
    /// pane happens to be searched first.
    pub fn view_state_of(&self, item_id: u64) -> Option<crate::shared::ViewState> {
        let focused = self.focused_side.get();
        self.with_tab(focused, item_id, |t| t.capture_view_state())
            .or_else(|| self.with_tab(focused.other(), item_id, |t| t.capture_view_state()))
    }

    /// The Corkboard navigation of whichever open tab shows `item_id` — focused
    /// side first, for the same reason [`Self::view_state_of`] asks it first.
    pub fn corkboard_state_of(&self, item_id: u64) -> Option<(Vec<u64>, String)> {
        let focused = self.focused_side.get();
        self.with_tab(focused, item_id, |t| t.capture_corkboard_state())
            .or_else(|| self.with_tab(focused.other(), item_id, |t| t.capture_corkboard_state()))
            .flatten()
    }

    /// Seed the Corkboard navigation a **not-yet-built** tab will open at, the
    /// counterpart of [`Self::seed_view_state`] on the workspace-restore path.
    pub fn seed_corkboard_state(
        &self,
        side: Side,
        item_id: u64,
        ids: &[u64],
        titles: &[String],
        query: &str,
    ) {
        self.with_tab(side, item_id, |t| {
            t.seed_corkboard_state(ids, titles, query)
        });
    }

    /// Push a caret + page scroll onto whichever open tab shows `item_id`,
    /// without rebuilding it — the surface's exit path. Focused side first, for
    /// the same reason [`Self::view_state_of`] asks it first.
    pub fn apply_view_state(&self, item_id: u64, state: crate::shared::ViewState) {
        let focused = self.focused_side.get();
        if self
            .with_tab(focused, item_id, |t| t.apply_view_state(state))
            .is_none()
        {
            self.with_tab(focused.other(), item_id, |t| t.apply_view_state(state));
        }
    }

    /// Run `f` against the tab in `side` showing `item_id`, if there is one.
    fn with_tab<R>(&self, side: Side, item_id: u64, f: impl Fn(&ContentTab) -> R) -> Option<R> {
        let pane = self.pane(side);
        for i in 0..pane.tabs.len() {
            let hit = pane.tabs.with_item(i, |h| {
                h.payload
                    .downcast_ref::<ContentTab>()
                    .filter(|t| t.item_id() == item_id)
                    .map(&f)
            });
            if let Some(Some(r)) = hit {
                return Some(r);
            }
        }
        None
    }

    /// Open (or focus) `item_id` in the primary pane — the default click / command
    /// path (back-compat with the activation callback + `editor.open_item`).
    pub fn open_or_focus(&self, item_id: u64, title: &str) {
        self.open_in(Side::Primary, item_id, title);
    }

    /// Open (or focus) `item_id` in the side pane, revealing the split first.
    pub fn open_to_side(&self, item_id: u64, title: &str) {
        self.set_split(true);
        self.open_in(Side::Secondary, item_id, title);
    }

    // ── Session snapshot / restore ────────────────────────────────────────────

    /// The ordered `BinderItem` ids of the open editor tabs in `side` (tab order).
    /// Read by the per-work session restore to persist which items were open.
    pub fn tab_item_ids(&self, side: Side) -> Vec<u64> {
        let pane = self.pane(side);
        (0..pane.tabs.len())
            .filter_map(|i| {
                pane.tabs
                    .with_item(i, |h| {
                        h.payload.downcast_ref::<ContentTab>().map(|t| t.item_id())
                    })
                    .flatten()
            })
            .collect()
    }

    /// The `BinderItem` id of the selected editor tab in `side`, if any.
    pub fn selected_item(&self, side: Side) -> Option<u64> {
        let tab = self.pane(side).selected.get()?;
        self.item_of_tab(side, tab)
    }

    /// Which pane currently feeds `active_item` (the focused pane) — captured so
    /// restore can re-mark the same pane focused.
    pub fn focused_side(&self) -> Side {
        self.focused_side.get()
    }

    /// Select the already-open tab for `item_id` in `side` — the session-restore
    /// step that re-marks the selected tab after re-opening the pane's items. A
    /// no-op if no tab there shows that item.
    pub fn select_item(&self, side: Side, item_id: u64) {
        if let Some(tid) = self.find_open(side, item_id) {
            self.pane(side).selected.set(Some(tid));
        }
    }

    // ── Split ───────────────────────────────────────────────────────────────

    /// Show or collapse the side pane. Collapsing **closes** the side pane's tabs
    /// (flushing each) and hides the pane.
    pub fn set_split(&self, active: bool) {
        if active {
            if !self.split_active.get() {
                // Raise the side pane's min width only while shown (the Splitter
                // sums hidden panes' min_size into its own minimum otherwise).
                self.splitter.set_min_size(1, PANE_MIN_WIDTH);
                self.splitter.set_pane_visible(1, true);
                self.split_active.set(true);
            }
            return;
        }
        // Collapse: flush + close every side tab in one pass, then hide + shrink
        // the side pane.
        self.drain_pane(Side::Secondary);
        self.splitter.set_min_size(1, 0.0);
        self.splitter.set_pane_visible(1, false);
        self.split_active.set(false);
        self.set_focused(Side::Primary);
    }

    /// Flush + close every tab in `side` in one shot: release each document, clear
    /// the model, and reset the pane's selection with a single selection-effect
    /// fire — instead of reselecting (and re-scanning the whole store to flush)
    /// once per closed tab.
    fn drain_pane(&self, side: Side) {
        let stack = self.ids.stack_id.get();
        let pane = self.pane(side);
        let items: Vec<u64> = (0..pane.tabs.len())
            .filter_map(|i| {
                pane.tabs
                    .with_item(i, |h| {
                        h.payload.downcast_ref::<ContentTab>().map(|t| t.item_id())
                    })
                    .flatten()
            })
            .collect();
        pane.tabs.clear();
        for id in items {
            self.docs.release(id, stack); // flushes + evicts on the last reference
        }
        pane.selected.set(None);
    }

    /// Toggle the split (the primary pane's split button).
    pub fn toggle_split(&self) {
        self.set_split(!self.split_active.get());
    }

    /// Collapse the split (the side pane's close-split button).
    pub fn close_split(&self) {
        self.set_split(false);
    }

    // ── Close / migrate ─────────────────────────────────────────────────────

    /// A pane's `TabWidget::on_close` hook: flush the tab, remove it, release its
    /// document. Closing the **last** side tab auto-collapses the split.
    pub fn close_in(&self, side: Side, tab_id: TabId) {
        self.close_tab(side, tab_id, true, true);
    }

    /// Flush + remove `tab_id` from `side`, reselect within the pane, and release
    /// its document (evicting on the last reference). With `auto_collapse`, an
    /// emptied side pane collapses the split.
    ///
    /// The tab is matched (and removed) by id **regardless of payload type** — a
    /// tab is always removable, so a stray non-`ContentTab` tab can never wedge a
    /// caller (e.g. a collapse loop); flush + document-release happen only for an
    /// editor tab.
    fn close_tab(&self, side: Side, tab_id: TabId, auto_collapse: bool, flush: bool) {
        let stack = self.ids.stack_id.get();
        let pane = self.pane(side);
        let mut idx = None;
        let mut item = None;
        for i in 0..pane.tabs.len() {
            // `Some(item_opt)` when the id matches (item_opt = the editor item id,
            // flushed here unless `flush` is false, or `None` for a non-editor tab);
            // `None` otherwise.
            let matched = pane
                .tabs
                .with_item(i, |h| {
                    (h.id == tab_id).then(|| {
                        h.payload.downcast_ref::<ContentTab>().map(|t| {
                            // Skip the flush when the item was hard-removed (Delete
                            // Forever / Empty Trash): its `Content` rows are already
                            // gone, so saving would error or resurrect an orphan.
                            if flush {
                                let _ = t.flush(stack);
                            }
                            t.item_id()
                        })
                    })
                })
                .flatten();
            if let Some(item_opt) = matched {
                idx = Some(i);
                item = item_opt;
                break;
            }
        }
        let Some(idx) = idx else {
            return;
        };
        pane.tabs.remove(idx);
        if pane.selected.get() == Some(tab_id) {
            let next = (0..pane.tabs.len()).find_map(|i| pane.tabs.with_item(i, |h| h.id));
            pane.selected.set(next);
        }
        if let Some(item_id) = item {
            self.docs.release(item_id, stack);
        }
        if auto_collapse && side == Side::Secondary && self.secondary.tabs.is_empty() {
            self.set_split(false);
        }
        self.sync_active_item();
    }

    /// A pane's `TabWidget::on_transfer_out` hook: the tab is migrating to the
    /// other pane, so remove it here (replacing the framework's default removal)
    /// but do **not** release its document — the moved tab keeps its reference. If
    /// this empties the side pane, collapse the split (the same invariant
    /// `close_in` enforces, but for the drag-out path).
    pub fn transfer_out(&self, side: Side, tab_id: TabId) {
        let pane = self.pane(side);
        let mut pos = None;
        for i in 0..pane.tabs.len() {
            if pane.tabs.with_item(i, |h| h.id == tab_id) == Some(true) {
                pos = Some(i);
                break;
            }
        }
        if let Some(p) = pos {
            pane.tabs.remove(p);
        }
        if pane.selected.get() == Some(tab_id) {
            let next = (0..pane.tabs.len()).find_map(|i| pane.tabs.with_item(i, |h| h.id));
            pane.selected.set(next);
        }
        if side == Side::Secondary && self.secondary.tabs.is_empty() {
            self.set_split(false);
        }
    }

    /// A pane's `TabWidget::on_tab_received` hook (cross-pane migration): if the
    /// target pane already shows this item, dedup — drop the incoming (releasing
    /// its now-redundant document reference) and focus the existing tab; otherwise
    /// insert the migrated handle (which keeps its document reference).
    pub fn receive_tab(&self, side: Side, handle: TabHandle) {
        let item_id = handle
            .payload
            .downcast_ref::<ContentTab>()
            .map(|t| t.item_id());
        if let Some(item_id) = item_id
            && let Some(existing) = self.find_open(side, item_id)
        {
            self.docs.release(item_id, self.ids.stack_id.get());
            self.pane(side).selected.set(Some(existing));
            self.set_focused(side);
            return;
        }
        let id = handle.id;
        self.pane(side).tabs.push(handle);
        self.pane(side).selected.set(Some(id));
        self.set_focused(side);
    }

    // ── Flush / save / reset ────────────────────────────────────────────────

    /// Persist every open document's edits back to its `Content` rows (changed
    /// fields only), through the per-Work undo stack. Each shared doc flushed once.
    pub fn flush_all(&self) {
        self.docs.flush_all(self.ids.stack_id.get());
    }

    /// Flush all editors to the store, then write the project to disk
    /// (`save_work`, a long operation). **Inert in backup mode**: the backup file
    /// is read-only, so edits are kept only via Save As / Restore — never a
    /// silent overwrite of the backup. (The content still lives in the store; it
    /// simply never reaches disk here.)
    pub fn save_to_disk(&self) {
        let _ = self.request_save();
    }

    /// [`Self::save_to_disk`], returning the **edit sequence** the resulting save
    /// will cover — everything mutated up to that point is on disk once
    /// [`Self::saved_seq`] reaches it. `None` in backup mode (saving is inert) or
    /// if the command could not be issued at all.
    ///
    /// Callers that *defer* an action until the edits are safely on disk (the close
    /// guard, the project-switch guard) wait on that sequence rather than on "some
    /// save finished": with autosave on, another `save_work` can already be in
    /// flight, and it may have gathered the store *before* this flush.
    ///
    /// **At most one `save_work` runs at a time**, across every window sharing
    /// this project ([`SaveStateViewModel`]'s `SaveQueue`): if one is already in
    /// flight — started from this window or another one — this queues a
    /// follow-up instead of starting a second op — two ops write the same path
    /// and their completion order is unspecified, so an older snapshot could
    /// land last and silently regress the file.
    pub fn request_save(&self) -> Option<u64> {
        if self.backup_mode.get() {
            return None;
        }
        // Flush first, then let the shared save state read the sequence: the
        // store now holds everything the user has typed, so a save started here
        // covers exactly this seq.
        self.flush_all();
        self.save_state.request_save()
    }

    /// Route a `LongOperation::Completed`. `None` if it wasn't **our** save — a
    /// backup's, an import's or a Save As's completion is left to their own
    /// view-models.
    ///
    /// If edits arrived while that save was running, the follow-up is issued here
    /// (flushing through `self`, the editors that happened to process this
    /// particular delivery — see [`SaveStateViewModel::on_save_completed`] for why
    /// that flush runs exactly once even with several windows sharing the save
    /// state).
    pub fn on_save_completed(&self, event: &Event) -> Option<SaveLanded> {
        let editors = self.clone();
        self.save_state
            .on_save_completed(event, move || editors.flush_all())
    }

    /// Route a `LongOperation::Failed`. `Some(error)` if it was our save: the queue
    /// goes idle and any queued follow-up is dropped. The edits are untouched —
    /// still in the store, still dirty — so nothing is lost by not retrying behind
    /// the user's back; the caller reports it.
    pub fn on_save_failed(&self, event: &Event) -> Option<String> {
        self.save_state.on_save_failed(event)
    }

    /// The highest edit sequence written to disk. With `dirty_seq` (bumped on every
    /// mutation) this is the truth behind "unsaved". Shared with every other window
    /// onto this project — see [`SaveStateViewModel`].
    pub fn saved_seq(&self) -> Signal<u64> {
        self.save_state.saved_seq()
    }

    /// Whether the store holds edits not yet on disk (`dirty_seq > saved_seq`) —
    /// the same derivation `App` publishes as `unsaved`, read synchronously (no
    /// dependence on the derived-signal effect having fired). The workspace-layout
    /// capture consults this: persisting tab **ordinals** while dirty could record
    /// positions against an in-store structure the on-disk file (what a reload
    /// resolves against) doesn't share.
    pub fn is_unsaved(&self) -> bool {
        self.save_state.is_unsaved()
    }

    /// A disk save is in flight. Shared with every other window onto this project.
    pub fn saving(&self) -> Signal<bool> {
        self.save_state.saving()
    }

    /// Mark everything currently in the store as "on disk" — a project just loaded,
    /// was created, or was closed: nothing is pending against *this* work.
    ///
    /// Also forgets any save still outstanding: it was a save of the **outgoing**
    /// project. Leaving it in the queue would let its completion fire the queued
    /// follow-up against the store that replaced it — a pointless full write of the
    /// new project, or (after a close) a save of an empty store whose failure would
    /// toast at a user who has already left.
    pub fn mark_clean(&self) {
        self.save_state.mark_clean();
    }

    /// Release every reference this window's own tabs (both panes) hold on the
    /// shared [`OpenDocsStore`] — the exact per-item inverse of every `open_in`/
    /// `open_to_side` this window ever ran. `stack` is the undo stack to flush
    /// through, snapshotted by the caller: this runs from the window's own
    /// `on_removed`-driven teardown (see `sessions::WorkRegistry::remove_window`),
    /// which fires *after* the window's tree — and, in today's single-window-
    /// per-Work flow, after `AppIds::clear()` has already zeroed `self.ids` — is
    /// gone, so reading `self.ids.stack_id` live here would always see `None`.
    ///
    /// Deliberately **not** `close_all`/`docs.clear()`: that method is for an
    /// in-place project switch happening in a window that is *staying open*
    /// (so it also clears this window's own tab lists, and clearing the whole
    /// shared store is correct there — the previous project is leaving this
    /// window and no other window shares its `WorkSession` yet). This one runs
    /// once teksilo confirms the window itself is gone — there is no tab list
    /// left worth clearing, only the shared store's refcounts *this* window
    /// itself was holding. With a second window sharing this Work
    /// (`AttachExisting`), `docs.clear()` here would drop items a sibling
    /// window still has open; releasing exactly this window's own
    /// `tab_item_ids` never does.
    pub fn release_own_open_docs(&self, stack: Option<u64>) {
        for side in [Side::Primary, Side::Secondary] {
            for item_id in self.tab_item_ids(side) {
                self.docs.release(item_id, stack);
            }
        }
    }

    /// Close every tab in both panes and reset the split (e.g. on project load).
    /// Does not flush — the outgoing work is saved/discarded by the close flow.
    pub fn close_all(&self) {
        self.primary.tabs.clear();
        self.secondary.tabs.clear();
        // Drop the documents *before* clearing selection: `selected.set(None)`
        // synchronously re-fires App's per-pane effect (which calls `flush_all`),
        // so emptying the store first keeps that a cheap no-op and honors this
        // method's no-flush contract.
        self.docs.clear();
        self.primary.selected.set(None);
        self.secondary.selected.set(None);
        self.splitter.set_min_size(1, 0.0);
        self.splitter.set_pane_visible(1, false);
        self.split_active.set(false);
        self.focused_side.set(Side::Primary);
        self.active_item.set(None);
        // The seam's view of the same thing. Set here rather than left to
        // `sync_active_item` because this method deliberately bypasses it (there
        // is nothing to recompute from — both panes are already empty), and a
        // second answer to "what is focused" that stayed behind would have a dock
        // still naming a document from the project that just closed.
        self.active_ctx.set(None);
    }

    // ── Internals ───────────────────────────────────────────────────────────

    fn pane(&self, side: Side) -> &Pane {
        match side {
            Side::Primary => &self.primary,
            Side::Secondary => &self.secondary,
        }
    }

    /// Subscribe to the backend changes the open tabs have to follow. Call once from the
    /// window's long-lived `build` (the subscriptions live as long as that build does).
    ///
    /// Two are per-item: an edited item may need its tab rebuilt or re-captioned, a removed
    /// one needs its tab closed. The rest are **structural** — they change no open item, but
    /// they renumber the manuscript, and an untitled chapter's caption *is* its number. They
    /// are the same set [`crate::models::BinderBinderItemsTreeModel::wire`] re-sources the
    /// binder on, for exactly the same reason; a name shown in two docks must change in both
    /// at once.
    pub fn wire(&self, ctx: &mut BuildContext) {
        use frontend::common::event::{
            BinderItemManagementEvent, DirectAccessEntity as E, EntityEvent, TrashManagementEvent,
        };

        {
            let me = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(E::BinderItem(EntityEvent::Updated)),
                move |event: &Event| me.items_updated(&event.ids),
            );
        }
        {
            let me = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(E::BinderItem(EntityEvent::Removed)),
                move |event: &Event| me.items_removed(&event.ids),
            );
        }
        let structural = [
            // A new chapter above an untitled one renumbers it, and nothing about the
            // untitled one itself changes.
            Origin::DirectAccess(E::BinderItem(EntityEvent::Created)),
            Origin::DirectAccess(E::Binder(EntityEvent::Created)),
            // A binder's item *order* lives on the binder, so a reorder is a `Binder`
            // update — not an item one.
            Origin::DirectAccess(E::Binder(EntityEvent::Updated)),
            Origin::DirectAccess(E::Binder(EntityEvent::Removed)),
            // "Number chapters and parts" and "Restart chapter numbers at each part" are
            // `Work` fields: turning either off renames every untitled chapter on screen.
            Origin::DirectAccess(E::Work(EntityEvent::Updated)),
            Origin::BinderItemManagement(BinderItemManagementEvent::Duplicate),
            Origin::BinderItemManagement(BinderItemManagementEvent::MoveItems),
            Origin::BinderItemManagement(BinderItemManagementEvent::MergeTwoScenes),
            Origin::BinderItemManagement(BinderItemManagementEvent::SplitScene),
            Origin::TrashManagement(TrashManagementEvent::TrashBinderItems),
            Origin::TrashManagement(TrashManagementEvent::TrashBinder),
            Origin::TrashManagement(TrashManagementEvent::RestoreItems),
            Origin::TrashManagement(TrashManagementEvent::EmptyTrash),
        ];
        for origin in structural {
            let me = self.clone();
            ctx.subscribe_event(origin, move |_e: &Event| me.refresh_captions());
        }
    }

    /// React to `BinderItem::Updated` for `item_ids`: **rebuild** any open tab whose item's
    /// *type* changed, **close** any open tab for an item that has just been trashed,
    /// re-seed the live trash/tag state, then re-caption every open tab.
    ///
    /// The view never reads entities itself, so the lookup lives here.
    pub fn items_updated(&self, item_ids: &[u64]) {
        let probe = SingleBinderItem::new(self.app_ctx.clone());
        for id in item_ids {
            probe.set_id(Some(*id));
            let Some(it) = probe.dto() else { continue };
            // Trash / restore fire `BinderItem::Updated` too — flip the shared
            // doc's trash state so the tab accent + banner react live.
            let trashed = !it.activated;
            let mut trash_flipped = false;
            if let Some(doc) = self.docs.peek(*id) {
                trash_flipped = doc.trashed.get() != trashed;
                doc.trashed.set(trashed);
                // Same event, same fetched DTO: keep the subtitle's dot row live.
                doc.tags.set(it.tags.clone());
            }
            // A Promote rewrites the item's type. The open tab was built for the *old*
            // one — it is still showing a chapter's segments and a chapter's editors for
            // what is now a Part — and its `OpenDoc` still owns the old type's fields. Both
            // have to be rebuilt.
            if self.retype(*id, &it) {
                // The rebuilt doc starts un-trashed; re-seed its trash state.
                if let Some(doc) = self.docs.peek(*id) {
                    doc.trashed.set(trashed);
                    doc.tags.set(it.tags.clone());
                }
            } else if trash_flipped && self.has_open_tab(*id) {
                if trashed {
                    self.close_trashed_tabs(*id);
                } else if let Some(doc) = self.docs.peek(*id) {
                    // Restoring. Usually a no-op now, because the trash branch
                    // above already closed the tab — this is the other way in: a
                    // trashed item opened *from the Trash dock*, which is
                    // read-only, and restoring it has to make it typable again.
                    // `RichTextEditor` fixes that policy when it is **built**, so
                    // re-binding cannot do it; only a rebuild can.
                    self.rebuild_tabs_for(*id, &it, doc);
                }
            }
        }
        // One sweep for the whole batch, not a per-item push: a rename changes one tab's
        // caption, but the same event fires for a trash or a move, which renumbers — and so
        // renames — every *untitled* structural tab after it. See [`Self::refresh_captions`].
        self.refresh_captions();
    }

    /// Close every open tab for an item that has just been trashed.
    ///
    /// Trashing something takes it out of the manuscript, and a tab is the app
    /// saying "you are working on this". Leaving one open — even a read-only one
    /// behind a banner — leaves the writer looking at a document they just put
    /// away.
    ///
    /// **`flush: true`**, unlike the hard-removal path in [`Self::items_removed`],
    /// and the difference is the whole safety of this. Trashing is soft and
    /// undoable: the content row is still there, and whatever was typed in the
    /// seconds before the trash has to reach it. Closing without the flush would
    /// drop those words into nothing, silently, at the one moment a writer is
    /// least likely to go back and check.
    ///
    /// Its own method so the policy can be tested: [`Self::items_updated`] cannot
    /// run at all without a store behind it.
    fn close_trashed_tabs(&self, item_id: u64) {
        for side in [Side::Primary, Side::Secondary] {
            if let Some(tid) = self.find_open(side, item_id) {
                self.close_tab(side, tid, true, true);
            }
        }
    }

    /// React to `BinderItem::Removed` for `item_ids`: close any open tab for a
    /// hard-removed item (Delete Forever / Empty Trash of an item that was open),
    /// so no tab is left pointing at a vanished entity.
    pub fn items_removed(&self, item_ids: &[u64]) {
        for &id in item_ids {
            for side in [Side::Primary, Side::Secondary] {
                if let Some(tid) = self.find_open(side, id) {
                    // No flush: the item + its content are already hard-removed.
                    self.close_tab(side, tid, true, false);
                }
            }
        }
    }

    /// Rebuild every open tab for `item_id` if its `(role, sub_role)` no longer matches
    /// the entity. Returns whether anything was rebuilt.
    ///
    /// The document is rebuilt *in place* in the store (same reference count), so every
    /// other holder — a split pane, a stream row — picks up the fresh one too rather than
    /// being handed the stale cached `Rc`.
    /// Rebuild every open tab for `item_id` from the document the store now holds.
    ///
    /// Split out of [`Self::retype`] because a **restore** needs exactly the same
    /// thing for a different reason: `RichTextEditor` fixes its read-only policy
    /// when it is *built*, so a tab opened read-only from the Trash dock stays
    /// read-only however the trash flag moves afterwards. Re-binding cannot help;
    /// only rebuilding can. See `tabs::shared::editor::writing_column`.
    ///
    /// The trashing direction does not come here — it closes the tab instead, in
    /// [`Self::items_updated`].
    ///
    /// A fresh `TabId` and remove+insert, not `set`: the `TabWidget` keys its
    /// mounted content by tab id, so reusing the id updates the strip and leaves
    /// the old editor on screen.
    fn rebuild_tabs_for(&self, item_id: u64, it: &BinderItemDto, doc: Rc<OpenDoc>) {
        for side in [Side::Primary, Side::Secondary] {
            let pane = self.pane(side);
            for i in 0..pane.tabs.len() {
                let hit = pane.tabs.with_item(i, |h| {
                    h.payload
                        .downcast_ref::<ContentTab>()
                        .is_some_and(|t| t.item_id() == item_id)
                        .then(|| h.clone())
                });
                let Some(Some(h)) = hit else { continue };
                let tab = self.make_tab(
                    doc.clone(),
                    self.distraction_free.clone(),
                    self.show_synopsis.clone(),
                    self.synopsis_placement.clone(),
                    self.caret_highlight.clone(),
                );
                let caption = self.caption(item_id, &it.title);
                let sub_role = it.sub_role.clone();
                let was_selected = pane.selected.get() == Some(h.id);
                let new_id = TabId::fresh();
                pane.tabs.remove(i);
                pane.tabs.insert(
                    i,
                    TabHandle::dynamic(
                        new_id,
                        "editor",
                        TabInfo::new()
                            .title(caption)
                            .closable(true)
                            .icon(move || crate::binder::icons::sub_role_icon(&sub_role)),
                        tab,
                    ),
                );
                if was_selected {
                    pane.selected.set(Some(new_id));
                }
            }
        }
    }

    /// Whether `item_id` has an open tab at all.
    fn has_open_tab(&self, item_id: u64) -> bool {
        [Side::Primary, Side::Secondary].iter().any(|&side| {
            let pane = self.pane(side);
            (0..pane.tabs.len()).any(|i| {
                pane.tabs
                    .with_item(i, |h| {
                        h.payload
                            .downcast_ref::<ContentTab>()
                            .is_some_and(|t| t.item_id() == item_id)
                    })
                    .unwrap_or(false)
            })
        })
    }

    fn retype(&self, item_id: u64, it: &BinderItemDto) -> bool {
        // Only *this item's own* tab going stale should force a rebuild. Without the
        // `item_id` guard, `needs_rebuild` fired whenever **any** open tab had a different
        // type than the updated item — so editing a Book's word-count goal while a Scene
        // tab was also open rebuilt the Book's tab (fresh `ContentTab`, its Pace segment
        // reset to 0), even though the Book's own type never changed.
        let stale = |t: &ContentTab| {
            t.item_id() == item_id && (t.role() != &it.role || t.sub_role() != &it.sub_role)
        };
        let needs_rebuild = [Side::Primary, Side::Secondary].iter().any(|&side| {
            let pane = self.pane(side);
            (0..pane.tabs.len()).any(|i| {
                pane.tabs
                    .with_item(i, |h| {
                        h.payload.downcast_ref::<ContentTab>().is_some_and(stale)
                    })
                    .unwrap_or(false)
            })
        });
        if !needs_rebuild {
            return false;
        }

        let stack = self.ids.stack_id.get();
        let Some(doc) = self.docs.rebuild(item_id, stack) else {
            return false;
        };
        // Through `rebuild_tabs_for` / `make_tab`, not a second inline
        // `ContentTab::new` — a duplicated argument list would let per-tab state
        // drift between the two, so a Promote could silently produce a tab
        // configured differently from a freshly opened one. Per-tab state (the
        // folded-away Side synopsis, its divider position) resets here: this is a
        // new tab of a new type. The caret does not — `tab_pane` seeds it.
        self.rebuild_tabs_for(item_id, it, doc);
        true
    }

    // ── Tab captions ────────────────────────────────────────────────────────

    /// The manuscript's naming context, or `None` with no project open — the welcome
    /// window, a headless test, the mock build. Every caption then falls back to the
    /// title the caller already holds.
    fn names(&self) -> Option<crate::models::NameContext> {
        let work_id = self.ids.work_id.get()?;
        Some(crate::models::NameContext::read(&self.app_ctx, work_id))
    }

    /// What `item_id`'s tab is called, reading the manuscript only when it has to.
    ///
    /// **An untitled chapter is not nameless.** Since numbering stopped writing "Chapter 7"
    /// into titles, every other surface — the binder, the stream, the corkboard, the
    /// Overview, the exported book — derives that name instead; tabs were the one place
    /// still saying "Untitled", so a writer who names no chapters got a strip of identical
    /// "Untitled"s with nothing to tell them apart.
    ///
    /// `title` is what the caller already holds (an outline row, a restored session), so
    /// the common titled case never touches the backend. Only a row with no title of its
    /// own pays for the whole-Work read, and it has to: its name is its ordinal, and that
    /// depends on every item before it.
    fn caption(&self, item_id: u64, title: &str) -> LocalizedString {
        if !title.trim().is_empty() {
            return lit!(title.to_string());
        }
        let generated = self.names().and_then(|names| {
            let it = names.item(item_id)?;
            names.generated_name(it)
        });
        Self::caption_of(title, generated)
    }

    /// The caption for a row whose title and generated name are both already known.
    fn caption_of(title: &str, generated: Option<String>) -> LocalizedString {
        if !title.trim().is_empty() {
            lit!(title.to_string())
        } else if let Some(name) = generated {
            lit!(name)
        } else {
            // A scene or a note has no name to generate — labelling one "Scene" would be
            // noise, so an unnamed one stays "Untitled" here (a tab, unlike a tree row,
            // cannot be blank: there would be nothing left to click).
            tr!(untitled())
        }
    }

    /// Re-caption every open tab from the manuscript's current state.
    ///
    /// A caption is a plain `LocalizedString` baked in when the tab is built — it does not
    /// follow a signal — so everything that can change a name has to push it. A rename is
    /// the obvious one; **renumbering is the other**, and it has no rename event to ride:
    /// inserting a chapter above an untitled one renames it from "Chapter 3" to "Chapter 4"
    /// without touching it at all, as does trashing one, moving one, or turning numbering
    /// off in Settings. Hence the whole sweep on every structural change rather than a
    /// per-item push.
    ///
    /// `TabHandle::info` is a public field and `payload` is an `Rc`, so each handle is
    /// rebuilt with a new caption around the *same* `ContentTab` — the same live documents,
    /// caret and scroll position are carried straight through, and the `TabWidget` keys its
    /// mounted content by tab id, which does not change.
    pub fn refresh_captions(&self) {
        if self.primary.tabs.is_empty() && self.secondary.tabs.is_empty() {
            return;
        }
        let Some(names) = self.names() else { return };
        for side in [Side::Primary, Side::Secondary] {
            let pane = self.pane(side);
            for i in 0..pane.tabs.len() {
                let hit = pane.tabs.with_item(i, |h| {
                    h.payload
                        .downcast_ref::<ContentTab>()
                        .map(|t| (t.item_id(), h.clone()))
                });
                let Some(Some((item_id, mut h))) = hit else {
                    continue;
                };
                // An item the store cannot answer for — a tab whose item is mid-teardown,
                // a mock build with no manuscript behind it — keeps what it is showing.
                // This only ever *corrects* a caption; it never blanks one.
                let Some(it) = names.item(item_id) else {
                    continue;
                };
                let caption = Self::caption_of(&it.title, names.generated_name(it));
                h.info = h.info.clone().title(caption);
                pane.tabs.set(i, h);
            }
        }
    }

    /// `Some(tab id)` if an editor for `item_id` is open in `side`.
    fn find_open(&self, side: Side, item_id: u64) -> Option<TabId> {
        let pane = self.pane(side);
        for i in 0..pane.tabs.len() {
            let hit = pane.tabs.with_item(i, |h| {
                h.payload
                    .downcast_ref::<ContentTab>()
                    .filter(|t| t.item_id() == item_id)
                    .map(|_| h.id)
            });
            if let Some(Some(tid)) = hit {
                return Some(tid);
            }
        }
        None
    }

    /// The `BinderItem` id behind a tab in `side`, if it's an editor tab.
    fn item_of_tab(&self, side: Side, tab: TabId) -> Option<u64> {
        let pane = self.pane(side);
        for i in 0..pane.tabs.len() {
            let hit = pane.tabs.with_item(i, |h| {
                if h.id == tab {
                    h.payload.downcast_ref::<ContentTab>().map(|t| t.item_id())
                } else {
                    None
                }
            });
            if let Some(Some(id)) = hit {
                return Some(id);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests;
