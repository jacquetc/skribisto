// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Project lifecycle: **opened**, **closed**, **saved** — for code outside this
//! workspace that keeps state beside a manuscript.
//!
//! ## Why a backend hook and not a UI subscription
//!
//! An extension can already watch the event hub from a panel. That answers a
//! different question, badly:
//!
//! - A panel is only wired **while it is visible**. An extension whose data layer
//!   depends on its dock having been opened does not load a project the writer
//!   never looked at it in — and then writes an empty file over their data.
//! - The hub's events carry ephemeral store **ids**, not `Work.unique_id` or a
//!   path. Resolving them means a live `AppContext`, which a `Send` closure may
//!   not hold (seam rule 2), and `EntityId` is re-minted on every load anyway
//!   (seam rule 4).
//! - `Closed` fires *after* the subtree is gone. There is nothing left to resolve
//!   an id against, so the payload has to be captured before teardown — which
//!   only the use case itself can do.
//!
//! So the payload here is plain, already-resolved data — `unique_id`, a path, a
//! `Vec<Uuid>` — captured inside the use case and handed over whole. No context
//! crosses the boundary in either direction.
//!
//! ## The pruning problem this exists to solve
//!
//! **No cascade reaches an extension.** Delete a scene and anything an extension
//! bound to it dangles forever; the core knows nothing about the extension's
//! tree. `Opened`/`Saved` therefore carry `live_binder_item_uids` — every uid
//! that exists in the project right now — so a listener can prune in one pass
//! with no lookup and no `AppContext`.
//!
//! ## Threads
//!
//! `LifecycleListener` is `Send + Sync` because `Saved` fires from the save
//! worker thread (a long operation), while `Opened`/`Closed` fire on whichever
//! thread ran the command — today the UI thread. A listener must work from
//! either, must not block, and cannot fail a save: a panic is caught and
//! reported, exactly as [`crate::bundle_contributors`] already does, because a
//! broken extension may never stand between a writer and their manuscript.

use std::sync::{Arc, LazyLock, RwLock};

use uuid::Uuid;

/// Which write produced a [`LifecycleEvent::Saved`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SaveKind {
    /// `save_work` — the project overwritten in place.
    Save,
    /// `save_as` — written to a new path, which becomes the project's.
    SaveAs,
    /// `backup_now` — a copy to a backup destination. Fires **once per
    /// destination**, and the project's own file is untouched.
    Backup,
}

/// Something that happened to a whole project.
#[derive(Clone, Debug)]
pub enum LifecycleEvent {
    /// A project is now open in the store. Fired once from `load_work` /
    /// `new_work`, strictly after the transaction commits.
    Opened {
        unique_id: String,
        /// Absolute path of the file or folder opened. Empty for a `new_work`
        /// that has never been saved — which is also the state whose
        /// `unique_id` cannot yet key a [`crate::project_store::ProjectStore`].
        path: String,
        /// Every `BinderItem` uid the project currently holds. Prune against it.
        live_binder_item_uids: Vec<Uuid>,
    },
    /// A project's subtree is gone from the store. Fired once from `close_work`,
    /// strictly **after** the teardown — but with a `unique_id` captured
    /// **before** it, since afterwards there is no row left to read it from.
    Closed { unique_id: String },
    /// A write landed. Once for a save or a save-as; once **per destination**
    /// for a backup.
    Saved {
        unique_id: String,
        /// Where these bytes went. For a backup that is the backup file, not the
        /// project.
        path: String,
        kind: SaveKind,
        live_binder_item_uids: Vec<Uuid>,
    },
}

/// A listener on project lifecycle.
pub trait LifecycleListener: Send + Sync {
    /// Must not block. A panic is caught and reported rather than propagated —
    /// see the module docs.
    fn on_event(&self, event: &LifecycleEvent);
}

/// Emptying a project's arrival tally when the project closes.
///
/// The tally itself lives in `common` because both ends need it and they sit on
/// opposite sides of this crate — see [`common::arrival`]. The `ProjectSlots`
/// impl has to be here, where the trait is.
impl ProjectSlots for common::arrival::Arrivals {
    fn forget(&self, work_unique_id: &str) {
        common::arrival::Arrivals::forget(self, work_unique_id);
    }
}

/// A per-project slot store that should be emptied when a project closes.
///
/// Object-safe so the registry can hold stores of different value types
/// together; `ProjectStore<T>` implements it for every `T`.
pub trait ProjectSlots: Send + Sync {
    fn forget(&self, work_unique_id: &str);
}

struct Registration {
    namespace: String,
    listener: Arc<dyn LifecycleListener>,
}

struct Eviction {
    namespace: String,
    slots: Arc<dyn ProjectSlots>,
}

static LISTENERS: LazyLock<RwLock<Vec<Registration>>> = LazyLock::new(|| RwLock::new(Vec::new()));
static EVICTIONS: LazyLock<RwLock<Vec<Eviction>>> = LazyLock::new(|| RwLock::new(Vec::new()));

