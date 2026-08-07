// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `BackupSettingsViewModel` — a cloneable handle over [`BackupSettingsService`].
//!
//! Registered as `app_state`; the settings panes and the backup scheduler each
//! hold a clone. Every clone shares the same underlying `SettingsFile`, so a
//! change made in one is visible to all. It stays a thin wrapper (the service
//! does the persistence) plus small conveniences for the two callers.

use std::cell::Cell;
use std::rc::Rc;

use bastyde::prelude::{AsyncRuntimeHandle, Signal, spawn_blocking};

use crate::backup_paths::{self, RootUsage};
use crate::models::{BackupPolicy, BackupSettingsService};

#[derive(Clone)]
pub struct BackupSettingsViewModel {
    service: BackupSettingsService,
    /// What the app's backup root holds, or `None` until first measured.
    ///
    /// Shared by every clone (it is one directory for the whole installation), so
    /// the settings pane and any future reader agree without rescanning.
    root_usage: Signal<Option<RootUsage>>,
    /// Whether the root has been measured, or a measurement is in flight.
    ///
    /// Shared by every clone for the same reason `root_usage` is: one directory,
    /// one answer, one scan. See [`Self::refresh_root_usage`].
    measured: Rc<Cell<bool>>,
    /// Bumped whenever the **policy** changes — the general one, or any project's
    /// override. See [`Self::policy_revision`].
    policy_revision: Signal<u64>,
}

impl BackupSettingsViewModel {
    pub fn new(service: BackupSettingsService) -> Self {
        Self {
            service,
            root_usage: Signal::new(None),
            measured: Rc::new(Cell::new(false)),
            policy_revision: Signal::new(0),
        }
    }

    /// A counter that moves whenever the effective policy of *any* project could
    /// have changed.
    ///
    /// The settings file is not reactive — it is a locked read-modify-write behind
    /// plain getters — so a reader that resolved something out of it once has no
    /// way to learn that it has moved. The Versions dock and the Timeline band
    /// resolve **where a project's past is kept** from the policy's destinations,
    /// and used to be re-pushed only when the project's name, its uid, a backup or
    /// a save changed. Add a destination in Settings and neither surface looked at
    /// it until one of those happened to fire.
    ///
    /// Deliberately one counter for the whole file rather than a signal per
    /// project: destinations can move by way of the *general* policy an override
    /// falls back to, so "did my project's policy change" is not a question a
    /// per-project key can answer.
    pub fn policy_revision(&self) -> Signal<u64> {
        self.policy_revision.clone()
    }

    fn bump_policy(&self) {
        let r = &self.policy_revision;
        r.set(r.get().wrapping_add(1));
    }

    // ── the app's backup root ──

    /// How much the app's backup root holds — `None` until [`Self::refresh_root_usage`]
    /// has landed once.
    pub fn root_usage(&self) -> Signal<Option<RootUsage>> {
        self.root_usage.clone()
    }

    /// Measure the backup root, off the UI thread when an executor is reachable.
    ///
    /// Moving the default destination out of the writer's own folder
    /// ([`crate::backup_paths`]) means they can no longer see the backups piling
    /// up, so the settings pane owes them the number. Retention bounds the *count*
    /// but not the *bytes* — a project carrying photographs can put gigabytes
    /// somewhere nobody looks.
    ///
    /// Degrades to a synchronous scan with no executor, exactly as
    /// `BackupsListViewModel::reload` does: a headless test still gets a real
    /// answer rather than a permanent `None`.
    ///
    /// **Measures at most once**, like every other scan in this feature
    /// (`VersionsViewModel::load_for`, `TimelineViewModel::scan`,
    /// `BackupsListPanel`). The caller is `general_pane`, which runs on every
    /// rebuild of the *whole* Settings window — so editing an unrelated field on
    /// an unrelated page re-entered this. Unguarded that meant a fresh recursive
    /// walk of every project's retained backups per keystroke, and with no
    /// executor reachable it ran that walk on the UI thread.
    ///
    /// [`Self::invalidate_root_usage`] is how the number stops being stale: a
    /// completed backup run calls it, and the next time the pane is built the
    /// measurement is taken again.
    pub fn refresh_root_usage(&self, rt: Option<AsyncRuntimeHandle>) {
        // Set before spawning, not after landing: two builds in the same frame
        // would otherwise both find it clear and both scan.
        if self.measured.replace(true) {
            return;
        }
        let out = self.root_usage.clone();
        let root = backup_paths::backup_root();
        match rt {
            Some(rt) => {
                rt.spawn_local(async move {
                    let usage = spawn_blocking(move || backup_paths::scan_root_usage(&root))
                        .await
                        .unwrap_or_default();
                    out.set(Some(usage));
                })
                .detach();
            }
            None => out.set(Some(backup_paths::scan_root_usage(&root))),
        }
    }

