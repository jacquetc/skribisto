// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `WorkRegistry` — the app-global (Tier 1) home for the one piece of id-only
//! state that is genuinely process-wide (`root_id`), plus a real per-`work_id`
//! lookup table of every currently-open [`WorkSession`].
//!
//! Phase 2 lands the second simultaneously-open Work: `ProjectWindowFactory`
//! builds each new project window a fresh `WorkSession` (own `AppIds`, own
//! singles, own tag/dictionary palettes — see `sessions::WorkSession`'s module
//! doc), then [`register`](WorkRegistry::register)s it here the moment its
//! `LoadWork`/`NewWork` event resolves a real `work_id` (the id is not knowable
//! before that, so registration can't happen any earlier) — and
//! [`register_window`](WorkRegistry::register_window)s that same window's
//! `TeksiloWindowId` against it, with the teardown to run once it stops
//! showing that Work — either because teksilo confirms the window itself is
//! gone ([`remove_window`](WorkRegistry::remove_window), driven by its
//! `on_removed` window-teardown hook — see `crate::shell::windows`'s wiring),
//! or because the *same* window switched to a different Work in place
//! (`register_window` superseding its own previous binding — File > New Work
//! / Open Work, ProjectSwitcher "Open here", never destroy the window, so
//! `on_removed` never fires for the Work being left). [`unregister`](WorkRegistry::unregister) removes
//! a session, but is no longer called directly from `App`: both of the paths
//! above are, and both are what decide whether a session's teardown (its
//! undo stack) actually runs — the framework's `on_removed` for a real close,
//! `register_window`'s own replace path for an in-place switch.
//! [`session_for`](WorkRegistry::session_for) is the create-**or**-share half of the design doc's
//! "create-or-share, refcounted" resolution mechanism, and Work ▸ New Window
//! (`PendingAction::AttachExisting{work_id}`) is what finally uses it: a second
//! window on an *already*-open Work resolves the live session through
//! [`attach`](WorkRegistry::attach) instead of minting a second one, so both
//! windows share one store view, one undo stack and one set of open documents,
//! and only the *last* of them to close tears any of it down.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use teksilo::prelude::{Signal, TeksiloWindowId};

use super::WorkSession;

/// One registered session plus how many windows are currently attached to it.
/// Refcounted so Phase 3's `AttachExisting` (a second window on an already-open
/// Work) can share the same `WorkSession` instead of minting a second one, and
/// so the *last* window on a Work is the one whose close actually tears it down.
struct Entry {
    session: WorkSession,
    refcount: usize,
    /// The ordinal [`register_window`](WorkRegistry::register_window) will hand
    /// out to the *next* window that attaches to this Work — see
    /// [`WorkRegistry::next_window_ordinal`]'s doc for why this counts up
    /// monotonically rather than tracking "how many windows are open right now".
    next_ordinal: usize,
}

/// The Work-session-scoped half of teardown: delete `work_id`'s undo/redo
/// stack, but only if the `bool` says this really was the *last* window
/// showing it (per [`WorkRegistry`]'s own refcount, never
/// `WindowRemovedEvent::remaining_windows` — see `crate::shell::windows`'s
/// wiring for why). Unlike [`WindowTeardown`], this is safe to run in **two**
/// situations: when the window itself is finally destroyed
/// ([`remove_window`](WorkRegistry::remove_window)), and when a still-live window
/// rebinds *away* from this Work in place — an in-place File > New Work /
/// Open Work / ProjectSwitcher "Open here" never destroys the window, so
/// `on_removed` never fires for it, yet the Work being switched away from is
/// exactly as gone as if its window had closed and its session/undo stack
/// must be torn down the same way (see [`register_window`](WorkRegistry::register_window)'s doc).
pub type StackTeardown = Rc<dyn Fn(bool)>;

