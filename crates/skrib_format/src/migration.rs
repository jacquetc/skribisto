// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Forward migration chain for the on-disk format, keyed on `format_version`.
//!
//! v1 → v2 added `WorkFile.unique_id`. That change is purely additive and RON is
//! name-keyed, so a v1 manifest deserializes fine via `#[serde(default)]` (empty
//! id) and the load path mints a fresh id when it's empty (see
//! `load_work_uc::materialize`) — no structural transform step was required.
//!
//! v2 → v3 added `BinderFile.uid` / `BinderItemFile.uid`. Also additive, but it
//! cannot be healed the same way: `materialize` heals ONE work-level id, while
//! this needs a fresh id per row, and every row must get one before anything
//! keys off it. So this is the chain's first real step — the `while` loop below
//! finally runs.
//!
//! **The legacy SQLite path does not come through here.** It builds its graph in
//! `load_work_uc::legacy_to_loaded` without ever constructing a `WorkBundle`, so
//! it mints its own uids; a fix confined to this file would silently miss every
//! legacy project.

use anyhow::Result;
// The Djot exporter's own escaper, shared rather than reimplemented: a second definition
// would disagree with it about exactly the awkward bodies (`- ` at a line start, a title
// in `[brackets]`, prose about `snake_case`) while agreeing on everything easy.
use text_document::{needs_djot_escaping, plain_text_to_djot};

use super::bundle::{FORMAT_VERSION, WorkBundle};

/// Walk `bundle` forward to [`FORMAT_VERSION`], one arm per transition.
///
/// **This does not decide whether the bundle is too new** — [`crate::version_gate`] does,
/// before any parsing, and it is the sole authority. A ceiling check here used to exist
/// and had to go: the gate judges `format_min_read_version` (the content-derived floor),
/// not the raw writer stamp, so `format_version` can legitimately exceed `FORMAT_VERSION`
/// on entry. That is the entire point of the floor scheme — a newer build saving a
/// project that contains nothing new stamps a higher version but a floor we understand.
/// Re-checking the stamp here would refuse exactly the file the gate just admitted, after
/// paying the full parse cost.
///
/// The loop below correctly no-ops for such a bundle, and the next save re-stamps
/// `format_version` to ours regardless (`folder_io::write_folder` writes the manifest,
/// `mapping::from_entities` builds it), so a from-the-future in-memory value never
/// reaches disk.
pub fn migrate_bundle(bundle: &mut WorkBundle) -> Result<()> {
    let v = bundle.manifest.format_version;
    if v == 0 {
        // The gate catches this pre-parse for every `read_bundle` call; kept here so the
        // function's own contract holds for a caller that builds a bundle in memory.
        anyhow::bail!("invalid .skrib format_version 0");
    }
    // One arm per transition, so the chain reads as the sequence it is and a
    // future v3→v4 step cannot be bolted onto an arm that already means
    // something else. A version with no arm fails loudly rather than being
    // silently stamped as current.
    while bundle.manifest.format_version < FORMAT_VERSION {
        match bundle.manifest.format_version {
            1 => step_v1_to_v2(bundle),
            2 => step_v2_to_v3(bundle),
            3 => step_v3_to_v4(bundle),
            4 => step_v4_to_v5(bundle),
            5 => step_v5_to_v6(bundle),
            6 => step_v6_to_v7(bundle),
            7 => step_v7_to_v8(bundle),
            8 => step_v8_to_v9(bundle),
            9 => step_v9_to_v10(bundle),
            10 => step_v10_to_v11(bundle),
            11 => step_v11_to_v12(bundle),
            other => anyhow::bail!("no migration step from .skrib format_version {other}"),
        }
        bundle.manifest.format_version += 1;
    }
    Ok(())
}

