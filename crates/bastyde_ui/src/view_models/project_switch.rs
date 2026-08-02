// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ProjectSwitchViewModel` — the unsaved-changes guard for **replacing this
//! window's project in place**.
//!
//! A window shows exactly one project (see `main.rs`'s launcher-window model),
//! and four commands swap it for another one *without going through a close*:
//!
//!   * **New Work** (Ctrl+N / File ▸ New Work) — `new_work`
//!   * **Open Work** (Ctrl+O / File ▸ Open Work…) — `load_work`
//!   * the project switcher's **"Open here"** — `load_work`
//!   * the Plume importer's **"Open now"** toast action — `load_work`
//!
//! `App`'s `LoadWork`/`NewWork` subscribers then call `EditorsViewModel::close_all`
//! — whose contract is explicitly *"does not flush — the outgoing work is
//! saved/discarded by the close flow"*. None of these four ran a close flow, so
//! every one of them silently destroyed the open project's unsaved edits: no
//! prompt, no save, no undo. The close paths (window X, Alt+F4, Ctrl+Q, Ctrl+W,
//! File ▸ Close Work) have always prompted; these simply never called that guard.
//!
//! **Phase 3 fix — the outgoing Work's backend subtree.** The backend no longer
//! closes the previous Work as part of `new_work`/`load_work` (Phase 2 removed
//! that sweep from `new_work_uc.rs`/`load_work_uc.rs` so two DIFFERENT Works can
//! coexist in two windows) — so [`Self::perform`]'s `OpenWork` branch now calls
//! `crate::app::close_outgoing_work` itself, right before `load_work`, to close
//! the SAME window's own outgoing Work explicitly. (The `NewWork` branch only
//! *shows the form* here — the actual replace happens later, on "Create Work",
//! so `NewWorkViewModel::create` closes the outgoing Work itself, right before
//! calling `new_work`; closing it here, before the form even appears, would
//! leave the window showing no project at all if the user then cancelled.) The
//! outgoing Work's id is threaded in as a plain parameter (`request`'s own
//! `outgoing_work_id`, down through `defer`/`perform`), resolved by each of the
//! four doors from THEIR OWN window's `AppIds` — never `ctx.app_state::<AppIds>()`,
//! which would answer with the wrong window's id once a second Work is open in a
//! second window (see `close_outgoing_work`'s own doc).
//!
//! This view-model *is* that guard, factored so all four doors share one branch
//! order — [`unsaved_decision`], which `work.close` also matches on, so the two
//! guards cannot drift apart:
//!
//! | open project | outcome |
//! |---|---|
//! | clean | switch now |
//! | dirty, autosave on | save, then switch when the write lands (no prompt) |
//! | dirty, autosave off | ask: **Save** / **Discard** / **Cancel** |
//! | dirty, backup mode | ask: **Discard** / **Cancel** — Save is off for a backup file; Save As and Restore are how those edits are kept |
//!
//! **The switch is deferred, not raced.** `save_work` is a long operation: it
//! returns immediately and writes on a background thread. Switching as soon as it
//! was *started* would tear the store out from under it (`close_current_work`
//! wipes the entities the background gather is reading). So a Save-branch switch
//! is parked in [`Self::pending`] and performed only when **its own** write lands.
//!
//! "Its own" is the load-bearing part. The switch waits on the **edit sequence**
//! its save covers ([`Self::on_saved`]), not on "a save finished": with autosave
//! on, a `save_work` can already be in flight when the guard asks for one, and its
//! snapshot may predate our flush. Firing the switch when *that* op lands would
//! wipe the store while the edits it never contained were still unwritten — the
//! last sentence typed would end up in no file at all. Waiting for
//! `saved_seq >= covers` is what makes "Save, then switch" mean it. (Not the op id
//! either: `save_queue` coalesces, so the op that finally carries our edits may be
//! a *follow-up* one, issued only when the in-flight save lands.)
//!
//! Single-instance live state: created in `main.rs` (where the `unsaved` /
//! `backup_mode` / autosave signals live) and registered as app-state; `App::build`
//! takes it from there to install its hooks and to serve the `work.new` /
//! `work.open` / `work.open_path` actions. The two doors that live *outside* `App`
//! — the project-switcher popover and the import toast — do **not** reach in for
//! this view-model: they fire the `work.open_path` intent, and `App` calls
//! [`Self::request`] for them. That is what keeps the view-model graph a DAG (see
//! the house rule: peers don't import peers; distant links graduate to the intent
//! bus). The two things only the view layer can do (write the editors to disk; put
//! the New Work form on screen) are injected by `App::build` as hooks — the same
//! idiom the backup scheduler uses for its flush.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::{
    EventContextMessageBoxExt, MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton,
    Toast,
};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::work_management::LoadWorkDto;

