// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ProjectLifecycleViewModel` — what happens when a project becomes live, or stops being.
//!
//! Three backend events bracket a project's life in this window: `LoadWork`, `NewWork`,
//! `CloseWork`. Each has to walk the same set of handles back to a coherent state — seed
//! the id-only global state, re-point the singles, rebuild the outline and the trash panel,
//! drop stale editor tabs, move the open-registry claim, re-point the spell-checker. Load
//! and New differ only in a tail (a new project is not on disk yet, so it is born dirty and
//! immediately written); Close is the same walk in reverse.
//!
//! That sequence used to live as three ~60-line closures inside `App::build`, each capturing
//! ten to thirteen clones, with the Load and New bodies kept in step by hand. Nothing tested
//! any of it, and the two copies had every opportunity to drift — a step added to one and
//! forgotten in the other is invisible until a user hits it.
//!
//! ## Ordering that is load-bearing
//!
//! The sequence is not arbitrary; several steps are ordered against each other:
//!
//! * `ids.seed` → `single_*.set_id` — the singles read the ids this seeds.
//! * `outline.set_binder_filter(None)` + `clear_search()` **before** `reload()` — a stale
//!   filter or query carried over from the previous project would reload into an empty tree.
//! * `open_registry` claim **while the singles still point at the project** — Close releases
//!   before unpointing, because the path is unreachable afterwards. Load/New use
//!   `replace_claim`, which drops this window's previous claim first: Load supersedes an
//!   earlier New/Load/Restore with no `CloseWork` in between.
//! * `refresh_spellcheck` **after** the tabs are closed — documents re-open afterwards and
//!   attach with the right language, so the `attach_all` inside it is a no-op at this point.
//! * `mark_clean` then the `dirty_seq` bump, in that order, on New only — "everything the
//!   previous project had is settled", then one step ahead so the brand-new project reads as
//!   unsaved until its create-and-save actually lands.
//!
//! Backup mode is deliberately **not** cleared by [`on_load`](ProjectLifecycleViewModel::on_load):
//! whether the opened file is a backup is sniffed by a second `LoadWork` subscriber in
//! `App::build`, which owns `backup_mode`/`backup_context` and the workspace-layout restore
//! for that path. Clearing it here would race that sniff. New and Close *do* clear it — a
//! brand-new project is never a backup, and a closed project is not anything.
//!
//! ## Why this may hold peer view-models
//!
//! The house rule is that peer view-models do not import each other and `App` mediates. This
//! one holds `OutlineViewModel`, `EditorsViewModel`, `TrashViewModel` and
//! `WorkspaceLayoutViewModel` — but it is not their peer, it is a layer above them: it only
//! ever calls *down*, nothing calls back into it, and no other view-model holds it. The
//! dependency graph stays the DAG the rule exists to protect (the rule's stated purpose is
//! "one direction per edge — avoid `Rc` cycles + re-entrant notifies"). `App` still mediates
//! every *peer-to-peer* edge; what moved here is the one many-to-one fan-out that `App` was
//! open-coding three times.

use std::collections::HashSet;
use std::rc::Rc;

use bastyde::prelude::Signal;

use frontend::AppContext;
use frontend::commands::dict_word_commands;

use crate::app_ids::AppIds;
use crate::backup::BackupContext;
use crate::models::OpenDocsStore;
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::spellcheck::SpellcheckService;

use super::{
    EditorsViewModel, OutlineViewModel, SaveStateViewModel, TrashViewModel,
    WorkspaceLayoutViewModel,
};

/// Reload the open Work's personal words (`DictWord`) into the checker's personal set.
///
/// The narrow half of the project-switch refresh: no language re-point (that only changes on
/// a project switch), no re-attach — the caller re-attaches, so a project switch attaches
/// once rather than twice. Also called directly from `App`'s `DictWord` event wiring, where
/// a word was added or removed but the project did not change.
pub(crate) fn reload_personal_words(app_ctx: &AppContext, spell: &SpellcheckService) {
    let personal: HashSet<String> = dict_word_commands::get_all_dict_word(app_ctx)
        .unwrap_or_default()
        .into_iter()
        .map(|w| w.word)
        .collect();
    spell.set_personal(personal);
}

struct Inner {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    outline: OutlineViewModel,
    editors: EditorsViewModel,
    trash: TrashViewModel,
    single_work: SingleWork,
    single_work_info: SingleWorkInfo,
    docs: OpenDocsStore,
    spellcheck: SpellcheckService,
    /// Work-scoped, shared with every other window onto this project — not owned
    /// here. See [`SaveStateViewModel`]'s module docs.
    save_state: SaveStateViewModel,
    backup_mode: Signal<bool>,
    backup_context: Signal<Option<BackupContext>>,
    /// Absent in a window with no layout service (a headless or launcher build).
    workspace_layout: Option<WorkspaceLayoutViewModel>,
}

/// Single-instance live state: one per project window, created once in `App::build` and
/// driven by the three `WorkManagement` events.
#[derive(Clone)]
pub struct ProjectLifecycleViewModel {
    inner: Rc<Inner>,
}

