// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ProjectStore` — one value per **open project**, for code outside this
//! workspace that has state of its own to keep beside a manuscript.
//!
//! ## The bug class this replaces
//!
//! Skribisto opens several `Work`s at once (`Root.works` is multi-valued, and
//! Work ▸ New Window puts two windows on one project). An extension that held its
//! state in a singleton — one `{work_unique_id, data}` pair, repointed whenever a
//! panel for a different project built — therefore *evicted* the first project's
//! state the moment the second one's panel opened. From that instant
//! [`BundleContributor::files`](crate::bundle_contributors::BundleContributor::files)
//! answered nothing for the first project, so its state silently stopped being
//! saved, with no error anywhere. This was not hypothetical; it is the shape the
//! first real extension shipped with.
//!
//! A `HashMap` keyed by the project cannot have that bug: there is no "current
//! project" to repoint.
//!
//! ## Keyed by the open Work, not by `unique_id`
//!
//! The key is [`OpenWork`](common::open_work::OpenWork), one per Work open in this process. It used to be
//! `Work.unique_id`, and that gave two projects one slot whenever two copies of
//! one project were open together, which is ordinary: Save As keeps the id, so
//! does a copy made in a file manager, and a backup carries it by design. Open a
//! backup from the Backups list beside the project, and the backup's state loaded
//! over the live project's, which the live project's next save then wrote out as
//! its own. Closing either copy emptied the slot both were using.
//!
//! Keep `unique_id` for what outlives the process: a file in the project's
//! bundle, a record kept between sessions, a request to a service. See
//! [`common::open_work`] for the rule.
//!
//! An unsaved project has a key like any other, so an extension can hold state
//! for it from the start, and its first save writes that state out.
//!
//! ## Send + Sync by construction
//!
//! `Arc<RwLock<..>>`, no `Rc`, no dependency on the UI framework — because the
//! save that reads it runs on a worker thread. The seam's third rule ("state the
//! UI edits and a save writes must be two types") is what this is the *save* half
//! of: pair it with a UI-side revision `Signal` that bumps only on a real change.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};

pub use common::open_work::OpenWork;

/// A `Send + Sync` store of one value per open project, keyed by
/// [`OpenWork`]. Cheap to clone; every clone shares one map.
pub struct ProjectStore<T> {
    inner: Arc<RwLock<HashMap<OpenWork, T>>>,
}

// Hand-written rather than derived: `#[derive(Clone)]` would demand `T: Clone`,
// which is wrong — an `Arc` clone does not clone what it points at.
impl<T> Clone for ProjectStore<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Default for ProjectStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Hand-written for the same reason `Clone` is, plus one more: printing every
// project's value would put a whole plan in a log line. The count is what a
// caller actually wants to see.
impl<T> std::fmt::Debug for ProjectStore<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectStore")
            .field("projects", &self.read().len())
            .finish()
    }
}

impl<T> ProjectStore<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// How many projects currently hold a value. For eviction tests and
    /// diagnostics; not a substitute for [`Self::scoped`].
    pub fn len(&self) -> usize {
        self.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn read(&self) -> std::sync::RwLockReadGuard<'_, HashMap<OpenWork, T>> {
        // Poison-safe, like `lock_or_recover` elsewhere in the workspace: a
        // panic in an unrelated extension must not make a manuscript unsaveable.
        self.inner.read().unwrap_or_else(|e| e.into_inner())
    }

    fn write(&self) -> std::sync::RwLockWriteGuard<'_, HashMap<OpenWork, T>> {
        self.inner.write().unwrap_or_else(|e| e.into_inner())
    }

    // ── The plumbing behind `WorkScoped`. ────────────────────────────────────

    pub(crate) fn get_by_work(&self, work: OpenWork) -> Option<T>
    where
        T: Clone,
    {
        self.read().get(&work).cloned()
    }

    pub(crate) fn with_by_work<R>(&self, work: OpenWork, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.read().get(&work).map(f)
    }

    pub(crate) fn set_by_work(&self, work: OpenWork, value: T) {
        self.write().insert(work, value);
    }

    pub(crate) fn update_by_work<R>(&self, work: OpenWork, f: impl FnOnce(&mut T) -> R) -> R
    where
        T: Default,
    {
        let mut guard = self.write();
        f(guard.entry(work).or_default())
    }

    pub(crate) fn forget_by_work(&self, work: OpenWork) {
        self.write().remove(&work);
    }

    // ── The public door ──────────────────────────────────────────────────────

    /// A handle permanently bound to one open project.
    ///
    /// Take `work` from the seam: [`LifecycleEvent`](crate::lifecycle::LifecycleEvent)
    /// and [`SaveContext`](crate::bundle_contributors::SaveContext) carry it, and
    /// the UI side derives it from the window's Work id.
    pub fn scoped(&self, work: OpenWork) -> WorkScoped<T> {
        WorkScoped {
            store: self.clone(),
            work,
        }
    }

    /// Drop a project's slot — on `LifecycleEvent::Closed`, or whenever the
    /// caller knows the project is gone.
    ///
    /// Takes a [`WorkScoped`] rather than a bare key so it cannot name a project
    /// the caller never opened.
    pub fn evict(&self, scope: &WorkScoped<T>) {
        scope.forget();
    }
}

