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
use std::rc::Rc;
use std::time::Instant;

use bastyde::data::ListModel;
use bastyde::prelude::*; // Signal, tr!, lit!
use skribisto_model::SubRoleExt;
use skribisto_model::scene_break::{self, SceneBreakTier};
use bastyde::widgets::{Orientation, PaneDescriptor, SplitterModel, TabHandle, TabId, TabInfo};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::direct_access::BinderItemDto;
use frontend::work_management::SaveWorkDto;

use frontend::common::event::Event;

use crate::app_ids::AppIds;
use crate::models::OpenDocsStore;
use crate::singles::SingleBinderItem;
use crate::tabs::ContentTab;

use super::long_op::{event_id, parse_payload};
use super::save_queue::{SaveQueue, SaveRequest};
use crate::view_models::EditorTypographySet;

/// A `save_work` of ours landed — what `App` needs to release whatever was waiting
/// on it (a deferred close, a parked project switch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaveLanded {
    /// Everything mutated up to this edit sequence is now on disk.
    pub saved_seq: u64,
    /// Edits had arrived during that save, so a follow-up was due — and it could
    /// **not** be issued. Nothing further is coming: no further completion, no
    /// failure event (there is no operation to fail). Anything still parked on a
    /// sequence beyond [`Self::saved_seq`] would wait forever, so `App` must drop it
    /// and say so, exactly as it does for an outright failure.
    pub follow_up_failed: bool,
}

/// Which editor pane. `Primary` is always present; `Secondary` is the side pane,
/// revealed by the split view.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    Primary,
    Secondary,
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
    /// Whether the focused pane's active tab edits a scene's own prose — the
    /// live form of [`Self::focused_carries_scene`], for menu enablement. Kept
    /// as a signal (not a derived map) because it is computed by walking the
    /// tab list, which is not itself in the signal graph.
    scene_focused: Signal<bool>,
    column_width: Signal<f32>,
    show_synopsis: Signal<bool>,
    typography: EditorTypographySet,
    /// Per-container-type "last view" memory, threaded into every `ContentTab`.
    view_memory: crate::view_models::EditorViewMemory,
    /// Corkboard default presentation, threaded into every container `ContentTab`.
    corkboard_defaults: crate::view_models::CorkboardDefaults,
    /// Id-only global state (work + undo-stack ids); write-back lands on
    /// `ids.stack_id` so it shares the tree edits' Ctrl+Z history.
    ids: AppIds,
    /// The shared holder of open documents (app-state clone), refcounted per item.
    docs: OpenDocsStore,
    /// `true` while a *backup file* is open: disk saves are inert (the file is
    /// read-only; the content stays editable and can only be kept via Save As).
    backup_mode: Signal<bool>,
    /// Monotonic edit sequence, bumped by `App` on **every** mutation (typing, tree
    /// edits, metadata). A save started after a flush at seq *n* covers *n*.
    dirty_seq: Signal<u64>,
    /// The highest edit sequence actually written to disk. With [`Self::dirty_seq`]
    /// this is the truth behind "unsaved" — `dirty_seq > saved_seq` — and what the
    /// deferred close/switch guards wait on.
    saved_seq: Signal<u64>,
    /// A `save_work` is in flight (or a follow-up is about to be). Drives the save
    /// indicator; read by the guards.
    saving: Signal<bool>,
    /// One `save_work` at a time, with coalescing — see [`SaveQueue`].
    queue: Rc<RefCell<SaveQueue>>,
}