use crate::toast_scope::ToastWorkExt;

use super::long_op::CapturedWork;

/// A project switch, either performed at once or parked until the save lands.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum PendingSwitch {
    #[default]
    None,
    /// Put the New Work form on screen; it creates the work (in place) on confirm.
    NewWork,
    /// Load this already-chosen `.skrib` path over the open project.
    OpenWork(String),
}

/// What to do about the open project's unsaved edits before something takes it
/// away. **The one branch order every unsaved-changes guard in the app shares** —
/// the four switch doors here, and `work.close` (Ctrl+W / File ▸ Close Work).
///
/// Pure, so it is testable without a widget tree (this crate has no `EventContext`
/// harness), and single-sourced so the guards cannot silently drift apart: what
/// happens to your unsaved chapter must not depend on *which* command is about to
/// discard it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnsavedDecision {
    /// Nothing to protect — go ahead now.
    Proceed,
    /// Save first (autosave is on: the user already said "just save"), then go
    /// ahead when the write lands. No prompt.
    SaveThenProceed,
    /// Ask: Save / Discard / Cancel.
    PromptSaveDiscardCancel,
    /// Backup mode: the backup file is read-only, so there is no Save to offer —
    /// only Discard / Cancel. (Save As and Restore are the ways to keep the edits,
    /// both on the banner.)
    PromptDiscardOnly,
}

/// The shared branch order (see [`UnsavedDecision`]). `unsaved` is true whenever
/// the project has edits not yet on disk — including text still sitting in an
/// editor buffer, since typing bumps it through the editors' `edited` signal (see
/// `App::build`).
pub fn unsaved_decision(unsaved: bool, backup_mode: bool, autosave: bool) -> UnsavedDecision {
    match (unsaved, backup_mode, autosave) {
        (false, _, _) => UnsavedDecision::Proceed,
        (true, true, _) => UnsavedDecision::PromptDiscardOnly,
        (true, false, true) => UnsavedDecision::SaveThenProceed,
        (true, false, false) => UnsavedDecision::PromptSaveDiscardCancel,
    }
}

/// Start a save, returning its long-operation id when one was begun.
type SaveHook = Rc<dyn Fn() -> Option<u64>>;
/// Put the New Work form on screen.
type NewWorkFormHook = Rc<dyn Fn(&mut EventContext)>;

#[derive(Clone)]
pub struct ProjectSwitchViewModel {
    app_ctx: Rc<AppContext>,
    /// The open project has edits not yet on disk. Maintained by `App`.
    unsaved: Signal<bool>,
    /// A *backup file* is open here — read-only, so Save is off.
    backup_mode: Signal<bool>,
    autosave: Signal<bool>,
    /// The switch waiting for the in-flight save to land.
    pending: Signal<PendingSwitch>,
    /// The **edit sequence** [`Self::pending`] is waiting to see on disk (see
    /// `save_queue`): the switch fires when `saved_seq >= this`, not merely when
    /// "a save finished" — an autosave already in flight may have gathered the
    /// store before our flush.
    pending_seq: Rc<Cell<Option<u64>>>,
    /// The OUTGOING Work [`Self::pending`] will close (see
    /// `crate::app::close_outgoing_work`) once it fires — the caller's own
    /// window's `AppIds.work_id` at the moment [`Self::request`] was called,
    /// captured here (alongside [`Self::pending_seq`]) rather than re-read
    /// later: by the time a parked switch actually performs, `App::build`'s own
    /// `LoadWork` subscriber may already have re-seeded `AppIds` for an
    /// unrelated reason, and re-reading it then would name the wrong Work (or
    /// none at all). A [`super::long_op::CapturedWork`] — see that type's doc
    /// (and [`CapturedWork::given`] in particular: this view-model holds no
    /// `AppIds` of its own, so the capture happens one layer up, in each
    /// switch door's own window).
    pending_work_id: Rc<Cell<CapturedWork>>,
    /// Flush the editors and ask for a disk write, returning the edit sequence it
    /// will cover (`EditorsViewModel::request_save`). Installed by `App::build`,
    /// which is where the editors are created; a no-op until then, and in headless
    /// tests.
    save_hook: Rc<RefCell<SaveHook>>,
    /// Put the New Work form on screen (`App::build` presents the modal). A view
    /// concern, injected — this view-model owns *when* the form may appear, not
    /// what it looks like.
    new_work_form_hook: Rc<RefCell<NewWorkFormHook>>,
}

