// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `WorkSession` — every piece of **Tier 2** ("per open `Work`") state, bundled
//! into one cloneable handle.
//!
//! Per the migration design doc (§2/§8), Skribisto is moving from one process
//! per open project to one process holding N simultaneously-open `Work`s, each
//! in M windows. Most of what a window needs to show a project is either
//! genuinely process-wide (Tier 1 — `AppContext`/`DbContext`/`EventHub`/…, see
//! `main.rs`) or genuinely per-*window* (Tier 3 — `EditorsViewModel`,
//! `scene_focused`, a dock's own visual arrangement). What's left is Tier 2:
//! state that is shared by every window looking at the *same* Work, but must
//! be a fresh, independent instance for a *different* Work.
//!
//! Multiple `WorkSession`s coexist today: `ProjectWindowFactory::window_config`
//! mints a fresh one for each newly-opened Work, and `WorkRegistry::attach`
//! shares an existing one with a second window on the same Work (Work ▸ New
//! Window). Loading a Work no longer closes any other open Work first — that
//! backend sweep was removed as part of the same migration (see
//! `work_management::load_work_uc`).
//!
//! **What moved here, and why `main.rs`'s existing `.app_state(...)`
//! registrations for these same instances are left in place.** Every field
//! below is still registered as `app_state` too (in `main.rs`), unchanged —
//! this bundle does not replace those registrations, it replaces the dozen
//! independent local variables `main.rs` used to construct and thread through
//! by hand. Widgets that already reach a field via `ctx.app_state::<T>()`
//! (there are many: `docks/inspector.rs`, `tags/tag_chip.rs`,
//! `spellcheck/language_pill_field.rs`, `settings.rs`, …) keep working exactly
//! as today, reaching the *same* instance either way — per the design doc's
//! "everything below `App::build` needs no change in shape". `App` itself is
//! the one place that now receives the bundle directly as a constructor
//! parameter (see `app.rs`'s `session` field) instead of re-fetching each
//! piece from `app_state` — the concrete slice of the "resolution mechanism"
//! this phase implements.
//!
//! **What did *not* move.** `AppIds.root_id` lifted OUT to
//! [`super::WorkRegistry`] (Tier 1 — one `Root` per process, not per Work; see
//! `app_ids.rs`'s module doc). `BackupSchedulerViewModel` lives here as one
//! whole Tier-2 instance, shared by every window on a Work; its `flush_hook`
//! is itself a per-window keyed collection internally
//! (`BackupSchedulerViewModel::register_flush_hook`/`unregister_flush_hook`),
//! not a single overwritten slot. `WorkspaceLayoutViewModel`/
//! `TreeExpansionViewModel` still move here as whole instances with their
//! Tier-3 `DockingModel` reference bundled in — a second window on the same
//! Work shares the first window's desk arrangement rather than getting its own.
//!
//! **Phase 3 correction — `backup_mode`/`backup_context` are minted here, not
//! passed in.** Through Phase 2 these were a *caller-supplied* `Signal<bool>`/
//! `Signal<Option<BackupContext>>` pair, constructed once in `main.rs` and
//! threaded unchanged into every window `ProjectWindowFactory` built — a
//! Tier-1 pair doing Tier-2 duty. With two Works open, loading/creating/
//! closing a project in one window (each of which writes
//! `backup_mode.set(..)`/`backup_context.set(..)` — see
//! `ProjectLifecycleViewModel::on_new`/`on_close`) silently flipped the
//! *other* window's backup-mode flag too: clearing its banner and
//! re-enabling Save on what was still, semantically, a read-only backup.
//! Whether a Work was opened from a backup is exactly as per-Work as
//! `save_state`/`tags`, so this struct now constructs a fresh pair per
//! `WorkSession` — `WorkSession::new` no longer takes `backup_mode` as a
//! parameter at all.

use std::rc::Rc;

use teksilo::prelude::Signal;
use teksilo::widgets::DockingModel;

