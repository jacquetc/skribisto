// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A stable, content-only fingerprint of a [`WorkBundle`].
//!
//! Used for skip-if-unchanged: two backups of byte-identical *content* taken
//! seconds apart must fingerprint identically, so per-destination dedup can skip
//! the redundant write. All volatile-but-content-irrelevant fields (entity
//! `created_at`/`updated_at`/`trashed_at`, and any backup marker) are neutralised
//! before hashing. blake3 is chosen for speed + cross-run determinism (unlike
//! `std::hash::DefaultHasher`, whose algorithm is not a stability guarantee).

use super::bundle::*;

/// Content fingerprint (blake3 hex) of `bundle`, insensitive to timestamps and
/// to whether the bundle has been marked as a backup.
pub fn content_fingerprint(bundle: &WorkBundle) -> String {
    let mut b = bundle.clone();
    strip_volatile(&mut b);
    // In-memory serialization of our own types does not realistically fail; the
    // Debug fallback stays deterministic (same stripped content) if it ever does,
    // so this never panics and never conflates different content.
    let repr = ron::ser::to_string(&b).unwrap_or_else(|_| format!("{b:?}"));
    blake3::hash(repr.as_bytes()).to_hex().to_string()
}

/// Zero every timestamp and backup marker so only content participates in the hash.
fn strip_volatile(b: &mut WorkBundle) {
    // Backup marker: a fingerprint must be identical whether or not the bundle was
    // already stamped (Layer 2 computes it before `mark_as_backup`, but be robust).
    b.manifest.kind = BundleKind::Regular;
    b.manifest.backup_of = None;
    b.manifest.backup_created_at = None;

    let w = &mut b.manifest.work;
    w.created_at.clear();
    w.updated_at.clear();

    for t in &mut b.tags {
        t.created_at.clear();
        t.updated_at.clear();
    }
    for d in &mut b.dict_words {
        d.created_at.clear();
        d.updated_at.clear();
    }
    for ti in &mut b.trash_infos {
        ti.created_at.clear();
        ti.updated_at.clear();
        ti.trashed_at.clear();
    }
    // Only the bookkeeping timestamps are volatile — a Pace's plan dates (start/end/
    // holiday/milestone dates) are CONTENT, so a changed deadline must change the
    // fingerprint (else skip-if-unchanged would drop a backup of a real edit).
    for p in &mut b.paces {
        p.created_at.clear();
        p.updated_at.clear();
        for h in &mut p.holidays {
            h.created_at.clear();
            h.updated_at.clear();
        }
        for ms in &mut p.milestones {
            ms.created_at.clear();
            ms.updated_at.clear();
        }
    }
    // `day` is NOT stripped — for a snapshot the day IS the content, so two different
    // days with the same total must not fingerprint identically (that would make backup
    // skip a genuinely changed day).
    for s in &mut b.progress_snapshots {
        s.created_at.clear();
        s.updated_at.clear();
    }
    for bb in &mut b.binders {
        bb.binder.created_at.clear();
        bb.binder.updated_at.clear();
        for it in &mut bb.items {
            it.item.created_at.clear();
            it.item.updated_at.clear();
            for ic in &mut it.item.inline_contents {
                ic.created_at.clear();
                ic.updated_at.clear();
            }
            for pr in &mut it.item.prose_refs {
                pr.created_at.clear();
                pr.updated_at.clear();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_bundle(title: &str, updated: &str) -> WorkBundle {
        WorkBundle {
            manifest: ProjectManifest {
                format_version: FORMAT_VERSION,
                shape: ShapeTag::Zip,
                work: WorkFile {
                    file_id: 1,
                    created_at: "2020-01-01T00:00:00Z".into(),
                    updated_at: updated.into(),
                    title: title.into(),
                    author_name: "A".into(),
                    dict_language: "en".into(),
                    tag_ids: vec![],
                    dict_word_ids: vec![],
                    unique_id: "uid-1".into(),
                    chapter_flat: false,
                },
                binder_order: vec![],
                kind: BundleKind::Regular,
                backup_of: None,
                backup_created_at: None,
            },
            tags: vec![],
            dict_words: vec![],
            trash_infos: vec![],
            paces: vec![],
            progress_snapshots: vec![],
            binders: vec![],
        }
    }

    #[test]
    fn same_content_different_timestamps_and_marker_fingerprints_equally() {
        let a = minimal_bundle("Novel", "2026-01-01T10:00:00Z");
        let mut b = minimal_bundle("Novel", "2026-02-02T22:22:22Z");
        // Marking b as a backup must not change the fingerprint.
        b.manifest.kind = BundleKind::Backup;
        b.manifest.backup_of = Some("/x.skrib".into());
        b.manifest.backup_created_at = Some("2026-02-02T22:22:22Z".into());
        assert_eq!(content_fingerprint(&a), content_fingerprint(&b));
    }

    #[test]
    fn different_content_fingerprints_differently() {
        let a = minimal_bundle("Novel", "2026-01-01T10:00:00Z");
        let b = minimal_bundle("Different Title", "2026-01-01T10:00:00Z");
        assert_ne!(content_fingerprint(&a), content_fingerprint(&b));
    }
}