/// Listen for project lifecycle events.
///
/// `namespace` identifies the listener: registering the same one twice
/// **replaces** the earlier entry rather than stacking a second copy. The
/// returned handle unregisters on drop.
///
/// ⚠ Unlike the UI-side registries, this one is re-read **live on every event**
/// rather than snapshotted when a surface is built — an event has no surface to
/// snapshot at. Registration must still land before the first `load_work` /
/// `new_work` a listener cares about, which for an extension means startup.
pub fn register(namespace: impl Into<String>, listener: Arc<dyn LifecycleListener>) -> Handle {
    let namespace = namespace.into();
    let mut reg = LISTENERS.write().unwrap_or_else(|e| e.into_inner());
    reg.retain(|r| r.namespace != namespace);
    reg.push(Registration {
        namespace: namespace.clone(),
        listener,
    });
    Handle { namespace }
}

/// Have a [`ProjectStore`](crate::project_store::ProjectStore) emptied of a
/// project's slot when that project closes.
///
/// Without this, every project a session opens stays resident for the life of
/// the process — a slow leak, and worse a *correctness* hazard: re-opening the
/// same project would find a stale slot instead of loading from disk. Eviction
/// is therefore the app's job on the extension's behalf, not something each
/// extension has to remember.
///
/// Eviction runs **after** every listener, so a listener still sees the closing
/// project's state one last time.
pub fn evict_on_close(namespace: impl Into<String>, slots: Arc<dyn ProjectSlots>) -> Handle {
    let namespace = namespace.into();
    let mut reg = EVICTIONS.write().unwrap_or_else(|e| e.into_inner());
    reg.retain(|r| r.namespace != namespace);
    reg.push(Eviction {
        namespace: namespace.clone(),
        slots,
    });
    Handle { namespace }
}

/// Unregisters its listener (and any eviction under the same namespace) on drop.
pub struct Handle {
    namespace: String,
}

impl Drop for Handle {
    fn drop(&mut self) {
        LISTENERS
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|r| r.namespace != self.namespace);
        EVICTIONS
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|r| r.namespace != self.namespace);
    }
}

/// Whether anything is listening.
///
/// Call this **before** building a payload, not merely inside `notify`: a
/// vanilla install with no extension should pay one lock read, not a walk of
/// every binder item in the project on every save.
pub fn has_listeners() -> bool {
    !LISTENERS
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .is_empty()
}