/// The window-instance-scoped half of teardown: release exactly this window's
/// own `OpenDoc` refs and retire its backup flush-hook registration. Built
/// once in `App::build`, at `LoadWork`/`NewWork` time, over handles that
/// outlive the window's own widget tree (a per-window `EditorsViewModel`
/// clone, an `OpenDocsStore`/`AppContext` handle) — nothing inside it may
/// touch the dead window's tree, per `on_removed`'s own contract.
///
/// Unlike [`StackTeardown`], this must run **only** when the window itself is
/// really, finally gone ([`remove_window`](WorkRegistry::remove_window)) — never on an
/// in-place Work switch: the window's `EditorsViewModel` and its backup flush
/// hook both survive the switch (the same window keeps showing tabs and
/// keeps needing its buffers flushed for whatever Work it shows next), so
/// releasing them mid-switch would silently break flushing/doc-tracking for
/// the *new* Work the window goes on to show.
pub type WindowTeardown = Rc<dyn Fn()>;

/// One window's binding to the Work it is showing, plus what to run when it
/// stops showing that particular Work ([`StackTeardown`]) and what to run only
/// once the window itself is confirmed gone ([`WindowTeardown`]).
struct WindowEntry {
    work_id: u64,
    /// This window's fixed "which window on `work_id` am I" number (Scope D —
    /// window titles). Assigned once, the first time this `window_id` binds to
    /// this particular `work_id`, and never reassigned/renumbered afterward —
    /// see [`WorkRegistry::register_window`]'s doc.
    ordinal: usize,
    stack_teardown: StackTeardown,
    window_teardown: WindowTeardown,
}

/// App-global registry: `root_id` (Tier 1 — one `Root` per process, never
/// per-Work) plus a `work_id`-keyed table of every currently-open [`WorkSession`],
/// plus the window→Work axis this struct's earlier revision did not have (see
/// [`register_window`](Self::register_window)/[`remove_window`](Self::remove_window)).
#[derive(Clone)]
pub struct WorkRegistry {
    root_id: Signal<Option<u64>>,
    sessions: Rc<RefCell<HashMap<u64, Entry>>>,
    /// Which Work each currently-open window is showing, and that window's own
    /// teardown. The genuinely new bookkeeping teksilo's `on_removed` hook makes
    /// possible: previously nothing in this crate could answer "which window is
    /// this `TeksiloWindowId`, and what does it hold onto" at all.
    windows: Rc<RefCell<HashMap<TeksiloWindowId, WindowEntry>>>,
}

