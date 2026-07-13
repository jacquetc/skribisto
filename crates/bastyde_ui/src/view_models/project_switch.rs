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
//! Both backend use cases call `work_io::close_current_work` as their first act
//! (`new_work_uc.rs` / `load_work_uc.rs`: "Opening/creating a work replaces the
//! currently-open one"), and `App`'s `LoadWork`/`NewWork` subscribers then call
//! `EditorsViewModel::close_all` — whose contract is explicitly *"does not flush
//! — the outgoing work is saved/discarded by the close flow"*. None of these four
//! ran a close flow, so every one of them silently destroyed the open project's
//! unsaved edits: no prompt, no save, no undo. The close paths (window X, Alt+F4,
//! Ctrl+Q, Ctrl+W, File ▸ Close Work) have always prompted; these simply never
//! called that guard.
//!
//! This view-model *is* that guard, factored so all four doors share one branch
//! order — the same one the close guard uses ([`switch_decision`]):
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
//! "Its own" is the load-bearing part, and it is why both the completion and the
//! failure path key on the long-operation **id** ([`Self::on_long_op_completed`] /
//! [`Self::on_save_failed`]) rather than on `WorkManagementEvent::SaveWork`, which
//! carries no id: with autosave on, a `save_work` can already be in flight when
//! the guard kicks its own. Firing the switch on *that* op's completion would
//! wipe the store while our save is still queued — and if the older op's gather
//! ran before our flush, the last sentence the user typed would end up in no file
//! at all. Matching the id also means a failing backup or import can neither fire
//! nor cancel a switch waiting on a save.
//!
//! Single-instance live state: created in `main.rs` (where the `unsaved` /
//! `backup_mode` / autosave signals live) and registered as app-state, so the two
//! doors outside `App` — the project-switcher popover and the import toast —
//! reach it with `ctx.app_state::<ProjectSwitchViewModel>()`. The two things only
//! the view layer can do (write the editors to disk; put the New Work form on
//! screen) are injected by `App::build` as hooks, the same idiom the backup
//! scheduler uses for its flush.

use std::cell::RefCell;
use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::{
    EventContextMessageBoxExt, MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton,
    Toast,
};

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::common::event::Event;
use frontend::work_management::LoadWorkDto;

use super::long_op::{event_id, parse_payload};

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

/// How a switch resolves against the open project's unsaved edits. Pure, so the
/// branch order is testable without a widget tree (this crate has no
/// `EventContext` harness) — and identical to the close guard's.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SwitchDecision {
    /// Nothing to protect — switch now.
    Proceed,
    /// Save first (autosave is on: the user already said "just save"), then switch
    /// when the write lands. No prompt.
    SaveThenSwitch,
    /// Ask: Save / Discard / Cancel.
    PromptSaveDiscardCancel,
    /// Backup mode: the backup file is read-only, so there is no Save to offer —
    /// only Discard / Cancel. (Save As and Restore are the ways to keep the edits,
    /// both on the banner.)
    PromptDiscardOnly,
}

