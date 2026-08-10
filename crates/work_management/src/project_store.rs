// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! [`ProjectStore`] — one value per **open project**, for code outside this
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
//! A `HashMap` keyed by `Work.unique_id` cannot have that bug: there is no
//! "current project" to repoint.
//!
//! ## Why the by-uid methods are not public
//!
//! [`ProjectStore::scoped`] is the only public door, and it returns `None` for
//! the empty uid — which is exactly what an unsaved `Work` carries. So "a handle
//! answering for no project" is a value that cannot be constructed, rather than a
//! runtime check every caller has to remember (the check `uid_is_usable()` exists
//! for on the UI side, generalised into the type). A public `set(uid, value)`
//! would be precisely the by-uid-string write path this abolishes, and it would
//! accept `""`.
//!
//! ## Send + Sync by construction
//!
//! `Arc<RwLock<..>>`, no `Rc`, no dependency on the UI framework — because the
//! save that reads it runs on a worker thread. The seam's third rule ("state the
//! UI edits and a save writes must be two types") is what this is the *save* half
//! of: pair it with a UI-side revision `Signal` that bumps only on a real change.

use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};

/// A `Send + Sync` store of one value per open project, keyed by
/// `Work.unique_id`. Cheap to clone; every clone shares one map.
pub struct ProjectStore<T> {
    inner: Arc<RwLock<HashMap<String, T>>>,
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

    fn read(&self) -> std::sync::RwLockReadGuard<'_, HashMap<String, T>> {
        // Poison-safe, like `lock_or_recover` elsewhere in the workspace: a
        // panic in an unrelated extension must not make a manuscript unsaveable.
        self.inner.read().unwrap_or_else(|e| e.into_inner())
    }

    fn write(&self) -> std::sync::RwLockWriteGuard<'_, HashMap<String, T>> {
        self.inner.write().unwrap_or_else(|e| e.into_inner())
    }

    // ── The by-uid plumbing. `pub(crate)` on purpose — see the module docs. ──

    pub(crate) fn get_by_uid(&self, uid: &str) -> Option<T>
    where
        T: Clone,
    {
        self.read().get(uid).cloned()
    }

    pub(crate) fn with_by_uid<R>(&self, uid: &str, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.read().get(uid).map(f)
    }

    pub(crate) fn set_by_uid(&self, uid: &str, value: T) {
        if uid.is_empty() {
            return;
        }
        self.write().insert(uid.to_string(), value);
    }

    pub(crate) fn update_by_uid<R>(&self, uid: &str, f: impl FnOnce(&mut T) -> R) -> Option<R>
    where
        T: Default,
    {
        if uid.is_empty() {
            return None;
        }
        let mut guard = self.write();
        Some(f(guard.entry(uid.to_string()).or_default()))
    }

    pub(crate) fn forget_by_uid(&self, uid: &str) {
        self.write().remove(uid);
    }

    // ── The public door ──────────────────────────────────────────────────────

    /// A handle permanently bound to one project, or `None` if `uid` names no
    /// project at all.
    ///
    /// The empty string is the uid an **unsaved** `Work` carries, and writing
    /// under it would pool every unsaved project's state into one slot that no
    /// save ever asks for. Refusing to build the handle is what makes that
    /// unexpressible instead of merely wrong.
    pub fn scoped(&self, uid: impl AsRef<str>) -> Option<WorkScoped<T>> {
        let uid = uid.as_ref();
        if uid.is_empty() {
            return None;
        }
        Some(WorkScoped {
            store: self.clone(),
            uid: Arc::from(uid),
        })
    }

    /// Drop a project's slot — on `LifecycleEvent::Closed`, or whenever the
    /// caller knows the project is gone.
    ///
    /// Takes a [`WorkScoped`] rather than a raw uid so it cannot name a project
    /// the caller never opened.
    pub fn evict(&self, scope: &WorkScoped<T>) {
        self.forget_by_uid(&scope.uid);
    }
}

/// A [`ProjectStore`] slot bound to one project for good.
///
/// It cannot be repointed, which is the whole difference from the singleton it
/// replaces: a value captured by a background command, an undo step, or a save
/// contributor keeps answering for the project it was made for, however many
/// others open afterwards.
pub struct WorkScoped<T> {
    store: ProjectStore<T>,
    uid: Arc<str>,
}

impl<T> Clone for WorkScoped<T> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            uid: Arc::clone(&self.uid),
        }
    }
}

impl<T> std::fmt::Debug for WorkScoped<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkScoped")
            .field("uid", &self.uid)
            .finish()
    }
}

impl<T> WorkScoped<T> {
    /// The `Work.unique_id` this handle is bound to. Never empty.
    pub fn uid(&self) -> &str {
        &self.uid
    }

    /// Whether this handle names the same project as `uid`.
    pub fn is(&self, uid: &str) -> bool {
        &*self.uid == uid
    }