/// v10 → v11 mints a durable `uid` for every tag, comment, footnote and content row.
///
/// The same idea as [`step_v2_to_v3`] and [`step_v9_to_v10`], extended to the rows an
/// **out-of-tree** consumer needs to name. Nothing in this workspace required them: a
/// comment finds its `Content` by where its sidecar sits on disk, and a tag is looked up
/// by name. Neither trick is available to code that is not this crate, and a `file_id` is
/// only a store id at save time — so without these, no row outside the core tree could
/// refer to a particular scene text, synopsis, note, tag, remark or footnote at all.
///
/// A tag also could not be referenced by *name* even in principle: names are editable, so
/// a rename would silently rebind every reference to it.
///
/// Idempotent, like both its predecessors: a row that already carries a uid keeps it, so
/// re-running the step — or meeting a half-migrated bundle — never re-mints and never
/// breaks an existing reference.
fn step_v10_to_v11(bundle: &mut WorkBundle) {
    for t in &mut bundle.tags {
        t.uid = common::uid::heal_uid(t.uid);
    }
    for c in &mut bundle.orphan_comments {
        c.uid = common::uid::heal_uid(c.uid);
    }
    for f in &mut bundle.orphan_footnotes {
        f.uid = common::uid::heal_uid(f.uid);
    }
    for bb in &mut bundle.binders {
        for bi in &mut bb.items {
            for pr in &mut bi.item.prose_refs {
                pr.uid = common::uid::heal_uid(pr.uid);
            }
            for ic in &mut bi.item.inline_contents {
                ic.uid = common::uid::heal_uid(ic.uid);
            }
            for list in bi.comments.values_mut() {
                for c in list {
                    c.uid = common::uid::heal_uid(c.uid);
                }
            }
            for list in bi.footnotes.values_mut() {
                for f in list {
                    f.uid = common::uid::heal_uid(f.uid);
                }
            }
        }
    }
}

/// v11 → v12 mints a `uid` for every comment **reply**, and promotes every comment and
/// reply body from plain text to Djot.
///
/// # Why replies needed their own identity
///
/// v11 gave the thread one. That is enough to *find* a returning thread and not enough to
/// reconcile what is inside it: an editor who answers in the middle of a conversation
/// shifts every later reply by one, so matching them positionally re-imports the tail as
/// duplicates. Same argument as v11's, one level down.
///
/// # Why the bodies are rewritten rather than reinterpreted
///
/// `body` is now Djot, and most stored bodies already *are* the Djot that means
/// themselves — "Check this scene." parses to "Check this scene.". The ones that are not
/// would change meaning on the next read: a remark reading `*not* like this` would come
/// back emphasised, and one opening `- ` would become a list item. So every body is run
/// through [`plain_text_to_djot`], the same escaper the Djot **exporter** uses, so the two
/// cannot disagree about which strings are awkward.
///
/// Escaping is skipped where it would be a no-op ([`needs_djot_escaping`]), which keeps
/// ordinary bodies byte-identical on disk — and that matters beyond tidiness: a body left
/// untouched still reads correctly in an older build, so only projects that genuinely held
/// markup-like text are changed at all.
///
/// # Idempotency
///
/// Re-running must not double-escape. That is not automatic — `plain_text_to_djot` is not
/// idempotent on its own output (`\*` would become `\\\*`) — so the guard is the format
/// stamp: `migrate_bundle` advances `format_version` past 11 once, and this step never runs
/// against a v12 bundle. The uid half *is* independently idempotent, via `heal_uid`.
///
/// # The two shapes that change, and why neither loses meaning
///
/// A *plain-text* round trip through Djot cannot reproduce a **blank line** inside a body,
/// nor **trailing whitespace** on a line — `text_document::djot_round_trip_is_lossy` names
/// both, and it is the right tool for a caller whose exact bytes matter.
///
/// It is deliberately not used here, because for this field the bytes are not the meaning.
/// A blank line in a plain-text comment body *is* the plain-text encoding of a paragraph
/// break; once the body is Djot that break is carried by the paragraph structure itself,
/// which is strictly more faithful than the character that stood in for it. The card
/// renders two paragraphs either way. Trailing spaces on a line carry no meaning in a
/// remark at all.
///
/// So nothing is quietly dropped: what changes is the *encoding* of a break the writer
/// will still see — which is why this step owes them no reporting channel.
fn step_v11_to_v12(bundle: &mut WorkBundle) {
    fn heal_body(body: &mut String) {
        if needs_djot_escaping(body) {
            *body = plain_text_to_djot(body);
        }
    }

    for c in &mut bundle.orphan_comments {
        heal_body(&mut c.body);
        for r in &mut c.replies {
            r.uid = common::uid::heal_uid(r.uid);
            heal_body(&mut r.body);
        }
    }
    for bb in &mut bundle.binders {
        for bi in &mut bb.items {
            for list in bi.comments.values_mut() {
                for c in list {
                    heal_body(&mut c.body);
                    for r in &mut c.replies {
                        r.uid = common::uid::heal_uid(r.uid);
                        heal_body(&mut r.body);
                    }
                }
            }
        }
    }
}

