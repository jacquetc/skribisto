// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

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

/// A `WindowTeardown` that records every `is_last` it was called with, plus a
/// handle to read them back — the same shape as the stack one above, because
/// the window teardown now takes that answer too (it performs the project's
/// close-out on the last window standing).
fn tracked_window_teardown() -> (WindowTeardown, Rc<RefCell<Vec<bool>>>) {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let seen = calls.clone();
    (
        Rc::new(move |is_last| seen.borrow_mut().push(is_last)),
        calls,
    )
}

fn inert_window_teardown() -> WindowTeardown {
    Rc::new(|_| {})
}

#[test]
fn removing_a_works_only_window_runs_teardown_as_last_and_drops_the_session() {
    let reg = WorkRegistry::new();
    reg.register(1, fixture_session());
    let (stack_teardown, stack_calls) = tracked_stack_teardown();
    let (window_teardown, window_calls) = tracked_window_teardown();
    reg.register_window(
        TeksiloWindowId::new(1),
        1,
        None,
        stack_teardown,
        window_teardown,
    );

    reg.remove_window(TeksiloWindowId::new(1));

    assert_eq!(
        *stack_calls.borrow(),
        vec![true],
        "the only window on Work 1 is the last one"
    );
    assert_eq!(
        *window_calls.borrow(),
        vec![true],
        "a real close must run the window's own teardown, told it was the last one — \
             which is what makes it perform the project's close-out"
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
        TeksiloWindowId::new(1),
        1,
        None,
        first_stack,
        inert_window_teardown(),
    );
    reg.register_window(
        TeksiloWindowId::new(2),
        1,
        None,
        second_stack,
        inert_window_teardown(),
    );

    reg.remove_window(TeksiloWindowId::new(1));
    assert_eq!(
        *first_stack_calls.borrow(),
        vec![false],
        "a sibling window still holds Work 1"
    );
    assert!(
        reg.session_for(1).is_some(),
        "the session must survive while a sibling window is open"
    );

    reg.remove_window(TeksiloWindowId::new(2));
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

/// The return value is the gate the heap trim rides on: a project window closing
/// while a sibling still holds the same `Work` has released nothing, so walking
/// every allocator arena there would cost milliseconds and return no pages.
/// See `crate::heap::release_free_pages` and its call in `remove_window`.
#[test]
fn only_the_last_window_on_a_work_reports_the_work_gone() {
    let reg = WorkRegistry::new();
    let (first_stack, _first_stack_calls) = tracked_stack_teardown();
    let (second_stack, _second_stack_calls) = tracked_stack_teardown();
    reg.register(1, fixture_session());
    reg.attach(1)
        .expect("a second window attaches to the same Work");
    reg.register_window(
        TeksiloWindowId::new(1),
        1,
        None,
        first_stack,
        inert_window_teardown(),
    );
    reg.register_window(
        TeksiloWindowId::new(2),
        1,
        None,
        second_stack,
        inert_window_teardown(),
    );

    assert!(
        !reg.remove_window(TeksiloWindowId::new(1)),
        "a sibling window still holds the Work, so nothing has been released"
    );
    assert!(
        reg.remove_window(TeksiloWindowId::new(2)),
        "the last window closing is what actually frees the project"
    );
    assert!(
        !reg.remove_window(TeksiloWindowId::new(2)),
        "a window this registry no longer knows frees nothing either"
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
        TeksiloWindowId::new(1),
        1,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );
    reg.register_window(
        TeksiloWindowId::new(2),
        2,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );

    reg.remove_window(TeksiloWindowId::new(1));

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
    reg.remove_window(TeksiloWindowId::new(99));
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
        TeksiloWindowId::new(1),
        1,
        None,
        Rc::new(|_| {}),
        window_teardown,
    );

    reg.remove_window(TeksiloWindowId::new(1));
    reg.remove_window(TeksiloWindowId::new(1));

    assert_eq!(
        *window_calls.borrow(),
        vec![true],
        "the second removal of the same window must not re-run its teardown"
    );
}

// ── in-place Work switch (`register_window` superseding its own binding) ──
//
// File > New Work / Open Work, and the ProjectSwitcher's "Open here", all
// reload the SAME window onto a different Work without ever destroying
// it — so teksilo's `on_removed` never fires for the Work being left. The
// regression these tests guard: `register_window` used to silently drop
// the previous `WindowEntry` (and its teardown) on replace, permanently
// leaking the old Work's `WorkRegistry` entry and undo stack.

