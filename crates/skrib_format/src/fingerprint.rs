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

    // Metadata only (bytes are `#[serde(skip)]`, see `WorkBundle::asset_bytes`'s own
    // note above) — but the bookkeeping stamps are exactly as volatile as every other
    // entity's, and this loop was missing entirely: an asset's `updated_at` alone
    // ticking (e.g. a re-import that reuses the same `content_hash`) used to change
    // the fingerprint with nothing about the *content* having changed.
    for a in &mut b.assets {
        a.created_at.clear();
        a.updated_at.clear();
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
    // The exact sibling of `orphan_comments` above, for the same reason: a footnote
    // whose annotated `Content` was purged moves to this bundle-root orphanage rather
    // than being dropped (see `WorkBundle::orphan_footnotes`'s own doc), and its
    // `created_at`/`updated_at` are bookkeeping, not content, like every other
    // timestamped row this function strips. This loop was the one omission in an
    // otherwise-exhaustive function: a footnote's `updated_at` alone advancing (an
    // edit that leaves label/body byte-identical, e.g. type-then-backspace) used to
    // fingerprint as a changed project and trigger a fully redundant backup.
    for f in &mut b.orphan_footnotes {
        strip_footnote(f);
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
            // `it.comments`'s exact sibling (line above) — same map shape, same
            // reason: anchored, not orphaned, so it lives per-item rather than at
            // the bundle root, but its timestamps are just as volatile.
            for list in it.footnotes.values_mut() {
                for f in list {
                    strip_footnote(f);
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

fn strip_footnote(f: &mut crate::bundle::FootnoteFile) {
    f.created_at.clear();
    f.updated_at.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

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
            orphan_footnotes: Vec::new(),
            history: Default::default(),
            binders: vec![],
            carried: Default::default(),
        }
    }

    /// The history log must be invisible to the fingerprint.
    ///
    /// This is the load-bearing guard on the whole feature's cost. The log grows on
    /// every save; if it reached the hash, every save would change the fingerprint,
    /// `skip_if_unchanged` would never skip again, and every close would write a
    /// full backup of a project nobody had edited. `strip_volatile`'s own comment
    /// states the rule for `asset_bytes` — *"nothing may be added that would include
    /// them"* — and this asserts the newer field obeys it.
    #[test]
    fn the_history_log_never_reaches_the_fingerprint() {
        use crate::history::{HistoryEntry, HistoryLog};

        let plain = minimal_bundle("Novel", "2026-08-07T10:00:00Z");
        let baseline = content_fingerprint(&plain);

        let mut with_history = minimal_bundle("Novel", "2026-08-07T10:00:00Z");
        with_history.history = HistoryLog {
            entries: vec![HistoryEntry {
                at: "2026-08-07T09:00:00Z".into(),
                item_uid: uuid::Uuid::from_u128(7),
                role: common::entities::ContentRole::SceneText,
                hash: "deadbeef".into(),
                bytes: 8,
            }],
            blobs: [("deadbeef".to_string(), "some past prose".to_string())]
                .into_iter()
                .collect(),
        };

        assert_eq!(
            content_fingerprint(&with_history),
            baseline,
            "a bundle that only differs by its history log must fingerprint identically, \
             or skip-if-unchanged stops skipping and every close writes a full backup",
        );
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

    fn footnote(updated: &str, label: &str, body: &str) -> FootnoteFile {
        FootnoteFile {
            file_id: 1,
            created_at: "2020-01-01T00:00:00Z".into(),
            updated_at: updated.into(),
            label: label.into(),
            body: body.into(),
        }
    }

    fn asset(updated: &str, hash: &str) -> AssetFile {
        AssetFile {
            file_id: 1,
            created_at: "2020-01-01T00:00:00Z".into(),
            updated_at: updated.into(),
            content_hash: hash.into(),
            file_name: "cover.png".into(),
            mime_type: "image/png".into(),
            width: 100,
            height: 100,
            byte_size: 1234,
            alt: String::new(),
            is_cover: false,
            path: "assets/abc123.png".into(),
        }
    }

    /// One item carrying a single anchored footnote, keyed under content `file_id`
    /// 42 — the shape `strip_volatile`'s `it.footnotes` loop walks.
    fn item_with_footnote(footnote: FootnoteFile) -> BundledItem {
        BundledItem {
            item: BinderItemFile {
                file_id: 10,
                uid: uuid::Uuid::nil(),
                created_at: "2020-01-01T00:00:00Z".into(),
                updated_at: "2020-01-01T00:00:00Z".into(),
                title: "Scene".into(),
                sub_title: String::new(),
                role: common::entities::BinderItemRole::Item,
                sub_role: common::entities::BinderItemSubRole::Scene,
                label: String::new(),
                activated: true,
                is_favorite: false,
                is_exportable: true,
                exclude_from_numbering: false,
                indent: 0,
                word_count_goal: 0,
                char_count_goal: 0,
                dict_language: vec![],
                aliases: vec![],
                inline_contents: vec![],
                prose_refs: vec![],
                reference_ids: vec![],
                point_of_view_ids: vec![],
                tag_ids: vec![],
            },
            prose: BTreeMap::new(),
            comments: BTreeMap::new(),
            footnotes: BTreeMap::from([(42u64, vec![footnote])]),
        }
    }

    /// **Regression for the gap this module's own doc names**: a footnote's
    /// `updated_at` moving alone — orphaned, anchored, and an asset's, all three
    /// shapes `strip_volatile` touches — must not change the fingerprint.
    #[test]
    fn a_footnotes_or_assets_timestamp_alone_does_not_change_the_fingerprint() {
        let mut a = minimal_bundle("Novel", "2026-01-01T10:00:00Z");
        a.orphan_footnotes = vec![footnote("2026-01-01T10:00:00Z", "fn1", "An orphaned note.")];
        a.assets = vec![asset("2026-01-01T10:00:00Z", "abc123")];
        a.binders = vec![BundledBinder {
            binder: BinderFile {
                file_id: 1,
                uid: uuid::Uuid::nil(),
                created_at: "2020-01-01T00:00:00Z".into(),
                updated_at: "2020-01-01T00:00:00Z".into(),
                name: "Manuscript".into(),
                activated: true,
                item_order: vec![10],
            },
            items: vec![item_with_footnote(footnote(
                "2026-01-01T10:00:00Z",
                "fn2",
                "An anchored note.",
            ))],
        }];

        let mut b = a.clone();
        // Every timestamp this test cares about moves — none of the content does.
        b.orphan_footnotes[0].updated_at = "2026-06-15T00:00:00Z".into();
        b.assets[0].updated_at = "2026-06-15T00:00:00Z".into();
        b.binders[0].items[0].footnotes.get_mut(&42).unwrap()[0].updated_at =
            "2026-06-15T00:00:00Z".into();

        assert_eq!(
            content_fingerprint(&a),
            content_fingerprint(&b),
            "a bookkeeping timestamp alone must not change the fingerprint \
             (backup_now's skip-if-unchanged gate would otherwise misfire)"
        );
    }

    /// The positive control for the test above: an orphaned note's actual words
    /// changing DOES have to move the fingerprint, or a real edit would be silently
    /// skipped by the same gate.
    #[test]
    fn a_footnotes_body_changing_does_change_the_fingerprint() {
        let mut a = minimal_bundle("Novel", "2026-01-01T10:00:00Z");
        a.orphan_footnotes = vec![footnote("2026-01-01T10:00:00Z", "fn1", "Original words.")];

        let mut b = a.clone();
        b.orphan_footnotes[0].body = "Rewritten words.".into();

        assert_ne!(content_fingerprint(&a), content_fingerprint(&b));
    }
}