impl ProjectSwitchViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        unsaved: Signal<bool>,
        backup_mode: Signal<bool>,
        autosave: Signal<bool>,
    ) -> Self {
        Self {
            app_ctx,
            unsaved,
            backup_mode,
            autosave,
            pending: Signal::new(PendingSwitch::None),
            pending_seq: Rc::new(Cell::new(None)),
            pending_work_id: Rc::new(Cell::new(CapturedWork::none())),
            save_hook: Rc::new(RefCell::new(Rc::new(|| None) as Rc<dyn Fn() -> Option<u64>>)),
            new_work_form_hook: Rc::new(RefCell::new(
                Rc::new(|_: &mut EventContext| {}) as Rc<dyn Fn(&mut EventContext)>
            )),
        }
    }

    /// Install "flush the editors and ask for a disk write", from `App::build`.
    /// Visible on every clone already handed out (shared cell).
    pub fn set_save_hook(&self, hook: Rc<dyn Fn() -> Option<u64>>) {
        *self.save_hook.borrow_mut() = hook;
    }

    /// Install "present the New Work form", from `App::build`.
    pub fn set_new_work_form_hook(&self, hook: Rc<dyn Fn(&mut EventContext)>) {
        *self.new_work_form_hook.borrow_mut() = hook;
    }

    /// **The guard.** Every in-place project switch goes through here: decide what
    /// to do with the open project's unsaved edits, then switch (now, or once the
    /// save lands, or not at all).
    ///
    /// `outgoing_work_id` is the CALLER's own window's `AppIds.work_id.get()` —
    /// the Work this switch is about to replace, resolved by the caller from its
    /// own, correct-for-this-window handle rather than by this view-model reaching
    /// for the process-wide `app_state` slot (see `crate::app::close_outgoing_work`'s
    /// doc for why that distinction matters once a second Work can be open in a
    /// second window). `None` when nothing is open yet.
    pub fn request(
        &self,
        ctx: &mut EventContext,
        switch: PendingSwitch,
        outgoing_work_id: Option<u64>,
    ) {
        match unsaved_decision(
            self.unsaved.get(),
            self.backup_mode.get(),
            self.autosave.get(),
        ) {
            UnsavedDecision::Proceed => self.perform(ctx, switch, outgoing_work_id),
            UnsavedDecision::SaveThenProceed => self.defer(ctx, switch, outgoing_work_id),
            UnsavedDecision::PromptDiscardOnly => {
                let me = self.clone();
                ctx.present_message_box(
                    MessageBox::question(tr!(switch_backup_discard_title()))
                        .text(tr!(switch_backup_discard_text()))
                        .buttons(MessageBoxButtons::Custom(vec![
                            MessageBoxButton::standard(StandardButton::Discard),
                            MessageBoxButton::standard(StandardButton::Cancel),
                        ]))
                        .default_button(StandardButton::Cancel)
                        .escape_button(StandardButton::Cancel)
                        .on_result(move |r, ctx| {
                            if r.button == StandardButton::Discard {
                                me.perform(ctx, switch.clone(), outgoing_work_id);
                            }
                        }),
                );
            }
            UnsavedDecision::PromptSaveDiscardCancel => {
                let me = self.clone();
                let title = match switch {
                    PendingSwitch::NewWork => tr!(new_work_unsaved_question()),
                    _ => tr!(open_work_unsaved_question()),
                };
                ctx.present_message_box(
                    MessageBox::question(title)
                        .text(tr!(unsaved_changes()))
                        .buttons(MessageBoxButtons::SaveDiscardCancel)
                        .default_button(StandardButton::Save)
                        .escape_button(StandardButton::Cancel)
                        .on_result(move |r, ctx| match r.button {
                            StandardButton::Save => me.defer(ctx, switch.clone(), outgoing_work_id),
                            // Discard: the edits stay in the store, unsaved — and
                            // the switch below wipes it. That is what was asked for.
                            StandardButton::Discard => {
                                me.perform(ctx, switch.clone(), outgoing_work_id)
                            }
                            _ => {}
                        }),
                );
            }
        }
    }

    /// Park the switch and ask for a save. [`Self::on_saved`] performs it once the
    /// edits it covers are actually on disk — see the module docs on why this must
    /// not race the background gather.
    ///
    /// The switch is parked only if a save was really asked for. If the command
    /// could not be issued at all, nothing is parked: a switch waiting on a write
    /// that will never happen is a command that silently never happens.
    fn defer(&self, ctx: &mut EventContext, switch: PendingSwitch, outgoing_work_id: Option<u64>) {
        let save = self.save_hook.borrow().clone();
        let Some(covers) = save() else {
            // Work-scoped: about the outgoing Work's own save, not every open
            // window's.
            ctx.show_toast(
                Toast::error(tr!(switch_save_not_started())).target_work(outgoing_work_id),
            );
            return;
        };
        self.pending.set(switch);
        self.pending_seq.set(Some(covers));
        self.pending_work_id
            .set(CapturedWork::given(outgoing_work_id));
    }

    /// Do the switch. The point of no return: `OpenWork` closes the outgoing Work's
    /// backend subtree itself (see `crate::app::close_outgoing_work`) immediately
    /// before `load_work`; `NewWork` only shows the form here — the outgoing Work
    /// stays open (and closeable-again by Cancel) until `NewWorkViewModel::create`
    /// itself closes it, right before the actual `new_work` call.
    fn perform(
        &self,
        ctx: &mut EventContext,
        switch: PendingSwitch,
        outgoing_work_id: Option<u64>,
    ) {
        // Persist the outgoing project's desk (open tabs + docks) while its store is
        // still alive — the in-place switches fire no `CloseWork` of their own before
        // this point, and `load_work` (via `close_outgoing_work`, right below) closes
        // the current Work before anyone could translate a tab into its persistable
        // ordinal. (For New Work, the form is only *shown* here; the current desk
        // captured now is the one being left.)
        //
        // `ProjectSwitchViewModel` is a single, Tier-1 shared instance (see this
        // view-model's module doc) — a known, disclosed Phase-3 boundary, the same
        // shape as `tags::tag_chip`/`view_models::overview`'s app_state fallback. It
        // cannot yet resolve "this window's own `WorkspaceLayoutViewModel`" the way
        // `close_work_and_return_to_launcher`/`quit_app` now do, so it still reaches
        // for the process-wide `app_state` registration — correct only while this is
        // the first (and, for in-place New/Open, still the *only* still-live) window.
        // (Unlike this, `outgoing_work_id` above is NOT resolved this way — the
        // caller supplies its own window's real id — because closing the wrong
        // window's Work is a correctness hazard `capture`-ing the wrong layout is
        // not: see `close_outgoing_work`'s doc.)
        if let Some(workspace_layout) = ctx
            .app_state::<crate::view_models::WorkspaceLayoutViewModel>()
            .cloned()
        {
            crate::app::capture_workspace_layout(&workspace_layout);
        }
        match switch {
            PendingSwitch::None => {}
            PendingSwitch::NewWork => {
                let form = self.new_work_form_hook.borrow().clone();
                form(ctx);
            }
            PendingSwitch::OpenWork(path) => {
                // Close the SAME window's own outgoing Work before replacing it —
                // see the module doc and `close_outgoing_work`'s own doc. This is
                // the terminal step (no further confirmation follows), so closing
                // here, immediately before `load_work`, never leaves the window
                // showing nothing for longer than this one synchronous call.
                crate::app::close_outgoing_work(&self.app_ctx, outgoing_work_id);
                if let Err(e) = work_management_commands::load_work(
                    &self.app_ctx,
                    &LoadWorkDto {
                        file_name: path.clone(),
                    },
                ) {
                    // The outgoing Work is already closed at this point (see
                    // above), so there is no live Work to target any more —
                    // this window's own audience is about to become the new
                    // (never-opened) one. Origin-window default is correct.
                    ctx.show_toast(super::open_failure_toast(&path, &e));
                }
            }
        }
    }

    /// A save landed, and everything mutated up to `saved_seq` is now on disk. If
    /// that covers what the parked switch was waiting for, perform it.
    ///
    /// Keyed on the **edit sequence**, not on "a save finished" — and not on the
    /// op id either, because the save that finally carries our edits may not be the
    /// op we started: `SaveQueue` coalesces, so if another `save_work` was already
    /// in flight, ours is a *follow-up* op issued when that one lands. The sequence
    /// is what actually answers "are my edits on disk yet?".
    pub fn on_saved(&self, ctx: &mut EventContext, saved_seq: u64) {
        let Some(waiting_for) = self.pending_seq.get() else {
            return;
        };
        if saved_seq < waiting_for {
            return; // an earlier save landed; ours is still coming
        }
        let outgoing_work_id: Option<u64> = self.pending_work_id.get().into();
        let switch = self.take_pending();
        if switch != PendingSwitch::None {
            self.perform(ctx, switch, outgoing_work_id);
        }
    }

    /// The save the switch was waiting on failed. The project is still open and
    /// still dirty, so drop the parked switch and say why, instead of leaving the
    /// user with a New/Open that silently never happens. Nothing is lost: the edits
    /// are exactly where they were.
    ///
    /// `error` is `None` when the save never started, so there is no backend message
    /// to quote.
    ///
    /// `true` if a switch *was* parked — the caller has then already reported the
    /// failure (this toast says both that the save failed and that the switch
    /// didn't happen) and must not toast a second time.
    pub fn on_save_failed(&self, ctx: &mut EventContext, error: Option<&str>) -> bool {
        if self.pending_seq.get().is_none() {
            return false;
        }
        // Read BEFORE `take_pending()` — it clears `pending_work_id` as part of
        // dropping the parked switch, so the outgoing Work's id must be
        // captured first or every one of these toasts would silently fall
        // back to the origin-window default.
        let outgoing_work_id = self.pending_work_id.get();
        self.take_pending();
        ctx.show_toast(
            Toast::error(match error {
                Some(e) => tr!(switch_save_failed(error = e.to_string())),
                None => tr!(switch_save_not_started()),
            })
            .target_work(outgoing_work_id),
        );
        true
    }

    /// Abandon any parked switch (the close flow won the race: the project is
    /// leaving this window entirely, so switching it is moot).
    pub fn cancel(&self) {
        self.take_pending();
    }

    fn take_pending(&self) -> PendingSwitch {
        let switch = self.pending.get();
        if switch != PendingSwitch::None {
            self.pending.set(PendingSwitch::None);
        }
        self.pending_seq.set(None);
        self.pending_work_id.set(CapturedWork::none());
        switch
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn test_vm(unsaved: bool, backup_mode: bool, autosave: bool) -> ProjectSwitchViewModel {
        ProjectSwitchViewModel::new(
            Rc::new(AppContext::new()),
            Signal::new(unsaved),
            Signal::new(backup_mode),
            Signal::new(autosave),
        )
    }

    // ── the branch order (pure) ──────────────────────────────────────────────

    #[test]
    fn a_clean_project_switches_with_no_prompt() {
        for backup in [false, true] {
            for autosave in [false, true] {
                assert_eq!(
                    unsaved_decision(false, backup, autosave),
                    UnsavedDecision::Proceed,
                    "nothing to protect (backup={backup}, autosave={autosave})"
                );
            }
        }
    }

    #[test]
    fn a_dirty_project_prompts_when_autosave_is_off() {
        assert_eq!(
            unsaved_decision(true, false, false),
            UnsavedDecision::PromptSaveDiscardCancel
        );
    }

    #[test]
    fn a_dirty_project_saves_first_when_autosave_is_on() {
        assert_eq!(
            unsaved_decision(true, false, true),
            UnsavedDecision::SaveThenProceed
        );
    }

    #[test]
    fn backup_mode_can_only_discard_because_save_is_off_there() {
        // Autosave must not silently "save" a backup window — `save_to_disk` is
        // inert there, so the switch would proceed having written nothing.
        for autosave in [false, true] {
            assert_eq!(
                unsaved_decision(true, true, autosave),
                UnsavedDecision::PromptDiscardOnly,
                "autosave={autosave}"
            );
        }
    }

    // ── the deferral (needs no `EventContext`) ───────────────────────────────

    /// `defer` needs an `EventContext` only for its "the save never started" toast,
    /// and this crate has no `EventContext` harness (see `backup_scheduler.rs`).
    /// This is the ctx-free core it is built on: ask for the save, park the switch
    /// against the edit sequence that save will cover, and the outgoing Work it
    /// will close once it fires.
    fn defer_headless(
        vm: &ProjectSwitchViewModel,
        switch: PendingSwitch,
        outgoing_work_id: Option<u64>,
    ) {
        let save = vm.save_hook.borrow().clone();
        if let Some(covers) = save() {
            vm.pending.set(switch);
            vm.pending_seq.set(Some(covers));
            vm.pending_work_id
                .set(CapturedWork::given(outgoing_work_id));
        }
    }

    #[test]
    fn deferring_saves_and_parks_the_switch_until_the_write_lands() {
        let vm = test_vm(true, false, true);
        let saves = Rc::new(Cell::new(0u32));
        {
            let saves = saves.clone();
            vm.set_save_hook(Rc::new(move || {
                saves.set(saves.get() + 1);
                Some(7)
            }));
        }
        defer_headless(
            &vm,
            PendingSwitch::OpenWork("/tmp/other.skrib".into()),
            Some(42),
        );
        assert_eq!(saves.get(), 1, "the deferral must ask for the disk write");
        assert_eq!(
            vm.pending.get(),
            PendingSwitch::OpenWork("/tmp/other.skrib".into()),
            "the switch must be parked, not performed — the write is still in flight"
        );
        assert_eq!(vm.pending_seq.get(), Some(7));
        assert_eq!(
            vm.pending_work_id.get(),
            Some(42),
            "the outgoing Work to close once the switch fires must be parked too"
        );
    }

    #[test]
    fn a_switch_waits_for_the_save_that_covers_its_edits() {
        // The race this closes: with autosave on, a `save_work` can already be in
        // flight when the guard asks for one, and its snapshot may predate our
        // flush. Releasing the switch when *that* one lands would wipe the store
        // while the last sentence typed was still unwritten. So an earlier save
        // landing (a lower sequence) must not release it.
        let vm = test_vm(true, false, true);
        vm.set_save_hook(Rc::new(|| Some(9))); // our edits are at seq 9
        defer_headless(&vm, PendingSwitch::NewWork, Some(7));

        assert!(
            vm.pending_seq.get().is_some_and(|s| 8 < s),
            "a save covering only seq 8 does not cover our seq-9 edits"
        );
        assert_eq!(
            vm.pending.get(),
            PendingSwitch::NewWork,
            "still parked: the save that landed predates our edits"
        );

        assert!(
            vm.pending_seq.get().is_some_and(|s| 9 >= s),
            "a save covering seq 9 does cover them"
        );
        assert_eq!(vm.take_pending(), PendingSwitch::NewWork);
        assert_eq!(vm.pending.get(), PendingSwitch::None);
        assert!(vm.pending_seq.get().is_none());
        assert_eq!(
            vm.pending_work_id.get(),
            None,
            "take_pending must drop the parked outgoing-Work id too"
        );
    }

    #[test]
    fn a_save_that_never_started_parks_nothing() {
        // A switch waiting on a write that will never happen is a command that
        // silently never happens. Better to report it and stay put.
        let vm = test_vm(true, false, true);
        vm.set_save_hook(Rc::new(|| None)); // the command could not be issued
        defer_headless(&vm, PendingSwitch::NewWork, Some(7));
        assert_eq!(vm.pending.get(), PendingSwitch::None);
        assert!(vm.pending_seq.get().is_none());
        assert_eq!(vm.pending_work_id.get(), None);
    }

    #[test]
    fn cancel_abandons_a_parked_switch() {
        // The close flow wins the race: the project is leaving the window, so the
        // parked switch must not fire behind it.
        let vm = test_vm(true, false, true);
        vm.set_save_hook(Rc::new(|| Some(1)));
        defer_headless(&vm, PendingSwitch::NewWork, Some(7));
        vm.cancel();
        assert_eq!(vm.pending.get(), PendingSwitch::None);
        assert!(vm.pending_seq.get().is_none());
        assert_eq!(vm.pending_work_id.get(), None);
    }

    #[test]
    fn hooks_default_to_harmless_no_ops_and_are_visible_on_earlier_clones() {
        // Un-hooked (the headless shape): deferring must not panic, and parks
        // nothing to wait on.
        let vm = test_vm(true, false, true);
        defer_headless(&vm, PendingSwitch::NewWork, Some(7));
        assert!(vm.pending_seq.get().is_none());

        // The hook cells are shared, so installing from `App::build` reaches the
        // clones handed out earlier (the switcher popover, the import toast).
        let earlier_clone = vm.clone();
        let saves = Rc::new(Cell::new(0u32));
        {
            let saves = saves.clone();
            vm.set_save_hook(Rc::new(move || {
                saves.set(saves.get() + 1);
                None
            }));
        }
        defer_headless(&earlier_clone, PendingSwitch::NewWork, Some(7));
        assert_eq!(saves.get(), 1, "the earlier clone must see the new hook");
    }
}
