// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `TreeExpansionViewModel` — remembering which rows a writer left expanded.
//!
//! A peer of [`WorkspaceLayoutViewModel`](super::WorkspaceLayoutViewModel), and shaped
//! the same way: it owns only the service handle plus the ids it needs to key by, and
//! `App` drives it at the doors. Neither imports the other.
//!
//! **Captured at the doors, never per toggle.** `SettingsFile::mutate` is a synchronous
//! locked read-modify-write, and chevron clicks are chatty — a writer opening their way
//! down a book would rewrite the whole file on every click. So the capture happens where
//! the project is being left (close / quit / project switch), in one batched write
//! covering every open container tab.
//!
//! The accepted cost is that an unclean exit forgets the session's chevrons. That matches
//! how the desk layout already behaves, and losing an expand state is not losing work.
//!
//! **Restored per tab, not per project.** Each Overview restores its own container's set
//! from `wire()`, right after its first row load — so there is no ordering constraint
//! against the workspace-layout restore and no load-time translation step. That is the
//! payoff of keying by durable uid: what is written is what is read.

use std::rc::Rc;

use uuid::Uuid;

use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::models::{TreeExpansionService, uid_is_usable};

#[derive(Clone)]
pub struct TreeExpansionViewModel {
    service: TreeExpansionService,
    app_ctx: Rc<AppContext>,
    ids: AppIds,
}

impl TreeExpansionViewModel {
    pub fn new(app_ctx: Rc<AppContext>, ids: AppIds, service: TreeExpansionService) -> Self {
        Self {
            service,
            app_ctx,
            ids,
        }
    }

    /// The open project's `Work.unique_id`, or `None` when there is nothing to key by.
    ///
    /// A brand-new unsaved project has no uid yet; keying by `""` would make every such
    /// project share one row, so both capture and restore go quiet instead.
    fn work_uid(&self) -> Option<String> {
        let work_id = self.ids.work_id.get()?;
        let work = frontend::commands::work_commands::get_work(&self.app_ctx, &work_id)
            .ok()
            .flatten()?;
        uid_is_usable(&work.unique_id).then_some(work.unique_id)
    }

    /// The remembered expand set for one container — what an Overview applies on wire.
    /// Empty when there is nothing remembered, which restores as "the default state".
    pub fn expanded_for(&self, container_uid: Uuid) -> Vec<Uuid> {
        match self.work_uid() {
            Some(uid) => self.service.expanded(&uid, container_uid),
            None => Vec::new(),
        }
    }

    /// The outline's remembered expanded rows, for it to apply on load.
    pub fn outline_expanded(&self) -> Vec<crate::models::BinderTreeKey> {
        match self.work_uid() {
            Some(uid) => self.service.outline(&uid),
            None => Vec::new(),
        }
    }

    /// Persist the outline's expanded rows.
    pub fn capture_outline(&self, expanded: &[crate::models::BinderTreeKey]) {
        let Some(work_uid) = self.work_uid() else {
            return;
        };
        let path = crate::current_project_path(&self.app_ctx, &self.ids).unwrap_or_default();
        if let Err(e) = self.service.set_outline(&work_uid, &path, expanded) {
            eprintln!("skribisto: could not persist outline expansion: {e}");
        }
    }