impl EditorsViewModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        column_width: Signal<f32>,
        show_synopsis: Signal<bool>,
        typography: EditorTypographySet,
        view_memory: crate::view_models::EditorViewMemory,
        corkboard_defaults: crate::view_models::CorkboardDefaults,
        ids: AppIds,
        docs: OpenDocsStore,
        backup_mode: Signal<bool>,
        dirty_seq: Signal<u64>,
        // Shared with the title-bar's Format menu — see `scene_focused_signal`.
        scene_focused: Signal<bool>,
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
            scene_focused,
            column_width,
            show_synopsis,
            typography,
            view_memory,
            corkboard_defaults,
            ids,
            docs,
            backup_mode,
            dirty_seq,
            saved_seq: Signal::new(0),
            saving: Signal::new(false),
            queue: Rc::new(RefCell::new(SaveQueue::default())),
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
        let carries = self.focused_carries_scene();
        if self.scene_focused.get() != carries {
            self.scene_focused.set(carries);
        }
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
    pub fn find_next(&self, ctx: &mut bastyde::prelude::EventContext) {
        if let Some(find) = self.focused_find() {
            find.next(ctx);
        }
    }
    pub fn find_prev(&self, ctx: &mut bastyde::prelude::EventContext) {
        if let Some(find) = self.focused_find() {
            find.prev(ctx);
        }
    }

    /// The focused prose editor's handle — `None` when nothing is open there or
    /// the active tab has no main prose field. See
    /// [`FindViewModel::editor_handle`] for why the handle lives there.
    pub fn focused_prose_handle(&self) -> Option<bastyde::widgets::rich_text::EditorHandle> {
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
    pub fn insert_scene_break(&self, tier: SceneBreakTier, ctx: &mut bastyde::prelude::EventContext) {
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

    /// The `FindViewModel` of the focused pane's active tab — `None` when nothing
    /// is open there or the active tab has no main prose field.
    fn focused_find(&self) -> Option<crate::view_models::FindViewModel> {
        let side = self.focused_side.get();
        let pane = self.pane(side);
        let tab_id = pane.selected.get()?;
        (0..pane.tabs.len()).find_map(|i| {
            pane.tabs
                .with_item(i, |h| {
                    if h.id == tab_id {
                        h.payload
                            .downcast_ref::<ContentTab>()
                            .and_then(|t| t.find().cloned())
                    } else {
                        None
                    }
                })
                .flatten()
        })
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
        let icon_color = doc
            .trashed
            .map(|t| if *t { TextRole::Warning } else { TextRole::Primary });
        let tab = ContentTab::new(
            self.app_ctx.clone(),
            self.ids.clone(),
            self.docs.clone(),
            doc,
            self.column_width.clone(),
            self.show_synopsis.clone(),
            self.typography.clone(),
            self.view_memory.clone(),
            self.corkboard_defaults.clone(),
        );
        let tab_title = if title.is_empty() {
            tr!(untitled())
        } else {
            lit!(title.to_string())
        };
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
    /// **At most one `save_work` runs at a time** ([`SaveQueue`]): if one is in
    /// flight this queues a follow-up instead of starting a second op — two ops
    /// write the same path and their completion order is unspecified, so an older
    /// snapshot could land last and silently regress the file.
    pub fn request_save(&self) -> Option<u64> {
        if self.backup_mode.get() {
            return None;
        }
        // Flush first, then read the sequence: the store now holds everything the
        // user has typed, so a save started here covers exactly this seq.
        self.flush_all();
        let covers = self.dirty_seq.get();
        let request = self.queue.borrow_mut().request(Instant::now());
        match request {
            // A save is already running; its completion issues the follow-up that
            // will cover this request.
            SaveRequest::Queued => Some(covers),
            SaveRequest::StartNow => self.start_save(covers).then_some(covers),
        }
    }

    /// Issue `save_work` and record it as the running op. `false` if the command
    /// could not be issued at all. The store has already been flushed by the caller.
    fn start_save(&self, covers: u64) -> bool {
        match work_management_commands::save_work(
            &self.app_ctx,
            &SaveWorkDto {
                file_name: String::new(),
                overwrite: true,
            },
        ) {
            Ok(op_id) => {
                self.queue
                    .borrow_mut()
                    .started(op_id, covers, Instant::now());
                self.saving.set(true);
                true
            }
            Err(_) => {
                self.queue.borrow_mut().start_failed();
                self.saving.set(false);
                false
            }
        }
    }

    /// Route a `LongOperation::Completed`. `None` if it wasn't **our** save — a
    /// backup's, an import's or a Save As's completion is left to their own
    /// view-models.
    ///
    /// If edits arrived while that save was running, the follow-up is issued here.
    pub fn on_save_completed(&self, event: &Event) -> Option<SaveLanded> {
        let op_id = event_id(event)?;
        let done = self.queue.borrow_mut().completed(&op_id)?;
        // Everything up to `covers` is now on disk. Never move it backwards.
        if done.covers > self.saved_seq.get() {
            self.saved_seq.set(done.covers);
        }
        let mut follow_up_failed = false;
        if done.restart {
            // Edits landed while that op was in flight, and its snapshot predates
            // them. Save again, covering everything typed since.
            self.flush_all();
            let covers = self.dirty_seq.get();
            follow_up_failed = !self.start_save(covers);
        } else {
            self.saving.set(false);
        }
        Some(SaveLanded {
            saved_seq: self.saved_seq.get(),
            follow_up_failed,
        })
    }

    /// Route a `LongOperation::Failed`. `Some(error)` if it was our save: the queue
    /// goes idle and any queued follow-up is dropped. The edits are untouched —
    /// still in the store, still dirty — so nothing is lost by not retrying behind
    /// the user's back; the caller reports it.
    pub fn on_save_failed(&self, event: &Event) -> Option<String> {
        let Some(op_id) = event_id(event) else {
            return None;
        };
        if !self.queue.borrow_mut().failed(&op_id) {
            return None;
        }
        self.saving.set(false);
        Some(
            parse_payload(event)
                .and_then(|p| p.get("error").and_then(|e| e.as_str()).map(str::to_string))
                .unwrap_or_default(),
        )
    }

    /// The highest edit sequence written to disk. With `dirty_seq` (bumped on every
    /// mutation) this is the truth behind "unsaved".
    pub fn saved_seq(&self) -> Signal<u64> {
        self.saved_seq.clone()
    }

    /// Whether the store holds edits not yet on disk (`dirty_seq > saved_seq`) —
    /// the same derivation `App` publishes as `unsaved`, read synchronously (no
    /// dependence on the derived-signal effect having fired). The workspace-layout
    /// capture consults this: persisting tab **ordinals** while dirty could record
    /// positions against an in-store structure the on-disk file (what a reload
    /// resolves against) doesn't share.
    pub fn is_unsaved(&self) -> bool {
        self.dirty_seq.get() > self.saved_seq.get()
    }

    /// A disk save is in flight.
    pub fn saving(&self) -> Signal<bool> {
        self.saving.clone()
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
        self.queue.borrow_mut().reset();
        self.saving.set(false);
        self.saved_seq.set(self.dirty_seq.get());
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
    }

    // ── Internals ───────────────────────────────────────────────────────────

    fn pane(&self, side: Side) -> &Pane {
        match side {
            Side::Primary => &self.primary,
            Side::Secondary => &self.secondary,
        }
    }

    /// React to `BinderItem::Updated` for `item_ids`: re-title any open tab whose item was
    /// renamed, and **rebuild** any whose *type* changed.
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
            if let Some(doc) = self.docs.peek(*id) {
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
                continue; // the rebuilt tab already carries the new caption
            }
            self.retitle(*id, &it.title);
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
                let tab = ContentTab::new(
                    self.app_ctx.clone(),
                    self.ids.clone(),
                    self.docs.clone(),
                    doc.clone(),
                    self.column_width.clone(),
                    self.show_synopsis.clone(),
                    self.typography.clone(),
                    self.view_memory.clone(),
                    self.corkboard_defaults.clone(),
                );
                let caption = if it.title.is_empty() {
                    tr!(untitled())
                } else {
                    lit!(it.title.clone())
                };
                let sub_role = it.sub_role.clone();
                // A **fresh** `TabId`, and remove+insert rather than `set`. The
                // `TabWidget` keys its mounted content widget by tab id, so swapping the
                // payload under the same id updates the strip but leaves the old editor on
                // screen — a chapter's segments for what is now a Part. A new id makes it a
                // new tab as far as the widget is concerned, so the content is rebuilt.
                // Same slot, and reselected if it was selected, so nothing moves.
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
        true
    }

    /// Re-title every open tab whose item was renamed.
    ///
    /// The tab's caption is a plain `LocalizedString` baked in at open time — it does not
    /// follow a signal — so a rename has to push it. `TabHandle::info` is a public field
    /// and `payload` is an `Rc`, so the handle is rebuilt with a new caption while the
    /// *same* `ContentTab` (and therefore the same live documents, caret and scroll
    /// position) is carried straight through.
    fn retitle(&self, item_id: u64, title: &str) {
        let caption = if title.is_empty() {
            tr!(untitled())
        } else {
            lit!(title.to_string())
        };
        for side in [Side::Primary, Side::Secondary] {
            let pane = self.pane(side);
            for i in 0..pane.tabs.len() {
                let hit = pane.tabs.with_item(i, |h| {
                    h.payload
                        .downcast_ref::<ContentTab>()
                        .is_some_and(|t| t.item_id() == item_id)
                        .then(|| h.clone())
                });
                if let Some(Some(mut h)) = hit {
                    h.info = h.info.clone().title(caption.clone());
                    pane.tabs.set(i, h);
                }
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
mod tests {
    use super::*;
    use crate::tabs;
    use crate::view_models::EditorTypography;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    fn test_typography() -> EditorTypographySet {
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
        }
    }

    fn editors() -> EditorsViewModel {
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let docs = OpenDocsStore::new(app_ctx.clone());
        EditorsViewModel::new(
            app_ctx,
            Signal::new(700.0),
            Signal::new(true),
            test_typography(),
            crate::view_models::EditorViewMemory::detached(false),
            crate::view_models::CorkboardDefaults::detached(),
            ids,
            docs,
            Signal::new(false),
            Signal::new(0),
            Signal::new(false),
        )
    }

    /// Push a tab directly into `side` (bypassing the backend / store) so tab
    /// management can be tested without a loaded project.
    fn push_tab(vm: &EditorsViewModel, side: Side, item_id: u64) -> TabId {
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

    #[test]
    fn open_or_focus_dedupes_within_the_primary_pane() {
        let vm = editors();
        let id = push_tab(&vm, Side::Primary, 42);
        assert_eq!(vm.tabs(Side::Primary).len(), 1);
        vm.open_or_focus(42, "Scene"); // already open → focuses, no backend hit
        assert_eq!(vm.tabs(Side::Primary).len(), 1);
        assert_eq!(vm.selected(Side::Primary).get(), Some(id));
    }

    /// The segment signal of the open tab for `item_id` in `side`, if any.
    #[cfg(feature = "mocks")]
    fn tab_segment(vm: &EditorsViewModel, side: Side, item_id: u64) -> Option<Signal<usize>> {
        let pane = vm.pane(side);
        (0..pane.tabs.len()).find_map(|i| {
            pane.tabs
                .with_item(i, |h| {
                    h.payload
                        .downcast_ref::<ContentTab>()
                        .filter(|t| t.item_id() == item_id)
                        .map(|t| t.segment.clone())
                })
                .flatten()
        })
    }

    /// Editing one item's scalar (a word-count goal, a rename) fires `BinderItem::Updated`
    /// → `items_updated` → `retype`. `retype` must rebuild **only the updated item's own
    /// tab, and only if that item's type actually changed** — never a tab merely because
    /// some *other* open tab has a different type. This regressions the "set a Book's goal
    /// and the Pace segment jumps back to Book" bug: with a differently-typed tab (a Scene)
    /// also open, the goal edit rebuilt the Book's tab (fresh `ContentTab`, segment → 0).
    #[cfg(feature = "mocks")]
    #[test]
    fn updating_an_item_does_not_rebuild_a_differently_typed_open_tab() {
        let vm = editors();
        vm.open_or_focus(101, "Book One"); // Folder/Book (mock fixture)
        vm.open_or_focus(103, "Scene at dawn"); // Item/Scene — a *different* type
        // Put the Book tab on the Pace segment (index 3).
        tab_segment(&vm, Side::Primary, 101)
            .expect("book tab open")
            .set(3);
        // Editing the Book's word-count goal fires `BinderItem::Updated` for the Book.
        vm.items_updated(&[101]);
        // The Book's tab must be the SAME one (its type did not change), so its segment
        // must still be Pace — not reset to 0 by a spurious rebuild.
        assert_eq!(
            tab_segment(&vm, Side::Primary, 101).map(|s| s.get()),
            Some(3),
            "editing the Book's goal rebuilt its tab (segment reset) because another \
             differently-typed tab was open"
        );
    }

    #[test]
    fn set_split_toggles_visibility_and_focus() {
        let vm = editors();
        assert!(!vm.split_active().get());
        vm.set_split(true);
        assert!(vm.split_active().get());
        assert!(vm.splitter().is_pane_visible(1));
        // Collapsing with a side tab open closes it and hides the pane.
        push_tab(&vm, Side::Secondary, 7);
        assert_eq!(vm.tabs(Side::Secondary).len(), 1);
        vm.set_split(false);
        assert!(!vm.split_active().get());
        assert!(!vm.splitter().is_pane_visible(1));
        assert_eq!(vm.tabs(Side::Secondary).len(), 0);
    }

    #[test]
    fn set_focused_switches_active_item_between_panes() {
        let vm = editors();
        vm.set_split(true);
        let p = push_tab(&vm, Side::Primary, 42);
        vm.pane(Side::Primary).selected.set(Some(p));
        let s = push_tab(&vm, Side::Secondary, 7);
        vm.pane(Side::Secondary).selected.set(Some(s));

        // Focusing a pane (e.g. clicking into its editor) makes its selected item
        // the active "inspected" item — the split-view Inspector focus fix. In the
        // app, `focus_within` on each pane drives `set_focused`.
        vm.set_focused(Side::Primary);
        assert_eq!(vm.active_item().get(), Some(42));
        vm.set_focused(Side::Secondary);
        assert_eq!(vm.active_item().get(), Some(7));
        vm.set_focused(Side::Primary);
        assert_eq!(vm.active_item().get(), Some(42));
    }

    #[test]
    fn closing_last_side_tab_auto_collapses() {
        let vm = editors();
        vm.set_split(true);
        let id = push_tab(&vm, Side::Secondary, 9);
        vm.close_in(Side::Secondary, id);
        assert_eq!(vm.tabs(Side::Secondary).len(), 0);
        assert!(
            !vm.split_active().get(),
            "emptying the side pane collapses the split"
        );
    }

    #[test]
    fn transfer_out_of_last_side_tab_collapses_the_split() {
        let vm = editors();
        vm.set_split(true);
        let id = push_tab(&vm, Side::Secondary, 3);
        // Simulate the framework's on_transfer_out (the tab dragged to the other
        // pane): it must empty the side pane AND collapse the split, like a close.
        vm.transfer_out(Side::Secondary, id);
        assert_eq!(vm.tabs(Side::Secondary).len(), 0);
        assert!(
            !vm.split_active().get(),
            "dragging out the last side tab collapses the split"
        );
    }

    #[test]
    fn receive_tab_dedupes_against_the_target_pane() {
        let vm = editors();
        // The same item is open in both panes (two tabs, one item).
        push_tab(&vm, Side::Primary, 5);
        let existing = push_tab(&vm, Side::Secondary, 5);
        // Migrating the primary's tab into the side pane (which already has it)
        // must not create a duplicate; it focuses the existing side tab.
        let migrating = TabHandle::dynamic(
            TabId::fresh(),
            "editor",
            TabInfo::new().closable(true),
            tabs::tab_for(
                &vm.app_ctx,
                5,
                &BinderItemRole::Item,
                &BinderItemSubRole::Scene,
                &[],
                vm.column_width.clone(),
                vm.show_synopsis.clone(),
                vm.typography.clone(),
                vm.view_memory.clone(),
                &vm.ids,
            ),
        );
        vm.receive_tab(Side::Secondary, migrating);
        assert_eq!(
            vm.tabs(Side::Secondary).len(),
            1,
            "no duplicate in the side pane"
        );
        assert_eq!(vm.selected(Side::Secondary).get(), Some(existing));
    }

    #[test]
    fn snapshot_helpers_report_order_selection_and_focus() {
        let vm = editors();
        // Primary: three tabs in order; select the middle one.
        let a = push_tab(&vm, Side::Primary, 10);
        let _b = push_tab(&vm, Side::Primary, 20);
        let _c = push_tab(&vm, Side::Primary, 30);
        vm.pane(Side::Primary).selected.set(Some(a));
        assert_eq!(
            vm.tab_item_ids(Side::Primary),
            vec![10, 20, 30],
            "tab order"
        );
        assert_eq!(
            vm.selected_item(Side::Primary),
            Some(10),
            "selected tab's item"
        );

        // Secondary pane + focus tracking.
        vm.set_split(true);
        let s = push_tab(&vm, Side::Secondary, 99);
        vm.pane(Side::Secondary).selected.set(Some(s));
        assert_eq!(vm.tab_item_ids(Side::Secondary), vec![99]);
        assert_eq!(vm.selected_item(Side::Secondary), Some(99));

        vm.set_focused(Side::Secondary);
        assert_eq!(vm.focused_side(), Side::Secondary);
        vm.set_focused(Side::Primary);
        assert_eq!(vm.focused_side(), Side::Primary);
    }

    #[test]
    fn select_item_reselects_the_tab_for_an_item() {
        // The restore step: after re-opening a pane's tabs, re-mark the one that was
        // selected — by item id, since the `TabId`s are freshly minted on restore.
        let vm = editors();
        let first = push_tab(&vm, Side::Primary, 10);
        let _second = push_tab(&vm, Side::Primary, 20);
        // Currently the first is selected; ask to select item 20's tab.
        vm.pane(Side::Primary).selected.set(Some(first));
        vm.select_item(Side::Primary, 20);
        assert_eq!(vm.selected_item(Side::Primary), Some(20));
        // An item with no open tab is a no-op (selection unchanged).
        vm.select_item(Side::Primary, 12345);
        assert_eq!(vm.selected_item(Side::Primary), Some(20));
    }

    #[test]
    fn close_all_empties_both_panes_and_resets_split() {
        let vm = editors();
        push_tab(&vm, Side::Primary, 1);
        vm.set_split(true);
        push_tab(&vm, Side::Secondary, 2);
        vm.close_all();
        assert_eq!(vm.tabs(Side::Primary).len(), 0);
        assert_eq!(vm.tabs(Side::Secondary).len(), 0);
        assert!(!vm.split_active().get());
        assert_eq!(vm.active_item().get(), None);
    }
}
