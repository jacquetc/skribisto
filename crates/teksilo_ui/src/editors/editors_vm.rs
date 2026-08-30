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

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use uuid::Uuid;

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
use crate::models::{OpenDoc, OpenDocsStore, TabViewState};
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
    /// Which of this pane's tabs the writer has pinned, by `BinderItem` **store
    /// id**.
    ///
    /// Keyed by item id rather than by any of the three obvious alternatives,
    /// each of which fails:
    ///
    /// * **`TabInfo::pinned`** — its fields are `pub(crate)` to teksilo, so the
    ///   app can write the flag but never read it back; and
    ///   [`EditorsViewModel::rebuild_tabs_for`] builds a brand-new `TabInfo`, so
    ///   it is destroyed by every Promote and every trash-restore.
    /// * **`ContentTab`** — destroyed by the same rebuild, which is what
    ///   `updating_an_item_does_not_rebuild_a_differently_typed_open_tab` already
    ///   pins for the segment.
    /// * **`TabId`** — that rebuild mints a *fresh* one, so a `TabId`-keyed pin
    ///   would need an explicit carry at that site. An item id survives it for
    ///   free, which makes the Promote case correct by construction rather than
    ///   by remembering.
    ///
    /// Not a `Uuid` either: pin checks run inside the close and reorder loops,
    /// and a `binder_ops::item_dto` read per check is both wasteful and awkward
    /// inside a `with_item` borrow. The one uid translation persistence needs
    /// happens exactly once, in the workspace capture.
    ///
    /// **A membership oracle only.** Order and liveness come from `tabs`, which
    /// is already the truth (`delegate, don't reimplement`): [`EditorsViewModel::pinned_item_ids`]
    /// walks the model and keeps the members, so an entry for an item that is not
    /// open here is inert and can never reach the capture. Entries are pruned on
    /// close for hygiene, not for correctness.
    pinned: Rc<RefCell<HashSet<u64>>>,
}

impl Pane {
    fn new() -> Self {
        Self {
            tabs: ListModel::from_vec(Vec::new()),
            selected: Signal::new(None),
            pinned: Rc::new(RefCell::new(HashSet::new())),
        }
    }
}

/// Minimum width of a pane, so the splitter can't crush an editor to nothing.
const PANE_MIN_WIDTH: f32 = 320.0;

/// Given a new tab's id and the [`TabInfo`] it is about to wear, return that info
/// with this window's tab-strip context menu attached.
///
/// Injected after construction ([`EditorsViewModel::set_tab_menu`]) rather than
/// taken as a constructor argument, the same two-phase shape
/// `set_item_view_states` and `WorkspaceLayoutViewModel::set_editors` take: the
/// menu is a surface *above* this view-model, so naming
/// [`crate::editors::tab_menu`] from in here would point a DAG edge the wrong way
/// up. `None` — a headless test, or any build before the injection — simply means
/// the tabs carry no menu, which is graceful and is what the existing fixtures in
/// `editors_vm/tests.rs` want.
pub type TabMenuInstaller = Rc<dyn Fn(TabId, TabInfo) -> TabInfo>;

#[derive(Clone)]
pub struct EditorsViewModel {
    app_ctx: Rc<AppContext>,
    /// Where the writer was in each item of this project, whether or not a tab is
    /// open on it. Injected after construction (`set_item_view_states`) rather than
    /// taken as a constructor argument, the same shape `WorkspaceLayoutViewModel`
    /// takes this view-model by: it is Tier 2 and this is Tier 3, so the handle is
    /// created first and pointed at afterwards.
    item_view_states: Rc<RefCell<Option<crate::shared::ItemViewStates>>>,
    /// Attaches this window's tab-strip context menu to every tab as it is built.
    /// See [`TabMenuInstaller`] for why it is injected rather than constructed.
    tab_menu: Rc<RefCell<Option<TabMenuInstaller>>>,
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
    /// This window's tag palette (Tier 2, its `WorkSession`'s `TagsViewModel`), threaded
    /// into every `ContentTab` so a segment like `note_details` reaches a handle bound to
    /// this project's own `Work` instead of reading `ctx.app_state::<TagsViewModel>()`.
    /// See `ContentTab::tags`'s own doc for the bug that closes.
    tags: crate::tags::TagsViewModel,
    statuses: crate::statuses::StatusesViewModel,
    /// Who is named where, across this Work. Threaded for exactly the reason
    /// [`crate::tabs::ContentTab::tags`] is: `note_details`'s "Appears in the manuscript"
    /// column reads it, and `ctx.app_state::<MentionIndex>()` answers with whichever
    /// session registered, which at first launch is `lib.rs`'s throwaway `WorkSession` on
    /// a never-seeded `AppIds`. That index's own `fire` returns early with no `work_id`,
    /// so it never scans anything and the column is empty for the whole session.
    mention_index: crate::mentions::MentionIndex,
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
        tags: crate::tags::TagsViewModel,
        statuses: crate::statuses::StatusesViewModel,
        mention_index: crate::mentions::MentionIndex,
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
            item_view_states: Rc::new(RefCell::new(None)),
            tab_menu: Rc::new(RefCell::new(None)),
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
            tags,
            statuses,
            mention_index,
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
        self.focused_own_find()?.editor_handle()
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

    /// The find banner Ctrl+F means: the one belonging to the **page on screen** in the
    /// focused pane's active tab. `None` when nothing is open there, or when that page
    /// has no prose to search.
    ///
    /// A segmented tab has two — its own document's, and its streams' — and which of
    /// them the writer means is the segment bar's answer, not the tab type's. See
    /// [`ContentTab::active_find`](crate::tabs::ContentTab::active_find).
    fn focused_find(&self) -> Option<crate::search::FindViewModel> {
        self.with_focused_tab(|t| t.active_find().cloned())
    }

