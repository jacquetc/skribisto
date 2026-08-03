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
//! `BastydeWindowId` against it, with the teardown to run once it stops
//! showing that Work — either because bastyde confirms the window itself is
//! gone ([`remove_window`](WorkRegistry::remove_window), driven by its
//! `on_removed` window-teardown hook — see `crate::shell::windows`'s wiring),
//! or because the *same* window switched to a different Work in place
//! (`register_window` superseding its own previous binding — File > New Work
//! / Open Work, ProjectSwitcher "Open here", never destroy the window, so
//! `on_removed` never fires for the Work being left). [`unregister`] removes
//! a session, but is no longer called directly from `App`: both of the paths
//! above are, and both are what decide whether a session's teardown (its
//! undo stack) actually runs — the framework's `on_removed` for a real close,
//! `register_window`'s own replace path for an in-place switch.
//! [`session_for`] is the create-**or**-share half of the design doc's
//! "create-or-share, refcounted" resolution mechanism, and Work ▸ New Window
//! (`PendingAction::AttachExisting{work_id}`) is what finally uses it: a second
//! window on an *already*-open Work resolves the live session through
//! [`attach`](WorkRegistry::attach) instead of minting a second one, so both
//! windows share one store view, one undo stack and one set of open documents,
//! and only the *last* of them to close tears any of it down.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use bastyde::prelude::{BastydeWindowId, Signal};

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
/// ([`remove_window`](Self::remove_window)), and when a still-live window
/// rebinds *away* from this Work in place — an in-place File > New Work /
/// Open Work / ProjectSwitcher "Open here" never destroys the window, so
/// `on_removed` never fires for it, yet the Work being switched away from is
/// exactly as gone as if its window had closed and its session/undo stack
/// must be torn down the same way (see [`register_window`](Self::register_window)'s doc).
pub type StackTeardown = Rc<dyn Fn(bool)>;