/// v3 → v4 turned `dict_language` from a space-separated string into a real list.
///
/// The split itself happens in the deserializer (`bundle::tags_or_legacy_string`), because a
/// type change has to be tolerated at *parse* time — this chain runs afterwards, and a v3
/// file would never reach it. So this arm only advances the stamp, exactly as v1 → v2 does
/// for a field healed elsewhere. It still has to exist: a version with no arm fails loudly.
fn step_v3_to_v4(_bundle: &mut WorkBundle) {}

/// v4 → v5 added the note templates. Nothing to heal: a v4 bundle simply had none, and
/// `read_folder` already yields an empty list for the absent `templates.ron`. The bump
/// exists to stop an *older* build opening (and then silently re-saving without) a
/// project that has templates — see [`FORMAT_VERSION`].
fn step_v4_to_v5(_bundle: &mut WorkBundle) {}

/// v5 → v6 added epigraphs. Nothing to heal in this direction either: a v5 bundle simply
/// has no `EpigraphText` rows, and an absent `*.epigraph.djot` is indistinguishable from a
/// project that never wrote one. The bump exists for the *other* direction — an older
/// build cannot deserialize the new `ContentRole` variant at all, so
/// [`version_gate`](crate::version_gate) refuses the bundle up front instead of letting
/// `items.ron` fail with a raw "unexpected variant".
fn step_v5_to_v6(_bundle: &mut WorkBundle) {}

/// v6 → v7 added paratexts. Nothing to heal: a v6 bundle simply has none. The bump exists
/// so an older build refuses the file rather than failing to deserialize the two new enum
/// variants — the same reason v6 exists.
fn step_v6_to_v7(_bundle: &mut WorkBundle) {}

/// v7 → v8 added binary assets. Nothing to heal: a v7 bundle has no `assets.ron`
/// and no `assets/` tree, and `#[serde(default)]` already reads that as an empty
/// list.
///
/// The bump exists for the *other* direction. An older build's `WorkBundle` has
/// no assets field, and both writers rebuild from what the bundle holds — the zip
/// from a fresh staging directory, the exploded shape by pruning what it does not
/// expect. So an older build's first save would delete every image in the
/// project. The version floor (see `version_gate::compute_min_read_version`)
/// turns that into a refusal to open, and only for projects that have images.
fn step_v7_to_v8(_bundle: &mut WorkBundle) {}

/// v8 → v9 added footnotes, and has nothing to do on the way **forward**: a v8
/// bundle carries no `.footnotes.ron` sidecars and no orphanage, and
/// `#[serde(default)]` already reads that as no notes.
///
/// The bump exists for the other direction, exactly as the asset one does — and
/// with more at stake. Both writers rebuild from what the bundle holds, so an
/// older build's first save would prune every footnote sidecar off disk. Unlike
/// an image, which the writer can re-insert from the file they still have, those
/// words exist nowhere else: they were typed into the book. The floor turns that
/// into a refusal to open, and only for projects that actually have notes.
fn step_v8_to_v9(_bundle: &mut WorkBundle) {}

/// v9 → v10 mints a durable `uid` for every note template that lacks one.
///
/// The same step [`step_v2_to_v3`] runs for binders and items, and for the same reason:
/// until v10 a template's Djot blob was named after its `file_id`, an `EntityId` that
/// `load_work` re-mints, so every reopen renamed every template file and `prune_dir`
/// deleted the old names. Healing here rather than only at load keeps the rule where the
/// format can see it — a bundle that has been through this chain is guaranteed to name
/// its blobs by something that survives.
///
/// Idempotent, like its v3 counterpart: a template that already carries a uid keeps it.
fn step_v9_to_v10(bundle: &mut WorkBundle) {
    for t in &mut bundle.note_templates {
        t.uid = common::uid::heal_uid(t.uid);
    }
}

/// Mint a durable `uid` for every binder and item that lacks one.
///
/// Idempotent: a row that already carries a uid keeps it, so re-running the
/// step (or meeting a partially-migrated bundle) never re-mints and never
/// breaks an existing reference. v1 bundles pass through here too — they are
/// missing the field for the same reason v2 ones are.
fn step_v2_to_v3(bundle: &mut WorkBundle) {
    for bb in &mut bundle.binders {
        bb.binder.uid = common::uid::heal_uid(bb.binder.uid);
        for bi in &mut bb.items {
            bi.item.uid = common::uid::heal_uid(bi.item.uid);
        }
    }
}

/// v1 → v2 added `WorkFile.unique_id`, healed downstream by
/// `load_work_uc::materialize` rather than here, so this step only advances the
/// stamp. It exists so the chain has one arm per transition: a v1 bundle must
/// still pass through v2 on its way to v3.
fn step_v1_to_v2(_bundle: &mut WorkBundle) {}