    /// This project's value, cloned out.
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.store.get_by_uid(&self.uid)
    }

    /// Read this project's value in place, without cloning it.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        self.store.with_by_uid(&self.uid, f)
    }

    pub fn get_or_default(&self) -> T
    where
        T: Clone + Default,
    {
        self.get().unwrap_or_default()
    }

    pub fn set(&self, value: T) {
        self.store.set_by_uid(&self.uid, value);
    }

    /// Mutate this project's value in place, creating it from `Default` if the
    /// project has none yet. Returns whatever `f` returns — return `bool` from it
    /// and hand that straight to `WorkHandle::mark_changed`.
    pub fn update<R>(&self, f: impl FnOnce(&mut T) -> R) -> R
    where
        T: Default,
    {
        self.store
            .update_by_uid(&self.uid, f)
            .expect("a WorkScoped is never built for the empty uid")
    }

    /// Whether this project has a value at all.
    pub fn exists(&self) -> bool {
        self.store.with_by_uid(&self.uid, |_| ()).is_some()
    }
}

/// Emptying a project's slot when the project closes — see
/// [`crate::lifecycle::evict_on_close`], which is what calls this.
impl<T: Send + Sync> crate::lifecycle::ProjectSlots for ProjectStore<T> {
    fn forget(&self, work_unique_id: &str) {
        self.forget_by_uid(work_unique_id);
    }
}

/// The one correct [`BundleContributor`](crate::bundle_contributors::BundleContributor)
/// for a [`ProjectStore`].
///
/// Replaces a hand-rolled contributor plus an "is this my project?" equality
/// check with a `HashMap` lookup: there is no current project to compare
/// against, so every uid a save asks about is answered independently and two
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
    fn files(&self, work_unique_id: &str) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
        let mut out = BTreeMap::new();
        if let Some(bytes) = self
            .store
            .with_by_uid(work_unique_id, |value| (self.serialize)(value))
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
    use crate::bundle_contributors::BundleContributor;

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
        let a = store.scoped("uid-a").expect("a real uid");
        let b = store.scoped("uid-b").expect("a real uid");

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

    /// The empty uid is what an **unsaved** `Work` carries. A handle for it must
    /// not exist, so nothing can pool every unsaved project into one slot.
    #[test]
    fn a_scoped_handle_cannot_be_built_for_the_empty_uid() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        assert!(store.scoped("").is_none());
        assert!(store.scoped(String::new()).is_none());
        // …and the crate-internal path refuses it too, so no internal caller can
        // sneak one in behind the public door.
        store.set_by_uid("", Plan::default());
        assert!(store.is_empty());
    }

    /// A scoped handle names one project for good — the property that lets an
    /// undo step or a save contributor hold one safely.
    #[test]
    fn a_scope_cannot_be_repointed() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        let a = store.scoped("uid-a").unwrap();
        let a2 = a.clone();
        let _b = store.scoped("uid-b").unwrap();
        assert_eq!(a.uid(), "uid-a");
        assert_eq!(a2.uid(), "uid-a");
        assert!(a.is("uid-a") && !a.is("uid-b"));
    }

    /// `update` creates from `Default` and returns whatever the closure returns —
    /// the shape that lets a caller hand a `bool` straight to
    /// `WorkHandle::mark_changed`.
    #[test]
    fn update_creates_on_first_touch_and_reports_the_change() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        let a = store.scoped("uid-a").unwrap();
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
        assert!(c.files("uid-never-opened").unwrap().is_empty());
    }

    /// …but a project whose value was *emptied* still writes, or the removal
    /// would never reach disk: carry-through would keep restoring the old file.
    #[test]
    fn the_contributor_writes_an_emptied_value_so_the_removal_persists() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        let a = store.scoped("uid-a").unwrap();
        a.set(Plan {
            beats: vec!["opening".into()],
        });
        a.set(Plan::default());

        let c = ProjectStoreContributor {
            path: "test/plan.txt",
            store: store.clone(),
            serialize,
        };
        let files = c.files("uid-a").unwrap();
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
        store.scoped("uid-a").unwrap().set(Plan {
            beats: vec!["a".into()],
        });
        store.scoped("uid-b").unwrap().set(Plan {
            beats: vec!["b".into()],
        });
        let c = ProjectStoreContributor {
            path: "test/plan.txt",
            store: store.clone(),
            serialize,
        };
        assert_eq!(
            c.files("uid-a").unwrap().get("test/plan.txt").unwrap(),
            b"a"
        );
        assert_eq!(
            c.files("uid-b").unwrap().get("test/plan.txt").unwrap(),
            b"b"
        );
    }

    /// Eviction is what stops a session accumulating every project it ever
    /// opened — and, worse, re-opening one finding a stale slot instead of
    /// loading from disk.
    #[test]
    fn evicting_a_scope_forgets_only_that_project() {
        let store: ProjectStore<Plan> = ProjectStore::new();
        let a = store.scoped("uid-a").unwrap();
        let b = store.scoped("uid-b").unwrap();
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
        store.scoped("uid-a").unwrap().set(Plan {
            beats: vec!["x".into()],
        });
        assert!(other.scoped("uid-a").unwrap().exists());
    }
}
