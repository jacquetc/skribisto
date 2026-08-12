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
//! Extracted from `App::build` (previously three near-duplicate closures kept in step by
//! hand) so Load/New/Close cannot silently drift apart and the sequence is unit-testable.
//!
//! ## Ordering that is load-bearing
//!
//! The sequence is not arbitrary; several steps are ordered against each other:
//!
//! * `ids.seed` → `single_*.set_id` — the singles read the ids this seeds.
//! * `outline.set_binder_filter(None)` + `clear_search()` **before** `reload()` — a stale
//!   filter or query carried over from the previous project would reload into an empty tree.
//! * `open_registry` claim **while the singles still point at the project** — Close releases
//!   before unpointing, because the path is unreachable afterwards. Load/New release THIS
//!   window's own previous claim (captured before `seed()` overwrites it), then claim the new
//!   path — never `replace_claim`/`release_all()`, which would drop every claim the whole
//!   *process* holds, including a sibling window's untouched, still-open Work (see `claim`'s
//!   own doc).
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

use teksilo::prelude::Signal;

use frontend::AppContext;
use frontend::commands::{dict_word_commands, work_commands};
use frontend::common::direct_access::work::WorkRelationshipField;

use crate::app_ids::AppIds;
use crate::backup::BackupContext;
use crate::binder::OutlineViewModel;
use crate::models::OpenDocsStore;
use crate::singles::{SingleWork, SingleWorkInfo};
use crate::spellcheck::SpellcheckService;
use crate::trash::TrashViewModel;

use super::{EditorsViewModel, SaveStateViewModel, WorkspaceLayoutViewModel};

/// Reload `work_id`'s Work's personal words (`DictWord`, via `Work.dict_words`) into the
/// checker's personal set for that Work. **Not** `dict_word_commands::get_all_dict_word`,
/// which returns every `DictWord` in the whole shared store: with a second Work
/// simultaneously open, that would merge both Works' personal dictionaries into one set,
/// and a personal word from Work B would silently stop flagging a genuine typo in Work A (and
/// vice-versa) — see `models::dict_word_list_model`'s identical fix/rationale for the
/// Settings-pane list this mirrors. A `None` `work_id` (no project open) reloads nothing.
///
/// The narrow half of the project-switch refresh: no language re-point (that only changes on
/// a project switch), no re-attach — the caller re-attaches, so a project switch attaches
/// once rather than twice. Also called directly from `App`'s `DictWord` event wiring, where
/// a word was added or removed but the project did not change.
pub(crate) fn reload_personal_words(
    app_ctx: &AppContext,
    spell: &SpellcheckService,
    work_id: Option<u64>,
) {
    let Some(work_id) = work_id else { return };
    let word_ids =
        work_commands::get_work_relationship(app_ctx, &work_id, &WorkRelationshipField::DictWords)
            .unwrap_or_default();
    let personal: HashSet<String> = dict_word_commands::get_dict_word_multi(app_ctx, &word_ids)
        .unwrap_or_default()
        .into_iter()
        .flatten()
        .map(|w| w.word)
        .collect();
    spell.set_personal(work_id, personal);
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
    ///
    /// `work_id` comes from the triggering `LoadWork`/`NewWork` event itself (see
    /// `AppIds::seed`'s docs) — never re-derived by asking the store "which Work is
    /// open", which stopped being answerable the moment a second Work could be open
    /// at the same time.
    fn seed(&self, work_id: u64) {
        let i = &self.inner;
        i.ids.seed(&i.app_ctx, work_id);
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
    ///
    /// **Releases only THIS window's own previous claim, never
    /// `replace_claim`/`release_all()`.** The latter drops every claim the whole
    /// *process* holds, so with two Works open in one process it would silently
    /// wipe a sibling window's still-open, untouched Work out of the open
    /// registry — vanished from every peer process's ProjectSwitcher and from
    /// IPC-raise reachability, with no error and no visible cause.
    /// `previous_path` is `self`'s own path from *before* [`Self::seed`]
    /// overwrote `single_work_info` (captured by the caller, `on_load`/`on_new`,
    /// which is why this takes it as a parameter rather than reading it here)
    /// — `None` for this window's very first Load/New, when it held no claim
    /// yet.
    fn claim(&self, previous_path: Option<&str>) {
        let i = &self.inner;
        if let Some(prev) = previous_path {
            crate::shell::open_registry::release(prev);
        }
        // Advertise it as open so other instances' switchers list it (and can raise this
        // window).
        if let Some(path) = i.single_work_info.file_name().get() {
            crate::shell::open_registry::claim(&path, &i.single_work.title().get());
        }
        self.refresh_spellcheck();
    }

    /// Refresh the checker for the now-live project: reload its personal words, point the
    /// open-docs store at its default language, and re-attach every open document.
    fn refresh_spellcheck(&self) {
        let i = &self.inner;
        reload_personal_words(&i.app_ctx, &i.spellcheck, i.ids.work_id.get());
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
    pub fn on_load(&self, work_id: u64) {
        // Captured before `seed()` overwrites `single_work_info` — see `claim`'s doc.
        let previous_path = self.inner.single_work_info.file_name().get();
        self.seed(work_id);
        self.claim(previous_path.as_deref());
    }

    /// `NewWork`: adopt the project, then write it to disk.
    ///
    /// A brand-new project is not on disk yet, so it is born unsaved and immediately saved.
    /// The write is a long op; the SaveWork-completion handler clears `unsaved` only once it
    /// lands, so an exit or close during the in-flight write is caught by the guards rather
    /// than dropping the file.
    pub fn on_new(&self, work_id: u64) {
        // Captured before `seed()` overwrites `single_work_info` — see `claim`'s doc.
        let previous_path = self.inner.single_work_info.file_name().get();
        let i = &self.inner;
        self.seed(work_id);
        // One step ahead of `mark_clean`: reads as unsaved until the create-and-save lands.
        // Sits between `seed` and `claim` because that is where the inline version had it.
        i.save_state.bump_dirty();
        self.claim(previous_path.as_deref());
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
        // Drop *this* project's own dictionaries, mutes and personal words: the next
        // project loaded in this window reloads lazily and starts unmuted.
        //
        // `SpellcheckService` is a single Tier-1 engine shared by every open Work (one
        // `spellbook::Dictionary` pool, correctly process-wide) — but its session-mute
        // set and personal words are partitioned by `work_id` (see the service's own
        // module doc), so `clear(work_id)` drops exactly this Work's own map entry and
        // never a different, still-open Work's. Read `work_id` before `ids.clear()`
        // zeroes it.
        if let Some(work_id) = i.ids.work_id.get() {
            i.spellcheck.clear(work_id);
        }
        // Deleting this Work's own undo/redo stack (`create_new_stack` minted it on
        // `on_load`/`on_new` above) does **not** happen here. `CloseWork` fires once
        // per window that had this Work open (`AttachExisting`: several windows
        // can share one Work), so deleting the stack unconditionally in every
        // one of those windows' own `on_close` would delete it out from under a
        // sibling window that still needs it. The real "is anyone still using this
        // stack" answer is Skribisto's own window→Work bookkeeping, not this
        // per-window lifecycle step — see `sessions::WorkRegistry::remove_window`
        // (driven by teksilo's `on_removed` window-teardown hook, for a real
        // close) and `register_window`'s own replace path (for an in-place Work
        // switch, which fires no `CloseWork` at all), either of which deletes
        // the stack exactly once, only on the last window standing.
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