#[test]
fn switching_a_windows_only_work_in_place_tears_down_the_old_session_and_stack() {
    let reg = WorkRegistry::new();
    reg.register(1, fixture_session());
    let (old_stack, old_stack_calls) = tracked_stack_teardown();
    let (old_window, old_window_calls) = tracked_window_teardown();
    reg.register_window(TeksiloWindowId::new(1), 1, None, old_stack, old_window);

    // The same window now loads Work 2 in place (no `remove_window` call —
    // the window was never destroyed).
    reg.register(2, fixture_session());
    let (new_stack, _new_stack_calls) = tracked_stack_teardown();
    reg.register_window(
        TeksiloWindowId::new(1),
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
    assert!(
        old_window_calls.borrow().is_empty(),
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
        TeksiloWindowId::new(1),
        1,
        None,
        window_a_stack,
        inert_window_teardown(),
    );
    reg.register_window(
        TeksiloWindowId::new(2),
        1,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );

    // Window 1 switches to Work 2 in place; window 2 still shows Work 1.
    reg.register(2, fixture_session());
    reg.register_window(
        TeksiloWindowId::new(1),
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
        TeksiloWindowId::new(1),
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
        TeksiloWindowId::new(1),
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
    reg.remove_window(TeksiloWindowId::new(1));
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
        TeksiloWindowId::new(1),
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
        TeksiloWindowId::new(1),
        1,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );
    let second = reg.register_window(
        TeksiloWindowId::new(2),
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
        TeksiloWindowId::new(1),
        1,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );
    reg.register_window(
        TeksiloWindowId::new(2),
        1,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );

    // Window 1 (ordinal 1) closes; window 2 (ordinal 2) stays open.
    reg.remove_window(TeksiloWindowId::new(1));
    assert!(reg.session_for(1).is_some(), "window 2 keeps Work 1 open");

    // A third window now attaches to the still-open Work 1.
    reg.attach(1)
        .expect("a third window attaches to the still-open Work");
    let third = reg.register_window(
        TeksiloWindowId::new(3),
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
        TeksiloWindowId::new(1),
        1,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );

    reg.register(1, fixture_session());
    let reloaded = reg.register_window(
        TeksiloWindowId::new(1),
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
        TeksiloWindowId::new(1),
        1,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );

    // A second, unrelated window is already the "second" window on Work 2.
    reg.register(2, fixture_session());
    reg.attach(2).expect("a second window attaches to Work 2");
    reg.register_window(
        TeksiloWindowId::new(2),
        2,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );

    // Window 1 now switches, in place, onto Work 2 — it becomes Work 2's
    // second window, not a reuse of its own former ordinal on Work 1.
    let switched = reg.register_window(
        TeksiloWindowId::new(1),
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
        TeksiloWindowId::new(1),
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
        TeksiloWindowId::new(2),
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
        TeksiloWindowId::new(1),
        1,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );

    // A rebuild that somehow carried a reservation must still keep this
    // window's established number — a title a KWin rule matched against
    // must not change under it (see `next_window_ordinal`'s doc).
    let again = reg.register_window(
        TeksiloWindowId::new(1),
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
/// build the same `attached_window_id_for` string id, and teksilo's window
/// map overwrites rather than rejects a duplicate.
#[test]
fn a_reservation_never_collides_with_a_live_window_whose_counter_lags() {
    let reg = WorkRegistry::new();
    reg.register(1, fixture_session());
    // Bind window 1 with an explicit ordinal, leaving the entry's own
    // counter untouched at 1 — the lagging-counter case.
    reg.register_window(
        TeksiloWindowId::new(1),
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
        TeksiloWindowId::new(1),
        1,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );
    reg.register_window(
        TeksiloWindowId::new(2),
        1,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );
    reg.register_window(
        TeksiloWindowId::new(3),
        2,
        None,
        Rc::new(|_| {}),
        inert_window_teardown(),
    );

    assert_eq!(
        reg.windows_for(1),
        vec![TeksiloWindowId::new(1), TeksiloWindowId::new(2)],
        "both of Work 1's windows, ordinal order"
    );
    assert_eq!(
        reg.windows_for(2),
        vec![TeksiloWindowId::new(3)],
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
        TeksiloWindowId::new(30),
        1,
        Some(3),
        Rc::new(|_| {}),
        inert_window_teardown(),
    );
    reg.register_window(
        TeksiloWindowId::new(20),
        1,
        Some(2),
        Rc::new(|_| {}),
        inert_window_teardown(),
    );
    reg.register_window(
        TeksiloWindowId::new(10),
        1,
        Some(1),
        Rc::new(|_| {}),
        inert_window_teardown(),
    );

    assert_eq!(
        reg.windows_for(1),
        vec![
            TeksiloWindowId::new(10),
            TeksiloWindowId::new(20),
            TeksiloWindowId::new(30)
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