    /// Persist every open container's expand state in **one** write.
    ///
    /// `folders` is `(container uid, expanded uids)` — gathered by `App` from the open
    /// editor tabs, because the tabs are `EditorsViewModel`'s and this view-model does
    /// not import a peer to reach them.
    ///
    /// Silently does nothing without a usable project uid: there is no key to write
    /// under, and inventing one would collide across unrelated new projects.
    pub fn capture(&self, folders: &[(Uuid, Vec<Uuid>)]) {
        if folders.is_empty() {
            return;
        }
        let Some(work_uid) = self.work_uid() else {
            return;
        };
        let path = crate::current_project_path(&self.app_ctx, &self.ids).unwrap_or_default();
        if let Err(e) = self.service.set_folders(&work_uid, &path, folders) {
            // Losing a remembered chevron is not worth interrupting a close for, but it
            // should not vanish silently either.
            eprintln!("skribisto: could not persist tree expansion: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tempfile::tempdir;

    fn vm(dir: &std::path::Path) -> TreeExpansionViewModel {
        let service =
            TreeExpansionService::open_at(dir.join("tree_expansion.toml"), Duration::ZERO).unwrap();
        TreeExpansionViewModel::new(Rc::new(AppContext::new()), AppIds::new(), service)
    }

    /// With no project open there is no key, so both directions go quiet rather than
    /// writing under a blank uid that every unsaved project would share.
    #[test]
    fn without_a_project_capture_and_restore_are_inert() {
        let dir = tempdir().unwrap();
        let vm = vm(dir.path());
        let container = Uuid::from_u128(1);

        vm.capture(&[(container, vec![Uuid::from_u128(2)])]);
        assert!(vm.expanded_for(container).is_empty());
        assert!(
            !dir.path().join("tree_expansion.toml").exists()
                || std::fs::read_to_string(dir.path().join("tree_expansion.toml"))
                    .unwrap()
                    .find("00000000-0000-0000-0000-000000000002")
                    .is_none(),
            "nothing should have been written under a blank work uid"
        );
    }

    /// An empty capture is not a write — closing a project with no container tabs open
    /// must not churn the file.
    #[test]
    fn an_empty_capture_writes_nothing() {
        let dir = tempdir().unwrap();
        let vm = vm(dir.path());
        vm.capture(&[]);
        assert!(!dir.path().join("tree_expansion.toml").exists());
    }

    /// The whole point, end to end against a **real** store: what a door captures is
    /// what the next tab restores, keyed by the open project's `Work.unique_id`.
    ///
    /// Needs the real backend — under `--features mocks` there is no `Work` to read a
    /// uid from, so `work_uid()` is `None` and both directions are correctly inert.
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn a_capture_round_trips_through_the_open_projects_uid() {
        use frontend::direct_access::CreateWorkDto;

        let dir = tempdir().unwrap();
        let service =
            TreeExpansionService::open_at(dir.path().join("tree_expansion.toml"), Duration::ZERO)
                .unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();

        let work = frontend::commands::work_commands::create_orphan_work(
            &app_ctx,
            None,
            &CreateWorkDto {
                unique_id: "project-alpha".to_string(),
                ..Default::default()
            },
        )
        .unwrap();
        ids.work_id.set(Some(work.id));

        let vm = TreeExpansionViewModel::new(app_ctx.clone(), ids.clone(), service.clone());
        let container = Uuid::from_u128(7);
        let expanded = vec![Uuid::from_u128(70), Uuid::from_u128(71)];

        assert!(
            vm.expanded_for(container).is_empty(),
            "nothing remembered yet"
        );
        vm.capture(&[(container, expanded.clone())]);
        assert_eq!(
            vm.expanded_for(container),
            expanded,
            "a fresh view-model over the same service reads back what was captured"
        );

        // A *different* project must not see it — the uid is the key, and two projects
        // sharing a container uid is exactly what keying by store id would have caused.
        let other = frontend::commands::work_commands::create_orphan_work(
            &app_ctx,
            None,
            &CreateWorkDto {
                unique_id: "project-beta".to_string(),
                ..Default::default()
            },
        )
        .unwrap();
        ids.work_id.set(Some(other.id));
        assert!(
            vm.expanded_for(container).is_empty(),
            "another project's expand state must not leak in"
        );
    }

    /// A project with no `unique_id` yet (brand-new, never saved) writes nothing —
    /// otherwise every unsaved project would share one row keyed by "".
    #[cfg(not(feature = "mocks"))]
    #[test]
    fn a_project_without_a_uid_is_not_persisted() {
        use frontend::direct_access::CreateWorkDto;

        let dir = tempdir().unwrap();
        let service =
            TreeExpansionService::open_at(dir.path().join("tree_expansion.toml"), Duration::ZERO)
                .unwrap();
        let app_ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        let work = frontend::commands::work_commands::create_orphan_work(
            &app_ctx,
            None,
            &CreateWorkDto::default(), // no unique_id
        )
        .unwrap();
        ids.work_id.set(Some(work.id));

        let vm = TreeExpansionViewModel::new(app_ctx, ids, service);
        let container = Uuid::from_u128(7);
        vm.capture(&[(container, vec![Uuid::from_u128(70)])]);
        assert!(vm.expanded_for(container).is_empty());
    }
}
