// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The save hook: letting code outside this workspace put its own files into a
//! project bundle.
//!
//! [`skrib_format::carry`] stops a save from *destroying* files this build does
//! not model. That is enough for a build which only has to preserve someone
//! else's data, and not enough for one that has data of its own: a save
//! rebuilds the bundle from the store plus whatever
//! [`skrib_format::carry::load`] found **on disk**, so an extension holding
//! live state in memory would watch every save faithfully write back the stale
//! copy it read a moment earlier.
//!
//! So a contributor is asked, at write time, what it wants in the bundle.
//!
//! ## Why a registry rather than a parameter
//!
//! Threading a map through `save_work` / `save_as` / `backup_now` means putting
//! it in their DTOs, and those are Qleany-generated from `qleany.yaml` — so the
//! core manifest would have to describe a facility only an out-of-tree crate
//! ever uses. An extension is a process-wide fact established once at startup,
//! not a per-call argument, and registering it reads that way.
//!
//! ## Two rules, both in the manuscript's favour
//!
//! 1. **A contributor cannot touch a modelled path.** Anything
//!    [`skrib_format::carry::is_modelled`] recognises — the manifest, prose
//!    blobs, sidecars, assets — is refused. An extension may add to a project;
//!    it may never rewrite the book.
//! 2. **A contributor can never fail a save.** An error, or a refused path, is
//!    reported on stderr and skipped. The alternative is a broken extension
//!    that stops a writer saving their manuscript, which is precisely the
//!    dependency the edition split exists to avoid.
//!
//! ## Layering
//!
//! This is the **backend** half of the extension seam, and is deliberately not
//! reachable through the UI-side extension trait: that trait composes tabs,
//! commands, panes and locales, and has no business in a save path it does not
//! run on. An extension's backend crate registers here directly, at startup,
//! next to wherever it builds its own store.

use std::collections::BTreeMap;
use std::sync::{Arc, LazyLock, RwLock};

/// Something that wants files of its own written into project bundles.
pub trait BundleContributor: Send + Sync {
    /// Files to write into the bundle for the project identified by
    /// `work_unique_id`, keyed by bundle-relative path (`/`-separated, e.g.
    /// `"structure/beats.ron"`).
    ///
    /// Called on **every** write — save, save-as and backup alike — so it must
    /// be cheap and must not block. Return an empty map for a project this
    /// contributor knows nothing about; returning data for the wrong project is
    /// how one project's plan ends up inside another's file.
    fn files(&self, work_unique_id: &str) -> anyhow::Result<BTreeMap<String, Vec<u8>>>;
}

struct Registration {
    namespace: String,
    contributor: Arc<dyn BundleContributor>,
}

static REGISTRY: LazyLock<RwLock<Vec<Registration>>> = LazyLock::new(|| RwLock::new(Vec::new()));

/// Registered contributors, in registration order.
///
/// `namespace` identifies the contributor, not the files it writes: registering
/// the same namespace twice **replaces** the earlier entry rather than stacking
/// a second copy, so a re-registration (a test, a re-initialised extension)
/// cannot quietly double-write.
///
/// The returned [`ContributorHandle`] unregisters on drop. Hold it for as long
/// as the contributor should be live — for an application, that is the whole
/// process; for a test, the test.
pub fn register(
    namespace: impl Into<String>,
    contributor: Arc<dyn BundleContributor>,
) -> ContributorHandle {
    let namespace = namespace.into();
    let mut reg = REGISTRY.write().unwrap_or_else(|e| e.into_inner());
    reg.retain(|r| r.namespace != namespace);
    reg.push(Registration {
        namespace: namespace.clone(),
        contributor,
    });
    ContributorHandle { namespace }
}

/// Unregisters its contributor when dropped.
pub struct ContributorHandle {
    namespace: String,
}

impl Drop for ContributorHandle {
    fn drop(&mut self) {
        let mut reg = REGISTRY.write().unwrap_or_else(|e| e.into_inner());
        reg.retain(|r| r.namespace != self.namespace);
    }
}

/// Everything the registered contributors want written for this project.
///
/// Never returns an error and never panics: a contributor that fails, or that
/// claims a path belonging to the manuscript, is skipped with a message on
/// stderr. See the module docs for why a save must survive a bad extension.
pub(crate) fn collect(work_unique_id: &str) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    let reg = REGISTRY.read().unwrap_or_else(|e| e.into_inner());
    for r in reg.iter() {
        let files = match r.contributor.files(work_unique_id) {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "skribisto: bundle contributor '{}' failed, skipping its files: {e}",
                    r.namespace
                );
                continue;
            }
        };
        for (path, bytes) in files {
            if skrib_format::carry::is_modelled(&path) {
                eprintln!(
                    "skribisto: bundle contributor '{}' tried to write '{path}', which belongs \
                     to the manuscript — refused",
                    r.namespace
                );
                continue;
            }
            out.insert(path, bytes);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixed(Vec<(&'static str, &'static [u8])>);
    impl BundleContributor for Fixed {
        fn files(&self, _work: &str) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
            Ok(self
                .0
                .iter()
                .map(|(p, b)| ((*p).to_string(), b.to_vec()))
                .collect())
        }
    }

    struct Broken;
    impl BundleContributor for Broken {
        fn files(&self, _work: &str) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
            Err(anyhow::anyhow!("deliberately broken"))
        }
    }

    #[test]
    fn a_contributor_may_not_overwrite_the_manuscript() {
        // The rule that makes this hook safe to expose at all. Both paths are
        // ones a careless extension could plausibly pick.
        let _h = register(
            "test.manuscript-guard",
            Arc::new(Fixed(vec![
                ("project.skrib", b"hijacked"),
                ("guard/ok.ron", b"fine"),
            ])),
        );
        let got = collect("uid-guard");
        assert!(
            !got.contains_key("project.skrib"),
            "an extension must never be able to rewrite the manifest"
        );
        assert_eq!(got.get("guard/ok.ron").map(Vec::as_slice), Some(&b"fine"[..]));
    }

    #[test]
    fn a_failing_contributor_does_not_break_the_save() {
        let _broken = register("test.broken", Arc::new(Broken));
        let _ok = register(
            "test.still-works",
            Arc::new(Fixed(vec![("broken/ok.ron", b"x")])),
        );
        // No panic, no error type at all — and the healthy contributor still
        // gets its file in, because one bad extension may not silence another.
        assert!(collect("uid-broken").contains_key("broken/ok.ron"));
    }

    #[test]
    fn re_registering_a_namespace_replaces_rather_than_stacks() {
        let _first = register("test.dup", Arc::new(Fixed(vec![("dup/a.ron", b"first")])));
        let _second = register("test.dup", Arc::new(Fixed(vec![("dup/a.ron", b"second")])));
        assert_eq!(
            collect("uid-dup").get("dup/a.ron").map(Vec::as_slice),
            Some(&b"second"[..]),
            "the later registration must win outright, not merge with the earlier"
        );
    }

    #[test]
    fn dropping_the_handle_unregisters() {
        {
            let _h = register("test.scoped", Arc::new(Fixed(vec![("scoped/a.ron", b"y")])));
            assert!(collect("uid-scoped").contains_key("scoped/a.ron"));
        }
        assert!(
            !collect("uid-scoped").contains_key("scoped/a.ron"),
            "a dropped handle must leave no contributor behind"
        );
    }
}
