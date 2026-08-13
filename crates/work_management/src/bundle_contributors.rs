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
//!
//! ## What a contributor is told
//!
//! [`SaveContext`](crate::bundle_contributors::SaveContext) — which project,
//! *which kind of write*, and a fingerprint of the manuscript. The last two exist
//! because "write a file into the bundle" and "notice that the book changed" are
//! different questions, and a contributor that had to answer the second from the
//! first could only do it by re-reading the project off disk on every save.
//!
//! [`manuscript_fingerprint`](crate::bundle_contributors::manuscript_fingerprint)
//! excludes `carried` outright, which is what closes the obvious circularity: a
//! contributor's own output is in `carried`, so a contributor whose bytes differ
//! on every save cannot move the number it is judging the book by. Without that
//! exclusion no two consecutive saves of an untouched manuscript would ever
//! agree, and every downstream "has this changed?" would answer yes forever.
//!
//! It is a free function, not a private step inside the write path, because a
//! contributor keeping a record across sessions has to ask the same question of
//! a bundle read straight off disk when a project *opens*. Two implementations of
//! "the manuscript's fingerprint" would disagree for reasons nobody could see.

use std::collections::BTreeMap;
use std::sync::{Arc, LazyLock, RwLock};

/// What a save tells its contributors about the write in progress.
///
/// Plain public fields and no `#[non_exhaustive]`, deliberately: a contributor
/// living outside this workspace has to be able to build one in its own unit
/// tests, and a hidden constructor would push every such test onto a real save.
/// Adding a field here is therefore a breaking change downstream, which is the
/// intended behaviour — the seam-drift build is what surfaces it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveContext {
    /// The project being written. Never an entity id: those are re-minted on
    /// every load, and a contributor's state outlives one.
    pub work_unique_id: String,
    /// Which write path asked. A contributor that records history wants
    /// [`SaveKind::Save`](crate::lifecycle::SaveKind::Save) only: a backup is a
    /// copy of a state the writer already reached, and a save-as is the same
    /// manuscript under a new name.
    pub kind: crate::lifecycle::SaveKind,
    /// blake3 hex of the manuscript, from
    /// [`manuscript_fingerprint`]
    /// — carried files excluded, so no contributor's own output is in it,
    /// including this one's.
    ///
    /// Equal across two writes of an unchanged manuscript and different after
    /// any edit, so a contributor can answer "did the book change since I last
    /// looked?" without re-reading the project.
    ///
    /// Not a substitute for the format's own
    /// [`content_fingerprint`](skrib_format::content_fingerprint): that one
    /// covers the whole bundle, carried files included, which is what backup
    /// skip-if-unchanged needs and what this deliberately is not.
    pub manuscript_fingerprint: String,
}

impl SaveContext {
    /// A context naming one project and a plain in-place save, **for testing a
    /// contributor in isolation**.
    ///
    /// [`Self::manuscript_fingerprint`] is empty, which no real write ever
    /// produces: a contributor that reads it should set the field rather than
    /// assert on this. What this exists for is the far commoner case of a
    /// contributor that only looks at the project id, whose tests would
    /// otherwise all hand-write the same three fields and all break together
    /// the day a fourth is added.
    ///
    /// ```
    /// # use work_management::bundle_contributors::SaveContext;
    /// # use work_management::lifecycle::SaveKind;
    /// let backup = SaveContext {
    ///     kind: SaveKind::Backup,
    ///     ..SaveContext::for_project("some-project-uid")
    /// };
    /// assert_eq!(backup.work_unique_id, "some-project-uid");
    /// ```
    pub fn for_project(work_unique_id: impl Into<String>) -> Self {
        Self {
            work_unique_id: work_unique_id.into(),
            kind: crate::lifecycle::SaveKind::Save,
            manuscript_fingerprint: String::new(),
        }
    }
}