    /// The focused tab's **own document's** banner, whichever page is on screen.
    ///
    /// Distinct from [`focused_find`](Self::focused_find) because it is not asked as a
    /// find question at all: it is how the prose-editing commands reach "this tab's main
    /// editor", which is a fact about the tab and not about the page — see
    /// [`focused_prose_handle`](Self::focused_prose_handle).
    fn focused_own_find(&self) -> Option<crate::search::FindViewModel> {
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
        let trashed = doc.trashed.clone();
        let tab = self.make_tab(
            doc,
            self.distraction_free.clone(),
            self.show_synopsis.clone(),
            self.synopsis_placement.clone(),
            self.caret_highlight.clone(),
        );
        let tab_title = self.caption(item_id, title);
        let id = TabId::fresh();
        // A freshly opened tab is never pinned: pinning is a gesture on a tab
        // that is already there, and the workspace restore re-applies a
        // remembered pin through `seed_pinned` once every tab has landed.
        let info = self.tab_info(id, tab_title, &sub_role, &trashed, false);
        self.pane(side)
            .tabs
            .push(TabHandle::dynamic(id, "editor", info, tab));
        self.pane(side).selected.set(Some(id));
        self.set_focused(side);
        // Where the writer was in this item last time, if this project remembers.
        // After the push, because the seed is written onto the `ContentTab` the pane
        // has just taken ownership of; before the pane builds, because that is when
        // the seed is read.
        self.seed_from_memory(side, item_id);
    }

    /// This window's own synopsis-visibility signal — what a **pane** tab is
    /// built with. The distraction-free surface passes its own instead.
    pub fn show_synopsis(&self) -> Signal<bool> {
        self.show_synopsis.clone()
    }

    /// The whole `TabInfo` an editor tab wears: caption, leading glyph, close
    /// affordance, tooltip and this window's context menu.
    ///
    /// **The one place it is assembled**, so [`Self::open_in`] and
    /// [`Self::rebuild_tabs_for`] cannot drift — which they already had. The
    /// rebuild (run on every Promote and every trash-restore) built a bare
    /// `sub_role_icon` and silently dropped the reactive warning-orange tint
    /// `open_in` sets, so a trashed open tab lost its tint the moment it was
    /// retyped. Anything added here — the menu factory above all — would have
    /// gone the same way. `refresh_captions` is the opposite and is deliberately
    /// left alone: it clones the existing info and overwrites only the title, so
    /// it preserves every field this builds without knowing any of them.
    ///
    /// `pinned` drives presentation only; the truth lives in [`Pane::pinned`].
    /// It is expressed as `closable(false)` plus a drawing-pin glyph rather than
    /// teksilo's own `TabInfo::pinned`, which would render the tab icon-only at a
    /// fixed 32 dp with its title demoted to a tooltip — for a manuscript that
    /// turns three pinned scenes into three identical glyphs, the exact failure
    /// `MIN_EDITOR_TAB_WIDTH` exists to prevent. `closable(false)` removes the
    /// close button, the middle-click close *and* the Delete key together
    /// (teksilo builds `on_close` only when closable), while icon-only rendering
    /// branches on `pinned` alone — so this is teksilo's pin semantics without
    /// teksilo's pin presentation, and the context menu's Close row becomes the
    /// only way to close a pinned tab.
    fn tab_info(
        &self,
        id: TabId,
        caption: LocalizedString,
        sub_role: &frontend::common::entities::BinderItemSubRole,
        trashed: &Signal<bool>,
        pinned: bool,
    ) -> TabInfo {
        // A trashed item can be open (from the trash dock) — tint its tab icon
        // warning-orange, reactively (flips live on trash/restore, no rebuild).
        let icon_color = trashed.map(|t| {
            if *t {
                TextRole::Warning
            } else {
                TextRole::Primary
            }
        });
        let sub_role = sub_role.clone();
        let mut info = TabInfo::new()
            .title(caption)
            .closable(!pinned)
            .icon(move || {
                if pinned {
                    crate::icons::editor::pinned_tab().color(icon_color.clone())
                } else {
                    crate::binder::icons::sub_role_icon(&sub_role).color(icon_color.clone())
                }
            });
        if pinned {
            // The glyph replaced the sub-role icon, so hovering is the only way
            // left to learn *why* this tab has no cross.
            info = info.tooltip(tr!(tab_pinned_tooltip()));
        }
        // Bound to a local first, deliberately: a `match` keeps its scrutinee's
        // temporaries alive for the whole arm, so `match self.tab_menu.borrow()…`
        // would hold the `Ref` across the installer call. Nothing re-enters today,
        // but an installer that ever touched this cell would panic rather than
        // fail visibly, and this costs nothing.
        let installer = self.tab_menu.borrow().clone();
        match installer {
            Some(install) => install(id, info),
            None => info,
        }
    }

