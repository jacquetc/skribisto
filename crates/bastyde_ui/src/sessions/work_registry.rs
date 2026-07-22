// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `WorkRegistry` — the app-global (Tier 1) home for the one piece of id-only
//! state that is genuinely process-wide (`root_id`), plus the seam Phase 2 will
//! turn into a real per-`work_id` lookup table.
//!
//! Today there is at most one live [`WorkSession`](super::WorkSession) — every
//! Work-touching UI feature still assumes a single open project (the "close
//! every other open Work" sweep in `load_work_uc`/`new_work_uc` guarantees it).
//! [`WorkRegistry::current`]/[`WorkRegistry::set_current`] simply hold that one
//! session; [`WorkRegistry::session_for`] answers "the session for this
//! `work_id`" by checking whether the current session is actually pointed at
//! it — a real, correct answer today (there is nothing else it could be), and
//! exactly the query Phase 2's `ProjectWindowFactory::window_config` will keep
//! calling once this becomes a genuine `HashMap<work_id, WorkSession>` (create-
//! or-share, refcounted per attached window). Building the query surface now,
//! against the single-instance answer, is what makes that swap a pure
//! implementation change later rather than a call-site hunt.

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::prelude::Signal;

use super::WorkSession;

/// App-global registry: `root_id` (Tier 1 — one `Root` per process, never
/// per-Work) plus the current [`WorkSession`], if one is open.
#[derive(Clone)]
pub struct WorkRegistry {
    root_id: Signal<Option<u64>>,
    current: Rc<RefCell<Option<WorkSession>>>,
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
            current: Rc::new(RefCell::new(None)),
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

    /// Register (or replace) the one live session. Phase 1 calls this once, at
    /// startup, with the single `WorkSession` `main.rs` builds — the session's
    /// own ids get re-pointed on every `LoadWork`/`NewWork`/`CloseWork`, so the
    /// same registered instance stays current across a project switch.
    pub fn set_current(&self, session: WorkSession) {
        *self.current.borrow_mut() = Some(session);
    }

    /// The current session, if any Work is open. Phase 1 has no caller yet
    /// (every consumer still reaches Tier-2 state via the `app_state`
    /// registrations `main.rs` also keeps, or via the `WorkSession` `App`
    /// receives as a constructor parameter) — this and `session_for` are the
    /// query surface Phase 2's per-window resolution will call instead.
    #[allow(dead_code)] // Phase 2's resolution mechanism; exercised by this module's own tests today
    pub fn current(&self) -> Option<WorkSession> {
        self.current.borrow().clone()
    }

    /// The session for `work_id`, if it is the one currently open. `None` both
    /// when nothing is open and when `work_id` names some *other* Work — today
    /// there is only ever one live session, so this can only ever answer with
    /// it or with nothing; Phase 2 turns this into a real table lookup without
    /// changing what callers do with the answer.
    #[allow(dead_code)] // Phase 2's resolution mechanism; exercised by this module's own tests today
    pub fn session_for(&self, work_id: u64) -> Option<WorkSession> {
        self.current()
            .filter(|s| s.ids.work_id.get() == Some(work_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_session() -> WorkSession {
        WorkSession::for_test()
    }

    #[test]
    fn a_fresh_registry_has_no_root_and_no_session() {
        let reg = WorkRegistry::new();
        assert_eq!(reg.root_id().get(), None);
        assert!(reg.current().is_none());
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
        let session = fixture_session();
        session.ids.work_id.set(Some(42));
        reg.set_current(session);

        assert!(reg.session_for(42).is_some(), "the open Work's own id must resolve");
        assert!(
            reg.session_for(99).is_none(),
            "a different id must not resolve to today's one session"
        );
    }

    #[test]
    fn no_session_means_nothing_resolves() {
        let reg = WorkRegistry::new();
        assert!(reg.session_for(1).is_none());
    }
}
