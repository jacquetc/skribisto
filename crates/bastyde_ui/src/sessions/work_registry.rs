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
//! "create-or-share, refcounted" resolution mechanism — today it is only ever
//! a share (a genuine *create* path needs `PendingAction` to carry a known
//! `work_id` up front, which is Phase 3's `AttachExisting{work_id}`, not yet
//! built) — but the table shape is the real, final one: a second window on an
//! *already* open Work will `attach`/`session_for` this map directly, no
//! further change needed here.

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
            });
    }

    /// A further window is attaching to an *already-registered* `work_id`'s
    /// session (Phase 3's `AttachExisting`). Returns the shared session, or
    /// `None` if `work_id` names no currently-open Work. Bumps the refcount on
    /// success — the caller must pair this with exactly one
    /// [`unregister`](Self::unregister) when its window closes.
    #[allow(dead_code)] // Phase 3's AttachExisting; exercised by this module's own tests today
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
    /// window — the create-or-share query `ProjectWindowFactory` will call
    /// once `PendingAction::AttachExisting{work_id}` exists (Phase 3). Today
    /// nothing in this crate calls it outside this module's own tests: Phase 2
    /// only ever registers a `work_id` it just this instant created (via
    /// `register`), never resolves one ahead of a window's own load.
    #[allow(dead_code)] // Phase 3's resolution mechanism; exercised by this module's own tests today
    pub fn session_for(&self, work_id: u64) -> Option<WorkSession> {
        self.sessions
            .borrow()
            .get(&work_id)
            .map(|e| e.session.clone())
    }

    /// Every currently-open Work's id — the query `ProjectSwitcher`/`app.quit`'s
    /// dirty-Works sweep will use (Phase 3). Exercised by this module's own
    /// tests today.
    #[allow(dead_code)] // Phase 3's ProjectSwitcher / app.quit sweep
    pub fn open_work_ids(&self) -> Vec<u64> {
        self.sessions.borrow().keys().copied().collect()
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
    pub fn register_window(
        &self,
        window_id: BastydeWindowId,
        work_id: u64,
        stack_teardown: StackTeardown,
        window_teardown: WindowTeardown,
    ) {
        let previous = self.windows.borrow_mut().insert(
            window_id,
            WindowEntry {
                work_id,
                stack_teardown,
                window_teardown,
            },
        );
        if let Some(previous) = previous {
            let is_last = self.unregister(previous.work_id);
            (previous.stack_teardown)(is_last);
        }
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

        assert!(reg.session_for(42).is_some(), "a registered Work's id must resolve");
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
        assert!(reg.session_for(2).is_some(), "Work 2 must survive Work 1 closing");
    }

    #[test]
    fn attach_shares_the_same_instance_and_refcounts_it() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());

        let shared = reg.attach(1).expect("already-open Work must attach");
        // Two windows now hold Work 1 — one `unregister` must not tear it down.
        assert!(!reg.unregister(1), "one of two windows closing must not finalize teardown");
        assert!(reg.session_for(1).is_some(), "the other window's session must still be live");
        assert!(reg.unregister(1), "the last window closing must finalize teardown");
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
        (Rc::new(move |is_last| seen.borrow_mut().push(is_last)), calls)
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
        reg.register_window(BastydeWindowId::new(1), 1, stack_teardown, window_teardown);

        reg.remove_window(BastydeWindowId::new(1));

        assert_eq!(*stack_calls.borrow(), vec![true], "the only window on Work 1 is the last one");
        assert_eq!(*window_calls.borrow(), 1, "a real close must run the window's own teardown");
        assert!(reg.session_for(1).is_none(), "the session must be torn down with it");
    }

    #[test]
    fn removing_one_of_two_windows_on_a_work_is_not_last_and_keeps_the_session() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        reg.attach(1).expect("a second window attaches to the same Work");
        let (first_stack, first_stack_calls) = tracked_stack_teardown();
        let (second_stack, second_stack_calls) = tracked_stack_teardown();
        reg.register_window(BastydeWindowId::new(1), 1, first_stack, inert_window_teardown());
        reg.register_window(BastydeWindowId::new(2), 1, second_stack, inert_window_teardown());

        reg.remove_window(BastydeWindowId::new(1));
        assert_eq!(*first_stack_calls.borrow(), vec![false], "a sibling window still holds Work 1");
        assert!(reg.session_for(1).is_some(), "the session must survive while a sibling window is open");

        reg.remove_window(BastydeWindowId::new(2));
        assert_eq!(*second_stack_calls.borrow(), vec![true], "this was the last window standing");
        assert!(reg.session_for(1).is_none(), "the session must be torn down once its last window closes");
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
        reg.register_window(BastydeWindowId::new(1), 1, Rc::new(|_| {}), inert_window_teardown());
        reg.register_window(BastydeWindowId::new(2), 2, Rc::new(|_| {}), inert_window_teardown());

        reg.remove_window(BastydeWindowId::new(1));

        assert!(reg.session_for(1).is_none(), "Work 1's window closed");
        assert!(reg.session_for(2).is_some(), "Work 2 must be untouched by Work 1's window closing");
    }

    #[test]
    fn removing_an_unbound_window_is_a_safe_no_op() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        // No `register_window` call for this id — nothing was ever bound.
        reg.remove_window(BastydeWindowId::new(99));
        assert!(reg.session_for(1).is_some(), "an unrelated window's removal must not touch Work 1");
    }

    #[test]
    fn removing_a_window_twice_is_a_safe_no_op_the_second_time() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        let (window_teardown, window_calls) = tracked_window_teardown();
        reg.register_window(BastydeWindowId::new(1), 1, Rc::new(|_| {}), window_teardown);

        reg.remove_window(BastydeWindowId::new(1));
        reg.remove_window(BastydeWindowId::new(1));

        assert_eq!(*window_calls.borrow(), 1, "the second removal of the same window must not re-run its teardown");
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
        reg.register_window(BastydeWindowId::new(1), 1, old_stack, old_window);

        // The same window now loads Work 2 in place (no `remove_window` call —
        // the window was never destroyed).
        reg.register(2, fixture_session());
        let (new_stack, _new_stack_calls) = tracked_stack_teardown();
        reg.register_window(BastydeWindowId::new(1), 2, new_stack, inert_window_teardown());

        assert_eq!(
            *old_stack_calls.borrow(),
            vec![true],
            "Work 1 had no other window on it, so switching away in place must be reported as last"
        );
        assert!(
            reg.session_for(1).is_none(),
            "Work 1's session must be torn down once its only window switched away in place"
        );
        assert!(reg.session_for(2).is_some(), "Work 2 must now be the window's live session");
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
        reg.attach(1).expect("a second window attaches to the same Work");
        let (window_a_stack, window_a_calls) = tracked_stack_teardown();
        reg.register_window(BastydeWindowId::new(1), 1, window_a_stack, inert_window_teardown());
        reg.register_window(BastydeWindowId::new(2), 1, Rc::new(|_| {}), inert_window_teardown());

        // Window 1 switches to Work 2 in place; window 2 still shows Work 1.
        reg.register(2, fixture_session());
        reg.register_window(BastydeWindowId::new(1), 2, Rc::new(|_| {}), inert_window_teardown());

        assert_eq!(
            *window_a_calls.borrow(),
            vec![false],
            "window 2 still shows Work 1 — switching window 1 away must not be reported as last"
        );
        assert!(reg.session_for(1).is_some(), "Work 1 must survive: window 2 still shows it");
    }

    #[test]
    fn reloading_the_same_work_in_place_does_not_prematurely_tear_it_down() {
        let reg = WorkRegistry::new();
        reg.register(1, fixture_session());
        let (first_stack, first_calls) = tracked_stack_teardown();
        reg.register_window(BastydeWindowId::new(1), 1, first_stack, inert_window_teardown());

        // The window reloads the very same Work (e.g. a Revert-style re-load):
        // `register` is called again for `work_id` 1, bumping its refcount a
        // second time before `register_window` supersedes its own binding.
        reg.register(1, fixture_session());
        let (second_stack, _second_calls) = tracked_stack_teardown();
        reg.register_window(BastydeWindowId::new(1), 1, second_stack, inert_window_teardown());

        assert_eq!(
            *first_calls.borrow(),
            vec![false],
            "the redundant refcount bump must be cancelled without ever reporting a real \
             last-window teardown for a Work this window still shows"
        );
        assert!(reg.session_for(1).is_some(), "Work 1 must still be open after reloading in place");

        // The window finally closes for real — now it really is last.
        reg.remove_window(BastydeWindowId::new(1));
        assert!(reg.session_for(1).is_none());
    }
}