    /// Point this view-model's tabs at the window's tab-strip context menu.
    /// See [`TabMenuInstaller`] for why this is injected rather than constructed.
    pub fn set_tab_menu(&self, installer: TabMenuInstaller) {
        *self.tab_menu.borrow_mut() = Some(installer);
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
            self.tags.clone(),
            self.statuses.clone(),
            self.mention_index.clone(),
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
        self.with_tab(side, item_id, |t| t.seed_remembered_view_state(state));
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

    /// The segment whichever open tab shows `item_id` is on, as its string id, or
    /// an empty string for a tab with no segmented bar. Focused side first, for the
    /// same reason [`Self::view_state_of`] asks it first.
    pub fn segment_of(&self, item_id: u64) -> Option<String> {
        let focused = self.focused_side.get();
        self.with_tab(focused, item_id, |t| t.segment_shown())
            .or_else(|| self.with_tab(focused.other(), item_id, |t| t.segment_shown()))
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

    /// Point this view-model at the project's remembered positions. Called once,
    /// beside `WorkspaceLayoutViewModel::set_editors`, for the reason given on the
    /// field.
    pub fn set_item_view_states(&self, states: crate::shared::ItemViewStates) {
        *self.item_view_states.borrow_mut() = Some(states);
    }

    /// The **writer** asked to go to `item_id`: open or raise its tab, and put the
    /// caret in it.
    ///
    /// The plain [`Self::open_or_focus`] deliberately does not do this. Opening a
    /// project restores tabs without ever taking the keyboard away from where the
    /// writer left it, and so do the several places that open a tab as a side
    /// effect of something else. This is the one door for a deliberate gesture: a
    /// click in the outline, a double-click or Enter on an Overview row.
    ///
    /// Two routes, because a tab that is already open is **not** rebuilt when it is
    /// raised: a `TabWidget` keeps its built pages, so a request parked for the next
    /// build would never be consumed. When the pane is already built its editor
    /// handle is live and is focused here and now; when it is not, the request is
    /// parked and the page takes it as it mounts.
    pub fn activate(&self, item_id: u64, title: &str, ctx: &mut teksilo::prelude::EventContext) {
        self.open_in(Side::Primary, item_id, title);
        self.focus_main_editor(Side::Primary, item_id, ctx);
    }

    /// [`Self::activate`], into the side pane: Ctrl+Enter, a middle-click, and both
    /// "Open to the Side" context-menu rows.
    pub fn activate_to_side(
        &self,
        item_id: u64,
        title: &str,
        ctx: &mut teksilo::prelude::EventContext,
    ) {
        self.set_split(true);
        self.open_in(Side::Secondary, item_id, title);
        self.focus_main_editor(Side::Secondary, item_id, ctx);
    }

    /// Focus now if the pane is built, park the request if it is not.
    ///
    /// A tab with no editor at all (a placeholder combination, a container sitting
    /// on its Corkboard or Overview) parks a request no page there can honour. That
    /// is deliberate and bounded: `RememberSegment` stands the request down the
    /// moment the writer chooses a different segment, so it can only ever be taken
    /// by the page they were heading for.
    fn focus_main_editor(
        &self,
        side: Side,
        item_id: u64,
        ctx: &mut teksilo::prelude::EventContext,
    ) {
        // Resolved before focusing, not inside the closure: `with_tab` takes a
        // `Fn(&ContentTab)` and `EditorHandle::focus` needs `&mut EventContext`.
        let handle = self
            .with_tab(side, item_id, |t| match t.view_state_ports().editor() {
                Some(handle) => Some(handle),
                None => {
                    t.request_focus();
                    None
                }
            })
            .flatten();
        if let Some(handle) = handle {
            handle.focus(ctx);
        }
    }

    /// The durable uid of an item, by a single-entity read rather than a walk of
    /// the whole binder. It is the same read `sync_active_item` already pays on
    /// every focus change.
    fn uid_of(&self, item_id: u64) -> Option<Uuid> {
        binder_ops::item_dto(&self.app_ctx, item_id).map(|dto| dto.uid)
    }

    /// Seed a freshly opened tab from the project's remembered positions.
    ///
    /// A no-op during a workspace restore in the sense that matters: `restore`
    /// seeds each tab explicitly straight afterwards, from the **per-pane** record,
    /// which is the more specific of the two and so wins by simply landing second.
    fn seed_from_memory(&self, side: Side, item_id: u64) {
        let Some(states) = self.item_view_states.borrow().clone() else {
            return;
        };
        let Some(uid) = self.uid_of(item_id) else {
            return;
        };
        let Some(remembered) = states.get(uid) else {
            return;
        };
        self.with_tab(side, item_id, |t| {
            t.seed_remembered_view_state(crate::shared::ViewState {
                caret: remembered.caret,
                scroll: remembered.scroll,
            });
            t.seed_segment(&remembered.segment);
        });
    }

    /// Write a tab's live position into the project's remembered positions.
    fn remember_position(&self, tab: &ContentTab) {
        let Some(states) = self.item_view_states.borrow().clone() else {
            return;
        };
        let Some(uid) = self.uid_of(tab.item_id()) else {
            return;
        };
        let live = tab.capture_view_state();
        states.record(TabViewState {
            uid,
            caret: live.caret,
            scroll: live.scroll,
            // The Corkboard's own navigation stays per pane, in `PaneLayout`: its
            // trail is a list of store ids that only mean anything against the tab
            // that walked it, and translating one here would need the whole item
            // stream for a piece of state a reopened tab is happy to start fresh on.
            corkboard: Default::default(),
            segment: tab.segment_shown(),
        });
    }

    /// Set the segment a **not-yet-built** tab will open on, by its string id. The
    /// workspace restore's counterpart of [`Self::seed_view_state`].
    pub fn seed_segment(&self, side: Side, item_id: u64, segment: &str) {
        self.with_tab(side, item_id, |t| t.seed_segment(segment));
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
                        h.payload.downcast_ref::<ContentTab>().map(|t| {
                            // Where the writer was, before the tab goes. Collapsing the
                            // split is a close the writer performed, exactly as clicking
                            // a tab's cross is, so it has to write the position down the
                            // same way `close_tab` does. It is a separate teardown path
                            // and the `ContentTab` is unreachable after `tabs.clear()`,
                            // so this cannot simply defer to that one.
                            self.remember_position(t);
                            t.item_id()
                        })
                    })
                    .flatten()
            })
            .collect();
        pane.tabs.clear();
        pane.pinned.borrow_mut().clear();
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

    // ── Tab identity (the context menu's questions) ──────────────────────────

    /// Which pane holds `tab_id`, if either still does.
    ///
    /// The tab-strip context menu resolves the side through this **at the moment
    /// it opens**, never at the moment the tab was created. A `Side` captured in
    /// the menu factory goes stale the instant the tab is dragged across:
    /// [`Self::receive_tab`] pushes the *same* `TabHandle` — payload, info and
    /// menu factory intact — into the other pane, so "Close others" on a migrated
    /// tab would silently empty the pane it came from.
    pub fn side_of(&self, tab_id: TabId) -> Option<Side> {
        [Side::Primary, Side::Secondary]
            .into_iter()
            .find(|&side| self.index_of(side, tab_id).is_some())
    }

    /// The `BinderItem` id behind a tab in `side`, if it is an editor tab.
    pub fn item_of(&self, side: Side, tab_id: TabId) -> Option<u64> {
        self.item_of_tab(side, tab_id)
    }

    /// The reverse: the tab showing `item_id` in `side`, if one is open.
    pub fn tab_id_of_item(&self, side: Side, item_id: u64) -> Option<TabId> {
        self.find_open(side, item_id)
    }

    /// The item id behind a tab **plus its stored title** — what the "open it
    /// over there" rows need, since [`Self::open_in`] takes a title to caption the
    /// new tab with. Read from the store rather than from the tab's own caption,
    /// which may already carry a generated chapter number that `caption` would
    /// then apply a second time.
    pub fn tab_target(&self, side: Side, tab_id: TabId) -> Option<(u64, String)> {
        let item_id = self.item_of_tab(side, tab_id)?;
        let dto = binder_ops::item_dto(&self.app_ctx, item_id)?;
        Some((item_id, dto.title))
    }

    /// The caption a tab is currently showing, for the menu's header row.
    ///
    /// Mirrors [`Self::caption_of`] — a named item shows its name, an unnamed one
    /// its generated chapter label, and anything else "Untitled" — so the row can
    /// never disagree with the tab it names.
    pub fn tab_caption(&self, side: Side, tab_id: TabId) -> Option<String> {
        let item_id = self.item_of_tab(side, tab_id)?;
        let dto = binder_ops::item_dto(&self.app_ctx, item_id)?;
        if !dto.title.trim().is_empty() {
            return Some(dto.title);
        }
        Some(match self.names().and_then(|n| n.generated_name(&dto)) {
            Some(generated) => generated,
            // Resolved eagerly: the caller renders it as `lit!` data (a document
            // title is never translated), and the menu is rebuilt from scratch on
            // every right-click, so it can never outlive a locale change.
            None => tr!(untitled()).resolve_now(),
        })
    }

    // ── Pinning ─────────────────────────────────────────────────────────────

    /// Is this tab pinned?
    pub fn is_pinned(&self, side: Side, tab_id: TabId) -> bool {
        self.item_of_tab(side, tab_id)
            .is_some_and(|item_id| self.pane(side).pinned.borrow().contains(&item_id))
    }

    /// The `BinderItem` **store ids** of `side`'s pinned tabs, in tab order.
    ///
    /// Walks the tab model and keeps the members, rather than reading the set
    /// out: the model is the truth for order and for liveness, so a stale entry
    /// for an item that is no longer open here can never reach the workspace
    /// capture. Always a prefix of [`Self::tab_item_ids`].
    pub fn pinned_item_ids(&self, side: Side) -> Vec<u64> {
        let pinned = self.pane(side).pinned.borrow();
        self.tab_item_ids(side)
            .into_iter()
            .filter(|id| pinned.contains(id))
            .collect()
    }

    /// Mark these items pinned in `side` and re-establish the pinned prefix.
    /// Ids that are not open in the pane are ignored — the workspace restore
    /// hands over what it remembered, which may name an item that no longer opens.
    pub fn seed_pinned(&self, side: Side, item_ids: &[u64]) {
        let open: HashSet<u64> = self.tab_item_ids(side).into_iter().collect();
        let mut added: Vec<u64> = Vec::new();
        {
            let mut pinned = self.pane(side).pinned.borrow_mut();
            for id in item_ids {
                if open.contains(id) && pinned.insert(*id) {
                    added.push(*id);
                }
            }
        }
        // The **presentation**, which membership alone does not carry: a restored
        // tab was opened by `open_in` as an ordinary one, and `enforce_pin_order`
        // only moves handles — it never rewrites a `TabInfo`. Without this a
        // remembered pin came back with its close cross, its middle-click close
        // and its Delete key, while the menu and both bulk closes still treated it
        // as pinned; one careless click then destroyed the pin for good.
        //
        // `redraw_pin`, not `repin_tab`: repainting must not move anything here.
        // Each `repin_tab` moves its tab to the boundary computed from the *whole*
        // seeded set, which reorders the restored pins relative to the order they
        // were captured in. The single `enforce_pin_order` below settles the order
        // once, preserving it. Also strictly after the `borrow_mut` above ends —
        // `redraw_pin` reads the same cell.
        for id in added {
            if let Some(tab_id) = self.find_open(side, id) {
                self.redraw_pin(side, tab_id, true);
            }
        }
        self.enforce_pin_order(side);
    }

    /// Pin a tab: it loses its close affordances, takes the drawing-pin glyph, and
    /// moves to the end of its pane's pinned run.
    pub fn pin(&self, side: Side, tab_id: TabId) {
        let Some(item_id) = self.item_of_tab(side, tab_id) else {
            return;
        };
        if !self.pane(side).pinned.borrow_mut().insert(item_id) {
            return; // already pinned — nothing to redraw, nothing to move
        }
        self.repin_tab(side, tab_id, true);
    }

    /// Unpin a tab: it gets its cross back and parks immediately after whatever
    /// pinned tabs remain.
    pub fn unpin(&self, side: Side, tab_id: TabId) {
        let Some(item_id) = self.item_of_tab(side, tab_id) else {
            return;
        };
        if !self.pane(side).pinned.borrow_mut().remove(&item_id) {
            return;
        }
        self.repin_tab(side, tab_id, false);
    }

    /// Flip a tab's pin — the single context-menu row, which shows whichever of
    /// Pin/Unpin applies.
    pub fn toggle_pin(&self, side: Side, tab_id: TabId) {
        if self.is_pinned(side, tab_id) {
            self.unpin(side, tab_id);
        } else {
            self.pin(side, tab_id);
        }
    }

    /// Repaint one tab for its pin state, **without moving it**.
    ///
    /// Rebuilds the whole `TabInfo` through [`Self::tab_info`] rather than
    /// patching a clone, so that pinning and unpinning are exact inverses.
    /// Patching cannot be: `TabInfo` offers no way to *clear* a tooltip (its
    /// three tooltip setters only clear one another), so an unpin that patched a
    /// clone would carry the "Pinned — Close others and Close all leave it open"
    /// tooltip onto a tab that is no longer pinned and does have a cross.
    ///
    /// The `TabId` is kept, so the `TabWidget`'s per-id pane memoization keeps
    /// the live editor — its caret, its scroll, its segment — mounted throughout.
    ///
    /// The caption is re-derived exactly as [`Self::refresh_captions`] does. When
    /// the store cannot answer for the item — mid-teardown, or a fixture with no
    /// manuscript behind it — this falls back to patching the clone, on the same
    /// principle that function states: only ever *correct* a caption, never blank
    /// one. The stale-tooltip case is the price, and it is unreachable in a live
    /// project.
    fn redraw_pin(&self, side: Side, tab_id: TabId, pinned: bool) {
        let pane = self.pane(side);
        let Some(idx) = self.index_of(side, tab_id) else {
            return;
        };
        let Some(mut handle) = pane.tabs.with_item(idx, |h| h.clone()) else {
            return;
        };
        let Some(Some((item_id, sub_role, trashed))) = pane.tabs.with_item(idx, |h| {
            h.payload.downcast_ref::<ContentTab>().map(|t| {
                (
                    t.item_id(),
                    t.sub_role().clone(),
                    t.open_doc.trashed.clone(),
                )
            })
        }) else {
            return;
        };
        let caption = self
            .names()
            .zip(binder_ops::item_dto(&self.app_ctx, item_id))
            .map(|(names, it)| Self::caption_of(&it.title, names.generated_name(&it)));
        handle.info = match caption {
            Some(caption) => self.tab_info(tab_id, caption, &sub_role, &trashed, pinned),
            None => {
                // No caption to rebuild with: keep the one on screen and patch the
                // two fields that are safe to patch.
                let icon_color = trashed.map(|t| {
                    if *t {
                        TextRole::Warning
                    } else {
                        TextRole::Primary
                    }
                });
                handle.info.clone().closable(!pinned).icon(move || {
                    if pinned {
                        crate::icons::editor::pinned_tab().color(icon_color.clone())
                    } else {
                        crate::binder::icons::sub_role_icon(&sub_role).color(icon_color.clone())
                    }
                })
            }
        };
        pane.tabs.set(idx, handle);
    }

    /// Repaint a tab for its new pin state **and** move it to the pinned
    /// boundary — what the Pin/Unpin menu row and the shortcut do.
    ///
    /// Split from [`Self::redraw_pin`] because the workspace restore needs the
    /// repaint half alone: it seeds every remembered pin at once and then settles
    /// the order in a single [`Self::enforce_pin_order`], where moving each tab to
    /// the boundary as it is repainted would shuffle the restored pins out of the
    /// order they were captured in.
    fn repin_tab(&self, side: Side, tab_id: TabId, pinned: bool) {
        self.redraw_pin(side, tab_id, pinned);
        // Where it belongs now. Pinning inserts at the end of the run *as it was
        // before this tab joined it*; unpinning parks just past whatever is left.
        let target = if pinned {
            self.pin_count(side).saturating_sub(1)
        } else {
            self.pin_count(side)
        };
        if let Some(from) = self.index_of(side, tab_id)
            && from != target
        {
            self.pane(side).tabs.move_item(from, target);
        }
    }

    /// How many of `side`'s open tabs are pinned.
    fn pin_count(&self, side: Side) -> usize {
        self.pinned_item_ids(side).len()
    }

    /// Re-establish "pinned tabs are a prefix of the pane's tab list" in one
    /// `replace_all`, preserving the relative order within each group.
    ///
    /// The invariant matters for two reasons that have nothing to do with looks.
    /// The workspace capture persists `tab_item_ids`, i.e. **model** order, and
    /// teksilo's arrow-key tab navigation walks the model too — so a pinned tab
    /// sitting in the middle of the model would be remembered in the wrong place
    /// and would send keyboard focus backwards. (Teksilo's own pinned strip
    /// re-partitions at render time and makes model order diverge from visual
    /// order; not setting `TabInfo::pinned` is what keeps the two identical here.)
    fn enforce_pin_order(&self, side: Side) {
        let pane = self.pane(side);
        let pinned = pane.pinned.borrow().clone();
        let mut head: Vec<TabHandle> = Vec::new();
        let mut tail: Vec<TabHandle> = Vec::new();
        for i in 0..pane.tabs.len() {
            let Some((handle, item)) = pane.tabs.with_item(i, |h| {
                (
                    h.clone(),
                    h.payload.downcast_ref::<ContentTab>().map(|t| t.item_id()),
                )
            }) else {
                continue;
            };
            if item.is_some_and(|id| pinned.contains(&id)) {
                head.push(handle);
            } else {
                tail.push(handle);
            }
        }
        if head.is_empty() {
            return;
        }
        // One `replace_all` (a single `Reset`), not N `move_item`s: the surviving
        // handles keep their `TabId`s, so the `TabWidget`'s per-id pane
        // memoization keeps every mounted editor exactly where it was.
        head.extend(tail);
        pane.tabs.replace_all(head);
        // A pin change never changes which tab is selected, and `replace_all`
        // leaves the selection signal alone, so there is deliberately no
        // `selected.set` here.
    }

    /// A pane's `TabWidget::on_reorder` hook, clamping a drag to the correct side
    /// of the pinned boundary.
    ///
    /// Installing this **replaces** teksilo's default handler, which is a bare
    /// `model.move_item` — so the move has to be performed here or drag-reorder
    /// silently stops working altogether.
    pub fn reorder_in(&self, side: Side, tab_id: TabId, to: usize) {
        let pane = self.pane(side);
        let Some(from) = self.index_of(side, tab_id) else {
            return;
        };
        let pins = self.pin_count(side);
        let pinned = self.is_pinned(side, tab_id);
        // A pinned tab may be reordered within the pinned run and an unpinned one
        // within the rest; neither may cross. Clamping (rather than refusing)
        // keeps the drag feeling live: the tab follows the pointer and stops at
        // the boundary instead of snapping back.
        let to = if pinned {
            to.min(pins.saturating_sub(1))
        } else {
            to.max(pins)
        };
        let to = to.min(pane.tabs.len().saturating_sub(1));
        if from != to {
            pane.tabs.move_item(from, to);
        }
    }

    // ── Bulk close ──────────────────────────────────────────────────────────

    /// Close every tab in `side` except `keep` — and except every pinned tab.
    pub fn close_others_in(&self, side: Side, keep: TabId) {
        self.close_many(side, |tab_id, _| tab_id == keep);
    }

    /// Close every tab in `side` except the pinned ones.
    ///
    /// Deliberately **not** [`Self::close_all`], which empties *both* panes and
    /// calls `docs.clear()` without flushing — that is the project-teardown path,
    /// and pointing a menu row at it would drop a sibling window's live documents
    /// along with any keystroke not yet written back.
    pub fn close_all_in(&self, side: Side) {
        self.close_many(side, |_, _| false);
    }

    /// The shared body of the two bulk closes: flush and release everything the
    /// `keep` predicate rejects, in **one** model write and **one** selection fire.
    ///
    /// Not an N-iteration `close_in` loop, for the reasons [`Self::drain_pane`]
    /// spells out: every `selected.set` synchronously re-fires `App`'s per-pane
    /// effect, which runs `flush_all` over the whole store — so closing eleven
    /// tabs would mean eleven full-store flushes — and an emptying side pane would
    /// trip `set_split(false)` (hence `drain_pane`) part-way through, destroying
    /// the very list being walked.
    ///
    /// `keep` is also handed the item id so a caller can reason about the
    /// document rather than the tab; a pinned tab is spared before it is asked.
    fn close_many(&self, side: Side, keep: impl Fn(TabId, Option<u64>) -> bool) {
        let stack = self.ids.stack_id.get();
        let pane = self.pane(side);
        let pinned = pane.pinned.borrow().clone();
        // Collect first, mutate after: `with_item` holds the model's `borrow()`
        // for the whole call, so any write from inside it is an immediate
        // `RefCell` panic.
        let mut survivors: Vec<TabHandle> = Vec::new();
        let mut closing: Vec<u64> = Vec::new();
        for i in 0..pane.tabs.len() {
            let Some((handle, item)) = pane.tabs.with_item(i, |h| {
                (
                    h.clone(),
                    h.payload.downcast_ref::<ContentTab>().map(|t| t.item_id()),
                )
            }) else {
                continue;
            };
            let is_pinned = item.is_some_and(|id| pinned.contains(&id));
            if is_pinned || keep(handle.id, item) {
                survivors.push(handle);
                continue;
            }
            // Flush and write down where the writer was *before* the handle goes:
            // the `ContentTab` is unreachable once the model is replaced, and a
            // bulk close is a close the writer performed exactly as clicking a
            // cross is — the gap `collapsing_the_split_records_the_side_tabs_position`
            // was written for.
            pane.tabs.with_item(i, |h| {
                if let Some(t) = h.payload.downcast_ref::<ContentTab>() {
                    let _ = t.flush(stack);
                    self.remember_position(t);
                }
            });
            if let Some(id) = item {
                closing.push(id);
            }
        }
        if closing.is_empty() {
            return;
        }
        let selected = pane.selected.get();
        let survives = survivors.iter().any(|h| Some(h.id) == selected);
        let next = if survives {
            selected
        } else {
            survivors.first().map(|h| h.id)
        };
        pane.tabs.replace_all(survivors);
        for id in &closing {
            pane.pinned.borrow_mut().remove(id);
        }
        // One selection write, and only when it actually changes.
        if next != selected {
            pane.selected.set(next);
        }
        for id in closing {
            self.docs.release(id, stack); // flushes + evicts on the last reference
        }
        if side == Side::Secondary && self.secondary.tabs.is_empty() {
            self.set_split(false);
        }
        self.sync_active_item();
    }

    // ── Move / open across panes ────────────────────────────────────────────

    /// Open this tab's item in the *other* pane as well, leaving this one alone —
    /// the menu's "Open to the Side" / "Open in the main pane" row.
    ///
    /// A duplicate, not a move: the store hands both tabs one live document, so
    /// the two panes show the same prose side by side. That is why this goes
    /// through `activate*` (which take a second `docs.open` reference) and
    /// [`Self::move_tab_to`] pointedly does not.
    pub fn open_other_side(
        &self,
        side: Side,
        tab_id: TabId,
        ctx: &mut teksilo::prelude::EventContext,
    ) {
        let Some((item_id, title)) = self.tab_target(side, tab_id) else {
            return;
        };
        match side {
            Side::Primary => self.activate_to_side(item_id, &title, ctx),
            Side::Secondary => self.activate(item_id, &title, ctx),
        }
    }

    /// Move a tab from one pane to the other — the menu's "Move to the side" /
    /// "Move to the main pane" row.
    ///
    /// Composes the framework's own two drag hooks in the framework's own order
    /// (receive, then remove), so the menu path and the drag path cannot diverge:
    /// [`Self::receive_tab`] takes the existing handle — which already holds the
    /// document reference — and [`Self::transfer_out`] removes it here **without**
    /// releasing. Calling `docs.open` anywhere in here would leak a reference that
    /// nothing gives back, keeping the document resident and its edits unflushed
    /// for the rest of the session.
    pub fn move_tab_to(&self, from: Side, to: Side, tab_id: TabId) {
        if from == to {
            return;
        }
        let Some(handle) = self.handle_of(from, tab_id) else {
            return;
        };
        // Reveal the split first: `receive_tab` does not, so a tab moved into a
        // hidden side pane would simply vanish from the writer's view.
        if to == Side::Secondary {
            self.set_split(true);
        }
        // Remember where the writer was before the handle changes panes — a
        // `receive_tab` that dedups drops this tab, and its position with it.
        self.remember_position_of(from, tab_id);
        // The pin carry and the stale-entry prune live in these two, so that the
        // drag path gets both for free.
        self.receive_tab(to, handle);
        self.transfer_out(from, tab_id);
        self.enforce_pin_order(to);
        // Focus lands on the destination, and has to be re-asserted **after**
        // `transfer_out`: removing the tab from the source pane rewrites that
        // pane's `selected`, which synchronously re-fires `focus_sync`'s per-pane
        // effect and hands the focused side back to the pane the tab just left.
        // The writer would then be looking at the moved document while every
        // focus-following surface — the binder's open-item marker, Format, the
        // Go menu — still answered about the other pane, and an emptied source
        // pane would leave `active_item` at `None` with a document plainly on
        // screen.
        // Guarded on the tab actually being there: `receive_tab` *dedups* when the
        // destination already showed this item, dropping the incoming handle and
        // selecting the existing tab — selecting `tab_id` then would name a tab
        // that no longer exists anywhere. Its own selection is already right, so
        // only the focused side needs re-asserting.
        if self.index_of(to, tab_id).is_some() {
            self.pane(to).selected.set(Some(tab_id));
        }
        if !self.pane(to).tabs.is_empty() {
            self.set_focused(to);
        }
    }

    /// Write down where the writer is in one tab, by tab id — what the cross-pane
    /// and cross-window moves call before the tab stops existing.
    pub fn remember_position_of(&self, side: Side, tab_id: TabId) {
        let pane = self.pane(side);
        let Some(idx) = self.index_of(side, tab_id) else {
            return;
        };
        pane.tabs.with_item(idx, |h| {
            if let Some(t) = h.payload.downcast_ref::<ContentTab>() {
                self.remember_position(t);
            }
        });
    }

    /// Open an item by id alone, resolving its title from the store — the entry
    /// point a freshly attached window uses to open the one tab it was told to
    /// show, where no caller is around to supply a title.
    pub fn open_by_id(&self, side: Side, item_id: u64) {
        let Some(dto) = binder_ops::item_dto(&self.app_ctx, item_id) else {
            return;
        };
        if side == Side::Secondary {
            self.set_split(true);
        }
        self.open_in(side, item_id, &dto.title);
    }

    /// A clone of the handle at `tab_id` in `side`.
    fn handle_of(&self, side: Side, tab_id: TabId) -> Option<TabHandle> {
        let idx = self.index_of(side, tab_id)?;
        self.pane(side).tabs.with_item(idx, |h| h.clone())
    }

    /// The model index of `tab_id` in `side`.
    fn index_of(&self, side: Side, tab_id: TabId) -> Option<usize> {
        let pane = self.pane(side);
        (0..pane.tabs.len()).find(|&i| pane.tabs.with_item(i, |h| h.id) == Some(tab_id))
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
                                // And write down where the writer was, so reopening
                                // this item comes back to it. Gated on the same flag
                                // for the same reason: a hard-removed item has no
                                // position worth keeping, and the uid this would key
                                // on no longer resolves. `close_all` never reaches
                                // here at all, which is what keeps a project switch
                                // from recording against a half-torn-down store.
                                self.remember_position(t);
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
            // Hygiene, not correctness: `pinned_item_ids` reads the tab model, so
            // an entry left behind here could never reach the workspace capture
            // anyway — but it *would* silently re-pin the tab if the same item
            // were reopened in this pane later.
            pane.pinned.borrow_mut().remove(&item_id);
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
            // Before the removal, while the payload is still reachable: the pin
            // has already been carried to the receiving pane by `receive_tab`, so
            // what is left here is a stale entry that would silently re-pin this
            // item if it were ever reopened in this pane.
            if let Some(Some(item_id)) = pane.tabs.with_item(p, |h| {
                h.payload.downcast_ref::<ContentTab>().map(|t| t.item_id())
            }) {
                pane.pinned.borrow_mut().remove(&item_id);
            }
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
        // Read the pin off the pane it is leaving **now**, while that entry is
        // still populated — `transfer_out` runs after this, per teksilo's own
        // receive-then-remove order. Doing the carry here rather than in
        // [`Self::move_tab_to`] is what makes a dragged tab and a menu-moved tab
        // behave identically; there are only two panes, so "the other one" is the
        // source by construction.
        let was_pinned =
            item_id.is_some_and(|id| self.pane(side.other()).pinned.borrow().contains(&id));
        if let Some(item_id) = item_id
            && let Some(existing) = self.find_open(side, item_id)
        {
            // Dedup: this pane already shows the item, so the incoming tab is
            // dropped. Its pin goes with it — the surviving tab's own pin state is
            // its own business and must not be overwritten by the arrival.
            self.docs.release(item_id, self.ids.stack_id.get());
            self.pane(side).selected.set(Some(existing));
            self.set_focused(side);
            return;
        }
        let id = handle.id;
        self.pane(side).tabs.push(handle);
        if was_pinned && let Some(item_id) = item_id {
            self.pane(side).pinned.borrow_mut().insert(item_id);
            self.repin_tab(side, id, true);
        }
        self.pane(side).selected.set(Some(id));
        self.set_focused(side);
    }

    // ── Flush / save / reset ────────────────────────────────────────────────

    /// Persist every open document's edits back to its `Content` rows (changed
    /// fields only), through the per-Work undo stack. Each shared doc flushed once.
    /// The store of open documents this window edits — for a caller that must
    /// react to a `Content` row changing underneath an open tab.
    pub fn open_docs(&self) -> OpenDocsStore {
        self.docs.clone()
    }

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
    /// This project's durable `Work.unique_id`, or `None` when it has none.
    ///
    /// The same answer, read the same way, as [`crate::tabs::ContentTab::work_unique_id`]:
    /// through the store off `ids.work_id`, because the uid is set when the
    /// project is first saved and this view-model long predates that. `None` is
    /// an **unsaved** project, which is the absence of an identity rather than
    /// one more identity, and every per-project store in the workspace refuses
    /// it rather than pooling them.
    pub fn work_unique_id(&self) -> Option<String> {
        let work_id = self.ids.work_id.get()?;
        let uid = frontend::commands::work_commands::get_work(&self.app_ctx, &work_id)
            .ok()
            .flatten()?
            .unique_id;
        (!uid.is_empty()).then_some(uid)
    }

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
        self.primary.pinned.borrow_mut().clear();
        self.secondary.pinned.borrow_mut().clear();
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
                let was_selected = pane.selected.get() == Some(h.id);
                let new_id = TabId::fresh();
                // The pin is keyed by item id precisely so it survives here: a
                // retype mints a fresh `TabId` and a fresh `ContentTab`, and
                // anything keyed on either would have to be carried by hand.
                let pinned = pane.pinned.borrow().contains(&item_id);
                // `it.sub_role`, not `doc.sub_role`: a Promote is exactly the case
                // where the document's cached type is the *old* one, and the
                // freshly read DTO is the whole reason this rebuild is happening.
                let info = self.tab_info(new_id, caption, &it.sub_role, &doc.trashed, pinned);
                pane.tabs.remove(i);
                pane.tabs
                    .insert(i, TabHandle::dynamic(new_id, "editor", info, tab));
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

