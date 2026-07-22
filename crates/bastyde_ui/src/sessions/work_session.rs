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
//! Phase 1 (this struct's introduction) is a pure refactor: `main.rs` still
//! constructs exactly **one** `WorkSession` at startup, and the app still opens
//! one Work at a time — the "close every other open Work" backend sweep is
//! untouched (see `frontend::tests::multi_work_scoping_test`'s tripwire tests).
//! The point is only to give Phase 2 a seam to multiply: once
//! `ProjectWindowFactory::window_config` can resolve a `work_id` up front, it
//! asks [`super::WorkRegistry::session_for`] for a `WorkSession` instead of
//! reaching into a dozen independent `app_state` singletons, and opening a
//! second Work becomes "construct a second `WorkSession`", not "audit every
//! consumer".
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
//! **What did *not* move, on purpose.** `AppIds.root_id` lifted OUT to
//! [`super::WorkRegistry`] (Tier 1 — one `Root` per process, not per Work; see
//! `app_ids.rs`'s module doc). `BackupSchedulerViewModel` moves here whole,
//! not split into its arbitration half (`pending`/`completed_epoch`) and its
//! `flush_hook` — that split, and fixing `flush_hook` from a single overwritten
//! slot into a per-window collection, is explicitly Phase 2 work (design doc
//! §3/§8): today there is only one window per Work, so the existing single
//! slot is not yet wrong. Likewise `WorkspaceLayoutViewModel`/
//! `TreeExpansionViewModel` move here as whole instances; splitting their
//! Tier-3 `DockingModel`/`EditorsViewModel`-reference half out is future work
//! the design doc flags but does not schedule for Phase 1.

use std::rc::Rc;

use bastyde::prelude::Signal;
use bastyde::widgets::DockingModel;

use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::models::{
    DictWordListModel, OpenDocsStore, TreeExpansionService, WorkTagsListModel,
    WorkspaceLayoutService,
};
use crate::singles::{SingleDictWord, SingleWork, SingleWorkInfo};
use crate::spellcheck::SpellcheckService;
use crate::view_models::{
    BackupSchedulerViewModel, BackupSettingsViewModel, MentionIndex, ProgressRecorder,
    SaveStateViewModel, TagsViewModel, TreeExpansionViewModel, UserDictionaryViewModel,
    WorkspaceLayoutViewModel,
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
    pub user_dictionary: UserDictionaryViewModel,
    pub mention_index: MentionIndex,
    pub progress_recorder: ProgressRecorder,
    /// Whole instance, not yet split — see the module doc's "what did not
    /// move" section.
    pub backup_scheduler: BackupSchedulerViewModel,
    pub workspace_layout: WorkspaceLayoutViewModel,
    pub tree_expansion: TreeExpansionViewModel,
    pub open_docs: OpenDocsStore,
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
        backup_mode: Signal<bool>,
        backup_settings: BackupSettingsViewModel,
        workspace_layout_service: WorkspaceLayoutService,
        tree_expansion_service: TreeExpansionService,
    ) -> Self {
        let single_work = SingleWork::new(app_ctx.clone());
        let single_work_info = SingleWorkInfo::new(app_ctx.clone());

        let open_docs = OpenDocsStore::new(app_ctx.clone());
        open_docs.set_spellcheck(spellcheck);

        let save_state = SaveStateViewModel::new(app_ctx.clone(), ids.clone());

        let work_tags = WorkTagsListModel::new(app_ctx.clone());
        let tags = TagsViewModel::new(work_tags, ids.clone());

        let dict_words = DictWordListModel::new(app_ctx.clone());
        let single_dict_word = SingleDictWord::new(app_ctx.clone());
        let user_dictionary = UserDictionaryViewModel::new(dict_words, single_dict_word, ids.clone());

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
        );

        let backup_scheduler = BackupSchedulerViewModel::new(
            app_ctx,
            backup_settings,
            single_work.clone(),
            single_work_info.clone(),
            backup_mode,
        );

        Self {
            ids,
            single_work,
            single_work_info,
            save_state,
            tags,
            user_dictionary,
            mention_index,
            progress_recorder,
            backup_scheduler,
            workspace_layout,
            tree_expansion,
            open_docs,
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
            Signal::new(false),
            BackupSettingsViewModel::new(crate::models::BackupSettingsService::in_memory_default()),
            WorkspaceLayoutService::in_memory_default(),
            TreeExpansionService::in_memory_default(),
        )
    }
}