/// A `ProjectStore` slot bound to one project for good.
///
/// It cannot be repointed, which is the whole difference from the singleton it
/// replaces: a value captured by a background command, an undo step, or a save
/// contributor keeps answering for the project it was made for, however many
/// others open afterwards.
pub struct WorkScoped<T> {
    store: ProjectStore<T>,
    work: OpenWork,
}

impl<T> Clone for WorkScoped<T> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            work: self.work,
        }
    }
}

impl<T> std::fmt::Debug for WorkScoped<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkScoped")
            .field("work", &self.work)
            .finish()
    }
}

impl<T> WorkScoped<T> {
    /// The open project this handle is bound to.
    pub fn work(&self) -> OpenWork {
        self.work
    }

    /// Whether this handle names the open project `work`.
    pub fn is(&self, work: OpenWork) -> bool {
        self.work == work
    }

    /// This project's value, cloned out.
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.store.get_by_work(self.work)
    }

    /// Read this project's value in place, without cloning it.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.store.with_by_work(self.work, f)
    }

    pub fn get_or_default(&self) -> T
    where
        T: Clone + Default,
    {
        self.get().unwrap_or_default()
    }

    pub fn set(&self, value: T) {
        self.store.set_by_work(self.work, value);
    }

    /// Mutate this project's value in place, creating it from `Default` if the
    /// project has none yet. Returns whatever `f` returns — return `bool` from it
    /// and hand that straight to `WorkHandle::mark_changed`.
    pub fn update<R>(&self, f: impl FnOnce(&mut T) -> R) -> R
    where
        T: Default,
    {
        self.store.update_by_work(self.work, f)
    }

    /// Whether this project has a value at all.
    pub fn exists(&self) -> bool {
        self.store.with_by_work(self.work, |_| ()).is_some()
    }

    /// Drop this project's slot, back to the state before anything was stored.
    ///
    /// Not the same as storing `T::default()`, and the difference reaches disk:
    /// [`ProjectStoreContributor`] writes a file for a project that *has* a
    /// value, however empty, and writes none for one that has no slot. So an undo
    /// that steps back past the first edit a project ever received has to come
    /// here, or it leaves an empty file behind in a project that was never
    /// touched.
    pub fn forget(&self) {
        self.store.forget_by_work(self.work);
    }
}

/// Emptying a project's slot when the project closes — see
/// [`crate::lifecycle::evict_on_close`], which is what calls this.
impl<T: Send + Sync> crate::lifecycle::ProjectSlots for ProjectStore<T> {
    fn forget(&self, work: OpenWork) {
        self.forget_by_work(work);
    }
}

/// The one correct [`BundleContributor`](crate::bundle_contributors::BundleContributor)
/// for a `ProjectStore`.
///
/// Replaces a hand-rolled contributor plus an "is this my project?" equality
/// check with a `HashMap` lookup: there is no current project to compare
/// against, so every project a save asks about is answered independently and two
/// simultaneously-open projects cannot leak into each other's bundle.
///
/// A project with no value writes **no file**, and carry-through then preserves
/// whatever the bundle already had — which is what makes an extension's data
/// survive a save by a build that has never heard of it.
pub struct ProjectStoreContributor<T> {
    /// Bundle-relative path, `/`-separated (e.g. `"pro/structure.ron"`). Must not
    /// be a modelled path; `collect` refuses those.
    pub path: &'static str,
    pub store: ProjectStore<T>,
    /// Serialise one project's value. An `Err` skips this contributor for this
    /// save with a message — it can never fail the save itself.
    pub serialize: fn(&T) -> anyhow::Result<Vec<u8>>,
}

impl<T: Send + Sync> crate::bundle_contributors::BundleContributor for ProjectStoreContributor<T> {
    /// Ignores everything in the context but the open project, deliberately: a
    /// store's contents are whatever the UI last put there, and writing them out
    /// is the same act whether the writer saved, saved-as or a backup ran. A
    /// contributor that *does* care which write this is — one recording history,
    /// say — wants its own implementation, not a flag here.
    fn files(
        &self,
        ctx: &crate::bundle_contributors::SaveContext,
    ) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
        let mut out = BTreeMap::new();
        if let Some(bytes) = self
            .store
            .with_by_work(ctx.work, |value| (self.serialize)(value))
            .transpose()?
        {
            out.insert(self.path.to_string(), bytes);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle_contributors::{BundleContributor, SaveContext};
    use std::sync::LazyLock;

    /// Two open projects. Every test builds its own store, so sharing the keys
    /// across tests cannot make two of them see each other.
    static A: LazyLock<OpenWork> = LazyLock::new(OpenWork::detached);
    static B: LazyLock<OpenWork> = LazyLock::new(OpenWork::detached);

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct Plan {
        beats: Vec<String>,
    }

    fn serialize(p: &Plan) -> anyhow::Result<Vec<u8>> {
        Ok(p.beats.join(",").into_bytes())
    }

    /// **The regression test for the singleton.** Two projects open at once, each
    /// with its own plan: neither may see, overwrite or evict the other's.
    #[test]
    fn two_projects_in_one_store_never_see_each_others_data() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        let a = store.scoped(*A);
        let b = store.scoped(*B);

        a.set(Plan {
            beats: vec!["opening".into()],
        });
        // Opening B's panel is what used to repoint the singleton.
        b.set(Plan {
            beats: vec!["midpoint".into()],
        });

        assert_eq!(a.get().unwrap().beats, vec!["opening".to_string()]);
        assert_eq!(b.get().unwrap().beats, vec!["midpoint".to_string()]);
        assert_eq!(store.len(), 2);
    }