/// Test-only fixtures for an `EditorsViewModel` with no backend behind it.
///
/// They live **inside** this module rather than in a sibling file because they
/// reach `EditorsViewModel`'s private innards (`pane`, `docs`, `app_ctx`, the
/// settings signals) to push a tab or seed a document without a loaded project.
/// `pub(crate)` so the tab-strip menu's own tests can build the same fixture
/// instead of copying forty lines of constructor.
#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use crate::save::SaveStateViewModel;
    use crate::settings::EditorTypography;
    use crate::tabs;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    pub(crate) fn test_typography() -> EditorTypographySet {
        let bundle = |family: &str| EditorTypography {
            font_family: Signal::new(family.to_string()),
            size: Signal::new(1.0),
            line_height: Signal::new(1.5),
            first_line_indent: Signal::new(0.0),
            para_spacing_before: Signal::new(0.0),
            para_spacing_after: Signal::new(0.0),
            size_range: crate::settings::TypographySizeRange::default(),
        };
        EditorTypographySet {
            scene: bundle("Literata"),
            synopsis: bundle("Literata"),
            notes: bundle("Inter"),
            corkboard: bundle("Literata"),
            distraction_free: bundle("Literata"),
        }
    }

    pub(crate) fn editors() -> EditorsViewModel {
        let app_ctx = Rc::new(AppContext::new());
        let save_state = SaveStateViewModel::new(app_ctx.clone(), AppIds::new());
        editors_with(app_ctx, save_state)
    }

    /// Build an `EditorsViewModel` over a caller-supplied `app_ctx` + save state,
    /// so a test can construct **two** instances sharing the same
    /// `SaveStateViewModel` — modelling two windows onto one project.
    pub(crate) fn editors_with(
        app_ctx: Rc<AppContext>,
        save_state: SaveStateViewModel,
    ) -> EditorsViewModel {
        let ids = AppIds::new();
        // Built before the call: the arguments it needs are moved into earlier parameters.
        let mention_index = crate::mentions::MentionIndex::new(app_ctx.clone(), ids.clone());
        let docs = OpenDocsStore::new(app_ctx.clone());
        let tree_expansion = crate::settings::TreeExpansionViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            crate::models::TreeExpansionService::in_memory_default(),
        );
        let tags = crate::tags::TagsViewModel::detached(app_ctx.clone(), ids.clone());
        let statuses = crate::statuses::StatusesViewModel::new(app_ctx.clone(), ids.clone());
        EditorsViewModel::new(
            app_ctx,
            Signal::new(700.0),
            Signal::new(true),
            Signal::new(crate::shared::SynopsisPlacement::default()),
            Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
            test_typography(),
            crate::shared::TypewriterSettings::off(),
            crate::shared::CaretHighlightSettings::off(),
            crate::settings::EditorViewMemory::detached(false),
            crate::settings::CorkboardDefaults::detached(),
            ids,
            docs,
            Signal::new(false),
            save_state,
            Signal::new(false),
            tree_expansion,
            Signal::new(false),
            Signal::new(620.0),
            crate::go::GoAvailability::new(),
            crate::format::FormatViewModel::detached(),
            crate::writing_session::WritingGamesViewModel::detached(),
            Signal::new(GoalUnit::default()),
            tags,
            statuses,
            mention_index,
        )
    }

    /// Push a tab directly into `side` (bypassing the backend / store) so tab
    /// management can be tested without a loaded project.
    pub(crate) fn push_tab(vm: &EditorsViewModel, side: Side, item_id: u64) -> TabId {
        let id = TabId::fresh();
        let tab = tabs::tab_for(
            &vm.app_ctx,
            item_id,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            vm.column_width.clone(),
            vm.show_synopsis.clone(),
            vm.typography.clone(),
            vm.view_memory.clone(),
            &vm.ids,
        );
        vm.pane(side).tabs.push(TabHandle::dynamic(
            id,
            "editor",
            TabInfo::new().closable(true),
            tab,
        ));
        id
    }

    /// Seed the store with a Scene document for `item_id`, bypassing the backend.
    pub(crate) fn seed_doc(vm: &EditorsViewModel, item_id: u64) {
        use crate::models::OpenDoc;
        vm.docs.insert_for_test(Rc::new(OpenDoc::build(
            &vm.app_ctx,
            item_id,
            &BinderItemRole::Item,
            &BinderItemSubRole::Scene,
            &[],
            Signal::new(0),
            std::path::Path::new(""),
        )));
    }

    /// The `AppContext` a fixture view-model was built over.
    pub(crate) fn app_ctx_of(vm: &EditorsViewModel) -> Rc<AppContext> {
        vm.app_ctx.clone()
    }

    /// The `AppIds` a fixture view-model was built over.
    pub(crate) fn ids_of(vm: &EditorsViewModel) -> AppIds {
        vm.ids.clone()
    }

    /// Push a Scene tab and seed its document in one step — what a menu test
    /// wants, since every row it exercises acts on a real open document.
    pub(crate) fn push_scene_tab(vm: &EditorsViewModel, side: Side, item_id: u64) -> TabId {
        seed_doc(vm, item_id);
        push_tab(vm, side, item_id)
    }

    /// A numbering manuscript with one binder, and `vm` pointed at it.
    ///
    /// Not available under `--features mocks`: the mock models answer from
    /// fabricated data with no store behind them, so there is nothing to create
    /// a `Work` in. Every test that calls this is gated the same way, and says so.
    #[cfg(not(feature = "mocks"))]
    pub(crate) fn seed_work(vm: &EditorsViewModel) -> u64 {
        use frontend::commands::{binder_commands, work_commands};
        use frontend::direct_access::{CreateBinderDto, CreateWorkDto};

        let work = work_commands::create_orphan_work(
            &vm.app_ctx,
            None,
            &CreateWorkDto {
                statuses: Vec::new(),
                number_chapters: true,
                dict_language: vec!["en".into()],
                ..Default::default()
            },
        )
        .expect("the in-memory store always accepts an orphan Work");
        vm.ids.work_id.set(Some(work.id));
        binder_commands::create_binder(
            &vm.app_ctx,
            None,
            &CreateBinderDto {
                name: "Manuscript".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            0,
        )
        .expect("the in-memory store always accepts a binder")
        .id
    }

    /// Append an item into `binder`. See [`seed_work`] for the mocks gate.
    #[cfg(not(feature = "mocks"))]
    pub(crate) fn seed_item(
        vm: &EditorsViewModel,
        binder: u64,
        title: &str,
        sub_role: frontend::common::entities::BinderItemSubRole,
    ) -> u64 {
        use frontend::commands::binder_item_commands;
        use frontend::direct_access::CreateBinderItemDto;

        binder_item_commands::create_binder_item(
            &vm.app_ctx,
            None,
            &CreateBinderItemDto {
                status: None,
                title: title.into(),
                role: BinderItemRole::Item,
                sub_role,
                activated: true,
                is_exportable: true,
                ..Default::default()
            },
            binder,
            -1,
        )
        .expect("the in-memory store always accepts a binder item")
        .id
    }

    /// Drive `f` with a real `&mut EventContext`, the same way a click in the
    /// outline or an Overview row reaches `activate`/`activate_to_side` in
    /// production, never a bypassing direct call.
    ///
    /// Hanging the closure off a button and clicking it is the only way to get an
    /// `EventContext` headlessly. Neither method subscribes to backend events, so
    /// a bare `WidgetTree` (no event source registered) is enough to host it.
    pub(crate) fn with_event_context(f: impl Fn(&mut teksilo::prelude::EventContext) + 'static) {
        use teksilo::core::widget_tree::WidgetTree;
        use teksilo::widgets::Button;

        let mut tree = WidgetTree::new();
        let trigger = tree.add(Button::new(lit!("go")).on_activate_fn(f));
        tree.layout(teksilo::prelude::SizeProposal::exact(200.0, 60.0));
        crate::test_support::click(&mut tree, trigger);
    }
}

#[cfg(test)]
mod tests;
