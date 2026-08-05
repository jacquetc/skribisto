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
///
/// It hashes the *serialized* form, so a change to any field's on-disk shape changes every
/// project's fingerprint once. `dict_language` becoming a list (format v4) did exactly that:
/// the first open after upgrading takes one backup of every project even where nothing was
/// edited. That is a one-off and arguably honest — the content really did change shape — but
/// it is a visible behaviour change in the backup layer, so it belongs in the release notes
/// rather than surprising someone reading skip-if-unchanged.
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
    // Asset *bytes* are absent from the serialised form by construction
    // (`WorkBundle::asset_bytes` is `#[serde(skip)]`), so nothing has to be
    // stripped here for them — and nothing may be added that would include
    // them. Each asset's `content_hash` is in its metadata row, so the bytes
    // are still covered: two projects differing only in an image's content
    // fingerprint differently, without this function ever seeing a pixel.
    // Backup marker: a fingerprint must be identical whether or not the bundle was
    // already stamped (Layer 2 computes it before `mark_as_backup`, but be robust).
    b.manifest.kind = BundleKind::Regular;
    b.manifest.backup_of = None;
    b.manifest.backup_created_at = None;
    // The read floor is a pure function of content already hashed elsewhere in this very
    // bundle, so stripping it loses no signal. Leaving it in would mean every future
    // refinement of `compute_min_read_version`'s scoring — with not one word of prose
    // edited — re-triggers a full backup cascade on the next save of every project: a
    // repeatable version of the one-off `dict_language` churn documented above.
    b.manifest.format_min_read_version = None;

    let w = &mut b.manifest.work;
    w.created_at.clear();
    w.updated_at.clear();
    // Nested under the Work rather than a top-level vector, so this is an
    // `if let` where its neighbours below are loops. The seven settings
    // themselves are real content and stay — only the timestamps are volatile.
    if let Some(sp) = &mut w.smart_punctuation {
        sp.created_at.clear();
        sp.updated_at.clear();
    }

    for t in &mut b.tags {
        t.created_at.clear();
        t.updated_at.clear();
    }
    for d in &mut b.dict_words {
        d.created_at.clear();
        d.updated_at.clear();
    }
    for r in &mut b.text_replacement_rules {
        r.created_at.clear();
        r.updated_at.clear();
    }
    // The body is real content and stays; only the bookkeeping stamps go. Without this
    // arm every save would fingerprint differently even when nothing changed, and
    // backup dedup ("skip if identical to the last one") would never skip again.
    for t in &mut b.note_templates {
        t.created_at.clear();
        t.updated_at.clear();
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
    // Comment bookkeeping timestamps are volatile; everything else about a comment
    // is content. Its BODY above all — but also `resolved` and the anchor payload,
    // since resolving a thread or re-anchoring it after an edit is a real change the
    // writer would expect a backup to capture. Miss these and skip-if-unchanged
    // silently drops a backup of genuine work.
    for c in &mut b.orphan_comments {
        strip_comment(c);
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
            for list in it.comments.values_mut() {
                for c in list {
                    strip_comment(c);
                }
            }
        }
    }
}

fn strip_comment(c: &mut crate::bundle::CommentFile) {
    c.created_at.clear();
    c.updated_at.clear();
    for r in &mut c.replies {
        r.created_at.clear();
        r.updated_at.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_bundle(title: &str, updated: &str) -> WorkBundle {
        WorkBundle {
            assets: Vec::new(),
            asset_bytes: Default::default(),
            manifest: ProjectManifest {
                format_version: FORMAT_VERSION,
                format_min_read_version: None,
                shape: ShapeTag::Zip,
                work: WorkFile {
                    file_id: 1,
                    created_at: "2020-01-01T00:00:00Z".into(),
                    updated_at: updated.into(),
                    title: title.into(),
                    author_name: "A".into(),
                    dict_language: vec!["en".to_string()],
                    tag_ids: vec![],
                    dict_word_ids: vec![],
                    unique_id: "uid-1".into(),
                    chapter_flat: false,
                    text_replacement_rule_ids: vec![],
                    custom_replacement_rules_enabled: false,
                    smart_punctuation: None,
                    number_chapters: true,
                    part_resets_chapter: false,
                },
                binder_order: vec![],
                kind: BundleKind::Regular,
                backup_of: None,
                backup_created_at: None,
            },
            tags: vec![],
            dict_words: vec![],
            note_templates: vec![],
            note_template_bodies: Default::default(),
            text_replacement_rules: vec![],
            trash_infos: vec![],
            paces: vec![],
            progress_snapshots: vec![],
            orphan_comments: vec![],
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