use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::backup::BackupContext;
use crate::models::{
    DictWordListModel, OpenDocsStore, TextReplacementRuleListModel, TreeExpansionService,
    WorkNoteTemplatesListModel, WorkTagsListModel, WorkspaceLayoutService,
};
use crate::singles::{SingleDictWord, SingleSmartPunctuation, SingleWork, SingleWorkInfo};
use crate::spellcheck::SpellcheckService;
use crate::view_models::{
    BackupSchedulerViewModel, BackupSettingsViewModel, MentionIndex, NoteTemplatesViewModel,
    ProgressRecorder, SaveStateViewModel, TagsViewModel, TextReplacementRulesViewModel,
    TreeExpansionViewModel, UserDictionaryViewModel, WorkspaceLayoutViewModel,
};

/// Every Tier-2 ("per open Work") view-model/single/model, bundled. Cloneable —
/// every field is itself a cheap `Rc`-backed handle, so cloning a `WorkSession`
/// is cloning a dozen `Rc`s, not copying data.
#[derive(Clone)]
pub struct WorkSession {
    /// `work_id`/`work_info_id`/`stack_id` — the app's id-only Tier-2 state.
    /// `root_id` is deliberately not here; see the module doc.
    pub ids: AppIds,
    pub single_work: SingleWork,
    pub single_work_info: SingleWorkInfo,
    /// Work-scoped, not per-window save tracking (`dirty_seq`/`saved_seq`/
    /// `saving` + the `SaveQueue`) — the seed this struct grew from. See its
    /// own module doc for why a per-window copy is a bug the moment a second
    /// window on the same Work exists.
    pub save_state: SaveStateViewModel,
    pub tags: TagsViewModel,
    /// This Work's note templates — Settings ▸ Work ▸ Templates and the Document menu's
    /// insert submenu both read it, so both always agree about what exists.
    pub note_templates: NoteTemplatesViewModel,
    pub user_dictionary: UserDictionaryViewModel,
    /// The project's punctuation house style (`Work.smart_punctuation`), and
    /// the per-project custom replacement lexicon ("btw" → "by the way",
    /// `Work.custom_replacement_rules_enabled` + `Work.text_replacement_rules`).
    /// Landed on a parallel branch to the multi-Work migration and, through the
    /// merge that reconciled the two, had no matching `WorkSession` field yet —
    /// closed here, the same "fresh instance per Work" shape as `tags`/
    /// `user_dictionary` above: a second, simultaneously-open Work must never
    /// share this Work's punctuation row or lexicon.
    pub smart_punctuation: SingleSmartPunctuation,
    pub text_replacements: TextReplacementRulesViewModel,
    pub mention_index: MentionIndex,
    pub progress_recorder: ProgressRecorder,
    /// Whole instance, not yet split — see the module doc's "what did not
    /// move" section.
    pub backup_scheduler: BackupSchedulerViewModel,
    pub workspace_layout: WorkspaceLayoutViewModel,
    pub tree_expansion: TreeExpansionViewModel,
    pub open_docs: OpenDocsStore,
    /// Is this project playing the **Always forward** writing game right now?
    ///
    /// Tier 2 on purpose: two windows on one `Work` must agree about whether
    /// Backspace works in the same document, while a second, simultaneously-open
    /// project must be free to draft normally. Minted fresh here and never
    /// persisted or restored — a commitment device that outlives the sitting it
    /// was made in reads as a broken keyboard, exactly as `FocusViewModel`
    /// argues for distraction-free mode. Paired with the app-global "which
    /// surfaces" settings into a
    /// [`WritingGamesViewModel`](crate::view_models::WritingGamesViewModel)
    /// wherever both are in hand.
    pub always_forward: Signal<bool>,
    /// `true` while a *backup file* is open under this Work (Save + auto-backup
    /// off; the file is read-only, the content is still editable). Minted fresh
    /// here — see the module doc's "Phase 3 correction" section — never passed
    /// in, so a second, simultaneously-open Work always gets its own flag.
    pub backup_mode: Signal<bool>,
    /// The open backup's details (drives the permanent banner + restore), or
    /// `None` for a normal project. Fresh per `WorkSession`, same reasoning as
    /// `backup_mode`.
    pub backup_context: Signal<Option<BackupContext>>,
    /// `true` while this Work has edits not yet written to disk — derived
    /// (`dirty_seq > saved_seq`, both read off `save_state`) by an effect
    /// `App::build` installs against THIS field, not a signal it owns itself.
    ///
    /// **Scope E fix.** Through Phase 2 this was a *caller-supplied*
    /// `Signal<bool>`, constructed once in `main.rs` and threaded unchanged
    /// into every window `ProjectWindowFactory` built — the exact same
    /// Tier-1-doing-Tier-2-duty shape `backup_mode`/`backup_context` had
    /// before their own Phase-3 fix (see this struct's module doc). Every
    /// window's own recompute effect wrote into the SAME shared signal
    /// (`app.rs`'s own comment on that effect used to read "every window
    /// derives the identical `unsaved` from the identical pair" — true only
    /// when there is truly one Work; false the moment a second, independent
    /// Work opens), so one Work's edits landing/saving could silently flip
    /// another Work's Save affordance and close-guard decision. A second
    /// window on the SAME Work (Phase 3's `AttachExisting`) is meant to
    /// share one `unsaved` — that is genuinely Tier 2 — so this lives here,
    /// not per-window like `pending_exit` (see that field's own doc for why
    /// IT moved the other way).
    pub unsaved: Signal<bool>,
}