impl Default for WorkRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkRegistry {
    pub fn new() -> Self {
        Self {
            root_id: Signal::new(None),
            sessions: Rc::new(RefCell::new(HashMap::new())),
            windows: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// The one shared `Root`'s id — set once at startup from
    /// `initialize_app`'s result, read by nothing today (see `app_ids.rs`'s
    /// module doc for why it moved here rather than staying on `AppIds`; it
    /// was equally write-only there before this migration, so this is not a
    /// new gap).
    #[allow(dead_code)] // the query half of a write-only-today value; see the doc above
    pub fn root_id(&self) -> Signal<Option<u64>> {
        self.root_id.clone()
    }

    pub fn set_root_id(&self, id: Option<u64>) {
        self.root_id.set(id);
    }

    /// Register a freshly-loaded/created Work's session, refcounted at 1 for
    /// its first window. Calling this again for a `work_id` that is already
    /// registered is a no-op on the *session* (the existing one wins — it is
    /// still the live one) but still bumps the refcount, matching
    /// [`attach`](Self::attach)'s contract: every caller that "opens onto" a
    /// `work_id`, whether by registering it for the first time or attaching a
    /// further window to an already-registered one, must pair with exactly one
    /// [`unregister`](Self::unregister) call.
    pub fn register(&self, work_id: u64, session: WorkSession) {
        let mut sessions = self.sessions.borrow_mut();
        sessions
            .entry(work_id)
            .and_modify(|e| e.refcount += 1)
            .or_insert(Entry {
                session,
                refcount: 1,
                next_ordinal: 1,
            });
    }

    /// A further window is attaching to an *already-registered* `work_id`'s
    /// session (Work ▸ New Window, `PendingAction::AttachExisting`). Returns
    /// the shared session, or `None` if `work_id` names no currently-open
    /// Work. Bumps the refcount on success — the caller must pair this with
    /// exactly one [`unregister`](Self::unregister) when its window closes.
    #[allow(dead_code)] // called by `ProjectWindowFactory::attached_window_config`
    pub fn attach(&self, work_id: u64) -> Option<WorkSession> {
        let mut sessions = self.sessions.borrow_mut();
        let entry = sessions.get_mut(&work_id)?;
        entry.refcount += 1;
        Some(entry.session.clone())
    }

    /// One window on `work_id` closed. Decrements the refcount; when it
    /// reaches zero, the entry is removed and `true` is returned — the signal
    /// that this was genuinely the *last* window on this Work, so it is safe
    /// to run the session's real teardown (deleting its undo stack). Called
    /// from [`remove_window`](Self::remove_window), driven by teksilo's
    /// `on_removed` window-teardown hook — not directly by `App` any more (see
    /// this module's doc). A `work_id` that isn't registered is a safe no-op
    /// returning `false` — nothing to tear down.
    pub fn unregister(&self, work_id: u64) -> bool {
        let mut sessions = self.sessions.borrow_mut();
        let Some(entry) = sessions.get_mut(&work_id) else {
            return false;
        };
        entry.refcount -= 1;
        if entry.refcount == 0 {
            sessions.remove(&work_id);
            true
        } else {
            false
        }
    }

    /// The session for `work_id`, if that Work is currently open in *any*
    /// window. Used by `shell::window_ids::find_open_project_window` (the
    /// registry fallback for "is this project already open?") and by
    /// `QuitSequencer` to reach each open Work's session in turn.
    #[allow(dead_code)] // also called outside this module's own tests
    pub fn session_for(&self, work_id: u64) -> Option<WorkSession> {
        self.sessions
            .borrow()
            .get(&work_id)
            .map(|e| e.session.clone())
    }

    /// Every currently-open Work's id. Used by `app.rs`'s "does another Work
    /// still have a window open" survivor check (whether closing this Work
    /// returns to the Launcher), `QuitSequencer`'s per-Work walk, and
    /// `shell::window_ids::find_open_project_window`.
    pub fn open_work_ids(&self) -> Vec<u64> {
        self.sessions.borrow().keys().copied().collect()
    }

    /// How many windows are currently attached to `work_id` (its live
    /// refcount) — `0` for an unregistered/unknown id. Window *titles*
    /// disambiguate via each window's own
    /// [`register_window`](Self::register_window)-assigned `ordinal` instead
    /// (stable per-window, unlike this live count — see
    /// `shell::windows::window_title_text`'s doc); this count is what
    /// `WindowRole::may_switch_in_place` and the project window's close guard
    /// use to tell "closing a view" (a sibling window still shows this Work)
    /// from "closing the project" (this is the only window on it).
    #[allow(dead_code)] // also called outside this module's own tests
    pub fn window_count_for(&self, work_id: u64) -> usize {
        self.sessions
            .borrow()
            .get(&work_id)
            .map(|e| e.refcount)
            .unwrap_or(0)
    }

    /// Hand out the next unused ordinal for a window newly attaching to
    /// `work_id` — a monotonically increasing, never-reused "which window on
    /// this Work am I" number (window 1 stays "1" for its whole lifetime even
    /// after window 2 closes; a later window 3 gets "3", not "2" again). This
    /// is deliberate: the design doc (§8) wants a *stable* per-window title —
    /// a KWin rule matches a window by title text, so a title that could
    /// change identity on a sibling's close (renumbering "2" of {1,2} down to
    /// "1" once window 1 closes, say) would silently break that binding.
    /// `1` for a `work_id` this registry has never seen (safe default — the
    /// only real caller always calls this right after `register`/`attach`,
    /// which is itself always called before `register_window`, so this path
    /// is a defensive fallback, not the normal one).
    ///
    /// **Reserved ahead of the window for Work ▸ New Window** ([`Self::reserve_window_ordinal`]):
    /// a second window's *persistence string id* is derived from its ordinal
    /// (`shell::windows::attached_window_id_for`), and that id has to be decided
    /// before `open_window` is called — long before the `TeksiloWindowId`
    /// `register_window` binds even exists. So the caller reserves first and
    /// hands the reserved number back to `register_window`, which then uses it
    /// verbatim instead of consuming a second one.
    fn next_window_ordinal(&self, work_id: u64) -> usize {
        // Never hand out a number a live window on this Work already carries.
        // The counter alone would be enough if it were bumped in lock-step with
        // every binding — but `reserve_window_ordinal` can be called before the
        // Work's *first* window has registered (its `register`/`register_window`
        // pair is not atomic), and handing that reservation a `1` would give the
        // new window the same ordinal, and therefore the same
        // `attached_window_id_for` string id, as the window it is meant to sit
        // beside. teksilo's window map overwrites rather than rejects a
        // duplicate id, so that collision would not fail loudly: the second
        // window would silently inherit the first's identity and geometry slot.
        //
        // Taking the max of the counter and "one past the highest live ordinal"
        // makes "never reused, never shared" a property of the data rather than
        // of the call order. It cannot renumber anyone: it only ever moves the
        // *next* number forward.
        let highest_live = self
            .windows
            .borrow()
            .values()
            .filter(|e| e.work_id == work_id)
            .map(|e| e.ordinal)
            .max();
        let mut sessions = self.sessions.borrow_mut();
        match sessions.get_mut(&work_id) {
            Some(entry) => {
                let ordinal = entry.next_ordinal.max(highest_live.map_or(1, |n| n + 1));
                entry.next_ordinal = ordinal + 1;
                ordinal
            }
            None => highest_live.map_or(1, |n| n + 1),
        }
    }

    /// Claim the ordinal a not-yet-created window will carry on `work_id` —
    /// the public half of [`Self::next_window_ordinal`], for Work ▸ New Window.
    /// Call it exactly once per window opened, and pass the result to
    /// [`register_window`](Self::register_window) as `preassigned`: the number
    /// is consumed here, so a reservation that is never registered simply
    /// leaves a gap in the sequence (harmless — ordinals are identifiers, not a
    /// count).
    pub fn reserve_window_ordinal(&self, work_id: u64) -> usize {
        self.next_window_ordinal(work_id)
    }

    /// Every window currently bound to `work_id` — the query "Close Work" needs
    /// once a project can have more than one window.
    ///
    /// Closing a *Work* has to close every window showing it: the backend
    /// subtree is gone, so a sibling left open would be a window onto nothing —
    /// its tabs closed by the shared `CloseWork` subscriber, its tree empty, its
    /// title still naming a project that no longer exists. Closing *a window*
    /// (the title-bar X) is the other, narrower gesture and does not come
    /// through here.
    ///
    /// Ordered by [`WindowEntry::ordinal`], so the first entry is the
    /// longest-standing window on the Work. That matters for the *other* caller:
    /// `shell::windows::open_or_focus_project` raises `windows_for(..).first()`
    /// when asked to show a project, and "whichever the `HashMap` happened to
    /// yield" would raise a different window on different runs.
    pub fn windows_for(&self, work_id: u64) -> Vec<TeksiloWindowId> {
        let windows = self.windows.borrow();
        let mut found: Vec<(usize, TeksiloWindowId)> = windows
            .iter()
            .filter(|(_, e)| e.work_id == work_id)
            .map(|(id, e)| (e.ordinal, *id))
            .collect();
        found.sort_by_key(|(ordinal, _)| *ordinal);
        found.into_iter().map(|(_, id)| id).collect()
    }

    // ── Window → Work bookkeeping ────────────────────────────────────────────
    //
    // teksilo's `WindowConfig::on_removed` hook (see `crate::shell::windows`'s
    // wiring) is the framework's guarantee that a window is really, finally
    // gone — tree dropped, platform window destroyed, every framework registry
    // entry purged. `register_window` binds a window to the Work it is showing
    // (as soon as its own `LoadWork`/`NewWork` resolves a real `work_id`);
    // `remove_window`, driven by that hook, releases the binding and the
    // window's own teardown the instant the hook fires. But a window can also
    // stop showing a Work *without* being destroyed — an in-place File > New
    // Work / Open Work never fires `on_removed` for the Work being left — so
    // `register_window`'s own replace path is the other place a Work's
    // session/undo-stack teardown can run (see its doc).

    /// Bind `window_id` to the `work_id` it is currently showing, with the
    /// [`StackTeardown`]/[`WindowTeardown`] to run once, respectively, this
    /// binding is superseded or teksilo confirms this window is gone. Called
    /// from `App::build`, right alongside [`register`](Self::register)/
    /// [`attach`](Self::attach) — the same moment this window's own `work_id`
    /// becomes known.
    ///
    /// Re-binding an already-bound `window_id` (an in-place File > New Work /
    /// Open Work / ProjectSwitcher "Open here" switching *this* window to a
    /// different Work — none of which destroy the window, so `on_removed`
    /// never fires for the Work being left) is where this earns its keep: the
    /// *previous* binding's [`unregister`](Self::unregister) runs right here,
    /// and its `StackTeardown` runs with whatever that reports — so the Work
    /// being switched away from gets its session/undo-stack torn down exactly
    /// as if its own last window had closed, even though this window survives.
    /// The previous binding's `WindowTeardown` is dropped **unrun**: this
    /// window's own `OpenDoc`s/flush hook belong to the window instance, not
    /// to whichever Work it happens to be showing, and stay alive for the Work
    /// it shows next (see [`WindowTeardown`]'s doc). A same-`work_id`
    /// re-registration (a reload of the Work this window already shows) is
    /// handled by the same path and stays correct: the redundant
    /// [`register`](Self::register) call this always follows bumped the
    /// refcount an extra time, and the `unregister` here exactly cancels that
    /// extra bump, so `is_last` can only ever be `true` when no window (this
    /// one included) still shows that Work.
    /// Returns this window's [`WindowEntry::ordinal`] for `work_id` — the
    /// stable "which window on this Work am I" number Scope D's window titles
    /// read (see [`next_window_ordinal`](Self::next_window_ordinal)'s doc). A
    /// window re-registering the SAME `work_id` it already showed (e.g. a
    /// reload) keeps its existing ordinal rather than being handed a fresh
    /// one; a window binding to a work_id for the first time — whether it has
    /// never registered before, or it is switching in place from a different
    /// Work — gets the next free one for its NEW `work_id`.
    ///
    /// `preassigned` is the ordinal a Work ▸ New Window caller already reserved
    /// with [`reserve_window_ordinal`](Self::reserve_window_ordinal) before the
    /// window existed (its persistence id is derived from it). It is used
    /// verbatim — no second number is consumed — and, like every other first
    /// binding, is remembered for as long as this window shows this Work.
    /// `None` is every other caller: assign/reuse as described above.
    pub fn register_window(
        &self,
        window_id: TeksiloWindowId,
        work_id: u64,
        preassigned: Option<usize>,
        stack_teardown: StackTeardown,
        window_teardown: WindowTeardown,
    ) -> usize {
        let reused_ordinal = self
            .windows
            .borrow()
            .get(&window_id)
            .filter(|e| e.work_id == work_id)
            .map(|e| e.ordinal);
        let ordinal = reused_ordinal
            .or(preassigned)
            .unwrap_or_else(|| self.next_window_ordinal(work_id));

        let previous = self.windows.borrow_mut().insert(
            window_id,
            WindowEntry {
                work_id,
                ordinal,
                stack_teardown,
                window_teardown,
            },
        );
        if let Some(previous) = previous {
            let is_last = self.unregister(previous.work_id);
            (previous.stack_teardown)(is_last);
        }
        ordinal
    }

    /// teksilo's `on_removed` hook fired for `window_id`: this window is really
    /// gone. Removes its own binding, runs the window's own `WindowTeardown`
    /// unconditionally (its `OpenDoc`s/flush hook belong to the window
    /// instance, and the window instance really is gone now), then decrements
    /// its Work's session refcount ([`unregister`](Self::unregister)) and runs
    /// the `StackTeardown` with whether this was the *last* window on that
    /// Work — so it tears down the Work-scoped resources (its undo stack) only
    /// when nothing else still shows it.
    ///
    /// A safe no-op for a window that never registered one — the Launcher (which
    /// shows no Work at all), or a project window force-closed before its own
    /// Load/New ever resolved a `work_id`.
    pub fn remove_window(&self, window_id: TeksiloWindowId) {
        let Some(entry) = self.windows.borrow_mut().remove(&window_id) else {
            return;
        };
        (entry.window_teardown)();
        let is_last = self.unregister(entry.work_id);
        (entry.stack_teardown)(is_last);
    }
}

#[cfg(test)]
mod tests;