/// Something that wants files of its own written into project bundles.
pub trait BundleContributor: Send + Sync {
    /// Files to write into the bundle for the project [`SaveContext`] names,
    /// keyed by bundle-relative path (`/`-separated, e.g.
    /// `"structure/beats.ron"`).
    ///
    /// Called on **every** write — save, save-as and backup alike, which is what
    /// [`SaveContext::kind`] distinguishes — so it must be cheap and must not
    /// block. Return an empty map for a project this contributor knows nothing
    /// about; returning data for the wrong project is how one project's plan
    /// ends up inside another's file.
    fn files(&self, ctx: &SaveContext) -> anyhow::Result<BTreeMap<String, Vec<u8>>>;
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

/// Whether anything is registered at all.
///
/// Mirrors [`crate::lifecycle::has_listeners`], and exists for the same reason:
/// it is the cheap question a write path asks before paying for anything a
/// vanilla build has no use for. The crate-internal `collect` applies it itself,
/// so no write path has to remember to.
pub fn has_contributors() -> bool {
    !REGISTRY
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .is_empty()
}

/// The manuscript's fingerprint: what a save puts in
/// [`SaveContext::manuscript_fingerprint`], as a function anything can call.
///
/// Public because the save is not the only place the question comes up. A
/// contributor that keeps a record across sessions has to ask it again when a
/// project *opens* — against the bundle
/// [`skrib_format::read_bundle`] just returned — to
/// find out whether the book moved while this build was not watching. Two
/// separate implementations of "the manuscript's fingerprint" would answer
/// differently for reasons nobody could see, so there is one.
///
/// **`carried` is excluded and `manifest.shape` is canonicalised.** The first is
/// what makes it *the manuscript* rather than the bundle: a contributor's own
/// bytes, and any file this build does not model, are not the book. The second
/// is because `backup_now_uc` writes every backup as a zip whatever the project
/// is, and a shape conversion is not an edit.
///
/// Equal across two writes of an unchanged manuscript, across a save and a
/// backup of it, and across closing the project and opening it again — that last
/// one despite every `file_id` in the bundle being a store id re-minted by each
/// load. `save_load_test::the_fingerprint_survives_a_close_and_a_reopen` is what
/// holds that.
pub fn manuscript_fingerprint(bundle: &skrib_format::WorkBundle) -> String {
    let mut manuscript = bundle.clone();
    manuscript.carried.clear();
    manuscript.manifest.shape = skrib_format::ShapeTag::Zip;
    skrib_format::content_fingerprint(&manuscript)
}

/// Everything the registered contributors want written for this project.
///
/// Called with the bundle as the store produced it, before `carry::load` — not
/// because the fingerprint depends on it (`manuscript_fingerprint` drops
/// `carried` whenever it is asked) but because cloning a bundle that has not yet
/// picked up a project's unmodelled files is cheaper, and the write paths have
/// nothing to gain from asking later.
///
/// Never returns an error and never panics: a contributor that fails, or that
/// claims a path belonging to the manuscript, is skipped with a message on
/// stderr. See the module docs for why a save must survive a bad extension.
pub(crate) fn collect(
    bundle: &skrib_format::WorkBundle,
    work_unique_id: &str,
    kind: crate::lifecycle::SaveKind,
) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    // The gate, and it is load-bearing rather than tidy: the fingerprint below
    // clones and RON-serialises the whole manuscript, so a build with no
    // extension in it must pay one lock read for this hook and not a hash pass
    // on every keystroke-triggered autosave.
    if !has_contributors() {
        return out;
    }

    // Scoped so the copy `manuscript_fingerprint` makes is gone before any
    // extension code runs.
    let ctx = SaveContext {
        work_unique_id: work_unique_id.to_string(),
        kind,
        manuscript_fingerprint: manuscript_fingerprint(bundle),
    };

    let reg = REGISTRY.read().unwrap_or_else(|e| e.into_inner());
    for r in reg.iter() {
        let files = match r.contributor.files(&ctx) {
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
    use crate::lifecycle::SaveKind;

    struct Fixed(Vec<(&'static str, &'static [u8])>);
    impl BundleContributor for Fixed {
        fn files(&self, _ctx: &SaveContext) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
            Ok(self
                .0
                .iter()
                .map(|(p, b)| ((*p).to_string(), b.to_vec()))
                .collect())
        }
    }

    struct Broken;
    impl BundleContributor for Broken {
        fn files(&self, _ctx: &SaveContext) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
            Err(anyhow::anyhow!("deliberately broken"))
        }
    }