/// The guard's branch order. `unsaved` is true whenever the project has edits not
/// yet on disk — including text still sitting in an editor buffer, since typing
/// bumps it through the editors' `edited` signal (see `App::build`).
pub fn switch_decision(unsaved: bool, backup_mode: bool, autosave: bool) -> SwitchDecision {
    match (unsaved, backup_mode, autosave) {
        (false, _, _) => SwitchDecision::Proceed,
        (true, true, _) => SwitchDecision::PromptDiscardOnly,
        (true, false, true) => SwitchDecision::SaveThenSwitch,
        (true, false, false) => SwitchDecision::PromptSaveDiscardCancel,
    }
}

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
    /// The `save_work` operation [`Self::pending`] is waiting on, so a *different*
    /// long op's failure (a backup, an import) can't cancel the switch.
    pending_op: Rc<RefCell<Option<String>>>,
    /// Flush the editors and start the disk write, returning its op id
    /// (`EditorsViewModel::save_to_disk_op`). Installed by `App::build`, which is
    /// where the editors are created; a no-op until then, and in headless tests.
    save_hook: Rc<RefCell<Rc<dyn Fn() -> Option<String>>>>,
    /// Put the New Work form on screen (`App::build` presents the modal). A view
    /// concern, injected — this view-model owns *when* the form may appear, not
    /// what it looks like.
    new_work_form_hook: Rc<RefCell<Rc<dyn Fn(&mut EventContext)>>>,
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
            pending_op: Rc::new(RefCell::new(None)),
            save_hook: Rc::new(RefCell::new(
                Rc::new(|| None) as Rc<dyn Fn() -> Option<String>>
            )),
            new_work_form_hook: Rc::new(RefCell::new(
                Rc::new(|_: &mut EventContext| {}) as Rc<dyn Fn(&mut EventContext)>
            )),
        }
    }

    /// Install "flush the editors and write the project to disk", from `App::build`.
    /// Visible on every clone already handed out (shared cell).
    pub fn set_save_hook(&self, hook: Rc<dyn Fn() -> Option<String>>) {
        *self.save_hook.borrow_mut() = hook;
    }

    /// Install "present the New Work form", from `App::build`.
    pub fn set_new_work_form_hook(&self, hook: Rc<dyn Fn(&mut EventContext)>) {
        *self.new_work_form_hook.borrow_mut() = hook;
    }

    /// **The guard.** Every in-place project switch goes through here: decide what
    /// to do with the open project's unsaved edits, then switch (now, or once the
    /// save lands, or not at all).
    pub fn request(&self, ctx: &mut EventContext, switch: PendingSwitch) {
        match switch_decision(
            self.unsaved.get(),
            self.backup_mode.get(),
            self.autosave.get(),
        ) {
            SwitchDecision::Proceed => self.perform(ctx, switch),
            SwitchDecision::SaveThenSwitch => self.defer(ctx, switch),
            SwitchDecision::PromptDiscardOnly => {
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
                                me.perform(ctx, switch.clone());
                            }
                        }),
                );
            }
            SwitchDecision::PromptSaveDiscardCancel => {
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
                            StandardButton::Save => me.defer(ctx, switch.clone()),
                            // Discard: the edits stay in the store, unsaved — and
                            // the switch below wipes it. That is what was asked for.
                            StandardButton::Discard => me.perform(ctx, switch.clone()),
                            _ => {}
                        }),
                );
            }
        }
    }

    /// Park the switch and kick the (asynchronous) save.
    /// [`Self::on_long_op_completed`] performs it once **that** write actually
    /// lands — see the module docs on why this must not race the background gather.
    ///
    /// The switch is parked only if the save really started. If the command could
    /// not be issued at all, nothing is parked: a switch waiting on an operation
    /// that will never complete is a command that silently never happens.
    fn defer(&self, ctx: &mut EventContext, switch: PendingSwitch) {
        let save = self.save_hook.borrow().clone();
        let Some(op_id) = save() else {
            ctx.show_toast(Toast::error(tr!(switch_save_not_started())));
            return;
        };
        self.pending.set(switch);
        *self.pending_op.borrow_mut() = Some(op_id);
    }

    /// Do the switch. The point of no return: both use cases close the open Work
    /// first, so everything not already in the store (or on disk) is gone.
    fn perform(&self, ctx: &mut EventContext, switch: PendingSwitch) {
        match switch {
            PendingSwitch::None => {}
            PendingSwitch::NewWork => {
                let form = self.new_work_form_hook.borrow().clone();
                form(ctx);
            }
            PendingSwitch::OpenWork(path) => {
                if let Err(e) = work_management_commands::load_work(
                    &self.app_ctx,
                    &LoadWorkDto {
                        file_name: path.clone(),
                    },
                ) {
                    ctx.show_toast(Toast::error(tr!(could_not_open_work(
                        error = e.to_string()
                    ))));
                }
            }
        }
    }

    /// A long operation completed: if it is **the save this switch is waiting on**,
    /// perform the switch.
    ///
    /// Matched by op id — deliberately *not* driven off `WorkManagementEvent::
    /// SaveWork`, which carries no operation id. Autosave can already have a
    /// `save_work` in flight when the guard kicks its own: that older op's
    /// completion would fire the switch early, and if its background gather had
    /// run *before* our flush, the sentence the user typed last would be in no
    /// file at all — the store is wiped by the switch a moment later. Waiting for
    /// our own op id is what makes "Save, then switch" mean it.
    pub fn on_long_op_completed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = event_id(event) else {
            return;
        };
        if !self.matches_pending_op(&op_id) {
            return; // someone else's op (an autosave, a backup, an import)
        }
        let switch = self.take_pending();
        if switch != PendingSwitch::None {
            self.perform(ctx, switch);
        }
    }

    /// Is `op_id` the save the parked switch is waiting on?
    fn matches_pending_op(&self, op_id: &str) -> bool {
        self.pending_op.borrow().as_deref() == Some(op_id)
    }

    /// A long operation failed. If it was *our* save, the project is still dirty
    /// and still open — so drop the parked switch and say why, instead of leaving
    /// the user with a New/Open that silently never happens. Nothing is lost: the
    /// edits are exactly where they were.
    pub fn on_save_failed(&self, ctx: &mut EventContext, event: &Event) {
        let Some(op_id) = event_id(event) else {
            return;
        };
        if !self.matches_pending_op(&op_id) {
            return; // someone else's op (a backup, an import, a save-as)
        }
        self.take_pending();
        let error = parse_payload(event)
            .and_then(|p| p.get("error").and_then(|e| e.as_str()).map(str::to_string))
            .unwrap_or_default();
        ctx.show_toast(Toast::error(tr!(switch_save_failed(error = error))));
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
        *self.pending_op.borrow_mut() = None;
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
                    switch_decision(false, backup, autosave),
                    SwitchDecision::Proceed,
                    "nothing to protect (backup={backup}, autosave={autosave})"
                );
            }
        }
    }

    #[test]
    fn a_dirty_project_prompts_when_autosave_is_off() {
        assert_eq!(
            switch_decision(true, false, false),
            SwitchDecision::PromptSaveDiscardCancel
        );
    }

    #[test]
    fn a_dirty_project_saves_first_when_autosave_is_on() {
        assert_eq!(
            switch_decision(true, false, true),
            SwitchDecision::SaveThenSwitch
        );
    }

    #[test]
    fn backup_mode_can_only_discard_because_save_is_off_there() {
        // Autosave must not silently "save" a backup window — `save_to_disk` is
        // inert there, so the switch would proceed having written nothing.
        for autosave in [false, true] {
            assert_eq!(
                switch_decision(true, true, autosave),
                SwitchDecision::PromptDiscardOnly,
                "autosave={autosave}"
            );
        }
    }

    // ── the deferral (needs no `EventContext`) ───────────────────────────────

    /// `defer` needs an `EventContext` only for its "the save never started" toast,
    /// and this crate has no `EventContext` harness (see `backup_scheduler.rs`).
    /// This is the ctx-free core it is built on: kick the save, park the switch
    /// against the returned op id.
    fn defer_headless(vm: &ProjectSwitchViewModel, switch: PendingSwitch) {
        let save = vm.save_hook.borrow().clone();
        if let Some(op_id) = save() {
            vm.pending.set(switch);
            *vm.pending_op.borrow_mut() = Some(op_id);
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
                Some("op-1".to_string())
            }));
        }
        defer_headless(&vm, PendingSwitch::OpenWork("/tmp/other.skrib".into()));
        assert_eq!(saves.get(), 1, "the deferral must kick the disk write");
        assert_eq!(
            vm.pending.get(),
            PendingSwitch::OpenWork("/tmp/other.skrib".into()),
            "the switch must be parked, not performed — the write is still in flight"
        );
        assert_eq!(vm.pending_op.borrow().as_deref(), Some("op-1"));
    }

    #[test]
    fn a_switch_waits_for_its_own_save_not_just_any_completion() {
        // The race this closes: with autosave on, a `save_work` can already be in
        // flight when the guard kicks its own. If the switch fired on *that* op's
        // completion, the store would be wiped while our save was still queued —
        // and if the older op's gather ran before our flush, the last sentence
        // typed would be in no file at all. So a foreign op id must not release it.
        let vm = test_vm(true, false, true);
        vm.set_save_hook(Rc::new(|| Some("ours".to_string())));
        defer_headless(&vm, PendingSwitch::NewWork);

        assert!(
            !vm.matches_pending_op("someone-elses"),
            "an unrelated long op must not release the parked switch"
        );
        assert_eq!(
            vm.pending.get(),
            PendingSwitch::NewWork,
            "the switch is still parked, waiting for its own save"
        );

        assert!(
            vm.matches_pending_op("ours"),
            "our own save must release it"
        );
        assert_eq!(vm.take_pending(), PendingSwitch::NewWork);
        assert_eq!(vm.pending.get(), PendingSwitch::None);
        assert!(vm.pending_op.borrow().is_none());
    }

    #[test]
    fn a_save_that_never_started_parks_nothing() {
        // A switch waiting on an operation that will never complete is a command
        // that silently never happens. Better to report it and stay put.
        let vm = test_vm(true, false, true);
        vm.set_save_hook(Rc::new(|| None)); // the command could not be issued
        defer_headless(&vm, PendingSwitch::NewWork);
        assert_eq!(vm.pending.get(), PendingSwitch::None);
        assert!(vm.pending_op.borrow().is_none());
    }

    #[test]
    fn cancel_abandons_a_parked_switch() {
        // The close flow wins the race: the project is leaving the window, so the
        // parked switch must not fire behind it.
        let vm = test_vm(true, false, true);
        vm.set_save_hook(Rc::new(|| Some("op-1".to_string())));
        defer_headless(&vm, PendingSwitch::NewWork);
        vm.cancel();
        assert_eq!(vm.pending.get(), PendingSwitch::None);
        assert!(vm.pending_op.borrow().is_none());
    }

    #[test]
    fn hooks_default_to_harmless_no_ops_and_are_visible_on_earlier_clones() {
        // Un-hooked (the headless shape): deferring must not panic, and parks
        // nothing to wait on.
        let vm = test_vm(true, false, true);
        defer_headless(&vm, PendingSwitch::NewWork);
        assert!(vm.pending_op.borrow().is_none());

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
        defer_headless(&earlier_clone, PendingSwitch::NewWork);
        assert_eq!(saves.get(), 1, "the earlier clone must see the new hook");
    }
}