impl WorkSession {
    /// Build the one Tier-2 bundle. Everything internal that a Tier-2 view-model
    /// needs but that nothing outside this struct touches directly (the tag
    /// palette's backing list model, the personal-dictionary list model + its
    /// single) is constructed here and folded straight into its owning
    /// view-model, rather than exposed as its own field — see `models.rs`'s
    /// `WorkTagsListModel`/`DictWordListModel` docs for why they are always
    /// reached through `TagsViewModel`/`UserDictionaryViewModel` instead.
    ///
    /// Takes `ids: AppIds` rather than minting its own: `OutlineViewModel` (not
    /// itself Tier 2 — see the module doc) needs the identical `AppIds` clone,
    /// and its `DockingModel` is in turn what `workspace_layout` mounts onto —
    /// so `main.rs` builds `ids` and `outline` first, then passes both in here,
    /// the same way every other `ids`-taking view-model already does.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        spellcheck: SpellcheckService,
        docking: DockingModel,
        backup_settings: BackupSettingsViewModel,
        workspace_layout_service: WorkspaceLayoutService,
        tree_expansion_service: TreeExpansionService,
    ) -> Self {
        // Fresh per Work — see the module doc's "Phase 3 correction" section.
        // Never a caller-supplied parameter: a second, simultaneously-open Work
        // must never share this Work's backup-mode flag/details.
        let backup_mode = Signal::new(false);
        let backup_context: Signal<Option<BackupContext>> = Signal::new(None);
        // Fresh per Work — see `Self::unsaved`'s own doc (Scope E fix).
        let unsaved = Signal::new(false);

        // Fresh per Work — never shared between two simultaneously-open
        // projects, and never seeded from anything persisted.
        let always_forward = Signal::new(false);

        let single_work = SingleWork::new(app_ctx.clone());
        let single_work_info = SingleWorkInfo::new(app_ctx.clone());

        let open_docs = OpenDocsStore::new(app_ctx.clone());
        open_docs.set_spellcheck(spellcheck);

        let save_state = SaveStateViewModel::new(app_ctx.clone(), ids.clone());

        let work_tags = WorkTagsListModel::new(app_ctx.clone(), ids.clone());
        let tags = TagsViewModel::new(work_tags, ids.clone());
        let note_templates = NoteTemplatesViewModel::new(
            WorkNoteTemplatesListModel::new(app_ctx.clone(), ids.clone()),
            ids.clone(),
        );

        let dict_words = DictWordListModel::new(app_ctx.clone(), ids.clone());
        let single_dict_word = SingleDictWord::new(app_ctx.clone());
        let user_dictionary =
            UserDictionaryViewModel::new(dict_words, single_dict_word, ids.clone());

        let smart_punctuation = SingleSmartPunctuation::new(app_ctx.clone());

        let replacement_rules = TextReplacementRuleListModel::new(app_ctx.clone(), ids.clone());
        let text_replacements =
            TextReplacementRulesViewModel::new(replacement_rules, single_work.clone(), ids.clone());
        // Hand it to the open-docs store, which owns the per-document attach
        // loop — exactly as the spell engine is handed over just above.
        open_docs.set_text_replacements(text_replacements.clone());

        let mention_index = MentionIndex::new(app_ctx.clone(), ids.clone());
        let progress_recorder = ProgressRecorder::new(app_ctx.clone(), ids.clone());

        let tree_expansion =
            TreeExpansionViewModel::new(app_ctx.clone(), ids.clone(), tree_expansion_service);
        let workspace_layout = WorkspaceLayoutViewModel::new(
            app_ctx.clone(),
            workspace_layout_service,
            docking,
            single_work.clone(),
            single_work_info.clone(),
            ids.clone(),
            backup_mode.clone(),
            tree_expansion.clone(),
        );

        let backup_scheduler = BackupSchedulerViewModel::new(
            app_ctx,
            ids.clone(),
            workspace_layout.clone(),
            backup_settings,
            single_work.clone(),
            single_work_info.clone(),
            backup_mode.clone(),
        );

        Self {
            ids,
            single_work,
            single_work_info,
            save_state,
            tags,
            note_templates,
            user_dictionary,
            smart_punctuation,
            text_replacements,
            mention_index,
            progress_recorder,
            backup_scheduler,
            workspace_layout,
            tree_expansion,
            open_docs,
            always_forward,
            backup_mode,
            backup_context,
            unsaved,
        }
    }

    /// A throwaway session for unit tests (e.g. `WorkRegistry`'s own tests) —
    /// a real, fully-wired instance over a fresh in-memory `AppContext`. Tests
    /// that need a specific `work_id` mutate `session.ids` directly afterward
    /// (it is the same shared `Signal` every field constructed here already
    /// holds a clone of, so the mutation is visible everywhere).
    #[cfg(test)]
    pub(crate) fn for_test() -> Self {
        let app_ctx = Rc::new(AppContext::new());
        Self::new(
            app_ctx,
            AppIds::new(),
            SpellcheckService::new(),
            DockingModel::new(),
            BackupSettingsViewModel::new(crate::models::BackupSettingsService::in_memory_default()),
            WorkspaceLayoutService::in_memory_default(),
            TreeExpansionService::in_memory_default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The regression this Phase-3 fix closes: two simultaneously-open Works
    /// must never share a `backup_mode`/`backup_context` identity. Before this
    /// fix, `WorkSession::new` took a caller-supplied `Signal<bool>` that every
    /// window shared — flipping one Work's flag silently flipped every other
    /// open Work's too.
    #[test]
    fn each_session_gets_its_own_backup_mode_signal() {
        let a = WorkSession::for_test();
        let b = WorkSession::for_test();

        a.backup_mode.set(true);
        assert!(
            a.backup_mode.get(),
            "Work A's own flag must reflect its own write"
        );
        assert!(
            !b.backup_mode.get(),
            "Work B's backup_mode must be untouched by Work A's write — a fresh Signal, not a shared one"
        );
    }

    #[test]
    fn each_session_gets_its_own_backup_context_signal() {
        let a = WorkSession::for_test();
        let b = WorkSession::for_test();

        let bc = BackupContext {
            path: "/tmp/a.skrib".to_string(),
            backup_of: Some("/tmp/original.skrib".to_string()),
            backup_created_at: None,
            authoritative: true,
        };
        a.backup_context.set(Some(bc.clone()));

        assert_eq!(a.backup_context.get(), Some(bc));
        assert_eq!(
            b.backup_context.get(),
            None,
            "Work B's backup_context must stay None — it never opened a backup"
        );
    }

    /// Scope E: two simultaneously-open Works must never share an `unsaved`
    /// identity either — the same class of bug `backup_mode`/`backup_context`
    /// had before their own fix (see `Self::unsaved`'s doc).
    #[test]
    fn each_session_gets_its_own_unsaved_signal() {
        let a = WorkSession::for_test();
        let b = WorkSession::for_test();

        a.unsaved.set(true);
        assert!(
            a.unsaved.get(),
            "Work A's own flag must reflect its own write"
        );
        assert!(
            !b.unsaved.get(),
            "Work B's unsaved must be untouched by Work A's write — a fresh Signal, not a shared one"
        );
    }

    /// The punctuation house style and the custom replacement lexicon landed
    /// on a parallel branch and, through the merge that reconciled it with the
    /// multi-Work migration, briefly had no `WorkSession` field at all —
    /// resolved instead via a process-wide `ctx.app_state`, the exact
    /// "first-window-wins" shape `backup_mode`/`backup_context`/`unsaved` had
    /// before their own fixes above. This pins the fix: two simultaneously-open
    /// Works must never share a `smart_punctuation` identity either.
    #[test]
    fn each_session_gets_its_own_smart_punctuation_handle() {
        let a = WorkSession::for_test();
        let b = WorkSession::for_test();

        // Read Work B's own starting value first — the mock fixture seeds a
        // non-default row (see `SingleSmartPunctuation`'s own mock doc), so this
        // must not assume any particular starting value, only that it is
        // independent of whatever Work A does next.
        let b_dashes_before = b.smart_punctuation.dashes().get();
        let a_dashes_before = a.smart_punctuation.dashes().get();

        a.smart_punctuation.set_dashes(!a_dashes_before);

        assert_eq!(
            a.smart_punctuation.dashes().get(),
            !a_dashes_before,
            "Work A's own flag must reflect its own write"
        );
        assert_eq!(
            b.smart_punctuation.dashes().get(),
            b_dashes_before,
            "Work B's punctuation handle must be untouched by Work A's write — a fresh instance, not a shared one"
        );
    }

    /// Same class of bug, for the custom replacement lexicon's master switch:
    /// `TextReplacementRulesViewModel::enabled_signal` proxies the session's
    /// own `SingleWork` — if two sessions' `text_replacements` ever aliased
    /// the same handle (or the same underlying `SingleWork`), flipping one
    /// Work's switch would flip the other's too.
    #[test]
    fn each_session_gets_its_own_text_replacements_handle() {
        let a = WorkSession::for_test();
        let b = WorkSession::for_test();

        a.text_replacements.set_enabled(true);
        assert!(
            a.text_replacements.enabled_signal().get(),
            "Work A's own switch must reflect its own write"
        );
        assert!(
            !b.text_replacements.enabled_signal().get(),
            "Work B's switch must be untouched by Work A's write — a fresh handle, not a shared one"
        );
    }

    /// Every clone of the *same* session must still share one identity (the
    /// whole point of bundling these as `Signal`s on a cloneable struct) — the
    /// fix is "one pair per Work", not "one pair per clone".
    #[test]
    fn clones_of_the_same_session_share_one_backup_mode_identity() {
        let a = WorkSession::for_test();
        let a_clone = a.clone();

        a_clone.backup_mode.set(true);

        assert!(
            a.backup_mode.get(),
            "a clone of the same session must share the same Signal, not a copy"
        );
    }
}