    /// Mark the measured usage stale, so the next [`Self::refresh_root_usage`]
    /// really looks again.
    ///
    /// Called when a backup run completes — that is the one event that changes
    /// what the root holds, by writing a new bundle and by pruning old ones. The
    /// last measurement is deliberately left on screen until a new one lands: a
    /// number that goes blank while a writer is reading it is worse than a number
    /// that is a few minutes old.
    pub fn invalidate_root_usage(&self) {
        self.measured.set(false);
    }

    /// The underlying service (read-only queries the scheduler may prefer direct).
    pub fn service(&self) -> &BackupSettingsService {
        &self.service
    }

    // ── general policy ──
    pub fn general(&self) -> BackupPolicy {
        self.service.general()
    }

    pub fn set_general(&self, policy: BackupPolicy) {
        if let Err(e) = self.service.set_general(policy) {
            eprintln!("backup settings: set_general failed: {e}");
            return;
        }
        self.bump_policy();
    }

    // ── per-project overrides ──
    pub fn has_override(&self, uid: &str) -> bool {
        self.service.has_override(uid)
    }

    pub fn effective_for(&self, uid: &str) -> BackupPolicy {
        self.service.effective_for(uid)
    }

    pub fn set_override(&self, uid: &str, last_path: &str, title: &str, policy: BackupPolicy) {
        if let Err(e) = self.service.set_override(uid, last_path, title, policy) {
            eprintln!("backup settings: set_override failed: {e}");
            return;
        }
        self.bump_policy();
    }

    pub fn clear_override(&self, uid: &str) {
        if let Err(e) = self.service.clear_override(uid) {
            eprintln!("backup settings: clear_override failed: {e}");
            return;
        }
        self.bump_policy();
    }

    // ── bookkeeping (dedup + "last backup" + nudge) ──
    pub fn last_backup_at(&self, uid: &str) -> Option<String> {
        self.service.last_backup_at(uid)
    }

    pub fn record_destination_success(
        &self,
        uid: &str,
        last_path: &str,
        dir: &str,
        hash: &str,
        backup_path: &str,
        at: &str,
    ) {
        if let Err(e) =
            self.service
                .record_destination_success(uid, last_path, dir, hash, backup_path, at)
        {
            eprintln!("backup settings: record_destination_success failed: {e}");
        }
    }

    pub fn was_nudged(&self, uid: &str) -> bool {
        self.service.was_nudged(uid)
    }

    pub fn mark_nudged(&self, uid: &str, last_path: &str) {
        if let Err(e) = self.service.mark_nudged(uid, last_path) {
            eprintln!("backup settings: mark_nudged failed: {e}");
        }
    }