impl ProjectLifecycleViewModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        outline: OutlineViewModel,
        editors: EditorsViewModel,
        trash: TrashViewModel,
        single_work: SingleWork,
        single_work_info: SingleWorkInfo,
        docs: OpenDocsStore,
        spellcheck: SpellcheckService,
        save_state: SaveStateViewModel,
        backup_mode: Signal<bool>,
        backup_context: Signal<Option<BackupContext>>,
        workspace_layout: Option<WorkspaceLayoutViewModel>,
    ) -> Self {
        Self {
            inner: Rc::new(Inner {
                app_ctx,
                ids,
                outline,
                editors,
                trash,
                single_work,
                single_work_info,
                docs,
                spellcheck,
                save_state,
                backup_mode,
                backup_context,
                workspace_layout,
            }),
        }
    }

    /// A project became live: seed the ids, open its undo stack, re-point the singles,
    /// rebuild the outline + trash, drop stale tabs, and settle the dirty tracking.
    ///
    /// Stops short of the registry claim and the spell-check re-point ([`Self::claim`]) so
    /// that New can slot its `dirty_seq` bump between the two, exactly where the inline
    /// version had it.
    fn seed(&self) {
        let i = &self.inner;
        i.ids.seed(&i.app_ctx);
        i.ids.open_stack(&i.app_ctx);
        // Start the freshly-live Work unfiltered: a stale binder filter or query from the
        // previous Work would reload into an empty tree.
        i.outline.set_binder_filter(None);
        i.outline.clear_search();
        i.outline.reload();
        // Source this project's trash (the model was last sourced when no work was open,
        // or held the previous project's rows after an in-place switch).
        i.trash.reload();
        i.editors.close_all();
        i.single_work.set_id(i.ids.work_id.get());
        i.single_work_info.set_id(i.ids.work_info_id.get());
        // Nothing is pending against a project that just became live (`unsaved` is derived
        // from this). New bumps `dirty_seq` afterwards to put itself one step ahead.
        i.editors.mark_clean();
    }

    /// Claim the project in the open registry and re-point the spell-checker — the tail of
    /// becoming live, shared by Load and New.
    fn claim(&self) {
        let i = &self.inner;
        // Advertise it as open so other instances' switchers list it (and can raise this
        // window). `replace_claim` drops any claim this window already held: Load supersedes
        // New/Load/Restore with no `CloseWork` in between.
        if let Some(path) = i.single_work_info.file_name().get() {
            crate::shell::open_registry::replace_claim(&path, &i.single_work.title().get());
        }
        self.refresh_spellcheck();
    }

    /// Refresh the checker for the now-live project: reload its personal words, point the
    /// open-docs store at its default language, and re-attach every open document.
    fn refresh_spellcheck(&self) {
        let i = &self.inner;
        reload_personal_words(&i.app_ctx, &i.spellcheck);
        i.docs
            .set_project_language(i.ids.work_id.get(), i.single_work.dict_language().get());
        i.docs.attach_all();
    }

    /// `LoadWork`: adopt the project and stop.
    ///
    /// Backup mode and the workspace-layout restore are **not** handled here — the second
    /// `LoadWork` subscriber in `App::build` sniffs whether the opened file is a backup and
    /// owns both, because the restore needs that answer (a backup gets a clean default desk,
    /// not the source project's).
    pub fn on_load(&self) {
        self.seed();
        self.claim();
    }

    /// `NewWork`: adopt the project, then write it to disk.
    ///
    /// A brand-new project is not on disk yet, so it is born unsaved and immediately saved.
    /// The write is a long op; the SaveWork-completion handler clears `unsaved` only once it
    /// lands, so an exit or close during the in-flight write is caught by the guards rather
    /// than dropping the file.
    pub fn on_new(&self) {
        let i = &self.inner;
        self.seed();
        // One step ahead of `mark_clean`: reads as unsaved until the create-and-save lands.
        // Sits between `seed` and `claim` because that is where the inline version had it.
        i.save_state.bump_dirty();
        self.claim();
        // A brand-new project is never a backup.
        i.backup_mode.set(false);
        i.backup_context.set(None);
        i.editors.save_to_disk();
        // A fresh project has a fresh `unique_id` and so no saved layout: reset the docks to
        // the default (bottom hidden), dropping any arrangement inherited from an in-place
        // switch, over an empty desk. Never a backup.
        if let Some(layout) = &i.workspace_layout {
            layout.restore(false);
        }
    }

    /// `CloseWork`: release the project and empty everything that pointed at it.
    pub fn on_close(&self) {
        let i = &self.inner;
        // Release exactly the project being closed, before the singles are unpointed and its
        // path becomes unreachable. Not `release_all()`: the registry is keyed on
        // `(pid, path)`, so a window that one day holds several projects must not drop the
        // others' claims when one closes.
        if let Some(path) = i.single_work_info.file_name().get() {
            crate::shell::open_registry::release(&path);
        }
        // Drop this project's dictionaries, mutes and personal words: the next project
        // reloads lazily and starts unmuted.
        i.spellcheck.clear();
        i.ids.clear();
        i.outline.set_binder_filter(None);
        i.outline.clear_search();
        i.outline.reload();
        i.trash.reload();
        i.editors.close_all();
        i.single_work.set_id(None);
        i.single_work_info.set_id(None);
        // No project open — nothing can be pending against it.
        i.editors.mark_clean();
        i.backup_mode.set(false);
        i.backup_context.set(None);
    }
}

impl std::fmt::Debug for ProjectLifecycleViewModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectLifecycleViewModel").finish()
    }
}