/// Deliver `event` to every listener, in registration order.
///
/// Never returns an error and never panics: a listener that panics is caught and
/// reported on stderr, and the remaining listeners still run. One broken
/// extension may not stop a project opening, closing or being saved.
pub(crate) fn notify(event: LifecycleEvent) {
    let listeners: Vec<(String, Arc<dyn LifecycleListener>)> = LISTENERS
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
        .map(|r| (r.namespace.clone(), r.listener.clone()))
        .collect();
    for (namespace, listener) in listeners {
        // `AssertUnwindSafe`: the listener is behind an `Arc` and this call
        // borrows nothing of ours that a panic could leave torn.
        let result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| listener.on_event(&event)));
        if result.is_err() {
            eprintln!("skribisto: lifecycle listener '{namespace}' panicked, skipping it");
        }
    }
    // Eviction last, so a listener above still saw the closing project's state.
    if let LifecycleEvent::Closed { unique_id } = &event {
        let slots: Vec<Arc<dyn ProjectSlots>> = EVICTIONS
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .map(|r| r.slots.clone())
            .collect();
        for s in slots {
            s.forget(unique_id);
        }
        // …and the "an extension changed something off-thread" generation with
        // them. A Work that is closing has either been written or has been
        // abandoned deliberately, and both are answers the writer has already
        // given, so there is nothing here to carry into the next project that
        // happens to reuse the id.
        crate::external_changes::forget(unique_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project_store::ProjectStore;
    use std::sync::Mutex;

    /// Records only events for **one** project.
    ///
    /// The registry is process-wide and tests run in parallel, so a recorder that
    /// kept everything would count a sibling test's `notify` as its own. Every
    /// test below therefore uses a uid it alone names.
    struct Recorder {
        uid: &'static str,
        seen: Mutex<Vec<LifecycleEvent>>,
    }
    impl Recorder {
        fn new(uid: &'static str) -> Arc<Self> {
            Arc::new(Self {
                uid,
                seen: Mutex::new(Vec::new()),
            })
        }
        fn count(&self) -> usize {
            self.seen.lock().unwrap().len()
        }
    }
    impl LifecycleListener for Recorder {
        fn on_event(&self, event: &LifecycleEvent) {
            let mine = match event {
                LifecycleEvent::Opened { unique_id, .. }
                | LifecycleEvent::Closed { unique_id }
                | LifecycleEvent::Saved { unique_id, .. } => unique_id == self.uid,
            };
            if mine {
                self.seen.lock().unwrap().push(event.clone());
            }
        }
    }

    struct Panicking;
    impl LifecycleListener for Panicking {
        fn on_event(&self, _event: &LifecycleEvent) {
            panic!("deliberately broken");
        }
    }

    fn closed(uid: &str) -> LifecycleEvent {
        LifecycleEvent::Closed {
            unique_id: uid.to_string(),
        }
    }

    #[test]
    fn a_listener_receives_events_and_drop_unregisters() {
        let rec = Recorder::new("uid-basic");
        {
            let _h = register("test.lc.basic", rec.clone());
            assert!(has_listeners());
            notify(closed("uid-basic"));
            assert_eq!(rec.count(), 1);
        }
        notify(closed("uid-basic"));
        assert_eq!(
            rec.count(),
            1,
            "a dropped handle must leave no listener behind"
        );
    }

    #[test]
    fn re_registering_a_namespace_replaces_rather_than_stacks() {
        let first = Recorder::new("uid-dup");
        let second = Recorder::new("uid-dup");
        let _a = register("test.lc.dup", first.clone());
        let _b = register("test.lc.dup", second.clone());
        notify(closed("uid-dup"));
        assert_eq!(first.count(), 0, "the earlier entry must be gone");
        assert_eq!(second.count(), 1);
    }

    /// One broken extension may not stop a project opening, closing or saving —
    /// nor silence another extension.
    #[test]
    fn a_panicking_listener_is_caught_and_the_rest_still_run() {
        let healthy = Recorder::new("uid-broken");
        let _broken = register("test.lc.broken", Arc::new(Panicking));
        let _ok = register("test.lc.healthy", healthy.clone());
        notify(closed("uid-broken"));
        assert_eq!(healthy.count(), 1);
    }

    /// **The leak, and the staler hazard behind it.** Without eviction every
    /// project a session opens stays resident, and re-opening one finds the old
    /// slot instead of loading from disk.
    #[test]
    fn closing_a_project_evicts_its_store_slot() {
        let store: ProjectStore<String> = ProjectStore::new();
        let a = store.scoped("uid-a").unwrap();
        let b = store.scoped("uid-b").unwrap();
        a.set("A's plan".into());
        b.set("B's plan".into());

        let _h = evict_on_close("test.lc.evict", Arc::new(store.clone()));
        notify(closed("uid-a"));

        assert!(!a.exists(), "the closed project's slot must be gone");
        assert_eq!(
            b.get().as_deref(),
            Some("B's plan"),
            "…and no other open project's may be touched"
        );
    }

    /// A listener must still see the closing project's own state — which is why
    /// eviction runs last.
    #[test]
    fn a_listener_still_sees_the_closing_projects_state() {
        /// Filtered to one project for the same reason [`Recorder`] is: `LISTENERS`
        /// is process-wide and tests run in parallel, so a sibling's `Closed`
        /// reaches this listener too — and an unfiltered one would overwrite
        /// `found` with that project's (absent) value.
        struct Peeker {
            uid: &'static str,
            store: ProjectStore<String>,
            found: Mutex<Option<String>>,
        }
        impl LifecycleListener for Peeker {
            fn on_event(&self, event: &LifecycleEvent) {
                let LifecycleEvent::Closed { unique_id } = event else {
                    return;
                };
                if unique_id != self.uid {
                    return;
                }
                let scope = self.store.scoped(unique_id).unwrap();
                *self.found.lock().unwrap() = scope.get();
            }
        }

        let store: ProjectStore<String> = ProjectStore::new();
        store.scoped("uid-peek").unwrap().set("still here".into());
        let peeker = Arc::new(Peeker {
            uid: "uid-peek",
            store: store.clone(),
            found: Mutex::new(None),
        });
        let _l = register("test.lc.peek", peeker.clone());
        let _e = evict_on_close("test.lc.peek.evict", Arc::new(store.clone()));

        notify(closed("uid-peek"));

        assert_eq!(peeker.found.lock().unwrap().as_deref(), Some("still here"));
        assert!(
            !store.scoped("uid-peek").unwrap().exists(),
            "…and it is gone afterwards all the same"
        );
    }

    /// Dropping the handle must take the eviction with it, or a store outlives
    /// its owner and keeps being swept.
    #[test]
    fn dropping_the_handle_also_drops_the_eviction() {
        let store: ProjectStore<String> = ProjectStore::new();
        let a = store.scoped("uid-gone").unwrap();
        {
            let _h = evict_on_close("test.lc.evict.drop", Arc::new(store.clone()));
        }
        a.set("kept".into());
        notify(closed("uid-gone"));
        assert!(a.exists(), "a dropped handle must leave no eviction behind");
    }

    /// `has_listeners` gates building an expensive payload — the uid walk over a
    /// whole manuscript — so it must answer about *listeners*, not about
    /// evictions. An eviction that counted would make a vanilla install pay for a
    /// payload nothing reads.
    ///
    /// Asserted by namespace, never by counting: sibling tests register and drop
    /// listeners of their own in parallel, so any assertion on a total is a flake
    /// waiting for a slow machine.
    #[test]
    fn an_eviction_alone_is_not_a_listener() {
        let store: ProjectStore<String> = ProjectStore::new();
        let _e = evict_on_close("test.lc.solo", Arc::new(store));

        let listening = LISTENERS
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .any(|r| r.namespace == "test.lc.solo");
        assert!(!listening, "an eviction must not register as a listener");
    }
}