/// The window-instance-scoped half of teardown: release exactly this window's
/// own `OpenDoc` refs and retire its backup flush-hook registration. Built
/// once in `App::build`, at `LoadWork`/`NewWork` time, over handles that
/// outlive the window's own widget tree (a per-window `EditorsViewModel`
/// clone, an `OpenDocsStore`/`AppContext` handle) — nothing inside it may
/// touch the dead window's tree, per `on_removed`'s own contract.
///
/// Unlike [`StackTeardown`], this must run **only** when the window itself is
/// really, finally gone ([`remove_window`](Self::remove_window)) — never on an
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
    /// teardown. The genuinely new bookkeeping bastyde's `on_removed` hook makes
    /// possible: previously nothing in this crate could answer "which window is
    /// this `BastydeWindowId`, and what does it hold onto" at all.
    windows: Rc<RefCell<HashMap<BastydeWindowId, WindowEntry>>>,
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
    /// from [`remove_window`](Self::remove_window), driven by bastyde's
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
    /// before `open_window` is called — long before the `BastydeWindowId`
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
        // beside. bastyde's window map overwrites rather than rejects a
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
    pub fn windows_for(&self, work_id: u64) -> Vec<BastydeWindowId> {
        let windows = self.windows.borrow();
        let mut found: Vec<(usize, BastydeWindowId)> = windows
            .iter()
            .filter(|(_, e)| e.work_id == work_id)
            .map(|(id, e)| (e.ordinal, *id))
            .collect();
        found.sort_by_key(|(ordinal, _)| *ordinal);
        found.into_iter().map(|(_, id)| id).collect()
    }

    // ── Window → Work bookkeeping ────────────────────────────────────────────
    //
    // bastyde's `WindowConfig::on_removed` hook (see `crate::shell::windows`'s
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
    /// binding is superseded or bastyde confirms this window is gone. Called
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
        window_id: BastydeWindowId,
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

    /// bastyde's `on_removed` hook fired for `window_id`: this window is really
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
    pub fn remove_window(&self, window_id: BastydeWindowId) {
        let Some(entry) = self.windows.borrow_mut().remove(&window_id) else {
            return;
        };
        (entry.window_teardown)();
        let is_last = self.unregister(entry.work_id);
        (entry.stack_teardown)(is_last);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_session() -> WorkSession {
        WorkSession::for_test()
    }

    #[test]
    fn a_fresh_registry_has_no_root_and_no_sessions() {
        let reg = WorkRegistry::new();
        assert_eq!(reg.root_id().get(), None);
        assert!(reg.open_work_ids().is_empty());
    }

    #[test]
    fn root_id_is_a_plain_shared_signal() {
        let reg = WorkRegistry::new();
        reg.set_root_id(Some(7));
        assert_eq!(reg.root_id().get(), Some(7));
        // Every clone shares the same signal (it's the whole point of `Signal`).
        assert_eq!(reg.clone().root_id().get(), Some(7));
    }

    #[test]
    fn session_for_answers_only_the_matching_work_id() {
        let reg = WorkRegistry::new();
        reg.register(42, fixture_session());

        assert!(
            reg.session_for(42).is_some(),
            "a registered Work's id must resolve"
        );
        assert!(
            reg.session_for(99).is_none(),
            "an unregistered id must not resolve to someone else's session"
        );
    }

    #[test]
    fn no_session_means_nothing_resolves() {
        let reg = WorkRegistry::new();
        assert!(reg.session_for(1).is_none());
    }

    #[test]
    fn two_works_coexist_independently() {
        let reg = WorkRegistry::new();
        let a = fixture_session();
        a.ids.work_id.set(Some(1));
        let b = fixture_session();
        b.ids.work_id.set(Some(2));
        reg.register(1, a);
        reg.register(2, b);

        assert_eq!(reg.session_for(1).unwrap().ids.work_id.get(), Some(1));
        assert_eq!(reg.session_for(2).unwrap().ids.work_id.get(), Some(2));
        assert_eq!(reg.open_work_ids().len(), 2);

        // Closing one leaves the other fully intact — the whole point of the migration.
        assert!(reg.unregister(1));
        assert!(reg.session_for(1).is_none());
        assert!(
            reg.session_for(2).is_some(),
            "Work 2 must survive Work 1 closing"
        );
    }

    #[test]
    fn attach_shares_the_same_instance_and_refcounts_it() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());

        let shared = reg.attach(1).expect("already-open Work must attach");
        // Two windows now hold Work 1 — one `unregister` must not tear it down.
        assert!(
            !reg.unregister(1),
            "one of two windows closing must not finalize teardown"
        );
        assert!(
            reg.session_for(1).is_some(),
            "the other window's session must still be live"
        );
        assert!(
            reg.unregister(1),
            "the last window closing must finalize teardown"
        );
        assert!(reg.session_for(1).is_none());
        drop(shared);
    }

    #[test]
    fn attach_to_an_unknown_work_id_resolves_nothing() {
        let reg = WorkRegistry::new();
        assert!(reg.attach(404).is_none());
    }

    #[test]
    fn unregister_an_unknown_work_id_is_a_safe_no_op() {
        let reg = WorkRegistry::new();
        assert!(!reg.unregister(404));
    }

    // ── `register_window` / `remove_window` (the `on_removed`-driven axis) ──

    /// A `StackTeardown` that records every `is_last` it was called with, plus
    /// a handle to read them back.
    fn tracked_stack_teardown() -> (StackTeardown, Rc<RefCell<Vec<bool>>>) {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let seen = calls.clone();
        (
            Rc::new(move |is_last| seen.borrow_mut().push(is_last)),
            calls,
        )
    }

    /// A `WindowTeardown` that counts how many times it ran, plus a handle to
    /// read the count back.
    fn tracked_window_teardown() -> (WindowTeardown, Rc<RefCell<u32>>) {
        let calls = Rc::new(RefCell::new(0));
        let seen = calls.clone();
        (Rc::new(move || *seen.borrow_mut() += 1), calls)
    }

    fn inert_window_teardown() -> WindowTeardown {
        Rc::new(|| {})
    }

    #[test]
    fn removing_a_works_only_window_runs_teardown_as_last_and_drops_the_session() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        let (stack_teardown, stack_calls) = tracked_stack_teardown();
        let (window_teardown, window_calls) = tracked_window_teardown();
        reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            stack_teardown,
            window_teardown,
        );

        reg.remove_window(BastydeWindowId::new(1));

        assert_eq!(
            *stack_calls.borrow(),
            vec![true],
            "the only window on Work 1 is the last one"
        );
        assert_eq!(
            *window_calls.borrow(),
            1,
            "a real close must run the window's own teardown"
        );
        assert!(
            reg.session_for(1).is_none(),
            "the session must be torn down with it"
        );
    }

    #[test]
    fn removing_one_of_two_windows_on_a_work_is_not_last_and_keeps_the_session() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        reg.attach(1)
            .expect("a second window attaches to the same Work");
        let (first_stack, first_stack_calls) = tracked_stack_teardown();
        let (second_stack, second_stack_calls) = tracked_stack_teardown();
        reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            first_stack,
            inert_window_teardown(),
        );
        reg.register_window(
            BastydeWindowId::new(2),
            1,
            None,
            second_stack,
            inert_window_teardown(),
        );

        reg.remove_window(BastydeWindowId::new(1));
        assert_eq!(
            *first_stack_calls.borrow(),
            vec![false],
            "a sibling window still holds Work 1"
        );
        assert!(
            reg.session_for(1).is_some(),
            "the session must survive while a sibling window is open"
        );

        reg.remove_window(BastydeWindowId::new(2));
        assert_eq!(
            *second_stack_calls.borrow(),
            vec![true],
            "this was the last window standing"
        );
        assert!(
            reg.session_for(1).is_none(),
            "the session must be torn down once its last window closes"
        );
    }

    #[test]
    fn removing_a_window_never_touches_a_sibling_works_session() {
        let reg = WorkRegistry::new();
        let a = fixture_session();
        a.ids.work_id.set(Some(1));
        let b = fixture_session();
        b.ids.work_id.set(Some(2));
        reg.register(1, a);
        reg.register(2, b);
        reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        reg.register_window(
            BastydeWindowId::new(2),
            2,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        reg.remove_window(BastydeWindowId::new(1));

        assert!(reg.session_for(1).is_none(), "Work 1's window closed");
        assert!(
            reg.session_for(2).is_some(),
            "Work 2 must be untouched by Work 1's window closing"
        );
    }

    #[test]
    fn removing_an_unbound_window_is_a_safe_no_op() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        // No `register_window` call for this id — nothing was ever bound.
        reg.remove_window(BastydeWindowId::new(99));
        assert!(
            reg.session_for(1).is_some(),
            "an unrelated window's removal must not touch Work 1"
        );
    }

    #[test]
    fn removing_a_window_twice_is_a_safe_no_op_the_second_time() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        let (window_teardown, window_calls) = tracked_window_teardown();
        reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            window_teardown,
        );

        reg.remove_window(BastydeWindowId::new(1));
        reg.remove_window(BastydeWindowId::new(1));

        assert_eq!(
            *window_calls.borrow(),
            1,
            "the second removal of the same window must not re-run its teardown"
        );
    }

    // ── in-place Work switch (`register_window` superseding its own binding) ──
    //
    // File > New Work / Open Work, and the ProjectSwitcher's "Open here", all
    // reload the SAME window onto a different Work without ever destroying
    // it — so bastyde's `on_removed` never fires for the Work being left. The
    // regression these tests guard: `register_window` used to silently drop
    // the previous `WindowEntry` (and its teardown) on replace, permanently
    // leaking the old Work's `WorkRegistry` entry and undo stack.

    #[test]
    fn switching_a_windows_only_work_in_place_tears_down_the_old_session_and_stack() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        let (old_stack, old_stack_calls) = tracked_stack_teardown();
        let (old_window, old_window_calls) = tracked_window_teardown();
        reg.register_window(BastydeWindowId::new(1), 1, None, old_stack, old_window);

        // The same window now loads Work 2 in place (no `remove_window` call —
        // the window was never destroyed).
        reg.register(2, fixture_session());
        let (new_stack, _new_stack_calls) = tracked_stack_teardown();
        reg.register_window(
            BastydeWindowId::new(1),
            2,
            None,
            new_stack,
            inert_window_teardown(),
        );

        assert_eq!(
            *old_stack_calls.borrow(),
            vec![true],
            "Work 1 had no other window on it, so switching away in place must be reported as last"
        );
        assert!(
            reg.session_for(1).is_none(),
            "Work 1's session must be torn down once its only window switched away in place"
        );
        assert!(
            reg.session_for(2).is_some(),
            "Work 2 must now be the window's live session"
        );
        assert_eq!(
            *old_window_calls.borrow(),
            0,
            "an in-place switch must NOT run the window's own teardown — its EditorsViewModel \
             and backup flush hook survive to serve whatever Work it shows next"
        );
    }

    #[test]
    fn switching_in_place_away_from_a_work_with_a_sibling_window_is_not_last() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        reg.attach(1)
            .expect("a second window attaches to the same Work");
        let (window_a_stack, window_a_calls) = tracked_stack_teardown();
        reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            window_a_stack,
            inert_window_teardown(),
        );
        reg.register_window(
            BastydeWindowId::new(2),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        // Window 1 switches to Work 2 in place; window 2 still shows Work 1.
        reg.register(2, fixture_session());
        reg.register_window(
            BastydeWindowId::new(1),
            2,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        assert_eq!(
            *window_a_calls.borrow(),
            vec![false],
            "window 2 still shows Work 1 — switching window 1 away must not be reported as last"
        );
        assert!(
            reg.session_for(1).is_some(),
            "Work 1 must survive: window 2 still shows it"
        );
    }

    #[test]
    fn reloading_the_same_work_in_place_does_not_prematurely_tear_it_down() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        let (first_stack, first_calls) = tracked_stack_teardown();
        reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            first_stack,
            inert_window_teardown(),
        );

        // The window reloads the very same Work (e.g. a Revert-style re-load):
        // `register` is called again for `work_id` 1, bumping its refcount a
        // second time before `register_window` supersedes its own binding.
        reg.register(1, fixture_session());
        let (second_stack, _second_calls) = tracked_stack_teardown();
        reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            second_stack,
            inert_window_teardown(),
        );

        assert_eq!(
            *first_calls.borrow(),
            vec![false],
            "the redundant refcount bump must be cancelled without ever reporting a real \
             last-window teardown for a Work this window still shows"
        );
        assert!(
            reg.session_for(1).is_some(),
            "Work 1 must still be open after reloading in place"
        );

        // The window finally closes for real — now it really is last.
        reg.remove_window(BastydeWindowId::new(1));
        assert!(reg.session_for(1).is_none());
    }

    // ── Window ordinal (Scope D — window titles) ─────────────────────────

    #[test]
    fn window_count_for_reports_zero_for_an_unregistered_work() {
        let reg = WorkRegistry::new();
        assert_eq!(reg.window_count_for(404), 0);
    }

    #[test]
    fn the_first_window_on_a_work_gets_ordinal_one() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        let ordinal = reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        assert_eq!(ordinal, 1);
        assert_eq!(reg.window_count_for(1), 1);
    }

    #[test]
    fn a_second_window_attaching_to_the_same_work_gets_the_next_ordinal() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        reg.attach(1)
            .expect("a second window attaches to the same Work");
        let first = reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        let second = reg.register_window(
            BastydeWindowId::new(2),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        assert_eq!(first, 1);
        assert_eq!(
            second, 2,
            "the second window must never reuse window 1's ordinal"
        );
        assert_eq!(reg.window_count_for(1), 2);
    }

    #[test]
    fn ordinals_are_never_reused_once_a_lower_numbered_sibling_closes() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        reg.attach(1).expect("a second window attaches");
        reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        reg.register_window(
            BastydeWindowId::new(2),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        // Window 1 (ordinal 1) closes; window 2 (ordinal 2) stays open.
        reg.remove_window(BastydeWindowId::new(1));
        assert!(reg.session_for(1).is_some(), "window 2 keeps Work 1 open");

        // A third window now attaches to the still-open Work 1.
        reg.attach(1)
            .expect("a third window attaches to the still-open Work");
        let third = reg.register_window(
            BastydeWindowId::new(3),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        assert_eq!(
            third, 3,
            "a fresh window must never be handed a closed sibling's old ordinal — a title \
             a KWin rule matched against must stay stable for the window it named"
        );
    }

    #[test]
    fn reloading_the_same_work_in_place_keeps_its_ordinal() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        let first = reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        reg.register(1, fixture_session());
        let reloaded = reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        assert_eq!(
            first, reloaded,
            "reloading the same Work in place must not renumber this window"
        );
    }

    #[test]
    fn switching_in_place_to_a_different_work_gets_that_works_own_ordinal() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        // A second, unrelated window is already the "second" window on Work 2.
        reg.register(2, fixture_session());
        reg.attach(2).expect("a second window attaches to Work 2");
        reg.register_window(
            BastydeWindowId::new(2),
            2,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        // Window 1 now switches, in place, onto Work 2 — it becomes Work 2's
        // second window, not a reuse of its own former ordinal on Work 1.
        let switched = reg.register_window(
            BastydeWindowId::new(1),
            2,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        assert_eq!(
            switched, 2,
            "window 1 must be numbered against Work 2's own window count, not Work 1's"
        );
    }

    // ── Reserved ordinals + `windows_for` (Work ▸ New Window) ───────────────
    //
    // A second window on an already-open Work has to know its ordinal *before*
    // it exists — its persistence string id is derived from it — so the number
    // is reserved up front and handed back to `register_window`.

    #[test]
    fn a_reserved_ordinal_is_used_verbatim_and_consumes_no_second_number() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        // Work ▸ New Window: reserve, open the window, then bind it.
        let reserved = reg.reserve_window_ordinal(1);
        assert_eq!(reserved, 2, "the reservation must be the next free ordinal");
        reg.attach(1)
            .expect("the new window attaches to the open Work");
        let bound = reg.register_window(
            BastydeWindowId::new(2),
            1,
            Some(reserved),
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        assert_eq!(bound, reserved, "the reserved number must be used verbatim");

        // A third window must follow the reserved one rather than collide with
        // it — proof the reservation consumed the number rather than peeking.
        assert_eq!(reg.reserve_window_ordinal(1), 3);
    }

    #[test]
    fn a_reserved_ordinal_never_overrides_a_window_already_on_that_work() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        let first = reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        // A rebuild that somehow carried a reservation must still keep this
        // window's established number — a title a KWin rule matched against
        // must not change under it (see `next_window_ordinal`'s doc).
        let again = reg.register_window(
            BastydeWindowId::new(1),
            1,
            Some(9),
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        assert_eq!(
            again, first,
            "an established binding outranks a reservation"
        );
    }

    #[test]
    fn reserving_for_an_unknown_work_is_a_safe_one() {
        let reg = WorkRegistry::new();
        assert_eq!(
            reg.reserve_window_ordinal(404),
            1,
            "no window on it yet, so the next one is the first"
        );
    }

    /// A reservation made before the Work's first window has bound itself must
    /// still not collide with it. `register` and `register_window` are not one
    /// atomic step, and an ordinal collision here is silent: two windows would
    /// build the same `attached_window_id_for` string id, and bastyde's window
    /// map overwrites rather than rejects a duplicate.
    #[test]
    fn a_reservation_never_collides_with_a_live_window_whose_counter_lags() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        // Bind window 1 with an explicit ordinal, leaving the entry's own
        // counter untouched at 1 — the lagging-counter case.
        reg.register_window(
            BastydeWindowId::new(1),
            1,
            Some(1),
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        assert_eq!(
            reg.reserve_window_ordinal(1),
            2,
            "the reservation must step over the live window's ordinal, not duplicate it"
        );
    }

    /// "Close Work" has to reach every window showing it — a sibling left open
    /// would be a window onto a project that no longer exists.
    #[test]
    fn windows_for_lists_every_window_on_a_work_and_no_others() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        reg.attach(1).expect("a second window attaches to Work 1");
        reg.register(2, fixture_session());
        reg.register_window(
            BastydeWindowId::new(1),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        reg.register_window(
            BastydeWindowId::new(2),
            1,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        reg.register_window(
            BastydeWindowId::new(3),
            2,
            None,
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        assert_eq!(
            reg.windows_for(1),
            vec![BastydeWindowId::new(1), BastydeWindowId::new(2)],
            "both of Work 1's windows, ordinal order"
        );
        assert_eq!(
            reg.windows_for(2),
            vec![BastydeWindowId::new(3)],
            "Work 2's window must not be swept in"
        );
        assert!(
            reg.windows_for(404).is_empty(),
            "an unknown Work has no windows"
        );
    }

    /// `open_or_focus_project` raises `windows_for(..).first()`, so the order is
    /// part of the contract, not an accident of iteration: the longest-standing
    /// window on a Work must come first, whatever order the windows registered
    /// in or the `HashMap` yields them.
    #[test]
    fn windows_for_is_ordered_by_ordinal_not_by_registration() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        reg.attach(1).expect("a second window");
        reg.attach(1).expect("a third window");
        // Register out of order, and give the LAST-registered window the LOWEST
        // ordinal via a reservation — so insertion order and ordinal disagree.
        reg.register_window(
            BastydeWindowId::new(30),
            1,
            Some(3),
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        reg.register_window(
            BastydeWindowId::new(20),
            1,
            Some(2),
            Rc::new(|_| {}),
            inert_window_teardown(),
        );
        reg.register_window(
            BastydeWindowId::new(10),
            1,
            Some(1),
            Rc::new(|_| {}),
            inert_window_teardown(),
        );

        assert_eq!(
            reg.windows_for(1),
            vec![
                BastydeWindowId::new(10),
                BastydeWindowId::new(20),
                BastydeWindowId::new(30)
            ]
        );
    }

    /// A Work with two windows is still ONE open Work — the fact `QuitSequencer`
    /// depends on to prompt once per project rather than once per window.
    #[test]
    fn open_work_ids_lists_a_multiply_attached_work_exactly_once() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        reg.attach(1).expect("a second window attaches to Work 1");
        assert_eq!(reg.open_work_ids(), vec![1]);
        assert_eq!(
            reg.window_count_for(1),
            2,
            "…even though two windows hold it"
        );
    }
}