    /// Writes down what it was told, so a test can assert on the context rather
    /// than only on the files that came back.
    ///
    /// **Scoped to one project id**, and that is not tidiness: the registry is
    /// process-wide and these tests run in parallel, so a recorder answering for
    /// every project interleaves every sibling test's writes into its own
    /// transcript. That is exactly how the first draft of these tests failed.
    struct Recording {
        uid: &'static str,
        seen: std::sync::Mutex<Vec<SaveContext>>,
    }
    impl Recording {
        fn watching(uid: &'static str) -> Arc<Self> {
            Arc::new(Self {
                uid,
                seen: std::sync::Mutex::new(Vec::new()),
            })
        }
        fn seen(&self) -> Vec<SaveContext> {
            self.seen.lock().unwrap_or_else(|e| e.into_inner()).clone()
        }
        fn fingerprints(&self) -> Vec<String> {
            self.seen()
                .into_iter()
                .map(|c| c.manuscript_fingerprint)
                .collect()
        }
    }
    impl BundleContributor for Recording {
        fn files(&self, ctx: &SaveContext) -> anyhow::Result<BTreeMap<String, Vec<u8>>> {
            if ctx.work_unique_id == self.uid {
                self.seen
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .push(ctx.clone());
            }
            Ok(BTreeMap::new())
        }
    }

    /// A manuscript, with the title standing in for "the writer edited
    /// something". The end-to-end behaviour is covered in `save_load_test`;
    /// what these tests need is a bundle whose content they can vary by hand.
    fn bundle(title: &str) -> skrib_format::WorkBundle {
        let mut b = crate::save_load_test::sample_bundle();
        b.manifest.work.title = title.to_string();
        b
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
        let got = collect(&bundle("guard"), "uid-guard", SaveKind::Save);
        assert!(
            !got.contains_key("project.skrib"),
            "an extension must never be able to rewrite the manifest"
        );
        assert_eq!(
            got.get("guard/ok.ron").map(Vec::as_slice),
            Some(&b"fine"[..])
        );
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
        assert!(
            collect(&bundle("broken"), "uid-broken", SaveKind::Save).contains_key("broken/ok.ron")
        );
    }

    #[test]
    fn re_registering_a_namespace_replaces_rather_than_stacks() {
        let _first = register("test.dup", Arc::new(Fixed(vec![("dup/a.ron", b"first")])));
        let _second = register("test.dup", Arc::new(Fixed(vec![("dup/a.ron", b"second")])));
        assert_eq!(
            collect(&bundle("dup"), "uid-dup", SaveKind::Save)
                .get("dup/a.ron")
                .map(Vec::as_slice),
            Some(&b"second"[..]),
            "the later registration must win outright, not merge with the earlier"
        );
    }

    #[test]
    fn dropping_the_handle_unregisters() {
        {
            let _h = register("test.scoped", Arc::new(Fixed(vec![("scoped/a.ron", b"y")])));
            assert!(
                collect(&bundle("scoped"), "uid-scoped", SaveKind::Save)
                    .contains_key("scoped/a.ron")
            );
        }
        assert!(
            !collect(&bundle("scoped"), "uid-scoped", SaveKind::Save).contains_key("scoped/a.ron"),
            "a dropped handle must leave no contributor behind"
        );
    }

    /// The gate every write path leans on: a stale `false` silently drops an
    /// extension's data from the bundle, and a stale `true` costs a vanilla
    /// build a hash pass per save.
    ///
    /// ⚠ **Only the `true` direction is assertable here, and the first cut of
    /// this test got that wrong.** It read the predicate before registering and
    /// required the same answer after dropping — which a sibling test
    /// registering or dropping in the window between the two reads makes false,
    /// because the registry is process-wide and these run in parallel. It
    /// flaked, roughly once in twenty full runs, in a way that looked like
    /// `has_contributors` itself was broken.
    ///
    /// The other direction is not lost: `dropping_the_handle_unregisters` above
    /// proves it through `collect`, scoped to a uid that test alone owns, which
    /// is the only way to ask the question without asking about everyone else's
    /// contributors too.
    #[test]
    fn has_contributors_sees_a_live_registration() {
        let _h = register("test.gate", Arc::new(Fixed(vec![("gate/a.ron", b"x")])));
        assert!(has_contributors(), "a live registration must be visible");
    }