    pub fn flush_now(&self) {
        if let Err(e) = self.service.flush_now() {
            eprintln!("backup settings: flush failed: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::BackupSettingsService;

    /// A value no real scan of a directory can produce, so its survival proves no
    /// scan ran.
    fn sentinel() -> RootUsage {
        RootUsage {
            count: usize::MAX,
            bytes: u64::MAX,
            oldest: "sentinel".into(),
        }
    }

    fn vm() -> BackupSettingsViewModel {
        BackupSettingsViewModel::new(BackupSettingsService::in_memory_default())
    }

    /// **The bug this guards.** `general_pane` calls `refresh_root_usage` on every
    /// rebuild of the Settings *window*, not of the Backup page — so typing in an
    /// unrelated field on an unrelated page re-entered it. With no executor that is
    /// a recursive walk of the whole backup root on the UI thread, per keystroke.
    #[test]
    fn the_backup_root_is_measured_once_however_often_the_pane_rebuilds() {
        let vm = vm();
        vm.refresh_root_usage(None);
        assert!(
            vm.root_usage().get().is_some(),
            "precondition: the first call really measures, inline with no executor",
        );

        vm.root_usage().set(Some(sentinel()));
        for _ in 0..20 {
            vm.refresh_root_usage(None);
        }
        assert_eq!(
            vm.root_usage().get(),
            Some(sentinel()),
            "a rebuild must not start a second scan of the backup root",
        );
    }

    /// …but the number must not be frozen for the session either: a completed
    /// backup writes a bundle and prunes old ones, so the next time the pane is
    /// built it has to look again.
    #[test]
    fn a_completed_backup_makes_the_next_build_measure_again() {
        let vm = vm();
        vm.refresh_root_usage(None);
        vm.root_usage().set(Some(sentinel()));

        vm.invalidate_root_usage();
        vm.refresh_root_usage(None);
        assert_ne!(
            vm.root_usage().get(),
            Some(sentinel()),
            "an invalidated measurement must be taken again, not kept",
        );
    }

    /// **The bug this guards.** The Versions dock and the Timeline band resolve
    /// where a project's past is kept from the policy's destinations, and the
    /// settings file is not reactive — so adding a destination in Settings left
    /// both surfaces reading the old list until an unrelated save or backup
    /// happened to re-push the handle.
    #[test]
    fn changing_the_policy_tells_the_readers_that_the_destinations_moved() {
        let vm = vm();
        let seen = vm.policy_revision();
        let at_start = seen.get();

        let mut policy = vm.general();
        policy.destinations = vec!["/somewhere/new".into()];
        vm.set_general(policy);
        let after_general = seen.get();
        assert_ne!(
            after_general, at_start,
            "a general policy edit must be seen"
        );

        vm.set_override("work-uid", "/Novel.skrib", "Novel", vm.general());
        let after_override = seen.get();
        assert_ne!(
            after_override, after_general,
            "…and so must a per-project override, which can move one project's \
             destinations without touching the general policy",
        );

        vm.clear_override("work-uid");
        assert_ne!(
            seen.get(),
            after_override,
            "…and dropping the override, which sends it back to the general one",
        );
    }

    /// Bookkeeping is not a policy change: the scheduler records a destination
    /// success on every run, and a counter that moved then would re-push the
    /// project handle — invalidating every cache in the feature — on each one.
    #[test]
    fn recording_a_backups_success_is_not_a_policy_change() {
        let vm = vm();
        let seen = vm.policy_revision();
        let before = seen.get();
        vm.record_destination_success(
            "work-uid",
            "/Novel.skrib",
            "/backups",
            "hash",
            "/backups/Novel-1.skrib",
            "2026-08-07T12:00:00Z",
        );
        vm.mark_nudged("work-uid", "/Novel.skrib");
        assert_eq!(seen.get(), before);
    }

    /// Every clone shares the one measurement, because there is one backup root.
    #[test]
    fn clones_share_the_measurement_rather_than_each_scanning() {
        let vm = vm();
        let other = vm.clone();
        vm.refresh_root_usage(None);
        vm.root_usage().set(Some(sentinel()));

        other.refresh_root_usage(None);
        assert_eq!(
            vm.root_usage().get(),
            Some(sentinel()),
            "the settings pane and the scheduler hold clones; one scan serves both",
        );
    }
}