    /// An unsaved project has a key like any other, so the handle always exists:
    /// there is no "no project" value left to refuse.
    #[test]
    fn every_open_project_gets_a_handle() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        store.scoped(*A).set(Plan::default());
        assert!(store.scoped(*A).exists());
        assert!(!store.scoped(*B).exists());
    }

    /// A scoped handle names one project for good — the property that lets an
    /// undo step or a save contributor hold one safely.
    #[test]
    fn a_scope_cannot_be_repointed() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        let a = store.scoped(*A);
        let a2 = a.clone();
        let _b = store.scoped(*B);
        assert_eq!(a.work(), *A);
        assert_eq!(a2.work(), *A);
        assert!(a.is(*A) && !a.is(*B));
    }

    /// `update` creates from `Default` and returns whatever the closure returns —
    /// the shape that lets a caller hand a `bool` straight to
    /// `WorkHandle::mark_changed`.
    #[test]
    fn update_creates_on_first_touch_and_reports_the_change() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        let a = store.scoped(*A);
        assert!(!a.exists());

        let changed = a.update(|p| {
            p.beats.push("opening".into());
            true
        });
        assert!(changed);
        assert!(a.exists());
        assert_eq!(a.get().unwrap().beats.len(), 1);
    }

    /// A save asks about a project the store knows nothing about: it must write
    /// **no file**, so carry-through preserves whatever the bundle already held.
    #[test]
    fn the_contributor_writes_nothing_for_an_untouched_project() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        let c = ProjectStoreContributor {
            path: "test/plan.txt",
            store: store.clone(),
            serialize,
        };
        assert!(
            c.files(&SaveContext::for_work(OpenWork::detached()))
                .unwrap()
                .is_empty()
        );
    }

    /// …but a project whose value was *emptied* still writes, or the removal
    /// would never reach disk: carry-through would keep restoring the old file.
    #[test]
    fn the_contributor_writes_an_emptied_value_so_the_removal_persists() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        let a = store.scoped(*A);
        a.set(Plan {
            beats: vec!["opening".into()],
        });
        a.set(Plan::default());

        let c = ProjectStoreContributor {
            path: "test/plan.txt",
            store: store.clone(),
            serialize,
        };
        let files = c.files(&SaveContext::for_work(*A)).unwrap();
        assert_eq!(
            files.get("test/plan.txt").map(Vec::as_slice),
            Some(&b""[..])
        );
    }

    /// Each project's save sees only its own value — the same isolation, checked
    /// where it actually reaches disk.
    #[test]
    fn the_contributor_answers_per_project() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        store.scoped(*A).set(Plan {
            beats: vec!["a".into()],
        });
        store.scoped(*B).set(Plan {
            beats: vec!["b".into()],
        });
        let c = ProjectStoreContributor {
            path: "test/plan.txt",
            store: store.clone(),
            serialize,
        };
        assert_eq!(
            c.files(&SaveContext::for_work(*A))
                .unwrap()
                .get("test/plan.txt")
                .unwrap(),
            b"a"
        );
        assert_eq!(
            c.files(&SaveContext::for_work(*B))
                .unwrap()
                .get("test/plan.txt")
                .unwrap(),
            b"b"
        );
    }

    /// Eviction is what stops a session accumulating every project it ever
    /// opened. A reopen is a new Work with a new key, so it never finds the old
    /// slot; without eviction the old slot simply stays.
    #[test]
    fn evicting_a_scope_forgets_only_that_project() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        let a = store.scoped(*A);
        let b = store.scoped(*B);
        a.set(Plan::default());
        b.set(Plan {
            beats: vec!["kept".into()],
        });

        store.evict(&a);
        assert!(!a.exists(), "the closed project's slot must be gone");
        assert_eq!(b.get().unwrap().beats, vec!["kept".to_string()]);
        assert_eq!(store.len(), 1);
    }

    /// Every clone is one store — the property a registered contributor depends
    /// on, since it holds a clone made at registration and the UI edits another.
    #[test]
    fn clones_share_one_map() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        let other = store.clone();
        store.scoped(*A).set(Plan {
            beats: vec!["x".into()],
        });
        assert!(other.scoped(*A).exists());
    }
}