    /// The kind is the whole reason a contributor can tell "the writer saved"
    /// from "a scheduled backup ran", which is the difference between recording
    /// a revision and recording the same revision three more times.
    #[test]
    fn the_context_carries_the_project_and_the_kind_through_unchanged() {
        let rec = Recording::watching("uid-ctx");
        let _h = register("test.ctx", rec.clone());
        for kind in [SaveKind::Save, SaveKind::SaveAs, SaveKind::Backup] {
            let _ = collect(&bundle("ctx"), "uid-ctx", kind);
        }
        let seen = rec.seen();
        assert_eq!(
            seen.iter().map(|c| c.kind).collect::<Vec<_>>(),
            vec![SaveKind::Save, SaveKind::SaveAs, SaveKind::Backup],
        );
        assert!(seen.iter().all(|c| c.work_unique_id == "uid-ctx"));
        assert!(
            seen.iter().all(|c| !c.manuscript_fingerprint.is_empty()),
            "every context carries a fingerprint, whatever the write path"
        );
    }

    /// Same manuscript, same string; one edit, a different one. Everything a
    /// contributor can conclude from this field rests on both halves.
    #[test]
    fn the_fingerprint_is_stable_across_writes_and_moves_with_the_manuscript() {
        let rec = Recording::watching("uid-fp");
        let _h = register("test.fp", rec.clone());
        let _ = collect(&bundle("Chapter one"), "uid-fp", SaveKind::Save);
        let _ = collect(&bundle("Chapter one"), "uid-fp", SaveKind::Save);
        let _ = collect(&bundle("Chapter two"), "uid-fp", SaveKind::Save);
        let f = rec.fingerprints();
        assert_eq!(f[0], f[1], "an unchanged manuscript must fingerprint alike");
        assert_ne!(f[1], f[2], "an edited manuscript must fingerprint anew");
    }

    /// **The shape is not content.** A backup is always written as a zip, so
    /// without canonicalising it a folder project's save and its own backup
    /// disagree about a manuscript neither of them touched.
    #[test]
    fn the_shape_does_not_reach_the_fingerprint() {
        let rec = Recording::watching("uid-shape");
        let _h = register("test.shape", rec.clone());
        let mut folder = bundle("same book");
        folder.manifest.shape = skrib_format::ShapeTag::Folder;
        let mut zip = bundle("same book");
        zip.manifest.shape = skrib_format::ShapeTag::Zip;
        let _ = collect(&folder, "uid-shape", SaveKind::Save);
        let _ = collect(&zip, "uid-shape", SaveKind::Backup);
        let f = rec.fingerprints();
        assert_eq!(
            f[0], f[1],
            "converting a project between shapes is not an edit to the book"
        );
    }

    /// **The circularity, closed by exclusion rather than by call order.**
    ///
    /// An extension's own bytes live in `carried`, and the format's
    /// `content_fingerprint` sees them on purpose — `CarriedFile` serialises its
    /// digest so backup skip-if-unchanged cannot skip a project whose only
    /// change is an extension's data. Feed that number back to the extension as
    /// "did the book change?" and a contributor writing different bytes every
    /// save makes every save look like an edit the writer never made, forever,
    /// with nothing downstream able to tell it apart from typing.
    ///
    /// So the manuscript fingerprint drops `carried` whenever it is asked, and
    /// the property holds wherever the question is put — mid-save, at open, or
    /// from a test like this one.
    #[test]
    fn a_contributors_own_output_cannot_reach_the_fingerprint() {
        let clean = bundle("untouched");
        let mut merged = clean.clone();
        merged.carried.insert(
            "ext/churn.ron".to_string(),
            skrib_format::CarriedFile::new(b"(call: 1)".to_vec()),
        );
        let mut again = clean.clone();
        again.carried.insert(
            "ext/churn.ron".to_string(),
            skrib_format::CarriedFile::new(b"(call: 2)".to_vec()),
        );

        assert_eq!(
            manuscript_fingerprint(&clean),
            manuscript_fingerprint(&merged),
            "an extension's file is not part of the book"
        );
        assert_eq!(
            manuscript_fingerprint(&merged),
            manuscript_fingerprint(&again),
            "…and neither is a changed one"
        );
        // The format's own hash still sees it, which is the property backups need
        // and the reason these two must be different functions.
        assert_ne!(
            skrib_format::content_fingerprint(&clean),
            skrib_format::content_fingerprint(&merged),
            "backup skip-if-unchanged must never skip a project whose extension data moved"
        );
    }
}
